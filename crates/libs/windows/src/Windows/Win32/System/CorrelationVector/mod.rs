#[repr(C)]
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CORRELATION_VECTOR {
    pub Version: super::super::Foundation::CHAR,
    pub Vector: [super::super::Foundation::CHAR; 129],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CORRELATION_VECTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CORRELATION_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CORRELATION_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CORRELATION_VECTOR").field("Version", &self.Version).field("Vector", &self.Vector).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CORRELATION_VECTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CORRELATION_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CORRELATION_VECTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CORRELATION_VECTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CORRELATION_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`*"]
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32;
    }
    RtlExtendCorrelationVector(::core::mem::transmute(correlationvector))
}
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32;
    }
    RtlIncrementCorrelationVector(::core::mem::transmute(correlationvector))
}
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: *const ::windows::core::GUID) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: *const ::windows::core::GUID) -> u32;
    }
    RtlInitializeCorrelationVector(::core::mem::transmute(correlationvector), version, ::core::mem::transmute(guid))
}
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32;
    }
    RtlValidateCorrelationVector(::core::mem::transmute(vector))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
