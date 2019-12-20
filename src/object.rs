#![allow(dead_code)]

use crate::*;

#[repr(C)]
pub struct Object {
    pub ptr: *const std::ffi::c_void,
}
