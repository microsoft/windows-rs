impl ::core::default::Default for DNS_AAAA_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_ADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_ADDR_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_APPLICATION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_APPLICATION_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for DNS_APPLICATION_SETTINGS {}
impl ::core::fmt::Debug for DNS_APPLICATION_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_APPLICATION_SETTINGS").field("Version", &self.Version).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for DNS_ATMA_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_ATMA_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.AddressType == other.AddressType && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for DNS_ATMA_DATA {}
impl ::core::fmt::Debug for DNS_ATMA_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_ATMA_DATA").field("AddressType", &self.AddressType).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for DNS_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress
    }
}
impl ::core::cmp::Eq for DNS_A_DATA {}
impl ::core::fmt::Debug for DNS_A_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_A_DATA").field("IpAddress", &self.IpAddress).finish()
    }
}
impl ::core::default::Default for DNS_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_CHARSET").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_CONFIG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_CONFIG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_CONFIG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_IFINDEX_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_CONNECTION_IFINDEX_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszConnectionName == other.pwszConnectionName && self.dwIfIndex == other.dwIfIndex
    }
}
impl ::core::cmp::Eq for DNS_CONNECTION_IFINDEX_ENTRY {}
impl ::core::fmt::Debug for DNS_CONNECTION_IFINDEX_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_IFINDEX_ENTRY").field("pwszConnectionName", &self.pwszConnectionName).field("dwIfIndex", &self.dwIfIndex).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_IFINDEX_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_CONNECTION_IFINDEX_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.pConnectionIfIndexEntries == other.pConnectionIfIndexEntries && self.nEntries == other.nEntries
    }
}
impl ::core::cmp::Eq for DNS_CONNECTION_IFINDEX_LIST {}
impl ::core::fmt::Debug for DNS_CONNECTION_IFINDEX_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_IFINDEX_LIST").field("pConnectionIfIndexEntries", &self.pConnectionIfIndexEntries).field("nEntries", &self.nEntries).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_CONNECTION_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.wszName == other.wszName
    }
}
impl ::core::cmp::Eq for DNS_CONNECTION_NAME {}
impl ::core::fmt::Debug for DNS_CONNECTION_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_NAME").field("wszName", &self.wszName).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_NAME_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_CONNECTION_NAME_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cNames == other.cNames && self.pNames == other.pNames
    }
}
impl ::core::cmp::Eq for DNS_CONNECTION_NAME_LIST {}
impl ::core::fmt::Debug for DNS_CONNECTION_NAME_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_NAME_LIST").field("cNames", &self.cNames).field("pNames", &self.pNames).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_CONNECTION_POLICY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszHost == other.pwszHost && self.pwszAppId == other.pwszAppId && self.cbAppSid == other.cbAppSid && self.pbAppSid == other.pbAppSid && self.nConnections == other.nConnections && self.ppwszConnections == other.ppwszConnections && self.dwPolicyEntryFlags == other.dwPolicyEntryFlags
    }
}
impl ::core::cmp::Eq for DNS_CONNECTION_POLICY_ENTRY {}
impl ::core::fmt::Debug for DNS_CONNECTION_POLICY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_POLICY_ENTRY").field("pwszHost", &self.pwszHost).field("pwszAppId", &self.pwszAppId).field("cbAppSid", &self.cbAppSid).field("pbAppSid", &self.pbAppSid).field("nConnections", &self.nConnections).field("ppwszConnections", &self.ppwszConnections).field("dwPolicyEntryFlags", &self.dwPolicyEntryFlags).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.pPolicyEntries == other.pPolicyEntries && self.nEntries == other.nEntries
    }
}
impl ::core::cmp::Eq for DNS_CONNECTION_POLICY_ENTRY_LIST {}
impl ::core::fmt::Debug for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_POLICY_ENTRY_LIST").field("pPolicyEntries", &self.pPolicyEntries).field("nEntries", &self.nEntries).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_POLICY_TAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_CONNECTION_POLICY_TAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_CONNECTION_POLICY_TAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_PROXY_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_CONNECTION_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_CONNECTION_PROXY_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_CONNECTION_PROXY_INFO_SWITCH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_CONNECTION_PROXY_INFO_SWITCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_CONNECTION_PROXY_INFO_SWITCH").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_PROXY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_CONNECTION_PROXY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cProxies == other.cProxies && self.pProxies == other.pProxies
    }
}
impl ::core::cmp::Eq for DNS_CONNECTION_PROXY_LIST {}
impl ::core::fmt::Debug for DNS_CONNECTION_PROXY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_PROXY_LIST").field("cProxies", &self.cProxies).field("pProxies", &self.pProxies).finish()
    }
}
impl ::core::default::Default for DNS_CONNECTION_PROXY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_CONNECTION_PROXY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_CONNECTION_PROXY_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_CUSTOM_SERVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_DHCID_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_DHCID_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwByteCount == other.dwByteCount && self.DHCID == other.DHCID
    }
}
impl ::core::cmp::Eq for DNS_DHCID_DATA {}
impl ::core::fmt::Debug for DNS_DHCID_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_DHCID_DATA").field("dwByteCount", &self.dwByteCount).field("DHCID", &self.DHCID).finish()
    }
}
impl ::core::default::Default for DNS_DS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_DS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wKeyTag == other.wKeyTag && self.chAlgorithm == other.chAlgorithm && self.chDigestType == other.chDigestType && self.wDigestLength == other.wDigestLength && self.wPad == other.wPad && self.Digest == other.Digest
    }
}
impl ::core::cmp::Eq for DNS_DS_DATA {}
impl ::core::fmt::Debug for DNS_DS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_DS_DATA").field("wKeyTag", &self.wKeyTag).field("chAlgorithm", &self.chAlgorithm).field("chDigestType", &self.chDigestType).field("wDigestLength", &self.wDigestLength).field("wPad", &self.wPad).field("Digest", &self.Digest).finish()
    }
}
impl ::core::default::Default for DNS_FREE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_FREE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_FREE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_HEADER_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_KEY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_KEY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wFlags == other.wFlags && self.chProtocol == other.chProtocol && self.chAlgorithm == other.chAlgorithm && self.wKeyLength == other.wKeyLength && self.wPad == other.wPad && self.Key == other.Key
    }
}
impl ::core::cmp::Eq for DNS_KEY_DATA {}
impl ::core::fmt::Debug for DNS_KEY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_KEY_DATA").field("wFlags", &self.wFlags).field("chProtocol", &self.chProtocol).field("chAlgorithm", &self.chAlgorithm).field("wKeyLength", &self.wKeyLength).field("wPad", &self.wPad).field("Key", &self.Key).finish()
    }
}
impl ::core::default::Default for DNS_LOC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_LOC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wSize == other.wSize && self.wHorPrec == other.wHorPrec && self.wVerPrec == other.wVerPrec && self.dwLatitude == other.dwLatitude && self.dwLongitude == other.dwLongitude && self.dwAltitude == other.dwAltitude
    }
}
impl ::core::cmp::Eq for DNS_LOC_DATA {}
impl ::core::fmt::Debug for DNS_LOC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_LOC_DATA").field("wVersion", &self.wVersion).field("wSize", &self.wSize).field("wHorPrec", &self.wHorPrec).field("wVerPrec", &self.wVerPrec).field("dwLatitude", &self.dwLatitude).field("dwLongitude", &self.dwLongitude).field("dwAltitude", &self.dwAltitude).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_MESSAGE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_MINFO_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_MINFO_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameMailbox == other.pNameMailbox && self.pNameErrorsMailbox == other.pNameErrorsMailbox
    }
}
impl ::core::cmp::Eq for DNS_MINFO_DATAA {}
impl ::core::fmt::Debug for DNS_MINFO_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_MINFO_DATAA").field("pNameMailbox", &self.pNameMailbox).field("pNameErrorsMailbox", &self.pNameErrorsMailbox).finish()
    }
}
impl ::core::default::Default for DNS_MINFO_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_MINFO_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameMailbox == other.pNameMailbox && self.pNameErrorsMailbox == other.pNameErrorsMailbox
    }
}
impl ::core::cmp::Eq for DNS_MINFO_DATAW {}
impl ::core::fmt::Debug for DNS_MINFO_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_MINFO_DATAW").field("pNameMailbox", &self.pNameMailbox).field("pNameErrorsMailbox", &self.pNameErrorsMailbox).finish()
    }
}
impl ::core::default::Default for DNS_MX_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_MX_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameExchange == other.pNameExchange && self.wPreference == other.wPreference && self.Pad == other.Pad
    }
}
impl ::core::cmp::Eq for DNS_MX_DATAA {}
impl ::core::fmt::Debug for DNS_MX_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_MX_DATAA").field("pNameExchange", &self.pNameExchange).field("wPreference", &self.wPreference).field("Pad", &self.Pad).finish()
    }
}
impl ::core::default::Default for DNS_MX_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_MX_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameExchange == other.pNameExchange && self.wPreference == other.wPreference && self.Pad == other.Pad
    }
}
impl ::core::cmp::Eq for DNS_MX_DATAW {}
impl ::core::fmt::Debug for DNS_MX_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_MX_DATAW").field("pNameExchange", &self.pNameExchange).field("wPreference", &self.wPreference).field("Pad", &self.Pad).finish()
    }
}
impl ::core::default::Default for DNS_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_NAME_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_NAPTR_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NAPTR_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.wOrder == other.wOrder && self.wPreference == other.wPreference && self.pFlags == other.pFlags && self.pService == other.pService && self.pRegularExpression == other.pRegularExpression && self.pReplacement == other.pReplacement
    }
}
impl ::core::cmp::Eq for DNS_NAPTR_DATAA {}
impl ::core::fmt::Debug for DNS_NAPTR_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NAPTR_DATAA").field("wOrder", &self.wOrder).field("wPreference", &self.wPreference).field("pFlags", &self.pFlags).field("pService", &self.pService).field("pRegularExpression", &self.pRegularExpression).field("pReplacement", &self.pReplacement).finish()
    }
}
impl ::core::default::Default for DNS_NAPTR_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NAPTR_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.wOrder == other.wOrder && self.wPreference == other.wPreference && self.pFlags == other.pFlags && self.pService == other.pService && self.pRegularExpression == other.pRegularExpression && self.pReplacement == other.pReplacement
    }
}
impl ::core::cmp::Eq for DNS_NAPTR_DATAW {}
impl ::core::fmt::Debug for DNS_NAPTR_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NAPTR_DATAW").field("wOrder", &self.wOrder).field("wPreference", &self.wPreference).field("pFlags", &self.pFlags).field("pService", &self.pService).field("pRegularExpression", &self.pRegularExpression).field("pReplacement", &self.pReplacement).finish()
    }
}
impl ::core::default::Default for DNS_NSEC3PARAM_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NSEC3PARAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.chAlgorithm == other.chAlgorithm && self.bFlags == other.bFlags && self.wIterations == other.wIterations && self.bSaltLength == other.bSaltLength && self.bPad == other.bPad && self.pbSalt == other.pbSalt
    }
}
impl ::core::cmp::Eq for DNS_NSEC3PARAM_DATA {}
impl ::core::fmt::Debug for DNS_NSEC3PARAM_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NSEC3PARAM_DATA").field("chAlgorithm", &self.chAlgorithm).field("bFlags", &self.bFlags).field("wIterations", &self.wIterations).field("bSaltLength", &self.bSaltLength).field("bPad", &self.bPad).field("pbSalt", &self.pbSalt).finish()
    }
}
impl ::core::default::Default for DNS_NSEC3_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NSEC3_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.chAlgorithm == other.chAlgorithm && self.bFlags == other.bFlags && self.wIterations == other.wIterations && self.bSaltLength == other.bSaltLength && self.bHashLength == other.bHashLength && self.wTypeBitMapsLength == other.wTypeBitMapsLength && self.chData == other.chData
    }
}
impl ::core::cmp::Eq for DNS_NSEC3_DATA {}
impl ::core::fmt::Debug for DNS_NSEC3_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NSEC3_DATA").field("chAlgorithm", &self.chAlgorithm).field("bFlags", &self.bFlags).field("wIterations", &self.wIterations).field("bSaltLength", &self.bSaltLength).field("bHashLength", &self.bHashLength).field("wTypeBitMapsLength", &self.wTypeBitMapsLength).field("chData", &self.chData).finish()
    }
}
impl ::core::default::Default for DNS_NSEC_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NSEC_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNextDomainName == other.pNextDomainName && self.wTypeBitMapsLength == other.wTypeBitMapsLength && self.wPad == other.wPad && self.TypeBitMaps == other.TypeBitMaps
    }
}
impl ::core::cmp::Eq for DNS_NSEC_DATAA {}
impl ::core::fmt::Debug for DNS_NSEC_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NSEC_DATAA").field("pNextDomainName", &self.pNextDomainName).field("wTypeBitMapsLength", &self.wTypeBitMapsLength).field("wPad", &self.wPad).field("TypeBitMaps", &self.TypeBitMaps).finish()
    }
}
impl ::core::default::Default for DNS_NSEC_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NSEC_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNextDomainName == other.pNextDomainName && self.wTypeBitMapsLength == other.wTypeBitMapsLength && self.wPad == other.wPad && self.TypeBitMaps == other.TypeBitMaps
    }
}
impl ::core::cmp::Eq for DNS_NSEC_DATAW {}
impl ::core::fmt::Debug for DNS_NSEC_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NSEC_DATAW").field("pNextDomainName", &self.pNextDomainName).field("wTypeBitMapsLength", &self.wTypeBitMapsLength).field("wPad", &self.wPad).field("TypeBitMaps", &self.TypeBitMaps).finish()
    }
}
impl ::core::default::Default for DNS_NULL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NULL_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwByteCount == other.dwByteCount && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DNS_NULL_DATA {}
impl ::core::fmt::Debug for DNS_NULL_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NULL_DATA").field("dwByteCount", &self.dwByteCount).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for DNS_NXT_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NXT_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameNext == other.pNameNext && self.wNumTypes == other.wNumTypes && self.wTypes == other.wTypes
    }
}
impl ::core::cmp::Eq for DNS_NXT_DATAA {}
impl ::core::fmt::Debug for DNS_NXT_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NXT_DATAA").field("pNameNext", &self.pNameNext).field("wNumTypes", &self.wNumTypes).field("wTypes", &self.wTypes).finish()
    }
}
impl ::core::default::Default for DNS_NXT_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_NXT_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameNext == other.pNameNext && self.wNumTypes == other.wNumTypes && self.wTypes == other.wTypes
    }
}
impl ::core::cmp::Eq for DNS_NXT_DATAW {}
impl ::core::fmt::Debug for DNS_NXT_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_NXT_DATAW").field("pNameNext", &self.pNameNext).field("wNumTypes", &self.wNumTypes).field("wTypes", &self.wTypes).finish()
    }
}
impl ::core::default::Default for DNS_OPT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_OPT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wDataLength == other.wDataLength && self.wPad == other.wPad && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DNS_OPT_DATA {}
impl ::core::fmt::Debug for DNS_OPT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_OPT_DATA").field("wDataLength", &self.wDataLength).field("wPad", &self.wPad).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for DNS_PROXY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_PROXY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.proxyInformationType == other.proxyInformationType && self.proxyName == other.proxyName
    }
}
impl ::core::cmp::Eq for DNS_PROXY_INFORMATION {}
impl ::core::fmt::Debug for DNS_PROXY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_PROXY_INFORMATION").field("version", &self.version).field("proxyInformationType", &self.proxyInformationType).field("proxyName", &self.proxyName).finish()
    }
}
impl ::core::default::Default for DNS_PROXY_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_PROXY_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_PROXY_INFORMATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_PTR_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_PTR_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameHost == other.pNameHost
    }
}
impl ::core::cmp::Eq for DNS_PTR_DATAA {}
impl ::core::fmt::Debug for DNS_PTR_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_PTR_DATAA").field("pNameHost", &self.pNameHost).finish()
    }
}
impl ::core::default::Default for DNS_PTR_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_PTR_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameHost == other.pNameHost
    }
}
impl ::core::cmp::Eq for DNS_PTR_DATAW {}
impl ::core::fmt::Debug for DNS_PTR_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_PTR_DATAW").field("pNameHost", &self.pNameHost).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_QUERY_CANCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DNS_QUERY_CANCEL {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DNS_QUERY_CANCEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DNS_QUERY_CANCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_QUERY_CANCEL").field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DNS_QUERY_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_QUERY_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_QUERY_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DNS_QUERY_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DNS_QUERY_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DNS_QUERY_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DNS_QUERY_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DNS_QUERY_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_QUERY_REQUEST3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_QUERY_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DNS_QUERY_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.QueryStatus == other.QueryStatus && self.QueryOptions == other.QueryOptions && self.pQueryRecords == other.pQueryRecords && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DNS_QUERY_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DNS_QUERY_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_QUERY_RESULT").field("Version", &self.Version).field("QueryStatus", &self.QueryStatus).field("QueryOptions", &self.QueryOptions).field("pQueryRecords", &self.pQueryRecords).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_RECORDA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_RECORDW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_RECORD_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_RECORD_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DNS_RECORD_FLAGS {}
impl ::core::fmt::Debug for DNS_RECORD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_RECORD_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_RECORD_OPTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_RRSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DNS_RRSET {
    fn eq(&self, other: &Self) -> bool {
        self.pFirstRR == other.pFirstRR && self.pLastRR == other.pLastRR
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DNS_RRSET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DNS_RRSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_RRSET").field("pFirstRR", &self.pFirstRR).field("pLastRR", &self.pLastRR).finish()
    }
}
impl ::core::default::Default for DNS_SECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_SECTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_SERVICE_BROWSE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_SERVICE_CANCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SERVICE_CANCEL {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for DNS_SERVICE_CANCEL {}
impl ::core::fmt::Debug for DNS_SERVICE_CANCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SERVICE_CANCEL").field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for DNS_SERVICE_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SERVICE_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.pszInstanceName == other.pszInstanceName && self.pszHostName == other.pszHostName && self.ip4Address == other.ip4Address && self.ip6Address == other.ip6Address && self.wPort == other.wPort && self.wPriority == other.wPriority && self.wWeight == other.wWeight && self.dwPropertyCount == other.dwPropertyCount && self.keys == other.keys && self.values == other.values && self.dwInterfaceIndex == other.dwInterfaceIndex
    }
}
impl ::core::cmp::Eq for DNS_SERVICE_INSTANCE {}
impl ::core::fmt::Debug for DNS_SERVICE_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SERVICE_INSTANCE").field("pszInstanceName", &self.pszInstanceName).field("pszHostName", &self.pszHostName).field("ip4Address", &self.ip4Address).field("ip6Address", &self.ip6Address).field("wPort", &self.wPort).field("wPriority", &self.wPriority).field("wWeight", &self.wWeight).field("dwPropertyCount", &self.dwPropertyCount).field("keys", &self.keys).field("values", &self.values).field("dwInterfaceIndex", &self.dwInterfaceIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_SERVICE_REGISTER_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_SERVICE_RESOLVE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_SIG_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SIG_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.wTypeCovered == other.wTypeCovered && self.chAlgorithm == other.chAlgorithm && self.chLabelCount == other.chLabelCount && self.dwOriginalTtl == other.dwOriginalTtl && self.dwExpiration == other.dwExpiration && self.dwTimeSigned == other.dwTimeSigned && self.wKeyTag == other.wKeyTag && self.wSignatureLength == other.wSignatureLength && self.pNameSigner == other.pNameSigner && self.Signature == other.Signature
    }
}
impl ::core::cmp::Eq for DNS_SIG_DATAA {}
impl ::core::fmt::Debug for DNS_SIG_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SIG_DATAA").field("wTypeCovered", &self.wTypeCovered).field("chAlgorithm", &self.chAlgorithm).field("chLabelCount", &self.chLabelCount).field("dwOriginalTtl", &self.dwOriginalTtl).field("dwExpiration", &self.dwExpiration).field("dwTimeSigned", &self.dwTimeSigned).field("wKeyTag", &self.wKeyTag).field("wSignatureLength", &self.wSignatureLength).field("pNameSigner", &self.pNameSigner).field("Signature", &self.Signature).finish()
    }
}
impl ::core::default::Default for DNS_SIG_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SIG_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.wTypeCovered == other.wTypeCovered && self.chAlgorithm == other.chAlgorithm && self.chLabelCount == other.chLabelCount && self.dwOriginalTtl == other.dwOriginalTtl && self.dwExpiration == other.dwExpiration && self.dwTimeSigned == other.dwTimeSigned && self.wKeyTag == other.wKeyTag && self.wSignatureLength == other.wSignatureLength && self.pNameSigner == other.pNameSigner && self.Signature == other.Signature
    }
}
impl ::core::cmp::Eq for DNS_SIG_DATAW {}
impl ::core::fmt::Debug for DNS_SIG_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SIG_DATAW").field("wTypeCovered", &self.wTypeCovered).field("chAlgorithm", &self.chAlgorithm).field("chLabelCount", &self.chLabelCount).field("dwOriginalTtl", &self.dwOriginalTtl).field("dwExpiration", &self.dwExpiration).field("dwTimeSigned", &self.dwTimeSigned).field("wKeyTag", &self.wKeyTag).field("wSignatureLength", &self.wSignatureLength).field("pNameSigner", &self.pNameSigner).field("Signature", &self.Signature).finish()
    }
}
impl ::core::default::Default for DNS_SOA_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SOA_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNamePrimaryServer == other.pNamePrimaryServer && self.pNameAdministrator == other.pNameAdministrator && self.dwSerialNo == other.dwSerialNo && self.dwRefresh == other.dwRefresh && self.dwRetry == other.dwRetry && self.dwExpire == other.dwExpire && self.dwDefaultTtl == other.dwDefaultTtl
    }
}
impl ::core::cmp::Eq for DNS_SOA_DATAA {}
impl ::core::fmt::Debug for DNS_SOA_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SOA_DATAA").field("pNamePrimaryServer", &self.pNamePrimaryServer).field("pNameAdministrator", &self.pNameAdministrator).field("dwSerialNo", &self.dwSerialNo).field("dwRefresh", &self.dwRefresh).field("dwRetry", &self.dwRetry).field("dwExpire", &self.dwExpire).field("dwDefaultTtl", &self.dwDefaultTtl).finish()
    }
}
impl ::core::default::Default for DNS_SOA_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SOA_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNamePrimaryServer == other.pNamePrimaryServer && self.pNameAdministrator == other.pNameAdministrator && self.dwSerialNo == other.dwSerialNo && self.dwRefresh == other.dwRefresh && self.dwRetry == other.dwRetry && self.dwExpire == other.dwExpire && self.dwDefaultTtl == other.dwDefaultTtl
    }
}
impl ::core::cmp::Eq for DNS_SOA_DATAW {}
impl ::core::fmt::Debug for DNS_SOA_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SOA_DATAW").field("pNamePrimaryServer", &self.pNamePrimaryServer).field("pNameAdministrator", &self.pNameAdministrator).field("dwSerialNo", &self.dwSerialNo).field("dwRefresh", &self.dwRefresh).field("dwRetry", &self.dwRetry).field("dwExpire", &self.dwExpire).field("dwDefaultTtl", &self.dwDefaultTtl).finish()
    }
}
impl ::core::default::Default for DNS_SRV_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SRV_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameTarget == other.pNameTarget && self.wPriority == other.wPriority && self.wWeight == other.wWeight && self.wPort == other.wPort && self.Pad == other.Pad
    }
}
impl ::core::cmp::Eq for DNS_SRV_DATAA {}
impl ::core::fmt::Debug for DNS_SRV_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SRV_DATAA").field("pNameTarget", &self.pNameTarget).field("wPriority", &self.wPriority).field("wWeight", &self.wWeight).field("wPort", &self.wPort).field("Pad", &self.Pad).finish()
    }
}
impl ::core::default::Default for DNS_SRV_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SRV_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameTarget == other.pNameTarget && self.wPriority == other.wPriority && self.wWeight == other.wWeight && self.wPort == other.wPort && self.Pad == other.Pad
    }
}
impl ::core::cmp::Eq for DNS_SRV_DATAW {}
impl ::core::fmt::Debug for DNS_SRV_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SRV_DATAW").field("pNameTarget", &self.pNameTarget).field("wPriority", &self.wPriority).field("wWeight", &self.wWeight).field("wPort", &self.wPort).field("Pad", &self.Pad).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_TKEY_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DNS_TKEY_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameAlgorithm == other.pNameAlgorithm && self.pAlgorithmPacket == other.pAlgorithmPacket && self.pKey == other.pKey && self.pOtherData == other.pOtherData && self.dwCreateTime == other.dwCreateTime && self.dwExpireTime == other.dwExpireTime && self.wMode == other.wMode && self.wError == other.wError && self.wKeyLength == other.wKeyLength && self.wOtherLength == other.wOtherLength && self.cAlgNameLength == other.cAlgNameLength && self.bPacketPointers == other.bPacketPointers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DNS_TKEY_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DNS_TKEY_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_TKEY_DATAA")
            .field("pNameAlgorithm", &self.pNameAlgorithm)
            .field("pAlgorithmPacket", &self.pAlgorithmPacket)
            .field("pKey", &self.pKey)
            .field("pOtherData", &self.pOtherData)
            .field("dwCreateTime", &self.dwCreateTime)
            .field("dwExpireTime", &self.dwExpireTime)
            .field("wMode", &self.wMode)
            .field("wError", &self.wError)
            .field("wKeyLength", &self.wKeyLength)
            .field("wOtherLength", &self.wOtherLength)
            .field("cAlgNameLength", &self.cAlgNameLength)
            .field("bPacketPointers", &self.bPacketPointers)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_TKEY_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DNS_TKEY_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameAlgorithm == other.pNameAlgorithm && self.pAlgorithmPacket == other.pAlgorithmPacket && self.pKey == other.pKey && self.pOtherData == other.pOtherData && self.dwCreateTime == other.dwCreateTime && self.dwExpireTime == other.dwExpireTime && self.wMode == other.wMode && self.wError == other.wError && self.wKeyLength == other.wKeyLength && self.wOtherLength == other.wOtherLength && self.cAlgNameLength == other.cAlgNameLength && self.bPacketPointers == other.bPacketPointers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DNS_TKEY_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DNS_TKEY_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_TKEY_DATAW")
            .field("pNameAlgorithm", &self.pNameAlgorithm)
            .field("pAlgorithmPacket", &self.pAlgorithmPacket)
            .field("pKey", &self.pKey)
            .field("pOtherData", &self.pOtherData)
            .field("dwCreateTime", &self.dwCreateTime)
            .field("dwExpireTime", &self.dwExpireTime)
            .field("wMode", &self.wMode)
            .field("wError", &self.wError)
            .field("wKeyLength", &self.wKeyLength)
            .field("wOtherLength", &self.wOtherLength)
            .field("cAlgNameLength", &self.cAlgNameLength)
            .field("bPacketPointers", &self.bPacketPointers)
            .finish()
    }
}
impl ::core::default::Default for DNS_TLSA_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_TLSA_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.bCertUsage == other.bCertUsage && self.bSelector == other.bSelector && self.bMatchingType == other.bMatchingType && self.bCertificateAssociationDataLength == other.bCertificateAssociationDataLength && self.bPad == other.bPad && self.bCertificateAssociationData == other.bCertificateAssociationData
    }
}
impl ::core::cmp::Eq for DNS_TLSA_DATA {}
impl ::core::fmt::Debug for DNS_TLSA_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_TLSA_DATA").field("bCertUsage", &self.bCertUsage).field("bSelector", &self.bSelector).field("bMatchingType", &self.bMatchingType).field("bCertificateAssociationDataLength", &self.bCertificateAssociationDataLength).field("bPad", &self.bPad).field("bCertificateAssociationData", &self.bCertificateAssociationData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_TSIG_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DNS_TSIG_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameAlgorithm == other.pNameAlgorithm && self.pAlgorithmPacket == other.pAlgorithmPacket && self.pSignature == other.pSignature && self.pOtherData == other.pOtherData && self.i64CreateTime == other.i64CreateTime && self.wFudgeTime == other.wFudgeTime && self.wOriginalXid == other.wOriginalXid && self.wError == other.wError && self.wSigLength == other.wSigLength && self.wOtherLength == other.wOtherLength && self.cAlgNameLength == other.cAlgNameLength && self.bPacketPointers == other.bPacketPointers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DNS_TSIG_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DNS_TSIG_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_TSIG_DATAA")
            .field("pNameAlgorithm", &self.pNameAlgorithm)
            .field("pAlgorithmPacket", &self.pAlgorithmPacket)
            .field("pSignature", &self.pSignature)
            .field("pOtherData", &self.pOtherData)
            .field("i64CreateTime", &self.i64CreateTime)
            .field("wFudgeTime", &self.wFudgeTime)
            .field("wOriginalXid", &self.wOriginalXid)
            .field("wError", &self.wError)
            .field("wSigLength", &self.wSigLength)
            .field("wOtherLength", &self.wOtherLength)
            .field("cAlgNameLength", &self.cAlgNameLength)
            .field("bPacketPointers", &self.bPacketPointers)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DNS_TSIG_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DNS_TSIG_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameAlgorithm == other.pNameAlgorithm && self.pAlgorithmPacket == other.pAlgorithmPacket && self.pSignature == other.pSignature && self.pOtherData == other.pOtherData && self.i64CreateTime == other.i64CreateTime && self.wFudgeTime == other.wFudgeTime && self.wOriginalXid == other.wOriginalXid && self.wError == other.wError && self.wSigLength == other.wSigLength && self.wOtherLength == other.wOtherLength && self.cAlgNameLength == other.cAlgNameLength && self.bPacketPointers == other.bPacketPointers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DNS_TSIG_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DNS_TSIG_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_TSIG_DATAW")
            .field("pNameAlgorithm", &self.pNameAlgorithm)
            .field("pAlgorithmPacket", &self.pAlgorithmPacket)
            .field("pSignature", &self.pSignature)
            .field("pOtherData", &self.pOtherData)
            .field("i64CreateTime", &self.i64CreateTime)
            .field("wFudgeTime", &self.wFudgeTime)
            .field("wOriginalXid", &self.wOriginalXid)
            .field("wError", &self.wError)
            .field("wSigLength", &self.wSigLength)
            .field("wOtherLength", &self.wOtherLength)
            .field("cAlgNameLength", &self.cAlgNameLength)
            .field("bPacketPointers", &self.bPacketPointers)
            .finish()
    }
}
impl ::core::default::Default for DNS_TXT_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_TXT_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStringCount == other.dwStringCount && self.pStringArray == other.pStringArray
    }
}
impl ::core::cmp::Eq for DNS_TXT_DATAA {}
impl ::core::fmt::Debug for DNS_TXT_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_TXT_DATAA").field("dwStringCount", &self.dwStringCount).field("pStringArray", &self.pStringArray).finish()
    }
}
impl ::core::default::Default for DNS_TXT_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_TXT_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStringCount == other.dwStringCount && self.pStringArray == other.pStringArray
    }
}
impl ::core::cmp::Eq for DNS_TXT_DATAW {}
impl ::core::fmt::Debug for DNS_TXT_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_TXT_DATAW").field("dwStringCount", &self.dwStringCount).field("pStringArray", &self.pStringArray).finish()
    }
}
impl ::core::default::Default for DNS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_UNKNOWN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_UNKNOWN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwByteCount == other.dwByteCount && self.bData == other.bData
    }
}
impl ::core::cmp::Eq for DNS_UNKNOWN_DATA {}
impl ::core::fmt::Debug for DNS_UNKNOWN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_UNKNOWN_DATA").field("dwByteCount", &self.dwByteCount).field("bData", &self.bData).finish()
    }
}
impl ::core::default::Default for DNS_WINSR_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_WINSR_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwMappingFlag == other.dwMappingFlag && self.dwLookupTimeout == other.dwLookupTimeout && self.dwCacheTimeout == other.dwCacheTimeout && self.pNameResultDomain == other.pNameResultDomain
    }
}
impl ::core::cmp::Eq for DNS_WINSR_DATAA {}
impl ::core::fmt::Debug for DNS_WINSR_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_WINSR_DATAA").field("dwMappingFlag", &self.dwMappingFlag).field("dwLookupTimeout", &self.dwLookupTimeout).field("dwCacheTimeout", &self.dwCacheTimeout).field("pNameResultDomain", &self.pNameResultDomain).finish()
    }
}
impl ::core::default::Default for DNS_WINSR_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_WINSR_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwMappingFlag == other.dwMappingFlag && self.dwLookupTimeout == other.dwLookupTimeout && self.dwCacheTimeout == other.dwCacheTimeout && self.pNameResultDomain == other.pNameResultDomain
    }
}
impl ::core::cmp::Eq for DNS_WINSR_DATAW {}
impl ::core::fmt::Debug for DNS_WINSR_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_WINSR_DATAW").field("dwMappingFlag", &self.dwMappingFlag).field("dwLookupTimeout", &self.dwLookupTimeout).field("dwCacheTimeout", &self.dwCacheTimeout).field("pNameResultDomain", &self.pNameResultDomain).finish()
    }
}
impl ::core::default::Default for DNS_WINS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_WINS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwMappingFlag == other.dwMappingFlag && self.dwLookupTimeout == other.dwLookupTimeout && self.dwCacheTimeout == other.dwCacheTimeout && self.cWinsServerCount == other.cWinsServerCount && self.WinsServers == other.WinsServers
    }
}
impl ::core::cmp::Eq for DNS_WINS_DATA {}
impl ::core::fmt::Debug for DNS_WINS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_WINS_DATA").field("dwMappingFlag", &self.dwMappingFlag).field("dwLookupTimeout", &self.dwLookupTimeout).field("dwCacheTimeout", &self.dwCacheTimeout).field("cWinsServerCount", &self.cWinsServerCount).field("WinsServers", &self.WinsServers).finish()
    }
}
impl ::core::default::Default for DNS_WIRE_QUESTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_WIRE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_WKS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_WKS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress && self.chProtocol == other.chProtocol && self.BitMask == other.BitMask
    }
}
impl ::core::cmp::Eq for DNS_WKS_DATA {}
impl ::core::fmt::Debug for DNS_WKS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_WKS_DATA").field("IpAddress", &self.IpAddress).field("chProtocol", &self.chProtocol).field("BitMask", &self.BitMask).finish()
    }
}
impl ::core::default::Default for IP4_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP4_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.AddrCount == other.AddrCount && self.AddrArray == other.AddrArray
    }
}
impl ::core::cmp::Eq for IP4_ARRAY {}
impl ::core::fmt::Debug for IP4_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP4_ARRAY").field("AddrCount", &self.AddrCount).field("AddrArray", &self.AddrArray).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for IP6_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IP6_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MDNS_QUERY_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MDNS_QUERY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.nameBuf == other.nameBuf && self.wType == other.wType && self.pSubscription == other.pSubscription && self.pWnfCallbackParams == other.pWnfCallbackParams && self.stateNameData == other.stateNameData
    }
}
impl ::core::cmp::Eq for MDNS_QUERY_HANDLE {}
impl ::core::fmt::Debug for MDNS_QUERY_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDNS_QUERY_HANDLE").field("nameBuf", &self.nameBuf).field("wType", &self.wType).field("pSubscription", &self.pSubscription).field("pWnfCallbackParams", &self.pWnfCallbackParams).field("stateNameData", &self.stateNameData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MDNS_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _DnsRecordOptA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
