#[cfg(feature = "shared")]
windows_link::link!("rpcrt4.dll" "system" fn IUnknown_AddRef_Proxy(this : *mut core::ffi::c_void) -> super::ULONG);
#[cfg(feature = "guiddef")]
windows_link::link!("rpcrt4.dll" "system" fn IUnknown_QueryInterface_Proxy(this : *mut core::ffi::c_void, riid : *const super::IID, ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rpcrt4.dll" "system" fn IUnknown_Release_Proxy(this : *mut core::ffi::c_void) -> super::ULONG);
pub type LPCLASSFACTORY = *mut core::ffi::c_void;
pub type LPUNKNOWN = *mut core::ffi::c_void;
