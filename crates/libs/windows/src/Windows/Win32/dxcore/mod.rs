#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn DXCoreCreateAdapterFactory(riid: *const super::IID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("dxcore.dll" "system" fn DXCoreCreateAdapterFactory(riid : *const super::IID, ppvfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DXCoreCreateAdapterFactory(riid as _, ppvfactory as _) }
}
