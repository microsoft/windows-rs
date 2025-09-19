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
