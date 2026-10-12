#[inline]
pub unsafe fn Kept() -> u32 {
    windows_core::link!("test.dll" "system" fn Kept() -> u32);
    unsafe { Kept() }
}
