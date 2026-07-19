#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
windows_link::link!("cryptui.dll" "system" fn CertSelectionGetSerializedBlob(pcsi : *const CERT_SELECTUI_INPUT, ppoutbuffer : *mut *mut core::ffi::c_void, puloutbuffersize : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgCertMgr(pcryptuicertmgr : *const CRYPTUI_CERT_MGR_STRUCT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef"))]
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgSelectCertificateFromStore(hcertstore : super::HCERTSTORE, hwnd : super::HWND, pwsztitle : windows_sys::core::PCWSTR, pwszdisplaystring : windows_sys::core::PCWSTR, dwdontusecolumn : u32, dwflags : u32, pvreserved : *const core::ffi::c_void) -> super::PCCERT_CONTEXT);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgViewCertificateA(pcertviewinfo : *const CRYPTUI_VIEWCERTIFICATE_STRUCTA, pfpropertieschanged : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgViewCertificateW(pcertviewinfo : *const CRYPTUI_VIEWCERTIFICATE_STRUCTW, pfpropertieschanged : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("cryptui.dll" "system" fn CryptUIDlgViewContext(dwcontexttype : u32, pvcontext : *const core::ffi::c_void, hwnd : super::HWND, pwsztitle : windows_sys::core::PCWSTR, dwflags : u32, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef"))]
windows_link::link!("cryptui.dll" "system" fn CryptUIWizDigitalSign(dwflags : u32, hwndparent : super::HWND, pwszwizardtitle : windows_sys::core::PCWSTR, pdigitalsigninfo : *const CRYPTUI_WIZ_DIGITAL_SIGN_INFO, ppsigncontext : *mut PCCRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef"))]
windows_link::link!("cryptui.dll" "system" fn CryptUIWizExport(dwflags : u32, hwndparent : super::HWND, pwszwizardtitle : windows_sys::core::PCWSTR, pexportinfo : *const CRYPTUI_WIZ_EXPORT_INFO, pvoid : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("cryptui.dll" "system" fn CryptUIWizFreeDigitalSignContext(psigncontext : *const CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef"))]
windows_link::link!("cryptui.dll" "system" fn CryptUIWizImport(dwflags : u32, hwndparent : super::HWND, pwszwizardtitle : windows_sys::core::PCWSTR, pimportsrc : *const CRYPTUI_WIZ_IMPORT_SRC_INFO, hdestcertstore : super::HCERTSTORE) -> windows_sys::core::BOOL);
pub const CERT_CREDENTIAL_PROVIDER_ID: i32 = -509;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct CERT_SELECTUI_INPUT {
    pub hStore: super::HCERTSTORE,
    pub prgpChain: *mut super::PCCERT_CHAIN_CONTEXT,
    pub cChain: u32,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CERT_SELECTUI_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_ACCEPT_DECLINE_STYLE: u32 = 64;
pub const CRYPTUI_CACHE_ONLY_URL_RETRIEVAL: u32 = 262144;
pub const CRYPTUI_CERT_MGR_PUBLISHER_TAB: u32 = 4;
pub const CRYPTUI_CERT_MGR_SINGLE_TAB_FLAG: u32 = 32768;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct CRYPTUI_CERT_MGR_STRUCT {
    pub dwSize: u32,
    pub hwndParent: super::HWND,
    pub dwFlags: u32,
    pub pwszTitle: windows_sys::core::PCWSTR,
    pub pszInitUsageOID: windows_sys::core::PCSTR,
}
#[cfg(feature = "windef")]
impl Default for CRYPTUI_CERT_MGR_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_CERT_MGR_TAB_MASK: u32 = 15;
pub const CRYPTUI_DISABLE_ADDTOSTORE: u32 = 16;
pub const CRYPTUI_DISABLE_EDITPROPERTIES: u32 = 4;
pub const CRYPTUI_DISABLE_EXPORT: u32 = 8192;
pub const CRYPTUI_DISABLE_HTMLLINK: u32 = 65536;
pub const CRYPTUI_DISABLE_ISSUERSTATEMENT: u32 = 131072;
pub const CRYPTUI_DONT_OPEN_STORES: u32 = 256;
pub const CRYPTUI_ENABLE_ADDTOSTORE: u32 = 32;
pub const CRYPTUI_ENABLE_EDITPROPERTIES: u32 = 8;
pub const CRYPTUI_ENABLE_REVOCATION_CHECKING: u32 = 2048;
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_CHAIN: u32 = 32768;
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 2048;
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_END_CERT: u32 = 16384;
pub const CRYPTUI_HIDE_DETAILPAGE: u32 = 2;
pub const CRYPTUI_HIDE_HIERARCHYPAGE: u32 = 1;
pub const CRYPTUI_IGNORE_UNTRUSTED_ROOT: u32 = 128;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_INITDIALOG_STRUCT {
    pub lParam: super::LPARAM,
    pub pCertContext: super::PCCERT_CONTEXT,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_INITDIALOG_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_ONLY_OPEN_ROOT_STORE: u32 = 512;
pub const CRYPTUI_SELECT_EXPIRATION_COLUMN: u32 = 32;
pub const CRYPTUI_SELECT_FRIENDLYNAME_COLUMN: u32 = 8;
pub const CRYPTUI_SELECT_INTENDEDUSE_COLUMN: u32 = 4;
pub const CRYPTUI_SELECT_ISSUEDBY_COLUMN: u32 = 2;
pub const CRYPTUI_SELECT_ISSUEDTO_COLUMN: u32 = 1;
pub const CRYPTUI_SELECT_LOCATION_COLUMN: u32 = 16;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    pub dwSize: u32,
    pub hwndParent: super::HWND,
    pub dwFlags: u32,
    pub szTitle: windows_sys::core::PCSTR,
    pub pCertContext: super::PCCERT_CONTEXT,
    pub rgszPurposes: *mut windows_sys::core::PCSTR,
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
    pub rgPropSheetPages: super::LPCPROPSHEETPAGEA,
    pub nStartPage: u32,
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
impl Default for CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    pub pCryptProviderData: *const super::CRYPT_PROVIDER_DATA,
    pub hWVTStateData: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
impl Default for CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    pub dwSize: u32,
    pub hwndParent: super::HWND,
    pub dwFlags: u32,
    pub szTitle: windows_sys::core::PCWSTR,
    pub pCertContext: super::PCCERT_CONTEXT,
    pub rgszPurposes: *mut windows_sys::core::PCSTR,
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
    pub rgPropSheetPages: super::LPCPROPSHEETPAGEW,
    pub nStartPage: u32,
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
impl Default for CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    pub pCryptProviderData: *const super::CRYPT_PROVIDER_DATA,
    pub hWVTStateData: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
impl Default for CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WARN_REMOTE_TRUST: u32 = 4096;
pub const CRYPTUI_WARN_UNTRUSTED_ROOT: u32 = 1024;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN: u32 = 1;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN_NO_ROOT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const CRYPTUI_WIZ_DIGITAL_SIGN_CERT: u32 = 1;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    pub dwSize: u32,
    pub pwszSigningCertFileName: windows_sys::core::PWSTR,
    pub dwPvkChoice: u32,
    pub Anonymous: CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0,
}
#[cfg(feature = "wincrypt")]
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    pub pPvkFileInfo: PCCRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO,
    pub pPvkProvInfo: super::PCRYPT_KEY_PROV_INFO,
}
#[cfg(feature = "wincrypt")]
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_COMMERCIAL: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    pub dwSize: u32,
    pub dwAttrFlags: u32,
    pub pwszDescription: windows_sys::core::PCWSTR,
    pub pwszMoreInfoLocation: windows_sys::core::PCWSTR,
    pub pszHashAlg: windows_sys::core::PCSTR,
    pub pwszSigningCertDisplayString: windows_sys::core::PCWSTR,
    pub hAdditionalCertStore: super::HCERTSTORE,
    pub psAuthenticated: super::PCRYPT_ATTRIBUTES,
    pub psUnauthenticated: super::PCRYPT_ATTRIBUTES,
}
#[cfg(feature = "wincrypt")]
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_INCLUDE_PAGE_HASHES: u32 = 4;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_INDIVIDUAL: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    pub dwSize: u32,
    pub dwSubjectChoice: u32,
    pub Anonymous: CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0,
    pub dwSigningCertChoice: u32,
    pub Anonymous2: CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1,
    pub pwszTimestampURL: windows_sys::core::PCWSTR,
    pub dwAdditionalCertChoice: u32,
    pub pSignExtInfo: PCCRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    pub pwszFileName: windows_sys::core::PCWSTR,
    pub pSignBlobInfo: PCCRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    pub pSigningCertContext: super::PCCERT_CONTEXT,
    pub pSigningCertStore: PCCRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO,
    pub pSigningCertPvkInfo: PCCRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK: u32 = 3;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK_PROV: u32 = 2;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_STORE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    pub dwSize: u32,
    pub cCertStore: u32,
    pub rghCertStore: *mut super::HCERTSTORE,
    pub pFilterCallback: PFNCFILTERPROC,
    pub pvCallbackData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_BLOB: u32 = 2;
pub const CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_FILE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    pub dwSize: u32,
    pub dwExportFormat: u32,
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
pub const CRYPTUI_WIZ_EXPORT_CERT_CONTEXT: u32 = 1;
pub const CRYPTUI_WIZ_EXPORT_CERT_STORE: u32 = 4;
pub const CRYPTUI_WIZ_EXPORT_CERT_STORE_CERTIFICATES_ONLY: u32 = 5;
pub const CRYPTUI_WIZ_EXPORT_CRL_CONTEXT: u32 = 3;
pub const CRYPTUI_WIZ_EXPORT_CTL_CONTEXT: u32 = 2;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_BASE64: u32 = 4;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_CRL: u32 = 6;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_CTL: u32 = 7;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_DER: u32 = 1;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_PFX: u32 = 2;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7: u32 = 3;
pub const CRYPTUI_WIZ_EXPORT_FORMAT_SERIALIZED_CERT_STORE: u32 = 5;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_EXPORT_INFO {
    pub dwSize: u32,
    pub pwszExportFileName: windows_sys::core::PCWSTR,
    pub dwSubjectChoice: u32,
    pub Anonymous: CRYPTUI_WIZ_EXPORT_INFO_0,
    pub cStores: u32,
    pub rghStores: *mut super::HCERTSTORE,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_WIZ_EXPORT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_EXPORT_INFO_0 {
    pub pCertContext: super::PCCERT_CONTEXT,
    pub pCTLContext: super::PCCTL_CONTEXT,
    pub pCRLContext: super::PCCRL_CONTEXT,
    pub hCertStore: super::HCERTSTORE,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_WIZ_EXPORT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CERT: u32 = 131072;
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CRL: u32 = 262144;
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CTL: u32 = 524288;
pub const CRYPTUI_WIZ_IMPORT_NO_CHANGE_DEST_STORE: u32 = 65536;
pub const CRYPTUI_WIZ_IMPORT_REMOTE_DEST_STORE: u32 = 4194304;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct CRYPTUI_WIZ_IMPORT_SRC_INFO {
    pub dwSize: u32,
    pub dwSubjectChoice: u32,
    pub Anonymous: CRYPTUI_WIZ_IMPORT_SRC_INFO_0,
    pub dwFlags: u32,
    pub pwszPassword: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_WIZ_IMPORT_SRC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub union CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    pub pwszFileName: windows_sys::core::PCWSTR,
    pub pCertContext: super::PCCERT_CONTEXT,
    pub pCTLContext: super::PCCTL_CONTEXT,
    pub pCRLContext: super::PCCRL_CONTEXT,
    pub hCertStore: super::HCERTSTORE,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_CONTEXT: u32 = 2;
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_STORE: u32 = 5;
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CRL_CONTEXT: u32 = 4;
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CTL_CONTEXT: u32 = 3;
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_FILE: u32 = 1;
pub const CRYPTUI_WIZ_IMPORT_TO_CURRENTUSER: u32 = 2097152;
pub const CRYPTUI_WIZ_IMPORT_TO_LOCALMACHINE: u32 = 1048576;
pub const CRYPTUI_WIZ_NO_UI: u32 = 1;
#[cfg(feature = "windef")]
pub type PCCRYPTUI_CERT_MGR_STRUCT = *const CRYPTUI_CERT_MGR_STRUCT;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
pub type PCCRYPTUI_VIEWCERTIFICATE_STRUCTA = *const CRYPTUI_VIEWCERTIFICATE_STRUCTA;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
pub type PCCRYPTUI_VIEWCERTIFICATE_STRUCTW = *const CRYPTUI_VIEWCERTIFICATE_STRUCTW;
pub type PCCRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO = *const CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO;
#[cfg(feature = "wincrypt")]
pub type PCCRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO = *const CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO;
pub type PCCRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT = *const CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT;
#[cfg(feature = "wincrypt")]
pub type PCCRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO = *const CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCCRYPTUI_WIZ_DIGITAL_SIGN_INFO = *const CRYPTUI_WIZ_DIGITAL_SIGN_INFO;
pub type PCCRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO = *const CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCCRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO = *const CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO;
pub type PCCRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO = *const CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCCRYPTUI_WIZ_EXPORT_INFO = *const CRYPTUI_WIZ_EXPORT_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCCRYPTUI_WIZ_IMPORT_SRC_INFO = *const CRYPTUI_WIZ_IMPORT_SRC_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCERT_SELECTUI_INPUT = *mut CERT_SELECTUI_INPUT;
#[cfg(feature = "windef")]
pub type PCRYPTUI_CERT_MGR_STRUCT = *mut CRYPTUI_CERT_MGR_STRUCT;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCRYPTUI_INITDIALOG_STRUCT = *mut CRYPTUI_INITDIALOG_STRUCT;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
pub type PCRYPTUI_VIEWCERTIFICATE_STRUCTA = *mut CRYPTUI_VIEWCERTIFICATE_STRUCTA;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "prsht", feature = "wincrypt", feature = "windef", feature = "winnt", feature = "wintrust", feature = "winuser"))]
pub type PCRYPTUI_VIEWCERTIFICATE_STRUCTW = *mut CRYPTUI_VIEWCERTIFICATE_STRUCTW;
pub type PCRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO = *mut CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO;
#[cfg(feature = "wincrypt")]
pub type PCRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO = *mut CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO;
pub type PCRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT = *mut CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT;
#[cfg(feature = "wincrypt")]
pub type PCRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO = *mut CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCRYPTUI_WIZ_DIGITAL_SIGN_INFO = *mut CRYPTUI_WIZ_DIGITAL_SIGN_INFO;
pub type PCRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO = *mut CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO = *mut CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO;
pub type PCRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO = *mut CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCRYPTUI_WIZ_EXPORT_INFO = *mut CRYPTUI_WIZ_EXPORT_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCRYPTUI_WIZ_IMPORT_SRC_INFO = *mut CRYPTUI_WIZ_IMPORT_SRC_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PFNCFILTERPROC = Option<unsafe extern "system" fn(pcertcontext: *const super::CERT_CONTEXT, pfinitialselectedcert: *mut windows_sys::core::BOOL, pvcallbackdata: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
