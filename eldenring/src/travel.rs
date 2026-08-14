use {
    crate::{
        emevd,
        event,
        mem::*,
        offsets::{
            ChainReadExt,
            code_cave::CaveAddress,
            menu_man,
            module_offsets::{BasePointer, Function, Hook},
        },
        pointer_cache::ResolvedPtr,
        resources::{ASM, bosses::Boss, graces::Grace},
        utils::{dlc_check, player_loaded_check},
    },
    gubtool_core::{address::Address, slice_ops::*, sys::ipc::CppValue},
    std::time::Duration,
};

pub fn warp_to_grace(grace_id: i64) -> anyhow::Result<()> {
    let mut fun = ASM.get_function("warp_to_grace");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
    write_to_slice::<i64>(&mut asm, fun.reloc("grace_id"), grace_id)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_grace_warp"), Function::GraceWarp)?;

    run_custom_function(asm)
}

pub async fn warp_to_block_id(
    block_id: i32,
    coords: [f32; 3],
    angle: f32,
    is_night: bool,
) -> anyhow::Result<()> {
    let area = (block_id >> 24) & 0xff;
    let block = (block_id >> 16) & 0xff;
    let map = (block_id >> 8) & 0xff;
    let alt_no = block_id & 0xff;

    let args = [
        CppValue::int32_t(area),
        CppValue::int32_t(block),
        CppValue::int32_t(map),
        CppValue::int32_t(alt_no),
    ];

    run_game_function(Function::BlockWarp, &args)?;

    hook_warp_coord_writes(coords, angle, is_night).await
}

async fn hook_warp_coord_writes(
    coords: [f32; 3],
    angle: f32,
    is_night: bool,
) -> anyhow::Result<()> {
    let mut target_coords: [u8; 16] = [0; 16];
    write_to_slice::<f32>(&mut target_coords, 0, coords[0])?;
    write_to_slice::<f32>(&mut target_coords, 4, coords[1])?;
    write_to_slice::<f32>(&mut target_coords, 8, coords[2])?;
    write_to_slice::<f32>(&mut target_coords, 12, 1.0_f32)?;

    write_bytes(CaveAddress::WarpCoords, &target_coords)?;
    write::<f32>(CaveAddress::WarpAngle.add_offset(4), angle)?;

    let mut fun = ASM.get_function("warp_coord_angle_hook");
    let mut asm = fun.take_bytes();

    let code_loc = CaveAddress::WarpCoordsHook;
    write_rel_i32(&mut asm, code_loc, fun.reloc("new_val"), CaveAddress::WarpCoords, 4)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("property_offset"), 0xaa0)?;
    write_rel_i32(
        &mut asm,
        code_loc,
        fun.reloc("hook_loc"),
        Hook::WarpCoordWrite.add_offset(7),
        4,
    )?;
    install_hook(&asm, code_loc, Hook::WarpCoordWrite, 7)?;

    let mut fun = ASM.get_function("warp_coord_angle_hook");
    let mut asm = fun.take_bytes();

    let code_loc = CaveAddress::WarpAngleHook;
    write_rel_i32(&mut asm, code_loc, fun.reloc("new_val"), CaveAddress::WarpAngle, 4)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("property_offset"), 0xab0)?;
    write_rel_i32(
        &mut asm,
        code_loc,
        fun.reloc("hook_loc"),
        Hook::WarpAngleWrite.add_offset(7),
        4,
    )?;
    install_hook(&asm, code_loc, Hook::WarpAngleWrite, 7)?;

    wait_to_unhook_warp(is_night).await
}

const COORD_HOOK_ORIGINAL: [u8; 7] = [0x0f, 0x11, 0x80, 0xa0, 0x0a, 0x00, 0x00];
const ANGLE_HOOK_ORIGINAL: [u8; 7] = [0x0f, 0x11, 0x80, 0xb0, 0x0a, 0x00, 0x00];
async fn wait_to_unhook_warp(is_night: bool) -> anyhow::Result<()> {
    let is_faded_ptr = ResolvedPtr::MenuMan
        .get()
        .add_offset(menu_man::is_fading())?;

    while !is_bit_set(is_faded_ptr, menu_man::fade_bit_flags::IS_FADE_SCREEN)? {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    while is_bit_set(is_faded_ptr, menu_man::fade_bit_flags::IS_FADE_SCREEN)? {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    if is_night {
        emevd::set_night()?;
    }

    write_bytes(Hook::WarpCoordWrite, &COORD_HOOK_ORIGINAL)?;
    write_bytes(Hook::WarpAngleWrite, &ANGLE_HOOK_ORIGINAL)?;
    Ok(())
}

impl Boss {
    pub async fn warp(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        if self.name == "Grafted Scion" && !event::get_event(10010801)? {
            warp_to_block_id(self.block_id, [-33.27, 21.37, -87.86], 2.92, self.is_night).await?;
        } else {
            warp_to_block_id(self.block_id, self.coords, self.angle, self.is_night).await?;
        }
        Ok(())
    }
}

impl Grace {
    pub fn warp(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        warp_to_grace(self.grace_entity_id)
    }
}
