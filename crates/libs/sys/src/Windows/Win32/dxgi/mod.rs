windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory(riid : *const windows_sys::core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory1(riid : *const windows_sys::core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory2(flags : u32, riid : *const windows_sys::core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn DXGIDeclareAdapterRemovalSupport() -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn DXGIDisableVBlankVirtualization() -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn DXGIGetDebugInterface1(flags : u32, riid : *const windows_sys::core::GUID, pdebug : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::LUID,
}
#[cfg(feature = "winnt")]
impl Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::LUID,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
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
    pub AdapterLuid: super::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "winnt")]
impl Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "winnt")]
impl Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ADAPTER_FLAG = i32;
pub type DXGI_ADAPTER_FLAG3 = u32;
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = 4;
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: DXGI_ADAPTER_FLAG3 = 4294967295;
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 = 32;
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = 0;
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = 1;
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = 2;
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = 8;
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = 16;
pub const DXGI_ADAPTER_FLAG_FORCE_DWORD: DXGI_ADAPTER_FLAG = -1;
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = 0;
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = 1;
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = 2;
pub type DXGI_ALPHA_MODE = i32;
pub const DXGI_ALPHA_MODE_FORCE_DWORD: DXGI_ALPHA_MODE = -1;
pub const DXGI_ALPHA_MODE_IGNORE: DXGI_ALPHA_MODE = 3;
pub const DXGI_ALPHA_MODE_PREMULTIPLIED: DXGI_ALPHA_MODE = 1;
pub const DXGI_ALPHA_MODE_STRAIGHT: DXGI_ALPHA_MODE = 2;
pub const DXGI_ALPHA_MODE_UNSPECIFIED: DXGI_ALPHA_MODE = 0;
pub const DXGI_CENTER_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967294;
pub const DXGI_COLOR_SPACE_CUSTOM: DXGI_COLOR_SPACE_TYPE = -1;
pub const DXGI_COLOR_SPACE_RESERVED: DXGI_COLOR_SPACE_TYPE = 4;
pub const DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P2020: DXGI_COLOR_SPACE_TYPE = 25;
pub const DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709: DXGI_COLOR_SPACE_TYPE = 1;
pub const DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020: DXGI_COLOR_SPACE_TYPE = 12;
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020: DXGI_COLOR_SPACE_TYPE = 17;
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: DXGI_COLOR_SPACE_TYPE = 0;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020: DXGI_COLOR_SPACE_TYPE = 14;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020: DXGI_COLOR_SPACE_TYPE = 3;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709: DXGI_COLOR_SPACE_TYPE = 2;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020: DXGI_COLOR_SPACE_TYPE = 21;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709: DXGI_COLOR_SPACE_TYPE = 20;
pub type DXGI_COLOR_SPACE_TYPE = i32;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020: DXGI_COLOR_SPACE_TYPE = 11;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601: DXGI_COLOR_SPACE_TYPE = 7;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709: DXGI_COLOR_SPACE_TYPE = 9;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601: DXGI_COLOR_SPACE_TYPE = 5;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = 19;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020: DXGI_COLOR_SPACE_TYPE = 13;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = 16;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020: DXGI_COLOR_SPACE_TYPE = 10;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601: DXGI_COLOR_SPACE_TYPE = 6;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709: DXGI_COLOR_SPACE_TYPE = 8;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = 15;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020: DXGI_COLOR_SPACE_TYPE = 23;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709: DXGI_COLOR_SPACE_TYPE = 22;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = 24;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020: DXGI_COLOR_SPACE_TYPE = 18;
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 1;
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 0;
pub type DXGI_COMPUTE_PREEMPTION_GRANULARITY = i32;
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 4;
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 3;
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = 2;
pub const DXGI_CPU_ACCESS_DYNAMIC: u32 = 1;
pub const DXGI_CPU_ACCESS_FIELD: u32 = 15;
pub const DXGI_CPU_ACCESS_NONE: u32 = 0;
pub const DXGI_CPU_ACCESS_READ_WRITE: u32 = 2;
pub const DXGI_CPU_ACCESS_SCRATCH: u32 = 3;
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 1;
pub const DXGI_DEBUG_ALL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe48ae283_da80_490b_87e6_43e9a9cfda08);
pub const DXGI_DEBUG_APP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06cd6e01_4219_4ebd_8709_27ed23360c62);
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1;
pub const DXGI_DEBUG_DX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x35cdd7fc_13b2_421d_a5d7_7e4451287d64);
pub const DXGI_DEBUG_DXGI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25cddaa4_b1c6_47e1_ac3e_98875b5a2e2a);
pub type DXGI_DEBUG_ID = windows_sys::core::GUID;
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = 7;
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = 2;
pub type DXGI_DEBUG_RLO_FLAGS = i32;
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = 4;
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [[f32; 2]; 8],
    pub WhitePoints: [[f32; 2]; 16],
}
impl Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 8;
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 1;
pub const DXGI_ENUM_MODES_SCALING: u32 = 2;
pub const DXGI_ENUM_MODES_STEREO: u32 = 4;
pub type DXGI_FEATURE = i32;
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = 0;
pub type DXGI_FORMAT = i32;
pub const DXGI_FORMAT_420_OPAQUE: DXGI_FORMAT = 106;
pub const DXGI_FORMAT_A4B4G4R4_UNORM: DXGI_FORMAT = 191;
pub const DXGI_FORMAT_A8P8: DXGI_FORMAT = 114;
pub const DXGI_FORMAT_A8_UNORM: DXGI_FORMAT = 65;
pub const DXGI_FORMAT_AI44: DXGI_FORMAT = 111;
pub const DXGI_FORMAT_AYUV: DXGI_FORMAT = 100;
pub const DXGI_FORMAT_B4G4R4A4_UNORM: DXGI_FORMAT = 115;
pub const DXGI_FORMAT_B5G5R5A1_UNORM: DXGI_FORMAT = 86;
pub const DXGI_FORMAT_B5G6R5_UNORM: DXGI_FORMAT = 85;
pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: DXGI_FORMAT = 90;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = 87;
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: DXGI_FORMAT = 91;
pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: DXGI_FORMAT = 92;
pub const DXGI_FORMAT_B8G8R8X8_UNORM: DXGI_FORMAT = 88;
pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: DXGI_FORMAT = 93;
pub const DXGI_FORMAT_BC1_TYPELESS: DXGI_FORMAT = 70;
pub const DXGI_FORMAT_BC1_UNORM: DXGI_FORMAT = 71;
pub const DXGI_FORMAT_BC1_UNORM_SRGB: DXGI_FORMAT = 72;
pub const DXGI_FORMAT_BC2_TYPELESS: DXGI_FORMAT = 73;
pub const DXGI_FORMAT_BC2_UNORM: DXGI_FORMAT = 74;
pub const DXGI_FORMAT_BC2_UNORM_SRGB: DXGI_FORMAT = 75;
pub const DXGI_FORMAT_BC3_TYPELESS: DXGI_FORMAT = 76;
pub const DXGI_FORMAT_BC3_UNORM: DXGI_FORMAT = 77;
pub const DXGI_FORMAT_BC3_UNORM_SRGB: DXGI_FORMAT = 78;
pub const DXGI_FORMAT_BC4_SNORM: DXGI_FORMAT = 81;
pub const DXGI_FORMAT_BC4_TYPELESS: DXGI_FORMAT = 79;
pub const DXGI_FORMAT_BC4_UNORM: DXGI_FORMAT = 80;
pub const DXGI_FORMAT_BC5_SNORM: DXGI_FORMAT = 84;
pub const DXGI_FORMAT_BC5_TYPELESS: DXGI_FORMAT = 82;
pub const DXGI_FORMAT_BC5_UNORM: DXGI_FORMAT = 83;
pub const DXGI_FORMAT_BC6H_SF16: DXGI_FORMAT = 96;
pub const DXGI_FORMAT_BC6H_TYPELESS: DXGI_FORMAT = 94;
pub const DXGI_FORMAT_BC6H_UF16: DXGI_FORMAT = 95;
pub const DXGI_FORMAT_BC7_TYPELESS: DXGI_FORMAT = 97;
pub const DXGI_FORMAT_BC7_UNORM: DXGI_FORMAT = 98;
pub const DXGI_FORMAT_BC7_UNORM_SRGB: DXGI_FORMAT = 99;
pub const DXGI_FORMAT_D16_UNORM: DXGI_FORMAT = 55;
pub const DXGI_FORMAT_D24_UNORM_S8_UINT: DXGI_FORMAT = 45;
pub const DXGI_FORMAT_D32_FLOAT: DXGI_FORMAT = 40;
pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: DXGI_FORMAT = 20;
pub const DXGI_FORMAT_DEFINED: u32 = 1;
pub const DXGI_FORMAT_FORCE_UINT: DXGI_FORMAT = -1;
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: DXGI_FORMAT = 69;
pub const DXGI_FORMAT_IA44: DXGI_FORMAT = 112;
pub const DXGI_FORMAT_NV11: DXGI_FORMAT = 110;
pub const DXGI_FORMAT_NV12: DXGI_FORMAT = 103;
pub const DXGI_FORMAT_P010: DXGI_FORMAT = 104;
pub const DXGI_FORMAT_P016: DXGI_FORMAT = 105;
pub const DXGI_FORMAT_P208: DXGI_FORMAT = 130;
pub const DXGI_FORMAT_P8: DXGI_FORMAT = 113;
pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: DXGI_FORMAT = 23;
pub const DXGI_FORMAT_R10G10B10A2_UINT: DXGI_FORMAT = 25;
pub const DXGI_FORMAT_R10G10B10A2_UNORM: DXGI_FORMAT = 24;
pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: DXGI_FORMAT = 89;
pub const DXGI_FORMAT_R11G11B10_FLOAT: DXGI_FORMAT = 26;
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: DXGI_FORMAT = 10;
pub const DXGI_FORMAT_R16G16B16A16_SINT: DXGI_FORMAT = 14;
pub const DXGI_FORMAT_R16G16B16A16_SNORM: DXGI_FORMAT = 13;
pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: DXGI_FORMAT = 9;
pub const DXGI_FORMAT_R16G16B16A16_UINT: DXGI_FORMAT = 12;
pub const DXGI_FORMAT_R16G16B16A16_UNORM: DXGI_FORMAT = 11;
pub const DXGI_FORMAT_R16G16_FLOAT: DXGI_FORMAT = 34;
pub const DXGI_FORMAT_R16G16_SINT: DXGI_FORMAT = 38;
pub const DXGI_FORMAT_R16G16_SNORM: DXGI_FORMAT = 37;
pub const DXGI_FORMAT_R16G16_TYPELESS: DXGI_FORMAT = 33;
pub const DXGI_FORMAT_R16G16_UINT: DXGI_FORMAT = 36;
pub const DXGI_FORMAT_R16G16_UNORM: DXGI_FORMAT = 35;
pub const DXGI_FORMAT_R16_FLOAT: DXGI_FORMAT = 54;
pub const DXGI_FORMAT_R16_SINT: DXGI_FORMAT = 59;
pub const DXGI_FORMAT_R16_SNORM: DXGI_FORMAT = 58;
pub const DXGI_FORMAT_R16_TYPELESS: DXGI_FORMAT = 53;
pub const DXGI_FORMAT_R16_UINT: DXGI_FORMAT = 57;
pub const DXGI_FORMAT_R16_UNORM: DXGI_FORMAT = 56;
pub const DXGI_FORMAT_R1_UNORM: DXGI_FORMAT = 66;
pub const DXGI_FORMAT_R24G8_TYPELESS: DXGI_FORMAT = 44;
pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: DXGI_FORMAT = 46;
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DXGI_FORMAT = 2;
pub const DXGI_FORMAT_R32G32B32A32_SINT: DXGI_FORMAT = 4;
pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: DXGI_FORMAT = 1;
pub const DXGI_FORMAT_R32G32B32A32_UINT: DXGI_FORMAT = 3;
pub const DXGI_FORMAT_R32G32B32_FLOAT: DXGI_FORMAT = 6;
pub const DXGI_FORMAT_R32G32B32_SINT: DXGI_FORMAT = 8;
pub const DXGI_FORMAT_R32G32B32_TYPELESS: DXGI_FORMAT = 5;
pub const DXGI_FORMAT_R32G32B32_UINT: DXGI_FORMAT = 7;
pub const DXGI_FORMAT_R32G32_FLOAT: DXGI_FORMAT = 16;
pub const DXGI_FORMAT_R32G32_SINT: DXGI_FORMAT = 18;
pub const DXGI_FORMAT_R32G32_TYPELESS: DXGI_FORMAT = 15;
pub const DXGI_FORMAT_R32G32_UINT: DXGI_FORMAT = 17;
pub const DXGI_FORMAT_R32G8X24_TYPELESS: DXGI_FORMAT = 19;
pub const DXGI_FORMAT_R32_FLOAT: DXGI_FORMAT = 41;
pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: DXGI_FORMAT = 21;
pub const DXGI_FORMAT_R32_SINT: DXGI_FORMAT = 43;
pub const DXGI_FORMAT_R32_TYPELESS: DXGI_FORMAT = 39;
pub const DXGI_FORMAT_R32_UINT: DXGI_FORMAT = 42;
pub const DXGI_FORMAT_R8G8B8A8_SINT: DXGI_FORMAT = 32;
pub const DXGI_FORMAT_R8G8B8A8_SNORM: DXGI_FORMAT = 31;
pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: DXGI_FORMAT = 27;
pub const DXGI_FORMAT_R8G8B8A8_UINT: DXGI_FORMAT = 30;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DXGI_FORMAT = 28;
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: DXGI_FORMAT = 29;
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: DXGI_FORMAT = 68;
pub const DXGI_FORMAT_R8G8_SINT: DXGI_FORMAT = 52;
pub const DXGI_FORMAT_R8G8_SNORM: DXGI_FORMAT = 51;
pub const DXGI_FORMAT_R8G8_TYPELESS: DXGI_FORMAT = 48;
pub const DXGI_FORMAT_R8G8_UINT: DXGI_FORMAT = 50;
pub const DXGI_FORMAT_R8G8_UNORM: DXGI_FORMAT = 49;
pub const DXGI_FORMAT_R8_SINT: DXGI_FORMAT = 64;
pub const DXGI_FORMAT_R8_SNORM: DXGI_FORMAT = 63;
pub const DXGI_FORMAT_R8_TYPELESS: DXGI_FORMAT = 60;
pub const DXGI_FORMAT_R8_UINT: DXGI_FORMAT = 62;
pub const DXGI_FORMAT_R8_UNORM: DXGI_FORMAT = 61;
pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: DXGI_FORMAT = 67;
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE: DXGI_FORMAT = 189;
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE: DXGI_FORMAT = 190;
pub const DXGI_FORMAT_UNKNOWN: DXGI_FORMAT = 0;
pub const DXGI_FORMAT_V208: DXGI_FORMAT = 131;
pub const DXGI_FORMAT_V408: DXGI_FORMAT = 132;
pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: DXGI_FORMAT = 47;
pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: DXGI_FORMAT = 22;
pub const DXGI_FORMAT_Y210: DXGI_FORMAT = 108;
pub const DXGI_FORMAT_Y216: DXGI_FORMAT = 109;
pub const DXGI_FORMAT_Y410: DXGI_FORMAT = 101;
pub const DXGI_FORMAT_Y416: DXGI_FORMAT = 102;
pub const DXGI_FORMAT_YUY2: DXGI_FORMAT = 107;
pub type DXGI_FRAME_PRESENTATION_MODE = i32;
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE = 0;
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE = 3;
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE = 2;
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
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
#[derive(Clone, Copy)]
pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}
impl Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: windows_sys::core::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: u32,
    pub ControlPointPositions: [f32; 1025],
}
impl Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_GPU_PREFERENCE = i32;
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = 2;
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = 1;
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = 0;
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 0;
pub type DXGI_GRAPHICS_PREEMPTION_GRANULARITY = i32;
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 4;
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 3;
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 1;
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = 2;
pub type DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = u32;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 4;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 1;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 2;
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
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut DXGI_INFO_QUEUE_MESSAGE_ID,
}
impl Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: DXGI_DEBUG_ID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: DXGI_INFO_QUEUE_MESSAGE_ID,
    pub pDescription: *const i8,
    pub DescriptionByteLength: usize,
}
impl Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_INFO_QUEUE_MESSAGE_CATEGORY = i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 3;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 4;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 9;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 2;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 1;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 8;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 10;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 5;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 7;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 6;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 0;
pub type DXGI_INFO_QUEUE_MESSAGE_ID = i32;
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0;
pub type DXGI_INFO_QUEUE_MESSAGE_SEVERITY = i32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 0;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 1;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 3;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 4;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 16],
    pub CodeValues: [u8; 162],
}
impl Default for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 12],
    pub CodeValues: [u8; 12],
}
impl Default for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [u8; 64],
}
impl Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_MAP_DISCARD: u32 = 4;
pub const DXGI_MAP_READ: u32 = 1;
pub const DXGI_MAP_WRITE: u32 = 2;
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
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16;
pub type DXGI_MEMORY_SEGMENT_GROUP = i32;
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = 0;
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Stereo: windows_sys::core::BOOL,
}
pub type DXGI_MODE_ROTATION = i32;
pub const DXGI_MODE_ROTATION_IDENTITY: DXGI_MODE_ROTATION = 1;
pub const DXGI_MODE_ROTATION_ROTATE180: DXGI_MODE_ROTATION = 3;
pub const DXGI_MODE_ROTATION_ROTATE270: DXGI_MODE_ROTATION = 4;
pub const DXGI_MODE_ROTATION_ROTATE90: DXGI_MODE_ROTATION = 2;
pub const DXGI_MODE_ROTATION_UNSPECIFIED: DXGI_MODE_ROTATION = 0;
pub type DXGI_MODE_SCALING = i32;
pub const DXGI_MODE_SCALING_CENTERED: DXGI_MODE_SCALING = 1;
pub const DXGI_MODE_SCALING_STRETCHED: DXGI_MODE_SCALING = 2;
pub const DXGI_MODE_SCALING_UNSPECIFIED: DXGI_MODE_SCALING = 0;
pub type DXGI_MODE_SCANLINE_ORDER = i32;
pub const DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER = 3;
pub const DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE: DXGI_MODE_SCANLINE_ORDER = 1;
pub const DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED: DXGI_MODE_SCANLINE_ORDER = 0;
pub const DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER = 2;
pub type DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = i32;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 2;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 1;
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 4;
pub const DXGI_MWA_NO_ALT_ENTER: u32 = 2;
pub const DXGI_MWA_NO_PRINT_SCREEN: u32 = 4;
pub const DXGI_MWA_NO_WINDOW_CHANGES: u32 = 1;
pub const DXGI_MWA_VALID: u32 = 7;
pub type DXGI_OFFER_RESOURCE_FLAGS = i32;
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = 1;
pub type DXGI_OFFER_RESOURCE_PRIORITY = i32;
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = 3;
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = 1;
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = 2;
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: DXGI_MODE_DESC,
    pub Rotation: DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: windows_sys::core::BOOL,
}
pub type DXGI_OUTDUPL_FLAG = i32;
#[repr(C)]
#[cfg(feature = "windef")]
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
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::POINT,
    pub DestinationRect: super::RECT,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::POINT,
    pub Visible: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::POINT,
}
pub type DXGI_OUTDUPL_POINTER_SHAPE_TYPE = i32;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 2;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 4;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::RECT,
    pub AttachedToDesktop: windows_sys::core::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::HMONITOR,
}
#[cfg(feature = "windef")]
impl Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::RECT,
    pub AttachedToDesktop: windows_sys::core::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(feature = "windef")]
