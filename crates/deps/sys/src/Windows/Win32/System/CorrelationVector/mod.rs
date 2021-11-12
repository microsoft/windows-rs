#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_System_CorrelationVector`*"]
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129u32;
#[doc = "*Required features: `Win32_System_CorrelationVector`*"]
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_CorrelationVector`*"]
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_System_CorrelationVector`*"]
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_CorrelationVector`*"]
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32;
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32;
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: *const ::windows_sys::core::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32;
}
