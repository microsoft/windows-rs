#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_System_ApplicationVerifier`*"]
pub const AVRF_MAX_TRACES: u32 = 32u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ApplicationVerifier`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifierEnumerateResource(process: super::super::Foundation::HANDLE, flags: VERIFIER_ENUM_RESOURCE_FLAGS, resourcetype: eAvrfResourceTypes, resourcecallback: AVRF_RESOURCE_ENUMERATE_CALLBACK, enumerationcontext: *mut ::core::ffi::c_void) -> u32;
}
