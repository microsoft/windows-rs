use crate::*;
use std::convert::TryFrom;
use std::result::Result as StdResult;

/// A WinRT string, sometimes called an [HSTRING](https://docs.microsoft.com/en-us/windows/win32/winrt/hstring),
/// is reference-counted and logically immutable. It should only be used for communicating with WinRT APIs.
#[repr(transparent)]
pub struct HString(*mut Header);

impl HString {
    /// Create an empty `HString`.
    ///
    /// This function does no allocation.
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }

    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        // An empty HSTRING is represented by a null pointer.
        self.0.is_null()
    }

    /// Returns the length of `self`.
    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        unsafe { (*self.0).len as usize }
    }

    /// Get the string as 16-bit wide characters (wchars).
    pub fn as_wide(&self) -> &[u16] {
        if self.is_empty() {
            return &[];
        }

        let header = self.0;
        unsafe { std::slice::from_raw_parts((*header).data, (*header).len as usize) }
    }

    /// Create a `HString` from a slice of 16 bit characters (wchars).
    pub fn from_wide(value: &[u16]) -> Self {
        unsafe { Self::from_wide_iter(value.iter().copied(), value.len() as u32) }
    }

    /// Get the contents of this `HString` as a String lossily.
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_wide())
    }

    /// Clear the contents of the string and free the memory if `self` holds the
    /// last reference to the string data.
    pub fn clear(&mut self) {
        if self.is_empty() {
            return;
        }

        unsafe {
            // This flag indicates a "fast pass" string created by some languages where the
            // header is allocated on the stack. Such strings must never be freed.
            let header = self.0;
            debug_assert!((*header).flags & REFERENCE_FLAG == 0);

            if (*((*header).shared.as_mut_ptr())).count.release() == 0 {
                heap_free(self.0 as RawPtr);
            }
        }

        self.0 = std::ptr::null_mut();
    }

    /// # Safety
    /// len must not be less than the number of items in the iterator.
    unsafe fn from_wide_iter<I: Iterator<Item = u16>>(iter: I, len: u32) -> Self {
        if len == 0 {
            return Self::new();
        }

        let mut ptr = Header::alloc(len);

        // Place each utf-16 character into the buffer and
        // increase len as we go along.
        for (index, wide) in iter.enumerate() {
            debug_assert!((index as u32) < len);

            std::ptr::write((*ptr).data.add(index), wide);
            (*ptr).len = index as u32 + 1;
        }

        // Write a 0 byte to the end of the buffer.
        std::ptr::write((*ptr).data.offset((*ptr).len as isize), 0);
        Self(ptr)
    }
}

unsafe impl Abi for HString {
    type Abi = RawPtr;

    fn set_abi(&mut self) -> *mut RawPtr {
        debug_assert!(self.is_empty());
        &mut self.0 as *mut _ as _
    }
}

unsafe impl RuntimeType for HString {
    type DefaultType = Self;
    const SIGNATURE: crate::ConstBuffer = crate::ConstBuffer::from_slice(b"string");
}

impl Default for HString {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for HString {
    fn clone(&self) -> Self {
        if self.is_empty() {
            return Self::new();
        }

        unsafe { Self((*self.0).duplicate()) }
    }
}

impl Drop for HString {
    fn drop(&mut self) {
        self.clear();
    }
}

impl std::fmt::Display for HString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for c in std::char::decode_utf16(self.as_wide().iter().cloned()) {
            f.write_char(c.map_err(|_| std::fmt::Error)?)?
        }
        Ok(())
    }
}

impl std::fmt::Debug for HString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<&str> for HString {
    fn from(value: &str) -> Self {
        unsafe { Self::from_wide_iter(value.encode_utf16(), value.len() as u32) }
    }
}

impl From<String> for HString {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&String> for HString {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl PartialEq for HString {
    fn eq(&self, other: &Self) -> bool {
        self.as_wide() == other.as_wide()
    }
}

impl PartialEq<String> for HString {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for HString {
    fn eq(&self, other: &str) -> bool {
        self == other
    }
}

impl PartialEq<&str> for HString {
    fn eq(&self, other: &&str) -> bool {
        self.as_wide().iter().copied().eq(other.encode_utf16())
    }
}

impl PartialEq<HString> for &str {
    fn eq(&self, other: &HString) -> bool {
        other == self
    }
}

impl<'a> TryFrom<&'a HString> for String {
    type Error = std::string::FromUtf16Error;

