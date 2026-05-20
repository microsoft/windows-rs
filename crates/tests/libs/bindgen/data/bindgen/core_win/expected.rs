pub mod Windows {
    pub mod Win32 {
        pub mod System {
            pub mod Com {
                #[inline]
                pub unsafe fn CoCreateGuid() -> windows_core::Result<windows_core::GUID> {
                    windows_core::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        CoCreateGuid(&mut result__).map(|| result__)
                    }
                }
            }
        }
    }
}
