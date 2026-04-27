pub mod Test {
    #[inline]
    pub unsafe fn GetValue() -> u32 {
        windows_link::link!("test.dll" "system" fn GetValue() -> u32);
        unsafe { GetValue() }
    }
    #[inline]
    pub unsafe fn SetValue(value: u32) {
        windows_link::link!("test.dll" "system" fn SetValue(value : u32));
        unsafe { SetValue(value) }
    }
}
