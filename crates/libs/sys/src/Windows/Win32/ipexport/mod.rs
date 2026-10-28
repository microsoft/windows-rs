#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ARP_SEND_REPLY {
    pub DestAddress: super::IPAddr,
    pub SrcAddress: super::IPAddr,
}
#[cfg(feature = "shared")]
pub type ICMPV6_ECHO_REPLY = ICMPV6_ECHO_REPLY_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ICMPV6_ECHO_REPLY_LH {
    pub Address: IPV6_ADDRESS_EX,
    pub Status: super::ULONG,
    pub RoundTripTime: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ICMP_ECHO_REPLY {
    pub Address: super::IPAddr,
    pub Status: super::ULONG,
    pub RoundTripTime: super::ULONG,
    pub DataSize: super::USHORT,
    pub Reserved: super::USHORT,
    pub Data: super::PVOID,
    pub Options: IP_OPTION_INFORMATION,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IPV6_ADDRESS_EX {
    pub sin6_port: super::USHORT,
    pub sin6_flowinfo: super::ULONG,
    pub sin6_addr: [super::USHORT; 8],
    pub sin6_scope_id: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for IPV6_ADDRESS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_INDEX_MAP {
    pub Index: super::ULONG,
    pub Name: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IP_ADAPTER_INDEX_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IP_ADAPTER_ORDER_MAP {
    pub NumAdapters: super::ULONG,
    pub AdapterOrder: [super::ULONG; 1],
}
#[cfg(feature = "shared")]
impl Default for IP_ADAPTER_ORDER_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_ADDR_ADDED: i32 = 11023;
pub const IP_ADDR_DELETED: i32 = 11019;
pub const IP_BAD_DESTINATION: i32 = 11018;
pub const IP_BAD_HEADER: i32 = 11042;
pub const IP_BAD_OPTION: i32 = 11007;
pub const IP_BAD_REQ: i32 = 11011;
pub const IP_BAD_ROUTE: i32 = 11012;
pub const IP_BIND_ADAPTER: i32 = 11026;
pub const IP_BUF_TOO_SMALL: i32 = 11001;
pub const IP_DEST_ADDR_UNREACHABLE: i32 = 11003;
pub const IP_DEST_HOST_UNREACHABLE: i32 = 11003;
pub const IP_DEST_NET_UNREACHABLE: i32 = 11002;
pub const IP_DEST_NO_ROUTE: i32 = 11002;
pub const IP_DEST_PORT_UNREACHABLE: i32 = 11005;
pub const IP_DEST_PROHIBITED: i32 = 11004;
pub const IP_DEST_PROT_UNREACHABLE: i32 = 11004;
pub const IP_DEST_SCOPE_MISMATCH: i32 = 11045;
pub const IP_DEST_UNREACHABLE: i32 = 11040;
pub const IP_DEVICE_DOES_NOT_EXIST: i32 = 11028;
pub const IP_DUPLICATE_ADDRESS: i32 = 11029;
pub const IP_DUPLICATE_IPADD: i32 = 11034;
pub const IP_EXPORT_INCLUDED: i32 = 1;
pub const IP_FLAG_DF: i32 = 2;
pub const IP_FLAG_REVERSE: i32 = 1;
pub const IP_GENERAL_FAILURE: i32 = 11050;
pub const IP_HOP_LIMIT_EXCEEDED: i32 = 11013;
pub const IP_HW_ERROR: i32 = 11008;
pub const IP_ICMP_ERROR: i32 = 11044;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IP_INTERFACE_INFO {
    pub NumAdapters: super::LONG,
    pub Adapter: [IP_ADAPTER_INDEX_MAP; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IP_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_INTERFACE_METRIC_CHANGE: i32 = 11030;
pub const IP_INTERFACE_WOL_CAPABILITY_CHANGE: i32 = 11033;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IP_MCAST_COUNTER_INFO {
    pub InMcastOctets: super::ULONG64,
    pub OutMcastOctets: super::ULONG64,
    pub InMcastPkts: super::ULONG64,
    pub OutMcastPkts: super::ULONG64,
}
pub const IP_MEDIA_CONNECT: i32 = 11024;
pub const IP_MEDIA_DISCONNECT: i32 = 11025;
pub const IP_MTU_CHANGE: i32 = 11021;
pub const IP_NEGOTIATING_IPSEC: i32 = 11032;
pub const IP_NO_RESOURCES: i32 = 11006;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct IP_OPTION_INFORMATION {
    pub Ttl: super::UCHAR,
    pub Tos: super::UCHAR,
    pub Flags: super::UCHAR,
    pub OptionsSize: super::UCHAR,
    pub OptionsData: super::PUCHAR,
}
pub const IP_OPTION_TOO_BIG: i32 = 11017;
pub const IP_OPT_EOL: i32 = 0;
pub const IP_OPT_LSRR: i32 = 131;
pub const IP_OPT_NOP: i32 = 1;
pub const IP_OPT_ROUTER_ALERT: i32 = 148;
pub const IP_OPT_RR: i32 = 7;
pub const IP_OPT_SECURITY: i32 = 130;
pub const IP_OPT_SID: i32 = 136;
pub const IP_OPT_SSRR: i32 = 137;
pub const IP_OPT_TS: i32 = 68;
pub const IP_PACKET_TOO_BIG: i32 = 11009;
pub const IP_PARAMETER_PROBLEM: i32 = 11015;
pub const IP_PARAM_PROBLEM: i32 = 11015;
pub const IP_PENDING: i32 = 11255;
pub const IP_REASSEMBLY_TIME_EXCEEDED: i32 = 11014;
pub const IP_RECONFIG_SECFLTR: i32 = 11031;
pub const IP_REQ_TIMED_OUT: i32 = 11010;
pub const IP_SOURCE_QUENCH: i32 = 11016;
pub const IP_SPEC_MTU_CHANGE: i32 = 11020;
pub type IP_STATUS = u32;
pub const IP_STATUS_BASE: i32 = 11000;
pub const IP_SUCCESS: i32 = 0;
pub const IP_TIME_EXCEEDED: i32 = 11041;
pub const IP_TTL_EXPIRED_REASSEM: i32 = 11014;
pub const IP_TTL_EXPIRED_TRANSIT: i32 = 11013;
pub const IP_UNBIND_ADAPTER: i32 = 11027;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    pub NumAdapters: super::ULONG,
    pub Address: [super::IPAddr; 1],
}
#[cfg(feature = "shared")]
impl Default for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_UNLOAD: i32 = 11022;
pub const IP_UNRECOGNIZED_NEXT_HEADER: i32 = 11043;
#[cfg(all(feature = "in6addr", feature = "shared"))]
pub type IPv6Addr = super::IN6_ADDR;
pub const MAX_ADAPTER_NAME: i32 = 128;
pub const MAX_IP_STATUS: i32 = 11050;
pub const MAX_OPT_SIZE: i32 = 40;
#[cfg(feature = "shared")]
pub type PARP_SEND_REPLY = *mut ARP_SEND_REPLY;
#[cfg(feature = "shared")]
pub type PICMPV6_ECHO_REPLY = *mut ICMPV6_ECHO_REPLY_LH;
#[cfg(feature = "shared")]
pub type PICMPV6_ECHO_REPLY_LH = *mut ICMPV6_ECHO_REPLY_LH;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PICMP_ECHO_REPLY = *mut ICMP_ECHO_REPLY;
#[cfg(feature = "shared")]
pub type PIPV6_ADDRESS_EX = *mut IPV6_ADDRESS_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIP_ADAPTER_INDEX_MAP = *mut IP_ADAPTER_INDEX_MAP;
#[cfg(feature = "shared")]
pub type PIP_ADAPTER_ORDER_MAP = *mut IP_ADAPTER_ORDER_MAP;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIP_INTERFACE_INFO = *mut IP_INTERFACE_INFO;
#[cfg(feature = "shared")]
pub type PIP_MCAST_COUNTER_INFO = *mut IP_MCAST_COUNTER_INFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PIP_OPTION_INFORMATION = *mut IP_OPTION_INFORMATION;
#[cfg(feature = "shared")]
pub type PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS = *mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS;
#[cfg(feature = "shared")]
pub type PTCP_RESERVE_PORT_RANGE = *mut TCP_RESERVE_PORT_RANGE;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TCP_RESERVE_PORT_RANGE {
    pub UpperRange: super::USHORT,
    pub LowerRange: super::USHORT,
}
