#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub Length: usize,
}
impl ::core::marker::Copy for NV_MEMORY_RANGE {}
impl ::core::clone::Clone for NV_MEMORY_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NV_MEMORY_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NV_MEMORY_RANGE").field("BaseAddress", &self.BaseAddress).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for NV_MEMORY_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NV_MEMORY_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NV_MEMORY_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NV_MEMORY_RANGE {}
impl ::core::default::Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemoryRanges(nvtoken: *const ::core::ffi::c_void, nvranges: &[NV_MEMORY_RANGE], flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFlushNonVolatileMemoryRanges(nvtoken: *const ::core::ffi::c_void, nvranges: *const NV_MEMORY_RANGE, numranges: usize, flags: u32) -> u32;
        }
        ::core::mem::transmute(RtlFlushNonVolatileMemoryRanges(::core::mem::transmute(nvtoken), ::core::mem::transmute(::windows::core::as_ptr_or_null(nvranges)), nvranges.len() as _, ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
