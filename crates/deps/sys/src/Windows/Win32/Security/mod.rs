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
    fn AccessCheck();
    fn AccessCheckAndAuditAlarmA();
    fn AccessCheckAndAuditAlarmW();
    fn AccessCheckByType();
    fn AccessCheckByTypeAndAuditAlarmA();
    fn AccessCheckByTypeAndAuditAlarmW();
    fn AccessCheckByTypeResultList();
    fn AccessCheckByTypeResultListAndAuditAlarmA();
    fn AccessCheckByTypeResultListAndAuditAlarmByHandleA();
    fn AccessCheckByTypeResultListAndAuditAlarmByHandleW();
    fn AccessCheckByTypeResultListAndAuditAlarmW();
    fn AddAccessAllowedAce();
    fn AddAccessAllowedAceEx();
    fn AddAccessAllowedObjectAce();
    fn AddAccessDeniedAce();
    fn AddAccessDeniedAceEx();
    fn AddAccessDeniedObjectAce();
    fn AddAce();
    fn AddAuditAccessAce();
    fn AddAuditAccessAceEx();
    fn AddAuditAccessObjectAce();
    fn AddConditionalAce();
    fn AddMandatoryAce();
    fn AddResourceAttributeAce();
    fn AddScopedPolicyIDAce();
    fn AdjustTokenGroups();
    fn AdjustTokenPrivileges();
    fn AllocateAndInitializeSid();
    fn AllocateLocallyUniqueId();
    fn AreAllAccessesGranted();
    fn AreAnyAccessesGranted();
    fn CheckTokenCapability();
    fn CheckTokenMembership();
    fn CheckTokenMembershipEx();
    fn ConvertToAutoInheritPrivateObjectSecurity();
    fn CopySid();
    fn CreatePrivateObjectSecurity();
    fn CreatePrivateObjectSecurityEx();
    fn CreatePrivateObjectSecurityWithMultipleInheritance();
    fn CreateRestrictedToken();
    fn CreateWellKnownSid();
    fn DeleteAce();
    fn DeriveCapabilitySidsFromName();
    fn DestroyPrivateObjectSecurity();
    fn DuplicateToken();
    fn DuplicateTokenEx();
    fn EqualDomainSid();
    fn EqualPrefixSid();
    fn EqualSid();
    fn FindFirstFreeAce();
    fn FreeSid();
    fn GetAce();
    fn GetAclInformation();
    fn GetAppContainerAce();
    fn GetCachedSigningLevel();
    fn GetFileSecurityA();
    fn GetFileSecurityW();
    fn GetKernelObjectSecurity();
    fn GetLengthSid();
    fn GetPrivateObjectSecurity();
    fn GetSecurityDescriptorControl();
    fn GetSecurityDescriptorDacl();
    fn GetSecurityDescriptorGroup();
    fn GetSecurityDescriptorLength();
    fn GetSecurityDescriptorOwner();
    fn GetSecurityDescriptorRMControl();
    fn GetSecurityDescriptorSacl();
    fn GetSidIdentifierAuthority();
    fn GetSidLengthRequired();
    fn GetSidSubAuthority();
    fn GetSidSubAuthorityCount();
    fn GetTokenInformation();
    fn GetUserObjectSecurity();
    fn GetWindowsAccountDomainSid();
    fn ImpersonateAnonymousToken();
    fn ImpersonateLoggedOnUser();
    fn ImpersonateSelf();
    fn InitializeAcl();
    fn InitializeSecurityDescriptor();
    fn InitializeSid();
    fn IsTokenRestricted();
    fn IsValidAcl();
    fn IsValidSecurityDescriptor();
    fn IsValidSid();
    fn IsWellKnownSid();
    fn LogonUserA();
    fn LogonUserExA();
    fn LogonUserExW();
    fn LogonUserW();
    fn LookupAccountNameA();
    fn LookupAccountNameW();
    fn LookupAccountSidA();
    fn LookupAccountSidW();
    fn LookupPrivilegeDisplayNameA();
    fn LookupPrivilegeDisplayNameW();
    fn LookupPrivilegeNameA();
    fn LookupPrivilegeNameW();
    fn LookupPrivilegeValueA();
    fn LookupPrivilegeValueW();
    fn MakeAbsoluteSD();
    fn MakeSelfRelativeSD();
    fn MapGenericMask();
    fn ObjectCloseAuditAlarmA();
    fn ObjectCloseAuditAlarmW();
    fn ObjectDeleteAuditAlarmA();
    fn ObjectDeleteAuditAlarmW();
    fn ObjectOpenAuditAlarmA();
    fn ObjectOpenAuditAlarmW();
    fn ObjectPrivilegeAuditAlarmA();
    fn ObjectPrivilegeAuditAlarmW();
    fn PrivilegeCheck();
    fn PrivilegedServiceAuditAlarmA();
    fn PrivilegedServiceAuditAlarmW();
    fn QuerySecurityAccessMask();
    fn RevertToSelf();
    fn RtlConvertSidToUnicodeString();
    fn RtlNormalizeSecurityDescriptor();
    fn SetAclInformation();
    fn SetCachedSigningLevel();
    fn SetFileSecurityA();
    fn SetFileSecurityW();
    fn SetKernelObjectSecurity();
    fn SetPrivateObjectSecurity();
    fn SetPrivateObjectSecurityEx();
    fn SetSecurityAccessMask();
    fn SetSecurityDescriptorControl();
    fn SetSecurityDescriptorDacl();
    fn SetSecurityDescriptorGroup();
    fn SetSecurityDescriptorOwner();
    fn SetSecurityDescriptorRMControl();
    fn SetSecurityDescriptorSacl();
    fn SetTokenInformation();
    fn SetUserObjectSecurity();
}
