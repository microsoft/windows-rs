#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPAddProvider();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPCreateIndirectData();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetCaps();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetSealedDigest();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetSignedDataMsg();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPLoad();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPPutSignedDataMsg();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRemoveProvider();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPRemoveSignedDataMsg();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRetrieveSubjectGuid();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRetrieveSubjectGuidForCatalogFile();
    #[doc = "*Required features: `Win32_Security_Cryptography_Sip`, `Win32_Foundation`, `Win32_Security_Cryptography_Catalog`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPVerifyIndirectData();
}
