extern crate cc;

fn main() {
    // let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/glibc/getopt.c");
    println!("cargo:rerun-if-changed=src/glibc/getopt_init.c");
    // println!("cargo:rustc-link-search=native={}/../target/debug/examples", manifest_dir);

    // std::process::Command::new("env").status().unwrap();

    // std::process::Command::new("gcc")
    //     .current_dir(out_dir)
    //     .arg(format!("{}/examples/call_from_c/main.c", manifest_dir))
    //     .args(&["-lcall_from_c","-o"])
    //     .arg(format!("{}/../target/debug/examples/call_rust", manifest_dir))
    //     .arg(format!("-L{}/../target/debug/examples", manifest_dir))
    //     .status()
    //     .expect("failed to execute process");

    cc::Build::new().file("src/glibc/getopt_init.c").file("src/glibc/getopt.c")
        .compile("my_glibc");
}
