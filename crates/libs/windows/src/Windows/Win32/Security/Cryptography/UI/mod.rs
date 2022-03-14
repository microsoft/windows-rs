#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const ACTION_REVOCATION_DEFAULT_CACHE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const ACTION_REVOCATION_DEFAULT_ONLINE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERTVIEW_CRYPTUI_LPARAM: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_CREDENTIAL_PROVIDER_ID: i32 = -509i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_DISTRUST_ADD_CA_CERT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_DISTRUST_ADD_LEAF_CERT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_DISTRUST_CA_CERT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_DISTRUST_LEAF_CERT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_SELECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_TRUST_ADD_CA_CERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_TRUST_ADD_LEAF_CERT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_TRUST_CA_CERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_DISPWELL_TRUST_LEAF_CERT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_INCLUDE_V1_CERTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_ISSUER_CERTS_ONLY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_KEY_EXISTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_LEAF_CERTS_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_OP_EQUALITY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_OP_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_OP_NOT_EXISTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_VALID_SIGNATURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_FILTER_VALID_TIME_RANGE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SELECTUI_INPUT {
    pub hStore: super::HCERTSTORE,
    pub prgpChain: *mut *mut super::CERT_CHAIN_CONTEXT,
    pub cChain: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_SELECTUI_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_SELECTUI_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_SELECTUI_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SELECTUI_INPUT").field("hStore", &self.hStore).field("prgpChain", &self.prgpChain).field("cChain", &self.cChain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CERT_SELECTUI_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_SELECTUI_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CERT_SELECTUI_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_SELECTUI_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SELECTUI_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SELECT_STRUCT_A {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pTemplateName: ::windows::core::PCSTR,
    pub dwFlags: CERT_SELECT_STRUCT_FLAGS,
    pub szTitle: ::windows::core::PCSTR,
    pub cCertStore: u32,
    pub arrayCertStore: *mut super::HCERTSTORE,
    pub szPurposeOid: ::windows::core::PCSTR,
    pub cCertContext: u32,
    pub arrayCertContext: *mut *mut super::CERT_CONTEXT,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub pfnHook: PFNCMHOOKPROC,
    pub pfnFilter: PFNCMFILTERPROC,
    pub szHelpFileName: ::windows::core::PCSTR,
    pub dwHelpId: u32,
    pub hprov: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_SELECT_STRUCT_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_SELECT_STRUCT_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_SELECT_STRUCT_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SELECT_STRUCT_A")
            .field("dwSize", &self.dwSize)
            .field("hwndParent", &self.hwndParent)
            .field("hInstance", &self.hInstance)
            .field("pTemplateName", &self.pTemplateName)
            .field("dwFlags", &self.dwFlags)
            .field("szTitle", &self.szTitle)
            .field("cCertStore", &self.cCertStore)
            .field("arrayCertStore", &self.arrayCertStore)
            .field("szPurposeOid", &self.szPurposeOid)
            .field("cCertContext", &self.cCertContext)
            .field("arrayCertContext", &self.arrayCertContext)
            .field("lCustData", &self.lCustData)
            .field("pfnHook", &self.pfnHook.map(|f| f as usize))
            .field("pfnFilter", &self.pfnFilter.map(|f| f as usize))
            .field("szHelpFileName", &self.szHelpFileName)
            .field("dwHelpId", &self.dwHelpId)
            .field("hprov", &self.hprov)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CERT_SELECT_STRUCT_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_SELECT_STRUCT_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CERT_SELECT_STRUCT_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_SELECT_STRUCT_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SELECT_STRUCT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CERT_SELECT_STRUCT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CSS_HIDE_PROPERTIES: CERT_SELECT_STRUCT_FLAGS = CERT_SELECT_STRUCT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CSS_ENABLEHOOK: CERT_SELECT_STRUCT_FLAGS = CERT_SELECT_STRUCT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CSS_ALLOWMULTISELECT: CERT_SELECT_STRUCT_FLAGS = CERT_SELECT_STRUCT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CSS_SHOW_HELP: CERT_SELECT_STRUCT_FLAGS = CERT_SELECT_STRUCT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CSS_ENABLETEMPLATE: CERT_SELECT_STRUCT_FLAGS = CERT_SELECT_STRUCT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CSS_ENABLETEMPLATEHANDLE: CERT_SELECT_STRUCT_FLAGS = CERT_SELECT_STRUCT_FLAGS(64u32);
