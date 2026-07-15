#[inline]
pub unsafe fn GetMode() -> Mode {
    windows_core::link!("test.dll" "system" fn GetMode() -> Mode);
    unsafe { GetMode() }
}
pub type Mode = i32;
