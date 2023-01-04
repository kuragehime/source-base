// --- string macros

/// &str to cstring ptr conversion
#[macro_export]
macro_rules! to_cstr {
    ($rstring:expr) => {
        alloc::format!("{}\0", $rstring).as_ptr() as *const i8
    };
    ($rstring:literal) => {
        concat!($rstring, "\0").as_ptr() as *const i8
    };
}
/// cstring ptr to &str conversion
#[macro_export]
macro_rules! to_rstr {
    ($cstring:expr) => {{
        #[allow(unused_unsafe)]
        let cstr = unsafe { core::ffi::CStr::from_ptr($cstring) };
        cstr.to_str().expect("couldn't convert string")
    }};
}

// ---- interface macros
/// Fetch interface pointer from global btreemap
#[macro_export]
macro_rules! iface {
    ($name:literal) => {{
        *$crate::hack::game::interfaces::INTERFACES
            .read()
            .get($name)
            .unwrap() as *mut usize
    }};
    ($name:literal,$t:ty) => {{
        *$crate::hack::game::interfaces::INTERFACES
            .read()
            .get($name)
            .unwrap() as *mut $t
    }};
}
/// Fetch interface reference from global btreemap
#[macro_export]
macro_rules! iface_ref {
    ($name:literal,$t:ty) => {{
        #[allow(unused_unsafe)]
        unsafe {
            (*$crate::hack::game::interfaces::INTERFACES
                .read()
                .get($name)
                .unwrap() as *mut $t)
                .as_ref()
                .unwrap()
        }
    }};
}

// ---- misc macros
/// AsRef macro
#[macro_export]
macro_rules! to_ref {
    ($thing:expr) => {{
        $thing.as_ref().unwrap()
    }};
}

// ---- netvar macros
/// Caches offsets from the netvar BTreeMap
#[macro_export]
macro_rules! cache_offset {
    ($self:expr,$netvar:literal) => {
        static mut cache: usize = 0usize;
        unsafe {
            if cache == 0
            {
                cache = netvars::get($netvar);
                return $self.get(cache);
            }
            return $self.get(cache);
        }
    };
}
