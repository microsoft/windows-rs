windows_link::link!("cryptui.dll" "system" fn CertSelectionGetSerializedBlob(pcsi : *const CERT_SELECTUI_INPUT, ppoutbuffer : *mut *mut core::ffi::c_void, puloutbuffersize : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgCertMgr(pcryptuicertmgr : *const CRYPTUI_CERT_MGR_STRUCT) -> windows_sys::core::BOOL);
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgSelectCertificateFromStore(hcertstore : super::HCERTSTORE, hwnd : super::super::super::Foundation::HWND, pwsztitle : windows_sys::core::PCWSTR, pwszdisplaystring : windows_sys::core::PCWSTR, dwdontusecolumn : u32, dwflags : u32, pvreserved : *const core::ffi::c_void) -> *mut super::CERT_CONTEXT);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgViewCertificateA(pcertviewinfo : *const CRYPTUI_VIEWCERTIFICATE_STRUCTA, pfpropertieschanged : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgViewCertificateW(pcertviewinfo : *const CRYPTUI_VIEWCERTIFICATE_STRUCTW, pfpropertieschanged : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgViewContext(dwcontexttype : u32, pvcontext : *const core::ffi::c_void, hwnd : super::super::super::Foundation::HWND, pwsztitle : windows_sys::core::PCWSTR, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("cryptui.dll" "system" fn CryptUIWizDigitalSign(dwflags : u32, hwndparent : super::super::super::Foundation::HWND, pwszwizardtitle : windows_sys::core::PCWSTR, pdigitalsigninfo : *const CRYPTUI_WIZ_DIGITAL_SIGN_INFO, ppsigncontext : *mut *mut CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("cryptui.dll" "system" fn CryptUIWizExport(dwflags : CRYPTUI_WIZ_FLAGS, hwndparent : super::super::super::Foundation::HWND, pwszwizardtitle : windows_sys::core::PCWSTR, pexportinfo : *const CRYPTUI_WIZ_EXPORT_INFO, pvoid : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("cryptui.dll" "system" fn CryptUIWizFreeDigitalSignContext(psigncontext : *const CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("cryptui.dll" "system" fn CryptUIWizImport(dwflags : CRYPTUI_WIZ_FLAGS, hwndparent : super::super::super::Foundation::HWND, pwszwizardtitle : windows_sys::core::PCWSTR, pimportsrc : *const CRYPTUI_WIZ_IMPORT_SRC_INFO, hdestcertstore : super::HCERTSTORE) -> windows_sys::core::BOOL);
pub const ACTION_REVOCATION_DEFAULT_CACHE: u32 = 131072;
pub const ACTION_REVOCATION_DEFAULT_ONLINE: u32 = 65536;
pub const CERTVIEW_CRYPTUI_LPARAM: u32 = 8388608;
pub const CERT_CERTIFICATE_ACTION_VERIFY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7801ebd0_cf4b_11d0_851f_0060979387ea);
pub const CERT_CREDENTIAL_PROVIDER_ID: i32 = -509;
pub const CERT_DISPWELL_DISTRUST_ADD_CA_CERT: u32 = 8;
pub const CERT_DISPWELL_DISTRUST_ADD_LEAF_CERT: u32 = 9;
pub const CERT_DISPWELL_DISTRUST_CA_CERT: u32 = 6;
pub const CERT_DISPWELL_DISTRUST_LEAF_CERT: u32 = 7;
pub const CERT_DISPWELL_SELECT: u32 = 1;
pub const CERT_DISPWELL_TRUST_ADD_CA_CERT: u32 = 4;
pub const CERT_DISPWELL_TRUST_ADD_LEAF_CERT: u32 = 5;
pub const CERT_DISPWELL_TRUST_CA_CERT: u32 = 2;
pub const CERT_DISPWELL_TRUST_LEAF_CERT: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_FILTER_DATA {
    pub dwSize: u32,
    pub cExtensionChecks: u32,
    pub arrayExtensionChecks: *mut CERT_FILTER_EXTENSION_MATCH,
    pub dwCheckingFlags: u32,
}
impl Default for CERT_FILTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_FILTER_EXTENSION_MATCH {
    pub szExtensionOID: windows_sys::core::PCSTR,
    pub dwTestOperation: u32,
    pub pbTestData: *mut u8,
    pub cbTestData: u32,
}
impl Default for CERT_FILTER_EXTENSION_MATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_FILTER_INCLUDE_V1_CERTS: u32 = 1;
pub const CERT_FILTER_ISSUER_CERTS_ONLY: u32 = 16;
pub const CERT_FILTER_KEY_EXISTS: u32 = 32;
pub const CERT_FILTER_LEAF_CERTS_ONLY: u32 = 8;
pub const CERT_FILTER_OP_EQUALITY: u32 = 3;
pub const CERT_FILTER_OP_EXISTS: u32 = 1;
pub const CERT_FILTER_OP_NOT_EXISTS: u32 = 2;
pub const CERT_FILTER_VALID_SIGNATURE: u32 = 4;
pub const CERT_FILTER_VALID_TIME_RANGE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_SELECTUI_INPUT {
    pub hStore: super::HCERTSTORE,
    pub prgpChain: *mut *mut super::CERT_CHAIN_CONTEXT,
    pub cChain: u32,
}
impl Default for CERT_SELECTUI_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CERT_SELECT_STRUCT_A {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pTemplateName: windows_sys::core::PCSTR,
    pub dwFlags: CERT_SELECT_STRUCT_FLAGS,
    pub szTitle: windows_sys::core::PCSTR,
    pub cCertStore: u32,
    pub arrayCertStore: *mut super::HCERTSTORE,
    pub szPurposeOid: windows_sys::core::PCSTR,
    pub cCertContext: u32,
    pub arrayCertContext: *mut *mut super::CERT_CONTEXT,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub pfnHook: PFNCMHOOKPROC,
    pub pfnFilter: PFNCMFILTERPROC,
    pub szHelpFileName: windows_sys::core::PCSTR,
    pub dwHelpId: u32,
    pub hprov: usize,
}
impl Default for CERT_SELECT_STRUCT_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CERT_SELECT_STRUCT_FLAGS = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CERT_SELECT_STRUCT_W {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pTemplateName: windows_sys::core::PCWSTR,
    pub dwFlags: CERT_SELECT_STRUCT_FLAGS,
    pub szTitle: windows_sys::core::PCWSTR,
    pub cCertStore: u32,
    pub arrayCertStore: *mut super::HCERTSTORE,
    pub szPurposeOid: windows_sys::core::PCSTR,
    pub cCertContext: u32,
    pub arrayCertContext: *mut *mut super::CERT_CONTEXT,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub pfnHook: PFNCMHOOKPROC,
    pub pfnFilter: PFNCMFILTERPROC,
    pub szHelpFileName: windows_sys::core::PCWSTR,
    pub dwHelpId: u32,
    pub hprov: usize,
}
impl Default for CERT_SELECT_STRUCT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CERT_TRUST_DO_FULL_SEARCH: u32 = 1;
pub const CERT_TRUST_DO_FULL_TRUST: u32 = 5;
pub const CERT_TRUST_MASK: u32 = 16777215;
pub const CERT_TRUST_PERMIT_MISSING_CRLS: u32 = 2;
pub const CERT_VALIDITY_AFTER_END: u32 = 2;
pub const CERT_VALIDITY_BEFORE_START: u32 = 1;
pub const CERT_VALIDITY_CERTIFICATE_REVOKED: u32 = 8;
pub const CERT_VALIDITY_CRL_OUT_OF_DATE: u32 = 1073741824;
pub const CERT_VALIDITY_EXPLICITLY_DISTRUSTED: u32 = 16777216;
pub const CERT_VALIDITY_EXTENDED_USAGE_FAILURE: u32 = 32;
pub const CERT_VALIDITY_ISSUER_DISTRUST: u32 = 33554432;
pub const CERT_VALIDITY_ISSUER_INVALID: u32 = 256;
pub const CERT_VALIDITY_KEY_USAGE_EXT_FAILURE: u32 = 16;
pub const CERT_VALIDITY_MASK_TRUST: u32 = 4294901760;
pub const CERT_VALIDITY_MASK_VALIDITY: u32 = 65535;
pub const CERT_VALIDITY_NAME_CONSTRAINTS_FAILURE: u32 = 64;
pub const CERT_VALIDITY_NO_CRL_FOUND: u32 = 536870912;
pub const CERT_VALIDITY_NO_ISSUER_CERT_FOUND: u32 = 268435456;
pub const CERT_VALIDITY_NO_TRUST_DATA: u32 = 2147483648;
pub const CERT_VALIDITY_OTHER_ERROR: u32 = 2048;
pub const CERT_VALIDITY_OTHER_EXTENSION_FAILURE: u32 = 512;
pub const CERT_VALIDITY_PERIOD_NESTING_FAILURE: u32 = 1024;
pub const CERT_VALIDITY_SIGNATURE_FAILS: u32 = 4;
pub const CERT_VALIDITY_UNKNOWN_CRITICAL_EXTENSION: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CERT_VERIFY_CERTIFICATE_TRUST {
    pub cbSize: u32,
    pub pccert: *const super::CERT_CONTEXT,
    pub dwFlags: u32,
    pub dwIgnoreErr: u32,
    pub pdwErrors: *mut u32,
    pub pszUsageOid: windows_sys::core::PSTR,
    pub hprov: usize,
    pub cRootStores: u32,
    pub rghstoreRoots: *mut super::HCERTSTORE,
    pub cStores: u32,
    pub rghstoreCAs: *mut super::HCERTSTORE,
    pub cTrustStores: u32,
    pub rghstoreTrust: *mut super::HCERTSTORE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub pfnTrustHelper: PFNTRUSTHELPER,
    pub pcChain: *mut u32,
    pub prgChain: *mut *mut *mut super::CERT_CONTEXT,
    pub prgdwErrors: *mut *mut u32,
    pub prgpbTrustInfo: *mut *mut super::CRYPT_INTEGER_BLOB,
}
impl Default for CERT_VERIFY_CERTIFICATE_TRUST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_VIEWPROPERTIES_STRUCT_A {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub dwFlags: CERT_VIEWPROPERTIES_STRUCT_FLAGS,
    pub szTitle: windows_sys::core::PCSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub arrayPurposes: *mut windows_sys::core::PSTR,
    pub cArrayPurposes: u32,
    pub cRootStores: u32,
    pub rghstoreRoots: *mut super::HCERTSTORE,
    pub cStores: u32,
    pub rghstoreCAs: *mut super::HCERTSTORE,
    pub cTrustStores: u32,
    pub rghstoreTrust: *mut super::HCERTSTORE,
    pub hprov: usize,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub dwPad: u32,
    pub szHelpFileName: windows_sys::core::PCSTR,
    pub dwHelpId: u32,
    pub nStartPage: u32,
    pub cArrayPropSheetPages: u32,
    pub arrayPropSheetPages: *mut super::super::super::UI::Controls::PROPSHEETPAGEA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for CERT_VIEWPROPERTIES_STRUCT_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CERT_VIEWPROPERTIES_STRUCT_FLAGS = u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_VIEWPROPERTIES_STRUCT_W {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub dwFlags: CERT_VIEWPROPERTIES_STRUCT_FLAGS,
    pub szTitle: windows_sys::core::PCWSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub arrayPurposes: *mut windows_sys::core::PSTR,
    pub cArrayPurposes: u32,
    pub cRootStores: u32,
    pub rghstoreRoots: *mut super::HCERTSTORE,
    pub cStores: u32,
    pub rghstoreCAs: *mut super::HCERTSTORE,
    pub cTrustStores: u32,
    pub rghstoreTrust: *mut super::HCERTSTORE,
    pub hprov: usize,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub dwPad: u32,
    pub szHelpFileName: windows_sys::core::PCWSTR,
    pub dwHelpId: u32,
    pub nStartPage: u32,
    pub cArrayPropSheetPages: u32,
    pub arrayPropSheetPages: *mut super::super::super::UI::Controls::PROPSHEETPAGEA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for CERT_VIEWPROPERTIES_STRUCT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CM_ADD_CERT_STORES: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 512;
pub const CM_ENABLEHOOK: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 1;
pub const CM_ENABLETEMPLATE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 8;
pub const CM_HIDE_ADVANCEPAGE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 16;
pub const CM_HIDE_DETAILPAGE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 256;
pub const CM_HIDE_TRUSTPAGE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 32;
pub const CM_NO_EDITTRUST: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 128;
pub const CM_NO_NAMECHANGE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 64;
pub const CM_SHOW_HELP: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 2;
pub const CM_SHOW_HELPICON: CERT_VIEWPROPERTIES_STRUCT_FLAGS = 4;
pub const CM_VIEWFLAGS_MASK: u32 = 16777215;
pub const CRYPTDLG_ACTION_MASK: u32 = 4294901760;
pub const CRYPTDLG_CACHE_ONLY_URL_RETRIEVAL: u32 = 268435456;
pub const CRYPTDLG_DISABLE_AIA: u32 = 134217728;
pub const CRYPTDLG_POLICY_MASK: u32 = 65535;
pub const CRYPTDLG_REVOCATION_CACHE: u32 = 1073741824;
pub const CRYPTDLG_REVOCATION_DEFAULT: u32 = 0;
pub const CRYPTDLG_REVOCATION_NONE: u32 = 536870912;
pub const CRYPTDLG_REVOCATION_ONLINE: u32 = 2147483648;
pub const CRYPTUI_ACCEPT_DECLINE_STYLE: CRYPTUI_VIEWCERTIFICATE_FLAGS = 64;
pub const CRYPTUI_CACHE_ONLY_URL_RETRIEVAL: CRYPTUI_VIEWCERTIFICATE_FLAGS = 262144;
pub const CRYPTUI_CERT_MGR_PUBLISHER_TAB: u32 = 4;
pub const CRYPTUI_CERT_MGR_SINGLE_TAB_FLAG: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTUI_CERT_MGR_STRUCT {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub pwszTitle: windows_sys::core::PCWSTR,
    pub pszInitUsageOID: windows_sys::core::PCSTR,
}
impl Default for CRYPTUI_CERT_MGR_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_CERT_MGR_TAB_MASK: u32 = 15;
pub const CRYPTUI_DISABLE_ADDTOSTORE: CRYPTUI_VIEWCERTIFICATE_FLAGS = 16;
pub const CRYPTUI_DISABLE_EDITPROPERTIES: CRYPTUI_VIEWCERTIFICATE_FLAGS = 4;
pub const CRYPTUI_DISABLE_EXPORT: CRYPTUI_VIEWCERTIFICATE_FLAGS = 8192;
pub const CRYPTUI_DISABLE_HTMLLINK: CRYPTUI_VIEWCERTIFICATE_FLAGS = 65536;
pub const CRYPTUI_DISABLE_ISSUERSTATEMENT: CRYPTUI_VIEWCERTIFICATE_FLAGS = 131072;
pub const CRYPTUI_DONT_OPEN_STORES: CRYPTUI_VIEWCERTIFICATE_FLAGS = 256;
pub const CRYPTUI_ENABLE_ADDTOSTORE: CRYPTUI_VIEWCERTIFICATE_FLAGS = 32;
pub const CRYPTUI_ENABLE_EDITPROPERTIES: CRYPTUI_VIEWCERTIFICATE_FLAGS = 8;
pub const CRYPTUI_ENABLE_REVOCATION_CHECKING: CRYPTUI_VIEWCERTIFICATE_FLAGS = 2048;
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_CHAIN: CRYPTUI_VIEWCERTIFICATE_FLAGS = 32768;
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: CRYPTUI_VIEWCERTIFICATE_FLAGS = 2048;
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_END_CERT: CRYPTUI_VIEWCERTIFICATE_FLAGS = 16384;
pub const CRYPTUI_HIDE_DETAILPAGE: CRYPTUI_VIEWCERTIFICATE_FLAGS = 2;
pub const CRYPTUI_HIDE_HIERARCHYPAGE: CRYPTUI_VIEWCERTIFICATE_FLAGS = 1;
pub const CRYPTUI_IGNORE_UNTRUSTED_ROOT: CRYPTUI_VIEWCERTIFICATE_FLAGS = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTUI_INITDIALOG_STRUCT {
    pub lParam: super::super::super::Foundation::LPARAM,
    pub pCertContext: *const super::CERT_CONTEXT,
}
impl Default for CRYPTUI_INITDIALOG_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_ONLY_OPEN_ROOT_STORE: CRYPTUI_VIEWCERTIFICATE_FLAGS = 512;
pub const CRYPTUI_SELECT_EXPIRATION_COLUMN: u64 = 32;
pub const CRYPTUI_SELECT_FRIENDLYNAME_COLUMN: u64 = 8;
pub const CRYPTUI_SELECT_INTENDEDUSE_COLUMN: u64 = 4;
pub const CRYPTUI_SELECT_ISSUEDBY_COLUMN: u64 = 2;
pub const CRYPTUI_SELECT_ISSUEDTO_COLUMN: u64 = 1;
pub const CRYPTUI_SELECT_LOCATION_COLUMN: u64 = 16;
pub type CRYPTUI_VIEWCERTIFICATE_FLAGS = u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub dwFlags: CRYPTUI_VIEWCERTIFICATE_FLAGS,
    pub szTitle: windows_sys::core::PCSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub rgszPurposes: *const windows_sys::core::PCSTR,
    pub cPurposes: u32,
    pub Anonymous: CRYPTUI_VIEWCERTIFICATE_STRUCTA_0,
    pub fpCryptProviderDataTrustedUsage: windows_sys::core::BOOL,
    pub idxSigner: u32,
    pub idxCert: u32,
    pub fCounterSigner: windows_sys::core::BOOL,
    pub idxCounterSigner: u32,
    pub cStores: u32,
    pub rghStores: *mut super::HCERTSTORE,
    pub cPropSheetPages: u32,
    pub rgPropSheetPages: *mut super::super::super::UI::Controls::PROPSHEETPAGEA,
    pub nStartPage: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    pub pCryptProviderData: *const super::super::WinTrust::CRYPT_PROVIDER_DATA,
    pub hWVTStateData: super::super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub dwFlags: CRYPTUI_VIEWCERTIFICATE_FLAGS,
    pub szTitle: windows_sys::core::PCWSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub rgszPurposes: *const windows_sys::core::PCSTR,
    pub cPurposes: u32,
    pub Anonymous: CRYPTUI_VIEWCERTIFICATE_STRUCTW_0,
    pub fpCryptProviderDataTrustedUsage: windows_sys::core::BOOL,
    pub idxSigner: u32,
    pub idxCert: u32,
    pub fCounterSigner: windows_sys::core::BOOL,
    pub idxCounterSigner: u32,
    pub cStores: u32,
    pub rghStores: *mut super::HCERTSTORE,
    pub cPropSheetPages: u32,
    pub rgPropSheetPages: *mut super::super::super::UI::Controls::PROPSHEETPAGEW,
    pub nStartPage: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    pub pCryptProviderData: *const super::super::WinTrust::CRYPT_PROVIDER_DATA,
    pub hWVTStateData: super::super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WARN_REMOTE_TRUST: CRYPTUI_VIEWCERTIFICATE_FLAGS = 4096;
pub const CRYPTUI_WARN_UNTRUSTED_ROOT: CRYPTUI_VIEWCERTIFICATE_FLAGS = 1024;
pub type CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE = u32;
pub type CRYPTUI_WIZ_DIGITAL_SIGN = u32;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN: CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE = 1;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN_NO_ROOT: CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE = 2;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_ADD_NONE: CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    pub dwSize: u32,
    pub pGuidSubject: *mut windows_sys::core::GUID,
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
    pub pwszDisplayName: windows_sys::core::PCWSTR,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_CERT: CRYPTUI_WIZ_DIGITAL_SIGN = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    pub dwSize: u32,
    pub pwszSigningCertFileName: windows_sys::core::PWSTR,
    pub dwPvkChoice: CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION,
    pub Anonymous: CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    pub pPvkFileInfo: *mut CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO,
    pub pPvkProvInfo: *mut super::CRYPT_KEY_PROV_INFO,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_COMMERCIAL: CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    pub dwSize: u32,
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_EXCLUDE_PAGE_HASHES: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    pub dwSize: u32,
    pub dwAttrFlags: CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE,
    pub pwszDescription: windows_sys::core::PCWSTR,
    pub pwszMoreInfoLocation: windows_sys::core::PCWSTR,
    pub pszHashAlg: windows_sys::core::PCSTR,
    pub pwszSigningCertDisplayString: windows_sys::core::PCWSTR,
    pub hAdditionalCertStore: super::HCERTSTORE,
    pub psAuthenticated: *mut super::CRYPT_ATTRIBUTES,
    pub psUnauthenticated: *mut super::CRYPT_ATTRIBUTES,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_INCLUDE_PAGE_HASHES: u32 = 4;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_INDIVIDUAL: CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    pub dwSize: u32,
    pub dwSubjectChoice: CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT,
    pub Anonymous1: CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0,
    pub dwSigningCertChoice: CRYPTUI_WIZ_DIGITAL_SIGN,
    pub Anonymous2: CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1,
    pub pwszTimestampURL: windows_sys::core::PCWSTR,
    pub dwAdditionalCertChoice: CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE,
    pub pSignExtInfo: *mut CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    pub pwszFileName: windows_sys::core::PCWSTR,
    pub pSignBlobInfo: *mut CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    pub pSigningCertContext: *const super::CERT_CONTEXT,
    pub pSigningCertStore: *mut CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO,
    pub pSigningCertPvkInfo: *mut CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_NONE: CRYPTUI_WIZ_DIGITAL_SIGN = 0;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK: CRYPTUI_WIZ_DIGITAL_SIGN = 3;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE: CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    pub dwSize: u32,
    pub pwszPvkFileName: windows_sys::core::PWSTR,
    pub pwszProvName: windows_sys::core::PWSTR,
    pub dwProvType: u32,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION = u32;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK_PROV: CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION = 2;
pub type CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE = u32;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_STORE: CRYPTUI_WIZ_DIGITAL_SIGN = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    pub dwSize: u32,
    pub cCertStore: u32,
    pub rghCertStore: *mut super::HCERTSTORE,
    pub pFilterCallback: PFNCFILTERPROC,
    pub pvCallbackData: *mut core::ffi::c_void,
}
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT = u32;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_BLOB: CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT = 2;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_FILE: CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT = 1;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_NONE: CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    pub dwSize: u32,
    pub dwExportFormat: CRYPTUI_WIZ_EXPORT_FORMAT,
    pub fExportChain: windows_sys::core::BOOL,
    pub fExportPrivateKeys: windows_sys::core::BOOL,
    pub pwszPassword: windows_sys::core::PCWSTR,
    pub fStrongEncryption: windows_sys::core::BOOL,
}
impl Default for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_EXPORT_CERT_CONTEXT: CRYPTUI_WIZ_EXPORT_SUBJECT = 1;
pub const CRYPTUI_WIZ_EXPORT_CERT_STORE: CRYPTUI_WIZ_EXPORT_SUBJECT = 4;
pub const CRYPTUI_WIZ_EXPORT_CERT_STORE_CERTIFICATES_ONLY: CRYPTUI_WIZ_EXPORT_SUBJECT = 5;
pub const CRYPTUI_WIZ_EXPORT_CRL_CONTEXT: CRYPTUI_WIZ_EXPORT_SUBJECT = 3;
pub const CRYPTUI_WIZ_EXPORT_CTL_CONTEXT: CRYPTUI_WIZ_EXPORT_SUBJECT = 2;
pub type CRYPTUI_WIZ_EXPORT_FORMAT = u32;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_BASE64: CRYPTUI_WIZ_EXPORT_FORMAT = 4;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_CRL: CRYPTUI_WIZ_EXPORT_FORMAT = 6;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_CTL: CRYPTUI_WIZ_EXPORT_FORMAT = 7;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_DER: CRYPTUI_WIZ_EXPORT_FORMAT = 1;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_PFX: CRYPTUI_WIZ_EXPORT_FORMAT = 2;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7: CRYPTUI_WIZ_EXPORT_FORMAT = 3;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_SERIALIZED_CERT_STORE: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_EXPORT_INFO {
    pub dwSize: u32,
    pub pwszExportFileName: windows_sys::core::PCWSTR,
    pub dwSubjectChoice: CRYPTUI_WIZ_EXPORT_SUBJECT,
    pub Anonymous: CRYPTUI_WIZ_EXPORT_INFO_0,
    pub cStores: u32,
    pub rghStores: *mut super::HCERTSTORE,
}
impl Default for CRYPTUI_WIZ_EXPORT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_EXPORT_INFO_0 {
    pub pCertContext: *const super::CERT_CONTEXT,
    pub pCTLContext: *mut super::CTL_CONTEXT,
    pub pCRLContext: *mut super::CRL_CONTEXT,
    pub hCertStore: super::HCERTSTORE,
}
impl Default for CRYPTUI_WIZ_EXPORT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_EXPORT_NO_DELETE_PRIVATE_KEY: CRYPTUI_WIZ_FLAGS = 512;
pub const CRYPTUI_WIZ_EXPORT_PRIVATE_KEY: CRYPTUI_WIZ_FLAGS = 256;
pub type CRYPTUI_WIZ_EXPORT_SUBJECT = u32;
pub type CRYPTUI_WIZ_FLAGS = u32;
pub const CRYPTUI_WIZ_IGNORE_NO_UI_FLAG_FOR_CSPS: CRYPTUI_WIZ_FLAGS = 2;
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CERT: CRYPTUI_WIZ_FLAGS = 131072;
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CRL: CRYPTUI_WIZ_FLAGS = 262144;
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CTL: CRYPTUI_WIZ_FLAGS = 524288;
pub const CRYPTUI_WIZ_IMPORT_NO_CHANGE_DEST_STORE: CRYPTUI_WIZ_FLAGS = 65536;
pub const CRYPTUI_WIZ_IMPORT_REMOTE_DEST_STORE: CRYPTUI_WIZ_FLAGS = 4194304;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_IMPORT_SRC_INFO {
    pub dwSize: u32,
    pub dwSubjectChoice: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION,
    pub Anonymous: CRYPTUI_WIZ_IMPORT_SRC_INFO_0,
    pub dwFlags: super::CRYPT_KEY_FLAGS,
    pub pwszPassword: windows_sys::core::PCWSTR,
}
impl Default for CRYPTUI_WIZ_IMPORT_SRC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    pub pwszFileName: windows_sys::core::PCWSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub pCTLContext: *mut super::CTL_CONTEXT,
    pub pCRLContext: *mut super::CRL_CONTEXT,
    pub hCertStore: super::HCERTSTORE,
}
impl Default for CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_CONTEXT: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = 2;
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_STORE: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = 5;
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CRL_CONTEXT: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = 4;
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CTL_CONTEXT: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = 3;
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_FILE: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = 1;
pub type CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = u32;
pub const CRYPTUI_WIZ_IMPORT_TO_CURRENTUSER: CRYPTUI_WIZ_FLAGS = 2097152;
pub const CRYPTUI_WIZ_IMPORT_TO_LOCALMACHINE: CRYPTUI_WIZ_FLAGS = 1048576;
pub const CRYPTUI_WIZ_NO_UI: CRYPTUI_WIZ_FLAGS = 1;
pub const CRYPTUI_WIZ_NO_UI_EXCEPT_CSP: CRYPTUI_WIZ_FLAGS = 3;
pub const CRYTPDLG_FLAGS_MASK: u32 = 4278190080;
pub const CSS_ALLOWMULTISELECT: CERT_SELECT_STRUCT_FLAGS = 4;
pub const CSS_ENABLEHOOK: CERT_SELECT_STRUCT_FLAGS = 2;
pub const CSS_ENABLETEMPLATE: CERT_SELECT_STRUCT_FLAGS = 32;
pub const CSS_ENABLETEMPLATEHANDLE: CERT_SELECT_STRUCT_FLAGS = 64;
pub const CSS_HIDE_PROPERTIES: CERT_SELECT_STRUCT_FLAGS = 1;
pub const CSS_SELECTCERT_MASK: u32 = 16777215;
pub const CSS_SHOW_HELP: CERT_SELECT_STRUCT_FLAGS = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTL_MODIFY_REQUEST {
    pub pccert: *const super::CERT_CONTEXT,
    pub dwOperation: CTL_MODIFY_REQUEST_OPERATION,
    pub dwError: u32,
}
impl Default for CTL_MODIFY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CTL_MODIFY_REQUEST_ADD_NOT_TRUSTED: CTL_MODIFY_REQUEST_OPERATION = 1;
pub const CTL_MODIFY_REQUEST_ADD_TRUSTED: CTL_MODIFY_REQUEST_OPERATION = 3;
pub type CTL_MODIFY_REQUEST_OPERATION = u32;
pub const CTL_MODIFY_REQUEST_REMOVE: CTL_MODIFY_REQUEST_OPERATION = 2;
pub type PFNCFILTERPROC = Option<unsafe extern "system" fn(pcertcontext: *const super::CERT_CONTEXT, pfinitialselectedcert: *mut windows_sys::core::BOOL, pvcallbackdata: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFNCMFILTERPROC = Option<unsafe extern "system" fn(pcertcontext: *const super::CERT_CONTEXT, param1: super::super::super::Foundation::LPARAM, param2: u32, param3: u32) -> windows_sys::core::BOOL>;
pub type PFNCMHOOKPROC = Option<unsafe extern "system" fn(hwnddialog: super::super::super::Foundation::HWND, message: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> u32>;
pub type PFNTRUSTHELPER = Option<unsafe extern "system" fn(pcertcontext: *const super::CERT_CONTEXT, lcustdata: super::super::super::Foundation::LPARAM, fleafcertificate: windows_sys::core::BOOL, pbtrustblob: *mut u8) -> windows_sys::core::HRESULT>;
pub const POLICY_IGNORE_NON_CRITICAL_BC: u32 = 1;
pub const SELCERT_ALGORITHM: u32 = 105;
pub const SELCERT_CERTLIST: u32 = 102;
pub const SELCERT_FINEPRINT: u32 = 101;
pub const SELCERT_ISSUED_TO: u32 = 103;
pub const SELCERT_PROPERTIES: u32 = 100;
pub const SELCERT_SERIAL_NUM: u32 = 106;
pub const SELCERT_THUMBPRINT: u32 = 107;
pub const SELCERT_VALIDITY: u32 = 104;
pub const szCERT_CERTIFICATE_ACTION_VERIFY: windows_sys::core::PCSTR = windows_sys::core::s!("{7801ebd0-cf4b-11d0-851f-0060979387ea}");