    fn try_from(hstring: &HString) -> StdResult<Self, Self::Error> {
        String::from_utf16(hstring.as_wide())
    }
}

impl TryFrom<HString> for String {
    type Error = std::string::FromUtf16Error;

    fn try_from(hstring: HString) -> StdResult<Self, Self::Error> {
        String::try_from(&hstring)
    }
}

const REFERENCE_FLAG: u32 = 1;

#[repr(C)]
pub struct Header {
    flags: u32,
    len: u32,
    _0: u32,
    _1: u32,
    data: *mut u16,
    shared: std::mem::MaybeUninit<Shared>,
}

#[repr(C)]
struct Shared {
    count: RefCount,
    buffer_start: u16,
}

impl Header {
    fn alloc(len: u32) -> *mut Header {
        debug_assert!(len != 0);
        // Allocate enough space for header and two bytes per character.
        let alloc_size = std::mem::size_of::<Header>() + 2 * len as usize;

        let header = heap_alloc(alloc_size) as *mut Header;

        if header.is_null() {
            panic!("Could not successfully allocate for HString");
        }

        unsafe {
            (*header).flags = 0;
            (*header).len = len;
            (*header).data = &mut (*(*header).shared.as_mut_ptr()).buffer_start;
            (*(*header).shared.as_mut_ptr()).count = RefCount::new();
        }
        header
    }

    fn duplicate(&mut self) -> *mut Header {
        if self.flags & REFERENCE_FLAG == 0 {
            // If this is not a "fast pass" string then simply increment the reference count.
            unsafe {
                (*self.shared.as_ptr()).count.add_ref();
                self
            }
        } else {
            // Otherwise, allocate a new string and copy the value into the new string.
            let copy = Header::alloc(self.len);
            unsafe {
                std::ptr::copy_nonoverlapping(self.data, (*copy).data, self.len as usize + 1);
            }
            copy
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type StringType = HString;

    #[test]
    fn hstring_works() {
        let empty = StringType::new();
        assert!(empty.is_empty());
        assert!(empty.len() == 0);

        let mut hello = StringType::from("Hello");
        assert!(!hello.is_empty());
        assert!(hello.len() == 5);

        let rust = hello.to_string();
        assert!(rust == "Hello");
        assert!(rust.len() == 5);

        let hello2 = hello.clone();
        hello.clear();
        assert!(hello.len() == 0);
        hello.clear();
        assert!(hello.len() == 0);
        assert!(!hello2.is_empty());
        assert!(hello2.len() == 5);

        assert!(StringType::from("Hello") == StringType::from("Hello"));
        assert!(StringType::from("Hello") != StringType::from("World"));

        assert!(StringType::from("Hello") == "Hello");
        assert!(StringType::from("Hello") != "Hello ");
        assert!(StringType::from("Hello") != "Hell");
        assert!(StringType::from("Hello") != "World");

        assert!(StringType::from("Hello").to_string() == String::from("Hello"));
    }

    #[test]
    fn display_format() {
        let value = StringType::from("Hello world");
        assert!(format!("{}", value) == "Hello world");
    }

    #[test]
    fn debug_format() {
        let value = StringType::from("Hello world");
        assert!(format!("{:?}", value) == "Hello world");
    }

    #[test]
    fn abi_transfer() {
        fn perform_transfer(from: StringType, to: &mut StringType) {
            let from = std::mem::ManuallyDrop::new(from);
            unsafe {
                let to = to.set_abi();
                let from = from.abi();
                *to = from
            };
        }

        let from = StringType::from("Hello");
        let mut to = StringType::new();
        perform_transfer(from, &mut to);

        assert!(format!("{}", to) == "Hello");
    }

    #[test]
    fn from_empty_string() {
        let h = StringType::from("");
        assert!(format!("{}", h) == "");
    }

    #[test]
    fn hstring_to_string() {
        let h = StringType::from("test");
        let s = String::try_from(h).unwrap();
        assert!(s == "test");
    }

    #[test]
    fn hstring_to_string_err() {
        // 𝄞mu<invalid>ic
        let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
        let h = StringType::from_wide(wide_data);
        let err = String::try_from(h);
        assert!(err.is_err());
    }

    #[test]
    fn hstring_to_string_lossy() {
        // 𝄞mu<invalid>ic
        let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
        let h = StringType::from_wide(wide_data);
        let s = h.to_string_lossy();
        assert_eq!(s, "𝄞mu�ic");
    }
}
