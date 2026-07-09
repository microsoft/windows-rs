#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10misc", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
windows_link::link!("d3d10_1.dll" "system" fn D3D10CreateDevice1(padapter : *mut core::ffi::c_void, drivertype : super::d3d10misc::D3D10_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10misc", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("d3d10_1.dll" "system" fn D3D10CreateDeviceAndSwapChain1(padapter : *mut core::ffi::c_void, drivertype : super::d3d10misc::D3D10_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, pswapchaindesc : *const super::dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const D3D10_1_DEFAULT_SAMPLE_MASK: u32 = 4294967295;
pub const D3D10_1_GS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1;
pub const D3D10_1_SDK_VERSION: u32 = 32;
pub const D3D10_1_SHADER_MAJOR_VERSION: u32 = 4;
pub const D3D10_1_SHADER_MINOR_VERSION: u32 = 1;
pub const D3D10_1_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048;
pub const D3D10_1_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256;
pub const D3D10_1_SO_BUFFER_SLOT_COUNT: u32 = 4;
pub const D3D10_1_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1;
pub const D3D10_1_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64;
pub const D3D10_1_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32;
pub const D3D10_1_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8;
pub const D3D10_1_VS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D10_1_VS_OUTPUT_REGISTER_COUNT: u32 = 32;
#[repr(C)]
#[cfg(feature = "Win32_d3d10")]
#[derive(Clone, Copy)]
pub struct D3D10_BLEND_DESC1 {
    pub AlphaToCoverageEnable: windows_sys::core::BOOL,
    pub IndependentBlendEnable: windows_sys::core::BOOL,
    pub RenderTarget: [D3D10_RENDER_TARGET_BLEND_DESC1; 8],
}
#[cfg(feature = "Win32_d3d10")]
impl Default for D3D10_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_CENTER_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -2;
pub type D3D10_FEATURE_LEVEL1 = i32;
pub const D3D10_FEATURE_LEVEL_10_0: D3D10_FEATURE_LEVEL1 = 40960;
pub const D3D10_FEATURE_LEVEL_10_1: D3D10_FEATURE_LEVEL1 = 41216;
pub const D3D10_FEATURE_LEVEL_9_1: D3D10_FEATURE_LEVEL1 = 37120;
pub const D3D10_FEATURE_LEVEL_9_2: D3D10_FEATURE_LEVEL1 = 37376;
pub const D3D10_FEATURE_LEVEL_9_3: D3D10_FEATURE_LEVEL1 = 37632;
#[repr(C)]
#[cfg(feature = "Win32_d3d10")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: windows_sys::core::BOOL,
    pub SrcBlend: super::d3d10::D3D10_BLEND,
    pub DestBlend: super::d3d10::D3D10_BLEND,
    pub BlendOp: super::d3d10::D3D10_BLEND_OP,
    pub SrcBlendAlpha: super::d3d10::D3D10_BLEND,
    pub DestBlendAlpha: super::d3d10::D3D10_BLEND,
    pub BlendOpAlpha: super::d3d10::D3D10_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D10_SRV_DIMENSION1,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub union D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: super::d3d10::D3D10_BUFFER_SRV,
    pub Texture1D: super::d3d10::D3D10_TEX1D_SRV,
    pub Texture1DArray: super::d3d10::D3D10_TEX1D_ARRAY_SRV,
    pub Texture2D: super::d3d10::D3D10_TEX2D_SRV,
    pub Texture2DArray: super::d3d10::D3D10_TEX2D_ARRAY_SRV,
    pub Texture2DMS: super::d3d10::D3D10_TEX2DMS_SRV,
    pub Texture2DMSArray: super::d3d10::D3D10_TEX2DMS_ARRAY_SRV,
    pub Texture3D: super::d3d10::D3D10_TEX3D_SRV,
    pub TextureCube: super::d3d10::D3D10_TEXCUBE_SRV,
    pub TextureCubeArray: D3D10_TEXCUBE_ARRAY_SRV1,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SRV_DIMENSION1 = super::d3dcommon::D3D_SRV_DIMENSION;
pub const D3D10_STANDARD_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -1;
pub type D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXCUBE_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10misc", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
pub type PFN_D3D10_CREATE_DEVICE1 = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::d3d10misc::D3D10_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10misc", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1 = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::d3d10misc::D3D10_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut super::dxgi::DXGI_SWAP_CHAIN_DESC, param7: *mut *mut core::ffi::c_void, param8: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
