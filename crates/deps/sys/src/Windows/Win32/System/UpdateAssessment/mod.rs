#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWaaSAssessor(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub struct OSUpdateAssessment(i32);
pub struct UpdateAssessment(i32);
pub struct UpdateAssessmentStatus(i32);
pub struct UpdateImpactLevel(i32);
pub struct WaaSAssessor(i32);
