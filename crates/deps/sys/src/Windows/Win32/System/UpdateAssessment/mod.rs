#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWaaSAssessor(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OSUpdateAssessment(i32);
#[repr(C)]
pub struct UpdateAssessment(i32);
#[repr(transparent)]
pub struct UpdateAssessmentStatus(pub i32);
pub const UpdateAssessmentStatus_Latest: UpdateAssessmentStatus = UpdateAssessmentStatus(0i32);
pub const UpdateAssessmentStatus_NotLatestSoftRestriction: UpdateAssessmentStatus = UpdateAssessmentStatus(1i32);
pub const UpdateAssessmentStatus_NotLatestHardRestriction: UpdateAssessmentStatus = UpdateAssessmentStatus(2i32);
pub const UpdateAssessmentStatus_NotLatestEndOfSupport: UpdateAssessmentStatus = UpdateAssessmentStatus(3i32);
pub const UpdateAssessmentStatus_NotLatestServicingTrain: UpdateAssessmentStatus = UpdateAssessmentStatus(4i32);
pub const UpdateAssessmentStatus_NotLatestDeferredFeature: UpdateAssessmentStatus = UpdateAssessmentStatus(5i32);
pub const UpdateAssessmentStatus_NotLatestDeferredQuality: UpdateAssessmentStatus = UpdateAssessmentStatus(6i32);
pub const UpdateAssessmentStatus_NotLatestPausedFeature: UpdateAssessmentStatus = UpdateAssessmentStatus(7i32);
pub const UpdateAssessmentStatus_NotLatestPausedQuality: UpdateAssessmentStatus = UpdateAssessmentStatus(8i32);
pub const UpdateAssessmentStatus_NotLatestManaged: UpdateAssessmentStatus = UpdateAssessmentStatus(9i32);
pub const UpdateAssessmentStatus_NotLatestUnknown: UpdateAssessmentStatus = UpdateAssessmentStatus(10i32);
pub const UpdateAssessmentStatus_NotLatestTargetedVersion: UpdateAssessmentStatus = UpdateAssessmentStatus(11i32);
#[repr(transparent)]
pub struct UpdateImpactLevel(pub i32);
pub const UpdateImpactLevel_None: UpdateImpactLevel = UpdateImpactLevel(0i32);
pub const UpdateImpactLevel_Low: UpdateImpactLevel = UpdateImpactLevel(1i32);
pub const UpdateImpactLevel_Medium: UpdateImpactLevel = UpdateImpactLevel(2i32);
pub const UpdateImpactLevel_High: UpdateImpactLevel = UpdateImpactLevel(3i32);
#[repr(C)]
pub struct WaaSAssessor(i32);
