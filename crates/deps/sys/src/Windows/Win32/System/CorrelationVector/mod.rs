#![allow(non_snake_case, non_camel_case_types)]
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
    pub fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32;
}
