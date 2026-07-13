windows_link::link!("dxcore.dll" "system" fn DXCoreCreateAdapterFactory(riid : *const windows_sys::core::GUID, ppvfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
