use core::mem;

use crate::{iface, utils::hooking::vmt::VMTHook};
use lazy_static::lazy_static;

use spin::RwLock;
use winapi::ctypes::*;

type FnPaint = unsafe extern "fastcall" fn(*const c_void, i32, i32);

lazy_static! {
    static ref PAINT_HOOK: RwLock<VMTHook> = RwLock::new(VMTHook::default());
    static ref original: FnPaint = unsafe { mem::transmute(PAINT_HOOK.read().original(13)) };
}

pub(super) fn init() -> bool
{
    PAINT_HOOK.write().init(iface!("VEngineVGui"));
    PAINT_HOOK.write().hook_func(13, paint_detour as _);
    lazy_static::initialize(&original);

    true
}

pub(super) fn destroy() -> bool
{
    PAINT_HOOK.write().destroy(13);

    true
}

unsafe extern "fastcall" fn paint_detour(engine: *const c_void, edx: i32, mode: i32)
{
    //inicrt_println!("hi from paint!");
    original(engine, edx, mode);
}
