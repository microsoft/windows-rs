use crate::runtime::*;

#[repr(C)]
pub struct String {
    hstring: HSTRING,
}

// TODO: if a Rust String/str is provided (rather than a WinRT string),
// store the HSTRING_HEADER and the Vec<16> buffer in a single heap allocation.
// Alternatively, if the WinRT string construction uses WindowsPreallocateStringBuffer,
// just use that and forget about hstring references as it's just one allocation and will
// avoid any subsequent allocation if hte HSTRING is promoted.

impl String {
    fn new() -> String {
        String {
            hstring: std::ptr::null_mut(),
        }
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: format the wchar buffer directly to avoid an allocation?
        write!(f, "{}", std::string::String::from(self))
    }
}

impl From<&str> for String {
    fn from(value: &str) -> String {
        // TODO: could avoid this temporary allocation by using an HSTRING buffer, but only if we
        // can definitively calculate the resulting length exactly (and efficienty). Profile whether
        // it is more efficient to call value.chars().count() and then using WindowsPreallocateStringBuffer
        // and filling the buffer with value.encode_utf16()...
        let wide: Vec<u16> = value.encode_utf16().collect();
        let mut hstring: HSTRING = std::ptr::null_mut();
        unsafe { WindowsCreateString(wide.as_ptr(), wide.len() as u32, &mut hstring).unwrap() };
        String { hstring }
    }
}

impl From<&String> for std::string::String {
    fn from(value: &String) -> std::string::String {
        let mut len = 0;
        let wide = unsafe { WindowsGetStringRawBuffer(value.hstring, &mut len) };
        if len == 0 {
            std::string::String::new()
        } else {
            std::string::String::from_utf16(unsafe {
                std::slice::from_raw_parts(wide, len as usize)
            })
            .unwrap()
        }
    }
}
