use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct VEngineClient {}

#[allow(non_snake_case)]
impl VEngineClient
{
    #[virtual_index(12)]
    pub fn GetLocalPlayer(&self) -> i32 {}
}
