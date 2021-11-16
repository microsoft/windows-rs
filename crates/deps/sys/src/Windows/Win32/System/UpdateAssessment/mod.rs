#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWaaSAssessor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWaaSAssessor {}
impl ::core::clone::Clone for IWaaSAssessor {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub const UpdateAssessmentStatus_Latest: i32 = 0i32;
pub const UpdateAssessmentStatus_NotLatestSoftRestriction: i32 = 1i32;
pub const UpdateAssessmentStatus_NotLatestHardRestriction: i32 = 2i32;
pub const UpdateAssessmentStatus_NotLatestEndOfSupport: i32 = 3i32;
pub const UpdateAssessmentStatus_NotLatestServicingTrain: i32 = 4i32;
pub const UpdateAssessmentStatus_NotLatestDeferredFeature: i32 = 5i32;
pub const UpdateAssessmentStatus_NotLatestDeferredQuality: i32 = 6i32;
pub const UpdateAssessmentStatus_NotLatestPausedFeature: i32 = 7i32;
pub const UpdateAssessmentStatus_NotLatestPausedQuality: i32 = 8i32;
pub const UpdateAssessmentStatus_NotLatestManaged: i32 = 9i32;
pub const UpdateAssessmentStatus_NotLatestUnknown: i32 = 10i32;
pub const UpdateAssessmentStatus_NotLatestTargetedVersion: i32 = 11i32;
pub const UpdateImpactLevel_None: i32 = 0i32;
pub const UpdateImpactLevel_Low: i32 = 1i32;
pub const UpdateImpactLevel_Medium: i32 = 2i32;
pub const UpdateImpactLevel_High: i32 = 3i32;
pub const WaaSAssessor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 160364657, data2: 64159, data3: 18095, data4: [137, 88, 192, 131, 81, 93, 124, 156] };
