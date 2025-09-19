#[inline]
pub unsafe fn GetVersion() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetVersion() -> u32);
    unsafe { GetVersion() }
}
