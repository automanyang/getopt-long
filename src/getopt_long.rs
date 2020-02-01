// -- getopt_long.rs --

use {
    crate::SpotError,
    std::{
        env,
        ffi::{CStr, CString, NulError},
        os::raw::{c_char, c_int},
    },
};

// --

// extern char *optarg;
// extern int optind, opterr, optopt;
//
// struct option {
//     const char *name;
//     int         has_arg;
//     int        *flag;
//     int         val;
// };
//
// int getopt(int argc, char * const argv[],
//     const char *optstring);
//
// int getopt_long(int argc, char * const argv[],
//     const char *optstring,
//     const struct option *longopts, int *longindex);

mod from_glibc {
    use std::{
        ffi::CStr,
        os::raw::{c_char, c_int},
    };

    #[allow(unused)]
    extern "C" {
        pub(super) static optarg: *mut c_char;
        pub(super) static optind: c_int;
        pub(super) static opterr: c_int;
        pub(super) static optopt: c_int;

        // fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int;

        pub(super) fn getopt_long(
            argc: c_int,
            argv: *const *mut c_char,
            optstr: *const c_char,
            longopts: *const LongOption,
            longindex: *mut c_int,
        ) -> c_int;
    }

    #[repr(C)]
    #[derive(Debug)]
    pub(super) struct LongOption {
        pub(super) name: *const c_char,
        pub(super) has_arg: c_int,
        pub(super) flag: *mut c_int,
        pub(super) val: c_int,
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
        pub(super) const NO_ARGUMENT: c_int = 0;
        pub(super) const REQUIRED_ARGUMENT: c_int = 1;
        pub(super) const OPTIONAL_ARGUMENT: c_int = 2;
    }
}

// --

#[derive(Debug)]
pub enum OptError {
    InvalidOption(String),
    MissingOptionArgument(String),
    Other(String),
}

impl std::error::Error for OptError {}

impl<T: Into<String>> std::convert::From<T> for OptError {
    fn from(e: T) -> Self {
        Self::Other(e.into())
    }
}

impl std::fmt::Display for OptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OptError({:?})", self)
    }
}

pub type OptResult<T> = Result<T, OptError>;

// --

#[repr(i32)]
pub enum HasArg {
    NoArgument = 0,
    RequiredArgument,
    OptionalArgument,
}

pub struct Opt {
    long_name: Option<String>,
    shadow_name: Option<CString>,
    short_name: Option<char>,
    has_arg: HasArg,
    desc: String,
}
impl Opt {
    pub fn new(
        long_name: Option<String>,
        short_name: Option<char>,
        has_arg: HasArg,
        desc: &str,
    ) -> Result<Self, NulError> {
        let shadow = long_name
            .as_ref()
            .map(|v| CString::new(v.clone()))
            .transpose()?;
        Ok(Self {
            long_name,
            shadow_name: shadow,
            short_name,
            has_arg,
            desc: desc.to_owned(),
        })
    }
    fn optstring(&self) -> Option<String> {
        self.short_name.map(|v| {
            let mut s = String::with_capacity(3);
            s.push(v);
            match self.has_arg {
                HasArg::NoArgument => {}
                HasArg::OptionalArgument => {
                    s.push_str("::");
                }
                HasArg::RequiredArgument => s.push(':'),
            }
            s
        })
    }
    fn long_option(&self) -> Option<from_glibc::LongOption> {
        self.shadow_name.as_ref().map(|v| from_glibc::LongOption {
            name: v.as_ptr(),
            has_arg: match self.has_arg {
                HasArg::NoArgument => from_glibc::LongOption::NO_ARGUMENT,
                HasArg::RequiredArgument => from_glibc::LongOption::REQUIRED_ARGUMENT,
                HasArg::OptionalArgument => from_glibc::LongOption::OPTIONAL_ARGUMENT,
            },
            flag: std::ptr::null_mut(),
            val: 0,
        })
    }
}
impl std::fmt::Display for Opt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<5}{:<12}{:<15} {}",
            self.short_name
                .map(|v| format!("-{},", v))
                .unwrap_or(String::new()),
            self.long_name
                .as_ref()
                .map(|v| format!("--{},", v))
                .unwrap_or(String::new()),
            match self.has_arg {
                HasArg::NoArgument => "",
                HasArg::OptionalArgument => "[=Arg]",
                HasArg::RequiredArgument => " =Arg ",
            },
            self.desc
        )
    }
}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub value: Option<String>,
}
impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "option: {}, value: {}",
            self.name,
            self.value.as_ref().unwrap_or(&String::new())
        )
    }
}

