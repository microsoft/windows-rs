#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D10_BUFFER_DESC {
    pub Base: D3D10_BUFFER_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D10_TEXTURE1D_DESC {
    pub Base: D3D10_TEXTURE1D_DESC,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D10_TEXTURE2D_DESC {
    pub Base: D3D10_TEXTURE2D_DESC,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_BLEND_DESC {
    pub AlphaToCoverageEnable: windows_core::BOOL,
    pub BlendEnable: [windows_core::BOOL; 8],
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D10_STENCIL_OP,
    pub StencilDepthFailOp: D3D10_STENCIL_OP,
    pub StencilPassOp: D3D10_STENCIL_OP,
    pub StencilFunc: D3D10_COMPARISON_FUNC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_DEPTH_STENCIL_DESC {
    pub DepthEnable: windows_core::BOOL,
    pub DepthWriteMask: D3D10_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D10_COMPARISON_FUNC,
    pub StencilEnable: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D10_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
pub const D3D10_INPUT_PER_INSTANCE_DATA: D3D10_INPUT_CLASSIFICATION = 1;
pub const D3D10_INPUT_PER_VERTEX_DATA: D3D10_INPUT_CLASSIFICATION = 0;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295;
pub const D3D10_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295;
pub const D3D10_MAG_FILTER_SHIFT: u32 = 2;
pub type D3D10_MAP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_PRIMITIVE(pub super::d3dcommon::D3D_PRIMITIVE);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_PRIMITIVE_TOPOLOGY(pub super::d3dcommon::D3D_PRIMITIVE_TOPOLOGY);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_RASTERIZER_DESC {
    pub FillMode: D3D10_FILL_MODE,
    pub CullMode: D3D10_CULL_MODE,
    pub FrontCounterClockwise: windows_core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_core::BOOL,
    pub ScissorEnable: windows_core::BOOL,
    pub MultisampleEnable: windows_core::BOOL,
    pub AntialiasedLineEnable: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SO_DECLARATION_ENTRY {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
pub const D3D10_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1;
pub const D3D10_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64;
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_SRV_DIMENSION(pub super::d3dcommon::D3D_SRV_DIMENSION);
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
pub const D3D10_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 18;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
windows_core::imp::define_interface!(ID3D10Asynchronous, ID3D10Asynchronous_Vtbl, 0x9b7e4c0d_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Asynchronous {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Asynchronous, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10Asynchronous {
    pub unsafe fn Begin(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn End(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn GetData(&self, pdata: Option<*mut core::ffi::c_void>, datasize: u32, getdataflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pdata.unwrap_or(core::mem::zeroed()) as _, datasize, getdataflags) }
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Asynchronous_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait ID3D10Asynchronous_Impl: ID3D10DeviceChild_Impl {
    fn Begin(&self);
    fn End(&self);
    fn GetData(&self, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::Result<()>;
    fn GetDataSize(&self) -> u32;
}
impl ID3D10Asynchronous_Vtbl {
    pub const fn new<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Asynchronous_Impl::Begin(this);
            }
        }
        unsafe extern "system" fn End<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Asynchronous_Impl::End(this);
            }
        }
        unsafe extern "system" fn GetData<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Asynchronous_Impl::GetData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&getdataflags)).into()
            }
        }
        unsafe extern "system" fn GetDataSize<Identity: ID3D10Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Asynchronous_Impl::GetDataSize(this)
            }
        }
        Self {
            base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            Begin: Begin::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetDataSize: GetDataSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Asynchronous as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Asynchronous {}
windows_core::imp::define_interface!(ID3D10BlendState, ID3D10BlendState_Vtbl, 0xedad8d19_8a35_4d6d_8566_2ea276cde161);
impl core::ops::Deref for ID3D10BlendState {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10BlendState, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10BlendState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BlendState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BLEND_DESC),
}
pub trait ID3D10BlendState_Impl: ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC);
}
impl ID3D10BlendState_Vtbl {
    pub const fn new<Identity: ID3D10BlendState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10BlendState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10BlendState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10BlendState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10BlendState {}
windows_core::imp::define_interface!(ID3D10Buffer, ID3D10Buffer_Vtbl, 0x9b7e4c02_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Buffer {
    type Target = ID3D10Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Buffer, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Resource);
impl ID3D10Buffer {
    pub unsafe fn Map(&self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), maptype, mapflags, ppdata as _) }
    }
    pub unsafe fn Unmap(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Buffer_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_MAP, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BUFFER_DESC),
}
pub trait ID3D10Buffer_Impl: ID3D10Resource_Impl {
    fn Map(&self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Unmap(&self);
    fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC);
}
impl ID3D10Buffer_Vtbl {
    pub const fn new<Identity: ID3D10Buffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Map<Identity: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Buffer_Impl::Map(this, core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags), core::mem::transmute_copy(&ppdata)).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Buffer_Impl::Unmap(this);
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Buffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Buffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Buffer as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Buffer {}
windows_core::imp::define_interface!(ID3D10Counter, ID3D10Counter_Vtbl, 0x9b7e4c11_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Counter {
    type Target = ID3D10Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Counter, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Asynchronous);
impl ID3D10Counter {
    pub unsafe fn GetDesc(&self) -> D3D10_COUNTER_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Counter_Vtbl {
    pub base__: ID3D10Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_COUNTER_DESC),
}
pub trait ID3D10Counter_Impl: ID3D10Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_COUNTER_DESC);
}
impl ID3D10Counter_Vtbl {
    pub const fn new<Identity: ID3D10Counter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10Counter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Counter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10Asynchronous_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Counter as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Counter {}
windows_core::imp::define_interface!(ID3D10DepthStencilState, ID3D10DepthStencilState_Vtbl, 0x2b4b1cc8_a4ad_41f8_8322_ca86fc3ec675);
impl core::ops::Deref for ID3D10DepthStencilState {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10DepthStencilState, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10DepthStencilState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DepthStencilState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_DEPTH_STENCIL_DESC),
}
pub trait ID3D10DepthStencilState_Impl: ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC);
}
impl ID3D10DepthStencilState_Vtbl {
    pub const fn new<Identity: ID3D10DepthStencilState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10DepthStencilState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DepthStencilState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DepthStencilState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10DepthStencilState {}
windows_core::imp::define_interface!(ID3D10DepthStencilView, ID3D10DepthStencilView_Vtbl, 0x9b7e4c09_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10DepthStencilView {
    type Target = ID3D10View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10DepthStencilView, windows_core::IUnknown, ID3D10DeviceChild, ID3D10View);
impl ID3D10DepthStencilView {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DepthStencilView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_DEPTH_STENCIL_VIEW_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D10DepthStencilView_Impl: ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D10DepthStencilView_Vtbl {
    pub const fn new<Identity: ID3D10DepthStencilView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10DepthStencilView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DepthStencilView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DepthStencilView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D10DepthStencilView {}
windows_core::imp::define_interface!(ID3D10Device, ID3D10Device_Vtbl, 0x9b7e4c0f_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D10Device, windows_core::IUnknown);
impl ID3D10Device {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0)
    where
        P0: windows_core::Param<ID3D10PixelShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetShader)(windows_core::Interface::as_raw(self), ppixelshader.param().abi());
        }
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0)
    where
        P0: windows_core::Param<ID3D10VertexShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetShader)(windows_core::Interface::as_raw(self), pvertexshader.param().abi());
        }
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexed)(windows_core::Interface::as_raw(self), indexcount, startindexlocation, basevertexlocation);
        }
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), vertexcount, startvertexlocation);
        }
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: windows_core::Param<ID3D10InputLayout>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IASetInputLayout)(windows_core::Interface::as_raw(self), pinputlayout.param().abi());
        }
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*const Option<ID3D10Buffer>>, pstrides: Option<*const u32>, poffsets: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).IASetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _, pstrides.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::dxgiformat::DXGI_FORMAT, offset: u32)
    where
        P0: windows_core::Param<ID3D10Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IASetIndexBuffer)(windows_core::Interface::as_raw(self), pindexbuffer.param().abi(), format, offset);
        }
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexedInstanced)(windows_core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation);
        }
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInstanced)(windows_core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation);
        }
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&[Option<ID3D10Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0)
    where
        P0: windows_core::Param<ID3D10GeometryShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetShader)(windows_core::Interface::as_raw(self), pshader.param().abi());
        }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: D3D10_PRIMITIVE_TOPOLOGY) {
        unsafe {
            (windows_core::Interface::vtable(self).IASetPrimitiveTopology)(windows_core::Interface::as_raw(self), topology);
        }
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn SetPredication<P0>(&self, ppredicate: P0, predicatevalue: bool)
    where
        P0: windows_core::Param<ID3D10Predicate>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), ppredicate.param().abi(), predicatevalue.into());
        }
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&[Option<ID3D10ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: Option<&[Option<ID3D10SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn OMSetRenderTargets<P2>(&self, pprendertargetviews: Option<&[Option<ID3D10RenderTargetView>]>, pdepthstencilview: P2)
    where
        P2: windows_core::Param<ID3D10DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.map_or(core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.param().abi());
        }
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: &[f32; 4], samplemask: u32)
    where
        P0: windows_core::Param<ID3D10BlendState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetBlendState)(windows_core::Interface::as_raw(self), pblendstate.param().abi(), core::mem::transmute(blendfactor.as_ptr()), samplemask);
        }
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: windows_core::Param<ID3D10DepthStencilState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencilstate.param().abi(), stencilref);
        }
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: Option<*const Option<ID3D10Buffer>>, poffsets: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).SOSetTargets)(windows_core::Interface::as_raw(self), numbuffers, ppsotargets.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn DrawAuto(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawAuto)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: windows_core::Param<ID3D10RasterizerState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetState)(windows_core::Interface::as_raw(self), prasterizerstate.param().abi());
        }
    }
    pub unsafe fn RSSetViewports(&self, pviewports: Option<&[D3D10_VIEWPORT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetViewports)(windows_core::Interface::as_raw(self), pviewports.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviewports.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RSSetScissorRects(&self, prects: Option<&[D3D10_RECT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetScissorRects)(windows_core::Interface::as_raw(self), prects.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prects.map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn CopySubresourceRegion<P0, P5>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P5, srcsubresource: u32, psrcbox: Option<*const D3D10_BOX>)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P5: windows_core::Param<ID3D10Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopySubresourceRegion)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.param().abi(), srcsubresource, psrcbox.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P1: windows_core::Param<ID3D10Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyResource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), psrcresource.param().abi());
        }
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const D3D10_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, pdstbox.unwrap_or(core::mem::zeroed()) as _, psrcdata, srcrowpitch, srcdepthpitch);
        }
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: &[f32; 4])
    where
        P0: windows_core::Param<ID3D10RenderTargetView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearRenderTargetView)(windows_core::Interface::as_raw(self), prendertargetview.param().abi(), core::mem::transmute(colorrgba.as_ptr()));
        }
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: windows_core::Param<ID3D10DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearDepthStencilView)(windows_core::Interface::as_raw(self), pdepthstencilview.param().abi(), clearflags, depth, stencil);
        }
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: windows_core::Param<ID3D10ShaderResourceView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMips)(windows_core::Interface::as_raw(self), pshaderresourceview.param().abi());
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn ResolveSubresource<P0, P2>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P2, srcsubresource: u32, format: super::dxgiformat::DXGI_FORMAT)
    where
        P0: windows_core::Param<ID3D10Resource>,
        P2: windows_core::Param<ID3D10Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, psrcresource.param().abi(), srcsubresource, format);
        }
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn PSGetShader(&self) -> windows_core::Result<ID3D10PixelShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSGetShader(&self) -> windows_core::Result<ID3D10VertexShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn IAGetInputLayout(&self) -> windows_core::Result<ID3D10InputLayout> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetInputLayout)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: Option<*mut Option<ID3D10Buffer>>, pstrides: Option<*mut u32>, poffsets: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).IAGetVertexBuffers)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _, pstrides.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: Option<*mut Option<ID3D10Buffer>>, format: Option<*mut super::dxgiformat::DXGI_FORMAT>, offset: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).IAGetIndexBuffer)(windows_core::Interface::as_raw(self), pindexbuffer.unwrap_or(core::mem::zeroed()) as _, format.unwrap_or(core::mem::zeroed()) as _, offset.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: Option<&mut [Option<ID3D10Buffer>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetConstantBuffers)(windows_core::Interface::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppconstantbuffers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GSGetShader(&self) -> windows_core::Result<ID3D10GeometryShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GSGetShader)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> D3D10_PRIMITIVE_TOPOLOGY {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetPrimitiveTopology)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GetPredication(&self, pppredicate: Option<*mut Option<ID3D10Predicate>>, ppredicatevalue: Option<*mut windows_core::BOOL>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetPredication)(windows_core::Interface::as_raw(self), pppredicate.unwrap_or(core::mem::zeroed()) as _, ppredicatevalue.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: Option<&mut [Option<ID3D10ShaderResourceView>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetShaderResources)(windows_core::Interface::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppshaderresourceviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: Option<&mut [Option<ID3D10SamplerState>]>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetSamplers)(windows_core::Interface::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppsamplers.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())));
        }
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: Option<&mut [Option<ID3D10RenderTargetView>]>, ppdepthstencilview: Option<*mut Option<ID3D10DepthStencilView>>) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetRenderTargets)(windows_core::Interface::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pprendertargetviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: Option<*mut Option<ID3D10BlendState>>, blendfactor: Option<&mut [f32; 4]>, psamplemask: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetBlendState)(windows_core::Interface::as_raw(self), ppblendstate.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(blendfactor.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psamplemask.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: Option<*mut Option<ID3D10DepthStencilState>>, pstencilref: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetDepthStencilState)(windows_core::Interface::as_raw(self), ppdepthstencilstate.unwrap_or(core::mem::zeroed()) as _, pstencilref.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn SOGetTargets(&self, numbuffers: u32, ppsotargets: Option<*mut Option<ID3D10Buffer>>, poffsets: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).SOGetTargets)(windows_core::Interface::as_raw(self), numbuffers, ppsotargets.unwrap_or(core::mem::zeroed()) as _, poffsets.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn RSGetState(&self) -> windows_core::Result<ID3D10RasterizerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RSGetState)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn RSGetViewports(&self, numviewports: *mut u32, pviewports: Option<*mut D3D10_VIEWPORT>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetViewports)(windows_core::Interface::as_raw(self), numviewports as _, pviewports.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RSGetScissorRects(&self, numrects: *mut u32, prects: Option<*mut D3D10_RECT>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetScissorRects)(windows_core::Interface::as_raw(self), numrects as _, prects.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExceptionMode)(windows_core::Interface::as_raw(self), raiseflags) }
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetExceptionMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize as _, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()) }
    }
    pub unsafe fn ClearState(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Flush(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>, ppbuffer: Option<*mut Option<ID3D10Buffer>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateBuffer)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture1D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTexture1D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture2D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTexture2D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: Option<*const D3D10_SUBRESOURCE_DATA>) -> windows_core::Result<ID3D10Texture3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTexture3D)(windows_core::Interface::as_raw(self), pdesc, pinitialdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_SHADER_RESOURCE_VIEW_DESC>, ppsrview: Option<*mut Option<ID3D10ShaderResourceView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateShaderResourceView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppsrview.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_RENDER_TARGET_VIEW_DESC>, pprtview: Option<*mut Option<ID3D10RenderTargetView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTargetView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, pprtview.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: Option<*const D3D10_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: Option<*mut Option<ID3D10DepthStencilView>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilView)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D10_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: Option<*mut Option<ID3D10InputLayout>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateInputLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len().try_into().unwrap(), core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len().try_into().unwrap(), ppinputlayout.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateVertexShader(&self, pshaderbytecode: &[u8], ppvertexshader: Option<*mut Option<ID3D10VertexShader>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), ppvertexshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateGeometryShader(&self, pshaderbytecode: &[u8], ppgeometryshader: Option<*mut Option<ID3D10GeometryShader>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateGeometryShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), ppgeometryshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: &[u8], psodeclaration: Option<&[D3D10_SO_DECLARATION_ENTRY]>, outputstreamstride: u32, ppgeometryshader: Option<*mut Option<ID3D10GeometryShader>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateGeometryShaderWithStreamOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), core::mem::transmute(psodeclaration.map_or(core::ptr::null(), |slice| slice.as_ptr())), psodeclaration.map_or(0, |slice| slice.len().try_into().unwrap()), outputstreamstride, ppgeometryshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreatePixelShader(&self, pshaderbytecode: &[u8], pppixelshader: Option<*mut Option<ID3D10PixelShader>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len().try_into().unwrap(), pppixelshader.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: Option<*mut Option<ID3D10BlendState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateBlendState)(windows_core::Interface::as_raw(self), pblendstatedesc, ppblendstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: Option<*mut Option<ID3D10DepthStencilState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilState)(windows_core::Interface::as_raw(self), pdepthstencildesc, ppdepthstencilstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: Option<*mut Option<ID3D10RasterizerState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRasterizerState)(windows_core::Interface::as_raw(self), prasterizerdesc, pprasterizerstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: Option<*mut Option<ID3D10SamplerState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateSamplerState)(windows_core::Interface::as_raw(self), psamplerdesc, ppsamplerstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC, ppquery: Option<*mut Option<ID3D10Query>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), pquerydesc, ppquery.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: Option<*mut Option<ID3D10Predicate>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreatePredicate)(windows_core::Interface::as_raw(self), ppredicatedesc, pppredicate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: Option<*mut Option<ID3D10Counter>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateCounter)(windows_core::Interface::as_raw(self), pcounterdesc, ppcounter.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CheckFormatSupport(&self, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckFormatSupport)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels)(windows_core::Interface::as_raw(self), format, samplecount, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D10_COUNTER_INFO {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCounterInfo)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: Option<windows_core::PSTR>, pnamelength: Option<*mut u32>, szunits: Option<windows_core::PSTR>, punitslength: Option<*mut u32>, szdescription: Option<windows_core::PSTR>, pdescriptionlength: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckCounter)(windows_core::Interface::as_raw(self), pdesc, ptype as _, pactivecounters as _, szname.unwrap_or(core::mem::zeroed()) as _, pnamelength.unwrap_or(core::mem::zeroed()) as _, szunits.unwrap_or(core::mem::zeroed()) as _, punitslength.unwrap_or(core::mem::zeroed()) as _, szdescription.unwrap_or(core::mem::zeroed()) as _, pdescriptionlength.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OpenSharedResource(&self, hresource: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: Option<*mut *mut core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenSharedResource)(windows_core::Interface::as_raw(self), hresource, returnedinterface, ppresource.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetTextFilterSize(&self, width: u32, height: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextFilterSize)(windows_core::Interface::as_raw(self), width, height);
        }
    }
    pub unsafe fn GetTextFilterSize(&self, pwidth: Option<*mut u32>, pheight: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTextFilterSize)(windows_core::Interface::as_raw(self), pwidth.unwrap_or(core::mem::zeroed()) as _, pheight.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Device_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub VSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub PSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DrawIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32),
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    pub PSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub IASetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub IASetVertexBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    #[cfg(feature = "Win32_dxgiformat")]
    pub IASetIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, u32),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    IASetIndexBuffer: usize,
    pub DrawIndexedInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, i32, u32),
    pub DrawInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32),
    pub GSSetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3dcommon")]
    pub IASetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, D3D10_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_d3dcommon"))]
    IASetPrimitiveTopology: usize,
    pub VSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL),
    pub GSSetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub OMSetRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OMSetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32, u32),
    pub OMSetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub SOSetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *const u32),
    pub DrawAuto: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub RSSetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RSSetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D10_VIEWPORT),
    #[cfg(feature = "Win32_windef")]
    pub RSSetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D10_RECT),
    #[cfg(not(feature = "Win32_windef"))]
    RSSetScissorRects: usize,
    pub CopySubresourceRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, u32, *const D3D10_BOX),
    pub CopyResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub UpdateSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D10_BOX, *const core::ffi::c_void, u32, u32),
    pub ClearRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, f32, u8),
    pub GenerateMips: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_dxgiformat")]
    pub ResolveSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, super::dxgiformat::DXGI_FORMAT),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    ResolveSubresource: usize,
    pub VSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub PSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub PSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub IAGetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub IAGetVertexBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    #[cfg(feature = "Win32_dxgiformat")]
    pub IAGetIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::dxgiformat::DXGI_FORMAT, *mut u32),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    IAGetIndexBuffer: usize,
    pub GSGetConstantBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(feature = "Win32_d3dcommon")]
    pub IAGetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PRIMITIVE_TOPOLOGY),
    #[cfg(not(feature = "Win32_d3dcommon"))]
    IAGetPrimitiveTopology: usize,
    pub VSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL),
    pub GSGetShaderResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetSamplers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub OMGetRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub OMGetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut f32, *mut u32),
    pub OMGetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub SOGetTargets: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32),
    pub RSGetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub RSGetViewports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D10_VIEWPORT),
    #[cfg(feature = "Win32_windef")]
    pub RSGetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D10_RECT),
    #[cfg(not(feature = "Win32_windef"))]
    RSGetScissorRects: usize,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub CreateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_BUFFER_DESC, *const D3D10_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateTexture1D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_TEXTURE1D_DESC, *const D3D10_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateTexture1D: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateTexture2D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_TEXTURE2D_DESC, *const D3D10_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateTexture2D: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateTexture3D: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_TEXTURE3D_DESC, *const D3D10_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateTexture3D: usize,
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub CreateShaderResourceView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_SHADER_RESOURCE_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat")))]
    CreateShaderResourceView: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_RENDER_TARGET_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateRenderTargetView: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_DEPTH_STENCIL_VIEW_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateDepthStencilView: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_INPUT_ELEMENT_DESC, u32, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateInputLayout: usize,
    pub CreateVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryShaderWithStreamOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *const D3D10_SO_DECLARATION_ENTRY, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_BLEND_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_DEPTH_STENCIL_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRasterizerState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_RASTERIZER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_SAMPLER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_QUERY_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePredicate: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_QUERY_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_COUNTER_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CheckFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CheckFormatSupport: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CheckMultisampleQualityLevels: usize,
    pub CheckCounterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_COUNTER_INFO),
    pub CheckCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_COUNTER_DESC, *mut D3D10_COUNTER_TYPE, *mut u32, windows_core::PSTR, *mut u32, windows_core::PSTR, *mut u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_winnt")]
    pub OpenSharedResource: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OpenSharedResource: usize,
    pub SetTextFilterSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    pub GetTextFilterSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32),
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait ID3D10Device_Impl: windows_core::IUnknownImpl {
    fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn PSSetShader(&self, ppixelshader: windows_core::Ref<ID3D10PixelShader>);
    fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn VSSetShader(&self, pvertexshader: windows_core::Ref<ID3D10VertexShader>);
    fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&self, vertexcount: u32, startvertexlocation: u32);
    fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn IASetInputLayout(&self, pinputlayout: windows_core::Ref<ID3D10InputLayout>);
    fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&self, pindexbuffer: windows_core::Ref<ID3D10Buffer>, format: super::dxgiformat::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn GSSetShader(&self, pshader: windows_core::Ref<ID3D10GeometryShader>);
    fn IASetPrimitiveTopology(&self, topology: D3D10_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn SetPredication(&self, ppredicate: windows_core::Ref<ID3D10Predicate>, predicatevalue: windows_core::BOOL);
    fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn OMSetRenderTargets(&self, numviews: u32, pprendertargetviews: *const Option<ID3D10RenderTargetView>, pdepthstencilview: windows_core::Ref<ID3D10DepthStencilView>);
    fn OMSetBlendState(&self, pblendstate: windows_core::Ref<ID3D10BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&self, pdepthstencilstate: windows_core::Ref<ID3D10DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const Option<ID3D10Buffer>, poffsets: *const u32);
    fn DrawAuto(&self);
    fn RSSetState(&self, prasterizerstate: windows_core::Ref<ID3D10RasterizerState>);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D10_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const D3D10_RECT);
    fn CopySubresourceRegion(&self, pdstresource: windows_core::Ref<ID3D10Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: windows_core::Ref<ID3D10Resource>, srcsubresource: u32, psrcbox: *const D3D10_BOX);
    fn CopyResource(&self, pdstresource: windows_core::Ref<ID3D10Resource>, psrcresource: windows_core::Ref<ID3D10Resource>);
    fn UpdateSubresource(&self, pdstresource: windows_core::Ref<ID3D10Resource>, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ClearRenderTargetView(&self, prendertargetview: windows_core::Ref<ID3D10RenderTargetView>, colorrgba: *const f32);
    fn ClearDepthStencilView(&self, pdepthstencilview: windows_core::Ref<ID3D10DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&self, pshaderresourceview: windows_core::Ref<ID3D10ShaderResourceView>);
    fn ResolveSubresource(&self, pdstresource: windows_core::Ref<ID3D10Resource>, dstsubresource: u32, psrcresource: windows_core::Ref<ID3D10Resource>, srcsubresource: u32, format: super::dxgiformat::DXGI_FORMAT);
    fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn PSGetShader(&self, pppixelshader: windows_core::OutRef<ID3D10PixelShader>);
    fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn VSGetShader(&self, ppvertexshader: windows_core::OutRef<ID3D10VertexShader>);
    fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn IAGetInputLayout(&self, ppinputlayout: windows_core::OutRef<ID3D10InputLayout>);
    fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: windows_core::OutRef<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&self, pindexbuffer: windows_core::OutRef<ID3D10Buffer>, format: *mut super::dxgiformat::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn GSGetShader(&self, ppgeometryshader: windows_core::OutRef<ID3D10GeometryShader>);
    fn IAGetPrimitiveTopology(&self, ptopology: *mut D3D10_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn GetPredication(&self, pppredicate: windows_core::OutRef<ID3D10Predicate>, ppredicatevalue: *mut windows_core::BOOL);
    fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut Option<ID3D10RenderTargetView>, ppdepthstencilview: windows_core::OutRef<ID3D10DepthStencilView>);
    fn OMGetBlendState(&self, ppblendstate: windows_core::OutRef<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&self, ppdepthstencilstate: windows_core::OutRef<ID3D10DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&self, numbuffers: u32, ppsotargets: windows_core::OutRef<ID3D10Buffer>, poffsets: *mut u32);
    fn RSGetState(&self, pprasterizerstate: windows_core::OutRef<ID3D10RasterizerState>);
    fn RSGetViewports(&self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT);
    fn RSGetScissorRects(&self, numrects: *mut u32, prects: *mut D3D10_RECT);
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ClearState(&self);
    fn Flush(&self);
    fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: windows_core::OutRef<ID3D10Buffer>) -> windows_core::Result<()>;
    fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture1D>;
    fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture2D>;
    fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture3D>;
    fn CreateShaderResourceView(&self, presource: windows_core::Ref<ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: windows_core::OutRef<ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn CreateRenderTargetView(&self, presource: windows_core::Ref<ID3D10Resource>, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: windows_core::OutRef<ID3D10RenderTargetView>) -> windows_core::Result<()>;
    fn CreateDepthStencilView(&self, presource: windows_core::Ref<ID3D10Resource>, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: windows_core::OutRef<ID3D10DepthStencilView>) -> windows_core::Result<()>;
    fn CreateInputLayout(&self, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: windows_core::OutRef<ID3D10InputLayout>) -> windows_core::Result<()>;
    fn CreateVertexShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppvertexshader: windows_core::OutRef<ID3D10VertexShader>) -> windows_core::Result<()>;
    fn CreateGeometryShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppgeometryshader: windows_core::OutRef<ID3D10GeometryShader>) -> windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: windows_core::OutRef<ID3D10GeometryShader>) -> windows_core::Result<()>;
    fn CreatePixelShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pppixelshader: windows_core::OutRef<ID3D10PixelShader>) -> windows_core::Result<()>;
    fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: windows_core::OutRef<ID3D10BlendState>) -> windows_core::Result<()>;
    fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: windows_core::OutRef<ID3D10DepthStencilState>) -> windows_core::Result<()>;
    fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: windows_core::OutRef<ID3D10RasterizerState>) -> windows_core::Result<()>;
    fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: windows_core::OutRef<ID3D10SamplerState>) -> windows_core::Result<()>;
    fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC, ppquery: windows_core::OutRef<ID3D10Query>) -> windows_core::Result<()>;
    fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: windows_core::OutRef<ID3D10Predicate>) -> windows_core::Result<()>;
    fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: windows_core::OutRef<ID3D10Counter>) -> windows_core::Result<()>;
    fn CheckFormatSupport(&self, format: super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(&self, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D10_COUNTER_INFO);
    fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::Result<()>;
    fn GetCreationFlags(&self) -> u32;
    fn OpenSharedResource(&self, hresource: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetTextFilterSize(&self, width: u32, height: u32);
    fn GetTextFilterSize(&self, pwidth: *mut u32, pheight: *mut u32);
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl ID3D10Device_Vtbl {
    pub const fn new<Identity: ID3D10Device_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn PSSetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelshader: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSSetShader(this, core::mem::transmute_copy(&ppixelshader));
            }
        }
        unsafe extern "system" fn PSSetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn VSSetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexshader: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSSetShader(this, core::mem::transmute_copy(&pvertexshader));
            }
        }
        unsafe extern "system" fn DrawIndexed<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::DrawIndexed(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation));
            }
        }
        unsafe extern "system" fn Draw<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::Draw(this, core::mem::transmute_copy(&vertexcount), core::mem::transmute_copy(&startvertexlocation));
            }
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn IASetInputLayout<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputlayout: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IASetInputLayout(this, core::mem::transmute_copy(&pinputlayout));
            }
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut core::ffi::c_void, pstrides: *const u32, poffsets: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IASetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets));
            }
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, offset: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IASetIndexBuffer(this, core::mem::transmute_copy(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset));
            }
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::DrawIndexedInstanced(this, core::mem::transmute_copy(&indexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation), core::mem::transmute_copy(&startinstancelocation));
            }
        }
        unsafe extern "system" fn DrawInstanced<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::DrawInstanced(this, core::mem::transmute_copy(&vertexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startvertexlocation), core::mem::transmute_copy(&startinstancelocation));
            }
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn GSSetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSSetShader(this, core::mem::transmute_copy(&pshader));
            }
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, topology: D3D10_PRIMITIVE_TOPOLOGY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IASetPrimitiveTopology(this, core::mem::transmute_copy(&topology));
            }
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn VSSetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn SetPredication<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicate: *mut core::ffi::c_void, predicatevalue: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetPredication(this, core::mem::transmute_copy(&ppredicate), core::mem::transmute_copy(&predicatevalue));
            }
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn GSSetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMSetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&pdepthstencilview));
            }
        }
        unsafe extern "system" fn OMSetBlendState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstate: *mut core::ffi::c_void, blendfactor: *const f32, samplemask: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMSetBlendState(this, core::mem::transmute_copy(&pblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&samplemask));
            }
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilstate: *mut core::ffi::c_void, stencilref: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMSetDepthStencilState(this, core::mem::transmute_copy(&pdepthstencilstate), core::mem::transmute_copy(&stencilref));
            }
        }
        unsafe extern "system" fn SOSetTargets<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut core::ffi::c_void, poffsets: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SOSetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets), core::mem::transmute_copy(&poffsets));
            }
        }
        unsafe extern "system" fn DrawAuto<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::DrawAuto(this);
            }
        }
        unsafe extern "system" fn RSSetState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerstate: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSSetState(this, core::mem::transmute_copy(&prasterizerstate));
            }
        }
        unsafe extern "system" fn RSSetViewports<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSSetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports));
            }
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: u32, prects: *const D3D10_RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSSetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects));
            }
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CopySubresourceRegion(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), core::mem::transmute_copy(&dstz), core::mem::transmute_copy(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox));
            }
        }
        unsafe extern "system" fn CopyResource<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, psrcresource: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CopyResource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&psrcresource));
            }
        }
        unsafe extern "system" fn UpdateSubresource<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::UpdateSubresource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch));
            }
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertargetview: *mut core::ffi::c_void, colorrgba: *const f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::ClearRenderTargetView(this, core::mem::transmute_copy(&prendertargetview), core::mem::transmute_copy(&colorrgba));
            }
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::ClearDepthStencilView(this, core::mem::transmute_copy(&pdepthstencilview), core::mem::transmute_copy(&clearflags), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&stencil));
            }
        }
        unsafe extern "system" fn GenerateMips<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderresourceview: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GenerateMips(this, core::mem::transmute_copy(&pshaderresourceview));
            }
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, format: super::dxgiformat::DXGI_FORMAT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::ResolveSubresource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&format));
            }
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn PSGetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppixelshader: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSGetShader(this, core::mem::transmute_copy(&pppixelshader));
            }
        }
        unsafe extern "system" fn PSGetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn VSGetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvertexshader: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSGetShader(this, core::mem::transmute_copy(&ppvertexshader));
            }
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::PSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinputlayout: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IAGetInputLayout(this, core::mem::transmute_copy(&ppinputlayout));
            }
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IAGetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets));
            }
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut *mut core::ffi::c_void, format: *mut super::dxgiformat::DXGI_FORMAT, offset: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IAGetIndexBuffer(this, core::mem::transmute_copy(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset));
            }
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers));
            }
        }
        unsafe extern "system" fn GSGetShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSGetShader(this, core::mem::transmute_copy(&ppgeometryshader));
            }
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut D3D10_PRIMITIVE_TOPOLOGY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::IAGetPrimitiveTopology(this, core::mem::transmute_copy(&ptopology));
            }
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn VSGetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::VSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn GetPredication<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppredicate: *mut *mut core::ffi::c_void, ppredicatevalue: *mut windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetPredication(this, core::mem::transmute_copy(&pppredicate), core::mem::transmute_copy(&ppredicatevalue));
            }
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews));
            }
        }
        unsafe extern "system" fn GSGetSamplers<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers));
            }
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut core::ffi::c_void, ppdepthstencilview: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMGetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&ppdepthstencilview));
            }
        }
        unsafe extern "system" fn OMGetBlendState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblendstate: *mut *mut core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMGetBlendState(this, core::mem::transmute_copy(&ppblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&psamplemask));
            }
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdepthstencilstate: *mut *mut core::ffi::c_void, pstencilref: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OMGetDepthStencilState(this, core::mem::transmute_copy(&ppdepthstencilstate), core::mem::transmute_copy(&pstencilref));
            }
        }
        unsafe extern "system" fn SOGetTargets<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut core::ffi::c_void, poffsets: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SOGetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets), core::mem::transmute_copy(&poffsets));
            }
        }
        unsafe extern "system" fn RSGetState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprasterizerstate: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSGetState(this, core::mem::transmute_copy(&pprasterizerstate));
            }
        }
        unsafe extern "system" fn RSGetViewports<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSGetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports));
            }
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: *mut u32, prects: *mut D3D10_RECT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::RSGetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects));
            }
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetDeviceRemovedReason(this).into()
            }
        }
        unsafe extern "system" fn SetExceptionMode<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, raiseflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetExceptionMode(this, core::mem::transmute_copy(&raiseflags)).into()
            }
        }
        unsafe extern "system" fn GetExceptionMode<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetExceptionMode(this)
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn ClearState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::ClearState(this);
            }
        }
        unsafe extern "system" fn Flush<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::Flush(this);
            }
        }
        unsafe extern "system" fn CreateBuffer<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateBuffer(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn CreateTexture1D<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CreateTexture1D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                    Ok(ok__) => {
                        pptexture1d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTexture2D<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CreateTexture2D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                    Ok(ok__) => {
                        pptexture2d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTexture3D<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CreateTexture3D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                    Ok(ok__) => {
                        pptexture3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateShaderResourceView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppsrview)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateRenderTargetView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pprtview)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateDepthStencilView(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppdepthstencilview)).into()
            }
        }
        unsafe extern "system" fn CreateInputLayout<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateInputLayout(this, core::mem::transmute_copy(&pinputelementdescs), core::mem::transmute_copy(&numelements), core::mem::transmute_copy(&pshaderbytecodewithinputsignature), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppinputlayout)).into()
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppvertexshader)).into()
            }
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateGeometryShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppgeometryshader)).into()
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateGeometryShaderWithStreamOutput(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&psodeclaration), core::mem::transmute_copy(&numentries), core::mem::transmute_copy(&outputstreamstride), core::mem::transmute_copy(&ppgeometryshader)).into()
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pppixelshader)).into()
            }
        }
        unsafe extern "system" fn CreateBlendState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateBlendState(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateDepthStencilState(this, core::mem::transmute_copy(&pdepthstencildesc), core::mem::transmute_copy(&ppdepthstencilstate)).into()
            }
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateRasterizerState(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
            }
        }
        unsafe extern "system" fn CreateSamplerState<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateSamplerState(this, core::mem::transmute_copy(&psamplerdesc), core::mem::transmute_copy(&ppsamplerstate)).into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateQuery(this, core::mem::transmute_copy(&pquerydesc), core::mem::transmute_copy(&ppquery)).into()
            }
        }
        unsafe extern "system" fn CreatePredicate<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreatePredicate(this, core::mem::transmute_copy(&ppredicatedesc), core::mem::transmute_copy(&pppredicate)).into()
            }
        }
        unsafe extern "system" fn CreateCounter<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CreateCounter(this, core::mem::transmute_copy(&pcounterdesc), core::mem::transmute_copy(&ppcounter)).into()
            }
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, pformatsupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CheckFormatSupport(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pformatsupport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Device_Impl::CheckMultisampleQualityLevels(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&samplecount)) {
                    Ok(ok__) => {
                        pnumqualitylevels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CheckCounterInfo(this, core::mem::transmute_copy(&pcounterinfo));
            }
        }
        unsafe extern "system" fn CheckCounter<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::CheckCounter(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pactivecounters), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pnamelength), core::mem::transmute_copy(&szunits), core::mem::transmute_copy(&punitslength), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pdescriptionlength)).into()
            }
        }
        unsafe extern "system" fn GetCreationFlags<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetCreationFlags(this)
            }
        }
        unsafe extern "system" fn OpenSharedResource<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::OpenSharedResource(this, core::mem::transmute_copy(&hresource), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
            }
        }
        unsafe extern "system" fn SetTextFilterSize<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::SetTextFilterSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height));
            }
        }
        unsafe extern "system" fn GetTextFilterSize<Identity: ID3D10Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device_Impl::GetTextFilterSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, OFFSET>,
            PSSetShader: PSSetShader::<Identity, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, OFFSET>,
            VSSetShader: VSSetShader::<Identity, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, OFFSET>,
            GSSetShader: GSSetShader::<Identity, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, OFFSET>,
            SetPredication: SetPredication::<Identity, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, OFFSET>,
            DrawAuto: DrawAuto::<Identity, OFFSET>,
            RSSetState: RSSetState::<Identity, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, OFFSET>,
            CopyResource: CopyResource::<Identity, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, OFFSET>,
            GenerateMips: GenerateMips::<Identity, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, OFFSET>,
            PSGetShader: PSGetShader::<Identity, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, OFFSET>,
            VSGetShader: VSGetShader::<Identity, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, OFFSET>,
            GSGetShader: GSGetShader::<Identity, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, OFFSET>,
            GetPredication: GetPredication::<Identity, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, OFFSET>,
            OMGetBlendState: OMGetBlendState::<Identity, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, OFFSET>,
            RSGetState: RSGetState::<Identity, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            ClearState: ClearState::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            CreateBuffer: CreateBuffer::<Identity, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, OFFSET>,
            CreateCounter: CreateCounter::<Identity, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, OFFSET>,
            CheckCounter: CheckCounter::<Identity, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, OFFSET>,
            SetTextFilterSize: SetTextFilterSize::<Identity, OFFSET>,
            GetTextFilterSize: GetTextFilterSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D10Device {}
