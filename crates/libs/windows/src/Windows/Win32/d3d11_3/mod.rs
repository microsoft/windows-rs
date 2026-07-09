#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_QUERY_DESC1 {
    pub Base: D3D11_QUERY_DESC1,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_RASTERIZER_DESC2 {
    pub Base: D3D11_RASTERIZER_DESC2,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct CD3D11_RENDER_TARGET_VIEW_DESC1 {
    pub Base: D3D11_RENDER_TARGET_VIEW_DESC1,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl Default for CD3D11_RENDER_TARGET_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct CD3D11_SHADER_RESOURCE_VIEW_DESC1 {
    pub Base: D3D11_SHADER_RESOURCE_VIEW_DESC1,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for CD3D11_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_TEXTURE2D_DESC1 {
    pub Base: D3D11_TEXTURE2D_DESC1,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_TEXTURE3D_DESC1 {
    pub Base: D3D11_TEXTURE3D_DESC1,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct CD3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    pub Base: D3D11_UNORDERED_ACCESS_VIEW_DESC1,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl Default for CD3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_CONSERVATIVE_RASTERIZATION_MODE = i32;
pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_OFF: D3D11_CONSERVATIVE_RASTERIZATION_MODE = 0;
pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_ON: D3D11_CONSERVATIVE_RASTERIZATION_MODE = 1;
pub type D3D11_CONTEXT_TYPE = i32;
pub const D3D11_CONTEXT_TYPE_3D: D3D11_CONTEXT_TYPE = 1;
pub const D3D11_CONTEXT_TYPE_ALL: D3D11_CONTEXT_TYPE = 0;
pub const D3D11_CONTEXT_TYPE_COMPUTE: D3D11_CONTEXT_TYPE = 2;
pub const D3D11_CONTEXT_TYPE_COPY: D3D11_CONTEXT_TYPE = 3;
pub const D3D11_CONTEXT_TYPE_VIDEO: D3D11_CONTEXT_TYPE = 4;
pub type D3D11_FENCE_FLAG = u32;
pub const D3D11_FENCE_FLAG_NONE: D3D11_FENCE_FLAG = 0;
pub const D3D11_FENCE_FLAG_NON_MONITORED: D3D11_FENCE_FLAG = 8;
pub const D3D11_FENCE_FLAG_SHARED: D3D11_FENCE_FLAG = 2;
pub const D3D11_FENCE_FLAG_SHARED_CROSS_ADAPTER: D3D11_FENCE_FLAG = 4;
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_QUERY_DESC1 {
    pub Query: super::d3d11::D3D11_QUERY,
    pub MiscFlags: u32,
    pub ContextType: D3D11_CONTEXT_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RASTERIZER_DESC2 {
    pub FillMode: super::d3d11::D3D11_FILL_MODE,
    pub CullMode: super::d3d11::D3D11_CULL_MODE,
    pub FrontCounterClockwise: windows_core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_core::BOOL,
    pub ScissorEnable: windows_core::BOOL,
    pub MultisampleEnable: windows_core::BOOL,
    pub AntialiasedLineEnable: windows_core::BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D11_CONSERVATIVE_RASTERIZATION_MODE,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC1 {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: super::d3d11::D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub union D3D11_RENDER_TARGET_VIEW_DESC1_0 {
    pub Buffer: super::d3d11::D3D11_BUFFER_RTV,
    pub Texture1D: super::d3d11::D3D11_TEX1D_RTV,
    pub Texture1DArray: super::d3d11::D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV1,
    pub Texture2DMS: super::d3d11::D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: super::d3d11::D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: super::d3d11::D3D11_TEX3D_RTV,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl Default for D3D11_RENDER_TARGET_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: super::d3d11::D3D11_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: super::d3d11::D3D11_BUFFER_SRV,
    pub Texture1D: super::d3d11::D3D11_TEX1D_SRV,
    pub Texture1DArray: super::d3d11::D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV1,
    pub Texture2DMS: super::d3d11::D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: super::d3d11::D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: super::d3d11::D3D11_TEX3D_SRV,
    pub TextureCube: super::d3d11::D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: super::d3d11::D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: super::d3d11::D3D11_BUFFEREX_SRV,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_RTV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_UAV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_RTV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEX2D_UAV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEXTURE2D_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub SampleDesc: super::dxgicommon::DXGI_SAMPLE_DESC,
    pub Usage: super::d3d11::D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub TextureLayout: D3D11_TEXTURE_LAYOUT,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TEXTURE3D_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub Usage: super::d3d11::D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub TextureLayout: D3D11_TEXTURE_LAYOUT,
}
pub type D3D11_TEXTURE_LAYOUT = i32;
pub const D3D11_TEXTURE_LAYOUT_64K_STANDARD_SWIZZLE: D3D11_TEXTURE_LAYOUT = 2;
pub const D3D11_TEXTURE_LAYOUT_ROW_MAJOR: D3D11_TEXTURE_LAYOUT = 1;
pub const D3D11_TEXTURE_LAYOUT_UNDEFINED: D3D11_TEXTURE_LAYOUT = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: super::d3d11::D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
    pub Buffer: super::d3d11::D3D11_BUFFER_UAV,
    pub Texture1D: super::d3d11::D3D11_TEX1D_UAV,
    pub Texture1DArray: super::d3d11::D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV1,
    pub Texture3D: super::d3d11::D3D11_TEX3D_UAV,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
windows_core::imp::define_interface!(ID3D11Device3, ID3D11Device3_Vtbl, 0xa05c8c37_d2c6_4732_b3a0_9ce0b0dc9ae6);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
impl core::ops::Deref for ID3D11Device3 {
    type Target = super::d3d11_2::ID3D11Device2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
windows_core::imp::interface_hierarchy!(ID3D11Device3, windows_core::IUnknown, super::d3d11::ID3D11Device, super::d3d11_1::ID3D11Device1, super::d3d11_2::ID3D11Device2);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
impl ID3D11Device3 {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateTexture2D1(&self, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: Option<*const super::d3d11::D3D11_SUBRESOURCE_DATA>, pptexture2d: Option<*mut Option<ID3D11Texture2D1>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture2D1)(windows_core::Interface::as_raw(self), pdesc1, pinitialdata.unwrap_or(core::mem::zeroed()) as _, pptexture2d.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateTexture3D1(&self, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: Option<*const super::d3d11::D3D11_SUBRESOURCE_DATA>, pptexture3d: Option<*mut Option<ID3D11Texture3D1>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture3D1)(windows_core::Interface::as_raw(self), pdesc1, pinitialdata.unwrap_or(core::mem::zeroed()) as _, pptexture3d.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateRasterizerState2(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: Option<*mut Option<ID3D11RasterizerState2>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRasterizerState2)(windows_core::Interface::as_raw(self), prasterizerdesc, pprasterizerstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateShaderResourceView1<P0>(&self, presource: P0, pdesc1: Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC1>, ppsrview1: Option<*mut Option<ID3D11ShaderResourceView1>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateShaderResourceView1)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc1.unwrap_or(core::mem::zeroed()) as _, ppsrview1.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateUnorderedAccessView1<P0>(&self, presource: P0, pdesc1: Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC1>, ppuaview1: Option<*mut Option<ID3D11UnorderedAccessView1>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateUnorderedAccessView1)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc1.unwrap_or(core::mem::zeroed()) as _, ppuaview1.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateRenderTargetView1<P0>(&self, presource: P0, pdesc1: Option<*const D3D11_RENDER_TARGET_VIEW_DESC1>, pprtview1: Option<*mut Option<ID3D11RenderTargetView1>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTargetView1)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc1.unwrap_or(core::mem::zeroed()) as _, pprtview1.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateQuery1(&self, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: Option<*mut Option<ID3D11Query1>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateQuery1)(windows_core::Interface::as_raw(self), pquerydesc1, ppquery1.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetImmediateContext3(&self) -> windows_core::Result<ID3D11DeviceContext3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmediateContext3)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn CreateDeferredContext3(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext3>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDeferredContext3)(windows_core::Interface::as_raw(self), contextflags, ppdeferredcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn WriteToSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const super::d3d11::D3D11_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).WriteToSubresource)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, pdstbox.unwrap_or(core::mem::zeroed()) as _, psrcdata, srcrowpitch, srcdepthpitch);
        }
    }
    pub unsafe fn ReadFromSubresource<P3>(&self, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: P3, srcsubresource: u32, psrcbox: Option<*const super::d3d11::D3D11_BOX>)
    where
        P3: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ReadFromSubresource)(windows_core::Interface::as_raw(self), pdstdata as _, dstrowpitch, dstdepthpitch, psrcresource.param().abi(), srcsubresource, psrcbox.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Device3_Vtbl {
    pub base__: super::d3d11_2::ID3D11Device2_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateTexture2D1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE2D_DESC1, *const super::d3d11::D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateTexture2D1: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateTexture3D1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_TEXTURE3D_DESC1, *const super::d3d11::D3D11_SUBRESOURCE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateTexture3D1: usize,
    pub CreateRasterizerState2: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_RASTERIZER_DESC2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub CreateShaderResourceView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_SHADER_RESOURCE_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat")))]
    CreateShaderResourceView1: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateUnorderedAccessView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateUnorderedAccessView1: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateRenderTargetView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_RENDER_TARGET_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateRenderTargetView1: usize,
    pub CreateQuery1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_QUERY_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImmediateContext3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub CreateDeferredContext3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteToSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const super::d3d11::D3D11_BOX, *const core::ffi::c_void, u32, u32),
    pub ReadFromSubresource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, u32, *const super::d3d11::D3D11_BOX),
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait ID3D11Device3_Impl: super::d3d11_2::ID3D11Device2_Impl {
    fn CreateTexture2D1(&self, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const super::d3d11::D3D11_SUBRESOURCE_DATA, pptexture2d: windows_core::OutRef<ID3D11Texture2D1>) -> windows_core::Result<()>;
    fn CreateTexture3D1(&self, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const super::d3d11::D3D11_SUBRESOURCE_DATA, pptexture3d: windows_core::OutRef<ID3D11Texture3D1>) -> windows_core::Result<()>;
    fn CreateRasterizerState2(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: windows_core::OutRef<ID3D11RasterizerState2>) -> windows_core::Result<()>;
    fn CreateShaderResourceView1(&self, presource: windows_core::Ref<super::d3d11::ID3D11Resource>, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: windows_core::OutRef<ID3D11ShaderResourceView1>) -> windows_core::Result<()>;
    fn CreateUnorderedAccessView1(&self, presource: windows_core::Ref<super::d3d11::ID3D11Resource>, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: windows_core::OutRef<ID3D11UnorderedAccessView1>) -> windows_core::Result<()>;
    fn CreateRenderTargetView1(&self, presource: windows_core::Ref<super::d3d11::ID3D11Resource>, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: windows_core::OutRef<ID3D11RenderTargetView1>) -> windows_core::Result<()>;
    fn CreateQuery1(&self, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: windows_core::OutRef<ID3D11Query1>) -> windows_core::Result<()>;
    fn GetImmediateContext3(&self, ppimmediatecontext: windows_core::OutRef<ID3D11DeviceContext3>);
    fn CreateDeferredContext3(&self, contextflags: u32, ppdeferredcontext: windows_core::OutRef<ID3D11DeviceContext3>) -> windows_core::Result<()>;
    fn WriteToSubresource(&self, pdstresource: windows_core::Ref<super::d3d11::ID3D11Resource>, dstsubresource: u32, pdstbox: *const super::d3d11::D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ReadFromSubresource(&self, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: windows_core::Ref<super::d3d11::ID3D11Resource>, srcsubresource: u32, psrcbox: *const super::d3d11::D3D11_BOX);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl ID3D11Device3_Vtbl {
    pub const fn new<Identity: ID3D11Device3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTexture2D1<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const super::d3d11::D3D11_SUBRESOURCE_DATA, pptexture2d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::CreateTexture2D1(this, core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture2d)).into()
            }
        }
        unsafe extern "system" fn CreateTexture3D1<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const super::d3d11::D3D11_SUBRESOURCE_DATA, pptexture3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::CreateTexture3D1(this, core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture3d)).into()
            }
        }
        unsafe extern "system" fn CreateRasterizerState2<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::CreateRasterizerState2(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
            }
        }
        unsafe extern "system" fn CreateShaderResourceView1<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::CreateShaderResourceView1(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&ppsrview1)).into()
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView1<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::CreateUnorderedAccessView1(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&ppuaview1)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetView1<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::CreateRenderTargetView1(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&pprtview1)).into()
            }
        }
        unsafe extern "system" fn CreateQuery1<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::CreateQuery1(this, core::mem::transmute_copy(&pquerydesc1), core::mem::transmute_copy(&ppquery1)).into()
            }
        }
        unsafe extern "system" fn GetImmediateContext3<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimmediatecontext: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::GetImmediateContext3(this, core::mem::transmute_copy(&ppimmediatecontext));
            }
        }
        unsafe extern "system" fn CreateDeferredContext3<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::CreateDeferredContext3(this, core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&ppdeferredcontext)).into()
            }
        }
        unsafe extern "system" fn WriteToSubresource<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const super::d3d11::D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::WriteToSubresource(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch));
            }
        }
        unsafe extern "system" fn ReadFromSubresource<Identity: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const super::d3d11::D3D11_BOX) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device3_Impl::ReadFromSubresource(this, core::mem::transmute_copy(&pdstdata), core::mem::transmute_copy(&dstrowpitch), core::mem::transmute_copy(&dstdepthpitch), core::mem::transmute_copy(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox));
            }
        }
        Self {
            base__: super::d3d11_2::ID3D11Device2_Vtbl::new::<Identity, OFFSET>(),
            CreateTexture2D1: CreateTexture2D1::<Identity, OFFSET>,
            CreateTexture3D1: CreateTexture3D1::<Identity, OFFSET>,
            CreateRasterizerState2: CreateRasterizerState2::<Identity, OFFSET>,
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, OFFSET>,
            CreateUnorderedAccessView1: CreateUnorderedAccessView1::<Identity, OFFSET>,
            CreateRenderTargetView1: CreateRenderTargetView1::<Identity, OFFSET>,
            CreateQuery1: CreateQuery1::<Identity, OFFSET>,
            GetImmediateContext3: GetImmediateContext3::<Identity, OFFSET>,
            CreateDeferredContext3: CreateDeferredContext3::<Identity, OFFSET>,
            WriteToSubresource: WriteToSubresource::<Identity, OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device3 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Device as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11Device1 as windows_core::Interface>::IID || iid == &<super::d3d11_2::ID3D11Device2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11Device3 {}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
windows_core::imp::define_interface!(ID3D11DeviceContext3, ID3D11DeviceContext3_Vtbl, 0xb4e3c01d_e79e_4637_91b2_510e9f4c9b8f);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
impl core::ops::Deref for ID3D11DeviceContext3 {
    type Target = super::d3d11_2::ID3D11DeviceContext2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext3, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11DeviceContext, super::d3d11_1::ID3D11DeviceContext1, super::d3d11_2::ID3D11DeviceContext2);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
impl ID3D11DeviceContext3 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn Flush1(&self, contexttype: D3D11_CONTEXT_TYPE, hevent: Option<super::winnt::HANDLE>) {
        unsafe {
            (windows_core::Interface::vtable(self).Flush1)(windows_core::Interface::as_raw(self), contexttype, hevent.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn SetHardwareProtectionState(&self, hwprotectionenable: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).SetHardwareProtectionState)(windows_core::Interface::as_raw(self), hwprotectionenable.into());
        }
    }
    pub unsafe fn GetHardwareProtectionState(&self) -> windows_core::BOOL {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHardwareProtectionState)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceContext3_Vtbl {
    pub base__: super::d3d11_2::ID3D11DeviceContext2_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub Flush1: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_CONTEXT_TYPE, super::winnt::HANDLE),
    #[cfg(not(feature = "Win32_winnt"))]
    Flush1: usize,
    pub SetHardwareProtectionState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
    pub GetHardwareProtectionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL),
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait ID3D11DeviceContext3_Impl: super::d3d11_2::ID3D11DeviceContext2_Impl {
    fn Flush1(&self, contexttype: D3D11_CONTEXT_TYPE, hevent: super::winnt::HANDLE);
    fn SetHardwareProtectionState(&self, hwprotectionenable: windows_core::BOOL);
    fn GetHardwareProtectionState(&self, phwprotectionenable: *mut windows_core::BOOL);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl ID3D11DeviceContext3_Vtbl {
    pub const fn new<Identity: ID3D11DeviceContext3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Flush1<Identity: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contexttype: D3D11_CONTEXT_TYPE, hevent: super::winnt::HANDLE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext3_Impl::Flush1(this, core::mem::transmute_copy(&contexttype), core::mem::transmute_copy(&hevent));
            }
        }
        unsafe extern "system" fn SetHardwareProtectionState<Identity: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwprotectionenable: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext3_Impl::SetHardwareProtectionState(this, core::mem::transmute_copy(&hwprotectionenable));
            }
        }
        unsafe extern "system" fn GetHardwareProtectionState<Identity: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwprotectionenable: *mut windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext3_Impl::GetHardwareProtectionState(this, core::mem::transmute_copy(&phwprotectionenable));
            }
        }
        Self {
            base__: super::d3d11_2::ID3D11DeviceContext2_Vtbl::new::<Identity, OFFSET>(),
            Flush1: Flush1::<Identity, OFFSET>,
            SetHardwareProtectionState: SetHardwareProtectionState::<Identity, OFFSET>,
            GetHardwareProtectionState: GetHardwareProtectionState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext3 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceContext as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11DeviceContext1 as windows_core::Interface>::IID || iid == &<super::d3d11_2::ID3D11DeviceContext2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11DeviceContext3 {}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
windows_core::imp::define_interface!(ID3D11DeviceContext4, ID3D11DeviceContext4_Vtbl, 0x917600da_f58c_4c33_98d8_3e15b390fa24);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
impl core::ops::Deref for ID3D11DeviceContext4 {
    type Target = ID3D11DeviceContext3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext4, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11DeviceContext, super::d3d11_1::ID3D11DeviceContext1, super::d3d11_2::ID3D11DeviceContext2, ID3D11DeviceContext3);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
impl ID3D11DeviceContext4 {
    pub unsafe fn Signal<P0>(&self, pfence: P0, value: u64) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Fence>,
    {
        unsafe { (windows_core::Interface::vtable(self).Signal)(windows_core::Interface::as_raw(self), pfence.param().abi(), value) }
    }
    pub unsafe fn Wait<P0>(&self, pfence: P0, value: u64) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Fence>,
    {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), pfence.param().abi(), value) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceContext4_Vtbl {
    pub base__: ID3D11DeviceContext3_Vtbl,
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait ID3D11DeviceContext4_Impl: ID3D11DeviceContext3_Impl {
    fn Signal(&self, pfence: windows_core::Ref<ID3D11Fence>, value: u64) -> windows_core::Result<()>;
    fn Wait(&self, pfence: windows_core::Ref<ID3D11Fence>, value: u64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl ID3D11DeviceContext4_Vtbl {
    pub const fn new<Identity: ID3D11DeviceContext4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Signal<Identity: ID3D11DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfence: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext4_Impl::Signal(this, core::mem::transmute_copy(&pfence), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Wait<Identity: ID3D11DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfence: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext4_Impl::Wait(this, core::mem::transmute_copy(&pfence), core::mem::transmute_copy(&value)).into()
            }
        }
        Self { base__: ID3D11DeviceContext3_Vtbl::new::<Identity, OFFSET>(), Signal: Signal::<Identity, OFFSET>, Wait: Wait::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext4 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceContext as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11DeviceContext1 as windows_core::Interface>::IID || iid == &<super::d3d11_2::ID3D11DeviceContext2 as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11DeviceContext4 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11Fence, ID3D11Fence_Vtbl, 0xaffde9d1_1df7_4bb7_8a34_0f46251dab80);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11Fence {
    type Target = super::d3d11::ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11Fence, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11Fence {
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
    pub unsafe fn CreateSharedHandle<P2>(&self, pattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, dwaccess: u32, lpname: P2) -> windows_core::Result<super::winnt::HANDLE>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSharedHandle)(windows_core::Interface::as_raw(self), pattributes.unwrap_or(core::mem::zeroed()) as _, dwaccess, lpname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCompletedValue(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetCompletedValue)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn SetEventOnCompletion(&self, value: u64, hevent: super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventOnCompletion)(windows_core::Interface::as_raw(self), value, hevent) }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Fence_Vtbl {
    pub base__: super::d3d11::ID3D11DeviceChild_Vtbl,
    #[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
    pub CreateSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::minwinbase::SECURITY_ATTRIBUTES, u32, windows_core::PCWSTR, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwinbase", feature = "Win32_winnt")))]
    CreateSharedHandle: usize,
    pub GetCompletedValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    #[cfg(feature = "Win32_winnt")]
    pub SetEventOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    SetEventOnCompletion: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
pub trait ID3D11Fence_Impl: super::d3d11::ID3D11DeviceChild_Impl {
    fn CreateSharedHandle(&self, pattributes: *const super::minwinbase::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &windows_core::PCWSTR) -> windows_core::Result<super::winnt::HANDLE>;
    fn GetCompletedValue(&self) -> u64;
    fn SetEventOnCompletion(&self, value: u64, hevent: super::winnt::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
impl ID3D11Fence_Vtbl {
    pub const fn new<Identity: ID3D11Fence_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSharedHandle<Identity: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *const super::minwinbase::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: windows_core::PCWSTR, phandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Fence_Impl::CreateSharedHandle(this, core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&dwaccess), core::mem::transmute(&lpname)) {
                    Ok(ok__) => {
                        phandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCompletedValue<Identity: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Fence_Impl::GetCompletedValue(this)
            }
        }
        unsafe extern "system" fn SetEventOnCompletion<Identity: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, hevent: super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Fence_Impl::SetEventOnCompletion(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&hevent)).into()
            }
        }
        Self {
            base__: super::d3d11::ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            CreateSharedHandle: CreateSharedHandle::<Identity, OFFSET>,
            GetCompletedValue: GetCompletedValue::<Identity, OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Fence as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11Fence {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11Query1, ID3D11Query1_Vtbl, 0x631b4766_36dc_461d_8db6_c47e13e60916);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11Query1 {
    type Target = super::d3d11::ID3D11Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11Query1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11Asynchronous, super::d3d11::ID3D11Query);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11Query1 {
    pub unsafe fn GetDesc1(&self) -> D3D11_QUERY_DESC1 {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Query1_Vtbl {
    pub base__: super::d3d11::ID3D11Query_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_QUERY_DESC1),
}
#[cfg(feature = "Win32_d3d11")]
pub trait ID3D11Query1_Impl: super::d3d11::ID3D11Query_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_QUERY_DESC1);
}
#[cfg(feature = "Win32_d3d11")]
impl ID3D11Query1_Vtbl {
    pub const fn new<Identity: ID3D11Query1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D11Query1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *mut D3D11_QUERY_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Query1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc1));
            }
        }
        Self { base__: super::d3d11::ID3D11Query_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Query1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Asynchronous as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Query as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d11")]
impl windows_core::RuntimeName for ID3D11Query1 {}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::define_interface!(ID3D11RasterizerState2, ID3D11RasterizerState2_Vtbl, 0x6fbd02fb_209f_46c4_b059_2ed15586a6ac);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl core::ops::Deref for ID3D11RasterizerState2 {
    type Target = super::d3d11_1::ID3D11RasterizerState1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::interface_hierarchy!(ID3D11RasterizerState2, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11RasterizerState, super::d3d11_1::ID3D11RasterizerState1);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl ID3D11RasterizerState2 {
    pub unsafe fn GetDesc2(&self, pdesc: *mut D3D11_RASTERIZER_DESC2) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc2)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11RasterizerState2_Vtbl {
    pub base__: super::d3d11_1::ID3D11RasterizerState1_Vtbl,
    pub GetDesc2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RASTERIZER_DESC2),
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
pub trait ID3D11RasterizerState2_Impl: super::d3d11_1::ID3D11RasterizerState1_Impl {
    fn GetDesc2(&self, pdesc: *mut D3D11_RASTERIZER_DESC2);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl ID3D11RasterizerState2_Vtbl {
    pub const fn new<Identity: ID3D11RasterizerState2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc2<Identity: ID3D11RasterizerState2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11RasterizerState2_Impl::GetDesc2(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: super::d3d11_1::ID3D11RasterizerState1_Vtbl::new::<Identity, OFFSET>(), GetDesc2: GetDesc2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RasterizerState2 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11RasterizerState as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11RasterizerState1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl windows_core::RuntimeName for ID3D11RasterizerState2 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11RenderTargetView1, ID3D11RenderTargetView1_Vtbl, 0xffbe2e23_f011_418a_ac56_5ceed7c5b94b);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11RenderTargetView1 {
    type Target = super::d3d11::ID3D11RenderTargetView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11RenderTargetView1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11View, super::d3d11::ID3D11RenderTargetView);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11RenderTargetView1 {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc1(&self, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc1 as _);
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11RenderTargetView1_Vtbl {
    pub base__: super::d3d11::ID3D11RenderTargetView_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RENDER_TARGET_VIEW_DESC1),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc1: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
pub trait ID3D11RenderTargetView1_Impl: super::d3d11::ID3D11RenderTargetView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl ID3D11RenderTargetView1_Vtbl {
    pub const fn new<Identity: ID3D11RenderTargetView1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D11RenderTargetView1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11RenderTargetView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc1));
            }
        }
        Self { base__: super::d3d11::ID3D11RenderTargetView_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11View as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11RenderTargetView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11RenderTargetView1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11ShaderResourceView1, ID3D11ShaderResourceView1_Vtbl, 0x91308b87_9040_411d_8c67_c39253ce3802);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11ShaderResourceView1 {
    type Target = super::d3d11::ID3D11ShaderResourceView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11ShaderResourceView1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11View, super::d3d11::ID3D11ShaderResourceView);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11ShaderResourceView1 {
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc1(&self, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc1 as _);
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11ShaderResourceView1_Vtbl {
    pub base__: super::d3d11::ID3D11ShaderResourceView_Vtbl,
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_SHADER_RESOURCE_VIEW_DESC1),
    #[cfg(not(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat")))]
    GetDesc1: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11ShaderResourceView1_Impl: super::d3d11::ID3D11ShaderResourceView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl ID3D11ShaderResourceView1_Vtbl {
    pub const fn new<Identity: ID3D11ShaderResourceView1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D11ShaderResourceView1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11ShaderResourceView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc1));
            }
        }
        Self { base__: super::d3d11::ID3D11ShaderResourceView_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11View as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11ShaderResourceView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11ShaderResourceView1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11Texture2D1, ID3D11Texture2D1_Vtbl, 0x51218251_1e33_4617_9ccb_4d3a4367e7bb);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11Texture2D1 {
    type Target = super::d3d11::ID3D11Texture2D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11Texture2D1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11Resource, super::d3d11::ID3D11Texture2D);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11Texture2D1 {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Texture2D1_Vtbl {
    pub base__: super::d3d11::ID3D11Texture2D_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE2D_DESC1),
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    GetDesc1: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11Texture2D1_Impl: super::d3d11::ID3D11Texture2D_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE2D_DESC1);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D11Texture2D1_Vtbl {
    pub const fn new<Identity: ID3D11Texture2D1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D11Texture2D1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Texture2D1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: super::d3d11::ID3D11Texture2D_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture2D1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Resource as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Texture2D as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11Texture2D1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11Texture3D1, ID3D11Texture3D1_Vtbl, 0x0c711683_2853_4846_9bb0_f3e60639e46a);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11Texture3D1 {
    type Target = super::d3d11::ID3D11Texture3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11Texture3D1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11Resource, super::d3d11::ID3D11Texture3D);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11Texture3D1 {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Texture3D1_Vtbl {
    pub base__: super::d3d11::ID3D11Texture3D_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE3D_DESC1),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc1: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
pub trait ID3D11Texture3D1_Impl: super::d3d11::ID3D11Texture3D_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE3D_DESC1);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl ID3D11Texture3D1_Vtbl {
    pub const fn new<Identity: ID3D11Texture3D1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D11Texture3D1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Texture3D1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: super::d3d11::ID3D11Texture3D_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture3D1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Resource as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Texture3D as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11Texture3D1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11UnorderedAccessView1, ID3D11UnorderedAccessView1_Vtbl, 0x7b3b6153_a886_4544_ab37_6537c8500403);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11UnorderedAccessView1 {
    type Target = super::d3d11::ID3D11UnorderedAccessView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11UnorderedAccessView1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11View, super::d3d11::ID3D11UnorderedAccessView);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11UnorderedAccessView1 {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc1(&self, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc1 as _);
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11UnorderedAccessView1_Vtbl {
    pub base__: super::d3d11::ID3D11UnorderedAccessView_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc1: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
pub trait ID3D11UnorderedAccessView1_Impl: super::d3d11::ID3D11UnorderedAccessView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl ID3D11UnorderedAccessView1_Vtbl {
    pub const fn new<Identity: ID3D11UnorderedAccessView1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D11UnorderedAccessView1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11UnorderedAccessView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc1));
            }
        }
        Self { base__: super::d3d11::ID3D11UnorderedAccessView_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11View as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11UnorderedAccessView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11UnorderedAccessView1 {}
