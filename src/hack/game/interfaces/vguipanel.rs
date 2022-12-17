use core::ffi::c_char;

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct VGUIPanel {}

#[allow(non_snake_case)]
impl VGUIPanel
{
    #[virtual_index(36)]
    pub fn GetPanelName(&self, panel: usize) -> *const c_char {}
}
