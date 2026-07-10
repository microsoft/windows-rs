pub const BROADCAST_NODETYPE: u32 = 1;
pub const DEFAULT_MINIMUM_ENTITIES: u32 = 32;
pub type FIXED_INFO = FIXED_INFO_W2KSP1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FIXED_INFO_W2KSP1 {
    pub HostName: [i8; 132],
    pub DomainName: [i8; 132],
    pub CurrentDnsServer: PIP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
    pub NodeType: u32,
    pub ScopeId: [i8; 260],
    pub EnableRouting: u32,
    pub EnableProxy: u32,
    pub EnableDns: u32,
}
impl Default for FIXED_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GAA_FLAG_INCLUDE_ALL_COMPARTMENTS: u32 = 512;
pub const GAA_FLAG_INCLUDE_ALL_INTERFACES: u32 = 256;
pub const GAA_FLAG_INCLUDE_GATEWAYS: u32 = 128;
pub const GAA_FLAG_INCLUDE_PREFIX: u32 = 16;
pub const GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER: u32 = 1024;
pub const GAA_FLAG_INCLUDE_WINS_INFO: u32 = 64;
pub const GAA_FLAG_SKIP_ANYCAST: u32 = 2;
pub const GAA_FLAG_SKIP_DNS_INFO: u32 = 2048;
pub const GAA_FLAG_SKIP_DNS_SERVER: u32 = 8;
pub const GAA_FLAG_SKIP_FRIENDLY_NAME: u32 = 32;
pub const GAA_FLAG_SKIP_MULTICAST: u32 = 4;
pub const GAA_FLAG_SKIP_UNICAST: u32 = 1;
pub const HYBRID_NODETYPE: u32 = 8;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
pub type IP_ADAPTER_ADDRESSES = IP_ADAPTER_ADDRESSES_LH;
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ADDRESSES_LH {
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_0,
    pub Next: *mut Self,
    pub AdapterName: super::winnt::PCHAR,
    pub FirstUnicastAddress: PIP_ADAPTER_UNICAST_ADDRESS_LH,
    pub FirstAnycastAddress: PIP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: PIP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: super::winnt::PWCHAR,
    pub Description: super::winnt::PWCHAR,
    pub FriendlyName: super::winnt::PWCHAR,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: u32,
    pub Anonymous2: IP_ADAPTER_ADDRESSES_LH_1,
    pub Mtu: u32,
    pub IfType: super::ipifcons::IFTYPE,
    pub OperStatus: super::ifdef::IF_OPER_STATUS,
    pub Ipv6IfIndex: super::ifdef::IF_INDEX,
    pub ZoneIndices: [u32; 16],
    pub FirstPrefix: PIP_ADAPTER_PREFIX_XP,
    pub TransmitLinkSpeed: u64,
    pub ReceiveLinkSpeed: u64,
    pub FirstWinsServerAddress: PIP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub FirstGatewayAddress: PIP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Ipv4Metric: u32,
    pub Ipv6Metric: u32,
    pub Luid: super::ifdef::IF_LUID,
    pub Dhcpv4Server: super::ws2::SOCKET_ADDRESS,
    pub CompartmentId: super::ifdef::NET_IF_COMPARTMENT_ID,
    pub NetworkGuid: super::ifdef::NET_IF_NETWORK_GUID,
    pub ConnectionType: super::ifdef::NET_IF_CONNECTION_TYPE,
    pub TunnelType: super::ifdef::TUNNEL_TYPE,
    pub Dhcpv6Server: super::ws2::SOCKET_ADDRESS,
    pub Dhcpv6ClientDuid: [u8; 130],
    pub Dhcpv6ClientDuidLength: u32,
    pub Dhcpv6Iaid: u32,
    pub FirstDnsSuffix: PIP_ADAPTER_DNS_SUFFIX,
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_0_0,
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_ADDRESSES_LH_0_0 {
    pub Length: u32,
    pub IfIndex: super::ifdef::IF_INDEX,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_LH_1 {
    pub Flags: u32,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_1_0,
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_LH_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_ADDRESSES_LH_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ADDRESSES_XP {
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0,
    pub Next: *mut Self,
    pub AdapterName: super::winnt::PCHAR,
    pub FirstUnicastAddress: PIP_ADAPTER_UNICAST_ADDRESS_XP,
    pub FirstAnycastAddress: PIP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: PIP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: super::winnt::PWCHAR,
    pub Description: super::winnt::PWCHAR,
    pub FriendlyName: super::winnt::PWCHAR,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: u32,
    pub Flags: u32,
    pub Mtu: u32,
    pub IfType: u32,
    pub OperStatus: super::ifdef::IF_OPER_STATUS,
    pub Ipv6IfIndex: u32,
    pub ZoneIndices: [u32; 16],
    pub FirstPrefix: PIP_ADAPTER_PREFIX_XP,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0_0,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_ADDRESSES_XP_0_0 {
    pub Length: u32,
    pub IfIndex: u32,
}
pub const IP_ADAPTER_ADDRESS_DNS_ELIGIBLE: u32 = 1;
pub const IP_ADAPTER_ADDRESS_TRANSIENT: u32 = 2;
#[cfg(feature = "ws2")]
pub type IP_ADAPTER_ANYCAST_ADDRESS = IP_ADAPTER_ANYCAST_ADDRESS_XP;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0,
    pub Next: *mut Self,
    pub Address: super::ws2::SOCKET_ADDRESS,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
pub const IP_ADAPTER_DDNS_ENABLED: u32 = 1;
pub const IP_ADAPTER_DHCP_ENABLED: u32 = 4;
#[cfg(feature = "ws2")]
pub type IP_ADAPTER_DNS_SERVER_ADDRESS = IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0,
    pub Next: *mut Self,
    pub Address: super::ws2::SOCKET_ADDRESS,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_DNS_SUFFIX {
    pub Next: *mut Self,
    pub String: [u16; 256],
}
impl Default for IP_ADAPTER_DNS_SUFFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "ws2")]
pub type IP_ADAPTER_GATEWAY_ADDRESS = IP_ADAPTER_GATEWAY_ADDRESS_LH;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0,
    pub Next: *mut Self,
    pub Address: super::ws2::SOCKET_ADDRESS,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "corecrt")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_INFO {
    pub Next: *mut Self,
    pub ComboIndex: u32,
    pub AdapterName: [i8; 260],
    pub Description: [i8; 132],
    pub AddressLength: u32,
    pub Address: [u8; 8],
    pub Index: u32,
    pub Type: u32,
    pub DhcpEnabled: u32,
    pub CurrentIpAddress: PIP_ADDR_STRING,
    pub IpAddressList: IP_ADDR_STRING,
    pub GatewayList: IP_ADDR_STRING,
    pub DhcpServer: IP_ADDR_STRING,
    pub HaveWins: windows_sys::core::BOOL,
    pub PrimaryWinsServer: IP_ADDR_STRING,
    pub SecondaryWinsServer: IP_ADDR_STRING,
    pub LeaseObtained: super::corecrt::time_t,
    pub LeaseExpires: super::corecrt::time_t,
}
#[cfg(feature = "corecrt")]
impl Default for IP_ADAPTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_ADAPTER_IPV4_ENABLED: u32 = 128;
pub const IP_ADAPTER_IPV6_ENABLED: u32 = 256;
pub const IP_ADAPTER_IPV6_MANAGE_ADDRESS_CONFIG: u32 = 512;
pub const IP_ADAPTER_IPV6_OTHER_STATEFUL_CONFIG: u32 = 32;
#[cfg(feature = "ws2")]
pub type IP_ADAPTER_MULTICAST_ADDRESS = IP_ADAPTER_MULTICAST_ADDRESS_XP;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0,
    pub Next: *mut Self,
    pub Address: super::ws2::SOCKET_ADDRESS,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
pub const IP_ADAPTER_NETBIOS_OVER_TCPIP_ENABLED: u32 = 64;
pub const IP_ADAPTER_NO_MULTICAST: u32 = 16;
#[cfg(feature = "ws2")]
pub type IP_ADAPTER_PREFIX = IP_ADAPTER_PREFIX_XP;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_PREFIX_XP {
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0,
    pub Next: *mut Self,
    pub Address: super::ws2::SOCKET_ADDRESS,
    pub PrefixLength: u32,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_PREFIX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_PREFIX_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0_0,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_PREFIX_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_PREFIX_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
pub const IP_ADAPTER_RECEIVE_ONLY: u32 = 8;
pub const IP_ADAPTER_REGISTER_ADAPTER_SUFFIX: u32 = 2;
#[cfg(all(feature = "nldef", feature = "ws2"))]
pub type IP_ADAPTER_UNICAST_ADDRESS = IP_ADAPTER_UNICAST_ADDRESS_LH;
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0,
    pub Next: *mut Self,
    pub Address: super::ws2::SOCKET_ADDRESS,
    pub PrefixOrigin: IP_PREFIX_ORIGIN,
    pub SuffixOrigin: IP_SUFFIX_ORIGIN,
    pub DadState: IP_DAD_STATE,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub LeaseLifetime: u32,
    pub OnLinkPrefixLength: u8,
}
#[cfg(all(feature = "nldef", feature = "ws2"))]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "nldef", feature = "ws2"))]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0,
    pub Next: *mut Self,
    pub Address: super::ws2::SOCKET_ADDRESS,
    pub PrefixOrigin: IP_PREFIX_ORIGIN,
    pub SuffixOrigin: IP_SUFFIX_ORIGIN,
    pub DadState: IP_DAD_STATE,
    pub ValidLifetime: u32,
    pub PreferredLifetime: u32,
    pub LeaseLifetime: u32,
}
#[cfg(all(feature = "nldef", feature = "ws2"))]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "nldef", feature = "ws2"))]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(feature = "ws2")]
pub type IP_ADAPTER_WINS_SERVER_ADDRESS = IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0,
    pub Next: *mut Self,
    pub Address: super::ws2::SOCKET_ADDRESS,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    pub Alignment: u64,
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0,
}
#[cfg(feature = "ws2")]
impl Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    pub Length: u32,
    pub Reserved: u32,
}
pub type IP_ADDRESS_STRING = IP_MASK_STRING;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IP_ADDR_STRING {
    pub Next: *mut Self,
    pub IpAddress: IP_ADDRESS_STRING,
    pub IpMask: IP_MASK_STRING,
    pub Context: u32,
}
impl Default for IP_ADDR_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "nldef")]
pub type IP_DAD_STATE = super::nldef::NL_DAD_STATE;
pub type IP_INTERFACE_NAME_INFO = IP_INTERFACE_NAME_INFO_W2KSP1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IP_INTERFACE_NAME_INFO_W2KSP1 {
    pub Index: u32,
    pub MediaType: u32,
    pub ConnectionType: u8,
    pub AccessType: u8,
    pub DeviceGuid: windows_sys::core::GUID,
    pub InterfaceGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IP_MASK_STRING {
    pub String: [i8; 16],
}
impl Default for IP_MASK_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IP_PER_ADAPTER_INFO = IP_PER_ADAPTER_INFO_W2KSP1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IP_PER_ADAPTER_INFO_W2KSP1 {
    pub AutoconfigEnabled: u32,
    pub AutoconfigActive: u32,
    pub CurrentDnsServer: PIP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
}
impl Default for IP_PER_ADAPTER_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "nldef")]
pub type IP_PREFIX_ORIGIN = super::nldef::NL_PREFIX_ORIGIN;
#[cfg(feature = "nldef")]
pub type IP_SUFFIX_ORIGIN = super::nldef::NL_SUFFIX_ORIGIN;
pub const MAX_ADAPTER_ADDRESS_LENGTH: u32 = 8;
pub const MAX_ADAPTER_DESCRIPTION_LENGTH: u32 = 128;
pub const MAX_ADAPTER_NAME_LENGTH: u32 = 256;
pub const MAX_DHCPV6_DUID_LENGTH: u32 = 130;
pub const MAX_DNS_SUFFIX_STRING_LENGTH: u32 = 256;
pub const MAX_DOMAIN_NAME_LEN: u32 = 128;
pub const MAX_HOSTNAME_LEN: u32 = 128;
pub const MAX_SCOPE_ID_LEN: u32 = 256;
pub const MIXED_NODETYPE: u32 = 4;
pub const PEER_TO_PEER_NODETYPE: u32 = 2;
pub type PFIXED_INFO = *mut FIXED_INFO_W2KSP1;
pub type PFIXED_INFO_W2KSP1 = *mut FIXED_INFO_W2KSP1;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
pub type PIP_ADAPTER_ADDRESSES = *mut IP_ADAPTER_ADDRESSES_LH;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "winnt", feature = "ws2"))]
pub type PIP_ADAPTER_ADDRESSES_LH = *mut IP_ADAPTER_ADDRESSES_LH;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "winnt", feature = "ws2"))]
pub type PIP_ADAPTER_ADDRESSES_XP = *mut IP_ADAPTER_ADDRESSES_XP;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_ANYCAST_ADDRESS = *mut IP_ADAPTER_ANYCAST_ADDRESS_XP;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_ANYCAST_ADDRESS_XP = *mut IP_ADAPTER_ANYCAST_ADDRESS_XP;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_DNS_SERVER_ADDRESS = *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_DNS_SERVER_ADDRESS_XP = *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
pub type PIP_ADAPTER_DNS_SUFFIX = *mut IP_ADAPTER_DNS_SUFFIX;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_GATEWAY_ADDRESS = *mut IP_ADAPTER_GATEWAY_ADDRESS_LH;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_GATEWAY_ADDRESS_LH = *mut IP_ADAPTER_GATEWAY_ADDRESS_LH;
#[cfg(feature = "corecrt")]
pub type PIP_ADAPTER_INFO = *mut IP_ADAPTER_INFO;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_MULTICAST_ADDRESS = *mut IP_ADAPTER_MULTICAST_ADDRESS_XP;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_MULTICAST_ADDRESS_XP = *mut IP_ADAPTER_MULTICAST_ADDRESS_XP;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_PREFIX = *mut IP_ADAPTER_PREFIX_XP;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_PREFIX_XP = *mut IP_ADAPTER_PREFIX_XP;
#[cfg(all(feature = "nldef", feature = "ws2"))]
pub type PIP_ADAPTER_UNICAST_ADDRESS = *mut IP_ADAPTER_UNICAST_ADDRESS_LH;
#[cfg(all(feature = "nldef", feature = "ws2"))]
pub type PIP_ADAPTER_UNICAST_ADDRESS_LH = *mut IP_ADAPTER_UNICAST_ADDRESS_LH;
#[cfg(all(feature = "nldef", feature = "ws2"))]
pub type PIP_ADAPTER_UNICAST_ADDRESS_XP = *mut IP_ADAPTER_UNICAST_ADDRESS_XP;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_WINS_SERVER_ADDRESS = *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
#[cfg(feature = "ws2")]
pub type PIP_ADAPTER_WINS_SERVER_ADDRESS_LH = *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
pub type PIP_ADDRESS_STRING = *mut IP_MASK_STRING;
pub type PIP_ADDR_STRING = *mut IP_ADDR_STRING;
pub type PIP_INTERFACE_NAME_INFO = *mut IP_INTERFACE_NAME_INFO_W2KSP1;
pub type PIP_INTERFACE_NAME_INFO_W2KSP1 = *mut IP_INTERFACE_NAME_INFO_W2KSP1;
pub type PIP_MASK_STRING = *mut IP_MASK_STRING;
pub type PIP_PER_ADAPTER_INFO = *mut IP_PER_ADAPTER_INFO_W2KSP1;
pub type PIP_PER_ADAPTER_INFO_W2KSP1 = *mut IP_PER_ADAPTER_INFO_W2KSP1;
