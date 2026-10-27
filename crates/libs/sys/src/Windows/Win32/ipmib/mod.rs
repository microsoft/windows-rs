pub const ICMP4_DST_UNREACH: ICMP4_TYPE = 3;
pub const ICMP4_ECHO_REPLY: ICMP4_TYPE = 0;
pub const ICMP4_ECHO_REQUEST: ICMP4_TYPE = 8;
pub const ICMP4_MASK_REPLY: ICMP4_TYPE = 18;
pub const ICMP4_MASK_REQUEST: ICMP4_TYPE = 17;
pub const ICMP4_PARAM_PROB: ICMP4_TYPE = 12;
pub const ICMP4_REDIRECT: ICMP4_TYPE = 5;
pub const ICMP4_ROUTER_ADVERT: ICMP4_TYPE = 9;
pub const ICMP4_ROUTER_SOLICIT: ICMP4_TYPE = 10;
pub const ICMP4_SOURCE_QUENCH: ICMP4_TYPE = 4;
pub const ICMP4_TIMESTAMP_REPLY: ICMP4_TYPE = 14;
pub const ICMP4_TIMESTAMP_REQUEST: ICMP4_TYPE = 13;
pub const ICMP4_TIME_EXCEEDED: ICMP4_TYPE = 11;
pub type ICMP4_TYPE = i32;
pub const ICMP6_DST_UNREACH: ICMP6_TYPE = 1;
pub const ICMP6_ECHO_REPLY: ICMP6_TYPE = 129;
pub const ICMP6_ECHO_REQUEST: ICMP6_TYPE = 128;
pub const ICMP6_INFOMSG_MASK: i32 = 128;
pub const ICMP6_MEMBERSHIP_QUERY: ICMP6_TYPE = 130;
pub const ICMP6_MEMBERSHIP_REDUCTION: ICMP6_TYPE = 132;
pub const ICMP6_MEMBERSHIP_REPORT: ICMP6_TYPE = 131;
pub const ICMP6_PACKET_TOO_BIG: ICMP6_TYPE = 2;
pub const ICMP6_PARAM_PROB: ICMP6_TYPE = 4;
pub const ICMP6_TIME_EXCEEDED: ICMP6_TYPE = 3;
pub type ICMP6_TYPE = i32;
pub const ICMP6_V2_MEMBERSHIP_REPORT: ICMP6_TYPE = 143;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIBICMPINFO {
    pub icmpInStats: MIBICMPSTATS,
    pub icmpOutStats: MIBICMPSTATS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIBICMPSTATS {
    pub dwMsgs: super::DWORD,
    pub dwErrors: super::DWORD,
    pub dwDestUnreachs: super::DWORD,
    pub dwTimeExcds: super::DWORD,
    pub dwParmProbs: super::DWORD,
    pub dwSrcQuenchs: super::DWORD,
    pub dwRedirects: super::DWORD,
    pub dwEchos: super::DWORD,
    pub dwEchoReps: super::DWORD,
    pub dwTimestamps: super::DWORD,
    pub dwTimestampReps: super::DWORD,
    pub dwAddrMasks: super::DWORD,
    pub dwAddrMaskReps: super::DWORD,
}
#[cfg(feature = "shared")]
pub type MIBICMPSTATS_EX = MIBICMPSTATS_EX_XPSP1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIBICMPSTATS_EX_XPSP1 {
    pub dwMsgs: super::DWORD,
    pub dwErrors: super::DWORD,
    pub rgdwTypeCount: [super::DWORD; 256],
}
#[cfg(feature = "shared")]
impl Default for MIBICMPSTATS_EX_XPSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_ICMP {
    pub stats: MIBICMPINFO,
}
#[cfg(feature = "shared")]
pub type MIB_ICMP_EX = MIB_ICMP_EX_XPSP1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_ICMP_EX_XPSP1 {
    pub icmpInStats: MIBICMPSTATS_EX,
    pub icmpOutStats: MIBICMPSTATS_EX,
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type MIB_IPADDRROW = MIB_IPADDRROW_XP;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPADDRROW_W2K {
    pub dwAddr: super::DWORD,
    pub dwIndex: super::DWORD,
    pub dwMask: super::DWORD,
    pub dwBCastAddr: super::DWORD,
    pub dwReasmSize: super::DWORD,
    pub unused1: u16,
    pub unused2: u16,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPADDRROW_XP {
    pub dwAddr: super::DWORD,
    pub dwIndex: super::IF_INDEX,
    pub dwMask: super::DWORD,
    pub dwBCastAddr: super::DWORD,
    pub dwReasmSize: super::DWORD,
    pub unused1: u16,
    pub wType: u16,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_IPADDRTABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_IPADDRROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
impl Default for MIB_IPADDRTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_IPADDR_DELETED: i32 = 64;
pub const MIB_IPADDR_DISCONNECTED: i32 = 8;
pub const MIB_IPADDR_DNS_ELIGIBLE: i32 = 256;
pub const MIB_IPADDR_DYNAMIC: i32 = 4;
pub const MIB_IPADDR_PRIMARY: i32 = 1;
pub const MIB_IPADDR_TRANSIENT: i32 = 128;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPFORWARDNUMBER {
    pub dwValue: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARDROW {
    pub dwForwardDest: super::DWORD,
    pub dwForwardMask: super::DWORD,
    pub dwForwardPolicy: super::DWORD,
    pub dwForwardNextHop: super::DWORD,
    pub dwForwardIfIndex: super::IF_INDEX,
    pub Anonymous: MIB_IPFORWARDROW_0,
    pub Anonymous2: MIB_IPFORWARDROW_1,
    pub dwForwardAge: super::DWORD,
    pub dwForwardNextHopAS: super::DWORD,
    pub dwForwardMetric1: super::DWORD,
    pub dwForwardMetric2: super::DWORD,
    pub dwForwardMetric3: super::DWORD,
    pub dwForwardMetric4: super::DWORD,
    pub dwForwardMetric5: super::DWORD,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
impl Default for MIB_IPFORWARDROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union MIB_IPFORWARDROW_0 {
    pub dwForwardType: super::DWORD,
    pub ForwardType: MIB_IPFORWARD_TYPE,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
impl Default for MIB_IPFORWARDROW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union MIB_IPFORWARDROW_1 {
    pub dwForwardProto: super::DWORD,
    pub ForwardProto: MIB_IPFORWARD_PROTO,
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
impl Default for MIB_IPFORWARDROW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARDTABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_IPFORWARDROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
impl Default for MIB_IPFORWARDTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "nldef")]
pub type MIB_IPFORWARD_PROTO = super::NL_ROUTE_PROTOCOL;
pub type MIB_IPFORWARD_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_GLOBAL {
    pub dwEnable: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_IF_ENTRY {
    pub dwIfIndex: super::DWORD,
    pub dwTtl: super::DWORD,
    pub dwProtocol: super::DWORD,
    pub dwRateLimit: super::DWORD,
    pub ulInMcastOctets: super::ULONG,
    pub ulOutMcastOctets: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_IF_TABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_IPMCAST_IF_ENTRY; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_IPMCAST_IF_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_MFE {
    pub dwGroup: super::DWORD,
    pub dwSource: super::DWORD,
    pub dwSrcMask: super::DWORD,
    pub dwUpStrmNgbr: super::DWORD,
    pub dwInIfIndex: super::DWORD,
    pub dwInIfProtocol: super::DWORD,
    pub dwRouteProtocol: super::DWORD,
    pub dwRouteNetwork: super::DWORD,
    pub dwRouteMask: super::DWORD,
    pub ulUpTime: super::ULONG,
    pub ulExpiryTime: super::ULONG,
    pub ulTimeOut: super::ULONG,
    pub ulNumOutIf: super::ULONG,
    pub fFlags: super::DWORD,
    pub dwReserved: super::DWORD,
    pub rgmioOutInfo: [MIB_IPMCAST_OIF; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_IPMCAST_MFE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_MFE_STATS {
    pub dwGroup: super::DWORD,
    pub dwSource: super::DWORD,
    pub dwSrcMask: super::DWORD,
    pub dwUpStrmNgbr: super::DWORD,
    pub dwInIfIndex: super::DWORD,
    pub dwInIfProtocol: super::DWORD,
    pub dwRouteProtocol: super::DWORD,
    pub dwRouteNetwork: super::DWORD,
    pub dwRouteMask: super::DWORD,
    pub ulUpTime: super::ULONG,
    pub ulExpiryTime: super::ULONG,
    pub ulNumOutIf: super::ULONG,
    pub ulInPkts: super::ULONG,
    pub ulInOctets: super::ULONG,
    pub ulPktsDifferentIf: super::ULONG,
    pub ulQueueOverflow: super::ULONG,
    pub rgmiosOutStats: [MIB_IPMCAST_OIF_STATS; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_IPMCAST_MFE_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type MIB_IPMCAST_MFE_STATS_EX = MIB_IPMCAST_MFE_STATS_EX_XP;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_MFE_STATS_EX_XP {
    pub dwGroup: super::DWORD,
    pub dwSource: super::DWORD,
    pub dwSrcMask: super::DWORD,
    pub dwUpStrmNgbr: super::DWORD,
    pub dwInIfIndex: super::DWORD,
    pub dwInIfProtocol: super::DWORD,
    pub dwRouteProtocol: super::DWORD,
    pub dwRouteNetwork: super::DWORD,
    pub dwRouteMask: super::DWORD,
    pub ulUpTime: super::ULONG,
    pub ulExpiryTime: super::ULONG,
    pub ulNumOutIf: super::ULONG,
    pub ulInPkts: super::ULONG,
    pub ulInOctets: super::ULONG,
    pub ulPktsDifferentIf: super::ULONG,
    pub ulQueueOverflow: super::ULONG,
    pub ulUninitMfe: super::ULONG,
    pub ulNegativeMfe: super::ULONG,
    pub ulInDiscards: super::ULONG,
    pub ulInHdrErrors: super::ULONG,
    pub ulTotalOutPackets: super::ULONG,
    pub rgmiosOutStats: [MIB_IPMCAST_OIF_STATS; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type MIB_IPMCAST_OIF = MIB_IPMCAST_OIF_XP;
#[cfg(feature = "shared")]
pub type MIB_IPMCAST_OIF_STATS = MIB_IPMCAST_OIF_STATS_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_OIF_STATS_LH {
    pub dwOutIfIndex: super::DWORD,
    pub dwNextHopAddr: super::DWORD,
    pub dwDialContext: super::DWORD,
    pub ulTtlTooLow: super::ULONG,
    pub ulFragNeeded: super::ULONG,
    pub ulOutPackets: super::ULONG,
    pub ulOutDiscards: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_OIF_STATS_W2K {
    pub dwOutIfIndex: super::DWORD,
    pub dwNextHopAddr: super::DWORD,
    pub pvDialContext: super::PVOID,
    pub ulTtlTooLow: super::ULONG,
    pub ulFragNeeded: super::ULONG,
    pub ulOutPackets: super::ULONG,
    pub ulOutDiscards: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_OIF_W2K {
    pub dwOutIfIndex: super::DWORD,
    pub dwNextHopAddr: super::DWORD,
    pub pvReserved: super::PVOID,
    pub dwReserved: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_OIF_XP {
    pub dwOutIfIndex: super::DWORD,
    pub dwNextHopAddr: super::DWORD,
    pub dwReserved: super::DWORD,
    pub dwReserved1: super::DWORD,
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type MIB_IPNETROW = MIB_IPNETROW_LH;
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNETROW_LH {
    pub dwIndex: super::IF_INDEX,
    pub dwPhysAddrLen: super::DWORD,
    pub bPhysAddr: [super::UCHAR; 8],
    pub dwAddr: super::DWORD,
    pub Anonymous: MIB_IPNETROW_LH_0,
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
impl Default for MIB_IPNETROW_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union MIB_IPNETROW_LH_0 {
    pub dwType: super::DWORD,
    pub Type: MIB_IPNET_TYPE,
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
impl Default for MIB_IPNETROW_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNETROW_W2K {
    pub dwIndex: super::IF_INDEX,
    pub dwPhysAddrLen: super::DWORD,
    pub bPhysAddr: [super::UCHAR; 8],
    pub dwAddr: super::DWORD,
    pub dwType: super::DWORD,
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
impl Default for MIB_IPNETROW_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_IPNETTABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_IPNETROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "shared"))]
impl Default for MIB_IPNETTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IPNET_TYPE = i32;
pub const MIB_IPNET_TYPE_DYNAMIC: MIB_IPNET_TYPE = 3;
pub const MIB_IPNET_TYPE_INVALID: MIB_IPNET_TYPE = 2;
pub const MIB_IPNET_TYPE_OTHER: MIB_IPNET_TYPE = 1;
pub const MIB_IPNET_TYPE_STATIC: MIB_IPNET_TYPE = 4;
#[cfg(feature = "shared")]
pub const MIB_IPROUTE_METRIC_UNUSED: super::DWORD = 4294967295;
pub const MIB_IPROUTE_TYPE_DIRECT: i32 = 3;
pub const MIB_IPROUTE_TYPE_INDIRECT: i32 = 4;
pub const MIB_IPROUTE_TYPE_INVALID: i32 = 2;
pub const MIB_IPROUTE_TYPE_OTHER: i32 = 1;
#[cfg(feature = "shared")]
pub type MIB_IPSTATS = MIB_IPSTATS_LH;
pub type MIB_IPSTATS_FORWARDING = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_IPSTATS_LH {
    pub Anonymous: MIB_IPSTATS_LH_0,
    pub dwDefaultTTL: super::DWORD,
    pub dwInReceives: super::DWORD,
    pub dwInHdrErrors: super::DWORD,
    pub dwInAddrErrors: super::DWORD,
    pub dwForwDatagrams: super::DWORD,
    pub dwInUnknownProtos: super::DWORD,
    pub dwInDiscards: super::DWORD,
    pub dwInDelivers: super::DWORD,
    pub dwOutRequests: super::DWORD,
    pub dwRoutingDiscards: super::DWORD,
    pub dwOutDiscards: super::DWORD,
    pub dwOutNoRoutes: super::DWORD,
    pub dwReasmTimeout: super::DWORD,
    pub dwReasmReqds: super::DWORD,
    pub dwReasmOks: super::DWORD,
    pub dwReasmFails: super::DWORD,
    pub dwFragOks: super::DWORD,
    pub dwFragFails: super::DWORD,
    pub dwFragCreates: super::DWORD,
    pub dwNumIf: super::DWORD,
    pub dwNumAddr: super::DWORD,
    pub dwNumRoutes: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for MIB_IPSTATS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MIB_IPSTATS_LH_0 {
    pub dwForwarding: super::DWORD,
    pub Forwarding: MIB_IPSTATS_FORWARDING,
}
#[cfg(feature = "shared")]
impl Default for MIB_IPSTATS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPSTATS_W2K {
    pub dwForwarding: super::DWORD,
    pub dwDefaultTTL: super::DWORD,
    pub dwInReceives: super::DWORD,
    pub dwInHdrErrors: super::DWORD,
    pub dwInAddrErrors: super::DWORD,
    pub dwForwDatagrams: super::DWORD,
    pub dwInUnknownProtos: super::DWORD,
    pub dwInDiscards: super::DWORD,
    pub dwInDelivers: super::DWORD,
    pub dwOutRequests: super::DWORD,
    pub dwRoutingDiscards: super::DWORD,
    pub dwOutDiscards: super::DWORD,
    pub dwOutNoRoutes: super::DWORD,
    pub dwReasmTimeout: super::DWORD,
    pub dwReasmReqds: super::DWORD,
    pub dwReasmOks: super::DWORD,
    pub dwReasmFails: super::DWORD,
    pub dwFragOks: super::DWORD,
    pub dwFragFails: super::DWORD,
    pub dwFragCreates: super::DWORD,
    pub dwNumIf: super::DWORD,
    pub dwNumAddr: super::DWORD,
    pub dwNumRoutes: super::DWORD,
}
pub const MIB_IP_FORWARDING: MIB_IPSTATS_FORWARDING = 1;
pub const MIB_IP_NOT_FORWARDING: MIB_IPSTATS_FORWARDING = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_MFE_STATS_TABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_IPMCAST_MFE_STATS; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_MFE_STATS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type MIB_MFE_STATS_TABLE_EX = MIB_MFE_STATS_TABLE_EX_XP;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_MFE_STATS_TABLE_EX_XP {
    pub dwNumEntries: super::DWORD,
    pub table: [PMIB_IPMCAST_MFE_STATS_EX_XP; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_MFE_STATS_TABLE_EX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_MFE_TABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_IPMCAST_MFE; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_MFE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const MIB_USE_CURRENT_FORWARDING: super::DWORD = 4294967295;
#[cfg(feature = "shared")]
pub const MIB_USE_CURRENT_TTL: super::DWORD = 4294967295;
pub const ND_NEIGHBOR_ADVERT: ICMP6_TYPE = 136;
pub const ND_NEIGHBOR_SOLICIT: ICMP6_TYPE = 135;
pub const ND_REDIRECT: ICMP6_TYPE = 137;
pub const ND_ROUTER_ADVERT: ICMP6_TYPE = 134;
pub const ND_ROUTER_SOLICIT: ICMP6_TYPE = 133;
pub type PICMP4_TYPE = *mut ICMP4_TYPE;
pub type PICMP6_TYPE = *mut ICMP6_TYPE;
#[cfg(feature = "shared")]
pub type PMIBICMPSTATS = *mut MIBICMPSTATS;
#[cfg(feature = "shared")]
pub type PMIBICMPSTATS_EX = *mut MIBICMPSTATS_EX_XPSP1;
#[cfg(feature = "shared")]
pub type PMIBICMPSTATS_EX_XPSP1 = *mut MIBICMPSTATS_EX_XPSP1;
#[cfg(feature = "shared")]
pub type PMIB_ICMP = *mut MIB_ICMP;
#[cfg(feature = "shared")]
pub type PMIB_ICMP_EX = *mut MIB_ICMP_EX_XPSP1;
#[cfg(feature = "shared")]
pub type PMIB_ICMP_EX_XPSP1 = *mut MIB_ICMP_EX_XPSP1;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_IPADDRROW = *mut MIB_IPADDRROW_XP;
#[cfg(feature = "shared")]
pub type PMIB_IPADDRROW_W2K = *mut MIB_IPADDRROW_W2K;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_IPADDRROW_XP = *mut MIB_IPADDRROW_XP;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_IPADDRTABLE = *mut MIB_IPADDRTABLE;
#[cfg(feature = "shared")]
pub type PMIB_IPFORWARDNUMBER = *mut MIB_IPFORWARDNUMBER;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
pub type PMIB_IPFORWARDROW = *mut MIB_IPFORWARDROW;
#[cfg(all(feature = "ifdef", feature = "nldef", feature = "shared"))]
pub type PMIB_IPFORWARDTABLE = *mut MIB_IPFORWARDTABLE;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_GLOBAL = *mut MIB_IPMCAST_GLOBAL;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_IF_ENTRY = *mut MIB_IPMCAST_IF_ENTRY;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_IF_TABLE = *mut MIB_IPMCAST_IF_TABLE;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_MFE = *mut MIB_IPMCAST_MFE;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_MFE_STATS = *mut MIB_IPMCAST_MFE_STATS;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_MFE_STATS_EX = *mut MIB_IPMCAST_MFE_STATS_EX_XP;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_MFE_STATS_EX_XP = *mut MIB_IPMCAST_MFE_STATS_EX_XP;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_OIF = *mut MIB_IPMCAST_OIF_XP;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_OIF_STATS = *mut MIB_IPMCAST_OIF_STATS_LH;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_OIF_STATS_LH = *mut MIB_IPMCAST_OIF_STATS_LH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_IPMCAST_OIF_STATS_W2K = *mut MIB_IPMCAST_OIF_STATS_W2K;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_IPMCAST_OIF_W2K = *mut MIB_IPMCAST_OIF_W2K;
#[cfg(feature = "shared")]
pub type PMIB_IPMCAST_OIF_XP = *mut MIB_IPMCAST_OIF_XP;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_IPNETROW = *mut MIB_IPNETROW_LH;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_IPNETROW_LH = *mut MIB_IPNETROW_LH;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_IPNETROW_W2K = *mut MIB_IPNETROW_W2K;
#[cfg(all(feature = "ifdef", feature = "shared"))]
pub type PMIB_IPNETTABLE = *mut MIB_IPNETTABLE;
#[cfg(feature = "shared")]
pub type PMIB_IPSTATS = *mut MIB_IPSTATS_LH;
pub type PMIB_IPSTATS_FORWARDING = *mut MIB_IPSTATS_FORWARDING;
#[cfg(feature = "shared")]
pub type PMIB_IPSTATS_LH = *mut MIB_IPSTATS_LH;
#[cfg(feature = "shared")]
pub type PMIB_IPSTATS_W2K = *mut MIB_IPSTATS_W2K;
#[cfg(feature = "shared")]
pub type PMIB_MFE_STATS_TABLE = *mut MIB_MFE_STATS_TABLE;
#[cfg(feature = "shared")]
pub type PMIB_MFE_STATS_TABLE_EX = *mut MIB_MFE_STATS_TABLE_EX_XP;
#[cfg(feature = "shared")]
pub type PMIB_MFE_STATS_TABLE_EX_XP = *mut MIB_MFE_STATS_TABLE_EX_XP;
#[cfg(feature = "shared")]
pub type PMIB_MFE_TABLE = *mut MIB_MFE_TABLE;
#[cfg(feature = "shared")]
pub const SIZEOF_BASIC_MIB_MFE: super::ULONG = 60;
#[cfg(feature = "shared")]
pub const SIZEOF_BASIC_MIB_MFE_STATS: super::ULONG = 64;
#[cfg(feature = "shared")]
pub const SIZEOF_BASIC_MIB_MFE_STATS_EX: super::ULONG = 84;
