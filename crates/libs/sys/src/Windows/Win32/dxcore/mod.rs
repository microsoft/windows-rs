#[cfg(feature = "guiddef")]
windows_link::link!("dxcore.dll" "system" fn DXCoreCreateAdapterFactory(riid : *const super::IID, ppvfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
