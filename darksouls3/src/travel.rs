use {
    crate::{
        game_state,
        mem::run_game_function,
        offsets::{
            code_cave::CaveAddr,
            module_offsets::{Function, Hook},
        },
        resources::{ASM, bonfires::Bonfire, bosses::Boss},
        utils::dlc_check,
    },
    assemble::patch::DWORD,
    gubtool_core::{
        address::{POINTER, Pointer},
        sys::{install_hook, ipc::FfiValue, write},
    },
    std::time::Duration,
};

fn bonfire_warp(bonfire_id: u32) -> anyhow::Result<()> {
    let dark_cemetary = if matches!(bonfire_id, 4001953 | 4001954) { 10 } else { 0 };

    let args = [
        FfiValue::pointer(Pointer::default()),
        FfiValue::uint32(bonfire_id),
        FfiValue::uint32(dark_cemetary),
    ];

    run_game_function(Function::BonfireWarp, &args)
}

async fn bonfire_warp_with_coords(
    bonfire_id: u32,
    coords: [f32; 3],
    angle: f32,
) -> anyhow::Result<()> {
    bonfire_warp(bonfire_id)?;

    let [x, y, z] = coords;
    let coords = [x, y, z, 1.0];
    write::<[f32; 4]>(CaveAddr::WarpCoords, coords)?;

    let angle = [0.0, angle, 0.0, 0.0];
    write::<[f32; 4]>(CaveAddr::WarpAngle, angle)?;

    let mut coords_hook = ASM.get_function("warp_hook");
    coords_hook.patch::<POINTER>("property_loc", CaveAddr::WarpCoords);
    coords_hook.patch::<DWORD>("property_offset", 0xa40);

    let mut angle_hook = ASM.get_function("warp_hook");
    angle_hook.patch::<POINTER>("property_loc", CaveAddr::WarpAngle);
    angle_hook.patch::<DWORD>("property_offset", 0xa50);

    while game_state::is_loaded() {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    install_hook(&coords_hook.bytes, CaveAddr::WarpCoordsHook, Hook::WarpCoords, 8)?;
    install_hook(&angle_hook.bytes, CaveAddr::WarpAngleHook, Hook::WarpAngle, 8)?;

    while !game_state::is_loaded() {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    write::<[u8; 8]>(Hook::WarpCoords, [0x66, 0x0f, 0x7f, 0x80, 0x40, 0x0a, 0x00, 0x00])?;
    write::<[u8; 8]>(Hook::WarpAngle, [0x66, 0x0f, 0x7f, 0x80, 0x50, 0x0a, 0x00, 0x00])?;

    Ok(())
}

impl Bonfire {
    pub fn warp(&self) -> anyhow::Result<()> {
        dlc_check(self.dlc)?;
        bonfire_warp(self.bonfire_id)
    }
}

impl Boss {
    pub async fn warp(&self) -> anyhow::Result<()> {
        dlc_check(self.dlc)?;
        bonfire_warp_with_coords(self.bonfire_id, self.coords, self.angle).await
    }
}
