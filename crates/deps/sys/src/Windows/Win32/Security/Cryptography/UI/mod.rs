#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSelectionGetSerializedBlob(pcsi: *const CERT_SELECTUI_INPUT, ppoutbuffer: *mut *mut ::core::ffi::c_void, puloutbuffersize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIDlgCertMgr(pcryptuicertmgr: *const CRYPTUI_CERT_MGR_STRUCT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIDlgSelectCertificateFromStore(hcertstore: *const ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, pwsztitle: super::super::super::Foundation::PWSTR, pwszdisplaystring: super::super::super::Foundation::PWSTR, dwdontusecolumn: u32, dwflags: u32, pvreserved: *const ::core::ffi::c_void) -> *mut super::CERT_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`, `Win32_Security_WinTrust`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CryptUIDlgViewCertificateA(pcertviewinfo: *const CRYPTUI_VIEWCERTIFICATE_STRUCTA, pfpropertieschanged: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`, `Win32_Security_WinTrust`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CryptUIDlgViewCertificateW(pcertviewinfo: *const CRYPTUI_VIEWCERTIFICATE_STRUCTW, pfpropertieschanged: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIDlgViewContext(dwcontexttype: u32, pvcontext: *const ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, pwsztitle: super::super::super::Foundation::PWSTR, dwflags: u32, pvreserved: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIWizDigitalSign(dwflags: u32, hwndparent: super::super::super::Foundation::HWND, pwszwizardtitle: super::super::super::Foundation::PWSTR, pdigitalsigninfo: *const CRYPTUI_WIZ_DIGITAL_SIGN_INFO, ppsigncontext: *mut *mut CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIWizExport(dwflags: CRYPTUI_WIZ_FLAGS, hwndparent: super::super::super::Foundation::HWND, pwszwizardtitle: super::super::super::Foundation::PWSTR, pexportinfo: *const CRYPTUI_WIZ_EXPORT_INFO, pvoid: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIWizFreeDigitalSignContext(psigncontext: *const CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUIWizImport(dwflags: CRYPTUI_WIZ_FLAGS, hwndparent: super::super::super::Foundation::HWND, pwszwizardtitle: super::super::super::Foundation::PWSTR, pimportsrc: *const CRYPTUI_WIZ_IMPORT_SRC_INFO, hdestcertstore: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
}
