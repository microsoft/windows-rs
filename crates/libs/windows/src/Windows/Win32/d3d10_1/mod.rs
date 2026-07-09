#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10misc", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn D3D10CreateDevice1<P0>(padapter: P0, drivertype: super::d3d10misc::D3D10_DRIVER_TYPE, software: super::minwindef::HMODULE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, ppdevice: Option<*mut Option<ID3D10Device1>>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d10_1.dll" "system" fn D3D10CreateDevice1(padapter : *mut core::ffi::c_void, drivertype : super::d3d10misc::D3D10_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CreateDevice1(padapter.param().abi(), drivertype, software, flags, hardwarelevel, sdkversion, ppdevice.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10misc", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain1<P0>(padapter: P0, drivertype: super::d3d10misc::D3D10_DRIVER_TYPE, software: super::minwindef::HMODULE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: Option<*const super::dxgi::DXGI_SWAP_CHAIN_DESC>, ppswapchain: Option<*mut Option<super::dxgi::IDXGISwapChain>>, ppdevice: Option<*mut Option<ID3D10Device1>>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d10_1.dll" "system" fn D3D10CreateDeviceAndSwapChain1(padapter : *mut core::ffi::c_void, drivertype : super::d3d10misc::D3D10_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, hardwarelevel : D3D10_FEATURE_LEVEL1, sdkversion : u32, pswapchaindesc : *const super::dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CreateDeviceAndSwapChain1(padapter.param().abi(), drivertype, software, flags, hardwarelevel, sdkversion, pswapchaindesc.unwrap_or(core::mem::zeroed()) as _, ppswapchain.unwrap_or(core::mem::zeroed()) as _, ppdevice.unwrap_or(core::mem::zeroed()) as _) }
}
pub const D3D10_1_DEFAULT_SAMPLE_MASK: u32 = 4294967295;
pub const D3D10_1_GS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D10_1_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128;
pub const D3D10_1_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32;
pub const D3D10_1_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1;
pub const D3D10_1_SDK_VERSION: u32 = 32;
pub const D3D10_1_SHADER_MAJOR_VERSION: u32 = 4;
pub const D3D10_1_SHADER_MINOR_VERSION: u32 = 1;
pub const D3D10_1_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048;
pub const D3D10_1_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 256;
pub const D3D10_1_SO_BUFFER_SLOT_COUNT: u32 = 4;
pub const D3D10_1_SO_MULTIPLE_BUFFER_ELEMENTS_PER_BUFFER: u32 = 1;
pub const D3D10_1_SO_SINGLE_BUFFER_COMPONENT_LIMIT: u32 = 64;
pub const D3D10_1_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32;
pub const D3D10_1_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8;
pub const D3D10_1_VS_INPUT_REGISTER_COUNT: u32 = 32;
pub const D3D10_1_VS_OUTPUT_REGISTER_COUNT: u32 = 32;
#[repr(C)]
#[cfg(feature = "Win32_d3d10")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D10_BLEND_DESC1 {
    pub AlphaToCoverageEnable: windows_core::BOOL,
    pub IndependentBlendEnable: windows_core::BOOL,
    pub RenderTarget: [D3D10_RENDER_TARGET_BLEND_DESC1; 8],
}
#[cfg(feature = "Win32_d3d10")]
impl Default for D3D10_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D10_CENTER_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -2;
pub type D3D10_FEATURE_LEVEL1 = i32;
pub const D3D10_FEATURE_LEVEL_10_0: D3D10_FEATURE_LEVEL1 = 40960;
pub const D3D10_FEATURE_LEVEL_10_1: D3D10_FEATURE_LEVEL1 = 41216;
pub const D3D10_FEATURE_LEVEL_9_1: D3D10_FEATURE_LEVEL1 = 37120;
pub const D3D10_FEATURE_LEVEL_9_2: D3D10_FEATURE_LEVEL1 = 37376;
pub const D3D10_FEATURE_LEVEL_9_3: D3D10_FEATURE_LEVEL1 = 37632;
#[repr(C)]
#[cfg(feature = "Win32_d3d10")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: windows_core::BOOL,
    pub SrcBlend: super::d3d10::D3D10_BLEND,
    pub DestBlend: super::d3d10::D3D10_BLEND,
    pub BlendOp: super::d3d10::D3D10_BLEND_OP,
    pub SrcBlendAlpha: super::d3d10::D3D10_BLEND,
    pub DestBlendAlpha: super::d3d10::D3D10_BLEND,
    pub BlendOpAlpha: super::d3d10::D3D10_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ViewDimension: D3D10_SRV_DIMENSION1,
    pub Anonymous: D3D10_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub union D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: super::d3d10::D3D10_BUFFER_SRV,
    pub Texture1D: super::d3d10::D3D10_TEX1D_SRV,
    pub Texture1DArray: super::d3d10::D3D10_TEX1D_ARRAY_SRV,
    pub Texture2D: super::d3d10::D3D10_TEX2D_SRV,
    pub Texture2DArray: super::d3d10::D3D10_TEX2D_ARRAY_SRV,
    pub Texture2DMS: super::d3d10::D3D10_TEX2DMS_SRV,
    pub Texture2DMSArray: super::d3d10::D3D10_TEX2DMS_ARRAY_SRV,
    pub Texture3D: super::d3d10::D3D10_TEX3D_SRV,
    pub TextureCube: super::d3d10::D3D10_TEXCUBE_SRV,
    pub TextureCubeArray: D3D10_TEXCUBE_ARRAY_SRV1,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl Default for D3D10_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_d3dcommon")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D10_SRV_DIMENSION1(pub super::d3dcommon::D3D_SRV_DIMENSION);
