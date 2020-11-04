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

pub type RawPtr = *mut std::ffi::c_void;
pub type RawComPtr = std::ptr::NonNull<std::ffi::c_void>;

unsafe impl<T: ComInterface> GetAbi for Option<T> {
    type Abi = RawPtr;

    unsafe fn get_abi(&self) -> RawPtr {
        if let Some(interface) = self {
            interface.as_raw_ptr()
        } else {
            std::ptr::null_mut()
        }
    }
}

unsafe impl<T: ComInterface> SetAbi for Option<T> {
    type Abi = *mut RawPtr;

    unsafe fn set_abi(&mut self) -> *mut RawPtr {
        if let Some(this) = self {
            (this.vtable_of::<IUnknown>().2)(this.as_raw_com_ptr());
            *self = std::mem::zeroed();
        }
        self as *mut Self as *mut RawPtr
    }
}

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
