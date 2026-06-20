#[inline]
pub unsafe fn Function() -> u32 {
    windows_core::link!("test.dll" "system" fn Function() -> u32);
    unsafe { Function() }
}
