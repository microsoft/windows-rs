#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CD3D10_BUFFER_DESC {
    pub Base: D3D10_BUFFER_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct CD3D10_TEXTURE1D_DESC {
    pub Base: D3D10_TEXTURE1D_DESC,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct CD3D10_TEXTURE2D_DESC {
    pub Base: D3D10_TEXTURE2D_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct CD3D10_TEXTURE3D_DESC {
    pub Base: D3D10_TEXTURE3D_DESC,
}
pub const D3D10_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535;
pub const D3D10_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295;
pub const D3D10_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255;
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
pub const D3D10_DEFAULT_DEPTH_BIAS: u32 = 0;
pub const D3D10_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0;
pub const D3D10_DEFAULT_SAMPLE_MASK: u32 = 4294967295;
pub const D3D10_DEFAULT_SCISSOR_ENDX: u32 = 0;
pub const D3D10_DEFAULT_SCISSOR_ENDY: u32 = 0;
pub const D3D10_DEFAULT_SCISSOR_STARTX: u32 = 0;
pub const D3D10_DEFAULT_SCISSOR_STARTY: u32 = 0;
pub const D3D10_DEFAULT_STENCIL_READ_MASK: u32 = 255;
pub const D3D10_DEFAULT_STENCIL_REFERENCE: u32 = 0;
pub const D3D10_DEFAULT_STENCIL_WRITE_MASK: u32 = 255;
pub const D3D10_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0;
pub const D3D10_DEFAULT_VIEWPORT_HEIGHT: u32 = 0;
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
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D10_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D10_DSV_DIMENSION,
    pub Anonymous: D3D10_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D10_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub union D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D10_TEX1D_DSV,
    pub Texture1DArray: D3D10_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D10_TEX2D_DSV,
    pub Texture2DArray: D3D10_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D10_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D10_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D10_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D10_DEPTH_WRITE_MASK = i32;
pub const D3D10_DEPTH_WRITE_MASK_ALL: D3D10_DEPTH_WRITE_MASK = 1;
pub const D3D10_DEPTH_WRITE_MASK_ZERO: D3D10_DEPTH_WRITE_MASK = 0;
pub type D3D10_DSV_DIMENSION = i32;
pub const D3D10_DSV_DIMENSION_TEXTURE1D: D3D10_DSV_DIMENSION = 1;
pub const D3D10_DSV_DIMENSION_TEXTURE1DARRAY: D3D10_DSV_DIMENSION = 2;
pub const D3D10_DSV_DIMENSION_TEXTURE2D: D3D10_DSV_DIMENSION = 3;
pub const D3D10_DSV_DIMENSION_TEXTURE2DARRAY: D3D10_DSV_DIMENSION = 4;
pub const D3D10_DSV_DIMENSION_TEXTURE2DMS: D3D10_DSV_DIMENSION = 5;
pub const D3D10_DSV_DIMENSION_TEXTURE2DMSARRAY: D3D10_DSV_DIMENSION = 6;
pub const D3D10_DSV_DIMENSION_UNKNOWN: D3D10_DSV_DIMENSION = 0;
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
pub type D3D10_INPUT_CLASSIFICATION = i32;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D10_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D10_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D10_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_INPUT_PER_INSTANCE_DATA: D3D10_INPUT_CLASSIFICATION = 1;
pub const D3D10_INPUT_PER_VERTEX_DATA: D3D10_INPUT_CLASSIFICATION = 0;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295;
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
pub const D3D10_MAX_MAXANISOTROPY: u32 = 16;
pub const D3D10_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32;
pub const D3D10_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17;
pub const D3D10_MIN_FILTER_SHIFT: u32 = 4;
pub const D3D10_MIN_MAXANISOTROPY: u32 = 0;
pub const D3D10_MIP_FILTER_SHIFT: u32 = 0;
pub const D3D10_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 6;
pub const D3D10_MIP_LOD_RANGE_BIT_COUNT: u32 = 8;
pub const D3D10_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0;
pub const D3D10_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 13;
pub const D3D10_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_PRIMITIVE = super::d3dcommon::D3D_PRIMITIVE;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_PRIMITIVE_TOPOLOGY = super::d3dcommon::D3D_PRIMITIVE_TOPOLOGY;
pub const D3D10_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295;
pub const D3D10_PS_FRONTFACING_FALSE_VALUE: u32 = 0;
pub const D3D10_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295;
pub const D3D10_PS_INPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_PS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D10_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2;
pub const D3D10_PS_INPUT_REGISTER_READ_PORTS: u32 = 1;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4;
pub const D3D10_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_PS_OUTPUT_REGISTER_COUNT: u32 = 8;
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
#[cfg(feature = "Win32_windef")]
pub type D3D10_RECT = super::windef::RECT;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D10_RENDER_TARGET_VIEW_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D10_RTV_DIMENSION,
    pub Anonymous: D3D10_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D10_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
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
#[cfg(feature = "Win32_dxgiformat")]
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
pub const D3D10_SDK_VERSION: u32 = 29;
pub const D3D10_SHADER_MAJOR_VERSION: u32 = 4;
pub const D3D10_SHADER_MINOR_VERSION: u32 = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D10_SRV_DIMENSION,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
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
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0;
pub const D3D10_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5;
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
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SRV_DIMENSION = super::d3dcommon::D3D_SRV_DIMENSION;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64;
pub const D3D10_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4;
pub const D3D10_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128;
pub const D3D10_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32;
pub const D3D10_STANDARD_VECTOR_SIZE: u32 = 4;
pub const D3D10_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 16;
pub const D3D10_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64;
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
pub struct D3D10_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
    pub Usage: D3D10_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D10_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
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
pub const D3D_MAJOR_VERSION: u32 = 10;
pub const D3D_MINOR_VERSION: u32 = 0;
pub const D3D_SPEC_DATE_DAY: u32 = 8;
pub const D3D_SPEC_DATE_MONTH: u32 = 8;
pub const D3D_SPEC_DATE_YEAR: u32 = 2006;