pub struct Parser {
    pub args: Vec<Arg>,
    pub operands: Vec<String>,
}
impl std::fmt::Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for arg in self.args.iter() {
            writeln!(f, "{}", arg)?;
        }
        for (i, v) in self.operands.iter().enumerate() {
            writeln!(f, "operand{}: {}", i, v)?;
        }
        Ok(())
    }
}

// --

pub fn getopt_long(opts: &[Opt]) -> OptResult<Parser> {
    let mut optstring = ":".to_owned();
    let mut longopts = Vec::new();
    opts.iter().for_each(|v| {
        if let Some(s) = v.optstring() {
            optstring.push_str(&s);
        }
        if let Some(lo) = v.long_option() {
            longopts.push(lo);
        }
    });
    let optstring = CString::new(optstring).map_err(|e| e.to_string())?;
    longopts.push(from_glibc::LongOption {
        name: std::ptr::null(),
        has_arg: 0,
        flag: std::ptr::null_mut(),
        val: 0,
    });
    let mut longindex: c_int = 0;

    // create a vector of zero terminated strings
    let mut argv = Vec::new();
    for v in env::args() {
        argv.push(CString::new(v).map_err(|e| OptError::Other(e.to_string()))?);
    }
    // convert the strings to raw pointers
    let mut argv = argv
        .into_iter()
        .map(|arg| arg.into_raw())
        .collect::<Vec<*mut c_char>>();
    let argc = argv.len() as c_int;
    let mut args = Vec::new();
    loop {
        match unsafe {
            from_glibc::getopt_long(
                argc,
                argv.as_ptr(),
                optstring.as_ptr(),
                longopts.as_ptr(),
                &mut longindex,
            )
        } {
            -1 => break,
            0 => {
                let longopt = &longopts[longindex as usize];
                args.push(Arg {
                    name: unsafe { CStr::from_ptr(longopt.name) }
                        .to_str()
                        .map_err(|e| e.to_string())?
                        .to_string(),
                    value: if unsafe { from_glibc::optarg.is_null() } {
                        None
                    } else {
                        Some(
                            unsafe { CStr::from_ptr(from_glibc::optarg) }
                                .to_str()
                                .map_err(|e| e.to_string())?
                                .to_string(),
                        )
                    },
                });
            }
            i => {
                let optopt = unsafe {
                    CStr::from_ptr(
                        *argv
                            .get(from_glibc::optind as usize - 1)
                            .ok_or(OptError::Other("optopt invalid.".to_owned()))?,
                    )
                }
                .to_str()
                .map_err(|e| e.to_string())?
                .to_string();
                if i == b'?' as c_int {
                    Err(OptError::InvalidOption(optopt.clone()))?;
                }
                if i == b':' as c_int {
                    Err(OptError::MissingOptionArgument(optopt.clone()))?;
                }

                args.push(Arg {
                    name: CStr::from_bytes_with_nul(&[i as u8, 0])
                        .map_err(|e| e.to_string())?
                        .to_str()
                        .map_err(|e| e.to_string())?
                        .to_string(),
                    value: if unsafe { from_glibc::optarg.is_null() } {
                        None
                    } else {
                        Some(
                            unsafe { CStr::from_ptr(from_glibc::optarg) }
                                .to_str()
                                .map_err(|e| e.to_string())?
                                .to_string(),
                        )
                    },
                });
            }
        }
    }

    let mut operands = Vec::new();
    for v in argv.split_off(unsafe { from_glibc::optind } as usize) {
        operands.push(
            unsafe { CStr::from_ptr(v) }
                .to_str()
                .map_err(|e| e.to_string())?
                .to_string(),
        );
    }
    Ok(Parser { args, operands })
}

pub fn usage(name: &str, desc: &str, opts: &[Opt]) {
    println!(
        "
Usage:
    {} [options [args]] [operands]
    {}
",
        name, desc
    );
    opts.iter().for_each(|v| println!("{:4}{}", " ", v));
    println!();
}
