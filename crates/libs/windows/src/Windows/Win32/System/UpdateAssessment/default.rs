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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OSUpdateAssessment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::default::Default for UpdateAssessment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UpdateAssessment {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.impact == other.impact && self.daysOutOfDate == other.daysOutOfDate
    }
}
impl ::core::cmp::Eq for UpdateAssessment {}
impl ::core::fmt::Debug for UpdateAssessment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UpdateAssessment").field("status", &self.status).field("impact", &self.impact).field("daysOutOfDate", &self.daysOutOfDate).finish()
    }
}
impl ::core::default::Default for UpdateAssessmentStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UpdateAssessmentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateAssessmentStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for UpdateImpactLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UpdateImpactLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateImpactLevel").field(&self.0).finish()
    }
}
