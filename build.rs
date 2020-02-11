extern crate cc;

// --

fn main() {
    if !cfg!(target_os = "linux") {
        cc::Build::new()
            .include("src/my_glibc")
            .file("src/my_glibc/getopt.c")
            .compile("my_glibc");
    }
}
