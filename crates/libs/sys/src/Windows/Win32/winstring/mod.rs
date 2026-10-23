#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCompareStringOrdinal(string1 : windows_sys::core::HSTRING, string2 : windows_sys::core::HSTRING, result : *mut super::INT32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsConcatString(string1 : windows_sys::core::HSTRING, string2 : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateString(sourcestring : super::PCNZWCH, length : super::UINT32, string : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "hstring", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateStringReference(sourcestring : windows_sys::core::PCWSTR, length : super::UINT32, hstringheader : *mut super::HSTRING_HEADER, string : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteString(string : windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
#[cfg(feature = "hstring")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteStringBuffer(bufferhandle : super::HSTRING_BUFFER) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDuplicateString(string : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringLen(string : windows_sys::core::HSTRING) -> super::UINT32);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringRawBuffer(string : windows_sys::core::HSTRING, length : *mut super::UINT32) -> windows_sys::core::PCWSTR);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsInspectString(targethstring : super::UINT_PTR, machine : super::USHORT, callback : PINSPECT_HSTRING_CALLBACK, context : *const core::ffi::c_void, length : *mut super::UINT32, targetstringaddress : *mut super::UINT_PTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-1.dll" "system" fn WindowsInspectString2(targethstring : super::UINT64, machine : super::USHORT, callback : PINSPECT_HSTRING_CALLBACK2, context : *const core::ffi::c_void, length : *mut super::UINT32, targetstringaddress : *mut super::UINT64) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsIsStringEmpty(string : windows_sys::core::HSTRING) -> windows_sys::core::BOOL);
#[cfg(all(feature = "hstring", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPreallocateStringBuffer(length : super::UINT32, charbuffer : *mut *mut super::WCHAR, bufferhandle : *mut super::HSTRING_BUFFER) -> windows_sys::core::HRESULT);
#[cfg(feature = "hstring")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPromoteStringBuffer(bufferhandle : super::HSTRING_BUFFER, string : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsReplaceString(string : windows_sys::core::HSTRING, stringreplaced : windows_sys::core::HSTRING, stringreplacewith : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsStringHasEmbeddedNull(string : windows_sys::core::HSTRING, hasembednull : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstring(string : windows_sys::core::HSTRING, startindex : super::UINT32, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstringWithSpecifiedLength(string : windows_sys::core::HSTRING, startindex : super::UINT32, length : super::UINT32, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringEnd(string : windows_sys::core::HSTRING, trimstring : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringStart(string : windows_sys::core::HSTRING, trimstring : windows_sys::core::HSTRING, newstring : *mut windows_sys::core::HSTRING) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
pub type PINSPECT_HSTRING_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: super::UINT_PTR, length: super::UINT32, buffer: *mut super::BYTE) -> windows_sys::core::HRESULT>;
#[cfg(feature = "shared")]
pub type PINSPECT_HSTRING_CALLBACK2 = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: super::UINT64, length: super::UINT32, buffer: *mut super::BYTE) -> windows_sys::core::HRESULT>;
