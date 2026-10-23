#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
#[inline]
pub unsafe fn MFCreateSinkWriterFromMediaSink<P0, P1>(pmediasink: P0, pattributes: P1) -> windows_core::Result<IMFSinkWriter>
where
    P0: windows_core::Param<super::IMFMediaSink>,
    P1: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromMediaSink(pmediasink : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsinkwriter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSinkWriterFromMediaSink(pmediasink.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSinkWriterFromURL<P0, P1, P2>(pwszoutputurl: P0, pbytestream: P1, pattributes: P2) -> windows_core::Result<IMFSinkWriter>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::IMFByteStream>,
    P2: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromURL(pwszoutputurl : windows_core::PCWSTR, pbytestream : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsinkwriter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSinkWriterFromURL(pwszoutputurl.param().abi(), pbytestream.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSourceReaderFromByteStream<P0, P1>(pbytestream: P0, pattributes: P1) -> windows_core::Result<IMFSourceReader>
where
    P0: windows_core::Param<super::IMFByteStream>,
    P1: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromByteStream(pbytestream : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSourceReaderFromByteStream(pbytestream.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
#[inline]
pub unsafe fn MFCreateSourceReaderFromMediaSource<P0, P1>(pmediasource: P0, pattributes: P1) -> windows_core::Result<IMFSourceReader>
where
    P0: windows_core::Param<super::IMFMediaSource>,
    P1: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromMediaSource(pmediasource : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSourceReaderFromMediaSource(pmediasource.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSourceReaderFromURL<P0, P1>(pwszurl: P0, pattributes: P1) -> windows_core::Result<IMFSourceReader>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromURL(pwszurl : windows_core::PCWSTR, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSourceReaderFromURL(pwszurl.param().abi(), pattributes.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
pub const CLSID_MFReadWriteClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0x48e2ed0f_98c2_4a37_bed5_166312ddd83f);
pub const CLSID_MFSinkWriter: windows_core::GUID = windows_core::GUID::from_u128(0xa3bbfb17_8273_4e52_9e0e_9739dc887990);
pub const CLSID_MFSourceReader: windows_core::GUID = windows_core::GUID::from_u128(0x1777133c_0881_411b_a577_ad545f0714c4);
windows_core::imp::define_interface!(IMFReadWriteClassFactory, IMFReadWriteClassFactory_Vtbl, 0xe7fe2e12_661c_40da_92f9_4f002ab67627);
windows_core::imp::interface_hierarchy!(IMFReadWriteClassFactory, windows_core::IUnknown);
impl IMFReadWriteClassFactory {
    #[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
    pub unsafe fn CreateInstanceFromURL<P1, P2>(&self, clsid: *const super::IID, pwszurl: P1, pattributes: P2, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceFromURL)(windows_core::Interface::as_raw(self), clsid, pwszurl.param().abi(), pattributes.param().abi(), riid, ppvobject as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
    pub unsafe fn CreateInstanceFromObject<P1, P2>(&self, clsid: *const super::IID, punkobject: P1, pattributes: P2, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceFromObject)(windows_core::Interface::as_raw(self), clsid, punkobject.param().abi(), pattributes.param().abi(), riid, ppvobject as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFReadWriteClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
    pub CreateInstanceFromURL: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, windows_core::PCWSTR, *mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef")))]
    CreateInstanceFromURL: usize,
    #[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
    pub CreateInstanceFromObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef")))]
    CreateInstanceFromObject: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
pub trait IMFReadWriteClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstanceFromURL(&self, clsid: *const super::IID, pwszurl: &windows_core::PCWSTR, pattributes: windows_core::Ref<super::IMFAttributes>, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
    fn CreateInstanceFromObject(&self, clsid: *const super::IID, punkobject: windows_core::Ref<windows_core::IUnknown>, pattributes: windows_core::Ref<super::IMFAttributes>, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
impl IMFReadWriteClassFactory_Vtbl {
    pub const fn new<Identity: IMFReadWriteClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstanceFromURL<Identity: IMFReadWriteClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const super::IID, pwszurl: windows_core::PCWSTR, pattributes: *mut core::ffi::c_void, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFReadWriteClassFactory_Impl::CreateInstanceFromURL(this, core::mem::transmute_copy(&clsid), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn CreateInstanceFromObject<Identity: IMFReadWriteClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const super::IID, punkobject: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef"))]
impl windows_core::RuntimeName for IMFReadWriteClassFactory {}
windows_core::imp::define_interface!(IMFSinkWriter, IMFSinkWriter_Vtbl, 0x3137f1cd_fe5e_4805_a5d8_fb477448cb3d);
windows_core::imp::interface_hierarchy!(IMFSinkWriter, windows_core::IUnknown);
impl IMFSinkWriter {
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn AddStream<P0>(&self, ptargetmediatype: P0) -> windows_core::Result<super::DWORD>
    where
        P0: windows_core::Param<super::IMFMediaType>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), ptargetmediatype.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetInputMediaType<P1, P2>(&self, dwstreamindex: super::DWORD, pinputmediatype: P1, pencodingparameters: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
        P2: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, pinputmediatype.param().abi(), pencodingparameters.param().abi()) }
    }
    pub unsafe fn BeginWriting(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginWriting)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn WriteSample<P1>(&self, dwstreamindex: super::DWORD, psample: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteSample)(windows_core::Interface::as_raw(self), dwstreamindex, psample.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SendStreamTick(&self, dwstreamindex: super::DWORD, lltimestamp: super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendStreamTick)(windows_core::Interface::as_raw(self), dwstreamindex, lltimestamp) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn PlaceMarker(&self, dwstreamindex: super::DWORD, pvcontext: super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PlaceMarker)(windows_core::Interface::as_raw(self), dwstreamindex, pvcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn NotifyEndOfSegment(&self, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NotifyEndOfSegment)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Flush(&self, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    pub unsafe fn Finalize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finalize)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetServiceForStream(&self, dwstreamindex: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetServiceForStream)(windows_core::Interface::as_raw(self), dwstreamindex, guidservice, riid, ppvobject as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatistics(&self, dwstreamindex: super::DWORD, pstats: *mut MF_SINK_WRITER_STATISTICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), dwstreamindex, pstats as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    AddStream: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetInputMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetInputMediaType: usize,
    pub BeginWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub WriteSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    WriteSample: usize,
    #[cfg(feature = "shared")]
    pub SendStreamTick: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SendStreamTick: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub PlaceMarker: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    PlaceMarker: usize,
    #[cfg(feature = "shared")]
    pub NotifyEndOfSegment: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    NotifyEndOfSegment: usize,
    #[cfg(feature = "shared")]
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Flush: usize,
    pub Finalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub GetServiceForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::GUID, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "shared")))]
    GetServiceForStream: usize,
    #[cfg(feature = "shared")]
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut MF_SINK_WRITER_STATISTICS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatistics: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared"))]
pub trait IMFSinkWriter_Impl: windows_core::IUnknownImpl {
    fn AddStream(&self, ptargetmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<super::DWORD>;
    fn SetInputMediaType(&self, dwstreamindex: super::DWORD, pinputmediatype: windows_core::Ref<super::IMFMediaType>, pencodingparameters: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
    fn BeginWriting(&self) -> windows_core::Result<()>;
    fn WriteSample(&self, dwstreamindex: super::DWORD, psample: windows_core::Ref<super::IMFSample>) -> windows_core::Result<()>;
    fn SendStreamTick(&self, dwstreamindex: super::DWORD, lltimestamp: super::LONGLONG) -> windows_core::Result<()>;
    fn PlaceMarker(&self, dwstreamindex: super::DWORD, pvcontext: super::LPVOID) -> windows_core::Result<()>;
    fn NotifyEndOfSegment(&self, dwstreamindex: super::DWORD) -> windows_core::Result<()>;
    fn Flush(&self, dwstreamindex: super::DWORD) -> windows_core::Result<()>;
    fn Finalize(&self) -> windows_core::Result<()>;
    fn GetServiceForStream(&self, dwstreamindex: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
    fn GetStatistics(&self, dwstreamindex: super::DWORD, pstats: *mut MF_SINK_WRITER_STATISTICS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared"))]
impl IMFSinkWriter_Vtbl {
    pub const fn new<Identity: IMFSinkWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddStream<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetmediatype: *mut core::ffi::c_void, pdwstreamindex: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetInputMediaType<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pinputmediatype: *mut core::ffi::c_void, pencodingparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn WriteSample<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::WriteSample(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn SendStreamTick<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, lltimestamp: super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::SendStreamTick(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&lltimestamp)).into()
            }
        }
        unsafe extern "system" fn PlaceMarker<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pvcontext: super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::PlaceMarker(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pvcontext)).into()
            }
        }
        unsafe extern "system" fn NotifyEndOfSegment<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::NotifyEndOfSegment(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetServiceForStream<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriter_Impl::GetServiceForStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: IMFSinkWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pstats: *mut MF_SINK_WRITER_STATISTICS) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IMFSinkWriter {}
windows_core::imp::define_interface!(IMFSinkWriterCallback, IMFSinkWriterCallback_Vtbl, 0x666f76de_33d2_41b9_a458_29ed0a972c58);
windows_core::imp::interface_hierarchy!(IMFSinkWriterCallback, windows_core::IUnknown);
impl IMFSinkWriterCallback {
    pub unsafe fn OnFinalize(&self, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnFinalize)(windows_core::Interface::as_raw(self), hrstatus) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn OnMarker(&self, dwstreamindex: super::DWORD, pvcontext: super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnMarker)(windows_core::Interface::as_raw(self), dwstreamindex, pvcontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriterCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnFinalize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub OnMarker: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    OnMarker: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IMFSinkWriterCallback_Impl: windows_core::IUnknownImpl {
    fn OnFinalize(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnMarker(&self, dwstreamindex: super::DWORD, pvcontext: super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IMFSinkWriterCallback_Vtbl {
    pub const fn new<Identity: IMFSinkWriterCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnFinalize<Identity: IMFSinkWriterCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterCallback_Impl::OnFinalize(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn OnMarker<Identity: IMFSinkWriterCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pvcontext: super::LPVOID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "shared"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn OnStreamError(&self, dwstreamindex: super::DWORD, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStreamError)(windows_core::Interface::as_raw(self), dwstreamindex, hrstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriterCallback2_Vtbl {
    pub base__: IMFSinkWriterCallback_Vtbl,
    pub OnTransformChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub OnStreamError: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnStreamError: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IMFSinkWriterCallback2_Impl: IMFSinkWriterCallback_Impl {
    fn OnTransformChange(&self) -> windows_core::Result<()>;
    fn OnStreamError(&self, dwstreamindex: super::DWORD, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IMFSinkWriterCallback2_Vtbl {
    pub const fn new<Identity: IMFSinkWriterCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTransformChange<Identity: IMFSinkWriterCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterCallback2_Impl::OnTransformChange(this).into()
            }
        }
        unsafe extern "system" fn OnStreamError<Identity: IMFSinkWriterCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IMFSinkWriterCallback2 {}
windows_core::imp::define_interface!(IMFSinkWriterEncoderConfig, IMFSinkWriterEncoderConfig_Vtbl, 0x17c3779e_3cde_4ede_8c60_3899f5f53ad6);
windows_core::imp::interface_hierarchy!(IMFSinkWriterEncoderConfig, windows_core::IUnknown);
impl IMFSinkWriterEncoderConfig {
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetTargetMediaType<P1, P2>(&self, dwstreamindex: super::DWORD, ptargetmediatype: P1, pencodingparameters: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
        P2: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, ptargetmediatype.param().abi(), pencodingparameters.param().abi()) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn PlaceEncodingParameters<P1>(&self, dwstreamindex: super::DWORD, pencodingparameters: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).PlaceEncodingParameters)(windows_core::Interface::as_raw(self), dwstreamindex, pencodingparameters.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriterEncoderConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetTargetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetTargetMediaType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub PlaceEncodingParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    PlaceEncodingParameters: usize,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub trait IMFSinkWriterEncoderConfig_Impl: windows_core::IUnknownImpl {
    fn SetTargetMediaType(&self, dwstreamindex: super::DWORD, ptargetmediatype: windows_core::Ref<super::IMFMediaType>, pencodingparameters: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
    fn PlaceEncodingParameters(&self, dwstreamindex: super::DWORD, pencodingparameters: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl IMFSinkWriterEncoderConfig_Vtbl {
    pub const fn new<Identity: IMFSinkWriterEncoderConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTargetMediaType<Identity: IMFSinkWriterEncoderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, ptargetmediatype: *mut core::ffi::c_void, pencodingparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSinkWriterEncoderConfig_Impl::SetTargetMediaType(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&ptargetmediatype), core::mem::transmute_copy(&pencodingparameters)).into()
            }
        }
        unsafe extern "system" fn PlaceEncodingParameters<Identity: IMFSinkWriterEncoderConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pencodingparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mfobjects", feature = "shared"))]
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
    #[cfg(all(feature = "mftransform", feature = "shared"))]
    pub unsafe fn GetTransformForStream(&self, dwstreamindex: super::DWORD, dwtransformindex: super::DWORD, pguidcategory: Option<*mut windows_core::GUID>, pptransform: *mut Option<super::IMFTransform>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransformForStream)(windows_core::Interface::as_raw(self), dwstreamindex, dwtransformindex, pguidcategory.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pptransform)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSinkWriterEx_Vtbl {
    pub base__: IMFSinkWriter_Vtbl,
    #[cfg(all(feature = "mftransform", feature = "shared"))]
    pub GetTransformForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mftransform", feature = "shared")))]
    GetTransformForStream: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "shared"))]
pub trait IMFSinkWriterEx_Impl: IMFSinkWriter_Impl {
    fn GetTransformForStream(&self, dwstreamindex: super::DWORD, dwtransformindex: super::DWORD, pguidcategory: *mut windows_core::GUID, pptransform: windows_core::OutRef<super::IMFTransform>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "shared"))]
impl IMFSinkWriterEx_Vtbl {
    pub const fn new<Identity: IMFSinkWriterEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransformForStream<Identity: IMFSinkWriterEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, dwtransformindex: super::DWORD, pguidcategory: *mut windows_core::GUID, pptransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IMFSinkWriterEx {}
windows_core::imp::define_interface!(IMFSourceReader, IMFSourceReader_Vtbl, 0x70ae66f2_c809_4e4f_8915_bdcb406b7993);
windows_core::imp::interface_hierarchy!(IMFSourceReader, windows_core::IUnknown);
impl IMFSourceReader {
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamSelection(&self, dwstreamindex: super::DWORD) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetStreamSelection(&self, dwstreamindex: super::DWORD, fselected: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, fselected.into()) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetNativeMediaType(&self, dwstreamindex: super::DWORD, dwmediatypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNativeMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, dwmediatypeindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn GetCurrentMediaType(&self, dwstreamindex: super::DWORD) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetCurrentMediaType<P2>(&self, dwstreamindex: super::DWORD, pdwreserved: Option<*mut super::DWORD>, pmediatype: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, pdwreserved.unwrap_or(core::mem::zeroed()) as _, pmediatype.param().abi()) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetCurrentPosition(&self, guidtimeformat: *const windows_core::GUID, varposition: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPosition)(windows_core::Interface::as_raw(self), guidtimeformat, varposition) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn ReadSample(&self, dwstreamindex: super::DWORD, dwcontrolflags: super::DWORD, pdwactualstreamindex: Option<*mut super::DWORD>, pdwstreamflags: Option<*mut super::DWORD>, plltimestamp: Option<*mut super::LONGLONG>, ppsample: Option<*mut Option<super::IMFSample>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadSample)(windows_core::Interface::as_raw(self), dwstreamindex, dwcontrolflags, pdwactualstreamindex.unwrap_or(core::mem::zeroed()) as _, pdwstreamflags.unwrap_or(core::mem::zeroed()) as _, plltimestamp.unwrap_or(core::mem::zeroed()) as _, ppsample.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Flush(&self, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetServiceForStream(&self, dwstreamindex: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetServiceForStream)(windows_core::Interface::as_raw(self), dwstreamindex, guidservice, riid, ppvobject as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPresentationAttribute(&self, dwstreamindex: super::DWORD, guidattribute: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT> {
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
    #[cfg(feature = "shared")]
    pub GetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamSelection: usize,
    #[cfg(feature = "shared")]
    pub SetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetStreamSelection: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetNativeMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetNativeMediaType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub GetCurrentMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    GetCurrentMediaType: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetCurrentMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetCurrentMediaType: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetCurrentPosition: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub ReadSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut super::DWORD, *mut super::DWORD, *mut super::LONGLONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    ReadSample: usize,
    #[cfg(feature = "shared")]
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Flush: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
    pub GetServiceForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::GUID, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "shared")))]
    GetServiceForStream: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPresentationAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetPresentationAttribute: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSourceReader_Impl: windows_core::IUnknownImpl {
    fn GetStreamSelection(&self, dwstreamindex: super::DWORD) -> windows_core::Result<windows_core::BOOL>;
    fn SetStreamSelection(&self, dwstreamindex: super::DWORD, fselected: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNativeMediaType(&self, dwstreamindex: super::DWORD, dwmediatypeindex: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn GetCurrentMediaType(&self, dwstreamindex: super::DWORD) -> windows_core::Result<super::IMFMediaType>;
    fn SetCurrentMediaType(&self, dwstreamindex: super::DWORD, pdwreserved: *mut super::DWORD, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
    fn SetCurrentPosition(&self, guidtimeformat: *const windows_core::GUID, varposition: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn ReadSample(&self, dwstreamindex: super::DWORD, dwcontrolflags: super::DWORD, pdwactualstreamindex: *mut super::DWORD, pdwstreamflags: *mut super::DWORD, plltimestamp: *mut super::LONGLONG, ppsample: windows_core::OutRef<super::IMFSample>) -> windows_core::Result<()>;
    fn Flush(&self, dwstreamindex: super::DWORD) -> windows_core::Result<()>;
    fn GetServiceForStream(&self, dwstreamindex: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
    fn GetPresentationAttribute(&self, dwstreamindex: super::DWORD, guidattribute: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSourceReader_Vtbl {
    pub const fn new<Identity: IMFSourceReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamSelection<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pfselected: *mut windows_core::BOOL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetStreamSelection<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, fselected: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetStreamSelection(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&fselected)).into()
            }
        }
        unsafe extern "system" fn GetNativeMediaType<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, dwmediatypeindex: super::DWORD, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCurrentMediaType<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetCurrentMediaType<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pdwreserved: *mut super::DWORD, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetCurrentMediaType(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pdwreserved), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtimeformat: *const windows_core::GUID, varposition: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetCurrentPosition(this, core::mem::transmute_copy(&guidtimeformat), core::mem::transmute_copy(&varposition)).into()
            }
        }
        unsafe extern "system" fn ReadSample<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, dwcontrolflags: super::DWORD, pdwactualstreamindex: *mut super::DWORD, pdwstreamflags: *mut super::DWORD, plltimestamp: *mut super::LONGLONG, ppsample: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::ReadSample(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwcontrolflags), core::mem::transmute_copy(&pdwactualstreamindex), core::mem::transmute_copy(&pdwstreamflags), core::mem::transmute_copy(&plltimestamp), core::mem::transmute_copy(&ppsample)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::Flush(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn GetServiceForStream<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::GetServiceForStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn GetPresentationAttribute<Identity: IMFSourceReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, guidattribute: *const windows_core::GUID, pvarattribute: *mut super::PROPVARIANT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSourceReader {}
windows_core::imp::define_interface!(IMFSourceReaderCallback, IMFSourceReaderCallback_Vtbl, 0xdeec8d99_fa1d_4d82_84c2_2c8969944867);
windows_core::imp::interface_hierarchy!(IMFSourceReaderCallback, windows_core::IUnknown);
impl IMFSourceReaderCallback {
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn OnReadSample<P4>(&self, hrstatus: windows_core::HRESULT, dwstreamindex: super::DWORD, dwstreamflags: super::DWORD, lltimestamp: super::LONGLONG, psample: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnReadSample)(windows_core::Interface::as_raw(self), hrstatus, dwstreamindex, dwstreamflags, lltimestamp, psample.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnFlush(&self, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnFlush)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn OnEvent<P1>(&self, dwstreamindex: super::DWORD, pevent: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), dwstreamindex, pevent.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceReaderCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub OnReadSample: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, super::DWORD, super::DWORD, super::LONGLONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    OnReadSample: usize,
    #[cfg(feature = "shared")]
    pub OnFlush: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnFlush: usize,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    OnEvent: usize,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub trait IMFSourceReaderCallback_Impl: windows_core::IUnknownImpl {
    fn OnReadSample(&self, hrstatus: windows_core::HRESULT, dwstreamindex: super::DWORD, dwstreamflags: super::DWORD, lltimestamp: super::LONGLONG, psample: windows_core::Ref<super::IMFSample>) -> windows_core::Result<()>;
    fn OnFlush(&self, dwstreamindex: super::DWORD) -> windows_core::Result<()>;
    fn OnEvent(&self, dwstreamindex: super::DWORD, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl IMFSourceReaderCallback_Vtbl {
    pub const fn new<Identity: IMFSourceReaderCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnReadSample<Identity: IMFSourceReaderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, dwstreamindex: super::DWORD, dwstreamflags: super::DWORD, lltimestamp: super::LONGLONG, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderCallback_Impl::OnReadSample(this, core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&dwstreamflags), core::mem::transmute_copy(&lltimestamp), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn OnFlush<Identity: IMFSourceReaderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderCallback_Impl::OnFlush(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn OnEvent<Identity: IMFSourceReaderCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mfobjects", feature = "shared"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn OnStreamError(&self, dwstreamindex: super::DWORD, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStreamError)(windows_core::Interface::as_raw(self), dwstreamindex, hrstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceReaderCallback2_Vtbl {
    pub base__: IMFSourceReaderCallback_Vtbl,
    pub OnTransformChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub OnStreamError: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnStreamError: usize,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
pub trait IMFSourceReaderCallback2_Impl: IMFSourceReaderCallback_Impl {
    fn OnTransformChange(&self) -> windows_core::Result<()>;
    fn OnStreamError(&self, dwstreamindex: super::DWORD, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl IMFSourceReaderCallback2_Vtbl {
    pub const fn new<Identity: IMFSourceReaderCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTransformChange<Identity: IMFSourceReaderCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderCallback2_Impl::OnTransformChange(this).into()
            }
        }
        unsafe extern "system" fn OnStreamError<Identity: IMFSourceReaderCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mfobjects", feature = "shared"))]
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
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub unsafe fn SetNativeMediaType<P1>(&self, dwstreamindex: super::DWORD, pmediatype: P1) -> windows_core::Result<super::DWORD>
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetNativeMediaType)(windows_core::Interface::as_raw(self), dwstreamindex, pmediatype.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddTransformForStream<P1>(&self, dwstreamindex: super::DWORD, ptransformoractivate: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddTransformForStream)(windows_core::Interface::as_raw(self), dwstreamindex, ptransformoractivate.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveAllTransformsForStream(&self, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllTransformsForStream)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
    #[cfg(all(feature = "mftransform", feature = "shared"))]
    pub unsafe fn GetTransformForStream(&self, dwstreamindex: super::DWORD, dwtransformindex: super::DWORD, pguidcategory: Option<*mut windows_core::GUID>, pptransform: *mut Option<super::IMFTransform>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransformForStream)(windows_core::Interface::as_raw(self), dwstreamindex, dwtransformindex, pguidcategory.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pptransform)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceReaderEx_Vtbl {
    pub base__: IMFSourceReader_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "shared"))]
    pub SetNativeMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "shared")))]
    SetNativeMediaType: usize,
    #[cfg(feature = "shared")]
    pub AddTransformForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddTransformForStream: usize,
    #[cfg(feature = "shared")]
    pub RemoveAllTransformsForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveAllTransformsForStream: usize,
    #[cfg(all(feature = "mftransform", feature = "shared"))]
    pub GetTransformForStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mftransform", feature = "shared")))]
    GetTransformForStream: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSourceReaderEx_Impl: IMFSourceReader_Impl {
    fn SetNativeMediaType(&self, dwstreamindex: super::DWORD, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<super::DWORD>;
    fn AddTransformForStream(&self, dwstreamindex: super::DWORD, ptransformoractivate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAllTransformsForStream(&self, dwstreamindex: super::DWORD) -> windows_core::Result<()>;
    fn GetTransformForStream(&self, dwstreamindex: super::DWORD, dwtransformindex: super::DWORD, pguidcategory: *mut windows_core::GUID, pptransform: windows_core::OutRef<super::IMFTransform>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSourceReaderEx_Vtbl {
    pub const fn new<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNativeMediaType<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, pmediatype: *mut core::ffi::c_void, pdwstreamflags: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddTransformForStream<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, ptransformoractivate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderEx_Impl::AddTransformForStream(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&ptransformoractivate)).into()
            }
        }
        unsafe extern "system" fn RemoveAllTransformsForStream<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReaderEx_Impl::RemoveAllTransformsForStream(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn GetTransformForStream<Identity: IMFSourceReaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: super::DWORD, dwtransformindex: super::DWORD, pguidcategory: *mut windows_core::GUID, pptransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSourceReaderEx {}
pub const MF_MEDIASINK_AUTOFINALIZE_SUPPORTED: windows_core::GUID = windows_core::GUID::from_u128(0x48c131be_135a_41cb_8290_03652509c999);
pub const MF_MEDIASINK_ENABLE_AUTOFINALIZE: windows_core::GUID = windows_core::GUID::from_u128(0x34014265_cb7e_4cde_ac7c_effd3b3c2530);
pub const MF_READWRITE_D3D_OPTIONAL: windows_core::GUID = windows_core::GUID::from_u128(0x216479d9_3071_42ca_bb6c_4c22102e1d18);
pub const MF_READWRITE_DISABLE_CONVERTERS: windows_core::GUID = windows_core::GUID::from_u128(0x98d5b065_1374_4847_8d5d_31520fee7156);
pub const MF_READWRITE_ENABLE_AUTOFINALIZE: windows_core::GUID = windows_core::GUID::from_u128(0xdd7ca129_8cd1_4dc5_9dde_ce168675de61);
pub const MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS: windows_core::GUID = windows_core::GUID::from_u128(0xa634a91c_822b_41b9_a494_4de4643612b0);
pub const MF_READWRITE_MMCSS_CLASS: windows_core::GUID = windows_core::GUID::from_u128(0x39384300_d0eb_40b1_87a0_3318871b5a53);
pub const MF_READWRITE_MMCSS_CLASS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x430847da_0890_4b0e_938c_054332c547e1);
pub const MF_READWRITE_MMCSS_PRIORITY: windows_core::GUID = windows_core::GUID::from_u128(0x43ad19ce_f33f_4ba9_a580_e4cd12f2d144);
pub const MF_READWRITE_MMCSS_PRIORITY_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x273db885_2de2_4db2_a6a7_fdb66fb40b61);
pub const MF_READWRITE_USE_ONLY_HARDWARE_TRANSFORMS: windows_core::GUID = windows_core::GUID::from_u128(0xf9074427_bf8b_4f69_bbaf_524969056fb6);
pub const MF_SINK_WRITER_ALL_STREAMS: i32 = -2;
pub const MF_SINK_WRITER_ASYNC_CALLBACK: windows_core::GUID = windows_core::GUID::from_u128(0x48cb183e_7b0b_46f4_822e_5e1d2dda4354);
pub const MF_SINK_WRITER_D3D_MANAGER: windows_core::GUID = windows_core::GUID::from_u128(0xec822da2_e1e9_4b29_a0d8_563c719f5269);
pub const MF_SINK_WRITER_DISABLE_THROTTLING: windows_core::GUID = windows_core::GUID::from_u128(0x08b845d8_2b74_4afe_9d53_be16d2d5ae4f);
pub const MF_SINK_WRITER_ENCODER_CONFIG: windows_core::GUID = windows_core::GUID::from_u128(0xad91cd04_a7cc_4ac7_99b6_a57b9a4a7c70);
pub const MF_SINK_WRITER_INVALID_STREAM_INDEX: i32 = -1;
pub const MF_SINK_WRITER_MEDIASINK: i32 = -1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MF_SINK_WRITER_STATISTICS {
    pub cb: super::DWORD,
    pub llLastTimestampReceived: super::LONGLONG,
    pub llLastTimestampEncoded: super::LONGLONG,
    pub llLastTimestampProcessed: super::LONGLONG,
    pub llLastStreamTickReceived: super::LONGLONG,
    pub llLastSinkSampleRequest: super::LONGLONG,
    pub qwNumSamplesReceived: super::QWORD,
    pub qwNumSamplesEncoded: super::QWORD,
    pub qwNumSamplesProcessed: super::QWORD,
    pub qwNumStreamTicksReceived: super::QWORD,
    pub dwByteCountQueued: super::DWORD,
    pub qwByteCountProcessed: super::QWORD,
    pub dwNumOutstandingSinkSampleRequests: super::DWORD,
    pub dwAverageSampleRateReceived: super::DWORD,
    pub dwAverageSampleRateEncoded: super::DWORD,
    pub dwAverageSampleRateProcessed: super::DWORD,
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
pub const MF_SOURCE_READER_ASYNC_CALLBACK: windows_core::GUID = windows_core::GUID::from_u128(0x1e3dbeac_bb43_4c35_b507_cd644464c965);
pub const MF_SOURCE_READER_CONTROLF_DRAIN: MF_SOURCE_READER_CONTROL_FLAG = 1;
pub type MF_SOURCE_READER_CONTROL_FLAG = i32;
pub const MF_SOURCE_READER_CURRENT_TYPE_INDEX: i32 = -1;
pub const MF_SOURCE_READER_D3D11_BIND_FLAGS: windows_core::GUID = windows_core::GUID::from_u128(0x33f3197b_f73a_4e14_8d85_0e4c4368788d);
pub const MF_SOURCE_READER_D3D_MANAGER: windows_core::GUID = windows_core::GUID::from_u128(0xec822da2_e1e9_4b29_a0d8_563c719f5269);
pub const MF_SOURCE_READER_DISABLE_CAMERA_PLUGINS: windows_core::GUID = windows_core::GUID::from_u128(0x9d3365dd_058f_4cfb_9f97_b314cc99c8ad);
pub const MF_SOURCE_READER_DISABLE_DXVA: windows_core::GUID = windows_core::GUID::from_u128(0xaa456cfd_3943_4a1e_a77d_1838c0ea2e35);
pub const MF_SOURCE_READER_DISCONNECT_MEDIASOURCE_ON_SHUTDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x56b67165_219e_456d_a22e_2d3004c7fe56);
pub const MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING: windows_core::GUID = windows_core::GUID::from_u128(0x0f81da2c_b537_4672_a8b2_a681b17307a3);
pub const MF_SOURCE_READER_ENABLE_TRANSCODE_ONLY_TRANSFORMS: windows_core::GUID = windows_core::GUID::from_u128(0xdfd4f008_b5fd_4e78_ae44_62a1e67bbe27);
pub const MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING: windows_core::GUID = windows_core::GUID::from_u128(0xfb394f3d_ccf1_42ee_bbb3_f9b845d5681d);
pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: i32 = -3;
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: i32 = -4;
pub type MF_SOURCE_READER_FLAG = i32;
pub const MF_SOURCE_READER_INVALID_STREAM_INDEX: i32 = -1;
pub const MF_SOURCE_READER_MEDIASOURCE: i32 = -1;
pub const MF_SOURCE_READER_MEDIASOURCE_CHARACTERISTICS: windows_core::GUID = windows_core::GUID::from_u128(0x6d23f5c8_c5d7_4a9b_9971_5d11f8bca880);
pub const MF_SOURCE_READER_MEDIASOURCE_CONFIG: windows_core::GUID = windows_core::GUID::from_u128(0x9085abeb_0354_48f9_abb5_200df838c68e);
pub const MF_SOURCE_READER_PASSTHROUGH_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x043ff126_fe2c_4708_a09b_da2ab435ced9);
