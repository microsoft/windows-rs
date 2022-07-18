use super::*;

/// A WinRT string ([HSTRING](https://docs.microsoft.com/en-us/windows/win32/winrt/hstring))
/// is reference-counted and immutable.
#[repr(transparent)]
pub struct HSTRING(*mut Header);

impl HSTRING {
    /// Create an empty `HSTRING`.
    ///
    /// This function does not allocate memory.
    pub const fn new() -> Self {
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
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    /// Returns a raw pointer to the `HSTRING` buffer.
    pub fn as_ptr(&self) -> *const u16 {
        if self.is_empty() {
            const EMPTY: [u16; 1] = [0];
            EMPTY.as_ptr()
        } else {
            let header = self.0;
            unsafe { (*header).data }
        }
    }

    /// Create a `HSTRING` from a slice of 16 bit characters (wchars).
    pub fn from_wide(value: &[u16]) -> Self {
        unsafe { Self::from_wide_iter(value.iter().copied(), value.len() as u32) }
    }

    /// Get the contents of this `HSTRING` as a String lossily.
    pub fn to_string_lossy(&self) -> alloc::string::String {
        alloc::string::String::from_utf16_lossy(self.as_wide())
    }

    /// Get the contents of this `HSTRING` as a OsString.
    #[cfg(windows)]
    pub fn to_os_string(&self) -> std::ffi::OsString {
        std::os::windows::ffi::OsStringExt::from_wide(self.as_wide())
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
        if self.is_empty() {
            return;
        }

        unsafe {
            let header = std::mem::replace(&mut self.0, core::ptr::null_mut());
            // REFERENCE_FLAG indicates a string backed by static or stack memory that is
            // thus not reference-counted and does not need to be freed.
            if (*header).flags & REFERENCE_FLAG == 0 && (*header).count.release() == 0 {
                heap_free(header as *mut core::ffi::c_void);
            }
        }
    }
}

unsafe impl Send for HSTRING {}
unsafe impl Sync for HSTRING {}

impl core::fmt::Display for HSTRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Decode(|| core::char::decode_utf16(self.as_wide().iter().cloned())))
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

#[cfg(windows)]
impl core::convert::From<&std::ffi::OsStr> for HSTRING {
    fn from(value: &std::ffi::OsStr) -> Self {
        unsafe { Self::from_wide_iter(std::os::windows::ffi::OsStrExt::encode_wide(value), value.len() as u32) }
    }
}

#[cfg(windows)]
impl core::convert::From<std::ffi::OsString> for HSTRING {
    fn from(value: std::ffi::OsString) -> Self {
        value.as_os_str().into()
    }
}

#[cfg(windows)]
impl core::convert::From<&std::ffi::OsString> for HSTRING {
    fn from(value: &std::ffi::OsString) -> Self {
        value.as_os_str().into()
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

#[cfg(windows)]
impl PartialEq<std::ffi::OsString> for HSTRING {
    fn eq(&self, other: &std::ffi::OsString) -> bool {
        *self == **other
    }
}

#[cfg(windows)]
impl PartialEq<std::ffi::OsString> for &HSTRING {
    fn eq(&self, other: &std::ffi::OsString) -> bool {
        **self == **other
    }
}

#[cfg(windows)]
impl PartialEq<&std::ffi::OsString> for HSTRING {
    fn eq(&self, other: &&std::ffi::OsString) -> bool {
        *self == ***other
    }
}

#[cfg(windows)]
impl PartialEq<std::ffi::OsStr> for HSTRING {
    fn eq(&self, other: &std::ffi::OsStr) -> bool {
        self.as_wide().iter().copied().eq(std::os::windows::ffi::OsStrExt::encode_wide(other))
    }
}

#[cfg(windows)]
impl PartialEq<std::ffi::OsStr> for &HSTRING {
    fn eq(&self, other: &std::ffi::OsStr) -> bool {
        **self == *other
    }
}

#[cfg(windows)]
impl PartialEq<&std::ffi::OsStr> for HSTRING {
    fn eq(&self, other: &&std::ffi::OsStr) -> bool {
        *self == **other
    }
}

#[cfg(windows)]
impl PartialEq<HSTRING> for std::ffi::OsStr {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == *self
    }
}

#[cfg(windows)]
impl PartialEq<HSTRING> for &std::ffi::OsStr {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == **self
    }
}

#[cfg(windows)]
impl PartialEq<&HSTRING> for std::ffi::OsStr {
    fn eq(&self, other: &&HSTRING) -> bool {
        **other == *self
    }
}

#[cfg(windows)]
impl PartialEq<HSTRING> for std::ffi::OsString {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == **self
    }
}

#[cfg(windows)]
impl PartialEq<HSTRING> for &std::ffi::OsString {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == ***self
    }
}

#[cfg(windows)]
impl PartialEq<&HSTRING> for std::ffi::OsString {
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

#[cfg(windows)]
impl<'a> core::convert::From<&'a HSTRING> for std::ffi::OsString {
    fn from(hstring: &HSTRING) -> Self {
        hstring.to_os_string()
    }
}

#[cfg(windows)]
impl core::convert::From<HSTRING> for std::ffi::OsString {
    fn from(hstring: HSTRING) -> Self {
        Self::from(&hstring)
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
    count: RefCount,
    buffer_start: u16,
}

impl Header {
    fn alloc(len: u32) -> *mut Header {
        debug_assert!(len != 0);
        // Allocate enough space for header and two bytes per character.
        // The space for the terminating null character is already accounted for inside of `Header`.
        let alloc_size = core::mem::size_of::<Header>() + 2 * len as usize;

        // TODO: allow this failure to propagate
        let header = heap_alloc(alloc_size).expect("Could not successfully allocate for HSTRING") as *mut Header;

        // SAFETY: uses `std::ptr::write` (since `header` is unintialized). `Header` is safe to be all zeros.
        unsafe {
            header.write(std::mem::MaybeUninit::<Header>::zeroed().assume_init());
            (*header).len = len;
            (*header).count = RefCount::new(1);
            (*header).data = &mut (*header).buffer_start;
        }
        header
    }

    fn duplicate(&mut self) -> *mut Header {
        if self.flags & REFERENCE_FLAG == 0 {
            // If this is not a "fast pass" string then simply increment the reference count.
            self.count.add_ref();
            self
        } else {
            // Otherwise, allocate a new string and copy the value into the new string.
            let copy = Header::alloc(self.len);
            // SAFETY: since we are duplicating the string it is safe to copy all data from self to the initialized `copy`.
            // We copy `len + 1` characters since `len` does not account for the terminating null character.
            unsafe {
                core::ptr::copy_nonoverlapping(self.data, (*copy).data, self.len as usize + 1);
            }
            copy
        }
    }
}
