#[inline]
pub unsafe fn GetVersion() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetVersion() -> u32);
    unsafe { GetVersion() }
}
