// -- getopt_long.rs --

use {
    crate::my_glibc,
    std::{
        collections::HashMap,
        env,
        ffi::{CStr, CString, NulError},
        os::raw::{c_char, c_int},
    },
};

// --

#[derive(Debug)]
pub enum OptError {
    InvalidOption(String),
    MissingOptionArgument(String),
    InvalidOptind(i32),
}

impl std::error::Error for OptError {}

impl std::fmt::Display for OptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OptError::InvalidOption(s) => format!("invalid option: {}", s),
                OptError::MissingOptionArgument(s) => format!("missing option argument: {}", s),
                OptError::InvalidOptind(i) => format!("invalid optind: {}", i),
            }
        )
    }
}

pub type OptResult<T> = Result<T, Box<dyn std::error::Error>>;

// --

#[repr(i32)]
pub enum HasArg {
    NoArgument = 0,
    RequiredArgument,
    OptionalArgument,
}

pub struct Opt {
    long_name: Option<CString>,
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
        Ok(Self {
            long_name: long_name.map(|v| CString::new(v)).transpose()?,
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
    fn long_option(&self) -> Option<my_glibc::LongOption> {
        self.long_name.as_ref().map(|v| my_glibc::LongOption {
            name: v.as_ptr(),
            has_arg: match self.has_arg {
                HasArg::NoArgument => my_glibc::LongOption::NO_ARGUMENT,
                HasArg::RequiredArgument => my_glibc::LongOption::REQUIRED_ARGUMENT,
                HasArg::OptionalArgument => my_glibc::LongOption::OPTIONAL_ARGUMENT,
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
            "{:<2}{:2}{:<16}{:<10} {}",
            self.short_name
                .map(|v| format!("-{}", v))
                .unwrap_or(String::new()),
            if self.short_name.is_none() || self.long_name.is_none() {
                ""
            } else {
                ", "
            },
            self.long_name
                .as_ref()
                .map(|v| format!("--{}", v.to_str().unwrap_or("")))
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
pub struct Arguments {
    pub args: HashMap<String, String>,
    pub operands: Vec<String>,
}
impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for a in self.args.iter() {
            writeln!(f, "{}: {}", a.0, a.1)?;
        }
        for o in self.operands.iter() {
            writeln!(f, "operand: {}", o)?;
        }
        Ok(())
    }
}

// --

pub fn getopt_long(opts: &[Opt]) -> OptResult<Arguments> {
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
    let optstring = CString::new(optstring)?;
    longopts.push(my_glibc::LongOption {
        name: std::ptr::null(),
        has_arg: 0,
        flag: std::ptr::null_mut(),
        val: 0,
    });
    let mut longindex: c_int = 0;

    // create a vector of zero terminated strings
    let mut argv = Vec::new();
    for v in env::args() {
        argv.push(CString::new(v)?);
    }
    // convert the strings to raw pointers
    let argv = argv
        .into_iter()
        .map(|arg| arg.into_raw())
        .collect::<Vec<*mut c_char>>();
    let mut argv = recover_guard::RecoverGuard::new(argv, |a| {
        // 回收传入c函数中的资源
        a.into_iter().for_each(|v| unsafe {
            CString::from_raw(v);
        })
    });
    let argc = argv.len() as c_int;

    let mut args = HashMap::new();
    loop {
        match unsafe {
            my_glibc::getopt_long(
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
                args.insert(
                    unsafe { CStr::from_ptr(longopt.name) }
                        .to_str()?
                        .to_string(),
                    if unsafe { my_glibc::optarg.is_null() } {
                        String::new()
                    } else {
                        unsafe { CStr::from_ptr(my_glibc::optarg) }
                            .to_str()?
                            .to_string()
                    },
                );
            }
            i => {
                let optopt = unsafe {
                    CStr::from_ptr(
                        *argv
                            .get(my_glibc::optind as usize - 1)
                            .ok_or(OptError::InvalidOptind(my_glibc::optind))?,
                    )
                }
                .to_str()?
                .to_string();
                if i == b'?' as c_int {
                    Err(OptError::InvalidOption(optopt.clone()))?;
                }
                if i == b':' as c_int {
                    Err(OptError::MissingOptionArgument(optopt.clone()))?;
                }

                args.insert(
                    CStr::from_bytes_with_nul(&[i as u8, 0])?
                        .to_str()?
                        .to_string(),
                    if unsafe { my_glibc::optarg.is_null() } {
                        String::new()
                    } else {
                        unsafe { CStr::from_ptr(my_glibc::optarg) }
                            .to_str()?
                            .to_string()
                    },
                );
            }
        }
    }

    let mut operands = Vec::new();
    for v in argv.split_off(unsafe { my_glibc::optind } as usize) {
        operands.push(unsafe { CStr::from_ptr(v) }.to_str()?.to_string());
    }

    Ok(Arguments { args, operands })
}

pub fn usage(name: &str, desc: &str, version: &str, opts: &[Opt]) {
    println!(
        r#"Description:
    {}
Version:
    {}
Usage:
    {} [options [args]] [operands]
Options:"#,
        desc, version, name
    );
    opts.iter().for_each(|v| println!("    {}", v));
    println!();
}

// --

mod recover_guard {
    use std::{
        boxed::Box,
        ops::{Deref, DerefMut, Drop, FnMut},
    };

    pub(crate) struct RecoverGuard<T, F: FnMut(T)> {
        data: Option<T>,
        func: Box<F>,
    }

    impl<T: Sized, F: FnMut(T)> RecoverGuard<T, F> {
        pub fn new(data: T, func: F) -> RecoverGuard<T, F> {
            RecoverGuard {
                data: Some(data),
                func: Box::new(func),
            }
        }
    }

    impl<T, F: FnMut(T)> Deref for RecoverGuard<T, F> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.data.as_ref().expect("the data should be here")
        }
    }

    impl<T, F: FnMut(T)> DerefMut for RecoverGuard<T, F> {
        fn deref_mut(&mut self) -> &mut T {
            self.data.as_mut().expect("the data should be here")
        }
    }

    impl<T, F: FnMut(T)> Drop for RecoverGuard<T, F> {
        fn drop(&mut self) {
            // let mut data: Option<T> = None;
            // std::mem::swap(&mut data, &mut self.data);
            let data = self.data.take();
            let ref mut f = self.func;
            f(data.expect("the data is here until the drop"));
        }
    }
}
