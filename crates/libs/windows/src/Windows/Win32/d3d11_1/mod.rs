#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_BLEND_DESC1 {
    pub Base: D3D11_BLEND_DESC1,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CD3D11_RASTERIZER_DESC1 {
    pub Base: D3D11_RASTERIZER_DESC1,
}
pub type D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG = i32;
pub const D3D11_1_CREATE_DEVICE_CONTEXT_STATE_SINGLETHREADED: D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG = 1;
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_BLEND_DESC1 {
    pub AlphaToCoverageEnable: windows_core::BOOL,
    pub IndependentBlendEnable: windows_core::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1; 8],
}
#[cfg(feature = "Win32_d3d11")]
impl Default for D3D11_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_COPY_DISCARD: D3D11_COPY_FLAGS = 2;
pub type D3D11_COPY_FLAGS = i32;
pub const D3D11_COPY_NO_OVERWRITE: D3D11_COPY_FLAGS = 1;
pub type D3D11_CRYPTO_SESSION_STATUS = i32;
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_AND_CONTENT_LOST: D3D11_CRYPTO_SESSION_STATUS = 2;
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_LOST: D3D11_CRYPTO_SESSION_STATUS = 1;
pub const D3D11_CRYPTO_SESSION_STATUS_OK: D3D11_CRYPTO_SESSION_STATUS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    pub HWProtectionFunctionID: u32,
    pub pInputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA,
    pub pOutputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA,
    pub Status: windows_core::HRESULT,
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    pub PrivateDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub pbInput: [u8; 4],
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    pub PrivateDataSize: u32,
    pub MaxHWProtectionDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub TransportTime: u64,
    pub ExecutionTime: u64,
    pub pbOutput: [u8; 4],
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_LOGIC_OP = i32;
pub const D3D11_LOGIC_OP_AND: D3D11_LOGIC_OP = 6;
pub const D3D11_LOGIC_OP_AND_INVERTED: D3D11_LOGIC_OP = 13;
pub const D3D11_LOGIC_OP_AND_REVERSE: D3D11_LOGIC_OP = 12;
pub const D3D11_LOGIC_OP_CLEAR: D3D11_LOGIC_OP = 0;
pub const D3D11_LOGIC_OP_COPY: D3D11_LOGIC_OP = 2;
pub const D3D11_LOGIC_OP_COPY_INVERTED: D3D11_LOGIC_OP = 3;
pub const D3D11_LOGIC_OP_EQUIV: D3D11_LOGIC_OP = 11;
pub const D3D11_LOGIC_OP_INVERT: D3D11_LOGIC_OP = 5;
pub const D3D11_LOGIC_OP_NAND: D3D11_LOGIC_OP = 7;
pub const D3D11_LOGIC_OP_NOOP: D3D11_LOGIC_OP = 4;
pub const D3D11_LOGIC_OP_NOR: D3D11_LOGIC_OP = 9;
pub const D3D11_LOGIC_OP_OR: D3D11_LOGIC_OP = 8;
pub const D3D11_LOGIC_OP_OR_INVERTED: D3D11_LOGIC_OP = 15;
pub const D3D11_LOGIC_OP_OR_REVERSE: D3D11_LOGIC_OP = 14;
pub const D3D11_LOGIC_OP_SET: D3D11_LOGIC_OP = 1;
pub const D3D11_LOGIC_OP_XOR: D3D11_LOGIC_OP = 10;
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RASTERIZER_DESC1 {
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
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: windows_core::BOOL,
    pub LogicOpEnable: windows_core::BOOL,
    pub SrcBlend: super::d3d11::D3D11_BLEND,
    pub DestBlend: super::d3d11::D3D11_BLEND,
    pub BlendOp: super::d3d11::D3D11_BLEND_OP,
    pub SrcBlendAlpha: super::d3d11::D3D11_BLEND,
    pub DestBlendAlpha: super::d3d11::D3D11_BLEND,
    pub BlendOpAlpha: super::d3d11::D3D11_BLEND_OP,
    pub LogicOp: D3D11_LOGIC_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Debug, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    pub pCryptoSession: core::mem::ManuallyDrop<Option<super::d3d11::ID3D11CryptoSession>>,
    pub BlobSize: u32,
    pub pBlob: *mut core::ffi::c_void,
    pub pKeyInfoId: *mut windows_core::GUID,
    pub PrivateDataSize: u32,
    pub pPrivateData: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d11")]
impl Default for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    pub BufferType: super::d3d11::D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
}
#[cfg(feature = "Win32_d3d11")]
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_DECODER_CAPS = i32;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE: D3D11_VIDEO_DECODER_CAPS = 1;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_DYNAMIC: D3D11_VIDEO_DECODER_CAPS = 4;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_REQUIRED: D3D11_VIDEO_DECODER_CAPS = 8;
pub const D3D11_VIDEO_DECODER_CAPS_NON_REAL_TIME: D3D11_VIDEO_DECODER_CAPS = 2;
pub const D3D11_VIDEO_DECODER_CAPS_UNSUPPORTED: D3D11_VIDEO_DECODER_CAPS = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    pub ClearSize: u32,
    pub EncryptedSize: u32,
}
pub type D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = i32;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_COLOR_SPACE_CONVERSION: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 4;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_RESIZE: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 2;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_ROTATION: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 1;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_TRIPLE_BUFFER_OUTPUT: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 8;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    pub Enable: windows_core::BOOL,
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_SAMPLE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11BlendState1, ID3D11BlendState1_Vtbl, 0xcc86fabe_da55_401d_85e7_e3c9de2877e9);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11BlendState1 {
    type Target = super::d3d11::ID3D11BlendState;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11BlendState1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11BlendState);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11BlendState1 {
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_BLEND_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11BlendState1_Vtbl {
    pub base__: super::d3d11::ID3D11BlendState_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_BLEND_DESC1),
}
#[cfg(feature = "Win32_d3d11")]
pub trait ID3D11BlendState1_Impl: super::d3d11::ID3D11BlendState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_BLEND_DESC1);
}
#[cfg(feature = "Win32_d3d11")]
impl ID3D11BlendState1_Vtbl {
    pub const fn new<Identity: ID3D11BlendState1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D11BlendState1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11BlendState1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: super::d3d11::ID3D11BlendState_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11BlendState1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11BlendState as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d11")]
impl windows_core::RuntimeName for ID3D11BlendState1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11Device1, ID3D11Device1_Vtbl, 0xa04bfb29_08ef_43d6_a49c_a9bdbdcbe686);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11Device1 {
    type Target = super::d3d11::ID3D11Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11Device1, windows_core::IUnknown, super::d3d11::ID3D11Device);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11Device1 {
    pub unsafe fn GetImmediateContext1(&self) -> windows_core::Result<ID3D11DeviceContext1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmediateContext1)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn CreateDeferredContext1(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext1>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDeferredContext1)(windows_core::Interface::as_raw(self), contextflags, ppdeferredcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: Option<*mut Option<ID3D11BlendState1>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateBlendState1)(windows_core::Interface::as_raw(self), pblendstatedesc, ppblendstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: Option<*mut Option<ID3D11RasterizerState1>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateRasterizerState1)(windows_core::Interface::as_raw(self), prasterizerdesc, pprasterizerstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_d3dcommon")]
    pub unsafe fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: &[super::d3dcommon::D3D_FEATURE_LEVEL], sdkversion: u32, emulatedinterface: *const windows_core::GUID, pchosenfeaturelevel: Option<*mut super::d3dcommon::D3D_FEATURE_LEVEL>, ppcontextstate: Option<*mut Option<ID3DDeviceContextState>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDeviceContextState)(windows_core::Interface::as_raw(self), flags, core::mem::transmute(pfeaturelevels.as_ptr()), pfeaturelevels.len().try_into().unwrap(), sdkversion, emulatedinterface, pchosenfeaturelevel.unwrap_or(core::mem::zeroed()) as _, ppcontextstate.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OpenSharedResource1<T>(&self, hresource: super::winnt::HANDLE) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OpenSharedResource1)(windows_core::Interface::as_raw(self), hresource, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn OpenSharedResourceByName<P0, T>(&self, lpname: P0, dwdesiredaccess: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OpenSharedResourceByName)(windows_core::Interface::as_raw(self), lpname.param().abi(), dwdesiredaccess, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Device1_Vtbl {
    pub base__: super::d3d11::ID3D11Device_Vtbl,
    pub GetImmediateContext1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub CreateDeferredContext1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlendState1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_BLEND_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRasterizerState1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D11_RASTERIZER_DESC1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3dcommon")]
    pub CreateDeviceContextState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3dcommon::D3D_FEATURE_LEVEL, u32, u32, *const windows_core::GUID, *mut super::d3dcommon::D3D_FEATURE_LEVEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3dcommon"))]
    CreateDeviceContextState: usize,
    #[cfg(feature = "Win32_winnt")]
    pub OpenSharedResource1: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OpenSharedResource1: usize,
    pub OpenSharedResourceByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait ID3D11Device1_Impl: super::d3d11::ID3D11Device_Impl {
    fn GetImmediateContext1(&self, ppimmediatecontext: windows_core::OutRef<ID3D11DeviceContext1>);
    fn CreateDeferredContext1(&self, contextflags: u32, ppdeferredcontext: windows_core::OutRef<ID3D11DeviceContext1>) -> windows_core::Result<()>;
    fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: windows_core::OutRef<ID3D11BlendState1>) -> windows_core::Result<()>;
    fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: windows_core::OutRef<ID3D11RasterizerState1>) -> windows_core::Result<()>;
    fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const windows_core::GUID, pchosenfeaturelevel: *mut super::d3dcommon::D3D_FEATURE_LEVEL, ppcontextstate: windows_core::OutRef<ID3DDeviceContextState>) -> windows_core::Result<()>;
    fn OpenSharedResource1(&self, hresource: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenSharedResourceByName(&self, lpname: &windows_core::PCWSTR, dwdesiredaccess: u32, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl ID3D11Device1_Vtbl {
    pub const fn new<Identity: ID3D11Device1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImmediateContext1<Identity: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimmediatecontext: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device1_Impl::GetImmediateContext1(this, core::mem::transmute_copy(&ppimmediatecontext));
            }
        }
        unsafe extern "system" fn CreateDeferredContext1<Identity: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device1_Impl::CreateDeferredContext1(this, core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&ppdeferredcontext)).into()
            }
        }
        unsafe extern "system" fn CreateBlendState1<Identity: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device1_Impl::CreateBlendState1(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
            }
        }
        unsafe extern "system" fn CreateRasterizerState1<Identity: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device1_Impl::CreateRasterizerState1(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
            }
        }
        unsafe extern "system" fn CreateDeviceContextState<Identity: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32, pfeaturelevels: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const windows_core::GUID, pchosenfeaturelevel: *mut super::d3dcommon::D3D_FEATURE_LEVEL, ppcontextstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device1_Impl::CreateDeviceContextState(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pfeaturelevels), core::mem::transmute_copy(&featurelevels), core::mem::transmute_copy(&sdkversion), core::mem::transmute_copy(&emulatedinterface), core::mem::transmute_copy(&pchosenfeaturelevel), core::mem::transmute_copy(&ppcontextstate)).into()
            }
        }
        unsafe extern "system" fn OpenSharedResource1<Identity: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device1_Impl::OpenSharedResource1(this, core::mem::transmute_copy(&hresource), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
            }
        }
        unsafe extern "system" fn OpenSharedResourceByName<Identity: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpname: windows_core::PCWSTR, dwdesiredaccess: u32, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device1_Impl::OpenSharedResourceByName(this, core::mem::transmute(&lpname), core::mem::transmute_copy(&dwdesiredaccess), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
            }
        }
        Self {
            base__: super::d3d11::ID3D11Device_Vtbl::new::<Identity, OFFSET>(),
            GetImmediateContext1: GetImmediateContext1::<Identity, OFFSET>,
            CreateDeferredContext1: CreateDeferredContext1::<Identity, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, OFFSET>,
            CreateRasterizerState1: CreateRasterizerState1::<Identity, OFFSET>,
            CreateDeviceContextState: CreateDeviceContextState::<Identity, OFFSET>,
            OpenSharedResource1: OpenSharedResource1::<Identity, OFFSET>,
            OpenSharedResourceByName: OpenSharedResourceByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11Device1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11DeviceContext1, ID3D11DeviceContext1_Vtbl, 0xbb2c6faa_b5fb_4082_8e6b_388b8cfa90e1);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11DeviceContext1 {
    type Target = super::d3d11::ID3D11DeviceContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11DeviceContext);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11DeviceContext1 {
    pub unsafe fn CopySubresourceRegion1<P0, P5>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P5, srcsubresource: u32, psrcbox: Option<*const super::d3d11::D3D11_BOX>, copyflags: u32)
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
        P5: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopySubresourceRegion1)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.param().abi(), srcsubresource, psrcbox.unwrap_or(core::mem::zeroed()) as _, copyflags);
        }
    }
    pub unsafe fn UpdateSubresource1<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: Option<*const super::d3d11::D3D11_BOX>, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32)
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateSubresource1)(windows_core::Interface::as_raw(self), pdstresource.param().abi(), dstsubresource, pdstbox.unwrap_or(core::mem::zeroed()) as _, psrcdata, srcrowpitch, srcdepthpitch, copyflags);
        }
    }
    pub unsafe fn DiscardResource<P0>(&self, presource: P0)
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DiscardResource)(windows_core::Interface::as_raw(self), presource.param().abi());
        }
    }
    pub unsafe fn DiscardView<P0>(&self, presourceview: P0)
    where
        P0: windows_core::Param<super::d3d11::ID3D11View>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DiscardView)(windows_core::Interface::as_raw(self), presourceview.param().abi());
        }
    }
    pub unsafe fn VSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn HSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn DSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn GSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn PSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn CSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*const Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*const u32>, pnumconstants: Option<*const u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn VSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn HSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn DSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn GSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn PSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn CSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: Option<*mut Option<super::d3d11::ID3D11Buffer>>, pfirstconstant: Option<*mut u32>, pnumconstants: Option<*mut u32>) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetConstantBuffers1)(windows_core::Interface::as_raw(self), startslot, numbuffers, ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _, pfirstconstant.unwrap_or(core::mem::zeroed()) as _, pnumconstants.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn SwapDeviceContextState<P0>(&self, pstate: P0) -> windows_core::Result<ID3DDeviceContextState>
    where
        P0: windows_core::Param<ID3DDeviceContextState>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SwapDeviceContextState)(windows_core::Interface::as_raw(self), pstate.param().abi(), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn ClearView<P0>(&self, pview: P0, color: &[f32; 4], prect: Option<&[super::d3d11::D3D11_RECT]>)
    where
        P0: windows_core::Param<super::d3d11::ID3D11View>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearView)(windows_core::Interface::as_raw(self), pview.param().abi(), core::mem::transmute(color.as_ptr()), core::mem::transmute(prect.map_or(core::ptr::null(), |slice| slice.as_ptr())), prect.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn DiscardView1<P0>(&self, presourceview: P0, prects: Option<&[super::d3d11::D3D11_RECT]>)
    where
        P0: windows_core::Param<super::d3d11::ID3D11View>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DiscardView1)(windows_core::Interface::as_raw(self), presourceview.param().abi(), core::mem::transmute(prects.map_or(core::ptr::null(), |slice| slice.as_ptr())), prects.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceContext1_Vtbl {
    pub base__: super::d3d11::ID3D11DeviceContext_Vtbl,
    pub CopySubresourceRegion1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, u32, *const super::d3d11::D3D11_BOX, u32),
    pub UpdateSubresource1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const super::d3d11::D3D11_BOX, *const core::ffi::c_void, u32, u32, u32),
    pub DiscardResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DiscardView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub VSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub HSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub DSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub GSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub PSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub CSSetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void, *const u32, *const u32),
    pub VSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub HSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub DSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub GSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub PSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub CSGetConstantBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32),
    pub SwapDeviceContextState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(feature = "Win32_windef")]
    pub ClearView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32, *const super::d3d11::D3D11_RECT, u32),
    #[cfg(not(feature = "Win32_windef"))]
    ClearView: usize,
    #[cfg(feature = "Win32_windef")]
    pub DiscardView1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::d3d11::D3D11_RECT, u32),
    #[cfg(not(feature = "Win32_windef"))]
    DiscardView1: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
