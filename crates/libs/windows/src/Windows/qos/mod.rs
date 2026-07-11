#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const QOS_GENERAL_ID_BASE: u32 = 2000;
pub const QOS_NOT_SPECIFIED: u32 = 4294967295;
pub const QOS_OBJECT_DESTADDR: u32 = 2004;
pub const QOS_OBJECT_END_OF_LIST: u32 = 2001;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_OBJECT_HDR {
    pub ObjectType: u32,
    pub ObjectLength: u32,
}
pub const QOS_OBJECT_SD_MODE: u32 = 2002;
pub const QOS_OBJECT_SHAPING_RATE: u32 = 2003;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_SD_MODE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapeDiscardMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_SHAPING_RATE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapingRate: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SERVICETYPE(pub u32);
pub const SERVICETYPE_BESTEFFORT: u32 = 1;
pub const SERVICETYPE_CONTROLLEDLOAD: u32 = 2;
pub const SERVICETYPE_GENERAL_INFORMATION: u32 = 5;
pub const SERVICETYPE_GUARANTEED: u32 = 3;
pub const SERVICETYPE_NETWORK_CONTROL: u32 = 10;
pub const SERVICETYPE_NETWORK_UNAVAILABLE: u32 = 4;
pub const SERVICETYPE_NOCHANGE: u32 = 6;
pub const SERVICETYPE_NONCONFORMING: u32 = 9;
pub const SERVICETYPE_NOTRAFFIC: u32 = 0;
pub const SERVICETYPE_QUALITATIVE: u32 = 13;
pub const SERVICE_BESTEFFORT: u32 = 2147549184;
pub const SERVICE_CONTROLLEDLOAD: u32 = 2147614720;
pub const SERVICE_GUARANTEED: u32 = 2147745792;
pub const SERVICE_NO_QOS_SIGNALING: u32 = 1073741824;
pub const SERVICE_NO_TRAFFIC_CONTROL: u32 = 2164260864;
pub const SERVICE_QUALITATIVE: u32 = 2149580800;
pub const TC_NONCONF_BORROW: u32 = 0;
pub const TC_NONCONF_BORROW_PLUS: u32 = 3;
pub const TC_NONCONF_DISCARD: u32 = 2;
pub const TC_NONCONF_SHAPE: u32 = 1;
