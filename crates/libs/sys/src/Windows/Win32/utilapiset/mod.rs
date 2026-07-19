windows_link::link!("kernel32.dll" "system" fn Beep(dwfreq : u32, dwduration : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DecodePointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-util-l1-1-1.dll" "system" fn DecodeRemotePointer(processhandle : super::HANDLE, ptr : *const core::ffi::c_void, decodedptr : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn DecodeSystemPointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn EncodePointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-util-l1-1-1.dll" "system" fn EncodeRemotePointer(processhandle : super::HANDLE, ptr : *const core::ffi::c_void, encodedptr : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn EncodeSystemPointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
