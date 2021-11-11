#![allow(non_snake_case, non_camel_case_types)]
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
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptAddContextFunction(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, dwposition: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCloseAlgorithmProvider(halgorithm: BCRYPT_ALG_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptConfigureContext(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, pconfig: *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptConfigureContextFunction(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pconfig: *const CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateContext(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, pconfig: *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut *mut ::core::ffi::c_void, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateMultiHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut *mut ::core::ffi::c_void, nhashes: u32, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDecrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pbiv: *mut u8, cbiv: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeleteContext(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKey(hsharedsecret: *const ::core::ffi::c_void, pwszkdf: super::super::Foundation::PWSTR, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKeyCapi(hhash: *const ::core::ffi::c_void, htargetalg: BCRYPT_ALG_HANDLE, pbderivedkey: *mut u8, cbderivedkey: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKeyPBKDF2(hprf: BCRYPT_ALG_HANDLE, pbpassword: *const u8, cbpassword: u32, pbsalt: *const u8, cbsalt: u32, citerations: u64, pbderivedkey: *mut u8, cbderivedkey: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroyHash(hhash: *mut ::core::ffi::c_void) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroyKey(hkey: BCRYPT_KEY_HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroySecret(hsecret: *mut ::core::ffi::c_void) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDuplicateHash(hhash: *const ::core::ffi::c_void, phnewhash: *mut *mut ::core::ffi::c_void, pbhashobject: *mut u8, cbhashobject: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDuplicateKey(hkey: BCRYPT_KEY_HANDLE, phnewkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEncrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pbiv: *mut u8, cbiv: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumAlgorithms(dwalgoperations: BCRYPT_OPERATION, palgcount: *mut u32, ppalglist: *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContextFunctionProviders(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContextFunctions(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTIONS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContexts(dwtable: BCRYPT_TABLE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXTS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumProviders(pszalgid: super::super::Foundation::PWSTR, pimplcount: *mut u32, ppimpllist: *mut *mut BCRYPT_PROVIDER_NAME, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumRegisteredProviders(pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDERS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptExportKey(hkey: BCRYPT_KEY_HANDLE, hexportkey: BCRYPT_KEY_HANDLE, pszblobtype: super::super::Foundation::PWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptFinalizeKeyPair(hkey: BCRYPT_KEY_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptFinishHash(hhash: *mut ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn BCryptFreeBuffer(pvbuffer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenRandom(halgorithm: BCRYPT_ALG_HANDLE, pbbuffer: *mut u8, cbbuffer: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenerateKeyPair(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, dwlength: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenerateSymmetricKey(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGetFipsAlgorithmMode(pfenabled: *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGetProperty(hobject: *const ::core::ffi::c_void, pszproperty: super::super::Foundation::PWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptHash(halgorithm: BCRYPT_ALG_HANDLE, pbsecret: *const u8, cbsecret: u32, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptHashData(hhash: *mut ::core::ffi::c_void, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptImportKey(halgorithm: BCRYPT_ALG_HANDLE, himportkey: BCRYPT_KEY_HANDLE, pszblobtype: super::super::Foundation::PWSTR, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptImportKeyPair(halgorithm: BCRYPT_ALG_HANDLE, himportkey: BCRYPT_KEY_HANDLE, pszblobtype: super::super::Foundation::PWSTR, phkey: *mut BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptKeyDerivation(hkey: BCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptOpenAlgorithmProvider(phalgorithm: *mut BCRYPT_ALG_HANDLE, pszalgid: super::super::Foundation::PWSTR, pszimplementation: super::super::Foundation::PWSTR, dwflags: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptProcessMultiOperations(hobject: *mut ::core::ffi::c_void, operationtype: BCRYPT_MULTI_OPERATION_TYPE, poperations: *const ::core::ffi::c_void, cboperations: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextConfiguration(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextFunctionConfiguration(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextFunctionProperty(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pszproperty: super::super::Foundation::PWSTR, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryProviderRegistration(pszprovider: super::super::Foundation::PWSTR, dwmode: BCRYPT_QUERY_PROVIDER_MODE, dwinterface: BCRYPT_INTERFACE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDER_REG) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptRegisterConfigChangeNotify(phevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptRemoveContextFunction(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptResolveProviders(pszcontext: super::super::Foundation::PWSTR, dwinterface: u32, pszfunction: super::super::Foundation::PWSTR, pszprovider: super::super::Foundation::PWSTR, dwmode: BCRYPT_QUERY_PROVIDER_MODE, dwflags: BCRYPT_RESOLVE_PROVIDERS_FLAGS, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDER_REFS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSecretAgreement(hprivkey: BCRYPT_KEY_HANDLE, hpubkey: BCRYPT_KEY_HANDLE, phagreedsecret: *mut *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSetContextFunctionProperty(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pszproperty: super::super::Foundation::PWSTR, cbvalue: u32, pbvalue: *const u8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSetProperty(hobject: *mut ::core::ffi::c_void, pszproperty: super::super::Foundation::PWSTR, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSignHash(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: *const ::core::ffi::c_void, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptUnregisterConfigChangeNotify(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptVerifySignature(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: *const ::core::ffi::c_void, pbhash: *const u8, cbhash: u32, pbsignature: *const u8, cbsignature: u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCRLContextToStore(hcertstore: *const ::core::ffi::c_void, pcrlcontext: *const CRL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCRLLinkToStore(hcertstore: *const ::core::ffi::c_void, pcrlcontext: *const CRL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCTLContextToStore(hcertstore: *const ::core::ffi::c_void, pctlcontext: *const CTL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCTLLinkToStore(hcertstore: *const ::core::ffi::c_void, pctlcontext: *const CTL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCertificateContextToStore(hcertstore: *const ::core::ffi::c_void, pcertcontext: *const CERT_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCertificateLinkToStore(hcertstore: *const ::core::ffi::c_void, pcertcontext: *const CERT_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCRLToStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, pbcrlencoded: *const u8, cbcrlencoded: u32, dwadddisposition: u32, ppcrlcontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCTLToStore(hcertstore: *const ::core::ffi::c_void, dwmsgandcertencodingtype: u32, pbctlencoded: *const u8, cbctlencoded: u32, dwadddisposition: u32, ppctlcontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, pbcertencoded: *const u8, cbcertencoded: u32, dwadddisposition: u32, ppcertcontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToSystemStoreA(szcertstorename: super::super::Foundation::PSTR, pbcertencoded: *const u8, cbcertencoded: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToSystemStoreW(szcertstorename: super::super::Foundation::PWSTR, pbcertencoded: *const u8, cbcertencoded: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEnhancedKeyUsageIdentifier(pcertcontext: *const CERT_CONTEXT, pszusageidentifier: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertAddRefServerOcspResponse(hserverocspresponse: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertAddRefServerOcspResponseContext(pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT);
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddSerializedElementToStore(hcertstore: *const ::core::ffi::c_void, pbelement: *const u8, cbelement: u32, dwadddisposition: u32, dwflags: u32, dwcontexttypeflags: u32, pdwcontexttype: *mut u32, ppvcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddStoreToCollection(hcollectionstore: *const ::core::ffi::c_void, hsiblingstore: *const ::core::ffi::c_void, dwupdateflags: u32, dwpriority: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAlgIdToOID(dwalgid: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertCloseServerOcspResponse(hserverocspresponse: *const ::core::ffi::c_void, dwflags: u32);
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCloseStore(hcertstore: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareCertificate(dwcertencodingtype: u32, pcertid1: *const CERT_INFO, pcertid2: *const CERT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareCertificateName(dwcertencodingtype: u32, pcertname1: *const CRYPTOAPI_BLOB, pcertname2: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareIntegerBlob(pint1: *const CRYPTOAPI_BLOB, pint2: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertComparePublicKeyInfo(dwcertencodingtype: u32, ppublickey1: *const CERT_PUBLIC_KEY_INFO, ppublickey2: *const CERT_PUBLIC_KEY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertControlStore(hcertstore: *const ::core::ffi::c_void, dwflags: CERT_CONTROL_STORE_FLAGS, dwctrltype: u32, pvctrlpara: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCRLContext(dwcertencodingtype: u32, pbcrlencoded: *const u8, cbcrlencoded: u32) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCTLContext(dwmsgandcertencodingtype: u32, pbctlencoded: *const u8, cbctlencoded: u32) -> *mut CTL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCTLEntryFromCertificateContextProperties(pcertcontext: *const CERT_CONTEXT, coptattr: u32, rgoptattr: *const CRYPT_ATTRIBUTE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pctlentry: *mut CTL_ENTRY, pcbctlentry: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCertificateChainEngine(pconfig: *const CERT_CHAIN_ENGINE_CONFIG, phchainengine: *mut HCERTCHAINENGINE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCertificateContext(dwcertencodingtype: u32, pbcertencoded: *const u8, cbcertencoded: u32) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateContext(dwcontexttype: u32, dwencodingtype: u32, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pcreatepara: *const ::core::mem::ManuallyDrop<CERT_CREATE_CONTEXT_PARA>) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateSelfSignCertificate(hcryptprovorncryptkey: usize, psubjectissuerblob: *const CRYPTOAPI_BLOB, dwflags: CERT_CREATE_SELFSIGN_FLAGS, pkeyprovinfo: *const CRYPT_KEY_PROV_INFO, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pstarttime: *const super::super::Foundation::SYSTEMTIME, pendtime: *const super::super::Foundation::SYSTEMTIME, pextensions: *const CERT_EXTENSIONS) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCRLFromStore(pcrlcontext: *const CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCTLFromStore(pctlcontext: *const CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCertificateFromStore(pcertcontext: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCRLContext(pcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCTLContext(pctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCertificateChain(pchaincontext: *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCertificateContext(pcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertDuplicateStore(hcertstore: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCRLContextProperties(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCRLsInStore(hcertstore: *const ::core::ffi::c_void, pprevcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCTLContextProperties(pctlcontext: *const CTL_CONTEXT, dwpropid: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCTLsInStore(hcertstore: *const ::core::ffi::c_void, pprevctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCertificateContextProperties(pcertcontext: *const CERT_CONTEXT, dwpropid: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCertificatesInStore(hcertstore: *const ::core::ffi::c_void, pprevcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenum: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSubjectInSortedCTL(pctlcontext: *const CTL_CONTEXT, ppvnextsubject: *mut *mut ::core::ffi::c_void, psubjectidentifier: *mut CRYPTOAPI_BLOB, pencodedattributes: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSystemStore(dwflags: u32, pvsystemstorelocationpara: *const ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void, pfnenum: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSystemStoreLocation(dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenum: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindAttribute(pszobjid: super::super::Foundation::PSTR, cattr: u32, rgattr: *const CRYPT_ATTRIBUTE) -> *mut CRYPT_ATTRIBUTE;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCRLInStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, dwfindflags: u32, dwfindtype: u32, pvfindpara: *const ::core::ffi::c_void, pprevcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCTLInStore(hcertstore: *const ::core::ffi::c_void, dwmsgandcertencodingtype: u32, dwfindflags: u32, dwfindtype: CERT_FIND_TYPE, pvfindpara: *const ::core::ffi::c_void, pprevctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCertificateInCRL(pcert: *const CERT_CONTEXT, pcrlcontext: *const CRL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppcrlentry: *mut *mut CRL_ENTRY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCertificateInStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, dwfindflags: u32, dwfindtype: CERT_FIND_FLAGS, pvfindpara: *const ::core::ffi::c_void, pprevcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindChainInStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, dwfindflags: CERT_FIND_CHAIN_IN_STORE_FLAGS, dwfindtype: u32, pvfindpara: *const ::core::ffi::c_void, pprevchaincontext: *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindExtension(pszobjid: super::super::Foundation::PSTR, cextensions: u32, rgextensions: *const CERT_EXTENSION) -> *mut CERT_EXTENSION;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindRDNAttr(pszobjid: super::super::Foundation::PSTR, pname: *const CERT_NAME_INFO) -> *mut CERT_RDN_ATTR;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindSubjectInCTL(dwencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> *mut CTL_ENTRY;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindSubjectInSortedCTL(psubjectidentifier: *const CRYPTOAPI_BLOB, pctlcontext: *const CTL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pencodedattributes: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCRLContext(pcrlcontext: *const CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCTLContext(pctlcontext: *const CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateChain(pchaincontext: *const CERT_CHAIN_CONTEXT);
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertFreeCertificateChainEngine(hchainengine: HCERTCHAINENGINE);
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateChainList(prgpselection: *const *const CERT_CHAIN_CONTEXT);
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateContext(pcertcontext: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertFreeServerOcspResponseContext(pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT);
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCRLContextProperty(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCRLFromStore(hcertstore: *const ::core::ffi::c_void, pissuercontext: *const CERT_CONTEXT, pprevcrlcontext: *const CRL_CONTEXT, pdwflags: *mut u32) -> *mut CRL_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCTLContextProperty(pctlcontext: *const CTL_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCertificateChain(hchainengine: HCERTCHAINENGINE, pcertcontext: *const CERT_CONTEXT, ptime: *const super::super::Foundation::FILETIME, hadditionalstore: *const ::core::ffi::c_void, pchainpara: *const CERT_CHAIN_PARA, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppchaincontext: *mut *mut CERT_CHAIN_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCertificateContextProperty(pcertcontext: *const CERT_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetEnhancedKeyUsage(pcertcontext: *const CERT_CONTEXT, dwflags: u32, pusage: *mut CTL_USAGE, pcbusage: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetIntendedKeyUsage(dwcertencodingtype: u32, pcertinfo: *const CERT_INFO, pbkeyusage: *mut u8, cbkeyusage: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetIssuerCertificateFromStore(hcertstore: *const ::core::ffi::c_void, psubjectcontext: *const CERT_CONTEXT, pprevissuercontext: *const CERT_CONTEXT, pdwflags: *mut u32) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetNameStringA(pcertcontext: *const CERT_CONTEXT, dwtype: u32, dwflags: u32, pvtypepara: *const ::core::ffi::c_void, psznamestring: super::super::Foundation::PSTR, cchnamestring: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetNameStringW(pcertcontext: *const CERT_CONTEXT, dwtype: u32, dwflags: u32, pvtypepara: *const ::core::ffi::c_void, psznamestring: super::super::Foundation::PWSTR, cchnamestring: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetPublicKeyLength(dwcertencodingtype: u32, ppublickey: *const CERT_PUBLIC_KEY_INFO) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertGetServerOcspResponseContext(hserverocspresponse: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> *mut CERT_SERVER_OCSP_RESPONSE_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetStoreProperty(hcertstore: *const ::core::ffi::c_void, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetSubjectCertificateFromStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, pcertid: *const CERT_INFO) -> *mut CERT_CONTEXT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetValidUsages(ccerts: u32, rghcerts: *const *const CERT_CONTEXT, cnumoids: *mut i32, rghoids: *mut super::super::Foundation::PSTR, pcboids: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsRDNAttrsInCertificateName(dwcertencodingtype: u32, dwflags: u32, pcertname: *const CRYPTOAPI_BLOB, prdn: *const CERT_RDN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsStrongHashToSign(pstrongsignpara: *const CERT_STRONG_SIGN_PARA, pwszcnghashalgid: super::super::Foundation::PWSTR, psigningcert: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsValidCRLForCertificate(pcert: *const CERT_CONTEXT, pcrl: *const CRL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsWeakHash(dwhashusetype: u32, pwszcnghashalgid: super::super::Foundation::PWSTR, dwchainflags: u32, psignerchaincontext: *const CERT_CHAIN_CONTEXT, ptimestamp: *const super::super::Foundation::FILETIME, pwszfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertNameToStrA(dwcertencodingtype: u32, pname: *const CRYPTOAPI_BLOB, dwstrtype: CERT_STRING_TYPE, psz: super::super::Foundation::PSTR, csz: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertNameToStrW(dwcertencodingtype: u32, pname: *const CRYPTOAPI_BLOB, dwstrtype: CERT_STRING_TYPE, psz: super::super::Foundation::PWSTR, csz: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOIDToAlgId(pszobjid: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenServerOcspResponse(pchaincontext: *const CERT_CHAIN_CONTEXT, dwflags: u32, popenpara: *const ::core::mem::ManuallyDrop<CERT_SERVER_OCSP_RESPONSE_OPEN_PARA>) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenStore(lpszstoreprovider: super::super::Foundation::PSTR, dwencodingtype: CERT_QUERY_ENCODING_TYPE, hcryptprov: usize, dwflags: CERT_OPEN_STORE_FLAGS, pvpara: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenSystemStoreA(hprov: usize, szsubsystemprotocol: super::super::Foundation::PSTR) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenSystemStoreW(hprov: usize, szsubsystemprotocol: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRDNValueToStrA(dwvaluetype: u32, pvalue: *const CRYPTOAPI_BLOB, psz: super::super::Foundation::PSTR, csz: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRDNValueToStrW(dwvaluetype: u32, pvalue: *const CRYPTOAPI_BLOB, psz: super::super::Foundation::PWSTR, csz: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRegisterPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pwszstorename: super::super::Foundation::PWSTR, pstoreinfo: *const CERT_PHYSICAL_STORE_INFO, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRegisterSystemStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pstoreinfo: *const CERT_SYSTEM_STORE_INFO, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRemoveEnhancedKeyUsageIdentifier(pcertcontext: *const CERT_CONTEXT, pszusageidentifier: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertRemoveStoreFromCollection(hcollectionstore: *const ::core::ffi::c_void, hsiblingstore: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertResyncCertificateChainEngine(hchainengine: HCERTCHAINENGINE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRetrieveLogoOrBiometricInfo(pcertcontext: *const CERT_CONTEXT, lpszlogoorbiometrictype: super::super::Foundation::PSTR, dwretrievalflags: u32, dwtimeout: u32, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ppwszmimetype: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSaveStore(hcertstore: *const ::core::ffi::c_void, dwencodingtype: CERT_QUERY_ENCODING_TYPE, dwsaveas: CERT_STORE_SAVE_AS, dwsaveto: CERT_STORE_SAVE_TO, pvsavetopara: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSelectCertificateChains(pselectioncontext: *const ::windows::runtime::GUID, dwflags: u32, pchainparameters: *const CERT_SELECT_CHAIN_PARA, ccriteria: u32, rgpcriteria: *const CERT_SELECT_CRITERIA, hstore: *const ::core::ffi::c_void, pcselection: *mut u32, pprgpselection: *mut *mut *mut CERT_CHAIN_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCRLStoreElement(pcrlcontext: *const CRL_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCTLStoreElement(pctlcontext: *const CTL_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCertificateStoreElement(pcertcontext: *const CERT_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCRLContextProperty(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCTLContextProperty(pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCertificateContextPropertiesFromCTLEntry(pcertcontext: *const CERT_CONTEXT, pctlentry: *const CTL_ENTRY, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCertificateContextProperty(pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetEnhancedKeyUsage(pcertcontext: *const CERT_CONTEXT, pusage: *const CTL_USAGE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetStoreProperty(hcertstore: *const ::core::ffi::c_void, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertStrToNameA(dwcertencodingtype: u32, pszx500: super::super::Foundation::PSTR, dwstrtype: CERT_STRING_TYPE, pvreserved: *mut ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32, ppszerror: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertStrToNameW(dwcertencodingtype: u32, pszx500: super::super::Foundation::PWSTR, dwstrtype: CERT_STRING_TYPE, pvreserved: *mut ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32, ppszerror: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertUnregisterPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pwszstorename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertUnregisterSystemStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCRLRevocation(dwcertencodingtype: u32, pcertid: *const CERT_INFO, ccrlinfo: u32, rgpcrlinfo: *const *const CRL_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCRLTimeValidity(ptimetoverify: *const super::super::Foundation::FILETIME, pcrlinfo: *const CRL_INFO) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCTLUsage(dwencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, psubjectusage: *const CTL_USAGE, dwflags: u32, pverifyusagepara: *const CTL_VERIFY_USAGE_PARA, pverifyusagestatus: *mut CTL_VERIFY_USAGE_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCertificateChainPolicy(pszpolicyoid: super::super::Foundation::PSTR, pchaincontext: *const CERT_CHAIN_CONTEXT, ppolicypara: *const CERT_CHAIN_POLICY_PARA, ppolicystatus: *mut CERT_CHAIN_POLICY_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyRevocation(dwencodingtype: u32, dwrevtype: u32, ccontext: u32, rgpvcontext: *const *const ::core::ffi::c_void, dwflags: u32, prevpara: *const CERT_REVOCATION_PARA, prevstatus: *mut CERT_REVOCATION_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifySubjectCertificateContext(psubject: *const CERT_CONTEXT, pissuer: *const CERT_CONTEXT, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyTimeValidity(ptimetoverify: *const super::super::Foundation::FILETIME, pcertinfo: *const CERT_INFO) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyValidityNesting(psubjectinfo: *const CERT_INFO, pissuerinfo: *const CERT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CloseCryptoHandle(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireCertificatePrivateKey(pcert: *const CERT_CONTEXT, dwflags: CRYPT_ACQUIRE_FLAGS, pvparameters: *const ::core::ffi::c_void, phcryptprovorncryptkey: *mut usize, pdwkeyspec: *mut CERT_KEY_SPEC, pfcallerfreeprovorncryptkey: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireContextA(phprov: *mut usize, szcontainer: super::super::Foundation::PSTR, szprovider: super::super::Foundation::PSTR, dwprovtype: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireContextW(phprov: *mut usize, szcontainer: super::super::Foundation::PWSTR, szprovider: super::super::Foundation::PWSTR, dwprovtype: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptBinaryToStringA(pbbinary: *const u8, cbbinary: u32, dwflags: CRYPT_STRING, pszstring: super::super::Foundation::PSTR, pcchstring: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptBinaryToStringW(pbbinary: *const u8, cbbinary: u32, dwflags: CRYPT_STRING, pszstring: super::super::Foundation::PWSTR, pcchstring: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCloseAsyncHandle(hasync: HCRYPTASYNC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptContextAddRef(hprov: usize, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateAsyncHandle(dwflags: u32, phasync: *mut HCRYPTASYNC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateHash(hprov: usize, algid: u32, hkey: usize, dwflags: u32, phhash: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateKeyIdentifierFromCSP(dwcertencodingtype: u32, pszpubkeyoid: super::super::Foundation::PSTR, ppubkeystruc: *const PUBLICKEYSTRUC, cbpubkeystruc: u32, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pbhash: *mut u8, pcbhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeMessage(dwmsgtypeflags: u32, pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara: *const ::core::mem::ManuallyDrop<CRYPT_VERIFY_MESSAGE_PARA>, dwsignerindex: u32, pbencodedblob: *const u8, cbencodedblob: u32, dwprevinnercontenttype: u32, pdwmsgtype: *mut u32, pdwinnercontenttype: *mut u32, pbdecoded: *mut u8, pcbdecoded: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeObject(dwcertencodingtype: u32, lpszstructtype: super::super::Foundation::PSTR, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pvstructinfo: *mut ::core::ffi::c_void, pcbstructinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeObjectEx(dwcertencodingtype: u32, lpszstructtype: super::super::Foundation::PSTR, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pdecodepara: *const ::core::mem::ManuallyDrop<CRYPT_DECODE_PARA>, pvstructinfo: *mut ::core::ffi::c_void, pcbstructinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecrypt(hkey: usize, hhash: usize, r#final: super::super::Foundation::BOOL, dwflags: u32, pbdata: *mut u8, pdwdatalen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecryptAndVerifyMessageSignature(pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara: *const ::core::mem::ManuallyDrop<CRYPT_VERIFY_MESSAGE_PARA>, dwsignerindex: u32, pbencryptedblob: *const u8, cbencryptedblob: u32, pbdecrypted: *mut u8, pcbdecrypted: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecryptMessage(pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pbencryptedblob: *const u8, cbencryptedblob: u32, pbdecrypted: *mut u8, pcbdecrypted: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDeriveKey(hprov: usize, algid: u32, hbasedata: usize, dwflags: u32, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDestroyHash(hhash: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDestroyKey(hkey: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDuplicateHash(hhash: usize, pdwreserved: *mut u32, dwflags: u32, phhash: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDuplicateKey(hkey: usize, pdwreserved: *mut u32, dwflags: u32, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncodeObject(dwcertencodingtype: u32, lpszstructtype: super::super::Foundation::PSTR, pvstructinfo: *const ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncodeObjectEx(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, lpszstructtype: super::super::Foundation::PSTR, pvstructinfo: *const ::core::ffi::c_void, dwflags: CRYPT_ENCODE_OBJECT_FLAGS, pencodepara: *const ::core::mem::ManuallyDrop<CRYPT_ENCODE_PARA>, pvencoded: *mut ::core::ffi::c_void, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncrypt(hkey: usize, hhash: usize, r#final: super::super::Foundation::BOOL, dwflags: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwbuflen: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncryptMessage(pencryptpara: *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert: u32, rgprecipientcert: *const *const CERT_CONTEXT, pbtobeencrypted: *const u8, cbtobeencrypted: u32, pbencryptedblob: *mut u8, pcbencryptedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumKeyIdentifierProperties(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void, pfnenum: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumoidfunc: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumOIDInfo(dwgroupid: u32, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumoidinfo: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProviderTypesA(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, sztypename: super::super::Foundation::PSTR, pcbtypename: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProviderTypesW(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, sztypename: super::super::Foundation::PWSTR, pcbtypename: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProvidersA(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, szprovname: super::super::Foundation::PSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProvidersW(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, szprovname: super::super::Foundation::PWSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportKey(hkey: usize, hexpkey: usize, dwblobtype: u32, dwflags: CRYPT_KEY_FLAGS, pbdata: *mut u8, pdwdatalen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPKCS8(hcryptprov: usize, dwkeyspec: u32, pszprivatekeyobjid: super::super::Foundation::PSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pbprivatekeyblob: *mut u8, pcbprivatekeyblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfo(hcryptprovorncryptkey: usize, dwkeyspec: u32, dwcertencodingtype: u32, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfoEx(hcryptprovorncryptkey: usize, dwkeyspec: u32, dwcertencodingtype: u32, pszpublickeyobjid: super::super::Foundation::PSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfoFromBCryptKeyHandle(hbcryptkey: BCRYPT_KEY_HANDLE, dwcertencodingtype: u32, pszpublickeyobjid: super::super::Foundation::PSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindCertificateKeyProvInfo(pcert: *const CERT_CONTEXT, dwflags: CRYPT_FIND_FLAGS, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindLocalizedName(pwszcryptname: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindOIDInfo(dwkeytype: u32, pvkey: *const ::core::ffi::c_void, dwgroupid: u32) -> *mut CRYPT_OID_INFO;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFormatObject(dwcertencodingtype: u32, dwformattype: u32, dwformatstrtype: u32, pformatstruct: *const ::core::ffi::c_void, lpszstructtype: super::super::Foundation::PSTR, pbencoded: *const u8, cbencoded: u32, pbformat: *mut ::core::ffi::c_void, pcbformat: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFreeOIDFunctionAddress(hfuncaddr: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGenKey(hprov: usize, algid: u32, dwflags: CRYPT_KEY_FLAGS, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGenRandom(hprov: usize, dwlen: u32, pbbuffer: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetAsyncParam(hasync: HCRYPTASYNC, pszparamoid: super::super::Foundation::PSTR, ppvparam: *mut *mut ::core::ffi::c_void, ppfnfree: *mut ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultOIDDllList(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pwszdlllist: super::super::Foundation::PWSTR, pcchdlllist: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultOIDFunctionAddress(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pwszdll: super::super::Foundation::PWSTR, dwflags: u32, ppvfuncaddr: *mut *mut ::core::ffi::c_void, phfuncaddr: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultProviderA(dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32, pszprovname: super::super::Foundation::PSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultProviderW(dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32, pszprovname: super::super::Foundation::PWSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetHashParam(hhash: usize, dwparam: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetKeyIdentifierProperty(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetKeyParam(hkey: usize, dwparam: CRYPT_KEY_PARAM_ID, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptGetMessageCertificates(dwmsgandcertencodingtype: u32, hcryptprov: usize, dwflags: u32, pbsignedblob: *const u8, cbsignedblob: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptGetMessageSignerCount(dwmsgencodingtype: u32, pbsignedblob: *const u8, cbsignedblob: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetOIDFunctionAddress(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pszoid: super::super::Foundation::PSTR, dwflags: u32, ppvfuncaddr: *mut *mut ::core::ffi::c_void, phfuncaddr: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetOIDFunctionValue(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR, pwszvaluename: super::super::Foundation::PWSTR, pdwvaluetype: *mut u32, pbvaluedata: *mut u8, pcbvaluedata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetObjectUrl(pszurloid: super::super::Foundation::PSTR, pvpara: *const ::core::ffi::c_void, dwflags: CRYPT_GET_URL_FLAGS, purlarray: *mut CRYPT_URL_ARRAY, pcburlarray: *mut u32, purlinfo: *mut CRYPT_URL_INFO, pcburlinfo: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetProvParam(hprov: usize, dwparam: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetUserKey(hprov: usize, dwkeyspec: u32, phuserkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashCertificate(hcryptprov: usize, algid: u32, dwflags: u32, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashCertificate2(pwszcnghashalgid: super::super::Foundation::PWSTR, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashData(hhash: usize, pbdata: *const u8, dwdatalen: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashMessage(phashpara: *const CRYPT_HASH_MESSAGE_PARA, fdetachedhash: super::super::Foundation::BOOL, ctobehashed: u32, rgpbtobehashed: *const *const u8, rgcbtobehashed: *const u32, pbhashedblob: *mut u8, pcbhashedblob: *mut u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashPublicKeyInfo(hcryptprov: usize, algid: u32, dwflags: u32, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashSessionKey(hhash: usize, hkey: usize, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashToBeSigned(hcryptprov: usize, dwcertencodingtype: u32, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportKey(hprov: usize, pbdata: *const u8, dwdatalen: u32, hpubkey: usize, dwflags: CRYPT_KEY_FLAGS, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPKCS8(sprivatekeyandparams: ::core::mem::ManuallyDrop<CRYPT_PKCS8_IMPORT_PARAMS>, dwflags: CRYPT_KEY_FLAGS, phcryptprov: *mut usize, pvauxinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfo(hcryptprov: usize, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfoEx(hcryptprov: usize, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, aikeyalg: u32, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfoEx2(dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, dwflags: CRYPT_IMPORT_PUBLIC_KEY_FLAGS, pvauxinfo: *const ::core::ffi::c_void, phkey: *mut BCRYPT_KEY_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInitOIDFunctionSet(pszfuncname: super::super::Foundation::PSTR, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallCancelRetrieval(pfncancel: ::windows::runtime::RawPtr, pvarg: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallDefaultContext(hcryptprov: usize, dwdefaulttype: CRYPT_DEFAULT_CONTEXT_TYPE, pvdefaultpara: *const ::core::ffi::c_void, dwflags: CRYPT_DEFAULT_CONTEXT_FLAGS, pvreserved: *mut ::core::ffi::c_void, phdefaultcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallOIDFunctionAddress(hmodule: super::super::Foundation::HINSTANCE, dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, cfuncentry: u32, rgfuncentry: *const CRYPT_OID_FUNC_ENTRY, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptMemAlloc(cbsize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptMemFree(pv: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptMemRealloc(pv: *const ::core::ffi::c_void, cbsize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCalculateEncodedLength(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: u32, pvmsgencodeinfo: *const ::core::ffi::c_void, pszinnercontentobjid: super::super::Foundation::PSTR, cbdata: u32) -> u32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgClose(hcryptmsg: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgControl(hcryptmsg: *const ::core::ffi::c_void, dwflags: u32, dwctrltype: u32, pvctrlpara: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCountersign(hcryptmsg: *const ::core::ffi::c_void, dwindex: u32, ccountersigners: u32, rgcountersigners: *const CMSG_SIGNER_ENCODE_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCountersignEncoded(dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, ccountersigners: u32, rgcountersigners: *const CMSG_SIGNER_ENCODE_INFO, pbcountersignature: *mut u8, pcbcountersignature: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptMsgDuplicate(hcryptmsg: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgEncodeAndSignCTL(dwmsgencodingtype: u32, pctlinfo: *const CTL_INFO, psigninfo: *const CMSG_SIGNED_ENCODE_INFO, dwflags: u32, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgGetAndVerifySigner(hcryptmsg: *const ::core::ffi::c_void, csignerstore: u32, rghsignerstore: *const *const ::core::ffi::c_void, dwflags: u32, ppsigner: *mut *mut CERT_CONTEXT, pdwsignerindex: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgGetParam(hcryptmsg: *const ::core::ffi::c_void, dwparamtype: u32, dwindex: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgOpenToDecode(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: u32, hcryptprov: usize, precipientinfo: *mut CERT_INFO, pstreaminfo: *const ::core::mem::ManuallyDrop<CMSG_STREAM_INFO>) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgOpenToEncode(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: CRYPT_MSG_TYPE, pvmsgencodeinfo: *const ::core::ffi::c_void, pszinnercontentobjid: super::super::Foundation::PSTR, pstreaminfo: *const ::core::mem::ManuallyDrop<CMSG_STREAM_INFO>) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgSignCTL(dwmsgencodingtype: u32, pbctlcontent: *const u8, cbctlcontent: u32, psigninfo: *const CMSG_SIGNED_ENCODE_INFO, dwflags: u32, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgUpdate(hcryptmsg: *const ::core::ffi::c_void, pbdata: *const u8, cbdata: u32, ffinal: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgVerifyCountersignatureEncoded(hcryptprov: usize, dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, pbsignerinfocountersignature: *const u8, cbsignerinfocountersignature: u32, pcicountersigner: *const CERT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgVerifyCountersignatureEncodedEx(hcryptprov: usize, dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, pbsignerinfocountersignature: *const u8, cbsignerinfocountersignature: u32, dwsignertype: u32, pvsigner: *const ::core::ffi::c_void, dwflags: u32, pvextra: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptProtectData(pdatain: *const CRYPTOAPI_BLOB, szdatadescr: super::super::Foundation::PWSTR, poptionalentropy: *const CRYPTOAPI_BLOB, pvreserved: *mut ::core::ffi::c_void, ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT, dwflags: u32, pdataout: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptProtectMemory(pdatain: *mut ::core::ffi::c_void, cbdatain: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptQueryObject(
        dwobjecttype: CERT_QUERY_OBJECT_TYPE,
        pvobject: *const ::core::ffi::c_void,
        dwexpectedcontenttypeflags: CERT_QUERY_CONTENT_TYPE_FLAGS,
        dwexpectedformattypeflags: CERT_QUERY_FORMAT_TYPE_FLAGS,
        dwflags: u32,
        pdwmsgandcertencodingtype: *mut CERT_QUERY_ENCODING_TYPE,
        pdwcontenttype: *mut CERT_QUERY_CONTENT_TYPE,
        pdwformattype: *mut CERT_QUERY_FORMAT_TYPE,
        phcertstore: *mut *mut ::core::ffi::c_void,
        phmsg: *mut *mut ::core::ffi::c_void,
        ppvcontext: *mut *mut ::core::ffi::c_void,
    ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterDefaultOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, dwindex: u32, pwszdll: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR, pwszdll: super::super::Foundation::PWSTR, pszoverridefuncname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterOIDInfo(pinfo: *const CRYPT_OID_INFO, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptReleaseContext(hprov: usize, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveObjectByUrlA(pszurl: super::super::Foundation::PSTR, pszobjectoid: super::super::Foundation::PSTR, dwretrievalflags: u32, dwtimeout: u32, ppvobject: *mut *mut ::core::ffi::c_void, hasyncretrieve: HCRYPTASYNC, pcredentials: *const CRYPT_CREDENTIALS, pvverify: *const ::core::ffi::c_void, pauxinfo: *mut CRYPT_RETRIEVE_AUX_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveObjectByUrlW(pszurl: super::super::Foundation::PWSTR, pszobjectoid: super::super::Foundation::PSTR, dwretrievalflags: u32, dwtimeout: u32, ppvobject: *mut *mut ::core::ffi::c_void, hasyncretrieve: HCRYPTASYNC, pcredentials: *const CRYPT_CREDENTIALS, pvverify: *const ::core::ffi::c_void, pauxinfo: *mut CRYPT_RETRIEVE_AUX_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveTimeStamp(wszurl: super::super::Foundation::PWSTR, dwretrievalflags: u32, dwtimeout: u32, pszhashid: super::super::Foundation::PSTR, ppara: *const CRYPT_TIMESTAMP_PARA, pbdata: *const u8, cbdata: u32, pptscontext: *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner: *mut *mut CERT_CONTEXT, phstore: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetAsyncParam(hasync: HCRYPTASYNC, pszparamoid: super::super::Foundation::PSTR, pvparam: *const ::core::ffi::c_void, pfnfree: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetHashParam(hhash: usize, dwparam: CRYPT_SET_HASH_PARAM, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetKeyIdentifierProperty(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetKeyParam(hkey: usize, dwparam: CRYPT_KEY_PARAM_ID, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CryptSetOIDFunctionValue(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR, pwszvaluename: super::super::Foundation::PWSTR, dwvaluetype: super::super::System::Registry::REG_VALUE_TYPE, pbvaluedata: *const u8, cbvaluedata: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProvParam(hprov: usize, dwparam: CRYPT_SET_PROV_PARAM_ID, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderA(pszprovname: super::super::Foundation::PSTR, dwprovtype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderExA(pszprovname: super::super::Foundation::PSTR, dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderExW(pszprovname: super::super::Foundation::PWSTR, dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderW(pszprovname: super::super::Foundation::PWSTR, dwprovtype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignAndEncodeCertificate(hcryptprovorncryptkey: usize, dwkeyspec: CERT_KEY_SPEC, dwcertencodingtype: u32, lpszstructtype: super::super::Foundation::PSTR, pvstructinfo: *const ::core::ffi::c_void, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo: *const ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignAndEncryptMessage(psignpara: *const CRYPT_SIGN_MESSAGE_PARA, pencryptpara: *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert: u32, rgprecipientcert: *const *const CERT_CONTEXT, pbtobesignedandencrypted: *const u8, cbtobesignedandencrypted: u32, pbsignedandencryptedblob: *mut u8, pcbsignedandencryptedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignCertificate(hcryptprovorncryptkey: usize, dwkeyspec: u32, dwcertencodingtype: u32, pbencodedtobesigned: *const u8, cbencodedtobesigned: u32, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo: *const ::core::ffi::c_void, pbsignature: *mut u8, pcbsignature: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignHashA(hhash: usize, dwkeyspec: u32, szdescription: super::super::Foundation::PSTR, dwflags: u32, pbsignature: *mut u8, pdwsiglen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignHashW(hhash: usize, dwkeyspec: u32, szdescription: super::super::Foundation::PWSTR, dwflags: u32, pbsignature: *mut u8, pdwsiglen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignMessage(psignpara: *const CRYPT_SIGN_MESSAGE_PARA, fdetachedsignature: super::super::Foundation::BOOL, ctobesigned: u32, rgpbtobesigned: *const *const u8, rgcbtobesigned: *const u32, pbsignedblob: *mut u8, pcbsignedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignMessageWithKey(psignpara: *const CRYPT_KEY_SIGN_MESSAGE_PARA, pbtobesigned: *const u8, cbtobesigned: u32, pbsignedblob: *mut u8, pcbsignedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptStringToBinaryA(pszstring: super::super::Foundation::PSTR, cchstring: u32, dwflags: CRYPT_STRING, pbbinary: *mut u8, pcbbinary: *mut u32, pdwskip: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptStringToBinaryW(pszstring: super::super::Foundation::PWSTR, cchstring: u32, dwflags: CRYPT_STRING, pbbinary: *mut u8, pcbbinary: *mut u32, pdwskip: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUninstallCancelRetrieval(dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUninstallDefaultContext(hdefaultcontext: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnprotectData(pdatain: *const CRYPTOAPI_BLOB, ppszdatadescr: *mut super::super::Foundation::PWSTR, poptionalentropy: *const CRYPTOAPI_BLOB, pvreserved: *mut ::core::ffi::c_void, ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT, dwflags: u32, pdataout: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnprotectMemory(pdatain: *mut ::core::ffi::c_void, cbdatain: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterDefaultOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pwszdll: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterOIDInfo(pinfo: *const CRYPT_OID_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUpdateProtectedState(poldsid: super::super::Foundation::PSID, pwszoldpassword: super::super::Foundation::PWSTR, dwflags: u32, pdwsuccesscount: *mut u32, pdwfailurecount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyCertificateSignature(hcryptprov: usize, dwcertencodingtype: u32, pbencoded: *const u8, cbencoded: u32, ppublickey: *const CERT_PUBLIC_KEY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyCertificateSignatureEx(hcryptprov: usize, dwcertencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, dwissuertype: u32, pvissuer: *const ::core::ffi::c_void, dwflags: CRYPT_VERIFY_CERT_FLAGS, pvextra: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyDetachedMessageHash(phashpara: *const CRYPT_HASH_MESSAGE_PARA, pbdetachedhashblob: *const u8, cbdetachedhashblob: u32, ctobehashed: u32, rgpbtobehashed: *const *const u8, rgcbtobehashed: *const u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyDetachedMessageSignature(pverifypara: *const ::core::mem::ManuallyDrop<CRYPT_VERIFY_MESSAGE_PARA>, dwsignerindex: u32, pbdetachedsignblob: *const u8, cbdetachedsignblob: u32, ctobesigned: u32, rgpbtobesigned: *const *const u8, rgcbtobesigned: *const u32, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageHash(phashpara: *const CRYPT_HASH_MESSAGE_PARA, pbhashedblob: *const u8, cbhashedblob: u32, pbtobehashed: *mut u8, pcbtobehashed: *mut u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageSignature(pverifypara: *const ::core::mem::ManuallyDrop<CRYPT_VERIFY_MESSAGE_PARA>, dwsignerindex: u32, pbsignedblob: *const u8, cbsignedblob: u32, pbdecoded: *mut u8, pcbdecoded: *mut u32, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageSignatureWithKey(pverifypara: *const CRYPT_KEY_VERIFY_MESSAGE_PARA, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pbsignedblob: *const u8, cbsignedblob: u32, pbdecoded: *mut u8, pcbdecoded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifySignatureA(hhash: usize, pbsignature: *const u8, dwsiglen: u32, hpubkey: usize, szdescription: super::super::Foundation::PSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifySignatureW(hhash: usize, pbsignature: *const u8, dwsiglen: u32, hpubkey: usize, szdescription: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyTimeStampSignature(pbtscontentinfo: *const u8, cbtscontentinfo: u32, pbdata: *const u8, cbdata: u32, hadditionalstore: *const ::core::ffi::c_void, pptscontext: *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner: *mut *mut CERT_CONTEXT, phstore: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlAddObject(hsignatureorobject: *const ::core::ffi::c_void, dwflags: u32, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, ppobject: *mut *mut CRYPT_XML_OBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlClose(hcryptxml: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlCreateReference(hcryptxml: *const ::core::ffi::c_void, dwflags: u32, wszid: super::super::Foundation::PWSTR, wszuri: super::super::Foundation::PWSTR, wsztype: super::super::Foundation::PWSTR, pdigestmethod: *const CRYPT_XML_ALGORITHM, ctransform: u32, rgtransform: *const CRYPT_XML_ALGORITHM, phreference: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlDigestReference(hreference: *const ::core::ffi::c_void, dwflags: u32, pdataproviderin: *const ::core::mem::ManuallyDrop<CRYPT_XML_DATA_PROVIDER>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlEncode(hcryptxml: *const ::core::ffi::c_void, dwcharset: CRYPT_XML_CHARSET, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pvcallbackstate: *mut ::core::ffi::c_void, pfnwrite: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlEnumAlgorithmInfo(dwgroupid: u32, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumalginfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlFindAlgorithmInfo(dwfindbytype: u32, pvfindby: *const ::core::ffi::c_void, dwgroupid: u32, dwflags: u32) -> *mut CRYPT_XML_ALGORITHM_INFO;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetAlgorithmInfo(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, dwflags: CRYPT_XML_FLAGS, ppalginfo: *mut *mut CRYPT_XML_ALGORITHM_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetDocContext(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_DOC_CTXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetReference(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_REFERENCE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetSignature(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_SIGNATURE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlGetStatus(hcryptxml: *const ::core::ffi::c_void, pstatus: *mut CRYPT_XML_STATUS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetTransforms(ppconfig: *mut *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlImportPublicKey(dwflags: CRYPT_XML_FLAGS, pkeyvalue: *const CRYPT_XML_KEY_VALUE, phkey: *mut BCRYPT_KEY_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlOpenToDecode(pconfig: *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags: CRYPT_XML_FLAGS, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, phcryptxml: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlOpenToEncode(pconfig: *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags: CRYPT_XML_FLAGS, wszid: super::super::Foundation::PWSTR, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, phsignature: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlSetHMACSecret(hsignature: *const ::core::ffi::c_void, pbsecret: *const u8, cbsecret: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlSign(hsignature: *const ::core::ffi::c_void, hkey: usize, dwkeyspec: CERT_KEY_SPEC, dwflags: CRYPT_XML_FLAGS, dwkeyinfospec: CRYPT_XML_KEYINFO_SPEC, pvkeyinfospec: *const ::core::ffi::c_void, psignaturemethod: *const CRYPT_XML_ALGORITHM, pcanonicalization: *const CRYPT_XML_ALGORITHM) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlVerifySignature(hsignature: *const ::core::ffi::c_void, hkey: BCRYPT_KEY_HANDLE, dwflags: CRYPT_XML_FLAGS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Decrypt(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, foaep: super::super::Foundation::BOOL, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Encrypt(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, foaep: super::super::Foundation::BOOL, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindCertsByIssuer(pcertchains: *mut CERT_CHAIN, pcbcertchains: *mut u32, pccertchains: *mut u32, pbencodedissuername: *const u8, cbencodedissuername: u32, pwszpurpose: super::super::Foundation::PWSTR, dwkeyspec: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeToken(pallocmemory: *const GENERIC_XML_TOKEN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateDerivedKey(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cblabel: u32, plabel: *const u8, cbnonce: u32, pnonce: *const u8, derivedkeylength: u32, offset: u32, algid: super::super::Foundation::PWSTR, pcbkey: *mut u32, ppkey: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn GetBrowserToken(dwparamtype: u32, pparam: *const ::core::ffi::c_void, pcbtoken: *mut u32, pptoken: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn GetCryptoTransform(hsymmetriccrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, mode: u32, padding: PaddingMode, feedbacksize: u32, direction: Direction, cbiv: u32, piv: *const u8, pphtransform: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn GetKeyedHash(hsymmetriccrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, pphhash: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetToken(cpolicychain: u32, ppolicychain: *const POLICY_ELEMENT, securitytoken: *mut *mut GENERIC_XML_TOKEN, phprooftokencrypto: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn HashCore(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn HashFinal(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportInformationCard(filename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn ManageCardSpace() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptCloseProtectionDescriptor(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptCreateClaim(hsubjectkey: usize, hauthoritykey: usize, dwclaimtype: u32, pparameterlist: *const BCryptBufferDesc, pbclaimblob: *mut u8, cbclaimblob: u32, pcbresult: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptCreatePersistedKey(hprovider: usize, phkey: *mut usize, pszalgid: super::super::Foundation::PWSTR, pszkeyname: super::super::Foundation::PWSTR, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptCreateProtectionDescriptor(pwszdescriptorstring: super::super::Foundation::PWSTR, dwflags: u32, phdescriptor: *mut super::NCRYPT_DESCRIPTOR_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptDecrypt(hkey: usize, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptDeleteKey(hkey: usize, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptDeriveKey(hsharedsecret: usize, pwszkdf: super::super::Foundation::PWSTR, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptEncrypt(hkey: usize, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumAlgorithms(hprovider: usize, dwalgoperations: NCRYPT_OPERATION, pdwalgcount: *mut u32, ppalglist: *mut *mut NCryptAlgorithmName, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumKeys(hprovider: usize, pszscope: super::super::Foundation::PWSTR, ppkeyname: *mut *mut NCryptKeyName, ppenumstate: *mut *mut ::core::ffi::c_void, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumStorageProviders(pdwprovidercount: *mut u32, ppproviderlist: *mut *mut NCryptProviderName, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptExportKey(hkey: usize, hexportkey: usize, pszblobtype: super::super::Foundation::PWSTR, pparameterlist: *const BCryptBufferDesc, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptFinalizeKey(hkey: usize, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptFreeBuffer(pvinput: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptFreeObject(hobject: usize) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptGetProperty(hobject: usize, pszproperty: super::super::Foundation::PWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: super::OBJECT_SECURITY_INFORMATION) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptGetProtectionDescriptorInfo(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, pmempara: *const ::core::mem::ManuallyDrop<NCRYPT_ALLOC_PARA>, dwinfotype: u32, ppvinfo: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptImportKey(hprovider: usize, himportkey: usize, pszblobtype: super::super::Foundation::PWSTR, pparameterlist: *const BCryptBufferDesc, phkey: *mut usize, pbdata: *const u8, cbdata: u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptIsAlgSupported(hprovider: usize, pszalgid: super::super::Foundation::PWSTR, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptIsKeyHandle(hkey: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptKeyDerivation(hkey: usize, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptNotifyChangeKey(hprovider: usize, phevent: *mut super::super::Foundation::HANDLE, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptOpenKey(hprovider: usize, phkey: *mut usize, pszkeyname: super::super::Foundation::PWSTR, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptOpenStorageProvider(phprovider: *mut usize, pszprovidername: super::super::Foundation::PWSTR, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptProtectSecret(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: u32, pbdata: *const u8, cbdata: u32, pmempara: *const ::core::mem::ManuallyDrop<NCRYPT_ALLOC_PARA>, hwnd: super::super::Foundation::HWND, ppbprotectedblob: *mut *mut u8, pcbprotectedblob: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptQueryProtectionDescriptorName(pwszname: super::super::Foundation::PWSTR, pwszdescriptorstring: super::super::Foundation::PWSTR, pcdescriptorstring: *mut usize, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptRegisterProtectionDescriptorName(pwszname: super::super::Foundation::PWSTR, pwszdescriptorstring: super::super::Foundation::PWSTR, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptSecretAgreement(hprivkey: usize, hpubkey: usize, phagreedsecret: *mut usize, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptSetProperty(hobject: usize, pszproperty: super::super::Foundation::PWSTR, pbinput: *const u8, cbinput: u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptSignHash(hkey: usize, ppaddinginfo: *const ::core::ffi::c_void, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *mut u8, cbsignature: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptStreamClose(hstream: super::NCRYPT_STREAM_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToProtect(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: u32, hwnd: super::super::Foundation::HWND, pstreaminfo: *const ::core::mem::ManuallyDrop<NCRYPT_PROTECT_STREAM_INFO>, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToUnprotect(pstreaminfo: *const ::core::mem::ManuallyDrop<NCRYPT_PROTECT_STREAM_INFO>, dwflags: u32, hwnd: super::super::Foundation::HWND, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToUnprotectEx(pstreaminfo: *const ::core::mem::ManuallyDrop<NCRYPT_PROTECT_STREAM_INFO_EX>, dwflags: u32, hwnd: super::super::Foundation::HWND, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamUpdate(hstream: super::NCRYPT_STREAM_HANDLE, pbdata: *const u8, cbdata: usize, ffinal: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptTranslateHandle(phprovider: *mut usize, phkey: *mut usize, hlegacyprov: usize, hlegacykey: usize, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptUnprotectSecret(phdescriptor: *mut super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: NCRYPT_FLAGS, pbprotectedblob: *const u8, cbprotectedblob: u32, pmempara: *const ::core::mem::ManuallyDrop<NCRYPT_ALLOC_PARA>, hwnd: super::super::Foundation::HWND, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptVerifyClaim(hsubjectkey: usize, hauthoritykey: usize, dwclaimtype: u32, pparameterlist: *const BCryptBufferDesc, pbclaimblob: *const u8, cbclaimblob: u32, poutput: *mut BCryptBufferDesc, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptVerifySignature(hkey: usize, ppaddinginfo: *const ::core::ffi::c_void, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *const u8, cbsignature: u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXExportCertStore(hstore: *const ::core::ffi::c_void, ppfx: *mut CRYPTOAPI_BLOB, szpassword: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXExportCertStoreEx(hstore: *const ::core::ffi::c_void, ppfx: *mut CRYPTOAPI_BLOB, szpassword: super::super::Foundation::PWSTR, pvpara: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXImportCertStore(ppfx: *const CRYPTOAPI_BLOB, szpassword: super::super::Foundation::PWSTR, dwflags: CRYPT_KEY_FLAGS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXIsPFXBlob(ppfx: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXVerifyPassword(ppfx: *const CRYPTOAPI_BLOB, szpassword: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SignHash(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbhash: u32, phash: *const u8, hashalgoid: super::super::Foundation::PWSTR, pcbsig: *mut u32, ppsig: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn TransformBlock(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn TransformFinalBlock(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyHash(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbhash: u32, phash: *const u8, hashalgoid: super::super::Foundation::PWSTR, cbsig: u32, psig: *const u8, pfverified: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
}
