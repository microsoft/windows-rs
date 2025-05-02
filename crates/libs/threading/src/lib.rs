#![allow(non_camel_case_types, dead_code, non_snake_case)]
#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![no_std]

mod bindings;
use bindings::*;

mod pool;
pub use pool::*;

mod priority;
pub use priority::*;

extern crate alloc;
use alloc::boxed::Box;
use core::ffi::c_void;

pub fn submit<F: FnOnce() + Send + 'static>(f: F) {
    try_submit(core::ptr::null(), f)
}

fn check<D: Default + PartialEq>(result: D) -> D {
    if result == D::default() {
        panic!("allocation failed");
    }

    result
}

fn try_submit<F: FnOnce() + Send + 'static>(environment: *const TP_CALLBACK_ENVIRON_V3, f: F) {
    unsafe extern "system" fn callback<F: FnOnce() + Send + 'static>(
        _: *const c_void,
        callback: *const c_void,
    ) {
        unsafe {
            Box::from_raw(callback as *mut F)();
        }
    }

    unsafe {
        check(TrySubmitThreadpoolCallback(
            callback::<F>,
            Box::into_raw(Box::new(f)) as _,
            environment,
        ));
    }
}

fn thread_id()
fn thread_model()