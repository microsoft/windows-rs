#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
windows_link::link!("d3d10.dll" "system" fn D3D10CompileEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, psrcfilename : windows_sys::core::PCSTR, pdefines : *const super::d3d10shader::D3D10_SHADER_MACRO, pinclude : *mut core::ffi::c_void, hlslflags : u32, fxflags : u32, ppcompiledeffect : *mut *mut core::ffi::c_void, pperrors : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3d10")]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateEffectFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : *mut core::ffi::c_void, peffectpool : *mut core::ffi::c_void, ppeffect : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3d10")]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateEffectPoolFromMemory(pdata : *const core::ffi::c_void, datalength : usize, fxflags : u32, pdevice : *mut core::ffi::c_void, ppeffectpool : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3d10")]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateStateBlock(pdevice : *mut core::ffi::c_void, pstateblockmask : *const D3D10_STATE_BLOCK_MASK, ppstateblock : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10DisassembleEffect(peffect : *mut core::ffi::c_void, enablecolorcode : windows_sys::core::BOOL, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDifference(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskDisableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableAll(pmask : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskEnableCapture(pmask : *mut D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, rangestart : u32, rangelength : u32) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskGetSetting(pmask : *const D3D10_STATE_BLOCK_MASK, statetype : D3D10_DEVICE_STATE_TYPES, entry : u32) -> windows_sys::core::BOOL);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskIntersect(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
windows_link::link!("d3d10.dll" "system" fn D3D10StateBlockMaskUnion(pa : *const D3D10_STATE_BLOCK_MASK, pb : *const D3D10_STATE_BLOCK_MASK, presult : *mut D3D10_STATE_BLOCK_MASK) -> windows_sys::core::HRESULT);
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
#[derive(Clone, Copy, Default)]
pub struct D3D10_EFFECT_DESC {
    pub IsChildEffect: windows_sys::core::BOOL,
    pub ConstantBuffers: u32,
    pub SharedConstantBuffers: u32,
    pub GlobalVariables: u32,
    pub SharedGlobalVariables: u32,
    pub Techniques: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_EFFECT_SHADER_DESC {
    pub pInputSignature: *const u8,
    pub IsInline: windows_sys::core::BOOL,
    pub pBytecode: *const u8,
    pub BytecodeLength: u32,
    pub SODecl: windows_sys::core::PCSTR,
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
#[derive(Clone, Copy)]
pub struct D3D10_EFFECT_TYPE_DESC {
    pub TypeName: windows_sys::core::PCSTR,
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
#[cfg(all(feature = "Win32_d3d10shader", feature = "Win32_d3dcommon"))]
impl Default for D3D10_EFFECT_TYPE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_VARIABLE_ANNOTATION: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_EFFECT_VARIABLE_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Semantic: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub Annotations: u32,
    pub BufferOffset: u32,
    pub ExplicitBindPoint: u32,
}
impl Default for D3D10_EFFECT_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_EFFECT_VARIABLE_EXPLICIT_BIND_POINT: u32 = 4;
pub const D3D10_EFFECT_VARIABLE_POOLED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D10_PASS_DESC {
    pub Name: windows_sys::core::PCSTR,
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
#[derive(Clone, Copy)]
pub struct D3D10_PASS_SHADER_DESC {
    pub pShaderVariable: *mut core::ffi::c_void,
    pub ShaderIndex: u32,
}
impl Default for D3D10_PASS_SHADER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct D3D10_TECHNIQUE_DESC {
    pub Name: windows_sys::core::PCSTR,
    pub Passes: u32,
    pub Annotations: u32,
}
impl Default for D3D10_TECHNIQUE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
