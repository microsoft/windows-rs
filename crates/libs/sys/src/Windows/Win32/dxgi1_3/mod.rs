windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory2(flags : u32, riid : *const windows_sys::core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn DXGIGetDebugInterface1(flags : u32, riid : *const windows_sys::core::GUID, pdebug : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
pub type DXGI_FRAME_PRESENTATION_MODE = i32;
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE = 0;
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE = 3;
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE = 2;
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
pub type DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = i32;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 2;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 1;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 4;
pub type DXGI_OVERLAY_SUPPORT_FLAG = i32;
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG = 1;
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG = 2;
