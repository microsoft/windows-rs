#[cfg(feature = "guiddef")]
windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory(riid : *const super::IID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory1(riid : *const super::IID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("dxgi.dll" "system" fn CreateDXGIFactory2(flags : super::UINT, riid : *const super::IID, ppfactory : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn DXGIDeclareAdapterRemovalSupport() -> windows_sys::core::HRESULT);
windows_link::link!("dxgi.dll" "system" fn DXGIDisableVBlankVirtualization() -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("dxgi.dll" "system" fn DXGIGetDebugInterface1(flags : super::UINT, riid : *const super::IID, pdebug : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [super::WCHAR; 128],
    pub VendorId: super::UINT,
    pub DeviceId: super::UINT,
    pub SubSysId: super::UINT,
    pub Revision: super::UINT,
    pub DedicatedVideoMemory: super::SIZE_T,
    pub DedicatedSystemMemory: super::SIZE_T,
    pub SharedSystemMemory: super::SIZE_T,
    pub AdapterLuid: super::LUID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [super::WCHAR; 128],
    pub VendorId: super::UINT,
    pub DeviceId: super::UINT,
    pub SubSysId: super::UINT,
    pub Revision: super::UINT,
    pub DedicatedVideoMemory: super::SIZE_T,
    pub DedicatedSystemMemory: super::SIZE_T,
    pub SharedSystemMemory: super::SIZE_T,
    pub AdapterLuid: super::LUID,
    pub Flags: super::UINT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [super::WCHAR; 128],
    pub VendorId: super::UINT,
    pub DeviceId: super::UINT,
    pub SubSysId: super::UINT,
    pub Revision: super::UINT,
    pub DedicatedVideoMemory: super::SIZE_T,
    pub DedicatedSystemMemory: super::SIZE_T,
    pub SharedSystemMemory: super::SIZE_T,
    pub AdapterLuid: super::LUID,
    pub Flags: super::UINT,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [super::WCHAR; 128],
    pub VendorId: super::UINT,
    pub DeviceId: super::UINT,
    pub SubSysId: super::UINT,
    pub Revision: super::UINT,
    pub DedicatedVideoMemory: super::SIZE_T,
    pub DedicatedSystemMemory: super::SIZE_T,
    pub SharedSystemMemory: super::SIZE_T,
    pub AdapterLuid: super::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ADAPTER_FLAG = i32;
pub type DXGI_ADAPTER_FLAG3 = i32;
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = 4;
pub const DXGI_ADAPTER_FLAG3_FORCE_DWORD: DXGI_ADAPTER_FLAG3 = -1;
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
pub const DXGI_CPU_ACCESS_DYNAMIC: i32 = 1;
pub const DXGI_CPU_ACCESS_FIELD: i32 = 15;
pub const DXGI_CPU_ACCESS_NONE: i32 = 0;
pub const DXGI_CPU_ACCESS_READ_WRITE: i32 = 2;
pub const DXGI_CPU_ACCESS_SCRATCH: i32 = 3;
pub const DXGI_CREATE_FACTORY_DEBUG: i32 = 1;
pub const DXGI_DEBUG_ALL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe48ae283_da80_490b_87e6_43e9a9cfda08);
pub const DXGI_DEBUG_APP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06cd6e01_4219_4ebd_8709_27ed23360c62);
pub const DXGI_DEBUG_BINARY_VERSION: i32 = 1;
pub const DXGI_DEBUG_DX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x35cdd7fc_13b2_421d_a5d7_7e4451287d64);
pub const DXGI_DEBUG_DXGI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25cddaa4_b1c6_47e1_ac3e_98875b5a2e2a);
pub type DXGI_DEBUG_ID = windows_sys::core::GUID;
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = 7;
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = 2;
pub type DXGI_DEBUG_RLO_FLAGS = i32;
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = 4;
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: super::UINT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [[super::FLOAT; 2]; 8],
    pub WhitePoints: [[super::FLOAT; 2]; 16],
}
#[cfg(feature = "minwindef")]
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
pub const DXGI_FORMAT_DEFINED: i32 = 1;
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: super::UINT,
    pub PresentRefreshCount: super::UINT,
    pub SyncRefreshCount: super::UINT,
    pub SyncQPCTime: super::LARGE_INTEGER,
    pub SyncGPUTime: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: super::UINT,
    pub PresentRefreshCount: super::UINT,
    pub SyncRefreshCount: super::UINT,
    pub SyncQPCTime: super::LARGE_INTEGER,
    pub SyncGPUTime: super::LARGE_INTEGER,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: super::UINT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: windows_sys::core::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: super::UINT,
    pub ControlPointPositions: [f32; 1025],
}
#[cfg(feature = "shared")]
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
pub type DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = i32;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 4;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 1;
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [super::UINT16; 2],
    pub GreenPrimary: [super::UINT16; 2],
    pub BluePrimary: [super::UINT16; 2],
    pub WhitePoint: [super::UINT16; 2],
    pub MaxMasteringLuminance: super::UINT,
    pub MinMasteringLuminance: super::UINT,
    pub MaxContentLightLevel: super::UINT16,
    pub MaxFrameAverageLightLevel: super::UINT16,
}
#[cfg(feature = "shared")]
impl Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [super::BYTE; 72],
}
#[cfg(feature = "shared")]
impl Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_HDR_METADATA_TYPE = i32;
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = 1;
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = 2;
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = 0;
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: i32 = 1024;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: super::UINT,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: super::UINT,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: super::UINT,
    pub pIDList: *mut DXGI_INFO_QUEUE_MESSAGE_ID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: DXGI_DEBUG_ID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: DXGI_INFO_QUEUE_MESSAGE_ID,
    pub pDescription: *const i8,
    pub DescriptionByteLength: super::SIZE_T,
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
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: i32 = 0;
pub type DXGI_INFO_QUEUE_MESSAGE_SEVERITY = i32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 0;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 1;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 3;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 4;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub CodeCounts: [super::BYTE; 16],
    pub CodeValues: [super::BYTE; 162],
}
#[cfg(feature = "shared")]
impl Default for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub CodeCounts: [super::BYTE; 12],
    pub CodeValues: [super::BYTE; 12],
}
#[cfg(feature = "shared")]
impl Default for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [super::BYTE; 64],
}
#[cfg(feature = "shared")]
impl Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: super::INT,
    pub pBits: *mut super::BYTE,
}
pub const DXGI_MAP_DISCARD: u32 = 4;
pub const DXGI_MAP_READ: u32 = 1;
pub const DXGI_MAP_WRITE: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: super::FLOAT,
    pub _12: super::FLOAT,
    pub _21: super::FLOAT,
    pub _22: super::FLOAT,
    pub _31: super::FLOAT,
    pub _32: super::FLOAT,
}
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: i32 = 16;
pub type DXGI_MEMORY_SEGMENT_GROUP = i32;
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = 0;
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MODE_DESC {
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MODE_DESC1 {
    pub Width: super::UINT,
    pub Height: super::UINT,
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
pub const DXGI_MWA_NO_ALT_ENTER: i32 = 2;
pub const DXGI_MWA_NO_PRINT_SCREEN: i32 = 4;
pub const DXGI_MWA_NO_WINDOW_CHANGES: i32 = 1;
pub const DXGI_MWA_VALID: i32 = 7;
pub type DXGI_OFFER_RESOURCE_FLAGS = i32;
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = 1;
pub type DXGI_OFFER_RESOURCE_PRIORITY = i32;
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = 3;
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = 1;
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = 2;
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: DXGI_MODE_DESC,
    pub Rotation: DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: windows_sys::core::BOOL,
}
pub type DXGI_OUTDUPL_FLAG = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: super::LARGE_INTEGER,
    pub LastMouseUpdateTime: super::LARGE_INTEGER,
    pub AccumulatedFrames: super::UINT,
    pub RectsCoalesced: windows_sys::core::BOOL,
    pub ProtectedContentMaskedOut: windows_sys::core::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: super::UINT,
    pub PointerShapeBufferSize: super::UINT,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::POINT,
    pub DestinationRect: super::RECT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::POINT,
    pub Visible: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: super::UINT,
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Pitch: super::UINT,
    pub HotSpot: super::POINT,
}
pub type DXGI_OUTDUPL_POINTER_SHAPE_TYPE = i32;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 2;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 4;
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [super::WCHAR; 32],
    pub DesktopCoordinates: super::RECT,
    pub AttachedToDesktop: windows_sys::core::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::HMONITOR,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [super::WCHAR; 32],
    pub DesktopCoordinates: super::RECT,
    pub AttachedToDesktop: windows_sys::core::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::HMONITOR,
    pub BitsPerColor: super::UINT,
    pub ColorSpace: DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [super::FLOAT; 2],
    pub GreenPrimary: [super::FLOAT; 2],
    pub BluePrimary: [super::FLOAT; 2],
    pub WhitePoint: [super::FLOAT; 2],
    pub MinLuminance: super::FLOAT,
    pub MaxLuminance: super::FLOAT,
    pub MaxFullFrameLuminance: super::FLOAT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
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
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: super::UINT,
    pub pDirtyRects: *mut super::RECT,
    pub pScrollRect: *mut super::RECT,
    pub pScrollOffset: *mut super::POINT,
}
pub const DXGI_PRESENT_RESTART: u32 = 4;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32;
pub const DXGI_PRESENT_TEST: u32 = 1;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: super::UINT64,
    pub CurrentUsage: super::UINT64,
    pub AvailableForReservation: super::UINT64,
    pub CurrentReservation: super::UINT64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_RATIONAL {
    pub Numerator: super::UINT,
    pub Denominator: super::UINT,
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
pub const DXGI_RESOURCE_PRIORITY_LOW: i32 = 1342177280;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200;
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: i32 = 671088640;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: i32 = 2013265920;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
pub type DXGI_RGBA = D3DCOLORVALUE;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: super::UINT,
    pub Quality: super::UINT,
}
pub type DXGI_SCALING = i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2;
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1;
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::HANDLE,
}
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648;
pub const DXGI_SHARED_RESOURCE_WRITE: i32 = 1;
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SURFACE_DESC {
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}
pub type DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = i32;
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = 2;
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: super::UINT,
    pub OutputWindow: super::HWND,
    pub Windowed: windows_sys::core::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: super::UINT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Format: DXGI_FORMAT,
    pub Stereo: windows_sys::core::BOOL,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: super::UINT,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: super::UINT,
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
#[cfg(feature = "shared")]
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
pub const IID_IDXGIAdapter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
pub const IID_IDXGIAdapter1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x29038f61_3839_4626_91fd_086879011a05);
pub const IID_IDXGIAdapter2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0aa1ae0a_fa0e_4b84_8644_e05ff8e5acb5);
pub const IID_IDXGIAdapter3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x645967a4_1392_4310_a798_8053ce3e93fd);
pub const IID_IDXGIAdapter4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3c8d99d1_4fbf_4181_a82c_af66bf7bd24e);
pub const IID_IDXGIDebug: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x119e7452_de9e_40fe_8806_88f90c12b441);
pub const IID_IDXGIDebug1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc5a05f0c_16f2_4adf_9f4d_a8c4d58ac550);
pub const IID_IDXGIDecodeSwapChain: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2633066b_4514_4c7a_8fd8_12ea98059d18);
pub const IID_IDXGIDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
pub const IID_IDXGIDevice1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x77db970f_6276_48ba_ba28_070143b4392c);
pub const IID_IDXGIDevice2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05008617_fbfd_4051_a790_144884b4f6a9);
pub const IID_IDXGIDevice3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6007896c_3244_4afd_bf18_a6d3beda5023);
pub const IID_IDXGIDevice4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x95b4f95f_d8da_4ca4_9ee6_3b76d5968a10);
pub const IID_IDXGIDeviceSubObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
pub const IID_IDXGIDisplayControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea9dbf1a_c88e_4486_854a_98aa0138f30c);
pub const IID_IDXGIFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
pub const IID_IDXGIFactory1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x770aae78_f26f_4dba_a829_253c83d1b387);
pub const IID_IDXGIFactory2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x50c83a1c_e072_4c48_87b0_3630fa36a6d0);
pub const IID_IDXGIFactory3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25483823_cd46_4c7d_86ca_47aa95b837bd);
pub const IID_IDXGIFactory4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1bc6ea02_ef36_464f_bf0c_21ca39e5168a);
pub const IID_IDXGIFactory5: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7632e1f5_ee65_4dca_87fd_84cd75f8838d);
pub const IID_IDXGIFactory6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc1b6694f_ff09_44a9_b03c_77900a0a1d17);
pub const IID_IDXGIFactory7: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4966eed_76db_44da_84c1_ee9a7afb20a8);
pub const IID_IDXGIFactoryMedia: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41e7d1f2_a591_4f7b_a2e5_fa9c843e1c12);
pub const IID_IDXGIInfoQueue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd67441c7_672a_476f_9e82_cd55b44949ce);
pub const IID_IDXGIKeyedMutex: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9d8e1289_d7b3_465f_8126_250e349af85d);
pub const IID_IDXGIObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
pub const IID_IDXGIOutput: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xae02eedb_c735_4690_8d52_5a8dc20213aa);
pub const IID_IDXGIOutput1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00cddea8_939b_4b83_a340_a685226666cc);
pub const IID_IDXGIOutput2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x595e39d1_2724_4663_99b1_da969de28364);
pub const IID_IDXGIOutput3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8a6bb301_7e7e_41f4_a8e0_5b32f7f99b18);
pub const IID_IDXGIOutput4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc7dca35_2196_414d_9f53_617884032a60);
pub const IID_IDXGIOutput5: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x80a07424_ab52_42eb_833c_0c42fd282d98);
pub const IID_IDXGIOutput6: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x068346e8_aaec_4b84_add7_137f513f77a1);
pub const IID_IDXGIOutputDuplication: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x191cfac3_a341_470d_b26e_a864f428319c);
pub const IID_IDXGIResource: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x035f3ab4_482e_4e50_b41f_8a7f8bd8960b);
pub const IID_IDXGIResource1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x30961379_4609_4a41_998e_54fe567ee0c1);
pub const IID_IDXGISurface: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
pub const IID_IDXGISurface1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4ae63092_6327_4c1b_80ae_bfe12ea32b86);
pub const IID_IDXGISurface2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaba496dd_b617_4cb8_a866_bc44d7eb1fa2);
pub const IID_IDXGISwapChain: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
pub const IID_IDXGISwapChain1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x790a45f7_0d42_4876_983a_0a55cfe6f4aa);
pub const IID_IDXGISwapChain2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa8be2ac4_199f_4946_b331_79599fb98de7);
pub const IID_IDXGISwapChain3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x94d99bdb_f1f8_4ab0_b236_7da0170edab1);
pub const IID_IDXGISwapChain4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3d585d5a_bd4a_489e_b1f4_3dbcb6452ffb);
pub const IID_IDXGISwapChainMedia: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdd95b90b_f05f_4f6a_bd65_25bfb264bd84);
pub const _FACDXGI: i32 = 2170;
