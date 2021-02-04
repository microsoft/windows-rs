/// A null-terminated wide-string that has been allocated with `CoTaskMemAlloc` or `CoTaskMemRealloc`
/// and is freed with `CoTaskMemFree`.
#[repr(transparent)]
pub struct CoString(*mut u16);

impl CoString {
    /// Create a new `CoString`
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }

    /// Create a CoString from an iterator of u16s and a length.
    ///
    /// SAFETY: The number of elements in the iterator must be less than or equal to `len`,
    /// `iter` must have at least one element, and `len` must be non-zero.
    unsafe fn from_wide_iter<I>(iter: I, len: usize) -> Self
    where
        I: IntoIterator<Item = u16>,
    {
        debug_assert!(len > 0, "Trying to allocate an CoString with len 0");
        let start = CoTaskMemAlloc(len * 2 + 2) as *mut u16;
        assert!(!start.is_null(), "Could not allocate memory for CoString");
        let mut cursor = start;
        for (index, c) in iter.into_iter().enumerate() {
            debug_assert!(index < len);
            cursor.write(c);
            cursor = cursor.add(1);
        }
        debug_assert!(
            cursor != start,
            "Trying to allocate a CoString with zero elements"
        );
        cursor.write(0);
        Self(start)
    }

    /// Get the string as 16-bit wide characters (wchars).
    ///
    /// This returns an iterator instead of slice, because the
    /// string does not keep track of its length.
    pub fn as_wide(&self) -> impl Iterator<Item = u16> {
        unsafe { WideStringIter::new(self.0) }
    }

    /// Returns whether the string is empty or not
    pub fn is_empty(&self) -> bool {
        self.0.is_null()
    }
}

impl Drop for CoString {
    fn drop(&mut self) {
        // Pointer is allowed to be null
        unsafe { CoTaskMemFree(self.0 as _) }
    }
}

impl From<&CoString> for String {
    fn from(cs: &CoString) -> Self {
        std::char::decode_utf16(cs.as_wide())
            .map(|r| r.unwrap())
            .collect::<String>()
    }
}

impl From<CoString> for String {
    fn from(cs: CoString) -> Self {
        (&cs).into()
    }
}

impl From<&str> for CoString {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            return Self::new();
        }

        // `len` is guaranteed to be == to the number of elements
        // and we've already checked for empty strings
        unsafe { Self::from_wide_iter(s.encode_utf16(), s.len()) }
    }
}

impl From<String> for CoString {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl std::fmt::Display for CoString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for c in std::char::decode_utf16(self.as_wide()) {
            f.write_char(c.unwrap())?
        }
        Ok(())
    }
}

impl std::fmt::Debug for CoString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[link(name = "OLE32")]
extern "system" {
    pub fn CoTaskMemAlloc(size: usize) -> *mut std::ffi::c_void;
    pub fn CoTaskMemFree(pv: *mut std::ffi::c_void);
}

struct WideStringIter {
    ptr: *mut u16,
}

impl WideStringIter {
    /// Create a new WideStringIterator
    /// The pointer passed to this type must be a valid pointer to a wide string OR null
    unsafe fn new(ptr: *mut u16) -> Self {
        Self { ptr }
    }
}

impl Iterator for WideStringIter {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            return None;
        }
        // `ptr` must be a valid wide string and we've already checked for null
        let val = unsafe { self.ptr.read() };
        if val == 0 {
            self.ptr = std::ptr::null_mut();
            return None;
        }
        // We've checked if we're at the end of the string, and we're not
        self.ptr = unsafe { self.ptr.add(1) };
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversions() {
        let original_string = "Hello, world!";
        let co_string: CoString = original_string.into();
        assert!(!co_string.is_empty());
        let string: String = co_string.into();
        assert_eq!(string, original_string);

        let empty: CoString = "".into();
        assert!(empty.is_empty());
        assert_eq!(&String::from(empty), "");
    }
}
