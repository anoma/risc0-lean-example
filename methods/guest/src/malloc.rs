extern crate alloc;
use core::alloc::Layout;
use core::{mem, ptr};

/// 8 is a safe choice for RV32 and matches typical max_align_t on 32-bit.
/// If you know you need larger, bump this to 16.
const C_ALIGN: usize = 8;

/// Simple header that stores the allocation size.
/// (Alignment is that of usize.)
#[repr(C)]
struct Header {
    size: usize,
}

#[inline]
fn header_layout(payload_size: usize) -> (Layout, usize) {
    // We allocate "header + payload", aligned to max(header align, C_ALIGN).
    let header_align = mem::align_of::<Header>();
    let align = header_align.max(C_ALIGN);
    // Reserve space for header, then round up header size to alignment.
    let header_size = mem::size_of::<Header>();
    // Add padding so the payload starts aligned to `align`.
    let padding = (align - (header_size % align)) % align;
    let total = header_size + padding + payload_size.max(1); // malloc(0) -> 1 byte
    let layout = Layout::from_size_align(total, align).unwrap();
    (layout, header_size + padding)
}

#[inline]
unsafe fn write_header(ptr: *mut u8, payload_size: usize) -> *mut u8 {
    // Layout: [header | padding | payload]
    let header = ptr.cast::<Header>();
    (*header).size = payload_size;
    // Move to payload start (after header+padding)
    let (_, off) = header_layout(payload_size);
    ptr.add(off)
}

#[inline]
unsafe fn header_from_payload(payload: *mut u8) -> (*mut Header, Layout, usize) {
    // Recompute the same offset to reach header.
    // We must know the stored size to rebuild the Layout.
    // First, step back conservatively to find header by recomputing padding using stored size later.
    // We stored size directly in header at a fixed distance: header is immediately before "off".
    // Compute off for all possible sizes? Not needed: we always compute off from stored size.
    // So first hop back to read size using the worst-case assumption: header is <= (size_of::<Header>() + C_ALIGN) behind.
    // But since we don't know size yet, we read at (payload - (size_of::<Header>())) which is aligned for Header.
    let header = payload.sub(mem::size_of::<Header>()).cast::<Header>();
    let size = (*header).size;
    let (layout, off) = header_layout(size);
    let real_header = payload.sub(off).cast::<Header>();
    (real_header, layout, size)
}

// ---- C ABI shims ---------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let (layout, _) = header_layout(size);
    let raw = alloc::alloc::alloc(layout);
    if raw.is_null() {
        ptr::null_mut()
    } else {
        write_header(raw, size)
    }
}

#[no_mangle]
pub unsafe extern "C" fn calloc(nmemb: usize, size: usize) -> *mut u8 {
    match nmemb.checked_mul(size) {
        Some(total) => {
            let (layout, _) = header_layout(total);
            let raw = alloc::alloc::alloc_zeroed(layout);
            if raw.is_null() {
                ptr::null_mut()
            } else {
                write_header(raw, total)
            }
        }
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr_: *mut u8) {
    if ptr_.is_null() {
        return;
    }
    let (_hdr, layout, _size) = header_from_payload(ptr_);
    alloc::alloc::dealloc(ptr_.sub(layout.size() - _size), layout);
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr_: *mut u8, new_size: usize) -> *mut u8 {
    if ptr_.is_null() {
        return malloc(new_size);
    }
    if new_size == 0 {
        free(ptr_);
        return ptr::null_mut();
    }
    // Rebuild original layout from header, then reallocate.
    let (_hdr, old_layout, _old_size) = header_from_payload(ptr_);
    let (new_layout, off_new) = header_layout(new_size);

    // Because GlobalAlloc::realloc needs the *original* layout, we call the raw API:
    let base = ptr_.sub(old_layout.size() - _old_size); // start of old alloc (header)
    let new_base = alloc::alloc::realloc(base, old_layout, new_layout.size());
    if new_base.is_null() {
        ptr::null_mut()
    } else {
        // Update header with new size and return payload
        write_header(new_base, new_size);
        new_base.add(off_new)
    }
}
