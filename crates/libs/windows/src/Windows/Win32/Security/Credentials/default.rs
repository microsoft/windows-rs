impl ::core::default::Default for BINARY_BLOB_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BINARY_BLOB_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbBlob == other.cbBlob && self.pbBlob == other.pbBlob
    }
}
impl ::core::cmp::Eq for BINARY_BLOB_CREDENTIAL_INFO {}
impl ::core::fmt::Debug for BINARY_BLOB_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BINARY_BLOB_CREDENTIAL_INFO").field("cbBlob", &self.cbBlob).field("pbBlob", &self.pbBlob).finish()
    }
}
impl ::core::default::Default for CERT_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERT_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rgbHashOfCert == other.rgbHashOfCert
    }
}
impl ::core::cmp::Eq for CERT_CREDENTIAL_INFO {}
impl ::core::fmt::Debug for CERT_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CREDENTIAL_INFO").field("cbSize", &self.cbSize).field("rgbHashOfCert", &self.rgbHashOfCert).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIALA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIALA {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Type == other.Type && self.TargetName == other.TargetName && self.Comment == other.Comment && self.LastWritten == other.LastWritten && self.CredentialBlobSize == other.CredentialBlobSize && self.CredentialBlob == other.CredentialBlob && self.Persist == other.Persist && self.AttributeCount == other.AttributeCount && self.Attributes == other.Attributes && self.TargetAlias == other.TargetAlias && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIALA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIALA")
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("TargetName", &self.TargetName)
            .field("Comment", &self.Comment)
            .field("LastWritten", &self.LastWritten)
            .field("CredentialBlobSize", &self.CredentialBlobSize)
            .field("CredentialBlob", &self.CredentialBlob)
            .field("Persist", &self.Persist)
            .field("AttributeCount", &self.AttributeCount)
            .field("Attributes", &self.Attributes)
            .field("TargetAlias", &self.TargetAlias)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREDENTIALW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREDENTIALW {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Type == other.Type && self.TargetName == other.TargetName && self.Comment == other.Comment && self.LastWritten == other.LastWritten && self.CredentialBlobSize == other.CredentialBlobSize && self.CredentialBlob == other.CredentialBlob && self.Persist == other.Persist && self.AttributeCount == other.AttributeCount && self.Attributes == other.Attributes && self.TargetAlias == other.TargetAlias && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREDENTIALW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIALW")
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("TargetName", &self.TargetName)
            .field("Comment", &self.Comment)
            .field("LastWritten", &self.LastWritten)
            .field("CredentialBlobSize", &self.CredentialBlobSize)
            .field("CredentialBlob", &self.CredentialBlob)
            .field("Persist", &self.Persist)
            .field("AttributeCount", &self.AttributeCount)
            .field("Attributes", &self.Attributes)
            .field("TargetAlias", &self.TargetAlias)
            .field("UserName", &self.UserName)
            .finish()
    }
}
impl ::core::default::Default for CREDENTIAL_ATTRIBUTEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREDENTIAL_ATTRIBUTEA {
    fn eq(&self, other: &Self) -> bool {
        self.Keyword == other.Keyword && self.Flags == other.Flags && self.ValueSize == other.ValueSize && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CREDENTIAL_ATTRIBUTEA {}
impl ::core::fmt::Debug for CREDENTIAL_ATTRIBUTEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_ATTRIBUTEA").field("Keyword", &self.Keyword).field("Flags", &self.Flags).field("ValueSize", &self.ValueSize).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_ATTRIBUTEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREDENTIAL_ATTRIBUTEW {
    fn eq(&self, other: &Self) -> bool {
        self.Keyword == other.Keyword && self.Flags == other.Flags && self.ValueSize == other.ValueSize && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CREDENTIAL_ATTRIBUTEW {}
impl ::core::fmt::Debug for CREDENTIAL_ATTRIBUTEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_ATTRIBUTEW").field("Keyword", &self.Keyword).field("Flags", &self.Flags).field("ValueSize", &self.ValueSize).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_TARGET_INFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREDENTIAL_TARGET_INFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName && self.NetbiosServerName == other.NetbiosServerName && self.DnsServerName == other.DnsServerName && self.NetbiosDomainName == other.NetbiosDomainName && self.DnsDomainName == other.DnsDomainName && self.DnsTreeName == other.DnsTreeName && self.PackageName == other.PackageName && self.Flags == other.Flags && self.CredTypeCount == other.CredTypeCount && self.CredTypes == other.CredTypes
    }
}
impl ::core::cmp::Eq for CREDENTIAL_TARGET_INFORMATIONA {}
impl ::core::fmt::Debug for CREDENTIAL_TARGET_INFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_TARGET_INFORMATIONA").field("TargetName", &self.TargetName).field("NetbiosServerName", &self.NetbiosServerName).field("DnsServerName", &self.DnsServerName).field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("DnsTreeName", &self.DnsTreeName).field("PackageName", &self.PackageName).field("Flags", &self.Flags).field("CredTypeCount", &self.CredTypeCount).field("CredTypes", &self.CredTypes).finish()
    }
}
impl ::core::default::Default for CREDENTIAL_TARGET_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREDENTIAL_TARGET_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName && self.NetbiosServerName == other.NetbiosServerName && self.DnsServerName == other.DnsServerName && self.NetbiosDomainName == other.NetbiosDomainName && self.DnsDomainName == other.DnsDomainName && self.DnsTreeName == other.DnsTreeName && self.PackageName == other.PackageName && self.Flags == other.Flags && self.CredTypeCount == other.CredTypeCount && self.CredTypes == other.CredTypes
    }
}
impl ::core::cmp::Eq for CREDENTIAL_TARGET_INFORMATIONW {}
impl ::core::fmt::Debug for CREDENTIAL_TARGET_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDENTIAL_TARGET_INFORMATIONW").field("TargetName", &self.TargetName).field("NetbiosServerName", &self.NetbiosServerName).field("DnsServerName", &self.DnsServerName).field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("DnsTreeName", &self.DnsTreeName).field("PackageName", &self.PackageName).field("Flags", &self.Flags).field("CredTypeCount", &self.CredTypeCount).field("CredTypes", &self.CredTypes).finish()
    }
}
impl ::core::default::Default for CREDSPP_SUBMIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDSPP_SUBMIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDSPP_SUBMIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREDSSP_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREDSSP_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pSchannelCred == other.pSchannelCred && self.pSpnegoCred == other.pSpnegoCred
    }
}
impl ::core::cmp::Eq for CREDSSP_CRED {}
impl ::core::fmt::Debug for CREDSSP_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDSSP_CRED").field("Type", &self.Type).field("pSchannelCred", &self.pSchannelCred).field("pSpnegoCred", &self.pSpnegoCred).finish()
    }
}
impl ::core::default::Default for CREDSSP_CRED_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREDSSP_CRED_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Version == other.Version && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Cred == other.Cred
    }
}
impl ::core::cmp::Eq for CREDSSP_CRED_EX {}
impl ::core::fmt::Debug for CREDSSP_CRED_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDSSP_CRED_EX").field("Type", &self.Type).field("Version", &self.Version).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Cred", &self.Cred).finish()
    }
}
impl ::core::default::Default for CREDUIWIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDUIWIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDUIWIN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CREDUIWIN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREDUIWIN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREDUIWIN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREDUIWIN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREDUIWIN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CREDUI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREDUI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREDUI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CREDUI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREDUI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREDUI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREDUI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREDUI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CREDUI_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CREDUI_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwndParent == other.hwndParent && self.pszMessageText == other.pszMessageText && self.pszCaptionText == other.pszCaptionText && self.hbmBanner == other.hbmBanner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CREDUI_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CREDUI_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDUI_INFOA").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pszMessageText", &self.pszMessageText).field("pszCaptionText", &self.pszCaptionText).field("hbmBanner", &self.hbmBanner).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CREDUI_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CREDUI_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwndParent == other.hwndParent && self.pszMessageText == other.pszMessageText && self.pszCaptionText == other.pszCaptionText && self.hbmBanner == other.hbmBanner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CREDUI_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CREDUI_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREDUI_INFOW").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pszMessageText", &self.pszMessageText).field("pszCaptionText", &self.pszCaptionText).field("hbmBanner", &self.hbmBanner).finish()
    }
}
impl ::core::default::Default for CRED_ENUMERATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRED_ENUMERATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_ENUMERATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRED_ENUMERATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRED_ENUMERATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRED_ENUMERATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRED_MARSHAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRED_MARSHAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_MARSHAL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRED_PACK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRED_PACK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_PACK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CRED_PACK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CRED_PACK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CRED_PACK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CRED_PACK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CRED_PACK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CRED_PERSIST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRED_PERSIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_PERSIST").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRED_PROTECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRED_PROTECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_PROTECTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRED_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRED_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KeyCredentialManagerInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KeyCredentialManagerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.containerId == other.containerId
    }
}
impl ::core::cmp::Eq for KeyCredentialManagerInfo {}
impl ::core::fmt::Debug for KeyCredentialManagerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KeyCredentialManagerInfo").field("containerId", &self.containerId).finish()
    }
}
impl ::core::default::Default for KeyCredentialManagerOperationErrorStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KeyCredentialManagerOperationErrorStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialManagerOperationErrorStates").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KeyCredentialManagerOperationErrorStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KeyCredentialManagerOperationErrorStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KeyCredentialManagerOperationErrorStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for KeyCredentialManagerOperationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KeyCredentialManagerOperationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialManagerOperationType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARDNAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARDNAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OPENCARDNAME_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OPENCARDNAME_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARD_SEARCH_CRITERIAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENCARD_SEARCH_CRITERIAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for READER_SEL_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for READER_SEL_REQUEST_MATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for READER_SEL_REQUEST_MATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READER_SEL_REQUEST_MATCH_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for READER_SEL_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READER_SEL_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.cbReaderNameOffset == other.cbReaderNameOffset && self.cchReaderNameLength == other.cchReaderNameLength && self.cbCardNameOffset == other.cbCardNameOffset && self.cchCardNameLength == other.cchCardNameLength
    }
}
impl ::core::cmp::Eq for READER_SEL_RESPONSE {}
impl ::core::fmt::Debug for READER_SEL_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READER_SEL_RESPONSE").field("cbReaderNameOffset", &self.cbReaderNameOffset).field("cchReaderNameLength", &self.cchReaderNameLength).field("cbCardNameOffset", &self.cbCardNameOffset).field("cchCardNameLength", &self.cchCardNameLength).finish()
    }
}
impl ::core::default::Default for SCARD_ATRMASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCARD_ATRMASK {
    fn eq(&self, other: &Self) -> bool {
        self.cbAtr == other.cbAtr && self.rgbAtr == other.rgbAtr && self.rgbMask == other.rgbMask
    }
}
impl ::core::cmp::Eq for SCARD_ATRMASK {}
impl ::core::fmt::Debug for SCARD_ATRMASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_ATRMASK").field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).field("rgbMask", &self.rgbMask).finish()
    }
}
impl ::core::default::Default for SCARD_IO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCARD_IO_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dwProtocol == other.dwProtocol && self.cbPciLength == other.cbPciLength
    }
}
impl ::core::cmp::Eq for SCARD_IO_REQUEST {}
impl ::core::fmt::Debug for SCARD_IO_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_IO_REQUEST").field("dwProtocol", &self.dwProtocol).field("cbPciLength", &self.cbPciLength).finish()
    }
}
impl ::core::default::Default for SCARD_READERSTATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCARD_READERSTATEA {
    fn eq(&self, other: &Self) -> bool {
        self.szReader == other.szReader && self.pvUserData == other.pvUserData && self.dwCurrentState == other.dwCurrentState && self.dwEventState == other.dwEventState && self.cbAtr == other.cbAtr && self.rgbAtr == other.rgbAtr
    }
}
impl ::core::cmp::Eq for SCARD_READERSTATEA {}
impl ::core::fmt::Debug for SCARD_READERSTATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_READERSTATEA").field("szReader", &self.szReader).field("pvUserData", &self.pvUserData).field("dwCurrentState", &self.dwCurrentState).field("dwEventState", &self.dwEventState).field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).finish()
    }
}
impl ::core::default::Default for SCARD_READERSTATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCARD_READERSTATEW {
    fn eq(&self, other: &Self) -> bool {
        self.szReader == other.szReader && self.pvUserData == other.pvUserData && self.dwCurrentState == other.dwCurrentState && self.dwEventState == other.dwEventState && self.cbAtr == other.cbAtr && self.rgbAtr == other.rgbAtr
    }
}
impl ::core::cmp::Eq for SCARD_READERSTATEW {}
impl ::core::fmt::Debug for SCARD_READERSTATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_READERSTATEW").field("szReader", &self.szReader).field("pvUserData", &self.pvUserData).field("dwCurrentState", &self.dwCurrentState).field("dwEventState", &self.dwEventState).field("cbAtr", &self.cbAtr).field("rgbAtr", &self.rgbAtr).finish()
    }
}
impl ::core::default::Default for SCARD_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCARD_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCARD_SCOPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCARD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCARD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCARD_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCARD_T0_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCARD_T0_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.bCla == other.bCla && self.bIns == other.bIns && self.bP1 == other.bP1 && self.bP2 == other.bP2 && self.bP3 == other.bP3
    }
}
impl ::core::cmp::Eq for SCARD_T0_COMMAND {}
impl ::core::fmt::Debug for SCARD_T0_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_T0_COMMAND").field("bCla", &self.bCla).field("bIns", &self.bIns).field("bP1", &self.bP1).field("bP2", &self.bP2).field("bP3", &self.bP3).finish()
    }
}
impl ::core::default::Default for SCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SCARD_T1_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCARD_T1_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.ioRequest == other.ioRequest
    }
}
impl ::core::cmp::Eq for SCARD_T1_REQUEST {}
impl ::core::fmt::Debug for SCARD_T1_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCARD_T1_REQUEST").field("ioRequest", &self.ioRequest).finish()
    }
}
impl ::core::default::Default for SecHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecHandle {
    fn eq(&self, other: &Self) -> bool {
        self.dwLower == other.dwLower && self.dwUpper == other.dwUpper
    }
}
impl ::core::cmp::Eq for SecHandle {}
impl ::core::fmt::Debug for SecHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecHandle").field("dwLower", &self.dwLower).field("dwUpper", &self.dwUpper).finish()
    }
}
impl ::core::default::Default for SecPkgContext_ClientCreds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_ClientCreds {
    fn eq(&self, other: &Self) -> bool {
        self.AuthBufferLen == other.AuthBufferLen && self.AuthBuffer == other.AuthBuffer
    }
}
impl ::core::cmp::Eq for SecPkgContext_ClientCreds {}
impl ::core::fmt::Debug for SecPkgContext_ClientCreds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ClientCreds").field("AuthBufferLen", &self.AuthBufferLen).field("AuthBuffer", &self.AuthBuffer).finish()
    }
}
impl ::core::default::Default for USERNAME_TARGET_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USERNAME_TARGET_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName
    }
}
impl ::core::cmp::Eq for USERNAME_TARGET_CREDENTIAL_INFO {}
impl ::core::fmt::Debug for USERNAME_TARGET_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USERNAME_TARGET_CREDENTIAL_INFO").field("UserName", &self.UserName).finish()
    }
}
