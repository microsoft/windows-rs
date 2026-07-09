#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_QUERY_DESC1 {
    pub Base: D3D11_QUERY_DESC1,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct CD3D11_TEXTURE2D_DESC1 {
    pub Base: D3D11_TEXTURE2D_DESC1,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_QUERY_DESC1 {
    pub Query: super::d3d11::D3D11_QUERY,
    pub MiscFlags: u32,
    pub ContextType: D3D11_CONTEXT_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_RASTERIZER_DESC2 {
    pub FillMode: super::d3d11::D3D11_FILL_MODE,
    pub CullMode: super::d3d11::D3D11_CULL_MODE,
    pub FrontCounterClockwise: windows_sys::core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_sys::core::BOOL,
    pub ScissorEnable: windows_sys::core::BOOL,
    pub MultisampleEnable: windows_sys::core::BOOL,
    pub AntialiasedLineEnable: windows_sys::core::BOOL,
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_ARRAY_RTV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_ARRAY_UAV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_RTV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TEX2D_UAV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
