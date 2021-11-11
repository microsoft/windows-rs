#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProtectFileToEnterpriseIdentity();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpCloseThreadNetworkContext();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpCreateThreadNetworkContext();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`*"]
    pub fn SrpDisablePermissiveModeFileEncryption();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`, `Win32_Storage_Packaging_Appx`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Appx"))]
    pub fn SrpDoesPolicyAllowAppExecution();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpEnablePermissiveModeFileEncryption();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpGetEnterpriseIds();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpGetEnterprisePolicy();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`*"]
    pub fn SrpHostingInitialize();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`*"]
    pub fn SrpHostingTerminate();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpIsTokenService();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SrpSetTokenEnterpriseId();
    #[doc = "*Required features: `Win32_Security_EnterpriseData`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnprotectFile();
}
