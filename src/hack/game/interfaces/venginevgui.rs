use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct VEngineVGui {}

#[allow(non_snake_case)]
impl VEngineVGui
{
    #[virtual_index(1)]
    pub fn GetPanel(&self, panel: u32) -> u32 {}
}