pub const D3D10_STANDARD_MULTISAMPLE_PATTERN: D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = -1;
pub type D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D10_TEXCUBE_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
#[cfg(feature = "Win32_d3d10")]
windows_core::imp::define_interface!(ID3D10BlendState1, ID3D10BlendState1_Vtbl, 0xedad8d99_8a35_4d6d_8566_2ea276cde161);
#[cfg(feature = "Win32_d3d10")]
impl core::ops::Deref for ID3D10BlendState1 {
    type Target = super::d3d10::ID3D10BlendState;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d10")]
windows_core::imp::interface_hierarchy!(ID3D10BlendState1, windows_core::IUnknown, super::d3d10::ID3D10DeviceChild, super::d3d10::ID3D10BlendState);
#[cfg(feature = "Win32_d3d10")]
impl ID3D10BlendState1 {
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D10_BLEND_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10BlendState1_Vtbl {
    pub base__: super::d3d10::ID3D10BlendState_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_BLEND_DESC1),
}
#[cfg(feature = "Win32_d3d10")]
pub trait ID3D10BlendState1_Impl: super::d3d10::ID3D10BlendState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D10_BLEND_DESC1);
}
#[cfg(feature = "Win32_d3d10")]
impl ID3D10BlendState1_Vtbl {
    pub const fn new<Identity: ID3D10BlendState1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D10BlendState1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10BlendState1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: super::d3d10::ID3D10BlendState_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10BlendState1 as windows_core::Interface>::IID || iid == &<super::d3d10::ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d10::ID3D10BlendState as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d10")]
impl windows_core::RuntimeName for ID3D10BlendState1 {}
#[cfg(feature = "Win32_d3d10")]
windows_core::imp::define_interface!(ID3D10Device1, ID3D10Device1_Vtbl, 0x9b7e4c8f_342c_4106_a19f_4f2704f689f0);
#[cfg(feature = "Win32_d3d10")]
impl core::ops::Deref for ID3D10Device1 {
    type Target = super::d3d10::ID3D10Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d10")]
windows_core::imp::interface_hierarchy!(ID3D10Device1, windows_core::IUnknown, super::d3d10::ID3D10Device);
#[cfg(feature = "Win32_d3d10")]
impl ID3D10Device1 {
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateShaderResourceView1<P0>(&self, presource: P0, pdesc: Option<*const D3D10_SHADER_RESOURCE_VIEW_DESC1>, ppsrview: Option<*mut Option<ID3D10ShaderResourceView1>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d10::ID3D10Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateShaderResourceView1)(windows_core::Interface::as_raw(self), presource.param().abi(), pdesc.unwrap_or(core::mem::zeroed()) as _, ppsrview.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: Option<*mut Option<ID3D10BlendState1>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateBlendState1)(windows_core::Interface::as_raw(self), pblendstatedesc, ppblendstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1 {
        unsafe { (windows_core::Interface::vtable(self).GetFeatureLevel)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d3d10")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10Device1_Vtbl {
    pub base__: super::d3d10::ID3D10Device_Vtbl,
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub CreateShaderResourceView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D10_SHADER_RESOURCE_VIEW_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat")))]
    CreateShaderResourceView1: usize,
    pub CreateBlendState1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D10_BLEND_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D10_FEATURE_LEVEL1,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait ID3D10Device1_Impl: super::d3d10::ID3D10Device_Impl {
    fn CreateShaderResourceView1(&self, presource: windows_core::Ref<super::d3d10::ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: windows_core::OutRef<ID3D10ShaderResourceView1>) -> windows_core::Result<()>;
    fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: windows_core::OutRef<ID3D10BlendState1>) -> windows_core::Result<()>;
    fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1;
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl ID3D10Device1_Vtbl {
    pub const fn new<Identity: ID3D10Device1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateShaderResourceView1<Identity: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device1_Impl::CreateShaderResourceView1(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppsrview)).into()
            }
        }
        unsafe extern "system" fn CreateBlendState1<Identity: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device1_Impl::CreateBlendState1(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
            }
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: ID3D10Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D10_FEATURE_LEVEL1 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10Device1_Impl::GetFeatureLevel(this)
            }
        }
        Self {
            base__: super::d3d10::ID3D10Device_Vtbl::new::<Identity, OFFSET>(),
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Device1 as windows_core::Interface>::IID || iid == &<super::d3d10::ID3D10Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D10Device1 {}
#[cfg(feature = "Win32_d3d10")]
windows_core::imp::define_interface!(ID3D10ShaderResourceView1, ID3D10ShaderResourceView1_Vtbl, 0x9b7e4c87_342c_4106_a19f_4f2704f689f0);
#[cfg(feature = "Win32_d3d10")]
impl core::ops::Deref for ID3D10ShaderResourceView1 {
    type Target = super::d3d10::ID3D10ShaderResourceView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d10")]
windows_core::imp::interface_hierarchy!(ID3D10ShaderResourceView1, windows_core::IUnknown, super::d3d10::ID3D10DeviceChild, super::d3d10::ID3D10View, super::d3d10::ID3D10ShaderResourceView);
#[cfg(feature = "Win32_d3d10")]
impl ID3D10ShaderResourceView1 {
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[cfg(feature = "Win32_d3d10")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D10ShaderResourceView1_Vtbl {
    pub base__: super::d3d10::ID3D10ShaderResourceView_Vtbl,
    #[cfg(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D10_SHADER_RESOURCE_VIEW_DESC1),
    #[cfg(not(all(feature = "Win32_d3dcommon", feature = "Win32_dxgiformat")))]
    GetDesc1: usize,
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
pub trait ID3D10ShaderResourceView1_Impl: super::d3d10::ID3D10ShaderResourceView_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl ID3D10ShaderResourceView1_Vtbl {
    pub const fn new<Identity: ID3D10ShaderResourceView1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D10ShaderResourceView1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D10ShaderResourceView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: super::d3d10::ID3D10ShaderResourceView_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView1 as windows_core::Interface>::IID || iid == &<super::d3d10::ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d10::ID3D10View as windows_core::Interface>::IID || iid == &<super::d3d10::ID3D10ShaderResourceView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D10ShaderResourceView1 {}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10misc", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
pub type PFN_D3D10_CREATE_DEVICE1 = Option<unsafe extern "system" fn(param0: windows_core::Ref<super::dxgi::IDXGIAdapter>, param1: super::d3d10misc::D3D10_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: windows_core::OutRef<ID3D10Device1>) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_d3d10misc", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type PFN_D3D10_CREATE_DEVICE_AND_SWAP_CHAIN1 = Option<unsafe extern "system" fn(param0: windows_core::Ref<super::dxgi::IDXGIAdapter>, param1: super::d3d10misc::D3D10_DRIVER_TYPE, param2: super::minwindef::HMODULE, param3: u32, param4: D3D10_FEATURE_LEVEL1, param5: u32, param6: *mut super::dxgi::DXGI_SWAP_CHAIN_DESC, param7: windows_core::OutRef<super::dxgi::IDXGISwapChain>, param8: windows_core::OutRef<ID3D10Device1>) -> windows_core::HRESULT>;
