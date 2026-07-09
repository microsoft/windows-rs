#[cfg(all(feature = "Win32_mfidl", feature = "Win32_mfobjects", feature = "Win32_objidlbase"))]
windows_link::link!("mf.dll" "system" fn MFCreateEncryptedMediaExtensionsStoreActivate(pmphost : *mut core::ffi::c_void, objectstream : *mut core::ffi::c_void, classid : windows_sys::core::PCWSTR, activate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
