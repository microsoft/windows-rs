pub const BROADCAST_NODETYPE: i32 = 1;
pub const DEFAULT_MINIMUM_ENTITIES: i32 = 32;
#[cfg(feature = "shared")]
pub type FIXED_INFO = FIXED_INFO_W2KSP1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct FIXED_INFO_W2KSP1 {
    pub HostName: [i8; 132],
    pub DomainName: [i8; 132],
    pub CurrentDnsServer: PIP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
    pub NodeType: super::UINT,
    pub ScopeId: [i8; 260],
    pub EnableRouting: super::UINT,
    pub EnableProxy: super::UINT,
    pub EnableDns: super::UINT,
}
#[cfg(feature = "shared")]
impl Default for FIXED_INFO_W2KSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GAA_FLAG_INCLUDE_ALL_COMPARTMENTS: i32 = 512;
pub const GAA_FLAG_INCLUDE_ALL_INTERFACES: i32 = 256;
pub const GAA_FLAG_INCLUDE_GATEWAYS: i32 = 128;
pub const GAA_FLAG_INCLUDE_PREFIX: i32 = 16;
pub const GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER: i32 = 1024;
pub const GAA_FLAG_INCLUDE_WINS_INFO: i32 = 64;
pub const GAA_FLAG_SKIP_ANYCAST: i32 = 2;
pub const GAA_FLAG_SKIP_DNS_INFO: i32 = 2048;
pub const GAA_FLAG_SKIP_DNS_SERVER: i32 = 8;
pub const GAA_FLAG_SKIP_FRIENDLY_NAME: i32 = 32;
pub const GAA_FLAG_SKIP_MULTICAST: i32 = 4;
pub const GAA_FLAG_SKIP_UNICAST: i32 = 1;
pub const HYBRID_NODETYPE: i32 = 8;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type IP_ADAPTER_ADDRESSES = IP_ADAPTER_ADDRESSES_LH;
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ADDRESSES_LH {
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_0,
    pub Next: *mut Self,
    pub AdapterName: super::PCHAR,
    pub FirstUnicastAddress: PIP_ADAPTER_UNICAST_ADDRESS_LH,
    pub FirstAnycastAddress: PIP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: PIP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: super::PWCHAR,
    pub Description: super::PWCHAR,
    pub FriendlyName: super::PWCHAR,
    pub PhysicalAddress: [super::BYTE; 8],
    pub PhysicalAddressLength: super::ULONG,
    pub Anonymous2: IP_ADAPTER_ADDRESSES_LH_1,
    pub Mtu: super::ULONG,
    pub IfType: super::IFTYPE,
    pub OperStatus: super::IF_OPER_STATUS,
    pub Ipv6IfIndex: super::IF_INDEX,
    pub ZoneIndices: [super::ULONG; 16],
    pub FirstPrefix: PIP_ADAPTER_PREFIX_XP,
    pub TransmitLinkSpeed: super::ULONG64,
    pub ReceiveLinkSpeed: super::ULONG64,
    pub FirstWinsServerAddress: PIP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    pub FirstGatewayAddress: PIP_ADAPTER_GATEWAY_ADDRESS_LH,
    pub Ipv4Metric: super::ULONG,
    pub Ipv6Metric: super::ULONG,
    pub Luid: super::IF_LUID,
    pub Dhcpv4Server: super::SOCKET_ADDRESS,
    pub CompartmentId: super::NET_IF_COMPARTMENT_ID,
    pub NetworkGuid: super::NET_IF_NETWORK_GUID,
    pub ConnectionType: super::NET_IF_CONNECTION_TYPE,
    pub TunnelType: super::TUNNEL_TYPE,
    pub Dhcpv6Server: super::SOCKET_ADDRESS,
    pub Dhcpv6ClientDuid: [super::BYTE; 130],
    pub Dhcpv6ClientDuidLength: super::ULONG,
    pub Dhcpv6Iaid: super::ULONG,
    pub FirstDnsSuffix: PIP_ADAPTER_DNS_SUFFIX,
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_LH_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_0_0,
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_ADDRESSES_LH_0_0 {
    pub Length: super::ULONG,
    pub IfIndex: super::IF_INDEX,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_LH_1 {
    pub Flags: super::ULONG,
    pub Anonymous: IP_ADAPTER_ADDRESSES_LH_1_0,
}
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_LH_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_ADDRESSES_LH_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ADDRESSES_XP {
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0,
    pub Next: *mut Self,
    pub AdapterName: super::PCHAR,
    pub FirstUnicastAddress: PIP_ADAPTER_UNICAST_ADDRESS_XP,
    pub FirstAnycastAddress: PIP_ADAPTER_ANYCAST_ADDRESS_XP,
    pub FirstMulticastAddress: PIP_ADAPTER_MULTICAST_ADDRESS_XP,
    pub FirstDnsServerAddress: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    pub DnsSuffix: super::PWCHAR,
    pub Description: super::PWCHAR,
    pub FriendlyName: super::PWCHAR,
    pub PhysicalAddress: [super::BYTE; 8],
    pub PhysicalAddressLength: super::DWORD,
    pub Flags: super::DWORD,
    pub Mtu: super::DWORD,
    pub IfType: super::DWORD,
    pub OperStatus: super::IF_OPER_STATUS,
    pub Ipv6IfIndex: super::DWORD,
    pub ZoneIndices: [super::DWORD; 16],
    pub FirstPrefix: PIP_ADAPTER_PREFIX_XP,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ADDRESSES_XP_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_ADDRESSES_XP_0_0,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
impl Default for IP_ADAPTER_ADDRESSES_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_ADDRESSES_XP_0_0 {
    pub Length: super::ULONG,
    pub IfIndex: super::DWORD,
}
pub const IP_ADAPTER_ADDRESS_DNS_ELIGIBLE: i32 = 1;
pub const IP_ADAPTER_ADDRESS_TRANSIENT: i32 = 2;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type IP_ADAPTER_ANYCAST_ADDRESS = IP_ADAPTER_ANYCAST_ADDRESS_XP;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0,
    pub Next: *mut Self,
    pub Address: super::SOCKET_ADDRESS,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_ANYCAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_ANYCAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_ANYCAST_ADDRESS_XP_0_0 {
    pub Length: super::ULONG,
    pub Flags: super::DWORD,
}
pub const IP_ADAPTER_DDNS_ENABLED: i32 = 1;
pub const IP_ADAPTER_DHCP_ENABLED: i32 = 4;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type IP_ADAPTER_DNS_SERVER_ADDRESS = IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0,
    pub Next: *mut Self,
    pub Address: super::SOCKET_ADDRESS,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP_0_0 {
    pub Length: super::ULONG,
    pub Reserved: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_DNS_SUFFIX {
    pub Next: *mut Self,
    pub String: [super::WCHAR; 256],
}
#[cfg(feature = "winnt")]
impl Default for IP_ADAPTER_DNS_SUFFIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type IP_ADAPTER_GATEWAY_ADDRESS = IP_ADAPTER_GATEWAY_ADDRESS_LH;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0,
    pub Next: *mut Self,
    pub Address: super::SOCKET_ADDRESS,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_GATEWAY_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_GATEWAY_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_GATEWAY_ADDRESS_LH_0_0 {
    pub Length: super::ULONG,
    pub Reserved: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "corecrt", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_INFO {
    pub Next: *mut Self,
    pub ComboIndex: super::DWORD,
    pub AdapterName: [i8; 260],
    pub Description: [i8; 132],
    pub AddressLength: super::UINT,
    pub Address: [super::BYTE; 8],
    pub Index: super::DWORD,
    pub Type: super::UINT,
    pub DhcpEnabled: super::UINT,
    pub CurrentIpAddress: PIP_ADDR_STRING,
    pub IpAddressList: IP_ADDR_STRING,
    pub GatewayList: IP_ADDR_STRING,
    pub DhcpServer: IP_ADDR_STRING,
    pub HaveWins: windows_sys::core::BOOL,
    pub PrimaryWinsServer: IP_ADDR_STRING,
    pub SecondaryWinsServer: IP_ADDR_STRING,
    pub LeaseObtained: super::time_t,
    pub LeaseExpires: super::time_t,
}
#[cfg(all(feature = "corecrt", feature = "shared"))]
impl Default for IP_ADAPTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_ADAPTER_IPV4_ENABLED: i32 = 128;
pub const IP_ADAPTER_IPV6_ENABLED: i32 = 256;
pub const IP_ADAPTER_IPV6_MANAGE_ADDRESS_CONFIG: i32 = 512;
pub const IP_ADAPTER_IPV6_OTHER_STATEFUL_CONFIG: i32 = 32;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type IP_ADAPTER_MULTICAST_ADDRESS = IP_ADAPTER_MULTICAST_ADDRESS_XP;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0,
    pub Next: *mut Self,
    pub Address: super::SOCKET_ADDRESS,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_MULTICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_MULTICAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_MULTICAST_ADDRESS_XP_0_0 {
    pub Length: super::ULONG,
    pub Flags: super::DWORD,
}
pub const IP_ADAPTER_NETBIOS_OVER_TCPIP_ENABLED: i32 = 64;
pub const IP_ADAPTER_NO_MULTICAST: i32 = 16;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type IP_ADAPTER_PREFIX = IP_ADAPTER_PREFIX_XP;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_PREFIX_XP {
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0,
    pub Next: *mut Self,
    pub Address: super::SOCKET_ADDRESS,
    pub PrefixLength: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_PREFIX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_PREFIX_XP_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_PREFIX_XP_0_0,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_PREFIX_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_PREFIX_XP_0_0 {
    pub Length: super::ULONG,
    pub Flags: super::DWORD,
}
pub const IP_ADAPTER_RECEIVE_ONLY: i32 = 8;
pub const IP_ADAPTER_REGISTER_ADAPTER_SUFFIX: i32 = 2;
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
pub type IP_ADAPTER_UNICAST_ADDRESS = IP_ADAPTER_UNICAST_ADDRESS_LH;
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0,
    pub Next: *mut Self,
    pub Address: super::SOCKET_ADDRESS,
    pub PrefixOrigin: IP_PREFIX_ORIGIN,
    pub SuffixOrigin: IP_SUFFIX_ORIGIN,
    pub DadState: IP_DAD_STATE,
    pub ValidLifetime: super::ULONG,
    pub PreferredLifetime: super::ULONG,
    pub LeaseLifetime: super::ULONG,
    pub OnLinkPrefixLength: super::UINT8,
}
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_LH_0_0 {
    pub Length: super::ULONG,
    pub Flags: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP {
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0,
    pub Next: *mut Self,
    pub Address: super::SOCKET_ADDRESS,
    pub PrefixOrigin: IP_PREFIX_ORIGIN,
    pub SuffixOrigin: IP_SUFFIX_ORIGIN,
    pub DadState: IP_DAD_STATE,
    pub ValidLifetime: super::ULONG,
    pub PreferredLifetime: super::ULONG,
    pub LeaseLifetime: super::ULONG,
}
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_UNICAST_ADDRESS_XP_0_0,
}
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_UNICAST_ADDRESS_XP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_UNICAST_ADDRESS_XP_0_0 {
    pub Length: super::ULONG,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type IP_ADAPTER_WINS_SERVER_ADDRESS = IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0,
    pub Next: *mut Self,
    pub Address: super::SOCKET_ADDRESS,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    pub Alignment: super::ULONGLONG,
    pub Anonymous: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl Default for IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Default)]
pub struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH_0_0 {
    pub Length: super::ULONG,
    pub Reserved: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IP_ADDRESS_STRING {
    pub String: [i8; 16],
}
impl Default for IP_ADDRESS_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IP_ADDR_STRING {
    pub Next: *mut Self,
    pub IpAddress: IP_ADDRESS_STRING,
    pub IpMask: IP_MASK_STRING,
    pub Context: super::DWORD,
}
#[cfg(feature = "nldef")]
pub type IP_DAD_STATE = super::NL_DAD_STATE;
#[cfg(feature = "shared")]
pub type IP_INTERFACE_NAME_INFO = IP_INTERFACE_NAME_INFO_W2KSP1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IP_INTERFACE_NAME_INFO_W2KSP1 {
    pub Index: super::ULONG,
    pub MediaType: super::ULONG,
    pub ConnectionType: super::UCHAR,
    pub AccessType: super::UCHAR,
    pub DeviceGuid: windows_sys::core::GUID,
    pub InterfaceGuid: windows_sys::core::GUID,
}
pub type IP_MASK_STRING = IP_ADDRESS_STRING;
#[cfg(feature = "shared")]
pub type IP_PER_ADAPTER_INFO = IP_PER_ADAPTER_INFO_W2KSP1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IP_PER_ADAPTER_INFO_W2KSP1 {
    pub AutoconfigEnabled: super::UINT,
    pub AutoconfigActive: super::UINT,
    pub CurrentDnsServer: PIP_ADDR_STRING,
    pub DnsServerList: IP_ADDR_STRING,
}
#[cfg(feature = "nldef")]
pub type IP_PREFIX_ORIGIN = super::NL_PREFIX_ORIGIN;
#[cfg(feature = "nldef")]
pub type IP_SUFFIX_ORIGIN = super::NL_SUFFIX_ORIGIN;
pub const MAX_ADAPTER_ADDRESS_LENGTH: i32 = 8;
pub const MAX_ADAPTER_DESCRIPTION_LENGTH: i32 = 128;
pub const MAX_ADAPTER_NAME_LENGTH: i32 = 256;
pub const MAX_DHCPV6_DUID_LENGTH: i32 = 130;
pub const MAX_DNS_SUFFIX_STRING_LENGTH: i32 = 256;
pub const MAX_DOMAIN_NAME_LEN: i32 = 128;
pub const MAX_HOSTNAME_LEN: i32 = 128;
pub const MAX_SCOPE_ID_LEN: i32 = 256;
pub const MIXED_NODETYPE: i32 = 4;
pub const PEER_TO_PEER_NODETYPE: i32 = 2;
#[cfg(feature = "shared")]
pub type PFIXED_INFO = *mut FIXED_INFO_W2KSP1;
#[cfg(feature = "shared")]
pub type PFIXED_INFO_W2KSP1 = *mut FIXED_INFO_W2KSP1;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PIP_ADAPTER_ADDRESSES = *mut IP_ADAPTER_ADDRESSES_LH;
#[cfg(all(feature = "ifdef", feature = "ipifcons", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PIP_ADAPTER_ADDRESSES_LH = *mut IP_ADAPTER_ADDRESSES_LH;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type PIP_ADAPTER_ADDRESSES_XP = *mut IP_ADAPTER_ADDRESSES_XP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_ANYCAST_ADDRESS = *mut IP_ADAPTER_ANYCAST_ADDRESS_XP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_ANYCAST_ADDRESS_XP = *mut IP_ADAPTER_ANYCAST_ADDRESS_XP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_DNS_SERVER_ADDRESS = *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_DNS_SERVER_ADDRESS_XP = *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
#[cfg(feature = "winnt")]
pub type PIP_ADAPTER_DNS_SUFFIX = *mut IP_ADAPTER_DNS_SUFFIX;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_GATEWAY_ADDRESS = *mut IP_ADAPTER_GATEWAY_ADDRESS_LH;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_GATEWAY_ADDRESS_LH = *mut IP_ADAPTER_GATEWAY_ADDRESS_LH;
#[cfg(all(feature = "corecrt", feature = "shared"))]
pub type PIP_ADAPTER_INFO = *mut IP_ADAPTER_INFO;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_MULTICAST_ADDRESS = *mut IP_ADAPTER_MULTICAST_ADDRESS_XP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_MULTICAST_ADDRESS_XP = *mut IP_ADAPTER_MULTICAST_ADDRESS_XP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_PREFIX = *mut IP_ADAPTER_PREFIX_XP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_PREFIX_XP = *mut IP_ADAPTER_PREFIX_XP;
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_UNICAST_ADDRESS = *mut IP_ADAPTER_UNICAST_ADDRESS_LH;
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_UNICAST_ADDRESS_LH = *mut IP_ADAPTER_UNICAST_ADDRESS_LH;
#[cfg(all(feature = "nldef", feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_UNICAST_ADDRESS_XP = *mut IP_ADAPTER_UNICAST_ADDRESS_XP;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_WINS_SERVER_ADDRESS = *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
#[cfg(all(feature = "shared", feature = "ws2"))]
pub type PIP_ADAPTER_WINS_SERVER_ADDRESS_LH = *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
pub type PIP_ADDRESS_STRING = *mut IP_ADDRESS_STRING;
#[cfg(feature = "shared")]
pub type PIP_ADDR_STRING = *mut IP_ADDR_STRING;
#[cfg(feature = "shared")]
pub type PIP_INTERFACE_NAME_INFO = *mut IP_INTERFACE_NAME_INFO_W2KSP1;
#[cfg(feature = "shared")]
pub type PIP_INTERFACE_NAME_INFO_W2KSP1 = *mut IP_INTERFACE_NAME_INFO_W2KSP1;
pub type PIP_MASK_STRING = *mut IP_ADDRESS_STRING;
#[cfg(feature = "shared")]
pub type PIP_PER_ADAPTER_INFO = *mut IP_PER_ADAPTER_INFO_W2KSP1;
#[cfg(feature = "shared")]
pub type PIP_PER_ADAPTER_INFO_W2KSP1 = *mut IP_PER_ADAPTER_INFO_W2KSP1;
