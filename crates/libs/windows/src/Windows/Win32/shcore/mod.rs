#[inline]
pub unsafe fn CreateRandomAccessStreamOnFile<P0, T>(filepath: P0, accessmode: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateRandomAccessStreamOnFile(filepath : windows_core::PCWSTR, accessmode : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateRandomAccessStreamOnFile(filepath.param().abi(), accessmode, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn CreateRandomAccessStreamOverStream<P0, T>(stream: P0, options: BSOS_OPTIONS) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateRandomAccessStreamOverStream(stream : *mut core::ffi::c_void, options : BSOS_OPTIONS, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateRandomAccessStreamOverStream(stream.param().abi(), options, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateStreamOverRandomAccessStream<P0, T>(randomaccessstream: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateStreamOverRandomAccessStream(randomaccessstream : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateStreamOverRandomAccessStream(randomaccessstream.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
pub const BSOS_DEFAULT: BSOS_OPTIONS = 0;
pub type BSOS_OPTIONS = i32;
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = 1;
