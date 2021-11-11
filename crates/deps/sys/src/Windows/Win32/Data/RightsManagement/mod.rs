#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAcquireAdvisories();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAcquireIssuanceLicenseTemplate();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAcquireLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMActivate();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAddLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMAddRightWithUser();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAttest();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMCheckSecurity();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMClearAllRights();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMCloseEnvironmentHandle();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMCloseHandle();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMClosePubHandle();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMCloseQueryHandle();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMCloseSession();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMConstructCertificateChain();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateBoundLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateClientSession();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateEnablingBitsDecryptor();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateEnablingBitsEncryptor();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateEnablingPrincipal();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateIssuanceLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateLicenseStorageSession();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateRight();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateUser();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMDecode();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMDeconstructCertificateChain();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMDecrypt();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMDeleteLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMDuplicateEnvironmentHandle();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMDuplicateHandle();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMDuplicatePubHandle();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMDuplicateSession();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMEncode();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMEncrypt();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMEnumerateLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetApplicationSpecificData();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetBoundLicenseAttribute();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetBoundLicenseAttributeCount();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetBoundLicenseObject();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetBoundLicenseObjectCount();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetCertificateChainCount();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMGetClientVersion();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetEnvironmentInfo();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetInfo();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMGetIntervalTime();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetIssuanceLicenseInfo();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetIssuanceLicenseTemplate();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetMetaData();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetNameAndDescription();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetOwnerLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetProcAddress();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetRevocationPoint();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetRightExtendedInfo();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetRightInfo();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetSecurityProvider();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetServiceLocation();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetSignedIssuanceLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetSignedIssuanceLicenseEx();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetTime();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUnboundLicenseAttribute();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUnboundLicenseAttributeCount();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUnboundLicenseObject();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUnboundLicenseObjectCount();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUsagePolicy();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUserInfo();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMGetUserRights();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMGetUsers();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMInitEnvironment();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMIsActivated();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMIsWindowProtected();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMLoadLibrary();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMParseUnboundLicense();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMRegisterContent();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMRegisterProtectedWindow();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMRegisterRevocationList();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMRepair();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetApplicationSpecificData();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMSetGlobalOptions();
    #[doc = "*Required features: `Win32_Data_RightsManagement`*"]
    pub fn DRMSetIntervalTime();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetMetaData();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetNameAndDescription();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetRevocationPoint();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetUsagePolicy();
    #[doc = "*Required features: `Win32_Data_RightsManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMVerify();
}
