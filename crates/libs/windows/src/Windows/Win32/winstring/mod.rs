#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WindowsCompareStringOrdinal(string1: &windows_core::HSTRING, string2: &windows_core::HSTRING) -> windows_core::Result<super::INT32> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCompareStringOrdinal(string1 : *mut core::ffi::c_void, string2 : *mut core::ffi::c_void, result : *mut super::INT32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsCompareStringOrdinal(core::mem::transmute_copy(string1), core::mem::transmute_copy(string2), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WindowsConcatString(string1: &windows_core::HSTRING, string2: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsConcatString(string1 : *mut core::ffi::c_void, string2 : *mut core::ffi::c_void, newstring : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsConcatString(core::mem::transmute_copy(string1), core::mem::transmute_copy(string2), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WindowsCreateString(sourcestring: Option<&[u16]>) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateString(sourcestring : super::PCNZWCH, length : super::UINT32, string : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsCreateString(core::mem::transmute(sourcestring.map_or(core::ptr::null(), |slice| slice.as_ptr())), sourcestring.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "hstring", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WindowsCreateStringReference(sourcestring: Option<windows_core::PCWSTR>, length: super::UINT32, hstringheader: *mut super::HSTRING_HEADER, string: *mut windows_core::HSTRING) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateStringReference(sourcestring : windows_core::PCWSTR, length : super::UINT32, hstringheader : *mut super::HSTRING_HEADER, string : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WindowsCreateStringReference(sourcestring.unwrap_or(core::mem::zeroed()) as _, length, hstringheader as _, core::mem::transmute(string)) }
}
#[inline]
pub unsafe fn WindowsDeleteString(string: &windows_core::HSTRING) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteString(string : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WindowsDeleteString(core::mem::transmute_copy(string)) }
}
#[cfg(feature = "hstring")]
#[inline]
pub unsafe fn WindowsDeleteStringBuffer(bufferhandle: Option<super::HSTRING_BUFFER>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteStringBuffer(bufferhandle : super::HSTRING_BUFFER) -> windows_core::HRESULT);
    unsafe { WindowsDeleteStringBuffer(bufferhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WindowsDuplicateString(string: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDuplicateString(string : *mut core::ffi::c_void, newstring : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsDuplicateString(core::mem::transmute_copy(string), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WindowsGetStringLen(string: &windows_core::HSTRING) -> super::UINT32 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringLen(string : *mut core::ffi::c_void) -> super::UINT32);
    unsafe { WindowsGetStringLen(core::mem::transmute_copy(string)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WindowsGetStringRawBuffer(string: &windows_core::HSTRING, length: Option<*mut super::UINT32>) -> windows_core::PCWSTR {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringRawBuffer(string : *mut core::ffi::c_void, length : *mut super::UINT32) -> windows_core::PCWSTR);
    unsafe { WindowsGetStringRawBuffer(core::mem::transmute_copy(string), length.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WindowsInspectString(targethstring: super::UINT_PTR, machine: super::USHORT, callback: PINSPECT_HSTRING_CALLBACK, context: Option<*const core::ffi::c_void>, length: *mut super::UINT32, targetstringaddress: *mut super::UINT_PTR) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsInspectString(targethstring : super::UINT_PTR, machine : super::USHORT, callback : PINSPECT_HSTRING_CALLBACK, context : *const core::ffi::c_void, length : *mut super::UINT32, targetstringaddress : *mut super::UINT_PTR) -> windows_core::HRESULT);
    unsafe { WindowsInspectString(targethstring, machine, callback, context.unwrap_or(core::mem::zeroed()) as _, length as _, targetstringaddress as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WindowsInspectString2(targethstring: super::UINT64, machine: super::USHORT, callback: PINSPECT_HSTRING_CALLBACK2, context: Option<*const core::ffi::c_void>, length: *mut super::UINT32, targetstringaddress: *mut super::UINT64) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-1.dll" "system" fn WindowsInspectString2(targethstring : super::UINT64, machine : super::USHORT, callback : PINSPECT_HSTRING_CALLBACK2, context : *const core::ffi::c_void, length : *mut super::UINT32, targetstringaddress : *mut super::UINT64) -> windows_core::HRESULT);
    unsafe { WindowsInspectString2(targethstring, machine, callback, context.unwrap_or(core::mem::zeroed()) as _, length as _, targetstringaddress as _) }
}
#[inline]
pub unsafe fn WindowsIsStringEmpty(string: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsIsStringEmpty(string : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WindowsIsStringEmpty(core::mem::transmute_copy(string)) }
}
#[cfg(all(feature = "hstring", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WindowsPreallocateStringBuffer(length: super::UINT32, charbuffer: *mut *mut super::WCHAR, bufferhandle: *mut super::HSTRING_BUFFER) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPreallocateStringBuffer(length : super::UINT32, charbuffer : *mut *mut super::WCHAR, bufferhandle : *mut super::HSTRING_BUFFER) -> windows_core::HRESULT);
    unsafe { WindowsPreallocateStringBuffer(length, charbuffer as _, bufferhandle as _) }
}
#[cfg(feature = "hstring")]
#[inline]
pub unsafe fn WindowsPromoteStringBuffer(bufferhandle: super::HSTRING_BUFFER) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPromoteStringBuffer(bufferhandle : super::HSTRING_BUFFER, string : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsPromoteStringBuffer(bufferhandle, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsReplaceString(string: &windows_core::HSTRING, stringreplaced: &windows_core::HSTRING, stringreplacewith: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsReplaceString(string : *mut core::ffi::c_void, stringreplaced : *mut core::ffi::c_void, stringreplacewith : *mut core::ffi::c_void, newstring : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsReplaceString(core::mem::transmute_copy(string), core::mem::transmute_copy(stringreplaced), core::mem::transmute_copy(stringreplacewith), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsStringHasEmbeddedNull(string: &windows_core::HSTRING) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsStringHasEmbeddedNull(string : *mut core::ffi::c_void, hasembednull : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsStringHasEmbeddedNull(core::mem::transmute_copy(string), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WindowsSubstring(string: &windows_core::HSTRING, startindex: super::UINT32) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstring(string : *mut core::ffi::c_void, startindex : super::UINT32, newstring : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsSubstring(core::mem::transmute_copy(string), startindex, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WindowsSubstringWithSpecifiedLength(string: &windows_core::HSTRING, startindex: super::UINT32, length: super::UINT32) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstringWithSpecifiedLength(string : *mut core::ffi::c_void, startindex : super::UINT32, length : super::UINT32, newstring : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsSubstringWithSpecifiedLength(core::mem::transmute_copy(string), startindex, length, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsTrimStringEnd(string: &windows_core::HSTRING, trimstring: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringEnd(string : *mut core::ffi::c_void, trimstring : *mut core::ffi::c_void, newstring : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsTrimStringEnd(core::mem::transmute_copy(string), core::mem::transmute_copy(trimstring), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsTrimStringStart(string: &windows_core::HSTRING, trimstring: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringStart(string : *mut core::ffi::c_void, trimstring : *mut core::ffi::c_void, newstring : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsTrimStringStart(core::mem::transmute_copy(string), core::mem::transmute_copy(trimstring), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "shared")]
pub type PINSPECT_HSTRING_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: super::UINT_PTR, length: super::UINT32, buffer: *mut super::BYTE) -> windows_core::HRESULT>;
#[cfg(feature = "shared")]
pub type PINSPECT_HSTRING_CALLBACK2 = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: super::UINT64, length: super::UINT32, buffer: *mut super::BYTE) -> windows_core::HRESULT>;
