use crate::*;

/// A WinRT array
pub struct Array<T: RuntimeType> {
    data: *mut T,
    len: u32,
}

impl<T: RuntimeType> Default for Array<T> {
    fn default() -> Self {
        Array {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl<T: RuntimeType> Array<T> {
    /// Creates an empty array.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an array of the given length with default values.
    pub fn with_len(len: usize) -> Self {
        assert!(len < std::u32::MAX as usize);

        // WinRT arrays must be allocated with CoTaskMemAlloc.
        let data = unsafe { CoTaskMemAlloc(len * std::mem::size_of::<T>()) as *mut T };

        if data.is_null() {
            panic!("Could not successfully allocate for Array");
        }

        // It is safe to zero-initialize WinRT types.
        unsafe {
            std::ptr::write_bytes(data, 0, len);
        }

        let len = len as u32;
        Self { data, len }
    }

    /// Returns `true` if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Clears the contents of the array.
    pub fn clear(&mut self) {
        if self.is_empty() {
            return;
        }

        let mut data = std::ptr::null_mut();
        let mut len = 0;

        std::mem::swap(&mut data, &mut self.data);
        std::mem::swap(&mut len, &mut self.len);

        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(data, len as usize));
            CoTaskMemFree(data as _);
        }
    }

    pub fn set_abi_len(&mut self) -> *mut u32 {
        &mut self.len
    }

    pub fn set_abi(&mut self) -> *mut *mut T::Abi {
        self.clear();
        &mut self.data as *mut _ as *mut _
    }

    pub fn into_abi(self) -> (*mut T::Abi, u32) {
        let abi = (self.data as *mut _, self.len);
        std::mem::forget(self);
        abi
    }
}

impl<T: RuntimeType> std::ops::Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        if self.is_empty() {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
    }
}

impl<T: RuntimeType> std::ops::DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        if self.is_empty() {
            return &mut [];
        }

        unsafe { std::slice::from_raw_parts_mut(self.data, self.len as usize) }
    }
}

impl<T: RuntimeType> Drop for Array<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let empty = Array::<bool>::new();
        assert!(empty.is_empty());
        assert!(empty.len() == 0);
    }

    #[test]
    fn with_len() {
        let empty = Array::<u32>::with_len(3);
        assert!(!empty.is_empty());
        assert!(empty.len() == 3);
        assert!(empty[0] == 0);
        assert!(empty[1] == 0);
        assert!(empty[2] == 0);
    }
}
