use std::path::PathBuf;

fn main() {
    // Link search path for the current target
    let target = std::env::var("TARGET").unwrap();
    let libdir = PathBuf::from("lib").join(&target);

    // Point rustc to the prebuilt archive and link it statically
    println!("cargo:rustc-link-search=native={}", libdir.display());
    println!("cargo:rustc-link-lib=static=Lean");
    println!("cargo:rustc-link-lib=static=Init");
    println!("cargo:rustc-link-lib=static=guest");

    // The cc crate will be invoked for the guest target set by RISC0’s build flow.
    cc::Build::new()
        .include("include")
        .file("risc0_lean.c")
        .flag_if_supported("-O3")
        .compile("c_risc0_lean");
}
