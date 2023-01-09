impl ::core::default::Default for ARP_SEND_REPLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ARP_SEND_REPLY {
    fn eq(&self, other: &Self) -> bool {
        self.DestAddress == other.DestAddress && self.SrcAddress == other.SrcAddress
    }
}
impl ::core::cmp::Eq for ARP_SEND_REPLY {}
impl ::core::fmt::Debug for ARP_SEND_REPLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ARP_SEND_REPLY").field("DestAddress", &self.DestAddress).field("SrcAddress", &self.SrcAddress).finish()
    }
}
impl ::core::default::Default for DNS_DOH_SERVER_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_DOH_SERVER_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.Template == other.Template && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for DNS_DOH_SERVER_SETTINGS {}
impl ::core::fmt::Debug for DNS_DOH_SERVER_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_DOH_SERVER_SETTINGS").field("Template", &self.Template).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for DNS_INTERFACE_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_INTERFACE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Domain == other.Domain && self.NameServer == other.NameServer && self.SearchList == other.SearchList && self.RegistrationEnabled == other.RegistrationEnabled && self.RegisterAdapterName == other.RegisterAdapterName && self.EnableLLMNR == other.EnableLLMNR && self.QueryAdapterName == other.QueryAdapterName && self.ProfileNameServer == other.ProfileNameServer
    }
}
impl ::core::cmp::Eq for DNS_INTERFACE_SETTINGS {}
impl ::core::fmt::Debug for DNS_INTERFACE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_INTERFACE_SETTINGS").field("Version", &self.Version).field("Flags", &self.Flags).field("Domain", &self.Domain).field("NameServer", &self.NameServer).field("SearchList", &self.SearchList).field("RegistrationEnabled", &self.RegistrationEnabled).field("RegisterAdapterName", &self.RegisterAdapterName).field("EnableLLMNR", &self.EnableLLMNR).field("QueryAdapterName", &self.QueryAdapterName).field("ProfileNameServer", &self.ProfileNameServer).finish()
    }
}
impl ::core::default::Default for DNS_INTERFACE_SETTINGS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_INTERFACE_SETTINGS3 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.Domain == other.Domain
            && self.NameServer == other.NameServer
            && self.SearchList == other.SearchList
            && self.RegistrationEnabled == other.RegistrationEnabled
            && self.RegisterAdapterName == other.RegisterAdapterName
            && self.EnableLLMNR == other.EnableLLMNR
            && self.QueryAdapterName == other.QueryAdapterName
            && self.ProfileNameServer == other.ProfileNameServer
            && self.DisableUnconstrainedQueries == other.DisableUnconstrainedQueries
            && self.SupplementalSearchList == other.SupplementalSearchList
            && self.cServerProperties == other.cServerProperties
            && self.ServerProperties == other.ServerProperties
            && self.cProfileServerProperties == other.cProfileServerProperties
            && self.ProfileServerProperties == other.ProfileServerProperties
    }
}
impl ::core::cmp::Eq for DNS_INTERFACE_SETTINGS3 {}
impl ::core::fmt::Debug for DNS_INTERFACE_SETTINGS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_INTERFACE_SETTINGS3")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("Domain", &self.Domain)
            .field("NameServer", &self.NameServer)
            .field("SearchList", &self.SearchList)
            .field("RegistrationEnabled", &self.RegistrationEnabled)
            .field("RegisterAdapterName", &self.RegisterAdapterName)
            .field("EnableLLMNR", &self.EnableLLMNR)
            .field("QueryAdapterName", &self.QueryAdapterName)
            .field("ProfileNameServer", &self.ProfileNameServer)
            .field("DisableUnconstrainedQueries", &self.DisableUnconstrainedQueries)
            .field("SupplementalSearchList", &self.SupplementalSearchList)
            .field("cServerProperties", &self.cServerProperties)
            .field("ServerProperties", &self.ServerProperties)
            .field("cProfileServerProperties", &self.cProfileServerProperties)
            .field("ProfileServerProperties", &self.ProfileServerProperties)
            .finish()
    }
}
impl ::core::default::Default for DNS_INTERFACE_SETTINGS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_INTERFACE_SETTINGS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.SettingsV1 == other.SettingsV1 && self.DisableUnconstrainedQueries == other.DisableUnconstrainedQueries && self.SupplementalSearchList == other.SupplementalSearchList
    }
}
impl ::core::cmp::Eq for DNS_INTERFACE_SETTINGS_EX {}
impl ::core::fmt::Debug for DNS_INTERFACE_SETTINGS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_INTERFACE_SETTINGS_EX").field("SettingsV1", &self.SettingsV1).field("DisableUnconstrainedQueries", &self.DisableUnconstrainedQueries).field("SupplementalSearchList", &self.SupplementalSearchList).finish()
    }
}
impl ::core::default::Default for DNS_SERVER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_SERVER_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DNS_SERVER_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DNS_SERVER_PROPERTY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DNS_SERVER_PROPERTY_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DNS_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Hostname == other.Hostname && self.Domain == other.Domain && self.SearchList == other.SearchList
    }
}
impl ::core::cmp::Eq for DNS_SETTINGS {}
impl ::core::fmt::Debug for DNS_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SETTINGS").field("Version", &self.Version).field("Flags", &self.Flags).field("Hostname", &self.Hostname).field("Domain", &self.Domain).field("SearchList", &self.SearchList).finish()
    }
}
impl ::core::default::Default for DNS_SETTINGS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DNS_SETTINGS2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Hostname == other.Hostname && self.Domain == other.Domain && self.SearchList == other.SearchList && self.SettingFlags == other.SettingFlags
    }
}
impl ::core::cmp::Eq for DNS_SETTINGS2 {}
impl ::core::fmt::Debug for DNS_SETTINGS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DNS_SETTINGS2").field("Version", &self.Version).field("Flags", &self.Flags).field("Hostname", &self.Hostname).field("Domain", &self.Domain).field("SearchList", &self.SearchList).field("SettingFlags", &self.SettingFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FIXED_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FIXED_INFO_W2KSP1 {
    fn eq(&self, other: &Self) -> bool {
        self.HostName == other.HostName && self.DomainName == other.DomainName && self.CurrentDnsServer == other.CurrentDnsServer && self.DnsServerList == other.DnsServerList && self.NodeType == other.NodeType && self.ScopeId == other.ScopeId && self.EnableRouting == other.EnableRouting && self.EnableProxy == other.EnableProxy && self.EnableDns == other.EnableDns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FIXED_INFO_W2KSP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FIXED_INFO_W2KSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIXED_INFO_W2KSP1").field("HostName", &self.HostName).field("DomainName", &self.DomainName).field("CurrentDnsServer", &self.CurrentDnsServer).field("DnsServerList", &self.DnsServerList).field("NodeType", &self.NodeType).field("ScopeId", &self.ScopeId).field("EnableRouting", &self.EnableRouting).field("EnableProxy", &self.EnableProxy).field("EnableDns", &self.EnableDns).finish()
    }
}
impl ::core::default::Default for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_ADAPTERS_ADDRESSES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_ADAPTERS_ADDRESSES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_ADAPTERS_ADDRESSES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GLOBAL_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLOBAL_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBAL_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for ICMP4_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ICMP4_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICMP4_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ICMP6_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ICMP6_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICMP6_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ICMPV6_ECHO_REPLY_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ICMP_ECHO_REPLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ICMP_ECHO_REPLY {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Status == other.Status && self.RoundTripTime == other.RoundTripTime && self.DataSize == other.DataSize && self.Reserved == other.Reserved && self.Data == other.Data && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for ICMP_ECHO_REPLY {}
impl ::core::fmt::Debug for ICMP_ECHO_REPLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICMP_ECHO_REPLY").field("Address", &self.Address).field("Status", &self.Status).field("RoundTripTime", &self.RoundTripTime).field("DataSize", &self.DataSize).field("Reserved", &self.Reserved).field("Data", &self.Data).field("Options", &self.Options).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ICMP_ECHO_REPLY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for ICMP_ECHO_REPLY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Status == other.Status && self.RoundTripTime == other.RoundTripTime && self.DataSize == other.DataSize && self.Reserved == other.Reserved && self.Data == other.Data && self.Options == other.Options
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for ICMP_ECHO_REPLY32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for ICMP_ECHO_REPLY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICMP_ECHO_REPLY32").field("Address", &self.Address).field("Status", &self.Status).field("RoundTripTime", &self.RoundTripTime).field("DataSize", &self.DataSize).field("Reserved", &self.Reserved).field("Data", &self.Data).field("Options", &self.Options).finish()
    }
}
impl ::core::default::Default for IF_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IF_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IF_ACCESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn eq(&self, other: &Self) -> bool {
        self.SystemTimestamp1 == other.SystemTimestamp1 && self.HardwareClockTimestamp == other.HardwareClockTimestamp && self.SystemTimestamp2 == other.SystemTimestamp2
    }
}
impl ::core::cmp::Eq for INTERFACE_HARDWARE_CROSSTIMESTAMP {}
impl ::core::fmt::Debug for INTERFACE_HARDWARE_CROSSTIMESTAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_HARDWARE_CROSSTIMESTAMP").field("SystemTimestamp1", &self.SystemTimestamp1).field("HardwareClockTimestamp", &self.HardwareClockTimestamp).field("SystemTimestamp2", &self.SystemTimestamp2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.PtpV2OverUdpIPv4EventMessageReceive == other.PtpV2OverUdpIPv4EventMessageReceive
            && self.PtpV2OverUdpIPv4AllMessageReceive == other.PtpV2OverUdpIPv4AllMessageReceive
            && self.PtpV2OverUdpIPv4EventMessageTransmit == other.PtpV2OverUdpIPv4EventMessageTransmit
            && self.PtpV2OverUdpIPv4AllMessageTransmit == other.PtpV2OverUdpIPv4AllMessageTransmit
            && self.PtpV2OverUdpIPv6EventMessageReceive == other.PtpV2OverUdpIPv6EventMessageReceive
            && self.PtpV2OverUdpIPv6AllMessageReceive == other.PtpV2OverUdpIPv6AllMessageReceive
            && self.PtpV2OverUdpIPv6EventMessageTransmit == other.PtpV2OverUdpIPv6EventMessageTransmit
            && self.PtpV2OverUdpIPv6AllMessageTransmit == other.PtpV2OverUdpIPv6AllMessageTransmit
            && self.AllReceive == other.AllReceive
            && self.AllTransmit == other.AllTransmit
            && self.TaggedTransmit == other.TaggedTransmit
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES")
            .field("PtpV2OverUdpIPv4EventMessageReceive", &self.PtpV2OverUdpIPv4EventMessageReceive)
            .field("PtpV2OverUdpIPv4AllMessageReceive", &self.PtpV2OverUdpIPv4AllMessageReceive)
            .field("PtpV2OverUdpIPv4EventMessageTransmit", &self.PtpV2OverUdpIPv4EventMessageTransmit)
            .field("PtpV2OverUdpIPv4AllMessageTransmit", &self.PtpV2OverUdpIPv4AllMessageTransmit)
            .field("PtpV2OverUdpIPv6EventMessageReceive", &self.PtpV2OverUdpIPv6EventMessageReceive)
            .field("PtpV2OverUdpIPv6AllMessageReceive", &self.PtpV2OverUdpIPv6AllMessageReceive)
            .field("PtpV2OverUdpIPv6EventMessageTransmit", &self.PtpV2OverUdpIPv6EventMessageTransmit)
            .field("PtpV2OverUdpIPv6AllMessageTransmit", &self.PtpV2OverUdpIPv6AllMessageTransmit)
            .field("AllReceive", &self.AllReceive)
            .field("AllTransmit", &self.AllTransmit)
            .field("TaggedTransmit", &self.TaggedTransmit)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.AllReceive == other.AllReceive && self.AllTransmit == other.AllTransmit && self.TaggedTransmit == other.TaggedTransmit
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES").field("AllReceive", &self.AllReceive).field("AllTransmit", &self.AllTransmit).field("TaggedTransmit", &self.TaggedTransmit).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERFACE_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERFACE_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.HardwareClockFrequencyHz == other.HardwareClockFrequencyHz && self.SupportsCrossTimestamp == other.SupportsCrossTimestamp && self.HardwareCapabilities == other.HardwareCapabilities && self.SoftwareCapabilities == other.SoftwareCapabilities
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERFACE_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERFACE_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_TIMESTAMP_CAPABILITIES").field("HardwareClockFrequencyHz", &self.HardwareClockFrequencyHz).field("SupportsCrossTimestamp", &self.SupportsCrossTimestamp).field("HardwareCapabilities", &self.HardwareCapabilities).field("SoftwareCapabilities", &self.SoftwareCapabilities).finish()
    }
}
impl ::core::default::Default for INTERNAL_IF_OPER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNAL_IF_OPER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNAL_IF_OPER_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPV6_ADDRESS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ADDRESSES_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IP_ADAPTER_DNS_SUFFIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP_ADAPTER_DNS_SUFFIX {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.String == other.String
    }
}
impl ::core::cmp::Eq for IP_ADAPTER_DNS_SUFFIX {}
impl ::core::fmt::Debug for IP_ADAPTER_DNS_SUFFIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_DNS_SUFFIX").field("Next", &self.Next).field("String", &self.String).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IP_ADAPTER_INDEX_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP_ADAPTER_INDEX_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for IP_ADAPTER_INDEX_MAP {}
impl ::core::fmt::Debug for IP_ADAPTER_INDEX_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_INDEX_MAP").field("Index", &self.Index).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IP_ADAPTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IP_ADAPTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.ComboIndex == other.ComboIndex && self.AdapterName == other.AdapterName && self.Description == other.Description && self.AddressLength == other.AddressLength && self.Address == other.Address && self.Index == other.Index && self.Type == other.Type && self.DhcpEnabled == other.DhcpEnabled && self.CurrentIpAddress == other.CurrentIpAddress && self.IpAddressList == other.IpAddressList && self.GatewayList == other.GatewayList && self.DhcpServer == other.DhcpServer && self.HaveWins == other.HaveWins && self.PrimaryWinsServer == other.PrimaryWinsServer && self.SecondaryWinsServer == other.SecondaryWinsServer && self.LeaseObtained == other.LeaseObtained && self.LeaseExpires == other.LeaseExpires
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IP_ADAPTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IP_ADAPTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_INFO")
            .field("Next", &self.Next)
            .field("ComboIndex", &self.ComboIndex)
            .field("AdapterName", &self.AdapterName)
            .field("Description", &self.Description)
            .field("AddressLength", &self.AddressLength)
            .field("Address", &self.Address)
            .field("Index", &self.Index)
            .field("Type", &self.Type)
            .field("DhcpEnabled", &self.DhcpEnabled)
            .field("CurrentIpAddress", &self.CurrentIpAddress)
            .field("IpAddressList", &self.IpAddressList)
            .field("GatewayList", &self.GatewayList)
            .field("DhcpServer", &self.DhcpServer)
            .field("HaveWins", &self.HaveWins)
            .field("PrimaryWinsServer", &self.PrimaryWinsServer)
            .field("SecondaryWinsServer", &self.SecondaryWinsServer)
            .field("LeaseObtained", &self.LeaseObtained)
            .field("LeaseExpires", &self.LeaseExpires)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IP_ADAPTER_ORDER_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP_ADAPTER_ORDER_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.NumAdapters == other.NumAdapters && self.AdapterOrder == other.AdapterOrder
    }
}
impl ::core::cmp::Eq for IP_ADAPTER_ORDER_MAP {}
impl ::core::fmt::Debug for IP_ADAPTER_ORDER_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADAPTER_ORDER_MAP").field("NumAdapters", &self.NumAdapters).field("AdapterOrder", &self.AdapterOrder).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_PREFIX_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_UNICAST_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_UNICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for IP_ADDRESS_PREFIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IP_ADDRESS_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IP_ADDRESS_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.String == other.String
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IP_ADDRESS_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IP_ADDRESS_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADDRESS_STRING").field("String", &self.String).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IP_ADDR_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IP_ADDR_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.IpAddress == other.IpAddress && self.IpMask == other.IpMask && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IP_ADDR_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IP_ADDR_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_ADDR_STRING").field("Next", &self.Next).field("IpAddress", &self.IpAddress).field("IpMask", &self.IpMask).field("Context", &self.Context).finish()
    }
}
impl ::core::default::Default for IP_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP_INTERFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumAdapters == other.NumAdapters && self.Adapter == other.Adapter
    }
}
impl ::core::cmp::Eq for IP_INTERFACE_INFO {}
impl ::core::fmt::Debug for IP_INTERFACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_INTERFACE_INFO").field("NumAdapters", &self.NumAdapters).field("Adapter", &self.Adapter).finish()
    }
}
impl ::core::default::Default for IP_INTERFACE_NAME_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP_INTERFACE_NAME_INFO_W2KSP1 {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.MediaType == other.MediaType && self.ConnectionType == other.ConnectionType && self.AccessType == other.AccessType && self.DeviceGuid == other.DeviceGuid && self.InterfaceGuid == other.InterfaceGuid
    }
}
impl ::core::cmp::Eq for IP_INTERFACE_NAME_INFO_W2KSP1 {}
impl ::core::fmt::Debug for IP_INTERFACE_NAME_INFO_W2KSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_INTERFACE_NAME_INFO_W2KSP1").field("Index", &self.Index).field("MediaType", &self.MediaType).field("ConnectionType", &self.ConnectionType).field("AccessType", &self.AccessType).field("DeviceGuid", &self.DeviceGuid).field("InterfaceGuid", &self.InterfaceGuid).finish()
    }
}
impl ::core::default::Default for IP_MCAST_COUNTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP_MCAST_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InMcastOctets == other.InMcastOctets && self.OutMcastOctets == other.OutMcastOctets && self.InMcastPkts == other.InMcastPkts && self.OutMcastPkts == other.OutMcastPkts
    }
}
impl ::core::cmp::Eq for IP_MCAST_COUNTER_INFO {}
impl ::core::fmt::Debug for IP_MCAST_COUNTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_MCAST_COUNTER_INFO").field("InMcastOctets", &self.InMcastOctets).field("OutMcastOctets", &self.OutMcastOctets).field("InMcastPkts", &self.InMcastPkts).field("OutMcastPkts", &self.OutMcastPkts).finish()
    }
}
impl ::core::default::Default for IP_OPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP_OPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Ttl == other.Ttl && self.Tos == other.Tos && self.Flags == other.Flags && self.OptionsSize == other.OptionsSize && self.OptionsData == other.OptionsData
    }
}
impl ::core::cmp::Eq for IP_OPTION_INFORMATION {}
impl ::core::fmt::Debug for IP_OPTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_OPTION_INFORMATION").field("Ttl", &self.Ttl).field("Tos", &self.Tos).field("Flags", &self.Flags).field("OptionsSize", &self.OptionsSize).field("OptionsData", &self.OptionsData).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for IP_OPTION_INFORMATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for IP_OPTION_INFORMATION32 {
    fn eq(&self, other: &Self) -> bool {
        self.Ttl == other.Ttl && self.Tos == other.Tos && self.Flags == other.Flags && self.OptionsSize == other.OptionsSize && self.OptionsData == other.OptionsData
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for IP_OPTION_INFORMATION32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for IP_OPTION_INFORMATION32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_OPTION_INFORMATION32").field("Ttl", &self.Ttl).field("Tos", &self.Tos).field("Flags", &self.Flags).field("OptionsSize", &self.OptionsSize).field("OptionsData", &self.OptionsData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn eq(&self, other: &Self) -> bool {
        self.AutoconfigEnabled == other.AutoconfigEnabled && self.AutoconfigActive == other.AutoconfigActive && self.CurrentDnsServer == other.CurrentDnsServer && self.DnsServerList == other.DnsServerList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IP_PER_ADAPTER_INFO_W2KSP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_PER_ADAPTER_INFO_W2KSP1").field("AutoconfigEnabled", &self.AutoconfigEnabled).field("AutoconfigActive", &self.AutoconfigActive).field("CurrentDnsServer", &self.CurrentDnsServer).field("DnsServerList", &self.DnsServerList).finish()
    }
}
impl ::core::default::Default for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.NumAdapters == other.NumAdapters && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {}
impl ::core::fmt::Debug for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_UNIDIRECTIONAL_ADAPTER_ADDRESS").field("NumAdapters", &self.NumAdapters).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for MIBICMPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIBICMPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.icmpInStats == other.icmpInStats && self.icmpOutStats == other.icmpOutStats
    }
}
impl ::core::cmp::Eq for MIBICMPINFO {}
impl ::core::fmt::Debug for MIBICMPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIBICMPINFO").field("icmpInStats", &self.icmpInStats).field("icmpOutStats", &self.icmpOutStats).finish()
    }
}
impl ::core::default::Default for MIBICMPSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIBICMPSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwMsgs == other.dwMsgs && self.dwErrors == other.dwErrors && self.dwDestUnreachs == other.dwDestUnreachs && self.dwTimeExcds == other.dwTimeExcds && self.dwParmProbs == other.dwParmProbs && self.dwSrcQuenchs == other.dwSrcQuenchs && self.dwRedirects == other.dwRedirects && self.dwEchos == other.dwEchos && self.dwEchoReps == other.dwEchoReps && self.dwTimestamps == other.dwTimestamps && self.dwTimestampReps == other.dwTimestampReps && self.dwAddrMasks == other.dwAddrMasks && self.dwAddrMaskReps == other.dwAddrMaskReps
    }
}
impl ::core::cmp::Eq for MIBICMPSTATS {}
impl ::core::fmt::Debug for MIBICMPSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIBICMPSTATS")
            .field("dwMsgs", &self.dwMsgs)
            .field("dwErrors", &self.dwErrors)
            .field("dwDestUnreachs", &self.dwDestUnreachs)
            .field("dwTimeExcds", &self.dwTimeExcds)
            .field("dwParmProbs", &self.dwParmProbs)
            .field("dwSrcQuenchs", &self.dwSrcQuenchs)
            .field("dwRedirects", &self.dwRedirects)
            .field("dwEchos", &self.dwEchos)
            .field("dwEchoReps", &self.dwEchoReps)
            .field("dwTimestamps", &self.dwTimestamps)
            .field("dwTimestampReps", &self.dwTimestampReps)
            .field("dwAddrMasks", &self.dwAddrMasks)
            .field("dwAddrMaskReps", &self.dwAddrMaskReps)
            .finish()
    }
}
impl ::core::default::Default for MIBICMPSTATS_EX_XPSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIBICMPSTATS_EX_XPSP1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwMsgs == other.dwMsgs && self.dwErrors == other.dwErrors && self.rgdwTypeCount == other.rgdwTypeCount
    }
}
impl ::core::cmp::Eq for MIBICMPSTATS_EX_XPSP1 {}
impl ::core::fmt::Debug for MIBICMPSTATS_EX_XPSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIBICMPSTATS_EX_XPSP1").field("dwMsgs", &self.dwMsgs).field("dwErrors", &self.dwErrors).field("rgdwTypeCount", &self.rgdwTypeCount).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_ANYCASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_ANYCASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_BEST_IF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_BEST_IF {
    fn eq(&self, other: &Self) -> bool {
        self.dwDestAddr == other.dwDestAddr && self.dwIfIndex == other.dwIfIndex
    }
}
impl ::core::cmp::Eq for MIB_BEST_IF {}
impl ::core::fmt::Debug for MIB_BEST_IF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_BEST_IF").field("dwDestAddr", &self.dwDestAddr).field("dwIfIndex", &self.dwIfIndex).finish()
    }
}
impl ::core::default::Default for MIB_BOUNDARYROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_BOUNDARYROW {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroupAddress == other.dwGroupAddress && self.dwGroupMask == other.dwGroupMask
    }
}
impl ::core::cmp::Eq for MIB_BOUNDARYROW {}
impl ::core::fmt::Debug for MIB_BOUNDARYROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_BOUNDARYROW").field("dwGroupAddress", &self.dwGroupAddress).field("dwGroupMask", &self.dwGroupMask).finish()
    }
}
impl ::core::default::Default for MIB_ICMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_ICMP {
    fn eq(&self, other: &Self) -> bool {
        self.stats == other.stats
    }
}
impl ::core::cmp::Eq for MIB_ICMP {}
impl ::core::fmt::Debug for MIB_ICMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_ICMP").field("stats", &self.stats).finish()
    }
}
impl ::core::default::Default for MIB_ICMP_EX_XPSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_ICMP_EX_XPSP1 {
    fn eq(&self, other: &Self) -> bool {
        self.icmpInStats == other.icmpInStats && self.icmpOutStats == other.icmpOutStats
    }
}
impl ::core::cmp::Eq for MIB_ICMP_EX_XPSP1 {}
impl ::core::fmt::Debug for MIB_ICMP_EX_XPSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_ICMP_EX_XPSP1").field("icmpInStats", &self.icmpInStats).field("icmpOutStats", &self.icmpOutStats).finish()
    }
}
impl ::core::default::Default for MIB_IFNUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IFNUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.dwValue == other.dwValue
    }
}
impl ::core::cmp::Eq for MIB_IFNUMBER {}
impl ::core::fmt::Debug for MIB_IFNUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFNUMBER").field("dwValue", &self.dwValue).finish()
    }
}
impl ::core::default::Default for MIB_IFROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IFROW {
    fn eq(&self, other: &Self) -> bool {
        self.wszName == other.wszName
            && self.dwIndex == other.dwIndex
            && self.dwType == other.dwType
            && self.dwMtu == other.dwMtu
            && self.dwSpeed == other.dwSpeed
            && self.dwPhysAddrLen == other.dwPhysAddrLen
            && self.bPhysAddr == other.bPhysAddr
            && self.dwAdminStatus == other.dwAdminStatus
            && self.dwOperStatus == other.dwOperStatus
            && self.dwLastChange == other.dwLastChange
            && self.dwInOctets == other.dwInOctets
            && self.dwInUcastPkts == other.dwInUcastPkts
            && self.dwInNUcastPkts == other.dwInNUcastPkts
            && self.dwInDiscards == other.dwInDiscards
            && self.dwInErrors == other.dwInErrors
            && self.dwInUnknownProtos == other.dwInUnknownProtos
            && self.dwOutOctets == other.dwOutOctets
            && self.dwOutUcastPkts == other.dwOutUcastPkts
            && self.dwOutNUcastPkts == other.dwOutNUcastPkts
            && self.dwOutDiscards == other.dwOutDiscards
            && self.dwOutErrors == other.dwOutErrors
            && self.dwOutQLen == other.dwOutQLen
            && self.dwDescrLen == other.dwDescrLen
            && self.bDescr == other.bDescr
    }
}
impl ::core::cmp::Eq for MIB_IFROW {}
impl ::core::fmt::Debug for MIB_IFROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFROW")
            .field("wszName", &self.wszName)
            .field("dwIndex", &self.dwIndex)
            .field("dwType", &self.dwType)
            .field("dwMtu", &self.dwMtu)
            .field("dwSpeed", &self.dwSpeed)
            .field("dwPhysAddrLen", &self.dwPhysAddrLen)
            .field("bPhysAddr", &self.bPhysAddr)
            .field("dwAdminStatus", &self.dwAdminStatus)
            .field("dwOperStatus", &self.dwOperStatus)
            .field("dwLastChange", &self.dwLastChange)
            .field("dwInOctets", &self.dwInOctets)
            .field("dwInUcastPkts", &self.dwInUcastPkts)
            .field("dwInNUcastPkts", &self.dwInNUcastPkts)
            .field("dwInDiscards", &self.dwInDiscards)
            .field("dwInErrors", &self.dwInErrors)
            .field("dwInUnknownProtos", &self.dwInUnknownProtos)
            .field("dwOutOctets", &self.dwOutOctets)
            .field("dwOutUcastPkts", &self.dwOutUcastPkts)
            .field("dwOutNUcastPkts", &self.dwOutNUcastPkts)
            .field("dwOutDiscards", &self.dwOutDiscards)
            .field("dwOutErrors", &self.dwOutErrors)
            .field("dwOutQLen", &self.dwOutQLen)
            .field("dwDescrLen", &self.dwDescrLen)
            .field("bDescr", &self.bDescr)
            .finish()
    }
}
impl ::core::default::Default for MIB_IFSTACK_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IFSTACK_ROW {
    fn eq(&self, other: &Self) -> bool {
        self.HigherLayerInterfaceIndex == other.HigherLayerInterfaceIndex && self.LowerLayerInterfaceIndex == other.LowerLayerInterfaceIndex
    }
}
impl ::core::cmp::Eq for MIB_IFSTACK_ROW {}
impl ::core::fmt::Debug for MIB_IFSTACK_ROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFSTACK_ROW").field("HigherLayerInterfaceIndex", &self.HigherLayerInterfaceIndex).field("LowerLayerInterfaceIndex", &self.LowerLayerInterfaceIndex).finish()
    }
}
impl ::core::default::Default for MIB_IFSTACK_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IFSTACK_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.NumEntries == other.NumEntries && self.Table == other.Table
    }
}
impl ::core::cmp::Eq for MIB_IFSTACK_TABLE {}
impl ::core::fmt::Debug for MIB_IFSTACK_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFSTACK_TABLE").field("NumEntries", &self.NumEntries).field("Table", &self.Table).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIB_IFSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIB_IFSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwIfIndex == other.dwIfIndex && self.dwAdminStatus == other.dwAdminStatus && self.dwOperationalStatus == other.dwOperationalStatus && self.bMHbeatActive == other.bMHbeatActive && self.bMHbeatAlive == other.bMHbeatAlive
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIB_IFSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MIB_IFSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFSTATUS").field("dwIfIndex", &self.dwIfIndex).field("dwAdminStatus", &self.dwAdminStatus).field("dwOperationalStatus", &self.dwOperationalStatus).field("bMHbeatActive", &self.bMHbeatActive).field("bMHbeatAlive", &self.bMHbeatAlive).finish()
    }
}
impl ::core::default::Default for MIB_IFTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IFTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_IFTABLE {}
impl ::core::fmt::Debug for MIB_IFTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IFTABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_IF_ENTRY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIB_IF_ENTRY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IF_ENTRY_LEVEL").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for MIB_IF_ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for MIB_IF_TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_IF_TABLE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIB_IF_TABLE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IF_TABLE_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIB_INVERTEDIFSTACK_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_INVERTEDIFSTACK_ROW {
    fn eq(&self, other: &Self) -> bool {
        self.LowerLayerInterfaceIndex == other.LowerLayerInterfaceIndex && self.HigherLayerInterfaceIndex == other.HigherLayerInterfaceIndex
    }
}
impl ::core::cmp::Eq for MIB_INVERTEDIFSTACK_ROW {}
impl ::core::fmt::Debug for MIB_INVERTEDIFSTACK_ROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_INVERTEDIFSTACK_ROW").field("LowerLayerInterfaceIndex", &self.LowerLayerInterfaceIndex).field("HigherLayerInterfaceIndex", &self.HigherLayerInterfaceIndex).finish()
    }
}
impl ::core::default::Default for MIB_INVERTEDIFSTACK_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_INVERTEDIFSTACK_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.NumEntries == other.NumEntries && self.Table == other.Table
    }
}
impl ::core::cmp::Eq for MIB_INVERTEDIFSTACK_TABLE {}
impl ::core::fmt::Debug for MIB_INVERTEDIFSTACK_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_INVERTEDIFSTACK_TABLE").field("NumEntries", &self.NumEntries).field("Table", &self.Table).finish()
    }
}
impl ::core::default::Default for MIB_IPADDRROW_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPADDRROW_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddr == other.dwAddr && self.dwIndex == other.dwIndex && self.dwMask == other.dwMask && self.dwBCastAddr == other.dwBCastAddr && self.dwReasmSize == other.dwReasmSize && self.unused1 == other.unused1 && self.unused2 == other.unused2
    }
}
impl ::core::cmp::Eq for MIB_IPADDRROW_W2K {}
impl ::core::fmt::Debug for MIB_IPADDRROW_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPADDRROW_W2K").field("dwAddr", &self.dwAddr).field("dwIndex", &self.dwIndex).field("dwMask", &self.dwMask).field("dwBCastAddr", &self.dwBCastAddr).field("dwReasmSize", &self.dwReasmSize).field("unused1", &self.unused1).field("unused2", &self.unused2).finish()
    }
}
impl ::core::default::Default for MIB_IPADDRROW_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPADDRROW_XP {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddr == other.dwAddr && self.dwIndex == other.dwIndex && self.dwMask == other.dwMask && self.dwBCastAddr == other.dwBCastAddr && self.dwReasmSize == other.dwReasmSize && self.unused1 == other.unused1 && self.wType == other.wType
    }
}
impl ::core::cmp::Eq for MIB_IPADDRROW_XP {}
impl ::core::fmt::Debug for MIB_IPADDRROW_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPADDRROW_XP").field("dwAddr", &self.dwAddr).field("dwIndex", &self.dwIndex).field("dwMask", &self.dwMask).field("dwBCastAddr", &self.dwBCastAddr).field("dwReasmSize", &self.dwReasmSize).field("unused1", &self.unused1).field("wType", &self.wType).finish()
    }
}
impl ::core::default::Default for MIB_IPADDRTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPADDRTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_IPADDRTABLE {}
impl ::core::fmt::Debug for MIB_IPADDRTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPADDRTABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPDESTROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPDESTTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_IPFORWARDNUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPFORWARDNUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.dwValue == other.dwValue
    }
}
impl ::core::cmp::Eq for MIB_IPFORWARDNUMBER {}
impl ::core::fmt::Debug for MIB_IPFORWARDNUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPFORWARDNUMBER").field("dwValue", &self.dwValue).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPFORWARDROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_IPFORWARDTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPFORWARD_ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPFORWARD_TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_IPFORWARD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIB_IPFORWARD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IPFORWARD_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPINTERFACE_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPINTERFACE_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_IPMCAST_BOUNDARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_BOUNDARY {
    fn eq(&self, other: &Self) -> bool {
        self.dwIfIndex == other.dwIfIndex && self.dwGroupAddress == other.dwGroupAddress && self.dwGroupMask == other.dwGroupMask && self.dwStatus == other.dwStatus
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_BOUNDARY {}
impl ::core::fmt::Debug for MIB_IPMCAST_BOUNDARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_BOUNDARY").field("dwIfIndex", &self.dwIfIndex).field("dwGroupAddress", &self.dwGroupAddress).field("dwGroupMask", &self.dwGroupMask).field("dwStatus", &self.dwStatus).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_BOUNDARY_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_BOUNDARY_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_BOUNDARY_TABLE {}
impl ::core::fmt::Debug for MIB_IPMCAST_BOUNDARY_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_BOUNDARY_TABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwEnable == other.dwEnable
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_GLOBAL {}
impl ::core::fmt::Debug for MIB_IPMCAST_GLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_GLOBAL").field("dwEnable", &self.dwEnable).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_IF_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_IF_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dwIfIndex == other.dwIfIndex && self.dwTtl == other.dwTtl && self.dwProtocol == other.dwProtocol && self.dwRateLimit == other.dwRateLimit && self.ulInMcastOctets == other.ulInMcastOctets && self.ulOutMcastOctets == other.ulOutMcastOctets
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_IF_ENTRY {}
impl ::core::fmt::Debug for MIB_IPMCAST_IF_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_IF_ENTRY").field("dwIfIndex", &self.dwIfIndex).field("dwTtl", &self.dwTtl).field("dwProtocol", &self.dwProtocol).field("dwRateLimit", &self.dwRateLimit).field("ulInMcastOctets", &self.ulInMcastOctets).field("ulOutMcastOctets", &self.ulOutMcastOctets).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_IF_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_IF_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_IF_TABLE {}
impl ::core::fmt::Debug for MIB_IPMCAST_IF_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_IF_TABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_MFE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_MFE {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroup == other.dwGroup && self.dwSource == other.dwSource && self.dwSrcMask == other.dwSrcMask && self.dwUpStrmNgbr == other.dwUpStrmNgbr && self.dwInIfIndex == other.dwInIfIndex && self.dwInIfProtocol == other.dwInIfProtocol && self.dwRouteProtocol == other.dwRouteProtocol && self.dwRouteNetwork == other.dwRouteNetwork && self.dwRouteMask == other.dwRouteMask && self.ulUpTime == other.ulUpTime && self.ulExpiryTime == other.ulExpiryTime && self.ulTimeOut == other.ulTimeOut && self.ulNumOutIf == other.ulNumOutIf && self.fFlags == other.fFlags && self.dwReserved == other.dwReserved && self.rgmioOutInfo == other.rgmioOutInfo
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_MFE {}
impl ::core::fmt::Debug for MIB_IPMCAST_MFE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_MFE")
            .field("dwGroup", &self.dwGroup)
            .field("dwSource", &self.dwSource)
            .field("dwSrcMask", &self.dwSrcMask)
            .field("dwUpStrmNgbr", &self.dwUpStrmNgbr)
            .field("dwInIfIndex", &self.dwInIfIndex)
            .field("dwInIfProtocol", &self.dwInIfProtocol)
            .field("dwRouteProtocol", &self.dwRouteProtocol)
            .field("dwRouteNetwork", &self.dwRouteNetwork)
            .field("dwRouteMask", &self.dwRouteMask)
            .field("ulUpTime", &self.ulUpTime)
            .field("ulExpiryTime", &self.ulExpiryTime)
            .field("ulTimeOut", &self.ulTimeOut)
            .field("ulNumOutIf", &self.ulNumOutIf)
            .field("fFlags", &self.fFlags)
            .field("dwReserved", &self.dwReserved)
            .field("rgmioOutInfo", &self.rgmioOutInfo)
            .finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_MFE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_MFE_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroup == other.dwGroup && self.dwSource == other.dwSource && self.dwSrcMask == other.dwSrcMask && self.dwUpStrmNgbr == other.dwUpStrmNgbr && self.dwInIfIndex == other.dwInIfIndex && self.dwInIfProtocol == other.dwInIfProtocol && self.dwRouteProtocol == other.dwRouteProtocol && self.dwRouteNetwork == other.dwRouteNetwork && self.dwRouteMask == other.dwRouteMask && self.ulUpTime == other.ulUpTime && self.ulExpiryTime == other.ulExpiryTime && self.ulNumOutIf == other.ulNumOutIf && self.ulInPkts == other.ulInPkts && self.ulInOctets == other.ulInOctets && self.ulPktsDifferentIf == other.ulPktsDifferentIf && self.ulQueueOverflow == other.ulQueueOverflow && self.rgmiosOutStats == other.rgmiosOutStats
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_MFE_STATS {}
impl ::core::fmt::Debug for MIB_IPMCAST_MFE_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_MFE_STATS")
            .field("dwGroup", &self.dwGroup)
            .field("dwSource", &self.dwSource)
            .field("dwSrcMask", &self.dwSrcMask)
            .field("dwUpStrmNgbr", &self.dwUpStrmNgbr)
            .field("dwInIfIndex", &self.dwInIfIndex)
            .field("dwInIfProtocol", &self.dwInIfProtocol)
            .field("dwRouteProtocol", &self.dwRouteProtocol)
            .field("dwRouteNetwork", &self.dwRouteNetwork)
            .field("dwRouteMask", &self.dwRouteMask)
            .field("ulUpTime", &self.ulUpTime)
            .field("ulExpiryTime", &self.ulExpiryTime)
            .field("ulNumOutIf", &self.ulNumOutIf)
            .field("ulInPkts", &self.ulInPkts)
            .field("ulInOctets", &self.ulInOctets)
            .field("ulPktsDifferentIf", &self.ulPktsDifferentIf)
            .field("ulQueueOverflow", &self.ulQueueOverflow)
            .field("rgmiosOutStats", &self.rgmiosOutStats)
            .finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroup == other.dwGroup
            && self.dwSource == other.dwSource
            && self.dwSrcMask == other.dwSrcMask
            && self.dwUpStrmNgbr == other.dwUpStrmNgbr
            && self.dwInIfIndex == other.dwInIfIndex
            && self.dwInIfProtocol == other.dwInIfProtocol
            && self.dwRouteProtocol == other.dwRouteProtocol
            && self.dwRouteNetwork == other.dwRouteNetwork
            && self.dwRouteMask == other.dwRouteMask
            && self.ulUpTime == other.ulUpTime
            && self.ulExpiryTime == other.ulExpiryTime
            && self.ulNumOutIf == other.ulNumOutIf
            && self.ulInPkts == other.ulInPkts
            && self.ulInOctets == other.ulInOctets
            && self.ulPktsDifferentIf == other.ulPktsDifferentIf
            && self.ulQueueOverflow == other.ulQueueOverflow
            && self.ulUninitMfe == other.ulUninitMfe
            && self.ulNegativeMfe == other.ulNegativeMfe
            && self.ulInDiscards == other.ulInDiscards
            && self.ulInHdrErrors == other.ulInHdrErrors
            && self.ulTotalOutPackets == other.ulTotalOutPackets
            && self.rgmiosOutStats == other.rgmiosOutStats
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_MFE_STATS_EX_XP {}
impl ::core::fmt::Debug for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_MFE_STATS_EX_XP")
            .field("dwGroup", &self.dwGroup)
            .field("dwSource", &self.dwSource)
            .field("dwSrcMask", &self.dwSrcMask)
            .field("dwUpStrmNgbr", &self.dwUpStrmNgbr)
            .field("dwInIfIndex", &self.dwInIfIndex)
            .field("dwInIfProtocol", &self.dwInIfProtocol)
            .field("dwRouteProtocol", &self.dwRouteProtocol)
            .field("dwRouteNetwork", &self.dwRouteNetwork)
            .field("dwRouteMask", &self.dwRouteMask)
            .field("ulUpTime", &self.ulUpTime)
            .field("ulExpiryTime", &self.ulExpiryTime)
            .field("ulNumOutIf", &self.ulNumOutIf)
            .field("ulInPkts", &self.ulInPkts)
            .field("ulInOctets", &self.ulInOctets)
            .field("ulPktsDifferentIf", &self.ulPktsDifferentIf)
            .field("ulQueueOverflow", &self.ulQueueOverflow)
            .field("ulUninitMfe", &self.ulUninitMfe)
            .field("ulNegativeMfe", &self.ulNegativeMfe)
            .field("ulInDiscards", &self.ulInDiscards)
            .field("ulInHdrErrors", &self.ulInHdrErrors)
            .field("ulTotalOutPackets", &self.ulTotalOutPackets)
            .field("rgmiosOutStats", &self.rgmiosOutStats)
            .finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_OIF_STATS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_OIF_STATS_LH {
    fn eq(&self, other: &Self) -> bool {
        self.dwOutIfIndex == other.dwOutIfIndex && self.dwNextHopAddr == other.dwNextHopAddr && self.dwDialContext == other.dwDialContext && self.ulTtlTooLow == other.ulTtlTooLow && self.ulFragNeeded == other.ulFragNeeded && self.ulOutPackets == other.ulOutPackets && self.ulOutDiscards == other.ulOutDiscards
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_OIF_STATS_LH {}
impl ::core::fmt::Debug for MIB_IPMCAST_OIF_STATS_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_OIF_STATS_LH").field("dwOutIfIndex", &self.dwOutIfIndex).field("dwNextHopAddr", &self.dwNextHopAddr).field("dwDialContext", &self.dwDialContext).field("ulTtlTooLow", &self.ulTtlTooLow).field("ulFragNeeded", &self.ulFragNeeded).field("ulOutPackets", &self.ulOutPackets).field("ulOutDiscards", &self.ulOutDiscards).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_OIF_STATS_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_OIF_STATS_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.dwOutIfIndex == other.dwOutIfIndex && self.dwNextHopAddr == other.dwNextHopAddr && self.pvDialContext == other.pvDialContext && self.ulTtlTooLow == other.ulTtlTooLow && self.ulFragNeeded == other.ulFragNeeded && self.ulOutPackets == other.ulOutPackets && self.ulOutDiscards == other.ulOutDiscards
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_OIF_STATS_W2K {}
impl ::core::fmt::Debug for MIB_IPMCAST_OIF_STATS_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_OIF_STATS_W2K").field("dwOutIfIndex", &self.dwOutIfIndex).field("dwNextHopAddr", &self.dwNextHopAddr).field("pvDialContext", &self.pvDialContext).field("ulTtlTooLow", &self.ulTtlTooLow).field("ulFragNeeded", &self.ulFragNeeded).field("ulOutPackets", &self.ulOutPackets).field("ulOutDiscards", &self.ulOutDiscards).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_OIF_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_OIF_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.dwOutIfIndex == other.dwOutIfIndex && self.dwNextHopAddr == other.dwNextHopAddr && self.pvReserved == other.pvReserved && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_OIF_W2K {}
impl ::core::fmt::Debug for MIB_IPMCAST_OIF_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_OIF_W2K").field("dwOutIfIndex", &self.dwOutIfIndex).field("dwNextHopAddr", &self.dwNextHopAddr).field("pvReserved", &self.pvReserved).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_OIF_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_OIF_XP {
    fn eq(&self, other: &Self) -> bool {
        self.dwOutIfIndex == other.dwOutIfIndex && self.dwNextHopAddr == other.dwNextHopAddr && self.dwReserved == other.dwReserved && self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_OIF_XP {}
impl ::core::fmt::Debug for MIB_IPMCAST_OIF_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_OIF_XP").field("dwOutIfIndex", &self.dwOutIfIndex).field("dwNextHopAddr", &self.dwNextHopAddr).field("dwReserved", &self.dwReserved).field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::core::default::Default for MIB_IPMCAST_SCOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPMCAST_SCOPE {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroupAddress == other.dwGroupAddress && self.dwGroupMask == other.dwGroupMask && self.snNameBuffer == other.snNameBuffer && self.dwStatus == other.dwStatus
    }
}
impl ::core::cmp::Eq for MIB_IPMCAST_SCOPE {}
impl ::core::fmt::Debug for MIB_IPMCAST_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPMCAST_SCOPE").field("dwGroupAddress", &self.dwGroupAddress).field("dwGroupMask", &self.dwGroupMask).field("snNameBuffer", &self.snNameBuffer).field("dwStatus", &self.dwStatus).finish()
    }
}
impl ::core::default::Default for MIB_IPNETROW_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_IPNETROW_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPNETROW_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.dwIndex == other.dwIndex && self.dwPhysAddrLen == other.dwPhysAddrLen && self.bPhysAddr == other.bPhysAddr && self.dwAddr == other.dwAddr && self.dwType == other.dwType
    }
}
impl ::core::cmp::Eq for MIB_IPNETROW_W2K {}
impl ::core::fmt::Debug for MIB_IPNETROW_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPNETROW_W2K").field("dwIndex", &self.dwIndex).field("dwPhysAddrLen", &self.dwPhysAddrLen).field("bPhysAddr", &self.bPhysAddr).field("dwAddr", &self.dwAddr).field("dwType", &self.dwType).finish()
    }
}
impl ::core::default::Default for MIB_IPNETTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPNET_ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPNET_TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_IPNET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIB_IPNET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IPNET_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPPATH_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IPPATH_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_IPSTATS_FORWARDING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIB_IPSTATS_FORWARDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_IPSTATS_FORWARDING").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIB_IPSTATS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_IPSTATS_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_IPSTATS_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.dwForwarding == other.dwForwarding
            && self.dwDefaultTTL == other.dwDefaultTTL
            && self.dwInReceives == other.dwInReceives
            && self.dwInHdrErrors == other.dwInHdrErrors
            && self.dwInAddrErrors == other.dwInAddrErrors
            && self.dwForwDatagrams == other.dwForwDatagrams
            && self.dwInUnknownProtos == other.dwInUnknownProtos
            && self.dwInDiscards == other.dwInDiscards
            && self.dwInDelivers == other.dwInDelivers
            && self.dwOutRequests == other.dwOutRequests
            && self.dwRoutingDiscards == other.dwRoutingDiscards
            && self.dwOutDiscards == other.dwOutDiscards
            && self.dwOutNoRoutes == other.dwOutNoRoutes
            && self.dwReasmTimeout == other.dwReasmTimeout
            && self.dwReasmReqds == other.dwReasmReqds
            && self.dwReasmOks == other.dwReasmOks
            && self.dwReasmFails == other.dwReasmFails
            && self.dwFragOks == other.dwFragOks
            && self.dwFragFails == other.dwFragFails
            && self.dwFragCreates == other.dwFragCreates
            && self.dwNumIf == other.dwNumIf
            && self.dwNumAddr == other.dwNumAddr
            && self.dwNumRoutes == other.dwNumRoutes
    }
}
impl ::core::cmp::Eq for MIB_IPSTATS_W2K {}
impl ::core::fmt::Debug for MIB_IPSTATS_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IPSTATS_W2K")
            .field("dwForwarding", &self.dwForwarding)
            .field("dwDefaultTTL", &self.dwDefaultTTL)
            .field("dwInReceives", &self.dwInReceives)
            .field("dwInHdrErrors", &self.dwInHdrErrors)
            .field("dwInAddrErrors", &self.dwInAddrErrors)
            .field("dwForwDatagrams", &self.dwForwDatagrams)
            .field("dwInUnknownProtos", &self.dwInUnknownProtos)
            .field("dwInDiscards", &self.dwInDiscards)
            .field("dwInDelivers", &self.dwInDelivers)
            .field("dwOutRequests", &self.dwOutRequests)
            .field("dwRoutingDiscards", &self.dwRoutingDiscards)
            .field("dwOutDiscards", &self.dwOutDiscards)
            .field("dwOutNoRoutes", &self.dwOutNoRoutes)
            .field("dwReasmTimeout", &self.dwReasmTimeout)
            .field("dwReasmReqds", &self.dwReasmReqds)
            .field("dwReasmOks", &self.dwReasmOks)
            .field("dwReasmFails", &self.dwReasmFails)
            .field("dwFragOks", &self.dwFragOks)
            .field("dwFragFails", &self.dwFragFails)
            .field("dwFragCreates", &self.dwFragCreates)
            .field("dwNumIf", &self.dwNumIf)
            .field("dwNumAddr", &self.dwNumAddr)
            .field("dwNumRoutes", &self.dwNumRoutes)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    fn eq(&self, other: &Self) -> bool {
        self.InboundBandwidthInformation == other.InboundBandwidthInformation && self.OutboundBandwidthInformation == other.OutboundBandwidthInformation
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES").field("InboundBandwidthInformation", &self.InboundBandwidthInformation).field("OutboundBandwidthInformation", &self.OutboundBandwidthInformation).finish()
    }
}
impl ::core::default::Default for MIB_MCAST_LIMIT_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_MCAST_LIMIT_ROW {
    fn eq(&self, other: &Self) -> bool {
        self.dwTtl == other.dwTtl && self.dwRateLimit == other.dwRateLimit
    }
}
impl ::core::cmp::Eq for MIB_MCAST_LIMIT_ROW {}
impl ::core::fmt::Debug for MIB_MCAST_LIMIT_ROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_MCAST_LIMIT_ROW").field("dwTtl", &self.dwTtl).field("dwRateLimit", &self.dwRateLimit).finish()
    }
}
impl ::core::default::Default for MIB_MFE_STATS_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_MFE_STATS_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_MFE_STATS_TABLE {}
impl ::core::fmt::Debug for MIB_MFE_STATS_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_MFE_STATS_TABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_MFE_STATS_TABLE_EX_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_MFE_STATS_TABLE_EX_XP {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_MFE_STATS_TABLE_EX_XP {}
impl ::core::fmt::Debug for MIB_MFE_STATS_TABLE_EX_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_MFE_STATS_TABLE_EX_XP").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_MFE_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_MFE_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_MFE_TABLE {}
impl ::core::fmt::Debug for MIB_MFE_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_MFE_TABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_MULTICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_MULTICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIB_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIB_OPAQUE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_OPAQUE_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_OPAQUE_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.dwVarId == other.dwVarId && self.rgdwVarIndex == other.rgdwVarIndex
    }
}
impl ::core::cmp::Eq for MIB_OPAQUE_QUERY {}
impl ::core::fmt::Debug for MIB_OPAQUE_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_OPAQUE_QUERY").field("dwVarId", &self.dwVarId).field("rgdwVarIndex", &self.rgdwVarIndex).finish()
    }
}
impl ::core::default::Default for MIB_PROXYARP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_PROXYARP {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddress == other.dwAddress && self.dwMask == other.dwMask && self.dwIfIndex == other.dwIfIndex
    }
}
impl ::core::cmp::Eq for MIB_PROXYARP {}
impl ::core::fmt::Debug for MIB_PROXYARP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_PROXYARP").field("dwAddress", &self.dwAddress).field("dwMask", &self.dwMask).field("dwIfIndex", &self.dwIfIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIB_ROUTESTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIB_ROUTESTATE {
    fn eq(&self, other: &Self) -> bool {
        self.bRoutesSetToStack == other.bRoutesSetToStack
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIB_ROUTESTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MIB_ROUTESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_ROUTESTATE").field("bRoutesSetToStack", &self.bRoutesSetToStack).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_TCP6ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_TCP6ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_TCP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCP6ROW_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.ucLocalAddr == other.ucLocalAddr && self.dwLocalScopeId == other.dwLocalScopeId && self.dwLocalPort == other.dwLocalPort && self.ucRemoteAddr == other.ucRemoteAddr && self.dwRemoteScopeId == other.dwRemoteScopeId && self.dwRemotePort == other.dwRemotePort && self.dwState == other.dwState && self.dwOwningPid == other.dwOwningPid && self.liCreateTimestamp == other.liCreateTimestamp && self.OwningModuleInfo == other.OwningModuleInfo
    }
}
impl ::core::cmp::Eq for MIB_TCP6ROW_OWNER_MODULE {}
impl ::core::fmt::Debug for MIB_TCP6ROW_OWNER_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCP6ROW_OWNER_MODULE")
            .field("ucLocalAddr", &self.ucLocalAddr)
            .field("dwLocalScopeId", &self.dwLocalScopeId)
            .field("dwLocalPort", &self.dwLocalPort)
            .field("ucRemoteAddr", &self.ucRemoteAddr)
            .field("dwRemoteScopeId", &self.dwRemoteScopeId)
            .field("dwRemotePort", &self.dwRemotePort)
            .field("dwState", &self.dwState)
            .field("dwOwningPid", &self.dwOwningPid)
            .field("liCreateTimestamp", &self.liCreateTimestamp)
            .field("OwningModuleInfo", &self.OwningModuleInfo)
            .finish()
    }
}
impl ::core::default::Default for MIB_TCP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCP6ROW_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        self.ucLocalAddr == other.ucLocalAddr && self.dwLocalScopeId == other.dwLocalScopeId && self.dwLocalPort == other.dwLocalPort && self.ucRemoteAddr == other.ucRemoteAddr && self.dwRemoteScopeId == other.dwRemoteScopeId && self.dwRemotePort == other.dwRemotePort && self.dwState == other.dwState && self.dwOwningPid == other.dwOwningPid
    }
}
impl ::core::cmp::Eq for MIB_TCP6ROW_OWNER_PID {}
impl ::core::fmt::Debug for MIB_TCP6ROW_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCP6ROW_OWNER_PID").field("ucLocalAddr", &self.ucLocalAddr).field("dwLocalScopeId", &self.dwLocalScopeId).field("dwLocalPort", &self.dwLocalPort).field("ucRemoteAddr", &self.ucRemoteAddr).field("dwRemoteScopeId", &self.dwRemoteScopeId).field("dwRemotePort", &self.dwRemotePort).field("dwState", &self.dwState).field("dwOwningPid", &self.dwOwningPid).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_TCP6TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_TCP6TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_TCP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCP6TABLE_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_TCP6TABLE_OWNER_MODULE {}
impl ::core::fmt::Debug for MIB_TCP6TABLE_OWNER_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCP6TABLE_OWNER_MODULE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_TCP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCP6TABLE_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_TCP6TABLE_OWNER_PID {}
impl ::core::fmt::Debug for MIB_TCP6TABLE_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCP6TABLE_OWNER_PID").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_TCPROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPROW2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwState == other.dwState && self.dwLocalAddr == other.dwLocalAddr && self.dwLocalPort == other.dwLocalPort && self.dwRemoteAddr == other.dwRemoteAddr && self.dwRemotePort == other.dwRemotePort && self.dwOwningPid == other.dwOwningPid && self.dwOffloadState == other.dwOffloadState
    }
}
impl ::core::cmp::Eq for MIB_TCPROW2 {}
impl ::core::fmt::Debug for MIB_TCPROW2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPROW2").field("dwState", &self.dwState).field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwRemoteAddr", &self.dwRemoteAddr).field("dwRemotePort", &self.dwRemotePort).field("dwOwningPid", &self.dwOwningPid).field("dwOffloadState", &self.dwOffloadState).finish()
    }
}
impl ::core::default::Default for MIB_TCPROW_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_TCPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPROW_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.dwState == other.dwState && self.dwLocalAddr == other.dwLocalAddr && self.dwLocalPort == other.dwLocalPort && self.dwRemoteAddr == other.dwRemoteAddr && self.dwRemotePort == other.dwRemotePort && self.dwOwningPid == other.dwOwningPid && self.liCreateTimestamp == other.liCreateTimestamp && self.OwningModuleInfo == other.OwningModuleInfo
    }
}
impl ::core::cmp::Eq for MIB_TCPROW_OWNER_MODULE {}
impl ::core::fmt::Debug for MIB_TCPROW_OWNER_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPROW_OWNER_MODULE").field("dwState", &self.dwState).field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwRemoteAddr", &self.dwRemoteAddr).field("dwRemotePort", &self.dwRemotePort).field("dwOwningPid", &self.dwOwningPid).field("liCreateTimestamp", &self.liCreateTimestamp).field("OwningModuleInfo", &self.OwningModuleInfo).finish()
    }
}
impl ::core::default::Default for MIB_TCPROW_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPROW_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        self.dwState == other.dwState && self.dwLocalAddr == other.dwLocalAddr && self.dwLocalPort == other.dwLocalPort && self.dwRemoteAddr == other.dwRemoteAddr && self.dwRemotePort == other.dwRemotePort && self.dwOwningPid == other.dwOwningPid
    }
}
impl ::core::cmp::Eq for MIB_TCPROW_OWNER_PID {}
impl ::core::fmt::Debug for MIB_TCPROW_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPROW_OWNER_PID").field("dwState", &self.dwState).field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwRemoteAddr", &self.dwRemoteAddr).field("dwRemotePort", &self.dwRemotePort).field("dwOwningPid", &self.dwOwningPid).finish()
    }
}
impl ::core::default::Default for MIB_TCPROW_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPROW_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.dwState == other.dwState && self.dwLocalAddr == other.dwLocalAddr && self.dwLocalPort == other.dwLocalPort && self.dwRemoteAddr == other.dwRemoteAddr && self.dwRemotePort == other.dwRemotePort
    }
}
impl ::core::cmp::Eq for MIB_TCPROW_W2K {}
impl ::core::fmt::Debug for MIB_TCPROW_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPROW_W2K").field("dwState", &self.dwState).field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwRemoteAddr", &self.dwRemoteAddr).field("dwRemotePort", &self.dwRemotePort).finish()
    }
}
impl ::core::default::Default for MIB_TCPSTATS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPSTATS2 {
    fn eq(&self, other: &Self) -> bool {
        self.RtoAlgorithm == other.RtoAlgorithm && self.dwRtoMin == other.dwRtoMin && self.dwRtoMax == other.dwRtoMax && self.dwMaxConn == other.dwMaxConn && self.dwActiveOpens == other.dwActiveOpens && self.dwPassiveOpens == other.dwPassiveOpens && self.dwAttemptFails == other.dwAttemptFails && self.dwEstabResets == other.dwEstabResets && self.dwCurrEstab == other.dwCurrEstab && self.dw64InSegs == other.dw64InSegs && self.dw64OutSegs == other.dw64OutSegs && self.dwRetransSegs == other.dwRetransSegs && self.dwInErrs == other.dwInErrs && self.dwOutRsts == other.dwOutRsts && self.dwNumConns == other.dwNumConns
    }
}
impl ::core::cmp::Eq for MIB_TCPSTATS2 {}
impl ::core::fmt::Debug for MIB_TCPSTATS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPSTATS2")
            .field("RtoAlgorithm", &self.RtoAlgorithm)
            .field("dwRtoMin", &self.dwRtoMin)
            .field("dwRtoMax", &self.dwRtoMax)
            .field("dwMaxConn", &self.dwMaxConn)
            .field("dwActiveOpens", &self.dwActiveOpens)
            .field("dwPassiveOpens", &self.dwPassiveOpens)
            .field("dwAttemptFails", &self.dwAttemptFails)
            .field("dwEstabResets", &self.dwEstabResets)
            .field("dwCurrEstab", &self.dwCurrEstab)
            .field("dw64InSegs", &self.dw64InSegs)
            .field("dw64OutSegs", &self.dw64OutSegs)
            .field("dwRetransSegs", &self.dwRetransSegs)
            .field("dwInErrs", &self.dwInErrs)
            .field("dwOutRsts", &self.dwOutRsts)
            .field("dwNumConns", &self.dwNumConns)
            .finish()
    }
}
impl ::core::default::Default for MIB_TCPSTATS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_TCPSTATS_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPSTATS_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.dwRtoAlgorithm == other.dwRtoAlgorithm && self.dwRtoMin == other.dwRtoMin && self.dwRtoMax == other.dwRtoMax && self.dwMaxConn == other.dwMaxConn && self.dwActiveOpens == other.dwActiveOpens && self.dwPassiveOpens == other.dwPassiveOpens && self.dwAttemptFails == other.dwAttemptFails && self.dwEstabResets == other.dwEstabResets && self.dwCurrEstab == other.dwCurrEstab && self.dwInSegs == other.dwInSegs && self.dwOutSegs == other.dwOutSegs && self.dwRetransSegs == other.dwRetransSegs && self.dwInErrs == other.dwInErrs && self.dwOutRsts == other.dwOutRsts && self.dwNumConns == other.dwNumConns
    }
}
impl ::core::cmp::Eq for MIB_TCPSTATS_W2K {}
impl ::core::fmt::Debug for MIB_TCPSTATS_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPSTATS_W2K")
            .field("dwRtoAlgorithm", &self.dwRtoAlgorithm)
            .field("dwRtoMin", &self.dwRtoMin)
            .field("dwRtoMax", &self.dwRtoMax)
            .field("dwMaxConn", &self.dwMaxConn)
            .field("dwActiveOpens", &self.dwActiveOpens)
            .field("dwPassiveOpens", &self.dwPassiveOpens)
            .field("dwAttemptFails", &self.dwAttemptFails)
            .field("dwEstabResets", &self.dwEstabResets)
            .field("dwCurrEstab", &self.dwCurrEstab)
            .field("dwInSegs", &self.dwInSegs)
            .field("dwOutSegs", &self.dwOutSegs)
            .field("dwRetransSegs", &self.dwRetransSegs)
            .field("dwInErrs", &self.dwInErrs)
            .field("dwOutRsts", &self.dwOutRsts)
            .field("dwNumConns", &self.dwNumConns)
            .finish()
    }
}
impl ::core::default::Default for MIB_TCPTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_TCPTABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPTABLE2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_TCPTABLE2 {}
impl ::core::fmt::Debug for MIB_TCPTABLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPTABLE2").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_TCPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPTABLE_OWNER_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_TCPTABLE_OWNER_MODULE {}
impl ::core::fmt::Debug for MIB_TCPTABLE_OWNER_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPTABLE_OWNER_MODULE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_TCPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_TCPTABLE_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_TCPTABLE_OWNER_PID {}
impl ::core::fmt::Debug for MIB_TCPTABLE_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_TCPTABLE_OWNER_PID").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_TCP_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIB_TCP_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIB_TCP_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_UDP6ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDP6ROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_UDP6ROW_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        self.ucLocalAddr == other.ucLocalAddr && self.dwLocalScopeId == other.dwLocalScopeId && self.dwLocalPort == other.dwLocalPort && self.dwOwningPid == other.dwOwningPid
    }
}
impl ::core::cmp::Eq for MIB_UDP6ROW_OWNER_PID {}
impl ::core::fmt::Debug for MIB_UDP6ROW_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDP6ROW_OWNER_PID").field("ucLocalAddr", &self.ucLocalAddr).field("dwLocalScopeId", &self.dwLocalScopeId).field("dwLocalPort", &self.dwLocalPort).field("dwOwningPid", &self.dwOwningPid).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MIB_UDP6TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDP6TABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_UDP6TABLE_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_UDP6TABLE_OWNER_PID {}
impl ::core::fmt::Debug for MIB_UDP6TABLE_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDP6TABLE_OWNER_PID").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_UDPROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_UDPROW {
    fn eq(&self, other: &Self) -> bool {
        self.dwLocalAddr == other.dwLocalAddr && self.dwLocalPort == other.dwLocalPort
    }
}
impl ::core::cmp::Eq for MIB_UDPROW {}
impl ::core::fmt::Debug for MIB_UDPROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPROW").field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).finish()
    }
}
impl ::core::default::Default for MIB_UDPROW2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDPROW_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_UDPROW_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        self.dwLocalAddr == other.dwLocalAddr && self.dwLocalPort == other.dwLocalPort && self.dwOwningPid == other.dwOwningPid
    }
}
impl ::core::cmp::Eq for MIB_UDPROW_OWNER_PID {}
impl ::core::fmt::Debug for MIB_UDPROW_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPROW_OWNER_PID").field("dwLocalAddr", &self.dwLocalAddr).field("dwLocalPort", &self.dwLocalPort).field("dwOwningPid", &self.dwOwningPid).finish()
    }
}
impl ::core::default::Default for MIB_UDPSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_UDPSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwInDatagrams == other.dwInDatagrams && self.dwNoPorts == other.dwNoPorts && self.dwInErrors == other.dwInErrors && self.dwOutDatagrams == other.dwOutDatagrams && self.dwNumAddrs == other.dwNumAddrs
    }
}
impl ::core::cmp::Eq for MIB_UDPSTATS {}
impl ::core::fmt::Debug for MIB_UDPSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPSTATS").field("dwInDatagrams", &self.dwInDatagrams).field("dwNoPorts", &self.dwNoPorts).field("dwInErrors", &self.dwInErrors).field("dwOutDatagrams", &self.dwOutDatagrams).field("dwNumAddrs", &self.dwNumAddrs).finish()
    }
}
impl ::core::default::Default for MIB_UDPSTATS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_UDPSTATS2 {
    fn eq(&self, other: &Self) -> bool {
        self.dw64InDatagrams == other.dw64InDatagrams && self.dwNoPorts == other.dwNoPorts && self.dwInErrors == other.dwInErrors && self.dw64OutDatagrams == other.dw64OutDatagrams && self.dwNumAddrs == other.dwNumAddrs
    }
}
impl ::core::cmp::Eq for MIB_UDPSTATS2 {}
impl ::core::fmt::Debug for MIB_UDPSTATS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPSTATS2").field("dw64InDatagrams", &self.dw64InDatagrams).field("dwNoPorts", &self.dwNoPorts).field("dwInErrors", &self.dwInErrors).field("dw64OutDatagrams", &self.dw64OutDatagrams).field("dwNumAddrs", &self.dwNumAddrs).finish()
    }
}
impl ::core::default::Default for MIB_UDPTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_UDPTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_UDPTABLE {}
impl ::core::fmt::Debug for MIB_UDPTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPTABLE").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
impl ::core::default::Default for MIB_UDPTABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIB_UDPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIB_UDPTABLE_OWNER_PID {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.table == other.table
    }
}
impl ::core::cmp::Eq for MIB_UDPTABLE_OWNER_PID {}
impl ::core::fmt::Debug for MIB_UDPTABLE_OWNER_PID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIB_UDPTABLE_OWNER_PID").field("dwNumEntries", &self.dwNumEntries).field("table", &self.table).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_UNICASTIPADDRESS_ROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MIB_UNICASTIPADDRESS_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NET_ADDRESS_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_ADDRESS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_ADDRESS_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for PFADDRESSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PFADDRESSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFADDRESSTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PFFORWARD_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PFFORWARD_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFFORWARD_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PFFRAMETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PFFRAMETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFFRAMETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PFLOGFRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PFLOGFRAME {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.pfeTypeOfFrame == other.pfeTypeOfFrame && self.dwTotalSizeUsed == other.dwTotalSizeUsed && self.dwFilterRule == other.dwFilterRule && self.wSizeOfAdditionalData == other.wSizeOfAdditionalData && self.wSizeOfIpHeader == other.wSizeOfIpHeader && self.dwInterfaceName == other.dwInterfaceName && self.dwIPIndex == other.dwIPIndex && self.bPacketData == other.bPacketData
    }
}
impl ::core::cmp::Eq for PFLOGFRAME {}
impl ::core::fmt::Debug for PFLOGFRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PFLOGFRAME").field("Timestamp", &self.Timestamp).field("pfeTypeOfFrame", &self.pfeTypeOfFrame).field("dwTotalSizeUsed", &self.dwTotalSizeUsed).field("dwFilterRule", &self.dwFilterRule).field("wSizeOfAdditionalData", &self.wSizeOfAdditionalData).field("wSizeOfIpHeader", &self.wSizeOfIpHeader).field("dwInterfaceName", &self.dwInterfaceName).field("dwIPIndex", &self.dwIPIndex).field("bPacketData", &self.bPacketData).finish()
    }
}
impl ::core::default::Default for PF_FILTER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PF_FILTER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwFilterFlags == other.dwFilterFlags && self.dwRule == other.dwRule && self.pfatType == other.pfatType && self.SrcAddr == other.SrcAddr && self.SrcMask == other.SrcMask && self.DstAddr == other.DstAddr && self.DstMask == other.DstMask && self.dwProtocol == other.dwProtocol && self.fLateBound == other.fLateBound && self.wSrcPort == other.wSrcPort && self.wDstPort == other.wDstPort && self.wSrcPortHighRange == other.wSrcPortHighRange && self.wDstPortHighRange == other.wDstPortHighRange
    }
}
impl ::core::cmp::Eq for PF_FILTER_DESCRIPTOR {}
impl ::core::fmt::Debug for PF_FILTER_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PF_FILTER_DESCRIPTOR")
            .field("dwFilterFlags", &self.dwFilterFlags)
            .field("dwRule", &self.dwRule)
            .field("pfatType", &self.pfatType)
            .field("SrcAddr", &self.SrcAddr)
            .field("SrcMask", &self.SrcMask)
            .field("DstAddr", &self.DstAddr)
            .field("DstMask", &self.DstMask)
            .field("dwProtocol", &self.dwProtocol)
            .field("fLateBound", &self.fLateBound)
            .field("wSrcPort", &self.wSrcPort)
            .field("wDstPort", &self.wDstPort)
            .field("wSrcPortHighRange", &self.wSrcPortHighRange)
            .field("wDstPortHighRange", &self.wDstPortHighRange)
            .finish()
    }
}
impl ::core::default::Default for PF_FILTER_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PF_FILTER_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPacketsFiltered == other.dwNumPacketsFiltered && self.info == other.info
    }
}
impl ::core::cmp::Eq for PF_FILTER_STATS {}
impl ::core::fmt::Debug for PF_FILTER_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PF_FILTER_STATS").field("dwNumPacketsFiltered", &self.dwNumPacketsFiltered).field("info", &self.info).finish()
    }
}
impl ::core::default::Default for PF_INTERFACE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PF_INTERFACE_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.pvDriverContext == other.pvDriverContext && self.dwFlags == other.dwFlags && self.dwInDrops == other.dwInDrops && self.dwOutDrops == other.dwOutDrops && self.eaInAction == other.eaInAction && self.eaOutAction == other.eaOutAction && self.dwNumInFilters == other.dwNumInFilters && self.dwNumOutFilters == other.dwNumOutFilters && self.dwFrag == other.dwFrag && self.dwSpoof == other.dwSpoof && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.liSYN == other.liSYN && self.liTotalLogged == other.liTotalLogged && self.dwLostLogEntries == other.dwLostLogEntries && self.FilterInfo == other.FilterInfo
    }
}
impl ::core::cmp::Eq for PF_INTERFACE_STATS {}
impl ::core::fmt::Debug for PF_INTERFACE_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PF_INTERFACE_STATS")
            .field("pvDriverContext", &self.pvDriverContext)
            .field("dwFlags", &self.dwFlags)
            .field("dwInDrops", &self.dwInDrops)
            .field("dwOutDrops", &self.dwOutDrops)
            .field("eaInAction", &self.eaInAction)
            .field("eaOutAction", &self.eaOutAction)
            .field("dwNumInFilters", &self.dwNumInFilters)
            .field("dwNumOutFilters", &self.dwNumOutFilters)
            .field("dwFrag", &self.dwFrag)
            .field("dwSpoof", &self.dwSpoof)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("liSYN", &self.liSYN)
            .field("liTotalLogged", &self.liTotalLogged)
            .field("dwLostLogEntries", &self.dwLostLogEntries)
            .field("FilterInfo", &self.FilterInfo)
            .finish()
    }
}
impl ::core::default::Default for PF_LATEBIND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PF_LATEBIND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SrcAddr == other.SrcAddr && self.DstAddr == other.DstAddr && self.Mask == other.Mask
    }
}
impl ::core::cmp::Eq for PF_LATEBIND_INFO {}
impl ::core::fmt::Debug for PF_LATEBIND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PF_LATEBIND_INFO").field("SrcAddr", &self.SrcAddr).field("DstAddr", &self.DstAddr).field("Mask", &self.Mask).finish()
    }
}
impl ::core::default::Default for TCPIP_OWNER_MODULE_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCPIP_OWNER_MODULE_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pModuleName == other.pModuleName && self.pModulePath == other.pModulePath
    }
}
impl ::core::cmp::Eq for TCPIP_OWNER_MODULE_BASIC_INFO {}
impl ::core::fmt::Debug for TCPIP_OWNER_MODULE_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCPIP_OWNER_MODULE_BASIC_INFO").field("pModuleName", &self.pModuleName).field("pModulePath", &self.pModulePath).finish()
    }
}
impl ::core::default::Default for TCPIP_OWNER_MODULE_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCPIP_OWNER_MODULE_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCPIP_OWNER_MODULE_INFO_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TCP_BOOLEAN_OPTIONAL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCP_BOOLEAN_OPTIONAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_BOOLEAN_OPTIONAL").field(&self.0).finish()
    }
}
impl ::core::default::Default for TCP_CONNECTION_OFFLOAD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCP_CONNECTION_OFFLOAD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_CONNECTION_OFFLOAD_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.OutboundBandwidth == other.OutboundBandwidth && self.InboundBandwidth == other.InboundBandwidth && self.OutboundInstability == other.OutboundInstability && self.InboundInstability == other.InboundInstability && self.OutboundBandwidthPeaked == other.OutboundBandwidthPeaked && self.InboundBandwidthPeaked == other.InboundBandwidthPeaked
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_BANDWIDTH_ROD_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_BANDWIDTH_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_BANDWIDTH_ROD_v0").field("OutboundBandwidth", &self.OutboundBandwidth).field("InboundBandwidth", &self.InboundBandwidth).field("OutboundInstability", &self.OutboundInstability).field("InboundInstability", &self.InboundInstability).field("OutboundBandwidthPeaked", &self.OutboundBandwidthPeaked).field("InboundBandwidthPeaked", &self.InboundBandwidthPeaked).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_BANDWIDTH_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_BANDWIDTH_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.EnableCollectionOutbound == other.EnableCollectionOutbound && self.EnableCollectionInbound == other.EnableCollectionInbound
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_BANDWIDTH_RW_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_BANDWIDTH_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_BANDWIDTH_RW_v0").field("EnableCollectionOutbound", &self.EnableCollectionOutbound).field("EnableCollectionInbound", &self.EnableCollectionInbound).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_DATA_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_DATA_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.DataBytesOut == other.DataBytesOut && self.DataSegsOut == other.DataSegsOut && self.DataBytesIn == other.DataBytesIn && self.DataSegsIn == other.DataSegsIn && self.SegsOut == other.SegsOut && self.SegsIn == other.SegsIn && self.SoftErrors == other.SoftErrors && self.SoftErrorReason == other.SoftErrorReason && self.SndUna == other.SndUna && self.SndNxt == other.SndNxt && self.SndMax == other.SndMax && self.ThruBytesAcked == other.ThruBytesAcked && self.RcvNxt == other.RcvNxt && self.ThruBytesReceived == other.ThruBytesReceived
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_DATA_ROD_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_DATA_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_DATA_ROD_v0")
            .field("DataBytesOut", &self.DataBytesOut)
            .field("DataSegsOut", &self.DataSegsOut)
            .field("DataBytesIn", &self.DataBytesIn)
            .field("DataSegsIn", &self.DataSegsIn)
            .field("SegsOut", &self.SegsOut)
            .field("SegsIn", &self.SegsIn)
            .field("SoftErrors", &self.SoftErrors)
            .field("SoftErrorReason", &self.SoftErrorReason)
            .field("SndUna", &self.SndUna)
            .field("SndNxt", &self.SndNxt)
            .field("SndMax", &self.SndMax)
            .field("ThruBytesAcked", &self.ThruBytesAcked)
            .field("RcvNxt", &self.RcvNxt)
            .field("ThruBytesReceived", &self.ThruBytesReceived)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_DATA_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_DATA_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.EnableCollection == other.EnableCollection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_DATA_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_DATA_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_DATA_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_FINE_RTT_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_FINE_RTT_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.RttVar == other.RttVar && self.MaxRtt == other.MaxRtt && self.MinRtt == other.MinRtt && self.SumRtt == other.SumRtt
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_FINE_RTT_ROD_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_FINE_RTT_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_FINE_RTT_ROD_v0").field("RttVar", &self.RttVar).field("MaxRtt", &self.MaxRtt).field("MinRtt", &self.MinRtt).field("SumRtt", &self.SumRtt).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_FINE_RTT_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_FINE_RTT_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.EnableCollection == other.EnableCollection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_FINE_RTT_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_FINE_RTT_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_FINE_RTT_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_OBS_REC_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_OBS_REC_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.CurRwinRcvd == other.CurRwinRcvd && self.MaxRwinRcvd == other.MaxRwinRcvd && self.MinRwinRcvd == other.MinRwinRcvd && self.WinScaleRcvd == other.WinScaleRcvd
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_OBS_REC_ROD_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_OBS_REC_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_OBS_REC_ROD_v0").field("CurRwinRcvd", &self.CurRwinRcvd).field("MaxRwinRcvd", &self.MaxRwinRcvd).field("MinRwinRcvd", &self.MinRwinRcvd).field("WinScaleRcvd", &self.WinScaleRcvd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_OBS_REC_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_OBS_REC_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.EnableCollection == other.EnableCollection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_OBS_REC_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_OBS_REC_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_OBS_REC_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_PATH_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_PATH_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.FastRetran == other.FastRetran
            && self.Timeouts == other.Timeouts
            && self.SubsequentTimeouts == other.SubsequentTimeouts
            && self.CurTimeoutCount == other.CurTimeoutCount
            && self.AbruptTimeouts == other.AbruptTimeouts
            && self.PktsRetrans == other.PktsRetrans
            && self.BytesRetrans == other.BytesRetrans
            && self.DupAcksIn == other.DupAcksIn
            && self.SacksRcvd == other.SacksRcvd
            && self.SackBlocksRcvd == other.SackBlocksRcvd
            && self.CongSignals == other.CongSignals
            && self.PreCongSumCwnd == other.PreCongSumCwnd
            && self.PreCongSumRtt == other.PreCongSumRtt
            && self.PostCongSumRtt == other.PostCongSumRtt
            && self.PostCongCountRtt == other.PostCongCountRtt
            && self.EcnSignals == other.EcnSignals
            && self.EceRcvd == other.EceRcvd
            && self.SendStall == other.SendStall
            && self.QuenchRcvd == other.QuenchRcvd
            && self.RetranThresh == other.RetranThresh
            && self.SndDupAckEpisodes == other.SndDupAckEpisodes
            && self.SumBytesReordered == other.SumBytesReordered
            && self.NonRecovDa == other.NonRecovDa
            && self.NonRecovDaEpisodes == other.NonRecovDaEpisodes
            && self.AckAfterFr == other.AckAfterFr
            && self.DsackDups == other.DsackDups
            && self.SampleRtt == other.SampleRtt
            && self.SmoothedRtt == other.SmoothedRtt
            && self.RttVar == other.RttVar
            && self.MaxRtt == other.MaxRtt
            && self.MinRtt == other.MinRtt
            && self.SumRtt == other.SumRtt
            && self.CountRtt == other.CountRtt
            && self.CurRto == other.CurRto
            && self.MaxRto == other.MaxRto
            && self.MinRto == other.MinRto
            && self.CurMss == other.CurMss
            && self.MaxMss == other.MaxMss
            && self.MinMss == other.MinMss
            && self.SpuriousRtoDetections == other.SpuriousRtoDetections
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_PATH_ROD_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_PATH_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_PATH_ROD_v0")
            .field("FastRetran", &self.FastRetran)
            .field("Timeouts", &self.Timeouts)
            .field("SubsequentTimeouts", &self.SubsequentTimeouts)
            .field("CurTimeoutCount", &self.CurTimeoutCount)
            .field("AbruptTimeouts", &self.AbruptTimeouts)
            .field("PktsRetrans", &self.PktsRetrans)
            .field("BytesRetrans", &self.BytesRetrans)
            .field("DupAcksIn", &self.DupAcksIn)
            .field("SacksRcvd", &self.SacksRcvd)
            .field("SackBlocksRcvd", &self.SackBlocksRcvd)
            .field("CongSignals", &self.CongSignals)
            .field("PreCongSumCwnd", &self.PreCongSumCwnd)
            .field("PreCongSumRtt", &self.PreCongSumRtt)
            .field("PostCongSumRtt", &self.PostCongSumRtt)
            .field("PostCongCountRtt", &self.PostCongCountRtt)
            .field("EcnSignals", &self.EcnSignals)
            .field("EceRcvd", &self.EceRcvd)
            .field("SendStall", &self.SendStall)
            .field("QuenchRcvd", &self.QuenchRcvd)
            .field("RetranThresh", &self.RetranThresh)
            .field("SndDupAckEpisodes", &self.SndDupAckEpisodes)
            .field("SumBytesReordered", &self.SumBytesReordered)
            .field("NonRecovDa", &self.NonRecovDa)
            .field("NonRecovDaEpisodes", &self.NonRecovDaEpisodes)
            .field("AckAfterFr", &self.AckAfterFr)
            .field("DsackDups", &self.DsackDups)
            .field("SampleRtt", &self.SampleRtt)
            .field("SmoothedRtt", &self.SmoothedRtt)
            .field("RttVar", &self.RttVar)
            .field("MaxRtt", &self.MaxRtt)
            .field("MinRtt", &self.MinRtt)
            .field("SumRtt", &self.SumRtt)
            .field("CountRtt", &self.CountRtt)
            .field("CurRto", &self.CurRto)
            .field("MaxRto", &self.MaxRto)
            .field("MinRto", &self.MinRto)
            .field("CurMss", &self.CurMss)
            .field("MaxMss", &self.MaxMss)
            .field("MinMss", &self.MinMss)
            .field("SpuriousRtoDetections", &self.SpuriousRtoDetections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_PATH_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_PATH_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.EnableCollection == other.EnableCollection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_PATH_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_PATH_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_PATH_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_REC_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_REC_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.CurRwinSent == other.CurRwinSent && self.MaxRwinSent == other.MaxRwinSent && self.MinRwinSent == other.MinRwinSent && self.LimRwin == other.LimRwin && self.DupAckEpisodes == other.DupAckEpisodes && self.DupAcksOut == other.DupAcksOut && self.CeRcvd == other.CeRcvd && self.EcnSent == other.EcnSent && self.EcnNoncesRcvd == other.EcnNoncesRcvd && self.CurReasmQueue == other.CurReasmQueue && self.MaxReasmQueue == other.MaxReasmQueue && self.CurAppRQueue == other.CurAppRQueue && self.MaxAppRQueue == other.MaxAppRQueue && self.WinScaleSent == other.WinScaleSent
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_REC_ROD_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_REC_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_REC_ROD_v0")
            .field("CurRwinSent", &self.CurRwinSent)
            .field("MaxRwinSent", &self.MaxRwinSent)
            .field("MinRwinSent", &self.MinRwinSent)
            .field("LimRwin", &self.LimRwin)
            .field("DupAckEpisodes", &self.DupAckEpisodes)
            .field("DupAcksOut", &self.DupAcksOut)
            .field("CeRcvd", &self.CeRcvd)
            .field("EcnSent", &self.EcnSent)
            .field("EcnNoncesRcvd", &self.EcnNoncesRcvd)
            .field("CurReasmQueue", &self.CurReasmQueue)
            .field("MaxReasmQueue", &self.MaxReasmQueue)
            .field("CurAppRQueue", &self.CurAppRQueue)
            .field("MaxAppRQueue", &self.MaxAppRQueue)
            .field("WinScaleSent", &self.WinScaleSent)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_REC_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_REC_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.EnableCollection == other.EnableCollection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_REC_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_REC_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_REC_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.CurRetxQueue == other.CurRetxQueue && self.MaxRetxQueue == other.MaxRetxQueue && self.CurAppWQueue == other.CurAppWQueue && self.MaxAppWQueue == other.MaxAppWQueue
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_SEND_BUFF_ROD_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_SEND_BUFF_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SEND_BUFF_ROD_v0").field("CurRetxQueue", &self.CurRetxQueue).field("MaxRetxQueue", &self.MaxRetxQueue).field("CurAppWQueue", &self.CurAppWQueue).field("MaxAppWQueue", &self.MaxAppWQueue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_SEND_BUFF_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_SEND_BUFF_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.EnableCollection == other.EnableCollection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_SEND_BUFF_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_SEND_BUFF_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SEND_BUFF_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_SND_CONG_ROD_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_SND_CONG_ROD_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.SndLimTransRwin == other.SndLimTransRwin && self.SndLimTimeRwin == other.SndLimTimeRwin && self.SndLimBytesRwin == other.SndLimBytesRwin && self.SndLimTransCwnd == other.SndLimTransCwnd && self.SndLimTimeCwnd == other.SndLimTimeCwnd && self.SndLimBytesCwnd == other.SndLimBytesCwnd && self.SndLimTransSnd == other.SndLimTransSnd && self.SndLimTimeSnd == other.SndLimTimeSnd && self.SndLimBytesSnd == other.SndLimBytesSnd && self.SlowStart == other.SlowStart && self.CongAvoid == other.CongAvoid && self.OtherReductions == other.OtherReductions && self.CurCwnd == other.CurCwnd && self.MaxSsCwnd == other.MaxSsCwnd && self.MaxCaCwnd == other.MaxCaCwnd && self.CurSsthresh == other.CurSsthresh && self.MaxSsthresh == other.MaxSsthresh && self.MinSsthresh == other.MinSsthresh
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_SND_CONG_ROD_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_SND_CONG_ROD_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SND_CONG_ROD_v0")
            .field("SndLimTransRwin", &self.SndLimTransRwin)
            .field("SndLimTimeRwin", &self.SndLimTimeRwin)
            .field("SndLimBytesRwin", &self.SndLimBytesRwin)
            .field("SndLimTransCwnd", &self.SndLimTransCwnd)
            .field("SndLimTimeCwnd", &self.SndLimTimeCwnd)
            .field("SndLimBytesCwnd", &self.SndLimBytesCwnd)
            .field("SndLimTransSnd", &self.SndLimTransSnd)
            .field("SndLimTimeSnd", &self.SndLimTimeSnd)
            .field("SndLimBytesSnd", &self.SndLimBytesSnd)
            .field("SlowStart", &self.SlowStart)
            .field("CongAvoid", &self.CongAvoid)
            .field("OtherReductions", &self.OtherReductions)
            .field("CurCwnd", &self.CurCwnd)
            .field("MaxSsCwnd", &self.MaxSsCwnd)
            .field("MaxCaCwnd", &self.MaxCaCwnd)
            .field("CurSsthresh", &self.CurSsthresh)
            .field("MaxSsthresh", &self.MaxSsthresh)
            .field("MinSsthresh", &self.MinSsthresh)
            .finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_SND_CONG_ROS_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ESTATS_SND_CONG_ROS_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.LimCwnd == other.LimCwnd
    }
}
impl ::core::cmp::Eq for TCP_ESTATS_SND_CONG_ROS_v0 {}
impl ::core::fmt::Debug for TCP_ESTATS_SND_CONG_ROS_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SND_CONG_ROS_v0").field("LimCwnd", &self.LimCwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_SND_CONG_RW_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_SND_CONG_RW_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.EnableCollection == other.EnableCollection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_SND_CONG_RW_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_SND_CONG_RW_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SND_CONG_RW_v0").field("EnableCollection", &self.EnableCollection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.ActiveOpen == other.ActiveOpen && self.MssRcvd == other.MssRcvd && self.MssSent == other.MssSent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_ESTATS_SYN_OPTS_ROS_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_ESTATS_SYN_OPTS_ROS_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ESTATS_SYN_OPTS_ROS_v0").field("ActiveOpen", &self.ActiveOpen).field("MssRcvd", &self.MssRcvd).field("MssSent", &self.MssSent).finish()
    }
}
impl ::core::default::Default for TCP_ESTATS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCP_ESTATS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_ESTATS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TCP_RESERVE_PORT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_RESERVE_PORT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.UpperRange == other.UpperRange && self.LowerRange == other.LowerRange
    }
}
impl ::core::cmp::Eq for TCP_RESERVE_PORT_RANGE {}
impl ::core::fmt::Debug for TCP_RESERVE_PORT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_RESERVE_PORT_RANGE").field("UpperRange", &self.UpperRange).field("LowerRange", &self.LowerRange).finish()
    }
}
impl ::core::default::Default for TCP_RTO_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCP_RTO_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_RTO_ALGORITHM").field(&self.0).finish()
    }
}
impl ::core::default::Default for TCP_SOFT_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCP_SOFT_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_SOFT_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for TCP_TABLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCP_TABLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_TABLE_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDP_TABLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDP_TABLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDP_TABLE_CLASS").field(&self.0).finish()
    }
}
