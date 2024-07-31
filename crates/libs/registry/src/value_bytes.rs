use super::*;

// Minimal `Vec<u8>` replacement providing at least `u16` alignment so that it can be used for wide strings.
pub struct ValueBytes {
    ptr: *mut u8,
    len: usize,
}

impl ValueBytes {
    // Creates a buffer with the specified length of zero bytes.
    pub fn new(len: usize) -> Result<Self> {
        unsafe {
            let bytes = Self::alloc(len)?;

            if len > 0 {
                core::ptr::write_bytes(bytes.ptr, 0, len);
            }

            Ok(bytes)
        }
    }

    // Returns the buffer as a slice of u16 for reading wide characters. The slice trims off any trailing zero bytes.
    pub fn as_wide(&self) -> &[u16] {
        if self.ptr.is_null() {
            &[]
        } else {
            let mut wide =
                unsafe { core::slice::from_raw_parts(self.ptr as *const u16, self.len / 2) };

            while wide.last() == Some(&0) {
                wide = &wide[..wide.len() - 1];
            }

            wide
        }
    }

    // Creates a buffer by copying the bytes from the slice.
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        unsafe {
            let bytes = Self::alloc(slice.len())?;

            if !slice.is_empty() {
                core::ptr::copy_nonoverlapping(slice.as_ptr(), bytes.ptr, slice.len());
            }

            Ok(bytes)
        }
    }

    // Allocates an uninitialized buffer.
    unsafe fn alloc(len: usize) -> Result<Self> {
        if len == 0 {
            Ok(Self {
                ptr: null_mut(),
                len: 0,
            })
        } else {
            // This pointer will have at least 8 byte alignment.
            let ptr = HeapAlloc(GetProcessHeap(), 0, len) as *mut u8;

            if ptr.is_null() {
                Err(Error::from_hresult(HRESULT::from_win32(ERROR_OUTOFMEMORY)))
            } else {
                Ok(Self { ptr, len })
            }
        }
    }
}

impl Drop for ValueBytes {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                HeapFree(GetProcessHeap(), 0, self.ptr as *mut _);
            }
        }
    }
}

impl Deref for ValueBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        if self.ptr.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
        }
    }
}

impl core::ops::DerefMut for ValueBytes {
    fn deref_mut(&mut self) -> &mut [u8] {
        if self.ptr.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
        }
    }
}

impl Clone for ValueBytes {
    fn clone(&self) -> Self {
        Self::from_slice(self).unwrap()
    }
}

impl PartialEq for ValueBytes {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl Eq for ValueBytes {}

impl core::fmt::Debug for ValueBytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<const N: usize> TryFrom<[u8; N]> for ValueBytes {
    type Error = Error;
    fn try_from(from: [u8; N]) -> Result<Self> {
        Self::from_slice(&from)
    }
}
