#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub mod Catalog;
#[cfg(feature = "Win32_Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
pub mod Sip;
#[cfg(feature = "Win32_Security_Cryptography_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptAddContextFunction(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: ::windows_sys::core::PCWSTR, dwposition: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCloseAlgorithmProvider(halgorithm: BCRYPT_ALG_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptConfigureContext(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, pconfig: *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptConfigureContextFunction(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: ::windows_sys::core::PCWSTR, pconfig: *const CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateContext(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, pconfig: *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut *mut ::core::ffi::c_void, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateMultiHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut *mut ::core::ffi::c_void, nhashes: u32, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDecrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pbiv: *mut u8, cbiv: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeleteContext(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKey(hsharedsecret: *const ::core::ffi::c_void, pwszkdf: ::windows_sys::core::PCWSTR, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKeyCapi(hhash: *const ::core::ffi::c_void, htargetalg: BCRYPT_ALG_HANDLE, pbderivedkey: *mut u8, cbderivedkey: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKeyPBKDF2(hprf: BCRYPT_ALG_HANDLE, pbpassword: *const u8, cbpassword: u32, pbsalt: *const u8, cbsalt: u32, citerations: u64, pbderivedkey: *mut u8, cbderivedkey: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroyHash(hhash: *mut ::core::ffi::c_void) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroyKey(hkey: BCRYPT_KEY_HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroySecret(hsecret: *mut ::core::ffi::c_void) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDuplicateHash(hhash: *const ::core::ffi::c_void, phnewhash: *mut *mut ::core::ffi::c_void, pbhashobject: *mut u8, cbhashobject: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDuplicateKey(hkey: BCRYPT_KEY_HANDLE, phnewkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEncrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pbiv: *mut u8, cbiv: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumAlgorithms(dwalgoperations: BCRYPT_OPERATION, palgcount: *mut u32, ppalglist: *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContextFunctionProviders(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: ::windows_sys::core::PCWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContextFunctions(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, dwinterface: BCRYPT_INTERFACE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTIONS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContexts(dwtable: BCRYPT_TABLE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXTS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumProviders(pszalgid: ::windows_sys::core::PCWSTR, pimplcount: *mut u32, ppimpllist: *mut *mut BCRYPT_PROVIDER_NAME, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumRegisteredProviders(pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDERS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptExportKey(hkey: BCRYPT_KEY_HANDLE, hexportkey: BCRYPT_KEY_HANDLE, pszblobtype: ::windows_sys::core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptFinalizeKeyPair(hkey: BCRYPT_KEY_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptFinishHash(hhash: *mut ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn BCryptFreeBuffer(pvbuffer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenRandom(halgorithm: BCRYPT_ALG_HANDLE, pbbuffer: *mut u8, cbbuffer: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenerateKeyPair(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, dwlength: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenerateSymmetricKey(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGetFipsAlgorithmMode(pfenabled: *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGetProperty(hobject: *const ::core::ffi::c_void, pszproperty: ::windows_sys::core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptHash(halgorithm: BCRYPT_ALG_HANDLE, pbsecret: *const u8, cbsecret: u32, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptHashData(hhash: *mut ::core::ffi::c_void, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptImportKey(halgorithm: BCRYPT_ALG_HANDLE, himportkey: BCRYPT_KEY_HANDLE, pszblobtype: ::windows_sys::core::PCWSTR, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptImportKeyPair(halgorithm: BCRYPT_ALG_HANDLE, himportkey: BCRYPT_KEY_HANDLE, pszblobtype: ::windows_sys::core::PCWSTR, phkey: *mut BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptKeyDerivation(hkey: BCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptOpenAlgorithmProvider(phalgorithm: *mut BCRYPT_ALG_HANDLE, pszalgid: ::windows_sys::core::PCWSTR, pszimplementation: ::windows_sys::core::PCWSTR, dwflags: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptProcessMultiOperations(hobject: *mut ::core::ffi::c_void, operationtype: BCRYPT_MULTI_OPERATION_TYPE, poperations: *const ::core::ffi::c_void, cboperations: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextConfiguration(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextFunctionConfiguration(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: ::windows_sys::core::PCWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextFunctionProperty(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: ::windows_sys::core::PCWSTR, pszproperty: ::windows_sys::core::PCWSTR, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryProviderRegistration(pszprovider: ::windows_sys::core::PCWSTR, dwmode: BCRYPT_QUERY_PROVIDER_MODE, dwinterface: BCRYPT_INTERFACE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDER_REG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptRegisterConfigChangeNotify(phevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptRemoveContextFunction(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: ::windows_sys::core::PCWSTR) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptResolveProviders(pszcontext: ::windows_sys::core::PCWSTR, dwinterface: u32, pszfunction: ::windows_sys::core::PCWSTR, pszprovider: ::windows_sys::core::PCWSTR, dwmode: BCRYPT_QUERY_PROVIDER_MODE, dwflags: BCRYPT_RESOLVE_PROVIDERS_FLAGS, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDER_REFS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSecretAgreement(hprivkey: BCRYPT_KEY_HANDLE, hpubkey: BCRYPT_KEY_HANDLE, phagreedsecret: *mut *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSetContextFunctionProperty(dwtable: BCRYPT_TABLE, pszcontext: ::windows_sys::core::PCWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: ::windows_sys::core::PCWSTR, pszproperty: ::windows_sys::core::PCWSTR, cbvalue: u32, pbvalue: *const u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSetProperty(hobject: *mut ::core::ffi::c_void, pszproperty: ::windows_sys::core::PCWSTR, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSignHash(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: *const ::core::ffi::c_void, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptUnregisterConfigChangeNotify(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptVerifySignature(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: *const ::core::ffi::c_void, pbhash: *const u8, cbhash: u32, pbsignature: *const u8, cbsignature: u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCRLContextToStore(hcertstore: HCERTSTORE, pcrlcontext: *const CRL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCRLLinkToStore(hcertstore: HCERTSTORE, pcrlcontext: *const CRL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCTLContextToStore(hcertstore: HCERTSTORE, pctlcontext: *const CTL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCTLLinkToStore(hcertstore: HCERTSTORE, pctlcontext: *const CTL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCertificateContextToStore(hcertstore: HCERTSTORE, pcertcontext: *const CERT_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCertificateLinkToStore(hcertstore: HCERTSTORE, pcertcontext: *const CERT_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCRLToStore(hcertstore: HCERTSTORE, dwcertencodingtype: u32, pbcrlencoded: *const u8, cbcrlencoded: u32, dwadddisposition: u32, ppcrlcontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCTLToStore(hcertstore: HCERTSTORE, dwmsgandcertencodingtype: u32, pbctlencoded: *const u8, cbctlencoded: u32, dwadddisposition: u32, ppctlcontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToStore(hcertstore: HCERTSTORE, dwcertencodingtype: u32, pbcertencoded: *const u8, cbcertencoded: u32, dwadddisposition: u32, ppcertcontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToSystemStoreA(szcertstorename: ::windows_sys::core::PCSTR, pbcertencoded: *const u8, cbcertencoded: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToSystemStoreW(szcertstorename: ::windows_sys::core::PCWSTR, pbcertencoded: *const u8, cbcertencoded: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEnhancedKeyUsageIdentifier(pcertcontext: *const CERT_CONTEXT, pszusageidentifier: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertAddRefServerOcspResponse(hserverocspresponse: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertAddRefServerOcspResponseContext(pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddSerializedElementToStore(hcertstore: HCERTSTORE, pbelement: *const u8, cbelement: u32, dwadddisposition: u32, dwflags: u32, dwcontexttypeflags: u32, pdwcontexttype: *mut u32, ppvcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddStoreToCollection(hcollectionstore: HCERTSTORE, hsiblingstore: HCERTSTORE, dwupdateflags: u32, dwpriority: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertAlgIdToOID(dwalgid: u32) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertCloseServerOcspResponse(hserverocspresponse: *const ::core::ffi::c_void, dwflags: u32);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCloseStore(hcertstore: HCERTSTORE, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareCertificate(dwcertencodingtype: u32, pcertid1: *const CERT_INFO, pcertid2: *const CERT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareCertificateName(dwcertencodingtype: u32, pcertname1: *const CRYPTOAPI_BLOB, pcertname2: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareIntegerBlob(pint1: *const CRYPTOAPI_BLOB, pint2: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertComparePublicKeyInfo(dwcertencodingtype: u32, ppublickey1: *const CERT_PUBLIC_KEY_INFO, ppublickey2: *const CERT_PUBLIC_KEY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertControlStore(hcertstore: HCERTSTORE, dwflags: CERT_CONTROL_STORE_FLAGS, dwctrltype: u32, pvctrlpara: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCRLContext(dwcertencodingtype: u32, pbcrlencoded: *const u8, cbcrlencoded: u32) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCTLContext(dwmsgandcertencodingtype: u32, pbctlencoded: *const u8, cbctlencoded: u32) -> *mut CTL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCTLEntryFromCertificateContextProperties(pcertcontext: *const CERT_CONTEXT, coptattr: u32, rgoptattr: *const CRYPT_ATTRIBUTE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pctlentry: *mut CTL_ENTRY, pcbctlentry: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCertificateChainEngine(pconfig: *const CERT_CHAIN_ENGINE_CONFIG, phchainengine: *mut HCERTCHAINENGINE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCertificateContext(dwcertencodingtype: u32, pbcertencoded: *const u8, cbcertencoded: u32) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateContext(dwcontexttype: u32, dwencodingtype: u32, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pcreatepara: *const CERT_CREATE_CONTEXT_PARA) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateSelfSignCertificate(hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, psubjectissuerblob: *const CRYPTOAPI_BLOB, dwflags: CERT_CREATE_SELFSIGN_FLAGS, pkeyprovinfo: *const CRYPT_KEY_PROV_INFO, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pstarttime: *const super::super::Foundation::SYSTEMTIME, pendtime: *const super::super::Foundation::SYSTEMTIME, pextensions: *const CERT_EXTENSIONS) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCRLFromStore(pcrlcontext: *const CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCTLFromStore(pctlcontext: *const CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCertificateFromStore(pcertcontext: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCRLContext(pcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCTLContext(pctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCertificateChain(pchaincontext: *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCertificateContext(pcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertDuplicateStore(hcertstore: HCERTSTORE) -> HCERTSTORE;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCRLContextProperties(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCRLsInStore(hcertstore: HCERTSTORE, pprevcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCTLContextProperties(pctlcontext: *const CTL_CONTEXT, dwpropid: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCTLsInStore(hcertstore: HCERTSTORE, pprevctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCertificateContextProperties(pcertcontext: *const CERT_CONTEXT, dwpropid: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCertificatesInStore(hcertstore: HCERTSTORE, pprevcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenum: PFN_CERT_ENUM_PHYSICAL_STORE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSubjectInSortedCTL(pctlcontext: *const CTL_CONTEXT, ppvnextsubject: *mut *mut ::core::ffi::c_void, psubjectidentifier: *mut CRYPTOAPI_BLOB, pencodedattributes: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSystemStore(dwflags: u32, pvsystemstorelocationpara: *const ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void, pfnenum: PFN_CERT_ENUM_SYSTEM_STORE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSystemStoreLocation(dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenum: PFN_CERT_ENUM_SYSTEM_STORE_LOCATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertFindAttribute(pszobjid: ::windows_sys::core::PCSTR, cattr: u32, rgattr: *const CRYPT_ATTRIBUTE) -> *mut CRYPT_ATTRIBUTE;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCRLInStore(hcertstore: HCERTSTORE, dwcertencodingtype: u32, dwfindflags: u32, dwfindtype: u32, pvfindpara: *const ::core::ffi::c_void, pprevcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCTLInStore(hcertstore: HCERTSTORE, dwmsgandcertencodingtype: u32, dwfindflags: u32, dwfindtype: CERT_FIND_TYPE, pvfindpara: *const ::core::ffi::c_void, pprevctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCertificateInCRL(pcert: *const CERT_CONTEXT, pcrlcontext: *const CRL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppcrlentry: *mut *mut CRL_ENTRY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCertificateInStore(hcertstore: HCERTSTORE, dwcertencodingtype: u32, dwfindflags: u32, dwfindtype: CERT_FIND_FLAGS, pvfindpara: *const ::core::ffi::c_void, pprevcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindChainInStore(hcertstore: HCERTSTORE, dwcertencodingtype: u32, dwfindflags: CERT_FIND_CHAIN_IN_STORE_FLAGS, dwfindtype: u32, pvfindpara: *const ::core::ffi::c_void, pprevchaincontext: *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindExtension(pszobjid: ::windows_sys::core::PCSTR, cextensions: u32, rgextensions: *const CERT_EXTENSION) -> *mut CERT_EXTENSION;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertFindRDNAttr(pszobjid: ::windows_sys::core::PCSTR, pname: *const CERT_NAME_INFO) -> *mut CERT_RDN_ATTR;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindSubjectInCTL(dwencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> *mut CTL_ENTRY;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindSubjectInSortedCTL(psubjectidentifier: *const CRYPTOAPI_BLOB, pctlcontext: *const CTL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pencodedattributes: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCRLContext(pcrlcontext: *const CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCTLContext(pctlcontext: *const CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateChain(pchaincontext: *const CERT_CHAIN_CONTEXT);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertFreeCertificateChainEngine(hchainengine: HCERTCHAINENGINE);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateChainList(prgpselection: *const *const CERT_CHAIN_CONTEXT);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateContext(pcertcontext: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertFreeServerOcspResponseContext(pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCRLContextProperty(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCRLFromStore(hcertstore: HCERTSTORE, pissuercontext: *const CERT_CONTEXT, pprevcrlcontext: *const CRL_CONTEXT, pdwflags: *mut u32) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCTLContextProperty(pctlcontext: *const CTL_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCertificateChain(hchainengine: HCERTCHAINENGINE, pcertcontext: *const CERT_CONTEXT, ptime: *const super::super::Foundation::FILETIME, hadditionalstore: HCERTSTORE, pchainpara: *const CERT_CHAIN_PARA, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppchaincontext: *mut *mut CERT_CHAIN_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCertificateContextProperty(pcertcontext: *const CERT_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetEnhancedKeyUsage(pcertcontext: *const CERT_CONTEXT, dwflags: u32, pusage: *mut CTL_USAGE, pcbusage: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetIntendedKeyUsage(dwcertencodingtype: u32, pcertinfo: *const CERT_INFO, pbkeyusage: *mut u8, cbkeyusage: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetIssuerCertificateFromStore(hcertstore: HCERTSTORE, psubjectcontext: *const CERT_CONTEXT, pprevissuercontext: *const CERT_CONTEXT, pdwflags: *mut u32) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetNameStringA(pcertcontext: *const CERT_CONTEXT, dwtype: u32, dwflags: u32, pvtypepara: *const ::core::ffi::c_void, psznamestring: ::windows_sys::core::PSTR, cchnamestring: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetNameStringW(pcertcontext: *const CERT_CONTEXT, dwtype: u32, dwflags: u32, pvtypepara: *const ::core::ffi::c_void, psznamestring: ::windows_sys::core::PWSTR, cchnamestring: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertGetPublicKeyLength(dwcertencodingtype: u32, ppublickey: *const CERT_PUBLIC_KEY_INFO) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertGetServerOcspResponseContext(hserverocspresponse: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> *mut CERT_SERVER_OCSP_RESPONSE_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetStoreProperty(hcertstore: HCERTSTORE, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetSubjectCertificateFromStore(hcertstore: HCERTSTORE, dwcertencodingtype: u32, pcertid: *const CERT_INFO) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetValidUsages(ccerts: u32, rghcerts: *const *const CERT_CONTEXT, cnumoids: *mut i32, rghoids: *mut ::windows_sys::core::PSTR, pcboids: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsRDNAttrsInCertificateName(dwcertencodingtype: u32, dwflags: u32, pcertname: *const CRYPTOAPI_BLOB, prdn: *const CERT_RDN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsStrongHashToSign(pstrongsignpara: *const CERT_STRONG_SIGN_PARA, pwszcnghashalgid: ::windows_sys::core::PCWSTR, psigningcert: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsValidCRLForCertificate(pcert: *const CERT_CONTEXT, pcrl: *const CRL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsWeakHash(dwhashusetype: u32, pwszcnghashalgid: ::windows_sys::core::PCWSTR, dwchainflags: u32, psignerchaincontext: *const CERT_CHAIN_CONTEXT, ptimestamp: *const super::super::Foundation::FILETIME, pwszfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertNameToStrA(dwcertencodingtype: u32, pname: *const CRYPTOAPI_BLOB, dwstrtype: CERT_STRING_TYPE, psz: ::windows_sys::core::PSTR, csz: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertNameToStrW(dwcertencodingtype: u32, pname: *const CRYPTOAPI_BLOB, dwstrtype: CERT_STRING_TYPE, psz: ::windows_sys::core::PWSTR, csz: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertOIDToAlgId(pszobjid: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenServerOcspResponse(pchaincontext: *const CERT_CHAIN_CONTEXT, dwflags: u32, popenpara: *const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertOpenStore(lpszstoreprovider: ::windows_sys::core::PCSTR, dwencodingtype: CERT_QUERY_ENCODING_TYPE, hcryptprov: HCRYPTPROV_LEGACY, dwflags: CERT_OPEN_STORE_FLAGS, pvpara: *const ::core::ffi::c_void) -> HCERTSTORE;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertOpenSystemStoreA(hprov: HCRYPTPROV_LEGACY, szsubsystemprotocol: ::windows_sys::core::PCSTR) -> HCERTSTORE;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertOpenSystemStoreW(hprov: HCRYPTPROV_LEGACY, szsubsystemprotocol: ::windows_sys::core::PCWSTR) -> HCERTSTORE;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertRDNValueToStrA(dwvaluetype: u32, pvalue: *const CRYPTOAPI_BLOB, psz: ::windows_sys::core::PSTR, csz: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertRDNValueToStrW(dwvaluetype: u32, pvalue: *const CRYPTOAPI_BLOB, psz: ::windows_sys::core::PWSTR, csz: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRegisterPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pwszstorename: ::windows_sys::core::PCWSTR, pstoreinfo: *const CERT_PHYSICAL_STORE_INFO, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRegisterSystemStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pstoreinfo: *const CERT_SYSTEM_STORE_INFO, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRemoveEnhancedKeyUsageIdentifier(pcertcontext: *const CERT_CONTEXT, pszusageidentifier: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CertRemoveStoreFromCollection(hcollectionstore: HCERTSTORE, hsiblingstore: HCERTSTORE);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertResyncCertificateChainEngine(hchainengine: HCERTCHAINENGINE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRetrieveLogoOrBiometricInfo(pcertcontext: *const CERT_CONTEXT, lpszlogoorbiometrictype: ::windows_sys::core::PCSTR, dwretrievalflags: u32, dwtimeout: u32, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ppwszmimetype: *mut ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSaveStore(hcertstore: HCERTSTORE, dwencodingtype: CERT_QUERY_ENCODING_TYPE, dwsaveas: CERT_STORE_SAVE_AS, dwsaveto: CERT_STORE_SAVE_TO, pvsavetopara: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSelectCertificateChains(pselectioncontext: *const ::windows_sys::core::GUID, dwflags: u32, pchainparameters: *const CERT_SELECT_CHAIN_PARA, ccriteria: u32, rgpcriteria: *const CERT_SELECT_CRITERIA, hstore: HCERTSTORE, pcselection: *mut u32, pprgpselection: *mut *mut *mut CERT_CHAIN_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCRLStoreElement(pcrlcontext: *const CRL_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCTLStoreElement(pctlcontext: *const CTL_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCertificateStoreElement(pcertcontext: *const CERT_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCRLContextProperty(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCTLContextProperty(pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCertificateContextPropertiesFromCTLEntry(pcertcontext: *const CERT_CONTEXT, pctlentry: *const CTL_ENTRY, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCertificateContextProperty(pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetEnhancedKeyUsage(pcertcontext: *const CERT_CONTEXT, pusage: *const CTL_USAGE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetStoreProperty(hcertstore: HCERTSTORE, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertStrToNameA(dwcertencodingtype: u32, pszx500: ::windows_sys::core::PCSTR, dwstrtype: CERT_STRING_TYPE, pvreserved: *mut ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32, ppszerror: *mut ::windows_sys::core::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertStrToNameW(dwcertencodingtype: u32, pszx500: ::windows_sys::core::PCWSTR, dwstrtype: CERT_STRING_TYPE, pvreserved: *mut ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32, ppszerror: *mut ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertUnregisterPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pwszstorename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertUnregisterSystemStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCRLRevocation(dwcertencodingtype: u32, pcertid: *const CERT_INFO, ccrlinfo: u32, rgpcrlinfo: *const *const CRL_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCRLTimeValidity(ptimetoverify: *const super::super::Foundation::FILETIME, pcrlinfo: *const CRL_INFO) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCTLUsage(dwencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, psubjectusage: *const CTL_USAGE, dwflags: u32, pverifyusagepara: *const CTL_VERIFY_USAGE_PARA, pverifyusagestatus: *mut CTL_VERIFY_USAGE_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCertificateChainPolicy(pszpolicyoid: ::windows_sys::core::PCSTR, pchaincontext: *const CERT_CHAIN_CONTEXT, ppolicypara: *const CERT_CHAIN_POLICY_PARA, ppolicystatus: *mut CERT_CHAIN_POLICY_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyRevocation(dwencodingtype: u32, dwrevtype: u32, ccontext: u32, rgpvcontext: *const *const ::core::ffi::c_void, dwflags: u32, prevpara: *const CERT_REVOCATION_PARA, prevstatus: *mut CERT_REVOCATION_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifySubjectCertificateContext(psubject: *const CERT_CONTEXT, pissuer: *const CERT_CONTEXT, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyTimeValidity(ptimetoverify: *const super::super::Foundation::FILETIME, pcertinfo: *const CERT_INFO) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyValidityNesting(psubjectinfo: *const CERT_INFO, pissuerinfo: *const CERT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CloseCryptoHandle(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireCertificatePrivateKey(pcert: *const CERT_CONTEXT, dwflags: CRYPT_ACQUIRE_FLAGS, pvparameters: *const ::core::ffi::c_void, phcryptprovorncryptkey: *mut HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, pdwkeyspec: *mut CERT_KEY_SPEC, pfcallerfreeprovorncryptkey: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireContextA(phprov: *mut usize, szcontainer: ::windows_sys::core::PCSTR, szprovider: ::windows_sys::core::PCSTR, dwprovtype: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireContextW(phprov: *mut usize, szcontainer: ::windows_sys::core::PCWSTR, szprovider: ::windows_sys::core::PCWSTR, dwprovtype: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptBinaryToStringA(pbbinary: *const u8, cbbinary: u32, dwflags: CRYPT_STRING, pszstring: ::windows_sys::core::PSTR, pcchstring: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptBinaryToStringW(pbbinary: *const u8, cbbinary: u32, dwflags: CRYPT_STRING, pszstring: ::windows_sys::core::PWSTR, pcchstring: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCloseAsyncHandle(hasync: HCRYPTASYNC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptContextAddRef(hprov: usize, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateAsyncHandle(dwflags: u32, phasync: *mut HCRYPTASYNC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateHash(hprov: usize, algid: u32, hkey: usize, dwflags: u32, phhash: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateKeyIdentifierFromCSP(dwcertencodingtype: u32, pszpubkeyoid: ::windows_sys::core::PCSTR, ppubkeystruc: *const PUBLICKEYSTRUC, cbpubkeystruc: u32, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pbhash: *mut u8, pcbhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeMessage(dwmsgtypeflags: u32, pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbencodedblob: *const u8, cbencodedblob: u32, dwprevinnercontenttype: u32, pdwmsgtype: *mut u32, pdwinnercontenttype: *mut u32, pbdecoded: *mut u8, pcbdecoded: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeObject(dwcertencodingtype: u32, lpszstructtype: ::windows_sys::core::PCSTR, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pvstructinfo: *mut ::core::ffi::c_void, pcbstructinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeObjectEx(dwcertencodingtype: u32, lpszstructtype: ::windows_sys::core::PCSTR, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pdecodepara: *const CRYPT_DECODE_PARA, pvstructinfo: *mut ::core::ffi::c_void, pcbstructinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecrypt(hkey: usize, hhash: usize, r#final: super::super::Foundation::BOOL, dwflags: u32, pbdata: *mut u8, pdwdatalen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecryptAndVerifyMessageSignature(pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbencryptedblob: *const u8, cbencryptedblob: u32, pbdecrypted: *mut u8, pcbdecrypted: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecryptMessage(pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pbencryptedblob: *const u8, cbencryptedblob: u32, pbdecrypted: *mut u8, pcbdecrypted: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDeriveKey(hprov: usize, algid: u32, hbasedata: usize, dwflags: u32, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDestroyHash(hhash: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDestroyKey(hkey: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDuplicateHash(hhash: usize, pdwreserved: *mut u32, dwflags: u32, phhash: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDuplicateKey(hkey: usize, pdwreserved: *mut u32, dwflags: u32, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncodeObject(dwcertencodingtype: u32, lpszstructtype: ::windows_sys::core::PCSTR, pvstructinfo: *const ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncodeObjectEx(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, lpszstructtype: ::windows_sys::core::PCSTR, pvstructinfo: *const ::core::ffi::c_void, dwflags: CRYPT_ENCODE_OBJECT_FLAGS, pencodepara: *const CRYPT_ENCODE_PARA, pvencoded: *mut ::core::ffi::c_void, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncrypt(hkey: usize, hhash: usize, r#final: super::super::Foundation::BOOL, dwflags: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwbuflen: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncryptMessage(pencryptpara: *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert: u32, rgprecipientcert: *const *const CERT_CONTEXT, pbtobeencrypted: *const u8, cbtobeencrypted: u32, pbencryptedblob: *mut u8, pcbencryptedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumKeyIdentifierProperties(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: ::windows_sys::core::PCWSTR, pvreserved: *mut ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void, pfnenum: PFN_CRYPT_ENUM_KEYID_PROP) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumOIDFunction(dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, pszoid: ::windows_sys::core::PCSTR, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumoidfunc: PFN_CRYPT_ENUM_OID_FUNC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumOIDInfo(dwgroupid: u32, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumoidinfo: PFN_CRYPT_ENUM_OID_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProviderTypesA(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, sztypename: ::windows_sys::core::PSTR, pcbtypename: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProviderTypesW(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, sztypename: ::windows_sys::core::PWSTR, pcbtypename: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProvidersA(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, szprovname: ::windows_sys::core::PSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProvidersW(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, szprovname: ::windows_sys::core::PWSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportKey(hkey: usize, hexpkey: usize, dwblobtype: u32, dwflags: CRYPT_KEY_FLAGS, pbdata: *mut u8, pdwdatalen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPKCS8(hcryptprov: usize, dwkeyspec: u32, pszprivatekeyobjid: ::windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pbprivatekeyblob: *mut u8, pcbprivatekeyblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfo(hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: u32, dwcertencodingtype: u32, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfoEx(hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: u32, dwcertencodingtype: u32, pszpublickeyobjid: ::windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfoFromBCryptKeyHandle(hbcryptkey: BCRYPT_KEY_HANDLE, dwcertencodingtype: u32, pszpublickeyobjid: ::windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindCertificateKeyProvInfo(pcert: *const CERT_CONTEXT, dwflags: CRYPT_FIND_FLAGS, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptFindLocalizedName(pwszcryptname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptFindOIDInfo(dwkeytype: u32, pvkey: *const ::core::ffi::c_void, dwgroupid: u32) -> *mut CRYPT_OID_INFO;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFormatObject(dwcertencodingtype: u32, dwformattype: u32, dwformatstrtype: u32, pformatstruct: *const ::core::ffi::c_void, lpszstructtype: ::windows_sys::core::PCSTR, pbencoded: *const u8, cbencoded: u32, pbformat: *mut ::core::ffi::c_void, pcbformat: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFreeOIDFunctionAddress(hfuncaddr: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGenKey(hprov: usize, algid: u32, dwflags: CRYPT_KEY_FLAGS, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGenRandom(hprov: usize, dwlen: u32, pbbuffer: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetAsyncParam(hasync: HCRYPTASYNC, pszparamoid: ::windows_sys::core::PCSTR, ppvparam: *mut *mut ::core::ffi::c_void, ppfnfree: *mut PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultOIDDllList(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pwszdlllist: ::windows_sys::core::PWSTR, pcchdlllist: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultOIDFunctionAddress(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pwszdll: ::windows_sys::core::PCWSTR, dwflags: u32, ppvfuncaddr: *mut *mut ::core::ffi::c_void, phfuncaddr: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultProviderA(dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32, pszprovname: ::windows_sys::core::PSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultProviderW(dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32, pszprovname: ::windows_sys::core::PWSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetHashParam(hhash: usize, dwparam: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetKeyIdentifierProperty(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: ::windows_sys::core::PCWSTR, pvreserved: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetKeyParam(hkey: usize, dwparam: CRYPT_KEY_PARAM_ID, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptGetMessageCertificates(dwmsgandcertencodingtype: u32, hcryptprov: HCRYPTPROV_LEGACY, dwflags: u32, pbsignedblob: *const u8, cbsignedblob: u32) -> HCERTSTORE;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptGetMessageSignerCount(dwmsgencodingtype: u32, pbsignedblob: *const u8, cbsignedblob: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetOIDFunctionAddress(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pszoid: ::windows_sys::core::PCSTR, dwflags: u32, ppvfuncaddr: *mut *mut ::core::ffi::c_void, phfuncaddr: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetOIDFunctionValue(dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, pszoid: ::windows_sys::core::PCSTR, pwszvaluename: ::windows_sys::core::PCWSTR, pdwvaluetype: *mut u32, pbvaluedata: *mut u8, pcbvaluedata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetObjectUrl(pszurloid: ::windows_sys::core::PCSTR, pvpara: *const ::core::ffi::c_void, dwflags: CRYPT_GET_URL_FLAGS, purlarray: *mut CRYPT_URL_ARRAY, pcburlarray: *mut u32, purlinfo: *mut CRYPT_URL_INFO, pcburlinfo: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetProvParam(hprov: usize, dwparam: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetUserKey(hprov: usize, dwkeyspec: u32, phuserkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashCertificate(hcryptprov: HCRYPTPROV_LEGACY, algid: u32, dwflags: u32, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashCertificate2(pwszcnghashalgid: ::windows_sys::core::PCWSTR, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashData(hhash: usize, pbdata: *const u8, dwdatalen: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashMessage(phashpara: *const CRYPT_HASH_MESSAGE_PARA, fdetachedhash: super::super::Foundation::BOOL, ctobehashed: u32, rgpbtobehashed: *const *const u8, rgcbtobehashed: *const u32, pbhashedblob: *mut u8, pcbhashedblob: *mut u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashPublicKeyInfo(hcryptprov: HCRYPTPROV_LEGACY, algid: u32, dwflags: u32, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashSessionKey(hhash: usize, hkey: usize, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashToBeSigned(hcryptprov: HCRYPTPROV_LEGACY, dwcertencodingtype: u32, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportKey(hprov: usize, pbdata: *const u8, dwdatalen: u32, hpubkey: usize, dwflags: CRYPT_KEY_FLAGS, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPKCS8(sprivatekeyandparams: CRYPT_PKCS8_IMPORT_PARAMS, dwflags: CRYPT_KEY_FLAGS, phcryptprov: *mut usize, pvauxinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfo(hcryptprov: usize, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfoEx(hcryptprov: usize, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, aikeyalg: u32, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfoEx2(dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, dwflags: CRYPT_IMPORT_PUBLIC_KEY_FLAGS, pvauxinfo: *const ::core::ffi::c_void, phkey: *mut BCRYPT_KEY_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptInitOIDFunctionSet(pszfuncname: ::windows_sys::core::PCSTR, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallCancelRetrieval(pfncancel: PFN_CRYPT_CANCEL_RETRIEVAL, pvarg: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallDefaultContext(hcryptprov: usize, dwdefaulttype: CRYPT_DEFAULT_CONTEXT_TYPE, pvdefaultpara: *const ::core::ffi::c_void, dwflags: CRYPT_DEFAULT_CONTEXT_FLAGS, pvreserved: *mut ::core::ffi::c_void, phdefaultcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallOIDFunctionAddress(hmodule: super::super::Foundation::HINSTANCE, dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, cfuncentry: u32, rgfuncentry: *const CRYPT_OID_FUNC_ENTRY, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptMemAlloc(cbsize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptMemFree(pv: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptMemRealloc(pv: *const ::core::ffi::c_void, cbsize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptMsgCalculateEncodedLength(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: u32, pvmsgencodeinfo: *const ::core::ffi::c_void, pszinnercontentobjid: ::windows_sys::core::PCSTR, cbdata: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgClose(hcryptmsg: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgControl(hcryptmsg: *const ::core::ffi::c_void, dwflags: u32, dwctrltype: u32, pvctrlpara: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCountersign(hcryptmsg: *const ::core::ffi::c_void, dwindex: u32, ccountersigners: u32, rgcountersigners: *const CMSG_SIGNER_ENCODE_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCountersignEncoded(dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, ccountersigners: u32, rgcountersigners: *const CMSG_SIGNER_ENCODE_INFO, pbcountersignature: *mut u8, pcbcountersignature: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptMsgDuplicate(hcryptmsg: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgEncodeAndSignCTL(dwmsgencodingtype: u32, pctlinfo: *const CTL_INFO, psigninfo: *const CMSG_SIGNED_ENCODE_INFO, dwflags: u32, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgGetAndVerifySigner(hcryptmsg: *const ::core::ffi::c_void, csignerstore: u32, rghsignerstore: *const HCERTSTORE, dwflags: u32, ppsigner: *mut *mut CERT_CONTEXT, pdwsignerindex: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgGetParam(hcryptmsg: *const ::core::ffi::c_void, dwparamtype: u32, dwindex: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgOpenToDecode(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: u32, hcryptprov: HCRYPTPROV_LEGACY, precipientinfo: *mut CERT_INFO, pstreaminfo: *const CMSG_STREAM_INFO) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgOpenToEncode(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: CRYPT_MSG_TYPE, pvmsgencodeinfo: *const ::core::ffi::c_void, pszinnercontentobjid: ::windows_sys::core::PCSTR, pstreaminfo: *const CMSG_STREAM_INFO) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgSignCTL(dwmsgencodingtype: u32, pbctlcontent: *const u8, cbctlcontent: u32, psigninfo: *const CMSG_SIGNED_ENCODE_INFO, dwflags: u32, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgUpdate(hcryptmsg: *const ::core::ffi::c_void, pbdata: *const u8, cbdata: u32, ffinal: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgVerifyCountersignatureEncoded(hcryptprov: HCRYPTPROV_LEGACY, dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, pbsignerinfocountersignature: *const u8, cbsignerinfocountersignature: u32, pcicountersigner: *const CERT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgVerifyCountersignatureEncodedEx(hcryptprov: HCRYPTPROV_LEGACY, dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, pbsignerinfocountersignature: *const u8, cbsignerinfocountersignature: u32, dwsignertype: u32, pvsigner: *const ::core::ffi::c_void, dwflags: u32, pvextra: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptProtectData(pdatain: *const CRYPTOAPI_BLOB, szdatadescr: ::windows_sys::core::PCWSTR, poptionalentropy: *const CRYPTOAPI_BLOB, pvreserved: *mut ::core::ffi::c_void, ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT, dwflags: u32, pdataout: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptProtectMemory(pdatain: *mut ::core::ffi::c_void, cbdatain: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptQueryObject(dwobjecttype: CERT_QUERY_OBJECT_TYPE, pvobject: *const ::core::ffi::c_void, dwexpectedcontenttypeflags: CERT_QUERY_CONTENT_TYPE_FLAGS, dwexpectedformattypeflags: CERT_QUERY_FORMAT_TYPE_FLAGS, dwflags: u32, pdwmsgandcertencodingtype: *mut CERT_QUERY_ENCODING_TYPE, pdwcontenttype: *mut CERT_QUERY_CONTENT_TYPE, pdwformattype: *mut CERT_QUERY_FORMAT_TYPE, phcertstore: *mut HCERTSTORE, phmsg: *mut *mut ::core::ffi::c_void, ppvcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterDefaultOIDFunction(dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, dwindex: u32, pwszdll: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterOIDFunction(dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, pszoid: ::windows_sys::core::PCSTR, pwszdll: ::windows_sys::core::PCWSTR, pszoverridefuncname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterOIDInfo(pinfo: *const CRYPT_OID_INFO, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptReleaseContext(hprov: usize, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveObjectByUrlA(pszurl: ::windows_sys::core::PCSTR, pszobjectoid: ::windows_sys::core::PCSTR, dwretrievalflags: u32, dwtimeout: u32, ppvobject: *mut *mut ::core::ffi::c_void, hasyncretrieve: HCRYPTASYNC, pcredentials: *const CRYPT_CREDENTIALS, pvverify: *const ::core::ffi::c_void, pauxinfo: *mut CRYPT_RETRIEVE_AUX_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveObjectByUrlW(pszurl: ::windows_sys::core::PCWSTR, pszobjectoid: ::windows_sys::core::PCSTR, dwretrievalflags: u32, dwtimeout: u32, ppvobject: *mut *mut ::core::ffi::c_void, hasyncretrieve: HCRYPTASYNC, pcredentials: *const CRYPT_CREDENTIALS, pvverify: *const ::core::ffi::c_void, pauxinfo: *mut CRYPT_RETRIEVE_AUX_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveTimeStamp(wszurl: ::windows_sys::core::PCWSTR, dwretrievalflags: u32, dwtimeout: u32, pszhashid: ::windows_sys::core::PCSTR, ppara: *const CRYPT_TIMESTAMP_PARA, pbdata: *const u8, cbdata: u32, pptscontext: *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner: *mut *mut CERT_CONTEXT, phstore: *mut HCERTSTORE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetAsyncParam(hasync: HCRYPTASYNC, pszparamoid: ::windows_sys::core::PCSTR, pvparam: *const ::core::ffi::c_void, pfnfree: PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetHashParam(hhash: usize, dwparam: CRYPT_SET_HASH_PARAM, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetKeyIdentifierProperty(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: ::windows_sys::core::PCWSTR, pvreserved: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetKeyParam(hkey: usize, dwparam: CRYPT_KEY_PARAM_ID, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CryptSetOIDFunctionValue(dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, pszoid: ::windows_sys::core::PCSTR, pwszvaluename: ::windows_sys::core::PCWSTR, dwvaluetype: super::super::System::Registry::REG_VALUE_TYPE, pbvaluedata: *const u8, cbvaluedata: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProvParam(hprov: usize, dwparam: CRYPT_SET_PROV_PARAM_ID, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderA(pszprovname: ::windows_sys::core::PCSTR, dwprovtype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderExA(pszprovname: ::windows_sys::core::PCSTR, dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderExW(pszprovname: ::windows_sys::core::PCWSTR, dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderW(pszprovname: ::windows_sys::core::PCWSTR, dwprovtype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignAndEncodeCertificate(hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: CERT_KEY_SPEC, dwcertencodingtype: u32, lpszstructtype: ::windows_sys::core::PCSTR, pvstructinfo: *const ::core::ffi::c_void, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo: *const ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignAndEncryptMessage(psignpara: *const CRYPT_SIGN_MESSAGE_PARA, pencryptpara: *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert: u32, rgprecipientcert: *const *const CERT_CONTEXT, pbtobesignedandencrypted: *const u8, cbtobesignedandencrypted: u32, pbsignedandencryptedblob: *mut u8, pcbsignedandencryptedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignCertificate(hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: u32, dwcertencodingtype: u32, pbencodedtobesigned: *const u8, cbencodedtobesigned: u32, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo: *const ::core::ffi::c_void, pbsignature: *mut u8, pcbsignature: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignHashA(hhash: usize, dwkeyspec: u32, szdescription: ::windows_sys::core::PCSTR, dwflags: u32, pbsignature: *mut u8, pdwsiglen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignHashW(hhash: usize, dwkeyspec: u32, szdescription: ::windows_sys::core::PCWSTR, dwflags: u32, pbsignature: *mut u8, pdwsiglen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignMessage(psignpara: *const CRYPT_SIGN_MESSAGE_PARA, fdetachedsignature: super::super::Foundation::BOOL, ctobesigned: u32, rgpbtobesigned: *const *const u8, rgcbtobesigned: *const u32, pbsignedblob: *mut u8, pcbsignedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignMessageWithKey(psignpara: *const CRYPT_KEY_SIGN_MESSAGE_PARA, pbtobesigned: *const u8, cbtobesigned: u32, pbsignedblob: *mut u8, pcbsignedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptStringToBinaryA(pszstring: ::windows_sys::core::PCSTR, cchstring: u32, dwflags: CRYPT_STRING, pbbinary: *mut u8, pcbbinary: *mut u32, pdwskip: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptStringToBinaryW(pszstring: ::windows_sys::core::PCWSTR, cchstring: u32, dwflags: CRYPT_STRING, pbbinary: *mut u8, pcbbinary: *mut u32, pdwskip: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUninstallCancelRetrieval(dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUninstallDefaultContext(hdefaultcontext: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnprotectData(pdatain: *const CRYPTOAPI_BLOB, ppszdatadescr: *mut ::windows_sys::core::PWSTR, poptionalentropy: *const CRYPTOAPI_BLOB, pvreserved: *mut ::core::ffi::c_void, ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT, dwflags: u32, pdataout: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnprotectMemory(pdatain: *mut ::core::ffi::c_void, cbdatain: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterDefaultOIDFunction(dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, pwszdll: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterOIDFunction(dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, pszoid: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterOIDInfo(pinfo: *const CRYPT_OID_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUpdateProtectedState(poldsid: super::super::Foundation::PSID, pwszoldpassword: ::windows_sys::core::PCWSTR, dwflags: u32, pdwsuccesscount: *mut u32, pdwfailurecount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyCertificateSignature(hcryptprov: HCRYPTPROV_LEGACY, dwcertencodingtype: u32, pbencoded: *const u8, cbencoded: u32, ppublickey: *const CERT_PUBLIC_KEY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyCertificateSignatureEx(hcryptprov: HCRYPTPROV_LEGACY, dwcertencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, dwissuertype: u32, pvissuer: *const ::core::ffi::c_void, dwflags: CRYPT_VERIFY_CERT_FLAGS, pvextra: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyDetachedMessageHash(phashpara: *const CRYPT_HASH_MESSAGE_PARA, pbdetachedhashblob: *const u8, cbdetachedhashblob: u32, ctobehashed: u32, rgpbtobehashed: *const *const u8, rgcbtobehashed: *const u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyDetachedMessageSignature(pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbdetachedsignblob: *const u8, cbdetachedsignblob: u32, ctobesigned: u32, rgpbtobesigned: *const *const u8, rgcbtobesigned: *const u32, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageHash(phashpara: *const CRYPT_HASH_MESSAGE_PARA, pbhashedblob: *const u8, cbhashedblob: u32, pbtobehashed: *mut u8, pcbtobehashed: *mut u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageSignature(pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbsignedblob: *const u8, cbsignedblob: u32, pbdecoded: *mut u8, pcbdecoded: *mut u32, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageSignatureWithKey(pverifypara: *const CRYPT_KEY_VERIFY_MESSAGE_PARA, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pbsignedblob: *const u8, cbsignedblob: u32, pbdecoded: *mut u8, pcbdecoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifySignatureA(hhash: usize, pbsignature: *const u8, dwsiglen: u32, hpubkey: usize, szdescription: ::windows_sys::core::PCSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifySignatureW(hhash: usize, pbsignature: *const u8, dwsiglen: u32, hpubkey: usize, szdescription: ::windows_sys::core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyTimeStampSignature(pbtscontentinfo: *const u8, cbtscontentinfo: u32, pbdata: *const u8, cbdata: u32, hadditionalstore: HCERTSTORE, pptscontext: *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner: *mut *mut CERT_CONTEXT, phstore: *mut HCERTSTORE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlAddObject(hsignatureorobject: *const ::core::ffi::c_void, dwflags: u32, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, ppobject: *mut *mut CRYPT_XML_OBJECT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlClose(hcryptxml: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlCreateReference(hcryptxml: *const ::core::ffi::c_void, dwflags: u32, wszid: ::windows_sys::core::PCWSTR, wszuri: ::windows_sys::core::PCWSTR, wsztype: ::windows_sys::core::PCWSTR, pdigestmethod: *const CRYPT_XML_ALGORITHM, ctransform: u32, rgtransform: *const CRYPT_XML_ALGORITHM, phreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlDigestReference(hreference: *const ::core::ffi::c_void, dwflags: u32, pdataproviderin: *const CRYPT_XML_DATA_PROVIDER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlEncode(hcryptxml: *const ::core::ffi::c_void, dwcharset: CRYPT_XML_CHARSET, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pvcallbackstate: *mut ::core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlEnumAlgorithmInfo(dwgroupid: u32, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumalginfo: PFN_CRYPT_XML_ENUM_ALG_INFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlFindAlgorithmInfo(dwfindbytype: u32, pvfindby: *const ::core::ffi::c_void, dwgroupid: u32, dwflags: u32) -> *mut CRYPT_XML_ALGORITHM_INFO;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlGetAlgorithmInfo(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, dwflags: CRYPT_XML_FLAGS, ppalginfo: *mut *mut CRYPT_XML_ALGORITHM_INFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlGetDocContext(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_DOC_CTXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlGetReference(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_REFERENCE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlGetSignature(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_SIGNATURE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlGetStatus(hcryptxml: *const ::core::ffi::c_void, pstatus: *mut CRYPT_XML_STATUS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlGetTransforms(ppconfig: *mut *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlImportPublicKey(dwflags: CRYPT_XML_FLAGS, pkeyvalue: *const CRYPT_XML_KEY_VALUE, phkey: *mut BCRYPT_KEY_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlOpenToDecode(pconfig: *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags: CRYPT_XML_FLAGS, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, phcryptxml: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlOpenToEncode(pconfig: *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags: CRYPT_XML_FLAGS, wszid: ::windows_sys::core::PCWSTR, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, phsignature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlSetHMACSecret(hsignature: *const ::core::ffi::c_void, pbsecret: *const u8, cbsecret: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlSign(hsignature: *const ::core::ffi::c_void, hkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: CERT_KEY_SPEC, dwflags: CRYPT_XML_FLAGS, dwkeyinfospec: CRYPT_XML_KEYINFO_SPEC, pvkeyinfospec: *const ::core::ffi::c_void, psignaturemethod: *const CRYPT_XML_ALGORITHM, pcanonicalization: *const CRYPT_XML_ALGORITHM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn CryptXmlVerifySignature(hsignature: *const ::core::ffi::c_void, hkey: BCRYPT_KEY_HANDLE, dwflags: CRYPT_XML_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Decrypt(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, foaep: super::super::Foundation::BOOL, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Encrypt(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, foaep: super::super::Foundation::BOOL, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn FindCertsByIssuer(pcertchains: *mut CERT_CHAIN, pcbcertchains: *mut u32, pccertchains: *mut u32, pbencodedissuername: *const u8, cbencodedissuername: u32, pwszpurpose: ::windows_sys::core::PCWSTR, dwkeyspec: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeToken(pallocmemory: *const GENERIC_XML_TOKEN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn GenerateDerivedKey(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cblabel: u32, plabel: *const u8, cbnonce: u32, pnonce: *const u8, derivedkeylength: u32, offset: u32, algid: ::windows_sys::core::PCWSTR, pcbkey: *mut u32, ppkey: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn GetBrowserToken(dwparamtype: u32, pparam: *const ::core::ffi::c_void, pcbtoken: *mut u32, pptoken: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn GetCryptoTransform(hsymmetriccrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, mode: u32, padding: PaddingMode, feedbacksize: u32, direction: Direction, cbiv: u32, piv: *const u8, pphtransform: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn GetKeyedHash(hsymmetriccrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, pphhash: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetToken(cpolicychain: u32, ppolicychain: *const POLICY_ELEMENT, securitytoken: *mut *mut GENERIC_XML_TOKEN, phprooftokencrypto: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn HashCore(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn HashFinal(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn ImportInformationCard(filename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn ManageCardSpace() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptCloseProtectionDescriptor(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptCreateClaim(hsubjectkey: NCRYPT_KEY_HANDLE, hauthoritykey: NCRYPT_KEY_HANDLE, dwclaimtype: u32, pparameterlist: *const BCryptBufferDesc, pbclaimblob: *mut u8, cbclaimblob: u32, pcbresult: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptCreatePersistedKey(hprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszalgid: ::windows_sys::core::PCWSTR, pszkeyname: ::windows_sys::core::PCWSTR, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptCreateProtectionDescriptor(pwszdescriptorstring: ::windows_sys::core::PCWSTR, dwflags: u32, phdescriptor: *mut super::NCRYPT_DESCRIPTOR_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptDecrypt(hkey: NCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptDeleteKey(hkey: NCRYPT_KEY_HANDLE, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptDeriveKey(hsharedsecret: NCRYPT_SECRET_HANDLE, pwszkdf: ::windows_sys::core::PCWSTR, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptEncrypt(hkey: NCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptEnumAlgorithms(hprovider: NCRYPT_PROV_HANDLE, dwalgoperations: NCRYPT_OPERATION, pdwalgcount: *mut u32, ppalglist: *mut *mut NCryptAlgorithmName, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptEnumKeys(hprovider: NCRYPT_PROV_HANDLE, pszscope: ::windows_sys::core::PCWSTR, ppkeyname: *mut *mut NCryptKeyName, ppenumstate: *mut *mut ::core::ffi::c_void, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptEnumStorageProviders(pdwprovidercount: *mut u32, ppproviderlist: *mut *mut NCryptProviderName, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptExportKey(hkey: NCRYPT_KEY_HANDLE, hexportkey: NCRYPT_KEY_HANDLE, pszblobtype: ::windows_sys::core::PCWSTR, pparameterlist: *const BCryptBufferDesc, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptFinalizeKey(hkey: NCRYPT_KEY_HANDLE, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptFreeBuffer(pvinput: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptFreeObject(hobject: NCRYPT_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptGetProperty(hobject: NCRYPT_HANDLE, pszproperty: ::windows_sys::core::PCWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: super::OBJECT_SECURITY_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptGetProtectionDescriptorInfo(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, pmempara: *const NCRYPT_ALLOC_PARA, dwinfotype: u32, ppvinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptImportKey(hprovider: NCRYPT_PROV_HANDLE, himportkey: NCRYPT_KEY_HANDLE, pszblobtype: ::windows_sys::core::PCWSTR, pparameterlist: *const BCryptBufferDesc, phkey: *mut NCRYPT_KEY_HANDLE, pbdata: *const u8, cbdata: u32, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptIsAlgSupported(hprovider: NCRYPT_PROV_HANDLE, pszalgid: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptIsKeyHandle(hkey: NCRYPT_KEY_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptKeyDerivation(hkey: NCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptNotifyChangeKey(hprovider: NCRYPT_PROV_HANDLE, phevent: *mut super::super::Foundation::HANDLE, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptOpenKey(hprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszkeyname: ::windows_sys::core::PCWSTR, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptOpenStorageProvider(phprovider: *mut NCRYPT_PROV_HANDLE, pszprovidername: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptProtectSecret(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: u32, pbdata: *const u8, cbdata: u32, pmempara: *const NCRYPT_ALLOC_PARA, hwnd: super::super::Foundation::HWND, ppbprotectedblob: *mut *mut u8, pcbprotectedblob: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptQueryProtectionDescriptorName(pwszname: ::windows_sys::core::PCWSTR, pwszdescriptorstring: ::windows_sys::core::PWSTR, pcdescriptorstring: *mut usize, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptRegisterProtectionDescriptorName(pwszname: ::windows_sys::core::PCWSTR, pwszdescriptorstring: ::windows_sys::core::PCWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptSecretAgreement(hprivkey: NCRYPT_KEY_HANDLE, hpubkey: NCRYPT_KEY_HANDLE, phagreedsecret: *mut NCRYPT_SECRET_HANDLE, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptSetProperty(hobject: NCRYPT_HANDLE, pszproperty: ::windows_sys::core::PCWSTR, pbinput: *const u8, cbinput: u32, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptSignHash(hkey: NCRYPT_KEY_HANDLE, ppaddinginfo: *const ::core::ffi::c_void, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *mut u8, cbsignature: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptStreamClose(hstream: super::NCRYPT_STREAM_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToProtect(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: u32, hwnd: super::super::Foundation::HWND, pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToUnprotect(pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO, dwflags: u32, hwnd: super::super::Foundation::HWND, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToUnprotectEx(pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO_EX, dwflags: u32, hwnd: super::super::Foundation::HWND, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamUpdate(hstream: super::NCRYPT_STREAM_HANDLE, pbdata: *const u8, cbdata: usize, ffinal: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptTranslateHandle(phprovider: *mut NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, hlegacyprov: usize, hlegacykey: usize, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptUnprotectSecret(phdescriptor: *mut super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: NCRYPT_FLAGS, pbprotectedblob: *const u8, cbprotectedblob: u32, pmempara: *const NCRYPT_ALLOC_PARA, hwnd: super::super::Foundation::HWND, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptVerifyClaim(hsubjectkey: NCRYPT_KEY_HANDLE, hauthoritykey: NCRYPT_KEY_HANDLE, dwclaimtype: u32, pparameterlist: *const BCryptBufferDesc, pbclaimblob: *const u8, cbclaimblob: u32, poutput: *mut BCryptBufferDesc, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn NCryptVerifySignature(hkey: NCRYPT_KEY_HANDLE, ppaddinginfo: *const ::core::ffi::c_void, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *const u8, cbsignature: u32, dwflags: NCRYPT_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXExportCertStore(hstore: HCERTSTORE, ppfx: *mut CRYPTOAPI_BLOB, szpassword: ::windows_sys::core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXExportCertStoreEx(hstore: HCERTSTORE, ppfx: *mut CRYPTOAPI_BLOB, szpassword: ::windows_sys::core::PCWSTR, pvpara: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn PFXImportCertStore(ppfx: *const CRYPTOAPI_BLOB, szpassword: ::windows_sys::core::PCWSTR, dwflags: CRYPT_KEY_FLAGS) -> HCERTSTORE;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXIsPFXBlob(ppfx: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXVerifyPassword(ppfx: *const CRYPTOAPI_BLOB, szpassword: ::windows_sys::core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn SignHash(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbhash: u32, phash: *const u8, hashalgoid: ::windows_sys::core::PCWSTR, pcbsig: *mut u32, ppsig: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn TransformBlock(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
    pub fn TransformFinalBlock(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyHash(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbhash: u32, phash: *const u8, hashalgoid: ::windows_sys::core::PCWSTR, cbsig: u32, psig: *const u8, pfverified: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_CLASS_ALL: u32 = 57344u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_CLASS_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_CLASS_DATA_ENCRYPT: u32 = 24576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_CLASS_HASH: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_CLASS_KEY_EXCHANGE: u32 = 40960u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_CLASS_MSG_ENCRYPT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_CLASS_SIGNATURE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_3DES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_3DES_112: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_AES: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_AES_128: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_AES_192: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_AES_256: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_AGREED_KEY_ANY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_CAST: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_CYLINK_MEK: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_DES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_DESX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_DH_EPHEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_DH_SANDF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_DSS_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_DSS_DMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_DSS_PKCS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_ECDH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_ECDH_EPHEM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_ECDSA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_ECMQV: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_EXAMPLE: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_HASH_REPLACE_OWF: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_HMAC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_IDEA: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_KEA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_MAC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_MD2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_MD4: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_MD5: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_PCT1_MASTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RC2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RC4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RC5: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RIPEMD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RIPEMD160: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RSA_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RSA_ENTRUST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RSA_MSATWORK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RSA_PGP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_RSA_PKCS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SAFERSK128: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SAFERSK64: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SCHANNEL_ENC_KEY: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SCHANNEL_MAC_KEY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SCHANNEL_MASTER_HASH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SEAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SHA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SHA1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SHA_256: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SHA_384: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SHA_512: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SKIPJACK: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SSL2_MASTER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SSL3SHAMD5: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_SSL3_MASTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_TEK: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_THIRDPARTY_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_TLS1PRF: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_SID_TLS1_MASTER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_BLOCK: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_DH: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_DSS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_ECDH: u32 = 3584u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_RSA: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_SECURECHANNEL: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_STREAM: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ALG_TYPE_THIRDPARTY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUDIT_CARD_DELETE: ::windows_sys::core::HRESULT = 1074070017i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUDIT_CARD_IMPORT: ::windows_sys::core::HRESULT = 1074070018i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUDIT_CARD_WRITTEN: ::windows_sys::core::HRESULT = 1074070016i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUDIT_SERVICE_IDLE_STOP: ::windows_sys::core::HRESULT = 1074070022i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUDIT_STORE_DELETE: ::windows_sys::core::HRESULT = 1074070021i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUDIT_STORE_EXPORT: ::windows_sys::core::HRESULT = 1074070020i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUDIT_STORE_IMPORT: ::windows_sys::core::HRESULT = 1074070019i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRegPolicySettings: u32,
    pub pSignerInfo: *mut CMSG_SIGNER_INFO,
}
impl ::core::marker::Copy for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {}
impl ::core::clone::Clone for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub fCommercial: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRegPolicySettings: u32,
    pub fCommercial: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_CA_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPTBUFFER_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_3DES_112_ALGORITHM: &str = "3DES_112";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_3DES_112_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 369u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_3DES_112_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 401u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_3DES_112_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 385u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_3DES_ALGORITHM: &str = "3DES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_3DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 321u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_3DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 353u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_3DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 337u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_ALGORITHM: &str = "AES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 417u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_CCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 465u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 449u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_CMAC_ALGORITHM: &str = "AES-CMAC";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_CMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 257u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 433u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_GCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 481u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_GMAC_ALGORITHM: &str = "AES-GMAC";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_GMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 273u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AES_WRAP_KEY_BLOB: &str = "Rfc3565KeyWrapBlob";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_ALGORITHM_IDENTIFIER {
    pub pszName: ::windows_sys::core::PWSTR,
    pub dwClass: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for BCRYPT_ALGORITHM_IDENTIFIER {}
impl ::core::clone::Clone for BCRYPT_ALGORITHM_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ALGORITHM_NAME: &str = "AlgorithmName";
pub type BCRYPT_ALG_HANDLE = isize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    pub cbSize: u32,
    pub dwInfoVersion: u32,
    pub pbNonce: *mut u8,
    pub cbNonce: u32,
    pub pbAuthData: *mut u8,
    pub cbAuthData: u32,
    pub pbTag: *mut u8,
    pub cbTag: u32,
    pub pbMacContext: *mut u8,
    pub cbMacContext: u32,
    pub cbAAD: u32,
    pub cbData: u64,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {}
impl ::core::clone::Clone for BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AUTH_MODE_CHAIN_CALLS_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AUTH_MODE_IN_PROGRESS_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_AUTH_TAG_LENGTH: &str = "AuthTagLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_BLOCK_LENGTH: &str = "BlockLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_BLOCK_PADDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_BLOCK_SIZE_LIST: &str = "BlockSizeList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_BUFFERS_LOCKED_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CAPI_AES_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CAPI_KDF_ALGORITHM: &str = "CAPI_KDF";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CAPI_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 801u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHACHA20_POLY1305_ALGORITHM: &str = "CHACHA20_POLY1305";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHACHA20_POLY1305_ALG_HANDLE: BCRYPT_ALG_HANDLE = 929u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHAINING_MODE: &str = "ChainingMode";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHAIN_MODE_CBC: &str = "ChainingModeCBC";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHAIN_MODE_CCM: &str = "ChainingModeCCM";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHAIN_MODE_CFB: &str = "ChainingModeCFB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHAIN_MODE_ECB: &str = "ChainingModeECB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHAIN_MODE_GCM: &str = "ChainingModeGCM";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CHAIN_MODE_NA: &str = "ChainingModeN/A";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DESX_ALGORITHM: &str = "DESX";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DESX_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 545u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DESX_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 577u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DESX_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 561u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DES_ALGORITHM: &str = "DES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 497u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 529u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 513u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DH_ALGORITHM: &str = "DH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DH_ALG_HANDLE: BCRYPT_ALG_HANDLE = 641u32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_DH_KEY_BLOB {
    pub dwMagic: BCRYPT_DH_KEY_BLOB_MAGIC,
    pub cbKey: u32,
}
impl ::core::marker::Copy for BCRYPT_DH_KEY_BLOB {}
impl ::core::clone::Clone for BCRYPT_DH_KEY_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_DH_KEY_BLOB_MAGIC = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DH_PUBLIC_MAGIC: BCRYPT_DH_KEY_BLOB_MAGIC = 1112557636u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DH_PRIVATE_MAGIC: BCRYPT_DH_KEY_BLOB_MAGIC = 1448101956u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DH_PARAMETERS: &str = "DHParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DH_PARAMETERS_MAGIC: u32 = 1297107012u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_DH_PARAMETER_HEADER {
    pub cbLength: u32,
    pub dwMagic: u32,
    pub cbKeyLength: u32,
}
impl ::core::marker::Copy for BCRYPT_DH_PARAMETER_HEADER {}
impl ::core::clone::Clone for BCRYPT_DH_PARAMETER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DH_PRIVATE_BLOB: &str = "DHPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DH_PUBLIC_BLOB: &str = "DHPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_ALGORITHM: &str = "DSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 721u32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_DSA_KEY_BLOB {
    pub dwMagic: BCRYPT_DSA_MAGIC,
    pub cbKey: u32,
    pub Count: [u8; 4],
    pub Seed: [u8; 20],
    pub q: [u8; 20],
}
impl ::core::marker::Copy for BCRYPT_DSA_KEY_BLOB {}
impl ::core::clone::Clone for BCRYPT_DSA_KEY_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_DSA_KEY_BLOB_V2 {
    pub dwMagic: BCRYPT_DSA_MAGIC,
    pub cbKey: u32,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: u32,
    pub cbGroupSize: u32,
    pub Count: [u8; 4],
}
impl ::core::marker::Copy for BCRYPT_DSA_KEY_BLOB_V2 {}
impl ::core::clone::Clone for BCRYPT_DSA_KEY_BLOB_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_DSA_MAGIC = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PUBLIC_MAGIC: BCRYPT_DSA_MAGIC = 1112560452u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PRIVATE_MAGIC: BCRYPT_DSA_MAGIC = 1448104772u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PARAMETERS: &str = "DSAParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PARAMETERS_MAGIC: u32 = 1297109828u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PARAMETERS_MAGIC_V2: u32 = 843927620u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_DSA_PARAMETER_HEADER {
    pub cbLength: u32,
    pub dwMagic: u32,
    pub cbKeyLength: u32,
    pub Count: [u8; 4],
    pub Seed: [u8; 20],
    pub q: [u8; 20],
}
impl ::core::marker::Copy for BCRYPT_DSA_PARAMETER_HEADER {}
impl ::core::clone::Clone for BCRYPT_DSA_PARAMETER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_DSA_PARAMETER_HEADER_V2 {
    pub cbLength: u32,
    pub dwMagic: u32,
    pub cbKeyLength: u32,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: u32,
    pub cbGroupSize: u32,
    pub Count: [u8; 4],
}
impl ::core::marker::Copy for BCRYPT_DSA_PARAMETER_HEADER_V2 {}
impl ::core::clone::Clone for BCRYPT_DSA_PARAMETER_HEADER_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PRIVATE_BLOB: &str = "DSAPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PRIVATE_MAGIC_V2: u32 = 844517444u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PUBLIC_BLOB: &str = "DSAPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_DSA_PUBLIC_MAGIC_V2: u32 = 843206724u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_ECCFULLKEY_BLOB {
    pub dwMagic: u32,
    pub dwVersion: u32,
    pub dwCurveType: ECC_CURVE_TYPE_ENUM,
    pub dwCurveGenerationAlgId: ECC_CURVE_ALG_ID_ENUM,
    pub cbFieldLength: u32,
    pub cbSubgroupOrder: u32,
    pub cbCofactor: u32,
    pub cbSeed: u32,
}
impl ::core::marker::Copy for BCRYPT_ECCFULLKEY_BLOB {}
impl ::core::clone::Clone for BCRYPT_ECCFULLKEY_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECCFULLPRIVATE_BLOB: &str = "ECCFULLPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECCFULLPUBLIC_BLOB: &str = "ECCFULLPUBLICBLOB";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_ECCKEY_BLOB {
    pub dwMagic: u32,
    pub cbKey: u32,
}
impl ::core::marker::Copy for BCRYPT_ECCKEY_BLOB {}
impl ::core::clone::Clone for BCRYPT_ECCKEY_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECCPRIVATE_BLOB: &str = "ECCPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECCPUBLIC_BLOB: &str = "ECCPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_25519: &str = "curve25519";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160R1: &str = "brainpoolP160r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160T1: &str = "brainpoolP160t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192R1: &str = "brainpoolP192r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192T1: &str = "brainpoolP192t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224R1: &str = "brainpoolP224r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224T1: &str = "brainpoolP224t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256R1: &str = "brainpoolP256r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256T1: &str = "brainpoolP256t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320R1: &str = "brainpoolP320r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320T1: &str = "brainpoolP320t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384R1: &str = "brainpoolP384r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384T1: &str = "brainpoolP384t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512R1: &str = "brainpoolP512r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512T1: &str = "brainpoolP512t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_EC192WAPI: &str = "ec192wapi";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NAME: &str = "ECCCurveName";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_ECC_CURVE_NAMES {
    pub dwEccCurveNames: u32,
    pub pEccCurveNames: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for BCRYPT_ECC_CURVE_NAMES {}
impl ::core::clone::Clone for BCRYPT_ECC_CURVE_NAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NAME_LIST: &str = "ECCCurveNameList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NISTP192: &str = "nistP192";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NISTP224: &str = "nistP224";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NISTP256: &str = "nistP256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NISTP384: &str = "nistP384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NISTP521: &str = "nistP521";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NUMSP256T1: &str = "numsP256t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NUMSP384T1: &str = "numsP384t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_NUMSP512T1: &str = "numsP512t1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP160K1: &str = "secP160k1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP160R1: &str = "secP160r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP160R2: &str = "secP160r2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP192K1: &str = "secP192k1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP192R1: &str = "secP192r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP224K1: &str = "secP224k1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP224R1: &str = "secP224r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP256K1: &str = "secP256k1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP256R1: &str = "secP256r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP384R1: &str = "secP384r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_SECP521R1: &str = "secP521r1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_WTLS12: &str = "wtls12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_WTLS7: &str = "wtls7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_WTLS9: &str = "wtls9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_X962P192V1: &str = "x962P192v1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_X962P192V2: &str = "x962P192v2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_X962P192V3: &str = "x962P192v3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_X962P239V1: &str = "x962P239v1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_X962P239V2: &str = "x962P239v2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_X962P239V3: &str = "x962P239v3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_CURVE_X962P256V1: &str = "x962P256v1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_FULLKEY_BLOB_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_PARAMETERS: &str = "ECCParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_PARAMETERS_MAGIC: u32 = 1346585413u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_ALGORITHM: &str = "ECDH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_ALG_HANDLE: BCRYPT_ALG_HANDLE = 657u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_P256_ALGORITHM: &str = "ECDH_P256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 673u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_P384_ALGORITHM: &str = "ECDH_P384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 689u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_P521_ALGORITHM: &str = "ECDH_P521";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = 705u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_PRIVATE_GENERIC_MAGIC: u32 = 1447772997u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_PRIVATE_P256_MAGIC: u32 = 843793221u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_PRIVATE_P384_MAGIC: u32 = 877347653u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_PRIVATE_P521_MAGIC: u32 = 910902085u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_PUBLIC_GENERIC_MAGIC: u32 = 1347109701u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_PUBLIC_P256_MAGIC: u32 = 827016005u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_PUBLIC_P384_MAGIC: u32 = 860570437u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDH_PUBLIC_P521_MAGIC: u32 = 894124869u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_ALGORITHM: &str = "ECDSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 241u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_P256_ALGORITHM: &str = "ECDSA_P256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 737u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_P384_ALGORITHM: &str = "ECDSA_P384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 753u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_P521_ALGORITHM: &str = "ECDSA_P521";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = 769u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_PRIVATE_GENERIC_MAGIC: u32 = 1447314245u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: u32 = 844317509u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: u32 = 877871941u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: u32 = 911426373u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_PUBLIC_GENERIC_MAGIC: u32 = 1346650949u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: u32 = 827540293u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: u32 = 861094725u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: u32 = 894649157u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_EFFECTIVE_KEY_LENGTH: &str = "EffectiveKeyLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ENABLE_INCOMPATIBLE_FIPS_CHECKS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_EXTENDED_KEYSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_GENERATE_IV: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_GLOBAL_PARAMETERS: &str = "SecretAgreementParam";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_BLOCK_LENGTH: &str = "HashBlockLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_INTERFACE_MAJORVERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_LENGTH: &str = "HashDigestLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_OID_LIST: &str = "HashOIDList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_HASH_OPERATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_OPERATION_HASH_DATA: BCRYPT_HASH_OPERATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_OPERATION_FINISH_HASH: BCRYPT_HASH_OPERATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HKDF_ALGORITHM: &str = "HKDF";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HKDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 913u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HKDF_HASH_ALGORITHM: &str = "HkdfHashAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HKDF_PRK_AND_FINALIZE: &str = "HkdfPrkAndFinalize";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HKDF_SALT_AND_FINALIZE: &str = "HkdfSaltAndFinalize";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HMAC_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 289u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HMAC_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 305u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HMAC_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = 145u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HMAC_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = 161u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HMAC_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 177u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HMAC_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 193u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HMAC_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 209u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_INITIALIZATION_VECTOR: &str = "IV";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_INTERFACE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: BCRYPT_INTERFACE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CIPHER_INTERFACE: BCRYPT_INTERFACE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_INTERFACE: BCRYPT_INTERFACE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RNG_INTERFACE: BCRYPT_INTERFACE = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SECRET_AGREEMENT_INTERFACE: BCRYPT_INTERFACE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SIGNATURE_INTERFACE: BCRYPT_INTERFACE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_STORAGE_INTERFACE: BCRYPT_INTERFACE = 65537u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SCHANNEL_INTERFACE: BCRYPT_INTERFACE = 65538u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SCHANNEL_SIGNATURE_INTERFACE: BCRYPT_INTERFACE = 65539u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_INTERFACE_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for BCRYPT_INTERFACE_VERSION {}
impl ::core::clone::Clone for BCRYPT_INTERFACE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_IS_IFX_TPM_WEAK_KEY: &str = "IsIfxTpmWeakKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_IS_KEYED_HASH: &str = "IsKeyedHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_IS_REUSABLE_HASH: &str = "IsReusableHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KDF_HASH: &str = "HASH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KDF_HKDF: &str = "HKDF";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KDF_HMAC: &str = "HMAC";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KDF_RAW_SECRET: &str = "TRUNCATE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KDF_SP80056A_CONCAT: &str = "SP800_56A_CONCAT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KDF_TLS_PRF: &str = "TLS_PRF";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_KEY_BLOB {
    pub Magic: u32,
}
impl ::core::marker::Copy for BCRYPT_KEY_BLOB {}
impl ::core::clone::Clone for BCRYPT_KEY_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_DATA_BLOB: &str = "KeyDataBlob";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_KEY_DATA_BLOB_HEADER {
    pub dwMagic: u32,
    pub dwVersion: u32,
    pub cbKeyData: u32,
}
impl ::core::marker::Copy for BCRYPT_KEY_DATA_BLOB_HEADER {}
impl ::core::clone::Clone for BCRYPT_KEY_DATA_BLOB_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_DATA_BLOB_MAGIC: u32 = 1296188491u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_DATA_BLOB_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_DERIVATION_INTERFACE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_DERIVATION_OPERATION: u32 = 64u32;
pub type BCRYPT_KEY_HANDLE = isize;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_LENGTH: &str = "KeyLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_LENGTHS: &str = "KeyLengths";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_KEY_LENGTHS_STRUCT {
    pub dwMinLength: u32,
    pub dwMaxLength: u32,
    pub dwIncrement: u32,
}
impl ::core::marker::Copy for BCRYPT_KEY_LENGTHS_STRUCT {}
impl ::core::clone::Clone for BCRYPT_KEY_LENGTHS_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_OBJECT_LENGTH: &str = "KeyObjectLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_STRENGTH: &str = "KeyStrength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_VALIDATION_RANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_VALIDATION_RANGE_AND_ORDER: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_KEY_VALIDATION_REGENERATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MD2_ALGORITHM: &str = "MD2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MD4_ALGORITHM: &str = "MD4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 17u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MD5_ALGORITHM: &str = "MD5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = 33u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MESSAGE_BLOCK_LENGTH: &str = "MessageBlockLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MULTI_FLAG: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_MULTI_HASH_OPERATION {
    pub iHash: u32,
    pub hashOperation: BCRYPT_HASH_OPERATION_TYPE,
    pub pbBuffer: *mut u8,
    pub cbBuffer: u32,
}
impl ::core::marker::Copy for BCRYPT_MULTI_HASH_OPERATION {}
impl ::core::clone::Clone for BCRYPT_MULTI_HASH_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_MULTI_OBJECT_LENGTH: &str = "MultiObjectLength";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    pub cbPerObject: u32,
    pub cbPerElement: u32,
}
impl ::core::marker::Copy for BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {}
impl ::core::clone::Clone for BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_MULTI_OPERATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_OPERATION_TYPE_HASH: BCRYPT_MULTI_OPERATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_NO_KEY_VALIDATION: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_OAEP_PADDING_INFO {
    pub pszAlgId: ::windows_sys::core::PCWSTR,
    pub pbLabel: *mut u8,
    pub cbLabel: u32,
}
impl ::core::marker::Copy for BCRYPT_OAEP_PADDING_INFO {}
impl ::core::clone::Clone for BCRYPT_OAEP_PADDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_OBJECT_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_OBJECT_LENGTH: &str = "ObjectLength";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_OID {
    pub cbOID: u32,
    pub pbOID: *mut u8,
}
impl ::core::marker::Copy for BCRYPT_OID {}
impl ::core::clone::Clone for BCRYPT_OID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_OID_LIST {
    pub dwOIDCount: u32,
    pub pOIDs: *mut BCRYPT_OID,
}
impl ::core::marker::Copy for BCRYPT_OID_LIST {}
impl ::core::clone::Clone for BCRYPT_OID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_OPAQUE_KEY_BLOB: &str = "OpaqueKeyBlob";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ALG_HANDLE_HMAC_FLAG: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PROV_DISPATCH: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_REUSABLE_FLAG: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_OPERATION = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_CIPHER_OPERATION: BCRYPT_OPERATION = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_HASH_OPERATION: BCRYPT_OPERATION = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: BCRYPT_OPERATION = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SECRET_AGREEMENT_OPERATION: BCRYPT_OPERATION = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SIGNATURE_OPERATION: BCRYPT_OPERATION = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RNG_OPERATION: BCRYPT_OPERATION = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PADDING_SCHEMES: &str = "PaddingSchemes";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PBKDF2_ALGORITHM: &str = "PBKDF2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PBKDF2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 817u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PCP_PLATFORM_TYPE_PROPERTY: &str = "PCP_PLATFORM_TYPE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PCP_PROVIDER_VERSION_PROPERTY: &str = "PCP_PROVIDER_VERSION";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_PKCS1_PADDING_INFO {
    pub pszAlgId: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for BCRYPT_PKCS1_PADDING_INFO {}
impl ::core::clone::Clone for BCRYPT_PKCS1_PADDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PRIMITIVE_TYPE: &str = "PrimitiveType";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PRIVATE_KEY: &str = "PrivKeyVal";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PRIVATE_KEY_BLOB: &str = "PRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PRIVATE_KEY_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PROVIDER_HANDLE: &str = "ProviderHandle";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_PROVIDER_NAME {
    pub pszProviderName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for BCRYPT_PROVIDER_NAME {}
impl ::core::clone::Clone for BCRYPT_PROVIDER_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_PSS_PADDING_INFO {
    pub pszAlgId: ::windows_sys::core::PCWSTR,
    pub cbSalt: u32,
}
impl ::core::marker::Copy for BCRYPT_PSS_PADDING_INFO {}
impl ::core::clone::Clone for BCRYPT_PSS_PADDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PUBLIC_KEY_BLOB: &str = "PUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PUBLIC_KEY_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PUBLIC_KEY_LENGTH: &str = "PublicKeyLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_QUERY_PROVIDER_MODE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ANY: BCRYPT_QUERY_PROVIDER_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_UM: BCRYPT_QUERY_PROVIDER_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_KM: BCRYPT_QUERY_PROVIDER_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MM: BCRYPT_QUERY_PROVIDER_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RC2_ALGORITHM: &str = "RC2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RC2_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 593u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RC2_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 625u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RC2_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 609u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RC4_ALGORITHM: &str = "RC4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RC4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 113u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_RESOLVE_PROVIDERS_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ALL_FUNCTIONS: BCRYPT_RESOLVE_PROVIDERS_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ALL_PROVIDERS: BCRYPT_RESOLVE_PROVIDERS_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RNG_ALGORITHM: &str = "RNG";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RNG_ALG_HANDLE: BCRYPT_ALG_HANDLE = 129u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RNG_DUAL_EC_ALGORITHM: &str = "DUALECRNG";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RNG_FIPS186_DSA_ALGORITHM: &str = "FIPS186DSARNG";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RNG_USE_ENTROPY_IN_BUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSAFULLPRIVATE_BLOB: &str = "RSAFULLPRIVATEBLOB";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCRYPT_RSAKEY_BLOB {
    pub Magic: BCRYPT_RSAKEY_BLOB_MAGIC,
    pub BitLength: u32,
    pub cbPublicExp: u32,
    pub cbModulus: u32,
    pub cbPrime1: u32,
    pub cbPrime2: u32,
}
impl ::core::marker::Copy for BCRYPT_RSAKEY_BLOB {}
impl ::core::clone::Clone for BCRYPT_RSAKEY_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_RSAKEY_BLOB_MAGIC = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSAPUBLIC_MAGIC: BCRYPT_RSAKEY_BLOB_MAGIC = 826364754u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSAPRIVATE_MAGIC: BCRYPT_RSAKEY_BLOB_MAGIC = 843141970u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSAFULLPRIVATE_MAGIC: BCRYPT_RSAKEY_BLOB_MAGIC = 859919186u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSAPRIVATE_BLOB: &str = "RSAPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSAPUBLIC_BLOB: &str = "RSAPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSA_ALGORITHM: &str = "RSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 225u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSA_SIGN_ALGORITHM: &str = "RSA_SIGN";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_RSA_SIGN_ALG_HANDLE: BCRYPT_ALG_HANDLE = 785u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SHA1_ALGORITHM: &str = "SHA1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = 49u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SHA256_ALGORITHM: &str = "SHA256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 65u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SHA384_ALGORITHM: &str = "SHA384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 81u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SHA512_ALGORITHM: &str = "SHA512";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 97u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SIGNATURE_LENGTH: &str = "SignatureLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SP800108_CTR_HMAC_ALGORITHM: &str = "SP800_108_CTR_HMAC";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SP800108_CTR_HMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 833u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SP80056A_CONCAT_ALGORITHM: &str = "SP800_56A_CONCAT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SP80056A_CONCAT_ALG_HANDLE: BCRYPT_ALG_HANDLE = 849u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SUPPORTED_PAD_OAEP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SUPPORTED_PAD_PKCS1_ENC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SUPPORTED_PAD_PKCS1_SIG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SUPPORTED_PAD_PSS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_SUPPORTED_PAD_ROUTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type BCRYPT_TABLE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LOCAL: BCRYPT_TABLE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DOMAIN: BCRYPT_TABLE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_TLS1_1_KDF_ALGORITHM: &str = "TLS1_1_KDF";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_TLS1_1_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 865u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_TLS1_2_KDF_ALGORITHM: &str = "TLS1_2_KDF";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_TLS1_2_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 881u32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_TLS_CBC_HMAC_VERIFY_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_XTS_AES_ALGORITHM: &str = "XTS-AES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_XTS_AES_ALG_HANDLE: BCRYPT_ALG_HANDLE = 897u32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCryptBuffer {
    pub cbBuffer: u32,
    pub BufferType: u32,
    pub pvBuffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for BCryptBuffer {}
impl ::core::clone::Clone for BCryptBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct BCryptBufferDesc {
    pub ulVersion: u32,
    pub cBuffers: u32,
    pub pBuffers: *mut BCryptBuffer,
}
impl ::core::marker::Copy for BCryptBufferDesc {}
impl ::core::clone::Clone for BCryptBufferDesc {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CALG_OID_INFO_CNG_ONLY: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CALG_OID_INFO_PARAMETERS: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CASetupProperty = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_INVALID: CASetupProperty = -1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_CATYPE: CASetupProperty = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_CAKEYINFORMATION: CASetupProperty = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_INTERACTIVE: CASetupProperty = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_CANAME: CASetupProperty = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_CADSSUFFIX: CASetupProperty = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_VALIDITYPERIOD: CASetupProperty = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_VALIDITYPERIODUNIT: CASetupProperty = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_EXPIRATIONDATE: CASetupProperty = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_PRESERVEDATABASE: CASetupProperty = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_DATABASEDIRECTORY: CASetupProperty = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_LOGDIRECTORY: CASetupProperty = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_SHAREDFOLDER: CASetupProperty = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_PARENTCAMACHINE: CASetupProperty = 12i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_PARENTCANAME: CASetupProperty = 13i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_REQUESTFILE: CASetupProperty = 14i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_WEBCAMACHINE: CASetupProperty = 15i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_SETUPPROP_WEBCANAME: CASetupProperty = 16i32;
pub const CCertSrvSetup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2518620175, data2: 62812, data3: 16701, data4: [169, 179, 125, 42, 244, 216, 228, 47] };
pub const CCertSrvSetupKeyInformation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 943143174, data2: 21555, data3: 17971, data4: [176, 251, 41, 183, 231, 130, 98, 225] };
pub const CCertificateEnrollmentPolicyServerSetup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2950887986, data2: 16817, data3: 17821, data4: [165, 222, 73, 173, 216, 167, 33, 130] };
pub const CCertificateEnrollmentServerSetup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2567107516, data2: 34991, data3: 19704, data4: [174, 98, 113, 64, 83, 21, 82, 182] };
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CEPSetupProperty = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_AUTHENTICATION: CEPSetupProperty = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_SSLCERTHASH: CEPSetupProperty = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_URL: CEPSetupProperty = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_KEYBASED_RENEWAL: CEPSetupProperty = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERTIFICATE_CHAIN_BLOB {
    pub certCount: u32,
    pub rawCertificates: *mut CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERTIFICATE_CHAIN_BLOB {}
impl ::core::clone::Clone for CERTIFICATE_CHAIN_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_ACCESS_DESCRIPTION {
    pub pszAccessMethod: ::windows_sys::core::PSTR,
    pub AccessLocation: CERT_ALT_NAME_ENTRY,
}
impl ::core::marker::Copy for CERT_ACCESS_DESCRIPTION {}
impl ::core::clone::Clone for CERT_ACCESS_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ACCESS_STATE_GP_SYSTEM_STORE_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ACCESS_STATE_LM_SYSTEM_STORE_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ACCESS_STATE_PROP_ID: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ACCESS_STATE_SHARED_USER_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ACCESS_STATE_SYSTEM_STORE_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ACCESS_STATE_WRITE_PERSIST_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AIA_URL_RETRIEVED_PROP_ID: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ALT_NAME_EDI_PARTY_NAME: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_ALT_NAME_ENTRY {
    pub dwAltNameChoice: u32,
    pub Anonymous: CERT_ALT_NAME_ENTRY_0,
}
impl ::core::marker::Copy for CERT_ALT_NAME_ENTRY {}
impl ::core::clone::Clone for CERT_ALT_NAME_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CERT_ALT_NAME_ENTRY_0 {
    pub pOtherName: *mut CERT_OTHER_NAME,
    pub pwszRfc822Name: ::windows_sys::core::PWSTR,
    pub pwszDNSName: ::windows_sys::core::PWSTR,
    pub DirectoryName: CRYPTOAPI_BLOB,
    pub pwszURL: ::windows_sys::core::PWSTR,
    pub IPAddress: CRYPTOAPI_BLOB,
    pub pszRegisteredID: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CERT_ALT_NAME_ENTRY_0 {}
impl ::core::clone::Clone for CERT_ALT_NAME_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_SHIFT: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_ALT_NAME_INFO {
    pub cAltEntry: u32,
    pub rgAltEntry: *mut CERT_ALT_NAME_ENTRY,
}
impl ::core::marker::Copy for CERT_ALT_NAME_INFO {}
impl ::core::clone::Clone for CERT_ALT_NAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_SHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ALT_NAME_X400_ADDRESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ARCHIVED_KEY_HASH_PROP_ID: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ARCHIVED_PROP_ID: u32 = 19u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_AUTHORITY_INFO_ACCESS {
    pub cAccDescr: u32,
    pub rgAccDescr: *mut CERT_ACCESS_DESCRIPTION,
}
impl ::core::marker::Copy for CERT_AUTHORITY_INFO_ACCESS {}
impl ::core::clone::Clone for CERT_AUTHORITY_INFO_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 68u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_AUTHORITY_KEY_ID2_INFO {
    pub KeyId: CRYPTOAPI_BLOB,
    pub AuthorityCertIssuer: CERT_ALT_NAME_INFO,
    pub AuthorityCertSerialNumber: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_AUTHORITY_KEY_ID2_INFO {}
impl ::core::clone::Clone for CERT_AUTHORITY_KEY_ID2_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_AUTHORITY_KEY_ID_INFO {
    pub KeyId: CRYPTOAPI_BLOB,
    pub CertIssuer: CRYPTOAPI_BLOB,
    pub CertSerialNumber: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_AUTHORITY_KEY_ID_INFO {}
impl ::core::clone::Clone for CERT_AUTHORITY_KEY_ID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_PARTIAL_CHAIN_LOGGING_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_UNTRUSTED_ROOT_LOGGING_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: &str = "EncodedCtl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_AUTO_UPDATE_FLAGS_VALUE_NAME: &str = "Flags";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: &str = "LastSyncTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME: &str = "RootDirUrl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: &str = "SyncDeltaTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_CAB_FILENAME: &str = "authrootstl.cab";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_CERT_EXT: &str = ".crt";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_CTL_FILENAME: &str = "authroot.stl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_CTL_FILENAME_A: &str = "authroot.stl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_SEQ_FILENAME: &str = "authrootseq.txt";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTO_ENROLL_PROP_ID: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTO_ENROLL_RETRY_PROP_ID: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTO_UPDATE_DISABLE_RANDOM_QUERY_STRING_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME: &str = "RootDirUrl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_AUTO_UPDATE_SYNC_FROM_DIR_URL_VALUE_NAME: &str = "SyncFromDirUrl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_BACKED_UP_PROP_ID: u32 = 69u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_BASIC_CONSTRAINTS2_INFO {
    pub fCA: super::super::Foundation::BOOL,
    pub fPathLenConstraint: super::super::Foundation::BOOL,
    pub dwPathLenConstraint: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_BASIC_CONSTRAINTS2_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_BASIC_CONSTRAINTS2_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_BASIC_CONSTRAINTS_INFO {
    pub SubjectType: CRYPT_BIT_BLOB,
    pub fPathLenConstraint: super::super::Foundation::BOOL,
    pub dwPathLenConstraint: u32,
    pub cSubtreesConstraint: u32,
    pub rgSubtreesConstraint: *mut CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_BASIC_CONSTRAINTS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_BASIC_CONSTRAINTS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_BIOMETRIC_DATA {
    pub dwTypeOfBiometricDataChoice: CERT_BIOMETRIC_DATA_TYPE,
    pub Anonymous: CERT_BIOMETRIC_DATA_0,
    pub HashedUrl: CERT_HASHED_URL,
}
impl ::core::marker::Copy for CERT_BIOMETRIC_DATA {}
impl ::core::clone::Clone for CERT_BIOMETRIC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CERT_BIOMETRIC_DATA_0 {
    pub dwPredefined: u32,
    pub pszObjId: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CERT_BIOMETRIC_DATA_0 {}
impl ::core::clone::Clone for CERT_BIOMETRIC_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_BIOMETRIC_DATA_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_BIOMETRIC_PREDEFINED_DATA_CHOICE: CERT_BIOMETRIC_DATA_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_BIOMETRIC_OID_DATA_CHOICE: CERT_BIOMETRIC_DATA_TYPE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_BIOMETRIC_EXT_INFO {
    pub cBiometricData: u32,
    pub rgBiometricData: *mut CERT_BIOMETRIC_DATA,
}
impl ::core::marker::Copy for CERT_BIOMETRIC_EXT_INFO {}
impl ::core::clone::Clone for CERT_BIOMETRIC_EXT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_BIOMETRIC_PICTURE_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_BIOMETRIC_SIGNATURE_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_BUNDLE_CERTIFICATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_BUNDLE_CRL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CASE_INSENSITIVE_IS_RDN_ATTRS_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CA_DISABLE_CRL_PROP_ID: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CA_SUBJECT_FLAG: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CEP_PROP_ID: u32 = 87u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_CHAIN {
    pub cCerts: u32,
    pub certs: *mut CRYPTOAPI_BLOB,
    pub keyLocatorInfo: CRYPT_KEY_PROV_INFO,
}
impl ::core::marker::Copy for CERT_CHAIN {}
impl ::core::clone::Clone for CERT_CHAIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_CURRENT_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_FLAGS_VALUE_NAME: &str = "AutoFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_FLUSH_DISABLE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_FLUSH_FIRST_DELTA_SECONDS_VALUE_NAME: &str = "AutoFlushFirstDeltaSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_FLUSH_NEXT_DELTA_SECONDS_VALUE_NAME: &str = "AutoFlushNextDeltaSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_HPKP_RULE_INFO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_IMPERSONATED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_LOCAL_MACHINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_LOG_CREATE_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_LOG_FILE_NAME_VALUE_NAME: &str = "AutoLogFileName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_LOG_FLUSH_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_LOG_FREE_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_NETWORK_INFO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_PINRULE_INFO: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_PROCESS_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_AUTO_SERIAL_LOCAL_MACHINE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_CACHE_END_CERT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_CACHE_RESYNC_FILETIME_VALUE_NAME: &str = "ChainCacheResyncFiletime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_CONFIG_REGPATH: &str = "Software\\Microsoft\\Cryptography\\OID\\EncodingType 0\\CertDllCreateCertificateChainEngine\\Config";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CHAIN_CONTEXT {
    pub cbSize: u32,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub cChain: u32,
    pub rgpChain: *mut *mut CERT_SIMPLE_CHAIN,
    pub cLowerQualityChainContext: u32,
    pub rgpLowerQualityChainContext: *mut *mut CERT_CHAIN_CONTEXT,
    pub fHasRevocationFreshnessTime: super::super::Foundation::BOOL,
    pub dwRevocationFreshnessTime: u32,
    pub dwCreateFlags: u32,
    pub ChainId: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_CHAIN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_CHAIN_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_DEFAULT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_VALUE_NAME: &str = "CRLValidityExtensionPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_CROSS_CERT_DOWNLOAD_INTERVAL_HOURS_VALUE_NAME: &str = "CrossCertDownloadIntervalHours";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DEFAULT_CONFIG_SUBDIR: &str = "Default";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_AIA: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_AIA_URL_RETRIEVAL_VALUE_NAME: &str = "DisableAIAUrlRetrieval";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_ALL_EKU_WEAK_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_AUTH_ROOT_AUTO_UPDATE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_AUTO_FLUSH_PROCESS_NAME_LIST_VALUE_NAME: &str = "DisableAutoFlushProcessNameList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_CA_NAME_CONSTRAINTS_VALUE_NAME: &str = "DisableCANameConstraints";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_CODE_SIGNING_WEAK_FLAG: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_ECC_PARA_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAG: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_MANDATORY_BASIC_CONSTRAINTS_VALUE_NAME: &str = "DisableMandatoryBasicConstraints";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_MD2_MD4: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_MOTW_FILE_HASH_WEAK_FLAG: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_HASH_WEAK_FLAG: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_MY_PEER_TRUST: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_PASS1_QUALITY_FILTERING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_SERIAL_CHAIN_VALUE_NAME: &str = "DisableSerialChain";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_SERVER_AUTH_WEAK_FLAG: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_SYNC_WITH_SSL_TIME_VALUE_NAME: &str = "DisableSyncWithSslTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAG: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_TIMESTAMP_WEAK_FLAG: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_DISABLE_UNSUPPORTED_CRITICAL_EXTENSIONS_VALUE_NAME: &str = "DisableUnsupportedCriticalExtensions";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CHAIN_ELEMENT {
    pub cbSize: u32,
    pub pCertContext: *const CERT_CONTEXT,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub pRevocationInfo: *mut CERT_REVOCATION_INFO,
    pub pIssuanceUsage: *mut CTL_USAGE,
    pub pApplicationUsage: *mut CTL_USAGE,
    pub pwszExtendedErrorInfo: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_CHAIN_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_CHAIN_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_ALL_EKU_HYGIENE_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_CODE_SIGNING_HYGIENE_FLAG: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_MD2_MD4_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_ONLY_WEAK_LOGGING_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_PEER_TRUST: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_SERVER_AUTH_HYGIENE_FLAG: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_SHARE_STORE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_TIMESTAMP_HYGIENE_FLAG: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_WEAK_LOGGING_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_WEAK_RSA_ROOT_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_WEAK_SETTINGS_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ENABLE_WEAK_SIGNATURE_FLAGS_VALUE_NAME: &str = "EnableWeakSignatureFlags";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_CHAIN_ENGINE_CONFIG {
    pub cbSize: u32,
    pub hRestrictedRoot: HCERTSTORE,
    pub hRestrictedTrust: HCERTSTORE,
    pub hRestrictedOther: HCERTSTORE,
    pub cAdditionalStore: u32,
    pub rghAdditionalStore: *mut HCERTSTORE,
    pub dwFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub MaximumCachedCertificates: u32,
    pub CycleDetectionModulus: u32,
    pub hExclusiveRoot: HCERTSTORE,
    pub hExclusiveTrustedPeople: HCERTSTORE,
    pub dwExclusiveFlags: u32,
}
impl ::core::marker::Copy for CERT_CHAIN_ENGINE_CONFIG {}
impl ::core::clone::Clone for CERT_CHAIN_ENGINE_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_EXCLUSIVE_ENABLE_CA_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_FIND_BY_ISSUER: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CHAIN_FIND_BY_ISSUER_PARA {
    pub cbSize: u32,
    pub pszUsageIdentifier: ::windows_sys::core::PCSTR,
    pub dwKeySpec: u32,
    pub dwAcquirePrivateKeyFlags: u32,
    pub cIssuer: u32,
    pub rgIssuer: *mut CRYPTOAPI_BLOB,
    pub pfnFindCallback: PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK,
    pub pvFindArg: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_CHAIN_FIND_BY_ISSUER_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_CHAIN_FIND_BY_ISSUER_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_HAS_MOTW: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_DEFAULT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_VALUE_NAME: &str = "MaxAIAUrlCountInCert";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_DEFAULT: u32 = 100000u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME: &str = "MaxAIAUrlRetrievalByteCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_DEFAULT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_VALUE_NAME: &str = "MaxAIAUrlRetrievalCertCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_DEFAULT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_VALUE_NAME: &str = "MaxAIAUrlRetrievalCountPerChain";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DEFAULT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DISABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_VALUE_NAME: &str = "MaxSslTimeUpdatedEventCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MAX_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME: &str = "MaxUrlRetrievalByteCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MIN_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DEFAULT: u32 = 1023u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME: &str = "MinRsaPubKeyBitLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_MOTW_IGNORE_AFTER_TIME_WEAK_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_OCSP_VALIDITY_SECONDS_VALUE_NAME: &str = "OcspValiditySeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_ONLY_ADDITIONAL_AND_AUTH_ROOT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_OPTIONS_VALUE_NAME: &str = "Options";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_OPTION_DISABLE_AIA_URL_RETRIEVAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_OPTION_ENABLE_SIA_URL_RETRIEVAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_OPT_IN_WEAK_FLAGS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_OPT_IN_WEAK_SIGNATURE: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_CHAIN_PARA {
    pub cbSize: u32,
    pub RequestedUsage: CERT_USAGE_MATCH,
}
impl ::core::marker::Copy for CERT_CHAIN_PARA {}
impl ::core::clone::Clone for CERT_CHAIN_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_AUTHENTICODE: ::windows_sys::core::PCSTR = 2i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_AUTHENTICODE_TS: ::windows_sys::core::PCSTR = 3i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_BASE: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_BASIC_CONSTRAINTS: ::windows_sys::core::PCSTR = 5i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_EV: ::windows_sys::core::PCSTR = 8i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_CHAIN_POLICY_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG: CERT_CHAIN_POLICY_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_CTL_NOT_TIME_VALID_FLAG: CERT_CHAIN_POLICY_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_NESTED_FLAG: CERT_CHAIN_POLICY_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS: CERT_CHAIN_POLICY_FLAGS = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_BASIC_CONSTRAINTS_FLAG: CERT_CHAIN_POLICY_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG: CERT_CHAIN_POLICY_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG: CERT_CHAIN_POLICY_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG: CERT_CHAIN_POLICY_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_POLICY_FLAG: CERT_CHAIN_POLICY_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG: CERT_CHAIN_POLICY_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_CTL_SIGNER_REV_UNKNOWN_FLAG: CERT_CHAIN_POLICY_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG: CERT_CHAIN_POLICY_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG: CERT_CHAIN_POLICY_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_ALL_REV_UNKNOWN_FLAGS: CERT_CHAIN_POLICY_FLAGS = 3840u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_ALLOW_TESTROOT_FLAG: CERT_CHAIN_POLICY_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_TRUST_TESTROOT_FLAG: CERT_CHAIN_POLICY_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_NOT_SUPPORTED_CRITICAL_EXT_FLAG: CERT_CHAIN_POLICY_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_PEER_TRUST_FLAG: CERT_CHAIN_POLICY_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_IGNORE_WEAK_SIGNATURE_FLAG: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_MICROSOFT_ROOT: ::windows_sys::core::PCSTR = 7i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_NT_AUTH: ::windows_sys::core::PCSTR = 6i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwFlags: CERT_CHAIN_POLICY_FLAGS,
    pub pvExtraPolicyPara: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CERT_CHAIN_POLICY_PARA {}
impl ::core::clone::Clone for CERT_CHAIN_POLICY_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL: ::windows_sys::core::PCSTR = 4i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_F12: ::windows_sys::core::PCSTR = 9i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_F12_ERROR_LEVEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_F12_NONE_CATEGORY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_F12_ROOT_PROGRAM_CATEGORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_F12_SUCCESS_LEVEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_F12_WARNING_LEVEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_F12_WEAK_CRYPTO_CATEGORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_HPKP_HEADER: ::windows_sys::core::PCSTR = 10i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN: ::windows_sys::core::PCSTR = 12i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_ERROR: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_ERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_WARNING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_SUCCESS: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwError: u32,
    pub lChainIndex: i32,
    pub lElementIndex: i32,
    pub pvExtraPolicyStatus: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CERT_CHAIN_POLICY_STATUS {}
impl ::core::clone::Clone for CERT_CHAIN_POLICY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_POLICY_THIRD_PARTY_ROOT: ::windows_sys::core::PCSTR = 11i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_RETURN_LOWER_QUALITY_CONTEXTS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_REVOCATION_CHECK_END_CERT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_REVOCATION_CHECK_OCSP_CERT: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_REV_ACCUMULATIVE_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: &str = "ChainRevAccumulativeUrlRetrievalTimeoutMilliseconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_SERIAL_CHAIN_LOG_FILE_NAME_VALUE_NAME: &str = "SerialChainLogFileName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_SSL_HANDSHAKE_LOG_FILE_NAME_VALUE_NAME: &str = "SslHandshakeLogFileName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_THREAD_STORE_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_TIMESTAMP_TIME: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: &str = "ChainUrlRetrievalTimeoutMilliseconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_USE_LOCAL_MACHINE_STORE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_AFTER_TIME_NAME: &str = "AfterTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_ALL_CONFIG_NAME: &str = "All";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_FILE_HASH_AFTER_TIME_NAME: &str = "FileHashAfterTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_FLAGS_NAME: &str = "Flags";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_HYGIENE_NAME: &str = "Hygiene";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_MIN_BIT_LENGTH_NAME: &str = "MinBitLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_PREFIX_NAME: &str = "Weak";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_RSA_PUB_KEY_TIME_VALUE_NAME: &str = "WeakRsaPubKeyTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_SHA256_ALLOW_NAME: &str = "Sha256Allow";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_SIGNATURE_LOG_DIR_VALUE_NAME: &str = "WeakSignatureLogDir";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_THIRD_PARTY_CONFIG_NAME: &str = "ThirdParty";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_WEAK_TIMESTAMP_HASH_AFTER_TIME_NAME: &str = "TimestampHashAfterTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CLOSE_STORE_CHECK_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CLOSE_STORE_FORCE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CLR_DELETE_KEY_PROP_ID: u32 = 125u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_ATTR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_CERT_ID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_CROSS_CERT_DIST_POINTS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_CTL_USAGE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_ENHKEY_USAGE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_EXISTING: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_HASH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_HASH_STR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_HAS_PRIVATE_KEY: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_ISSUER_OF: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_KEY_IDENTIFIER: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_KEY_SPEC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_MD5_HASH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_NAME_STR_A: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_NAME_STR_W: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_PROPERTY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_PUBKEY_MD5_HASH: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_PUBLIC_KEY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_SHA1_HASH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_SHIFT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_SIGNATURE_HASH: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_SUBJECT_CERT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_COMPARE_SUBJECT_INFO_ACCESS: u32 = 19u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CONTEXT {
    pub dwCertEncodingType: u32,
    pub pbCertEncoded: *mut u8,
    pub cbCertEncoded: u32,
    pub pCertInfo: *mut CERT_INFO,
    pub hCertStore: HCERTSTORE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CONTEXT_REVOCATION_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_CONTROL_STORE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTRL_COMMIT_FORCE_FLAG: CERT_CONTROL_STORE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTRL_COMMIT_CLEAR_FLAG: CERT_CONTROL_STORE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTRL_INHIBIT_DUPLICATE_HANDLE_FLAG: CERT_CONTROL_STORE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CREATE_CONTEXT_NOCOPY_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CREATE_CONTEXT_NO_ENTRY_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CREATE_CONTEXT_NO_HCRYPTMSG_FLAG: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CREATE_CONTEXT_PARA {
    pub cbSize: u32,
    pub pfnFree: PFN_CRYPT_FREE,
    pub pvFree: *mut ::core::ffi::c_void,
    pub pfnSort: PFN_CERT_CREATE_CONTEXT_SORT_FUNC,
    pub pvSort: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_CREATE_CONTEXT_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_CREATE_CONTEXT_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CREATE_CONTEXT_SORTED_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_CREATE_SELFSIGN_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CREATE_SELFSIGN_NO_KEY_INFO: CERT_CREATE_SELFSIGN_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CREATE_SELFSIGN_NO_SIGN: CERT_CREATE_SELFSIGN_FLAGS = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CRL_CONTEXT_PAIR {
    pub pCertContext: *const CERT_CONTEXT,
    pub pCrlContext: *mut CRL_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_CRL_CONTEXT_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_CRL_CONTEXT_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CRL_SIGN_KEY_USAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CROSS_CERT_DIST_POINTS_PROP_ID: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CTL_USAGE_PROP_ID: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DATA_ENCIPHERMENT_KEY_USAGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DATE_STAMP_PROP_ID: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DECIPHER_ONLY_KEY_USAGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DEFAULT_OID_PUBLIC_KEY_SIGN: &str = "1.2.840.113549.1.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DEFAULT_OID_PUBLIC_KEY_XCHG: &str = "1.2.840.113549.1.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DESCRIPTION_PROP_ID: u32 = 13u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_DH_PARAMETERS {
    pub p: CRYPTOAPI_BLOB,
    pub g: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_DH_PARAMETERS {}
impl ::core::clone::Clone for CERT_DH_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DIGITAL_SIGNATURE_KEY_USAGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISABLE_PIN_RULES_AUTO_UPDATE_VALUE_NAME: &str = "DisablePinRulesAutoUpdate";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISABLE_ROOT_AUTO_UPDATE_VALUE_NAME: &str = "DisableRootAutoUpdate";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: &str = "DisallowedCertEncodedCtl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: &str = "DisallowedCertLastSyncTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_LIST_IDENTIFIER: &str = "DisallowedCert_AutoUpdate_1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: &str = "DisallowedCertSyncDeltaTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_CERT_CAB_FILENAME: &str = "disallowedcertstl.cab";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_CERT_CTL_FILENAME: &str = "disallowedcert.stl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_CERT_CTL_FILENAME_A: &str = "disallowedcert.stl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: u32 = 122u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DISALLOWED_FILETIME_PROP_ID: u32 = 104u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_DSS_PARAMETERS {
    pub p: CRYPTOAPI_BLOB,
    pub q: CRYPTOAPI_BLOB,
    pub g: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_DSS_PARAMETERS {}
impl ::core::clone::Clone for CERT_DSS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DSS_R_LEN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_DSS_S_LEN: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_ECC_SIGNATURE {
    pub r: CRYPTOAPI_BLOB,
    pub s: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_ECC_SIGNATURE {}
impl ::core::clone::Clone for CERT_ECC_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_EFSBLOB_VALUE_NAME: &str = "EFSBlob";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_EFS_PROP_ID: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ENABLE_DISALLOWED_CERT_AUTO_UPDATE_VALUE_NAME: &str = "EnableDisallowedCertAutoUpdate";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ENCIPHER_ONLY_KEY_USAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ENCODING_TYPE_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_END_ENTITY_SUBJECT_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ENHKEY_USAGE_PROP_ID: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ENROLLMENT_PROP_ID: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_EXCLUDED_SUBTREE_BIT: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_EXTENDED_ERROR_INFO_PROP_ID: u32 = 30u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_EXTENSION {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub fCritical: super::super::Foundation::BOOL,
    pub Value: CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_EXTENSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_EXTENSIONS {
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_EXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_EXTENSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FILE_HASH_USE_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FILE_STORE_COMMIT_ENABLE_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_FIND_CHAIN_IN_STORE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_FIND_BY_ISSUER_COMPARE_KEY_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_FIND_BY_ISSUER_COMPLEX_CHAIN_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_URL_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_FIND_BY_ISSUER_LOCAL_MACHINE_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_CHAIN_FIND_BY_ISSUER_NO_KEY_FLAG: CERT_FIND_CHAIN_IN_STORE_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_FIND_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_ANY: CERT_FIND_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_CERT_ID: CERT_FIND_FLAGS = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_CTL_USAGE: CERT_FIND_FLAGS = 655360u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_ENHKEY_USAGE: CERT_FIND_FLAGS = 655360u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_EXISTING: CERT_FIND_FLAGS = 851968u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_HASH: CERT_FIND_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_HAS_PRIVATE_KEY: CERT_FIND_FLAGS = 1376256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_ISSUER_ATTR: CERT_FIND_FLAGS = 196612u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_ISSUER_NAME: CERT_FIND_FLAGS = 131076u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_ISSUER_OF: CERT_FIND_FLAGS = 786432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_ISSUER_STR: CERT_FIND_FLAGS = 524292u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_KEY_IDENTIFIER: CERT_FIND_FLAGS = 983040u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_KEY_SPEC: CERT_FIND_FLAGS = 589824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_MD5_HASH: CERT_FIND_FLAGS = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_PROPERTY: CERT_FIND_FLAGS = 327680u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_PUBLIC_KEY: CERT_FIND_FLAGS = 393216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SHA1_HASH: CERT_FIND_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SIGNATURE_HASH: CERT_FIND_FLAGS = 917504u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SUBJECT_ATTR: CERT_FIND_FLAGS = 196615u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SUBJECT_CERT: CERT_FIND_FLAGS = 720896u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SUBJECT_NAME: CERT_FIND_FLAGS = 131079u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SUBJECT_STR: CERT_FIND_FLAGS = 524295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_CROSS_CERT_DIST_POINTS: CERT_FIND_FLAGS = 1114112u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_PUBKEY_MD5_HASH: CERT_FIND_FLAGS = 1179648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SUBJECT_STR_A: CERT_FIND_FLAGS = 458759u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SUBJECT_STR_W: CERT_FIND_FLAGS = 524295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_ISSUER_STR_A: CERT_FIND_FLAGS = 458756u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_ISSUER_STR_W: CERT_FIND_FLAGS = 524292u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_SUBJECT_INFO_ACCESS: CERT_FIND_FLAGS = 1245184u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_HASH_STR: CERT_FIND_FLAGS = 1310720u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_NO_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_OR_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_VALID_ENHKEY_USAGE_FLAG: CERT_FIND_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_OPTIONAL_CTL_USAGE_FLAG: CERT_FIND_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_EXT_ONLY_CTL_USAGE_FLAG: CERT_FIND_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_PROP_ONLY_CTL_USAGE_FLAG: CERT_FIND_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_NO_CTL_USAGE_FLAG: CERT_FIND_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_OR_CTL_USAGE_FLAG: CERT_FIND_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIND_VALID_CTL_USAGE_FLAG: CERT_FIND_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_FIND_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_FIND_ANY: CERT_FIND_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_FIND_SHA1_HASH: CERT_FIND_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_FIND_MD5_HASH: CERT_FIND_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_FIND_USAGE: CERT_FIND_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_FIND_SAME_USAGE_FLAG: CERT_FIND_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_FIND_EXISTING: CERT_FIND_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_FIND_SUBJECT: CERT_FIND_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIRST_RESERVED_PROP_ID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FIRST_USER_PROP_ID: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_FORTEZZA_DATA_PROP {
    pub SerialNumber: [u8; 8],
    pub CertIndex: i32,
    pub CertLabel: [u8; 36],
}
impl ::core::marker::Copy for CERT_FORTEZZA_DATA_PROP {}
impl ::core::clone::Clone for CERT_FORTEZZA_DATA_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FORTEZZA_DATA_PROP_ID: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_FRIENDLY_NAME_PROP_ID: u32 = 11u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_GENERAL_SUBTREE {
    pub Base: CERT_ALT_NAME_ENTRY,
    pub dwMinimum: u32,
    pub fMaximum: super::super::Foundation::BOOL,
    pub dwMaximum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_GENERAL_SUBTREE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_GENERAL_SUBTREE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH: &str = "Software\\Policies\\Microsoft\\SystemCertificates";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_HASHED_URL {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Hash: CRYPTOAPI_BLOB,
    pub pwszUrl: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CERT_HASHED_URL {}
impl ::core::clone::Clone for CERT_HASHED_URL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_HASH_PROP_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_HCRYPTPROV_TRANSFER_PROP_ID: u32 = 100u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_ID {
    pub dwIdChoice: CERT_ID_OPTION,
    pub Anonymous: CERT_ID_0,
}
impl ::core::marker::Copy for CERT_ID {}
impl ::core::clone::Clone for CERT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CERT_ID_0 {
    pub IssuerSerialNumber: CERT_ISSUER_SERIAL_NUMBER,
    pub KeyId: CRYPTOAPI_BLOB,
    pub HashId: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_ID_0 {}
impl ::core::clone::Clone for CERT_ID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_ID_OPTION = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ID_ISSUER_SERIAL_NUMBER: CERT_ID_OPTION = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ID_KEY_IDENTIFIER: CERT_ID_OPTION = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ID_SHA1_HASH: CERT_ID_OPTION = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_IE30_RESERVED_PROP_ID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_IE_DIRTY_FLAGS_REGPATH: &str = "Software\\Microsoft\\Cryptography\\IEDirtyFlags";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_INFO {
    pub dwVersion: u32,
    pub SerialNumber: CRYPTOAPI_BLOB,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Issuer: CRYPTOAPI_BLOB,
    pub NotBefore: super::super::Foundation::FILETIME,
    pub NotAfter: super::super::Foundation::FILETIME,
    pub Subject: CRYPTOAPI_BLOB,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub IssuerUniqueId: CRYPT_BIT_BLOB,
    pub SubjectUniqueId: CRYPT_BIT_BLOB,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_EXTENSION_FLAG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_ISSUER_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_ISSUER_UNIQUE_ID_FLAG: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_NOT_AFTER_FLAG: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_NOT_BEFORE_FLAG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_SERIAL_NUMBER_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_SIGNATURE_ALGORITHM_FLAG: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_SUBJECT_FLAG: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_SUBJECT_PUBLIC_KEY_INFO_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_SUBJECT_UNIQUE_ID_FLAG: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_INFO_VERSION_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ISOLATED_KEY_PROP_ID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: u32 = 94u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_ISSUER_SERIAL_NUMBER {
    pub Issuer: CRYPTOAPI_BLOB,
    pub SerialNumber: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_ISSUER_SERIAL_NUMBER {}
impl ::core::clone::Clone for CERT_ISSUER_SERIAL_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: u32 = 28u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_KEYGEN_REQUEST_INFO {
    pub dwVersion: u32,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub pwszChallengeString: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CERT_KEYGEN_REQUEST_INFO {}
impl ::core::clone::Clone for CERT_KEYGEN_REQUEST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEYGEN_REQUEST_V1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_AGREEMENT_KEY_USAGE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_KEY_ATTRIBUTES_INFO {
    pub KeyId: CRYPTOAPI_BLOB,
    pub IntendedKeyUsage: CRYPT_BIT_BLOB,
    pub pPrivateKeyUsagePeriod: *mut CERT_PRIVATE_KEY_VALIDITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_KEY_ATTRIBUTES_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_KEY_ATTRIBUTES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_CERT_SIGN_KEY_USAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_CLASSIFICATION_PROP_ID: u32 = 120u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_KEY_CONTEXT {
    pub cbSize: u32,
    pub Anonymous: CERT_KEY_CONTEXT_0,
    pub dwKeySpec: u32,
}
impl ::core::marker::Copy for CERT_KEY_CONTEXT {}
impl ::core::clone::Clone for CERT_KEY_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CERT_KEY_CONTEXT_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl ::core::marker::Copy for CERT_KEY_CONTEXT_0 {}
impl ::core::clone::Clone for CERT_KEY_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_CONTEXT_PROP_ID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_ENCIPHERMENT_KEY_USAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_IDENTIFIER_PROP_ID: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_PROV_HANDLE_PROP_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_PROV_INFO_PROP_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_KEY_SPEC = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AT_KEYEXCHANGE: CERT_KEY_SPEC = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AT_SIGNATURE: CERT_KEY_SPEC = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NCRYPT_KEY_SPEC: CERT_KEY_SPEC = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_KEY_SPEC_PROP_ID: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_KEY_USAGE_RESTRICTION_INFO {
    pub cCertPolicyId: u32,
    pub rgCertPolicyId: *mut CERT_POLICY_ID,
    pub RestrictedKeyUsage: CRYPT_BIT_BLOB,
}
impl ::core::marker::Copy for CERT_KEY_USAGE_RESTRICTION_INFO {}
impl ::core::clone::Clone for CERT_KEY_USAGE_RESTRICTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LAST_RESERVED_PROP_ID: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LAST_USER_PROP_ID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LDAP_STORE_AREC_EXCLUSIVE_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LDAP_STORE_OPENED_FLAG: u32 = 262144u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LDAP_STORE_OPENED_PARA {
    pub pvLdapSessionHandle: *mut ::core::ffi::c_void,
    pub pwszLdapUrl: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for CERT_LDAP_STORE_OPENED_PARA {}
impl ::core::clone::Clone for CERT_LDAP_STORE_OPENED_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LDAP_STORE_SIGN_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LDAP_STORE_UNBIND_FLAG: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LOCAL_MACHINE_SYSTEM_STORE_REGPATH: &str = "Software\\Microsoft\\SystemCertificates";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_AUDIO {
    pub LogotypeDetails: CERT_LOGOTYPE_DETAILS,
    pub pLogotypeAudioInfo: *mut CERT_LOGOTYPE_AUDIO_INFO,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_AUDIO {}
impl ::core::clone::Clone for CERT_LOGOTYPE_AUDIO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_AUDIO_INFO {
    pub dwFileSize: u32,
    pub dwPlayTime: u32,
    pub dwChannels: u32,
    pub dwSampleRate: u32,
    pub pwszLanguage: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_AUDIO_INFO {}
impl ::core::clone::Clone for CERT_LOGOTYPE_AUDIO_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_LOGOTYPE_CHOICE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LOGOTYPE_NO_IMAGE_RESOLUTION_CHOICE: CERT_LOGOTYPE_CHOICE = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LOGOTYPE_BITS_IMAGE_RESOLUTION_CHOICE: CERT_LOGOTYPE_CHOICE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LOGOTYPE_TABLE_SIZE_IMAGE_RESOLUTION_CHOICE: CERT_LOGOTYPE_CHOICE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_DATA {
    pub cLogotypeImage: u32,
    pub rgLogotypeImage: *mut CERT_LOGOTYPE_IMAGE,
    pub cLogotypeAudio: u32,
    pub rgLogotypeAudio: *mut CERT_LOGOTYPE_AUDIO,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_DATA {}
impl ::core::clone::Clone for CERT_LOGOTYPE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_DETAILS {
    pub pwszMimeType: ::windows_sys::core::PWSTR,
    pub cHashedUrl: u32,
    pub rgHashedUrl: *mut CERT_HASHED_URL,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_DETAILS {}
impl ::core::clone::Clone for CERT_LOGOTYPE_DETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_EXT_INFO {
    pub cCommunityLogo: u32,
    pub rgCommunityLogo: *mut CERT_LOGOTYPE_INFO,
    pub pIssuerLogo: *mut CERT_LOGOTYPE_INFO,
    pub pSubjectLogo: *mut CERT_LOGOTYPE_INFO,
    pub cOtherLogo: u32,
    pub rgOtherLogo: *mut CERT_OTHER_LOGOTYPE_INFO,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_EXT_INFO {}
impl ::core::clone::Clone for CERT_LOGOTYPE_EXT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_IMAGE {
    pub LogotypeDetails: CERT_LOGOTYPE_DETAILS,
    pub pLogotypeImageInfo: *mut CERT_LOGOTYPE_IMAGE_INFO,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_IMAGE {}
impl ::core::clone::Clone for CERT_LOGOTYPE_IMAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_IMAGE_INFO {
    pub dwLogotypeImageInfoChoice: CERT_LOGOTYPE_IMAGE_INFO_TYPE,
    pub dwFileSize: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwLogotypeImageResolutionChoice: CERT_LOGOTYPE_CHOICE,
    pub Anonymous: CERT_LOGOTYPE_IMAGE_INFO_0,
    pub pwszLanguage: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_IMAGE_INFO {}
impl ::core::clone::Clone for CERT_LOGOTYPE_IMAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CERT_LOGOTYPE_IMAGE_INFO_0 {
    pub dwNumBits: u32,
    pub dwTableSize: u32,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_IMAGE_INFO_0 {}
impl ::core::clone::Clone for CERT_LOGOTYPE_IMAGE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_LOGOTYPE_IMAGE_INFO_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LOGOTYPE_GRAY_SCALE_IMAGE_INFO_CHOICE: CERT_LOGOTYPE_IMAGE_INFO_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LOGOTYPE_COLOR_IMAGE_INFO_CHOICE: CERT_LOGOTYPE_IMAGE_INFO_TYPE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_INFO {
    pub dwLogotypeInfoChoice: CERT_LOGOTYPE_OPTION,
    pub Anonymous: CERT_LOGOTYPE_INFO_0,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_INFO {}
impl ::core::clone::Clone for CERT_LOGOTYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CERT_LOGOTYPE_INFO_0 {
    pub pLogotypeDirectInfo: *mut CERT_LOGOTYPE_DATA,
    pub pLogotypeIndirectInfo: *mut CERT_LOGOTYPE_REFERENCE,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_INFO_0 {}
impl ::core::clone::Clone for CERT_LOGOTYPE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_LOGOTYPE_OPTION = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LOGOTYPE_DIRECT_INFO_CHOICE: CERT_LOGOTYPE_OPTION = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_LOGOTYPE_INDIRECT_INFO_CHOICE: CERT_LOGOTYPE_OPTION = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_LOGOTYPE_REFERENCE {
    pub cHashedUrl: u32,
    pub rgHashedUrl: *mut CERT_HASHED_URL,
}
impl ::core::marker::Copy for CERT_LOGOTYPE_REFERENCE {}
impl ::core::clone::Clone for CERT_LOGOTYPE_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_MD5_HASH_PROP_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_ATTR_TYPE: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_NAME_CONSTRAINTS_INFO {
    pub cPermittedSubtree: u32,
    pub rgPermittedSubtree: *mut CERT_GENERAL_SUBTREE,
    pub cExcludedSubtree: u32,
    pub rgExcludedSubtree: *mut CERT_GENERAL_SUBTREE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_NAME_CONSTRAINTS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_NAME_CONSTRAINTS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_DISABLE_IE4_UTF8_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_DNS_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_EMAIL_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_FRIENDLY_DISPLAY_TYPE: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_NAME_INFO {
    pub cRDN: u32,
    pub rgRDN: *mut CERT_RDN,
}
impl ::core::marker::Copy for CERT_NAME_INFO {}
impl ::core::clone::Clone for CERT_NAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_ISSUER_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_RDN_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_SEARCH_ALL_NAMES_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_SIMPLE_DISPLAY_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_COMMA_FLAG: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_CRLF_FLAG: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_FORWARD_FLAG: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_NO_PLUS_FLAG: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_NO_QUOTING_FLAG: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_REVERSE_FLAG: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_STR_SEMICOLON_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_UPN_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NAME_URL_TYPE: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_NAME_VALUE {
    pub dwValueType: u32,
    pub Value: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_NAME_VALUE {}
impl ::core::clone::Clone for CERT_NAME_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NEW_KEY_PROP_ID: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NEXT_UPDATE_LOCATION_PROP_ID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NON_REPUDIATION_KEY_USAGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NOT_BEFORE_FILETIME_PROP_ID: u32 = 126u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_OCM_SUBCOMPONENTS_LOCAL_MACHINE_REGPATH: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Setup\\OC Manager\\Subcomponents";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_OCM_SUBCOMPONENTS_ROOT_AUTO_UPDATE_VALUE_NAME: &str = "RootAutoUpdate";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_OCSP_CACHE_PREFIX_PROP_ID: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_OCSP_MUST_STAPLE_PROP_ID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_OCSP_RESPONSE_PROP_ID: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_OFFLINE_CRL_SIGN_KEY_USAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_OPEN_STORE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_BACKUP_RESTORE_FLAG: CERT_OPEN_STORE_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CREATE_NEW_FLAG: CERT_OPEN_STORE_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG: CERT_OPEN_STORE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_DELETE_FLAG: CERT_OPEN_STORE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_ENUM_ARCHIVED_FLAG: CERT_OPEN_STORE_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_MAXIMUM_ALLOWED_FLAG: CERT_OPEN_STORE_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_NO_CRYPT_RELEASE_FLAG: CERT_OPEN_STORE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_OPEN_EXISTING_FLAG: CERT_OPEN_STORE_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_READONLY_FLAG: CERT_OPEN_STORE_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SET_LOCALIZED_NAME_FLAG: CERT_OPEN_STORE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SHARE_CONTEXT_FLAG: CERT_OPEN_STORE_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_UPDATE_KEYID_FLAG: CERT_OPEN_STORE_FLAGS = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_OR_CRL_BLOB {
    pub dwChoice: u32,
    pub cbEncoded: u32,
    pub pbEncoded: *mut u8,
}
impl ::core::marker::Copy for CERT_OR_CRL_BLOB {}
impl ::core::clone::Clone for CERT_OR_CRL_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_OR_CRL_BUNDLE {
    pub cItem: u32,
    pub rgItem: *mut CERT_OR_CRL_BLOB,
}
impl ::core::marker::Copy for CERT_OR_CRL_BUNDLE {}
impl ::core::clone::Clone for CERT_OR_CRL_BUNDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_OTHER_LOGOTYPE_INFO {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub LogotypeInfo: CERT_LOGOTYPE_INFO,
}
impl ::core::marker::Copy for CERT_OTHER_LOGOTYPE_INFO {}
impl ::core::clone::Clone for CERT_OTHER_LOGOTYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_OTHER_NAME {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub Value: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_OTHER_NAME {}
impl ::core::clone::Clone for CERT_OTHER_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_PAIR {
    pub Forward: CRYPTOAPI_BLOB,
    pub Reverse: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_PAIR {}
impl ::core::clone::Clone for CERT_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_ADD_ENABLE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_AUTH_ROOT_NAME: &str = ".AuthRoot";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_DEFAULT_NAME: &str = ".Default";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_DS_USER_CERTIFICATE_NAME: &str = ".UserCertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_ENTERPRISE_NAME: &str = ".Enterprise";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_GROUP_POLICY_NAME: &str = ".GroupPolicy";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_PHYSICAL_STORE_INFO {
    pub cbSize: u32,
    pub pszOpenStoreProvider: ::windows_sys::core::PSTR,
    pub dwOpenEncodingType: u32,
    pub dwOpenFlags: u32,
    pub OpenParameters: CRYPTOAPI_BLOB,
    pub dwFlags: u32,
    pub dwPriority: u32,
}
impl ::core::marker::Copy for CERT_PHYSICAL_STORE_INFO {}
impl ::core::clone::Clone for CERT_PHYSICAL_STORE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_INSERT_COMPUTER_NAME_ENABLE_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_LOCAL_MACHINE_GROUP_POLICY_NAME: &str = ".LocalMachineGroupPolicy";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_LOCAL_MACHINE_NAME: &str = ".LocalMachine";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_OPEN_DISABLE_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_PREDEFINED_ENUM_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_REMOTE_OPEN_DISABLE_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PHYSICAL_STORE_SMART_CARD_NAME: &str = ".SmartCard";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PIN_RULES_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: &str = "PinRulesEncodedCtl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PIN_RULES_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: &str = "PinRulesLastSyncTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PIN_RULES_AUTO_UPDATE_LIST_IDENTIFIER: &str = "PinRules_AutoUpdate_1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PIN_RULES_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: &str = "PinRulesSyncDeltaTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PIN_RULES_CAB_FILENAME: &str = "pinrulesstl.cab";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PIN_RULES_CTL_FILENAME: &str = "pinrules.stl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PIN_RULES_CTL_FILENAME_A: &str = "pinrules.stl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PIN_SHA256_HASH_PROP_ID: u32 = 124u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICIES_INFO {
    pub cPolicyInfo: u32,
    pub rgPolicyInfo: *mut CERT_POLICY_INFO,
}
impl ::core::marker::Copy for CERT_POLICIES_INFO {}
impl ::core::clone::Clone for CERT_POLICIES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICY95_QUALIFIER1 {
    pub pszPracticesReference: ::windows_sys::core::PWSTR,
    pub pszNoticeIdentifier: ::windows_sys::core::PSTR,
    pub pszNSINoticeIdentifier: ::windows_sys::core::PSTR,
    pub cCPSURLs: u32,
    pub rgCPSURLs: *mut CPS_URLS,
}
impl ::core::marker::Copy for CERT_POLICY95_QUALIFIER1 {}
impl ::core::clone::Clone for CERT_POLICY95_QUALIFIER1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_CONSTRAINTS_INFO {
    pub fRequireExplicitPolicy: super::super::Foundation::BOOL,
    pub dwRequireExplicitPolicySkipCerts: u32,
    pub fInhibitPolicyMapping: super::super::Foundation::BOOL,
    pub dwInhibitPolicyMappingSkipCerts: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_POLICY_CONSTRAINTS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_POLICY_CONSTRAINTS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICY_ID {
    pub cCertPolicyElementId: u32,
    pub rgpszCertPolicyElementId: *mut ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CERT_POLICY_ID {}
impl ::core::clone::Clone for CERT_POLICY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICY_INFO {
    pub pszPolicyIdentifier: ::windows_sys::core::PSTR,
    pub cPolicyQualifier: u32,
    pub rgPolicyQualifier: *mut CERT_POLICY_QUALIFIER_INFO,
}
impl ::core::marker::Copy for CERT_POLICY_INFO {}
impl ::core::clone::Clone for CERT_POLICY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICY_MAPPING {
    pub pszIssuerDomainPolicy: ::windows_sys::core::PSTR,
    pub pszSubjectDomainPolicy: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CERT_POLICY_MAPPING {}
impl ::core::clone::Clone for CERT_POLICY_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICY_MAPPINGS_INFO {
    pub cPolicyMapping: u32,
    pub rgPolicyMapping: *mut CERT_POLICY_MAPPING,
}
impl ::core::marker::Copy for CERT_POLICY_MAPPINGS_INFO {}
impl ::core::clone::Clone for CERT_POLICY_MAPPINGS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICY_QUALIFIER_INFO {
    pub pszPolicyQualifierId: ::windows_sys::core::PSTR,
    pub Qualifier: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_POLICY_QUALIFIER_INFO {}
impl ::core::clone::Clone for CERT_POLICY_QUALIFIER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    pub pszOrganization: ::windows_sys::core::PSTR,
    pub cNoticeNumbers: u32,
    pub rgNoticeNumbers: *mut i32,
}
impl ::core::marker::Copy for CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {}
impl ::core::clone::Clone for CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_POLICY_QUALIFIER_USER_NOTICE {
    pub pNoticeReference: *mut CERT_POLICY_QUALIFIER_NOTICE_REFERENCE,
    pub pszDisplayText: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CERT_POLICY_QUALIFIER_USER_NOTICE {}
impl ::core::clone::Clone for CERT_POLICY_QUALIFIER_USER_NOTICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_PRIVATE_KEY_VALIDITY {
    pub NotBefore: super::super::Foundation::FILETIME,
    pub NotAfter: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_PRIVATE_KEY_VALIDITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_PRIVATE_KEY_VALIDITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_DISABLE_CURRENT_USER_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_DISABLE_LM_AUTH_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_DISABLE_NOT_DEFINED_NAME_CONSTRAINT_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_DISABLE_NT_AUTH_REQUIRED_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_DISABLE_PEER_TRUST: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_FLAGS_VALUE_NAME: &str = "Flags";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_INHIBIT_ADD_AT_INIT_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_INHIBIT_PURGE_LM_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_ONLY_LM_GPT_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME: &str = "PeerUsages";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME_A: &str = "PeerUsages";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PUBKEY_ALG_PARA_PROP_ID: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PUBKEY_HASH_RESERVED_PROP_ID: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_PUBLIC_KEY_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub PublicKey: CRYPT_BIT_BLOB,
}
impl ::core::marker::Copy for CERT_PUBLIC_KEY_INFO {}
impl ::core::clone::Clone for CERT_PUBLIC_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_PVK_FILE_PROP_ID: u32 = 12u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_QC_STATEMENT {
    pub pszStatementId: ::windows_sys::core::PSTR,
    pub StatementInfo: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_QC_STATEMENT {}
impl ::core::clone::Clone for CERT_QC_STATEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_QC_STATEMENTS_EXT_INFO {
    pub cStatement: u32,
    pub rgStatement: *mut CERT_QC_STATEMENT,
}
impl ::core::marker::Copy for CERT_QC_STATEMENTS_EXT_INFO {}
impl ::core::clone::Clone for CERT_QC_STATEMENTS_EXT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_QUERY_CONTENT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_CERT: CERT_QUERY_CONTENT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_CTL: CERT_QUERY_CONTENT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_CRL: CERT_QUERY_CONTENT_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_SERIALIZED_STORE: CERT_QUERY_CONTENT_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_SERIALIZED_CERT: CERT_QUERY_CONTENT_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_SERIALIZED_CTL: CERT_QUERY_CONTENT_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_SERIALIZED_CRL: CERT_QUERY_CONTENT_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_PKCS7_SIGNED: CERT_QUERY_CONTENT_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_PKCS7_UNSIGNED: CERT_QUERY_CONTENT_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED: CERT_QUERY_CONTENT_TYPE = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_PKCS10: CERT_QUERY_CONTENT_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_PFX: CERT_QUERY_CONTENT_TYPE = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_CERT_PAIR: CERT_QUERY_CONTENT_TYPE = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_PFX_AND_LOAD: CERT_QUERY_CONTENT_TYPE = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_QUERY_CONTENT_TYPE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_CERT: CERT_QUERY_CONTENT_TYPE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_CTL: CERT_QUERY_CONTENT_TYPE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_CRL: CERT_QUERY_CONTENT_TYPE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE: CERT_QUERY_CONTENT_TYPE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT: CERT_QUERY_CONTENT_TYPE_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CTL: CERT_QUERY_CONTENT_TYPE_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CRL: CERT_QUERY_CONTENT_TYPE_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED: CERT_QUERY_CONTENT_TYPE_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED: CERT_QUERY_CONTENT_TYPE_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED_EMBED: CERT_QUERY_CONTENT_TYPE_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_PKCS10: CERT_QUERY_CONTENT_TYPE_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_PFX: CERT_QUERY_CONTENT_TYPE_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_CERT_PAIR: CERT_QUERY_CONTENT_TYPE_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_PFX_AND_LOAD: CERT_QUERY_CONTENT_TYPE_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_ALL: CERT_QUERY_CONTENT_TYPE_FLAGS = 16382u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_CONTENT_FLAG_ALL_ISSUER_CERT: CERT_QUERY_CONTENT_TYPE_FLAGS = 818u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_QUERY_ENCODING_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ASN_ENCODING: CERT_QUERY_ENCODING_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_7_ASN_ENCODING: CERT_QUERY_ENCODING_TYPE = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_QUERY_FORMAT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_FORMAT_BINARY: CERT_QUERY_FORMAT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_FORMAT_BASE64_ENCODED: CERT_QUERY_FORMAT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_FORMAT_ASN_ASCII_HEX_ENCODED: CERT_QUERY_FORMAT_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_QUERY_FORMAT_TYPE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_FORMAT_FLAG_BINARY: CERT_QUERY_FORMAT_TYPE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_FORMAT_FLAG_BASE64_ENCODED: CERT_QUERY_FORMAT_TYPE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_FORMAT_FLAG_ASN_ASCII_HEX_ENCODED: CERT_QUERY_FORMAT_TYPE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_FORMAT_FLAG_ALL: CERT_QUERY_FORMAT_TYPE_FLAGS = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_QUERY_OBJECT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_OBJECT_FILE: CERT_QUERY_OBJECT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_QUERY_OBJECT_BLOB: CERT_QUERY_OBJECT_TYPE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_RDN {
    pub cRDNAttr: u32,
    pub rgRDNAttr: *mut CERT_RDN_ATTR,
}
impl ::core::marker::Copy for CERT_RDN {}
impl ::core::clone::Clone for CERT_RDN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_RDN_ATTR {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub dwValueType: CERT_RDN_ATTR_VALUE_TYPE,
    pub Value: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CERT_RDN_ATTR {}
impl ::core::clone::Clone for CERT_RDN_ATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_RDN_ATTR_VALUE_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_ANY_TYPE: CERT_RDN_ATTR_VALUE_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_NUMERIC_STRING: CERT_RDN_ATTR_VALUE_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_PRINTABLE_STRING: CERT_RDN_ATTR_VALUE_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_T61_STRING: CERT_RDN_ATTR_VALUE_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_VIDEOTEX_STRING: CERT_RDN_ATTR_VALUE_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_IA5_STRING: CERT_RDN_ATTR_VALUE_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_GRAPHIC_STRING: CERT_RDN_ATTR_VALUE_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_ISO646_STRING: CERT_RDN_ATTR_VALUE_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_GENERAL_STRING: CERT_RDN_ATTR_VALUE_TYPE = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_INT4_STRING: CERT_RDN_ATTR_VALUE_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_UNICODE_STRING: CERT_RDN_ATTR_VALUE_TYPE = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_BMP_STRING: CERT_RDN_ATTR_VALUE_TYPE = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_ENCODED_BLOB: CERT_RDN_ATTR_VALUE_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_OCTET_STRING: CERT_RDN_ATTR_VALUE_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_TELETEX_STRING: CERT_RDN_ATTR_VALUE_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_UNIVERSAL_STRING: CERT_RDN_ATTR_VALUE_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_UTF8_STRING: CERT_RDN_ATTR_VALUE_TYPE = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_VISIBLE_STRING: CERT_RDN_ATTR_VALUE_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_DISABLE_CHECK_TYPE_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_DISABLE_IE4_UTF8_FLAG: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_ENABLE_PUNYCODE_FLAG: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_ENABLE_T61_UNICODE_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_ENABLE_UTF8_UNICODE_FLAG: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_FLAGS_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_FORCE_UTF8_UNICODE_FLAG: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RDN_TYPE_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REGISTRY_STORE_CLIENT_GPT_FLAG: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub struct CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    pub hKeyBase: super::super::System::Registry::HKEY,
    pub pwszRegPath: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for CERT_REGISTRY_STORE_CLIENT_GPT_PARA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REGISTRY_STORE_EXTERNAL_FLAG: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REGISTRY_STORE_LM_GPT_FLAG: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REGISTRY_STORE_MY_IE_DIRTY_FLAG: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REGISTRY_STORE_REMOTE_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REGISTRY_STORE_ROAMING_FLAG: u32 = 262144u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub struct CERT_REGISTRY_STORE_ROAMING_PARA {
    pub hKey: super::super::System::Registry::HKEY,
    pub pwszStoreDirectory: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for CERT_REGISTRY_STORE_ROAMING_PARA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for CERT_REGISTRY_STORE_ROAMING_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REGISTRY_STORE_SERIALIZED_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RENEWAL_PROP_ID: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_REQUEST_INFO {
    pub dwVersion: u32,
    pub Subject: CRYPTOAPI_BLOB,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub cAttribute: u32,
    pub rgAttribute: *mut CRYPT_ATTRIBUTE,
}
impl ::core::marker::Copy for CERT_REQUEST_INFO {}
impl ::core::clone::Clone for CERT_REQUEST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REQUEST_ORIGINATOR_PROP_ID: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_REQUEST_V1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RETRIEVE_BIOMETRIC_PREDEFINED_BASE_TYPE: ::windows_sys::core::PCSTR = 1000i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RETRIEVE_COMMUNITY_LOGO: ::windows_sys::core::PCSTR = 3i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RETRIEVE_ISSUER_LOGO: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RETRIEVE_SUBJECT_LOGO: ::windows_sys::core::PCSTR = 2i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RETR_BEHAVIOR_FILE_VALUE_NAME: &str = "AllowFileUrlScheme";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RETR_BEHAVIOR_INET_AUTH_VALUE_NAME: &str = "EnableInetUnknownAuth";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RETR_BEHAVIOR_INET_STATUS_VALUE_NAME: &str = "EnableInetLocal";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RETR_BEHAVIOR_LDAP_VALUE_NAME: &str = "DisableLDAPSignAndEncrypt";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_CHAIN_PARA {
    pub cbSize: u32,
    pub hChainEngine: HCERTCHAINENGINE,
    pub hAdditionalStore: HCERTSTORE,
    pub dwChainFlags: u32,
    pub dwUrlRetrievalTimeout: u32,
    pub pftCurrentTime: *mut super::super::Foundation::FILETIME,
    pub pftCacheResync: *mut super::super::Foundation::FILETIME,
    pub cbMaxUrlRetrievalByteCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_REVOCATION_CHAIN_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_REVOCATION_CHAIN_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_CRL_INFO {
    pub cbSize: u32,
    pub pBaseCrlContext: *mut CRL_CONTEXT,
    pub pDeltaCrlContext: *mut CRL_CONTEXT,
    pub pCrlEntry: *mut CRL_ENTRY,
    pub fDeltaCrlEntry: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_REVOCATION_CRL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_REVOCATION_CRL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_INFO {
    pub cbSize: u32,
    pub dwRevocationResult: u32,
    pub pszRevocationOid: ::windows_sys::core::PCSTR,
    pub pvOidSpecificInfo: *mut ::core::ffi::c_void,
    pub fHasFreshnessTime: super::super::Foundation::BOOL,
    pub dwFreshnessTime: u32,
    pub pCrlInfo: *mut CERT_REVOCATION_CRL_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_REVOCATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_REVOCATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_PARA {
    pub cbSize: u32,
    pub pIssuerCert: *const CERT_CONTEXT,
    pub cCertStore: u32,
    pub rgCertStore: *mut HCERTSTORE,
    pub hCrlStore: HCERTSTORE,
    pub pftTimeToUse: *mut super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_REVOCATION_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_REVOCATION_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_STATUS {
    pub cbSize: u32,
    pub dwIndex: u32,
    pub dwError: u32,
    pub dwReason: CERT_REVOCATION_STATUS_REASON,
    pub fHasFreshnessTime: super::super::Foundation::BOOL,
    pub dwFreshnessTime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_REVOCATION_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_REVOCATION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_REVOCATION_STATUS_REASON = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_UNSPECIFIED: CERT_REVOCATION_STATUS_REASON = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_KEY_COMPROMISE: CERT_REVOCATION_STATUS_REASON = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_CA_COMPROMISE: CERT_REVOCATION_STATUS_REASON = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_AFFILIATION_CHANGED: CERT_REVOCATION_STATUS_REASON = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_SUPERSEDED: CERT_REVOCATION_STATUS_REASON = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_CESSATION_OF_OPERATION: CERT_REVOCATION_STATUS_REASON = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_CERTIFICATE_HOLD: CERT_REVOCATION_STATUS_REASON = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_REMOVE_FROM_CRL: CERT_REVOCATION_STATUS_REASON = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_ROOT_PROGRAM_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ROOT_PROGRAM_FLAG_LSC: CERT_ROOT_PROGRAM_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ROOT_PROGRAM_FLAG_ORG: CERT_ROOT_PROGRAM_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ROOT_PROGRAM_FLAG_SUBJECT_LOGO: CERT_ROOT_PROGRAM_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ROOT_PROGRAM_FLAG_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ROOT_PROGRAM_FLAG_OU: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_RSA_PUBLIC_KEY_OBJID: &str = "1.2.840.113549.1.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCARD_PIN_ID_PROP_ID: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCARD_PIN_INFO_PROP_ID: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_CA_CERT_PROP_ID: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_FLAGS_PROP_ID: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_GUID_PROP_ID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_NONCE_PROP_ID: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_SERVER_CERTS_PROP_ID: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SCEP_SIGNER_CERT_PROP_ID: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_ALLOW_DUPLICATES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_ALLOW_EXPIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_FRIENDLYNAME: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_ISSUER_DISPLAYNAME: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_THUMBPRINT: u32 = 14u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SELECT_CHAIN_PARA {
    pub hChainEngine: HCERTCHAINENGINE,
    pub pTime: *mut super::super::Foundation::FILETIME,
    pub hAdditionalStore: HCERTSTORE,
    pub pChainPara: *mut CERT_CHAIN_PARA,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_SELECT_CHAIN_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_SELECT_CHAIN_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_SELECT_CRITERIA {
    pub dwType: CERT_SELECT_CRITERIA_TYPE,
    pub cPara: u32,
    pub ppPara: *mut *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CERT_SELECT_CRITERIA {}
impl ::core::clone::Clone for CERT_SELECT_CRITERIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_SELECT_CRITERIA_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_ENHKEY_USAGE: CERT_SELECT_CRITERIA_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_KEY_USAGE: CERT_SELECT_CRITERIA_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_POLICY_OID: CERT_SELECT_CRITERIA_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_PROV_NAME: CERT_SELECT_CRITERIA_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_EXTENSION: CERT_SELECT_CRITERIA_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_SUBJECT_HOST_NAME: CERT_SELECT_CRITERIA_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_ISSUER_ATTR: CERT_SELECT_CRITERIA_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_SUBJECT_ATTR: CERT_SELECT_CRITERIA_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_ISSUER_NAME: CERT_SELECT_CRITERIA_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_PUBLIC_KEY: CERT_SELECT_CRITERIA_TYPE = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_BY_TLS_SIGNATURES: CERT_SELECT_CRITERIA_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_DISALLOW_SELFSIGNED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_HARDWARE_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_HAS_KEY_FOR_KEY_EXCHANGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_HAS_KEY_FOR_SIGNATURE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_HAS_PRIVATE_KEY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_IGNORE_AUTOSELECT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_MAX_PARA: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SELECT_TRUSTED_ROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SERIAL_CHAIN_PROP_ID: u32 = 119u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SERVER_OCSP_RESPONSE_ASYNC_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    pub cbSize: u32,
    pub pbEncodedOcspResponse: *mut u8,
    pub cbEncodedOcspResponse: u32,
}
impl ::core::marker::Copy for CERT_SERVER_OCSP_RESPONSE_CONTEXT {}
impl ::core::clone::Clone for CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pcbUsedSize: *mut u32,
    pub pwszOcspDirectory: ::windows_sys::core::PWSTR,
    pub pfnUpdateCallback: PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK,
    pub pvUpdateCallbackArg: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_READ_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_WRITE_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SET_PROPERTY_INHIBIT_PERSIST_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SHA1_HASH_PROP_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SHA256_HASH_PROP_ID: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SIGNATURE_HASH_PROP_ID: u32 = 15u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_SIGNED_CONTENT_INFO {
    pub ToBeSigned: CRYPTOAPI_BLOB,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Signature: CRYPT_BIT_BLOB,
}
impl ::core::marker::Copy for CERT_SIGNED_CONTENT_INFO {}
impl ::core::clone::Clone for CERT_SIGNED_CONTENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SIGN_HASH_CNG_ALG_PROP_ID: u32 = 89u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SIMPLE_CHAIN {
    pub cbSize: u32,
    pub TrustStatus: CERT_TRUST_STATUS,
    pub cElement: u32,
    pub rgpElement: *mut *mut CERT_CHAIN_ELEMENT,
    pub pTrustListInfo: *mut CERT_TRUST_LIST_INFO,
    pub fHasRevocationFreshnessTime: super::super::Foundation::BOOL,
    pub dwRevocationFreshnessTime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_SIMPLE_CHAIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_SIMPLE_CHAIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SMART_CARD_DATA_PROP_ID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SMART_CARD_READER_PROP_ID: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SMART_CARD_ROOT_INFO_PROP_ID: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SOURCE_LOCATION_PROP_ID: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SOURCE_URL_PROP_ID: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SRV_OCSP_RESP_MAX_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: &str = "SrvOcspRespMaxBeforeNextUpdateSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SRV_OCSP_RESP_MAX_SYNC_CERT_FILE_SECONDS_VALUE_NAME: &str = "SrvOcspRespMaxSyncCertFileSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SRV_OCSP_RESP_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME: &str = "SrvOcspRespMinAfterNextUpdateSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SRV_OCSP_RESP_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: &str = "SrvOcspRespMinBeforeNextUpdateSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_DEFAULT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_VALUE_NAME: &str = "SrvOcspRespMinSyncCertFileSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SRV_OCSP_RESP_MIN_VALIDITY_SECONDS_VALUE_NAME: &str = "SrvOcspRespMinValiditySeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SRV_OCSP_RESP_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: &str = "SrvOcspRespUrlRetrievalTimeoutMilliseconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_ADD_ALWAYS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_ADD_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_ADD_NEWER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_ADD_REPLACE_EXISTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_ADD_USE_EXISTING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_BASE_CRL_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CERTIFICATE_CONTEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CRL_CONTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTL_CONTEXT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTRL_AUTO_RESYNC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTRL_CANCEL_NOTIFY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTRL_COMMIT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTRL_NOTIFY_CHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_CTRL_RESYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_DELTA_CRL_FLAG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_LOCALIZED_NAME_PROP_ID: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_MANIFOLD_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_NO_CRL_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_NO_ISSUER_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_CLOSE_FUNC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_COLLECTION: ::windows_sys::core::PCSTR = 11i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_CONTROL_FUNC: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_DELETE_CERT_FUNC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_DELETE_CRL_FUNC: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_DELETE_CTL_FUNC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FILE: ::windows_sys::core::PCSTR = 3i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FILENAME: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FILENAME_A: ::windows_sys::core::PCSTR = 7i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FILENAME_W: ::windows_sys::core::PCSTR = 8i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FIND_CERT_FUNC: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FIND_CRL_FUNC: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FIND_CTL_FUNC: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_STORE_PROV_FIND_INFO {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub dwFindFlags: u32,
    pub dwFindType: u32,
    pub pvFindPara: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for CERT_STORE_PROV_FIND_INFO {}
impl ::core::clone::Clone for CERT_STORE_PROV_FIND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_STORE_PROV_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_EXTERNAL_FLAG: CERT_STORE_PROV_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_DELETED_FLAG: CERT_STORE_PROV_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_NO_PERSIST_FLAG: CERT_STORE_PROV_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SYSTEM_STORE_FLAG: CERT_STORE_PROV_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_LM_SYSTEM_STORE_FLAG: CERT_STORE_PROV_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FREE_FIND_CERT_FUNC: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FREE_FIND_CRL_FUNC: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_FREE_FIND_CTL_FUNC: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_GET_CERT_PROPERTY_FUNC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_GET_CRL_PROPERTY_FUNC: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_GET_CTL_PROPERTY_FUNC: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_GP_SYSTEM_STORE_FLAG: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_STORE_PROV_INFO {
    pub cbSize: u32,
    pub cStoreProvFunc: u32,
    pub rgpvStoreProvFunc: *mut *mut ::core::ffi::c_void,
    pub hStoreProv: HCERTSTOREPROV,
    pub dwStoreProvFlags: CERT_STORE_PROV_FLAGS,
    pub hStoreProvFuncAddr2: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CERT_STORE_PROV_INFO {}
impl ::core::clone::Clone for CERT_STORE_PROV_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_LDAP: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_LDAP_W: ::windows_sys::core::PCSTR = 16i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_MEMORY: ::windows_sys::core::PCSTR = 2i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_MSG: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_PHYSICAL: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_PHYSICAL_W: ::windows_sys::core::PCSTR = 14i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_PKCS12: ::windows_sys::core::PCSTR = 17i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_PKCS7: ::windows_sys::core::PCSTR = 5i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_READ_CERT_FUNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_READ_CRL_FUNC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_READ_CTL_FUNC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_REG: ::windows_sys::core::PCSTR = 4i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SERIALIZED: ::windows_sys::core::PCSTR = 6i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SET_CERT_PROPERTY_FUNC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SET_CRL_PROPERTY_FUNC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SET_CTL_PROPERTY_FUNC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SHARED_USER_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SMART_CARD: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SMART_CARD_W: ::windows_sys::core::PCSTR = 15i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SYSTEM: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SYSTEM_A: ::windows_sys::core::PCSTR = 9i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SYSTEM_REGISTRY: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SYSTEM_REGISTRY_A: ::windows_sys::core::PCSTR = 12i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SYSTEM_REGISTRY_W: ::windows_sys::core::PCSTR = 13i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_SYSTEM_W: ::windows_sys::core::PCSTR = 10i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_WRITE_ADD_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_WRITE_CERT_FUNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_WRITE_CRL_FUNC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_PROV_WRITE_CTL_FUNC: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_REVOCATION_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_STORE_SAVE_AS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SAVE_AS_PKCS7: CERT_STORE_SAVE_AS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SAVE_AS_STORE: CERT_STORE_SAVE_AS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SAVE_AS_PKCS12: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_STORE_SAVE_TO = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SAVE_TO_FILE: CERT_STORE_SAVE_TO = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SAVE_TO_FILENAME: CERT_STORE_SAVE_TO = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SAVE_TO_FILENAME_A: CERT_STORE_SAVE_TO = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SAVE_TO_FILENAME_W: CERT_STORE_SAVE_TO = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SAVE_TO_MEMORY: CERT_STORE_SAVE_TO = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SHARE_STORE_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_SIGNATURE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_TIME_VALIDITY_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STORE_UNSAFE_PHYSICAL_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_STRING_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SIMPLE_NAME_STR: CERT_STRING_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_OID_NAME_STR: CERT_STRING_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_X500_NAME_STR: CERT_STRING_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STRONG_SIGN_ECDSA_ALGORITHM: &str = "ECDSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_STRONG_SIGN_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STRONG_SIGN_ENABLE_CRL_CHECK: CERT_STRONG_SIGN_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STRONG_SIGN_ENABLE_OCSP_CHECK: CERT_STRONG_SIGN_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STRONG_SIGN_OID_INFO_CHOICE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_STRONG_SIGN_PARA {
    pub cbSize: u32,
    pub dwInfoChoice: u32,
    pub Anonymous: CERT_STRONG_SIGN_PARA_0,
}
impl ::core::marker::Copy for CERT_STRONG_SIGN_PARA {}
impl ::core::clone::Clone for CERT_STRONG_SIGN_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CERT_STRONG_SIGN_PARA_0 {
    pub pvInfo: *mut ::core::ffi::c_void,
    pub pSerializedInfo: *mut CERT_STRONG_SIGN_SERIALIZED_INFO,
    pub pszOID: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CERT_STRONG_SIGN_PARA_0 {}
impl ::core::clone::Clone for CERT_STRONG_SIGN_PARA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_STRONG_SIGN_SERIALIZED_INFO {
    pub dwFlags: CERT_STRONG_SIGN_FLAGS,
    pub pwszCNGSignHashAlgids: ::windows_sys::core::PWSTR,
    pub pwszCNGPubKeyMinBitLengths: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CERT_STRONG_SIGN_SERIALIZED_INFO {}
impl ::core::clone::Clone for CERT_STRONG_SIGN_SERIALIZED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SUBJECT_DISABLE_CRL_PROP_ID: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SUBJECT_INFO_ACCESS_PROP_ID: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: u32 = 92u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_SUPPORTED_ALGORITHM_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub IntendedKeyUsage: CRYPT_BIT_BLOB,
    pub IntendedCertPolicies: CERT_POLICIES_INFO,
}
impl ::core::marker::Copy for CERT_SUPPORTED_ALGORITHM_INFO {}
impl ::core::clone::Clone for CERT_SUPPORTED_ALGORITHM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_CURRENT_SERVICE_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY_ID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_CURRENT_USER_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_DEFER_READ_FLAG: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CERT_SYSTEM_STORE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_LOCATION_MASK: CERT_SYSTEM_STORE_FLAGS = 16711680u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_RELOCATE_FLAG: CERT_SYSTEM_STORE_FLAGS = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_SYSTEM_STORE_INFO {
    pub cbSize: u32,
}
impl ::core::marker::Copy for CERT_SYSTEM_STORE_INFO {}
impl ::core::clone::Clone for CERT_SYSTEM_STORE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE_ID: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY_ID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_WCOS_ID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_LOCATION_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_MASK: u32 = 4294901760u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub struct CERT_SYSTEM_STORE_RELOCATE_PARA {
    pub Anonymous1: CERT_SYSTEM_STORE_RELOCATE_PARA_0,
    pub Anonymous2: CERT_SYSTEM_STORE_RELOCATE_PARA_1,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for CERT_SYSTEM_STORE_RELOCATE_PARA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for CERT_SYSTEM_STORE_RELOCATE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub union CERT_SYSTEM_STORE_RELOCATE_PARA_0 {
    pub hKeyBase: super::super::System::Registry::HKEY,
    pub pvBase: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for CERT_SYSTEM_STORE_RELOCATE_PARA_0 {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for CERT_SYSTEM_STORE_RELOCATE_PARA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub union CERT_SYSTEM_STORE_RELOCATE_PARA_1 {
    pub pvSystemStore: *mut ::core::ffi::c_void,
    pub pszSystemStore: ::windows_sys::core::PCSTR,
    pub pwszSystemStore: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for CERT_SYSTEM_STORE_RELOCATE_PARA_1 {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for CERT_SYSTEM_STORE_RELOCATE_PARA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_SERVICES_ID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_UNPROTECTED_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SYSTEM_STORE_USERS_ID: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_TEMPLATE_EXT {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub dwMajorVersion: u32,
    pub fMinorVersion: super::super::Foundation::BOOL,
    pub dwMinorVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_TEMPLATE_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_TEMPLATE_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TIMESTAMP_HASH_USE_TYPE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_TPM_SPECIFICATION_INFO {
    pub pwszFamily: ::windows_sys::core::PWSTR,
    pub dwLevel: u32,
    pub dwRevision: u32,
}
impl ::core::marker::Copy for CERT_TPM_SPECIFICATION_INFO {}
impl ::core::clone::Clone for CERT_TPM_SPECIFICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_AUTO_UPDATE_CA_REVOCATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_AUTO_UPDATE_END_REVOCATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_CTL_IS_NOT_SIGNATURE_VALID: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_CTL_IS_NOT_TIME_VALID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_CTL_IS_NOT_VALID_FOR_USAGE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_ALLOW_WEAK_SIGNATURE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_AUTO_UPDATE_WEAK_SIGNATURE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_CRL_VALIDITY_EXTENDED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_EXACT_MATCH_ISSUER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_EXCLUDED_NAME_CONSTRAINT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_ISSUANCE_CHAIN_POLICY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_KEY_MATCH_ISSUER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_NAME_MATCH_ISSUER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_NOT_DEFINED_NAME_CONSTRAINT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_NOT_PERMITTED_NAME_CONSTRAINT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_NOT_SUPPORTED_CRITICAL_EXT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_NOT_SUPPORTED_NAME_CONSTRAINT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_PREFERRED_ISSUER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_VALID_NAME_CONSTRAINTS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_WEAK_HYGIENE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_HAS_WEAK_SIGNATURE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_INVALID_BASIC_CONSTRAINTS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_INVALID_EXTENSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_INVALID_NAME_CONSTRAINTS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_INVALID_POLICY_CONSTRAINTS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_CA_TRUSTED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_COMPLEX_CHAIN: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_CYCLIC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_EXPLICIT_DISTRUST: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_FROM_EXCLUSIVE_TRUST_STORE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_KEY_ROLLOVER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_NOT_SIGNATURE_VALID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_NOT_TIME_NESTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_NOT_TIME_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_NOT_VALID_FOR_USAGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_OFFLINE_REVOCATION: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_PARTIAL_CHAIN: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_PEER_TRUSTED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_REVOKED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_SELF_SIGNED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_IS_UNTRUSTED_ROOT: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_TRUST_LIST_INFO {
    pub cbSize: u32,
    pub pCtlEntry: *mut CTL_ENTRY,
    pub pCtlContext: *mut CTL_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_TRUST_LIST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_TRUST_LIST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_NO_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_NO_ISSUANCE_CHAIN_POLICY: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_NO_OCSP_FAILOVER_TO_CRL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_NO_TIME_CHECK: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_PUB_ALLOW_END_USER_TRUST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_PUB_ALLOW_ENTERPRISE_ADMIN_TRUST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_PUB_ALLOW_MACHINE_ADMIN_TRUST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_PUB_ALLOW_TRUST_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_PUB_AUTHENTICODE_FLAGS_VALUE_NAME: &str = "AuthenticodeFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_PUB_CHECK_PUBLISHER_REV_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_PUB_CHECK_TIMESTAMP_REV_FLAG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_REVOCATION_STATUS_UNKNOWN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_SSL_HANDSHAKE_OCSP: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_SSL_RECONNECT_OCSP: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_SSL_TIME_VALID: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_TRUST_SSL_TIME_VALID_OCSP: u32 = 524288u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_TRUST_STATUS {
    pub dwErrorStatus: u32,
    pub dwInfoStatus: u32,
}
impl ::core::marker::Copy for CERT_TRUST_STATUS {}
impl ::core::clone::Clone for CERT_TRUST_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_UNICODE_ATTR_ERR_INDEX_MASK: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_UNICODE_ATTR_ERR_INDEX_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_UNICODE_IS_RDN_ATTRS_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_UNICODE_RDN_ERR_INDEX_MASK: u32 = 1023u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_UNICODE_RDN_ERR_INDEX_SHIFT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_UNICODE_VALUE_ERR_INDEX_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_UNICODE_VALUE_ERR_INDEX_SHIFT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_USAGE_MATCH {
    pub dwType: u32,
    pub Usage: CTL_USAGE,
}
impl ::core::marker::Copy for CERT_USAGE_MATCH {}
impl ::core::clone::Clone for CERT_USAGE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_V1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_V2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_V3: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_ALLOW_MORE_USAGE_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_CACHE_ONLY_BASED_REVOCATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_INHIBIT_CTL_UPDATE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_NO_TIME_CHECK_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_REV_ACCUMULATIVE_TIMEOUT_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_REV_CHAIN_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_REV_NO_OCSP_FAILOVER_TO_CRL_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_REV_SERVER_OCSP_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_REV_SERVER_OCSP_WIRE_ONLY_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_TRUSTED_SIGNERS_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_VERIFY_UPDATED_CTL_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_X942_DH_PARAMETERS {
    pub p: CRYPTOAPI_BLOB,
    pub g: CRYPTOAPI_BLOB,
    pub q: CRYPTOAPI_BLOB,
    pub j: CRYPTOAPI_BLOB,
    pub pValidationParams: *mut CERT_X942_DH_VALIDATION_PARAMS,
}
impl ::core::marker::Copy for CERT_X942_DH_PARAMETERS {}
impl ::core::clone::Clone for CERT_X942_DH_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CERT_X942_DH_VALIDATION_PARAMS {
    pub seed: CRYPT_BIT_BLOB,
    pub pgenCounter: u32,
}
impl ::core::marker::Copy for CERT_X942_DH_VALIDATION_PARAMS {}
impl ::core::clone::Clone for CERT_X942_DH_VALIDATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_XML_NAME_STR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CESSetupProperty = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CESSETUPPROP_USE_IISAPPPOOLIDENTITY: CESSetupProperty = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CESSETUPPROP_CACONFIG: CESSetupProperty = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CESSETUPPROP_AUTHENTICATION: CESSetupProperty = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CESSETUPPROP_SSLCERTHASH: CESSetupProperty = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CESSETUPPROP_URL: CESSetupProperty = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CESSETUPPROP_RENEWALONLY: CESSetupProperty = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CESSETUPPROP_ALLOW_KEYBASED_RENEWAL: CESSetupProperty = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CLAIMLIST {
    pub count: u32,
    pub claims: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CLAIMLIST {}
impl ::core::clone::Clone for CLAIMLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_ADD_ATTRIBUTES: ::windows_sys::core::PCSTR = 63i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMC_ADD_ATTRIBUTES_INFO {
    pub dwCmcDataReference: u32,
    pub cCertReference: u32,
    pub rgdwCertReference: *mut u32,
    pub cAttribute: u32,
    pub rgAttribute: *mut CRYPT_ATTRIBUTE,
}
impl ::core::marker::Copy for CMC_ADD_ATTRIBUTES_INFO {}
impl ::core::clone::Clone for CMC_ADD_ATTRIBUTES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_ADD_EXTENSIONS: ::windows_sys::core::PCSTR = 62i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_ADD_EXTENSIONS_INFO {
    pub dwCmcDataReference: u32,
    pub cCertReference: u32,
    pub rgdwCertReference: *mut u32,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMC_ADD_EXTENSIONS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMC_ADD_EXTENSIONS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_DATA: ::windows_sys::core::PCSTR = 59i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMC_DATA_INFO {
    pub cTaggedAttribute: u32,
    pub rgTaggedAttribute: *mut CMC_TAGGED_ATTRIBUTE,
    pub cTaggedRequest: u32,
    pub rgTaggedRequest: *mut CMC_TAGGED_REQUEST,
    pub cTaggedContentInfo: u32,
    pub rgTaggedContentInfo: *mut CMC_TAGGED_CONTENT_INFO,
    pub cTaggedOtherMsg: u32,
    pub rgTaggedOtherMsg: *mut CMC_TAGGED_OTHER_MSG,
}
impl ::core::marker::Copy for CMC_DATA_INFO {}
impl ::core::clone::Clone for CMC_DATA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_BAD_ALG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_BAD_CERT_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_BAD_IDENTITY: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_BAD_MESSAGE_CHECK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_BAD_REQUEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_BAD_TIME: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_INTERNAL_CA_ERROR: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_MUST_ARCHIVE_KEYS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_NO_KEY_REUSE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_POP_FAILED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_POP_REQUIRED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_TRY_LATER: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_FAIL_UNSUPORTED_EXT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_OTHER_INFO_FAIL_CHOICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_OTHER_INFO_NO_CHOICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_OTHER_INFO_PEND_CHOICE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_PEND_INFO {
    pub PendToken: CRYPTOAPI_BLOB,
    pub PendTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMC_PEND_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMC_PEND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_RESPONSE: ::windows_sys::core::PCSTR = 60i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMC_RESPONSE_INFO {
    pub cTaggedAttribute: u32,
    pub rgTaggedAttribute: *mut CMC_TAGGED_ATTRIBUTE,
    pub cTaggedContentInfo: u32,
    pub rgTaggedContentInfo: *mut CMC_TAGGED_CONTENT_INFO,
    pub cTaggedOtherMsg: u32,
    pub rgTaggedOtherMsg: *mut CMC_TAGGED_OTHER_MSG,
}
impl ::core::marker::Copy for CMC_RESPONSE_INFO {}
impl ::core::clone::Clone for CMC_RESPONSE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_STATUS: ::windows_sys::core::PCSTR = 61i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_STATUS_CONFIRM_REQUIRED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_STATUS_FAILED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_STATUS_INFO {
    pub dwStatus: u32,
    pub cBodyList: u32,
    pub rgdwBodyList: *mut u32,
    pub pwszStatusString: ::windows_sys::core::PWSTR,
    pub dwOtherInfoChoice: u32,
    pub Anonymous: CMC_STATUS_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMC_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMC_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMC_STATUS_INFO_0 {
    pub dwFailInfo: u32,
    pub pPendInfo: *mut CMC_PEND_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMC_STATUS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMC_STATUS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_STATUS_NO_SUPPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_STATUS_PENDING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_STATUS_SUCCESS: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMC_TAGGED_ATTRIBUTE {
    pub dwBodyPartID: u32,
    pub Attribute: CRYPT_ATTRIBUTE,
}
impl ::core::marker::Copy for CMC_TAGGED_ATTRIBUTE {}
impl ::core::clone::Clone for CMC_TAGGED_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMC_TAGGED_CERT_REQUEST {
    pub dwBodyPartID: u32,
    pub SignedCertRequest: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CMC_TAGGED_CERT_REQUEST {}
impl ::core::clone::Clone for CMC_TAGGED_CERT_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMC_TAGGED_CERT_REQUEST_CHOICE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMC_TAGGED_CONTENT_INFO {
    pub dwBodyPartID: u32,
    pub EncodedContentInfo: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CMC_TAGGED_CONTENT_INFO {}
impl ::core::clone::Clone for CMC_TAGGED_CONTENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMC_TAGGED_OTHER_MSG {
    pub dwBodyPartID: u32,
    pub pszObjId: ::windows_sys::core::PSTR,
    pub Value: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CMC_TAGGED_OTHER_MSG {}
impl ::core::clone::Clone for CMC_TAGGED_OTHER_MSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMC_TAGGED_REQUEST {
    pub dwTaggedRequestChoice: u32,
    pub Anonymous: CMC_TAGGED_REQUEST_0,
}
impl ::core::marker::Copy for CMC_TAGGED_REQUEST {}
impl ::core::clone::Clone for CMC_TAGGED_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CMC_TAGGED_REQUEST_0 {
    pub pTaggedCertRequest: *mut CMC_TAGGED_CERT_REQUEST,
}
impl ::core::marker::Copy for CMC_TAGGED_REQUEST_0 {}
impl ::core::clone::Clone for CMC_TAGGED_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CMSCEPSetup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2857327618, data2: 36476, data3: 18884, data4: [148, 250, 103, 165, 204, 94, 173, 180] };
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ATTR_CERT_COUNT_PARAM: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ATTR_CERT_PARAM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_AUTHENTICATED_ATTRIBUTES_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_BARE_CONTENT_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_BARE_CONTENT_PARAM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CERT_COUNT_PARAM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CERT_PARAM: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CMS_ENCAPSULATED_CONTENT_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CMS_ENCAPSULATED_CTL_FLAG: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CMS_RECIPIENT_COUNT_PARAM: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CMS_RECIPIENT_ENCRYPTED_KEY_INDEX_PARAM: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CMS_RECIPIENT_INDEX_PARAM: u32 = 34u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CMS_RECIPIENT_INFO {
    pub dwRecipientChoice: u32,
    pub Anonymous: CMSG_CMS_RECIPIENT_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_CMS_RECIPIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_CMS_RECIPIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_CMS_RECIPIENT_INFO_0 {
    pub pKeyTrans: *mut CMSG_KEY_TRANS_RECIPIENT_INFO,
    pub pKeyAgree: *mut CMSG_KEY_AGREE_RECIPIENT_INFO,
    pub pMailList: *mut CMSG_MAIL_LIST_RECIPIENT_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_CMS_RECIPIENT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_CMS_RECIPIENT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CMS_RECIPIENT_INFO_PARAM: u32 = 36u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_CMS_SIGNER_INFO {
    pub dwVersion: u32,
    pub SignerId: CERT_ID,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedHash: CRYPTOAPI_BLOB,
    pub AuthAttrs: CRYPT_ATTRIBUTES,
    pub UnauthAttrs: CRYPT_ATTRIBUTES,
}
impl ::core::marker::Copy for CMSG_CMS_SIGNER_INFO {}
impl ::core::clone::Clone for CMSG_CMS_SIGNER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CMS_SIGNER_INFO_PARAM: u32 = 39u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_CNG_CONTENT_DECRYPT_INFO {
    pub cbSize: u32,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pfnAlloc: PFN_CMSG_ALLOC,
    pub pfnFree: PFN_CMSG_FREE,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
    pub pbContentEncryptKey: *mut u8,
    pub cbContentEncryptKey: u32,
    pub hCNGContentEncryptKey: BCRYPT_KEY_HANDLE,
    pub pbCNGContentEncryptKeyObject: *mut u8,
}
impl ::core::marker::Copy for CMSG_CNG_CONTENT_DECRYPT_INFO {}
impl ::core::clone::Clone for CMSG_CNG_CONTENT_DECRYPT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_COMPUTED_HASH_PARAM: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CONTENTS_OCTETS_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CONTENT_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CONTENT_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CONTENT_ENCRYPT_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut ::core::ffi::c_void,
    pub cRecipients: u32,
    pub rgCmsRecipients: *mut CMSG_RECIPIENT_ENCODE_INFO,
    pub pfnAlloc: PFN_CMSG_ALLOC,
    pub pfnFree: PFN_CMSG_FREE,
    pub dwEncryptFlags: u32,
    pub Anonymous: CMSG_CONTENT_ENCRYPT_INFO_0,
    pub dwFlags: u32,
    pub fCNG: super::super::Foundation::BOOL,
    pub pbCNGContentEncryptKeyObject: *mut u8,
    pub pbContentEncryptKey: *mut u8,
    pub cbContentEncryptKey: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_CONTENT_ENCRYPT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_CONTENT_ENCRYPT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_CONTENT_ENCRYPT_INFO_0 {
    pub hContentEncryptKey: usize,
    pub hCNGContentEncryptKey: BCRYPT_KEY_HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_CONTENT_ENCRYPT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_CONTENT_ENCRYPT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CONTENT_ENCRYPT_PAD_ENCODED_LEN_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CONTENT_ENCRYPT_RELEASE_CONTEXT_FLAG: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CONTENT_PARAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CRL_COUNT_PARAM: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CRL_PARAM: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CRYPT_RELEASE_CONTEXT_FLAG: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_ADD_ATTR_CERT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_ADD_CERT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_ADD_CMS_SIGNER_INFO: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_ADD_CRL: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_ADD_SIGNER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    pub cbSize: u32,
    pub dwSignerIndex: u32,
    pub blob: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {}
impl ::core::clone::Clone for CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_DECRYPT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_CTRL_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub dwRecipientIndex: u32,
}
impl ::core::marker::Copy for CMSG_CTRL_DECRYPT_PARA {}
impl ::core::clone::Clone for CMSG_CTRL_DECRYPT_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CMSG_CTRL_DECRYPT_PARA_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl ::core::marker::Copy for CMSG_CTRL_DECRYPT_PARA_0 {}
impl ::core::clone::Clone for CMSG_CTRL_DECRYPT_PARA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_DEL_ATTR_CERT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_DEL_CERT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_DEL_CRL: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_DEL_SIGNER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    pub cbSize: u32,
    pub dwSignerIndex: u32,
    pub dwUnauthAttrIndex: u32,
}
impl ::core::marker::Copy for CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {}
impl ::core::clone::Clone for CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_ENABLE_STRONG_SIGNATURE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_KEY_AGREE_DECRYPT: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub pKeyAgree: *mut CMSG_KEY_AGREE_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
    pub dwRecipientEncryptedKeyIndex: u32,
    pub OriginatorPublicKey: CRYPT_BIT_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_KEY_TRANS_DECRYPT: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    pub cbSize: u32,
    pub Anonymous: CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0,
    pub dwKeySpec: u32,
    pub pKeyTrans: *mut CMSG_KEY_TRANS_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
}
impl ::core::marker::Copy for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {}
impl ::core::clone::Clone for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl ::core::marker::Copy for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0 {}
impl ::core::clone::Clone for CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_MAIL_LIST_DECRYPT: u32 = 18u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    pub cbSize: u32,
    pub hCryptProv: usize,
    pub pMailList: *mut CMSG_MAIL_LIST_RECIPIENT_INFO,
    pub dwRecipientIndex: u32,
    pub dwKeyChoice: u32,
    pub Anonymous: CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0 {
    pub hKeyEncryptionKey: usize,
    pub pvKeyEncryptionKey: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_VERIFY_HASH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_VERIFY_SIGNATURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_CTRL_VERIFY_SIGNATURE_EX: u32 = 19u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub dwSignerIndex: u32,
    pub dwSignerType: u32,
    pub pvSigner: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {}
impl ::core::clone::Clone for CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_DEFAULT_INSTALLABLE_FUNC_OID: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_DETACHED_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENCODED_MESSAGE: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENCODED_SIGNER: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENCODE_SORTED_CTL_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENCODING_TYPE_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENCRYPTED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENCRYPTED_DIGEST: u32 = 27u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_ENCRYPTED_ENCODE_INFO {
    pub cbSize: u32,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CMSG_ENCRYPTED_ENCODE_INFO {}
impl ::core::clone::Clone for CMSG_ENCRYPTED_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENCRYPT_PARAM: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED_DATA_CMS_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED_DATA_PKCS_1_5_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED_DATA_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED_DATA_V2: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_ENVELOPED_ENCODE_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut ::core::ffi::c_void,
    pub cRecipients: u32,
    pub rgpRecipients: *mut *mut CERT_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_ENVELOPED_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_ENVELOPED_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED_RECIPIENT_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED_RECIPIENT_V2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED_RECIPIENT_V3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED_RECIPIENT_V4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPE_ALGORITHM_PARAM: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_HASHED_DATA_CMS_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_HASHED_DATA_PKCS_1_5_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_HASHED_DATA_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_HASHED_DATA_V2: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_HASHED_ENCODE_INFO {
    pub cbSize: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CMSG_HASHED_ENCODE_INFO {}
impl ::core::clone::Clone for CMSG_HASHED_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_HASH_ALGORITHM_PARAM: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_HASH_DATA_PARAM: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_INDEFINITE_LENGTH: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_INNER_CONTENT_TYPE_PARAM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_MATERIAL_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_OBJID_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_ALG_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_BITS_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_PARA_FLAG: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_KEY_AGREE_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub UserKeyingMaterial: CRYPTOAPI_BLOB,
    pub dwOriginatorChoice: CMSG_KEY_AGREE_ORIGINATOR,
    pub Anonymous: CMSG_KEY_AGREE_ENCRYPT_INFO_0,
    pub cKeyAgreeKeyEncryptInfo: u32,
    pub rgpKeyAgreeKeyEncryptInfo: *mut *mut CMSG_KEY_AGREE_KEY_ENCRYPT_INFO,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CMSG_KEY_AGREE_ENCRYPT_INFO {}
impl ::core::clone::Clone for CMSG_KEY_AGREE_ENCRYPT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CMSG_KEY_AGREE_ENCRYPT_INFO_0 {
    pub OriginatorCertId: CERT_ID,
    pub OriginatorPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
}
impl ::core::marker::Copy for CMSG_KEY_AGREE_ENCRYPT_INFO_0 {}
impl ::core::clone::Clone for CMSG_KEY_AGREE_ENCRYPT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    pub cbSize: u32,
    pub EncryptedKey: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {}
impl ::core::clone::Clone for CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CMSG_KEY_AGREE_OPTION = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_EPHEMERAL_KEY_CHOICE: CMSG_KEY_AGREE_OPTION = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_STATIC_KEY_CHOICE: CMSG_KEY_AGREE_OPTION = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CMSG_KEY_AGREE_ORIGINATOR = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_ORIGINATOR_CERT: CMSG_KEY_AGREE_ORIGINATOR = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_ORIGINATOR_PUBLIC_KEY: CMSG_KEY_AGREE_ORIGINATOR = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_RECIPIENT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut ::core::ffi::c_void,
    pub KeyWrapAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyWrapAuxInfo: *mut ::core::ffi::c_void,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub dwKeySpec: u32,
    pub dwKeyChoice: CMSG_KEY_AGREE_OPTION,
    pub Anonymous: CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0,
    pub UserKeyingMaterial: CRYPTOAPI_BLOB,
    pub cRecipientEncryptedKeys: u32,
    pub rgpRecipientEncryptedKeys: *mut *mut CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0 {
    pub pEphemeralAlgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER,
    pub pSenderId: *mut CERT_ID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_KEY_AGREE_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub dwOriginatorChoice: CMSG_KEY_AGREE_ORIGINATOR,
    pub Anonymous: CMSG_KEY_AGREE_RECIPIENT_INFO_0,
    pub UserKeyingMaterial: CRYPTOAPI_BLOB,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub cRecipientEncryptedKeys: u32,
    pub rgpRecipientEncryptedKeys: *mut *mut CMSG_RECIPIENT_ENCRYPTED_KEY_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_KEY_AGREE_RECIPIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_KEY_AGREE_RECIPIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_KEY_AGREE_RECIPIENT_INFO_0 {
    pub OriginatorCertId: CERT_ID,
    pub OriginatorPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_KEY_AGREE_RECIPIENT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_KEY_AGREE_RECIPIENT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_AGREE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_TRANS_CMS_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_KEY_TRANS_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPTOAPI_BLOB,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CMSG_KEY_TRANS_ENCRYPT_INFO {}
impl ::core::clone::Clone for CMSG_KEY_TRANS_ENCRYPT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_TRANS_PKCS_1_5_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_KEY_TRANS_RECIPIENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut ::core::ffi::c_void,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub RecipientPublicKey: CRYPT_BIT_BLOB,
    pub RecipientId: CERT_ID,
}
impl ::core::marker::Copy for CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {}
impl ::core::clone::Clone for CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_KEY_TRANS_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub RecipientId: CERT_ID,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CMSG_KEY_TRANS_RECIPIENT_INFO {}
impl ::core::clone::Clone for CMSG_KEY_TRANS_RECIPIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_LENGTH_ONLY_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_MAIL_LIST_ENCRYPT_INFO {
    pub cbSize: u32,
    pub dwRecipientIndex: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPTOAPI_BLOB,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CMSG_MAIL_LIST_ENCRYPT_INFO {}
impl ::core::clone::Clone for CMSG_MAIL_LIST_ENCRYPT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_MAIL_LIST_HANDLE_KEY_CHOICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_MAIL_LIST_RECIPIENT: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    pub cbSize: u32,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvKeyEncryptionAuxInfo: *mut ::core::ffi::c_void,
    pub hCryptProv: usize,
    pub dwKeyChoice: u32,
    pub Anonymous: CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0,
    pub KeyId: CRYPTOAPI_BLOB,
    pub Date: super::super::Foundation::FILETIME,
    pub pOtherAttr: *mut CRYPT_ATTRIBUTE_TYPE_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0 {
    pub hKeyEncryptionKey: usize,
    pub pvKeyEncryptionKey: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_MAIL_LIST_RECIPIENT_INFO {
    pub dwVersion: u32,
    pub KeyId: CRYPTOAPI_BLOB,
    pub KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedKey: CRYPTOAPI_BLOB,
    pub Date: super::super::Foundation::FILETIME,
    pub pOtherAttr: *mut CRYPT_ATTRIBUTE_TYPE_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_MAIL_LIST_RECIPIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_MAIL_LIST_RECIPIENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_MAIL_LIST_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_MAX_LENGTH_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CAPI1_EXPORT_KEY_AGREE_FUNC: &str = "CryptMsgDllExportKeyAgree";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CAPI1_EXPORT_KEY_TRANS_FUNC: &str = "CryptMsgDllExportKeyTrans";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CAPI1_EXPORT_MAIL_LIST_FUNC: &str = "CryptMsgDllExportMailList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CAPI1_GEN_CONTENT_ENCRYPT_KEY_FUNC: &str = "CryptMsgDllGenContentEncryptKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CAPI1_IMPORT_KEY_AGREE_FUNC: &str = "CryptMsgDllImportKeyAgree";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CAPI1_IMPORT_KEY_TRANS_FUNC: &str = "CryptMsgDllImportKeyTrans";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CAPI1_IMPORT_MAIL_LIST_FUNC: &str = "CryptMsgDllImportMailList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CNG_EXPORT_KEY_AGREE_FUNC: &str = "CryptMsgDllCNGExportKeyAgree";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CNG_EXPORT_KEY_TRANS_FUNC: &str = "CryptMsgDllCNGExportKeyTrans";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CNG_GEN_CONTENT_ENCRYPT_KEY_FUNC: &str = "CryptMsgDllCNGGenContentEncryptKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CNG_IMPORT_CONTENT_ENCRYPT_KEY_FUNC: &str = "CryptMsgDllCNGImportContentEncryptKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CNG_IMPORT_KEY_AGREE_FUNC: &str = "CryptMsgDllCNGImportKeyAgree";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_CNG_IMPORT_KEY_TRANS_FUNC: &str = "CryptMsgDllCNGImportKeyTrans";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_EXPORT_ENCRYPT_KEY_FUNC: &str = "CryptMsgDllExportEncryptKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_EXPORT_KEY_AGREE_FUNC: &str = "CryptMsgDllExportKeyAgree";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_EXPORT_KEY_TRANS_FUNC: &str = "CryptMsgDllExportKeyTrans";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_EXPORT_MAIL_LIST_FUNC: &str = "CryptMsgDllExportMailList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_GEN_CONTENT_ENCRYPT_KEY_FUNC: &str = "CryptMsgDllGenContentEncryptKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_GEN_ENCRYPT_KEY_FUNC: &str = "CryptMsgDllGenEncryptKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_IMPORT_ENCRYPT_KEY_FUNC: &str = "CryptMsgDllImportEncryptKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_IMPORT_KEY_AGREE_FUNC: &str = "CryptMsgDllImportKeyAgree";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_IMPORT_KEY_TRANS_FUNC: &str = "CryptMsgDllImportKeyTrans";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_OID_IMPORT_MAIL_LIST_FUNC: &str = "CryptMsgDllImportMailList";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_RC2_AUX_INFO {
    pub cbSize: u32,
    pub dwBitLen: u32,
}
impl ::core::marker::Copy for CMSG_RC2_AUX_INFO {}
impl ::core::clone::Clone for CMSG_RC2_AUX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_RC4_AUX_INFO {
    pub cbSize: u32,
    pub dwBitLen: u32,
}
impl ::core::marker::Copy for CMSG_RC4_AUX_INFO {}
impl ::core::clone::Clone for CMSG_RC4_AUX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_RC4_NO_SALT_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_RECIPIENT_COUNT_PARAM: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_RECIPIENT_ENCODE_INFO {
    pub dwRecipientChoice: u32,
    pub Anonymous: CMSG_RECIPIENT_ENCODE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_RECIPIENT_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_RECIPIENT_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_RECIPIENT_ENCODE_INFO_0 {
    pub pKeyTrans: *mut CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO,
    pub pKeyAgree: *mut CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO,
    pub pMailList: *mut CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_RECIPIENT_ENCODE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_RECIPIENT_ENCODE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    pub cbSize: u32,
    pub RecipientPublicKey: CRYPT_BIT_BLOB,
    pub RecipientId: CERT_ID,
    pub Date: super::super::Foundation::FILETIME,
    pub pOtherAttr: *mut CRYPT_ATTRIBUTE_TYPE_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    pub RecipientId: CERT_ID,
    pub EncryptedKey: CRYPTOAPI_BLOB,
    pub Date: super::super::Foundation::FILETIME,
    pub pOtherAttr: *mut CRYPT_ATTRIBUTE_TYPE_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_RECIPIENT_INDEX_PARAM: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_RECIPIENT_INFO_PARAM: u32 = 19u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    pub cbSize: u32,
    pub SignedInfo: CMSG_SIGNED_ENCODE_INFO,
    pub EnvelopedInfo: CMSG_ENVELOPED_ENCODE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNED_DATA_CMS_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNED_DATA_NO_SIGN_FLAG: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNED_DATA_PKCS_1_5_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNED_DATA_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNED_DATA_V3: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_SIGNED_ENCODE_INFO {
    pub cbSize: u32,
    pub cSigners: u32,
    pub rgSigners: *mut CMSG_SIGNER_ENCODE_INFO,
    pub cCertEncoded: u32,
    pub rgCertEncoded: *mut CRYPTOAPI_BLOB,
    pub cCrlEncoded: u32,
    pub rgCrlEncoded: *mut CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_SIGNED_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_SIGNED_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_AUTH_ATTR_PARAM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_CERT_ID_PARAM: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_CERT_INFO_PARAM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_COUNT_PARAM: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_SIGNER_ENCODE_INFO {
    pub cbSize: u32,
    pub pCertInfo: *mut CERT_INFO,
    pub Anonymous: CMSG_SIGNER_ENCODE_INFO_0,
    pub dwKeySpec: u32,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut ::core::ffi::c_void,
    pub cAuthAttr: u32,
    pub rgAuthAttr: *mut CRYPT_ATTRIBUTE,
    pub cUnauthAttr: u32,
    pub rgUnauthAttr: *mut CRYPT_ATTRIBUTE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_SIGNER_ENCODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_SIGNER_ENCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CMSG_SIGNER_ENCODE_INFO_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_SIGNER_ENCODE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_SIGNER_ENCODE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_HASH_ALGORITHM_PARAM: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_SIGNER_INFO {
    pub dwVersion: u32,
    pub Issuer: CRYPTOAPI_BLOB,
    pub SerialNumber: CRYPTOAPI_BLOB,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedHash: CRYPTOAPI_BLOB,
    pub AuthAttrs: CRYPT_ATTRIBUTES,
    pub UnauthAttrs: CRYPT_ATTRIBUTES,
}
impl ::core::marker::Copy for CMSG_SIGNER_INFO {}
impl ::core::clone::Clone for CMSG_SIGNER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_INFO_CMS_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_INFO_PARAM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_INFO_PKCS_1_5_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_INFO_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_INFO_V3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_ONLY_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNER_UNAUTH_ATTR_PARAM: u32 = 10u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMSG_SP3_COMPATIBLE_AUX_INFO {
    pub cbSize: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CMSG_SP3_COMPATIBLE_AUX_INFO {}
impl ::core::clone::Clone for CMSG_SP3_COMPATIBLE_AUX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SP3_COMPATIBLE_ENCRYPT_FLAG: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_STREAM_INFO {
    pub cbContent: u32,
    pub pfnStreamOutput: PFN_CMSG_STREAM_OUTPUT,
    pub pvArg: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CMSG_STREAM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CMSG_STREAM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_TRUSTED_SIGNER_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_TYPE_PARAM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_UNPROTECTED_ATTR_PARAM: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_USE_SIGNER_INDEX_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_VERIFY_COUNTER_SIGN_ENABLE_STRONG_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_VERIFY_SIGNER_CERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_VERIFY_SIGNER_CHAIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_VERIFY_SIGNER_NULL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_VERIFY_SIGNER_PUBKEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_VERSION_PARAM: u32 = 30u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMS_DH_KEY_INFO {
    pub dwVersion: u32,
    pub Algid: u32,
    pub pszContentEncObjId: ::windows_sys::core::PSTR,
    pub PubInfo: CRYPTOAPI_BLOB,
    pub pReserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CMS_DH_KEY_INFO {}
impl ::core::clone::Clone for CMS_DH_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CMS_KEY_INFO {
    pub dwVersion: u32,
    pub Algid: u32,
    pub pbOID: *mut u8,
    pub cbOID: u32,
}
impl ::core::marker::Copy for CMS_KEY_INFO {}
impl ::core::clone::Clone for CMS_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMS_SIGNER_INFO: ::windows_sys::core::PCSTR = 501i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CNG_RSA_PRIVATE_KEY_BLOB: ::windows_sys::core::PCSTR = 83i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CNG_RSA_PUBLIC_KEY_BLOB: ::windows_sys::core::PCSTR = 72i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CONTEXT_OID_CAPI2_ANY: ::windows_sys::core::PCSTR = 5i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CONTEXT_OID_CERTIFICATE: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CONTEXT_OID_CREATE_OBJECT_CONTEXT_FUNC: &str = "ContextDllCreateObjectContext";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CONTEXT_OID_CRL: ::windows_sys::core::PCSTR = 2i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CONTEXT_OID_CTL: ::windows_sys::core::PCSTR = 3i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CONTEXT_OID_OCSP_RESP: ::windows_sys::core::PCSTR = 6i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CONTEXT_OID_PKCS7: ::windows_sys::core::PCSTR = 4i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CPS_URLS {
    pub pszURL: ::windows_sys::core::PWSTR,
    pub pAlgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER,
    pub pDigest: *mut CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CPS_URLS {}
impl ::core::clone::Clone for CPS_URLS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS_A: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS_W: ::windows_sys::core::PCSTR = 2i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_CONTEXT {
    pub dwCertEncodingType: u32,
    pub pbCrlEncoded: *mut u8,
    pub cbCrlEncoded: u32,
    pub pCrlInfo: *mut CRL_INFO,
    pub hCertStore: HCERTSTORE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRL_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRL_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRL_DIST_POINT {
    pub DistPointName: CRL_DIST_POINT_NAME,
    pub ReasonFlags: CRYPT_BIT_BLOB,
    pub CRLIssuer: CERT_ALT_NAME_INFO,
}
impl ::core::marker::Copy for CRL_DIST_POINT {}
impl ::core::clone::Clone for CRL_DIST_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRL_DIST_POINTS_INFO {
    pub cDistPoint: u32,
    pub rgDistPoint: *mut CRL_DIST_POINT,
}
impl ::core::marker::Copy for CRL_DIST_POINTS_INFO {}
impl ::core::clone::Clone for CRL_DIST_POINTS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_DIST_POINT_ERR_CRL_ISSUER_BIT: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_DIST_POINT_ERR_INDEX_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_DIST_POINT_ERR_INDEX_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_DIST_POINT_FULL_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_DIST_POINT_ISSUER_RDN_NAME: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRL_DIST_POINT_NAME {
    pub dwDistPointNameChoice: u32,
    pub Anonymous: CRL_DIST_POINT_NAME_0,
}
impl ::core::marker::Copy for CRL_DIST_POINT_NAME {}
impl ::core::clone::Clone for CRL_DIST_POINT_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CRL_DIST_POINT_NAME_0 {
    pub FullName: CERT_ALT_NAME_INFO,
}
impl ::core::marker::Copy for CRL_DIST_POINT_NAME_0 {}
impl ::core::clone::Clone for CRL_DIST_POINT_NAME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_DIST_POINT_NO_NAME: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_ENTRY {
    pub SerialNumber: CRYPTOAPI_BLOB,
    pub RevocationDate: super::super::Foundation::FILETIME,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRL_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRL_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_EXISTING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_ISSUED_BY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_ISSUED_BY_AKI_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_ISSUED_BY_BASE_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_ISSUED_BY_DELTA_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_ISSUED_BY_SIGNATURE_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_ISSUED_FOR: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_FIND_ISSUED_FOR_PARA {
    pub pSubjectCert: *const CERT_CONTEXT,
    pub pIssuerCert: *const CERT_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRL_FIND_ISSUED_FOR_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRL_FIND_ISSUED_FOR_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_FIND_ISSUED_FOR_SET_STRONG_PROPERTIES_FLAG: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_INFO {
    pub dwVersion: u32,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Issuer: CRYPTOAPI_BLOB,
    pub ThisUpdate: super::super::Foundation::FILETIME,
    pub NextUpdate: super::super::Foundation::FILETIME,
    pub cCRLEntry: u32,
    pub rgCRLEntry: *mut CRL_ENTRY,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_ISSUING_DIST_POINT {
    pub DistPointName: CRL_DIST_POINT_NAME,
    pub fOnlyContainsUserCerts: super::super::Foundation::BOOL,
    pub fOnlyContainsCACerts: super::super::Foundation::BOOL,
    pub OnlySomeReasonFlags: CRYPT_BIT_BLOB,
    pub fIndirectCRL: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRL_ISSUING_DIST_POINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRL_ISSUING_DIST_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_AA_COMPROMISE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_AA_COMPROMISE_FLAG: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_AFFILIATION_CHANGED_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_CA_COMPROMISE_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_CERTIFICATE_HOLD_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_CESSATION_OF_OPERATION_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_KEY_COMPROMISE_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_PRIVILEGE_WITHDRAWN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_PRIVILEGE_WITHDRAWN_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_SUPERSEDED_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_REASON_UNUSED_FLAG: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_REVOCATION_INFO {
    pub pCrlEntry: *mut CRL_ENTRY,
    pub pCrlContext: *mut CRL_CONTEXT,
    pub pCrlIssuerChain: *mut CERT_CHAIN_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRL_REVOCATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRL_REVOCATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_V1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRL_V2: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CROSS_CERT_DIST_POINTS_INFO {
    pub dwSyncDeltaTime: u32,
    pub cDistPoint: u32,
    pub rgDistPoint: *mut CERT_ALT_NAME_INFO,
}
impl ::core::marker::Copy for CROSS_CERT_DIST_POINTS_INFO {}
impl ::core::clone::Clone for CROSS_CERT_DIST_POINTS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_DEFAULT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_VALUE_NAME: &str = "CryptnetCachedOcspSwitchToCrlCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_BEFORE_OCSP_ENABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_DISABLE_INFORMATION_EVENTS_VALUE_NAME: &str = "DisableInformationEvents";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_LOG_FILE_NAME_VALUE_NAME: &str = "LogFileName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_VALUE_NAME: &str = "MaxAgeSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME: &str = "MinAfterNextUpdateSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: &str = "MinBeforeNextUpdateSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_PROCESS_NAME_LIST_VALUE_NAME: &str = "ProcessNameList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: &str = "PublishBeforeNextUpdateSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_RANDOM_INTERVAL_SECONDS_VALUE_NAME: &str = "PublishRandomIntervalSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_TIMEOUT_SECONDS_VALUE_NAME: &str = "TimeoutSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_CRL_PRE_FETCH_URL_LIST_VALUE_NAME: &str = "PreFetchUrlList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_DEFAULT: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_VALUE_NAME: &str = "CryptnetMaxCachedOcspPerCrlCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_OCSP_AFTER_CRL_DISABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_AFTER_CURRENT_TIME_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchAfterCurrentTimePreFetchPeriodSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_DEFAULT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_VALUE_NAME: &str = "CryptnetPreFetchAfterPublishPreFetchDivisor";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME: &str = "CryptnetPreFetchBeforeNextUpdatePreFetchDivisor";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_MAX_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchMaxAfterNextUpdatePreFetchPeriodSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_MAX_MAX_AGE_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchMaxMaxAgeSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchMinAfterNextUpdatePreFetchPeriodSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchMinBeforeNextUpdatePreFetchSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_MIN_MAX_AGE_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchMinMaxAgeSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_MIN_OCSP_VALIDITY_PERIOD_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchMinOcspValidityPeriodSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_RETRIEVAL_TIMEOUT_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchRetrievalTimeoutSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_DEFAULT: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchScanAfterTriggerDelaySeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_TRIGGER_DISABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_TRIGGER_PERIOD_SECONDS_VALUE_NAME: &str = "CryptnetPreFetchTriggerPeriodSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME: &str = "CryptnetPreFetchValidityPeriodAfterNextUpdatePreFetchDivisor";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH_EXEMPT_SECONDS_VALUE_NAME: &str = "CryptnetDefaultFlushExemptSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_DISABLE_FLUSH: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTNET_URL_CACHE_FLUSH_INFO {
    pub cbSize: u32,
    pub dwExemptSeconds: u32,
    pub ExpireTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTNET_URL_CACHE_FLUSH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTNET_URL_CACHE_FLUSH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_PRE_FETCH_AUTOROOT_CAB: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_PRE_FETCH_BLOB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_PRE_FETCH_CRL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_PRE_FETCH_DISALLOWED_CERT_CAB: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    pub cbSize: u32,
    pub dwObjectType: u32,
    pub dwError: u32,
    pub dwReserved: u32,
    pub ThisUpdateTime: super::super::Foundation::FILETIME,
    pub NextUpdateTime: super::super::Foundation::FILETIME,
    pub PublishTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTNET_URL_CACHE_PRE_FETCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_PRE_FETCH_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_PRE_FETCH_OCSP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_PRE_FETCH_PIN_RULES_CAB: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_RESPONSE_HTTP: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTNET_URL_CACHE_RESPONSE_INFO {
    pub cbSize: u32,
    pub wResponseType: u16,
    pub wResponseFlags: u16,
    pub LastModifiedTime: super::super::Foundation::FILETIME,
    pub dwMaxAge: u32,
    pub pwszETag: ::windows_sys::core::PCWSTR,
    pub dwProxyId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTNET_URL_CACHE_RESPONSE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTNET_URL_CACHE_RESPONSE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_RESPONSE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTNET_URL_CACHE_RESPONSE_VALIDATED: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPTOAPI_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for CRYPTOAPI_BLOB {}
impl ::core::clone::Clone for CRYPTOAPI_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECTMEMORY_BLOCK_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECTMEMORY_CROSS_PROCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECTMEMORY_SAME_LOGON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECTMEMORY_SAME_PROCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_AUDIT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_CRED_REGENERATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_CRED_SYNC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_FIRST_RESERVED_FLAGVAL: u32 = 268435455u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_LAST_RESERVED_FLAGVAL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_LOCAL_MACHINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_NO_RECOVERY: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTPROTECT_PROMPTSTRUCT {
    pub cbSize: u32,
    pub dwPromptFlags: u32,
    pub hwndApp: super::super::Foundation::HWND,
    pub szPrompt: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTPROTECT_PROMPTSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTPROTECT_PROMPTSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_PROMPT_ON_PROTECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_PROMPT_ON_UNPROTECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_PROMPT_REQUIRE_STRONG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_PROMPT_RESERVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_PROMPT_STRONG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_UI_FORBIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPTPROTECT_VERIFY_PROTECTION: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_3DES_KEY_STATE {
    pub Key: [u8; 24],
    pub IV: [u8; 8],
    pub Feedback: [u8; 8],
}
impl ::core::marker::Copy for CRYPT_3DES_KEY_STATE {}
impl ::core::clone::Clone for CRYPT_3DES_KEY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACCUMULATIVE_TIMEOUT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_ACQUIRE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_CACHE_FLAG: CRYPT_ACQUIRE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_COMPARE_KEY_FLAG: CRYPT_ACQUIRE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_NO_HEALING: CRYPT_ACQUIRE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_SILENT_FLAG: CRYPT_ACQUIRE_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_USE_PROV_INFO_FLAG: CRYPT_ACQUIRE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_NCRYPT_KEY_FLAGS_MASK: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ACQUIRE_WINDOW_HANDLE_FLAG: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_AES_128_KEY_STATE {
    pub Key: [u8; 16],
    pub IV: [u8; 16],
    pub EncryptionState: [u8; 176],
    pub DecryptionState: [u8; 176],
    pub Feedback: [u8; 16],
}
impl ::core::marker::Copy for CRYPT_AES_128_KEY_STATE {}
impl ::core::clone::Clone for CRYPT_AES_128_KEY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_AES_256_KEY_STATE {
    pub Key: [u8; 32],
    pub IV: [u8; 16],
    pub EncryptionState: [u8; 240],
    pub DecryptionState: [u8; 240],
    pub Feedback: [u8; 16],
}
impl ::core::marker::Copy for CRYPT_AES_256_KEY_STATE {}
impl ::core::clone::Clone for CRYPT_AES_256_KEY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_AIA_RETRIEVAL: u32 = 524288u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ALGORITHM_IDENTIFIER {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub Parameters: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_ALGORITHM_IDENTIFIER {}
impl ::core::clone::Clone for CRYPT_ALGORITHM_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ARCHIVE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ASN_ENCODING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ASYNC_RETRIEVAL: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    pub pfnCompletion: PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC,
    pub pvCompletion: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_ASYNC_RETRIEVAL_COMPLETION {}
impl ::core::clone::Clone for CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ATTRIBUTE {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub cValue: u32,
    pub rgValue: *mut CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_ATTRIBUTE {}
impl ::core::clone::Clone for CRYPT_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ATTRIBUTES {
    pub cAttr: u32,
    pub rgAttr: *mut CRYPT_ATTRIBUTE,
}
impl ::core::marker::Copy for CRYPT_ATTRIBUTES {}
impl ::core::clone::Clone for CRYPT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ATTRIBUTE_TYPE_VALUE {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub Value: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_ATTRIBUTE_TYPE_VALUE {}
impl ::core::clone::Clone for CRYPT_ATTRIBUTE_TYPE_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_BIT_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
    pub cUnusedBits: u32,
}
impl ::core::marker::Copy for CRYPT_BIT_BLOB {}
impl ::core::clone::Clone for CRYPT_BIT_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_BLOB_ARRAY {
    pub cBlob: u32,
    pub rgBlob: *mut CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_BLOB_ARRAY {}
impl ::core::clone::Clone for CRYPT_BLOB_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_CACHE_ONLY_RETRIEVAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_CHECK_FRESHNESS_TIME_VALIDITY: u32 = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CONTENT_INFO {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub Content: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_CONTENT_INFO {}
impl ::core::clone::Clone for CRYPT_CONTENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub cValue: u32,
    pub rgValue: *mut CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {}
impl ::core::clone::Clone for CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CONTEXTS {
    pub cContexts: u32,
    pub rgpszContexts: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_CONTEXTS {}
impl ::core::clone::Clone for CRYPT_CONTEXTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CONTEXT_CONFIG {
    pub dwFlags: CRYPT_CONTEXT_CONFIG_FLAGS,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for CRYPT_CONTEXT_CONFIG {}
impl ::core::clone::Clone for CRYPT_CONTEXT_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_CONTEXT_CONFIG_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_EXCLUSIVE: CRYPT_CONTEXT_CONFIG_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OVERRIDE: CRYPT_CONTEXT_CONFIG_FLAGS = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CONTEXT_FUNCTIONS {
    pub cFunctions: u32,
    pub rgpszFunctions: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_CONTEXT_FUNCTIONS {}
impl ::core::clone::Clone for CRYPT_CONTEXT_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CONTEXT_FUNCTION_CONFIG {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for CRYPT_CONTEXT_FUNCTION_CONFIG {}
impl ::core::clone::Clone for CRYPT_CONTEXT_FUNCTION_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    pub cProviders: u32,
    pub rgpszProviders: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_CONTEXT_FUNCTION_PROVIDERS {}
impl ::core::clone::Clone for CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_CREATE_NEW_FLUSH_ENTRY: u32 = 268435456u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CREDENTIALS {
    pub cbSize: u32,
    pub pszCredentialsOid: ::windows_sys::core::PCSTR,
    pub pvCredentials: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_CREDENTIALS {}
impl ::core::clone::Clone for CRYPT_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_CSP_PROVIDER {
    pub dwKeySpec: u32,
    pub pwszProviderName: ::windows_sys::core::PWSTR,
    pub Signature: CRYPT_BIT_BLOB,
}
impl ::core::marker::Copy for CRYPT_CSP_PROVIDER {}
impl ::core::clone::Clone for CRYPT_CSP_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECODE_ALLOC_FLAG: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECODE_ENABLE_PUNYCODE_FLAG: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECODE_ENABLE_UTF8PERCENT_FLAG: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECODE_NOCOPY_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_DECODE_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_CRYPT_ALLOC,
    pub pfnFree: PFN_CRYPT_FREE,
}
impl ::core::marker::Copy for CRYPT_DECODE_PARA {}
impl ::core::clone::Clone for CRYPT_DECODE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECODE_SHARE_OID_STRING_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECODE_TO_BE_SIGNED_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECRYPT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_DECRYPT_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub cCertStore: u32,
    pub rghCertStore: *mut HCERTSTORE,
}
impl ::core::marker::Copy for CRYPT_DECRYPT_MESSAGE_PARA {}
impl ::core::clone::Clone for CRYPT_DECRYPT_MESSAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DECRYPT_RSA_NO_PADDING_CHECK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DEFAULT_CONTAINER_OPTIONAL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DEFAULT_CONTEXT: &str = "Default";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_DEFAULT_CONTEXT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DEFAULT_CONTEXT_AUTO_RELEASE_FLAG: CRYPT_DEFAULT_CONTEXT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DEFAULT_CONTEXT_PROCESS_FLAG: CRYPT_DEFAULT_CONTEXT_FLAGS = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    pub cOID: u32,
    pub rgpszOID: *mut ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {}
impl ::core::clone::Clone for CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_DEFAULT_CONTEXT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DEFAULT_CONTEXT_CERT_SIGN_OID: CRYPT_DEFAULT_CONTEXT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DEFAULT_CONTEXT_MULTI_CERT_SIGN_OID: CRYPT_DEFAULT_CONTEXT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DEFAULT_OID: &str = "DEFAULT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DELETEKEYSET: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DELETE_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DELETE_KEYSET: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_DES_KEY_STATE {
    pub Key: [u8; 8],
    pub IV: [u8; 8],
    pub Feedback: [u8; 8],
}
impl ::core::marker::Copy for CRYPT_DES_KEY_STATE {}
impl ::core::clone::Clone for CRYPT_DES_KEY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DONT_CACHE_RESULT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DONT_CHECK_TIME_VALIDITY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DONT_VERIFY_SIGNATURE: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ECC_CMS_SHARED_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EntityUInfo: CRYPTOAPI_BLOB,
    pub rgbSuppPubInfo: [u8; 4],
}
impl ::core::marker::Copy for CRYPT_ECC_CMS_SHARED_INFO {}
impl ::core::clone::Clone for CRYPT_ECC_CMS_SHARED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ECC_CMS_SHARED_INFO_SUPPPUBINFO_BYTE_LENGTH: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ECC_PRIVATE_KEY_INFO {
    pub dwVersion: u32,
    pub PrivateKey: CRYPTOAPI_BLOB,
    pub szCurveOid: ::windows_sys::core::PSTR,
    pub PublicKey: CRYPT_BIT_BLOB,
}
impl ::core::marker::Copy for CRYPT_ECC_PRIVATE_KEY_INFO {}
impl ::core::clone::Clone for CRYPT_ECC_PRIVATE_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ECC_PRIVATE_KEY_INFO_v1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENABLE_FILE_RETRIEVAL: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENABLE_SSL_REVOCATION_RETRIEVAL: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENCODE_DECODE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENCODE_ENABLE_UTF8PERCENT_FLAG: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENCODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_ENCODE_OBJECT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENCODE_ALLOC_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENCODE_ENABLE_PUNYCODE_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_UNICODE_NAME_ENCODE_DISABLE_CHECK_TYPE_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_UNICODE_NAME_ENCODE_ENABLE_T61_UNICODE_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_UNICODE_NAME_ENCODE_ENABLE_UTF8_UNICODE_FLAG: CRYPT_ENCODE_OBJECT_FLAGS = 536870912u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ENCODE_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_CRYPT_ALLOC,
    pub pfnFree: PFN_CRYPT_FREE,
}
impl ::core::marker::Copy for CRYPT_ENCODE_PARA {}
impl ::core::clone::Clone for CRYPT_ENCODE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENCRYPT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {
    pub EncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub EncryptedPrivateKey: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {}
impl ::core::clone::Clone for CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENCRYPT_ALG_OID_GROUP_ID: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ENCRYPT_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvEncryptionAuxInfo: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub dwInnerContentType: u32,
}
impl ::core::marker::Copy for CRYPT_ENCRYPT_MESSAGE_PARA {}
impl ::core::clone::Clone for CRYPT_ENCRYPT_MESSAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ENHKEY_USAGE_OID_GROUP_ID: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pwszValue: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_ENROLLMENT_NAME_VALUE_PAIR {}
impl ::core::clone::Clone for CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_EXPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_EXPORT_KEY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_EXT_OR_ATTR_OID_GROUP_ID: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FAILED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FASTSGC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_FIND_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FIND_USER_KEYSET_FLAG: CRYPT_FIND_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FIND_MACHINE_KEYSET_FLAG: CRYPT_FIND_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FIND_SILENT_KEYSET_FLAG: CRYPT_FIND_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FIRST_ALG_OID_GROUP_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FLAG_IPSEC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FLAG_PCT1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FLAG_SIGNING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FLAG_SSL2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FLAG_SSL3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FLAG_TLS1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_COMMA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_CRLF: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_OID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_RDN_CRLF: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_RDN_REVERSE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_RDN_SEMICOLON: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_RDN_UNQUOTE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_SEMICOLON: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_SIMPLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_STR_MULTI_LINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_STR_NO_HEX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORMAT_X509: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_GET_INSTALLED_OID_FUNC_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    pub cbSize: u32,
    pub iDeltaCrlIndicator: i32,
    pub pftCacheResync: *mut super::super::Foundation::FILETIME,
    pub pLastSyncTime: *mut super::super::Foundation::FILETIME,
    pub pMaxAgeTime: *mut super::super::Foundation::FILETIME,
    pub pChainPara: *mut CERT_REVOCATION_CHAIN_PARA,
    pub pDeltaCrlIndicator: *mut CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_GET_URL_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_GET_URL_FROM_PROPERTY: CRYPT_GET_URL_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_GET_URL_FROM_EXTENSION: CRYPT_GET_URL_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_GET_URL_FROM_UNAUTH_ATTRIBUTE: CRYPT_GET_URL_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_GET_URL_FROM_AUTH_ATTRIBUTE: CRYPT_GET_URL_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_HASH_ALG_OID_GROUP_ID: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_HASH_INFO {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Hash: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_HASH_INFO {}
impl ::core::clone::Clone for CRYPT_HASH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_HASH_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_HASH_MESSAGE_PARA {}
impl ::core::clone::Clone for CRYPT_HASH_MESSAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_HTTP_POST_RETRIEVAL: u32 = 1048576u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_IMAGE_REF {
    pub pszImage: ::windows_sys::core::PWSTR,
    pub dwFlags: CRYPT_IMAGE_REF_FLAGS,
}
impl ::core::marker::Copy for CRYPT_IMAGE_REF {}
impl ::core::clone::Clone for CRYPT_IMAGE_REF {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_IMAGE_REF_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MIN_DEPENDENCIES: CRYPT_IMAGE_REF_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PROCESS_ISOLATE: CRYPT_IMAGE_REF_FLAGS = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_IMAGE_REG {
    pub pszImage: ::windows_sys::core::PWSTR,
    pub cInterfaces: u32,
    pub rgpInterfaces: *mut *mut CRYPT_INTERFACE_REG,
}
impl ::core::marker::Copy for CRYPT_IMAGE_REG {}
impl ::core::clone::Clone for CRYPT_IMAGE_REG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_IMPL_HARDWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_IMPL_MIXED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_IMPL_REMOVABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_IMPL_SOFTWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_IMPL_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_IMPORT_KEY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_IMPORT_PUBLIC_KEY_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG: CRYPT_IMPORT_PUBLIC_KEY_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG: CRYPT_IMPORT_PUBLIC_KEY_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_INSTALL_OID_FUNC_BEFORE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_INSTALL_OID_INFO_BEFORE_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_INTERFACE_REG {
    pub dwInterface: BCRYPT_INTERFACE,
    pub dwFlags: BCRYPT_TABLE,
    pub cFunctions: u32,
    pub rgpszFunctions: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_INTERFACE_REG {}
impl ::core::clone::Clone for CRYPT_INTERFACE_REG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_KDF_OID_GROUP_ID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_KEEP_TIME_VALID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_KEYID_ALLOC_FLAG: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_KEYID_DELETE_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_KEYID_MACHINE_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_KEYID_SET_NEW_FLAG: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_KEY_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_EXPORTABLE: CRYPT_KEY_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_USER_PROTECTED: CRYPT_KEY_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ARCHIVABLE: CRYPT_KEY_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_CREATE_IV: CRYPT_KEY_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_CREATE_SALT: CRYPT_KEY_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DATA_KEY: CRYPT_KEY_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_FORCE_KEY_PROTECTION_HIGH: CRYPT_KEY_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_KEK: CRYPT_KEY_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_INITIATOR: CRYPT_KEY_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_NO_SALT: CRYPT_KEY_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_ONLINE: CRYPT_KEY_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PREGEN: CRYPT_KEY_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RECIPIENT: CRYPT_KEY_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SF: CRYPT_KEY_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SGCKEY: CRYPT_KEY_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VOLATILE: CRYPT_KEY_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MACHINE_KEYSET: CRYPT_KEY_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_USER_KEYSET: CRYPT_KEY_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_PREFER_CNG_KSP: CRYPT_KEY_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ALWAYS_CNG_KSP: CRYPT_KEY_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ALLOW_OVERWRITE_KEY: CRYPT_KEY_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_NO_PERSIST_KEY: CRYPT_KEY_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_INCLUDE_EXTENDED_PROPERTIES: CRYPT_KEY_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OAEP: CRYPT_KEY_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_BLOB_VER3: CRYPT_KEY_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_DESTROYKEY: CRYPT_KEY_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SSL2_FALLBACK: CRYPT_KEY_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_Y_ONLY: CRYPT_KEY_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_IPSEC_HMAC_KEY: CRYPT_KEY_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SET_KEY_PROV_HANDLE_PROP_ID: CRYPT_KEY_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CERT_SET_KEY_CONTEXT_PROP_ID: CRYPT_KEY_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_KEY_PARAM_ID = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_ALGID: CRYPT_KEY_PARAM_ID = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_CERTIFICATE: CRYPT_KEY_PARAM_ID = 26u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PERMISSIONS: CRYPT_KEY_PARAM_ID = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_SALT: CRYPT_KEY_PARAM_ID = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_SALT_EX: CRYPT_KEY_PARAM_ID = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_BLOCKLEN: CRYPT_KEY_PARAM_ID = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_GET_USE_COUNT: CRYPT_KEY_PARAM_ID = 42u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_KEYLEN: CRYPT_KEY_PARAM_ID = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_KEY_PROV_INFO {
    pub pwszContainerName: ::windows_sys::core::PWSTR,
    pub pwszProvName: ::windows_sys::core::PWSTR,
    pub dwProvType: u32,
    pub dwFlags: CRYPT_KEY_FLAGS,
    pub cProvParam: u32,
    pub rgProvParam: *mut CRYPT_KEY_PROV_PARAM,
    pub dwKeySpec: u32,
}
impl ::core::marker::Copy for CRYPT_KEY_PROV_INFO {}
impl ::core::clone::Clone for CRYPT_KEY_PROV_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_KEY_PROV_PARAM {
    pub dwParam: u32,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CRYPT_KEY_PROV_PARAM {}
impl ::core::clone::Clone for CRYPT_KEY_PROV_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_KEY_SIGN_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: CERT_QUERY_ENCODING_TYPE,
    pub Anonymous: CRYPT_KEY_SIGN_MESSAGE_PARA_0,
    pub dwKeySpec: CERT_KEY_SPEC,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut ::core::ffi::c_void,
    pub PubKeyAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
}
impl ::core::marker::Copy for CRYPT_KEY_SIGN_MESSAGE_PARA {}
impl ::core::clone::Clone for CRYPT_KEY_SIGN_MESSAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CRYPT_KEY_SIGN_MESSAGE_PARA_0 {
    pub hCryptProv: usize,
    pub hNCryptKey: NCRYPT_KEY_HANDLE,
}
impl ::core::marker::Copy for CRYPT_KEY_SIGN_MESSAGE_PARA_0 {}
impl ::core::clone::Clone for CRYPT_KEY_SIGN_MESSAGE_PARA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_KEY_VERIFY_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
}
impl ::core::marker::Copy for CRYPT_KEY_VERIFY_MESSAGE_PARA {}
impl ::core::clone::Clone for CRYPT_KEY_VERIFY_MESSAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LAST_ALG_OID_GROUP_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LAST_OID_GROUP_ID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LDAP_AREC_EXCLUSIVE_RETRIEVAL: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LDAP_INSERT_ENTRY_ATTRIBUTE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LDAP_SCOPE_BASE_ONLY_RETRIEVAL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LDAP_SIGN_RETRIEVAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LITTLE_ENDIAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LOCALIZED_NAME_ENCODING_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_LOCALIZED_NAME_OID: &str = "LocalizedNames";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MAC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MACHINE_DEFAULT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_MASK_GEN_ALGORITHM {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
}
impl ::core::marker::Copy for CRYPT_MASK_GEN_ALGORITHM {}
impl ::core::clone::Clone for CRYPT_MASK_GEN_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MATCH_ANY_ENCODING_TYPE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MESSAGE_BARE_CONTENT_OUT_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MESSAGE_ENCAPSULATED_CONTENT_OUT_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MESSAGE_KEYID_RECIPIENT_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MESSAGE_KEYID_SIGNER_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MESSAGE_SILENT_KEYSET_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_CBC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_CBCI: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_CBCOFM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_CBCOFMI: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_CFB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_CFBP: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_CTS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_ECB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_OFB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_MODE_OFBP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_MSG_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_DATA: CRYPT_MSG_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNED: CRYPT_MSG_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_ENVELOPED: CRYPT_MSG_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_SIGNED_AND_ENVELOPED: CRYPT_MSG_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CMSG_HASHED: CRYPT_MSG_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_NDR_ENCODING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_NEWKEYSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_NEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_NOHASHOID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_NOT_MODIFIED_RETRIEVAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_NO_AUTH_RETRIEVAL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_NO_OCSP_FAILOVER_TO_CRL_RETRIEVAL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OBJECT_LOCATOR_FIRST_RESERVED_USER_NAME_TYPE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_NAME_TYPE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_USER_NAME_TYPE: u32 = 65535u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE {
    pub cbSize: u32,
    pub pfnGet: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET,
    pub pfnRelease: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE,
    pub pfnFreePassword: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD,
    pub pfnFree: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE,
    pub pfnFreeIdentifier: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_OBJECT_LOCATOR_RELEASE_REASON = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OBJECT_LOCATOR_RELEASE_SYSTEM_SHUTDOWN: CRYPT_OBJECT_LOCATOR_RELEASE_REASON = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OBJECT_LOCATOR_RELEASE_SERVICE_STOP: CRYPT_OBJECT_LOCATOR_RELEASE_REASON = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OBJECT_LOCATOR_RELEASE_PROCESS_EXIT: CRYPT_OBJECT_LOCATOR_RELEASE_REASON = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OBJECT_LOCATOR_RELEASE_DLL_UNLOAD: CRYPT_OBJECT_LOCATOR_RELEASE_REASON = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OBJECT_LOCATOR_SPN_NAME_TYPE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_OBJID_TABLE {
    pub dwAlgId: u32,
    pub pszObjId: ::windows_sys::core::PCSTR,
}
impl ::core::marker::Copy for CRYPT_OBJID_TABLE {}
impl ::core::clone::Clone for CRYPT_OBJID_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OCSP_ONLY_RETRIEVAL: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OFFLINE_CHECK_RETRIEVAL: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_CREATE_COM_OBJECT_FUNC: &str = "CryptDllCreateCOMObject";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_DECODE_OBJECT_EX_FUNC: &str = "CryptDllDecodeObjectEx";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_DECODE_OBJECT_FUNC: &str = "CryptDllDecodeObject";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_DISABLE_SEARCH_DS_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_ENCODE_OBJECT_EX_FUNC: &str = "CryptDllEncodeObjectEx";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_ENCODE_OBJECT_FUNC: &str = "CryptDllEncodeObject";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_ENUM_PHYSICAL_STORE_FUNC: &str = "CertDllEnumPhysicalStore";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_ENUM_SYSTEM_STORE_FUNC: &str = "CertDllEnumSystemStore";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_EXPORT_PRIVATE_KEY_INFO_FUNC: &str = "CryptDllExportPrivateKeyInfoEx";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC: &str = "CryptDllExportPublicKeyInfoEx2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC: &str = "CryptDllExportPublicKeyInfoFromBCryptKeyHandle";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FUNC: &str = "CryptDllExportPublicKeyInfoEx";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC: &str = "CryptDllExtractEncodedSignatureParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_FIND_LOCALIZED_NAME_FUNC: &str = "CryptDllFindLocalizedName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_FIND_OID_INFO_FUNC: &str = "CryptDllFindOIDInfo";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_FORMAT_OBJECT_FUNC: &str = "CryptDllFormatObject";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_OID_FUNC_ENTRY {
    pub pszOID: ::windows_sys::core::PCSTR,
    pub pvFuncAddr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_OID_FUNC_ENTRY {}
impl ::core::clone::Clone for CRYPT_OID_FUNC_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_IMPORT_PRIVATE_KEY_INFO_FUNC: &str = "CryptDllImportPrivateKeyInfoEx";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC: &str = "CryptDllImportPublicKeyInfoEx2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_FUNC: &str = "CryptDllImportPublicKeyInfoEx";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_OID_INFO {
    pub cbSize: u32,
    pub pszOID: ::windows_sys::core::PCSTR,
    pub pwszName: ::windows_sys::core::PCWSTR,
    pub dwGroupId: u32,
    pub Anonymous: CRYPT_OID_INFO_0,
    pub ExtraInfo: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_OID_INFO {}
impl ::core::clone::Clone for CRYPT_OID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CRYPT_OID_INFO_0 {
    pub dwValue: u32,
    pub Algid: u32,
    pub dwLength: u32,
}
impl ::core::marker::Copy for CRYPT_OID_INFO_0 {}
impl ::core::clone::Clone for CRYPT_OID_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_ALGID_KEY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_CNG_ALGID_KEY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_CNG_SIGN_KEY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_ECC_PARAMETERS_ALGORITHM: &str = "CryptOIDInfoECCParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_ECC_WRAP_PARAMETERS_ALGORITHM: &str = "CryptOIDInfoECCWrapParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_HASH_PARAMETERS_ALGORITHM: &str = "CryptOIDInfoHashParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_MGF1_PARAMETERS_ALGORITHM: &str = "CryptOIDInfoMgf1Parameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_NAME_KEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_NO_PARAMETERS_ALGORITHM: &str = "CryptOIDInfoNoParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_NO_SIGN_ALGORITHM: &str = "CryptOIDInfoNoSign";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_OAEP_PARAMETERS_ALGORITHM: &str = "CryptOIDInfoOAEPParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: u32 = 268369920u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_OID_KEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_OID_KEY_FLAGS_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INFO_SIGN_KEY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_INHIBIT_SIGNATURE_FORMAT_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_NO_NULL_ALGORITHM_PARA_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_OPEN_STORE_PROV_FUNC: &str = "CertDllOpenStoreProv";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_OPEN_SYSTEM_STORE_PROV_FUNC: &str = "CertDllOpenSystemStoreProv";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_PREFER_CNG_ALGID_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_PUBKEY_ENCRYPT_ONLY_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_PUBKEY_SIGN_ONLY_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_REGISTER_PHYSICAL_STORE_FUNC: &str = "CertDllRegisterPhysicalStore";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_REGISTER_SYSTEM_STORE_FUNC: &str = "CertDllRegisterSystemStore";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_REGPATH: &str = "Software\\Microsoft\\Cryptography\\OID";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_REG_DLL_VALUE_NAME: &str = "Dll";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_REG_ENCODING_TYPE_PREFIX: &str = "EncodingType ";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_REG_FLAGS_VALUE_NAME: &str = "CryptFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_REG_FUNC_NAME_VALUE_NAME: &str = "FuncName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_REG_FUNC_NAME_VALUE_NAME_A: &str = "FuncName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_SIGN_AND_ENCODE_HASH_FUNC: &str = "CryptDllSignAndEncodeHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_SYSTEM_STORE_LOCATION_VALUE_NAME: &str = "SystemStoreLocation";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_UNREGISTER_PHYSICAL_STORE_FUNC: &str = "CertDllUnregisterPhysicalStore";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_UNREGISTER_SYSTEM_STORE_FUNC: &str = "CertDllUnregisterSystemStore";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_USE_PUBKEY_PARA_FOR_PKCS7_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_VERIFY_CERTIFICATE_CHAIN_POLICY_FUNC: &str = "CertDllVerifyCertificateChainPolicy";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_VERIFY_CTL_USAGE_FUNC: &str = "CertDllVerifyCTLUsage";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_VERIFY_ENCODED_SIGNATURE_FUNC: &str = "CryptDllVerifyEncodedSignature";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OID_VERIFY_REVOCATION_FUNC: &str = "CertDllVerifyRevocation";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OVERWRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_OWF_REPL_LM_HASH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PARAM_ASYNC_RETRIEVAL_COMPLETION: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PARAM_CANCEL_ASYNC_RETRIEVAL: ::windows_sys::core::PCSTR = 2i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PASSWORD_CREDENTIALSA {
    pub cbSize: u32,
    pub pszUsername: ::windows_sys::core::PSTR,
    pub pszPassword: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CRYPT_PASSWORD_CREDENTIALSA {}
impl ::core::clone::Clone for CRYPT_PASSWORD_CREDENTIALSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PASSWORD_CREDENTIALSW {
    pub cbSize: u32,
    pub pszUsername: ::windows_sys::core::PWSTR,
    pub pszPassword: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_PASSWORD_CREDENTIALSW {}
impl ::core::clone::Clone for CRYPT_PASSWORD_CREDENTIALSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PKCS12_PBE_PARAMS {
    pub iIterations: i32,
    pub cbSalt: u32,
}
impl ::core::marker::Copy for CRYPT_PKCS12_PBE_PARAMS {}
impl ::core::clone::Clone for CRYPT_PKCS12_PBE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PKCS8_EXPORT_PARAMS {
    pub hCryptProv: usize,
    pub dwKeySpec: u32,
    pub pszPrivateKeyObjId: ::windows_sys::core::PSTR,
    pub pEncryptPrivateKeyFunc: PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC,
    pub pVoidEncryptFunc: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_PKCS8_EXPORT_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_PKCS8_EXPORT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PKCS8_IMPORT_PARAMS {
    pub PrivateKey: CRYPTOAPI_BLOB,
    pub pResolvehCryptProvFunc: PCRYPT_RESOLVE_HCRYPTPROV_FUNC,
    pub pVoidResolveFunc: *mut ::core::ffi::c_void,
    pub pDecryptPrivateKeyFunc: PCRYPT_DECRYPT_PRIVATE_KEY_FUNC,
    pub pVoidDecryptFunc: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_PKCS8_IMPORT_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_PKCS8_IMPORT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_POLICY_OID_GROUP_ID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PRIORITY_BOTTOM: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PRIORITY_TOP: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PRIVATE_KEY_INFO {
    pub Version: u32,
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub PrivateKey: CRYPTOAPI_BLOB,
    pub pAttributes: *mut CRYPT_ATTRIBUTES,
}
impl ::core::marker::Copy for CRYPT_PRIVATE_KEY_INFO {}
impl ::core::clone::Clone for CRYPT_PRIVATE_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PROPERTY_REF {
    pub pszProperty: ::windows_sys::core::PWSTR,
    pub cbValue: u32,
    pub pbValue: *mut u8,
}
impl ::core::marker::Copy for CRYPT_PROPERTY_REF {}
impl ::core::clone::Clone for CRYPT_PROPERTY_REF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PROVIDERS {
    pub cProviders: u32,
    pub rgpszProviders: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_PROVIDERS {}
impl ::core::clone::Clone for CRYPT_PROVIDERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PROVIDER_REF {
    pub dwInterface: u32,
    pub pszFunction: ::windows_sys::core::PWSTR,
    pub pszProvider: ::windows_sys::core::PWSTR,
    pub cProperties: u32,
    pub rgpProperties: *mut *mut CRYPT_PROPERTY_REF,
    pub pUM: *mut CRYPT_IMAGE_REF,
    pub pKM: *mut CRYPT_IMAGE_REF,
}
impl ::core::marker::Copy for CRYPT_PROVIDER_REF {}
impl ::core::clone::Clone for CRYPT_PROVIDER_REF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PROVIDER_REFS {
    pub cProviders: u32,
    pub rgpProviders: *mut *mut CRYPT_PROVIDER_REF,
}
impl ::core::marker::Copy for CRYPT_PROVIDER_REFS {}
impl ::core::clone::Clone for CRYPT_PROVIDER_REFS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PROVIDER_REG {
    pub cAliases: u32,
    pub rgpszAliases: *mut ::windows_sys::core::PWSTR,
    pub pUM: *mut CRYPT_IMAGE_REG,
    pub pKM: *mut CRYPT_IMAGE_REG,
}
impl ::core::marker::Copy for CRYPT_PROVIDER_REG {}
impl ::core::clone::Clone for CRYPT_PROVIDER_REG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PROXY_CACHE_RETRIEVAL: u32 = 2097152u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_PSOURCE_ALGORITHM {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub EncodingParameters: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_PSOURCE_ALGORITHM {}
impl ::core::clone::Clone for CRYPT_PSOURCE_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PSTORE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_PUBKEY_ALG_OID_GROUP_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RANDOM_QUERY_STRING_RETRIEVAL: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RC2_128BIT_VERSION: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RC2_40BIT_VERSION: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RC2_56BIT_VERSION: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RC2_64BIT_VERSION: u32 = 120u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_RC2_CBC_PARAMETERS {
    pub dwVersion: u32,
    pub fIV: super::super::Foundation::BOOL,
    pub rgbIV: [u8; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_RC2_CBC_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_RC2_CBC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_RC4_KEY_STATE {
    pub Key: [u8; 16],
    pub SBox: [u8; 256],
    pub i: u8,
    pub j: u8,
}
impl ::core::marker::Copy for CRYPT_RC4_KEY_STATE {}
impl ::core::clone::Clone for CRYPT_RC4_KEY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RDN_ATTR_OID_GROUP_ID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_READ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_REGISTER_FIRST_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_REGISTER_LAST_INDEX: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_RETRIEVE_AUX_INFO {
    pub cbSize: u32,
    pub pLastSyncTime: *mut super::super::Foundation::FILETIME,
    pub dwMaxUrlRetrievalByteCount: u32,
    pub pPreFetchInfo: *mut CRYPTNET_URL_CACHE_PRE_FETCH_INFO,
    pub pFlushInfo: *mut CRYPTNET_URL_CACHE_FLUSH_INFO,
    pub ppResponseInfo: *mut *mut CRYPTNET_URL_CACHE_RESPONSE_INFO,
    pub pwszCacheFileNamePrefix: ::windows_sys::core::PWSTR,
    pub pftCacheResync: *mut super::super::Foundation::FILETIME,
    pub fProxyCacheRetrieval: super::super::Foundation::BOOL,
    pub dwHttpStatusCode: u32,
    pub ppwszErrorResponseHeaders: *mut ::windows_sys::core::PWSTR,
    pub ppErrorContentBlob: *mut *mut CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_RETRIEVE_AUX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_RETRIEVE_AUX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RETRIEVE_MAX_ERROR_CONTENT_LENGTH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_RETRIEVE_MULTIPLE_OBJECTS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_RSAES_OAEP_PARAMETERS {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub MaskGenAlgorithm: CRYPT_MASK_GEN_ALGORITHM,
    pub PSourceAlgorithm: CRYPT_PSOURCE_ALGORITHM,
}
impl ::core::marker::Copy for CRYPT_RSAES_OAEP_PARAMETERS {}
impl ::core::clone::Clone for CRYPT_RSAES_OAEP_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_RSA_SSA_PSS_PARAMETERS {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub MaskGenAlgorithm: CRYPT_MASK_GEN_ALGORITHM,
    pub dwSaltLength: u32,
    pub dwTrailerField: u32,
}
impl ::core::marker::Copy for CRYPT_RSA_SSA_PSS_PARAMETERS {}
impl ::core::clone::Clone for CRYPT_RSA_SSA_PSS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SECRETDIGEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SEC_DESCR: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_SEQUENCE_OF_ANY {
    pub cValue: u32,
    pub rgValue: *mut CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_SEQUENCE_OF_ANY {}
impl ::core::clone::Clone for CRYPT_SEQUENCE_OF_ANY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SERVER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_SET_HASH_PARAM = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const HP_HMAC_INFO: CRYPT_SET_HASH_PARAM = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const HP_HASHVAL: CRYPT_SET_HASH_PARAM = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_SET_PROV_PARAM_ID = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_CLIENT_HWND: CRYPT_SET_PROV_PARAM_ID = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_DELETEKEY: CRYPT_SET_PROV_PARAM_ID = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEYEXCHANGE_ALG: CRYPT_SET_PROV_PARAM_ID = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEYEXCHANGE_PIN: CRYPT_SET_PROV_PARAM_ID = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEYEXCHANGE_KEYSIZE: CRYPT_SET_PROV_PARAM_ID = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEYSET_SEC_DESCR: CRYPT_SET_PROV_PARAM_ID = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_PIN_PROMPT_STRING: CRYPT_SET_PROV_PARAM_ID = 44u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_ROOT_CERTSTORE: CRYPT_SET_PROV_PARAM_ID = 46u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SIGNATURE_ALG: CRYPT_SET_PROV_PARAM_ID = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SIGNATURE_PIN: CRYPT_SET_PROV_PARAM_ID = 33u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SIGNATURE_KEYSIZE: CRYPT_SET_PROV_PARAM_ID = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_UI_PROMPT: CRYPT_SET_PROV_PARAM_ID = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_USE_HARDWARE_RNG: CRYPT_SET_PROV_PARAM_ID = 38u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_USER_CERTSTORE: CRYPT_SET_PROV_PARAM_ID = 42u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SECURE_KEYEXCHANGE_PIN: CRYPT_SET_PROV_PARAM_ID = 47u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SECURE_SIGNATURE_PIN: CRYPT_SET_PROV_PARAM_ID = 48u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SMARTCARD_READER: CRYPT_SET_PROV_PARAM_ID = 43u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SGC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SGC_ENUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SIGN_ALG_OID_GROUP_ID: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_SIGN_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgEncodingType: u32,
    pub pSigningCert: *const CERT_CONTEXT,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub pvHashAuxInfo: *mut ::core::ffi::c_void,
    pub cMsgCert: u32,
    pub rgpMsgCert: *mut *mut CERT_CONTEXT,
    pub cMsgCrl: u32,
    pub rgpMsgCrl: *mut *mut CRL_CONTEXT,
    pub cAuthAttr: u32,
    pub rgAuthAttr: *mut CRYPT_ATTRIBUTE,
    pub cUnauthAttr: u32,
    pub rgUnauthAttr: *mut CRYPT_ATTRIBUTE,
    pub dwFlags: u32,
    pub dwInnerContentType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_SIGN_MESSAGE_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_SIGN_MESSAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SILENT: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_SMART_CARD_ROOT_INFO {
    pub rgbCardID: [u8; 16],
    pub luid: ROOT_INFO_LUID,
}
impl ::core::marker::Copy for CRYPT_SMART_CARD_ROOT_INFO {}
impl ::core::clone::Clone for CRYPT_SMART_CARD_ROOT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_SMIME_CAPABILITIES {
    pub cCapability: u32,
    pub rgCapability: *mut CRYPT_SMIME_CAPABILITY,
}
impl ::core::marker::Copy for CRYPT_SMIME_CAPABILITIES {}
impl ::core::clone::Clone for CRYPT_SMIME_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_SMIME_CAPABILITY {
    pub pszObjId: ::windows_sys::core::PSTR,
    pub Parameters: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_SMIME_CAPABILITY {}
impl ::core::clone::Clone for CRYPT_SMIME_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SORTED_CTL_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STICKY_CACHE_RETRIEVAL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_STRING = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_BASE64HEADER: CRYPT_STRING = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_BASE64: CRYPT_STRING = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_BINARY: CRYPT_STRING = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_BASE64REQUESTHEADER: CRYPT_STRING = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_HEX: CRYPT_STRING = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_HEXASCII: CRYPT_STRING = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_BASE64X509CRLHEADER: CRYPT_STRING = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_HEXADDR: CRYPT_STRING = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_HEXASCIIADDR: CRYPT_STRING = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_HEXRAW: CRYPT_STRING = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_STRICT: CRYPT_STRING = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_BASE64_ANY: CRYPT_STRING = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_ANY: CRYPT_STRING = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_HEX_ANY: CRYPT_STRING = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_BASE64URI: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_ENCODEMASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_HASHDATA: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_NOCR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_NOCRLF: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_PERCENTESCAPE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_RESERVED100: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_STRING_RESERVED200: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_SUCCEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_TEMPLATE_OID_GROUP_ID: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_TIMESTAMP_ACCURACY {
    pub dwSeconds: u32,
    pub dwMillis: u32,
    pub dwMicros: u32,
}
impl ::core::marker::Copy for CRYPT_TIMESTAMP_ACCURACY {}
impl ::core::clone::Clone for CRYPT_TIMESTAMP_ACCURACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_CONTEXT {
    pub cbEncoded: u32,
    pub pbEncoded: *mut u8,
    pub pTimeStamp: *mut CRYPT_TIMESTAMP_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_TIMESTAMP_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_TIMESTAMP_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_INFO {
    pub dwVersion: u32,
    pub pszTSAPolicyId: ::windows_sys::core::PSTR,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashedMessage: CRYPTOAPI_BLOB,
    pub SerialNumber: CRYPTOAPI_BLOB,
    pub ftTime: super::super::Foundation::FILETIME,
    pub pvAccuracy: *mut CRYPT_TIMESTAMP_ACCURACY,
    pub fOrdering: super::super::Foundation::BOOL,
    pub Nonce: CRYPTOAPI_BLOB,
    pub Tsa: CRYPTOAPI_BLOB,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_TIMESTAMP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_TIMESTAMP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_PARA {
    pub pszTSAPolicyId: ::windows_sys::core::PCSTR,
    pub fRequestCerts: super::super::Foundation::BOOL,
    pub Nonce: CRYPTOAPI_BLOB,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_TIMESTAMP_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_TIMESTAMP_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_REQUEST {
    pub dwVersion: CRYPT_TIMESTAMP_VERSION,
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub HashedMessage: CRYPTOAPI_BLOB,
    pub pszTSAPolicyId: ::windows_sys::core::PSTR,
    pub Nonce: CRYPTOAPI_BLOB,
    pub fCertReq: super::super::Foundation::BOOL,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_TIMESTAMP_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_TIMESTAMP_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_TIMESTAMP_RESPONSE {
    pub dwStatus: CRYPT_TIMESTAMP_RESPONSE_STATUS,
    pub cFreeText: u32,
    pub rgFreeText: *mut ::windows_sys::core::PWSTR,
    pub FailureInfo: CRYPT_BIT_BLOB,
    pub ContentInfo: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_TIMESTAMP_RESPONSE {}
impl ::core::clone::Clone for CRYPT_TIMESTAMP_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_TIMESTAMP_RESPONSE_STATUS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_STATUS_GRANTED: CRYPT_TIMESTAMP_RESPONSE_STATUS = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_STATUS_GRANTED_WITH_MODS: CRYPT_TIMESTAMP_RESPONSE_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_STATUS_REJECTED: CRYPT_TIMESTAMP_RESPONSE_STATUS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_STATUS_WAITING: CRYPT_TIMESTAMP_RESPONSE_STATUS = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_STATUS_REVOCATION_WARNING: CRYPT_TIMESTAMP_RESPONSE_STATUS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_STATUS_REVOKED: CRYPT_TIMESTAMP_RESPONSE_STATUS = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_TIMESTAMP_VERSION = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_VERSION: CRYPT_TIMESTAMP_VERSION = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_TIME_STAMP_REQUEST_INFO {
    pub pszTimeStampAlgorithm: ::windows_sys::core::PSTR,
    pub pszContentType: ::windows_sys::core::PSTR,
    pub Content: CRYPTOAPI_BLOB,
    pub cAttribute: u32,
    pub rgAttribute: *mut CRYPT_ATTRIBUTE,
}
impl ::core::marker::Copy for CRYPT_TIME_STAMP_REQUEST_INFO {}
impl ::core::clone::Clone for CRYPT_TIME_STAMP_REQUEST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_TYPE2_FORMAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_UI_PROMPT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_UNICODE_NAME_DECODE_DISABLE_IE4_UTF8_FLAG: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_UNICODE_NAME_ENCODE_FORCE_UTF8_UNICODE_FLAG: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_UPDATE_KEY: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_URL_ARRAY {
    pub cUrl: u32,
    pub rgwszUrl: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_URL_ARRAY {}
impl ::core::clone::Clone for CRYPT_URL_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_URL_INFO {
    pub cbSize: u32,
    pub dwSyncDeltaTime: u32,
    pub cGroup: u32,
    pub rgcGroupEntry: *mut u32,
}
impl ::core::marker::Copy for CRYPT_URL_INFO {}
impl ::core::clone::Clone for CRYPT_URL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_USERDATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_USER_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_USER_PROTECTED_STRONG: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFYCONTEXT: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_VERIFY_CERT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_DISABLE_MD2_MD4_FLAG: CRYPT_VERIFY_CERT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_SET_STRONG_PROPERTIES_FLAG: CRYPT_VERIFY_CERT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_RETURN_STRONG_PROPERTIES_FLAG: CRYPT_VERIFY_CERT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_CHECK_WEAK_HASH_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CHAIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_NULL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_PUBKEY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    pub CertSignHashCNGAlgPropData: CRYPTOAPI_BLOB,
    pub CertIssuerPubKeyBitLengthPropData: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {}
impl ::core::clone::Clone for CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_BLOB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CRL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_OCSP_BASIC_SIGNED_RESPONSE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    pub cCNGHashAlgid: u32,
    pub rgpwszCNGHashAlgid: *mut ::windows_sys::core::PWSTR,
    pub dwWeakIndex: u32,
}
impl ::core::marker::Copy for CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {}
impl ::core::clone::Clone for CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_CONTEXT_SIGNATURE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_VERIFY_DATA_HASH: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_VERIFY_MESSAGE_PARA {
    pub cbSize: u32,
    pub dwMsgAndCertEncodingType: u32,
    pub hCryptProv: HCRYPTPROV_LEGACY,
    pub pfnGetSignerCertificate: PFN_CRYPT_GET_SIGNER_CERTIFICATE,
    pub pvGetArg: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_VERIFY_MESSAGE_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_VERIFY_MESSAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_WIRE_ONLY_RETRIEVAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_WRITE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_X931_FORMAT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_X942_COUNTER_BYTE_LENGTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_X942_KEY_LENGTH_BYTE_LENGTH: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_X942_OTHER_INFO {
    pub pszContentEncryptionObjId: ::windows_sys::core::PSTR,
    pub rgbCounter: [u8; 4],
    pub rgbKeyLength: [u8; 4],
    pub PubInfo: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_X942_OTHER_INFO {}
impl ::core::clone::Clone for CRYPT_X942_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_ALGORITHM {
    pub cbSize: u32,
    pub wszAlgorithm: ::windows_sys::core::PCWSTR,
    pub Encoded: CRYPT_XML_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_ALGORITHM {}
impl ::core::clone::Clone for CRYPT_XML_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_ALGORITHM_INFO {
    pub cbSize: u32,
    pub wszAlgorithmURI: ::windows_sys::core::PWSTR,
    pub wszName: ::windows_sys::core::PWSTR,
    pub dwGroupId: CRYPT_XML_GROUP_ID,
    pub wszCNGAlgid: ::windows_sys::core::PWSTR,
    pub wszCNGExtraAlgid: ::windows_sys::core::PWSTR,
    pub dwSignFlags: u32,
    pub dwVerifyFlags: u32,
    pub pvPaddingInfo: *mut ::core::ffi::c_void,
    pub pvExtraInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_XML_ALGORITHM_INFO {}
impl ::core::clone::Clone for CRYPT_XML_ALGORITHM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_ALGID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_SIGN_ALGID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_URI: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_BLOB {
    pub dwCharset: CRYPT_XML_CHARSET,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for CRYPT_XML_BLOB {}
impl ::core::clone::Clone for CRYPT_XML_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_BLOB_MAX: u32 = 2147483640u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_CHARSET = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_CHARSET_AUTO: CRYPT_XML_CHARSET = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_CHARSET_UTF8: CRYPT_XML_CHARSET = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_CHARSET_UTF16LE: CRYPT_XML_CHARSET = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_CHARSET_UTF16BE: CRYPT_XML_CHARSET = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_CRYPTOGRAPHIC_INTERFACE {
    pub cbSize: u32,
    pub fpCryptXmlEncodeAlgorithm: CryptXmlDllEncodeAlgorithm,
    pub fpCryptXmlCreateDigest: CryptXmlDllCreateDigest,
    pub fpCryptXmlDigestData: CryptXmlDllDigestData,
    pub fpCryptXmlFinalizeDigest: CryptXmlDllFinalizeDigest,
    pub fpCryptXmlCloseDigest: CryptXmlDllCloseDigest,
    pub fpCryptXmlSignData: CryptXmlDllSignData,
    pub fpCryptXmlVerifySignature: CryptXmlDllVerifySignature,
    pub fpCryptXmlGetAlgorithmInfo: CryptXmlDllGetAlgorithmInfo,
}
impl ::core::marker::Copy for CRYPT_XML_CRYPTOGRAPHIC_INTERFACE {}
impl ::core::clone::Clone for CRYPT_XML_CRYPTOGRAPHIC_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_DATA_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for CRYPT_XML_DATA_BLOB {}
impl ::core::clone::Clone for CRYPT_XML_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_DATA_PROVIDER {
    pub pvCallbackState: *mut ::core::ffi::c_void,
    pub cbBufferSize: u32,
    pub pfnRead: PFN_CRYPT_XML_DATA_PROVIDER_READ,
    pub pfnClose: PFN_CRYPT_XML_DATA_PROVIDER_CLOSE,
}
impl ::core::marker::Copy for CRYPT_XML_DATA_PROVIDER {}
impl ::core::clone::Clone for CRYPT_XML_DATA_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_DIGEST_REFERENCE_DATA_TRANSFORMED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_DIGEST_VALUE_MAX: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_DOC_CTXT {
    pub cbSize: u32,
    pub hDocCtxt: *mut ::core::ffi::c_void,
    pub pTransformsConfig: *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG,
    pub cSignature: u32,
    pub rgpSignature: *mut *mut CRYPT_XML_SIGNATURE,
}
impl ::core::marker::Copy for CRYPT_XML_DOC_CTXT {}
impl ::core::clone::Clone for CRYPT_XML_DOC_CTXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_ALGORITHM: ::windows_sys::core::HRESULT = -2146885372i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_BASE: ::windows_sys::core::HRESULT = -2146885376i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_ENCODING: ::windows_sys::core::HRESULT = -2146885373i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_HANDLE: ::windows_sys::core::HRESULT = -2146885370i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_HASH_FAILED: ::windows_sys::core::HRESULT = -2146885365i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_INVALID_DIGEST: ::windows_sys::core::HRESULT = -2146885367i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_INVALID_KEYVALUE: ::windows_sys::core::HRESULT = -2146885361i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_INVALID_SIGNATURE: ::windows_sys::core::HRESULT = -2146885366i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_LARGE: ::windows_sys::core::HRESULT = -2146885375i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_LAST: ::windows_sys::core::HRESULT = -2146885358i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_NON_UNIQUE_ID: ::windows_sys::core::HRESULT = -2146885358i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_OPERATION: ::windows_sys::core::HRESULT = -2146885369i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_SIGNER: ::windows_sys::core::HRESULT = -2146885359i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_SIGN_FAILED: ::windows_sys::core::HRESULT = -2146885364i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_TOO_MANY_SIGNATURES: ::windows_sys::core::HRESULT = -2146885362i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_TOO_MANY_TRANSFORMS: ::windows_sys::core::HRESULT = -2146885374i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_TRANSFORM: ::windows_sys::core::HRESULT = -2146885371i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_UNEXPECTED_XML: ::windows_sys::core::HRESULT = -2146885360i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_UNRESOLVED_REFERENCE: ::windows_sys::core::HRESULT = -2146885368i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_E_VERIFY_FAILED: ::windows_sys::core::HRESULT = -2146885363i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_FLAG_DISABLE_EXTENSIONS: CRYPT_XML_FLAGS = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_FLAG_NO_SERIALIZE: CRYPT_XML_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_SIGN_ADD_KEYVALUE: CRYPT_XML_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_FLAG_ADD_OBJECT_CREATE_COPY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_FLAG_CREATE_REFERENCE_AS_OBJECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_FLAG_ECDSA_DSIG11: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_GROUP_ID = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_GROUP_ID_HASH: CRYPT_XML_GROUP_ID = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_GROUP_ID_SIGN: CRYPT_XML_GROUP_ID = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_ID_MAX: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_ISSUER_SERIAL {
    pub wszIssuer: ::windows_sys::core::PCWSTR,
    pub wszSerial: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for CRYPT_XML_ISSUER_SERIAL {}
impl ::core::clone::Clone for CRYPT_XML_ISSUER_SERIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_KEYINFO_PARAM {
    pub wszId: ::windows_sys::core::PCWSTR,
    pub wszKeyName: ::windows_sys::core::PCWSTR,
    pub SKI: CRYPTOAPI_BLOB,
    pub wszSubjectName: ::windows_sys::core::PCWSTR,
    pub cCertificate: u32,
    pub rgCertificate: *mut CRYPTOAPI_BLOB,
    pub cCRL: u32,
    pub rgCRL: *mut CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_KEYINFO_PARAM {}
impl ::core::clone::Clone for CRYPT_XML_KEYINFO_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_KEYINFO_SPEC = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEYINFO_SPEC_NONE: CRYPT_XML_KEYINFO_SPEC = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEYINFO_SPEC_ENCODED: CRYPT_XML_KEYINFO_SPEC = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEYINFO_SPEC_PARAM: CRYPT_XML_KEYINFO_SPEC = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_KEYINFO_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEYINFO_TYPE_KEYNAME: CRYPT_XML_KEYINFO_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEYINFO_TYPE_KEYVALUE: CRYPT_XML_KEYINFO_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEYINFO_TYPE_RETRIEVAL: CRYPT_XML_KEYINFO_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEYINFO_TYPE_X509DATA: CRYPT_XML_KEYINFO_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEYINFO_TYPE_CUSTOM: CRYPT_XML_KEYINFO_TYPE = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_KEY_DSA_KEY_VALUE {
    pub P: CRYPT_XML_DATA_BLOB,
    pub Q: CRYPT_XML_DATA_BLOB,
    pub G: CRYPT_XML_DATA_BLOB,
    pub Y: CRYPT_XML_DATA_BLOB,
    pub J: CRYPT_XML_DATA_BLOB,
    pub Seed: CRYPT_XML_DATA_BLOB,
    pub Counter: CRYPT_XML_DATA_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_KEY_DSA_KEY_VALUE {}
impl ::core::clone::Clone for CRYPT_XML_KEY_DSA_KEY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    pub wszNamedCurve: ::windows_sys::core::PCWSTR,
    pub X: CRYPT_XML_DATA_BLOB,
    pub Y: CRYPT_XML_DATA_BLOB,
    pub ExplicitPara: CRYPT_XML_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_KEY_ECDSA_KEY_VALUE {}
impl ::core::clone::Clone for CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_KEY_INFO {
    pub cbSize: u32,
    pub wszId: ::windows_sys::core::PCWSTR,
    pub cKeyInfo: u32,
    pub rgKeyInfo: *mut CRYPT_XML_KEY_INFO_ITEM,
    pub hVerifyKey: BCRYPT_KEY_HANDLE,
}
impl ::core::marker::Copy for CRYPT_XML_KEY_INFO {}
impl ::core::clone::Clone for CRYPT_XML_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_KEY_INFO_ITEM {
    pub dwType: CRYPT_XML_KEYINFO_TYPE,
    pub Anonymous: CRYPT_XML_KEY_INFO_ITEM_0,
}
impl ::core::marker::Copy for CRYPT_XML_KEY_INFO_ITEM {}
impl ::core::clone::Clone for CRYPT_XML_KEY_INFO_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CRYPT_XML_KEY_INFO_ITEM_0 {
    pub wszKeyName: ::windows_sys::core::PCWSTR,
    pub KeyValue: CRYPT_XML_KEY_VALUE,
    pub RetrievalMethod: CRYPT_XML_BLOB,
    pub X509Data: CRYPT_XML_X509DATA,
    pub Custom: CRYPT_XML_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_KEY_INFO_ITEM_0 {}
impl ::core::clone::Clone for CRYPT_XML_KEY_INFO_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_KEY_RSA_KEY_VALUE {
    pub Modulus: CRYPT_XML_DATA_BLOB,
    pub Exponent: CRYPT_XML_DATA_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_KEY_RSA_KEY_VALUE {}
impl ::core::clone::Clone for CRYPT_XML_KEY_RSA_KEY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_KEY_VALUE {
    pub dwType: CRYPT_XML_KEY_VALUE_TYPE,
    pub Anonymous: CRYPT_XML_KEY_VALUE_0,
}
impl ::core::marker::Copy for CRYPT_XML_KEY_VALUE {}
impl ::core::clone::Clone for CRYPT_XML_KEY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CRYPT_XML_KEY_VALUE_0 {
    pub DSAKeyValue: CRYPT_XML_KEY_DSA_KEY_VALUE,
    pub RSAKeyValue: CRYPT_XML_KEY_RSA_KEY_VALUE,
    pub ECDSAKeyValue: CRYPT_XML_KEY_ECDSA_KEY_VALUE,
    pub Custom: CRYPT_XML_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_KEY_VALUE_0 {}
impl ::core::clone::Clone for CRYPT_XML_KEY_VALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_KEY_VALUE_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEY_VALUE_TYPE_DSA: CRYPT_XML_KEY_VALUE_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEY_VALUE_TYPE_RSA: CRYPT_XML_KEY_VALUE_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEY_VALUE_TYPE_ECDSA: CRYPT_XML_KEY_VALUE_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_KEY_VALUE_TYPE_CUSTOM: CRYPT_XML_KEY_VALUE_TYPE = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_OBJECT {
    pub cbSize: u32,
    pub hObject: *mut ::core::ffi::c_void,
    pub wszId: ::windows_sys::core::PCWSTR,
    pub wszMimeType: ::windows_sys::core::PCWSTR,
    pub wszEncoding: ::windows_sys::core::PCWSTR,
    pub Manifest: CRYPT_XML_REFERENCES,
    pub Encoded: CRYPT_XML_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_OBJECT {}
impl ::core::clone::Clone for CRYPT_XML_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_OBJECTS_MAX: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_PROPERTY {
    pub dwPropId: CRYPT_XML_PROPERTY_ID,
    pub pvValue: *const ::core::ffi::c_void,
    pub cbValue: u32,
}
impl ::core::marker::Copy for CRYPT_XML_PROPERTY {}
impl ::core::clone::Clone for CRYPT_XML_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_PROPERTY_MAX_HEAP_SIZE: CRYPT_XML_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_PROPERTY_SIGNATURE_LOCATION: CRYPT_XML_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_PROPERTY_MAX_SIGNATURES: CRYPT_XML_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_PROPERTY_DOC_DECLARATION: CRYPT_XML_PROPERTY_ID = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_PROPERTY_XML_OUTPUT_CHARSET: CRYPT_XML_PROPERTY_ID = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_REFERENCE {
    pub cbSize: u32,
    pub hReference: *mut ::core::ffi::c_void,
    pub wszId: ::windows_sys::core::PCWSTR,
    pub wszUri: ::windows_sys::core::PCWSTR,
    pub wszType: ::windows_sys::core::PCWSTR,
    pub DigestMethod: CRYPT_XML_ALGORITHM,
    pub DigestValue: CRYPTOAPI_BLOB,
    pub cTransform: u32,
    pub rgTransform: *mut CRYPT_XML_ALGORITHM,
}
impl ::core::marker::Copy for CRYPT_XML_REFERENCE {}
impl ::core::clone::Clone for CRYPT_XML_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_REFERENCES {
    pub cReference: u32,
    pub rgpReference: *mut *mut CRYPT_XML_REFERENCE,
}
impl ::core::marker::Copy for CRYPT_XML_REFERENCES {}
impl ::core::clone::Clone for CRYPT_XML_REFERENCES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_REFERENCES_MAX: u32 = 32760u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_SIGNATURE {
    pub cbSize: u32,
    pub hSignature: *mut ::core::ffi::c_void,
    pub wszId: ::windows_sys::core::PCWSTR,
    pub SignedInfo: CRYPT_XML_SIGNED_INFO,
    pub SignatureValue: CRYPTOAPI_BLOB,
    pub pKeyInfo: *mut CRYPT_XML_KEY_INFO,
    pub cObject: u32,
    pub rgpObject: *mut *mut CRYPT_XML_OBJECT,
}
impl ::core::marker::Copy for CRYPT_XML_SIGNATURE {}
impl ::core::clone::Clone for CRYPT_XML_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_SIGNATURES_MAX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_SIGNATURE_VALUE_MAX: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_SIGNED_INFO {
    pub cbSize: u32,
    pub wszId: ::windows_sys::core::PCWSTR,
    pub Canonicalization: CRYPT_XML_ALGORITHM,
    pub SignatureMethod: CRYPT_XML_ALGORITHM,
    pub cReference: u32,
    pub rgpReference: *mut *mut CRYPT_XML_REFERENCE,
    pub Encoded: CRYPT_XML_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_SIGNED_INFO {}
impl ::core::clone::Clone for CRYPT_XML_SIGNED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_STATUS {
    pub cbSize: u32,
    pub dwErrorStatus: CRYPT_XML_STATUS_ERROR_STATUS,
    pub dwInfoStatus: CRYPT_XML_STATUS_INFO_STATUS,
}
impl ::core::marker::Copy for CRYPT_XML_STATUS {}
impl ::core::clone::Clone for CRYPT_XML_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_STATUS_ERROR_STATUS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_ERROR_NOT_RESOLVED: CRYPT_XML_STATUS_ERROR_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_ERROR_DIGEST_INVALID: CRYPT_XML_STATUS_ERROR_STATUS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_ALGORITHM: CRYPT_XML_STATUS_ERROR_STATUS = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_TRANSFORM: CRYPT_XML_STATUS_ERROR_STATUS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_ERROR_SIGNATURE_INVALID: CRYPT_XML_STATUS_ERROR_STATUS = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_ERROR_KEYINFO_NOT_PARSED: CRYPT_XML_STATUS_ERROR_STATUS = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_STATUS_INFO_STATUS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_INTERNAL_REFERENCE: CRYPT_XML_STATUS_INFO_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_KEY_AVAILABLE: CRYPT_XML_STATUS_INFO_STATUS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_DIGESTING: CRYPT_XML_STATUS_INFO_STATUS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_DIGEST_VALID: CRYPT_XML_STATUS_INFO_STATUS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_SIGNATURE_VALID: CRYPT_XML_STATUS_INFO_STATUS = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_OPENED_TO_ENCODE: CRYPT_XML_STATUS_INFO_STATUS = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_STATUS_NO_ERROR: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    pub cbSize: u32,
    pub cTransformInfo: u32,
    pub rgpTransformInfo: *mut *mut CRYPT_XML_TRANSFORM_INFO,
}
impl ::core::marker::Copy for CRYPT_XML_TRANSFORM_CHAIN_CONFIG {}
impl ::core::clone::Clone for CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_TRANSFORM_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_TRANSFORM_ON_STREAM: CRYPT_XML_TRANSFORM_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_TRANSFORM_ON_NODESET: CRYPT_XML_TRANSFORM_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_TRANSFORM_URI_QUERY_STRING: CRYPT_XML_TRANSFORM_FLAGS = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_TRANSFORM_INFO {
    pub cbSize: u32,
    pub wszAlgorithm: ::windows_sys::core::PCWSTR,
    pub cbBufferSize: u32,
    pub dwFlags: CRYPT_XML_TRANSFORM_FLAGS,
    pub pfnCreateTransform: PFN_CRYPT_XML_CREATE_TRANSFORM,
}
impl ::core::marker::Copy for CRYPT_XML_TRANSFORM_INFO {}
impl ::core::clone::Clone for CRYPT_XML_TRANSFORM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_TRANSFORM_MAX: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_X509DATA {
    pub cX509Data: u32,
    pub rgX509Data: *mut CRYPT_XML_X509DATA_ITEM,
}
impl ::core::marker::Copy for CRYPT_XML_X509DATA {}
impl ::core::clone::Clone for CRYPT_XML_X509DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CRYPT_XML_X509DATA_ITEM {
    pub dwType: CRYPT_XML_X509DATA_TYPE,
    pub Anonymous: CRYPT_XML_X509DATA_ITEM_0,
}
impl ::core::marker::Copy for CRYPT_XML_X509DATA_ITEM {}
impl ::core::clone::Clone for CRYPT_XML_X509DATA_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union CRYPT_XML_X509DATA_ITEM_0 {
    pub IssuerSerial: CRYPT_XML_ISSUER_SERIAL,
    pub SKI: CRYPT_XML_DATA_BLOB,
    pub wszSubjectName: ::windows_sys::core::PCWSTR,
    pub Certificate: CRYPT_XML_DATA_BLOB,
    pub CRL: CRYPT_XML_DATA_BLOB,
    pub Custom: CRYPT_XML_BLOB,
}
impl ::core::marker::Copy for CRYPT_XML_X509DATA_ITEM_0 {}
impl ::core::clone::Clone for CRYPT_XML_X509DATA_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CRYPT_XML_X509DATA_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_X509DATA_TYPE_ISSUER_SERIAL: CRYPT_XML_X509DATA_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_X509DATA_TYPE_SKI: CRYPT_XML_X509DATA_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_X509DATA_TYPE_SUBJECT_NAME: CRYPT_XML_X509DATA_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_X509DATA_TYPE_CERTIFICATE: CRYPT_XML_X509DATA_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_X509DATA_TYPE_CRL: CRYPT_XML_X509DATA_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CRYPT_XML_X509DATA_TYPE_CUSTOM: CRYPT_XML_X509DATA_TYPE = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CTL_ANY_SUBJECT_INFO {
    pub SubjectAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub SubjectIdentifier: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for CTL_ANY_SUBJECT_INFO {}
impl ::core::clone::Clone for CTL_ANY_SUBJECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_ANY_SUBJECT_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_CERT_SUBJECT_TYPE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_CONTEXT {
    pub dwMsgAndCertEncodingType: u32,
    pub pbCtlEncoded: *mut u8,
    pub cbCtlEncoded: u32,
    pub pCtlInfo: *mut CTL_INFO,
    pub hCertStore: HCERTSTORE,
    pub hCryptMsg: *mut ::core::ffi::c_void,
    pub pbCtlContent: *mut u8,
    pub cbCtlContent: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CTL_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CTL_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CTL_ENTRY {
    pub SubjectIdentifier: CRYPTOAPI_BLOB,
    pub cAttribute: u32,
    pub rgAttribute: *mut CRYPT_ATTRIBUTE,
}
impl ::core::marker::Copy for CTL_ENTRY {}
impl ::core::clone::Clone for CTL_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_ENTRY_FROM_PROP_CHAIN_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_FIND_NO_LIST_ID_CBDATA: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_FIND_SUBJECT_PARA {
    pub cbSize: u32,
    pub pUsagePara: *mut CTL_FIND_USAGE_PARA,
    pub dwSubjectType: u32,
    pub pvSubject: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CTL_FIND_SUBJECT_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CTL_FIND_SUBJECT_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_FIND_USAGE_PARA {
    pub cbSize: u32,
    pub SubjectUsage: CTL_USAGE,
    pub ListIdentifier: CRYPTOAPI_BLOB,
    pub pSigner: *mut CERT_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CTL_FIND_USAGE_PARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CTL_FIND_USAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_INFO {
    pub dwVersion: u32,
    pub SubjectUsage: CTL_USAGE,
    pub ListIdentifier: CRYPTOAPI_BLOB,
    pub SequenceNumber: CRYPTOAPI_BLOB,
    pub ThisUpdate: super::super::Foundation::FILETIME,
    pub NextUpdate: super::super::Foundation::FILETIME,
    pub SubjectAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub cCTLEntry: u32,
    pub rgCTLEntry: *mut CTL_ENTRY,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CTL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CTL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CTL_USAGE {
    pub cUsageIdentifier: u32,
    pub rgpszUsageIdentifier: *mut ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for CTL_USAGE {}
impl ::core::clone::Clone for CTL_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CTL_USAGE_MATCH {
    pub dwType: u32,
    pub Usage: CTL_USAGE,
}
impl ::core::marker::Copy for CTL_USAGE_MATCH {}
impl ::core::clone::Clone for CTL_USAGE_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CTL_V1: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct CTL_VERIFY_USAGE_PARA {
    pub cbSize: u32,
    pub ListIdentifier: CRYPTOAPI_BLOB,
    pub cCtlStore: u32,
    pub rghCtlStore: *mut HCERTSTORE,
    pub cSignerStore: u32,
    pub rghSignerStore: *mut HCERTSTORE,
}
impl ::core::marker::Copy for CTL_VERIFY_USAGE_PARA {}
impl ::core::clone::Clone for CTL_VERIFY_USAGE_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_VERIFY_USAGE_STATUS {
    pub cbSize: u32,
    pub dwError: u32,
    pub dwFlags: u32,
    pub ppCtl: *mut *mut CTL_CONTEXT,
    pub dwCtlEntryIndex: u32,
    pub ppSigner: *mut *mut CERT_CONTEXT,
    pub dwSignerIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CTL_VERIFY_USAGE_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CTL_VERIFY_USAGE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const CUR_BLOB_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CertKeyType = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypeOther: CertKeyType = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypeVirtualSmartCard: CertKeyType = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypePhysicalSmartCard: CertKeyType = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypePassport: CertKeyType = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypePassportRemote: CertKeyType = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypePassportSmartCard: CertKeyType = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypeHardware: CertKeyType = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypeSoftware: CertKeyType = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KeyTypeSelfSigned: CertKeyType = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllCloseDigest = ::core::option::Option<unsafe extern "system" fn(hdigest: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllCreateDigest = ::core::option::Option<unsafe extern "system" fn(pdigestmethod: *const CRYPT_XML_ALGORITHM, pcbsize: *mut u32, phdigest: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllCreateKey = ::core::option::Option<unsafe extern "system" fn(pencoded: *const CRYPT_XML_BLOB, phkey: *mut BCRYPT_KEY_HANDLE) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllDigestData = ::core::option::Option<unsafe extern "system" fn(hdigest: *const ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllEncodeAlgorithm = ::core::option::Option<unsafe extern "system" fn(palginfo: *const CRYPT_XML_ALGORITHM_INFO, dwcharset: CRYPT_XML_CHARSET, pvcallbackstate: *mut ::core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllEncodeKeyValue = ::core::option::Option<unsafe extern "system" fn(hkey: NCRYPT_KEY_HANDLE, dwcharset: CRYPT_XML_CHARSET, pvcallbackstate: *mut ::core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllFinalizeDigest = ::core::option::Option<unsafe extern "system" fn(hdigest: *const ::core::ffi::c_void, pbdigest: *mut u8, cbdigest: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllGetAlgorithmInfo = ::core::option::Option<unsafe extern "system" fn(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, ppalginfo: *mut *mut CRYPT_XML_ALGORITHM_INFO) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllGetInterface = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, pmethod: *const CRYPT_XML_ALGORITHM_INFO, pinterface: *mut CRYPT_XML_CRYPTOGRAPHIC_INTERFACE) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllSignData = ::core::option::Option<unsafe extern "system" fn(psignaturemethod: *const CRYPT_XML_ALGORITHM, hcryptprovorncryptkey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: u32, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type CryptXmlDllVerifySignature = ::core::option::Option<unsafe extern "system" fn(psignaturemethod: *const CRYPT_XML_ALGORITHM, hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, pbsignature: *const u8, cbsignature: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type DSAFIPSVERSION_ENUM = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const DSA_FIPS186_2: DSAFIPSVERSION_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const DSA_FIPS186_3: DSAFIPSVERSION_ENUM = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct DSSSEED {
    pub counter: u32,
    pub seed: [u8; 20],
}
impl ::core::marker::Copy for DSSSEED {}
impl ::core::clone::Clone for DSSSEED {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type Direction = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const DirectionEncrypt: Direction = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const DirectionDecrypt: Direction = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ECC_CMS_SHARED_INFO: ::windows_sys::core::PCSTR = 77i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type ECC_CURVE_ALG_ID_ENUM = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_NO_CURVE_GENERATION_ALG_ID: ECC_CURVE_ALG_ID_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type ECC_CURVE_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_PRIME_SHORT_WEIERSTRASS_CURVE: ECC_CURVE_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_PRIME_TWISTED_EDWARDS_CURVE: ECC_CURVE_TYPE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_ECC_PRIME_MONTGOMERY_CURVE: ECC_CURVE_TYPE_ENUM = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct ENDPOINTADDRESS {
    pub serviceUrl: ::windows_sys::core::PCWSTR,
    pub policyUrl: ::windows_sys::core::PCWSTR,
    pub rawCertificate: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for ENDPOINTADDRESS {}
impl ::core::clone::Clone for ENDPOINTADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct ENDPOINTADDRESS2 {
    pub serviceUrl: ::windows_sys::core::PCWSTR,
    pub policyUrl: ::windows_sys::core::PCWSTR,
    pub identityType: u32,
    pub identityBytes: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for ENDPOINTADDRESS2 {}
impl ::core::clone::Clone for ENDPOINTADDRESS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwRootProgramQualifierFlags: CERT_ROOT_PROGRAM_FLAGS,
}
impl ::core::marker::Copy for EV_EXTRA_CERT_CHAIN_POLICY_PARA {}
impl ::core::clone::Clone for EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwQualifiers: u32,
    pub dwIssuanceUsageIndex: u32,
}
impl ::core::marker::Copy for EV_EXTRA_CERT_CHAIN_POLICY_STATUS {}
impl ::core::clone::Clone for EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const EXPORT_PRIVATE_KEYS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const EXPO_OFFLOAD_FUNC_NAME: &str = "OffloadModExpo";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const EXPO_OFFLOAD_REG_VALUE: &str = "ExpoOffload";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_ARGUMENT: ::windows_sys::core::HRESULT = -1073413883i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_COMMUNICATION: ::windows_sys::core::HRESULT = -1073413888i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_DATA_ACCESS: ::windows_sys::core::HRESULT = -1073413887i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_EXPORT: ::windows_sys::core::HRESULT = -1073413886i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_FAIL: ::windows_sys::core::HRESULT = -1073413867i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_FAILED_REQUIRED_CLAIMS: ::windows_sys::core::HRESULT = -1073413756i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_IDENTITY: ::windows_sys::core::HRESULT = -1073413885i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_IMPORT: ::windows_sys::core::HRESULT = -1073413884i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_INFORMATIONCARD: ::windows_sys::core::HRESULT = -1073413881i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_INVALID_PROOF_KEY: ::windows_sys::core::HRESULT = -1073413758i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_LOGOVALIDATION: ::windows_sys::core::HRESULT = -1073413879i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_MISSING_APPLIESTO: ::windows_sys::core::HRESULT = -1073413759i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_PASSWORDVALIDATION: ::windows_sys::core::HRESULT = -1073413878i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_POLICY: ::windows_sys::core::HRESULT = -1073413877i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_PROCESSDIED: ::windows_sys::core::HRESULT = -1073413876i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_REFRESH_REQUIRED: ::windows_sys::core::HRESULT = -1073413760i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_REQUEST: ::windows_sys::core::HRESULT = -1073413882i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_SERVICE: ::windows_sys::core::HRESULT = -1073413874i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_SERVICEBUSY: ::windows_sys::core::HRESULT = -1073413875i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_SHUTTINGDOWN: ::windows_sys::core::HRESULT = -1073413873i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_STOREKEY: ::windows_sys::core::HRESULT = -1073413880i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_STORE_IMPORT: ::windows_sys::core::HRESULT = -1073413868i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_TOKENCREATION: ::windows_sys::core::HRESULT = -1073413872i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_TRUSTEXCHANGE: ::windows_sys::core::HRESULT = -1073413871i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_UI_INITIALIZATION: ::windows_sys::core::HRESULT = -1073413862i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_UNKNOWN_REFERENCE: ::windows_sys::core::HRESULT = -1073413757i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_UNTRUSTED: ::windows_sys::core::HRESULT = -1073413870i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const E_ICARD_USERCANCELLED: ::windows_sys::core::HRESULT = -1073413869i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GENERIC_XML_TOKEN {
    pub createDate: super::super::Foundation::FILETIME,
    pub expiryDate: super::super::Foundation::FILETIME,
    pub xmlToken: ::windows_sys::core::PWSTR,
    pub internalTokenReference: ::windows_sys::core::PWSTR,
    pub externalTokenReference: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GENERIC_XML_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GENERIC_XML_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type HASHALGORITHM_ENUM = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const DSA_HASH_ALGORITHM_SHA1: HASHALGORITHM_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const DSA_HASH_ALGORITHM_SHA256: HASHALGORITHM_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const DSA_HASH_ALGORITHM_SHA512: HASHALGORITHM_ENUM = 2i32;
pub type HCERTCHAINENGINE = isize;
pub type HCERTSTORE = *mut ::core::ffi::c_void;
pub type HCERTSTOREPROV = *mut ::core::ffi::c_void;
pub type HCRYPTASYNC = isize;
pub type HCRYPTPROV_LEGACY = usize;
pub type HCRYPTPROV_OR_NCRYPT_KEY_HANDLE = usize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct HMAC_Info {
    pub HashAlgid: u32,
    pub pbInnerString: *mut u8,
    pub cbInnerString: u32,
    pub pbOuterString: *mut u8,
    pub cbOuterString: u32,
}
impl ::core::marker::Copy for HMAC_Info {}
impl ::core::clone::Clone for HMAC_Info {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const HP_ALGID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const HP_HASHSIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const HP_TLS1PRF_LABEL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const HP_TLS1PRF_SEED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUTHTYPE_CLIENT: HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const AUTHTYPE_SERVER: HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct HTTPSPolicyCallbackData {
    pub Anonymous: HTTPSPolicyCallbackData_0,
    pub dwAuthType: HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE,
    pub fdwChecks: u32,
    pub pwszServerName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for HTTPSPolicyCallbackData {}
impl ::core::clone::Clone for HTTPSPolicyCallbackData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub union HTTPSPolicyCallbackData_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl ::core::marker::Copy for HTTPSPolicyCallbackData_0 {}
impl ::core::clone::Clone for HTTPSPolicyCallbackData_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
#[repr(transparent)]
pub struct HandleType(pub i32);
impl HandleType {
    pub const Asymmetric: Self = Self(1i32);
    pub const Symmetric: Self = Self(2i32);
    pub const Transform: Self = Self(3i32);
    pub const Hash: Self = Self(4i32);
}
impl ::core::marker::Copy for HandleType {}
impl ::core::clone::Clone for HandleType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ICertSrvSetup = *mut ::core::ffi::c_void;
pub type ICertSrvSetupKeyInformation = *mut ::core::ffi::c_void;
pub type ICertSrvSetupKeyInformationCollection = *mut ::core::ffi::c_void;
pub type ICertificateEnrollmentPolicyServerSetup = *mut ::core::ffi::c_void;
pub type ICertificateEnrollmentServerSetup = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const IFX_RSA_KEYGEN_VUL_NOT_AFFECTED: u32 = 0u32;
pub type IMSCEPSetup = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS {
    pub keySize: i32,
    pub keyExchangeAlgorithm: ::windows_sys::core::PWSTR,
    pub signatureAlgorithm: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS {}
impl ::core::clone::Clone for INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct INFORMATIONCARD_CRYPTO_HANDLE {
    pub r#type: HandleType,
    pub expiration: i64,
    pub cryptoParameters: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for INFORMATIONCARD_CRYPTO_HANDLE {}
impl ::core::clone::Clone for INFORMATIONCARD_CRYPTO_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INFORMATIONCARD_HASH_CRYPTO_PARAMETERS {
    pub hashSize: i32,
    pub transform: INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INFORMATIONCARD_HASH_CRYPTO_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INFORMATIONCARD_HASH_CRYPTO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS {
    pub keySize: i32,
    pub blockSize: i32,
    pub feedbackSize: i32,
}
impl ::core::marker::Copy for INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS {}
impl ::core::clone::Clone for INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS {
    pub inputBlockSize: i32,
    pub outputBlockSize: i32,
    pub canTransformMultipleBlocks: super::super::Foundation::BOOL,
    pub canReuseTransform: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const INTERNATIONAL_USAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_ALGORITHMID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_CONTEXT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_GENERIC_PARAMETER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_HASH_ALGORITHM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_HKDF_INFO: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_HKDF_SALT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_HMAC_KEY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_ITERATION_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_KEYBITLENGTH: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_LABEL: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_PARTYUINFO: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_PARTYVINFO: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_SALT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_SECRET_APPEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_SECRET_HANDLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_SECRET_PREPEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_SUPPPRIVINFO: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_SUPPPUBINFO: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_TLS_PRF_LABEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_TLS_PRF_PROTOCOL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_TLS_PRF_SEED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KDF_USE_SECRET_AS_HMAC_KEY_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KEYSTATEBLOB: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KEY_LENGTH_MASK: u32 = 4294901760u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct KEY_TYPE_SUBTYPE {
    pub dwKeySpec: u32,
    pub Type: ::windows_sys::core::GUID,
    pub Subtype: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for KEY_TYPE_SUBTYPE {}
impl ::core::clone::Clone for KEY_TYPE_SUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_ADMIN_PIN: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_CLEAR_KEY: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_CLIENT_RANDOM: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_CMS_DH_KEY_INFO: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_CMS_KEY_INFO: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_EFFECTIVE_KEYLEN: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_G: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_HIGHEST_VERSION: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_INFO: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_IV: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_KEYEXCHANGE_PIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_KEYVAL: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_MODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_MODE_BITS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_OAEP_PARAMS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_P: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PADDING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PIN_ID: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PIN_INFO: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PRECOMP_MD5: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PRECOMP_SHA: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PREHASH: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PUB_EX_LEN: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PUB_EX_VAL: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_PUB_PARAMS: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_Q: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_RA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_RB: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_ROUNDS: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_RP: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_SCHANNEL_ALG: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_SERVER_RANDOM: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_SIGNATURE_PIN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_VERIFY_PARAMS: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_X: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const KP_Y: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const LEGACY_DH_PRIVATE_BLOB: &str = "CAPIDHPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const LEGACY_DH_PUBLIC_BLOB: &str = "CAPIDHPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const LEGACY_DSA_PRIVATE_BLOB: &str = "CAPIDSAPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const LEGACY_DSA_PUBLIC_BLOB: &str = "CAPIDSAPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const LEGACY_DSA_V2_PRIVATE_BLOB: &str = "V2CAPIDSAPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const LEGACY_DSA_V2_PUBLIC_BLOB: &str = "V2CAPIDSAPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const LEGACY_RSAPRIVATE_BLOB: &str = "CAPIPRIVATEBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const LEGACY_RSAPUBLIC_BLOB: &str = "CAPIPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MAXUIDLEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_CHECK_APPLICATION_ROOT_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_DISABLE_FLIGHT_ROOT_FLAG: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_ENABLE_TEST_ROOT_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type MSCEPSetupProperty = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_USELOCALSYSTEM: MSCEPSetupProperty = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_USECHALLENGE: MSCEPSetupProperty = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_RANAME_CN: MSCEPSetupProperty = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_RANAME_EMAIL: MSCEPSetupProperty = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_RANAME_COMPANY: MSCEPSetupProperty = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_RANAME_DEPT: MSCEPSetupProperty = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_RANAME_CITY: MSCEPSetupProperty = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_RANAME_STATE: MSCEPSetupProperty = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_RANAME_COUNTRY: MSCEPSetupProperty = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_SIGNINGKEYINFORMATION: MSCEPSetupProperty = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_EXCHANGEKEYINFORMATION: MSCEPSetupProperty = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_CAINFORMATION: MSCEPSetupProperty = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_MSCEPURL: MSCEPSetupProperty = 12i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ENUM_CEPSETUPPROP_CHALLENGEURL: MSCEPSetupProperty = 13i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DH_SCHANNEL_PROV: &str = "Microsoft DH SChannel Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DH_SCHANNEL_PROV_A: &str = "Microsoft DH SChannel Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DH_SCHANNEL_PROV_W: &str = "Microsoft DH SChannel Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DSS_DH_PROV: &str = "Microsoft Base DSS and Diffie-Hellman Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DSS_DH_PROV_A: &str = "Microsoft Base DSS and Diffie-Hellman Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DSS_DH_PROV_W: &str = "Microsoft Base DSS and Diffie-Hellman Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DSS_PROV: &str = "Microsoft Base DSS Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DSS_PROV_A: &str = "Microsoft Base DSS Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_DSS_PROV_W: &str = "Microsoft Base DSS Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_PROV: &str = "Microsoft Base Cryptographic Provider v1.0";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_PROV_A: &str = "Microsoft Base Cryptographic Provider v1.0";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_PROV_W: &str = "Microsoft Base Cryptographic Provider v1.0";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_RSA_SCHANNEL_PROV: &str = "Microsoft RSA SChannel Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_RSA_SCHANNEL_PROV_A: &str = "Microsoft RSA SChannel Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_RSA_SCHANNEL_PROV_W: &str = "Microsoft RSA SChannel Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_RSA_SIG_PROV: &str = "Microsoft RSA Signature Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_RSA_SIG_PROV_A: &str = "Microsoft RSA Signature Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_DEF_RSA_SIG_PROV_W: &str = "Microsoft RSA Signature Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENHANCED_PROV: &str = "Microsoft Enhanced Cryptographic Provider v1.0";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENHANCED_PROV_A: &str = "Microsoft Enhanced Cryptographic Provider v1.0";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENHANCED_PROV_W: &str = "Microsoft Enhanced Cryptographic Provider v1.0";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_DSS_DH_PROV: &str = "Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_DSS_DH_PROV_A: &str = "Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_DSS_DH_PROV_W: &str = "Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_RSA_AES_PROV: &str = "Microsoft Enhanced RSA and AES Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_RSA_AES_PROV_A: &str = "Microsoft Enhanced RSA and AES Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_RSA_AES_PROV_W: &str = "Microsoft Enhanced RSA and AES Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_RSA_AES_PROV_XP: &str = "Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_RSA_AES_PROV_XP_A: &str = "Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_ENH_RSA_AES_PROV_XP_W: &str = "Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_KEY_PROTECTION_PROVIDER: &str = "Microsoft Key Protection Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_KEY_STORAGE_PROVIDER: &str = "Microsoft Software Key Storage Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_NGC_KEY_STORAGE_PROVIDER: &str = "Microsoft Passport Key Storage Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_PLATFORM_CRYPTO_PROVIDER: &str = "Microsoft Platform Crypto Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_PLATFORM_KEY_STORAGE_PROVIDER: &str = "Microsoft Platform Crypto Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_PRIMITIVE_PROVIDER: &str = "Microsoft Primitive Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_SCARD_PROV: &str = "Microsoft Base Smart Card Crypto Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_SCARD_PROV_A: &str = "Microsoft Base Smart Card Crypto Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_SCARD_PROV_W: &str = "Microsoft Base Smart Card Crypto Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_SMART_CARD_KEY_STORAGE_PROVIDER: &str = "Microsoft Smart Card Key Storage Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_STRONG_PROV: &str = "Microsoft Strong Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_STRONG_PROV_A: &str = "Microsoft Strong Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const MS_STRONG_PROV_W: &str = "Microsoft Strong Cryptographic Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_ATTESTATIONSTATEMENT_BLOB: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_ATTESTATION_CLAIM_CHALLENGE_REQUIRED: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_ATTESTATION_CLAIM_TYPE: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_CERT_BLOB: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_CLAIM_IDBINDING_NONCE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_CLAIM_KEYATTESTATION_NONCE: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_ECC_CURVE_NAME: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_ECC_PARAMETERS: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_EMPTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_KEY_PROPERTY_FLAGS: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PKCS_ALG_ID: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PKCS_ALG_OID: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PKCS_ALG_PARAM: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PKCS_ATTRS: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PKCS_KEY_NAME: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PKCS_OID: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PKCS_SECRET: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PROTECTION_DESCRIPTOR_STRING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_PROTECTION_FLAGS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_SSL_CLEAR_KEY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_SSL_CLIENT_RANDOM: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_SSL_HIGHEST_VERSION: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_SSL_KEY_ARG_DATA: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_SSL_SERVER_RANDOM: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_SSL_SESSION_HASH: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_NONCE: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_PCR_MASK: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_STATIC_CREATE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_TPM_SEAL_NO_DA_PROTECTION: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_TPM_SEAL_PASSWORD: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_TPM_SEAL_POLICYINFO: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_TPM_SEAL_TICKET: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPTBUFFER_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_3DES_112_ALGORITHM: &str = "3DES_112";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_3DES_ALGORITHM: &str = "3DES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_AES_ALGORITHM: &str = "AES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_AES_ALGORITHM_GROUP: &str = "AES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALGORITHM_GROUP_PROPERTY: &str = "Algorithm Group";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type NCRYPT_ALGORITHM_NAME_CLASS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: NCRYPT_ALGORITHM_NAME_CLASS = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SECRET_AGREEMENT_INTERFACE: NCRYPT_ALGORITHM_NAME_CLASS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SIGNATURE_INTERFACE: NCRYPT_ALGORITHM_NAME_CLASS = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALGORITHM_PROPERTY: &str = "Algorithm Name";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_ALLOC_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_NCRYPT_ALLOC,
    pub pfnFree: PFN_NCRYPT_FREE,
}
impl ::core::marker::Copy for NCRYPT_ALLOC_PARA {}
impl ::core::clone::Clone for NCRYPT_ALLOC_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_ALL_USAGES: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_ARCHIVING_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_DECRYPT_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_EXPORT_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_KEY_AGREEMENT_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_KEY_IMPORT_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_SIGNING_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALLOW_SILENT_KEY_ACCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ALTERNATE_KEY_STORAGE_LOCATION_PROPERTY: &str = "PCP_ALTERNATE_KEY_STORAGE_LOCATION";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ASSOCIATED_ECDH_KEY: &str = "SmartCardAssociatedECDHKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ATTESTATION_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_AUTHORITY_KEY_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_AUTH_TAG_LENGTH: &str = "AuthTagLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_BLOCK_LENGTH_PROPERTY: &str = "Block Length";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CAPI_KDF_ALGORITHM: &str = "CAPI_KDF";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CERTIFICATE_PROPERTY: &str = "SmartCardKeyCertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CHAINING_MODE_PROPERTY: &str = "Chaining Mode";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CHANGEPASSWORD_PROPERTY: &str = "PCP_CHANGEPASSWORD";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CIPHER_BLOCK_PADDING_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CIPHER_KEY_BLOB: &str = "CipherKeyBlob";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CIPHER_KEY_BLOB_MAGIC: u32 = 1380470851u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CIPHER_NO_PADDING_FLAG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CIPHER_OTHER_PADDING_FLAG: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_CIPHER_PADDING_INFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pbIV: *mut u8,
    pub cbIV: u32,
    pub pbOtherInfo: *mut u8,
    pub cbOtherInfo: u32,
}
impl ::core::marker::Copy for NCRYPT_CIPHER_PADDING_INFO {}
impl ::core::clone::Clone for NCRYPT_CIPHER_PADDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CLAIM_AUTHORITY_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CLAIM_PLATFORM: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CLAIM_SUBJECT_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CLAIM_UNKNOWN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CLAIM_VSM_KEY_ATTESTATION_STATEMENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CLAIM_WEB_AUTH_SUBJECT_ONLY: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DESCR_DELIMITER_AND: &str = "AND";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DESCR_DELIMITER_OR: &str = "OR";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DESCR_EQUAL: &str = "=";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DESX_ALGORITHM: &str = "DESX";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DES_ALGORITHM: &str = "DES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DES_ALGORITHM_GROUP: &str = "DES";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DH_ALGORITHM: &str = "DH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DH_ALGORITHM_GROUP: &str = "DH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DH_PARAMETERS_PROPERTY: &str = "DHParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DISMISS_UI_TIMEOUT_SEC_PROPERTY: &str = "SmartCardDismissUITimeoutSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DO_NOT_FINALIZE_FLAG: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DSA_ALGORITHM: &str = "DSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_DSA_ALGORITHM_GROUP: &str = "DSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECC_CURVE_NAME_LIST_PROPERTY: &str = "ECCCurveNameList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECC_CURVE_NAME_PROPERTY: &str = "ECCCurveName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECC_PARAMETERS_PROPERTY: &str = "ECCParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDH_ALGORITHM: &str = "ECDH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDH_ALGORITHM_GROUP: &str = "ECDH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDH_P256_ALGORITHM: &str = "ECDH_P256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDH_P384_ALGORITHM: &str = "ECDH_P384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDH_P521_ALGORITHM: &str = "ECDH_P521";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDSA_ALGORITHM: &str = "ECDSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDSA_ALGORITHM_GROUP: &str = "ECDSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDSA_P256_ALGORITHM: &str = "ECDSA_P256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDSA_P384_ALGORITHM: &str = "ECDSA_P384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ECDSA_P521_ALGORITHM: &str = "ECDSA_P521";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    pub Header: NCRYPT_EXPORTED_ISOLATED_KEY_HEADER,
}
impl ::core::marker::Copy for NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {}
impl ::core::clone::Clone for NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    pub Version: u32,
    pub KeyUsage: u32,
    pub _bitfield: u32,
    pub cbAlgName: u32,
    pub cbNonce: u32,
    pub cbAuthTag: u32,
    pub cbWrappingKey: u32,
    pub cbIsolatedKey: u32,
}
impl ::core::marker::Copy for NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {}
impl ::core::clone::Clone for NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_CURRENT_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_EXPORT_LEGACY_FLAG: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_EXPORT_POLICY_PROPERTY: &str = "Export Policy";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type NCRYPT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PAD_NONE: NCRYPT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PAD_OAEP: NCRYPT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PAD_PKCS1: NCRYPT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const BCRYPT_PAD_PSS: NCRYPT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SILENT_FLAG: NCRYPT_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_NO_PADDING_FLAG: NCRYPT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PAD_OAEP_FLAG: NCRYPT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PAD_PKCS1_FLAG: NCRYPT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_REGISTER_NOTIFY_FLAG: NCRYPT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_UNREGISTER_NOTIFY_FLAG: NCRYPT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MACHINE_KEY_FLAG: NCRYPT_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_UNPROTECT_NO_DECRYPT: NCRYPT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_OVERWRITE_KEY_FLAG: NCRYPT_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_NO_KEY_VALIDATION: NCRYPT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_WRITE_KEY_TO_LEGACY_STORE_FLAG: NCRYPT_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PAD_PSS_FLAG: NCRYPT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PERSIST_FLAG: NCRYPT_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PERSIST_ONLY_FLAG: NCRYPT_FLAGS = 1073741824u32;
pub type NCRYPT_HANDLE = usize;
pub type NCRYPT_HASH_HANDLE = usize;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_HMAC_SHA256_ALGORITHM: &str = "HMAC-SHA256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_IGNORE_DEVICE_STATE_FLAG: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_IMPL_HARDWARE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_IMPL_HARDWARE_RNG_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_IMPL_REMOVABLE_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_IMPL_SOFTWARE_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_IMPL_TYPE_PROPERTY: &str = "Impl Type";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_IMPL_VIRTUAL_ISOLATION_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_INITIALIZATION_VECTOR: &str = "IV";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    pub Version: u32,
    pub Flags: u32,
    pub cbPublicKeyBlob: u32,
}
impl ::core::marker::Copy for NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {}
impl ::core::clone::Clone for NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_CURRENT_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ISOLATED_KEY_ENVELOPE_BLOB: &str = "ISOLATED_KEY_ENVELOPE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ISOLATED_KEY_FLAG_CREATED_IN_ISOLATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ISOLATED_KEY_FLAG_IMPORT_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KDF_KEY_BLOB: &str = "KDFKeyBlob";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KDF_KEY_BLOB_MAGIC: u32 = 826688587u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KDF_SECRET_VALUE: &str = "KDFKeySecret";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_KEY_ACCESS_POLICY_BLOB {
    pub dwVersion: u32,
    pub dwPolicyFlags: u32,
    pub cbUserSid: u32,
    pub cbApplicationSid: u32,
}
impl ::core::marker::Copy for NCRYPT_KEY_ACCESS_POLICY_BLOB {}
impl ::core::clone::Clone for NCRYPT_KEY_ACCESS_POLICY_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_ACCESS_POLICY_PROPERTY: &str = "Key Access Policy";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_ACCESS_POLICY_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_ATTEST_MAGIC: u32 = 1146110283u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_KEY_ATTEST_PADDING_INFO {
    pub magic: u32,
    pub pbKeyBlob: *mut u8,
    pub cbKeyBlob: u32,
    pub pbKeyAuth: *mut u8,
    pub cbKeyAuth: u32,
}
impl ::core::marker::Copy for NCRYPT_KEY_ATTEST_PADDING_INFO {}
impl ::core::clone::Clone for NCRYPT_KEY_ATTEST_PADDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_KEY_BLOB_HEADER {
    pub cbSize: u32,
    pub dwMagic: u32,
    pub cbAlgName: u32,
    pub cbKeyData: u32,
}
impl ::core::marker::Copy for NCRYPT_KEY_BLOB_HEADER {}
impl ::core::clone::Clone for NCRYPT_KEY_BLOB_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_DERIVATION_GROUP: &str = "KEY_DERIVATION";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_DERIVATION_INTERFACE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_DERIVATION_OPERATION: u32 = 64u32;
pub type NCRYPT_KEY_HANDLE = usize;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_CERTIFICATE: &str = "CERTIFICATE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_LOCAL: &str = "LOCAL";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_LOCKEDCREDENTIALS: &str = "LOCKEDCREDENTIALS";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_SDDL: &str = "SDDL";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_SID: &str = "SID";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_ALGORITHM_WEBCREDENTIALS: &str = "WEBCREDENTIALS";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_CERT_CERTBLOB: &str = "CertBlob";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_CERT_HASHID: &str = "HashId";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_INTERFACE: u32 = 65540u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_LOCAL_LOGON: &str = "logon";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_LOCAL_MACHINE: &str = "machine";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_PROTECTION_LOCAL_USER: &str = "user";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_STORAGE_ALGORITHM: &str = "KEY_STORAGE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_TYPE_PROPERTY: &str = "Key Type";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_KEY_USAGE_PROPERTY: &str = "Key Usage";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_LAST_MODIFIED_PROPERTY: &str = "Modified";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_LENGTHS_PROPERTY: &str = "Lengths";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_LENGTH_PROPERTY: &str = "Length";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MAX_ALG_ID_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MAX_KEY_NAME_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MAX_NAME_LENGTH_PROPERTY: &str = "Max Name Length";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MAX_PROPERTY_DATA: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MAX_PROPERTY_NAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MD2_ALGORITHM: &str = "MD2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MD4_ALGORITHM: &str = "MD4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_MD5_ALGORITHM: &str = "MD5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_NAMED_DESCRIPTOR_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_NAME_PROPERTY: &str = "Name";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_NO_CACHED_PASSWORD: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_OPAQUETRANSPORT_BLOB: &str = "OpaqueTransport";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type NCRYPT_OPERATION = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_CIPHER_OPERATION: NCRYPT_OPERATION = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_HASH_OPERATION: NCRYPT_OPERATION = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: NCRYPT_OPERATION = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SECRET_AGREEMENT_OPERATION: NCRYPT_OPERATION = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SIGNATURE_OPERATION: NCRYPT_OPERATION = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PAD_CIPHER_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PBKDF2_ALGORITHM: &str = "PBKDF2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_ALTERNATE_KEY_STORAGE_LOCATION_PROPERTY: &str = "PCP_ALTERNATE_KEY_STORAGE_LOCATION";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_CHANGEPASSWORD_PROPERTY: &str = "PCP_CHANGEPASSWORD";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_ECC_EKCERT_PROPERTY: &str = "PCP_ECC_EKCERT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_ECC_EKNVCERT_PROPERTY: &str = "PCP_ECC_EKNVCERT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_ECC_EKPUB_PROPERTY: &str = "PCP_ECC_EKPUB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_EKCERT_PROPERTY: &str = "PCP_EKCERT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_EKNVCERT_PROPERTY: &str = "PCP_EKNVCERT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_EKPUB_PROPERTY: &str = "PCP_EKPUB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_ENCRYPTION_KEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_EXPORT_ALLOWED_PROPERTY: &str = "PCP_EXPORT_ALLOWED";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_HMACVERIFICATION_KEY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_HMAC_AUTH_NONCE: &str = "PCP_HMAC_AUTH_NONCE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_HMAC_AUTH_POLICYINFO: &str = "PCP_HMAC_AUTH_POLICYINFO";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_HMAC_AUTH_POLICYREF: &str = "PCP_HMAC_AUTH_POLICYREF";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_HMAC_AUTH_SIGNATURE: &str = "PCP_HMAC_AUTH_SIGNATURE";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    pub dwVersion: u32,
    pub iExpiration: i32,
    pub pabNonce: [u8; 32],
    pub pabPolicyRef: [u8; 32],
    pub pabHMAC: [u8; 32],
}
impl ::core::marker::Copy for NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {}
impl ::core::clone::Clone for NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_HMAC_AUTH_TICKET: &str = "PCP_HMAC_AUTH_TICKET";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_IDENTITY_KEY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_INTERMEDIATE_CA_EKCERT_PROPERTY: &str = "PCP_INTERMEDIATE_CA_EKCERT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_KEYATTESTATION_PROPERTY: &str = "PCP_TPM12_KEYATTESTATION";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_KEY_CREATIONHASH_PROPERTY: &str = "PCP_KEY_CREATIONHASH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_KEY_CREATIONTICKET_PROPERTY: &str = "PCP_KEY_CREATIONTICKET";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY: &str = "PCP_KEY_USAGE_POLICY";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_MIGRATIONPASSWORD_PROPERTY: &str = "PCP_MIGRATIONPASSWORD";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_NO_DA_PROTECTION_PROPERTY: &str = "PCP_NO_DA_PROTECTION";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PASSWORD_REQUIRED_PROPERTY: &str = "PCP_PASSWORD_REQUIRED";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PCRTABLE_ALGORITHM_PROPERTY: &str = "PCP_PCRTABLE_ALGORITHM";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PCRTABLE_PROPERTY: &str = "PCP_PCRTABLE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PLATFORMHANDLE_PROPERTY: &str = "PCP_PLATFORMHANDLE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRALGID_PROPERTY: &str = "PCP_PLATFORM_BINDING_PCRALGID";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRDIGESTLIST_PROPERTY: &str = "PCP_PLATFORM_BINDING_PCRDIGESTLIST";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRDIGEST_PROPERTY: &str = "PCP_PLATFORM_BINDING_PCRDIGEST";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRMASK_PROPERTY: &str = "PCP_PLATFORM_BINDING_PCRMASK";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PLATFORM_TYPE_PROPERTY: &str = "PCP_PLATFORM_TYPE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PROVIDERHANDLE_PROPERTY: &str = "PCP_PROVIDERMHANDLE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PROVIDER_VERSION_PROPERTY: &str = "PCP_PROVIDER_VERSION";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_PSS_SALT_SIZE_PROPERTY: &str = "PSS Salt Size";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_PCP_RAW_POLICYDIGEST {
    pub dwVersion: u32,
    pub cbDigest: u32,
}
impl ::core::marker::Copy for NCRYPT_PCP_RAW_POLICYDIGEST {}
impl ::core::clone::Clone for NCRYPT_PCP_RAW_POLICYDIGEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_RAW_POLICYDIGEST_PROPERTY: &str = "PCP_RAW_POLICYDIGEST";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_RSA_EKCERT_PROPERTY: &str = "PCP_RSA_EKCERT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_RSA_EKNVCERT_PROPERTY: &str = "PCP_RSA_EKNVCERT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_RSA_EKPUB_PROPERTY: &str = "PCP_RSA_EKPUB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_RSA_SCHEME_HASH_ALG_PROPERTY: &str = "PCP_RSA_SCHEME_HASH_ALG";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_RSA_SCHEME_PROPERTY: &str = "PCP_RSA_SCHEME";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_SESSIONID_PROPERTY: &str = "PCP_SESSIONID";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_SIGNATURE_KEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_SRKPUB_PROPERTY: &str = "PCP_SRKPUB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_STORAGEPARENT_PROPERTY: &str = "PCP_STORAGEPARENT";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_STORAGE_KEY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_SYMMETRIC_KEYBITS_PROPERTY: &str = "PCP_SYMMETRIC_KEYBITS";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM12_IDACTIVATION_PROPERTY: &str = "PCP_TPM12_IDACTIVATION";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM12_IDBINDING_DYNAMIC_PROPERTY: &str = "PCP_TPM12_IDBINDING_DYNAMIC";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM12_IDBINDING_PROPERTY: &str = "PCP_TPM12_IDBINDING";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM2BNAME_PROPERTY: &str = "PCP_TPM2BNAME";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_PCP_TPM_FW_VERSION_INFO {
    pub major1: u16,
    pub major2: u16,
    pub minor1: u16,
    pub minor2: u16,
}
impl ::core::marker::Copy for NCRYPT_PCP_TPM_FW_VERSION_INFO {}
impl ::core::clone::Clone for NCRYPT_PCP_TPM_FW_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM_FW_VERSION_PROPERTY: &str = "PCP_TPM_FW_VERSION";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_PROHIBITED_PROPERTY: &str = "PCP_TPM_IFX_RSA_KEYGEN_PROHIBITED";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY_PROPERTY: &str = "PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM_MANUFACTURER_ID_PROPERTY: &str = "PCP_TPM_MANUFACTURER_ID";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_TPM_VERSION_PROPERTY: &str = "PCP_TPM_VERSION";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub HeaderSize: u32,
    pub cbCertifyInfo: u32,
    pub cbSignature: u32,
    pub cbTpmPublic: u32,
}
impl ::core::marker::Copy for NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {}
impl ::core::clone::Clone for NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PCP_USAGEAUTH_PROPERTY: &str = "PCP_USAGEAUTH";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_APPLICATION_IMAGE_PROPERTY: &str = "PinCacheApplicationImage";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_APPLICATION_STATUS_PROPERTY: &str = "PinCacheApplicationStatus";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_APPLICATION_TICKET_BYTE_LENGTH: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_APPLICATION_TICKET_PROPERTY: &str = "PinCacheApplicationTicket";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_CLEAR_FOR_CALLING_PROCESS_OPTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_CLEAR_PROPERTY: &str = "PinCacheClear";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_DISABLE_DPL_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_FLAGS_PROPERTY: &str = "PinCacheFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_FREE_APPLICATION_TICKET_PROPERTY: &str = "PinCacheFreeApplicationTicket";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_IS_GESTURE_REQUIRED_PROPERTY: &str = "PinCacheIsGestureRequired";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_PIN_PROPERTY: &str = "PinCachePin";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_CACHE_REQUIRE_GESTURE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_PROMPT_PROPERTY: &str = "SmartCardPinPrompt";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PIN_PROPERTY: &str = "SmartCardPin";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PKCS7_ENVELOPE_BLOB: &str = "PKCS7_ENVELOPE";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PKCS8_PRIVATE_KEY_BLOB: &str = "PKCS8_PRIVATEKEY";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PLATFORM_ATTEST_MAGIC: u32 = 1146110288u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_PLATFORM_ATTEST_PADDING_INFO {
    pub magic: u32,
    pub pcrMask: u32,
}
impl ::core::marker::Copy for NCRYPT_PLATFORM_ATTEST_PADDING_INFO {}
impl ::core::clone::Clone for NCRYPT_PLATFORM_ATTEST_PADDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PREFER_VIRTUAL_ISOLATION_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PROTECTED_KEY_BLOB: &str = "ProtectedKeyBlob";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PROTECTED_KEY_BLOB_MAGIC: u32 = 1263817296u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PROTECTION_INFO_TYPE_DESCRIPTOR_STRING: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NCRYPT_PROTECT_STREAM_INFO {
    pub pfnStreamOutput: PFNCryptStreamOutputCallback,
    pub pvCallbackCtxt: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NCRYPT_PROTECT_STREAM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NCRYPT_PROTECT_STREAM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NCRYPT_PROTECT_STREAM_INFO_EX {
    pub pfnStreamOutput: PFNCryptStreamOutputCallbackEx,
    pub pvCallbackCtxt: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NCRYPT_PROTECT_STREAM_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NCRYPT_PROTECT_STREAM_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PROTECT_TO_LOCAL_SYSTEM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PROVIDER_HANDLE_PROPERTY: &str = "Provider Handle";
pub type NCRYPT_PROV_HANDLE = usize;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_PUBLIC_LENGTH_PROPERTY: &str = "PublicKeyLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_RC2_ALGORITHM: &str = "RC2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_RC2_ALGORITHM_GROUP: &str = "RC2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_READER_ICON_PROPERTY: &str = "SmartCardReaderIcon";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_READER_PROPERTY: &str = "SmartCardReader";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_ROOT_CERTSTORE_PROPERTY: &str = "SmartcardRootCertStore";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_RSA_ALGORITHM: &str = "RSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_RSA_ALGORITHM_GROUP: &str = "RSA";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_RSA_SIGN_ALGORITHM: &str = "RSA_SIGN";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SCARD_NGC_KEY_NAME: &str = "SmartCardNgcKeyName";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SCARD_PIN_ID: &str = "SmartCardPinId";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SCARD_PIN_INFO: &str = "SmartCardPinInfo";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SEALING_FLAG: u32 = 256u32;
pub type NCRYPT_SECRET_HANDLE = usize;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SECURE_PIN_PROPERTY: &str = "SmartCardSecurePin";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SECURITY_DESCR_PROPERTY: &str = "Security Descr";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY: &str = "Security Descr Support";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SHA1_ALGORITHM: &str = "SHA1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SHA256_ALGORITHM: &str = "SHA256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SHA384_ALGORITHM: &str = "SHA384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SHA512_ALGORITHM: &str = "SHA512";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SIGNATURE_LENGTH_PROPERTY: &str = "SignatureLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SMARTCARD_GUID_PROPERTY: &str = "SmartCardGuid";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SP800108_CTR_HMAC_ALGORITHM: &str = "SP800_108_CTR_HMAC";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_SP80056A_CONCAT_ALGORITHM: &str = "SP800_56A_CONCAT";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_SUPPORTED_LENGTHS {
    pub dwMinLength: u32,
    pub dwMaxLength: u32,
    pub dwIncrement: u32,
    pub dwDefaultLength: u32,
}
impl ::core::marker::Copy for NCRYPT_SUPPORTED_LENGTHS {}
impl ::core::clone::Clone for NCRYPT_SUPPORTED_LENGTHS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM12_PROVIDER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM_LOADABLE_KEY_BLOB: &str = "PcpTpmProtectedKeyBlob";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    pub magic: u32,
    pub cbHeader: u32,
    pub cbPublic: u32,
    pub cbPrivate: u32,
    pub cbName: u32,
}
impl ::core::marker::Copy for NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {}
impl ::core::clone::Clone for NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM_LOADABLE_KEY_BLOB_MAGIC: u32 = 1297371211u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM_PAD_PSS_IGNORE_SALT: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub pcrAlg: u32,
    pub cbSignature: u32,
    pub cbQuote: u32,
    pub cbPcrs: u32,
}
impl ::core::marker::Copy for NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {}
impl ::core::clone::Clone for NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_CURRENT_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM_PSS_SALT_SIZE_HASHSIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM_PSS_SALT_SIZE_MAXIMUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TPM_PSS_SALT_SIZE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_TREAT_NIST_AS_GENERIC_ECC_FLAG: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_UI_POLICY {
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pszCreationTitle: ::windows_sys::core::PCWSTR,
    pub pszFriendlyName: ::windows_sys::core::PCWSTR,
    pub pszDescription: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for NCRYPT_UI_POLICY {}
impl ::core::clone::Clone for NCRYPT_UI_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_UI_POLICY_PROPERTY: &str = "UI Policy";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_UI_PROTECT_KEY_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_UNIQUE_NAME_PROPERTY: &str = "Unique Name";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_USER_CERTSTORE_PROPERTY: &str = "SmartCardUserCertStore";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_USE_CONTEXT_PROPERTY: &str = "Use Context";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_USE_COUNT_ENABLED_PROPERTY: &str = "Enabled Use Count";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_USE_COUNT_PROPERTY: &str = "Use Count";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_USE_PER_BOOT_KEY_FLAG: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_USE_PER_BOOT_KEY_PROPERTY: &str = "Per Boot Key";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_USE_VIRTUAL_ISOLATION_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_USE_VIRTUAL_ISOLATION_PROPERTY: &str = "Virtual Iso";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_VERSION_PROPERTY: &str = "Version";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS {
    pub Version: u32,
    pub TrustletId: u64,
    pub MinSvn: u32,
    pub FlagsMask: u32,
    pub FlagsExpected: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS {}
impl ::core::clone::Clone for NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS_CURRENT_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS_V0: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCRYPT_VSM_KEY_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub cbSignature: u32,
    pub cbReport: u32,
    pub cbAttributes: u32,
}
impl ::core::marker::Copy for NCRYPT_VSM_KEY_ATTESTATION_STATEMENT {}
impl ::core::clone::Clone for NCRYPT_VSM_KEY_ATTESTATION_STATEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_VSM_KEY_ATTESTATION_STATEMENT_CURRENT_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_VSM_KEY_ATTESTATION_STATEMENT_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NCRYPT_WINDOW_HANDLE_PROPERTY: &str = "HWND Handle";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCryptAlgorithmName {
    pub pszName: ::windows_sys::core::PWSTR,
    pub dwClass: NCRYPT_ALGORITHM_NAME_CLASS,
    pub dwAlgOperations: NCRYPT_OPERATION,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for NCryptAlgorithmName {}
impl ::core::clone::Clone for NCryptAlgorithmName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCryptKeyName {
    pub pszName: ::windows_sys::core::PWSTR,
    pub pszAlgid: ::windows_sys::core::PWSTR,
    pub dwLegacyKeySpec: CERT_KEY_SPEC,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for NCryptKeyName {}
impl ::core::clone::Clone for NCryptKeyName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct NCryptProviderName {
    pub pszName: ::windows_sys::core::PWSTR,
    pub pszComment: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for NCryptProviderName {}
impl ::core::clone::Clone for NCryptProviderName {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NETSCAPE_SIGN_CA_CERT_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NETSCAPE_SIGN_CERT_TYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NETSCAPE_SMIME_CA_CERT_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NETSCAPE_SMIME_CERT_TYPE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NETSCAPE_SSL_CA_CERT_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NETSCAPE_SSL_CLIENT_AUTH_CERT_TYPE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const NETSCAPE_SSL_SERVER_AUTH_CERT_TYPE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_BASIC_BY_KEY_RESPONDER_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_BASIC_BY_NAME_RESPONDER_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_BASIC_GOOD_CERT_STATUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_BASIC_RESPONSE: ::windows_sys::core::PCSTR = 69i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_BASIC_RESPONSE_ENTRY {
    pub CertId: OCSP_CERT_ID,
    pub dwCertStatus: u32,
    pub Anonymous: OCSP_BASIC_RESPONSE_ENTRY_0,
    pub ThisUpdate: super::super::Foundation::FILETIME,
    pub NextUpdate: super::super::Foundation::FILETIME,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCSP_BASIC_RESPONSE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCSP_BASIC_RESPONSE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union OCSP_BASIC_RESPONSE_ENTRY_0 {
    pub pRevokedInfo: *mut OCSP_BASIC_REVOKED_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCSP_BASIC_RESPONSE_ENTRY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCSP_BASIC_RESPONSE_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_BASIC_RESPONSE_INFO {
    pub dwVersion: u32,
    pub dwResponderIdChoice: u32,
    pub Anonymous: OCSP_BASIC_RESPONSE_INFO_0,
    pub ProducedAt: super::super::Foundation::FILETIME,
    pub cResponseEntry: u32,
    pub rgResponseEntry: *mut OCSP_BASIC_RESPONSE_ENTRY,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCSP_BASIC_RESPONSE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCSP_BASIC_RESPONSE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union OCSP_BASIC_RESPONSE_INFO_0 {
    pub ByNameResponderId: CRYPTOAPI_BLOB,
    pub ByKeyResponderId: CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCSP_BASIC_RESPONSE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCSP_BASIC_RESPONSE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_BASIC_RESPONSE_V1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_BASIC_REVOKED_CERT_STATUS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_BASIC_REVOKED_INFO {
    pub RevocationDate: super::super::Foundation::FILETIME,
    pub dwCrlReasonCode: CERT_REVOCATION_STATUS_REASON,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCSP_BASIC_REVOKED_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCSP_BASIC_REVOKED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_BASIC_SIGNED_RESPONSE: ::windows_sys::core::PCSTR = 68i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct OCSP_BASIC_SIGNED_RESPONSE_INFO {
    pub ToBeSigned: CRYPTOAPI_BLOB,
    pub SignatureInfo: OCSP_SIGNATURE_INFO,
}
impl ::core::marker::Copy for OCSP_BASIC_SIGNED_RESPONSE_INFO {}
impl ::core::clone::Clone for OCSP_BASIC_SIGNED_RESPONSE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_BASIC_UNKNOWN_CERT_STATUS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct OCSP_CERT_ID {
    pub HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub IssuerNameHash: CRYPTOAPI_BLOB,
    pub IssuerKeyHash: CRYPTOAPI_BLOB,
    pub SerialNumber: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for OCSP_CERT_ID {}
impl ::core::clone::Clone for OCSP_CERT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_INTERNAL_ERROR_RESPONSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_MALFORMED_REQUEST_RESPONSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_REQUEST: ::windows_sys::core::PCSTR = 66i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_REQUEST_ENTRY {
    pub CertId: OCSP_CERT_ID,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCSP_REQUEST_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCSP_REQUEST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_REQUEST_INFO {
    pub dwVersion: u32,
    pub pRequestorName: *mut CERT_ALT_NAME_ENTRY,
    pub cRequestEntry: u32,
    pub rgRequestEntry: *mut OCSP_REQUEST_ENTRY,
    pub cExtension: u32,
    pub rgExtension: *mut CERT_EXTENSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCSP_REQUEST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCSP_REQUEST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_REQUEST_V1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_RESPONSE: ::windows_sys::core::PCSTR = 67i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct OCSP_RESPONSE_INFO {
    pub dwStatus: u32,
    pub pszObjId: ::windows_sys::core::PSTR,
    pub Value: CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for OCSP_RESPONSE_INFO {}
impl ::core::clone::Clone for OCSP_RESPONSE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct OCSP_SIGNATURE_INFO {
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Signature: CRYPT_BIT_BLOB,
    pub cCertEncoded: u32,
    pub rgCertEncoded: *mut CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for OCSP_SIGNATURE_INFO {}
impl ::core::clone::Clone for OCSP_SIGNATURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_SIGNED_REQUEST: ::windows_sys::core::PCSTR = 65i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct OCSP_SIGNED_REQUEST_INFO {
    pub ToBeSigned: CRYPTOAPI_BLOB,
    pub pOptionalSignatureInfo: *mut OCSP_SIGNATURE_INFO,
}
impl ::core::marker::Copy for OCSP_SIGNED_REQUEST_INFO {}
impl ::core::clone::Clone for OCSP_SIGNED_REQUEST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_SIG_REQUIRED_RESPONSE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_SUCCESSFUL_RESPONSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_TRY_LATER_RESPONSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OCSP_UNAUTHORIZED_RESPONSE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const OPAQUEKEYBLOB: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCRYPT_DECRYPT_PRIVATE_KEY_FUNC = ::core::option::Option<unsafe extern "system" fn(algorithm: CRYPT_ALGORITHM_IDENTIFIER, encryptedprivatekey: CRYPTOAPI_BLOB, pbcleartextkey: *mut u8, pcbcleartextkey: *mut u32, pvoiddecryptfunc: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC = ::core::option::Option<unsafe extern "system" fn(palgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER, pcleartextprivatekey: *const CRYPTOAPI_BLOB, pbencryptedkey: *mut u8, pcbencryptedkey: *mut u32, pvoidencryptfunc: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCRYPT_RESOLVE_HCRYPTPROV_FUNC = ::core::option::Option<unsafe extern "system" fn(pprivatekeyinfo: *mut CRYPT_PRIVATE_KEY_INFO, phcryptprov: *mut usize, pvoidresolvefunc: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNCryptStreamOutputCallback = ::core::option::Option<unsafe extern "system" fn(pvcallbackctxt: *const ::core::ffi::c_void, pbdata: *const u8, cbdata: usize, ffinal: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNCryptStreamOutputCallbackEx = ::core::option::Option<unsafe extern "system" fn(pvcallbackctxt: *const ::core::ffi::c_void, pbdata: *const u8, cbdata: usize, hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, ffinal: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CANCEL_ASYNC_RETRIEVAL_FUNC = ::core::option::Option<unsafe extern "system" fn(hasyncretrieve: HCRYPTASYNC) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pcert: *const CERT_CONTEXT, pvfindarg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_CREATE_CONTEXT_SORT_FUNC = ::core::option::Option<unsafe extern "system" fn(cbtotalencoded: u32, cbremainencoded: u32, centry: u32, pvsort: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_DLL_OPEN_STORE_PROV_FUNC = ::core::option::Option<unsafe extern "system" fn(lpszstoreprovider: ::windows_sys::core::PCSTR, dwencodingtype: CERT_QUERY_ENCODING_TYPE, hcryptprov: HCRYPTPROV_LEGACY, dwflags: CERT_OPEN_STORE_FLAGS, pvpara: *const ::core::ffi::c_void, hcertstore: HCERTSTORE, pstoreprovinfo: *mut CERT_STORE_PROV_INFO) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_ENUM_PHYSICAL_STORE = ::core::option::Option<unsafe extern "system" fn(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pwszstorename: ::windows_sys::core::PCWSTR, pstoreinfo: *const CERT_PHYSICAL_STORE_INFO, pvreserved: *mut ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_ENUM_SYSTEM_STORE = ::core::option::Option<unsafe extern "system" fn(pvsystemstore: *const ::core::ffi::c_void, dwflags: CERT_SYSTEM_STORE_FLAGS, pstoreinfo: *const CERT_SYSTEM_STORE_INFO, pvreserved: *mut ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_ENUM_SYSTEM_STORE_LOCATION = ::core::option::Option<unsafe extern "system" fn(pwszstorelocation: ::windows_sys::core::PCWSTR, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_IS_WEAK_HASH = ::core::option::Option<unsafe extern "system" fn(dwhashusetype: u32, pwszcnghashalgid: ::windows_sys::core::PCWSTR, dwchainflags: u32, psignerchaincontext: *const CERT_CHAIN_CONTEXT, ptimestamp: *const super::super::Foundation::FILETIME, pwszfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pchaincontext: *const CERT_CHAIN_CONTEXT, pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT, pnewcrlcontext: *const CRL_CONTEXT, pprevcrlcontext: *const CRL_CONTEXT, pvarg: *mut ::core::ffi::c_void, dwwriteocspfileerror: u32)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CERT_STORE_PROV_CLOSE = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, dwflags: u32)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_CONTROL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, dwflags: u32, dwctrltype: u32, pvctrlpara: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_DELETE_CERT = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_DELETE_CRL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_DELETE_CTL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_FIND_CERT = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevcertcontext: *const CERT_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut ::core::ffi::c_void, ppprovcertcontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_FIND_CRL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevcrlcontext: *const CRL_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut ::core::ffi::c_void, ppprovcrlcontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_FIND_CTL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pfindinfo: *const CERT_STORE_PROV_FIND_INFO, pprevctlcontext: *const CTL_CONTEXT, dwflags: u32, ppvstoreprovfindinfo: *mut *mut ::core::ffi::c_void, ppprovctlcontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_FREE_FIND_CERT = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, pvstoreprovfindinfo: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_FREE_FIND_CRL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, pvstoreprovfindinfo: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_FREE_FIND_CTL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, pvstoreprovfindinfo: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_GET_CERT_PROPERTY = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_GET_CRL_PROPERTY = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_GET_CTL_PROPERTY = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_READ_CERT = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorecertcontext: *const CERT_CONTEXT, dwflags: u32, ppprovcertcontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_READ_CRL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorecrlcontext: *const CRL_CONTEXT, dwflags: u32, ppprovcrlcontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_READ_CTL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pstorectlcontext: *const CTL_CONTEXT, dwflags: u32, ppprovctlcontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_SET_CERT_PROPERTY = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_SET_CRL_PROPERTY = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_SET_CTL_PROPERTY = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_WRITE_CERT = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcertcontext: *const CERT_CONTEXT, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_WRITE_CRL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pcrlcontext: *const CRL_CONTEXT, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CERT_STORE_PROV_WRITE_CTL = ::core::option::Option<unsafe extern "system" fn(hstoreprov: HCERTSTOREPROV, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CMSG_ALLOC = ::core::option::Option<unsafe extern "system" fn(cb: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_CNG_IMPORT_CONTENT_ENCRYPT_KEY = ::core::option::Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_CNG_IMPORT_KEY_AGREE = ::core::option::Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, pkeyagreedecryptpara: *const CMSG_CTRL_KEY_AGREE_DECRYPT_PARA, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_CNG_IMPORT_KEY_TRANS = ::core::option::Option<unsafe extern "system" fn(pcngcontentdecryptinfo: *mut CMSG_CNG_CONTENT_DECRYPT_INFO, pkeytransdecryptpara: *const CMSG_CTRL_KEY_TRANS_DECRYPT_PARA, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_EXPORT_ENCRYPT_KEY = ::core::option::Option<unsafe extern "system" fn(hcryptprov: usize, hencryptkey: usize, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pbdata: *mut u8, pcbdata: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_EXPORT_KEY_AGREE = ::core::option::Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pkeyagreeencodeinfo: *const CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO, pkeyagreeencryptinfo: *mut CMSG_KEY_AGREE_ENCRYPT_INFO, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_EXPORT_KEY_TRANS = ::core::option::Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pkeytransencodeinfo: *const CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO, pkeytransencryptinfo: *mut CMSG_KEY_TRANS_ENCRYPT_INFO, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_EXPORT_MAIL_LIST = ::core::option::Option<unsafe extern "system" fn(pcontentencryptinfo: *const CMSG_CONTENT_ENCRYPT_INFO, pmaillistencodeinfo: *const CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO, pmaillistencryptinfo: *mut CMSG_MAIL_LIST_ENCRYPT_INFO, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CMSG_FREE = ::core::option::Option<unsafe extern "system" fn(pv: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_GEN_CONTENT_ENCRYPT_KEY = ::core::option::Option<unsafe extern "system" fn(pcontentencryptinfo: *mut CMSG_CONTENT_ENCRYPT_INFO, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_GEN_ENCRYPT_KEY = ::core::option::Option<unsafe extern "system" fn(phcryptprov: *mut usize, paiencrypt: *const CRYPT_ALGORITHM_IDENTIFIER, pvencryptauxinfo: *const ::core::ffi::c_void, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pfnalloc: PFN_CMSG_ALLOC, phencryptkey: *mut usize, ppbencryptparameters: *mut *mut u8, pcbencryptparameters: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_IMPORT_ENCRYPT_KEY = ::core::option::Option<unsafe extern "system" fn(hcryptprov: usize, dwkeyspec: u32, paiencrypt: *const CRYPT_ALGORITHM_IDENTIFIER, paipubkey: *const CRYPT_ALGORITHM_IDENTIFIER, pbencodedkey: *const u8, cbencodedkey: u32, phencryptkey: *mut usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_IMPORT_KEY_AGREE = ::core::option::Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pkeyagreedecryptpara: *const CMSG_CTRL_KEY_AGREE_DECRYPT_PARA, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, phcontentencryptkey: *mut usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_IMPORT_KEY_TRANS = ::core::option::Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pkeytransdecryptpara: *const CMSG_CTRL_KEY_TRANS_DECRYPT_PARA, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, phcontentencryptkey: *mut usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_IMPORT_MAIL_LIST = ::core::option::Option<unsafe extern "system" fn(pcontentencryptionalgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pmaillistdecryptpara: *const CMSG_CTRL_MAIL_LIST_DECRYPT_PARA, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, phcontentencryptkey: *mut usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CMSG_STREAM_OUTPUT = ::core::option::Option<unsafe extern "system" fn(pvarg: *const ::core::ffi::c_void, pbdata: *const u8, cbdata: u32, ffinal: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_ALLOC = ::core::option::Option<unsafe extern "system" fn(cbsize: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_ASYNC_PARAM_FREE_FUNC = ::core::option::Option<unsafe extern "system" fn(pszparamoid: ::windows_sys::core::PCSTR, pvparam: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC = ::core::option::Option<unsafe extern "system" fn(pvcompletion: *mut ::core::ffi::c_void, dwcompletioncode: u32, pszurl: ::windows_sys::core::PCSTR, pszobjectoid: ::windows_sys::core::PCSTR, pvobject: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_CANCEL_RETRIEVAL = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, pvarg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_ENUM_KEYID_PROP = ::core::option::Option<unsafe extern "system" fn(pkeyidentifier: *const CRYPTOAPI_BLOB, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void, cprop: u32, rgdwpropid: *const u32, rgpvdata: *const *const ::core::ffi::c_void, rgcbdata: *const u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_ENUM_OID_FUNC = ::core::option::Option<unsafe extern "system" fn(dwencodingtype: u32, pszfuncname: ::windows_sys::core::PCSTR, pszoid: ::windows_sys::core::PCSTR, cvalue: u32, rgdwvaluetype: *const u32, rgpwszvaluename: *const ::windows_sys::core::PWSTR, rgpbvaluedata: *const *const u8, rgcbvaluedata: *const u32, pvarg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_ENUM_OID_INFO = ::core::option::Option<unsafe extern "system" fn(pinfo: *const CRYPT_OID_INFO, pvarg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC = ::core::option::Option<unsafe extern "system" fn(hncryptkey: NCRYPT_KEY_HANDLE, dwcertencodingtype: u32, pszpublickeyobjid: ::windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC = ::core::option::Option<unsafe extern "system" fn(hbcryptkey: BCRYPT_KEY_HANDLE, dwcertencodingtype: u32, pszpublickeyobjid: ::windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC = ::core::option::Option<unsafe extern "system" fn(dwcertencodingtype: u32, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, ppvdecodedsignpara: *mut *mut ::core::ffi::c_void, ppwszcnghashalgid: *mut ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_FREE = ::core::option::Option<unsafe extern "system" fn(pv: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_GET_SIGNER_CERTIFICATE = ::core::option::Option<unsafe extern "system" fn(pvgetarg: *mut ::core::ffi::c_void, dwcertencodingtype: u32, psignerid: *const CERT_INFO, hmsgcertstore: HCERTSTORE) -> *mut CERT_CONTEXT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH = ::core::option::Option<unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, rgidentifierornamelist: *const *const CRYPTOAPI_BLOB, dwidentifierornamelistcount: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE = ::core::option::Option<unsafe extern "system" fn(pplugincontext: *const ::core::ffi::c_void, pbdata: *const u8)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER = ::core::option::Option<unsafe extern "system" fn(pplugincontext: *const ::core::ffi::c_void, pidentifier: *const CRYPTOAPI_BLOB)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD = ::core::option::Option<unsafe extern "system" fn(pplugincontext: *const ::core::ffi::c_void, pwszpassword: ::windows_sys::core::PCWSTR)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET = ::core::option::Option<unsafe extern "system" fn(pplugincontext: *const ::core::ffi::c_void, pidentifier: *const CRYPTOAPI_BLOB, dwnametype: u32, pnameblob: *const CRYPTOAPI_BLOB, ppbcontent: *mut *mut u8, pcbcontent: *mut u32, ppwszpassword: *mut ::windows_sys::core::PWSTR, ppidentifier: *mut *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_INITIALIZE = ::core::option::Option<unsafe extern "system" fn(pfnflush: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH, pcontext: *const ::core::ffi::c_void, pdwexpectedobjectcount: *mut u32, ppfunctable: *mut *mut CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE, ppplugincontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE = ::core::option::Option<unsafe extern "system" fn(dwreason: CRYPT_OBJECT_LOCATOR_RELEASE_REASON, pplugincontext: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_SIGN_AND_ENCODE_HASH_FUNC = ::core::option::Option<unsafe extern "system" fn(hkey: NCRYPT_KEY_HANDLE, dwcertencodingtype: u32, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvdecodedsignpara: *const ::core::ffi::c_void, pwszcngpubkeyalgid: ::windows_sys::core::PCWSTR, pwszcnghashalgid: ::windows_sys::core::PCWSTR, pbcomputedhash: *const u8, cbcomputedhash: u32, pbsignature: *mut u8, pcbsignature: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_VERIFY_ENCODED_SIGNATURE_FUNC = ::core::option::Option<unsafe extern "system" fn(dwcertencodingtype: u32, ppubkeyinfo: *const CERT_PUBLIC_KEY_INFO, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvdecodedsignpara: *const ::core::ffi::c_void, pwszcngpubkeyalgid: ::windows_sys::core::PCWSTR, pwszcnghashalgid: ::windows_sys::core::PCWSTR, pbcomputedhash: *const u8, cbcomputedhash: u32, pbsignature: *const u8, cbsignature: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_XML_CREATE_TRANSFORM = ::core::option::Option<unsafe extern "system" fn(ptransform: *const CRYPT_XML_ALGORITHM, pproviderin: *const CRYPT_XML_DATA_PROVIDER, pproviderout: *mut CRYPT_XML_DATA_PROVIDER) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_XML_DATA_PROVIDER_CLOSE = ::core::option::Option<unsafe extern "system" fn(pvcallbackstate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_XML_DATA_PROVIDER_READ = ::core::option::Option<unsafe extern "system" fn(pvcallbackstate: *mut ::core::ffi::c_void, pbdata: *mut u8, cbdata: u32, pcbread: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CRYPT_XML_ENUM_ALG_INFO = ::core::option::Option<unsafe extern "system" fn(pinfo: *const CRYPT_XML_ALGORITHM_INFO, pvarg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_CRYPT_XML_WRITE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pvcallbackstate: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EXPORT_PRIV_KEY_FUNC = ::core::option::Option<unsafe extern "system" fn(hcryptprov: usize, dwkeyspec: u32, pszprivatekeyobjid: ::windows_sys::core::PCSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pprivatekeyinfo: *mut CRYPT_PRIVATE_KEY_INFO, pcbprivatekeyinfo: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_FREE_ENCODED_OBJECT_FUNC = ::core::option::Option<unsafe extern "system" fn(pszobjectoid: ::windows_sys::core::PCSTR, pobject: *mut CRYPT_BLOB_ARRAY, pvfreecontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_IMPORT_PRIV_KEY_FUNC = ::core::option::Option<unsafe extern "system" fn(hcryptprov: usize, pprivatekeyinfo: *const CRYPT_PRIVATE_KEY_INFO, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC = ::core::option::Option<unsafe extern "system" fn(dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, phkey: *mut BCRYPT_KEY_HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_NCRYPT_ALLOC = ::core::option::Option<unsafe extern "system" fn(cbsize: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub type PFN_NCRYPT_FREE = ::core::option::Option<unsafe extern "system" fn(pv: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_CONFIG_REGPATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\PFX";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_DISABLE_ENCRYPT_CERTIFICATES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ENCRYPT_CERTIFICATES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ENCRYPT_CERTIFICATES_VALUE_NAME: &str = "EncryptCertificates";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_EXPORT_ECC_CURVE_OID: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_EXPORT_ECC_CURVE_PARAMETERS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_EXPORT_PBES2_PARAMS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_EXPORT_RESERVED_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_EXPORT_SILENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_IMPORT_RESERVED_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_IMPORT_SILENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ONLY_CERTIFICATES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ONLY_CERTIFICATES_CONTAINER_NAME: &str = "PfxContainer";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_NAME: &str = "PfxProvider";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_ONLY_NOT_ENCRYPTED_CERTIFICATES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_PBES2_ALG_AES256_SHA256: &str = "AES256-SHA256";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct PKCS12_PBES2_EXPORT_PARAMS {
    pub dwSize: u32,
    pub hNcryptDescriptor: *mut ::core::ffi::c_void,
    pub pwszPbes2Alg: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for PKCS12_PBES2_EXPORT_PARAMS {}
impl ::core::clone::Clone for PKCS12_PBES2_EXPORT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_PBKDF2_ID_HMAC_SHA1: &str = "1.2.840.113549.2.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_PBKDF2_ID_HMAC_SHA256: &str = "1.2.840.113549.2.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_PBKDF2_ID_HMAC_SHA384: &str = "1.2.840.113549.2.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_PBKDF2_ID_HMAC_SHA512: &str = "1.2.840.113549.2.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_PROTECT_TO_DOMAIN_SIDS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS12_VIRTUAL_ISOLATION_KEY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS5_PADDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS7_SIGNER_INFO: ::windows_sys::core::PCSTR = 500i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_7_NDR_ENCODING: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_ATTRIBUTE: ::windows_sys::core::PCSTR = 22i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_ATTRIBUTES: ::windows_sys::core::PCSTR = 48i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_CONTENT_INFO: ::windows_sys::core::PCSTR = 33i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_CONTENT_INFO_SEQUENCE_OF_ANY: ::windows_sys::core::PCSTR = 23i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_CTL: ::windows_sys::core::PCSTR = 37i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_ENCRYPTED_PRIVATE_KEY_INFO: ::windows_sys::core::PCSTR = 45i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_PRIVATE_KEY_INFO: ::windows_sys::core::PCSTR = 44i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_RC2_CBC_PARAMETERS: ::windows_sys::core::PCSTR = 41i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_RSAES_OAEP_PARAMETERS: ::windows_sys::core::PCSTR = 76i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_RSA_PRIVATE_KEY: ::windows_sys::core::PCSTR = 43i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_RSA_SSA_PSS_PARAMETERS: ::windows_sys::core::PCSTR = 75i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_RSA_SSA_PSS_TRAILER_FIELD_BC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_SMIME_CAPABILITIES: ::windows_sys::core::PCSTR = 42i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_SORTED_CTL: ::windows_sys::core::PCSTR = 49i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_TIME_REQUEST: ::windows_sys::core::PCSTR = 18i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PKCS_UTC_TIME: ::windows_sys::core::PCSTR = 17i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PLAINTEXTKEYBLOB: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_ELEMENT {
    pub targetEndpointAddress: ::windows_sys::core::PCWSTR,
    pub issuerEndpointAddress: ::windows_sys::core::PCWSTR,
    pub issuedTokenParameters: ::windows_sys::core::PCWSTR,
    pub privacyNoticeLink: ::windows_sys::core::PCWSTR,
    pub privacyNoticeVersion: u32,
    pub useManagedPresentation: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICY_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICY_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_ADMIN_PIN: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_APPLI_CERT: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_CERTCHAIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_CHANGE_PASSWORD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_CONTAINER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_CONTEXT_INFO: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_CRYPT_COUNT_KEY_USE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_DISMISS_PIN_UI_SEC: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_ENUMALGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_ENUMALGS_EX: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_ENUMCONTAINERS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_ENUMELECTROOTS: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_ENUMEX_SIGNING_PROT: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_ENUMMANDROOTS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_IMPTYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEYSET_TYPE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEYSPEC: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEYSTORAGE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEYX_KEYSIZE_INC: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_KEY_TYPE_SUBTYPE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_PROVTYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SESSION_KEYSIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SGC_INFO: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SIG_KEYSIZE_INC: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SMARTCARD_GUID: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SMARTCARD_READER_ICON: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_SYM_KEYSIZE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_UNIQUE_CONTAINER: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PP_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PRIVATEKEYBLOB: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct PRIVKEYVER3 {
    pub magic: u32,
    pub bitlenP: u32,
    pub bitlenQ: u32,
    pub bitlenJ: u32,
    pub bitlenX: u32,
    pub DSSSeed: DSSSEED,
}
impl ::core::marker::Copy for PRIVKEYVER3 {}
impl ::core::clone::Clone for PRIVKEYVER3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_DH_SCHANNEL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_DSS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_DSS_DH: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_EC_ECDSA_FULL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_EC_ECDSA_SIG: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_EC_ECNRA_FULL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_EC_ECNRA_SIG: u32 = 15u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROV_ENUMALGS {
    pub aiAlgid: u32,
    pub dwBitLen: u32,
    pub dwNameLen: u32,
    pub szName: [super::super::Foundation::CHAR; 20],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROV_ENUMALGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROV_ENUMALGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROV_ENUMALGS_EX {
    pub aiAlgid: u32,
    pub dwDefaultLen: u32,
    pub dwMinLen: u32,
    pub dwMaxLen: u32,
    pub dwProtocols: u32,
    pub dwNameLen: u32,
    pub szName: [super::super::Foundation::CHAR; 20],
    pub dwLongNameLen: u32,
    pub szLongName: [super::super::Foundation::CHAR; 40],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROV_ENUMALGS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROV_ENUMALGS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_FORTEZZA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_INTEL_SEC: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_MS_EXCHANGE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_REPLACE_OWF: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_RNG: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_RSA_AES: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_RSA_FULL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_RSA_SCHANNEL: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_RSA_SIG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_SPYRUS_LYNKS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_SSL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_STT_ACQ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_STT_BRND: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_STT_ISS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_STT_MER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PROV_STT_ROOT: u32 = 10u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct PUBKEY {
    pub magic: u32,
    pub bitlen: u32,
}
impl ::core::marker::Copy for PUBKEY {}
impl ::core::clone::Clone for PUBKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct PUBKEYVER3 {
    pub magic: u32,
    pub bitlenP: u32,
    pub bitlenQ: u32,
    pub bitlenJ: u32,
    pub DSSSeed: DSSSEED,
}
impl ::core::marker::Copy for PUBKEYVER3 {}
impl ::core::clone::Clone for PUBKEYVER3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PUBLICKEYBLOB: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const PUBLICKEYBLOBEX: u32 = 10u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct PUBLICKEYSTRUC {
    pub bType: u8,
    pub bVersion: u8,
    pub reserved: u16,
    pub aiKeyAlg: u32,
}
impl ::core::marker::Copy for PUBLICKEYSTRUC {}
impl ::core::clone::Clone for PUBLICKEYSTRUC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
#[repr(transparent)]
pub struct PaddingMode(pub i32);
impl PaddingMode {
    pub const None: Self = Self(1i32);
    pub const PKCS7: Self = Self(2i32);
    pub const Zeros: Self = Self(3i32);
    pub const ANSIX923: Self = Self(4i32);
    pub const ISO10126: Self = Self(5i32);
}
impl ::core::marker::Copy for PaddingMode {}
impl ::core::clone::Clone for PaddingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const RANDOM_PADDING: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct RECIPIENTPOLICY {
    pub recipient: ENDPOINTADDRESS,
    pub issuer: ENDPOINTADDRESS,
    pub tokenType: ::windows_sys::core::PCWSTR,
    pub requiredClaims: CLAIMLIST,
    pub optionalClaims: CLAIMLIST,
    pub privacyUrl: ::windows_sys::core::PCWSTR,
    pub privacyVersion: u32,
}
impl ::core::marker::Copy for RECIPIENTPOLICY {}
impl ::core::clone::Clone for RECIPIENTPOLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct RECIPIENTPOLICY2 {
    pub recipient: ENDPOINTADDRESS2,
    pub issuer: ENDPOINTADDRESS2,
    pub tokenType: ::windows_sys::core::PCWSTR,
    pub requiredClaims: CLAIMLIST,
    pub optionalClaims: CLAIMLIST,
    pub privacyUrl: ::windows_sys::core::PCWSTR,
    pub privacyVersion: u32,
}
impl ::core::marker::Copy for RECIPIENTPOLICY2 {}
impl ::core::clone::Clone for RECIPIENTPOLICY2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const RECIPIENTPOLICYV1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const RECIPIENTPOLICYV2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const REPORT_NOT_ABLE_TO_EXPORT_PRIVATE_KEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const REPORT_NO_PRIVATE_KEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const REVOCATION_OID_CRL_REVOCATION: ::windows_sys::core::PCSTR = 1i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct ROOT_INFO_LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for ROOT_INFO_LUID {}
impl ::core::clone::Clone for ROOT_INFO_LUID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const RSA1024BIT_KEY: u32 = 67108864u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct RSAPUBKEY {
    pub magic: u32,
    pub bitlen: u32,
    pub pubexp: u32,
}
impl ::core::marker::Copy for RSAPUBKEY {}
impl ::core::clone::Clone for RSAPUBKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const RSA_CSP_PUBLICKEYBLOB: ::windows_sys::core::PCSTR = 19i32 as _;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct SCHANNEL_ALG {
    pub dwUse: u32,
    pub Algid: u32,
    pub cBits: u32,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for SCHANNEL_ALG {}
impl ::core::clone::Clone for SCHANNEL_ALG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SCHANNEL_ENC_KEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SCHANNEL_MAC_KEY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SCHEME_OID_RETRIEVE_ENCODED_OBJECTW_FUNC: &str = "SchemeDllRetrieveEncodedObjectW";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SCHEME_OID_RETRIEVE_ENCODED_OBJECT_FUNC: &str = "SchemeDllRetrieveEncodedObject";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SIGNATURE_RESOURCE_NUMBER: u32 = 666u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SIMPLEBLOB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SITE_PIN_RULES_ALL_SUBDOMAINS_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SORTED_CTL_EXT_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct SSL_ECCKEY_BLOB {
    pub dwCurveType: u32,
    pub cbKey: u32,
}
impl ::core::marker::Copy for SSL_ECCKEY_BLOB {}
impl ::core::clone::Clone for SSL_ECCKEY_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_ECCPUBLIC_BLOB: &str = "SSLECCPUBLICBLOB";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_F12_ERROR_TEXT_LENGTH: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub dwErrorLevel: u32,
    pub dwErrorCategory: u32,
    pub dwReserved: u32,
    pub wszErrorText: [u16; 256],
}
impl ::core::marker::Copy for SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {}
impl ::core::clone::Clone for SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_HPKP_HEADER_COUNT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwReserved: u32,
    pub pwszServerName: ::windows_sys::core::PWSTR,
    pub rgpszHpkpValue: [::windows_sys::core::PSTR; 2],
}
impl ::core::marker::Copy for SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {}
impl ::core::clone::Clone for SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_HPKP_PKP_HEADER_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_HPKP_PKP_RO_HEADER_INDEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_KEY_PIN_ERROR_TEXT_LENGTH: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    pub cbSize: u32,
    pub dwReserved: u32,
    pub pwszServerName: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {}
impl ::core::clone::Clone for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    pub cbSize: u32,
    pub lError: i32,
    pub wszErrorText: [u16; 512],
}
impl ::core::marker::Copy for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {}
impl ::core::clone::Clone for SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_OBJECT_LOCATOR_CERT_VALIDATION_CONFIG_FUNC: &str = "SslObjectLocatorInitializeCertValidationConfig";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_OBJECT_LOCATOR_ISSUER_LIST_FUNC: &str = "SslObjectLocatorInitializeIssuerList";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SSL_OBJECT_LOCATOR_PFX_FUNC: &str = "SslObjectLocatorInitializePfx";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const SYMMETRICWRAPKEYBLOB: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_DONT_HASH_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_FAILURE_BAD_ALG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_FAILURE_BAD_FORMAT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_FAILURE_BAD_REQUEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_FAILURE_EXTENSION_NOT_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_FAILURE_INFO_NOT_AVAILABLE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_FAILURE_POLICY_NOT_SUPPORTED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_FAILURE_SYSTEM_FAILURE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_FAILURE_TIME_NOT_AVAILABLE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_INFO: ::windows_sys::core::PCSTR = 80i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_NO_AUTH_RETRIEVAL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_REQUEST: ::windows_sys::core::PCSTR = 78i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_RESPONSE: ::windows_sys::core::PCSTR = 79i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIMESTAMP_VERIFY_CONTEXT_SIGNATURE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_FLUSH_CRL: ::windows_sys::core::PCSTR = 2i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_FLUSH_CRL_FROM_CERT: ::windows_sys::core::PCSTR = 3i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_FLUSH_CTL: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CERT: ::windows_sys::core::PCSTR = 4i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CRL: ::windows_sys::core::PCSTR = 5i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_FLUSH_OBJECT_FUNC: &str = "TimeValidDllFlushObject";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_GET_CRL: ::windows_sys::core::PCSTR = 2i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_GET_CRL_FROM_CERT: ::windows_sys::core::PCSTR = 3i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_GET_CTL: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CERT: ::windows_sys::core::PCSTR = 4i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CRL: ::windows_sys::core::PCSTR = 5i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TIME_VALID_OID_GET_OBJECT_FUNC: &str = "TimeValidDllGetObject";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const TPM_RSA_SRK_SEAL_KEY: &str = "MICROSOFT_PCP_KSP_RSA_SEAL_KEY_3BD1C4BF-004E-4E2F-8A4D-0BF633DCB074";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CERTIFICATE_CRL_DIST_POINT: ::windows_sys::core::PCSTR = 2i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CERTIFICATE_CRL_DIST_POINT_AND_OCSP: ::windows_sys::core::PCSTR = 11i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CERTIFICATE_FRESHEST_CRL: ::windows_sys::core::PCSTR = 6i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CERTIFICATE_ISSUER: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CERTIFICATE_OCSP: ::windows_sys::core::PCSTR = 9i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CERTIFICATE_OCSP_AND_CRL_DIST_POINT: ::windows_sys::core::PCSTR = 10i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CERTIFICATE_ONLY_OCSP: ::windows_sys::core::PCSTR = 13i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CRL_FRESHEST_CRL: ::windows_sys::core::PCSTR = 7i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CRL_ISSUER: ::windows_sys::core::PCSTR = 5i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CROSS_CERT_DIST_POINT: ::windows_sys::core::PCSTR = 8i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CROSS_CERT_SUBJECT_INFO_ACCESS: ::windows_sys::core::PCSTR = 12i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CTL_ISSUER: ::windows_sys::core::PCSTR = 3i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_CTL_NEXT_UPDATE: ::windows_sys::core::PCSTR = 4i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const URL_OID_GET_OBJECT_URL_FUNC: &str = "UrlDllGetObjectUrl";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const USAGE_MATCH_TYPE_AND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const USAGE_MATCH_TYPE_OR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ALGORITHM_IDENTIFIER: ::windows_sys::core::PCSTR = 74i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ALTERNATE_NAME: ::windows_sys::core::PCSTR = 12i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ANY_STRING: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_AUTHORITY_INFO_ACCESS: ::windows_sys::core::PCSTR = 32i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_AUTHORITY_KEY_ID: ::windows_sys::core::PCSTR = 9i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_AUTHORITY_KEY_ID2: ::windows_sys::core::PCSTR = 31i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_BASIC_CONSTRAINTS: ::windows_sys::core::PCSTR = 13i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_BASIC_CONSTRAINTS2: ::windows_sys::core::PCSTR = 15i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_BIOMETRIC_EXT: ::windows_sys::core::PCSTR = 71i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_BITS: ::windows_sys::core::PCSTR = 26i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_BITS_WITHOUT_TRAILING_ZEROES: ::windows_sys::core::PCSTR = 51i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CERT: ::windows_sys::core::PCSTR = 1i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CERTIFICATE_TEMPLATE: ::windows_sys::core::PCSTR = 64i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CERT_BUNDLE: ::windows_sys::core::PCSTR = 81i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CERT_CRL_TO_BE_SIGNED: ::windows_sys::core::PCSTR = 3i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CERT_PAIR: ::windows_sys::core::PCSTR = 53i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CERT_POLICIES: ::windows_sys::core::PCSTR = 16i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CERT_REQUEST_TO_BE_SIGNED: ::windows_sys::core::PCSTR = 4i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CERT_TO_BE_SIGNED: ::windows_sys::core::PCSTR = 2i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CHOICE_OF_TIME: ::windows_sys::core::PCSTR = 30i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CRL_DIST_POINTS: ::windows_sys::core::PCSTR = 35i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CRL_REASON_CODE: i32 = 29i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_CROSS_CERT_DIST_POINTS: ::windows_sys::core::PCSTR = 58i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_DH_PARAMETERS: ::windows_sys::core::PCSTR = 47i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_DH_PUBLICKEY: i32 = 38i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_DSS_PARAMETERS: ::windows_sys::core::PCSTR = 39i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_DSS_PUBLICKEY: i32 = 38i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_DSS_SIGNATURE: ::windows_sys::core::PCSTR = 40i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ECC_PARAMETERS: ::windows_sys::core::PCSTR = 85i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ECC_PRIVATE_KEY: ::windows_sys::core::PCSTR = 82i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ECC_SIGNATURE: ::windows_sys::core::PCSTR = 47i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ENHANCED_KEY_USAGE: ::windows_sys::core::PCSTR = 36i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ENUMERATED: ::windows_sys::core::PCSTR = 29i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_EXTENSIONS: ::windows_sys::core::PCSTR = 5i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_INTEGER: ::windows_sys::core::PCSTR = 27i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_ISSUING_DIST_POINT: ::windows_sys::core::PCSTR = 54i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_KEYGEN_REQUEST_TO_BE_SIGNED: ::windows_sys::core::PCSTR = 21i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_KEY_ATTRIBUTES: ::windows_sys::core::PCSTR = 10i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_KEY_USAGE: ::windows_sys::core::PCSTR = 14i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_KEY_USAGE_RESTRICTION: ::windows_sys::core::PCSTR = 11i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_LOGOTYPE_EXT: ::windows_sys::core::PCSTR = 70i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_MULTI_BYTE_INTEGER: ::windows_sys::core::PCSTR = 28i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_MULTI_BYTE_UINT: ::windows_sys::core::PCSTR = 38i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_NAME: ::windows_sys::core::PCSTR = 7i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_NAME_CONSTRAINTS: ::windows_sys::core::PCSTR = 55i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_NAME_VALUE: ::windows_sys::core::PCSTR = 6i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_NDR_ENCODING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_OBJECT_IDENTIFIER: ::windows_sys::core::PCSTR = 73i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_OCTET_STRING: ::windows_sys::core::PCSTR = 25i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_PKIX_POLICY_QUALIFIER_USERNOTICE: ::windows_sys::core::PCSTR = 46i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_POLICY_CONSTRAINTS: ::windows_sys::core::PCSTR = 57i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_POLICY_MAPPINGS: ::windows_sys::core::PCSTR = 56i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_PUBLIC_KEY_INFO: ::windows_sys::core::PCSTR = 8i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_QC_STATEMENTS_EXT: ::windows_sys::core::PCSTR = 42i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_SEQUENCE_OF_ANY: ::windows_sys::core::PCSTR = 34i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_SUBJECT_DIR_ATTRS: ::windows_sys::core::PCSTR = 84i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_SUBJECT_INFO_ACCESS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_UNICODE_ANY_STRING: i32 = 24i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_UNICODE_NAME: ::windows_sys::core::PCSTR = 20i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X509_UNICODE_NAME_VALUE: ::windows_sys::core::PCSTR = 24i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X942_DH_PARAMETERS: ::windows_sys::core::PCSTR = 50i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const X942_OTHER_INFO: ::windows_sys::core::PCSTR = 52i32 as _;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const ZERO_PADDING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const cPRIV_KEY_CACHE_MAX_ITEMS_DEFAULT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const cPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS_DEFAULT: u32 = 86400u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const dwFORCE_KEY_PROTECTION_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const dwFORCE_KEY_PROTECTION_HIGH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const dwFORCE_KEY_PROTECTION_USER_SELECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szFORCE_KEY_PROTECTION: &str = "ForceKeyProtection";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szKEY_CACHE_ENABLED: &str = "CachePrivateKeys";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szKEY_CACHE_SECONDS: &str = "PrivateKeyLifetimeSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szKEY_CRYPTOAPI_PRIVATE_KEY_OPTIONS: &str = "Software\\Policies\\Microsoft\\Cryptography";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOIDVerisign_FailInfo: &str = "2.16.840.1.113733.1.9.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOIDVerisign_MessageType: &str = "2.16.840.1.113733.1.9.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOIDVerisign_PkiStatus: &str = "2.16.840.1.113733.1.9.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOIDVerisign_RecipientNonce: &str = "2.16.840.1.113733.1.9.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOIDVerisign_SenderNonce: &str = "2.16.840.1.113733.1.9.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOIDVerisign_TransactionID: &str = "2.16.840.1.113733.1.9.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ANSI_X942: &str = "1.2.840.10046";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ANSI_X942_DH: &str = "1.2.840.10046.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ANY_APPLICATION_POLICY: &str = "1.3.6.1.4.1.311.10.12.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ANY_CERT_POLICY: &str = "2.5.29.32.0";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ANY_ENHANCED_KEY_USAGE: &str = "2.5.29.37.0";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_APPLICATION_CERT_POLICIES: &str = "1.3.6.1.4.1.311.21.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_APPLICATION_POLICY_CONSTRAINTS: &str = "1.3.6.1.4.1.311.21.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_APPLICATION_POLICY_MAPPINGS: &str = "1.3.6.1.4.1.311.21.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ARCHIVED_KEY_ATTR: &str = "1.3.6.1.4.1.311.21.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ARCHIVED_KEY_CERT_HASH: &str = "1.3.6.1.4.1.311.21.16";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ATTEST_WHQL_CRYPTO: &str = "1.3.6.1.4.1.311.10.3.5.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ATTR_PLATFORM_SPECIFICATION: &str = "2.23.133.2.17";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ATTR_SUPPORTED_ALGORITHMS: &str = "2.5.4.52";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ATTR_TPM_SECURITY_ASSERTIONS: &str = "2.23.133.2.18";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ATTR_TPM_SPECIFICATION: &str = "2.23.133.2.16";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_AUTHORITY_INFO_ACCESS: &str = "1.3.6.1.5.5.7.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_AUTHORITY_KEY_IDENTIFIER: &str = "2.5.29.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_AUTHORITY_KEY_IDENTIFIER2: &str = "2.5.29.35";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_AUTHORITY_REVOCATION_LIST: &str = "2.5.4.38";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_AUTO_ENROLL_CTL_USAGE: &str = "1.3.6.1.4.1.311.20.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_BACKGROUND_OTHER_LOGOTYPE: &str = "1.3.6.1.5.5.7.20.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_BASIC_CONSTRAINTS: &str = "2.5.29.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_BASIC_CONSTRAINTS2: &str = "2.5.29.19";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_BIOMETRIC_EXT: &str = "1.3.6.1.5.5.7.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_BIOMETRIC_SIGNING: &str = "1.3.6.1.4.1.311.10.3.41";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_BUSINESS_CATEGORY: &str = "2.5.4.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CA_CERTIFICATE: &str = "2.5.4.37";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERTIFICATE_REVOCATION_LIST: &str = "2.5.4.39";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERTIFICATE_TEMPLATE: &str = "1.3.6.1.4.1.311.21.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERTSRV_CA_VERSION: &str = "1.3.6.1.4.1.311.21.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERTSRV_CROSSCA_VERSION: &str = "1.3.6.1.4.1.311.21.22";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERTSRV_PREVIOUS_CERT_HASH: &str = "1.3.6.1.4.1.311.21.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_DISALLOWED_FILETIME_PROP_ID: &str = "1.3.6.1.4.1.311.10.11.104";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_EXTENSIONS: &str = "1.3.6.1.4.1.311.2.1.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: &str = "1.3.6.1.4.1.311.10.11.28";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_KEY_IDENTIFIER_PROP_ID: &str = "1.3.6.1.4.1.311.10.11.20";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_MANIFOLD: &str = "1.3.6.1.4.1.311.20.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_MD5_HASH_PROP_ID: &str = "1.3.6.1.4.1.311.10.11.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_POLICIES: &str = "2.5.29.32";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_POLICIES_95: &str = "2.5.29.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_POLICIES_95_QUALIFIER1: &str = "2.16.840.1.113733.1.7.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_PROP_ID_PREFIX: &str = "1.3.6.1.4.1.311.10.11.";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_SIGNATURE_HASH_PROP_ID: &str = "1.3.6.1.4.1.311.10.11.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_STRONG_KEY_OS_1: &str = "1.3.6.1.4.1.311.72.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_STRONG_KEY_OS_CURRENT: &str = "1.3.6.1.4.1.311.72.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_STRONG_KEY_OS_PREFIX: &str = "1.3.6.1.4.1.311.72.2.";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_STRONG_SIGN_OS_1: &str = "1.3.6.1.4.1.311.72.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_STRONG_SIGN_OS_CURRENT: &str = "1.3.6.1.4.1.311.72.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_STRONG_SIGN_OS_PREFIX: &str = "1.3.6.1.4.1.311.72.1.";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: &str = "1.3.6.1.4.1.311.10.11.29";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC: &str = "1.3.6.1.5.5.7.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_ADD_ATTRIBUTES: &str = "1.3.6.1.4.1.311.10.10.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_ADD_EXTENSIONS: &str = "1.3.6.1.5.5.7.7.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_DATA_RETURN: &str = "1.3.6.1.5.5.7.7.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_DECRYPTED_POP: &str = "1.3.6.1.5.5.7.7.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_ENCRYPTED_POP: &str = "1.3.6.1.5.5.7.7.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_GET_CERT: &str = "1.3.6.1.5.5.7.7.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_GET_CRL: &str = "1.3.6.1.5.5.7.7.16";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_IDENTIFICATION: &str = "1.3.6.1.5.5.7.7.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_IDENTITY_PROOF: &str = "1.3.6.1.5.5.7.7.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_ID_CONFIRM_CERT_ACCEPTANCE: &str = "1.3.6.1.5.5.7.7.24";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_ID_POP_LINK_RANDOM: &str = "1.3.6.1.5.5.7.7.22";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_ID_POP_LINK_WITNESS: &str = "1.3.6.1.5.5.7.7.23";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_LRA_POP_WITNESS: &str = "1.3.6.1.5.5.7.7.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_QUERY_PENDING: &str = "1.3.6.1.5.5.7.7.21";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_RECIPIENT_NONCE: &str = "1.3.6.1.5.5.7.7.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_REG_INFO: &str = "1.3.6.1.5.5.7.7.18";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_RESPONSE_INFO: &str = "1.3.6.1.5.5.7.7.19";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_REVOKE_REQUEST: &str = "1.3.6.1.5.5.7.7.17";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_SENDER_NONCE: &str = "1.3.6.1.5.5.7.7.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_STATUS_INFO: &str = "1.3.6.1.5.5.7.7.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CMC_TRANSACTION_ID: &str = "1.3.6.1.5.5.7.7.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CN_ECDSA_SHA256: &str = "1.2.156.11235.1.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_COMMON_NAME: &str = "2.5.4.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_COUNTRY_NAME: &str = "2.5.4.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CRL_DIST_POINTS: &str = "2.5.29.31";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CRL_NEXT_PUBLISH: &str = "1.3.6.1.4.1.311.21.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CRL_NUMBER: &str = "2.5.29.20";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CRL_REASON_CODE: &str = "2.5.29.21";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CRL_SELF_CDP: &str = "1.3.6.1.4.1.311.21.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CRL_VIRTUAL_BASE: &str = "1.3.6.1.4.1.311.21.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CROSS_CERTIFICATE_PAIR: &str = "2.5.4.40";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CROSS_CERT_DIST_POINTS: &str = "1.3.6.1.4.1.311.10.9.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CTL: &str = "1.3.6.1.4.1.311.10.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CT_CERT_SCTLIST: &str = "1.3.6.1.4.1.11129.2.4.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CT_PKI_DATA: &str = "1.3.6.1.5.5.7.12.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_CT_PKI_RESPONSE: &str = "1.3.6.1.5.5.7.12.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DELTA_CRL_INDICATOR: &str = "2.5.29.27";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DESCRIPTION: &str = "2.5.4.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DESTINATION_INDICATOR: &str = "2.5.4.27";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DEVICE_SERIAL_NUMBER: &str = "2.5.4.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DH_SINGLE_PASS_STDDH_SHA1_KDF: &str = "1.3.133.16.840.63.0.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DH_SINGLE_PASS_STDDH_SHA256_KDF: &str = "1.3.132.1.11.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DH_SINGLE_PASS_STDDH_SHA384_KDF: &str = "1.3.132.1.11.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DISALLOWED_HASH: &str = "1.3.6.1.4.1.311.10.11.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DISALLOWED_LIST: &str = "1.3.6.1.4.1.311.10.3.30";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DN_QUALIFIER: &str = "2.5.4.46";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DOMAIN_COMPONENT: &str = "0.9.2342.19200300.100.1.25";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DRM: &str = "1.3.6.1.4.1.311.10.5.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DRM_INDIVIDUALIZATION: &str = "1.3.6.1.4.1.311.10.5.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DS: &str = "2.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DSALG: &str = "2.5.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DSALG_CRPT: &str = "2.5.8.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DSALG_HASH: &str = "2.5.8.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DSALG_RSA: &str = "2.5.8.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DSALG_SIGN: &str = "2.5.8.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DS_EMAIL_REPLICATION: &str = "1.3.6.1.4.1.311.21.19";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_DYNAMIC_CODE_GEN_SIGNER: &str = "1.3.6.1.4.1.311.76.5.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP160R1: &str = "1.3.36.3.3.2.8.1.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP160T1: &str = "1.3.36.3.3.2.8.1.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP192R1: &str = "1.3.36.3.3.2.8.1.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP192T1: &str = "1.3.36.3.3.2.8.1.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP224R1: &str = "1.3.36.3.3.2.8.1.1.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP224T1: &str = "1.3.36.3.3.2.8.1.1.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP256R1: &str = "1.3.36.3.3.2.8.1.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP256T1: &str = "1.3.36.3.3.2.8.1.1.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP320R1: &str = "1.3.36.3.3.2.8.1.1.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP320T1: &str = "1.3.36.3.3.2.8.1.1.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP384R1: &str = "1.3.36.3.3.2.8.1.1.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP384T1: &str = "1.3.36.3.3.2.8.1.1.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP512R1: &str = "1.3.36.3.3.2.8.1.1.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_BRAINPOOLP512T1: &str = "1.3.36.3.3.2.8.1.1.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_EC192WAPI: &str = "1.2.156.11235.1.1.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_NISTP192: &str = "1.2.840.10045.3.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_NISTP224: &str = "1.3.132.0.33";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_NISTP256: &str = "1.2.840.10045.3.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_NISTP384: &str = "1.3.132.0.34";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_NISTP521: &str = "1.3.132.0.35";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_P256: &str = "1.2.840.10045.3.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_P384: &str = "1.3.132.0.34";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_P521: &str = "1.3.132.0.35";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP160K1: &str = "1.3.132.0.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP160R1: &str = "1.3.132.0.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP160R2: &str = "1.3.132.0.30";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP192K1: &str = "1.3.132.0.31";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP192R1: &str = "1.2.840.10045.3.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP224K1: &str = "1.3.132.0.32";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP224R1: &str = "1.3.132.0.33";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP256K1: &str = "1.3.132.0.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP256R1: &str = "1.2.840.10045.3.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP384R1: &str = "1.3.132.0.34";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_SECP521R1: &str = "1.3.132.0.35";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_WTLS12: &str = "1.3.132.0.33";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_WTLS7: &str = "1.3.132.0.30";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_WTLS9: &str = "2.23.43.1.4.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_X962P192V1: &str = "1.2.840.10045.3.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_X962P192V2: &str = "1.2.840.10045.3.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_X962P192V3: &str = "1.2.840.10045.3.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_X962P239V1: &str = "1.2.840.10045.3.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_X962P239V2: &str = "1.2.840.10045.3.1.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_X962P239V3: &str = "1.2.840.10045.3.1.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_CURVE_X962P256V1: &str = "1.2.840.10045.3.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECC_PUBLIC_KEY: &str = "1.2.840.10045.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECDSA_SHA1: &str = "1.2.840.10045.4.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECDSA_SHA256: &str = "1.2.840.10045.4.3.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECDSA_SHA384: &str = "1.2.840.10045.4.3.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECDSA_SHA512: &str = "1.2.840.10045.4.3.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ECDSA_SPECIFIED: &str = "1.2.840.10045.4.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_EFS_RECOVERY: &str = "1.3.6.1.4.1.311.10.3.4.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_EMBEDDED_NT_CRYPTO: &str = "1.3.6.1.4.1.311.10.3.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENCLAVE_SIGNING: &str = "1.3.6.1.4.1.311.10.3.42";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENCRYPTED_KEY_HASH: &str = "1.3.6.1.4.1.311.21.21";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENHANCED_KEY_USAGE: &str = "2.5.29.37";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLLMENT_AGENT: &str = "1.3.6.1.4.1.311.20.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLLMENT_CSP_PROVIDER: &str = "1.3.6.1.4.1.311.13.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLLMENT_NAME_VALUE_PAIR: &str = "1.3.6.1.4.1.311.13.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_AIK_INFO: &str = "1.3.6.1.4.1.311.21.39";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_ATTESTATION_CHALLENGE: &str = "1.3.6.1.4.1.311.21.28";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_ATTESTATION_STATEMENT: &str = "1.3.6.1.4.1.311.21.24";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_CAXCHGCERT_HASH: &str = "1.3.6.1.4.1.311.21.27";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_CERTTYPE_EXTENSION: &str = "1.3.6.1.4.1.311.20.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_EKPUB_CHALLENGE: &str = "1.3.6.1.4.1.311.21.26";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_EKVERIFYCERT: &str = "1.3.6.1.4.1.311.21.31";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_EKVERIFYCREDS: &str = "1.3.6.1.4.1.311.21.32";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_EKVERIFYKEY: &str = "1.3.6.1.4.1.311.21.30";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_EK_CA_KEYID: &str = "1.3.6.1.4.1.311.21.43";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_EK_INFO: &str = "1.3.6.1.4.1.311.21.23";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_ENCRYPTION_ALGORITHM: &str = "1.3.6.1.4.1.311.21.29";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_KEY_AFFINITY: &str = "1.3.6.1.4.1.311.21.41";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_KSP_NAME: &str = "1.3.6.1.4.1.311.21.25";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_SCEP_CHALLENGE_ANSWER: &str = "1.3.6.1.4.1.311.21.35";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_SCEP_CLIENT_REQUEST: &str = "1.3.6.1.4.1.311.21.37";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_SCEP_ERROR: &str = "1.3.6.1.4.1.311.21.33";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_SCEP_SERVER_MESSAGE: &str = "1.3.6.1.4.1.311.21.38";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_SCEP_SERVER_SECRET: &str = "1.3.6.1.4.1.311.21.40";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_SCEP_SERVER_STATE: &str = "1.3.6.1.4.1.311.21.34";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENROLL_SCEP_SIGNER_HASH: &str = "1.3.6.1.4.1.311.21.42";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ENTERPRISE_OID_ROOT: &str = "1.3.6.1.4.1.311.21.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_EV_RDN_COUNTRY: &str = "1.3.6.1.4.1.311.60.2.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_EV_RDN_LOCALE: &str = "1.3.6.1.4.1.311.60.2.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_EV_RDN_STATE_OR_PROVINCE: &str = "1.3.6.1.4.1.311.60.2.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_EV_WHQL_CRYPTO: &str = "1.3.6.1.4.1.311.10.3.39";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_FACSIMILE_TELEPHONE_NUMBER: &str = "2.5.4.23";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_FRESHEST_CRL: &str = "2.5.29.46";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_GIVEN_NAME: &str = "2.5.4.42";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_HPKP_DOMAIN_NAME_CTL: &str = "1.3.6.1.4.1.311.10.3.60";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_HPKP_HEADER_VALUE_CTL: &str = "1.3.6.1.4.1.311.10.3.61";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC: &str = "2.16.840.1.101.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_SuiteAConfidentiality: &str = "2.16.840.1.101.2.1.1.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_SuiteAIntegrity: &str = "2.16.840.1.101.2.1.1.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_SuiteAKMandSig: &str = "2.16.840.1.101.2.1.1.18";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_SuiteAKeyManagement: &str = "2.16.840.1.101.2.1.1.17";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_SuiteASignature: &str = "2.16.840.1.101.2.1.1.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_SuiteATokenProtection: &str = "2.16.840.1.101.2.1.1.16";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicConfidentiality: &str = "2.16.840.1.101.2.1.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicIntegrity: &str = "2.16.840.1.101.2.1.1.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicKMandSig: &str = "2.16.840.1.101.2.1.1.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicKMandUpdSig: &str = "2.16.840.1.101.2.1.1.20";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicKeyManagement: &str = "2.16.840.1.101.2.1.1.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicSignature: &str = "2.16.840.1.101.2.1.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicTokenProtection: &str = "2.16.840.1.101.2.1.1.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicUpdatedInteg: &str = "2.16.840.1.101.2.1.1.21";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_mosaicUpdatedSig: &str = "2.16.840.1.101.2.1.1.19";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_sdnsConfidentiality: &str = "2.16.840.1.101.2.1.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_sdnsIntegrity: &str = "2.16.840.1.101.2.1.1.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_sdnsKMandSig: &str = "2.16.840.1.101.2.1.1.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_sdnsKeyManagement: &str = "2.16.840.1.101.2.1.1.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_sdnsSignature: &str = "2.16.840.1.101.2.1.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INFOSEC_sdnsTokenProtection: &str = "2.16.840.1.101.2.1.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INHIBIT_ANY_POLICY: &str = "2.5.29.54";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INITIALS: &str = "2.5.4.43";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INTERNATIONALIZED_EMAIL_ADDRESS: &str = "1.3.6.1.4.1.311.20.2.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_INTERNATIONAL_ISDN_NUMBER: &str = "2.5.4.25";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_IPSEC_KP_IKE_INTERMEDIATE: &str = "1.3.6.1.5.5.8.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ISSUED_CERT_HASH: &str = "1.3.6.1.4.1.311.21.17";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ISSUER_ALT_NAME: &str = "2.5.29.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ISSUER_ALT_NAME2: &str = "2.5.29.18";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ISSUING_DIST_POINT: &str = "2.5.29.28";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_IUM_SIGNING: &str = "1.3.6.1.4.1.311.10.3.37";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KEYID_RDN: &str = "1.3.6.1.4.1.311.10.7.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KEY_ATTRIBUTES: &str = "2.5.29.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KEY_USAGE: &str = "2.5.29.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KEY_USAGE_RESTRICTION: &str = "2.5.29.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_CA_EXCHANGE: &str = "1.3.6.1.4.1.311.21.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_CSP_SIGNATURE: &str = "1.3.6.1.4.1.311.10.3.16";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_CTL_USAGE_SIGNING: &str = "1.3.6.1.4.1.311.10.3.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_DOCUMENT_SIGNING: &str = "1.3.6.1.4.1.311.10.3.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_EFS: &str = "1.3.6.1.4.1.311.10.3.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_FLIGHT_SIGNING: &str = "1.3.6.1.4.1.311.10.3.27";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_KERNEL_MODE_CODE_SIGNING: &str = "1.3.6.1.4.1.311.61.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING: &str = "1.3.6.1.4.1.311.61.5.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING: &str = "1.3.6.1.4.1.311.61.4.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_KEY_RECOVERY: &str = "1.3.6.1.4.1.311.10.3.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_KEY_RECOVERY_AGENT: &str = "1.3.6.1.4.1.311.21.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_LIFETIME_SIGNING: &str = "1.3.6.1.4.1.311.10.3.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_MOBILE_DEVICE_SOFTWARE: &str = "1.3.6.1.4.1.311.10.3.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_PRIVACY_CA: &str = "1.3.6.1.4.1.311.21.36";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_QUALIFIED_SUBORDINATION: &str = "1.3.6.1.4.1.311.10.3.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_SMARTCARD_LOGON: &str = "1.3.6.1.4.1.311.20.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_SMART_DISPLAY: &str = "1.3.6.1.4.1.311.10.3.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_TIME_STAMP_SIGNING: &str = "1.3.6.1.4.1.311.10.3.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_TPM_AIK_CERTIFICATE: &str = "2.23.133.8.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_TPM_EK_CERTIFICATE: &str = "2.23.133.8.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_KP_TPM_PLATFORM_CERTIFICATE: &str = "2.23.133.8.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_LEGACY_POLICY_MAPPINGS: &str = "2.5.29.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_LICENSES: &str = "1.3.6.1.4.1.311.10.6.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_LICENSE_SERVER: &str = "1.3.6.1.4.1.311.10.6.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_LOCALITY_NAME: &str = "2.5.4.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_LOCAL_MACHINE_KEYSET: &str = "1.3.6.1.4.1.311.17.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_LOGOTYPE_EXT: &str = "1.3.6.1.5.5.7.1.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_LOYALTY_OTHER_LOGOTYPE: &str = "1.3.6.1.5.5.7.20.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_MEMBER: &str = "2.5.4.31";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_MICROSOFT_PUBLISHER_SIGNER: &str = "1.3.6.1.4.1.311.76.8.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NAME_CONSTRAINTS: &str = "2.5.29.30";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE: &str = "2.16.840.1.113730";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_BASE_URL: &str = "2.16.840.1.113730.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_CA_POLICY_URL: &str = "2.16.840.1.113730.1.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_CA_REVOCATION_URL: &str = "2.16.840.1.113730.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_CERT_EXTENSION: &str = "2.16.840.1.113730.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_CERT_RENEWAL_URL: &str = "2.16.840.1.113730.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_CERT_SEQUENCE: &str = "2.16.840.1.113730.2.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_CERT_TYPE: &str = "2.16.840.1.113730.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_COMMENT: &str = "2.16.840.1.113730.1.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_DATA_TYPE: &str = "2.16.840.1.113730.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_REVOCATION_URL: &str = "2.16.840.1.113730.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NETSCAPE_SSL_SERVER_NAME: &str = "2.16.840.1.113730.1.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NEXT_UPDATE_LOCATION: &str = "1.3.6.1.4.1.311.10.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_AES128_CBC: &str = "2.16.840.1.101.3.4.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_AES128_WRAP: &str = "2.16.840.1.101.3.4.1.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_AES192_CBC: &str = "2.16.840.1.101.3.4.1.22";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_AES192_WRAP: &str = "2.16.840.1.101.3.4.1.25";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_AES256_CBC: &str = "2.16.840.1.101.3.4.1.42";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_AES256_WRAP: &str = "2.16.840.1.101.3.4.1.45";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_sha256: &str = "2.16.840.1.101.3.4.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_sha384: &str = "2.16.840.1.101.3.4.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NIST_sha512: &str = "2.16.840.1.101.3.4.2.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NT5_CRYPTO: &str = "1.3.6.1.4.1.311.10.3.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NTDS_REPLICATION: &str = "1.3.6.1.4.1.311.25.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_NT_PRINCIPAL_NAME: &str = "1.3.6.1.4.1.311.20.2.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OEM_WHQL_CRYPTO: &str = "1.3.6.1.4.1.311.10.3.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIW: &str = "1.3.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWDIR: &str = "1.3.14.7.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWDIR_CRPT: &str = "1.3.14.7.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWDIR_HASH: &str = "1.3.14.7.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWDIR_SIGN: &str = "1.3.14.7.2.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWDIR_md2: &str = "1.3.14.7.2.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWDIR_md2RSA: &str = "1.3.14.7.2.3.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC: &str = "1.3.14.3.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_desCBC: &str = "1.3.14.3.2.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_desCFB: &str = "1.3.14.3.2.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_desECB: &str = "1.3.14.3.2.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_desEDE: &str = "1.3.14.3.2.17";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_desMAC: &str = "1.3.14.3.2.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_desOFB: &str = "1.3.14.3.2.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_dhCommMod: &str = "1.3.14.3.2.16";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_dsa: &str = "1.3.14.3.2.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_dsaComm: &str = "1.3.14.3.2.20";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_dsaCommSHA: &str = "1.3.14.3.2.21";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_dsaCommSHA1: &str = "1.3.14.3.2.28";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_dsaSHA1: &str = "1.3.14.3.2.27";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_keyHashSeal: &str = "1.3.14.3.2.23";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_md2RSASign: &str = "1.3.14.3.2.24";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_md4RSA: &str = "1.3.14.3.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_md4RSA2: &str = "1.3.14.3.2.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_md5RSA: &str = "1.3.14.3.2.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_md5RSASign: &str = "1.3.14.3.2.25";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_mdc2: &str = "1.3.14.3.2.19";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_mdc2RSA: &str = "1.3.14.3.2.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_rsaSign: &str = "1.3.14.3.2.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_rsaXchg: &str = "1.3.14.3.2.22";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_sha: &str = "1.3.14.3.2.18";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_sha1: &str = "1.3.14.3.2.26";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_sha1RSASign: &str = "1.3.14.3.2.29";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_shaDSA: &str = "1.3.14.3.2.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OIWSEC_shaRSA: &str = "1.3.14.3.2.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ORGANIZATIONAL_UNIT_NAME: &str = "2.5.4.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ORGANIZATION_NAME: &str = "2.5.4.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OS_VERSION: &str = "1.3.6.1.4.1.311.13.2.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_OWNER: &str = "2.5.4.32";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PHYSICAL_DELIVERY_OFFICE_NAME: &str = "2.5.4.19";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PIN_RULES_CTL: &str = "1.3.6.1.4.1.311.10.3.32";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PIN_RULES_DOMAIN_NAME: &str = "1.3.6.1.4.1.311.10.3.34";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PIN_RULES_EXT: &str = "1.3.6.1.4.1.311.10.3.33";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PIN_RULES_LOG_END_DATE_EXT: &str = "1.3.6.1.4.1.311.10.3.35";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PIN_RULES_SIGNER: &str = "1.3.6.1.4.1.311.10.3.31";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS: &str = "1.2.840.113549.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_1: &str = "1.2.840.113549.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_10: &str = "1.2.840.113549.1.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12: &str = "1.2.840.113549.1.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_EXTENDED_ATTRIBUTES: &str = "1.3.6.1.4.1.311.17.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_FRIENDLY_NAME_ATTR: &str = "1.2.840.113549.1.9.20";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_KEY_PROVIDER_NAME_ATTR: &str = "1.3.6.1.4.1.311.17.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_LOCAL_KEY_ID: &str = "1.2.840.113549.1.9.21";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID: &str = "1.3.6.1.4.1.311.17.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_PbeIds: &str = "1.2.840.113549.1.12.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_pbeWithSHA1And128BitRC2: &str = "1.2.840.113549.1.12.1.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_pbeWithSHA1And128BitRC4: &str = "1.2.840.113549.1.12.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_pbeWithSHA1And2KeyTripleDES: &str = "1.2.840.113549.1.12.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_pbeWithSHA1And3KeyTripleDES: &str = "1.2.840.113549.1.12.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_pbeWithSHA1And40BitRC2: &str = "1.2.840.113549.1.12.1.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_12_pbeWithSHA1And40BitRC4: &str = "1.2.840.113549.1.12.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_2: &str = "1.2.840.113549.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_3: &str = "1.2.840.113549.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_4: &str = "1.2.840.113549.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_5: &str = "1.2.840.113549.1.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_5_PBES2: &str = "1.2.840.113549.1.5.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_5_PBKDF2: &str = "1.2.840.113549.1.5.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_6: &str = "1.2.840.113549.1.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_7: &str = "1.2.840.113549.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_7_DATA: &str = "1.2.840.113549.1.7.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_7_DIGESTED: &str = "1.2.840.113549.1.7.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_7_ENCRYPTED: &str = "1.2.840.113549.1.7.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_7_ENVELOPED: &str = "1.2.840.113549.1.7.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_7_SIGNED: &str = "1.2.840.113549.1.7.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_7_SIGNEDANDENVELOPED: &str = "1.2.840.113549.1.7.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_8: &str = "1.2.840.113549.1.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_9: &str = "1.2.840.113549.1.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_9_CONTENT_TYPE: &str = "1.2.840.113549.1.9.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKCS_9_MESSAGE_DIGEST: &str = "1.2.840.113549.1.9.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKINIT_KP_KDC: &str = "1.3.6.1.5.2.3.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX: &str = "1.3.6.1.5.5.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_ACC_DESCR: &str = "1.3.6.1.5.5.7.48";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_CA_ISSUERS: &str = "1.3.6.1.5.5.7.48.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_CA_REPOSITORY: &str = "1.3.6.1.5.5.7.48.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP: &str = "1.3.6.1.5.5.7.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_CLIENT_AUTH: &str = "1.3.6.1.5.5.7.3.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_CODE_SIGNING: &str = "1.3.6.1.5.5.7.3.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_EMAIL_PROTECTION: &str = "1.3.6.1.5.5.7.3.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_IPSEC_END_SYSTEM: &str = "1.3.6.1.5.5.7.3.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_IPSEC_TUNNEL: &str = "1.3.6.1.5.5.7.3.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_IPSEC_USER: &str = "1.3.6.1.5.5.7.3.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_OCSP_SIGNING: &str = "1.3.6.1.5.5.7.3.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_SERVER_AUTH: &str = "1.3.6.1.5.5.7.3.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_KP_TIMESTAMP_SIGNING: &str = "1.3.6.1.5.5.7.3.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_NO_SIGNATURE: &str = "1.3.6.1.5.5.7.6.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_OCSP: &str = "1.3.6.1.5.5.7.48.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_OCSP_BASIC_SIGNED_RESPONSE: &str = "1.3.6.1.5.5.7.48.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_OCSP_NOCHECK: &str = "1.3.6.1.5.5.7.48.1.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_OCSP_NONCE: &str = "1.3.6.1.5.5.7.48.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_PE: &str = "1.3.6.1.5.5.7.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_POLICY_QUALIFIER_CPS: &str = "1.3.6.1.5.5.7.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_POLICY_QUALIFIER_USERNOTICE: &str = "1.3.6.1.5.5.7.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PKIX_TIME_STAMPING: &str = "1.3.6.1.5.5.7.48.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PLATFORM_MANIFEST_BINARY_ID: &str = "1.3.6.1.4.1.311.10.3.28";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_POLICY_CONSTRAINTS: &str = "2.5.29.36";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_POLICY_MAPPINGS: &str = "2.5.29.33";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_POSTAL_ADDRESS: &str = "2.5.4.16";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_POSTAL_CODE: &str = "2.5.4.17";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_POST_OFFICE_BOX: &str = "2.5.4.18";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PREFERRED_DELIVERY_METHOD: &str = "2.5.4.28";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PRESENTATION_ADDRESS: &str = "2.5.4.29";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PRIVATEKEY_USAGE_PERIOD: &str = "2.5.29.16";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PRODUCT_UPDATE: &str = "1.3.6.1.4.1.311.31.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PROTECTED_PROCESS_LIGHT_SIGNER: &str = "1.3.6.1.4.1.311.10.3.22";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_PROTECTED_PROCESS_SIGNER: &str = "1.3.6.1.4.1.311.10.3.24";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_QC_EU_COMPLIANCE: &str = "0.4.0.1862.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_QC_SSCD: &str = "0.4.0.1862.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_QC_STATEMENTS_EXT: &str = "1.3.6.1.5.5.7.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RDN_DUMMY_SIGNER: &str = "1.3.6.1.4.1.311.21.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RDN_TCG_PLATFORM_MANUFACTURER: &str = "2.23.133.2.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RDN_TCG_PLATFORM_MODEL: &str = "2.23.133.2.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RDN_TCG_PLATFORM_VERSION: &str = "2.23.133.2.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RDN_TPM_MANUFACTURER: &str = "2.23.133.2.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RDN_TPM_MODEL: &str = "2.23.133.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RDN_TPM_VERSION: &str = "2.23.133.2.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_REASON_CODE_HOLD: &str = "2.5.29.23";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_REGISTERED_ADDRESS: &str = "2.5.4.26";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_REMOVE_CERTIFICATE: &str = "1.3.6.1.4.1.311.10.8.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RENEWAL_CERTIFICATE: &str = "1.3.6.1.4.1.311.13.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_REQUEST_CLIENT_INFO: &str = "1.3.6.1.4.1.311.21.20";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_REQUIRE_CERT_CHAIN_POLICY: &str = "1.3.6.1.4.1.311.21.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_REVOKED_LIST_SIGNER: &str = "1.3.6.1.4.1.311.10.3.19";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RFC3161_counterSign: &str = "1.3.6.1.4.1.311.3.3.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ROLE_OCCUPANT: &str = "2.5.4.33";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ROOT_LIST_SIGNER: &str = "1.3.6.1.4.1.311.10.3.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION: &str = "1.3.6.1.4.1.311.60.3.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION: &str = "1.3.6.1.4.1.311.60.3.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ROOT_PROGRAM_FLAGS: &str = "1.3.6.1.4.1.311.60.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL: &str = "1.3.6.1.4.1.311.60.3.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA: &str = "1.2.840.113549";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSAES_OAEP: &str = "1.2.840.113549.1.1.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_DES_EDE3_CBC: &str = "1.2.840.113549.3.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_DH: &str = "1.2.840.113549.1.3.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_ENCRYPT: &str = "1.2.840.113549.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_HASH: &str = "1.2.840.113549.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_MD2: &str = "1.2.840.113549.2.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_MD2RSA: &str = "1.2.840.113549.1.1.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_MD4: &str = "1.2.840.113549.2.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_MD4RSA: &str = "1.2.840.113549.1.1.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_MD5: &str = "1.2.840.113549.2.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_MD5RSA: &str = "1.2.840.113549.1.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_MGF1: &str = "1.2.840.113549.1.1.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_PSPECIFIED: &str = "1.2.840.113549.1.1.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_RC2CBC: &str = "1.2.840.113549.3.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_RC4: &str = "1.2.840.113549.3.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_RC5_CBCPad: &str = "1.2.840.113549.3.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_RSA: &str = "1.2.840.113549.1.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SETOAEP_RSA: &str = "1.2.840.113549.1.1.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SHA1RSA: &str = "1.2.840.113549.1.1.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SHA256RSA: &str = "1.2.840.113549.1.1.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SHA384RSA: &str = "1.2.840.113549.1.1.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SHA512RSA: &str = "1.2.840.113549.1.1.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SMIMECapabilities: &str = "1.2.840.113549.1.9.15";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SMIMEalg: &str = "1.2.840.113549.1.9.16.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SMIMEalgCMS3DESwrap: &str = "1.2.840.113549.1.9.16.3.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SMIMEalgCMSRC2wrap: &str = "1.2.840.113549.1.9.16.3.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SMIMEalgESDH: &str = "1.2.840.113549.1.9.16.3.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_SSA_PSS: &str = "1.2.840.113549.1.1.10";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_certExtensions: &str = "1.2.840.113549.1.9.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_challengePwd: &str = "1.2.840.113549.1.9.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_contentType: &str = "1.2.840.113549.1.9.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_counterSign: &str = "1.2.840.113549.1.9.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_data: &str = "1.2.840.113549.1.7.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_digestedData: &str = "1.2.840.113549.1.7.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_emailAddr: &str = "1.2.840.113549.1.9.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_encryptedData: &str = "1.2.840.113549.1.7.6";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_envelopedData: &str = "1.2.840.113549.1.7.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_extCertAttrs: &str = "1.2.840.113549.1.9.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_hashedData: &str = "1.2.840.113549.1.7.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_messageDigest: &str = "1.2.840.113549.1.9.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_preferSignedData: &str = "1.2.840.113549.1.9.15.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_signEnvData: &str = "1.2.840.113549.1.7.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_signedData: &str = "1.2.840.113549.1.7.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_signingTime: &str = "1.2.840.113549.1.9.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_unstructAddr: &str = "1.2.840.113549.1.9.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_RSA_unstructName: &str = "1.2.840.113549.1.9.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SEARCH_GUIDE: &str = "2.5.4.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SEE_ALSO: &str = "2.5.4.34";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SERIALIZED: &str = "1.3.6.1.4.1.311.10.3.3.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SERVER_GATED_CRYPTO: &str = "1.3.6.1.4.1.311.10.3.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SGC_NETSCAPE: &str = "2.16.840.1.113730.4.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SITE_PIN_RULES_FLAGS_ATTR: &str = "1.3.6.1.4.1.311.10.4.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SITE_PIN_RULES_INDEX_ATTR: &str = "1.3.6.1.4.1.311.10.4.2";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SORTED_CTL: &str = "1.3.6.1.4.1.311.10.1.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_STATE_OR_PROVINCE_NAME: &str = "2.5.4.8";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_STREET_ADDRESS: &str = "2.5.4.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SUBJECT_ALT_NAME: &str = "2.5.29.7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SUBJECT_ALT_NAME2: &str = "2.5.29.17";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SUBJECT_DIR_ATTRS: &str = "2.5.29.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SUBJECT_INFO_ACCESS: &str = "1.3.6.1.5.5.7.1.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SUBJECT_KEY_IDENTIFIER: &str = "2.5.29.14";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SUPPORTED_APPLICATION_CONTEXT: &str = "2.5.4.30";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SUR_NAME: &str = "2.5.4.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_SYNC_ROOT_CTL_EXT: &str = "1.3.6.1.4.1.311.10.3.50";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_TELEPHONE_NUMBER: &str = "2.5.4.20";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_TELETEXT_TERMINAL_IDENTIFIER: &str = "2.5.4.22";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_TELEX_NUMBER: &str = "2.5.4.21";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_TIMESTAMP_TOKEN: &str = "1.2.840.113549.1.9.16.1.4";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_TITLE: &str = "2.5.4.12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_TLS_FEATURES_EXT: &str = "1.3.6.1.5.5.7.1.24";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_USER_CERTIFICATE: &str = "2.5.4.36";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_USER_PASSWORD: &str = "2.5.4.35";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_VERISIGN_BITSTRING_6_13: &str = "2.16.840.1.113733.1.6.13";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_VERISIGN_ISS_STRONG_CRYPTO: &str = "2.16.840.1.113733.1.8.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_VERISIGN_ONSITE_JURISDICTION_HASH: &str = "2.16.840.1.113733.1.6.11";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_VERISIGN_PRIVATE_6_9: &str = "2.16.840.1.113733.1.6.9";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_WHQL_CRYPTO: &str = "1.3.6.1.4.1.311.10.3.5";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_WINDOWS_KITS_SIGNER: &str = "1.3.6.1.4.1.311.10.3.20";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_WINDOWS_RT_SIGNER: &str = "1.3.6.1.4.1.311.10.3.21";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_WINDOWS_SOFTWARE_EXTENSION_SIGNER: &str = "1.3.6.1.4.1.311.10.3.26";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_WINDOWS_STORE_SIGNER: &str = "1.3.6.1.4.1.311.76.3.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_WINDOWS_TCB_SIGNER: &str = "1.3.6.1.4.1.311.10.3.23";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_WINDOWS_THIRD_PARTY_COMPONENT_SIGNER: &str = "1.3.6.1.4.1.311.10.3.25";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_X21_ADDRESS: &str = "2.5.4.24";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_X957: &str = "1.2.840.10040";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_X957_DSA: &str = "1.2.840.10040.4.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_X957_SHA1DSA: &str = "1.2.840.10040.4.3";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szOID_YESNO_TRUST_ATTR: &str = "1.3.6.1.4.1.311.10.4.1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szPRIV_KEY_CACHE_MAX_ITEMS: &str = "PrivKeyCacheMaxItems";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const szPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS: &str = "PrivKeyCachePurgeIntervalSeconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_COLLECTION: &str = "Collection";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_FILENAME: &str = "File";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_FILENAME_W: &str = "File";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_LDAP: &str = "Ldap";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_LDAP_W: &str = "Ldap";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_MEMORY: &str = "Memory";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_PHYSICAL: &str = "Physical";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_PHYSICAL_W: &str = "Physical";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_PKCS12: &str = "PKCS12";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_PKCS7: &str = "PKCS7";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_SERIALIZED: &str = "Serialized";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_SMART_CARD: &str = "SmartCard";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_SMART_CARD_W: &str = "SmartCard";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_SYSTEM: &str = "System";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_SYSTEM_REGISTRY: &str = "SystemRegistry";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_SYSTEM_REGISTRY_W: &str = "SystemRegistry";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const sz_CERT_STORE_PROV_SYSTEM_W: &str = "System";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_CANONICALIZATION_C14N: &str = "http://www.w3.org/TR/2001/REC-xml-c14n-20010315";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_CANONICALIZATION_C14NC: &str = "http://www.w3.org/TR/2001/REC-xml-c14n-20010315#WithComments";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_CANONICALIZATION_EXSLUSIVE_C14N: &str = "http://www.w3.org/2001/10/xml-exc-c14n#";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_CANONICALIZATION_EXSLUSIVE_C14NC: &str = "http://www.w3.org/2001/10/xml-exc-c14n#WithComments";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_TRANSFORM_XPATH: &str = "http://www.w3.org/TR/1999/REC-xpath-19991116";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_BASE64: &str = "http://www.w3.org/2000/09/xmldsig#base64";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_DSA_SHA1: &str = "http://www.w3.org/2000/09/xmldsig#dsa-sha1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA1: &str = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA256: &str = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA384: &str = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA512: &str = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA1: &str = "http://www.w3.org/2000/09/xmldsig#hmac-sha1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA256: &str = "http://www.w3.org/2001/04/xmldsig-more#hmac-sha256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA384: &str = "http://www.w3.org/2001/04/xmldsig-more#hmac-sha384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA512: &str = "http://www.w3.org/2001/04/xmldsig-more#hmac-sha512";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_RSA_SHA1: &str = "http://www.w3.org/2000/09/xmldsig#rsa-sha1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_RSA_SHA256: &str = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_RSA_SHA384: &str = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_RSA_SHA512: &str = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha512";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_SHA1: &str = "http://www.w3.org/2000/09/xmldsig#sha1";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_SHA256: &str = "http://www.w3.org/2001/04/xmlenc#sha256";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_SHA384: &str = "http://www.w3.org/2001/04/xmldsig-more#sha384";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_DIGSIG_SHA512: &str = "http://www.w3.org/2001/04/xmlenc#sha512";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_TRANSFORM_BASE64: &str = "http://www.w3.org/2000/09/xmldsig#base64";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszURI_XMLNS_TRANSFORM_ENVELOPED: &str = "http://www.w3.org/2000/09/xmldsig#enveloped-signature";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszXMLNS_DIGSIG: &str = "http://www.w3.org/2000/09/xmldsig#";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszXMLNS_DIGSIG_Id: &str = "Id";
#[doc = "*Required features: `\"Win32_Security_Cryptography\"`*"]
pub const wszXMLNS_DIGSIG_SignatureProperties: &str = "http://www.w3.org/2000/09/xmldsig#SignatureProperties";
