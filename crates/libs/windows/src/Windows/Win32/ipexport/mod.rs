#[repr(C)]
#[cfg(feature = "ntddndis")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ARP_SEND_REPLY {
    pub DestAddress: super::IPAddr,
    pub SrcAddress: super::IPAddr,
}
pub type ICMPV6_ECHO_REPLY = ICMPV6_ECHO_REPLY_LH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ICMPV6_ECHO_REPLY_LH {
    pub Address: IPV6_ADDRESS_EX,
    pub Status: u32,
    pub RoundTripTime: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICMP_ECHO_REPLY {
    pub Address: super::IPAddr,
    pub Status: u32,
    pub RoundTripTime: u32,
    pub DataSize: u16,
    pub Reserved: u16,
    pub Data: *mut core::ffi::c_void,
    pub Options: IP_OPTION_INFORMATION,
}
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
impl Default for ICMP_ECHO_REPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "ntddndis")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICMP_ECHO_REPLY32 {
    pub Address: super::IPAddr,
    pub Status: u32,
    pub RoundTripTime: u32,
    pub DataSize: u16,
    pub Reserved: u16,
    pub Data: *mut core::ffi::c_void,
    pub Options: IP_OPTION_INFORMATION32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "ntddndis")]
impl Default for ICMP_ECHO_REPLY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IPV6_ADDRESS_EX {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl Default for IPV6_ADDRESS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IP_ADAPTER_INDEX_MAP {
    pub Index: u32,
    pub Name: [u16; 128],
}
impl Default for IP_ADAPTER_INDEX_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IP_ADAPTER_ORDER_MAP {
    pub NumAdapters: u32,
    pub AdapterOrder: [u32; 1],
}
impl Default for IP_ADAPTER_ORDER_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_ADDR_ADDED: u32 = 11023;
pub const IP_ADDR_DELETED: u32 = 11019;
pub const IP_BAD_DESTINATION: u32 = 11018;
pub const IP_BAD_HEADER: u32 = 11042;
pub const IP_BAD_OPTION: u32 = 11007;
pub const IP_BAD_REQ: u32 = 11011;
pub const IP_BAD_ROUTE: u32 = 11012;
pub const IP_BIND_ADAPTER: u32 = 11026;
pub const IP_BUF_TOO_SMALL: u32 = 11001;
pub const IP_DEST_ADDR_UNREACHABLE: u32 = 11003;
pub const IP_DEST_HOST_UNREACHABLE: u32 = 11003;
pub const IP_DEST_NET_UNREACHABLE: u32 = 11002;
pub const IP_DEST_NO_ROUTE: u32 = 11002;
pub const IP_DEST_PORT_UNREACHABLE: u32 = 11005;
pub const IP_DEST_PROHIBITED: u32 = 11004;
pub const IP_DEST_PROT_UNREACHABLE: u32 = 11004;
pub const IP_DEST_SCOPE_MISMATCH: u32 = 11045;
pub const IP_DEST_UNREACHABLE: u32 = 11040;
pub const IP_DEVICE_DOES_NOT_EXIST: u32 = 11028;
pub const IP_DUPLICATE_ADDRESS: u32 = 11029;
pub const IP_DUPLICATE_IPADD: u32 = 11034;
pub const IP_EXPORT_INCLUDED: u32 = 1;
pub const IP_FLAG_DF: u32 = 2;
pub const IP_FLAG_REVERSE: u32 = 1;
pub const IP_GENERAL_FAILURE: u32 = 11050;
pub const IP_HOP_LIMIT_EXCEEDED: u32 = 11013;
pub const IP_HW_ERROR: u32 = 11008;
pub const IP_ICMP_ERROR: u32 = 11044;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IP_INTERFACE_INFO {
    pub NumAdapters: i32,
    pub Adapter: [IP_ADAPTER_INDEX_MAP; 1],
}
impl Default for IP_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_INTERFACE_METRIC_CHANGE: u32 = 11030;
pub const IP_INTERFACE_WOL_CAPABILITY_CHANGE: u32 = 11033;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IP_MCAST_COUNTER_INFO {
    pub InMcastOctets: u64,
    pub OutMcastOctets: u64,
    pub InMcastPkts: u64,
    pub OutMcastPkts: u64,
}
pub const IP_MEDIA_CONNECT: u32 = 11024;
pub const IP_MEDIA_DISCONNECT: u32 = 11025;
pub const IP_MTU_CHANGE: u32 = 11021;
pub const IP_NEGOTIATING_IPSEC: u32 = 11032;
pub const IP_NO_RESOURCES: u32 = 11006;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IP_OPTION_INFORMATION {
    pub Ttl: u8,
    pub Tos: u8,
    pub Flags: u8,
    pub OptionsSize: u8,
    pub OptionsData: super::PUCHAR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IP_OPTION_INFORMATION32 {
    pub Ttl: u8,
    pub Tos: u8,
    pub Flags: u8,
    pub OptionsSize: u8,
    pub OptionsData: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for IP_OPTION_INFORMATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_OPTION_TOO_BIG: u32 = 11017;
pub const IP_OPT_EOL: u32 = 0;
pub const IP_OPT_LSRR: u32 = 131;
pub const IP_OPT_NOP: u32 = 1;
pub const IP_OPT_ROUTER_ALERT: u32 = 148;
pub const IP_OPT_RR: u32 = 7;
pub const IP_OPT_SECURITY: u32 = 130;
pub const IP_OPT_SID: u32 = 136;
pub const IP_OPT_SSRR: u32 = 137;
pub const IP_OPT_TS: u32 = 68;
pub const IP_PACKET_TOO_BIG: u32 = 11009;
pub const IP_PARAMETER_PROBLEM: u32 = 11015;
pub const IP_PARAM_PROBLEM: u32 = 11015;
pub const IP_PENDING: u32 = 11255;
pub const IP_REASSEMBLY_TIME_EXCEEDED: u32 = 11014;
pub const IP_RECONFIG_SECFLTR: u32 = 11031;
pub const IP_REQ_TIMED_OUT: u32 = 11010;
pub const IP_SOURCE_QUENCH: u32 = 11016;
pub const IP_SPEC_MTU_CHANGE: u32 = 11020;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IP_STATUS(pub u32);
pub const IP_STATUS_BASE: u32 = 11000;
pub const IP_SUCCESS: u32 = 0;
pub const IP_TIME_EXCEEDED: u32 = 11041;
pub const IP_TTL_EXPIRED_REASSEM: u32 = 11014;
pub const IP_TTL_EXPIRED_TRANSIT: u32 = 11013;
pub const IP_UNBIND_ADAPTER: u32 = 11027;
#[repr(C)]
#[cfg(feature = "ntddndis")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    pub NumAdapters: u32,
    pub Address: [super::IPAddr; 1],
}
#[cfg(feature = "ntddndis")]
impl Default for IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_UNLOAD: u32 = 11022;
pub const IP_UNRECOGNIZED_NEXT_HEADER: u32 = 11043;
#[cfg(feature = "in6addr")]
pub type IPv6Addr = super::IN6_ADDR;
pub const MAX_ADAPTER_NAME: u32 = 128;
pub const MAX_IP_STATUS: u32 = 11050;
pub const MAX_OPT_SIZE: u32 = 40;
#[cfg(feature = "ntddndis")]
pub type PARP_SEND_REPLY = *mut ARP_SEND_REPLY;
pub type PICMPV6_ECHO_REPLY = *mut ICMPV6_ECHO_REPLY_LH;
pub type PICMPV6_ECHO_REPLY_LH = *mut ICMPV6_ECHO_REPLY_LH;
#[cfg(all(feature = "minwindef", feature = "ntddndis"))]
pub type PICMP_ECHO_REPLY = *mut ICMP_ECHO_REPLY;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "ntddndis")]
pub type PICMP_ECHO_REPLY32 = *mut ICMP_ECHO_REPLY32;
pub type PIPV6_ADDRESS_EX = *mut IPV6_ADDRESS_EX;
pub type PIP_ADAPTER_INDEX_MAP = *mut IP_ADAPTER_INDEX_MAP;
pub type PIP_ADAPTER_ORDER_MAP = *mut IP_ADAPTER_ORDER_MAP;
pub type PIP_INTERFACE_INFO = *mut IP_INTERFACE_INFO;
pub type PIP_MCAST_COUNTER_INFO = *mut IP_MCAST_COUNTER_INFO;
#[cfg(feature = "minwindef")]
pub type PIP_OPTION_INFORMATION = *mut IP_OPTION_INFORMATION;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PIP_OPTION_INFORMATION32 = *mut IP_OPTION_INFORMATION32;
#[cfg(feature = "ntddndis")]
pub type PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS = *mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS;
pub type PTCP_RESERVE_PORT_RANGE = *mut TCP_RESERVE_PORT_RANGE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_RESERVE_PORT_RANGE {
    pub UpperRange: u16,
    pub LowerRange: u16,
}
