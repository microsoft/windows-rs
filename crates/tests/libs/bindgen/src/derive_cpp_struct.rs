#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
impl Default for POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
impl Default for SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
