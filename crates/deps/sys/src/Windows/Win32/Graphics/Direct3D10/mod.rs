#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct D3D10_ASYNC_GETDATA_FLAG(i32);
#[repr(C)]
pub struct D3D10_BIND_FLAG(i32);
#[repr(C)]
pub struct D3D10_BLEND(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_BLEND_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_BLEND_DESC1(i32);
#[repr(C)]
pub struct D3D10_BLEND_OP(i32);
#[repr(C)]
pub struct D3D10_BOX(i32);
#[repr(C)]
pub struct D3D10_BUFFER_DESC(i32);
#[repr(C)]
pub struct D3D10_BUFFER_RTV(i32);
#[repr(C)]
pub struct D3D10_BUFFER_SRV(i32);
#[repr(C)]
pub struct D3D10_CLEAR_FLAG(i32);
pub const D3D10_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D10_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
#[repr(C)]
pub struct D3D10_COLOR_WRITE_ENABLE(i32);
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
#[repr(C)]
pub struct D3D10_COMPARISON_FUNC(i32);
#[repr(C)]
pub struct D3D10_COUNTER(i32);
#[repr(C)]
pub struct D3D10_COUNTER_DESC(i32);
#[repr(C)]
pub struct D3D10_COUNTER_INFO(i32);
#[repr(C)]
pub struct D3D10_COUNTER_TYPE(i32);
#[repr(C)]
pub struct D3D10_CPU_ACCESS_FLAG(i32);
#[repr(C)]
pub struct D3D10_CREATE_DEVICE_FLAG(i32);
#[repr(C)]
pub struct D3D10_CULL_MODE(i32);
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
pub struct D3D10_DEPTH_STENCILOP_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_DEPTH_STENCIL_DESC(i32);
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[repr(C)]
pub struct D3D10_DEPTH_STENCIL_VIEW_DESC(i32);
#[repr(C)]
pub struct D3D10_DEPTH_WRITE_MASK(i32);
#[repr(C)]
pub struct D3D10_DEVICE_STATE_TYPES(i32);
#[repr(C)]
pub struct D3D10_DRIVER_TYPE(i32);
#[repr(C)]
pub struct D3D10_DSV_DIMENSION(i32);
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: u32 = 2u32;
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_EFFECT_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_EFFECT_SHADER_DESC(i32);
pub const D3D10_EFFECT_SINGLE_THREADED: u32 = 8u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[repr(C)]
pub struct D3D10_EFFECT_TYPE_DESC(i32);
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_EFFECT_VARIABLE_DESC(i32);
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4u32;
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1u32;
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
#[repr(C)]
pub struct D3D10_FEATURE_LEVEL1(i32);
#[repr(C)]
pub struct D3D10_FILL_MODE(i32);
#[repr(C)]
pub struct D3D10_FILTER(i32);
#[repr(C)]
pub struct D3D10_FILTER_TYPE(i32);
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
#[repr(C)]
pub struct D3D10_FORMAT_SUPPORT(i32);
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
pub struct D3D10_INFO_QUEUE_FILTER(i32);
#[repr(C)]
pub struct D3D10_INFO_QUEUE_FILTER_DESC(i32);
#[repr(C)]
pub struct D3D10_INPUT_CLASSIFICATION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
#[repr(C)]
pub struct D3D10_INPUT_ELEMENT_DESC(i32);
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D10_LINEAR_GAMMA: f32 = 1f32;
pub const D3D10_MAG_FILTER_SHIFT: u32 = 2u32;
#[repr(C)]
pub struct D3D10_MAP(i32);
#[repr(C)]
pub struct D3D10_MAPPED_TEXTURE2D(i32);
#[repr(C)]
pub struct D3D10_MAPPED_TEXTURE3D(i32);
#[repr(C)]
pub struct D3D10_MAP_FLAG(i32);
pub const D3D10_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
pub const D3D10_MAX_DEPTH: f32 = 1f32;
pub const D3D10_MAX_MAXANISOTROPY: u32 = 16u32;
pub const D3D10_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
pub const D3D10_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
pub const D3D10_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
#[repr(C)]
pub struct D3D10_MESSAGE(i32);
#[repr(C)]
pub struct D3D10_MESSAGE_CATEGORY(i32);
#[repr(C)]
pub struct D3D10_MESSAGE_ID(i32);
#[repr(C)]
pub struct D3D10_MESSAGE_SEVERITY(i32);
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_PASS_DESC(i32);
#[repr(C)]
pub struct D3D10_PASS_SHADER_DESC(i32);
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
#[repr(C)]
pub struct D3D10_QUERY(i32);
#[repr(C)]
pub struct D3D10_QUERY_DATA_PIPELINE_STATISTICS(i32);
#[repr(C)]
pub struct D3D10_QUERY_DATA_SO_STATISTICS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_QUERY_DATA_TIMESTAMP_DISJOINT(i32);
#[repr(C)]
pub struct D3D10_QUERY_DESC(i32);
#[repr(C)]
pub struct D3D10_QUERY_MISC_FLAG(i32);
#[repr(C)]
pub struct D3D10_RAISE_FLAG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_RASTERIZER_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_RENDER_TARGET_BLEND_DESC1(i32);
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[repr(C)]
pub struct D3D10_RENDER_TARGET_VIEW_DESC(i32);
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
#[repr(C)]
pub struct D3D10_RESOURCE_DIMENSION(i32);
#[repr(C)]
pub struct D3D10_RESOURCE_MISC_FLAG(i32);
#[repr(C)]
pub struct D3D10_RTV_DIMENSION(i32);
#[repr(C)]
pub struct D3D10_SAMPLER_DESC(i32);
pub const D3D10_SDK_LAYERS_VERSION: u32 = 11u32;
pub const D3D10_SDK_VERSION: u32 = 29u32;
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[repr(C)]
pub struct D3D10_SHADER_BUFFER_DESC(i32);
pub const D3D10_SHADER_DEBUG: u32 = 1u32;
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_FILE_INFO(i32);
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_INFO(i32);
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_INPUT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_INST_INFO(i32);
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_OUTPUTREG_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_OUTPUTVAR(i32);
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_REGTYPE(i32);
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_SCOPETYPE(i32);
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_SCOPEVAR_INFO(i32);
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_SCOPE_INFO(i32);
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_TOKEN_INFO(i32);
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_VARTYPE(i32);
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[repr(C)]
pub struct D3D10_SHADER_DEBUG_VAR_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[repr(C)]
pub struct D3D10_SHADER_DESC(i32);
pub const D3D10_SHADER_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
pub const D3D10_SHADER_ENABLE_STRICTNESS: u32 = 2048u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
pub const D3D10_SHADER_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
pub const D3D10_SHADER_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
pub const D3D10_SHADER_IEEE_STRICTNESS: u32 = 8192u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[repr(C)]
pub struct D3D10_SHADER_INPUT_BIND_DESC(i32);
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
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[repr(C)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC(i32);
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
#[repr(C)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC1(i32);
pub const D3D10_SHADER_SKIP_OPTIMIZATION: u32 = 4u32;
pub const D3D10_SHADER_SKIP_VALIDATION: u32 = 2u32;
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[repr(C)]
pub struct D3D10_SHADER_TYPE_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_SHADER_VARIABLE_DESC(i32);
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144u32;
pub const D3D10_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
#[repr(C)]
pub struct D3D10_SIGNATURE_PARAMETER_DESC(i32);
pub const D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D10_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D10_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256u32;
pub const D3D10_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D10_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_SO_DECLARATION_ENTRY(i32);
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
#[repr(C)]
pub struct D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS(i32);
pub const D3D10_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D10_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D10_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D10_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 16u32;
pub const D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
#[repr(C)]
pub struct D3D10_STATE_BLOCK_MASK(i32);
#[repr(C)]
pub struct D3D10_STENCIL_OP(i32);
pub const D3D10_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[repr(C)]
pub struct D3D10_SUBRESOURCE_DATA(i32);
pub const D3D10_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D10_TECHNIQUE_DESC(i32);
#[repr(C)]
pub struct D3D10_TEX1D_ARRAY_DSV(i32);
#[repr(C)]
pub struct D3D10_TEX1D_ARRAY_RTV(i32);
#[repr(C)]
pub struct D3D10_TEX1D_ARRAY_SRV(i32);
#[repr(C)]
pub struct D3D10_TEX1D_DSV(i32);
#[repr(C)]
pub struct D3D10_TEX1D_RTV(i32);
#[repr(C)]
pub struct D3D10_TEX1D_SRV(i32);
#[repr(C)]
pub struct D3D10_TEX2DMS_ARRAY_DSV(i32);
#[repr(C)]
pub struct D3D10_TEX2DMS_ARRAY_RTV(i32);
#[repr(C)]
pub struct D3D10_TEX2DMS_ARRAY_SRV(i32);
#[repr(C)]
pub struct D3D10_TEX2DMS_DSV(i32);
#[repr(C)]
pub struct D3D10_TEX2DMS_RTV(i32);
#[repr(C)]
pub struct D3D10_TEX2DMS_SRV(i32);
#[repr(C)]
pub struct D3D10_TEX2D_ARRAY_DSV(i32);
#[repr(C)]
pub struct D3D10_TEX2D_ARRAY_RTV(i32);
#[repr(C)]
pub struct D3D10_TEX2D_ARRAY_SRV(i32);
#[repr(C)]
pub struct D3D10_TEX2D_DSV(i32);
#[repr(C)]
pub struct D3D10_TEX2D_RTV(i32);
#[repr(C)]
pub struct D3D10_TEX2D_SRV(i32);
#[repr(C)]
pub struct D3D10_TEX3D_RTV(i32);
#[repr(C)]
pub struct D3D10_TEX3D_SRV(i32);
#[repr(C)]
pub struct D3D10_TEXCUBE_ARRAY_SRV1(i32);
#[repr(C)]
pub struct D3D10_TEXCUBE_SRV(i32);
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18u32;
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[repr(C)]
pub struct D3D10_TEXTURE1D_DESC(i32);
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[repr(C)]
pub struct D3D10_TEXTURE2D_DESC(i32);
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[repr(C)]
pub struct D3D10_TEXTURE3D_DESC(i32);
#[repr(C)]
pub struct D3D10_TEXTURECUBE_FACE(i32);
#[repr(C)]
pub struct D3D10_TEXTURE_ADDRESS_MODE(i32);
pub const D3D10_TEXT_1BIT_BIT: u32 = 2147483648u32;
pub const D3D10_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
#[repr(C)]
pub struct D3D10_USAGE(i32);
#[repr(C)]
pub struct D3D10_VIEWPORT(i32);
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
pub const DXGI_DEBUG_D3D10: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 607865938, data2: 13830, data3: 19770, data4: [153, 215, 167, 231, 179, 62, 215, 6] };
pub const GUID_DeviceType: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3609393997, data2: 31336, data3: 17274, data4: [178, 12, 88, 4, 238, 36, 148, 166] };
#[repr(transparent)]
pub struct ID3D10Asynchronous(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10BlendState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10BlendState1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Buffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Counter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Debug(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10DepthStencilState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10DepthStencilView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Device(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Device1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10DeviceChild(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Effect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectBlendVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectConstantBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectDepthStencilVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectDepthStencilViewVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectMatrixVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectPass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectPool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectRasterizerVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectRenderTargetViewVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectSamplerVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectScalarVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectShaderResourceVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectShaderVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectStringVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectTechnique(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10EffectVectorVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10GeometryShader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10InfoQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10InputLayout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Multithread(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10PixelShader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Predicate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Query(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10RasterizerState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10RenderTargetView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Resource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10SamplerState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10ShaderReflection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10ShaderReflection1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10ShaderReflectionConstantBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10ShaderReflectionType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10ShaderReflectionVariable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10ShaderResourceView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10ShaderResourceView1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10StateBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10SwitchToRef(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Texture1D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Texture2D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10Texture3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10VertexShader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID3D10View(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PFN_D3D10_CREATE_DEVICE1(i32);
#[repr(C)]
pub struct PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1(i32);
pub const _FACD3D10: u32 = 2169u32;
