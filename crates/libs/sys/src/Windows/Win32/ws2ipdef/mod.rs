#[repr(C)]
#[cfg(feature = "Win32_ws2def")]
#[derive(Clone, Copy)]
pub struct GROUP_FILTER {
    pub gf_interface: u32,
    pub gf_group: super::ws2def::SOCKADDR_STORAGE,
    pub gf_fmode: MULTICAST_MODE_TYPE,
    pub gf_numsrc: u32,
    pub gf_slist: [super::ws2def::SOCKADDR_STORAGE; 1],
}
#[cfg(feature = "Win32_ws2def")]
impl Default for GROUP_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2def")]
#[derive(Clone, Copy, Default)]
pub struct GROUP_REQ {
    pub gr_interface: u32,
    pub gr_group: super::ws2def::SOCKADDR_STORAGE,
}
#[repr(C)]
#[cfg(feature = "Win32_ws2def")]
#[derive(Clone, Copy, Default)]
pub struct GROUP_SOURCE_REQ {
    pub gsr_interface: u32,
    pub gsr_group: super::ws2def::SOCKADDR_STORAGE,
    pub gsr_source: super::ws2def::SOCKADDR_STORAGE,
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct ICMP_ERROR_INFO {
    pub srcaddress: SOCKADDR_INET,
    pub protocol: super::ws2def::IPPROTO,
    pub r#type: u8,
    pub code: u8,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
impl Default for ICMP_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IFF_BROADCAST: u32 = 2;
pub const IFF_LOOPBACK: u32 = 4;
pub const IFF_MULTICAST: u32 = 16;
pub const IFF_POINTTOPOINT: u32 = 8;
pub const IFF_UP: u32 = 1;
pub const IN6ADDR_6TO4PREFIX_LENGTH: u32 = 16;
pub const IN6ADDR_LINKLOCALPREFIX_LENGTH: u32 = 64;
pub const IN6ADDR_MULTICASTPREFIX_LENGTH: u32 = 8;
pub const IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH: u32 = 104;
pub const IN6ADDR_TEREDOPREFIX_LENGTH: u32 = 32;
pub const IN6ADDR_V4MAPPEDPREFIX_LENGTH: u32 = 96;
#[repr(C)]
#[cfg(feature = "Win32_in6addr")]
#[derive(Clone, Copy)]
pub struct IN6_PKTINFO {
    pub ipi6_addr: super::in6addr::IN6_ADDR,
    pub ipi6_ifindex: u32,
}
#[cfg(feature = "Win32_in6addr")]
impl Default for IN6_PKTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct IN6_PKTINFO_EX {
    pub pkt_info: IN6_PKTINFO,
    pub scope_id: super::ws2def::SCOPE_ID,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
impl Default for IN6_PKTINFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INET6_ADDRSTRLEN: u32 = 65;
pub const INET_ADDRSTRLEN: u32 = 22;
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct INTERFACE_INFO {
    pub iiFlags: u32,
    pub iiAddress: sockaddr_gen,
    pub iiBroadcastAddress: sockaddr_gen,
    pub iiNetmask: sockaddr_gen,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
impl Default for INTERFACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2def")]
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_INFO_EX {
    pub iiFlags: u32,
    pub iiAddress: super::ws2def::SOCKET_ADDRESS,
    pub iiBroadcastAddress: super::ws2def::SOCKET_ADDRESS,
    pub iiNetmask: super::ws2def::SOCKET_ADDRESS,
}
#[repr(C)]
#[cfg(feature = "Win32_inaddr")]
#[derive(Clone, Copy)]
pub struct IN_PKTINFO {
    pub ipi_addr: super::inaddr::IN_ADDR,
    pub ipi_ifindex: u32,
}
#[cfg(feature = "Win32_inaddr")]
impl Default for IN_PKTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_inaddr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct IN_PKTINFO_EX {
    pub pkt_info: IN_PKTINFO,
    pub scope_id: super::ws2def::SCOPE_ID,
}
#[cfg(all(feature = "Win32_inaddr", feature = "Win32_ws2def"))]
impl Default for IN_PKTINFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ws2def")]
#[derive(Clone, Copy, Default)]
pub struct IN_RECVERR {
    pub protocol: super::ws2def::IPPROTO,
    pub info: u32,
    pub r#type: u8,
    pub code: u8,
}
pub const IPV6_ADDRESS_BITS: u32 = 128;
pub const IPV6_ADD_IFLIST: u32 = 29;
pub const IPV6_ADD_MEMBERSHIP: u32 = 12;
pub const IPV6_CHECKSUM: u32 = 26;
pub const IPV6_DEL_IFLIST: u32 = 30;
pub const IPV6_DONTFRAG: u32 = 14;
pub const IPV6_DROP_MEMBERSHIP: u32 = 13;
pub const IPV6_ECN: u32 = 50;
pub const IPV6_GET_IFLIST: u32 = 33;
pub const IPV6_HDRINCL: u32 = 2;
pub const IPV6_HOPLIMIT: u32 = 21;
pub const IPV6_HOPOPTS: u32 = 1;
pub const IPV6_IFLIST: u32 = 28;
pub const IPV6_JOIN_GROUP: u32 = 12;
pub const IPV6_LEAVE_GROUP: u32 = 13;
#[repr(C)]
#[cfg(feature = "Win32_in6addr")]
#[derive(Clone, Copy)]
pub struct IPV6_MREQ {
    pub ipv6mr_multiaddr: super::in6addr::IN6_ADDR,
    pub ipv6mr_interface: u32,
}
#[cfg(feature = "Win32_in6addr")]
impl Default for IPV6_MREQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPV6_MTU: u32 = 72;
pub const IPV6_MTU_DISCOVER: u32 = 71;
pub const IPV6_MULTICAST_HOPS: u32 = 10;
pub const IPV6_MULTICAST_IF: u32 = 9;
pub const IPV6_MULTICAST_LOOP: u32 = 11;
pub const IPV6_NRT_INTERFACE: u32 = 74;
pub const IPV6_PKTINFO: u32 = 19;
pub const IPV6_PKTINFO_EX: u32 = 51;
pub const IPV6_PROTECTION_LEVEL: u32 = 23;
pub const IPV6_RECVDSTADDR: u32 = 25;
pub const IPV6_RECVECN: u32 = 50;
pub const IPV6_RECVERR: u32 = 75;
pub const IPV6_RECVIF: u32 = 24;
pub const IPV6_RECVRTHDR: u32 = 38;
pub const IPV6_RECVTCLASS: u32 = 40;
pub const IPV6_RTHDR: u32 = 32;
pub const IPV6_TCLASS: u32 = 39;
pub const IPV6_UNICAST_HOPS: u32 = 4;
pub const IPV6_UNICAST_IF: u32 = 31;
pub const IPV6_USER_MTU: u32 = 76;
pub const IPV6_V6ONLY: u32 = 27;
pub const IPV6_WFP_REDIRECT_CONTEXT: u32 = 70;
pub const IPV6_WFP_REDIRECT_RECORDS: u32 = 60;
pub const IP_ADD_IFLIST: u32 = 29;
pub const IP_ADD_MEMBERSHIP: u32 = 12;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 15;
pub const IP_BLOCK_SOURCE: u32 = 17;
pub const IP_DEL_IFLIST: u32 = 30;
pub const IP_DONTFRAGMENT: u32 = 14;
pub const IP_DROP_MEMBERSHIP: u32 = 13;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 16;
pub const IP_ECN: u32 = 50;
pub const IP_GET_IFLIST: u32 = 33;
pub const IP_HDRINCL: u32 = 2;
pub const IP_HOPLIMIT: u32 = 21;
pub const IP_IFLIST: u32 = 28;
#[repr(C)]
#[cfg(feature = "Win32_inaddr")]
#[derive(Clone, Copy)]
pub struct IP_MREQ {
    pub imr_multiaddr: super::inaddr::IN_ADDR,
    pub imr_interface: super::inaddr::IN_ADDR,
}
#[cfg(feature = "Win32_inaddr")]
impl Default for IP_MREQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_inaddr")]
#[derive(Clone, Copy)]
pub struct IP_MREQ_SOURCE {
    pub imr_multiaddr: super::inaddr::IN_ADDR,
    pub imr_sourceaddr: super::inaddr::IN_ADDR,
    pub imr_interface: super::inaddr::IN_ADDR,
}
#[cfg(feature = "Win32_inaddr")]
impl Default for IP_MREQ_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_inaddr")]
#[derive(Clone, Copy)]
pub struct IP_MSFILTER {
    pub imsf_multiaddr: super::inaddr::IN_ADDR,
    pub imsf_interface: super::inaddr::IN_ADDR,
    pub imsf_fmode: MULTICAST_MODE_TYPE,
    pub imsf_numsrc: u32,
    pub imsf_slist: [super::inaddr::IN_ADDR; 1],
}
#[cfg(feature = "Win32_inaddr")]
impl Default for IP_MSFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_MTU: u32 = 73;
pub const IP_MTU_DISCOVER: u32 = 71;
pub const IP_MULTICAST_IF: u32 = 9;
pub const IP_MULTICAST_LOOP: u32 = 11;
pub const IP_MULTICAST_TTL: u32 = 10;
pub const IP_NRT_INTERFACE: u32 = 74;
pub const IP_OPTIONS: u32 = 1;
pub const IP_ORIGINAL_ARRIVAL_IF: u32 = 47;
pub const IP_PKTINFO: u32 = 19;
pub const IP_PKTINFO_EX: u32 = 51;
pub const IP_PMTUDISC_DO: PMTUD_STATE = 1;
pub const IP_PMTUDISC_DONT: PMTUD_STATE = 2;
pub const IP_PMTUDISC_MAX: PMTUD_STATE = 4;
pub const IP_PMTUDISC_NOT_SET: PMTUD_STATE = 0;
pub const IP_PMTUDISC_PROBE: PMTUD_STATE = 3;
pub const IP_PROTECTION_LEVEL: u32 = 23;
pub const IP_RECEIVE_BROADCAST: u32 = 22;
pub const IP_RECVDSTADDR: u32 = 25;
pub const IP_RECVECN: u32 = 50;
pub const IP_RECVERR: u32 = 75;
pub const IP_RECVIF: u32 = 24;
pub const IP_RECVRTHDR: u32 = 38;
pub const IP_RECVTCLASS: u32 = 40;
pub const IP_RECVTOS: u32 = 40;
pub const IP_RECVTTL: u32 = 21;
pub const IP_RTHDR: u32 = 32;
pub const IP_TCLASS: u32 = 39;
pub const IP_TOS: u32 = 3;
pub const IP_TTL: u32 = 4;
pub const IP_UNBLOCK_SOURCE: u32 = 18;
pub const IP_UNICAST_IF: u32 = 31;
pub const IP_UNSPECIFIED_HOP_LIMIT: i32 = -1;
pub const IP_UNSPECIFIED_TYPE_OF_SERVICE: i32 = -1;
pub const IP_USER_MTU: u32 = 76;
pub const IP_WFP_REDIRECT_CONTEXT: u32 = 70;
pub const IP_WFP_REDIRECT_RECORDS: u32 = 60;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
pub type LPINTERFACE_INFO = *mut INTERFACE_INFO;
#[cfg(feature = "Win32_ws2def")]
pub type LPINTERFACE_INFO_EX = *mut INTERFACE_INFO_EX;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
pub type LPSOCKADDR_IN6 = *mut SOCKADDR_IN6_LH;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
pub type LPSOCKADDR_IN6_LH = *mut SOCKADDR_IN6_LH;
#[cfg(feature = "Win32_in6addr")]
pub type LPSOCKADDR_IN6_W2KSP1 = *mut SOCKADDR_IN6_W2KSP1;
pub const MCAST_BLOCK_SOURCE: u32 = 43;
pub const MCAST_EXCLUDE: MULTICAST_MODE_TYPE = 1;
pub const MCAST_INCLUDE: MULTICAST_MODE_TYPE = 0;
pub const MCAST_JOIN_GROUP: u32 = 41;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 45;
pub const MCAST_LEAVE_GROUP: u32 = 42;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 46;
pub const MCAST_UNBLOCK_SOURCE: u32 = 44;
pub type MULTICAST_MODE_TYPE = i32;
#[cfg(feature = "Win32_ws2def")]
pub type PGROUP_FILTER = *mut GROUP_FILTER;
#[cfg(feature = "Win32_ws2def")]
pub type PGROUP_REQ = *mut GROUP_REQ;
#[cfg(feature = "Win32_ws2def")]
pub type PGROUP_SOURCE_REQ = *mut GROUP_SOURCE_REQ;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
pub type PICMP_ERROR_INFO = *mut ICMP_ERROR_INFO;
#[cfg(feature = "Win32_in6addr")]
pub type PIN6_PKTINFO = *mut IN6_PKTINFO;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
pub type PIN6_PKTINFO_EX = *mut IN6_PKTINFO_EX;
#[cfg(feature = "Win32_inaddr")]
pub type PIN_PKTINFO = *mut IN_PKTINFO;
#[cfg(all(feature = "Win32_inaddr", feature = "Win32_ws2def"))]
pub type PIN_PKTINFO_EX = *mut IN_PKTINFO_EX;
#[cfg(feature = "Win32_ws2def")]
pub type PIN_RECVERR = *mut IN_RECVERR;
#[cfg(feature = "Win32_in6addr")]
pub type PIPV6_MREQ = *mut IPV6_MREQ;
#[cfg(feature = "Win32_inaddr")]
pub type PIP_MREQ = *mut IP_MREQ;
#[cfg(feature = "Win32_inaddr")]
pub type PIP_MREQ_SOURCE = *mut IP_MREQ_SOURCE;
#[cfg(feature = "Win32_inaddr")]
pub type PIP_MSFILTER = *mut IP_MSFILTER;
pub type PMTUD_STATE = i32;
pub type PPMTUD_STATE = *mut PMTUD_STATE;
pub const PROTECTION_LEVEL_DEFAULT: u32 = 4294967295;
pub const PROTECTION_LEVEL_EDGERESTRICTED: u32 = 20;
pub const PROTECTION_LEVEL_RESTRICTED: u32 = 30;
pub const PROTECTION_LEVEL_UNRESTRICTED: u32 = 10;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
pub type PSOCKADDR_IN6 = *mut SOCKADDR_IN6_LH;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
pub type PSOCKADDR_IN6_LH = *mut SOCKADDR_IN6_LH;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
pub type PSOCKADDR_IN6_PAIR = *mut SOCKADDR_IN6_PAIR;
#[cfg(feature = "Win32_in6addr")]
pub type PSOCKADDR_IN6_W2KSP1 = *mut SOCKADDR_IN6_W2KSP1;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
pub type PSOCKADDR_INET = *mut SOCKADDR_INET;
pub const SIOCGIPMSFILTER: i32 = -2147191684;
pub const SIOCGMSFILTER: i32 = -2147191681;
pub const SIOCSIPMSFILTER: i32 = -2147191683;
pub const SIOCSMSFILTER: i32 = -2147191682;
pub const SIO_GET_INTERFACE_LIST: u32 = 1074033791;
pub const SIO_GET_INTERFACE_LIST_EX: u32 = 1074033790;
pub const SIO_GET_MULTICAST_FILTER: i32 = -2147191684;
pub const SIO_IDEAL_SEND_BACKLOG_CHANGE: u32 = 536900730;
pub const SIO_IDEAL_SEND_BACKLOG_QUERY: u32 = 1074033787;
pub const SIO_SET_MULTICAST_FILTER: i32 = -2147191683;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
pub type SOCKADDR_IN6 = SOCKADDR_IN6_LH;
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct SOCKADDR_IN6_LH {
    pub sin6_family: super::ws2def::ADDRESS_FAMILY,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: super::in6addr::IN6_ADDR,
    pub Anonymous: SOCKADDR_IN6_LH_0,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
impl Default for SOCKADDR_IN6_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub union SOCKADDR_IN6_LH_0 {
    pub sin6_scope_id: u32,
    pub sin6_scope_struct: super::ws2def::SCOPE_ID,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
impl Default for SOCKADDR_IN6_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub struct SOCKADDR_IN6_PAIR {
    pub SourceAddress: PSOCKADDR_IN6,
    pub DestinationAddress: PSOCKADDR_IN6,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ws2def"))]
impl Default for SOCKADDR_IN6_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_in6addr")]
#[derive(Clone, Copy)]
pub struct SOCKADDR_IN6_W2KSP1 {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: super::in6addr::IN6_ADDR,
    pub sin6_scope_id: u32,
}
#[cfg(feature = "Win32_in6addr")]
impl Default for SOCKADDR_IN6_W2KSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub union SOCKADDR_INET {
    pub Ipv4: super::ws2def::SOCKADDR_IN,
    pub Ipv6: SOCKADDR_IN6,
    pub si_family: super::ws2def::ADDRESS_FAMILY,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
impl Default for SOCKADDR_INET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TCP_ATMARK: u32 = 8;
pub const TCP_CONGESTION_ALGORITHM: u32 = 12;
pub const TCP_DELAY_FIN_ACK: u32 = 13;
pub const TCP_EXPEDITED_1122: u32 = 2;
pub const TCP_FAIL_CONNECT_ON_ICMP_ERROR: u32 = 18;
pub const TCP_FASTOPEN: u32 = 15;
pub const TCP_ICMP_ERROR_INFO: u32 = 19;
pub const TCP_KEEPALIVE: u32 = 3;
pub const TCP_KEEPCNT: u32 = 16;
pub const TCP_KEEPIDLE: u32 = 3;
pub const TCP_KEEPINTVL: u32 = 17;
pub const TCP_MAXRT: u32 = 5;
pub const TCP_MAXRTMS: u32 = 14;
pub const TCP_MAXSEG: u32 = 4;
pub const TCP_NOSYNRETRIES: u32 = 9;
pub const TCP_NOURG: u32 = 7;
pub const TCP_OFFLOAD_NOT_PREFERRED: u32 = 1;
pub const TCP_OFFLOAD_NO_PREFERENCE: u32 = 0;
pub const TCP_OFFLOAD_PREFERENCE: u32 = 11;
pub const TCP_OFFLOAD_PREFERRED: u32 = 2;
pub const TCP_STDURG: u32 = 6;
pub const TCP_TIMESTAMPS: u32 = 10;
pub const UDP_COALESCED_INFO: u32 = 3;
pub const UDP_RECV_MAX_COALESCED_SIZE: u32 = 3;
pub const UDP_SEND_MSG_SIZE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
#[derive(Clone, Copy)]
pub union sockaddr_gen {
    pub Address: super::ws2def::SOCKADDR,
    pub AddressIn: super::ws2def::SOCKADDR_IN,
    pub AddressIn6: sockaddr_in6_old,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def"))]
impl Default for sockaddr_gen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_in6addr")]
#[derive(Clone, Copy)]
pub struct sockaddr_in6_old {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: super::in6addr::IN6_ADDR,
}
#[cfg(feature = "Win32_in6addr")]
impl Default for sockaddr_in6_old {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
