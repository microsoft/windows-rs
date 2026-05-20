#[inline]
pub unsafe fn GetTickCount() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
    unsafe { GetTickCount() }
}
