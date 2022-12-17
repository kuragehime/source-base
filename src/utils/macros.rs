// --- string macros

#[macro_export]
macro_rules! to_cstr {
    ($rstring:expr) => {
        alloc::format!("{}\0", $rstring).as_ptr() as *const i8
    };
    ($rstring:literal) => {
        concat!($rstring, "\0").as_ptr() as *const i8
    };
}
#[macro_export]
macro_rules! to_rstr {
    ($cstring:expr) => {{
        #[allow(unused_unsafe)]
        let cstr = unsafe { core::ffi::CStr::from_ptr($cstring) };
        cstr.to_str().expect("couldn't convert string")
    }};
}

// ---- interface macros

#[macro_export]
macro_rules! iface {
    ($name:literal) => {{
        *$crate::hack::game::interfaces::INTERFACES
            .read()
            .get($name)
            .unwrap() as *mut usize
    }};
}

// ---- hook macros
#[macro_export]
macro_rules! hook_add {
    ($name:literal,$hook:expr) => {{
        $crate::hack::game::hooks::HOOKS
            .write()
            .insert($name, $hook)
    }};
}

#[macro_export]
macro_rules! hook_get {
    ($name:literal) => {{
        $crate::hack::game::hooks::HOOKS.read().get($name).unwrap()
    }};
}
