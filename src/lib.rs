// -- lib.rs --

mod getopt_long;
mod my_glibc;

// --

pub use getopt_long::{getopt_long, usage, HasArg, Opt, OptError, OptResult, Arguments};
