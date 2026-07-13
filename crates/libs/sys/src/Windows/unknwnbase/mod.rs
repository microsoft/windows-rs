windows_link::link!("rpcrt4.dll" "system" fn IUnknown_AddRef_Proxy(this : *mut core::ffi::c_void) -> u32);
windows_link::link!("rpcrt4.dll" "system" fn IUnknown_QueryInterface_Proxy(this : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("rpcrt4.dll" "system" fn IUnknown_Release_Proxy(this : *mut core::ffi::c_void) -> u32);
