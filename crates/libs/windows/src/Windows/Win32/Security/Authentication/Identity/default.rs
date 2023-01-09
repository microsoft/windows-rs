impl ::core::default::Default for ASC_REQ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASC_REQ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASC_REQ_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ASC_REQ_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ASC_REQ_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ASC_REQ_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ASC_REQ_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ASC_REQ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for ASC_REQ_HIGH_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASC_REQ_HIGH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASC_REQ_HIGH_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ASC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ASC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ASC_REQ_HIGH_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ASC_REQ_HIGH_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ASC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for AUDIT_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUDIT_POLICY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AuditSubCategoryGuid == other.AuditSubCategoryGuid && self.AuditingInformation == other.AuditingInformation && self.AuditCategoryGuid == other.AuditCategoryGuid
    }
}
impl ::core::cmp::Eq for AUDIT_POLICY_INFORMATION {}
impl ::core::fmt::Debug for AUDIT_POLICY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_POLICY_INFORMATION").field("AuditSubCategoryGuid", &self.AuditSubCategoryGuid).field("AuditingInformation", &self.AuditingInformation).field("AuditCategoryGuid", &self.AuditCategoryGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CENTRAL_ACCESS_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CENTRAL_ACCESS_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.CAPID == other.CAPID && self.Name == other.Name && self.Description == other.Description && self.ChangeId == other.ChangeId && self.Flags == other.Flags && self.CAPECount == other.CAPECount && self.CAPEs == other.CAPEs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CENTRAL_ACCESS_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CENTRAL_ACCESS_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CENTRAL_ACCESS_POLICY").field("CAPID", &self.CAPID).field("Name", &self.Name).field("Description", &self.Description).field("ChangeId", &self.ChangeId).field("Flags", &self.Flags).field("CAPECount", &self.CAPECount).field("CAPEs", &self.CAPEs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CENTRAL_ACCESS_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CENTRAL_ACCESS_POLICY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Description == other.Description && self.ChangeId == other.ChangeId && self.LengthAppliesTo == other.LengthAppliesTo && self.AppliesTo == other.AppliesTo && self.LengthSD == other.LengthSD && self.SD == other.SD && self.LengthStagedSD == other.LengthStagedSD && self.StagedSD == other.StagedSD && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CENTRAL_ACCESS_POLICY_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CENTRAL_ACCESS_POLICY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CENTRAL_ACCESS_POLICY_ENTRY").field("Name", &self.Name).field("Description", &self.Description).field("ChangeId", &self.ChangeId).field("LengthAppliesTo", &self.LengthAppliesTo).field("AppliesTo", &self.AppliesTo).field("LengthSD", &self.LengthSD).field("SD", &self.SD).field("LengthStagedSD", &self.LengthStagedSD).field("StagedSD", &self.StagedSD).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLEAR_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLEAR_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLEAR_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLEAR_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLEAR_BLOCK").field("data", &self.data).finish()
    }
}
impl ::core::default::Default for CRED_FETCH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRED_FETCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRED_FETCH").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOMAIN_PASSWORD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOMAIN_PASSWORD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MinPasswordLength == other.MinPasswordLength && self.PasswordHistoryLength == other.PasswordHistoryLength && self.PasswordProperties == other.PasswordProperties && self.MaxPasswordAge == other.MaxPasswordAge && self.MinPasswordAge == other.MinPasswordAge
    }
}
impl ::core::cmp::Eq for DOMAIN_PASSWORD_INFORMATION {}
impl ::core::fmt::Debug for DOMAIN_PASSWORD_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAIN_PASSWORD_INFORMATION").field("MinPasswordLength", &self.MinPasswordLength).field("PasswordHistoryLength", &self.PasswordHistoryLength).field("PasswordProperties", &self.PasswordProperties).field("MaxPasswordAge", &self.MaxPasswordAge).field("MinPasswordAge", &self.MinPasswordAge).finish()
    }
}
impl ::core::default::Default for DOMAIN_PASSWORD_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOMAIN_PASSWORD_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOMAIN_PASSWORD_PROPERTIES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DOMAIN_PASSWORD_PROPERTIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DOMAIN_PASSWORD_PROPERTIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DOMAIN_PASSWORD_PROPERTIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DOMAIN_PASSWORD_PROPERTIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DOMAIN_PASSWORD_PROPERTIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::default::Default for ENCRYPTED_CREDENTIALW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::cmp::PartialEq for ENCRYPTED_CREDENTIALW {
    fn eq(&self, other: &Self) -> bool {
        self.Cred == other.Cred && self.ClearCredentialBlobSize == other.ClearCredentialBlobSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::cmp::Eq for ENCRYPTED_CREDENTIALW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::fmt::Debug for ENCRYPTED_CREDENTIALW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTED_CREDENTIALW").field("Cred", &self.Cred).field("ClearCredentialBlobSize", &self.ClearCredentialBlobSize).finish()
    }
}
impl ::core::default::Default for EXPORT_SECURITY_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXPORT_SECURITY_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPORT_SECURITY_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EXPORT_SECURITY_CONTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EXPORT_SECURITY_CONTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EXPORT_SECURITY_CONTEXT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EXPORT_SECURITY_CONTEXT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EXPORT_SECURITY_CONTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for EXTENDED_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXTENDED_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXTENDED_NAME_FORMAT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICcgDomainAuthCredentials {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICcgDomainAuthCredentials {}
impl ::core::fmt::Debug for ICcgDomainAuthCredentials {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICcgDomainAuthCredentials").field(&self.0).finish()
    }
}
impl ::core::default::Default for ISC_REQ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ISC_REQ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISC_REQ_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ISC_REQ_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ISC_REQ_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ISC_REQ_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ISC_REQ_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ISC_REQ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for ISC_REQ_HIGH_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ISC_REQ_HIGH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISC_REQ_HIGH_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ISC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ISC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ISC_REQ_HIGH_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ISC_REQ_HIGH_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ISC_REQ_HIGH_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KDC_PROXY_CACHE_ENTRY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KDC_PROXY_CACHE_ENTRY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.SinceLastUsed == other.SinceLastUsed && self.DomainName == other.DomainName && self.ProxyServerName == other.ProxyServerName && self.ProxyServerVdir == other.ProxyServerVdir && self.ProxyServerPort == other.ProxyServerPort && self.LogonId == other.LogonId && self.CredUserName == other.CredUserName && self.CredDomainName == other.CredDomainName && self.GlobalCache == other.GlobalCache
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KDC_PROXY_CACHE_ENTRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KDC_PROXY_CACHE_ENTRY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KDC_PROXY_CACHE_ENTRY_DATA").field("SinceLastUsed", &self.SinceLastUsed).field("DomainName", &self.DomainName).field("ProxyServerName", &self.ProxyServerName).field("ProxyServerVdir", &self.ProxyServerVdir).field("ProxyServerPort", &self.ProxyServerPort).field("LogonId", &self.LogonId).field("CredUserName", &self.CredUserName).field("CredDomainName", &self.CredDomainName).field("GlobalCache", &self.GlobalCache).finish()
    }
}
impl ::core::default::Default for KERB_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KERB_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KERB_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.RealmName == other.RealmName && self.KdcAddress == other.KdcAddress && self.AddressType == other.AddressType && self.DcFlags == other.DcFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST").field("MessageType", &self.MessageType).field("RealmName", &self.RealmName).field("KdcAddress", &self.KdcAddress).field("AddressType", &self.AddressType).field("DcFlags", &self.DcFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.RealmName == other.RealmName && self.KdcAddress == other.KdcAddress && self.AddressType == other.AddressType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_ADD_BINDING_CACHE_ENTRY_REQUEST").field("MessageType", &self.MessageType).field("RealmName", &self.RealmName).field("KdcAddress", &self.KdcAddress).field("AddressType", &self.AddressType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_ADD_CREDENTIALS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_ADD_CREDENTIALS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.UserName == other.UserName && self.DomainName == other.DomainName && self.Password == other.Password && self.LogonId == other.LogonId && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_ADD_CREDENTIALS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_ADD_CREDENTIALS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_ADD_CREDENTIALS_REQUEST").field("MessageType", &self.MessageType).field("UserName", &self.UserName).field("DomainName", &self.DomainName).field("Password", &self.Password).field("LogonId", &self.LogonId).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_ADD_CREDENTIALS_REQUEST_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_ADD_CREDENTIALS_REQUEST_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Credentials == other.Credentials && self.PrincipalNameCount == other.PrincipalNameCount && self.PrincipalNames == other.PrincipalNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_ADD_CREDENTIALS_REQUEST_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_ADD_CREDENTIALS_REQUEST_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_ADD_CREDENTIALS_REQUEST_EX").field("Credentials", &self.Credentials).field("PrincipalNameCount", &self.PrincipalNameCount).field("PrincipalNames", &self.PrincipalNames).finish()
    }
}
impl ::core::default::Default for KERB_AUTH_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_AUTH_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for KERB_AUTH_DATA {}
impl ::core::fmt::Debug for KERB_AUTH_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_AUTH_DATA").field("Type", &self.Type).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_BINDING_CACHE_ENTRY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_BINDING_CACHE_ENTRY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DiscoveryTime == other.DiscoveryTime && self.RealmName == other.RealmName && self.KdcAddress == other.KdcAddress && self.AddressType == other.AddressType && self.Flags == other.Flags && self.DcFlags == other.DcFlags && self.CacheFlags == other.CacheFlags && self.KdcName == other.KdcName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_BINDING_CACHE_ENTRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_BINDING_CACHE_ENTRY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_BINDING_CACHE_ENTRY_DATA").field("DiscoveryTime", &self.DiscoveryTime).field("RealmName", &self.RealmName).field("KdcAddress", &self.KdcAddress).field("AddressType", &self.AddressType).field("Flags", &self.Flags).field("DcFlags", &self.DcFlags).field("CacheFlags", &self.CacheFlags).field("KdcName", &self.KdcName).finish()
    }
}
impl ::core::default::Default for KERB_CERTIFICATE_HASHINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_CERTIFICATE_HASHINFO {
    fn eq(&self, other: &Self) -> bool {
        self.StoreNameLength == other.StoreNameLength && self.HashLength == other.HashLength
    }
}
impl ::core::cmp::Eq for KERB_CERTIFICATE_HASHINFO {}
impl ::core::fmt::Debug for KERB_CERTIFICATE_HASHINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CERTIFICATE_HASHINFO").field("StoreNameLength", &self.StoreNameLength).field("HashLength", &self.HashLength).finish()
    }
}
impl ::core::default::Default for KERB_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_CERTIFICATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CertInfoSize == other.CertInfoSize && self.InfoType == other.InfoType
    }
}
impl ::core::cmp::Eq for KERB_CERTIFICATE_INFO {}
impl ::core::fmt::Debug for KERB_CERTIFICATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CERTIFICATE_INFO").field("CertInfoSize", &self.CertInfoSize).field("InfoType", &self.InfoType).finish()
    }
}
impl ::core::default::Default for KERB_CERTIFICATE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KERB_CERTIFICATE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KERB_CERTIFICATE_INFO_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_CERTIFICATE_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_CERTIFICATE_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.DomainName == other.DomainName && self.UserName == other.UserName && self.Pin == other.Pin && self.Flags == other.Flags && self.CspDataLength == other.CspDataLength && self.CspData == other.CspData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_CERTIFICATE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_CERTIFICATE_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CERTIFICATE_LOGON").field("MessageType", &self.MessageType).field("DomainName", &self.DomainName).field("UserName", &self.UserName).field("Pin", &self.Pin).field("Flags", &self.Flags).field("CspDataLength", &self.CspDataLength).field("CspData", &self.CspData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_CERTIFICATE_S4U_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_CERTIFICATE_S4U_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.UserPrincipalName == other.UserPrincipalName && self.DomainName == other.DomainName && self.CertificateLength == other.CertificateLength && self.Certificate == other.Certificate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_CERTIFICATE_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_CERTIFICATE_S4U_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CERTIFICATE_S4U_LOGON").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("UserPrincipalName", &self.UserPrincipalName).field("DomainName", &self.DomainName).field("CertificateLength", &self.CertificateLength).field("Certificate", &self.Certificate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_CERTIFICATE_UNLOCK_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_CERTIFICATE_UNLOCK_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.Logon == other.Logon && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_CERTIFICATE_UNLOCK_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_CERTIFICATE_UNLOCK_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CERTIFICATE_UNLOCK_LOGON").field("Logon", &self.Logon).field("LogonId", &self.LogonId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_CHANGEPASSWORD_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_CHANGEPASSWORD_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.DomainName == other.DomainName && self.AccountName == other.AccountName && self.OldPassword == other.OldPassword && self.NewPassword == other.NewPassword && self.Impersonating == other.Impersonating
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_CHANGEPASSWORD_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_CHANGEPASSWORD_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CHANGEPASSWORD_REQUEST").field("MessageType", &self.MessageType).field("DomainName", &self.DomainName).field("AccountName", &self.AccountName).field("OldPassword", &self.OldPassword).field("NewPassword", &self.NewPassword).field("Impersonating", &self.Impersonating).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).finish()
    }
}
impl ::core::default::Default for KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {}
impl ::core::fmt::Debug for KERB_CLOUD_KERBEROS_DEBUG_DATA_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CLOUD_KERBEROS_DEBUG_DATA_V0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_CLOUD_KERBEROS_DEBUG_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_CLOUD_KERBEROS_DEBUG_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_CLOUD_KERBEROS_DEBUG_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_CLOUD_KERBEROS_DEBUG_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CLOUD_KERBEROS_DEBUG_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).finish()
    }
}
impl ::core::default::Default for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Version == other.Version && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {}
impl ::core::fmt::Debug for KERB_CLOUD_KERBEROS_DEBUG_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CLOUD_KERBEROS_DEBUG_RESPONSE").field("MessageType", &self.MessageType).field("Version", &self.Version).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for KERB_CRYPTO_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_CRYPTO_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.KeyType == other.KeyType && self.Length == other.Length && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for KERB_CRYPTO_KEY {}
impl ::core::fmt::Debug for KERB_CRYPTO_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CRYPTO_KEY").field("KeyType", &self.KeyType).field("Length", &self.Length).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for KERB_CRYPTO_KEY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_CRYPTO_KEY32 {
    fn eq(&self, other: &Self) -> bool {
        self.KeyType == other.KeyType && self.Length == other.Length && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for KERB_CRYPTO_KEY32 {}
impl ::core::fmt::Debug for KERB_CRYPTO_KEY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_CRYPTO_KEY32").field("KeyType", &self.KeyType).field("Length", &self.Length).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for KERB_CRYPTO_KEY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KERB_CRYPTO_KEY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KERB_CRYPTO_KEY_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_DECRYPT_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_DECRYPT_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId && self.Flags == other.Flags && self.CryptoType == other.CryptoType && self.KeyUsage == other.KeyUsage && self.Key == other.Key && self.EncryptedDataSize == other.EncryptedDataSize && self.InitialVectorSize == other.InitialVectorSize && self.InitialVector == other.InitialVector && self.EncryptedData == other.EncryptedData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_DECRYPT_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_DECRYPT_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_DECRYPT_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).field("Flags", &self.Flags).field("CryptoType", &self.CryptoType).field("KeyUsage", &self.KeyUsage).field("Key", &self.Key).field("EncryptedDataSize", &self.EncryptedDataSize).field("InitialVectorSize", &self.InitialVectorSize).field("InitialVector", &self.InitialVector).field("EncryptedData", &self.EncryptedData).finish()
    }
}
impl ::core::default::Default for KERB_DECRYPT_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_DECRYPT_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.DecryptedData == other.DecryptedData
    }
}
impl ::core::cmp::Eq for KERB_DECRYPT_RESPONSE {}
impl ::core::fmt::Debug for KERB_DECRYPT_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_DECRYPT_RESPONSE").field("DecryptedData", &self.DecryptedData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_EXTERNAL_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_EXTERNAL_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.NameType == other.NameType && self.NameCount == other.NameCount && self.Names == other.Names
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_EXTERNAL_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_EXTERNAL_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_EXTERNAL_NAME").field("NameType", &self.NameType).field("NameCount", &self.NameCount).field("Names", &self.Names).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_EXTERNAL_TICKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_EXTERNAL_TICKET {
    fn eq(&self, other: &Self) -> bool {
        self.ServiceName == other.ServiceName && self.TargetName == other.TargetName && self.ClientName == other.ClientName && self.DomainName == other.DomainName && self.TargetDomainName == other.TargetDomainName && self.AltTargetDomainName == other.AltTargetDomainName && self.SessionKey == other.SessionKey && self.TicketFlags == other.TicketFlags && self.Flags == other.Flags && self.KeyExpirationTime == other.KeyExpirationTime && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.RenewUntil == other.RenewUntil && self.TimeSkew == other.TimeSkew && self.EncodedTicketSize == other.EncodedTicketSize && self.EncodedTicket == other.EncodedTicket
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_EXTERNAL_TICKET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_EXTERNAL_TICKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_EXTERNAL_TICKET")
            .field("ServiceName", &self.ServiceName)
            .field("TargetName", &self.TargetName)
            .field("ClientName", &self.ClientName)
            .field("DomainName", &self.DomainName)
            .field("TargetDomainName", &self.TargetDomainName)
            .field("AltTargetDomainName", &self.AltTargetDomainName)
            .field("SessionKey", &self.SessionKey)
            .field("TicketFlags", &self.TicketFlags)
            .field("Flags", &self.Flags)
            .field("KeyExpirationTime", &self.KeyExpirationTime)
            .field("StartTime", &self.StartTime)
            .field("EndTime", &self.EndTime)
            .field("RenewUntil", &self.RenewUntil)
            .field("TimeSkew", &self.TimeSkew)
            .field("EncodedTicketSize", &self.EncodedTicketSize)
            .field("EncodedTicket", &self.EncodedTicket)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_INTERACTIVE_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_INTERACTIVE_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonDomainName == other.LogonDomainName && self.UserName == other.UserName && self.Password == other.Password
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_INTERACTIVE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_INTERACTIVE_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_INTERACTIVE_LOGON").field("MessageType", &self.MessageType).field("LogonDomainName", &self.LogonDomainName).field("UserName", &self.UserName).field("Password", &self.Password).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_INTERACTIVE_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_INTERACTIVE_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonCount == other.LogonCount && self.BadPasswordCount == other.BadPasswordCount && self.LogonTime == other.LogonTime && self.LogoffTime == other.LogoffTime && self.KickOffTime == other.KickOffTime && self.PasswordLastSet == other.PasswordLastSet && self.PasswordCanChange == other.PasswordCanChange && self.PasswordMustChange == other.PasswordMustChange && self.LogonScript == other.LogonScript && self.HomeDirectory == other.HomeDirectory && self.FullName == other.FullName && self.ProfilePath == other.ProfilePath && self.HomeDirectoryDrive == other.HomeDirectoryDrive && self.LogonServer == other.LogonServer && self.UserFlags == other.UserFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_INTERACTIVE_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_INTERACTIVE_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_INTERACTIVE_PROFILE")
            .field("MessageType", &self.MessageType)
            .field("LogonCount", &self.LogonCount)
            .field("BadPasswordCount", &self.BadPasswordCount)
            .field("LogonTime", &self.LogonTime)
            .field("LogoffTime", &self.LogoffTime)
            .field("KickOffTime", &self.KickOffTime)
            .field("PasswordLastSet", &self.PasswordLastSet)
            .field("PasswordCanChange", &self.PasswordCanChange)
            .field("PasswordMustChange", &self.PasswordMustChange)
            .field("LogonScript", &self.LogonScript)
            .field("HomeDirectory", &self.HomeDirectory)
            .field("FullName", &self.FullName)
            .field("ProfilePath", &self.ProfilePath)
            .field("HomeDirectoryDrive", &self.HomeDirectoryDrive)
            .field("LogonServer", &self.LogonServer)
            .field("UserFlags", &self.UserFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_INTERACTIVE_UNLOCK_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_INTERACTIVE_UNLOCK_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.Logon == other.Logon && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_INTERACTIVE_UNLOCK_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_INTERACTIVE_UNLOCK_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_INTERACTIVE_UNLOCK_LOGON").field("Logon", &self.Logon).field("LogonId", &self.LogonId).finish()
    }
}
impl ::core::default::Default for KERB_LOGON_SUBMIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KERB_LOGON_SUBMIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KERB_LOGON_SUBMIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KERB_NET_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_NET_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Family == other.Family && self.Length == other.Length && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for KERB_NET_ADDRESS {}
impl ::core::fmt::Debug for KERB_NET_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_NET_ADDRESS").field("Family", &self.Family).field("Length", &self.Length).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for KERB_NET_ADDRESSES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_NET_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.Addresses == other.Addresses
    }
}
impl ::core::cmp::Eq for KERB_NET_ADDRESSES {}
impl ::core::fmt::Debug for KERB_NET_ADDRESSES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_NET_ADDRESSES").field("Number", &self.Number).field("Addresses", &self.Addresses).finish()
    }
}
impl ::core::default::Default for KERB_PROFILE_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KERB_PROFILE_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KERB_PROFILE_BUFFER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KERB_PROTOCOL_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KERB_PROTOCOL_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KERB_PROTOCOL_MESSAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KERB_PURGE_BINDING_CACHE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_PURGE_BINDING_CACHE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType
    }
}
impl ::core::cmp::Eq for KERB_PURGE_BINDING_CACHE_REQUEST {}
impl ::core::fmt::Debug for KERB_PURGE_BINDING_CACHE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_PURGE_BINDING_CACHE_REQUEST").field("MessageType", &self.MessageType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_PURGE_KDC_PROXY_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_PURGE_KDC_PROXY_CACHE_REQUEST").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("LogonId", &self.LogonId).finish()
    }
}
impl ::core::default::Default for KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CountOfPurged == other.CountOfPurged
    }
}
impl ::core::cmp::Eq for KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {}
impl ::core::fmt::Debug for KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_PURGE_KDC_PROXY_CACHE_RESPONSE").field("MessageType", &self.MessageType).field("CountOfPurged", &self.CountOfPurged).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_PURGE_TKT_CACHE_EX_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_PURGE_TKT_CACHE_EX_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId && self.Flags == other.Flags && self.TicketTemplate == other.TicketTemplate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_PURGE_TKT_CACHE_EX_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_PURGE_TKT_CACHE_EX_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_PURGE_TKT_CACHE_EX_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).field("Flags", &self.Flags).field("TicketTemplate", &self.TicketTemplate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_PURGE_TKT_CACHE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_PURGE_TKT_CACHE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId && self.ServerName == other.ServerName && self.RealmName == other.RealmName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_PURGE_TKT_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_PURGE_TKT_CACHE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_PURGE_TKT_CACHE_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).field("ServerName", &self.ServerName).field("RealmName", &self.RealmName).finish()
    }
}
impl ::core::default::Default for KERB_QUERY_BINDING_CACHE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_QUERY_BINDING_CACHE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType
    }
}
impl ::core::cmp::Eq for KERB_QUERY_BINDING_CACHE_REQUEST {}
impl ::core::fmt::Debug for KERB_QUERY_BINDING_CACHE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_BINDING_CACHE_REQUEST").field("MessageType", &self.MessageType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_BINDING_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_BINDING_CACHE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CountOfEntries == other.CountOfEntries && self.Entries == other.Entries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_BINDING_CACHE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_BINDING_CACHE_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_BINDING_CACHE_RESPONSE").field("MessageType", &self.MessageType).field("CountOfEntries", &self.CountOfEntries).field("Entries", &self.Entries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.DomainName == other.DomainName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("DomainName", &self.DomainName).finish()
    }
}
impl ::core::default::Default for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.ExtendedPolicies == other.ExtendedPolicies && self.DsFlags == other.DsFlags
    }
}
impl ::core::cmp::Eq for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {}
impl ::core::fmt::Debug for KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("ExtendedPolicies", &self.ExtendedPolicies).field("DsFlags", &self.DsFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_KDC_PROXY_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_KDC_PROXY_CACHE_REQUEST").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("LogonId", &self.LogonId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CountOfEntries == other.CountOfEntries && self.Entries == other.Entries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_KDC_PROXY_CACHE_RESPONSE").field("MessageType", &self.MessageType).field("CountOfEntries", &self.CountOfEntries).field("Entries", &self.Entries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_S4U2PROXY_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_S4U2PROXY_CACHE_REQUEST").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("LogonId", &self.LogonId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CountOfCreds == other.CountOfCreds && self.Creds == other.Creds
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_S4U2PROXY_CACHE_RESPONSE").field("MessageType", &self.MessageType).field("CountOfCreds", &self.CountOfCreds).field("Creds", &self.Creds).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CountOfTickets == other.CountOfTickets && self.Tickets == other.Tickets
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_TKT_CACHE_EX2_RESPONSE").field("MessageType", &self.MessageType).field("CountOfTickets", &self.CountOfTickets).field("Tickets", &self.Tickets).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CountOfTickets == other.CountOfTickets && self.Tickets == other.Tickets
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_TKT_CACHE_EX3_RESPONSE").field("MessageType", &self.MessageType).field("CountOfTickets", &self.CountOfTickets).field("Tickets", &self.Tickets).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CountOfTickets == other.CountOfTickets && self.Tickets == other.Tickets
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_TKT_CACHE_EX_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_TKT_CACHE_EX_RESPONSE").field("MessageType", &self.MessageType).field("CountOfTickets", &self.CountOfTickets).field("Tickets", &self.Tickets).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_TKT_CACHE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_TKT_CACHE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_TKT_CACHE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_TKT_CACHE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_TKT_CACHE_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_QUERY_TKT_CACHE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_QUERY_TKT_CACHE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CountOfTickets == other.CountOfTickets && self.Tickets == other.Tickets
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_QUERY_TKT_CACHE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_QUERY_TKT_CACHE_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_QUERY_TKT_CACHE_RESPONSE").field("MessageType", &self.MessageType).field("CountOfTickets", &self.CountOfTickets).field("Tickets", &self.Tickets).finish()
    }
}
impl ::core::default::Default for KERB_REFRESH_POLICY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_REFRESH_POLICY_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KERB_REFRESH_POLICY_REQUEST {}
impl ::core::fmt::Debug for KERB_REFRESH_POLICY_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_REFRESH_POLICY_REQUEST").field("MessageType", &self.MessageType).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for KERB_REFRESH_POLICY_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_REFRESH_POLICY_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KERB_REFRESH_POLICY_RESPONSE {}
impl ::core::fmt::Debug for KERB_REFRESH_POLICY_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_REFRESH_POLICY_RESPONSE").field("MessageType", &self.MessageType).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_REFRESH_SCCRED_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_REFRESH_SCCRED_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CredentialBlob == other.CredentialBlob && self.LogonId == other.LogonId && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_REFRESH_SCCRED_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_REFRESH_SCCRED_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_REFRESH_SCCRED_REQUEST").field("MessageType", &self.MessageType).field("CredentialBlob", &self.CredentialBlob).field("LogonId", &self.LogonId).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for KERB_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KERB_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KERB_REQUEST_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_RETRIEVE_KEY_TAB_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_RETRIEVE_KEY_TAB_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.UserName == other.UserName && self.DomainName == other.DomainName && self.Password == other.Password
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_RETRIEVE_KEY_TAB_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_RETRIEVE_KEY_TAB_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_RETRIEVE_KEY_TAB_REQUEST").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("UserName", &self.UserName).field("DomainName", &self.DomainName).field("Password", &self.Password).finish()
    }
}
impl ::core::default::Default for KERB_RETRIEVE_KEY_TAB_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_RETRIEVE_KEY_TAB_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.KeyTabLength == other.KeyTabLength && self.KeyTab == other.KeyTab
    }
}
impl ::core::cmp::Eq for KERB_RETRIEVE_KEY_TAB_RESPONSE {}
impl ::core::fmt::Debug for KERB_RETRIEVE_KEY_TAB_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_RETRIEVE_KEY_TAB_RESPONSE").field("MessageType", &self.MessageType).field("KeyTabLength", &self.KeyTabLength).field("KeyTab", &self.KeyTab).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::default::Default for KERB_RETRIEVE_TKT_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::cmp::PartialEq for KERB_RETRIEVE_TKT_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId && self.TargetName == other.TargetName && self.TicketFlags == other.TicketFlags && self.CacheOptions == other.CacheOptions && self.EncryptionType == other.EncryptionType && self.CredentialsHandle == other.CredentialsHandle
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::cmp::Eq for KERB_RETRIEVE_TKT_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::fmt::Debug for KERB_RETRIEVE_TKT_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_RETRIEVE_TKT_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).field("TargetName", &self.TargetName).field("TicketFlags", &self.TicketFlags).field("CacheOptions", &self.CacheOptions).field("EncryptionType", &self.EncryptionType).field("CredentialsHandle", &self.CredentialsHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_RETRIEVE_TKT_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_RETRIEVE_TKT_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.Ticket == other.Ticket
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_RETRIEVE_TKT_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_RETRIEVE_TKT_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_RETRIEVE_TKT_RESPONSE").field("Ticket", &self.Ticket).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ServerName == other.ServerName && self.Flags == other.Flags && self.LastStatus == other.LastStatus && self.Expiry == other.Expiry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_S4U2PROXY_CACHE_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_S4U2PROXY_CACHE_ENTRY_INFO").field("ServerName", &self.ServerName).field("Flags", &self.Flags).field("LastStatus", &self.LastStatus).field("Expiry", &self.Expiry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_S4U2PROXY_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_S4U2PROXY_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.DomainName == other.DomainName && self.Flags == other.Flags && self.LastStatus == other.LastStatus && self.Expiry == other.Expiry && self.CountOfEntries == other.CountOfEntries && self.Entries == other.Entries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_S4U2PROXY_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_S4U2PROXY_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_S4U2PROXY_CRED").field("UserName", &self.UserName).field("DomainName", &self.DomainName).field("Flags", &self.Flags).field("LastStatus", &self.LastStatus).field("Expiry", &self.Expiry).field("CountOfEntries", &self.CountOfEntries).field("Entries", &self.Entries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_S4U_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_S4U_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.ClientUpn == other.ClientUpn && self.ClientRealm == other.ClientRealm
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_S4U_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_S4U_LOGON").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("ClientUpn", &self.ClientUpn).field("ClientRealm", &self.ClientRealm).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::default::Default for KERB_SETPASSWORD_EX_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::cmp::PartialEq for KERB_SETPASSWORD_EX_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId && self.CredentialsHandle == other.CredentialsHandle && self.Flags == other.Flags && self.AccountRealm == other.AccountRealm && self.AccountName == other.AccountName && self.Password == other.Password && self.ClientRealm == other.ClientRealm && self.ClientName == other.ClientName && self.Impersonating == other.Impersonating && self.KdcAddress == other.KdcAddress && self.KdcAddressType == other.KdcAddressType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::cmp::Eq for KERB_SETPASSWORD_EX_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::fmt::Debug for KERB_SETPASSWORD_EX_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_SETPASSWORD_EX_REQUEST")
            .field("MessageType", &self.MessageType)
            .field("LogonId", &self.LogonId)
            .field("CredentialsHandle", &self.CredentialsHandle)
            .field("Flags", &self.Flags)
            .field("AccountRealm", &self.AccountRealm)
            .field("AccountName", &self.AccountName)
            .field("Password", &self.Password)
            .field("ClientRealm", &self.ClientRealm)
            .field("ClientName", &self.ClientName)
            .field("Impersonating", &self.Impersonating)
            .field("KdcAddress", &self.KdcAddress)
            .field("KdcAddressType", &self.KdcAddressType)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::default::Default for KERB_SETPASSWORD_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::cmp::PartialEq for KERB_SETPASSWORD_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId && self.CredentialsHandle == other.CredentialsHandle && self.Flags == other.Flags && self.DomainName == other.DomainName && self.AccountName == other.AccountName && self.Password == other.Password
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::cmp::Eq for KERB_SETPASSWORD_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::fmt::Debug for KERB_SETPASSWORD_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_SETPASSWORD_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).field("CredentialsHandle", &self.CredentialsHandle).field("Flags", &self.Flags).field("DomainName", &self.DomainName).field("AccountName", &self.AccountName).field("Password", &self.Password).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_SMART_CARD_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_SMART_CARD_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Pin == other.Pin && self.CspDataLength == other.CspDataLength && self.CspData == other.CspData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_SMART_CARD_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_SMART_CARD_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_SMART_CARD_LOGON").field("MessageType", &self.MessageType).field("Pin", &self.Pin).field("CspDataLength", &self.CspDataLength).field("CspData", &self.CspData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_SMART_CARD_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_SMART_CARD_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.Profile == other.Profile && self.CertificateSize == other.CertificateSize && self.CertificateData == other.CertificateData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_SMART_CARD_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_SMART_CARD_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_SMART_CARD_PROFILE").field("Profile", &self.Profile).field("CertificateSize", &self.CertificateSize).field("CertificateData", &self.CertificateData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_SMART_CARD_UNLOCK_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_SMART_CARD_UNLOCK_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.Logon == other.Logon && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_SMART_CARD_UNLOCK_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_SMART_CARD_UNLOCK_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_SMART_CARD_UNLOCK_LOGON").field("Logon", &self.Logon).field("LogonId", &self.LogonId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_SUBMIT_TKT_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_SUBMIT_TKT_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId && self.Flags == other.Flags && self.Key == other.Key && self.KerbCredSize == other.KerbCredSize && self.KerbCredOffset == other.KerbCredOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_SUBMIT_TKT_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_SUBMIT_TKT_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_SUBMIT_TKT_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).field("Flags", &self.Flags).field("Key", &self.Key).field("KerbCredSize", &self.KerbCredSize).field("KerbCredOffset", &self.KerbCredOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_TICKET_CACHE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_TICKET_CACHE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ServerName == other.ServerName && self.RealmName == other.RealmName && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.RenewTime == other.RenewTime && self.EncryptionType == other.EncryptionType && self.TicketFlags == other.TicketFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_TICKET_CACHE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_TICKET_CACHE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_TICKET_CACHE_INFO").field("ServerName", &self.ServerName).field("RealmName", &self.RealmName).field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("RenewTime", &self.RenewTime).field("EncryptionType", &self.EncryptionType).field("TicketFlags", &self.TicketFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_TICKET_CACHE_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_TICKET_CACHE_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName && self.ClientRealm == other.ClientRealm && self.ServerName == other.ServerName && self.ServerRealm == other.ServerRealm && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.RenewTime == other.RenewTime && self.EncryptionType == other.EncryptionType && self.TicketFlags == other.TicketFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_TICKET_CACHE_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_TICKET_CACHE_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_TICKET_CACHE_INFO_EX").field("ClientName", &self.ClientName).field("ClientRealm", &self.ClientRealm).field("ServerName", &self.ServerName).field("ServerRealm", &self.ServerRealm).field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("RenewTime", &self.RenewTime).field("EncryptionType", &self.EncryptionType).field("TicketFlags", &self.TicketFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_TICKET_CACHE_INFO_EX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_TICKET_CACHE_INFO_EX2 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName && self.ClientRealm == other.ClientRealm && self.ServerName == other.ServerName && self.ServerRealm == other.ServerRealm && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.RenewTime == other.RenewTime && self.EncryptionType == other.EncryptionType && self.TicketFlags == other.TicketFlags && self.SessionKeyType == other.SessionKeyType && self.BranchId == other.BranchId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_TICKET_CACHE_INFO_EX2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_TICKET_CACHE_INFO_EX2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_TICKET_CACHE_INFO_EX2").field("ClientName", &self.ClientName).field("ClientRealm", &self.ClientRealm).field("ServerName", &self.ServerName).field("ServerRealm", &self.ServerRealm).field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("RenewTime", &self.RenewTime).field("EncryptionType", &self.EncryptionType).field("TicketFlags", &self.TicketFlags).field("SessionKeyType", &self.SessionKeyType).field("BranchId", &self.BranchId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_TICKET_CACHE_INFO_EX3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_TICKET_CACHE_INFO_EX3 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName && self.ClientRealm == other.ClientRealm && self.ServerName == other.ServerName && self.ServerRealm == other.ServerRealm && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.RenewTime == other.RenewTime && self.EncryptionType == other.EncryptionType && self.TicketFlags == other.TicketFlags && self.SessionKeyType == other.SessionKeyType && self.BranchId == other.BranchId && self.CacheFlags == other.CacheFlags && self.KdcCalled == other.KdcCalled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_TICKET_CACHE_INFO_EX3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_TICKET_CACHE_INFO_EX3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_TICKET_CACHE_INFO_EX3")
            .field("ClientName", &self.ClientName)
            .field("ClientRealm", &self.ClientRealm)
            .field("ServerName", &self.ServerName)
            .field("ServerRealm", &self.ServerRealm)
            .field("StartTime", &self.StartTime)
            .field("EndTime", &self.EndTime)
            .field("RenewTime", &self.RenewTime)
            .field("EncryptionType", &self.EncryptionType)
            .field("TicketFlags", &self.TicketFlags)
            .field("SessionKeyType", &self.SessionKeyType)
            .field("BranchId", &self.BranchId)
            .field("CacheFlags", &self.CacheFlags)
            .field("KdcCalled", &self.KdcCalled)
            .finish()
    }
}
impl ::core::default::Default for KERB_TICKET_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KERB_TICKET_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KERB_TICKET_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for KERB_TICKET_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KERB_TICKET_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KERB_TICKET_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KERB_TICKET_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KERB_TICKET_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for KERB_TICKET_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERB_TICKET_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.ServiceTicketLength == other.ServiceTicketLength && self.TicketGrantingTicketLength == other.TicketGrantingTicketLength && self.ServiceTicket == other.ServiceTicket && self.TicketGrantingTicket == other.TicketGrantingTicket
    }
}
impl ::core::cmp::Eq for KERB_TICKET_LOGON {}
impl ::core::fmt::Debug for KERB_TICKET_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_TICKET_LOGON").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("ServiceTicketLength", &self.ServiceTicketLength).field("TicketGrantingTicketLength", &self.TicketGrantingTicketLength).field("ServiceTicket", &self.ServiceTicket).field("TicketGrantingTicket", &self.TicketGrantingTicket).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_TICKET_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_TICKET_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.Profile == other.Profile && self.SessionKey == other.SessionKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_TICKET_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_TICKET_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_TICKET_PROFILE").field("Profile", &self.Profile).field("SessionKey", &self.SessionKey).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_TICKET_UNLOCK_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_TICKET_UNLOCK_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.Logon == other.Logon && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_TICKET_UNLOCK_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_TICKET_UNLOCK_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_TICKET_UNLOCK_LOGON").field("Logon", &self.Logon).field("LogonId", &self.LogonId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KERB_TRANSFER_CRED_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KERB_TRANSFER_CRED_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.OriginLogonId == other.OriginLogonId && self.DestinationLogonId == other.DestinationLogonId && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KERB_TRANSFER_CRED_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KERB_TRANSFER_CRED_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERB_TRANSFER_CRED_REQUEST").field("MessageType", &self.MessageType).field("OriginLogonId", &self.OriginLogonId).field("DestinationLogonId", &self.DestinationLogonId).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for KSEC_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEC_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEC_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for KSEC_LIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for KSEC_LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.List == other.List && self.RefCount == other.RefCount && self.Signature == other.Signature && self.OwningList == other.OwningList && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for KSEC_LIST_ENTRY {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for KSEC_LIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSEC_LIST_ENTRY").field("List", &self.List).field("RefCount", &self.RefCount).field("Signature", &self.Signature).field("OwningList", &self.OwningList).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for LOGON_HOURS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOGON_HOURS {
    fn eq(&self, other: &Self) -> bool {
        self.UnitsPerWeek == other.UnitsPerWeek && self.LogonHours == other.LogonHours
    }
}
impl ::core::cmp::Eq for LOGON_HOURS {}
impl ::core::fmt::Debug for LOGON_HOURS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGON_HOURS").field("UnitsPerWeek", &self.UnitsPerWeek).field("LogonHours", &self.LogonHours).finish()
    }
}
impl ::core::default::Default for LSA_AUTH_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LSA_AUTH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LastUpdateTime == other.LastUpdateTime && self.AuthType == other.AuthType && self.AuthInfoLength == other.AuthInfoLength && self.AuthInfo == other.AuthInfo
    }
}
impl ::core::cmp::Eq for LSA_AUTH_INFORMATION {}
impl ::core::fmt::Debug for LSA_AUTH_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_AUTH_INFORMATION").field("LastUpdateTime", &self.LastUpdateTime).field("AuthType", &self.AuthType).field("AuthInfoLength", &self.AuthInfoLength).field("AuthInfo", &self.AuthInfo).finish()
    }
}
impl ::core::default::Default for LSA_AUTH_INFORMATION_AUTH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LSA_AUTH_INFORMATION_AUTH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LSA_AUTH_INFORMATION_AUTH_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for LSA_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_ENUMERATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_ENUMERATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Sid == other.Sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_ENUMERATION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_ENUMERATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_ENUMERATION_INFORMATION").field("Sid", &self.Sid).finish()
    }
}
impl ::core::default::Default for LSA_FOREST_TRUST_BINARY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LSA_FOREST_TRUST_BINARY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for LSA_FOREST_TRUST_BINARY_DATA {}
impl ::core::fmt::Debug for LSA_FOREST_TRUST_BINARY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_FOREST_TRUST_BINARY_DATA").field("Length", &self.Length).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_FOREST_TRUST_COLLISION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_FOREST_TRUST_COLLISION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.RecordCount == other.RecordCount && self.Entries == other.Entries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_FOREST_TRUST_COLLISION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_FOREST_TRUST_COLLISION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_FOREST_TRUST_COLLISION_INFORMATION").field("RecordCount", &self.RecordCount).field("Entries", &self.Entries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_FOREST_TRUST_COLLISION_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_FOREST_TRUST_COLLISION_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.Type == other.Type && self.Flags == other.Flags && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_FOREST_TRUST_COLLISION_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_FOREST_TRUST_COLLISION_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_FOREST_TRUST_COLLISION_RECORD").field("Index", &self.Index).field("Type", &self.Type).field("Flags", &self.Flags).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for LSA_FOREST_TRUST_COLLISION_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LSA_FOREST_TRUST_COLLISION_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LSA_FOREST_TRUST_COLLISION_RECORD_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_FOREST_TRUST_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_FOREST_TRUST_DOMAIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Sid == other.Sid && self.DnsName == other.DnsName && self.NetbiosName == other.NetbiosName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_FOREST_TRUST_DOMAIN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_FOREST_TRUST_DOMAIN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_FOREST_TRUST_DOMAIN_INFO").field("Sid", &self.Sid).field("DnsName", &self.DnsName).field("NetbiosName", &self.NetbiosName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_FOREST_TRUST_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_FOREST_TRUST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.RecordCount == other.RecordCount && self.Entries == other.Entries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_FOREST_TRUST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_FOREST_TRUST_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_FOREST_TRUST_INFORMATION").field("RecordCount", &self.RecordCount).field("Entries", &self.Entries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_FOREST_TRUST_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LSA_FOREST_TRUST_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LSA_FOREST_TRUST_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LSA_FOREST_TRUST_RECORD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for LSA_LAST_INTER_LOGON_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LSA_LAST_INTER_LOGON_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastSuccessfulLogon == other.LastSuccessfulLogon && self.LastFailedLogon == other.LastFailedLogon && self.FailedAttemptCountSinceLastSuccessfulLogon == other.FailedAttemptCountSinceLastSuccessfulLogon
    }
}
impl ::core::cmp::Eq for LSA_LAST_INTER_LOGON_INFO {}
impl ::core::fmt::Debug for LSA_LAST_INTER_LOGON_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_LAST_INTER_LOGON_INFO").field("LastSuccessfulLogon", &self.LastSuccessfulLogon).field("LastFailedLogon", &self.LastFailedLogon).field("FailedAttemptCountSinceLastSuccessfulLogon", &self.FailedAttemptCountSinceLastSuccessfulLogon).finish()
    }
}
impl ::core::default::Default for LSA_LOOKUP_DOMAIN_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LSA_LOOKUP_DOMAIN_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LSA_LOOKUP_DOMAIN_INFO_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_REFERENCED_DOMAIN_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_REFERENCED_DOMAIN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Entries == other.Entries && self.Domains == other.Domains
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_REFERENCED_DOMAIN_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_REFERENCED_DOMAIN_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_REFERENCED_DOMAIN_LIST").field("Entries", &self.Entries).field("Domains", &self.Domains).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
impl ::core::default::Default for LSA_SECPKG_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_TOKEN_INFORMATION_NULL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_TOKEN_INFORMATION_NULL {
    fn eq(&self, other: &Self) -> bool {
        self.ExpirationTime == other.ExpirationTime && self.Groups == other.Groups
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_TOKEN_INFORMATION_NULL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_TOKEN_INFORMATION_NULL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_TOKEN_INFORMATION_NULL").field("ExpirationTime", &self.ExpirationTime).field("Groups", &self.Groups).finish()
    }
}
impl ::core::default::Default for LSA_TOKEN_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LSA_TOKEN_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LSA_TOKEN_INFORMATION_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_TOKEN_INFORMATION_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_TOKEN_INFORMATION_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.ExpirationTime == other.ExpirationTime && self.User == other.User && self.Groups == other.Groups && self.PrimaryGroup == other.PrimaryGroup && self.Privileges == other.Privileges && self.Owner == other.Owner && self.DefaultDacl == other.DefaultDacl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_TOKEN_INFORMATION_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_TOKEN_INFORMATION_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_TOKEN_INFORMATION_V1").field("ExpirationTime", &self.ExpirationTime).field("User", &self.User).field("Groups", &self.Groups).field("PrimaryGroup", &self.PrimaryGroup).field("Privileges", &self.Privileges).field("Owner", &self.Owner).field("DefaultDacl", &self.DefaultDacl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_TOKEN_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_TOKEN_INFORMATION_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.ExpirationTime == other.ExpirationTime && self.User == other.User && self.Groups == other.Groups && self.PrimaryGroup == other.PrimaryGroup && self.Privileges == other.Privileges && self.Owner == other.Owner && self.DefaultDacl == other.DefaultDacl && self.UserClaims == other.UserClaims && self.DeviceClaims == other.DeviceClaims && self.DeviceGroups == other.DeviceGroups
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_TOKEN_INFORMATION_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_TOKEN_INFORMATION_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_TOKEN_INFORMATION_V3").field("ExpirationTime", &self.ExpirationTime).field("User", &self.User).field("Groups", &self.Groups).field("PrimaryGroup", &self.PrimaryGroup).field("Privileges", &self.Privileges).field("Owner", &self.Owner).field("DefaultDacl", &self.DefaultDacl).field("UserClaims", &self.UserClaims).field("DeviceClaims", &self.DeviceClaims).field("DeviceGroups", &self.DeviceGroups).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_TRANSLATED_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_TRANSLATED_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Use == other.Use && self.Name == other.Name && self.DomainIndex == other.DomainIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_TRANSLATED_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_TRANSLATED_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_TRANSLATED_NAME").field("Use", &self.Use).field("Name", &self.Name).field("DomainIndex", &self.DomainIndex).finish()
    }
}
impl ::core::default::Default for LSA_TRANSLATED_SID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LSA_TRANSLATED_SID {
    fn eq(&self, other: &Self) -> bool {
        self.Use == other.Use && self.RelativeId == other.RelativeId && self.DomainIndex == other.DomainIndex
    }
}
impl ::core::cmp::Eq for LSA_TRANSLATED_SID {}
impl ::core::fmt::Debug for LSA_TRANSLATED_SID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_TRANSLATED_SID").field("Use", &self.Use).field("RelativeId", &self.RelativeId).field("DomainIndex", &self.DomainIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_TRANSLATED_SID2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_TRANSLATED_SID2 {
    fn eq(&self, other: &Self) -> bool {
        self.Use == other.Use && self.Sid == other.Sid && self.DomainIndex == other.DomainIndex && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_TRANSLATED_SID2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_TRANSLATED_SID2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_TRANSLATED_SID2").field("Use", &self.Use).field("Sid", &self.Sid).field("DomainIndex", &self.DomainIndex).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LSA_TRUST_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LSA_TRUST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Sid == other.Sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LSA_TRUST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LSA_TRUST_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LSA_TRUST_INFORMATION").field("Name", &self.Name).field("Sid", &self.Sid).finish()
    }
}
impl ::core::default::Default for MSV1_0 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV1_0").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSV1_0_AVID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV1_0_AVID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV1_0_AVID").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSV1_0_AV_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_AV_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.AvId == other.AvId && self.AvLen == other.AvLen
    }
}
impl ::core::cmp::Eq for MSV1_0_AV_PAIR {}
impl ::core::fmt::Debug for MSV1_0_AV_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_AV_PAIR").field("AvId", &self.AvId).field("AvLen", &self.AvLen).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSV1_0_CHANGEPASSWORD_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSV1_0_CHANGEPASSWORD_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.DomainName == other.DomainName && self.AccountName == other.AccountName && self.OldPassword == other.OldPassword && self.NewPassword == other.NewPassword && self.Impersonating == other.Impersonating
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSV1_0_CHANGEPASSWORD_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSV1_0_CHANGEPASSWORD_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_CHANGEPASSWORD_REQUEST").field("MessageType", &self.MessageType).field("DomainName", &self.DomainName).field("AccountName", &self.AccountName).field("OldPassword", &self.OldPassword).field("NewPassword", &self.NewPassword).field("Impersonating", &self.Impersonating).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSV1_0_CHANGEPASSWORD_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSV1_0_CHANGEPASSWORD_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.PasswordInfoValid == other.PasswordInfoValid && self.DomainPasswordInfo == other.DomainPasswordInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSV1_0_CHANGEPASSWORD_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSV1_0_CHANGEPASSWORD_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_CHANGEPASSWORD_RESPONSE").field("MessageType", &self.MessageType).field("PasswordInfoValid", &self.PasswordInfoValid).field("DomainPasswordInfo", &self.DomainPasswordInfo).finish()
    }
}
impl ::core::default::Default for MSV1_0_CREDENTIAL_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_CREDENTIAL_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for MSV1_0_CREDENTIAL_KEY {}
impl ::core::fmt::Debug for MSV1_0_CREDENTIAL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_CREDENTIAL_KEY").field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for MSV1_0_CREDENTIAL_KEY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV1_0_CREDENTIAL_KEY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV1_0_CREDENTIAL_KEY_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSV1_0_INTERACTIVE_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSV1_0_INTERACTIVE_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonDomainName == other.LogonDomainName && self.UserName == other.UserName && self.Password == other.Password
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSV1_0_INTERACTIVE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSV1_0_INTERACTIVE_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_INTERACTIVE_LOGON").field("MessageType", &self.MessageType).field("LogonDomainName", &self.LogonDomainName).field("UserName", &self.UserName).field("Password", &self.Password).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSV1_0_INTERACTIVE_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSV1_0_INTERACTIVE_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonCount == other.LogonCount && self.BadPasswordCount == other.BadPasswordCount && self.LogonTime == other.LogonTime && self.LogoffTime == other.LogoffTime && self.KickOffTime == other.KickOffTime && self.PasswordLastSet == other.PasswordLastSet && self.PasswordCanChange == other.PasswordCanChange && self.PasswordMustChange == other.PasswordMustChange && self.LogonScript == other.LogonScript && self.HomeDirectory == other.HomeDirectory && self.FullName == other.FullName && self.ProfilePath == other.ProfilePath && self.HomeDirectoryDrive == other.HomeDirectoryDrive && self.LogonServer == other.LogonServer && self.UserFlags == other.UserFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSV1_0_INTERACTIVE_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSV1_0_INTERACTIVE_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_INTERACTIVE_PROFILE")
            .field("MessageType", &self.MessageType)
            .field("LogonCount", &self.LogonCount)
            .field("BadPasswordCount", &self.BadPasswordCount)
            .field("LogonTime", &self.LogonTime)
            .field("LogoffTime", &self.LogoffTime)
            .field("KickOffTime", &self.KickOffTime)
            .field("PasswordLastSet", &self.PasswordLastSet)
            .field("PasswordCanChange", &self.PasswordCanChange)
            .field("PasswordMustChange", &self.PasswordMustChange)
            .field("LogonScript", &self.LogonScript)
            .field("HomeDirectory", &self.HomeDirectory)
            .field("FullName", &self.FullName)
            .field("ProfilePath", &self.ProfilePath)
            .field("HomeDirectoryDrive", &self.HomeDirectoryDrive)
            .field("LogonServer", &self.LogonServer)
            .field("UserFlags", &self.UserFlags)
            .finish()
    }
}
impl ::core::default::Default for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.EncryptedCredsSize == other.EncryptedCredsSize && self.EncryptedCreds == other.EncryptedCreds
    }
}
impl ::core::cmp::Eq for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {}
impl ::core::fmt::Debug for MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL").field("Version", &self.Version).field("EncryptedCredsSize", &self.EncryptedCredsSize).field("EncryptedCreds", &self.EncryptedCreds).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MSV1_0_LM20_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for MSV1_0_LM20_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonDomainName == other.LogonDomainName && self.UserName == other.UserName && self.Workstation == other.Workstation && self.ChallengeToClient == other.ChallengeToClient && self.CaseSensitiveChallengeResponse == other.CaseSensitiveChallengeResponse && self.CaseInsensitiveChallengeResponse == other.CaseInsensitiveChallengeResponse && self.ParameterControl == other.ParameterControl
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for MSV1_0_LM20_LOGON {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for MSV1_0_LM20_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_LM20_LOGON").field("MessageType", &self.MessageType).field("LogonDomainName", &self.LogonDomainName).field("UserName", &self.UserName).field("Workstation", &self.Workstation).field("ChallengeToClient", &self.ChallengeToClient).field("CaseSensitiveChallengeResponse", &self.CaseSensitiveChallengeResponse).field("CaseInsensitiveChallengeResponse", &self.CaseInsensitiveChallengeResponse).field("ParameterControl", &self.ParameterControl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSV1_0_LM20_LOGON_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSV1_0_LM20_LOGON_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.KickOffTime == other.KickOffTime && self.LogoffTime == other.LogoffTime && self.UserFlags == other.UserFlags && self.UserSessionKey == other.UserSessionKey && self.LogonDomainName == other.LogonDomainName && self.LanmanSessionKey == other.LanmanSessionKey && self.LogonServer == other.LogonServer && self.UserParameters == other.UserParameters
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSV1_0_LM20_LOGON_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSV1_0_LM20_LOGON_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_LM20_LOGON_PROFILE").field("MessageType", &self.MessageType).field("KickOffTime", &self.KickOffTime).field("LogoffTime", &self.LogoffTime).field("UserFlags", &self.UserFlags).field("UserSessionKey", &self.UserSessionKey).field("LogonDomainName", &self.LogonDomainName).field("LanmanSessionKey", &self.LanmanSessionKey).field("LogonServer", &self.LogonServer).field("UserParameters", &self.UserParameters).finish()
    }
}
impl ::core::default::Default for MSV1_0_LOGON_SUBMIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV1_0_LOGON_SUBMIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV1_0_LOGON_SUBMIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSV1_0_NTLM3_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_NTLM3_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.Response == other.Response && self.RespType == other.RespType && self.HiRespType == other.HiRespType && self.Flags == other.Flags && self.MsgWord == other.MsgWord && self.TimeStamp == other.TimeStamp && self.ChallengeFromClient == other.ChallengeFromClient && self.AvPairsOff == other.AvPairsOff && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for MSV1_0_NTLM3_RESPONSE {}
impl ::core::fmt::Debug for MSV1_0_NTLM3_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_NTLM3_RESPONSE").field("Response", &self.Response).field("RespType", &self.RespType).field("HiRespType", &self.HiRespType).field("Flags", &self.Flags).field("MsgWord", &self.MsgWord).field("TimeStamp", &self.TimeStamp).field("ChallengeFromClient", &self.ChallengeFromClient).field("AvPairsOff", &self.AvPairsOff).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSV1_0_PASSTHROUGH_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSV1_0_PASSTHROUGH_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.DomainName == other.DomainName && self.PackageName == other.PackageName && self.DataLength == other.DataLength && self.LogonData == other.LogonData && self.Pad == other.Pad
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSV1_0_PASSTHROUGH_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSV1_0_PASSTHROUGH_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_PASSTHROUGH_REQUEST").field("MessageType", &self.MessageType).field("DomainName", &self.DomainName).field("PackageName", &self.PackageName).field("DataLength", &self.DataLength).field("LogonData", &self.LogonData).field("Pad", &self.Pad).finish()
    }
}
impl ::core::default::Default for MSV1_0_PASSTHROUGH_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_PASSTHROUGH_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Pad == other.Pad && self.DataLength == other.DataLength && self.ValidationData == other.ValidationData
    }
}
impl ::core::cmp::Eq for MSV1_0_PASSTHROUGH_RESPONSE {}
impl ::core::fmt::Debug for MSV1_0_PASSTHROUGH_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_PASSTHROUGH_RESPONSE").field("MessageType", &self.MessageType).field("Pad", &self.Pad).field("DataLength", &self.DataLength).field("ValidationData", &self.ValidationData).finish()
    }
}
impl ::core::default::Default for MSV1_0_PROFILE_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV1_0_PROFILE_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV1_0_PROFILE_BUFFER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSV1_0_PROTOCOL_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV1_0_PROTOCOL_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV1_0_PROTOCOL_MESSAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSV1_0_S4U_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSV1_0_S4U_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.UserPrincipalName == other.UserPrincipalName && self.DomainName == other.DomainName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSV1_0_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSV1_0_S4U_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_S4U_LOGON").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("UserPrincipalName", &self.UserPrincipalName).field("DomainName", &self.DomainName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MSV1_0_SUBAUTH_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for MSV1_0_SUBAUTH_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonDomainName == other.LogonDomainName && self.UserName == other.UserName && self.Workstation == other.Workstation && self.ChallengeToClient == other.ChallengeToClient && self.AuthenticationInfo1 == other.AuthenticationInfo1 && self.AuthenticationInfo2 == other.AuthenticationInfo2 && self.ParameterControl == other.ParameterControl && self.SubAuthPackageId == other.SubAuthPackageId
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for MSV1_0_SUBAUTH_LOGON {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for MSV1_0_SUBAUTH_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_SUBAUTH_LOGON").field("MessageType", &self.MessageType).field("LogonDomainName", &self.LogonDomainName).field("UserName", &self.UserName).field("Workstation", &self.Workstation).field("ChallengeToClient", &self.ChallengeToClient).field("AuthenticationInfo1", &self.AuthenticationInfo1).field("AuthenticationInfo2", &self.AuthenticationInfo2).field("ParameterControl", &self.ParameterControl).field("SubAuthPackageId", &self.SubAuthPackageId).finish()
    }
}
impl ::core::default::Default for MSV1_0_SUBAUTH_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_SUBAUTH_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.SubAuthPackageId == other.SubAuthPackageId && self.SubAuthInfoLength == other.SubAuthInfoLength && self.SubAuthSubmitBuffer == other.SubAuthSubmitBuffer
    }
}
impl ::core::cmp::Eq for MSV1_0_SUBAUTH_REQUEST {}
impl ::core::fmt::Debug for MSV1_0_SUBAUTH_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_SUBAUTH_REQUEST").field("MessageType", &self.MessageType).field("SubAuthPackageId", &self.SubAuthPackageId).field("SubAuthInfoLength", &self.SubAuthInfoLength).field("SubAuthSubmitBuffer", &self.SubAuthSubmitBuffer).finish()
    }
}
impl ::core::default::Default for MSV1_0_SUBAUTH_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_SUBAUTH_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.SubAuthInfoLength == other.SubAuthInfoLength && self.SubAuthReturnBuffer == other.SubAuthReturnBuffer
    }
}
impl ::core::cmp::Eq for MSV1_0_SUBAUTH_RESPONSE {}
impl ::core::fmt::Debug for MSV1_0_SUBAUTH_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_SUBAUTH_RESPONSE").field("MessageType", &self.MessageType).field("SubAuthInfoLength", &self.SubAuthInfoLength).field("SubAuthReturnBuffer", &self.SubAuthReturnBuffer).finish()
    }
}
impl ::core::default::Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.LmPassword == other.LmPassword && self.NtPassword == other.NtPassword
    }
}
impl ::core::cmp::Eq for MSV1_0_SUPPLEMENTAL_CREDENTIAL {}
impl ::core::fmt::Debug for MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_SUPPLEMENTAL_CREDENTIAL").field("Version", &self.Version).field("Flags", &self.Flags).field("LmPassword", &self.LmPassword).field("NtPassword", &self.NtPassword).finish()
    }
}
impl ::core::default::Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.NtPassword == other.NtPassword && self.CredentialKey == other.CredentialKey
    }
}
impl ::core::cmp::Eq for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {}
impl ::core::fmt::Debug for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2").field("Version", &self.Version).field("Flags", &self.Flags).field("NtPassword", &self.NtPassword).field("CredentialKey", &self.CredentialKey).finish()
    }
}
impl ::core::default::Default for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.CredentialKeyType == other.CredentialKeyType && self.NtPassword == other.NtPassword && self.CredentialKey == other.CredentialKey && self.ShaPassword == other.ShaPassword
    }
}
impl ::core::cmp::Eq for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {}
impl ::core::fmt::Debug for MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3").field("Version", &self.Version).field("Flags", &self.Flags).field("CredentialKeyType", &self.CredentialKeyType).field("NtPassword", &self.NtPassword).field("CredentialKey", &self.CredentialKey).field("ShaPassword", &self.ShaPassword).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::default::Default for MSV1_0_VALIDATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::cmp::PartialEq for MSV1_0_VALIDATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LogoffTime == other.LogoffTime && self.KickoffTime == other.KickoffTime && self.LogonServer == other.LogonServer && self.LogonDomainName == other.LogonDomainName && self.SessionKey == other.SessionKey && self.Authoritative == other.Authoritative && self.UserFlags == other.UserFlags && self.WhichFields == other.WhichFields && self.UserId == other.UserId
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::cmp::Eq for MSV1_0_VALIDATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::fmt::Debug for MSV1_0_VALIDATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSV1_0_VALIDATION_INFO").field("LogoffTime", &self.LogoffTime).field("KickoffTime", &self.KickoffTime).field("LogonServer", &self.LogonServer).field("LogonDomainName", &self.LogonDomainName).field("SessionKey", &self.SessionKey).field("Authoritative", &self.Authoritative).field("UserFlags", &self.UserFlags).field("WhichFields", &self.WhichFields).field("UserId", &self.UserId).finish()
    }
}
impl ::core::default::Default for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV_SUBAUTH_LOGON_PARAMETER_CONTROL").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MSV_SUBAUTH_LOGON_PARAMETER_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MSV_SUB_AUTHENTICATION_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV_SUB_AUTHENTICATION_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV_SUB_AUTHENTICATION_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MSV_SUPPLEMENTAL_CREDENTIAL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NEGOTIATE_CALLER_NAME_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NEGOTIATE_CALLER_NAME_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.LogonId == other.LogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NEGOTIATE_CALLER_NAME_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NEGOTIATE_CALLER_NAME_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEGOTIATE_CALLER_NAME_REQUEST").field("MessageType", &self.MessageType).field("LogonId", &self.LogonId).finish()
    }
}
impl ::core::default::Default for NEGOTIATE_CALLER_NAME_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NEGOTIATE_CALLER_NAME_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.CallerName == other.CallerName
    }
}
impl ::core::cmp::Eq for NEGOTIATE_CALLER_NAME_RESPONSE {}
impl ::core::fmt::Debug for NEGOTIATE_CALLER_NAME_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEGOTIATE_CALLER_NAME_RESPONSE").field("MessageType", &self.MessageType).field("CallerName", &self.CallerName).finish()
    }
}
impl ::core::default::Default for NEGOTIATE_MESSAGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NEGOTIATE_MESSAGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NEGOTIATE_MESSAGES").field(&self.0).finish()
    }
}
impl ::core::default::Default for NEGOTIATE_PACKAGE_PREFIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NEGOTIATE_PACKAGE_PREFIX {
    fn eq(&self, other: &Self) -> bool {
        self.PackageId == other.PackageId && self.PackageDataA == other.PackageDataA && self.PackageDataW == other.PackageDataW && self.PrefixLen == other.PrefixLen && self.Prefix == other.Prefix
    }
}
impl ::core::cmp::Eq for NEGOTIATE_PACKAGE_PREFIX {}
impl ::core::fmt::Debug for NEGOTIATE_PACKAGE_PREFIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEGOTIATE_PACKAGE_PREFIX").field("PackageId", &self.PackageId).field("PackageDataA", &self.PackageDataA).field("PackageDataW", &self.PackageDataW).field("PrefixLen", &self.PrefixLen).field("Prefix", &self.Prefix).finish()
    }
}
impl ::core::default::Default for NEGOTIATE_PACKAGE_PREFIXES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NEGOTIATE_PACKAGE_PREFIXES {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.PrefixCount == other.PrefixCount && self.Offset == other.Offset && self.Pad == other.Pad
    }
}
impl ::core::cmp::Eq for NEGOTIATE_PACKAGE_PREFIXES {}
impl ::core::fmt::Debug for NEGOTIATE_PACKAGE_PREFIXES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEGOTIATE_PACKAGE_PREFIXES").field("MessageType", &self.MessageType).field("PrefixCount", &self.PrefixCount).field("Offset", &self.Offset).field("Pad", &self.Pad).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETLOGON_GENERIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETLOGON_GENERIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Identity == other.Identity && self.PackageName == other.PackageName && self.DataLength == other.DataLength && self.LogonData == other.LogonData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETLOGON_GENERIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NETLOGON_GENERIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_GENERIC_INFO").field("Identity", &self.Identity).field("PackageName", &self.PackageName).field("DataLength", &self.DataLength).field("LogonData", &self.LogonData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::default::Default for NETLOGON_INTERACTIVE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::cmp::PartialEq for NETLOGON_INTERACTIVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Identity == other.Identity && self.LmOwfPassword == other.LmOwfPassword && self.NtOwfPassword == other.NtOwfPassword
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::cmp::Eq for NETLOGON_INTERACTIVE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::fmt::Debug for NETLOGON_INTERACTIVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INTERACTIVE_INFO").field("Identity", &self.Identity).field("LmOwfPassword", &self.LmOwfPassword).field("NtOwfPassword", &self.NtOwfPassword).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETLOGON_LOGON_IDENTITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETLOGON_LOGON_IDENTITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LogonDomainName == other.LogonDomainName && self.ParameterControl == other.ParameterControl && self.LogonId == other.LogonId && self.UserName == other.UserName && self.Workstation == other.Workstation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETLOGON_LOGON_IDENTITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NETLOGON_LOGON_IDENTITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_LOGON_IDENTITY_INFO").field("LogonDomainName", &self.LogonDomainName).field("ParameterControl", &self.ParameterControl).field("LogonId", &self.LogonId).field("UserName", &self.UserName).field("Workstation", &self.Workstation).finish()
    }
}
impl ::core::default::Default for NETLOGON_LOGON_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETLOGON_LOGON_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETLOGON_LOGON_INFO_CLASS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for NETLOGON_NETWORK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for NETLOGON_NETWORK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Identity == other.Identity && self.LmChallenge == other.LmChallenge && self.NtChallengeResponse == other.NtChallengeResponse && self.LmChallengeResponse == other.LmChallengeResponse
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for NETLOGON_NETWORK_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for NETLOGON_NETWORK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_NETWORK_INFO").field("Identity", &self.Identity).field("LmChallenge", &self.LmChallenge).field("NtChallengeResponse", &self.NtChallengeResponse).field("LmChallengeResponse", &self.LmChallengeResponse).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::default::Default for NETLOGON_SERVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::cmp::PartialEq for NETLOGON_SERVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Identity == other.Identity && self.LmOwfPassword == other.LmOwfPassword && self.NtOwfPassword == other.NtOwfPassword
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::cmp::Eq for NETLOGON_SERVICE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::fmt::Debug for NETLOGON_SERVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_SERVICE_INFO").field("Identity", &self.Identity).field("LmOwfPassword", &self.LmOwfPassword).field("NtOwfPassword", &self.NtOwfPassword).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PKU2U_CERTIFICATE_S4U_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PKU2U_CERTIFICATE_S4U_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.UserPrincipalName == other.UserPrincipalName && self.DomainName == other.DomainName && self.CertificateLength == other.CertificateLength && self.Certificate == other.Certificate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PKU2U_CERTIFICATE_S4U_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PKU2U_CERTIFICATE_S4U_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PKU2U_CERTIFICATE_S4U_LOGON").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("UserPrincipalName", &self.UserPrincipalName).field("DomainName", &self.DomainName).field("CertificateLength", &self.CertificateLength).field("Certificate", &self.Certificate).finish()
    }
}
impl ::core::default::Default for PKU2U_CERT_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PKU2U_CERT_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.CertOffset == other.CertOffset && self.CertLength == other.CertLength
    }
}
impl ::core::cmp::Eq for PKU2U_CERT_BLOB {}
impl ::core::fmt::Debug for PKU2U_CERT_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PKU2U_CERT_BLOB").field("CertOffset", &self.CertOffset).field("CertLength", &self.CertLength).finish()
    }
}
impl ::core::default::Default for PKU2U_CREDUI_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PKU2U_CREDUI_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.cbHeaderLength == other.cbHeaderLength && self.cbStructureLength == other.cbStructureLength && self.CertArrayCount == other.CertArrayCount && self.CertArrayOffset == other.CertArrayOffset
    }
}
impl ::core::cmp::Eq for PKU2U_CREDUI_CONTEXT {}
impl ::core::fmt::Debug for PKU2U_CREDUI_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PKU2U_CREDUI_CONTEXT").field("Version", &self.Version).field("cbHeaderLength", &self.cbHeaderLength).field("cbStructureLength", &self.cbStructureLength).field("CertArrayCount", &self.CertArrayCount).field("CertArrayOffset", &self.CertArrayOffset).finish()
    }
}
impl ::core::default::Default for PKU2U_LOGON_SUBMIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PKU2U_LOGON_SUBMIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PKU2U_LOGON_SUBMIT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_ACCOUNT_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_ACCOUNT_DOMAIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DomainName == other.DomainName && self.DomainSid == other.DomainSid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_ACCOUNT_DOMAIN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_ACCOUNT_DOMAIN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_ACCOUNT_DOMAIN_INFO").field("DomainName", &self.DomainName).field("DomainSid", &self.DomainSid).finish()
    }
}
impl ::core::default::Default for POLICY_AUDIT_CATEGORIES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_AUDIT_CATEGORIES_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumCategoryCount == other.MaximumCategoryCount && self.SubCategoriesInfo == other.SubCategoriesInfo
    }
}
impl ::core::cmp::Eq for POLICY_AUDIT_CATEGORIES_INFO {}
impl ::core::fmt::Debug for POLICY_AUDIT_CATEGORIES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_AUDIT_CATEGORIES_INFO").field("MaximumCategoryCount", &self.MaximumCategoryCount).field("SubCategoriesInfo", &self.SubCategoriesInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_AUDIT_EVENTS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_AUDIT_EVENTS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AuditingMode == other.AuditingMode && self.EventAuditingOptions == other.EventAuditingOptions && self.MaximumAuditEventCount == other.MaximumAuditEventCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_AUDIT_EVENTS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_AUDIT_EVENTS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_AUDIT_EVENTS_INFO").field("AuditingMode", &self.AuditingMode).field("EventAuditingOptions", &self.EventAuditingOptions).field("MaximumAuditEventCount", &self.MaximumAuditEventCount).finish()
    }
}
impl ::core::default::Default for POLICY_AUDIT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POLICY_AUDIT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POLICY_AUDIT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_AUDIT_FULL_QUERY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_AUDIT_FULL_QUERY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ShutDownOnFull == other.ShutDownOnFull && self.LogIsFull == other.LogIsFull
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_AUDIT_FULL_QUERY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_AUDIT_FULL_QUERY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_AUDIT_FULL_QUERY_INFO").field("ShutDownOnFull", &self.ShutDownOnFull).field("LogIsFull", &self.LogIsFull).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_AUDIT_FULL_SET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_AUDIT_FULL_SET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ShutDownOnFull == other.ShutDownOnFull
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_AUDIT_FULL_SET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_AUDIT_FULL_SET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_AUDIT_FULL_SET_INFO").field("ShutDownOnFull", &self.ShutDownOnFull).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_AUDIT_LOG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_AUDIT_LOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AuditLogPercentFull == other.AuditLogPercentFull && self.MaximumLogSize == other.MaximumLogSize && self.AuditRetentionPeriod == other.AuditRetentionPeriod && self.AuditLogFullShutdownInProgress == other.AuditLogFullShutdownInProgress && self.TimeToShutdown == other.TimeToShutdown && self.NextAuditRecordId == other.NextAuditRecordId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_AUDIT_LOG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_AUDIT_LOG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_AUDIT_LOG_INFO").field("AuditLogPercentFull", &self.AuditLogPercentFull).field("MaximumLogSize", &self.MaximumLogSize).field("AuditRetentionPeriod", &self.AuditRetentionPeriod).field("AuditLogFullShutdownInProgress", &self.AuditLogFullShutdownInProgress).field("TimeToShutdown", &self.TimeToShutdown).field("NextAuditRecordId", &self.NextAuditRecordId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_AUDIT_SID_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_AUDIT_SID_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.UsersCount == other.UsersCount && self.UserSidArray == other.UserSidArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_AUDIT_SID_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_AUDIT_SID_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_AUDIT_SID_ARRAY").field("UsersCount", &self.UsersCount).field("UserSidArray", &self.UserSidArray).finish()
    }
}
impl ::core::default::Default for POLICY_AUDIT_SUBCATEGORIES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_AUDIT_SUBCATEGORIES_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumSubCategoryCount == other.MaximumSubCategoryCount && self.EventAuditingOptions == other.EventAuditingOptions
    }
}
impl ::core::cmp::Eq for POLICY_AUDIT_SUBCATEGORIES_INFO {}
impl ::core::fmt::Debug for POLICY_AUDIT_SUBCATEGORIES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_AUDIT_SUBCATEGORIES_INFO").field("MaximumSubCategoryCount", &self.MaximumSubCategoryCount).field("EventAuditingOptions", &self.EventAuditingOptions).finish()
    }
}
impl ::core::default::Default for POLICY_DEFAULT_QUOTA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_DEFAULT_QUOTA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.QuotaLimits == other.QuotaLimits
    }
}
impl ::core::cmp::Eq for POLICY_DEFAULT_QUOTA_INFO {}
impl ::core::fmt::Debug for POLICY_DEFAULT_QUOTA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_DEFAULT_QUOTA_INFO").field("QuotaLimits", &self.QuotaLimits).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_DNS_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_DNS_DOMAIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.DnsDomainName == other.DnsDomainName && self.DnsForestName == other.DnsForestName && self.DomainGuid == other.DomainGuid && self.Sid == other.Sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_DNS_DOMAIN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_DNS_DOMAIN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_DNS_DOMAIN_INFO").field("Name", &self.Name).field("DnsDomainName", &self.DnsDomainName).field("DnsForestName", &self.DnsForestName).field("DomainGuid", &self.DomainGuid).field("Sid", &self.Sid).finish()
    }
}
impl ::core::default::Default for POLICY_DOMAIN_EFS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_DOMAIN_EFS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InfoLength == other.InfoLength && self.EfsBlob == other.EfsBlob
    }
}
impl ::core::cmp::Eq for POLICY_DOMAIN_EFS_INFO {}
impl ::core::fmt::Debug for POLICY_DOMAIN_EFS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_DOMAIN_EFS_INFO").field("InfoLength", &self.InfoLength).field("EfsBlob", &self.EfsBlob).finish()
    }
}
impl ::core::default::Default for POLICY_DOMAIN_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POLICY_DOMAIN_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POLICY_DOMAIN_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AuthenticationOptions == other.AuthenticationOptions && self.MaxServiceTicketAge == other.MaxServiceTicketAge && self.MaxTicketAge == other.MaxTicketAge && self.MaxRenewAge == other.MaxRenewAge && self.MaxClockSkew == other.MaxClockSkew && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for POLICY_DOMAIN_KERBEROS_TICKET_INFO {}
impl ::core::fmt::Debug for POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_DOMAIN_KERBEROS_TICKET_INFO").field("AuthenticationOptions", &self.AuthenticationOptions).field("MaxServiceTicketAge", &self.MaxServiceTicketAge).field("MaxTicketAge", &self.MaxTicketAge).field("MaxRenewAge", &self.MaxRenewAge).field("MaxClockSkew", &self.MaxClockSkew).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for POLICY_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POLICY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POLICY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for POLICY_LSA_SERVER_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POLICY_LSA_SERVER_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POLICY_LSA_SERVER_ROLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for POLICY_LSA_SERVER_ROLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_LSA_SERVER_ROLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LsaServerRole == other.LsaServerRole
    }
}
impl ::core::cmp::Eq for POLICY_LSA_SERVER_ROLE_INFO {}
impl ::core::fmt::Debug for POLICY_LSA_SERVER_ROLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_LSA_SERVER_ROLE_INFO").field("LsaServerRole", &self.LsaServerRole).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_MACHINE_ACCT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_MACHINE_ACCT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Rid == other.Rid && self.Sid == other.Sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_MACHINE_ACCT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_MACHINE_ACCT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_MACHINE_ACCT_INFO").field("Rid", &self.Rid).field("Sid", &self.Sid).finish()
    }
}
impl ::core::default::Default for POLICY_MODIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_MODIFICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ModifiedId == other.ModifiedId && self.DatabaseCreationTime == other.DatabaseCreationTime
    }
}
impl ::core::cmp::Eq for POLICY_MODIFICATION_INFO {}
impl ::core::fmt::Debug for POLICY_MODIFICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_MODIFICATION_INFO").field("ModifiedId", &self.ModifiedId).field("DatabaseCreationTime", &self.DatabaseCreationTime).finish()
    }
}
impl ::core::default::Default for POLICY_NOTIFICATION_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POLICY_NOTIFICATION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POLICY_NOTIFICATION_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_PD_ACCOUNT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_PD_ACCOUNT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_PD_ACCOUNT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_PD_ACCOUNT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_PD_ACCOUNT_INFO").field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_PRIMARY_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_PRIMARY_DOMAIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Sid == other.Sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_PRIMARY_DOMAIN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_PRIMARY_DOMAIN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_PRIMARY_DOMAIN_INFO").field("Name", &self.Name).field("Sid", &self.Sid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICY_REPLICA_SOURCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICY_REPLICA_SOURCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ReplicaSource == other.ReplicaSource && self.ReplicaAccountName == other.ReplicaAccountName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICY_REPLICA_SOURCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICY_REPLICA_SOURCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_REPLICA_SOURCE_INFO").field("ReplicaSource", &self.ReplicaSource).field("ReplicaAccountName", &self.ReplicaAccountName).finish()
    }
}
impl ::core::default::Default for PctPublicKey {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PctPublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.cbKey == other.cbKey && self.pKey == other.pKey
    }
}
impl ::core::cmp::Eq for PctPublicKey {}
impl ::core::fmt::Debug for PctPublicKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PctPublicKey").field("Type", &self.Type).field("cbKey", &self.cbKey).field("pKey", &self.pKey).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAM_REGISTER_MAPPING_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAM_REGISTER_MAPPING_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Original == other.Original && self.Mapped == other.Mapped && self.Continuable == other.Continuable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAM_REGISTER_MAPPING_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAM_REGISTER_MAPPING_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAM_REGISTER_MAPPING_ELEMENT").field("Original", &self.Original).field("Mapped", &self.Mapped).field("Continuable", &self.Continuable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAM_REGISTER_MAPPING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAM_REGISTER_MAPPING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAM_REGISTER_MAPPING_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAM_REGISTER_MAPPING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAM_REGISTER_MAPPING_LIST").field("Count", &self.Count).field("Elements", &self.Elements).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAM_REGISTER_MAPPING_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAM_REGISTER_MAPPING_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lists == other.Lists
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAM_REGISTER_MAPPING_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAM_REGISTER_MAPPING_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAM_REGISTER_MAPPING_TABLE").field("Count", &self.Count).field("Lists", &self.Lists).finish()
    }
}
impl ::core::default::Default for SASL_AUTHZID_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SASL_AUTHZID_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SASL_AUTHZID_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCHANNEL_ALERT_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCHANNEL_ALERT_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.dwTokenType == other.dwTokenType && self.dwAlertType == other.dwAlertType && self.dwAlertNumber == other.dwAlertNumber
    }
}
impl ::core::cmp::Eq for SCHANNEL_ALERT_TOKEN {}
impl ::core::fmt::Debug for SCHANNEL_ALERT_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHANNEL_ALERT_TOKEN").field("dwTokenType", &self.dwTokenType).field("dwAlertType", &self.dwAlertType).field("dwAlertNumber", &self.dwAlertNumber).finish()
    }
}
impl ::core::default::Default for SCHANNEL_ALERT_TOKEN_ALERT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCHANNEL_ALERT_TOKEN_ALERT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHANNEL_ALERT_TOKEN_ALERT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCHANNEL_CERT_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCHANNEL_CERT_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwFlags == other.dwFlags && self.hProv == other.hProv && self.ShaHash == other.ShaHash
    }
}
impl ::core::cmp::Eq for SCHANNEL_CERT_HASH {}
impl ::core::fmt::Debug for SCHANNEL_CERT_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHANNEL_CERT_HASH").field("dwLength", &self.dwLength).field("dwFlags", &self.dwFlags).field("hProv", &self.hProv).field("ShaHash", &self.ShaHash).finish()
    }
}
impl ::core::default::Default for SCHANNEL_CERT_HASH_STORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCHANNEL_CERT_HASH_STORE {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwFlags == other.dwFlags && self.hProv == other.hProv && self.ShaHash == other.ShaHash && self.pwszStoreName == other.pwszStoreName
    }
}
impl ::core::cmp::Eq for SCHANNEL_CERT_HASH_STORE {}
impl ::core::fmt::Debug for SCHANNEL_CERT_HASH_STORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHANNEL_CERT_HASH_STORE").field("dwLength", &self.dwLength).field("dwFlags", &self.dwFlags).field("hProv", &self.hProv).field("ShaHash", &self.ShaHash).field("pwszStoreName", &self.pwszStoreName).finish()
    }
}
impl ::core::default::Default for SCHANNEL_CLIENT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCHANNEL_CLIENT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.cbLength == other.cbLength && self.aiHash == other.aiHash && self.cbHash == other.cbHash && self.HashValue == other.HashValue && self.CertThumbprint == other.CertThumbprint
    }
}
impl ::core::cmp::Eq for SCHANNEL_CLIENT_SIGNATURE {}
impl ::core::fmt::Debug for SCHANNEL_CLIENT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHANNEL_CLIENT_SIGNATURE").field("cbLength", &self.cbLength).field("aiHash", &self.aiHash).field("cbHash", &self.cbHash).field("HashValue", &self.HashValue).field("CertThumbprint", &self.CertThumbprint).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for SCHANNEL_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for SCHANNEL_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cCreds == other.cCreds && self.paCred == other.paCred && self.hRootStore == other.hRootStore && self.cMappers == other.cMappers && self.aphMappers == other.aphMappers && self.cSupportedAlgs == other.cSupportedAlgs && self.palgSupportedAlgs == other.palgSupportedAlgs && self.grbitEnabledProtocols == other.grbitEnabledProtocols && self.dwMinimumCipherStrength == other.dwMinimumCipherStrength && self.dwMaximumCipherStrength == other.dwMaximumCipherStrength && self.dwSessionLifespan == other.dwSessionLifespan && self.dwFlags == other.dwFlags && self.dwCredFormat == other.dwCredFormat
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for SCHANNEL_CRED {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for SCHANNEL_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHANNEL_CRED")
            .field("dwVersion", &self.dwVersion)
            .field("cCreds", &self.cCreds)
            .field("paCred", &self.paCred)
            .field("hRootStore", &self.hRootStore)
            .field("cMappers", &self.cMappers)
            .field("aphMappers", &self.aphMappers)
            .field("cSupportedAlgs", &self.cSupportedAlgs)
            .field("palgSupportedAlgs", &self.palgSupportedAlgs)
            .field("grbitEnabledProtocols", &self.grbitEnabledProtocols)
            .field("dwMinimumCipherStrength", &self.dwMinimumCipherStrength)
            .field("dwMaximumCipherStrength", &self.dwMaximumCipherStrength)
            .field("dwSessionLifespan", &self.dwSessionLifespan)
            .field("dwFlags", &self.dwFlags)
            .field("dwCredFormat", &self.dwCredFormat)
            .finish()
    }
}
impl ::core::default::Default for SCHANNEL_CRED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCHANNEL_CRED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHANNEL_CRED_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SCHANNEL_CRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SCHANNEL_CRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SCHANNEL_CRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SCHANNEL_CRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SCHANNEL_CRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SCHANNEL_SESSION_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCHANNEL_SESSION_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.dwTokenType == other.dwTokenType && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for SCHANNEL_SESSION_TOKEN {}
impl ::core::fmt::Debug for SCHANNEL_SESSION_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHANNEL_SESSION_TOKEN").field("dwTokenType", &self.dwTokenType).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for SCHANNEL_SESSION_TOKEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCHANNEL_SESSION_TOKEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCHANNEL_SESSION_TOKEN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCH_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCH_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cCreds == other.cCreds && self.paSecret == other.paSecret && self.paPublic == other.paPublic && self.cMappers == other.cMappers && self.aphMappers == other.aphMappers
    }
}
impl ::core::cmp::Eq for SCH_CRED {}
impl ::core::fmt::Debug for SCH_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCH_CRED").field("dwVersion", &self.dwVersion).field("cCreds", &self.cCreds).field("paSecret", &self.paSecret).field("paPublic", &self.paPublic).field("cMappers", &self.cMappers).field("aphMappers", &self.aphMappers).finish()
    }
}
impl ::core::default::Default for SCH_CRED_PUBLIC_CERTCHAIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCH_CRED_PUBLIC_CERTCHAIN {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.cbCertChain == other.cbCertChain && self.pCertChain == other.pCertChain
    }
}
impl ::core::cmp::Eq for SCH_CRED_PUBLIC_CERTCHAIN {}
impl ::core::fmt::Debug for SCH_CRED_PUBLIC_CERTCHAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCH_CRED_PUBLIC_CERTCHAIN").field("dwType", &self.dwType).field("cbCertChain", &self.cbCertChain).field("pCertChain", &self.pCertChain).finish()
    }
}
impl ::core::default::Default for SCH_CRED_SECRET_CAPI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCH_CRED_SECRET_CAPI {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.hProv == other.hProv
    }
}
impl ::core::cmp::Eq for SCH_CRED_SECRET_CAPI {}
impl ::core::fmt::Debug for SCH_CRED_SECRET_CAPI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCH_CRED_SECRET_CAPI").field("dwType", &self.dwType).field("hProv", &self.hProv).finish()
    }
}
impl ::core::default::Default for SCH_CRED_SECRET_PRIVKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCH_CRED_SECRET_PRIVKEY {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pPrivateKey == other.pPrivateKey && self.cbPrivateKey == other.cbPrivateKey && self.pszPassword == other.pszPassword
    }
}
impl ::core::cmp::Eq for SCH_CRED_SECRET_PRIVKEY {}
impl ::core::fmt::Debug for SCH_CRED_SECRET_PRIVKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCH_CRED_SECRET_PRIVKEY").field("dwType", &self.dwType).field("pPrivateKey", &self.pPrivateKey).field("cbPrivateKey", &self.cbPrivateKey).field("pszPassword", &self.pszPassword).finish()
    }
}
impl ::core::default::Default for SCH_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCH_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ExtensionType == other.ExtensionType && self.pExtData == other.pExtData && self.cbExtData == other.cbExtData
    }
}
impl ::core::cmp::Eq for SCH_EXTENSION_DATA {}
impl ::core::fmt::Debug for SCH_EXTENSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCH_EXTENSION_DATA").field("ExtensionType", &self.ExtensionType).field("pExtData", &self.pExtData).field("cbExtData", &self.cbExtData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_APP_MODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_APP_MODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.UserFunction == other.UserFunction && self.Argument1 == other.Argument1 && self.Argument2 == other.Argument2 && self.UserData == other.UserData && self.ReturnToLsa == other.ReturnToLsa
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_APP_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_APP_MODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_APP_MODE_INFO").field("UserFunction", &self.UserFunction).field("Argument1", &self.Argument1).field("Argument2", &self.Argument2).field("UserData", &self.UserData).field("ReturnToLsa", &self.ReturnToLsa).finish()
    }
}
impl ::core::default::Default for SECPKG_ATTR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_ATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_ATTR").field(&self.0).finish()
    }
}
impl ::core::default::Default for SECPKG_ATTR_LCT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_ATTR_LCT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_ATTR_LCT_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SECPKG_BYTE_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_BYTE_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.ByteArrayOffset == other.ByteArrayOffset && self.ByteArrayLength == other.ByteArrayLength
    }
}
impl ::core::cmp::Eq for SECPKG_BYTE_VECTOR {}
impl ::core::fmt::Debug for SECPKG_BYTE_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_BYTE_VECTOR").field("ByteArrayOffset", &self.ByteArrayOffset).field("ByteArrayLength", &self.ByteArrayLength).finish()
    }
}
impl ::core::default::Default for SECPKG_CALL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_CALL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessId == other.ProcessId && self.ThreadId == other.ThreadId && self.Attributes == other.Attributes && self.CallCount == other.CallCount && self.MechOid == other.MechOid
    }
}
impl ::core::cmp::Eq for SECPKG_CALL_INFO {}
impl ::core::fmt::Debug for SECPKG_CALL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_CALL_INFO").field("ProcessId", &self.ProcessId).field("ThreadId", &self.ThreadId).field("Attributes", &self.Attributes).field("CallCount", &self.CallCount).field("MechOid", &self.MechOid).finish()
    }
}
impl ::core::default::Default for SECPKG_CALL_PACKAGE_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_CALL_PACKAGE_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_CALL_PACKAGE_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags && self.DomainName == other.DomainName && self.DcName == other.DcName && self.DcFlags == other.DcFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_CALL_PACKAGE_PIN_DC_REQUEST").field("MessageType", &self.MessageType).field("Flags", &self.Flags).field("DomainName", &self.DomainName).field("DcName", &self.DcName).field("DcFlags", &self.DcFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.OriginLogonId == other.OriginLogonId && self.DestinationLogonId == other.DestinationLogonId && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST").field("MessageType", &self.MessageType).field("OriginLogonId", &self.OriginLogonId).field("DestinationLogonId", &self.DestinationLogonId).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {}
impl ::core::fmt::Debug for SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST").field("MessageType", &self.MessageType).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_CLIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_CLIENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LogonId == other.LogonId && self.ProcessID == other.ProcessID && self.ThreadID == other.ThreadID && self.HasTcbPrivilege == other.HasTcbPrivilege && self.Impersonating == other.Impersonating && self.Restricted == other.Restricted && self.ClientFlags == other.ClientFlags && self.ImpersonationLevel == other.ImpersonationLevel && self.ClientToken == other.ClientToken
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_CLIENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_CLIENT_INFO").field("LogonId", &self.LogonId).field("ProcessID", &self.ProcessID).field("ThreadID", &self.ThreadID).field("HasTcbPrivilege", &self.HasTcbPrivilege).field("Impersonating", &self.Impersonating).field("Restricted", &self.Restricted).field("ClientFlags", &self.ClientFlags).field("ImpersonationLevel", &self.ImpersonationLevel).field("ClientToken", &self.ClientToken).finish()
    }
}
impl ::core::default::Default for SECPKG_CONTEXT_THUNKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_CONTEXT_THUNKS {
    fn eq(&self, other: &Self) -> bool {
        self.InfoLevelCount == other.InfoLevelCount && self.Levels == other.Levels
    }
}
impl ::core::cmp::Eq for SECPKG_CONTEXT_THUNKS {}
impl ::core::fmt::Debug for SECPKG_CONTEXT_THUNKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_CONTEXT_THUNKS").field("InfoLevelCount", &self.InfoLevelCount).field("Levels", &self.Levels).finish()
    }
}
impl ::core::default::Default for SECPKG_CRED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_CRED").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.cbHeaderLength == other.cbHeaderLength && self.cbStructureLength == other.cbStructureLength && self.ClientProcess == other.ClientProcess && self.ClientThread == other.ClientThread && self.LogonId == other.LogonId && self.ClientToken == other.ClientToken && self.SessionId == other.SessionId && self.ModifiedId == other.ModifiedId && self.fCredentials == other.fCredentials && self.Flags == other.Flags && self.PrincipalName == other.PrincipalName && self.PackageList == other.PackageList && self.MarshaledSuppliedCreds == other.MarshaledSuppliedCreds
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_CREDENTIAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_CREDENTIAL")
            .field("Version", &self.Version)
            .field("cbHeaderLength", &self.cbHeaderLength)
            .field("cbStructureLength", &self.cbStructureLength)
            .field("ClientProcess", &self.ClientProcess)
            .field("ClientThread", &self.ClientThread)
            .field("LogonId", &self.LogonId)
            .field("ClientToken", &self.ClientToken)
            .field("SessionId", &self.SessionId)
            .field("ModifiedId", &self.ModifiedId)
            .field("fCredentials", &self.fCredentials)
            .field("Flags", &self.Flags)
            .field("PrincipalName", &self.PrincipalName)
            .field("PackageList", &self.PackageList)
            .field("MarshaledSuppliedCreds", &self.MarshaledSuppliedCreds)
            .finish()
    }
}
impl ::core::default::Default for SECPKG_CRED_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_CRED_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_CRED_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_DLL_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SECPKG_EVENT_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_EVENT_NOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.EventClass == other.EventClass && self.Reserved == other.Reserved && self.EventDataSize == other.EventDataSize && self.EventData == other.EventData && self.PackageParameter == other.PackageParameter
    }
}
impl ::core::cmp::Eq for SECPKG_EVENT_NOTIFY {}
impl ::core::fmt::Debug for SECPKG_EVENT_NOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_EVENT_NOTIFY").field("EventClass", &self.EventClass).field("Reserved", &self.Reserved).field("EventDataSize", &self.EventDataSize).field("EventData", &self.EventData).field("PackageParameter", &self.PackageParameter).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_EVENT_PACKAGE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_EVENT_PACKAGE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.ChangeType == other.ChangeType && self.PackageId == other.PackageId && self.PackageName == other.PackageName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_EVENT_PACKAGE_CHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_EVENT_PACKAGE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_EVENT_PACKAGE_CHANGE").field("ChangeType", &self.ChangeType).field("PackageId", &self.PackageId).field("PackageName", &self.PackageName).finish()
    }
}
impl ::core::default::Default for SECPKG_EVENT_ROLE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_EVENT_ROLE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousRole == other.PreviousRole && self.NewRole == other.NewRole
    }
}
impl ::core::cmp::Eq for SECPKG_EVENT_ROLE_CHANGE {}
impl ::core::fmt::Debug for SECPKG_EVENT_ROLE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_EVENT_ROLE_CHANGE").field("PreviousRole", &self.PreviousRole).field("NewRole", &self.NewRole).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SECPKG_EXTENDED_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_EXTENDED_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_EXTENDED_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SECPKG_EXTRA_OIDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_EXTRA_OIDS {
    fn eq(&self, other: &Self) -> bool {
        self.OidCount == other.OidCount && self.Oids == other.Oids
    }
}
impl ::core::cmp::Eq for SECPKG_EXTRA_OIDS {}
impl ::core::fmt::Debug for SECPKG_EXTRA_OIDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_EXTRA_OIDS").field("OidCount", &self.OidCount).field("Oids", &self.Oids).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials", feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
impl ::core::default::Default for SECPKG_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SECPKG_GSS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_GSS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EncodedIdLength == other.EncodedIdLength && self.EncodedId == other.EncodedId
    }
}
impl ::core::cmp::Eq for SECPKG_GSS_INFO {}
impl ::core::fmt::Debug for SECPKG_GSS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_GSS_INFO").field("EncodedIdLength", &self.EncodedIdLength).field("EncodedId", &self.EncodedId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for SECPKG_KERNEL_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for SECPKG_KERNEL_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SECPKG_MUTUAL_AUTH_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_MUTUAL_AUTH_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.MutualAuthLevel == other.MutualAuthLevel
    }
}
impl ::core::cmp::Eq for SECPKG_MUTUAL_AUTH_LEVEL {}
impl ::core::fmt::Debug for SECPKG_MUTUAL_AUTH_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_MUTUAL_AUTH_LEVEL").field("MutualAuthLevel", &self.MutualAuthLevel).finish()
    }
}
impl ::core::default::Default for SECPKG_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_NAME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SECPKG_NEGO2_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_NEGO2_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AuthScheme == other.AuthScheme && self.PackageFlags == other.PackageFlags
    }
}
impl ::core::cmp::Eq for SECPKG_NEGO2_INFO {}
impl ::core::fmt::Debug for SECPKG_NEGO2_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_NEGO2_INFO").field("AuthScheme", &self.AuthScheme).field("PackageFlags", &self.PackageFlags).finish()
    }
}
impl ::core::default::Default for SECPKG_PACKAGE_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_PACKAGE_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_PACKAGE_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.MachineState == other.MachineState && self.SetupMode == other.SetupMode && self.DomainSid == other.DomainSid && self.DomainName == other.DomainName && self.DnsDomainName == other.DnsDomainName && self.DomainGuid == other.DomainGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_PARAMETERS").field("Version", &self.Version).field("MachineState", &self.MachineState).field("SetupMode", &self.SetupMode).field("DomainSid", &self.DomainSid).field("DomainName", &self.DomainName).field("DnsDomainName", &self.DnsDomainName).field("DomainGuid", &self.DomainGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_POST_LOGON_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_POST_LOGON_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.LogonId == other.LogonId && self.LinkedLogonId == other.LinkedLogonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_POST_LOGON_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_POST_LOGON_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_POST_LOGON_USER_INFO").field("Flags", &self.Flags).field("LogonId", &self.LogonId).field("LinkedLogonId", &self.LinkedLogonId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_PRIMARY_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_PRIMARY_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.LogonId == other.LogonId && self.DownlevelName == other.DownlevelName && self.DomainName == other.DomainName && self.Password == other.Password && self.OldPassword == other.OldPassword && self.UserSid == other.UserSid && self.Flags == other.Flags && self.DnsDomainName == other.DnsDomainName && self.Upn == other.Upn && self.LogonServer == other.LogonServer && self.Spare1 == other.Spare1 && self.Spare2 == other.Spare2 && self.Spare3 == other.Spare3 && self.Spare4 == other.Spare4
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_PRIMARY_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_PRIMARY_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_PRIMARY_CRED")
            .field("LogonId", &self.LogonId)
            .field("DownlevelName", &self.DownlevelName)
            .field("DomainName", &self.DomainName)
            .field("Password", &self.Password)
            .field("OldPassword", &self.OldPassword)
            .field("UserSid", &self.UserSid)
            .field("Flags", &self.Flags)
            .field("DnsDomainName", &self.DnsDomainName)
            .field("Upn", &self.Upn)
            .field("LogonServer", &self.LogonServer)
            .field("Spare1", &self.Spare1)
            .field("Spare2", &self.Spare2)
            .field("Spare3", &self.Spare3)
            .field("Spare4", &self.Spare4)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_PRIMARY_CRED_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_PRIMARY_CRED_EX {
    fn eq(&self, other: &Self) -> bool {
        self.LogonId == other.LogonId && self.DownlevelName == other.DownlevelName && self.DomainName == other.DomainName && self.Password == other.Password && self.OldPassword == other.OldPassword && self.UserSid == other.UserSid && self.Flags == other.Flags && self.DnsDomainName == other.DnsDomainName && self.Upn == other.Upn && self.LogonServer == other.LogonServer && self.Spare1 == other.Spare1 && self.Spare2 == other.Spare2 && self.Spare3 == other.Spare3 && self.Spare4 == other.Spare4 && self.PackageId == other.PackageId && self.PrevLogonId == other.PrevLogonId && self.FlagsEx == other.FlagsEx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_PRIMARY_CRED_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_PRIMARY_CRED_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_PRIMARY_CRED_EX")
            .field("LogonId", &self.LogonId)
            .field("DownlevelName", &self.DownlevelName)
            .field("DomainName", &self.DomainName)
            .field("Password", &self.Password)
            .field("OldPassword", &self.OldPassword)
            .field("UserSid", &self.UserSid)
            .field("Flags", &self.Flags)
            .field("DnsDomainName", &self.DnsDomainName)
            .field("Upn", &self.Upn)
            .field("LogonServer", &self.LogonServer)
            .field("Spare1", &self.Spare1)
            .field("Spare2", &self.Spare2)
            .field("Spare3", &self.Spare3)
            .field("Spare4", &self.Spare4)
            .field("PackageId", &self.PackageId)
            .field("PrevLogonId", &self.PrevLogonId)
            .field("FlagsEx", &self.FlagsEx)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_REDIRECTED_LOGON_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SECPKG_SERIALIZED_OID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_SERIALIZED_OID {
    fn eq(&self, other: &Self) -> bool {
        self.OidLength == other.OidLength && self.OidAttributes == other.OidAttributes && self.OidValue == other.OidValue
    }
}
impl ::core::cmp::Eq for SECPKG_SERIALIZED_OID {}
impl ::core::fmt::Debug for SECPKG_SERIALIZED_OID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_SERIALIZED_OID").field("OidLength", &self.OidLength).field("OidAttributes", &self.OidAttributes).field("OidValue", &self.OidValue).finish()
    }
}
impl ::core::default::Default for SECPKG_SESSIONINFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECPKG_SESSIONINFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECPKG_SESSIONINFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SECPKG_SHORT_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_SHORT_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.ShortArrayOffset == other.ShortArrayOffset && self.ShortArrayCount == other.ShortArrayCount
    }
}
impl ::core::cmp::Eq for SECPKG_SHORT_VECTOR {}
impl ::core::fmt::Debug for SECPKG_SHORT_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_SHORT_VECTOR").field("ShortArrayOffset", &self.ShortArrayOffset).field("ShortArrayCount", &self.ShortArrayCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_SUPPLEMENTAL_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_SUPPLEMENTAL_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.PackageName == other.PackageName && self.CredentialSize == other.CredentialSize && self.Credentials == other.Credentials
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_SUPPLEMENTAL_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_SUPPLEMENTAL_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_SUPPLEMENTAL_CRED").field("PackageName", &self.PackageName).field("CredentialSize", &self.CredentialSize).field("Credentials", &self.Credentials).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.CredentialCount == other.CredentialCount && self.Credentials == other.Credentials
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_SUPPLEMENTAL_CRED_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_SUPPLEMENTAL_CRED_ARRAY").field("CredentialCount", &self.CredentialCount).field("Credentials", &self.Credentials).finish()
    }
}
impl ::core::default::Default for SECPKG_SUPPLIED_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_SUPPLIED_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.cbHeaderLength == other.cbHeaderLength && self.cbStructureLength == other.cbStructureLength && self.UserName == other.UserName && self.DomainName == other.DomainName && self.PackedCredentials == other.PackedCredentials && self.CredFlags == other.CredFlags
    }
}
impl ::core::cmp::Eq for SECPKG_SUPPLIED_CREDENTIAL {}
impl ::core::fmt::Debug for SECPKG_SUPPLIED_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_SUPPLIED_CREDENTIAL").field("cbHeaderLength", &self.cbHeaderLength).field("cbStructureLength", &self.cbStructureLength).field("UserName", &self.UserName).field("DomainName", &self.DomainName).field("PackedCredentials", &self.PackedCredentials).field("CredFlags", &self.CredFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_SURROGATE_LOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_SURROGATE_LOGON {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.SurrogateLogonID == other.SurrogateLogonID && self.EntryCount == other.EntryCount && self.Entries == other.Entries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_SURROGATE_LOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_SURROGATE_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_SURROGATE_LOGON").field("Version", &self.Version).field("SurrogateLogonID", &self.SurrogateLogonID).field("EntryCount", &self.EntryCount).field("Entries", &self.Entries).finish()
    }
}
impl ::core::default::Default for SECPKG_SURROGATE_LOGON_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECPKG_SURROGATE_LOGON_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for SECPKG_SURROGATE_LOGON_ENTRY {}
impl ::core::fmt::Debug for SECPKG_SURROGATE_LOGON_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_SURROGATE_LOGON_ENTRY").field("Type", &self.Type).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_TARGETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_TARGETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.DomainSid == other.DomainSid && self.ComputerName == other.ComputerName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_TARGETINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_TARGETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_TARGETINFO").field("DomainSid", &self.DomainSid).field("ComputerName", &self.ComputerName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_USER_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECPKG_WOW_CLIENT_DLL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECPKG_WOW_CLIENT_DLL {
    fn eq(&self, other: &Self) -> bool {
        self.WowClientDllPath == other.WowClientDllPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECPKG_WOW_CLIENT_DLL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECPKG_WOW_CLIENT_DLL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECPKG_WOW_CLIENT_DLL").field("WowClientDllPath", &self.WowClientDllPath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_LOGON_SESSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_LOGON_SESSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.LogonId == other.LogonId
            && self.UserName == other.UserName
            && self.LogonDomain == other.LogonDomain
            && self.AuthenticationPackage == other.AuthenticationPackage
            && self.LogonType == other.LogonType
            && self.Session == other.Session
            && self.Sid == other.Sid
            && self.LogonTime == other.LogonTime
            && self.LogonServer == other.LogonServer
            && self.DnsDomainName == other.DnsDomainName
            && self.Upn == other.Upn
            && self.UserFlags == other.UserFlags
            && self.LastLogonInfo == other.LastLogonInfo
            && self.LogonScript == other.LogonScript
            && self.ProfilePath == other.ProfilePath
            && self.HomeDirectory == other.HomeDirectory
            && self.HomeDirectoryDrive == other.HomeDirectoryDrive
            && self.LogoffTime == other.LogoffTime
            && self.KickOffTime == other.KickOffTime
            && self.PasswordLastSet == other.PasswordLastSet
            && self.PasswordCanChange == other.PasswordCanChange
            && self.PasswordMustChange == other.PasswordMustChange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_LOGON_SESSION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_LOGON_SESSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_LOGON_SESSION_DATA")
            .field("Size", &self.Size)
            .field("LogonId", &self.LogonId)
            .field("UserName", &self.UserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("AuthenticationPackage", &self.AuthenticationPackage)
            .field("LogonType", &self.LogonType)
            .field("Session", &self.Session)
            .field("Sid", &self.Sid)
            .field("LogonTime", &self.LogonTime)
            .field("LogonServer", &self.LogonServer)
            .field("DnsDomainName", &self.DnsDomainName)
            .field("Upn", &self.Upn)
            .field("UserFlags", &self.UserFlags)
            .field("LastLogonInfo", &self.LastLogonInfo)
            .field("LogonScript", &self.LogonScript)
            .field("ProfilePath", &self.ProfilePath)
            .field("HomeDirectory", &self.HomeDirectory)
            .field("HomeDirectoryDrive", &self.HomeDirectoryDrive)
            .field("LogoffTime", &self.LogoffTime)
            .field("KickOffTime", &self.KickOffTime)
            .field("PasswordLastSet", &self.PasswordLastSet)
            .field("PasswordCanChange", &self.PasswordCanChange)
            .field("PasswordMustChange", &self.PasswordMustChange)
            .finish()
    }
}
impl ::core::default::Default for SECURITY_LOGON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECURITY_LOGON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_LOGON_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SECURITY_PACKAGE_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECURITY_PACKAGE_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type && self.Flags == other.Flags && self.SignatureSize == other.SignatureSize && self.Signature == other.Signature
    }
}
impl ::core::cmp::Eq for SECURITY_PACKAGE_OPTIONS {}
impl ::core::fmt::Debug for SECURITY_PACKAGE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_PACKAGE_OPTIONS").field("Size", &self.Size).field("Type", &self.Type).field("Flags", &self.Flags).field("SignatureSize", &self.SignatureSize).field("Signature", &self.Signature).finish()
    }
}
impl ::core::default::Default for SECURITY_PACKAGE_OPTIONS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECURITY_PACKAGE_OPTIONS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_PACKAGE_OPTIONS_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_USER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_USER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.LogonDomainName == other.LogonDomainName && self.LogonServer == other.LogonServer && self.pSid == other.pSid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_USER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_USER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_USER_DATA").field("UserName", &self.UserName).field("LogonDomainName", &self.LogonDomainName).field("LogonServer", &self.LogonServer).field("pSid", &self.pSid).finish()
    }
}
impl ::core::default::Default for SEC_APPLICATION_PROTOCOLS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_APPLICATION_PROTOCOLS {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolListsSize == other.ProtocolListsSize && self.ProtocolLists == other.ProtocolLists
    }
}
impl ::core::cmp::Eq for SEC_APPLICATION_PROTOCOLS {}
impl ::core::fmt::Debug for SEC_APPLICATION_PROTOCOLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_APPLICATION_PROTOCOLS").field("ProtocolListsSize", &self.ProtocolListsSize).field("ProtocolLists", &self.ProtocolLists).finish()
    }
}
impl ::core::default::Default for SEC_APPLICATION_PROTOCOL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_APPLICATION_PROTOCOL_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.ProtoNegoExt == other.ProtoNegoExt && self.ProtocolListSize == other.ProtocolListSize && self.ProtocolList == other.ProtocolList
    }
}
impl ::core::cmp::Eq for SEC_APPLICATION_PROTOCOL_LIST {}
impl ::core::fmt::Debug for SEC_APPLICATION_PROTOCOL_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_APPLICATION_PROTOCOL_LIST").field("ProtoNegoExt", &self.ProtoNegoExt).field("ProtocolListSize", &self.ProtocolListSize).field("ProtocolList", &self.ProtocolList).finish()
    }
}
impl ::core::default::Default for SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEC_CHANNEL_BINDINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_CHANNEL_BINDINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwInitiatorAddrType == other.dwInitiatorAddrType && self.cbInitiatorLength == other.cbInitiatorLength && self.dwInitiatorOffset == other.dwInitiatorOffset && self.dwAcceptorAddrType == other.dwAcceptorAddrType && self.cbAcceptorLength == other.cbAcceptorLength && self.dwAcceptorOffset == other.dwAcceptorOffset && self.cbApplicationDataLength == other.cbApplicationDataLength && self.dwApplicationDataOffset == other.dwApplicationDataOffset
    }
}
impl ::core::cmp::Eq for SEC_CHANNEL_BINDINGS {}
impl ::core::fmt::Debug for SEC_CHANNEL_BINDINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_CHANNEL_BINDINGS")
            .field("dwInitiatorAddrType", &self.dwInitiatorAddrType)
            .field("cbInitiatorLength", &self.cbInitiatorLength)
            .field("dwInitiatorOffset", &self.dwInitiatorOffset)
            .field("dwAcceptorAddrType", &self.dwAcceptorAddrType)
            .field("cbAcceptorLength", &self.cbAcceptorLength)
            .field("dwAcceptorOffset", &self.dwAcceptorOffset)
            .field("cbApplicationDataLength", &self.cbApplicationDataLength)
            .field("dwApplicationDataOffset", &self.dwApplicationDataOffset)
            .finish()
    }
}
impl ::core::default::Default for SEC_DTLS_MTU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_DTLS_MTU {
    fn eq(&self, other: &Self) -> bool {
        self.PathMTU == other.PathMTU
    }
}
impl ::core::cmp::Eq for SEC_DTLS_MTU {}
impl ::core::fmt::Debug for SEC_DTLS_MTU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_DTLS_MTU").field("PathMTU", &self.PathMTU).finish()
    }
}
impl ::core::default::Default for SEC_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SEC_FLAGS {}
impl ::core::fmt::Debug for SEC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_FLAGS").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for SEC_NEGOTIATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_NEGOTIATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.NameLength == other.NameLength && self.Name == other.Name && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for SEC_NEGOTIATION_INFO {}
impl ::core::fmt::Debug for SEC_NEGOTIATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_NEGOTIATION_INFO").field("Size", &self.Size).field("NameLength", &self.NameLength).field("Name", &self.Name).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for SEC_PRESHAREDKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_PRESHAREDKEY {
    fn eq(&self, other: &Self) -> bool {
        self.KeySize == other.KeySize && self.Key == other.Key
    }
}
impl ::core::cmp::Eq for SEC_PRESHAREDKEY {}
impl ::core::fmt::Debug for SEC_PRESHAREDKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_PRESHAREDKEY").field("KeySize", &self.KeySize).field("Key", &self.Key).finish()
    }
}
impl ::core::default::Default for SEC_PRESHAREDKEY_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_PRESHAREDKEY_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.KeyIdentitySize == other.KeyIdentitySize && self.KeyIdentity == other.KeyIdentity
    }
}
impl ::core::cmp::Eq for SEC_PRESHAREDKEY_IDENTITY {}
impl ::core::fmt::Debug for SEC_PRESHAREDKEY_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_PRESHAREDKEY_IDENTITY").field("KeyIdentitySize", &self.KeyIdentitySize).field("KeyIdentity", &self.KeyIdentity).finish()
    }
}
impl ::core::default::Default for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.MasterKeyIdentifierSize == other.MasterKeyIdentifierSize && self.MasterKeyIdentifier == other.MasterKeyIdentifier
    }
}
impl ::core::cmp::Eq for SEC_SRTP_MASTER_KEY_IDENTIFIER {}
impl ::core::fmt::Debug for SEC_SRTP_MASTER_KEY_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_SRTP_MASTER_KEY_IDENTIFIER").field("MasterKeyIdentifierSize", &self.MasterKeyIdentifierSize).field("MasterKeyIdentifier", &self.MasterKeyIdentifier).finish()
    }
}
impl ::core::default::Default for SEC_SRTP_PROTECTION_PROFILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_SRTP_PROTECTION_PROFILES {
    fn eq(&self, other: &Self) -> bool {
        self.ProfilesSize == other.ProfilesSize && self.ProfilesList == other.ProfilesList
    }
}
impl ::core::cmp::Eq for SEC_SRTP_PROTECTION_PROFILES {}
impl ::core::fmt::Debug for SEC_SRTP_PROTECTION_PROFILES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_SRTP_PROTECTION_PROFILES").field("ProfilesSize", &self.ProfilesSize).field("ProfilesList", &self.ProfilesList).finish()
    }
}
impl ::core::default::Default for SEC_TOKEN_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_TOKEN_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.KeyParametersSize == other.KeyParametersSize && self.KeyParameters == other.KeyParameters
    }
}
impl ::core::cmp::Eq for SEC_TOKEN_BINDING {}
impl ::core::fmt::Debug for SEC_TOKEN_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_TOKEN_BINDING").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("KeyParametersSize", &self.KeyParametersSize).field("KeyParameters", &self.KeyParameters).finish()
    }
}
impl ::core::default::Default for SEC_TRAFFIC_SECRETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_TRAFFIC_SECRETS {
    fn eq(&self, other: &Self) -> bool {
        self.SymmetricAlgId == other.SymmetricAlgId && self.ChainingMode == other.ChainingMode && self.HashAlgId == other.HashAlgId && self.KeySize == other.KeySize && self.IvSize == other.IvSize && self.MsgSequenceStart == other.MsgSequenceStart && self.MsgSequenceEnd == other.MsgSequenceEnd && self.TrafficSecretType == other.TrafficSecretType && self.TrafficSecretSize == other.TrafficSecretSize && self.TrafficSecret == other.TrafficSecret
    }
}
impl ::core::cmp::Eq for SEC_TRAFFIC_SECRETS {}
impl ::core::fmt::Debug for SEC_TRAFFIC_SECRETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_TRAFFIC_SECRETS").field("SymmetricAlgId", &self.SymmetricAlgId).field("ChainingMode", &self.ChainingMode).field("HashAlgId", &self.HashAlgId).field("KeySize", &self.KeySize).field("IvSize", &self.IvSize).field("MsgSequenceStart", &self.MsgSequenceStart).field("MsgSequenceEnd", &self.MsgSequenceEnd).field("TrafficSecretType", &self.TrafficSecretType).field("TrafficSecretSize", &self.TrafficSecretSize).field("TrafficSecret", &self.TrafficSecret).finish()
    }
}
impl ::core::default::Default for SEC_TRAFFIC_SECRET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEC_TRAFFIC_SECRET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEC_TRAFFIC_SECRET_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY32 {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY32 {}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY32").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_EX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_EX2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.cbHeaderLength == other.cbHeaderLength && self.cbStructureLength == other.cbStructureLength && self.UserOffset == other.UserOffset && self.UserLength == other.UserLength && self.DomainOffset == other.DomainOffset && self.DomainLength == other.DomainLength && self.PackedCredentialsOffset == other.PackedCredentialsOffset && self.PackedCredentialsLength == other.PackedCredentialsLength && self.Flags == other.Flags && self.PackageListOffset == other.PackageListOffset && self.PackageListLength == other.PackageListLength
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_EX2 {}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_EX2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_EX2")
            .field("Version", &self.Version)
            .field("cbHeaderLength", &self.cbHeaderLength)
            .field("cbStructureLength", &self.cbStructureLength)
            .field("UserOffset", &self.UserOffset)
            .field("UserLength", &self.UserLength)
            .field("DomainOffset", &self.DomainOffset)
            .field("DomainLength", &self.DomainLength)
            .field("PackedCredentialsOffset", &self.PackedCredentialsOffset)
            .field("PackedCredentialsLength", &self.PackedCredentialsLength)
            .field("Flags", &self.Flags)
            .field("PackageListOffset", &self.PackageListOffset)
            .field("PackageListLength", &self.PackageListLength)
            .finish()
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_EX32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_EX32 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags && self.PackageList == other.PackageList && self.PackageListLength == other.PackageListLength
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_EX32 {}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_EX32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_EX32").field("Version", &self.Version).field("Length", &self.Length).field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).field("PackageList", &self.PackageList).field("PackageListLength", &self.PackageListLength).finish()
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_EXA {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags && self.PackageList == other.PackageList && self.PackageListLength == other.PackageListLength
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_EXA {}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_EXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_EXA").field("Version", &self.Version).field("Length", &self.Length).field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).field("PackageList", &self.PackageList).field("PackageListLength", &self.PackageListLength).finish()
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_EXW {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags && self.PackageList == other.PackageList && self.PackageListLength == other.PackageListLength
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_EXW {}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_EXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_EXW").field("Version", &self.Version).field("Length", &self.Length).field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).field("PackageList", &self.PackageList).field("PackageListLength", &self.PackageListLength).finish()
    }
}
#[cfg(feature = "Win32_System_Rpc")]
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SEND_GENERIC_TLS_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEND_GENERIC_TLS_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.ExtensionType == other.ExtensionType && self.HandshakeType == other.HandshakeType && self.Flags == other.Flags && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for SEND_GENERIC_TLS_EXTENSION {}
impl ::core::fmt::Debug for SEND_GENERIC_TLS_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEND_GENERIC_TLS_EXTENSION").field("ExtensionType", &self.ExtensionType).field("HandshakeType", &self.HandshakeType).field("Flags", &self.Flags).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for SE_ADT_ACCESS_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SE_ADT_ACCESS_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.AccessMask == other.AccessMask && self.AccessReasons == other.AccessReasons && self.ObjectTypeIndex == other.ObjectTypeIndex && self.AccessGranted == other.AccessGranted && self.SecurityDescriptor == other.SecurityDescriptor
    }
}
impl ::core::cmp::Eq for SE_ADT_ACCESS_REASON {}
impl ::core::fmt::Debug for SE_ADT_ACCESS_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ADT_ACCESS_REASON").field("AccessMask", &self.AccessMask).field("AccessReasons", &self.AccessReasons).field("ObjectTypeIndex", &self.ObjectTypeIndex).field("AccessGranted", &self.AccessGranted).field("SecurityDescriptor", &self.SecurityDescriptor).finish()
    }
}
impl ::core::default::Default for SE_ADT_CLAIMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SE_ADT_CLAIMS {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Claims == other.Claims
    }
}
impl ::core::cmp::Eq for SE_ADT_CLAIMS {}
impl ::core::fmt::Debug for SE_ADT_CLAIMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ADT_CLAIMS").field("Length", &self.Length).field("Claims", &self.Claims).finish()
    }
}
impl ::core::default::Default for SE_ADT_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SE_ADT_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectType == other.ObjectType && self.Flags == other.Flags && self.Level == other.Level && self.AccessMask == other.AccessMask
    }
}
impl ::core::cmp::Eq for SE_ADT_OBJECT_TYPE {}
impl ::core::fmt::Debug for SE_ADT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ADT_OBJECT_TYPE").field("ObjectType", &self.ObjectType).field("Flags", &self.Flags).field("Level", &self.Level).field("AccessMask", &self.AccessMask).finish()
    }
}
impl ::core::default::Default for SE_ADT_PARAMETER_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SE_ADT_PARAMETER_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.CategoryId == other.CategoryId && self.AuditId == other.AuditId && self.ParameterCount == other.ParameterCount && self.Length == other.Length && self.FlatSubCategoryId == other.FlatSubCategoryId && self.Type == other.Type && self.Flags == other.Flags && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for SE_ADT_PARAMETER_ARRAY {}
impl ::core::fmt::Debug for SE_ADT_PARAMETER_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ADT_PARAMETER_ARRAY").field("CategoryId", &self.CategoryId).field("AuditId", &self.AuditId).field("ParameterCount", &self.ParameterCount).field("Length", &self.Length).field("FlatSubCategoryId", &self.FlatSubCategoryId).field("Type", &self.Type).field("Flags", &self.Flags).field("Parameters", &self.Parameters).finish()
    }
}
impl ::core::default::Default for SE_ADT_PARAMETER_ARRAY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SE_ADT_PARAMETER_ARRAY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Length == other.Length && self.Data == other.Data && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for SE_ADT_PARAMETER_ARRAY_ENTRY {}
impl ::core::fmt::Debug for SE_ADT_PARAMETER_ARRAY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ADT_PARAMETER_ARRAY_ENTRY").field("Type", &self.Type).field("Length", &self.Length).field("Data", &self.Data).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for SE_ADT_PARAMETER_ARRAY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SE_ADT_PARAMETER_ARRAY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.CategoryId == other.CategoryId && self.AuditId == other.AuditId && self.Version == other.Version && self.ParameterCount == other.ParameterCount && self.Length == other.Length && self.FlatSubCategoryId == other.FlatSubCategoryId && self.Type == other.Type && self.Flags == other.Flags && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for SE_ADT_PARAMETER_ARRAY_EX {}
impl ::core::fmt::Debug for SE_ADT_PARAMETER_ARRAY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ADT_PARAMETER_ARRAY_EX").field("CategoryId", &self.CategoryId).field("AuditId", &self.AuditId).field("Version", &self.Version).field("ParameterCount", &self.ParameterCount).field("Length", &self.Length).field("FlatSubCategoryId", &self.FlatSubCategoryId).field("Type", &self.Type).field("Flags", &self.Flags).field("Parameters", &self.Parameters).finish()
    }
}
impl ::core::default::Default for SE_ADT_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SE_ADT_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SE_ADT_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SLDATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SLDATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SLDATATYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SLIDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SLIDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SLIDTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SLLICENSINGSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SLLICENSINGSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SLLICENSINGSTATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SLREFERRALTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SLREFERRALTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SLREFERRALTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SL_ACTIVATION_INFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SL_ACTIVATION_INFO_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.r#type == other.r#type
    }
}
impl ::core::cmp::Eq for SL_ACTIVATION_INFO_HEADER {}
impl ::core::fmt::Debug for SL_ACTIVATION_INFO_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SL_ACTIVATION_INFO_HEADER").field("cbSize", &self.cbSize).field("type", &self.r#type).finish()
    }
}
impl ::core::default::Default for SL_ACTIVATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SL_ACTIVATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SL_ACTIVATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SL_AD_ACTIVATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SL_AD_ACTIVATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pwszProductKey == other.pwszProductKey && self.pwszActivationObjectName == other.pwszActivationObjectName
    }
}
impl ::core::cmp::Eq for SL_AD_ACTIVATION_INFO {}
impl ::core::fmt::Debug for SL_AD_ACTIVATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SL_AD_ACTIVATION_INFO").field("header", &self.header).field("pwszProductKey", &self.pwszProductKey).field("pwszActivationObjectName", &self.pwszActivationObjectName).finish()
    }
}
impl ::core::default::Default for SL_GENUINE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SL_GENUINE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SL_GENUINE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SL_LICENSING_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SL_LICENSING_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.SkuId == other.SkuId && self.eStatus == other.eStatus && self.dwGraceTime == other.dwGraceTime && self.dwTotalGraceDays == other.dwTotalGraceDays && self.hrReason == other.hrReason && self.qwValidityExpiration == other.qwValidityExpiration
    }
}
impl ::core::cmp::Eq for SL_LICENSING_STATUS {}
impl ::core::fmt::Debug for SL_LICENSING_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SL_LICENSING_STATUS").field("SkuId", &self.SkuId).field("eStatus", &self.eStatus).field("dwGraceTime", &self.dwGraceTime).field("dwTotalGraceDays", &self.dwTotalGraceDays).field("hrReason", &self.hrReason).field("qwValidityExpiration", &self.qwValidityExpiration).finish()
    }
}
impl ::core::default::Default for SL_NONGENUINE_UI_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SL_NONGENUINE_UI_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pComponentId == other.pComponentId && self.hResultUI == other.hResultUI
    }
}
impl ::core::cmp::Eq for SL_NONGENUINE_UI_OPTIONS {}
impl ::core::fmt::Debug for SL_NONGENUINE_UI_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SL_NONGENUINE_UI_OPTIONS").field("cbSize", &self.cbSize).field("pComponentId", &self.pComponentId).field("hResultUI", &self.hResultUI).finish()
    }
}
impl ::core::default::Default for SL_SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SL_SYSTEM_POLICY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for SL_SYSTEM_POLICY_INFORMATION {}
impl ::core::fmt::Debug for SL_SYSTEM_POLICY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SL_SYSTEM_POLICY_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for SR_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SR_SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.SecurityDescriptor == other.SecurityDescriptor
    }
}
impl ::core::cmp::Eq for SR_SECURITY_DESCRIPTOR {}
impl ::core::fmt::Debug for SR_SECURITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SR_SECURITY_DESCRIPTOR").field("Length", &self.Length).field("SecurityDescriptor", &self.SecurityDescriptor).finish()
    }
}
impl ::core::default::Default for SSL_CREDENTIAL_CERTIFICATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSL_CREDENTIAL_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbPrivateKey == other.cbPrivateKey && self.pPrivateKey == other.pPrivateKey && self.cbCertificate == other.cbCertificate && self.pCertificate == other.pCertificate && self.pszPassword == other.pszPassword
    }
}
impl ::core::cmp::Eq for SSL_CREDENTIAL_CERTIFICATE {}
impl ::core::fmt::Debug for SSL_CREDENTIAL_CERTIFICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSL_CREDENTIAL_CERTIFICATE").field("cbPrivateKey", &self.cbPrivateKey).field("pPrivateKey", &self.pPrivateKey).field("cbCertificate", &self.cbCertificate).field("pCertificate", &self.pCertificate).field("pszPassword", &self.pszPassword).finish()
    }
}
impl ::core::default::Default for SUBSCRIBE_GENERIC_TLS_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SUBSCRIBE_GENERIC_TLS_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.SubscriptionsCount == other.SubscriptionsCount && self.Subscriptions == other.Subscriptions
    }
}
impl ::core::cmp::Eq for SUBSCRIBE_GENERIC_TLS_EXTENSION {}
impl ::core::fmt::Debug for SUBSCRIBE_GENERIC_TLS_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SUBSCRIBE_GENERIC_TLS_EXTENSION").field("Flags", &self.Flags).field("SubscriptionsCount", &self.SubscriptionsCount).field("Subscriptions", &self.Subscriptions).finish()
    }
}
impl ::core::default::Default for SchGetExtensionsOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SchGetExtensionsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SchGetExtensionsOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SchGetExtensionsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SchGetExtensionsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SchGetExtensionsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SchGetExtensionsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SchGetExtensionsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SecBuffer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.cbBuffer == other.cbBuffer && self.BufferType == other.BufferType && self.pvBuffer == other.pvBuffer
    }
}
impl ::core::cmp::Eq for SecBuffer {}
impl ::core::fmt::Debug for SecBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecBuffer").field("cbBuffer", &self.cbBuffer).field("BufferType", &self.BufferType).field("pvBuffer", &self.pvBuffer).finish()
    }
}
impl ::core::default::Default for SecBufferDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecBufferDesc {
    fn eq(&self, other: &Self) -> bool {
        self.ulVersion == other.ulVersion && self.cBuffers == other.cBuffers && self.pBuffers == other.pBuffers
    }
}
impl ::core::cmp::Eq for SecBufferDesc {}
impl ::core::fmt::Debug for SecBufferDesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecBufferDesc").field("ulVersion", &self.ulVersion).field("cBuffers", &self.cBuffers).field("pBuffers", &self.pBuffers).finish()
    }
}
impl ::core::default::Default for SecDelegationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SecDelegationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecDelegationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SecPkgContext_AccessToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_AccessToken {
    fn eq(&self, other: &Self) -> bool {
        self.AccessToken == other.AccessToken
    }
}
impl ::core::cmp::Eq for SecPkgContext_AccessToken {}
impl ::core::fmt::Debug for SecPkgContext_AccessToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_AccessToken").field("AccessToken", &self.AccessToken).finish()
    }
}
impl ::core::default::Default for SecPkgContext_ApplicationProtocol {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_ApplicationProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.ProtoNegoStatus == other.ProtoNegoStatus && self.ProtoNegoExt == other.ProtoNegoExt && self.ProtocolIdSize == other.ProtocolIdSize && self.ProtocolId == other.ProtocolId
    }
}
impl ::core::cmp::Eq for SecPkgContext_ApplicationProtocol {}
impl ::core::fmt::Debug for SecPkgContext_ApplicationProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ApplicationProtocol").field("ProtoNegoStatus", &self.ProtoNegoStatus).field("ProtoNegoExt", &self.ProtoNegoExt).field("ProtocolIdSize", &self.ProtocolIdSize).field("ProtocolId", &self.ProtocolId).finish()
    }
}
impl ::core::default::Default for SecPkgContext_AuthorityA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_AuthorityA {
    fn eq(&self, other: &Self) -> bool {
        self.sAuthorityName == other.sAuthorityName
    }
}
impl ::core::cmp::Eq for SecPkgContext_AuthorityA {}
impl ::core::fmt::Debug for SecPkgContext_AuthorityA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_AuthorityA").field("sAuthorityName", &self.sAuthorityName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_AuthorityW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_AuthorityW {
    fn eq(&self, other: &Self) -> bool {
        self.sAuthorityName == other.sAuthorityName
    }
}
impl ::core::cmp::Eq for SecPkgContext_AuthorityW {}
impl ::core::fmt::Debug for SecPkgContext_AuthorityW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_AuthorityW").field("sAuthorityName", &self.sAuthorityName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_AuthzID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_AuthzID {
    fn eq(&self, other: &Self) -> bool {
        self.AuthzIDLength == other.AuthzIDLength && self.AuthzID == other.AuthzID
    }
}
impl ::core::cmp::Eq for SecPkgContext_AuthzID {}
impl ::core::fmt::Debug for SecPkgContext_AuthzID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_AuthzID").field("AuthzIDLength", &self.AuthzIDLength).field("AuthzID", &self.AuthzID).finish()
    }
}
impl ::core::default::Default for SecPkgContext_Bindings {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_Bindings {
    fn eq(&self, other: &Self) -> bool {
        self.BindingsLength == other.BindingsLength && self.Bindings == other.Bindings
    }
}
impl ::core::cmp::Eq for SecPkgContext_Bindings {}
impl ::core::fmt::Debug for SecPkgContext_Bindings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_Bindings").field("BindingsLength", &self.BindingsLength).field("Bindings", &self.Bindings).finish()
    }
}
impl ::core::default::Default for SecPkgContext_CertInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_CertInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbSubjectName == other.cbSubjectName && self.pwszSubjectName == other.pwszSubjectName && self.cbIssuerName == other.cbIssuerName && self.pwszIssuerName == other.pwszIssuerName && self.dwKeySize == other.dwKeySize
    }
}
impl ::core::cmp::Eq for SecPkgContext_CertInfo {}
impl ::core::fmt::Debug for SecPkgContext_CertInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_CertInfo").field("dwVersion", &self.dwVersion).field("cbSubjectName", &self.cbSubjectName).field("pwszSubjectName", &self.pwszSubjectName).field("cbIssuerName", &self.cbIssuerName).field("pwszIssuerName", &self.pwszIssuerName).field("dwKeySize", &self.dwKeySize).finish()
    }
}
impl ::core::default::Default for SecPkgContext_CertificateValidationResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_CertificateValidationResult {
    fn eq(&self, other: &Self) -> bool {
        self.dwChainErrorStatus == other.dwChainErrorStatus && self.hrVerifyChainStatus == other.hrVerifyChainStatus
    }
}
impl ::core::cmp::Eq for SecPkgContext_CertificateValidationResult {}
impl ::core::fmt::Debug for SecPkgContext_CertificateValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_CertificateValidationResult").field("dwChainErrorStatus", &self.dwChainErrorStatus).field("hrVerifyChainStatus", &self.hrVerifyChainStatus).finish()
    }
}
impl ::core::default::Default for SecPkgContext_Certificates {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_Certificates {
    fn eq(&self, other: &Self) -> bool {
        self.cCertificates == other.cCertificates && self.cbCertificateChain == other.cbCertificateChain && self.pbCertificateChain == other.pbCertificateChain
    }
}
impl ::core::cmp::Eq for SecPkgContext_Certificates {}
impl ::core::fmt::Debug for SecPkgContext_Certificates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_Certificates").field("cCertificates", &self.cCertificates).field("cbCertificateChain", &self.cbCertificateChain).field("pbCertificateChain", &self.pbCertificateChain).finish()
    }
}
impl ::core::default::Default for SecPkgContext_CipherInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_CipherInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwProtocol == other.dwProtocol && self.dwCipherSuite == other.dwCipherSuite && self.dwBaseCipherSuite == other.dwBaseCipherSuite && self.szCipherSuite == other.szCipherSuite && self.szCipher == other.szCipher && self.dwCipherLen == other.dwCipherLen && self.dwCipherBlockLen == other.dwCipherBlockLen && self.szHash == other.szHash && self.dwHashLen == other.dwHashLen && self.szExchange == other.szExchange && self.dwMinExchangeLen == other.dwMinExchangeLen && self.dwMaxExchangeLen == other.dwMaxExchangeLen && self.szCertificate == other.szCertificate && self.dwKeyType == other.dwKeyType
    }
}
impl ::core::cmp::Eq for SecPkgContext_CipherInfo {}
impl ::core::fmt::Debug for SecPkgContext_CipherInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_CipherInfo")
            .field("dwVersion", &self.dwVersion)
            .field("dwProtocol", &self.dwProtocol)
            .field("dwCipherSuite", &self.dwCipherSuite)
            .field("dwBaseCipherSuite", &self.dwBaseCipherSuite)
            .field("szCipherSuite", &self.szCipherSuite)
            .field("szCipher", &self.szCipher)
            .field("dwCipherLen", &self.dwCipherLen)
            .field("dwCipherBlockLen", &self.dwCipherBlockLen)
            .field("szHash", &self.szHash)
            .field("dwHashLen", &self.dwHashLen)
            .field("szExchange", &self.szExchange)
            .field("dwMinExchangeLen", &self.dwMinExchangeLen)
            .field("dwMaxExchangeLen", &self.dwMaxExchangeLen)
            .field("szCertificate", &self.szCertificate)
            .field("dwKeyType", &self.dwKeyType)
            .finish()
    }
}
impl ::core::default::Default for SecPkgContext_ClientCertPolicyResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_ClientCertPolicyResult {
    fn eq(&self, other: &Self) -> bool {
        self.dwPolicyResult == other.dwPolicyResult && self.guidPolicyId == other.guidPolicyId
    }
}
impl ::core::cmp::Eq for SecPkgContext_ClientCertPolicyResult {}
impl ::core::fmt::Debug for SecPkgContext_ClientCertPolicyResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ClientCertPolicyResult").field("dwPolicyResult", &self.dwPolicyResult).field("guidPolicyId", &self.guidPolicyId).finish()
    }
}
impl ::core::default::Default for SecPkgContext_ClientSpecifiedTarget {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_ClientSpecifiedTarget {
    fn eq(&self, other: &Self) -> bool {
        self.sTargetName == other.sTargetName
    }
}
impl ::core::cmp::Eq for SecPkgContext_ClientSpecifiedTarget {}
impl ::core::fmt::Debug for SecPkgContext_ClientSpecifiedTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ClientSpecifiedTarget").field("sTargetName", &self.sTargetName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_ConnectionInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_ConnectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwProtocol == other.dwProtocol && self.aiCipher == other.aiCipher && self.dwCipherStrength == other.dwCipherStrength && self.aiHash == other.aiHash && self.dwHashStrength == other.dwHashStrength && self.aiExch == other.aiExch && self.dwExchStrength == other.dwExchStrength
    }
}
impl ::core::cmp::Eq for SecPkgContext_ConnectionInfo {}
impl ::core::fmt::Debug for SecPkgContext_ConnectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ConnectionInfo").field("dwProtocol", &self.dwProtocol).field("aiCipher", &self.aiCipher).field("dwCipherStrength", &self.dwCipherStrength).field("aiHash", &self.aiHash).field("dwHashStrength", &self.dwHashStrength).field("aiExch", &self.aiExch).field("dwExchStrength", &self.dwExchStrength).finish()
    }
}
impl ::core::default::Default for SecPkgContext_ConnectionInfoEx {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_ConnectionInfoEx {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwProtocol == other.dwProtocol && self.szCipher == other.szCipher && self.dwCipherStrength == other.dwCipherStrength && self.szHash == other.szHash && self.dwHashStrength == other.dwHashStrength && self.szExchange == other.szExchange && self.dwExchStrength == other.dwExchStrength
    }
}
impl ::core::cmp::Eq for SecPkgContext_ConnectionInfoEx {}
impl ::core::fmt::Debug for SecPkgContext_ConnectionInfoEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ConnectionInfoEx").field("dwVersion", &self.dwVersion).field("dwProtocol", &self.dwProtocol).field("szCipher", &self.szCipher).field("dwCipherStrength", &self.dwCipherStrength).field("szHash", &self.szHash).field("dwHashStrength", &self.dwHashStrength).field("szExchange", &self.szExchange).field("dwExchStrength", &self.dwExchStrength).finish()
    }
}
impl ::core::default::Default for SecPkgContext_CredInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_CredInfo {
    fn eq(&self, other: &Self) -> bool {
        self.CredClass == other.CredClass && self.IsPromptingNeeded == other.IsPromptingNeeded
    }
}
impl ::core::cmp::Eq for SecPkgContext_CredInfo {}
impl ::core::fmt::Debug for SecPkgContext_CredInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_CredInfo").field("CredClass", &self.CredClass).field("IsPromptingNeeded", &self.IsPromptingNeeded).finish()
    }
}
impl ::core::default::Default for SecPkgContext_CredentialNameA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_CredentialNameA {
    fn eq(&self, other: &Self) -> bool {
        self.CredentialType == other.CredentialType && self.sCredentialName == other.sCredentialName
    }
}
impl ::core::cmp::Eq for SecPkgContext_CredentialNameA {}
impl ::core::fmt::Debug for SecPkgContext_CredentialNameA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_CredentialNameA").field("CredentialType", &self.CredentialType).field("sCredentialName", &self.sCredentialName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_CredentialNameW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_CredentialNameW {
    fn eq(&self, other: &Self) -> bool {
        self.CredentialType == other.CredentialType && self.sCredentialName == other.sCredentialName
    }
}
impl ::core::cmp::Eq for SecPkgContext_CredentialNameW {}
impl ::core::fmt::Debug for SecPkgContext_CredentialNameW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_CredentialNameW").field("CredentialType", &self.CredentialType).field("sCredentialName", &self.sCredentialName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_DceInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_DceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.AuthzSvc == other.AuthzSvc && self.pPac == other.pPac
    }
}
impl ::core::cmp::Eq for SecPkgContext_DceInfo {}
impl ::core::fmt::Debug for SecPkgContext_DceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_DceInfo").field("AuthzSvc", &self.AuthzSvc).field("pPac", &self.pPac).finish()
    }
}
impl ::core::default::Default for SecPkgContext_EapKeyBlock {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_EapKeyBlock {
    fn eq(&self, other: &Self) -> bool {
        self.rgbKeys == other.rgbKeys && self.rgbIVs == other.rgbIVs
    }
}
impl ::core::cmp::Eq for SecPkgContext_EapKeyBlock {}
impl ::core::fmt::Debug for SecPkgContext_EapKeyBlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_EapKeyBlock").field("rgbKeys", &self.rgbKeys).field("rgbIVs", &self.rgbIVs).finish()
    }
}
impl ::core::default::Default for SecPkgContext_EapPrfInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_EapPrfInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbPrfData == other.cbPrfData && self.pbPrfData == other.pbPrfData
    }
}
impl ::core::cmp::Eq for SecPkgContext_EapPrfInfo {}
impl ::core::fmt::Debug for SecPkgContext_EapPrfInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_EapPrfInfo").field("dwVersion", &self.dwVersion).field("cbPrfData", &self.cbPrfData).field("pbPrfData", &self.pbPrfData).finish()
    }
}
impl ::core::default::Default for SecPkgContext_EarlyStart {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_EarlyStart {
    fn eq(&self, other: &Self) -> bool {
        self.dwEarlyStartFlags == other.dwEarlyStartFlags
    }
}
impl ::core::cmp::Eq for SecPkgContext_EarlyStart {}
impl ::core::fmt::Debug for SecPkgContext_EarlyStart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_EarlyStart").field("dwEarlyStartFlags", &self.dwEarlyStartFlags).finish()
    }
}
impl ::core::default::Default for SecPkgContext_Flags {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_Flags {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SecPkgContext_Flags {}
impl ::core::fmt::Debug for SecPkgContext_Flags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_Flags").field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for SecPkgContext_IssuerListInfoEx {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for SecPkgContext_IssuerListInfoEx {
    fn eq(&self, other: &Self) -> bool {
        self.aIssuers == other.aIssuers && self.cIssuers == other.cIssuers
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for SecPkgContext_IssuerListInfoEx {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for SecPkgContext_IssuerListInfoEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_IssuerListInfoEx").field("aIssuers", &self.aIssuers).field("cIssuers", &self.cIssuers).finish()
    }
}
impl ::core::default::Default for SecPkgContext_KeyInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_KeyInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.sSignatureAlgorithmName == other.sSignatureAlgorithmName && self.sEncryptAlgorithmName == other.sEncryptAlgorithmName && self.KeySize == other.KeySize && self.SignatureAlgorithm == other.SignatureAlgorithm && self.EncryptAlgorithm == other.EncryptAlgorithm
    }
}
impl ::core::cmp::Eq for SecPkgContext_KeyInfoA {}
impl ::core::fmt::Debug for SecPkgContext_KeyInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_KeyInfoA").field("sSignatureAlgorithmName", &self.sSignatureAlgorithmName).field("sEncryptAlgorithmName", &self.sEncryptAlgorithmName).field("KeySize", &self.KeySize).field("SignatureAlgorithm", &self.SignatureAlgorithm).field("EncryptAlgorithm", &self.EncryptAlgorithm).finish()
    }
}
impl ::core::default::Default for SecPkgContext_KeyInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_KeyInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.sSignatureAlgorithmName == other.sSignatureAlgorithmName && self.sEncryptAlgorithmName == other.sEncryptAlgorithmName && self.KeySize == other.KeySize && self.SignatureAlgorithm == other.SignatureAlgorithm && self.EncryptAlgorithm == other.EncryptAlgorithm
    }
}
impl ::core::cmp::Eq for SecPkgContext_KeyInfoW {}
impl ::core::fmt::Debug for SecPkgContext_KeyInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_KeyInfoW").field("sSignatureAlgorithmName", &self.sSignatureAlgorithmName).field("sEncryptAlgorithmName", &self.sEncryptAlgorithmName).field("KeySize", &self.KeySize).field("SignatureAlgorithm", &self.SignatureAlgorithm).field("EncryptAlgorithm", &self.EncryptAlgorithm).finish()
    }
}
impl ::core::default::Default for SecPkgContext_KeyingMaterial {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_KeyingMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.cbKeyingMaterial == other.cbKeyingMaterial && self.pbKeyingMaterial == other.pbKeyingMaterial
    }
}
impl ::core::cmp::Eq for SecPkgContext_KeyingMaterial {}
impl ::core::fmt::Debug for SecPkgContext_KeyingMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_KeyingMaterial").field("cbKeyingMaterial", &self.cbKeyingMaterial).field("pbKeyingMaterial", &self.pbKeyingMaterial).finish()
    }
}
impl ::core::default::Default for SecPkgContext_KeyingMaterialInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_KeyingMaterialInfo {
    fn eq(&self, other: &Self) -> bool {
        self.cbLabel == other.cbLabel && self.pszLabel == other.pszLabel && self.cbContextValue == other.cbContextValue && self.pbContextValue == other.pbContextValue && self.cbKeyingMaterial == other.cbKeyingMaterial
    }
}
impl ::core::cmp::Eq for SecPkgContext_KeyingMaterialInfo {}
impl ::core::fmt::Debug for SecPkgContext_KeyingMaterialInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_KeyingMaterialInfo").field("cbLabel", &self.cbLabel).field("pszLabel", &self.pszLabel).field("cbContextValue", &self.cbContextValue).field("pbContextValue", &self.pbContextValue).field("cbKeyingMaterial", &self.cbKeyingMaterial).finish()
    }
}
impl ::core::default::Default for SecPkgContext_KeyingMaterial_Inproc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_KeyingMaterial_Inproc {
    fn eq(&self, other: &Self) -> bool {
        self.cbLabel == other.cbLabel && self.pszLabel == other.pszLabel && self.cbContextValue == other.cbContextValue && self.pbContextValue == other.pbContextValue && self.cbKeyingMaterial == other.cbKeyingMaterial && self.pbKeyingMaterial == other.pbKeyingMaterial
    }
}
impl ::core::cmp::Eq for SecPkgContext_KeyingMaterial_Inproc {}
impl ::core::fmt::Debug for SecPkgContext_KeyingMaterial_Inproc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_KeyingMaterial_Inproc").field("cbLabel", &self.cbLabel).field("pszLabel", &self.pszLabel).field("cbContextValue", &self.cbContextValue).field("pbContextValue", &self.pbContextValue).field("cbKeyingMaterial", &self.cbKeyingMaterial).field("pbKeyingMaterial", &self.pbKeyingMaterial).finish()
    }
}
impl ::core::default::Default for SecPkgContext_LastClientTokenStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_LastClientTokenStatus {
    fn eq(&self, other: &Self) -> bool {
        self.LastClientTokenStatus == other.LastClientTokenStatus
    }
}
impl ::core::cmp::Eq for SecPkgContext_LastClientTokenStatus {}
impl ::core::fmt::Debug for SecPkgContext_LastClientTokenStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_LastClientTokenStatus").field("LastClientTokenStatus", &self.LastClientTokenStatus).finish()
    }
}
impl ::core::default::Default for SecPkgContext_Lifespan {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_Lifespan {
    fn eq(&self, other: &Self) -> bool {
        self.tsStart == other.tsStart && self.tsExpiry == other.tsExpiry
    }
}
impl ::core::cmp::Eq for SecPkgContext_Lifespan {}
impl ::core::fmt::Debug for SecPkgContext_Lifespan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_Lifespan").field("tsStart", &self.tsStart).field("tsExpiry", &self.tsExpiry).finish()
    }
}
impl ::core::default::Default for SecPkgContext_LocalCredentialInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_LocalCredentialInfo {
    fn eq(&self, other: &Self) -> bool {
        self.cbCertificateChain == other.cbCertificateChain && self.pbCertificateChain == other.pbCertificateChain && self.cCertificates == other.cCertificates && self.fFlags == other.fFlags && self.dwBits == other.dwBits
    }
}
impl ::core::cmp::Eq for SecPkgContext_LocalCredentialInfo {}
impl ::core::fmt::Debug for SecPkgContext_LocalCredentialInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_LocalCredentialInfo").field("cbCertificateChain", &self.cbCertificateChain).field("pbCertificateChain", &self.pbCertificateChain).field("cCertificates", &self.cCertificates).field("fFlags", &self.fFlags).field("dwBits", &self.dwBits).finish()
    }
}
impl ::core::default::Default for SecPkgContext_LogoffTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_LogoffTime {
    fn eq(&self, other: &Self) -> bool {
        self.tsLogoffTime == other.tsLogoffTime
    }
}
impl ::core::cmp::Eq for SecPkgContext_LogoffTime {}
impl ::core::fmt::Debug for SecPkgContext_LogoffTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_LogoffTime").field("tsLogoffTime", &self.tsLogoffTime).finish()
    }
}
impl ::core::default::Default for SecPkgContext_MappedCredAttr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_MappedCredAttr {
    fn eq(&self, other: &Self) -> bool {
        self.dwAttribute == other.dwAttribute && self.pvBuffer == other.pvBuffer
    }
}
impl ::core::cmp::Eq for SecPkgContext_MappedCredAttr {}
impl ::core::fmt::Debug for SecPkgContext_MappedCredAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_MappedCredAttr").field("dwAttribute", &self.dwAttribute).field("pvBuffer", &self.pvBuffer).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NamesA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NamesA {
    fn eq(&self, other: &Self) -> bool {
        self.sUserName == other.sUserName
    }
}
impl ::core::cmp::Eq for SecPkgContext_NamesA {}
impl ::core::fmt::Debug for SecPkgContext_NamesA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NamesA").field("sUserName", &self.sUserName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NamesW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NamesW {
    fn eq(&self, other: &Self) -> bool {
        self.sUserName == other.sUserName
    }
}
impl ::core::cmp::Eq for SecPkgContext_NamesW {}
impl ::core::fmt::Debug for SecPkgContext_NamesW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NamesW").field("sUserName", &self.sUserName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NativeNamesA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NativeNamesA {
    fn eq(&self, other: &Self) -> bool {
        self.sClientName == other.sClientName && self.sServerName == other.sServerName
    }
}
impl ::core::cmp::Eq for SecPkgContext_NativeNamesA {}
impl ::core::fmt::Debug for SecPkgContext_NativeNamesA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NativeNamesA").field("sClientName", &self.sClientName).field("sServerName", &self.sServerName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NativeNamesW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NativeNamesW {
    fn eq(&self, other: &Self) -> bool {
        self.sClientName == other.sClientName && self.sServerName == other.sServerName
    }
}
impl ::core::cmp::Eq for SecPkgContext_NativeNamesW {}
impl ::core::fmt::Debug for SecPkgContext_NativeNamesW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NativeNamesW").field("sClientName", &self.sClientName).field("sServerName", &self.sServerName).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NegoKeys {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NegoKeys {
    fn eq(&self, other: &Self) -> bool {
        self.KeyType == other.KeyType && self.KeyLength == other.KeyLength && self.KeyValue == other.KeyValue && self.VerifyKeyType == other.VerifyKeyType && self.VerifyKeyLength == other.VerifyKeyLength && self.VerifyKeyValue == other.VerifyKeyValue
    }
}
impl ::core::cmp::Eq for SecPkgContext_NegoKeys {}
impl ::core::fmt::Debug for SecPkgContext_NegoKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NegoKeys").field("KeyType", &self.KeyType).field("KeyLength", &self.KeyLength).field("KeyValue", &self.KeyValue).field("VerifyKeyType", &self.VerifyKeyType).field("VerifyKeyLength", &self.VerifyKeyLength).field("VerifyKeyValue", &self.VerifyKeyValue).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NegoPackageInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NegoPackageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.PackageMask == other.PackageMask
    }
}
impl ::core::cmp::Eq for SecPkgContext_NegoPackageInfo {}
impl ::core::fmt::Debug for SecPkgContext_NegoPackageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NegoPackageInfo").field("PackageMask", &self.PackageMask).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NegoStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NegoStatus {
    fn eq(&self, other: &Self) -> bool {
        self.LastStatus == other.LastStatus
    }
}
impl ::core::cmp::Eq for SecPkgContext_NegoStatus {}
impl ::core::fmt::Debug for SecPkgContext_NegoStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NegoStatus").field("LastStatus", &self.LastStatus).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NegotiatedTlsExtensions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NegotiatedTlsExtensions {
    fn eq(&self, other: &Self) -> bool {
        self.ExtensionsCount == other.ExtensionsCount && self.Extensions == other.Extensions
    }
}
impl ::core::cmp::Eq for SecPkgContext_NegotiatedTlsExtensions {}
impl ::core::fmt::Debug for SecPkgContext_NegotiatedTlsExtensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NegotiatedTlsExtensions").field("ExtensionsCount", &self.ExtensionsCount).field("Extensions", &self.Extensions).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NegotiationInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NegotiationInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.PackageInfo == other.PackageInfo && self.NegotiationState == other.NegotiationState
    }
}
impl ::core::cmp::Eq for SecPkgContext_NegotiationInfoA {}
impl ::core::fmt::Debug for SecPkgContext_NegotiationInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NegotiationInfoA").field("PackageInfo", &self.PackageInfo).field("NegotiationState", &self.NegotiationState).finish()
    }
}
impl ::core::default::Default for SecPkgContext_NegotiationInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_NegotiationInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.PackageInfo == other.PackageInfo && self.NegotiationState == other.NegotiationState
    }
}
impl ::core::cmp::Eq for SecPkgContext_NegotiationInfoW {}
impl ::core::fmt::Debug for SecPkgContext_NegotiationInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_NegotiationInfoW").field("PackageInfo", &self.PackageInfo).field("NegotiationState", &self.NegotiationState).finish()
    }
}
impl ::core::default::Default for SecPkgContext_PackageInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_PackageInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.PackageInfo == other.PackageInfo
    }
}
impl ::core::cmp::Eq for SecPkgContext_PackageInfoA {}
impl ::core::fmt::Debug for SecPkgContext_PackageInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_PackageInfoA").field("PackageInfo", &self.PackageInfo).finish()
    }
}
impl ::core::default::Default for SecPkgContext_PackageInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_PackageInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.PackageInfo == other.PackageInfo
    }
}
impl ::core::cmp::Eq for SecPkgContext_PackageInfoW {}
impl ::core::fmt::Debug for SecPkgContext_PackageInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_PackageInfoW").field("PackageInfo", &self.PackageInfo).finish()
    }
}
impl ::core::default::Default for SecPkgContext_PasswordExpiry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_PasswordExpiry {
    fn eq(&self, other: &Self) -> bool {
        self.tsPasswordExpires == other.tsPasswordExpires
    }
}
impl ::core::cmp::Eq for SecPkgContext_PasswordExpiry {}
impl ::core::fmt::Debug for SecPkgContext_PasswordExpiry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_PasswordExpiry").field("tsPasswordExpires", &self.tsPasswordExpires).finish()
    }
}
impl ::core::default::Default for SecPkgContext_ProtoInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_ProtoInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.sProtocolName == other.sProtocolName && self.majorVersion == other.majorVersion && self.minorVersion == other.minorVersion
    }
}
impl ::core::cmp::Eq for SecPkgContext_ProtoInfoA {}
impl ::core::fmt::Debug for SecPkgContext_ProtoInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ProtoInfoA").field("sProtocolName", &self.sProtocolName).field("majorVersion", &self.majorVersion).field("minorVersion", &self.minorVersion).finish()
    }
}
impl ::core::default::Default for SecPkgContext_ProtoInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_ProtoInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.sProtocolName == other.sProtocolName && self.majorVersion == other.majorVersion && self.minorVersion == other.minorVersion
    }
}
impl ::core::cmp::Eq for SecPkgContext_ProtoInfoW {}
impl ::core::fmt::Debug for SecPkgContext_ProtoInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_ProtoInfoW").field("sProtocolName", &self.sProtocolName).field("majorVersion", &self.majorVersion).field("minorVersion", &self.minorVersion).finish()
    }
}
impl ::core::default::Default for SecPkgContext_RemoteCredentialInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_RemoteCredentialInfo {
    fn eq(&self, other: &Self) -> bool {
        self.cbCertificateChain == other.cbCertificateChain && self.pbCertificateChain == other.pbCertificateChain && self.cCertificates == other.cCertificates && self.fFlags == other.fFlags && self.dwBits == other.dwBits
    }
}
impl ::core::cmp::Eq for SecPkgContext_RemoteCredentialInfo {}
impl ::core::fmt::Debug for SecPkgContext_RemoteCredentialInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_RemoteCredentialInfo").field("cbCertificateChain", &self.cbCertificateChain).field("pbCertificateChain", &self.pbCertificateChain).field("cCertificates", &self.cCertificates).field("fFlags", &self.fFlags).field("dwBits", &self.dwBits).finish()
    }
}
impl ::core::default::Default for SecPkgContext_SaslContext {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_SaslContext {
    fn eq(&self, other: &Self) -> bool {
        self.SaslContext == other.SaslContext
    }
}
impl ::core::cmp::Eq for SecPkgContext_SaslContext {}
impl ::core::fmt::Debug for SecPkgContext_SaslContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_SaslContext").field("SaslContext", &self.SaslContext).finish()
    }
}
impl ::core::default::Default for SecPkgContext_SessionAppData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_SessionAppData {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.cbAppData == other.cbAppData && self.pbAppData == other.pbAppData
    }
}
impl ::core::cmp::Eq for SecPkgContext_SessionAppData {}
impl ::core::fmt::Debug for SecPkgContext_SessionAppData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_SessionAppData").field("dwFlags", &self.dwFlags).field("cbAppData", &self.cbAppData).field("pbAppData", &self.pbAppData).finish()
    }
}
impl ::core::default::Default for SecPkgContext_SessionInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_SessionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.cbSessionId == other.cbSessionId && self.rgbSessionId == other.rgbSessionId
    }
}
impl ::core::cmp::Eq for SecPkgContext_SessionInfo {}
impl ::core::fmt::Debug for SecPkgContext_SessionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_SessionInfo").field("dwFlags", &self.dwFlags).field("cbSessionId", &self.cbSessionId).field("rgbSessionId", &self.rgbSessionId).finish()
    }
}
impl ::core::default::Default for SecPkgContext_SessionKey {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_SessionKey {
    fn eq(&self, other: &Self) -> bool {
        self.SessionKeyLength == other.SessionKeyLength && self.SessionKey == other.SessionKey
    }
}
impl ::core::cmp::Eq for SecPkgContext_SessionKey {}
impl ::core::fmt::Debug for SecPkgContext_SessionKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_SessionKey").field("SessionKeyLength", &self.SessionKeyLength).field("SessionKey", &self.SessionKey).finish()
    }
}
impl ::core::default::Default for SecPkgContext_Sizes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_Sizes {
    fn eq(&self, other: &Self) -> bool {
        self.cbMaxToken == other.cbMaxToken && self.cbMaxSignature == other.cbMaxSignature && self.cbBlockSize == other.cbBlockSize && self.cbSecurityTrailer == other.cbSecurityTrailer
    }
}
impl ::core::cmp::Eq for SecPkgContext_Sizes {}
impl ::core::fmt::Debug for SecPkgContext_Sizes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_Sizes").field("cbMaxToken", &self.cbMaxToken).field("cbMaxSignature", &self.cbMaxSignature).field("cbBlockSize", &self.cbBlockSize).field("cbSecurityTrailer", &self.cbSecurityTrailer).finish()
    }
}
impl ::core::default::Default for SecPkgContext_SrtpParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_SrtpParameters {
    fn eq(&self, other: &Self) -> bool {
        self.ProtectionProfile == other.ProtectionProfile && self.MasterKeyIdentifierSize == other.MasterKeyIdentifierSize && self.MasterKeyIdentifier == other.MasterKeyIdentifier
    }
}
impl ::core::cmp::Eq for SecPkgContext_SrtpParameters {}
impl ::core::fmt::Debug for SecPkgContext_SrtpParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_SrtpParameters").field("ProtectionProfile", &self.ProtectionProfile).field("MasterKeyIdentifierSize", &self.MasterKeyIdentifierSize).field("MasterKeyIdentifier", &self.MasterKeyIdentifier).finish()
    }
}
impl ::core::default::Default for SecPkgContext_StreamSizes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_StreamSizes {
    fn eq(&self, other: &Self) -> bool {
        self.cbHeader == other.cbHeader && self.cbTrailer == other.cbTrailer && self.cbMaximumMessage == other.cbMaximumMessage && self.cBuffers == other.cBuffers && self.cbBlockSize == other.cbBlockSize
    }
}
impl ::core::cmp::Eq for SecPkgContext_StreamSizes {}
impl ::core::fmt::Debug for SecPkgContext_StreamSizes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_StreamSizes").field("cbHeader", &self.cbHeader).field("cbTrailer", &self.cbTrailer).field("cbMaximumMessage", &self.cbMaximumMessage).field("cBuffers", &self.cBuffers).field("cbBlockSize", &self.cbBlockSize).finish()
    }
}
impl ::core::default::Default for SecPkgContext_SubjectAttributes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_SubjectAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.AttributeInfo == other.AttributeInfo
    }
}
impl ::core::cmp::Eq for SecPkgContext_SubjectAttributes {}
impl ::core::fmt::Debug for SecPkgContext_SubjectAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_SubjectAttributes").field("AttributeInfo", &self.AttributeInfo).finish()
    }
}
impl ::core::default::Default for SecPkgContext_SupportedSignatures {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_SupportedSignatures {
    fn eq(&self, other: &Self) -> bool {
        self.cSignatureAndHashAlgorithms == other.cSignatureAndHashAlgorithms && self.pSignatureAndHashAlgorithms == other.pSignatureAndHashAlgorithms
    }
}
impl ::core::cmp::Eq for SecPkgContext_SupportedSignatures {}
impl ::core::fmt::Debug for SecPkgContext_SupportedSignatures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_SupportedSignatures").field("cSignatureAndHashAlgorithms", &self.cSignatureAndHashAlgorithms).field("pSignatureAndHashAlgorithms", &self.pSignatureAndHashAlgorithms).finish()
    }
}
impl ::core::default::Default for SecPkgContext_Target {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_Target {
    fn eq(&self, other: &Self) -> bool {
        self.TargetLength == other.TargetLength && self.Target == other.Target
    }
}
impl ::core::cmp::Eq for SecPkgContext_Target {}
impl ::core::fmt::Debug for SecPkgContext_Target {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_Target").field("TargetLength", &self.TargetLength).field("Target", &self.Target).finish()
    }
}
impl ::core::default::Default for SecPkgContext_TargetInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_TargetInformation {
    fn eq(&self, other: &Self) -> bool {
        self.MarshalledTargetInfoLength == other.MarshalledTargetInfoLength && self.MarshalledTargetInfo == other.MarshalledTargetInfo
    }
}
impl ::core::cmp::Eq for SecPkgContext_TargetInformation {}
impl ::core::fmt::Debug for SecPkgContext_TargetInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_TargetInformation").field("MarshalledTargetInfoLength", &self.MarshalledTargetInfoLength).field("MarshalledTargetInfo", &self.MarshalledTargetInfo).finish()
    }
}
impl ::core::default::Default for SecPkgContext_TokenBinding {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_TokenBinding {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.KeyParametersSize == other.KeyParametersSize && self.KeyParameters == other.KeyParameters
    }
}
impl ::core::cmp::Eq for SecPkgContext_TokenBinding {}
impl ::core::fmt::Debug for SecPkgContext_TokenBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_TokenBinding").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("KeyParametersSize", &self.KeyParametersSize).field("KeyParameters", &self.KeyParameters).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SecPkgContext_UiInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SecPkgContext_UiInfo {
    fn eq(&self, other: &Self) -> bool {
        self.hParentWindow == other.hParentWindow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SecPkgContext_UiInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SecPkgContext_UiInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_UiInfo").field("hParentWindow", &self.hParentWindow).finish()
    }
}
impl ::core::default::Default for SecPkgContext_UserFlags {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgContext_UserFlags {
    fn eq(&self, other: &Self) -> bool {
        self.UserFlags == other.UserFlags
    }
}
impl ::core::cmp::Eq for SecPkgContext_UserFlags {}
impl ::core::fmt::Debug for SecPkgContext_UserFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgContext_UserFlags").field("UserFlags", &self.UserFlags).finish()
    }
}
impl ::core::default::Default for SecPkgCred_CipherStrengths {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCred_CipherStrengths {
    fn eq(&self, other: &Self) -> bool {
        self.dwMinimumCipherStrength == other.dwMinimumCipherStrength && self.dwMaximumCipherStrength == other.dwMaximumCipherStrength
    }
}
impl ::core::cmp::Eq for SecPkgCred_CipherStrengths {}
impl ::core::fmt::Debug for SecPkgCred_CipherStrengths {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCred_CipherStrengths").field("dwMinimumCipherStrength", &self.dwMinimumCipherStrength).field("dwMaximumCipherStrength", &self.dwMaximumCipherStrength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SecPkgCred_ClientCertPolicy {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SecPkgCred_ClientCertPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.guidPolicyId == other.guidPolicyId && self.dwCertFlags == other.dwCertFlags && self.dwUrlRetrievalTimeout == other.dwUrlRetrievalTimeout && self.fCheckRevocationFreshnessTime == other.fCheckRevocationFreshnessTime && self.dwRevocationFreshnessTime == other.dwRevocationFreshnessTime && self.fOmitUsageCheck == other.fOmitUsageCheck && self.pwszSslCtlStoreName == other.pwszSslCtlStoreName && self.pwszSslCtlIdentifier == other.pwszSslCtlIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SecPkgCred_ClientCertPolicy {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SecPkgCred_ClientCertPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCred_ClientCertPolicy")
            .field("dwFlags", &self.dwFlags)
            .field("guidPolicyId", &self.guidPolicyId)
            .field("dwCertFlags", &self.dwCertFlags)
            .field("dwUrlRetrievalTimeout", &self.dwUrlRetrievalTimeout)
            .field("fCheckRevocationFreshnessTime", &self.fCheckRevocationFreshnessTime)
            .field("dwRevocationFreshnessTime", &self.dwRevocationFreshnessTime)
            .field("fOmitUsageCheck", &self.fOmitUsageCheck)
            .field("pwszSslCtlStoreName", &self.pwszSslCtlStoreName)
            .field("pwszSslCtlIdentifier", &self.pwszSslCtlIdentifier)
            .finish()
    }
}
impl ::core::default::Default for SecPkgCred_SessionTicketKey {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCred_SessionTicketKey {
    fn eq(&self, other: &Self) -> bool {
        self.TicketInfoVersion == other.TicketInfoVersion && self.KeyId == other.KeyId && self.KeyingMaterial == other.KeyingMaterial && self.KeyingMaterialSize == other.KeyingMaterialSize
    }
}
impl ::core::cmp::Eq for SecPkgCred_SessionTicketKey {}
impl ::core::fmt::Debug for SecPkgCred_SessionTicketKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCred_SessionTicketKey").field("TicketInfoVersion", &self.TicketInfoVersion).field("KeyId", &self.KeyId).field("KeyingMaterial", &self.KeyingMaterial).field("KeyingMaterialSize", &self.KeyingMaterialSize).finish()
    }
}
impl ::core::default::Default for SecPkgCred_SessionTicketKeys {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCred_SessionTicketKeys {
    fn eq(&self, other: &Self) -> bool {
        self.cSessionTicketKeys == other.cSessionTicketKeys && self.pSessionTicketKeys == other.pSessionTicketKeys
    }
}
impl ::core::cmp::Eq for SecPkgCred_SessionTicketKeys {}
impl ::core::fmt::Debug for SecPkgCred_SessionTicketKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCred_SessionTicketKeys").field("cSessionTicketKeys", &self.cSessionTicketKeys).field("pSessionTicketKeys", &self.pSessionTicketKeys).finish()
    }
}
impl ::core::default::Default for SecPkgCred_SupportedAlgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCred_SupportedAlgs {
    fn eq(&self, other: &Self) -> bool {
        self.cSupportedAlgs == other.cSupportedAlgs && self.palgSupportedAlgs == other.palgSupportedAlgs
    }
}
impl ::core::cmp::Eq for SecPkgCred_SupportedAlgs {}
impl ::core::fmt::Debug for SecPkgCred_SupportedAlgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCred_SupportedAlgs").field("cSupportedAlgs", &self.cSupportedAlgs).field("palgSupportedAlgs", &self.palgSupportedAlgs).finish()
    }
}
impl ::core::default::Default for SecPkgCred_SupportedProtocols {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCred_SupportedProtocols {
    fn eq(&self, other: &Self) -> bool {
        self.grbitProtocol == other.grbitProtocol
    }
}
impl ::core::cmp::Eq for SecPkgCred_SupportedProtocols {}
impl ::core::fmt::Debug for SecPkgCred_SupportedProtocols {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCred_SupportedProtocols").field("grbitProtocol", &self.grbitProtocol).finish()
    }
}
impl ::core::default::Default for SecPkgCredentials_Cert {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCredentials_Cert {
    fn eq(&self, other: &Self) -> bool {
        self.EncodedCertSize == other.EncodedCertSize && self.EncodedCert == other.EncodedCert
    }
}
impl ::core::cmp::Eq for SecPkgCredentials_Cert {}
impl ::core::fmt::Debug for SecPkgCredentials_Cert {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCredentials_Cert").field("EncodedCertSize", &self.EncodedCertSize).field("EncodedCert", &self.EncodedCert).finish()
    }
}
impl ::core::default::Default for SecPkgCredentials_KdcProxySettingsW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCredentials_KdcProxySettingsW {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.ProxyServerOffset == other.ProxyServerOffset && self.ProxyServerLength == other.ProxyServerLength && self.ClientTlsCredOffset == other.ClientTlsCredOffset && self.ClientTlsCredLength == other.ClientTlsCredLength
    }
}
impl ::core::cmp::Eq for SecPkgCredentials_KdcProxySettingsW {}
impl ::core::fmt::Debug for SecPkgCredentials_KdcProxySettingsW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCredentials_KdcProxySettingsW").field("Version", &self.Version).field("Flags", &self.Flags).field("ProxyServerOffset", &self.ProxyServerOffset).field("ProxyServerLength", &self.ProxyServerLength).field("ClientTlsCredOffset", &self.ClientTlsCredOffset).field("ClientTlsCredLength", &self.ClientTlsCredLength).finish()
    }
}
impl ::core::default::Default for SecPkgCredentials_NamesA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCredentials_NamesA {
    fn eq(&self, other: &Self) -> bool {
        self.sUserName == other.sUserName
    }
}
impl ::core::cmp::Eq for SecPkgCredentials_NamesA {}
impl ::core::fmt::Debug for SecPkgCredentials_NamesA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCredentials_NamesA").field("sUserName", &self.sUserName).finish()
    }
}
impl ::core::default::Default for SecPkgCredentials_NamesW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCredentials_NamesW {
    fn eq(&self, other: &Self) -> bool {
        self.sUserName == other.sUserName
    }
}
impl ::core::cmp::Eq for SecPkgCredentials_NamesW {}
impl ::core::fmt::Debug for SecPkgCredentials_NamesW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCredentials_NamesW").field("sUserName", &self.sUserName).finish()
    }
}
impl ::core::default::Default for SecPkgCredentials_SSIProviderA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCredentials_SSIProviderA {
    fn eq(&self, other: &Self) -> bool {
        self.sProviderName == other.sProviderName && self.ProviderInfoLength == other.ProviderInfoLength && self.ProviderInfo == other.ProviderInfo
    }
}
impl ::core::cmp::Eq for SecPkgCredentials_SSIProviderA {}
impl ::core::fmt::Debug for SecPkgCredentials_SSIProviderA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCredentials_SSIProviderA").field("sProviderName", &self.sProviderName).field("ProviderInfoLength", &self.ProviderInfoLength).field("ProviderInfo", &self.ProviderInfo).finish()
    }
}
impl ::core::default::Default for SecPkgCredentials_SSIProviderW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgCredentials_SSIProviderW {
    fn eq(&self, other: &Self) -> bool {
        self.sProviderName == other.sProviderName && self.ProviderInfoLength == other.ProviderInfoLength && self.ProviderInfo == other.ProviderInfo
    }
}
impl ::core::cmp::Eq for SecPkgCredentials_SSIProviderW {}
impl ::core::fmt::Debug for SecPkgCredentials_SSIProviderW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgCredentials_SSIProviderW").field("sProviderName", &self.sProviderName).field("ProviderInfoLength", &self.ProviderInfoLength).field("ProviderInfo", &self.ProviderInfo).finish()
    }
}
impl ::core::default::Default for SecPkgInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.fCapabilities == other.fCapabilities && self.wVersion == other.wVersion && self.wRPCID == other.wRPCID && self.cbMaxToken == other.cbMaxToken && self.Name == other.Name && self.Comment == other.Comment
    }
}
impl ::core::cmp::Eq for SecPkgInfoA {}
impl ::core::fmt::Debug for SecPkgInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgInfoA").field("fCapabilities", &self.fCapabilities).field("wVersion", &self.wVersion).field("wRPCID", &self.wRPCID).field("cbMaxToken", &self.cbMaxToken).field("Name", &self.Name).field("Comment", &self.Comment).finish()
    }
}
impl ::core::default::Default for SecPkgInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SecPkgInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.fCapabilities == other.fCapabilities && self.wVersion == other.wVersion && self.wRPCID == other.wRPCID && self.cbMaxToken == other.cbMaxToken && self.Name == other.Name && self.Comment == other.Comment
    }
}
impl ::core::cmp::Eq for SecPkgInfoW {}
impl ::core::fmt::Debug for SecPkgInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SecPkgInfoW").field("fCapabilities", &self.fCapabilities).field("wVersion", &self.wVersion).field("wRPCID", &self.wRPCID).field("cbMaxToken", &self.cbMaxToken).field("Name", &self.Name).field("Comment", &self.Comment).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::default::Default for SecurityFunctionTableA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Credentials"))]
impl ::core::default::Default for SecurityFunctionTableW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TLS_EXTENSION_SUBSCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TLS_EXTENSION_SUBSCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.ExtensionType == other.ExtensionType && self.HandshakeType == other.HandshakeType
    }
}
impl ::core::cmp::Eq for TLS_EXTENSION_SUBSCRIPTION {}
impl ::core::fmt::Debug for TLS_EXTENSION_SUBSCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TLS_EXTENSION_SUBSCRIPTION").field("ExtensionType", &self.ExtensionType).field("HandshakeType", &self.HandshakeType).finish()
    }
}
impl ::core::default::Default for TOKENBINDING_EXTENSION_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKENBINDING_EXTENSION_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKENBINDING_EXTENSION_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOKENBINDING_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOKENBINDING_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.keyType == other.keyType
    }
}
impl ::core::cmp::Eq for TOKENBINDING_IDENTIFIER {}
impl ::core::fmt::Debug for TOKENBINDING_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKENBINDING_IDENTIFIER").field("keyType", &self.keyType).finish()
    }
}
impl ::core::default::Default for TOKENBINDING_KEY_PARAMETERS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKENBINDING_KEY_PARAMETERS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKENBINDING_KEY_PARAMETERS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOKENBINDING_KEY_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOKENBINDING_KEY_TYPES {
    fn eq(&self, other: &Self) -> bool {
        self.keyCount == other.keyCount && self.keyType == other.keyType
    }
}
impl ::core::cmp::Eq for TOKENBINDING_KEY_TYPES {}
impl ::core::fmt::Debug for TOKENBINDING_KEY_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKENBINDING_KEY_TYPES").field("keyCount", &self.keyCount).field("keyType", &self.keyType).finish()
    }
}
impl ::core::default::Default for TOKENBINDING_RESULT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOKENBINDING_RESULT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.bindingType == other.bindingType && self.identifierSize == other.identifierSize && self.identifierData == other.identifierData && self.extensionFormat == other.extensionFormat && self.extensionSize == other.extensionSize && self.extensionData == other.extensionData
    }
}
impl ::core::cmp::Eq for TOKENBINDING_RESULT_DATA {}
impl ::core::fmt::Debug for TOKENBINDING_RESULT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKENBINDING_RESULT_DATA").field("bindingType", &self.bindingType).field("identifierSize", &self.identifierSize).field("identifierData", &self.identifierData).field("extensionFormat", &self.extensionFormat).field("extensionSize", &self.extensionSize).field("extensionData", &self.extensionData).finish()
    }
}
impl ::core::default::Default for TOKENBINDING_RESULT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOKENBINDING_RESULT_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.resultCount == other.resultCount && self.resultData == other.resultData
    }
}
impl ::core::cmp::Eq for TOKENBINDING_RESULT_LIST {}
impl ::core::fmt::Debug for TOKENBINDING_RESULT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKENBINDING_RESULT_LIST").field("resultCount", &self.resultCount).field("resultData", &self.resultData).finish()
    }
}
impl ::core::default::Default for TOKENBINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKENBINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKENBINDING_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRUSTED_CONTROLLERS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRUSTED_CONTROLLERS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Entries == other.Entries && self.Names == other.Names
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRUSTED_CONTROLLERS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRUSTED_CONTROLLERS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_CONTROLLERS_INFO").field("Entries", &self.Entries).field("Names", &self.Names).finish()
    }
}
impl ::core::default::Default for TRUSTED_DOMAIN_AUTH_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRUSTED_DOMAIN_AUTH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IncomingAuthInfos == other.IncomingAuthInfos && self.IncomingAuthenticationInformation == other.IncomingAuthenticationInformation && self.IncomingPreviousAuthenticationInformation == other.IncomingPreviousAuthenticationInformation && self.OutgoingAuthInfos == other.OutgoingAuthInfos && self.OutgoingAuthenticationInformation == other.OutgoingAuthenticationInformation && self.OutgoingPreviousAuthenticationInformation == other.OutgoingPreviousAuthenticationInformation
    }
}
impl ::core::cmp::Eq for TRUSTED_DOMAIN_AUTH_INFORMATION {}
impl ::core::fmt::Debug for TRUSTED_DOMAIN_AUTH_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_DOMAIN_AUTH_INFORMATION")
            .field("IncomingAuthInfos", &self.IncomingAuthInfos)
            .field("IncomingAuthenticationInformation", &self.IncomingAuthenticationInformation)
            .field("IncomingPreviousAuthenticationInformation", &self.IncomingPreviousAuthenticationInformation)
            .field("OutgoingAuthInfos", &self.OutgoingAuthInfos)
            .field("OutgoingAuthenticationInformation", &self.OutgoingAuthenticationInformation)
            .field("OutgoingPreviousAuthenticationInformation", &self.OutgoingPreviousAuthenticationInformation)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRUSTED_DOMAIN_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRUSTED_DOMAIN_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Information == other.Information && self.PosixOffset == other.PosixOffset && self.AuthInformation == other.AuthInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRUSTED_DOMAIN_FULL_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRUSTED_DOMAIN_FULL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_DOMAIN_FULL_INFORMATION").field("Information", &self.Information).field("PosixOffset", &self.PosixOffset).field("AuthInformation", &self.AuthInformation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRUSTED_DOMAIN_FULL_INFORMATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRUSTED_DOMAIN_FULL_INFORMATION2 {
    fn eq(&self, other: &Self) -> bool {
        self.Information == other.Information && self.PosixOffset == other.PosixOffset && self.AuthInformation == other.AuthInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRUSTED_DOMAIN_FULL_INFORMATION2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRUSTED_DOMAIN_FULL_INFORMATION2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_DOMAIN_FULL_INFORMATION2").field("Information", &self.Information).field("PosixOffset", &self.PosixOffset).field("AuthInformation", &self.AuthInformation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRUSTED_DOMAIN_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRUSTED_DOMAIN_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.FlatName == other.FlatName && self.Sid == other.Sid && self.TrustDirection == other.TrustDirection && self.TrustType == other.TrustType && self.TrustAttributes == other.TrustAttributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRUSTED_DOMAIN_INFORMATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRUSTED_DOMAIN_INFORMATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_DOMAIN_INFORMATION_EX").field("Name", &self.Name).field("FlatName", &self.FlatName).field("Sid", &self.Sid).field("TrustDirection", &self.TrustDirection).field("TrustType", &self.TrustType).field("TrustAttributes", &self.TrustAttributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRUSTED_DOMAIN_INFORMATION_EX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRUSTED_DOMAIN_INFORMATION_EX2 {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.FlatName == other.FlatName && self.Sid == other.Sid && self.TrustDirection == other.TrustDirection && self.TrustType == other.TrustType && self.TrustAttributes == other.TrustAttributes && self.ForestTrustLength == other.ForestTrustLength && self.ForestTrustInfo == other.ForestTrustInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRUSTED_DOMAIN_INFORMATION_EX2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRUSTED_DOMAIN_INFORMATION_EX2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_DOMAIN_INFORMATION_EX2").field("Name", &self.Name).field("FlatName", &self.FlatName).field("Sid", &self.Sid).field("TrustDirection", &self.TrustDirection).field("TrustType", &self.TrustType).field("TrustAttributes", &self.TrustAttributes).field("ForestTrustLength", &self.ForestTrustLength).field("ForestTrustInfo", &self.ForestTrustInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRUSTED_DOMAIN_NAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRUSTED_DOMAIN_NAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRUSTED_DOMAIN_NAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRUSTED_DOMAIN_NAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_DOMAIN_NAME_INFO").field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    fn eq(&self, other: &Self) -> bool {
        self.SupportedEncryptionTypes == other.SupportedEncryptionTypes
    }
}
impl ::core::cmp::Eq for TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {}
impl ::core::fmt::Debug for TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES").field("SupportedEncryptionTypes", &self.SupportedEncryptionTypes).finish()
    }
}
impl ::core::default::Default for TRUSTED_DOMAIN_TRUST_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRUSTED_DOMAIN_TRUST_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUSTED_DOMAIN_TRUST_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRUSTED_DOMAIN_TRUST_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRUSTED_DOMAIN_TRUST_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUSTED_DOMAIN_TRUST_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRUSTED_DOMAIN_TRUST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRUSTED_DOMAIN_TRUST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUSTED_DOMAIN_TRUST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRUSTED_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRUSTED_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUSTED_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRUSTED_PASSWORD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRUSTED_PASSWORD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Password == other.Password && self.OldPassword == other.OldPassword
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRUSTED_PASSWORD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRUSTED_PASSWORD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_PASSWORD_INFO").field("Password", &self.Password).field("OldPassword", &self.OldPassword).finish()
    }
}
impl ::core::default::Default for TRUSTED_POSIX_OFFSET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRUSTED_POSIX_OFFSET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for TRUSTED_POSIX_OFFSET_INFO {}
impl ::core::fmt::Debug for TRUSTED_POSIX_OFFSET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTED_POSIX_OFFSET_INFO").field("Offset", &self.Offset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_ALL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::default::Default for USER_SESSION_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::cmp::PartialEq for USER_SESSION_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::cmp::Eq for USER_SESSION_KEY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PasswordManagement"))]
impl ::core::fmt::Debug for USER_SESSION_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_SESSION_KEY").field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for X509Certificate {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for X509Certificate {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.SerialNumber == other.SerialNumber && self.SignatureAlgorithm == other.SignatureAlgorithm && self.ValidFrom == other.ValidFrom && self.ValidUntil == other.ValidUntil && self.pszIssuer == other.pszIssuer && self.pszSubject == other.pszSubject && self.pPublicKey == other.pPublicKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for X509Certificate {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for X509Certificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("X509Certificate").field("Version", &self.Version).field("SerialNumber", &self.SerialNumber).field("SignatureAlgorithm", &self.SignatureAlgorithm).field("ValidFrom", &self.ValidFrom).field("ValidUntil", &self.ValidUntil).field("pszIssuer", &self.pszIssuer).field("pszSubject", &self.pszSubject).field("pPublicKey", &self.pPublicKey).finish()
    }
}
impl ::core::default::Default for eTlsHashAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eTlsHashAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eTlsHashAlgorithm").field(&self.0).finish()
    }
}
impl ::core::default::Default for eTlsSignatureAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eTlsSignatureAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eTlsSignatureAlgorithm").field(&self.0).finish()
    }
}
