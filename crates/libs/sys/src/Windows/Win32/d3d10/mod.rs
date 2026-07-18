#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10CompileEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, psrcfilename : windows_sys::core::PCSTR, pdefines : *const D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, hlslflags : u32, fxflags : u32, ppcompiledeffect : *mut *mut core::ffi::c_void, pperrors : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10CompileShader(psrcdata : windows_sys::core::PCSTR, srcdatasize : usize, pfilename : windows_sys::core::PCSTR, pdefines : *const D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pfunctionname : windows_sys::core::PCSTR, pprofile : windows_sys::core::PCSTR, flags : u32, ppshader : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateBlob(numbytes : usize, ppbuffer : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "dxgi", feature = "minwindef"))]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateDevice(padapter : *mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::HMODULE, flags : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "dxgi", feature = "minwindef"))]
windows_link::link!("d3d10_1.dll" "system" fn D3D10CreateDevice1(padapter : *mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "dxgi", feature = "minwindef", feature = "windef"))]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateDeviceAndSwapChain(padapter : *mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::HMODULE, flags : u32, sdkversion : u32, pswapchaindesc : *const super::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "dxgi", feature = "minwindef", feature = "windef"))]
windows_link::link!("d3d10_1.dll" "system" fn D3D10CreateDeviceAndSwapChain1(padapter : *mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, pswapchaindesc : *const super::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10CreateEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : *mut core::ffi::c_void, peffectpool : *mut core::ffi::c_void, ppeffect : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10CreateEffectPoolFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : *mut core::ffi::c_void, ppeffectpool : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10CreateStateBlock(pdevice : *mut core::ffi::c_void, pstateblockmask : *const D3D10_STATE_BLOCK_MASK, ppstateblock : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10DisassembleEffect(peffect : *mut core::ffi::c_void, enablecolorcode : windows_sys::core::BOOL, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10DisassembleShader(pshader : *const core::ffi::c_void, bytecodelength : usize, enablecolorcode : windows_sys::core::BOOL, pcomments : windows_sys::core::PCSTR, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10GetGeometryShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_sys::core::PCSTR);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetInputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10GetPixelShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_sys::core::PCSTR);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetShaderDebugInfo(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppdebuginfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10GetVertexShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_sys::core::PCSTR);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10PreprocessShader(psrcdata : windows_sys::core::PCSTR, srcdatasize : usize, pfilename : windows_sys::core::PCSTR, pdefines : *const D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, ppshadertext : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10ReflectShader(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppreflector : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDifference(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskGetSetting(pmask : *const D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, entry : u32) -> windows_sys::core::BOOL);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskIntersect(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskUnion(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D10_BUFFER_DESC {
    pub Base: D3D10_BUFFER_DESC,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct CD3D10_TEXTURE1D_DESC {
    pub Base: D3D10_TEXTURE1D_DESC,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct CD3D10_TEXTURE2D_DESC {
    pub Base: D3D10_TEXTURE2D_DESC,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct CD3D10_TEXTURE3D_DESC {
    pub Base: D3D10_TEXTURE3D_DESC,
}
pub const D3D10_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535;
pub const D3D10_1_DEFAULT_SAMPLE_MASK: u32 = 4294967295;
pub const D3D10_1_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6;
pub const D3D10_1_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6;
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
pub const D3D10_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295;
pub const D3D10_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255;
pub const D3D10_ALL_RESOURCES_BOUND: u32 = 2097152;
pub const D3D10_ANISOTROPIC_FILTERING_BIT: u32 = 64;
pub const D3D10_APPEND_ALIGNED_ELEMENT: u32 = 4294967295;
pub const D3D10_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9;
pub const D3D10_ASYNC_GETDATA_DONOTFLUSH: D3D10_ASYNC_GETDATA_FLAG = 1;
pub type D3D10_ASYNC_GETDATA_FLAG = i32;
pub const D3D10_BIND_CONSTANT_BUFFER: D3D10_BIND_FLAG = 4;
pub const D3D10_BIND_DEPTH_STENCIL: D3D10_BIND_FLAG = 64;
pub type D3D10_BIND_FLAG = i32;
pub const D3D10_BIND_INDEX_BUFFER: D3D10_BIND_FLAG = 2;
pub const D3D10_BIND_RENDER_TARGET: D3D10_BIND_FLAG = 32;
pub const D3D10_BIND_SHADER_RESOURCE: D3D10_BIND_FLAG = 8;
pub const D3D10_BIND_STREAM_OUTPUT: D3D10_BIND_FLAG = 16;
pub const D3D10_BIND_VERTEX_BUFFER: D3D10_BIND_FLAG = 1;
pub type D3D10_BLEND = i32;
pub const D3D10_BLEND_BLEND_FACTOR: D3D10_BLEND = 14;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_BLEND_DESC {
    pub AlphaToCoverageEnable: windows_sys::core::BOOL,
    pub BlendEnable: [windows_sys::core::BOOL; 8],
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: [u8; 8],
}
impl Default for D3D10_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_BLEND_DESC1 {
    pub AlphaToCoverageEnable: windows_sys::core::BOOL,
    pub IndependentBlendEnable: windows_sys::core::BOOL,
    pub RenderTarget: [D3D10_RENDER_TARGET_BLEND_DESC1; 8],
}
impl Default for D3D10_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_BLEND_DEST_ALPHA: D3D10_BLEND = 7;
pub const D3D10_BLEND_DEST_COLOR: D3D10_BLEND = 9;
pub const D3D10_BLEND_INV_BLEND_FACTOR: D3D10_BLEND = 15;
pub const D3D10_BLEND_INV_DEST_ALPHA: D3D10_BLEND = 8;
pub const D3D10_BLEND_INV_DEST_COLOR: D3D10_BLEND = 10;
pub const D3D10_BLEND_INV_SRC1_ALPHA: D3D10_BLEND = 19;
pub const D3D10_BLEND_INV_SRC1_COLOR: D3D10_BLEND = 17;
pub const D3D10_BLEND_INV_SRC_ALPHA: D3D10_BLEND = 6;
pub const D3D10_BLEND_INV_SRC_COLOR: D3D10_BLEND = 4;
pub const D3D10_BLEND_ONE: D3D10_BLEND = 2;
pub type D3D10_BLEND_OP = i32;
pub const D3D10_BLEND_OP_ADD: D3D10_BLEND_OP = 1;
pub const D3D10_BLEND_OP_MAX: D3D10_BLEND_OP = 5;
pub const D3D10_BLEND_OP_MIN: D3D10_BLEND_OP = 4;
pub const D3D10_BLEND_OP_REV_SUBTRACT: D3D10_BLEND_OP = 3;
pub const D3D10_BLEND_OP_SUBTRACT: D3D10_BLEND_OP = 2;
pub const D3D10_BLEND_SRC1_ALPHA: D3D10_BLEND = 18;
pub const D3D10_BLEND_SRC1_COLOR: D3D10_BLEND = 16;
pub const D3D10_BLEND_SRC_ALPHA: D3D10_BLEND = 5;
pub const D3D10_BLEND_SRC_ALPHA_SAT: D3D10_BLEND = 11;
pub const D3D10_BLEND_SRC_COLOR: D3D10_BLEND = 3;
pub const D3D10_BLEND_ZERO: D3D10_BLEND = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_BUFFER_RTV {
    pub Anonymous: D3D10_BUFFER_RTV_0,
    pub Anonymous2: D3D10_BUFFER_RTV_1,
}
impl Default for D3D10_BUFFER_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D10_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D10_BUFFER_RTV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D10_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D10_BUFFER_RTV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_BUFFER_SRV {
    pub Anonymous: D3D10_BUFFER_SRV_0,
    pub Anonymous2: D3D10_BUFFER_SRV_1,
}
impl Default for D3D10_BUFFER_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D10_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D10_BUFFER_SRV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D10_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D10_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "d3dcommon")]
pub type D3D10_CBUFFER_TYPE = super::D3D_CBUFFER_TYPE;
pub const D3D10_CENTER_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -2;
pub const D3D10_CLEAR_DEPTH: D3D10_CLEAR_FLAG = 1;
pub type D3D10_CLEAR_FLAG = i32;
pub const D3D10_CLEAR_STENCIL: D3D10_CLEAR_FLAG = 2;
pub const D3D10_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8;
pub const D3D10_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2;
pub type D3D10_COLOR_WRITE_ENABLE = i32;
pub const D3D10_COLOR_WRITE_ENABLE_ALL: D3D10_COLOR_WRITE_ENABLE = 15;
pub const D3D10_COLOR_WRITE_ENABLE_ALPHA: D3D10_COLOR_WRITE_ENABLE = 8;
pub const D3D10_COLOR_WRITE_ENABLE_BLUE: D3D10_COLOR_WRITE_ENABLE = 4;
pub const D3D10_COLOR_WRITE_ENABLE_GREEN: D3D10_COLOR_WRITE_ENABLE = 2;
pub const D3D10_COLOR_WRITE_ENABLE_RED: D3D10_COLOR_WRITE_ENABLE = 1;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D10_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D10_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1;
pub const D3D10_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16;
pub const D3D10_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3;
pub const D3D10_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3;
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10;
pub const D3D10_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10;
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8;
pub const D3D10_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7;
pub const D3D10_COMPARISON_ALWAYS: D3D10_COMPARISON_FUNC = 8;
pub const D3D10_COMPARISON_EQUAL: D3D10_COMPARISON_FUNC = 3;
pub const D3D10_COMPARISON_FILTERING_BIT: u32 = 128;
pub type D3D10_COMPARISON_FUNC = i32;
pub const D3D10_COMPARISON_GREATER: D3D10_COMPARISON_FUNC = 5;
pub const D3D10_COMPARISON_GREATER_EQUAL: D3D10_COMPARISON_FUNC = 7;
pub const D3D10_COMPARISON_LESS: D3D10_COMPARISON_FUNC = 2;
pub const D3D10_COMPARISON_LESS_EQUAL: D3D10_COMPARISON_FUNC = 4;
pub const D3D10_COMPARISON_NEVER: D3D10_COMPARISON_FUNC = 1;
pub const D3D10_COMPARISON_NOT_EQUAL: D3D10_COMPARISON_FUNC = 6;
pub type D3D10_COUNTER = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_COUNTER_DESC {
    pub Counter: D3D10_COUNTER,
    pub MiscFlags: u32,
}
pub const D3D10_COUNTER_DEVICE_DEPENDENT_0: D3D10_COUNTER = 1073741824;
pub const D3D10_COUNTER_FILLRATE_THROUGHPUT_UTILIZATION: D3D10_COUNTER = 9;
pub const D3D10_COUNTER_GEOMETRY_PROCESSING: D3D10_COUNTER = 2;
pub const D3D10_COUNTER_GPU_IDLE: D3D10_COUNTER = 0;
pub const D3D10_COUNTER_GS_COMPUTATION_LIMITED: D3D10_COUNTER = 13;
pub const D3D10_COUNTER_GS_MEMORY_LIMITED: D3D10_COUNTER = 12;
pub const D3D10_COUNTER_HOST_ADAPTER_BANDWIDTH_UTILIZATION: D3D10_COUNTER = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D10_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
pub const D3D10_COUNTER_LOCAL_VIDMEM_BANDWIDTH_UTILIZATION: D3D10_COUNTER = 6;
pub const D3D10_COUNTER_OTHER_GPU_PROCESSING: D3D10_COUNTER = 4;
pub const D3D10_COUNTER_PIXEL_PROCESSING: D3D10_COUNTER = 3;
pub const D3D10_COUNTER_POST_TRANSFORM_CACHE_HIT_RATE: D3D10_COUNTER = 16;
pub const D3D10_COUNTER_PS_COMPUTATION_LIMITED: D3D10_COUNTER = 15;
pub const D3D10_COUNTER_PS_MEMORY_LIMITED: D3D10_COUNTER = 14;
pub const D3D10_COUNTER_TEXTURE_CACHE_HIT_RATE: D3D10_COUNTER = 17;
pub const D3D10_COUNTER_TRIANGLE_SETUP_THROUGHPUT_UTILIZATION: D3D10_COUNTER = 8;
pub type D3D10_COUNTER_TYPE = i32;
pub const D3D10_COUNTER_TYPE_FLOAT32: D3D10_COUNTER_TYPE = 0;
pub const D3D10_COUNTER_TYPE_UINT16: D3D10_COUNTER_TYPE = 1;
pub const D3D10_COUNTER_TYPE_UINT32: D3D10_COUNTER_TYPE = 2;
pub const D3D10_COUNTER_TYPE_UINT64: D3D10_COUNTER_TYPE = 3;
pub const D3D10_COUNTER_VERTEX_PROCESSING: D3D10_COUNTER = 1;
pub const D3D10_COUNTER_VERTEX_THROUGHPUT_UTILIZATION: D3D10_COUNTER = 7;
pub const D3D10_COUNTER_VS_COMPUTATION_LIMITED: D3D10_COUNTER = 11;
pub const D3D10_COUNTER_VS_MEMORY_LIMITED: D3D10_COUNTER = 10;
pub type D3D10_CPU_ACCESS_FLAG = i32;
pub const D3D10_CPU_ACCESS_READ: D3D10_CPU_ACCESS_FLAG = 131072;
pub const D3D10_CPU_ACCESS_WRITE: D3D10_CPU_ACCESS_FLAG = 65536;
pub const D3D10_CREATE_DEVICE_ALLOW_NULL_FROM_MAP: D3D10_CREATE_DEVICE_FLAG = 16;
pub const D3D10_CREATE_DEVICE_BGRA_SUPPORT: D3D10_CREATE_DEVICE_FLAG = 32;
pub const D3D10_CREATE_DEVICE_DEBUG: D3D10_CREATE_DEVICE_FLAG = 2;
pub const D3D10_CREATE_DEVICE_DEBUGGABLE: D3D10_CREATE_DEVICE_FLAG = 1024;
pub type D3D10_CREATE_DEVICE_FLAG = i32;
pub const D3D10_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D10_CREATE_DEVICE_FLAG = 128;
pub const D3D10_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D10_CREATE_DEVICE_FLAG = 8;
pub const D3D10_CREATE_DEVICE_SINGLETHREADED: D3D10_CREATE_DEVICE_FLAG = 1;
pub const D3D10_CREATE_DEVICE_STRICT_VALIDATION: D3D10_CREATE_DEVICE_FLAG = 512;
pub const D3D10_CREATE_DEVICE_SWITCH_TO_REF: D3D10_CREATE_DEVICE_FLAG = 4;
pub const D3D10_CULL_BACK: D3D10_CULL_MODE = 3;
pub const D3D10_CULL_FRONT: D3D10_CULL_MODE = 2;
pub type D3D10_CULL_MODE = i32;
pub const D3D10_CULL_NONE: D3D10_CULL_MODE = 1;
pub const D3D10_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2;
pub const D3D10_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1;
pub const D3D10_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4;
pub const D3D10_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1.0;
pub const D3D10_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1.0;
pub const D3D10_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1.0;
pub const D3D10_DEFAULT_BLEND_FACTOR_RED: f32 = 1.0;
pub const D3D10_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0.0;
pub const D3D10_DEFAULT_DEPTH_BIAS: u32 = 0;
pub const D3D10_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0.0;
pub const D3D10_DEFAULT_MAX_ANISOTROPY: f32 = 16.0;
pub const D3D10_DEFAULT_MIP_LOD_BIAS: f32 = 0.0;
pub const D3D10_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0;
pub const D3D10_DEFAULT_SAMPLE_MASK: u32 = 4294967295;
pub const D3D10_DEFAULT_SCISSOR_ENDX: u32 = 0;
pub const D3D10_DEFAULT_SCISSOR_ENDY: u32 = 0;
pub const D3D10_DEFAULT_SCISSOR_STARTX: u32 = 0;
pub const D3D10_DEFAULT_SCISSOR_STARTY: u32 = 0;
pub const D3D10_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0.0;
pub const D3D10_DEFAULT_STENCIL_READ_MASK: u32 = 255;
pub const D3D10_DEFAULT_STENCIL_REFERENCE: u32 = 0;
pub const D3D10_DEFAULT_STENCIL_WRITE_MASK: u32 = 255;
pub const D3D10_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0;
pub const D3D10_DEFAULT_VIEWPORT_HEIGHT: u32 = 0;
pub const D3D10_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0.0;
pub const D3D10_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0.0;
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0;
pub const D3D10_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0;
pub const D3D10_DEFAULT_VIEWPORT_WIDTH: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D10_STENCIL_OP,
    pub StencilDepthFailOp: D3D10_STENCIL_OP,
    pub StencilPassOp: D3D10_STENCIL_OP,
    pub StencilFunc: D3D10_COMPARISON_FUNC,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_DEPTH_STENCIL_DESC {
    pub DepthEnable: windows_sys::core::BOOL,
    pub DepthWriteMask: D3D10_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D10_COMPARISON_FUNC,
    pub StencilEnable: windows_sys::core::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D10_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D10_DEPTH_STENCILOP_DESC,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D10_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::DXGI_FORMAT,
    pub ViewDimension: D3D10_DSV_DIMENSION,
    pub Anonymous: D3D10_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "dxgi")]
impl Default for D3D10_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub union D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D10_TEX1D_DSV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D10_TEX2D_DSV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D10_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "dxgi")]
impl Default for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D10_DEPTH_WRITE_MASK = i32;
pub const D3D10_DEPTH_WRITE_MASK_ALL: D3D10_DEPTH_WRITE_MASK = 1;
pub const D3D10_DEPTH_WRITE_MASK_ZERO: D3D10_DEPTH_WRITE_MASK = 0;
pub type D3D10_DEVICE_STATE_TYPES = i32;
pub type D3D10_DRIVER_TYPE = i32;
pub const D3D10_DRIVER_TYPE_HARDWARE: D3D10_DRIVER_TYPE = 0;
pub const D3D10_DRIVER_TYPE_NULL: D3D10_DRIVER_TYPE = 2;
pub const D3D10_DRIVER_TYPE_REFERENCE: D3D10_DRIVER_TYPE = 1;
pub const D3D10_DRIVER_TYPE_SOFTWARE: D3D10_DRIVER_TYPE = 3;
pub const D3D10_DRIVER_TYPE_WARP: D3D10_DRIVER_TYPE = 5;
pub const D3D10_DST_GS: D3D10_DEVICE_STATE_TYPES = 9;
pub const D3D10_DST_GS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 12;
pub const D3D10_DST_GS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 10;
pub const D3D10_DST_GS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 11;
pub const D3D10_DST_IA_INDEX_BUFFER: D3D10_DEVICE_STATE_TYPES = 18;
pub const D3D10_DST_IA_INPUT_LAYOUT: D3D10_DEVICE_STATE_TYPES = 19;
pub const D3D10_DST_IA_PRIMITIVE_TOPOLOGY: D3D10_DEVICE_STATE_TYPES = 20;
pub const D3D10_DST_IA_VERTEX_BUFFERS: D3D10_DEVICE_STATE_TYPES = 17;
pub const D3D10_DST_OM_BLEND_STATE: D3D10_DEVICE_STATE_TYPES = 4;
pub const D3D10_DST_OM_DEPTH_STENCIL_STATE: D3D10_DEVICE_STATE_TYPES = 3;
pub const D3D10_DST_OM_RENDER_TARGETS: D3D10_DEVICE_STATE_TYPES = 2;
pub const D3D10_DST_PREDICATION: D3D10_DEVICE_STATE_TYPES = 24;
pub const D3D10_DST_PS: D3D10_DEVICE_STATE_TYPES = 13;
pub const D3D10_DST_PS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 16;
pub const D3D10_DST_PS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 14;
pub const D3D10_DST_PS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 15;
pub const D3D10_DST_RS_RASTERIZER_STATE: D3D10_DEVICE_STATE_TYPES = 23;
pub const D3D10_DST_RS_SCISSOR_RECTS: D3D10_DEVICE_STATE_TYPES = 22;
pub const D3D10_DST_RS_VIEWPORTS: D3D10_DEVICE_STATE_TYPES = 21;
pub const D3D10_DST_SO_BUFFERS: D3D10_DEVICE_STATE_TYPES = 1;
pub const D3D10_DST_VS: D3D10_DEVICE_STATE_TYPES = 5;
pub const D3D10_DST_VS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 8;
pub const D3D10_DST_VS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 6;
pub const D3D10_DST_VS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 7;
pub type D3D10_DSV_DIMENSION = i32;
pub const D3D10_DSV_DIMENSION_TEXTURE1D: D3D10_DSV_DIMENSION = 1;
pub const D3D10_DSV_DIMENSION_TEXTURE1DARRAY: D3D10_DSV_DIMENSION = 2;
pub const D3D10_DSV_DIMENSION_TEXTURE2D: D3D10_DSV_DIMENSION = 3;
pub const D3D10_DSV_DIMENSION_TEXTURE2DARRAY: D3D10_DSV_DIMENSION = 4;
pub const D3D10_DSV_DIMENSION_TEXTURE2DMS: D3D10_DSV_DIMENSION = 5;
pub const D3D10_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D10_DSV_DIMENSION = 6;
pub const D3D10_DSV_DIMENSION_UNKNOWN: D3D10_DSV_DIMENSION = 0;
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: u32 = 2;
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_EFFECT_DESC {
    pub IsChildEffect: windows_sys::core::BOOL,
    pub ConstantBuffers: u32,
    pub SharedConstantBuffers: u32,
    pub GlobalVariables: u32,
    pub SharedGlobalVariables: u32,
    pub Techniques: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_EFFECT_SHADER_DESC {
    pub pInputSignature: *const u8,
    pub IsInline: windows_sys::core::BOOL,
    pub pBytecode: *const u8,
    pub BytecodeLength: u32,
    pub SODecl: windows_sys::core::PCSTR,
    pub NumInputSignatureEntries: u32,
    pub NumOutputSignatureEntries: u32,
}
impl Default for D3D10_EFFECT_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_SINGLE_THREADED: u32 = 8;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D10_EFFECT_TYPE_DESC {
    pub TypeName: windows_sys::core::PCSTR,
    pub Class: D3D10_SHADER_VARIABLE_CLASS,
    pub Type: D3D10_SHADER_VARIABLE_TYPE,
    pub Elements: u32,
    pub Members: u32,
    pub Rows: u32,
    pub Columns: u32,
    pub PackedSize: u32,
    pub UnpackedSize: u32,
    pub Stride: u32,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D10_EFFECT_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_EFFECT_VARIABLE_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Semantic: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub Annotations: u32,
    pub BufferOffset: u32,
    pub ExplicitBindPoint: u32,
}
impl Default for D3D10_EFFECT_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4;
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1;
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576;
pub type D3D10_FEATURE_LEVEL1 = i32;
pub const D3D10_FEATURE_LEVEL_10_0: D3D10_FEATURE_LEVEL1 = 40960;
pub const D3D10_FEATURE_LEVEL_10_1: D3D10_FEATURE_LEVEL1 = 41216;
pub const D3D10_FEATURE_LEVEL_9_1: D3D10_FEATURE_LEVEL1 = 37120;
pub const D3D10_FEATURE_LEVEL_9_2: D3D10_FEATURE_LEVEL1 = 37376;
pub const D3D10_FEATURE_LEVEL_9_3: D3D10_FEATURE_LEVEL1 = 37632;
pub type D3D10_FILL_MODE = i32;
pub const D3D10_FILL_SOLID: D3D10_FILL_MODE = 3;
pub const D3D10_FILL_WIREFRAME: D3D10_FILL_MODE = 2;
pub type D3D10_FILTER = i32;
pub const D3D10_FILTER_ANISOTROPIC: D3D10_FILTER = 85;
pub const D3D10_FILTER_COMPARISON_ANISOTROPIC: D3D10_FILTER = 213;
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: D3D10_FILTER = 144;
pub const D3D10_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D10_FILTER = 145;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: D3D10_FILTER = 148;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: D3D10_FILTER = 149;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_MIP_POINT: D3D10_FILTER = 128;
pub const D3D10_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: D3D10_FILTER = 129;
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D10_FILTER = 132;
pub const D3D10_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: D3D10_FILTER = 133;
pub const D3D10_FILTER_MIN_LINEAR_MAG_MIP_POINT: D3D10_FILTER = 16;
pub const D3D10_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D3D10_FILTER = 17;
pub const D3D10_FILTER_MIN_MAG_LINEAR_MIP_POINT: D3D10_FILTER = 20;
pub const D3D10_FILTER_MIN_MAG_MIP_LINEAR: D3D10_FILTER = 21;
pub const D3D10_FILTER_MIN_MAG_MIP_POINT: D3D10_FILTER = 0;
pub const D3D10_FILTER_MIN_MAG_POINT_MIP_LINEAR: D3D10_FILTER = 1;
pub const D3D10_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D3D10_FILTER = 4;
pub const D3D10_FILTER_MIN_POINT_MAG_MIP_LINEAR: D3D10_FILTER = 5;
pub const D3D10_FILTER_TEXT_1BIT: D3D10_FILTER = -2147483648;
pub type D3D10_FILTER_TYPE = i32;
pub const D3D10_FILTER_TYPE_LINEAR: D3D10_FILTER_TYPE = 1;
pub const D3D10_FILTER_TYPE_MASK: u32 = 3;
pub const D3D10_FILTER_TYPE_POINT: D3D10_FILTER_TYPE = 0;
pub const D3D10_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6;
pub const D3D10_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000.0;
pub const D3D10_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6;
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4;
pub const D3D10_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1.0;
pub const D3D10_FLOAT_TO_SRGB_OFFSET: f32 = 0.055;
pub const D3D10_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92;
pub const D3D10_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055;
pub const D3D10_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308;
pub type D3D10_FORMAT_SUPPORT = i32;
pub const D3D10_FORMAT_SUPPORT_BACK_BUFFER_CAST: D3D10_FORMAT_SUPPORT = 16777216;
pub const D3D10_FORMAT_SUPPORT_BLENDABLE: D3D10_FORMAT_SUPPORT = 32768;
pub const D3D10_FORMAT_SUPPORT_BUFFER: D3D10_FORMAT_SUPPORT = 1;
pub const D3D10_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT: D3D10_FORMAT_SUPPORT = 1048576;
pub const D3D10_FORMAT_SUPPORT_CPU_LOCKABLE: D3D10_FORMAT_SUPPORT = 131072;
pub const D3D10_FORMAT_SUPPORT_DEPTH_STENCIL: D3D10_FORMAT_SUPPORT = 65536;
pub const D3D10_FORMAT_SUPPORT_DISPLAY: D3D10_FORMAT_SUPPORT = 524288;
pub const D3D10_FORMAT_SUPPORT_IA_INDEX_BUFFER: D3D10_FORMAT_SUPPORT = 4;
pub const D3D10_FORMAT_SUPPORT_IA_VERTEX_BUFFER: D3D10_FORMAT_SUPPORT = 2;
pub const D3D10_FORMAT_SUPPORT_MIP: D3D10_FORMAT_SUPPORT = 4096;
pub const D3D10_FORMAT_SUPPORT_MIP_AUTOGEN: D3D10_FORMAT_SUPPORT = 8192;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_LOAD: D3D10_FORMAT_SUPPORT = 4194304;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET: D3D10_FORMAT_SUPPORT = 2097152;
pub const D3D10_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE: D3D10_FORMAT_SUPPORT = 262144;
pub const D3D10_FORMAT_SUPPORT_RENDER_TARGET: D3D10_FORMAT_SUPPORT = 16384;
pub const D3D10_FORMAT_SUPPORT_SHADER_GATHER: D3D10_FORMAT_SUPPORT = 8388608;
pub const D3D10_FORMAT_SUPPORT_SHADER_LOAD: D3D10_FORMAT_SUPPORT = 256;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE: D3D10_FORMAT_SUPPORT = 512;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON: D3D10_FORMAT_SUPPORT = 1024;
pub const D3D10_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT: D3D10_FORMAT_SUPPORT = 2048;
pub const D3D10_FORMAT_SUPPORT_SO_BUFFER: D3D10_FORMAT_SUPPORT = 8;
pub const D3D10_FORMAT_SUPPORT_TEXTURE1D: D3D10_FORMAT_SUPPORT = 16;
pub const D3D10_FORMAT_SUPPORT_TEXTURE2D: D3D10_FORMAT_SUPPORT = 32;
pub const D3D10_FORMAT_SUPPORT_TEXTURE3D: D3D10_FORMAT_SUPPORT = 64;
pub const D3D10_FORMAT_SUPPORT_TEXTURECUBE: D3D10_FORMAT_SUPPORT = 128;
pub const D3D10_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600.0;
pub const D3D10_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300.0;
pub const D3D10_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0.0;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D10_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_GS_INPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_GS_INPUT_REGISTER_COUNT: u32 = 16;
pub const D3D10_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D10_GS_INPUT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_GS_INPUT_REGISTER_VERTICES: u32 = 6;
pub const D3D10_GS_OUTPUT_ELEMENTS: u32 = 32;
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_GS_OUTPUT_REGISTER_COUNT: u32 = 32;
pub const D3D10_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0;
pub const D3D10_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0;
pub const D3D10_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0;
pub const D3D10_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1;
pub const D3D10_IA_INSTANCE_ID_BIT_COUNT: u32 = 32;
pub const D3D10_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32;
pub const D3D10_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32;
pub const D3D10_IA_VERTEX_ID_BIT_COUNT: u32 = 32;
pub const D3D10_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 16;
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 64;
pub const D3D10_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 16;
#[cfg(feature = "d3dcommon")]
pub type D3D10_INCLUDE_TYPE = super::D3D_INCLUDE_TYPE;
pub const D3D10_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_INFO_QUEUE_FILTER {
    pub AllowList: D3D10_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D10_INFO_QUEUE_FILTER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D10_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D10_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D10_MESSAGE_ID,
}
impl Default for D3D10_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D10_INPUT_CLASSIFICATION = i32;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D10_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D10_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "dxgi")]
impl Default for D3D10_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_INPUT_PER_INSTANCE_DATA: D3D10_INPUT_CLASSIFICATION = 1;
pub const D3D10_INPUT_PER_VERTEX_DATA: D3D10_INPUT_CLASSIFICATION = 0;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295;
pub const D3D10_LINEAR_GAMMA: f32 = 1.0;
pub const D3D10_MAG_FILTER_SHIFT: u32 = 2;
pub type D3D10_MAP = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_MAPPED_TEXTURE2D {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
}
impl Default for D3D10_MAPPED_TEXTURE2D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_MAPPED_TEXTURE3D {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl Default for D3D10_MAPPED_TEXTURE3D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D10_MAP_FLAG = i32;
pub const D3D10_MAP_FLAG_DO_NOT_WAIT: D3D10_MAP_FLAG = 1048576;
pub const D3D10_MAP_READ: D3D10_MAP = 1;
pub const D3D10_MAP_READ_WRITE: D3D10_MAP = 3;
pub const D3D10_MAP_WRITE: D3D10_MAP = 2;
pub const D3D10_MAP_WRITE_DISCARD: D3D10_MAP = 4;
pub const D3D10_MAP_WRITE_NO_OVERWRITE: D3D10_MAP = 5;
pub const D3D10_MAX_BORDER_COLOR_COMPONENT: f32 = 1.0;
pub const D3D10_MAX_DEPTH: f32 = 1.0;
pub const D3D10_MAX_MAXANISOTROPY: u32 = 16;
pub const D3D10_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32;
pub const D3D10_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000.0;
pub const D3D10_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_MESSAGE {
    pub Category: D3D10_MESSAGE_CATEGORY,
    pub Severity: D3D10_MESSAGE_SEVERITY,
    pub ID: D3D10_MESSAGE_ID,
    pub pDescription: *const i8,
    pub DescriptionByteLength: usize,
}
impl Default for D3D10_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D10_MESSAGE_CATEGORY = i32;
pub const D3D10_MESSAGE_CATEGORY_APPLICATION_DEFINED: D3D10_MESSAGE_CATEGORY = 0;
pub const D3D10_MESSAGE_CATEGORY_CLEANUP: D3D10_MESSAGE_CATEGORY = 3;
pub const D3D10_MESSAGE_CATEGORY_COMPILATION: D3D10_MESSAGE_CATEGORY = 4;
pub const D3D10_MESSAGE_CATEGORY_EXECUTION: D3D10_MESSAGE_CATEGORY = 9;
pub const D3D10_MESSAGE_CATEGORY_INITIALIZATION: D3D10_MESSAGE_CATEGORY = 2;
pub const D3D10_MESSAGE_CATEGORY_MISCELLANEOUS: D3D10_MESSAGE_CATEGORY = 1;
pub const D3D10_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: D3D10_MESSAGE_CATEGORY = 8;
pub const D3D10_MESSAGE_CATEGORY_SHADER: D3D10_MESSAGE_CATEGORY = 10;
pub const D3D10_MESSAGE_CATEGORY_STATE_CREATION: D3D10_MESSAGE_CATEGORY = 5;
pub const D3D10_MESSAGE_CATEGORY_STATE_GETTING: D3D10_MESSAGE_CATEGORY = 7;
pub const D3D10_MESSAGE_CATEGORY_STATE_SETTING: D3D10_MESSAGE_CATEGORY = 6;
pub type D3D10_MESSAGE_ID = i32;
pub const D3D10_MESSAGE_ID_BLENDSTATE_GETDESC_LEGACY: D3D10_MESSAGE_ID = 392;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = 297;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 298;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = 296;
pub const D3D10_MESSAGE_ID_BUFFER_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = 295;
pub const D3D10_MESSAGE_ID_BUFFER_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = 299;
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_OUTOFRANGE_COUNTER: D3D10_MESSAGE_ID = 402;
pub const D3D10_MESSAGE_ID_CHECKCOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D10_MESSAGE_ID = 403;
pub const D3D10_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_DEPRECATED: D3D10_MESSAGE_ID = 321;
pub const D3D10_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_FORMAT_DEPRECATED: D3D10_MESSAGE_ID = 322;
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DENORMFLUSH: D3D10_MESSAGE_ID = 262;
pub const D3D10_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: D3D10_MESSAGE_ID = 263;
pub const D3D10_MESSAGE_ID_CLEARRENDERTARGETVIEW_DENORMFLUSH: D3D10_MESSAGE_ID = 261;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = 285;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCE: D3D10_MESSAGE_ID = 284;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCESTATE: D3D10_MESSAGE_ID = 286;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_3D_MISMATCHED_UPDATES: D3D10_MESSAGE_ID = 1048637;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_3D_READBACK: D3D10_MESSAGE_ID = 1048597;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_ONLY_READBACK: D3D10_MESSAGE_ID = 1048598;
pub const D3D10_MESSAGE_ID_COPYRESOURCE_ONLY_TEXTURE_2D_WITHIN_GPU_MEMORY: D3D10_MESSAGE_ID = 1048596;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = 282;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSUBRESOURCE: D3D10_MESSAGE_ID = 278;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCE: D3D10_MESSAGE_ID = 281;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCEBOX: D3D10_MESSAGE_ID = 280;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESTATE: D3D10_MESSAGE_ID = 283;
pub const D3D10_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESUBRESOURCE: D3D10_MESSAGE_ID = 279;
pub const D3D10_MESSAGE_ID_CORRUPTED_MULTITHREADING: D3D10_MESSAGE_ID = 28;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER1: D3D10_MESSAGE_ID = 13;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER10: D3D10_MESSAGE_ID = 22;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER11: D3D10_MESSAGE_ID = 23;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER12: D3D10_MESSAGE_ID = 24;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER13: D3D10_MESSAGE_ID = 25;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER14: D3D10_MESSAGE_ID = 26;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER15: D3D10_MESSAGE_ID = 27;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER2: D3D10_MESSAGE_ID = 14;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER3: D3D10_MESSAGE_ID = 15;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER4: D3D10_MESSAGE_ID = 16;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER5: D3D10_MESSAGE_ID = 17;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER6: D3D10_MESSAGE_ID = 18;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER7: D3D10_MESSAGE_ID = 19;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER8: D3D10_MESSAGE_ID = 20;
pub const D3D10_MESSAGE_ID_CORRUPTED_PARAMETER9: D3D10_MESSAGE_ID = 21;
pub const D3D10_MESSAGE_ID_CORRUPTED_THIS: D3D10_MESSAGE_ID = 12;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: D3D10_MESSAGE_ID = 214;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: D3D10_MESSAGE_ID = 217;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: D3D10_MESSAGE_ID = 213;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: D3D10_MESSAGE_ID = 216;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: D3D10_MESSAGE_ID = 218;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: D3D10_MESSAGE_ID = 212;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: D3D10_MESSAGE_ID = 215;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_ALPHA_TO_COVERAGE: D3D10_MESSAGE_ID = 1048600;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_BLEND_ENABLE: D3D10_MESSAGE_ID = 1048612;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_WRITE_MASKS: D3D10_MESSAGE_ID = 1048613;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_MRT_BLEND: D3D10_MESSAGE_ID = 1048623;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NO_SEPARATE_ALPHA_BLEND: D3D10_MESSAGE_ID = 1048622;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_NULLDESC: D3D10_MESSAGE_ID = 220;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_OPERATION_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048624;
pub const D3D10_MESSAGE_ID_CREATEBLENDSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 219;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 69;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = 64;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCONSTANTBUFFERBINDINGS: D3D10_MESSAGE_ID = 72;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 63;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 66;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDINITIALDATA: D3D10_MESSAGE_ID = 65;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = 67;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 68;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_INVALIDSAMPLES: D3D10_MESSAGE_ID = 58;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_LARGEALLOCATION: D3D10_MESSAGE_ID = 73;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_NULLDESC: D3D10_MESSAGE_ID = 71;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 70;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = 60;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 61;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 57;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = 62;
pub const D3D10_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = 59;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NONEXCLUSIVE_RETURN: D3D10_MESSAGE_ID = 400;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_NULLDESC: D3D10_MESSAGE_ID = 401;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 399;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_OUTOFRANGE_COUNTER: D3D10_MESSAGE_ID = 396;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_SIMULTANEOUS_ACTIVE_COUNTERS_EXHAUSTED: D3D10_MESSAGE_ID = 397;
pub const D3D10_MESSAGE_ID_CREATECOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: D3D10_MESSAGE_ID = 398;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: D3D10_MESSAGE_ID = 206;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: D3D10_MESSAGE_ID = 209;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: D3D10_MESSAGE_ID = 208;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: D3D10_MESSAGE_ID = 207;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: D3D10_MESSAGE_ID = 201;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: D3D10_MESSAGE_ID = 200;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: D3D10_MESSAGE_ID = 202;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: D3D10_MESSAGE_ID = 205;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: D3D10_MESSAGE_ID = 204;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: D3D10_MESSAGE_ID = 203;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_NULLDESC: D3D10_MESSAGE_ID = 211;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_STENCIL_NO_TWO_SIDED: D3D10_MESSAGE_ID = 1048577;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 210;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 148;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: D3D10_MESSAGE_ID = 143;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 145;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = 144;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = 146;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 149;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 147;
pub const D3D10_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 142;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: D3D10_MESSAGE_ID = 188;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: D3D10_MESSAGE_ID = 189;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_EXPECTEDDECL: D3D10_MESSAGE_ID = 177;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: D3D10_MESSAGE_ID = 181;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: D3D10_MESSAGE_ID = 183;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: D3D10_MESSAGE_ID = 174;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: D3D10_MESSAGE_ID = 179;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: D3D10_MESSAGE_ID = 185;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = 172;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = 173;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: D3D10_MESSAGE_ID = 182;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: D3D10_MESSAGE_ID = 187;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: D3D10_MESSAGE_ID = 190;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: D3D10_MESSAGE_ID = 186;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: D3D10_MESSAGE_ID = 180;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: D3D10_MESSAGE_ID = 171;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: D3D10_MESSAGE_ID = 178;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: D3D10_MESSAGE_ID = 175;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: D3D10_MESSAGE_ID = 184;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: D3D10_MESSAGE_ID = 386;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDDECL: D3D10_MESSAGE_ID = 176;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = 169;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = 170;
pub const D3D10_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = 168;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: D3D10_MESSAGE_ID = 160;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: D3D10_MESSAGE_ID = 420;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: D3D10_MESSAGE_ID = 153;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: D3D10_MESSAGE_ID = 159;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: D3D10_MESSAGE_ID = 152;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: D3D10_MESSAGE_ID = 155;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: D3D10_MESSAGE_ID = 154;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: D3D10_MESSAGE_ID = 157;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: D3D10_MESSAGE_ID = 158;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: D3D10_MESSAGE_ID = 163;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLDESC: D3D10_MESSAGE_ID = 164;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: D3D10_MESSAGE_ID = 162;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: D3D10_MESSAGE_ID = 150;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: D3D10_MESSAGE_ID = 156;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: D3D10_MESSAGE_ID = 151;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: D3D10_MESSAGE_ID = 385;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: D3D10_MESSAGE_ID = 391;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: D3D10_MESSAGE_ID = 161;
pub const D3D10_MESSAGE_ID_CREATEINPUTLAYOUT_UNSUPPORTED_FORMAT: D3D10_MESSAGE_ID = 1048599;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = 192;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = 193;
pub const D3D10_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = 191;
pub const D3D10_MESSAGE_ID_CREATEPREDICATE_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 395;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 233;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDQUERY: D3D10_MESSAGE_ID = 232;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_NULLDESC: D3D10_MESSAGE_ID = 235;
pub const D3D10_MESSAGE_ID_CREATEQUERYORPREDICATE_UNEXPECTEDMISCFLAG: D3D10_MESSAGE_ID = 234;
pub const D3D10_MESSAGE_ID_CREATEQUERY_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 394;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthBiasClamp_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048578;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_DepthClipEnable_MUST_BE_TRUE: D3D10_MESSAGE_ID = 1048601;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: D3D10_MESSAGE_ID = 195;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: D3D10_MESSAGE_ID = 196;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: D3D10_MESSAGE_ID = 194;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: D3D10_MESSAGE_ID = 197;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_NULLDESC: D3D10_MESSAGE_ID = 199;
pub const D3D10_MESSAGE_ID_CREATERASTERIZERSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 198;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 140;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: D3D10_MESSAGE_ID = 135;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 137;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = 136;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = 138;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 141;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 139;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 133;
pub const D3D10_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = 134;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_EXCEEDS_FEATURE_LEVEL_DEFINITION: D3D10_MESSAGE_ID = 1048630;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DIMENSION_OUT_OF_RANGE: D3D10_MESSAGE_ID = 1048588;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_DXGI_FORMAT_R8G8B8A8_CANNOT_BE_SHARED: D3D10_MESSAGE_ID = 1048617;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_MSAA_PRECLUDES_SHADER_RESOURCE: D3D10_MESSAGE_ID = 1048610;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NON_POW_2_MIPMAP: D3D10_MESSAGE_ID = 1048634;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_RENDER_TARGET: D3D10_MESSAGE_ID = 1048608;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_SHADER_RESOURCE: D3D10_MESSAGE_ID = 1048589;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_ARRAYS: D3D10_MESSAGE_ID = 1048585;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_AUTOGEN_FOR_VOLUMES: D3D10_MESSAGE_ID = 1048616;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_DWORD_INDEX_BUFFER: D3D10_MESSAGE_ID = 1048609;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_STREAM_OUT: D3D10_MESSAGE_ID = 1048614;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_TEXTURE_1D: D3D10_MESSAGE_ID = 1048587;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_NO_VB_AND_IB_BIND: D3D10_MESSAGE_ID = 1048586;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_SINGLE_MIP_LEVEL_DEPTH_STENCIL_SUPPORTED: D3D10_MESSAGE_ID = 1048631;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_ONLY_VB_IB_FOR_BUFFERS: D3D10_MESSAGE_ID = 1048615;
pub const D3D10_MESSAGE_ID_CREATERESOURCE_PRESENTATION_PRECLUDES_SHADER_RESOURCE: D3D10_MESSAGE_ID = 1048611;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048635;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_OUT_OF_RANGE: D3D10_MESSAGE_ID = 1048581;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_EXCESSIVE_ANISOTROPY: D3D10_MESSAGE_ID = 1048580;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSU: D3D10_MESSAGE_ID = 222;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSV: D3D10_MESSAGE_ID = 223;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSW: D3D10_MESSAGE_ID = 224;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDCOMPARISONFUNC: D3D10_MESSAGE_ID = 227;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDFILTER: D3D10_MESSAGE_ID = 221;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXANISOTROPY: D3D10_MESSAGE_ID = 226;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXLOD: D3D10_MESSAGE_ID = 229;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMINLOD: D3D10_MESSAGE_ID = 228;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMIPLODBIAS: D3D10_MESSAGE_ID = 225;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MAXLOD_MUST_BE_FLT_MAX: D3D10_MESSAGE_ID = 1048605;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_MINLOD_MUST_NOT_BE_FRACTIONAL: D3D10_MESSAGE_ID = 1048604;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_COMPARISON_SUPPORT: D3D10_MESSAGE_ID = 1048579;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NO_MIRRORONCE: D3D10_MESSAGE_ID = 1048625;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_NULLDESC: D3D10_MESSAGE_ID = 231;
pub const D3D10_MESSAGE_ID_CREATESAMPLERSTATE_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 230;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_CUBES_MUST_HAVE_6_SIDES: D3D10_MESSAGE_ID = 1048607;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_FIRSTARRAYSLICE_MUST_BE_ZERO: D3D10_MESSAGE_ID = 1048606;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 131;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: D3D10_MESSAGE_ID = 126;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 128;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: D3D10_MESSAGE_ID = 127;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: D3D10_MESSAGE_ID = 129;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_MUST_USE_LOWEST_LOD: D3D10_MESSAGE_ID = 1048603;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 132;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_TOOMANYOBJECTS: D3D10_MESSAGE_ID = 130;
pub const D3D10_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 125;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 87;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = 82;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 81;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 84;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = 83;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = 85;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 86;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_INVALIDSAMPLES: D3D10_MESSAGE_ID = 76;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_LARGEALLOCATION: D3D10_MESSAGE_ID = 90;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_NULLDESC: D3D10_MESSAGE_ID = 89;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 88;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = 78;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 79;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 74;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = 80;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = 77;
pub const D3D10_MESSAGE_ID_CREATETEXTURE1D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = 75;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 104;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = 99;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 98;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 101;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = 100;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = 102;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 103;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_INVALIDSAMPLES: D3D10_MESSAGE_ID = 93;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_LARGEALLOCATION: D3D10_MESSAGE_ID = 107;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_NULLDESC: D3D10_MESSAGE_ID = 106;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 105;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = 95;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 96;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 91;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = 97;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = 94;
pub const D3D10_MESSAGE_ID_CREATETEXTURE2D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = 92;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 121;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDBINDFLAGS: D3D10_MESSAGE_ID = 116;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 115;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 118;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDINITIALDATA: D3D10_MESSAGE_ID = 117;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMIPLEVELS: D3D10_MESSAGE_ID = 119;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDMISCFLAGS: D3D10_MESSAGE_ID = 120;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_INVALIDSAMPLES: D3D10_MESSAGE_ID = 110;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_LARGEALLOCATION: D3D10_MESSAGE_ID = 124;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_NULLDESC: D3D10_MESSAGE_ID = 123;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 122;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDBINDFLAGS: D3D10_MESSAGE_ID = 112;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDCPUACCESSFLAGS: D3D10_MESSAGE_ID = 113;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDFORMAT: D3D10_MESSAGE_ID = 108;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDMISCFLAGS: D3D10_MESSAGE_ID = 114;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDUSAGE: D3D10_MESSAGE_ID = 111;
pub const D3D10_MESSAGE_ID_CREATETEXTURE3D_UNSUPPORTEDFORMAT: D3D10_MESSAGE_ID = 109;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: D3D10_MESSAGE_ID = 166;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: D3D10_MESSAGE_ID = 167;
pub const D3D10_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: D3D10_MESSAGE_ID = 165;
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_END: D3D10_MESSAGE_ID = 1048638;
pub const D3D10_MESSAGE_ID_D3D10L9_MESSAGES_START: D3D10_MESSAGE_ID = 1048576;
pub const D3D10_MESSAGE_ID_D3D10_MESSAGES_END: D3D10_MESSAGE_ID = 443;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INDEXPOS_OVERFLOW: D3D10_MESSAGE_ID = 340;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INSTANCEPOS_OVERFLOW: D3D10_MESSAGE_ID = 339;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINDEXED_INDEXPOS_OVERFLOW: D3D10_MESSAGE_ID = 336;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_INSTANCEPOS_OVERFLOW: D3D10_MESSAGE_ID = 338;
pub const D3D10_MESSAGE_ID_DEVICE_DRAWINSTANCED_VERTEXPOS_OVERFLOW: D3D10_MESSAGE_ID = 337;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_BOUND_RESOURCE_MAPPED: D3D10_MESSAGE_ID = 364;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_NOT_SET: D3D10_MESSAGE_ID = 350;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = 351;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_GS_INPUT_PRIMITIVE_MISMATCH: D3D10_MESSAGE_ID = 360;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_FORMAT_INVALID: D3D10_MESSAGE_ID = 358;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_NOT_SET: D3D10_MESSAGE_ID = 357;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = 359;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INDEX_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 368;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INPUTLAYOUT_NOT_SET: D3D10_MESSAGE_ID = 349;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_PRIMITIVETOPOLOGY: D3D10_MESSAGE_ID = 365;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: D3D10_MESSAGE_ID = 417;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: D3D10_MESSAGE_ID = 377;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: D3D10_MESSAGE_ID = 376;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_NOT_SET: D3D10_MESSAGE_ID = 363;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 369;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_POSITION_NOT_PRESENT: D3D10_MESSAGE_ID = 362;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_PS_OUTPUT_TYPE_MISMATCH: D3D10_MESSAGE_ID = 415;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_GATHER_UNSUPPORTED: D3D10_MESSAGE_ID = 416;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_LD_UNSUPPORTED: D3D10_MESSAGE_ID = 370;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_C_UNSUPPORTED: D3D10_MESSAGE_ID = 372;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_UNSUPPORTED: D3D10_MESSAGE_ID = 371;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_MULTISAMPLE_UNSUPPORTED: D3D10_MESSAGE_ID = 373;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_RETURN_TYPE_MISMATCH: D3D10_MESSAGE_ID = 361;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_RESOURCE_SAMPLE_COUNT_MISMATCH: D3D10_MESSAGE_ID = 421;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_MISMATCH: D3D10_MESSAGE_ID = 390;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SAMPLER_NOT_SET: D3D10_MESSAGE_ID = 352;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SHADERRESOURCEVIEW_NOT_SET: D3D10_MESSAGE_ID = 353;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_STRIDE_LARGER_THAN_BUFFER: D3D10_MESSAGE_ID = 375;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_SO_TARGETS_BOUND_WITHOUT_SOURCE: D3D10_MESSAGE_ID = 374;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEXPOS_OVERFLOW: D3D10_MESSAGE_ID = 335;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_NOT_SET: D3D10_MESSAGE_ID = 348;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: D3D10_MESSAGE_ID = 355;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_TOO_SMALL: D3D10_MESSAGE_ID = 356;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 366;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_SHADER_NOT_SET: D3D10_MESSAGE_ID = 341;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VERTEX_STRIDE_UNALIGNED: D3D10_MESSAGE_ID = 367;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEWPORT_NOT_SET: D3D10_MESSAGE_ID = 384;
pub const D3D10_MESSAGE_ID_DEVICE_DRAW_VIEW_DIMENSION_MISMATCH: D3D10_MESSAGE_ID = 354;
pub const D3D10_MESSAGE_ID_DEVICE_GENERATEMIPS_RESOURCE_INVALID: D3D10_MESSAGE_ID = 277;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 269;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 270;
pub const D3D10_MESSAGE_ID_DEVICE_GSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 268;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 251;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = 6;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 252;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = 5;
pub const D3D10_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 249;
pub const D3D10_MESSAGE_ID_DEVICE_IAGETVERTEXBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 264;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_FORMAT_INVALID: D3D10_MESSAGE_ID = 242;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_HAZARD: D3D10_MESSAGE_ID = 2;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_TOO_LARGE: D3D10_MESSAGE_ID = 243;
pub const D3D10_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 244;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_ADJACENCY_UNSUPPORTED: D3D10_MESSAGE_ID = 1048594;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNDEFINED: D3D10_MESSAGE_ID = 237;
pub const D3D10_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNRECOGNIZED: D3D10_MESSAGE_ID = 236;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 240;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_HAZARD: D3D10_MESSAGE_ID = 1;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_INVALIDRANGE: D3D10_MESSAGE_ID = 419;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_OFFSET_TOO_LARGE: D3D10_MESSAGE_ID = 239;
pub const D3D10_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_STRIDE_TOO_LARGE: D3D10_MESSAGE_ID = 418;
pub const D3D10_MESSAGE_ID_DEVICE_OMSETRENDERTARGETS_HAZARD: D3D10_MESSAGE_ID = 9;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BADINTERFACE_RETURN: D3D10_MESSAGE_ID = 383;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 381;
pub const D3D10_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_OUTOFMEMORY_RETURN: D3D10_MESSAGE_ID = 382;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 273;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 274;
pub const D3D10_MESSAGE_ID_DEVICE_PSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 272;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 257;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = 8;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 258;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = 7;
pub const D3D10_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 255;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: D3D10_MESSAGE_ID = 378;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: D3D10_MESSAGE_ID = 380;
pub const D3D10_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: D3D10_MESSAGE_ID = 379;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_INVALID: D3D10_MESSAGE_ID = 290;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_SUBRESOURCE_INVALID: D3D10_MESSAGE_ID = 291;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_FORMAT_INVALID: D3D10_MESSAGE_ID = 294;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_INVALID: D3D10_MESSAGE_ID = 292;
pub const D3D10_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_SUBRESOURCE_INVALID: D3D10_MESSAGE_ID = 293;
pub const D3D10_MESSAGE_ID_DEVICE_RSGETSCISSORRECTS_RECTS_EMPTY: D3D10_MESSAGE_ID = 276;
pub const D3D10_MESSAGE_ID_DEVICE_RSGETVIEWPORTS_VIEWPORTS_EMPTY: D3D10_MESSAGE_ID = 275;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_INVALIDSCISSOR: D3D10_MESSAGE_ID = 260;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_NEGATIVESCISSOR: D3D10_MESSAGE_ID = 1048632;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_TOO_MANY_SCISSORS: D3D10_MESSAGE_ID = 1048595;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_DENORMFLUSH: D3D10_MESSAGE_ID = 387;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_INVALIDVIEWPORT: D3D10_MESSAGE_ID = 259;
pub const D3D10_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_TOO_MANY_VIEWPORTS: D3D10_MESSAGE_ID = 1048593;
pub const D3D10_MESSAGE_ID_DEVICE_SETTEXTFILTERSIZE_INVALIDDIMENSIONS: D3D10_MESSAGE_ID = 389;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_COMPONENTTYPE: D3D10_MESSAGE_ID = 344;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: D3D10_MESSAGE_ID = 347;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERINDEX: D3D10_MESSAGE_ID = 343;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERMASK: D3D10_MESSAGE_ID = 345;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: D3D10_MESSAGE_ID = 342;
pub const D3D10_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SYSTEMVALUE: D3D10_MESSAGE_ID = 346;
pub const D3D10_MESSAGE_ID_DEVICE_SOGETTARGETS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 271;
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_HAZARD: D3D10_MESSAGE_ID = 10;
pub const D3D10_MESSAGE_ID_DEVICE_SOSETTARGETS_OFFSET_UNALIGNED: D3D10_MESSAGE_ID = 254;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 266;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 267;
pub const D3D10_MESSAGE_ID_DEVICE_VSGETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 265;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_BUFFERS_EMPTY: D3D10_MESSAGE_ID = 247;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_HAZARD: D3D10_MESSAGE_ID = 4;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSAMPLERS_SAMPLERS_EMPTY: D3D10_MESSAGE_ID = 248;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_HAZARD: D3D10_MESSAGE_ID = 3;
pub const D3D10_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_VIEWS_EMPTY: D3D10_MESSAGE_ID = 245;
pub const D3D10_MESSAGE_ID_DRAWINDEXEDINSTANCED_NOT_SUPPORTED_BELOW_9_3: D3D10_MESSAGE_ID = 1048627;
pub const D3D10_MESSAGE_ID_DRAWINDEXED_POINTLIST_UNSUPPORTED: D3D10_MESSAGE_ID = 1048628;
pub const D3D10_MESSAGE_ID_DRAWINDEXED_STARTINDEXLOCATION_MUST_BE_POSITIVE: D3D10_MESSAGE_ID = 1048602;
pub const D3D10_MESSAGE_ID_DRAWINSTANCED_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048626;
pub const D3D10_MESSAGE_ID_GEOMETRY_SHADER_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048619;
pub const D3D10_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: D3D10_MESSAGE_ID = 51;
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = 250;
pub const D3D10_MESSAGE_ID_GSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 39;
pub const D3D10_MESSAGE_ID_GSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 40;
pub const D3D10_MESSAGE_ID_GSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 38;
pub const D3D10_MESSAGE_ID_GSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 37;
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_INVALIDBUFFER: D3D10_MESSAGE_ID = 241;
pub const D3D10_MESSAGE_ID_IASETINDEXBUFFER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 32;
pub const D3D10_MESSAGE_ID_IASETINPUTLAYOUT_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 30;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_BAD_BUFFER_INDEX: D3D10_MESSAGE_ID = 1048592;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = 238;
pub const D3D10_MESSAGE_ID_IASETVERTEXBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 31;
pub const D3D10_MESSAGE_ID_LIVE_BLENDSTATE: D3D10_MESSAGE_ID = 435;
pub const D3D10_MESSAGE_ID_LIVE_BUFFER: D3D10_MESSAGE_ID = 423;
pub const D3D10_MESSAGE_ID_LIVE_COUNTER: D3D10_MESSAGE_ID = 440;
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE: D3D10_MESSAGE_ID = 436;
pub const D3D10_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW: D3D10_MESSAGE_ID = 429;
pub const D3D10_MESSAGE_ID_LIVE_DEVICE: D3D10_MESSAGE_ID = 441;
pub const D3D10_MESSAGE_ID_LIVE_GEOMETRYSHADER: D3D10_MESSAGE_ID = 431;
pub const D3D10_MESSAGE_ID_LIVE_INPUTLAYOUT: D3D10_MESSAGE_ID = 433;
pub const D3D10_MESSAGE_ID_LIVE_OBJECT_SUMMARY: D3D10_MESSAGE_ID = 422;
pub const D3D10_MESSAGE_ID_LIVE_PIXELSHADER: D3D10_MESSAGE_ID = 432;
pub const D3D10_MESSAGE_ID_LIVE_PREDICATE: D3D10_MESSAGE_ID = 439;
pub const D3D10_MESSAGE_ID_LIVE_QUERY: D3D10_MESSAGE_ID = 438;
pub const D3D10_MESSAGE_ID_LIVE_RASTERIZERSTATE: D3D10_MESSAGE_ID = 437;
pub const D3D10_MESSAGE_ID_LIVE_RENDERTARGETVIEW: D3D10_MESSAGE_ID = 428;
pub const D3D10_MESSAGE_ID_LIVE_SAMPLER: D3D10_MESSAGE_ID = 434;
pub const D3D10_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW: D3D10_MESSAGE_ID = 427;
pub const D3D10_MESSAGE_ID_LIVE_SWAPCHAIN: D3D10_MESSAGE_ID = 442;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE1D: D3D10_MESSAGE_ID = 424;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE2D: D3D10_MESSAGE_ID = 425;
pub const D3D10_MESSAGE_ID_LIVE_TEXTURE3D: D3D10_MESSAGE_ID = 426;
pub const D3D10_MESSAGE_ID_LIVE_VERTEXSHADER: D3D10_MESSAGE_ID = 430;
pub const D3D10_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: D3D10_MESSAGE_ID = 29;
pub const D3D10_MESSAGE_ID_OMSETBLENDSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 47;
pub const D3D10_MESSAGE_ID_OMSETDEPTHSTENCILSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 48;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_INVALIDVIEW: D3D10_MESSAGE_ID = 388;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_DIFFERING_BIT_DEPTHS: D3D10_MESSAGE_ID = 1048591;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_NO_SRGB_MRT: D3D10_MESSAGE_ID = 1048636;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_TOO_MANY_RENDER_TARGETS: D3D10_MESSAGE_ID = 1048590;
pub const D3D10_MESSAGE_ID_OMSETRENDERTARGETS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 49;
pub const D3D10_MESSAGE_ID_PREDICATE_BEGIN_DURING_PREDICATION: D3D10_MESSAGE_ID = 406;
pub const D3D10_MESSAGE_ID_PREDICATE_END_DURING_PREDICATION: D3D10_MESSAGE_ID = 409;
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = 256;
pub const D3D10_MESSAGE_ID_PSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 44;
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D10_MESSAGE_ID = 1048584;
pub const D3D10_MESSAGE_ID_PSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 45;
pub const D3D10_MESSAGE_ID_PSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 43;
pub const D3D10_MESSAGE_ID_PSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 42;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_ABANDONING_PREVIOUS_RESULTS: D3D10_MESSAGE_ID = 408;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_DUPLICATE: D3D10_MESSAGE_ID = 407;
pub const D3D10_MESSAGE_ID_QUERY_BEGIN_UNSUPPORTED: D3D10_MESSAGE_ID = 405;
pub const D3D10_MESSAGE_ID_QUERY_END_ABANDONING_PREVIOUS_RESULTS: D3D10_MESSAGE_ID = 410;
pub const D3D10_MESSAGE_ID_QUERY_END_WITHOUT_BEGIN: D3D10_MESSAGE_ID = 411;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_CALL: D3D10_MESSAGE_ID = 414;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_DATASIZE: D3D10_MESSAGE_ID = 412;
pub const D3D10_MESSAGE_ID_QUERY_GETDATA_INVALID_FLAGS: D3D10_MESSAGE_ID = 413;
pub const D3D10_MESSAGE_ID_REF_ACCESSING_INDEXABLE_TEMP_OUT_OF_RANGE: D3D10_MESSAGE_ID = 331;
pub const D3D10_MESSAGE_ID_REF_HARDWARE_EXCEPTION: D3D10_MESSAGE_ID = 330;
pub const D3D10_MESSAGE_ID_REF_INFO: D3D10_MESSAGE_ID = 334;
pub const D3D10_MESSAGE_ID_REF_KMDRIVER_EXCEPTION: D3D10_MESSAGE_ID = 329;
pub const D3D10_MESSAGE_ID_REF_OUT_OF_MEMORY: D3D10_MESSAGE_ID = 333;
pub const D3D10_MESSAGE_ID_REF_PROBLEM_PARSING_SHADER: D3D10_MESSAGE_ID = 332;
pub const D3D10_MESSAGE_ID_REF_SIMULATING_INFINITELY_FAST_HARDWARE: D3D10_MESSAGE_ID = 326;
pub const D3D10_MESSAGE_ID_REF_THREADING_MODE: D3D10_MESSAGE_ID = 327;
pub const D3D10_MESSAGE_ID_REF_UMDRIVER_EXCEPTION: D3D10_MESSAGE_ID = 328;
pub const D3D10_MESSAGE_ID_RSSETSTATE_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 46;
pub const D3D10_MESSAGE_ID_SETBLENDSTATE_SAMPLE_MASK_CANNOT_BE_ZERO: D3D10_MESSAGE_ID = 1048629;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 325;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_INVALIDARG_RETURN: D3D10_MESSAGE_ID = 324;
pub const D3D10_MESSAGE_ID_SETEXCEPTIONMODE_UNRECOGNIZEDFLAGS: D3D10_MESSAGE_ID = 323;
pub const D3D10_MESSAGE_ID_SETPREDICATION_INVALID_PREDICATE_STATE: D3D10_MESSAGE_ID = 404;
pub const D3D10_MESSAGE_ID_SETPREDICATION_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 50;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: D3D10_MESSAGE_ID = 55;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFLAGS: D3D10_MESSAGE_ID = 54;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: D3D10_MESSAGE_ID = 52;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_INVALIDIUNKNOWN: D3D10_MESSAGE_ID = 53;
pub const D3D10_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: D3D10_MESSAGE_ID = 56;
pub const D3D10_MESSAGE_ID_SHADERRESOURCEVIEW_GETDESC_LEGACY: D3D10_MESSAGE_ID = 393;
pub const D3D10_MESSAGE_ID_SLOT_ZERO_MUST_BE_D3D10_INPUT_PER_VERTEX_DATA: D3D10_MESSAGE_ID = 1048633;
pub const D3D10_MESSAGE_ID_SOSETTARGETS_INVALIDBUFFER: D3D10_MESSAGE_ID = 253;
pub const D3D10_MESSAGE_ID_SOSETTARGETS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 41;
pub const D3D10_MESSAGE_ID_STREAM_OUT_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048620;
pub const D3D10_MESSAGE_ID_STRING_FROM_APPLICATION: D3D10_MESSAGE_ID = 11;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = 303;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 304;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = 302;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = 300;
pub const D3D10_MESSAGE_ID_TEXTURE1D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 301;
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 305;
pub const D3D10_MESSAGE_ID_TEXTURE1D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = 306;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = 310;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 311;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = 309;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = 307;
pub const D3D10_MESSAGE_ID_TEXTURE2D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 308;
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 312;
pub const D3D10_MESSAGE_ID_TEXTURE2D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = 313;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_ALREADYMAPPED: D3D10_MESSAGE_ID = 317;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_DEVICEREMOVED_RETURN: D3D10_MESSAGE_ID = 318;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDFLAGS: D3D10_MESSAGE_ID = 316;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDMAPTYPE: D3D10_MESSAGE_ID = 314;
pub const D3D10_MESSAGE_ID_TEXTURE3D_MAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 315;
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_INVALIDSUBRESOURCE: D3D10_MESSAGE_ID = 319;
pub const D3D10_MESSAGE_ID_TEXTURE3D_UNMAP_NOTMAPPED: D3D10_MESSAGE_ID = 320;
pub const D3D10_MESSAGE_ID_TEXT_FILTER_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048621;
pub const D3D10_MESSAGE_ID_UNKNOWN: D3D10_MESSAGE_ID = 0;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONBOX: D3D10_MESSAGE_ID = 288;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSTATE: D3D10_MESSAGE_ID = 289;
pub const D3D10_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSUBRESOURCE: D3D10_MESSAGE_ID = 287;
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFER: D3D10_MESSAGE_ID = 246;
pub const D3D10_MESSAGE_ID_VSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 35;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048582;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_TOO_MANY_SAMPLERS: D3D10_MESSAGE_ID = 1048583;
pub const D3D10_MESSAGE_ID_VSSETSAMPLERS_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 36;
pub const D3D10_MESSAGE_ID_VSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 34;
pub const D3D10_MESSAGE_ID_VSSETSHADER_UNBINDDELETINGOBJECT: D3D10_MESSAGE_ID = 33;
pub const D3D10_MESSAGE_ID_VSSHADERRESOURCES_NOT_SUPPORTED: D3D10_MESSAGE_ID = 1048618;
pub type D3D10_MESSAGE_SEVERITY = i32;
pub const D3D10_MESSAGE_SEVERITY_CORRUPTION: D3D10_MESSAGE_SEVERITY = 0;
pub const D3D10_MESSAGE_SEVERITY_ERROR: D3D10_MESSAGE_SEVERITY = 1;
pub const D3D10_MESSAGE_SEVERITY_INFO: D3D10_MESSAGE_SEVERITY = 3;
pub const D3D10_MESSAGE_SEVERITY_MESSAGE: D3D10_MESSAGE_SEVERITY = 4;
pub const D3D10_MESSAGE_SEVERITY_WARNING: D3D10_MESSAGE_SEVERITY = 2;
pub const D3D10_MIN_BORDER_COLOR_COMPONENT: f32 = 0.0;
pub const D3D10_MIN_DEPTH: f32 = 0.0;
pub const D3D10_MIN_FILTER_SHIFT: u32 = 4;
pub const D3D10_MIN_MAXANISOTROPY: u32 = 0;
pub const D3D10_MIP_FILTER_SHIFT: u32 = 0;
pub const D3D10_MIP_LOD_BIAS_MAX: f32 = 15.99;
pub const D3D10_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 6;
pub const D3D10_MIP_LOD_RANGE_BIT_COUNT: u32 = 8;
pub const D3D10_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4;
#[cfg(feature = "d3dcommon")]
pub type D3D10_NAME = super::D3D_NAME;
pub const D3D10_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_PASS_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Annotations: u32,
    pub pIAInputSignature: *mut u8,
    pub IAInputSignatureSize: usize,
    pub StencilRef: u32,
    pub SampleMask: u32,
    pub BlendFactor: [f32; 4],
}
impl Default for D3D10_PASS_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_PASS_SHADER_DESC {
    pub pShaderVariable: *mut core::ffi::c_void,
    pub ShaderIndex: u32,
}
impl Default for D3D10_PASS_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 13;
pub const D3D10_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15;
#[cfg(feature = "d3dcommon")]
pub type D3D10_PRIMITIVE = super::D3D_PRIMITIVE;
#[cfg(feature = "d3dcommon")]
pub type D3D10_PRIMITIVE_TOPOLOGY = super::D3D_PRIMITIVE_TOPOLOGY;
pub const D3D10_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295;
pub const D3D10_PS_FRONTFACING_FALSE_VALUE: u32 = 0;
pub const D3D10_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295;
pub const D3D10_PS_INPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_PS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D10_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D10_PS_INPUT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.0;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_PS_OUTPUT_REGISTER_COUNT: u32 = 8;
pub const D3D10_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5;
pub type D3D10_QUERY = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_QUERY_DESC {
    pub Query: D3D10_QUERY,
    pub MiscFlags: u32,
}
pub const D3D10_QUERY_EVENT: D3D10_QUERY = 0;
pub type D3D10_QUERY_MISC_FLAG = i32;
pub const D3D10_QUERY_MISC_PREDICATEHINT: D3D10_QUERY_MISC_FLAG = 1;
pub const D3D10_QUERY_OCCLUSION: D3D10_QUERY = 1;
pub const D3D10_QUERY_OCCLUSION_PREDICATE: D3D10_QUERY = 5;
pub const D3D10_QUERY_PIPELINE_STATISTICS: D3D10_QUERY = 4;
pub const D3D10_QUERY_SO_OVERFLOW_PREDICATE: D3D10_QUERY = 7;
pub const D3D10_QUERY_SO_STATISTICS: D3D10_QUERY = 6;
pub const D3D10_QUERY_TIMESTAMP: D3D10_QUERY = 2;
pub const D3D10_QUERY_TIMESTAMP_DISJOINT: D3D10_QUERY = 3;
pub type D3D10_RAISE_FLAG = i32;
pub const D3D10_RAISE_FLAG_DRIVER_INTERNAL_ERROR: D3D10_RAISE_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_RASTERIZER_DESC {
    pub FillMode: D3D10_FILL_MODE,
    pub CullMode: D3D10_CULL_MODE,
    pub FrontCounterClockwise: windows_sys::core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_sys::core::BOOL,
    pub ScissorEnable: windows_sys::core::BOOL,
    pub MultisampleEnable: windows_sys::core::BOOL,
    pub AntialiasedLineEnable: windows_sys::core::BOOL,
}
#[cfg(feature = "windef")]
pub type D3D10_RECT = super::RECT;
#[cfg(feature = "d3dcommon")]
pub type D3D10_REGISTER_COMPONENT_TYPE = super::D3D_REGISTER_COMPONENT_TYPE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: windows_sys::core::BOOL,
    pub SrcBlend: D3D10_BLEND,
    pub DestBlend: D3D10_BLEND,
    pub BlendOp: D3D10_BLEND_OP,
    pub SrcBlendAlpha: D3D10_BLEND,
    pub DestBlendAlpha: D3D10_BLEND,
    pub BlendOpAlpha: D3D10_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
pub struct D3D10_RENDER_TARGET_VIEW_DESC {
    pub Format: super::DXGI_FORMAT,
    pub ViewDimension: D3D10_RTV_DIMENSION,
    pub Anonymous: D3D10_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "dxgi")]
impl Default for D3D10_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy)]
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
#[cfg(feature = "dxgi")]
impl Default for D3D10_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_REQ_BLEND_OBJECT_COUNT_PER_CONTEXT: u32 = 4096;
pub const D3D10_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27;
pub const D3D10_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096;
pub const D3D10_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_CONTEXT: u32 = 4096;
pub const D3D10_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32;
pub const D3D10_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32;
pub const D3D10_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 8192;
pub const D3D10_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024;
pub const D3D10_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096;
pub const D3D10_REQ_MAXANISOTROPY: u32 = 16;
pub const D3D10_REQ_MIP_LEVELS: u32 = 14;
pub const D3D10_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048;
pub const D3D10_REQ_RASTERIZER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096;
pub const D3D10_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 8192;
pub const D3D10_REQ_RESOURCE_SIZE_IN_MEGABYTES: u32 = 128;
pub const D3D10_REQ_RESOURCE_VIEW_COUNT_PER_CONTEXT_2_TO_EXP: u32 = 20;
pub const D3D10_REQ_SAMPLER_OBJECT_COUNT_PER_CONTEXT: u32 = 4096;
pub const D3D10_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 512;
pub const D3D10_REQ_TEXTURE1D_U_DIMENSION: u32 = 8192;
pub const D3D10_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 512;
pub const D3D10_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 8192;
pub const D3D10_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048;
pub const D3D10_REQ_TEXTURECUBE_DIMENSION: u32 = 8192;
pub const D3D10_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0;
pub type D3D10_RESOURCE_DIMENSION = i32;
pub const D3D10_RESOURCE_DIMENSION_BUFFER: D3D10_RESOURCE_DIMENSION = 1;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE1D: D3D10_RESOURCE_DIMENSION = 2;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE2D: D3D10_RESOURCE_DIMENSION = 3;
pub const D3D10_RESOURCE_DIMENSION_TEXTURE3D: D3D10_RESOURCE_DIMENSION = 4;
pub const D3D10_RESOURCE_DIMENSION_UNKNOWN: D3D10_RESOURCE_DIMENSION = 0;
pub type D3D10_RESOURCE_MISC_FLAG = i32;
pub const D3D10_RESOURCE_MISC_GDI_COMPATIBLE: D3D10_RESOURCE_MISC_FLAG = 32;
pub const D3D10_RESOURCE_MISC_GENERATE_MIPS: D3D10_RESOURCE_MISC_FLAG = 1;
pub const D3D10_RESOURCE_MISC_SHARED: D3D10_RESOURCE_MISC_FLAG = 2;
pub const D3D10_RESOURCE_MISC_SHARED_KEYEDMUTEX: D3D10_RESOURCE_MISC_FLAG = 16;
pub const D3D10_RESOURCE_MISC_TEXTURECUBE: D3D10_RESOURCE_MISC_FLAG = 4;
#[cfg(feature = "d3dcommon")]
pub type D3D10_RESOURCE_RETURN_TYPE = super::D3D_RESOURCE_RETURN_TYPE;
pub type D3D10_RTV_DIMENSION = i32;
pub const D3D10_RTV_DIMENSION_BUFFER: D3D10_RTV_DIMENSION = 1;
pub const D3D10_RTV_DIMENSION_TEXTURE1D: D3D10_RTV_DIMENSION = 2;
pub const D3D10_RTV_DIMENSION_TEXTURE1DARRAY: D3D10_RTV_DIMENSION = 3;
pub const D3D10_RTV_DIMENSION_TEXTURE2D: D3D10_RTV_DIMENSION = 4;
pub const D3D10_RTV_DIMENSION_TEXTURE2DARRAY: D3D10_RTV_DIMENSION = 5;
pub const D3D10_RTV_DIMENSION_TEXTURE2DMS: D3D10_RTV_DIMENSION = 6;
pub const D3D10_RTV_DIMENSION_TEXTURE2DMSARRAY: D3D10_RTV_DIMENSION = 7;
pub const D3D10_RTV_DIMENSION_TEXTURE3D: D3D10_RTV_DIMENSION = 8;
pub const D3D10_RTV_DIMENSION_UNKNOWN: D3D10_RTV_DIMENSION = 0;
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for D3D10_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SDK_LAYERS_VERSION: u32 = 11;
pub const D3D10_SDK_VERSION: u32 = 29;
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_BUFFER_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: D3D10_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D10_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "d3dcommon")]
pub type D3D10_SHADER_CBUFFER_FLAGS = super::D3D_SHADER_CBUFFER_FLAGS;
pub const D3D10_SHADER_DEBUG: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_FILE_INFO {
    pub FileName: u32,
    pub FileNameLen: u32,
    pub FileData: u32,
    pub FileLen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_INPUT_INFO {
    pub Var: u32,
    pub InitialRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub InitialBank: u32,
    pub InitialRegister: u32,
    pub InitialComponent: u32,
    pub InitialValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for D3D10_SHADER_DEBUG_INST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608;
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    pub OutputRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub OutputReg: u32,
    pub TempArrayReg: u32,
    pub OutputComponents: [u32; 4],
    pub OutputVars: [D3D10_SHADER_DEBUG_OUTPUTVAR; 4],
    pub IndexReg: u32,
    pub IndexComp: u32,
}
impl Default for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_OUTPUTVAR {
    pub Var: u32,
    pub uValueMin: u32,
    pub uValueMax: u32,
    pub iValueMin: i32,
    pub iValueMax: i32,
    pub fValueMin: f32,
    pub fValueMax: f32,
    pub bNaNPossible: windows_sys::core::BOOL,
    pub bInfPossible: windows_sys::core::BOOL,
}
pub type D3D10_SHADER_DEBUG_REGTYPE = i32;
pub const D3D10_SHADER_DEBUG_REG_CBUFFER: D3D10_SHADER_DEBUG_REGTYPE = 2;
pub const D3D10_SHADER_DEBUG_REG_FORCE_DWORD: D3D10_SHADER_DEBUG_REGTYPE = 2147483647;
pub const D3D10_SHADER_DEBUG_REG_IMMEDIATECBUFFER: D3D10_SHADER_DEBUG_REGTYPE = 8;
pub const D3D10_SHADER_DEBUG_REG_INPUT: D3D10_SHADER_DEBUG_REGTYPE = 0;
pub const D3D10_SHADER_DEBUG_REG_LITERAL: D3D10_SHADER_DEBUG_REGTYPE = 9;
pub const D3D10_SHADER_DEBUG_REG_OUTPUT: D3D10_SHADER_DEBUG_REGTYPE = 1;
pub const D3D10_SHADER_DEBUG_REG_SAMPLER: D3D10_SHADER_DEBUG_REGTYPE = 7;
pub const D3D10_SHADER_DEBUG_REG_TBUFFER: D3D10_SHADER_DEBUG_REGTYPE = 3;
pub const D3D10_SHADER_DEBUG_REG_TEMP: D3D10_SHADER_DEBUG_REGTYPE = 4;
pub const D3D10_SHADER_DEBUG_REG_TEMPARRAY: D3D10_SHADER_DEBUG_REGTYPE = 5;
pub const D3D10_SHADER_DEBUG_REG_TEXTURE: D3D10_SHADER_DEBUG_REGTYPE = 6;
pub const D3D10_SHADER_DEBUG_REG_UNUSED: D3D10_SHADER_DEBUG_REGTYPE = 10;
pub type D3D10_SHADER_DEBUG_SCOPETYPE = i32;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    pub TokenId: u32,
    pub VarType: D3D10_SHADER_DEBUG_VARTYPE,
    pub Class: D3D10_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub StructMemberScope: u32,
    pub uArrayIndices: u32,
    pub ArrayElements: u32,
    pub ArrayStrides: u32,
    pub uVariables: u32,
    pub uFirstVariable: u32,
}
pub const D3D10_SHADER_DEBUG_SCOPE_ANNOTATION: D3D10_SHADER_DEBUG_SCOPETYPE = 7;
pub const D3D10_SHADER_DEBUG_SCOPE_BLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = 1;
pub const D3D10_SHADER_DEBUG_SCOPE_FORCE_DWORD: D3D10_SHADER_DEBUG_SCOPETYPE = 2147483647;
pub const D3D10_SHADER_DEBUG_SCOPE_FORLOOP: D3D10_SHADER_DEBUG_SCOPETYPE = 2;
pub const D3D10_SHADER_DEBUG_SCOPE_FUNC_PARAMS: D3D10_SHADER_DEBUG_SCOPETYPE = 4;
pub const D3D10_SHADER_DEBUG_SCOPE_GLOBAL: D3D10_SHADER_DEBUG_SCOPETYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_SCOPE_INFO {
    pub ScopeType: D3D10_SHADER_DEBUG_SCOPETYPE,
    pub Name: u32,
    pub uNameLen: u32,
    pub uVariables: u32,
    pub VariableData: u32,
}
pub const D3D10_SHADER_DEBUG_SCOPE_NAMESPACE: D3D10_SHADER_DEBUG_SCOPETYPE = 6;
pub const D3D10_SHADER_DEBUG_SCOPE_STATEBLOCK: D3D10_SHADER_DEBUG_SCOPETYPE = 5;
pub const D3D10_SHADER_DEBUG_SCOPE_STRUCT: D3D10_SHADER_DEBUG_SCOPETYPE = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_TOKEN_INFO {
    pub File: u32,
    pub Line: u32,
    pub Column: u32,
    pub TokenLength: u32,
    pub TokenId: u32,
}
pub type D3D10_SHADER_DEBUG_VARTYPE = i32;
pub const D3D10_SHADER_DEBUG_VAR_FORCE_DWORD: D3D10_SHADER_DEBUG_VARTYPE = 2147483647;
pub const D3D10_SHADER_DEBUG_VAR_FUNCTION: D3D10_SHADER_DEBUG_VARTYPE = 1;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_VAR_INFO {
    pub TokenId: u32,
    pub Type: D3D10_SHADER_VARIABLE_TYPE,
    pub Register: u32,
    pub Component: u32,
    pub ScopeVar: u32,
    pub ScopeVarOffset: u32,
}
pub const D3D10_SHADER_DEBUG_VAR_VARIABLE: D3D10_SHADER_DEBUG_VARTYPE = 0;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_DESC {
    pub Version: u32,
    pub Creator: windows_sys::core::PCSTR,
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
    pub GSOutputTopology: D3D10_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D10_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096;
pub const D3D10_SHADER_ENABLE_STRICTNESS: u32 = 2048;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32;
pub const D3D10_SHADER_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0;
pub const D3D10_SHADER_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128;
pub const D3D10_SHADER_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64;
pub const D3D10_SHADER_IEEE_STRICTNESS: u32 = 8192;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_INPUT_BIND_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: D3D10_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: D3D10_RESOURCE_RETURN_TYPE,
    pub Dimension: D3D10_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D10_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "d3dcommon")]
