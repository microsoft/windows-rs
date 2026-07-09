#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_FILE_INFO {
    pub FileName: u32,
    pub FileNameLen: u32,
    pub FileData: u32,
    pub FileLen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_INPUT_INFO {
    pub Var: u32,
    pub InitialRegisterSet: D3D10_SHADER_DEBUG_REGTYPE,
    pub InitialBank: u32,
    pub InitialRegister: u32,
    pub InitialComponent: u32,
    pub InitialValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DEBUG_OUTPUTVAR {
    pub Var: u32,
    pub uValueMin: u32,
    pub uValueMax: u32,
    pub iValueMin: i32,
    pub iValueMax: i32,
    pub fValueMin: f32,
    pub fValueMax: f32,
    pub bNaNPossible: windows_core::BOOL,
    pub bInfPossible: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
windows_core::imp::define_interface!(ID3D10ShaderReflection1, ID3D10ShaderReflection1_Vtbl, 0xc3457783_a846_47ce_9520_cea6f66e7447);
windows_core::imp::interface_hierarchy!(ID3D10ShaderReflection1, windows_core::IUnknown);
impl ID3D10ShaderReflection1 {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut super::d3d10shader::D3D10_SHADER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    #[cfg(feature = "Win32_d3d10shader")]
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<super::d3d10shader::ID3D10ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    #[cfg(feature = "Win32_d3d10shader")]
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<super::d3d10shader::ID3D10ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut super::d3d10shader::D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc as _) }
    }
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3d10shader")]
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<super::d3d10shader::ID3D10ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut super::d3d10shader::D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc as _) }
    }
    pub unsafe fn GetMovInstructionCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMovInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMovcInstructionCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMovcInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConversionInstructionCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConversionInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBitwiseInstructionCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitwiseInstructionCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetGSInputPrimitive(&self) -> windows_core::Result<super::d3d10::D3D10_PRIMITIVE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGSInputPrimitive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLevel9Shader(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLevel9Shader)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSampleFrequencyShader(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSampleFrequencyShader)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflection1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d10shader::D3D10_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_d3d10shader")]
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<super::d3d10shader::ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(not(feature = "Win32_d3d10shader"))]
    GetConstantBufferByIndex: usize,
    #[cfg(feature = "Win32_d3d10shader")]
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<super::d3d10shader::ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(not(feature = "Win32_d3d10shader"))]
    GetConstantBufferByName: usize,
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d10shader::D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon")))]
    GetResourceBindingDesc: usize,
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub GetInputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon")))]
    GetInputParameterDesc: usize,
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub GetOutputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon")))]
    GetOutputParameterDesc: usize,
    #[cfg(feature = "Win32_d3d10shader")]
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<super::d3d10shader::ID3D10ShaderReflectionVariable>,
    #[cfg(not(feature = "Win32_d3d10shader"))]
    GetVariableByName: usize,
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut super::d3d10shader::D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon")))]
    GetResourceBindingDescByName: usize,
    pub GetMovInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMovcInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConversionInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBitwiseInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
    pub GetGSInputPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3d10::D3D10_PRIMITIVE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon")))]
    GetGSInputPrimitive: usize,
    pub IsLevel9Shader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsSampleFrequencyShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
pub trait ID3D10ShaderReflection1_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDesc(&self, pdesc: *mut super::d3d10shader::D3D10_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<super::d3d10shader::ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<super::d3d10shader::ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut super::d3d10shader::D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<super::d3d10shader::ID3D10ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut super::d3d10shader::D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetMovInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetMovcInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetConversionInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetBitwiseInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetGSInputPrimitive(&self) -> windows_core::Result<super::d3d10::D3D10_PRIMITIVE>;
    fn IsLevel9Shader(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsSampleFrequencyShader(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl ID3D10ShaderReflection1_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut super::d3d10shader::D3D10_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<super::d3d10shader::ID3D10ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<super::d3d10shader::ID3D10ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut super::d3d10shader::D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<super::d3d10shader::ID3D10ShaderReflectionVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut super::d3d10shader::D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection1_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetMovInstructionCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetMovcInstructionCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetConversionInstructionCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetBitwiseInstructionCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprim: *mut super::d3d10::D3D10_PRIMITIVE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::GetGSInputPrimitive(this) {
                    Ok(ok__) => {
                        pprim.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLevel9Shader<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblevel9shader: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::IsLevel9Shader(this) {
                    Ok(ok__) => {
                        pblevel9shader.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ID3D10ShaderReflection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsamplefrequency: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10ShaderReflection1_Impl::IsSampleFrequencyShader(this) {
                    Ok(ok__) => {
                        pbsamplefrequency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, OFFSET>,
            IsLevel9Shader: IsLevel9Shader::<Identity, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl windows_core::RuntimeName for ID3D10ShaderReflection1 {}
