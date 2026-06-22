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
