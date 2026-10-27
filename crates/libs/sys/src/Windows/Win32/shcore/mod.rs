#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateRandomAccessStreamOnFile(filepath : windows_sys::core::PCWSTR, accessmode : super::DWORD, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidlbase"))]
windows_link::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateRandomAccessStreamOverStream(stream : *mut core::ffi::c_void, options : BSOS_OPTIONS, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateStreamOverRandomAccessStream(randomaccessstream : *mut core::ffi::c_void, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const BSOS_DEFAULT: BSOS_OPTIONS = 0;
pub type BSOS_OPTIONS = i32;
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = 1;
