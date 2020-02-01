// -- main.rs --
// #[macro_use]
// use getopt_long::*;
// use libc::{c_int, gethostname};
use std::{
    env,
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
};

extern "C" {
    fn gethostname(name: *mut c_char, len: usize) -> i32;
    // extern char *optarg;
    // extern int optind, opterr, optopt;
    //
    // int getopt_long(int argc, char * const argv[],
    //     const char *optstring,
    //     const struct option *longopts, int *longindex);
    // struct option {
    //     const char *name;
    //     int         has_arg;
    //     int        *flag;
    //     int         val;
    // };

    static optarg: *mut c_char;
    static optind: c_int;
    static opterr: c_int;
    static optopt: c_int;

    // fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int;

    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstr: *const c_char,
        longopts: *const LongOption,
        longindex: *mut c_int,
    ) -> c_int;
}

#[repr(i32)]
enum HasArg {
    NoArgument = 0,
    RequiredArgument,
    OptionalArgument,
}

#[repr(C)]
struct LongOption {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

// pub unsafe extern "C" fn getopt(
//     argc: c_int,
//     argv: *const *mut c_char,
//     optstr: *const c_char
// ) -> c_int

// --

fn main() {
    let longopts = unsafe {
        &[
            LongOption {
                name: CStr::from_bytes_with_nul_unchecked(b"add\0").as_ptr(),
                has_arg: HasArg::RequiredArgument as c_int,
                flag: 0 as *mut c_int,
                val: 0,
            },
            LongOption {
                name: CStr::from_bytes_with_nul_unchecked(b"remove\0").as_ptr(),
                has_arg: HasArg::OptionalArgument as c_int,
                flag: 0 as *mut c_int,
                val: 0,
            },
            LongOption {
                name: CStr::from_bytes_with_nul_unchecked(b"modify\0").as_ptr(),
                has_arg: HasArg::NoArgument as c_int,
                flag: 0 as *mut c_int,
                val: 0,
            },
            LongOption {
                name: 0 as *const c_char,
                has_arg: 0,
                flag: 0 as *mut c_int,
                val: 0,
            },
        ]
    };
    let mut longindex: c_int = 0;

    // create a vector of zero terminated strings
    let argv = env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let argv = argv
        .into_iter()
        .map(|arg| arg.into_raw())
        .collect::<Vec<*mut c_char>>();
    let argc = argv.len() as c_int;
    let optstring = unsafe { CStr::from_bytes_with_nul_unchecked(b"a:b:cd\0") }.as_ptr();
    loop {
        // match unsafe { getopt(argc, argv.as_ptr(), optstring) } {
        match unsafe {
            getopt_long(
                argc,
                argv.as_ptr(),
                optstring,
                longopts.as_ptr(),
                &mut longindex,
            )
        } {
            -1 => break,
            0 => unsafe {
                let longopt = &longopts[longindex as usize];
                println!(
                    "longopts: name = {:?}, optarg = {:?}",
                    CStr::from_ptr(longopt.name),
                    if optarg.is_null() {
                        CStr::from_bytes_with_nul_unchecked(b"null\0")
                    } else {
                        CStr::from_ptr(optarg)
                    }
                );
            },
            // 97 =>
            i => println!("{}", unsafe {
                format!(
                    "opt = {}, optarg = {:?}, optind = {}, optopt = {}, opterr = {}",
                    i,
                    if optarg.is_null() {
                        CStr::from_bytes_with_nul_unchecked(b"null\0")
                    } else {
                        CStr::from_ptr(optarg)
                    },
                    optind,
                    optopt,
                    opterr
                )
            }),
        }
    }

    let mut buf = [0u8; 255];
    let len = buf.len();

    let ptr = buf.as_mut_ptr() as *mut c_char;
    unsafe {
        gethostname(ptr, len);
        println!("hostname: {:?}", CStr::from_ptr(ptr));
    }
}
