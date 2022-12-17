#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(pointer_byte_offsets)]
#![feature(abi_thiscall)]
#![allow(non_upper_case_globals)]
use core::{
    alloc::{GlobalAlloc, Layout},
    panic::PanicInfo,
};
#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> !
{
    // zzzzzzzzzz mimimimimi

    minicrt_println!("{:?}", _panic);
    loop
    {}
}
#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> !
{
    // zzzzzzzzzz mimimimimi
    loop
    {}
}
pub mod hack;
pub mod utils;

extern crate alloc;

use core::ptr;

use winapi::{
    ctypes::c_void,
    um::{
        handleapi::CloseHandle,
        heapapi::{GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc},
        processthreadsapi::CreateThread,
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, HEAP_ZERO_MEMORY},
    },
};

pub struct HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator
{
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8
    {
        HeapAlloc(GetProcessHeap(), 0, _layout.size()) as *mut u8
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
    {
        HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout)
    {
        HeapFree(GetProcessHeap(), 0, _ptr as *mut c_void);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8
    {
        HeapReAlloc(
            GetProcessHeap(),
            HEAP_ZERO_MEMORY,
            ptr as *mut c_void,
            new_size,
        ) as *mut u8
    }
}

#[global_allocator]
static GLOBAL: HeapAllocator = HeapAllocator;

#[no_mangle]
unsafe extern "system" fn _DllMainCRTStartup(module: *const u8, reason: u32, _: *const u8) -> u32
{
    match reason
    {
        DLL_PROCESS_ATTACH =>
        {
            let handle = CreateThread(
                ptr::null_mut(),
                0,
                Some(hack::init),
                module as *mut _,
                0,
                ptr::null_mut(),
            );
            if !handle.is_null()
            {
                CloseHandle(handle);
            }
        }
        DLL_PROCESS_DETACH =>
        {
            hack::terminate();
        }
        _ =>
        {}
    }
    1
}
