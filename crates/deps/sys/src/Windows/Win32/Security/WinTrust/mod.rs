#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPersonalTrustDBDialog(hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPersonalTrustDBDialogEx(hwndparent: super::super::Foundation::HWND, dwflags: u32, pvreserved: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperCertCheckValidSignature(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WTHelperCertIsSelfSigned(dwencoding: u32, pcert: *mut super::Cryptography::CERT_INFO) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WTHelperGetProvCertFromChain(psgnr: *mut CRYPT_PROVIDER_SGNR, idxcert: u32) -> *mut CRYPT_PROVIDER_CERT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperGetProvPrivateDataFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, pgproviderid: *mut ::windows_sys::core::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperGetProvSignerFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: super::super::Foundation::BOOL, idxcountersigner: u32) -> *mut CRYPT_PROVIDER_SGNR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperProvDataFromStateData(hstatedata: super::super::Foundation::HANDLE) -> *mut CRYPT_PROVIDER_DATA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinVerifyTrust(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows_sys::core::GUID, pwvtdata: *mut ::core::ffi::c_void) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WinVerifyTrustEx(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows_sys::core::GUID, pwintrustdata: *mut WINTRUST_DATA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustAddActionID(pgactionid: *const ::windows_sys::core::GUID, fdwflags: u32, psprovinfo: *const CRYPT_REGISTER_ACTIONID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustAddDefaultForUsage(pszusageoid: super::super::Foundation::PSTR, psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustGetDefaultForUsage(dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION, pszusageoid: super::super::Foundation::PSTR, psusage: *mut CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL;
    pub fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WintrustLoadFunctionPointers(pgactionid: *mut ::windows_sys::core::GUID, ppfns: *mut CRYPT_PROVIDER_FUNCTIONS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustRemoveActionID(pgactionid: *const ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes: super::super::Foundation::BOOL);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustSetRegPolicyFlags(dwpolicyflags: WINTRUST_POLICY_FLAGS) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CAT_MEMBERINFO(i32);
#[repr(C)]
pub struct CAT_MEMBERINFO2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct CAT_NAMEVALUE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct CONFIG_CI_PROV_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONFIG_CI_PROV_INFO_RESULT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct CRYPT_PROVIDER_CERT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[repr(C)]
pub struct CRYPT_PROVIDER_DATA(i32);
#[repr(C)]
pub struct CRYPT_PROVIDER_DEFUSAGE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[repr(C)]
pub struct CRYPT_PROVIDER_FUNCTIONS(i32);
#[repr(C)]
pub struct CRYPT_PROVIDER_PRIVDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CRYPT_PROVIDER_REGDEFUSAGE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct CRYPT_PROVIDER_SGNR(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct CRYPT_PROVIDER_SIGSTATE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CRYPT_PROVUI_DATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[repr(C)]
pub struct CRYPT_PROVUI_FUNCS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CRYPT_REGISTER_ACTIONID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CRYPT_TRUST_REG_ENTRY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct DRIVER_VER_INFO(i32);
#[repr(C)]
pub struct DRIVER_VER_MAJORMINOR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct INTENT_TO_SEAL_ATTRIBUTE(i32);
#[repr(C)]
pub struct PFN_ALLOCANDFILLDEFUSAGE(i32);
#[repr(C)]
pub struct PFN_CPD_ADD_CERT(i32);
#[repr(C)]
pub struct PFN_CPD_ADD_PRIVDATA(i32);
#[repr(C)]
pub struct PFN_CPD_ADD_SGNR(i32);
#[repr(C)]
pub struct PFN_CPD_ADD_STORE(i32);
#[repr(C)]
pub struct PFN_CPD_MEM_ALLOC(i32);
#[repr(C)]
pub struct PFN_CPD_MEM_FREE(i32);
#[repr(C)]
pub struct PFN_FREEDEFUSAGE(i32);
#[repr(C)]
pub struct PFN_PROVIDER_CERTCHKPOLICY_CALL(i32);
#[repr(C)]
pub struct PFN_PROVIDER_CERTTRUST_CALL(i32);
#[repr(C)]
pub struct PFN_PROVIDER_CLEANUP_CALL(i32);
#[repr(C)]
pub struct PFN_PROVIDER_FINALPOLICY_CALL(i32);
#[repr(C)]
pub struct PFN_PROVIDER_INIT_CALL(i32);
#[repr(C)]
pub struct PFN_PROVIDER_OBJTRUST_CALL(i32);
#[repr(C)]
pub struct PFN_PROVIDER_SIGTRUST_CALL(i32);
#[repr(C)]
pub struct PFN_PROVIDER_TESTFINALPOLICY_CALL(i32);
#[repr(C)]
pub struct PFN_PROVUI_CALL(i32);
#[repr(C)]
pub struct PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[repr(C)]
pub struct PROVDATA_SIP(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct SEALING_SIGNATURE_ATTRIBUTE(i32);
#[cfg(feature = "Win32_Security_Cryptography")]
#[repr(C)]
pub struct SEALING_TIMESTAMP_ATTRIBUTE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPC_FINANCIAL_CRITERIA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct SPC_IMAGE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct SPC_INDIRECT_DATA_CONTENT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct SPC_LINK(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct SPC_PE_IMAGE_DATA(i32);
#[cfg(feature = "Win32_Security_Cryptography")]
#[repr(C)]
pub struct SPC_SERIALIZED_OBJECT(i32);
#[repr(C)]
pub struct SPC_SIGINFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct SPC_SP_AGENCY_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct SPC_SP_OPUS_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPC_STATEMENT_TYPE(i32);
pub const SPC_UUID_LENGTH: u32 = 16u32;
pub const TRUSTERROR_MAX_STEPS: u32 = 38u32;
pub const TRUSTERROR_STEP_CATALOGFILE: u32 = 6u32;
pub const TRUSTERROR_STEP_CERTSTORE: u32 = 7u32;
pub const TRUSTERROR_STEP_FILEIO: u32 = 2u32;
pub const TRUSTERROR_STEP_FINAL_CERTCHKPROV: u32 = 35u32;
pub const TRUSTERROR_STEP_FINAL_CERTPROV: u32 = 34u32;
pub const TRUSTERROR_STEP_FINAL_INITPROV: u32 = 31u32;
pub const TRUSTERROR_STEP_FINAL_OBJPROV: u32 = 32u32;
pub const TRUSTERROR_STEP_FINAL_POLICYPROV: u32 = 36u32;
pub const TRUSTERROR_STEP_FINAL_SIGPROV: u32 = 33u32;
pub const TRUSTERROR_STEP_FINAL_UIPROV: u32 = 37u32;
pub const TRUSTERROR_STEP_FINAL_WVTINIT: u32 = 30u32;
pub const TRUSTERROR_STEP_MESSAGE: u32 = 8u32;
pub const TRUSTERROR_STEP_MSG_CERTCHAIN: u32 = 15u32;
pub const TRUSTERROR_STEP_MSG_COUNTERSIGCERT: u32 = 17u32;
pub const TRUSTERROR_STEP_MSG_COUNTERSIGINFO: u32 = 16u32;
pub const TRUSTERROR_STEP_MSG_INNERCNT: u32 = 11u32;
pub const TRUSTERROR_STEP_MSG_INNERCNTTYPE: u32 = 10u32;
pub const TRUSTERROR_STEP_MSG_SIGNERCERT: u32 = 14u32;
pub const TRUSTERROR_STEP_MSG_SIGNERCOUNT: u32 = 9u32;
pub const TRUSTERROR_STEP_MSG_SIGNERINFO: u32 = 13u32;
pub const TRUSTERROR_STEP_MSG_STORE: u32 = 12u32;
pub const TRUSTERROR_STEP_SIP: u32 = 3u32;
pub const TRUSTERROR_STEP_SIPSUBJINFO: u32 = 5u32;
pub const TRUSTERROR_STEP_VERIFY_MSGHASH: u32 = 18u32;
pub const TRUSTERROR_STEP_VERIFY_MSGINDIRECTDATA: u32 = 19u32;
pub const TRUSTERROR_STEP_WVTPARAMS: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINTRUST_BLOB_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WINTRUST_CATALOG_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WINTRUST_CERT_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WINTRUST_DATA(i32);
#[repr(transparent)]
pub struct WINTRUST_DATA_REVOCATION_CHECKS(pub u32);
pub const WTD_REVOKE_NONE: WINTRUST_DATA_REVOCATION_CHECKS = WINTRUST_DATA_REVOCATION_CHECKS(0u32);
pub const WTD_REVOKE_WHOLECHAIN: WINTRUST_DATA_REVOCATION_CHECKS = WINTRUST_DATA_REVOCATION_CHECKS(1u32);
#[repr(transparent)]
pub struct WINTRUST_DATA_STATE_ACTION(pub u32);
pub const WTD_STATEACTION_IGNORE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(0u32);
pub const WTD_STATEACTION_VERIFY: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(1u32);
pub const WTD_STATEACTION_CLOSE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(2u32);
pub const WTD_STATEACTION_AUTO_CACHE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(3u32);
pub const WTD_STATEACTION_AUTO_CACHE_FLUSH: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(4u32);
#[repr(transparent)]
pub struct WINTRUST_DATA_UICHOICE(pub u32);
pub const WTD_UI_ALL: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(1u32);
pub const WTD_UI_NONE: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(2u32);
pub const WTD_UI_NOBAD: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(3u32);
pub const WTD_UI_NOGOOD: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(4u32);
#[repr(transparent)]
pub struct WINTRUST_DATA_UICONTEXT(pub u32);
pub const WTD_UICONTEXT_EXECUTE: WINTRUST_DATA_UICONTEXT = WINTRUST_DATA_UICONTEXT(0u32);
pub const WTD_UICONTEXT_INSTALL: WINTRUST_DATA_UICONTEXT = WINTRUST_DATA_UICONTEXT(1u32);
#[repr(transparent)]
pub struct WINTRUST_DATA_UNION_CHOICE(pub u32);
pub const WTD_CHOICE_FILE: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(1u32);
pub const WTD_CHOICE_CATALOG: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(2u32);
pub const WTD_CHOICE_BLOB: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(3u32);
pub const WTD_CHOICE_SIGNER: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(4u32);
pub const WTD_CHOICE_CERT: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(5u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINTRUST_FILE_INFO(i32);
#[repr(transparent)]
pub struct WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(pub u32);
pub const DWACTION_ALLOCANDFILL: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(1u32);
pub const DWACTION_FREE: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(2u32);
pub const WINTRUST_MAX_HASH_BYTES_TO_MAP_DEFAULT: u32 = 1048576u32;
pub const WINTRUST_MAX_HEADER_BYTES_TO_MAP_DEFAULT: u32 = 10485760u32;
#[repr(transparent)]
pub struct WINTRUST_POLICY_FLAGS(pub u32);
pub const WTPF_TRUSTTEST: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(32u32);
pub const WTPF_TESTCANBEVALID: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(128u32);
pub const WTPF_IGNOREEXPIRATION: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(256u32);
pub const WTPF_IGNOREREVOKATION: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(512u32);
pub const WTPF_OFFLINEOK_IND: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(1024u32);
pub const WTPF_OFFLINEOK_COM: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(2048u32);
pub const WTPF_OFFLINEOKNBU_IND: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(4096u32);
pub const WTPF_OFFLINEOKNBU_COM: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(8192u32);
pub const WTPF_VERIFY_V1_OFF: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(65536u32);
pub const WTPF_IGNOREREVOCATIONONTS: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(131072u32);
pub const WTPF_ALLOWONLYPERTRUST: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(262144u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WINTRUST_SGNR_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WINTRUST_SIGNATURE_SETTINGS(i32);
#[repr(transparent)]
pub struct WINTRUST_SIGNATURE_SETTINGS_FLAGS(pub u32);
pub const WSS_VERIFY_SPECIFIC: WINTRUST_SIGNATURE_SETTINGS_FLAGS = WINTRUST_SIGNATURE_SETTINGS_FLAGS(1u32);
pub const WSS_GET_SECONDARY_SIG_COUNT: WINTRUST_SIGNATURE_SETTINGS_FLAGS = WINTRUST_SIGNATURE_SETTINGS_FLAGS(2u32);
#[repr(C)]
pub struct WIN_CERTIFICATE(i32);
pub const WIN_CERT_REVISION_1_0: u32 = 256u32;
pub const WIN_CERT_REVISION_2_0: u32 = 512u32;
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u32 = 2u32;
pub const WIN_CERT_TYPE_RESERVED_1: u32 = 3u32;
pub const WIN_CERT_TYPE_TS_STACK_SIGNED: u32 = 4u32;
pub const WIN_CERT_TYPE_X509: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WIN_SPUB_TRUSTED_PUBLISHER_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT(i32);
#[repr(C)]
pub struct WIN_TRUST_ACTDATA_SUBJECT_ONLY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WIN_TRUST_SUBJECT_FILE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WIN_TRUST_SUBJECT_FILE_AND_DISPLAY(i32);
pub const WSS_CERTTRUST_SUPPORT: u32 = 4u32;
pub const WSS_INPUT_FLAG_MASK: u32 = 7u32;
pub const WSS_OBJTRUST_SUPPORT: u32 = 1u32;
pub const WSS_OUTPUT_FLAG_MASK: u32 = 3758096384u32;
pub const WSS_OUT_FILE_SUPPORTS_SEAL: u32 = 536870912u32;
pub const WSS_OUT_HAS_SEALING_INTENT: u32 = 1073741824u32;
pub const WSS_OUT_SEALING_STATUS_VERIFIED: u32 = 2147483648u32;
pub const WSS_SIGTRUST_SUPPORT: u32 = 2u32;
pub const WSS_VERIFY_SEALING: u32 = 4u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WTD_GENERIC_CHAIN_POLICY_CREATE_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[repr(C)]
pub struct WTD_GENERIC_CHAIN_POLICY_DATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO(i32);
pub const WT_ADD_ACTION_ID_RET_RESULT_FLAG: u32 = 1u32;
pub const WT_CURRENT_VERSION: u32 = 512u32;
pub const WT_TRUSTDBDIALOG_NO_UI_FLAG: u32 = 1u32;
pub const WT_TRUSTDBDIALOG_ONLY_PUB_TAB_FLAG: u32 = 2u32;
pub const WT_TRUSTDBDIALOG_WRITE_IEAK_STORE_FLAG: u32 = 512u32;
pub const WT_TRUSTDBDIALOG_WRITE_LEGACY_REG_FLAG: u32 = 256u32;
