#[cfg(all(feature = "qos", feature = "shared"))]
pub type LPQOS_DIFFSERV = *mut QOS_DIFFSERV;
#[cfg(feature = "shared")]
pub type LPQOS_DIFFSERV_RULE = *mut QOS_DIFFSERV_RULE;
#[cfg(all(feature = "qos", feature = "shared"))]
pub type LPQOS_DS_CLASS = *mut QOS_DS_CLASS;
#[cfg(all(feature = "qos", feature = "shared", feature = "winnt"))]
pub type LPQOS_FRIENDLY_NAME = *mut QOS_FRIENDLY_NAME;
#[cfg(all(feature = "qos", feature = "shared"))]
pub type LPQOS_TCP_TRAFFIC = *mut QOS_TCP_TRAFFIC;
#[cfg(all(feature = "qos", feature = "shared"))]
pub type LPQOS_TRAFFIC_CLASS = *mut QOS_TRAFFIC_CLASS;
#[repr(C)]
#[cfg(all(feature = "qos", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct QOS_DIFFSERV {
    pub ObjectHdr: super::QOS_OBJECT_HDR,
    pub DSFieldCount: super::ULONG,
    pub DiffservRule: [super::UCHAR; 1],
}
#[cfg(all(feature = "qos", feature = "shared"))]
impl Default for QOS_DIFFSERV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QOS_DIFFSERV_RULE {
    pub InboundDSField: super::UCHAR,
    pub ConformingOutboundDSField: super::UCHAR,
    pub NonConformingOutboundDSField: super::UCHAR,
    pub ConformingUserPriority: super::UCHAR,
    pub NonConformingUserPriority: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "qos", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct QOS_DS_CLASS {
    pub ObjectHdr: super::QOS_OBJECT_HDR,
    pub DSField: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "qos", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct QOS_FRIENDLY_NAME {
    pub ObjectHdr: super::QOS_OBJECT_HDR,
    pub FriendlyName: [super::WCHAR; 256],
}
#[cfg(all(feature = "qos", feature = "shared", feature = "winnt"))]
impl Default for QOS_FRIENDLY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QOS_MAX_OBJECT_STRING_LENGTH: i32 = 256;
pub const QOS_OBJECT_DIFFSERV: i32 = 4003;
pub const QOS_OBJECT_DS_CLASS: i32 = 4001;
pub const QOS_OBJECT_FRIENDLY_NAME: i32 = 4005;
pub const QOS_OBJECT_TCP_TRAFFIC: i32 = 4004;
pub const QOS_OBJECT_TRAFFIC_CLASS: i32 = 4002;
#[repr(C)]
#[cfg(all(feature = "qos", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct QOS_TCP_TRAFFIC {
    pub ObjectHdr: super::QOS_OBJECT_HDR,
}
#[repr(C)]
#[cfg(all(feature = "qos", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct QOS_TRAFFIC_CLASS {
    pub ObjectHdr: super::QOS_OBJECT_HDR,
    pub TrafficClass: super::ULONG,
}
pub const QOS_TRAFFIC_GENERAL_ID_BASE: i32 = 4000;
