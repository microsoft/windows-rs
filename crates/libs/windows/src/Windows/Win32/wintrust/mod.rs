#[cfg(feature = "windef")]
#[inline]
pub unsafe fn OpenPersonalTrustDBDialog(hwndparent: Option<super::HWND>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn OpenPersonalTrustDBDialog(hwndparent : super::HWND) -> windows_core::BOOL);
    unsafe { OpenPersonalTrustDBDialog(hwndparent.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn OpenPersonalTrustDBDialogEx(hwndparent: Option<super::HWND>, dwflags: u32, pvreserved: Option<*mut *mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn OpenPersonalTrustDBDialogEx(hwndparent : super::HWND, dwflags : u32, pvreserved : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { OpenPersonalTrustDBDialogEx(hwndparent.unwrap_or(core::mem::zeroed()) as _, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WTHelperCertCheckValidSignature(pprovdata: *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT {
    windows_core::link!("wintrust.dll" "system" fn WTHelperCertCheckValidSignature(pprovdata : *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT);
    unsafe { WTHelperCertCheckValidSignature(pprovdata as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[inline]
pub unsafe fn WTHelperCertIsSelfSigned(dwencoding: u32, pcert: *mut super::CERT_INFO) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn WTHelperCertIsSelfSigned(dwencoding : u32, pcert : *mut super::CERT_INFO) -> windows_core::BOOL);
    unsafe { WTHelperCertIsSelfSigned(dwencoding, pcert as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[inline]
pub unsafe fn WTHelperGetProvCertFromChain(psgnr: *mut CRYPT_PROVIDER_SGNR, idxcert: u32) -> *mut CRYPT_PROVIDER_CERT {
    windows_core::link!("wintrust.dll" "system" fn WTHelperGetProvCertFromChain(psgnr : *mut CRYPT_PROVIDER_SGNR, idxcert : u32) -> *mut CRYPT_PROVIDER_CERT);
    unsafe { WTHelperGetProvCertFromChain(psgnr as _, idxcert) }
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WTHelperGetProvPrivateDataFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, pgproviderid: *mut windows_core::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA {
    windows_core::link!("wintrust.dll" "system" fn WTHelperGetProvPrivateDataFromChain(pprovdata : *mut CRYPT_PROVIDER_DATA, pgproviderid : *mut windows_core::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA);
    unsafe { WTHelperGetProvPrivateDataFromChain(pprovdata as _, pgproviderid as _) }
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WTHelperGetProvSignerFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: bool, idxcountersigner: u32) -> *mut CRYPT_PROVIDER_SGNR {
    windows_core::link!("wintrust.dll" "system" fn WTHelperGetProvSignerFromChain(pprovdata : *mut CRYPT_PROVIDER_DATA, idxsigner : u32, fcountersigner : windows_core::BOOL, idxcountersigner : u32) -> *mut CRYPT_PROVIDER_SGNR);
    unsafe { WTHelperGetProvSignerFromChain(pprovdata as _, idxsigner, fcountersigner.into(), idxcountersigner) }
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WTHelperProvDataFromStateData(hstatedata: super::HANDLE) -> *mut CRYPT_PROVIDER_DATA {
    windows_core::link!("wintrust.dll" "system" fn WTHelperProvDataFromStateData(hstatedata : super::HANDLE) -> *mut CRYPT_PROVIDER_DATA);
    unsafe { WTHelperProvDataFromStateData(hstatedata) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WinVerifyTrust(hwnd: super::HWND, pgactionid: *mut windows_core::GUID, pwvtdata: *mut core::ffi::c_void) -> i32 {
    windows_core::link!("wintrust.dll" "system" fn WinVerifyTrust(hwnd : super::HWND, pgactionid : *mut windows_core::GUID, pwvtdata : *mut core::ffi::c_void) -> i32);
    unsafe { WinVerifyTrust(hwnd, pgactionid as _, pwvtdata as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WinVerifyTrustEx(hwnd: super::HWND, pgactionid: *mut windows_core::GUID, pwintrustdata: *mut WINTRUST_DATA) -> i32 {
    windows_core::link!("wintrust.dll" "system" fn WinVerifyTrustEx(hwnd : super::HWND, pgactionid : *mut windows_core::GUID, pwintrustdata : *mut WINTRUST_DATA) -> i32);
    unsafe { WinVerifyTrustEx(hwnd, pgactionid as _, pwintrustdata as _) }
}
#[inline]
pub unsafe fn WintrustAddActionID(pgactionid: *const windows_core::GUID, fdwflags: u32, psprovinfo: *const CRYPT_REGISTER_ACTIONID) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn WintrustAddActionID(pgactionid : *const windows_core::GUID, fdwflags : u32, psprovinfo : *const CRYPT_REGISTER_ACTIONID) -> windows_core::BOOL);
    unsafe { WintrustAddActionID(pgactionid, fdwflags, psprovinfo) }
}
#[inline]
pub unsafe fn WintrustAddDefaultForUsage(pszusageoid: *const i8, psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn WintrustAddDefaultForUsage(pszusageoid : *const i8, psdefusage : *const CRYPT_PROVIDER_REGDEFUSAGE) -> windows_core::BOOL);
    unsafe { WintrustAddDefaultForUsage(pszusageoid, psdefusage) }
}
#[inline]
pub unsafe fn WintrustGetDefaultForUsage(dwaction: u32, pszusageoid: *const i8, psusage: *mut CRYPT_PROVIDER_DEFUSAGE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn WintrustGetDefaultForUsage(dwaction : u32, pszusageoid : *const i8, psusage : *mut CRYPT_PROVIDER_DEFUSAGE) -> windows_core::BOOL);
    unsafe { WintrustGetDefaultForUsage(dwaction, pszusageoid, psusage as _) }
}
#[inline]
pub unsafe fn WintrustGetRegPolicyFlags() -> u32 {
    windows_core::link!("wintrust.dll" "system" fn WintrustGetRegPolicyFlags(pdwpolicyflags : *mut u32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WintrustGetRegPolicyFlags(&mut result__);
        result__
    }
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WintrustLoadFunctionPointers(pgactionid: *mut windows_core::GUID, ppfns: *mut CRYPT_PROVIDER_FUNCTIONS) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn WintrustLoadFunctionPointers(pgactionid : *mut windows_core::GUID, ppfns : *mut CRYPT_PROVIDER_FUNCTIONS) -> windows_core::BOOL);
    unsafe { WintrustLoadFunctionPointers(pgactionid as _, ppfns as _) }
}
#[inline]
pub unsafe fn WintrustRemoveActionID(pgactionid: *const windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn WintrustRemoveActionID(pgactionid : *const windows_core::GUID) -> windows_core::BOOL);
    unsafe { WintrustRemoveActionID(pgactionid) }
}
#[inline]
pub unsafe fn WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes: bool) {
    windows_core::link!("wintrust.dll" "system" fn WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes : windows_core::BOOL));
    unsafe { WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes.into()) }
}
#[inline]
pub unsafe fn WintrustSetRegPolicyFlags(dwpolicyflags: u32) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn WintrustSetRegPolicyFlags(dwpolicyflags : u32) -> windows_core::BOOL);
    unsafe { WintrustSetRegPolicyFlags(dwpolicyflags) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAT_MEMBERINFO {
    pub pwszSubjGuid: windows_core::PWSTR,
    pub dwCertVersion: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAT_MEMBERINFO2 {
    pub SubjectGuid: windows_core::GUID,
    pub dwCertVersion: u32,
}
pub const CAT_MEMBERINFO2_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.2.3");
pub const CAT_MEMBERINFO2_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2223 as _);
pub const CAT_MEMBERINFO_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.2.2");
pub const CAT_MEMBERINFO_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2222 as _);
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAT_NAMEVALUE {
    pub pwszTag: windows_core::PWSTR,
    pub fdwFlags: u32,
    pub Value: super::CRYPT_DATA_BLOB,
}
pub const CAT_NAMEVALUE_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.2.1");
pub const CAT_NAMEVALUE_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2221 as _);
pub const CERT_CONFIDENCE_AUTHIDEXT: i32 = 65536;
pub const CERT_CONFIDENCE_HIGHEST: i32 = 286330880;
pub const CERT_CONFIDENCE_HYGIENE: i32 = 4096;
pub const CERT_CONFIDENCE_SIG: i32 = 268435456;
pub const CERT_CONFIDENCE_TIME: i32 = 16777216;
pub const CERT_CONFIDENCE_TIMENEST: i32 = 1048576;
pub const CPD_CHOICE_SIP: i32 = 1;
pub const CPD_RETURN_LOWER_QUALITY_CHAINS: i32 = 1048576;
pub const CPD_REVOCATION_CHECK_CHAIN: i32 = 262144;
pub const CPD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: i32 = 524288;
pub const CPD_REVOCATION_CHECK_END_CERT: i32 = 131072;
pub const CPD_REVOCATION_CHECK_NONE: i32 = 65536;
pub const CPD_RFC3161v21: i32 = 2097152;
pub const CPD_UISTATE_MODE_ALLOW: i32 = 2;
pub const CPD_UISTATE_MODE_BLOCK: i32 = 1;
pub const CPD_UISTATE_MODE_MASK: i32 = 3;
pub const CPD_UISTATE_MODE_PROMPT: i32 = 0;
pub const CPD_USE_NT5_CHAIN_FLAG: u32 = 2147483648;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_CERT {
    pub cbStruct: u32,
    pub pCert: super::PCCERT_CONTEXT,
    pub fCommercial: windows_core::BOOL,
    pub fTrustedRoot: windows_core::BOOL,
    pub fSelfSigned: windows_core::BOOL,
    pub fTestCert: windows_core::BOOL,
    pub dwRevokedReason: u32,
    pub dwConfidence: u32,
    pub dwError: u32,
    pub pTrustListContext: *mut super::CTL_CONTEXT,
    pub fTrustListSignerCert: windows_core::BOOL,
    pub pCtlContext: super::PCCTL_CONTEXT,
    pub dwCtlError: u32,
    pub fIsCyclic: windows_core::BOOL,
    pub pChainElement: super::PCERT_CHAIN_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CRYPT_PROVIDER_DATA {
    pub cbStruct: u32,
    pub pWintrustData: *mut WINTRUST_DATA,
    pub fOpenedFile: windows_core::BOOL,
    pub hWndParent: super::HWND,
    pub pgActionID: *mut windows_core::GUID,
    pub hProv: super::HCRYPTPROV,
    pub dwError: u32,
    pub dwRegSecuritySettings: u32,
    pub dwRegPolicySettings: u32,
    pub psPfns: *mut CRYPT_PROVIDER_FUNCTIONS,
    pub cdwTrustStepErrors: u32,
    pub padwTrustStepErrors: *mut u32,
    pub chStores: u32,
    pub pahStores: *mut super::HCERTSTORE,
    pub dwEncoding: u32,
    pub hMsg: super::HCRYPTMSG,
    pub csSigners: u32,
    pub pasSigners: *mut CRYPT_PROVIDER_SGNR,
    pub csProvPrivData: u32,
    pub pasProvPrivData: *mut CRYPT_PROVIDER_PRIVDATA,
    pub dwSubjectChoice: u32,
    pub Anonymous: CRYPT_PROVIDER_DATA_0,
    pub pszUsageOID: *mut i8,
    pub fRecallWithState: windows_core::BOOL,
    pub sftSystemTime: super::FILETIME,
    pub pszCTLSignerUsageOID: *mut i8,
    pub dwProvFlags: u32,
    pub dwFinalError: u32,
    pub pRequestUsage: super::PCERT_USAGE_MATCH,
    pub dwTrustPubSettings: u32,
    pub dwUIStateFlags: u32,
    pub pSigState: *mut CRYPT_PROVIDER_SIGSTATE,
    pub pSigSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
impl Default for CRYPT_PROVIDER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union CRYPT_PROVIDER_DATA_0 {
    pub pPDSip: *mut PROVDATA_SIP,
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
impl Default for CRYPT_PROVIDER_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_DEFUSAGE {
    pub cbStruct: u32,
    pub gActionID: windows_core::GUID,
    pub pDefPolicyCallbackData: *mut core::ffi::c_void,
    pub pDefSIPClientData: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_PROVIDER_FUNCTIONS {
    pub cbStruct: u32,
    pub pfnAlloc: PFN_CPD_MEM_ALLOC,
    pub pfnFree: PFN_CPD_MEM_FREE,
    pub pfnAddStore2Chain: PFN_CPD_ADD_STORE,
    pub pfnAddSgnr2Chain: PFN_CPD_ADD_SGNR,
    pub pfnAddCert2Chain: PFN_CPD_ADD_CERT,
    pub pfnAddPrivData2Chain: PFN_CPD_ADD_PRIVDATA,
    pub pfnInitialize: PFN_PROVIDER_INIT_CALL,
    pub pfnObjectTrust: PFN_PROVIDER_OBJTRUST_CALL,
    pub pfnSignatureTrust: PFN_PROVIDER_SIGTRUST_CALL,
    pub pfnCertificateTrust: PFN_PROVIDER_CERTTRUST_CALL,
    pub pfnFinalPolicy: PFN_PROVIDER_FINALPOLICY_CALL,
    pub pfnCertCheckPolicy: PFN_PROVIDER_CERTCHKPOLICY_CALL,
    pub pfnTestFinalPolicy: PFN_PROVIDER_TESTFINALPOLICY_CALL,
    pub psUIpfns: *mut CRYPT_PROVUI_FUNCS,
    pub pfnCleanupPolicy: PFN_PROVIDER_CLEANUP_CALL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_PRIVDATA {
    pub cbStruct: u32,
    pub gProviderID: windows_core::GUID,
    pub cbProvData: u32,
    pub pvProvData: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_REGDEFUSAGE {
    pub cbStruct: u32,
    pub pgActionID: *mut windows_core::GUID,
    pub pwszDllName: *mut u16,
    pub pwszLoadCallbackDataFunctionName: *mut i8,
    pub pwszFreeCallbackDataFunctionName: *mut i8,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_SGNR {
    pub cbStruct: u32,
    pub sftVerifyAsOf: super::FILETIME,
    pub csCertChain: u32,
    pub pasCertChain: *mut CRYPT_PROVIDER_CERT,
    pub dwSignerType: u32,
    pub psSigner: *mut super::CMSG_SIGNER_INFO,
    pub dwError: u32,
    pub csCounterSigners: u32,
    pub pasCounterSigners: *mut Self,
    pub pChainContext: super::PCCERT_CHAIN_CONTEXT,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_SIGSTATE {
    pub cbStruct: u32,
    pub rhSecondarySigs: *mut super::HCRYPTMSG,
    pub hPrimarySig: super::HCRYPTMSG,
    pub fFirstAttemptMade: windows_core::BOOL,
    pub fNoMoreSigs: windows_core::BOOL,
    pub cSecondarySigs: u32,
    pub dwCurrentIndex: u32,
    pub fSupportMultiSig: windows_core::BOOL,
    pub dwCryptoPolicySupport: u32,
    pub iAttemptCount: u32,
    pub fCheckedSealing: windows_core::BOOL,
    pub pSealingSignature: *mut SEALING_SIGNATURE_ATTRIBUTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVUI_DATA {
    pub cbStruct: u32,
    pub dwFinalError: u32,
    pub pYesButtonText: *mut u16,
    pub pNoButtonText: *mut u16,
    pub pMoreInfoButtonText: *mut u16,
    pub pAdvancedLinkText: *mut u16,
    pub pCopyActionText: *mut u16,
    pub pCopyActionTextNoTS: *mut u16,
    pub pCopyActionTextNotSigned: *mut u16,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_PROVUI_FUNCS {
    pub cbStruct: u32,
    pub psUIData: *mut CRYPT_PROVUI_DATA,
    pub pfnOnMoreInfoClick: PFN_PROVUI_CALL,
    pub pfnOnMoreInfoClickDefault: PFN_PROVUI_CALL,
    pub pfnOnAdvancedClick: PFN_PROVUI_CALL,
    pub pfnOnAdvancedClickDefault: PFN_PROVUI_CALL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_REGISTER_ACTIONID {
    pub cbStruct: u32,
    pub sInitProvider: CRYPT_TRUST_REG_ENTRY,
    pub sObjectProvider: CRYPT_TRUST_REG_ENTRY,
    pub sSignatureProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCertificateProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCertificatePolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sFinalPolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sTestPolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCleanupProvider: CRYPT_TRUST_REG_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_TRUST_REG_ENTRY {
    pub cbStruct: u32,
    pub pwszDLLName: *mut u16,
    pub pwszFunctionName: *mut u16,
}
pub const DWACTION_ALLOCANDFILL: i32 = 1;
pub const DWACTION_FREE: i32 = 2;
#[cfg(feature = "winnt")]
pub type HCATADMIN = super::HANDLE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTENT_TO_SEAL_ATTRIBUTE {
    pub version: u32,
    pub seal: bool,
}
pub const INTENT_TO_SEAL_ATTRIBUTE_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2010 as _);
pub type LPWIN_CERTIFICATE = *mut WIN_CERTIFICATE;
#[cfg(feature = "winnt")]
pub type LPWIN_SPUB_TRUSTED_PUBLISHER_DATA = *mut WIN_SPUB_TRUSTED_PUBLISHER_DATA;
#[cfg(feature = "winnt")]
pub type LPWIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT = *mut WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT;
pub type LPWIN_TRUST_ACTDATA_SUBJECT_ONLY = *mut WIN_TRUST_ACTDATA_SUBJECT_ONLY;
#[cfg(feature = "winnt")]
pub type LPWIN_TRUST_SUBJECT_FILE = *mut WIN_TRUST_SUBJECT_FILE;
#[cfg(feature = "winnt")]
pub type LPWIN_TRUST_SUBJECT_FILE_AND_DISPLAY = *mut WIN_TRUST_SUBJECT_FILE_AND_DISPLAY;
pub type PCAT_MEMBERINFO = *mut CAT_MEMBERINFO;
pub type PCAT_MEMBERINFO2 = *mut CAT_MEMBERINFO2;
#[cfg(feature = "wincrypt")]
pub type PCAT_NAMEVALUE = *mut CAT_NAMEVALUE;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCRYPT_PROVIDER_CERT = *mut CRYPT_PROVIDER_CERT;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PCRYPT_PROVIDER_DATA = *mut CRYPT_PROVIDER_DATA;
pub type PCRYPT_PROVIDER_DEFUSAGE = *mut CRYPT_PROVIDER_DEFUSAGE;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PCRYPT_PROVIDER_FUNCTIONS = *mut CRYPT_PROVIDER_FUNCTIONS;
pub type PCRYPT_PROVIDER_PRIVDATA = *mut CRYPT_PROVIDER_PRIVDATA;
pub type PCRYPT_PROVIDER_REGDEFUSAGE = *mut CRYPT_PROVIDER_REGDEFUSAGE;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PCRYPT_PROVIDER_SGNR = *mut CRYPT_PROVIDER_SGNR;
#[cfg(feature = "wincrypt")]
pub type PCRYPT_PROVIDER_SIGSTATE = *mut CRYPT_PROVIDER_SIGSTATE;
pub type PCRYPT_PROVUI_DATA = *mut CRYPT_PROVUI_DATA;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PCRYPT_PROVUI_FUNCS = *mut CRYPT_PROVUI_FUNCS;
pub type PCRYPT_REGISTER_ACTIONID = *mut CRYPT_REGISTER_ACTIONID;
pub type PCRYPT_TRUST_REG_ENTRY = *mut CRYPT_TRUST_REG_ENTRY;
pub type PFN_ALLOCANDFILLDEFUSAGE = Option<unsafe extern "system" fn(pszusageoid: *const i8, psdefusage: *const CRYPT_PROVIDER_DEFUSAGE) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_CPD_ADD_CERT = Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: windows_core::BOOL, idxcountersigner: u32, pcert2add: *const super::CERT_CONTEXT) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_CPD_ADD_PRIVDATA = Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, pprivdata2add: *const CRYPT_PROVIDER_PRIVDATA) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_CPD_ADD_SGNR = Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, fcountersigner: windows_core::BOOL, idxsigner: u32, psgnr2add: *const CRYPT_PROVIDER_SGNR) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_CPD_ADD_STORE = Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, hstore2add: super::HCERTSTORE) -> windows_core::BOOL>;
pub type PFN_CPD_MEM_ALLOC = Option<unsafe extern "system" fn(cbsize: u32) -> *mut core::ffi::c_void>;
pub type PFN_CPD_MEM_FREE = Option<unsafe extern "system" fn(pvmem2free: *const core::ffi::c_void)>;
pub type PFN_FREEDEFUSAGE = Option<unsafe extern "system" fn(pszusageoid: *const i8, psdefusage: *const CRYPT_PROVIDER_DEFUSAGE) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVIDER_CERTCHKPOLICY_CALL = Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersignerchain: windows_core::BOOL, idxcountersigner: u32) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVIDER_CERTTRUST_CALL = Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVIDER_CLEANUP_CALL = Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVIDER_FINALPOLICY_CALL = Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVIDER_INIT_CALL = Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVIDER_OBJTRUST_CALL = Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVIDER_SIGTRUST_CALL = Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVIDER_TESTFINALPOLICY_CALL = Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PFN_PROVUI_CALL = Option<unsafe extern "system" fn(hwndsecuritydialog: super::HWND, pprovdata: *const CRYPT_PROVIDER_DATA) -> windows_core::BOOL>;
pub type PINTENT_TO_SEAL_ATTRIBUTE = *mut INTENT_TO_SEAL_ATTRIBUTE;
#[cfg(all(feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "winnt"))]
pub type PPROVDATA_SIP = *mut PROVDATA_SIP;
#[repr(C)]
#[cfg(all(feature = "mscat", feature = "mssip", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROVDATA_SIP {
    pub cbStruct: u32,
    pub gSubject: windows_core::GUID,
    pub pSip: *mut super::SIP_DISPATCH_INFO,
    pub pCATSip: *mut super::SIP_DISPATCH_INFO,
    pub psSipSubjectInfo: *mut super::SIP_SUBJECTINFO,
    pub psSipCATSubjectInfo: *mut super::SIP_SUBJECTINFO,
    pub psIndirectData: *mut super::SIP_INDIRECT_DATA,
}
#[cfg(feature = "wincrypt")]
pub type PSEALING_SIGNATURE_ATTRIBUTE = *mut SEALING_SIGNATURE_ATTRIBUTE;
#[cfg(feature = "wincrypt")]
pub type PSEALING_TIMESTAMP_ATTRIBUTE = *mut SEALING_TIMESTAMP_ATTRIBUTE;
pub type PSPC_FINANCIAL_CRITERIA = *mut SPC_FINANCIAL_CRITERIA;
#[cfg(feature = "wincrypt")]
pub type PSPC_IMAGE = *mut SPC_IMAGE;
#[cfg(feature = "wincrypt")]
pub type PSPC_INDIRECT_DATA_CONTENT = *mut SPC_INDIRECT_DATA_CONTENT;
#[cfg(feature = "wincrypt")]
pub type PSPC_LINK = *mut SPC_LINK;
#[cfg(feature = "wincrypt")]
pub type PSPC_PE_IMAGE_DATA = *mut SPC_PE_IMAGE_DATA;
#[cfg(feature = "wincrypt")]
pub type PSPC_SERIALIZED_OBJECT = *mut SPC_SERIALIZED_OBJECT;
pub type PSPC_SIGINFO = *mut SPC_SIGINFO;
#[cfg(feature = "wincrypt")]
pub type PSPC_SP_AGENCY_INFO = *mut SPC_SP_AGENCY_INFO;
#[cfg(feature = "wincrypt")]
pub type PSPC_SP_OPUS_INFO = *mut SPC_SP_OPUS_INFO;
pub type PSPC_STATEMENT_TYPE = *mut SPC_STATEMENT_TYPE;
pub type PWINTRUST_BLOB_INFO = *mut WINTRUST_BLOB_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "winnt"))]
pub type PWINTRUST_CATALOG_INFO = *mut WINTRUST_CATALOG_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PWINTRUST_CERT_INFO = *mut WINTRUST_CERT_INFO;
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "winnt"))]
pub type PWINTRUST_DATA = *mut WINTRUST_DATA;
pub type PWINTRUST_DETACHED_SIG_BLOBS = *mut WINTRUST_DETACHED_SIG_BLOBS;
#[cfg(feature = "winnt")]
pub type PWINTRUST_DETACHED_SIG_FILE_HANDLES = *mut WINTRUST_DETACHED_SIG_FILE_HANDLES;
#[cfg(feature = "winnt")]
pub type PWINTRUST_DETACHED_SIG_INFO = *mut WINTRUST_DETACHED_SIG_INFO;
#[cfg(feature = "winnt")]
pub type PWINTRUST_FILE_INFO = *mut WINTRUST_FILE_INFO;
#[cfg(feature = "wincrypt")]
pub type PWINTRUST_SGNR_INFO = *mut WINTRUST_SGNR_INFO;
#[cfg(feature = "wincrypt")]
pub type PWINTRUST_SIGNATURE_SETTINGS = *mut WINTRUST_SIGNATURE_SETTINGS;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEALING_SIGNATURE_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub signatureAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub encryptedDigest: super::CRYPT_DIGEST_BLOB,
}
pub const SEALING_SIGNATURE_ATTRIBUTE_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2011 as _);
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEALING_TIMESTAMP_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub sealTimeStampToken: super::CRYPT_DATA_BLOB,
}
pub const SEALING_TIMESTAMP_ATTRIBUTE_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2012 as _);
pub const SGNR_TYPE_TIMESTAMP: i32 = 16;
pub const SPC_CAB_DATA_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.25");
pub const SPC_CAB_DATA_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2008 as _);
pub const SPC_CERT_EXTENSIONS_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.14");
pub const SPC_COMMERCIAL_SP_KEY_PURPOSE_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.22");
pub const SPC_ENCRYPTED_DIGEST_RETRY_COUNT_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.6.2");
pub const SPC_FILE_LINK_CHOICE: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPC_FINANCIAL_CRITERIA {
    pub fFinancialInfoAvailable: windows_core::BOOL,
    pub fMeetsCriteria: windows_core::BOOL,
}
pub const SPC_FINANCIAL_CRITERIA_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.27");
pub const SPC_FINANCIAL_CRITERIA_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2002 as _);
pub const SPC_GLUE_RDN_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.25");
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPC_IMAGE {
    pub pImageLink: *mut SPC_LINK,
    pub Bitmap: super::CRYPT_DATA_BLOB,
    pub Metafile: super::CRYPT_DATA_BLOB,
    pub EnhancedMetafile: super::CRYPT_DATA_BLOB,
    pub GifFile: super::CRYPT_DATA_BLOB,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPC_INDIRECT_DATA_CONTENT {
    pub Data: super::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::CRYPT_HASH_BLOB,
}
pub const SPC_INDIRECT_DATA_CONTENT_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2003 as _);
pub const SPC_INDIRECT_DATA_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.4");
pub const SPC_INDIVIDUAL_SP_KEY_PURPOSE_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.21");
pub const SPC_JAVA_CLASS_DATA_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.20");
pub const SPC_JAVA_CLASS_DATA_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2009 as _);
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct SPC_LINK {
    pub dwLinkChoice: u32,
    pub Anonymous: SPC_LINK_0,
}
#[cfg(feature = "wincrypt")]
impl Default for SPC_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub union SPC_LINK_0 {
    pub pwszUrl: windows_core::PWSTR,
    pub Moniker: SPC_SERIALIZED_OBJECT,
    pub pwszFile: windows_core::PWSTR,
}
#[cfg(feature = "wincrypt")]
impl Default for SPC_LINK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPC_LINK_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.28");
pub const SPC_LINK_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2005 as _);
pub const SPC_MINIMAL_CRITERIA_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.26");
pub const SPC_MINIMAL_CRITERIA_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2001 as _);
pub const SPC_MONIKER_LINK_CHOICE: i32 = 2;
pub const SPC_NATURAL_AUTH_PLUGIN_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.96.1.1");
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPC_PE_IMAGE_DATA {
    pub Flags: super::CRYPT_BIT_BLOB,
    pub pFile: PSPC_LINK,
}
pub const SPC_PE_IMAGE_DATA_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.15");
pub const SPC_PE_IMAGE_DATA_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2004 as _);
pub const SPC_PE_IMAGE_PAGE_HASHES_V1_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.3.1");
pub const SPC_PE_IMAGE_PAGE_HASHES_V2_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.3.2");
pub const SPC_RAW_FILE_DATA_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.18");
pub const SPC_RELAXED_PE_MARKER_CHECK_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.6.1");
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPC_SERIALIZED_OBJECT {
    pub ClassId: SPC_UUID,
    pub SerializedData: super::CRYPT_DATA_BLOB,
}
#[cfg(feature = "wincrypt")]
impl Default for SPC_SERIALIZED_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPC_SIGINFO {
    pub dwSipVersion: u32,
    pub gSIPGuid: windows_core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwReserved4: u32,
    pub dwReserved5: u32,
}
pub const SPC_SIGINFO_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.30");
pub const SPC_SIGINFO_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2130 as _);
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPC_SP_AGENCY_INFO {
    pub pPolicyInformation: *mut SPC_LINK,
    pub pwszPolicyDisplayText: windows_core::PWSTR,
    pub pLogoImage: PSPC_IMAGE,
    pub pLogoLink: *mut SPC_LINK,
}
pub const SPC_SP_AGENCY_INFO_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.10");
pub const SPC_SP_AGENCY_INFO_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2000 as _);
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPC_SP_OPUS_INFO {
    pub pwszProgramName: windows_core::PCWSTR,
    pub pMoreInfo: *mut SPC_LINK,
    pub pPublisherInfo: *mut SPC_LINK,
}
pub const SPC_SP_OPUS_INFO_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.12");
pub const SPC_SP_OPUS_INFO_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2007 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPC_STATEMENT_TYPE {
    pub cKeyPurposeId: u32,
    pub rgpszKeyPurposeId: *mut windows_core::PSTR,
}
pub const SPC_STATEMENT_TYPE_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.11");
pub const SPC_STATEMENT_TYPE_STRUCT: windows_core::PCSTR = windows_core::PCSTR(2006 as _);
pub const SPC_STRUCTURED_STORAGE_DATA_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.1.19");
pub const SPC_TIME_STAMP_REQUEST_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.3.2.1");
pub const SPC_URL_LINK_CHOICE: i32 = 1;
pub type SPC_UUID = [u8; 16];
pub const SPC_UUID_LENGTH: i32 = 16;
pub const SPC_WINDOWS_HELLO_COMPATIBILITY_OBJID: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.10.41.1");
pub const TRUSTERROR_MAX_STEPS: i32 = 38;
pub const TRUSTERROR_STEP_CATALOGFILE: i32 = 6;
pub const TRUSTERROR_STEP_CERTSTORE: i32 = 7;
pub const TRUSTERROR_STEP_FILEIO: i32 = 2;
pub const TRUSTERROR_STEP_FINAL_CERTCHKPROV: i32 = 35;
pub const TRUSTERROR_STEP_FINAL_CERTPROV: i32 = 34;
pub const TRUSTERROR_STEP_FINAL_INITPROV: i32 = 31;
pub const TRUSTERROR_STEP_FINAL_OBJPROV: i32 = 32;
pub const TRUSTERROR_STEP_FINAL_POLICYPROV: i32 = 36;
pub const TRUSTERROR_STEP_FINAL_SIGPROV: i32 = 33;
pub const TRUSTERROR_STEP_FINAL_UIPROV: i32 = 37;
pub const TRUSTERROR_STEP_FINAL_WVTINIT: i32 = 30;
pub const TRUSTERROR_STEP_MESSAGE: i32 = 8;
pub const TRUSTERROR_STEP_MSG_CERTCHAIN: i32 = 15;
pub const TRUSTERROR_STEP_MSG_COUNTERSIGCERT: i32 = 17;
pub const TRUSTERROR_STEP_MSG_COUNTERSIGINFO: i32 = 16;
pub const TRUSTERROR_STEP_MSG_INNERCNT: i32 = 11;
pub const TRUSTERROR_STEP_MSG_INNERCNTTYPE: i32 = 10;
pub const TRUSTERROR_STEP_MSG_SIGNERCERT: i32 = 14;
pub const TRUSTERROR_STEP_MSG_SIGNERCOUNT: i32 = 9;
pub const TRUSTERROR_STEP_MSG_SIGNERINFO: i32 = 13;
pub const TRUSTERROR_STEP_MSG_STORE: i32 = 12;
pub const TRUSTERROR_STEP_SIP: i32 = 3;
pub const TRUSTERROR_STEP_SIPSUBJINFO: i32 = 5;
pub const TRUSTERROR_STEP_VERIFY_MSGHASH: i32 = 18;
pub const TRUSTERROR_STEP_VERIFY_MSGINDIRECTDATA: i32 = 19;
pub const TRUSTERROR_STEP_WVTPARAMS: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINTRUST_BLOB_INFO {
    pub cbStruct: u32,
    pub gSubject: windows_core::GUID,
    pub pcwszDisplayName: windows_core::PCWSTR,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINTRUST_CATALOG_INFO {
    pub cbStruct: u32,
    pub dwCatalogVersion: u32,
    pub pcwszCatalogFilePath: windows_core::PCWSTR,
    pub pcwszMemberTag: windows_core::PCWSTR,
    pub pcwszMemberFilePath: windows_core::PCWSTR,
    pub hMemberFile: super::HANDLE,
    pub pbCalculatedFileHash: *mut u8,
    pub cbCalculatedFileHash: u32,
    pub pcCatalogContext: super::PCCTL_CONTEXT,
    pub hCatAdmin: HCATADMIN,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINTRUST_CERT_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: windows_core::PCWSTR,
    pub psCertContext: *mut super::CERT_CONTEXT,
    pub chStores: u32,
    pub pahStores: *mut super::HCERTSTORE,
    pub dwFlags: u32,
    pub psftVerifyAsOf: *mut super::FILETIME,
}
pub const WINTRUST_CONFIG_REGPATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Cryptography\\Wintrust\\Config");
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WINTRUST_DATA {
    pub cbStruct: u32,
    pub pPolicyCallbackData: *mut core::ffi::c_void,
    pub pSIPClientData: *mut core::ffi::c_void,
    pub dwUIChoice: u32,
    pub fdwRevocationChecks: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: WINTRUST_DATA_0,
    pub dwStateAction: u32,
    pub hWVTStateData: super::HANDLE,
    pub pwszURLReference: *mut u16,
    pub dwProvFlags: u32,
    pub dwUIContext: u32,
    pub pSignatureSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "winnt"))]
