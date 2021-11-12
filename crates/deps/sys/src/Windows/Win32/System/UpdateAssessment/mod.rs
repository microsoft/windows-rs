#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWaaSAssessor(pub *mut ::core::ffi::c_void);
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OSUpdateAssessment {
    pub isEndOfSupport: super::super::Foundation::BOOL,
    pub assessmentForCurrent: UpdateAssessment,
    pub assessmentForUpToDate: UpdateAssessment,
    pub securityStatus: UpdateAssessmentStatus,
    pub assessmentTime: super::super::Foundation::FILETIME,
    pub releaseInfoTime: super::super::Foundation::FILETIME,
    pub currentOSBuild: super::super::Foundation::PWSTR,
    pub currentOSReleaseTime: super::super::Foundation::FILETIME,
    pub upToDateOSBuild: super::super::Foundation::PWSTR,
    pub upToDateOSReleaseTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OSUpdateAssessment {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OSUpdateAssessment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct UpdateAssessment {
    pub status: UpdateAssessmentStatus,
    pub impact: UpdateImpactLevel,
    pub daysOutOfDate: u32,
}
impl ::core::marker::Copy for UpdateAssessment {}
impl ::core::clone::Clone for UpdateAssessment {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for UpdateAssessmentStatus {}
impl ::core::clone::Clone for UpdateAssessmentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UpdateImpactLevel(pub i32);
pub const UpdateImpactLevel_None: UpdateImpactLevel = UpdateImpactLevel(0i32);
pub const UpdateImpactLevel_Low: UpdateImpactLevel = UpdateImpactLevel(1i32);
pub const UpdateImpactLevel_Medium: UpdateImpactLevel = UpdateImpactLevel(2i32);
pub const UpdateImpactLevel_High: UpdateImpactLevel = UpdateImpactLevel(3i32);
impl ::core::marker::Copy for UpdateImpactLevel {}
impl ::core::clone::Clone for UpdateImpactLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WaaSAssessor: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 160364657, data2: 64159, data3: 18095, data4: [137, 88, 192, 131, 81, 93, 124, 156] };
