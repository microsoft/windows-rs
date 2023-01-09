impl ::core::default::Default for DATE_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DATE_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowDateTime == other.dwLowDateTime && self.dwHighDateTime == other.dwHighDateTime
    }
}
impl ::core::cmp::Eq for DATE_TIME {}
impl ::core::fmt::Debug for DATE_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATE_TIME").field("dwLowDateTime", &self.dwLowDateTime).field("dwHighDateTime", &self.dwHighDateTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPAPI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPAPI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.OptionId == other.OptionId && self.IsVendor == other.IsVendor && self.Data == other.Data && self.nBytesData == other.nBytesData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPAPI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPAPI_PARAMS").field("Flags", &self.Flags).field("OptionId", &self.OptionId).field("IsVendor", &self.IsVendor).field("Data", &self.Data).field("nBytesData", &self.nBytesData).finish()
    }
}
impl ::core::default::Default for DHCPCAPI_CLASSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPCAPI_CLASSID {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Data == other.Data && self.nBytesData == other.nBytesData
    }
}
impl ::core::cmp::Eq for DHCPCAPI_CLASSID {}
impl ::core::fmt::Debug for DHCPCAPI_CLASSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPCAPI_CLASSID").field("Flags", &self.Flags).field("Data", &self.Data).field("nBytesData", &self.nBytesData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPCAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPCAPI_PARAMS_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.nParams == other.nParams && self.Params == other.Params
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPCAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPCAPI_PARAMS_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPCAPI_PARAMS_ARRAY").field("nParams", &self.nParams).field("Params", &self.Params).finish()
    }
}
impl ::core::default::Default for DHCPDS_SERVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPDS_SERVER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ServerName == other.ServerName && self.ServerAddress == other.ServerAddress && self.Flags == other.Flags && self.State == other.State && self.DsLocation == other.DsLocation && self.DsLocType == other.DsLocType
    }
}
impl ::core::cmp::Eq for DHCPDS_SERVER {}
impl ::core::fmt::Debug for DHCPDS_SERVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPDS_SERVER").field("Version", &self.Version).field("ServerName", &self.ServerName).field("ServerAddress", &self.ServerAddress).field("Flags", &self.Flags).field("State", &self.State).field("DsLocation", &self.DsLocation).field("DsLocType", &self.DsLocType).finish()
    }
}
impl ::core::default::Default for DHCPDS_SERVERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPDS_SERVERS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumElements == other.NumElements && self.Servers == other.Servers
    }
}
impl ::core::cmp::Eq for DHCPDS_SERVERS {}
impl ::core::fmt::Debug for DHCPDS_SERVERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPDS_SERVERS").field("Flags", &self.Flags).field("NumElements", &self.NumElements).field("Servers", &self.Servers).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress
            && self.SubnetMask == other.SubnetMask
            && self.ClientHardwareAddress == other.ClientHardwareAddress
            && self.ClientName == other.ClientName
            && self.ClientComment == other.ClientComment
            && self.ClientLeaseExpires == other.ClientLeaseExpires
            && self.OwnerHost == other.OwnerHost
            && self.bClientType == other.bClientType
            && self.AddressState == other.AddressState
            && self.Status == other.Status
            && self.ProbationEnds == other.ProbationEnds
            && self.QuarantineCapable == other.QuarantineCapable
            && self.SentPotExpTime == other.SentPotExpTime
            && self.AckPotExpTime == other.AckPotExpTime
            && self.RecvPotExpTime == other.RecvPotExpTime
            && self.StartTime == other.StartTime
            && self.CltLastTransTime == other.CltLastTransTime
            && self.LastBndUpdTime == other.LastBndUpdTime
            && self.BndMsgStatus == other.BndMsgStatus
            && self.PolicyName == other.PolicyName
            && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV4_FAILOVER_CLIENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV4_FAILOVER_CLIENT_INFO")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("SentPotExpTime", &self.SentPotExpTime)
            .field("AckPotExpTime", &self.AckPotExpTime)
            .field("RecvPotExpTime", &self.RecvPotExpTime)
            .field("StartTime", &self.StartTime)
            .field("CltLastTransTime", &self.CltLastTransTime)
            .field("LastBndUpdTime", &self.LastBndUpdTime)
            .field("BndMsgStatus", &self.BndMsgStatus)
            .field("PolicyName", &self.PolicyName)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV4_FAILOVER_CLIENT_INFO_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress
            && self.SubnetMask == other.SubnetMask
            && self.ClientHardwareAddress == other.ClientHardwareAddress
            && self.ClientName == other.ClientName
            && self.ClientComment == other.ClientComment
            && self.ClientLeaseExpires == other.ClientLeaseExpires
            && self.OwnerHost == other.OwnerHost
            && self.bClientType == other.bClientType
            && self.AddressState == other.AddressState
            && self.Status == other.Status
            && self.ProbationEnds == other.ProbationEnds
            && self.QuarantineCapable == other.QuarantineCapable
            && self.SentPotExpTime == other.SentPotExpTime
            && self.AckPotExpTime == other.AckPotExpTime
            && self.RecvPotExpTime == other.RecvPotExpTime
            && self.StartTime == other.StartTime
            && self.CltLastTransTime == other.CltLastTransTime
            && self.LastBndUpdTime == other.LastBndUpdTime
            && self.BndMsgStatus == other.BndMsgStatus
            && self.PolicyName == other.PolicyName
            && self.Flags == other.Flags
            && self.AddressStateEx == other.AddressStateEx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV4_FAILOVER_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV4_FAILOVER_CLIENT_INFO_EX")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("SentPotExpTime", &self.SentPotExpTime)
            .field("AckPotExpTime", &self.AckPotExpTime)
            .field("RecvPotExpTime", &self.RecvPotExpTime)
            .field("StartTime", &self.StartTime)
            .field("CltLastTransTime", &self.CltLastTransTime)
            .field("LastBndUpdTime", &self.LastBndUpdTime)
            .field("BndMsgStatus", &self.BndMsgStatus)
            .field("PolicyName", &self.PolicyName)
            .field("Flags", &self.Flags)
            .field("AddressStateEx", &self.AddressStateEx)
            .finish()
    }
}
impl ::core::default::Default for DHCPV6CAPI_CLASSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPV6CAPI_CLASSID {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Data == other.Data && self.nBytesData == other.nBytesData
    }
}
impl ::core::cmp::Eq for DHCPV6CAPI_CLASSID {}
impl ::core::fmt::Debug for DHCPV6CAPI_CLASSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6CAPI_CLASSID").field("Flags", &self.Flags).field("Data", &self.Data).field("nBytesData", &self.nBytesData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6CAPI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6CAPI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.OptionId == other.OptionId && self.IsVendor == other.IsVendor && self.Data == other.Data && self.nBytesData == other.nBytesData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6CAPI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6CAPI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6CAPI_PARAMS").field("Flags", &self.Flags).field("OptionId", &self.OptionId).field("IsVendor", &self.IsVendor).field("Data", &self.Data).field("nBytesData", &self.nBytesData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6CAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6CAPI_PARAMS_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.nParams == other.nParams && self.Params == other.Params
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6CAPI_PARAMS_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6CAPI_PARAMS_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6CAPI_PARAMS_ARRAY").field("nParams", &self.nParams).field("Params", &self.Params).finish()
    }
}
impl ::core::default::Default for DHCPV6Prefix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPV6Prefix {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix && self.prefixLength == other.prefixLength && self.preferredLifeTime == other.preferredLifeTime && self.validLifeTime == other.validLifeTime && self.status == other.status
    }
}
impl ::core::cmp::Eq for DHCPV6Prefix {}
impl ::core::fmt::Debug for DHCPV6Prefix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6Prefix").field("prefix", &self.prefix).field("prefixLength", &self.prefixLength).field("preferredLifeTime", &self.preferredLifeTime).field("validLifeTime", &self.validLifeTime).field("status", &self.status).finish()
    }
}
impl ::core::default::Default for DHCPV6PrefixLeaseInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPV6PrefixLeaseInformation {
    fn eq(&self, other: &Self) -> bool {
        self.nPrefixes == other.nPrefixes && self.prefixArray == other.prefixArray && self.iaid == other.iaid && self.T1 == other.T1 && self.T2 == other.T2 && self.MaxLeaseExpirationTime == other.MaxLeaseExpirationTime && self.LastRenewalTime == other.LastRenewalTime && self.status == other.status && self.ServerId == other.ServerId && self.ServerIdLen == other.ServerIdLen
    }
}
impl ::core::cmp::Eq for DHCPV6PrefixLeaseInformation {}
impl ::core::fmt::Debug for DHCPV6PrefixLeaseInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6PrefixLeaseInformation").field("nPrefixes", &self.nPrefixes).field("prefixArray", &self.prefixArray).field("iaid", &self.iaid).field("T1", &self.T1).field("T2", &self.T2).field("MaxLeaseExpirationTime", &self.MaxLeaseExpirationTime).field("LastRenewalTime", &self.LastRenewalTime).field("status", &self.status).field("ServerId", &self.ServerId).field("ServerIdLen", &self.ServerIdLen).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_BIND_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.fBoundToDHCPServer == other.fBoundToDHCPServer && self.AdapterPrimaryAddress == other.AdapterPrimaryAddress && self.AdapterSubnetAddress == other.AdapterSubnetAddress && self.IfDescription == other.IfDescription && self.IpV6IfIndex == other.IpV6IfIndex && self.IfIdSize == other.IfIdSize && self.IfId == other.IfId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6_BIND_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_BIND_ELEMENT").field("Flags", &self.Flags).field("fBoundToDHCPServer", &self.fBoundToDHCPServer).field("AdapterPrimaryAddress", &self.AdapterPrimaryAddress).field("AdapterSubnetAddress", &self.AdapterSubnetAddress).field("IfDescription", &self.IfDescription).field("IpV6IfIndex", &self.IpV6IfIndex).field("IfIdSize", &self.IfIdSize).field("IfId", &self.IfId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_BIND_ELEMENT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6_BIND_ELEMENT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_BIND_ELEMENT_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCPV6_IP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPV6_IP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCPV6_IP_ARRAY {}
impl ::core::fmt::Debug for DHCPV6_IP_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_IP_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCPV6_STATELESS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.PurgeInterval == other.PurgeInterval
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCPV6_STATELESS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCPV6_STATELESS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_STATELESS_PARAMS").field("Status", &self.Status).field("PurgeInterval", &self.PurgeInterval).finish()
    }
}
impl ::core::default::Default for DHCPV6_STATELESS_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCPV6_STATELESS_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCPV6_STATELESS_PARAM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCPV6_STATELESS_SCOPE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_SCOPE_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.NumStatelessClientsAdded == other.NumStatelessClientsAdded && self.NumStatelessClientsRemoved == other.NumStatelessClientsRemoved
    }
}
impl ::core::cmp::Eq for DHCPV6_STATELESS_SCOPE_STATS {}
impl ::core::fmt::Debug for DHCPV6_STATELESS_SCOPE_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_STATELESS_SCOPE_STATS").field("SubnetAddress", &self.SubnetAddress).field("NumStatelessClientsAdded", &self.NumStatelessClientsAdded).field("NumStatelessClientsRemoved", &self.NumStatelessClientsRemoved).finish()
    }
}
impl ::core::default::Default for DHCPV6_STATELESS_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCPV6_STATELESS_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.NumScopes == other.NumScopes && self.ScopeStats == other.ScopeStats
    }
}
impl ::core::cmp::Eq for DHCPV6_STATELESS_STATS {}
impl ::core::fmt::Debug for DHCPV6_STATELESS_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCPV6_STATELESS_STATS").field("NumScopes", &self.NumScopes).field("ScopeStats", &self.ScopeStats).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ADDR_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ADDR_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        self.MatchHWType == other.MatchHWType && self.HWType == other.HWType && self.IsWildcard == other.IsWildcard && self.Length == other.Length && self.Pattern == other.Pattern
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ADDR_PATTERN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ADDR_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ADDR_PATTERN").field("MatchHWType", &self.MatchHWType).field("HWType", &self.HWType).field("IsWildcard", &self.IsWildcard).field("Length", &self.Length).field("Pattern", &self.Pattern).finish()
    }
}
impl ::core::default::Default for DHCP_ALL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_ALL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NonVendorOptions == other.NonVendorOptions && self.NumVendorOptions == other.NumVendorOptions && self.VendorOptions == other.VendorOptions
    }
}
impl ::core::cmp::Eq for DHCP_ALL_OPTIONS {}
impl ::core::fmt::Debug for DHCP_ALL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTIONS").field("Flags", &self.Flags).field("NonVendorOptions", &self.NonVendorOptions).field("NumVendorOptions", &self.NumVendorOptions).field("VendorOptions", &self.VendorOptions).finish()
    }
}
impl ::core::default::Default for DHCP_ALL_OPTIONS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_ALL_OPTIONS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Option == other.Option && self.VendorName == other.VendorName && self.ClassName == other.ClassName
    }
}
impl ::core::cmp::Eq for DHCP_ALL_OPTIONS_0 {}
impl ::core::fmt::Debug for DHCP_ALL_OPTIONS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTIONS_0").field("Option", &self.Option).field("VendorName", &self.VendorName).field("ClassName", &self.ClassName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumElements == other.NumElements && self.Options == other.Options
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ALL_OPTION_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTION_VALUES").field("Flags", &self.Flags).field("NumElements", &self.NumElements).field("Options", &self.Options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ClassName == other.ClassName && self.VendorName == other.VendorName && self.IsVendor == other.IsVendor && self.OptionsArray == other.OptionsArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ALL_OPTION_VALUES_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTION_VALUES_0").field("ClassName", &self.ClassName).field("VendorName", &self.VendorName).field("IsVendor", &self.IsVendor).field("OptionsArray", &self.OptionsArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_PB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_PB {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumElements == other.NumElements && self.Options == other.Options
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ALL_OPTION_VALUES_PB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTION_VALUES_PB").field("Flags", &self.Flags).field("NumElements", &self.NumElements).field("Options", &self.Options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PolicyName == other.PolicyName && self.VendorName == other.VendorName && self.IsVendor == other.IsVendor && self.OptionsArray == other.OptionsArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ALL_OPTION_VALUES_PB_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ALL_OPTION_VALUES_PB_0").field("PolicyName", &self.PolicyName).field("VendorName", &self.VendorName).field("IsVendor", &self.IsVendor).field("OptionsArray", &self.OptionsArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ATTRIB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_ATTRIB_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_ATTRIB_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.DhcpAttribs == other.DhcpAttribs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_ATTRIB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_ATTRIB_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_ATTRIB_ARRAY").field("NumElements", &self.NumElements).field("DhcpAttribs", &self.DhcpAttribs).finish()
    }
}
impl ::core::default::Default for DHCP_BINARY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_BINARY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DHCP_BINARY_DATA {}
impl ::core::fmt::Debug for DHCP_BINARY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_BINARY_DATA").field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_BIND_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.fBoundToDHCPServer == other.fBoundToDHCPServer && self.AdapterPrimaryAddress == other.AdapterPrimaryAddress && self.AdapterSubnetAddress == other.AdapterSubnetAddress && self.IfDescription == other.IfDescription && self.IfIdSize == other.IfIdSize && self.IfId == other.IfId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_BIND_ELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_BIND_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_BIND_ELEMENT").field("Flags", &self.Flags).field("fBoundToDHCPServer", &self.fBoundToDHCPServer).field("AdapterPrimaryAddress", &self.AdapterPrimaryAddress).field("AdapterSubnetAddress", &self.AdapterSubnetAddress).field("IfDescription", &self.IfDescription).field("IfIdSize", &self.IfIdSize).field("IfId", &self.IfId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_BIND_ELEMENT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_BIND_ELEMENT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_BIND_ELEMENT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_BIND_ELEMENT_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_BOOTP_IP_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_BOOTP_IP_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.EndAddress == other.EndAddress && self.BootpAllocated == other.BootpAllocated && self.MaxBootpAllowed == other.MaxBootpAllowed
    }
}
impl ::core::cmp::Eq for DHCP_BOOTP_IP_RANGE {}
impl ::core::fmt::Debug for DHCP_BOOTP_IP_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_BOOTP_IP_RANGE").field("StartAddress", &self.StartAddress).field("EndAddress", &self.EndAddress).field("BootpAllocated", &self.BootpAllocated).field("MaxBootpAllowed", &self.MaxBootpAllowed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CALLOUT_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClassName == other.ClassName && self.ClassComment == other.ClassComment && self.ClassDataLength == other.ClassDataLength && self.IsVendor == other.IsVendor && self.Flags == other.Flags && self.ClassData == other.ClassData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLASS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLASS_INFO").field("ClassName", &self.ClassName).field("ClassComment", &self.ClassComment).field("ClassDataLength", &self.ClassDataLength).field("IsVendor", &self.IsVendor).field("Flags", &self.Flags).field("ClassData", &self.ClassData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Classes == other.Classes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLASS_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLASS_INFO_ARRAY").field("NumElements", &self.NumElements).field("Classes", &self.Classes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Classes == other.Classes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_ARRAY_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLASS_INFO_ARRAY_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLASS_INFO_ARRAY_V6").field("NumElements", &self.NumElements).field("Classes", &self.Classes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLASS_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLASS_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.ClassName == other.ClassName && self.ClassComment == other.ClassComment && self.ClassDataLength == other.ClassDataLength && self.IsVendor == other.IsVendor && self.EnterpriseNumber == other.EnterpriseNumber && self.Flags == other.Flags && self.ClassData == other.ClassData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLASS_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLASS_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLASS_INFO_V6").field("ClassName", &self.ClassName).field("ClassComment", &self.ClassComment).field("ClassDataLength", &self.ClassDataLength).field("IsVendor", &self.IsVendor).field("EnterpriseNumber", &self.EnterpriseNumber).field("Flags", &self.Flags).field("ClassData", &self.ClassData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState && self.Status == other.Status && self.ProbationEnds == other.ProbationEnds && self.QuarantineCapable == other.QuarantineCapable && self.FilterStatus == other.FilterStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_FILTER_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_FILTER_STATUS_INFO")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("FilterStatus", &self.FilterStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::core::default::Default for DHCP_CLIENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO {}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO").field("ClientIpAddress", &self.ClientIpAddress).field("SubnetMask", &self.SubnetMask).field("ClientHardwareAddress", &self.ClientHardwareAddress).field("ClientName", &self.ClientName).field("ClientComment", &self.ClientComment).field("ClientLeaseExpires", &self.ClientLeaseExpires).field("OwnerHost", &self.OwnerHost).finish()
    }
}
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY {}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V4 {}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY_V4").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V5 {}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY_V5").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_V6 {}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY_V6").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_ARRAY_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_ARRAY_VQ").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState && self.Status == other.Status && self.ProbationEnds == other.ProbationEnds && self.QuarantineCapable == other.QuarantineCapable && self.FilterStatus == other.FilterStatus && self.PolicyName == other.PolicyName && self.Properties == other.Properties
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_EX")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("FilterStatus", &self.FilterStatus)
            .field("PolicyName", &self.PolicyName)
            .field("Properties", &self.Properties)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_EX_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_EX_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_EX_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_EX_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_PB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_PB {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState && self.Status == other.Status && self.ProbationEnds == other.ProbationEnds && self.QuarantineCapable == other.QuarantineCapable && self.FilterStatus == other.FilterStatus && self.PolicyName == other.PolicyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_PB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_PB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_PB")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .field("FilterStatus", &self.FilterStatus)
            .field("PolicyName", &self.PolicyName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_PB_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_PB_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Clients == other.Clients
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_PB_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_PB_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_PB_ARRAY").field("NumElements", &self.NumElements).field("Clients", &self.Clients).finish()
    }
}
impl ::core::default::Default for DHCP_CLIENT_INFO_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V4 {}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_V4").field("ClientIpAddress", &self.ClientIpAddress).field("SubnetMask", &self.SubnetMask).field("ClientHardwareAddress", &self.ClientHardwareAddress).field("ClientName", &self.ClientName).field("ClientComment", &self.ClientComment).field("ClientLeaseExpires", &self.ClientLeaseExpires).field("OwnerHost", &self.OwnerHost).field("bClientType", &self.bClientType).finish()
    }
}
impl ::core::default::Default for DHCP_CLIENT_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V5 {}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_V5").field("ClientIpAddress", &self.ClientIpAddress).field("SubnetMask", &self.SubnetMask).field("ClientHardwareAddress", &self.ClientHardwareAddress).field("ClientName", &self.ClientName).field("ClientComment", &self.ClientComment).field("ClientLeaseExpires", &self.ClientLeaseExpires).field("OwnerHost", &self.OwnerHost).field("bClientType", &self.bClientType).field("AddressState", &self.AddressState).finish()
    }
}
impl ::core::default::Default for DHCP_CLIENT_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.ClientDUID == other.ClientDUID && self.AddressType == other.AddressType && self.IAID == other.IAID && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientValidLeaseExpires == other.ClientValidLeaseExpires && self.ClientPrefLeaseExpires == other.ClientPrefLeaseExpires && self.OwnerHost == other.OwnerHost
    }
}
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_V6 {}
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_V6").field("ClientIpAddress", &self.ClientIpAddress).field("ClientDUID", &self.ClientDUID).field("AddressType", &self.AddressType).field("IAID", &self.IAID).field("ClientName", &self.ClientName).field("ClientComment", &self.ClientComment).field("ClientValidLeaseExpires", &self.ClientValidLeaseExpires).field("ClientPrefLeaseExpires", &self.ClientPrefLeaseExpires).field("OwnerHost", &self.OwnerHost).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_CLIENT_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_CLIENT_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.ClientIpAddress == other.ClientIpAddress && self.SubnetMask == other.SubnetMask && self.ClientHardwareAddress == other.ClientHardwareAddress && self.ClientName == other.ClientName && self.ClientComment == other.ClientComment && self.ClientLeaseExpires == other.ClientLeaseExpires && self.OwnerHost == other.OwnerHost && self.bClientType == other.bClientType && self.AddressState == other.AddressState && self.Status == other.Status && self.ProbationEnds == other.ProbationEnds && self.QuarantineCapable == other.QuarantineCapable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_CLIENT_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_CLIENT_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_CLIENT_INFO_VQ")
            .field("ClientIpAddress", &self.ClientIpAddress)
            .field("SubnetMask", &self.SubnetMask)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClientName", &self.ClientName)
            .field("ClientComment", &self.ClientComment)
            .field("ClientLeaseExpires", &self.ClientLeaseExpires)
            .field("OwnerHost", &self.OwnerHost)
            .field("bClientType", &self.bClientType)
            .field("AddressState", &self.AddressState)
            .field("Status", &self.Status)
            .field("ProbationEnds", &self.ProbationEnds)
            .field("QuarantineCapable", &self.QuarantineCapable)
            .finish()
    }
}
impl ::core::default::Default for DHCP_FAILOVER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_FAILOVER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_FAILOVER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_FAILOVER_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_FAILOVER_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryServer == other.PrimaryServer && self.SecondaryServer == other.SecondaryServer && self.Mode == other.Mode && self.ServerType == other.ServerType && self.State == other.State && self.PrevState == other.PrevState && self.Mclt == other.Mclt && self.SafePeriod == other.SafePeriod && self.RelationshipName == other.RelationshipName && self.PrimaryServerName == other.PrimaryServerName && self.SecondaryServerName == other.SecondaryServerName && self.pScopes == other.pScopes && self.Percentage == other.Percentage && self.SharedSecret == other.SharedSecret
    }
}
impl ::core::cmp::Eq for DHCP_FAILOVER_RELATIONSHIP {}
impl ::core::fmt::Debug for DHCP_FAILOVER_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FAILOVER_RELATIONSHIP")
            .field("PrimaryServer", &self.PrimaryServer)
            .field("SecondaryServer", &self.SecondaryServer)
            .field("Mode", &self.Mode)
            .field("ServerType", &self.ServerType)
            .field("State", &self.State)
            .field("PrevState", &self.PrevState)
            .field("Mclt", &self.Mclt)
            .field("SafePeriod", &self.SafePeriod)
            .field("RelationshipName", &self.RelationshipName)
            .field("PrimaryServerName", &self.PrimaryServerName)
            .field("SecondaryServerName", &self.SecondaryServerName)
            .field("pScopes", &self.pScopes)
            .field("Percentage", &self.Percentage)
            .field("SharedSecret", &self.SharedSecret)
            .finish()
    }
}
impl ::core::default::Default for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.pRelationships == other.pRelationships
    }
}
impl ::core::cmp::Eq for DHCP_FAILOVER_RELATIONSHIP_ARRAY {}
impl ::core::fmt::Debug for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FAILOVER_RELATIONSHIP_ARRAY").field("NumElements", &self.NumElements).field("pRelationships", &self.pRelationships).finish()
    }
}
impl ::core::default::Default for DHCP_FAILOVER_SERVER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_FAILOVER_SERVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_FAILOVER_SERVER").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_FAILOVER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_FAILOVER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.NumAddr == other.NumAddr && self.AddrFree == other.AddrFree && self.AddrInUse == other.AddrInUse && self.PartnerAddrFree == other.PartnerAddrFree && self.ThisAddrFree == other.ThisAddrFree && self.PartnerAddrInUse == other.PartnerAddrInUse && self.ThisAddrInUse == other.ThisAddrInUse
    }
}
impl ::core::cmp::Eq for DHCP_FAILOVER_STATISTICS {}
impl ::core::fmt::Debug for DHCP_FAILOVER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FAILOVER_STATISTICS").field("NumAddr", &self.NumAddr).field("AddrFree", &self.AddrFree).field("AddrInUse", &self.AddrInUse).field("PartnerAddrFree", &self.PartnerAddrFree).field("ThisAddrFree", &self.ThisAddrFree).field("PartnerAddrInUse", &self.PartnerAddrInUse).field("ThisAddrInUse", &self.ThisAddrInUse).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_ADD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_ADD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPatt == other.AddrPatt && self.Comment == other.Comment && self.ListType == other.ListType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_ADD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_FILTER_ADD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FILTER_ADD_INFO").field("AddrPatt", &self.AddrPatt).field("Comment", &self.Comment).field("ListType", &self.ListType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_ENUM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_ENUM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.pEnumRecords == other.pEnumRecords
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_ENUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_FILTER_ENUM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FILTER_ENUM_INFO").field("NumElements", &self.NumElements).field("pEnumRecords", &self.pEnumRecords).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_GLOBAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_GLOBAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EnforceAllowList == other.EnforceAllowList && self.EnforceDenyList == other.EnforceDenyList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_GLOBAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_FILTER_GLOBAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FILTER_GLOBAL_INFO").field("EnforceAllowList", &self.EnforceAllowList).field("EnforceDenyList", &self.EnforceDenyList).finish()
    }
}
impl ::core::default::Default for DHCP_FILTER_LIST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_FILTER_LIST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_FILTER_LIST_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_FILTER_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_FILTER_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPatt == other.AddrPatt && self.Comment == other.Comment
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_FILTER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_FILTER_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_FILTER_RECORD").field("AddrPatt", &self.AddrPatt).field("Comment", &self.Comment).finish()
    }
}
impl ::core::default::Default for DHCP_FORCE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_FORCE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_FORCE_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_HOST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_HOST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress && self.NetBiosName == other.NetBiosName && self.HostName == other.HostName
    }
}
impl ::core::cmp::Eq for DHCP_HOST_INFO {}
impl ::core::fmt::Debug for DHCP_HOST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_HOST_INFO").field("IpAddress", &self.IpAddress).field("NetBiosName", &self.NetBiosName).field("HostName", &self.HostName).finish()
    }
}
impl ::core::default::Default for DHCP_HOST_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_HOST_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress && self.NetBiosName == other.NetBiosName && self.HostName == other.HostName
    }
}
impl ::core::cmp::Eq for DHCP_HOST_INFO_V6 {}
impl ::core::fmt::Debug for DHCP_HOST_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_HOST_INFO_V6").field("IpAddress", &self.IpAddress).field("NetBiosName", &self.NetBiosName).field("HostName", &self.HostName).finish()
    }
}
impl ::core::default::Default for DHCP_IPV6_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IPV6_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.HighOrderBits == other.HighOrderBits && self.LowOrderBits == other.LowOrderBits
    }
}
impl ::core::cmp::Eq for DHCP_IPV6_ADDRESS {}
impl ::core::fmt::Debug for DHCP_IPV6_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IPV6_ADDRESS").field("HighOrderBits", &self.HighOrderBits).field("LowOrderBits", &self.LowOrderBits).finish()
    }
}
impl ::core::default::Default for DHCP_IP_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_IP_ARRAY {}
impl ::core::fmt::Debug for DHCP_IP_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_IP_CLUSTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_CLUSTER {
    fn eq(&self, other: &Self) -> bool {
        self.ClusterAddress == other.ClusterAddress && self.ClusterMask == other.ClusterMask
    }
}
impl ::core::cmp::Eq for DHCP_IP_CLUSTER {}
impl ::core::fmt::Debug for DHCP_IP_CLUSTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_CLUSTER").field("ClusterAddress", &self.ClusterAddress).field("ClusterMask", &self.ClusterMask).finish()
    }
}
impl ::core::default::Default for DHCP_IP_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.EndAddress == other.EndAddress
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE {}
impl ::core::fmt::Debug for DHCP_IP_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RANGE").field("StartAddress", &self.StartAddress).field("EndAddress", &self.EndAddress).finish()
    }
}
impl ::core::default::Default for DHCP_IP_RANGE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE_ARRAY {}
impl ::core::fmt::Debug for DHCP_IP_RANGE_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RANGE_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_IP_RANGE_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_RANGE_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.EndAddress == other.EndAddress
    }
}
impl ::core::cmp::Eq for DHCP_IP_RANGE_V6 {}
impl ::core::fmt::Debug for DHCP_IP_RANGE_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RANGE_V6").field("StartAddress", &self.StartAddress).field("EndAddress", &self.EndAddress).finish()
    }
}
impl ::core::default::Default for DHCP_IP_RESERVATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedForClient == other.ReservedForClient
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION {}
impl ::core::fmt::Debug for DHCP_IP_RESERVATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RESERVATION").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedForClient", &self.ReservedForClient).finish()
    }
}
impl ::core::default::Default for DHCP_IP_RESERVATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedForClient == other.ReservedForClient && self.ReservedClientName == other.ReservedClientName && self.ReservedClientDesc == other.ReservedClientDesc && self.bAllowedClientTypes == other.bAllowedClientTypes && self.fOptionsPresent == other.fOptionsPresent
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_INFO {}
impl ::core::fmt::Debug for DHCP_IP_RESERVATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RESERVATION_INFO").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedForClient", &self.ReservedForClient).field("ReservedClientName", &self.ReservedClientName).field("ReservedClientDesc", &self.ReservedClientDesc).field("bAllowedClientTypes", &self.bAllowedClientTypes).field("fOptionsPresent", &self.fOptionsPresent).finish()
    }
}
impl ::core::default::Default for DHCP_IP_RESERVATION_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedForClient == other.ReservedForClient && self.bAllowedClientTypes == other.bAllowedClientTypes
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_V4 {}
impl ::core::fmt::Debug for DHCP_IP_RESERVATION_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RESERVATION_V4").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedForClient", &self.ReservedForClient).field("bAllowedClientTypes", &self.bAllowedClientTypes).finish()
    }
}
impl ::core::default::Default for DHCP_IP_RESERVATION_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_IP_RESERVATION_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedForClient == other.ReservedForClient && self.InterfaceId == other.InterfaceId
    }
}
impl ::core::cmp::Eq for DHCP_IP_RESERVATION_V6 {}
impl ::core::fmt::Debug for DHCP_IP_RESERVATION_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_IP_RESERVATION_V6").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedForClient", &self.ReservedForClient).field("InterfaceId", &self.InterfaceId).finish()
    }
}
impl ::core::default::Default for DHCP_MIB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Discovers == other.Discovers && self.Offers == other.Offers && self.Requests == other.Requests && self.Acks == other.Acks && self.Naks == other.Naks && self.Declines == other.Declines && self.Releases == other.Releases && self.ServerStartTime == other.ServerStartTime && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO {}
impl ::core::fmt::Debug for DHCP_MIB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_MIB_INFO").field("Discovers", &self.Discovers).field("Offers", &self.Offers).field("Requests", &self.Requests).field("Acks", &self.Acks).field("Naks", &self.Naks).field("Declines", &self.Declines).field("Releases", &self.Releases).field("ServerStartTime", &self.ServerStartTime).field("Scopes", &self.Scopes).field("ScopeInfo", &self.ScopeInfo).finish()
    }
}
impl ::core::default::Default for DHCP_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.Discovers == other.Discovers && self.Offers == other.Offers && self.Requests == other.Requests && self.Acks == other.Acks && self.Naks == other.Naks && self.Declines == other.Declines && self.Releases == other.Releases && self.ServerStartTime == other.ServerStartTime && self.QtnNumLeases == other.QtnNumLeases && self.QtnPctQtnLeases == other.QtnPctQtnLeases && self.QtnProbationLeases == other.QtnProbationLeases && self.QtnNonQtnLeases == other.QtnNonQtnLeases && self.QtnExemptLeases == other.QtnExemptLeases && self.QtnCapableClients == other.QtnCapableClients && self.QtnIASErrors == other.QtnIASErrors && self.DelayedOffers == other.DelayedOffers && self.ScopesWithDelayedOffers == other.ScopesWithDelayedOffers && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_V5 {}
impl ::core::fmt::Debug for DHCP_MIB_INFO_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_MIB_INFO_V5")
            .field("Discovers", &self.Discovers)
            .field("Offers", &self.Offers)
            .field("Requests", &self.Requests)
            .field("Acks", &self.Acks)
            .field("Naks", &self.Naks)
            .field("Declines", &self.Declines)
            .field("Releases", &self.Releases)
            .field("ServerStartTime", &self.ServerStartTime)
            .field("QtnNumLeases", &self.QtnNumLeases)
            .field("QtnPctQtnLeases", &self.QtnPctQtnLeases)
            .field("QtnProbationLeases", &self.QtnProbationLeases)
            .field("QtnNonQtnLeases", &self.QtnNonQtnLeases)
            .field("QtnExemptLeases", &self.QtnExemptLeases)
            .field("QtnCapableClients", &self.QtnCapableClients)
            .field("QtnIASErrors", &self.QtnIASErrors)
            .field("DelayedOffers", &self.DelayedOffers)
            .field("ScopesWithDelayedOffers", &self.ScopesWithDelayedOffers)
            .field("Scopes", &self.Scopes)
            .field("ScopeInfo", &self.ScopeInfo)
            .finish()
    }
}
impl ::core::default::Default for DHCP_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.Solicits == other.Solicits && self.Advertises == other.Advertises && self.Requests == other.Requests && self.Renews == other.Renews && self.Rebinds == other.Rebinds && self.Replies == other.Replies && self.Confirms == other.Confirms && self.Declines == other.Declines && self.Releases == other.Releases && self.Informs == other.Informs && self.ServerStartTime == other.ServerStartTime && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_V6 {}
impl ::core::fmt::Debug for DHCP_MIB_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_MIB_INFO_V6")
            .field("Solicits", &self.Solicits)
            .field("Advertises", &self.Advertises)
            .field("Requests", &self.Requests)
            .field("Renews", &self.Renews)
            .field("Rebinds", &self.Rebinds)
            .field("Replies", &self.Replies)
            .field("Confirms", &self.Confirms)
            .field("Declines", &self.Declines)
            .field("Releases", &self.Releases)
            .field("Informs", &self.Informs)
            .field("ServerStartTime", &self.ServerStartTime)
            .field("Scopes", &self.Scopes)
            .field("ScopeInfo", &self.ScopeInfo)
            .finish()
    }
}
impl ::core::default::Default for DHCP_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_MIB_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.Discovers == other.Discovers && self.Offers == other.Offers && self.Requests == other.Requests && self.Acks == other.Acks && self.Naks == other.Naks && self.Declines == other.Declines && self.Releases == other.Releases && self.ServerStartTime == other.ServerStartTime && self.QtnNumLeases == other.QtnNumLeases && self.QtnPctQtnLeases == other.QtnPctQtnLeases && self.QtnProbationLeases == other.QtnProbationLeases && self.QtnNonQtnLeases == other.QtnNonQtnLeases && self.QtnExemptLeases == other.QtnExemptLeases && self.QtnCapableClients == other.QtnCapableClients && self.QtnIASErrors == other.QtnIASErrors && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo
    }
}
impl ::core::cmp::Eq for DHCP_MIB_INFO_VQ {}
impl ::core::fmt::Debug for DHCP_MIB_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_MIB_INFO_VQ")
            .field("Discovers", &self.Discovers)
            .field("Offers", &self.Offers)
            .field("Requests", &self.Requests)
            .field("Acks", &self.Acks)
            .field("Naks", &self.Naks)
            .field("Declines", &self.Declines)
            .field("Releases", &self.Releases)
            .field("ServerStartTime", &self.ServerStartTime)
            .field("QtnNumLeases", &self.QtnNumLeases)
            .field("QtnPctQtnLeases", &self.QtnPctQtnLeases)
            .field("QtnProbationLeases", &self.QtnProbationLeases)
            .field("QtnNonQtnLeases", &self.QtnNonQtnLeases)
            .field("QtnExemptLeases", &self.QtnExemptLeases)
            .field("QtnCapableClients", &self.QtnCapableClients)
            .field("QtnIASErrors", &self.QtnIASErrors)
            .field("Scopes", &self.Scopes)
            .field("ScopeInfo", &self.ScopeInfo)
            .finish()
    }
}
impl ::core::default::Default for DHCP_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_OPTION {
    fn eq(&self, other: &Self) -> bool {
        self.OptionID == other.OptionID && self.OptionName == other.OptionName && self.OptionComment == other.OptionComment && self.DefaultValue == other.DefaultValue && self.OptionType == other.OptionType
    }
}
impl ::core::cmp::Eq for DHCP_OPTION {}
impl ::core::fmt::Debug for DHCP_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION").field("OptionID", &self.OptionID).field("OptionName", &self.OptionName).field("OptionComment", &self.OptionComment).field("DefaultValue", &self.DefaultValue).field("OptionType", &self.OptionType).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_OPTION_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_ARRAY {}
impl ::core::fmt::Debug for DHCP_OPTION_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_ARRAY").field("NumElements", &self.NumElements).field("Options", &self.Options).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_OPTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_DATA {}
impl ::core::fmt::Debug for DHCP_OPTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_DATA").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_DATA_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_OPTION_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_OPTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumOptions == other.NumOptions && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_LIST {}
impl ::core::fmt::Debug for DHCP_OPTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_LIST").field("NumOptions", &self.NumOptions).field("Options", &self.Options).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_INFO6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_SCOPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_SCOPE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_SCOPE_TYPE6 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_SCOPE_TYPE6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_SCOPE_TYPE6").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_OPTION_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.OptionID == other.OptionID && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_VALUE {}
impl ::core::fmt::Debug for DHCP_OPTION_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_VALUE").field("OptionID", &self.OptionID).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for DHCP_OPTION_VALUE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_OPTION_VALUE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for DHCP_OPTION_VALUE_ARRAY {}
impl ::core::fmt::Debug for DHCP_OPTION_VALUE_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_OPTION_VALUE_ARRAY").field("NumElements", &self.NumElements).field("Values", &self.Values).finish()
    }
}
impl ::core::default::Default for DHCP_PERF_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_PERF_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPacketsReceived == other.dwNumPacketsReceived
            && self.dwNumPacketsDuplicate == other.dwNumPacketsDuplicate
            && self.dwNumPacketsExpired == other.dwNumPacketsExpired
            && self.dwNumMilliSecondsProcessed == other.dwNumMilliSecondsProcessed
            && self.dwNumPacketsInActiveQueue == other.dwNumPacketsInActiveQueue
            && self.dwNumPacketsInPingQueue == other.dwNumPacketsInPingQueue
            && self.dwNumDiscoversReceived == other.dwNumDiscoversReceived
            && self.dwNumOffersSent == other.dwNumOffersSent
            && self.dwNumRequestsReceived == other.dwNumRequestsReceived
            && self.dwNumInformsReceived == other.dwNumInformsReceived
            && self.dwNumAcksSent == other.dwNumAcksSent
            && self.dwNumNacksSent == other.dwNumNacksSent
            && self.dwNumDeclinesReceived == other.dwNumDeclinesReceived
            && self.dwNumReleasesReceived == other.dwNumReleasesReceived
            && self.dwNumDelayedOfferInQueue == other.dwNumDelayedOfferInQueue
            && self.dwNumPacketsProcessed == other.dwNumPacketsProcessed
            && self.dwNumPacketsInQuarWaitingQueue == other.dwNumPacketsInQuarWaitingQueue
            && self.dwNumPacketsInQuarReadyQueue == other.dwNumPacketsInQuarReadyQueue
            && self.dwNumPacketsInQuarDecisionQueue == other.dwNumPacketsInQuarDecisionQueue
    }
}
impl ::core::cmp::Eq for DHCP_PERF_STATS {}
impl ::core::fmt::Debug for DHCP_PERF_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_PERF_STATS")
            .field("dwNumPacketsReceived", &self.dwNumPacketsReceived)
            .field("dwNumPacketsDuplicate", &self.dwNumPacketsDuplicate)
            .field("dwNumPacketsExpired", &self.dwNumPacketsExpired)
            .field("dwNumMilliSecondsProcessed", &self.dwNumMilliSecondsProcessed)
            .field("dwNumPacketsInActiveQueue", &self.dwNumPacketsInActiveQueue)
            .field("dwNumPacketsInPingQueue", &self.dwNumPacketsInPingQueue)
            .field("dwNumDiscoversReceived", &self.dwNumDiscoversReceived)
            .field("dwNumOffersSent", &self.dwNumOffersSent)
            .field("dwNumRequestsReceived", &self.dwNumRequestsReceived)
            .field("dwNumInformsReceived", &self.dwNumInformsReceived)
            .field("dwNumAcksSent", &self.dwNumAcksSent)
            .field("dwNumNacksSent", &self.dwNumNacksSent)
            .field("dwNumDeclinesReceived", &self.dwNumDeclinesReceived)
            .field("dwNumReleasesReceived", &self.dwNumReleasesReceived)
            .field("dwNumDelayedOfferInQueue", &self.dwNumDelayedOfferInQueue)
            .field("dwNumPacketsProcessed", &self.dwNumPacketsProcessed)
            .field("dwNumPacketsInQuarWaitingQueue", &self.dwNumPacketsInQuarWaitingQueue)
            .field("dwNumPacketsInQuarReadyQueue", &self.dwNumPacketsInQuarReadyQueue)
            .field("dwNumPacketsInQuarDecisionQueue", &self.dwNumPacketsInQuarDecisionQueue)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.PolicyName == other.PolicyName && self.IsGlobalPolicy == other.IsGlobalPolicy && self.Subnet == other.Subnet && self.ProcessingOrder == other.ProcessingOrder && self.Conditions == other.Conditions && self.Expressions == other.Expressions && self.Ranges == other.Ranges && self.Description == other.Description && self.Enabled == other.Enabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POLICY").field("PolicyName", &self.PolicyName).field("IsGlobalPolicy", &self.IsGlobalPolicy).field("Subnet", &self.Subnet).field("ProcessingOrder", &self.ProcessingOrder).field("Conditions", &self.Conditions).field("Expressions", &self.Expressions).field("Ranges", &self.Ranges).field("Description", &self.Description).field("Enabled", &self.Enabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_POLICY_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POLICY_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PolicyName == other.PolicyName && self.IsGlobalPolicy == other.IsGlobalPolicy && self.Subnet == other.Subnet && self.ProcessingOrder == other.ProcessingOrder && self.Conditions == other.Conditions && self.Expressions == other.Expressions && self.Ranges == other.Ranges && self.Description == other.Description && self.Enabled == other.Enabled && self.Properties == other.Properties
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_POLICY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POLICY_EX").field("PolicyName", &self.PolicyName).field("IsGlobalPolicy", &self.IsGlobalPolicy).field("Subnet", &self.Subnet).field("ProcessingOrder", &self.ProcessingOrder).field("Conditions", &self.Conditions).field("Expressions", &self.Expressions).field("Ranges", &self.Ranges).field("Description", &self.Description).field("Enabled", &self.Enabled).field("Properties", &self.Properties).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_POLICY_EX_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_POLICY_EX_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_POLICY_EX_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_POLICY_EX_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POLICY_EX_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_POLICY_FIELDS_TO_UPDATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_POLICY_FIELDS_TO_UPDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_POLICY_FIELDS_TO_UPDATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_POL_ATTR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_POL_ATTR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_POL_ATTR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_POL_COMPARATOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_POL_COMPARATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_POL_COMPARATOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_POL_COND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_POL_COND {
    fn eq(&self, other: &Self) -> bool {
        self.ParentExpr == other.ParentExpr && self.Type == other.Type && self.OptionID == other.OptionID && self.SubOptionID == other.SubOptionID && self.VendorName == other.VendorName && self.Operator == other.Operator && self.Value == other.Value && self.ValueLength == other.ValueLength
    }
}
impl ::core::cmp::Eq for DHCP_POL_COND {}
impl ::core::fmt::Debug for DHCP_POL_COND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POL_COND").field("ParentExpr", &self.ParentExpr).field("Type", &self.Type).field("OptionID", &self.OptionID).field("SubOptionID", &self.SubOptionID).field("VendorName", &self.VendorName).field("Operator", &self.Operator).field("Value", &self.Value).field("ValueLength", &self.ValueLength).finish()
    }
}
impl ::core::default::Default for DHCP_POL_COND_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_POL_COND_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_POL_COND_ARRAY {}
impl ::core::fmt::Debug for DHCP_POL_COND_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POL_COND_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_POL_EXPR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_POL_EXPR {
    fn eq(&self, other: &Self) -> bool {
        self.ParentExpr == other.ParentExpr && self.Operator == other.Operator
    }
}
impl ::core::cmp::Eq for DHCP_POL_EXPR {}
impl ::core::fmt::Debug for DHCP_POL_EXPR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POL_EXPR").field("ParentExpr", &self.ParentExpr).field("Operator", &self.Operator).finish()
    }
}
impl ::core::default::Default for DHCP_POL_EXPR_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_POL_EXPR_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_POL_EXPR_ARRAY {}
impl ::core::fmt::Debug for DHCP_POL_EXPR_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_POL_EXPR_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_POL_LOGIC_OPER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_POL_LOGIC_OPER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_POL_LOGIC_OPER").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_PROPERTY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_PROPERTY_ARRAY {}
impl ::core::fmt::Debug for DHCP_PROPERTY_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_PROPERTY_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_PROPERTY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_RESERVATION_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_RESERVATION_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_RESERVATION_INFO_ARRAY {}
impl ::core::fmt::Debug for DHCP_RESERVATION_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_RESERVATION_INFO_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_RESERVED_SCOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_RESERVED_SCOPE {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedIpSubnetAddress == other.ReservedIpSubnetAddress
    }
}
impl ::core::cmp::Eq for DHCP_RESERVED_SCOPE {}
impl ::core::fmt::Debug for DHCP_RESERVED_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_RESERVED_SCOPE").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedIpSubnetAddress", &self.ReservedIpSubnetAddress).finish()
    }
}
impl ::core::default::Default for DHCP_RESERVED_SCOPE6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_RESERVED_SCOPE6 {
    fn eq(&self, other: &Self) -> bool {
        self.ReservedIpAddress == other.ReservedIpAddress && self.ReservedIpSubnetAddress == other.ReservedIpSubnetAddress
    }
}
impl ::core::cmp::Eq for DHCP_RESERVED_SCOPE6 {}
impl ::core::fmt::Debug for DHCP_RESERVED_SCOPE6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_RESERVED_SCOPE6").field("ReservedIpAddress", &self.ReservedIpAddress).field("ReservedIpSubnetAddress", &self.ReservedIpSubnetAddress).finish()
    }
}
impl ::core::default::Default for DHCP_SCAN_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_SCAN_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SCAN_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_SCAN_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SCAN_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress && self.ScanFlag == other.ScanFlag
    }
}
impl ::core::cmp::Eq for DHCP_SCAN_ITEM {}
impl ::core::fmt::Debug for DHCP_SCAN_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SCAN_ITEM").field("IpAddress", &self.IpAddress).field("ScanFlag", &self.ScanFlag).finish()
    }
}
impl ::core::default::Default for DHCP_SCAN_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SCAN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumScanItems == other.NumScanItems && self.ScanItems == other.ScanItems
    }
}
impl ::core::cmp::Eq for DHCP_SCAN_LIST {}
impl ::core::fmt::Debug for DHCP_SCAN_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SCAN_LIST").field("NumScanItems", &self.NumScanItems).field("ScanItems", &self.ScanItems).finish()
    }
}
impl ::core::default::Default for DHCP_SEARCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_SEARCH_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_SEARCH_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SEARCH_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_SEARCH_INFO_TYPE_V6 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_SEARCH_INFO_TYPE_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SEARCH_INFO_TYPE_V6").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_SEARCH_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.APIProtocolSupport == other.APIProtocolSupport && self.DatabaseName == other.DatabaseName && self.DatabasePath == other.DatabasePath && self.BackupPath == other.BackupPath && self.BackupInterval == other.BackupInterval && self.DatabaseLoggingFlag == other.DatabaseLoggingFlag && self.RestoreFlag == other.RestoreFlag && self.DatabaseCleanupInterval == other.DatabaseCleanupInterval && self.DebugFlag == other.DebugFlag
    }
}
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO {}
impl ::core::fmt::Debug for DHCP_SERVER_CONFIG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_CONFIG_INFO").field("APIProtocolSupport", &self.APIProtocolSupport).field("DatabaseName", &self.DatabaseName).field("DatabasePath", &self.DatabasePath).field("BackupPath", &self.BackupPath).field("BackupInterval", &self.BackupInterval).field("DatabaseLoggingFlag", &self.DatabaseLoggingFlag).field("RestoreFlag", &self.RestoreFlag).field("DatabaseCleanupInterval", &self.DatabaseCleanupInterval).field("DebugFlag", &self.DebugFlag).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.APIProtocolSupport == other.APIProtocolSupport && self.DatabaseName == other.DatabaseName && self.DatabasePath == other.DatabasePath && self.BackupPath == other.BackupPath && self.BackupInterval == other.BackupInterval && self.DatabaseLoggingFlag == other.DatabaseLoggingFlag && self.RestoreFlag == other.RestoreFlag && self.DatabaseCleanupInterval == other.DatabaseCleanupInterval && self.DebugFlag == other.DebugFlag && self.dwPingRetries == other.dwPingRetries && self.cbBootTableString == other.cbBootTableString && self.wszBootTableString == other.wszBootTableString && self.fAuditLog == other.fAuditLog
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_SERVER_CONFIG_INFO_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_CONFIG_INFO_V4")
            .field("APIProtocolSupport", &self.APIProtocolSupport)
            .field("DatabaseName", &self.DatabaseName)
            .field("DatabasePath", &self.DatabasePath)
            .field("BackupPath", &self.BackupPath)
            .field("BackupInterval", &self.BackupInterval)
            .field("DatabaseLoggingFlag", &self.DatabaseLoggingFlag)
            .field("RestoreFlag", &self.RestoreFlag)
            .field("DatabaseCleanupInterval", &self.DatabaseCleanupInterval)
            .field("DebugFlag", &self.DebugFlag)
            .field("dwPingRetries", &self.dwPingRetries)
            .field("cbBootTableString", &self.cbBootTableString)
            .field("wszBootTableString", &self.wszBootTableString)
            .field("fAuditLog", &self.fAuditLog)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.UnicastFlag == other.UnicastFlag && self.RapidCommitFlag == other.RapidCommitFlag && self.PreferredLifetime == other.PreferredLifetime && self.ValidLifetime == other.ValidLifetime && self.T1 == other.T1 && self.T2 == other.T2 && self.PreferredLifetimeIATA == other.PreferredLifetimeIATA && self.ValidLifetimeIATA == other.ValidLifetimeIATA && self.fAuditLog == other.fAuditLog
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_V6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_SERVER_CONFIG_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_CONFIG_INFO_V6").field("UnicastFlag", &self.UnicastFlag).field("RapidCommitFlag", &self.RapidCommitFlag).field("PreferredLifetime", &self.PreferredLifetime).field("ValidLifetime", &self.ValidLifetime).field("T1", &self.T1).field("T2", &self.T2).field("PreferredLifetimeIATA", &self.PreferredLifetimeIATA).field("ValidLifetimeIATA", &self.ValidLifetimeIATA).field("fAuditLog", &self.fAuditLog).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_CONFIG_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_CONFIG_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.APIProtocolSupport == other.APIProtocolSupport && self.DatabaseName == other.DatabaseName && self.DatabasePath == other.DatabasePath && self.BackupPath == other.BackupPath && self.BackupInterval == other.BackupInterval && self.DatabaseLoggingFlag == other.DatabaseLoggingFlag && self.RestoreFlag == other.RestoreFlag && self.DatabaseCleanupInterval == other.DatabaseCleanupInterval && self.DebugFlag == other.DebugFlag && self.dwPingRetries == other.dwPingRetries && self.cbBootTableString == other.cbBootTableString && self.wszBootTableString == other.wszBootTableString && self.fAuditLog == other.fAuditLog && self.QuarantineOn == other.QuarantineOn && self.QuarDefFail == other.QuarDefFail && self.QuarRuntimeStatus == other.QuarRuntimeStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_CONFIG_INFO_VQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_SERVER_CONFIG_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_CONFIG_INFO_VQ")
            .field("APIProtocolSupport", &self.APIProtocolSupport)
            .field("DatabaseName", &self.DatabaseName)
            .field("DatabasePath", &self.DatabasePath)
            .field("BackupPath", &self.BackupPath)
            .field("BackupInterval", &self.BackupInterval)
            .field("DatabaseLoggingFlag", &self.DatabaseLoggingFlag)
            .field("RestoreFlag", &self.RestoreFlag)
            .field("DatabaseCleanupInterval", &self.DatabaseCleanupInterval)
            .field("DebugFlag", &self.DebugFlag)
            .field("dwPingRetries", &self.dwPingRetries)
            .field("cbBootTableString", &self.cbBootTableString)
            .field("wszBootTableString", &self.wszBootTableString)
            .field("fAuditLog", &self.fAuditLog)
            .field("QuarantineOn", &self.QuarantineOn)
            .field("QuarDefFail", &self.QuarDefFail)
            .field("QuarRuntimeStatus", &self.QuarRuntimeStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DHCP_SERVER_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DHCP_SERVER_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.MessageType == other.MessageType
            && self.SubnetMask == other.SubnetMask
            && self.RequestedAddress == other.RequestedAddress
            && self.RequestLeaseTime == other.RequestLeaseTime
            && self.OverlayFields == other.OverlayFields
            && self.RouterAddress == other.RouterAddress
            && self.Server == other.Server
            && self.ParameterRequestList == other.ParameterRequestList
            && self.ParameterRequestListLength == other.ParameterRequestListLength
            && self.MachineName == other.MachineName
            && self.MachineNameLength == other.MachineNameLength
            && self.ClientHardwareAddressType == other.ClientHardwareAddressType
            && self.ClientHardwareAddressLength == other.ClientHardwareAddressLength
            && self.ClientHardwareAddress == other.ClientHardwareAddress
            && self.ClassIdentifier == other.ClassIdentifier
            && self.ClassIdentifierLength == other.ClassIdentifierLength
            && self.VendorClass == other.VendorClass
            && self.VendorClassLength == other.VendorClassLength
            && self.DNSFlags == other.DNSFlags
            && self.DNSNameLength == other.DNSNameLength
            && self.DNSName == other.DNSName
            && self.DSDomainNameRequested == other.DSDomainNameRequested
            && self.DSDomainName == other.DSDomainName
            && self.DSDomainNameLen == other.DSDomainNameLen
            && self.ScopeId == other.ScopeId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DHCP_SERVER_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DHCP_SERVER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_OPTIONS")
            .field("MessageType", &self.MessageType)
            .field("SubnetMask", &self.SubnetMask)
            .field("RequestedAddress", &self.RequestedAddress)
            .field("RequestLeaseTime", &self.RequestLeaseTime)
            .field("OverlayFields", &self.OverlayFields)
            .field("RouterAddress", &self.RouterAddress)
            .field("Server", &self.Server)
            .field("ParameterRequestList", &self.ParameterRequestList)
            .field("ParameterRequestListLength", &self.ParameterRequestListLength)
            .field("MachineName", &self.MachineName)
            .field("MachineNameLength", &self.MachineNameLength)
            .field("ClientHardwareAddressType", &self.ClientHardwareAddressType)
            .field("ClientHardwareAddressLength", &self.ClientHardwareAddressLength)
            .field("ClientHardwareAddress", &self.ClientHardwareAddress)
            .field("ClassIdentifier", &self.ClassIdentifier)
            .field("ClassIdentifierLength", &self.ClassIdentifierLength)
            .field("VendorClass", &self.VendorClass)
            .field("VendorClassLength", &self.VendorClassLength)
            .field("DNSFlags", &self.DNSFlags)
            .field("DNSNameLength", &self.DNSNameLength)
            .field("DNSName", &self.DNSName)
            .field("DSDomainNameRequested", &self.DSDomainNameRequested)
            .field("DSDomainName", &self.DSDomainName)
            .field("DSDomainNameLen", &self.DSDomainNameLen)
            .field("ScopeId", &self.ScopeId)
            .finish()
    }
}
impl ::core::default::Default for DHCP_SERVER_SPECIFIC_STRINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SERVER_SPECIFIC_STRINGS {
    fn eq(&self, other: &Self) -> bool {
        self.DefaultVendorClassName == other.DefaultVendorClassName && self.DefaultUserClassName == other.DefaultUserClassName
    }
}
impl ::core::cmp::Eq for DHCP_SERVER_SPECIFIC_STRINGS {}
impl ::core::fmt::Debug for DHCP_SERVER_SPECIFIC_STRINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SERVER_SPECIFIC_STRINGS").field("DefaultVendorClassName", &self.DefaultVendorClassName).field("DefaultUserClassName", &self.DefaultUserClassName).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY {}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_ELEMENT_INFO_ARRAY").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.NumElements == other.NumElements && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6").field("NumElements", &self.NumElements).field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_ELEMENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_ELEMENT_TYPE_V6 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_ELEMENT_TYPE_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_ELEMENT_TYPE_V6").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.SubnetMask == other.SubnetMask && self.SubnetName == other.SubnetName && self.SubnetComment == other.SubnetComment && self.PrimaryHost == other.PrimaryHost && self.SubnetState == other.SubnetState
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_INFO {}
impl ::core::fmt::Debug for DHCP_SUBNET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_INFO").field("SubnetAddress", &self.SubnetAddress).field("SubnetMask", &self.SubnetMask).field("SubnetName", &self.SubnetName).field("SubnetComment", &self.SubnetComment).field("PrimaryHost", &self.PrimaryHost).field("SubnetState", &self.SubnetState).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.Prefix == other.Prefix && self.Preference == other.Preference && self.SubnetName == other.SubnetName && self.SubnetComment == other.SubnetComment && self.State == other.State && self.ScopeId == other.ScopeId
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_INFO_V6 {}
impl ::core::fmt::Debug for DHCP_SUBNET_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_INFO_V6").field("SubnetAddress", &self.SubnetAddress).field("Prefix", &self.Prefix).field("Preference", &self.Preference).field("SubnetName", &self.SubnetName).field("SubnetComment", &self.SubnetComment).field("State", &self.State).field("ScopeId", &self.ScopeId).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUBNET_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.SubnetMask == other.SubnetMask && self.SubnetName == other.SubnetName && self.SubnetComment == other.SubnetComment && self.PrimaryHost == other.PrimaryHost && self.SubnetState == other.SubnetState && self.QuarantineOn == other.QuarantineOn && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3 && self.Reserved4 == other.Reserved4
    }
}
impl ::core::cmp::Eq for DHCP_SUBNET_INFO_VQ {}
impl ::core::fmt::Debug for DHCP_SUBNET_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUBNET_INFO_VQ").field("SubnetAddress", &self.SubnetAddress).field("SubnetMask", &self.SubnetMask).field("SubnetName", &self.SubnetName).field("SubnetComment", &self.SubnetComment).field("PrimaryHost", &self.PrimaryHost).field("SubnetState", &self.SubnetState).field("QuarantineOn", &self.QuarantineOn).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).finish()
    }
}
impl ::core::default::Default for DHCP_SUBNET_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DHCP_SUBNET_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DHCP_SUPER_SCOPE_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUPER_SCOPE_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pEntries == other.pEntries
    }
}
impl ::core::cmp::Eq for DHCP_SUPER_SCOPE_TABLE {}
impl ::core::fmt::Debug for DHCP_SUPER_SCOPE_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUPER_SCOPE_TABLE").field("cEntries", &self.cEntries).field("pEntries", &self.pEntries).finish()
    }
}
impl ::core::default::Default for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.SubnetAddress == other.SubnetAddress && self.SuperScopeNumber == other.SuperScopeNumber && self.NextInSuperScope == other.NextInSuperScope && self.SuperScopeName == other.SuperScopeName
    }
}
impl ::core::cmp::Eq for DHCP_SUPER_SCOPE_TABLE_ENTRY {}
impl ::core::fmt::Debug for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DHCP_SUPER_SCOPE_TABLE_ENTRY").field("SubnetAddress", &self.SubnetAddress).field("SuperScopeNumber", &self.SuperScopeNumber).field("NextInSuperScope", &self.NextInSuperScope).field("SuperScopeName", &self.SuperScopeName).finish()
    }
}
impl ::core::default::Default for DWORD_DWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWORD_DWORD {
    fn eq(&self, other: &Self) -> bool {
        self.DWord1 == other.DWord1 && self.DWord2 == other.DWord2
    }
}
impl ::core::cmp::Eq for DWORD_DWORD {}
impl ::core::fmt::Debug for DWORD_DWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWORD_DWORD").field("DWord1", &self.DWord1).field("DWord2", &self.DWord2).finish()
    }
}
impl ::core::default::Default for FSM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FSM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FSM_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for QuarantineStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QuarantineStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuarantineStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCOPE_MIB_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Subnet == other.Subnet && self.NumAddressesInuse == other.NumAddressesInuse && self.NumAddressesFree == other.NumAddressesFree && self.NumPendingOffers == other.NumPendingOffers
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO {}
impl ::core::fmt::Debug for SCOPE_MIB_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_MIB_INFO").field("Subnet", &self.Subnet).field("NumAddressesInuse", &self.NumAddressesInuse).field("NumAddressesFree", &self.NumAddressesFree).field("NumPendingOffers", &self.NumPendingOffers).finish()
    }
}
impl ::core::default::Default for SCOPE_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.Subnet == other.Subnet && self.NumAddressesInuse == other.NumAddressesInuse && self.NumAddressesFree == other.NumAddressesFree && self.NumPendingOffers == other.NumPendingOffers
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_V5 {}
impl ::core::fmt::Debug for SCOPE_MIB_INFO_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_MIB_INFO_V5").field("Subnet", &self.Subnet).field("NumAddressesInuse", &self.NumAddressesInuse).field("NumAddressesFree", &self.NumAddressesFree).field("NumPendingOffers", &self.NumPendingOffers).finish()
    }
}
impl ::core::default::Default for SCOPE_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.Subnet == other.Subnet && self.NumAddressesInuse == other.NumAddressesInuse && self.NumAddressesFree == other.NumAddressesFree && self.NumPendingAdvertises == other.NumPendingAdvertises
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_V6 {}
impl ::core::fmt::Debug for SCOPE_MIB_INFO_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_MIB_INFO_V6").field("Subnet", &self.Subnet).field("NumAddressesInuse", &self.NumAddressesInuse).field("NumAddressesFree", &self.NumAddressesFree).field("NumPendingAdvertises", &self.NumPendingAdvertises).finish()
    }
}
impl ::core::default::Default for SCOPE_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCOPE_MIB_INFO_VQ {
    fn eq(&self, other: &Self) -> bool {
        self.Subnet == other.Subnet && self.NumAddressesInuse == other.NumAddressesInuse && self.NumAddressesFree == other.NumAddressesFree && self.NumPendingOffers == other.NumPendingOffers && self.QtnNumLeases == other.QtnNumLeases && self.QtnPctQtnLeases == other.QtnPctQtnLeases && self.QtnProbationLeases == other.QtnProbationLeases && self.QtnNonQtnLeases == other.QtnNonQtnLeases && self.QtnExemptLeases == other.QtnExemptLeases && self.QtnCapableClients == other.QtnCapableClients
    }
}
impl ::core::cmp::Eq for SCOPE_MIB_INFO_VQ {}
impl ::core::fmt::Debug for SCOPE_MIB_INFO_VQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_MIB_INFO_VQ")
            .field("Subnet", &self.Subnet)
            .field("NumAddressesInuse", &self.NumAddressesInuse)
            .field("NumAddressesFree", &self.NumAddressesFree)
            .field("NumPendingOffers", &self.NumPendingOffers)
            .field("QtnNumLeases", &self.QtnNumLeases)
            .field("QtnPctQtnLeases", &self.QtnPctQtnLeases)
            .field("QtnProbationLeases", &self.QtnProbationLeases)
            .field("QtnNonQtnLeases", &self.QtnNonQtnLeases)
            .field("QtnExemptLeases", &self.QtnExemptLeases)
            .field("QtnCapableClients", &self.QtnCapableClients)
            .finish()
    }
}
impl ::core::default::Default for StatusCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusCode").field(&self.0).finish()
    }
}
