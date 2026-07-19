pub const BEST_IF: u32 = 20;
pub const BEST_ROUTE: u32 = 21;
pub const DEST_LONGER: u32 = 29;
pub const DEST_MATCHING: u32 = 28;
pub const DEST_SHORTER: u32 = 30;
pub const ICMP_STATS: u32 = 11;
pub const IF_NUMBER: u32 = 0;
pub const IF_ROW: u32 = 2;
pub const IF_STATUS: u32 = 25;
pub const IF_TABLE: u32 = 1;
pub const IP6_STATS: u32 = 36;
pub const IPRTRMGR_PID: u32 = 10000;
pub const IP_ADDRROW: u32 = 5;
pub const IP_ADDRTABLE: u32 = 4;
pub const IP_FORWARDNUMBER: u32 = 6;
pub const IP_FORWARDROW: u32 = 8;
pub const IP_FORWARDTABLE: u32 = 7;
pub const IP_NETROW: u32 = 10;
pub const IP_NETTABLE: u32 = 9;
pub const IP_STATS: u32 = 3;
pub const MAX_MIB_OFFSET: u32 = 8;
pub const MAX_SCOPE_NAME_LEN: u32 = 255;
pub const MCAST_BOUNDARY: u32 = 26;
pub const MCAST_GLOBAL: u32 = 24;
pub const MCAST_IF_ENTRY: u32 = 23;
pub const MCAST_MFE: u32 = 18;
pub const MCAST_MFE_STATS: u32 = 19;
pub const MCAST_MFE_STATS_EX: u32 = 35;
pub const MCAST_SCOPE: u32 = 27;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_BEST_IF {
    pub dwDestAddr: u32,
    pub dwIfIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_BOUNDARYROW {
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IFSTATUS {
    pub dwIfIndex: u32,
    pub dwAdminStatus: u32,
    pub dwOperationalStatus: u32,
    pub bMHbeatActive: windows_core::BOOL,
    pub bMHbeatAlive: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPDESTROW {
    pub ForwardRow: super::MIB_IPFORWARDROW,
    pub dwForwardPreference: u32,
    pub dwForwardViewSet: u32,
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
impl Default for MIB_IPDESTROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
#[derive(Clone, Copy)]
pub struct MIB_IPDESTTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPDESTROW; 1],
}
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
impl Default for MIB_IPDESTTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_IPMCAST_BOUNDARY {
    pub dwIfIndex: u32,
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
    pub dwStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_IPMCAST_BOUNDARY_TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_IPMCAST_BOUNDARY; 1],
}
impl Default for MIB_IPMCAST_BOUNDARY_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_IPMCAST_SCOPE {
    pub dwGroupAddress: u32,
    pub dwGroupMask: u32,
    pub snNameBuffer: SCOPE_NAME_BUFFER,
    pub dwStatus: u32,
}
impl Default for MIB_IPMCAST_SCOPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_MCAST_LIMIT_ROW {
    pub dwTtl: u32,
    pub dwRateLimit: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_OPAQUE_INFO {
    pub dwId: u32,
    pub Anonymous: MIB_OPAQUE_INFO_0,
}
impl Default for MIB_OPAQUE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_OPAQUE_INFO_0 {
    pub ullAlign: u64,
    pub rgbyData: [u8; 1],
}
impl Default for MIB_OPAQUE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_OPAQUE_QUERY {
    pub dwVarId: u32,
    pub rgdwVarIndex: [u32; 1],
}
impl Default for MIB_OPAQUE_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_PROXYARP {
    pub dwAddress: u32,
    pub dwMask: u32,
    pub dwIfIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_ROUTESTATE {
    pub bRoutesSetToStack: windows_core::BOOL,
}
pub const NUMBER_OF_EXPORTED_VARIABLES: u32 = 39;
pub type PMIB_BEST_IF = *mut MIB_BEST_IF;
pub type PMIB_BOUNDARYROW = *mut MIB_BOUNDARYROW;
pub type PMIB_IFSTATUS = *mut MIB_IFSTATUS;
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
pub type PMIB_IPDESTROW = *mut MIB_IPDESTROW;
#[cfg(all(feature = "ifdef", feature = "ipmib", feature = "nldef"))]
pub type PMIB_IPDESTTABLE = *mut MIB_IPDESTTABLE;
pub type PMIB_IPMCAST_BOUNDARY = *mut MIB_IPMCAST_BOUNDARY;
pub type PMIB_IPMCAST_BOUNDARY_TABLE = *mut MIB_IPMCAST_BOUNDARY_TABLE;
pub type PMIB_IPMCAST_SCOPE = *mut MIB_IPMCAST_SCOPE;
pub type PMIB_MCAST_LIMIT_ROW = *mut MIB_MCAST_LIMIT_ROW;
pub type PMIB_OPAQUE_INFO = *mut MIB_OPAQUE_INFO;
pub type PMIB_OPAQUE_QUERY = *mut MIB_OPAQUE_QUERY;
pub type PMIB_PROXYARP = *mut MIB_PROXYARP;
pub type PMIB_ROUTESTATE = *mut MIB_ROUTESTATE;
pub const PROXY_ARP: u32 = 22;
#[cfg(feature = "winnt")]
pub type PTCPIP_OWNER_MODULE_BASIC_INFO = *mut TCPIP_OWNER_MODULE_BASIC_INFO;
pub type PTCPIP_OWNER_MODULE_INFO_CLASS = *mut TCPIP_OWNER_MODULE_INFO_CLASS;
pub type PTCP_TABLE_CLASS = *mut TCP_TABLE_CLASS;
pub type PUDP_TABLE_CLASS = *mut UDP_TABLE_CLASS;
pub const ROUTE_LONGER: u32 = 32;
pub const ROUTE_MATCHING: u32 = 31;
pub const ROUTE_SHORTER: u32 = 33;
pub const ROUTE_STATE: u32 = 34;
pub type SCOPE_NAME = *mut SN_CHAR;
pub type SCOPE_NAME_BUFFER = [SN_CHAR; 256];
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SN_CHAR(pub u16);
pub const TCP6_STATS: u32 = 38;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCPIP_OWNER_MODULE_BASIC_INFO {
    pub pModuleName: super::PWCHAR,
    pub pModulePath: super::PWCHAR,
}
pub const TCPIP_OWNER_MODULE_INFO_BASIC: TCPIP_OWNER_MODULE_INFO_CLASS = 0;
pub type TCPIP_OWNER_MODULE_INFO_CLASS = i32;
pub const TCP_ROW: u32 = 14;
pub const TCP_STATS: u32 = 12;
pub const TCP_TABLE: u32 = 13;
pub const TCP_TABLE_BASIC_ALL: TCP_TABLE_CLASS = 2;
pub const TCP_TABLE_BASIC_CONNECTIONS: TCP_TABLE_CLASS = 1;
pub const TCP_TABLE_BASIC_LISTENER: TCP_TABLE_CLASS = 0;
pub type TCP_TABLE_CLASS = i32;
pub const TCP_TABLE_OWNER_MODULE_ALL: TCP_TABLE_CLASS = 8;
pub const TCP_TABLE_OWNER_MODULE_CONNECTIONS: TCP_TABLE_CLASS = 7;
pub const TCP_TABLE_OWNER_MODULE_LISTENER: TCP_TABLE_CLASS = 6;
pub const TCP_TABLE_OWNER_PID_ALL: TCP_TABLE_CLASS = 5;
pub const TCP_TABLE_OWNER_PID_CONNECTIONS: TCP_TABLE_CLASS = 4;
pub const TCP_TABLE_OWNER_PID_LISTENER: TCP_TABLE_CLASS = 3;
pub const UDP6_STATS: u32 = 37;
pub const UDP_ROW: u32 = 17;
pub const UDP_STATS: u32 = 15;
pub const UDP_TABLE: u32 = 16;
pub const UDP_TABLE_BASIC: UDP_TABLE_CLASS = 0;
pub type UDP_TABLE_CLASS = i32;
pub const UDP_TABLE_OWNER_MODULE: UDP_TABLE_CLASS = 2;
pub const UDP_TABLE_OWNER_PID: UDP_TABLE_CLASS = 1;
