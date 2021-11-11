#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPersonalTrustDBDialog();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPersonalTrustDBDialogEx();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperCertCheckValidSignature();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WTHelperCertIsSelfSigned();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WTHelperGetProvCertFromChain();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperGetProvPrivateDataFromChain();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperGetProvSignerFromChain();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperProvDataFromStateData();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinVerifyTrust();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WinVerifyTrustEx();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustAddActionID();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustAddDefaultForUsage();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustGetDefaultForUsage();
    #[doc = "*Required features: `Win32_Security_WinTrust`*"]
    pub fn WintrustGetRegPolicyFlags();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WintrustLoadFunctionPointers();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustRemoveActionID();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustSetDefaultIncludePEPageHashes();
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustSetRegPolicyFlags();
}
