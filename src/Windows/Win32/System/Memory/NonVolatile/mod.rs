#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub Length: usize,
}
impl NV_MEMORY_RANGE {}
impl ::core::default::Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for NV_MEMORY_RANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("NV_MEMORY_RANGE").field("BaseAddress", &self.BaseAddress).field("Length", &self.Length).finish()
    }
}
impl ::core::cmp::PartialEq for NV_MEMORY_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for NV_MEMORY_RANGE {}
unsafe impl ::windows::core::Abi for NV_MEMORY_RANGE {
    type Abi = Self;
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[inline]
pub unsafe fn RtlDrainNonVolatileFlush(nvtoken: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlDrainNonVolatileFlush(nvtoken: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(RtlDrainNonVolatileFlush(::core::mem::transmute(nvtoken)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[inline]
pub unsafe fn RtlFillNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvdestination: *mut ::core::ffi::c_void, size: usize, value: u8, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFillNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvdestination: *mut ::core::ffi::c_void, size: usize, value: u8, flags: u32) -> u32;
        }
        ::core::mem::transmute(RtlFillNonVolatileMemory(::core::mem::transmute(nvtoken), ::core::mem::transmute(nvdestination), ::core::mem::transmute(size), ::core::mem::transmute(value), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvbuffer: *const ::core::ffi::c_void, size: usize, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFlushNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvbuffer: *const ::core::ffi::c_void, size: usize, flags: u32) -> u32;
        }
        ::core::mem::transmute(RtlFlushNonVolatileMemory(::core::mem::transmute(nvtoken), ::core::mem::transmute(nvbuffer), ::core::mem::transmute(size), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemoryRanges(nvtoken: *const ::core::ffi::c_void, nvranges: *const NV_MEMORY_RANGE, numranges: usize, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFlushNonVolatileMemoryRanges(nvtoken: *const ::core::ffi::c_void, nvranges: *const NV_MEMORY_RANGE, numranges: usize, flags: u32) -> u32;
        }
        ::core::mem::transmute(RtlFlushNonVolatileMemoryRanges(::core::mem::transmute(nvtoken), ::core::mem::transmute(nvranges), ::core::mem::transmute(numranges), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[inline]
pub unsafe fn RtlFreeNonVolatileToken(nvtoken: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFreeNonVolatileToken(nvtoken: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(RtlFreeNonVolatileToken(::core::mem::transmute(nvtoken)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[inline]
pub unsafe fn RtlGetNonVolatileToken(nvbuffer: *const ::core::ffi::c_void, size: usize, nvtoken: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlGetNonVolatileToken(nvbuffer: *const ::core::ffi::c_void, size: usize, nvtoken: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(RtlGetNonVolatileToken(::core::mem::transmute(nvbuffer), ::core::mem::transmute(size), ::core::mem::transmute(nvtoken)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[inline]
pub unsafe fn RtlWriteNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvdestination: *mut ::core::ffi::c_void, source: *const ::core::ffi::c_void, size: usize, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlWriteNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvdestination: *mut ::core::ffi::c_void, source: *const ::core::ffi::c_void, size: usize, flags: u32) -> u32;
        }
        ::core::mem::transmute(RtlWriteNonVolatileMemory(::core::mem::transmute(nvtoken), ::core::mem::transmute(nvdestination), ::core::mem::transmute(source), ::core::mem::transmute(size), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