impl ::core::marker::Copy for CERT_SELECT_STRUCT_FLAGS {}
impl ::core::clone::Clone for CERT_SELECT_STRUCT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CERT_SELECT_STRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CERT_SELECT_STRUCT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CERT_SELECT_STRUCT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_SELECT_STRUCT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_SELECT_STRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_SELECT_STRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_SELECT_STRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_SELECT_STRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_SELECT_STRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_SELECT_STRUCT_W {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pTemplateName: ::windows::core::PCWSTR,
    pub dwFlags: CERT_SELECT_STRUCT_FLAGS,
    pub szTitle: ::windows::core::PCWSTR,
    pub cCertStore: u32,
    pub arrayCertStore: *mut super::HCERTSTORE,
    pub szPurposeOid: ::windows::core::PCSTR,
    pub cCertContext: u32,
    pub arrayCertContext: *mut *mut super::CERT_CONTEXT,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub pfnHook: PFNCMHOOKPROC,
    pub pfnFilter: PFNCMFILTERPROC,
    pub szHelpFileName: ::windows::core::PCWSTR,
    pub dwHelpId: u32,
    pub hprov: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_SELECT_STRUCT_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_SELECT_STRUCT_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_SELECT_STRUCT_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SELECT_STRUCT_W")
            .field("dwSize", &self.dwSize)
            .field("hwndParent", &self.hwndParent)
            .field("hInstance", &self.hInstance)
            .field("pTemplateName", &self.pTemplateName)
            .field("dwFlags", &self.dwFlags)
            .field("szTitle", &self.szTitle)
            .field("cCertStore", &self.cCertStore)
            .field("arrayCertStore", &self.arrayCertStore)
            .field("szPurposeOid", &self.szPurposeOid)
            .field("cCertContext", &self.cCertContext)
            .field("arrayCertContext", &self.arrayCertContext)
            .field("lCustData", &self.lCustData)
            .field("pfnHook", &self.pfnHook.map(|f| f as usize))
            .field("pfnFilter", &self.pfnFilter.map(|f| f as usize))
            .field("szHelpFileName", &self.szHelpFileName)
            .field("dwHelpId", &self.dwHelpId)
            .field("hprov", &self.hprov)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CERT_SELECT_STRUCT_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_SELECT_STRUCT_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CERT_SELECT_STRUCT_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_SELECT_STRUCT_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SELECT_STRUCT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_TRUST_DO_FULL_SEARCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_TRUST_DO_FULL_TRUST: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_TRUST_MASK: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_TRUST_PERMIT_MISSING_CRLS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_AFTER_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_BEFORE_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_CERTIFICATE_REVOKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_CRL_OUT_OF_DATE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_EXPLICITLY_DISTRUSTED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_EXTENDED_USAGE_FAILURE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_ISSUER_DISTRUST: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_ISSUER_INVALID: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_KEY_USAGE_EXT_FAILURE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_MASK_TRUST: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_MASK_VALIDITY: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_NAME_CONSTRAINTS_FAILURE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_NO_CRL_FOUND: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_NO_ISSUER_CERT_FOUND: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_NO_TRUST_DATA: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_OTHER_ERROR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_OTHER_EXTENSION_FAILURE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_PERIOD_NESTING_FAILURE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_SIGNATURE_FAILS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CERT_VALIDITY_UNKNOWN_CRITICAL_EXTENSION: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CERT_VERIFY_CERTIFICATE_TRUST {
    pub cbSize: u32,
    pub pccert: *const super::CERT_CONTEXT,
    pub dwFlags: u32,
    pub dwIgnoreErr: u32,
    pub pdwErrors: *mut u32,
    pub pszUsageOid: ::windows::core::PSTR,
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
    pub prgpbTrustInfo: *mut *mut super::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CERT_VERIFY_CERTIFICATE_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CERT_VERIFY_CERTIFICATE_TRUST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_VERIFY_CERTIFICATE_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_VERIFY_CERTIFICATE_TRUST")
            .field("cbSize", &self.cbSize)
            .field("pccert", &self.pccert)
            .field("dwFlags", &self.dwFlags)
            .field("dwIgnoreErr", &self.dwIgnoreErr)
            .field("pdwErrors", &self.pdwErrors)
            .field("pszUsageOid", &self.pszUsageOid)
            .field("hprov", &self.hprov)
            .field("cRootStores", &self.cRootStores)
            .field("rghstoreRoots", &self.rghstoreRoots)
            .field("cStores", &self.cStores)
            .field("rghstoreCAs", &self.rghstoreCAs)
            .field("cTrustStores", &self.cTrustStores)
            .field("rghstoreTrust", &self.rghstoreTrust)
            .field("lCustData", &self.lCustData)
            .field("pfnTrustHelper", &self.pfnTrustHelper.map(|f| f as usize))
            .field("pcChain", &self.pcChain)
            .field("prgChain", &self.prgChain)
            .field("prgdwErrors", &self.prgdwErrors)
            .field("prgpbTrustInfo", &self.prgpbTrustInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CERT_VERIFY_CERTIFICATE_TRUST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_VERIFY_CERTIFICATE_TRUST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CERT_VERIFY_CERTIFICATE_TRUST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_VERIFY_CERTIFICATE_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_VERIFY_CERTIFICATE_TRUST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CERT_VIEWPROPERTIES_STRUCT_A {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub dwFlags: CERT_VIEWPROPERTIES_STRUCT_FLAGS,
    pub szTitle: ::windows::core::PCSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub arrayPurposes: *mut ::windows::core::PSTR,
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
    pub szHelpFileName: ::windows::core::PCSTR,
    pub dwHelpId: u32,
    pub nStartPage: u32,
    pub cArrayPropSheetPages: u32,
    pub arrayPropSheetPages: *mut super::super::super::UI::Controls::PROPSHEETPAGEA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CERT_VIEWPROPERTIES_STRUCT_A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CERT_VIEWPROPERTIES_STRUCT_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for CERT_VIEWPROPERTIES_STRUCT_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_VIEWPROPERTIES_STRUCT_A")
            .field("dwSize", &self.dwSize)
            .field("hwndParent", &self.hwndParent)
            .field("hInstance", &self.hInstance)
            .field("dwFlags", &self.dwFlags)
            .field("szTitle", &self.szTitle)
            .field("pCertContext", &self.pCertContext)
            .field("arrayPurposes", &self.arrayPurposes)
            .field("cArrayPurposes", &self.cArrayPurposes)
            .field("cRootStores", &self.cRootStores)
            .field("rghstoreRoots", &self.rghstoreRoots)
            .field("cStores", &self.cStores)
            .field("rghstoreCAs", &self.rghstoreCAs)
            .field("cTrustStores", &self.cTrustStores)
            .field("rghstoreTrust", &self.rghstoreTrust)
            .field("hprov", &self.hprov)
            .field("lCustData", &self.lCustData)
            .field("dwPad", &self.dwPad)
            .field("szHelpFileName", &self.szHelpFileName)
            .field("dwHelpId", &self.dwHelpId)
            .field("nStartPage", &self.nStartPage)
            .field("cArrayPropSheetPages", &self.cArrayPropSheetPages)
            .field("arrayPropSheetPages", &self.arrayPropSheetPages)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CERT_VIEWPROPERTIES_STRUCT_A {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for CERT_VIEWPROPERTIES_STRUCT_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CERT_VIEWPROPERTIES_STRUCT_A>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for CERT_VIEWPROPERTIES_STRUCT_A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CERT_VIEWPROPERTIES_STRUCT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CERT_VIEWPROPERTIES_STRUCT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_ENABLEHOOK: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_SHOW_HELP: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_SHOW_HELPICON: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_ENABLETEMPLATE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_HIDE_ADVANCEPAGE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_HIDE_TRUSTPAGE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_NO_NAMECHANGE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_NO_EDITTRUST: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_HIDE_DETAILPAGE: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_ADD_CERT_STORES: CERT_VIEWPROPERTIES_STRUCT_FLAGS = CERT_VIEWPROPERTIES_STRUCT_FLAGS(512u32);
impl ::core::marker::Copy for CERT_VIEWPROPERTIES_STRUCT_FLAGS {}
impl ::core::clone::Clone for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_VIEWPROPERTIES_STRUCT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CERT_VIEWPROPERTIES_STRUCT_W {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub dwFlags: CERT_VIEWPROPERTIES_STRUCT_FLAGS,
    pub szTitle: ::windows::core::PCWSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub arrayPurposes: *mut ::windows::core::PSTR,
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
    pub szHelpFileName: ::windows::core::PCWSTR,
    pub dwHelpId: u32,
    pub nStartPage: u32,
    pub cArrayPropSheetPages: u32,
    pub arrayPropSheetPages: *mut super::super::super::UI::Controls::PROPSHEETPAGEA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CERT_VIEWPROPERTIES_STRUCT_W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CERT_VIEWPROPERTIES_STRUCT_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for CERT_VIEWPROPERTIES_STRUCT_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_VIEWPROPERTIES_STRUCT_W")
            .field("dwSize", &self.dwSize)
            .field("hwndParent", &self.hwndParent)
            .field("hInstance", &self.hInstance)
            .field("dwFlags", &self.dwFlags)
            .field("szTitle", &self.szTitle)
            .field("pCertContext", &self.pCertContext)
            .field("arrayPurposes", &self.arrayPurposes)
            .field("cArrayPurposes", &self.cArrayPurposes)
            .field("cRootStores", &self.cRootStores)
            .field("rghstoreRoots", &self.rghstoreRoots)
            .field("cStores", &self.cStores)
            .field("rghstoreCAs", &self.rghstoreCAs)
            .field("cTrustStores", &self.cTrustStores)
            .field("rghstoreTrust", &self.rghstoreTrust)
            .field("hprov", &self.hprov)
            .field("lCustData", &self.lCustData)
            .field("dwPad", &self.dwPad)
            .field("szHelpFileName", &self.szHelpFileName)
            .field("dwHelpId", &self.dwHelpId)
            .field("nStartPage", &self.nStartPage)
            .field("cArrayPropSheetPages", &self.cArrayPropSheetPages)
            .field("arrayPropSheetPages", &self.arrayPropSheetPages)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CERT_VIEWPROPERTIES_STRUCT_W {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for CERT_VIEWPROPERTIES_STRUCT_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CERT_VIEWPROPERTIES_STRUCT_W>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for CERT_VIEWPROPERTIES_STRUCT_W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CERT_VIEWPROPERTIES_STRUCT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub struct CMFLTR {
    pub dwSize: u32,
    pub cExtensionChecks: u32,
    pub arrayExtensionChecks: *mut CMOID,
    pub dwCheckingFlags: u32,
}
impl ::core::marker::Copy for CMFLTR {}
impl ::core::clone::Clone for CMFLTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CMFLTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMFLTR").field("dwSize", &self.dwSize).field("cExtensionChecks", &self.cExtensionChecks).field("arrayExtensionChecks", &self.arrayExtensionChecks).field("dwCheckingFlags", &self.dwCheckingFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for CMFLTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CMFLTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CMFLTR>()) == 0 }
    }
}
impl ::core::cmp::Eq for CMFLTR {}
impl ::core::default::Default for CMFLTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub struct CMOID {
    pub szExtensionOID: ::windows::core::PCSTR,
    pub dwTestOperation: u32,
    pub pbTestData: *mut u8,
    pub cbTestData: u32,
}
impl ::core::marker::Copy for CMOID {}
impl ::core::clone::Clone for CMOID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CMOID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMOID").field("szExtensionOID", &self.szExtensionOID).field("dwTestOperation", &self.dwTestOperation).field("pbTestData", &self.pbTestData).field("cbTestData", &self.cbTestData).finish()
    }
}
unsafe impl ::windows::core::Abi for CMOID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CMOID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CMOID>()) == 0 }
    }
}
impl ::core::cmp::Eq for CMOID {}
impl ::core::default::Default for CMOID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CM_VIEWFLAGS_MASK: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTDLG_ACTION_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTDLG_CACHE_ONLY_URL_RETRIEVAL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTDLG_DISABLE_AIA: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTDLG_POLICY_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTDLG_REVOCATION_CACHE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTDLG_REVOCATION_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTDLG_REVOCATION_NONE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTDLG_REVOCATION_ONLINE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_CERT_MGR_PUBLISHER_TAB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_CERT_MGR_SINGLE_TAB_FLAG: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_CERT_MGR_STRUCT {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub pwszTitle: ::windows::core::PCWSTR,
    pub pszInitUsageOID: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_CERT_MGR_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_CERT_MGR_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTUI_CERT_MGR_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_CERT_MGR_STRUCT").field("dwSize", &self.dwSize).field("hwndParent", &self.hwndParent).field("dwFlags", &self.dwFlags).field("pwszTitle", &self.pwszTitle).field("pszInitUsageOID", &self.pszInitUsageOID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_CERT_MGR_STRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_CERT_MGR_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_CERT_MGR_STRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_CERT_MGR_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_CERT_MGR_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_CERT_MGR_TAB_MASK: u32 = 15u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_INITDIALOG_STRUCT {
    pub lParam: super::super::super::Foundation::LPARAM,
    pub pCertContext: *const super::CERT_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_INITDIALOG_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_INITDIALOG_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTUI_INITDIALOG_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_INITDIALOG_STRUCT").field("lParam", &self.lParam).field("pCertContext", &self.pCertContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_INITDIALOG_STRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_INITDIALOG_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_INITDIALOG_STRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_INITDIALOG_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_INITDIALOG_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_SELECT_EXPIRATION_COLUMN: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_SELECT_FRIENDLYNAME_COLUMN: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_SELECT_INTENDEDUSE_COLUMN: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_SELECT_ISSUEDBY_COLUMN: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_SELECT_ISSUEDTO_COLUMN: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_SELECT_LOCATION_COLUMN: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_VIEWCERTIFICATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_HIDE_HIERARCHYPAGE: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_HIDE_DETAILPAGE: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_DISABLE_EDITPROPERTIES: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_ENABLE_EDITPROPERTIES: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_DISABLE_ADDTOSTORE: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_ENABLE_ADDTOSTORE: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_ACCEPT_DECLINE_STYLE: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_IGNORE_UNTRUSTED_ROOT: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_DONT_OPEN_STORES: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_ONLY_OPEN_ROOT_STORE: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WARN_UNTRUSTED_ROOT: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_ENABLE_REVOCATION_CHECKING: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WARN_REMOTE_TRUST: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_DISABLE_EXPORT: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_END_CERT: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_CHAIN: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_ENABLE_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_DISABLE_HTMLLINK: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_DISABLE_ISSUERSTATEMENT: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_CACHE_ONLY_URL_RETRIEVAL: CRYPTUI_VIEWCERTIFICATE_FLAGS = CRYPTUI_VIEWCERTIFICATE_FLAGS(262144u32);
impl ::core::marker::Copy for CRYPTUI_VIEWCERTIFICATE_FLAGS {}
impl ::core::clone::Clone for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_VIEWCERTIFICATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`, `\"Win32_Security_WinTrust\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub dwFlags: CRYPTUI_VIEWCERTIFICATE_FLAGS,
    pub szTitle: ::windows::core::PCSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub rgszPurposes: *mut ::windows::core::PSTR,
    pub cPurposes: u32,
    pub Anonymous: CRYPTUI_VIEWCERTIFICATE_STRUCTA_0,
    pub fpCryptProviderDataTrustedUsage: super::super::super::Foundation::BOOL,
    pub idxSigner: u32,
    pub idxCert: u32,
    pub fCounterSigner: super::super::super::Foundation::BOOL,
    pub idxCounterSigner: u32,
    pub cStores: u32,
    pub rghStores: *mut super::HCERTSTORE,
    pub cPropSheetPages: u32,
    pub rgPropSheetPages: *mut super::super::super::UI::Controls::PROPSHEETPAGEA,
    pub nStartPage: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CRYPTUI_VIEWCERTIFICATE_STRUCTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_VIEWCERTIFICATE_STRUCTA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for CRYPTUI_VIEWCERTIFICATE_STRUCTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`, `\"Win32_Security_WinTrust\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub union CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    pub pCryptProviderData: *const super::super::WinTrust::CRYPT_PROVIDER_DATA,
    pub hWVTStateData: super::super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_VIEWCERTIFICATE_STRUCTA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CRYPTUI_VIEWCERTIFICATE_STRUCTA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`, `\"Win32_Security_WinTrust\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    pub dwSize: u32,
    pub hwndParent: super::super::super::Foundation::HWND,
    pub dwFlags: CRYPTUI_VIEWCERTIFICATE_FLAGS,
    pub szTitle: ::windows::core::PCWSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub rgszPurposes: *mut ::windows::core::PSTR,
    pub cPurposes: u32,
    pub Anonymous: CRYPTUI_VIEWCERTIFICATE_STRUCTW_0,
    pub fpCryptProviderDataTrustedUsage: super::super::super::Foundation::BOOL,
    pub idxSigner: u32,
    pub idxCert: u32,
    pub fCounterSigner: super::super::super::Foundation::BOOL,
    pub idxCounterSigner: u32,
    pub cStores: u32,
    pub rghStores: *mut super::HCERTSTORE,
    pub cPropSheetPages: u32,
    pub rgPropSheetPages: *mut super::super::super::UI::Controls::PROPSHEETPAGEW,
    pub nStartPage: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CRYPTUI_VIEWCERTIFICATE_STRUCTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_VIEWCERTIFICATE_STRUCTW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for CRYPTUI_VIEWCERTIFICATE_STRUCTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`, `\"Win32_Security_WinTrust\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub union CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    pub pCryptProviderData: *const super::super::WinTrust::CRYPT_PROVIDER_DATA,
    pub hWVTStateData: super::super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_VIEWCERTIFICATE_STRUCTW_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CRYPTUI_VIEWCERTIFICATE_STRUCTW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN: CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE = CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_ADD_CHAIN_NO_ROOT: CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE = CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_ADD_NONE: CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE = CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE(0u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_CERT: CRYPTUI_WIZ_DIGITAL_SIGN = CRYPTUI_WIZ_DIGITAL_SIGN(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_STORE: CRYPTUI_WIZ_DIGITAL_SIGN = CRYPTUI_WIZ_DIGITAL_SIGN(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK: CRYPTUI_WIZ_DIGITAL_SIGN = CRYPTUI_WIZ_DIGITAL_SIGN(3u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_NONE: CRYPTUI_WIZ_DIGITAL_SIGN = CRYPTUI_WIZ_DIGITAL_SIGN(0u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_SIGN").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    pub dwSize: u32,
    pub pGuidSubject: *mut ::windows::core::GUID,
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
    pub pwszDisplayName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO").field("dwSize", &self.dwSize).field("pGuidSubject", &self.pGuidSubject).field("cbBlob", &self.cbBlob).field("pbBlob", &self.pbBlob).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    pub dwSize: u32,
    pub pwszSigningCertFileName: ::windows::core::PWSTR,
    pub dwPvkChoice: CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION,
    pub Anonymous: CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0,
}
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    pub pPvkFileInfo: *mut CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO,
    pub pPvkProvInfo: *mut super::CRYPT_KEY_PROV_INFO,
}
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    pub dwSize: u32,
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT").field("dwSize", &self.dwSize).field("cbBlob", &self.cbBlob).field("pbBlob", &self.pbBlob).finish()
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_EXCLUDE_PAGE_HASHES: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    pub dwSize: u32,
    pub dwAttrFlags: CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE,
    pub pwszDescription: ::windows::core::PCWSTR,
    pub pwszMoreInfoLocation: ::windows::core::PCWSTR,
    pub pszHashAlg: ::windows::core::PCSTR,
    pub pwszSigningCertDisplayString: ::windows::core::PCWSTR,
    pub hAdditionalCertStore: super::HCERTSTORE,
    pub psAuthenticated: *mut super::CRYPT_ATTRIBUTES,
    pub psUnauthenticated: *mut super::CRYPT_ATTRIBUTES,
}
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO")
            .field("dwSize", &self.dwSize)
            .field("dwAttrFlags", &self.dwAttrFlags)
            .field("pwszDescription", &self.pwszDescription)
            .field("pwszMoreInfoLocation", &self.pwszMoreInfoLocation)
            .field("pszHashAlg", &self.pszHashAlg)
            .field("pwszSigningCertDisplayString", &self.pwszSigningCertDisplayString)
            .field("hAdditionalCertStore", &self.hAdditionalCertStore)
            .field("psAuthenticated", &self.psAuthenticated)
            .field("psUnauthenticated", &self.psUnauthenticated)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_INCLUDE_PAGE_HASHES: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    pub dwSize: u32,
    pub dwSubjectChoice: CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT,
    pub Anonymous1: CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0,
    pub dwSigningCertChoice: CRYPTUI_WIZ_DIGITAL_SIGN,
    pub Anonymous2: CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1,
    pub pwszTimestampURL: ::windows::core::PCWSTR,
    pub dwAdditionalCertChoice: CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE,
    pub pSignExtInfo: *mut CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    pub pwszFileName: ::windows::core::PCWSTR,
    pub pSignBlobInfo: *mut CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    pub pSigningCertContext: *const super::CERT_CONTEXT,
    pub pSigningCertStore: *mut CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO,
    pub pSigningCertPvkInfo: *mut CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    pub dwSize: u32,
    pub pwszPvkFileName: ::windows::core::PWSTR,
    pub pwszProvName: ::windows::core::PWSTR,
    pub dwProvType: u32,
}
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO").field("dwSize", &self.dwSize).field("pwszPvkFileName", &self.pwszPvkFileName).field("pwszProvName", &self.pwszProvName).field("dwProvType", &self.dwProvType).finish()
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE: CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION = CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_PVK_PROV: CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION = CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION(2u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_COMMERCIAL: CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE = CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_INDIVIDUAL: CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE = CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE(2u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    pub dwSize: u32,
    pub cCertStore: u32,
    pub rghCertStore: *mut super::HCERTSTORE,
    pub pFilterCallback: PFNCFILTERPROC,
    pub pvCallbackData: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO").field("dwSize", &self.dwSize).field("cCertStore", &self.cCertStore).field("rghCertStore", &self.rghCertStore).field("pFilterCallback", &self.pFilterCallback.map(|f| f as usize)).field("pvCallbackData", &self.pvCallbackData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_BLOB: CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT = CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_FILE: CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT = CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT_NONE: CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT = CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT(0u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT {}
impl ::core::clone::Clone for CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    pub dwSize: u32,
    pub dwExportFormat: CRYPTUI_WIZ_EXPORT_FORMAT,
    pub fExportChain: super::super::super::Foundation::BOOL,
    pub fExportPrivateKeys: super::super::super::Foundation::BOOL,
    pub pwszPassword: ::windows::core::PCWSTR,
    pub fStrongEncryption: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO").field("dwSize", &self.dwSize).field("dwExportFormat", &self.dwExportFormat).field("fExportChain", &self.fExportChain).field("fExportPrivateKeys", &self.fExportPrivateKeys).field("pwszPassword", &self.pwszPassword).field("fStrongEncryption", &self.fStrongEncryption).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_EXPORT_FORMAT(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_FORMAT_DER: CRYPTUI_WIZ_EXPORT_FORMAT = CRYPTUI_WIZ_EXPORT_FORMAT(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_FORMAT_PFX: CRYPTUI_WIZ_EXPORT_FORMAT = CRYPTUI_WIZ_EXPORT_FORMAT(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7: CRYPTUI_WIZ_EXPORT_FORMAT = CRYPTUI_WIZ_EXPORT_FORMAT(3u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_FORMAT_BASE64: CRYPTUI_WIZ_EXPORT_FORMAT = CRYPTUI_WIZ_EXPORT_FORMAT(4u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_FORMAT_CRL: CRYPTUI_WIZ_EXPORT_FORMAT = CRYPTUI_WIZ_EXPORT_FORMAT(6u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_FORMAT_CTL: CRYPTUI_WIZ_EXPORT_FORMAT = CRYPTUI_WIZ_EXPORT_FORMAT(7u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_EXPORT_FORMAT {}
impl ::core::clone::Clone for CRYPTUI_WIZ_EXPORT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_EXPORT_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_EXPORT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_EXPORT_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_FORMAT_SERIALIZED_CERT_STORE: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_EXPORT_INFO {
    pub dwSize: u32,
    pub pwszExportFileName: ::windows::core::PCWSTR,
    pub dwSubjectChoice: CRYPTUI_WIZ_EXPORT_SUBJECT,
    pub Anonymous: CRYPTUI_WIZ_EXPORT_INFO_0,
    pub cStores: u32,
    pub rghStores: *mut super::HCERTSTORE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_EXPORT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_EXPORT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_EXPORT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_EXPORT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_EXPORT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_EXPORT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CRYPTUI_WIZ_EXPORT_INFO_0 {
    pub pCertContext: *const super::CERT_CONTEXT,
    pub pCTLContext: *mut super::CTL_CONTEXT,
    pub pCRLContext: *mut super::CRL_CONTEXT,
    pub hCertStore: super::HCERTSTORE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_EXPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_EXPORT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_EXPORT_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_EXPORT_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_EXPORT_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_EXPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_EXPORT_SUBJECT(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_CERT_CONTEXT: CRYPTUI_WIZ_EXPORT_SUBJECT = CRYPTUI_WIZ_EXPORT_SUBJECT(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_CTL_CONTEXT: CRYPTUI_WIZ_EXPORT_SUBJECT = CRYPTUI_WIZ_EXPORT_SUBJECT(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_CRL_CONTEXT: CRYPTUI_WIZ_EXPORT_SUBJECT = CRYPTUI_WIZ_EXPORT_SUBJECT(3u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_CERT_STORE: CRYPTUI_WIZ_EXPORT_SUBJECT = CRYPTUI_WIZ_EXPORT_SUBJECT(4u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_CERT_STORE_CERTIFICATES_ONLY: CRYPTUI_WIZ_EXPORT_SUBJECT = CRYPTUI_WIZ_EXPORT_SUBJECT(5u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_EXPORT_SUBJECT {}
impl ::core::clone::Clone for CRYPTUI_WIZ_EXPORT_SUBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_SUBJECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_EXPORT_SUBJECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_EXPORT_SUBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_EXPORT_SUBJECT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_NO_UI: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IGNORE_NO_UI_FLAG_FOR_CSPS: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_NO_UI_EXCEPT_CSP: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CERT: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CRL: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CTL: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_NO_CHANGE_DEST_STORE: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_TO_LOCALMACHINE: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_TO_CURRENTUSER: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_REMOTE_DEST_STORE: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_PRIVATE_KEY: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_EXPORT_NO_DELETE_PRIVATE_KEY: CRYPTUI_WIZ_FLAGS = CRYPTUI_WIZ_FLAGS(512u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_FLAGS {}
impl ::core::clone::Clone for CRYPTUI_WIZ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRYPTUI_WIZ_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRYPTUI_WIZ_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRYPTUI_WIZ_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRYPTUI_WIZ_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRYPTUI_WIZ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTUI_WIZ_IMPORT_SRC_INFO {
    pub dwSize: u32,
    pub dwSubjectChoice: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION,
    pub Anonymous: CRYPTUI_WIZ_IMPORT_SRC_INFO_0,
    pub dwFlags: super::CRYPT_KEY_FLAGS,
    pub pwszPassword: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_IMPORT_SRC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_IMPORT_SRC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_IMPORT_SRC_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_IMPORT_SRC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_IMPORT_SRC_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_IMPORT_SRC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_IMPORT_SRC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    pub pwszFileName: ::windows::core::PCWSTR,
    pub pCertContext: *const super::CERT_CONTEXT,
    pub pCTLContext: *mut super::CTL_CONTEXT,
    pub pCRLContext: *mut super::CRL_CONTEXT,
    pub hCertStore: super::HCERTSTORE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CRYPTUI_WIZ_IMPORT_SRC_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_IMPORT_SRC_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_FILE: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_CONTEXT: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION(2u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CTL_CONTEXT: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION(3u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CRL_CONTEXT: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION(4u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYPTUI_WIZ_IMPORT_SUBJECT_CERT_STORE: CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION = CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION(5u32);
impl ::core::marker::Copy for CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION {}
impl ::core::clone::Clone for CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CRYTPDLG_FLAGS_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CSS_SELECTCERT_MASK: u32 = 16777215u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CTL_MODIFY_REQUEST {
    pub pccert: *const super::CERT_CONTEXT,
    pub dwOperation: CTL_MODIFY_REQUEST_OPERATION,
    pub dwError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CTL_MODIFY_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CTL_MODIFY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CTL_MODIFY_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_MODIFY_REQUEST").field("pccert", &self.pccert).field("dwOperation", &self.dwOperation).field("dwError", &self.dwError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CTL_MODIFY_REQUEST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CTL_MODIFY_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CTL_MODIFY_REQUEST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CTL_MODIFY_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CTL_MODIFY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CTL_MODIFY_REQUEST_OPERATION(pub u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CTL_MODIFY_REQUEST_ADD_TRUSTED: CTL_MODIFY_REQUEST_OPERATION = CTL_MODIFY_REQUEST_OPERATION(3u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CTL_MODIFY_REQUEST_ADD_NOT_TRUSTED: CTL_MODIFY_REQUEST_OPERATION = CTL_MODIFY_REQUEST_OPERATION(1u32);
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const CTL_MODIFY_REQUEST_REMOVE: CTL_MODIFY_REQUEST_OPERATION = CTL_MODIFY_REQUEST_OPERATION(2u32);
impl ::core::marker::Copy for CTL_MODIFY_REQUEST_OPERATION {}
impl ::core::clone::Clone for CTL_MODIFY_REQUEST_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CTL_MODIFY_REQUEST_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CTL_MODIFY_REQUEST_OPERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for CTL_MODIFY_REQUEST_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CTL_MODIFY_REQUEST_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CertSelectionGetSerializedBlob(pcsi: *const CERT_SELECTUI_INPUT, ppoutbuffer: *mut *mut ::core::ffi::c_void, puloutbuffersize: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CertSelectionGetSerializedBlob(pcsi: *const CERT_SELECTUI_INPUT, ppoutbuffer: *mut *mut ::core::ffi::c_void, puloutbuffersize: *mut u32) -> ::windows::core::HRESULT;
        }
        CertSelectionGetSerializedBlob(::core::mem::transmute(pcsi), ::core::mem::transmute(ppoutbuffer), ::core::mem::transmute(puloutbuffersize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptUIDlgCertMgr(pcryptuicertmgr: *const CRYPTUI_CERT_MGR_STRUCT) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIDlgCertMgr(pcryptuicertmgr: *const CRYPTUI_CERT_MGR_STRUCT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptUIDlgCertMgr(::core::mem::transmute(pcryptuicertmgr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptUIDlgSelectCertificateFromStore<'a, Param0: ::windows::core::IntoParam<'a, super::HCERTSTORE>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hcertstore: Param0, hwnd: Param1, pwsztitle: Param2, pwszdisplaystring: Param3, dwdontusecolumn: u32, dwflags: u32, pvreserved: *const ::core::ffi::c_void) -> *mut super::CERT_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIDlgSelectCertificateFromStore(hcertstore: super::HCERTSTORE, hwnd: super::super::super::Foundation::HWND, pwsztitle: ::windows::core::PCWSTR, pwszdisplaystring: ::windows::core::PCWSTR, dwdontusecolumn: u32, dwflags: u32, pvreserved: *const ::core::ffi::c_void) -> *mut super::CERT_CONTEXT;
        }
        ::core::mem::transmute(CryptUIDlgSelectCertificateFromStore(hcertstore.into_param().abi(), hwnd.into_param().abi(), pwsztitle.into_param().abi(), pwszdisplaystring.into_param().abi(), ::core::mem::transmute(dwdontusecolumn), ::core::mem::transmute(dwflags), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`, `\"Win32_Security_WinTrust\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CryptUIDlgViewCertificateA(pcertviewinfo: *const CRYPTUI_VIEWCERTIFICATE_STRUCTA, pfpropertieschanged: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIDlgViewCertificateA(pcertviewinfo: *const CRYPTUI_VIEWCERTIFICATE_STRUCTA, pfpropertieschanged: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptUIDlgViewCertificateA(::core::mem::transmute(pcertviewinfo), ::core::mem::transmute(pfpropertieschanged)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security_Cryptography_Catalog\"`, `\"Win32_Security_Cryptography_Sip\"`, `\"Win32_Security_WinTrust\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CryptUIDlgViewCertificateW(pcertviewinfo: *const CRYPTUI_VIEWCERTIFICATE_STRUCTW, pfpropertieschanged: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIDlgViewCertificateW(pcertviewinfo: *const CRYPTUI_VIEWCERTIFICATE_STRUCTW, pfpropertieschanged: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptUIDlgViewCertificateW(::core::mem::transmute(pcertviewinfo), ::core::mem::transmute(pfpropertieschanged)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptUIDlgViewContext<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwcontexttype: u32, pvcontext: *const ::core::ffi::c_void, hwnd: Param2, pwsztitle: Param3, dwflags: u32, pvreserved: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIDlgViewContext(dwcontexttype: u32, pvcontext: *const ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, pwsztitle: ::windows::core::PCWSTR, dwflags: u32, pvreserved: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptUIDlgViewContext(::core::mem::transmute(dwcontexttype), ::core::mem::transmute(pvcontext), hwnd.into_param().abi(), pwsztitle.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptUIWizDigitalSign<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwflags: u32, hwndparent: Param1, pwszwizardtitle: Param2, pdigitalsigninfo: *const CRYPTUI_WIZ_DIGITAL_SIGN_INFO, ppsigncontext: *mut *mut CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIWizDigitalSign(dwflags: u32, hwndparent: super::super::super::Foundation::HWND, pwszwizardtitle: ::windows::core::PCWSTR, pdigitalsigninfo: *const CRYPTUI_WIZ_DIGITAL_SIGN_INFO, ppsigncontext: *mut *mut CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptUIWizDigitalSign(::core::mem::transmute(dwflags), hwndparent.into_param().abi(), pwszwizardtitle.into_param().abi(), ::core::mem::transmute(pdigitalsigninfo), ::core::mem::transmute(ppsigncontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptUIWizExport<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwflags: CRYPTUI_WIZ_FLAGS, hwndparent: Param1, pwszwizardtitle: Param2, pexportinfo: *const CRYPTUI_WIZ_EXPORT_INFO, pvoid: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIWizExport(dwflags: CRYPTUI_WIZ_FLAGS, hwndparent: super::super::super::Foundation::HWND, pwszwizardtitle: ::windows::core::PCWSTR, pexportinfo: *const CRYPTUI_WIZ_EXPORT_INFO, pvoid: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptUIWizExport(::core::mem::transmute(dwflags), hwndparent.into_param().abi(), pwszwizardtitle.into_param().abi(), ::core::mem::transmute(pexportinfo), ::core::mem::transmute(pvoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptUIWizFreeDigitalSignContext(psigncontext: *const CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIWizFreeDigitalSignContext(psigncontext: *const CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptUIWizFreeDigitalSignContext(::core::mem::transmute(psigncontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CryptUIWizImport<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::HCERTSTORE>>(dwflags: CRYPTUI_WIZ_FLAGS, hwndparent: Param1, pwszwizardtitle: Param2, pimportsrc: *const CRYPTUI_WIZ_IMPORT_SRC_INFO, hdestcertstore: Param4) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUIWizImport(dwflags: CRYPTUI_WIZ_FLAGS, hwndparent: super::super::super::Foundation::HWND, pwszwizardtitle: ::windows::core::PCWSTR, pimportsrc: *const CRYPTUI_WIZ_IMPORT_SRC_INFO, hdestcertstore: super::HCERTSTORE) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CryptUIWizImport(::core::mem::transmute(dwflags), hwndparent.into_param().abi(), pwszwizardtitle.into_param().abi(), ::core::mem::transmute(pimportsrc), hdestcertstore.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNCFILTERPROC = ::core::option::Option<unsafe extern "system" fn(pcertcontext: *const super::CERT_CONTEXT, pfinitialselectedcert: *mut super::super::super::Foundation::BOOL, pvcallbackdata: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNCMFILTERPROC = ::core::option::Option<unsafe extern "system" fn(pcertcontext: *const super::CERT_CONTEXT, param1: super::super::super::Foundation::LPARAM, param2: u32, param3: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNCMHOOKPROC = ::core::option::Option<unsafe extern "system" fn(hwnddialog: super::super::super::Foundation::HWND, message: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNTRUSTHELPER = ::core::option::Option<unsafe extern "system" fn(pcertcontext: *const super::CERT_CONTEXT, lcustdata: super::super::super::Foundation::LPARAM, fleafcertificate: super::super::super::Foundation::BOOL, pbtrustblob: *mut u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const POLICY_IGNORE_NON_CRITICAL_BC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const SELCERT_ALGORITHM: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const SELCERT_CERTLIST: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const SELCERT_FINEPRINT: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const SELCERT_ISSUED_TO: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const SELCERT_PROPERTIES: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const SELCERT_SERIAL_NUM: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const SELCERT_THUMBPRINT: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const SELCERT_VALIDITY: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_UI\"`*"]
pub const szCERT_CERTIFICATE_ACTION_VERIFY: &'static str = "{7801ebd0-cf4b-11d0-851f-0060979387ea}";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
