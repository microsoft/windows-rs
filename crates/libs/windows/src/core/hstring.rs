use super::*;

/// A WinRT string ([HSTRING](https://docs.microsoft.com/en-us/windows/win32/winrt/hstring)),
/// is reference-counted and immutable.
#[repr(transparent)]
pub struct HSTRING(*mut Header);

impl HSTRING {
    /// Create an empty `HSTRING`.
    ///
    /// This function does not allocate memory.
    pub fn new() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        // An empty HSTRING is represented by a null pointer.
        self.0.is_null()
    }

    /// Returns the length of the string.
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
        unsafe { core::slice::from_raw_parts((*header).data, (*header).len as usize) }
    }

    /// Create a `HSTRING` from a slice of 16 bit characters (wchars).
    pub fn from_wide(value: &[u16]) -> Self {
        unsafe { Self::from_wide_iter(value.iter().copied(), value.len() as u32) }
    }

    /// Get the contents of this `HSTRING` as a String lossily.
    pub fn to_string_lossy(&self) -> alloc::string::String {
        alloc::string::String::from_utf16_lossy(self.as_wide())
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

        self.0 = core::ptr::null_mut();
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

            core::ptr::write((*ptr).data.add(index), wide);
            (*ptr).len = index as u32 + 1;
        }

        // Write a 0 byte to the end of the buffer.
        core::ptr::write((*ptr).data.offset((*ptr).len as isize), 0);
        Self(ptr)
    }
}

unsafe impl Abi for HSTRING {
    type Abi = core::mem::ManuallyDrop<Self>;
}

unsafe impl RuntimeType for HSTRING {
    const SIGNATURE: ConstBuffer = ConstBuffer::from_slice(b"string");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> Result<Self> {
        Ok(from.clone())
    }
}

impl Default for HSTRING {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for HSTRING {
    fn clone(&self) -> Self {
        if self.is_empty() {
            return Self::new();
        }

        unsafe { Self((*self.0).duplicate()) }
    }
}

impl Drop for HSTRING {
    fn drop(&mut self) {
        self.clear();
    }
}

unsafe impl Send for HSTRING {}
unsafe impl Sync for HSTRING {}

impl core::fmt::Display for HSTRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use ::core::fmt::Write;
        for c in core::char::decode_utf16(self.as_wide().iter().cloned()) {
            f.write_char(c.map_err(|_| core::fmt::Error)?)?
        }
        Ok(())
    }
}

impl core::fmt::Debug for HSTRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl core::convert::From<&str> for HSTRING {
    fn from(value: &str) -> Self {
        unsafe { Self::from_wide_iter(value.encode_utf16(), value.len() as u32) }
    }
}

impl core::convert::From<alloc::string::String> for HSTRING {
    fn from(value: alloc::string::String) -> Self {
        value.as_str().into()
    }
}

impl core::convert::From<&alloc::string::String> for HSTRING {
    fn from(value: &alloc::string::String) -> Self {
        value.as_str().into()
    }
}

impl PartialEq for HSTRING {
    fn eq(&self, other: &Self) -> bool {
        *self.as_wide() == *other.as_wide()
    }
}

impl PartialEq<alloc::string::String> for HSTRING {
    fn eq(&self, other: &alloc::string::String) -> bool {
        *self == **other
    }
}

impl PartialEq<alloc::string::String> for &HSTRING {
    fn eq(&self, other: &alloc::string::String) -> bool {
        **self == **other
    }
}

impl PartialEq<&alloc::string::String> for HSTRING {
    fn eq(&self, other: &&alloc::string::String) -> bool {
        *self == ***other
    }
}

impl PartialEq<str> for HSTRING {
    fn eq(&self, other: &str) -> bool {
        self.as_wide().iter().copied().eq(other.encode_utf16())
    }
}

impl PartialEq<str> for &HSTRING {
    fn eq(&self, other: &str) -> bool {
        **self == *other
    }
}

impl PartialEq<&str> for HSTRING {
    fn eq(&self, other: &&str) -> bool {
        *self == **other
    }
}

impl PartialEq<HSTRING> for str {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == *self
    }
}

impl PartialEq<HSTRING> for &str {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == **self
    }
}

impl PartialEq<&HSTRING> for str {
    fn eq(&self, other: &&HSTRING) -> bool {
        **other == *self
    }
}

impl PartialEq<HSTRING> for alloc::string::String {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == **self
    }
}

impl PartialEq<HSTRING> for &alloc::string::String {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == ***self
    }
}

impl PartialEq<&HSTRING> for alloc::string::String {
    fn eq(&self, other: &&HSTRING) -> bool {
        **other == **self
    }
}

impl<'a> core::convert::TryFrom<&'a HSTRING> for alloc::string::String {
    type Error = alloc::string::FromUtf16Error;

    fn try_from(hstring: &HSTRING) -> core::result::Result<Self, Self::Error> {
        alloc::string::String::from_utf16(hstring.as_wide())
    }
}

impl core::convert::TryFrom<HSTRING> for alloc::string::String {
    type Error = alloc::string::FromUtf16Error;

    fn try_from(hstring: HSTRING) -> core::result::Result<Self, Self::Error> {
        alloc::string::String::try_from(&hstring)
    }
}

#[cfg(feature = "alloc")]
impl<'a> IntoParam<'a, HSTRING> for &str {
    fn into_param(self) -> Param<'a, HSTRING> {
        Param::Owned(self.into())
    }
}

#[cfg(feature = "alloc")]
impl<'a> IntoParam<'a, HSTRING> for alloc::string::String {
    fn into_param(self) -> Param<'a, HSTRING> {
        Param::Owned(self.into())
    }
}

const REFERENCE_FLAG: u32 = 1;

#[repr(C)]
struct Header {
    flags: u32,
    len: u32,
    _0: u32,
    _1: u32,
    data: *mut u16,
    shared: core::mem::MaybeUninit<Shared>,
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
        let alloc_size = core::mem::size_of::<Header>() + 2 * len as usize;

        // TODO: allow this failure to propagate
        let header = heap_alloc(alloc_size).expect("Could not successfully allocate for HSTRING") as *mut Header;

        unsafe {
            (*header).flags = 0;
            (*header).len = len;
            (*header).data = &mut (*(*header).shared.as_mut_ptr()).buffer_start;
            (*(*header).shared.as_mut_ptr()).count = RefCount::new(1);
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
                core::ptr::copy_nonoverlapping(self.data, (*copy).data, self.len as usize + 1);
            }
            copy
        }
    }
}
