pub type LPSDP_LARGE_INTEGER_16 = *mut SDP_LARGE_INTEGER_16;
pub type LPSDP_ULARGE_INTEGER_16 = *mut SDP_ULARGE_INTEGER_16;
pub type NodeContainerType = i32;
pub const NodeContainerTypeAlternative: NodeContainerType = 1;
pub const NodeContainerTypeSequence: NodeContainerType = 0;
pub type PSDP_ERROR = *mut u16;
pub type PSDP_LARGE_INTEGER_16 = *mut SDP_LARGE_INTEGER_16;
pub type PSDP_ULARGE_INTEGER_16 = *mut SDP_ULARGE_INTEGER_16;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SDP_ERROR(pub u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SDP_LARGE_INTEGER_16 {
    pub LowPart: u64,
    pub HighPart: i64,
}
pub type SDP_SPECIFICTYPE = i32;
pub const SDP_ST_INT128: SDP_SPECIFICTYPE = 1056;
pub const SDP_ST_INT16: SDP_SPECIFICTYPE = 288;
pub const SDP_ST_INT32: SDP_SPECIFICTYPE = 544;
pub const SDP_ST_INT64: SDP_SPECIFICTYPE = 800;
pub const SDP_ST_INT8: SDP_SPECIFICTYPE = 32;
pub const SDP_ST_NONE: SDP_SPECIFICTYPE = 0;
pub const SDP_ST_UINT128: SDP_SPECIFICTYPE = 1040;
pub const SDP_ST_UINT16: SDP_SPECIFICTYPE = 272;
pub const SDP_ST_UINT32: SDP_SPECIFICTYPE = 528;
pub const SDP_ST_UINT64: SDP_SPECIFICTYPE = 784;
pub const SDP_ST_UINT8: SDP_SPECIFICTYPE = 16;
pub const SDP_ST_UUID128: SDP_SPECIFICTYPE = 1072;
pub const SDP_ST_UUID16: SDP_SPECIFICTYPE = 304;
pub const SDP_ST_UUID32: SDP_SPECIFICTYPE = 544;
pub type SDP_TYPE = i32;
pub const SDP_TYPE_ALTERNATIVE: SDP_TYPE = 7;
pub const SDP_TYPE_BOOLEAN: SDP_TYPE = 5;
pub const SDP_TYPE_CONTAINER: SDP_TYPE = 32;
pub const SDP_TYPE_INT: SDP_TYPE = 2;
pub const SDP_TYPE_NIL: SDP_TYPE = 0;
pub const SDP_TYPE_SEQUENCE: SDP_TYPE = 6;
pub const SDP_TYPE_STRING: SDP_TYPE = 4;
pub const SDP_TYPE_UINT: SDP_TYPE = 1;
pub const SDP_TYPE_URL: SDP_TYPE = 8;
pub const SDP_TYPE_UUID: SDP_TYPE = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SDP_ULARGE_INTEGER_16 {
    pub LowPart: u64,
    pub HighPart: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SdpAttributeRange {
    pub minAttribute: u16,
    pub maxAttribute: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SdpQueryUuid {
    pub u: SdpQueryUuidUnion,
    pub uuidType: u16,
}
impl Default for SdpQueryUuid {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SdpQueryUuidUnion {
    pub uuid128: windows_core::GUID,
    pub uuid32: u32,
    pub uuid16: u16,
}
impl Default for SdpQueryUuidUnion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
