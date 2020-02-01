// -- lib.rs --

#[macro_export]
macro_rules! on_the_spot {
    () => {
        SpotError::new(file!(), line()!, "");
    };
    ($val:expr) => {
        SpotError::new(file!(), line!(), $val);
    };
}

#[macro_export]
macro_rules! output {
    () => {
        println!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        match $val {
            tmp => {
                println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), tmp);
            }
        }
    };
    ($val:expr,) => { output!($val) };
    ($($val:expr),+ $(,)?) => {
        ($(output!($val)),+,)
    };
}

// --

mod spot_error;
mod getopt_long;
mod getopt_long2;

// --

pub use spot_error::{SpotError, SpotResult};
// pub use getopt_long::{getopt_long, usage, Opt, HasArg, Parser, OptError, OptResult};
pub use getopt_long2::{getopt_long, usage, Opt, HasArg, Parser, OptError, OptResult};

// --

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
