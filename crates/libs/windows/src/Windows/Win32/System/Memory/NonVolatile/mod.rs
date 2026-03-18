#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlDrainNonVolatileFlush(nvtoken: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlDrainNonVolatileFlush(nvtoken : *mut core::ffi::c_void) -> u32);
    unsafe { RtlDrainNonVolatileFlush(nvtoken as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFillNonVolatileMemory(nvtoken: *mut core::ffi::c_void, nvdestination: *mut core::ffi::c_void, size: usize, value: u8, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFillNonVolatileMemory(nvtoken : *mut core::ffi::c_void, nvdestination : *mut core::ffi::c_void, size : usize, value : u8, flags : u32) -> u32);
    unsafe { RtlFillNonVolatileMemory(nvtoken as _, nvdestination as _, size, value, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemory(nvtoken: *mut core::ffi::c_void, nvbuffer: *mut core::ffi::c_void, size: usize, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFlushNonVolatileMemory(nvtoken : *mut core::ffi::c_void, nvbuffer : *mut core::ffi::c_void, size : usize, flags : u32) -> u32);
    unsafe { RtlFlushNonVolatileMemory(nvtoken as _, nvbuffer as _, size, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemoryRanges(nvtoken: *mut core::ffi::c_void, nvranges: *mut NV_MEMORY_RANGE, numranges: usize, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFlushNonVolatileMemoryRanges(nvtoken : *mut core::ffi::c_void, nvranges : *mut NV_MEMORY_RANGE, numranges : usize, flags : u32) -> u32);
    unsafe { RtlFlushNonVolatileMemoryRanges(nvtoken as _, nvranges as _, numranges, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFreeNonVolatileToken(nvtoken: *const core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeNonVolatileToken(nvtoken : *const core::ffi::c_void) -> u32);
    unsafe { RtlFreeNonVolatileToken(nvtoken) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlGetNonVolatileToken(nvbuffer: *mut core::ffi::c_void, size: usize, nvtoken: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlGetNonVolatileToken(nvbuffer : *mut core::ffi::c_void, size : usize, nvtoken : *mut *mut core::ffi::c_void) -> u32);
    unsafe { RtlGetNonVolatileToken(nvbuffer as _, size, nvtoken as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlWriteNonVolatileMemory(nvtoken: *const core::ffi::c_void, nvdestination: *mut core::ffi::c_void, source: *const core::ffi::c_void, size: usize, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlWriteNonVolatileMemory(nvtoken : *const core::ffi::c_void, nvdestination : *mut core::ffi::c_void, source : *const core::ffi::c_void, size : usize, flags : u32) -> u32);
    unsafe { RtlWriteNonVolatileMemory(nvtoken, nvdestination as _, source, size, flags) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut core::ffi::c_void,
    pub Length: usize,
}
impl Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
