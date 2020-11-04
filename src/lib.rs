extern crate self as winrt;

#[macro_export]
macro_rules! include_bindings {
    () => {
        include!(concat!(env!("OUT_DIR"), "\\winrt.rs"));
    };
}

// mod bindings {
//     #![allow(unused_variables)]
//     include_bindings!();
// }

// #[doc(hidden)]
// pub use bindings::windows::foundation;

#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;

#[doc(hidden)]
pub use const_sha1::ConstBuffer;

mod interfaces;
mod result;
mod runtime;
mod strings;
mod traits;

pub use interfaces::*;
pub use result::*;
pub use runtime::*;
pub use strings::*;
pub use traits::*;
