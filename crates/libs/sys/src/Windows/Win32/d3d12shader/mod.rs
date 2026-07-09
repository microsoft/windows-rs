#[cfg(feature = "Win32_d3dcommon")]
pub type D3D12_CBUFFER_TYPE = super::d3dcommon::D3D_CBUFFER_TYPE;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D12_FUNCTION_DESC {
    pub Version: u32,
    pub Creator: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
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
    pub MovInstructionCount: u32,
    pub MovcInstructionCount: u32,
    pub ConversionInstructionCount: u32,
    pub BitwiseInstructionCount: u32,
    pub MinFeatureLevel: super::d3dcommon::D3D_FEATURE_LEVEL,
    pub RequiredFeatureFlags: u64,
    pub Name: windows_sys::core::PCSTR,
    pub FunctionParameterCount: i32,
    pub HasReturn: windows_sys::core::BOOL,
    pub Has10Level9VertexShader: windows_sys::core::BOOL,
    pub Has10Level9PixelShader: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D12_FUNCTION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_LIBRARY_DESC {
    pub Creator: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
impl Default for D3D12_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D12_PARAMETER_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub SemanticName: windows_sys::core::PCSTR,
    pub Type: super::d3dcommon::D3D_SHADER_VARIABLE_TYPE,
    pub Class: super::d3dcommon::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub InterpolationMode: super::d3dcommon::D3D_INTERPOLATION_MODE,
    pub Flags: super::d3dcommon::D3D_PARAMETER_FLAGS,
    pub FirstInRegister: u32,
    pub FirstInComponent: u32,
    pub FirstOutRegister: u32,
    pub FirstOutComponent: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D12_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D12_RESOURCE_RETURN_TYPE = super::d3dcommon::D3D_RESOURCE_RETURN_TYPE;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D12_SHADER_BUFFER_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: super::d3dcommon::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D12_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D12_SHADER_DESC {
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
    pub GSOutputTopology: super::d3dcommon::D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
    pub InputPrimitive: super::d3dcommon::D3D_PRIMITIVE,
    pub PatchConstantParameters: u32,
    pub cGSInstanceCount: u32,
    pub cControlPoints: u32,
    pub HSOutputPrimitive: super::d3dcommon::D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    pub HSPartitioning: super::d3dcommon::D3D_TESSELLATOR_PARTITIONING,
    pub TessellatorDomain: super::d3dcommon::D3D_TESSELLATOR_DOMAIN,
    pub cBarrierInstructions: u32,
    pub cInterlockedInstructions: u32,
    pub cTextureStoreInstructions: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D12_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D12_SHADER_INPUT_BIND_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: super::d3dcommon::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::d3dcommon::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::d3dcommon::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
    pub Space: u32,
    pub uID: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D12_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D12_SHADER_TYPE_DESC {
    pub Class: super::d3dcommon::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::d3dcommon::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
    pub Name: windows_sys::core::PCSTR,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D12_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_SHADER_VARIABLE_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut core::ffi::c_void,
    pub StartTexture: u32,
    pub TextureSize: u32,
    pub StartSampler: u32,
    pub SamplerSize: u32,
}
impl Default for D3D12_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_SHADER_VERSION_TYPE = i32;
pub const D3D12_SHVER_AMPLIFICATION_SHADER: D3D12_SHADER_VERSION_TYPE = 14;
pub const D3D12_SHVER_ANY_HIT_SHADER: D3D12_SHADER_VERSION_TYPE = 9;
pub const D3D12_SHVER_CALLABLE_SHADER: D3D12_SHADER_VERSION_TYPE = 12;
pub const D3D12_SHVER_CLOSEST_HIT_SHADER: D3D12_SHADER_VERSION_TYPE = 10;
pub const D3D12_SHVER_COMPUTE_SHADER: D3D12_SHADER_VERSION_TYPE = 5;
pub const D3D12_SHVER_DOMAIN_SHADER: D3D12_SHADER_VERSION_TYPE = 4;
pub const D3D12_SHVER_GEOMETRY_SHADER: D3D12_SHADER_VERSION_TYPE = 2;
pub const D3D12_SHVER_HULL_SHADER: D3D12_SHADER_VERSION_TYPE = 3;
pub const D3D12_SHVER_INTERSECTION_SHADER: D3D12_SHADER_VERSION_TYPE = 8;
pub const D3D12_SHVER_LIBRARY: D3D12_SHADER_VERSION_TYPE = 6;
pub const D3D12_SHVER_MESH_SHADER: D3D12_SHADER_VERSION_TYPE = 13;
pub const D3D12_SHVER_MISS_SHADER: D3D12_SHADER_VERSION_TYPE = 11;
pub const D3D12_SHVER_NODE_SHADER: D3D12_SHADER_VERSION_TYPE = 15;
pub const D3D12_SHVER_PIXEL_SHADER: D3D12_SHADER_VERSION_TYPE = 0;
pub const D3D12_SHVER_RAY_GENERATION_SHADER: D3D12_SHADER_VERSION_TYPE = 7;
pub const D3D12_SHVER_RESERVED0: D3D12_SHADER_VERSION_TYPE = 65520;
pub const D3D12_SHVER_VERTEX_SHADER: D3D12_SHADER_VERSION_TYPE = 1;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D12_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: windows_sys::core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::d3dcommon::D3D_NAME,
    pub ComponentType: super::d3dcommon::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
    pub Stream: u32,
    pub MinPrecision: super::d3dcommon::D3D_MIN_PRECISION,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D12_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D12_TESSELLATOR_DOMAIN = super::d3dcommon::D3D_TESSELLATOR_DOMAIN;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D12_TESSELLATOR_OUTPUT_PRIMITIVE = super::d3dcommon::D3D_TESSELLATOR_OUTPUT_PRIMITIVE;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D12_TESSELLATOR_PARTITIONING = super::d3dcommon::D3D_TESSELLATOR_PARTITIONING;
pub const D3D_SHADER_REQUIRES_ADVANCED_TEXTURE_OPS: u32 = 536870912;
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_DESCRIPTOR_HEAP_RESOURCE: u32 = 268435456;
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_GROUP_SHARED: u32 = 8388608;
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_TYPED_RESOURCE: u32 = 4194304;
pub const D3D_SHADER_REQUIRES_BARYCENTRICS: u32 = 131072;
pub const D3D_SHADER_REQUIRES_DERIVATIVES_IN_MESH_AND_AMPLIFICATION_SHADERS: u32 = 16777216;
pub const D3D_SHADER_REQUIRES_EXTENDED_COMMAND_INFO: u64 = 4294967296;
pub const D3D_SHADER_REQUIRES_INNER_COVERAGE: u32 = 1024;
pub const D3D_SHADER_REQUIRES_INT64_OPS: u32 = 32768;
pub const D3D_SHADER_REQUIRES_NATIVE_16BIT_OPS: u32 = 262144;
pub const D3D_SHADER_REQUIRES_RAYTRACING_TIER_1_1: u32 = 1048576;
pub const D3D_SHADER_REQUIRES_RESOURCE_DESCRIPTOR_HEAP_INDEXING: u32 = 33554432;
pub const D3D_SHADER_REQUIRES_ROVS: u32 = 4096;
pub const D3D_SHADER_REQUIRES_SAMPLER_DESCRIPTOR_HEAP_INDEXING: u32 = 67108864;
pub const D3D_SHADER_REQUIRES_SAMPLER_FEEDBACK: u32 = 2097152;
pub const D3D_SHADER_REQUIRES_SAMPLE_CMP_GRADIENT_OR_BIAS: u32 = 2147483648;
pub const D3D_SHADER_REQUIRES_SHADING_RATE: u32 = 524288;
pub const D3D_SHADER_REQUIRES_STENCIL_REF: u32 = 512;
pub const D3D_SHADER_REQUIRES_TYPED_UAV_LOAD_ADDITIONAL_FORMATS: u32 = 2048;
pub const D3D_SHADER_REQUIRES_VIEWPORT_AND_RT_ARRAY_INDEX_FROM_ANY_SHADER_FEEDING_RASTERIZER: u32 = 8192;
pub const D3D_SHADER_REQUIRES_VIEW_ID: u32 = 65536;
pub const D3D_SHADER_REQUIRES_WAVE_MMA: u32 = 134217728;
pub const D3D_SHADER_REQUIRES_WAVE_OPS: u32 = 16384;
pub const D3D_SHADER_REQUIRES_WRITEABLE_MSAA_TEXTURES: u32 = 1073741824;
