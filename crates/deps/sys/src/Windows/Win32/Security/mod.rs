#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Security_AppLocker")]
pub mod AppLocker;
#[cfg(feature = "Win32_Security_Authentication")]
pub mod Authentication;
#[cfg(feature = "Win32_Security_Authorization")]
pub mod Authorization;
#[cfg(feature = "Win32_Security_ConfigurationSnapin")]
pub mod ConfigurationSnapin;
#[cfg(feature = "Win32_Security_Credentials")]
pub mod Credentials;
#[cfg(feature = "Win32_Security_Cryptography")]
pub mod Cryptography;
#[cfg(feature = "Win32_Security_DiagnosticDataQuery")]
pub mod DiagnosticDataQuery;
#[cfg(feature = "Win32_Security_DirectoryServices")]
pub mod DirectoryServices;
#[cfg(feature = "Win32_Security_EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub mod ExtensibleAuthenticationProtocol;
#[cfg(feature = "Win32_Security_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_Security_LicenseProtection")]
pub mod LicenseProtection;
#[cfg(feature = "Win32_Security_NetworkAccessProtection")]
pub mod NetworkAccessProtection;
#[cfg(feature = "Win32_Security_Tpm")]
pub mod Tpm;
#[cfg(feature = "Win32_Security_WinTrust")]
pub mod WinTrust;
#[cfg(feature = "Win32_Security_WinWlx")]
pub mod WinWlx;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheck();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckAndAuditAlarmA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckAndAuditAlarmW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByType();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeAndAuditAlarmA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeAndAuditAlarmW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultList();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedAceEx();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedObjectAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedAceEx();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedObjectAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessAceEx();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessObjectAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConditionalAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMandatoryAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddResourceAttributeAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddScopedPolicyIDAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustTokenGroups();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustTokenPrivileges();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateAndInitializeSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateLocallyUniqueId();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreAllAccessesGranted();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreAnyAccessesGranted();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenCapability();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenMembership();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenMembershipEx();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertToAutoInheritPrivateObjectSecurity();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopySid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurity();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurityEx();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurityWithMultipleInheritance();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRestrictedToken();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateWellKnownSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveCapabilitySidsFromName();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPrivateObjectSecurity();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DuplicateToken();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DuplicateTokenEx();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualDomainSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualPrefixSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFreeAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAclInformation();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppContainerAce();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCachedSigningLevel();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSecurityA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSecurityW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKernelObjectSecurity();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLengthSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateObjectSecurity();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorControl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorDacl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorGroup();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorLength();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorOwner();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorRMControl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorSacl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidIdentifierAuthority();
    #[doc = "*Required features: `Win32_Security`*"]
    pub fn GetSidLengthRequired();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidSubAuthority();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidSubAuthorityCount();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTokenInformation();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectSecurity();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsAccountDomainSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateAnonymousToken();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateLoggedOnUser();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateSelf();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeAcl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSecurityDescriptor();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTokenRestricted();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidAcl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidSecurityDescriptor();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWellKnownSid();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserExA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserExW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountNameA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountNameW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountSidA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountSidW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeDisplayNameA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeDisplayNameW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeNameA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeNameW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeValueA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeValueW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeAbsoluteSD();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeSelfRelativeSD();
    #[doc = "*Required features: `Win32_Security`*"]
    pub fn MapGenericMask();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectCloseAuditAlarmA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectCloseAuditAlarmW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectDeleteAuditAlarmA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectDeleteAuditAlarmW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectOpenAuditAlarmA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectOpenAuditAlarmW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectPrivilegeAuditAlarmA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectPrivilegeAuditAlarmW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegeCheck();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegedServiceAuditAlarmA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegedServiceAuditAlarmW();
    #[doc = "*Required features: `Win32_Security`*"]
    pub fn QuerySecurityAccessMask();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevertToSelf();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlConvertSidToUnicodeString();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlNormalizeSecurityDescriptor();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAclInformation();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCachedSigningLevel();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileSecurityA();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileSecurityW();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetKernelObjectSecurity();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrivateObjectSecurity();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrivateObjectSecurityEx();
    #[doc = "*Required features: `Win32_Security`*"]
    pub fn SetSecurityAccessMask();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorControl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorDacl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorGroup();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorOwner();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorRMControl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorSacl();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTokenInformation();
    #[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectSecurity();
}
