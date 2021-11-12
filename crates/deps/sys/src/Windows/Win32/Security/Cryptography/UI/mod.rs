#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSelectionGetSerializedBlob(pcsi: *const CERT_SELECTUI_INPUT, ppoutbuffer: *mut *mut ::core::ffi::c_void, puloutbuffersize: *mut u32) -> ::windows_sys::core::HRESULT;
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
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const ACTION_REVOCATION_DEFAULT_CACHE: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const ACTION_REVOCATION_DEFAULT_ONLINE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERTVIEW_CRYPTUI_LPARAM: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_CREDENTIAL_PROVIDER_ID: i32 = -509i32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_DISTRUST_ADD_CA_CERT: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_DISTRUST_ADD_LEAF_CERT: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_DISTRUST_CA_CERT: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_DISTRUST_LEAF_CERT: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_SELECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_TRUST_ADD_CA_CERT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_TRUST_ADD_LEAF_CERT: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_TRUST_CA_CERT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_DISPWELL_TRUST_LEAF_CERT: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_INCLUDE_V1_CERTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_ISSUER_CERTS_ONLY: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_KEY_EXISTS: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_LEAF_CERTS_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_OP_EQUALITY: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_OP_EXISTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_OP_NOT_EXISTS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_VALID_SIGNATURE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_FILTER_VALID_TIME_RANGE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SELECTUI_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SELECT_STRUCT_A(i32);
pub struct CERT_SELECT_STRUCT_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SELECT_STRUCT_W(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_TRUST_DO_FULL_SEARCH: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_TRUST_DO_FULL_TRUST: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_TRUST_MASK: u32 = 16777215u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_TRUST_PERMIT_MISSING_CRLS: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_AFTER_END: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_BEFORE_START: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_CERTIFICATE_REVOKED: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_CRL_OUT_OF_DATE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_EXPLICITLY_DISTRUSTED: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_EXTENDED_USAGE_FAILURE: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_ISSUER_DISTRUST: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_ISSUER_INVALID: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_KEY_USAGE_EXT_FAILURE: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_MASK_TRUST: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_MASK_VALIDITY: u32 = 65535u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_NAME_CONSTRAINTS_FAILURE: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_NO_CRL_FOUND: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_NO_ISSUER_CERT_FOUND: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_NO_TRUST_DATA: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_OTHER_ERROR: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_OTHER_EXTENSION_FAILURE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_PERIOD_NESTING_FAILURE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_SIGNATURE_FAILS: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CERT_VALIDITY_UNKNOWN_CRITICAL_EXTENSION: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_VERIFY_CERTIFICATE_TRUST(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CERT_VIEWPROPERTIES_STRUCT_A(i32);
pub struct CERT_VIEWPROPERTIES_STRUCT_FLAGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CERT_VIEWPROPERTIES_STRUCT_W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMFLTR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMOID(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CM_VIEWFLAGS_MASK: u32 = 16777215u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTDLG_ACTION_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTDLG_CACHE_ONLY_URL_RETRIEVAL: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTDLG_DISABLE_AIA: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTDLG_POLICY_MASK: u32 = 65535u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTDLG_REVOCATION_CACHE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTDLG_REVOCATION_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTDLG_REVOCATION_NONE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTDLG_REVOCATION_ONLINE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_CERT_MGR_PUBLISHER_TAB: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_CERT_MGR_SINGLE_TAB_FLAG: u32 = 32768u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_CERT_MGR_STRUCT(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_CERT_MGR_TAB_MASK: u32 = 15u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_INITDIALOG_STRUCT(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_SELECT_EXPIRATION_COLUMN: u64 = 32u64;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_SELECT_FRIENDLYNAME_COLUMN: u64 = 8u64;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_SELECT_INTENDEDUSE_COLUMN: u64 = 4u64;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_SELECT_ISSUEDBY_COLUMN: u64 = 2u64;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_SELECT_ISSUEDTO_COLUMN: u64 = 1u64;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_SELECT_LOCATION_COLUMN: u64 = 16u64;
pub struct CRYPTUI_VIEWCERTIFICATE_FLAGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CRYPTUI_VIEWCERTIFICATE_STRUCTA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CRYPTUI_VIEWCERTIFICATE_STRUCTW(i32);
pub struct CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE(i32);
pub struct CRYPTUI_WIZ_DIGITAL_SIGN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO(i32);
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_EXCLUDE_PAGE_HASHES: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_INCLUDE_PAGE_HASHES: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO(i32);
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION(i32);
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO(i32);
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO(i32);
pub struct CRYPTUI_WIZ_EXPORT_FORMAT(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYPTUI_WIZ_EXPORT_FORMAT_SERIALIZED_CERT_STORE: u32 = 5u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_EXPORT_INFO(i32);
pub struct CRYPTUI_WIZ_EXPORT_SUBJECT(i32);
pub struct CRYPTUI_WIZ_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_IMPORT_SRC_INFO(i32);
pub struct CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CRYTPDLG_FLAGS_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const CSS_SELECTCERT_MASK: u32 = 16777215u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_MODIFY_REQUEST(i32);
pub struct CTL_MODIFY_REQUEST_OPERATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PFNCFILTERPROC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PFNCMFILTERPROC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PFNCMHOOKPROC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PFNTRUSTHELPER(i32);
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const POLICY_IGNORE_NON_CRITICAL_BC: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const SELCERT_ALGORITHM: u32 = 105u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const SELCERT_CERTLIST: u32 = 102u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const SELCERT_FINEPRINT: u32 = 101u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const SELCERT_ISSUED_TO: u32 = 103u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const SELCERT_PROPERTIES: u32 = 100u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const SELCERT_SERIAL_NUM: u32 = 106u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const SELCERT_THUMBPRINT: u32 = 107u32;
#[doc = "*Required features: `Win32_Security_Cryptography_UI`*"]
pub const SELCERT_VALIDITY: u32 = 104u32;
