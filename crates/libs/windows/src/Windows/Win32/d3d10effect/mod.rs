#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
#[inline]
pub unsafe fn D3D10CompileEffectFromMemory<P2, P4>(pdata: *const core::ffi::c_void, datalength: usize, psrcfilename: P2, pdefines: Option<*const super::d3d10shader::D3D10_SHADER_MACRO>, pinclude: P4, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut Option<super::d3dcommon::ID3D10Blob>, pperrors: Option<*mut Option<super::d3dcommon::ID3D10Blob>>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::d3dcommon::ID3DInclude>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CompileEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, psrcfilename : windows_core::PCSTR, pdefines : *const super::d3d10shader::D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, hlslflags : u32, fxflags : u32, ppcompiledeffect : *mut *mut core::ffi::c_void, pperrors : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CompileEffectFromMemory(pdata, datalength, psrcfilename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), hlslflags, fxflags, core::mem::transmute(ppcompiledeffect), pperrors.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_d3d10")]
#[inline]
pub unsafe fn D3D10CreateEffectFromMemory<P3, P4>(pdata: *const core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: P3, peffectpool: P4) -> windows_core::Result<ID3D10Effect>
where
    P3: windows_core::Param<super::d3d10::ID3D10Device>,
    P4: windows_core::Param<ID3D10EffectPool>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : *mut core::ffi::c_void, peffectpool : *mut core::ffi::c_void, ppeffect : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10CreateEffectFromMemory(pdata, datalength, fxflags, pdevice.param().abi(), peffectpool.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_d3d10")]
#[inline]
pub unsafe fn D3D10CreateEffectPoolFromMemory<P3>(pdata: *const core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: P3) -> windows_core::Result<ID3D10EffectPool>
where
    P3: windows_core::Param<super::d3d10::ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateEffectPoolFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : *mut core::ffi::c_void, ppeffectpool : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10CreateEffectPoolFromMemory(pdata, datalength, fxflags, pdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_d3d10")]
#[inline]
pub unsafe fn D3D10CreateStateBlock<P0>(pdevice: P0, pstateblockmask: *const D3D10_STATE_BLOCK_MASK) -> windows_core::Result<ID3D10StateBlock>
where
    P0: windows_core::Param<super::d3d10::ID3D10Device>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateStateBlock(pdevice : *mut core::ffi::c_void, pstateblockmask : *const D3D10_STATE_BLOCK_MASK, ppstateblock : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10CreateStateBlock(pdevice.param().abi(), pstateblockmask, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10DisassembleEffect<P0>(peffect: P0, enablecolorcode: bool) -> windows_core::Result<super::d3dcommon::ID3D10Blob>
where
    P0: windows_core::Param<ID3D10Effect>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10DisassembleEffect(peffect : *mut core::ffi::c_void, enablecolorcode : windows_core::BOOL, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10DisassembleEffect(peffect.param().abi(), enablecolorcode.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDifference(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskDifference(pa, pb, presult as _) }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskDisableAll(pmask as _) }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> windows_core::HRESULT {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskDisableCapture(pmask as _, statetype, rangestart, rangelength) }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskEnableAll(pmask as _) }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> windows_core::HRESULT {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskEnableCapture(pmask as _, statetype, rangestart, rangelength) }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> windows_core::BOOL {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskGetSetting(pmask : *const D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, entry : u32) -> windows_core::BOOL);
    unsafe { D3D10StateBlockMaskGetSetting(pmask, statetype, entry) }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskIntersect(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskIntersect(pa, pb, presult as _) }
}
#[inline]
pub unsafe fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
    windows_core::link!("d3d10.dll" "system" fn D3D10StateBlockMaskUnion(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT);
    unsafe { D3D10StateBlockMaskUnion(pa, pb, presult as _) }
}
pub type D3D10_DEVICE_STATE_TYPES = i32;
pub const D3D10_DST_GS: D3D10_DEVICE_STATE_TYPES = 9;
pub const D3D10_DST_GS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 12;
pub const D3D10_DST_GS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 10;
pub const D3D10_DST_GS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 11;
pub const D3D10_DST_IA_INDEX_BUFFER: D3D10_DEVICE_STATE_TYPES = 18;
pub const D3D10_DST_IA_INPUT_LAYOUT: D3D10_DEVICE_STATE_TYPES = 19;
pub const D3D10_DST_IA_PRIMITIVE_TOPOLOGY: D3D10_DEVICE_STATE_TYPES = 20;
pub const D3D10_DST_IA_VERTEX_BUFFERS: D3D10_DEVICE_STATE_TYPES = 17;
pub const D3D10_DST_OM_BLEND_STATE: D3D10_DEVICE_STATE_TYPES = 4;
pub const D3D10_DST_OM_DEPTH_STENCIL_STATE: D3D10_DEVICE_STATE_TYPES = 3;
pub const D3D10_DST_OM_RENDER_TARGETS: D3D10_DEVICE_STATE_TYPES = 2;
pub const D3D10_DST_PREDICATION: D3D10_DEVICE_STATE_TYPES = 24;
pub const D3D10_DST_PS: D3D10_DEVICE_STATE_TYPES = 13;
pub const D3D10_DST_PS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 16;
pub const D3D10_DST_PS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 14;
pub const D3D10_DST_PS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 15;
pub const D3D10_DST_RS_RASTERIZER_STATE: D3D10_DEVICE_STATE_TYPES = 23;
pub const D3D10_DST_RS_SCISSOR_RECTS: D3D10_DEVICE_STATE_TYPES = 22;
pub const D3D10_DST_RS_VIEWPORTS: D3D10_DEVICE_STATE_TYPES = 21;
pub const D3D10_DST_SO_BUFFERS: D3D10_DEVICE_STATE_TYPES = 1;
pub const D3D10_DST_VS: D3D10_DEVICE_STATE_TYPES = 5;
pub const D3D10_DST_VS_CONSTANT_BUFFERS: D3D10_DEVICE_STATE_TYPES = 8;
pub const D3D10_DST_VS_SAMPLERS: D3D10_DEVICE_STATE_TYPES = 6;
pub const D3D10_DST_VS_SHADER_RESOURCES: D3D10_DEVICE_STATE_TYPES = 7;
pub const D3D10_EFFECT_COMPILE_ALLOW_SLOW_OPS: u32 = 2;
pub const D3D10_EFFECT_COMPILE_CHILD_EFFECT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_EFFECT_DESC {
    pub IsChildEffect: windows_core::BOOL,
    pub ConstantBuffers: u32,
    pub SharedConstantBuffers: u32,
    pub GlobalVariables: u32,
    pub SharedGlobalVariables: u32,
    pub Techniques: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_EFFECT_SHADER_DESC {
    pub pInputSignature: *const u8,
    pub IsInline: windows_core::BOOL,
    pub pBytecode: *const u8,
    pub BytecodeLength: u32,
    pub SODecl: windows_core::PCSTR,
    pub NumInputSignatureEntries: u32,
    pub NumOutputSignatureEntries: u32,
}
impl Default for D3D10_EFFECT_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_SINGLE_THREADED: u32 = 8;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_EFFECT_TYPE_DESC {
    pub TypeName: windows_core::PCSTR,
    pub Class: super::d3d10shader::D3D10_SHADER_VARIABLE_CLASS,
    pub Type: super::d3d10shader::D3D10_SHADER_VARIABLE_TYPE,
    pub Elements: u32,
    pub Members: u32,
    pub Rows: u32,
    pub Columns: u32,
    pub PackedSize: u32,
    pub UnpackedSize: u32,
    pub Stride: u32,
}
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_EFFECT_VARIABLE_DESC {
    pub Name: windows_core::PCSTR,
    pub Semantic: windows_core::PCSTR,
    pub Flags: u32,
    pub Annotations: u32,
    pub BufferOffset: u32,
    pub ExplicitBindPoint: u32,
}
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4;
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_PASS_DESC {
    pub Name: windows_core::PCSTR,
    pub Annotations: u32,
    pub pIAInputSignature: *mut u8,
    pub IAInputSignatureSize: usize,
    pub StencilRef: u32,
    pub SampleMask: u32,
    pub BlendFactor: [f32; 4],
}
impl Default for D3D10_PASS_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D10_PASS_SHADER_DESC {
    pub pShaderVariable: core::mem::ManuallyDrop<Option<ID3D10EffectShaderVariable>>,
    pub ShaderIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_STATE_BLOCK_MASK {
    pub VS: u8,
    pub VSSamplers: [u8; 2],
    pub VSShaderResources: [u8; 16],
    pub VSConstantBuffers: [u8; 2],
    pub GS: u8,
    pub GSSamplers: [u8; 2],
    pub GSShaderResources: [u8; 16],
    pub GSConstantBuffers: [u8; 2],
    pub PS: u8,
    pub PSSamplers: [u8; 2],
    pub PSShaderResources: [u8; 16],
    pub PSConstantBuffers: [u8; 2],
    pub IAVertexBuffers: [u8; 2],
    pub IAIndexBuffer: u8,
    pub IAInputLayout: u8,
    pub IAPrimitiveTopology: u8,
    pub OMRenderTargets: u8,
    pub OMDepthStencilState: u8,
    pub OMBlendState: u8,
    pub RSViewports: u8,
    pub RSScissorRects: u8,
    pub RSRasterizerState: u8,
    pub SOBuffers: u8,
    pub Predication: u8,
}
impl Default for D3D10_STATE_BLOCK_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TECHNIQUE_DESC {
    pub Name: windows_core::PCSTR,
    pub Passes: u32,
    pub Annotations: u32,
}
windows_core::imp::define_interface!(ID3D10Effect, ID3D10Effect_Vtbl, 0x51b0ca8b_ec0b_4519_870d_8ee1cb5017c7);
windows_core::imp::interface_hierarchy!(ID3D10Effect, windows_core::IUnknown);
impl ID3D10Effect {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsPool(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsPool)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetDevice(&self) -> windows_core::Result<super::d3d10::ID3D10Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetConstantBufferByName<P0>(&self, name: P0) -> Option<ID3D10EffectConstantBuffer>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConstantBufferByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetVariableByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetVariableBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVariableBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetTechniqueByIndex(&self, index: u32) -> Option<ID3D10EffectTechnique> {
        unsafe { (windows_core::Interface::vtable(self).GetTechniqueByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetTechniqueByName<P0>(&self, name: P0) -> Option<ID3D10EffectTechnique>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTechniqueByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn Optimize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Optimize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsOptimized(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsOptimized)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Effect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub IsPool: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(feature = "Win32_d3d10")]
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetDevice: usize,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_DESC) -> windows_core::HRESULT,
    pub GetConstantBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectConstantBuffer>,
    pub GetConstantBufferByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectConstantBuffer>,
    pub GetVariableByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetVariableByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetVariableBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetTechniqueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectTechnique>,
    pub GetTechniqueByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectTechnique>,
    pub Optimize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOptimized: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10Effect_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn IsValid(&self) -> windows_core::BOOL;
    fn IsPool(&self) -> windows_core::BOOL;
    fn GetDevice(&self) -> windows_core::Result<super::d3d10::ID3D10Device>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10EffectConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectConstantBuffer>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetVariableBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetTechniqueByIndex(&self, index: u32) -> Option<ID3D10EffectTechnique>;
    fn GetTechniqueByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectTechnique>;
    fn Optimize(&self) -> windows_core::Result<()>;
    fn IsOptimized(&self) -> windows_core::BOOL;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10Effect_Vtbl {
    pub const fn new<Identity: ID3D10Effect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::Release(this)
            }
        }
        unsafe extern "system" fn IsValid<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn IsPool<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::IsPool(this)
            }
        }
        unsafe extern "system" fn GetDevice<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10Effect_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetVariableByName<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetVariableByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetVariableBySemantic<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetVariableBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetTechniqueByIndex<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectTechnique> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetTechniqueByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetTechniqueByName<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectTechnique> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::GetTechniqueByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn Optimize<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::Optimize(this).into()
            }
        }
        unsafe extern "system" fn IsOptimized<Identity: ID3D10Effect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Effect_Impl::IsOptimized(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            IsValid: IsValid::<Identity, OFFSET>,
            IsPool: IsPool::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Identity, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, OFFSET>,
            GetVariableBySemantic: GetVariableBySemantic::<Identity, OFFSET>,
            GetTechniqueByIndex: GetTechniqueByIndex::<Identity, OFFSET>,
            GetTechniqueByName: GetTechniqueByName::<Identity, OFFSET>,
            Optimize: Optimize::<Identity, OFFSET>,
            IsOptimized: IsOptimized::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Effect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d10")]
impl windows_core::RuntimeName for ID3D10Effect {}
windows_core::imp::define_interface!(ID3D10EffectBlendVariable, ID3D10EffectBlendVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectBlendVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectBlendVariable, ID3D10EffectVariable);
impl ID3D10EffectBlendVariable {
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetBlendState(&self, index: u32) -> windows_core::Result<super::d3d10::ID3D10BlendState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlendState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetBackingStore(&self, index: u32, pblenddesc: *mut super::d3d10::D3D10_BLEND_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, pblenddesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectBlendVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub GetBlendState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetBlendState: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d10::D3D10_BLEND_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetBackingStore: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10EffectBlendVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetBlendState(&self, index: u32) -> windows_core::Result<super::d3d10::ID3D10BlendState>;
    fn GetBackingStore(&self, index: u32, pblenddesc: *mut super::d3d10::D3D10_BLEND_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectBlendVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectBlendVariable_Impl>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetBlendState<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectBlendVariable_Impl::GetBlendState(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppblendstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pblenddesc: *mut super::d3d10::D3D10_BLEND_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectBlendVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pblenddesc)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            GetBlendState: GetBlendState::<Identity>,
            GetBackingStore: GetBackingStore::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
struct ID3D10EffectBlendVariable_ImplVtbl<T: ID3D10EffectBlendVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3d10")]
impl<T: ID3D10EffectBlendVariable_Impl> ID3D10EffectBlendVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectBlendVariable_Vtbl = ID3D10EffectBlendVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectBlendVariable {
    pub fn new<'a, T: ID3D10EffectBlendVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectBlendVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectConstantBuffer, ID3D10EffectConstantBuffer_Vtbl);
impl core::ops::Deref for ID3D10EffectConstantBuffer {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectConstantBuffer, ID3D10EffectVariable);
impl ID3D10EffectConstantBuffer {
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn SetConstantBuffer<P0>(&self, pconstantbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d10::ID3D10Buffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetConstantBuffer)(windows_core::Interface::as_raw(self), pconstantbuffer.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetConstantBuffer(&self) -> windows_core::Result<super::d3d10::ID3D10Buffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConstantBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn SetTextureBuffer<P0>(&self, ptexturebuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d10::ID3D10ShaderResourceView>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTextureBuffer)(windows_core::Interface::as_raw(self), ptexturebuffer.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetTextureBuffer(&self) -> windows_core::Result<super::d3d10::ID3D10ShaderResourceView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextureBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectConstantBuffer_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub SetConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    SetConstantBuffer: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetConstantBuffer: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub SetTextureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    SetTextureBuffer: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetTextureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetTextureBuffer: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10EffectConstantBuffer_Impl: ID3D10EffectVariable_Impl {
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn SetConstantBuffer(&self, pconstantbuffer: windows_core::Ref<super::d3d10::ID3D10Buffer>) -> windows_core::Result<()>;
    fn GetConstantBuffer(&self) -> windows_core::Result<super::d3d10::ID3D10Buffer>;
    fn SetTextureBuffer(&self, ptexturebuffer: windows_core::Ref<super::d3d10::ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn GetTextureBuffer(&self) -> windows_core::Result<super::d3d10::ID3D10ShaderResourceView>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectConstantBuffer_Vtbl {
    pub const fn new<Identity: ID3D10EffectConstantBuffer_Impl>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn SetConstantBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pconstantbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::SetConstantBuffer(this, core::mem::transmute_copy(&pconstantbuffer)).into()
            }
        }
        unsafe extern "system" fn GetConstantBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, ppconstantbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectConstantBuffer_Impl::GetConstantBuffer(this) {
                    Ok(ok__) => {
                        ppconstantbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTextureBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, ptexturebuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectConstantBuffer_Impl::SetTextureBuffer(this, core::mem::transmute_copy(&ptexturebuffer)).into()
            }
        }
        unsafe extern "system" fn GetTextureBuffer<Identity: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pptexturebuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectConstantBuffer_Impl::GetTextureBuffer(this) {
                    Ok(ok__) => {
                        pptexturebuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            SetConstantBuffer: SetConstantBuffer::<Identity>,
            GetConstantBuffer: GetConstantBuffer::<Identity>,
            SetTextureBuffer: SetTextureBuffer::<Identity>,
            GetTextureBuffer: GetTextureBuffer::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
struct ID3D10EffectConstantBuffer_ImplVtbl<T: ID3D10EffectConstantBuffer_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3d10")]
impl<T: ID3D10EffectConstantBuffer_Impl> ID3D10EffectConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D10EffectConstantBuffer_Vtbl = ID3D10EffectConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectConstantBuffer {
    pub fn new<'a, T: ID3D10EffectConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectDepthStencilVariable, ID3D10EffectDepthStencilVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectDepthStencilVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectDepthStencilVariable, ID3D10EffectVariable);
impl ID3D10EffectDepthStencilVariable {
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetDepthStencilState(&self, index: u32) -> windows_core::Result<super::d3d10::ID3D10DepthStencilState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDepthStencilState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetBackingStore(&self, index: u32, pdepthstencildesc: *mut super::d3d10::D3D10_DEPTH_STENCIL_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, pdepthstencildesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectDepthStencilVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub GetDepthStencilState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetDepthStencilState: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d10::D3D10_DEPTH_STENCIL_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetBackingStore: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10EffectDepthStencilVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetDepthStencilState(&self, index: u32) -> windows_core::Result<super::d3d10::ID3D10DepthStencilState>;
    fn GetBackingStore(&self, index: u32, pdepthstencildesc: *mut super::d3d10::D3D10_DEPTH_STENCIL_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectDepthStencilVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectDepthStencilVariable_Impl>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencilState<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppdepthstencilstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectDepthStencilVariable_Impl::GetDepthStencilState(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppdepthstencilstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pdepthstencildesc: *mut super::d3d10::D3D10_DEPTH_STENCIL_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pdepthstencildesc)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            GetDepthStencilState: GetDepthStencilState::<Identity>,
            GetBackingStore: GetBackingStore::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
struct ID3D10EffectDepthStencilVariable_ImplVtbl<T: ID3D10EffectDepthStencilVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3d10")]
impl<T: ID3D10EffectDepthStencilVariable_Impl> ID3D10EffectDepthStencilVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectDepthStencilVariable_Vtbl = ID3D10EffectDepthStencilVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectDepthStencilVariable {
    pub fn new<'a, T: ID3D10EffectDepthStencilVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectDepthStencilVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectDepthStencilViewVariable, ID3D10EffectDepthStencilViewVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectDepthStencilViewVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectDepthStencilViewVariable, ID3D10EffectVariable);
impl ID3D10EffectDepthStencilViewVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn SetDepthStencil<P0>(&self, presource: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d10::ID3D10DepthStencilView>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDepthStencil)(windows_core::Interface::as_raw(self), presource.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetDepthStencil(&self) -> windows_core::Result<super::d3d10::ID3D10DepthStencilView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDepthStencil)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn SetDepthStencilArray(&self, ppresources: &[Option<super::d3d10::ID3D10DepthStencilView>], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDepthStencilArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetDepthStencilArray(&self, ppresources: &mut [Option<super::d3d10::ID3D10DepthStencilView>], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDepthStencilArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub SetDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    SetDepthStencil: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetDepthStencil: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub SetDepthStencilArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    SetDepthStencilArray: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetDepthStencilArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetDepthStencilArray: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10EffectDepthStencilViewVariable_Impl: ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn SetDepthStencil(&self, presource: windows_core::Ref<super::d3d10::ID3D10DepthStencilView>) -> windows_core::Result<()>;
    fn GetDepthStencil(&self) -> windows_core::Result<super::d3d10::ID3D10DepthStencilView>;
    fn SetDepthStencilArray(&self, ppresources: *const Option<super::d3d10::ID3D10DepthStencilView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetDepthStencilArray(&self, ppresources: *mut Option<super::d3d10::ID3D10DepthStencilView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectDepthStencilViewVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn SetDepthStencil<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::SetDepthStencil(this, core::mem::transmute_copy(&presource)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencil<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectDepthStencilViewVariable_Impl::GetDepthStencil(this) {
                    Ok(ok__) => {
                        ppresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepthStencilArray<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::SetDepthStencilArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencilArray<Identity: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectDepthStencilViewVariable_Impl::GetDepthStencilArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            SetDepthStencil: SetDepthStencil::<Identity>,
            GetDepthStencil: GetDepthStencil::<Identity>,
            SetDepthStencilArray: SetDepthStencilArray::<Identity>,
            GetDepthStencilArray: GetDepthStencilArray::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
struct ID3D10EffectDepthStencilViewVariable_ImplVtbl<T: ID3D10EffectDepthStencilViewVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3d10")]
impl<T: ID3D10EffectDepthStencilViewVariable_Impl> ID3D10EffectDepthStencilViewVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectDepthStencilViewVariable_Vtbl = ID3D10EffectDepthStencilViewVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectDepthStencilViewVariable {
    pub fn new<'a, T: ID3D10EffectDepthStencilViewVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectDepthStencilViewVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectMatrixVariable, ID3D10EffectMatrixVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectMatrixVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectMatrixVariable, ID3D10EffectVariable);
impl ID3D10EffectMatrixVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, byteoffset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, byteoffset, bytecount) }
    }
    pub unsafe fn SetMatrix(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMatrix(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatrix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
    pub unsafe fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
    pub unsafe fn SetMatrixTranspose(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetMatrixTranspose)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMatrixTranspose(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatrixTranspose)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixTransposeArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
    pub unsafe fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixTransposeArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectMatrixVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMatrixArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetMatrixArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub SetMatrixTranspose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMatrixTranspose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMatrixTransposeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetMatrixTransposeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
}
pub trait ID3D10EffectMatrixVariable_Impl: ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn SetMatrix(&self) -> windows_core::Result<f32>;
    fn GetMatrix(&self) -> windows_core::Result<f32>;
    fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetMatrixTranspose(&self) -> windows_core::Result<f32>;
    fn GetMatrixTranspose(&self) -> windows_core::Result<f32>;
    fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectMatrixVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectMatrixVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&byteoffset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&byteoffset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn SetMatrix<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectMatrixVariable_Impl::SetMatrix(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMatrix<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectMatrixVariable_Impl::GetMatrix(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMatrixArray<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::SetMatrixArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetMatrixArray<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMatrixArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetMatrixTranspose<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectMatrixVariable_Impl::SetMatrixTranspose(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMatrixTranspose<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectMatrixVariable_Impl::GetMatrixTranspose(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMatrixTransposeArray<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::SetMatrixTransposeArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetMatrixTransposeArray<Identity: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectMatrixVariable_Impl::GetMatrixTransposeArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            SetMatrix: SetMatrix::<Identity>,
            GetMatrix: GetMatrix::<Identity>,
            SetMatrixArray: SetMatrixArray::<Identity>,
            GetMatrixArray: GetMatrixArray::<Identity>,
            SetMatrixTranspose: SetMatrixTranspose::<Identity>,
            GetMatrixTranspose: GetMatrixTranspose::<Identity>,
            SetMatrixTransposeArray: SetMatrixTransposeArray::<Identity>,
            GetMatrixTransposeArray: GetMatrixTransposeArray::<Identity>,
        }
    }
}
struct ID3D10EffectMatrixVariable_ImplVtbl<T: ID3D10EffectMatrixVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectMatrixVariable_Impl> ID3D10EffectMatrixVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectMatrixVariable_Vtbl = ID3D10EffectMatrixVariable_Vtbl::new::<T>();
}
impl ID3D10EffectMatrixVariable {
    pub fn new<'a, T: ID3D10EffectMatrixVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectMatrixVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectPass, ID3D10EffectPass_Vtbl);
impl ID3D10EffectPass {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetVertexShaderDesc(&self) -> windows_core::Result<D3D10_PASS_SHADER_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexShaderDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetGeometryShaderDesc(&self) -> windows_core::Result<D3D10_PASS_SHADER_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeometryShaderDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPixelShaderDesc(&self) -> windows_core::Result<D3D10_PASS_SHADER_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelShaderDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn Apply(&self, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ComputeStateBlockMask)(windows_core::Interface::as_raw(self), pstateblockmask as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectPass_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_DESC) -> windows_core::HRESULT,
    pub GetVertexShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetGeometryShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetPixelShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ComputeStateBlockMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT,
}
pub trait ID3D10EffectPass_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> windows_core::Result<()>;
    fn GetVertexShaderDesc(&self) -> windows_core::Result<D3D10_PASS_SHADER_DESC>;
    fn GetGeometryShaderDesc(&self) -> windows_core::Result<D3D10_PASS_SHADER_DESC>;
    fn GetPixelShaderDesc(&self) -> windows_core::Result<D3D10_PASS_SHADER_DESC>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn Apply(&self, flags: u32) -> windows_core::Result<()>;
    fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()>;
}
impl ID3D10EffectPass_Vtbl {
    pub const fn new<Identity: ID3D10EffectPass_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderDesc<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectPass_Impl::GetVertexShaderDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGeometryShaderDesc<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectPass_Impl::GetGeometryShaderDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelShaderDesc<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectPass_Impl::GetPixelShaderDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn Apply<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::Apply(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn ComputeStateBlockMask<Identity: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectPass_Impl::ComputeStateBlockMask(this, core::mem::transmute_copy(&pstateblockmask)).into()
            }
        }
        Self {
            IsValid: IsValid::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetVertexShaderDesc: GetVertexShaderDesc::<Identity>,
            GetGeometryShaderDesc: GetGeometryShaderDesc::<Identity>,
            GetPixelShaderDesc: GetPixelShaderDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            Apply: Apply::<Identity>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Identity>,
        }
    }
}
struct ID3D10EffectPass_ImplVtbl<T: ID3D10EffectPass_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectPass_Impl> ID3D10EffectPass_ImplVtbl<T> {
    const VTABLE: ID3D10EffectPass_Vtbl = ID3D10EffectPass_Vtbl::new::<T>();
}
impl ID3D10EffectPass {
    pub fn new<'a, T: ID3D10EffectPass_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectPass_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectPool, ID3D10EffectPool_Vtbl, 0x9537ab04_3250_412e_8213_fcd2f8677933);
windows_core::imp::interface_hierarchy!(ID3D10EffectPool, windows_core::IUnknown);
impl ID3D10EffectPool {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsEffect(&self) -> Option<ID3D10Effect> {
        unsafe { (windows_core::Interface::vtable(self).AsEffect)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectPool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub AsEffect: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10Effect>,
}
pub trait ID3D10EffectPool_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn AsEffect(&self) -> Option<ID3D10Effect>;
}
impl ID3D10EffectPool_Vtbl {
    pub const fn new<Identity: ID3D10EffectPool_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D10EffectPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10EffectPool_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D10EffectPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10EffectPool_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D10EffectPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10EffectPool_Impl::Release(this)
            }
        }
        unsafe extern "system" fn AsEffect<Identity: ID3D10EffectPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<ID3D10Effect> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10EffectPool_Impl::AsEffect(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            AsEffect: AsEffect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10EffectPool as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D10EffectPool {}
windows_core::imp::define_interface!(ID3D10EffectRasterizerVariable, ID3D10EffectRasterizerVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectRasterizerVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectRasterizerVariable, ID3D10EffectVariable);
impl ID3D10EffectRasterizerVariable {
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetRasterizerState(&self, index: u32) -> windows_core::Result<super::d3d10::ID3D10RasterizerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRasterizerState)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetBackingStore(&self, index: u32, prasterizerdesc: *mut super::d3d10::D3D10_RASTERIZER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, prasterizerdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectRasterizerVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub GetRasterizerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetRasterizerState: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d10::D3D10_RASTERIZER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetBackingStore: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10EffectRasterizerVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRasterizerState(&self, index: u32) -> windows_core::Result<super::d3d10::ID3D10RasterizerState>;
    fn GetBackingStore(&self, index: u32, prasterizerdesc: *mut super::d3d10::D3D10_RASTERIZER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectRasterizerVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectRasterizerVariable_Impl>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRasterizerState<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectRasterizerVariable_Impl::GetRasterizerState(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprasterizerstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, prasterizerdesc: *mut super::d3d10::D3D10_RASTERIZER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRasterizerVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&prasterizerdesc)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            GetRasterizerState: GetRasterizerState::<Identity>,
            GetBackingStore: GetBackingStore::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
struct ID3D10EffectRasterizerVariable_ImplVtbl<T: ID3D10EffectRasterizerVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3d10")]
impl<T: ID3D10EffectRasterizerVariable_Impl> ID3D10EffectRasterizerVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectRasterizerVariable_Vtbl = ID3D10EffectRasterizerVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectRasterizerVariable {
    pub fn new<'a, T: ID3D10EffectRasterizerVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectRasterizerVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectRenderTargetViewVariable, ID3D10EffectRenderTargetViewVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectRenderTargetViewVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectRenderTargetViewVariable, ID3D10EffectVariable);
impl ID3D10EffectRenderTargetViewVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn SetRenderTarget<P0>(&self, presource: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d10::ID3D10RenderTargetView>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTarget)(windows_core::Interface::as_raw(self), presource.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetRenderTarget(&self) -> windows_core::Result<super::d3d10::ID3D10RenderTargetView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn SetRenderTargetArray(&self, ppresources: &[Option<super::d3d10::ID3D10RenderTargetView>], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTargetArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetRenderTargetArray(&self, ppresources: &mut [Option<super::d3d10::ID3D10RenderTargetView>], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRenderTargetArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub SetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    SetRenderTarget: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetRenderTarget: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub SetRenderTargetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    SetRenderTargetArray: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetRenderTargetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetRenderTargetArray: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10EffectRenderTargetViewVariable_Impl: ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn SetRenderTarget(&self, presource: windows_core::Ref<super::d3d10::ID3D10RenderTargetView>) -> windows_core::Result<()>;
    fn GetRenderTarget(&self) -> windows_core::Result<super::d3d10::ID3D10RenderTargetView>;
    fn SetRenderTargetArray(&self, ppresources: *const Option<super::d3d10::ID3D10RenderTargetView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetRenderTargetArray(&self, ppresources: *mut Option<super::d3d10::ID3D10RenderTargetView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectRenderTargetViewVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn SetRenderTarget<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::SetRenderTarget(this, core::mem::transmute_copy(&presource)).into()
            }
        }
        unsafe extern "system" fn GetRenderTarget<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectRenderTargetViewVariable_Impl::GetRenderTarget(this) {
                    Ok(ok__) => {
                        ppresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRenderTargetArray<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::SetRenderTargetArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetRenderTargetArray<Identity: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectRenderTargetViewVariable_Impl::GetRenderTargetArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            SetRenderTarget: SetRenderTarget::<Identity>,
            GetRenderTarget: GetRenderTarget::<Identity>,
            SetRenderTargetArray: SetRenderTargetArray::<Identity>,
            GetRenderTargetArray: GetRenderTargetArray::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
struct ID3D10EffectRenderTargetViewVariable_ImplVtbl<T: ID3D10EffectRenderTargetViewVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3d10")]
impl<T: ID3D10EffectRenderTargetViewVariable_Impl> ID3D10EffectRenderTargetViewVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectRenderTargetViewVariable_Vtbl = ID3D10EffectRenderTargetViewVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectRenderTargetViewVariable {
    pub fn new<'a, T: ID3D10EffectRenderTargetViewVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectRenderTargetViewVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectSamplerVariable, ID3D10EffectSamplerVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectSamplerVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectSamplerVariable, ID3D10EffectVariable);
impl ID3D10EffectSamplerVariable {
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetSampler(&self, index: u32) -> windows_core::Result<super::d3d10::ID3D10SamplerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSampler)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetBackingStore(&self, index: u32, psamplerdesc: *mut super::d3d10::D3D10_SAMPLER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBackingStore)(windows_core::Interface::as_raw(self), index, psamplerdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectSamplerVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub GetSampler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetSampler: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetBackingStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::d3d10::D3D10_SAMPLER_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetBackingStore: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10EffectSamplerVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetSampler(&self, index: u32) -> windows_core::Result<super::d3d10::ID3D10SamplerState>;
    fn GetBackingStore(&self, index: u32, psamplerdesc: *mut super::d3d10::D3D10_SAMPLER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectSamplerVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectSamplerVariable_Impl>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetSampler<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppsampler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectSamplerVariable_Impl::GetSampler(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppsampler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackingStore<Identity: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, psamplerdesc: *mut super::d3d10::D3D10_SAMPLER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectSamplerVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&psamplerdesc)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            GetSampler: GetSampler::<Identity>,
            GetBackingStore: GetBackingStore::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
struct ID3D10EffectSamplerVariable_ImplVtbl<T: ID3D10EffectSamplerVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3d10")]
impl<T: ID3D10EffectSamplerVariable_Impl> ID3D10EffectSamplerVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectSamplerVariable_Vtbl = ID3D10EffectSamplerVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectSamplerVariable {
    pub fn new<'a, T: ID3D10EffectSamplerVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectSamplerVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectScalarVariable, ID3D10EffectScalarVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectScalarVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectScalarVariable, ID3D10EffectVariable);
impl ID3D10EffectScalarVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, byteoffset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, byteoffset, bytecount) }
    }
    pub unsafe fn SetFloat(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFloat)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetFloat(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFloat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFloatArray(&self, pdata: &[f32], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFloatArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()) }
    }
    pub unsafe fn GetFloatArray(&self, pdata: &mut [f32], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFloatArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()) }
    }
    pub unsafe fn SetInt(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInt)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetInt(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIntArray(&self, pdata: &[i32], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIntArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()) }
    }
    pub unsafe fn GetIntArray(&self, pdata: &mut [i32], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIntArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()) }
    }
    pub unsafe fn SetBool(&self, value: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBool)(windows_core::Interface::as_raw(self), value.into()) }
    }
    pub unsafe fn GetBool(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBool)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBoolArray(&self, pdata: &[windows_core::BOOL], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBoolArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()) }
    }
    pub unsafe fn GetBoolArray(&self, pdata: &mut [windows_core::BOOL], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBoolArray)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), offset, pdata.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectScalarVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetFloat: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetFloat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetFloatArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, u32) -> windows_core::HRESULT,
    pub GetFloatArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub SetInt: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetInt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIntArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32, u32) -> windows_core::HRESULT,
    pub GetIntArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, u32) -> windows_core::HRESULT,
    pub SetBool: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetBool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBoolArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    pub GetBoolArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
}
pub trait ID3D10EffectScalarVariable_Impl: ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn SetFloat(&self, value: f32) -> windows_core::Result<()>;
    fn GetFloat(&self) -> windows_core::Result<f32>;
    fn SetFloatArray(&self, pdata: *const f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetFloatArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetInt(&self, value: i32) -> windows_core::Result<()>;
    fn GetInt(&self) -> windows_core::Result<i32>;
    fn SetIntArray(&self, pdata: *const i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetIntArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetBool(&self, value: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetBool(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetBoolArray(&self, pdata: *const windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetBoolArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectScalarVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectScalarVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&byteoffset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&byteoffset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn SetFloat<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetFloat(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetFloat<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectScalarVariable_Impl::GetFloat(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFloatArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetFloatArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetFloatArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetFloatArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetInt<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetInt(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetInt<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectScalarVariable_Impl::GetInt(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIntArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetIntArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetIntArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetIntArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetBool<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetBool(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetBool<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectScalarVariable_Impl::GetBool(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBoolArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::SetBoolArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetBoolArray<Identity: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectScalarVariable_Impl::GetBoolArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            SetFloat: SetFloat::<Identity>,
            GetFloat: GetFloat::<Identity>,
            SetFloatArray: SetFloatArray::<Identity>,
            GetFloatArray: GetFloatArray::<Identity>,
            SetInt: SetInt::<Identity>,
            GetInt: GetInt::<Identity>,
            SetIntArray: SetIntArray::<Identity>,
            GetIntArray: GetIntArray::<Identity>,
            SetBool: SetBool::<Identity>,
            GetBool: GetBool::<Identity>,
            SetBoolArray: SetBoolArray::<Identity>,
            GetBoolArray: GetBoolArray::<Identity>,
        }
    }
}
struct ID3D10EffectScalarVariable_ImplVtbl<T: ID3D10EffectScalarVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectScalarVariable_Impl> ID3D10EffectScalarVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectScalarVariable_Vtbl = ID3D10EffectScalarVariable_Vtbl::new::<T>();
}
impl ID3D10EffectScalarVariable {
    pub fn new<'a, T: ID3D10EffectScalarVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectScalarVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectShaderResourceVariable, ID3D10EffectShaderResourceVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectShaderResourceVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectShaderResourceVariable, ID3D10EffectVariable);
impl ID3D10EffectShaderResourceVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn SetResource<P0>(&self, presource: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d10::ID3D10ShaderResourceView>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetResource)(windows_core::Interface::as_raw(self), presource.param().abi()) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetResource(&self) -> windows_core::Result<super::d3d10::ID3D10ShaderResourceView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn SetResourceArray(&self, ppresources: &[Option<super::d3d10::ID3D10ShaderResourceView>], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResourceArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetResourceArray(&self, ppresources: &mut [Option<super::d3d10::ID3D10ShaderResourceView>], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResourceArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources.as_ptr()), offset, ppresources.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectShaderResourceVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub SetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    SetResource: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetResource: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub SetResourceArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    SetResourceArray: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetResourceArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetResourceArray: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10EffectShaderResourceVariable_Impl: ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn SetResource(&self, presource: windows_core::Ref<super::d3d10::ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn GetResource(&self) -> windows_core::Result<super::d3d10::ID3D10ShaderResourceView>;
    fn SetResourceArray(&self, ppresources: *const Option<super::d3d10::ID3D10ShaderResourceView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetResourceArray(&self, ppresources: *mut Option<super::d3d10::ID3D10ShaderResourceView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectShaderResourceVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectShaderResourceVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn SetResource<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::SetResource(this, core::mem::transmute_copy(&presource)).into()
            }
        }
        unsafe extern "system" fn GetResource<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectShaderResourceVariable_Impl::GetResource(this) {
                    Ok(ok__) => {
                        ppresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetResourceArray<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::SetResourceArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetResourceArray<Identity: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderResourceVariable_Impl::GetResourceArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            SetResource: SetResource::<Identity>,
            GetResource: GetResource::<Identity>,
            SetResourceArray: SetResourceArray::<Identity>,
            GetResourceArray: GetResourceArray::<Identity>,
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
struct ID3D10EffectShaderResourceVariable_ImplVtbl<T: ID3D10EffectShaderResourceVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_d3d10")]
impl<T: ID3D10EffectShaderResourceVariable_Impl> ID3D10EffectShaderResourceVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectShaderResourceVariable_Vtbl = ID3D10EffectShaderResourceVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10EffectShaderResourceVariable {
    pub fn new<'a, T: ID3D10EffectShaderResourceVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectShaderResourceVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectShaderVariable, ID3D10EffectShaderVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectShaderVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectShaderVariable, ID3D10EffectVariable);
impl ID3D10EffectShaderVariable {
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    pub unsafe fn GetShaderDesc(&self, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetShaderDesc)(windows_core::Interface::as_raw(self), shaderindex, pdesc as _) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetVertexShader(&self, shaderindex: u32) -> windows_core::Result<super::d3d10::ID3D10VertexShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetGeometryShader(&self, shaderindex: u32) -> windows_core::Result<super::d3d10::ID3D10GeometryShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeometryShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetPixelShader(&self, shaderindex: u32) -> windows_core::Result<super::d3d10::ID3D10PixelShader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelShader)(windows_core::Interface::as_raw(self), shaderindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputSignatureElementDesc)(windows_core::Interface::as_raw(self), shaderindex, element, pdesc as _) }
    }
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputSignatureElementDesc)(windows_core::Interface::as_raw(self), shaderindex, element, pdesc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectShaderVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetShaderDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub GetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetVertexShader: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetGeometryShader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetGeometryShader: usize,
    #[cfg(feature = "Win32_d3d10")]
    pub GetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetPixelShader: usize,
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub GetInputSignatureElementDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon")))]
    GetInputSignatureElementDesc: usize,
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub GetOutputSignatureElementDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon")))]
    GetOutputSignatureElementDesc: usize,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
pub trait ID3D10EffectShaderVariable_Impl: ID3D10EffectVariable_Impl {
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetShaderDesc(&self, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::Result<()>;
    fn GetVertexShader(&self, shaderindex: u32) -> windows_core::Result<super::d3d10::ID3D10VertexShader>;
    fn GetGeometryShader(&self, shaderindex: u32) -> windows_core::Result<super::d3d10::ID3D10GeometryShader>;
    fn GetPixelShader(&self, shaderindex: u32) -> windows_core::Result<super::d3d10::ID3D10PixelShader>;
    fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl ID3D10EffectShaderVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectShaderVariable_Impl>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetShaderDesc<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetShaderDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVertexShader<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppvs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectShaderVariable_Impl::GetVertexShader(this, core::mem::transmute_copy(&shaderindex)) {
                    Ok(ok__) => {
                        ppvs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGeometryShader<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppgs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectShaderVariable_Impl::GetGeometryShader(this, core::mem::transmute_copy(&shaderindex)) {
                    Ok(ok__) => {
                        ppgs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelShader<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectShaderVariable_Impl::GetPixelShader(this, core::mem::transmute_copy(&shaderindex)) {
                    Ok(ok__) => {
                        ppps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputSignatureElementDesc<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetInputSignatureElementDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&element), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetOutputSignatureElementDesc<Identity: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut super::d3d10shader::D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectShaderVariable_Impl::GetOutputSignatureElementDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&element), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            GetShaderDesc: GetShaderDesc::<Identity>,
            GetVertexShader: GetVertexShader::<Identity>,
            GetGeometryShader: GetGeometryShader::<Identity>,
            GetPixelShader: GetPixelShader::<Identity>,
            GetInputSignatureElementDesc: GetInputSignatureElementDesc::<Identity>,
            GetOutputSignatureElementDesc: GetOutputSignatureElementDesc::<Identity>,
        }
    }
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
struct ID3D10EffectShaderVariable_ImplVtbl<T: ID3D10EffectShaderVariable_Impl>(core::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl<T: ID3D10EffectShaderVariable_Impl> ID3D10EffectShaderVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectShaderVariable_Vtbl = ID3D10EffectShaderVariable_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl ID3D10EffectShaderVariable {
    pub fn new<'a, T: ID3D10EffectShaderVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectShaderVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectStringVariable, ID3D10EffectStringVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectStringVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectStringVariable, ID3D10EffectVariable);
impl ID3D10EffectStringVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
    pub unsafe fn GetString(&self) -> windows_core::Result<windows_core::PCSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStringArray(&self, ppstrings: &mut [windows_core::PCSTR], offset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStringArray)(windows_core::Interface::as_raw(self), core::mem::transmute(ppstrings.as_ptr()), offset, ppstrings.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectStringVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCSTR) -> windows_core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCSTR, u32, u32) -> windows_core::HRESULT,
}
pub trait ID3D10EffectStringVariable_Impl: ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetString(&self) -> windows_core::Result<windows_core::PCSTR>;
    fn GetStringArray(&self, ppstrings: *mut windows_core::PCSTR, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectStringVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectStringVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetString<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, ppstring: *mut windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectStringVariable_Impl::GetString(this) {
                    Ok(ok__) => {
                        ppstring.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringArray<Identity: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, ppstrings: *mut windows_core::PCSTR, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectStringVariable_Impl::GetStringArray(this, core::mem::transmute_copy(&ppstrings), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            GetString: GetString::<Identity>,
            GetStringArray: GetStringArray::<Identity>,
        }
    }
}
struct ID3D10EffectStringVariable_ImplVtbl<T: ID3D10EffectStringVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectStringVariable_Impl> ID3D10EffectStringVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectStringVariable_Vtbl = ID3D10EffectStringVariable_Vtbl::new::<T>();
}
impl ID3D10EffectStringVariable {
    pub fn new<'a, T: ID3D10EffectStringVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectStringVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectTechnique, ID3D10EffectTechnique_Vtbl);
impl ID3D10EffectTechnique {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self) -> windows_core::Result<D3D10_TECHNIQUE_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetPassByIndex(&self, index: u32) -> Option<ID3D10EffectPass> {
        unsafe { (windows_core::Interface::vtable(self).GetPassByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetPassByName<P0>(&self, name: P0) -> Option<ID3D10EffectPass>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPassByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ComputeStateBlockMask)(windows_core::Interface::as_raw(self), pstateblockmask as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectTechnique_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_TECHNIQUE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetPassByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectPass>,
    pub GetPassByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectPass>,
    pub ComputeStateBlockMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT,
}
pub trait ID3D10EffectTechnique_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetDesc(&self) -> windows_core::Result<D3D10_TECHNIQUE_DESC>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetPassByIndex(&self, index: u32) -> Option<ID3D10EffectPass>;
    fn GetPassByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectPass>;
    fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()>;
}
impl ID3D10EffectTechnique_Vtbl {
    pub const fn new<Identity: ID3D10EffectTechnique_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectTechnique_Impl::GetDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetPassByIndex<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectPass> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetPassByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetPassByName<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectPass> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::GetPassByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn ComputeStateBlockMask<Identity: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectTechnique_Impl::ComputeStateBlockMask(this, core::mem::transmute_copy(&pstateblockmask)).into()
            }
        }
        Self {
            IsValid: IsValid::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetPassByIndex: GetPassByIndex::<Identity>,
            GetPassByName: GetPassByName::<Identity>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Identity>,
        }
    }
}
struct ID3D10EffectTechnique_ImplVtbl<T: ID3D10EffectTechnique_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectTechnique_Impl> ID3D10EffectTechnique_ImplVtbl<T> {
    const VTABLE: ID3D10EffectTechnique_Vtbl = ID3D10EffectTechnique_Vtbl::new::<T>();
}
impl ID3D10EffectTechnique {
    pub fn new<'a, T: ID3D10EffectTechnique_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectTechnique_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectType, ID3D10EffectType_Vtbl);
impl ID3D10EffectType {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::HRESULT {
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
    pub unsafe fn GetMemberTypeBySemantic<P0>(&self, semantic: P0) -> Option<Self>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberTypeBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetMemberName(&self, index: u32) -> windows_core::PCSTR {
        unsafe { (windows_core::Interface::vtable(self).GetMemberName)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberSemantic(&self, index: u32) -> windows_core::PCSTR {
        unsafe { (windows_core::Interface::vtable(self).GetMemberSemantic)(windows_core::Interface::as_raw(self), index) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectType_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon")))]
    GetDesc: usize,
    pub GetMemberTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectType>,
    pub GetMemberTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectType>,
    pub GetMemberTypeBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectType>,
    pub GetMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
    pub GetMemberSemantic: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
pub trait ID3D10EffectType_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10EffectType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectType>;
    fn GetMemberTypeBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectType>;
    fn GetMemberName(&self, index: u32) -> windows_core::PCSTR;
    fn GetMemberSemantic(&self, index: u32) -> windows_core::PCSTR;
}
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl ID3D10EffectType_Vtbl {
    pub const fn new<Identity: ID3D10EffectType_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberTypeByName<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberTypeBySemantic<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberTypeBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetMemberName<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberName(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberSemantic<Identity: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectType_Impl::GetMemberSemantic(this, core::mem::transmute_copy(&index))
            }
        }
        Self {
            IsValid: IsValid::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Identity>,
            GetMemberTypeByName: GetMemberTypeByName::<Identity>,
            GetMemberTypeBySemantic: GetMemberTypeBySemantic::<Identity>,
            GetMemberName: GetMemberName::<Identity>,
            GetMemberSemantic: GetMemberSemantic::<Identity>,
        }
    }
}
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
struct ID3D10EffectType_ImplVtbl<T: ID3D10EffectType_Impl>(core::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl<T: ID3D10EffectType_Impl> ID3D10EffectType_ImplVtbl<T> {
    const VTABLE: ID3D10EffectType_Vtbl = ID3D10EffectType_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl ID3D10EffectType {
    pub fn new<'a, T: ID3D10EffectType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectVariable, ID3D10EffectVariable_Vtbl);
impl ID3D10EffectVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<Self>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<Self>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<Self>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<ID3D10EffectVectorVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, offset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, offset, bytecount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVariable_Vtbl {
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        Self {
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
        }
    }
}
struct ID3D10EffectVariable_ImplVtbl<T: ID3D10EffectVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectVariable_Impl> ID3D10EffectVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectVariable_Vtbl = ID3D10EffectVariable_Vtbl::new::<T>();
}
impl ID3D10EffectVariable {
    pub fn new<'a, T: ID3D10EffectVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10EffectVectorVariable, ID3D10EffectVectorVariable_Vtbl);
impl core::ops::Deref for ID3D10EffectVectorVariable {
    type Target = ID3D10EffectVariable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D10EffectVectorVariable, ID3D10EffectVariable);
impl ID3D10EffectVectorVariable {
    pub unsafe fn IsValid(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsValid)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> Option<ID3D10EffectType> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAnnotationByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberByName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> Option<ID3D10EffectVariable>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMemberBySemantic)(windows_core::Interface::as_raw(self), semantic.param().abi()) }
    }
    pub unsafe fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable> {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetParentConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsScalar)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsVector(&self) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).AsVector)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsMatrix)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsString(&self) -> Option<ID3D10EffectStringVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsString)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShaderResource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRenderTargetView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencilView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer> {
        unsafe { (windows_core::Interface::vtable(self).AsConstantBuffer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsShader(&self) -> Option<ID3D10EffectShaderVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsShader)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsBlend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsDepthStencil)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsRasterizer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable> {
        unsafe { (windows_core::Interface::vtable(self).AsSampler)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRawValue(&self, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRawValue)(windows_core::Interface::as_raw(self), pdata, byteoffset, bytecount) }
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRawValue)(windows_core::Interface::as_raw(self), pdata as _, byteoffset, bytecount) }
    }
    pub unsafe fn SetBoolVector(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetBoolVector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIntVector(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetIntVector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFloatVector(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetFloatVector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBoolVector(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoolVector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIntVector(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntVector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFloatVector(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFloatVector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBoolVectorArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBoolVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
    pub unsafe fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIntVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
    pub unsafe fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFloatVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
    pub unsafe fn GetBoolVectorArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBoolVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
    pub unsafe fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIntVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
    pub unsafe fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFloatVectorArray)(windows_core::Interface::as_raw(self), pdata as _, offset, count) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10EffectVectorVariable_Vtbl {
    pub base__: ID3D10EffectVariable_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectType>,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT,
    pub GetAnnotationByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetAnnotationByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetMemberBySemantic: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> Option<ID3D10EffectVariable>,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<ID3D10EffectVariable>,
    pub GetParentConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsScalar: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable>,
    pub AsVector: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable>,
    pub AsMatrix: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable>,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable>,
    pub AsShaderResource: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable>,
    pub AsRenderTargetView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable>,
    pub AsDepthStencilView: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable>,
    pub AsConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer>,
    pub AsShader: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable>,
    pub AsBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable>,
    pub AsDepthStencil: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable>,
    pub AsRasterizer: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable>,
    pub AsSampler: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable>,
    pub SetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetRawValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetBoolVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIntVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFloatVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetBoolVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetIntVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFloatVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBoolVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    pub SetIntVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, u32) -> windows_core::HRESULT,
    pub SetFloatVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetBoolVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    pub GetIntVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, u32) -> windows_core::HRESULT,
    pub GetFloatVectorArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
}
pub trait ID3D10EffectVectorVariable_Impl: ID3D10EffectVariable_Impl {
    fn IsValid(&self) -> windows_core::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn SetBoolVector(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIntVector(&self) -> windows_core::Result<i32>;
    fn SetFloatVector(&self) -> windows_core::Result<f32>;
    fn GetBoolVector(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetIntVector(&self) -> windows_core::Result<i32>;
    fn GetFloatVector(&self) -> windows_core::Result<f32>;
    fn SetBoolVectorArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetBoolVectorArray(&self, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectVectorVariable_Vtbl {
    pub const fn new<Identity: ID3D10EffectVectorVariable_Impl>() -> Self {
        unsafe extern "system" fn IsValid<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::IsValid(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetAnnotationByIndex<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetAnnotationByName<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberByIndex<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetMemberByName<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn GetMemberBySemantic<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
            }
        }
        unsafe extern "system" fn GetElement<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn GetParentConstantBuffer<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetParentConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsScalar<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsScalar(this)
            }
        }
        unsafe extern "system" fn AsVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsVector(this)
            }
        }
        unsafe extern "system" fn AsMatrix<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsMatrix(this)
            }
        }
        unsafe extern "system" fn AsString<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsString(this)
            }
        }
        unsafe extern "system" fn AsShaderResource<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsShaderResource(this)
            }
        }
        unsafe extern "system" fn AsRenderTargetView<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsRenderTargetView(this)
            }
        }
        unsafe extern "system" fn AsDepthStencilView<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsDepthStencilView(this)
            }
        }
        unsafe extern "system" fn AsConstantBuffer<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsConstantBuffer(this)
            }
        }
        unsafe extern "system" fn AsShader<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsShader(this)
            }
        }
        unsafe extern "system" fn AsBlend<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsBlend(this)
            }
        }
        unsafe extern "system" fn AsDepthStencil<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsDepthStencil(this)
            }
        }
        unsafe extern "system" fn AsRasterizer<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsRasterizer(this)
            }
        }
        unsafe extern "system" fn AsSampler<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::AsSampler(this)
            }
        }
        unsafe extern "system" fn SetRawValue<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&byteoffset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn GetRawValue<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, byteoffset: u32, bytecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&byteoffset), core::mem::transmute_copy(&bytecount)).into()
            }
        }
        unsafe extern "system" fn SetBoolVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectVectorVariable_Impl::SetBoolVector(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIntVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectVectorVariable_Impl::SetIntVector(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFloatVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectVectorVariable_Impl::SetFloatVector(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoolVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectVectorVariable_Impl::GetBoolVector(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIntVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectVectorVariable_Impl::GetIntVector(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFloatVector<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match ID3D10EffectVectorVariable_Impl::GetFloatVector(this) {
                    Ok(ok__) => {
                        pdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBoolVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetBoolVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetIntVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetIntVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetFloatVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::SetFloatVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetBoolVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut windows_core::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetBoolVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetIntVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetIntVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetFloatVectorArray<Identity: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                ID3D10EffectVectorVariable_Impl::GetFloatVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Identity>(),
            IsValid: IsValid::<Identity>,
            GetType: GetType::<Identity>,
            GetDesc: GetDesc::<Identity>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Identity>,
            GetAnnotationByName: GetAnnotationByName::<Identity>,
            GetMemberByIndex: GetMemberByIndex::<Identity>,
            GetMemberByName: GetMemberByName::<Identity>,
            GetMemberBySemantic: GetMemberBySemantic::<Identity>,
            GetElement: GetElement::<Identity>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Identity>,
            AsScalar: AsScalar::<Identity>,
            AsVector: AsVector::<Identity>,
            AsMatrix: AsMatrix::<Identity>,
            AsString: AsString::<Identity>,
            AsShaderResource: AsShaderResource::<Identity>,
            AsRenderTargetView: AsRenderTargetView::<Identity>,
            AsDepthStencilView: AsDepthStencilView::<Identity>,
            AsConstantBuffer: AsConstantBuffer::<Identity>,
            AsShader: AsShader::<Identity>,
            AsBlend: AsBlend::<Identity>,
            AsDepthStencil: AsDepthStencil::<Identity>,
            AsRasterizer: AsRasterizer::<Identity>,
            AsSampler: AsSampler::<Identity>,
            SetRawValue: SetRawValue::<Identity>,
            GetRawValue: GetRawValue::<Identity>,
            SetBoolVector: SetBoolVector::<Identity>,
            SetIntVector: SetIntVector::<Identity>,
            SetFloatVector: SetFloatVector::<Identity>,
            GetBoolVector: GetBoolVector::<Identity>,
            GetIntVector: GetIntVector::<Identity>,
            GetFloatVector: GetFloatVector::<Identity>,
            SetBoolVectorArray: SetBoolVectorArray::<Identity>,
            SetIntVectorArray: SetIntVectorArray::<Identity>,
            SetFloatVectorArray: SetFloatVectorArray::<Identity>,
            GetBoolVectorArray: GetBoolVectorArray::<Identity>,
            GetIntVectorArray: GetIntVectorArray::<Identity>,
            GetFloatVectorArray: GetFloatVectorArray::<Identity>,
        }
    }
}
struct ID3D10EffectVectorVariable_ImplVtbl<T: ID3D10EffectVectorVariable_Impl>(core::marker::PhantomData<T>);
impl<T: ID3D10EffectVectorVariable_Impl> ID3D10EffectVectorVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectVectorVariable_Vtbl = ID3D10EffectVectorVariable_Vtbl::new::<T>();
}
impl ID3D10EffectVectorVariable {
    pub fn new<'a, T: ID3D10EffectVectorVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectVectorVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(ID3D10StateBlock, ID3D10StateBlock_Vtbl, 0x0803425a_57f5_4dd6_9465_a87570834a08);
windows_core::imp::interface_hierarchy!(ID3D10StateBlock, windows_core::IUnknown);
impl ID3D10StateBlock {
    pub unsafe fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), iid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Capture(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Capture)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Apply(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseAllDeviceObjects(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseAllDeviceObjects)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d3d10")]
    pub unsafe fn GetDevice(&self) -> windows_core::Result<super::d3d10::ID3D10Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10StateBlock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Capture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseAllDeviceObjects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d10")]
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d10"))]
    GetDevice: usize,
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10StateBlock_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn Capture(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn ReleaseAllDeviceObjects(&self) -> windows_core::Result<()>;
    fn GetDevice(&self) -> windows_core::Result<super::d3d10::ID3D10Device>;
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10StateBlock_Vtbl {
    pub const fn new<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::QueryInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::Release(this)
            }
        }
        unsafe extern "system" fn Capture<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::Capture(this).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::Apply(this).into()
            }
        }
        unsafe extern "system" fn ReleaseAllDeviceObjects<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10StateBlock_Impl::ReleaseAllDeviceObjects(this).into()
            }
        }
        unsafe extern "system" fn GetDevice<Identity: ID3D10StateBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D10StateBlock_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
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
            Capture: Capture::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            ReleaseAllDeviceObjects: ReleaseAllDeviceObjects::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10StateBlock as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d10")]
impl windows_core::RuntimeName for ID3D10StateBlock {}
