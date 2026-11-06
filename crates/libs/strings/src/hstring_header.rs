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
        size_of::<Self>()
            .checked_add((len as usize).checked_mul(2).unwrap())
            .unwrap()
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
            (*header).data = &raw mut (*header).buffer_start;
        }

        header
    }

    pub unsafe fn free(header: *mut Self) {
        if header.is_null() {
            return;
        }

        unsafe {
            heap_free(header.cast());
        }
    }

    pub unsafe fn duplicate(header: *mut Self) -> *mut Self {
        unsafe {
            if header.is_null() {
                return header;
            }

            if (*header).flags & HSTRING_REFERENCE_FLAG == 0 {
                (*header).count.add_ref();
                header
            } else {
                let copy = Self::alloc((*header).len);
                // Fast-pass strings keep the terminator in the referenced buffer.
                core::ptr::copy_nonoverlapping(
                    (*header).data,
                    (*copy).data,
                    (*header).len as usize + 1,
                );
                copy
            }
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
        // The logical string length can shrink. Keep the allocation size outside the ABI header.
        let layout =
            alloc::alloc::Layout::from_size_align(bytes.checked_add(PREFIX).unwrap(), ALIGN)
                .unwrap();
        unsafe {
            let allocation = alloc::alloc::alloc(layout);
            if allocation.is_null() {
                return allocation;
            }
            allocation.cast::<usize>().write(layout.size());
            allocation.add(PREFIX)
        }
    }
}

/// Frees a block previously returned by `heap_alloc`.
///
/// # Safety
/// `ptr` must come from an unfreed `heap_alloc` call.
unsafe fn heap_free(ptr: *mut u8) {
    #[cfg(windows)]
    {
        unsafe {
            bindings::HeapFree(bindings::GetProcessHeap(), 0, ptr as *mut _);
        }
    }
    #[cfg(not(windows))]
    {
        unsafe {
            let allocation = ptr.sub(PREFIX);
            let bytes = allocation.cast::<usize>().read();
            let layout = alloc::alloc::Layout::from_size_align(bytes, ALIGN).unwrap();
            alloc::alloc::dealloc(allocation, layout);
        }
    }
}

#[cfg(not(windows))]
const ALIGN: usize = align_of::<HStringHeader>();

#[cfg(not(windows))]
const PREFIX: usize = size_of::<usize>().next_multiple_of(ALIGN);
