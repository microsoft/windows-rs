use super::*;

pub const HSTRING_REFERENCE_FLAG: u32 = 1;

#[repr(C)]
pub struct HStringHeader {
    pub flags: u32,
    pub len: u32,
    pub _0: u32,
    pub _1: u32,
    pub data: *mut u16,
    pub count: RefCount,
    pub buffer_start: u16,
}

impl HStringHeader {
    /// Bytes required for `len` code units plus the header's inline terminator.
    fn alloc_bytes(len: u32) -> usize {
        size_of::<Self>() + 2 * len as usize
    }

    pub fn alloc(len: u32) -> *mut Self {
        if len == 0 {
            return core::ptr::null_mut();
        }

        let bytes = Self::alloc_bytes(len);
        let header = unsafe { heap_alloc(bytes) } as *mut Self;

        assert!(!header.is_null(), "allocation failed");

        unsafe {
            header.write(Self {
                flags: 0,
                len,
                _0: 0,
                _1: 0,
                data: core::ptr::null_mut(), // set below
                count: RefCount::new(1),
                buffer_start: 0,
            });
            (*header).data = &mut (*header).buffer_start;
        }

        header
    }

    pub unsafe fn free(header: *mut Self) {
        if header.is_null() {
            return;
        }

        unsafe {
            let bytes = Self::alloc_bytes((*header).len);
            heap_free(header as *mut u8, bytes);
        }
    }

    pub fn duplicate(&self) -> *mut Self {
        if self.flags & HSTRING_REFERENCE_FLAG == 0 {
            self.count.add_ref();
            self as *const Self as *mut Self
        } else {
            let copy = Self::alloc(self.len);
            // SAFETY: `copy` is initialized and sized for `len + 1`, including the terminator.
            unsafe {
                core::ptr::copy_nonoverlapping(self.data, (*copy).data, self.len as usize + 1);
            }
            copy
        }
    }
}

// HSTRING storage uses the host platform's heap on Windows (compatible with
// `WindowsDeleteString` / `HeapFree`) and the Rust global allocator elsewhere.

/// Allocates memory for an `HStringHeader`, returning null on failure.
///
/// # Safety
/// `bytes` must be non-zero.
unsafe fn heap_alloc(bytes: usize) -> *mut u8 {
    #[cfg(windows)]
    {
        unsafe { bindings::HeapAlloc(bindings::GetProcessHeap(), 0, bytes) as *mut u8 }
    }
    #[cfg(not(windows))]
    {
        // The alignment is fixed, and callers supply sizes accepted by the allocator.
        let layout = alloc::alloc::Layout::from_size_align(bytes, ALIGN).unwrap();
        unsafe { alloc::alloc::alloc(layout) }
    }
}

/// Frees a block previously returned by `heap_alloc`.
///
/// # Safety
/// `ptr` must come from an unfreed `heap_alloc(bytes)` call.
unsafe fn heap_free(ptr: *mut u8, bytes: usize) {
    #[cfg(windows)]
    {
        let _ = bytes;
        unsafe {
            bindings::HeapFree(bindings::GetProcessHeap(), 0, ptr as *mut _);
        }
    }
    #[cfg(not(windows))]
    {
        // SAFETY: `bytes` and `ALIGN` match the allocation contract above.
        let layout = alloc::alloc::Layout::from_size_align(bytes, ALIGN).unwrap();
        unsafe { alloc::alloc::dealloc(ptr, layout) };
    }
}

#[cfg(not(windows))]
const ALIGN: usize = align_of::<HStringHeader>();
