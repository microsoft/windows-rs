#![allow(dead_code)]

pub mod abi;
mod activation;
mod error;
mod guid;
mod runtime;
mod string;
mod traits;

//pub use abi::*;
pub use activation::*;
pub use error::*;
pub use guid::*;
pub use runtime::*;
pub use string::*;
pub use traits::*;
pub use winrt_macros::*;

use abi::Void;

pub fn init() {
    let mut cookie = abi::Void::null_mut();
    unsafe { CoIncrementMTAUsage(&mut cookie).unwrap() };
}
