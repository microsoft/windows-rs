#[cfg(feature = "Win32_d3dcommon")]
pub type D3D11_CBUFFER_TYPE = super::d3dcommon::D3D_CBUFFER_TYPE;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_FUNCTION_DESC {
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
impl Default for D3D11_FUNCTION_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_LIBRARY_DESC {
    pub Creator: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
impl Default for D3D11_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_PARAMETER_DESC {
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
impl Default for D3D11_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D11_RESOURCE_RETURN_TYPE = super::d3dcommon::D3D_RESOURCE_RETURN_TYPE;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_BUFFER_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: super::d3dcommon::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D11_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_DESC {
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
impl Default for D3D11_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_INPUT_BIND_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: super::d3dcommon::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::d3dcommon::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::d3dcommon::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D11_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_TYPE_DESC {
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
impl Default for D3D11_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_VARIABLE_DESC {
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
impl Default for D3D11_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_SHADER_VERSION_TYPE = i32;
pub const D3D11_SHVER_COMPUTE_SHADER: D3D11_SHADER_VERSION_TYPE = 5;
pub const D3D11_SHVER_DOMAIN_SHADER: D3D11_SHADER_VERSION_TYPE = 4;
pub const D3D11_SHVER_GEOMETRY_SHADER: D3D11_SHADER_VERSION_TYPE = 2;
pub const D3D11_SHVER_HULL_SHADER: D3D11_SHADER_VERSION_TYPE = 3;
pub const D3D11_SHVER_PIXEL_SHADER: D3D11_SHADER_VERSION_TYPE = 0;
pub const D3D11_SHVER_RESERVED0: D3D11_SHADER_VERSION_TYPE = 65520;
pub const D3D11_SHVER_VERTEX_SHADER: D3D11_SHADER_VERSION_TYPE = 1;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D11_SIGNATURE_PARAMETER_DESC {
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
impl Default for D3D11_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D11_TESSELLATOR_DOMAIN = super::d3dcommon::D3D_TESSELLATOR_DOMAIN;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D11_TESSELLATOR_OUTPUT_PRIMITIVE = super::d3dcommon::D3D_TESSELLATOR_OUTPUT_PRIMITIVE;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D11_TESSELLATOR_PARTITIONING = super::d3dcommon::D3D_TESSELLATOR_PARTITIONING;
pub const D3D_RETURN_PARAMETER_INDEX: i32 = -1;
pub const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: u32 = 32;
pub const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: u32 = 64;
pub const D3D_SHADER_REQUIRES_64_UAVS: u32 = 8;
pub const D3D_SHADER_REQUIRES_DOUBLES: u32 = 1;
pub const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: u32 = 2;
pub const D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING: u32 = 128;
pub const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: u32 = 16;
pub const D3D_SHADER_REQUIRES_TILED_RESOURCES: u32 = 256;
pub const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: u32 = 4;
