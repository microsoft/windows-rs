#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D12_CBUFFER_TYPE(pub super::d3dcommon::D3D_CBUFFER_TYPE);
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FUNCTION_DESC {
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
pub struct D3D12_LIBRARY_DESC {
    pub Creator: windows_core::PCSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_PARAMETER_DESC {
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
pub struct D3D12_RESOURCE_RETURN_TYPE(pub super::d3dcommon::D3D_RESOURCE_RETURN_TYPE);
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_SHADER_BUFFER_DESC {
    pub Name: windows_core::PCSTR,
    pub Type: super::d3dcommon::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_SHADER_DESC {
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
pub struct D3D12_SHADER_INPUT_BIND_DESC {
    pub Name: windows_core::PCSTR,
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
#[repr(C)]
#[cfg(feature = "Win32_d3dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_SHADER_TYPE_DESC {
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
pub struct D3D12_SHADER_VARIABLE_DESC {
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_SIGNATURE_PARAMETER_DESC {
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
pub struct D3D12_TESSELLATOR_DOMAIN(pub super::d3dcommon::D3D_TESSELLATOR_DOMAIN);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D12_TESSELLATOR_OUTPUT_PRIMITIVE(pub super::d3dcommon::D3D_TESSELLATOR_OUTPUT_PRIMITIVE);
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D12_TESSELLATOR_PARTITIONING(pub super::d3dcommon::D3D_TESSELLATOR_PARTITIONING);
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
windows_core::imp::define_interface!(ID3D12FunctionParameterReflection, ID3D12FunctionParameterReflection_Vtbl);
impl ID3D12FunctionParameterReflection {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12FunctionParameterReflection_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D12FunctionParameterReflection_Impl {
    fn GetDesc(&self, pdesc: *mut D3D12_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D12FunctionParameterReflection_Vtbl {
    pub const fn new<Identity: ID3D12FunctionParameterReflection_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12FunctionParameterReflection_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12FunctionParameterReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self { GetDesc: GetDesc::<Identity> }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
struct ID3D12FunctionParameterReflection_ImplVtbl<T: ID3D12FunctionParameterReflection_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D12FunctionParameterReflection_Impl> ID3D12FunctionParameterReflection_ImplVtbl<T> {
    const VTABLE: ID3D12FunctionParameterReflection_Vtbl = ID3D12FunctionParameterReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D12FunctionParameterReflection {
    pub fn new<'a, T: ID3D12FunctionParameterReflection_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12FunctionParameterReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D12FunctionReflection, ID3D12FunctionReflection_Vtbl);
impl ID3D12FunctionReflection {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_FUNCTION_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, bufferindex: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), bufferindex) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc as _) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDescByName)(windows_core::Interface::as_raw(self), name.param().abi(), pdesc as _) }
    }
    pub unsafe fn GetFunctionParameter(&self, parameterindex: i32) -> Option<ID3D12FunctionParameterReflection> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionParameter)(windows_core::Interface::as_raw(self), parameterindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12FunctionReflection_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_FUNCTION_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetResourceBindingDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetResourceBindingDescByName: usize,
    pub GetFunctionParameter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> Option<ID3D12FunctionParameterReflection>,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D12FunctionReflection_Impl {
    fn GetDesc(&self, pdesc: *mut D3D12_FUNCTION_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, bufferindex: u32) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetFunctionParameter(&self, parameterindex: i32) -> Option<ID3D12FunctionParameterReflection>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D12FunctionReflection_Vtbl {
    pub const fn new<Identity: ID3D12FunctionReflection_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_FUNCTION_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12FunctionReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, bufferindex: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12FunctionReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&bufferindex))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12FunctionReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12FunctionReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12FunctionReflection_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12FunctionReflection_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetFunctionParameter<Identity: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, parameterindex: i32) -> Option<ID3D12FunctionParameterReflection> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12FunctionReflection_Impl::GetFunctionParameter(this, core::mem::transmute_copy(&parameterindex))
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
struct ID3D12FunctionReflection_ImplVtbl<T: ID3D12FunctionReflection_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D12FunctionReflection_Impl> ID3D12FunctionReflection_ImplVtbl<T> {
    const VTABLE: ID3D12FunctionReflection_Vtbl = ID3D12FunctionReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D12FunctionReflection {
    pub fn new<'a, T: ID3D12FunctionReflection_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12FunctionReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D12LibraryReflection, ID3D12LibraryReflection_Vtbl, 0x8e349d19_54db_4a56_9dc9_119d87bdb804);
windows_core::imp::interface_hierarchy!(ID3D12LibraryReflection, windows_core::IUnknown);
impl ID3D12LibraryReflection {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self) -> windows_core::Result<D3D12_LIBRARY_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFunctionByIndex(&self, functionindex: i32) -> Option<ID3D12FunctionReflection> {
        unsafe { (windows_core::Interface::vtable(self).GetFunctionByIndex)(windows_core::Interface::as_raw(self), functionindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12LibraryReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_LIBRARY_DESC) -> windows_core::HRESULT,
    pub GetFunctionByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> Option<ID3D12FunctionReflection>,
}
pub trait ID3D12LibraryReflection_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDesc(&self) -> windows_core::Result<D3D12_LIBRARY_DESC>;
    fn GetFunctionByIndex(&self, functionindex: i32) -> Option<ID3D12FunctionReflection>;
}
impl ID3D12LibraryReflection_Vtbl {
    pub const fn new<Identity: ID3D12LibraryReflection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12LibraryReflection_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12LibraryReflection_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12LibraryReflection_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_LIBRARY_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D12LibraryReflection_Impl::GetDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Identity: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionindex: i32) -> Option<ID3D12FunctionReflection> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12LibraryReflection_Impl::GetFunctionByIndex(this, core::mem::transmute_copy(&functionindex))
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
        iid == &<ID3D12LibraryReflection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D12LibraryReflection {}
windows_core::imp::define_interface!(ID3D12ShaderReflection, ID3D12ShaderReflection_Vtbl, 0x5a58797d_a72c_478d_8ba2_efc6b0efe88e);
windows_core::imp::interface_hierarchy!(ID3D12ShaderReflection, windows_core::IUnknown);
impl ID3D12ShaderReflection {
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
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResourceBindingDesc)(windows_core::Interface::as_raw(self), resourceindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPatchConstantParameterDesc)(windows_core::Interface::as_raw(self), parameterindex, pdesc as _) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetResourceBindingDescByName<P0>(&self, name: P0, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT
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
pub struct ID3D12ShaderReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetResourceBindingDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetResourceBindingDesc: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetInputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetInputParameterDesc: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetOutputParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetOutputParameterDesc: usize,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetPatchConstantParameterDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetPatchConstantParameterDesc: usize,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>,
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetResourceBindingDescByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT,
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
pub trait ID3D12ShaderReflection_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
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
impl ID3D12ShaderReflection_Vtbl {
    pub const fn new<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetPatchConstantParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetMovInstructionCount(this)
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetMovcInstructionCount(this)
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetConversionInstructionCount(this)
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetBitwiseInstructionCount(this)
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::d3dcommon::D3D_PRIMITIVE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetGSInputPrimitive(this)
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::IsSampleFrequencyShader(this)
            }
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetNumInterfaceSlots(this)
            }
        }
        unsafe extern "system" fn GetMinFeatureLevel<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut super::d3dcommon::D3D_FEATURE_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D12ShaderReflection_Impl::GetMinFeatureLevel(this) {
                    Ok(ok__) => {
                        plevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetThreadGroupSize(this, core::mem::transmute_copy(&psizex), core::mem::transmute_copy(&psizey), core::mem::transmute_copy(&psizez))
            }
        }
        unsafe extern "system" fn GetRequiresFlags<Identity: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12ShaderReflection_Impl::GetRequiresFlags(this)
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
        iid == &<ID3D12ShaderReflection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3dcommon")]
impl windows_core::RuntimeName for ID3D12ShaderReflection {}
windows_core::imp::define_interface!(ID3D12ShaderReflectionConstantBuffer, ID3D12ShaderReflectionConstantBuffer_Vtbl);
impl ID3D12ShaderReflectionConstantBuffer {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D12ShaderReflectionVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ShaderReflectionConstantBuffer_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_BUFFER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D12ShaderReflectionConstantBuffer_Impl {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D12ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Identity: ID3D12ShaderReflectionConstantBuffer_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionConstantBuffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D12ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionConstantBuffer_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionConstantBuffer_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        Self { GetDesc: GetDesc::<Identity>, GetVariableByIndex: GetVariableByIndex::<Identity>, GetVariableByName: GetVariableByName::<Identity> }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
struct ID3D12ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D12ShaderReflectionConstantBuffer_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D12ShaderReflectionConstantBuffer_Impl> ID3D12ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionConstantBuffer_Vtbl = ID3D12ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D12ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D12ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D12ShaderReflectionType, ID3D12ShaderReflectionType_Vtbl);
impl ID3D12ShaderReflectionType {
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> windows_core::HRESULT {
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
pub struct ID3D12ShaderReflectionType_Vtbl {
    #[cfg(feature = "Win32_d3dcommon")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D12ShaderReflectionType>,
    pub GetMemberTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType>,
    pub GetBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType>,
    pub GetNumInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetInterfaceByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D12ShaderReflectionType>,
    pub IsOfType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImplementsInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3dcommon")]
pub trait ID3D12ShaderReflectionType_Impl {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR;
    fn IsEqual(&self, ptype: windows_core::Ref<ID3D12ShaderReflectionType>) -> windows_core::Result<()>;
    fn GetSubType(&self) -> Option<ID3D12ShaderReflectionType>;
    fn GetBaseClass(&self) -> Option<ID3D12ShaderReflectionType>;
    fn GetNumInterfaces(&self) -> u32;
    fn GetInterfaceByIndex(&self, uindex: u32) -> Option<ID3D12ShaderReflectionType>;
    fn IsOfType(&self, ptype: windows_core::Ref<ID3D12ShaderReflectionType>) -> windows_core::Result<()>;
    fn ImplementsInterface(&self, pbase: windows_core::Ref<ID3D12ShaderReflectionType>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D12ShaderReflectionType_Vtbl {
    pub const fn new<Identity: ID3D12ShaderReflectionType_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D12ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberTypeName<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::GetMemberTypeName(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::IsEqual(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn GetSubType<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::GetSubType(this)
            }
        }
        unsafe extern "system" fn GetBaseClass<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::GetBaseClass(this)
            }
        }
        unsafe extern "system" fn GetNumInterfaces<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::GetNumInterfaces(this)
            }
        }
        unsafe extern "system" fn GetInterfaceByIndex<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, uindex: u32) -> Option<ID3D12ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::GetInterfaceByIndex(this, core::mem::transmute_copy(&uindex))
            }
        }
        unsafe extern "system" fn IsOfType<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::IsOfType(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn ImplementsInterface<Identity: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pbase: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionType_Impl::ImplementsInterface(this, core::mem::transmute_copy(&pbase)).into()
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
struct ID3D12ShaderReflectionType_ImplVtbl<T: ID3D12ShaderReflectionType_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3dcommon")]
impl<T: ID3D12ShaderReflectionType_Impl> ID3D12ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionType_Vtbl = ID3D12ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3dcommon")]
impl ID3D12ShaderReflectionType {
    pub fn new<'a, T: ID3D12ShaderReflectionType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D12ShaderReflectionVariable, ID3D12ShaderReflectionVariable_Vtbl);
impl ID3D12ShaderReflectionVariable {
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D12ShaderReflectionType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBuffer(&self) -> Option<ID3D12ShaderReflectionConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetInterfaceSlot)(windows_core::Interface::as_raw(self), uarrayindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12ShaderReflectionVariable_Vtbl {
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_SHADER_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType>,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionConstantBuffer>,
    pub GetInterfaceSlot: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
}
pub trait ID3D12ShaderReflectionVariable_Impl {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetType(&self) -> Option<ID3D12ShaderReflectionType>;
    fn GetBuffer(&self) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32;
}
impl ID3D12ShaderReflectionVariable_Vtbl {
    pub const fn new<Identity: ID3D12ShaderReflectionVariable_Impl>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D12ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: ID3D12ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionVariable_Impl::GetBuffer(this)
            }
        }
        unsafe extern "system" fn GetInterfaceSlot<Identity: ID3D12ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, uarrayindex: u32) -> u32 {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D12ShaderReflectionVariable_Impl::GetInterfaceSlot(this, core::mem::transmute_copy(&uarrayindex))
            }
        }
        Self { GetDesc: GetDesc::<Identity>, GetType: GetType::<Identity>, GetBuffer: GetBuffer::<Identity>, GetInterfaceSlot: GetInterfaceSlot::<Identity> }
    }
}
struct ID3D12ShaderReflectionVariable_ImplVtbl<T: ID3D12ShaderReflectionVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D12ShaderReflectionVariable_Impl> ID3D12ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionVariable_Vtbl = ID3D12ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D12ShaderReflectionVariable {
    pub fn new<'a, T: ID3D12ShaderReflectionVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
