#![allow(dead_code)]

mod error;
mod guid;
mod runtime;
mod string;

pub use error::*;
pub use guid::*;
pub use runtime::*;
pub use string::*;
pub use winrt_macros::*;

pub fn init() {
    let mut cookie: *mut Void = std::ptr::null_mut();
    unsafe { CoIncrementMTAUsage(&mut cookie).unwrap() };
}
