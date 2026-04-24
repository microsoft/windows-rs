use super::*;

// A byte buffer with at least `u16` alignment so that it can be safely reinterpreted as a wide string.
pub struct Data {
    buf: Vec<u16>,
    len: usize,
}

impl Data {
    // Creates a buffer with the specified length of zero bytes.
    pub fn new(len: usize) -> Self {
        Self {
            buf: vec![0u16; (len + 1) / 2],
            len,
        }
    }

    // Returns the buffer as a slice of u16 for reading wide characters.
    pub fn as_wide(&self) -> &[u16] {
        &self.buf[..self.len / 2]
    }

    // Creates a buffer by copying the bytes from the slice.
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut data = Self::new(slice.len());
        let dest: &mut [u8] = &mut data;
        dest.copy_from_slice(slice);
        data
    }
}

impl Deref for Data {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        let (prefix, bytes, suffix) = self.buf.align_to::<u8>();
        debug_assert!(prefix.is_empty() && suffix.is_empty());
        &bytes[..self.len]
    }
}

impl core::ops::DerefMut for Data {
    fn deref_mut(&mut self) -> &mut [u8] {
        let len = self.len;
        let (prefix, bytes, suffix) = self.buf.align_to_mut::<u8>();
        debug_assert!(prefix.is_empty() && suffix.is_empty());
        &mut bytes[..len]
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        Self::from_slice(self)
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl Eq for Data {}

impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<const N: usize> From<[u8; N]> for Data {
    fn from(from: [u8; N]) -> Self {
        Self::from_slice(&from)
    }
}
