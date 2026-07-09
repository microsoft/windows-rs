#[inline]
pub unsafe fn CreateDXGIFactory<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory(riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateDXGIFactory1<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory1(riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory1(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateDXGIFactory2<T>(flags: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory2(flags : u32, riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory2(flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn DXGIDeclareAdapterRemovalSupport() -> windows_core::HRESULT {
    windows_core::link!("dxgi.dll" "system" fn DXGIDeclareAdapterRemovalSupport() -> windows_core::HRESULT);
    unsafe { DXGIDeclareAdapterRemovalSupport() }
}
#[inline]
pub unsafe fn DXGIDisableVBlankVirtualization() -> windows_core::HRESULT {
    windows_core::link!("dxgi.dll" "system" fn DXGIDisableVBlankVirtualization() -> windows_core::HRESULT);
    unsafe { DXGIDisableVBlankVirtualization() }
}
#[inline]
pub unsafe fn DXGIGetDebugInterface1<T>(flags: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn DXGIGetDebugInterface1(flags : u32, riid : *const windows_core::GUID, pdebug : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DXGIGetDebugInterface1(flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC1 {
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
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::winnt::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
#[cfg(feature = "Win32_winnt")]
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
pub const DXGI_DEBUG_ALL: windows_core::GUID = windows_core::GUID::from_u128(0xe48ae283_da80_490b_87e6_43e9a9cfda08);
pub const DXGI_DEBUG_APP: windows_core::GUID = windows_core::GUID::from_u128(0x06cd6e01_4219_4ebd_8709_27ed23360c62);
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1;
pub const DXGI_DEBUG_DX: windows_core::GUID = windows_core::GUID::from_u128(0x35cdd7fc_13b2_421d_a5d7_7e4451287d64);
pub const DXGI_DEBUG_DXGI: windows_core::GUID = windows_core::GUID::from_u128(0x25cddaa4_b1c6_47e1_ac3e_98875b5a2e2a);
pub type DXGI_DEBUG_ID = windows_core::GUID;
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = 7;
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = 2;
pub type DXGI_DEBUG_RLO_FLAGS = i32;
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = 4;
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DXGI_INFO_QUEUE_MESSAGE_ID(pub i32);
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0;
pub type DXGI_INFO_QUEUE_MESSAGE_SEVERITY = i32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 0;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 1;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 3;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 4;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [u8; 64],
}
impl Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Stereo: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: DXGI_MODE_DESC,
    pub Rotation: DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: windows_core::BOOL,
}
pub type DXGI_OUTDUPL_FLAG = i32;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: windows_core::BOOL,
    pub ProtectedContentMaskedOut: windows_core::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::windef::POINT,
    pub DestinationRect: super::windef::RECT,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::windef::POINT,
    pub Visible: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::windef::RECT,
    pub AttachedToDesktop: windows_core::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::windef::HMONITOR,
}
#[cfg(feature = "Win32_windef")]
impl Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::windef::RECT,
    pub AttachedToDesktop: windows_core::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: super::windef::HMONITOR,
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
#[cfg(feature = "Win32_windef")]
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
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const DXGI_PRESENT_RESTART: u32 = 4;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: u32 = 64;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: u32 = 16;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: u32 = 32;
pub const DXGI_PRESENT_TEST: u32 = 1;
pub const DXGI_PRESENT_USE_DURATION: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
pub type DXGI_RGBA = D3DCOLORVALUE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
pub type DXGI_SCALING = i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2;
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1;
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::winnt::HANDLE,
}
pub const DXGI_SHARED_RESOURCE_READ: u32 = 2147483648;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 1;
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: super::windef::HWND,
    pub Windowed: windows_core::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub Stereo: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: DXGI_RATIONAL,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Windowed: windows_core::BOOL,
}
pub type DXGI_SWAP_EFFECT = i32;
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DXGI_USAGE(pub u32);
pub const DXGI_USAGE_BACK_BUFFER: u32 = 64;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 512;
pub const DXGI_USAGE_READ_ONLY: u32 = 256;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 32;
pub const DXGI_USAGE_SHADER_INPUT: u32 = 16;
pub const DXGI_USAGE_SHARED: u32 = 128;
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 1024;
windows_core::imp::define_interface!(IDXGIAdapter, IDXGIAdapter_Vtbl, 0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
impl core::ops::Deref for IDXGIAdapter {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter, windows_core::IUnknown, IDXGIObject);
impl IDXGIAdapter {
    pub unsafe fn EnumOutputs(&self, output: u32, ppoutput: *mut Option<IDXGIOutput>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumOutputs)(windows_core::Interface::as_raw(self), output, core::mem::transmute(ppoutput)) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckInterfaceSupport)(windows_core::Interface::as_raw(self), interfacename, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumOutputs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDesc: usize,
    pub CheckInterfaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut i64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIAdapter_Impl: IDXGIObject_Impl {
    fn EnumOutputs(&self, output: u32, ppoutput: windows_core::OutRef<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::Result<()>;
    fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: IDXGIAdapter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumOutputs<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: u32, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter_Impl::EnumOutputs(this, core::mem::transmute_copy(&output), core::mem::transmute_copy(&ppoutput)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacename: *const windows_core::GUID, pumdversion: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::CheckInterfaceSupport(this, core::mem::transmute_copy(&interfacename)) {
                    Ok(ok__) => {
                        pumdversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIAdapter {}
windows_core::imp::define_interface!(IDXGIAdapter1, IDXGIAdapter1_Vtbl, 0x29038f61_3839_4626_91fd_086879011a05);
impl core::ops::Deref for IDXGIAdapter1 {
    type Target = IDXGIAdapter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter1, windows_core::IUnknown, IDXGIObject, IDXGIAdapter);
impl IDXGIAdapter1 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter1_Vtbl {
    pub base__: IDXGIAdapter_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDesc1: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIAdapter1_Impl: IDXGIAdapter_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIAdapter1_Vtbl {
    pub const fn new<Identity: IDXGIAdapter1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGIAdapter1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self { base__: IDXGIAdapter_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIAdapter1 {}
windows_core::imp::define_interface!(IDXGIAdapter2, IDXGIAdapter2_Vtbl, 0x0aa1ae0a_fa0e_4b84_8644_e05ff8e5acb5);
impl core::ops::Deref for IDXGIAdapter2 {
    type Target = IDXGIAdapter1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter2, windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1);
impl IDXGIAdapter2 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc2)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter2_Vtbl {
    pub base__: IDXGIAdapter1_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetDesc2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDesc2: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIAdapter2_Impl: IDXGIAdapter1_Impl {
    fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIAdapter2_Vtbl {
    pub const fn new<Identity: IDXGIAdapter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc2<Identity: IDXGIAdapter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter2_Impl::GetDesc2(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self { base__: IDXGIAdapter1_Vtbl::new::<Identity, OFFSET>(), GetDesc2: GetDesc2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIAdapter2 {}
windows_core::imp::define_interface!(IDXGIAdapter3, IDXGIAdapter3_Vtbl, 0x645967a4_1392_4310_a798_8053ce3e93fd);
impl core::ops::Deref for IDXGIAdapter3 {
    type Target = IDXGIAdapter2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter3, windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter2);
impl IDXGIAdapter3 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterHardwareContentProtectionTeardownStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterHardwareContentProtectionTeardownStatus)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryVideoMemoryInfo)(windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, pvideomemoryinfo as _) }
    }
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVideoMemoryReservation)(windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, reservation) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterVideoMemoryBudgetChangeNotificationEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterVideoMemoryBudgetChangeNotification)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter3_Vtbl {
    pub base__: IDXGIAdapter2_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterHardwareContentProtectionTeardownStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterHardwareContentProtectionTeardownStatusEvent: usize,
    pub UnregisterHardwareContentProtectionTeardownStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub QueryVideoMemoryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_MEMORY_SEGMENT_GROUP, *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT,
    pub SetVideoMemoryReservation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_MEMORY_SEGMENT_GROUP, u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterVideoMemoryBudgetChangeNotificationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterVideoMemoryBudgetChangeNotificationEvent: usize,
    pub UnregisterVideoMemoryBudgetChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIAdapter3_Impl: IDXGIAdapter2_Impl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32);
    fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::Result<()>;
    fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::Result<()>;
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32);
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIAdapter3_Vtbl {
    pub const fn new<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter3_Impl::RegisterHardwareContentProtectionTeardownStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::UnregisterHardwareContentProtectionTeardownStatus(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::QueryVideoMemoryInfo(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&pvideomemoryinfo)).into()
            }
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::SetVideoMemoryReservation(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&reservation)).into()
            }
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter3_Impl::RegisterVideoMemoryBudgetChangeNotificationEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::UnregisterVideoMemoryBudgetChangeNotification(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        Self {
            base__: IDXGIAdapter2_Vtbl::new::<Identity, OFFSET>(),
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Identity, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Identity, OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Identity, OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Identity, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Identity, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIAdapter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIAdapter3 {}
windows_core::imp::define_interface!(IDXGIAdapter4, IDXGIAdapter4_Vtbl, 0x3c8d99d1_4fbf_4181_a82c_af66bf7bd24e);
impl core::ops::Deref for IDXGIAdapter4 {
    type Target = IDXGIAdapter3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter4, windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter2, IDXGIAdapter3);
impl IDXGIAdapter4 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc3)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter4_Vtbl {
    pub base__: IDXGIAdapter3_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetDesc3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetDesc3: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIAdapter4_Impl: IDXGIAdapter3_Impl {
    fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIAdapter4_Vtbl {
    pub const fn new<Identity: IDXGIAdapter4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc3<Identity: IDXGIAdapter4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter4_Impl::GetDesc3(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self { base__: IDXGIAdapter3_Vtbl::new::<Identity, OFFSET>(), GetDesc3: GetDesc3::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIAdapter2 as windows_core::Interface>::IID || iid == &<IDXGIAdapter3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIAdapter4 {}
windows_core::imp::define_interface!(IDXGIDebug, IDXGIDebug_Vtbl, 0x119e7452_de9e_40fe_8806_88f90c12b441);
windows_core::imp::interface_hierarchy!(IDXGIDebug, windows_core::IUnknown);
impl IDXGIDebug {
    pub unsafe fn ReportLiveObjects(&self, apiid: windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReportLiveObjects)(windows_core::Interface::as_raw(self), core::mem::transmute(apiid), flags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportLiveObjects: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT,
}
pub trait IDXGIDebug_Impl: windows_core::IUnknownImpl {
    fn ReportLiveObjects(&self, apiid: &windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::Result<()>;
}
impl IDXGIDebug_Vtbl {
    pub const fn new<Identity: IDXGIDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportLiveObjects<Identity: IDXGIDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apiid: windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug_Impl::ReportLiveObjects(this, core::mem::transmute(&apiid), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportLiveObjects: ReportLiveObjects::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDebug {}
windows_core::imp::define_interface!(IDXGIDebug1, IDXGIDebug1_Vtbl, 0xc5a05f0c_16f2_4adf_9f4d_a8c4d58ac550);
impl core::ops::Deref for IDXGIDebug1 {
    type Target = IDXGIDebug;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDebug1, windows_core::IUnknown, IDXGIDebug);
impl IDXGIDebug1 {
    pub unsafe fn EnableLeakTrackingForThread(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).EnableLeakTrackingForThread)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn DisableLeakTrackingForThread(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).DisableLeakTrackingForThread)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsLeakTrackingEnabledForThread)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug1_Vtbl {
    pub base__: IDXGIDebug_Vtbl,
    pub EnableLeakTrackingForThread: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub DisableLeakTrackingForThread: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub IsLeakTrackingEnabledForThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait IDXGIDebug1_Impl: IDXGIDebug_Impl {
    fn EnableLeakTrackingForThread(&self);
    fn DisableLeakTrackingForThread(&self);
    fn IsLeakTrackingEnabledForThread(&self) -> windows_core::BOOL;
}
impl IDXGIDebug1_Vtbl {
    pub const fn new<Identity: IDXGIDebug1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableLeakTrackingForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::EnableLeakTrackingForThread(this);
            }
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::DisableLeakTrackingForThread(this);
            }
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::IsLeakTrackingEnabledForThread(this)
            }
        }
        Self {
            base__: IDXGIDebug_Vtbl::new::<Identity, OFFSET>(),
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Identity, OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Identity, OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug1 as windows_core::Interface>::IID || iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDebug1 {}
windows_core::imp::define_interface!(IDXGIDecodeSwapChain, IDXGIDecodeSwapChain_Vtbl, 0x2633066b_4514_4c7a_8fd8_12ea98059d18);
windows_core::imp::interface_hierarchy!(IDXGIDecodeSwapChain, windows_core::IUnknown);
impl IDXGIDecodeSwapChain {
    pub unsafe fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PresentBuffer)(windows_core::Interface::as_raw(self), buffertopresent, syncinterval, flags) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetSourceRect(&self, prect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceRect)(windows_core::Interface::as_raw(self), prect) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetTargetRect(&self, prect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTargetRect)(windows_core::Interface::as_raw(self), prect) }
    }
    pub unsafe fn SetDestSize(&self, width: u32, height: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDestSize)(windows_core::Interface::as_raw(self), width, height) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetSourceRect(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetTargetRect(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDestSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _) }
    }
    pub unsafe fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorSpace)(windows_core::Interface::as_raw(self), colorspace) }
    }
    pub unsafe fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
        unsafe { (windows_core::Interface::vtable(self).GetColorSpace)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDecodeSwapChain_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PresentBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub SetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetSourceRect: usize,
    #[cfg(feature = "Win32_windef")]
    pub SetTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetTargetRect: usize,
    pub SetDestSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetSourceRect: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetTargetRect: usize,
    pub GetDestSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT,
    pub GetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIDecodeSwapChain_Impl: windows_core::IUnknownImpl {
    fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> windows_core::Result<()>;
    fn SetSourceRect(&self, prect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn SetTargetRect(&self, prect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn SetDestSize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn GetSourceRect(&self) -> windows_core::Result<super::windef::RECT>;
    fn GetTargetRect(&self) -> windows_core::Result<super::windef::RECT>;
    fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()>;
    fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::Result<()>;
    fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIDecodeSwapChain_Vtbl {
    pub const fn new<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PresentBuffer<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::PresentBuffer(this, core::mem::transmute_copy(&buffertopresent), core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn SetSourceRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetSourceRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn SetTargetRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetTargetRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn SetDestSize<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetDestSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetSourceRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDecodeSwapChain_Impl::GetSourceRect(this) {
                    Ok(ok__) => {
                        prect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDecodeSwapChain_Impl::GetTargetRect(this) {
                    Ok(ok__) => {
                        prect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDestSize<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::GetDestSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn SetColorSpace<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
            }
        }
        unsafe extern "system" fn GetColorSpace<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::GetColorSpace(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PresentBuffer: PresentBuffer::<Identity, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, OFFSET>,
            SetTargetRect: SetTargetRect::<Identity, OFFSET>,
            SetDestSize: SetDestSize::<Identity, OFFSET>,
            GetSourceRect: GetSourceRect::<Identity, OFFSET>,
            GetTargetRect: GetTargetRect::<Identity, OFFSET>,
            GetDestSize: GetDestSize::<Identity, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, OFFSET>,
            GetColorSpace: GetColorSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDecodeSwapChain as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIDecodeSwapChain {}
windows_core::imp::define_interface!(IDXGIDevice, IDXGIDevice_Vtbl, 0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
impl core::ops::Deref for IDXGIDevice {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice, windows_core::IUnknown, IDXGIObject);
impl IDXGIDevice {
    pub unsafe fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [Option<IDXGISurface>]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), pdesc, ppsurface.len().try_into().unwrap(), usage, psharedresource.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppsurface.as_ptr())) }
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const Option<windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryResourceResidency)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources), presidencystatus as _, numresources) }
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGPUThreadPriority)(windows_core::Interface::as_raw(self), priority) }
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPUThreadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_SURFACE_DESC, u32, DXGI_USAGE, *const DXGI_SHARED_RESOURCE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateSurface: usize,
    pub QueryResourceResidency: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, *mut DXGI_RESIDENCY, u32) -> windows_core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIDevice_Impl: IDXGIObject_Impl {
    fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter>;
    fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut Option<IDXGISurface>) -> windows_core::Result<()>;
    fn QueryResourceResidency(&self, ppresources: *const Option<windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIDevice_Vtbl {
    pub const fn new<Identity: IDXGIDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdapter<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetAdapter(this) {
                    Ok(ok__) => {
                        padapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::CreateSurface(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&numsurfaces), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&psharedresource), core::mem::transmute_copy(&ppsurface)).into()
            }
        }
        unsafe extern "system" fn QueryResourceResidency<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::QueryResourceResidency(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presidencystatus), core::mem::transmute_copy(&numresources)).into()
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetGPUThreadPriority(this) {
                    Ok(ok__) => {
                        ppriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIDevice {}
windows_core::imp::define_interface!(IDXGIDevice1, IDXGIDevice1_Vtbl, 0x77db970f_6276_48ba_ba28_070143b4392c);
impl core::ops::Deref for IDXGIDevice1 {
    type Target = IDXGIDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice1, windows_core::IUnknown, IDXGIObject, IDXGIDevice);
impl IDXGIDevice1 {
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency) }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_Vtbl {
    pub base__: IDXGIDevice_Vtbl,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIDevice1_Impl: IDXGIDevice_Impl {
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIDevice1_Vtbl {
    pub const fn new<Identity: IDXGIDevice1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice1_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice1_Impl::GetMaximumFrameLatency(this) {
                    Ok(ok__) => {
                        pmaxlatency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDevice_Vtbl::new::<Identity, OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIDevice1 {}
windows_core::imp::define_interface!(IDXGIDevice2, IDXGIDevice2_Vtbl, 0x05008617_fbfd_4051_a790_144884b4f6a9);
impl core::ops::Deref for IDXGIDevice2 {
    type Target = IDXGIDevice1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice2, windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1);
impl IDXGIDevice2 {
    pub unsafe fn OfferResources(&self, ppresources: &[Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OfferResources)(windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), core::mem::transmute(ppresources.as_ptr()), priority) }
    }
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, pdiscarded: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReclaimResources)(windows_core::Interface::as_raw(self), numresources, core::mem::transmute(ppresources), pdiscarded.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn EnqueueSetEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnqueueSetEvent)(windows_core::Interface::as_raw(self), hevent) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice2_Vtbl {
    pub base__: IDXGIDevice1_Vtbl,
    pub OfferResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT,
    pub ReclaimResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub EnqueueSetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    EnqueueSetEvent: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIDevice2_Impl: IDXGIDevice1_Impl {
    fn OfferResources(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::Result<()>;
    fn ReclaimResources(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, pdiscarded: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn EnqueueSetEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIDevice2_Vtbl {
    pub const fn new<Identity: IDXGIDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OfferResources<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::OfferResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn ReclaimResources<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, pdiscarded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::ReclaimResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&pdiscarded)).into()
            }
        }
        unsafe extern "system" fn EnqueueSetEvent<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::EnqueueSetEvent(this, core::mem::transmute_copy(&hevent)).into()
            }
        }
        Self {
            base__: IDXGIDevice1_Vtbl::new::<Identity, OFFSET>(),
            OfferResources: OfferResources::<Identity, OFFSET>,
            ReclaimResources: ReclaimResources::<Identity, OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIDevice2 {}
windows_core::imp::define_interface!(IDXGIDevice3, IDXGIDevice3_Vtbl, 0x6007896c_3244_4afd_bf18_a6d3beda5023);
impl core::ops::Deref for IDXGIDevice3 {
    type Target = IDXGIDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice3, windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1, IDXGIDevice2);
impl IDXGIDevice3 {
    pub unsafe fn Trim(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Trim)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice3_Vtbl {
    pub base__: IDXGIDevice2_Vtbl,
    pub Trim: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIDevice3_Impl: IDXGIDevice2_Impl {
    fn Trim(&self);
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIDevice3_Vtbl {
    pub const fn new<Identity: IDXGIDevice3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Trim<Identity: IDXGIDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice3_Impl::Trim(this);
            }
        }
        Self { base__: IDXGIDevice2_Vtbl::new::<Identity, OFFSET>(), Trim: Trim::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIDevice3 {}
windows_core::imp::define_interface!(IDXGIDevice4, IDXGIDevice4_Vtbl, 0x95b4f95f_d8da_4ca4_9ee6_3b76d5968a10);
impl core::ops::Deref for IDXGIDevice4 {
    type Target = IDXGIDevice3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice4, windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1, IDXGIDevice2, IDXGIDevice3);
impl IDXGIDevice4 {
    pub unsafe fn OfferResources1(&self, ppresources: &[Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OfferResources1)(windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), core::mem::transmute(ppresources.as_ptr()), priority, flags) }
    }
    pub unsafe fn ReclaimResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReclaimResources1)(windows_core::Interface::as_raw(self), numresources, core::mem::transmute(ppresources), presults as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice4_Vtbl {
    pub base__: IDXGIDevice3_Vtbl,
    pub OfferResources1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, DXGI_OFFER_RESOURCE_PRIORITY, u32) -> windows_core::HRESULT,
    pub ReclaimResources1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIDevice4_Impl: IDXGIDevice3_Impl {
    fn OfferResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::Result<()>;
    fn ReclaimResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIDevice4_Vtbl {
    pub const fn new<Identity: IDXGIDevice4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OfferResources1<Identity: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice4_Impl::OfferResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn ReclaimResources1<Identity: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice4_Impl::ReclaimResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presults)).into()
            }
        }
        Self {
            base__: IDXGIDevice3_Vtbl::new::<Identity, OFFSET>(),
            OfferResources1: OfferResources1::<Identity, OFFSET>,
            ReclaimResources1: ReclaimResources1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIDevice2 as windows_core::Interface>::IID || iid == &<IDXGIDevice3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIDevice4 {}
windows_core::imp::define_interface!(IDXGIDeviceSubObject, IDXGIDeviceSubObject_Vtbl, 0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
impl core::ops::Deref for IDXGIDeviceSubObject {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDeviceSubObject, windows_core::IUnknown, IDXGIObject);
impl IDXGIDeviceSubObject {
    pub unsafe fn GetDevice<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDXGIDeviceSubObject_Impl: IDXGIObject_Impl {
    fn GetDevice(&self, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXGIDeviceSubObject_Vtbl {
    pub const fn new<Identity: IDXGIDeviceSubObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDXGIDeviceSubObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDeviceSubObject_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        Self { base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(), GetDevice: GetDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDeviceSubObject {}
windows_core::imp::define_interface!(IDXGIDisplayControl, IDXGIDisplayControl_Vtbl, 0xea9dbf1a_c88e_4486_854a_98aa0138f30c);
windows_core::imp::interface_hierarchy!(IDXGIDisplayControl, windows_core::IUnknown);
impl IDXGIDisplayControl {
    pub unsafe fn IsStereoEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsStereoEnabled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetStereoEnabled(&self, enabled: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).SetStereoEnabled)(windows_core::Interface::as_raw(self), enabled.into());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDisplayControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
}
pub trait IDXGIDisplayControl_Impl: windows_core::IUnknownImpl {
    fn IsStereoEnabled(&self) -> windows_core::BOOL;
    fn SetStereoEnabled(&self, enabled: windows_core::BOOL);
}
impl IDXGIDisplayControl_Vtbl {
    pub const fn new<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsStereoEnabled<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDisplayControl_Impl::IsStereoEnabled(this)
            }
        }
        unsafe extern "system" fn SetStereoEnabled<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDisplayControl_Impl::SetStereoEnabled(this, core::mem::transmute_copy(&enabled));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStereoEnabled: IsStereoEnabled::<Identity, OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDisplayControl {}
windows_core::imp::define_interface!(IDXGIFactory, IDXGIFactory_Vtbl, 0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
impl core::ops::Deref for IDXGIFactory {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory, windows_core::IUnknown, IDXGIObject);
impl IDXGIFactory {
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapters)(windows_core::Interface::as_raw(self), adapter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn MakeWindowAssociation(&self, windowhandle: super::windef::HWND, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MakeWindowAssociation)(windows_core::Interface::as_raw(self), windowhandle, flags) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetWindowAssociation(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindowAssociation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> windows_core::Result<IDXGISwapChain>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChain)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pdesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn CreateSoftwareAdapter(&self, module: super::minwindef::HMODULE) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSoftwareAdapter)(windows_core::Interface::as_raw(self), module, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub MakeWindowAssociation: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    MakeWindowAssociation: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetWindowAssociation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetWindowAssociation: usize,
    #[cfg(feature = "Win32_windef")]
    pub CreateSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    CreateSwapChain: usize,
    #[cfg(feature = "Win32_minwindef")]
    pub CreateSoftwareAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::HMODULE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    CreateSoftwareAdapter: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDXGIFactory_Impl: IDXGIObject_Impl {
    fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(&self, windowhandle: super::windef::HWND, flags: u32) -> windows_core::Result<()>;
    fn GetWindowAssociation(&self) -> windows_core::Result<super::windef::HWND>;
    fn CreateSwapChain(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> windows_core::Result<IDXGISwapChain>;
    fn CreateSoftwareAdapter(&self, module: super::minwindef::HMODULE) -> windows_core::Result<IDXGIAdapter>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDXGIFactory_Vtbl {
    pub const fn new<Identity: IDXGIFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapters<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::EnumAdapters(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::windef::HWND, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory_Impl::MakeWindowAssociation(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetWindowAssociation<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindowhandle: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::GetWindowAssociation(this) {
                    Ok(ok__) => {
                        pwindowhandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSwapChain<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::CreateSwapChain(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pdesc)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: super::minwindef::HMODULE, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::CreateSoftwareAdapter(this, core::mem::transmute_copy(&module)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapters: EnumAdapters::<Identity, OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Identity, OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Identity, OFFSET>,
            CreateSwapChain: CreateSwapChain::<Identity, OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIFactory {}
windows_core::imp::define_interface!(IDXGIFactory1, IDXGIFactory1_Vtbl, 0x770aae78_f26f_4dba_a829_253c83d1b387);
impl core::ops::Deref for IDXGIFactory1 {
    type Target = IDXGIFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory1, windows_core::IUnknown, IDXGIObject, IDXGIFactory);
impl IDXGIFactory1 {
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapters1)(windows_core::Interface::as_raw(self), adapter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsCurrent(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsCurrent)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_Vtbl {
    pub base__: IDXGIFactory_Vtbl,
    pub EnumAdapters1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCurrent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDXGIFactory1_Impl: IDXGIFactory_Impl {
    fn EnumAdapters1(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter1>;
    fn IsCurrent(&self) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDXGIFactory1_Vtbl {
    pub const fn new<Identity: IDXGIFactory1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapters1<Identity: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory1_Impl::EnumAdapters1(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCurrent<Identity: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory1_Impl::IsCurrent(this)
            }
        }
        Self { base__: IDXGIFactory_Vtbl::new::<Identity, OFFSET>(), EnumAdapters1: EnumAdapters1::<Identity, OFFSET>, IsCurrent: IsCurrent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIFactory1 {}
windows_core::imp::define_interface!(IDXGIFactory2, IDXGIFactory2_Vtbl, 0x50c83a1c_e072_4c48_87b0_3630fa36a6d0);
impl core::ops::Deref for IDXGIFactory2 {
    type Target = IDXGIFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory2, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1);
impl IDXGIFactory2 {
    pub unsafe fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsWindowedStereoEnabled)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn CreateSwapChainForHwnd<P0, P4>(&self, pdevice: P0, hwnd: super::windef::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P4) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForHwnd)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hwnd, pdesc, pfullscreendesc.unwrap_or(core::mem::zeroed()) as _, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P3>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P3) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForCoreWindow)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pwindow.param().abi(), pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetSharedResourceAdapterLuid(&self, hresource: super::winnt::HANDLE) -> windows_core::Result<super::winnt::LUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharedResourceAdapterLuid)(windows_core::Interface::as_raw(self), hresource, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RegisterStereoStatusWindow(&self, windowhandle: super::windef::HWND, wmsg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusWindow)(windows_core::Interface::as_raw(self), windowhandle, wmsg, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterStereoStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterStereoStatus)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RegisterOcclusionStatusWindow(&self, windowhandle: super::windef::HWND, wmsg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusWindow)(windows_core::Interface::as_raw(self), windowhandle, wmsg, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterOcclusionStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterOcclusionStatus)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
    pub unsafe fn CreateSwapChainForComposition<P0, P2>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForComposition)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_Vtbl {
    pub base__: IDXGIFactory1_Vtbl,
    pub IsWindowedStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(feature = "Win32_windef")]
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::windef::HWND, *const DXGI_SWAP_CHAIN_DESC1, *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    CreateSwapChainForHwnd: usize,
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut super::winnt::LUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetSharedResourceAdapterLuid: usize,
    #[cfg(feature = "Win32_windef")]
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    RegisterStereoStatusWindow: usize,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterStereoStatusEvent: usize,
    pub UnregisterStereoStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_windef")]
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    RegisterOcclusionStatusWindow: usize,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterOcclusionStatusEvent: usize,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub CreateSwapChainForComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory2_Impl: IDXGIFactory1_Impl {
    fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL;
    fn CreateSwapChainForHwnd(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hwnd: super::windef::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn CreateSwapChainForCoreWindow(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pwindow: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn GetSharedResourceAdapterLuid(&self, hresource: super::winnt::HANDLE) -> windows_core::Result<super::winnt::LUID>;
    fn RegisterStereoStatusWindow(&self, windowhandle: super::windef::HWND, wmsg: u32) -> windows_core::Result<u32>;
    fn RegisterStereoStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterStereoStatus(&self, dwcookie: u32);
    fn RegisterOcclusionStatusWindow(&self, windowhandle: super::windef::HWND, wmsg: u32) -> windows_core::Result<u32>;
    fn RegisterOcclusionStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterOcclusionStatus(&self, dwcookie: u32);
    fn CreateSwapChainForComposition(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory2_Vtbl {
    pub const fn new<Identity: IDXGIFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsWindowedStereoEnabled<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::IsWindowedStereoEnabled(this)
            }
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hwnd: super::windef::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForHwnd(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pfullscreendesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pwindow: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForCoreWindow(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pwindow), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::winnt::HANDLE, pluid: *mut super::winnt::LUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::GetSharedResourceAdapterLuid(this, core::mem::transmute_copy(&hresource)) {
                    Ok(ok__) => {
                        pluid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::windef::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterStereoStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterStereoStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::UnregisterStereoStatus(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::windef::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterOcclusionStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterOcclusionStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::UnregisterOcclusionStatus(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForComposition(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIFactory1_Vtbl::new::<Identity, OFFSET>(),
            IsWindowedStereoEnabled: IsWindowedStereoEnabled::<Identity, OFFSET>,
            CreateSwapChainForHwnd: CreateSwapChainForHwnd::<Identity, OFFSET>,
            CreateSwapChainForCoreWindow: CreateSwapChainForCoreWindow::<Identity, OFFSET>,
            GetSharedResourceAdapterLuid: GetSharedResourceAdapterLuid::<Identity, OFFSET>,
            RegisterStereoStatusWindow: RegisterStereoStatusWindow::<Identity, OFFSET>,
            RegisterStereoStatusEvent: RegisterStereoStatusEvent::<Identity, OFFSET>,
            UnregisterStereoStatus: UnregisterStereoStatus::<Identity, OFFSET>,
            RegisterOcclusionStatusWindow: RegisterOcclusionStatusWindow::<Identity, OFFSET>,
            RegisterOcclusionStatusEvent: RegisterOcclusionStatusEvent::<Identity, OFFSET>,
            UnregisterOcclusionStatus: UnregisterOcclusionStatus::<Identity, OFFSET>,
            CreateSwapChainForComposition: CreateSwapChainForComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory2 {}
windows_core::imp::define_interface!(IDXGIFactory3, IDXGIFactory3_Vtbl, 0x25483823_cd46_4c7d_86ca_47aa95b837bd);
impl core::ops::Deref for IDXGIFactory3 {
    type Target = IDXGIFactory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory3, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2);
impl IDXGIFactory3 {
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory3_Vtbl {
    pub base__: IDXGIFactory2_Vtbl,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory3_Impl: IDXGIFactory2_Impl {
    fn GetCreationFlags(&self) -> u32;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory3_Vtbl {
    pub const fn new<Identity: IDXGIFactory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCreationFlags<Identity: IDXGIFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory3_Impl::GetCreationFlags(this)
            }
        }
        Self { base__: IDXGIFactory2_Vtbl::new::<Identity, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory3 {}
windows_core::imp::define_interface!(IDXGIFactory4, IDXGIFactory4_Vtbl, 0x1bc6ea02_ef36_464f_bf0c_21ca39e5168a);
impl core::ops::Deref for IDXGIFactory4 {
    type Target = IDXGIFactory3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory4, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3);
impl IDXGIFactory4 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::winnt::LUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterByLuid)(windows_core::Interface::as_raw(self), core::mem::transmute(adapterluid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumWarpAdapter)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory4_Vtbl {
    pub base__: IDXGIFactory3_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub EnumAdapterByLuid: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    EnumAdapterByLuid: usize,
    pub EnumWarpAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory4_Impl: IDXGIFactory3_Impl {
    fn EnumAdapterByLuid(&self, adapterluid: &super::winnt::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumWarpAdapter(&self, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory4_Vtbl {
    pub const fn new<Identity: IDXGIFactory4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapterByLuid<Identity: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapterluid: super::winnt::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory4_Impl::EnumAdapterByLuid(this, core::mem::transmute(&adapterluid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        unsafe extern "system" fn EnumWarpAdapter<Identity: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory4_Impl::EnumWarpAdapter(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        Self {
            base__: IDXGIFactory3_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapterByLuid: EnumAdapterByLuid::<Identity, OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory4 {}
windows_core::imp::define_interface!(IDXGIFactory5, IDXGIFactory5_Vtbl, 0x7632e1f5_ee65_4dca_87fd_84cd75f8838d);
impl core::ops::Deref for IDXGIFactory5 {
    type Target = IDXGIFactory4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory5, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4);
impl IDXGIFactory5 {
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, pfeaturesupportdata as _, featuresupportdatasize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory5_Vtbl {
    pub base__: IDXGIFactory4_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_FEATURE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory5_Impl: IDXGIFactory4_Impl {
    fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory5_Vtbl {
    pub const fn new<Identity: IDXGIFactory5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckFeatureSupport<Identity: IDXGIFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory5_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
            }
        }
        Self { base__: IDXGIFactory4_Vtbl::new::<Identity, OFFSET>(), CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory5 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory5 {}
windows_core::imp::define_interface!(IDXGIFactory6, IDXGIFactory6_Vtbl, 0xc1b6694f_ff09_44a9_b03c_77900a0a1d17);
impl core::ops::Deref for IDXGIFactory6 {
    type Target = IDXGIFactory5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory6, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5);
impl IDXGIFactory6 {
    pub unsafe fn EnumAdapterByGpuPreference<T>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterByGpuPreference)(windows_core::Interface::as_raw(self), adapter, gpupreference, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory6_Vtbl {
    pub base__: IDXGIFactory5_Vtbl,
    pub EnumAdapterByGpuPreference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_GPU_PREFERENCE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory6_Impl: IDXGIFactory5_Impl {
    fn EnumAdapterByGpuPreference(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory6_Vtbl {
    pub const fn new<Identity: IDXGIFactory6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Identity: IDXGIFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory6_Impl::EnumAdapterByGpuPreference(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&gpupreference), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        Self { base__: IDXGIFactory5_Vtbl::new::<Identity, OFFSET>(), EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory6 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIFactory5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory6 {}
windows_core::imp::define_interface!(IDXGIFactory7, IDXGIFactory7_Vtbl, 0xa4966eed_76db_44da_84c1_ee9a7afb20a8);
impl core::ops::Deref for IDXGIFactory7 {
    type Target = IDXGIFactory6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory7, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5, IDXGIFactory6);
impl IDXGIFactory7 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterAdaptersChangedEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterAdaptersChangedEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterAdaptersChangedEvent)(windows_core::Interface::as_raw(self), dwcookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory7_Vtbl {
    pub base__: IDXGIFactory6_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterAdaptersChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterAdaptersChangedEvent: usize,
    pub UnregisterAdaptersChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory7_Impl: IDXGIFactory6_Impl {
    fn RegisterAdaptersChangedEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory7_Vtbl {
    pub const fn new<Identity: IDXGIFactory7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Identity: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory7_Impl::RegisterAdaptersChangedEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Identity: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory7_Impl::UnregisterAdaptersChangedEvent(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: IDXGIFactory6_Vtbl::new::<Identity, OFFSET>(),
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Identity, OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory7 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIFactory5 as windows_core::Interface>::IID || iid == &<IDXGIFactory6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory7 {}
windows_core::imp::define_interface!(IDXGIFactoryMedia, IDXGIFactoryMedia_Vtbl, 0x41e7d1f2_a591_4f7b_a2e5_fa9c843e1c12);
windows_core::imp::interface_hierarchy!(IDXGIFactoryMedia, windows_core::IUnknown);
impl IDXGIFactoryMedia {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<P0, P3>(&self, pdevice: P0, hsurface: Option<super::winnt::HANDLE>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P3) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForCompositionSurfaceHandle)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hsurface.unwrap_or(core::mem::zeroed()) as _, pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<P0, P3, P4>(&self, pdevice: P0, hsurface: Option<super::winnt::HANDLE>, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: P3, prestricttooutput: P4) -> windows_core::Result<IDXGIDecodeSwapChain>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IDXGIResource>,
        P4: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDecodeSwapChainForCompositionSurfaceHandle)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hsurface.unwrap_or(core::mem::zeroed()) as _, pdesc, pyuvdecodebuffers.param().abi(), prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactoryMedia_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub CreateSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::winnt::HANDLE, *const DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateSwapChainForCompositionSurfaceHandle: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CreateDecodeSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::winnt::HANDLE, *const DXGI_DECODE_SWAP_CHAIN_DESC, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateDecodeSwapChainForCompositionSurfaceHandle: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIFactoryMedia_Impl: windows_core::IUnknownImpl {
    fn CreateSwapChainForCompositionSurfaceHandle(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hsurface: super::winnt::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hsurface: super::winnt::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: windows_core::Ref<IDXGIResource>, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGIDecodeSwapChain>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIFactoryMedia_Vtbl {
    pub const fn new<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::winnt::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactoryMedia_Impl::CreateSwapChainForCompositionSurfaceHandle(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::winnt::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut core::ffi::c_void, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactoryMedia_Impl::CreateDecodeSwapChainForCompositionSurfaceHandle(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pyuvdecodebuffers), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Identity, OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactoryMedia as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIFactoryMedia {}
windows_core::imp::define_interface!(IDXGIInfoQueue, IDXGIInfoQueue_Vtbl, 0xd67441c7_672a_476f_9e82_cd55b44949ce);
windows_core::imp::interface_hierarchy!(IDXGIInfoQueue, windows_core::IUnknown);
impl IDXGIInfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, producer: DXGI_DEBUG_ID, messagecountlimit: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), messagecountlimit) }
    }
    pub unsafe fn ClearStoredMessages(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearStoredMessages)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn GetMessage(&self, producer: DXGI_DEBUG_ID, messageindex: u64, pmessage: Option<*mut DXGI_INFO_QUEUE_MESSAGE>, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), messageindex, pmessage.unwrap_or(core::mem::zeroed()) as _, pmessagebytelength as _) }
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilters)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumStoredMessages(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumStoredMessages)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetMessageCountLimit(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddStorageFilterEntries(&self, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddStorageFilterEntries)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter) }
    }
    pub unsafe fn GetStorageFilter(&self, producer: DXGI_DEBUG_ID, pfilter: Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter.unwrap_or(core::mem::zeroed()) as _, pfilterbytelength as _) }
    }
    pub unsafe fn ClearStorageFilter(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn PushEmptyStorageFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushEmptyStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushDenyAllStorageFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushDenyAllStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushCopyOfStorageFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushCopyOfStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushStorageFilter(&self, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter) }
    }
    pub unsafe fn PopStorageFilter(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).PopStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn GetStorageFilterStackSize(&self, producer: DXGI_DEBUG_ID) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetStorageFilterStackSize)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRetrievalFilterEntries)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter) }
    }
    pub unsafe fn GetRetrievalFilter(&self, producer: DXGI_DEBUG_ID, pfilter: Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter.unwrap_or(core::mem::zeroed()) as _, pfilterbytelength as _) }
    }
    pub unsafe fn ClearRetrievalFilter(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushEmptyRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushDenyAllRetrievalFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushDenyAllRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushCopyOfRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushRetrievalFilter(&self, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter) }
    }
    pub unsafe fn PopRetrievalFilter(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).PopRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self, producer: DXGI_DEBUG_ID) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetRetrievalFilterStackSize)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddMessage<P4>(&self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: DXGI_INFO_QUEUE_MESSAGE_ID, pdescription: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category, severity, id, pdescription.param().abi()) }
    }
    pub unsafe fn AddApplicationMessage<P1>(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddApplicationMessage)(windows_core::Interface::as_raw(self), severity, pdescription.param().abi()) }
    }
    pub unsafe fn SetBreakOnCategory(&self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnCategory)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category, benable.into()) }
    }
    pub unsafe fn SetBreakOnSeverity(&self, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnSeverity)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), severity, benable.into()) }
    }
    pub unsafe fn SetBreakOnID(&self, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID, benable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnID)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), id, benable.into()) }
    }
    pub unsafe fn GetBreakOnCategory(&self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnCategory)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category) }
    }
    pub unsafe fn GetBreakOnSeverity(&self, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnSeverity)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), severity) }
    }
    pub unsafe fn GetBreakOnID(&self, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnID)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), id) }
    }
    pub unsafe fn SetMuteDebugOutput(&self, producer: DXGI_DEBUG_ID, bmute: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMuteDebugOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), bmute.into());
        }
    }
    pub unsafe fn GetMuteDebugOutput(&self, producer: DXGI_DEBUG_ID) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMuteDebugOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIInfoQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, u64) -> windows_core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, u64, *mut DXGI_INFO_QUEUE_MESSAGE, *mut usize) -> windows_core::HRESULT,
    pub GetNumStoredMessagesAllowedByRetrievalFilters: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *mut DXGI_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushDenyAllStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *mut DXGI_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushDenyAllRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u32,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, DXGI_INFO_QUEUE_MESSAGE_ID, windows_core::PCSTR) -> windows_core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_ID, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL,
    pub GetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_ID) -> windows_core::BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, windows_core::BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::BOOL,
}
pub trait IDXGIInfoQueue_Impl: windows_core::IUnknownImpl {
    fn SetMessageCountLimit(&self, producer: &DXGI_DEBUG_ID, messagecountlimit: u64) -> windows_core::Result<()>;
    fn ClearStoredMessages(&self, producer: &DXGI_DEBUG_ID);
    fn GetMessage(&self, producer: &DXGI_DEBUG_ID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::Result<()>;
    fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetNumStoredMessages(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetMessageCountLimit(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetNumMessagesAllowedByStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn AddStorageFilterEntries(&self, producer: &DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetStorageFilter(&self, producer: &DXGI_DEBUG_ID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearStorageFilter(&self, producer: &DXGI_DEBUG_ID);
    fn PushEmptyStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushDenyAllStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushCopyOfStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushStorageFilter(&self, producer: &DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopStorageFilter(&self, producer: &DXGI_DEBUG_ID);
    fn GetStorageFilterStackSize(&self, producer: &DXGI_DEBUG_ID) -> u32;
    fn AddRetrievalFilterEntries(&self, producer: &DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetRetrievalFilter(&self, producer: &DXGI_DEBUG_ID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearRetrievalFilter(&self, producer: &DXGI_DEBUG_ID);
    fn PushEmptyRetrievalFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushDenyAllRetrievalFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushRetrievalFilter(&self, producer: &DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopRetrievalFilter(&self, producer: &DXGI_DEBUG_ID);
    fn GetRetrievalFilterStackSize(&self, producer: &DXGI_DEBUG_ID) -> u32;
    fn AddMessage(&self, producer: &DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: DXGI_INFO_QUEUE_MESSAGE_ID, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn AddApplicationMessage(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetBreakOnCategory(&self, producer: &DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnSeverity(&self, producer: &DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnID(&self, producer: &DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetBreakOnCategory(&self, producer: &DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL;
    fn GetBreakOnSeverity(&self, producer: &DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL;
    fn GetBreakOnID(&self, producer: &DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID) -> windows_core::BOOL;
    fn SetMuteDebugOutput(&self, producer: &DXGI_DEBUG_ID, bmute: windows_core::BOOL);
    fn GetMuteDebugOutput(&self, producer: &DXGI_DEBUG_ID) -> windows_core::BOOL;
}
impl IDXGIInfoQueue_Vtbl {
    pub const fn new<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, messagecountlimit: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messagecountlimit)).into()
            }
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearStoredMessages(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn GetMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
            }
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilters(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumStoredMessages(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMessageCountLimit(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn GetStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
            }
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearStorageFilter(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushEmptyStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushDenyAllStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushCopyOfStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn PopStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PopStorageFilter(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetStorageFilterStackSize(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
            }
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearRetrievalFilter(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushEmptyRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushDenyAllRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushCopyOfRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PopRetrievalFilter(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetRetrievalFilterStackSize(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: DXGI_INFO_QUEUE_MESSAGE_ID, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
            }
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnID<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category))
            }
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity))
            }
        }
        unsafe extern "system" fn GetBreakOnID<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id))
            }
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, bmute: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute(&producer), core::mem::transmute_copy(&bmute));
            }
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMuteDebugOutput(this, core::mem::transmute(&producer))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters: GetNumStoredMessagesAllowedByRetrievalFilters::<Identity, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, OFFSET>,
            PushDenyAllStorageFilter: PushDenyAllStorageFilter::<Identity, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, OFFSET>,
            PushDenyAllRetrievalFilter: PushDenyAllRetrievalFilter::<Identity, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, OFFSET>,
            AddMessage: AddMessage::<Identity, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIInfoQueue as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIInfoQueue {}
windows_core::imp::define_interface!(IDXGIKeyedMutex, IDXGIKeyedMutex_Vtbl, 0x9d8e1289_d7b3_465f_8126_250e349af85d);
impl core::ops::Deref for IDXGIKeyedMutex {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIKeyedMutex, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGIKeyedMutex {
    pub unsafe fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireSync)(windows_core::Interface::as_raw(self), key, dwmilliseconds) }
    }
    pub unsafe fn ReleaseSync(&self, key: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseSync)(windows_core::Interface::as_raw(self), key) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIKeyedMutex_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub AcquireSync: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32) -> windows_core::HRESULT,
    pub ReleaseSync: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
pub trait IDXGIKeyedMutex_Impl: IDXGIDeviceSubObject_Impl {
    fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn ReleaseSync(&self, key: u64) -> windows_core::Result<()>;
}
impl IDXGIKeyedMutex_Vtbl {
    pub const fn new<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcquireSync<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64, dwmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIKeyedMutex_Impl::AcquireSync(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn ReleaseSync<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIKeyedMutex_Impl::ReleaseSync(this, core::mem::transmute_copy(&key)).into()
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            AcquireSync: AcquireSync::<Identity, OFFSET>,
            ReleaseSync: ReleaseSync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIKeyedMutex as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIKeyedMutex {}
windows_core::imp::define_interface!(IDXGIObject, IDXGIObject_Vtbl, 0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
windows_core::imp::interface_hierarchy!(IDXGIObject, windows_core::IUnknown);
impl IDXGIObject {
    pub unsafe fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), name, datasize, pdata) }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, name: *const windows_core::GUID, punknown: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), name, punknown.param().abi()) }
    }
    pub unsafe fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), name, pdatasize as _, pdata as _) }
    }
    pub unsafe fn GetParent<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDXGIObject_Impl: windows_core::IUnknownImpl {
    fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, name: *const windows_core::GUID, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParent(&self, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: IDXGIObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrivateData<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetParent<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetParent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIObject {}
windows_core::imp::define_interface!(IDXGIOutput, IDXGIOutput_Vtbl, 0xae02eedb_c735_4690_8d52_5a8dc20213aa);
impl core::ops::Deref for IDXGIOutput {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput, windows_core::IUnknown, IDXGIObject);
impl IDXGIOutput {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetDisplayModeList(&self, enumformat: DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: Option<*mut DXGI_MODE_DESC>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeList)(windows_core::Interface::as_raw(self), enumformat, flags, pnummodes as _, pdesc.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn FindClosestMatchingMode<P2>(&self, pmodetomatch: *const DXGI_MODE_DESC, pclosestmatch: *mut DXGI_MODE_DESC, pconcerneddevice: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindClosestMatchingMode)(windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch as _, pconcerneddevice.param().abi()) }
    }
    pub unsafe fn WaitForVBlank(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForVBlank)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TakeOwnership<P0>(&self, pdevice: P0, exclusive: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).TakeOwnership)(windows_core::Interface::as_raw(self), pdevice.param().abi(), exclusive.into()) }
    }
    pub unsafe fn ReleaseOwnership(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseOwnership)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGammaControlCapabilities)(windows_core::Interface::as_raw(self), pgammacaps as _) }
    }
    pub unsafe fn SetGammaControl(&self, parray: *const DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGammaControl)(windows_core::Interface::as_raw(self), parray) }
    }
    pub unsafe fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGammaControl)(windows_core::Interface::as_raw(self), parray as _) }
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplaySurface)(windows_core::Interface::as_raw(self), pscanoutsurface.param().abi()) }
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDisplaySurfaceData)(windows_core::Interface::as_raw(self), pdestination.param().abi()) }
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), pstats as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetDesc: usize,
    pub GetDisplayModeList: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_FORMAT, u32, *mut u32, *mut DXGI_MODE_DESC) -> windows_core::HRESULT,
    pub FindClosestMatchingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MODE_DESC, *mut DXGI_MODE_DESC, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForVBlank: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ReleaseOwnership: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetGammaControlCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT,
    pub SetGammaControl: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_GAMMA_CONTROL) -> windows_core::HRESULT,
    pub GetGammaControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_GAMMA_CONTROL) -> windows_core::HRESULT,
    pub SetDisplaySurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplaySurfaceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIOutput_Impl: IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::Result<()>;
    fn GetDisplayModeList(&self, enumformat: DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn FindClosestMatchingMode(&self, pmodetomatch: *const DXGI_MODE_DESC, pclosestmatch: *mut DXGI_MODE_DESC, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn WaitForVBlank(&self) -> windows_core::Result<()>;
    fn TakeOwnership(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, exclusive: windows_core::BOOL) -> windows_core::Result<()>;
    fn ReleaseOwnership(&self);
    fn GetGammaControlCapabilities(&self, pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::Result<()>;
    fn SetGammaControl(&self, parray: *const DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn SetDisplaySurface(&self, pscanoutsurface: windows_core::Ref<IDXGISurface>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData(&self, pdestination: windows_core::Ref<IDXGISurface>) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIOutput_Vtbl {
    pub const fn new<Identity: IDXGIOutput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeList<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDisplayModeList(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC, pclosestmatch: *mut DXGI_MODE_DESC, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::FindClosestMatchingMode(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), core::mem::transmute_copy(&pconcerneddevice)).into()
            }
        }
        unsafe extern "system" fn WaitForVBlank<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::WaitForVBlank(this).into()
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, exclusive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::TakeOwnership(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&exclusive)).into()
            }
        }
        unsafe extern "system" fn ReleaseOwnership<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::ReleaseOwnership(this);
            }
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetGammaControlCapabilities(this, core::mem::transmute_copy(&pgammacaps)).into()
            }
        }
        unsafe extern "system" fn SetGammaControl<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *const DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::SetGammaControl(this, core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn GetGammaControl<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *mut DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetGammaControl(this, core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn SetDisplaySurface<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscanoutsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::SetDisplaySurface(this, core::mem::transmute_copy(&pscanoutsurface)).into()
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDisplaySurfaceData(this, core::mem::transmute_copy(&pdestination)).into()
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Identity, OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Identity, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Identity, OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Identity, OFFSET>,
            SetGammaControl: SetGammaControl::<Identity, OFFSET>,
            GetGammaControl: GetGammaControl::<Identity, OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Identity, OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIOutput {}
windows_core::imp::define_interface!(IDXGIOutput1, IDXGIOutput1_Vtbl, 0x00cddea8_939b_4b83_a340_a685226666cc);
impl core::ops::Deref for IDXGIOutput1 {
    type Target = IDXGIOutput;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput1, windows_core::IUnknown, IDXGIObject, IDXGIOutput);
impl IDXGIOutput1 {
    pub unsafe fn GetDisplayModeList1(&self, enumformat: DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: Option<*mut DXGI_MODE_DESC1>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeList1)(windows_core::Interface::as_raw(self), enumformat, flags, pnummodes as _, pdesc.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn FindClosestMatchingMode1<P2>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindClosestMatchingMode1)(windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch as _, pconcerneddevice.param().abi()) }
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDXGIResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDisplaySurfaceData1)(windows_core::Interface::as_raw(self), pdestination.param().abi()) }
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> windows_core::Result<IDXGIOutputDuplication>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateOutput)(windows_core::Interface::as_raw(self), pdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput1_Vtbl {
    pub base__: IDXGIOutput_Vtbl,
    pub GetDisplayModeList1: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_FORMAT, u32, *mut u32, *mut DXGI_MODE_DESC1) -> windows_core::HRESULT,
    pub FindClosestMatchingMode1: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MODE_DESC1, *mut DXGI_MODE_DESC1, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplaySurfaceData1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DuplicateOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIOutput1_Impl: IDXGIOutput_Impl {
    fn GetDisplayModeList1(&self, enumformat: DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::Result<()>;
    fn FindClosestMatchingMode1(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData1(&self, pdestination: windows_core::Ref<IDXGIResource>) -> windows_core::Result<()>;
    fn DuplicateOutput(&self, pdevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIOutput1_Vtbl {
    pub const fn new<Identity: IDXGIOutput1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayModeList1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::GetDisplayModeList1(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::FindClosestMatchingMode1(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), core::mem::transmute_copy(&pconcerneddevice)).into()
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::GetDisplaySurfaceData1(this, core::mem::transmute_copy(&pdestination)).into()
            }
        }
        unsafe extern "system" fn DuplicateOutput<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput1_Impl::DuplicateOutput(this, core::mem::transmute_copy(&pdevice)) {
                    Ok(ok__) => {
                        ppoutputduplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIOutput_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayModeList1: GetDisplayModeList1::<Identity, OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Identity, OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Identity, OFFSET>,
            DuplicateOutput: DuplicateOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIOutput1 {}
windows_core::imp::define_interface!(IDXGIOutput2, IDXGIOutput2_Vtbl, 0x595e39d1_2724_4663_99b1_da969de28364);
impl core::ops::Deref for IDXGIOutput2 {
    type Target = IDXGIOutput1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput2, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1);
impl IDXGIOutput2 {
    pub unsafe fn SupportsOverlays(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).SupportsOverlays)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput2_Vtbl {
    pub base__: IDXGIOutput1_Vtbl,
    pub SupportsOverlays: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIOutput2_Impl: IDXGIOutput1_Impl {
    fn SupportsOverlays(&self) -> windows_core::BOOL;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIOutput2_Vtbl {
    pub const fn new<Identity: IDXGIOutput2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportsOverlays<Identity: IDXGIOutput2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput2_Impl::SupportsOverlays(this)
            }
        }
        Self { base__: IDXGIOutput1_Vtbl::new::<Identity, OFFSET>(), SupportsOverlays: SupportsOverlays::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIOutput2 {}
windows_core::imp::define_interface!(IDXGIOutput3, IDXGIOutput3_Vtbl, 0x8a6bb301_7e7e_41f4_a8e0_5b32f7f99b18);
impl core::ops::Deref for IDXGIOutput3 {
    type Target = IDXGIOutput2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput3, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2);
impl IDXGIOutput3 {
    pub unsafe fn CheckOverlaySupport<P1>(&self, enumformat: DXGI_FORMAT, pconcerneddevice: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckOverlaySupport)(windows_core::Interface::as_raw(self), enumformat, pconcerneddevice.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput3_Vtbl {
    pub base__: IDXGIOutput2_Vtbl,
    pub CheckOverlaySupport: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_FORMAT, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIOutput3_Impl: IDXGIOutput2_Impl {
    fn CheckOverlaySupport(&self, enumformat: DXGI_FORMAT, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIOutput3_Vtbl {
    pub const fn new<Identity: IDXGIOutput3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckOverlaySupport<Identity: IDXGIOutput3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: DXGI_FORMAT, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput3_Impl::CheckOverlaySupport(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&pconcerneddevice)) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIOutput2_Vtbl::new::<Identity, OFFSET>(), CheckOverlaySupport: CheckOverlaySupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIOutput3 {}
windows_core::imp::define_interface!(IDXGIOutput4, IDXGIOutput4_Vtbl, 0xdc7dca35_2196_414d_9f53_617884032a60);
impl core::ops::Deref for IDXGIOutput4 {
    type Target = IDXGIOutput3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput4, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3);
impl IDXGIOutput4 {
    pub unsafe fn CheckOverlayColorSpaceSupport<P2>(&self, format: DXGI_FORMAT, colorspace: DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P2) -> windows_core::Result<u32>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckOverlayColorSpaceSupport)(windows_core::Interface::as_raw(self), format, colorspace, pconcerneddevice.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput4_Vtbl {
    pub base__: IDXGIOutput3_Vtbl,
    pub CheckOverlayColorSpaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_FORMAT, DXGI_COLOR_SPACE_TYPE, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIOutput4_Impl: IDXGIOutput3_Impl {
    fn CheckOverlayColorSpaceSupport(&self, format: DXGI_FORMAT, colorspace: DXGI_COLOR_SPACE_TYPE, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIOutput4_Vtbl {
    pub const fn new<Identity: IDXGIOutput4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Identity: IDXGIOutput4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: DXGI_FORMAT, colorspace: DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput4_Impl::CheckOverlayColorSpaceSupport(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&colorspace), core::mem::transmute_copy(&pconcerneddevice)) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIOutput3_Vtbl::new::<Identity, OFFSET>(), CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIOutput4 {}
windows_core::imp::define_interface!(IDXGIOutput5, IDXGIOutput5_Vtbl, 0x80a07424_ab52_42eb_833c_0c42fd282d98);
impl core::ops::Deref for IDXGIOutput5 {
    type Target = IDXGIOutput4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput5, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3, IDXGIOutput4);
impl IDXGIOutput5 {
    pub unsafe fn DuplicateOutput1<P0>(&self, pdevice: P0, flags: u32, psupportedformats: &[DXGI_FORMAT]) -> windows_core::Result<IDXGIOutputDuplication>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateOutput1)(windows_core::Interface::as_raw(self), pdevice.param().abi(), flags, psupportedformats.len().try_into().unwrap(), core::mem::transmute(psupportedformats.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput5_Vtbl {
    pub base__: IDXGIOutput4_Vtbl,
    pub DuplicateOutput1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const DXGI_FORMAT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIOutput5_Impl: IDXGIOutput4_Impl {
    fn DuplicateOutput1(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, flags: u32, supportedformatscount: u32, psupportedformats: *const DXGI_FORMAT) -> windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIOutput5_Vtbl {
    pub const fn new<Identity: IDXGIOutput5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DuplicateOutput1<Identity: IDXGIOutput5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const DXGI_FORMAT, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput5_Impl::DuplicateOutput1(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&supportedformatscount), core::mem::transmute_copy(&psupportedformats)) {
                    Ok(ok__) => {
                        ppoutputduplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIOutput4_Vtbl::new::<Identity, OFFSET>(), DuplicateOutput1: DuplicateOutput1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput5 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIOutput4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIOutput5 {}
windows_core::imp::define_interface!(IDXGIOutput6, IDXGIOutput6_Vtbl, 0x068346e8_aaec_4b84_add7_137f513f77a1);
impl core::ops::Deref for IDXGIOutput6 {
    type Target = IDXGIOutput5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput6, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3, IDXGIOutput4, IDXGIOutput5);
impl IDXGIOutput6 {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn CheckHardwareCompositionSupport(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckHardwareCompositionSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput6_Vtbl {
    pub base__: IDXGIOutput5_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetDesc1: usize,
    pub CheckHardwareCompositionSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIOutput6_Impl: IDXGIOutput5_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::Result<()>;
    fn CheckHardwareCompositionSupport(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIOutput6_Vtbl {
    pub const fn new<Identity: IDXGIOutput6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput6_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Identity: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput6_Impl::CheckHardwareCompositionSupport(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIOutput5_Vtbl::new::<Identity, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput6 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIOutput4 as windows_core::Interface>::IID || iid == &<IDXGIOutput5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIOutput6 {}
windows_core::imp::define_interface!(IDXGIOutputDuplication, IDXGIOutputDuplication_Vtbl, 0x191cfac3_a341_470d_b26e_a864f428319c);
impl core::ops::Deref for IDXGIOutputDuplication {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutputDuplication, windows_core::IUnknown, IDXGIObject);
impl IDXGIOutputDuplication {
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut Option<IDXGIResource>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireNextFrame)(windows_core::Interface::as_raw(self), timeoutinmilliseconds, pframeinfo as _, core::mem::transmute(ppdesktopresource)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::windef::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameDirtyRects)(windows_core::Interface::as_raw(self), dirtyrectsbuffersize, pdirtyrectsbuffer as _, pdirtyrectsbuffersizerequired as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameMoveRects)(windows_core::Interface::as_raw(self), moverectsbuffersize, pmoverectbuffer as _, pmoverectsbuffersizerequired as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFramePointerShape)(windows_core::Interface::as_raw(self), pointershapebuffersize, ppointershapebuffer as _, ppointershapebuffersizerequired as _, ppointershapeinfo as _) }
    }
    pub unsafe fn MapDesktopSurface(&self) -> windows_core::Result<DXGI_MAPPED_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MapDesktopSurface)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnMapDesktopSurface(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnMapDesktopSurface)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseFrame(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFrame)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutputDuplication_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTDUPL_DESC),
    #[cfg(feature = "Win32_windef")]
    pub AcquireNextFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXGI_OUTDUPL_FRAME_INFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    AcquireNextFrame: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetFrameDirtyRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::windef::RECT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetFrameDirtyRects: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetFrameMoveRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXGI_OUTDUPL_MOVE_RECT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetFrameMoveRects: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetFramePointerShape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetFramePointerShape: usize,
    pub MapDesktopSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MAPPED_RECT) -> windows_core::HRESULT,
    pub UnMapDesktopSurface: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGIOutputDuplication_Impl: IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC);
    fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: windows_core::OutRef<IDXGIResource>) -> windows_core::Result<()>;
    fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::windef::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::Result<()>;
    fn MapDesktopSurface(&self) -> windows_core::Result<DXGI_MAPPED_RECT>;
    fn UnMapDesktopSurface(&self) -> windows_core::Result<()>;
    fn ReleaseFrame(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGIOutputDuplication_Vtbl {
    pub const fn new<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        unsafe extern "system" fn AcquireNextFrame<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::AcquireNextFrame(this, core::mem::transmute_copy(&timeoutinmilliseconds), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&ppdesktopresource)).into()
            }
        }
        unsafe extern "system" fn GetFrameDirtyRects<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::windef::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFrameDirtyRects(this, core::mem::transmute_copy(&dirtyrectsbuffersize), core::mem::transmute_copy(&pdirtyrectsbuffer), core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into()
            }
        }
        unsafe extern "system" fn GetFrameMoveRects<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFrameMoveRects(this, core::mem::transmute_copy(&moverectsbuffersize), core::mem::transmute_copy(&pmoverectbuffer), core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into()
            }
        }
        unsafe extern "system" fn GetFramePointerShape<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFramePointerShape(this, core::mem::transmute_copy(&pointershapebuffersize), core::mem::transmute_copy(&ppointershapebuffer), core::mem::transmute_copy(&ppointershapebuffersizerequired), core::mem::transmute_copy(&ppointershapeinfo)).into()
            }
        }
        unsafe extern "system" fn MapDesktopSurface<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutputDuplication_Impl::MapDesktopSurface(this) {
                    Ok(ok__) => {
                        plockedrect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::UnMapDesktopSurface(this).into()
            }
        }
        unsafe extern "system" fn ReleaseFrame<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::ReleaseFrame(this).into()
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            AcquireNextFrame: AcquireNextFrame::<Identity, OFFSET>,
            GetFrameDirtyRects: GetFrameDirtyRects::<Identity, OFFSET>,
            GetFrameMoveRects: GetFrameMoveRects::<Identity, OFFSET>,
            GetFramePointerShape: GetFramePointerShape::<Identity, OFFSET>,
            MapDesktopSurface: MapDesktopSurface::<Identity, OFFSET>,
            UnMapDesktopSurface: UnMapDesktopSurface::<Identity, OFFSET>,
            ReleaseFrame: ReleaseFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutputDuplication as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGIOutputDuplication {}
windows_core::imp::define_interface!(IDXGIResource, IDXGIResource_Vtbl, 0x035f3ab4_482e_4e50_b41f_8a7f8bd8960b);
impl core::ops::Deref for IDXGIResource {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIResource, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGIResource {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetSharedHandle(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharedHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUsage(&self) -> windows_core::Result<DXGI_USAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUsage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEvictionPriority)(windows_core::Interface::as_raw(self), evictionpriority) }
    }
    pub unsafe fn GetEvictionPriority(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEvictionPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetSharedHandle: usize,
    pub GetUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_USAGE) -> windows_core::HRESULT,
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDXGIResource_Impl: IDXGIDeviceSubObject_Impl {
    fn GetSharedHandle(&self) -> windows_core::Result<super::winnt::HANDLE>;
    fn GetUsage(&self) -> windows_core::Result<DXGI_USAGE>;
    fn SetEvictionPriority(&self, evictionpriority: u32) -> windows_core::Result<()>;
    fn GetEvictionPriority(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_winnt")]
impl IDXGIResource_Vtbl {
    pub const fn new<Identity: IDXGIResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSharedHandle<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetSharedHandle(this) {
                    Ok(ok__) => {
                        psharedhandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUsage<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusage: *mut DXGI_USAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetUsage(this) {
                    Ok(ok__) => {
                        pusage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIResource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority)).into()
            }
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevictionpriority: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetEvictionPriority(this) {
                    Ok(ok__) => {
                        pevictionpriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            GetSharedHandle: GetSharedHandle::<Identity, OFFSET>,
            GetUsage: GetUsage::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDXGIResource {}
windows_core::imp::define_interface!(IDXGIResource1, IDXGIResource1_Vtbl, 0x30961379_4609_4a41_998e_54fe567ee0c1);
impl core::ops::Deref for IDXGIResource1 {
    type Target = IDXGIResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIResource1, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGIResource);
impl IDXGIResource1 {
    pub unsafe fn CreateSubresourceSurface(&self, index: u32) -> windows_core::Result<IDXGISurface2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSubresourceSurface)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
    pub unsafe fn CreateSharedHandle<P2>(&self, pattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, dwaccess: u32, lpname: P2) -> windows_core::Result<super::winnt::HANDLE>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSharedHandle)(windows_core::Interface::as_raw(self), pattributes.unwrap_or(core::mem::zeroed()) as _, dwaccess, lpname.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource1_Vtbl {
    pub base__: IDXGIResource_Vtbl,
    pub CreateSubresourceSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
    pub CreateSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::minwinbase::SECURITY_ATTRIBUTES, u32, windows_core::PCWSTR, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_winnt")))]
    CreateSharedHandle: usize,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
pub trait IDXGIResource1_Impl: IDXGIResource_Impl {
    fn CreateSubresourceSurface(&self, index: u32) -> windows_core::Result<IDXGISurface2>;
    fn CreateSharedHandle(&self, pattributes: *const super::minwinbase::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &windows_core::PCWSTR) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
impl IDXGIResource1_Vtbl {
    pub const fn new<Identity: IDXGIResource1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSubresourceSurface<Identity: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource1_Impl::CreateSubresourceSurface(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *const super::minwinbase::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: windows_core::PCWSTR, phandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource1_Impl::CreateSharedHandle(this, core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&dwaccess), core::mem::transmute(&lpname)) {
                    Ok(ok__) => {
                        phandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIResource_Vtbl::new::<Identity, OFFSET>(),
            CreateSubresourceSurface: CreateSubresourceSurface::<Identity, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIResource1 {}
windows_core::imp::define_interface!(IDXGISurface, IDXGISurface_Vtbl, 0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
impl core::ops::Deref for IDXGISurface {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISurface {
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), plockedrect as _, mapflags) }
    }
    pub unsafe fn Unmap(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MAPPED_RECT, u32) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDXGISurface_Impl: IDXGIDeviceSubObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::Result<()>;
    fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> windows_core::Result<()>;
    fn Unmap(&self) -> windows_core::Result<()>;
}
impl IDXGISurface_Vtbl {
    pub const fn new<Identity: IDXGISurface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn Map<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface_Impl::Map(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&mapflags)).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface_Impl::Unmap(this).into()
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGISurface {}
windows_core::imp::define_interface!(IDXGISurface1, IDXGISurface1_Vtbl, 0x4ae63092_6327_4c1b_80ae_bfe12ea32b86);
impl core::ops::Deref for IDXGISurface1 {
    type Target = IDXGISurface;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface1, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface);
impl IDXGISurface1 {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetDC(&self, discard: bool) -> windows_core::Result<super::windef::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), discard.into(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn ReleaseDC(&self, pdirtyrect: Option<*const super::windef::RECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), pdirtyrect.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface1_Vtbl {
    pub base__: IDXGISurface_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetDC: usize,
    #[cfg(feature = "Win32_windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    ReleaseDC: usize,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGISurface1_Impl: IDXGISurface_Impl {
    fn GetDC(&self, discard: windows_core::BOOL) -> windows_core::Result<super::windef::HDC>;
    fn ReleaseDC(&self, pdirtyrect: *const super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGISurface1_Vtbl {
    pub const fn new<Identity: IDXGISurface1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDC<Identity: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discard: windows_core::BOOL, phdc: *mut super::windef::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISurface1_Impl::GetDC(this, core::mem::transmute_copy(&discard)) {
                    Ok(ok__) => {
                        phdc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface1_Impl::ReleaseDC(this, core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self { base__: IDXGISurface_Vtbl::new::<Identity, OFFSET>(), GetDC: GetDC::<Identity, OFFSET>, ReleaseDC: ReleaseDC::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISurface as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGISurface1 {}
windows_core::imp::define_interface!(IDXGISurface2, IDXGISurface2_Vtbl, 0xaba496dd_b617_4cb8_a866_bc44d7eb1fa2);
impl core::ops::Deref for IDXGISurface2 {
    type Target = IDXGISurface1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface2, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface, IDXGISurface1);
impl IDXGISurface2 {
    pub unsafe fn GetResource<T>(&self, psubresourceindex: *mut u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &T::IID, &mut result__, psubresourceindex as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface2_Vtbl {
    pub base__: IDXGISurface1_Vtbl,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGISurface2_Impl: IDXGISurface1_Impl {
    fn GetResource(&self, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGISurface2_Vtbl {
    pub const fn new<Identity: IDXGISurface2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource<Identity: IDXGISurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface2_Impl::GetResource(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparentresource), core::mem::transmute_copy(&psubresourceindex)).into()
            }
        }
        Self { base__: IDXGISurface1_Vtbl::new::<Identity, OFFSET>(), GetResource: GetResource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISurface as windows_core::Interface>::IID || iid == &<IDXGISurface1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGISurface2 {}
windows_core::imp::define_interface!(IDXGISwapChain, IDXGISwapChain_Vtbl, 0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
impl core::ops::Deref for IDXGISwapChain {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISwapChain {
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), syncinterval, flags) }
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn SetFullscreenState<P1>(&self, fullscreen: bool, ptarget: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDXGIOutput>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFullscreenState)(windows_core::Interface::as_raw(self), fullscreen.into(), ptarget.param().abi()) }
    }
    pub unsafe fn GetFullscreenState(&self, pfullscreen: Option<*mut windows_core::BOOL>, pptarget: Option<*mut Option<IDXGIOutput>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFullscreenState)(windows_core::Interface::as_raw(self), pfullscreen.unwrap_or(core::mem::zeroed()) as _, pptarget.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: DXGI_FORMAT, swapchainflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResizeBuffers)(windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags) }
    }
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const DXGI_MODE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResizeTarget)(windows_core::Interface::as_raw(self), pnewtargetparameters) }
    }
    pub unsafe fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainingOutput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), pstats as _) }
    }
    pub unsafe fn GetLastPresentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPresentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFullscreenState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFullscreenState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetDesc: usize,
    pub ResizeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, DXGI_FORMAT, u32) -> windows_core::HRESULT,
    pub ResizeTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MODE_DESC) -> windows_core::HRESULT,
    pub GetContainingOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT,
    pub GetLastPresentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGISwapChain_Impl: IDXGIDeviceSubObject_Impl {
    fn Present(&self, syncinterval: u32, flags: u32) -> windows_core::Result<()>;
    fn GetBuffer(&self, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetFullscreenState(&self, fullscreen: windows_core::BOOL, ptarget: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetFullscreenState(&self, pfullscreen: *mut windows_core::BOOL, pptarget: windows_core::OutRef<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::Result<()>;
    fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: DXGI_FORMAT, swapchainflags: u32) -> windows_core::Result<()>;
    fn ResizeTarget(&self, pnewtargetparameters: *const DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
    fn GetLastPresentCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGISwapChain_Vtbl {
    pub const fn new<Identity: IDXGISwapChain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Present<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::Present(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsurface)).into()
            }
        }
        unsafe extern "system" fn SetFullscreenState<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreen: windows_core::BOOL, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::SetFullscreenState(this, core::mem::transmute_copy(&fullscreen), core::mem::transmute_copy(&ptarget)).into()
            }
        }
        unsafe extern "system" fn GetFullscreenState<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfullscreen: *mut windows_core::BOOL, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetFullscreenState(this, core::mem::transmute_copy(&pfullscreen), core::mem::transmute_copy(&pptarget)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn ResizeBuffers<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: DXGI_FORMAT, swapchainflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::ResizeBuffers(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&newformat), core::mem::transmute_copy(&swapchainflags)).into()
            }
        }
        unsafe extern "system" fn ResizeTarget<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewtargetparameters: *const DXGI_MODE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::ResizeTarget(this, core::mem::transmute_copy(&pnewtargetparameters)).into()
            }
        }
        unsafe extern "system" fn GetContainingOutput<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain_Impl::GetContainingOutput(this) {
                    Ok(ok__) => {
                        ppoutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain_Impl::GetLastPresentCount(this) {
                    Ok(ok__) => {
                        plastpresentcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SetFullscreenState: SetFullscreenState::<Identity, OFFSET>,
            GetFullscreenState: GetFullscreenState::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            ResizeBuffers: ResizeBuffers::<Identity, OFFSET>,
            ResizeTarget: ResizeTarget::<Identity, OFFSET>,
            GetContainingOutput: GetContainingOutput::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGISwapChain {}
windows_core::imp::define_interface!(IDXGISwapChain1, IDXGISwapChain1_Vtbl, 0x790a45f7_0d42_4876_983a_0a55cfe6f4aa);
impl core::ops::Deref for IDXGISwapChain1 {
    type Target = IDXGISwapChain;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain1, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain);
impl IDXGISwapChain1 {
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFullscreenDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetHwnd(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCoreWindow)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present1)(windows_core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters) }
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsTemporaryMonoSupported)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRestrictToOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictToOutput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBackgroundColor)(windows_core::Interface::as_raw(self), pcolor) }
    }
    pub unsafe fn GetBackgroundColor(&self) -> windows_core::Result<DXGI_RGBA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackgroundColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), rotation) }
    }
    pub unsafe fn GetRotation(&self) -> windows_core::Result<DXGI_MODE_ROTATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_Vtbl {
    pub base__: IDXGISwapChain_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT,
    pub GetFullscreenDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetHwnd: usize,
    pub GetCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub Present1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    Present1: usize,
    pub IsTemporaryMonoSupported: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetRestrictToOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_RGBA) -> windows_core::HRESULT,
    pub GetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_RGBA) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MODE_ROTATION) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDXGISwapChain1_Impl: IDXGISwapChain_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::Result<()>;
    fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::Result<()>;
    fn GetHwnd(&self) -> windows_core::Result<super::windef::HWND>;
    fn GetCoreWindow(&self, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::Result<()>;
    fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL;
    fn GetRestrictToOutput(&self) -> windows_core::Result<IDXGIOutput>;
    fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> windows_core::Result<()>;
    fn GetBackgroundColor(&self) -> windows_core::Result<DXGI_RGBA>;
    fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> windows_core::Result<()>;
    fn GetRotation(&self) -> windows_core::Result<DXGI_MODE_ROTATION>;
}
#[cfg(feature = "Win32_windef")]
impl IDXGISwapChain1_Vtbl {
    pub const fn new<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetFullscreenDesc<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::GetFullscreenDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetHwnd<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetHwnd(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCoreWindow<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::GetCoreWindow(this, core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn Present1<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::Present1(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&presentflags), core::mem::transmute_copy(&ppresentparameters)).into()
            }
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::IsTemporaryMonoSupported(this)
            }
        }
        unsafe extern "system" fn GetRestrictToOutput<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprestricttooutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetRestrictToOutput(this) {
                    Ok(ok__) => {
                        pprestricttooutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *const DXGI_RGBA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::SetBackgroundColor(this, core::mem::transmute_copy(&pcolor)).into()
            }
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetBackgroundColor(this) {
                    Ok(ok__) => {
                        pcolor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::SetRotation(this, core::mem::transmute_copy(&rotation)).into()
            }
        }
        unsafe extern "system" fn GetRotation<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protation: *mut DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetRotation(this) {
                    Ok(ok__) => {
                        protation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGISwapChain_Vtbl::new::<Identity, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, OFFSET>,
            GetFullscreenDesc: GetFullscreenDesc::<Identity, OFFSET>,
            GetHwnd: GetHwnd::<Identity, OFFSET>,
            GetCoreWindow: GetCoreWindow::<Identity, OFFSET>,
            Present1: Present1::<Identity, OFFSET>,
            IsTemporaryMonoSupported: IsTemporaryMonoSupported::<Identity, OFFSET>,
            GetRestrictToOutput: GetRestrictToOutput::<Identity, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            GetRotation: GetRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDXGISwapChain1 {}
windows_core::imp::define_interface!(IDXGISwapChain2, IDXGISwapChain2_Vtbl, 0xa8be2ac4_199f_4946_b331_79599fb98de7);
impl core::ops::Deref for IDXGISwapChain2 {
    type Target = IDXGISwapChain1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain2, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1);
impl IDXGISwapChain2 {
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceSize)(windows_core::Interface::as_raw(self), width, height) }
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSourceSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _) }
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency) }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::winnt::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).GetFrameLatencyWaitableObject)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixTransform)(windows_core::Interface::as_raw(self), pmatrix) }
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixTransform)(windows_core::Interface::as_raw(self), pmatrix as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_Vtbl {
    pub base__: IDXGISwapChain1_Vtbl,
    pub SetSourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::winnt::HANDLE,
    #[cfg(not(feature = "Win32_winnt"))]
    GetFrameLatencyWaitableObject: usize,
    pub SetMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGISwapChain2_Impl: IDXGISwapChain1_Impl {
    fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
    fn GetFrameLatencyWaitableObject(&self) -> super::winnt::HANDLE;
    fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::Result<()>;
    fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGISwapChain2_Vtbl {
    pub const fn new<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSourceSize<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetSourceSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetSourceSize<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetSourceSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain2_Impl::GetMaximumFrameLatency(this) {
                    Ok(ok__) => {
                        pmaxlatency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::winnt::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetFrameLatencyWaitableObject(this)
            }
        }
        unsafe extern "system" fn SetMatrixTransform<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetMatrixTransform<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        Self {
            base__: IDXGISwapChain1_Vtbl::new::<Identity, OFFSET>(),
            SetSourceSize: SetSourceSize::<Identity, OFFSET>,
            GetSourceSize: GetSourceSize::<Identity, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
            GetFrameLatencyWaitableObject: GetFrameLatencyWaitableObject::<Identity, OFFSET>,
            SetMatrixTransform: SetMatrixTransform::<Identity, OFFSET>,
            GetMatrixTransform: GetMatrixTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGISwapChain2 {}
windows_core::imp::define_interface!(IDXGISwapChain3, IDXGISwapChain3_Vtbl, 0x94d99bdb_f1f8_4ab0_b236_7da0170edab1);
impl core::ops::Deref for IDXGISwapChain3 {
    type Target = IDXGISwapChain2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain3, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2);
impl IDXGISwapChain3 {
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentBackBufferIndex)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckColorSpaceSupport)(windows_core::Interface::as_raw(self), colorspace, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetColorSpace1(&self, colorspace: DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorSpace1)(windows_core::Interface::as_raw(self), colorspace) }
    }
    pub unsafe fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResizeBuffers1)(windows_core::Interface::as_raw(self), buffercount, width, height, format, swapchainflags, pcreationnodemask, core::mem::transmute(pppresentqueue)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain3_Vtbl {
    pub base__: IDXGISwapChain2_Vtbl,
    pub GetCurrentBackBufferIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub CheckColorSpaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_COLOR_SPACE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub SetColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT,
    pub ResizeBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, DXGI_FORMAT, u32, *const u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGISwapChain3_Impl: IDXGISwapChain2_Impl {
    fn GetCurrentBackBufferIndex(&self) -> u32;
    fn CheckColorSpaceSupport(&self, colorspace: DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<u32>;
    fn SetColorSpace1(&self, colorspace: DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()>;
    fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGISwapChain3_Vtbl {
    pub const fn new<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::GetCurrentBackBufferIndex(this)
            }
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain3_Impl::CheckColorSpaceSupport(this, core::mem::transmute_copy(&colorspace)) {
                    Ok(ok__) => {
                        pcolorspacesupport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColorSpace1<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::SetColorSpace1(this, core::mem::transmute_copy(&colorspace)).into()
            }
        }
        unsafe extern "system" fn ResizeBuffers1<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::ResizeBuffers1(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&swapchainflags), core::mem::transmute_copy(&pcreationnodemask), core::mem::transmute_copy(&pppresentqueue)).into()
            }
        }
        Self {
            base__: IDXGISwapChain2_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Identity, OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Identity, OFFSET>,
            SetColorSpace1: SetColorSpace1::<Identity, OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGISwapChain2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGISwapChain3 {}
windows_core::imp::define_interface!(IDXGISwapChain4, IDXGISwapChain4_Vtbl, 0x3d585d5a_bd4a_489e_b1f4_3dbcb6452ffb);
impl core::ops::Deref for IDXGISwapChain4 {
    type Target = IDXGISwapChain3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain4, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3);
impl IDXGISwapChain4 {
    pub unsafe fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, pmetadata: Option<&[u8]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHDRMetaData)(windows_core::Interface::as_raw(self), r#type, pmetadata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pmetadata.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain4_Vtbl {
    pub base__: IDXGISwapChain3_Vtbl,
    pub SetHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_HDR_METADATA_TYPE, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGISwapChain4_Impl: IDXGISwapChain3_Impl {
    fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGISwapChain4_Vtbl {
    pub const fn new<Identity: IDXGISwapChain4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHDRMetaData<Identity: IDXGISwapChain4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain4_Impl::SetHDRMetaData(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        Self { base__: IDXGISwapChain3_Vtbl::new::<Identity, OFFSET>(), SetHDRMetaData: SetHDRMetaData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGISwapChain2 as windows_core::Interface>::IID || iid == &<IDXGISwapChain3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGISwapChain4 {}
windows_core::imp::define_interface!(IDXGISwapChainMedia, IDXGISwapChainMedia_Vtbl, 0xdd95b90b_f05f_4f6a_bd65_25bfb264bd84);
windows_core::imp::interface_hierarchy!(IDXGISwapChainMedia, windows_core::IUnknown);
impl IDXGISwapChainMedia {
    pub unsafe fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatisticsMedia)(windows_core::Interface::as_raw(self), pstats as _) }
    }
    pub unsafe fn SetPresentDuration(&self, duration: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPresentDuration)(windows_core::Interface::as_raw(self), duration) }
    }
    pub unsafe fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckPresentDurationSupport)(windows_core::Interface::as_raw(self), desiredpresentduration, pclosestsmallerpresentduration as _, pclosestlargerpresentduration as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChainMedia_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrameStatisticsMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT,
    pub SetPresentDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CheckPresentDurationSupport: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IDXGISwapChainMedia_Impl: windows_core::IUnknownImpl {
    fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::Result<()>;
    fn SetPresentDuration(&self, duration: u32) -> windows_core::Result<()>;
    fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::Result<()>;
}
impl IDXGISwapChainMedia_Vtbl {
    pub const fn new<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrameStatisticsMedia<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::GetFrameStatisticsMedia(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn SetPresentDuration<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::SetPresentDuration(this, core::mem::transmute_copy(&duration)).into()
            }
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::CheckPresentDurationSupport(this, core::mem::transmute_copy(&desiredpresentduration), core::mem::transmute_copy(&pclosestsmallerpresentduration), core::mem::transmute_copy(&pclosestlargerpresentduration)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Identity, OFFSET>,
            SetPresentDuration: SetPresentDuration::<Identity, OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChainMedia as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGISwapChainMedia {}
