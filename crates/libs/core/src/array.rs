use super::*;

/// A WinRT array stores elements contiguously in a heap-allocated buffer.
pub struct Array<T: Type<T>> {
    pub(crate) data: *mut T::Default,
    pub(crate) len: u32,
}

impl<T: Type<T>> Default for Array<T> {
    fn default() -> Self {
        Self {
            data: core::ptr::null_mut(),
            len: 0,
        }
    }
}

impl<T: Type<T>> Array<T> {
    /// Creates an empty array.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an array of the given length with default values.
    pub fn with_len(len: usize) -> Self {
        assert!(len < u32::MAX as usize);
        let bytes_amount = len
            .checked_mul(core::mem::size_of::<T>())
            .expect("Attempted to allocate too large an Array");

        // SAFETY: `bytes_amount` was computed via a checked multiplication
        // above, so the underlying allocator can never be asked for more than
        // `isize::MAX` bytes. `array_alloc::<T>` returns a pointer with
        // alignment at least `align_of::<T>()`.
        let data = unsafe { array_alloc::<T>(bytes_amount) as *mut T::Default };

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
    /// On Windows, `data` must have been allocated with `CoTaskMemAlloc` so that the buffer remains
    /// compatible with native callers that free it via `CoTaskMemFree`. On other targets, `data`
    /// must have been allocated by an `Array<T>` constructor (which uses the Rust global allocator),
    /// since no Windows ABI consumer is in play.
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

        let mut data = core::ptr::null_mut();
        let mut len = 0;

        core::mem::swap(&mut data, &mut self.data);
        core::mem::swap(&mut len, &mut self.len);

        // SAFETY: At this point, self has been reset to zero so any panics in T's destructor would
        // only leak data not leave the array in bad state.
        unsafe {
            // Call the destructors of all the elements of the old array
            // SAFETY: the slice cannot be used after the call to `drop_in_place`
            core::ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(data, len as usize));
            // Free the data memory where the elements were
            // SAFETY: we have unique access to the data pointer at this point,
            // and `len` matches the number of elements that were allocated, so
            // the byte count handed to `array_free` matches the prior alloc.
            let bytes_amount = (len as usize) * core::mem::size_of::<T>();
            array_free::<T>(data as *mut u8, bytes_amount);
        }
    }

    #[doc(hidden)]
    /// Get a mutable pointer to the array's length
    ///
    /// # Safety
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

impl<T: Type<T>> core::ops::Deref for Array<T> {
    type Target = [T::Default];

    fn deref(&self) -> &[T::Default] {
        if self.is_empty() {
            return &[];
        }

        // SAFETY: data must not be null if the array is not empty
        unsafe { core::slice::from_raw_parts(self.data, self.len as usize) }
    }
}

impl<T: Type<T>> core::ops::DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T::Default] {
        if self.is_empty() {
            return &mut [];
        }

        // SAFETY: data must not be null if the array is not empty
        unsafe { core::slice::from_raw_parts_mut(self.data, self.len as usize) }
    }
}

impl<T: Type<T>> Drop for Array<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T: Type<T>> core::fmt::Debug for Array<T>
where
    T::Default: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::ops::Deref::deref(self).fmt(f)
    }
}

// `Array<T>`'s buffer is allocated through `CoTaskMemAlloc` on Windows so that
// values handed off across the WinRT ABI remain compatible with native callers
// that free them via `CoTaskMemFree`. On other targets (where there is no
// Windows ABI consumer) the Rust global allocator is used. The two helpers
// below are the only place in the module that knows about either allocator.

/// Allocates `bytes` bytes of memory suitable to back an `Array<T>`.
///
/// Returns null on allocation failure.
///
/// # Safety
/// `bytes` must equal `n * size_of::<T>()` for some `n` (it may be zero), and
/// must not exceed `isize::MAX`.
unsafe fn array_alloc<T>(bytes: usize) -> *mut u8 {
    #[cfg(windows)]
    {
        // SAFETY: `CoTaskMemAlloc(0)` is allowed and may return null.
        unsafe { imp::CoTaskMemAlloc(bytes) as *mut u8 }
    }
    #[cfg(not(windows))]
    {
        if bytes == 0 {
            // `Layout::from_size_align` allows zero-sized allocations, but
            // `alloc::alloc::alloc` is UB on a zero-sized layout. WinRT-shaped
            // empty arrays usually skip allocation entirely; mirror that here.
            return core::ptr::null_mut();
        }
        // `Layout::from_size_align` only fails when `align` is not a power of
        // two or `bytes` rounded up exceeds `isize::MAX`. `align_of::<T>` is a
        // power of two by construction, and `bytes` is bounded by the caller.
        let layout =
            alloc::alloc::Layout::from_size_align(bytes, core::mem::align_of::<T>()).unwrap();
        unsafe { alloc::alloc::alloc(layout) }
    }
}

/// Frees a block previously returned by `array_alloc::<T>(bytes)`.
///
/// # Safety
/// `ptr` must be the result of an `array_alloc::<T>(bytes)` call that has not
/// yet been freed, with the same `T` and `bytes`.
unsafe fn array_free<T>(ptr: *mut u8, bytes: usize) {
    #[cfg(windows)]
    {
        let _ = bytes;
        // SAFETY: per the safety contract above.
        unsafe { imp::CoTaskMemFree(ptr as _) };
    }
    #[cfg(not(windows))]
    {
        if bytes == 0 {
            // Mirrors the zero-byte short-circuit in `array_alloc`.
            return;
        }
        // SAFETY: `bytes` and `align_of::<T>()` match the previous successful
        // call to `array_alloc::<T>(bytes)` (per the safety contract above),
        // so `Layout::from_size_align` cannot fail here.
        let layout =
            alloc::alloc::Layout::from_size_align(bytes, core::mem::align_of::<T>()).unwrap();
        unsafe { alloc::alloc::dealloc(ptr, layout) };
    }
}
