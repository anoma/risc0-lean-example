use std::path::PathBuf;

fn main() {
    // Link search path for the current target
    let lean_risc0_path = std::env::var("LEAN_RISC0_PATH").unwrap();
    let lean_libdir = PathBuf::from(lean_risc0_path.clone()).join("lib");
    let includedir = PathBuf::from(lean_risc0_path).join("include");

    // Point rustc to the prebuilt archive and link it statically
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-search=native={}", lean_libdir.display());
    println!("cargo:rustc-link-lib=static=nosys");
    println!("cargo:rustc-link-lib=static=Lean");
    println!("cargo:rustc-link-lib=static=Init");
    println!("cargo:rustc-link-lib=static=Guest");

    // The cc crate will be invoked for the guest target set by RISC0’s build flow.
    cc::Build::new()
        .include(includedir.display().to_string())
        .file("risc0_lean.c")
        .flag("-DNDEBUG")
        .flag_if_supported("-O3")
        .compile("c_risc0_lean");
}
