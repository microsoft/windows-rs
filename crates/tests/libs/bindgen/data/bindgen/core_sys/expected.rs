pub mod Windows {
    pub mod Win32 {
        pub mod System {
            pub mod Com {
                pub type CoCreateGuid = unsafe extern "system" fn(
                    pguid: *mut windows_sys::core::GUID,
                )
                    -> windows_sys::core::HRESULT;
                windows_link::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
            }
        }
    }
}
