#[inline]
pub unsafe fn Fixed(count: u32) -> u32 {
    windows_core::link!("test.dll" "system" fn Fixed(count : u32) -> u32);
    unsafe { Fixed(count) }
}
