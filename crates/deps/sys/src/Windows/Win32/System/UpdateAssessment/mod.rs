#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWaaSAssessor(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OSUpdateAssessment(i32);
pub struct UpdateAssessment(i32);
pub struct UpdateAssessmentStatus(i32);
pub struct UpdateImpactLevel(i32);
pub struct WaaSAssessor(i32);
