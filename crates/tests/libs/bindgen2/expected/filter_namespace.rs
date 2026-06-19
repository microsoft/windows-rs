pub mod Test {
    pub mod One {
        #[inline]
        pub unsafe fn First() -> u32 {
            windows_core::link!("test.dll" "system" fn First() -> u32);
            unsafe { First() }
        }
    }
    pub mod Two {
        #[inline]
        pub unsafe fn Second() -> u32 {
            windows_core::link!("test.dll" "system" fn Second() -> u32);
            unsafe { Second() }
        }
    }
}
