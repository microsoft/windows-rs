#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
pub const Desktop: RdpSessionType = 0;
pub const E_DUPLICATE_WINDOW_HINT: i32 = -2147024713;
pub const E_MAPPEDRENDERER_SHUTDOWN: i32 = -2147019873;
windows_core::imp::define_interface!(IWTSBitmapRenderService, IWTSBitmapRenderService_Vtbl, 0xea326091_05fe_40c1_b49c_3d2ef4626a0e);
windows_core::imp::interface_hierarchy!(IWTSBitmapRenderService, windows_core::IUnknown);
impl IWTSBitmapRenderService {
    pub unsafe fn GetMappedRenderer<P1>(&self, mappingid: u64, pmappedrenderercallback: P1) -> windows_core::Result<IWTSBitmapRenderer>
    where
        P1: windows_core::Param<IWTSBitmapRendererCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMappedRenderer)(windows_core::Interface::as_raw(self), mappingid, pmappedrenderercallback.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMappedRenderer: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSBitmapRenderService_Impl: windows_core::IUnknownImpl {
    fn GetMappedRenderer(&self, mappingid: u64, pmappedrenderercallback: windows_core::Ref<IWTSBitmapRendererCallback>) -> windows_core::Result<IWTSBitmapRenderer>;
}
impl IWTSBitmapRenderService_Vtbl {
    pub const fn new<Identity: IWTSBitmapRenderService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMappedRenderer<Identity: IWTSBitmapRenderService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mappingid: u64, pmappedrenderercallback: *mut core::ffi::c_void, ppmappedrenderer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSBitmapRenderService_Impl::GetMappedRenderer(this, core::mem::transmute_copy(&mappingid), core::mem::transmute_copy(&pmappedrenderercallback)) {
                    Ok(ok__) => {
                        ppmappedrenderer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMappedRenderer: GetMappedRenderer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRenderService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSBitmapRenderService {}
windows_core::imp::define_interface!(IWTSBitmapRenderer, IWTSBitmapRenderer_Vtbl, 0x5b7acc97_f3c9_46f7_8c5b_fa685d3441b1);
windows_core::imp::interface_hierarchy!(IWTSBitmapRenderer, windows_core::IUnknown);
impl IWTSBitmapRenderer {
    pub unsafe fn Render(&self, imageformat: windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), imageformat, dwwidth, dwheight, cbstride, cbimagebuffer, pimagebuffer) }
    }
    pub unsafe fn GetRendererStatistics(&self) -> windows_core::Result<BITMAP_RENDERER_STATISTICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRendererStatistics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveMapping(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveMapping)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, i32, u32, *const u8) -> windows_core::HRESULT,
    pub GetRendererStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BITMAP_RENDERER_STATISTICS) -> windows_core::HRESULT,
    pub RemoveMapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSBitmapRenderer_Impl: windows_core::IUnknownImpl {
    fn Render(&self, imageformat: &windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> windows_core::Result<()>;
    fn GetRendererStatistics(&self) -> windows_core::Result<BITMAP_RENDERER_STATISTICS>;
    fn RemoveMapping(&self) -> windows_core::Result<()>;
}
impl IWTSBitmapRenderer_Vtbl {
    pub const fn new<Identity: IWTSBitmapRenderer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Render<Identity: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageformat: windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSBitmapRenderer_Impl::Render(this, core::mem::transmute(&imageformat), core::mem::transmute_copy(&dwwidth), core::mem::transmute_copy(&dwheight), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbimagebuffer), core::mem::transmute_copy(&pimagebuffer)).into()
            }
        }
        unsafe extern "system" fn GetRendererStatistics<Identity: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSBitmapRenderer_Impl::GetRendererStatistics(this) {
                    Ok(ok__) => {
                        pstatistics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveMapping<Identity: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSBitmapRenderer_Impl::RemoveMapping(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Render: Render::<Identity, OFFSET>,
            GetRendererStatistics: GetRendererStatistics::<Identity, OFFSET>,
            RemoveMapping: RemoveMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRenderer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSBitmapRenderer {}
windows_core::imp::define_interface!(IWTSBitmapRendererCallback, IWTSBitmapRendererCallback_Vtbl, 0xd782928e_fe4e_4e77_ae90_9cd0b3e3b353);
windows_core::imp::interface_hierarchy!(IWTSBitmapRendererCallback, windows_core::IUnknown);
impl IWTSBitmapRendererCallback {
    #[cfg(feature = "windef")]
    pub unsafe fn OnTargetSizeChanged(&self, rcnewsize: super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTargetSizeChanged)(windows_core::Interface::as_raw(self), rcnewsize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRendererCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub OnTargetSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    OnTargetSizeChanged: usize,
}
#[cfg(feature = "windef")]
pub trait IWTSBitmapRendererCallback_Impl: windows_core::IUnknownImpl {
    fn OnTargetSizeChanged(&self, rcnewsize: &super::RECT) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IWTSBitmapRendererCallback_Vtbl {
    pub const fn new<Identity: IWTSBitmapRendererCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTargetSizeChanged<Identity: IWTSBitmapRendererCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcnewsize: super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSBitmapRendererCallback_Impl::OnTargetSizeChanged(this, core::mem::transmute(&rcnewsize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTargetSizeChanged: OnTargetSizeChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRendererCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IWTSBitmapRendererCallback {}
windows_core::imp::define_interface!(IWTSListener, IWTSListener_Vtbl, 0xa1230206_9a39_4d58_8674_cdb4dff4e73b);
windows_core::imp::interface_hierarchy!(IWTSListener, windows_core::IUnknown);
impl IWTSListener {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetConfiguration(&self) -> windows_core::Result<super::IPropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetConfiguration: usize,
}
#[cfg(feature = "oaidl")]
pub trait IWTSListener_Impl: windows_core::IUnknownImpl {
    fn GetConfiguration(&self) -> windows_core::Result<super::IPropertyBag>;
}
#[cfg(feature = "oaidl")]
impl IWTSListener_Vtbl {
    pub const fn new<Identity: IWTSListener_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConfiguration<Identity: IWTSListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSListener_Impl::GetConfiguration(this) {
                    Ok(ok__) => {
                        pppropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetConfiguration: GetConfiguration::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSListener as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IWTSListener {}
windows_core::imp::define_interface!(IWTSListenerCallback, IWTSListenerCallback_Vtbl, 0xa1230203_d6a7_11d8_b9fd_000bdbd1f198);
windows_core::imp::interface_hierarchy!(IWTSListenerCallback, windows_core::IUnknown);
impl IWTSListenerCallback {
    pub unsafe fn OnNewChannelConnection<P0>(&self, pchannel: P0, data: &windows_core::BSTR, pbaccept: *mut windows_core::BOOL, ppcallback: *mut Option<IWTSVirtualChannelCallback>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWTSVirtualChannel>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnNewChannelConnection)(windows_core::Interface::as_raw(self), pchannel.param().abi(), core::mem::transmute_copy(data), pbaccept as _, core::mem::transmute(ppcallback)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListenerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNewChannelConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSListenerCallback_Impl: windows_core::IUnknownImpl {
    fn OnNewChannelConnection(&self, pchannel: windows_core::Ref<IWTSVirtualChannel>, data: &windows_core::BSTR, pbaccept: *mut windows_core::BOOL, ppcallback: windows_core::OutRef<IWTSVirtualChannelCallback>) -> windows_core::Result<()>;
}
impl IWTSListenerCallback_Vtbl {
    pub const fn new<Identity: IWTSListenerCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNewChannelConnection<Identity: IWTSListenerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, data: *mut core::ffi::c_void, pbaccept: *mut windows_core::BOOL, ppcallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSListenerCallback_Impl::OnNewChannelConnection(this, core::mem::transmute_copy(&pchannel), core::mem::transmute(&data), core::mem::transmute_copy(&pbaccept), core::mem::transmute_copy(&ppcallback)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNewChannelConnection: OnNewChannelConnection::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSListenerCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSListenerCallback {}
windows_core::imp::define_interface!(IWTSPlugin, IWTSPlugin_Vtbl, 0xa1230201_1439_4e62_a414_190d0ac3d40e);
windows_core::imp::interface_hierarchy!(IWTSPlugin, windows_core::IUnknown);
impl IWTSPlugin {
    pub unsafe fn Initialize<P0>(&self, pchannelmgr: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWTSVirtualChannelManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pchannelmgr.param().abi()) }
    }
    pub unsafe fn Connected(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Connected)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Disconnected(&self, dwdisconnectcode: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disconnected)(windows_core::Interface::as_raw(self), dwdisconnectcode) }
    }
    pub unsafe fn Terminated(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Terminated)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Terminated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSPlugin_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pchannelmgr: windows_core::Ref<IWTSVirtualChannelManager>) -> windows_core::Result<()>;
    fn Connected(&self) -> windows_core::Result<()>;
    fn Disconnected(&self, dwdisconnectcode: u32) -> windows_core::Result<()>;
    fn Terminated(&self) -> windows_core::Result<()>;
}
impl IWTSPlugin_Vtbl {
    pub const fn new<Identity: IWTSPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannelmgr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSPlugin_Impl::Initialize(this, core::mem::transmute_copy(&pchannelmgr)).into()
            }
        }
        unsafe extern "system" fn Connected<Identity: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSPlugin_Impl::Connected(this).into()
            }
        }
        unsafe extern "system" fn Disconnected<Identity: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdisconnectcode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSPlugin_Impl::Disconnected(this, core::mem::transmute_copy(&dwdisconnectcode)).into()
            }
        }
        unsafe extern "system" fn Terminated<Identity: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSPlugin_Impl::Terminated(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Connected: Connected::<Identity, OFFSET>,
            Disconnected: Disconnected::<Identity, OFFSET>,
            Terminated: Terminated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSPlugin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSPlugin {}
windows_core::imp::define_interface!(IWTSPluginServiceProvider, IWTSPluginServiceProvider_Vtbl, 0xd3e07363_087c_476c_86a7_dbb15f46ddb4);
windows_core::imp::interface_hierarchy!(IWTSPluginServiceProvider, windows_core::IUnknown);
impl IWTSPluginServiceProvider {
    pub unsafe fn GetService(&self, serviceid: windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), serviceid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPluginServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSPluginServiceProvider_Impl: windows_core::IUnknownImpl {
    fn GetService(&self, serviceid: &windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl IWTSPluginServiceProvider_Vtbl {
    pub const fn new<Identity: IWTSPluginServiceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetService<Identity: IWTSPluginServiceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: windows_core::GUID, ppunkobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSPluginServiceProvider_Impl::GetService(this, core::mem::transmute(&serviceid)) {
                    Ok(ok__) => {
                        ppunkobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSPluginServiceProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSPluginServiceProvider {}
windows_core::imp::define_interface!(IWTSVirtualChannel, IWTSVirtualChannel_Vtbl, 0xa1230207_d6a7_11d8_b9fd_000bdbd1f198);
windows_core::imp::interface_hierarchy!(IWTSVirtualChannel, windows_core::IUnknown);
impl IWTSVirtualChannel {
    pub unsafe fn Write<P2>(&self, cbsize: u32, pbuffer: *const u8, preserved: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), cbsize, pbuffer, preserved.param().abi()) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSVirtualChannel_Impl: windows_core::IUnknownImpl {
    fn Write(&self, cbsize: u32, pbuffer: *const u8, preserved: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IWTSVirtualChannel_Vtbl {
    pub const fn new<Identity: IWTSVirtualChannel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Write<Identity: IWTSVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSVirtualChannel_Impl::Write(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&preserved)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWTSVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSVirtualChannel_Impl::Close(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Write: Write::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSVirtualChannel {}
windows_core::imp::define_interface!(IWTSVirtualChannelCallback, IWTSVirtualChannelCallback_Vtbl, 0xa1230204_d6a7_11d8_b9fd_000bdbd1f198);
windows_core::imp::interface_hierarchy!(IWTSVirtualChannelCallback, windows_core::IUnknown);
impl IWTSVirtualChannelCallback {
    pub unsafe fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnDataReceived)(windows_core::Interface::as_raw(self), cbsize, pbuffer) }
    }
    pub unsafe fn OnClose(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub OnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSVirtualChannelCallback_Impl: windows_core::IUnknownImpl {
    fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
    fn OnClose(&self) -> windows_core::Result<()>;
}
impl IWTSVirtualChannelCallback_Vtbl {
    pub const fn new<Identity: IWTSVirtualChannelCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDataReceived<Identity: IWTSVirtualChannelCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSVirtualChannelCallback_Impl::OnDataReceived(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn OnClose<Identity: IWTSVirtualChannelCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSVirtualChannelCallback_Impl::OnClose(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataReceived: OnDataReceived::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannelCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSVirtualChannelCallback {}
windows_core::imp::define_interface!(IWTSVirtualChannelManager, IWTSVirtualChannelManager_Vtbl, 0xa1230205_d6a7_11d8_b9fd_000bdbd1f198);
windows_core::imp::interface_hierarchy!(IWTSVirtualChannelManager, windows_core::IUnknown);
impl IWTSVirtualChannelManager {
    pub unsafe fn CreateListener<P2>(&self, pszchannelname: *const i8, uflags: u32, plistenercallback: P2) -> windows_core::Result<IWTSListener>
    where
        P2: windows_core::Param<IWTSListenerCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateListener)(windows_core::Interface::as_raw(self), pszchannelname, uflags, plistenercallback.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateListener: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSVirtualChannelManager_Impl: windows_core::IUnknownImpl {
    fn CreateListener(&self, pszchannelname: *const i8, uflags: u32, plistenercallback: windows_core::Ref<IWTSListenerCallback>) -> windows_core::Result<IWTSListener>;
}
impl IWTSVirtualChannelManager_Vtbl {
    pub const fn new<Identity: IWTSVirtualChannelManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateListener<Identity: IWTSVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszchannelname: *const i8, uflags: u32, plistenercallback: *mut core::ffi::c_void, pplistener: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSVirtualChannelManager_Impl::CreateListener(this, core::mem::transmute_copy(&pszchannelname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&plistenercallback)) {
                    Ok(ok__) => {
                        pplistener.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateListener: CreateListener::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannelManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSVirtualChannelManager {}
windows_core::imp::define_interface!(IWTSWindowChangedCallback, IWTSWindowChangedCallback_Vtbl, 0xe8a47fd3_1af6_4d7f_8e15_515ccae1c00c);
windows_core::imp::interface_hierarchy!(IWTSWindowChangedCallback, windows_core::IUnknown);
impl IWTSWindowChangedCallback {
    #[cfg(feature = "windef")]
    pub unsafe fn WindowChanged(&self, windowinfo: *const WTSWindowInfo) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WindowChanged)(windows_core::Interface::as_raw(self), windowinfo) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSWindowChangedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub WindowChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTSWindowInfo) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    WindowChanged: usize,
}
#[cfg(feature = "windef")]
pub trait IWTSWindowChangedCallback_Impl: windows_core::IUnknownImpl {
    fn WindowChanged(&self, windowinfo: *const WTSWindowInfo) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IWTSWindowChangedCallback_Vtbl {
    pub const fn new<Identity: IWTSWindowChangedCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowChanged<Identity: IWTSWindowChangedCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowinfo: *const WTSWindowInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSWindowChangedCallback_Impl::WindowChanged(this, core::mem::transmute_copy(&windowinfo)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), WindowChanged: WindowChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSWindowChangedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IWTSWindowChangedCallback {}
windows_core::imp::define_interface!(IWTSWindowInfoService, IWTSWindowInfoService_Vtbl, 0x2a002c7b_0120_4d04_bfb1_3c73110c8581);
windows_core::imp::interface_hierarchy!(IWTSWindowInfoService, windows_core::IUnknown);
impl IWTSWindowInfoService {
    #[cfg(feature = "windef")]
    pub unsafe fn GetWindowInfo(&self, remotehwnd: super::HWND, windowinfo: *mut WTSWindowInfo) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWindowInfo)(windows_core::Interface::as_raw(self), remotehwnd, windowinfo as _) }
    }
    pub unsafe fn GetRdpClientProcessId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRdpClientProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRdpSessionType(&self) -> windows_core::Result<RdpSessionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRdpSessionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SubscribeWindowChanged<P1>(&self, remotehwnd: super::HWND, windowchanged: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWTSWindowChangedCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubscribeWindowChanged)(windows_core::Interface::as_raw(self), remotehwnd, windowchanged.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn UnsubscribeWindowChanged<P1>(&self, remotehwnd: super::HWND, windowchanged: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWTSWindowChangedCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnsubscribeWindowChanged)(windows_core::Interface::as_raw(self), remotehwnd, windowchanged.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSWindowInfoService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetWindowInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut WTSWindowInfo) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWindowInfo: usize,
    pub GetRdpClientProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRdpSessionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RdpSessionType) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SubscribeWindowChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SubscribeWindowChanged: usize,
    #[cfg(feature = "windef")]
    pub UnsubscribeWindowChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    UnsubscribeWindowChanged: usize,
}
#[cfg(feature = "windef")]
pub trait IWTSWindowInfoService_Impl: windows_core::IUnknownImpl {
    fn GetWindowInfo(&self, remotehwnd: super::HWND, windowinfo: *mut WTSWindowInfo) -> windows_core::Result<()>;
    fn GetRdpClientProcessId(&self) -> windows_core::Result<u32>;
    fn GetRdpSessionType(&self) -> windows_core::Result<RdpSessionType>;
    fn SubscribeWindowChanged(&self, remotehwnd: super::HWND, windowchanged: windows_core::Ref<IWTSWindowChangedCallback>) -> windows_core::Result<()>;
    fn UnsubscribeWindowChanged(&self, remotehwnd: super::HWND, windowchanged: windows_core::Ref<IWTSWindowChangedCallback>) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IWTSWindowInfoService_Vtbl {
    pub const fn new<Identity: IWTSWindowInfoService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindowInfo<Identity: IWTSWindowInfoService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotehwnd: super::HWND, windowinfo: *mut WTSWindowInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSWindowInfoService_Impl::GetWindowInfo(this, core::mem::transmute_copy(&remotehwnd), core::mem::transmute_copy(&windowinfo)).into()
            }
        }
        unsafe extern "system" fn GetRdpClientProcessId<Identity: IWTSWindowInfoService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSWindowInfoService_Impl::GetRdpClientProcessId(this) {
                    Ok(ok__) => {
                        processid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRdpSessionType<Identity: IWTSWindowInfoService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessiontype: *mut RdpSessionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSWindowInfoService_Impl::GetRdpSessionType(this) {
                    Ok(ok__) => {
                        sessiontype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubscribeWindowChanged<Identity: IWTSWindowInfoService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotehwnd: super::HWND, windowchanged: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSWindowInfoService_Impl::SubscribeWindowChanged(this, core::mem::transmute_copy(&remotehwnd), core::mem::transmute_copy(&windowchanged)).into()
            }
        }
        unsafe extern "system" fn UnsubscribeWindowChanged<Identity: IWTSWindowInfoService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotehwnd: super::HWND, windowchanged: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSWindowInfoService_Impl::UnsubscribeWindowChanged(this, core::mem::transmute_copy(&remotehwnd), core::mem::transmute_copy(&windowchanged)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindowInfo: GetWindowInfo::<Identity, OFFSET>,
            GetRdpClientProcessId: GetRdpClientProcessId::<Identity, OFFSET>,
            GetRdpSessionType: GetRdpSessionType::<Identity, OFFSET>,
            SubscribeWindowChanged: SubscribeWindowChanged::<Identity, OFFSET>,
            UnsubscribeWindowChanged: UnsubscribeWindowChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSWindowInfoService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IWTSWindowInfoService {}
pub type PBITMAP_RENDERER_STATISTICS = *mut BITMAP_RENDERER_STATISTICS;
pub type RdpSessionType = i32;
pub const RemoteApp: RdpSessionType = 1;
pub const TS_VC_LISTENER_STATIC_CHANNEL: i32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTSWindowInfo {
    pub Hwnd: super::HWND,
    pub Height: i32,
    pub Width: i32,
    pub ViewWidth: i32,
    pub ViewHeight: i32,
    pub ViewOffsetX: i32,
    pub ViewOffsetY: i32,
    pub Scale: f32,
}
pub const WTS_PROPERTY_DEFAULT_CONFIG: windows_core::PCWSTR = windows_core::w!("DefaultConfig");
