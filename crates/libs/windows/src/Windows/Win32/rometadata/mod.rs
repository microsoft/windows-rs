#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("rometadata.dll" "system" fn MetaDataGetDispenser(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MetaDataGetDispenser(rclsid, riid, ppv as _) }
}
