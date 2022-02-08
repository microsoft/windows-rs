use super::*;
use bindings::*;

/// A WinRT array stores elements contiguously in a heap-allocated buffer.
pub struct Array<T: RuntimeType> {
    data: *mut T::DefaultType,
    len: u32,
}

impl<T: RuntimeType> Default for Array<T> {
    fn default() -> Self {
        Array { data: core::ptr::null_mut(), len: 0 }
    }
}

impl<T: RuntimeType> Array<T> {
    /// Creates an empty array.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an array of the given length with default values.
    pub fn with_len(len: usize) -> Self {
        assert!(len < core::u32::MAX as usize);
        let bytes_amount = len.checked_mul(core::mem::size_of::<T>()).expect("Attempted to allocate too large an Array");

        // WinRT arrays must be allocated with CoTaskMemAlloc.
        // SAFETY: the call to CoTaskMemAlloc is safe to perform
        // if len is zero and overflow was checked above.
        // We ensured we alloc enough space by multiplying len * size_of::<T>
        let data = unsafe { CoTaskMemAlloc(bytes_amount) as *mut T::DefaultType };

        assert!(!data.is_null(), "Could not successfully allocate for Array");

        // SAFETY: It is by definition safe to zero-initialize WinRT types.
        // `write_bytes` will write 0 to (len * size_of::<T>())
        // bytes making the entire array zero initialized. We have assured
        // above that the data ptr is not null.
        unsafe {
            core::ptr::write_bytes(data, 0, len);
        }

        let len = len as u32;
        Self { data, len }
    }

    /// Creates an array by copying the elements from the slice.
    pub fn from_slice(values: &[T::DefaultType]) -> Self
    where
        T::DefaultType: Clone,
    {
        let mut array = Self::with_len(values.len());
        array.clone_from_slice(values);
        array
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

        let mut data = core::ptr::null_mut();
        let mut len = 0;

        core::mem::swap(&mut data, &mut self.data);
        core::mem::swap(&mut len, &mut self.len);

        // SAFETY: At this point, self has been reset to zero so any panics in T's destructor would
        // only leak data not leave the array in bad state.
        unsafe {
            // Call the destructors of all the elements of the old array
            // SAFETY: the slice cannot be used after the call to `drop_in_place`
            core::ptr::drop_in_place(core::slice::from_raw_parts_mut(data, len as usize));
            // Free the data memory where the elements were
            // SAFETY: we have unique access to the data pointer at this point
            // so freeing it is the right thing to do
            CoTaskMemFree(data as _);
        }
    }

    #[doc(hidden)]
    /// Get a mutable pointer to the array's length
    ///
    /// This function is safe but writing to the pointer is not. Calling this without
    /// a subsequent call to `set_abi` is likely to either leak memory or cause UB
    pub unsafe fn set_abi_len(&mut self) -> *mut u32 {
        &mut self.len
    }

    #[doc(hidden)]
    /// Turn the array into a pointer to its data and its length
    pub fn into_abi(self) -> (*mut T::Abi, u32) {
        let abi = (self.data as *mut _, self.len);
        core::mem::forget(self);
        abi
    }
}

impl<T: RuntimeType> core::ops::Deref for Array<T> {
    type Target = [T::DefaultType];

    fn deref(&self) -> &[T::DefaultType] {
        if self.is_empty() {
            return &[];
        }

        // SAFETY: data must not be null if the array is not empty
        unsafe { core::slice::from_raw_parts(self.data, self.len as usize) }
    }
}

impl<T: RuntimeType> core::ops::DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T::DefaultType] {
        if self.is_empty() {
            return &mut [];
        }

        // SAFETY: data must not be null if the array is not empty
        unsafe { core::slice::from_raw_parts_mut(self.data, self.len as usize) }
    }
}

impl<T: RuntimeType> Drop for Array<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[doc(hidden)]
pub struct ArrayProxy<T: RuntimeType> {
    data: *mut *mut T::DefaultType,
    len: *mut u32,
    temp: core::mem::ManuallyDrop<Array<T>>,
}

impl<T: RuntimeType> ArrayProxy<T> {
    pub fn from_raw_parts(data: *mut *mut T::DefaultType, len: *mut u32) -> Self {
        Self { data, len, temp: core::mem::ManuallyDrop::new(Array::new()) }
    }

    pub fn as_array(&mut self) -> &mut Array<T> {
        &mut self.temp
    }
}

impl<T: RuntimeType> Drop for ArrayProxy<T> {
    fn drop(&mut self) {
        unsafe {
            *self.data = self.temp.data;
            *self.len = self.temp.len;
        }
    }
}
