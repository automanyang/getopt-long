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
    fn long_option(&self) -> Option<my_glibc::LongOption> {
        self.shadow_name.as_ref().map(|v| my_glibc::LongOption {
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
            "{:<4}{:<20}{:<10} {}",
            self.short_name
                .map(|v| format!("-{},", v))
                .unwrap_or(String::new()),
            self.long_name
                .as_ref()
                .map(|v| format!("--{}", v))
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
    let optstring = CString::new(optstring).map_err(|e| e.to_string())?;
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
        argv.push(CString::new(v).map_err(|e| OptError::Other(e.to_string()))?);
    }
    // convert the strings to raw pointers
    let mut argv = argv
        .into_iter()
        .map(|arg| arg.into_raw())
        .collect::<Vec<*mut c_char>>();
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
                        .to_str()
                        .map_err(|e| e.to_string())?
                        .to_string(),
                    if unsafe { my_glibc::optarg.is_null() } {
                        String::new()
                    } else {
                        unsafe { CStr::from_ptr(my_glibc::optarg) }
                            .to_str()
                            .map_err(|e| e.to_string())?
                            .to_string()
                    },
                );
            }
            i => {
                let optopt = unsafe {
                    CStr::from_ptr(
                        *argv
                            .get(my_glibc::optind as usize - 1)
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

                args.insert(
                    CStr::from_bytes_with_nul(&[i as u8, 0])
                        .map_err(|e| e.to_string())?
                        .to_str()
                        .map_err(|e| e.to_string())?
                        .to_string(),
                    if unsafe { my_glibc::optarg.is_null() } {
                        String::new()
                    } else {
                        unsafe { CStr::from_ptr(my_glibc::optarg) }
                            .to_str()
                            .map_err(|e| e.to_string())?
                            .to_string()
                    },
                );
            }
        }
    }

    let mut operands = Vec::new();
    for v in argv.split_off(unsafe { my_glibc::optind } as usize) {
        operands.push(
            unsafe { CStr::from_ptr(v) }
                .to_str()
                .map_err(|e| e.to_string())?
                .to_string(),
        );
    }
    Ok(Arguments { args, operands })
}

pub fn usage(name: &str, desc: &str, version: &str, opts: &[Opt]) {
    println!(
        "Description:
    {}
Version: 
    {}
Usage:
    {} [options [args]] [operands]
Options:",
        desc, version, name
    );
    opts.iter().for_each(|v| println!("    {}", v));
    println!();
}
