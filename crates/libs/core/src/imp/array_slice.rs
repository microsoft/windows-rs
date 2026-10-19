/// Borrows a WinRT input array, which may be null when `len` is zero.
pub unsafe fn array_slice<'a, T>(data: *const T, len: u32) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(data, len as usize) }
    }
}

/// Borrows a WinRT fill array, which may be null when `len` is zero.
pub unsafe fn array_slice_mut<'a, T>(data: *mut T, len: u32) -> &'a mut [T] {
    if len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(data, len as usize) }
    }
}
