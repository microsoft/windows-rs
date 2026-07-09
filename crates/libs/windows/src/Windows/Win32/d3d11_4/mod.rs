pub type D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS = u32;
pub const D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAG_NONE: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    pub ExtendedNV12SharedTextureSupported: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    pub DecoderDesc: super::d3d11::D3D11_VIDEO_DECODER_DESC,
    pub Components: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS,
    pub BinCount: u32,
    pub CounterBitDepth: u32,
}
pub type D3D11_FEATURE_VIDEO = i32;
pub const D3D11_FEATURE_VIDEO_DECODER_HISTOGRAM: D3D11_FEATURE_VIDEO = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    pub BufferType: super::d3d11::D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut super::d3d11_1::D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
    pub cBlocksStripeEncrypted: u32,
    pub cBlocksStripeClear: u32,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_A: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 3;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_B: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 2;
pub type D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_A: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 8;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_B: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 4;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_G: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 2;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_NONE: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 0;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_R: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 1;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_U: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 2;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_V: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 4;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_Y: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS = 1;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_G: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 1;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_R: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 0;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_U: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 1;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_V: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 2;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_Y: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT = 0;
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
windows_core::imp::define_interface!(ID3D11Device4, ID3D11Device4_Vtbl, 0x8992ab71_02e6_4b8d_ba48_b056dcda42c4);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
impl core::ops::Deref for ID3D11Device4 {
    type Target = super::d3d11_3::ID3D11Device3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
windows_core::imp::interface_hierarchy!(ID3D11Device4, windows_core::IUnknown, super::d3d11::ID3D11Device, super::d3d11_1::ID3D11Device1, super::d3d11_2::ID3D11Device2, super::d3d11_3::ID3D11Device3);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
impl ID3D11Device4 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterDeviceRemovedEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterDeviceRemovedEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterDeviceRemoved(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterDeviceRemoved)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Device4_Vtbl {
    pub base__: super::d3d11_3::ID3D11Device3_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterDeviceRemovedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterDeviceRemovedEvent: usize,
    pub UnregisterDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait ID3D11Device4_Impl: super::d3d11_3::ID3D11Device3_Impl {
    fn RegisterDeviceRemovedEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterDeviceRemoved(&self, dwcookie: u32);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl ID3D11Device4_Vtbl {
    pub const fn new<Identity: ID3D11Device4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterDeviceRemovedEvent<Identity: ID3D11Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device4_Impl::RegisterDeviceRemovedEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterDeviceRemoved<Identity: ID3D11Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device4_Impl::UnregisterDeviceRemoved(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        Self {
            base__: super::d3d11_3::ID3D11Device3_Vtbl::new::<Identity, OFFSET>(),
            RegisterDeviceRemovedEvent: RegisterDeviceRemovedEvent::<Identity, OFFSET>,
            UnregisterDeviceRemoved: UnregisterDeviceRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device4 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Device as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11Device1 as windows_core::Interface>::IID || iid == &<super::d3d11_2::ID3D11Device2 as windows_core::Interface>::IID || iid == &<super::d3d11_3::ID3D11Device3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11Device4 {}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
windows_core::imp::define_interface!(ID3D11Device5, ID3D11Device5_Vtbl, 0x8ffde202_a0e7_45df_9e01_e837801b5ea0);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
impl core::ops::Deref for ID3D11Device5 {
    type Target = ID3D11Device4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
windows_core::imp::interface_hierarchy!(ID3D11Device5, windows_core::IUnknown, super::d3d11::ID3D11Device, super::d3d11_1::ID3D11Device1, super::d3d11_2::ID3D11Device2, super::d3d11_3::ID3D11Device3, ID3D11Device4);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
impl ID3D11Device5 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OpenSharedFence<T>(&self, hfence: super::winnt::HANDLE, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenSharedFence)(windows_core::Interface::as_raw(self), hfence, &T::IID, result__ as *mut _ as *mut _).ok() }
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: super::d3d11_3::D3D11_FENCE_FLAG, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateFence)(windows_core::Interface::as_raw(self), initialvalue, flags, &T::IID, result__ as *mut _ as *mut _).ok() }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Device5_Vtbl {
    pub base__: ID3D11Device4_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub OpenSharedFence: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OpenSharedFence: usize,
    pub CreateFence: unsafe extern "system" fn(*mut core::ffi::c_void, u64, super::d3d11_3::D3D11_FENCE_FLAG, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait ID3D11Device5_Impl: ID3D11Device4_Impl {
    fn OpenSharedFence(&self, hfence: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateFence(&self, initialvalue: u64, flags: super::d3d11_3::D3D11_FENCE_FLAG, returnedinterface: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl ID3D11Device5_Vtbl {
    pub const fn new<Identity: ID3D11Device5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenSharedFence<Identity: ID3D11Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfence: super::winnt::HANDLE, returnedinterface: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device5_Impl::OpenSharedFence(this, core::mem::transmute_copy(&hfence), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppfence)).into()
            }
        }
        unsafe extern "system" fn CreateFence<Identity: ID3D11Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: u64, flags: super::d3d11_3::D3D11_FENCE_FLAG, returnedinterface: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device5_Impl::CreateFence(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppfence)).into()
            }
        }
        Self {
            base__: ID3D11Device4_Vtbl::new::<Identity, OFFSET>(),
            OpenSharedFence: OpenSharedFence::<Identity, OFFSET>,
            CreateFence: CreateFence::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device5 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Device as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11Device1 as windows_core::Interface>::IID || iid == &<super::d3d11_2::ID3D11Device2 as windows_core::Interface>::IID || iid == &<super::d3d11_3::ID3D11Device3 as windows_core::Interface>::IID || iid == &<ID3D11Device4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3d11_2", feature = "Win32_d3d11_3", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11Device5 {}
windows_core::imp::define_interface!(ID3D11Multithread, ID3D11Multithread_Vtbl, 0x9b7e4e00_342c_4106_a19f_4f2704f689f0);
windows_core::imp::interface_hierarchy!(ID3D11Multithread, windows_core::IUnknown);
impl ID3D11Multithread {
    pub unsafe fn Enter(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Leave(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Leave)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn SetMultithreadProtected(&self, bmtprotect: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).SetMultithreadProtected)(windows_core::Interface::as_raw(self), bmtprotect.into()) }
    }
    pub unsafe fn GetMultithreadProtected(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMultithreadProtected)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Multithread_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    pub GetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait ID3D11Multithread_Impl: windows_core::IUnknownImpl {
    fn Enter(&self);
    fn Leave(&self);
    fn SetMultithreadProtected(&self, bmtprotect: windows_core::BOOL) -> windows_core::BOOL;
    fn GetMultithreadProtected(&self) -> windows_core::BOOL;
}
impl ID3D11Multithread_Vtbl {
    pub const fn new<Identity: ID3D11Multithread_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enter<Identity: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Multithread_Impl::Enter(this);
            }
        }
        unsafe extern "system" fn Leave<Identity: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Multithread_Impl::Leave(this);
            }
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmtprotect: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Multithread_Impl::SetMultithreadProtected(this, core::mem::transmute_copy(&bmtprotect))
            }
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Multithread_Impl::GetMultithreadProtected(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, OFFSET>,
            Leave: Leave::<Identity, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Multithread as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Multithread {}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::define_interface!(ID3D11VideoContext2, ID3D11VideoContext2_Vtbl, 0xc4e7374c_6243_4d1b_ae87_52b4f740e261);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl core::ops::Deref for ID3D11VideoContext2 {
    type Target = super::d3d11_1::ID3D11VideoContext1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::interface_hierarchy!(ID3D11VideoContext2, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11VideoContext, super::d3d11_1::ID3D11VideoContext1);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl ID3D11VideoContext2 {
    #[cfg(feature = "Win32_dxgi1_5")]
    pub unsafe fn VideoProcessorSetOutputHDRMetaData<P0>(&self, pvideoprocessor: P0, r#type: super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: Option<*const core::ffi::c_void>)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputHDRMetaData)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), r#type, size, phdrmetadata.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dxgi1_5")]
    pub unsafe fn VideoProcessorGetOutputHDRMetaData<P0>(&self, pvideoprocessor: P0, ptype: *mut super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: Option<*mut core::ffi::c_void>)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputHDRMetaData)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), ptype as _, size, pmetadata.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dxgi1_5")]
    pub unsafe fn VideoProcessorSetStreamHDRMetaData<P0>(&self, pvideoprocessor: P0, streamindex: u32, r#type: super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: Option<*const core::ffi::c_void>)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamHDRMetaData)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, r#type, size, phdrmetadata.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "Win32_dxgi1_5")]
    pub unsafe fn VideoProcessorGetStreamHDRMetaData<P0>(&self, pvideoprocessor: P0, streamindex: u32, ptype: *mut super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: Option<*mut core::ffi::c_void>)
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamHDRMetaData)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), streamindex, ptype as _, size, pmetadata.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoContext2_Vtbl {
    pub base__: super::d3d11_1::ID3D11VideoContext1_Vtbl,
    #[cfg(feature = "Win32_dxgi1_5")]
    pub VideoProcessorSetOutputHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dxgi1_5::DXGI_HDR_METADATA_TYPE, u32, *const core::ffi::c_void),
    #[cfg(not(feature = "Win32_dxgi1_5"))]
    VideoProcessorSetOutputHDRMetaData: usize,
    #[cfg(feature = "Win32_dxgi1_5")]
    pub VideoProcessorGetOutputHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::dxgi1_5::DXGI_HDR_METADATA_TYPE, u32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dxgi1_5"))]
    VideoProcessorGetOutputHDRMetaData: usize,
    #[cfg(feature = "Win32_dxgi1_5")]
    pub VideoProcessorSetStreamHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::dxgi1_5::DXGI_HDR_METADATA_TYPE, u32, *const core::ffi::c_void),
    #[cfg(not(feature = "Win32_dxgi1_5"))]
    VideoProcessorSetStreamHDRMetaData: usize,
    #[cfg(feature = "Win32_dxgi1_5")]
    pub VideoProcessorGetStreamHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::dxgi1_5::DXGI_HDR_METADATA_TYPE, u32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_dxgi1_5"))]
    VideoProcessorGetStreamHDRMetaData: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait ID3D11VideoContext2_Impl: super::d3d11_1::ID3D11VideoContext1_Impl {
    fn VideoProcessorSetOutputHDRMetaData(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, r#type: super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const core::ffi::c_void);
    fn VideoProcessorGetOutputHDRMetaData(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, ptype: *mut super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut core::ffi::c_void);
    fn VideoProcessorSetStreamHDRMetaData(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, streamindex: u32, r#type: super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const core::ffi::c_void);
    fn VideoProcessorGetStreamHDRMetaData(&self, pvideoprocessor: windows_core::Ref<super::d3d11::ID3D11VideoProcessor>, streamindex: u32, ptype: *mut super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut core::ffi::c_void);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl ID3D11VideoContext2_Vtbl {
    pub const fn new<Identity: ID3D11VideoContext2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VideoProcessorSetOutputHDRMetaData<Identity: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, r#type: super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext2_Impl::VideoProcessorSetOutputHDRMetaData(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&phdrmetadata));
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputHDRMetaData<Identity: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, ptype: *mut super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext2_Impl::VideoProcessorGetOutputHDRMetaData(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata));
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamHDRMetaData<Identity: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, r#type: super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext2_Impl::VideoProcessorSetStreamHDRMetaData(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&phdrmetadata));
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamHDRMetaData<Identity: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, ptype: *mut super::dxgi1_5::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext2_Impl::VideoProcessorGetStreamHDRMetaData(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata));
            }
        }
        Self {
            base__: super::d3d11_1::ID3D11VideoContext1_Vtbl::new::<Identity, OFFSET>(),
            VideoProcessorSetOutputHDRMetaData: VideoProcessorSetOutputHDRMetaData::<Identity, OFFSET>,
            VideoProcessorGetOutputHDRMetaData: VideoProcessorGetOutputHDRMetaData::<Identity, OFFSET>,
            VideoProcessorSetStreamHDRMetaData: VideoProcessorSetStreamHDRMetaData::<Identity, OFFSET>,
            VideoProcessorGetStreamHDRMetaData: VideoProcessorGetStreamHDRMetaData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext2 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11VideoContext as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11VideoContext1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11VideoContext2 {}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::define_interface!(ID3D11VideoContext3, ID3D11VideoContext3_Vtbl, 0xa9e2faa0_cb39_418f_a0b7_d8aad4de672e);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl core::ops::Deref for ID3D11VideoContext3 {
    type Target = ID3D11VideoContext2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::interface_hierarchy!(ID3D11VideoContext3, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11VideoContext, super::d3d11_1::ID3D11VideoContext1, ID3D11VideoContext2);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl ID3D11VideoContext3 {
    pub unsafe fn DecoderBeginFrame1<P0, P1>(&self, pdecoder: P0, pview: P1, contentkeysize: u32, pcontentkey: Option<*const core::ffi::c_void>, numcomponenthistograms: u32, phistogramoffsets: Option<*const u32>, pphistogrambuffers: Option<*const Option<super::d3d11::ID3D11Buffer>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoDecoder>,
        P1: windows_core::Param<super::d3d11::ID3D11VideoDecoderOutputView>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecoderBeginFrame1)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pview.param().abi(), contentkeysize, pcontentkey.unwrap_or(core::mem::zeroed()) as _, numcomponenthistograms, phistogramoffsets.unwrap_or(core::mem::zeroed()) as _, pphistogrambuffers.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SubmitDecoderBuffers2<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC2]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11VideoDecoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubmitDecoderBuffers2)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), pbufferdesc.len().try_into().unwrap(), core::mem::transmute(pbufferdesc.as_ptr())) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoContext3_Vtbl {
    pub base__: ID3D11VideoContext2_Vtbl,
    pub DecoderBeginFrame1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *const u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubmitDecoderBuffers2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait ID3D11VideoContext3_Impl: ID3D11VideoContext2_Impl {
    fn DecoderBeginFrame1(&self, pdecoder: windows_core::Ref<super::d3d11::ID3D11VideoDecoder>, pview: windows_core::Ref<super::d3d11::ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const Option<super::d3d11::ID3D11Buffer>) -> windows_core::Result<()>;
    fn SubmitDecoderBuffers2(&self, pdecoder: windows_core::Ref<super::d3d11::ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl ID3D11VideoContext3_Vtbl {
    pub const fn new<Identity: ID3D11VideoContext3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DecoderBeginFrame1<Identity: ID3D11VideoContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, pview: *mut core::ffi::c_void, contentkeysize: u32, pcontentkey: *const core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext3_Impl::DecoderBeginFrame1(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&pview), core::mem::transmute_copy(&contentkeysize), core::mem::transmute_copy(&pcontentkey), core::mem::transmute_copy(&numcomponenthistograms), core::mem::transmute_copy(&phistogramoffsets), core::mem::transmute_copy(&pphistogrambuffers)).into()
            }
        }
        unsafe extern "system" fn SubmitDecoderBuffers2<Identity: ID3D11VideoContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext3_Impl::SubmitDecoderBuffers2(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&pbufferdesc)).into()
            }
        }
        Self {
            base__: ID3D11VideoContext2_Vtbl::new::<Identity, OFFSET>(),
            DecoderBeginFrame1: DecoderBeginFrame1::<Identity, OFFSET>,
            SubmitDecoderBuffers2: SubmitDecoderBuffers2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext3 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11VideoContext as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11VideoContext1 as windows_core::Interface>::IID || iid == &<ID3D11VideoContext2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgi1_5", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11VideoContext3 {}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::define_interface!(ID3D11VideoDevice2, ID3D11VideoDevice2_Vtbl, 0x59c0cb01_35f0_4a70_8f67_87905c906a53);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl core::ops::Deref for ID3D11VideoDevice2 {
    type Target = super::d3d11_1::ID3D11VideoDevice1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::interface_hierarchy!(ID3D11VideoDevice2, windows_core::IUnknown, super::d3d11::ID3D11VideoDevice, super::d3d11_1::ID3D11VideoDevice1);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl ID3D11VideoDevice2 {
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, pfeaturesupportdata as _, featuresupportdatasize) }
    }
    pub unsafe fn NegotiateCryptoSessionKeyExchangeMT<P0>(&self, pcryptosession: P0, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11CryptoSession>,
    {
        unsafe { (windows_core::Interface::vtable(self).NegotiateCryptoSessionKeyExchangeMT)(windows_core::Interface::as_raw(self), pcryptosession.param().abi(), flags, datasize, pdata as _) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11VideoDevice2_Vtbl {
    pub base__: super::d3d11_1::ID3D11VideoDevice1_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_FEATURE_VIDEO, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NegotiateCryptoSessionKeyExchangeMT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D11VideoDevice2_Impl: super::d3d11_1::ID3D11VideoDevice1_Impl {
    fn CheckFeatureSupport(&self, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
    fn NegotiateCryptoSessionKeyExchangeMT(&self, pcryptosession: windows_core::Ref<super::d3d11::ID3D11CryptoSession>, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D11VideoDevice2_Vtbl {
    pub const fn new<Identity: ID3D11VideoDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ID3D11VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice2_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
            }
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchangeMT<Identity: ID3D11VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice2_Impl::NegotiateCryptoSessionKeyExchangeMT(this, core::mem::transmute_copy(&pcryptosession), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: super::d3d11_1::ID3D11VideoDevice1_Vtbl::new::<Identity, OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET>,
            NegotiateCryptoSessionKeyExchangeMT: NegotiateCryptoSessionKeyExchangeMT::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDevice2 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11VideoDevice as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11VideoDevice1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D11VideoDevice2 {}
