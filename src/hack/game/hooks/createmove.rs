use core::{
    ffi::{c_float, c_void},
    mem,
};

use pelite::pattern;

use crate::utils::{hooking, memory};

type FnCreateMove =
    unsafe extern "fastcall" fn(*const c_void, *const c_void, c_float, *const c_void) -> bool;
use lazy_static::lazy_static;

lazy_static! {
    static ref original: FnCreateMove = unsafe {
        mem::transmute(memory::pattern::find(
            "client.dll",
            pattern!("55 8B EC 56 8D 75 04 8B 0E E8 ? ? ? ? 8B"),
        ))
    };
}

pub(super) fn init() -> bool
{
    hooking::steam::hook(*original as _, createmove_detour as _, 0);
    true
}
pub(super) fn destroy() -> bool
{
    hooking::steam::destroy(*original as _, false);
    true
}
unsafe extern "fastcall" fn createmove_detour(
    ecx: *const c_void, edx: *const c_void, sampletime: c_float, pcmd: *const c_void,
) -> bool
{
    original(ecx, edx, sampletime, pcmd);
    return false;
}
