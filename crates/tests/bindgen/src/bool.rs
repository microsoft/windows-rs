#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn EnableMouseInPointer(fenable: bool) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : windows::Win32::Foundation:: BOOL) -> windows::Win32::Foundation:: BOOL);
    EnableMouseInPointer(fenable.into()).ok()
}
