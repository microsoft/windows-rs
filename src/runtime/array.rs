use crate::*;

/// A WinRT array stores elements contiguously in a heap-allocated buffer.
pub struct Array<T: RuntimeType> {
    data: *mut T::DefaultType,
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
        let bytes_amount = len
            .checked_mul(std::mem::size_of::<T>())
            .expect("Attempted to allocate too large an Array");

        // WinRT arrays must be allocated with CoTaskMemAlloc.
        // SAFETY: the call to CoTaskMemAlloc is safe to perform
        // if len is zero and overflow was checked above.
        // We ensured we alloc enough space by multiplying len * size_of::<T>
        let data = unsafe { CoTaskMemAlloc(bytes_amount) as *mut T::DefaultType };

        if data.is_null() {
            panic!("Could not successfully allocate for Array");
        }

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
    pub fn set_abi_len(&mut self) -> *mut u32 {
        &mut self.len
    }

    #[doc(hidden)]
    /// Get a mutable pointer to the array's data
    ///
    /// This function is safe but writing to the pointer is not. Calling this without
    /// a subsequent call to `set_abi_len` is likely to either leak memory or cause UB
    pub fn set_abi(&mut self) -> *mut *mut T::Abi {
        self.clear();
        &mut self.data as *mut _ as *mut _
    }

    #[doc(hidden)]
    /// Turn the array into a pointer to its data and its length
    pub fn into_abi(self) -> (*mut T::Abi, u32) {
        let abi = (self.data as *mut _, self.len);
        std::mem::forget(self);
        abi
    }
}

impl<T: RuntimeType> std::ops::Deref for Array<T> {
    type Target = [T::DefaultType];

    fn deref(&self) -> &[T::DefaultType] {
        if self.is_empty() {
            return &[];
        }

        // SAFETY: data must not be null if the array is not empty
        unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
    }
}

impl<T: RuntimeType> std::ops::DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T::DefaultType] {
        if self.is_empty() {
            return &mut [];
        }

        // SAFETY: data must not be null if the array is not empty
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len as usize) }
    }
}

impl<T: RuntimeType> Drop for Array<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[link(name = "ole32")]
extern "system" {
    fn CoTaskMemAlloc(len: usize) -> RawPtr;
    fn CoTaskMemFree(ptr: RawPtr);
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

    #[test]
    fn uri() {
        use winrt::foundation::Uri;

        let a = Array::<Uri>::new();
        assert!(a.is_empty());

        let mut a = Array::<Uri>::with_len(2);
        assert!(a[0] == None);
        assert!(a[1] == None);

        a[0] = Uri::create_uri("http://kennykerr.ca").ok();
        a[1] = Uri::create_uri("http://microsoft.com").ok();

        // TODO: this seems rather tedious... may warrant a winrt::Option<T> that's more convenient
        // that could handle both nullable and IReference<T> behaviors in a single abstraction.
        assert!(a[0].as_ref().unwrap().domain().unwrap() == "kennykerr.ca");
        assert!(a[1].as_ref().unwrap().domain().unwrap() == "microsoft.com");
    }
}