impl Default for WINTRUST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WINTRUST_DATA_0 {
    pub pFile: *mut WINTRUST_FILE_INFO,
    pub pCatalog: *mut WINTRUST_CATALOG_INFO,
    pub pBlob: *mut WINTRUST_BLOB_INFO,
    pub pSgnr: *mut WINTRUST_SGNR_INFO,
    pub pCert: *mut WINTRUST_CERT_INFO,
    pub pDetachedSig: *mut WINTRUST_DETACHED_SIG_INFO,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "winnt"))]
impl Default for WINTRUST_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINTRUST_DETACHED_SIG_BLOBS {
    pub cbContentObject: i64,
    pub pbContentObject: *mut u8,
    pub cbSignatureObject: u32,
    pub pbSignatureObject: *mut u8,
}
pub const WINTRUST_DETACHED_SIG_CHOICE_BLOB: i32 = 2;
pub const WINTRUST_DETACHED_SIG_CHOICE_HANDLE: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINTRUST_DETACHED_SIG_FILE_HANDLES {
    pub hContentFile: super::HANDLE,
    pub hSignatureFile: super::HANDLE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WINTRUST_DETACHED_SIG_INFO {
    pub cbStruct: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: WINTRUST_DETACHED_SIG_INFO_0,
}
#[cfg(feature = "winnt")]
impl Default for WINTRUST_DETACHED_SIG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union WINTRUST_DETACHED_SIG_INFO_0 {
    pub pDetachedSigHandles: *mut WINTRUST_DETACHED_SIG_FILE_HANDLES,
    pub pDetachedSigBlobs: *mut WINTRUST_DETACHED_SIG_BLOBS,
}
#[cfg(feature = "winnt")]
impl Default for WINTRUST_DETACHED_SIG_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINTRUST_FILE_INFO {
    pub cbStruct: u32,
    pub pcwszFilePath: windows_core::PCWSTR,
    pub hFile: super::HANDLE,
    pub pgKnownSubject: *mut windows_core::GUID,
}
pub const WINTRUST_MAX_HASH_BYTES_TO_MAP_DEFAULT: i32 = 1048576;
pub const WINTRUST_MAX_HASH_BYTES_TO_MAP_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxHashBytesToMap");
pub const WINTRUST_MAX_HEADER_BYTES_TO_MAP_DEFAULT: i32 = 10485760;
pub const WINTRUST_MAX_HEADER_BYTES_TO_MAP_VALUE_NAME: windows_core::PCWSTR = windows_core::w!("MaxHeaderBytesToMap");
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINTRUST_SGNR_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: windows_core::PCWSTR,
    pub psSignerInfo: *mut super::CMSG_SIGNER_INFO,
    pub chStores: u32,
    pub pahStores: *mut super::HCERTSTORE,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINTRUST_SIGNATURE_SETTINGS {
    pub cbStruct: u32,
    pub dwIndex: u32,
    pub dwFlags: u32,
    pub cSecondarySigs: u32,
    pub dwVerifiedSigIndex: u32,
    pub pCryptoPolicy: super::PCERT_STRONG_SIGN_PARA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN_CERTIFICATE {
    pub dwLength: u32,
    pub wRevision: u16,
    pub wCertificateType: u16,
    pub bCertificate: [u8; 1],
}
impl Default for WIN_CERTIFICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIN_CERT_REVISION_1_0: i32 = 256;
pub const WIN_CERT_REVISION_2_0: i32 = 512;
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: i32 = 2;
pub const WIN_CERT_TYPE_RESERVED_1: i32 = 3;
pub const WIN_CERT_TYPE_TS_STACK_SIGNED: i32 = 4;
pub const WIN_CERT_TYPE_X509: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    pub hClientToken: super::HANDLE,
    pub lpCertificate: LPWIN_CERTIFICATE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    pub hClientToken: super::HANDLE,
    pub SubjectType: *mut windows_core::GUID,
    pub Subject: WIN_TRUST_SUBJECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    pub SubjectType: *mut windows_core::GUID,
    pub Subject: WIN_TRUST_SUBJECT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WIN_TRUST_SUBJECT(pub *mut core::ffi::c_void);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN_TRUST_SUBJECT_FILE {
    pub hFile: super::HANDLE,
    pub lpPath: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    pub hFile: super::HANDLE,
    pub lpPath: windows_core::PCWSTR,
    pub lpDisplayName: windows_core::PCWSTR,
}
pub const WSS_CERTTRUST_SUPPORT: i32 = 4;
pub const WSS_GET_SECONDARY_SIG_COUNT: i32 = 2;
pub const WSS_INPUT_FLAG_MASK: i32 = 7;
pub const WSS_OBJTRUST_SUPPORT: i32 = 1;
pub const WSS_OUTPUT_FLAG_MASK: u32 = 3758096384;
pub const WSS_OUT_FILE_SUPPORTS_SEAL: i32 = 536870912;
pub const WSS_OUT_HAS_SEALING_INTENT: i32 = 1073741824;
pub const WSS_OUT_SEALING_STATUS_VERIFIED: u32 = 2147483648;
pub const WSS_SIGTRUST_SUPPORT: i32 = 2;
pub const WSS_VERIFY_SEALING: i32 = 4;
pub const WSS_VERIFY_SPECIFIC: i32 = 1;
pub const WTCI_DONT_OPEN_STORES: i32 = 1;
pub const WTCI_OPEN_ONLY_ROOT: i32 = 2;
pub const WTCI_USE_LOCAL_MACHINE: i32 = 4;
pub const WTD_CACHE_ONLY_URL_RETRIEVAL: i32 = 4096;
pub const WTD_CHOICE_BLOB: i32 = 3;
pub const WTD_CHOICE_CATALOG: i32 = 2;
pub const WTD_CHOICE_CERT: i32 = 5;
pub const WTD_CHOICE_DETACHED_SIG: i32 = 6;
pub const WTD_CHOICE_FILE: i32 = 1;
pub const WTD_CHOICE_SIGNER: i32 = 4;
pub const WTD_CODE_INTEGRITY_DRIVER_MODE: i32 = 32768;
pub const WTD_DISABLE_MD2_MD4: i32 = 8192;
pub const WTD_HASH_ONLY_FLAG: i32 = 512;
pub const WTD_LIFETIME_SIGNING_FLAG: i32 = 2048;
pub const WTD_MOTW: i32 = 16384;
pub const WTD_NO_IE4_CHAIN_FLAG: i32 = 2;
pub const WTD_NO_POLICY_USAGE_FLAG: i32 = 4;
pub const WTD_PROV_FLAGS_MASK: i32 = 65535;
pub const WTD_REVOCATION_CHECK_CHAIN: i32 = 64;
pub const WTD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: i32 = 128;
pub const WTD_REVOCATION_CHECK_END_CERT: i32 = 32;
pub const WTD_REVOCATION_CHECK_NONE: i32 = 16;
pub const WTD_REVOKE_NONE: i32 = 0;
pub const WTD_REVOKE_WHOLECHAIN: i32 = 1;
pub const WTD_SAFER_FLAG: i32 = 256;
pub const WTD_STATEACTION_AUTO_CACHE: i32 = 3;
pub const WTD_STATEACTION_AUTO_CACHE_FLUSH: i32 = 4;
pub const WTD_STATEACTION_CLOSE: i32 = 2;
pub const WTD_STATEACTION_IGNORE: i32 = 0;
pub const WTD_STATEACTION_VERIFY: i32 = 1;
pub const WTD_UICONTEXT_EXECUTE: i32 = 0;
pub const WTD_UICONTEXT_INSTALL: i32 = 1;
pub const WTD_UI_ALL: i32 = 1;
pub const WTD_UI_NOBAD: i32 = 3;
pub const WTD_UI_NOGOOD: i32 = 4;
pub const WTD_UI_NONE: i32 = 2;
pub const WTD_USE_DEFAULT_OSVER_CHECK: i32 = 1024;
pub const WTD_USE_IE4_TRUST_FLAG: i32 = 1;
pub const WTD_USE_LOCAL_MACHINE_CERTS: i32 = 8;
pub const WTPF_ALLOWONLYPERTRUST: i32 = 262144;
pub const WTPF_IGNOREEXPIRATION: i32 = 256;
pub const WTPF_IGNOREREVOCATIONONTS: i32 = 131072;
pub const WTPF_IGNOREREVOKATION: i32 = 512;
pub const WTPF_OFFLINEOKNBU_COM: i32 = 8192;
pub const WTPF_OFFLINEOKNBU_IND: i32 = 4096;
pub const WTPF_OFFLINEOK_COM: i32 = 2048;
pub const WTPF_OFFLINEOK_IND: i32 = 1024;
pub const WTPF_TESTCANBEVALID: i32 = 128;
pub const WTPF_TRUSTTEST: i32 = 32;
pub const WTPF_VERIFY_V1_OFF: i32 = 65536;
pub const WT_ADD_ACTION_ID_RET_RESULT_FLAG: i32 = 1;
pub const WT_CURRENT_VERSION: i32 = 512;
pub const WT_PROVIDER_CERTTRUST_FUNCTION: windows_core::PCWSTR = windows_core::w!("WintrustCertificateTrust");
pub const WT_PROVIDER_DLL_NAME: windows_core::PCWSTR = windows_core::w!("WINTRUST.DLL");
pub const WT_TRUSTDBDIALOG_NO_UI_FLAG: i32 = 1;
pub const WT_TRUSTDBDIALOG_ONLY_PUB_TAB_FLAG: i32 = 2;
pub const WT_TRUSTDBDIALOG_WRITE_IEAK_STORE_FLAG: i32 = 512;
pub const WT_TRUSTDBDIALOG_WRITE_LEGACY_REG_FLAG: i32 = 256;
pub const szOID_ENHANCED_HASH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.5.1");
pub const szOID_INTENT_TO_SEAL: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.4.2");
pub const szOID_NESTED_SIGNATURE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.4.1");
pub const szOID_PKCS_9_SEQUENCE_NUMBER: windows_core::PCSTR = windows_core::s!("1.2.840.113549.1.9.25.4");
pub const szOID_SEALING_SIGNATURE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.4.3");
pub const szOID_SEALING_TIMESTAMP: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.4.4");
pub const szOID_SIGNED_ATTRIBUTE_AUTHOR: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.8");
pub const szOID_SIGNED_ATTRIBUTE_FILE_DESCRIPTION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.3");
pub const szOID_SIGNED_ATTRIBUTE_FILE_VERSION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.2");
pub const szOID_SIGNED_ATTRIBUTE_INTERNAL_NAME: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.1");
pub const szOID_SIGNED_ATTRIBUTE_LANGUAGE: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.7");
pub const szOID_SIGNED_ATTRIBUTE_ORIGINAL_FILENAME: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.6");
pub const szOID_SIGNED_ATTRIBUTE_PRODUCT: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.4");
pub const szOID_SIGNED_ATTRIBUTE_PRODUCT_VERSION: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.5");
pub const szOID_SIGNED_ATTRIBUTE_PUBLISH_TIME: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.9");
pub const szOID_SIGNED_ATTRIBUTE_SOURCE_URL: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.7.10");
pub const szOID_TRUSTED_CLIENT_AUTH_CA_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.2.2");
pub const szOID_TRUSTED_CODESIGNING_CA_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.2.1");
pub const szOID_TRUSTED_SERVER_AUTH_CA_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.2.2.3");
