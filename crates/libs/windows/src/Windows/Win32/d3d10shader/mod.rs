#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10CompileShader<P2, P4, P5, P6>(psrcdata: &[u8], pfilename: P2, pdefines: Option<*const D3D10_SHADER_MACRO>, pinclude: P4, pfunctionname: P5, pprofile: P6, flags: u32, ppshader: *mut Option<super::d3dcommon::ID3D10Blob>, pperrormsgs: Option<*mut Option<super::d3dcommon::ID3D10Blob>>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::d3dcommon::ID3DInclude>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CompileShader(psrcdata : windows_core::PCSTR, srcdatasize : usize, pfilename : windows_core::PCSTR, pdefines : *const D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pfunctionname : windows_core::PCSTR, pprofile : windows_core::PCSTR, flags : u32, ppshader : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CompileShader(core::mem::transmute(psrcdata.as_ptr()), psrcdata.len().try_into().unwrap(), pfilename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), pfunctionname.param().abi(), pprofile.param().abi(), flags, core::mem::transmute(ppshader), pperrormsgs.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10DisassembleShader<P3>(pshader: *const core::ffi::c_void, bytecodelength: usize, enablecolorcode: bool, pcomments: P3) -> windows_core::Result<super::d3dcommon::ID3D10Blob>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10DisassembleShader(pshader : *const core::ffi::c_void, bytecodelength : usize, enablecolorcode : windows_core::BOOL, pcomments : windows_core::PCSTR, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10DisassembleShader(pshader, bytecodelength, enablecolorcode.into(), pcomments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_d3d10")]
#[inline]
pub unsafe fn D3D10GetGeometryShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<super::d3d10::ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10GetGeometryShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_core::PCSTR);
    unsafe { D3D10GetGeometryShaderProfile(pdevice.param().abi()) }
}
#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::d3dcommon::ID3D10Blob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10GetInputAndOutputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10GetInputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::d3dcommon::ID3D10Blob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10GetInputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10GetInputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::d3dcommon::ID3D10Blob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10GetOutputSignatureBlob(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10GetOutputSignatureBlob(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_d3d10")]
#[inline]
pub unsafe fn D3D10GetPixelShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<super::d3d10::ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10GetPixelShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_core::PCSTR);
    unsafe { D3D10GetPixelShaderProfile(pdevice.param().abi()) }
}
#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10GetShaderDebugInfo(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<super::d3dcommon::ID3D10Blob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10GetShaderDebugInfo(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppdebuginfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10GetShaderDebugInfo(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_d3d10")]
#[inline]
pub unsafe fn D3D10GetVertexShaderProfile<P0>(pdevice: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<super::d3d10::ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10GetVertexShaderProfile(pdevice : *mut core::ffi::c_void) -> windows_core::PCSTR);
    unsafe { D3D10GetVertexShaderProfile(pdevice.param().abi()) }
}
#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10PreprocessShader<P2, P4>(psrcdata: &[u8], pfilename: P2, pdefines: Option<*const D3D10_SHADER_MACRO>, pinclude: P4, ppshadertext: *mut Option<super::d3dcommon::ID3D10Blob>, pperrormsgs: Option<*mut Option<super::d3dcommon::ID3D10Blob>>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::d3dcommon::ID3DInclude>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10PreprocessShader(psrcdata : windows_core::PCSTR, srcdatasize : usize, pfilename : windows_core::PCSTR, pdefines : *const D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, ppshadertext : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10PreprocessShader(core::mem::transmute(psrcdata.as_ptr()), psrcdata.len().try_into().unwrap(), pfilename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), core::mem::transmute(ppshadertext), pperrormsgs.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn D3D10ReflectShader(pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize) -> windows_core::Result<ID3D10ShaderReflection> {
    windows_core::link!("d3d10.dll" "system" fn D3D10ReflectShader(pshaderbytecode : *const core::ffi::c_void, bytecodelength : usize, ppreflector : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10ReflectShader(pshaderbytecode, bytecodelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub const D3D10_ALL_RESOURCES_BOUND: u32 = 2097152;
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_CBUFFER_TYPE(pub super::d3dcommon::D3D_CBUFFER_TYPE);
pub const D3D10_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576;
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_INCLUDE_TYPE(pub super::d3dcommon::D3D_INCLUDE_TYPE);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_NAME(pub super::d3dcommon::D3D_NAME);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_REGISTER_COMPONENT_TYPE(pub super::d3dcommon::D3D_REGISTER_COMPONENT_TYPE);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_RESOURCE_RETURN_TYPE(pub super::d3dcommon::D3D_RESOURCE_RETURN_TYPE);
pub const D3D10_SHADER_AVOID_FLOW_CONTROL: u32 = 512;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_BUFFER_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: D3D10_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_SHADER_CBUFFER_FLAGS(pub super::d3dcommon::D3D_SHADER_CBUFFER_FLAGS);
pub const D3D10_SHADER_DEBUG: u32 = 1;
pub const D3D10_SHADER_DEBUG_NAME_FOR_BINARY: u32 = 8388608;
pub const D3D10_SHADER_DEBUG_NAME_FOR_SOURCE: u32 = 4194304;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_DESC {
    pub Version: u32,
    pub Creator: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SHADER_INPUT_BIND_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: D3D10_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: D3D10_RESOURCE_RETURN_TYPE,
    pub Dimension: super::d3d10::D3D10_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_SHADER_INPUT_FLAGS(pub super::d3dcommon::D3D_SHADER_INPUT_FLAGS);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_SHADER_INPUT_TYPE(pub super::d3dcommon::D3D_SHADER_INPUT_TYPE);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_SHADER_VARIABLE_CLASS(pub super::d3dcommon::D3D_SHADER_VARIABLE_CLASS);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_SHADER_VARIABLE_DESC {
    pub Name: windows_core::PCSTR,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_SHADER_VARIABLE_FLAGS(pub super::d3dcommon::D3D_SHADER_VARIABLE_FLAGS);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_SHADER_VARIABLE_TYPE(pub super::d3dcommon::D3D_SHADER_VARIABLE_TYPE);
pub const D3D10_SHADER_WARNINGS_ARE_ERRORS: u32 = 262144;
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: D3D10_NAME,
    pub ComponentType: D3D10_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
}
windows_core::imp::define_interface!(ID3D10ShaderReflection, ID3D10ShaderReflection_Vtbl, 0xd40e20b6_f8f7_42ad_ab20_4baf8f15dfaa);
windows_core::imp::interface_hierarchy!(ID3D10ShaderReflection, windows_core::IUnknown);
impl ID3D10ShaderReflection {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon")))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>,
    #[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon")))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetInputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetOutputParameterDesc: usize,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
pub trait ID3D10ShaderReflection_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
impl ID3D10ShaderReflection_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ID3D10ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderReflection_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon"))]
impl windows_core::RuntimeName for ID3D10ShaderReflection {}
windows_core::imp::define_interface!(ID3D10ShaderReflectionConstantBuffer, ID3D10ShaderReflectionConstantBuffer_Vtbl);
impl ID3D10ShaderReflectionConstantBuffer {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D10ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D10ShaderReflectionConstantBuffer_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D10ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflectionConstantBuffer_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionConstantBuffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionConstantBuffer_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionConstantBuffer_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        Self { GetDesc: GetDesc::<Identity>, GetVariableByIndex: GetVariableByIndex::<Identity>, GetVariableByName: GetVariableByName::<Identity> }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
struct ID3D10ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D10ShaderReflectionConstantBuffer_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D10ShaderReflectionConstantBuffer_Impl> ID3D10ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionConstantBuffer_Vtbl = ID3D10ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D10ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D10ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10ShaderReflectionType, ID3D10ShaderReflectionType_Vtbl);
impl ID3D10ShaderReflectionType {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetMemberTypeByIndex(&self, index: u32) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberTypeByName<P0>(&self, name: P0) -> Option<Self>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeName)(windows_core::Interface::as_raw(self), index) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D10ShaderReflectionType_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D10ShaderReflectionType_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflectionType_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberTypeName<Identity: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionType_Impl::GetMemberTypeName(this, core::mem::transmute_copy(&index))
            }
        }
        Self {
            GetDesc: GetDesc::<Identity>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity>,
            GetMemberTypeName: GetMemberTypeName::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
struct ID3D10ShaderReflectionType_ImplVtbl<T: ID3D10ShaderReflectionType_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D10ShaderReflectionType_Impl> ID3D10ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionType_Vtbl = ID3D10ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D10ShaderReflectionType {
    pub fn new<'a, T: ID3D10ShaderReflectionType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10ShaderReflectionVariable, ID3D10ShaderReflectionVariable_Vtbl);
impl ID3D10ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10ShaderReflectionType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10ShaderReflectionType>,
}
pub trait ID3D10ShaderReflectionVariable_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetType(&self) -> Option<ID3D10ShaderReflectionType>;
}
impl ID3D10ShaderReflectionVariable_Vtbl {
    pub const fn new<Identity: ID3D10ShaderReflectionVariable_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D10ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10ShaderReflectionVariable_Impl::GetType(this)
            }
        }
        Self { GetDesc: GetDesc::<Identity>, GetType: GetType::<Identity> }
    }
}
struct ID3D10ShaderReflectionVariable_ImplVtbl<T: ID3D10ShaderReflectionVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10ShaderReflectionVariable_Impl> ID3D10ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionVariable_Vtbl = ID3D10ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D10ShaderReflectionVariable {
    pub fn new<'a, T: ID3D10ShaderReflectionVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3D10_CBUFFER_TYPE(pub *mut D3D10_CBUFFER_TYPE);
#[cfg(feature = "Win32_d3dcommon")]
impl LPD3D10_CBUFFER_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for LPD3D10_CBUFFER_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3D10_SHADER_CBUFFER_FLAGS(pub *mut D3D10_SHADER_CBUFFER_FLAGS);
#[cfg(feature = "Win32_d3dcommon")]
impl LPD3D10_SHADER_CBUFFER_FLAGS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for LPD3D10_SHADER_CBUFFER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3D10_SHADER_INPUT_FLAGS(pub *mut D3D10_SHADER_INPUT_FLAGS);
#[cfg(feature = "Win32_d3dcommon")]
impl LPD3D10_SHADER_INPUT_FLAGS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for LPD3D10_SHADER_INPUT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3D10_SHADER_INPUT_TYPE(pub *mut D3D10_SHADER_INPUT_TYPE);
#[cfg(feature = "Win32_d3dcommon")]
impl LPD3D10_SHADER_INPUT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for LPD3D10_SHADER_INPUT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3D10_SHADER_MACRO(pub *mut D3D10_SHADER_MACRO);
#[cfg(feature = "Win32_d3dcommon")]
impl LPD3D10_SHADER_MACRO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for LPD3D10_SHADER_MACRO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3D10_SHADER_VARIABLE_CLASS(pub *mut D3D10_SHADER_VARIABLE_CLASS);
#[cfg(feature = "Win32_d3dcommon")]
impl LPD3D10_SHADER_VARIABLE_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for LPD3D10_SHADER_VARIABLE_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3D10_SHADER_VARIABLE_FLAGS(pub *mut D3D10_SHADER_VARIABLE_FLAGS);
#[cfg(feature = "Win32_d3dcommon")]
impl LPD3D10_SHADER_VARIABLE_FLAGS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for LPD3D10_SHADER_VARIABLE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3D10_SHADER_VARIABLE_TYPE(pub *mut D3D10_SHADER_VARIABLE_TYPE);
#[cfg(feature = "Win32_d3dcommon")]
impl LPD3D10_SHADER_VARIABLE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl Default for LPD3D10_SHADER_VARIABLE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
