windows_core::imp::define_interface!(IMFCaptureEngine, IMFCaptureEngine_Vtbl, 0xa6bba433_176b_48b2_b375_53aa03473207);
windows_core::imp::interface_hierarchy!(IMFCaptureEngine, windows_core::IUnknown);
impl IMFCaptureEngine {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, peventcallback: P0, pattributes: P1, paudiosource: P2, pvideosource: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFCaptureEngineOnEventCallback>,
        P1: windows_core::Param<super::mfobjects::IMFAttributes>,
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
            (windows_core::Interface::vtable(self).GetSink)(windows_core::Interface::as_raw(self), mfcaptureenginesinktype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<IMFCaptureSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    fn Initialize(&self, peventcallback: windows_core::Ref<IMFCaptureEngineOnEventCallback>, pattributes: windows_core::Ref<super::mfobjects::IMFAttributes>, paudiosource: windows_core::Ref<windows_core::IUnknown>, pvideosource: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
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
    pub unsafe fn CreateInstance(&self, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), clsid, riid, ppvobject as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureEngineClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFCaptureEngineClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFCaptureEngineClassFactory_Vtbl {
    pub const fn new<Identity: IMFCaptureEngineClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IMFCaptureEngineClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IMFCaptureEngineClassFactory {}
windows_core::imp::define_interface!(IMFCaptureEngineOnEventCallback, IMFCaptureEngineOnEventCallback_Vtbl, 0xaeda51c0_9025_4983_9012_de597b88b089);
windows_core::imp::interface_hierarchy!(IMFCaptureEngineOnEventCallback, windows_core::IUnknown);
impl IMFCaptureEngineOnEventCallback {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnEvent<P0>(&self, pevent: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mfobjects::IMFMediaEvent>,
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
    fn OnEvent(&self, pevent: windows_core::Ref<super::mfobjects::IMFMediaEvent>) -> windows_core::Result<()>;
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
        P0: windows_core::Param<super::mfobjects::IMFSample>,
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
    fn OnSample(&self, psample: windows_core::Ref<super::mfobjects::IMFSample>) -> windows_core::Result<()>;
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
        P0: windows_core::Param<super::mfobjects::IMFMediaEvent>,
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
    fn OnSynchronizedEvent(&self, pevent: windows_core::Ref<super::mfobjects::IMFMediaEvent>) -> windows_core::Result<()>;
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
        P0: windows_core::Param<super::mfobjects::IMFByteStream>,
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
#[cfg(feature = "mfobjects")]
pub trait IMFCapturePhotoSink_Impl: IMFCaptureSink_Impl {
    fn SetOutputFileName(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSampleCallback(&self, pcallback: windows_core::Ref<IMFCaptureEngineOnSampleCallback>) -> windows_core::Result<()>;
    fn SetOutputByteStream(&self, pbytestream: windows_core::Ref<super::mfobjects::IMFByteStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
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
#[cfg(feature = "mfobjects")]
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
    pub unsafe fn SetRenderHandle(&self, handle: super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderHandle)(windows_core::Interface::as_raw(self), handle) }
    }
    pub unsafe fn SetRenderSurface<P0>(&self, psurface: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRenderSurface)(windows_core::Interface::as_raw(self), psurface.param().abi()) }
    }
    #[cfg(all(feature = "mfidl", feature = "windef"))]
    pub unsafe fn UpdateVideo(&self, psrc: Option<*const super::mfidl::MFVideoNormalizedRect>, pdst: Option<*const super::windef::RECT>, pborderclr: Option<*const super::windef::COLORREF>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateVideo)(windows_core::Interface::as_raw(self), psrc.unwrap_or(core::mem::zeroed()) as _, pdst.unwrap_or(core::mem::zeroed()) as _, pborderclr.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetSampleCallback<P1>(&self, dwstreamsinkindex: u32, pcallback: P1) -> windows_core::HRESULT
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
    pub unsafe fn GetRotation(&self, dwstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRotation(&self, dwstreamindex: u32, dwrotationvalue: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), dwstreamindex, dwrotationvalue) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn SetCustomSink<P0>(&self, pmediasink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mfidl::IMFMediaSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCustomSink)(windows_core::Interface::as_raw(self), pmediasink.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCapturePreviewSink_Vtbl {
    pub base__: IMFCaptureSink_Vtbl,
    #[cfg(feature = "winnt")]
    pub SetRenderHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetRenderHandle: usize,
    pub SetRenderSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "mfidl", feature = "windef"))]
    pub UpdateVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::mfidl::MFVideoNormalizedRect, *const super::windef::RECT, *const super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "windef")))]
    UpdateVideo: usize,
    pub SetSampleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMirrorState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetMirrorState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfidl")]
    pub SetCustomSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    SetCustomSink: usize,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef", feature = "winnt"))]
pub trait IMFCapturePreviewSink_Impl: IMFCaptureSink_Impl {
    fn SetRenderHandle(&self, handle: super::winnt::HANDLE) -> windows_core::Result<()>;
    fn SetRenderSurface(&self, psurface: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UpdateVideo(&self, psrc: *const super::mfidl::MFVideoNormalizedRect, pdst: *const super::windef::RECT, pborderclr: *const super::windef::COLORREF) -> windows_core::Result<()>;
    fn SetSampleCallback(&self, dwstreamsinkindex: u32, pcallback: windows_core::Ref<IMFCaptureEngineOnSampleCallback>) -> windows_core::Result<()>;
    fn GetMirrorState(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetMirrorState(&self, fmirrorstate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetRotation(&self, dwstreamindex: u32) -> windows_core::Result<u32>;
    fn SetRotation(&self, dwstreamindex: u32, dwrotationvalue: u32) -> windows_core::Result<()>;
    fn SetCustomSink(&self, pmediasink: windows_core::Ref<super::mfidl::IMFMediaSink>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef", feature = "winnt"))]
impl IMFCapturePreviewSink_Vtbl {
    pub const fn new<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRenderHandle<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::winnt::HANDLE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn UpdateVideo<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *const super::mfidl::MFVideoNormalizedRect, pdst: *const super::windef::RECT, pborderclr: *const super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePreviewSink_Impl::UpdateVideo(this, core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pborderclr)).into()
            }
        }
        unsafe extern "system" fn SetSampleCallback<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamsinkindex: u32, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetRotation<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pdwrotationvalue: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetRotation<Identity: IMFCapturePreviewSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, dwrotationvalue: u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef", feature = "winnt"))]
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
        P0: windows_core::Param<super::mfobjects::IMFByteStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputByteStream)(windows_core::Interface::as_raw(self), pbytestream.param().abi(), guidcontainertype) }
    }
    pub unsafe fn SetOutputFileName<P0>(&self, filename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFileName)(windows_core::Interface::as_raw(self), filename.param().abi()) }
    }
    pub unsafe fn SetSampleCallback<P1>(&self, dwstreamsinkindex: u32, pcallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFCaptureEngineOnSampleCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSampleCallback)(windows_core::Interface::as_raw(self), dwstreamsinkindex, pcallback.param().abi()) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn SetCustomSink<P0>(&self, pmediasink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mfidl::IMFMediaSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCustomSink)(windows_core::Interface::as_raw(self), pmediasink.param().abi()) }
    }
    pub unsafe fn GetRotation(&self, dwstreamindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRotation(&self, dwstreamindex: u32, dwrotationvalue: u32) -> windows_core::HRESULT {
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
    pub SetSampleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfidl")]
    pub SetCustomSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    SetCustomSink: usize,
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
pub trait IMFCaptureRecordSink_Impl: IMFCaptureSink_Impl {
    fn SetOutputByteStream(&self, pbytestream: windows_core::Ref<super::mfobjects::IMFByteStream>, guidcontainertype: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetOutputFileName(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSampleCallback(&self, dwstreamsinkindex: u32, pcallback: windows_core::Ref<IMFCaptureEngineOnSampleCallback>) -> windows_core::Result<()>;
    fn SetCustomSink(&self, pmediasink: windows_core::Ref<super::mfidl::IMFMediaSink>) -> windows_core::Result<()>;
    fn GetRotation(&self, dwstreamindex: u32) -> windows_core::Result<u32>;
    fn SetRotation(&self, dwstreamindex: u32, dwrotationvalue: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
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
        unsafe extern "system" fn SetSampleCallback<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamsinkindex: u32, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetRotation<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pdwrotationvalue: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetRotation<Identity: IMFCaptureRecordSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, dwrotationvalue: u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
impl windows_core::RuntimeName for IMFCaptureRecordSink {}
windows_core::imp::define_interface!(IMFCaptureSink, IMFCaptureSink_Vtbl, 0x72d6135b_35e9_412c_b926_fd5265f2a885);
windows_core::imp::interface_hierarchy!(IMFCaptureSink, windows_core::IUnknown);
impl IMFCaptureSink {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetOutputMediaType(&self, dwsinkstreamindex: u32, ppmediatype: Option<*mut Option<super::mfobjects::IMFMediaType>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputMediaType)(windows_core::Interface::as_raw(self), dwsinkstreamindex, ppmediatype.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetService(&self, dwsinkstreamindex: u32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunknown: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), dwsinkstreamindex, rguidservice, riid, ppunknown.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn AddStream<P1, P2>(&self, dwsourcestreamindex: u32, pmediatype: P1, pattributes: P2, pdwsinkstreamindex: Option<*mut u32>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFMediaType>,
        P2: windows_core::Param<super::mfobjects::IMFAttributes>,
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
    #[cfg(feature = "mfobjects")]
    pub GetOutputMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetOutputMediaType: usize,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    AddStream: usize,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllStreams: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFCaptureSink_Impl: windows_core::IUnknownImpl {
    fn GetOutputMediaType(&self, dwsinkstreamindex: u32, ppmediatype: windows_core::OutRef<super::mfobjects::IMFMediaType>) -> windows_core::Result<()>;
    fn GetService(&self, dwsinkstreamindex: u32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunknown: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddStream(&self, dwsourcestreamindex: u32, pmediatype: windows_core::Ref<super::mfobjects::IMFMediaType>, pattributes: windows_core::Ref<super::mfobjects::IMFAttributes>, pdwsinkstreamindex: *mut u32) -> windows_core::Result<()>;
    fn Prepare(&self) -> windows_core::Result<()>;
    fn RemoveAllStreams(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFCaptureSink_Vtbl {
    pub const fn new<Identity: IMFCaptureSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutputMediaType<Identity: IMFCaptureSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsinkstreamindex: u32, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSink_Impl::GetOutputMediaType(this, core::mem::transmute_copy(&dwsinkstreamindex), core::mem::transmute_copy(&ppmediatype)).into()
            }
        }
        unsafe extern "system" fn GetService<Identity: IMFCaptureSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsinkstreamindex: u32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSink_Impl::GetService(this, core::mem::transmute_copy(&dwsinkstreamindex), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunknown)).into()
            }
        }
        unsafe extern "system" fn AddStream<Identity: IMFCaptureSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: u32, pmediatype: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void, pdwsinkstreamindex: *mut u32) -> windows_core::HRESULT {
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
#[cfg(feature = "mfobjects")]
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
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetOutputMediaType<P1, P2>(&self, dwstreamindex: u32, pmediatype: P1, pencodingattributes: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFMediaType>,
        P2: windows_core::Param<super::mfobjects::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, pmediatype.param().abi(), pencodingattributes.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCaptureSink2_Vtbl {
    pub base__: IMFCaptureSink_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetOutputMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetOutputMediaType: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFCaptureSink2_Impl: IMFCaptureSink_Impl {
    fn SetOutputMediaType(&self, dwstreamindex: u32, pmediatype: windows_core::Ref<super::mfobjects::IMFMediaType>, pencodingattributes: windows_core::Ref<super::mfobjects::IMFAttributes>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFCaptureSink2_Vtbl {
    pub const fn new<Identity: IMFCaptureSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOutputMediaType<Identity: IMFCaptureSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pmediatype: *mut core::ffi::c_void, pencodingattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFCaptureSink2 {}
windows_core::imp::define_interface!(IMFCaptureSource, IMFCaptureSource_Vtbl, 0x439a42a8_0d2c_4505_be83_f79b2a05d5c4);
windows_core::imp::interface_hierarchy!(IMFCaptureSource, windows_core::IUnknown);
impl IMFCaptureSource {
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub unsafe fn GetCaptureDeviceSource(&self, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppmediasource: Option<*mut Option<super::mfidl::IMFMediaSource>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaptureDeviceSource)(windows_core::Interface::as_raw(self), mfcaptureenginedevicetype, ppmediasource.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetCaptureDeviceActivate(&self, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppactivate: Option<*mut Option<super::mfobjects::IMFActivate>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaptureDeviceActivate)(windows_core::Interface::as_raw(self), mfcaptureenginedevicetype, ppactivate.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetService(&self, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunknown: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), rguidservice, riid, ppunknown.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn AddEffect<P1>(&self, dwsourcestreamindex: u32, punknown: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEffect)(windows_core::Interface::as_raw(self), dwsourcestreamindex, punknown.param().abi()) }
    }
    pub unsafe fn RemoveEffect<P1>(&self, dwsourcestreamindex: u32, punknown: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveEffect)(windows_core::Interface::as_raw(self), dwsourcestreamindex, punknown.param().abi()) }
    }
    pub unsafe fn RemoveAllEffects(&self, dwsourcestreamindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllEffects)(windows_core::Interface::as_raw(self), dwsourcestreamindex) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetAvailableDeviceMediaType(&self, dwsourcestreamindex: u32, dwmediatypeindex: u32, ppmediatype: Option<*mut Option<super::mfobjects::IMFMediaType>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableDeviceMediaType)(windows_core::Interface::as_raw(self), dwsourcestreamindex, dwmediatypeindex, ppmediatype.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetCurrentDeviceMediaType<P1>(&self, dwsourcestreamindex: u32, pmediatype: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentDeviceMediaType)(windows_core::Interface::as_raw(self), dwsourcestreamindex, pmediatype.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetCurrentDeviceMediaType(&self, dwsourcestreamindex: u32) -> windows_core::Result<super::mfobjects::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentDeviceMediaType)(windows_core::Interface::as_raw(self), dwsourcestreamindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeviceStreamCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceStreamCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceStreamCategory(&self, dwsourcestreamindex: u32) -> windows_core::Result<MF_CAPTURE_ENGINE_STREAM_CATEGORY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceStreamCategory)(windows_core::Interface::as_raw(self), dwsourcestreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMirrorState(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMirrorState)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMirrorState(&self, dwstreamindex: u32, fmirrorstate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMirrorState)(windows_core::Interface::as_raw(self), dwstreamindex, fmirrorstate.into()) }
    }
    pub unsafe fn GetStreamIndexFromFriendlyName(&self, uifriendlyname: u32) -> windows_core::Result<u32> {
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
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetAvailableDeviceMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetAvailableDeviceMediaType: usize,
    #[cfg(feature = "mfobjects")]
    pub SetCurrentDeviceMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetCurrentDeviceMediaType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetCurrentDeviceMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetCurrentDeviceMediaType: usize,
    pub GetDeviceStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceStreamCategory: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MF_CAPTURE_ENGINE_STREAM_CATEGORY) -> windows_core::HRESULT,
    pub GetMirrorState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetMirrorState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetStreamIndexFromFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
pub trait IMFCaptureSource_Impl: windows_core::IUnknownImpl {
    fn GetCaptureDeviceSource(&self, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppmediasource: windows_core::OutRef<super::mfidl::IMFMediaSource>) -> windows_core::Result<()>;
    fn GetCaptureDeviceActivate(&self, mfcaptureenginedevicetype: MF_CAPTURE_ENGINE_DEVICE_TYPE, ppactivate: windows_core::OutRef<super::mfobjects::IMFActivate>) -> windows_core::Result<()>;
    fn GetService(&self, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunknown: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddEffect(&self, dwsourcestreamindex: u32, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveEffect(&self, dwsourcestreamindex: u32, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAllEffects(&self, dwsourcestreamindex: u32) -> windows_core::Result<()>;
    fn GetAvailableDeviceMediaType(&self, dwsourcestreamindex: u32, dwmediatypeindex: u32, ppmediatype: windows_core::OutRef<super::mfobjects::IMFMediaType>) -> windows_core::Result<()>;
    fn SetCurrentDeviceMediaType(&self, dwsourcestreamindex: u32, pmediatype: windows_core::Ref<super::mfobjects::IMFMediaType>) -> windows_core::Result<()>;
    fn GetCurrentDeviceMediaType(&self, dwsourcestreamindex: u32) -> windows_core::Result<super::mfobjects::IMFMediaType>;
    fn GetDeviceStreamCount(&self) -> windows_core::Result<u32>;
    fn GetDeviceStreamCategory(&self, dwsourcestreamindex: u32) -> windows_core::Result<MF_CAPTURE_ENGINE_STREAM_CATEGORY>;
    fn GetMirrorState(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetMirrorState(&self, dwstreamindex: u32, fmirrorstate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetStreamIndexFromFriendlyName(&self, uifriendlyname: u32) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
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
        unsafe extern "system" fn GetService<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::GetService(this, core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunknown)).into()
            }
        }
        unsafe extern "system" fn AddEffect<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: u32, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::AddEffect(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn RemoveEffect<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: u32, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::RemoveEffect(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn RemoveAllEffects<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::RemoveAllEffects(this, core::mem::transmute_copy(&dwsourcestreamindex)).into()
            }
        }
        unsafe extern "system" fn GetAvailableDeviceMediaType<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: u32, dwmediatypeindex: u32, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::GetAvailableDeviceMediaType(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&dwmediatypeindex), core::mem::transmute_copy(&ppmediatype)).into()
            }
        }
        unsafe extern "system" fn SetCurrentDeviceMediaType<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: u32, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::SetCurrentDeviceMediaType(this, core::mem::transmute_copy(&dwsourcestreamindex), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        unsafe extern "system" fn GetCurrentDeviceMediaType<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: u32, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDeviceStreamCount<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstreamcount: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDeviceStreamCategory<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcestreamindex: u32, pstreamcategory: *mut MF_CAPTURE_ENGINE_STREAM_CATEGORY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetMirrorState<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pfmirrorstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetMirrorState<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, fmirrorstate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCaptureSource_Impl::SetMirrorState(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&fmirrorstate)).into()
            }
        }
        unsafe extern "system" fn GetStreamIndexFromFriendlyName<Identity: IMFCaptureSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uifriendlyname: u32, pdwactualstreamindex: *mut u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
impl windows_core::RuntimeName for IMFCaptureSource {}
pub const MF_CAPTURE_ENGINE_AUDIO_PROCESSING_DEFAULT: MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = 0;
pub type MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = i32;
pub const MF_CAPTURE_ENGINE_AUDIO_PROCESSING_RAW: MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE = 1;
pub type MF_CAPTURE_ENGINE_DEVICE_TYPE = i32;
pub const MF_CAPTURE_ENGINE_DEVICE_TYPE_AUDIO: MF_CAPTURE_ENGINE_DEVICE_TYPE = 0;
pub const MF_CAPTURE_ENGINE_DEVICE_TYPE_VIDEO: MF_CAPTURE_ENGINE_DEVICE_TYPE = 1;
pub const MF_CAPTURE_ENGINE_MEDIASOURCE: i32 = -1;
pub type MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = i32;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_COMMUNICATIONS: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 1;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_FARFIELDSPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 5;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_GAMECHAT: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 3;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_MEDIA: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 2;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_OTHER: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 0;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_SPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 4;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_UNIFORMSPEECH: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 6;
pub const MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_VOICETYPING: MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE = 7;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_AUDIO: i32 = -9;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_METADATA: i32 = -10;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_PHOTO: i32 = -8;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_VIDEO_PREVIEW: i32 = -6;
pub const MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_VIDEO_RECORD: i32 = -7;
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
