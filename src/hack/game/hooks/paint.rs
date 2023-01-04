use core::mem;

use crate::{
    hack::game::interfaces::vguisurface::VGUISurface,
    iface, iface_ref,
    utils::{hooking::vmt::VMTHook, memory},
};
use lazy_static::lazy_static;

use pelite::pattern;
use spin::RwLock;
use winapi::ctypes::*;

type FnPaint = unsafe extern "thiscall" fn(*const c_void, i32);
type FnStartDraw = unsafe extern "thiscall" fn(*const c_void);
type FnEndDraw = unsafe extern "thiscall" fn(*const c_void);
type FnSetClipRect = unsafe extern "thiscall" fn(*const c_void, i32, i32, i32, i32);

lazy_static! {
    static ref PAINT_HOOK: RwLock<VMTHook> = RwLock::new(VMTHook::default());
    static ref original: FnPaint = unsafe { mem::transmute(PAINT_HOOK.read().original(14)) };
    static ref start_draw: FnStartDraw = unsafe {
        mem::transmute(memory::pattern::find(
            "vguimatsurface.dll",
            pattern!("55 8B EC 83 E4 C0 83 EC 38 "),
        ))
    };
    static ref end_draw: FnEndDraw = unsafe {
        mem::transmute(memory::pattern::find(
            "vguimatsurface.dll",
            pattern!("8B 0D ? ? ? ? 56 C6 05 ? ? ? ? ?"),
        ))
    };
    static ref set_clip_rect: FnSetClipRect = unsafe {
        mem::transmute(memory::pattern::find(
            "vguimatsurface.dll",
            pattern!("55 8B EC 56 8B F1 83 BE ? ? ? ? ? 74 7F"),
        ))
    };
    static ref surface: &'static VGUISurface = iface_ref!("VGUI_Surface", VGUISurface) as _;
}

pub(super) fn init() -> bool
{
    PAINT_HOOK.write().init(iface!("VEngineVGui"));
    PAINT_HOOK.write().hook_func(14, paint_detour as _);

    true
}

pub(super) fn destroy() -> bool
{
    PAINT_HOOK.write().destroy(14);

    true
}

unsafe extern "thiscall" fn paint_detour(engine: *const c_void, mode: i32)
{
    original(engine, mode);

    if mode & 1 != 0
    {
        start_draw(iface!("VGUI_Surface") as _); // Just like i mentioned in createmove, don't do this and get ifaces once.
        {
            surface.SetDrawColor(255, 255, 255, 255);

            surface.DrawFilledRect(50, 50, 200, 200);
        }
        end_draw(iface!("VGUI_Surface") as _);
    }
}
