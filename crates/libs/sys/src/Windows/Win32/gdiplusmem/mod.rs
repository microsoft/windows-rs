#[cfg(feature = "shared")]
windows_link::link!("gdiplus.dll" "system" fn GdipAlloc(size : super::size_t) -> *mut core::ffi::c_void);
windows_link::link!("gdiplus.dll" "system" fn GdipFree(ptr : *mut core::ffi::c_void));
