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

        unsafe {
            std::ptr::drop_in_place(&mut self[..]);
            CoTaskMemFree(self.data as _);
        }
    }

    pub unsafe fn set_abi_len(&mut self) -> *mut u32 {
        &mut self.len
    }

    pub unsafe fn set_abi(&mut self) -> *mut *mut T::Abi {
        self.clear();
        &mut self.data as *mut _ as *mut _
    }
}

impl<T: RuntimeType> std::ops::Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        if self.data.is_null() {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
    }
}

impl<T: RuntimeType> std::ops::DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        if self.data.is_null() {
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
