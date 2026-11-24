/// Borrows a WinRT input array, which may be null when `len` is zero.
pub unsafe fn array_slice<'a, T>(data: *const T, len: u32) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(data, len as usize) }
    }
}

/// Initializes and borrows a WinRT fill array, which may be null when `len` is zero.
///
/// # Safety
///
/// For nonzero `len`, `data` must point to `len` writable, aligned elements with no other references.
/// Zero must be a valid representation of `T`. Existing contents are output storage, not owned values.
pub unsafe fn array_slice_mut<'a, T>(data: *mut T, len: u32) -> &'a mut [T] {
    if len == 0 {
        &mut []
    } else {
        unsafe {
            core::ptr::write_bytes(data, 0, len as usize);
            core::slice::from_raw_parts_mut(data, len as usize)
        }
    }
}
