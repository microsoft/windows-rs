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
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    pub TokenId: u32,
    pub VarType: D3D10_SHADER_DEBUG_VARTYPE,
    pub Class: super::d3d10shader::D3D10_SHADER_VARIABLE_CLASS,
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
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
#[derive(Clone, Copy, Default)]
pub struct D3D10_SHADER_DEBUG_VAR_INFO {
    pub TokenId: u32,
    pub Type: super::d3d10shader::D3D10_SHADER_VARIABLE_TYPE,
    pub Register: u32,
    pub Component: u32,
    pub ScopeVar: u32,
    pub ScopeVarOffset: u32,
}
pub const D3D10_SHADER_DEBUG_VAR_VARIABLE: D3D10_SHADER_DEBUG_VARTYPE = 0;
pub const D3D11_SHADER_DEBUG_REG_INTERFACE_POINTERS: D3D10_SHADER_DEBUG_REGTYPE = 11;
pub const D3D11_SHADER_DEBUG_REG_UAV: D3D10_SHADER_DEBUG_REGTYPE = 12;
