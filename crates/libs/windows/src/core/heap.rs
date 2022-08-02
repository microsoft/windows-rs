use super::*;
use bindings::*;

/// Allocate memory of size `bytes` using `HeapAlloc`.
///
/// The memory allocated by this function is uninitialized.
///
/// This function will fail in OOM situations, if the heap is otherwise corrupt,
/// or if getting a handle to the process heap fails.
pub fn heap_alloc(bytes: usize) -> Result<*mut core::ffi::c_void> {
    let ptr = unsafe { HeapAlloc(GetProcessHeap()?, HEAP_NONE, bytes) };

    if ptr.is_null() {
        Err(E_OUTOFMEMORY.into())
    } else {
        // HeapAlloc is not guaranteed to return zero memory but usually does. This just ensures that
        // it predictably returns non-zero memory for testing purposes. This is similar to what MSVC's
        // debug allocator does for the same reason.
        #[cfg(debug_assertions)]
        unsafe {
            std::ptr::write_bytes(ptr, 0xCC, bytes);
        }
        Ok(ptr)
    }
}

/// Free memory allocated by `HeapAlloc` or `HeapReAlloc`.
///
/// The pointer is allowed to be null. If there is an error getting the process heap,
/// the memory will be leaked.
///
/// # Safety
///
/// `ptr` must be a valid pointer to memory allocated by `HeapAlloc` or `HeapReAlloc`
pub unsafe fn heap_free(ptr: *mut core::ffi::c_void) {
    if let Ok(heap) = GetProcessHeap() {
        HeapFree(heap, HEAP_NONE, ptr);
    }
}
