#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn CoGetCallerTID() -> windows_result::Result<u32> {
    windows_link::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : *mut u32) -> windows_result::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetCallerTID(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IsCharLowerA(ch: i8) -> windows_result::Result<()> {
    windows_link::link!("user32.dll" "system" fn IsCharLowerA(ch : i8) -> windows_result::BOOL);
    unsafe { IsCharLowerA(ch).ok() }
}
#[inline]
pub unsafe fn SysFreeString(bstrstring: &windows_strings::BSTR) {
    windows_link::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : * mut core::ffi::c_void));
    unsafe { SysFreeString(core::mem::transmute_copy(bstrstring)) }
}
#[inline]
pub unsafe fn WindowsGetStringRawBuffer(
    string: &windows_strings::HSTRING,
    length: Option<*mut u32>,
) -> windows_strings::PCWSTR {
    windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringRawBuffer(string : * mut core::ffi::c_void, length : *mut u32) -> windows_strings::PCWSTR);
    unsafe {
        WindowsGetStringRawBuffer(
            core::mem::transmute_copy(string),
            length.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn WindowsStringHasEmbeddedNull(
    string: &windows_strings::HSTRING,
) -> windows_result::Result<windows_result::BOOL> {
    windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsStringHasEmbeddedNull(string : * mut core::ffi::c_void, hasembednull : *mut windows_result::BOOL) -> windows_result::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsStringHasEmbeddedNull(core::mem::transmute_copy(string), &mut result__)
            .map(|| result__)
    }
}
