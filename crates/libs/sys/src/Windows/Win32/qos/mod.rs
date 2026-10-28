#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FLOWSPEC {
    pub TokenRate: super::ULONG,
    pub TokenBucketSize: super::ULONG,
    pub PeakBandwidth: super::ULONG,
    pub Latency: super::ULONG,
    pub DelayVariation: super::ULONG,
    pub ServiceType: SERVICETYPE,
    pub MaxSduSize: super::ULONG,
    pub MinimumPolicedSize: super::ULONG,
}
#[cfg(feature = "shared")]
pub type LPFLOWSPEC = *mut FLOWSPEC;
#[cfg(feature = "shared")]
pub type LPQOS_OBJECT_HDR = *mut QOS_OBJECT_HDR;
#[cfg(feature = "shared")]
pub type LPQOS_SD_MODE = *mut QOS_SD_MODE;
#[cfg(feature = "shared")]
pub type LPQOS_SHAPING_RATE = *mut QOS_SHAPING_RATE;
#[cfg(feature = "shared")]
pub type PFLOWSPEC = *mut FLOWSPEC;
pub const POSITIVE_INFINITY_RATE: u32 = 4294967294;
pub const QOS_GENERAL_ID_BASE: i32 = 2000;
pub const QOS_NOT_SPECIFIED: u32 = 4294967295;
pub const QOS_OBJECT_DESTADDR: i32 = 2004;
pub const QOS_OBJECT_END_OF_LIST: i32 = 2001;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QOS_OBJECT_HDR {
    pub ObjectType: super::ULONG,
    pub ObjectLength: super::ULONG,
}
pub const QOS_OBJECT_SD_MODE: i32 = 2002;
pub const QOS_OBJECT_SHAPING_RATE: i32 = 2003;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QOS_SD_MODE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapeDiscardMode: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QOS_SHAPING_RATE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapingRate: super::ULONG,
}
pub type SERVICETYPE = u32;
pub const SERVICETYPE_BESTEFFORT: i32 = 1;
pub const SERVICETYPE_CONTROLLEDLOAD: i32 = 2;
pub const SERVICETYPE_GENERAL_INFORMATION: i32 = 5;
pub const SERVICETYPE_GUARANTEED: i32 = 3;
pub const SERVICETYPE_NETWORK_CONTROL: i32 = 10;
pub const SERVICETYPE_NETWORK_UNAVAILABLE: i32 = 4;
pub const SERVICETYPE_NOCHANGE: i32 = 6;
pub const SERVICETYPE_NONCONFORMING: i32 = 9;
pub const SERVICETYPE_NOTRAFFIC: i32 = 0;
pub const SERVICETYPE_QUALITATIVE: i32 = 13;
pub const SERVICE_BESTEFFORT: u32 = 2147549184;
pub const SERVICE_CONTROLLEDLOAD: u32 = 2147614720;
pub const SERVICE_GUARANTEED: u32 = 2147745792;
pub const SERVICE_NO_QOS_SIGNALING: i32 = 1073741824;
pub const SERVICE_NO_TRAFFIC_CONTROL: u32 = 2164260864;
pub const SERVICE_QUALITATIVE: u32 = 2149580800;
pub const TC_NONCONF_BORROW: i32 = 0;
pub const TC_NONCONF_BORROW_PLUS: i32 = 3;
pub const TC_NONCONF_DISCARD: i32 = 2;
pub const TC_NONCONF_SHAPE: i32 = 1;
