#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_UpdateAssessment`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWaaSAssessor(pub ::windows::runtime::IUnknown);
impl IWaaSAssessor {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_UpdateAssessment`, `Win32_Foundation`*"]
    pub unsafe fn GetOSUpdateAssessment(&self) -> ::windows::runtime::Result<OSUpdateAssessment> {
        let mut result__: <OSUpdateAssessment as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OSUpdateAssessment>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWaaSAssessor {
    type Vtable = IWaaSAssessor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(591903727, 6715, 17828, [144, 45, 62, 9, 194, 105, 180, 94]);
}
impl ::std::convert::From<IWaaSAssessor> for ::windows::runtime::IUnknown {
    fn from(value: IWaaSAssessor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWaaSAssessor> for ::windows::runtime::IUnknown {
    fn from(value: &IWaaSAssessor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWaaSAssessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWaaSAssessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaaSAssessor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result: *mut OSUpdateAssessment) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_UpdateAssessment`, `Win32_Foundation`*"]
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
impl OSUpdateAssessment {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OSUpdateAssessment {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OSUpdateAssessment {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OSUpdateAssessment")
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
impl ::std::cmp::PartialEq for OSUpdateAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.isEndOfSupport == other.isEndOfSupport
            && self.assessmentForCurrent == other.assessmentForCurrent
            && self.assessmentForUpToDate == other.assessmentForUpToDate
            && self.securityStatus == other.securityStatus
            && self.assessmentTime == other.assessmentTime
            && self.releaseInfoTime == other.releaseInfoTime
            && self.currentOSBuild == other.currentOSBuild
            && self.currentOSReleaseTime == other.currentOSReleaseTime
            && self.upToDateOSBuild == other.upToDateOSBuild
            && self.upToDateOSReleaseTime == other.upToDateOSReleaseTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OSUpdateAssessment {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OSUpdateAssessment {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_UpdateAssessment`*"]
pub struct UpdateAssessment {
    pub status: UpdateAssessmentStatus,
    pub impact: UpdateImpactLevel,
    pub daysOutOfDate: u32,
}
impl UpdateAssessment {}
impl ::std::default::Default for UpdateAssessment {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for UpdateAssessment {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UpdateAssessment").field("status", &self.status).field("impact", &self.impact).field("daysOutOfDate", &self.daysOutOfDate).finish()
    }
}
impl ::std::cmp::PartialEq for UpdateAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.impact == other.impact && self.daysOutOfDate == other.daysOutOfDate
    }
}
impl ::std::cmp::Eq for UpdateAssessment {}
unsafe impl ::windows::runtime::Abi for UpdateAssessment {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_UpdateAssessment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for UpdateAssessmentStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UpdateAssessmentStatus {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_UpdateAssessment`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UpdateImpactLevel(pub i32);
pub const UpdateImpactLevel_None: UpdateImpactLevel = UpdateImpactLevel(0i32);
pub const UpdateImpactLevel_Low: UpdateImpactLevel = UpdateImpactLevel(1i32);
pub const UpdateImpactLevel_Medium: UpdateImpactLevel = UpdateImpactLevel(2i32);
pub const UpdateImpactLevel_High: UpdateImpactLevel = UpdateImpactLevel(3i32);
impl ::std::convert::From<i32> for UpdateImpactLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UpdateImpactLevel {
    type Abi = Self;
}
pub const WaaSAssessor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(160364657, 64159, 18095, [137, 88, 192, 131, 81, 93, 124, 156]);
