pub const UpdateAssessmentStatus_Latest: UpdateAssessmentStatus = 0i32;
pub const UpdateAssessmentStatus_NotLatestDeferredFeature: UpdateAssessmentStatus = 5i32;
pub const UpdateAssessmentStatus_NotLatestDeferredQuality: UpdateAssessmentStatus = 6i32;
pub const UpdateAssessmentStatus_NotLatestEndOfSupport: UpdateAssessmentStatus = 3i32;
pub const UpdateAssessmentStatus_NotLatestHardRestriction: UpdateAssessmentStatus = 2i32;
pub const UpdateAssessmentStatus_NotLatestManaged: UpdateAssessmentStatus = 9i32;
pub const UpdateAssessmentStatus_NotLatestPausedFeature: UpdateAssessmentStatus = 7i32;
pub const UpdateAssessmentStatus_NotLatestPausedQuality: UpdateAssessmentStatus = 8i32;
pub const UpdateAssessmentStatus_NotLatestServicingTrain: UpdateAssessmentStatus = 4i32;
pub const UpdateAssessmentStatus_NotLatestSoftRestriction: UpdateAssessmentStatus = 1i32;
pub const UpdateAssessmentStatus_NotLatestTargetedVersion: UpdateAssessmentStatus = 11i32;
pub const UpdateAssessmentStatus_NotLatestUnknown: UpdateAssessmentStatus = 10i32;
pub const UpdateImpactLevel_High: UpdateImpactLevel = 3i32;
pub const UpdateImpactLevel_Low: UpdateImpactLevel = 1i32;
pub const UpdateImpactLevel_Medium: UpdateImpactLevel = 2i32;
pub const UpdateImpactLevel_None: UpdateImpactLevel = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAssessmentStatus(pub i32);
impl windows_core::TypeKind for UpdateAssessmentStatus {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateImpactLevel(pub i32);
impl windows_core::TypeKind for UpdateImpactLevel {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSUpdateAssessment {
    pub isEndOfSupport: super::super::Foundation::BOOL,
    pub assessmentForCurrent: UpdateAssessment,
    pub assessmentForUpToDate: UpdateAssessment,
    pub securityStatus: UpdateAssessmentStatus,
    pub assessmentTime: super::super::Foundation::FILETIME,
    pub releaseInfoTime: super::super::Foundation::FILETIME,
    pub currentOSBuild: windows_core::PWSTR,
    pub currentOSReleaseTime: super::super::Foundation::FILETIME,
    pub upToDateOSBuild: windows_core::PWSTR,
    pub upToDateOSReleaseTime: super::super::Foundation::FILETIME,
}
impl Default for OSUpdateAssessment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OSUpdateAssessment {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateAssessment {
    pub status: UpdateAssessmentStatus,
    pub impact: UpdateImpactLevel,
    pub daysOutOfDate: u32,
}
impl Default for UpdateAssessment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UpdateAssessment {
    type TypeKind = windows_core::CopyType;
}
pub const WaaSAssessor: windows_core::GUID = windows_core::GUID::from_u128(0x098ef871_fa9f_46af_8958_c083515d7c9c);
