#[cfg(feature = "Win32_objidlbase")]
windows_link::link!("api-ms-win-core-winrt-robuffer-l1-1-0.dll" "system" fn RoGetBufferMarshaler(buffermarshaler : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
