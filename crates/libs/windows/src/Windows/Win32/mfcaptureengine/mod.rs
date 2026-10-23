pub const CLSID_MFCaptureEngine: windows_core::GUID = windows_core::GUID::from_u128(0xefce38d3_8914_4674_a7df_ae1b3d654b8a);
pub const CLSID_MFCaptureEngineClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0xefce38d3_8914_4674_a7df_ae1b3d654b8a);
windows_core::imp::define_interface!(IMFCaptureEngine, IMFCaptureEngine_Vtbl, 0xa6bba433_176b_48b2_b375_53aa03473207);
windows_core::imp::interface_hierarchy!(IMFCaptureEngine, windows_core::IUnknown);
impl IMFCaptureEngine {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, peventcallback: P0, pattributes: P1, paudiosource: P2, pvideosource: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFCaptureEngineOnEventCallback>,
        P1: windows_core::Param<super::IMFAttributes>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), peventcallback.param().abi(), pattributes.param().abi(), paudiosource.param().abi(), pvideosource.param().abi()) }
    }
    pub unsafe fn StartPreview(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartPreview)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn StopPreview(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopPreview)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn StartRecord(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartRecord)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn StopRecord(&self, bfinalize: bool, bflushunprocessedsamples: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopRecord)(windows_core::Interface::as_raw(self), bfinalize.into(), bflushunprocessedsamples.into()) }
    }
    pub unsafe fn TakePhoto(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TakePhoto)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSink(&self, mfcaptureenginesinktype: MF_CAPTURE_ENGINE_SINK_TYPE) -> windows_core::Result<IMFCaptureSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSink)(windows_core::Interface::as_raw(self), mfcaptureenginesinktype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<IMFCaptureSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    Initialize: usize,
    pub StartPreview: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopPreview: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartRecord: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopRecord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub TakePhoto: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSink: unsafe extern "system" fn(*mut core::ffi::c_void, MF_CAPTURE_ENGINE_SINK_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFCaptureEngine_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, peventcallback: windows_core::Ref<IMFCaptureEngineOnEventCallback>, pattributes: windows_core::Ref<super::IMFAttributes>, paudiosource: windows_core::Ref<windows_core::IUnknown>, pvideosource: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn StartPreview(&self) -> windows_core::Result<()>;
    fn StopPreview(&self) -> windows_core::Result<()>;
    fn StartRecord(&self) -> windows_core::Result<()>;
    fn StopRecord(&self, bfinalize: windows_core::BOOL, bflushunprocessedsamples: windows_core::BOOL) -> windows_core::Result<()>;
    fn TakePhoto(&self) -> windows_core::Result<()>;
    fn GetSink(&self, mfcaptureenginesinktype: MF_CAPTURE_ENGINE_SINK_TYPE) -> windows_core::Result<IMFCaptureSink>;
    fn GetSource(&self) -> windows_core::Result<IMFCaptureSource>;
}
#[cfg(feature = "mfobjects")]
impl IMFCaptureEngine_Vtbl {
    pub const fn new<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventcallback: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void, paudiosource: *mut core::ffi::c_void, pvideosource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngine_Impl::Initialize(this, core::mem::transmute_copy(&peventcallback), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&paudiosource), core::mem::transmute_copy(&pvideosource)).into()
            }
        }
        unsafe extern "system" fn StartPreview<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngine_Impl::StartPreview(this).into()
            }
        }
        unsafe extern "system" fn StopPreview<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngine_Impl::StopPreview(this).into()
            }
        }
        unsafe extern "system" fn StartRecord<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngine_Impl::StartRecord(this).into()
            }
        }
        unsafe extern "system" fn StopRecord<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfinalize: windows_core::BOOL, bflushunprocessedsamples: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngine_Impl::StopRecord(this, core::mem::transmute_copy(&bfinalize), core::mem::transmute_copy(&bflushunprocessedsamples)).into()
            }
        }
        unsafe extern "system" fn TakePhoto<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngine_Impl::TakePhoto(this).into()
            }
        }
        unsafe extern "system" fn GetSink<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mfcaptureenginesinktype: MF_CAPTURE_ENGINE_SINK_TYPE, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCaptureEngine_Impl::GetSink(this, core::mem::transmute_copy(&mfcaptureenginesinktype)) {
                    Ok(ok__) => {
                        ppsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSource<Identity: IMFCaptureEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCaptureEngine_Impl::GetSource(this) {
                    Ok(ok__) => {
                        ppsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            StartPreview: StartPreview::<Identity, OFFSET>,
            StopPreview: StopPreview::<Identity, OFFSET>,
            StartRecord: StartRecord::<Identity, OFFSET>,
            StopRecord: StopRecord::<Identity, OFFSET>,
            TakePhoto: TakePhoto::<Identity, OFFSET>,
            GetSink: GetSink::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureEngine as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFCaptureEngine {}
windows_core::imp::define_interface!(IMFCaptureEngineClassFactory, IMFCaptureEngineClassFactory_Vtbl, 0x8f02d140_56fc_4302_a705_3a97c78be779);
windows_core::imp::interface_hierarchy!(IMFCaptureEngineClassFactory, windows_core::IUnknown);
impl IMFCaptureEngineClassFactory {
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn CreateInstance(&self, clsid: *const super::IID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), clsid, riid, ppvobject as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureEngineClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    CreateInstance: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub trait IMFCaptureEngineClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, clsid: *const super::IID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
impl IMFCaptureEngineClassFactory_Vtbl {
    pub const fn new<Identity: IMFCaptureEngineClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IMFCaptureEngineClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const super::IID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngineClassFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureEngineClassFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
impl windows_core::RuntimeName for IMFCaptureEngineClassFactory {}
windows_core::imp::define_interface!(IMFCaptureEngineOnEventCallback, IMFCaptureEngineOnEventCallback_Vtbl, 0xaeda51c0_9025_4983_9012_de597b88b089);
windows_core::imp::interface_hierarchy!(IMFCaptureEngineOnEventCallback, windows_core::IUnknown);
impl IMFCaptureEngineOnEventCallback {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnEvent<P0>(&self, pevent: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), pevent.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureEngineOnEventCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    OnEvent: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFCaptureEngineOnEventCallback_Impl: windows_core::IUnknownImpl {
    fn OnEvent(&self, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFCaptureEngineOnEventCallback_Vtbl {
    pub const fn new<Identity: IMFCaptureEngineOnEventCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEvent<Identity: IMFCaptureEngineOnEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngineOnEventCallback_Impl::OnEvent(this, core::mem::transmute_copy(&pevent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureEngineOnEventCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFCaptureEngineOnEventCallback {}
windows_core::imp::define_interface!(IMFCaptureEngineOnSampleCallback, IMFCaptureEngineOnSampleCallback_Vtbl, 0x52150b82_ab39_4467_980f_e48bf0822ecd);
windows_core::imp::interface_hierarchy!(IMFCaptureEngineOnSampleCallback, windows_core::IUnknown);
impl IMFCaptureEngineOnSampleCallback {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnSample<P0>(&self, psample: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSample)(windows_core::Interface::as_raw(self), psample.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureEngineOnSampleCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub OnSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    OnSample: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFCaptureEngineOnSampleCallback_Impl: windows_core::IUnknownImpl {
    fn OnSample(&self, psample: windows_core::Ref<super::IMFSample>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFCaptureEngineOnSampleCallback_Vtbl {
    pub const fn new<Identity: IMFCaptureEngineOnSampleCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSample<Identity: IMFCaptureEngineOnSampleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngineOnSampleCallback_Impl::OnSample(this, core::mem::transmute_copy(&psample)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSample: OnSample::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureEngineOnSampleCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFCaptureEngineOnSampleCallback {}
windows_core::imp::define_interface!(IMFCaptureEngineOnSampleCallback2, IMFCaptureEngineOnSampleCallback2_Vtbl, 0xe37ceed7_340f_4514_9f4d_9c2ae026100b);
impl core::ops::Deref for IMFCaptureEngineOnSampleCallback2 {
    type Target = IMFCaptureEngineOnSampleCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFCaptureEngineOnSampleCallback2, windows_core::IUnknown, IMFCaptureEngineOnSampleCallback);
impl IMFCaptureEngineOnSampleCallback2 {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnSynchronizedEvent<P0>(&self, pevent: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSynchronizedEvent)(windows_core::Interface::as_raw(self), pevent.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureEngineOnSampleCallback2_Vtbl {
    pub base__: IMFCaptureEngineOnSampleCallback_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub OnSynchronizedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    OnSynchronizedEvent: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFCaptureEngineOnSampleCallback2_Impl: IMFCaptureEngineOnSampleCallback_Impl {
    fn OnSynchronizedEvent(&self, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFCaptureEngineOnSampleCallback2_Vtbl {
    pub const fn new<Identity: IMFCaptureEngineOnSampleCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSynchronizedEvent<Identity: IMFCaptureEngineOnSampleCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureEngineOnSampleCallback2_Impl::OnSynchronizedEvent(this, core::mem::transmute_copy(&pevent)).into()
            }
        }
        Self { base__: IMFCaptureEngineOnSampleCallback_Vtbl::new::<Identity, OFFSET>(), OnSynchronizedEvent: OnSynchronizedEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureEngineOnSampleCallback2 as windows_core::Interface>::IID || iid == &<IMFCaptureEngineOnSampleCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFCaptureEngineOnSampleCallback2 {}
windows_core::imp::define_interface!(IMFCapturePhotoSink, IMFCapturePhotoSink_Vtbl, 0xd2d43cc8_48bb_4aa7_95db_10c06977e777);
impl core::ops::Deref for IMFCapturePhotoSink {
    type Target = IMFCaptureSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFCapturePhotoSink, windows_core::IUnknown, IMFCaptureSink);
impl IMFCapturePhotoSink {
    pub unsafe fn SetOutputFileName<P0>(&self, filename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFileName)(windows_core::Interface::as_raw(self), filename.param().abi()) }
    }
    pub unsafe fn SetSampleCallback<P0>(&self, pcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFCaptureEngineOnSampleCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSampleCallback)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetOutputByteStream<P0>(&self, pbytestream: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFByteStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputByteStream)(windows_core::Interface::as_raw(self), pbytestream.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCapturePhotoSink_Vtbl {
    pub base__: IMFCaptureSink_Vtbl,
    pub SetOutputFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSampleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub SetOutputByteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetOutputByteStream: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
pub trait IMFCapturePhotoSink_Impl: IMFCaptureSink_Impl {
    fn SetOutputFileName(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSampleCallback(&self, pcallback: windows_core::Ref<IMFCaptureEngineOnSampleCallback>) -> windows_core::Result<()>;
    fn SetOutputByteStream(&self, pbytestream: windows_core::Ref<super::IMFByteStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
impl IMFCapturePhotoSink_Vtbl {
    pub const fn new<Identity: IMFCapturePhotoSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOutputFileName<Identity: IMFCapturePhotoSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePhotoSink_Impl::SetOutputFileName(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn SetSampleCallback<Identity: IMFCapturePhotoSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePhotoSink_Impl::SetSampleCallback(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn SetOutputByteStream<Identity: IMFCapturePhotoSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePhotoSink_Impl::SetOutputByteStream(this, core::mem::transmute_copy(&pbytestream)).into()
            }
        }
        Self {
            base__: IMFCaptureSink_Vtbl::new::<Identity, OFFSET>(),
            SetOutputFileName: SetOutputFileName::<Identity, OFFSET>,
            SetSampleCallback: SetSampleCallback::<Identity, OFFSET>,
            SetOutputByteStream: SetOutputByteStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCapturePhotoSink as windows_core::Interface>::IID || iid == &<IMFCaptureSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFCapturePhotoSink {}
windows_core::imp::define_interface!(IMFCapturePreviewSink, IMFCapturePreviewSink_Vtbl, 0x77346cfd_5b49_4d73_ace0_5b52a859f2e0);
impl core::ops::Deref for IMFCapturePreviewSink {
    type Target = IMFCaptureSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFCapturePreviewSink, windows_core::IUnknown, IMFCaptureSink);
impl IMFCapturePreviewSink {
    #[cfg(feature = "winnt")]
    pub unsafe fn SetRenderHandle(&self, handle: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderHandle)(windows_core::Interface::as_raw(self), handle) }
    }
    pub unsafe fn SetRenderSurface<P0>(&self, psurface: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRenderSurface)(windows_core::Interface::as_raw(self), psurface.param().abi()) }
    }
    #[cfg(all(feature = "mfidl", feature = "shared", feature = "windef"))]
    pub unsafe fn UpdateVideo(&self, psrc: Option<*const super::MFVideoNormalizedRect>, pdst: Option<*const super::RECT>, pborderclr: Option<*const super::COLORREF>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateVideo)(windows_core::Interface::as_raw(self), psrc.unwrap_or(core::mem::zeroed()) as _, pdst.unwrap_or(core::mem::zeroed()) as _, pborderclr.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSampleCallback<P1>(&self, dwstreamsinkindex: super::DWORD, pcallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFCaptureEngineOnSampleCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSampleCallback)(windows_core::Interface::as_raw(self), dwstreamsinkindex, pcallback.param().abi()) }
    }
    pub unsafe fn GetMirrorState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMirrorState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMirrorState(&self, fmirrorstate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMirrorState)(windows_core::Interface::as_raw(self), fmirrorstate.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRotation(&self, dwstreamindex: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRotation(&self, dwstreamindex: super::DWORD, dwrotationvalue: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), dwstreamindex, dwrotationvalue) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn SetCustomSink<P0>(&self, pmediasink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFMediaSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCustomSink)(windows_core::Interface::as_raw(self), pmediasink.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCapturePreviewSink_Vtbl {
    pub base__: IMFCaptureSink_Vtbl,
    #[cfg(feature = "winnt")]
    pub SetRenderHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetRenderHandle: usize,
    pub SetRenderSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "mfidl", feature = "shared", feature = "windef"))]
    pub UpdateVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MFVideoNormalizedRect, *const super::RECT, *const super::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "shared", feature = "windef")))]
    UpdateVideo: usize,
    #[cfg(feature = "shared")]
    pub SetSampleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSampleCallback: usize,
    pub GetMirrorState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetMirrorState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRotation: usize,
    #[cfg(feature = "shared")]
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRotation: usize,
    #[cfg(feature = "mfidl")]
    pub SetCustomSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    SetCustomSink: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait IMFCapturePreviewSink_Impl: IMFCaptureSink_Impl {
    fn SetRenderHandle(&self, handle: super::HANDLE) -> windows_core::Result<()>;
    fn SetRenderSurface(&self, psurface: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UpdateVideo(&self, psrc: *const super::MFVideoNormalizedRect, pdst: *const super::RECT, pborderclr: *const super::COLORREF) -> windows_core::Result<()>;
    fn SetSampleCallback(&self, dwstreamsinkindex: super::DWORD, pcallback: windows_core::Ref<IMFCaptureEngineOnSampleCallback>) -> windows_core::Result<()>;
    fn GetMirrorState(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetMirrorState(&self, fmirrorstate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetRotation(&self, dwstreamindex: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn SetRotation(&self, dwstreamindex: super::DWORD, dwrotationvalue: super::DWORD) -> windows_core::Result<()>;
    fn SetCustomSink(&self, pmediasink: windows_core::Ref<super::IMFMediaSink>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared", feature = "windef", feature = "winnt"))]
impl IMFCapturePreviewSink_Vtbl {
    pub const fn new<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRenderHandle<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePreviewSink_Impl::SetRenderHandle(this, core::mem::transmute_copy(&handle)).into()
            }
        }
        unsafe extern "system" fn SetRenderSurface<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePreviewSink_Impl::SetRenderSurface(this, core::mem::transmute_copy(&psurface)).into()
            }
        }
        unsafe extern "system" fn UpdateVideo<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *const super::MFVideoNormalizedRect, pdst: *const super::RECT, pborderclr: *const super::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePreviewSink_Impl::UpdateVideo(this, core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pborderclr)).into()
            }
        }
        unsafe extern "system" fn SetSampleCallback<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamsinkindex: super::DWORD, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePreviewSink_Impl::SetSampleCallback(this, core::mem::transmute_copy(&dwstreamsinkindex), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn GetMirrorState<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmirrorstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCapturePreviewSink_Impl::GetMirrorState(this) {
                    Ok(ok__) => {
                        pfmirrorstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMirrorState<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmirrorstate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePreviewSink_Impl::SetMirrorState(this, core::mem::transmute_copy(&fmirrorstate)).into()
            }
        }
        unsafe extern "system" fn GetRotation<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pdwrotationvalue: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCapturePreviewSink_Impl::GetRotation(this, core::mem::transmute_copy(&dwstreamindex)) {
                    Ok(ok__) => {
                        pdwrotationvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, dwrotationvalue: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePreviewSink_Impl::SetRotation(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwrotationvalue)).into()
            }
        }
        unsafe extern "system" fn SetCustomSink<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediasink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePreviewSink_Impl::SetCustomSink(this, core::mem::transmute_copy(&pmediasink)).into()
            }
        }
        Self {
            base__: IMFCaptureSink_Vtbl::new::<Identity, OFFSET>(),
            SetRenderHandle: SetRenderHandle::<Identity, OFFSET>,
            SetRenderSurface: SetRenderSurface::<Identity, OFFSET>,
            UpdateVideo: UpdateVideo::<Identity, OFFSET>,
            SetSampleCallback: SetSampleCallback::<Identity, OFFSET>,
            GetMirrorState: GetMirrorState::<Identity, OFFSET>,
            SetMirrorState: SetMirrorState::<Identity, OFFSET>,
            GetRotation: GetRotation::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            SetCustomSink: SetCustomSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCapturePreviewSink as windows_core::Interface>::IID || iid == &<IMFCaptureSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IMFCapturePreviewSink {}
windows_core::imp::define_interface!(IMFCaptureRecordSink, IMFCaptureRecordSink_Vtbl, 0x3323b55a_f92a_4fe2_8edc_e9bfc0634d77);
impl core::ops::Deref for IMFCaptureRecordSink {
    type Target = IMFCaptureSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFCaptureRecordSink, windows_core::IUnknown, IMFCaptureSink);
impl IMFCaptureRecordSink {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetOutputByteStream<P0>(&self, pbytestream: P0, guidcontainertype: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFByteStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputByteStream)(windows_core::Interface::as_raw(self), pbytestream.param().abi(), guidcontainertype) }
    }
    pub unsafe fn SetOutputFileName<P0>(&self, filename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFileName)(windows_core::Interface::as_raw(self), filename.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSampleCallback<P1>(&self, dwstreamsinkindex: super::DWORD, pcallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFCaptureEngineOnSampleCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSampleCallback)(windows_core::Interface::as_raw(self), dwstreamsinkindex, pcallback.param().abi()) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn SetCustomSink<P0>(&self, pmediasink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFMediaSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCustomSink)(windows_core::Interface::as_raw(self), pmediasink.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRotation(&self, dwstreamindex: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRotation(&self, dwstreamindex: super::DWORD, dwrotationvalue: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), dwstreamindex, dwrotationvalue) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureRecordSink_Vtbl {
    pub base__: IMFCaptureSink_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetOutputByteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetOutputByteStream: usize,
    pub SetOutputFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetSampleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSampleCallback: usize,
    #[cfg(feature = "mfidl")]
    pub SetCustomSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    SetCustomSink: usize,
    #[cfg(feature = "shared")]
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRotation: usize,
    #[cfg(feature = "shared")]
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRotation: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared"))]
pub trait IMFCaptureRecordSink_Impl: IMFCaptureSink_Impl {
    fn SetOutputByteStream(&self, pbytestream: windows_core::Ref<super::IMFByteStream>, guidcontainertype: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetOutputFileName(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSampleCallback(&self, dwstreamsinkindex: super::DWORD, pcallback: windows_core::Ref<IMFCaptureEngineOnSampleCallback>) -> windows_core::Result<()>;
    fn SetCustomSink(&self, pmediasink: windows_core::Ref<super::IMFMediaSink>) -> windows_core::Result<()>;
    fn GetRotation(&self, dwstreamindex: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn SetRotation(&self, dwstreamindex: super::DWORD, dwrotationvalue: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared"))]
impl IMFCaptureRecordSink_Vtbl {
    pub const fn new<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOutputByteStream<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void, guidcontainertype: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureRecordSink_Impl::SetOutputByteStream(this, core::mem::transmute_copy(&pbytestream), core::mem::transmute_copy(&guidcontainertype)).into()
            }
        }
        unsafe extern "system" fn SetOutputFileName<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureRecordSink_Impl::SetOutputFileName(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn SetSampleCallback<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamsinkindex: super::DWORD, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureRecordSink_Impl::SetSampleCallback(this, core::mem::transmute_copy(&dwstreamsinkindex), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn SetCustomSink<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediasink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureRecordSink_Impl::SetCustomSink(this, core::mem::transmute_copy(&pmediasink)).into()
            }
        }
        unsafe extern "system" fn GetRotation<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pdwrotationvalue: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCaptureRecordSink_Impl::GetRotation(this, core::mem::transmute_copy(&dwstreamindex)) {
                    Ok(ok__) => {
                        pdwrotationvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, dwrotationvalue: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureRecordSink_Impl::SetRotation(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwrotationvalue)).into()
            }
        }
        Self {
            base__: IMFCaptureSink_Vtbl::new::<Identity, OFFSET>(),
            SetOutputByteStream: SetOutputByteStream::<Identity, OFFSET>,
            SetOutputFileName: SetOutputFileName::<Identity, OFFSET>,
            SetSampleCallback: SetSampleCallback::<Identity, OFFSET>,
            SetCustomSink: SetCustomSink::<Identity, OFFSET>,
            GetRotation: GetRotation::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureRecordSink as windows_core::Interface>::IID || iid == &<IMFCaptureSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFCaptureRecordSink {}
windows_core::imp::define_interface!(IMFCaptureSink, IMFCaptureSink_Vtbl, 0x72d6135b_35e9_412c_b926_fd5265f2a885);
windows_core::imp::interface_hierarchy!(IMFCaptureSink, windows_core::IUnknown);
impl IMFCaptureSink {
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetOutputMediaType(&self, dwsinkstreamindex: super::DWORD, ppmediatype: Option<*mut Option<super::IMFMediaType>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputMediaType)(windows_core::Interface::as_raw(self), dwsinkstreamindex, ppmediatype.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetService(&self, dwsinkstreamindex: super::DWORD, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunknown: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), dwsinkstreamindex, rguidservice, riid, ppunknown.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn AddStream<P1, P2>(&self, dwsourcestreamindex: super::DWORD, pmediatype: P1, pattributes: P2, pdwsinkstreamindex: Option<*mut super::DWORD>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
        P2: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), dwsourcestreamindex, pmediatype.param().abi(), pattributes.param().abi(), pdwsinkstreamindex.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Prepare(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RemoveAllStreams(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllStreams)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetOutputMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetOutputMediaType: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::GUID, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetService: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    AddStream: usize,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllStreams: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
pub trait IMFCaptureSink_Impl: windows_core::IUnknownImpl {
    fn GetOutputMediaType(&self, dwsinkstreamindex: super::DWORD, ppmediatype: windows_core::OutRef<super::IMFMediaType>) -> windows_core::Result<()>;
    fn GetService(&self, dwsinkstreamindex: super::DWORD, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunknown: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddStream(&self, dwsourcestreamindex: super::DWORD, pmediatype: windows_core::Ref<super::IMFMediaType>, pattributes: windows_core::Ref<super::IMFAttributes>, pdwsinkstreamindex: *mut super::DWORD) -> windows_core::Result<()>;
    fn Prepare(&self) -> windows_core::Result<()>;
    fn RemoveAllStreams(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
impl IMFCaptureSink_Vtbl {
    pub const fn new<Identity: IMFCaptureSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutputMediaType<Identity: IMFCaptureSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsinkstreamindex: super::DWORD, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSink_Impl::GetOutputMediaType(this, core::mem::transmute_copy(&dwsinkstreamindex), core::mem::transmute_copy(&ppmediatype)).into()
            }
        }
        unsafe extern "system" fn GetService<Identity: IMFCaptureSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsinkstreamindex: super::DWORD, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSink_Impl::GetService(this, core::mem::transmute_copy(&dwsinkstreamindex), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunknown)).into()
            }
        }
        unsafe extern "system" fn AddStream<Identity: IMFCaptureSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: super::DWORD, pmediatype: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void, pdwsinkstreamindex: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSink_Impl::AddStream(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&pmediatype), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&pdwsinkstreamindex)).into()
            }
        }
        unsafe extern "system" fn Prepare<Identity: IMFCaptureSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSink_Impl::Prepare(this).into()
            }
        }
        unsafe extern "system" fn RemoveAllStreams<Identity: IMFCaptureSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSink_Impl::RemoveAllStreams(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutputMediaType: GetOutputMediaType::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            Prepare: Prepare::<Identity, OFFSET>,
            RemoveAllStreams: RemoveAllStreams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFCaptureSink {}
windows_core::imp::define_interface!(IMFCaptureSink2, IMFCaptureSink2_Vtbl, 0xf9e4219e_6197_4b5e_b888_bee310ab2c59);
impl core::ops::Deref for IMFCaptureSink2 {
    type Target = IMFCaptureSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFCaptureSink2, windows_core::IUnknown, IMFCaptureSink);
impl IMFCaptureSink2 {
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetOutputMediaType<P1, P2>(&self, dwstreamindex: super::DWORD, pmediatype: P1, pencodingattributes: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
        P2: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, pmediatype.param().abi(), pencodingattributes.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureSink2_Vtbl {
    pub base__: IMFCaptureSink_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetOutputMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetOutputMediaType: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
pub trait IMFCaptureSink2_Impl: IMFCaptureSink_Impl {
    fn SetOutputMediaType(&self, dwstreamindex: super::DWORD, pmediatype: windows_core::Ref<super::IMFMediaType>, pencodingattributes: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
impl IMFCaptureSink2_Vtbl {
    pub const fn new<Identity: IMFCaptureSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOutputMediaType<Identity: IMFCaptureSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pmediatype: *mut core::ffi::c_void, pencodingattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSink2_Impl::SetOutputMediaType(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pmediatype), core::mem::transmute_copy(&pencodingattributes)).into()
            }
        }
        Self { base__: IMFCaptureSink_Vtbl::new::<Identity, OFFSET>(), SetOutputMediaType: SetOutputMediaType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureSink2 as windows_core::Interface>::IID || iid == &<IMFCaptureSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFCaptureSink2 {}
windows_core::imp::define_interface!(IMFCaptureSource, IMFCaptureSource_Vtbl, 0x439a42a8_0d2c_4505_be83_f79b2a05d5c4);
windows_core::imp::interface_hierarchy!(IMFCaptureSource, windows_core::IUnknown);
impl IMFCaptureSource {
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub unsafe fn GetCaptureDeviceSource(&self, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppmediasource: Option<*mut Option<super::IMFMediaSource>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaptureDeviceSource)(windows_core::Interface::as_raw(self), mfcaptureenginedevicetype, ppmediasource.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetCaptureDeviceActivate(&self, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppactivate: Option<*mut Option<super::IMFActivate>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaptureDeviceActivate)(windows_core::Interface::as_raw(self), mfcaptureenginedevicetype, ppactivate.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetService(&self, rguidservice: *const super::IID, riid: *const super::IID, ppunknown: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), rguidservice, riid, ppunknown.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddEffect<P1>(&self, dwsourcestreamindex: super::DWORD, punknown: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEffect)(windows_core::Interface::as_raw(self), dwsourcestreamindex, punknown.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveEffect<P1>(&self, dwsourcestreamindex: super::DWORD, punknown: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveEffect)(windows_core::Interface::as_raw(self), dwsourcestreamindex, punknown.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveAllEffects(&self, dwsourcestreamindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllEffects)(windows_core::Interface::as_raw(self), dwsourcestreamindex) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetAvailableDeviceMediaType(&self, dwsourcestreamindex: super::DWORD, dwmediatypeindex: super::DWORD, ppmediatype: Option<*mut Option<super::IMFMediaType>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableDeviceMediaType)(windows_core::Interface::as_raw(self), dwsourcestreamindex, dwmediatypeindex, ppmediatype.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetCurrentDeviceMediaType<P1>(&self, dwsourcestreamindex: super::DWORD, pmediatype: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentDeviceMediaType)(windows_core::Interface::as_raw(self), dwsourcestreamindex, pmediatype.param().abi()) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetCurrentDeviceMediaType(&self, dwsourcestreamindex: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentDeviceMediaType)(windows_core::Interface::as_raw(self), dwsourcestreamindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDeviceStreamCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceStreamCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDeviceStreamCategory(&self, dwsourcestreamindex: super::DWORD) -> windows_core::Result<MF_CAPTURE_ENGINE_STREAM_CATEGORY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceStreamCategory)(windows_core::Interface::as_raw(self), dwsourcestreamindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMirrorState(&self, dwstreamindex: super::DWORD) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMirrorState)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMirrorState(&self, dwstreamindex: super::DWORD, fmirrorstate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMirrorState)(windows_core::Interface::as_raw(self), dwstreamindex, fmirrorstate.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamIndexFromFriendlyName(&self, uifriendlyname: super::UINT32) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamIndexFromFriendlyName)(windows_core::Interface::as_raw(self), uifriendlyname, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub GetCaptureDeviceSource: unsafe extern "system" fn(*mut core::ffi::c_void, MF_CAPTURE_ENGINE_DEVICE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mfobjects")))]
    GetCaptureDeviceSource: usize,
    #[cfg(feature = "mfobjects")]
    pub GetCaptureDeviceActivate: unsafe extern "system" fn(*mut core::ffi::c_void, MF_CAPTURE_ENGINE_DEVICE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetCaptureDeviceActivate: usize,
    #[cfg(feature = "guiddef")]
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetService: usize,
    #[cfg(feature = "shared")]
    pub AddEffect: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddEffect: usize,
    #[cfg(feature = "shared")]
    pub RemoveEffect: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveEffect: usize,
    #[cfg(feature = "shared")]
    pub RemoveAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveAllEffects: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetAvailableDeviceMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetAvailableDeviceMediaType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetCurrentDeviceMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetCurrentDeviceMediaType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetCurrentDeviceMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetCurrentDeviceMediaType: usize,
    #[cfg(feature = "shared")]
    pub GetDeviceStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDeviceStreamCount: usize,
    #[cfg(feature = "shared")]
    pub GetDeviceStreamCategory: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut MF_CAPTURE_ENGINE_STREAM_CATEGORY) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDeviceStreamCategory: usize,
    #[cfg(feature = "shared")]
    pub GetMirrorState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMirrorState: usize,
    #[cfg(feature = "shared")]
    pub SetMirrorState: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMirrorState: usize,
    #[cfg(feature = "shared")]
    pub GetStreamIndexFromFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamIndexFromFriendlyName: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared"))]
pub trait IMFCaptureSource_Impl: windows_core::IUnknownImpl {
    fn GetCaptureDeviceSource(&self, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppmediasource: windows_core::OutRef<super::IMFMediaSource>) -> windows_core::Result<()>;
    fn GetCaptureDeviceActivate(&self, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppactivate: windows_core::OutRef<super::IMFActivate>) -> windows_core::Result<()>;
    fn GetService(&self, rguidservice: *const super::IID, riid: *const super::IID, ppunknown: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddEffect(&self, dwsourcestreamindex: super::DWORD, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveEffect(&self, dwsourcestreamindex: super::DWORD, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAllEffects(&self, dwsourcestreamindex: super::DWORD) -> windows_core::Result<()>;
    fn GetAvailableDeviceMediaType(&self, dwsourcestreamindex: super::DWORD, dwmediatypeindex: super::DWORD, ppmediatype: windows_core::OutRef<super::IMFMediaType>) -> windows_core::Result<()>;
    fn SetCurrentDeviceMediaType(&self, dwsourcestreamindex: super::DWORD, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
    fn GetCurrentDeviceMediaType(&self, dwsourcestreamindex: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetDeviceStreamCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetDeviceStreamCategory(&self, dwsourcestreamindex: super::DWORD) -> windows_core::Result<MF_CAPTURE_ENGINE_STREAM_CATEGORY>;
    fn GetMirrorState(&self, dwstreamindex: super::DWORD) -> windows_core::Result<windows_core::BOOL>;
    fn SetMirrorState(&self, dwstreamindex: super::DWORD, fmirrorstate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetStreamIndexFromFriendlyName(&self, uifriendlyname: super::UINT32) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared"))]
impl IMFCaptureSource_Vtbl {
    pub const fn new<Identity: IMFCaptureSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaptureDeviceSource<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppmediasource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::GetCaptureDeviceSource(this, core::mem::transmute_copy(&mfcaptureenginedevicetype), core::mem::transmute_copy(&ppmediasource)).into()
            }
        }
        unsafe extern "system" fn GetCaptureDeviceActivate<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppactivate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::GetCaptureDeviceActivate(this, core::mem::transmute_copy(&mfcaptureenginedevicetype), core::mem::transmute_copy(&ppactivate)).into()
            }
        }
        unsafe extern "system" fn GetService<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidservice: *const super::IID, riid: *const super::IID, ppunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::GetService(this, core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunknown)).into()
            }
        }
        unsafe extern "system" fn AddEffect<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: super::DWORD, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::AddEffect(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn RemoveEffect<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: super::DWORD, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::RemoveEffect(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn RemoveAllEffects<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::RemoveAllEffects(this, core::mem::transmute_copy(&dwsourcestreamindex)).into()
            }
        }
        unsafe extern "system" fn GetAvailableDeviceMediaType<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: super::DWORD, dwmediatypeindex: super::DWORD, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::GetAvailableDeviceMediaType(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&dwmediatypeindex), core::mem::transmute_copy(&ppmediatype)).into()
            }
        }
        unsafe extern "system" fn SetCurrentDeviceMediaType<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: super::DWORD, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::SetCurrentDeviceMediaType(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        unsafe extern "system" fn GetCurrentDeviceMediaType<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: super::DWORD, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCaptureSource_Impl::GetCurrentDeviceMediaType(this, core::mem::transmute_copy(&dwsourcestreamindex)) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceStreamCount<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstreamcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCaptureSource_Impl::GetDeviceStreamCount(this) {
                    Ok(ok__) => {
                        pdwstreamcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceStreamCategory<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: super::DWORD, pstreamcategory: *mut MF_CAPTURE_ENGINE_STREAM_CATEGORY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCaptureSource_Impl::GetDeviceStreamCategory(this, core::mem::transmute_copy(&dwsourcestreamindex)) {
                    Ok(ok__) => {
                        pstreamcategory.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMirrorState<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pfmirrorstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCaptureSource_Impl::GetMirrorState(this, core::mem::transmute_copy(&dwstreamindex)) {
                    Ok(ok__) => {
                        pfmirrorstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMirrorState<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, fmirrorstate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::SetMirrorState(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&fmirrorstate)).into()
            }
        }
        unsafe extern "system" fn GetStreamIndexFromFriendlyName<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uifriendlyname: super::UINT32, pdwactualstreamindex: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCaptureSource_Impl::GetStreamIndexFromFriendlyName(this, core::mem::transmute_copy(&uifriendlyname)) {
                    Ok(ok__) => {
                        pdwactualstreamindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaptureDeviceSource: GetCaptureDeviceSource::<Identity, OFFSET>,
            GetCaptureDeviceActivate: GetCaptureDeviceActivate::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
            AddEffect: AddEffect::<Identity, OFFSET>,
            RemoveEffect: RemoveEffect::<Identity, OFFSET>,
            RemoveAllEffects: RemoveAllEffects::<Identity, OFFSET>,
            GetAvailableDeviceMediaType: GetAvailableDeviceMediaType::<Identity, OFFSET>,
            SetCurrentDeviceMediaType: SetCurrentDeviceMediaType::<Identity, OFFSET>,
            GetCurrentDeviceMediaType: GetCurrentDeviceMediaType::<Identity, OFFSET>,
            GetDeviceStreamCount: GetDeviceStreamCount::<Identity, OFFSET>,
            GetDeviceStreamCategory: GetDeviceStreamCategory::<Identity, OFFSET>,
            GetMirrorState: GetMirrorState::<Identity, OFFSET>,
            SetMirrorState: SetMirrorState::<Identity, OFFSET>,
            GetStreamIndexFromFriendlyName: GetStreamIndexFromFriendlyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCaptureSource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFCaptureSource {}
pub const MFSampleExtension_DeviceReferenceSystemTime: windows_core::GUID = windows_core::GUID::from_u128(0x6523775a_ba2d_405f_b2c5_01ff88e2e8f6);
pub const MF_CAPTURE_ENGINE_ALL_EFFECTS_REMOVED: windows_core::GUID = windows_core::GUID::from_u128(0xfded7521_8ed8_431a_a96b_f3e2565e981c);
pub const MF_CAPTURE_ENGINE_AUDIO_PROCESSING: windows_core::GUID = windows_core::GUID::from_u128(0x10f1be5e_7e11_410b_973d_f4b6109000fe);
pub const MF_CAPTURE_ENGINE_AUDIO_PROCESSING_DEFAULT: MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = 0;
pub type MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = i32;
pub const MF_CAPTURE_ENGINE_AUDIO_PROCESSING_RAW: MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = 1;
pub const MF_CAPTURE_ENGINE_CAMERA_STREAM_BLOCKED: windows_core::GUID = windows_core::GUID::from_u128(0xa4209417_8d39_46f3_b759_5912528f4207);
pub const MF_CAPTURE_ENGINE_CAMERA_STREAM_UNBLOCKED: windows_core::GUID = windows_core::GUID::from_u128(0x9be9eef0_cdaf_4717_8564_834aae66415c);
pub const MF_CAPTURE_ENGINE_D3D_MANAGER: windows_core::GUID = windows_core::GUID::from_u128(0x76e25e7b_d595_4283_962c_c594afd78ddf);
pub const MF_CAPTURE_ENGINE_DECODER_MFT_FIELDOFUSE_UNLOCK_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x2b8ad2e8_7acb_4321_a606_325c4249f4fc);
pub type MF_CAPTURE_ENGINE_DEVICE_TYPE = i32;
pub const MF_CAPTURE_ENGINE_DEVICE_TYPE_AUDIO: MF_CAPTURE_ENGINE_DEVICE_TYPE = 0;
pub const MF_CAPTURE_ENGINE_DEVICE_TYPE_VIDEO: MF_CAPTURE_ENGINE_DEVICE_TYPE = 1;
pub const MF_CAPTURE_ENGINE_DISABLE_DXVA: windows_core::GUID = windows_core::GUID::from_u128(0xf9818862_179d_433f_a32f_74cbcf74466d);
pub const MF_CAPTURE_ENGINE_DISABLE_HARDWARE_TRANSFORMS: windows_core::GUID = windows_core::GUID::from_u128(0xb7c42a6b_3207_4495_b4e7_81f9c35d5991);
pub const MF_CAPTURE_ENGINE_EFFECT_ADDED: windows_core::GUID = windows_core::GUID::from_u128(0xaa8dc7b5_a048_4e13_8ebe_f23c46c830c1);
pub const MF_CAPTURE_ENGINE_EFFECT_REMOVED: windows_core::GUID = windows_core::GUID::from_u128(0xc6e8db07_fb09_4a48_89c6_bf92a04222c9);
pub const MF_CAPTURE_ENGINE_ENABLE_CAMERA_STREAMSTATE_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x4c808e9d_aaed_4713_90fb_cb24064ab8da);
pub const MF_CAPTURE_ENGINE_ENCODER_MFT_FIELDOFUSE_UNLOCK_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0x54c63a00_78d5_422f_aa3e_5e99ac649269);
pub const MF_CAPTURE_ENGINE_ERROR: windows_core::GUID = windows_core::GUID::from_u128(0x46b89fc6_33cc_4399_9dad_784de77d587c);
pub const MF_CAPTURE_ENGINE_EVENT_GENERATOR_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xabfa8ad5_fc6d_4911_87e0_961945f8f7ce);
pub const MF_CAPTURE_ENGINE_EVENT_STREAM_INDEX: windows_core::GUID = windows_core::GUID::from_u128(0x82697f44_b1cf_42eb_9753_f86d649c8865);
pub const MF_CAPTURE_ENGINE_INITIALIZED: windows_core::GUID = windows_core::GUID::from_u128(0x219992bc_cf92_4531_a1ae_96e1e886c8f1);
pub const MF_CAPTURE_ENGINE_MEDIASOURCE: i32 = -1;
pub const MF_CAPTURE_ENGINE_MEDIASOURCE_CONFIG: windows_core::GUID = windows_core::GUID::from_u128(0xbc6989d2_0fc1_46e1_a74f_efd36bc788de);
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY: windows_core::GUID = windows_core::GUID::from_u128(0x8e3f5bd5_dbbf_42f0_8542_d07a3971762a);
pub type MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = i32;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_COMMUNICATIONS: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 1;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_FARFIELDSPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 5;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_GAMECHAT: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 3;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_MEDIA: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 2;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_OTHER: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 0;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_SPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 4;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_UNIFORMSPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 6;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_VOICETYPING: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 7;
pub const MF_CAPTURE_ENGINE_OUTPUT_MEDIA_TYPE_SET: windows_core::GUID = windows_core::GUID::from_u128(0xcaaad994_83ec_45e9_a30a_1f20aadb9831);
pub const MF_CAPTURE_ENGINE_PHOTO_TAKEN: windows_core::GUID = windows_core::GUID::from_u128(0x3c50c445_7304_48eb_865d_bba19ba3af5c);
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_AUDIO: i32 = -9;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_METADATA: i32 = -10;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_PHOTO: i32 = -8;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_VIDEO_PREVIEW: i32 = -6;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_VIDEO_RECORD: i32 = -7;
pub const MF_CAPTURE_ENGINE_PREVIEW_STARTED: windows_core::GUID = windows_core::GUID::from_u128(0xa416df21_f9d3_4a74_991b_b817298952c4);
pub const MF_CAPTURE_ENGINE_PREVIEW_STOPPED: windows_core::GUID = windows_core::GUID::from_u128(0x13d5143c_1edd_4e50_a2ef_350a47678060);
pub const MF_CAPTURE_ENGINE_RECORD_SINK_AUDIO_MAX_PROCESSED_SAMPLES: windows_core::GUID = windows_core::GUID::from_u128(0x9896e12a_f707_4500_b6bd_db8eb810b50f);
pub const MF_CAPTURE_ENGINE_RECORD_SINK_AUDIO_MAX_UNPROCESSED_SAMPLES: windows_core::GUID = windows_core::GUID::from_u128(0x1cddb141_a7f4_4d58_9896_4d15a53c4efe);
pub const MF_CAPTURE_ENGINE_RECORD_SINK_VIDEO_MAX_PROCESSED_SAMPLES: windows_core::GUID = windows_core::GUID::from_u128(0xe7b4a49e_382c_4aef_a946_aed5490b7111);
pub const MF_CAPTURE_ENGINE_RECORD_SINK_VIDEO_MAX_UNPROCESSED_SAMPLES: windows_core::GUID = windows_core::GUID::from_u128(0xb467f705_7913_4894_9d42_a215fea23da9);
pub const MF_CAPTURE_ENGINE_RECORD_STARTED: windows_core::GUID = windows_core::GUID::from_u128(0xac2b027b_ddf9_48a0_89be_38ab35ef45c0);
pub const MF_CAPTURE_ENGINE_RECORD_STOPPED: windows_core::GUID = windows_core::GUID::from_u128(0x55e5200a_f98f_4c0d_a9ec_9eb25ed3d773);
pub const MF_CAPTURE_ENGINE_SELECTEDCAMERAPROFILE: windows_core::GUID = windows_core::GUID::from_u128(0x03160b7e_1c6f_4db2_ad56_a7c430f82392);
pub const MF_CAPTURE_ENGINE_SELECTEDCAMERAPROFILE_INDEX: windows_core::GUID = windows_core::GUID::from_u128(0x3ce88613_2214_46c3_b417_82f8a313c9c3);
pub type MF_CAPTURE_ENGINE_SINK_TYPE = i32;
pub const MF_CAPTURE_ENGINE_SINK_TYPE_PHOTO: MF_CAPTURE_ENGINE_SINK_TYPE = 2;
pub const MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW: MF_CAPTURE_ENGINE_SINK_TYPE = 1;
pub const MF_CAPTURE_ENGINE_SINK_TYPE_RECORD: MF_CAPTURE_ENGINE_SINK_TYPE = 0;
pub type MF_CAPTURE_ENGINE_STREAM_CATEGORY = i32;
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_AUDIO: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 4;
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_METADATA: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 6;
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_PHOTO_DEPENDENT: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 3;
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_PHOTO_INDEPENDENT: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 2;
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_UNSUPPORTED: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 5;
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_VIDEO_CAPTURE: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 1;
pub const MF_CAPTURE_ENGINE_STREAM_CATEGORY_VIDEO_PREVIEW: MF_CAPTURE_ENGINE_STREAM_CATEGORY = 0;
pub const MF_CAPTURE_ENGINE_USE_AUDIO_DEVICE_ONLY: windows_core::GUID = windows_core::GUID::from_u128(0x1c8077da_8466_4dc4_8b8e_276b3f85923b);
pub const MF_CAPTURE_ENGINE_USE_VIDEO_DEVICE_ONLY: windows_core::GUID = windows_core::GUID::from_u128(0x7e025171_cf32_4f2e_8f19_410577b73a66);
pub const MF_CAPTURE_SINK_PREPARED: windows_core::GUID = windows_core::GUID::from_u128(0x7bfce257_12b1_4409_8c34_d445daab7578);
pub const MF_CAPTURE_SOURCE_CURRENT_DEVICE_MEDIA_TYPE_SET: windows_core::GUID = windows_core::GUID::from_u128(0xe7e75e4c_039c_4410_815b_8741307b63aa);
