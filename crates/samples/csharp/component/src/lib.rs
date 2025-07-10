#[no_mangle]
unsafe extern "system" fn Add(left: u32, right: u32, result: *mut u32) -> i32 {
    *result = left + right;
    0 // S_OK
}
