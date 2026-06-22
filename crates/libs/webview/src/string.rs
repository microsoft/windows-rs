use super::*;

/// Encodes a string slice as a null-terminated UTF-16 buffer suitable for
/// passing as an `LPCWSTR` parameter. The returned buffer must be kept alive
/// for the duration of the call.
pub fn encode(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Decodes a null-terminated UTF-16 string returned by WebView2 into an owned
/// `String`. Returns an empty string for a null pointer.
pub unsafe fn decode(value: LPCWSTR) -> String {
    if value.is_null() {
        return String::new();
    }

    unsafe {
        let mut len = 0;
        while *value.add(len) != 0 {
            len += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(value, len))
    }
}

/// Decodes a caller-owned `LPWSTR` returned by WebView2 into an owned `String`
/// and frees the original buffer with `CoTaskMemFree`, as required for WebView2
/// `[out]` string parameters.
pub unsafe fn take(value: LPWSTR) -> String {
    let result = unsafe { decode(value) };

    if !value.is_null() {
        unsafe { CoTaskMemFree(value as *const _) };
    }

    result
}

/// Allocates a null-terminated UTF-16 copy of `value` with `CoTaskMemAlloc`,
/// transferring ownership to the caller (WebView2). Used to return strings from
/// `[out]` getters of COM interfaces the crate implements, where WebView2 frees
/// the buffer with `CoTaskMemFree`.
pub unsafe fn allocate(value: &str) -> Result<LPWSTR> {
    let encoded: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
    let bytes = encoded.len() * size_of::<u16>();

    let buffer = unsafe { CoTaskMemAlloc(bytes) } as LPWSTR;

    if buffer.is_null() {
        return Err(Error::from_hresult(E_OUTOFMEMORY));
    }

    unsafe { std::ptr::copy_nonoverlapping(encoded.as_ptr(), buffer, encoded.len()) };
    Ok(buffer)
}
