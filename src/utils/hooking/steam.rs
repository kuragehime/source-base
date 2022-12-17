use core::{ffi::c_void, mem};
use lazy_static::lazy_static;
use pelite::pattern;

use crate::utils::memory;
type FnHook = unsafe extern "cdecl" fn(*const c_void, *const c_void, i32); // original,detour,jumps_to_follow
type FnUnHook = unsafe extern "cdecl" fn(*const c_void, bool); // original,should_log

lazy_static! {
    static ref steamhook: FnHook = unsafe {
        mem::transmute(memory::pattern::find(
            "GameOverlayRenderer.dll",
            pattern!("55 8B ?? 83 ?? ?? 53 56 FF ?? ?? 8D ?? ?? C7"),
        ))
    };
    static ref steamunhook: FnUnHook = unsafe {
        mem::transmute(memory::pattern::find("GameOverlayRenderer.dll",pattern!("55 8B ?? 64 ?? ?? ?? ?? ?? 6A ?? 68 ?? ?? ?? ?? 50 64 ?? ?? ?? ?? ?? ?? 81 ?? ?? ?? ?? ?? 56 8B ?? ?? 85")) )
    };
}

pub fn hook(original: *const c_void, detour: *const c_void, jumps_to_follow: i32)
{
    unsafe { steamhook(original, detour, jumps_to_follow) }
}
pub fn destroy(original: *const c_void, should_log: bool)
{
    unsafe { steamunhook(original, should_log) }
}
