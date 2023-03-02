use super::*;
use imp::*;

/// A WinRT array stores elements contiguously in a heap-allocated buffer.
pub struct Array<T: Type<T>> {
    data: *mut T::Default,
    len: u32,
}

impl<T: Type<T>> Default for Array<T> {
    fn default() -> Self {
        Array { data: std::ptr::null_mut(), len: 0 }
    }
}

impl<T: Type<T>> Array<T> {
    /// Creates an empty array.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an array of the given length with default values.
    pub fn with_len(len: usize) -> Self {
        assert!(len < std::u32::MAX as usize);
        let bytes_amount = len.checked_mul(std::mem::size_of::<T>()).expect("Attempted to allocate too large an Array");

        // WinRT arrays must be allocated with CoTaskMemAlloc.
        // SAFETY: the call to CoTaskMemAlloc is safe to perform
        // if len is zero and overflow was checked above.
        // We ensured we alloc enough space by multiplying len * size_of::<T>
        let data = unsafe { CoTaskMemAlloc(bytes_amount) as *mut T::Default };

        assert!(!data.is_null(), "Could not successfully allocate for Array");

        // SAFETY: It is by definition safe to zero-initialize WinRT types.
        // `write_bytes` will write 0 to (len * size_of::<T>())
        // bytes making the entire array zero initialized. We have assured
        // above that the data ptr is not null.
        unsafe {
            std::ptr::write_bytes(data, 0, len);
        }

        let len = len as u32;
        Self { data, len }
    }

    /// Creates an array by copying the elements from the slice.
    pub fn from_slice(values: &[T::Default]) -> Self
    where
        T::Default: Clone,
    {
        let mut array = Self::with_len(values.len());
        array.clone_from_slice(values);
        array
    }

    /// Creates an array from a pointer and length. The `len` argument is the number of elements, not the number of bytes.
    /// # Safety
    /// The `data` argument must have been allocated with `CoTaskMemAlloc`.
    pub unsafe fn from_raw_parts(data: *mut T::Default, len: u32) -> Self {
        Self { data, len }
    }

    /// Returns a slice containing the entire array.
    pub fn as_slice(&self) -> &[T::Default] {
        self
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

        // SAFETY: At this point, self has been reset to zero so any panics in T's destructor would
        // only leak data not leave the array in bad state.
        unsafe {
            // Call the destructors of all the elements of the old array
            // SAFETY: the slice cannot be used after the call to `drop_in_place`
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(data, len as usize));
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
        std::mem::forget(self);
        abi
    }
}

impl<T: Type<T>> std::ops::Deref for Array<T> {
    type Target = [T::Default];

    fn deref(&self) -> &[T::Default] {
        if self.is_empty() {
            return &[];
        }

        // SAFETY: data must not be null if the array is not empty
        unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
    }
}

impl<T: Type<T>> std::ops::DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T::Default] {
        if self.is_empty() {
            return &mut [];
        }

        // SAFETY: data must not be null if the array is not empty
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len as usize) }
    }
}

impl<T: Type<T>> Drop for Array<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[doc(hidden)]
pub struct ArrayProxy<T: Type<T>> {
    data: *mut *mut T::Default,
    len: *mut u32,
    temp: std::mem::ManuallyDrop<Array<T>>,
}

impl<T: Type<T>> ArrayProxy<T> {
    pub fn from_raw_parts(data: *mut *mut T::Default, len: *mut u32) -> Self {
        Self { data, len, temp: std::mem::ManuallyDrop::new(Array::new()) }
    }

    pub fn as_array(&mut self) -> &mut Array<T> {
        &mut self.temp
    }
}

impl<T: Type<T>> Drop for ArrayProxy<T> {
    fn drop(&mut self) {
        unsafe {
            *self.data = self.temp.data;
            *self.len = self.temp.len;
        }
    }
}
