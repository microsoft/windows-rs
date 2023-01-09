impl ::core::default::Default for AAL5_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AAL5_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ForwardMaxCPCSSDUSize == other.ForwardMaxCPCSSDUSize && self.BackwardMaxCPCSSDUSize == other.BackwardMaxCPCSSDUSize && self.Mode == other.Mode && self.SSCSType == other.SSCSType
    }
}
impl ::core::cmp::Eq for AAL5_PARAMETERS {}
impl ::core::fmt::Debug for AAL5_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AAL5_PARAMETERS").field("ForwardMaxCPCSSDUSize", &self.ForwardMaxCPCSSDUSize).field("BackwardMaxCPCSSDUSize", &self.BackwardMaxCPCSSDUSize).field("Mode", &self.Mode).field("SSCSType", &self.SSCSType).finish()
    }
}
impl ::core::default::Default for AALUSER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AALUSER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.UserDefined == other.UserDefined
    }
}
impl ::core::cmp::Eq for AALUSER_PARAMETERS {}
impl ::core::fmt::Debug for AALUSER_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AALUSER_PARAMETERS").field("UserDefined", &self.UserDefined).finish()
    }
}
impl ::core::default::Default for AAL_PARAMETERS_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AAL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADDRESS_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADDRESS_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDRESS_FAMILY").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_next == other.ai_next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOA").field("ai_flags", &self.ai_flags).field("ai_family", &self.ai_family).field("ai_socktype", &self.ai_socktype).field("ai_protocol", &self.ai_protocol).field("ai_addrlen", &self.ai_addrlen).field("ai_canonname", &self.ai_canonname).field("ai_addr", &self.ai_addr).field("ai_next", &self.ai_next).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOEX2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOEX2A {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next && self.ai_version == other.ai_version && self.ai_fqdn == other.ai_fqdn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOEX2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOEX2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOEX2A")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOEX2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOEX2W {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next && self.ai_version == other.ai_version && self.ai_fqdn == other.ai_fqdn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOEX2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOEX2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOEX2W")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOEX3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOEX3 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next && self.ai_version == other.ai_version && self.ai_fqdn == other.ai_fqdn && self.ai_interfaceindex == other.ai_interfaceindex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOEX3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOEX3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOEX3")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOEX4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOEX4 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next && self.ai_version == other.ai_version && self.ai_fqdn == other.ai_fqdn && self.ai_interfaceindex == other.ai_interfaceindex && self.ai_resolutionhandle == other.ai_resolutionhandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOEX4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOEX4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOEX4")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOEX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOEX5 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next && self.ai_version == other.ai_version && self.ai_fqdn == other.ai_fqdn && self.ai_interfaceindex == other.ai_interfaceindex && self.ai_resolutionhandle == other.ai_resolutionhandle && self.ai_ttl == other.ai_ttl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOEX5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOEX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOEX5")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .field("ai_ttl", &self.ai_ttl)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOEX6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOEX6 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next && self.ai_version == other.ai_version && self.ai_fqdn == other.ai_fqdn && self.ai_interfaceindex == other.ai_interfaceindex && self.ai_resolutionhandle == other.ai_resolutionhandle && self.ai_ttl == other.ai_ttl && self.ai_numservers == other.ai_numservers && self.ai_servers == other.ai_servers && self.ai_responseflags == other.ai_responseflags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOEX6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOEX6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOEX6")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .field("ai_ttl", &self.ai_ttl)
            .field("ai_numservers", &self.ai_numservers)
            .field("ai_servers", &self.ai_servers)
            .field("ai_responseflags", &self.ai_responseflags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOEXA").field("ai_flags", &self.ai_flags).field("ai_family", &self.ai_family).field("ai_socktype", &self.ai_socktype).field("ai_protocol", &self.ai_protocol).field("ai_addrlen", &self.ai_addrlen).field("ai_canonname", &self.ai_canonname).field("ai_addr", &self.ai_addr).field("ai_blob", &self.ai_blob).field("ai_bloblen", &self.ai_bloblen).field("ai_provider", &self.ai_provider).field("ai_next", &self.ai_next).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_blob == other.ai_blob && self.ai_bloblen == other.ai_bloblen && self.ai_provider == other.ai_provider && self.ai_next == other.ai_next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOEXW").field("ai_flags", &self.ai_flags).field("ai_family", &self.ai_family).field("ai_socktype", &self.ai_socktype).field("ai_protocol", &self.ai_protocol).field("ai_addrlen", &self.ai_addrlen).field("ai_canonname", &self.ai_canonname).field("ai_addr", &self.ai_addr).field("ai_blob", &self.ai_blob).field("ai_bloblen", &self.ai_bloblen).field("ai_provider", &self.ai_provider).field("ai_next", &self.ai_next).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags && self.ai_family == other.ai_family && self.ai_socktype == other.ai_socktype && self.ai_protocol == other.ai_protocol && self.ai_addrlen == other.ai_addrlen && self.ai_canonname == other.ai_canonname && self.ai_addr == other.ai_addr && self.ai_next == other.ai_next
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOW").field("ai_flags", &self.ai_flags).field("ai_family", &self.ai_family).field("ai_socktype", &self.ai_socktype).field("ai_protocol", &self.ai_protocol).field("ai_addrlen", &self.ai_addrlen).field("ai_canonname", &self.ai_canonname).field("ai_addr", &self.ai_addr).field("ai_next", &self.ai_next).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRINFO_DNS_SERVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AFPROTOCOLS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AFPROTOCOLS {
    fn eq(&self, other: &Self) -> bool {
        self.iAddressFamily == other.iAddressFamily && self.iProtocol == other.iProtocol
    }
}
impl ::core::cmp::Eq for AFPROTOCOLS {}
impl ::core::fmt::Debug for AFPROTOCOLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AFPROTOCOLS").field("iAddressFamily", &self.iAddressFamily).field("iProtocol", &self.iProtocol).finish()
    }
}
impl ::core::default::Default for ARP_HARDWARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ARP_HARDWARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARP_HARDWARE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ARP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ARP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.HardwareAddressSpace == other.HardwareAddressSpace && self.ProtocolAddressSpace == other.ProtocolAddressSpace && self.HardwareAddressLength == other.HardwareAddressLength && self.ProtocolAddressLength == other.ProtocolAddressLength && self.Opcode == other.Opcode && self.SenderHardwareAddress == other.SenderHardwareAddress
    }
}
impl ::core::cmp::Eq for ARP_HEADER {}
impl ::core::fmt::Debug for ARP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ARP_HEADER").field("HardwareAddressSpace", &self.HardwareAddressSpace).field("ProtocolAddressSpace", &self.ProtocolAddressSpace).field("HardwareAddressLength", &self.HardwareAddressLength).field("ProtocolAddressLength", &self.ProtocolAddressLength).field("Opcode", &self.Opcode).field("SenderHardwareAddress", &self.SenderHardwareAddress).finish()
    }
}
impl ::core::default::Default for ARP_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ARP_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARP_OPCODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.Handle == other.Handle
    }
}
impl ::core::cmp::Eq for ASSOCIATE_NAMERES_CONTEXT_INPUT {}
impl ::core::fmt::Debug for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASSOCIATE_NAMERES_CONTEXT_INPUT").field("TransportSettingId", &self.TransportSettingId).field("Handle", &self.Handle).finish()
    }
}
impl ::core::default::Default for ATM_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressType == other.AddressType && self.NumofDigits == other.NumofDigits && self.Addr == other.Addr
    }
}
impl ::core::cmp::Eq for ATM_ADDRESS {}
impl ::core::fmt::Debug for ATM_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_ADDRESS").field("AddressType", &self.AddressType).field("NumofDigits", &self.NumofDigits).field("Addr", &self.Addr).finish()
    }
}
impl ::core::default::Default for ATM_BHLI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_BHLI {
    fn eq(&self, other: &Self) -> bool {
        self.HighLayerInfoType == other.HighLayerInfoType && self.HighLayerInfoLength == other.HighLayerInfoLength && self.HighLayerInfo == other.HighLayerInfo
    }
}
impl ::core::cmp::Eq for ATM_BHLI {}
impl ::core::fmt::Debug for ATM_BHLI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_BHLI").field("HighLayerInfoType", &self.HighLayerInfoType).field("HighLayerInfoLength", &self.HighLayerInfoLength).field("HighLayerInfo", &self.HighLayerInfo).finish()
    }
}
impl ::core::default::Default for ATM_BLLI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_BLLI {
    fn eq(&self, other: &Self) -> bool {
        self.Layer2Protocol == other.Layer2Protocol && self.Layer2UserSpecifiedProtocol == other.Layer2UserSpecifiedProtocol && self.Layer3Protocol == other.Layer3Protocol && self.Layer3UserSpecifiedProtocol == other.Layer3UserSpecifiedProtocol && self.Layer3IPI == other.Layer3IPI && self.SnapID == other.SnapID
    }
}
impl ::core::cmp::Eq for ATM_BLLI {}
impl ::core::fmt::Debug for ATM_BLLI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_BLLI").field("Layer2Protocol", &self.Layer2Protocol).field("Layer2UserSpecifiedProtocol", &self.Layer2UserSpecifiedProtocol).field("Layer3Protocol", &self.Layer3Protocol).field("Layer3UserSpecifiedProtocol", &self.Layer3UserSpecifiedProtocol).field("Layer3IPI", &self.Layer3IPI).field("SnapID", &self.SnapID).finish()
    }
}
impl ::core::default::Default for ATM_BLLI_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_BLLI_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Layer2Protocol == other.Layer2Protocol && self.Layer2Mode == other.Layer2Mode && self.Layer2WindowSize == other.Layer2WindowSize && self.Layer2UserSpecifiedProtocol == other.Layer2UserSpecifiedProtocol && self.Layer3Protocol == other.Layer3Protocol && self.Layer3Mode == other.Layer3Mode && self.Layer3DefaultPacketSize == other.Layer3DefaultPacketSize && self.Layer3PacketWindowSize == other.Layer3PacketWindowSize && self.Layer3UserSpecifiedProtocol == other.Layer3UserSpecifiedProtocol && self.Layer3IPI == other.Layer3IPI && self.SnapID == other.SnapID
    }
}
impl ::core::cmp::Eq for ATM_BLLI_IE {}
impl ::core::fmt::Debug for ATM_BLLI_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_BLLI_IE")
            .field("Layer2Protocol", &self.Layer2Protocol)
            .field("Layer2Mode", &self.Layer2Mode)
            .field("Layer2WindowSize", &self.Layer2WindowSize)
            .field("Layer2UserSpecifiedProtocol", &self.Layer2UserSpecifiedProtocol)
            .field("Layer3Protocol", &self.Layer3Protocol)
            .field("Layer3Mode", &self.Layer3Mode)
            .field("Layer3DefaultPacketSize", &self.Layer3DefaultPacketSize)
            .field("Layer3PacketWindowSize", &self.Layer3PacketWindowSize)
            .field("Layer3UserSpecifiedProtocol", &self.Layer3UserSpecifiedProtocol)
            .field("Layer3IPI", &self.Layer3IPI)
            .field("SnapID", &self.SnapID)
            .finish()
    }
}
impl ::core::default::Default for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn eq(&self, other: &Self) -> bool {
        self.BearerClass == other.BearerClass && self.TrafficType == other.TrafficType && self.TimingRequirements == other.TimingRequirements && self.ClippingSusceptability == other.ClippingSusceptability && self.UserPlaneConnectionConfig == other.UserPlaneConnectionConfig
    }
}
impl ::core::cmp::Eq for ATM_BROADBAND_BEARER_CAPABILITY_IE {}
impl ::core::fmt::Debug for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_BROADBAND_BEARER_CAPABILITY_IE").field("BearerClass", &self.BearerClass).field("TrafficType", &self.TrafficType).field("TimingRequirements", &self.TimingRequirements).field("ClippingSusceptability", &self.ClippingSusceptability).field("UserPlaneConnectionConfig", &self.UserPlaneConnectionConfig).finish()
    }
}
impl ::core::default::Default for ATM_CALLING_PARTY_NUMBER_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_CALLING_PARTY_NUMBER_IE {
    fn eq(&self, other: &Self) -> bool {
        self.ATM_Number == other.ATM_Number && self.Presentation_Indication == other.Presentation_Indication && self.Screening_Indicator == other.Screening_Indicator
    }
}
impl ::core::cmp::Eq for ATM_CALLING_PARTY_NUMBER_IE {}
impl ::core::fmt::Debug for ATM_CALLING_PARTY_NUMBER_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_CALLING_PARTY_NUMBER_IE").field("ATM_Number", &self.ATM_Number).field("Presentation_Indication", &self.Presentation_Indication).field("Screening_Indicator", &self.Screening_Indicator).finish()
    }
}
impl ::core::default::Default for ATM_CAUSE_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_CAUSE_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location && self.Cause == other.Cause && self.DiagnosticsLength == other.DiagnosticsLength && self.Diagnostics == other.Diagnostics
    }
}
impl ::core::cmp::Eq for ATM_CAUSE_IE {}
impl ::core::fmt::Debug for ATM_CAUSE_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_CAUSE_IE").field("Location", &self.Location).field("Cause", &self.Cause).field("DiagnosticsLength", &self.DiagnosticsLength).field("Diagnostics", &self.Diagnostics).finish()
    }
}
impl ::core::default::Default for ATM_CONNECTION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_CONNECTION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceNumber == other.DeviceNumber && self.VPI == other.VPI && self.VCI == other.VCI
    }
}
impl ::core::cmp::Eq for ATM_CONNECTION_ID {}
impl ::core::fmt::Debug for ATM_CONNECTION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_CONNECTION_ID").field("DeviceNumber", &self.DeviceNumber).field("VPI", &self.VPI).field("VCI", &self.VCI).finish()
    }
}
impl ::core::default::Default for ATM_PVC_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ATM_QOS_CLASS_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_QOS_CLASS_IE {
    fn eq(&self, other: &Self) -> bool {
        self.QOSClassForward == other.QOSClassForward && self.QOSClassBackward == other.QOSClassBackward
    }
}
impl ::core::cmp::Eq for ATM_QOS_CLASS_IE {}
impl ::core::fmt::Debug for ATM_QOS_CLASS_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_QOS_CLASS_IE").field("QOSClassForward", &self.QOSClassForward).field("QOSClassBackward", &self.QOSClassBackward).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ATM_TD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ATM_TD {
    fn eq(&self, other: &Self) -> bool {
        self.PeakCellRate_CLP0 == other.PeakCellRate_CLP0 && self.PeakCellRate_CLP01 == other.PeakCellRate_CLP01 && self.SustainableCellRate_CLP0 == other.SustainableCellRate_CLP0 && self.SustainableCellRate_CLP01 == other.SustainableCellRate_CLP01 && self.MaxBurstSize_CLP0 == other.MaxBurstSize_CLP0 && self.MaxBurstSize_CLP01 == other.MaxBurstSize_CLP01 && self.Tagging == other.Tagging
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ATM_TD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ATM_TD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_TD").field("PeakCellRate_CLP0", &self.PeakCellRate_CLP0).field("PeakCellRate_CLP01", &self.PeakCellRate_CLP01).field("SustainableCellRate_CLP0", &self.SustainableCellRate_CLP0).field("SustainableCellRate_CLP01", &self.SustainableCellRate_CLP01).field("MaxBurstSize_CLP0", &self.MaxBurstSize_CLP0).field("MaxBurstSize_CLP01", &self.MaxBurstSize_CLP01).field("Tagging", &self.Tagging).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Forward == other.Forward && self.Backward == other.Backward && self.BestEffort == other.BestEffort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ATM_TRAFFIC_DESCRIPTOR_IE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_TRAFFIC_DESCRIPTOR_IE").field("Forward", &self.Forward).field("Backward", &self.Backward).field("BestEffort", &self.BestEffort).finish()
    }
}
impl ::core::default::Default for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn eq(&self, other: &Self) -> bool {
        self.TypeOfNetworkId == other.TypeOfNetworkId && self.NetworkIdPlan == other.NetworkIdPlan && self.NetworkIdLength == other.NetworkIdLength && self.NetworkId == other.NetworkId
    }
}
impl ::core::cmp::Eq for ATM_TRANSIT_NETWORK_SELECTION_IE {}
impl ::core::fmt::Debug for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_TRANSIT_NETWORK_SELECTION_IE").field("TypeOfNetworkId", &self.TypeOfNetworkId).field("NetworkIdPlan", &self.NetworkIdPlan).field("NetworkIdLength", &self.NetworkIdLength).field("NetworkId", &self.NetworkId).finish()
    }
}
impl ::core::default::Default for CMSGHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMSGHDR {
    fn eq(&self, other: &Self) -> bool {
        self.cmsg_len == other.cmsg_len && self.cmsg_level == other.cmsg_level && self.cmsg_type == other.cmsg_type
    }
}
impl ::core::cmp::Eq for CMSGHDR {}
impl ::core::fmt::Debug for CMSGHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMSGHDR").field("cmsg_len", &self.cmsg_len).field("cmsg_level", &self.cmsg_level).field("cmsg_type", &self.cmsg_type).finish()
    }
}
impl ::core::default::Default for CONTROL_CHANNEL_TRIGGER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTROL_CHANNEL_TRIGGER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTROL_CHANNEL_TRIGGER_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CSADDR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CSADDR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LocalAddr == other.LocalAddr && self.RemoteAddr == other.RemoteAddr && self.iSocketType == other.iSocketType && self.iProtocol == other.iProtocol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CSADDR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CSADDR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSADDR_INFO").field("LocalAddr", &self.LocalAddr).field("RemoteAddr", &self.RemoteAddr).field("iSocketType", &self.iSocketType).field("iProtocol", &self.iProtocol).finish()
    }
}
impl ::core::default::Default for DL_EI48 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DL_EI64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DL_EUI48 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DL_EUI64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DL_OUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DL_TEREDO_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DL_TEREDO_ADDRESS_PRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for DL_TUNNEL_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ETHERNET_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FALLBACK_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FALLBACK_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FALLBACK_INDEX").field(&self.0).finish()
    }
}
impl ::core::default::Default for FD_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FD_SET {
    fn eq(&self, other: &Self) -> bool {
        self.fd_count == other.fd_count && self.fd_array == other.fd_array
    }
}
impl ::core::cmp::Eq for FD_SET {}
impl ::core::fmt::Debug for FD_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FD_SET").field("fd_count", &self.fd_count).field("fd_array", &self.fd_array).finish()
    }
}
impl ::core::default::Default for FLOWSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FLOWSPEC {
    fn eq(&self, other: &Self) -> bool {
        self.TokenRate == other.TokenRate && self.TokenBucketSize == other.TokenBucketSize && self.PeakBandwidth == other.PeakBandwidth && self.Latency == other.Latency && self.DelayVariation == other.DelayVariation && self.ServiceType == other.ServiceType && self.MaxSduSize == other.MaxSduSize && self.MinimumPolicedSize == other.MinimumPolicedSize
    }
}
impl ::core::cmp::Eq for FLOWSPEC {}
impl ::core::fmt::Debug for FLOWSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOWSPEC").field("TokenRate", &self.TokenRate).field("TokenBucketSize", &self.TokenBucketSize).field("PeakBandwidth", &self.PeakBandwidth).field("Latency", &self.Latency).field("DelayVariation", &self.DelayVariation).field("ServiceType", &self.ServiceType).field("MaxSduSize", &self.MaxSduSize).field("MinimumPolicedSize", &self.MinimumPolicedSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.gf_interface == other.gf_interface && self.gf_group == other.gf_group && self.gf_fmode == other.gf_fmode && self.gf_numsrc == other.gf_numsrc && self.gf_slist == other.gf_slist
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_FILTER").field("gf_interface", &self.gf_interface).field("gf_group", &self.gf_group).field("gf_fmode", &self.gf_fmode).field("gf_numsrc", &self.gf_numsrc).field("gf_slist", &self.gf_slist).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_REQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_REQ {
    fn eq(&self, other: &Self) -> bool {
        self.gr_interface == other.gr_interface && self.gr_group == other.gr_group
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_REQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_REQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_REQ").field("gr_interface", &self.gr_interface).field("gr_group", &self.gr_group).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_SOURCE_REQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_SOURCE_REQ {
    fn eq(&self, other: &Self) -> bool {
        self.gsr_interface == other.gsr_interface && self.gsr_group == other.gsr_group && self.gsr_source == other.gsr_source
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_SOURCE_REQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_SOURCE_REQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_SOURCE_REQ").field("gsr_interface", &self.gsr_interface).field("gsr_group", &self.gsr_group).field("gsr_source", &self.gsr_source).finish()
    }
}
impl ::core::default::Default for HOSTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HOSTENT {
    fn eq(&self, other: &Self) -> bool {
        self.h_name == other.h_name && self.h_aliases == other.h_aliases && self.h_addrtype == other.h_addrtype && self.h_length == other.h_length && self.h_addr_list == other.h_addr_list
    }
}
impl ::core::cmp::Eq for HOSTENT {}
impl ::core::fmt::Debug for HOSTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HOSTENT").field("h_name", &self.h_name).field("h_aliases", &self.h_aliases).field("h_addrtype", &self.h_addrtype).field("h_length", &self.h_length).field("h_addr_list", &self.h_addr_list).finish()
    }
}
impl ::core::default::Default for ICMP4_TIME_EXCEED_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ICMP4_TIME_EXCEED_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICMP4_TIME_EXCEED_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ICMP4_UNREACH_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ICMP4_UNREACH_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICMP4_UNREACH_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ICMPV4_ADDRESS_MASK_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ICMPV4_ROUTER_ADVERT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ICMPV4_ROUTER_ADVERT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ICMPV4_ROUTER_SOLICIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ICMPV4_TIMESTAMP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ICMP_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ICMP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ICMP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Code == other.Code && self.Checksum == other.Checksum
    }
}
impl ::core::cmp::Eq for ICMP_HEADER {}
impl ::core::fmt::Debug for ICMP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICMP_HEADER").field("Type", &self.Type).field("Code", &self.Code).field("Checksum", &self.Checksum).finish()
    }
}
impl ::core::default::Default for ICMP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IGMPV3_QUERY_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IGMPV3_REPORT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IGMPV3_REPORT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved == other.Reserved && self.Checksum == other.Checksum && self.Reserved2 == other.Reserved2 && self.RecordCount == other.RecordCount
    }
}
impl ::core::cmp::Eq for IGMPV3_REPORT_HEADER {}
impl ::core::fmt::Debug for IGMPV3_REPORT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IGMPV3_REPORT_HEADER").field("Type", &self.Type).field("Reserved", &self.Reserved).field("Checksum", &self.Checksum).field("Reserved2", &self.Reserved2).field("RecordCount", &self.RecordCount).finish()
    }
}
impl ::core::default::Default for IGMPV3_REPORT_RECORD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IGMP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IGMP_MAX_RESP_CODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IGMP_MAX_RESP_CODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGMP_MAX_RESP_CODE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IN6_ADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IN6_PKTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IN6_PKTINFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for INET_PORT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INET_PORT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartPort == other.StartPort && self.NumberOfPorts == other.NumberOfPorts
    }
}
impl ::core::cmp::Eq for INET_PORT_RANGE {}
impl ::core::fmt::Debug for INET_PORT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_PORT_RANGE").field("StartPort", &self.StartPort).field("NumberOfPorts", &self.NumberOfPorts).finish()
    }
}
impl ::core::default::Default for INET_PORT_RESERVATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INET_PORT_RESERVATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.OwningPid == other.OwningPid
    }
}
impl ::core::cmp::Eq for INET_PORT_RESERVATION_INFORMATION {}
impl ::core::fmt::Debug for INET_PORT_RESERVATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_PORT_RESERVATION_INFORMATION").field("OwningPid", &self.OwningPid).finish()
    }
}
impl ::core::default::Default for INET_PORT_RESERVATION_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INET_PORT_RESERVATION_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.Reservation == other.Reservation && self.Token == other.Token
    }
}
impl ::core::cmp::Eq for INET_PORT_RESERVATION_INSTANCE {}
impl ::core::fmt::Debug for INET_PORT_RESERVATION_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_PORT_RESERVATION_INSTANCE").field("Reservation", &self.Reservation).field("Token", &self.Token).finish()
    }
}
impl ::core::default::Default for INET_PORT_RESERVATION_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INET_PORT_RESERVATION_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token
    }
}
impl ::core::cmp::Eq for INET_PORT_RESERVATION_TOKEN {}
impl ::core::fmt::Debug for INET_PORT_RESERVATION_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_PORT_RESERVATION_TOKEN").field("Token", &self.Token).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERFACE_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERFACE_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.iiFlags == other.iiFlags && self.iiAddress == other.iiAddress && self.iiBroadcastAddress == other.iiBroadcastAddress && self.iiNetmask == other.iiNetmask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERFACE_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERFACE_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_INFO_EX").field("iiFlags", &self.iiFlags).field("iiAddress", &self.iiAddress).field("iiBroadcastAddress", &self.iiBroadcastAddress).field("iiNetmask", &self.iiNetmask).finish()
    }
}
impl ::core::default::Default for IN_ADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IN_PKTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IN_PKTINFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IN_RECVERR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IN_RECVERR {
    fn eq(&self, other: &Self) -> bool {
        self.protocol == other.protocol && self.info == other.info && self.r#type == other.r#type && self.code == other.code
    }
}
impl ::core::cmp::Eq for IN_RECVERR {}
impl ::core::fmt::Debug for IN_RECVERR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_RECVERR").field("protocol", &self.protocol).field("info", &self.info).field("type", &self.r#type).field("code", &self.code).finish()
    }
}
impl ::core::default::Default for IPPROTO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPPROTO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPPROTO").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPTLS_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV4_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV4_OPTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV4_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPV4_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPV4_OPTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPV4_ROUTING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV4_TIMESTAMP_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV6_EXTENSION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPV6_EXTENSION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.NextHeader == other.NextHeader && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for IPV6_EXTENSION_HEADER {}
impl ::core::fmt::Debug for IPV6_EXTENSION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_EXTENSION_HEADER").field("NextHeader", &self.NextHeader).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for IPV6_FRAGMENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV6_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV6_MREQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV6_OPTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPV6_OPTION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataLength == other.DataLength
    }
}
impl ::core::cmp::Eq for IPV6_OPTION_HEADER {}
impl ::core::fmt::Debug for IPV6_OPTION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_OPTION_HEADER").field("Type", &self.Type).field("DataLength", &self.DataLength).finish()
    }
}
impl ::core::default::Default for IPV6_OPTION_JUMBOGRAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPV6_OPTION_JUMBOGRAM {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.JumbogramLength == other.JumbogramLength
    }
}
impl ::core::cmp::Eq for IPV6_OPTION_JUMBOGRAM {}
impl ::core::fmt::Debug for IPV6_OPTION_JUMBOGRAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_OPTION_JUMBOGRAM").field("Header", &self.Header).field("JumbogramLength", &self.JumbogramLength).finish()
    }
}
impl ::core::default::Default for IPV6_OPTION_ROUTER_ALERT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPV6_OPTION_ROUTER_ALERT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for IPV6_OPTION_ROUTER_ALERT {}
impl ::core::fmt::Debug for IPV6_OPTION_ROUTER_ALERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_OPTION_ROUTER_ALERT").field("Header", &self.Header).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for IPV6_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPV6_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPV6_OPTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPV6_ROUTING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPV6_ROUTING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.NextHeader == other.NextHeader && self.Length == other.Length && self.RoutingType == other.RoutingType && self.SegmentsLeft == other.SegmentsLeft && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for IPV6_ROUTING_HEADER {}
impl ::core::fmt::Debug for IPV6_ROUTING_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_ROUTING_HEADER").field("NextHeader", &self.NextHeader).field("Length", &self.Length).field("RoutingType", &self.RoutingType).field("SegmentsLeft", &self.SegmentsLeft).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IPX_ADDRESS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IPX_ADDRESS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.adapternum == other.adapternum && self.netnum == other.netnum && self.nodenum == other.nodenum && self.wan == other.wan && self.status == other.status && self.maxpkt == other.maxpkt && self.linkspeed == other.linkspeed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IPX_ADDRESS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IPX_ADDRESS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_ADDRESS_DATA").field("adapternum", &self.adapternum).field("netnum", &self.netnum).field("nodenum", &self.nodenum).field("wan", &self.wan).field("status", &self.status).field("maxpkt", &self.maxpkt).field("linkspeed", &self.linkspeed).finish()
    }
}
impl ::core::default::Default for IPX_NETNUM_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPX_NETNUM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.netnum == other.netnum && self.hopcount == other.hopcount && self.netdelay == other.netdelay && self.cardnum == other.cardnum && self.router == other.router
    }
}
impl ::core::cmp::Eq for IPX_NETNUM_DATA {}
impl ::core::fmt::Debug for IPX_NETNUM_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_NETNUM_DATA").field("netnum", &self.netnum).field("hopcount", &self.hopcount).field("netdelay", &self.netdelay).field("cardnum", &self.cardnum).field("router", &self.router).finish()
    }
}
impl ::core::default::Default for IPX_SPXCONNSTATUS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPX_SPXCONNSTATUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionState == other.ConnectionState
            && self.WatchDogActive == other.WatchDogActive
            && self.LocalConnectionId == other.LocalConnectionId
            && self.RemoteConnectionId == other.RemoteConnectionId
            && self.LocalSequenceNumber == other.LocalSequenceNumber
            && self.LocalAckNumber == other.LocalAckNumber
            && self.LocalAllocNumber == other.LocalAllocNumber
            && self.RemoteAckNumber == other.RemoteAckNumber
            && self.RemoteAllocNumber == other.RemoteAllocNumber
            && self.LocalSocket == other.LocalSocket
            && self.ImmediateAddress == other.ImmediateAddress
            && self.RemoteNetwork == other.RemoteNetwork
            && self.RemoteNode == other.RemoteNode
            && self.RemoteSocket == other.RemoteSocket
            && self.RetransmissionCount == other.RetransmissionCount
            && self.EstimatedRoundTripDelay == other.EstimatedRoundTripDelay
            && self.RetransmittedPackets == other.RetransmittedPackets
            && self.SuppressedPacket == other.SuppressedPacket
    }
}
impl ::core::cmp::Eq for IPX_SPXCONNSTATUS_DATA {}
impl ::core::fmt::Debug for IPX_SPXCONNSTATUS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_SPXCONNSTATUS_DATA")
            .field("ConnectionState", &self.ConnectionState)
            .field("WatchDogActive", &self.WatchDogActive)
            .field("LocalConnectionId", &self.LocalConnectionId)
            .field("RemoteConnectionId", &self.RemoteConnectionId)
            .field("LocalSequenceNumber", &self.LocalSequenceNumber)
            .field("LocalAckNumber", &self.LocalAckNumber)
            .field("LocalAllocNumber", &self.LocalAllocNumber)
            .field("RemoteAckNumber", &self.RemoteAckNumber)
            .field("RemoteAllocNumber", &self.RemoteAllocNumber)
            .field("LocalSocket", &self.LocalSocket)
            .field("ImmediateAddress", &self.ImmediateAddress)
            .field("RemoteNetwork", &self.RemoteNetwork)
            .field("RemoteNode", &self.RemoteNode)
            .field("RemoteSocket", &self.RemoteSocket)
            .field("RetransmissionCount", &self.RetransmissionCount)
            .field("EstimatedRoundTripDelay", &self.EstimatedRoundTripDelay)
            .field("RetransmittedPackets", &self.RetransmittedPackets)
            .field("SuppressedPacket", &self.SuppressedPacket)
            .finish()
    }
}
impl ::core::default::Default for IP_MREQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IP_MREQ_SOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IP_MSFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IP_OPTION_TIMESTAMP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IP_OPTION_TIMESTAMP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IP_OPTION_TIMESTAMP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for LINGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LINGER {
    fn eq(&self, other: &Self) -> bool {
        self.l_onoff == other.l_onoff && self.l_linger == other.l_linger
    }
}
impl ::core::cmp::Eq for LINGER {}
impl ::core::fmt::Debug for LINGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LINGER").field("l_onoff", &self.l_onoff).field("l_linger", &self.l_linger).finish()
    }
}
impl ::core::default::Default for LM_IRPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LM_IRPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.nTXDataBytes == other.nTXDataBytes && self.nRXDataBytes == other.nRXDataBytes && self.nBaudRate == other.nBaudRate && self.thresholdTime == other.thresholdTime && self.discTime == other.discTime && self.nMSLinkTurn == other.nMSLinkTurn && self.nTXPackets == other.nTXPackets && self.nRXPackets == other.nRXPackets
    }
}
impl ::core::cmp::Eq for LM_IRPARMS {}
impl ::core::fmt::Debug for LM_IRPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LM_IRPARMS").field("nTXDataBytes", &self.nTXDataBytes).field("nRXDataBytes", &self.nRXDataBytes).field("nBaudRate", &self.nBaudRate).field("thresholdTime", &self.thresholdTime).field("discTime", &self.discTime).field("nMSLinkTurn", &self.nMSLinkTurn).field("nTXPackets", &self.nTXPackets).field("nRXPackets", &self.nRXPackets).finish()
    }
}
impl ::core::default::Default for MLDV2_QUERY_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MLDV2_REPORT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MLDV2_REPORT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.IcmpHeader == other.IcmpHeader && self.Reserved == other.Reserved && self.RecordCount == other.RecordCount
    }
}
impl ::core::cmp::Eq for MLDV2_REPORT_HEADER {}
impl ::core::fmt::Debug for MLDV2_REPORT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLDV2_REPORT_HEADER").field("IcmpHeader", &self.IcmpHeader).field("Reserved", &self.Reserved).field("RecordCount", &self.RecordCount).finish()
    }
}
impl ::core::default::Default for MLDV2_REPORT_RECORD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MLD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MLD_MAX_RESP_CODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLD_MAX_RESP_CODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLD_MAX_RESP_CODE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MULTICAST_MODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MULTICAST_MODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MULTICAST_MODE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.AuthLevel == other.AuthLevel && self.cchDomainName == other.cchDomainName && self.OffsetNextDomainDescription == other.OffsetNextDomainDescription && self.OffsetThisDomainName == other.OffsetThisDomainName
    }
}
impl ::core::cmp::Eq for NAPI_DOMAIN_DESCRIPTION_BLOB {}
impl ::core::fmt::Debug for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAPI_DOMAIN_DESCRIPTION_BLOB").field("AuthLevel", &self.AuthLevel).field("cchDomainName", &self.cchDomainName).field("OffsetNextDomainDescription", &self.OffsetNextDomainDescription).field("OffsetThisDomainName", &self.OffsetThisDomainName).finish()
    }
}
impl ::core::default::Default for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwProviderType == other.dwProviderType && self.fSupportsWildCard == other.fSupportsWildCard && self.cDomains == other.cDomains && self.OffsetFirstDomain == other.OffsetFirstDomain
    }
}
impl ::core::cmp::Eq for NAPI_PROVIDER_INSTALLATION_BLOB {}
impl ::core::fmt::Debug for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAPI_PROVIDER_INSTALLATION_BLOB").field("dwVersion", &self.dwVersion).field("dwProviderType", &self.dwProviderType).field("fSupportsWildCard", &self.fSupportsWildCard).field("cDomains", &self.cDomains).field("OffsetFirstDomain", &self.OffsetFirstDomain).finish()
    }
}
impl ::core::default::Default for NAPI_PROVIDER_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAPI_PROVIDER_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAPI_PROVIDER_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAPI_PROVIDER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAPI_PROVIDER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAPI_PROVIDER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ND_NEIGHBOR_ADVERT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ND_NEIGHBOR_SOLICIT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ND_OPTION_DNSSL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ND_OPTION_DNSSL {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_dnssl_type == other.nd_opt_dnssl_type && self.nd_opt_dnssl_len == other.nd_opt_dnssl_len && self.nd_opt_dnssl_reserved == other.nd_opt_dnssl_reserved && self.nd_opt_dnssl_lifetime == other.nd_opt_dnssl_lifetime
    }
}
impl ::core::cmp::Eq for ND_OPTION_DNSSL {}
impl ::core::fmt::Debug for ND_OPTION_DNSSL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ND_OPTION_DNSSL").field("nd_opt_dnssl_type", &self.nd_opt_dnssl_type).field("nd_opt_dnssl_len", &self.nd_opt_dnssl_len).field("nd_opt_dnssl_reserved", &self.nd_opt_dnssl_reserved).field("nd_opt_dnssl_lifetime", &self.nd_opt_dnssl_lifetime).finish()
    }
}
impl ::core::default::Default for ND_OPTION_HDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ND_OPTION_HDR {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_type == other.nd_opt_type && self.nd_opt_len == other.nd_opt_len
    }
}
impl ::core::cmp::Eq for ND_OPTION_HDR {}
impl ::core::fmt::Debug for ND_OPTION_HDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ND_OPTION_HDR").field("nd_opt_type", &self.nd_opt_type).field("nd_opt_len", &self.nd_opt_len).finish()
    }
}
impl ::core::default::Default for ND_OPTION_MTU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ND_OPTION_MTU {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_mtu_type == other.nd_opt_mtu_type && self.nd_opt_mtu_len == other.nd_opt_mtu_len && self.nd_opt_mtu_reserved == other.nd_opt_mtu_reserved && self.nd_opt_mtu_mtu == other.nd_opt_mtu_mtu
    }
}
impl ::core::cmp::Eq for ND_OPTION_MTU {}
impl ::core::fmt::Debug for ND_OPTION_MTU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ND_OPTION_MTU").field("nd_opt_mtu_type", &self.nd_opt_mtu_type).field("nd_opt_mtu_len", &self.nd_opt_mtu_len).field("nd_opt_mtu_reserved", &self.nd_opt_mtu_reserved).field("nd_opt_mtu_mtu", &self.nd_opt_mtu_mtu).finish()
    }
}
impl ::core::default::Default for ND_OPTION_PREFIX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ND_OPTION_RDNSS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ND_OPTION_RDNSS {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_rdnss_type == other.nd_opt_rdnss_type && self.nd_opt_rdnss_len == other.nd_opt_rdnss_len && self.nd_opt_rdnss_reserved == other.nd_opt_rdnss_reserved && self.nd_opt_rdnss_lifetime == other.nd_opt_rdnss_lifetime
    }
}
impl ::core::cmp::Eq for ND_OPTION_RDNSS {}
impl ::core::fmt::Debug for ND_OPTION_RDNSS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ND_OPTION_RDNSS").field("nd_opt_rdnss_type", &self.nd_opt_rdnss_type).field("nd_opt_rdnss_len", &self.nd_opt_rdnss_len).field("nd_opt_rdnss_reserved", &self.nd_opt_rdnss_reserved).field("nd_opt_rdnss_lifetime", &self.nd_opt_rdnss_lifetime).finish()
    }
}
impl ::core::default::Default for ND_OPTION_RD_HDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ND_OPTION_RD_HDR {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_rh_type == other.nd_opt_rh_type && self.nd_opt_rh_len == other.nd_opt_rh_len && self.nd_opt_rh_reserved1 == other.nd_opt_rh_reserved1 && self.nd_opt_rh_reserved2 == other.nd_opt_rh_reserved2
    }
}
impl ::core::cmp::Eq for ND_OPTION_RD_HDR {}
impl ::core::fmt::Debug for ND_OPTION_RD_HDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ND_OPTION_RD_HDR").field("nd_opt_rh_type", &self.nd_opt_rh_type).field("nd_opt_rh_len", &self.nd_opt_rh_len).field("nd_opt_rh_reserved1", &self.nd_opt_rh_reserved1).field("nd_opt_rh_reserved2", &self.nd_opt_rh_reserved2).finish()
    }
}
impl ::core::default::Default for ND_OPTION_ROUTE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ND_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ND_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ND_OPTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ND_REDIRECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ND_ROUTER_ADVERT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ND_ROUTER_SOLICIT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NETRESOURCE2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETRESOURCE2A {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope && self.dwType == other.dwType && self.dwUsage == other.dwUsage && self.dwDisplayType == other.dwDisplayType && self.lpLocalName == other.lpLocalName && self.lpRemoteName == other.lpRemoteName && self.lpComment == other.lpComment && self.ns_info == other.ns_info && self.ServiceType == other.ServiceType && self.dwProtocols == other.dwProtocols && self.lpiProtocols == other.lpiProtocols
    }
}
impl ::core::cmp::Eq for NETRESOURCE2A {}
impl ::core::fmt::Debug for NETRESOURCE2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETRESOURCE2A").field("dwScope", &self.dwScope).field("dwType", &self.dwType).field("dwUsage", &self.dwUsage).field("dwDisplayType", &self.dwDisplayType).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("lpComment", &self.lpComment).field("ns_info", &self.ns_info).field("ServiceType", &self.ServiceType).field("dwProtocols", &self.dwProtocols).field("lpiProtocols", &self.lpiProtocols).finish()
    }
}
impl ::core::default::Default for NETRESOURCE2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETRESOURCE2W {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope && self.dwType == other.dwType && self.dwUsage == other.dwUsage && self.dwDisplayType == other.dwDisplayType && self.lpLocalName == other.lpLocalName && self.lpRemoteName == other.lpRemoteName && self.lpComment == other.lpComment && self.ns_info == other.ns_info && self.ServiceType == other.ServiceType && self.dwProtocols == other.dwProtocols && self.lpiProtocols == other.lpiProtocols
    }
}
impl ::core::cmp::Eq for NETRESOURCE2W {}
impl ::core::fmt::Debug for NETRESOURCE2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETRESOURCE2W").field("dwScope", &self.dwScope).field("dwType", &self.dwType).field("dwUsage", &self.dwUsage).field("dwDisplayType", &self.dwDisplayType).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("lpComment", &self.lpComment).field("ns_info", &self.ns_info).field("ServiceType", &self.ServiceType).field("dwProtocols", &self.dwProtocols).field("lpiProtocols", &self.lpiProtocols).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NLA_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NLA_BLOB_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NLA_BLOB_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLA_BLOB_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NLA_CONNECTIVITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NLA_CONNECTIVITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLA_CONNECTIVITY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NLA_INTERNET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NLA_INTERNET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLA_INTERNET").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_ADDRESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_BANDWIDTH_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_BANDWIDTH_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_BANDWIDTH_FLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NL_BANDWIDTH_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NL_BANDWIDTH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bandwidth == other.Bandwidth && self.Instability == other.Instability && self.BandwidthPeaked == other.BandwidthPeaked
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NL_BANDWIDTH_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NL_BANDWIDTH_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NL_BANDWIDTH_INFORMATION").field("Bandwidth", &self.Bandwidth).field("Instability", &self.Instability).field("BandwidthPeaked", &self.BandwidthPeaked).finish()
    }
}
impl ::core::default::Default for NL_DAD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_DAD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_DAD_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_INTERFACE_NETWORK_CATEGORY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_INTERFACE_NETWORK_CATEGORY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_INTERFACE_NETWORK_CATEGORY_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_INTERFACE_OFFLOAD_ROD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NL_INTERFACE_OFFLOAD_ROD {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NL_INTERFACE_OFFLOAD_ROD {}
impl ::core::fmt::Debug for NL_INTERFACE_OFFLOAD_ROD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NL_INTERFACE_OFFLOAD_ROD").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_LINK_LOCAL_ADDRESS_BEHAVIOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_NEIGHBOR_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_NEIGHBOR_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_NEIGHBOR_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_NETWORK_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_NETWORK_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_NETWORK_CATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_NETWORK_CONNECTIVITY_COST_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_NETWORK_CONNECTIVITY_COST_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_NETWORK_CONNECTIVITY_COST_HINT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NL_NETWORK_CONNECTIVITY_HINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NL_NETWORK_CONNECTIVITY_HINT {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectivityLevel == other.ConnectivityLevel && self.ConnectivityCost == other.ConnectivityCost && self.ApproachingDataLimit == other.ApproachingDataLimit && self.OverDataLimit == other.OverDataLimit && self.Roaming == other.Roaming
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NL_NETWORK_CONNECTIVITY_HINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NL_NETWORK_CONNECTIVITY_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NL_NETWORK_CONNECTIVITY_HINT").field("ConnectivityLevel", &self.ConnectivityLevel).field("ConnectivityCost", &self.ConnectivityCost).field("ApproachingDataLimit", &self.ApproachingDataLimit).field("OverDataLimit", &self.OverDataLimit).field("Roaming", &self.Roaming).finish()
    }
}
impl ::core::default::Default for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_NETWORK_CONNECTIVITY_LEVEL_HINT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NL_PATH_BANDWIDTH_ROD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NL_PATH_BANDWIDTH_ROD {
    fn eq(&self, other: &Self) -> bool {
        self.Bandwidth == other.Bandwidth && self.Instability == other.Instability && self.BandwidthPeaked == other.BandwidthPeaked
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NL_PATH_BANDWIDTH_ROD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NL_PATH_BANDWIDTH_ROD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NL_PATH_BANDWIDTH_ROD").field("Bandwidth", &self.Bandwidth).field("Instability", &self.Instability).field("BandwidthPeaked", &self.BandwidthPeaked).finish()
    }
}
impl ::core::default::Default for NL_PREFIX_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_PREFIX_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_PREFIX_ORIGIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_ROUTER_DISCOVERY_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_ROUTER_DISCOVERY_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_ROUTER_DISCOVERY_BEHAVIOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_ROUTE_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_ROUTE_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_ROUTE_ORIGIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_ROUTE_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_ROUTE_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_ROUTE_PROTOCOL").field(&self.0).finish()
    }
}
impl ::core::default::Default for NL_SUFFIX_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_SUFFIX_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_SUFFIX_ORIGIN").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NPI_MODULEID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NPI_MODULEID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NPI_MODULEID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NPI_MODULEID_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for NSPV2_ROUTINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
impl ::core::default::Default for NSP_ROUTINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwNameSpaceFlags == other.dwNameSpaceFlags && self.lpNameSpace == other.lpNameSpace
    }
}
impl ::core::cmp::Eq for NS_INFOA {}
impl ::core::fmt::Debug for NS_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NS_INFOA").field("dwNameSpace", &self.dwNameSpace).field("dwNameSpaceFlags", &self.dwNameSpaceFlags).field("lpNameSpace", &self.lpNameSpace).finish()
    }
}
impl ::core::default::Default for NS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwNameSpaceFlags == other.dwNameSpaceFlags && self.lpNameSpace == other.lpNameSpace
    }
}
impl ::core::cmp::Eq for NS_INFOW {}
impl ::core::fmt::Debug for NS_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NS_INFOW").field("dwNameSpace", &self.dwNameSpace).field("dwNameSpaceFlags", &self.dwNameSpaceFlags).field("lpNameSpace", &self.lpNameSpace).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NS_SERVICE_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for NS_SERVICE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.ServiceInfo == other.ServiceInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for NS_SERVICE_INFOA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for NS_SERVICE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NS_SERVICE_INFOA").field("dwNameSpace", &self.dwNameSpace).field("ServiceInfo", &self.ServiceInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NS_SERVICE_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for NS_SERVICE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.ServiceInfo == other.ServiceInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for NS_SERVICE_INFOW {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for NS_SERVICE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NS_SERVICE_INFOW").field("dwNameSpace", &self.dwNameSpace).field("ServiceInfo", &self.ServiceInfo).finish()
    }
}
impl ::core::default::Default for PMTUD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PMTUD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PMTUD_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PRIORITY_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRIORITY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Sender == other.Sender && self.Receiver == other.Receiver
    }
}
impl ::core::cmp::Eq for PRIORITY_STATUS {}
impl ::core::fmt::Debug for PRIORITY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRIORITY_STATUS").field("Sender", &self.Sender).field("Receiver", &self.Receiver).finish()
    }
}
impl ::core::default::Default for PROTOCOL_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROTOCOL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags == other.dwServiceFlags && self.iAddressFamily == other.iAddressFamily && self.iMaxSockAddr == other.iMaxSockAddr && self.iMinSockAddr == other.iMinSockAddr && self.iSocketType == other.iSocketType && self.iProtocol == other.iProtocol && self.dwMessageSize == other.dwMessageSize && self.lpProtocol == other.lpProtocol
    }
}
impl ::core::cmp::Eq for PROTOCOL_INFOA {}
impl ::core::fmt::Debug for PROTOCOL_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOL_INFOA").field("dwServiceFlags", &self.dwServiceFlags).field("iAddressFamily", &self.iAddressFamily).field("iMaxSockAddr", &self.iMaxSockAddr).field("iMinSockAddr", &self.iMinSockAddr).field("iSocketType", &self.iSocketType).field("iProtocol", &self.iProtocol).field("dwMessageSize", &self.dwMessageSize).field("lpProtocol", &self.lpProtocol).finish()
    }
}
impl ::core::default::Default for PROTOCOL_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROTOCOL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags == other.dwServiceFlags && self.iAddressFamily == other.iAddressFamily && self.iMaxSockAddr == other.iMaxSockAddr && self.iMinSockAddr == other.iMinSockAddr && self.iSocketType == other.iSocketType && self.iProtocol == other.iProtocol && self.dwMessageSize == other.dwMessageSize && self.lpProtocol == other.lpProtocol
    }
}
impl ::core::cmp::Eq for PROTOCOL_INFOW {}
impl ::core::fmt::Debug for PROTOCOL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOL_INFOW").field("dwServiceFlags", &self.dwServiceFlags).field("iAddressFamily", &self.iAddressFamily).field("iMaxSockAddr", &self.iMaxSockAddr).field("iMinSockAddr", &self.iMinSockAddr).field("iSocketType", &self.iSocketType).field("iProtocol", &self.iProtocol).field("dwMessageSize", &self.dwMessageSize).field("lpProtocol", &self.lpProtocol).finish()
    }
}
impl ::core::default::Default for PROTOENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROTOENT {
    fn eq(&self, other: &Self) -> bool {
        self.p_name == other.p_name && self.p_aliases == other.p_aliases && self.p_proto == other.p_proto
    }
}
impl ::core::cmp::Eq for PROTOENT {}
impl ::core::fmt::Debug for PROTOENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOENT").field("p_name", &self.p_name).field("p_aliases", &self.p_aliases).field("p_proto", &self.p_proto).finish()
    }
}
impl ::core::default::Default for Q2931_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Q2931_IE {
    fn eq(&self, other: &Self) -> bool {
        self.IEType == other.IEType && self.IELength == other.IELength && self.IE == other.IE
    }
}
impl ::core::cmp::Eq for Q2931_IE {}
impl ::core::fmt::Debug for Q2931_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Q2931_IE").field("IEType", &self.IEType).field("IELength", &self.IELength).field("IE", &self.IE).finish()
    }
}
impl ::core::default::Default for Q2931_IE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Q2931_IE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Q2931_IE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for QOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS {
    fn eq(&self, other: &Self) -> bool {
        self.SendingFlowspec == other.SendingFlowspec && self.ReceivingFlowspec == other.ReceivingFlowspec && self.ProviderSpecific == other.ProviderSpecific
    }
}
impl ::core::cmp::Eq for QOS {}
impl ::core::fmt::Debug for QOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS").field("SendingFlowspec", &self.SendingFlowspec).field("ReceivingFlowspec", &self.ReceivingFlowspec).field("ProviderSpecific", &self.ProviderSpecific).finish()
    }
}
impl ::core::default::Default for RCVALL_IF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RCVALL_IF {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Interface == other.Interface
    }
}
impl ::core::cmp::Eq for RCVALL_IF {}
impl ::core::fmt::Debug for RCVALL_IF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RCVALL_IF").field("Mode", &self.Mode).field("Interface", &self.Interface).finish()
    }
}
impl ::core::default::Default for RCVALL_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RCVALL_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RCVALL_VALUE").field(&self.0).finish()
    }
}
impl ::core::default::Default for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.BrokerEventGuid == other.BrokerEventGuid
    }
}
impl ::core::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_INPUT {}
impl ::core::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REAL_TIME_NOTIFICATION_SETTING_INPUT").field("TransportSettingId", &self.TransportSettingId).field("BrokerEventGuid", &self.BrokerEventGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.BrokerEventGuid == other.BrokerEventGuid && self.Unmark == other.Unmark
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REAL_TIME_NOTIFICATION_SETTING_INPUT_EX").field("TransportSettingId", &self.TransportSettingId).field("BrokerEventGuid", &self.BrokerEventGuid).field("Unmark", &self.Unmark).finish()
    }
}
impl ::core::default::Default for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelStatus == other.ChannelStatus
    }
}
impl ::core::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {}
impl ::core::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REAL_TIME_NOTIFICATION_SETTING_OUTPUT").field("ChannelStatus", &self.ChannelStatus).finish()
    }
}
impl ::core::default::Default for RESOURCE_DISPLAY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESOURCE_DISPLAY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESOURCE_DISPLAY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RIORESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RIORESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.BytesTransferred == other.BytesTransferred && self.SocketContext == other.SocketContext && self.RequestContext == other.RequestContext
    }
}
impl ::core::cmp::Eq for RIORESULT {}
impl ::core::fmt::Debug for RIORESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIORESULT").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).field("SocketContext", &self.SocketContext).field("RequestContext", &self.RequestContext).finish()
    }
}
impl ::core::default::Default for RIO_BUF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RIO_BUF {
    fn eq(&self, other: &Self) -> bool {
        self.BufferId == other.BufferId && self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for RIO_BUF {}
impl ::core::fmt::Debug for RIO_BUF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_BUF").field("BufferId", &self.BufferId).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for RIO_CMSG_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RIO_CMSG_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.TotalLength == other.TotalLength
    }
}
impl ::core::cmp::Eq for RIO_CMSG_BUFFER {}
impl ::core::fmt::Debug for RIO_CMSG_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_CMSG_BUFFER").field("TotalLength", &self.TotalLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RIO_EXTENSION_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RIO_NOTIFICATION_COMPLETION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RIO_NOTIFICATION_COMPLETION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RIO_NOTIFICATION_COMPLETION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RIO_NOTIFICATION_COMPLETION_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_FEC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_FEC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FECBlockSize == other.FECBlockSize && self.FECProActivePackets == other.FECProActivePackets && self.FECGroupSize == other.FECGroupSize && self.fFECOnDemandParityEnabled == other.fFECOnDemandParityEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RM_FEC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RM_FEC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_FEC_INFO").field("FECBlockSize", &self.FECBlockSize).field("FECProActivePackets", &self.FECProActivePackets).field("FECGroupSize", &self.FECGroupSize).field("fFECOnDemandParityEnabled", &self.fFECOnDemandParityEnabled).finish()
    }
}
impl ::core::default::Default for RM_RECEIVER_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RM_RECEIVER_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.NumODataPacketsReceived == other.NumODataPacketsReceived
            && self.NumRDataPacketsReceived == other.NumRDataPacketsReceived
            && self.NumDuplicateDataPackets == other.NumDuplicateDataPackets
            && self.DataBytesReceived == other.DataBytesReceived
            && self.TotalBytesReceived == other.TotalBytesReceived
            && self.RateKBitsPerSecOverall == other.RateKBitsPerSecOverall
            && self.RateKBitsPerSecLast == other.RateKBitsPerSecLast
            && self.TrailingEdgeSeqId == other.TrailingEdgeSeqId
            && self.LeadingEdgeSeqId == other.LeadingEdgeSeqId
            && self.AverageSequencesInWindow == other.AverageSequencesInWindow
            && self.MinSequencesInWindow == other.MinSequencesInWindow
            && self.MaxSequencesInWindow == other.MaxSequencesInWindow
            && self.FirstNakSequenceNumber == other.FirstNakSequenceNumber
            && self.NumPendingNaks == other.NumPendingNaks
            && self.NumOutstandingNaks == other.NumOutstandingNaks
            && self.NumDataPacketsBuffered == other.NumDataPacketsBuffered
            && self.TotalSelectiveNaksSent == other.TotalSelectiveNaksSent
            && self.TotalParityNaksSent == other.TotalParityNaksSent
    }
}
impl ::core::cmp::Eq for RM_RECEIVER_STATS {}
impl ::core::fmt::Debug for RM_RECEIVER_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_RECEIVER_STATS")
            .field("NumODataPacketsReceived", &self.NumODataPacketsReceived)
            .field("NumRDataPacketsReceived", &self.NumRDataPacketsReceived)
            .field("NumDuplicateDataPackets", &self.NumDuplicateDataPackets)
            .field("DataBytesReceived", &self.DataBytesReceived)
            .field("TotalBytesReceived", &self.TotalBytesReceived)
            .field("RateKBitsPerSecOverall", &self.RateKBitsPerSecOverall)
            .field("RateKBitsPerSecLast", &self.RateKBitsPerSecLast)
            .field("TrailingEdgeSeqId", &self.TrailingEdgeSeqId)
            .field("LeadingEdgeSeqId", &self.LeadingEdgeSeqId)
            .field("AverageSequencesInWindow", &self.AverageSequencesInWindow)
            .field("MinSequencesInWindow", &self.MinSequencesInWindow)
            .field("MaxSequencesInWindow", &self.MaxSequencesInWindow)
            .field("FirstNakSequenceNumber", &self.FirstNakSequenceNumber)
            .field("NumPendingNaks", &self.NumPendingNaks)
            .field("NumOutstandingNaks", &self.NumOutstandingNaks)
            .field("NumDataPacketsBuffered", &self.NumDataPacketsBuffered)
            .field("TotalSelectiveNaksSent", &self.TotalSelectiveNaksSent)
            .field("TotalParityNaksSent", &self.TotalParityNaksSent)
            .finish()
    }
}
impl ::core::default::Default for RM_SENDER_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RM_SENDER_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.DataBytesSent == other.DataBytesSent && self.TotalBytesSent == other.TotalBytesSent && self.NaksReceived == other.NaksReceived && self.NaksReceivedTooLate == other.NaksReceivedTooLate && self.NumOutstandingNaks == other.NumOutstandingNaks && self.NumNaksAfterRData == other.NumNaksAfterRData && self.RepairPacketsSent == other.RepairPacketsSent && self.BufferSpaceAvailable == other.BufferSpaceAvailable && self.TrailingEdgeSeqId == other.TrailingEdgeSeqId && self.LeadingEdgeSeqId == other.LeadingEdgeSeqId && self.RateKBitsPerSecOverall == other.RateKBitsPerSecOverall && self.RateKBitsPerSecLast == other.RateKBitsPerSecLast && self.TotalODataPacketsSent == other.TotalODataPacketsSent
    }
}
impl ::core::cmp::Eq for RM_SENDER_STATS {}
impl ::core::fmt::Debug for RM_SENDER_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_SENDER_STATS")
            .field("DataBytesSent", &self.DataBytesSent)
            .field("TotalBytesSent", &self.TotalBytesSent)
            .field("NaksReceived", &self.NaksReceived)
            .field("NaksReceivedTooLate", &self.NaksReceivedTooLate)
            .field("NumOutstandingNaks", &self.NumOutstandingNaks)
            .field("NumNaksAfterRData", &self.NumNaksAfterRData)
            .field("RepairPacketsSent", &self.RepairPacketsSent)
            .field("BufferSpaceAvailable", &self.BufferSpaceAvailable)
            .field("TrailingEdgeSeqId", &self.TrailingEdgeSeqId)
            .field("LeadingEdgeSeqId", &self.LeadingEdgeSeqId)
            .field("RateKBitsPerSecOverall", &self.RateKBitsPerSecOverall)
            .field("RateKBitsPerSecLast", &self.RateKBitsPerSecLast)
            .field("TotalODataPacketsSent", &self.TotalODataPacketsSent)
            .finish()
    }
}
impl ::core::default::Default for RM_SEND_WINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RM_SEND_WINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.RateKbitsPerSec == other.RateKbitsPerSec && self.WindowSizeInMSecs == other.WindowSizeInMSecs && self.WindowSizeInBytes == other.WindowSizeInBytes
    }
}
impl ::core::cmp::Eq for RM_SEND_WINDOW {}
impl ::core::fmt::Debug for RM_SEND_WINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_SEND_WINDOW").field("RateKbitsPerSec", &self.RateKbitsPerSec).field("WindowSizeInMSecs", &self.WindowSizeInMSecs).field("WindowSizeInBytes", &self.WindowSizeInBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RSS_SCALABILITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RSS_SCALABILITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RssEnabled == other.RssEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RSS_SCALABILITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RSS_SCALABILITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSS_SCALABILITY_INFO").field("RssEnabled", &self.RssEnabled).finish()
    }
}
impl ::core::default::Default for SCOPE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SCOPE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCOPE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCOPE_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEND_RECV_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEND_RECV_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEND_RECV_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SEND_RECV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SEND_RECV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SEND_RECV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SEND_RECV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SEND_RECV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SERVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SERVENT {
    fn eq(&self, other: &Self) -> bool {
        self.s_name == other.s_name && self.s_aliases == other.s_aliases && self.s_proto == other.s_proto && self.s_port == other.s_port
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SERVENT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SERVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVENT").field("s_name", &self.s_name).field("s_aliases", &self.s_aliases).field("s_proto", &self.s_proto).field("s_port", &self.s_port).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SERVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SERVENT {
    fn eq(&self, other: &Self) -> bool {
        self.s_name == other.s_name && self.s_aliases == other.s_aliases && self.s_port == other.s_port && self.s_proto == other.s_proto
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SERVENT {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for SERVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVENT").field("s_name", &self.s_name).field("s_aliases", &self.s_aliases).field("s_port", &self.s_port).field("s_proto", &self.s_proto).finish()
    }
}
impl ::core::default::Default for SERVICE_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddressType == other.dwAddressType && self.dwAddressFlags == other.dwAddressFlags && self.dwAddressLength == other.dwAddressLength && self.dwPrincipalLength == other.dwPrincipalLength && self.lpAddress == other.lpAddress && self.lpPrincipal == other.lpPrincipal
    }
}
impl ::core::cmp::Eq for SERVICE_ADDRESS {}
impl ::core::fmt::Debug for SERVICE_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_ADDRESS").field("dwAddressType", &self.dwAddressType).field("dwAddressFlags", &self.dwAddressFlags).field("dwAddressLength", &self.dwAddressLength).field("dwPrincipalLength", &self.dwPrincipalLength).field("lpAddress", &self.lpAddress).field("lpPrincipal", &self.lpPrincipal).finish()
    }
}
impl ::core::default::Default for SERVICE_ADDRESSES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddressCount == other.dwAddressCount && self.Addresses == other.Addresses
    }
}
impl ::core::cmp::Eq for SERVICE_ADDRESSES {}
impl ::core::fmt::Debug for SERVICE_ADDRESSES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_ADDRESSES").field("dwAddressCount", &self.dwAddressCount).field("Addresses", &self.Addresses).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_ASYNC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SERVICE_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SERVICE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceType == other.lpServiceType && self.lpServiceName == other.lpServiceName && self.lpComment == other.lpComment && self.lpLocale == other.lpLocale && self.dwDisplayHint == other.dwDisplayHint && self.dwVersion == other.dwVersion && self.dwTime == other.dwTime && self.lpMachineName == other.lpMachineName && self.lpServiceAddress == other.lpServiceAddress && self.ServiceSpecificInfo == other.ServiceSpecificInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SERVICE_INFOA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SERVICE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFOA").field("lpServiceType", &self.lpServiceType).field("lpServiceName", &self.lpServiceName).field("lpComment", &self.lpComment).field("lpLocale", &self.lpLocale).field("dwDisplayHint", &self.dwDisplayHint).field("dwVersion", &self.dwVersion).field("dwTime", &self.dwTime).field("lpMachineName", &self.lpMachineName).field("lpServiceAddress", &self.lpServiceAddress).field("ServiceSpecificInfo", &self.ServiceSpecificInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SERVICE_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SERVICE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceType == other.lpServiceType && self.lpServiceName == other.lpServiceName && self.lpComment == other.lpComment && self.lpLocale == other.lpLocale && self.dwDisplayHint == other.dwDisplayHint && self.dwVersion == other.dwVersion && self.dwTime == other.dwTime && self.lpMachineName == other.lpMachineName && self.lpServiceAddress == other.lpServiceAddress && self.ServiceSpecificInfo == other.ServiceSpecificInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SERVICE_INFOW {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SERVICE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFOW").field("lpServiceType", &self.lpServiceType).field("lpServiceName", &self.lpServiceName).field("lpComment", &self.lpComment).field("lpLocale", &self.lpLocale).field("dwDisplayHint", &self.dwDisplayHint).field("dwVersion", &self.dwVersion).field("dwTime", &self.dwTime).field("lpMachineName", &self.lpMachineName).field("lpServiceAddress", &self.lpServiceAddress).field("ServiceSpecificInfo", &self.ServiceSpecificInfo).finish()
    }
}
impl ::core::default::Default for SERVICE_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwTypeNameOffset == other.dwTypeNameOffset && self.dwValueCount == other.dwValueCount && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_INFO {}
impl ::core::fmt::Debug for SERVICE_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_INFO").field("dwTypeNameOffset", &self.dwTypeNameOffset).field("dwValueCount", &self.dwValueCount).field("Values", &self.Values).finish()
    }
}
impl ::core::default::Default for SERVICE_TYPE_INFO_ABSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_INFO_ABSA {
    fn eq(&self, other: &Self) -> bool {
        self.lpTypeName == other.lpTypeName && self.dwValueCount == other.dwValueCount && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_INFO_ABSA {}
impl ::core::fmt::Debug for SERVICE_TYPE_INFO_ABSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_INFO_ABSA").field("lpTypeName", &self.lpTypeName).field("dwValueCount", &self.dwValueCount).field("Values", &self.Values).finish()
    }
}
impl ::core::default::Default for SERVICE_TYPE_INFO_ABSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_INFO_ABSW {
    fn eq(&self, other: &Self) -> bool {
        self.lpTypeName == other.lpTypeName && self.dwValueCount == other.dwValueCount && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_INFO_ABSW {}
impl ::core::fmt::Debug for SERVICE_TYPE_INFO_ABSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_INFO_ABSW").field("lpTypeName", &self.lpTypeName).field("dwValueCount", &self.dwValueCount).field("Values", &self.Values).finish()
    }
}
impl ::core::default::Default for SERVICE_TYPE_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.dwValueNameOffset == other.dwValueNameOffset && self.dwValueOffset == other.dwValueOffset
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_VALUE {}
impl ::core::fmt::Debug for SERVICE_TYPE_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_VALUE").field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("dwValueNameOffset", &self.dwValueNameOffset).field("dwValueOffset", &self.dwValueOffset).finish()
    }
}
impl ::core::default::Default for SERVICE_TYPE_VALUE_ABSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_VALUE_ABSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.lpValueName == other.lpValueName && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_VALUE_ABSA {}
impl ::core::fmt::Debug for SERVICE_TYPE_VALUE_ABSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_VALUE_ABSA").field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("lpValueName", &self.lpValueName).field("lpValue", &self.lpValue).finish()
    }
}
impl ::core::default::Default for SERVICE_TYPE_VALUE_ABSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_VALUE_ABSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.lpValueName == other.lpValueName && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_VALUE_ABSW {}
impl ::core::fmt::Debug for SERVICE_TYPE_VALUE_ABSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_VALUE_ABSW").field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("lpValueName", &self.lpValueName).field("lpValue", &self.lpValue).finish()
    }
}
impl ::core::default::Default for SET_SERVICE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SET_SERVICE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_SERVICE_OPERATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for SNAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SNAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Dsap == other.Dsap && self.Ssap == other.Ssap && self.Control == other.Control && self.Oui == other.Oui && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for SNAP_HEADER {}
impl ::core::fmt::Debug for SNAP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SNAP_HEADER").field("Dsap", &self.Dsap).field("Ssap", &self.Ssap).field("Control", &self.Control).field("Oui", &self.Oui).field("Type", &self.Type).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        self.sa_family == other.sa_family && self.sa_data == other.sa_data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKADDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR").field("sa_family", &self.sa_family).field("sa_data", &self.sa_data).finish()
    }
}
impl ::core::default::Default for SOCKADDR_ATM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_ATM {
    fn eq(&self, other: &Self) -> bool {
        self.satm_family == other.satm_family && self.satm_number == other.satm_number && self.satm_blli == other.satm_blli && self.satm_bhli == other.satm_bhli
    }
}
impl ::core::cmp::Eq for SOCKADDR_ATM {}
impl ::core::fmt::Debug for SOCKADDR_ATM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_ATM").field("satm_family", &self.satm_family).field("satm_number", &self.satm_number).field("satm_blli", &self.satm_blli).field("satm_bhli", &self.satm_bhli).finish()
    }
}
impl ::core::default::Default for SOCKADDR_DL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_DL {
    fn eq(&self, other: &Self) -> bool {
        self.sdl_family == other.sdl_family && self.sdl_data == other.sdl_data && self.sdl_zero == other.sdl_zero
    }
}
impl ::core::cmp::Eq for SOCKADDR_DL {}
impl ::core::fmt::Debug for SOCKADDR_DL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_DL").field("sdl_family", &self.sdl_family).field("sdl_data", &self.sdl_data).field("sdl_zero", &self.sdl_zero).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR_IN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SOCKADDR_IN6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SOCKADDR_IN6_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_IN6_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.SourceAddress == other.SourceAddress && self.DestinationAddress == other.DestinationAddress
    }
}
impl ::core::cmp::Eq for SOCKADDR_IN6_PAIR {}
impl ::core::fmt::Debug for SOCKADDR_IN6_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IN6_PAIR").field("SourceAddress", &self.SourceAddress).field("DestinationAddress", &self.DestinationAddress).finish()
    }
}
impl ::core::default::Default for SOCKADDR_IN6_W2KSP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR_INET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR_IPX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKADDR_IPX {
    fn eq(&self, other: &Self) -> bool {
        self.sa_family == other.sa_family && self.sa_netnum == other.sa_netnum && self.sa_nodenum == other.sa_nodenum && self.sa_socket == other.sa_socket
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKADDR_IPX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKADDR_IPX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IPX").field("sa_family", &self.sa_family).field("sa_netnum", &self.sa_netnum).field("sa_nodenum", &self.sa_nodenum).field("sa_socket", &self.sa_socket).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR_IRDA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKADDR_IRDA {
    fn eq(&self, other: &Self) -> bool {
        self.irdaAddressFamily == other.irdaAddressFamily && self.irdaDeviceID == other.irdaDeviceID && self.irdaServiceName == other.irdaServiceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKADDR_IRDA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKADDR_IRDA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IRDA").field("irdaAddressFamily", &self.irdaAddressFamily).field("irdaDeviceID", &self.irdaDeviceID).field("irdaServiceName", &self.irdaServiceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR_NB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKADDR_NB {
    fn eq(&self, other: &Self) -> bool {
        self.snb_family == other.snb_family && self.snb_type == other.snb_type && self.snb_name == other.snb_name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKADDR_NB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKADDR_NB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_NB").field("snb_family", &self.snb_family).field("snb_type", &self.snb_type).field("snb_name", &self.snb_name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR_STORAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKADDR_STORAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ss_family == other.ss_family && self.__ss_pad1 == other.__ss_pad1 && self.__ss_align == other.__ss_align && self.__ss_pad2 == other.__ss_pad2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKADDR_STORAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKADDR_STORAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_STORAGE").field("ss_family", &self.ss_family).field("__ss_pad1", &self.__ss_pad1).field("__ss_align", &self.__ss_align).field("__ss_pad2", &self.__ss_pad2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR_STORAGE_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKADDR_STORAGE_XP {
    fn eq(&self, other: &Self) -> bool {
        self.ss_family == other.ss_family && self.__ss_pad1 == other.__ss_pad1 && self.__ss_align == other.__ss_align && self.__ss_pad2 == other.__ss_pad2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKADDR_STORAGE_XP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKADDR_STORAGE_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_STORAGE_XP").field("ss_family", &self.ss_family).field("__ss_pad1", &self.__ss_pad1).field("__ss_align", &self.__ss_align).field("__ss_pad2", &self.__ss_pad2).finish()
    }
}
impl ::core::default::Default for SOCKADDR_TP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_TP {
    fn eq(&self, other: &Self) -> bool {
        self.tp_family == other.tp_family && self.tp_addr_type == other.tp_addr_type && self.tp_taddr_len == other.tp_taddr_len && self.tp_tsel_len == other.tp_tsel_len && self.tp_addr == other.tp_addr
    }
}
impl ::core::cmp::Eq for SOCKADDR_TP {}
impl ::core::fmt::Debug for SOCKADDR_TP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_TP").field("tp_family", &self.tp_family).field("tp_addr_type", &self.tp_addr_type).field("tp_taddr_len", &self.tp_taddr_len).field("tp_tsel_len", &self.tp_tsel_len).field("tp_addr", &self.tp_addr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKADDR_UN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKADDR_UN {
    fn eq(&self, other: &Self) -> bool {
        self.sun_family == other.sun_family && self.sun_path == other.sun_path
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKADDR_UN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKADDR_UN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_UN").field("sun_family", &self.sun_family).field("sun_path", &self.sun_path).finish()
    }
}
impl ::core::default::Default for SOCKADDR_VNS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_VNS {
    fn eq(&self, other: &Self) -> bool {
        self.sin_family == other.sin_family && self.net_address == other.net_address && self.subnet_addr == other.subnet_addr && self.port == other.port && self.hops == other.hops && self.filler == other.filler
    }
}
impl ::core::cmp::Eq for SOCKADDR_VNS {}
impl ::core::fmt::Debug for SOCKADDR_VNS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_VNS").field("sin_family", &self.sin_family).field("net_address", &self.net_address).field("subnet_addr", &self.subnet_addr).field("port", &self.port).field("hops", &self.hops).field("filler", &self.filler).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKET_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKET_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.lpSockaddr == other.lpSockaddr && self.iSockaddrLength == other.iSockaddrLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKET_ADDRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKET_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_ADDRESS").field("lpSockaddr", &self.lpSockaddr).field("iSockaddrLength", &self.iSockaddrLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKET_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKET_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.iAddressCount == other.iAddressCount && self.Address == other.Address
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKET_ADDRESS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKET_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_ADDRESS_LIST").field("iAddressCount", &self.iAddressCount).field("Address", &self.Address).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKET_PEER_TARGET_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKET_PEER_TARGET_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.PeerAddress == other.PeerAddress && self.PeerTargetNameStringLen == other.PeerTargetNameStringLen && self.AllStrings == other.AllStrings
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKET_PEER_TARGET_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKET_PEER_TARGET_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_PEER_TARGET_NAME").field("SecurityProtocol", &self.SecurityProtocol).field("PeerAddress", &self.PeerAddress).field("PeerTargetNameStringLen", &self.PeerTargetNameStringLen).field("AllStrings", &self.AllStrings).finish()
    }
}
impl ::core::default::Default for SOCKET_PRIORITY_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOCKET_PRIORITY_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOCKET_PRIORITY_HINT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for SOCKET_PROCESSOR_AFFINITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for SOCKET_PROCESSOR_AFFINITY {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor && self.NumaNodeId == other.NumaNodeId && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for SOCKET_PROCESSOR_AFFINITY {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for SOCKET_PROCESSOR_AFFINITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_PROCESSOR_AFFINITY").field("Processor", &self.Processor).field("NumaNodeId", &self.NumaNodeId).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for SOCKET_SECURITY_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOCKET_SECURITY_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOCKET_SECURITY_PROTOCOL").field(&self.0).finish()
    }
}
impl ::core::default::Default for SOCKET_SECURITY_QUERY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_QUERY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.Flags == other.Flags && self.PeerApplicationAccessTokenHandle == other.PeerApplicationAccessTokenHandle && self.PeerMachineAccessTokenHandle == other.PeerMachineAccessTokenHandle
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_QUERY_INFO {}
impl ::core::fmt::Debug for SOCKET_SECURITY_QUERY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_QUERY_INFO").field("SecurityProtocol", &self.SecurityProtocol).field("Flags", &self.Flags).field("PeerApplicationAccessTokenHandle", &self.PeerApplicationAccessTokenHandle).field("PeerMachineAccessTokenHandle", &self.PeerMachineAccessTokenHandle).finish()
    }
}
impl ::core::default::Default for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.Flags == other.Flags && self.PeerApplicationAccessTokenHandle == other.PeerApplicationAccessTokenHandle && self.PeerMachineAccessTokenHandle == other.PeerMachineAccessTokenHandle && self.MmSaId == other.MmSaId && self.QmSaId == other.QmSaId && self.NegotiationWinerr == other.NegotiationWinerr && self.SaLookupContext == other.SaLookupContext
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {}
impl ::core::fmt::Debug for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_QUERY_INFO_IPSEC2").field("SecurityProtocol", &self.SecurityProtocol).field("Flags", &self.Flags).field("PeerApplicationAccessTokenHandle", &self.PeerApplicationAccessTokenHandle).field("PeerMachineAccessTokenHandle", &self.PeerMachineAccessTokenHandle).field("MmSaId", &self.MmSaId).field("QmSaId", &self.QmSaId).field("NegotiationWinerr", &self.NegotiationWinerr).field("SaLookupContext", &self.SaLookupContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.PeerAddress == other.PeerAddress && self.PeerTokenAccessMask == other.PeerTokenAccessMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKET_SECURITY_QUERY_TEMPLATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_QUERY_TEMPLATE").field("SecurityProtocol", &self.SecurityProtocol).field("PeerAddress", &self.PeerAddress).field("PeerTokenAccessMask", &self.PeerTokenAccessMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.PeerAddress == other.PeerAddress && self.PeerTokenAccessMask == other.PeerTokenAccessMask && self.Flags == other.Flags && self.FieldMask == other.FieldMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2").field("SecurityProtocol", &self.SecurityProtocol).field("PeerAddress", &self.PeerAddress).field("PeerTokenAccessMask", &self.PeerTokenAccessMask).field("Flags", &self.Flags).field("FieldMask", &self.FieldMask).finish()
    }
}
impl ::core::default::Default for SOCKET_SECURITY_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.SecurityFlags == other.SecurityFlags
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_SETTINGS {}
impl ::core::fmt::Debug for SOCKET_SECURITY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_SETTINGS").field("SecurityProtocol", &self.SecurityProtocol).field("SecurityFlags", &self.SecurityFlags).finish()
    }
}
impl ::core::default::Default for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.SecurityFlags == other.SecurityFlags && self.IpsecFlags == other.IpsecFlags && self.AuthipMMPolicyKey == other.AuthipMMPolicyKey && self.AuthipQMPolicyKey == other.AuthipQMPolicyKey && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2 && self.UserNameStringLen == other.UserNameStringLen && self.DomainNameStringLen == other.DomainNameStringLen && self.PasswordStringLen == other.PasswordStringLen && self.AllStrings == other.AllStrings
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_SETTINGS_IPSEC {}
impl ::core::fmt::Debug for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_SETTINGS_IPSEC")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("SecurityFlags", &self.SecurityFlags)
            .field("IpsecFlags", &self.IpsecFlags)
            .field("AuthipMMPolicyKey", &self.AuthipMMPolicyKey)
            .field("AuthipQMPolicyKey", &self.AuthipQMPolicyKey)
            .field("Reserved", &self.Reserved)
            .field("Reserved2", &self.Reserved2)
            .field("UserNameStringLen", &self.UserNameStringLen)
            .field("DomainNameStringLen", &self.DomainNameStringLen)
            .field("PasswordStringLen", &self.PasswordStringLen)
            .field("AllStrings", &self.AllStrings)
            .finish()
    }
}
impl ::core::default::Default for SOCKET_USAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOCKET_USAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOCKET_USAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SOCK_NOTIFY_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOCK_NOTIFY_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.socket == other.socket && self.completionKey == other.completionKey && self.eventFilter == other.eventFilter && self.operation == other.operation && self.triggerFlags == other.triggerFlags && self.registrationResult == other.registrationResult
    }
}
impl ::core::cmp::Eq for SOCK_NOTIFY_REGISTRATION {}
impl ::core::fmt::Debug for SOCK_NOTIFY_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCK_NOTIFY_REGISTRATION").field("socket", &self.socket).field("completionKey", &self.completionKey).field("eventFilter", &self.eventFilter).field("operation", &self.operation).field("triggerFlags", &self.triggerFlags).field("registrationResult", &self.registrationResult).finish()
    }
}
impl ::core::default::Default for TCPSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCPSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCPSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TCP_ACK_FREQUENCY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ACK_FREQUENCY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.TcpDelayedAckFrequency == other.TcpDelayedAckFrequency
    }
}
impl ::core::cmp::Eq for TCP_ACK_FREQUENCY_PARAMETERS {}
impl ::core::fmt::Debug for TCP_ACK_FREQUENCY_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ACK_FREQUENCY_PARAMETERS").field("TcpDelayedAckFrequency", &self.TcpDelayedAckFrequency).finish()
    }
}
impl ::core::default::Default for TCP_HDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCP_ICW_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCP_ICW_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_ICW_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for TCP_ICW_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_ICW_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level
    }
}
impl ::core::cmp::Eq for TCP_ICW_PARAMETERS {}
impl ::core::fmt::Debug for TCP_ICW_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ICW_PARAMETERS").field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_INFO_v0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_INFO_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.Mss == other.Mss && self.ConnectionTimeMs == other.ConnectionTimeMs && self.TimestampsEnabled == other.TimestampsEnabled && self.RttUs == other.RttUs && self.MinRttUs == other.MinRttUs && self.BytesInFlight == other.BytesInFlight && self.Cwnd == other.Cwnd && self.SndWnd == other.SndWnd && self.RcvWnd == other.RcvWnd && self.RcvBuf == other.RcvBuf && self.BytesOut == other.BytesOut && self.BytesIn == other.BytesIn && self.BytesReordered == other.BytesReordered && self.BytesRetrans == other.BytesRetrans && self.FastRetrans == other.FastRetrans && self.DupAcksIn == other.DupAcksIn && self.TimeoutEpisodes == other.TimeoutEpisodes && self.SynRetrans == other.SynRetrans
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_INFO_v0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_INFO_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_INFO_v0")
            .field("State", &self.State)
            .field("Mss", &self.Mss)
            .field("ConnectionTimeMs", &self.ConnectionTimeMs)
            .field("TimestampsEnabled", &self.TimestampsEnabled)
            .field("RttUs", &self.RttUs)
            .field("MinRttUs", &self.MinRttUs)
            .field("BytesInFlight", &self.BytesInFlight)
            .field("Cwnd", &self.Cwnd)
            .field("SndWnd", &self.SndWnd)
            .field("RcvWnd", &self.RcvWnd)
            .field("RcvBuf", &self.RcvBuf)
            .field("BytesOut", &self.BytesOut)
            .field("BytesIn", &self.BytesIn)
            .field("BytesReordered", &self.BytesReordered)
            .field("BytesRetrans", &self.BytesRetrans)
            .field("FastRetrans", &self.FastRetrans)
            .field("DupAcksIn", &self.DupAcksIn)
            .field("TimeoutEpisodes", &self.TimeoutEpisodes)
            .field("SynRetrans", &self.SynRetrans)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCP_INFO_v1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCP_INFO_v1 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
            && self.Mss == other.Mss
            && self.ConnectionTimeMs == other.ConnectionTimeMs
            && self.TimestampsEnabled == other.TimestampsEnabled
            && self.RttUs == other.RttUs
            && self.MinRttUs == other.MinRttUs
            && self.BytesInFlight == other.BytesInFlight
            && self.Cwnd == other.Cwnd
            && self.SndWnd == other.SndWnd
            && self.RcvWnd == other.RcvWnd
            && self.RcvBuf == other.RcvBuf
            && self.BytesOut == other.BytesOut
            && self.BytesIn == other.BytesIn
            && self.BytesReordered == other.BytesReordered
            && self.BytesRetrans == other.BytesRetrans
            && self.FastRetrans == other.FastRetrans
            && self.DupAcksIn == other.DupAcksIn
            && self.TimeoutEpisodes == other.TimeoutEpisodes
            && self.SynRetrans == other.SynRetrans
            && self.SndLimTransRwin == other.SndLimTransRwin
            && self.SndLimTimeRwin == other.SndLimTimeRwin
            && self.SndLimBytesRwin == other.SndLimBytesRwin
            && self.SndLimTransCwnd == other.SndLimTransCwnd
            && self.SndLimTimeCwnd == other.SndLimTimeCwnd
            && self.SndLimBytesCwnd == other.SndLimBytesCwnd
            && self.SndLimTransSnd == other.SndLimTransSnd
            && self.SndLimTimeSnd == other.SndLimTimeSnd
            && self.SndLimBytesSnd == other.SndLimBytesSnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCP_INFO_v1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCP_INFO_v1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_INFO_v1")
            .field("State", &self.State)
            .field("Mss", &self.Mss)
            .field("ConnectionTimeMs", &self.ConnectionTimeMs)
            .field("TimestampsEnabled", &self.TimestampsEnabled)
            .field("RttUs", &self.RttUs)
            .field("MinRttUs", &self.MinRttUs)
            .field("BytesInFlight", &self.BytesInFlight)
            .field("Cwnd", &self.Cwnd)
            .field("SndWnd", &self.SndWnd)
            .field("RcvWnd", &self.RcvWnd)
            .field("RcvBuf", &self.RcvBuf)
            .field("BytesOut", &self.BytesOut)
            .field("BytesIn", &self.BytesIn)
            .field("BytesReordered", &self.BytesReordered)
            .field("BytesRetrans", &self.BytesRetrans)
            .field("FastRetrans", &self.FastRetrans)
            .field("DupAcksIn", &self.DupAcksIn)
            .field("TimeoutEpisodes", &self.TimeoutEpisodes)
            .field("SynRetrans", &self.SynRetrans)
            .field("SndLimTransRwin", &self.SndLimTransRwin)
            .field("SndLimTimeRwin", &self.SndLimTimeRwin)
            .field("SndLimBytesRwin", &self.SndLimBytesRwin)
            .field("SndLimTransCwnd", &self.SndLimTransCwnd)
            .field("SndLimTimeCwnd", &self.SndLimTimeCwnd)
            .field("SndLimBytesCwnd", &self.SndLimBytesCwnd)
            .field("SndLimTransSnd", &self.SndLimTransSnd)
            .field("SndLimTimeSnd", &self.SndLimTimeSnd)
            .field("SndLimBytesSnd", &self.SndLimBytesSnd)
            .finish()
    }
}
impl ::core::default::Default for TCP_INITIAL_RTO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TCP_INITIAL_RTO_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Rtt == other.Rtt && self.MaxSynRetransmissions == other.MaxSynRetransmissions
    }
}
impl ::core::cmp::Eq for TCP_INITIAL_RTO_PARAMETERS {}
impl ::core::fmt::Debug for TCP_INITIAL_RTO_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_INITIAL_RTO_PARAMETERS").field("Rtt", &self.Rtt).field("MaxSynRetransmissions", &self.MaxSynRetransmissions).finish()
    }
}
impl ::core::default::Default for TCP_OPT_FASTOPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCP_OPT_MSS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCP_OPT_SACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCP_OPT_SACK_PERMITTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCP_OPT_TS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCP_OPT_UNKNOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCP_OPT_WS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TIMESTAMPING_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIMESTAMPING_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.TxTimestampsBuffered == other.TxTimestampsBuffered
    }
}
impl ::core::cmp::Eq for TIMESTAMPING_CONFIG {}
impl ::core::fmt::Debug for TIMESTAMPING_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMESTAMPING_CONFIG").field("Flags", &self.Flags).field("TxTimestampsBuffered", &self.TxTimestampsBuffered).finish()
    }
}
impl ::core::default::Default for TIMEVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIMEVAL {
    fn eq(&self, other: &Self) -> bool {
        self.tv_sec == other.tv_sec && self.tv_usec == other.tv_usec
    }
}
impl ::core::cmp::Eq for TIMEVAL {}
impl ::core::fmt::Debug for TIMEVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMEVAL").field("tv_sec", &self.tv_sec).field("tv_usec", &self.tv_usec).finish()
    }
}
impl ::core::default::Default for TRANSMIT_FILE_BUFFERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSMIT_FILE_BUFFERS {
    fn eq(&self, other: &Self) -> bool {
        self.Head == other.Head && self.HeadLength == other.HeadLength && self.Tail == other.Tail && self.TailLength == other.TailLength
    }
}
impl ::core::cmp::Eq for TRANSMIT_FILE_BUFFERS {}
impl ::core::fmt::Debug for TRANSMIT_FILE_BUFFERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMIT_FILE_BUFFERS").field("Head", &self.Head).field("HeadLength", &self.HeadLength).field("Tail", &self.Tail).field("TailLength", &self.TailLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSMIT_PACKETS_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TRANSPORT_SETTING_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSPORT_SETTING_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid
    }
}
impl ::core::cmp::Eq for TRANSPORT_SETTING_ID {}
impl ::core::fmt::Debug for TRANSPORT_SETTING_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_SETTING_ID").field("Guid", &self.Guid).finish()
    }
}
impl ::core::default::Default for TUNNEL_SUB_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TUNNEL_SUB_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TUNNEL_SUB_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VLAN_TAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCE_DEVICELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCE_DEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.numDevice == other.numDevice && self.Device == other.Device
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCE_DEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCE_DEVICELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCE_DEVICELIST").field("numDevice", &self.numDevice).field("Device", &self.Device).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCE_IRDA_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCE_IRDA_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.irdaDeviceID == other.irdaDeviceID && self.irdaDeviceName == other.irdaDeviceName && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCE_IRDA_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCE_IRDA_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCE_IRDA_DEVICE_INFO").field("irdaDeviceID", &self.irdaDeviceID).field("irdaDeviceName", &self.irdaDeviceName).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWS_DEVICELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINDOWS_DEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.numDevice == other.numDevice && self.Device == other.Device
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINDOWS_DEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINDOWS_DEVICELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_DEVICELIST").field("numDevice", &self.numDevice).field("Device", &self.Device).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWS_IAS_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWS_IAS_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWS_IRDA_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINDOWS_IRDA_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.irdaDeviceID == other.irdaDeviceID && self.irdaDeviceName == other.irdaDeviceName && self.irdaDeviceHints1 == other.irdaDeviceHints1 && self.irdaDeviceHints2 == other.irdaDeviceHints2 && self.irdaCharSet == other.irdaCharSet
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINDOWS_IRDA_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINDOWS_IRDA_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IRDA_DEVICE_INFO").field("irdaDeviceID", &self.irdaDeviceID).field("irdaDeviceName", &self.irdaDeviceName).field("irdaDeviceHints1", &self.irdaDeviceHints1).field("irdaDeviceHints2", &self.irdaDeviceHints2).field("irdaCharSet", &self.irdaCharSet).finish()
    }
}
impl ::core::default::Default for WSABUF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSABUF {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.buf == other.buf
    }
}
impl ::core::cmp::Eq for WSABUF {}
impl ::core::fmt::Debug for WSABUF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSABUF").field("len", &self.len).field("buf", &self.buf).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WSACOMPLETION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSACOMPLETIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSACOMPLETIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSACOMPLETIONTYPE").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSADATA {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wHighVersion == other.wHighVersion && self.iMaxSockets == other.iMaxSockets && self.iMaxUdpDg == other.iMaxUdpDg && self.lpVendorInfo == other.lpVendorInfo && self.szDescription == other.szDescription && self.szSystemStatus == other.szSystemStatus
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSADATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSADATA").field("wVersion", &self.wVersion).field("wHighVersion", &self.wHighVersion).field("iMaxSockets", &self.iMaxSockets).field("iMaxUdpDg", &self.iMaxUdpDg).field("lpVendorInfo", &self.lpVendorInfo).field("szDescription", &self.szDescription).field("szSystemStatus", &self.szSystemStatus).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSADATA {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wHighVersion == other.wHighVersion && self.szDescription == other.szDescription && self.szSystemStatus == other.szSystemStatus && self.iMaxSockets == other.iMaxSockets && self.iMaxUdpDg == other.iMaxUdpDg && self.lpVendorInfo == other.lpVendorInfo
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSADATA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSADATA").field("wVersion", &self.wVersion).field("wHighVersion", &self.wHighVersion).field("szDescription", &self.szDescription).field("szSystemStatus", &self.szSystemStatus).field("iMaxSockets", &self.iMaxSockets).field("iMaxUdpDg", &self.iMaxUdpDg).field("lpVendorInfo", &self.lpVendorInfo).finish()
    }
}
impl ::core::default::Default for WSAECOMPARATOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSAECOMPARATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSAECOMPARATOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSAESETSERVICEOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSAESETSERVICEOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSAESETSERVICEOP").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSAMSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSAMSG {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.namelen == other.namelen && self.lpBuffers == other.lpBuffers && self.dwBufferCount == other.dwBufferCount && self.Control == other.Control && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSAMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSAMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAMSG").field("name", &self.name).field("namelen", &self.namelen).field("lpBuffers", &self.lpBuffers).field("dwBufferCount", &self.dwBufferCount).field("Control", &self.Control).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSANAMESPACE_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSANAMESPACE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId && self.dwNameSpace == other.dwNameSpace && self.fActive == other.fActive && self.dwVersion == other.dwVersion && self.lpszIdentifier == other.lpszIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSANAMESPACE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSANAMESPACE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANAMESPACE_INFOA").field("NSProviderId", &self.NSProviderId).field("dwNameSpace", &self.dwNameSpace).field("fActive", &self.fActive).field("dwVersion", &self.dwVersion).field("lpszIdentifier", &self.lpszIdentifier).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for WSANAMESPACE_INFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for WSANAMESPACE_INFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId && self.dwNameSpace == other.dwNameSpace && self.fActive == other.fActive && self.dwVersion == other.dwVersion && self.lpszIdentifier == other.lpszIdentifier && self.ProviderSpecific == other.ProviderSpecific
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for WSANAMESPACE_INFOEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for WSANAMESPACE_INFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANAMESPACE_INFOEXA").field("NSProviderId", &self.NSProviderId).field("dwNameSpace", &self.dwNameSpace).field("fActive", &self.fActive).field("dwVersion", &self.dwVersion).field("lpszIdentifier", &self.lpszIdentifier).field("ProviderSpecific", &self.ProviderSpecific).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for WSANAMESPACE_INFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for WSANAMESPACE_INFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId && self.dwNameSpace == other.dwNameSpace && self.fActive == other.fActive && self.dwVersion == other.dwVersion && self.lpszIdentifier == other.lpszIdentifier && self.ProviderSpecific == other.ProviderSpecific
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for WSANAMESPACE_INFOEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for WSANAMESPACE_INFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANAMESPACE_INFOEXW").field("NSProviderId", &self.NSProviderId).field("dwNameSpace", &self.dwNameSpace).field("fActive", &self.fActive).field("dwVersion", &self.dwVersion).field("lpszIdentifier", &self.lpszIdentifier).field("ProviderSpecific", &self.ProviderSpecific).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSANAMESPACE_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSANAMESPACE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId && self.dwNameSpace == other.dwNameSpace && self.fActive == other.fActive && self.dwVersion == other.dwVersion && self.lpszIdentifier == other.lpszIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSANAMESPACE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSANAMESPACE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANAMESPACE_INFOW").field("NSProviderId", &self.NSProviderId).field("dwNameSpace", &self.dwNameSpace).field("fActive", &self.fActive).field("dwVersion", &self.dwVersion).field("lpszIdentifier", &self.lpszIdentifier).finish()
    }
}
impl ::core::default::Default for WSANETWORKEVENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSANETWORKEVENTS {
    fn eq(&self, other: &Self) -> bool {
        self.lNetworkEvents == other.lNetworkEvents && self.iErrorCode == other.iErrorCode
    }
}
impl ::core::cmp::Eq for WSANETWORKEVENTS {}
impl ::core::fmt::Debug for WSANETWORKEVENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANETWORKEVENTS").field("lNetworkEvents", &self.lNetworkEvents).field("iErrorCode", &self.iErrorCode).finish()
    }
}
impl ::core::default::Default for WSANSCLASSINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSANSCLASSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpszName == other.lpszName && self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for WSANSCLASSINFOA {}
impl ::core::fmt::Debug for WSANSCLASSINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANSCLASSINFOA").field("lpszName", &self.lpszName).field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("lpValue", &self.lpValue).finish()
    }
}
impl ::core::default::Default for WSANSCLASSINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSANSCLASSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpszName == other.lpszName && self.dwNameSpace == other.dwNameSpace && self.dwValueType == other.dwValueType && self.dwValueSize == other.dwValueSize && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for WSANSCLASSINFOW {}
impl ::core::fmt::Debug for WSANSCLASSINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANSCLASSINFOW").field("lpszName", &self.lpszName).field("dwNameSpace", &self.dwNameSpace).field("dwValueType", &self.dwValueType).field("dwValueSize", &self.dwValueSize).field("lpValue", &self.lpValue).finish()
    }
}
impl ::core::default::Default for WSAPOLLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSAPOLLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.result == other.result && self.fds == other.fds && self.timeout == other.timeout && self.fdArray == other.fdArray
    }
}
impl ::core::cmp::Eq for WSAPOLLDATA {}
impl ::core::fmt::Debug for WSAPOLLDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPOLLDATA").field("result", &self.result).field("fds", &self.fds).field("timeout", &self.timeout).field("fdArray", &self.fdArray).finish()
    }
}
impl ::core::default::Default for WSAPOLLFD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSAPOLLFD {
    fn eq(&self, other: &Self) -> bool {
        self.fd == other.fd && self.events == other.events && self.revents == other.revents
    }
}
impl ::core::cmp::Eq for WSAPOLLFD {}
impl ::core::fmt::Debug for WSAPOLLFD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPOLLFD").field("fd", &self.fd).field("events", &self.events).field("revents", &self.revents).finish()
    }
}
impl ::core::default::Default for WSAPROTOCOLCHAIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSAPROTOCOLCHAIN {
    fn eq(&self, other: &Self) -> bool {
        self.ChainLen == other.ChainLen && self.ChainEntries == other.ChainEntries
    }
}
impl ::core::cmp::Eq for WSAPROTOCOLCHAIN {}
impl ::core::fmt::Debug for WSAPROTOCOLCHAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPROTOCOLCHAIN").field("ChainLen", &self.ChainLen).field("ChainEntries", &self.ChainEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSAPROTOCOL_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSAPROTOCOL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags1 == other.dwServiceFlags1
            && self.dwServiceFlags2 == other.dwServiceFlags2
            && self.dwServiceFlags3 == other.dwServiceFlags3
            && self.dwServiceFlags4 == other.dwServiceFlags4
            && self.dwProviderFlags == other.dwProviderFlags
            && self.ProviderId == other.ProviderId
            && self.dwCatalogEntryId == other.dwCatalogEntryId
            && self.ProtocolChain == other.ProtocolChain
            && self.iVersion == other.iVersion
            && self.iAddressFamily == other.iAddressFamily
            && self.iMaxSockAddr == other.iMaxSockAddr
            && self.iMinSockAddr == other.iMinSockAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
            && self.iProtocolMaxOffset == other.iProtocolMaxOffset
            && self.iNetworkByteOrder == other.iNetworkByteOrder
            && self.iSecurityScheme == other.iSecurityScheme
            && self.dwMessageSize == other.dwMessageSize
            && self.dwProviderReserved == other.dwProviderReserved
            && self.szProtocol == other.szProtocol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSAPROTOCOL_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSAPROTOCOL_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPROTOCOL_INFOA")
            .field("dwServiceFlags1", &self.dwServiceFlags1)
            .field("dwServiceFlags2", &self.dwServiceFlags2)
            .field("dwServiceFlags3", &self.dwServiceFlags3)
            .field("dwServiceFlags4", &self.dwServiceFlags4)
            .field("dwProviderFlags", &self.dwProviderFlags)
            .field("ProviderId", &self.ProviderId)
            .field("dwCatalogEntryId", &self.dwCatalogEntryId)
            .field("ProtocolChain", &self.ProtocolChain)
            .field("iVersion", &self.iVersion)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("iProtocolMaxOffset", &self.iProtocolMaxOffset)
            .field("iNetworkByteOrder", &self.iNetworkByteOrder)
            .field("iSecurityScheme", &self.iSecurityScheme)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("dwProviderReserved", &self.dwProviderReserved)
            .field("szProtocol", &self.szProtocol)
            .finish()
    }
}
impl ::core::default::Default for WSAPROTOCOL_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSAPROTOCOL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags1 == other.dwServiceFlags1
            && self.dwServiceFlags2 == other.dwServiceFlags2
            && self.dwServiceFlags3 == other.dwServiceFlags3
            && self.dwServiceFlags4 == other.dwServiceFlags4
            && self.dwProviderFlags == other.dwProviderFlags
            && self.ProviderId == other.ProviderId
            && self.dwCatalogEntryId == other.dwCatalogEntryId
            && self.ProtocolChain == other.ProtocolChain
            && self.iVersion == other.iVersion
            && self.iAddressFamily == other.iAddressFamily
            && self.iMaxSockAddr == other.iMaxSockAddr
            && self.iMinSockAddr == other.iMinSockAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
            && self.iProtocolMaxOffset == other.iProtocolMaxOffset
            && self.iNetworkByteOrder == other.iNetworkByteOrder
            && self.iSecurityScheme == other.iSecurityScheme
            && self.dwMessageSize == other.dwMessageSize
            && self.dwProviderReserved == other.dwProviderReserved
            && self.szProtocol == other.szProtocol
    }
}
impl ::core::cmp::Eq for WSAPROTOCOL_INFOW {}
impl ::core::fmt::Debug for WSAPROTOCOL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPROTOCOL_INFOW")
            .field("dwServiceFlags1", &self.dwServiceFlags1)
            .field("dwServiceFlags2", &self.dwServiceFlags2)
            .field("dwServiceFlags3", &self.dwServiceFlags3)
            .field("dwServiceFlags4", &self.dwServiceFlags4)
            .field("dwProviderFlags", &self.dwProviderFlags)
            .field("ProviderId", &self.ProviderId)
            .field("dwCatalogEntryId", &self.dwCatalogEntryId)
            .field("ProtocolChain", &self.ProtocolChain)
            .field("iVersion", &self.iVersion)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("iProtocolMaxOffset", &self.iProtocolMaxOffset)
            .field("iNetworkByteOrder", &self.iNetworkByteOrder)
            .field("iSecurityScheme", &self.iSecurityScheme)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("dwProviderReserved", &self.dwProviderReserved)
            .field("szProtocol", &self.szProtocol)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for WSAQUERYSET2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for WSAQUERYSET2A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpszServiceInstanceName == other.lpszServiceInstanceName && self.lpVersion == other.lpVersion && self.lpszComment == other.lpszComment && self.dwNameSpace == other.dwNameSpace && self.lpNSProviderId == other.lpNSProviderId && self.lpszContext == other.lpszContext && self.dwNumberOfProtocols == other.dwNumberOfProtocols && self.lpafpProtocols == other.lpafpProtocols && self.lpszQueryString == other.lpszQueryString && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs && self.lpcsaBuffer == other.lpcsaBuffer && self.dwOutputFlags == other.dwOutputFlags && self.lpBlob == other.lpBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for WSAQUERYSET2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for WSAQUERYSET2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAQUERYSET2A")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for WSAQUERYSET2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for WSAQUERYSET2W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpszServiceInstanceName == other.lpszServiceInstanceName && self.lpVersion == other.lpVersion && self.lpszComment == other.lpszComment && self.dwNameSpace == other.dwNameSpace && self.lpNSProviderId == other.lpNSProviderId && self.lpszContext == other.lpszContext && self.dwNumberOfProtocols == other.dwNumberOfProtocols && self.lpafpProtocols == other.lpafpProtocols && self.lpszQueryString == other.lpszQueryString && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs && self.lpcsaBuffer == other.lpcsaBuffer && self.dwOutputFlags == other.dwOutputFlags && self.lpBlob == other.lpBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for WSAQUERYSET2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for WSAQUERYSET2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAQUERYSET2W")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for WSAQUERYSETA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for WSAQUERYSETA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpszServiceInstanceName == other.lpszServiceInstanceName && self.lpServiceClassId == other.lpServiceClassId && self.lpVersion == other.lpVersion && self.lpszComment == other.lpszComment && self.dwNameSpace == other.dwNameSpace && self.lpNSProviderId == other.lpNSProviderId && self.lpszContext == other.lpszContext && self.dwNumberOfProtocols == other.dwNumberOfProtocols && self.lpafpProtocols == other.lpafpProtocols && self.lpszQueryString == other.lpszQueryString && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs && self.lpcsaBuffer == other.lpcsaBuffer && self.dwOutputFlags == other.dwOutputFlags && self.lpBlob == other.lpBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for WSAQUERYSETA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for WSAQUERYSETA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAQUERYSETA")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpServiceClassId", &self.lpServiceClassId)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for WSAQUERYSETW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for WSAQUERYSETW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpszServiceInstanceName == other.lpszServiceInstanceName && self.lpServiceClassId == other.lpServiceClassId && self.lpVersion == other.lpVersion && self.lpszComment == other.lpszComment && self.dwNameSpace == other.dwNameSpace && self.lpNSProviderId == other.lpNSProviderId && self.lpszContext == other.lpszContext && self.dwNumberOfProtocols == other.dwNumberOfProtocols && self.lpafpProtocols == other.lpafpProtocols && self.lpszQueryString == other.lpszQueryString && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs && self.lpcsaBuffer == other.lpcsaBuffer && self.dwOutputFlags == other.dwOutputFlags && self.lpBlob == other.lpBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for WSAQUERYSETW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for WSAQUERYSETW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAQUERYSETW")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpServiceClassId", &self.lpServiceClassId)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WSASENDMSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSASERVICECLASSINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSASERVICECLASSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceClassId == other.lpServiceClassId && self.lpszServiceClassName == other.lpszServiceClassName && self.dwCount == other.dwCount && self.lpClassInfos == other.lpClassInfos
    }
}
impl ::core::cmp::Eq for WSASERVICECLASSINFOA {}
impl ::core::fmt::Debug for WSASERVICECLASSINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSASERVICECLASSINFOA").field("lpServiceClassId", &self.lpServiceClassId).field("lpszServiceClassName", &self.lpszServiceClassName).field("dwCount", &self.dwCount).field("lpClassInfos", &self.lpClassInfos).finish()
    }
}
impl ::core::default::Default for WSASERVICECLASSINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSASERVICECLASSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceClassId == other.lpServiceClassId && self.lpszServiceClassName == other.lpszServiceClassName && self.dwCount == other.dwCount && self.lpClassInfos == other.lpClassInfos
    }
}
impl ::core::cmp::Eq for WSASERVICECLASSINFOW {}
impl ::core::fmt::Debug for WSASERVICECLASSINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSASERVICECLASSINFOW").field("lpServiceClassId", &self.lpServiceClassId).field("lpszServiceClassName", &self.lpszServiceClassName).field("dwCount", &self.dwCount).field("lpClassInfos", &self.lpClassInfos).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSATHREADID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSATHREADID {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadHandle == other.ThreadHandle && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSATHREADID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSATHREADID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSATHREADID").field("ThreadHandle", &self.ThreadHandle).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WSAVERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSAVERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.ecHow == other.ecHow
    }
}
impl ::core::cmp::Eq for WSAVERSION {}
impl ::core::fmt::Debug for WSAVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAVERSION").field("dwVersion", &self.dwVersion).field("ecHow", &self.ecHow).finish()
    }
}
impl ::core::default::Default for WSA_COMPATIBILITY_BEHAVIOR_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSA_COMPATIBILITY_BEHAVIOR_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSA_COMPATIBILITY_BEHAVIOR_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSA_COMPATIBILITY_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSA_COMPATIBILITY_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.BehaviorId == other.BehaviorId && self.TargetOsVersion == other.TargetOsVersion
    }
}
impl ::core::cmp::Eq for WSA_COMPATIBILITY_MODE {}
impl ::core::fmt::Debug for WSA_COMPATIBILITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSA_COMPATIBILITY_MODE").field("BehaviorId", &self.BehaviorId).field("TargetOsVersion", &self.TargetOsVersion).finish()
    }
}
impl ::core::default::Default for WSA_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSA_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSA_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSC_PROVIDER_AUDIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSC_PROVIDER_AUDIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RecordSize == other.RecordSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WSC_PROVIDER_AUDIT_INFO {}
impl ::core::fmt::Debug for WSC_PROVIDER_AUDIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSC_PROVIDER_AUDIT_INFO").field("RecordSize", &self.RecordSize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WSC_PROVIDER_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSC_PROVIDER_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_PROVIDER_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wHighVersion == other.wHighVersion && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for WSPDATA {}
impl ::core::fmt::Debug for WSPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSPDATA").field("wVersion", &self.wVersion).field("wHighVersion", &self.wHighVersion).field("szDescription", &self.szDescription).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WSPPROC_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSPUPCALLTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for eWINDOW_ADVANCE_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eWINDOW_ADVANCE_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eWINDOW_ADVANCE_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for netent {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for netent {
    fn eq(&self, other: &Self) -> bool {
        self.n_name == other.n_name && self.n_aliases == other.n_aliases && self.n_addrtype == other.n_addrtype && self.n_net == other.n_net
    }
}
impl ::core::cmp::Eq for netent {}
impl ::core::fmt::Debug for netent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("netent").field("n_name", &self.n_name).field("n_aliases", &self.n_aliases).field("n_addrtype", &self.n_addrtype).field("n_net", &self.n_net).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for sockaddr_gen {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for sockaddr_in6_old {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for sockproto {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for sockproto {
    fn eq(&self, other: &Self) -> bool {
        self.sp_family == other.sp_family && self.sp_protocol == other.sp_protocol
    }
}
impl ::core::cmp::Eq for sockproto {}
impl ::core::fmt::Debug for sockproto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockproto").field("sp_family", &self.sp_family).field("sp_protocol", &self.sp_protocol).finish()
    }
}
impl ::core::default::Default for tcp_keepalive {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for tcp_keepalive {
    fn eq(&self, other: &Self) -> bool {
        self.onoff == other.onoff && self.keepalivetime == other.keepalivetime && self.keepaliveinterval == other.keepaliveinterval
    }
}
impl ::core::cmp::Eq for tcp_keepalive {}
impl ::core::fmt::Debug for tcp_keepalive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_keepalive").field("onoff", &self.onoff).field("keepalivetime", &self.keepalivetime).field("keepaliveinterval", &self.keepaliveinterval).finish()
    }
}
