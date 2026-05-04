#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn SetConsoleCtrlHandler(
    handlerroutine: PHANDLER_ROUTINE,
    add: bool,
) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCtrlHandler(handlerroutine : PHANDLER_ROUTINE, add : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetConsoleCtrlHandler(handlerroutine, add.into()).ok() }
}
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(ctrltype: u32) -> windows_core::BOOL>;