impl Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = i32;
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = 1;
pub type DXGI_OVERLAY_SUPPORT_FLAG = i32;
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG = 1;
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG = 2;
pub const DXGI_PRESENT_ALLOW_TEARING: u32 = 512;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: u32 = 2;
pub const DXGI_PRESENT_DO_NOT_WAIT: u32 = 8;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::RECT,
    pub pScrollRect: *mut super::RECT,
    pub pScrollOffset: *mut super::POINT,
}
#[cfg(feature = "windef")]
impl Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_PRESENT_RESTART: u32 = 4;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32;
pub const DXGI_PRESENT_TEST: u32 = 1;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub type DXGI_RECLAIM_RESOURCE_RESULTS = i32;
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS = 1;
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS = 2;
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS = 0;
pub type DXGI_RESIDENCY = i32;
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = 3;
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = 1;
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = 2;
pub const DXGI_RESOURCE_PRIORITY_HIGH: u32 = 2684354560;
pub const DXGI_RESOURCE_PRIORITY_LOW: u32 = 1342177280;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200;
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
pub type DXGI_RGBA = D3DCOLORVALUE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
pub type DXGI_SCALING = i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2;
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1;
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1;
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}
pub type DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = i32;
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = 2;
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: super::HWND,
    pub Windowed: windows_sys::core::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[cfg(feature = "windef")]
impl Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub Stereo: windows_sys::core::BOOL,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: u32,
}
pub type DXGI_SWAP_CHAIN_FLAG = i32;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = 2;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = 2048;
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = 32;
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = 128;
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = 64;
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = 256;
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = 4;
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = 1024;
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = 1;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = 8;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = 4096;
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = 16;
pub const DXGI_SWAP_CHAIN_FLAG_USE_DEFAULT_COLOR_SPACE: DXGI_SWAP_CHAIN_FLAG = 32768;
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = 512;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: DXGI_RATIONAL,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Windowed: windows_sys::core::BOOL,
}
pub type DXGI_SWAP_EFFECT = i32;
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1;
pub type DXGI_USAGE = u32;
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512;
pub const DXGI_USAGE_READ_ONLY: u32 = 256;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32;
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16;
pub const DXGI_USAGE_SHARED: u32 = 128;
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024;
