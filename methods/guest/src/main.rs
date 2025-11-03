#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use std::ffi::{c_int, c_void};

mod malloc;

extern "C" {
    fn free(ptr: *mut c_void);
}

extern "C" {
    fn lean_risc0_main(
        input: *const u8,
        input_length: usize,
        output: *mut *const u8,
        output_length: *mut usize,
    ) -> c_int;
}

/// Safe wrapper: get a Vec<u8> from the C API.
fn safe_lean_risc0_main(input: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let mut len: usize = 0;
    let mut ptr: *const u8 = std::ptr::null();

    let res = unsafe {
        lean_risc0_main(
            input.as_ptr(),
            input.len(),
            &mut ptr as *mut *const u8,
            &mut len as *mut usize,
        )
    };

    if res != 0 {
        return Err("lean_risc0_main failed");
    }

    if ptr.is_null() {
        return Err("lean_risc0_main returned NULL");
    }

    // If length is zero, don't form a slice from a NULL pointer!
    // from_raw_parts(NULL, 0) is UB, so special-case it.
    if len == 0 {
        unsafe { free(ptr as *mut c_void) };
        return Ok(Vec::new());
    }

    if len > isize::MAX as usize {
        unsafe { free(ptr as *mut c_void) };
        return Err("buffer length too large");
    }

    // SAFETY:
    // - `ptr` is valid for reads of `len` bytes (guaranteed by the C function).
    // - data is not mutated while we read.
    // - `len > 0` and ptr is non-NULL here.
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };

    let vec = slice.to_vec();

    unsafe { free(ptr as *mut c_void) };

    Ok(vec)
}

fn main() {
    let input: u32 = env::read();

    let result: Vec<u8> = safe_lean_risc0_main(input.to_string().into_bytes()).unwrap();

    let value: u32 = String::from_utf8(result)
        .expect("Invalid UTF-8 from Lean")
        .trim()
        .parse::<u32>()
        .expect("Failed to parse output from Lean as u32");

    env::commit(&value);
}
