// #![allow(unused_variables)]
// #![allow(dead_code)]

mod runtime;
use std::io::*;
use runtime::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> std::io::Result<()> {
    unsafe {
        let hr = RoInitialize(ApartmentType::MultiThreaded);
        println!("{:#x}", hr);
        assert!(hr == 0);
        let hr = RoInitialize(ApartmentType::MultiThreaded);
        assert!(hr == 1);

        let mut hstring : PVOID = std::ptr::null_mut();
        let wchars : Vec<u16> = "hello".encode_utf16().collect();
        let hr = WindowsCreateString(wchars.as_ptr(), 5, &mut hstring);
        assert!(hr == 0);

        let mut length = 0;
        let raw = WindowsGetStringRawBuffer(hstring, &mut length);
        let value = String::from_utf16(std::slice::from_raw_parts(raw, length as usize)).unwrap();
        println!("{} : {} ({})", length, value, value.len());
    }
    Ok(())
}
