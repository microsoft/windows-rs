use super::*;
use alloc::alloc::{alloc_zeroed, dealloc, Layout};

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
    fn layout(len: u32) -> Layout {
        // Allocate enough space for header and two bytes per character.
        // The space for the terminating null character is already accounted for inside of `HStringHeader`.
        let bytes = core::mem::size_of::<Self>() + 2 * len as usize;
        Layout::from_size_align(bytes, core::mem::align_of::<Self>()).expect("invalid layout")
    }

    pub fn alloc(len: u32) -> *mut Self {
        if len == 0 {
            return core::ptr::null_mut();
        }

        let header = unsafe { alloc_zeroed(Self::layout(len)) } as *mut Self;

        if header.is_null() {
            panic!("allocation failed");
        }

        unsafe {
            // `alloc_zeroed` zero-initializes the header; only non-zero fields need to be set.
            (*header).len = len;
            (*header).count = RefCount::new(1);
            (*header).data = &mut (*header).buffer_start;
        }

        header
    }

    pub unsafe fn free(header: *mut Self) {
        if header.is_null() {
            return;
        }

        let len = unsafe { (*header).len };
        unsafe { dealloc(header as *mut u8, Self::layout(len)) };
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
