use super::*;
use core::ffi::c_void;

/// Allocate memory of size `bytes` using `HeapAlloc`.
pub fn heap_alloc(bytes: usize) -> crate::Result<*mut c_void> {
    #[cfg(windows)]
    let ptr: *mut c_void = unsafe { bindings::HeapAlloc(bindings::GetProcessHeap(), 0, bytes) };

    #[cfg(not(windows))]
    let ptr: *mut c_void = unsafe {
        extern "C" {
            fn malloc(bytes: usize) -> *mut c_void;
        }

        malloc(bytes)
    };

    if ptr.is_null() {
        Err(Error::from_hresult(HRESULT(bindings::E_OUTOFMEMORY)))
    } else {
        Ok(ptr)
    }
}

/// Free memory allocated by `heap_alloc`.
pub unsafe fn heap_free(ptr: *mut c_void) {
    #[cfg(windows)]
    {
        bindings::HeapFree(bindings::GetProcessHeap(), 0, ptr);
    }

    #[cfg(not(windows))]
    {
        extern "C" {
            fn free(ptr: *mut c_void);
        }

        free(ptr);
    }
}
