fn main() {
    println!("Hello, world!");
    p62::hello_from_rust();

    let mut my_arr = [10, 20, 30, 40];
    let c_len = my_arr.len();
    let c_ptr = my_arr.as_mut_ptr();
    unsafe {
        p62::rust_print_array(c_ptr, c_len);
        p62::rust_update_array(c_ptr, c_len);
    }
}

// let key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
// let mut block: [u8; 16] = [
//     10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
// ];

// let exp_key: [u8; 16 * 11] = miniaes::expand_key(&key);

// miniaes::encrypt1(&exp_key, &mut block);
// miniaes::decrypt1(&exp_key, &mut block);
// println!("exp_key[0] = {:?}", block)
