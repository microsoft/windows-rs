pub type HSTRING_BUFFER = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct HSTRING_HEADER {
    pub Reserved: HSTRING_HEADER_0,
}
#[cfg(target_arch = "x86")]
impl Default for HSTRING_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union HSTRING_HEADER_0 {
    pub Reserved1: *mut core::ffi::c_void,
    pub Reserved2: [i8; 20],
}
#[cfg(target_arch = "x86")]
impl Default for HSTRING_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct HSTRING_HEADER {
    pub Reserved: HSTRING_HEADER_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for HSTRING_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union HSTRING_HEADER_0 {
    pub Reserved1: *mut core::ffi::c_void,
    pub Reserved2: [i8; 24],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for HSTRING_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
