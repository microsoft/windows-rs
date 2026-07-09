#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10CompileShader(psrcdata : windows_sys::core::PCSTR, srcdatasize : usize, pfilename : windows_sys::core::PCSTR, pdefines : *const D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pfunctionname : windows_sys::core::PCSTR, pprofile : windows_sys::core::PCSTR, flags : u32, ppshader : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10DisassembleShader(pshader : *const core::ffi::c_void, bytecodelength : usize, enablecolorcode : windows_sys::core::BOOL, pcomments : windows_sys::core::PCSTR, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3d10")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetGeometryShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_sys::core::PCSTR);
#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetInputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3d10")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetPixelShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_sys::core::PCSTR);
#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetShaderDebugInfo(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppdebuginfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3d10")]
windows_link::link!("d3d10.dll" "system" fn D3D10GetVertexShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_sys::core::PCSTR);
#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10PreprocessShader(psrcdata : windows_sys::core::PCSTR, srcdatasize : usize, pfilename : windows_sys::core::PCSTR, pdefines : *const D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, ppshadertext : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10ReflectShader(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppreflector : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const D3D10_ALL_RESOURCES_BOUND: u32 = 2097152;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_CBUFFER_TYPE = super::d3dcommon::D3D_CBUFFER_TYPE;
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_INCLUDE_TYPE = super::d3dcommon::D3D_INCLUDE_TYPE;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_NAME = super::d3dcommon::D3D_NAME;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_REGISTER_COMPONENT_TYPE = super::d3dcommon::D3D_REGISTER_COMPONENT_TYPE;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_RESOURCE_RETURN_TYPE = super::d3dcommon::D3D_RESOURCE_RETURN_TYPE;
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_BUFFER_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: D3D10_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D10_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SHADER_CBUFFER_FLAGS = super::d3dcommon::D3D_SHADER_CBUFFER_FLAGS;
pub const D3D10_SHADER_DEBUG: u32 = 1;
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608;
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
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
    pub GSOutputTopology: super::d3d10::D3D10_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
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
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_INPUT_BIND_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Type: D3D10_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: D3D10_RESOURCE_RETURN_TYPE,
    pub Dimension: super::d3d10::D3D10_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
impl Default for D3D10_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SHADER_INPUT_FLAGS = super::d3dcommon::D3D_SHADER_INPUT_FLAGS;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SHADER_INPUT_TYPE = super::d3dcommon::D3D_SHADER_INPUT_TYPE;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SHADER_MACRO = super::d3dcommon::D3D_SHADER_MACRO;
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
pub const D3D10_SHADER_SKIP_OPTIMIZATION: u32 = 4;
pub const D3D10_SHADER_SKIP_VALIDATION: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
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
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SHADER_VARIABLE_CLASS = super::d3dcommon::D3D_SHADER_VARIABLE_CLASS;
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
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SHADER_VARIABLE_FLAGS = super::d3dcommon::D3D_SHADER_VARIABLE_FLAGS;
#[cfg(feature = "Win32_d3dcommon")]
pub type D3D10_SHADER_VARIABLE_TYPE = super::d3dcommon::D3D_SHADER_VARIABLE_TYPE;
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
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
#[cfg(feature = "Win32_d3dcommon")]
impl Default for D3D10_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
pub type LPD3D10_CBUFFER_TYPE = *mut D3D10_CBUFFER_TYPE;
#[cfg(feature = "Win32_d3dcommon")]
pub type LPD3D10_SHADER_CBUFFER_FLAGS = *mut D3D10_SHADER_CBUFFER_FLAGS;
#[cfg(feature = "Win32_d3dcommon")]
pub type LPD3D10_SHADER_INPUT_FLAGS = *mut D3D10_SHADER_INPUT_FLAGS;
#[cfg(feature = "Win32_d3dcommon")]
pub type LPD3D10_SHADER_INPUT_TYPE = *mut D3D10_SHADER_INPUT_TYPE;
#[cfg(feature = "Win32_d3dcommon")]
pub type LPD3D10_SHADER_MACRO = *mut D3D10_SHADER_MACRO;
#[cfg(feature = "Win32_d3dcommon")]
pub type LPD3D10_SHADER_VARIABLE_CLASS = *mut D3D10_SHADER_VARIABLE_CLASS;
#[cfg(feature = "Win32_d3dcommon")]
pub type LPD3D10_SHADER_VARIABLE_FLAGS = *mut D3D10_SHADER_VARIABLE_FLAGS;
#[cfg(feature = "Win32_d3dcommon")]
pub type LPD3D10_SHADER_VARIABLE_TYPE = *mut D3D10_SHADER_VARIABLE_TYPE;
