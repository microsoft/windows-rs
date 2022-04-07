use super::*;
use bindings::*;

// TODO: why not Option<RawPtr>
pub fn heap_alloc(bytes: usize) -> Result<RawPtr> {
    let ptr = unsafe { HeapAlloc(GetProcessHeap()?, HEAP_NONE, bytes) };

    if ptr.is_null() {
        Err(E_OUTOFMEMORY.into())
    } else {
        Ok(ptr)
    }
}

/// # Safety
pub unsafe fn heap_free(ptr: RawPtr) {
    if let Ok(heap) = GetProcessHeap() {
        HeapFree(heap, HEAP_NONE, ptr);
    }
}

/// Copy a slice of `T` into a freshly allocated buffer with an additional default `T` at the end.
///
/// Returns a pointer to the beginning of the buffer
pub fn heap_string<T: Copy + Default + Sized>(slice: &[T]) -> *const T {
    unsafe {
        let buffer = heap_alloc((slice.len() + 1) * std::mem::size_of::<T>()).expect("could not allocate string") as *mut T;
        for offset in 0..slice.len() {
            std::ptr::write(buffer.add(offset), slice[offset]);
        }
        std::ptr::write(buffer.add(slice.len()), T::default());
        buffer
    }
}
