use super::*;

/// Decodes a caller-owned string returned by a fallible WebView2 getter into
/// an owned `String`, freeing the buffer and treating a failed call as empty.
/// See [`take`] for the ownership and freeing contract.
pub(crate) unsafe fn take_result(value: Result<*mut u16>) -> String {
    value
        .map(|value| unsafe { take(value) })
        .unwrap_or_default()
}

/// Decodes a caller-owned string returned by WebView2 into an owned `String`
/// and frees the original buffer with `CoTaskMemFree`, as required for WebView2
/// `[out]` string parameters.
pub(crate) unsafe fn take(value: *mut u16) -> String {
    if value.is_null() {
        return String::new();
    }

    let result = unsafe { PCWSTR::from_raw(value).to_string().unwrap_or_default() };
    unsafe { CoTaskMemFree(value as *mut _) };
    result
}

/// Allocates a null-terminated UTF-16 copy of `value` with `CoTaskMemAlloc`,
/// transferring ownership to the caller (WebView2). Used to return strings from
/// `[out]` getters of COM interfaces the crate implements, where WebView2 frees
/// the buffer with `CoTaskMemFree`.
pub(crate) unsafe fn allocate(value: &str) -> Result<*mut u16> {
    let encoded: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
    let bytes = encoded.len() * size_of::<u16>();

    let buffer = unsafe { CoTaskMemAlloc(bytes) } as *mut u16;

    if buffer.is_null() {
        return Err(Error::from_hresult(E_OUTOFMEMORY));
    }

    unsafe { std::ptr::copy_nonoverlapping(encoded.as_ptr(), buffer, encoded.len()) };
    Ok(buffer)
}
