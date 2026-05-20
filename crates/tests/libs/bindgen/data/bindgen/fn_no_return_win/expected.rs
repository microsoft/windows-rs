#[inline]
pub unsafe fn FatalExit(exitcode: i32) -> ! {
    windows_core::link!("kernel32.dll" "system" fn FatalExit(exitcode : i32) -> !);
    unsafe { FatalExit(exitcode) }
}
