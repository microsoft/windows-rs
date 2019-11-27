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
