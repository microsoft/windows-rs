#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Security_Authentication_Identity_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn AcceptSecurityContext();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AcquireCredentialsHandleA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AcquireCredentialsHandleW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AddCredentialsA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn AddCredentialsW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSecurityPackageA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSecurityPackageW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn ApplyControlToken();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditComputeEffectivePolicyBySid();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditComputeEffectivePolicyByToken();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumerateCategories();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumeratePerUserPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditEnumerateSubCategories();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn AuditFree();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryGuidFromCategoryId();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryIdFromCategoryGuid();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryNameA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupCategoryNameW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupSubCategoryNameA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditLookupSubCategoryNameW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryGlobalSaclA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryGlobalSaclW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQueryPerUserPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQuerySecurity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditQuerySystemPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetGlobalSaclA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetGlobalSaclW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetPerUserPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetSecurity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AuditSetSystemPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeAccountPasswordA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeAccountPasswordW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn CompleteAuthToken();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn CredMarshalTargetInfo();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn CredUnmarshalTargetInfo();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn DecryptMessage();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn DeleteSecurityContext();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSecurityPackageA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSecurityPackageW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn EncryptMessage();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn EnumerateSecurityPackagesA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn EnumerateSecurityPackagesW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn ExportSecurityContext();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn FreeContextBuffer();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn FreeCredentialsHandle();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerObjectNameA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerObjectNameW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameExA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameExW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn ImpersonateSecurityContext();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn ImportSecurityContextA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn ImportSecurityContextW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn InitSecurityInterfaceA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn InitSecurityInterfaceW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn InitializeSecurityContextA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn InitializeSecurityContextW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaAddAccountRights();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaCallAuthenticationPackage();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaClose();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaConnectUntrusted();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaCreateTrustedDomainEx();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaDeleteTrustedDomain();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaDeregisterLogonProcess();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateAccountRights();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateAccountsWithUserRight();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateLogonSessions();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateTrustedDomains();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaEnumerateTrustedDomainsEx();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaFreeMemory();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaFreeReturnBuffer();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaGetAppliedCAPIDs();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaGetLogonSessionData();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LsaLogonUser();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LsaLookupAuthenticationPackage();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaLookupNames();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaLookupNames2();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaLookupSids();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaLookupSids2();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaNtStatusToWinError();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn LsaOpenPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaOpenTrustedDomainByName();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryCAPs();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryDomainInformationPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryForestTrustInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryInformationPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryTrustedDomainInfo();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaQueryTrustedDomainInfoByName();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LsaRegisterLogonProcess();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaRegisterPolicyChangeNotification();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaRemoveAccountRights();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaRetrievePrivateData();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetCAPs();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetDomainInformationPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetForestTrustInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetInformationPolicy();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetTrustedDomainInfoByName();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaSetTrustedDomainInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaStorePrivateData();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsaUnregisterPolicyChangeNotification();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn MakeSignature();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryContextAttributesA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryContextAttributesExA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryContextAttributesExW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryContextAttributesW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryCredentialsAttributesA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryCredentialsAttributesExA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryCredentialsAttributesExW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QueryCredentialsAttributesW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn QuerySecurityContextToken();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QuerySecurityPackageInfoA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QuerySecurityPackageInfoW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn RevertSecurityContext();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLAcquireGenuineTicket();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLActivateProduct();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLClose();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLConsumeRight();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLDepositOfflineConfirmationId();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLDepositOfflineConfirmationIdEx();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLFireEvent();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGenerateOfflineInstallationId();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGenerateOfflineInstallationIdEx();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetApplicationInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetGenuineInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetInstalledProductKeyIds();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetLicense();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetLicenseFileId();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetLicenseInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetLicensingStatusInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPKeyId();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPKeyInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPolicyInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetPolicyInformationDWORD();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetProductSkuInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetReferralInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLGetSLIDList();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetServerStatus();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetServiceInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetWindowsInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLGetWindowsInformationDWORD();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLInstallLicense();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLInstallProofOfPurchase();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLIsGenuineLocal();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLOpen();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLQueryLicenseValueFromApp();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLRegisterEvent();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLSetCurrentProductKey();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLSetGenuineInformation();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLUninstallLicense();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SLUninstallProofOfPurchase();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SLUnregisterEvent();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SaslAcceptSecurityContext();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaslEnumerateProfilesA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaslEnumerateProfilesW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SaslGetContextOption();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaslGetProfilePackageA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaslGetProfilePackageW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SaslIdentifyPackageA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SaslIdentifyPackageW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn SaslInitializeSecurityContextA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`, `Win32_Security_Credentials`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
    pub fn SaslInitializeSecurityContextW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SaslSetContextOption();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SetContextAttributesA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SetContextAttributesW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SetCredentialsAttributesA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn SetCredentialsAttributesW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SslCrackCertificate();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SslEmptyCacheA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SslEmptyCacheW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SslFreeCertificate();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SslGenerateRandomBits();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SslGetExtensions();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SslGetMaximumKeySize();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SslGetServerIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiCompareAuthIdentities();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiCopyAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiDecryptAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiDecryptAuthIdentityEx();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiEncodeAuthIdentityAsStrings();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiEncodeStringsAsAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiEncryptAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiEncryptAuthIdentityEx();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiExcludePackage();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiFreeAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiGetTargetHostName();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiIsAuthIdentityEncrypted();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiIsPromptingNeeded();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiLocalFree();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiMarshalAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiPrepareForCredRead();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiPrepareForCredWrite();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiPromptForCredentialsA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiPromptForCredentialsW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SspiUnmarshalAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiValidateAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn SspiZeroAuthIdentity();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemFunction036();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemFunction040();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemFunction041();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingDeleteAllBindings();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TokenBindingDeleteBinding();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TokenBindingGenerateBinding();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGenerateID();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TokenBindingGenerateIDForUri();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGenerateMessage();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGetHighestSupportedVersion();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGetKeyTypesClient();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingGetKeyTypesServer();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`*"]
    pub fn TokenBindingVerifyMessage();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateNameA();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateNameW();
    #[doc = "*Required features: `Win32_Security_Authentication_Identity`, `Win32_Security_Credentials`*"]
    #[cfg(feature = "Win32_Security_Credentials")]
    pub fn VerifySignature();
}
