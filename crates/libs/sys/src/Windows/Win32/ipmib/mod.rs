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
pub const ICMP6_INFOMSG_MASK: u32 = 128;
pub const ICMP6_MEMBERSHIP_QUERY: ICMP6_TYPE = 130;
pub const ICMP6_MEMBERSHIP_REDUCTION: ICMP6_TYPE = 132;
pub const ICMP6_MEMBERSHIP_REPORT: ICMP6_TYPE = 131;
pub const ICMP6_PACKET_TOO_BIG: ICMP6_TYPE = 2;
pub const ICMP6_PARAM_PROB: ICMP6_TYPE = 4;
pub const ICMP6_TIME_EXCEEDED: ICMP6_TYPE = 3;
pub type ICMP6_TYPE = i32;
pub const ICMP6_V2_MEMBERSHIP_REPORT: ICMP6_TYPE = 143;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIBICMPINFO {
    pub icmpInStats: MIBICMPSTATS,
    pub icmpOutStats: MIBICMPSTATS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIBICMPSTATS {
    pub dwMsgs: u32,
    pub dwErrors: u32,
    pub dwDestUnreachs: u32,
    pub dwTimeExcds: u32,
    pub dwParmProbs: u32,
    pub dwSrcQuenchs: u32,
    pub dwRedirects: u32,
    pub dwEchos: u32,
    pub dwEchoReps: u32,
    pub dwTimestamps: u32,
    pub dwTimestampReps: u32,
    pub dwAddrMasks: u32,
    pub dwAddrMaskReps: u32,
}
pub type MIBICMPSTATS_EX = MIBICMPSTATS_EX_XPSP1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIBICMPSTATS_EX_XPSP1 {
    pub dwMsgs: u32,
    pub dwErrors: u32,
    pub rgdwTypeCount: [u32; 256],
}
impl Default for MIBICMPSTATS_EX_XPSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_ICMP {
    pub stats: MIBICMPINFO,
}
pub type MIB_ICMP_EX = MIB_ICMP_EX_XPSP1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_ICMP_EX_XPSP1 {
    pub icmpInStats: MIBICMPSTATS_EX,
    pub icmpOutStats: MIBICMPSTATS_EX,
}
#[cfg(feature = "ifdef")]
pub type MIB_IPADDRROW = MIB_IPADDRROW_XP;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPADDRROW_W2K {
    pub dwAddr: u32,
    pub dwIndex: u32,
    pub dwMask: u32,
    pub dwBCastAddr: u32,
    pub dwReasmSize: u32,
    pub unused1: u16,
    pub unused2: u16,
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPADDRROW_XP {
    pub dwAddr: u32,
    pub dwIndex: super::ifdef::IF_INDEX,
    pub dwMask: u32,
    pub dwBCastAddr: u32,
    pub dwReasmSize: u32,
    pub unused1: u16,
    pub wType: u16,
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy)]
pub struct MIB_IPADDRTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPADDRROW; 1],
}
#[cfg(feature = "ifdef")]
impl Default for MIB_IPADDRTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_IPADDR_DELETED: u32 = 64;
pub const MIB_IPADDR_DISCONNECTED: u32 = 8;
pub const MIB_IPADDR_DNS_ELIGIBLE: u32 = 256;
pub const MIB_IPADDR_DYNAMIC: u32 = 4;
pub const MIB_IPADDR_PRIMARY: u32 = 1;
pub const MIB_IPADDR_TRANSIENT: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPFORWARDNUMBER {
    pub dwValue: u32,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARDROW {
    pub dwForwardDest: u32,
    pub dwForwardMask: u32,
    pub dwForwardPolicy: u32,
    pub dwForwardNextHop: u32,
    pub dwForwardIfIndex: super::ifdef::IF_INDEX,
    pub Anonymous: MIB_IPFORWARDROW_0,
    pub Anonymous2: MIB_IPFORWARDROW_1,
    pub dwForwardAge: u32,
    pub dwForwardNextHopAS: u32,
    pub dwForwardMetric1: u32,
    pub dwForwardMetric2: u32,
    pub dwForwardMetric3: u32,
    pub dwForwardMetric4: u32,
    pub dwForwardMetric5: u32,
}
#[cfg(all(feature = "ifdef", feature = "nldef"))]
impl Default for MIB_IPFORWARDROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef"))]
#[derive(Clone, Copy)]
pub union MIB_IPFORWARDROW_0 {
    pub dwForwardType: u32,
    pub ForwardType: MIB_IPFORWARD_TYPE,
}
#[cfg(all(feature = "ifdef", feature = "nldef"))]
impl Default for MIB_IPFORWARDROW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef"))]
#[derive(Clone, Copy)]
pub union MIB_IPFORWARDROW_1 {
    pub dwForwardProto: u32,
    pub ForwardProto: MIB_IPFORWARD_PROTO,
}
#[cfg(all(feature = "ifdef", feature = "nldef"))]
impl Default for MIB_IPFORWARDROW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "nldef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPFORWARDTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPFORWARDROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "nldef"))]
impl Default for MIB_IPFORWARDTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "nldef")]
pub type MIB_IPFORWARD_PROTO = super::nldef::NL_ROUTE_PROTOCOL;
pub type MIB_IPFORWARD_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_GLOBAL {
    pub dwEnable: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_IF_ENTRY {
    pub dwIfIndex: u32,
    pub dwTtl: u32,
    pub dwProtocol: u32,
    pub dwRateLimit: u32,
    pub ulInMcastOctets: u32,
    pub ulOutMcastOctets: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_IF_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_IF_ENTRY; 1],
}
impl Default for MIB_IPMCAST_IF_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_MFE {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulTimeOut: u32,
    pub ulNumOutIf: u32,
    pub fFlags: u32,
    pub dwReserved: u32,
    pub rgmioOutInfo: [MIB_IPMCAST_OIF; 1],
}
impl Default for MIB_IPMCAST_MFE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_MFE_STATS {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulNumOutIf: u32,
    pub ulInPkts: u32,
    pub ulInOctets: u32,
    pub ulPktsDifferentIf: u32,
    pub ulQueueOverflow: u32,
    pub rgmiosOutStats: [MIB_IPMCAST_OIF_STATS; 1],
}
impl Default for MIB_IPMCAST_MFE_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IPMCAST_MFE_STATS_EX = MIB_IPMCAST_MFE_STATS_EX_XP;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_MFE_STATS_EX_XP {
    pub dwGroup: u32,
    pub dwSource: u32,
    pub dwSrcMask: u32,
    pub dwUpStrmNgbr: u32,
    pub dwInIfIndex: u32,
    pub dwInIfProtocol: u32,
    pub dwRouteProtocol: u32,
    pub dwRouteNetwork: u32,
    pub dwRouteMask: u32,
    pub ulUpTime: u32,
    pub ulExpiryTime: u32,
    pub ulNumOutIf: u32,
    pub ulInPkts: u32,
    pub ulInOctets: u32,
    pub ulPktsDifferentIf: u32,
    pub ulQueueOverflow: u32,
    pub ulUninitMfe: u32,
    pub ulNegativeMfe: u32,
    pub ulInDiscards: u32,
    pub ulInHdrErrors: u32,
    pub ulTotalOutPackets: u32,
    pub rgmiosOutStats: [MIB_IPMCAST_OIF_STATS; 1],
}
impl Default for MIB_IPMCAST_MFE_STATS_EX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_IPMCAST_OIF = MIB_IPMCAST_OIF_XP;
pub type MIB_IPMCAST_OIF_STATS = MIB_IPMCAST_OIF_STATS_LH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_OIF_STATS_LH {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub dwDialContext: u32,
    pub ulTtlTooLow: u32,
    pub ulFragNeeded: u32,
    pub ulOutPackets: u32,
    pub ulOutDiscards: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_OIF_STATS_W2K {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub pvDialContext: *mut core::ffi::c_void,
    pub ulTtlTooLow: u32,
    pub ulFragNeeded: u32,
    pub ulOutPackets: u32,
    pub ulOutDiscards: u32,
}
impl Default for MIB_IPMCAST_OIF_STATS_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPMCAST_OIF_W2K {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
}
impl Default for MIB_IPMCAST_OIF_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPMCAST_OIF_XP {
    pub dwOutIfIndex: u32,
    pub dwNextHopAddr: u32,
    pub dwReserved: u32,
    pub dwReserved1: u32,
}
#[cfg(feature = "ifdef")]
pub type MIB_IPNETROW = MIB_IPNETROW_LH;
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy)]
pub struct MIB_IPNETROW_LH {
    pub dwIndex: super::ifdef::IF_INDEX,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAddr: u32,
    pub Anonymous: MIB_IPNETROW_LH_0,
}
#[cfg(feature = "ifdef")]
impl Default for MIB_IPNETROW_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy)]
pub union MIB_IPNETROW_LH_0 {
    pub dwType: u32,
    pub Type: MIB_IPNET_TYPE,
}
#[cfg(feature = "ifdef")]
impl Default for MIB_IPNETROW_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy)]
pub struct MIB_IPNETROW_W2K {
    pub dwIndex: super::ifdef::IF_INDEX,
    pub dwPhysAddrLen: u32,
    pub bPhysAddr: [u8; 8],
    pub dwAddr: u32,
    pub dwType: u32,
}
#[cfg(feature = "ifdef")]
impl Default for MIB_IPNETROW_W2K {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ifdef")]
#[derive(Clone, Copy)]
pub struct MIB_IPNETTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPNETROW; 1],
}
#[cfg(feature = "ifdef")]
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
pub const MIB_IPROUTE_METRIC_UNUSED: u32 = 4294967295;
pub const MIB_IPROUTE_TYPE_DIRECT: MIB_IPFORWARD_TYPE = 3;
pub const MIB_IPROUTE_TYPE_INDIRECT: MIB_IPFORWARD_TYPE = 4;
pub const MIB_IPROUTE_TYPE_INVALID: MIB_IPFORWARD_TYPE = 2;
pub const MIB_IPROUTE_TYPE_OTHER: MIB_IPFORWARD_TYPE = 1;
pub type MIB_IPSTATS = MIB_IPSTATS_LH;
pub type MIB_IPSTATS_FORWARDING = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_IPSTATS_LH {
    pub Anonymous: MIB_IPSTATS_LH_0,
    pub dwDefaultTTL: u32,
    pub dwInReceives: u32,
    pub dwInHdrErrors: u32,
    pub dwInAddrErrors: u32,
    pub dwForwDatagrams: u32,
    pub dwInUnknownProtos: u32,
    pub dwInDiscards: u32,
    pub dwInDelivers: u32,
    pub dwOutRequests: u32,
    pub dwRoutingDiscards: u32,
    pub dwOutDiscards: u32,
    pub dwOutNoRoutes: u32,
    pub dwReasmTimeout: u32,
    pub dwReasmReqds: u32,
    pub dwReasmOks: u32,
    pub dwReasmFails: u32,
    pub dwFragOks: u32,
    pub dwFragFails: u32,
    pub dwFragCreates: u32,
    pub dwNumIf: u32,
    pub dwNumAddr: u32,
    pub dwNumRoutes: u32,
}
impl Default for MIB_IPSTATS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_IPSTATS_LH_0 {
    pub dwForwarding: u32,
    pub Forwarding: MIB_IPSTATS_FORWARDING,
}
impl Default for MIB_IPSTATS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_IPSTATS_W2K {
    pub dwForwarding: u32,
    pub dwDefaultTTL: u32,
    pub dwInReceives: u32,
    pub dwInHdrErrors: u32,
    pub dwInAddrErrors: u32,
    pub dwForwDatagrams: u32,
    pub dwInUnknownProtos: u32,
    pub dwInDiscards: u32,
    pub dwInDelivers: u32,
    pub dwOutRequests: u32,
    pub dwRoutingDiscards: u32,
    pub dwOutDiscards: u32,
    pub dwOutNoRoutes: u32,
    pub dwReasmTimeout: u32,
    pub dwReasmReqds: u32,
    pub dwReasmOks: u32,
    pub dwReasmFails: u32,
    pub dwFragOks: u32,
    pub dwFragFails: u32,
    pub dwFragCreates: u32,
    pub dwNumIf: u32,
    pub dwNumAddr: u32,
    pub dwNumRoutes: u32,
}
pub const MIB_IP_FORWARDING: MIB_IPSTATS_FORWARDING = 1;
pub const MIB_IP_NOT_FORWARDING: MIB_IPSTATS_FORWARDING = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_MFE_STATS_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_MFE_STATS; 1],
}
impl Default for MIB_MFE_STATS_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_MFE_STATS_TABLE_EX = MIB_MFE_STATS_TABLE_EX_XP;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_MFE_STATS_TABLE_EX_XP {
    pub dwNumEntries: u32,
    pub table: [PMIB_IPMCAST_MFE_STATS_EX_XP; 1],
}
impl Default for MIB_MFE_STATS_TABLE_EX_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_MFE_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_MFE; 1],
}
impl Default for MIB_MFE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_USE_CURRENT_FORWARDING: u32 = 4294967295;
pub const MIB_USE_CURRENT_TTL: u32 = 4294967295;
pub const ND_NEIGHBOR_ADVERT: ICMP6_TYPE = 136;
pub const ND_NEIGHBOR_SOLICIT: ICMP6_TYPE = 135;
pub const ND_REDIRECT: ICMP6_TYPE = 137;
pub const ND_ROUTER_ADVERT: ICMP6_TYPE = 134;
pub const ND_ROUTER_SOLICIT: ICMP6_TYPE = 133;
pub type PICMP4_TYPE = *mut ICMP4_TYPE;
pub type PICMP6_TYPE = *mut ICMP6_TYPE;
pub type PMIBICMPSTATS = *mut MIBICMPSTATS;
pub type PMIBICMPSTATS_EX = *mut MIBICMPSTATS_EX_XPSP1;
pub type PMIBICMPSTATS_EX_XPSP1 = *mut MIBICMPSTATS_EX_XPSP1;
pub type PMIB_ICMP = *mut MIB_ICMP;
pub type PMIB_ICMP_EX = *mut MIB_ICMP_EX_XPSP1;
pub type PMIB_ICMP_EX_XPSP1 = *mut MIB_ICMP_EX_XPSP1;
#[cfg(feature = "ifdef")]
pub type PMIB_IPADDRROW = *mut MIB_IPADDRROW_XP;
pub type PMIB_IPADDRROW_W2K = *mut MIB_IPADDRROW_W2K;
#[cfg(feature = "ifdef")]
pub type PMIB_IPADDRROW_XP = *mut MIB_IPADDRROW_XP;
#[cfg(feature = "ifdef")]
pub type PMIB_IPADDRTABLE = *mut MIB_IPADDRTABLE;
pub type PMIB_IPFORWARDNUMBER = *mut MIB_IPFORWARDNUMBER;
#[cfg(all(feature = "ifdef", feature = "nldef"))]
pub type PMIB_IPFORWARDROW = *mut MIB_IPFORWARDROW;
#[cfg(all(feature = "ifdef", feature = "nldef"))]
pub type PMIB_IPFORWARDTABLE = *mut MIB_IPFORWARDTABLE;
pub type PMIB_IPMCAST_GLOBAL = *mut MIB_IPMCAST_GLOBAL;
pub type PMIB_IPMCAST_IF_ENTRY = *mut MIB_IPMCAST_IF_ENTRY;
pub type PMIB_IPMCAST_IF_TABLE = *mut MIB_IPMCAST_IF_TABLE;
pub type PMIB_IPMCAST_MFE = *mut MIB_IPMCAST_MFE;
pub type PMIB_IPMCAST_MFE_STATS = *mut MIB_IPMCAST_MFE_STATS;
pub type PMIB_IPMCAST_MFE_STATS_EX = *mut MIB_IPMCAST_MFE_STATS_EX_XP;
pub type PMIB_IPMCAST_MFE_STATS_EX_XP = *mut MIB_IPMCAST_MFE_STATS_EX_XP;
pub type PMIB_IPMCAST_OIF = *mut MIB_IPMCAST_OIF_XP;
pub type PMIB_IPMCAST_OIF_STATS = *mut MIB_IPMCAST_OIF_STATS_LH;
pub type PMIB_IPMCAST_OIF_STATS_LH = *mut MIB_IPMCAST_OIF_STATS_LH;
pub type PMIB_IPMCAST_OIF_STATS_W2K = *mut MIB_IPMCAST_OIF_STATS_W2K;
pub type PMIB_IPMCAST_OIF_W2K = *mut MIB_IPMCAST_OIF_W2K;
pub type PMIB_IPMCAST_OIF_XP = *mut MIB_IPMCAST_OIF_XP;
#[cfg(feature = "ifdef")]
pub type PMIB_IPNETROW = *mut MIB_IPNETROW_LH;
#[cfg(feature = "ifdef")]
pub type PMIB_IPNETROW_LH = *mut MIB_IPNETROW_LH;
#[cfg(feature = "ifdef")]
pub type PMIB_IPNETROW_W2K = *mut MIB_IPNETROW_W2K;
#[cfg(feature = "ifdef")]
pub type PMIB_IPNETTABLE = *mut MIB_IPNETTABLE;
pub type PMIB_IPSTATS = *mut MIB_IPSTATS_LH;
pub type PMIB_IPSTATS_FORWARDING = *mut MIB_IPSTATS_FORWARDING;
pub type PMIB_IPSTATS_LH = *mut MIB_IPSTATS_LH;
pub type PMIB_IPSTATS_W2K = *mut MIB_IPSTATS_W2K;
pub type PMIB_MFE_STATS_TABLE = *mut MIB_MFE_STATS_TABLE;
pub type PMIB_MFE_STATS_TABLE_EX = *mut MIB_MFE_STATS_TABLE_EX_XP;
pub type PMIB_MFE_STATS_TABLE_EX_XP = *mut MIB_MFE_STATS_TABLE_EX_XP;
pub type PMIB_MFE_TABLE = *mut MIB_MFE_TABLE;
pub const SIZEOF_BASIC_MIB_MFE: u32 = 60;
pub const SIZEOF_BASIC_MIB_MFE_STATS: u32 = 64;
pub const SIZEOF_BASIC_MIB_MFE_STATS_EX: u32 = 84;