pub type D3D10_SHADER_INPUT_FLAGS = super::D3D_SHADER_INPUT_FLAGS;
#[cfg(feature = "d3dcommon")]
pub type D3D10_SHADER_INPUT_TYPE = super::D3D_SHADER_INPUT_TYPE;
#[cfg(feature = "d3dcommon")]
pub type D3D10_SHADER_MACRO = super::D3D_SHADER_MACRO;
pub const D3D10_SHADER_MAJOR_VERSION: u32 = 4;
pub const D3D10_SHADER_MINOR_VERSION: u32 = 0;
pub const D3D10_SHADER_NO_PRESHADER: u32 = 256;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL0: u32 = 16384;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL1: u32 = 0;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL2: u32 = 49152;
pub const D3D10_SHADER_OPTIMIZATION_LEVEL3: u32 = 32768;
pub const D3D10_SHADER_PACK_MATRIX_COLUMN_MAJOR: u32 = 16;
pub const D3D10_SHADER_PACK_MATRIX_ROW_MAJOR: u32 = 8;
pub const D3D10_SHADER_PARTIAL_PRECISION: u32 = 32;
pub const D3D10_SHADER_PREFER_FLOW_CONTROL: u32 = 1024;
pub const D3D10_SHADER_RESOURCES_MAY_ALIAS: u32 = 524288;
#[repr(C)]
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::DXGI_FORMAT,
    pub ViewDimension: D3D10_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
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
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::DXGI_FORMAT,
    pub ViewDimension: D3D10_SRV_DIMENSION1,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
