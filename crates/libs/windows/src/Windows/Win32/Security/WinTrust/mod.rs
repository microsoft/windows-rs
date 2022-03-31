#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct CAT_MEMBERINFO {
    pub pwszSubjGuid: ::windows::core::PWSTR,
    pub dwCertVersion: u32,
}
impl ::core::marker::Copy for CAT_MEMBERINFO {}
impl ::core::clone::Clone for CAT_MEMBERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAT_MEMBERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAT_MEMBERINFO").field("pwszSubjGuid", &self.pwszSubjGuid).field("dwCertVersion", &self.dwCertVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for CAT_MEMBERINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAT_MEMBERINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAT_MEMBERINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAT_MEMBERINFO {}
impl ::core::default::Default for CAT_MEMBERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct CAT_MEMBERINFO2 {
    pub SubjectGuid: ::windows::core::GUID,
    pub dwCertVersion: u32,
}
impl ::core::marker::Copy for CAT_MEMBERINFO2 {}
impl ::core::clone::Clone for CAT_MEMBERINFO2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAT_MEMBERINFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAT_MEMBERINFO2").field("SubjectGuid", &self.SubjectGuid).field("dwCertVersion", &self.dwCertVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for CAT_MEMBERINFO2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAT_MEMBERINFO2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAT_MEMBERINFO2>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAT_MEMBERINFO2 {}
impl ::core::default::Default for CAT_MEMBERINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CAT_MEMBERINFO2_OBJID: &'static str = "1.3.6.1.4.1.311.12.2.3";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CAT_MEMBERINFO2_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2223i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CAT_MEMBERINFO_OBJID: &'static str = "1.3.6.1.4.1.311.12.2.2";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CAT_MEMBERINFO_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2222i32 as _);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct CAT_NAMEVALUE {
    pub pwszTag: ::windows::core::PWSTR,
    pub fdwFlags: u32,
    pub Value: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for CAT_NAMEVALUE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for CAT_NAMEVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for CAT_NAMEVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAT_NAMEVALUE").field("pwszTag", &self.pwszTag).field("fdwFlags", &self.fdwFlags).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for CAT_NAMEVALUE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for CAT_NAMEVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAT_NAMEVALUE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for CAT_NAMEVALUE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for CAT_NAMEVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CAT_NAMEVALUE_OBJID: &'static str = "1.3.6.1.4.1.311.12.2.1";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CAT_NAMEVALUE_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2221i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CCPI_RESULT_ALLOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CCPI_RESULT_AUDIT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CCPI_RESULT_DENY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CERT_CONFIDENCE_AUTHIDEXT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CERT_CONFIDENCE_HIGHEST: u32 = 286330880u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CERT_CONFIDENCE_HYGIENE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CERT_CONFIDENCE_SIG: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CERT_CONFIDENCE_TIME: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CERT_CONFIDENCE_TIMENEST: u32 = 1048576u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CONFIG_CI_PROV_INFO {
    pub cbSize: u32,
    pub dwPolicies: u32,
    pub pPolicies: *mut super::Cryptography::CRYPTOAPI_BLOB,
    pub result: CONFIG_CI_PROV_INFO_RESULT,
    pub dwScenario: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for CONFIG_CI_PROV_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CONFIG_CI_PROV_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CONFIG_CI_PROV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIG_CI_PROV_INFO").field("cbSize", &self.cbSize).field("dwPolicies", &self.dwPolicies).field("pPolicies", &self.pPolicies).field("result", &self.result).field("dwScenario", &self.dwScenario).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for CONFIG_CI_PROV_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CONFIG_CI_PROV_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONFIG_CI_PROV_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CONFIG_CI_PROV_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CONFIG_CI_PROV_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFIG_CI_PROV_INFO_RESULT {
    pub hr: ::windows::core::HRESULT,
    pub dwResult: u32,
    pub dwPolicyIndex: u32,
    pub fIsExplicitDeny: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONFIG_CI_PROV_INFO_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONFIG_CI_PROV_INFO_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONFIG_CI_PROV_INFO_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIG_CI_PROV_INFO_RESULT").field("hr", &self.hr).field("dwResult", &self.dwResult).field("dwPolicyIndex", &self.dwPolicyIndex).field("fIsExplicitDeny", &self.fIsExplicitDeny).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONFIG_CI_PROV_INFO_RESULT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONFIG_CI_PROV_INFO_RESULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONFIG_CI_PROV_INFO_RESULT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONFIG_CI_PROV_INFO_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONFIG_CI_PROV_INFO_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_CHOICE_SIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_RETURN_LOWER_QUALITY_CHAINS: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_REVOCATION_CHECK_CHAIN: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_REVOCATION_CHECK_END_CERT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_REVOCATION_CHECK_NONE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_UISTATE_MODE_ALLOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_UISTATE_MODE_BLOCK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_UISTATE_MODE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_UISTATE_MODE_PROMPT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const CPD_USE_NT5_CHAIN_FLAG: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CRYPT_PROVIDER_CERT {
    pub cbStruct: u32,
    pub pCert: *const super::Cryptography::CERT_CONTEXT,
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
impl ::core::marker::Copy for CRYPT_PROVIDER_CERT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_CERT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CRYPT_PROVIDER_CERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_CERT")
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
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_CERT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_CERT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_CERT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_CERT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CRYPT_PROVIDER_CERT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct CRYPT_PROVIDER_DATA {
    pub cbStruct: u32,
    pub pWintrustData: *mut WINTRUST_DATA,
    pub fOpenedFile: super::super::Foundation::BOOL,
    pub hWndParent: super::super::Foundation::HWND,
    pub pgActionID: *mut ::windows::core::GUID,
    pub hProv: usize,
    pub dwError: u32,
    pub dwRegSecuritySettings: u32,
    pub dwRegPolicySettings: u32,
    pub psPfns: *mut CRYPT_PROVIDER_FUNCTIONS,
    pub cdwTrustStepErrors: u32,
    pub padwTrustStepErrors: *mut u32,
    pub chStores: u32,
    pub pahStores: *mut super::Cryptography::HCERTSTORE,
    pub dwEncoding: u32,
    pub hMsg: *mut ::core::ffi::c_void,
    pub csSigners: u32,
    pub pasSigners: *mut CRYPT_PROVIDER_SGNR,
    pub csProvPrivData: u32,
    pub pasProvPrivData: *mut CRYPT_PROVIDER_PRIVDATA,
    pub dwSubjectChoice: u32,
    pub Anonymous: CRYPT_PROVIDER_DATA_0,
    pub pszUsageOID: ::windows::core::PSTR,
    pub fRecallWithState: super::super::Foundation::BOOL,
    pub sftSystemTime: super::super::Foundation::FILETIME,
    pub pszCTLSignerUsageOID: ::windows::core::PSTR,
    pub dwProvFlags: u32,
    pub dwFinalError: u32,
    pub pRequestUsage: *mut super::Cryptography::CERT_USAGE_MATCH,
    pub dwTrustPubSettings: u32,
    pub dwUIStateFlags: u32,
    pub pSigState: *mut CRYPT_PROVIDER_SIGSTATE,
    pub pSigSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPT_PROVIDER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub union CRYPT_PROVIDER_DATA_0 {
    pub pPDSip: *mut PROVDATA_SIP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_DATA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_DATA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPT_PROVIDER_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct CRYPT_PROVIDER_DEFUSAGE {
    pub cbStruct: u32,
    pub gActionID: ::windows::core::GUID,
    pub pDefPolicyCallbackData: *mut ::core::ffi::c_void,
    pub pDefSIPClientData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_PROVIDER_DEFUSAGE {}
impl ::core::clone::Clone for CRYPT_PROVIDER_DEFUSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPT_PROVIDER_DEFUSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_DEFUSAGE").field("cbStruct", &self.cbStruct).field("gActionID", &self.gActionID).field("pDefPolicyCallbackData", &self.pDefPolicyCallbackData).field("pDefSIPClientData", &self.pDefSIPClientData).finish()
    }
}
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_DEFUSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_DEFUSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_DEFUSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_DEFUSAGE {}
impl ::core::default::Default for CRYPT_PROVIDER_DEFUSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::fmt::Debug for CRYPT_PROVIDER_FUNCTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_FUNCTIONS")
            .field("cbStruct", &self.cbStruct)
            .field("pfnAlloc", &self.pfnAlloc.map(|f| f as usize))
            .field("pfnFree", &self.pfnFree.map(|f| f as usize))
            .field("pfnAddStore2Chain", &self.pfnAddStore2Chain.map(|f| f as usize))
            .field("pfnAddSgnr2Chain", &self.pfnAddSgnr2Chain.map(|f| f as usize))
            .field("pfnAddCert2Chain", &self.pfnAddCert2Chain.map(|f| f as usize))
            .field("pfnAddPrivData2Chain", &self.pfnAddPrivData2Chain.map(|f| f as usize))
            .field("pfnInitialize", &self.pfnInitialize.map(|f| f as usize))
            .field("pfnObjectTrust", &self.pfnObjectTrust.map(|f| f as usize))
            .field("pfnSignatureTrust", &self.pfnSignatureTrust.map(|f| f as usize))
            .field("pfnCertificateTrust", &self.pfnCertificateTrust.map(|f| f as usize))
            .field("pfnFinalPolicy", &self.pfnFinalPolicy.map(|f| f as usize))
            .field("pfnCertCheckPolicy", &self.pfnCertCheckPolicy.map(|f| f as usize))
            .field("pfnTestFinalPolicy", &self.pfnTestFinalPolicy.map(|f| f as usize))
            .field("psUIpfns", &self.psUIpfns)
            .field("pfnCleanupPolicy", &self.pfnCleanupPolicy.map(|f| f as usize))
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_FUNCTIONS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_FUNCTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_FUNCTIONS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPT_PROVIDER_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct CRYPT_PROVIDER_PRIVDATA {
    pub cbStruct: u32,
    pub gProviderID: ::windows::core::GUID,
    pub cbProvData: u32,
    pub pvProvData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_PROVIDER_PRIVDATA {}
impl ::core::clone::Clone for CRYPT_PROVIDER_PRIVDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPT_PROVIDER_PRIVDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_PRIVDATA").field("cbStruct", &self.cbStruct).field("gProviderID", &self.gProviderID).field("cbProvData", &self.cbProvData).field("pvProvData", &self.pvProvData).finish()
    }
}
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_PRIVDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_PRIVDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_PRIVDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_PRIVDATA {}
impl ::core::default::Default for CRYPT_PROVIDER_PRIVDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct CRYPT_PROVIDER_REGDEFUSAGE {
    pub cbStruct: u32,
    pub pgActionID: *mut ::windows::core::GUID,
    pub pwszDllName: ::windows::core::PWSTR,
    pub pwszLoadCallbackDataFunctionName: ::windows::core::PSTR,
    pub pwszFreeCallbackDataFunctionName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for CRYPT_PROVIDER_REGDEFUSAGE {}
impl ::core::clone::Clone for CRYPT_PROVIDER_REGDEFUSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPT_PROVIDER_REGDEFUSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_REGDEFUSAGE").field("cbStruct", &self.cbStruct).field("pgActionID", &self.pgActionID).field("pwszDllName", &self.pwszDllName).field("pwszLoadCallbackDataFunctionName", &self.pwszLoadCallbackDataFunctionName).field("pwszFreeCallbackDataFunctionName", &self.pwszFreeCallbackDataFunctionName).finish()
    }
}
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_REGDEFUSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_REGDEFUSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_REGDEFUSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPT_PROVIDER_REGDEFUSAGE {}
impl ::core::default::Default for CRYPT_PROVIDER_REGDEFUSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
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
impl ::core::marker::Copy for CRYPT_PROVIDER_SGNR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_SGNR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CRYPT_PROVIDER_SGNR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_SGNR").field("cbStruct", &self.cbStruct).field("sftVerifyAsOf", &self.sftVerifyAsOf).field("csCertChain", &self.csCertChain).field("pasCertChain", &self.pasCertChain).field("dwSignerType", &self.dwSignerType).field("psSigner", &self.psSigner).field("dwError", &self.dwError).field("csCounterSigners", &self.csCounterSigners).field("pasCounterSigners", &self.pasCounterSigners).field("pChainContext", &self.pChainContext).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_SGNR {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_SGNR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_SGNR>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_SGNR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CRYPT_PROVIDER_SGNR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CRYPT_PROVIDER_SIGSTATE {
    pub cbStruct: u32,
    pub rhSecondarySigs: *mut *mut ::core::ffi::c_void,
    pub hPrimarySig: *mut ::core::ffi::c_void,
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
impl ::core::marker::Copy for CRYPT_PROVIDER_SIGSTATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_SIGSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CRYPT_PROVIDER_SIGSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVIDER_SIGSTATE")
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
unsafe impl ::windows::core::Abi for CRYPT_PROVIDER_SIGSTATE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CRYPT_PROVIDER_SIGSTATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVIDER_SIGSTATE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CRYPT_PROVIDER_SIGSTATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CRYPT_PROVIDER_SIGSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct CRYPT_PROVUI_DATA {
    pub cbStruct: u32,
    pub dwFinalError: u32,
    pub pYesButtonText: ::windows::core::PWSTR,
    pub pNoButtonText: ::windows::core::PWSTR,
    pub pMoreInfoButtonText: ::windows::core::PWSTR,
    pub pAdvancedLinkText: ::windows::core::PWSTR,
    pub pCopyActionText: ::windows::core::PWSTR,
    pub pCopyActionTextNoTS: ::windows::core::PWSTR,
    pub pCopyActionTextNotSigned: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_PROVUI_DATA {}
impl ::core::clone::Clone for CRYPT_PROVUI_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPT_PROVUI_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVUI_DATA")
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
unsafe impl ::windows::core::Abi for CRYPT_PROVUI_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPT_PROVUI_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVUI_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPT_PROVUI_DATA {}
impl ::core::default::Default for CRYPT_PROVUI_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct CRYPT_PROVUI_FUNCS {
    pub cbStruct: u32,
    pub psUIData: *mut CRYPT_PROVUI_DATA,
    pub pfnOnMoreInfoClick: PFN_PROVUI_CALL,
    pub pfnOnMoreInfoClickDefault: PFN_PROVUI_CALL,
    pub pfnOnAdvancedClick: PFN_PROVUI_CALL,
    pub pfnOnAdvancedClickDefault: PFN_PROVUI_CALL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPT_PROVUI_FUNCS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPT_PROVUI_FUNCS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::fmt::Debug for CRYPT_PROVUI_FUNCS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_PROVUI_FUNCS").field("cbStruct", &self.cbStruct).field("psUIData", &self.psUIData).field("pfnOnMoreInfoClick", &self.pfnOnMoreInfoClick.map(|f| f as usize)).field("pfnOnMoreInfoClickDefault", &self.pfnOnMoreInfoClickDefault.map(|f| f as usize)).field("pfnOnAdvancedClick", &self.pfnOnAdvancedClick.map(|f| f as usize)).field("pfnOnAdvancedClickDefault", &self.pfnOnAdvancedClickDefault.map(|f| f as usize)).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::core::Abi for CRYPT_PROVUI_FUNCS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for CRYPT_PROVUI_FUNCS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_PROVUI_FUNCS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for CRYPT_PROVUI_FUNCS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for CRYPT_PROVUI_FUNCS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
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
impl ::core::marker::Copy for CRYPT_REGISTER_ACTIONID {}
impl ::core::clone::Clone for CRYPT_REGISTER_ACTIONID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPT_REGISTER_ACTIONID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_REGISTER_ACTIONID")
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
unsafe impl ::windows::core::Abi for CRYPT_REGISTER_ACTIONID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPT_REGISTER_ACTIONID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_REGISTER_ACTIONID>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPT_REGISTER_ACTIONID {}
impl ::core::default::Default for CRYPT_REGISTER_ACTIONID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct CRYPT_TRUST_REG_ENTRY {
    pub cbStruct: u32,
    pub pwszDLLName: ::windows::core::PWSTR,
    pub pwszFunctionName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CRYPT_TRUST_REG_ENTRY {}
impl ::core::clone::Clone for CRYPT_TRUST_REG_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPT_TRUST_REG_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPT_TRUST_REG_ENTRY").field("cbStruct", &self.cbStruct).field("pwszDLLName", &self.pwszDLLName).field("pwszFunctionName", &self.pwszFunctionName).finish()
    }
}
unsafe impl ::windows::core::Abi for CRYPT_TRUST_REG_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPT_TRUST_REG_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPT_TRUST_REG_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPT_TRUST_REG_ENTRY {}
impl ::core::default::Default for CRYPT_TRUST_REG_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const DRIVER_CLEANUPPOLICY_FUNCTION: &'static str = "DriverCleanupPolicy";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const DRIVER_FINALPOLPROV_FUNCTION: &'static str = "DriverFinalPolicy";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const DRIVER_INITPROV_FUNCTION: &'static str = "DriverInitializePolicy";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct DRIVER_VER_INFO {
    pub cbStruct: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub dwPlatform: u32,
    pub dwVersion: u32,
    pub wszVersion: [u16; 260],
    pub wszSignedBy: [u16; 260],
    pub pcSignerCertContext: *const super::Cryptography::CERT_CONTEXT,
    pub sOSVersionLow: DRIVER_VER_MAJORMINOR,
    pub sOSVersionHigh: DRIVER_VER_MAJORMINOR,
    pub dwBuildNumberLow: u32,
    pub dwBuildNumberHigh: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for DRIVER_VER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for DRIVER_VER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for DRIVER_VER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_VER_INFO")
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
unsafe impl ::windows::core::Abi for DRIVER_VER_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for DRIVER_VER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRIVER_VER_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for DRIVER_VER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for DRIVER_VER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct DRIVER_VER_MAJORMINOR {
    pub dwMajor: u32,
    pub dwMinor: u32,
}
impl ::core::marker::Copy for DRIVER_VER_MAJORMINOR {}
impl ::core::clone::Clone for DRIVER_VER_MAJORMINOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRIVER_VER_MAJORMINOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVER_VER_MAJORMINOR").field("dwMajor", &self.dwMajor).field("dwMinor", &self.dwMinor).finish()
    }
}
unsafe impl ::windows::core::Abi for DRIVER_VER_MAJORMINOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRIVER_VER_MAJORMINOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRIVER_VER_MAJORMINOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRIVER_VER_MAJORMINOR {}
impl ::core::default::Default for DRIVER_VER_MAJORMINOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const GENERIC_CHAIN_CERTTRUST_FUNCTION: &'static str = "GenericChainCertificateTrust";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const GENERIC_CHAIN_FINALPOLICY_FUNCTION: &'static str = "GenericChainFinalProv";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const HTTPS_CERTTRUST_FUNCTION: &'static str = "HTTPSCertificateTrust";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const HTTPS_CHKCERT_FUNCTION: &'static str = "HTTPSCheckCertProv";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const HTTPS_FINALPOLICY_FUNCTION: &'static str = "HTTPSFinalProv";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INTENT_TO_SEAL_ATTRIBUTE {
    pub version: u32,
    pub seal: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTENT_TO_SEAL_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTENT_TO_SEAL_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTENT_TO_SEAL_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTENT_TO_SEAL_ATTRIBUTE").field("version", &self.version).field("seal", &self.seal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTENT_TO_SEAL_ATTRIBUTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTENT_TO_SEAL_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTENT_TO_SEAL_ATTRIBUTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTENT_TO_SEAL_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTENT_TO_SEAL_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const INTENT_TO_SEAL_ATTRIBUTE_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2010i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const OFFICE_CLEANUPPOLICY_FUNCTION: &'static str = "OfficeCleanupPolicy";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const OFFICE_INITPROV_FUNCTION: &'static str = "OfficeInitializePolicy";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const OFFICE_POLICY_PROVIDER_DLL_NAME: &'static str = "WINTRUST.DLL";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenPersonalTrustDBDialog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPersonalTrustDBDialog(hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(OpenPersonalTrustDBDialog(hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenPersonalTrustDBDialogEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, dwflags: u32, pvreserved: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPersonalTrustDBDialogEx(hwndparent: super::super::Foundation::HWND, dwflags: u32, pvreserved: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(OpenPersonalTrustDBDialogEx(hwndparent.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_ALLOCANDFILLDEFUSAGE = ::core::option::Option<unsafe extern "system" fn(pszusageoid: ::windows::core::PCSTR, psdefusage: *const CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_CERT = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: super::super::Foundation::BOOL, idxcountersigner: u32, pcert2add: *const super::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_PRIVDATA = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, pprivdata2add: *const CRYPT_PROVIDER_PRIVDATA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_SGNR = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, fcountersigner: super::super::Foundation::BOOL, idxsigner: u32, psgnr2add: *const CRYPT_PROVIDER_SGNR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_STORE = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, hstore2add: super::Cryptography::HCERTSTORE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub type PFN_CPD_MEM_ALLOC = ::core::option::Option<unsafe extern "system" fn(cbsize: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub type PFN_CPD_MEM_FREE = ::core::option::Option<unsafe extern "system" fn(pvmem2free: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FREEDEFUSAGE = ::core::option::Option<unsafe extern "system" fn(pszusageoid: ::windows::core::PCSTR, psdefusage: *const CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CERTCHKPOLICY_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersignerchain: super::super::Foundation::BOOL, idxcountersigner: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CERTTRUST_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CLEANUP_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_FINALPOLICY_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_INIT_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_OBJTRUST_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_SIGTRUST_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_TESTFINALPOLICY_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVUI_CALL = ::core::option::Option<unsafe extern "system" fn(hwndsecuritydialog: super::super::Foundation::HWND, pprovdata: *const CRYPT_PROVIDER_DATA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA, dwsteperror: u32, dwregpolicysettings: u32, csigner: u32, rgpsigner: *mut *mut WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO, pvpolicyarg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct PROVDATA_SIP {
    pub cbStruct: u32,
    pub gSubject: ::windows::core::GUID,
    pub pSip: *mut super::Cryptography::Sip::SIP_DISPATCH_INFO,
    pub pCATSip: *mut super::Cryptography::Sip::SIP_DISPATCH_INFO,
    pub psSipSubjectInfo: *mut super::Cryptography::Sip::SIP_SUBJECTINFO,
    pub psSipCATSubjectInfo: *mut super::Cryptography::Sip::SIP_SUBJECTINFO,
    pub psIndirectData: *mut super::Cryptography::Sip::SIP_INDIRECT_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for PROVDATA_SIP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for PROVDATA_SIP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::fmt::Debug for PROVDATA_SIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROVDATA_SIP").field("cbStruct", &self.cbStruct).field("gSubject", &self.gSubject).field("pSip", &self.pSip).field("pCATSip", &self.pCATSip).field("psSipSubjectInfo", &self.psSipSubjectInfo).field("psSipCATSubjectInfo", &self.psSipCATSubjectInfo).field("psIndirectData", &self.psIndirectData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::core::Abi for PROVDATA_SIP {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for PROVDATA_SIP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROVDATA_SIP>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for PROVDATA_SIP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for PROVDATA_SIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SEALING_SIGNATURE_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub signatureAlgorithm: super::Cryptography::CRYPT_ALGORITHM_IDENTIFIER,
    pub encryptedDigest: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SEALING_SIGNATURE_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SEALING_SIGNATURE_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SEALING_SIGNATURE_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEALING_SIGNATURE_ATTRIBUTE").field("version", &self.version).field("signerIndex", &self.signerIndex).field("signatureAlgorithm", &self.signatureAlgorithm).field("encryptedDigest", &self.encryptedDigest).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SEALING_SIGNATURE_ATTRIBUTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SEALING_SIGNATURE_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SEALING_SIGNATURE_ATTRIBUTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SEALING_SIGNATURE_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SEALING_SIGNATURE_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SEALING_SIGNATURE_ATTRIBUTE_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2011i32 as _);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SEALING_TIMESTAMP_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub sealTimeStampToken: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SEALING_TIMESTAMP_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SEALING_TIMESTAMP_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SEALING_TIMESTAMP_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEALING_TIMESTAMP_ATTRIBUTE").field("version", &self.version).field("signerIndex", &self.signerIndex).field("sealTimeStampToken", &self.sealTimeStampToken).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SEALING_TIMESTAMP_ATTRIBUTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SEALING_TIMESTAMP_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SEALING_TIMESTAMP_ATTRIBUTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SEALING_TIMESTAMP_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SEALING_TIMESTAMP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SEALING_TIMESTAMP_ATTRIBUTE_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2012i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SGNR_TYPE_TIMESTAMP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_CAB_DATA_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.25";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_CAB_DATA_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2008i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_CERT_EXTENSIONS_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.14";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_COMMERCIAL_SP_KEY_PURPOSE_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.22";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_COMMON_NAME_OBJID: &'static str = "2.5.4.3";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_ENCRYPTED_DIGEST_RETRY_COUNT_OBJID: &'static str = "1.3.6.1.4.1.311.2.6.2";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_FILE_LINK_CHOICE: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SPC_FINANCIAL_CRITERIA {
    pub fFinancialInfoAvailable: super::super::Foundation::BOOL,
    pub fMeetsCriteria: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPC_FINANCIAL_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPC_FINANCIAL_CRITERIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SPC_FINANCIAL_CRITERIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_FINANCIAL_CRITERIA").field("fFinancialInfoAvailable", &self.fFinancialInfoAvailable).field("fMeetsCriteria", &self.fMeetsCriteria).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SPC_FINANCIAL_CRITERIA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SPC_FINANCIAL_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_FINANCIAL_CRITERIA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SPC_FINANCIAL_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SPC_FINANCIAL_CRITERIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_FINANCIAL_CRITERIA_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.27";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_FINANCIAL_CRITERIA_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2002i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_GLUE_RDN_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.25";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SPC_IMAGE {
    pub pImageLink: *mut SPC_LINK,
    pub Bitmap: super::Cryptography::CRYPTOAPI_BLOB,
    pub Metafile: super::Cryptography::CRYPTOAPI_BLOB,
    pub EnhancedMetafile: super::Cryptography::CRYPTOAPI_BLOB,
    pub GifFile: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_IMAGE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_IMAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_IMAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_IMAGE").field("pImageLink", &self.pImageLink).field("Bitmap", &self.Bitmap).field("Metafile", &self.Metafile).field("EnhancedMetafile", &self.EnhancedMetafile).field("GifFile", &self.GifFile).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SPC_IMAGE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_IMAGE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_IMAGE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SPC_INDIRECT_DATA_CONTENT {
    pub Data: super::Cryptography::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::Cryptography::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_INDIRECT_DATA_CONTENT {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_INDIRECT_DATA_CONTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_INDIRECT_DATA_CONTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_INDIRECT_DATA_CONTENT").field("Data", &self.Data).field("DigestAlgorithm", &self.DigestAlgorithm).field("Digest", &self.Digest).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SPC_INDIRECT_DATA_CONTENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_INDIRECT_DATA_CONTENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_INDIRECT_DATA_CONTENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_INDIRECT_DATA_CONTENT {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_INDIRECT_DATA_CONTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_INDIRECT_DATA_CONTENT_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2003i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_INDIRECT_DATA_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.4";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_INDIVIDUAL_SP_KEY_PURPOSE_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.21";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_JAVA_CLASS_DATA_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.20";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_JAVA_CLASS_DATA_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2009i32 as _);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SPC_LINK {
    pub dwLinkChoice: u32,
    pub Anonymous: SPC_LINK_0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_LINK {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_LINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SPC_LINK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_LINK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_LINK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_LINK {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_LINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub union SPC_LINK_0 {
    pub pwszUrl: ::windows::core::PWSTR,
    pub Moniker: SPC_SERIALIZED_OBJECT,
    pub pwszFile: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_LINK_0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_LINK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SPC_LINK_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_LINK_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_LINK_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_LINK_0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_LINK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_LINK_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.28";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_LINK_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2005i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_MINIMAL_CRITERIA_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.26";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_MINIMAL_CRITERIA_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2001i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_MONIKER_LINK_CHOICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_NATURAL_AUTH_PLUGIN_OBJID: &'static str = "1.3.6.1.4.1.311.96.1.1";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SPC_PE_IMAGE_DATA {
    pub Flags: super::Cryptography::CRYPT_BIT_BLOB,
    pub pFile: *mut SPC_LINK,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_PE_IMAGE_DATA {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_PE_IMAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_PE_IMAGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_PE_IMAGE_DATA").field("Flags", &self.Flags).field("pFile", &self.pFile).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SPC_PE_IMAGE_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_PE_IMAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_PE_IMAGE_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_PE_IMAGE_DATA {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_PE_IMAGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_PE_IMAGE_DATA_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.15";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_PE_IMAGE_DATA_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2004i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_PE_IMAGE_PAGE_HASHES_V1_OBJID: &'static str = "1.3.6.1.4.1.311.2.3.1";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_PE_IMAGE_PAGE_HASHES_V2_OBJID: &'static str = "1.3.6.1.4.1.311.2.3.2";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_RAW_FILE_DATA_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.18";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_RELAXED_PE_MARKER_CHECK_OBJID: &'static str = "1.3.6.1.4.1.311.2.6.1";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SPC_SERIALIZED_OBJECT {
    pub ClassId: [u8; 16],
    pub SerializedData: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_SERIALIZED_OBJECT {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_SERIALIZED_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_SERIALIZED_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_SERIALIZED_OBJECT").field("ClassId", &self.ClassId).field("SerializedData", &self.SerializedData).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SPC_SERIALIZED_OBJECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_SERIALIZED_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_SERIALIZED_OBJECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_SERIALIZED_OBJECT {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_SERIALIZED_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct SPC_SIGINFO {
    pub dwSipVersion: u32,
    pub gSIPGuid: ::windows::core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwReserved4: u32,
    pub dwReserved5: u32,
}
impl ::core::marker::Copy for SPC_SIGINFO {}
impl ::core::clone::Clone for SPC_SIGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPC_SIGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_SIGINFO").field("dwSipVersion", &self.dwSipVersion).field("gSIPGuid", &self.gSIPGuid).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).field("dwReserved4", &self.dwReserved4).field("dwReserved5", &self.dwReserved5).finish()
    }
}
unsafe impl ::windows::core::Abi for SPC_SIGINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SPC_SIGINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_SIGINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SPC_SIGINFO {}
impl ::core::default::Default for SPC_SIGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_SIGINFO_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.30";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_SIGINFO_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2130i32 as _);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SPC_SP_AGENCY_INFO {
    pub pPolicyInformation: *mut SPC_LINK,
    pub pwszPolicyDisplayText: ::windows::core::PWSTR,
    pub pLogoImage: *mut SPC_IMAGE,
    pub pLogoLink: *mut SPC_LINK,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_SP_AGENCY_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_SP_AGENCY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_SP_AGENCY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_SP_AGENCY_INFO").field("pPolicyInformation", &self.pPolicyInformation).field("pwszPolicyDisplayText", &self.pwszPolicyDisplayText).field("pLogoImage", &self.pLogoImage).field("pLogoLink", &self.pLogoLink).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SPC_SP_AGENCY_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_SP_AGENCY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_SP_AGENCY_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_SP_AGENCY_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_SP_AGENCY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_SP_AGENCY_INFO_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.10";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_SP_AGENCY_INFO_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2000i32 as _);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SPC_SP_OPUS_INFO {
    pub pwszProgramName: ::windows::core::PCWSTR,
    pub pMoreInfo: *mut SPC_LINK,
    pub pPublisherInfo: *mut SPC_LINK,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_SP_OPUS_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_SP_OPUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SPC_SP_OPUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_SP_OPUS_INFO").field("pwszProgramName", &self.pwszProgramName).field("pMoreInfo", &self.pMoreInfo).field("pPublisherInfo", &self.pPublisherInfo).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for SPC_SP_OPUS_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SPC_SP_OPUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_SP_OPUS_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SPC_SP_OPUS_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SPC_SP_OPUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_SP_OPUS_INFO_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.12";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_SP_OPUS_INFO_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2007i32 as _);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct SPC_STATEMENT_TYPE {
    pub cKeyPurposeId: u32,
    pub rgpszKeyPurposeId: *mut ::windows::core::PSTR,
}
impl ::core::marker::Copy for SPC_STATEMENT_TYPE {}
impl ::core::clone::Clone for SPC_STATEMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPC_STATEMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPC_STATEMENT_TYPE").field("cKeyPurposeId", &self.cKeyPurposeId).field("rgpszKeyPurposeId", &self.rgpszKeyPurposeId).finish()
    }
}
unsafe impl ::windows::core::Abi for SPC_STATEMENT_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SPC_STATEMENT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPC_STATEMENT_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SPC_STATEMENT_TYPE {}
impl ::core::default::Default for SPC_STATEMENT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_STATEMENT_TYPE_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.11";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_STATEMENT_TYPE_STRUCT: ::windows::core::PCSTR = ::windows::core::PCSTR(2006i32 as _);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_STRUCTURED_STORAGE_DATA_OBJID: &'static str = "1.3.6.1.4.1.311.2.1.19";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_TIME_STAMP_REQUEST_OBJID: &'static str = "1.3.6.1.4.1.311.3.2.1";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_URL_LINK_CHOICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_UUID_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SPC_WINDOWS_HELLO_COMPATIBILITY_OBJID: &'static str = "1.3.6.1.4.1.311.10.41.1";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_CHKCERT_FUNCTION: &'static str = "SoftpubCheckCert";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_CLEANUPPOLICY_FUNCTION: &'static str = "SoftpubCleanup";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_FINALPOLICY_FUNCTION: &'static str = "SoftpubAuthenticode";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_GENERIC_CERT_INIT_FUNCTION: &'static str = "SoftpubDefCertInit";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_INIT_FUNCTION: &'static str = "SoftpubInitialize";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_OBJTRUST_FUNCTION: &'static str = "SoftpubLoadMessage";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_POLICY_PROVIDER_DLL_NAME: &'static str = "WINTRUST.DLL";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_SIGTRUST_FUNCTION: &'static str = "SoftpubLoadSignature";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const SP_TESTDUMPPOLICY_FUNCTION_TEST: &'static str = "SoftpubDumpStructure";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_MAX_STEPS: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_CATALOGFILE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_CERTSTORE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FILEIO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FINAL_CERTCHKPROV: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FINAL_CERTPROV: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FINAL_INITPROV: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FINAL_OBJPROV: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FINAL_POLICYPROV: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FINAL_SIGPROV: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FINAL_UIPROV: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_FINAL_WVTINIT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_CERTCHAIN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_COUNTERSIGCERT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_COUNTERSIGINFO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_INNERCNT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_INNERCNTTYPE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_SIGNERCERT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_SIGNERCOUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_SIGNERINFO: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_MSG_STORE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_SIP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_SIPSUBJINFO: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_VERIFY_MSGHASH: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_VERIFY_MSGINDIRECTDATA: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const TRUSTERROR_STEP_WVTPARAMS: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct WINTRUST_BLOB_INFO {
    pub cbStruct: u32,
    pub gSubject: ::windows::core::GUID,
    pub pcwszDisplayName: ::windows::core::PCWSTR,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
