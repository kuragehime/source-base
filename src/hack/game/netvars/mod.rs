use alloc::{collections::BTreeMap, string::String};
use lazy_static::lazy_static;
use spin::RwLock;

use crate::{iface_ref, to_rstr};

use super::{
    interfaces::{vclient::VClient, INTERFACES},
    sdk::dt_recv::{CRecvTable, EPropType},
};
lazy_static! {
    pub static ref NETVARS: RwLock<BTreeMap<String, usize>> = RwLock::new(BTreeMap::new());
}

pub fn init()
{
    let vclient = unsafe { iface_ref!("VClient", VClient) };

    let mut client_class = vclient.GetAllClasses();

    while !client_class.is_null()
    {
        let recv_table = unsafe { client_class.read().recv_table };
        populate(recv_table);
        client_class = unsafe { client_class.read().next as _ };
    }
}

pub fn get(netvar: &str) -> usize { *NETVARS.read().get(netvar).unwrap() }

fn populate(table: *mut CRecvTable)
{
    let table_name = to_rstr!(table.read().table_name);

    for i in 0..unsafe { table.read().n_props }
    {
        let prop = unsafe { table.read().p_props.offset(i as _).read() };
        let prop_name = to_rstr!(prop.prop_name);

        if prop.prop_type == EPropType::DataTable
        {
            populate(prop.data_table);
        }

        NETVARS.write().insert(
            alloc::format!("{}::{}", table_name, prop_name),
            prop.offset as _,
        );
    }
}
