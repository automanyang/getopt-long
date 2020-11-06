// -- mod.rs --

#[cfg_attr(not(target_os = "linux"), link(name = "my_glibc"))]
mod my_glibc {
    use std::{
        ffi::CStr,
        os::raw::{c_char, c_int},
    };

    #[allow(unused)]
    extern "C" {
        pub(crate) static optarg: *mut c_char;
        pub(crate) static optind: c_int;

        pub(crate) fn getopt_long(
            argc: c_int,
            argv: *const *mut c_char,
            optstr: *const c_char,
            longopts: *const LongOption,
            longindex: *mut c_int,
        ) -> c_int;
    }

    #[repr(C)]
    #[derive(Debug)]
    pub(crate) struct LongOption {
        pub(crate) name: *const c_char,
        pub(crate) has_arg: c_int,
        pub(crate) flag: *mut c_int,
        pub(crate) val: c_int,
    }
    impl std::fmt::Display for LongOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "name: {:?}, has_arg: {}, flag: {:?}, value: {}",
                unsafe { CStr::from_ptr(self.name) },
                self.has_arg,
                self.flag,
                self.val
            )
        }
    }
    impl LongOption {
        pub(crate) const NO_ARGUMENT: c_int = 0;
        pub(crate) const REQUIRED_ARGUMENT: c_int = 1;
        pub(crate) const OPTIONAL_ARGUMENT: c_int = 2;
    }
}

// --

pub(crate) use my_glibc::{getopt_long, optarg, optind, LongOption};
