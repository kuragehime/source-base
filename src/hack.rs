pub mod game;
use winapi::{
    ctypes::c_void,
    um::{
        libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA},
        synchapi::Sleep,
        winuser::{GetAsyncKeyState, VK_END},
    },
};

use crate::{
    hack::game::interfaces::{self, InterfaceReg, INTERFACES},
    iface, minicrt_println, to_cstr, to_rstr,
    utils::wrappers,
};

pub unsafe extern "system" fn init(module: *mut c_void) -> u32
{
    wrappers::alloc_console("base");
    minicrt_println!("Hello world!");

    interfaces::init(&[
        "client.dll",
        "engine.dll",
        "vgui2.dll",
        "vguimatsurface.dll",
    ]);

    game::hooks::init();
    loop
    {
        if GetAsyncKeyState(VK_END) != 0
        {
            FreeLibraryAndExitThread(module as _, 0);
            break;
        }
        Sleep(50);
    }
    0
}
pub unsafe extern "system" fn terminate() -> u32
{
    wrappers::free_console();
    game::hooks::destroy();
    1
}
