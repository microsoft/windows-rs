use crate::*;

/// A WinRT string, sometimes called an [HSTRING](https://docs.microsoft.com/en-us/windows/win32/winrt/hstring).
///
/// A WinRT string is reference-counted and logically immutable. It can only be
/// changed by clearing its value and replacing with a new value. It should only
/// be used for FFI purposes with WinRT APIs.
#[repr(transparent)]
pub struct HString {
    ptr: *mut Header,
}

impl HString {
    /// Create an empty HString.
    ///
    /// This function does no allocation.
    pub fn new() -> HString {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        // An empty HSTRING is represented by a null pointer.
        self.ptr.is_null()
    }

    /// Returns the length of `self`.
    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        unsafe { (*self.ptr).len as usize }
    }

    /// Get the string as 16-bit wide characters (wchars).
    pub fn as_wide(&self) -> &[u16] {
        if self.is_empty() {
            return &[];
        }

        let header = self.ptr;
        unsafe { std::slice::from_raw_parts((*header).data, (*header).len as usize) }
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
            let header = self.ptr;
            debug_assert!((*header).flags & REFERENCE_FLAG == 0);

            if (*((*header).shared.as_mut_ptr())).count.release() == 0 {
                HeapFree(GetProcessHeap(), 0, self.ptr as RawPtr);
            }
        }

        self.ptr = std::ptr::null_mut();
    }
}

unsafe impl AbiTransferable for HString {
    type Abi = *mut Header;

    fn get_abi(&self) -> Self::Abi {
        self.ptr
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.clear();
        &mut self.ptr
    }
}

unsafe impl RuntimeType for HString {
    fn signature() -> String {
        "string".to_owned()
    }
}

impl Default for HString {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for HString {
    fn clone(&self) -> HString {
        if self.is_empty() {
            return Self::new();
        }

        Self {
            ptr: unsafe { (*self.ptr).duplicate() },
        }
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
    fn from(value: &str) -> HString {
        if value.is_empty() {
            return HString::new();
        }

        let mut ptr = Header::alloc(value.len() as u32);

        // Place each utf-16 character into the buffer and
        // increase len as we go along.
        for (index, wide) in value.encode_utf16().enumerate() {
            unsafe {
                std::ptr::write((*ptr).data.add(index), wide);
                (*ptr).len = index as u32 + 1;
            }
        }

        // Write a 0 byte to the end of the buffer.
        unsafe { std::ptr::write((*ptr).data.offset((*ptr).len as isize), 0) };
        Self { ptr }
    }
}

impl From<String> for HString {
    fn from(value: String) -> HString {
        value.as_str().into()
    }
}

impl From<&String> for HString {
    fn from(value: &String) -> HString {
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

impl<'a> From<&'a HString> for String {
    fn from(hstring: &HString) -> Self {
        String::from_utf16(hstring.as_wide()).unwrap()
    }
}

impl From<HString> for String {
    fn from(hstring: HString) -> Self {
        String::from(&hstring)
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

        let header = unsafe { HeapAlloc(GetProcessHeap(), 0, alloc_size) as *mut Header };

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

    #[test]
    fn hstring_works() {
        let empty = HString::new();
        assert!(empty.is_empty());
        assert!(empty.len() == 0);

        let mut hello = HString::from("Hello");
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

        assert!(HString::from("Hello") == HString::from("Hello"));
        assert!(HString::from("Hello") != HString::from("World"));

        assert!(HString::from("Hello") == "Hello");
        assert!(HString::from("Hello") != "Hello ");
        assert!(HString::from("Hello") != "Hell");
        assert!(HString::from("Hello") != "World");

        assert!(HString::from("Hello").to_string() == String::from("Hello"));
    }

    #[test]
    fn display_format() {
        let value = HString::from("Hello world");
        assert!(format!("{}", value) == "Hello world");
    }

    #[test]
    fn debug_format() {
        let value = HString::from("Hello world");
        assert!(format!("{:?}", value) == "Hello world");
    }

    #[test]
    fn abi_transfer() {
        fn perform_transfer(from: HString, to: &mut HString) {
            let from = std::mem::ManuallyDrop::new(from);
            let to = to.set_abi();
            let from = from.get_abi();
            unsafe { *to = from };
        }

        let from = HString::from("Hello");
        let mut to = HString::new();
        perform_transfer(from, &mut to);

        assert!(format!("{}", to) == "Hello");
    }

    #[test]
    fn from_empty_string() {
        let h = HString::from("");
        assert!(format!("{}", h) == "");
    }

    #[test]
    fn hstring_to_string() {
        let h = HString::from("test");
        let s = String::from(h);
        assert!(s == "test");
    }
}
