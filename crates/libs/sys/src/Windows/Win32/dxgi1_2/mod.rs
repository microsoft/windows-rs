#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ALPHA_MODE = i32;
pub const DXGI_ALPHA_MODE_FORCE_DWORD: DXGI_ALPHA_MODE = -1;
pub const DXGI_ALPHA_MODE_IGNORE: DXGI_ALPHA_MODE = 3;
pub const DXGI_ALPHA_MODE_PREMULTIPLIED: DXGI_ALPHA_MODE = 1;
pub const DXGI_ALPHA_MODE_STRAIGHT: DXGI_ALPHA_MODE = 2;
pub const DXGI_ALPHA_MODE_UNSPECIFIED: DXGI_ALPHA_MODE = 0;
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 1;
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 0;
pub type DXGI_COMPUTE_PREEMPTION_GRANULARITY = i32;
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 4;
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 3;
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 2;
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8;
pub const DXGI_ENUM_MODES_STEREO: u32 = 4;
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 0;
pub type DXGI_GRAPHICS_PREEMPTION_GRANULARITY = i32;
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 4;
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 3;
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 1;
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: super::dxgicommon::DXGI_RATIONAL,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ScanlineOrdering: super::dxgitype::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: super::dxgitype::DXGI_MODE_SCALING,
    pub Stereo: windows_sys::core::BOOL,
}
pub type DXGI_OFFER_RESOURCE_PRIORITY = i32;
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = 3;
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = 1;
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: super::dxgitype::DXGI_MODE_DESC,
    pub Rotation: super::dxgitype::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: windows_sys::core::BOOL,
    pub ProtectedContentMaskedOut: windows_sys::core::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::windef::POINT,
    pub DestinationRect: super::windef::RECT,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::windef::POINT,
    pub Visible: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::windef::POINT,
}
pub type DXGI_OUTDUPL_POINTER_SHAPE_TYPE = i32;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 2;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 4;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 1;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::windef::RECT,
    pub pScrollRect: *mut super::windef::RECT,
    pub pScrollOffset: *mut super::windef::POINT,
}
#[cfg(feature = "Win32_windef")]
impl Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_SCALING = i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2;
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1;
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0;
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub Stereo: windows_sys::core::BOOL,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
    pub BufferUsage: super::dxgi::DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: super::dxgi::DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: super::dxgicommon::DXGI_RATIONAL,
    pub ScanlineOrdering: super::dxgitype::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: super::dxgitype::DXGI_MODE_SCALING,
    pub Windowed: windows_sys::core::BOOL,
}