windows_core::imp::define_interface!(ID3D10DeviceChild, ID3D10DeviceChild_Vtbl, 0x9b7e4c00_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D10DeviceChild, windows_core::IUnknown);
impl ID3D10DeviceChild {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D10Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize as _, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10DeviceChild_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ID3D10DeviceChild_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: windows_core::OutRef<ID3D10Device>);
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ID3D10DeviceChild_Vtbl {
    pub const fn new<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice));
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DeviceChild_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DeviceChild_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ID3D10DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10DeviceChild_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10DeviceChild {}
windows_core::imp::define_interface!(ID3D10GeometryShader, ID3D10GeometryShader_Vtbl, 0x6316be88_54cd_4040_ab44_20461bc81f68);
impl core::ops::Deref for ID3D10GeometryShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10GeometryShader, windows_core::IUnknown, ID3D10DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10GeometryShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
pub trait ID3D10GeometryShader_Impl: ID3D10DeviceChild_Impl {}
impl ID3D10GeometryShader_Vtbl {
    pub const fn new<Identity: ID3D10GeometryShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10GeometryShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10GeometryShader {}
windows_core::imp::define_interface!(ID3D10InputLayout, ID3D10InputLayout_Vtbl, 0x9b7e4c0b_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10InputLayout {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10InputLayout, windows_core::IUnknown, ID3D10DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10InputLayout_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
pub trait ID3D10InputLayout_Impl: ID3D10DeviceChild_Impl {}
impl ID3D10InputLayout_Vtbl {
    pub const fn new<Identity: ID3D10InputLayout_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10InputLayout as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10InputLayout {}
windows_core::imp::define_interface!(ID3D10Multithread, ID3D10Multithread_Vtbl, 0x9b7e4e00_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D10Multithread, windows_core::IUnknown);
impl ID3D10Multithread {
    pub unsafe fn Enter(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Leave(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Leave)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn SetMultithreadProtected(&self, bmtprotect: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).SetMultithreadProtected)(windows_core::Interface::as_raw(self), bmtprotect.into()) }
    }
    pub unsafe fn GetMultithreadProtected(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMultithreadProtected)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Multithread_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    pub GetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait ID3D10Multithread_Impl: windows_core::IUnknownImpl {
    fn Enter(&self);
    fn Leave(&self);
    fn SetMultithreadProtected(&self, bmtprotect: windows_core::BOOL) -> windows_core::BOOL;
    fn GetMultithreadProtected(&self) -> windows_core::BOOL;
}
impl ID3D10Multithread_Vtbl {
    pub const fn new<Identity: ID3D10Multithread_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enter<Identity: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Multithread_Impl::Enter(this);
            }
        }
        unsafe extern "system" fn Leave<Identity: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Multithread_Impl::Leave(this);
            }
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmtprotect: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Multithread_Impl::SetMultithreadProtected(this, core::mem::transmute_copy(&bmtprotect))
            }
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: ID3D10Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Multithread_Impl::GetMultithreadProtected(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, OFFSET>,
            Leave: Leave::<Identity, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Multithread as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Multithread {}
windows_core::imp::define_interface!(ID3D10PixelShader, ID3D10PixelShader_Vtbl, 0x4968b601_9d00_4cde_8346_8e7f675819b6);
impl core::ops::Deref for ID3D10PixelShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10PixelShader, windows_core::IUnknown, ID3D10DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10PixelShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
pub trait ID3D10PixelShader_Impl: ID3D10DeviceChild_Impl {}
impl ID3D10PixelShader_Vtbl {
    pub const fn new<Identity: ID3D10PixelShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10PixelShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10PixelShader {}
windows_core::imp::define_interface!(ID3D10Predicate, ID3D10Predicate_Vtbl, 0x9b7e4c10_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Predicate {
    type Target = ID3D10Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Predicate, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Asynchronous, ID3D10Query);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Predicate_Vtbl {
    pub base__: ID3D10Query_Vtbl,
}
pub trait ID3D10Predicate_Impl: ID3D10Query_Impl {}
impl ID3D10Predicate_Vtbl {
    pub const fn new<Identity: ID3D10Predicate_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10Query_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Predicate as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID || iid == &<ID3D10Query as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Predicate {}
windows_core::imp::define_interface!(ID3D10Query, ID3D10Query_Vtbl, 0x9b7e4c0e_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Query {
    type Target = ID3D10Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Query, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Asynchronous);
impl ID3D10Query {
    pub unsafe fn GetDesc(&self) -> D3D10_QUERY_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Query_Vtbl {
    pub base__: ID3D10Asynchronous_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_QUERY_DESC),
}
pub trait ID3D10Query_Impl: ID3D10Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_QUERY_DESC);
}
impl ID3D10Query_Vtbl {
    pub const fn new<Identity: ID3D10Query_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10Query_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Query_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10Asynchronous_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Query as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Query {}
windows_core::imp::define_interface!(ID3D10RasterizerState, ID3D10RasterizerState_Vtbl, 0xa2a07292_89af_4345_be2e_c53d9fbb6e9f);
impl core::ops::Deref for ID3D10RasterizerState {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10RasterizerState, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10RasterizerState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_RASTERIZER_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10RasterizerState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RASTERIZER_DESC),
}
pub trait ID3D10RasterizerState_Impl: ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_RASTERIZER_DESC);
}
impl ID3D10RasterizerState_Vtbl {
    pub const fn new<Identity: ID3D10RasterizerState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10RasterizerState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10RasterizerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10RasterizerState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10RasterizerState {}
windows_core::imp::define_interface!(ID3D10RenderTargetView, ID3D10RenderTargetView_Vtbl, 0x9b7e4c08_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10RenderTargetView {
    type Target = ID3D10View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10RenderTargetView, windows_core::IUnknown, ID3D10DeviceChild, ID3D10View);
impl ID3D10RenderTargetView {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10RenderTargetView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RENDER_TARGET_VIEW_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D10RenderTargetView_Impl: ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D10RenderTargetView_Vtbl {
    pub const fn new<Identity: ID3D10RenderTargetView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10RenderTargetView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10RenderTargetView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10RenderTargetView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D10RenderTargetView {}
windows_core::imp::define_interface!(ID3D10Resource, ID3D10Resource_Vtbl, 0x9b7e4c01_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Resource {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Resource, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10Resource {
    pub unsafe fn GetType(&self) -> D3D10_RESOURCE_DIMENSION {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetEvictionPriority)(windows_core::Interface::as_raw(self), evictionpriority);
        }
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetEvictionPriority)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Resource_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_RESOURCE_DIMENSION),
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait ID3D10Resource_Impl: ID3D10DeviceChild_Impl {
    fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&self, evictionpriority: u32);
    fn GetEvictionPriority(&self) -> u32;
}
impl ID3D10Resource_Vtbl {
    pub const fn new<Identity: ID3D10Resource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Resource_Impl::GetType(this, core::mem::transmute_copy(&rtype));
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Resource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority));
            }
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ID3D10Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Resource_Impl::GetEvictionPriority(this)
            }
        }
        Self {
            base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Resource as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10Resource {}
windows_core::imp::define_interface!(ID3D10SamplerState, ID3D10SamplerState_Vtbl, 0x9b7e4c0c_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10SamplerState {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10SamplerState, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10SamplerState {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SAMPLER_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10SamplerState_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SAMPLER_DESC),
}
pub trait ID3D10SamplerState_Impl: ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SAMPLER_DESC);
}
impl ID3D10SamplerState_Vtbl {
    pub const fn new<Identity: ID3D10SamplerState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10SamplerState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10SamplerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10SamplerState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10SamplerState {}
windows_core::imp::define_interface!(ID3D10ShaderResourceView, ID3D10ShaderResourceView_Vtbl, 0x9b7e4c07_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10ShaderResourceView {
    type Target = ID3D10View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10ShaderResourceView, windows_core::IUnknown, ID3D10DeviceChild, ID3D10View);
impl ID3D10ShaderResourceView {
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderResourceView_Vtbl {
    pub base__: ID3D10View_Vtbl,
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_RESOURCE_VIEW_DESC),
    #[cfg(not(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat")))]
    GetDesc: usize,
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
pub trait ID3D10ShaderResourceView_Impl: ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl ID3D10ShaderResourceView_Vtbl {
    pub const fn new<Identity: ID3D10ShaderResourceView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderResourceView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderResourceView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D10ShaderResourceView {}
windows_core::imp::define_interface!(ID3D10Texture1D, ID3D10Texture1D_Vtbl, 0x9b7e4c03_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Texture1D {
    type Target = ID3D10Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Texture1D, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Resource);
impl ID3D10Texture1D {
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, ppdata as _) }
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource);
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture1D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE1D_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D10Texture1D_Impl: ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D10Texture1D_Vtbl {
    pub const fn new<Identity: ID3D10Texture1D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Map<Identity: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture1D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags), core::mem::transmute_copy(&ppdata)).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture1D_Impl::Unmap(this, core::mem::transmute_copy(&subresource));
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Texture1D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture1D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture1D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D10Texture1D {}
windows_core::imp::define_interface!(ID3D10Texture2D, ID3D10Texture2D_Vtbl, 0x9b7e4c04_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Texture2D {
    type Target = ID3D10Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Texture2D, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Resource);
impl ID3D10Texture2D {
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE2D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource);
        }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture2D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut D3D10_MAPPED_TEXTURE2D) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE2D_DESC),
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    GetDesc: usize,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D10Texture2D_Impl: ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE2D>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC);
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D10Texture2D_Vtbl {
    pub const fn new<Identity: ID3D10Texture2D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Map<Identity: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Texture2D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags)) {
                    Ok(ok__) => {
                        pmappedtex2d.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture2D_Impl::Unmap(this, core::mem::transmute_copy(&subresource));
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Texture2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture2D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture2D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D10Texture2D {}
windows_core::imp::define_interface!(ID3D10Texture3D, ID3D10Texture3D_Vtbl, 0x9b7e4c05_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10Texture3D {
    type Target = ID3D10Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10Texture3D, windows_core::IUnknown, ID3D10DeviceChild, ID3D10Resource);
impl ID3D10Texture3D {
    pub unsafe fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), subresource, maptype, mapflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unmap(&self, subresource: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self), subresource);
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Texture3D_Vtbl {
    pub base__: ID3D10Resource_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3D10_MAP, u32, *mut D3D10_MAPPED_TEXTURE3D) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TEXTURE3D_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_dxgiformat")]
pub trait ID3D10Texture3D_Impl: ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE3D>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_dxgiformat")]
impl ID3D10Texture3D_Vtbl {
    pub const fn new<Identity: ID3D10Texture3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Map<Identity: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Texture3D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags)) {
                    Ok(ok__) => {
                        pmappedtex3d.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture3D_Impl::Unmap(this, core::mem::transmute_copy(&subresource));
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Texture3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Texture3D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture3D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dxgiformat")]
impl windows_core::RuntimeName for ID3D10Texture3D {}
windows_core::imp::define_interface!(ID3D10VertexShader, ID3D10VertexShader_Vtbl, 0x9b7e4c0a_342c_4106_a19f_4f2704f689f0);
impl core::ops::Deref for ID3D10VertexShader {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10VertexShader, windows_core::IUnknown, ID3D10DeviceChild);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10VertexShader_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
}
pub trait ID3D10VertexShader_Impl: ID3D10DeviceChild_Impl {}
impl ID3D10VertexShader_Vtbl {
    pub const fn new<Identity: ID3D10VertexShader_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10VertexShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10VertexShader {}
windows_core::imp::define_interface!(ID3D10View, ID3D10View_Vtbl, 0xc902b03f_60a7_49ba_9936_2a3ab37a7e33);
impl core::ops::Deref for ID3D10View {
    type Target = ID3D10DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10View, windows_core::IUnknown, ID3D10DeviceChild);
impl ID3D10View {
    pub unsafe fn GetResource(&self) -> windows_core::Result<ID3D10Resource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10View_Vtbl {
    pub base__: ID3D10DeviceChild_Vtbl,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait ID3D10View_Impl: ID3D10DeviceChild_Impl {
    fn GetResource(&self, ppresource: windows_core::OutRef<ID3D10Resource>);
}
impl ID3D10View_Vtbl {
    pub const fn new<Identity: ID3D10View_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource<Identity: ID3D10View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10View_Impl::GetResource(this, core::mem::transmute_copy(&ppresource));
            }
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetResource: GetResource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10View as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10View {}
