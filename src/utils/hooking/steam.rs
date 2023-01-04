//! Detour hooking via GameOverlayRenderer
use crate::utils::memory;
use core::{ffi::c_void, mem, ptr};
use lazy_static::lazy_static;
use pelite::pattern;

type FnHook = unsafe extern "cdecl" fn(*const c_void, *const c_void, *const c_void, i32); // original,detour,jumps_to_follow
type FnUnHook = unsafe extern "cdecl" fn(*const c_void, bool); // original,should_log

lazy_static! {
    static ref steamhook: FnHook = unsafe {
        let func = mem::transmute(memory::pattern::find(
            "GameOverlayRenderer.dll",
            pattern!("55 8B EC 51 8B 45 10 C7"),
        ));

        return func;
    };
    static ref steamunhook: FnUnHook = unsafe {
        let func = mem::transmute(memory::pattern::find("GameOverlayRenderer.dll",pattern!("55 8B ?? 64 ?? ?? ?? ?? ?? 6A ?? 68 ?? ?? ?? ?? 50 64 ?? ?? ?? ?? ?? ?? 81 ?? ?? ?? ?? ?? 56 8B ?? ?? 85")) );

        return func;
    };
}

// This is really unstable, not sure why but i wouldn't use it. I need to write a hooking lib.
pub fn hook(original: *const c_void, detour: *const c_void, jumps_to_follow: i32)
{
    unsafe { steamhook(original, detour, ptr::null_mut(), jumps_to_follow) };
}

pub fn destroy(original: *const c_void, should_log: bool)
{
    unsafe { steamunhook(original, should_log) }
}
