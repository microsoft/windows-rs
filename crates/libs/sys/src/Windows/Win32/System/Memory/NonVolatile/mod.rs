#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ntdll.dll" "system" fn RtlDrainNonVolatileFlush(nvtoken : *mut core::ffi::c_void) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ntdll.dll" "system" fn RtlFillNonVolatileMemory(nvtoken : *mut core::ffi::c_void, nvdestination : *mut core::ffi::c_void, size : usize, value : u8, flags : u32) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ntdll.dll" "system" fn RtlFlushNonVolatileMemory(nvtoken : *const core::ffi::c_void, nvbuffer : *const core::ffi::c_void, size : usize, flags : u32) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ntdll.dll" "system" fn RtlFlushNonVolatileMemoryRanges(nvtoken : *mut core::ffi::c_void, nvranges : *mut NV_MEMORY_RANGE, numranges : usize, flags : u32) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ntdll.dll" "system" fn RtlFreeNonVolatileToken(nvtoken : *const core::ffi::c_void) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ntdll.dll" "system" fn RtlGetNonVolatileToken(nvbuffer : *mut core::ffi::c_void, size : usize, nvtoken : *mut *mut core::ffi::c_void) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("ntdll.dll" "system" fn RtlWriteNonVolatileMemory(nvtoken : *mut core::ffi::c_void, nvdestination : *mut core::ffi::c_void, source : *mut core::ffi::c_void, size : usize, flags : u32) -> u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut core::ffi::c_void,
    pub Length: usize,
}
impl Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
