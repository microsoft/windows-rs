#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GdipAlloc(size: super::size_t) -> *mut core::ffi::c_void {
    windows_core::link!("gdiplus.dll" "system" fn GdipAlloc(size : super::size_t) -> *mut core::ffi::c_void);
    unsafe { GdipAlloc(size) }
}
#[inline]
pub unsafe fn GdipFree(ptr: *mut core::ffi::c_void) {
    windows_core::link!("gdiplus.dll" "system" fn GdipFree(ptr : *mut core::ffi::c_void));
    unsafe { GdipFree(ptr as _) }
}
