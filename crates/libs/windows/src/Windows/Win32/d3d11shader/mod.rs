#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_CBUFFER_TYPE(pub super::d3dcommon::D3D_CBUFFER_TYPE);
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FUNCTION_DESC {
    pub Version: u32,
    pub Creator: windows_core::PCSTR,
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
    pub Name: windows_core::PCSTR,
    pub FunctionParameterCount: i32,
    pub HasReturn: windows_core::BOOL,
    pub Has10Level9VertexShader: windows_core::BOOL,
    pub Has10Level9PixelShader: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_LIBRARY_DESC {
    pub Creator: windows_core::PCSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_PARAMETER_DESC {
    pub Name: windows_core::PCSTR,
    pub SemanticName: windows_core::PCSTR,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_RESOURCE_RETURN_TYPE(pub super::d3dcommon::D3D_RESOURCE_RETURN_TYPE);
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_SHADER_BUFFER_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::d3dcommon::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_SHADER_DESC {
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
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_SHADER_INPUT_BIND_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::d3dcommon::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::d3dcommon::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::d3dcommon::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_SHADER_TYPE_DESC {
    pub Class: super::d3dcommon::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::d3dcommon::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
    pub Name: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_SHADER_VARIABLE_DESC {
    pub Name: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: windows_core::PCSTR,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_TESSELLATOR_DOMAIN(pub super::d3dcommon::D3D_TESSELLATOR_DOMAIN);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_TESSELLATOR_OUTPUT_PRIMITIVE(pub super::d3dcommon::D3D_TESSELLATOR_OUTPUT_PRIMITIVE);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_TESSELLATOR_PARTITIONING(pub super::d3dcommon::D3D_TESSELLATOR_PARTITIONING);
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
windows_core::imp::define_interface!(ID3D11FunctionLinkingGraph, ID3D11FunctionLinkingGraph_Vtbl, 0x54133220_1ce8_43d3_8236_9855c5ceecff);
windows_core::imp::interface_hierarchy!(ID3D11FunctionLinkingGraph, windows_core::IUnknown);
impl ID3D11FunctionLinkingGraph {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn CreateModuleInstance(&self, ppmoduleinstance: *mut Option<ID3D11ModuleInstance>, pperrorbuffer: *mut Option<super::d3dcommon::ID3D10Blob>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateModuleInstance)(windows_core::Interface::as_raw(self), core::mem::transmute(ppmoduleinstance), core::mem::transmute(pperrorbuffer)) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn SetInputSignature(&self, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32) -> windows_core::Result<ID3D11LinkingNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetInputSignature)(windows_core::Interface::as_raw(self), pinputparameters, cinputparameters, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn SetOutputSignature(&self, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32) -> windows_core::Result<ID3D11LinkingNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetOutputSignature)(windows_core::Interface::as_raw(self), poutputparameters, coutputparameters, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CallFunction<P0, P1, P2>(&self, pmoduleinstancenamespace: P0, pmodulewithfunctionprototype: P1, pfunctionname: P2) -> windows_core::Result<ID3D11LinkingNode>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<ID3D11Module>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CallFunction)(windows_core::Interface::as_raw(self), pmoduleinstancenamespace.param().abi(), pmodulewithfunctionprototype.param().abi(), pfunctionname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PassValue<P0, P2>(&self, psrcnode: P0, srcparameterindex: i32, pdstnode: P2, dstparameterindex: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11LinkingNode>,
        P2: windows_core::Param<ID3D11LinkingNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).PassValue)(windows_core::Interface::as_raw(self), psrcnode.param().abi(), srcparameterindex, pdstnode.param().abi(), dstparameterindex) }
    }
    pub unsafe fn PassValueWithSwizzle<P0, P2, P3, P5>(&self, psrcnode: P0, srcparameterindex: i32, psrcswizzle: P2, pdstnode: P3, dstparameterindex: i32, pdstswizzle: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11LinkingNode>,
        P2: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<ID3D11LinkingNode>,
        P5: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PassValueWithSwizzle)(windows_core::Interface::as_raw(self), psrcnode.param().abi(), srcparameterindex, psrcswizzle.param().abi(), pdstnode.param().abi(), dstparameterindex, pdstswizzle.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetLastError(&self) -> windows_core::Result<super::d3dcommon::ID3D10Blob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GenerateHlsl(&self, uflags: u32) -> windows_core::Result<super::d3dcommon::ID3D10Blob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateHlsl)(windows_core::Interface::as_raw(self), uflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11FunctionLinkingGraph_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3dcommon")]
    pub CreateModuleInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    CreateModuleInstance: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub SetInputSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_PARAMETER_DESC, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    SetInputSignature: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub SetOutputSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_PARAMETER_DESC, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    SetOutputSignature: usize,
    pub CallFunction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut core::ffi::c_void, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PassValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PassValueWithSwizzle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, windows_core::PCSTR, *mut core::ffi::c_void, i32, windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetLastError: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GenerateHlsl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GenerateHlsl: usize,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D11FunctionLinkingGraph_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn CreateModuleInstance(&self, ppmoduleinstance: windows_core::OutRef<ID3D11ModuleInstance>, pperrorbuffer: windows_core::OutRef<super::d3dcommon::ID3D10Blob>) -> windows_core::Result<()>;
    fn SetInputSignature(&self, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32) -> windows_core::Result<ID3D11LinkingNode>;
    fn SetOutputSignature(&self, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32) -> windows_core::Result<ID3D11LinkingNode>;
    fn CallFunction(&self, pmoduleinstancenamespace: &windows_core::PCSTR, pmodulewithfunctionprototype: windows_core::Ref<ID3D11Module>, pfunctionname: &windows_core::PCSTR) -> windows_core::Result<ID3D11LinkingNode>;
    fn PassValue(&self, psrcnode: windows_core::Ref<ID3D11LinkingNode>, srcparameterindex: i32, pdstnode: windows_core::Ref<ID3D11LinkingNode>, dstparameterindex: i32) -> windows_core::Result<()>;
    fn PassValueWithSwizzle(&self, psrcnode: windows_core::Ref<ID3D11LinkingNode>, srcparameterindex: i32, psrcswizzle: &windows_core::PCSTR, pdstnode: windows_core::Ref<ID3D11LinkingNode>, dstparameterindex: i32, pdstswizzle: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn GetLastError(&self) -> windows_core::Result<super::d3dcommon::ID3D10Blob>;
    fn GenerateHlsl(&self, uflags: u32) -> windows_core::Result<super::d3dcommon::ID3D10Blob>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11FunctionLinkingGraph_Vtbl {
    pub const fn new<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11FunctionLinkingGraph_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11FunctionLinkingGraph_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11FunctionLinkingGraph_Impl::Release(this)
            }
        }
        unsafe extern "system" fn CreateModuleInstance<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmoduleinstance: *mut *mut core::ffi::c_void, pperrorbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11FunctionLinkingGraph_Impl::CreateModuleInstance(this, core::mem::transmute_copy(&ppmoduleinstance), core::mem::transmute_copy(&pperrorbuffer)).into()
            }
        }
        unsafe extern "system" fn SetInputSignature<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32, ppinputnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11FunctionLinkingGraph_Impl::SetInputSignature(this, core::mem::transmute_copy(&pinputparameters), core::mem::transmute_copy(&cinputparameters)) {
                    Ok(ok__) => {
                        ppinputnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutputSignature<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32, ppoutputnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11FunctionLinkingGraph_Impl::SetOutputSignature(this, core::mem::transmute_copy(&poutputparameters), core::mem::transmute_copy(&coutputparameters)) {
                    Ok(ok__) => {
                        ppoutputnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CallFunction<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmoduleinstancenamespace: windows_core::PCSTR, pmodulewithfunctionprototype: *mut core::ffi::c_void, pfunctionname: windows_core::PCSTR, ppcallnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11FunctionLinkingGraph_Impl::CallFunction(this, core::mem::transmute(&pmoduleinstancenamespace), core::mem::transmute_copy(&pmodulewithfunctionprototype), core::mem::transmute(&pfunctionname)) {
                    Ok(ok__) => {
                        ppcallnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PassValue<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcnode: *mut core::ffi::c_void, srcparameterindex: i32, pdstnode: *mut core::ffi::c_void, dstparameterindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11FunctionLinkingGraph_Impl::PassValue(this, core::mem::transmute_copy(&psrcnode), core::mem::transmute_copy(&srcparameterindex), core::mem::transmute_copy(&pdstnode), core::mem::transmute_copy(&dstparameterindex)).into()
            }
        }
        unsafe extern "system" fn PassValueWithSwizzle<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcnode: *mut core::ffi::c_void, srcparameterindex: i32, psrcswizzle: windows_core::PCSTR, pdstnode: *mut core::ffi::c_void, dstparameterindex: i32, pdstswizzle: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11FunctionLinkingGraph_Impl::PassValueWithSwizzle(this, core::mem::transmute_copy(&psrcnode), core::mem::transmute_copy(&srcparameterindex), core::mem::transmute(&psrcswizzle), core::mem::transmute_copy(&pdstnode), core::mem::transmute_copy(&dstparameterindex), core::mem::transmute(&pdstswizzle)).into()
            }
        }
        unsafe extern "system" fn GetLastError<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperrorbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11FunctionLinkingGraph_Impl::GetLastError(this) {
                    Ok(ok__) => {
                        pperrorbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateHlsl<Identity: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11FunctionLinkingGraph_Impl::GenerateHlsl(this, core::mem::transmute_copy(&uflags)) {
                    Ok(ok__) => {
                        ppbuffer.write(core::mem::transmute(ok__));
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
            CreateModuleInstance: CreateModuleInstance::<Identity, OFFSET>,
            SetInputSignature: SetInputSignature::<Identity, OFFSET>,
            SetOutputSignature: SetOutputSignature::<Identity, OFFSET>,
            CallFunction: CallFunction::<Identity, OFFSET>,
            PassValue: PassValue::<Identity, OFFSET>,
            PassValueWithSwizzle: PassValueWithSwizzle::<Identity, OFFSET>,
            GetLastError: GetLastError::<Identity, OFFSET>,
            GenerateHlsl: GenerateHlsl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11FunctionLinkingGraph as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl windows_core::RuntimeName for ID3D11FunctionLinkingGraph {}
windows_core::imp::define_interface!(ID3D11FunctionParameterReflection, ID3D11FunctionParameterReflection_Vtbl);
impl ID3D11FunctionParameterReflection {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11FunctionParameterReflection_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D11FunctionParameterReflection_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11FunctionParameterReflection_Vtbl {
    pub const fn new<Identity: ID3D11FunctionParameterReflection_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11FunctionParameterReflection_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11FunctionParameterReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self { GetDesc: GetDesc::<Identity> }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
struct ID3D11FunctionParameterReflection_ImplVtbl<T: ID3D11FunctionParameterReflection_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D11FunctionParameterReflection_Impl> ID3D11FunctionParameterReflection_ImplVtbl<T> {
    const VTABLE: ID3D11FunctionParameterReflection_Vtbl = ID3D11FunctionParameterReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11FunctionParameterReflection {
    pub fn new<'a, T: ID3D11FunctionParameterReflection_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11FunctionParameterReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D11FunctionReflection, ID3D11FunctionReflection_Vtbl);
impl ID3D11FunctionReflection {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_FUNCTION_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, bufferindex: u32) -> Option<ID3D11ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), bufferindex) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc as _) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc as _) }
    }
    pub unsafe fn GetFunctionParameter(&self, parameterindex: i32) -> Option<ID3D11FunctionParameterReflection> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionParameter)(windows_core::Interface::as_raw(self), parameterindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11FunctionReflection_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_FUNCTION_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetResourceBindingDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetResourceBindingDescByName: usize,
    pub GetFunctionParameter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> Option<ID3D11FunctionParameterReflection>,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D11FunctionReflection_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_FUNCTION_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, bufferindex: u32) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetFunctionParameter(&self, parameterindex: i32) -> Option<ID3D11FunctionParameterReflection>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11FunctionReflection_Vtbl {
    pub const fn new<Identity: ID3D11FunctionReflection_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_FUNCTION_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11FunctionReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, bufferindex: u32) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11FunctionReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&bufferindex))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11FunctionReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11FunctionReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11FunctionReflection_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11FunctionReflection_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetFunctionParameter<Identity: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, parameterindex: i32) -> Option<ID3D11FunctionParameterReflection> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11FunctionReflection_Impl::GetFunctionParameter(this, core::mem::transmute_copy(&parameterindex))
            }
        }
        Self {
            GetDesc: GetDesc::<Identity>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity>,
            GetVariableByName: GetVariableByName::<Identity>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity>,
            GetFunctionParameter: GetFunctionParameter::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
struct ID3D11FunctionReflection_ImplVtbl<T: ID3D11FunctionReflection_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D11FunctionReflection_Impl> ID3D11FunctionReflection_ImplVtbl<T> {
    const VTABLE: ID3D11FunctionReflection_Vtbl = ID3D11FunctionReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11FunctionReflection {
    pub fn new<'a, T: ID3D11FunctionReflection_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11FunctionReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D11LibraryReflection, ID3D11LibraryReflection_Vtbl, 0x54384f1b_5b3e_4bb7_ae01_60ba3097cbb6);
windows_core::imp::interface_hierarchy!(ID3D11LibraryReflection, windows_core::IUnknown);
impl ID3D11LibraryReflection {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self) -> windows_core::Result<D3D11_LIBRARY_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFunctionByIndex(&self, functionindex: i32) -> Option<ID3D11FunctionReflection> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionByIndex)(windows_core::Interface::as_raw(self), functionindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11LibraryReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_LIBRARY_DESC) -> windows_core::HRESULT,
    pub GetFunctionByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> Option<ID3D11FunctionReflection>,
}
pub trait ID3D11LibraryReflection_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDesc(&self) -> windows_core::Result<D3D11_LIBRARY_DESC>;
    fn GetFunctionByIndex(&self, functionindex: i32) -> Option<ID3D11FunctionReflection>;
}
impl ID3D11LibraryReflection_Vtbl {
    pub const fn new<Identity: ID3D11LibraryReflection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11LibraryReflection_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11LibraryReflection_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11LibraryReflection_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_LIBRARY_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11LibraryReflection_Impl::GetDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Identity: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionindex: i32) -> Option<ID3D11FunctionReflection> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11LibraryReflection_Impl::GetFunctionByIndex(this, core::mem::transmute_copy(&functionindex))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11LibraryReflection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11LibraryReflection {}
windows_core::imp::define_interface!(ID3D11Linker, ID3D11Linker_Vtbl, 0x59a6cd0e_e10d_4c1f_88c0_63aba1daf30e);
windows_core::imp::interface_hierarchy!(ID3D11Linker, windows_core::IUnknown);
impl ID3D11Linker {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn Link<P0, P1, P2>(&self, pentry: P0, pentryname: P1, ptargetname: P2, uflags: u32, ppshaderblob: *mut Option<super::d3dcommon::ID3D10Blob>, pperrorbuffer: *mut Option<super::d3dcommon::ID3D10Blob>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11ModuleInstance>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Link)(windows_core::Interface::as_raw(self), pentry.param().abi(), pentryname.param().abi(), ptargetname.param().abi(), uflags, core::mem::transmute(ppshaderblob), core::mem::transmute(pperrorbuffer)) }
    }
    pub unsafe fn UseLibrary<P0>(&self, plibrarymi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11ModuleInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).UseLibrary)(windows_core::Interface::as_raw(self), plibrarymi.param().abi()) }
    }
    pub unsafe fn AddClipPlaneFromCBuffer(&self, ucbufferslot: u32, ucbufferentry: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddClipPlaneFromCBuffer)(windows_core::Interface::as_raw(self), ucbufferslot, ucbufferentry) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Linker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3dcommon")]
    pub Link: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    Link: usize,
    pub UseLibrary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddClipPlaneFromCBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D11Linker_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn Link(&self, pentry: windows_core::Ref<ID3D11ModuleInstance>, pentryname: &windows_core::PCSTR, ptargetname: &windows_core::PCSTR, uflags: u32, ppshaderblob: windows_core::OutRef<super::d3dcommon::ID3D10Blob>, pperrorbuffer: windows_core::OutRef<super::d3dcommon::ID3D10Blob>) -> windows_core::Result<()>;
    fn UseLibrary(&self, plibrarymi: windows_core::Ref<ID3D11ModuleInstance>) -> windows_core::Result<()>;
    fn AddClipPlaneFromCBuffer(&self, ucbufferslot: u32, ucbufferentry: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11Linker_Vtbl {
    pub const fn new<Identity: ID3D11Linker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Linker_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Linker_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Linker_Impl::Release(this)
            }
        }
        unsafe extern "system" fn Link<Identity: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentry: *mut core::ffi::c_void, pentryname: windows_core::PCSTR, ptargetname: windows_core::PCSTR, uflags: u32, ppshaderblob: *mut *mut core::ffi::c_void, pperrorbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Linker_Impl::Link(this, core::mem::transmute_copy(&pentry), core::mem::transmute(&pentryname), core::mem::transmute(&ptargetname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ppshaderblob), core::mem::transmute_copy(&pperrorbuffer)).into()
            }
        }
        unsafe extern "system" fn UseLibrary<Identity: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibrarymi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Linker_Impl::UseLibrary(this, core::mem::transmute_copy(&plibrarymi)).into()
            }
        }
        unsafe extern "system" fn AddClipPlaneFromCBuffer<Identity: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucbufferslot: u32, ucbufferentry: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Linker_Impl::AddClipPlaneFromCBuffer(this, core::mem::transmute_copy(&ucbufferslot), core::mem::transmute_copy(&ucbufferentry)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            UseLibrary: UseLibrary::<Identity, OFFSET>,
            AddClipPlaneFromCBuffer: AddClipPlaneFromCBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Linker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl windows_core::RuntimeName for ID3D11Linker {}
windows_core::imp::define_interface!(ID3D11LinkingNode, ID3D11LinkingNode_Vtbl, 0xd80dd70c_8d2f_4751_94a1_03c79b3556db);
windows_core::imp::interface_hierarchy!(ID3D11LinkingNode, windows_core::IUnknown);
impl ID3D11LinkingNode {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11LinkingNode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait ID3D11LinkingNode_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
}
impl ID3D11LinkingNode_Vtbl {
    pub const fn new<Identity: ID3D11LinkingNode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D11LinkingNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11LinkingNode_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D11LinkingNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11LinkingNode_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D11LinkingNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11LinkingNode_Impl::Release(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11LinkingNode as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11LinkingNode {}
windows_core::imp::define_interface!(ID3D11Module, ID3D11Module_Vtbl, 0xcac701ee_80fc_4122_8242_10b39c8cec34);
windows_core::imp::interface_hierarchy!(ID3D11Module, windows_core::IUnknown);
impl ID3D11Module {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateInstance<P0>(&self, pnamespace: P0) -> windows_core::Result<ID3D11ModuleInstance>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Module_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ID3D11Module_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn CreateInstance(&self, pnamespace: &windows_core::PCSTR) -> windows_core::Result<ID3D11ModuleInstance>;
}
impl ID3D11Module_Vtbl {
    pub const fn new<Identity: ID3D11Module_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D11Module_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Module_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D11Module_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Module_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D11Module_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Module_Impl::Release(this)
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ID3D11Module_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: windows_core::PCSTR, ppmoduleinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Module_Impl::CreateInstance(this, core::mem::transmute(&pnamespace)) {
                    Ok(ok__) => {
                        ppmoduleinstance.write(core::mem::transmute(ok__));
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
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Module as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Module {}
windows_core::imp::define_interface!(ID3D11ModuleInstance, ID3D11ModuleInstance_Vtbl, 0x469e07f7_045a_48d5_aa12_68a478cdf75d);
windows_core::imp::interface_hierarchy!(ID3D11ModuleInstance, windows_core::IUnknown);
impl ID3D11ModuleInstance {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BindConstantBuffer(&self, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindConstantBuffer)(windows_core::Interface::as_raw(self), usrcslot, udstslot, cbdstoffset) }
    }
    pub unsafe fn BindConstantBufferByName<P0>(&self, pname: P0, udstslot: u32, cbdstoffset: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindConstantBufferByName)(windows_core::Interface::as_raw(self), pname.param().abi(), udstslot, cbdstoffset) }
    }
    pub unsafe fn BindResource(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindResource)(windows_core::Interface::as_raw(self), usrcslot, udstslot, ucount) }
    }
    pub unsafe fn BindResourceByName<P0>(&self, pname: P0, udstslot: u32, ucount: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindResourceByName)(windows_core::Interface::as_raw(self), pname.param().abi(), udstslot, ucount) }
    }
    pub unsafe fn BindSampler(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindSampler)(windows_core::Interface::as_raw(self), usrcslot, udstslot, ucount) }
    }
    pub unsafe fn BindSamplerByName<P0>(&self, pname: P0, udstslot: u32, ucount: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindSamplerByName)(windows_core::Interface::as_raw(self), pname.param().abi(), udstslot, ucount) }
    }
    pub unsafe fn BindUnorderedAccessView(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindUnorderedAccessView)(windows_core::Interface::as_raw(self), usrcslot, udstslot, ucount) }
    }
    pub unsafe fn BindUnorderedAccessViewByName<P0>(&self, pname: P0, udstslot: u32, ucount: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindUnorderedAccessViewByName)(windows_core::Interface::as_raw(self), pname.param().abi(), udstslot, ucount) }
    }
    pub unsafe fn BindResourceAsUnorderedAccessView(&self, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindResourceAsUnorderedAccessView)(windows_core::Interface::as_raw(self), usrcsrvslot, udstuavslot, ucount) }
    }
    pub unsafe fn BindResourceAsUnorderedAccessViewByName<P0>(&self, psrvname: P0, udstuavslot: u32, ucount: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindResourceAsUnorderedAccessViewByName)(windows_core::Interface::as_raw(self), psrvname.param().abi(), udstuavslot, ucount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ModuleInstance_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub BindConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
    pub BindResource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindResourceByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
    pub BindSampler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindSamplerByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
    pub BindUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindUnorderedAccessViewByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
    pub BindResourceAsUnorderedAccessView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub BindResourceAsUnorderedAccessViewByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
}
pub trait ID3D11ModuleInstance_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn BindConstantBuffer(&self, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> windows_core::Result<()>;
    fn BindConstantBufferByName(&self, pname: &windows_core::PCSTR, udstslot: u32, cbdstoffset: u32) -> windows_core::Result<()>;
    fn BindResource(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindResourceByName(&self, pname: &windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindSampler(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindSamplerByName(&self, pname: &windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindUnorderedAccessView(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindUnorderedAccessViewByName(&self, pname: &windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindResourceAsUnorderedAccessView(&self, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindResourceAsUnorderedAccessViewByName(&self, psrvname: &windows_core::PCSTR, udstuavslot: u32, ucount: u32) -> windows_core::Result<()>;
}
impl ID3D11ModuleInstance_Vtbl {
    pub const fn new<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::Release(this)
            }
        }
        unsafe extern "system" fn BindConstantBuffer<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindConstantBuffer(this, core::mem::transmute_copy(&usrcslot), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&cbdstoffset)).into()
            }
        }
        unsafe extern "system" fn BindConstantBufferByName<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCSTR, udstslot: u32, cbdstoffset: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindConstantBufferByName(this, core::mem::transmute(&pname), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&cbdstoffset)).into()
            }
        }
        unsafe extern "system" fn BindResource<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindResource(this, core::mem::transmute_copy(&usrcslot), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
            }
        }
        unsafe extern "system" fn BindResourceByName<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindResourceByName(this, core::mem::transmute(&pname), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
            }
        }
        unsafe extern "system" fn BindSampler<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindSampler(this, core::mem::transmute_copy(&usrcslot), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
            }
        }
        unsafe extern "system" fn BindSamplerByName<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindSamplerByName(this, core::mem::transmute(&pname), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
            }
        }
        unsafe extern "system" fn BindUnorderedAccessView<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindUnorderedAccessView(this, core::mem::transmute_copy(&usrcslot), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
            }
        }
        unsafe extern "system" fn BindUnorderedAccessViewByName<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindUnorderedAccessViewByName(this, core::mem::transmute(&pname), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
            }
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessView<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindResourceAsUnorderedAccessView(this, core::mem::transmute_copy(&usrcsrvslot), core::mem::transmute_copy(&udstuavslot), core::mem::transmute_copy(&ucount)).into()
            }
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessViewByName<Identity: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrvname: windows_core::PCSTR, udstuavslot: u32, ucount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ModuleInstance_Impl::BindResourceAsUnorderedAccessViewByName(this, core::mem::transmute(&psrvname), core::mem::transmute_copy(&udstuavslot), core::mem::transmute_copy(&ucount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            BindConstantBuffer: BindConstantBuffer::<Identity, OFFSET>,
            BindConstantBufferByName: BindConstantBufferByName::<Identity, OFFSET>,
            BindResource: BindResource::<Identity, OFFSET>,
            BindResourceByName: BindResourceByName::<Identity, OFFSET>,
            BindSampler: BindSampler::<Identity, OFFSET>,
            BindSamplerByName: BindSamplerByName::<Identity, OFFSET>,
            BindUnorderedAccessView: BindUnorderedAccessView::<Identity, OFFSET>,
            BindUnorderedAccessViewByName: BindUnorderedAccessViewByName::<Identity, OFFSET>,
            BindResourceAsUnorderedAccessView: BindResourceAsUnorderedAccessView::<Identity, OFFSET>,
            BindResourceAsUnorderedAccessViewByName: BindResourceAsUnorderedAccessViewByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ModuleInstance as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11ModuleInstance {}
windows_core::imp::define_interface!(ID3D11ShaderReflection, ID3D11ShaderReflection_Vtbl, 0x8d536ca1_0cca_4956_a837_786963755584);
windows_core::imp::interface_hierarchy!(ID3D11ShaderReflection, windows_core::IUnknown);
impl ID3D11ShaderReflection {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPatchConstantParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc as _) }
    }
    pub unsafe fn GetMovInstructionCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetMovInstructionCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMovcInstructionCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetMovcInstructionCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetConversionInstructionCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetConversionInstructionCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBitwiseInstructionCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetBitwiseInstructionCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetGSInputPrimitive(&self) -> super::d3dcommon::D3D_PRIMITIVE {
        unsafe { (windows_core::Interface::vtable(self).GetGSInputPrimitive)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsSampleFrequencyShader(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsSampleFrequencyShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNumInterfaceSlots(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNumInterfaceSlots)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetMinFeatureLevel(&self) -> windows_core::Result<super::d3dcommon::D3D_FEATURE_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinFeatureLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetThreadGroupSize(&self, psizex: Option<*mut u32>, psizey: Option<*mut u32>, psizez: Option<*mut u32>) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetThreadGroupSize)(windows_core::Interface::as_raw(self), psizex.unwrap_or(core::mem::zeroed()) as _, psizey.unwrap_or(core::mem::zeroed()) as _, psizez.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetRequiresFlags(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetRequiresFlags)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ShaderReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetInputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetOutputParameterDesc: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetPatchConstantParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetPatchConstantParameterDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetResourceBindingDescByName: usize,
    pub GetMovInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetMovcInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetConversionInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetBitwiseInstructionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetGSInputPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::d3dcommon::D3D_PRIMITIVE,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetGSInputPrimitive: usize,
    pub IsSampleFrequencyShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetNumInterfaceSlots: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetMinFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d3dcommon::D3D_FEATURE_LEVEL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetMinFeatureLevel: usize,
    pub GetThreadGroupSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> u32,
    pub GetRequiresFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D11ShaderReflection_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetMovInstructionCount(&self) -> u32;
    fn GetMovcInstructionCount(&self) -> u32;
    fn GetConversionInstructionCount(&self) -> u32;
    fn GetBitwiseInstructionCount(&self) -> u32;
    fn GetGSInputPrimitive(&self) -> super::d3dcommon::D3D_PRIMITIVE;
    fn IsSampleFrequencyShader(&self) -> windows_core::BOOL;
    fn GetNumInterfaceSlots(&self) -> u32;
    fn GetMinFeatureLevel(&self) -> windows_core::Result<super::d3dcommon::D3D_FEATURE_LEVEL>;
    fn GetThreadGroupSize(&self, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32;
    fn GetRequiresFlags(&self) -> u64;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11ShaderReflection_Vtbl {
    pub const fn new<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetPatchConstantParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetMovInstructionCount(this)
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetMovcInstructionCount(this)
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetConversionInstructionCount(this)
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetBitwiseInstructionCount(this)
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3dcommon::D3D_PRIMITIVE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetGSInputPrimitive(this)
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::IsSampleFrequencyShader(this)
            }
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetNumInterfaceSlots(this)
            }
        }
        unsafe extern "system" fn GetMinFeatureLevel<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut super::d3dcommon::D3D_FEATURE_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11ShaderReflection_Impl::GetMinFeatureLevel(this) {
                    Ok(ok__) => {
                        plevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetThreadGroupSize(this, core::mem::transmute_copy(&psizex), core::mem::transmute_copy(&psizey), core::mem::transmute_copy(&psizez))
            }
        }
        unsafe extern "system" fn GetRequiresFlags<Identity: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderReflection_Impl::GetRequiresFlags(this)
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
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Identity, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Identity, OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Identity, OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Identity, OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ShaderReflection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl windows_core::RuntimeName for ID3D11ShaderReflection {}
windows_core::imp::define_interface!(ID3D11ShaderReflectionConstantBuffer, ID3D11ShaderReflectionConstantBuffer_Vtbl);
impl ID3D11ShaderReflectionConstantBuffer {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D11ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_BUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D11ShaderReflectionConstantBuffer_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Identity: ID3D11ShaderReflectionConstantBuffer_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionConstantBuffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D11ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionConstantBuffer_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionConstantBuffer_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        Self { GetDesc: GetDesc::<Identity>, GetVariableByIndex: GetVariableByIndex::<Identity>, GetVariableByName: GetVariableByName::<Identity> }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
struct ID3D11ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D11ShaderReflectionConstantBuffer_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D11ShaderReflectionConstantBuffer_Impl> ID3D11ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionConstantBuffer_Vtbl = ID3D11ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D11ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D11ShaderReflectionType, ID3D11ShaderReflectionType_Vtbl);
impl ID3D11ShaderReflectionType {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> windows_core::HRESULT {
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
    pub unsafe fn IsEqual<P0>(&self, ptype: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), ptype.param().abi()) }
    }
    pub unsafe fn GetSubType(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).GetSubType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBaseClass(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).GetBaseClass)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNumInterfaces(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNumInterfaces)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetInterfaceByIndex(&self, uindex: u32) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).GetInterfaceByIndex)(windows_core::Interface::as_raw(self), uindex) }
    }
    pub unsafe fn IsOfType<P0>(&self, ptype: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsOfType)(windows_core::Interface::as_raw(self), ptype.param().abi()) }
    }
    pub unsafe fn ImplementsInterface<P0>(&self, pbase: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).ImplementsInterface)(windows_core::Interface::as_raw(self), pbase.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D11ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType>,
    pub GetBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType>,
    pub GetNumInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetInterfaceByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D11ShaderReflectionType>,
    pub IsOfType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImplementsInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D11ShaderReflectionType_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR;
    fn IsEqual(&self, ptype: windows_core::Ref<ID3D11ShaderReflectionType>) -> windows_core::Result<()>;
    fn GetSubType(&self) -> Option<ID3D11ShaderReflectionType>;
    fn GetBaseClass(&self) -> Option<ID3D11ShaderReflectionType>;
    fn GetNumInterfaces(&self) -> u32;
    fn GetInterfaceByIndex(&self, uindex: u32) -> Option<ID3D11ShaderReflectionType>;
    fn IsOfType(&self, ptype: windows_core::Ref<ID3D11ShaderReflectionType>) -> windows_core::Result<()>;
    fn ImplementsInterface(&self, pbase: windows_core::Ref<ID3D11ShaderReflectionType>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11ShaderReflectionType_Vtbl {
    pub const fn new<Identity: ID3D11ShaderReflectionType_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D11ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberTypeName<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::GetMemberTypeName(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::IsEqual(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn GetSubType<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::GetSubType(this)
            }
        }
        unsafe extern "system" fn GetBaseClass<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::GetBaseClass(this)
            }
        }
        unsafe extern "system" fn GetNumInterfaces<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::GetNumInterfaces(this)
            }
        }
        unsafe extern "system" fn GetInterfaceByIndex<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, uindex: u32) -> Option<ID3D11ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::GetInterfaceByIndex(this, core::mem::transmute_copy(&uindex))
            }
        }
        unsafe extern "system" fn IsOfType<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::IsOfType(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn ImplementsInterface<Identity: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pbase: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionType_Impl::ImplementsInterface(this, core::mem::transmute_copy(&pbase)).into()
            }
        }
        Self {
            GetDesc: GetDesc::<Identity>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity>,
            GetMemberTypeName: GetMemberTypeName::<Identity>,
            IsEqual: IsEqual::<Identity>,
            GetSubType: GetSubType::<Identity>,
            GetBaseClass: GetBaseClass::<Identity>,
            GetNumInterfaces: GetNumInterfaces::<Identity>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Identity>,
            IsOfType: IsOfType::<Identity>,
            ImplementsInterface: ImplementsInterface::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
struct ID3D11ShaderReflectionType_ImplVtbl<T: ID3D11ShaderReflectionType_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D11ShaderReflectionType_Impl> ID3D11ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionType_Vtbl = ID3D11ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D11ShaderReflectionType {
    pub fn new<'a, T: ID3D11ShaderReflectionType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D11ShaderReflectionVariable, ID3D11ShaderReflectionVariable_Vtbl);
impl ID3D11ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D11ShaderReflectionType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBuffer(&self) -> Option<ID3D11ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetInterfaceSlot)(windows_core::Interface::as_raw(self), uarrayindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType>,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionConstantBuffer>,
    pub GetInterfaceSlot: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
}
pub trait ID3D11ShaderReflectionVariable_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetType(&self) -> Option<ID3D11ShaderReflectionType>;
    fn GetBuffer(&self) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32;
}
impl ID3D11ShaderReflectionVariable_Vtbl {
    pub const fn new<Identity: ID3D11ShaderReflectionVariable_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D11ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: ID3D11ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionVariable_Impl::GetBuffer(this)
            }
        }
        unsafe extern "system" fn GetInterfaceSlot<Identity: ID3D11ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, uarrayindex: u32) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D11ShaderReflectionVariable_Impl::GetInterfaceSlot(this, core::mem::transmute_copy(&uarrayindex))
            }
        }
        Self { GetDesc: GetDesc::<Identity>, GetType: GetType::<Identity>, GetBuffer: GetBuffer::<Identity>, GetInterfaceSlot: GetInterfaceSlot::<Identity> }
    }
}
struct ID3D11ShaderReflectionVariable_ImplVtbl<T: ID3D11ShaderReflectionVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D11ShaderReflectionVariable_Impl> ID3D11ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionVariable_Vtbl = ID3D11ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D11ShaderReflectionVariable {
    pub fn new<'a, T: ID3D11ShaderReflectionVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
