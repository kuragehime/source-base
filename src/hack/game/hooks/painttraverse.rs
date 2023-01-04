use core::mem;

use crate::{iface, utils::hooking::vmt::VMTHook};
use lazy_static::lazy_static;

use spin::RwLock;
use winapi::ctypes::*;

type FnPaintTraverse = unsafe extern "fastcall" fn(
    ecx: *const c_void,
    edx: *const c_void,
    panel: u32,
    force_repaint: bool,
    allow_force: bool,
);
lazy_static! {
    static ref PAINTTRAVERSE_HOOK: RwLock<VMTHook> = RwLock::new(VMTHook::default());
    static ref original: FnPaintTraverse =
        unsafe { mem::transmute(PAINTTRAVERSE_HOOK.read().original(41)) };
}

pub(super) fn init() -> bool
{
    PAINTTRAVERSE_HOOK.write().init(iface!("VGUI_Panel"));
    PAINTTRAVERSE_HOOK
        .write()
        .hook_func(41, painttraverse_detour as _);
    lazy_static::initialize(&original);

    true
}

pub(super) fn destroy() -> bool
{
    PAINTTRAVERSE_HOOK.write().destroy(41);

    true
}

unsafe extern "fastcall" fn painttraverse_detour(
    ecx: *const c_void, edx: *const c_void, panel: u32, force_repaint: bool, allow_force: bool,
)
{
    original(ecx, edx, panel, force_repaint, allow_force);
}