pub trait ID3D11DeviceContext1_Impl: super::d3d11::ID3D11DeviceContext_Impl {
    fn CopySubresourceRegion1(&self, pdstresource: windows_core::Ref<super::d3d11::ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: windows_core::Ref<super::d3d11::ID3D11Resource>, srcsubresource: u32, psrcbox: *const super::d3d11::D3D11_BOX, copyflags: u32);
    fn UpdateSubresource1(&self, pdstresource: windows_core::Ref<super::d3d11::ID3D11Resource>, dstsubresource: u32, pdstbox: *const super::d3d11::D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32);
    fn DiscardResource(&self, presource: windows_core::Ref<super::d3d11::ID3D11Resource>);
    fn DiscardView(&self, presourceview: windows_core::Ref<super::d3d11::ID3D11View>);
    fn VSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<super::d3d11::ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn HSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<super::d3d11::ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn DSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<super::d3d11::ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn GSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<super::d3d11::ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn PSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<super::d3d11::ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn CSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<super::d3d11::ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn VSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: windows_core::OutRef<super::d3d11::ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn HSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: windows_core::OutRef<super::d3d11::ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn DSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: windows_core::OutRef<super::d3d11::ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn GSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: windows_core::OutRef<super::d3d11::ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn PSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: windows_core::OutRef<super::d3d11::ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn CSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: windows_core::OutRef<super::d3d11::ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn SwapDeviceContextState(&self, pstate: windows_core::Ref<ID3DDeviceContextState>, pppreviousstate: windows_core::OutRef<ID3DDeviceContextState>);
    fn ClearView(&self, pview: windows_core::Ref<super::d3d11::ID3D11View>, color: *const f32, prect: *const super::d3d11::D3D11_RECT, numrects: u32);
    fn DiscardView1(&self, presourceview: windows_core::Ref<super::d3d11::ID3D11View>, prects: *const super::d3d11::D3D11_RECT, numrects: u32);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl ID3D11DeviceContext1_Vtbl {
    pub const fn new<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopySubresourceRegion1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const super::d3d11::D3D11_BOX, copyflags: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::CopySubresourceRegion1(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), core::mem::transmute_copy(&dstz), core::mem::transmute_copy(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox), core::mem::transmute_copy(&copyflags));
            }
        }
        unsafe extern "system" fn UpdateSubresource1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const super::d3d11::D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::UpdateSubresource1(this, core::mem::transmute_copy(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch), core::mem::transmute_copy(&copyflags));
            }
        }
        unsafe extern "system" fn DiscardResource<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::DiscardResource(this, core::mem::transmute_copy(&presource));
            }
        }
        unsafe extern "system" fn DiscardView<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourceview: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::DiscardView(this, core::mem::transmute_copy(&presourceview));
            }
        }
        unsafe extern "system" fn VSSetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::VSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn HSSetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::HSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn DSSetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::DSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn GSSetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::GSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn PSSetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::PSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn CSSetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::CSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn VSGetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::VSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn HSGetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::HSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn DSGetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::DSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn GSGetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::GSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn PSGetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::PSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn CSGetConstantBuffers1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::CSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants));
            }
        }
        unsafe extern "system" fn SwapDeviceContextState<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void, pppreviousstate: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::SwapDeviceContextState(this, core::mem::transmute_copy(&pstate), core::mem::transmute_copy(&pppreviousstate));
            }
        }
        unsafe extern "system" fn ClearView<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pview: *mut core::ffi::c_void, color: *const f32, prect: *const super::d3d11::D3D11_RECT, numrects: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::ClearView(this, core::mem::transmute_copy(&pview), core::mem::transmute_copy(&color), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&numrects));
            }
        }
        unsafe extern "system" fn DiscardView1<Identity: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourceview: *mut core::ffi::c_void, prects: *const super::d3d11::D3D11_RECT, numrects: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext1_Impl::DiscardView1(this, core::mem::transmute_copy(&presourceview), core::mem::transmute_copy(&prects), core::mem::transmute_copy(&numrects));
            }
        }
        Self {
            base__: super::d3d11::ID3D11DeviceContext_Vtbl::new::<Identity, OFFSET>(),
            CopySubresourceRegion1: CopySubresourceRegion1::<Identity, OFFSET>,
            UpdateSubresource1: UpdateSubresource1::<Identity, OFFSET>,
            DiscardResource: DiscardResource::<Identity, OFFSET>,
            DiscardView: DiscardView::<Identity, OFFSET>,
            VSSetConstantBuffers1: VSSetConstantBuffers1::<Identity, OFFSET>,
            HSSetConstantBuffers1: HSSetConstantBuffers1::<Identity, OFFSET>,
            DSSetConstantBuffers1: DSSetConstantBuffers1::<Identity, OFFSET>,
            GSSetConstantBuffers1: GSSetConstantBuffers1::<Identity, OFFSET>,
            PSSetConstantBuffers1: PSSetConstantBuffers1::<Identity, OFFSET>,
            CSSetConstantBuffers1: CSSetConstantBuffers1::<Identity, OFFSET>,
            VSGetConstantBuffers1: VSGetConstantBuffers1::<Identity, OFFSET>,
            HSGetConstantBuffers1: HSGetConstantBuffers1::<Identity, OFFSET>,
            DSGetConstantBuffers1: DSGetConstantBuffers1::<Identity, OFFSET>,
            GSGetConstantBuffers1: GSGetConstantBuffers1::<Identity, OFFSET>,
            PSGetConstantBuffers1: PSGetConstantBuffers1::<Identity, OFFSET>,
            CSGetConstantBuffers1: CSGetConstantBuffers1::<Identity, OFFSET>,
            SwapDeviceContextState: SwapDeviceContextState::<Identity, OFFSET>,
            ClearView: ClearView::<Identity, OFFSET>,
            DiscardView1: DiscardView1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D11DeviceContext1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11RasterizerState1, ID3D11RasterizerState1_Vtbl, 0x1217d7a6_5039_418c_b042_9cbe256afd6e);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11RasterizerState1 {
    type Target = super::d3d11::ID3D11RasterizerState;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11RasterizerState1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11RasterizerState);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11RasterizerState1 {
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_RASTERIZER_DESC1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), pdesc as _);
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11RasterizerState1_Vtbl {
    pub base__: super::d3d11::ID3D11RasterizerState_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RASTERIZER_DESC1),
}
#[cfg(feature = "Win32_d3d11")]
pub trait ID3D11RasterizerState1_Impl: super::d3d11::ID3D11RasterizerState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_RASTERIZER_DESC1);
}
#[cfg(feature = "Win32_d3d11")]
impl ID3D11RasterizerState1_Vtbl {
    pub const fn new<Identity: ID3D11RasterizerState1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: ID3D11RasterizerState1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11RasterizerState1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self { base__: super::d3d11::ID3D11RasterizerState_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RasterizerState1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11RasterizerState as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d11")]
impl windows_core::RuntimeName for ID3D11RasterizerState1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11VideoContext1, ID3D11VideoContext1_Vtbl, 0xa7f026da_a5f8_4487_a564_15e34357651e);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11VideoContext1 {
    type Target = super::d3d11::ID3D11VideoContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11VideoContext1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11VideoContext);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11VideoContext1 {
    pub unsafe fn SubmitDecoderBuffers1<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC1]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubmitDecoderBuffers1)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pbufferdesc.len().try_into().unwrap(), core::mem::transmute(pbufferdesc.as_ptr())) }
    }
    pub unsafe fn GetDataForNewHardwareKey<P0>(&self, pcryptosession: P0, pprivatinputdata: &[u8]) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<super::d3d11::ID3D11CryptoSession>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataForNewHardwareKey)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), pprivatinputdata.len().try_into().unwrap(), core::mem::transmute(pprivatinputdata.as_ptr()), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CheckCryptoSessionStatus<P0>(&self, pcryptosession: P0) -> windows_core::Result<D3D11_CRYPTO_SESSION_STATUS>
    where
        P0: windows_core::Param<super::d3d11::ID3D11CryptoSession>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCryptoSessionStatus)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn DecoderEnableDownsampling<P0>(&self, pdecoder: P0, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecoderEnableDownsampling)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), inputcolorspace, poutputdesc, referenceframecount) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn DecoderUpdateDownsampling<P0>(&self, pdecoder: P0, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecoderUpdateDownsampling)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), poutputdesc) }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn VideoProcessorSetOutputColorSpace1<P0>(&self, pvideoprocessor: P0, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputColorSpace1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), colorspace);
        }
    }
    pub unsafe fn VideoProcessorSetOutputShaderUsage<P0>(&self, pvideoprocessor: P0, shaderusage: bool)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputShaderUsage)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), shaderusage.into());
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn VideoProcessorGetOutputColorSpace1<P0>(&self, pvideoprocessor: P0) -> super::dxgicommon::DXGI_COLOR_SPACE_TYPE
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputColorSpace1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), &mut result__);
            result__
        }
    }
    pub unsafe fn VideoProcessorGetOutputShaderUsage<P0>(&self, pvideoprocessor: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputShaderUsage)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), &mut result__);
            result__
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn VideoProcessorSetStreamColorSpace1<P0>(&self, pvideoprocessor: P0, streamindex: u32, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamColorSpace1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, colorspace);
        }
    }
    pub unsafe fn VideoProcessorSetStreamMirror<P0>(&self, pvideoprocessor: P0, streamindex: u32, enable: bool, fliphorizontal: bool, flipvertical: bool)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamMirror)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, enable.into(), fliphorizontal.into(), flipvertical.into());
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn VideoProcessorGetStreamColorSpace1<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> super::dxgicommon::DXGI_COLOR_SPACE_TYPE
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamColorSpace1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, &mut result__);
            result__
        }
    }
    pub unsafe fn VideoProcessorGetStreamMirror<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut windows_core::BOOL, pfliphorizontal: *mut windows_core::BOOL, pflipvertical: *mut windows_core::BOOL)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamMirror)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, penable as _, pfliphorizontal as _, pflipvertical as _);
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn VideoProcessorGetBehaviorHints<P0>(&self, pvideoprocessor: P0, outputwidth: u32, outputheight: u32, outputformat: super::dxgiformat::DXGI_FORMAT, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT]) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetBehaviorHints)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), outputwidth, outputheight, outputformat, pstreams.len().try_into().unwrap(), core::mem::transmute(pstreams.as_ptr()), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoContext1_Vtbl {
    pub base__: super::d3d11::ID3D11VideoContext_Vtbl,
    pub SubmitDecoderBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> windows_core::HRESULT,
    pub GetDataForNewHardwareKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub CheckCryptoSessionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut D3D11_CRYPTO_SESSION_STATUS) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub DecoderEnableDownsampling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, *const D3D11_VIDEO_SAMPLE_DESC, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    DecoderEnableDownsampling: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub DecoderUpdateDownsampling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    DecoderUpdateDownsampling: usize,
    #[cfg(feature = "Win32_dxgicommon")]
    pub VideoProcessorSetOutputColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dxgicommon::DXGI_COLOR_SPACE_TYPE),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    VideoProcessorSetOutputColorSpace1: usize,
    pub VideoProcessorSetOutputShaderUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL),
    #[cfg(feature = "Win32_dxgicommon")]
    pub VideoProcessorGetOutputColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::dxgicommon::DXGI_COLOR_SPACE_TYPE),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    VideoProcessorGetOutputColorSpace1: usize,
    pub VideoProcessorGetOutputShaderUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL),
    #[cfg(feature = "Win32_dxgicommon")]
    pub VideoProcessorSetStreamColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::dxgicommon::DXGI_COLOR_SPACE_TYPE),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    VideoProcessorSetStreamColorSpace1: usize,
    pub VideoProcessorSetStreamMirror: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, windows_core::BOOL, windows_core::BOOL),
    #[cfg(feature = "Win32_dxgicommon")]
    pub VideoProcessorGetStreamColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::dxgicommon::DXGI_COLOR_SPACE_TYPE),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    VideoProcessorGetStreamColorSpace1: usize,
    pub VideoProcessorGetStreamMirror: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut windows_core::BOOL, *mut windows_core::BOOL),
    #[cfg(feature = "Win32_dxgiformat")]
    pub VideoProcessorGetBehaviorHints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, super::dxgiformat::DXGI_FORMAT, u32, *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    VideoProcessorGetBehaviorHints: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait ID3D11VideoContext1_Impl: super::d3d11::ID3D11VideoContext_Impl {
    fn SubmitDecoderBuffers1(&self, pdecoder: windows_core::Ref<super::d3d11::ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> windows_core::Result<()>;
    fn GetDataForNewHardwareKey(&self, pcryptosession: windows_core::Ref<super::d3d11::ID3D11CryptoSession>, privateinputsize: u32, pprivatinputdata: *const core::ffi::c_void) -> windows_core::Result<u64>;
    fn CheckCryptoSessionStatus(&self, pcryptosession: windows_core::Ref<super::d3d11::ID3D11CryptoSession>) -> windows_core::Result<D3D11_CRYPTO_SESSION_STATUS>;
    fn DecoderEnableDownsampling(&self, pdecoder: windows_core::Ref<super::d3d11::ID3D11VideoDecoder>, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> windows_core::Result<()>;
    fn DecoderUpdateDownsampling(&self, pdecoder: windows_core::Ref<super::d3d11::ID3D11VideoDecoder>, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> windows_core::Result<()>;
    fn VideoProcessorSetOutputColorSpace1(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetOutputShaderUsage(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, shaderusage: windows_core::BOOL);
    fn VideoProcessorGetOutputColorSpace1(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, pcolorspace: *mut super::dxgicommon::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetOutputShaderUsage(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, pshaderusage: *mut windows_core::BOOL);
    fn VideoProcessorSetStreamColorSpace1(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, streamindex: u32, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetStreamMirror(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, streamindex: u32, enable: windows_core::BOOL, fliphorizontal: windows_core::BOOL, flipvertical: windows_core::BOOL);
    fn VideoProcessorGetStreamColorSpace1(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut super::dxgicommon::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetStreamMirror(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, streamindex: u32, penable: *mut windows_core::BOOL, pfliphorizontal: *mut windows_core::BOOL, pflipvertical: *mut windows_core::BOOL);
    fn VideoProcessorGetBehaviorHints(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, outputwidth: u32, outputheight: u32, outputformat: super::dxgiformat::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl ID3D11VideoContext1_Vtbl {
    pub const fn new<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubmitDecoderBuffers1<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::SubmitDecoderBuffers1(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&pbufferdesc)).into()
            }
        }
        unsafe extern "system" fn GetDataForNewHardwareKey<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, privateinputsize: u32, pprivatinputdata: *const core::ffi::c_void, pprivateoutputdata: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoContext1_Impl::GetDataForNewHardwareKey(this, core::mem::transmute_copy(&pcryptosession), core::mem::transmute_copy(&privateinputsize), core::mem::transmute_copy(&pprivatinputdata)) {
                    Ok(ok__) => {
                        pprivateoutputdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckCryptoSessionStatus<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, pstatus: *mut D3D11_CRYPTO_SESSION_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoContext1_Impl::CheckCryptoSessionStatus(this, core::mem::transmute_copy(&pcryptosession)) {
                    Ok(ok__) => {
                        pstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DecoderEnableDownsampling<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::DecoderEnableDownsampling(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&inputcolorspace), core::mem::transmute_copy(&poutputdesc), core::mem::transmute_copy(&referenceframecount)).into()
            }
        }
        unsafe extern "system" fn DecoderUpdateDownsampling<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::DecoderUpdateDownsampling(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&poutputdesc)).into()
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace1<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::VideoProcessorSetOutputColorSpace1(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&colorspace));
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputShaderUsage<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, shaderusage: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::VideoProcessorSetOutputShaderUsage(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&shaderusage));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace1<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pcolorspace: *mut super::dxgicommon::DXGI_COLOR_SPACE_TYPE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::VideoProcessorGetOutputColorSpace1(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&pcolorspace));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputShaderUsage<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pshaderusage: *mut windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::VideoProcessorGetOutputShaderUsage(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&pshaderusage));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace1<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::VideoProcessorSetStreamColorSpace1(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&colorspace));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamMirror<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: windows_core::BOOL, fliphorizontal: windows_core::BOOL, flipvertical: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::VideoProcessorSetStreamMirror(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&fliphorizontal), core::mem::transmute_copy(&flipvertical));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace1<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pcolorspace: *mut super::dxgicommon::DXGI_COLOR_SPACE_TYPE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::VideoProcessorGetStreamColorSpace1(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pcolorspace));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamMirror<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penable: *mut windows_core::BOOL, pfliphorizontal: *mut windows_core::BOOL, pflipvertical: *mut windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext1_Impl::VideoProcessorGetStreamMirror(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penable), core::mem::transmute_copy(&pfliphorizontal), core::mem::transmute_copy(&pflipvertical));
            }
        }
        unsafe extern "system" fn VideoProcessorGetBehaviorHints<Identity: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, outputwidth: u32, outputheight: u32, outputformat: super::dxgiformat::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, pbehaviorhints: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoContext1_Impl::VideoProcessorGetBehaviorHints(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&outputwidth), core::mem::transmute_copy(&outputheight), core::mem::transmute_copy(&outputformat), core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreams)) {
                    Ok(ok__) => {
                        pbehaviorhints.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d3d11::ID3D11VideoContext_Vtbl::new::<Identity, OFFSET>(),
            SubmitDecoderBuffers1: SubmitDecoderBuffers1::<Identity, OFFSET>,
            GetDataForNewHardwareKey: GetDataForNewHardwareKey::<Identity, OFFSET>,
            CheckCryptoSessionStatus: CheckCryptoSessionStatus::<Identity, OFFSET>,
            DecoderEnableDownsampling: DecoderEnableDownsampling::<Identity, OFFSET>,
            DecoderUpdateDownsampling: DecoderUpdateDownsampling::<Identity, OFFSET>,
            VideoProcessorSetOutputColorSpace1: VideoProcessorSetOutputColorSpace1::<Identity, OFFSET>,
            VideoProcessorSetOutputShaderUsage: VideoProcessorSetOutputShaderUsage::<Identity, OFFSET>,
            VideoProcessorGetOutputColorSpace1: VideoProcessorGetOutputColorSpace1::<Identity, OFFSET>,
            VideoProcessorGetOutputShaderUsage: VideoProcessorGetOutputShaderUsage::<Identity, OFFSET>,
            VideoProcessorSetStreamColorSpace1: VideoProcessorSetStreamColorSpace1::<Identity, OFFSET>,
            VideoProcessorSetStreamMirror: VideoProcessorSetStreamMirror::<Identity, OFFSET>,
            VideoProcessorGetStreamColorSpace1: VideoProcessorGetStreamColorSpace1::<Identity, OFFSET>,
            VideoProcessorGetStreamMirror: VideoProcessorGetStreamMirror::<Identity, OFFSET>,
            VideoProcessorGetBehaviorHints: VideoProcessorGetBehaviorHints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11VideoContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11VideoContext1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11VideoDevice1, ID3D11VideoDevice1_Vtbl, 0x29da1d51_1321_4454_804b_f5fc9f861f0f);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11VideoDevice1 {
    type Target = super::d3d11::ID3D11VideoDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11VideoDevice1, windows_core::IUnknown, super::d3d11::ID3D11VideoDevice);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11VideoDevice1 {
    pub unsafe fn GetCryptoSessionPrivateDataSize(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: Option<*const windows_core::GUID>, pkeyexchangetype: *const windows_core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCryptoSessionPrivateDataSize)(windows_core::Interface::as_raw(self), pcryptotype, pdecoderprofile.unwrap_or(core::mem::zeroed()) as _, pkeyexchangetype, pprivateinputsize as _, pprivateoutputsize as _) }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn GetVideoDecoderCaps(&self, pdecoderprofile: *const windows_core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::dxgicommon::DXGI_RATIONAL, bitrate: u32, pcryptotype: Option<*const windows_core::GUID>) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoDecoderCaps)(windows_core::Interface::as_raw(self), pdecoderprofile, samplewidth, sampleheight, pframerate, bitrate, pcryptotype.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CheckVideoDecoderDownsampling(&self, pinputdesc: *const super::d3d11::D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const super::d3d11::D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::dxgicommon::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut windows_core::BOOL, prealtimehint: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckVideoDecoderDownsampling)(windows_core::Interface::as_raw(self), pinputdesc, inputcolorspace, pinputconfig, pframerate, poutputdesc, psupported as _, prealtimehint as _) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn RecommendVideoDecoderDownsampleParameters(&self, pinputdesc: *const super::d3d11::D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const super::d3d11::D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::dxgicommon::DXGI_RATIONAL) -> windows_core::Result<D3D11_VIDEO_SAMPLE_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecommendVideoDecoderDownsampleParameters)(windows_core::Interface::as_raw(self), pinputdesc, inputcolorspace, pinputconfig, pframerate, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoDevice1_Vtbl {
    pub base__: super::d3d11::ID3D11VideoDevice_Vtbl,
    pub GetCryptoSessionPrivateDataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgicommon")]
    pub GetVideoDecoderCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *const super::dxgicommon::DXGI_RATIONAL, u32, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgicommon"))]
    GetVideoDecoderCaps: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CheckVideoDecoderDownsampling: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d11::D3D11_VIDEO_DECODER_DESC, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, *const super::d3d11::D3D11_VIDEO_DECODER_CONFIG, *const super::dxgicommon::DXGI_RATIONAL, *const D3D11_VIDEO_SAMPLE_DESC, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CheckVideoDecoderDownsampling: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub RecommendVideoDecoderDownsampleParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d3d11::D3D11_VIDEO_DECODER_DESC, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, *const super::d3d11::D3D11_VIDEO_DECODER_CONFIG, *const super::dxgicommon::DXGI_RATIONAL, *mut D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    RecommendVideoDecoderDownsampleParameters: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11VideoDevice1_Impl: super::d3d11::ID3D11VideoDevice_Impl {
    fn GetCryptoSessionPrivateDataSize(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pkeyexchangetype: *const windows_core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> windows_core::Result<()>;
    fn GetVideoDecoderCaps(&self, pdecoderprofile: *const windows_core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::dxgicommon::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn CheckVideoDecoderDownsampling(&self, pinputdesc: *const super::d3d11::D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const super::d3d11::D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::dxgicommon::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut windows_core::BOOL, prealtimehint: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn RecommendVideoDecoderDownsampleParameters(&self, pinputdesc: *const super::d3d11::D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const super::d3d11::D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::dxgicommon::DXGI_RATIONAL) -> windows_core::Result<D3D11_VIDEO_SAMPLE_DESC>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D11VideoDevice1_Vtbl {
    pub const fn new<Identity: ID3D11VideoDevice1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCryptoSessionPrivateDataSize<Identity: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pkeyexchangetype: *const windows_core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice1_Impl::GetCryptoSessionPrivateDataSize(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&pkeyexchangetype), core::mem::transmute_copy(&pprivateinputsize), core::mem::transmute_copy(&pprivateoutputsize)).into()
            }
        }
        unsafe extern "system" fn GetVideoDecoderCaps<Identity: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoderprofile: *const windows_core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::dxgicommon::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const windows_core::GUID, pdecodercaps: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice1_Impl::GetVideoDecoderCaps(this, core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&samplewidth), core::mem::transmute_copy(&sampleheight), core::mem::transmute_copy(&pframerate), core::mem::transmute_copy(&bitrate), core::mem::transmute_copy(&pcryptotype)) {
                    Ok(ok__) => {
                        pdecodercaps.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckVideoDecoderDownsampling<Identity: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputdesc: *const super::d3d11::D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const super::d3d11::D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::dxgicommon::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut windows_core::BOOL, prealtimehint: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice1_Impl::CheckVideoDecoderDownsampling(this, core::mem::transmute_copy(&pinputdesc), core::mem::transmute_copy(&inputcolorspace), core::mem::transmute_copy(&pinputconfig), core::mem::transmute_copy(&pframerate), core::mem::transmute_copy(&poutputdesc), core::mem::transmute_copy(&psupported), core::mem::transmute_copy(&prealtimehint)).into()
            }
        }
        unsafe extern "system" fn RecommendVideoDecoderDownsampleParameters<Identity: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputdesc: *const super::d3d11::D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const super::d3d11::D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::dxgicommon::DXGI_RATIONAL, precommendedoutputdesc: *mut D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice1_Impl::RecommendVideoDecoderDownsampleParameters(this, core::mem::transmute_copy(&pinputdesc), core::mem::transmute_copy(&inputcolorspace), core::mem::transmute_copy(&pinputconfig), core::mem::transmute_copy(&pframerate)) {
                    Ok(ok__) => {
                        precommendedoutputdesc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d3d11::ID3D11VideoDevice_Vtbl::new::<Identity, OFFSET>(),
            GetCryptoSessionPrivateDataSize: GetCryptoSessionPrivateDataSize::<Identity, OFFSET>,
            GetVideoDecoderCaps: GetVideoDecoderCaps::<Identity, OFFSET>,
            CheckVideoDecoderDownsampling: CheckVideoDecoderDownsampling::<Identity, OFFSET>,
            RecommendVideoDecoderDownsampleParameters: RecommendVideoDecoderDownsampleParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDevice1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11VideoDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11VideoDevice1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3D11VideoProcessorEnumerator1, ID3D11VideoProcessorEnumerator1_Vtbl, 0x465217f2_5568_43cf_b5b9_f61d54531ca1);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3D11VideoProcessorEnumerator1 {
    type Target = super::d3d11::ID3D11VideoProcessorEnumerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorEnumerator1, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11VideoProcessorEnumerator);
#[cfg(feature = "Win32_d3d11")]
impl ID3D11VideoProcessorEnumerator1 {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CheckVideoProcessorFormatConversion(&self, inputformat: super::dxgiformat::DXGI_FORMAT, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, outputformat: super::dxgiformat::DXGI_FORMAT, outputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckVideoProcessorFormatConversion)(windows_core::Interface::as_raw(self), inputformat, inputcolorspace, outputformat, outputcolorspace, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoProcessorEnumerator1_Vtbl {
    pub base__: super::d3d11::ID3D11VideoProcessorEnumerator_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CheckVideoProcessorFormatConversion: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, super::dxgiformat::DXGI_FORMAT, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CheckVideoProcessorFormatConversion: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11VideoProcessorEnumerator1_Impl: super::d3d11::ID3D11VideoProcessorEnumerator_Impl {
    fn CheckVideoProcessorFormatConversion(&self, inputformat: super::dxgiformat::DXGI_FORMAT, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, outputformat: super::dxgiformat::DXGI_FORMAT, outputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D11VideoProcessorEnumerator1_Vtbl {
    pub const fn new<Identity: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckVideoProcessorFormatConversion<Identity: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputformat: super::dxgiformat::DXGI_FORMAT, inputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, outputformat: super::dxgiformat::DXGI_FORMAT, outputcolorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, psupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoProcessorEnumerator1_Impl::CheckVideoProcessorFormatConversion(this, core::mem::transmute_copy(&inputformat), core::mem::transmute_copy(&inputcolorspace), core::mem::transmute_copy(&outputformat), core::mem::transmute_copy(&outputcolorspace)) {
                    Ok(ok__) => {
                        psupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d3d11::ID3D11VideoProcessorEnumerator_Vtbl::new::<Identity, OFFSET>(),
            CheckVideoProcessorFormatConversion: CheckVideoProcessorFormatConversion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator1 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11VideoProcessorEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11VideoProcessorEnumerator1 {}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::define_interface!(ID3DDeviceContextState, ID3DDeviceContextState_Vtbl, 0x5c1e0d8a_7c23_48f9_8c59_a92958ceff11);
#[cfg(feature = "Win32_d3d11")]
impl core::ops::Deref for ID3DDeviceContextState {
    type Target = super::d3d11::ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d11")]
windows_core::imp::interface_hierarchy!(ID3DDeviceContextState, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild);
#[cfg(feature = "Win32_d3d11")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3DDeviceContextState_Vtbl {
    pub base__: super::d3d11::ID3D11DeviceChild_Vtbl,
}
#[cfg(feature = "Win32_d3d11")]
pub trait ID3DDeviceContextState_Impl: super::d3d11::ID3D11DeviceChild_Impl {}
#[cfg(feature = "Win32_d3d11")]
impl ID3DDeviceContextState_Vtbl {
    pub const fn new<Identity: ID3DDeviceContextState_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::d3d11::ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3DDeviceContextState as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d11")]
impl windows_core::RuntimeName for ID3DDeviceContextState {}
windows_core::imp::define_interface!(ID3DUserDefinedAnnotation, ID3DUserDefinedAnnotation_Vtbl, 0xb2daad8b_03d4_4dbf_95eb_32ab4b63d0ab);
windows_core::imp::interface_hierarchy!(ID3DUserDefinedAnnotation, windows_core::IUnknown);
impl ID3DUserDefinedAnnotation {
    pub unsafe fn BeginEvent<P0>(&self, name: P0) -> i32
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginEvent)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn EndEvent(&self) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMarker<P0>(&self, name: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetMarker)(windows_core::Interface::as_raw(self), name.param().abi());
        }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3DUserDefinedAnnotation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> i32,
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    pub SetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait ID3DUserDefinedAnnotation_Impl: windows_core::IUnknownImpl {
    fn BeginEvent(&self, name: &windows_core::PCWSTR) -> i32;
    fn EndEvent(&self) -> i32;
    fn SetMarker(&self, name: &windows_core::PCWSTR);
    fn GetStatus(&self) -> windows_core::BOOL;
}
impl ID3DUserDefinedAnnotation_Vtbl {
    pub const fn new<Identity: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginEvent<Identity: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3DUserDefinedAnnotation_Impl::BeginEvent(this, core::mem::transmute(&name))
            }
        }
        unsafe extern "system" fn EndEvent<Identity: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3DUserDefinedAnnotation_Impl::EndEvent(this)
            }
        }
        unsafe extern "system" fn SetMarker<Identity: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3DUserDefinedAnnotation_Impl::SetMarker(this, core::mem::transmute(&name));
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3DUserDefinedAnnotation_Impl::GetStatus(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginEvent: BeginEvent::<Identity, OFFSET>,
            EndEvent: EndEvent::<Identity, OFFSET>,
            SetMarker: SetMarker::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3DUserDefinedAnnotation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3DUserDefinedAnnotation {}
