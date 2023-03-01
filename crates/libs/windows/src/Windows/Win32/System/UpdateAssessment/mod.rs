#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
#[repr(transparent)]
pub struct IWaaSAssessor(::windows::core::IUnknown);
impl IWaaSAssessor {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOSUpdateAssessment(&self) -> ::windows::core::Result<OSUpdateAssessment> {
        let mut result__ = ::windows::core::zeroed::<OSUpdateAssessment>();
        (::windows::core::Interface::vtable(self).GetOSUpdateAssessment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWaaSAssessor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWaaSAssessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWaaSAssessor {}
impl ::core::fmt::Debug for IWaaSAssessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWaaSAssessor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWaaSAssessor {
    type Vtable = IWaaSAssessor_Vtbl;
}
impl ::core::clone::Clone for IWaaSAssessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWaaSAssessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2347bbef_1a3b_45a4_902d_3e09c269b45e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaaSAssessor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOSUpdateAssessment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOSUpdateAssessment: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const WaaSAssessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x098ef871_fa9f_46af_8958_c083515d7c9c);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UpdateAssessmentStatus(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_Latest: UpdateAssessmentStatus = UpdateAssessmentStatus(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestSoftRestriction: UpdateAssessmentStatus = UpdateAssessmentStatus(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestHardRestriction: UpdateAssessmentStatus = UpdateAssessmentStatus(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestEndOfSupport: UpdateAssessmentStatus = UpdateAssessmentStatus(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestServicingTrain: UpdateAssessmentStatus = UpdateAssessmentStatus(4i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestDeferredFeature: UpdateAssessmentStatus = UpdateAssessmentStatus(5i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestDeferredQuality: UpdateAssessmentStatus = UpdateAssessmentStatus(6i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestPausedFeature: UpdateAssessmentStatus = UpdateAssessmentStatus(7i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestPausedQuality: UpdateAssessmentStatus = UpdateAssessmentStatus(8i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestManaged: UpdateAssessmentStatus = UpdateAssessmentStatus(9i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestUnknown: UpdateAssessmentStatus = UpdateAssessmentStatus(10i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateAssessmentStatus_NotLatestTargetedVersion: UpdateAssessmentStatus = UpdateAssessmentStatus(11i32);
impl ::core::marker::Copy for UpdateAssessmentStatus {}
impl ::core::clone::Clone for UpdateAssessmentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateAssessmentStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for UpdateAssessmentStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for UpdateAssessmentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateAssessmentStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UpdateImpactLevel(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateImpactLevel_None: UpdateImpactLevel = UpdateImpactLevel(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateImpactLevel_Low: UpdateImpactLevel = UpdateImpactLevel(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateImpactLevel_Medium: UpdateImpactLevel = UpdateImpactLevel(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
pub const UpdateImpactLevel_High: UpdateImpactLevel = UpdateImpactLevel(3i32);
impl ::core::marker::Copy for UpdateImpactLevel {}
impl ::core::clone::Clone for UpdateImpactLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateImpactLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for UpdateImpactLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for UpdateImpactLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateImpactLevel").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OSUpdateAssessment {
    pub isEndOfSupport: super::super::Foundation::BOOL,
    pub assessmentForCurrent: UpdateAssessment,
    pub assessmentForUpToDate: UpdateAssessment,
    pub securityStatus: UpdateAssessmentStatus,
    pub assessmentTime: super::super::Foundation::FILETIME,
    pub releaseInfoTime: super::super::Foundation::FILETIME,
    pub currentOSBuild: ::windows::core::PWSTR,
    pub currentOSReleaseTime: super::super::Foundation::FILETIME,
    pub upToDateOSBuild: ::windows::core::PWSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OSUpdateAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSUpdateAssessment")
            .field("isEndOfSupport", &self.isEndOfSupport)
            .field("assessmentForCurrent", &self.assessmentForCurrent)
            .field("assessmentForUpToDate", &self.assessmentForUpToDate)
            .field("securityStatus", &self.securityStatus)
            .field("assessmentTime", &self.assessmentTime)
            .field("releaseInfoTime", &self.releaseInfoTime)
            .field("currentOSBuild", &self.currentOSBuild)
            .field("currentOSReleaseTime", &self.currentOSReleaseTime)
            .field("upToDateOSBuild", &self.upToDateOSBuild)
            .field("upToDateOSReleaseTime", &self.upToDateOSReleaseTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OSUpdateAssessment {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OSUpdateAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.isEndOfSupport == other.isEndOfSupport && self.assessmentForCurrent == other.assessmentForCurrent && self.assessmentForUpToDate == other.assessmentForUpToDate && self.securityStatus == other.securityStatus && self.assessmentTime == other.assessmentTime && self.releaseInfoTime == other.releaseInfoTime && self.currentOSBuild == other.currentOSBuild && self.currentOSReleaseTime == other.currentOSReleaseTime && self.upToDateOSBuild == other.upToDateOSBuild && self.upToDateOSReleaseTime == other.upToDateOSReleaseTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OSUpdateAssessment {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OSUpdateAssessment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`*"]
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
impl ::core::fmt::Debug for UpdateAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UpdateAssessment").field("status", &self.status).field("impact", &self.impact).field("daysOutOfDate", &self.daysOutOfDate).finish()
    }
}
impl ::windows::core::TypeKind for UpdateAssessment {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for UpdateAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.impact == other.impact && self.daysOutOfDate == other.daysOutOfDate
    }
}
impl ::core::cmp::Eq for UpdateAssessment {}
impl ::core::default::Default for UpdateAssessment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
