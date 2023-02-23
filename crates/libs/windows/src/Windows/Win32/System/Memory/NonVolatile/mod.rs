#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlDrainNonVolatileFlush(nvtoken: *const ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlDrainNonVolatileFlush ( nvtoken : *const ::core::ffi::c_void ) -> u32 );
    RtlDrainNonVolatileFlush(nvtoken)
}
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFillNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvdestination: *mut ::core::ffi::c_void, size: usize, value: u8, flags: u32) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlFillNonVolatileMemory ( nvtoken : *const ::core::ffi::c_void , nvdestination : *mut ::core::ffi::c_void , size : usize , value : u8 , flags : u32 ) -> u32 );
    RtlFillNonVolatileMemory(nvtoken, nvdestination, size, value, flags)
}
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvbuffer: *const ::core::ffi::c_void, size: usize, flags: u32) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlFlushNonVolatileMemory ( nvtoken : *const ::core::ffi::c_void , nvbuffer : *const ::core::ffi::c_void , size : usize , flags : u32 ) -> u32 );
    RtlFlushNonVolatileMemory(nvtoken, nvbuffer, size, flags)
}
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemoryRanges(nvtoken: *const ::core::ffi::c_void, nvranges: &[NV_MEMORY_RANGE], flags: u32) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlFlushNonVolatileMemoryRanges ( nvtoken : *const ::core::ffi::c_void , nvranges : *const NV_MEMORY_RANGE , numranges : usize , flags : u32 ) -> u32 );
    RtlFlushNonVolatileMemoryRanges(nvtoken, ::core::mem::transmute(nvranges.as_ptr()), nvranges.len() as _, flags)
}
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFreeNonVolatileToken(nvtoken: *const ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlFreeNonVolatileToken ( nvtoken : *const ::core::ffi::c_void ) -> u32 );
    RtlFreeNonVolatileToken(nvtoken)
}
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlGetNonVolatileToken(nvbuffer: *const ::core::ffi::c_void, size: usize, nvtoken: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlGetNonVolatileToken ( nvbuffer : *const ::core::ffi::c_void , size : usize , nvtoken : *mut *mut ::core::ffi::c_void ) -> u32 );
    RtlGetNonVolatileToken(nvbuffer, size, nvtoken)
}
#[doc = "*Required features: `\"Win32_System_Memory_NonVolatile\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlWriteNonVolatileMemory(nvtoken: *const ::core::ffi::c_void, nvdestination: *mut ::core::ffi::c_void, source: *const ::core::ffi::c_void, size: usize, flags: u32) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlWriteNonVolatileMemory ( nvtoken : *const ::core::ffi::c_void , nvdestination : *mut ::core::ffi::c_void , source : *const ::core::ffi::c_void , size : usize , flags : u32 ) -> u32 );
    RtlWriteNonVolatileMemory(nvtoken, nvdestination, source, size, flags)
}
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
impl ::windows::core::TypeKind for NV_MEMORY_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NV_MEMORY_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for NV_MEMORY_RANGE {}
impl ::core::default::Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
