#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSelectionGetSerializedBlob();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIDlgCertMgr();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIDlgSelectCertificateFromStore();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`, `Win32_Security_WinTrust`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CryptUIDlgViewCertificateA();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`, `Win32_Security_WinTrust`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CryptUIDlgViewCertificateW();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIDlgViewContext();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIWizDigitalSign();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIWizExport();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIWizFreeDigitalSignContext();
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIWizImport();
}
