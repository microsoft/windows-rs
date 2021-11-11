#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlExtendCorrelationVector();
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIncrementCorrelationVector();
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlInitializeCorrelationVector();
    #[doc = "*Required features: `Win32_System_CorrelationVector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlValidateCorrelationVector();
}
