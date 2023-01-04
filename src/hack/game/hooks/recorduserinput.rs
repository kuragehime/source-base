use core::{ffi::c_void, mem};

use spin::RwLock;
use winapi::um::libloaderapi::GetModuleHandleA;

use crate::{to_cstr, utils::hooking::vmt::VMTHook};

type FnRecordUserInput = unsafe extern "thiscall" fn(*const c_void, i32) -> i32;
use lazy_static::lazy_static;

lazy_static! {
    static ref USERINPUT_HOOK: RwLock<VMTHook> = RwLock::new(VMTHook::default());
    static ref original: FnRecordUserInput =
        unsafe { mem::transmute(USERINPUT_HOOK.read().original(5)) };
}

pub(super) fn init() -> bool
{
    let enginedll = unsafe { GetModuleHandleA(to_cstr!("engine.dll")) as *mut usize };
    let class_base = unsafe { enginedll.byte_offset(0x59D648) };

    USERINPUT_HOOK.write().init(class_base as _);
    USERINPUT_HOOK
        .write()
        .hook_func(5, recorduserinput_detour as _);

    lazy_static::initialize(&original);

    true
}
pub(super) fn destroy() -> bool
{
    USERINPUT_HOOK.write().destroy(5);

    true
}
unsafe extern "thiscall" fn recorduserinput_detour(this: *const c_void, cmdnumber: i32) -> i32
{
    original(this, cmdnumber)
}
