use core::ffi::c_int;
use std::slice;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("CAPI: Hello from Rust!");
}

#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[no_mangle]
/// # Safety
/// This function is exposed as a C-API
pub unsafe extern "C" fn rust_print_array(ptr: *const c_int, len: libc::size_t) {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    for item in slice {
        print!("{item}");
    }
    println!()
}

#[no_mangle]
/// # Safety
/// This function is exposed as a C-API
pub unsafe extern "C" fn rust_update_array(ptr: *mut c_int, len: libc::size_t) {
    let slice = unsafe { slice::from_raw_parts_mut(ptr, len) };
    for item in slice {
        *item += 1;
    }
}