#[derive(Clone, Copy)]
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
#[cfg(all(feature = "d3dcommon", feature = "dxgi"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SHADER_SKIP_OPTIMIZATION: u32 = 4;
pub const D3D10_SHADER_SKIP_VALIDATION: u32 = 2;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_TYPE_DESC {
    pub Class: D3D10_SHADER_VARIABLE_CLASS,
    pub Type: D3D10_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
}
#[cfg(feature = "d3dcommon")]
pub type D3D10_SHADER_VARIABLE_CLASS = super::D3D_SHADER_VARIABLE_CLASS;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_VARIABLE_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut core::ffi::c_void,
}
impl Default for D3D10_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "d3dcommon")]
pub type D3D10_SHADER_VARIABLE_FLAGS = super::D3D_SHADER_VARIABLE_FLAGS;
#[cfg(feature = "d3dcommon")]
pub type D3D10_SHADER_VARIABLE_TYPE = super::D3D_SHADER_VARIABLE_TYPE;
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144;
pub const D3D10_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0;
pub const D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5;
#[repr(C)]
#[cfg(feature = "d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D10_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: D3D10_NAME,
    pub ComponentType: D3D10_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
}
#[cfg(feature = "d3dcommon")]
impl Default for D3D10_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8;
pub const D3D10_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048;
pub const D3D10_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256;
pub const D3D10_SO_BUFFER_SLOT_COUNT: u32 = 4;
pub const D3D10_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_SO_DECLARATION_ENTRY {
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
impl Default for D3D10_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1;
pub const D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64;
pub const D3D10_SRGB_GAMMA: f32 = 2.2;
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92;
pub const D3D10_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055;
pub const D3D10_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4;
pub const D3D10_SRGB_TO_FLOAT_OFFSET: f32 = 0.055;
pub const D3D10_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045;
pub const D3D10_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5;
#[cfg(feature = "d3dcommon")]
pub type D3D10_SRV_DIMENSION = super::D3D_SRV_DIMENSION;
#[cfg(feature = "d3dcommon")]
pub type D3D10_SRV_DIMENSION1 = super::D3D_SRV_DIMENSION;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64;
pub const D3D10_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4;
pub const D3D10_STANDARD_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -1;
pub type D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = i32;
pub const D3D10_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128;
pub const D3D10_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32;
pub const D3D10_STANDARD_VECTOR_SIZE: u32 = 4;
pub const D3D10_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 16;
pub const D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for D3D10_STATE_BLOCK_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D10_STENCIL_OP = i32;
pub const D3D10_STENCIL_OP_DECR: D3D10_STENCIL_OP = 8;
pub const D3D10_STENCIL_OP_DECR_SAT: D3D10_STENCIL_OP = 5;
pub const D3D10_STENCIL_OP_INCR: D3D10_STENCIL_OP = 7;
pub const D3D10_STENCIL_OP_INCR_SAT: D3D10_STENCIL_OP = 4;
pub const D3D10_STENCIL_OP_INVERT: D3D10_STENCIL_OP = 6;
pub const D3D10_STENCIL_OP_KEEP: D3D10_STENCIL_OP = 1;
pub const D3D10_STENCIL_OP_REPLACE: D3D10_STENCIL_OP = 3;
pub const D3D10_STENCIL_OP_ZERO: D3D10_STENCIL_OP = 2;
pub const D3D10_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_SUBRESOURCE_DATA {
    pub pSysMem: *const core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl Default for D3D10_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_TECHNIQUE_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Passes: u32,
    pub Annotations: u32,
}
impl Default for D3D10_TECHNIQUE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX1D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX1D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXCUBE_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::DXGI_FORMAT,
    pub SampleDesc: super::DXGI_SAMPLE_DESC,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
pub type D3D10_TEXTURECUBE_FACE = i32;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_X: D3D10_TEXTURECUBE_FACE = 1;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Y: D3D10_TEXTURECUBE_FACE = 3;
pub const D3D10_TEXTURECUBE_FACE_NEGATIVE_Z: D3D10_TEXTURECUBE_FACE = 5;
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_X: D3D10_TEXTURECUBE_FACE = 0;
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Y: D3D10_TEXTURECUBE_FACE = 2;
pub const D3D10_TEXTURECUBE_FACE_POSITIVE_Z: D3D10_TEXTURECUBE_FACE = 4;
pub const D3D10_TEXTURE_ADDRESS_BORDER: D3D10_TEXTURE_ADDRESS_MODE = 4;
pub const D3D10_TEXTURE_ADDRESS_CLAMP: D3D10_TEXTURE_ADDRESS_MODE = 3;
pub const D3D10_TEXTURE_ADDRESS_MIRROR: D3D10_TEXTURE_ADDRESS_MODE = 2;
pub const D3D10_TEXTURE_ADDRESS_MIRROR_ONCE: D3D10_TEXTURE_ADDRESS_MODE = 5;
pub type D3D10_TEXTURE_ADDRESS_MODE = i32;
pub const D3D10_TEXTURE_ADDRESS_WRAP: D3D10_TEXTURE_ADDRESS_MODE = 1;
pub const D3D10_TEXT_1BIT_BIT: u32 = 2147483648;
pub const D3D10_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0;
pub type D3D10_USAGE = i32;
pub const D3D10_USAGE_DEFAULT: D3D10_USAGE = 0;
pub const D3D10_USAGE_DYNAMIC: D3D10_USAGE = 2;
pub const D3D10_USAGE_IMMUTABLE: D3D10_USAGE = 1;
pub const D3D10_USAGE_STAGING: D3D10_USAGE = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D10_VIEWPORT {
    pub TopLeftX: i32,
    pub TopLeftY: i32,
    pub Width: u32,
    pub Height: u32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
pub const D3D10_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15;
pub const D3D10_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16;
pub const D3D10_VIEWPORT_BOUNDS_MAX: u32 = 16383;
pub const D3D10_VIEWPORT_BOUNDS_MIN: i32 = -16384;
pub const D3D10_VS_INPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_VS_INPUT_REGISTER_COUNT: u32 = 16;
pub const D3D10_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D10_VS_INPUT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_VS_OUTPUT_REGISTER_COUNT: u32 = 16;
pub const D3D10_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10;
pub const D3D10_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25;
pub const D3D10_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25;
pub const D3D11_SHADER_DEBUG_REG_INTERFACE_POINTERS: D3D10_SHADER_DEBUG_REGTYPE = 11;
pub const D3D11_SHADER_DEBUG_REG_UAV: D3D10_SHADER_DEBUG_REGTYPE = 12;
pub const D3D_MAJOR_VERSION: u32 = 10;
pub const D3D_MINOR_VERSION: u32 = 0;
pub const D3D_SPEC_DATE_DAY: u32 = 8;
pub const D3D_SPEC_DATE_MONTH: u32 = 8;
pub const D3D_SPEC_DATE_YEAR: u32 = 2006;
pub const D3D_SPEC_VERSION: f64 = 1.050005;
pub const DXGI_DEBUG_D3D10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x243b4c52_3606_4d3a_99d7_a7e7b33ed706);
pub const GUID_DeviceType: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd722fb4d_7a68_437a_b20c_5804ee2494a6);
#[cfg(feature = "d3dcommon")]
pub type LPD3D10_CBUFFER_TYPE = *mut D3D10_CBUFFER_TYPE;
#[cfg(feature = "d3dcommon")]
pub type LPD3D10_SHADER_CBUFFER_FLAGS = *mut D3D10_SHADER_CBUFFER_FLAGS;
#[cfg(feature = "d3dcommon")]
pub type LPD3D10_SHADER_INPUT_FLAGS = *mut D3D10_SHADER_INPUT_FLAGS;
#[cfg(feature = "d3dcommon")]
pub type LPD3D10_SHADER_INPUT_TYPE = *mut D3D10_SHADER_INPUT_TYPE;
#[cfg(feature = "d3dcommon")]
pub type LPD3D10_SHADER_MACRO = *mut D3D10_SHADER_MACRO;
#[cfg(feature = "d3dcommon")]
pub type LPD3D10_SHADER_VARIABLE_CLASS = *mut D3D10_SHADER_VARIABLE_CLASS;
#[cfg(feature = "d3dcommon")]
pub type LPD3D10_SHADER_VARIABLE_FLAGS = *mut D3D10_SHADER_VARIABLE_FLAGS;
#[cfg(feature = "d3dcommon")]
pub type LPD3D10_SHADER_VARIABLE_TYPE = *mut D3D10_SHADER_VARIABLE_TYPE;
#[cfg(all(feature = "dxgi", feature = "minwindef"))]
pub type PFN_D3D10_CREATE_DEVICE1 = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: D3D10_DRIVER_TYPE, param2: super::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "dxgi", feature = "minwindef", feature = "windef"))]
pub type PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1 = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: D3D10_DRIVER_TYPE, param2: super::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut super::DXGI_SWAP_CHAIN_DESC, param7: *mut *mut core::ffi::c_void, param8: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
