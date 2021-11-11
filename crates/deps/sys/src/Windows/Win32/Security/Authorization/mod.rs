#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Security_Authorization_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzAccessCheck();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzAddSidsToContext();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzCachedAccessCheck();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzEnumerateSecurityEventSources();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzEvaluateSacl();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeAuditEvent();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeCentralAccessPolicyCache();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeContext();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeHandle();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzFreeResourceManager();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzGetInformationFromContext();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeCompoundContext();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromAuthzContext();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromSid();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeContextFromToken();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeObjectAccessAuditEvent();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeObjectAccessAuditEvent2();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeRemoteResourceManager();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeResourceManager();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInitializeResourceManagerEx();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzInstallSecurityEventSource();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifyClaims();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifySecurityAttributes();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzModifySids();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzOpenObjectAudit();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn AuthzRegisterCapChangeNotification();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzRegisterSecurityEventSource();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzReportSecurityEvent();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzReportSecurityEventFromParams();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzSetAppContainerInformation();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUninstallSecurityEventSource();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUnregisterCapChangeNotification();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuthzUnregisterSecurityEventSource();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildExplicitAccessWithNameA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildExplicitAccessWithNameW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildImpersonateExplicitAccessWithNameA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildImpersonateExplicitAccessWithNameW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildImpersonateTrusteeA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildImpersonateTrusteeW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildSecurityDescriptorA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildSecurityDescriptorW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithNameA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithNameW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndNameA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndNameW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndSidA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithObjectsAndSidW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithSidA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildTrusteeWithSidW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSecurityDescriptorToStringSecurityDescriptorA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSecurityDescriptorToStringSecurityDescriptorW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSidToStringSidA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertSidToStringSidW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSecurityDescriptorToSecurityDescriptorA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSecurityDescriptorToSecurityDescriptorW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSidToSidA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertStringSidToSidW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInheritedFromArray();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAuditedPermissionsFromAclA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAuditedPermissionsFromAclW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEffectiveRightsFromAclA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEffectiveRightsFromAclW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExplicitEntriesFromAclA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExplicitEntriesFromAclW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInheritanceSourceA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInheritanceSourceW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMultipleTrusteeA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMultipleTrusteeOperationA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMultipleTrusteeOperationW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMultipleTrusteeW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedSecurityInfoA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedSecurityInfoW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityInfo();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeFormA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeFormW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeNameA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeNameW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeTypeA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTrusteeTypeW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupSecurityDescriptorPartsA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupSecurityDescriptorPartsW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEntriesInAclA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEntriesInAclW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNamedSecurityInfoA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNamedSecurityInfoW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityInfo();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeResetNamedSecurityInfoA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeResetNamedSecurityInfoW();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeSetNamedSecurityInfoA();
    #[doc = "*Required features: `Win32_Security_Authorization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TreeSetNamedSecurityInfoW();
}
