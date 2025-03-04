#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("kernel32.dll" "system" fn FatalExit(exitcode : i32) -> !);
windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
pub const ERROR_OUTOFMEMORY: WIN32_ERROR = 14u32;
pub const E_FAIL: HRESULT = 0x80004005_u32 as _;
pub const E_OUTOFMEMORY: HRESULT = 0x8007000E_u32 as _;
pub type HRESULT = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
pub const S_FALSE: HRESULT = 0x1_u32 as _;
pub type WIN32_ERROR = u32;
