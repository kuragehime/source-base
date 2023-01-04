use core::{
    ffi::{c_float, c_void},
    mem,
};

use lazy_static::lazy_static;

use spin::RwLock;

use crate::{
    hack::game::{
        interfaces::{cliententitylist::VClientEntityList, engineclient::VEngineClient},
        sdk::usercmd::CUserCMD,
    },
    iface, iface_ref, to_ref,
    utils::hooking::vmt::VMTHook,
};

type FnCreateMove =
    unsafe extern "fastcall" fn(*const c_void, *const c_void, c_float, *mut CUserCMD) -> bool;

lazy_static! {
    static ref CMOVE_HOOK: RwLock<VMTHook> = RwLock::new(VMTHook::default());
    static ref original: FnCreateMove = unsafe { mem::transmute(CMOVE_HOOK.read().original(24)) };
}

pub(super) fn init() -> bool
{
    let client = iface!("VClient");

    let client_mode =
        unsafe { **(((*((*(client as *mut *mut usize)).offset(10))) + 5) as *mut *mut *mut usize) }; // TODO: Implement a way to NOT do this

    CMOVE_HOOK.write().init(client_mode);
    CMOVE_HOOK.write().hook_func(24, createmove_detour as _);

    lazy_static::initialize(&original);

    true
}

pub(super) fn destroy() -> bool
{
    CMOVE_HOOK.write().destroy(24);
    true
}

unsafe extern "fastcall" fn createmove_detour(
    ecx: *const c_void, edx: *const c_void, sampletime: c_float, pcmd: *mut CUserCMD,
) -> bool
{
    if pcmd.read().command_number == 0 || pcmd.read().tick_count == 0
    {
        return false;
    }

    let entlist = iface_ref!("VClientEntityList", VClientEntityList); // Don't do this in your hack, this is just for testing, save static refs or something
    let engine = iface_ref!("VEngineClient", VEngineClient); // TODO: should probably implement something like the netvar offset macro for these
    let local = to_ref!(entlist.GetClientEntity(engine.GetLocalPlayer()));

    if pcmd.read().buttons & 2 != 0 && local.flags() & (1 << 0) == 0
    // bhop exampole
    {
        (*pcmd).buttons &= !(1 << 1);
    }

    return original(ecx, edx, sampletime, pcmd);
}
