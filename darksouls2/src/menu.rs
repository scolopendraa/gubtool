use {
    crate::{
        mem::*,
        offsets::{code_cave::CaveAddress, game_manager_imp, module_offsets::Function},
        pointer_cache::ResolvedPtr,
        resources::menus::{MenuType, Shop, Trade},
        utils::player_loaded_check,
    },
    gubtool_core::{
        address::Address,
        attached::is_32,
        sys::ipc::{CppValue, X86CallingConvention},
    },
    std::{thread, time::Duration},
};

pub fn open_shop(shop: Shop) -> anyhow::Result<()> {
    write::<u32>(CaveAddress::NpcTalkArgs.add_offset(0x4), shop as u32)?;
    write::<u32>(CaveAddress::NpcTalkArgs.add_offset(0x8), shop as u32 + 999)?;
    open_menu(MenuType::Shop)
}

pub fn open_trade(trade: Trade) -> anyhow::Result<()> {
    write::<u32>(CaveAddress::NpcTalkArgs.add_offset(0x14), trade as u32)?;
    write::<u32>(CaveAddress::NpcTalkArgs.add_offset(0x2c), trade as u32 + 999)?;
    open_menu(MenuType::Trading)
}

pub fn open_menu(menu_type: MenuType) -> anyhow::Result<()> {
    player_loaded_check()?;

    let args_loc = CaveAddress::OpenMenuArgs.addr();
    let npc_args_loc = CaveAddress::NpcTalkArgs.addr();

    if is_32() {
        write::<u32>(args_loc, npc_args_loc as u32)?;
        write::<u8>(args_loc + 0x4, menu_type as u8)?;
        write::<u32>(args_loc + 0x20, 0x1)?;
    } else {
        write::<u64>(args_loc, npc_args_loc)?;
        write::<u8>(args_loc + 0x8, menu_type as u8)?;
        write::<u64>(args_loc + 0x28, 0x1)?;
    }

    let args = [
        CppValue::uintptr_t(ResolvedPtr::WindowManager.get()?),
        CppValue::uintptr_t(args_loc),
        CppValue::uintptr_t(CaveAddress::NpcPos.addr()),
    ];

    set_menu_open_chr_state(true)?;
    run_game_function(Function::OpenMenu, &args, X86CallingConvention::__thiscall)?;

    tokio::spawn(async {
        while is_menu_open() {
            thread::sleep(Duration::from_millis(100));
        }
        set_menu_open_chr_state(false)
    });
    Ok(())
}

fn is_menu_open() -> bool {
    follow_pointers(&game_manager_imp::fe_item_select_menu_chain(), true).is_ok()
}

fn set_menu_open_chr_state(state: bool) -> anyhow::Result<()> {
    let args = [
        CppValue::uintptr_t(ResolvedPtr::DlBackAllocator.get()?),
        CppValue::uint8_t(state as u8),
    ];

    run_game_function(Function::MenuChrState, &args, X86CallingConvention::__thiscall)
}
