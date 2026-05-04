#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn EnableMouseInPointer(fenable: bool) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EnableMouseInPointer(fenable.into()).ok() }
}
