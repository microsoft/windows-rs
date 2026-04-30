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
    /// Bytes required to back an `HStringHeader` whose `len` field is `len`.
    /// The space for the terminating null character is already accounted for
    /// inside of `HStringHeader` (via `buffer_start`).
    fn alloc_bytes(len: u32) -> usize {
        core::mem::size_of::<Self>() + 2 * len as usize
    }

    pub fn alloc(len: u32) -> *mut Self {
        if len == 0 {
            return core::ptr::null_mut();
        }

        let bytes = Self::alloc_bytes(len);
        let header = unsafe { heap_alloc(bytes) } as *mut Self;

        if header.is_null() {
            panic!("allocation failed");
        }

        unsafe {
            // Use `ptr::write` (since `header` is uninitialized).
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
            // If this is not a "fast pass" string then simply increment the reference count.
            self.count.add_ref();
            self as *const Self as *mut Self
        } else {
            // Otherwise, allocate a new string and copy the value into the new string.
            let copy = Self::alloc(self.len);
            // SAFETY: since we are duplicating the string it is safe to copy all data from self to the initialized `copy`.
            // We copy `len + 1` characters since `len` does not account for the terminating null character.
            unsafe {
                core::ptr::copy_nonoverlapping(self.data, (*copy).data, self.len as usize + 1);
            }
            copy
        }
    }
}

// HSTRING storage is allocated through the host platform's heap on Windows
// (so values allocated here are compatible with native callers that free
// them via `WindowsDeleteString` / `HeapFree`) and through the Rust global
// allocator on other platforms. The two helpers below are the only place in
// the crate that knows about either allocator.

/// Allocates `bytes` bytes of memory suitable to back an `HStringHeader`.
///
/// Returns null on allocation failure.
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
        // `Layout::from_size_align` only fails when `align` is not a power of
        // two or the rounded-up size would exceed `isize::MAX`. `align_of::<T>`
        // is a power of two by construction, and `bytes` is bounded by
        // `size_of::<HStringHeader>() + 2 * u32::MAX`, well under `isize::MAX`
        // on every supported target.
        let layout = alloc::alloc::Layout::from_size_align(bytes, ALIGN).unwrap();
        unsafe { alloc::alloc::alloc(layout) }
    }
}

/// Frees a block previously returned by `heap_alloc`.
///
/// # Safety
/// `ptr` must be the result of an `heap_alloc(bytes)` call that has not yet
/// been freed.
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
        // SAFETY: `bytes` and `ALIGN` match a previous successful call to
        // `heap_alloc(bytes)` (see safety contract above), so
        // `Layout::from_size_align` cannot fail here.
        let layout = alloc::alloc::Layout::from_size_align(bytes, ALIGN).unwrap();
        unsafe { alloc::alloc::dealloc(ptr, layout) };
    }
}

#[cfg(not(windows))]
const ALIGN: usize = core::mem::align_of::<HStringHeader>();
