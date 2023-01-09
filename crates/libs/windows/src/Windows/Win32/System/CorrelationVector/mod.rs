#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    ::windows::core::link ! ( "ntdll.dll""system" fn RtlExtendCorrelationVector ( correlationvector : *mut CORRELATION_VECTOR ) -> u32 );
    RtlExtendCorrelationVector(correlationvector)
}
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    ::windows::core::link ! ( "ntdll.dll""system" fn RtlIncrementCorrelationVector ( correlationvector : *mut CORRELATION_VECTOR ) -> u32 );
    RtlIncrementCorrelationVector(correlationvector)
}
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: ::core::option::Option<*const ::windows::core::GUID>) -> u32 {
    ::windows::core::link ! ( "ntdll.dll""system" fn RtlInitializeCorrelationVector ( correlationvector : *mut CORRELATION_VECTOR , version : i32 , guid : *const :: windows::core::GUID ) -> u32 );
    RtlInitializeCorrelationVector(correlationvector, version, ::core::mem::transmute(guid.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_CorrelationVector\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32 {
    ::windows::core::link ! ( "ntdll.dll""system" fn RtlValidateCorrelationVector ( vector : *const CORRELATION_VECTOR ) -> u32 );
    RtlValidateCorrelationVector(vector)
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
unsafe impl ::windows::core::Abi for CORRELATION_VECTOR {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
