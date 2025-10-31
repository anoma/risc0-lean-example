use std::path::PathBuf;

fn main() {
    let lean_risc0_path = std::env::var("LEAN_RISC0_PATH").unwrap();
    let lean_libdir = PathBuf::from(lean_risc0_path.clone()).join("lib");
    let risc0_path = std::env::var("RISC0_TOOLCHAIN_PATH").unwrap();
    let risc0_libdir = PathBuf::from(risc0_path.clone())
        .join("riscv32-unknown-elf")
        .join("lib");
    let includedir = PathBuf::from(lean_risc0_path).join("include");

    println!("cargo::rerun-if-changed=risc0_lean.c");
    println!("cargo::rerun-if-changed=shims.c");
    println!("cargo::rerun-if-changed=lib");

    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-search=native={}", lean_libdir.display());
    println!("cargo:rustc-link-search=native={}", risc0_libdir.display());
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=Lean");
    println!("cargo:rustc-link-lib=static=Init");
    println!("cargo:rustc-link-lib=static=Guest");
    println!("cargo::rustc-link-arg-bins=--allow-multiple-definition");
    println!("cargo::rustc-link-arg-bins=--error-limit=0");

    cc::Build::new()
        .include(includedir.display().to_string())
        .file("shims.c")
        .file("risc0_lean.c")
        .flag("-DNDEBUG")
        .flag_if_supported("-O3")
        .compile("c_risc0_lean");
}
