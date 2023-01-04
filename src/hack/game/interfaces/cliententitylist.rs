use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::hack::game::sdk::cbaseentity::C_BaseEntity;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct VClientEntityList {}

#[allow(non_snake_case)]
impl VClientEntityList
{
    #[virtual_index(6)]
    pub fn GetHighestEntIdx(&self) -> i32 {}
    #[virtual_index(3)]
    pub fn GetClientEntity(&self, idx: i32) -> *mut C_BaseEntity {}
}
