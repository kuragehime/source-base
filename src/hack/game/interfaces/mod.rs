pub mod vguisurface;
pub mod vguipanel;
pub mod venginevgui;
pub mod vclient;
use alloc::{collections::BTreeMap, string::String};
use core::ffi::c_char;
use lazy_static::lazy_static;

use spin::RwLock;
use winapi::{
    ctypes::c_void,
    um::libloaderapi::{GetModuleHandleA, GetProcAddress},
};

use crate::{minicrt_println, to_cstr, to_rstr};

lazy_static! {
    pub static ref INTERFACES: RwLock<BTreeMap<String, usize>> = RwLock::new(BTreeMap::new());
}

type InstantiateInterfaceFn = extern "C" fn() -> *mut c_void;

pub struct InterfaceReg
{
    pub m_create_fn: InstantiateInterfaceFn,
    pub m_p_name:    *const c_char,
    pub m_p_next:    *mut InterfaceReg,
}
pub fn init(mod_names: &[&str]) -> bool
{
    for name in mod_names
    {
        unsafe {
            let module = GetModuleHandleA(to_cstr!(name));

            let create_iface_addr =
                GetProcAddress(module, to_cstr!("CreateInterface")) as *mut usize;
            /*
            .text:52BE9D80                   CreateInterface proc near               ; DATA XREF: .rdata:off_52E200D8↓o
            .text:52BE9D80 55                                push    ebp
            .text:52BE9D81 8B EC                             mov     ebp, esp
            .text:52BE9D83 5D                                pop     ebp
            .text:52BE9D84 E9 87 FC FF FF                    jmp     CreateInterfaceInternal
            .text:52BE9D84                   CreateInterface endp
            */
            let jump_addr = create_iface_addr.byte_add(4);
            let relative_addr = jump_addr.byte_add(1).read();

            let create_iface_internal = jump_addr.byte_add(5).byte_add(relative_addr);
            /*
            .text:52BE9A10 55                                push    ebp
            .text:52BE9A11 8B EC                             mov     ebp, esp
            .text:52BE9A13 56                                push    esi
            .text:52BE9A14 8B 35 E4 87 F8 52                 mov     esi, InterfaceReg__s_pInterfaceRegs
            .text:52BE9A1A 57                                push    edi
            .text:52BE9A1B 85 F6                             test    esi, esi
            */
            let interface_list = create_iface_internal.byte_add(6).read();
            let interface_list: *mut InterfaceReg = *(interface_list as *mut *mut InterfaceReg);

            minicrt_println!("{} -> {:#x?}", name, module as *mut usize);

            let mut current = interface_list;

            while !current.is_null()
            {
                let current_ref = &*current;
                let iface_name = to_rstr!(current_ref.m_p_name)
                    .chars()
                    .filter(|c| !c.is_numeric())
                    .collect::<String>();
                let iface_addr = (current_ref.m_create_fn)();
                minicrt_println!(
                    "[{}] {} -> {:#x?}",
                    name,
                    iface_name,
                    iface_addr as *mut usize
                );
                INTERFACES.write().insert(iface_name, iface_addr as usize);
                current = current_ref.m_p_next;
            }
        }
    }

    true
}
