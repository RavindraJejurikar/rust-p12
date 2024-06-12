// use libc::c_int;
use core::ffi::c_int;

#[link(name = "p62")]
extern "C" {
    fn hello_from_rust();
    fn rust_add(a: c_int, b: c_int) -> c_int;
    fn rust_print_array(ptr: *const c_int, len: usize);
    fn rust_update_array(ptr: *mut c_int, len: usize);
}

fn safe_rust_add(a: i32, b: i32) -> i32 {
    unsafe { rust_add(a, b) }
}

fn test_1() {
    println!("Hello, world!");
    unsafe {
        hello_from_rust();
    }
    let s = safe_rust_add(8, 8);
    println!("CLIB: sum = {s}");
}

fn main() {
    test_1();
    let mut my_arr = [10, 20, 30, 40];
    let c_len = my_arr.len();
    let c_ptr = my_arr.as_mut_ptr();

    unsafe {
        rust_print_array(c_ptr, c_len);
    }

    unsafe {
        rust_update_array(c_ptr, c_len);
        println!("{:?}", my_arr);
    }
}
