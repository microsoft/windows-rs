use crate::*;
use std::convert::TryFrom;
use std::result::Result as StdResult;

/// A Basic string, sometimes called a [BSTR](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr),
/// is a string data type used by certain COM interfaces and interop functions.
#[repr(transparent)]
pub struct BString(RawPtr);

impl BString {
    /// Create an empty `BString`.
    ///
    /// This function does no allocation.
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }

    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of `self`.
    pub fn len(&self) -> usize {
        unsafe { SysStringLen(self.0) as usize }
    }

    /// Get the string as 16-bit wide characters (wchars).
    pub fn as_wide(&self) -> &[u16] {
        if self.is_empty() {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(self.0 as *const u16, self.len()) }
    }

    /// Create a `BString` from a slice of 16 bit characters (wchars).
    pub fn from_wide(value: &[u16]) -> Self {
        if value.len() == 0 {
            return Self::new();
        }

        unsafe { Self(SysAllocStringLen(value.as_ptr(), value.len() as u32)) }
    }

    /// Get the contents of this `BString` as a `String` lossily.
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_wide())
    }

    /// Frees the memory occupied by the string.
    pub fn clear(&mut self) {
        unsafe {
            SysFreeString(self.0);
        }

        self.0 = std::ptr::null_mut();
    }
}

unsafe impl Abi for BString {
    type Abi = RawPtr;

    fn set_abi(&mut self) -> *mut RawPtr {
        debug_assert!(self.is_empty());
        &mut self.0 as *mut _ as _
    }
}

impl Default for BString {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for BString {
    fn clone(&self) -> Self {
        Self::from_wide(self.as_wide())
    }
}

impl Drop for BString {
    fn drop(&mut self) {
        self.clear();
    }
}

impl std::fmt::Display for BString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for c in std::char::decode_utf16(self.as_wide().iter().cloned()) {
            f.write_char(c.map_err(|_| std::fmt::Error)?)?
        }
        Ok(())
    }
}

impl std::fmt::Debug for BString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<&str> for BString {
    fn from(value: &str) -> Self {
        let value: Vec<u16> = value.encode_utf16().collect();
        Self::from_wide(&value)
    }
}

impl From<String> for BString {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&String> for BString {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl PartialEq for BString {
    fn eq(&self, other: &Self) -> bool {
        self.as_wide() == other.as_wide()
    }
}

impl PartialEq<String> for BString {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for BString {
    fn eq(&self, other: &str) -> bool {
        self == other
    }
}

impl PartialEq<&str> for BString {
    fn eq(&self, other: &&str) -> bool {
        self.as_wide().iter().copied().eq(other.encode_utf16())
    }
}

impl PartialEq<BString> for &str {
    fn eq(&self, other: &BString) -> bool {
        other == self
    }
}

impl<'a> TryFrom<&'a BString> for String {
    type Error = std::string::FromUtf16Error;

    fn try_from(value: &BString) -> StdResult<Self, Self::Error> {
        String::from_utf16(value.as_wide())
    }
}

impl TryFrom<BString> for String {
    type Error = std::string::FromUtf16Error;

    fn try_from(value: BString) -> StdResult<Self, Self::Error> {
        String::try_from(&value)
    }
}

#[link(name = "oleaut32")]
extern "system" {
    fn SysStringLen(bstr: RawPtr) -> u32;
    fn SysFreeString(bstr: RawPtr);
    fn SysAllocStringLen(value: *const u16, len: u32) -> RawPtr;
}

#[cfg(test)]
mod tests {
    use super::*;
    type StringType = BString;

    #[test]
    fn bstring_works() {
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
    fn bstring_to_string() {
        let h = StringType::from("test");
        let s = String::try_from(h).unwrap();
        assert!(s == "test");
    }

    #[test]
    fn bstring_to_string_err() {
        // 𝄞mu<invalid>ic
        let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
        let h = StringType::from_wide(wide_data);
        let err = String::try_from(h);
        assert!(err.is_err());
    }

    #[test]
    fn bstring_to_string_lossy() {
        // 𝄞mu<invalid>ic
        let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
        let h = StringType::from_wide(wide_data);
        let s = h.to_string_lossy();
        assert_eq!(s, "𝄞mu�ic");
    }
}
