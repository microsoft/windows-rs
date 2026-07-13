#[inline]
pub unsafe fn Beep(dwfreq: u32, dwduration: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Beep(dwfreq : u32, dwduration : u32) -> windows_core::BOOL);
    unsafe { Beep(dwfreq, dwduration) }
}
#[inline]
pub unsafe fn DecodePointer(ptr: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn DecodePointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { DecodePointer(ptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DecodeRemotePointer(processhandle: super::winnt::HANDLE, ptr: Option<*const core::ffi::c_void>, decodedptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-util-l1-1-1.dll" "system" fn DecodeRemotePointer(processhandle : super::winnt::HANDLE, ptr : *const core::ffi::c_void, decodedptr : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DecodeRemotePointer(processhandle, ptr.unwrap_or(core::mem::zeroed()) as _, decodedptr as _) }
}
#[inline]
pub unsafe fn DecodeSystemPointer(ptr: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn DecodeSystemPointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { DecodeSystemPointer(ptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EncodePointer(ptr: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn EncodePointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { EncodePointer(ptr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EncodeRemotePointer(processhandle: super::winnt::HANDLE, ptr: Option<*const core::ffi::c_void>, encodedptr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-util-l1-1-1.dll" "system" fn EncodeRemotePointer(processhandle : super::winnt::HANDLE, ptr : *const core::ffi::c_void, encodedptr : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { EncodeRemotePointer(processhandle, ptr.unwrap_or(core::mem::zeroed()) as _, encodedptr as _) }
}
#[inline]
pub unsafe fn EncodeSystemPointer(ptr: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn EncodeSystemPointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { EncodeSystemPointer(ptr.unwrap_or(core::mem::zeroed()) as _) }
}
