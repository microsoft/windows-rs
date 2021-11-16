#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10CompileEffectFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, psrcfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: super::Direct3D::ID3DInclude, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut super::Direct3D::ID3DBlob, pperrors: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10CompileShader(psrcdata: super::super::Foundation::PSTR, srcdatasize: usize, pfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: super::Direct3D::ID3DInclude, pfunctionname: super::super::Foundation::PSTR, pprofile: super::super::Foundation::PSTR, flags: u32, ppshader: *mut super::Direct3D::ID3DBlob, pperrormsgs: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10CreateBlob(numbytes: usize, ppbuffer: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D10CreateDevice(padapter: super::Dxgi::IDXGIAdapter, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, sdkversion: u32, ppdevice: *mut ID3D10Device) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D10CreateDevice1(padapter: super::Dxgi::IDXGIAdapter, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, ppdevice: *mut ID3D10Device1) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D10CreateDeviceAndSwapChain(padapter: super::Dxgi::IDXGIAdapter, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut super::Dxgi::IDXGISwapChain, ppdevice: *mut ID3D10Device) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D10CreateDeviceAndSwapChain1(padapter: super::Dxgi::IDXGIAdapter, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut super::Dxgi::IDXGISwapChain, ppdevice: *mut ID3D10Device1) -> ::windows_sys::core::HRESULT;
    pub fn D3D10CreateEffectFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: ID3D10Device, peffectpool: ID3D10EffectPool, ppeffect: *mut ID3D10Effect) -> ::windows_sys::core::HRESULT;
    pub fn D3D10CreateEffectPoolFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: ID3D10Device, ppeffectpool: *mut ID3D10EffectPool) -> ::windows_sys::core::HRESULT;
    pub fn D3D10CreateStateBlock(pdevice: ID3D10Device, pstateblockmask: *const D3D10_STATE_BLOCK_MASK, ppstateblock: *mut ID3D10StateBlock) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10DisassembleEffect(peffect: ID3D10Effect, enablecolorcode: super::super::Foundation::BOOL, ppdisassembly: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10DisassembleShader(pshader: *const ::core::ffi::c_void, bytecodelength: usize, enablecolorcode: super::super::Foundation::BOOL, pcomments: super::super::Foundation::PSTR, ppdisassembly: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetGeometryShaderProfile(pdevice: ID3D10Device) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetInputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetPixelShaderProfile(pdevice: ID3D10Device) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetShaderDebugInfo(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppdebuginfo: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetVertexShaderProfile(pdevice: ID3D10Device) -> super::super::Foundation::PSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10PreprocessShader(psrcdata: super::super::Foundation::PSTR, srcdatasize: usize, pfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: super::Direct3D::ID3DInclude, ppshadertext: *mut super::Direct3D::ID3DBlob, pperrormsgs: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    pub fn D3D10ReflectShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppreflector: *mut ID3D10ShaderReflection) -> ::windows_sys::core::HRESULT;
    pub fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_sys::core::HRESULT;
    pub fn D3D10StateBlockMaskDisableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_sys::core::HRESULT;
    pub fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows_sys::core::HRESULT;
    pub fn D3D10StateBlockMaskEnableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_sys::core::HRESULT;
    pub fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> super::super::Foundation::BOOL;
    pub fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_sys::core::HRESULT;
    pub fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_sys::core::HRESULT;
}
pub const D3D10_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
pub const D3D10_1_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D10_1_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D10_1_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D10_1_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16u32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_1_SHADER_MAJOR_VERSION: u32 = 4u32;
pub const D3D10_1_SHADER_MINOR_VERSION: u32 = 1u32;
pub const D3D10_1_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D10_1_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
pub const D3D10_1_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D10_1_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1u32;
pub const D3D10_1_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64u32;
pub const D3D10_1_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
pub const D3D10_1_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D10_1_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_1_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
pub const D3D10_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
pub const D3D10_ALL_RESOURCES_BOUND: u32 = 2097152u32;
pub const D3D10_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
pub const D3D10_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
pub const D3D10_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
pub const D3D10_ASYNC_GETDATA_DONOTFLUSH: i32 = 1i32;
pub const D3D10_BIND_VERTEX_BUFFER: i32 = 1i32;
pub const D3D10_BIND_INDEX_BUFFER: i32 = 2i32;
pub const D3D10_BIND_CONSTANT_BUFFER: i32 = 4i32;
pub const D3D10_BIND_SHADER_RESOURCE: i32 = 8i32;
pub const D3D10_BIND_STREAM_OUTPUT: i32 = 16i32;
pub const D3D10_BIND_RENDER_TARGET: i32 = 32i32;
pub const D3D10_BIND_DEPTH_STENCIL: i32 = 64i32;
pub const D3D10_BLEND_ZERO: i32 = 1i32;
pub const D3D10_BLEND_ONE: i32 = 2i32;
pub const D3D10_BLEND_SRC_COLOR: i32 = 3i32;
pub const D3D10_BLEND_INV_SRC_COLOR: i32 = 4i32;
pub const D3D10_BLEND_SRC_ALPHA: i32 = 5i32;
pub const D3D10_BLEND_INV_SRC_ALPHA: i32 = 6i32;
pub const D3D10_BLEND_DEST_ALPHA: i32 = 7i32;
pub const D3D10_BLEND_INV_DEST_ALPHA: i32 = 8i32;
pub const D3D10_BLEND_DEST_COLOR: i32 = 9i32;
pub const D3D10_BLEND_INV_DEST_COLOR: i32 = 10i32;
pub const D3D10_BLEND_SRC_ALPHA_SAT: i32 = 11i32;
pub const D3D10_BLEND_BLEND_FACTOR: i32 = 14i32;
pub const D3D10_BLEND_INV_BLEND_FACTOR: i32 = 15i32;
pub const D3D10_BLEND_SRC1_COLOR: i32 = 16i32;
pub const D3D10_BLEND_INV_SRC1_COLOR: i32 = 17i32;
pub const D3D10_BLEND_SRC1_ALPHA: i32 = 18i32;
pub const D3D10_BLEND_INV_SRC1_ALPHA: i32 = 19i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_BLEND_DESC {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub BlendEnable: [super::super::Foundation::BOOL; 8],
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: [u8; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_BLEND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_BLEND_DESC1 {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D10_RENDER_TARGET_BLEND_DESC1; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_BLEND_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_BLEND_OP_ADD: i32 = 1i32;
pub const D3D10_BLEND_OP_SUBTRACT: i32 = 2i32;
pub const D3D10_BLEND_OP_REV_SUBTRACT: i32 = 3i32;
pub const D3D10_BLEND_OP_MIN: i32 = 4i32;
pub const D3D10_BLEND_OP_MAX: i32 = 5i32;
#[repr(C)]
pub struct D3D10_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl ::core::marker::Copy for D3D10_BOX {}
impl ::core::clone::Clone for D3D10_BOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_DESC {}
impl ::core::clone::Clone for D3D10_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_BUFFER_RTV {
    pub Anonymous1: D3D10_BUFFER_RTV_0,
    pub Anonymous2: D3D10_BUFFER_RTV_1,
}
impl ::core::marker::Copy for D3D10_BUFFER_RTV {}
impl ::core::clone::Clone for D3D10_BUFFER_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D10_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_RTV_0 {}
impl ::core::clone::Clone for D3D10_BUFFER_RTV_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D10_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_RTV_1 {}
impl ::core::clone::Clone for D3D10_BUFFER_RTV_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_BUFFER_SRV {
    pub Anonymous1: D3D10_BUFFER_SRV_0,
    pub Anonymous2: D3D10_BUFFER_SRV_1,
}
impl ::core::marker::Copy for D3D10_BUFFER_SRV {}
impl ::core::clone::Clone for D3D10_BUFFER_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D10_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_SRV_0 {}
impl ::core::clone::Clone for D3D10_BUFFER_SRV_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D10_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl ::core::marker::Copy for D3D10_BUFFER_SRV_1 {}
impl ::core::clone::Clone for D3D10_BUFFER_SRV_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_CLEAR_DEPTH: i32 = 1i32;
pub const D3D10_CLEAR_STENCIL: i32 = 2i32;
pub const D3D10_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D10_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
pub const D3D10_COLOR_WRITE_ENABLE_RED: i32 = 1i32;
pub const D3D10_COLOR_WRITE_ENABLE_GREEN: i32 = 2i32;
pub const D3D10_COLOR_WRITE_ENABLE_BLUE: i32 = 4i32;
pub const D3D10_COLOR_WRITE_ENABLE_ALPHA: i32 = 8i32;
pub const D3D10_COLOR_WRITE_ENABLE_ALL: i32 = 15i32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
pub const D3D10_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
pub const D3D10_COMPARISON_FILTERING_BIT: u32 = 128u32;
pub const D3D10_COMPARISON_NEVER: i32 = 1i32;
pub const D3D10_COMPARISON_LESS: i32 = 2i32;
pub const D3D10_COMPARISON_EQUAL: i32 = 3i32;
pub const D3D10_COMPARISON_LESS_EQUAL: i32 = 4i32;
pub const D3D10_COMPARISON_GREATER: i32 = 5i32;
pub const D3D10_COMPARISON_NOT_EQUAL: i32 = 6i32;
pub const D3D10_COMPARISON_GREATER_EQUAL: i32 = 7i32;
pub const D3D10_COMPARISON_ALWAYS: i32 = 8i32;
pub const D3D10_COUNTER_GPU_IDLE: i32 = 0i32;
pub const D3D10_COUNTER_VERTEX_PROCESSING: i32 = 1i32;
pub const D3D10_COUNTER_GEOMETRY_PROCESSING: i32 = 2i32;
pub const D3D10_COUNTER_PIXEL_PROCESSING: i32 = 3i32;
pub const D3D10_COUNTER_OTHER_GPU_PROCESSING: i32 = 4i32;
pub const D3D10_COUNTER_HOST_ADAPTER_BANDWIDTH_UTILIZATION: i32 = 5i32;
pub const D3D10_COUNTER_LOCAL_VIDMEM_BANDWIDTH_UTILIZATION: i32 = 6i32;
pub const D3D10_COUNTER_VERTEX_THROUGHPUT_UTILIZATION: i32 = 7i32;
pub const D3D10_COUNTER_TRIANGLE_SETUP_THROUGHPUT_UTILIZATION: i32 = 8i32;
pub const D3D10_COUNTER_FILLRATE_THROUGHPUT_UTILIZATION: i32 = 9i32;
pub const D3D10_COUNTER_VS_MEMORY_LIMITED: i32 = 10i32;
pub const D3D10_COUNTER_VS_COMPUTATION_LIMITED: i32 = 11i32;
pub const D3D10_COUNTER_GS_MEMORY_LIMITED: i32 = 12i32;
pub const D3D10_COUNTER_GS_COMPUTATION_LIMITED: i32 = 13i32;
pub const D3D10_COUNTER_PS_MEMORY_LIMITED: i32 = 14i32;
pub const D3D10_COUNTER_PS_COMPUTATION_LIMITED: i32 = 15i32;
pub const D3D10_COUNTER_POST_TRANSFORM_CACHE_HIT_RATE: i32 = 16i32;
pub const D3D10_COUNTER_TEXTURE_CACHE_HIT_RATE: i32 = 17i32;
pub const D3D10_COUNTER_DEVICE_DEPENDENT_0: i32 = 1073741824i32;
#[repr(C)]
pub struct D3D10_COUNTER_DESC {
    pub Counter: D3D10_COUNTER,
    pub MiscFlags: u32,
}
impl ::core::marker::Copy for D3D10_COUNTER_DESC {}
impl ::core::clone::Clone for D3D10_COUNTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D10_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
impl ::core::marker::Copy for D3D10_COUNTER_INFO {}
impl ::core::clone::Clone for D3D10_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_COUNTER_TYPE_FLOAT32: i32 = 0i32;
pub const D3D10_COUNTER_TYPE_UINT16: i32 = 1i32;
pub const D3D10_COUNTER_TYPE_UINT32: i32 = 2i32;
pub const D3D10_COUNTER_TYPE_UINT64: i32 = 3i32;
pub const D3D10_CPU_ACCESS_WRITE: i32 = 65536i32;
pub const D3D10_CPU_ACCESS_READ: i32 = 131072i32;
pub const D3D10_CREATE_DEVICE_SINGLETHREADED: i32 = 1i32;
pub const D3D10_CREATE_DEVICE_DEBUG: i32 = 2i32;
pub const D3D10_CREATE_DEVICE_SWITCH_TO_REF: i32 = 4i32;
pub const D3D10_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: i32 = 8i32;
pub const D3D10_CREATE_DEVICE_ALLOW_NULL_FROM_MAP: i32 = 16i32;
pub const D3D10_CREATE_DEVICE_BGRA_SUPPORT: i32 = 32i32;
pub const D3D10_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: i32 = 128i32;
pub const D3D10_CREATE_DEVICE_STRICT_VALIDATION: i32 = 512i32;
pub const D3D10_CREATE_DEVICE_DEBUGGABLE: i32 = 1024i32;
pub const D3D10_CULL_NONE: i32 = 1i32;
pub const D3D10_CULL_FRONT: i32 = 2i32;
pub const D3D10_CULL_BACK: i32 = 3i32;
pub const D3D10_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2u32;
pub const D3D10_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1u32;
pub const D3D10_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4u32;
pub const D3D10_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
pub const D3D10_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
pub const D3D10_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D10_DEFAULT_DEPTH_BIAS: u32 = 0u32;
pub const D3D10_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
pub const D3D10_DEFAULT_MAX_ANISOTROPY: f32 = 16f32;
pub const D3D10_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
pub const D3D10_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
pub const D3D10_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D10_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
pub const D3D10_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
pub const D3D10_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
pub const D3D10_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
pub const D3D10_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
pub const D3D10_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
pub const D3D10_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
pub const D3D10_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
pub const D3D10_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
#[repr(C)]
pub struct D3D10_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D10_STENCIL_OP,
    pub StencilDepthFailOp: D3D10_STENCIL_OP,
    pub StencilPassOp: D3D10_STENCIL_OP,
    pub StencilFunc: D3D10_COMPARISON_FUNC,
}
impl ::core::marker::Copy for D3D10_DEPTH_STENCILOP_DESC {}
impl ::core::clone::Clone for D3D10_DEPTH_STENCILOP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_DEPTH_STENCIL_DESC {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D10_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D10_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D10_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D10_DEPTH_STENCILOP_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D10_DSV_DIMENSION,
    pub Anonymous: D3D10_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_DEPTH_STENCIL_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_DEPTH_STENCIL_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D10_TEX1D_DSV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D10_TEX2D_DSV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D10_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_DEPTH_WRITE_MASK_ZERO: i32 = 0i32;
pub const D3D10_DEPTH_WRITE_MASK_ALL: i32 = 1i32;
pub const D3D10_DST_SO_BUFFERS: i32 = 1i32;
pub const D3D10_DST_OM_RENDER_TARGETS: i32 = 2i32;
pub const D3D10_DST_OM_DEPTH_STENCIL_STATE: i32 = 3i32;
pub const D3D10_DST_OM_BLEND_STATE: i32 = 4i32;
pub const D3D10_DST_VS: i32 = 5i32;
pub const D3D10_DST_VS_SAMPLERS: i32 = 6i32;
pub const D3D10_DST_VS_SHADER_RESOURCES: i32 = 7i32;
pub const D3D10_DST_VS_CONSTANT_BUFFERS: i32 = 8i32;
pub const D3D10_DST_GS: i32 = 9i32;
pub const D3D10_DST_GS_SAMPLERS: i32 = 10i32;
pub const D3D10_DST_GS_SHADER_RESOURCES: i32 = 11i32;
pub const D3D10_DST_GS_CONSTANT_BUFFERS: i32 = 12i32;
pub const D3D10_DST_PS: i32 = 13i32;
pub const D3D10_DST_PS_SAMPLERS: i32 = 14i32;
pub const D3D10_DST_PS_SHADER_RESOURCES: i32 = 15i32;
pub const D3D10_DST_PS_CONSTANT_BUFFERS: i32 = 16i32;
pub const D3D10_DST_IA_VERTEX_BUFFERS: i32 = 17i32;
pub const D3D10_DST_IA_INDEX_BUFFER: i32 = 18i32;
pub const D3D10_DST_IA_INPUT_LAYOUT: i32 = 19i32;
pub const D3D10_DST_IA_PRIMITIVE_TOPOLOGY: i32 = 20i32;
pub const D3D10_DST_RS_VIEWPORTS: i32 = 21i32;
pub const D3D10_DST_RS_SCISSOR_RECTS: i32 = 22i32;
pub const D3D10_DST_RS_RASTERIZER_STATE: i32 = 23i32;
pub const D3D10_DST_PREDICATION: i32 = 24i32;
pub const D3D10_DRIVER_TYPE_HARDWARE: i32 = 0i32;
pub const D3D10_DRIVER_TYPE_REFERENCE: i32 = 1i32;
pub const D3D10_DRIVER_TYPE_NULL: i32 = 2i32;
pub const D3D10_DRIVER_TYPE_SOFTWARE: i32 = 3i32;
pub const D3D10_DRIVER_TYPE_WARP: i32 = 5i32;
pub const D3D10_DSV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D10_DSV_DIMENSION_TEXTURE1D: i32 = 1i32;
pub const D3D10_DSV_DIMENSION_TEXTURE1DARRAY: i32 = 2i32;
pub const D3D10_DSV_DIMENSION_TEXTURE2D: i32 = 3i32;
pub const D3D10_DSV_DIMENSION_TEXTURE2DARRAY: i32 = 4i32;
pub const D3D10_DSV_DIMENSION_TEXTURE2DMS: i32 = 5i32;
pub const D3D10_DSV_DIMENSION_TEXTURE2DMSARRAY: i32 = 6i32;
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: u32 = 2u32;
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_EFFECT_DESC {
    pub IsChildEffect: super::super::Foundation::BOOL,
    pub ConstantBuffers: u32,
    pub SharedConstantBuffers: u32,
    pub GlobalVariables: u32,
    pub SharedGlobalVariables: u32,
    pub Techniques: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_EFFECT_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_EFFECT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_EFFECT_SHADER_DESC {
    pub pInputSignature: *mut u8,
    pub IsInline: super::super::Foundation::BOOL,
    pub pBytecode: *mut u8,
    pub BytecodeLength: u32,
    pub SODecl: super::super::Foundation::PSTR,
    pub NumInputSignatureEntries: u32,
    pub NumOutputSignatureEntries: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_EFFECT_SHADER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_EFFECT_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_EFFECT_SINGLE_THREADED: u32 = 8u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_EFFECT_TYPE_DESC {
    pub TypeName: super::super::Foundation::PSTR,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Elements: u32,
    pub Members: u32,
    pub Rows: u32,
    pub Columns: u32,
    pub PackedSize: u32,
    pub UnpackedSize: u32,
    pub Stride: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_EFFECT_TYPE_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_EFFECT_TYPE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_EFFECT_VARIABLE_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Semantic: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub Annotations: u32,
    pub BufferOffset: u32,
    pub ExplicitBindPoint: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_EFFECT_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_EFFECT_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4u32;
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1u32;
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
pub const D3D10_FEATURE_LEVEL_10_0: i32 = 40960i32;
pub const D3D10_FEATURE_LEVEL_10_1: i32 = 41216i32;
pub const D3D10_FEATURE_LEVEL_9_1: i32 = 37120i32;
pub const D3D10_FEATURE_LEVEL_9_2: i32 = 37376i32;
pub const D3D10_FEATURE_LEVEL_9_3: i32 = 37632i32;
pub const D3D10_FILL_WIREFRAME: i32 = 2i32;
pub const D3D10_FILL_SOLID: i32 = 3i32;
pub const D3D10_FILTER_MIN_MAG_MIP_POINT: i32 = 0i32;
pub const D3D10_FILTER_MIN_MAG_POINT_MIP_LINEAR: i32 = 1i32;
pub const D3D10_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 4i32;
pub const D3D10_FILTER_MIN_POINT_MAG_MIP_LINEAR: i32 = 5i32;
pub const D3D10_FILTER_MIN_LINEAR_MAG_MIP_POINT: i32 = 16i32;
pub const D3D10_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 17i32;
pub const D3D10_FILTER_MIN_MAG_LINEAR_MIP_POINT: i32 = 20i32;
pub const D3D10_FILTER_MIN_MAG_MIP_LINEAR: i32 = 21i32;
pub const D3D10_FILTER_ANISOTROPIC: i32 = 85i32;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_POINT: i32 = 128i32;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: i32 = 129i32;
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 132i32;
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: i32 = 133i32;
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: i32 = 144i32;
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 145i32;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: i32 = 148i32;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: i32 = 149i32;
pub const D3D10_FILTER_COMPARISON_ANISOTROPIC: i32 = 213i32;
pub const D3D10_FILTER_TEXT_1BIT: i32 = -2147483648i32;
pub const D3D10_FILTER_TYPE_POINT: i32 = 0i32;
pub const D3D10_FILTER_TYPE_LINEAR: i32 = 1i32;
pub const D3D10_FILTER_TYPE_MASK: u32 = 3u32;
pub const D3D10_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D10_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
pub const D3D10_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
pub const D3D10_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
pub const D3D10_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
pub const D3D10_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
pub const D3D10_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
pub const D3D10_FORMAT_SUPPORT_BUFFER: i32 = 1i32;
pub const D3D10_FORMAT_SUPPORT_IA_VERTEX_BUFFER: i32 = 2i32;
pub const D3D10_FORMAT_SUPPORT_IA_INDEX_BUFFER: i32 = 4i32;
pub const D3D10_FORMAT_SUPPORT_SO_BUFFER: i32 = 8i32;
pub const D3D10_FORMAT_SUPPORT_TEXTURE1D: i32 = 16i32;
pub const D3D10_FORMAT_SUPPORT_TEXTURE2D: i32 = 32i32;
pub const D3D10_FORMAT_SUPPORT_TEXTURE3D: i32 = 64i32;
pub const D3D10_FORMAT_SUPPORT_TEXTURECUBE: i32 = 128i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_LOAD: i32 = 256i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE: i32 = 512i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON: i32 = 1024i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT: i32 = 2048i32;
pub const D3D10_FORMAT_SUPPORT_MIP: i32 = 4096i32;
pub const D3D10_FORMAT_SUPPORT_MIP_AUTOGEN: i32 = 8192i32;
pub const D3D10_FORMAT_SUPPORT_RENDER_TARGET: i32 = 16384i32;
pub const D3D10_FORMAT_SUPPORT_BLENDABLE: i32 = 32768i32;
pub const D3D10_FORMAT_SUPPORT_DEPTH_STENCIL: i32 = 65536i32;
pub const D3D10_FORMAT_SUPPORT_CPU_LOCKABLE: i32 = 131072i32;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE: i32 = 262144i32;
pub const D3D10_FORMAT_SUPPORT_DISPLAY: i32 = 524288i32;
pub const D3D10_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT: i32 = 1048576i32;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET: i32 = 2097152i32;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_LOAD: i32 = 4194304i32;
pub const D3D10_FORMAT_SUPPORT_SHADER_GATHER: i32 = 8388608i32;
pub const D3D10_FORMAT_SUPPORT_BACK_BUFFER_CAST: i32 = 16777216i32;
pub const D3D10_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
pub const D3D10_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
pub const D3D10_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
pub const D3D10_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_INPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_GS_INPUT_REGISTER_VERTICES: u32 = 6u32;
pub const D3D10_GS_OUTPUT_ELEMENTS: u32 = 32u32;
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D10_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
pub const D3D10_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D10_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
pub const D3D10_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
pub const D3D10_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16u32;
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 64u32;
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
pub struct D3D10_INFO_QUEUE_FILTER {
    pub AllowList: D3D10_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D10_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for D3D10_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for D3D10_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D10_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D10_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D10_MESSAGE_ID,
}
impl ::core::marker::Copy for D3D10_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for D3D10_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_INPUT_PER_VERTEX_DATA: i32 = 0i32;
pub const D3D10_INPUT_PER_INSTANCE_DATA: i32 = 1i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D10_INPUT_ELEMENT_DESC {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D10_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_INPUT_ELEMENT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D10_LINEAR_GAMMA: f32 = 1f32;
pub const D3D10_MAG_FILTER_SHIFT: u32 = 2u32;
pub const D3D10_MAP_READ: i32 = 1i32;
pub const D3D10_MAP_WRITE: i32 = 2i32;
pub const D3D10_MAP_READ_WRITE: i32 = 3i32;
pub const D3D10_MAP_WRITE_DISCARD: i32 = 4i32;
pub const D3D10_MAP_WRITE_NO_OVERWRITE: i32 = 5i32;
#[repr(C)]
pub struct D3D10_MAPPED_TEXTURE2D {
    pub pData: *mut ::core::ffi::c_void,
    pub RowPitch: u32,
}
impl ::core::marker::Copy for D3D10_MAPPED_TEXTURE2D {}
impl ::core::clone::Clone for D3D10_MAPPED_TEXTURE2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_MAPPED_TEXTURE3D {
    pub pData: *mut ::core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl ::core::marker::Copy for D3D10_MAPPED_TEXTURE3D {}
impl ::core::clone::Clone for D3D10_MAPPED_TEXTURE3D {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_MAP_FLAG_DO_NOT_WAIT: i32 = 1048576i32;
pub const D3D10_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
pub const D3D10_MAX_DEPTH: f32 = 1f32;
pub const D3D10_MAX_MAXANISOTROPY: u32 = 16u32;
pub const D3D10_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
pub const D3D10_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
pub const D3D10_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
#[repr(C)]
pub struct D3D10_MESSAGE {
    pub Category: D3D10_MESSAGE_CATEGORY,
    pub Severity: D3D10_MESSAGE_SEVERITY,
    pub ID: D3D10_MESSAGE_ID,
    pub pDescription: *mut u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for D3D10_MESSAGE {}
impl ::core::clone::Clone for D3D10_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_MESSAGE_CATEGORY_APPLICATION_DEFINED: i32 = 0i32;
pub const D3D10_MESSAGE_CATEGORY_MISCELLANEOUS: i32 = 1i32;
pub const D3D10_MESSAGE_CATEGORY_INITIALIZATION: i32 = 2i32;
pub const D3D10_MESSAGE_CATEGORY_CLEANUP: i32 = 3i32;
pub const D3D10_MESSAGE_CATEGORY_COMPILATION: i32 = 4i32;
pub const D3D10_MESSAGE_CATEGORY_STATE_CREATION: i32 = 5i32;
pub const D3D10_MESSAGE_CATEGORY_STATE_SETTING: i32 = 6i32;
pub const D3D10_MESSAGE_CATEGORY_STATE_GETTING: i32 = 7i32;
pub const D3D10_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: i32 = 8i32;
pub const D3D10_MESSAGE_CATEGORY_EXECUTION: i32 = 9i32;
pub const D3D10_MESSAGE_CATEGORY_SHADER: i32 = 10i32;
pub const D3D10_MESSAGE_ID_UNKNOWN: i32 = 0i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_HAZARD: i32 = 1i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_HAZARD: i32 = 2i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_HAZARD: i32 = 3i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_HAZARD: i32 = 4i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_HAZARD: i32 = 5i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_HAZARD: i32 = 6i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_HAZARD: i32 = 7i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_HAZARD: i32 = 8i32;
pub const D3D10_MESSAGE_ID_DEVICE_OMSETRENDERTARGETS_HAZARD: i32 = 9i32;
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_HAZARD: i32 = 10i32;
pub const D3D10_MESSAGE_ID_STRING_FROM_APPLICATION: i32 = 11i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_THIS: i32 = 12i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER1: i32 = 13i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER2: i32 = 14i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER3: i32 = 15i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER4: i32 = 16i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER5: i32 = 17i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER6: i32 = 18i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER7: i32 = 19i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER8: i32 = 20i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER9: i32 = 21i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER10: i32 = 22i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER11: i32 = 23i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER12: i32 = 24i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER13: i32 = 25i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER14: i32 = 26i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER15: i32 = 27i32;
pub const D3D10_MESSAGE_ID_CORRUPTED_MULTITHREADING: i32 = 28i32;
pub const D3D10_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: i32 = 29i32;
pub const D3D10_MESSAGE_ID_IASETINPUTLAYOUT_UNBINDDELETINGOBJECT: i32 = 30i32;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_UNBINDDELETINGOBJECT: i32 = 31i32;
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_UNBINDDELETINGOBJECT: i32 = 32i32;
pub const D3D10_MESSAGE_ID_VSSETSHADER_UNBINDDELETINGOBJECT: i32 = 33i32;
pub const D3D10_MESSAGE_ID_VSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 34i32;
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 35i32;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 36i32;
pub const D3D10_MESSAGE_ID_GSSETSHADER_UNBINDDELETINGOBJECT: i32 = 37i32;
pub const D3D10_MESSAGE_ID_GSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 38i32;
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 39i32;
pub const D3D10_MESSAGE_ID_GSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 40i32;
pub const D3D10_MESSAGE_ID_SOSETTARGETS_UNBINDDELETINGOBJECT: i32 = 41i32;
pub const D3D10_MESSAGE_ID_PSSETSHADER_UNBINDDELETINGOBJECT: i32 = 42i32;
pub const D3D10_MESSAGE_ID_PSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 43i32;
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 44i32;
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 45i32;
pub const D3D10_MESSAGE_ID_RSSETSTATE_UNBINDDELETINGOBJECT: i32 = 46i32;
pub const D3D10_MESSAGE_ID_OMSETBLENDSTATE_UNBINDDELETINGOBJECT: i32 = 47i32;
pub const D3D10_MESSAGE_ID_OMSETDEPTHSTENCILSTATE_UNBINDDELETINGOBJECT: i32 = 48i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_UNBINDDELETINGOBJECT: i32 = 49i32;
pub const D3D10_MESSAGE_ID_SETPREDICATION_UNBINDDELETINGOBJECT: i32 = 50i32;
pub const D3D10_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: i32 = 51i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: i32 = 52i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDIUNKNOWN: i32 = 53i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFLAGS: i32 = 54i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: i32 = 55i32;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: i32 = 56i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDFORMAT: i32 = 57i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDSAMPLES: i32 = 58i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDUSAGE: i32 = 59i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDBINDFLAGS: i32 = 60i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDCPUACCESSFLAGS: i32 = 61i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDMISCFLAGS: i32 = 62i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCPUACCESSFLAGS: i32 = 63i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDBINDFLAGS: i32 = 64i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDINITIALDATA: i32 = 65i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDDIMENSIONS: i32 = 66i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMIPLEVELS: i32 = 67i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMISCFLAGS: i32 = 68i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDARG_RETURN: i32 = 69i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_OUTOFMEMORY_RETURN: i32 = 70i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_NULLDESC: i32 = 71i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCONSTANTBUFFERBINDINGS: i32 = 72i32;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_LARGEALLOCATION: i32 = 73i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDFORMAT: i32 = 74i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNSUPPORTEDFORMAT: i32 = 75i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDSAMPLES: i32 = 76i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDUSAGE: i32 = 77i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDBINDFLAGS: i32 = 78i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDCPUACCESSFLAGS: i32 = 79i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDMISCFLAGS: i32 = 80i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDCPUACCESSFLAGS: i32 = 81i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDBINDFLAGS: i32 = 82i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDINITIALDATA: i32 = 83i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDDIMENSIONS: i32 = 84i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMIPLEVELS: i32 = 85i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMISCFLAGS: i32 = 86i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDARG_RETURN: i32 = 87i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_OUTOFMEMORY_RETURN: i32 = 88i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_NULLDESC: i32 = 89i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_LARGEALLOCATION: i32 = 90i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDFORMAT: i32 = 91i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNSUPPORTEDFORMAT: i32 = 92i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDSAMPLES: i32 = 93i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDUSAGE: i32 = 94i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDBINDFLAGS: i32 = 95i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDCPUACCESSFLAGS: i32 = 96i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDMISCFLAGS: i32 = 97i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDCPUACCESSFLAGS: i32 = 98i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDBINDFLAGS: i32 = 99i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDINITIALDATA: i32 = 100i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDDIMENSIONS: i32 = 101i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMIPLEVELS: i32 = 102i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMISCFLAGS: i32 = 103i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDARG_RETURN: i32 = 104i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_OUTOFMEMORY_RETURN: i32 = 105i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_NULLDESC: i32 = 106i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_LARGEALLOCATION: i32 = 107i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDFORMAT: i32 = 108i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNSUPPORTEDFORMAT: i32 = 109i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDSAMPLES: i32 = 110i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDUSAGE: i32 = 111i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDBINDFLAGS: i32 = 112i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDCPUACCESSFLAGS: i32 = 113i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDMISCFLAGS: i32 = 114i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDCPUACCESSFLAGS: i32 = 115i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDBINDFLAGS: i32 = 116i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDINITIALDATA: i32 = 117i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDDIMENSIONS: i32 = 118i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMIPLEVELS: i32 = 119i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMISCFLAGS: i32 = 120i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDARG_RETURN: i32 = 121i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_OUTOFMEMORY_RETURN: i32 = 122i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_NULLDESC: i32 = 123i32;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_LARGEALLOCATION: i32 = 124i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: i32 = 125i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: i32 = 126i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: i32 = 127i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: i32 = 128i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: i32 = 129i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_TOOMANYOBJECTS: i32 = 130i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDARG_RETURN: i32 = 131i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_OUTOFMEMORY_RETURN: i32 = 132i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: i32 = 133i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: i32 = 134i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: i32 = 135i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: i32 = 136i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: i32 = 137i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: i32 = 138i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_TOOMANYOBJECTS: i32 = 139i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDARG_RETURN: i32 = 140i32;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_OUTOFMEMORY_RETURN: i32 = 141i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: i32 = 142i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: i32 = 143i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: i32 = 144i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: i32 = 145i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: i32 = 146i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_TOOMANYOBJECTS: i32 = 147i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDARG_RETURN: i32 = 148i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_OUTOFMEMORY_RETURN: i32 = 149i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: i32 = 150i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: i32 = 151i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: i32 = 152i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: i32 = 153i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: i32 = 154i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: i32 = 155i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: i32 = 156i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: i32 = 157i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: i32 = 158i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: i32 = 159i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: i32 = 160i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: i32 = 161i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: i32 = 162i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: i32 = 163i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLDESC: i32 = 164i32;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: i32 = 165i32;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: i32 = 166i32;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: i32 = 167i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: i32 = 168i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: i32 = 169i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: i32 = 170i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: i32 = 171i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: i32 = 172i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: i32 = 173i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: i32 = 174i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: i32 = 175i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDDECL: i32 = 176i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_EXPECTEDDECL: i32 = 177i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: i32 = 178i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: i32 = 179i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: i32 = 180i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: i32 = 181i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: i32 = 182i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: i32 = 183i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: i32 = 184i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: i32 = 185i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: i32 = 186i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: i32 = 187i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: i32 = 188i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: i32 = 189i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: i32 = 190i32;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: i32 = 191i32;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: i32 = 192i32;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: i32 = 193i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: i32 = 194i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: i32 = 195i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: i32 = 196i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: i32 = 197i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_TOOMANYOBJECTS: i32 = 198i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_NULLDESC: i32 = 199i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: i32 = 200i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: i32 = 201i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: i32 = 202i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: i32 = 203i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: i32 = 204i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: i32 = 205i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: i32 = 206i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: i32 = 207i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: i32 = 208i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: i32 = 209i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_TOOMANYOBJECTS: i32 = 210i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_NULLDESC: i32 = 211i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: i32 = 212i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: i32 = 213i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: i32 = 214i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: i32 = 215i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: i32 = 216i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: i32 = 217i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: i32 = 218i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_TOOMANYOBJECTS: i32 = 219i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NULLDESC: i32 = 220i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDFILTER: i32 = 221i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSU: i32 = 222i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSV: i32 = 223i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSW: i32 = 224i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMIPLODBIAS: i32 = 225i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXANISOTROPY: i32 = 226i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDCOMPARISONFUNC: i32 = 227i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMINLOD: i32 = 228i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXLOD: i32 = 229i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_TOOMANYOBJECTS: i32 = 230i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NULLDESC: i32 = 231i32;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDQUERY: i32 = 232i32;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDMISCFLAGS: i32 = 233i32;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_UNEXPECTEDMISCFLAG: i32 = 234i32;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_NULLDESC: i32 = 235i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNRECOGNIZED: i32 = 236i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNDEFINED: i32 = 237i32;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_INVALIDBUFFER: i32 = 238i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_OFFSET_TOO_LARGE: i32 = 239i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_BUFFERS_EMPTY: i32 = 240i32;
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_INVALIDBUFFER: i32 = 241i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_FORMAT_INVALID: i32 = 242i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_TOO_LARGE: i32 = 243i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_UNALIGNED: i32 = 244i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 245i32;
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 246i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 247i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 248i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 249i32;
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 250i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 251i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 252i32;
pub const D3D10_MESSAGE_ID_SOSETTARGETS_INVALIDBUFFER: i32 = 253i32;
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_OFFSET_UNALIGNED: i32 = 254i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 255i32;
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 256i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 257i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 258i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_INVALIDVIEWPORT: i32 = 259i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_INVALIDSCISSOR: i32 = 260i32;
pub const D3D10_MESSAGE_ID_CLEARRENDERTARGETVIEW_DENORMFLUSH: i32 = 261i32;
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DENORMFLUSH: i32 = 262i32;
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: i32 = 263i32;
pub const D3D10_MESSAGE_ID_DEVICE_IAGETVERTEXBUFFERS_BUFFERS_EMPTY: i32 = 264i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 265i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 266i32;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 267i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 268i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 269i32;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 270i32;
pub const D3D10_MESSAGE_ID_DEVICE_SOGETTARGETS_BUFFERS_EMPTY: i32 = 271i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 272i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 273i32;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 274i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSGETVIEWPORTS_VIEWPORTS_EMPTY: i32 = 275i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSGETSCISSORRECTS_RECTS_EMPTY: i32 = 276i32;
pub const D3D10_MESSAGE_ID_DEVICE_GENERATEMIPS_RESOURCE_INVALID: i32 = 277i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSUBRESOURCE: i32 = 278i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESUBRESOURCE: i32 = 279i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCEBOX: i32 = 280i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCE: i32 = 281i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSTATE: i32 = 282i32;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESTATE: i32 = 283i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCE: i32 = 284i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDDESTINATIONSTATE: i32 = 285i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCESTATE: i32 = 286i32;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSUBRESOURCE: i32 = 287i32;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONBOX: i32 = 288i32;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSTATE: i32 = 289i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_INVALID: i32 = 290i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_SUBRESOURCE_INVALID: i32 = 291i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_INVALID: i32 = 292i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_SUBRESOURCE_INVALID: i32 = 293i32;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_FORMAT_INVALID: i32 = 294i32;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDMAPTYPE: i32 = 295i32;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDFLAGS: i32 = 296i32;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_ALREADYMAPPED: i32 = 297i32;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_DEVICEREMOVED_RETURN: i32 = 298i32;
pub const D3D10_MESSAGE_ID_BUFFER_UNMAP_NOTMAPPED: i32 = 299i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDMAPTYPE: i32 = 300i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDSUBRESOURCE: i32 = 301i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDFLAGS: i32 = 302i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_ALREADYMAPPED: i32 = 303i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_DEVICEREMOVED_RETURN: i32 = 304i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_INVALIDSUBRESOURCE: i32 = 305i32;
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_NOTMAPPED: i32 = 306i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDMAPTYPE: i32 = 307i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDSUBRESOURCE: i32 = 308i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDFLAGS: i32 = 309i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_ALREADYMAPPED: i32 = 310i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_DEVICEREMOVED_RETURN: i32 = 311i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_INVALIDSUBRESOURCE: i32 = 312i32;
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_NOTMAPPED: i32 = 313i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDMAPTYPE: i32 = 314i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDSUBRESOURCE: i32 = 315i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDFLAGS: i32 = 316i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_ALREADYMAPPED: i32 = 317i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_DEVICEREMOVED_RETURN: i32 = 318i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_INVALIDSUBRESOURCE: i32 = 319i32;
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_NOTMAPPED: i32 = 320i32;
pub const D3D10_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_DEPRECATED: i32 = 321i32;
pub const D3D10_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_FORMAT_DEPRECATED: i32 = 322i32;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_UNRECOGNIZEDFLAGS: i32 = 323i32;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_INVALIDARG_RETURN: i32 = 324i32;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_DEVICEREMOVED_RETURN: i32 = 325i32;
pub const D3D10_MESSAGE_ID_REF_SIMULATING_INFINITELY_FAST_HARDWARE: i32 = 326i32;
pub const D3D10_MESSAGE_ID_REF_THREADING_MODE: i32 = 327i32;
pub const D3D10_MESSAGE_ID_REF_UMDRIVER_EXCEPTION: i32 = 328i32;
pub const D3D10_MESSAGE_ID_REF_KMDRIVER_EXCEPTION: i32 = 329i32;
pub const D3D10_MESSAGE_ID_REF_HARDWARE_EXCEPTION: i32 = 330i32;
pub const D3D10_MESSAGE_ID_REF_ACCESSING_INDEXABLE_TEMP_OUT_OF_RANGE: i32 = 331i32;
pub const D3D10_MESSAGE_ID_REF_PROBLEM_PARSING_SHADER: i32 = 332i32;
pub const D3D10_MESSAGE_ID_REF_OUT_OF_MEMORY: i32 = 333i32;
pub const D3D10_MESSAGE_ID_REF_INFO: i32 = 334i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEXPOS_OVERFLOW: i32 = 335i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXED_INDEXPOS_OVERFLOW: i32 = 336i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_VERTEXPOS_OVERFLOW: i32 = 337i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_INSTANCEPOS_OVERFLOW: i32 = 338i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INSTANCEPOS_OVERFLOW: i32 = 339i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INDEXPOS_OVERFLOW: i32 = 340i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_SHADER_NOT_SET: i32 = 341i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: i32 = 342i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERINDEX: i32 = 343i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_COMPONENTTYPE: i32 = 344i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERMASK: i32 = 345i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SYSTEMVALUE: i32 = 346i32;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: i32 = 347i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_NOT_SET: i32 = 348i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INPUTLAYOUT_NOT_SET: i32 = 349i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_NOT_SET: i32 = 350i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_TOO_SMALL: i32 = 351i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_NOT_SET: i32 = 352i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SHADERRESOURCEVIEW_NOT_SET: i32 = 353i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEW_DIMENSION_MISMATCH: i32 = 354i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: i32 = 355i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_TOO_SMALL: i32 = 356i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_NOT_SET: i32 = 357i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_FORMAT_INVALID: i32 = 358i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_TOO_SMALL: i32 = 359i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_GS_INPUT_PRIMITIVE_MISMATCH: i32 = 360i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_RETURN_TYPE_MISMATCH: i32 = 361i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_POSITION_NOT_PRESENT: i32 = 362i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_NOT_SET: i32 = 363i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_BOUND_RESOURCE_MAPPED: i32 = 364i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_PRIMITIVETOPOLOGY: i32 = 365i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_OFFSET_UNALIGNED: i32 = 366i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_STRIDE_UNALIGNED: i32 = 367i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_OFFSET_UNALIGNED: i32 = 368i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_OFFSET_UNALIGNED: i32 = 369i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_LD_UNSUPPORTED: i32 = 370i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_UNSUPPORTED: i32 = 371i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_C_UNSUPPORTED: i32 = 372i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_MULTISAMPLE_UNSUPPORTED: i32 = 373i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_TARGETS_BOUND_WITHOUT_SOURCE: i32 = 374i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_STRIDE_LARGER_THAN_BUFFER: i32 = 375i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: i32 = 376i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: i32 = 377i32;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: i32 = 378i32;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: i32 = 379i32;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: i32 = 380i32;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_INVALIDARG_RETURN: i32 = 381i32;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_OUTOFMEMORY_RETURN: i32 = 382i32;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BADINTERFACE_RETURN: i32 = 383i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEWPORT_NOT_SET: i32 = 384i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: i32 = 385i32;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: i32 = 386i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_DENORMFLUSH: i32 = 387i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_INVALIDVIEW: i32 = 388i32;
pub const D3D10_MESSAGE_ID_DEVICE_SETTEXTFILTERSIZE_INVALIDDIMENSIONS: i32 = 389i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_MISMATCH: i32 = 390i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: i32 = 391i32;
pub const D3D10_MESSAGE_ID_BLENDSTATE_GETDESC_LEGACY: i32 = 392i32;
pub const D3D10_MESSAGE_ID_SHADERRESOURCEVIEW_GETDESC_LEGACY: i32 = 393i32;
pub const D3D10_MESSAGE_ID_CREATEQUERY_OUTOFMEMORY_RETURN: i32 = 394i32;
pub const D3D10_MESSAGE_ID_CREATEPREDICATE_OUTOFMEMORY_RETURN: i32 = 395i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFRANGE_COUNTER: i32 = 396i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_SIMULTANEOUS_ACTIVE_COUNTERS_EXHAUSTED: i32 = 397i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: i32 = 398i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFMEMORY_RETURN: i32 = 399i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NONEXCLUSIVE_RETURN: i32 = 400i32;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NULLDESC: i32 = 401i32;
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_OUTOFRANGE_COUNTER: i32 = 402i32;
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: i32 = 403i32;
pub const D3D10_MESSAGE_ID_SETPREDICATION_INVALID_PREDICATE_STATE: i32 = 404i32;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_UNSUPPORTED: i32 = 405i32;
pub const D3D10_MESSAGE_ID_PREDICATE_BEGIN_DURING_PREDICATION: i32 = 406i32;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_DUPLICATE: i32 = 407i32;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_ABANDONING_PREVIOUS_RESULTS: i32 = 408i32;
pub const D3D10_MESSAGE_ID_PREDICATE_END_DURING_PREDICATION: i32 = 409i32;
pub const D3D10_MESSAGE_ID_QUERY_END_ABANDONING_PREVIOUS_RESULTS: i32 = 410i32;
pub const D3D10_MESSAGE_ID_QUERY_END_WITHOUT_BEGIN: i32 = 411i32;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_DATASIZE: i32 = 412i32;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_FLAGS: i32 = 413i32;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_CALL: i32 = 414i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_PS_OUTPUT_TYPE_MISMATCH: i32 = 415i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_GATHER_UNSUPPORTED: i32 = 416i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: i32 = 417i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_STRIDE_TOO_LARGE: i32 = 418i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_INVALIDRANGE: i32 = 419i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: i32 = 420i32;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_SAMPLE_COUNT_MISMATCH: i32 = 421i32;
pub const D3D10_MESSAGE_ID_LIVE_OBJECT_SUMMARY: i32 = 422i32;
pub const D3D10_MESSAGE_ID_LIVE_BUFFER: i32 = 423i32;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE1D: i32 = 424i32;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE2D: i32 = 425i32;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE3D: i32 = 426i32;
pub const D3D10_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW: i32 = 427i32;
pub const D3D10_MESSAGE_ID_LIVE_RENDERTARGETVIEW: i32 = 428i32;
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW: i32 = 429i32;
pub const D3D10_MESSAGE_ID_LIVE_VERTEXSHADER: i32 = 430i32;
pub const D3D10_MESSAGE_ID_LIVE_GEOMETRYSHADER: i32 = 431i32;
pub const D3D10_MESSAGE_ID_LIVE_PIXELSHADER: i32 = 432i32;
pub const D3D10_MESSAGE_ID_LIVE_INPUTLAYOUT: i32 = 433i32;
pub const D3D10_MESSAGE_ID_LIVE_SAMPLER: i32 = 434i32;
pub const D3D10_MESSAGE_ID_LIVE_BLENDSTATE: i32 = 435i32;
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE: i32 = 436i32;
pub const D3D10_MESSAGE_ID_LIVE_RASTERIZERSTATE: i32 = 437i32;
pub const D3D10_MESSAGE_ID_LIVE_QUERY: i32 = 438i32;
pub const D3D10_MESSAGE_ID_LIVE_PREDICATE: i32 = 439i32;
pub const D3D10_MESSAGE_ID_LIVE_COUNTER: i32 = 440i32;
pub const D3D10_MESSAGE_ID_LIVE_DEVICE: i32 = 441i32;
pub const D3D10_MESSAGE_ID_LIVE_SWAPCHAIN: i32 = 442i32;
pub const D3D10_MESSAGE_ID_D3D10_MESSAGES_END: i32 = 443i32;
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_START: i32 = 1048576i32;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_STENCIL_NO_TWO_SIDED: i32 = 1048577i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthBiasClamp_NOT_SUPPORTED: i32 = 1048578i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_COMPARISON_SUPPORT: i32 = 1048579i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_EXCESSIVE_ANISOTROPY: i32 = 1048580i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_OUT_OF_RANGE: i32 = 1048581i32;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_NOT_SUPPORTED: i32 = 1048582i32;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_TOO_MANY_SAMPLERS: i32 = 1048583i32;
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_TOO_MANY_SAMPLERS: i32 = 1048584i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_ARRAYS: i32 = 1048585i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_VB_AND_IB_BIND: i32 = 1048586i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_TEXTURE_1D: i32 = 1048587i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_OUT_OF_RANGE: i32 = 1048588i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_SHADER_RESOURCE: i32 = 1048589i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_TOO_MANY_RENDER_TARGETS: i32 = 1048590i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_DIFFERING_BIT_DEPTHS: i32 = 1048591i32;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_BAD_BUFFER_INDEX: i32 = 1048592i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_TOO_MANY_VIEWPORTS: i32 = 1048593i32;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_ADJACENCY_UNSUPPORTED: i32 = 1048594i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_TOO_MANY_SCISSORS: i32 = 1048595i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_ONLY_TEXTURE_2D_WITHIN_GPU_MEMORY: i32 = 1048596i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_3D_READBACK: i32 = 1048597i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_ONLY_READBACK: i32 = 1048598i32;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNSUPPORTED_FORMAT: i32 = 1048599i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_ALPHA_TO_COVERAGE: i32 = 1048600i32;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthClipEnable_MUST_BE_TRUE: i32 = 1048601i32;
pub const D3D10_MESSAGE_ID_DRAWINDEXED_STARTINDEXLOCATION_MUST_BE_POSITIVE: i32 = 1048602i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_MUST_USE_LOWEST_LOD: i32 = 1048603i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MINLOD_MUST_NOT_BE_FRACTIONAL: i32 = 1048604i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MAXLOD_MUST_BE_FLT_MAX: i32 = 1048605i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_FIRSTARRAYSLICE_MUST_BE_ZERO: i32 = 1048606i32;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_CUBES_MUST_HAVE_6_SIDES: i32 = 1048607i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_RENDER_TARGET: i32 = 1048608i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_DWORD_INDEX_BUFFER: i32 = 1048609i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_MSAA_PRECLUDES_SHADER_RESOURCE: i32 = 1048610i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_PRESENTATION_PRECLUDES_SHADER_RESOURCE: i32 = 1048611i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_BLEND_ENABLE: i32 = 1048612i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_WRITE_MASKS: i32 = 1048613i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_STREAM_OUT: i32 = 1048614i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_VB_IB_FOR_BUFFERS: i32 = 1048615i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_AUTOGEN_FOR_VOLUMES: i32 = 1048616i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DXGI_FORMAT_R8G8B8A8_CANNOT_BE_SHARED: i32 = 1048617i32;
pub const D3D10_MESSAGE_ID_VSSHADERRESOURCES_NOT_SUPPORTED: i32 = 1048618i32;
pub const D3D10_MESSAGE_ID_GEOMETRY_SHADER_NOT_SUPPORTED: i32 = 1048619i32;
pub const D3D10_MESSAGE_ID_STREAM_OUT_NOT_SUPPORTED: i32 = 1048620i32;
pub const D3D10_MESSAGE_ID_TEXT_FILTER_NOT_SUPPORTED: i32 = 1048621i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_SEPARATE_ALPHA_BLEND: i32 = 1048622i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_MRT_BLEND: i32 = 1048623i32;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_OPERATION_NOT_SUPPORTED: i32 = 1048624i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_MIRRORONCE: i32 = 1048625i32;
pub const D3D10_MESSAGE_ID_DRAWINSTANCED_NOT_SUPPORTED: i32 = 1048626i32;
pub const D3D10_MESSAGE_ID_DRAWINDEXEDINSTANCED_NOT_SUPPORTED_BELOW_9_3: i32 = 1048627i32;
pub const D3D10_MESSAGE_ID_DRAWINDEXED_POINTLIST_UNSUPPORTED: i32 = 1048628i32;
pub const D3D10_MESSAGE_ID_SETBLENDSTATE_SAMPLE_MASK_CANNOT_BE_ZERO: i32 = 1048629i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_EXCEEDS_FEATURE_LEVEL_DEFINITION: i32 = 1048630i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_SINGLE_MIP_LEVEL_DEPTH_STENCIL_SUPPORTED: i32 = 1048631i32;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_NEGATIVESCISSOR: i32 = 1048632i32;
pub const D3D10_MESSAGE_ID_SLOT_ZERO_MUST_BE_D3D10_INPUT_PER_VERTEX_DATA: i32 = 1048633i32;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NON_POW_2_MIPMAP: i32 = 1048634i32;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_NOT_SUPPORTED: i32 = 1048635i32;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_SRGB_MRT: i32 = 1048636i32;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_3D_MISMATCHED_UPDATES: i32 = 1048637i32;
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_END: i32 = 1048638i32;
pub const D3D10_MESSAGE_SEVERITY_CORRUPTION: i32 = 0i32;
pub const D3D10_MESSAGE_SEVERITY_ERROR: i32 = 1i32;
pub const D3D10_MESSAGE_SEVERITY_WARNING: i32 = 2i32;
pub const D3D10_MESSAGE_SEVERITY_INFO: i32 = 3i32;
pub const D3D10_MESSAGE_SEVERITY_MESSAGE: i32 = 4i32;
pub const D3D10_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D10_MIN_DEPTH: f32 = 0f32;
pub const D3D10_MIN_FILTER_SHIFT: u32 = 4u32;
pub const D3D10_MIN_MAXANISOTROPY: u32 = 0u32;
pub const D3D10_MIP_FILTER_SHIFT: u32 = 0u32;
pub const D3D10_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
pub const D3D10_MIP_LOD_BIAS_MIN: f32 = -16f32;
pub const D3D10_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 6u32;
pub const D3D10_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
pub const D3D10_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
pub const D3D10_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_PASS_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Annotations: u32,
    pub pIAInputSignature: *mut u8,
    pub IAInputSignatureSize: usize,
    pub StencilRef: u32,
    pub SampleMask: u32,
    pub BlendFactor: [f32; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_PASS_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_PASS_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_PASS_SHADER_DESC {
    pub pShaderVariable: ID3D10EffectShaderVariable,
    pub ShaderIndex: u32,
}
impl ::core::marker::Copy for D3D10_PASS_SHADER_DESC {}
impl ::core::clone::Clone for D3D10_PASS_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 13u32;
pub const D3D10_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
pub const D3D10_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
pub const D3D10_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
pub const D3D10_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
pub const D3D10_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D10_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
pub const D3D10_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
pub const D3D10_QUERY_EVENT: i32 = 0i32;
pub const D3D10_QUERY_OCCLUSION: i32 = 1i32;
pub const D3D10_QUERY_TIMESTAMP: i32 = 2i32;
pub const D3D10_QUERY_TIMESTAMP_DISJOINT: i32 = 3i32;
pub const D3D10_QUERY_PIPELINE_STATISTICS: i32 = 4i32;
pub const D3D10_QUERY_OCCLUSION_PREDICATE: i32 = 5i32;
pub const D3D10_QUERY_SO_STATISTICS: i32 = 6i32;
pub const D3D10_QUERY_SO_OVERFLOW_PREDICATE: i32 = 7i32;
#[repr(C)]
pub struct D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
}
impl ::core::marker::Copy for D3D10_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::clone::Clone for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
impl ::core::marker::Copy for D3D10_QUERY_DATA_SO_STATISTICS {}
impl ::core::clone::Clone for D3D10_QUERY_DATA_SO_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_QUERY_DESC {
    pub Query: D3D10_QUERY,
    pub MiscFlags: u32,
}
impl ::core::marker::Copy for D3D10_QUERY_DESC {}
impl ::core::clone::Clone for D3D10_QUERY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_QUERY_MISC_PREDICATEHINT: i32 = 1i32;
pub const D3D10_RAISE_FLAG_DRIVER_INTERNAL_ERROR: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_RASTERIZER_DESC {
    pub FillMode: D3D10_FILL_MODE,
    pub CullMode: D3D10_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_RASTERIZER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_RENDER_TARGET_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_RENDER_TARGET_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D10_RTV_DIMENSION,
    pub Anonymous: D3D10_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_RENDER_TARGET_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_RENDER_TARGET_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D10_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D10_BUFFER_RTV,
    pub Texture1D: D3D10_TEX1D_RTV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D10_TEX2D_RTV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D10_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D10_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_RENDER_TARGET_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_REQ_BLEND_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
pub const D3D10_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D10_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D10_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D10_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
pub const D3D10_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D10_REQ_MAXANISOTROPY: u32 = 16u32;
pub const D3D10_REQ_MIP_LEVELS: u32 = 14u32;
pub const D3D10_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
pub const D3D10_REQ_RASTERIZER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 8192u32;
pub const D3D10_REQ_RESOURCE_SIZE_IN_MEGABYTES: u32 = 128u32;
pub const D3D10_REQ_RESOURCE_VIEW_COUNT_PER_CONTEXT_2_TO_EXP: u32 = 20u32;
pub const D3D10_REQ_SAMPLER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096u32;
pub const D3D10_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 512u32;
pub const D3D10_REQ_TEXTURE1D_U_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 512u32;
pub const D3D10_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 8192u32;
pub const D3D10_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
pub const D3D10_REQ_TEXTURECUBE_DIMENSION: u32 = 8192u32;
pub const D3D10_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
pub const D3D10_RESOURCE_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D10_RESOURCE_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE2D: i32 = 3i32;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE3D: i32 = 4i32;
pub const D3D10_RESOURCE_MISC_GENERATE_MIPS: i32 = 1i32;
pub const D3D10_RESOURCE_MISC_SHARED: i32 = 2i32;
pub const D3D10_RESOURCE_MISC_TEXTURECUBE: i32 = 4i32;
pub const D3D10_RESOURCE_MISC_SHARED_KEYEDMUTEX: i32 = 16i32;
pub const D3D10_RESOURCE_MISC_GDI_COMPATIBLE: i32 = 32i32;
pub const D3D10_RTV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D10_RTV_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D10_RTV_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D10_RTV_DIMENSION_TEXTURE1DARRAY: i32 = 3i32;
pub const D3D10_RTV_DIMENSION_TEXTURE2D: i32 = 4i32;
pub const D3D10_RTV_DIMENSION_TEXTURE2DARRAY: i32 = 5i32;
pub const D3D10_RTV_DIMENSION_TEXTURE2DMS: i32 = 6i32;
pub const D3D10_RTV_DIMENSION_TEXTURE2DMSARRAY: i32 = 7i32;
pub const D3D10_RTV_DIMENSION_TEXTURE3D: i32 = 8i32;
#[repr(C)]
pub struct D3D10_SAMPLER_DESC {
    pub Filter: D3D10_FILTER,
    pub AddressU: D3D10_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D10_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D10_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D10_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl ::core::marker::Copy for D3D10_SAMPLER_DESC {}
impl ::core::clone::Clone for D3D10_SAMPLER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SDK_LAYERS_VERSION: u32 = 11u32;
pub const D3D10_SDK_VERSION: u32 = 29u32;
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_SHADER_BUFFER_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_SHADER_BUFFER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_SHADER_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SHADER_DEBUG: u32 = 1u32;
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_FILE_INFO {
    pub FileName: u32,
    pub FileNameLen: u32,
    pub FileData: u32,
    pub FileLen: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_FILE_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_INFO {
    pub Size: u32,
    pub Creator: u32,
    pub EntrypointName: u32,
    pub ShaderTarget: u32,
    pub CompileFlags: u32,
    pub Files: u32,
    pub FileInfo: u32,
    pub Instructions: u32,
    pub InstructionInfo: u32,
    pub Variables: u32,
    pub VariableInfo: u32,
    pub InputVariables: u32,
    pub InputVariableInfo: u32,
    pub Tokens: u32,
    pub TokenInfo: u32,
    pub Scopes: u32,
    pub ScopeInfo: u32,
    pub ScopeVariables: u32,
    pub ScopeVariableInfo: u32,
    pub UintOffset: u32,
    pub StringOffset: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_INPUT_INFO {
    pub Var: u32,
    pub InitialRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub InitialBank: u32,
    pub InitialRegister: u32,
    pub InitialComponent: u32,
    pub InitialValue: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_INPUT_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SHADER_DEBUG_INST_INFO {
    pub Id: u32,
    pub Opcode: u32,
    pub uOutputs: u32,
    pub pOutputs: [D3D10_SHADER_DEBUG_OUTPUTREG_INFO; 2],
    pub TokenId: u32,
    pub NestingLevel: u32,
    pub Scopes: u32,
    pub ScopeInfo: u32,
    pub AccessedVars: u32,
    pub AccessedVarsInfo: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_INST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_INST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    pub OutputRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub OutputReg: u32,
    pub TempArrayReg: u32,
    pub OutputComponents: [u32; 4],
    pub OutputVars: [D3D10_SHADER_DEBUG_OUTPUTVAR; 4],
    pub IndexReg: u32,
    pub IndexComp: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SHADER_DEBUG_OUTPUTVAR {
    pub Var: u32,
    pub uValueMin: u32,
    pub uValueMax: u32,
    pub iValueMin: i32,
    pub iValueMax: i32,
    pub fValueMin: f32,
    pub fValueMax: f32,
    pub bNaNPossible: super::super::Foundation::BOOL,
    pub bInfPossible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_OUTPUTVAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SHADER_DEBUG_REG_INPUT: i32 = 0i32;
pub const D3D10_SHADER_DEBUG_REG_OUTPUT: i32 = 1i32;
pub const D3D10_SHADER_DEBUG_REG_CBUFFER: i32 = 2i32;
pub const D3D10_SHADER_DEBUG_REG_TBUFFER: i32 = 3i32;
pub const D3D10_SHADER_DEBUG_REG_TEMP: i32 = 4i32;
pub const D3D10_SHADER_DEBUG_REG_TEMPARRAY: i32 = 5i32;
pub const D3D10_SHADER_DEBUG_REG_TEXTURE: i32 = 6i32;
pub const D3D10_SHADER_DEBUG_REG_SAMPLER: i32 = 7i32;
pub const D3D10_SHADER_DEBUG_REG_IMMEDIATECBUFFER: i32 = 8i32;
pub const D3D10_SHADER_DEBUG_REG_LITERAL: i32 = 9i32;
pub const D3D10_SHADER_DEBUG_REG_UNUSED: i32 = 10i32;
pub const D3D11_SHADER_DEBUG_REG_INTERFACE_POINTERS: i32 = 11i32;
pub const D3D11_SHADER_DEBUG_REG_UAV: i32 = 12i32;
pub const D3D10_SHADER_DEBUG_REG_FORCE_DWORD: i32 = 2147483647i32;
pub const D3D10_SHADER_DEBUG_SCOPE_GLOBAL: i32 = 0i32;
pub const D3D10_SHADER_DEBUG_SCOPE_BLOCK: i32 = 1i32;
pub const D3D10_SHADER_DEBUG_SCOPE_FORLOOP: i32 = 2i32;
pub const D3D10_SHADER_DEBUG_SCOPE_STRUCT: i32 = 3i32;
pub const D3D10_SHADER_DEBUG_SCOPE_FUNC_PARAMS: i32 = 4i32;
pub const D3D10_SHADER_DEBUG_SCOPE_STATEBLOCK: i32 = 5i32;
pub const D3D10_SHADER_DEBUG_SCOPE_NAMESPACE: i32 = 6i32;
pub const D3D10_SHADER_DEBUG_SCOPE_ANNOTATION: i32 = 7i32;
pub const D3D10_SHADER_DEBUG_SCOPE_FORCE_DWORD: i32 = 2147483647i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    pub TokenId: u32,
    pub VarType: D3D10_SHADER_DEBUG_VARTYPE,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub StructMemberScope: u32,
    pub uArrayIndices: u32,
    pub ArrayElements: u32,
    pub ArrayStrides: u32,
    pub uVariables: u32,
    pub uFirstVariable: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_SCOPE_INFO {
    pub ScopeType: D3D10_SHADER_DEBUG_SCOPETYPE,
    pub Name: u32,
    pub uNameLen: u32,
    pub uVariables: u32,
    pub VariableData: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_SCOPE_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_TOKEN_INFO {
    pub File: u32,
    pub Line: u32,
    pub Column: u32,
    pub TokenLength: u32,
    pub TokenId: u32,
}
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_TOKEN_INFO {}
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SHADER_DEBUG_VAR_VARIABLE: i32 = 0i32;
pub const D3D10_SHADER_DEBUG_VAR_FUNCTION: i32 = 1i32;
pub const D3D10_SHADER_DEBUG_VAR_FORCE_DWORD: i32 = 2147483647i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_DEBUG_VAR_INFO {
    pub TokenId: u32,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Register: u32,
    pub Component: u32,
    pub ScopeVar: u32,
    pub ScopeVarOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_DEBUG_VAR_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_DEBUG_VAR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_SHADER_DESC {
    pub Version: u32,
    pub Creator: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InputParameters: u32,
    pub OutputParameters: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub CutInstructionCount: u32,
    pub EmitInstructionCount: u32,
    pub GSOutputTopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_SHADER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SHADER_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
pub const D3D10_SHADER_ENABLE_STRICTNESS: u32 = 2048u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
pub const D3D10_SHADER_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
pub const D3D10_SHADER_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
pub const D3D10_SHADER_IEEE_STRICTNESS: u32 = 8192u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_SHADER_INPUT_BIND_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::Direct3D::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_SHADER_INPUT_BIND_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_SHADER_INPUT_BIND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SHADER_MAJOR_VERSION: u32 = 4u32;
pub const D3D10_SHADER_MINOR_VERSION: u32 = 0u32;
pub const D3D10_SHADER_NO_PRESHADER: u32 = 256u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL0: u32 = 16384u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL1: u32 = 0u32;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL3: u32 = 32768u32;
pub const D3D10_SHADER_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
pub const D3D10_SHADER_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
pub const D3D10_SHADER_PARTIAL_PRECISION: u32 = 32u32;
pub const D3D10_SHADER_PREFER_FLOW_CONTROL: u32 = 1024u32;
pub const D3D10_SHADER_RESOURCES_MAY_ALIAS: u32 = 524288u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_SHADER_RESOURCE_VIEW_DESC {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_SHADER_RESOURCE_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub union D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D10_BUFFER_SRV,
    pub Texture1D: D3D10_TEX1D_SRV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D10_TEX2D_SRV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D10_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D10_TEX3D_SRV,
    pub TextureCube: D3D10_TEXCUBE_SRV,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_SHADER_RESOURCE_VIEW_DESC1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub union D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: D3D10_BUFFER_SRV,
    pub Texture1D: D3D10_TEX1D_SRV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D10_TEX2D_SRV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D10_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D10_TEX3D_SRV,
    pub TextureCube: D3D10_TEXCUBE_SRV,
    pub TextureCubeArray: D3D10_TEXCUBE_ARRAY_SRV1,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SHADER_SKIP_OPTIMIZATION: u32 = 4u32;
pub const D3D10_SHADER_SKIP_VALIDATION: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D10_SHADER_TYPE_DESC {
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D10_SHADER_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D10_SHADER_TYPE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SHADER_VARIABLE_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SHADER_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SHADER_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144u32;
pub const D3D10_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D10_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D10_SIGNATURE_PARAMETER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D10_SIGNATURE_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D10_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D10_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
pub const D3D10_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D10_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_SO_DECLARATION_ENTRY {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_SO_DECLARATION_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_SO_DECLARATION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1u32;
pub const D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64u32;
pub const D3D10_SRGB_GAMMA: f32 = 2.2f32;
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
pub const D3D10_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
pub const D3D10_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
pub const D3D10_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
pub const D3D10_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
pub const D3D10_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
pub const D3D10_STANDARD_MULTISAMPLE_PATTERN: i32 = -1i32;
pub const D3D10_CENTER_MULTISAMPLE_PATTERN: i32 = -2i32;
pub const D3D10_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D10_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D10_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D10_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
#[repr(C)]
pub struct D3D10_STATE_BLOCK_MASK {
    pub VS: u8,
    pub VSSamplers: [u8; 2],
    pub VSShaderResources: [u8; 16],
    pub VSConstantBuffers: [u8; 2],
    pub GS: u8,
    pub GSSamplers: [u8; 2],
    pub GSShaderResources: [u8; 16],
    pub GSConstantBuffers: [u8; 2],
    pub PS: u8,
    pub PSSamplers: [u8; 2],
    pub PSShaderResources: [u8; 16],
    pub PSConstantBuffers: [u8; 2],
    pub IAVertexBuffers: [u8; 2],
    pub IAIndexBuffer: u8,
    pub IAInputLayout: u8,
    pub IAPrimitiveTopology: u8,
    pub OMRenderTargets: u8,
    pub OMDepthStencilState: u8,
    pub OMBlendState: u8,
    pub RSViewports: u8,
    pub RSScissorRects: u8,
    pub RSRasterizerState: u8,
    pub SOBuffers: u8,
    pub Predication: u8,
}
impl ::core::marker::Copy for D3D10_STATE_BLOCK_MASK {}
impl ::core::clone::Clone for D3D10_STATE_BLOCK_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_STENCIL_OP_KEEP: i32 = 1i32;
pub const D3D10_STENCIL_OP_ZERO: i32 = 2i32;
pub const D3D10_STENCIL_OP_REPLACE: i32 = 3i32;
pub const D3D10_STENCIL_OP_INCR_SAT: i32 = 4i32;
pub const D3D10_STENCIL_OP_DECR_SAT: i32 = 5i32;
pub const D3D10_STENCIL_OP_INVERT: i32 = 6i32;
pub const D3D10_STENCIL_OP_INCR: i32 = 7i32;
pub const D3D10_STENCIL_OP_DECR: i32 = 8i32;
pub const D3D10_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[repr(C)]
pub struct D3D10_SUBRESOURCE_DATA {
    pub pSysMem: *mut ::core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl ::core::marker::Copy for D3D10_SUBRESOURCE_DATA {}
impl ::core::clone::Clone for D3D10_SUBRESOURCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 6u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D10_TECHNIQUE_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Passes: u32,
    pub Annotations: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D10_TECHNIQUE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D10_TECHNIQUE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D10_TEX1D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D10_TEX1D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D10_TEX1D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX1D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_DSV {}
impl ::core::clone::Clone for D3D10_TEX1D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX1D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_RTV {}
impl ::core::clone::Clone for D3D10_TEX1D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D10_TEX1D_SRV {}
impl ::core::clone::Clone for D3D10_TEX1D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_ARRAY_DSV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_ARRAY_RTV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_ARRAY_SRV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_DSV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_RTV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D10_TEX2DMS_SRV {}
impl ::core::clone::Clone for D3D10_TEX2DMS_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D10_TEX2D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D10_TEX2D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D10_TEX2D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_DSV {}
impl ::core::clone::Clone for D3D10_TEX2D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_RTV {}
impl ::core::clone::Clone for D3D10_TEX2D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D10_TEX2D_SRV {}
impl ::core::clone::Clone for D3D10_TEX2D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl ::core::marker::Copy for D3D10_TEX3D_RTV {}
impl ::core::clone::Clone for D3D10_TEX3D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D10_TEX3D_SRV {}
impl ::core::clone::Clone for D3D10_TEX3D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEXCUBE_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
impl ::core::marker::Copy for D3D10_TEXCUBE_ARRAY_SRV1 {}
impl ::core::clone::Clone for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D10_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D10_TEXCUBE_SRV {}
impl ::core::clone::Clone for D3D10_TEXCUBE_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_TEXTURE1D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_TEXTURE1D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_TEXTURE2D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_TEXTURE2D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D10_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D10_TEXTURE3D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D10_TEXTURE3D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_X: i32 = 0i32;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_X: i32 = 1i32;
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Y: i32 = 2i32;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Y: i32 = 3i32;
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Z: i32 = 4i32;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Z: i32 = 5i32;
pub const D3D10_TEXTURE_ADDRESS_WRAP: i32 = 1i32;
pub const D3D10_TEXTURE_ADDRESS_MIRROR: i32 = 2i32;
pub const D3D10_TEXTURE_ADDRESS_CLAMP: i32 = 3i32;
pub const D3D10_TEXTURE_ADDRESS_BORDER: i32 = 4i32;
pub const D3D10_TEXTURE_ADDRESS_MIRROR_ONCE: i32 = 5i32;
pub const D3D10_TEXT_1BIT_BIT: u32 = 2147483648u32;
pub const D3D10_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
pub const D3D10_USAGE_DEFAULT: i32 = 0i32;
pub const D3D10_USAGE_IMMUTABLE: i32 = 1i32;
pub const D3D10_USAGE_DYNAMIC: i32 = 2i32;
pub const D3D10_USAGE_STAGING: i32 = 3i32;
#[repr(C)]
pub struct D3D10_VIEWPORT {
    pub TopLeftX: i32,
    pub TopLeftY: i32,
    pub Width: u32,
    pub Height: u32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
impl ::core::marker::Copy for D3D10_VIEWPORT {}
impl ::core::clone::Clone for D3D10_VIEWPORT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D10_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
pub const D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
pub const D3D10_VIEWPORT_BOUNDS_MAX: u32 = 16383u32;
pub const D3D10_VIEWPORT_BOUNDS_MIN: i32 = -16384i32;
pub const D3D10_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_VS_INPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D10_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D10_VS_OUTPUT_REGISTER_COUNT: u32 = 16u32;
pub const D3D10_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
pub const D3D10_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D10_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D_MAJOR_VERSION: u32 = 10u32;
pub const D3D_MINOR_VERSION: u32 = 0u32;
pub const D3D_SPEC_DATE_DAY: u32 = 8u32;
pub const D3D_SPEC_DATE_MONTH: u32 = 8u32;
pub const D3D_SPEC_DATE_YEAR: u32 = 2006u32;
pub const D3D_SPEC_VERSION: f64 = 1.050005f64;
pub const DXGI_DEBUG_D3D10: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 607865938, data2: 13830, data3: 19770, data4: [153, 215, 167, 231, 179, 62, 215, 6] };
pub const GUID_DeviceType: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3609393997, data2: 31336, data3: 17274, data4: [178, 12, 88, 4, 238, 36, 148, 166] };
#[repr(transparent)]
pub struct ID3D10Asynchronous(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Asynchronous {}
impl ::core::clone::Clone for ID3D10Asynchronous {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10BlendState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10BlendState {}
impl ::core::clone::Clone for ID3D10BlendState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10BlendState1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10BlendState1 {}
impl ::core::clone::Clone for ID3D10BlendState1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Buffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Buffer {}
impl ::core::clone::Clone for ID3D10Buffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Counter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Counter {}
impl ::core::clone::Clone for ID3D10Counter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Debug(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Debug {}
impl ::core::clone::Clone for ID3D10Debug {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10DepthStencilState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10DepthStencilState {}
impl ::core::clone::Clone for ID3D10DepthStencilState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10DepthStencilView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10DepthStencilView {}
impl ::core::clone::Clone for ID3D10DepthStencilView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Device(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Device {}
impl ::core::clone::Clone for ID3D10Device {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Device1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Device1 {}
impl ::core::clone::Clone for ID3D10Device1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10DeviceChild(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10DeviceChild {}
impl ::core::clone::Clone for ID3D10DeviceChild {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Effect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Effect {}
impl ::core::clone::Clone for ID3D10Effect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectBlendVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectBlendVariable {}
impl ::core::clone::Clone for ID3D10EffectBlendVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectConstantBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectConstantBuffer {}
impl ::core::clone::Clone for ID3D10EffectConstantBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectDepthStencilVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectDepthStencilVariable {}
impl ::core::clone::Clone for ID3D10EffectDepthStencilVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectDepthStencilViewVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectDepthStencilViewVariable {}
impl ::core::clone::Clone for ID3D10EffectDepthStencilViewVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectMatrixVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectMatrixVariable {}
impl ::core::clone::Clone for ID3D10EffectMatrixVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectPass(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectPass {}
impl ::core::clone::Clone for ID3D10EffectPass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectPool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectPool {}
impl ::core::clone::Clone for ID3D10EffectPool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectRasterizerVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectRasterizerVariable {}
impl ::core::clone::Clone for ID3D10EffectRasterizerVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectRenderTargetViewVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectRenderTargetViewVariable {}
impl ::core::clone::Clone for ID3D10EffectRenderTargetViewVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectSamplerVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectSamplerVariable {}
impl ::core::clone::Clone for ID3D10EffectSamplerVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectScalarVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectScalarVariable {}
impl ::core::clone::Clone for ID3D10EffectScalarVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectShaderResourceVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectShaderResourceVariable {}
impl ::core::clone::Clone for ID3D10EffectShaderResourceVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectShaderVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectShaderVariable {}
impl ::core::clone::Clone for ID3D10EffectShaderVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectStringVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectStringVariable {}
impl ::core::clone::Clone for ID3D10EffectStringVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectTechnique(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectTechnique {}
impl ::core::clone::Clone for ID3D10EffectTechnique {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectType {}
impl ::core::clone::Clone for ID3D10EffectType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectVariable {}
impl ::core::clone::Clone for ID3D10EffectVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10EffectVectorVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10EffectVectorVariable {}
impl ::core::clone::Clone for ID3D10EffectVectorVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10GeometryShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10GeometryShader {}
impl ::core::clone::Clone for ID3D10GeometryShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10InfoQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10InfoQueue {}
impl ::core::clone::Clone for ID3D10InfoQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10InputLayout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10InputLayout {}
impl ::core::clone::Clone for ID3D10InputLayout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Multithread(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Multithread {}
impl ::core::clone::Clone for ID3D10Multithread {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10PixelShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10PixelShader {}
impl ::core::clone::Clone for ID3D10PixelShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Predicate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Predicate {}
impl ::core::clone::Clone for ID3D10Predicate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Query(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Query {}
impl ::core::clone::Clone for ID3D10Query {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10RasterizerState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10RasterizerState {}
impl ::core::clone::Clone for ID3D10RasterizerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10RenderTargetView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10RenderTargetView {}
impl ::core::clone::Clone for ID3D10RenderTargetView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Resource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Resource {}
impl ::core::clone::Clone for ID3D10Resource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10SamplerState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10SamplerState {}
impl ::core::clone::Clone for ID3D10SamplerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10ShaderReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10ShaderReflection {}
impl ::core::clone::Clone for ID3D10ShaderReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10ShaderReflection1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10ShaderReflection1 {}
impl ::core::clone::Clone for ID3D10ShaderReflection1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10ShaderReflectionConstantBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10ShaderReflectionConstantBuffer {}
impl ::core::clone::Clone for ID3D10ShaderReflectionConstantBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10ShaderReflectionType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10ShaderReflectionType {}
impl ::core::clone::Clone for ID3D10ShaderReflectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10ShaderReflectionVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10ShaderReflectionVariable {}
impl ::core::clone::Clone for ID3D10ShaderReflectionVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10ShaderResourceView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10ShaderResourceView {}
impl ::core::clone::Clone for ID3D10ShaderResourceView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10ShaderResourceView1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10ShaderResourceView1 {}
impl ::core::clone::Clone for ID3D10ShaderResourceView1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10StateBlock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10StateBlock {}
impl ::core::clone::Clone for ID3D10StateBlock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10SwitchToRef(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10SwitchToRef {}
impl ::core::clone::Clone for ID3D10SwitchToRef {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Texture1D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Texture1D {}
impl ::core::clone::Clone for ID3D10Texture1D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Texture2D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Texture2D {}
impl ::core::clone::Clone for ID3D10Texture2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10Texture3D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10Texture3D {}
impl ::core::clone::Clone for ID3D10Texture3D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10VertexShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10VertexShader {}
impl ::core::clone::Clone for ID3D10VertexShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D10View(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D10View {}
impl ::core::clone::Clone for ID3D10View {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub type PFN_D3D10_CREATE_DEVICE1 = unsafe extern "system" fn(param0: super::Dxgi::IDXGIAdapter, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HINSTANCE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut ID3D10Device1) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
pub type PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1 = unsafe extern "system" fn(param0: super::Dxgi::IDXGIAdapter, param1: D3D10_DRIVER_TYPE, param2: super::super::Foundation::HINSTANCE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut super::Dxgi::DXGI_SWAP_CHAIN_DESC, param7: *mut super::Dxgi::IDXGISwapChain, param8: *mut ID3D10Device1) -> ::windows_sys::core::HRESULT;
pub const _FACD3D10: u32 = 2169u32;
