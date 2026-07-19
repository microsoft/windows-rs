windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCompareStringOrdinal(string1 : windows_sys::core::HSTRING, string2 : windows_sys::core::HSTRING, result : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsConcatString(string1 : windows_sys::core::HSTRING, string2 : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateString(sourcestring : *const u16, length : u32, string : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
#[cfg(feature = "hstring")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateStringReference(sourcestring : windows_sys::core::PCWSTR, length : u32, hstringheader : *mut super::HSTRING_HEADER, string : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteString(string : windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
#[cfg(feature = "hstring")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteStringBuffer(bufferhandle : super::HSTRING_BUFFER) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDuplicateString(string : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringLen(string : windows_sys::core::HSTRING) -> u32);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringRawBuffer(string : windows_sys::core::HSTRING, length : *mut u32) -> windows_sys::core::PCWSTR);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsInspectString(targethstring : usize, machine : u16, callback : PINSPECT_HSTRING_CALLBACK, context : *const core::ffi::c_void, length : *mut u32, targetstringaddress : *mut usize) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-1.dll" "system" fn WindowsInspectString2(targethstring : u64, machine : u16, callback : PINSPECT_HSTRING_CALLBACK2, context : *const core::ffi::c_void, length : *mut u32, targetstringaddress : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsIsStringEmpty(string : windows_sys::core::HSTRING) -> windows_sys::core::BOOL);
#[cfg(feature = "hstring")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPreallocateStringBuffer(length : u32, charbuffer : *mut *mut u16, bufferhandle : *mut super::HSTRING_BUFFER) -> windows_sys::core::HRESULT);
#[cfg(feature = "hstring")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPromoteStringBuffer(bufferhandle : super::HSTRING_BUFFER, string : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsReplaceString(string : windows_sys::core::HSTRING, stringreplaced : windows_sys::core::HSTRING, stringreplacewith : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsStringHasEmbeddedNull(string : windows_sys::core::HSTRING, hasembednull : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstring(string : windows_sys::core::HSTRING, startindex : u32, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstringWithSpecifiedLength(string : windows_sys::core::HSTRING, startindex : u32, length : u32, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringEnd(string : windows_sys::core::HSTRING, trimstring : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringStart(string : windows_sys::core::HSTRING, trimstring : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
pub type PINSPECT_HSTRING_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> windows_sys::core::HRESULT>;
pub type PINSPECT_HSTRING_CALLBACK2 = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> windows_sys::core::HRESULT>;
