#[cfg(feature = "Win32_qos")]
pub type LPQOS_DIFFSERV = *mut QOS_DIFFSERV;
pub type LPQOS_DIFFSERV_RULE = *mut QOS_DIFFSERV_RULE;
#[cfg(feature = "Win32_qos")]
pub type LPQOS_DS_CLASS = *mut QOS_DS_CLASS;
#[cfg(feature = "Win32_qos")]
pub type LPQOS_FRIENDLY_NAME = *mut QOS_FRIENDLY_NAME;
#[cfg(feature = "Win32_qos")]
pub type LPQOS_TCP_TRAFFIC = *mut QOS_TCP_TRAFFIC;
#[cfg(feature = "Win32_qos")]
pub type LPQOS_TRAFFIC_CLASS = *mut QOS_TRAFFIC_CLASS;
#[repr(C)]
#[cfg(feature = "Win32_qos")]
#[derive(Clone, Copy)]
pub struct QOS_DIFFSERV {
    pub ObjectHdr: super::qos::QOS_OBJECT_HDR,
    pub DSFieldCount: u32,
    pub DiffservRule: [u8; 1],
}
#[cfg(feature = "Win32_qos")]
impl Default for QOS_DIFFSERV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QOS_DIFFSERV_RULE {
    pub InboundDSField: u8,
    pub ConformingOutboundDSField: u8,
    pub NonConformingOutboundDSField: u8,
    pub ConformingUserPriority: u8,
    pub NonConformingUserPriority: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_qos")]
#[derive(Clone, Copy, Default)]
pub struct QOS_DS_CLASS {
    pub ObjectHdr: super::qos::QOS_OBJECT_HDR,
    pub DSField: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_qos")]
#[derive(Clone, Copy)]
pub struct QOS_FRIENDLY_NAME {
    pub ObjectHdr: super::qos::QOS_OBJECT_HDR,
    pub FriendlyName: [u16; 256],
}
#[cfg(feature = "Win32_qos")]
impl Default for QOS_FRIENDLY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QOS_MAX_OBJECT_STRING_LENGTH: u32 = 256;
pub const QOS_OBJECT_DIFFSERV: u32 = 4003;
pub const QOS_OBJECT_DS_CLASS: u32 = 4001;
pub const QOS_OBJECT_FRIENDLY_NAME: u32 = 4005;
pub const QOS_OBJECT_TCP_TRAFFIC: u32 = 4004;
pub const QOS_OBJECT_TRAFFIC_CLASS: u32 = 4002;
#[repr(C)]
#[cfg(feature = "Win32_qos")]
#[derive(Clone, Copy, Default)]
pub struct QOS_TCP_TRAFFIC {
    pub ObjectHdr: super::qos::QOS_OBJECT_HDR,
}
#[repr(C)]
#[cfg(feature = "Win32_qos")]
#[derive(Clone, Copy, Default)]
pub struct QOS_TRAFFIC_CLASS {
    pub ObjectHdr: super::qos::QOS_OBJECT_HDR,
    pub TrafficClass: u32,
}
pub const QOS_TRAFFIC_GENERAL_ID_BASE: u32 = 4000;
