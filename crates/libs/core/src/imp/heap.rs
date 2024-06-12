use super::*;
use core::ffi::c_void;

/// Allocate memory of size `bytes` using `HeapAlloc`.
///
/// The memory allocated by this function is uninitialized.
///
/// This function will fail in OOM situations, if the heap is otherwise corrupt,
/// or if getting a handle to the process heap fails.
pub fn heap_alloc(bytes: usize) -> crate::Result<*mut c_void> {
    #[cfg(windows)]
    let ptr: *mut c_void = unsafe { HeapAlloc(GetProcessHeap(), 0, bytes) };

    #[cfg(not(windows))]
    let ptr: *mut c_void = unsafe {
        extern "C" {
            fn malloc(bytes: usize) -> *mut c_void;
        }

        malloc(bytes)
    };

    if ptr.is_null() {
        Err(E_OUTOFMEMORY.into())
    } else {
        // HeapAlloc is not guaranteed to return zero memory but usually does. This just ensures that
        // it predictably returns non-zero memory for testing purposes. This is similar to what MSVC's
        // debug allocator does for the same reason.
        #[cfg(debug_assertions)]
        unsafe {
            core::ptr::write_bytes(ptr, 0xCC, bytes);
        }
        Ok(ptr)
    }
}

/// Free memory allocated by `HeapAlloc` or `HeapReAlloc`.
///
/// The pointer is allowed to be null.
///
/// # Safety
///
/// `ptr` must be a valid pointer to memory allocated by `HeapAlloc` or `HeapReAlloc`
pub unsafe fn heap_free(ptr: *mut c_void) {
    #[cfg(windows)]
    {
        HeapFree(GetProcessHeap(), 0, ptr);
    }

    #[cfg(not(windows))]
    {
        extern "C" {
            fn free(ptr: *mut c_void);
        }

        if !ptr.is_null() {
            free(ptr);
        }
    }
}
