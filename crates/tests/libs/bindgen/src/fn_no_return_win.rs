#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn FatalExit(exitcode: i32) -> ! {
    windows_core::link!("kernel32.dll" "system" fn FatalExit(exitcode : i32) -> !);
    unsafe { FatalExit(exitcode) }
}
