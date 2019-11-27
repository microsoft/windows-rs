#![allow(dead_code)]

mod error;
mod guid;
mod runtime;
mod string;

pub use guid::*;
use runtime::*;
pub use string::*;

fn load_runtime() {
    let mut cookie: *mut VOID = std::ptr::null_mut();
    unsafe { CoIncrementMTAUsage(&mut cookie).unwrap() };
}

fn main() {
    // let hr = HRESULT(-2147024891i32);
    // hr.unwrap();

    load_runtime();

    let a: crate::string::String = crate::string::String::from("hello");

    let b: std::string::String = std::string::String::from(&a);

    println!("a: {}", a);
    println!("b: {}", b);

    let a = Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99");

    let b = Guid::from_values(0xCFF52E04, 0xCCA6, 0x4614, &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99]);

    assert!(a == b);
}
