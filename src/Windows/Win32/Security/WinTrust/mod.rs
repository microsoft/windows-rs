#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct CAT_MEMBERINFO {
    pub pwszSubjGuid: super::super::Foundation::PWSTR,
    pub dwCertVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CAT_MEMBERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CAT_MEMBERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CAT_MEMBERINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAT_MEMBERINFO").field("pwszSubjGuid", &self.pwszSubjGuid).field("dwCertVersion", &self.dwCertVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CAT_MEMBERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszSubjGuid == other.pwszSubjGuid && self.dwCertVersion == other.dwCertVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CAT_MEMBERINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CAT_MEMBERINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub struct CAT_MEMBERINFO2 {
    pub SubjectGuid: ::windows::runtime::GUID,
    pub dwCertVersion: u32,
}
impl CAT_MEMBERINFO2 {}
impl ::std::default::Default for CAT_MEMBERINFO2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAT_MEMBERINFO2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAT_MEMBERINFO2").field("SubjectGuid", &self.SubjectGuid).field("dwCertVersion", &self.dwCertVersion).finish()
    }
}
impl ::std::cmp::PartialEq for CAT_MEMBERINFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectGuid == other.SubjectGuid && self.dwCertVersion == other.dwCertVersion
    }
}
impl ::std::cmp::Eq for CAT_MEMBERINFO2 {}
unsafe impl ::windows::runtime::Abi for CAT_MEMBERINFO2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct CAT_NAMEVALUE {
    pub pwszTag: super::super::Foundation::PWSTR,
    pub fdwFlags: u32,
    pub Value: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl CAT_NAMEVALUE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for CAT_NAMEVALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for CAT_NAMEVALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAT_NAMEVALUE").field("pwszTag", &self.pwszTag).field("fdwFlags", &self.fdwFlags).field("Value", &self.Value).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for CAT_NAMEVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszTag == other.pwszTag && self.fdwFlags == other.fdwFlags && self.Value == other.Value
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for CAT_NAMEVALUE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for CAT_NAMEVALUE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct CONFIG_CI_PROV_INFO {
    pub cbSize: u32,
    pub dwPolicies: u32,
    pub pPolicies: *mut super::Cryptography::CRYPTOAPI_BLOB,
    pub result: CONFIG_CI_PROV_INFO_RESULT,
    pub dwScenario: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl CONFIG_CI_PROV_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for CONFIG_CI_PROV_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for CONFIG_CI_PROV_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONFIG_CI_PROV_INFO").field("cbSize", &self.cbSize).field("dwPolicies", &self.dwPolicies).field("pPolicies", &self.pPolicies).field("result", &self.result).field("dwScenario", &self.dwScenario).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for CONFIG_CI_PROV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwPolicies == other.dwPolicies && self.pPolicies == other.pPolicies && self.result == other.result && self.dwScenario == other.dwScenario
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for CONFIG_CI_PROV_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for CONFIG_CI_PROV_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct CONFIG_CI_PROV_INFO_RESULT {
    pub hr: ::windows::runtime::HRESULT,
    pub dwResult: u32,
    pub dwPolicyIndex: u32,
    pub fIsExplicitDeny: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl CONFIG_CI_PROV_INFO_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONFIG_CI_PROV_INFO_RESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONFIG_CI_PROV_INFO_RESULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONFIG_CI_PROV_INFO_RESULT").field("hr", &self.hr).field("dwResult", &self.dwResult).field("dwPolicyIndex", &self.dwPolicyIndex).field("fIsExplicitDeny", &self.fIsExplicitDeny).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONFIG_CI_PROV_INFO_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.dwResult == other.dwResult && self.dwPolicyIndex == other.dwPolicyIndex && self.fIsExplicitDeny == other.fIsExplicitDeny
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONFIG_CI_PROV_INFO_RESULT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONFIG_CI_PROV_INFO_RESULT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct CRYPT_PROVIDER_CERT {
    pub cbStruct: u32,
    pub pCert: *mut super::Cryptography::CERT_CONTEXT,
    pub fCommercial: super::super::Foundation::BOOL,
    pub fTrustedRoot: super::super::Foundation::BOOL,
    pub fSelfSigned: super::super::Foundation::BOOL,
    pub fTestCert: super::super::Foundation::BOOL,
    pub dwRevokedReason: u32,
    pub dwConfidence: u32,
    pub dwError: u32,
    pub pTrustListContext: *mut super::Cryptography::CTL_CONTEXT,
    pub fTrustListSignerCert: super::super::Foundation::BOOL,
    pub pCtlContext: *mut super::Cryptography::CTL_CONTEXT,
    pub dwCtlError: u32,
    pub fIsCyclic: super::super::Foundation::BOOL,
    pub pChainElement: *mut super::Cryptography::CERT_CHAIN_ELEMENT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl CRYPT_PROVIDER_CERT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for CRYPT_PROVIDER_CERT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for CRYPT_PROVIDER_CERT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_CERT")
            .field("cbStruct", &self.cbStruct)
            .field("pCert", &self.pCert)
            .field("fCommercial", &self.fCommercial)
            .field("fTrustedRoot", &self.fTrustedRoot)
            .field("fSelfSigned", &self.fSelfSigned)
            .field("fTestCert", &self.fTestCert)
            .field("dwRevokedReason", &self.dwRevokedReason)
            .field("dwConfidence", &self.dwConfidence)
            .field("dwError", &self.dwError)
            .field("pTrustListContext", &self.pTrustListContext)
            .field("fTrustListSignerCert", &self.fTrustListSignerCert)
            .field("pCtlContext", &self.pCtlContext)
            .field("dwCtlError", &self.dwCtlError)
            .field("fIsCyclic", &self.fIsCyclic)
            .field("pChainElement", &self.pChainElement)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_CERT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pCert == other.pCert
            && self.fCommercial == other.fCommercial
            && self.fTrustedRoot == other.fTrustedRoot
            && self.fSelfSigned == other.fSelfSigned
            && self.fTestCert == other.fTestCert
            && self.dwRevokedReason == other.dwRevokedReason
            && self.dwConfidence == other.dwConfidence
            && self.dwError == other.dwError
            && self.pTrustListContext == other.pTrustListContext
            && self.fTrustListSignerCert == other.fTrustListSignerCert
            && self.pCtlContext == other.pCtlContext
            && self.dwCtlError == other.dwCtlError
            && self.fIsCyclic == other.fIsCyclic
            && self.pChainElement == other.pChainElement
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_CERT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_CERT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
pub struct CRYPT_PROVIDER_DATA {
    pub cbStruct: u32,
    pub pWintrustData: *mut WINTRUST_DATA,
    pub fOpenedFile: super::super::Foundation::BOOL,
    pub hWndParent: super::super::Foundation::HWND,
    pub pgActionID: *mut ::windows::runtime::GUID,
    pub hProv: usize,
    pub dwError: u32,
    pub dwRegSecuritySettings: u32,
    pub dwRegPolicySettings: u32,
    pub psPfns: *mut CRYPT_PROVIDER_FUNCTIONS,
    pub cdwTrustStepErrors: u32,
    pub padwTrustStepErrors: *mut u32,
    pub chStores: u32,
    pub pahStores: *mut *mut ::std::ffi::c_void,
    pub dwEncoding: u32,
    pub hMsg: *mut ::std::ffi::c_void,
    pub csSigners: u32,
    pub pasSigners: *mut CRYPT_PROVIDER_SGNR,
    pub csProvPrivData: u32,
    pub pasProvPrivData: *mut CRYPT_PROVIDER_PRIVDATA,
    pub dwSubjectChoice: u32,
    pub Anonymous: CRYPT_PROVIDER_DATA_0,
    pub pszUsageOID: super::super::Foundation::PSTR,
    pub fRecallWithState: super::super::Foundation::BOOL,
    pub sftSystemTime: super::super::Foundation::FILETIME,
    pub pszCTLSignerUsageOID: super::super::Foundation::PSTR,
    pub dwProvFlags: u32,
    pub dwFinalError: u32,
    pub pRequestUsage: *mut super::Cryptography::CERT_USAGE_MATCH,
    pub dwTrustPubSettings: u32,
    pub dwUIStateFlags: u32,
    pub pSigState: *mut CRYPT_PROVIDER_SIGSTATE,
    pub pSigSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl CRYPT_PROVIDER_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::default::Default for CRYPT_PROVIDER_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
pub union CRYPT_PROVIDER_DATA_0 {
    pub pPDSip: *mut PROVDATA_SIP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl CRYPT_PROVIDER_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::default::Default for CRYPT_PROVIDER_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_DATA_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub struct CRYPT_PROVIDER_DEFUSAGE {
    pub cbStruct: u32,
    pub gActionID: ::windows::runtime::GUID,
    pub pDefPolicyCallbackData: *mut ::std::ffi::c_void,
    pub pDefSIPClientData: *mut ::std::ffi::c_void,
}
impl CRYPT_PROVIDER_DEFUSAGE {}
impl ::std::default::Default for CRYPT_PROVIDER_DEFUSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CRYPT_PROVIDER_DEFUSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_DEFUSAGE").field("cbStruct", &self.cbStruct).field("gActionID", &self.gActionID).field("pDefPolicyCallbackData", &self.pDefPolicyCallbackData).field("pDefSIPClientData", &self.pDefSIPClientData).finish()
    }
}
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_DEFUSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.gActionID == other.gActionID && self.pDefPolicyCallbackData == other.pDefPolicyCallbackData && self.pDefSIPClientData == other.pDefSIPClientData
    }
}
impl ::std::cmp::Eq for CRYPT_PROVIDER_DEFUSAGE {}
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_DEFUSAGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
pub struct CRYPT_PROVIDER_FUNCTIONS {
    pub cbStruct: u32,
    pub pfnAlloc: ::std::option::Option<PFN_CPD_MEM_ALLOC>,
    pub pfnFree: ::std::option::Option<PFN_CPD_MEM_FREE>,
    pub pfnAddStore2Chain: ::std::option::Option<PFN_CPD_ADD_STORE>,
    pub pfnAddSgnr2Chain: ::std::option::Option<PFN_CPD_ADD_SGNR>,
    pub pfnAddCert2Chain: ::std::option::Option<PFN_CPD_ADD_CERT>,
    pub pfnAddPrivData2Chain: ::std::option::Option<PFN_CPD_ADD_PRIVDATA>,
    pub pfnInitialize: ::std::option::Option<PFN_PROVIDER_INIT_CALL>,
    pub pfnObjectTrust: ::std::option::Option<PFN_PROVIDER_OBJTRUST_CALL>,
    pub pfnSignatureTrust: ::std::option::Option<PFN_PROVIDER_SIGTRUST_CALL>,
    pub pfnCertificateTrust: ::std::option::Option<PFN_PROVIDER_CERTTRUST_CALL>,
    pub pfnFinalPolicy: ::std::option::Option<PFN_PROVIDER_FINALPOLICY_CALL>,
    pub pfnCertCheckPolicy: ::std::option::Option<PFN_PROVIDER_CERTCHKPOLICY_CALL>,
    pub pfnTestFinalPolicy: ::std::option::Option<PFN_PROVIDER_TESTFINALPOLICY_CALL>,
    pub psUIpfns: *mut CRYPT_PROVUI_FUNCS,
    pub pfnCleanupPolicy: ::std::option::Option<PFN_PROVIDER_CLEANUP_CALL>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl CRYPT_PROVIDER_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::default::Default for CRYPT_PROVIDER_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::fmt::Debug for CRYPT_PROVIDER_FUNCTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_FUNCTIONS").field("cbStruct", &self.cbStruct).field("psUIpfns", &self.psUIpfns).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_FUNCTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pfnAlloc.map(|f| f as usize) == other.pfnAlloc.map(|f| f as usize)
            && self.pfnFree.map(|f| f as usize) == other.pfnFree.map(|f| f as usize)
            && self.pfnAddStore2Chain.map(|f| f as usize) == other.pfnAddStore2Chain.map(|f| f as usize)
            && self.pfnAddSgnr2Chain.map(|f| f as usize) == other.pfnAddSgnr2Chain.map(|f| f as usize)
            && self.pfnAddCert2Chain.map(|f| f as usize) == other.pfnAddCert2Chain.map(|f| f as usize)
            && self.pfnAddPrivData2Chain.map(|f| f as usize) == other.pfnAddPrivData2Chain.map(|f| f as usize)
            && self.pfnInitialize.map(|f| f as usize) == other.pfnInitialize.map(|f| f as usize)
            && self.pfnObjectTrust.map(|f| f as usize) == other.pfnObjectTrust.map(|f| f as usize)
            && self.pfnSignatureTrust.map(|f| f as usize) == other.pfnSignatureTrust.map(|f| f as usize)
            && self.pfnCertificateTrust.map(|f| f as usize) == other.pfnCertificateTrust.map(|f| f as usize)
            && self.pfnFinalPolicy.map(|f| f as usize) == other.pfnFinalPolicy.map(|f| f as usize)
            && self.pfnCertCheckPolicy.map(|f| f as usize) == other.pfnCertCheckPolicy.map(|f| f as usize)
            && self.pfnTestFinalPolicy.map(|f| f as usize) == other.pfnTestFinalPolicy.map(|f| f as usize)
            && self.psUIpfns == other.psUIpfns
            && self.pfnCleanupPolicy.map(|f| f as usize) == other.pfnCleanupPolicy.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_FUNCTIONS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub struct CRYPT_PROVIDER_PRIVDATA {
    pub cbStruct: u32,
    pub gProviderID: ::windows::runtime::GUID,
    pub cbProvData: u32,
    pub pvProvData: *mut ::std::ffi::c_void,
}
impl CRYPT_PROVIDER_PRIVDATA {}
impl ::std::default::Default for CRYPT_PROVIDER_PRIVDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CRYPT_PROVIDER_PRIVDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_PRIVDATA").field("cbStruct", &self.cbStruct).field("gProviderID", &self.gProviderID).field("cbProvData", &self.cbProvData).field("pvProvData", &self.pvProvData).finish()
    }
}
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_PRIVDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.gProviderID == other.gProviderID && self.cbProvData == other.cbProvData && self.pvProvData == other.pvProvData
    }
}
impl ::std::cmp::Eq for CRYPT_PROVIDER_PRIVDATA {}
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_PRIVDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct CRYPT_PROVIDER_REGDEFUSAGE {
    pub cbStruct: u32,
    pub pgActionID: *mut ::windows::runtime::GUID,
    pub pwszDllName: super::super::Foundation::PWSTR,
    pub pwszLoadCallbackDataFunctionName: super::super::Foundation::PSTR,
    pub pwszFreeCallbackDataFunctionName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPT_PROVIDER_REGDEFUSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPT_PROVIDER_REGDEFUSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPT_PROVIDER_REGDEFUSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_REGDEFUSAGE")
            .field("cbStruct", &self.cbStruct)
            .field("pgActionID", &self.pgActionID)
            .field("pwszDllName", &self.pwszDllName)
            .field("pwszLoadCallbackDataFunctionName", &self.pwszLoadCallbackDataFunctionName)
            .field("pwszFreeCallbackDataFunctionName", &self.pwszFreeCallbackDataFunctionName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_REGDEFUSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pgActionID == other.pgActionID && self.pwszDllName == other.pwszDllName && self.pwszLoadCallbackDataFunctionName == other.pwszLoadCallbackDataFunctionName && self.pwszFreeCallbackDataFunctionName == other.pwszFreeCallbackDataFunctionName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPT_PROVIDER_REGDEFUSAGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_REGDEFUSAGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct CRYPT_PROVIDER_SGNR {
    pub cbStruct: u32,
    pub sftVerifyAsOf: super::super::Foundation::FILETIME,
    pub csCertChain: u32,
    pub pasCertChain: *mut CRYPT_PROVIDER_CERT,
    pub dwSignerType: u32,
    pub psSigner: *mut super::Cryptography::CMSG_SIGNER_INFO,
    pub dwError: u32,
    pub csCounterSigners: u32,
    pub pasCounterSigners: *mut CRYPT_PROVIDER_SGNR,
    pub pChainContext: *mut super::Cryptography::CERT_CHAIN_CONTEXT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl CRYPT_PROVIDER_SGNR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for CRYPT_PROVIDER_SGNR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for CRYPT_PROVIDER_SGNR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_SGNR")
            .field("cbStruct", &self.cbStruct)
            .field("sftVerifyAsOf", &self.sftVerifyAsOf)
            .field("csCertChain", &self.csCertChain)
            .field("pasCertChain", &self.pasCertChain)
            .field("dwSignerType", &self.dwSignerType)
            .field("psSigner", &self.psSigner)
            .field("dwError", &self.dwError)
            .field("csCounterSigners", &self.csCounterSigners)
            .field("pasCounterSigners", &self.pasCounterSigners)
            .field("pChainContext", &self.pChainContext)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_SGNR {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.sftVerifyAsOf == other.sftVerifyAsOf && self.csCertChain == other.csCertChain && self.pasCertChain == other.pasCertChain && self.dwSignerType == other.dwSignerType && self.psSigner == other.psSigner && self.dwError == other.dwError && self.csCounterSigners == other.csCounterSigners && self.pasCounterSigners == other.pasCounterSigners && self.pChainContext == other.pChainContext
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_SGNR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_SGNR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct CRYPT_PROVIDER_SIGSTATE {
    pub cbStruct: u32,
    pub rhSecondarySigs: *mut *mut ::std::ffi::c_void,
    pub hPrimarySig: *mut ::std::ffi::c_void,
    pub fFirstAttemptMade: super::super::Foundation::BOOL,
    pub fNoMoreSigs: super::super::Foundation::BOOL,
    pub cSecondarySigs: u32,
    pub dwCurrentIndex: u32,
    pub fSupportMultiSig: super::super::Foundation::BOOL,
    pub dwCryptoPolicySupport: u32,
    pub iAttemptCount: u32,
    pub fCheckedSealing: super::super::Foundation::BOOL,
    pub pSealingSignature: *mut SEALING_SIGNATURE_ATTRIBUTE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl CRYPT_PROVIDER_SIGSTATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for CRYPT_PROVIDER_SIGSTATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for CRYPT_PROVIDER_SIGSTATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_SIGSTATE")
            .field("cbStruct", &self.cbStruct)
            .field("rhSecondarySigs", &self.rhSecondarySigs)
            .field("hPrimarySig", &self.hPrimarySig)
            .field("fFirstAttemptMade", &self.fFirstAttemptMade)
            .field("fNoMoreSigs", &self.fNoMoreSigs)
            .field("cSecondarySigs", &self.cSecondarySigs)
            .field("dwCurrentIndex", &self.dwCurrentIndex)
            .field("fSupportMultiSig", &self.fSupportMultiSig)
            .field("dwCryptoPolicySupport", &self.dwCryptoPolicySupport)
            .field("iAttemptCount", &self.iAttemptCount)
            .field("fCheckedSealing", &self.fCheckedSealing)
            .field("pSealingSignature", &self.pSealingSignature)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_SIGSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.rhSecondarySigs == other.rhSecondarySigs
            && self.hPrimarySig == other.hPrimarySig
            && self.fFirstAttemptMade == other.fFirstAttemptMade
            && self.fNoMoreSigs == other.fNoMoreSigs
            && self.cSecondarySigs == other.cSecondarySigs
            && self.dwCurrentIndex == other.dwCurrentIndex
            && self.fSupportMultiSig == other.fSupportMultiSig
            && self.dwCryptoPolicySupport == other.dwCryptoPolicySupport
            && self.iAttemptCount == other.iAttemptCount
            && self.fCheckedSealing == other.fCheckedSealing
            && self.pSealingSignature == other.pSealingSignature
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_SIGSTATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_SIGSTATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct CRYPT_PROVUI_DATA {
    pub cbStruct: u32,
    pub dwFinalError: u32,
    pub pYesButtonText: super::super::Foundation::PWSTR,
    pub pNoButtonText: super::super::Foundation::PWSTR,
    pub pMoreInfoButtonText: super::super::Foundation::PWSTR,
    pub pAdvancedLinkText: super::super::Foundation::PWSTR,
    pub pCopyActionText: super::super::Foundation::PWSTR,
    pub pCopyActionTextNoTS: super::super::Foundation::PWSTR,
    pub pCopyActionTextNotSigned: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPT_PROVUI_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPT_PROVUI_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPT_PROVUI_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVUI_DATA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFinalError", &self.dwFinalError)
            .field("pYesButtonText", &self.pYesButtonText)
            .field("pNoButtonText", &self.pNoButtonText)
            .field("pMoreInfoButtonText", &self.pMoreInfoButtonText)
            .field("pAdvancedLinkText", &self.pAdvancedLinkText)
            .field("pCopyActionText", &self.pCopyActionText)
            .field("pCopyActionTextNoTS", &self.pCopyActionTextNoTS)
            .field("pCopyActionTextNotSigned", &self.pCopyActionTextNotSigned)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPT_PROVUI_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFinalError == other.dwFinalError && self.pYesButtonText == other.pYesButtonText && self.pNoButtonText == other.pNoButtonText && self.pMoreInfoButtonText == other.pMoreInfoButtonText && self.pAdvancedLinkText == other.pAdvancedLinkText && self.pCopyActionText == other.pCopyActionText && self.pCopyActionTextNoTS == other.pCopyActionTextNoTS && self.pCopyActionTextNotSigned == other.pCopyActionTextNotSigned
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPT_PROVUI_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVUI_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
pub struct CRYPT_PROVUI_FUNCS {
    pub cbStruct: u32,
    pub psUIData: *mut CRYPT_PROVUI_DATA,
    pub pfnOnMoreInfoClick: ::std::option::Option<PFN_PROVUI_CALL>,
    pub pfnOnMoreInfoClickDefault: ::std::option::Option<PFN_PROVUI_CALL>,
    pub pfnOnAdvancedClick: ::std::option::Option<PFN_PROVUI_CALL>,
    pub pfnOnAdvancedClickDefault: ::std::option::Option<PFN_PROVUI_CALL>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl CRYPT_PROVUI_FUNCS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::default::Default for CRYPT_PROVUI_FUNCS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::fmt::Debug for CRYPT_PROVUI_FUNCS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVUI_FUNCS").field("cbStruct", &self.cbStruct).field("psUIData", &self.psUIData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::PartialEq for CRYPT_PROVUI_FUNCS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.psUIData == other.psUIData && self.pfnOnMoreInfoClick.map(|f| f as usize) == other.pfnOnMoreInfoClick.map(|f| f as usize) && self.pfnOnMoreInfoClickDefault.map(|f| f as usize) == other.pfnOnMoreInfoClickDefault.map(|f| f as usize) && self.pfnOnAdvancedClick.map(|f| f as usize) == other.pfnOnAdvancedClick.map(|f| f as usize) && self.pfnOnAdvancedClickDefault.map(|f| f as usize) == other.pfnOnAdvancedClickDefault.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::Eq for CRYPT_PROVUI_FUNCS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVUI_FUNCS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl CRYPT_REGISTER_ACTIONID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPT_REGISTER_ACTIONID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPT_REGISTER_ACTIONID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_REGISTER_ACTIONID")
            .field("cbStruct", &self.cbStruct)
            .field("sInitProvider", &self.sInitProvider)
            .field("sObjectProvider", &self.sObjectProvider)
            .field("sSignatureProvider", &self.sSignatureProvider)
            .field("sCertificateProvider", &self.sCertificateProvider)
            .field("sCertificatePolicyProvider", &self.sCertificatePolicyProvider)
            .field("sFinalPolicyProvider", &self.sFinalPolicyProvider)
            .field("sTestPolicyProvider", &self.sTestPolicyProvider)
            .field("sCleanupProvider", &self.sCleanupProvider)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPT_REGISTER_ACTIONID {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.sInitProvider == other.sInitProvider && self.sObjectProvider == other.sObjectProvider && self.sSignatureProvider == other.sSignatureProvider && self.sCertificateProvider == other.sCertificateProvider && self.sCertificatePolicyProvider == other.sCertificatePolicyProvider && self.sFinalPolicyProvider == other.sFinalPolicyProvider && self.sTestPolicyProvider == other.sTestPolicyProvider && self.sCleanupProvider == other.sCleanupProvider
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPT_REGISTER_ACTIONID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPT_REGISTER_ACTIONID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct CRYPT_TRUST_REG_ENTRY {
    pub cbStruct: u32,
    pub pwszDLLName: super::super::Foundation::PWSTR,
    pub pwszFunctionName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPT_TRUST_REG_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPT_TRUST_REG_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPT_TRUST_REG_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_TRUST_REG_ENTRY").field("cbStruct", &self.cbStruct).field("pwszDLLName", &self.pwszDLLName).field("pwszFunctionName", &self.pwszFunctionName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPT_TRUST_REG_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pwszDLLName == other.pwszDLLName && self.pwszFunctionName == other.pwszFunctionName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPT_TRUST_REG_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPT_TRUST_REG_ENTRY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct DRIVER_VER_INFO {
    pub cbStruct: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub dwPlatform: u32,
    pub dwVersion: u32,
    pub wszVersion: [u16; 260],
    pub wszSignedBy: [u16; 260],
    pub pcSignerCertContext: *mut super::Cryptography::CERT_CONTEXT,
    pub sOSVersionLow: DRIVER_VER_MAJORMINOR,
    pub sOSVersionHigh: DRIVER_VER_MAJORMINOR,
    pub dwBuildNumberLow: u32,
    pub dwBuildNumberHigh: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl DRIVER_VER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for DRIVER_VER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for DRIVER_VER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRIVER_VER_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwVersion", &self.dwVersion)
            .field("wszVersion", &self.wszVersion)
            .field("wszSignedBy", &self.wszSignedBy)
            .field("pcSignerCertContext", &self.pcSignerCertContext)
            .field("sOSVersionLow", &self.sOSVersionLow)
            .field("sOSVersionHigh", &self.sOSVersionHigh)
            .field("dwBuildNumberLow", &self.dwBuildNumberLow)
            .field("dwBuildNumberHigh", &self.dwBuildNumberHigh)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for DRIVER_VER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwPlatform == other.dwPlatform
            && self.dwVersion == other.dwVersion
            && self.wszVersion == other.wszVersion
            && self.wszSignedBy == other.wszSignedBy
            && self.pcSignerCertContext == other.pcSignerCertContext
            && self.sOSVersionLow == other.sOSVersionLow
            && self.sOSVersionHigh == other.sOSVersionHigh
            && self.dwBuildNumberLow == other.dwBuildNumberLow
            && self.dwBuildNumberHigh == other.dwBuildNumberHigh
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for DRIVER_VER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for DRIVER_VER_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub struct DRIVER_VER_MAJORMINOR {
    pub dwMajor: u32,
    pub dwMinor: u32,
}
impl DRIVER_VER_MAJORMINOR {}
impl ::std::default::Default for DRIVER_VER_MAJORMINOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRIVER_VER_MAJORMINOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRIVER_VER_MAJORMINOR").field("dwMajor", &self.dwMajor).field("dwMinor", &self.dwMinor).finish()
    }
}
impl ::std::cmp::PartialEq for DRIVER_VER_MAJORMINOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwMajor == other.dwMajor && self.dwMinor == other.dwMinor
    }
}
impl ::std::cmp::Eq for DRIVER_VER_MAJORMINOR {}
unsafe impl ::windows::runtime::Abi for DRIVER_VER_MAJORMINOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct INTENT_TO_SEAL_ATTRIBUTE {
    pub version: u32,
    pub seal: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl INTENT_TO_SEAL_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for INTENT_TO_SEAL_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for INTENT_TO_SEAL_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTENT_TO_SEAL_ATTRIBUTE").field("version", &self.version).field("seal", &self.seal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for INTENT_TO_SEAL_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.seal == other.seal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for INTENT_TO_SEAL_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INTENT_TO_SEAL_ATTRIBUTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn OpenPersonalTrustDBDialog<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPersonalTrustDBDialog(hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OpenPersonalTrustDBDialog(hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn OpenPersonalTrustDBDialogEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, dwflags: u32, pvreserved: *mut *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPersonalTrustDBDialogEx(hwndparent: super::super::Foundation::HWND, dwflags: u32, pvreserved: *mut *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OpenPersonalTrustDBDialogEx(hwndparent.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type PFN_ALLOCANDFILLDEFUSAGE = unsafe extern "system" fn(pszusageoid: super::super::Foundation::PSTR, psdefusage: *const CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_CERT = unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: super::super::Foundation::BOOL, idxcountersigner: u32, pcert2add: *const super::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_PRIVDATA = unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, pprivdata2add: *const CRYPT_PROVIDER_PRIVDATA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_SGNR = unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, fcountersigner: super::super::Foundation::BOOL, idxsigner: u32, psgnr2add: *const CRYPT_PROVIDER_SGNR) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_STORE = unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, hstore2add: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
pub type PFN_CPD_MEM_ALLOC = unsafe extern "system" fn(cbsize: u32) -> *mut ::std::ffi::c_void;
pub type PFN_CPD_MEM_FREE = unsafe extern "system" fn(pvmem2free: *const ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FREEDEFUSAGE = unsafe extern "system" fn(pszusageoid: super::super::Foundation::PSTR, psdefusage: *const CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CERTCHKPOLICY_CALL = unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersignerchain: super::super::Foundation::BOOL, idxcountersigner: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CERTTRUST_CALL = unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CLEANUP_CALL = unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_FINALPOLICY_CALL = unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_INIT_CALL = unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_OBJTRUST_CALL = unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_SIGTRUST_CALL = unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_TESTFINALPOLICY_CALL = unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVUI_CALL = unsafe extern "system" fn(hwndsecuritydialog: super::super::Foundation::HWND, pprovdata: *const CRYPT_PROVIDER_DATA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK = unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA, dwsteperror: u32, dwregpolicysettings: u32, csigner: u32, rgpsigner: *mut *mut WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO, pvpolicyarg: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
pub struct PROVDATA_SIP {
    pub cbStruct: u32,
    pub gSubject: ::windows::runtime::GUID,
    pub pSip: *mut super::Cryptography::Sip::SIP_DISPATCH_INFO,
    pub pCATSip: *mut super::Cryptography::Sip::SIP_DISPATCH_INFO,
    pub psSipSubjectInfo: *mut super::Cryptography::Sip::SIP_SUBJECTINFO,
    pub psSipCATSubjectInfo: *mut super::Cryptography::Sip::SIP_SUBJECTINFO,
    pub psIndirectData: *mut super::Cryptography::Sip::SIP_INDIRECT_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl PROVDATA_SIP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::default::Default for PROVDATA_SIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::fmt::Debug for PROVDATA_SIP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROVDATA_SIP")
            .field("cbStruct", &self.cbStruct)
            .field("gSubject", &self.gSubject)
            .field("pSip", &self.pSip)
            .field("pCATSip", &self.pCATSip)
            .field("psSipSubjectInfo", &self.psSipSubjectInfo)
            .field("psSipCATSubjectInfo", &self.psSipCATSubjectInfo)
            .field("psIndirectData", &self.psIndirectData)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::PartialEq for PROVDATA_SIP {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.gSubject == other.gSubject && self.pSip == other.pSip && self.pCATSip == other.pCATSip && self.psSipSubjectInfo == other.psSipSubjectInfo && self.psSipCATSubjectInfo == other.psSipCATSubjectInfo && self.psIndirectData == other.psIndirectData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::Eq for PROVDATA_SIP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::runtime::Abi for PROVDATA_SIP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct SEALING_SIGNATURE_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub signatureAlgorithm: super::Cryptography::CRYPT_ALGORITHM_IDENTIFIER,
    pub encryptedDigest: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SEALING_SIGNATURE_ATTRIBUTE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SEALING_SIGNATURE_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for SEALING_SIGNATURE_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SEALING_SIGNATURE_ATTRIBUTE").field("version", &self.version).field("signerIndex", &self.signerIndex).field("signatureAlgorithm", &self.signatureAlgorithm).field("encryptedDigest", &self.encryptedDigest).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SEALING_SIGNATURE_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.signerIndex == other.signerIndex && self.signatureAlgorithm == other.signatureAlgorithm && self.encryptedDigest == other.encryptedDigest
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SEALING_SIGNATURE_ATTRIBUTE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SEALING_SIGNATURE_ATTRIBUTE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Security_Cryptography`*"]
pub struct SEALING_TIMESTAMP_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub sealTimeStampToken: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl SEALING_TIMESTAMP_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::default::Default for SEALING_TIMESTAMP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::fmt::Debug for SEALING_TIMESTAMP_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SEALING_TIMESTAMP_ATTRIBUTE").field("version", &self.version).field("signerIndex", &self.signerIndex).field("sealTimeStampToken", &self.sealTimeStampToken).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::PartialEq for SEALING_TIMESTAMP_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.signerIndex == other.signerIndex && self.sealTimeStampToken == other.sealTimeStampToken
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::Eq for SEALING_TIMESTAMP_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::runtime::Abi for SEALING_TIMESTAMP_ATTRIBUTE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct SPC_FINANCIAL_CRITERIA {
    pub fFinancialInfoAvailable: super::super::Foundation::BOOL,
    pub fMeetsCriteria: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl SPC_FINANCIAL_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SPC_FINANCIAL_CRITERIA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SPC_FINANCIAL_CRITERIA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_FINANCIAL_CRITERIA").field("fFinancialInfoAvailable", &self.fFinancialInfoAvailable).field("fMeetsCriteria", &self.fMeetsCriteria).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SPC_FINANCIAL_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        self.fFinancialInfoAvailable == other.fFinancialInfoAvailable && self.fMeetsCriteria == other.fMeetsCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SPC_FINANCIAL_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SPC_FINANCIAL_CRITERIA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct SPC_IMAGE {
    pub pImageLink: *mut SPC_LINK,
    pub Bitmap: super::Cryptography::CRYPTOAPI_BLOB,
    pub Metafile: super::Cryptography::CRYPTOAPI_BLOB,
    pub EnhancedMetafile: super::Cryptography::CRYPTOAPI_BLOB,
    pub GifFile: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SPC_IMAGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SPC_IMAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for SPC_IMAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_IMAGE").field("pImageLink", &self.pImageLink).field("Bitmap", &self.Bitmap).field("Metafile", &self.Metafile).field("EnhancedMetafile", &self.EnhancedMetafile).field("GifFile", &self.GifFile).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SPC_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.pImageLink == other.pImageLink && self.Bitmap == other.Bitmap && self.Metafile == other.Metafile && self.EnhancedMetafile == other.EnhancedMetafile && self.GifFile == other.GifFile
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SPC_IMAGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SPC_IMAGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct SPC_INDIRECT_DATA_CONTENT {
    pub Data: super::Cryptography::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::Cryptography::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SPC_INDIRECT_DATA_CONTENT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SPC_INDIRECT_DATA_CONTENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for SPC_INDIRECT_DATA_CONTENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_INDIRECT_DATA_CONTENT").field("Data", &self.Data).field("DigestAlgorithm", &self.DigestAlgorithm).field("Digest", &self.Digest).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SPC_INDIRECT_DATA_CONTENT {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data && self.DigestAlgorithm == other.DigestAlgorithm && self.Digest == other.Digest
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SPC_INDIRECT_DATA_CONTENT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SPC_INDIRECT_DATA_CONTENT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct SPC_LINK {
    pub dwLinkChoice: u32,
    pub Anonymous: SPC_LINK_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SPC_LINK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SPC_LINK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SPC_LINK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SPC_LINK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SPC_LINK {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub union SPC_LINK_0 {
    pub pwszUrl: super::super::Foundation::PWSTR,
    pub Moniker: SPC_SERIALIZED_OBJECT,
    pub pwszFile: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SPC_LINK_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SPC_LINK_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SPC_LINK_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SPC_LINK_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SPC_LINK_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct SPC_PE_IMAGE_DATA {
    pub Flags: super::Cryptography::CRYPT_BIT_BLOB,
    pub pFile: *mut SPC_LINK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SPC_PE_IMAGE_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SPC_PE_IMAGE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for SPC_PE_IMAGE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_PE_IMAGE_DATA").field("Flags", &self.Flags).field("pFile", &self.pFile).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SPC_PE_IMAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pFile == other.pFile
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SPC_PE_IMAGE_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SPC_PE_IMAGE_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Security_Cryptography`*"]
pub struct SPC_SERIALIZED_OBJECT {
    pub ClassId: [u8; 16],
    pub SerializedData: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl SPC_SERIALIZED_OBJECT {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::default::Default for SPC_SERIALIZED_OBJECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::fmt::Debug for SPC_SERIALIZED_OBJECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_SERIALIZED_OBJECT").field("ClassId", &self.ClassId).field("SerializedData", &self.SerializedData).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::PartialEq for SPC_SERIALIZED_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.ClassId == other.ClassId && self.SerializedData == other.SerializedData
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::Eq for SPC_SERIALIZED_OBJECT {}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::runtime::Abi for SPC_SERIALIZED_OBJECT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub struct SPC_SIGINFO {
    pub dwSipVersion: u32,
    pub gSIPGuid: ::windows::runtime::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwReserved4: u32,
    pub dwReserved5: u32,
}
impl SPC_SIGINFO {}
impl ::std::default::Default for SPC_SIGINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SPC_SIGINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_SIGINFO")
            .field("dwSipVersion", &self.dwSipVersion)
            .field("gSIPGuid", &self.gSIPGuid)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwReserved4", &self.dwReserved4)
            .field("dwReserved5", &self.dwReserved5)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SPC_SIGINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSipVersion == other.dwSipVersion && self.gSIPGuid == other.gSIPGuid && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3 && self.dwReserved4 == other.dwReserved4 && self.dwReserved5 == other.dwReserved5
    }
}
impl ::std::cmp::Eq for SPC_SIGINFO {}
unsafe impl ::windows::runtime::Abi for SPC_SIGINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct SPC_SP_AGENCY_INFO {
    pub pPolicyInformation: *mut SPC_LINK,
    pub pwszPolicyDisplayText: super::super::Foundation::PWSTR,
    pub pLogoImage: *mut SPC_IMAGE,
    pub pLogoLink: *mut SPC_LINK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SPC_SP_AGENCY_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SPC_SP_AGENCY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for SPC_SP_AGENCY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_SP_AGENCY_INFO").field("pPolicyInformation", &self.pPolicyInformation).field("pwszPolicyDisplayText", &self.pwszPolicyDisplayText).field("pLogoImage", &self.pLogoImage).field("pLogoLink", &self.pLogoLink).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SPC_SP_AGENCY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pPolicyInformation == other.pPolicyInformation && self.pwszPolicyDisplayText == other.pwszPolicyDisplayText && self.pLogoImage == other.pLogoImage && self.pLogoLink == other.pLogoLink
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SPC_SP_AGENCY_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SPC_SP_AGENCY_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct SPC_SP_OPUS_INFO {
    pub pwszProgramName: super::super::Foundation::PWSTR,
    pub pMoreInfo: *mut SPC_LINK,
    pub pPublisherInfo: *mut SPC_LINK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SPC_SP_OPUS_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SPC_SP_OPUS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for SPC_SP_OPUS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_SP_OPUS_INFO").field("pwszProgramName", &self.pwszProgramName).field("pMoreInfo", &self.pMoreInfo).field("pPublisherInfo", &self.pPublisherInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SPC_SP_OPUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszProgramName == other.pwszProgramName && self.pMoreInfo == other.pMoreInfo && self.pPublisherInfo == other.pPublisherInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SPC_SP_OPUS_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SPC_SP_OPUS_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct SPC_STATEMENT_TYPE {
    pub cKeyPurposeId: u32,
    pub rgpszKeyPurposeId: *mut super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SPC_STATEMENT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SPC_STATEMENT_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SPC_STATEMENT_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_STATEMENT_TYPE").field("cKeyPurposeId", &self.cKeyPurposeId).field("rgpszKeyPurposeId", &self.rgpszKeyPurposeId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SPC_STATEMENT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cKeyPurposeId == other.cKeyPurposeId && self.rgpszKeyPurposeId == other.rgpszKeyPurposeId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SPC_STATEMENT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SPC_STATEMENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const SPC_UUID_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_MAX_STEPS: u32 = 38u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_CATALOGFILE: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_CERTSTORE: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FILEIO: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FINAL_CERTCHKPROV: u32 = 35u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FINAL_CERTPROV: u32 = 34u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FINAL_INITPROV: u32 = 31u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FINAL_OBJPROV: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FINAL_POLICYPROV: u32 = 36u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FINAL_SIGPROV: u32 = 33u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FINAL_UIPROV: u32 = 37u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_FINAL_WVTINIT: u32 = 30u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_CERTCHAIN: u32 = 15u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_COUNTERSIGCERT: u32 = 17u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_COUNTERSIGINFO: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_INNERCNT: u32 = 11u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_INNERCNTTYPE: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_SIGNERCERT: u32 = 14u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_SIGNERCOUNT: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_SIGNERINFO: u32 = 13u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_MSG_STORE: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_SIP: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_SIPSUBJINFO: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_VERIFY_MSGHASH: u32 = 18u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_VERIFY_MSGINDIRECTDATA: u32 = 19u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const TRUSTERROR_STEP_WVTPARAMS: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct WINTRUST_BLOB_INFO {
    pub cbStruct: u32,
    pub gSubject: ::windows::runtime::GUID,
    pub pcwszDisplayName: super::super::Foundation::PWSTR,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WINTRUST_BLOB_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINTRUST_BLOB_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINTRUST_BLOB_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_BLOB_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("gSubject", &self.gSubject)
            .field("pcwszDisplayName", &self.pcwszDisplayName)
            .field("cbMemObject", &self.cbMemObject)
            .field("pbMemObject", &self.pbMemObject)
            .field("cbMemSignedMsg", &self.cbMemSignedMsg)
            .field("pbMemSignedMsg", &self.pbMemSignedMsg)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINTRUST_BLOB_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.gSubject == other.gSubject && self.pcwszDisplayName == other.pcwszDisplayName && self.cbMemObject == other.cbMemObject && self.pbMemObject == other.pbMemObject && self.cbMemSignedMsg == other.cbMemSignedMsg && self.pbMemSignedMsg == other.pbMemSignedMsg
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINTRUST_BLOB_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINTRUST_BLOB_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WINTRUST_CATALOG_INFO {
    pub cbStruct: u32,
    pub dwCatalogVersion: u32,
    pub pcwszCatalogFilePath: super::super::Foundation::PWSTR,
    pub pcwszMemberTag: super::super::Foundation::PWSTR,
    pub pcwszMemberFilePath: super::super::Foundation::PWSTR,
    pub hMemberFile: super::super::Foundation::HANDLE,
    pub pbCalculatedFileHash: *mut u8,
    pub cbCalculatedFileHash: u32,
    pub pcCatalogContext: *mut super::Cryptography::CTL_CONTEXT,
    pub hCatAdmin: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WINTRUST_CATALOG_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WINTRUST_CATALOG_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for WINTRUST_CATALOG_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_CATALOG_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("dwCatalogVersion", &self.dwCatalogVersion)
            .field("pcwszCatalogFilePath", &self.pcwszCatalogFilePath)
            .field("pcwszMemberTag", &self.pcwszMemberTag)
            .field("pcwszMemberFilePath", &self.pcwszMemberFilePath)
            .field("hMemberFile", &self.hMemberFile)
            .field("pbCalculatedFileHash", &self.pbCalculatedFileHash)
            .field("cbCalculatedFileHash", &self.cbCalculatedFileHash)
            .field("pcCatalogContext", &self.pcCatalogContext)
            .field("hCatAdmin", &self.hCatAdmin)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WINTRUST_CATALOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwCatalogVersion == other.dwCatalogVersion && self.pcwszCatalogFilePath == other.pcwszCatalogFilePath && self.pcwszMemberTag == other.pcwszMemberTag && self.pcwszMemberFilePath == other.pcwszMemberFilePath && self.hMemberFile == other.hMemberFile && self.pbCalculatedFileHash == other.pbCalculatedFileHash && self.cbCalculatedFileHash == other.cbCalculatedFileHash && self.pcCatalogContext == other.pcCatalogContext && self.hCatAdmin == other.hCatAdmin
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WINTRUST_CATALOG_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WINTRUST_CATALOG_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WINTRUST_CERT_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: super::super::Foundation::PWSTR,
    pub psCertContext: *mut super::Cryptography::CERT_CONTEXT,
    pub chStores: u32,
    pub pahStores: *mut *mut ::std::ffi::c_void,
    pub dwFlags: u32,
    pub psftVerifyAsOf: *mut super::super::Foundation::FILETIME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WINTRUST_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WINTRUST_CERT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for WINTRUST_CERT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_CERT_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("pcwszDisplayName", &self.pcwszDisplayName)
            .field("psCertContext", &self.psCertContext)
            .field("chStores", &self.chStores)
            .field("pahStores", &self.pahStores)
            .field("dwFlags", &self.dwFlags)
            .field("psftVerifyAsOf", &self.psftVerifyAsOf)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WINTRUST_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pcwszDisplayName == other.pcwszDisplayName && self.psCertContext == other.psCertContext && self.chStores == other.chStores && self.pahStores == other.pahStores && self.dwFlags == other.dwFlags && self.psftVerifyAsOf == other.psftVerifyAsOf
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WINTRUST_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WINTRUST_CERT_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WINTRUST_DATA {
    pub cbStruct: u32,
    pub pPolicyCallbackData: *mut ::std::ffi::c_void,
    pub pSIPClientData: *mut ::std::ffi::c_void,
    pub dwUIChoice: WINTRUST_DATA_UICHOICE,
    pub fdwRevocationChecks: WINTRUST_DATA_REVOCATION_CHECKS,
    pub dwUnionChoice: WINTRUST_DATA_UNION_CHOICE,
    pub Anonymous: WINTRUST_DATA_0,
    pub dwStateAction: WINTRUST_DATA_STATE_ACTION,
    pub hWVTStateData: super::super::Foundation::HANDLE,
    pub pwszURLReference: super::super::Foundation::PWSTR,
    pub dwProvFlags: u32,
    pub dwUIContext: WINTRUST_DATA_UICONTEXT,
    pub pSignatureSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WINTRUST_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WINTRUST_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WINTRUST_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WINTRUST_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub union WINTRUST_DATA_0 {
    pub pFile: *mut WINTRUST_FILE_INFO,
    pub pCatalog: *mut WINTRUST_CATALOG_INFO,
    pub pBlob: *mut WINTRUST_BLOB_INFO,
    pub pSgnr: *mut WINTRUST_SGNR_INFO,
    pub pCert: *mut WINTRUST_CERT_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WINTRUST_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WINTRUST_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WINTRUST_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WINTRUST_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINTRUST_DATA_REVOCATION_CHECKS(pub u32);
pub const WTD_REVOKE_NONE: WINTRUST_DATA_REVOCATION_CHECKS = WINTRUST_DATA_REVOCATION_CHECKS(0u32);
pub const WTD_REVOKE_WHOLECHAIN: WINTRUST_DATA_REVOCATION_CHECKS = WINTRUST_DATA_REVOCATION_CHECKS(1u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_REVOCATION_CHECKS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_REVOCATION_CHECKS {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_REVOCATION_CHECKS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_REVOCATION_CHECKS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_REVOCATION_CHECKS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_REVOCATION_CHECKS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_REVOCATION_CHECKS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINTRUST_DATA_STATE_ACTION(pub u32);
pub const WTD_STATEACTION_IGNORE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(0u32);
pub const WTD_STATEACTION_VERIFY: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(1u32);
pub const WTD_STATEACTION_CLOSE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(2u32);
pub const WTD_STATEACTION_AUTO_CACHE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(3u32);
pub const WTD_STATEACTION_AUTO_CACHE_FLUSH: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(4u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_STATE_ACTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_STATE_ACTION {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_STATE_ACTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_STATE_ACTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_STATE_ACTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_STATE_ACTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_STATE_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINTRUST_DATA_UICHOICE(pub u32);
pub const WTD_UI_ALL: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(1u32);
pub const WTD_UI_NONE: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(2u32);
pub const WTD_UI_NOBAD: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(3u32);
pub const WTD_UI_NOGOOD: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(4u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_UICHOICE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_UICHOICE {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_UICHOICE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_UICHOICE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_UICHOICE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_UICHOICE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_UICHOICE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINTRUST_DATA_UICONTEXT(pub u32);
pub const WTD_UICONTEXT_EXECUTE: WINTRUST_DATA_UICONTEXT = WINTRUST_DATA_UICONTEXT(0u32);
pub const WTD_UICONTEXT_INSTALL: WINTRUST_DATA_UICONTEXT = WINTRUST_DATA_UICONTEXT(1u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_UICONTEXT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_UICONTEXT {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_UICONTEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_UICONTEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_UICONTEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_UICONTEXT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_UICONTEXT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINTRUST_DATA_UNION_CHOICE(pub u32);
pub const WTD_CHOICE_FILE: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(1u32);
pub const WTD_CHOICE_CATALOG: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(2u32);
pub const WTD_CHOICE_BLOB: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(3u32);
pub const WTD_CHOICE_SIGNER: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(4u32);
pub const WTD_CHOICE_CERT: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(5u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_UNION_CHOICE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_UNION_CHOICE {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_UNION_CHOICE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_UNION_CHOICE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_UNION_CHOICE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_UNION_CHOICE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_UNION_CHOICE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct WINTRUST_FILE_INFO {
    pub cbStruct: u32,
    pub pcwszFilePath: super::super::Foundation::PWSTR,
    pub hFile: super::super::Foundation::HANDLE,
    pub pgKnownSubject: *mut ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WINTRUST_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINTRUST_FILE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINTRUST_FILE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_FILE_INFO").field("cbStruct", &self.cbStruct).field("pcwszFilePath", &self.pcwszFilePath).field("hFile", &self.hFile).field("pgKnownSubject", &self.pgKnownSubject).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINTRUST_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pcwszFilePath == other.pcwszFilePath && self.hFile == other.hFile && self.pgKnownSubject == other.pgKnownSubject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINTRUST_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINTRUST_FILE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(pub u32);
pub const DWACTION_ALLOCANDFILL: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(1u32);
pub const DWACTION_FREE: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(2u32);
impl ::std::convert::From<u32> for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WINTRUST_MAX_HASH_BYTES_TO_MAP_DEFAULT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WINTRUST_MAX_HEADER_BYTES_TO_MAP_DEFAULT: u32 = 10485760u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for WINTRUST_POLICY_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_POLICY_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_POLICY_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_POLICY_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WINTRUST_SGNR_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: super::super::Foundation::PWSTR,
    pub psSignerInfo: *mut super::Cryptography::CMSG_SIGNER_INFO,
    pub chStores: u32,
    pub pahStores: *mut *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WINTRUST_SGNR_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WINTRUST_SGNR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for WINTRUST_SGNR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_SGNR_INFO").field("cbStruct", &self.cbStruct).field("pcwszDisplayName", &self.pcwszDisplayName).field("psSignerInfo", &self.psSignerInfo).field("chStores", &self.chStores).field("pahStores", &self.pahStores).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WINTRUST_SGNR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pcwszDisplayName == other.pcwszDisplayName && self.psSignerInfo == other.psSignerInfo && self.chStores == other.chStores && self.pahStores == other.pahStores
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WINTRUST_SGNR_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WINTRUST_SGNR_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WINTRUST_SIGNATURE_SETTINGS {
    pub cbStruct: u32,
    pub dwIndex: u32,
    pub dwFlags: WINTRUST_SIGNATURE_SETTINGS_FLAGS,
    pub cSecondarySigs: u32,
    pub dwVerifiedSigIndex: u32,
    pub pCryptoPolicy: *mut super::Cryptography::CERT_STRONG_SIGN_PARA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WINTRUST_SIGNATURE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WINTRUST_SIGNATURE_SETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for WINTRUST_SIGNATURE_SETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_SIGNATURE_SETTINGS").field("cbStruct", &self.cbStruct).field("dwIndex", &self.dwIndex).field("dwFlags", &self.dwFlags).field("cSecondarySigs", &self.cSecondarySigs).field("dwVerifiedSigIndex", &self.dwVerifiedSigIndex).field("pCryptoPolicy", &self.pCryptoPolicy).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WINTRUST_SIGNATURE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwIndex == other.dwIndex && self.dwFlags == other.dwFlags && self.cSecondarySigs == other.cSecondarySigs && self.dwVerifiedSigIndex == other.dwVerifiedSigIndex && self.pCryptoPolicy == other.pCryptoPolicy
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WINTRUST_SIGNATURE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WINTRUST_SIGNATURE_SETTINGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINTRUST_SIGNATURE_SETTINGS_FLAGS(pub u32);
pub const WSS_VERIFY_SPECIFIC: WINTRUST_SIGNATURE_SETTINGS_FLAGS = WINTRUST_SIGNATURE_SETTINGS_FLAGS(1u32);
pub const WSS_GET_SECONDARY_SIG_COUNT: WINTRUST_SIGNATURE_SETTINGS_FLAGS = WINTRUST_SIGNATURE_SETTINGS_FLAGS(2u32);
impl ::std::convert::From<u32> for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub struct WIN_CERTIFICATE {
    pub dwLength: u32,
    pub wRevision: u16,
    pub wCertificateType: u16,
    pub bCertificate: [u8; 1],
}
impl WIN_CERTIFICATE {}
impl ::std::default::Default for WIN_CERTIFICATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN_CERTIFICATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_CERTIFICATE").field("dwLength", &self.dwLength).field("wRevision", &self.wRevision).field("wCertificateType", &self.wCertificateType).field("bCertificate", &self.bCertificate).finish()
    }
}
impl ::std::cmp::PartialEq for WIN_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.wRevision == other.wRevision && self.wCertificateType == other.wCertificateType && self.bCertificate == other.bCertificate
    }
}
impl ::std::cmp::Eq for WIN_CERTIFICATE {}
unsafe impl ::windows::runtime::Abi for WIN_CERTIFICATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WIN_CERT_REVISION_1_0: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WIN_CERT_REVISION_2_0: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WIN_CERT_TYPE_RESERVED_1: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WIN_CERT_TYPE_TS_STACK_SIGNED: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WIN_CERT_TYPE_X509: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    pub hClientToken: super::super::Foundation::HANDLE,
    pub lpCertificate: *mut WIN_CERTIFICATE,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN_SPUB_TRUSTED_PUBLISHER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_SPUB_TRUSTED_PUBLISHER_DATA").field("hClientToken", &self.hClientToken).field("lpCertificate", &self.lpCertificate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.hClientToken == other.hClientToken && self.lpCertificate == other.lpCertificate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN_SPUB_TRUSTED_PUBLISHER_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    pub hClientToken: super::super::Foundation::HANDLE,
    pub SubjectType: *mut ::windows::runtime::GUID,
    pub Subject: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT").field("hClientToken", &self.hClientToken).field("SubjectType", &self.SubjectType).field("Subject", &self.Subject).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.hClientToken == other.hClientToken && self.SubjectType == other.SubjectType && self.Subject == other.Subject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub struct WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    pub SubjectType: *mut ::windows::runtime::GUID,
    pub Subject: *mut ::std::ffi::c_void,
}
impl WIN_TRUST_ACTDATA_SUBJECT_ONLY {}
impl ::std::default::Default for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_TRUST_ACTDATA_SUBJECT_ONLY").field("SubjectType", &self.SubjectType).field("Subject", &self.Subject).finish()
    }
}
impl ::std::cmp::PartialEq for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectType == other.SubjectType && self.Subject == other.Subject
    }
}
impl ::std::cmp::Eq for WIN_TRUST_ACTDATA_SUBJECT_ONLY {}
unsafe impl ::windows::runtime::Abi for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct WIN_TRUST_SUBJECT_FILE {
    pub hFile: super::super::Foundation::HANDLE,
    pub lpPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN_TRUST_SUBJECT_FILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN_TRUST_SUBJECT_FILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN_TRUST_SUBJECT_FILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_TRUST_SUBJECT_FILE").field("hFile", &self.hFile).field("lpPath", &self.lpPath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN_TRUST_SUBJECT_FILE {
    fn eq(&self, other: &Self) -> bool {
        self.hFile == other.hFile && self.lpPath == other.lpPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN_TRUST_SUBJECT_FILE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN_TRUST_SUBJECT_FILE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
pub struct WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    pub hFile: super::super::Foundation::HANDLE,
    pub lpPath: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_TRUST_SUBJECT_FILE_AND_DISPLAY").field("hFile", &self.hFile).field("lpPath", &self.lpPath).field("lpDisplayName", &self.lpDisplayName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.hFile == other.hFile && self.lpPath == other.lpPath && self.lpDisplayName == other.lpDisplayName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_CERTTRUST_SUPPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_INPUT_FLAG_MASK: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_OBJTRUST_SUPPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_OUTPUT_FLAG_MASK: u32 = 3758096384u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_OUT_FILE_SUPPORTS_SEAL: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_OUT_HAS_SEALING_INTENT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_OUT_SEALING_STATUS_VERIFIED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_SIGTRUST_SUPPORT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WSS_VERIFY_SEALING: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0,
    pub hChainEngine: super::Cryptography::HCERTCHAINENGINE,
    pub pChainPara: *mut super::Cryptography::CERT_CHAIN_PARA,
    pub dwFlags: u32,
    pub pvReserved: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub union WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {}
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::clone::Clone for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
pub struct WTD_GENERIC_CHAIN_POLICY_DATA {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_DATA_0,
    pub pSignerChainInfo: *mut WTD_GENERIC_CHAIN_POLICY_CREATE_INFO,
    pub pCounterSignerChainInfo: *mut WTD_GENERIC_CHAIN_POLICY_CREATE_INFO,
    pub pfnPolicyCallback: ::std::option::Option<PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK>,
    pub pvPolicyArg: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl WTD_GENERIC_CHAIN_POLICY_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_DATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub union WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl WTD_GENERIC_CHAIN_POLICY_DATA_0 {}
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_DATA_0 {}
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
pub struct WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0,
    pub pChainContext: *mut super::Cryptography::CERT_CHAIN_CONTEXT,
    pub dwSignerType: u32,
    pub pMsgSignerInfo: *mut super::Cryptography::CMSG_SIGNER_INFO,
    pub dwError: u32,
    pub cCounterSigner: u32,
    pub rgpCounterSigner: *mut *mut WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub union WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {}
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
#[inline]
pub unsafe fn WTHelperCertCheckValidSignature(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperCertCheckValidSignature(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
        }
        WTHelperCertCheckValidSignature(::std::mem::transmute(pprovdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
#[inline]
pub unsafe fn WTHelperCertIsSelfSigned(dwencoding: u32, pcert: *mut super::Cryptography::CERT_INFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperCertIsSelfSigned(dwencoding: u32, pcert: *mut super::Cryptography::CERT_INFO) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WTHelperCertIsSelfSigned(::std::mem::transmute(dwencoding), ::std::mem::transmute(pcert)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
#[inline]
pub unsafe fn WTHelperGetProvCertFromChain(psgnr: *mut CRYPT_PROVIDER_SGNR, idxcert: u32) -> *mut CRYPT_PROVIDER_CERT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvCertFromChain(psgnr: *mut CRYPT_PROVIDER_SGNR, idxcert: u32) -> *mut CRYPT_PROVIDER_CERT;
        }
        ::std::mem::transmute(WTHelperGetProvCertFromChain(::std::mem::transmute(psgnr), ::std::mem::transmute(idxcert)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
#[inline]
pub unsafe fn WTHelperGetProvPrivateDataFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, pgproviderid: *mut ::windows::runtime::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvPrivateDataFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, pgproviderid: *mut ::windows::runtime::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA;
        }
        ::std::mem::transmute(WTHelperGetProvPrivateDataFromChain(::std::mem::transmute(pprovdata), ::std::mem::transmute(pgproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
#[inline]
pub unsafe fn WTHelperGetProvSignerFromChain<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pprovdata: *mut CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: Param2, idxcountersigner: u32) -> *mut CRYPT_PROVIDER_SGNR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvSignerFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: super::super::Foundation::BOOL, idxcountersigner: u32) -> *mut CRYPT_PROVIDER_SGNR;
        }
        ::std::mem::transmute(WTHelperGetProvSignerFromChain(::std::mem::transmute(pprovdata), ::std::mem::transmute(idxsigner), fcountersigner.into_param().abi(), ::std::mem::transmute(idxcountersigner)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
#[inline]
pub unsafe fn WTHelperProvDataFromStateData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hstatedata: Param0) -> *mut CRYPT_PROVIDER_DATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperProvDataFromStateData(hstatedata: super::super::Foundation::HANDLE) -> *mut CRYPT_PROVIDER_DATA;
        }
        ::std::mem::transmute(WTHelperProvDataFromStateData(hstatedata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WT_ADD_ACTION_ID_RET_RESULT_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WT_CURRENT_VERSION: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WT_TRUSTDBDIALOG_NO_UI_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WT_TRUSTDBDIALOG_ONLY_PUB_TAB_FLAG: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WT_TRUSTDBDIALOG_WRITE_IEAK_STORE_FLAG: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
pub const WT_TRUSTDBDIALOG_WRITE_LEGACY_REG_FLAG: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WinVerifyTrust<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pgactionid: *mut ::windows::runtime::GUID, pwvtdata: *mut ::std::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinVerifyTrust(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows::runtime::GUID, pwvtdata: *mut ::std::ffi::c_void) -> i32;
        }
        ::std::mem::transmute(WinVerifyTrust(hwnd.into_param().abi(), ::std::mem::transmute(pgactionid), ::std::mem::transmute(pwvtdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
#[inline]
pub unsafe fn WinVerifyTrustEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pgactionid: *mut ::windows::runtime::GUID, pwintrustdata: *mut WINTRUST_DATA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinVerifyTrustEx(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows::runtime::GUID, pwintrustdata: *mut WINTRUST_DATA) -> i32;
        }
        ::std::mem::transmute(WinVerifyTrustEx(hwnd.into_param().abi(), ::std::mem::transmute(pgactionid), ::std::mem::transmute(pwintrustdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WintrustAddActionID(pgactionid: *const ::windows::runtime::GUID, fdwflags: u32, psprovinfo: *const CRYPT_REGISTER_ACTIONID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustAddActionID(pgactionid: *const ::windows::runtime::GUID, fdwflags: u32, psprovinfo: *const CRYPT_REGISTER_ACTIONID) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustAddActionID(::std::mem::transmute(pgactionid), ::std::mem::transmute(fdwflags), ::std::mem::transmute(psprovinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WintrustAddDefaultForUsage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszusageoid: Param0, psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustAddDefaultForUsage(pszusageoid: super::super::Foundation::PSTR, psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustAddDefaultForUsage(pszusageoid.into_param().abi(), ::std::mem::transmute(psdefusage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WintrustGetDefaultForUsage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION, pszusageoid: Param1, psusage: *mut CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustGetDefaultForUsage(dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION, pszusageoid: super::super::Foundation::PSTR, psusage: *mut CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustGetDefaultForUsage(::std::mem::transmute(dwaction), pszusageoid.into_param().abi(), ::std::mem::transmute(psusage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_WinTrust`*"]
#[inline]
pub unsafe fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS);
        }
        ::std::mem::transmute(WintrustGetRegPolicyFlags(::std::mem::transmute(pdwpolicyflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`, `Win32_Security_Cryptography`, `Win32_Security_Cryptography_Catalog`, `Win32_Security_Cryptography_Sip`*"]
#[inline]
pub unsafe fn WintrustLoadFunctionPointers(pgactionid: *mut ::windows::runtime::GUID, ppfns: *mut CRYPT_PROVIDER_FUNCTIONS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustLoadFunctionPointers(pgactionid: *mut ::windows::runtime::GUID, ppfns: *mut ::std::mem::ManuallyDrop<CRYPT_PROVIDER_FUNCTIONS>) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustLoadFunctionPointers(::std::mem::transmute(pgactionid), ::std::mem::transmute(ppfns)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WintrustRemoveActionID(pgactionid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustRemoveActionID(pgactionid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustRemoveActionID(::std::mem::transmute(pgactionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WintrustSetDefaultIncludePEPageHashes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(fincludepepagehashes: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes: super::super::Foundation::BOOL);
        }
        ::std::mem::transmute(WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_WinTrust`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WintrustSetRegPolicyFlags(dwpolicyflags: WINTRUST_POLICY_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustSetRegPolicyFlags(dwpolicyflags: WINTRUST_POLICY_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustSetRegPolicyFlags(::std::mem::transmute(dwpolicyflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
