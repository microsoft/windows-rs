#[inline]
pub unsafe fn DXCoreCreateAdapterFactory<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxcore.dll" "system" fn DXCoreCreateAdapterFactory(riid : *const windows_core::GUID, ppvfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DXCoreCreateAdapterFactory(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