impl ::core::marker::Copy for WINTRUST_BLOB_INFO {}
impl ::core::clone::Clone for WINTRUST_BLOB_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINTRUST_BLOB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_BLOB_INFO").field("cbStruct", &self.cbStruct).field("gSubject", &self.gSubject).field("pcwszDisplayName", &self.pcwszDisplayName).field("cbMemObject", &self.cbMemObject).field("pbMemObject", &self.pbMemObject).field("cbMemSignedMsg", &self.cbMemSignedMsg).field("pbMemSignedMsg", &self.pbMemSignedMsg).finish()
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_BLOB_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINTRUST_BLOB_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINTRUST_BLOB_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINTRUST_BLOB_INFO {}
impl ::core::default::Default for WINTRUST_BLOB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WINTRUST_CATALOG_INFO {
    pub cbStruct: u32,
    pub dwCatalogVersion: u32,
    pub pcwszCatalogFilePath: ::windows::core::PCWSTR,
    pub pcwszMemberTag: ::windows::core::PCWSTR,
    pub pcwszMemberFilePath: ::windows::core::PCWSTR,
    pub hMemberFile: super::super::Foundation::HANDLE,
    pub pbCalculatedFileHash: *mut u8,
    pub cbCalculatedFileHash: u32,
    pub pcCatalogContext: *mut super::Cryptography::CTL_CONTEXT,
    pub hCatAdmin: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_CATALOG_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_CATALOG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WINTRUST_CATALOG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_CATALOG_INFO")
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
unsafe impl ::windows::core::Abi for WINTRUST_CATALOG_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WINTRUST_CATALOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINTRUST_CATALOG_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WINTRUST_CATALOG_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WINTRUST_CATALOG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WINTRUST_CERT_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: ::windows::core::PCWSTR,
    pub psCertContext: *mut super::Cryptography::CERT_CONTEXT,
    pub chStores: u32,
    pub pahStores: *mut super::Cryptography::HCERTSTORE,
    pub dwFlags: u32,
    pub psftVerifyAsOf: *mut super::super::Foundation::FILETIME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_CERT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WINTRUST_CERT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_CERT_INFO").field("cbStruct", &self.cbStruct).field("pcwszDisplayName", &self.pcwszDisplayName).field("psCertContext", &self.psCertContext).field("chStores", &self.chStores).field("pahStores", &self.pahStores).field("dwFlags", &self.dwFlags).field("psftVerifyAsOf", &self.psftVerifyAsOf).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WINTRUST_CERT_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WINTRUST_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINTRUST_CERT_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WINTRUST_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WINTRUST_CERT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WINTRUST_CONFIG_REGPATH: &'static str = "Software\\Microsoft\\Cryptography\\Wintrust\\Config";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WINTRUST_DATA {
    pub cbStruct: u32,
    pub pPolicyCallbackData: *mut ::core::ffi::c_void,
    pub pSIPClientData: *mut ::core::ffi::c_void,
    pub dwUIChoice: WINTRUST_DATA_UICHOICE,
    pub fdwRevocationChecks: WINTRUST_DATA_REVOCATION_CHECKS,
    pub dwUnionChoice: WINTRUST_DATA_UNION_CHOICE,
    pub Anonymous: WINTRUST_DATA_0,
    pub dwStateAction: WINTRUST_DATA_STATE_ACTION,
    pub hWVTStateData: super::super::Foundation::HANDLE,
    pub pwszURLReference: ::windows::core::PWSTR,
    pub dwProvFlags: WINTRUST_DATA_PROVIDER_FLAGS,
    pub dwUIContext: WINTRUST_DATA_UICONTEXT,
    pub pSignatureSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WINTRUST_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WINTRUST_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINTRUST_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WINTRUST_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WINTRUST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub union WINTRUST_DATA_0 {
    pub pFile: *mut WINTRUST_FILE_INFO,
    pub pCatalog: *mut WINTRUST_CATALOG_INFO,
    pub pBlob: *mut WINTRUST_BLOB_INFO,
    pub pSgnr: *mut WINTRUST_SGNR_INFO,
    pub pCert: *mut WINTRUST_CERT_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WINTRUST_DATA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WINTRUST_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINTRUST_DATA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WINTRUST_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WINTRUST_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_DATA_PROVIDER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_USE_IE4_TRUST_FLAG: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_NO_IE4_CHAIN_FLAG: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_NO_POLICY_USAGE_FLAG: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_REVOCATION_CHECK_NONE: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_REVOCATION_CHECK_END_CERT: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_REVOCATION_CHECK_CHAIN: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_SAFER_FLAG: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_HASH_ONLY_FLAG: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_USE_DEFAULT_OSVER_CHECK: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_LIFETIME_SIGNING_FLAG: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_CACHE_ONLY_URL_RETRIEVAL: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_DISABLE_MD2_MD4: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_MOTW: WINTRUST_DATA_PROVIDER_FLAGS = WINTRUST_DATA_PROVIDER_FLAGS(16384u32);
impl ::core::marker::Copy for WINTRUST_DATA_PROVIDER_FLAGS {}
impl ::core::clone::Clone for WINTRUST_DATA_PROVIDER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_DATA_PROVIDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_DATA_PROVIDER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_DATA_PROVIDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_PROVIDER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINTRUST_DATA_PROVIDER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINTRUST_DATA_PROVIDER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINTRUST_DATA_PROVIDER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINTRUST_DATA_PROVIDER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINTRUST_DATA_PROVIDER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_DATA_REVOCATION_CHECKS(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_REVOKE_NONE: WINTRUST_DATA_REVOCATION_CHECKS = WINTRUST_DATA_REVOCATION_CHECKS(0u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_REVOKE_WHOLECHAIN: WINTRUST_DATA_REVOCATION_CHECKS = WINTRUST_DATA_REVOCATION_CHECKS(1u32);
impl ::core::marker::Copy for WINTRUST_DATA_REVOCATION_CHECKS {}
impl ::core::clone::Clone for WINTRUST_DATA_REVOCATION_CHECKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_DATA_REVOCATION_CHECKS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_DATA_REVOCATION_CHECKS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_DATA_REVOCATION_CHECKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_REVOCATION_CHECKS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_DATA_STATE_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_STATEACTION_IGNORE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(0u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_STATEACTION_VERIFY: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(1u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_STATEACTION_CLOSE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(2u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_STATEACTION_AUTO_CACHE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(3u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_STATEACTION_AUTO_CACHE_FLUSH: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(4u32);
impl ::core::marker::Copy for WINTRUST_DATA_STATE_ACTION {}
impl ::core::clone::Clone for WINTRUST_DATA_STATE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_DATA_STATE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_DATA_STATE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_DATA_STATE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_STATE_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_DATA_UICHOICE(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_UI_ALL: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(1u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_UI_NONE: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(2u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_UI_NOBAD: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(3u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_UI_NOGOOD: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(4u32);
impl ::core::marker::Copy for WINTRUST_DATA_UICHOICE {}
impl ::core::clone::Clone for WINTRUST_DATA_UICHOICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_DATA_UICHOICE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_DATA_UICHOICE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_DATA_UICHOICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_UICHOICE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_DATA_UICONTEXT(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_UICONTEXT_EXECUTE: WINTRUST_DATA_UICONTEXT = WINTRUST_DATA_UICONTEXT(0u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_UICONTEXT_INSTALL: WINTRUST_DATA_UICONTEXT = WINTRUST_DATA_UICONTEXT(1u32);
impl ::core::marker::Copy for WINTRUST_DATA_UICONTEXT {}
impl ::core::clone::Clone for WINTRUST_DATA_UICONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_DATA_UICONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_DATA_UICONTEXT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_DATA_UICONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_UICONTEXT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_DATA_UNION_CHOICE(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_CHOICE_FILE: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(1u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_CHOICE_CATALOG: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(2u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_CHOICE_BLOB: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(3u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_CHOICE_SIGNER: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(4u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_CHOICE_CERT: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(5u32);
impl ::core::marker::Copy for WINTRUST_DATA_UNION_CHOICE {}
impl ::core::clone::Clone for WINTRUST_DATA_UNION_CHOICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_DATA_UNION_CHOICE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_DATA_UNION_CHOICE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_DATA_UNION_CHOICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_DATA_UNION_CHOICE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINTRUST_FILE_INFO {
    pub cbStruct: u32,
    pub pcwszFilePath: ::windows::core::PCWSTR,
    pub hFile: super::super::Foundation::HANDLE,
    pub pgKnownSubject: *mut ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINTRUST_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINTRUST_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINTRUST_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_FILE_INFO").field("cbStruct", &self.cbStruct).field("pcwszFilePath", &self.pcwszFilePath).field("hFile", &self.hFile).field("pgKnownSubject", &self.pgKnownSubject).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINTRUST_FILE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINTRUST_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINTRUST_FILE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINTRUST_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINTRUST_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const DWACTION_ALLOCANDFILL: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(1u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const DWACTION_FREE: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(2u32);
impl ::core::marker::Copy for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {}
impl ::core::clone::Clone for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WINTRUST_MAX_HASH_BYTES_TO_MAP_DEFAULT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WINTRUST_MAX_HASH_BYTES_TO_MAP_VALUE_NAME: &'static str = "MaxHashBytesToMap";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WINTRUST_MAX_HEADER_BYTES_TO_MAP_DEFAULT: u32 = 10485760u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WINTRUST_MAX_HEADER_BYTES_TO_MAP_VALUE_NAME: &'static str = "MaxHeaderBytesToMap";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_POLICY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_TRUSTTEST: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_TESTCANBEVALID: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_IGNOREEXPIRATION: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_IGNOREREVOKATION: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_OFFLINEOK_IND: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_OFFLINEOK_COM: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_OFFLINEOKNBU_IND: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_OFFLINEOKNBU_COM: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_VERIFY_V1_OFF: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_IGNOREREVOCATIONONTS: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTPF_ALLOWONLYPERTRUST: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(262144u32);
impl ::core::marker::Copy for WINTRUST_POLICY_FLAGS {}
impl ::core::clone::Clone for WINTRUST_POLICY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_POLICY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_POLICY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_POLICY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_POLICY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WINTRUST_POLICY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WINTRUST_POLICY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WINTRUST_SGNR_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: ::windows::core::PCWSTR,
    pub psSignerInfo: *mut super::Cryptography::CMSG_SIGNER_INFO,
    pub chStores: u32,
    pub pahStores: *mut super::Cryptography::HCERTSTORE,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WINTRUST_SGNR_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WINTRUST_SGNR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WINTRUST_SGNR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_SGNR_INFO").field("cbStruct", &self.cbStruct).field("pcwszDisplayName", &self.pcwszDisplayName).field("psSignerInfo", &self.psSignerInfo).field("chStores", &self.chStores).field("pahStores", &self.pahStores).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for WINTRUST_SGNR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WINTRUST_SGNR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINTRUST_SGNR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WINTRUST_SGNR_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WINTRUST_SGNR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WINTRUST_SIGNATURE_SETTINGS {
    pub cbStruct: u32,
    pub dwIndex: u32,
    pub dwFlags: WINTRUST_SIGNATURE_SETTINGS_FLAGS,
    pub cSecondarySigs: u32,
    pub dwVerifiedSigIndex: u32,
    pub pCryptoPolicy: *mut super::Cryptography::CERT_STRONG_SIGN_PARA,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WINTRUST_SIGNATURE_SETTINGS {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WINTRUST_SIGNATURE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for WINTRUST_SIGNATURE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINTRUST_SIGNATURE_SETTINGS").field("cbStruct", &self.cbStruct).field("dwIndex", &self.dwIndex).field("dwFlags", &self.dwFlags).field("cSecondarySigs", &self.cSecondarySigs).field("dwVerifiedSigIndex", &self.dwVerifiedSigIndex).field("pCryptoPolicy", &self.pCryptoPolicy).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for WINTRUST_SIGNATURE_SETTINGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WINTRUST_SIGNATURE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINTRUST_SIGNATURE_SETTINGS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WINTRUST_SIGNATURE_SETTINGS {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WINTRUST_SIGNATURE_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINTRUST_SIGNATURE_SETTINGS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_VERIFY_SPECIFIC: WINTRUST_SIGNATURE_SETTINGS_FLAGS = WINTRUST_SIGNATURE_SETTINGS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_GET_SECONDARY_SIG_COUNT: WINTRUST_SIGNATURE_SETTINGS_FLAGS = WINTRUST_SIGNATURE_SETTINGS_FLAGS(2u32);
impl ::core::marker::Copy for WINTRUST_SIGNATURE_SETTINGS_FLAGS {}
impl ::core::clone::Clone for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINTRUST_SIGNATURE_SETTINGS_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct WIN_CERTIFICATE {
    pub dwLength: u32,
    pub wRevision: u16,
    pub wCertificateType: u16,
    pub bCertificate: [u8; 1],
}
impl ::core::marker::Copy for WIN_CERTIFICATE {}
impl ::core::clone::Clone for WIN_CERTIFICATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN_CERTIFICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_CERTIFICATE").field("dwLength", &self.dwLength).field("wRevision", &self.wRevision).field("wCertificateType", &self.wCertificateType).field("bCertificate", &self.bCertificate).finish()
    }
}
unsafe impl ::windows::core::Abi for WIN_CERTIFICATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIN_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN_CERTIFICATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIN_CERTIFICATE {}
impl ::core::default::Default for WIN_CERTIFICATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WIN_CERT_REVISION_1_0: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WIN_CERT_REVISION_2_0: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WIN_CERT_TYPE_RESERVED_1: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WIN_CERT_TYPE_TS_STACK_SIGNED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WIN_CERT_TYPE_X509: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    pub hClientToken: super::super::Foundation::HANDLE,
    pub lpCertificate: *mut WIN_CERTIFICATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN_SPUB_TRUSTED_PUBLISHER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_SPUB_TRUSTED_PUBLISHER_DATA").field("hClientToken", &self.hClientToken).field("lpCertificate", &self.lpCertificate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN_SPUB_TRUSTED_PUBLISHER_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN_SPUB_TRUSTED_PUBLISHER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    pub hClientToken: super::super::Foundation::HANDLE,
    pub SubjectType: *mut ::windows::core::GUID,
    pub Subject: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT").field("hClientToken", &self.hClientToken).field("SubjectType", &self.SubjectType).field("Subject", &self.Subject).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub struct WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    pub SubjectType: *mut ::windows::core::GUID,
    pub Subject: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WIN_TRUST_ACTDATA_SUBJECT_ONLY {}
impl ::core::clone::Clone for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_TRUST_ACTDATA_SUBJECT_ONLY").field("SubjectType", &self.SubjectType).field("Subject", &self.Subject).finish()
    }
}
unsafe impl ::windows::core::Abi for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN_TRUST_ACTDATA_SUBJECT_ONLY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIN_TRUST_ACTDATA_SUBJECT_ONLY {}
impl ::core::default::Default for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_SUBJECT_FILE {
    pub hFile: super::super::Foundation::HANDLE,
    pub lpPath: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN_TRUST_SUBJECT_FILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN_TRUST_SUBJECT_FILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN_TRUST_SUBJECT_FILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_TRUST_SUBJECT_FILE").field("hFile", &self.hFile).field("lpPath", &self.lpPath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIN_TRUST_SUBJECT_FILE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN_TRUST_SUBJECT_FILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN_TRUST_SUBJECT_FILE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN_TRUST_SUBJECT_FILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN_TRUST_SUBJECT_FILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    pub hFile: super::super::Foundation::HANDLE,
    pub lpPath: ::windows::core::PCWSTR,
    pub lpDisplayName: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN_TRUST_SUBJECT_FILE_AND_DISPLAY").field("hFile", &self.hFile).field("lpPath", &self.lpPath).field("lpDisplayName", &self.lpDisplayName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN_TRUST_SUBJECT_FILE_AND_DISPLAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_CERTTRUST_SUPPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_INPUT_FLAG_MASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_OBJTRUST_SUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_OUTPUT_FLAG_MASK: u32 = 3758096384u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_OUT_FILE_SUPPORTS_SEAL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_OUT_HAS_SEALING_INTENT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_OUT_SEALING_STATUS_VERIFIED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_SIGTRUST_SUPPORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WSS_VERIFY_SEALING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTCI_DONT_OPEN_STORES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTCI_OPEN_ONLY_ROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTCI_USE_LOCAL_MACHINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_CODE_INTEGRITY_DRIVER_MODE: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0,
    pub hChainEngine: super::Cryptography::HCERTCHAINENGINE,
    pub pChainPara: *mut super::Cryptography::CERT_CHAIN_PARA,
    pub dwFlags: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTD_GENERIC_CHAIN_POLICY_CREATE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub union WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct WTD_GENERIC_CHAIN_POLICY_DATA {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_DATA_0,
    pub pSignerChainInfo: *mut WTD_GENERIC_CHAIN_POLICY_CREATE_INFO,
    pub pCounterSignerChainInfo: *mut WTD_GENERIC_CHAIN_POLICY_CREATE_INFO,
    pub pfnPolicyCallback: PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK,
    pub pvPolicyArg: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::core::Abi for WTD_GENERIC_CHAIN_POLICY_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTD_GENERIC_CHAIN_POLICY_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub union WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
unsafe impl ::windows::core::Abi for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTD_GENERIC_CHAIN_POLICY_DATA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
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
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub union WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WTD_PROV_FLAGS_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn WTHelperCertCheckValidSignature(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperCertCheckValidSignature(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::core::HRESULT;
        }
        WTHelperCertCheckValidSignature(::core::mem::transmute(pprovdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn WTHelperCertIsSelfSigned(dwencoding: u32, pcert: *mut super::Cryptography::CERT_INFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperCertIsSelfSigned(dwencoding: u32, pcert: *mut super::Cryptography::CERT_INFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WTHelperCertIsSelfSigned(::core::mem::transmute(dwencoding), ::core::mem::transmute(pcert)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn WTHelperGetProvCertFromChain(psgnr: *mut CRYPT_PROVIDER_SGNR, idxcert: u32) -> *mut CRYPT_PROVIDER_CERT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvCertFromChain(psgnr: *mut CRYPT_PROVIDER_SGNR, idxcert: u32) -> *mut CRYPT_PROVIDER_CERT;
        }
        ::core::mem::transmute(WTHelperGetProvCertFromChain(::core::mem::transmute(psgnr), ::core::mem::transmute(idxcert)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn WTHelperGetProvPrivateDataFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, pgproviderid: *mut ::windows::core::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvPrivateDataFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, pgproviderid: *mut ::windows::core::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA;
        }
        ::core::mem::transmute(WTHelperGetProvPrivateDataFromChain(::core::mem::transmute(pprovdata), ::core::mem::transmute(pgproviderid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn WTHelperGetProvSignerFromChain<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pprovdata: *mut CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: Param2, idxcountersigner: u32) -> *mut CRYPT_PROVIDER_SGNR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvSignerFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: super::super::Foundation::BOOL, idxcountersigner: u32) -> *mut CRYPT_PROVIDER_SGNR;
        }
        ::core::mem::transmute(WTHelperGetProvSignerFromChain(::core::mem::transmute(pprovdata), ::core::mem::transmute(idxsigner), fcountersigner.into_param().abi(), ::core::mem::transmute(idxcountersigner)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn WTHelperProvDataFromStateData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hstatedata: Param0) -> *mut CRYPT_PROVIDER_DATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperProvDataFromStateData(hstatedata: super::super::Foundation::HANDLE) -> *mut CRYPT_PROVIDER_DATA;
        }
        ::core::mem::transmute(WTHelperProvDataFromStateData(hstatedata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WT_ADD_ACTION_ID_RET_RESULT_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WT_CURRENT_VERSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WT_PROVIDER_CERTTRUST_FUNCTION: &'static str = "WintrustCertificateTrust";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WT_PROVIDER_DLL_NAME: &'static str = "WINTRUST.DLL";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WT_TRUSTDBDIALOG_NO_UI_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WT_TRUSTDBDIALOG_ONLY_PUB_TAB_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WT_TRUSTDBDIALOG_WRITE_IEAK_STORE_FLAG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const WT_TRUSTDBDIALOG_WRITE_LEGACY_REG_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinVerifyTrust<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pgactionid: *mut ::windows::core::GUID, pwvtdata: *mut ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinVerifyTrust(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows::core::GUID, pwvtdata: *mut ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(WinVerifyTrust(hwnd.into_param().abi(), ::core::mem::transmute(pgactionid), ::core::mem::transmute(pwvtdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn WinVerifyTrustEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pgactionid: *mut ::windows::core::GUID, pwintrustdata: *mut WINTRUST_DATA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinVerifyTrustEx(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows::core::GUID, pwintrustdata: *mut WINTRUST_DATA) -> i32;
        }
        ::core::mem::transmute(WinVerifyTrustEx(hwnd.into_param().abi(), ::core::mem::transmute(pgactionid), ::core::mem::transmute(pwintrustdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WintrustAddActionID(pgactionid: *const ::windows::core::GUID, fdwflags: u32, psprovinfo: *const CRYPT_REGISTER_ACTIONID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustAddActionID(pgactionid: *const ::windows::core::GUID, fdwflags: u32, psprovinfo: *const CRYPT_REGISTER_ACTIONID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WintrustAddActionID(::core::mem::transmute(pgactionid), ::core::mem::transmute(fdwflags), ::core::mem::transmute(psprovinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WintrustAddDefaultForUsage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszusageoid: Param0, psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustAddDefaultForUsage(pszusageoid: ::windows::core::PCSTR, psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WintrustAddDefaultForUsage(pszusageoid.into_param().abi(), ::core::mem::transmute(psdefusage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WintrustGetDefaultForUsage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION, pszusageoid: Param1, psusage: *mut CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustGetDefaultForUsage(dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION, pszusageoid: ::windows::core::PCSTR, psusage: *mut CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WintrustGetDefaultForUsage(::core::mem::transmute(dwaction), pszusageoid.into_param().abi(), ::core::mem::transmute(psusage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
#[inline]
pub unsafe fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS);
        }
        WintrustGetRegPolicyFlags(::core::mem::transmute(pdwpolicyflags))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
#[inline]
pub unsafe fn WintrustLoadFunctionPointers(pgactionid: *mut ::windows::core::GUID, ppfns: *mut CRYPT_PROVIDER_FUNCTIONS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustLoadFunctionPointers(pgactionid: *mut ::windows::core::GUID, ppfns: *mut CRYPT_PROVIDER_FUNCTIONS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WintrustLoadFunctionPointers(::core::mem::transmute(pgactionid), ::core::mem::transmute(ppfns)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WintrustRemoveActionID(pgactionid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustRemoveActionID(pgactionid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WintrustRemoveActionID(::core::mem::transmute(pgactionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WintrustSetDefaultIncludePEPageHashes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(fincludepepagehashes: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes: super::super::Foundation::BOOL);
        }
        WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WintrustSetRegPolicyFlags(dwpolicyflags: WINTRUST_POLICY_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustSetRegPolicyFlags(dwpolicyflags: WINTRUST_POLICY_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WintrustSetRegPolicyFlags(::core::mem::transmute(dwpolicyflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_ENHANCED_HASH: &'static str = "1.3.6.1.4.1.311.2.5.1";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_INTENT_TO_SEAL: &'static str = "1.3.6.1.4.1.311.2.4.2";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_NESTED_SIGNATURE: &'static str = "1.3.6.1.4.1.311.2.4.1";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_PKCS_9_SEQUENCE_NUMBER: &'static str = "1.2.840.113549.1.9.25.4";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_SEALING_SIGNATURE: &'static str = "1.3.6.1.4.1.311.2.4.3";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_SEALING_TIMESTAMP: &'static str = "1.3.6.1.4.1.311.2.4.4";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_TRUSTED_CLIENT_AUTH_CA_LIST: &'static str = "1.3.6.1.4.1.311.2.2.2";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_TRUSTED_CODESIGNING_CA_LIST: &'static str = "1.3.6.1.4.1.311.2.2.1";
#[doc = "*Required features: `\"Win32_Security_WinTrust\"`*"]
pub const szOID_TRUSTED_SERVER_AUTH_CA_LIST: &'static str = "1.3.6.1.4.1.311.2.2.3";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
