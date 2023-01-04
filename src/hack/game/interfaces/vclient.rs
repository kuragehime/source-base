use core::ffi::c_char;

use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

use crate::hack::game::sdk::dt_recv::CRecvTable;

type CreateClientClassFn = unsafe extern "system" fn(ent: i32, serial: i32);
type CreateEventFn = unsafe extern "system" fn();

#[derive(Clone)]
#[repr(C)]
pub struct ClientClass
{
    create_client_class: CreateClientClassFn,
    create_event:        CreateEventFn,
    network_name:        *mut c_char,
    pub recv_table:      *mut CRecvTable,
    pub next:            *mut usize,
    pub class_id:        i32,
}
#[has_vtable]
#[derive(VTable, Debug)]
pub struct VClient {}

#[allow(non_snake_case)]
impl VClient
{
    #[virtual_index(8)]
    pub fn GetAllClasses(&self) -> *const ClientClass {}
}
