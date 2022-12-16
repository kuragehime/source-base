use core::mem;

use crate::{hook_add, hook_get, iface, minicrt_println, utils::hooking::vmt::VMTHook};
use lazy_static::lazy_static;
use spin::RwLock;
use winapi::{ctypes::*, um::winnt::PVOID};
type FnPaint = unsafe extern "fastcall" fn(*const c_void, i32, i32);
lazy_static! {
    static ref PAINT_HOOK: RwLock<VMTHook> = RwLock::new(VMTHook::default());
}
pub(super) fn init() -> bool
{
    unsafe {
        PAINT_HOOK.write().init(iface!("VEngineVGui"));
        PAINT_HOOK.write().hook_func(13, paint_detour as _);
    };

    true
}
pub(super) fn destroy() -> bool
{
    unsafe {
        PAINT_HOOK.write().destroy(13);
        true
    }
}
unsafe extern "fastcall" fn paint_detour(engine: *const c_void, edx: i32, mode: i32)
{
    let original: FnPaint = mem::transmute(PAINT_HOOK.read().original(13));
    //inicrt_println!("hi from paint!");
    original(engine, edx, mode);
}
