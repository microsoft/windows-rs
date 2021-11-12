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
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptAddContextFunction(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, dwposition: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCloseAlgorithmProvider(halgorithm: BCRYPT_ALG_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptConfigureContext(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, pconfig: *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptConfigureContextFunction(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pconfig: *const CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateContext(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, pconfig: *const CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut *mut ::core::ffi::c_void, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateMultiHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut *mut ::core::ffi::c_void, nhashes: u32, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDecrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pbiv: *mut u8, cbiv: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeleteContext(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKey(hsharedsecret: *const ::core::ffi::c_void, pwszkdf: super::super::Foundation::PWSTR, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKeyCapi(hhash: *const ::core::ffi::c_void, htargetalg: BCRYPT_ALG_HANDLE, pbderivedkey: *mut u8, cbderivedkey: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKeyPBKDF2(hprf: BCRYPT_ALG_HANDLE, pbpassword: *const u8, cbpassword: u32, pbsalt: *const u8, cbsalt: u32, citerations: u64, pbderivedkey: *mut u8, cbderivedkey: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroyHash(hhash: *mut ::core::ffi::c_void) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroyKey(hkey: BCRYPT_KEY_HANDLE) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroySecret(hsecret: *mut ::core::ffi::c_void) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDuplicateHash(hhash: *const ::core::ffi::c_void, phnewhash: *mut *mut ::core::ffi::c_void, pbhashobject: *mut u8, cbhashobject: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDuplicateKey(hkey: BCRYPT_KEY_HANDLE, phnewkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEncrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pbiv: *mut u8, cbiv: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumAlgorithms(dwalgoperations: BCRYPT_OPERATION, palgcount: *mut u32, ppalglist: *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContextFunctionProviders(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContextFunctions(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTIONS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContexts(dwtable: BCRYPT_TABLE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXTS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumProviders(pszalgid: super::super::Foundation::PWSTR, pimplcount: *mut u32, ppimpllist: *mut *mut BCRYPT_PROVIDER_NAME, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumRegisteredProviders(pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDERS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptExportKey(hkey: BCRYPT_KEY_HANDLE, hexportkey: BCRYPT_KEY_HANDLE, pszblobtype: super::super::Foundation::PWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptFinalizeKeyPair(hkey: BCRYPT_KEY_HANDLE, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptFinishHash(hhash: *mut ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    pub fn BCryptFreeBuffer(pvbuffer: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenRandom(halgorithm: BCRYPT_ALG_HANDLE, pbbuffer: *mut u8, cbbuffer: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenerateKeyPair(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, dwlength: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenerateSymmetricKey(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGetFipsAlgorithmMode(pfenabled: *mut u8) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGetProperty(hobject: *const ::core::ffi::c_void, pszproperty: super::super::Foundation::PWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptHash(halgorithm: BCRYPT_ALG_HANDLE, pbsecret: *const u8, cbsecret: u32, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptHashData(hhash: *mut ::core::ffi::c_void, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptImportKey(halgorithm: BCRYPT_ALG_HANDLE, himportkey: BCRYPT_KEY_HANDLE, pszblobtype: super::super::Foundation::PWSTR, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: *mut u8, cbkeyobject: u32, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptImportKeyPair(halgorithm: BCRYPT_ALG_HANDLE, himportkey: BCRYPT_KEY_HANDLE, pszblobtype: super::super::Foundation::PWSTR, phkey: *mut BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptKeyDerivation(hkey: BCRYPT_KEY_HANDLE, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptOpenAlgorithmProvider(phalgorithm: *mut BCRYPT_ALG_HANDLE, pszalgid: super::super::Foundation::PWSTR, pszimplementation: super::super::Foundation::PWSTR, dwflags: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptProcessMultiOperations(hobject: *mut ::core::ffi::c_void, operationtype: BCRYPT_MULTI_OPERATION_TYPE, poperations: *const ::core::ffi::c_void, cboperations: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextConfiguration(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextFunctionConfiguration(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_CONTEXT_FUNCTION_CONFIG) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextFunctionProperty(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pszproperty: super::super::Foundation::PWSTR, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryProviderRegistration(pszprovider: super::super::Foundation::PWSTR, dwmode: BCRYPT_QUERY_PROVIDER_MODE, dwinterface: BCRYPT_INTERFACE, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDER_REG) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptRegisterConfigChangeNotify(phevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptRemoveContextFunction(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptResolveProviders(pszcontext: super::super::Foundation::PWSTR, dwinterface: u32, pszfunction: super::super::Foundation::PWSTR, pszprovider: super::super::Foundation::PWSTR, dwmode: BCRYPT_QUERY_PROVIDER_MODE, dwflags: BCRYPT_RESOLVE_PROVIDERS_FLAGS, pcbbuffer: *mut u32, ppbuffer: *mut *mut CRYPT_PROVIDER_REFS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSecretAgreement(hprivkey: BCRYPT_KEY_HANDLE, hpubkey: BCRYPT_KEY_HANDLE, phagreedsecret: *mut *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSetContextFunctionProperty(dwtable: BCRYPT_TABLE, pszcontext: super::super::Foundation::PWSTR, dwinterface: BCRYPT_INTERFACE, pszfunction: super::super::Foundation::PWSTR, pszproperty: super::super::Foundation::PWSTR, cbvalue: u32, pbvalue: *const u8) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSetProperty(hobject: *mut ::core::ffi::c_void, pszproperty: super::super::Foundation::PWSTR, pbinput: *const u8, cbinput: u32, dwflags: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSignHash(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: *const ::core::ffi::c_void, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptUnregisterConfigChangeNotify(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptVerifySignature(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: *const ::core::ffi::c_void, pbhash: *const u8, cbhash: u32, pbsignature: *const u8, cbsignature: u32, dwflags: NCRYPT_FLAGS) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCRLContextToStore(hcertstore: *const ::core::ffi::c_void, pcrlcontext: *const CRL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCRLLinkToStore(hcertstore: *const ::core::ffi::c_void, pcrlcontext: *const CRL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCTLContextToStore(hcertstore: *const ::core::ffi::c_void, pctlcontext: *const CTL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCTLLinkToStore(hcertstore: *const ::core::ffi::c_void, pctlcontext: *const CTL_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCertificateContextToStore(hcertstore: *const ::core::ffi::c_void, pcertcontext: *const CERT_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCertificateLinkToStore(hcertstore: *const ::core::ffi::c_void, pcertcontext: *const CERT_CONTEXT, dwadddisposition: u32, ppstorecontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCRLToStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, pbcrlencoded: *const u8, cbcrlencoded: u32, dwadddisposition: u32, ppcrlcontext: *mut *mut CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCTLToStore(hcertstore: *const ::core::ffi::c_void, dwmsgandcertencodingtype: u32, pbctlencoded: *const u8, cbctlencoded: u32, dwadddisposition: u32, ppctlcontext: *mut *mut CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, pbcertencoded: *const u8, cbcertencoded: u32, dwadddisposition: u32, ppcertcontext: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToSystemStoreA(szcertstorename: super::super::Foundation::PSTR, pbcertencoded: *const u8, cbcertencoded: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToSystemStoreW(szcertstorename: super::super::Foundation::PWSTR, pbcertencoded: *const u8, cbcertencoded: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEnhancedKeyUsageIdentifier(pcertcontext: *const CERT_CONTEXT, pszusageidentifier: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    pub fn CertAddRefServerOcspResponse(hserverocspresponse: *const ::core::ffi::c_void);
    pub fn CertAddRefServerOcspResponseContext(pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddSerializedElementToStore(hcertstore: *const ::core::ffi::c_void, pbelement: *const u8, cbelement: u32, dwadddisposition: u32, dwflags: u32, dwcontexttypeflags: u32, pdwcontexttype: *mut u32, ppvcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddStoreToCollection(hcollectionstore: *const ::core::ffi::c_void, hsiblingstore: *const ::core::ffi::c_void, dwupdateflags: u32, dwpriority: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAlgIdToOID(dwalgid: u32) -> super::super::Foundation::PSTR;
    pub fn CertCloseServerOcspResponse(hserverocspresponse: *const ::core::ffi::c_void, dwflags: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCloseStore(hcertstore: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareCertificate(dwcertencodingtype: u32, pcertid1: *const CERT_INFO, pcertid2: *const CERT_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareCertificateName(dwcertencodingtype: u32, pcertname1: *const CRYPTOAPI_BLOB, pcertname2: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareIntegerBlob(pint1: *const CRYPTOAPI_BLOB, pint2: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertComparePublicKeyInfo(dwcertencodingtype: u32, ppublickey1: *const CERT_PUBLIC_KEY_INFO, ppublickey2: *const CERT_PUBLIC_KEY_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertControlStore(hcertstore: *const ::core::ffi::c_void, dwflags: CERT_CONTROL_STORE_FLAGS, dwctrltype: u32, pvctrlpara: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCRLContext(dwcertencodingtype: u32, pbcrlencoded: *const u8, cbcrlencoded: u32) -> *mut CRL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCTLContext(dwmsgandcertencodingtype: u32, pbctlencoded: *const u8, cbctlencoded: u32) -> *mut CTL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCTLEntryFromCertificateContextProperties(pcertcontext: *const CERT_CONTEXT, coptattr: u32, rgoptattr: *const CRYPT_ATTRIBUTE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pctlentry: *mut CTL_ENTRY, pcbctlentry: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCertificateChainEngine(pconfig: *const CERT_CHAIN_ENGINE_CONFIG, phchainengine: *mut HCERTCHAINENGINE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCertificateContext(dwcertencodingtype: u32, pbcertencoded: *const u8, cbcertencoded: u32) -> *mut CERT_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateContext(dwcontexttype: u32, dwencodingtype: u32, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pcreatepara: *const CERT_CREATE_CONTEXT_PARA) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateSelfSignCertificate(hcryptprovorncryptkey: usize, psubjectissuerblob: *const CRYPTOAPI_BLOB, dwflags: CERT_CREATE_SELFSIGN_FLAGS, pkeyprovinfo: *const CRYPT_KEY_PROV_INFO, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pstarttime: *const super::super::Foundation::SYSTEMTIME, pendtime: *const super::super::Foundation::SYSTEMTIME, pextensions: *const CERT_EXTENSIONS) -> *mut CERT_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCRLFromStore(pcrlcontext: *const CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCTLFromStore(pctlcontext: *const CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCertificateFromStore(pcertcontext: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCRLContext(pcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCTLContext(pctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCertificateChain(pchaincontext: *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCertificateContext(pcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    pub fn CertDuplicateStore(hcertstore: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCRLContextProperties(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCRLsInStore(hcertstore: *const ::core::ffi::c_void, pprevcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCTLContextProperties(pctlcontext: *const CTL_CONTEXT, dwpropid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCTLsInStore(hcertstore: *const ::core::ffi::c_void, pprevctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCertificateContextProperties(pcertcontext: *const CERT_CONTEXT, dwpropid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCertificatesInStore(hcertstore: *const ::core::ffi::c_void, pprevcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenum: PFN_CERT_ENUM_PHYSICAL_STORE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSubjectInSortedCTL(pctlcontext: *const CTL_CONTEXT, ppvnextsubject: *mut *mut ::core::ffi::c_void, psubjectidentifier: *mut CRYPTOAPI_BLOB, pencodedattributes: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSystemStore(dwflags: u32, pvsystemstorelocationpara: *const ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void, pfnenum: PFN_CERT_ENUM_SYSTEM_STORE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSystemStoreLocation(dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenum: PFN_CERT_ENUM_SYSTEM_STORE_LOCATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindAttribute(pszobjid: super::super::Foundation::PSTR, cattr: u32, rgattr: *const CRYPT_ATTRIBUTE) -> *mut CRYPT_ATTRIBUTE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCRLInStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, dwfindflags: u32, dwfindtype: u32, pvfindpara: *const ::core::ffi::c_void, pprevcrlcontext: *const CRL_CONTEXT) -> *mut CRL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCTLInStore(hcertstore: *const ::core::ffi::c_void, dwmsgandcertencodingtype: u32, dwfindflags: u32, dwfindtype: CERT_FIND_TYPE, pvfindpara: *const ::core::ffi::c_void, pprevctlcontext: *const CTL_CONTEXT) -> *mut CTL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCertificateInCRL(pcert: *const CERT_CONTEXT, pcrlcontext: *const CRL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppcrlentry: *mut *mut CRL_ENTRY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCertificateInStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, dwfindflags: u32, dwfindtype: CERT_FIND_FLAGS, pvfindpara: *const ::core::ffi::c_void, pprevcertcontext: *const CERT_CONTEXT) -> *mut CERT_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindChainInStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, dwfindflags: CERT_FIND_CHAIN_IN_STORE_FLAGS, dwfindtype: u32, pvfindpara: *const ::core::ffi::c_void, pprevchaincontext: *const CERT_CHAIN_CONTEXT) -> *mut CERT_CHAIN_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindExtension(pszobjid: super::super::Foundation::PSTR, cextensions: u32, rgextensions: *const CERT_EXTENSION) -> *mut CERT_EXTENSION;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindRDNAttr(pszobjid: super::super::Foundation::PSTR, pname: *const CERT_NAME_INFO) -> *mut CERT_RDN_ATTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindSubjectInCTL(dwencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, pctlcontext: *const CTL_CONTEXT, dwflags: u32) -> *mut CTL_ENTRY;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindSubjectInSortedCTL(psubjectidentifier: *const CRYPTOAPI_BLOB, pctlcontext: *const CTL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pencodedattributes: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCRLContext(pcrlcontext: *const CRL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCTLContext(pctlcontext: *const CTL_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateChain(pchaincontext: *const CERT_CHAIN_CONTEXT);
    pub fn CertFreeCertificateChainEngine(hchainengine: HCERTCHAINENGINE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateChainList(prgpselection: *const *const CERT_CHAIN_CONTEXT);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateContext(pcertcontext: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    pub fn CertFreeServerOcspResponseContext(pserverocspresponsecontext: *const CERT_SERVER_OCSP_RESPONSE_CONTEXT);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCRLContextProperty(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCRLFromStore(hcertstore: *const ::core::ffi::c_void, pissuercontext: *const CERT_CONTEXT, pprevcrlcontext: *const CRL_CONTEXT, pdwflags: *mut u32) -> *mut CRL_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCTLContextProperty(pctlcontext: *const CTL_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCertificateChain(hchainengine: HCERTCHAINENGINE, pcertcontext: *const CERT_CONTEXT, ptime: *const super::super::Foundation::FILETIME, hadditionalstore: *const ::core::ffi::c_void, pchainpara: *const CERT_CHAIN_PARA, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppchaincontext: *mut *mut CERT_CHAIN_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCertificateContextProperty(pcertcontext: *const CERT_CONTEXT, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetEnhancedKeyUsage(pcertcontext: *const CERT_CONTEXT, dwflags: u32, pusage: *mut CTL_USAGE, pcbusage: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetIntendedKeyUsage(dwcertencodingtype: u32, pcertinfo: *const CERT_INFO, pbkeyusage: *mut u8, cbkeyusage: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetIssuerCertificateFromStore(hcertstore: *const ::core::ffi::c_void, psubjectcontext: *const CERT_CONTEXT, pprevissuercontext: *const CERT_CONTEXT, pdwflags: *mut u32) -> *mut CERT_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetNameStringA(pcertcontext: *const CERT_CONTEXT, dwtype: u32, dwflags: u32, pvtypepara: *const ::core::ffi::c_void, psznamestring: super::super::Foundation::PSTR, cchnamestring: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetNameStringW(pcertcontext: *const CERT_CONTEXT, dwtype: u32, dwflags: u32, pvtypepara: *const ::core::ffi::c_void, psznamestring: super::super::Foundation::PWSTR, cchnamestring: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetPublicKeyLength(dwcertencodingtype: u32, ppublickey: *const CERT_PUBLIC_KEY_INFO) -> u32;
    pub fn CertGetServerOcspResponseContext(hserverocspresponse: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> *mut CERT_SERVER_OCSP_RESPONSE_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetStoreProperty(hcertstore: *const ::core::ffi::c_void, dwpropid: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetSubjectCertificateFromStore(hcertstore: *const ::core::ffi::c_void, dwcertencodingtype: u32, pcertid: *const CERT_INFO) -> *mut CERT_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetValidUsages(ccerts: u32, rghcerts: *const *const CERT_CONTEXT, cnumoids: *mut i32, rghoids: *mut super::super::Foundation::PSTR, pcboids: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsRDNAttrsInCertificateName(dwcertencodingtype: u32, dwflags: u32, pcertname: *const CRYPTOAPI_BLOB, prdn: *const CERT_RDN) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsStrongHashToSign(pstrongsignpara: *const CERT_STRONG_SIGN_PARA, pwszcnghashalgid: super::super::Foundation::PWSTR, psigningcert: *const CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsValidCRLForCertificate(pcert: *const CERT_CONTEXT, pcrl: *const CRL_CONTEXT, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsWeakHash(dwhashusetype: u32, pwszcnghashalgid: super::super::Foundation::PWSTR, dwchainflags: u32, psignerchaincontext: *const CERT_CHAIN_CONTEXT, ptimestamp: *const super::super::Foundation::FILETIME, pwszfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertNameToStrA(dwcertencodingtype: u32, pname: *const CRYPTOAPI_BLOB, dwstrtype: CERT_STRING_TYPE, psz: super::super::Foundation::PSTR, csz: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertNameToStrW(dwcertencodingtype: u32, pname: *const CRYPTOAPI_BLOB, dwstrtype: CERT_STRING_TYPE, psz: super::super::Foundation::PWSTR, csz: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOIDToAlgId(pszobjid: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenServerOcspResponse(pchaincontext: *const CERT_CHAIN_CONTEXT, dwflags: u32, popenpara: *const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenStore(lpszstoreprovider: super::super::Foundation::PSTR, dwencodingtype: CERT_QUERY_ENCODING_TYPE, hcryptprov: usize, dwflags: CERT_OPEN_STORE_FLAGS, pvpara: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenSystemStoreA(hprov: usize, szsubsystemprotocol: super::super::Foundation::PSTR) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenSystemStoreW(hprov: usize, szsubsystemprotocol: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRDNValueToStrA(dwvaluetype: u32, pvalue: *const CRYPTOAPI_BLOB, psz: super::super::Foundation::PSTR, csz: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRDNValueToStrW(dwvaluetype: u32, pvalue: *const CRYPTOAPI_BLOB, psz: super::super::Foundation::PWSTR, csz: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRegisterPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pwszstorename: super::super::Foundation::PWSTR, pstoreinfo: *const CERT_PHYSICAL_STORE_INFO, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRegisterSystemStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pstoreinfo: *const CERT_SYSTEM_STORE_INFO, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRemoveEnhancedKeyUsageIdentifier(pcertcontext: *const CERT_CONTEXT, pszusageidentifier: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    pub fn CertRemoveStoreFromCollection(hcollectionstore: *const ::core::ffi::c_void, hsiblingstore: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertResyncCertificateChainEngine(hchainengine: HCERTCHAINENGINE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRetrieveLogoOrBiometricInfo(pcertcontext: *const CERT_CONTEXT, lpszlogoorbiometrictype: super::super::Foundation::PSTR, dwretrievalflags: u32, dwtimeout: u32, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ppwszmimetype: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSaveStore(hcertstore: *const ::core::ffi::c_void, dwencodingtype: CERT_QUERY_ENCODING_TYPE, dwsaveas: CERT_STORE_SAVE_AS, dwsaveto: CERT_STORE_SAVE_TO, pvsavetopara: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSelectCertificateChains(pselectioncontext: *const ::windows_sys::core::GUID, dwflags: u32, pchainparameters: *const CERT_SELECT_CHAIN_PARA, ccriteria: u32, rgpcriteria: *const CERT_SELECT_CRITERIA, hstore: *const ::core::ffi::c_void, pcselection: *mut u32, pprgpselection: *mut *mut *mut CERT_CHAIN_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCRLStoreElement(pcrlcontext: *const CRL_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCTLStoreElement(pctlcontext: *const CTL_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCertificateStoreElement(pcertcontext: *const CERT_CONTEXT, dwflags: u32, pbelement: *mut u8, pcbelement: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCRLContextProperty(pcrlcontext: *const CRL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCTLContextProperty(pctlcontext: *const CTL_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCertificateContextPropertiesFromCTLEntry(pcertcontext: *const CERT_CONTEXT, pctlentry: *const CTL_ENTRY, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCertificateContextProperty(pcertcontext: *const CERT_CONTEXT, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetEnhancedKeyUsage(pcertcontext: *const CERT_CONTEXT, pusage: *const CTL_USAGE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetStoreProperty(hcertstore: *const ::core::ffi::c_void, dwpropid: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertStrToNameA(dwcertencodingtype: u32, pszx500: super::super::Foundation::PSTR, dwstrtype: CERT_STRING_TYPE, pvreserved: *mut ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32, ppszerror: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertStrToNameW(dwcertencodingtype: u32, pszx500: super::super::Foundation::PWSTR, dwstrtype: CERT_STRING_TYPE, pvreserved: *mut ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32, ppszerror: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertUnregisterPhysicalStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32, pwszstorename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertUnregisterSystemStore(pvsystemstore: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCRLRevocation(dwcertencodingtype: u32, pcertid: *const CERT_INFO, ccrlinfo: u32, rgpcrlinfo: *const *const CRL_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCRLTimeValidity(ptimetoverify: *const super::super::Foundation::FILETIME, pcrlinfo: *const CRL_INFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCTLUsage(dwencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, psubjectusage: *const CTL_USAGE, dwflags: u32, pverifyusagepara: *const CTL_VERIFY_USAGE_PARA, pverifyusagestatus: *mut CTL_VERIFY_USAGE_STATUS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCertificateChainPolicy(pszpolicyoid: super::super::Foundation::PSTR, pchaincontext: *const CERT_CHAIN_CONTEXT, ppolicypara: *const CERT_CHAIN_POLICY_PARA, ppolicystatus: *mut CERT_CHAIN_POLICY_STATUS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyRevocation(dwencodingtype: u32, dwrevtype: u32, ccontext: u32, rgpvcontext: *const *const ::core::ffi::c_void, dwflags: u32, prevpara: *const CERT_REVOCATION_PARA, prevstatus: *mut CERT_REVOCATION_STATUS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifySubjectCertificateContext(psubject: *const CERT_CONTEXT, pissuer: *const CERT_CONTEXT, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyTimeValidity(ptimetoverify: *const super::super::Foundation::FILETIME, pcertinfo: *const CERT_INFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyValidityNesting(psubjectinfo: *const CERT_INFO, pissuerinfo: *const CERT_INFO) -> super::super::Foundation::BOOL;
    pub fn CloseCryptoHandle(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireCertificatePrivateKey(pcert: *const CERT_CONTEXT, dwflags: CRYPT_ACQUIRE_FLAGS, pvparameters: *const ::core::ffi::c_void, phcryptprovorncryptkey: *mut usize, pdwkeyspec: *mut CERT_KEY_SPEC, pfcallerfreeprovorncryptkey: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireContextA(phprov: *mut usize, szcontainer: super::super::Foundation::PSTR, szprovider: super::super::Foundation::PSTR, dwprovtype: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireContextW(phprov: *mut usize, szcontainer: super::super::Foundation::PWSTR, szprovider: super::super::Foundation::PWSTR, dwprovtype: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptBinaryToStringA(pbbinary: *const u8, cbbinary: u32, dwflags: CRYPT_STRING, pszstring: super::super::Foundation::PSTR, pcchstring: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptBinaryToStringW(pbbinary: *const u8, cbbinary: u32, dwflags: CRYPT_STRING, pszstring: super::super::Foundation::PWSTR, pcchstring: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCloseAsyncHandle(hasync: HCRYPTASYNC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptContextAddRef(hprov: usize, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateAsyncHandle(dwflags: u32, phasync: *mut HCRYPTASYNC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateHash(hprov: usize, algid: u32, hkey: usize, dwflags: u32, phhash: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateKeyIdentifierFromCSP(dwcertencodingtype: u32, pszpubkeyoid: super::super::Foundation::PSTR, ppubkeystruc: *const PUBLICKEYSTRUC, cbpubkeystruc: u32, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pbhash: *mut u8, pcbhash: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeMessage(dwmsgtypeflags: u32, pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbencodedblob: *const u8, cbencodedblob: u32, dwprevinnercontenttype: u32, pdwmsgtype: *mut u32, pdwinnercontenttype: *mut u32, pbdecoded: *mut u8, pcbdecoded: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeObject(dwcertencodingtype: u32, lpszstructtype: super::super::Foundation::PSTR, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pvstructinfo: *mut ::core::ffi::c_void, pcbstructinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeObjectEx(dwcertencodingtype: u32, lpszstructtype: super::super::Foundation::PSTR, pbencoded: *const u8, cbencoded: u32, dwflags: u32, pdecodepara: *const CRYPT_DECODE_PARA, pvstructinfo: *mut ::core::ffi::c_void, pcbstructinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecrypt(hkey: usize, hhash: usize, r#final: super::super::Foundation::BOOL, dwflags: u32, pbdata: *mut u8, pdwdatalen: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecryptAndVerifyMessageSignature(pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbencryptedblob: *const u8, cbencryptedblob: u32, pbdecrypted: *mut u8, pcbdecrypted: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecryptMessage(pdecryptpara: *const CRYPT_DECRYPT_MESSAGE_PARA, pbencryptedblob: *const u8, cbencryptedblob: u32, pbdecrypted: *mut u8, pcbdecrypted: *mut u32, ppxchgcert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDeriveKey(hprov: usize, algid: u32, hbasedata: usize, dwflags: u32, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDestroyHash(hhash: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDestroyKey(hkey: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDuplicateHash(hhash: usize, pdwreserved: *mut u32, dwflags: u32, phhash: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDuplicateKey(hkey: usize, pdwreserved: *mut u32, dwflags: u32, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncodeObject(dwcertencodingtype: u32, lpszstructtype: super::super::Foundation::PSTR, pvstructinfo: *const ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncodeObjectEx(dwcertencodingtype: CERT_QUERY_ENCODING_TYPE, lpszstructtype: super::super::Foundation::PSTR, pvstructinfo: *const ::core::ffi::c_void, dwflags: CRYPT_ENCODE_OBJECT_FLAGS, pencodepara: *const CRYPT_ENCODE_PARA, pvencoded: *mut ::core::ffi::c_void, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncrypt(hkey: usize, hhash: usize, r#final: super::super::Foundation::BOOL, dwflags: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwbuflen: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncryptMessage(pencryptpara: *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert: u32, rgprecipientcert: *const *const CERT_CONTEXT, pbtobeencrypted: *const u8, cbtobeencrypted: u32, pbencryptedblob: *mut u8, pcbencryptedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumKeyIdentifierProperties(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void, pvarg: *mut ::core::ffi::c_void, pfnenum: PFN_CRYPT_ENUM_KEYID_PROP) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumoidfunc: PFN_CRYPT_ENUM_OID_FUNC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumOIDInfo(dwgroupid: u32, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumoidinfo: PFN_CRYPT_ENUM_OID_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProviderTypesA(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, sztypename: super::super::Foundation::PSTR, pcbtypename: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProviderTypesW(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, sztypename: super::super::Foundation::PWSTR, pcbtypename: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProvidersA(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, szprovname: super::super::Foundation::PSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProvidersW(dwindex: u32, pdwreserved: *mut u32, dwflags: u32, pdwprovtype: *mut u32, szprovname: super::super::Foundation::PWSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportKey(hkey: usize, hexpkey: usize, dwblobtype: u32, dwflags: CRYPT_KEY_FLAGS, pbdata: *mut u8, pdwdatalen: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPKCS8(hcryptprov: usize, dwkeyspec: u32, pszprivatekeyobjid: super::super::Foundation::PSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pbprivatekeyblob: *mut u8, pcbprivatekeyblob: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfo(hcryptprovorncryptkey: usize, dwkeyspec: u32, dwcertencodingtype: u32, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfoEx(hcryptprovorncryptkey: usize, dwkeyspec: u32, dwcertencodingtype: u32, pszpublickeyobjid: super::super::Foundation::PSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfoFromBCryptKeyHandle(hbcryptkey: BCRYPT_KEY_HANDLE, dwcertencodingtype: u32, pszpublickeyobjid: super::super::Foundation::PSTR, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, pinfo: *mut CERT_PUBLIC_KEY_INFO, pcbinfo: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindCertificateKeyProvInfo(pcert: *const CERT_CONTEXT, dwflags: CRYPT_FIND_FLAGS, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindLocalizedName(pwszcryptname: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindOIDInfo(dwkeytype: u32, pvkey: *const ::core::ffi::c_void, dwgroupid: u32) -> *mut CRYPT_OID_INFO;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFormatObject(dwcertencodingtype: u32, dwformattype: u32, dwformatstrtype: u32, pformatstruct: *const ::core::ffi::c_void, lpszstructtype: super::super::Foundation::PSTR, pbencoded: *const u8, cbencoded: u32, pbformat: *mut ::core::ffi::c_void, pcbformat: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFreeOIDFunctionAddress(hfuncaddr: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGenKey(hprov: usize, algid: u32, dwflags: CRYPT_KEY_FLAGS, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGenRandom(hprov: usize, dwlen: u32, pbbuffer: *mut u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetAsyncParam(hasync: HCRYPTASYNC, pszparamoid: super::super::Foundation::PSTR, ppvparam: *mut *mut ::core::ffi::c_void, ppfnfree: *mut PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultOIDDllList(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pwszdlllist: super::super::Foundation::PWSTR, pcchdlllist: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultOIDFunctionAddress(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pwszdll: super::super::Foundation::PWSTR, dwflags: u32, ppvfuncaddr: *mut *mut ::core::ffi::c_void, phfuncaddr: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultProviderA(dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32, pszprovname: super::super::Foundation::PSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultProviderW(dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32, pszprovname: super::super::Foundation::PWSTR, pcbprovname: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetHashParam(hhash: usize, dwparam: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetKeyIdentifierProperty(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetKeyParam(hkey: usize, dwparam: CRYPT_KEY_PARAM_ID, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    pub fn CryptGetMessageCertificates(dwmsgandcertencodingtype: u32, hcryptprov: usize, dwflags: u32, pbsignedblob: *const u8, cbsignedblob: u32) -> *mut ::core::ffi::c_void;
    pub fn CryptGetMessageSignerCount(dwmsgencodingtype: u32, pbsignedblob: *const u8, cbsignedblob: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetOIDFunctionAddress(hfuncset: *const ::core::ffi::c_void, dwencodingtype: u32, pszoid: super::super::Foundation::PSTR, dwflags: u32, ppvfuncaddr: *mut *mut ::core::ffi::c_void, phfuncaddr: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetOIDFunctionValue(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR, pwszvaluename: super::super::Foundation::PWSTR, pdwvaluetype: *mut u32, pbvaluedata: *mut u8, pcbvaluedata: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetObjectUrl(pszurloid: super::super::Foundation::PSTR, pvpara: *const ::core::ffi::c_void, dwflags: CRYPT_GET_URL_FLAGS, purlarray: *mut CRYPT_URL_ARRAY, pcburlarray: *mut u32, purlinfo: *mut CRYPT_URL_INFO, pcburlinfo: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetProvParam(hprov: usize, dwparam: u32, pbdata: *mut u8, pdwdatalen: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetUserKey(hprov: usize, dwkeyspec: u32, phuserkey: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashCertificate(hcryptprov: usize, algid: u32, dwflags: u32, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashCertificate2(pwszcnghashalgid: super::super::Foundation::PWSTR, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashData(hhash: usize, pbdata: *const u8, dwdatalen: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashMessage(phashpara: *const CRYPT_HASH_MESSAGE_PARA, fdetachedhash: super::super::Foundation::BOOL, ctobehashed: u32, rgpbtobehashed: *const *const u8, rgcbtobehashed: *const u32, pbhashedblob: *mut u8, pcbhashedblob: *mut u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashPublicKeyInfo(hcryptprov: usize, algid: u32, dwflags: u32, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashSessionKey(hhash: usize, hkey: usize, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashToBeSigned(hcryptprov: usize, dwcertencodingtype: u32, pbencoded: *const u8, cbencoded: u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportKey(hprov: usize, pbdata: *const u8, dwdatalen: u32, hpubkey: usize, dwflags: CRYPT_KEY_FLAGS, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPKCS8(sprivatekeyandparams: CRYPT_PKCS8_IMPORT_PARAMS, dwflags: CRYPT_KEY_FLAGS, phcryptprov: *mut usize, pvauxinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfo(hcryptprov: usize, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfoEx(hcryptprov: usize, dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, aikeyalg: u32, dwflags: u32, pvauxinfo: *const ::core::ffi::c_void, phkey: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfoEx2(dwcertencodingtype: u32, pinfo: *const CERT_PUBLIC_KEY_INFO, dwflags: CRYPT_IMPORT_PUBLIC_KEY_FLAGS, pvauxinfo: *const ::core::ffi::c_void, phkey: *mut BCRYPT_KEY_HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInitOIDFunctionSet(pszfuncname: super::super::Foundation::PSTR, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallCancelRetrieval(pfncancel: PFN_CRYPT_CANCEL_RETRIEVAL, pvarg: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallDefaultContext(hcryptprov: usize, dwdefaulttype: CRYPT_DEFAULT_CONTEXT_TYPE, pvdefaultpara: *const ::core::ffi::c_void, dwflags: CRYPT_DEFAULT_CONTEXT_FLAGS, pvreserved: *mut ::core::ffi::c_void, phdefaultcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallOIDFunctionAddress(hmodule: super::super::Foundation::HINSTANCE, dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, cfuncentry: u32, rgfuncentry: *const CRYPT_OID_FUNC_ENTRY, dwflags: u32) -> super::super::Foundation::BOOL;
    pub fn CryptMemAlloc(cbsize: u32) -> *mut ::core::ffi::c_void;
    pub fn CryptMemFree(pv: *const ::core::ffi::c_void);
    pub fn CryptMemRealloc(pv: *const ::core::ffi::c_void, cbsize: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCalculateEncodedLength(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: u32, pvmsgencodeinfo: *const ::core::ffi::c_void, pszinnercontentobjid: super::super::Foundation::PSTR, cbdata: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgClose(hcryptmsg: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgControl(hcryptmsg: *const ::core::ffi::c_void, dwflags: u32, dwctrltype: u32, pvctrlpara: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCountersign(hcryptmsg: *const ::core::ffi::c_void, dwindex: u32, ccountersigners: u32, rgcountersigners: *const CMSG_SIGNER_ENCODE_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCountersignEncoded(dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, ccountersigners: u32, rgcountersigners: *const CMSG_SIGNER_ENCODE_INFO, pbcountersignature: *mut u8, pcbcountersignature: *mut u32) -> super::super::Foundation::BOOL;
    pub fn CryptMsgDuplicate(hcryptmsg: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgEncodeAndSignCTL(dwmsgencodingtype: u32, pctlinfo: *const CTL_INFO, psigninfo: *const CMSG_SIGNED_ENCODE_INFO, dwflags: u32, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgGetAndVerifySigner(hcryptmsg: *const ::core::ffi::c_void, csignerstore: u32, rghsignerstore: *const *const ::core::ffi::c_void, dwflags: u32, ppsigner: *mut *mut CERT_CONTEXT, pdwsignerindex: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgGetParam(hcryptmsg: *const ::core::ffi::c_void, dwparamtype: u32, dwindex: u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgOpenToDecode(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: u32, hcryptprov: usize, precipientinfo: *mut CERT_INFO, pstreaminfo: *const CMSG_STREAM_INFO) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgOpenToEncode(dwmsgencodingtype: u32, dwflags: u32, dwmsgtype: CRYPT_MSG_TYPE, pvmsgencodeinfo: *const ::core::ffi::c_void, pszinnercontentobjid: super::super::Foundation::PSTR, pstreaminfo: *const CMSG_STREAM_INFO) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgSignCTL(dwmsgencodingtype: u32, pbctlcontent: *const u8, cbctlcontent: u32, psigninfo: *const CMSG_SIGNED_ENCODE_INFO, dwflags: u32, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgUpdate(hcryptmsg: *const ::core::ffi::c_void, pbdata: *const u8, cbdata: u32, ffinal: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgVerifyCountersignatureEncoded(hcryptprov: usize, dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, pbsignerinfocountersignature: *const u8, cbsignerinfocountersignature: u32, pcicountersigner: *const CERT_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgVerifyCountersignatureEncodedEx(hcryptprov: usize, dwencodingtype: u32, pbsignerinfo: *const u8, cbsignerinfo: u32, pbsignerinfocountersignature: *const u8, cbsignerinfocountersignature: u32, dwsignertype: u32, pvsigner: *const ::core::ffi::c_void, dwflags: u32, pvextra: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptProtectData(pdatain: *const CRYPTOAPI_BLOB, szdatadescr: super::super::Foundation::PWSTR, poptionalentropy: *const CRYPTOAPI_BLOB, pvreserved: *mut ::core::ffi::c_void, ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT, dwflags: u32, pdataout: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptProtectMemory(pdatain: *mut ::core::ffi::c_void, cbdatain: u32, dwflags: u32) -> super::super::Foundation::BOOL;
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
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterDefaultOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, dwindex: u32, pwszdll: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR, pwszdll: super::super::Foundation::PWSTR, pszoverridefuncname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterOIDInfo(pinfo: *const CRYPT_OID_INFO, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptReleaseContext(hprov: usize, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveObjectByUrlA(pszurl: super::super::Foundation::PSTR, pszobjectoid: super::super::Foundation::PSTR, dwretrievalflags: u32, dwtimeout: u32, ppvobject: *mut *mut ::core::ffi::c_void, hasyncretrieve: HCRYPTASYNC, pcredentials: *const CRYPT_CREDENTIALS, pvverify: *const ::core::ffi::c_void, pauxinfo: *mut CRYPT_RETRIEVE_AUX_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveObjectByUrlW(pszurl: super::super::Foundation::PWSTR, pszobjectoid: super::super::Foundation::PSTR, dwretrievalflags: u32, dwtimeout: u32, ppvobject: *mut *mut ::core::ffi::c_void, hasyncretrieve: HCRYPTASYNC, pcredentials: *const CRYPT_CREDENTIALS, pvverify: *const ::core::ffi::c_void, pauxinfo: *mut CRYPT_RETRIEVE_AUX_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveTimeStamp(wszurl: super::super::Foundation::PWSTR, dwretrievalflags: u32, dwtimeout: u32, pszhashid: super::super::Foundation::PSTR, ppara: *const CRYPT_TIMESTAMP_PARA, pbdata: *const u8, cbdata: u32, pptscontext: *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner: *mut *mut CERT_CONTEXT, phstore: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetAsyncParam(hasync: HCRYPTASYNC, pszparamoid: super::super::Foundation::PSTR, pvparam: *const ::core::ffi::c_void, pfnfree: PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetHashParam(hhash: usize, dwparam: CRYPT_SET_HASH_PARAM, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetKeyIdentifierProperty(pkeyidentifier: *const CRYPTOAPI_BLOB, dwpropid: u32, dwflags: u32, pwszcomputername: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetKeyParam(hkey: usize, dwparam: CRYPT_KEY_PARAM_ID, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CryptSetOIDFunctionValue(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR, pwszvaluename: super::super::Foundation::PWSTR, dwvaluetype: super::super::System::Registry::REG_VALUE_TYPE, pbvaluedata: *const u8, cbvaluedata: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProvParam(hprov: usize, dwparam: CRYPT_SET_PROV_PARAM_ID, pbdata: *const u8, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderA(pszprovname: super::super::Foundation::PSTR, dwprovtype: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderExA(pszprovname: super::super::Foundation::PSTR, dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderExW(pszprovname: super::super::Foundation::PWSTR, dwprovtype: u32, pdwreserved: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderW(pszprovname: super::super::Foundation::PWSTR, dwprovtype: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignAndEncodeCertificate(hcryptprovorncryptkey: usize, dwkeyspec: CERT_KEY_SPEC, dwcertencodingtype: u32, lpszstructtype: super::super::Foundation::PSTR, pvstructinfo: *const ::core::ffi::c_void, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo: *const ::core::ffi::c_void, pbencoded: *mut u8, pcbencoded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignAndEncryptMessage(psignpara: *const CRYPT_SIGN_MESSAGE_PARA, pencryptpara: *const CRYPT_ENCRYPT_MESSAGE_PARA, crecipientcert: u32, rgprecipientcert: *const *const CERT_CONTEXT, pbtobesignedandencrypted: *const u8, cbtobesignedandencrypted: u32, pbsignedandencryptedblob: *mut u8, pcbsignedandencryptedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignCertificate(hcryptprovorncryptkey: usize, dwkeyspec: u32, dwcertencodingtype: u32, pbencodedtobesigned: *const u8, cbencodedtobesigned: u32, psignaturealgorithm: *const CRYPT_ALGORITHM_IDENTIFIER, pvhashauxinfo: *const ::core::ffi::c_void, pbsignature: *mut u8, pcbsignature: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignHashA(hhash: usize, dwkeyspec: u32, szdescription: super::super::Foundation::PSTR, dwflags: u32, pbsignature: *mut u8, pdwsiglen: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignHashW(hhash: usize, dwkeyspec: u32, szdescription: super::super::Foundation::PWSTR, dwflags: u32, pbsignature: *mut u8, pdwsiglen: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignMessage(psignpara: *const CRYPT_SIGN_MESSAGE_PARA, fdetachedsignature: super::super::Foundation::BOOL, ctobesigned: u32, rgpbtobesigned: *const *const u8, rgcbtobesigned: *const u32, pbsignedblob: *mut u8, pcbsignedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignMessageWithKey(psignpara: *const CRYPT_KEY_SIGN_MESSAGE_PARA, pbtobesigned: *const u8, cbtobesigned: u32, pbsignedblob: *mut u8, pcbsignedblob: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptStringToBinaryA(pszstring: super::super::Foundation::PSTR, cchstring: u32, dwflags: CRYPT_STRING, pbbinary: *mut u8, pcbbinary: *mut u32, pdwskip: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptStringToBinaryW(pszstring: super::super::Foundation::PWSTR, cchstring: u32, dwflags: CRYPT_STRING, pbbinary: *mut u8, pcbbinary: *mut u32, pdwskip: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUninstallCancelRetrieval(dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUninstallDefaultContext(hdefaultcontext: *const ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnprotectData(pdatain: *const CRYPTOAPI_BLOB, ppszdatadescr: *mut super::super::Foundation::PWSTR, poptionalentropy: *const CRYPTOAPI_BLOB, pvreserved: *mut ::core::ffi::c_void, ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT, dwflags: u32, pdataout: *mut CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnprotectMemory(pdatain: *mut ::core::ffi::c_void, cbdatain: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterDefaultOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pwszdll: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterOIDFunction(dwencodingtype: u32, pszfuncname: super::super::Foundation::PSTR, pszoid: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterOIDInfo(pinfo: *const CRYPT_OID_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUpdateProtectedState(poldsid: super::super::Foundation::PSID, pwszoldpassword: super::super::Foundation::PWSTR, dwflags: u32, pdwsuccesscount: *mut u32, pdwfailurecount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyCertificateSignature(hcryptprov: usize, dwcertencodingtype: u32, pbencoded: *const u8, cbencoded: u32, ppublickey: *const CERT_PUBLIC_KEY_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyCertificateSignatureEx(hcryptprov: usize, dwcertencodingtype: u32, dwsubjecttype: u32, pvsubject: *const ::core::ffi::c_void, dwissuertype: u32, pvissuer: *const ::core::ffi::c_void, dwflags: CRYPT_VERIFY_CERT_FLAGS, pvextra: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyDetachedMessageHash(phashpara: *const CRYPT_HASH_MESSAGE_PARA, pbdetachedhashblob: *const u8, cbdetachedhashblob: u32, ctobehashed: u32, rgpbtobehashed: *const *const u8, rgcbtobehashed: *const u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyDetachedMessageSignature(pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbdetachedsignblob: *const u8, cbdetachedsignblob: u32, ctobesigned: u32, rgpbtobesigned: *const *const u8, rgcbtobesigned: *const u32, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageHash(phashpara: *const CRYPT_HASH_MESSAGE_PARA, pbhashedblob: *const u8, cbhashedblob: u32, pbtobehashed: *mut u8, pcbtobehashed: *mut u32, pbcomputedhash: *mut u8, pcbcomputedhash: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageSignature(pverifypara: *const CRYPT_VERIFY_MESSAGE_PARA, dwsignerindex: u32, pbsignedblob: *const u8, cbsignedblob: u32, pbdecoded: *mut u8, pcbdecoded: *mut u32, ppsignercert: *mut *mut CERT_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageSignatureWithKey(pverifypara: *const CRYPT_KEY_VERIFY_MESSAGE_PARA, ppublickeyinfo: *const CERT_PUBLIC_KEY_INFO, pbsignedblob: *const u8, cbsignedblob: u32, pbdecoded: *mut u8, pcbdecoded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifySignatureA(hhash: usize, pbsignature: *const u8, dwsiglen: u32, hpubkey: usize, szdescription: super::super::Foundation::PSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifySignatureW(hhash: usize, pbsignature: *const u8, dwsiglen: u32, hpubkey: usize, szdescription: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyTimeStampSignature(pbtscontentinfo: *const u8, cbtscontentinfo: u32, pbdata: *const u8, cbdata: u32, hadditionalstore: *const ::core::ffi::c_void, pptscontext: *mut *mut CRYPT_TIMESTAMP_CONTEXT, pptssigner: *mut *mut CERT_CONTEXT, phstore: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlAddObject(hsignatureorobject: *const ::core::ffi::c_void, dwflags: u32, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, ppobject: *mut *mut CRYPT_XML_OBJECT) -> ::windows_sys::core::HRESULT;
    pub fn CryptXmlClose(hcryptxml: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlCreateReference(hcryptxml: *const ::core::ffi::c_void, dwflags: u32, wszid: super::super::Foundation::PWSTR, wszuri: super::super::Foundation::PWSTR, wsztype: super::super::Foundation::PWSTR, pdigestmethod: *const CRYPT_XML_ALGORITHM, ctransform: u32, rgtransform: *const CRYPT_XML_ALGORITHM, phreference: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CryptXmlDigestReference(hreference: *const ::core::ffi::c_void, dwflags: u32, pdataproviderin: *const CRYPT_XML_DATA_PROVIDER) -> ::windows_sys::core::HRESULT;
    pub fn CryptXmlEncode(hcryptxml: *const ::core::ffi::c_void, dwcharset: CRYPT_XML_CHARSET, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pvcallbackstate: *mut ::core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlEnumAlgorithmInfo(dwgroupid: u32, dwflags: u32, pvarg: *mut ::core::ffi::c_void, pfnenumalginfo: PFN_CRYPT_XML_ENUM_ALG_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlFindAlgorithmInfo(dwfindbytype: u32, pvfindby: *const ::core::ffi::c_void, dwgroupid: u32, dwflags: u32) -> *mut CRYPT_XML_ALGORITHM_INFO;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetAlgorithmInfo(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, dwflags: CRYPT_XML_FLAGS, ppalginfo: *mut *mut CRYPT_XML_ALGORITHM_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetDocContext(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_DOC_CTXT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetReference(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_REFERENCE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetSignature(hcryptxml: *const ::core::ffi::c_void, ppstruct: *mut *mut CRYPT_XML_SIGNATURE) -> ::windows_sys::core::HRESULT;
    pub fn CryptXmlGetStatus(hcryptxml: *const ::core::ffi::c_void, pstatus: *mut CRYPT_XML_STATUS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetTransforms(ppconfig: *mut *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlImportPublicKey(dwflags: CRYPT_XML_FLAGS, pkeyvalue: *const CRYPT_XML_KEY_VALUE, phkey: *mut BCRYPT_KEY_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlOpenToDecode(pconfig: *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags: CRYPT_XML_FLAGS, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, phcryptxml: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlOpenToEncode(pconfig: *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags: CRYPT_XML_FLAGS, wszid: super::super::Foundation::PWSTR, rgproperty: *const CRYPT_XML_PROPERTY, cproperty: u32, pencoded: *const CRYPT_XML_BLOB, phsignature: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CryptXmlSetHMACSecret(hsignature: *const ::core::ffi::c_void, pbsecret: *const u8, cbsecret: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlSign(hsignature: *const ::core::ffi::c_void, hkey: usize, dwkeyspec: CERT_KEY_SPEC, dwflags: CRYPT_XML_FLAGS, dwkeyinfospec: CRYPT_XML_KEYINFO_SPEC, pvkeyinfospec: *const ::core::ffi::c_void, psignaturemethod: *const CRYPT_XML_ALGORITHM, pcanonicalization: *const CRYPT_XML_ALGORITHM) -> ::windows_sys::core::HRESULT;
    pub fn CryptXmlVerifySignature(hsignature: *const ::core::ffi::c_void, hkey: BCRYPT_KEY_HANDLE, dwflags: CRYPT_XML_FLAGS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Decrypt(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, foaep: super::super::Foundation::BOOL, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Encrypt(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, foaep: super::super::Foundation::BOOL, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindCertsByIssuer(pcertchains: *mut CERT_CHAIN, pcbcertchains: *mut u32, pccertchains: *mut u32, pbencodedissuername: *const u8, cbencodedissuername: u32, pwszpurpose: super::super::Foundation::PWSTR, dwkeyspec: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeToken(pallocmemory: *const GENERIC_XML_TOKEN) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateDerivedKey(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cblabel: u32, plabel: *const u8, cbnonce: u32, pnonce: *const u8, derivedkeylength: u32, offset: u32, algid: super::super::Foundation::PWSTR, pcbkey: *mut u32, ppkey: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn GetBrowserToken(dwparamtype: u32, pparam: *const ::core::ffi::c_void, pcbtoken: *mut u32, pptoken: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn GetCryptoTransform(hsymmetriccrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, mode: u32, padding: PaddingMode, feedbacksize: u32, direction: Direction, cbiv: u32, piv: *const u8, pphtransform: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn GetKeyedHash(hsymmetriccrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, pphhash: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetToken(cpolicychain: u32, ppolicychain: *const POLICY_ELEMENT, securitytoken: *mut *mut GENERIC_XML_TOKEN, phprooftokencrypto: *mut *mut INFORMATIONCARD_CRYPTO_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn HashCore(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8) -> ::windows_sys::core::HRESULT;
    pub fn HashFinal(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportInformationCard(filename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn ManageCardSpace() -> ::windows_sys::core::HRESULT;
    pub fn NCryptCloseProtectionDescriptor(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE) -> i32;
    pub fn NCryptCreateClaim(hsubjectkey: usize, hauthoritykey: usize, dwclaimtype: u32, pparameterlist: *const BCryptBufferDesc, pbclaimblob: *mut u8, cbclaimblob: u32, pcbresult: *mut u32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptCreatePersistedKey(hprovider: usize, phkey: *mut usize, pszalgid: super::super::Foundation::PWSTR, pszkeyname: super::super::Foundation::PWSTR, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: NCRYPT_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptCreateProtectionDescriptor(pwszdescriptorstring: super::super::Foundation::PWSTR, dwflags: u32, phdescriptor: *mut super::NCRYPT_DESCRIPTOR_HANDLE) -> i32;
    pub fn NCryptDecrypt(hkey: usize, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> i32;
    pub fn NCryptDeleteKey(hkey: usize, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptDeriveKey(hsharedsecret: usize, pwszkdf: super::super::Foundation::PWSTR, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> i32;
    pub fn NCryptEncrypt(hkey: usize, pbinput: *const u8, cbinput: u32, ppaddinginfo: *const ::core::ffi::c_void, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumAlgorithms(hprovider: usize, dwalgoperations: NCRYPT_OPERATION, pdwalgcount: *mut u32, ppalglist: *mut *mut NCryptAlgorithmName, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumKeys(hprovider: usize, pszscope: super::super::Foundation::PWSTR, ppkeyname: *mut *mut NCryptKeyName, ppenumstate: *mut *mut ::core::ffi::c_void, dwflags: NCRYPT_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumStorageProviders(pdwprovidercount: *mut u32, ppproviderlist: *mut *mut NCryptProviderName, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptExportKey(hkey: usize, hexportkey: usize, pszblobtype: super::super::Foundation::PWSTR, pparameterlist: *const BCryptBufferDesc, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> i32;
    pub fn NCryptFinalizeKey(hkey: usize, dwflags: NCRYPT_FLAGS) -> i32;
    pub fn NCryptFreeBuffer(pvinput: *mut ::core::ffi::c_void) -> i32;
    pub fn NCryptFreeObject(hobject: usize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptGetProperty(hobject: usize, pszproperty: super::super::Foundation::PWSTR, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32, dwflags: super::OBJECT_SECURITY_INFORMATION) -> i32;
    pub fn NCryptGetProtectionDescriptorInfo(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, pmempara: *const NCRYPT_ALLOC_PARA, dwinfotype: u32, ppvinfo: *mut *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptImportKey(hprovider: usize, himportkey: usize, pszblobtype: super::super::Foundation::PWSTR, pparameterlist: *const BCryptBufferDesc, phkey: *mut usize, pbdata: *const u8, cbdata: u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptIsAlgSupported(hprovider: usize, pszalgid: super::super::Foundation::PWSTR, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptIsKeyHandle(hkey: usize) -> super::super::Foundation::BOOL;
    pub fn NCryptKeyDerivation(hkey: usize, pparameterlist: *const BCryptBufferDesc, pbderivedkey: *mut u8, cbderivedkey: u32, pcbresult: *mut u32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptNotifyChangeKey(hprovider: usize, phevent: *mut super::super::Foundation::HANDLE, dwflags: NCRYPT_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptOpenKey(hprovider: usize, phkey: *mut usize, pszkeyname: super::super::Foundation::PWSTR, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: NCRYPT_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptOpenStorageProvider(phprovider: *mut usize, pszprovidername: super::super::Foundation::PWSTR, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptProtectSecret(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: u32, pbdata: *const u8, cbdata: u32, pmempara: *const NCRYPT_ALLOC_PARA, hwnd: super::super::Foundation::HWND, ppbprotectedblob: *mut *mut u8, pcbprotectedblob: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptQueryProtectionDescriptorName(pwszname: super::super::Foundation::PWSTR, pwszdescriptorstring: super::super::Foundation::PWSTR, pcdescriptorstring: *mut usize, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptRegisterProtectionDescriptorName(pwszname: super::super::Foundation::PWSTR, pwszdescriptorstring: super::super::Foundation::PWSTR, dwflags: u32) -> i32;
    pub fn NCryptSecretAgreement(hprivkey: usize, hpubkey: usize, phagreedsecret: *mut usize, dwflags: NCRYPT_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptSetProperty(hobject: usize, pszproperty: super::super::Foundation::PWSTR, pbinput: *const u8, cbinput: u32, dwflags: NCRYPT_FLAGS) -> i32;
    pub fn NCryptSignHash(hkey: usize, ppaddinginfo: *const ::core::ffi::c_void, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *mut u8, cbsignature: u32, pcbresult: *mut u32, dwflags: NCRYPT_FLAGS) -> i32;
    pub fn NCryptStreamClose(hstream: super::NCRYPT_STREAM_HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToProtect(hdescriptor: super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: u32, hwnd: super::super::Foundation::HWND, pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToUnprotect(pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO, dwflags: u32, hwnd: super::super::Foundation::HWND, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToUnprotectEx(pstreaminfo: *const NCRYPT_PROTECT_STREAM_INFO_EX, dwflags: u32, hwnd: super::super::Foundation::HWND, phstream: *mut super::NCRYPT_STREAM_HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamUpdate(hstream: super::NCRYPT_STREAM_HANDLE, pbdata: *const u8, cbdata: usize, ffinal: super::super::Foundation::BOOL) -> i32;
    pub fn NCryptTranslateHandle(phprovider: *mut usize, phkey: *mut usize, hlegacyprov: usize, hlegacykey: usize, dwlegacykeyspec: CERT_KEY_SPEC, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptUnprotectSecret(phdescriptor: *mut super::NCRYPT_DESCRIPTOR_HANDLE, dwflags: NCRYPT_FLAGS, pbprotectedblob: *const u8, cbprotectedblob: u32, pmempara: *const NCRYPT_ALLOC_PARA, hwnd: super::super::Foundation::HWND, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> i32;
    pub fn NCryptVerifyClaim(hsubjectkey: usize, hauthoritykey: usize, dwclaimtype: u32, pparameterlist: *const BCryptBufferDesc, pbclaimblob: *const u8, cbclaimblob: u32, poutput: *mut BCryptBufferDesc, dwflags: u32) -> i32;
    pub fn NCryptVerifySignature(hkey: usize, ppaddinginfo: *const ::core::ffi::c_void, pbhashvalue: *const u8, cbhashvalue: u32, pbsignature: *const u8, cbsignature: u32, dwflags: NCRYPT_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXExportCertStore(hstore: *const ::core::ffi::c_void, ppfx: *mut CRYPTOAPI_BLOB, szpassword: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXExportCertStoreEx(hstore: *const ::core::ffi::c_void, ppfx: *mut CRYPTOAPI_BLOB, szpassword: super::super::Foundation::PWSTR, pvpara: *const ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXImportCertStore(ppfx: *const CRYPTOAPI_BLOB, szpassword: super::super::Foundation::PWSTR, dwflags: CRYPT_KEY_FLAGS) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXIsPFXBlob(ppfx: *const CRYPTOAPI_BLOB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXVerifyPassword(ppfx: *const CRYPTOAPI_BLOB, szpassword: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SignHash(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbhash: u32, phash: *const u8, hashalgoid: super::super::Foundation::PWSTR, pcbsig: *mut u32, ppsig: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn TransformBlock(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn TransformFinalBlock(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbindata: u32, pindata: *const u8, pcboutdata: *mut u32, ppoutdata: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyHash(hcrypto: *const INFORMATIONCARD_CRYPTO_HANDLE, cbhash: u32, phash: *const u8, hashalgoid: super::super::Foundation::PWSTR, cbsig: u32, psig: *const u8, pfverified: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
}
pub const ALG_CLASS_ALL: u32 = 57344u32;
pub const ALG_CLASS_ANY: u32 = 0u32;
pub const ALG_CLASS_DATA_ENCRYPT: u32 = 24576u32;
pub const ALG_CLASS_HASH: u32 = 32768u32;
pub const ALG_CLASS_KEY_EXCHANGE: u32 = 40960u32;
pub const ALG_CLASS_MSG_ENCRYPT: u32 = 16384u32;
pub const ALG_CLASS_SIGNATURE: u32 = 8192u32;
pub const ALG_SID_3DES: u32 = 3u32;
pub const ALG_SID_3DES_112: u32 = 9u32;
pub const ALG_SID_AES: u32 = 17u32;
pub const ALG_SID_AES_128: u32 = 14u32;
pub const ALG_SID_AES_192: u32 = 15u32;
pub const ALG_SID_AES_256: u32 = 16u32;
pub const ALG_SID_AGREED_KEY_ANY: u32 = 3u32;
pub const ALG_SID_ANY: u32 = 0u32;
pub const ALG_SID_CAST: u32 = 6u32;
pub const ALG_SID_CYLINK_MEK: u32 = 12u32;
pub const ALG_SID_DES: u32 = 1u32;
pub const ALG_SID_DESX: u32 = 4u32;
pub const ALG_SID_DH_EPHEM: u32 = 2u32;
pub const ALG_SID_DH_SANDF: u32 = 1u32;
pub const ALG_SID_DSS_ANY: u32 = 0u32;
pub const ALG_SID_DSS_DMS: u32 = 2u32;
pub const ALG_SID_DSS_PKCS: u32 = 1u32;
pub const ALG_SID_ECDH: u32 = 5u32;
pub const ALG_SID_ECDH_EPHEM: u32 = 6u32;
pub const ALG_SID_ECDSA: u32 = 3u32;
pub const ALG_SID_ECMQV: u32 = 1u32;
pub const ALG_SID_EXAMPLE: u32 = 80u32;
pub const ALG_SID_HASH_REPLACE_OWF: u32 = 11u32;
pub const ALG_SID_HMAC: u32 = 9u32;
pub const ALG_SID_IDEA: u32 = 5u32;
pub const ALG_SID_KEA: u32 = 4u32;
pub const ALG_SID_MAC: u32 = 5u32;
pub const ALG_SID_MD2: u32 = 1u32;
pub const ALG_SID_MD4: u32 = 2u32;
pub const ALG_SID_MD5: u32 = 3u32;
pub const ALG_SID_PCT1_MASTER: u32 = 4u32;
pub const ALG_SID_RC2: u32 = 2u32;
pub const ALG_SID_RC4: u32 = 1u32;
pub const ALG_SID_RC5: u32 = 13u32;
pub const ALG_SID_RIPEMD: u32 = 6u32;
pub const ALG_SID_RIPEMD160: u32 = 7u32;
pub const ALG_SID_RSA_ANY: u32 = 0u32;
pub const ALG_SID_RSA_ENTRUST: u32 = 3u32;
pub const ALG_SID_RSA_MSATWORK: u32 = 2u32;
pub const ALG_SID_RSA_PGP: u32 = 4u32;
pub const ALG_SID_RSA_PKCS: u32 = 1u32;
pub const ALG_SID_SAFERSK128: u32 = 8u32;
pub const ALG_SID_SAFERSK64: u32 = 7u32;
pub const ALG_SID_SCHANNEL_ENC_KEY: u32 = 7u32;
pub const ALG_SID_SCHANNEL_MAC_KEY: u32 = 3u32;
pub const ALG_SID_SCHANNEL_MASTER_HASH: u32 = 2u32;
pub const ALG_SID_SEAL: u32 = 2u32;
pub const ALG_SID_SHA: u32 = 4u32;
pub const ALG_SID_SHA1: u32 = 4u32;
pub const ALG_SID_SHA_256: u32 = 12u32;
pub const ALG_SID_SHA_384: u32 = 13u32;
pub const ALG_SID_SHA_512: u32 = 14u32;
pub const ALG_SID_SKIPJACK: u32 = 10u32;
pub const ALG_SID_SSL2_MASTER: u32 = 5u32;
pub const ALG_SID_SSL3SHAMD5: u32 = 8u32;
pub const ALG_SID_SSL3_MASTER: u32 = 1u32;
pub const ALG_SID_TEK: u32 = 11u32;
pub const ALG_SID_THIRDPARTY_ANY: u32 = 0u32;
pub const ALG_SID_TLS1PRF: u32 = 10u32;
pub const ALG_SID_TLS1_MASTER: u32 = 6u32;
pub const ALG_TYPE_ANY: u32 = 0u32;
pub const ALG_TYPE_BLOCK: u32 = 1536u32;
pub const ALG_TYPE_DH: u32 = 2560u32;
pub const ALG_TYPE_DSS: u32 = 512u32;
pub const ALG_TYPE_ECDH: u32 = 3584u32;
pub const ALG_TYPE_RSA: u32 = 1024u32;
pub const ALG_TYPE_SECURECHANNEL: u32 = 3072u32;
pub const ALG_TYPE_STREAM: u32 = 2048u32;
pub const ALG_TYPE_THIRDPARTY: u32 = 4096u32;
pub const AUDIT_CARD_DELETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074070017i32 as _);
pub const AUDIT_CARD_IMPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074070018i32 as _);
pub const AUDIT_CARD_WRITTEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074070016i32 as _);
pub const AUDIT_SERVICE_IDLE_STOP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074070022i32 as _);
pub const AUDIT_STORE_DELETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074070021i32 as _);
pub const AUDIT_STORE_EXPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074070020i32 as _);
pub const AUDIT_STORE_IMPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1074070019i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA(i32);
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_CA_FLAG: u32 = 2147483648u32;
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG: u32 = 1073741824u32;
pub const BCRYPTBUFFER_VERSION: u32 = 0u32;
pub const BCRYPT_3DES_112_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(369u32 as _);
pub const BCRYPT_3DES_112_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(401u32 as _);
pub const BCRYPT_3DES_112_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(385u32 as _);
pub const BCRYPT_3DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(321u32 as _);
pub const BCRYPT_3DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(353u32 as _);
pub const BCRYPT_3DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(337u32 as _);
pub const BCRYPT_AES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(417u32 as _);
pub const BCRYPT_AES_CCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(465u32 as _);
pub const BCRYPT_AES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(449u32 as _);
pub const BCRYPT_AES_CMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(257u32 as _);
pub const BCRYPT_AES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(433u32 as _);
pub const BCRYPT_AES_GCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(481u32 as _);
pub const BCRYPT_AES_GMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(273u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub struct BCRYPT_ALGORITHM_IDENTIFIER(i32);
pub struct BCRYPT_ALG_HANDLE(i32);
pub struct BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO(i32);
pub const BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO_VERSION: u32 = 1u32;
pub const BCRYPT_AUTH_MODE_CHAIN_CALLS_FLAG: u32 = 1u32;
pub const BCRYPT_AUTH_MODE_IN_PROGRESS_FLAG: u32 = 2u32;
pub const BCRYPT_BLOCK_PADDING: u32 = 1u32;
pub const BCRYPT_BUFFERS_LOCKED_FLAG: u32 = 64u32;
pub const BCRYPT_CAPI_AES_FLAG: u32 = 16u32;
pub const BCRYPT_CAPI_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(801u32 as _);
pub const BCRYPT_CHACHA20_POLY1305_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(929u32 as _);
pub const BCRYPT_DESX_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(545u32 as _);
pub const BCRYPT_DESX_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(577u32 as _);
pub const BCRYPT_DESX_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(561u32 as _);
pub const BCRYPT_DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(497u32 as _);
pub const BCRYPT_DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(529u32 as _);
pub const BCRYPT_DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(513u32 as _);
pub const BCRYPT_DH_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(641u32 as _);
pub struct BCRYPT_DH_KEY_BLOB(i32);
pub struct BCRYPT_DH_KEY_BLOB_MAGIC(i32);
pub const BCRYPT_DH_PARAMETERS_MAGIC: u32 = 1297107012u32;
pub struct BCRYPT_DH_PARAMETER_HEADER(i32);
pub const BCRYPT_DSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(721u32 as _);
pub struct BCRYPT_DSA_KEY_BLOB(i32);
pub struct BCRYPT_DSA_KEY_BLOB_V2(i32);
pub struct BCRYPT_DSA_MAGIC(i32);
pub const BCRYPT_DSA_PARAMETERS_MAGIC: u32 = 1297109828u32;
pub const BCRYPT_DSA_PARAMETERS_MAGIC_V2: u32 = 843927620u32;
pub struct BCRYPT_DSA_PARAMETER_HEADER(i32);
pub struct BCRYPT_DSA_PARAMETER_HEADER_V2(i32);
pub const BCRYPT_DSA_PRIVATE_MAGIC_V2: u32 = 844517444u32;
pub const BCRYPT_DSA_PUBLIC_MAGIC_V2: u32 = 843206724u32;
pub struct BCRYPT_ECCFULLKEY_BLOB(i32);
pub struct BCRYPT_ECCKEY_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BCRYPT_ECC_CURVE_NAMES(i32);
pub const BCRYPT_ECC_FULLKEY_BLOB_V1: u32 = 1u32;
pub const BCRYPT_ECC_PARAMETERS_MAGIC: u32 = 1346585413u32;
pub const BCRYPT_ECDH_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(657u32 as _);
pub const BCRYPT_ECDH_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(673u32 as _);
pub const BCRYPT_ECDH_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(689u32 as _);
pub const BCRYPT_ECDH_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(705u32 as _);
pub const BCRYPT_ECDH_PRIVATE_GENERIC_MAGIC: u32 = 1447772997u32;
pub const BCRYPT_ECDH_PRIVATE_P256_MAGIC: u32 = 843793221u32;
pub const BCRYPT_ECDH_PRIVATE_P384_MAGIC: u32 = 877347653u32;
pub const BCRYPT_ECDH_PRIVATE_P521_MAGIC: u32 = 910902085u32;
pub const BCRYPT_ECDH_PUBLIC_GENERIC_MAGIC: u32 = 1347109701u32;
pub const BCRYPT_ECDH_PUBLIC_P256_MAGIC: u32 = 827016005u32;
pub const BCRYPT_ECDH_PUBLIC_P384_MAGIC: u32 = 860570437u32;
pub const BCRYPT_ECDH_PUBLIC_P521_MAGIC: u32 = 894124869u32;
pub const BCRYPT_ECDSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(241u32 as _);
pub const BCRYPT_ECDSA_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(737u32 as _);
pub const BCRYPT_ECDSA_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(753u32 as _);
pub const BCRYPT_ECDSA_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(769u32 as _);
pub const BCRYPT_ECDSA_PRIVATE_GENERIC_MAGIC: u32 = 1447314245u32;
pub const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: u32 = 844317509u32;
pub const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: u32 = 877871941u32;
pub const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: u32 = 911426373u32;
pub const BCRYPT_ECDSA_PUBLIC_GENERIC_MAGIC: u32 = 1346650949u32;
pub const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: u32 = 827540293u32;
pub const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: u32 = 861094725u32;
pub const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: u32 = 894649157u32;
pub const BCRYPT_ENABLE_INCOMPATIBLE_FIPS_CHECKS: u32 = 256u32;
pub const BCRYPT_EXTENDED_KEYSIZE: u32 = 128u32;
pub const BCRYPT_GENERATE_IV: u32 = 32u32;
pub const BCRYPT_HASH_INTERFACE_MAJORVERSION_2: u32 = 2u32;
pub struct BCRYPT_HASH_OPERATION_TYPE(i32);
pub const BCRYPT_HKDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(913u32 as _);
pub const BCRYPT_HMAC_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(289u32 as _);
pub const BCRYPT_HMAC_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(305u32 as _);
pub const BCRYPT_HMAC_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(145u32 as _);
pub const BCRYPT_HMAC_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(161u32 as _);
pub const BCRYPT_HMAC_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(177u32 as _);
pub const BCRYPT_HMAC_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(193u32 as _);
pub const BCRYPT_HMAC_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(209u32 as _);
pub struct BCRYPT_INTERFACE(i32);
pub struct BCRYPT_INTERFACE_VERSION(i32);
pub struct BCRYPT_KEY_BLOB(i32);
pub struct BCRYPT_KEY_DATA_BLOB_HEADER(i32);
pub const BCRYPT_KEY_DATA_BLOB_MAGIC: u32 = 1296188491u32;
pub const BCRYPT_KEY_DATA_BLOB_VERSION1: u32 = 1u32;
pub const BCRYPT_KEY_DERIVATION_INTERFACE: u32 = 7u32;
pub const BCRYPT_KEY_DERIVATION_OPERATION: u32 = 64u32;
pub struct BCRYPT_KEY_HANDLE(i32);
pub struct BCRYPT_KEY_LENGTHS_STRUCT(i32);
pub const BCRYPT_KEY_VALIDATION_RANGE: u32 = 16u32;
pub const BCRYPT_KEY_VALIDATION_RANGE_AND_ORDER: u32 = 24u32;
pub const BCRYPT_KEY_VALIDATION_REGENERATE: u32 = 32u32;
pub const BCRYPT_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(1u32 as _);
pub const BCRYPT_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(17u32 as _);
pub const BCRYPT_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(33u32 as _);
pub const BCRYPT_MULTI_FLAG: u32 = 64u32;
pub struct BCRYPT_MULTI_HASH_OPERATION(i32);
pub struct BCRYPT_MULTI_OBJECT_LENGTH_STRUCT(i32);
pub struct BCRYPT_MULTI_OPERATION_TYPE(i32);
pub const BCRYPT_NO_KEY_VALIDATION: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct BCRYPT_OAEP_PADDING_INFO(i32);
pub const BCRYPT_OBJECT_ALIGNMENT: u32 = 16u32;
pub struct BCRYPT_OID(i32);
pub struct BCRYPT_OID_LIST(i32);
pub struct BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(i32);
pub struct BCRYPT_OPERATION(i32);
pub const BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID: u32 = 16u32;
pub const BCRYPT_PBKDF2_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(817u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub struct BCRYPT_PKCS1_PADDING_INFO(i32);
pub const BCRYPT_PRIVATE_KEY_FLAG: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct BCRYPT_PROVIDER_NAME(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BCRYPT_PSS_PADDING_INFO(i32);
pub const BCRYPT_PUBLIC_KEY_FLAG: u32 = 1u32;
pub struct BCRYPT_QUERY_PROVIDER_MODE(i32);
pub const BCRYPT_RC2_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(593u32 as _);
pub const BCRYPT_RC2_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(625u32 as _);
pub const BCRYPT_RC2_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(609u32 as _);
pub const BCRYPT_RC4_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(113u32 as _);
pub struct BCRYPT_RESOLVE_PROVIDERS_FLAGS(i32);
pub const BCRYPT_RNG_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(129u32 as _);
pub const BCRYPT_RNG_USE_ENTROPY_IN_BUFFER: u32 = 1u32;
pub struct BCRYPT_RSAKEY_BLOB(i32);
pub struct BCRYPT_RSAKEY_BLOB_MAGIC(i32);
pub const BCRYPT_RSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(225u32 as _);
pub const BCRYPT_RSA_SIGN_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(785u32 as _);
pub const BCRYPT_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(49u32 as _);
pub const BCRYPT_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(65u32 as _);
pub const BCRYPT_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(81u32 as _);
pub const BCRYPT_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(97u32 as _);
pub const BCRYPT_SP800108_CTR_HMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(833u32 as _);
pub const BCRYPT_SP80056A_CONCAT_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(849u32 as _);
pub const BCRYPT_SUPPORTED_PAD_OAEP: u32 = 8u32;
pub const BCRYPT_SUPPORTED_PAD_PKCS1_ENC: u32 = 2u32;
pub const BCRYPT_SUPPORTED_PAD_PKCS1_SIG: u32 = 4u32;
pub const BCRYPT_SUPPORTED_PAD_PSS: u32 = 16u32;
pub const BCRYPT_SUPPORTED_PAD_ROUTER: u32 = 1u32;
pub struct BCRYPT_TABLE(i32);
pub const BCRYPT_TLS1_1_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(865u32 as _);
pub const BCRYPT_TLS1_2_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(881u32 as _);
pub const BCRYPT_TLS_CBC_HMAC_VERIFY_FLAG: u32 = 4u32;
pub const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 2u32;
pub const BCRYPT_XTS_AES_ALG_HANDLE: BCRYPT_ALG_HANDLE = BCRYPT_ALG_HANDLE(897u32 as _);
pub struct BCryptBuffer(i32);
pub struct BCryptBufferDesc(i32);
pub const CALG_OID_INFO_CNG_ONLY: u32 = 4294967295u32;
pub const CALG_OID_INFO_PARAMETERS: u32 = 4294967294u32;
pub struct CASetupProperty(i32);
pub struct CCertSrvSetup(i32);
pub struct CCertSrvSetupKeyInformation(i32);
pub struct CCertificateEnrollmentPolicyServerSetup(i32);
pub struct CCertificateEnrollmentServerSetup(i32);
pub struct CEPSetupProperty(i32);
pub struct CERTIFICATE_CHAIN_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_ACCESS_DESCRIPTION(i32);
pub const CERT_ACCESS_STATE_GP_SYSTEM_STORE_FLAG: u32 = 8u32;
pub const CERT_ACCESS_STATE_LM_SYSTEM_STORE_FLAG: u32 = 4u32;
pub const CERT_ACCESS_STATE_PROP_ID: u32 = 14u32;
pub const CERT_ACCESS_STATE_SHARED_USER_FLAG: u32 = 16u32;
pub const CERT_ACCESS_STATE_SYSTEM_STORE_FLAG: u32 = 2u32;
pub const CERT_ACCESS_STATE_WRITE_PERSIST_FLAG: u32 = 1u32;
pub const CERT_AIA_URL_RETRIEVED_PROP_ID: u32 = 67u32;
pub const CERT_ALT_NAME_EDI_PARTY_NAME: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_ALT_NAME_ENTRY(i32);
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_MASK: u32 = 255u32;
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_SHIFT: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_ALT_NAME_INFO(i32);
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_MASK: u32 = 65535u32;
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_SHIFT: u32 = 0u32;
pub const CERT_ALT_NAME_X400_ADDRESS: u32 = 4u32;
pub const CERT_ARCHIVED_KEY_HASH_PROP_ID: u32 = 65u32;
pub const CERT_ARCHIVED_PROP_ID: u32 = 19u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_AUTHORITY_INFO_ACCESS(i32);
pub const CERT_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 68u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_AUTHORITY_KEY_ID2_INFO(i32);
pub struct CERT_AUTHORITY_KEY_ID_INFO(i32);
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_PARTIAL_CHAIN_LOGGING_FLAG: u32 = 2u32;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_UNTRUSTED_ROOT_LOGGING_FLAG: u32 = 1u32;
pub const CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: u32 = 98u32;
pub const CERT_AUTO_ENROLL_PROP_ID: u32 = 21u32;
pub const CERT_AUTO_ENROLL_RETRY_PROP_ID: u32 = 66u32;
pub const CERT_AUTO_UPDATE_DISABLE_RANDOM_QUERY_STRING_FLAG: u32 = 4u32;
pub const CERT_BACKED_UP_PROP_ID: u32 = 69u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_BASIC_CONSTRAINTS2_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_BASIC_CONSTRAINTS_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_BIOMETRIC_DATA(i32);
pub struct CERT_BIOMETRIC_DATA_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_BIOMETRIC_EXT_INFO(i32);
pub const CERT_BIOMETRIC_PICTURE_TYPE: u32 = 0u32;
pub const CERT_BIOMETRIC_SIGNATURE_TYPE: u32 = 1u32;
pub const CERT_BUNDLE_CERTIFICATE: u32 = 0u32;
pub const CERT_BUNDLE_CRL: u32 = 1u32;
pub const CERT_CASE_INSENSITIVE_IS_RDN_ATTRS_FLAG: u32 = 2u32;
pub const CERT_CA_DISABLE_CRL_PROP_ID: u32 = 82u32;
pub const CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 81u32;
pub const CERT_CA_SUBJECT_FLAG: u32 = 128u32;
pub const CERT_CEP_PROP_ID: u32 = 87u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CHAIN(i32);
pub const CERT_CHAIN_AUTO_CURRENT_USER: u32 = 1u32;
pub const CERT_CHAIN_AUTO_FLUSH_DISABLE_FLAG: u32 = 1u32;
pub const CERT_CHAIN_AUTO_HPKP_RULE_INFO: u32 = 8u32;
pub const CERT_CHAIN_AUTO_IMPERSONATED: u32 = 3u32;
pub const CERT_CHAIN_AUTO_LOCAL_MACHINE: u32 = 2u32;
pub const CERT_CHAIN_AUTO_LOG_CREATE_FLAG: u32 = 2u32;
pub const CERT_CHAIN_AUTO_LOG_FLUSH_FLAG: u32 = 8u32;
pub const CERT_CHAIN_AUTO_LOG_FREE_FLAG: u32 = 4u32;
pub const CERT_CHAIN_AUTO_NETWORK_INFO: u32 = 6u32;
pub const CERT_CHAIN_AUTO_PINRULE_INFO: u32 = 5u32;
pub const CERT_CHAIN_AUTO_PROCESS_INFO: u32 = 4u32;
pub const CERT_CHAIN_AUTO_SERIAL_LOCAL_MACHINE: u32 = 7u32;
pub const CERT_CHAIN_CACHE_END_CERT: u32 = 1u32;
pub const CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CHAIN_CONTEXT(i32);
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_DEFAULT: u32 = 12u32;
pub const CERT_CHAIN_DISABLE_AIA: u32 = 8192u32;
pub const CERT_CHAIN_DISABLE_ALL_EKU_WEAK_FLAG: u32 = 65536u32;
pub const CERT_CHAIN_DISABLE_AUTH_ROOT_AUTO_UPDATE: u32 = 256u32;
pub const CERT_CHAIN_DISABLE_CODE_SIGNING_WEAK_FLAG: u32 = 4194304u32;
pub const CERT_CHAIN_DISABLE_ECC_PARA_FLAG: u32 = 16u32;
pub const CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAG: u32 = 4096u32;
pub const CERT_CHAIN_DISABLE_MD2_MD4: u32 = 4096u32;
pub const CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG: u32 = 8388608u32;
pub const CERT_CHAIN_DISABLE_MOTW_FILE_HASH_WEAK_FLAG: u32 = 8192u32;
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_HASH_WEAK_FLAG: u32 = 32768u32;
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG: u32 = 134217728u32;
pub const CERT_CHAIN_DISABLE_MY_PEER_TRUST: u32 = 2048u32;
pub const CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG: u32 = 262144u32;
pub const CERT_CHAIN_DISABLE_PASS1_QUALITY_FILTERING: u32 = 64u32;
pub const CERT_CHAIN_DISABLE_SERVER_AUTH_WEAK_FLAG: u32 = 1048576u32;
pub const CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAG: u32 = 16384u32;
pub const CERT_CHAIN_DISABLE_TIMESTAMP_WEAK_FLAG: u32 = 67108864u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CHAIN_ELEMENT(i32);
pub const CERT_CHAIN_ENABLE_ALL_EKU_HYGIENE_FLAG: u32 = 131072u32;
pub const CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE: u32 = 16u32;
pub const CERT_CHAIN_ENABLE_CODE_SIGNING_HYGIENE_FLAG: u32 = 16777216u32;
pub const CERT_CHAIN_ENABLE_MD2_MD4_FLAG: u32 = 1u32;
pub const CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG: u32 = 33554432u32;
pub const CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG: u32 = 536870912u32;
pub const CERT_CHAIN_ENABLE_ONLY_WEAK_LOGGING_FLAG: u32 = 8u32;
pub const CERT_CHAIN_ENABLE_PEER_TRUST: u32 = 1024u32;
pub const CERT_CHAIN_ENABLE_SERVER_AUTH_HYGIENE_FLAG: u32 = 2097152u32;
pub const CERT_CHAIN_ENABLE_SHARE_STORE: u32 = 32u32;
pub const CERT_CHAIN_ENABLE_TIMESTAMP_HYGIENE_FLAG: u32 = 268435456u32;
pub const CERT_CHAIN_ENABLE_WEAK_LOGGING_FLAG: u32 = 4u32;
pub const CERT_CHAIN_ENABLE_WEAK_RSA_ROOT_FLAG: u32 = 2u32;
pub const CERT_CHAIN_ENABLE_WEAK_SETTINGS_FLAG: u32 = 2147483648u32;
pub struct CERT_CHAIN_ENGINE_CONFIG(i32);
pub const CERT_CHAIN_EXCLUSIVE_ENABLE_CA_FLAG: u32 = 1u32;
pub const CERT_CHAIN_FIND_BY_ISSUER: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CHAIN_FIND_BY_ISSUER_PARA(i32);
pub const CERT_CHAIN_HAS_MOTW: u32 = 16384u32;
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_DEFAULT: u32 = 5u32;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_DEFAULT: u32 = 100000u32;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_DEFAULT: u32 = 10u32;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_DEFAULT: u32 = 3u32;
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DEFAULT: u32 = 5u32;
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DISABLE: u32 = 4294967295u32;
pub const CERT_CHAIN_MIN_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295u32;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DEFAULT: u32 = 1023u32;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DISABLE: u32 = 4294967295u32;
pub const CERT_CHAIN_MOTW_IGNORE_AFTER_TIME_WEAK_FLAG: u32 = 1073741824u32;
pub const CERT_CHAIN_ONLY_ADDITIONAL_AND_AUTH_ROOT: u32 = 32768u32;
pub const CERT_CHAIN_OPTION_DISABLE_AIA_URL_RETRIEVAL: u32 = 2u32;
pub const CERT_CHAIN_OPTION_ENABLE_SIA_URL_RETRIEVAL: u32 = 4u32;
pub const CERT_CHAIN_OPT_IN_WEAK_FLAGS: u32 = 262144u32;
pub const CERT_CHAIN_OPT_IN_WEAK_SIGNATURE: u32 = 65536u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CHAIN_PARA(i32);
pub struct CERT_CHAIN_POLICY_FLAGS(i32);
pub const CERT_CHAIN_POLICY_IGNORE_WEAK_SIGNATURE_FLAG: u32 = 134217728u32;
pub struct CERT_CHAIN_POLICY_PARA(i32);
pub const CERT_CHAIN_POLICY_SSL_F12_ERROR_LEVEL: u32 = 2u32;
pub const CERT_CHAIN_POLICY_SSL_F12_NONE_CATEGORY: u32 = 0u32;
pub const CERT_CHAIN_POLICY_SSL_F12_ROOT_PROGRAM_CATEGORY: u32 = 2u32;
pub const CERT_CHAIN_POLICY_SSL_F12_SUCCESS_LEVEL: u32 = 0u32;
pub const CERT_CHAIN_POLICY_SSL_F12_WARNING_LEVEL: u32 = 1u32;
pub const CERT_CHAIN_POLICY_SSL_F12_WEAK_CRYPTO_CATEGORY: u32 = 1u32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_ERROR: i32 = -2i32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_WARNING: u32 = 2u32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_ERROR: i32 = -1i32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_WARNING: u32 = 1u32;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_SUCCESS: u32 = 0u32;
pub struct CERT_CHAIN_POLICY_STATUS(i32);
pub const CERT_CHAIN_RETURN_LOWER_QUALITY_CONTEXTS: u32 = 128u32;
pub const CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT: u32 = 134217728u32;
pub const CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 2147483648u32;
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN: u32 = 536870912u32;
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 1073741824u32;
pub const CERT_CHAIN_REVOCATION_CHECK_END_CERT: u32 = 268435456u32;
pub const CERT_CHAIN_REVOCATION_CHECK_OCSP_CERT: u32 = 67108864u32;
pub const CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG: u32 = 1u32;
pub const CERT_CHAIN_THREAD_STORE_SYNC: u32 = 2u32;
pub const CERT_CHAIN_TIMESTAMP_TIME: u32 = 512u32;
pub const CERT_CHAIN_USE_LOCAL_MACHINE_STORE: u32 = 8u32;
pub const CERT_CLOSE_STORE_CHECK_FLAG: u32 = 2u32;
pub const CERT_CLOSE_STORE_FORCE_FLAG: u32 = 1u32;
pub const CERT_CLR_DELETE_KEY_PROP_ID: u32 = 125u32;
pub const CERT_COMPARE_ANY: u32 = 0u32;
pub const CERT_COMPARE_ATTR: u32 = 3u32;
pub const CERT_COMPARE_CERT_ID: u32 = 16u32;
pub const CERT_COMPARE_CROSS_CERT_DIST_POINTS: u32 = 17u32;
pub const CERT_COMPARE_CTL_USAGE: u32 = 10u32;
pub const CERT_COMPARE_ENHKEY_USAGE: u32 = 10u32;
pub const CERT_COMPARE_EXISTING: u32 = 13u32;
pub const CERT_COMPARE_HASH: u32 = 1u32;
pub const CERT_COMPARE_HASH_STR: u32 = 20u32;
pub const CERT_COMPARE_HAS_PRIVATE_KEY: u32 = 21u32;
pub const CERT_COMPARE_ISSUER_OF: u32 = 12u32;
pub const CERT_COMPARE_KEY_IDENTIFIER: u32 = 15u32;
pub const CERT_COMPARE_KEY_SPEC: u32 = 9u32;
pub const CERT_COMPARE_MASK: u32 = 65535u32;
pub const CERT_COMPARE_MD5_HASH: u32 = 4u32;
pub const CERT_COMPARE_NAME: u32 = 2u32;
pub const CERT_COMPARE_NAME_STR_A: u32 = 7u32;
pub const CERT_COMPARE_NAME_STR_W: u32 = 8u32;
pub const CERT_COMPARE_PROPERTY: u32 = 5u32;
pub const CERT_COMPARE_PUBKEY_MD5_HASH: u32 = 18u32;
pub const CERT_COMPARE_PUBLIC_KEY: u32 = 6u32;
pub const CERT_COMPARE_SHA1_HASH: u32 = 1u32;
pub const CERT_COMPARE_SHIFT: i32 = 16i32;
pub const CERT_COMPARE_SIGNATURE_HASH: u32 = 14u32;
pub const CERT_COMPARE_SUBJECT_CERT: u32 = 11u32;
pub const CERT_COMPARE_SUBJECT_INFO_ACCESS: u32 = 19u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CONTEXT(i32);
pub const CERT_CONTEXT_REVOCATION_TYPE: u32 = 1u32;
pub struct CERT_CONTROL_STORE_FLAGS(i32);
pub const CERT_CREATE_CONTEXT_NOCOPY_FLAG: u32 = 1u32;
pub const CERT_CREATE_CONTEXT_NO_ENTRY_FLAG: u32 = 8u32;
pub const CERT_CREATE_CONTEXT_NO_HCRYPTMSG_FLAG: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CREATE_CONTEXT_PARA(i32);
pub const CERT_CREATE_CONTEXT_SORTED_FLAG: u32 = 2u32;
pub struct CERT_CREATE_SELFSIGN_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_CRL_CONTEXT_PAIR(i32);
pub const CERT_CRL_SIGN_KEY_USAGE: u32 = 2u32;
pub const CERT_CROSS_CERT_DIST_POINTS_PROP_ID: u32 = 23u32;
pub const CERT_CTL_USAGE_PROP_ID: u32 = 9u32;
pub const CERT_DATA_ENCIPHERMENT_KEY_USAGE: u32 = 16u32;
pub const CERT_DATE_STAMP_PROP_ID: u32 = 27u32;
pub const CERT_DECIPHER_ONLY_KEY_USAGE: u32 = 128u32;
pub const CERT_DESCRIPTION_PROP_ID: u32 = 13u32;
pub struct CERT_DH_PARAMETERS(i32);
pub const CERT_DIGITAL_SIGNATURE_KEY_USAGE: u32 = 128u32;
pub const CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: u32 = 122u32;
pub const CERT_DISALLOWED_FILETIME_PROP_ID: u32 = 104u32;
pub struct CERT_DSS_PARAMETERS(i32);
pub const CERT_DSS_R_LEN: u32 = 20u32;
pub const CERT_DSS_S_LEN: u32 = 20u32;
pub struct CERT_ECC_SIGNATURE(i32);
pub const CERT_EFS_PROP_ID: u32 = 17u32;
pub const CERT_ENCIPHER_ONLY_KEY_USAGE: u32 = 1u32;
pub const CERT_ENCODING_TYPE_MASK: u32 = 65535u32;
pub const CERT_END_ENTITY_SUBJECT_FLAG: u32 = 64u32;
pub const CERT_ENHKEY_USAGE_PROP_ID: u32 = 9u32;
pub const CERT_ENROLLMENT_PROP_ID: u32 = 26u32;
pub const CERT_EXCLUDED_SUBTREE_BIT: i32 = -2147483648i32;
pub const CERT_EXTENDED_ERROR_INFO_PROP_ID: u32 = 30u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_EXTENSION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_EXTENSIONS(i32);
pub const CERT_FILE_HASH_USE_TYPE: u32 = 1u32;
pub const CERT_FILE_STORE_COMMIT_ENABLE_FLAG: u32 = 65536u32;
pub struct CERT_FIND_CHAIN_IN_STORE_FLAGS(i32);
pub struct CERT_FIND_FLAGS(i32);
pub struct CERT_FIND_TYPE(i32);
pub const CERT_FIRST_RESERVED_PROP_ID: u32 = 128u32;
pub const CERT_FIRST_USER_PROP_ID: u32 = 32768u32;
pub struct CERT_FORTEZZA_DATA_PROP(i32);
pub const CERT_FORTEZZA_DATA_PROP_ID: u32 = 18u32;
pub const CERT_FRIENDLY_NAME_PROP_ID: u32 = 11u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_GENERAL_SUBTREE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_HASHED_URL(i32);
pub const CERT_HASH_PROP_ID: u32 = 3u32;
pub const CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 79u32;
pub const CERT_HCRYPTPROV_TRANSFER_PROP_ID: u32 = 100u32;
pub struct CERT_ID(i32);
pub struct CERT_ID_OPTION(i32);
pub const CERT_IE30_RESERVED_PROP_ID: u32 = 7u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_INFO(i32);
pub const CERT_INFO_EXTENSION_FLAG: u32 = 11u32;
pub const CERT_INFO_ISSUER_FLAG: u32 = 4u32;
pub const CERT_INFO_ISSUER_UNIQUE_ID_FLAG: u32 = 9u32;
pub const CERT_INFO_NOT_AFTER_FLAG: u32 = 6u32;
pub const CERT_INFO_NOT_BEFORE_FLAG: u32 = 5u32;
pub const CERT_INFO_SERIAL_NUMBER_FLAG: u32 = 2u32;
pub const CERT_INFO_SIGNATURE_ALGORITHM_FLAG: u32 = 3u32;
pub const CERT_INFO_SUBJECT_FLAG: u32 = 7u32;
pub const CERT_INFO_SUBJECT_PUBLIC_KEY_INFO_FLAG: u32 = 8u32;
pub const CERT_INFO_SUBJECT_UNIQUE_ID_FLAG: u32 = 10u32;
pub const CERT_INFO_VERSION_FLAG: u32 = 1u32;
pub const CERT_ISOLATED_KEY_PROP_ID: u32 = 118u32;
pub const CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: u32 = 96u32;
pub const CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: u32 = 95u32;
pub const CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 24u32;
pub const CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: u32 = 94u32;
pub struct CERT_ISSUER_SERIAL_NUMBER(i32);
pub const CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: u32 = 28u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_KEYGEN_REQUEST_INFO(i32);
pub const CERT_KEYGEN_REQUEST_V1: u32 = 0u32;
pub const CERT_KEY_AGREEMENT_KEY_USAGE: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_KEY_ATTRIBUTES_INFO(i32);
pub const CERT_KEY_CERT_SIGN_KEY_USAGE: u32 = 4u32;
pub const CERT_KEY_CLASSIFICATION_PROP_ID: u32 = 120u32;
pub struct CERT_KEY_CONTEXT(i32);
pub const CERT_KEY_CONTEXT_PROP_ID: u32 = 5u32;
pub const CERT_KEY_ENCIPHERMENT_KEY_USAGE: u32 = 32u32;
pub const CERT_KEY_IDENTIFIER_PROP_ID: u32 = 20u32;
pub const CERT_KEY_PROV_HANDLE_PROP_ID: u32 = 1u32;
pub const CERT_KEY_PROV_INFO_PROP_ID: u32 = 2u32;
pub const CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: u32 = 103u32;
pub struct CERT_KEY_SPEC(i32);
pub const CERT_KEY_SPEC_PROP_ID: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_KEY_USAGE_RESTRICTION_INFO(i32);
pub const CERT_LAST_RESERVED_PROP_ID: u32 = 32767u32;
pub const CERT_LAST_USER_PROP_ID: u32 = 65535u32;
pub const CERT_LDAP_STORE_AREC_EXCLUSIVE_FLAG: u32 = 131072u32;
pub const CERT_LDAP_STORE_OPENED_FLAG: u32 = 262144u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LDAP_STORE_OPENED_PARA(i32);
pub const CERT_LDAP_STORE_SIGN_FLAG: u32 = 65536u32;
pub const CERT_LDAP_STORE_UNBIND_FLAG: u32 = 524288u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_AUDIO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_AUDIO_INFO(i32);
pub struct CERT_LOGOTYPE_CHOICE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_DETAILS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_EXT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_IMAGE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_IMAGE_INFO(i32);
pub struct CERT_LOGOTYPE_IMAGE_INFO_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_INFO(i32);
pub struct CERT_LOGOTYPE_OPTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_LOGOTYPE_REFERENCE(i32);
pub const CERT_MD5_HASH_PROP_ID: u32 = 4u32;
pub const CERT_NAME_ATTR_TYPE: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_NAME_CONSTRAINTS_INFO(i32);
pub const CERT_NAME_DISABLE_IE4_UTF8_FLAG: u32 = 65536u32;
pub const CERT_NAME_DNS_TYPE: u32 = 6u32;
pub const CERT_NAME_EMAIL_TYPE: u32 = 1u32;
pub const CERT_NAME_FRIENDLY_DISPLAY_TYPE: u32 = 5u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_NAME_INFO(i32);
pub const CERT_NAME_ISSUER_FLAG: u32 = 1u32;
pub const CERT_NAME_RDN_TYPE: u32 = 2u32;
pub const CERT_NAME_SEARCH_ALL_NAMES_FLAG: u32 = 2u32;
pub const CERT_NAME_SIMPLE_DISPLAY_TYPE: u32 = 4u32;
pub const CERT_NAME_STR_COMMA_FLAG: u32 = 67108864u32;
pub const CERT_NAME_STR_CRLF_FLAG: u32 = 134217728u32;
pub const CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: u32 = 65536u32;
pub const CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: u32 = 1048576u32;
pub const CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: u32 = 2097152u32;
pub const CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: u32 = 131072u32;
pub const CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: u32 = 262144u32;
pub const CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: u32 = 524288u32;
pub const CERT_NAME_STR_FORWARD_FLAG: u32 = 16777216u32;
pub const CERT_NAME_STR_NO_PLUS_FLAG: u32 = 536870912u32;
pub const CERT_NAME_STR_NO_QUOTING_FLAG: u32 = 268435456u32;
pub const CERT_NAME_STR_REVERSE_FLAG: u32 = 33554432u32;
pub const CERT_NAME_STR_SEMICOLON_FLAG: u32 = 1073741824u32;
pub const CERT_NAME_UPN_TYPE: u32 = 8u32;
pub const CERT_NAME_URL_TYPE: u32 = 7u32;
pub struct CERT_NAME_VALUE(i32);
pub const CERT_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 78u32;
pub const CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: u32 = 99u32;
pub const CERT_NEW_KEY_PROP_ID: u32 = 74u32;
pub const CERT_NEXT_UPDATE_LOCATION_PROP_ID: u32 = 10u32;
pub const CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: u32 = 123u32;
pub const CERT_NON_REPUDIATION_KEY_USAGE: u32 = 64u32;
pub const CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: u32 = 127u32;
pub const CERT_NOT_BEFORE_FILETIME_PROP_ID: u32 = 126u32;
pub const CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: u32 = 77u32;
pub const CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: u32 = 97u32;
pub const CERT_OCSP_CACHE_PREFIX_PROP_ID: u32 = 75u32;
pub const CERT_OCSP_MUST_STAPLE_PROP_ID: u32 = 121u32;
pub const CERT_OCSP_RESPONSE_PROP_ID: u32 = 70u32;
pub const CERT_OFFLINE_CRL_SIGN_KEY_USAGE: u32 = 2u32;
pub struct CERT_OPEN_STORE_FLAGS(i32);
pub struct CERT_OR_CRL_BLOB(i32);
pub struct CERT_OR_CRL_BUNDLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_OTHER_LOGOTYPE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_OTHER_NAME(i32);
pub struct CERT_PAIR(i32);
pub const CERT_PHYSICAL_STORE_ADD_ENABLE_FLAG: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_PHYSICAL_STORE_INFO(i32);
pub const CERT_PHYSICAL_STORE_INSERT_COMPUTER_NAME_ENABLE_FLAG: u32 = 8u32;
pub const CERT_PHYSICAL_STORE_OPEN_DISABLE_FLAG: u32 = 2u32;
pub const CERT_PHYSICAL_STORE_PREDEFINED_ENUM_FLAG: u32 = 1u32;
pub const CERT_PHYSICAL_STORE_REMOTE_OPEN_DISABLE_FLAG: u32 = 4u32;
pub const CERT_PIN_SHA256_HASH_PROP_ID: u32 = 124u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICIES_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY95_QUALIFIER1(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_CONSTRAINTS_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_MAPPING(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_MAPPINGS_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_QUALIFIER_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_QUALIFIER_NOTICE_REFERENCE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_POLICY_QUALIFIER_USER_NOTICE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_PRIVATE_KEY_VALIDITY(i32);
pub const CERT_PROT_ROOT_DISABLE_CURRENT_USER_FLAG: u32 = 1u32;
pub const CERT_PROT_ROOT_DISABLE_LM_AUTH_FLAG: u32 = 8u32;
pub const CERT_PROT_ROOT_DISABLE_NOT_DEFINED_NAME_CONSTRAINT_FLAG: u32 = 32u32;
pub const CERT_PROT_ROOT_DISABLE_NT_AUTH_REQUIRED_FLAG: u32 = 16u32;
pub const CERT_PROT_ROOT_DISABLE_PEER_TRUST: u32 = 65536u32;
pub const CERT_PROT_ROOT_INHIBIT_ADD_AT_INIT_FLAG: u32 = 2u32;
pub const CERT_PROT_ROOT_INHIBIT_PURGE_LM_FLAG: u32 = 4u32;
pub const CERT_PROT_ROOT_ONLY_LM_GPT_FLAG: u32 = 8u32;
pub const CERT_PUBKEY_ALG_PARA_PROP_ID: u32 = 22u32;
pub const CERT_PUBKEY_HASH_RESERVED_PROP_ID: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_PUBLIC_KEY_INFO(i32);
pub const CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: u32 = 93u32;
pub const CERT_PVK_FILE_PROP_ID: u32 = 12u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_QC_STATEMENT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_QC_STATEMENTS_EXT_INFO(i32);
pub struct CERT_QUERY_CONTENT_TYPE(i32);
pub struct CERT_QUERY_CONTENT_TYPE_FLAGS(i32);
pub struct CERT_QUERY_ENCODING_TYPE(i32);
pub struct CERT_QUERY_FORMAT_TYPE(i32);
pub struct CERT_QUERY_FORMAT_TYPE_FLAGS(i32);
pub struct CERT_QUERY_OBJECT_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_RDN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_RDN_ATTR(i32);
pub struct CERT_RDN_ATTR_VALUE_TYPE(i32);
pub const CERT_RDN_DISABLE_CHECK_TYPE_FLAG: u32 = 1073741824u32;
pub const CERT_RDN_DISABLE_IE4_UTF8_FLAG: u32 = 16777216u32;
pub const CERT_RDN_ENABLE_PUNYCODE_FLAG: u32 = 33554432u32;
pub const CERT_RDN_ENABLE_T61_UNICODE_FLAG: u32 = 2147483648u32;
pub const CERT_RDN_ENABLE_UTF8_UNICODE_FLAG: u32 = 536870912u32;
pub const CERT_RDN_FLAGS_MASK: u32 = 4278190080u32;
pub const CERT_RDN_FORCE_UTF8_UNICODE_FLAG: u32 = 268435456u32;
pub const CERT_RDN_TYPE_MASK: u32 = 255u32;
pub const CERT_REGISTRY_STORE_CLIENT_GPT_FLAG: u32 = 2147483648u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CERT_REGISTRY_STORE_CLIENT_GPT_PARA(i32);
pub const CERT_REGISTRY_STORE_EXTERNAL_FLAG: u32 = 1048576u32;
pub const CERT_REGISTRY_STORE_LM_GPT_FLAG: u32 = 16777216u32;
pub const CERT_REGISTRY_STORE_MY_IE_DIRTY_FLAG: u32 = 524288u32;
pub const CERT_REGISTRY_STORE_REMOTE_FLAG: u32 = 65536u32;
pub const CERT_REGISTRY_STORE_ROAMING_FLAG: u32 = 262144u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CERT_REGISTRY_STORE_ROAMING_PARA(i32);
pub const CERT_REGISTRY_STORE_SERIALIZED_FLAG: u32 = 131072u32;
pub const CERT_RENEWAL_PROP_ID: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REQUEST_INFO(i32);
pub const CERT_REQUEST_ORIGINATOR_PROP_ID: u32 = 71u32;
pub const CERT_REQUEST_V1: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_CHAIN_PARA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_CRL_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_PARA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_REVOCATION_STATUS(i32);
pub struct CERT_REVOCATION_STATUS_REASON(i32);
pub const CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: u32 = 83u32;
pub const CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: u32 = 105u32;
pub struct CERT_ROOT_PROGRAM_FLAGS(i32);
pub const CERT_ROOT_PROGRAM_FLAG_ADDRESS: u32 = 8u32;
pub const CERT_ROOT_PROGRAM_FLAG_OU: u32 = 16u32;
pub const CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: u32 = 84u32;
pub const CERT_SCARD_PIN_ID_PROP_ID: u32 = 90u32;
pub const CERT_SCARD_PIN_INFO_PROP_ID: u32 = 91u32;
pub const CERT_SCEP_CA_CERT_PROP_ID: u32 = 111u32;
pub const CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: u32 = 114u32;
pub const CERT_SCEP_FLAGS_PROP_ID: u32 = 115u32;
pub const CERT_SCEP_GUID_PROP_ID: u32 = 116u32;
pub const CERT_SCEP_NONCE_PROP_ID: u32 = 113u32;
pub const CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: u32 = 110u32;
pub const CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: u32 = 109u32;
pub const CERT_SCEP_SERVER_CERTS_PROP_ID: u32 = 108u32;
pub const CERT_SCEP_SIGNER_CERT_PROP_ID: u32 = 112u32;
pub const CERT_SELECT_ALLOW_DUPLICATES: u32 = 128u32;
pub const CERT_SELECT_ALLOW_EXPIRED: u32 = 1u32;
pub const CERT_SELECT_BY_FRIENDLYNAME: u32 = 13u32;
pub const CERT_SELECT_BY_ISSUER_DISPLAYNAME: u32 = 12u32;
pub const CERT_SELECT_BY_THUMBPRINT: u32 = 14u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SELECT_CHAIN_PARA(i32);
pub struct CERT_SELECT_CRITERIA(i32);
pub struct CERT_SELECT_CRITERIA_TYPE(i32);
pub const CERT_SELECT_DISALLOW_SELFSIGNED: u32 = 4u32;
pub const CERT_SELECT_HARDWARE_ONLY: u32 = 64u32;
pub const CERT_SELECT_HAS_KEY_FOR_KEY_EXCHANGE: u32 = 32u32;
pub const CERT_SELECT_HAS_KEY_FOR_SIGNATURE: u32 = 16u32;
pub const CERT_SELECT_HAS_PRIVATE_KEY: u32 = 8u32;
pub const CERT_SELECT_IGNORE_AUTOSELECT: u32 = 256u32;
pub const CERT_SELECT_MAX_PARA: u32 = 500u32;
pub const CERT_SELECT_TRUSTED_ROOT: u32 = 2u32;
pub const CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: u32 = 102u32;
pub const CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: u32 = 117u32;
pub const CERT_SERIAL_CHAIN_PROP_ID: u32 = 119u32;
pub const CERT_SERVER_OCSP_RESPONSE_ASYNC_FLAG: u32 = 1u32;
pub struct CERT_SERVER_OCSP_RESPONSE_CONTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SERVER_OCSP_RESPONSE_OPEN_PARA(i32);
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_READ_FLAG: u32 = 1u32;
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_WRITE_FLAG: u32 = 2u32;
pub const CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG: u32 = 2147483648u32;
pub const CERT_SET_PROPERTY_INHIBIT_PERSIST_FLAG: u32 = 1073741824u32;
pub const CERT_SHA1_HASH_PROP_ID: u32 = 3u32;
pub const CERT_SHA256_HASH_PROP_ID: u32 = 107u32;
pub const CERT_SIGNATURE_HASH_PROP_ID: u32 = 15u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SIGNED_CONTENT_INFO(i32);
pub const CERT_SIGN_HASH_CNG_ALG_PROP_ID: u32 = 89u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SIMPLE_CHAIN(i32);
pub const CERT_SMART_CARD_DATA_PROP_ID: u32 = 16u32;
pub const CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: u32 = 106u32;
pub const CERT_SMART_CARD_READER_PROP_ID: u32 = 101u32;
pub const CERT_SMART_CARD_ROOT_INFO_PROP_ID: u32 = 76u32;
pub const CERT_SOURCE_LOCATION_PROP_ID: u32 = 72u32;
pub const CERT_SOURCE_URL_PROP_ID: u32 = 73u32;
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_DEFAULT: u32 = 5u32;
pub const CERT_STORE_ADD_ALWAYS: u32 = 4u32;
pub const CERT_STORE_ADD_NEW: u32 = 1u32;
pub const CERT_STORE_ADD_NEWER: u32 = 6u32;
pub const CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES: u32 = 7u32;
pub const CERT_STORE_ADD_REPLACE_EXISTING: u32 = 3u32;
pub const CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES: u32 = 5u32;
pub const CERT_STORE_ADD_USE_EXISTING: u32 = 2u32;
pub const CERT_STORE_BASE_CRL_FLAG: u32 = 256u32;
pub const CERT_STORE_CERTIFICATE_CONTEXT: u32 = 1u32;
pub const CERT_STORE_CRL_CONTEXT: u32 = 2u32;
pub const CERT_STORE_CTL_CONTEXT: u32 = 3u32;
pub const CERT_STORE_CTRL_AUTO_RESYNC: u32 = 4u32;
pub const CERT_STORE_CTRL_CANCEL_NOTIFY: u32 = 5u32;
pub const CERT_STORE_CTRL_COMMIT: u32 = 3u32;
pub const CERT_STORE_CTRL_NOTIFY_CHANGE: u32 = 2u32;
pub const CERT_STORE_CTRL_RESYNC: u32 = 1u32;
pub const CERT_STORE_DELTA_CRL_FLAG: u32 = 512u32;
pub const CERT_STORE_LOCALIZED_NAME_PROP_ID: u32 = 4096u32;
pub const CERT_STORE_MANIFOLD_FLAG: u32 = 256u32;
pub const CERT_STORE_NO_CRL_FLAG: u32 = 65536u32;
pub const CERT_STORE_NO_ISSUER_FLAG: u32 = 131072u32;
pub const CERT_STORE_PROV_CLOSE_FUNC: u32 = 0u32;
pub const CERT_STORE_PROV_CONTROL_FUNC: u32 = 13u32;
pub const CERT_STORE_PROV_DELETE_CERT_FUNC: u32 = 3u32;
pub const CERT_STORE_PROV_DELETE_CRL_FUNC: u32 = 7u32;
pub const CERT_STORE_PROV_DELETE_CTL_FUNC: u32 = 11u32;
pub const CERT_STORE_PROV_FIND_CERT_FUNC: u32 = 14u32;
pub const CERT_STORE_PROV_FIND_CRL_FUNC: u32 = 17u32;
pub const CERT_STORE_PROV_FIND_CTL_FUNC: u32 = 20u32;
pub struct CERT_STORE_PROV_FIND_INFO(i32);
pub struct CERT_STORE_PROV_FLAGS(i32);
pub const CERT_STORE_PROV_FREE_FIND_CERT_FUNC: u32 = 15u32;
pub const CERT_STORE_PROV_FREE_FIND_CRL_FUNC: u32 = 18u32;
pub const CERT_STORE_PROV_FREE_FIND_CTL_FUNC: u32 = 21u32;
pub const CERT_STORE_PROV_GET_CERT_PROPERTY_FUNC: u32 = 16u32;
pub const CERT_STORE_PROV_GET_CRL_PROPERTY_FUNC: u32 = 19u32;
pub const CERT_STORE_PROV_GET_CTL_PROPERTY_FUNC: u32 = 22u32;
pub const CERT_STORE_PROV_GP_SYSTEM_STORE_FLAG: u32 = 32u32;
pub struct CERT_STORE_PROV_INFO(i32);
pub const CERT_STORE_PROV_READ_CERT_FUNC: u32 = 1u32;
pub const CERT_STORE_PROV_READ_CRL_FUNC: u32 = 5u32;
pub const CERT_STORE_PROV_READ_CTL_FUNC: u32 = 9u32;
pub const CERT_STORE_PROV_SET_CERT_PROPERTY_FUNC: u32 = 4u32;
pub const CERT_STORE_PROV_SET_CRL_PROPERTY_FUNC: u32 = 8u32;
pub const CERT_STORE_PROV_SET_CTL_PROPERTY_FUNC: u32 = 12u32;
pub const CERT_STORE_PROV_SHARED_USER_FLAG: u32 = 64u32;
pub const CERT_STORE_PROV_WRITE_ADD_FLAG: u32 = 1u32;
pub const CERT_STORE_PROV_WRITE_CERT_FUNC: u32 = 2u32;
pub const CERT_STORE_PROV_WRITE_CRL_FUNC: u32 = 6u32;
pub const CERT_STORE_PROV_WRITE_CTL_FUNC: u32 = 10u32;
pub const CERT_STORE_REVOCATION_FLAG: u32 = 4u32;
pub struct CERT_STORE_SAVE_AS(i32);
pub const CERT_STORE_SAVE_AS_PKCS12: u32 = 3u32;
pub struct CERT_STORE_SAVE_TO(i32);
pub const CERT_STORE_SHARE_STORE_FLAG: u32 = 64u32;
pub const CERT_STORE_SIGNATURE_FLAG: u32 = 1u32;
pub const CERT_STORE_TIME_VALIDITY_FLAG: u32 = 2u32;
pub const CERT_STORE_UNSAFE_PHYSICAL_FLAG: u32 = 32u32;
pub struct CERT_STRING_TYPE(i32);
pub struct CERT_STRONG_SIGN_FLAGS(i32);
pub const CERT_STRONG_SIGN_OID_INFO_CHOICE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_STRONG_SIGN_PARA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_STRONG_SIGN_SERIALIZED_INFO(i32);
pub const CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE: u32 = 1u32;
pub const CERT_SUBJECT_DISABLE_CRL_PROP_ID: u32 = 86u32;
pub const CERT_SUBJECT_INFO_ACCESS_PROP_ID: u32 = 80u32;
pub const CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: u32 = 29u32;
pub const CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: u32 = 85u32;
pub const CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 25u32;
pub const CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: u32 = 92u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SUPPORTED_ALGORITHM_INFO(i32);
pub const CERT_SYSTEM_STORE_CURRENT_SERVICE_ID: u32 = 4u32;
pub const CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY_ID: u32 = 7u32;
pub const CERT_SYSTEM_STORE_CURRENT_USER_ID: u32 = 1u32;
pub const CERT_SYSTEM_STORE_DEFER_READ_FLAG: u32 = 536870912u32;
pub struct CERT_SYSTEM_STORE_FLAGS(i32);
pub struct CERT_SYSTEM_STORE_INFO(i32);
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE_ID: u32 = 9u32;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY_ID: u32 = 8u32;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ID: u32 = 2u32;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_WCOS_ID: u32 = 10u32;
pub const CERT_SYSTEM_STORE_LOCATION_SHIFT: u32 = 16u32;
pub const CERT_SYSTEM_STORE_MASK: u32 = 4294901760u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CERT_SYSTEM_STORE_RELOCATE_PARA(i32);
pub const CERT_SYSTEM_STORE_SERVICES_ID: u32 = 5u32;
pub const CERT_SYSTEM_STORE_UNPROTECTED_FLAG: u32 = 1073741824u32;
pub const CERT_SYSTEM_STORE_USERS_ID: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_TEMPLATE_EXT(i32);
pub const CERT_TIMESTAMP_HASH_USE_TYPE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_TPM_SPECIFICATION_INFO(i32);
pub const CERT_TRUST_AUTO_UPDATE_CA_REVOCATION: u32 = 16u32;
pub const CERT_TRUST_AUTO_UPDATE_END_REVOCATION: u32 = 32u32;
pub const CERT_TRUST_CTL_IS_NOT_SIGNATURE_VALID: u32 = 262144u32;
pub const CERT_TRUST_CTL_IS_NOT_TIME_VALID: u32 = 131072u32;
pub const CERT_TRUST_CTL_IS_NOT_VALID_FOR_USAGE: u32 = 524288u32;
pub const CERT_TRUST_HAS_ALLOW_WEAK_SIGNATURE: u32 = 131072u32;
pub const CERT_TRUST_HAS_AUTO_UPDATE_WEAK_SIGNATURE: u32 = 32768u32;
pub const CERT_TRUST_HAS_CRL_VALIDITY_EXTENDED: u32 = 4096u32;
pub const CERT_TRUST_HAS_EXACT_MATCH_ISSUER: u32 = 1u32;
pub const CERT_TRUST_HAS_EXCLUDED_NAME_CONSTRAINT: u32 = 32768u32;
pub const CERT_TRUST_HAS_ISSUANCE_CHAIN_POLICY: u32 = 512u32;
pub const CERT_TRUST_HAS_KEY_MATCH_ISSUER: u32 = 2u32;
pub const CERT_TRUST_HAS_NAME_MATCH_ISSUER: u32 = 4u32;
pub const CERT_TRUST_HAS_NOT_DEFINED_NAME_CONSTRAINT: u32 = 8192u32;
pub const CERT_TRUST_HAS_NOT_PERMITTED_NAME_CONSTRAINT: u32 = 16384u32;
pub const CERT_TRUST_HAS_NOT_SUPPORTED_CRITICAL_EXT: u32 = 134217728u32;
pub const CERT_TRUST_HAS_NOT_SUPPORTED_NAME_CONSTRAINT: u32 = 4096u32;
pub const CERT_TRUST_HAS_PREFERRED_ISSUER: u32 = 256u32;
pub const CERT_TRUST_HAS_VALID_NAME_CONSTRAINTS: u32 = 1024u32;
pub const CERT_TRUST_HAS_WEAK_HYGIENE: u32 = 2097152u32;
pub const CERT_TRUST_HAS_WEAK_SIGNATURE: u32 = 1048576u32;
pub const CERT_TRUST_INVALID_BASIC_CONSTRAINTS: u32 = 1024u32;
pub const CERT_TRUST_INVALID_EXTENSION: u32 = 256u32;
pub const CERT_TRUST_INVALID_NAME_CONSTRAINTS: u32 = 2048u32;
pub const CERT_TRUST_INVALID_POLICY_CONSTRAINTS: u32 = 512u32;
pub const CERT_TRUST_IS_CA_TRUSTED: u32 = 16384u32;
pub const CERT_TRUST_IS_COMPLEX_CHAIN: u32 = 65536u32;
pub const CERT_TRUST_IS_CYCLIC: u32 = 128u32;
pub const CERT_TRUST_IS_EXPLICIT_DISTRUST: u32 = 67108864u32;
pub const CERT_TRUST_IS_FROM_EXCLUSIVE_TRUST_STORE: u32 = 8192u32;
pub const CERT_TRUST_IS_KEY_ROLLOVER: u32 = 128u32;
pub const CERT_TRUST_IS_NOT_SIGNATURE_VALID: u32 = 8u32;
pub const CERT_TRUST_IS_NOT_TIME_NESTED: u32 = 2u32;
pub const CERT_TRUST_IS_NOT_TIME_VALID: u32 = 1u32;
pub const CERT_TRUST_IS_NOT_VALID_FOR_USAGE: u32 = 16u32;
pub const CERT_TRUST_IS_OFFLINE_REVOCATION: u32 = 16777216u32;
pub const CERT_TRUST_IS_PARTIAL_CHAIN: u32 = 65536u32;
pub const CERT_TRUST_IS_PEER_TRUSTED: u32 = 2048u32;
pub const CERT_TRUST_IS_REVOKED: u32 = 4u32;
pub const CERT_TRUST_IS_SELF_SIGNED: u32 = 8u32;
pub const CERT_TRUST_IS_UNTRUSTED_ROOT: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_TRUST_LIST_INFO(i32);
pub const CERT_TRUST_NO_ERROR: u32 = 0u32;
pub const CERT_TRUST_NO_ISSUANCE_CHAIN_POLICY: u32 = 33554432u32;
pub const CERT_TRUST_NO_OCSP_FAILOVER_TO_CRL: u32 = 64u32;
pub const CERT_TRUST_NO_TIME_CHECK: u32 = 33554432u32;
pub const CERT_TRUST_PUB_ALLOW_END_USER_TRUST: u32 = 0u32;
pub const CERT_TRUST_PUB_ALLOW_ENTERPRISE_ADMIN_TRUST: u32 = 2u32;
pub const CERT_TRUST_PUB_ALLOW_MACHINE_ADMIN_TRUST: u32 = 1u32;
pub const CERT_TRUST_PUB_ALLOW_TRUST_MASK: u32 = 3u32;
pub const CERT_TRUST_PUB_CHECK_PUBLISHER_REV_FLAG: u32 = 256u32;
pub const CERT_TRUST_PUB_CHECK_TIMESTAMP_REV_FLAG: u32 = 512u32;
pub const CERT_TRUST_REVOCATION_STATUS_UNKNOWN: u32 = 64u32;
pub const CERT_TRUST_SSL_HANDSHAKE_OCSP: u32 = 262144u32;
pub const CERT_TRUST_SSL_RECONNECT_OCSP: u32 = 1048576u32;
pub const CERT_TRUST_SSL_TIME_VALID: u32 = 16777216u32;
pub const CERT_TRUST_SSL_TIME_VALID_OCSP: u32 = 524288u32;
pub struct CERT_TRUST_STATUS(i32);
pub const CERT_UNICODE_ATTR_ERR_INDEX_MASK: u32 = 63u32;
pub const CERT_UNICODE_ATTR_ERR_INDEX_SHIFT: u32 = 16u32;
pub const CERT_UNICODE_IS_RDN_ATTRS_FLAG: u32 = 1u32;
pub const CERT_UNICODE_RDN_ERR_INDEX_MASK: u32 = 1023u32;
pub const CERT_UNICODE_RDN_ERR_INDEX_SHIFT: u32 = 22u32;
pub const CERT_UNICODE_VALUE_ERR_INDEX_MASK: u32 = 65535u32;
pub const CERT_UNICODE_VALUE_ERR_INDEX_SHIFT: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_USAGE_MATCH(i32);
pub const CERT_V1: u32 = 0u32;
pub const CERT_V2: u32 = 1u32;
pub const CERT_V3: u32 = 2u32;
pub const CERT_VERIFY_ALLOW_MORE_USAGE_FLAG: u32 = 8u32;
pub const CERT_VERIFY_CACHE_ONLY_BASED_REVOCATION: u32 = 2u32;
pub const CERT_VERIFY_INHIBIT_CTL_UPDATE_FLAG: u32 = 1u32;
pub const CERT_VERIFY_NO_TIME_CHECK_FLAG: u32 = 4u32;
pub const CERT_VERIFY_REV_ACCUMULATIVE_TIMEOUT_FLAG: u32 = 4u32;
pub const CERT_VERIFY_REV_CHAIN_FLAG: u32 = 1u32;
pub const CERT_VERIFY_REV_NO_OCSP_FAILOVER_TO_CRL_FLAG: u32 = 16u32;
pub const CERT_VERIFY_REV_SERVER_OCSP_FLAG: u32 = 8u32;
pub const CERT_VERIFY_REV_SERVER_OCSP_WIRE_ONLY_FLAG: u32 = 32u32;
pub const CERT_VERIFY_TRUSTED_SIGNERS_FLAG: u32 = 2u32;
pub const CERT_VERIFY_UPDATED_CTL_FLAG: u32 = 1u32;
pub struct CERT_X942_DH_PARAMETERS(i32);
pub struct CERT_X942_DH_VALIDATION_PARAMS(i32);
pub const CERT_XML_NAME_STR: u32 = 4u32;
pub struct CESSetupProperty(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CLAIMLIST(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_ADD_ATTRIBUTES_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_ADD_EXTENSIONS_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_DATA_INFO(i32);
pub const CMC_FAIL_BAD_ALG: u32 = 0u32;
pub const CMC_FAIL_BAD_CERT_ID: u32 = 4u32;
pub const CMC_FAIL_BAD_IDENTITY: u32 = 7u32;
pub const CMC_FAIL_BAD_MESSAGE_CHECK: u32 = 1u32;
pub const CMC_FAIL_BAD_REQUEST: u32 = 2u32;
pub const CMC_FAIL_BAD_TIME: u32 = 3u32;
pub const CMC_FAIL_INTERNAL_CA_ERROR: u32 = 11u32;
pub const CMC_FAIL_MUST_ARCHIVE_KEYS: u32 = 6u32;
pub const CMC_FAIL_NO_KEY_REUSE: u32 = 10u32;
pub const CMC_FAIL_POP_FAILED: u32 = 9u32;
pub const CMC_FAIL_POP_REQUIRED: u32 = 8u32;
pub const CMC_FAIL_TRY_LATER: u32 = 12u32;
pub const CMC_FAIL_UNSUPORTED_EXT: u32 = 5u32;
pub const CMC_OTHER_INFO_FAIL_CHOICE: u32 = 1u32;
pub const CMC_OTHER_INFO_NO_CHOICE: u32 = 0u32;
pub const CMC_OTHER_INFO_PEND_CHOICE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_PEND_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_RESPONSE_INFO(i32);
pub const CMC_STATUS_CONFIRM_REQUIRED: u32 = 5u32;
pub const CMC_STATUS_FAILED: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_STATUS_INFO(i32);
pub const CMC_STATUS_NO_SUPPORT: u32 = 4u32;
pub const CMC_STATUS_PENDING: u32 = 3u32;
pub const CMC_STATUS_SUCCESS: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_TAGGED_ATTRIBUTE(i32);
pub struct CMC_TAGGED_CERT_REQUEST(i32);
pub const CMC_TAGGED_CERT_REQUEST_CHOICE: u32 = 1u32;
pub struct CMC_TAGGED_CONTENT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMC_TAGGED_OTHER_MSG(i32);
pub struct CMC_TAGGED_REQUEST(i32);
pub struct CMSCEPSetup(i32);
pub const CMSG_ATTR_CERT_COUNT_PARAM: u32 = 31u32;
pub const CMSG_ATTR_CERT_PARAM: u32 = 32u32;
pub const CMSG_AUTHENTICATED_ATTRIBUTES_FLAG: u32 = 8u32;
pub const CMSG_BARE_CONTENT_FLAG: u32 = 1u32;
pub const CMSG_BARE_CONTENT_PARAM: u32 = 3u32;
pub const CMSG_CERT_COUNT_PARAM: u32 = 11u32;
pub const CMSG_CERT_PARAM: u32 = 12u32;
pub const CMSG_CMS_ENCAPSULATED_CONTENT_FLAG: u32 = 64u32;
pub const CMSG_CMS_ENCAPSULATED_CTL_FLAG: u32 = 32768u32;
pub const CMSG_CMS_RECIPIENT_COUNT_PARAM: u32 = 33u32;
pub const CMSG_CMS_RECIPIENT_ENCRYPTED_KEY_INDEX_PARAM: u32 = 35u32;
pub const CMSG_CMS_RECIPIENT_INDEX_PARAM: u32 = 34u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CMS_RECIPIENT_INFO(i32);
pub const CMSG_CMS_RECIPIENT_INFO_PARAM: u32 = 36u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CMS_SIGNER_INFO(i32);
pub const CMSG_CMS_SIGNER_INFO_PARAM: u32 = 39u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CNG_CONTENT_DECRYPT_INFO(i32);
pub const CMSG_COMPUTED_HASH_PARAM: u32 = 22u32;
pub const CMSG_CONTENTS_OCTETS_FLAG: u32 = 16u32;
pub const CMSG_CONTENT_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
pub const CMSG_CONTENT_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CONTENT_ENCRYPT_INFO(i32);
pub const CMSG_CONTENT_ENCRYPT_PAD_ENCODED_LEN_FLAG: u32 = 1u32;
pub const CMSG_CONTENT_ENCRYPT_RELEASE_CONTEXT_FLAG: u32 = 32768u32;
pub const CMSG_CONTENT_PARAM: u32 = 2u32;
pub const CMSG_CRL_COUNT_PARAM: u32 = 13u32;
pub const CMSG_CRL_PARAM: u32 = 14u32;
pub const CMSG_CRYPT_RELEASE_CONTEXT_FLAG: u32 = 32768u32;
pub const CMSG_CTRL_ADD_ATTR_CERT: u32 = 14u32;
pub const CMSG_CTRL_ADD_CERT: u32 = 10u32;
pub const CMSG_CTRL_ADD_CMS_SIGNER_INFO: u32 = 20u32;
pub const CMSG_CTRL_ADD_CRL: u32 = 12u32;
pub const CMSG_CTRL_ADD_SIGNER: u32 = 6u32;
pub const CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR: u32 = 8u32;
pub struct CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA(i32);
pub const CMSG_CTRL_DECRYPT: u32 = 2u32;
pub struct CMSG_CTRL_DECRYPT_PARA(i32);
pub const CMSG_CTRL_DEL_ATTR_CERT: u32 = 15u32;
pub const CMSG_CTRL_DEL_CERT: u32 = 11u32;
pub const CMSG_CTRL_DEL_CRL: u32 = 13u32;
pub const CMSG_CTRL_DEL_SIGNER: u32 = 7u32;
pub const CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR: u32 = 9u32;
pub struct CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA(i32);
pub const CMSG_CTRL_ENABLE_STRONG_SIGNATURE: u32 = 21u32;
pub const CMSG_CTRL_KEY_AGREE_DECRYPT: u32 = 17u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CTRL_KEY_AGREE_DECRYPT_PARA(i32);
pub const CMSG_CTRL_KEY_TRANS_DECRYPT: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CTRL_KEY_TRANS_DECRYPT_PARA(i32);
pub const CMSG_CTRL_MAIL_LIST_DECRYPT: u32 = 18u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_CTRL_MAIL_LIST_DECRYPT_PARA(i32);
pub const CMSG_CTRL_VERIFY_HASH: u32 = 5u32;
pub const CMSG_CTRL_VERIFY_SIGNATURE: u32 = 1u32;
pub const CMSG_CTRL_VERIFY_SIGNATURE_EX: u32 = 19u32;
pub struct CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA(i32);
pub const CMSG_DETACHED_FLAG: u32 = 4u32;
pub const CMSG_ENCODED_MESSAGE: u32 = 29u32;
pub const CMSG_ENCODED_SIGNER: u32 = 28u32;
pub const CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 2u32;
pub const CMSG_ENCODE_SORTED_CTL_FLAG: u32 = 1u32;
pub const CMSG_ENCODING_TYPE_MASK: u32 = 4294901760u32;
pub const CMSG_ENCRYPTED: u32 = 6u32;
pub const CMSG_ENCRYPTED_DIGEST: u32 = 27u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_ENCRYPTED_ENCODE_INFO(i32);
pub const CMSG_ENCRYPT_PARAM: u32 = 26u32;
pub const CMSG_ENVELOPED_DATA_CMS_VERSION: u32 = 2u32;
pub const CMSG_ENVELOPED_DATA_PKCS_1_5_VERSION: u32 = 0u32;
pub const CMSG_ENVELOPED_DATA_V0: u32 = 0u32;
pub const CMSG_ENVELOPED_DATA_V2: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_ENVELOPED_ENCODE_INFO(i32);
pub const CMSG_ENVELOPED_RECIPIENT_V0: u32 = 0u32;
pub const CMSG_ENVELOPED_RECIPIENT_V2: u32 = 2u32;
pub const CMSG_ENVELOPED_RECIPIENT_V3: u32 = 3u32;
pub const CMSG_ENVELOPED_RECIPIENT_V4: u32 = 4u32;
pub const CMSG_ENVELOPE_ALGORITHM_PARAM: u32 = 15u32;
pub const CMSG_HASHED_DATA_CMS_VERSION: u32 = 2u32;
pub const CMSG_HASHED_DATA_PKCS_1_5_VERSION: u32 = 0u32;
pub const CMSG_HASHED_DATA_V0: u32 = 0u32;
pub const CMSG_HASHED_DATA_V2: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_HASHED_ENCODE_INFO(i32);
pub const CMSG_HASH_ALGORITHM_PARAM: u32 = 20u32;
pub const CMSG_HASH_DATA_PARAM: u32 = 21u32;
pub const CMSG_INDEFINITE_LENGTH: u32 = 4294967295u32;
pub const CMSG_INNER_CONTENT_TYPE_PARAM: u32 = 4u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_MATERIAL_FLAG: u32 = 2u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_OBJID_FLAG: u32 = 32u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_ALG_FLAG: u32 = 4u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_BITS_FLAG: u32 = 16u32;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_PARA_FLAG: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_KEY_AGREE_ENCRYPT_INFO(i32);
pub struct CMSG_KEY_AGREE_KEY_ENCRYPT_INFO(i32);
pub struct CMSG_KEY_AGREE_OPTION(i32);
pub struct CMSG_KEY_AGREE_ORIGINATOR(i32);
pub const CMSG_KEY_AGREE_RECIPIENT: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_KEY_AGREE_RECIPIENT_INFO(i32);
pub const CMSG_KEY_AGREE_VERSION: u32 = 3u32;
pub const CMSG_KEY_TRANS_CMS_VERSION: u32 = 2u32;
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_KEY_TRANS_ENCRYPT_INFO(i32);
pub const CMSG_KEY_TRANS_PKCS_1_5_VERSION: u32 = 0u32;
pub const CMSG_KEY_TRANS_RECIPIENT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_KEY_TRANS_RECIPIENT_INFO(i32);
pub const CMSG_LENGTH_ONLY_FLAG: u32 = 2u32;
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_OBJID_FLAG: u32 = 2u32;
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_PARA_FLAG: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_MAIL_LIST_ENCRYPT_INFO(i32);
pub const CMSG_MAIL_LIST_HANDLE_KEY_CHOICE: u32 = 1u32;
pub const CMSG_MAIL_LIST_RECIPIENT: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_MAIL_LIST_RECIPIENT_INFO(i32);
pub const CMSG_MAIL_LIST_VERSION: u32 = 4u32;
pub const CMSG_MAX_LENGTH_FLAG: u32 = 32u32;
pub struct CMSG_RC2_AUX_INFO(i32);
pub struct CMSG_RC4_AUX_INFO(i32);
pub const CMSG_RC4_NO_SALT_FLAG: u32 = 1073741824u32;
pub const CMSG_RECIPIENT_COUNT_PARAM: u32 = 17u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_RECIPIENT_ENCODE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_RECIPIENT_ENCRYPTED_KEY_INFO(i32);
pub const CMSG_RECIPIENT_INDEX_PARAM: u32 = 18u32;
pub const CMSG_RECIPIENT_INFO_PARAM: u32 = 19u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO(i32);
pub const CMSG_SIGNED_DATA_CMS_VERSION: u32 = 3u32;
pub const CMSG_SIGNED_DATA_NO_SIGN_FLAG: u32 = 128u32;
pub const CMSG_SIGNED_DATA_PKCS_1_5_VERSION: u32 = 1u32;
pub const CMSG_SIGNED_DATA_V1: u32 = 1u32;
pub const CMSG_SIGNED_DATA_V3: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_SIGNED_ENCODE_INFO(i32);
pub const CMSG_SIGNER_AUTH_ATTR_PARAM: u32 = 9u32;
pub const CMSG_SIGNER_CERT_ID_PARAM: u32 = 38u32;
pub const CMSG_SIGNER_CERT_INFO_PARAM: u32 = 7u32;
pub const CMSG_SIGNER_COUNT_PARAM: u32 = 5u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_SIGNER_ENCODE_INFO(i32);
pub const CMSG_SIGNER_HASH_ALGORITHM_PARAM: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_SIGNER_INFO(i32);
pub const CMSG_SIGNER_INFO_CMS_VERSION: u32 = 3u32;
pub const CMSG_SIGNER_INFO_PARAM: u32 = 6u32;
pub const CMSG_SIGNER_INFO_PKCS_1_5_VERSION: u32 = 1u32;
pub const CMSG_SIGNER_INFO_V1: u32 = 1u32;
pub const CMSG_SIGNER_INFO_V3: u32 = 3u32;
pub const CMSG_SIGNER_ONLY_FLAG: u32 = 2u32;
pub const CMSG_SIGNER_UNAUTH_ATTR_PARAM: u32 = 10u32;
pub struct CMSG_SP3_COMPATIBLE_AUX_INFO(i32);
pub const CMSG_SP3_COMPATIBLE_ENCRYPT_FLAG: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMSG_STREAM_INFO(i32);
pub const CMSG_TRUSTED_SIGNER_FLAG: u32 = 1u32;
pub const CMSG_TYPE_PARAM: u32 = 1u32;
pub const CMSG_UNPROTECTED_ATTR_PARAM: u32 = 37u32;
pub const CMSG_USE_SIGNER_INDEX_FLAG: u32 = 4u32;
pub const CMSG_VERIFY_COUNTER_SIGN_ENABLE_STRONG_FLAG: u32 = 1u32;
pub const CMSG_VERIFY_SIGNER_CERT: u32 = 2u32;
pub const CMSG_VERIFY_SIGNER_CHAIN: u32 = 3u32;
pub const CMSG_VERIFY_SIGNER_NULL: u32 = 4u32;
pub const CMSG_VERIFY_SIGNER_PUBKEY: u32 = 1u32;
pub const CMSG_VERSION_PARAM: u32 = 30u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CMS_DH_KEY_INFO(i32);
pub struct CMS_KEY_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CPS_URLS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_CONTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_DIST_POINT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_DIST_POINTS_INFO(i32);
pub const CRL_DIST_POINT_ERR_CRL_ISSUER_BIT: i32 = -2147483648i32;
pub const CRL_DIST_POINT_ERR_INDEX_MASK: u32 = 127u32;
pub const CRL_DIST_POINT_ERR_INDEX_SHIFT: u32 = 24u32;
pub const CRL_DIST_POINT_FULL_NAME: u32 = 1u32;
pub const CRL_DIST_POINT_ISSUER_RDN_NAME: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_DIST_POINT_NAME(i32);
pub const CRL_DIST_POINT_NO_NAME: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_ENTRY(i32);
pub const CRL_FIND_ANY: u32 = 0u32;
pub const CRL_FIND_EXISTING: u32 = 2u32;
pub const CRL_FIND_ISSUED_BY: u32 = 1u32;
pub const CRL_FIND_ISSUED_BY_AKI_FLAG: u32 = 1u32;
pub const CRL_FIND_ISSUED_BY_BASE_FLAG: u32 = 8u32;
pub const CRL_FIND_ISSUED_BY_DELTA_FLAG: u32 = 4u32;
pub const CRL_FIND_ISSUED_BY_SIGNATURE_FLAG: u32 = 2u32;
pub const CRL_FIND_ISSUED_FOR: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_FIND_ISSUED_FOR_PARA(i32);
pub const CRL_FIND_ISSUED_FOR_SET_STRONG_PROPERTIES_FLAG: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_ISSUING_DIST_POINT(i32);
pub const CRL_REASON_AA_COMPROMISE: u32 = 10u32;
pub const CRL_REASON_AA_COMPROMISE_FLAG: u32 = 128u32;
pub const CRL_REASON_AFFILIATION_CHANGED_FLAG: u32 = 16u32;
pub const CRL_REASON_CA_COMPROMISE_FLAG: u32 = 32u32;
pub const CRL_REASON_CERTIFICATE_HOLD_FLAG: u32 = 2u32;
pub const CRL_REASON_CESSATION_OF_OPERATION_FLAG: u32 = 4u32;
pub const CRL_REASON_KEY_COMPROMISE_FLAG: u32 = 64u32;
pub const CRL_REASON_PRIVILEGE_WITHDRAWN: u32 = 9u32;
pub const CRL_REASON_PRIVILEGE_WITHDRAWN_FLAG: u32 = 1u32;
pub const CRL_REASON_SUPERSEDED_FLAG: u32 = 8u32;
pub const CRL_REASON_UNUSED_FLAG: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRL_REVOCATION_INFO(i32);
pub const CRL_V1: u32 = 0u32;
pub const CRL_V2: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CROSS_CERT_DIST_POINTS_INFO(i32);
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_MASK: u32 = 255u32;
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_SHIFT: u32 = 24u32;
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_DEFAULT: u32 = 50u32;
pub const CRYPTNET_CRL_BEFORE_OCSP_ENABLE: u32 = 4294967295u32;
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_DEFAULT: u32 = 500u32;
pub const CRYPTNET_OCSP_AFTER_CRL_DISABLE: u32 = 4294967295u32;
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_DEFAULT: u32 = 10u32;
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: u32 = 20u32;
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_DEFAULT: u32 = 60u32;
pub const CRYPTNET_PRE_FETCH_TRIGGER_DISABLE: u32 = 4294967295u32;
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: u32 = 10u32;
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH: u32 = 0u32;
pub const CRYPTNET_URL_CACHE_DISABLE_FLUSH: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTNET_URL_CACHE_FLUSH_INFO(i32);
pub const CRYPTNET_URL_CACHE_PRE_FETCH_AUTOROOT_CAB: u32 = 5u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_BLOB: u32 = 1u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_CRL: u32 = 2u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_DISALLOWED_CERT_CAB: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTNET_URL_CACHE_PRE_FETCH_INFO(i32);
pub const CRYPTNET_URL_CACHE_PRE_FETCH_NONE: u32 = 0u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_OCSP: u32 = 3u32;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_PIN_RULES_CAB: u32 = 7u32;
pub const CRYPTNET_URL_CACHE_RESPONSE_HTTP: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTNET_URL_CACHE_RESPONSE_INFO(i32);
pub const CRYPTNET_URL_CACHE_RESPONSE_NONE: u32 = 0u32;
pub const CRYPTNET_URL_CACHE_RESPONSE_VALIDATED: u32 = 32768u32;
pub struct CRYPTOAPI_BLOB(i32);
pub const CRYPTPROTECTMEMORY_BLOCK_SIZE: u32 = 16u32;
pub const CRYPTPROTECTMEMORY_CROSS_PROCESS: u32 = 1u32;
pub const CRYPTPROTECTMEMORY_SAME_LOGON: u32 = 2u32;
pub const CRYPTPROTECTMEMORY_SAME_PROCESS: u32 = 0u32;
pub const CRYPTPROTECT_AUDIT: u32 = 16u32;
pub const CRYPTPROTECT_CRED_REGENERATE: u32 = 128u32;
pub const CRYPTPROTECT_CRED_SYNC: u32 = 8u32;
pub const CRYPTPROTECT_FIRST_RESERVED_FLAGVAL: u32 = 268435455u32;
pub const CRYPTPROTECT_LAST_RESERVED_FLAGVAL: u32 = 4294967295u32;
pub const CRYPTPROTECT_LOCAL_MACHINE: u32 = 4u32;
pub const CRYPTPROTECT_NO_RECOVERY: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTPROTECT_PROMPTSTRUCT(i32);
pub const CRYPTPROTECT_PROMPT_ON_PROTECT: u32 = 2u32;
pub const CRYPTPROTECT_PROMPT_ON_UNPROTECT: u32 = 1u32;
pub const CRYPTPROTECT_PROMPT_REQUIRE_STRONG: u32 = 16u32;
pub const CRYPTPROTECT_PROMPT_RESERVED: u32 = 4u32;
pub const CRYPTPROTECT_PROMPT_STRONG: u32 = 8u32;
pub const CRYPTPROTECT_UI_FORBIDDEN: u32 = 1u32;
pub const CRYPTPROTECT_VERIFY_PROTECTION: u32 = 64u32;
pub struct CRYPT_3DES_KEY_STATE(i32);
pub const CRYPT_ACCUMULATIVE_TIMEOUT: u32 = 2048u32;
pub const CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG: u32 = 65536u32;
pub struct CRYPT_ACQUIRE_FLAGS(i32);
pub const CRYPT_ACQUIRE_NCRYPT_KEY_FLAGS_MASK: u32 = 458752u32;
pub const CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG: u32 = 262144u32;
pub const CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG: u32 = 131072u32;
pub const CRYPT_ACQUIRE_WINDOW_HANDLE_FLAG: u32 = 128u32;
pub struct CRYPT_AES_128_KEY_STATE(i32);
pub struct CRYPT_AES_256_KEY_STATE(i32);
pub const CRYPT_AIA_RETRIEVAL: u32 = 524288u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ALGORITHM_IDENTIFIER(i32);
pub const CRYPT_ARCHIVE: u32 = 256u32;
pub const CRYPT_ASN_ENCODING: u32 = 1u32;
pub const CRYPT_ASYNC_RETRIEVAL: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ASYNC_RETRIEVAL_COMPLETION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ATTRIBUTE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ATTRIBUTES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ATTRIBUTE_TYPE_VALUE(i32);
pub struct CRYPT_BIT_BLOB(i32);
pub struct CRYPT_BLOB_ARRAY(i32);
pub const CRYPT_CACHE_ONLY_RETRIEVAL: u32 = 2u32;
pub const CRYPT_CHECK_FRESHNESS_TIME_VALIDITY: u32 = 1024u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_CONTENT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_CONTEXTS(i32);
pub struct CRYPT_CONTEXT_CONFIG(i32);
pub struct CRYPT_CONTEXT_CONFIG_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_CONTEXT_FUNCTIONS(i32);
pub struct CRYPT_CONTEXT_FUNCTION_CONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_CONTEXT_FUNCTION_PROVIDERS(i32);
pub const CRYPT_CREATE_NEW_FLUSH_ENTRY: u32 = 268435456u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_CREDENTIALS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_CSP_PROVIDER(i32);
pub const CRYPT_DECODE_ALLOC_FLAG: u32 = 32768u32;
pub const CRYPT_DECODE_ENABLE_PUNYCODE_FLAG: u32 = 33554432u32;
pub const CRYPT_DECODE_ENABLE_UTF8PERCENT_FLAG: u32 = 67108864u32;
pub const CRYPT_DECODE_NOCOPY_FLAG: u32 = 1u32;
pub const CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 8u32;
pub struct CRYPT_DECODE_PARA(i32);
pub const CRYPT_DECODE_SHARE_OID_STRING_FLAG: u32 = 4u32;
pub const CRYPT_DECODE_TO_BE_SIGNED_FLAG: u32 = 2u32;
pub const CRYPT_DECRYPT: u32 = 2u32;
pub struct CRYPT_DECRYPT_MESSAGE_PARA(i32);
pub const CRYPT_DECRYPT_RSA_NO_PADDING_CHECK: u32 = 32u32;
pub const CRYPT_DEFAULT_CONTAINER_OPTIONAL: u32 = 128u32;
pub struct CRYPT_DEFAULT_CONTEXT_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA(i32);
pub struct CRYPT_DEFAULT_CONTEXT_TYPE(i32);
pub const CRYPT_DELETEKEYSET: u32 = 16u32;
pub const CRYPT_DELETE_DEFAULT: u32 = 4u32;
pub const CRYPT_DELETE_KEYSET: u32 = 16u32;
pub struct CRYPT_DES_KEY_STATE(i32);
pub const CRYPT_DONT_CACHE_RESULT: u32 = 8u32;
pub const CRYPT_DONT_CHECK_TIME_VALIDITY: u32 = 512u32;
pub const CRYPT_DONT_VERIFY_SIGNATURE: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ECC_CMS_SHARED_INFO(i32);
pub const CRYPT_ECC_CMS_SHARED_INFO_SUPPPUBINFO_BYTE_LENGTH: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ECC_PRIVATE_KEY_INFO(i32);
pub const CRYPT_ECC_PRIVATE_KEY_INFO_v1: u32 = 1u32;
pub const CRYPT_ENABLE_FILE_RETRIEVAL: u32 = 134217728u32;
pub const CRYPT_ENABLE_SSL_REVOCATION_RETRIEVAL: u32 = 8388608u32;
pub const CRYPT_ENCODE_DECODE_NONE: u32 = 0u32;
pub const CRYPT_ENCODE_ENABLE_UTF8PERCENT_FLAG: u32 = 262144u32;
pub const CRYPT_ENCODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 8u32;
pub struct CRYPT_ENCODE_OBJECT_FLAGS(i32);
pub struct CRYPT_ENCODE_PARA(i32);
pub const CRYPT_ENCRYPT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ENCRYPTED_PRIVATE_KEY_INFO(i32);
pub const CRYPT_ENCRYPT_ALG_OID_GROUP_ID: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ENCRYPT_MESSAGE_PARA(i32);
pub const CRYPT_ENHKEY_USAGE_OID_GROUP_ID: u32 = 7u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_ENROLLMENT_NAME_VALUE_PAIR(i32);
pub const CRYPT_EXPORT: u32 = 4u32;
pub const CRYPT_EXPORT_KEY: u32 = 64u32;
pub const CRYPT_EXT_OR_ATTR_OID_GROUP_ID: u32 = 6u32;
pub const CRYPT_FAILED: u32 = 0u32;
pub const CRYPT_FASTSGC: u32 = 2u32;
pub struct CRYPT_FIND_FLAGS(i32);
pub const CRYPT_FIRST: u32 = 1u32;
pub const CRYPT_FIRST_ALG_OID_GROUP_ID: u32 = 1u32;
pub const CRYPT_FLAG_IPSEC: u32 = 16u32;
pub const CRYPT_FLAG_PCT1: u32 = 1u32;
pub const CRYPT_FLAG_SIGNING: u32 = 32u32;
pub const CRYPT_FLAG_SSL2: u32 = 2u32;
pub const CRYPT_FLAG_SSL3: u32 = 4u32;
pub const CRYPT_FLAG_TLS1: u32 = 8u32;
pub const CRYPT_FORMAT_COMMA: u32 = 4096u32;
pub const CRYPT_FORMAT_CRLF: u32 = 512u32;
pub const CRYPT_FORMAT_OID: u32 = 4u32;
pub const CRYPT_FORMAT_RDN_CRLF: u32 = 512u32;
pub const CRYPT_FORMAT_RDN_REVERSE: u32 = 2048u32;
pub const CRYPT_FORMAT_RDN_SEMICOLON: u32 = 256u32;
pub const CRYPT_FORMAT_RDN_UNQUOTE: u32 = 1024u32;
pub const CRYPT_FORMAT_SEMICOLON: u32 = 256u32;
pub const CRYPT_FORMAT_SIMPLE: u32 = 1u32;
pub const CRYPT_FORMAT_STR_MULTI_LINE: u32 = 1u32;
pub const CRYPT_FORMAT_STR_NO_HEX: u32 = 16u32;
pub const CRYPT_FORMAT_X509: u32 = 2u32;
pub const CRYPT_GET_INSTALLED_OID_FUNC_FLAG: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO(i32);
pub struct CRYPT_GET_URL_FLAGS(i32);
pub const CRYPT_HASH_ALG_OID_GROUP_ID: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_HASH_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_HASH_MESSAGE_PARA(i32);
pub const CRYPT_HTTP_POST_RETRIEVAL: u32 = 1048576u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_IMAGE_REF(i32);
pub struct CRYPT_IMAGE_REF_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_IMAGE_REG(i32);
pub const CRYPT_IMPL_HARDWARE: u32 = 1u32;
pub const CRYPT_IMPL_MIXED: u32 = 3u32;
pub const CRYPT_IMPL_REMOVABLE: u32 = 8u32;
pub const CRYPT_IMPL_SOFTWARE: u32 = 2u32;
pub const CRYPT_IMPL_UNKNOWN: u32 = 4u32;
pub const CRYPT_IMPORT_KEY: u32 = 128u32;
pub struct CRYPT_IMPORT_PUBLIC_KEY_FLAGS(i32);
pub const CRYPT_INSTALL_OID_FUNC_BEFORE_FLAG: u32 = 1u32;
pub const CRYPT_INSTALL_OID_INFO_BEFORE_FLAG: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_INTERFACE_REG(i32);
pub const CRYPT_KDF_OID_GROUP_ID: u32 = 10u32;
pub const CRYPT_KEEP_TIME_VALID: u32 = 128u32;
pub const CRYPT_KEYID_ALLOC_FLAG: u32 = 32768u32;
pub const CRYPT_KEYID_DELETE_FLAG: u32 = 16u32;
pub const CRYPT_KEYID_MACHINE_FLAG: u32 = 32u32;
pub const CRYPT_KEYID_SET_NEW_FLAG: u32 = 8192u32;
pub struct CRYPT_KEY_FLAGS(i32);
pub struct CRYPT_KEY_PARAM_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_KEY_PROV_INFO(i32);
pub struct CRYPT_KEY_PROV_PARAM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_KEY_SIGN_MESSAGE_PARA(i32);
pub struct CRYPT_KEY_VERIFY_MESSAGE_PARA(i32);
pub const CRYPT_LAST_ALG_OID_GROUP_ID: u32 = 4u32;
pub const CRYPT_LAST_OID_GROUP_ID: u32 = 10u32;
pub const CRYPT_LDAP_AREC_EXCLUSIVE_RETRIEVAL: u32 = 262144u32;
pub const CRYPT_LDAP_INSERT_ENTRY_ATTRIBUTE: u32 = 32768u32;
pub const CRYPT_LDAP_SCOPE_BASE_ONLY_RETRIEVAL: u32 = 8192u32;
pub const CRYPT_LDAP_SIGN_RETRIEVAL: u32 = 65536u32;
pub const CRYPT_LITTLE_ENDIAN: u32 = 1u32;
pub const CRYPT_LOCALIZED_NAME_ENCODING_TYPE: u32 = 0u32;
pub const CRYPT_MAC: u32 = 32u32;
pub const CRYPT_MACHINE_DEFAULT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_MASK_GEN_ALGORITHM(i32);
pub const CRYPT_MATCH_ANY_ENCODING_TYPE: u32 = 4294967295u32;
pub const CRYPT_MESSAGE_BARE_CONTENT_OUT_FLAG: u32 = 1u32;
pub const CRYPT_MESSAGE_ENCAPSULATED_CONTENT_OUT_FLAG: u32 = 2u32;
pub const CRYPT_MESSAGE_KEYID_RECIPIENT_FLAG: u32 = 4u32;
pub const CRYPT_MESSAGE_KEYID_SIGNER_FLAG: u32 = 4u32;
pub const CRYPT_MESSAGE_SILENT_KEYSET_FLAG: u32 = 64u32;
pub const CRYPT_MODE_CBC: u32 = 1u32;
pub const CRYPT_MODE_CBCI: u32 = 6u32;
pub const CRYPT_MODE_CBCOFM: u32 = 9u32;
pub const CRYPT_MODE_CBCOFMI: u32 = 10u32;
pub const CRYPT_MODE_CFB: u32 = 4u32;
pub const CRYPT_MODE_CFBP: u32 = 7u32;
pub const CRYPT_MODE_CTS: u32 = 5u32;
pub const CRYPT_MODE_ECB: u32 = 2u32;
pub const CRYPT_MODE_OFB: u32 = 3u32;
pub const CRYPT_MODE_OFBP: u32 = 8u32;
pub struct CRYPT_MSG_TYPE(i32);
pub const CRYPT_NDR_ENCODING: u32 = 2u32;
pub const CRYPT_NEWKEYSET: u32 = 8u32;
pub const CRYPT_NEXT: u32 = 2u32;
pub const CRYPT_NOHASHOID: u32 = 1u32;
pub const CRYPT_NOT_MODIFIED_RETRIEVAL: u32 = 4194304u32;
pub const CRYPT_NO_AUTH_RETRIEVAL: u32 = 131072u32;
pub const CRYPT_NO_OCSP_FAILOVER_TO_CRL_RETRIEVAL: u32 = 33554432u32;
pub const CRYPT_OBJECT_LOCATOR_FIRST_RESERVED_USER_NAME_TYPE: u32 = 33u32;
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_NAME_TYPE: u32 = 32u32;
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_USER_NAME_TYPE: u32 = 65535u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE(i32);
pub struct CRYPT_OBJECT_LOCATOR_RELEASE_REASON(i32);
pub const CRYPT_OBJECT_LOCATOR_SPN_NAME_TYPE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_OBJID_TABLE(i32);
pub const CRYPT_OCSP_ONLY_RETRIEVAL: u32 = 16777216u32;
pub const CRYPT_OFFLINE_CHECK_RETRIEVAL: u32 = 16384u32;
pub const CRYPT_OID_DISABLE_SEARCH_DS_FLAG: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_OID_FUNC_ENTRY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_OID_INFO(i32);
pub const CRYPT_OID_INFO_ALGID_KEY: u32 = 3u32;
pub const CRYPT_OID_INFO_CNG_ALGID_KEY: u32 = 5u32;
pub const CRYPT_OID_INFO_CNG_SIGN_KEY: u32 = 6u32;
pub const CRYPT_OID_INFO_NAME_KEY: u32 = 2u32;
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: u32 = 268369920u32;
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: u32 = 16u32;
pub const CRYPT_OID_INFO_OID_KEY: u32 = 1u32;
pub const CRYPT_OID_INFO_OID_KEY_FLAGS_MASK: u32 = 4294901760u32;
pub const CRYPT_OID_INFO_SIGN_KEY: u32 = 4u32;
pub const CRYPT_OID_INHIBIT_SIGNATURE_FORMAT_FLAG: u32 = 1u32;
pub const CRYPT_OID_NO_NULL_ALGORITHM_PARA_FLAG: u32 = 4u32;
pub const CRYPT_OID_PREFER_CNG_ALGID_FLAG: u32 = 1073741824u32;
pub const CRYPT_OID_PUBKEY_ENCRYPT_ONLY_FLAG: u32 = 1073741824u32;
pub const CRYPT_OID_PUBKEY_SIGN_ONLY_FLAG: u32 = 2147483648u32;
pub const CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: u32 = 536870912u32;
pub const CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: u32 = 268435456u32;
pub const CRYPT_OID_USE_PUBKEY_PARA_FOR_PKCS7_FLAG: u32 = 2u32;
pub const CRYPT_OVERWRITE: u32 = 1u32;
pub const CRYPT_OWF_REPL_LM_HASH: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PASSWORD_CREDENTIALSA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PASSWORD_CREDENTIALSW(i32);
pub struct CRYPT_PKCS12_PBE_PARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PKCS8_EXPORT_PARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PKCS8_IMPORT_PARAMS(i32);
pub const CRYPT_POLICY_OID_GROUP_ID: u32 = 8u32;
pub const CRYPT_PRIORITY_BOTTOM: u32 = 4294967295u32;
pub const CRYPT_PRIORITY_TOP: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PRIVATE_KEY_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROPERTY_REF(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROVIDERS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROVIDER_REF(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROVIDER_REFS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROVIDER_REG(i32);
pub const CRYPT_PROXY_CACHE_RETRIEVAL: u32 = 2097152u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PSOURCE_ALGORITHM(i32);
pub const CRYPT_PSTORE: u32 = 2u32;
pub const CRYPT_PUBKEY_ALG_OID_GROUP_ID: u32 = 3u32;
pub const CRYPT_RANDOM_QUERY_STRING_RETRIEVAL: u32 = 67108864u32;
pub const CRYPT_RC2_128BIT_VERSION: u32 = 58u32;
pub const CRYPT_RC2_40BIT_VERSION: u32 = 160u32;
pub const CRYPT_RC2_56BIT_VERSION: u32 = 52u32;
pub const CRYPT_RC2_64BIT_VERSION: u32 = 120u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_RC2_CBC_PARAMETERS(i32);
pub struct CRYPT_RC4_KEY_STATE(i32);
pub const CRYPT_RDN_ATTR_OID_GROUP_ID: u32 = 5u32;
pub const CRYPT_READ: u32 = 8u32;
pub const CRYPT_REGISTER_FIRST_INDEX: u32 = 0u32;
pub const CRYPT_REGISTER_LAST_INDEX: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_RETRIEVE_AUX_INFO(i32);
pub const CRYPT_RETRIEVE_MAX_ERROR_CONTENT_LENGTH: u32 = 4096u32;
pub const CRYPT_RETRIEVE_MULTIPLE_OBJECTS: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_RSAES_OAEP_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_RSA_SSA_PSS_PARAMETERS(i32);
pub const CRYPT_SECRETDIGEST: u32 = 1u32;
pub const CRYPT_SEC_DESCR: u32 = 1u32;
pub struct CRYPT_SEQUENCE_OF_ANY(i32);
pub const CRYPT_SERVER: u32 = 1024u32;
pub struct CRYPT_SET_HASH_PARAM(i32);
pub struct CRYPT_SET_PROV_PARAM_ID(i32);
pub const CRYPT_SGC: u32 = 1u32;
pub const CRYPT_SGC_ENUM: u32 = 4u32;
pub const CRYPT_SIGN_ALG_OID_GROUP_ID: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_SIGN_MESSAGE_PARA(i32);
pub const CRYPT_SILENT: u32 = 64u32;
pub struct CRYPT_SMART_CARD_ROOT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_SMIME_CAPABILITIES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_SMIME_CAPABILITY(i32);
pub const CRYPT_SORTED_CTL_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 65536u32;
pub const CRYPT_STICKY_CACHE_RETRIEVAL: u32 = 4096u32;
pub struct CRYPT_STRING(i32);
pub const CRYPT_STRING_BASE64URI: u32 = 13u32;
pub const CRYPT_STRING_ENCODEMASK: u32 = 255u32;
pub const CRYPT_STRING_HASHDATA: u32 = 268435456u32;
pub const CRYPT_STRING_NOCR: u32 = 2147483648u32;
pub const CRYPT_STRING_NOCRLF: u32 = 1073741824u32;
pub const CRYPT_STRING_PERCENTESCAPE: u32 = 134217728u32;
pub const CRYPT_STRING_RESERVED100: u32 = 256u32;
pub const CRYPT_STRING_RESERVED200: u32 = 512u32;
pub const CRYPT_SUCCEED: u32 = 1u32;
pub const CRYPT_TEMPLATE_OID_GROUP_ID: u32 = 9u32;
pub struct CRYPT_TIMESTAMP_ACCURACY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_CONTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_PARA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIMESTAMP_RESPONSE(i32);
pub struct CRYPT_TIMESTAMP_RESPONSE_STATUS(i32);
pub struct CRYPT_TIMESTAMP_VERSION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TIME_STAMP_REQUEST_INFO(i32);
pub const CRYPT_TYPE2_FORMAT: u32 = 2u32;
pub const CRYPT_UI_PROMPT: u32 = 4u32;
pub const CRYPT_UNICODE_NAME_DECODE_DISABLE_IE4_UTF8_FLAG: u32 = 16777216u32;
pub const CRYPT_UNICODE_NAME_ENCODE_FORCE_UTF8_UNICODE_FLAG: u32 = 268435456u32;
pub const CRYPT_UPDATE_KEY: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_URL_ARRAY(i32);
pub struct CRYPT_URL_INFO(i32);
pub const CRYPT_USERDATA: u32 = 1u32;
pub const CRYPT_USER_DEFAULT: u32 = 2u32;
pub const CRYPT_USER_PROTECTED_STRONG: u32 = 1048576u32;
pub const CRYPT_VERIFYCONTEXT: u32 = 4026531840u32;
pub struct CRYPT_VERIFY_CERT_FLAGS(i32);
pub const CRYPT_VERIFY_CERT_SIGN_CHECK_WEAK_HASH_FLAG: u32 = 8u32;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CERT: u32 = 2u32;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CHAIN: u32 = 3u32;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_NULL: u32 = 4u32;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_PUBKEY: u32 = 1u32;
pub struct CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO(i32);
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_BLOB: u32 = 1u32;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CERT: u32 = 2u32;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CRL: u32 = 3u32;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_OCSP_BASIC_SIGNED_RESPONSE: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO(i32);
pub const CRYPT_VERIFY_CONTEXT_SIGNATURE: u32 = 32u32;
pub const CRYPT_VERIFY_DATA_HASH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_VERIFY_MESSAGE_PARA(i32);
pub const CRYPT_WIRE_ONLY_RETRIEVAL: u32 = 4u32;
pub const CRYPT_WRITE: u32 = 16u32;
pub const CRYPT_X931_FORMAT: u32 = 4u32;
pub const CRYPT_X942_COUNTER_BYTE_LENGTH: u32 = 4u32;
pub const CRYPT_X942_KEY_LENGTH_BYTE_LENGTH: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_X942_OTHER_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_ALGORITHM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_ALGORITHM_INFO(i32);
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_ALGID: u32 = 3u32;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_SIGN_ALGID: u32 = 4u32;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_NAME: u32 = 2u32;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_URI: u32 = 1u32;
pub struct CRYPT_XML_BLOB(i32);
pub const CRYPT_XML_BLOB_MAX: u32 = 2147483640u32;
pub struct CRYPT_XML_CHARSET(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_CRYPTOGRAPHIC_INTERFACE(i32);
pub struct CRYPT_XML_DATA_BLOB(i32);
pub struct CRYPT_XML_DATA_PROVIDER(i32);
pub const CRYPT_XML_DIGEST_REFERENCE_DATA_TRANSFORMED: u32 = 1u32;
pub const CRYPT_XML_DIGEST_VALUE_MAX: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_DOC_CTXT(i32);
pub const CRYPT_XML_E_ALGORITHM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885372i32 as _);
pub const CRYPT_XML_E_BASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885376i32 as _);
pub const CRYPT_XML_E_ENCODING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885373i32 as _);
pub const CRYPT_XML_E_HANDLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885370i32 as _);
pub const CRYPT_XML_E_HASH_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885365i32 as _);
pub const CRYPT_XML_E_INVALID_DIGEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885367i32 as _);
pub const CRYPT_XML_E_INVALID_KEYVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885361i32 as _);
pub const CRYPT_XML_E_INVALID_SIGNATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885366i32 as _);
pub const CRYPT_XML_E_LARGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885375i32 as _);
pub const CRYPT_XML_E_LAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885358i32 as _);
pub const CRYPT_XML_E_NON_UNIQUE_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885358i32 as _);
pub const CRYPT_XML_E_OPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885369i32 as _);
pub const CRYPT_XML_E_SIGNER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885359i32 as _);
pub const CRYPT_XML_E_SIGN_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885364i32 as _);
pub const CRYPT_XML_E_TOO_MANY_SIGNATURES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885362i32 as _);
pub const CRYPT_XML_E_TOO_MANY_TRANSFORMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885374i32 as _);
pub const CRYPT_XML_E_TRANSFORM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885371i32 as _);
pub const CRYPT_XML_E_UNEXPECTED_XML: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885360i32 as _);
pub const CRYPT_XML_E_UNRESOLVED_REFERENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885368i32 as _);
pub const CRYPT_XML_E_VERIFY_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146885363i32 as _);
pub struct CRYPT_XML_FLAGS(i32);
pub const CRYPT_XML_FLAG_ADD_OBJECT_CREATE_COPY: u32 = 1u32;
pub const CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT: u32 = 1073741824u32;
pub const CRYPT_XML_FLAG_CREATE_REFERENCE_AS_OBJECT: u32 = 1u32;
pub const CRYPT_XML_FLAG_ECDSA_DSIG11: u32 = 67108864u32;
pub const CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT: u32 = 134217728u32;
pub const CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT: u32 = 536870912u32;
pub struct CRYPT_XML_GROUP_ID(i32);
pub const CRYPT_XML_GROUP_ID_HASH: u32 = 1u32;
pub const CRYPT_XML_GROUP_ID_SIGN: u32 = 2u32;
pub const CRYPT_XML_ID_MAX: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_ISSUER_SERIAL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_KEYINFO_PARAM(i32);
pub struct CRYPT_XML_KEYINFO_SPEC(i32);
pub struct CRYPT_XML_KEYINFO_TYPE(i32);
pub struct CRYPT_XML_KEY_DSA_KEY_VALUE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_KEY_ECDSA_KEY_VALUE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_KEY_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_KEY_INFO_ITEM(i32);
pub struct CRYPT_XML_KEY_RSA_KEY_VALUE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_KEY_VALUE(i32);
pub struct CRYPT_XML_KEY_VALUE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_OBJECT(i32);
pub const CRYPT_XML_OBJECTS_MAX: u32 = 256u32;
pub struct CRYPT_XML_PROPERTY(i32);
pub struct CRYPT_XML_PROPERTY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_REFERENCE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_REFERENCES(i32);
pub const CRYPT_XML_REFERENCES_MAX: u32 = 32760u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_SIGNATURE(i32);
pub const CRYPT_XML_SIGNATURES_MAX: u32 = 16u32;
pub const CRYPT_XML_SIGNATURE_VALUE_MAX: u32 = 2048u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_SIGNED_INFO(i32);
pub struct CRYPT_XML_STATUS(i32);
pub struct CRYPT_XML_STATUS_ERROR_STATUS(i32);
pub struct CRYPT_XML_STATUS_INFO_STATUS(i32);
pub const CRYPT_XML_STATUS_NO_ERROR: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_TRANSFORM_CHAIN_CONFIG(i32);
pub struct CRYPT_XML_TRANSFORM_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_TRANSFORM_INFO(i32);
pub const CRYPT_XML_TRANSFORM_MAX: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_X509DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_XML_X509DATA_ITEM(i32);
pub struct CRYPT_XML_X509DATA_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_ANY_SUBJECT_INFO(i32);
pub const CTL_ANY_SUBJECT_TYPE: u32 = 1u32;
pub const CTL_CERT_SUBJECT_TYPE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_CONTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_ENTRY(i32);
pub const CTL_ENTRY_FROM_PROP_CHAIN_FLAG: u32 = 1u32;
pub const CTL_FIND_NO_LIST_ID_CBDATA: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_FIND_SUBJECT_PARA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_FIND_USAGE_PARA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_USAGE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_USAGE_MATCH(i32);
pub const CTL_V1: u32 = 0u32;
pub struct CTL_VERIFY_USAGE_PARA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_VERIFY_USAGE_STATUS(i32);
pub const CUR_BLOB_VERSION: u32 = 2u32;
pub struct CertKeyType(i32);
pub struct CryptXmlDllCloseDigest(i32);
pub struct CryptXmlDllCreateDigest(i32);
pub struct CryptXmlDllCreateKey(i32);
pub struct CryptXmlDllDigestData(i32);
pub struct CryptXmlDllEncodeAlgorithm(i32);
pub struct CryptXmlDllEncodeKeyValue(i32);
pub struct CryptXmlDllFinalizeDigest(i32);
pub struct CryptXmlDllGetAlgorithmInfo(i32);
pub struct CryptXmlDllGetInterface(i32);
pub struct CryptXmlDllSignData(i32);
pub struct CryptXmlDllVerifySignature(i32);
pub struct DSAFIPSVERSION_ENUM(i32);
pub struct DSSSEED(i32);
pub struct Direction(i32);
pub struct ECC_CURVE_ALG_ID_ENUM(i32);
pub struct ECC_CURVE_TYPE_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ENDPOINTADDRESS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ENDPOINTADDRESS2(i32);
pub struct EV_EXTRA_CERT_CHAIN_POLICY_PARA(i32);
pub struct EV_EXTRA_CERT_CHAIN_POLICY_STATUS(i32);
pub const EXPORT_PRIVATE_KEYS: u32 = 4u32;
pub const E_ICARD_ARGUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413883i32 as _);
pub const E_ICARD_COMMUNICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413888i32 as _);
pub const E_ICARD_DATA_ACCESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413887i32 as _);
pub const E_ICARD_EXPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413886i32 as _);
pub const E_ICARD_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413867i32 as _);
pub const E_ICARD_FAILED_REQUIRED_CLAIMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413756i32 as _);
pub const E_ICARD_IDENTITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413885i32 as _);
pub const E_ICARD_IMPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413884i32 as _);
pub const E_ICARD_INFORMATIONCARD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413881i32 as _);
pub const E_ICARD_INVALID_PROOF_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413758i32 as _);
pub const E_ICARD_LOGOVALIDATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413879i32 as _);
pub const E_ICARD_MISSING_APPLIESTO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413759i32 as _);
pub const E_ICARD_PASSWORDVALIDATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413878i32 as _);
pub const E_ICARD_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413877i32 as _);
pub const E_ICARD_PROCESSDIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413876i32 as _);
pub const E_ICARD_REFRESH_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413760i32 as _);
pub const E_ICARD_REQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413882i32 as _);
pub const E_ICARD_SERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413874i32 as _);
pub const E_ICARD_SERVICEBUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413875i32 as _);
pub const E_ICARD_SHUTTINGDOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413873i32 as _);
pub const E_ICARD_STOREKEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413880i32 as _);
pub const E_ICARD_STORE_IMPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413868i32 as _);
pub const E_ICARD_TOKENCREATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413872i32 as _);
pub const E_ICARD_TRUSTEXCHANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413871i32 as _);
pub const E_ICARD_UI_INITIALIZATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413862i32 as _);
pub const E_ICARD_UNKNOWN_REFERENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413757i32 as _);
pub const E_ICARD_UNTRUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413870i32 as _);
pub const E_ICARD_USERCANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073413869i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub struct GENERIC_XML_TOKEN(i32);
pub struct HASHALGORITHM_ENUM(i32);
pub struct HCERTCHAINENGINE(i32);
pub struct HCRYPTASYNC(i32);
pub struct HMAC_Info(i32);
pub const HP_ALGID: u32 = 1u32;
pub const HP_HASHSIZE: u32 = 4u32;
pub const HP_TLS1PRF_LABEL: u32 = 6u32;
pub const HP_TLS1PRF_SEED: u32 = 7u32;
pub struct HTTPSPOLICY_CALLBACK_DATA_AUTH_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct HTTPSPolicyCallbackData(i32);
pub struct HandleType(i32);
#[repr(transparent)]
pub struct ICertSrvSetup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertSrvSetupKeyInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertSrvSetupKeyInformationCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateEnrollmentPolicyServerSetup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICertificateEnrollmentServerSetup(pub *mut ::core::ffi::c_void);
pub const IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_1: u32 = 1u32;
pub const IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_2: u32 = 2u32;
pub const IFX_RSA_KEYGEN_VUL_NOT_AFFECTED: u32 = 0u32;
#[repr(transparent)]
pub struct IMSCEPSetup(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub struct INFORMATIONCARD_ASYMMETRIC_CRYPTO_PARAMETERS(i32);
pub struct INFORMATIONCARD_CRYPTO_HANDLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct INFORMATIONCARD_HASH_CRYPTO_PARAMETERS(i32);
pub struct INFORMATIONCARD_SYMMETRIC_CRYPTO_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct INFORMATIONCARD_TRANSFORM_CRYPTO_PARAMETERS(i32);
pub const INTERNATIONAL_USAGE: u32 = 1u32;
pub const KDF_ALGORITHMID: u32 = 8u32;
pub const KDF_CONTEXT: u32 = 14u32;
pub const KDF_GENERIC_PARAMETER: u32 = 17u32;
pub const KDF_HASH_ALGORITHM: u32 = 0u32;
pub const KDF_HKDF_INFO: u32 = 20u32;
pub const KDF_HKDF_SALT: u32 = 19u32;
pub const KDF_HMAC_KEY: u32 = 3u32;
pub const KDF_ITERATION_COUNT: u32 = 16u32;
pub const KDF_KEYBITLENGTH: u32 = 18u32;
pub const KDF_LABEL: u32 = 13u32;
pub const KDF_PARTYUINFO: u32 = 9u32;
pub const KDF_PARTYVINFO: u32 = 10u32;
pub const KDF_SALT: u32 = 15u32;
pub const KDF_SECRET_APPEND: u32 = 2u32;
pub const KDF_SECRET_HANDLE: u32 = 6u32;
pub const KDF_SECRET_PREPEND: u32 = 1u32;
pub const KDF_SUPPPRIVINFO: u32 = 12u32;
pub const KDF_SUPPPUBINFO: u32 = 11u32;
pub const KDF_TLS_PRF_LABEL: u32 = 4u32;
pub const KDF_TLS_PRF_PROTOCOL: u32 = 7u32;
pub const KDF_TLS_PRF_SEED: u32 = 5u32;
pub const KDF_USE_SECRET_AS_HMAC_KEY_FLAG: u32 = 1u32;
pub const KEYSTATEBLOB: u32 = 12u32;
pub const KEY_LENGTH_MASK: u32 = 4294901760u32;
pub struct KEY_TYPE_SUBTYPE(i32);
pub const KP_ADMIN_PIN: u32 = 31u32;
pub const KP_CLEAR_KEY: u32 = 27u32;
pub const KP_CLIENT_RANDOM: u32 = 21u32;
pub const KP_CMS_DH_KEY_INFO: u32 = 38u32;
pub const KP_CMS_KEY_INFO: u32 = 37u32;
pub const KP_EFFECTIVE_KEYLEN: u32 = 19u32;
pub const KP_G: u32 = 12u32;
pub const KP_HIGHEST_VERSION: u32 = 41u32;
pub const KP_INFO: u32 = 18u32;
pub const KP_IV: u32 = 1u32;
pub const KP_KEYEXCHANGE_PIN: u32 = 32u32;
pub const KP_KEYVAL: u32 = 30u32;
pub const KP_MODE: u32 = 4u32;
pub const KP_MODE_BITS: u32 = 5u32;
pub const KP_OAEP_PARAMS: u32 = 36u32;
pub const KP_P: u32 = 11u32;
pub const KP_PADDING: u32 = 3u32;
pub const KP_PIN_ID: u32 = 43u32;
pub const KP_PIN_INFO: u32 = 44u32;
pub const KP_PRECOMP_MD5: u32 = 24u32;
pub const KP_PRECOMP_SHA: u32 = 25u32;
pub const KP_PREHASH: u32 = 34u32;
pub const KP_PUB_EX_LEN: u32 = 28u32;
pub const KP_PUB_EX_VAL: u32 = 29u32;
pub const KP_PUB_PARAMS: u32 = 39u32;
pub const KP_Q: u32 = 13u32;
pub const KP_RA: u32 = 16u32;
pub const KP_RB: u32 = 17u32;
pub const KP_ROUNDS: u32 = 35u32;
pub const KP_RP: u32 = 23u32;
pub const KP_SCHANNEL_ALG: u32 = 20u32;
pub const KP_SERVER_RANDOM: u32 = 22u32;
pub const KP_SIGNATURE_PIN: u32 = 33u32;
pub const KP_VERIFY_PARAMS: u32 = 40u32;
pub const KP_X: u32 = 14u32;
pub const KP_Y: u32 = 15u32;
pub const MAXUIDLEN: u32 = 64u32;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_CHECK_APPLICATION_ROOT_FLAG: u32 = 131072u32;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_DISABLE_FLIGHT_ROOT_FLAG: u32 = 262144u32;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_ENABLE_TEST_ROOT_FLAG: u32 = 65536u32;
pub struct MSCEPSetupProperty(i32);
pub const NCRYPTBUFFER_ATTESTATIONSTATEMENT_BLOB: u32 = 51u32;
pub const NCRYPTBUFFER_ATTESTATION_CLAIM_CHALLENGE_REQUIRED: u32 = 53u32;
pub const NCRYPTBUFFER_ATTESTATION_CLAIM_TYPE: u32 = 52u32;
pub const NCRYPTBUFFER_CERT_BLOB: u32 = 47u32;
pub const NCRYPTBUFFER_CLAIM_IDBINDING_NONCE: u32 = 48u32;
pub const NCRYPTBUFFER_CLAIM_KEYATTESTATION_NONCE: u32 = 49u32;
pub const NCRYPTBUFFER_DATA: u32 = 1u32;
pub const NCRYPTBUFFER_ECC_CURVE_NAME: u32 = 60u32;
pub const NCRYPTBUFFER_ECC_PARAMETERS: u32 = 61u32;
pub const NCRYPTBUFFER_EMPTY: u32 = 0u32;
pub const NCRYPTBUFFER_KEY_PROPERTY_FLAGS: u32 = 50u32;
pub const NCRYPTBUFFER_PKCS_ALG_ID: u32 = 43u32;
pub const NCRYPTBUFFER_PKCS_ALG_OID: u32 = 41u32;
pub const NCRYPTBUFFER_PKCS_ALG_PARAM: u32 = 42u32;
pub const NCRYPTBUFFER_PKCS_ATTRS: u32 = 44u32;
pub const NCRYPTBUFFER_PKCS_KEY_NAME: u32 = 45u32;
pub const NCRYPTBUFFER_PKCS_OID: u32 = 40u32;
pub const NCRYPTBUFFER_PKCS_SECRET: u32 = 46u32;
pub const NCRYPTBUFFER_PROTECTION_DESCRIPTOR_STRING: u32 = 3u32;
pub const NCRYPTBUFFER_PROTECTION_FLAGS: u32 = 4u32;
pub const NCRYPTBUFFER_SSL_CLEAR_KEY: u32 = 23u32;
pub const NCRYPTBUFFER_SSL_CLIENT_RANDOM: u32 = 20u32;
pub const NCRYPTBUFFER_SSL_HIGHEST_VERSION: u32 = 22u32;
pub const NCRYPTBUFFER_SSL_KEY_ARG_DATA: u32 = 24u32;
pub const NCRYPTBUFFER_SSL_SERVER_RANDOM: u32 = 21u32;
pub const NCRYPTBUFFER_SSL_SESSION_HASH: u32 = 25u32;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_NONCE: u32 = 81u32;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_PCR_MASK: u32 = 80u32;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_STATIC_CREATE: u32 = 82u32;
pub const NCRYPTBUFFER_TPM_SEAL_NO_DA_PROTECTION: u32 = 73u32;
pub const NCRYPTBUFFER_TPM_SEAL_PASSWORD: u32 = 70u32;
pub const NCRYPTBUFFER_TPM_SEAL_POLICYINFO: u32 = 71u32;
pub const NCRYPTBUFFER_TPM_SEAL_TICKET: u32 = 72u32;
pub const NCRYPTBUFFER_VERSION: u32 = 0u32;
pub const NCRYPTBUFFER_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS: u32 = 54u32;
pub struct NCRYPT_ALGORITHM_NAME_CLASS(i32);
pub struct NCRYPT_ALLOC_PARA(i32);
pub const NCRYPT_ALLOW_ALL_USAGES: u32 = 16777215u32;
pub const NCRYPT_ALLOW_ARCHIVING_FLAG: u32 = 4u32;
pub const NCRYPT_ALLOW_DECRYPT_FLAG: u32 = 1u32;
pub const NCRYPT_ALLOW_EXPORT_FLAG: u32 = 1u32;
pub const NCRYPT_ALLOW_KEY_AGREEMENT_FLAG: u32 = 4u32;
pub const NCRYPT_ALLOW_KEY_IMPORT_FLAG: u32 = 8u32;
pub const NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: u32 = 8u32;
pub const NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: u32 = 2u32;
pub const NCRYPT_ALLOW_SIGNING_FLAG: u32 = 2u32;
pub const NCRYPT_ALLOW_SILENT_KEY_ACCESS: u32 = 1u32;
pub const NCRYPT_ATTESTATION_FLAG: u32 = 32u32;
pub const NCRYPT_AUTHORITY_KEY_FLAG: u32 = 256u32;
pub const NCRYPT_CIPHER_BLOCK_PADDING_FLAG: u32 = 1u32;
pub const NCRYPT_CIPHER_KEY_BLOB_MAGIC: u32 = 1380470851u32;
pub const NCRYPT_CIPHER_NO_PADDING_FLAG: u32 = 0u32;
pub const NCRYPT_CIPHER_OTHER_PADDING_FLAG: u32 = 2u32;
pub struct NCRYPT_CIPHER_PADDING_INFO(i32);
pub const NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT: u32 = 3u32;
pub const NCRYPT_CLAIM_AUTHORITY_ONLY: u32 = 1u32;
pub const NCRYPT_CLAIM_PLATFORM: u32 = 65536u32;
pub const NCRYPT_CLAIM_SUBJECT_ONLY: u32 = 2u32;
pub const NCRYPT_CLAIM_UNKNOWN: u32 = 4096u32;
pub const NCRYPT_CLAIM_VSM_KEY_ATTESTATION_STATEMENT: u32 = 4u32;
pub const NCRYPT_CLAIM_WEB_AUTH_SUBJECT_ONLY: u32 = 258u32;
pub const NCRYPT_DO_NOT_FINALIZE_FLAG: u32 = 1024u32;
pub struct NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE(i32);
pub struct NCRYPT_EXPORTED_ISOLATED_KEY_HEADER(i32);
pub const NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_V0: u32 = 0u32;
pub const NCRYPT_EXPORT_LEGACY_FLAG: u32 = 2048u32;
pub struct NCRYPT_FLAGS(i32);
pub const NCRYPT_IGNORE_DEVICE_STATE_FLAG: u32 = 4096u32;
pub const NCRYPT_IMPL_HARDWARE_FLAG: u32 = 1u32;
pub const NCRYPT_IMPL_HARDWARE_RNG_FLAG: u32 = 16u32;
pub const NCRYPT_IMPL_REMOVABLE_FLAG: u32 = 8u32;
pub const NCRYPT_IMPL_SOFTWARE_FLAG: u32 = 2u32;
pub const NCRYPT_IMPL_VIRTUAL_ISOLATION_FLAG: u32 = 32u32;
pub struct NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES(i32);
pub const NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_V0: u32 = 0u32;
pub const NCRYPT_ISOLATED_KEY_FLAG_CREATED_IN_ISOLATION: u32 = 1u32;
pub const NCRYPT_ISOLATED_KEY_FLAG_IMPORT_ONLY: u32 = 2u32;
pub const NCRYPT_KDF_KEY_BLOB_MAGIC: u32 = 826688587u32;
pub struct NCRYPT_KEY_ACCESS_POLICY_BLOB(i32);
pub const NCRYPT_KEY_ACCESS_POLICY_VERSION: u32 = 1u32;
pub const NCRYPT_KEY_ATTEST_MAGIC: u32 = 1146110283u32;
pub struct NCRYPT_KEY_ATTEST_PADDING_INFO(i32);
pub struct NCRYPT_KEY_BLOB_HEADER(i32);
pub const NCRYPT_KEY_DERIVATION_INTERFACE: u32 = 7u32;
pub const NCRYPT_KEY_DERIVATION_OPERATION: u32 = 64u32;
pub const NCRYPT_KEY_PROTECTION_INTERFACE: u32 = 65540u32;
pub const NCRYPT_MAX_ALG_ID_LENGTH: u32 = 512u32;
pub const NCRYPT_MAX_KEY_NAME_LENGTH: u32 = 512u32;
pub const NCRYPT_MAX_PROPERTY_DATA: u32 = 1048576u32;
pub const NCRYPT_MAX_PROPERTY_NAME: u32 = 64u32;
pub const NCRYPT_NAMED_DESCRIPTOR_FLAG: u32 = 1u32;
pub const NCRYPT_NO_CACHED_PASSWORD: u32 = 16384u32;
pub struct NCRYPT_OPERATION(i32);
pub const NCRYPT_PAD_CIPHER_FLAG: u32 = 16u32;
pub const NCRYPT_PCP_ENCRYPTION_KEY: u32 = 2u32;
pub const NCRYPT_PCP_HMACVERIFICATION_KEY: u32 = 16u32;
pub struct NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO(i32);
pub const NCRYPT_PCP_IDENTITY_KEY: u32 = 8u32;
pub struct NCRYPT_PCP_RAW_POLICYDIGEST(i32);
pub const NCRYPT_PCP_SIGNATURE_KEY: u32 = 1u32;
pub const NCRYPT_PCP_STORAGE_KEY: u32 = 4u32;
pub struct NCRYPT_PCP_TPM_FW_VERSION_INFO(i32);
pub const NCRYPT_PIN_CACHE_APPLICATION_TICKET_BYTE_LENGTH: u32 = 90u32;
pub const NCRYPT_PIN_CACHE_CLEAR_FOR_CALLING_PROCESS_OPTION: u32 = 1u32;
pub const NCRYPT_PIN_CACHE_DISABLE_DPL_FLAG: u32 = 1u32;
pub const NCRYPT_PIN_CACHE_REQUIRE_GESTURE_FLAG: u32 = 1u32;
pub const NCRYPT_PLATFORM_ATTEST_MAGIC: u32 = 1146110288u32;
pub struct NCRYPT_PLATFORM_ATTEST_PADDING_INFO(i32);
pub const NCRYPT_PREFER_VIRTUAL_ISOLATION_FLAG: u32 = 65536u32;
pub const NCRYPT_PROTECTED_KEY_BLOB_MAGIC: u32 = 1263817296u32;
pub const NCRYPT_PROTECTION_INFO_TYPE_DESCRIPTOR_STRING: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NCRYPT_PROTECT_STREAM_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NCRYPT_PROTECT_STREAM_INFO_EX(i32);
pub const NCRYPT_PROTECT_TO_LOCAL_SYSTEM: u32 = 32768u32;
pub const NCRYPT_SEALING_FLAG: u32 = 256u32;
pub struct NCRYPT_SUPPORTED_LENGTHS(i32);
pub const NCRYPT_TPM12_PROVIDER: u32 = 65536u32;
pub struct NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER(i32);
pub const NCRYPT_TPM_LOADABLE_KEY_BLOB_MAGIC: u32 = 1297371211u32;
pub const NCRYPT_TPM_PAD_PSS_IGNORE_SALT: u32 = 32u32;
pub struct NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT(i32);
pub const NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_V0: u32 = 0u32;
pub const NCRYPT_TPM_PSS_SALT_SIZE_HASHSIZE: u32 = 2u32;
pub const NCRYPT_TPM_PSS_SALT_SIZE_MAXIMUM: u32 = 1u32;
pub const NCRYPT_TPM_PSS_SALT_SIZE_UNKNOWN: u32 = 0u32;
pub const NCRYPT_TREAT_NIST_AS_GENERIC_ECC_FLAG: u32 = 8192u32;
pub const NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG: u32 = 8u32;
pub const NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG: u32 = 4u32;
pub const NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NCRYPT_UI_POLICY(i32);
pub const NCRYPT_UI_PROTECT_KEY_FLAG: u32 = 1u32;
pub const NCRYPT_USE_PER_BOOT_KEY_FLAG: u32 = 262144u32;
pub const NCRYPT_USE_VIRTUAL_ISOLATION_FLAG: u32 = 131072u32;
pub struct NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS(i32);
pub const NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_VSM_KEY_ATTESTATION_CLAIM_RESTRICTIONS_V0: u32 = 0u32;
pub struct NCRYPT_VSM_KEY_ATTESTATION_STATEMENT(i32);
pub const NCRYPT_VSM_KEY_ATTESTATION_STATEMENT_CURRENT_VERSION: u32 = 0u32;
pub const NCRYPT_VSM_KEY_ATTESTATION_STATEMENT_V0: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NCryptAlgorithmName(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NCryptKeyName(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NCryptProviderName(i32);
pub const NETSCAPE_SIGN_CA_CERT_TYPE: u32 = 1u32;
pub const NETSCAPE_SIGN_CERT_TYPE: u32 = 16u32;
pub const NETSCAPE_SMIME_CA_CERT_TYPE: u32 = 2u32;
pub const NETSCAPE_SMIME_CERT_TYPE: u32 = 32u32;
pub const NETSCAPE_SSL_CA_CERT_TYPE: u32 = 4u32;
pub const NETSCAPE_SSL_CLIENT_AUTH_CERT_TYPE: u32 = 128u32;
pub const NETSCAPE_SSL_SERVER_AUTH_CERT_TYPE: u32 = 64u32;
pub const OCSP_BASIC_BY_KEY_RESPONDER_ID: u32 = 2u32;
pub const OCSP_BASIC_BY_NAME_RESPONDER_ID: u32 = 1u32;
pub const OCSP_BASIC_GOOD_CERT_STATUS: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_BASIC_RESPONSE_ENTRY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_BASIC_RESPONSE_INFO(i32);
pub const OCSP_BASIC_RESPONSE_V1: u32 = 0u32;
pub const OCSP_BASIC_REVOKED_CERT_STATUS: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_BASIC_REVOKED_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_BASIC_SIGNED_RESPONSE_INFO(i32);
pub const OCSP_BASIC_UNKNOWN_CERT_STATUS: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_CERT_ID(i32);
pub const OCSP_INTERNAL_ERROR_RESPONSE: u32 = 2u32;
pub const OCSP_MALFORMED_REQUEST_RESPONSE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_REQUEST_ENTRY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_REQUEST_INFO(i32);
pub const OCSP_REQUEST_V1: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_RESPONSE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_SIGNATURE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OCSP_SIGNED_REQUEST_INFO(i32);
pub const OCSP_SIG_REQUIRED_RESPONSE: u32 = 5u32;
pub const OCSP_SUCCESSFUL_RESPONSE: u32 = 0u32;
pub const OCSP_TRY_LATER_RESPONSE: u32 = 3u32;
pub const OCSP_UNAUTHORIZED_RESPONSE: u32 = 6u32;
pub const OPAQUEKEYBLOB: u32 = 9u32;
pub struct PCRYPT_DECRYPT_PRIVATE_KEY_FUNC(i32);
pub struct PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC(i32);
pub struct PCRYPT_RESOLVE_HCRYPTPROV_FUNC(i32);
pub struct PFNCryptStreamOutputCallback(i32);
pub struct PFNCryptStreamOutputCallbackEx(i32);
pub struct PFN_CANCEL_ASYNC_RETRIEVAL_FUNC(i32);
pub struct PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK(i32);
pub struct PFN_CERT_CREATE_CONTEXT_SORT_FUNC(i32);
pub struct PFN_CERT_DLL_OPEN_STORE_PROV_FUNC(i32);
pub struct PFN_CERT_ENUM_PHYSICAL_STORE(i32);
pub struct PFN_CERT_ENUM_SYSTEM_STORE(i32);
pub struct PFN_CERT_ENUM_SYSTEM_STORE_LOCATION(i32);
pub struct PFN_CERT_IS_WEAK_HASH(i32);
pub struct PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK(i32);
pub struct PFN_CERT_STORE_PROV_CLOSE(i32);
pub struct PFN_CERT_STORE_PROV_CONTROL(i32);
pub struct PFN_CERT_STORE_PROV_DELETE_CERT(i32);
pub struct PFN_CERT_STORE_PROV_DELETE_CRL(i32);
pub struct PFN_CERT_STORE_PROV_DELETE_CTL(i32);
pub struct PFN_CERT_STORE_PROV_FIND_CERT(i32);
pub struct PFN_CERT_STORE_PROV_FIND_CRL(i32);
pub struct PFN_CERT_STORE_PROV_FIND_CTL(i32);
pub struct PFN_CERT_STORE_PROV_FREE_FIND_CERT(i32);
pub struct PFN_CERT_STORE_PROV_FREE_FIND_CRL(i32);
pub struct PFN_CERT_STORE_PROV_FREE_FIND_CTL(i32);
pub struct PFN_CERT_STORE_PROV_GET_CERT_PROPERTY(i32);
pub struct PFN_CERT_STORE_PROV_GET_CRL_PROPERTY(i32);
pub struct PFN_CERT_STORE_PROV_GET_CTL_PROPERTY(i32);
pub struct PFN_CERT_STORE_PROV_READ_CERT(i32);
pub struct PFN_CERT_STORE_PROV_READ_CRL(i32);
pub struct PFN_CERT_STORE_PROV_READ_CTL(i32);
pub struct PFN_CERT_STORE_PROV_SET_CERT_PROPERTY(i32);
pub struct PFN_CERT_STORE_PROV_SET_CRL_PROPERTY(i32);
pub struct PFN_CERT_STORE_PROV_SET_CTL_PROPERTY(i32);
pub struct PFN_CERT_STORE_PROV_WRITE_CERT(i32);
pub struct PFN_CERT_STORE_PROV_WRITE_CRL(i32);
pub struct PFN_CERT_STORE_PROV_WRITE_CTL(i32);
pub struct PFN_CMSG_ALLOC(i32);
pub struct PFN_CMSG_CNG_IMPORT_CONTENT_ENCRYPT_KEY(i32);
pub struct PFN_CMSG_CNG_IMPORT_KEY_AGREE(i32);
pub struct PFN_CMSG_CNG_IMPORT_KEY_TRANS(i32);
pub struct PFN_CMSG_EXPORT_ENCRYPT_KEY(i32);
pub struct PFN_CMSG_EXPORT_KEY_AGREE(i32);
pub struct PFN_CMSG_EXPORT_KEY_TRANS(i32);
pub struct PFN_CMSG_EXPORT_MAIL_LIST(i32);
pub struct PFN_CMSG_FREE(i32);
pub struct PFN_CMSG_GEN_CONTENT_ENCRYPT_KEY(i32);
pub struct PFN_CMSG_GEN_ENCRYPT_KEY(i32);
pub struct PFN_CMSG_IMPORT_ENCRYPT_KEY(i32);
pub struct PFN_CMSG_IMPORT_KEY_AGREE(i32);
pub struct PFN_CMSG_IMPORT_KEY_TRANS(i32);
pub struct PFN_CMSG_IMPORT_MAIL_LIST(i32);
pub struct PFN_CMSG_STREAM_OUTPUT(i32);
pub struct PFN_CRYPT_ALLOC(i32);
pub struct PFN_CRYPT_ASYNC_PARAM_FREE_FUNC(i32);
pub struct PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC(i32);
pub struct PFN_CRYPT_CANCEL_RETRIEVAL(i32);
pub struct PFN_CRYPT_ENUM_KEYID_PROP(i32);
pub struct PFN_CRYPT_ENUM_OID_FUNC(i32);
pub struct PFN_CRYPT_ENUM_OID_INFO(i32);
pub struct PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC(i32);
pub struct PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC(i32);
pub struct PFN_CRYPT_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC(i32);
pub struct PFN_CRYPT_FREE(i32);
pub struct PFN_CRYPT_GET_SIGNER_CERTIFICATE(i32);
pub struct PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH(i32);
pub struct PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE(i32);
pub struct PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER(i32);
pub struct PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD(i32);
pub struct PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET(i32);
pub struct PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_INITIALIZE(i32);
pub struct PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE(i32);
pub struct PFN_CRYPT_SIGN_AND_ENCODE_HASH_FUNC(i32);
pub struct PFN_CRYPT_VERIFY_ENCODED_SIGNATURE_FUNC(i32);
pub struct PFN_CRYPT_XML_CREATE_TRANSFORM(i32);
pub struct PFN_CRYPT_XML_DATA_PROVIDER_CLOSE(i32);
pub struct PFN_CRYPT_XML_DATA_PROVIDER_READ(i32);
pub struct PFN_CRYPT_XML_ENUM_ALG_INFO(i32);
pub struct PFN_CRYPT_XML_WRITE_CALLBACK(i32);
pub struct PFN_EXPORT_PRIV_KEY_FUNC(i32);
pub struct PFN_FREE_ENCODED_OBJECT_FUNC(i32);
pub struct PFN_IMPORT_PRIV_KEY_FUNC(i32);
pub struct PFN_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC(i32);
pub struct PFN_NCRYPT_ALLOC(i32);
pub struct PFN_NCRYPT_FREE(i32);
pub const PKCS12_DISABLE_ENCRYPT_CERTIFICATES: u32 = 256u32;
pub const PKCS12_ENCRYPT_CERTIFICATES: u32 = 512u32;
pub const PKCS12_EXPORT_ECC_CURVE_OID: u32 = 8192u32;
pub const PKCS12_EXPORT_ECC_CURVE_PARAMETERS: u32 = 4096u32;
pub const PKCS12_EXPORT_PBES2_PARAMS: u32 = 128u32;
pub const PKCS12_EXPORT_RESERVED_MASK: u32 = 4294901760u32;
pub const PKCS12_EXPORT_SILENT: u32 = 64u32;
pub const PKCS12_IMPORT_RESERVED_MASK: u32 = 4294901760u32;
pub const PKCS12_IMPORT_SILENT: u32 = 64u32;
pub const PKCS12_ONLY_CERTIFICATES: u32 = 1024u32;
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_TYPE: u32 = 0u32;
pub const PKCS12_ONLY_NOT_ENCRYPTED_CERTIFICATES: u32 = 2048u32;
#[cfg(feature = "Win32_Foundation")]
pub struct PKCS12_PBES2_EXPORT_PARAMS(i32);
pub const PKCS12_PROTECT_TO_DOMAIN_SIDS: u32 = 32u32;
pub const PKCS12_VIRTUAL_ISOLATION_KEY: u32 = 65536u32;
pub const PKCS5_PADDING: u32 = 1u32;
pub const PKCS_7_NDR_ENCODING: u32 = 131072u32;
pub const PKCS_RSA_SSA_PSS_TRAILER_FIELD_BC: u32 = 1u32;
pub const PLAINTEXTKEYBLOB: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct POLICY_ELEMENT(i32);
pub const PP_ADMIN_PIN: u32 = 31u32;
pub const PP_APPLI_CERT: u32 = 18u32;
pub const PP_CERTCHAIN: u32 = 9u32;
pub const PP_CHANGE_PASSWORD: u32 = 7u32;
pub const PP_CONTAINER: u32 = 6u32;
pub const PP_CONTEXT_INFO: u32 = 11u32;
pub const PP_CRYPT_COUNT_KEY_USE: u32 = 41u32;
pub const PP_DISMISS_PIN_UI_SEC: u32 = 49u32;
pub const PP_ENUMALGS: u32 = 1u32;
pub const PP_ENUMALGS_EX: u32 = 22u32;
pub const PP_ENUMCONTAINERS: u32 = 2u32;
pub const PP_ENUMELECTROOTS: u32 = 26u32;
pub const PP_ENUMEX_SIGNING_PROT: u32 = 40u32;
pub const PP_ENUMMANDROOTS: u32 = 25u32;
pub const PP_IMPTYPE: u32 = 3u32;
pub const PP_KEYSET_TYPE: u32 = 27u32;
pub const PP_KEYSPEC: u32 = 39u32;
pub const PP_KEYSTORAGE: u32 = 17u32;
pub const PP_KEYX_KEYSIZE_INC: u32 = 35u32;
pub const PP_KEY_TYPE_SUBTYPE: u32 = 10u32;
pub const PP_NAME: u32 = 4u32;
pub const PP_PROVTYPE: u32 = 16u32;
pub const PP_SESSION_KEYSIZE: u32 = 20u32;
pub const PP_SGC_INFO: u32 = 37u32;
pub const PP_SIG_KEYSIZE_INC: u32 = 34u32;
pub const PP_SMARTCARD_GUID: u32 = 45u32;
pub const PP_SMARTCARD_READER_ICON: u32 = 47u32;
pub const PP_SYM_KEYSIZE: u32 = 19u32;
pub const PP_UNIQUE_CONTAINER: u32 = 36u32;
pub const PP_VERSION: u32 = 5u32;
pub const PRIVATEKEYBLOB: u32 = 7u32;
pub struct PRIVKEYVER3(i32);
pub const PROV_DH_SCHANNEL: u32 = 18u32;
pub const PROV_DSS: u32 = 3u32;
pub const PROV_DSS_DH: u32 = 13u32;
pub const PROV_EC_ECDSA_FULL: u32 = 16u32;
pub const PROV_EC_ECDSA_SIG: u32 = 14u32;
pub const PROV_EC_ECNRA_FULL: u32 = 17u32;
pub const PROV_EC_ECNRA_SIG: u32 = 15u32;
#[cfg(feature = "Win32_Foundation")]
pub struct PROV_ENUMALGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PROV_ENUMALGS_EX(i32);
pub const PROV_FORTEZZA: u32 = 4u32;
pub const PROV_INTEL_SEC: u32 = 22u32;
pub const PROV_MS_EXCHANGE: u32 = 5u32;
pub const PROV_REPLACE_OWF: u32 = 23u32;
pub const PROV_RNG: u32 = 21u32;
pub const PROV_RSA_AES: u32 = 24u32;
pub const PROV_RSA_FULL: u32 = 1u32;
pub const PROV_RSA_SCHANNEL: u32 = 12u32;
pub const PROV_RSA_SIG: u32 = 2u32;
pub const PROV_SPYRUS_LYNKS: u32 = 20u32;
pub const PROV_SSL: u32 = 6u32;
pub const PROV_STT_ACQ: u32 = 8u32;
pub const PROV_STT_BRND: u32 = 9u32;
pub const PROV_STT_ISS: u32 = 11u32;
pub const PROV_STT_MER: u32 = 7u32;
pub const PROV_STT_ROOT: u32 = 10u32;
pub struct PUBKEY(i32);
pub struct PUBKEYVER3(i32);
pub const PUBLICKEYBLOB: u32 = 6u32;
pub const PUBLICKEYBLOBEX: u32 = 10u32;
pub struct PUBLICKEYSTRUC(i32);
pub struct PaddingMode(i32);
pub const RANDOM_PADDING: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct RECIPIENTPOLICY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RECIPIENTPOLICY2(i32);
pub const RECIPIENTPOLICYV1: u32 = 1u32;
pub const RECIPIENTPOLICYV2: u32 = 2u32;
pub const REPORT_NOT_ABLE_TO_EXPORT_PRIVATE_KEY: u32 = 2u32;
pub const REPORT_NO_PRIVATE_KEY: u32 = 1u32;
pub struct ROOT_INFO_LUID(i32);
pub const RSA1024BIT_KEY: u32 = 67108864u32;
pub struct RSAPUBKEY(i32);
pub struct SCHANNEL_ALG(i32);
pub const SCHANNEL_ENC_KEY: u32 = 1u32;
pub const SCHANNEL_MAC_KEY: u32 = 0u32;
pub const SIGNATURE_RESOURCE_NUMBER: u32 = 666u32;
pub const SIMPLEBLOB: u32 = 1u32;
pub const SITE_PIN_RULES_ALL_SUBDOMAINS_FLAG: u32 = 1u32;
pub const SORTED_CTL_EXT_HASHED_SUBJECT_IDENTIFIER_FLAG: u32 = 1u32;
pub struct SSL_ECCKEY_BLOB(i32);
pub const SSL_F12_ERROR_TEXT_LENGTH: u32 = 256u32;
pub struct SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS(i32);
pub const SSL_HPKP_HEADER_COUNT: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA(i32);
pub const SSL_HPKP_PKP_HEADER_INDEX: u32 = 0u32;
pub const SSL_HPKP_PKP_RO_HEADER_INDEX: u32 = 1u32;
pub const SSL_KEY_PIN_ERROR_TEXT_LENGTH: u32 = 512u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA(i32);
pub struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS(i32);
pub const SYMMETRICWRAPKEYBLOB: u32 = 11u32;
pub const TIMESTAMP_DONT_HASH_DATA: u32 = 1u32;
pub const TIMESTAMP_FAILURE_BAD_ALG: u32 = 0u32;
pub const TIMESTAMP_FAILURE_BAD_FORMAT: u32 = 5u32;
pub const TIMESTAMP_FAILURE_BAD_REQUEST: u32 = 2u32;
pub const TIMESTAMP_FAILURE_EXTENSION_NOT_SUPPORTED: u32 = 16u32;
pub const TIMESTAMP_FAILURE_INFO_NOT_AVAILABLE: u32 = 17u32;
pub const TIMESTAMP_FAILURE_POLICY_NOT_SUPPORTED: u32 = 15u32;
pub const TIMESTAMP_FAILURE_SYSTEM_FAILURE: u32 = 25u32;
pub const TIMESTAMP_FAILURE_TIME_NOT_AVAILABLE: u32 = 14u32;
pub const TIMESTAMP_NO_AUTH_RETRIEVAL: u32 = 131072u32;
pub const TIMESTAMP_VERIFY_CONTEXT_SIGNATURE: u32 = 32u32;
pub const USAGE_MATCH_TYPE_AND: u32 = 0u32;
pub const USAGE_MATCH_TYPE_OR: u32 = 1u32;
pub const X509_NDR_ENCODING: u32 = 2u32;
pub const ZERO_PADDING: u32 = 3u32;
pub struct __NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT(i32);
