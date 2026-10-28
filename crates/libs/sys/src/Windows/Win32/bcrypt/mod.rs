#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptAddContextFunction(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_sys::core::PCWSTR, dwposition : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptCloseAlgorithmProvider(halgorithm : BCRYPT_ALG_HANDLE, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptConfigureContext(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, pconfig : PCRYPT_CONTEXT_CONFIG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptConfigureContextFunction(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_sys::core::PCWSTR, pconfig : PCRYPT_CONTEXT_FUNCTION_CONFIG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptCreateContext(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, pconfig : PCRYPT_CONTEXT_CONFIG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptCreateHash(halgorithm : BCRYPT_ALG_HANDLE, phhash : *mut BCRYPT_HASH_HANDLE, pbhashobject : super::PUCHAR, cbhashobject : super::ULONG, pbsecret : super::PUCHAR, cbsecret : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptCreateMultiHash(halgorithm : BCRYPT_ALG_HANDLE, phhash : *mut BCRYPT_HASH_HANDLE, nhashes : super::ULONG, pbhashobject : super::PUCHAR, cbhashobject : super::ULONG, pbsecret : super::PUCHAR, cbsecret : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptDecapsulate(hkey : BCRYPT_KEY_HANDLE, pbciphertext : super::PUCHAR, cbciphertext : super::ULONG, pbsecretkey : super::PUCHAR, cbsecretkey : super::ULONG, pcbsecretkey : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptDecrypt(hkey : BCRYPT_KEY_HANDLE, pbinput : super::PUCHAR, cbinput : super::ULONG, ppaddinginfo : *const core::ffi::c_void, pbiv : super::PUCHAR, cbiv : super::ULONG, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptDeleteContext(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptDeriveKey(hsharedsecret : BCRYPT_SECRET_HANDLE, pwszkdf : windows_sys::core::PCWSTR, pparameterlist : *const BCryptBufferDesc, pbderivedkey : super::PUCHAR, cbderivedkey : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptDeriveKeyCapi(hhash : BCRYPT_HASH_HANDLE, htargetalg : BCRYPT_ALG_HANDLE, pbderivedkey : super::PUCHAR, cbderivedkey : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptDeriveKeyPBKDF2(hprf : BCRYPT_ALG_HANDLE, pbpassword : super::PUCHAR, cbpassword : super::ULONG, pbsalt : super::PUCHAR, cbsalt : super::ULONG, citerations : super::ULONGLONG, pbderivedkey : super::PUCHAR, cbderivedkey : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
windows_link::link!("bcrypt.dll" "system" fn BCryptDestroyHash(hhash : BCRYPT_HASH_HANDLE) -> windows_sys::core::NTSTATUS);
windows_link::link!("bcrypt.dll" "system" fn BCryptDestroyKey(hkey : BCRYPT_KEY_HANDLE) -> windows_sys::core::NTSTATUS);
windows_link::link!("bcrypt.dll" "system" fn BCryptDestroySecret(hsecret : BCRYPT_SECRET_HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptDuplicateHash(hhash : BCRYPT_HASH_HANDLE, phnewhash : *mut BCRYPT_HASH_HANDLE, pbhashobject : super::PUCHAR, cbhashobject : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptDuplicateKey(hkey : BCRYPT_KEY_HANDLE, phnewkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : super::PUCHAR, cbkeyobject : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptEncapsulate(hkey : BCRYPT_KEY_HANDLE, pbsecretkey : super::PUCHAR, cbsecretkey : super::ULONG, pcbsecretkey : *mut super::ULONG, pbciphertext : super::PUCHAR, cbciphertext : super::ULONG, pcbciphertext : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptEncrypt(hkey : BCRYPT_KEY_HANDLE, pbinput : super::PUCHAR, cbinput : super::ULONG, ppaddinginfo : *const core::ffi::c_void, pbiv : super::PUCHAR, cbiv : super::ULONG, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptEnumAlgorithms(dwalgoperations : super::ULONG, palgcount : *mut super::ULONG, ppalglist : *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptEnumContextFunctionProviders(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_sys::core::PCWSTR, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXT_FUNCTION_PROVIDERS) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptEnumContextFunctions(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXT_FUNCTIONS) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptEnumContexts(dwtable : super::ULONG, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXTS) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptEnumProviders(pszalgid : windows_sys::core::PCWSTR, pimplcount : *mut super::ULONG, ppimpllist : *mut *mut BCRYPT_PROVIDER_NAME, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptEnumRegisteredProviders(pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_PROVIDERS) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptExportKey(hkey : BCRYPT_KEY_HANDLE, hexportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_sys::core::PCWSTR, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptFinalizeKeyPair(hkey : BCRYPT_KEY_HANDLE, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptFinishHash(hhash : BCRYPT_HASH_HANDLE, pboutput : super::PUCHAR, cboutput : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "winnt")]
windows_link::link!("bcrypt.dll" "system" fn BCryptFreeBuffer(pvbuffer : super::PVOID));
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptGenRandom(halgorithm : BCRYPT_ALG_HANDLE, pbbuffer : super::PUCHAR, cbbuffer : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptGenerateKeyPair(halgorithm : BCRYPT_ALG_HANDLE, phkey : *mut BCRYPT_KEY_HANDLE, dwlength : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptGenerateSymmetricKey(halgorithm : BCRYPT_ALG_HANDLE, phkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : super::PUCHAR, cbkeyobject : super::ULONG, pbsecret : super::PUCHAR, cbsecret : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptGetFipsAlgorithmMode(pfenabled : *mut super::BOOLEAN) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptGetProperty(hobject : BCRYPT_HANDLE, pszproperty : windows_sys::core::PCWSTR, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptHash(halgorithm : BCRYPT_ALG_HANDLE, pbsecret : super::PUCHAR, cbsecret : super::ULONG, pbinput : super::PUCHAR, cbinput : super::ULONG, pboutput : super::PUCHAR, cboutput : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptHashData(hhash : BCRYPT_HASH_HANDLE, pbinput : super::PUCHAR, cbinput : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptImportKey(halgorithm : BCRYPT_ALG_HANDLE, himportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_sys::core::PCWSTR, phkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : super::PUCHAR, cbkeyobject : super::ULONG, pbinput : super::PUCHAR, cbinput : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptImportKeyPair(halgorithm : BCRYPT_ALG_HANDLE, himportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_sys::core::PCWSTR, phkey : *mut BCRYPT_KEY_HANDLE, pbinput : super::PUCHAR, cbinput : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptKeyDerivation(hkey : BCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, pbderivedkey : super::PUCHAR, cbderivedkey : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptOpenAlgorithmProvider(phalgorithm : *mut BCRYPT_ALG_HANDLE, pszalgid : windows_sys::core::PCWSTR, pszimplementation : windows_sys::core::PCWSTR, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptProcessMultiOperations(hobject : BCRYPT_HANDLE, operationtype : BCRYPT_MULTI_OPERATION_TYPE, poperations : super::PVOID, cboperations : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptQueryContextConfiguration(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXT_CONFIG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptQueryContextFunctionConfiguration(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_sys::core::PCWSTR, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXT_FUNCTION_CONFIG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptQueryContextFunctionProperty(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_sys::core::PCWSTR, pszproperty : windows_sys::core::PCWSTR, pcbvalue : *mut super::ULONG, ppbvalue : *mut super::PUCHAR) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptQueryProviderRegistration(pszprovider : windows_sys::core::PCWSTR, dwmode : super::ULONG, dwinterface : super::ULONG, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_PROVIDER_REG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "winnt")]
windows_link::link!("bcrypt.dll" "system" fn BCryptRegisterConfigChangeNotify(phevent : *mut super::HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptRemoveContextFunction(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_sys::core::PCWSTR) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptResolveProviders(pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_sys::core::PCWSTR, pszprovider : windows_sys::core::PCWSTR, dwmode : super::ULONG, dwflags : super::ULONG, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_PROVIDER_REFS) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("bcrypt.dll" "system" fn BCryptSecretAgreement(hprivkey : BCRYPT_KEY_HANDLE, hpubkey : BCRYPT_KEY_HANDLE, phagreedsecret : *mut BCRYPT_SECRET_HANDLE, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptSetContextFunctionProperty(dwtable : super::ULONG, pszcontext : windows_sys::core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_sys::core::PCWSTR, pszproperty : windows_sys::core::PCWSTR, cbvalue : super::ULONG, pbvalue : super::PUCHAR) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptSetProperty(hobject : BCRYPT_HANDLE, pszproperty : windows_sys::core::PCWSTR, pbinput : super::PUCHAR, cbinput : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptSignHash(hkey : BCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbinput : super::PUCHAR, cbinput : super::ULONG, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "winnt")]
windows_link::link!("bcrypt.dll" "system" fn BCryptUnregisterConfigChangeNotify(hevent : super::HANDLE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("bcrypt.dll" "system" fn BCryptVerifySignature(hkey : BCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbhash : super::PUCHAR, cbhash : super::ULONG, pbsignature : super::PUCHAR, cbsignature : super::ULONG, dwflags : super::ULONG) -> windows_sys::core::NTSTATUS);
pub const BCRYPTBUFFER_VERSION: i32 = 0;
pub const BCRYPT_3DES_112_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("3DES_112");
pub const BCRYPT_3DES_112_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 369 as _;
pub const BCRYPT_3DES_112_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 401 as _;
pub const BCRYPT_3DES_112_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 385 as _;
pub const BCRYPT_3DES_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("3DES");
pub const BCRYPT_3DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 321 as _;
pub const BCRYPT_3DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 353 as _;
pub const BCRYPT_3DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 337 as _;
pub const BCRYPT_AES_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("AES");
pub const BCRYPT_AES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 417 as _;
pub const BCRYPT_AES_CCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 465 as _;
pub const BCRYPT_AES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 449 as _;
pub const BCRYPT_AES_CMAC_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("AES-CMAC");
pub const BCRYPT_AES_CMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 257 as _;
pub const BCRYPT_AES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 433 as _;
pub const BCRYPT_AES_GCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 481 as _;
pub const BCRYPT_AES_GMAC_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("AES-GMAC");
pub const BCRYPT_AES_GMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 273 as _;
pub const BCRYPT_AES_WRAP_KEY_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("Rfc3565KeyWrapBlob");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_ALGORITHM_IDENTIFIER {
    pub pszName: windows_sys::core::PWSTR,
    pub dwClass: super::ULONG,
    pub dwFlags: super::ULONG,
}
pub const BCRYPT_ALGORITHM_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AlgorithmName");
pub type BCRYPT_ALG_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_ALG_HANDLE_HMAC_FLAG: i32 = 8;
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: i32 = 3;
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    pub cbSize: super::ULONG,
    pub dwInfoVersion: super::ULONG,
    pub pbNonce: super::PUCHAR,
    pub cbNonce: super::ULONG,
    pub pbAuthData: super::PUCHAR,
    pub cbAuthData: super::ULONG,
    pub pbTag: super::PUCHAR,
    pub cbTag: super::ULONG,
    pub pbMacContext: super::PUCHAR,
    pub cbMacContext: super::ULONG,
    pub cbAAD: super::ULONG,
    pub cbData: super::ULONGLONG,
    pub dwFlags: super::ULONG,
}
pub const BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO_VERSION: i32 = 1;
pub const BCRYPT_AUTH_MODE_CHAIN_CALLS_FLAG: i32 = 1;
pub const BCRYPT_AUTH_MODE_IN_PROGRESS_FLAG: i32 = 2;
pub const BCRYPT_AUTH_TAG_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("AuthTagLength");
#[cfg(feature = "shared")]
pub type BCRYPT_AUTH_TAG_LENGTHS_STRUCT = BCRYPT_KEY_LENGTHS_STRUCT;
pub const BCRYPT_BLOCK_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("BlockLength");
pub const BCRYPT_BLOCK_PADDING: i32 = 1;
pub const BCRYPT_BLOCK_SIZE_LIST: windows_sys::core::PCWSTR = windows_sys::core::w!("BlockSizeList");
pub const BCRYPT_BUFFERS_LOCKED_FLAG: i32 = 64;
pub const BCRYPT_CAPI_AES_FLAG: i32 = 16;
pub const BCRYPT_CAPI_KDF_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CAPI_KDF");
pub const BCRYPT_CAPI_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 801 as _;
pub const BCRYPT_CHACHA20_POLY1305_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CHACHA20_POLY1305");
pub const BCRYPT_CHACHA20_POLY1305_ALG_HANDLE: BCRYPT_ALG_HANDLE = 929 as _;
pub const BCRYPT_CHAINING_MODE: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainingMode");
pub const BCRYPT_CHAIN_MODE_CBC: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainingModeCBC");
pub const BCRYPT_CHAIN_MODE_CCM: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainingModeCCM");
pub const BCRYPT_CHAIN_MODE_CFB: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainingModeCFB");
pub const BCRYPT_CHAIN_MODE_ECB: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainingModeECB");
pub const BCRYPT_CHAIN_MODE_GCM: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainingModeGCM");
pub const BCRYPT_CHAIN_MODE_NA: windows_sys::core::PCWSTR = windows_sys::core::w!("ChainingModeN/A");
pub const BCRYPT_CIPHER_INTERFACE: i32 = 1;
pub const BCRYPT_CIPHER_OPERATION: i32 = 1;
pub const BCRYPT_CSHAKE128_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CSHAKE128");
pub const BCRYPT_CSHAKE128_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1041 as _;
pub const BCRYPT_CSHAKE256_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("CSHAKE256");
pub const BCRYPT_CSHAKE256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1057 as _;
pub const BCRYPT_CUSTOMIZATION_STRING: windows_sys::core::PCWSTR = windows_sys::core::w!("CustomizationString");
pub const BCRYPT_DESX_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("DESX");
pub const BCRYPT_DESX_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 545 as _;
pub const BCRYPT_DESX_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 577 as _;
pub const BCRYPT_DESX_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 561 as _;
pub const BCRYPT_DES_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("DES");
pub const BCRYPT_DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 497 as _;
pub const BCRYPT_DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 529 as _;
pub const BCRYPT_DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 513 as _;
pub const BCRYPT_DH_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("DH");
pub const BCRYPT_DH_ALG_HANDLE: BCRYPT_ALG_HANDLE = 641 as _;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_DH_KEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
}
pub const BCRYPT_DH_PARAMETERS: windows_sys::core::PCWSTR = windows_sys::core::w!("DHParameters");
pub const BCRYPT_DH_PARAMETERS_MAGIC: i32 = 1297107012;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_DH_PARAMETER_HEADER {
    pub cbLength: super::ULONG,
    pub dwMagic: super::ULONG,
    pub cbKeyLength: super::ULONG,
}
pub const BCRYPT_DH_PRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("DHPRIVATEBLOB");
pub const BCRYPT_DH_PRIVATE_MAGIC: i32 = 1448101956;
pub const BCRYPT_DH_PUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("DHPUBLICBLOB");
pub const BCRYPT_DH_PUBLIC_MAGIC: i32 = 1112557636;
pub const BCRYPT_DSA_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("DSA");
pub const BCRYPT_DSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 721 as _;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BCRYPT_DSA_KEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
    pub Count: [super::UCHAR; 4],
    pub Seed: [super::UCHAR; 20],
    pub q: [super::UCHAR; 20],
}
#[cfg(feature = "shared")]
impl Default for BCRYPT_DSA_KEY_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BCRYPT_DSA_KEY_BLOB_V2 {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: super::ULONG,
    pub cbGroupSize: super::ULONG,
    pub Count: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for BCRYPT_DSA_KEY_BLOB_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_DSA_PARAMETERS: windows_sys::core::PCWSTR = windows_sys::core::w!("DSAParameters");
pub const BCRYPT_DSA_PARAMETERS_MAGIC: i32 = 1297109828;
pub const BCRYPT_DSA_PARAMETERS_MAGIC_V2: i32 = 843927620;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BCRYPT_DSA_PARAMETER_HEADER {
    pub cbLength: super::ULONG,
    pub dwMagic: super::ULONG,
    pub cbKeyLength: super::ULONG,
    pub Count: [super::UCHAR; 4],
    pub Seed: [super::UCHAR; 20],
    pub q: [super::UCHAR; 20],
}
#[cfg(feature = "shared")]
impl Default for BCRYPT_DSA_PARAMETER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BCRYPT_DSA_PARAMETER_HEADER_V2 {
    pub cbLength: super::ULONG,
    pub dwMagic: super::ULONG,
    pub cbKeyLength: super::ULONG,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: super::ULONG,
    pub cbGroupSize: super::ULONG,
    pub Count: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for BCRYPT_DSA_PARAMETER_HEADER_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_DSA_PRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("DSAPRIVATEBLOB");
pub const BCRYPT_DSA_PRIVATE_MAGIC: i32 = 1448104772;
pub const BCRYPT_DSA_PRIVATE_MAGIC_V2: i32 = 844517444;
pub const BCRYPT_DSA_PUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("DSAPUBLICBLOB");
pub const BCRYPT_DSA_PUBLIC_MAGIC: i32 = 1112560452;
pub const BCRYPT_DSA_PUBLIC_MAGIC_V2: i32 = 843206724;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_ECCFULLKEY_BLOB {
    pub dwMagic: super::ULONG,
    pub dwVersion: super::ULONG,
    pub dwCurveType: ECC_CURVE_TYPE_ENUM,
    pub dwCurveGenerationAlgId: ECC_CURVE_ALG_ID_ENUM,
    pub cbFieldLength: super::ULONG,
    pub cbSubgroupOrder: super::ULONG,
    pub cbCofactor: super::ULONG,
    pub cbSeed: super::ULONG,
}
pub const BCRYPT_ECCFULLPRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("ECCFULLPRIVATEBLOB");
pub const BCRYPT_ECCFULLPUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("ECCFULLPUBLICBLOB");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_ECCKEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
}
pub const BCRYPT_ECCPRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("ECCPRIVATEBLOB");
pub const BCRYPT_ECCPUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("ECCPUBLICBLOB");
pub const BCRYPT_ECC_CURVE_25519: windows_sys::core::PCWSTR = windows_sys::core::w!("curve25519");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160R1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP160r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160T1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP160t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192R1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP192r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192T1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP192t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224R1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP224r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224T1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP224t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256R1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP256r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256T1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP256t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320R1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP320r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320T1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP320t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384R1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP384r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384T1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP384t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512R1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP512r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512T1: windows_sys::core::PCWSTR = windows_sys::core::w!("brainpoolP512t1");
pub const BCRYPT_ECC_CURVE_EC192WAPI: windows_sys::core::PCWSTR = windows_sys::core::w!("ec192wapi");
pub const BCRYPT_ECC_CURVE_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ECCCurveName");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_ECC_CURVE_NAMES {
    pub dwEccCurveNames: super::ULONG,
    pub pEccCurveNames: *mut windows_sys::core::PWSTR,
}
pub const BCRYPT_ECC_CURVE_NAME_LIST: windows_sys::core::PCWSTR = windows_sys::core::w!("ECCCurveNameList");
pub const BCRYPT_ECC_CURVE_NISTP192: windows_sys::core::PCWSTR = windows_sys::core::w!("nistP192");
pub const BCRYPT_ECC_CURVE_NISTP224: windows_sys::core::PCWSTR = windows_sys::core::w!("nistP224");
pub const BCRYPT_ECC_CURVE_NISTP256: windows_sys::core::PCWSTR = windows_sys::core::w!("nistP256");
pub const BCRYPT_ECC_CURVE_NISTP384: windows_sys::core::PCWSTR = windows_sys::core::w!("nistP384");
pub const BCRYPT_ECC_CURVE_NISTP521: windows_sys::core::PCWSTR = windows_sys::core::w!("nistP521");
pub const BCRYPT_ECC_CURVE_NUMSP256T1: windows_sys::core::PCWSTR = windows_sys::core::w!("numsP256t1");
pub const BCRYPT_ECC_CURVE_NUMSP384T1: windows_sys::core::PCWSTR = windows_sys::core::w!("numsP384t1");
pub const BCRYPT_ECC_CURVE_NUMSP512T1: windows_sys::core::PCWSTR = windows_sys::core::w!("numsP512t1");
pub const BCRYPT_ECC_CURVE_SECP160K1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP160k1");
pub const BCRYPT_ECC_CURVE_SECP160R1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP160r1");
pub const BCRYPT_ECC_CURVE_SECP160R2: windows_sys::core::PCWSTR = windows_sys::core::w!("secP160r2");
pub const BCRYPT_ECC_CURVE_SECP192K1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP192k1");
pub const BCRYPT_ECC_CURVE_SECP192R1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP192r1");
pub const BCRYPT_ECC_CURVE_SECP224K1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP224k1");
pub const BCRYPT_ECC_CURVE_SECP224R1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP224r1");
pub const BCRYPT_ECC_CURVE_SECP256K1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP256k1");
pub const BCRYPT_ECC_CURVE_SECP256R1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP256r1");
pub const BCRYPT_ECC_CURVE_SECP384R1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP384r1");
pub const BCRYPT_ECC_CURVE_SECP521R1: windows_sys::core::PCWSTR = windows_sys::core::w!("secP521r1");
pub const BCRYPT_ECC_CURVE_WTLS12: windows_sys::core::PCWSTR = windows_sys::core::w!("wtls12");
pub const BCRYPT_ECC_CURVE_WTLS7: windows_sys::core::PCWSTR = windows_sys::core::w!("wtls7");
pub const BCRYPT_ECC_CURVE_WTLS9: windows_sys::core::PCWSTR = windows_sys::core::w!("wtls9");
pub const BCRYPT_ECC_CURVE_X962P192V1: windows_sys::core::PCWSTR = windows_sys::core::w!("x962P192v1");
pub const BCRYPT_ECC_CURVE_X962P192V2: windows_sys::core::PCWSTR = windows_sys::core::w!("x962P192v2");
pub const BCRYPT_ECC_CURVE_X962P192V3: windows_sys::core::PCWSTR = windows_sys::core::w!("x962P192v3");
pub const BCRYPT_ECC_CURVE_X962P239V1: windows_sys::core::PCWSTR = windows_sys::core::w!("x962P239v1");
pub const BCRYPT_ECC_CURVE_X962P239V2: windows_sys::core::PCWSTR = windows_sys::core::w!("x962P239v2");
pub const BCRYPT_ECC_CURVE_X962P239V3: windows_sys::core::PCWSTR = windows_sys::core::w!("x962P239v3");
pub const BCRYPT_ECC_CURVE_X962P256V1: windows_sys::core::PCWSTR = windows_sys::core::w!("x962P256v1");
pub const BCRYPT_ECC_FULLKEY_BLOB_V1: i32 = 1;
pub const BCRYPT_ECC_PARAMETERS: windows_sys::core::PCWSTR = windows_sys::core::w!("ECCParameters");
pub const BCRYPT_ECC_PARAMETERS_MAGIC: i32 = 1346585413;
pub const BCRYPT_ECC_PRIME_MONTGOMERY_CURVE: ECC_CURVE_TYPE_ENUM = 3;
pub const BCRYPT_ECC_PRIME_SHORT_WEIERSTRASS_CURVE: ECC_CURVE_TYPE_ENUM = 1;
pub const BCRYPT_ECC_PRIME_TWISTED_EDWARDS_CURVE: ECC_CURVE_TYPE_ENUM = 2;
pub const BCRYPT_ECDH_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDH");
pub const BCRYPT_ECDH_ALG_HANDLE: BCRYPT_ALG_HANDLE = 657 as _;
pub const BCRYPT_ECDH_P256_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDH_P256");
pub const BCRYPT_ECDH_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 673 as _;
pub const BCRYPT_ECDH_P384_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDH_P384");
pub const BCRYPT_ECDH_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 689 as _;
pub const BCRYPT_ECDH_P521_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDH_P521");
pub const BCRYPT_ECDH_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = 705 as _;
pub const BCRYPT_ECDH_PRIVATE_GENERIC_MAGIC: i32 = 1447772997;
pub const BCRYPT_ECDH_PRIVATE_P256_MAGIC: i32 = 843793221;
pub const BCRYPT_ECDH_PRIVATE_P384_MAGIC: i32 = 877347653;
pub const BCRYPT_ECDH_PRIVATE_P521_MAGIC: i32 = 910902085;
pub const BCRYPT_ECDH_PUBLIC_GENERIC_MAGIC: i32 = 1347109701;
pub const BCRYPT_ECDH_PUBLIC_P256_MAGIC: i32 = 827016005;
pub const BCRYPT_ECDH_PUBLIC_P384_MAGIC: i32 = 860570437;
pub const BCRYPT_ECDH_PUBLIC_P521_MAGIC: i32 = 894124869;
pub const BCRYPT_ECDSA_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDSA");
pub const BCRYPT_ECDSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 241 as _;
pub const BCRYPT_ECDSA_P256_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDSA_P256");
pub const BCRYPT_ECDSA_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 737 as _;
pub const BCRYPT_ECDSA_P384_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDSA_P384");
pub const BCRYPT_ECDSA_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 753 as _;
pub const BCRYPT_ECDSA_P521_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ECDSA_P521");
pub const BCRYPT_ECDSA_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = 769 as _;
pub const BCRYPT_ECDSA_PRIVATE_GENERIC_MAGIC: i32 = 1447314245;
pub const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: i32 = 844317509;
pub const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: i32 = 877871941;
pub const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: i32 = 911426373;
pub const BCRYPT_ECDSA_PUBLIC_GENERIC_MAGIC: i32 = 1346650949;
pub const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: i32 = 827540293;
pub const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: i32 = 861094725;
pub const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: i32 = 894649157;
pub const BCRYPT_EFFECTIVE_KEY_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("EffectiveKeyLength");
pub const BCRYPT_ENABLE_INCOMPATIBLE_FIPS_CHECKS: i32 = 256;
pub const BCRYPT_EXTENDED_KEYSIZE: i32 = 128;
pub const BCRYPT_FUNCTION_NAME_STRING: windows_sys::core::PCWSTR = windows_sys::core::w!("FunctionNameString");
pub const BCRYPT_GENERATE_IV: i32 = 32;
pub const BCRYPT_GLOBAL_PARAMETERS: windows_sys::core::PCWSTR = windows_sys::core::w!("SecretAgreementParam");
pub type BCRYPT_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_HASH_BLOCK_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("HashBlockLength");
pub const BCRYPT_HASH_DONT_RESET_FLAG: i32 = 1;
pub type BCRYPT_HASH_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_HASH_INTERFACE: i32 = 2;
pub const BCRYPT_HASH_INTERFACE_MAJORVERSION_2: i32 = 2;
pub const BCRYPT_HASH_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("HashDigestLength");
pub const BCRYPT_HASH_OID_LIST: windows_sys::core::PCWSTR = windows_sys::core::w!("HashOIDList");
pub const BCRYPT_HASH_OPERATION: i32 = 2;
pub const BCRYPT_HASH_OPERATION_FINISH_HASH: BCRYPT_HASH_OPERATION_TYPE = 2;
pub const BCRYPT_HASH_OPERATION_HASH_DATA: BCRYPT_HASH_OPERATION_TYPE = 1;
pub type BCRYPT_HASH_OPERATION_TYPE = i32;
pub const BCRYPT_HASH_REUSABLE_FLAG: i32 = 32;
pub const BCRYPT_HKDF_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("HKDF");
pub const BCRYPT_HKDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 913 as _;
pub const BCRYPT_HKDF_HASH_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("HkdfHashAlgorithm");
pub const BCRYPT_HKDF_PRK_AND_FINALIZE: windows_sys::core::PCWSTR = windows_sys::core::w!("HkdfPrkAndFinalize");
pub const BCRYPT_HKDF_SALT_AND_FINALIZE: windows_sys::core::PCWSTR = windows_sys::core::w!("HkdfSaltAndFinalize");
pub const BCRYPT_HMAC_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 289 as _;
pub const BCRYPT_HMAC_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 305 as _;
pub const BCRYPT_HMAC_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = 145 as _;
pub const BCRYPT_HMAC_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = 161 as _;
pub const BCRYPT_HMAC_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 177 as _;
pub const BCRYPT_HMAC_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 193 as _;
pub const BCRYPT_HMAC_SHA3_256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 993 as _;
pub const BCRYPT_HMAC_SHA3_384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1009 as _;
pub const BCRYPT_HMAC_SHA3_512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1025 as _;
pub const BCRYPT_HMAC_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 209 as _;
pub const BCRYPT_INITIALIZATION_VECTOR: windows_sys::core::PCWSTR = windows_sys::core::w!("IV");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_INTERFACE_VERSION {
    pub MajorVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
}
pub const BCRYPT_IS_IFX_TPM_WEAK_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("IsIfxTpmWeakKey");
pub const BCRYPT_IS_KEYED_HASH: windows_sys::core::PCWSTR = windows_sys::core::w!("IsKeyedHash");
pub const BCRYPT_IS_REUSABLE_HASH: windows_sys::core::PCWSTR = windows_sys::core::w!("IsReusableHash");
pub const BCRYPT_KDF_HASH: windows_sys::core::PCWSTR = windows_sys::core::w!("HASH");
pub const BCRYPT_KDF_HKDF: windows_sys::core::PCWSTR = windows_sys::core::w!("HKDF");
pub const BCRYPT_KDF_HMAC: windows_sys::core::PCWSTR = windows_sys::core::w!("HMAC");
pub const BCRYPT_KDF_RAW_SECRET: windows_sys::core::PCWSTR = windows_sys::core::w!("TRUNCATE");
pub const BCRYPT_KDF_SP80056A_CONCAT: windows_sys::core::PCWSTR = windows_sys::core::w!("SP800_56A_CONCAT");
pub const BCRYPT_KDF_TLS_PRF: windows_sys::core::PCWSTR = windows_sys::core::w!("TLS_PRF");
pub const BCRYPT_KEM_CIPHERTEXT_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("KEMCiphertextLength");
pub const BCRYPT_KEM_SHARED_SECRET_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("KEMSharedSecretLength");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_KEY_BLOB {
    pub Magic: super::ULONG,
}
pub const BCRYPT_KEY_DATA_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyDataBlob");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_KEY_DATA_BLOB_HEADER {
    pub dwMagic: super::ULONG,
    pub dwVersion: super::ULONG,
    pub cbKeyData: super::ULONG,
}
pub const BCRYPT_KEY_DATA_BLOB_MAGIC: i32 = 1296188491;
pub const BCRYPT_KEY_DATA_BLOB_VERSION1: i32 = 1;
pub const BCRYPT_KEY_DERIVATION_INTERFACE: i32 = 7;
pub const BCRYPT_KEY_DERIVATION_OPERATION: i32 = 64;
pub const BCRYPT_KEY_ENCAPSULATION_INTERFACE: i32 = 8;
pub const BCRYPT_KEY_ENCAPSULATION_OPERATION: i32 = 128;
pub type BCRYPT_KEY_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_KEY_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyLength");
pub const BCRYPT_KEY_LENGTHS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyLengths");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_KEY_LENGTHS_STRUCT {
    pub dwMinLength: super::ULONG,
    pub dwMaxLength: super::ULONG,
    pub dwIncrement: super::ULONG,
}
pub const BCRYPT_KEY_OBJECT_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyObjectLength");
pub const BCRYPT_KEY_STRENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyStrength");
pub const BCRYPT_KEY_VALIDATION_RANGE: i32 = 16;
pub const BCRYPT_KEY_VALIDATION_RANGE_AND_ORDER: i32 = 24;
pub const BCRYPT_KEY_VALIDATION_REGENERATE: i32 = 32;
pub const BCRYPT_KMAC128_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("KMAC128");
pub const BCRYPT_KMAC128_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1073 as _;
pub const BCRYPT_KMAC256_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("KMAC256");
pub const BCRYPT_KMAC256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1089 as _;
pub const BCRYPT_LMS_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("LMS");
pub const BCRYPT_LMS_PUBLIC_MAGIC: i32 = 1263553868;
pub const BCRYPT_MD2_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("MD2");
pub const BCRYPT_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1 as _;
pub const BCRYPT_MD4_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("MD4");
pub const BCRYPT_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 17 as _;
pub const BCRYPT_MD5_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("MD5");
pub const BCRYPT_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = 33 as _;
pub const BCRYPT_MESSAGE_BLOCK_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("MessageBlockLength");
pub const BCRYPT_MLDSA_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ML-DSA");
pub const BCRYPT_MLDSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1137 as _;
pub const BCRYPT_MLDSA_EXTERNAL_MU: i32 = 64;
pub const BCRYPT_MLDSA_PARAMETER_SET_44: windows_sys::core::PCWSTR = windows_sys::core::w!("44");
pub const BCRYPT_MLDSA_PARAMETER_SET_65: windows_sys::core::PCWSTR = windows_sys::core::w!("65");
pub const BCRYPT_MLDSA_PARAMETER_SET_87: windows_sys::core::PCWSTR = windows_sys::core::w!("87");
pub const BCRYPT_MLDSA_PRIVATE_MAGIC: i32 = 1263752004;
pub const BCRYPT_MLDSA_PRIVATE_SEED_MAGIC: i32 = 1397969732;
pub const BCRYPT_MLDSA_PUBLIC_MAGIC: i32 = 1263555396;
pub const BCRYPT_MLKEM_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("ML-KEM");
pub const BCRYPT_MLKEM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1153 as _;
pub const BCRYPT_MLKEM_DECAPSULATION_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("MLKEMPRIVATEBLOB");
pub const BCRYPT_MLKEM_ENCAPSULATION_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("MLKEMPUBLICBLOB");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_MLKEM_KEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbParameterSet: super::ULONG,
    pub cbKey: super::ULONG,
}
pub const BCRYPT_MLKEM_PARAMETER_SET_1024: windows_sys::core::PCWSTR = windows_sys::core::w!("1024");
pub const BCRYPT_MLKEM_PARAMETER_SET_512: windows_sys::core::PCWSTR = windows_sys::core::w!("512");
pub const BCRYPT_MLKEM_PARAMETER_SET_768: windows_sys::core::PCWSTR = windows_sys::core::w!("768");
pub const BCRYPT_MLKEM_PRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("MLKEMPRIVATEBLOB");
pub const BCRYPT_MLKEM_PRIVATE_MAGIC: i32 = 1380666445;
pub const BCRYPT_MLKEM_PRIVATE_SEED_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("MLKEMPRIVATESEEDBLOB");
pub const BCRYPT_MLKEM_PRIVATE_SEED_MAGIC: i32 = 1397443661;
pub const BCRYPT_MLKEM_PUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("MLKEMPUBLICBLOB");
pub const BCRYPT_MLKEM_PUBLIC_MAGIC: i32 = 1347112013;
pub const BCRYPT_MULTI_FLAG: i32 = 64;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_MULTI_HASH_OPERATION {
    pub iHash: super::ULONG,
    pub hashOperation: BCRYPT_HASH_OPERATION_TYPE,
    pub pbBuffer: super::PUCHAR,
    pub cbBuffer: super::ULONG,
}
pub const BCRYPT_MULTI_OBJECT_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("MultiObjectLength");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    pub cbPerObject: super::ULONG,
    pub cbPerElement: super::ULONG,
}
pub type BCRYPT_MULTI_OPERATION_TYPE = i32;
pub const BCRYPT_NO_CURVE_GENERATION_ALG_ID: ECC_CURVE_ALG_ID_ENUM = 0;
pub const BCRYPT_NO_KEY_VALIDATION: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_OAEP_PADDING_INFO {
    pub pszAlgId: windows_sys::core::PCWSTR,
    pub pbLabel: super::PUCHAR,
    pub cbLabel: super::ULONG,
}
pub const BCRYPT_OBJECT_ALIGNMENT: i32 = 16;
pub const BCRYPT_OBJECT_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("ObjectLength");
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_OID {
    pub cbOID: super::ULONG,
    pub pbOID: super::PUCHAR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_OID_LIST {
    pub dwOIDCount: super::ULONG,
    pub pOIDs: *mut BCRYPT_OID,
}
pub const BCRYPT_OPAQUE_KEY_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("OpaqueKeyBlob");
pub const BCRYPT_OPERATION_TYPE_HASH: BCRYPT_MULTI_OPERATION_TYPE = 1;
pub const BCRYPT_PADDING_SCHEMES: windows_sys::core::PCWSTR = windows_sys::core::w!("PaddingSchemes");
pub const BCRYPT_PAD_NONE: i32 = 1;
pub const BCRYPT_PAD_OAEP: i32 = 4;
pub const BCRYPT_PAD_PKCS1: i32 = 2;
pub const BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID: i32 = 16;
pub const BCRYPT_PAD_PQDSA: i32 = 32;
pub const BCRYPT_PAD_PSS: i32 = 8;
pub const BCRYPT_PARAMETER_SET_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ParameterSetName");
pub const BCRYPT_PBKDF2_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("PBKDF2");
pub const BCRYPT_PBKDF2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 817 as _;
pub const BCRYPT_PCP_PLATFORM_TYPE_PROPERTY: windows_sys::core::PCWSTR = windows_sys::core::w!("PCP_PLATFORM_TYPE");
pub const BCRYPT_PCP_PROVIDER_VERSION_PROPERTY: windows_sys::core::PCWSTR = windows_sys::core::w!("PCP_PROVIDER_VERSION");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_PKCS11_RSA_AES_WRAP_BLOB {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
    pub cbPaddingAlgId: super::ULONG,
    pub cbPaddingLabel: super::ULONG,
}
pub const BCRYPT_PKCS11_RSA_AES_WRAP_BLOB_MAGIC: i32 = 1464877394;
pub const BCRYPT_PKCS11_RSA_AES_WRAP_KEY_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("PKCS11RsaAesWrapBlob");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_PKCS1_PADDING_INFO {
    pub pszAlgId: windows_sys::core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_PQDSA_KEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbParameterSet: super::ULONG,
    pub cbKey: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_PQDSA_PADDING_INFO {
    pub pbCtx: super::PUCHAR,
    pub cbCtx: super::ULONG,
    pub pszPrehashAlgId: windows_sys::core::PCWSTR,
}
pub const BCRYPT_PQDSA_PRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("PQDSAPRIVATEBLOB");
pub const BCRYPT_PQDSA_PRIVATE_SEED_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("PQDSAPRIVATESEEDBLOB");
pub const BCRYPT_PQDSA_PUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("PQDSAPUBLICBLOB");
pub const BCRYPT_PRIMITIVE_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("PrimitiveType");
pub const BCRYPT_PRIVATE_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("PrivKeyVal");
pub const BCRYPT_PRIVATE_KEY_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("PRIVATEBLOB");
pub const BCRYPT_PRIVATE_KEY_FLAG: i32 = 2;
pub const BCRYPT_PROVIDER_HANDLE: windows_sys::core::PCWSTR = windows_sys::core::w!("ProviderHandle");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_PROVIDER_NAME {
    pub pszProviderName: windows_sys::core::PWSTR,
}
pub const BCRYPT_PROV_DISPATCH: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_PSS_PADDING_INFO {
    pub pszAlgId: windows_sys::core::PCWSTR,
    pub cbSalt: super::ULONG,
}
pub const BCRYPT_PUBLIC_KEY_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("PUBLICBLOB");
pub const BCRYPT_PUBLIC_KEY_FLAG: i32 = 1;
pub const BCRYPT_PUBLIC_KEY_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("PublicKeyLength");
pub const BCRYPT_RC2_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("RC2");
pub const BCRYPT_RC2_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 593 as _;
pub const BCRYPT_RC2_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 625 as _;
pub const BCRYPT_RC2_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 609 as _;
pub const BCRYPT_RC4_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("RC4");
pub const BCRYPT_RC4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 113 as _;
pub const BCRYPT_RNG_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("RNG");
pub const BCRYPT_RNG_ALG_HANDLE: BCRYPT_ALG_HANDLE = 129 as _;
pub const BCRYPT_RNG_DUAL_EC_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("DUALECRNG");
pub const BCRYPT_RNG_FIPS186_DSA_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("FIPS186DSARNG");
pub const BCRYPT_RNG_INTERFACE: i32 = 6;
pub const BCRYPT_RNG_OPERATION: i32 = 32;
pub const BCRYPT_RNG_USE_ENTROPY_IN_BUFFER: i32 = 1;
pub const BCRYPT_RSAFULLPRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("RSAFULLPRIVATEBLOB");
pub const BCRYPT_RSAFULLPRIVATE_MAGIC: i32 = 859919186;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BCRYPT_RSAKEY_BLOB {
    pub Magic: super::ULONG,
    pub BitLength: super::ULONG,
    pub cbPublicExp: super::ULONG,
    pub cbModulus: super::ULONG,
    pub cbPrime1: super::ULONG,
    pub cbPrime2: super::ULONG,
}
pub const BCRYPT_RSAPRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("RSAPRIVATEBLOB");
pub const BCRYPT_RSAPRIVATE_MAGIC: i32 = 843141970;
pub const BCRYPT_RSAPUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("RSAPUBLICBLOB");
pub const BCRYPT_RSAPUBLIC_MAGIC: i32 = 826364754;
pub const BCRYPT_RSA_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("RSA");
pub const BCRYPT_RSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 225 as _;
pub const BCRYPT_RSA_SIGN_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("RSA_SIGN");
pub const BCRYPT_RSA_SIGN_ALG_HANDLE: BCRYPT_ALG_HANDLE = 785 as _;
pub const BCRYPT_SECRET_AGREEMENT_INTERFACE: i32 = 4;
pub const BCRYPT_SECRET_AGREEMENT_OPERATION: i32 = 8;
pub type BCRYPT_SECRET_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_SHA1_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA1");
pub const BCRYPT_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = 49 as _;
pub const BCRYPT_SHA256_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA256");
pub const BCRYPT_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 65 as _;
pub const BCRYPT_SHA384_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA384");
pub const BCRYPT_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 81 as _;
pub const BCRYPT_SHA3_256_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA3-256");
pub const BCRYPT_SHA3_256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 945 as _;
pub const BCRYPT_SHA3_384_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA3-384");
pub const BCRYPT_SHA3_384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 961 as _;
pub const BCRYPT_SHA3_512_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA3-512");
pub const BCRYPT_SHA3_512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 977 as _;
pub const BCRYPT_SHA512_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA512");
pub const BCRYPT_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 97 as _;
pub const BCRYPT_SHAKE128_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHAKE128");
pub const BCRYPT_SHAKE128_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1105 as _;
pub const BCRYPT_SHAKE256_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SHAKE256");
pub const BCRYPT_SHAKE256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1121 as _;
pub const BCRYPT_SIGNATURE_INTERFACE: i32 = 5;
pub const BCRYPT_SIGNATURE_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("SignatureLength");
pub const BCRYPT_SIGNATURE_OPERATION: i32 = 16;
pub const BCRYPT_SLHDSA_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SLH-DSA");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_128F: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA2-128f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_128S: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA2-128s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_192F: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA2-192f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_192S: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA2-192s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_256F: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA2-256f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_256S: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA2-256s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_128F: windows_sys::core::PCWSTR = windows_sys::core::w!("SHAKE-128f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_128S: windows_sys::core::PCWSTR = windows_sys::core::w!("SHAKE-128s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_192F: windows_sys::core::PCWSTR = windows_sys::core::w!("SHAKE-192f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_192S: windows_sys::core::PCWSTR = windows_sys::core::w!("SHAKE-192s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_256F: windows_sys::core::PCWSTR = windows_sys::core::w!("SHAKE-256f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_256S: windows_sys::core::PCWSTR = windows_sys::core::w!("SHAKE-256s");
pub const BCRYPT_SLHDSA_PRIVATE_MAGIC: i32 = 1263749203;
pub const BCRYPT_SLHDSA_PUBLIC_MAGIC: i32 = 1263552595;
pub const BCRYPT_SP800108_CTR_HMAC_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SP800_108_CTR_HMAC");
pub const BCRYPT_SP800108_CTR_HMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 833 as _;
pub const BCRYPT_SP80056A_CONCAT_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("SP800_56A_CONCAT");
pub const BCRYPT_SP80056A_CONCAT_ALG_HANDLE: BCRYPT_ALG_HANDLE = 849 as _;
pub const BCRYPT_SUPPORTED_PAD_OAEP: i32 = 8;
pub const BCRYPT_SUPPORTED_PAD_PKCS1_ENC: i32 = 2;
pub const BCRYPT_SUPPORTED_PAD_PKCS1_SIG: i32 = 4;
pub const BCRYPT_SUPPORTED_PAD_PSS: i32 = 16;
pub const BCRYPT_SUPPORTED_PAD_ROUTER: i32 = 1;
pub const BCRYPT_TLS1_1_KDF_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("TLS1_1_KDF");
pub const BCRYPT_TLS1_1_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 865 as _;
pub const BCRYPT_TLS1_2_KDF_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("TLS1_2_KDF");
pub const BCRYPT_TLS1_2_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 881 as _;
pub const BCRYPT_TLS_CBC_HMAC_VERIFY_FLAG: i32 = 4;
pub const BCRYPT_USE_SYSTEM_PREFERRED_RNG: i32 = 2;
pub const BCRYPT_XMSS_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("XMSS");
pub const BCRYPT_XMSS_PUBLIC_MAGIC: i32 = 1263553880;
pub const BCRYPT_XTS_AES_ALGORITHM: windows_sys::core::PCWSTR = windows_sys::core::w!("XTS-AES");
pub const BCRYPT_XTS_AES_ALG_HANDLE: BCRYPT_ALG_HANDLE = 897 as _;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct BCryptBuffer {
    pub cbBuffer: super::ULONG,
    pub BufferType: super::ULONG,
    pub pvBuffer: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct BCryptBufferDesc {
    pub ulVersion: super::ULONG,
    pub cBuffers: super::ULONG,
    pub pBuffers: PBCryptBuffer,
}
pub const CRYPT_ALL_FUNCTIONS: i32 = 1;
pub const CRYPT_ALL_PROVIDERS: i32 = 2;
pub const CRYPT_ANY: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_CONTEXTS {
    pub cContexts: super::ULONG,
    pub rgpszContexts: *mut windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_CONTEXT_CONFIG {
    pub dwFlags: super::ULONG,
    pub dwReserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_CONTEXT_FUNCTIONS {
    pub cFunctions: super::ULONG,
    pub rgpszFunctions: *mut windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_CONTEXT_FUNCTION_CONFIG {
    pub dwFlags: super::ULONG,
    pub dwReserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    pub cProviders: super::ULONG,
    pub rgpszProviders: *mut windows_sys::core::PWSTR,
}
pub const CRYPT_DEFAULT_CONTEXT: windows_sys::core::PCWSTR = windows_sys::core::w!("Default");
pub const CRYPT_DOMAIN: i32 = 2;
pub const CRYPT_EXCLUSIVE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_IMAGE_REF {
    pub pszImage: windows_sys::core::PWSTR,
    pub dwFlags: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_IMAGE_REG {
    pub pszImage: windows_sys::core::PWSTR,
    pub cInterfaces: super::ULONG,
    pub rgpInterfaces: *mut PCRYPT_INTERFACE_REG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_INTERFACE_REG {
    pub dwInterface: super::ULONG,
    pub dwFlags: super::ULONG,
    pub cFunctions: super::ULONG,
    pub rgpszFunctions: *mut windows_sys::core::PWSTR,
}
pub const CRYPT_KM: i32 = 2;
pub const CRYPT_LOCAL: i32 = 1;
pub const CRYPT_MIN_DEPENDENCIES: i32 = 1;
pub const CRYPT_MM: i32 = 3;
pub const CRYPT_OVERRIDE: i32 = 65536;
pub const CRYPT_OVERWRITE: i32 = 1;
pub const CRYPT_PRIORITY_BOTTOM: u32 = 4294967295;
pub const CRYPT_PRIORITY_TOP: i32 = 0;
pub const CRYPT_PROCESS_ISOLATE: i32 = 65536;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_PROPERTY_REF {
    pub pszProperty: windows_sys::core::PWSTR,
    pub cbValue: super::ULONG,
    pub pbValue: super::PUCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_PROVIDERS {
    pub cProviders: super::ULONG,
    pub rgpszProviders: *mut windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_PROVIDER_REF {
    pub dwInterface: super::ULONG,
    pub pszFunction: windows_sys::core::PWSTR,
    pub pszProvider: windows_sys::core::PWSTR,
    pub cProperties: super::ULONG,
    pub rgpProperties: *mut PCRYPT_PROPERTY_REF,
    pub pUM: PCRYPT_IMAGE_REF,
    pub pKM: PCRYPT_IMAGE_REF,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_PROVIDER_REFS {
    pub cProviders: super::ULONG,
    pub rgpProviders: *mut PCRYPT_PROVIDER_REF,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_PROVIDER_REG {
    pub cAliases: super::ULONG,
    pub rgpszAliases: *mut windows_sys::core::PWSTR,
    pub pUM: PCRYPT_IMAGE_REG,
    pub pKM: PCRYPT_IMAGE_REG,
}
pub const CRYPT_UM: i32 = 1;
pub type DSAFIPSVERSION_ENUM = i32;
pub const DSA_FIPS186_2: DSAFIPSVERSION_ENUM = 0;
pub const DSA_FIPS186_3: DSAFIPSVERSION_ENUM = 1;
pub const DSA_HASH_ALGORITHM_SHA1: HASHALGORITHM_ENUM = 0;
pub const DSA_HASH_ALGORITHM_SHA256: HASHALGORITHM_ENUM = 1;
pub const DSA_HASH_ALGORITHM_SHA512: HASHALGORITHM_ENUM = 2;
pub type ECC_CURVE_ALG_ID_ENUM = i32;
pub type ECC_CURVE_TYPE_ENUM = i32;
pub type HASHALGORITHM_ENUM = i32;
pub const KDF_ALGORITHMID: i32 = 8;
pub const KDF_CONTEXT: i32 = 14;
pub const KDF_GENERIC_PARAMETER: i32 = 17;
pub const KDF_HASH_ALGORITHM: i32 = 0;
pub const KDF_HKDF_INFO: i32 = 20;
pub const KDF_HKDF_SALT: i32 = 19;
pub const KDF_HMAC_KEY: i32 = 3;
pub const KDF_ITERATION_COUNT: i32 = 16;
pub const KDF_KEYBITLENGTH: i32 = 18;
pub const KDF_LABEL: i32 = 13;
pub const KDF_PARTYUINFO: i32 = 9;
pub const KDF_PARTYVINFO: i32 = 10;
pub const KDF_SALT: i32 = 15;
pub const KDF_SECRET_APPEND: i32 = 2;
pub const KDF_SECRET_HANDLE: i32 = 6;
pub const KDF_SECRET_PREPEND: i32 = 1;
pub const KDF_SUPPPRIVINFO: i32 = 12;
pub const KDF_SUPPPUBINFO: i32 = 11;
pub const KDF_TLS_PRF_LABEL: i32 = 4;
pub const KDF_TLS_PRF_PROTOCOL: i32 = 7;
pub const KDF_TLS_PRF_SEED: i32 = 5;
pub const KDF_USE_SECRET_AS_HMAC_KEY_FLAG: i32 = 1;
pub const LEGACY_DH_PRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("CAPIDHPRIVATEBLOB");
pub const LEGACY_DH_PUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("CAPIDHPUBLICBLOB");
pub const LEGACY_DSA_PRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("CAPIDSAPRIVATEBLOB");
pub const LEGACY_DSA_PUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("CAPIDSAPUBLICBLOB");
pub const LEGACY_DSA_V2_PRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("V2CAPIDSAPRIVATEBLOB");
pub const LEGACY_DSA_V2_PUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("V2CAPIDSAPUBLICBLOB");
pub const LEGACY_RSAPRIVATE_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("CAPIPRIVATEBLOB");
pub const LEGACY_RSAPUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("CAPIPUBLICBLOB");
pub const MAX_HYBRID_KEY_EXCHANGE_BLOBS: i32 = 4;
pub const MS_PLATFORM_CRYPTO_PROVIDER: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Platform Crypto Provider");
pub const MS_PRIMITIVE_PROVIDER: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Primitive Provider");
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO = *mut BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO;
#[cfg(feature = "shared")]
pub type PBCRYPT_DH_KEY_BLOB = *mut BCRYPT_DH_KEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_DSA_KEY_BLOB = *mut BCRYPT_DSA_KEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_DSA_KEY_BLOB_V2 = *mut BCRYPT_DSA_KEY_BLOB_V2;
#[cfg(feature = "shared")]
pub type PBCRYPT_ECCFULLKEY_BLOB = *mut BCRYPT_ECCFULLKEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_ECCKEY_BLOB = *mut BCRYPT_ECCKEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_INTERFACE_VERSION = *mut BCRYPT_INTERFACE_VERSION;
#[cfg(feature = "shared")]
pub type PBCRYPT_KEY_DATA_BLOB_HEADER = *mut BCRYPT_KEY_DATA_BLOB_HEADER;
#[cfg(feature = "shared")]
pub type PBCRYPT_MLKEM_KEY_BLOB = *mut BCRYPT_MLKEM_KEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_PKCS11_RSA_AES_WRAP_BLOB = *mut BCRYPT_PKCS11_RSA_AES_WRAP_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_PQDSA_KEY_BLOB = *mut BCRYPT_PQDSA_KEY_BLOB;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBCryptBuffer = *mut BCryptBuffer;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBCryptBufferDesc = *mut BCryptBufferDesc;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXTS = *mut CRYPT_CONTEXTS;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXT_CONFIG = *mut CRYPT_CONTEXT_CONFIG;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXT_FUNCTIONS = *mut CRYPT_CONTEXT_FUNCTIONS;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXT_FUNCTION_CONFIG = *mut CRYPT_CONTEXT_FUNCTION_CONFIG;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXT_FUNCTION_PROVIDERS = *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS;
#[cfg(feature = "shared")]
pub type PCRYPT_IMAGE_REF = *mut CRYPT_IMAGE_REF;
#[cfg(feature = "shared")]
pub type PCRYPT_IMAGE_REG = *mut CRYPT_IMAGE_REG;
#[cfg(feature = "shared")]
pub type PCRYPT_INTERFACE_REG = *mut CRYPT_INTERFACE_REG;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCRYPT_PROPERTY_REF = *mut CRYPT_PROPERTY_REF;
#[cfg(feature = "shared")]
pub type PCRYPT_PROVIDERS = *mut CRYPT_PROVIDERS;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCRYPT_PROVIDER_REF = *mut CRYPT_PROVIDER_REF;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCRYPT_PROVIDER_REFS = *mut CRYPT_PROVIDER_REFS;
#[cfg(feature = "shared")]
pub type PCRYPT_PROVIDER_REG = *mut CRYPT_PROVIDER_REG;
#[cfg(feature = "shared")]
pub type PSSL_ECCKEY_BLOB = *mut SSL_ECCKEY_BLOB;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PTLS_HYBRID_KEYEXCHANGE_BLOB = *mut TLS_HYBRID_KEYEXCHANGE_BLOB;
#[cfg(feature = "shared")]
pub type PTLS_KEM_CIPHERTEXT_BLOB = *mut TLS_KEM_CIPHERTEXT_BLOB;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SSL_ECCKEY_BLOB {
    pub dwCurveType: super::ULONG,
    pub cbKey: super::ULONG,
}
pub const SSL_ECCPUBLIC_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("SSLECCPUBLICBLOB");
pub const TLS_13_PRE_SHARED_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("TLS13PRESHAREDKEY");
pub const TLS_HYBRID_KEYEXCHANGE: windows_sys::core::PCWSTR = windows_sys::core::w!("TLS_HYBRID_KEYEXCHANGE");
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct TLS_HYBRID_KEYEXCHANGE_BLOB {
    pub dwMagic: super::ULONG,
    pub dwKeyType: super::ULONG,
    pub NumBlobs: super::UCHAR,
    pub rgwszBlobTypes: [windows_sys::core::PCWSTR; 4],
    pub rgbBlobs: [super::PUCHAR; 4],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for TLS_HYBRID_KEYEXCHANGE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TLS_HYBRID_KEYEXCHANGE_MAGIC: i32 = 1213418580;
pub const TLS_KEM_CIPHERTEXT: windows_sys::core::PCWSTR = windows_sys::core::w!("TLS_KEM_CIPHER_TEXT");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TLS_KEM_CIPHERTEXT_BLOB {
    pub dwMagic: super::ULONG,
    pub cbLength: super::ULONG,
}
pub const TLS_KEM_CIPHERTEXT_MAGIC: i32 = 1129008205;
