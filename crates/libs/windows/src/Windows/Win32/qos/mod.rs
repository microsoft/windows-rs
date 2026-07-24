#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FLOWSPEC {
    pub TokenRate: u32,
    pub TokenBucketSize: u32,
    pub PeakBandwidth: u32,
    pub Latency: u32,
    pub DelayVariation: u32,
    pub ServiceType: SERVICETYPE,
    pub MaxSduSize: u32,
    pub MinimumPolicedSize: u32,
}
pub type LPFLOWSPEC = *mut FLOWSPEC;
pub type LPQOS_OBJECT_HDR = *mut QOS_OBJECT_HDR;
pub type LPQOS_SD_MODE = *mut QOS_SD_MODE;
pub type LPQOS_SHAPING_RATE = *mut QOS_SHAPING_RATE;
pub type PFLOWSPEC = *mut FLOWSPEC;
pub const POSITIVE_INFINITY_RATE: u32 = 4294967294;
pub const QOS_GENERAL_ID_BASE: i32 = 2000;
pub const QOS_NOT_SPECIFIED: u32 = 4294967295;
pub const QOS_OBJECT_DESTADDR: i32 = 2004;
pub const QOS_OBJECT_END_OF_LIST: i32 = 2001;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_OBJECT_HDR {
    pub ObjectType: u32,
    pub ObjectLength: u32,
}
pub const QOS_OBJECT_SD_MODE: i32 = 2002;
pub const QOS_OBJECT_SHAPING_RATE: i32 = 2003;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_SD_MODE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapeDiscardMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_SHAPING_RATE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapingRate: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SERVICETYPE(pub u32);
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
