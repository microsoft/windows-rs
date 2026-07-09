pub type DXGI_FEATURE = i32;
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_HDR_METADATA_TYPE = i32;
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = 1;
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = 2;
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = 0;
pub type DXGI_OFFER_RESOURCE_FLAGS = i32;
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = 1;
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = 1;
pub type DXGI_OUTDUPL_FLAG = i32;
pub type DXGI_RECLAIM_RESOURCE_RESULTS = i32;
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS = 1;
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS = 2;
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS = 0;
