windows_link::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoFreeParameterizedTypeExtra(extra : ROPARAMIIDHANDLE));
windows_link::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoGetParameterizedTypeInstanceIID(nameelementcount : u32, nameelements : *const windows_sys::core::PCWSTR, metadatalocator : *mut core::ffi::c_void, iid : *mut windows_sys::core::GUID, pextra : *mut ROPARAMIIDHANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoParameterizedTypeExtraGetTypeSignature(extra : ROPARAMIIDHANDLE) -> windows_sys::core::PCSTR);
pub type ROPARAMIIDHANDLE = *mut core::ffi::c_void;
