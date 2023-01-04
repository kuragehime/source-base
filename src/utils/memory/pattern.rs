//! Pattern scanner
use pelite::{
    pattern::Atom,
    pe::{Pe, PeView},
};
use winapi::um::libloaderapi::GetModuleHandleA;

use crate::to_cstr;

pub fn find<'pat>(module: &str, pattern: &'pat [Atom]) -> *mut usize
{
    let mut save = [0; 1];

    let module_handle = unsafe { GetModuleHandleA(to_cstr!(module)) };

    let view = unsafe { PeView::module(module_handle as _) };

    let _ = view.scanner().matches_code(pattern).next(&mut save);

    (module_handle as usize + save[0] as usize) as *mut usize
}
