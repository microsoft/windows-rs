impl ::core::default::Default for CERT_FILTER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_FILTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cExtensionChecks == other.cExtensionChecks && self.arrayExtensionChecks == other.arrayExtensionChecks && self.dwCheckingFlags == other.dwCheckingFlags
    }
}
impl ::core::cmp::Eq for CERT_FILTER_DATA {}
impl ::core::fmt::Debug for CERT_FILTER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_FILTER_DATA").field("dwSize", &self.dwSize).field("cExtensionChecks", &self.cExtensionChecks).field("arrayExtensionChecks", &self.arrayExtensionChecks).field("dwCheckingFlags", &self.dwCheckingFlags).finish()
    }
}
impl ::core::default::Default for CERT_FILTER_EXTENSION_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_FILTER_EXTENSION_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.szExtensionOID == other.szExtensionOID && self.dwTestOperation == other.dwTestOperation && self.pbTestData == other.pbTestData && self.cbTestData == other.cbTestData
    }
}
impl ::core::cmp::Eq for CERT_FILTER_EXTENSION_MATCH {}
impl ::core::fmt::Debug for CERT_FILTER_EXTENSION_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_FILTER_EXTENSION_MATCH").field("szExtensionOID", &self.szExtensionOID).field("dwTestOperation", &self.dwTestOperation).field("pbTestData", &self.pbTestData).field("cbTestData", &self.cbTestData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SELECTUI_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CERT_SELECTUI_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.hStore == other.hStore && self.prgpChain == other.prgpChain && self.cChain == other.cChain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CERT_SELECTUI_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CERT_SELECTUI_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_SELECTUI_INPUT").field("hStore", &self.hStore).field("prgpChain", &self.prgpChain).field("cChain", &self.cChain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SELECT_STRUCT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CERT_SELECT_STRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_SELECT_STRUCT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CERT_VERIFY_CERTIFICATE_TRUST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CERT_VIEWPROPERTIES_STRUCT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for CERT_VIEWPROPERTIES_STRUCT_A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.hwndParent == other.hwndParent
            && self.hInstance == other.hInstance
            && self.dwFlags == other.dwFlags
            && self.szTitle == other.szTitle
            && self.pCertContext == other.pCertContext
            && self.arrayPurposes == other.arrayPurposes
            && self.cArrayPurposes == other.cArrayPurposes
            && self.cRootStores == other.cRootStores
            && self.rghstoreRoots == other.rghstoreRoots
            && self.cStores == other.cStores
            && self.rghstoreCAs == other.rghstoreCAs
            && self.cTrustStores == other.cTrustStores
            && self.rghstoreTrust == other.rghstoreTrust
            && self.hprov == other.hprov
            && self.lCustData == other.lCustData
            && self.dwPad == other.dwPad
            && self.szHelpFileName == other.szHelpFileName
            && self.dwHelpId == other.dwHelpId
            && self.nStartPage == other.nStartPage
            && self.cArrayPropSheetPages == other.cArrayPropSheetPages
            && self.arrayPropSheetPages == other.arrayPropSheetPages
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for CERT_VIEWPROPERTIES_STRUCT_A {}
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
impl ::core::default::Default for CERT_VIEWPROPERTIES_STRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CERT_VIEWPROPERTIES_STRUCT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for CERT_VIEWPROPERTIES_STRUCT_W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.hwndParent == other.hwndParent
            && self.hInstance == other.hInstance
            && self.dwFlags == other.dwFlags
            && self.szTitle == other.szTitle
            && self.pCertContext == other.pCertContext
            && self.arrayPurposes == other.arrayPurposes
            && self.cArrayPurposes == other.cArrayPurposes
            && self.cRootStores == other.cRootStores
            && self.rghstoreRoots == other.rghstoreRoots
            && self.cStores == other.cStores
            && self.rghstoreCAs == other.rghstoreCAs
            && self.cTrustStores == other.cTrustStores
            && self.rghstoreTrust == other.rghstoreTrust
            && self.hprov == other.hprov
            && self.lCustData == other.lCustData
            && self.dwPad == other.dwPad
            && self.szHelpFileName == other.szHelpFileName
            && self.dwHelpId == other.dwHelpId
            && self.nStartPage == other.nStartPage
            && self.cArrayPropSheetPages == other.cArrayPropSheetPages
            && self.arrayPropSheetPages == other.arrayPropSheetPages
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for CERT_VIEWPROPERTIES_STRUCT_W {}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_CERT_MGR_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_CERT_MGR_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hwndParent == other.hwndParent && self.dwFlags == other.dwFlags && self.pwszTitle == other.pwszTitle && self.pszInitUsageOID == other.pszInitUsageOID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_CERT_MGR_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTUI_CERT_MGR_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_CERT_MGR_STRUCT").field("dwSize", &self.dwSize).field("hwndParent", &self.hwndParent).field("dwFlags", &self.dwFlags).field("pwszTitle", &self.pwszTitle).field("pszInitUsageOID", &self.pszInitUsageOID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_INITDIALOG_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_INITDIALOG_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.lParam == other.lParam && self.pCertContext == other.pCertContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_INITDIALOG_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTUI_INITDIALOG_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_INITDIALOG_STRUCT").field("lParam", &self.lParam).field("pCertContext", &self.pCertContext).finish()
    }
}
impl ::core::default::Default for CRYPTUI_VIEWCERTIFICATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CRYPTUI_VIEWCERTIFICATE_STRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip", feature = "Win32_Security_WinTrust", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CRYPTUI_VIEWCERTIFICATE_STRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_ADDITIONAL_CERT_CHOICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_SIGN").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pGuidSubject == other.pGuidSubject && self.cbBlob == other.cbBlob && self.pbBlob == other.pbBlob && self.pwszDisplayName == other.pwszDisplayName
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_DIGITAL_SIGN_BLOB_INFO").field("dwSize", &self.dwSize).field("pGuidSubject", &self.pGuidSubject).field("cbBlob", &self.cbBlob).field("pbBlob", &self.pbBlob).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_CERT_PVK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cbBlob == other.cbBlob && self.pbBlob == other.pbBlob
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_DIGITAL_SIGN_CONTEXT").field("dwSize", &self.dwSize).field("cbBlob", &self.cbBlob).field("pbBlob", &self.pbBlob).finish()
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwAttrFlags == other.dwAttrFlags && self.pwszDescription == other.pwszDescription && self.pwszMoreInfoLocation == other.pwszMoreInfoLocation && self.pszHashAlg == other.pszHashAlg && self.pwszSigningCertDisplayString == other.pwszSigningCertDisplayString && self.hAdditionalCertStore == other.hAdditionalCertStore && self.psAuthenticated == other.psAuthenticated && self.psUnauthenticated == other.psUnauthenticated
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_EXTENDED_INFO {}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pwszPvkFileName == other.pwszPvkFileName && self.pwszProvName == other.pwszProvName && self.dwProvType == other.dwProvType
    }
}
impl ::core::cmp::Eq for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_DIGITAL_SIGN_PVK_FILE_INFO").field("dwSize", &self.dwSize).field("pwszPvkFileName", &self.pwszPvkFileName).field("pwszProvName", &self.pwszProvName).field("dwProvType", &self.dwProvType).finish()
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_SIGN_PVK_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_SIGN_SIG_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_STORE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_DIGITAL_SIGN_SUBJECT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwExportFormat == other.dwExportFormat && self.fExportChain == other.fExportChain && self.fExportPrivateKeys == other.fExportPrivateKeys && self.pwszPassword == other.pwszPassword && self.fStrongEncryption == other.fStrongEncryption
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CRYPTUI_WIZ_EXPORT_CERTCONTEXT_INFO").field("dwSize", &self.dwSize).field("dwExportFormat", &self.dwExportFormat).field("fExportChain", &self.fExportChain).field("fExportPrivateKeys", &self.fExportPrivateKeys).field("pwszPassword", &self.pwszPassword).field("fStrongEncryption", &self.fStrongEncryption).finish()
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_EXPORT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_EXPORT_FORMAT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_EXPORT_SUBJECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_EXPORT_SUBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_EXPORT_SUBJECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CRYPTUI_WIZ_IMPORT_SRC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRYPTUI_WIZ_IMPORT_SUBJECT_OPTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CTL_MODIFY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CTL_MODIFY_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.pccert == other.pccert && self.dwOperation == other.dwOperation && self.dwError == other.dwError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CTL_MODIFY_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CTL_MODIFY_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CTL_MODIFY_REQUEST").field("pccert", &self.pccert).field("dwOperation", &self.dwOperation).field("dwError", &self.dwError).finish()
    }
}
impl ::core::default::Default for CTL_MODIFY_REQUEST_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CTL_MODIFY_REQUEST_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CTL_MODIFY_REQUEST_OPERATION").field(&self.0).finish()
    }
}
