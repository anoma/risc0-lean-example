use risc0_zkvm::guest::env;

extern "C" {
    fn lean_risc0_main(
        input: *const u8,
        input_length: size_t,
        output: *mut *const u8,
        output_length: *mut size_t,
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
        return Err("foo_get_bytes returned NULL");
    }

    // If length is zero, don't form a slice from a NULL pointer!
    // from_raw_parts(NULL, 0) is UB, so special-case it.
    if len == 0 {
        unsafe { libc::free(ptr as *mut c_void) };
        return Ok(Vec::new());
    }

    if len > isize::MAX as usize {
        unsafe { libc::free(ptr as *mut c_void) };
        return Err("buffer length too large");
    }

    // SAFETY:
    // - `ptr` is valid for reads of `len` bytes (guaranteed by the C function).
    // - data is not mutated while we read.
    // - `len > 0` and ptr is non-NULL here.
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };

    let vec = slice.to_vec();

    unsafe { libc::free(ptr as *mut c_void) };

    Ok(vec)
}

fn main() {
    let input: u32 = env::read();

    let result: Vec<u8> =
        safe_lean_risc0_main(input.to_string().into_bytes()).expect("lean_risc0_main failed");

    let value: u32 = String::from_utf8(result)
        .expect("Invalid UTF-8 from Lean")
        .trim()
        .parse()
        .expect("Failed to parse output from Lean as u32");

    env::commit(&value);
}
