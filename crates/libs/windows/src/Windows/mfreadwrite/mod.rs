#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
#[inline]
pub unsafe fn MFCreateSinkWriterFromMediaSink<P0, P1>(pmediasink: P0, pattributes: P1) -> windows_core::Result<IMFSinkWriter>
where
    P0: windows_core::Param<super::mfidl::IMFMediaSink>,
    P1: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromMediaSink(pmediasink : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsinkwriter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSinkWriterFromMediaSink(pmediasink.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSinkWriterFromURL<P0, P1, P2>(pwszoutputurl: P0, pbytestream: P1, pattributes: P2) -> windows_core::Result<IMFSinkWriter>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::mfobjects::IMFByteStream>,
    P2: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromURL(pwszoutputurl : windows_core::PCWSTR, pbytestream : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsinkwriter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSinkWriterFromURL(pwszoutputurl.param().abi(), pbytestream.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSourceReaderFromByteStream<P0, P1>(pbytestream: P0, pattributes: P1) -> windows_core::Result<IMFSourceReader>
where
    P0: windows_core::Param<super::mfobjects::IMFByteStream>,
    P1: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromByteStream(pbytestream : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSourceReaderFromByteStream(pbytestream.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
#[inline]
pub unsafe fn MFCreateSourceReaderFromMediaSource<P0, P1>(pmediasource: P0, pattributes: P1) -> windows_core::Result<IMFSourceReader>
where
    P0: windows_core::Param<super::mfidl::IMFMediaSource>,
    P1: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromMediaSource(pmediasource : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSourceReaderFromMediaSource(pmediasource.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSourceReaderFromURL<P0, P1>(pwszurl: P0, pattributes: P1) -> windows_core::Result<IMFSourceReader>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromURL(pwszurl : windows_core::PCWSTR, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSourceReaderFromURL(pwszurl.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IMFReadWriteClassFactory, IMFReadWriteClassFactory_Vtbl, 0xe7fe2e12_661c_40da_92f9_4f002ab67627);
windows_core::imp::interface_hierarchy!(IMFReadWriteClassFactory, windows_core::IUnknown);
impl IMFReadWriteClassFactory {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn CreateInstanceFromURL<P1, P2, T>(&self, clsid: *const windows_core::GUID, pwszurl: P1, pattributes: P2) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::mfobjects::IMFAttributes>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceFromURL)(windows_core::Interface::as_raw(self), clsid, pwszurl.param().abi(), pattributes.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn CreateInstanceFromObject<P1, P2, T>(&self, clsid: *const windows_core::GUID, punkobject: P1, pattributes: P2) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<super::mfobjects::IMFAttributes>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceFromObject)(windows_core::Interface::as_raw(self), clsid, punkobject.param().abi(), pattributes.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFReadWriteClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub CreateInstanceFromURL: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    CreateInstanceFromURL: usize,
    #[cfg(feature = "mfobjects")]
    pub CreateInstanceFromObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    CreateInstanceFromObject: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFReadWriteClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstanceFromURL(&self, clsid: *const windows_core::GUID, pwszurl: &windows_core::PCWSTR, pattributes: windows_core::Ref<super::mfobjects::IMFAttributes>, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateInstanceFromObject(&self, clsid: *const windows_core::GUID, punkobject: windows_core::Ref<windows_core::IUnknown>, pattributes: windows_core::Ref<super::mfobjects::IMFAttributes>, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFReadWriteClassFactory_Vtbl {
    pub const fn new<Identity: IMFReadWriteClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstanceFromURL<Identity: IMFReadWriteClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, pwszurl: windows_core::PCWSTR, pattributes: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFReadWriteClassFactory_Impl::CreateInstanceFromURL(this, core::mem::transmute_copy(&clsid), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn CreateInstanceFromObject<Identity: IMFReadWriteClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, punkobject: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFReadWriteClassFactory_Impl::CreateInstanceFromObject(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&punkobject), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstanceFromURL: CreateInstanceFromURL::<Identity, OFFSET>,
            CreateInstanceFromObject: CreateInstanceFromObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFReadWriteClassFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFReadWriteClassFactory {}
windows_core::imp::define_interface!(IMFSinkWriter, IMFSinkWriter_Vtbl, 0x3137f1cd_fe5e_4805_a5d8_fb477448cb3d);
windows_core::imp::interface_hierarchy!(IMFSinkWriter, windows_core::IUnknown);
impl IMFSinkWriter {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn AddStream<P0>(&self, ptargetmediatype: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::mfobjects::IMFMediaType>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), ptargetmediatype.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetInputMediaType<P1, P2>(&self, dwstreamindex: u32, pinputmediatype: P1, pencodingparameters: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFMediaType>,
        P2: windows_core::Param<super::mfobjects::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, pinputmediatype.param().abi(), pencodingparameters.param().abi()) }
    }
    pub unsafe fn BeginWriting(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginWriting)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn WriteSample<P1>(&self, dwstreamindex: u32, psample: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteSample)(windows_core::Interface::as_raw(self), dwstreamindex, psample.param().abi()) }
    }
    pub unsafe fn SendStreamTick(&self, dwstreamindex: u32, lltimestamp: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendStreamTick)(windows_core::Interface::as_raw(self), dwstreamindex, lltimestamp) }
    }
    pub unsafe fn PlaceMarker(&self, dwstreamindex: u32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PlaceMarker)(windows_core::Interface::as_raw(self), dwstreamindex, pvcontext) }
    }
    pub unsafe fn NotifyEndOfSegment(&self, dwstreamindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NotifyEndOfSegment)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    pub unsafe fn Flush(&self, dwstreamindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    pub unsafe fn Finalize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finalize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetServiceForStream<T>(&self, dwstreamindex: u32, guidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetServiceForStream)(windows_core::Interface::as_raw(self), dwstreamindex, guidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetStatistics(&self, dwstreamindex: u32, pstats: *mut MF_SINK_WRITER_STATISTICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), dwstreamindex, pstats as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    AddStream: usize,
    #[cfg(feature = "mfobjects")]
    pub SetInputMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetInputMediaType: usize,
    pub BeginWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub WriteSample: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    WriteSample: usize,
    pub SendStreamTick: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i64) -> windows_core::HRESULT,
    pub PlaceMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyEndOfSegment: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServiceForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MF_SINK_WRITER_STATISTICS) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSinkWriter_Impl: windows_core::IUnknownImpl {
    fn AddStream(&self, ptargetmediatype: windows_core::Ref<super::mfobjects::IMFMediaType>) -> windows_core::Result<u32>;
    fn SetInputMediaType(&self, dwstreamindex: u32, pinputmediatype: windows_core::Ref<super::mfobjects::IMFMediaType>, pencodingparameters: windows_core::Ref<super::mfobjects::IMFAttributes>) -> windows_core::Result<()>;
    fn BeginWriting(&self) -> windows_core::Result<()>;
    fn WriteSample(&self, dwstreamindex: u32, psample: windows_core::Ref<super::mfobjects::IMFSample>) -> windows_core::Result<()>;
    fn SendStreamTick(&self, dwstreamindex: u32, lltimestamp: i64) -> windows_core::Result<()>;
    fn PlaceMarker(&self, dwstreamindex: u32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn NotifyEndOfSegment(&self, dwstreamindex: u32) -> windows_core::Result<()>;
    fn Flush(&self, dwstreamindex: u32) -> windows_core::Result<()>;
    fn Finalize(&self) -> windows_core::Result<()>;
    fn GetServiceForStream(&self, dwstreamindex: u32, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStatistics(&self, dwstreamindex: u32, pstats: *mut MF_SINK_WRITER_STATISTICS) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFSinkWriter_Vtbl {
    pub const fn new<Identity: IMFSinkWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddStream<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetmediatype: *mut core::ffi::c_void, pdwstreamindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSinkWriter_Impl::AddStream(this, core::mem::transmute_copy(&ptargetmediatype)) {
                    Ok(ok__) => {
                        pdwstreamindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputMediaType<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pinputmediatype: *mut core::ffi::c_void, pencodingparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::SetInputMediaType(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pinputmediatype), core::mem::transmute_copy(&pencodingparameters)).into()
            }
        }
        unsafe extern "system" fn BeginWriting<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::BeginWriting(this).into()
            }
        }
        unsafe extern "system" fn WriteSample<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::WriteSample(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn SendStreamTick<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, lltimestamp: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::SendStreamTick(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&lltimestamp)).into()
            }
        }
        unsafe extern "system" fn PlaceMarker<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::PlaceMarker(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn NotifyEndOfSegment<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::NotifyEndOfSegment(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::Flush(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn Finalize<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::Finalize(this).into()
            }
        }
        unsafe extern "system" fn GetServiceForStream<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::GetServiceForStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pstats: *mut MF_SINK_WRITER_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::GetStatistics(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pstats)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddStream: AddStream::<Identity, OFFSET>,
            SetInputMediaType: SetInputMediaType::<Identity, OFFSET>,
            BeginWriting: BeginWriting::<Identity, OFFSET>,
            WriteSample: WriteSample::<Identity, OFFSET>,
            SendStreamTick: SendStreamTick::<Identity, OFFSET>,
            PlaceMarker: PlaceMarker::<Identity, OFFSET>,
            NotifyEndOfSegment: NotifyEndOfSegment::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            Finalize: Finalize::<Identity, OFFSET>,
            GetServiceForStream: GetServiceForStream::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSinkWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSinkWriter {}
windows_core::imp::define_interface!(IMFSinkWriterCallback, IMFSinkWriterCallback_Vtbl, 0x666f76de_33d2_41b9_a458_29ed0a972c58);
windows_core::imp::interface_hierarchy!(IMFSinkWriterCallback, windows_core::IUnknown);
impl IMFSinkWriterCallback {
    pub unsafe fn OnFinalize(&self, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnFinalize)(windows_core::Interface::as_raw(self), hrstatus) }
    }
    pub unsafe fn OnMarker(&self, dwstreamindex: u32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnMarker)(windows_core::Interface::as_raw(self), dwstreamindex, pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriterCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnFinalize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFSinkWriterCallback_Impl: windows_core::IUnknownImpl {
    fn OnFinalize(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnMarker(&self, dwstreamindex: u32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFSinkWriterCallback_Vtbl {
    pub const fn new<Identity: IMFSinkWriterCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnFinalize<Identity: IMFSinkWriterCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterCallback_Impl::OnFinalize(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn OnMarker<Identity: IMFSinkWriterCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterCallback_Impl::OnMarker(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnFinalize: OnFinalize::<Identity, OFFSET>,
            OnMarker: OnMarker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSinkWriterCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSinkWriterCallback {}
windows_core::imp::define_interface!(IMFSinkWriterCallback2, IMFSinkWriterCallback2_Vtbl, 0x2456bd58_c067_4513_84fe_8d0c88ffdc61);
impl core::ops::Deref for IMFSinkWriterCallback2 {
    type Target = IMFSinkWriterCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFSinkWriterCallback2, windows_core::IUnknown, IMFSinkWriterCallback);
impl IMFSinkWriterCallback2 {
    pub unsafe fn OnTransformChange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTransformChange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnStreamError(&self, dwstreamindex: u32, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStreamError)(windows_core::Interface::as_raw(self), dwstreamindex, hrstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriterCallback2_Vtbl {
    pub base__: IMFSinkWriterCallback_Vtbl,
    pub OnTransformChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnStreamError: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IMFSinkWriterCallback2_Impl: IMFSinkWriterCallback_Impl {
    fn OnTransformChange(&self) -> windows_core::Result<()>;
    fn OnStreamError(&self, dwstreamindex: u32, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IMFSinkWriterCallback2_Vtbl {
    pub const fn new<Identity: IMFSinkWriterCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTransformChange<Identity: IMFSinkWriterCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterCallback2_Impl::OnTransformChange(this).into()
            }
        }
        unsafe extern "system" fn OnStreamError<Identity: IMFSinkWriterCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterCallback2_Impl::OnStreamError(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self {
            base__: IMFSinkWriterCallback_Vtbl::new::<Identity, OFFSET>(),
            OnTransformChange: OnTransformChange::<Identity, OFFSET>,
            OnStreamError: OnStreamError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSinkWriterCallback2 as windows_core::Interface>::IID || iid == &<IMFSinkWriterCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSinkWriterCallback2 {}
windows_core::imp::define_interface!(IMFSinkWriterEncoderConfig, IMFSinkWriterEncoderConfig_Vtbl, 0x17c3779e_3cde_4ede_8c60_3899f5f53ad6);
windows_core::imp::interface_hierarchy!(IMFSinkWriterEncoderConfig, windows_core::IUnknown);
impl IMFSinkWriterEncoderConfig {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetTargetMediaType<P1, P2>(&self, dwstreamindex: u32, ptargetmediatype: P1, pencodingparameters: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFMediaType>,
        P2: windows_core::Param<super::mfobjects::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, ptargetmediatype.param().abi(), pencodingparameters.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn PlaceEncodingParameters<P1>(&self, dwstreamindex: u32, pencodingparameters: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).PlaceEncodingParameters)(windows_core::Interface::as_raw(self), dwstreamindex, pencodingparameters.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriterEncoderConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetTargetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetTargetMediaType: usize,
    #[cfg(feature = "mfobjects")]
    pub PlaceEncodingParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    PlaceEncodingParameters: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSinkWriterEncoderConfig_Impl: windows_core::IUnknownImpl {
    fn SetTargetMediaType(&self, dwstreamindex: u32, ptargetmediatype: windows_core::Ref<super::mfobjects::IMFMediaType>, pencodingparameters: windows_core::Ref<super::mfobjects::IMFAttributes>) -> windows_core::Result<()>;
    fn PlaceEncodingParameters(&self, dwstreamindex: u32, pencodingparameters: windows_core::Ref<super::mfobjects::IMFAttributes>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFSinkWriterEncoderConfig_Vtbl {
    pub const fn new<Identity: IMFSinkWriterEncoderConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTargetMediaType<Identity: IMFSinkWriterEncoderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, ptargetmediatype: *mut core::ffi::c_void, pencodingparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterEncoderConfig_Impl::SetTargetMediaType(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&ptargetmediatype), core::mem::transmute_copy(&pencodingparameters)).into()
            }
        }
        unsafe extern "system" fn PlaceEncodingParameters<Identity: IMFSinkWriterEncoderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pencodingparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterEncoderConfig_Impl::PlaceEncodingParameters(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pencodingparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTargetMediaType: SetTargetMediaType::<Identity, OFFSET>,
            PlaceEncodingParameters: PlaceEncodingParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSinkWriterEncoderConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSinkWriterEncoderConfig {}
windows_core::imp::define_interface!(IMFSinkWriterEx, IMFSinkWriterEx_Vtbl, 0x588d72ab_5bc1_496a_8714_b70617141b25);
impl core::ops::Deref for IMFSinkWriterEx {
    type Target = IMFSinkWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFSinkWriterEx, windows_core::IUnknown, IMFSinkWriter);
impl IMFSinkWriterEx {
    #[cfg(feature = "mftransform")]
    pub unsafe fn GetTransformForStream(&self, dwstreamindex: u32, dwtransformindex: u32, pguidcategory: Option<*mut windows_core::GUID>, pptransform: *mut Option<super::mftransform::IMFTransform>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransformForStream)(windows_core::Interface::as_raw(self), dwstreamindex, dwtransformindex, pguidcategory.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pptransform)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriterEx_Vtbl {
    pub base__: IMFSinkWriter_Vtbl,
    #[cfg(feature = "mftransform")]
    pub GetTransformForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mftransform"))]
    GetTransformForStream: usize,
}
#[cfg(all(feature = "mfobjects", feature = "mftransform"))]
pub trait IMFSinkWriterEx_Impl: IMFSinkWriter_Impl {
    fn GetTransformForStream(&self, dwstreamindex: u32, dwtransformindex: u32, pguidcategory: *mut windows_core::GUID, pptransform: windows_core::OutRef<super::mftransform::IMFTransform>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "mftransform"))]
impl IMFSinkWriterEx_Vtbl {
    pub const fn new<Identity: IMFSinkWriterEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransformForStream<Identity: IMFSinkWriterEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, dwtransformindex: u32, pguidcategory: *mut windows_core::GUID, pptransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterEx_Impl::GetTransformForStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwtransformindex), core::mem::transmute_copy(&pguidcategory), core::mem::transmute_copy(&pptransform)).into()
            }
        }
        Self { base__: IMFSinkWriter_Vtbl::new::<Identity, OFFSET>(), GetTransformForStream: GetTransformForStream::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSinkWriterEx as windows_core::Interface>::IID || iid == &<IMFSinkWriter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "mftransform"))]
impl windows_core::RuntimeName for IMFSinkWriterEx {}
windows_core::imp::define_interface!(IMFSourceReader, IMFSourceReader_Vtbl, 0x70ae66f2_c809_4e4f_8915_bdcb406b7993);
windows_core::imp::interface_hierarchy!(IMFSourceReader, windows_core::IUnknown);
impl IMFSourceReader {
    pub unsafe fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStreamSelection(&self, dwstreamindex: u32, fselected: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, fselected.into()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetNativeMediaType(&self, dwstreamindex: u32, dwmediatypeindex: u32) -> windows_core::Result<super::mfobjects::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNativeMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, dwmediatypeindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetCurrentMediaType(&self, dwstreamindex: u32) -> windows_core::Result<super::mfobjects::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetCurrentMediaType<P2>(&self, dwstreamindex: u32, pdwreserved: Option<*const u32>, pmediatype: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::mfobjects::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, pdwreserved.unwrap_or(core::mem::zeroed()) as _, pmediatype.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetCurrentPosition(&self, guidtimeformat: *const windows_core::GUID, varposition: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPosition)(windows_core::Interface::as_raw(self), guidtimeformat, varposition) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn ReadSample(&self, dwstreamindex: u32, dwcontrolflags: u32, pdwactualstreamindex: Option<*mut u32>, pdwstreamflags: Option<*mut u32>, plltimestamp: Option<*mut i64>, ppsample: Option<*mut Option<super::mfobjects::IMFSample>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadSample)(windows_core::Interface::as_raw(self), dwstreamindex, dwcontrolflags, pdwactualstreamindex.unwrap_or(core::mem::zeroed()) as _, pdwstreamflags.unwrap_or(core::mem::zeroed()) as _, plltimestamp.unwrap_or(core::mem::zeroed()) as _, ppsample.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Flush(&self, dwstreamindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    pub unsafe fn GetServiceForStream<T>(&self, dwstreamindex: u32, guidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetServiceForStream)(windows_core::Interface::as_raw(self), dwstreamindex, guidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPresentationAttribute(&self, dwstreamindex: u32, guidattribute: *const windows_core::GUID) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentationAttribute)(windows_core::Interface::as_raw(self), dwstreamindex, guidattribute, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetNativeMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetNativeMediaType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetCurrentMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetCurrentMediaType: usize,
    #[cfg(feature = "mfobjects")]
    pub SetCurrentMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetCurrentMediaType: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetCurrentPosition: usize,
    #[cfg(feature = "mfobjects")]
    pub ReadSample: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, *mut u32, *mut i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    ReadSample: usize,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetServiceForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPresentationAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetPresentationAttribute: usize,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSourceReader_Impl: windows_core::IUnknownImpl {
    fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetStreamSelection(&self, dwstreamindex: u32, fselected: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNativeMediaType(&self, dwstreamindex: u32, dwmediatypeindex: u32) -> windows_core::Result<super::mfobjects::IMFMediaType>;
    fn GetCurrentMediaType(&self, dwstreamindex: u32) -> windows_core::Result<super::mfobjects::IMFMediaType>;
    fn SetCurrentMediaType(&self, dwstreamindex: u32, pdwreserved: *const u32, pmediatype: windows_core::Ref<super::mfobjects::IMFMediaType>) -> windows_core::Result<()>;
    fn SetCurrentPosition(&self, guidtimeformat: *const windows_core::GUID, varposition: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn ReadSample(&self, dwstreamindex: u32, dwcontrolflags: u32, pdwactualstreamindex: *mut u32, pdwstreamflags: *mut u32, plltimestamp: *mut i64, ppsample: windows_core::OutRef<super::mfobjects::IMFSample>) -> windows_core::Result<()>;
    fn Flush(&self, dwstreamindex: u32) -> windows_core::Result<()>;
    fn GetServiceForStream(&self, dwstreamindex: u32, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPresentationAttribute(&self, dwstreamindex: u32, guidattribute: *const windows_core::GUID) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSourceReader_Vtbl {
    pub const fn new<Identity: IMFSourceReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamSelection<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pfselected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReader_Impl::GetStreamSelection(this, core::mem::transmute_copy(&dwstreamindex)) {
                    Ok(ok__) => {
                        pfselected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamSelection<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, fselected: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetStreamSelection(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&fselected)).into()
            }
        }
        unsafe extern "system" fn GetNativeMediaType<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, dwmediatypeindex: u32, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReader_Impl::GetNativeMediaType(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwmediatypeindex)) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentMediaType<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReader_Impl::GetCurrentMediaType(this, core::mem::transmute_copy(&dwstreamindex)) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentMediaType<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pdwreserved: *const u32, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetCurrentMediaType(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pdwreserved), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtimeformat: *const windows_core::GUID, varposition: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetCurrentPosition(this, core::mem::transmute_copy(&guidtimeformat), core::mem::transmute_copy(&varposition)).into()
            }
        }
        unsafe extern "system" fn ReadSample<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, dwcontrolflags: u32, pdwactualstreamindex: *mut u32, pdwstreamflags: *mut u32, plltimestamp: *mut i64, ppsample: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::ReadSample(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwcontrolflags), core::mem::transmute_copy(&pdwactualstreamindex), core::mem::transmute_copy(&pdwstreamflags), core::mem::transmute_copy(&plltimestamp), core::mem::transmute_copy(&ppsample)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::Flush(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn GetServiceForStream<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::GetServiceForStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn GetPresentationAttribute<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, guidattribute: *const windows_core::GUID, pvarattribute: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReader_Impl::GetPresentationAttribute(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&guidattribute)) {
                    Ok(ok__) => {
                        pvarattribute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamSelection: GetStreamSelection::<Identity, OFFSET>,
            SetStreamSelection: SetStreamSelection::<Identity, OFFSET>,
            GetNativeMediaType: GetNativeMediaType::<Identity, OFFSET>,
            GetCurrentMediaType: GetCurrentMediaType::<Identity, OFFSET>,
            SetCurrentMediaType: SetCurrentMediaType::<Identity, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, OFFSET>,
            ReadSample: ReadSample::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            GetServiceForStream: GetServiceForStream::<Identity, OFFSET>,
            GetPresentationAttribute: GetPresentationAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSourceReader {}
windows_core::imp::define_interface!(IMFSourceReaderCallback, IMFSourceReaderCallback_Vtbl, 0xdeec8d99_fa1d_4d82_84c2_2c8969944867);
windows_core::imp::interface_hierarchy!(IMFSourceReaderCallback, windows_core::IUnknown);
impl IMFSourceReaderCallback {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnReadSample<P4>(&self, hrstatus: windows_core::HRESULT, dwstreamindex: u32, dwstreamflags: u32, lltimestamp: i64, psample: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::mfobjects::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnReadSample)(windows_core::Interface::as_raw(self), hrstatus, dwstreamindex, dwstreamflags, lltimestamp, psample.param().abi()) }
    }
    pub unsafe fn OnFlush(&self, dwstreamindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnFlush)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnEvent<P1>(&self, dwstreamindex: u32, pevent: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), dwstreamindex, pevent.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceReaderCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub OnReadSample: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, u32, i64, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    OnReadSample: usize,
    pub OnFlush: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    OnEvent: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSourceReaderCallback_Impl: windows_core::IUnknownImpl {
    fn OnReadSample(&self, hrstatus: windows_core::HRESULT, dwstreamindex: u32, dwstreamflags: u32, lltimestamp: i64, psample: windows_core::Ref<super::mfobjects::IMFSample>) -> windows_core::Result<()>;
    fn OnFlush(&self, dwstreamindex: u32) -> windows_core::Result<()>;
    fn OnEvent(&self, dwstreamindex: u32, pevent: windows_core::Ref<super::mfobjects::IMFMediaEvent>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFSourceReaderCallback_Vtbl {
    pub const fn new<Identity: IMFSourceReaderCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnReadSample<Identity: IMFSourceReaderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, dwstreamindex: u32, dwstreamflags: u32, lltimestamp: i64, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderCallback_Impl::OnReadSample(this, core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwstreamflags), core::mem::transmute_copy(&lltimestamp), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn OnFlush<Identity: IMFSourceReaderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderCallback_Impl::OnFlush(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn OnEvent<Identity: IMFSourceReaderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderCallback_Impl::OnEvent(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pevent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnReadSample: OnReadSample::<Identity, OFFSET>,
            OnFlush: OnFlush::<Identity, OFFSET>,
            OnEvent: OnEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceReaderCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSourceReaderCallback {}
windows_core::imp::define_interface!(IMFSourceReaderCallback2, IMFSourceReaderCallback2_Vtbl, 0xcf839fe6_8c2a_4dd2_b6ea_c22d6961af05);
impl core::ops::Deref for IMFSourceReaderCallback2 {
    type Target = IMFSourceReaderCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFSourceReaderCallback2, windows_core::IUnknown, IMFSourceReaderCallback);
impl IMFSourceReaderCallback2 {
    pub unsafe fn OnTransformChange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTransformChange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnStreamError(&self, dwstreamindex: u32, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStreamError)(windows_core::Interface::as_raw(self), dwstreamindex, hrstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceReaderCallback2_Vtbl {
    pub base__: IMFSourceReaderCallback_Vtbl,
    pub OnTransformChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnStreamError: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSourceReaderCallback2_Impl: IMFSourceReaderCallback_Impl {
    fn OnTransformChange(&self) -> windows_core::Result<()>;
    fn OnStreamError(&self, dwstreamindex: u32, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFSourceReaderCallback2_Vtbl {
    pub const fn new<Identity: IMFSourceReaderCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTransformChange<Identity: IMFSourceReaderCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderCallback2_Impl::OnTransformChange(this).into()
            }
        }
        unsafe extern "system" fn OnStreamError<Identity: IMFSourceReaderCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderCallback2_Impl::OnStreamError(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self {
            base__: IMFSourceReaderCallback_Vtbl::new::<Identity, OFFSET>(),
            OnTransformChange: OnTransformChange::<Identity, OFFSET>,
            OnStreamError: OnStreamError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceReaderCallback2 as windows_core::Interface>::IID || iid == &<IMFSourceReaderCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSourceReaderCallback2 {}
windows_core::imp::define_interface!(IMFSourceReaderEx, IMFSourceReaderEx_Vtbl, 0x7b981cf0_560e_4116_9875_b099895f23d7);
impl core::ops::Deref for IMFSourceReaderEx {
    type Target = IMFSourceReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFSourceReaderEx, windows_core::IUnknown, IMFSourceReader);
impl IMFSourceReaderEx {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetNativeMediaType<P1>(&self, dwstreamindex: u32, pmediatype: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<super::mfobjects::IMFMediaType>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetNativeMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, pmediatype.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddTransformForStream<P1>(&self, dwstreamindex: u32, ptransformoractivate: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddTransformForStream)(windows_core::Interface::as_raw(self), dwstreamindex, ptransformoractivate.param().abi()) }
    }
    pub unsafe fn RemoveAllTransformsForStream(&self, dwstreamindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllTransformsForStream)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    #[cfg(feature = "mftransform")]
    pub unsafe fn GetTransformForStream(&self, dwstreamindex: u32, dwtransformindex: u32, pguidcategory: Option<*mut windows_core::GUID>, pptransform: *mut Option<super::mftransform::IMFTransform>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransformForStream)(windows_core::Interface::as_raw(self), dwstreamindex, dwtransformindex, pguidcategory.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pptransform)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceReaderEx_Vtbl {
    pub base__: IMFSourceReader_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetNativeMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetNativeMediaType: usize,
    pub AddTransformForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllTransformsForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "mftransform")]
    pub GetTransformForStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mftransform"))]
    GetTransformForStream: usize,
}
#[cfg(all(feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSourceReaderEx_Impl: IMFSourceReader_Impl {
    fn SetNativeMediaType(&self, dwstreamindex: u32, pmediatype: windows_core::Ref<super::mfobjects::IMFMediaType>) -> windows_core::Result<u32>;
    fn AddTransformForStream(&self, dwstreamindex: u32, ptransformoractivate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAllTransformsForStream(&self, dwstreamindex: u32) -> windows_core::Result<()>;
    fn GetTransformForStream(&self, dwstreamindex: u32, dwtransformindex: u32, pguidcategory: *mut windows_core::GUID, pptransform: windows_core::OutRef<super::mftransform::IMFTransform>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSourceReaderEx_Vtbl {
    pub const fn new<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNativeMediaType<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pmediatype: *mut core::ffi::c_void, pdwstreamflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReaderEx_Impl::SetNativeMediaType(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pmediatype)) {
                    Ok(ok__) => {
                        pdwstreamflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTransformForStream<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, ptransformoractivate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderEx_Impl::AddTransformForStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&ptransformoractivate)).into()
            }
        }
        unsafe extern "system" fn RemoveAllTransformsForStream<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderEx_Impl::RemoveAllTransformsForStream(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn GetTransformForStream<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, dwtransformindex: u32, pguidcategory: *mut windows_core::GUID, pptransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderEx_Impl::GetTransformForStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwtransformindex), core::mem::transmute_copy(&pguidcategory), core::mem::transmute_copy(&pptransform)).into()
            }
        }
        Self {
            base__: IMFSourceReader_Vtbl::new::<Identity, OFFSET>(),
            SetNativeMediaType: SetNativeMediaType::<Identity, OFFSET>,
            AddTransformForStream: AddTransformForStream::<Identity, OFFSET>,
            RemoveAllTransformsForStream: RemoveAllTransformsForStream::<Identity, OFFSET>,
            GetTransformForStream: GetTransformForStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceReaderEx as windows_core::Interface>::IID || iid == &<IMFSourceReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSourceReaderEx {}
pub const MF_SINK_WRITER_ALL_STREAMS: i32 = -2;
pub const MF_SINK_WRITER_INVALID_STREAM_INDEX: i32 = -1;
pub const MF_SINK_WRITER_MEDIASINK: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MF_SINK_WRITER_STATISTICS {
    pub cb: u32,
    pub llLastTimestampReceived: i64,
    pub llLastTimestampEncoded: i64,
    pub llLastTimestampProcessed: i64,
    pub llLastStreamTickReceived: i64,
    pub llLastSinkSampleRequest: i64,
    pub qwNumSamplesReceived: u64,
    pub qwNumSamplesEncoded: u64,
    pub qwNumSamplesProcessed: u64,
    pub qwNumStreamTicksReceived: u64,
    pub dwByteCountQueued: u32,
    pub qwByteCountProcessed: u64,
    pub dwNumOutstandingSinkSampleRequests: u32,
    pub dwAverageSampleRateReceived: u32,
    pub dwAverageSampleRateEncoded: u32,
    pub dwAverageSampleRateProcessed: u32,
}
pub const MF_SOURCE_READERF_ALLEFFECTSREMOVED: MF_SOURCE_READER_FLAG = 512;
pub const MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 32;
pub const MF_SOURCE_READERF_ENDOFSTREAM: MF_SOURCE_READER_FLAG = 2;
pub const MF_SOURCE_READERF_ERROR: MF_SOURCE_READER_FLAG = 1;
pub const MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 16;
pub const MF_SOURCE_READERF_NEWSTREAM: MF_SOURCE_READER_FLAG = 4;
pub const MF_SOURCE_READERF_STREAMTICK: MF_SOURCE_READER_FLAG = 256;
pub const MF_SOURCE_READER_ALL_STREAMS: i32 = -2;
pub const MF_SOURCE_READER_ANY_STREAM: i32 = -2;
pub const MF_SOURCE_READER_CONTROLF_DRAIN: MF_SOURCE_READER_CONTROL_FLAG = 1;
pub type MF_SOURCE_READER_CONTROL_FLAG = u32;
pub const MF_SOURCE_READER_CURRENT_TYPE_INDEX: i32 = -1;
pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: i32 = -3;
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: i32 = -4;
pub type MF_SOURCE_READER_FLAG = u32;
pub const MF_SOURCE_READER_INVALID_STREAM_INDEX: i32 = -1;
pub const MF_SOURCE_READER_MEDIASOURCE: i32 = -1;
