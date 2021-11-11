#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPersonalTrustDBDialog(hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPersonalTrustDBDialogEx(hwndparent: super::super::Foundation::HWND, dwflags: u32, pvreserved: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperCertCheckValidSignature(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WTHelperCertIsSelfSigned(dwencoding: u32, pcert: *mut super::Cryptography::CERT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WTHelperGetProvCertFromChain(psgnr: *mut CRYPT_PROVIDER_SGNR, idxcert: u32) -> *mut CRYPT_PROVIDER_CERT;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperGetProvPrivateDataFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, pgproviderid: *mut ::windows::runtime::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperGetProvSignerFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: super::super::Foundation::BOOL, idxcountersigner: u32) -> *mut CRYPT_PROVIDER_SGNR;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperProvDataFromStateData(hstatedata: super::super::Foundation::HANDLE) -> *mut CRYPT_PROVIDER_DATA;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinVerifyTrust(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows::runtime::GUID, pwvtdata: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WinVerifyTrustEx(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows::runtime::GUID, pwintrustdata: *mut WINTRUST_DATA) -> i32;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustAddActionID(pgactionid: *const ::windows::runtime::GUID, fdwflags: u32, psprovinfo: *const CRYPT_REGISTER_ACTIONID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustAddDefaultForUsage(pszusageoid: super::super::Foundation::PSTR, psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustGetDefaultForUsage(dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION, pszusageoid: super::super::Foundation::PSTR, psusage: *mut CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_WinTrust`*"]
    pub fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS);
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WintrustLoadFunctionPointers(pgactionid: *mut ::windows::runtime::GUID, ppfns: *mut ::core::mem::ManuallyDrop<CRYPT_PROVIDER_FUNCTIONS>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustRemoveActionID(pgactionid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustSetRegPolicyFlags(dwpolicyflags: WINTRUST_POLICY_FLAGS) -> super::super::Foundation::BOOL;
}
