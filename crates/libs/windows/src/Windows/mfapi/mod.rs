#[inline]
pub unsafe fn MFAddPeriodicCallback<P1>(callback: MFPERIODICCALLBACK, pcontext: P1, pdwkey: Option<*mut u32>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFAddPeriodicCallback(callback : MFPERIODICCALLBACK, pcontext : *mut core::ffi::c_void, pdwkey : *mut u32) -> windows_core::HRESULT);
    unsafe { MFAddPeriodicCallback(callback, pcontext.param().abi(), pdwkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MFAllocateSerialWorkQueue(dwworkqueue: u32) -> windows_core::Result<u32> {
    windows_core::link!("mfplat.dll" "system" fn MFAllocateSerialWorkQueue(dwworkqueue : u32, pdwworkqueue : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFAllocateSerialWorkQueue(dwworkqueue, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFAllocateWorkQueue() -> windows_core::Result<u32> {
    windows_core::link!("mfplat.dll" "system" fn MFAllocateWorkQueue(pdwworkqueue : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFAllocateWorkQueue(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFAllocateWorkQueueEx(workqueuetype: MFASYNC_WORKQUEUE_TYPE) -> windows_core::Result<u32> {
    windows_core::link!("mfplat.dll" "system" fn MFAllocateWorkQueueEx(workqueuetype : MFASYNC_WORKQUEUE_TYPE, pdwworkqueue : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFAllocateWorkQueueEx(workqueuetype, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFAverageTimePerFrameToFrameRate(unaveragetimeperframe: u64, punnumerator: *mut u32, pundenominator: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFAverageTimePerFrameToFrameRate(unaveragetimeperframe : u64, punnumerator : *mut u32, pundenominator : *mut u32) -> windows_core::HRESULT);
    unsafe { MFAverageTimePerFrameToFrameRate(unaveragetimeperframe, punnumerator as _, pundenominator as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFBeginCreateFile<P3, P4, P5>(accessmode: super::mfobjects::MF_FILE_ACCESSMODE, openmode: super::mfobjects::MF_FILE_OPENMODE, fflags: super::mfobjects::MF_FILE_FLAGS, pwszfilepath: P3, pcallback: P4, pstate: P5) -> windows_core::Result<windows_core::IUnknown>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    P5: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFBeginCreateFile(accessmode : super::mfobjects::MF_FILE_ACCESSMODE, openmode : super::mfobjects::MF_FILE_OPENMODE, fflags : super::mfobjects::MF_FILE_FLAGS, pwszfilepath : windows_core::PCWSTR, pcallback : *mut core::ffi::c_void, pstate : *mut core::ffi::c_void, ppcancelcookie : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFBeginCreateFile(accessmode, openmode, fflags, pwszfilepath.param().abi(), pcallback.param().abi(), pstate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFBeginRegisterWorkQueueWithMMCSS<P1, P3, P4>(dwworkqueueid: u32, wszclass: P1, dwtaskid: u32, pdonecallback: P3, pdonestate: P4) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    P4: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFBeginRegisterWorkQueueWithMMCSS(dwworkqueueid : u32, wszclass : windows_core::PCWSTR, dwtaskid : u32, pdonecallback : *mut core::ffi::c_void, pdonestate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFBeginRegisterWorkQueueWithMMCSS(dwworkqueueid, wszclass.param().abi(), dwtaskid, pdonecallback.param().abi(), pdonestate.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFBeginRegisterWorkQueueWithMMCSSEx<P1, P4, P5>(dwworkqueueid: u32, wszclass: P1, dwtaskid: u32, lpriority: i32, pdonecallback: P4, pdonestate: P5) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    P5: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFBeginRegisterWorkQueueWithMMCSSEx(dwworkqueueid : u32, wszclass : windows_core::PCWSTR, dwtaskid : u32, lpriority : i32, pdonecallback : *mut core::ffi::c_void, pdonestate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFBeginRegisterWorkQueueWithMMCSSEx(dwworkqueueid, wszclass.param().abi(), dwtaskid, lpriority, pdonecallback.param().abi(), pdonestate.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFBeginUnregisterWorkQueueWithMMCSS<P1, P2>(dwworkqueueid: u32, pdonecallback: P1, pdonestate: P2) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFBeginUnregisterWorkQueueWithMMCSS(dwworkqueueid : u32, pdonecallback : *mut core::ffi::c_void, pdonestate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFBeginUnregisterWorkQueueWithMMCSS(dwworkqueueid, pdonecallback.param().abi(), pdonestate.param().abi()) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn MFCalculateBitmapImageSize(pbmih: *const super::wingdi::BITMAPINFOHEADER, cbbufsize: u32, pcbimagesize: *mut u32, pbknown: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFCalculateBitmapImageSize(pbmih : *const super::wingdi::BITMAPINFOHEADER, cbbufsize : u32, pcbimagesize : *mut u32, pbknown : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { MFCalculateBitmapImageSize(pbmih, cbbufsize, pcbimagesize as _, pbknown.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MFCalculateImageSize(guidsubtype: *const windows_core::GUID, unwidth: u32, unheight: u32) -> windows_core::Result<u32> {
    windows_core::link!("mfplat.dll" "system" fn MFCalculateImageSize(guidsubtype : *const windows_core::GUID, unwidth : u32, unheight : u32, pcbimagesize : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCalculateImageSize(guidsubtype, unwidth, unheight, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFCancelCreateFile<P0>(pcancelcookie: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCancelCreateFile(pcancelcookie : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFCancelCreateFile(pcancelcookie.param().abi()) }
}
#[inline]
pub unsafe fn MFCancelWorkItem(key: MFWORKITEM_KEY) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFCancelWorkItem(key : MFWORKITEM_KEY) -> windows_core::HRESULT);
    unsafe { MFCancelWorkItem(key) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCombineSamples<P0, P1>(psample: P0, psampletoadd: P1, dwmaxmergeddurationinms: u32) -> windows_core::Result<windows_core::BOOL>
where
    P0: windows_core::Param<super::mfobjects::IMFSample>,
    P1: windows_core::Param<super::mfobjects::IMFSample>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCombineSamples(psample : *mut core::ffi::c_void, psampletoadd : *mut core::ffi::c_void, dwmaxmergeddurationinms : u32, pmerged : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCombineSamples(psample.param().abi(), psampletoadd.param().abi(), dwmaxmergeddurationinms, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCompareFullToPartialMediaType<P0, P1>(pmftypefull: P0, pmftypepartial: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
    P1: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCompareFullToPartialMediaType(pmftypefull : *mut core::ffi::c_void, pmftypepartial : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { MFCompareFullToPartialMediaType(pmftypefull.param().abi(), pmftypepartial.param().abi()) }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFConvertColorInfoFromDXVA(ptoformat: *mut super::mfobjects::MFVIDEOFORMAT, dwfromdxva: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFConvertColorInfoFromDXVA(ptoformat : *mut super::mfobjects::MFVIDEOFORMAT, dwfromdxva : u32) -> windows_core::HRESULT);
    unsafe { MFConvertColorInfoFromDXVA(ptoformat as _, dwfromdxva) }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFConvertColorInfoToDXVA(pdwtodxva: *mut u32, pfromformat: *const super::mfobjects::MFVIDEOFORMAT) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFConvertColorInfoToDXVA(pdwtodxva : *mut u32, pfromformat : *const super::mfobjects::MFVIDEOFORMAT) -> windows_core::HRESULT);
    unsafe { MFConvertColorInfoToDXVA(pdwtodxva as _, pfromformat) }
}
#[inline]
pub unsafe fn MFConvertFromFP16Array(pdest: *mut f32, psrc: *const u16, dwcount: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFConvertFromFP16Array(pdest : *mut f32, psrc : *const u16, dwcount : u32) -> windows_core::HRESULT);
    unsafe { MFConvertFromFP16Array(pdest as _, psrc, dwcount) }
}
#[inline]
pub unsafe fn MFConvertToFP16Array(pdest: *mut u16, psrc: *const f32, dwcount: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFConvertToFP16Array(pdest : *mut u16, psrc : *const f32, dwcount : u32) -> windows_core::HRESULT);
    unsafe { MFConvertToFP16Array(pdest as _, psrc, dwcount) }
}
#[inline]
pub unsafe fn MFCopyImage(pdest: *mut u8, ldeststride: i32, psrc: *const u8, lsrcstride: i32, dwwidthinbytes: u32, dwlines: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFCopyImage(pdest : *mut u8, ldeststride : i32, psrc : *const u8, lsrcstride : i32, dwwidthinbytes : u32, dwlines : u32) -> windows_core::HRESULT);
    unsafe { MFCopyImage(pdest as _, ldeststride, psrc, lsrcstride, dwwidthinbytes, dwlines) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreate2DMediaBuffer(dwwidth: u32, dwheight: u32, dwfourcc: u32, fbottomup: bool) -> windows_core::Result<super::mfobjects::IMFMediaBuffer> {
    windows_core::link!("mfplat.dll" "system" fn MFCreate2DMediaBuffer(dwwidth : u32, dwheight : u32, dwfourcc : u32, fbottomup : windows_core::BOOL, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreate2DMediaBuffer(dwwidth, dwheight, dwfourcc, fbottomup.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "strmif"))]
#[inline]
pub unsafe fn MFCreateAMMediaTypeFromMFMediaType<P0>(pmftype: P0, guidformatblocktype: windows_core::GUID, ppamtype: *mut *mut super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateAMMediaTypeFromMFMediaType(pmftype : *mut core::ffi::c_void, guidformatblocktype : windows_core::GUID, ppamtype : *mut *mut super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MFCreateAMMediaTypeFromMFMediaType(pmftype.param().abi(), guidformatblocktype, ppamtype as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateAlignedMemoryBuffer(cbmaxlength: u32, cbaligment: u32) -> windows_core::Result<super::mfobjects::IMFMediaBuffer> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateAlignedMemoryBuffer(cbmaxlength : u32, cbaligment : u32, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateAlignedMemoryBuffer(cbmaxlength, cbaligment, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateAsyncResult<P0, P1, P2>(punkobject: P0, pcallback: P1, punkstate: P2) -> windows_core::Result<super::mfobjects::IMFAsyncResult>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateAsyncResult(punkobject : *mut core::ffi::c_void, pcallback : *mut core::ffi::c_void, punkstate : *mut core::ffi::c_void, ppasyncresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateAsyncResult(punkobject.param().abi(), pcallback.param().abi(), punkstate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateAttributes(ppmfattributes: *mut Option<super::mfobjects::IMFAttributes>, cinitialsize: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFCreateAttributes(ppmfattributes : *mut *mut core::ffi::c_void, cinitialsize : u32) -> windows_core::HRESULT);
    unsafe { MFCreateAttributes(core::mem::transmute(ppmfattributes), cinitialsize) }
}
#[cfg(all(feature = "mfobjects", feature = "mmeapi"))]
#[inline]
pub unsafe fn MFCreateAudioMediaType(paudioformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::Result<super::mfobjects::IMFAudioMediaType> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateAudioMediaType(paudioformat : *const super::mmeapi::WAVEFORMATEX, ppiaudiomediatype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateAudioMediaType(paudioformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateCollection() -> windows_core::Result<super::mfobjects::IMFCollection> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateCollection(ppimfcollection : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateCollection(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateDXGICrossAdapterBuffer<P1, P2>(riid: *const windows_core::GUID, punkdevice: P1, pmediatype: P2, usubresource: u32) -> windows_core::Result<super::mfobjects::IMFMediaBuffer>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    P2: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateDXGICrossAdapterBuffer(riid : *const windows_core::GUID, punkdevice : *mut core::ffi::c_void, pmediatype : *mut core::ffi::c_void, usubresource : u32, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateDXGICrossAdapterBuffer(riid, punkdevice.param().abi(), pmediatype.param().abi(), usubresource, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateDXGIDeviceManager(resettoken: *mut u32, ppdevicemanager: *mut Option<super::mfobjects::IMFDXGIDeviceManager>) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFCreateDXGIDeviceManager(resettoken : *mut u32, ppdevicemanager : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFCreateDXGIDeviceManager(resettoken as _, core::mem::transmute(ppdevicemanager)) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateDXGISurfaceBuffer<P1>(riid: *const windows_core::GUID, punksurface: P1, usubresourceindex: u32, fbottomupwhenlinear: bool) -> windows_core::Result<super::mfobjects::IMFMediaBuffer>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateDXGISurfaceBuffer(riid : *const windows_core::GUID, punksurface : *mut core::ffi::c_void, usubresourceindex : u32, fbottomupwhenlinear : windows_core::BOOL, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateDXGISurfaceBuffer(riid, punksurface.param().abi(), usubresourceindex, fbottomupwhenlinear.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateDXSurfaceBuffer<P1>(riid: *const windows_core::GUID, punksurface: P1, fbottomupwhenlinear: bool) -> windows_core::Result<super::mfobjects::IMFMediaBuffer>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateDXSurfaceBuffer(riid : *const windows_core::GUID, punksurface : *mut core::ffi::c_void, fbottomupwhenlinear : windows_core::BOOL, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateDXSurfaceBuffer(riid, punksurface.param().abi(), fbottomupwhenlinear.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateEventQueue() -> windows_core::Result<super::mfobjects::IMFMediaEventQueue> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateEventQueue(ppmediaeventqueue : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateEventQueue(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateFile<P3>(accessmode: super::mfobjects::MF_FILE_ACCESSMODE, openmode: super::mfobjects::MF_FILE_OPENMODE, fflags: super::mfobjects::MF_FILE_FLAGS, pwszfileurl: P3) -> windows_core::Result<super::mfobjects::IMFByteStream>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateFile(accessmode : super::mfobjects::MF_FILE_ACCESSMODE, openmode : super::mfobjects::MF_FILE_OPENMODE, fflags : super::mfobjects::MF_FILE_FLAGS, pwszfileurl : windows_core::PCWSTR, ppibytestream : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateFile(accessmode, openmode, fflags, pwszfileurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mediaobj", feature = "mfobjects"))]
#[inline]
pub unsafe fn MFCreateLegacyMediaBufferOnMFMediaBuffer<P0, P1>(psample: P0, pmfmediabuffer: P1, cboffset: u32) -> windows_core::Result<super::mediaobj::IMediaBuffer>
where
    P0: windows_core::Param<super::mfobjects::IMFSample>,
    P1: windows_core::Param<super::mfobjects::IMFMediaBuffer>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateLegacyMediaBufferOnMFMediaBuffer(psample : *mut core::ffi::c_void, pmfmediabuffer : *mut core::ffi::c_void, cboffset : u32, ppmediabuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateLegacyMediaBufferOnMFMediaBuffer(psample.param().abi(), pmfmediabuffer.param().abi(), cboffset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMFByteStreamWrapper<P0>(pstream: P0) -> windows_core::Result<super::mfobjects::IMFByteStream>
where
    P0: windows_core::Param<super::mfobjects::IMFByteStream>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMFByteStreamWrapper(pstream : *mut core::ffi::c_void, ppstreamwrapper : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMFByteStreamWrapper(pstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFCreateMFVideoFormatFromMFMediaType<P0>(pmftype: P0, ppmfvf: *mut *mut super::mfobjects::MFVIDEOFORMAT, pcbsize: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMFVideoFormatFromMFMediaType(pmftype : *mut core::ffi::c_void, ppmfvf : *mut *mut super::mfobjects::MFVIDEOFORMAT, pcbsize : *mut u32) -> windows_core::HRESULT);
    unsafe { MFCreateMFVideoFormatFromMFMediaType(pmftype.param().abi(), ppmfvf as _, pcbsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMediaBufferFromMediaType<P0>(pmediatype: P0, llduration: i64, dwminlength: u32, dwminalignment: u32) -> windows_core::Result<super::mfobjects::IMFMediaBuffer>
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaBufferFromMediaType(pmediatype : *mut core::ffi::c_void, llduration : i64, dwminlength : u32, dwminalignment : u32, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMediaBufferFromMediaType(pmediatype.param().abi(), llduration, dwminlength, dwminalignment, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMediaBufferWrapper<P0>(pbuffer: P0, cboffset: u32, dwlength: u32) -> windows_core::Result<super::mfobjects::IMFMediaBuffer>
where
    P0: windows_core::Param<super::mfobjects::IMFMediaBuffer>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaBufferWrapper(pbuffer : *mut core::ffi::c_void, cboffset : u32, dwlength : u32, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMediaBufferWrapper(pbuffer.param().abi(), cboffset, dwlength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MFCreateMediaEvent(met: super::mfobjects::MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, pvvalue: Option<*const super::propidlbase::PROPVARIANT>) -> windows_core::Result<super::mfobjects::IMFMediaEvent> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaEvent(met : super::mfobjects::MediaEventType, guidextendedtype : *const windows_core::GUID, hrstatus : windows_core::HRESULT, pvvalue : *const super::propidlbase::PROPVARIANT, ppevent : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMediaEvent(met, guidextendedtype, hrstatus, pvvalue.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateMediaExtensionActivate<P0, P1, T>(szactivatableclassid: P0, pconfiguration: P1) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaExtensionActivate(szactivatableclassid : windows_core::PCWSTR, pconfiguration : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreateMediaExtensionActivate(szactivatableclassid.param().abi(), pconfiguration.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMediaType() -> windows_core::Result<super::mfobjects::IMFMediaType> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaType(ppmftype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMediaType(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMediaTypeFromRepresentation(guidrepresentation: windows_core::GUID, pvrepresentation: *const core::ffi::c_void) -> windows_core::Result<super::mfobjects::IMFMediaType> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaTypeFromRepresentation(guidrepresentation : windows_core::GUID, pvrepresentation : *const core::ffi::c_void, ppimediatype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMediaTypeFromRepresentation(guidrepresentation, pvrepresentation, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMemoryBuffer(cbmaxlength: u32) -> windows_core::Result<super::mfobjects::IMFMediaBuffer> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateMemoryBuffer(cbmaxlength : u32, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMemoryBuffer(cbmaxlength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMuxStreamAttributes<P0>(pattributestomux: P0) -> windows_core::Result<super::mfobjects::IMFAttributes>
where
    P0: windows_core::Param<super::mfobjects::IMFCollection>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMuxStreamAttributes(pattributestomux : *mut core::ffi::c_void, ppmuxattribs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMuxStreamAttributes(pattributestomux.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMuxStreamMediaType<P0>(pmediatypestomux: P0) -> windows_core::Result<super::mfobjects::IMFMediaType>
where
    P0: windows_core::Param<super::mfobjects::IMFCollection>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMuxStreamMediaType(pmediatypestomux : *mut core::ffi::c_void, ppmuxmediatype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMuxStreamMediaType(pmediatypestomux.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMuxStreamSample<P0>(psamplestomux: P0) -> windows_core::Result<super::mfobjects::IMFSample>
where
    P0: windows_core::Param<super::mfobjects::IMFCollection>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMuxStreamSample(psamplestomux : *mut core::ffi::c_void, ppmuxsample : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMuxStreamSample(psamplestomux.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSample() -> windows_core::Result<super::mfobjects::IMFSample> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateSample(ppimfsample : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSample(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateTempFile(accessmode: super::mfobjects::MF_FILE_ACCESSMODE, openmode: super::mfobjects::MF_FILE_OPENMODE, fflags: super::mfobjects::MF_FILE_FLAGS) -> windows_core::Result<super::mfobjects::IMFByteStream> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateTempFile(accessmode : super::mfobjects::MF_FILE_ACCESSMODE, openmode : super::mfobjects::MF_FILE_OPENMODE, fflags : super::mfobjects::MF_FILE_FLAGS, ppibytestream : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTempFile(accessmode, openmode, fflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFCreateVideoMediaType(pvideoformat: *const super::mfobjects::MFVIDEOFORMAT) -> windows_core::Result<super::mfobjects::IMFVideoMediaType> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateVideoMediaType(pvideoformat : *const super::mfobjects::MFVIDEOFORMAT, ppivideomediatype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateVideoMediaType(pvideoformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "wingdi"))]
#[inline]
pub unsafe fn MFCreateVideoMediaTypeFromBitMapInfoHeader(pbmihbitmapinfoheader: *const super::wingdi::BITMAPINFOHEADER, dwpixelaspectratiox: u32, dwpixelaspectratioy: u32, interlacemode: super::mfobjects::MFVideoInterlaceMode, videoflags: u64, qwframespersecondnumerator: u64, qwframesperseconddenominator: u64, dwmaxbitrate: u32) -> windows_core::Result<super::mfobjects::IMFVideoMediaType> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateVideoMediaTypeFromBitMapInfoHeader(pbmihbitmapinfoheader : *const super::wingdi::BITMAPINFOHEADER, dwpixelaspectratiox : u32, dwpixelaspectratioy : u32, interlacemode : super::mfobjects::MFVideoInterlaceMode, videoflags : u64, qwframespersecondnumerator : u64, qwframesperseconddenominator : u64, dwmaxbitrate : u32, ppivideomediatype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateVideoMediaTypeFromBitMapInfoHeader(pbmihbitmapinfoheader, dwpixelaspectratiox, dwpixelaspectratioy, interlacemode, videoflags, qwframespersecondnumerator, qwframesperseconddenominator, dwmaxbitrate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "wingdi"))]
#[inline]
pub unsafe fn MFCreateVideoMediaTypeFromBitMapInfoHeaderEx(pbmihbitmapinfoheader: *const super::wingdi::BITMAPINFOHEADER, cbbitmapinfoheader: u32, dwpixelaspectratiox: u32, dwpixelaspectratioy: u32, interlacemode: super::mfobjects::MFVideoInterlaceMode, videoflags: u64, dwframespersecondnumerator: u32, dwframesperseconddenominator: u32, dwmaxbitrate: u32) -> windows_core::Result<super::mfobjects::IMFVideoMediaType> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateVideoMediaTypeFromBitMapInfoHeaderEx(pbmihbitmapinfoheader : *const super::wingdi::BITMAPINFOHEADER, cbbitmapinfoheader : u32, dwpixelaspectratiox : u32, dwpixelaspectratioy : u32, interlacemode : super::mfobjects::MFVideoInterlaceMode, videoflags : u64, dwframespersecondnumerator : u32, dwframesperseconddenominator : u32, dwmaxbitrate : u32, ppivideomediatype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateVideoMediaTypeFromBitMapInfoHeaderEx(pbmihbitmapinfoheader, cbbitmapinfoheader, dwpixelaspectratiox, dwpixelaspectratioy, interlacemode, videoflags, dwframespersecondnumerator, dwframesperseconddenominator, dwmaxbitrate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateVideoMediaTypeFromSubtype(pamsubtype: *const windows_core::GUID) -> windows_core::Result<super::mfobjects::IMFVideoMediaType> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateVideoMediaTypeFromSubtype(pamsubtype : *const windows_core::GUID, ppivideomediatype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateVideoMediaTypeFromSubtype(pamsubtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateVideoSampleAllocatorEx<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateVideoSampleAllocatorEx(riid : *const windows_core::GUID, ppsampleallocator : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreateVideoSampleAllocatorEx(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateWICBitmapBuffer<P1>(riid: *const windows_core::GUID, punksurface: P1) -> windows_core::Result<super::mfobjects::IMFMediaBuffer>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateWICBitmapBuffer(riid : *const windows_core::GUID, punksurface : *mut core::ffi::c_void, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateWICBitmapBuffer(riid, punksurface.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "mmeapi"))]
#[inline]
pub unsafe fn MFCreateWaveFormatExFromMFMediaType<P0>(pmftype: P0, ppwf: *mut *mut super::mmeapi::WAVEFORMATEX, pcbsize: Option<*mut u32>, flags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateWaveFormatExFromMFMediaType(pmftype : *mut core::ffi::c_void, ppwf : *mut *mut super::mmeapi::WAVEFORMATEX, pcbsize : *mut u32, flags : u32) -> windows_core::HRESULT);
    unsafe { MFCreateWaveFormatExFromMFMediaType(pmftype.param().abi(), ppwf as _, pcbsize.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFEndCreateFile<P0>(presult: P0) -> windows_core::Result<super::mfobjects::IMFByteStream>
where
    P0: windows_core::Param<super::mfobjects::IMFAsyncResult>,
{
    windows_core::link!("mfplat.dll" "system" fn MFEndCreateFile(presult : *mut core::ffi::c_void, ppfile : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFEndCreateFile(presult.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFEndRegisterWorkQueueWithMMCSS<P0>(presult: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::mfobjects::IMFAsyncResult>,
{
    windows_core::link!("mfplat.dll" "system" fn MFEndRegisterWorkQueueWithMMCSS(presult : *mut core::ffi::c_void, pdwtaskid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFEndRegisterWorkQueueWithMMCSS(presult.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFEndUnregisterWorkQueueWithMMCSS<P0>(presult: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFAsyncResult>,
{
    windows_core::link!("mfplat.dll" "system" fn MFEndUnregisterWorkQueueWithMMCSS(presult : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFEndUnregisterWorkQueueWithMMCSS(presult.param().abi()) }
}
#[inline]
pub unsafe fn MFFrameRateToAverageTimePerFrame(unnumerator: u32, undenominator: u32) -> windows_core::Result<u64> {
    windows_core::link!("mfplat.dll" "system" fn MFFrameRateToAverageTimePerFrame(unnumerator : u32, undenominator : u32, punaveragetimeperframe : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFFrameRateToAverageTimePerFrame(unnumerator, undenominator, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFGetAttributesAsBlob<P0>(pattributes: P0, pbuf: &mut [u8]) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfplat.dll" "system" fn MFGetAttributesAsBlob(pattributes : *mut core::ffi::c_void, pbuf : *mut u8, cbbufsize : u32) -> windows_core::HRESULT);
    unsafe { MFGetAttributesAsBlob(pattributes.param().abi(), pbuf.as_mut_ptr(), pbuf.len().try_into().unwrap()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFGetAttributesAsBlobSize<P0>(pattributes: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfplat.dll" "system" fn MFGetAttributesAsBlobSize(pattributes : *mut core::ffi::c_void, pcbbufsize : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetAttributesAsBlobSize(pattributes.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFGetContentProtectionSystemCLSID(guidprotectionsystemid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("mfplat.dll" "system" fn MFGetContentProtectionSystemCLSID(guidprotectionsystemid : *const windows_core::GUID, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetContentProtectionSystemCLSID(guidprotectionsystemid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFGetDXGIDeviceManageMode<P0>(pdevicemanager: P0) -> windows_core::Result<super::mfobjects::MF_DXGI_DEVICE_MANAGER_MODE>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFGetDXGIDeviceManageMode(pdevicemanager : *mut core::ffi::c_void, mode : *mut super::mfobjects::MF_DXGI_DEVICE_MANAGER_MODE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetDXGIDeviceManageMode(pdevicemanager.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFGetMFTMerit(pmft: &Option<windows_core::IUnknown>, verifier: &[u8], merit: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFGetMFTMerit(pmft : *mut core::ffi::c_void, cbverifier : u32, verifier : *const u8, merit : *mut u32) -> windows_core::HRESULT);
    unsafe { MFGetMFTMerit(core::mem::transmute_copy(pmft), verifier.len().try_into().unwrap(), verifier.as_ptr(), merit as _) }
}
#[inline]
pub unsafe fn MFGetPlaneSize(format: u32, dwwidth: u32, dwheight: u32) -> windows_core::Result<u32> {
    windows_core::link!("mfplat.dll" "system" fn MFGetPlaneSize(format : u32, dwwidth : u32, dwheight : u32, pdwplanesize : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetPlaneSize(format, dwwidth, dwheight, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFGetPluginControl() -> windows_core::Result<super::mfobjects::IMFPluginControl> {
    windows_core::link!("mfplat.dll" "system" fn MFGetPluginControl(ppplugincontrol : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetPluginControl(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFGetStrideForBitmapInfoHeader(format: u32, dwwidth: u32) -> windows_core::Result<i32> {
    windows_core::link!("mfplat.dll" "system" fn MFGetStrideForBitmapInfoHeader(format : u32, dwwidth : u32, pstride : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetStrideForBitmapInfoHeader(format, dwwidth, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFGetTimerPeriodicity() -> windows_core::Result<u32> {
    windows_core::link!("mfplat.dll" "system" fn MFGetTimerPeriodicity(periodicity : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetTimerPeriodicity(&mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFGetUncompressedVideoFormat(pvideoformat: *const super::mfobjects::MFVIDEOFORMAT) -> u32 {
    windows_core::link!("mfplat.dll" "system" fn MFGetUncompressedVideoFormat(pvideoformat : *const super::mfobjects::MFVIDEOFORMAT) -> u32);
    unsafe { MFGetUncompressedVideoFormat(pvideoformat) }
}
#[inline]
pub unsafe fn MFGetWorkQueueMMCSSClass(dwworkqueueid: u32, pwszclass: Option<windows_core::PWSTR>, pcchclass: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFGetWorkQueueMMCSSClass(dwworkqueueid : u32, pwszclass : windows_core::PWSTR, pcchclass : *mut u32) -> windows_core::HRESULT);
    unsafe { MFGetWorkQueueMMCSSClass(dwworkqueueid, pwszclass.unwrap_or(core::mem::zeroed()) as _, pcchclass as _) }
}
#[inline]
pub unsafe fn MFGetWorkQueueMMCSSPriority(dwworkqueueid: u32) -> windows_core::Result<i32> {
    windows_core::link!("mfplat.dll" "system" fn MFGetWorkQueueMMCSSPriority(dwworkqueueid : u32, lpriority : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetWorkQueueMMCSSPriority(dwworkqueueid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFGetWorkQueueMMCSSTaskId(dwworkqueueid: u32) -> windows_core::Result<u32> {
    windows_core::link!("mfplat.dll" "system" fn MFGetWorkQueueMMCSSTaskId(dwworkqueueid : u32, pdwtaskid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetWorkQueueMMCSSTaskId(dwworkqueueid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFHeapAlloc(nsize: usize, dwflags: u32, pszfile: Option<*const i8>, line: i32, eat: EAllocationType) -> *mut core::ffi::c_void {
    windows_core::link!("mfplat.dll" "system" fn MFHeapAlloc(nsize : usize, dwflags : u32, pszfile : *const i8, line : i32, eat : EAllocationType) -> *mut core::ffi::c_void);
    unsafe { MFHeapAlloc(nsize, dwflags, pszfile.unwrap_or(core::mem::zeroed()) as _, line, eat) }
}
#[inline]
pub unsafe fn MFHeapFree(pv: *mut core::ffi::c_void) {
    windows_core::link!("mfplat.dll" "system" fn MFHeapFree(pv : *mut core::ffi::c_void));
    unsafe { MFHeapFree(pv as _) }
}
#[cfg(all(feature = "mfobjects", feature = "strmif"))]
#[inline]
pub unsafe fn MFInitAMMediaTypeFromMFMediaType<P0>(pmftype: P0, guidformatblocktype: windows_core::GUID, pamtype: *mut super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitAMMediaTypeFromMFMediaType(pmftype : *mut core::ffi::c_void, guidformatblocktype : windows_core::GUID, pamtype : *mut super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MFInitAMMediaTypeFromMFMediaType(pmftype.param().abi(), guidformatblocktype, pamtype) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFInitAttributesFromBlob<P0>(pattributes: P0, pbuf: &[u8]) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitAttributesFromBlob(pattributes : *mut core::ffi::c_void, pbuf : *const u8, cbbufsize : u32) -> windows_core::HRESULT);
    unsafe { MFInitAttributesFromBlob(pattributes.param().abi(), pbuf.as_ptr(), pbuf.len().try_into().unwrap()) }
}
#[cfg(all(feature = "mfobjects", feature = "strmif"))]
#[inline]
pub unsafe fn MFInitMediaTypeFromAMMediaType<P0>(pmftype: P0, pamtype: *const super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitMediaTypeFromAMMediaType(pmftype : *mut core::ffi::c_void, pamtype : *const super::strmif::AM_MEDIA_TYPE) -> windows_core::HRESULT);
    unsafe { MFInitMediaTypeFromAMMediaType(pmftype.param().abi(), pamtype) }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFInitMediaTypeFromMFVideoFormat<P0>(pmftype: P0, pmfvf: *const super::mfobjects::MFVIDEOFORMAT, cbbufsize: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitMediaTypeFromMFVideoFormat(pmftype : *mut core::ffi::c_void, pmfvf : *const super::mfobjects::MFVIDEOFORMAT, cbbufsize : u32) -> windows_core::HRESULT);
    unsafe { MFInitMediaTypeFromMFVideoFormat(pmftype.param().abi(), pmfvf, cbbufsize) }
}
#[cfg(all(feature = "amvideo", feature = "ksmedia", feature = "mfobjects", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn MFInitMediaTypeFromMPEG1VideoInfo<P0>(pmftype: P0, pmp1vi: *const super::amvideo::MPEG1VIDEOINFO, cbbufsize: u32, psubtype: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitMediaTypeFromMPEG1VideoInfo(pmftype : *mut core::ffi::c_void, pmp1vi : *const super::amvideo::MPEG1VIDEOINFO, cbbufsize : u32, psubtype : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { MFInitMediaTypeFromMPEG1VideoInfo(pmftype.param().abi(), pmp1vi, cbbufsize, psubtype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFInitMediaTypeFromMPEG2VideoInfo<P0>(pmftype: P0, pmp2vi: *const MPEG2VIDEOINFO, cbbufsize: u32, psubtype: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitMediaTypeFromMPEG2VideoInfo(pmftype : *mut core::ffi::c_void, pmp2vi : *const MPEG2VIDEOINFO, cbbufsize : u32, psubtype : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { MFInitMediaTypeFromMPEG2VideoInfo(pmftype.param().abi(), pmp2vi, cbbufsize, psubtype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "amvideo", feature = "ksmedia", feature = "mfobjects", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn MFInitMediaTypeFromVideoInfoHeader<P0>(pmftype: P0, pvih: *const super::amvideo::VIDEOINFOHEADER, cbbufsize: u32, psubtype: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitMediaTypeFromVideoInfoHeader(pmftype : *mut core::ffi::c_void, pvih : *const super::amvideo::VIDEOINFOHEADER, cbbufsize : u32, psubtype : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { MFInitMediaTypeFromVideoInfoHeader(pmftype.param().abi(), pvih, cbbufsize, psubtype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFInitMediaTypeFromVideoInfoHeader2<P0>(pmftype: P0, pvih2: *const VIDEOINFOHEADER2, cbbufsize: u32, psubtype: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitMediaTypeFromVideoInfoHeader2(pmftype : *mut core::ffi::c_void, pvih2 : *const VIDEOINFOHEADER2, cbbufsize : u32, psubtype : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { MFInitMediaTypeFromVideoInfoHeader2(pmftype.param().abi(), pvih2, cbbufsize, psubtype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mfobjects", feature = "mmeapi"))]
#[inline]
pub unsafe fn MFInitMediaTypeFromWaveFormatEx<P0>(pmftype: P0, pwaveformat: *const super::mmeapi::WAVEFORMATEX, cbbufsize: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInitMediaTypeFromWaveFormatEx(pmftype : *mut core::ffi::c_void, pwaveformat : *const super::mmeapi::WAVEFORMATEX, cbbufsize : u32) -> windows_core::HRESULT);
    unsafe { MFInitMediaTypeFromWaveFormatEx(pmftype.param().abi(), pwaveformat, cbbufsize) }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFInitVideoFormat(pvideoformat: *const super::mfobjects::MFVIDEOFORMAT, r#type: super::mfobjects::MFStandardVideoFormat) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFInitVideoFormat(pvideoformat : *const super::mfobjects::MFVIDEOFORMAT, r#type : super::mfobjects::MFStandardVideoFormat) -> windows_core::HRESULT);
    unsafe { MFInitVideoFormat(pvideoformat, r#type) }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFInitVideoFormat_RGB(pvideoformat: *const super::mfobjects::MFVIDEOFORMAT, dwwidth: u32, dwheight: u32, d3dfmt: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFInitVideoFormat_RGB(pvideoformat : *const super::mfobjects::MFVIDEOFORMAT, dwwidth : u32, dwheight : u32, d3dfmt : u32) -> windows_core::HRESULT);
    unsafe { MFInitVideoFormat_RGB(pvideoformat, dwwidth, dwheight, d3dfmt) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFInvokeCallback<P0>(pasyncresult: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFAsyncResult>,
{
    windows_core::link!("mfplat.dll" "system" fn MFInvokeCallback(pasyncresult : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFInvokeCallback(pasyncresult.param().abi()) }
}
#[inline]
pub unsafe fn MFIsFormatYUV(format: u32) -> windows_core::BOOL {
    windows_core::link!("evr.dll" "system" fn MFIsFormatYUV(format : u32) -> windows_core::BOOL);
    unsafe { MFIsFormatYUV(format) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFLockDXGIDeviceManager(presettoken: Option<*mut u32>, ppmanager: *mut Option<super::mfobjects::IMFDXGIDeviceManager>) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFLockDXGIDeviceManager(presettoken : *mut u32, ppmanager : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFLockDXGIDeviceManager(presettoken.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppmanager)) }
}
#[inline]
pub unsafe fn MFLockPlatform() -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFLockPlatform() -> windows_core::HRESULT);
    unsafe { MFLockPlatform() }
}
#[inline]
pub unsafe fn MFLockSharedWorkQueue<P0>(wszclass: P0, basepriority: i32, pdwtaskid: *mut u32, pid: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfplat.dll" "system" fn MFLockSharedWorkQueue(wszclass : windows_core::PCWSTR, basepriority : i32, pdwtaskid : *mut u32, pid : *mut u32) -> windows_core::HRESULT);
    unsafe { MFLockSharedWorkQueue(wszclass.param().abi(), basepriority, pdwtaskid as _, pid as _) }
}
#[inline]
pub unsafe fn MFLockWorkQueue(dwworkqueue: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFLockWorkQueue(dwworkqueue : u32) -> windows_core::HRESULT);
    unsafe { MFLockWorkQueue(dwworkqueue) }
}
#[cfg(feature = "dxgi")]
#[inline]
pub unsafe fn MFMapDX9FormatToDXGIFormat(dx9: u32) -> super::dxgi::DXGI_FORMAT {
    windows_core::link!("mfplat.dll" "system" fn MFMapDX9FormatToDXGIFormat(dx9 : u32) -> super::dxgi::DXGI_FORMAT);
    unsafe { MFMapDX9FormatToDXGIFormat(dx9) }
}
#[cfg(feature = "dxgi")]
#[inline]
pub unsafe fn MFMapDXGIFormatToDX9Format(dx11: super::dxgi::DXGI_FORMAT) -> u32 {
    windows_core::link!("mfplat.dll" "system" fn MFMapDXGIFormatToDX9Format(dx11 : super::dxgi::DXGI_FORMAT) -> u32);
    unsafe { MFMapDXGIFormatToDX9Format(dx11) }
}
#[cfg(all(feature = "mfobjects", feature = "winnt"))]
#[inline]
pub unsafe fn MFPutWaitingWorkItem<P2>(hevent: super::winnt::HANDLE, priority: i32, presult: P2, pkey: Option<*mut MFWORKITEM_KEY>) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::mfobjects::IMFAsyncResult>,
{
    windows_core::link!("mfplat.dll" "system" fn MFPutWaitingWorkItem(hevent : super::winnt::HANDLE, priority : i32, presult : *mut core::ffi::c_void, pkey : *mut MFWORKITEM_KEY) -> windows_core::HRESULT);
    unsafe { MFPutWaitingWorkItem(hevent, priority, presult.param().abi(), pkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFPutWorkItem<P1, P2>(dwqueue: u32, pcallback: P1, pstate: P2) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFPutWorkItem(dwqueue : u32, pcallback : *mut core::ffi::c_void, pstate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFPutWorkItem(dwqueue, pcallback.param().abi(), pstate.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFPutWorkItem2<P2, P3>(dwqueue: u32, priority: i32, pcallback: P2, pstate: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    P3: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFPutWorkItem2(dwqueue : u32, priority : i32, pcallback : *mut core::ffi::c_void, pstate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFPutWorkItem2(dwqueue, priority, pcallback.param().abi(), pstate.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFPutWorkItemEx<P1>(dwqueue: u32, presult: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::mfobjects::IMFAsyncResult>,
{
    windows_core::link!("mfplat.dll" "system" fn MFPutWorkItemEx(dwqueue : u32, presult : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFPutWorkItemEx(dwqueue, presult.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFPutWorkItemEx2<P2>(dwqueue: u32, priority: i32, presult: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::mfobjects::IMFAsyncResult>,
{
    windows_core::link!("mfplat.dll" "system" fn MFPutWorkItemEx2(dwqueue : u32, priority : i32, presult : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFPutWorkItemEx2(dwqueue, priority, presult.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFRegisterLocalByteStreamHandler<P0, P1, P2>(szfileextension: P0, szmimetype: P1, pactivate: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::mfobjects::IMFActivate>,
{
    windows_core::link!("mfplat.dll" "system" fn MFRegisterLocalByteStreamHandler(szfileextension : windows_core::PCWSTR, szmimetype : windows_core::PCWSTR, pactivate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFRegisterLocalByteStreamHandler(szfileextension.param().abi(), szmimetype.param().abi(), pactivate.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFRegisterLocalSchemeHandler<P0, P1>(szscheme: P0, pactivate: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::mfobjects::IMFActivate>,
{
    windows_core::link!("mfplat.dll" "system" fn MFRegisterLocalSchemeHandler(szscheme : windows_core::PCWSTR, pactivate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFRegisterLocalSchemeHandler(szscheme.param().abi(), pactivate.param().abi()) }
}
#[inline]
pub unsafe fn MFRegisterPlatformWithMMCSS<P0>(wszclass: P0, pdwtaskid: *mut u32, lpriority: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfplat.dll" "system" fn MFRegisterPlatformWithMMCSS(wszclass : windows_core::PCWSTR, pdwtaskid : *mut u32, lpriority : i32) -> windows_core::HRESULT);
    unsafe { MFRegisterPlatformWithMMCSS(wszclass.param().abi(), pdwtaskid as _, lpriority) }
}
#[inline]
pub unsafe fn MFRemovePeriodicCallback(dwkey: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFRemovePeriodicCallback(dwkey : u32) -> windows_core::HRESULT);
    unsafe { MFRemovePeriodicCallback(dwkey) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFScheduleWorkItem<P0, P1>(pcallback: P0, pstate: P1, timeout: i64, pkey: Option<*mut MFWORKITEM_KEY>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFScheduleWorkItem(pcallback : *mut core::ffi::c_void, pstate : *mut core::ffi::c_void, timeout : i64, pkey : *mut MFWORKITEM_KEY) -> windows_core::HRESULT);
    unsafe { MFScheduleWorkItem(pcallback.param().abi(), pstate.param().abi(), timeout, pkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFScheduleWorkItemEx<P0>(presult: P0, timeout: i64, pkey: Option<*mut MFWORKITEM_KEY>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFAsyncResult>,
{
    windows_core::link!("mfplat.dll" "system" fn MFScheduleWorkItemEx(presult : *mut core::ffi::c_void, timeout : i64, pkey : *mut MFWORKITEM_KEY) -> windows_core::HRESULT);
    unsafe { MFScheduleWorkItemEx(presult.param().abi(), timeout, pkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MFShutdown() -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFShutdown() -> windows_core::HRESULT);
    unsafe { MFShutdown() }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFSplitSample<P0>(psample: P0, poutputsamples: &mut [Option<super::mfobjects::IMFSample>], pdwoutputsamplecount: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::mfobjects::IMFSample>,
{
    windows_core::link!("mfplat.dll" "system" fn MFSplitSample(psample : *mut core::ffi::c_void, poutputsamples : *mut *mut core::ffi::c_void, dwoutputsamplemaxcount : u32, pdwoutputsamplecount : *mut u32) -> windows_core::HRESULT);
    unsafe { MFSplitSample(psample.param().abi(), core::mem::transmute(poutputsamples.as_mut_ptr()), poutputsamples.len().try_into().unwrap(), pdwoutputsamplecount as _) }
}
#[inline]
pub unsafe fn MFStartup(version: u32, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFStartup(version : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { MFStartup(version, dwflags) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFTEnum<P4>(guidcategory: windows_core::GUID, flags: u32, pinputtype: Option<*const super::mfobjects::MFT_REGISTER_TYPE_INFO>, poutputtype: Option<*const super::mfobjects::MFT_REGISTER_TYPE_INFO>, pattributes: P4, ppclsidmft: *mut *mut windows_core::GUID, pcmfts: *mut u32) -> windows_core::HRESULT
where
    P4: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfplat.dll" "system" fn MFTEnum(guidcategory : windows_core::GUID, flags : u32, pinputtype : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, poutputtype : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, pattributes : *mut core::ffi::c_void, ppclsidmft : *mut *mut windows_core::GUID, pcmfts : *mut u32) -> windows_core::HRESULT);
    unsafe { MFTEnum(guidcategory, flags, pinputtype.unwrap_or(core::mem::zeroed()) as _, poutputtype.unwrap_or(core::mem::zeroed()) as _, pattributes.param().abi(), ppclsidmft as _, pcmfts as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFTEnum2<P4>(guidcategory: windows_core::GUID, flags: u32, pinputtype: Option<*const super::mfobjects::MFT_REGISTER_TYPE_INFO>, poutputtype: Option<*const super::mfobjects::MFT_REGISTER_TYPE_INFO>, pattributes: P4, pppmftactivate: *mut *mut Option<super::mfobjects::IMFActivate>, pnummftactivate: *mut u32) -> windows_core::HRESULT
where
    P4: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfplat.dll" "system" fn MFTEnum2(guidcategory : windows_core::GUID, flags : u32, pinputtype : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, poutputtype : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, pattributes : *mut core::ffi::c_void, pppmftactivate : *mut *mut *mut core::ffi::c_void, pnummftactivate : *mut u32) -> windows_core::HRESULT);
    unsafe { MFTEnum2(guidcategory, flags, pinputtype.unwrap_or(core::mem::zeroed()) as _, poutputtype.unwrap_or(core::mem::zeroed()) as _, pattributes.param().abi(), pppmftactivate as _, pnummftactivate as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFTEnumEx(guidcategory: windows_core::GUID, flags: u32, pinputtype: Option<*const super::mfobjects::MFT_REGISTER_TYPE_INFO>, poutputtype: Option<*const super::mfobjects::MFT_REGISTER_TYPE_INFO>, pppmftactivate: *mut *mut Option<super::mfobjects::IMFActivate>, pnummftactivate: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFTEnumEx(guidcategory : windows_core::GUID, flags : u32, pinputtype : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, poutputtype : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, pppmftactivate : *mut *mut *mut core::ffi::c_void, pnummftactivate : *mut u32) -> windows_core::HRESULT);
    unsafe { MFTEnumEx(guidcategory, flags, pinputtype.unwrap_or(core::mem::zeroed()) as _, poutputtype.unwrap_or(core::mem::zeroed()) as _, pppmftactivate as _, pnummftactivate as _) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFTGetInfo(clsidmft: windows_core::GUID, pszname: Option<*mut windows_core::PWSTR>, ppinputtypes: *mut *mut super::mfobjects::MFT_REGISTER_TYPE_INFO, pcinputtypes: Option<*mut u32>, ppoutputtypes: *mut *mut super::mfobjects::MFT_REGISTER_TYPE_INFO, pcoutputtypes: Option<*mut u32>, ppattributes: *mut Option<super::mfobjects::IMFAttributes>) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFTGetInfo(clsidmft : windows_core::GUID, pszname : *mut windows_core::PWSTR, ppinputtypes : *mut *mut super::mfobjects::MFT_REGISTER_TYPE_INFO, pcinputtypes : *mut u32, ppoutputtypes : *mut *mut super::mfobjects::MFT_REGISTER_TYPE_INFO, pcoutputtypes : *mut u32, ppattributes : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFTGetInfo(clsidmft, pszname.unwrap_or(core::mem::zeroed()) as _, ppinputtypes as _, pcinputtypes.unwrap_or(core::mem::zeroed()) as _, ppoutputtypes as _, pcoutputtypes.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppattributes)) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFTRegister<P2, P8>(clsidmft: windows_core::GUID, guidcategory: windows_core::GUID, pszname: P2, flags: u32, pinputtypes: Option<&[super::mfobjects::MFT_REGISTER_TYPE_INFO]>, poutputtypes: Option<&[super::mfobjects::MFT_REGISTER_TYPE_INFO]>, pattributes: P8) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<super::mfobjects::IMFAttributes>,
{
    windows_core::link!("mfplat.dll" "system" fn MFTRegister(clsidmft : windows_core::GUID, guidcategory : windows_core::GUID, pszname : windows_core::PCWSTR, flags : u32, cinputtypes : u32, pinputtypes : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, coutputtypes : u32, poutputtypes : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, pattributes : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFTRegister(clsidmft, guidcategory, pszname.param().abi(), flags, pinputtypes.map_or(0, |slice| slice.len().try_into().unwrap()), pinputtypes.map_or(core::ptr::null(), |slice| slice.as_ptr()), poutputtypes.map_or(0, |slice| slice.len().try_into().unwrap()), poutputtypes.map_or(core::ptr::null(), |slice| slice.as_ptr()), pattributes.param().abi()) }
}
#[cfg(all(feature = "mfobjects", feature = "unknwnbase"))]
#[inline]
pub unsafe fn MFTRegisterLocal<P0, P2>(pclassfactory: P0, guidcategory: *const windows_core::GUID, pszname: P2, flags: u32, pinputtypes: Option<&[super::mfobjects::MFT_REGISTER_TYPE_INFO]>, poutputtypes: Option<&[super::mfobjects::MFT_REGISTER_TYPE_INFO]>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::unknwnbase::IClassFactory>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfplat.dll" "system" fn MFTRegisterLocal(pclassfactory : *mut core::ffi::c_void, guidcategory : *const windows_core::GUID, pszname : windows_core::PCWSTR, flags : u32, cinputtypes : u32, pinputtypes : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, coutputtypes : u32, poutputtypes : *const super::mfobjects::MFT_REGISTER_TYPE_INFO) -> windows_core::HRESULT);
    unsafe { MFTRegisterLocal(pclassfactory.param().abi(), guidcategory, pszname.param().abi(), flags, pinputtypes.map_or(0, |slice| slice.len().try_into().unwrap()), pinputtypes.map_or(core::ptr::null(), |slice| slice.as_ptr()), poutputtypes.map_or(0, |slice| slice.len().try_into().unwrap()), poutputtypes.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFTRegisterLocalByCLSID<P2>(clisdmft: *const windows_core::GUID, guidcategory: *const windows_core::GUID, pszname: P2, flags: u32, pinputtypes: Option<&[super::mfobjects::MFT_REGISTER_TYPE_INFO]>, poutputtypes: Option<&[super::mfobjects::MFT_REGISTER_TYPE_INFO]>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfplat.dll" "system" fn MFTRegisterLocalByCLSID(clisdmft : *const windows_core::GUID, guidcategory : *const windows_core::GUID, pszname : windows_core::PCWSTR, flags : u32, cinputtypes : u32, pinputtypes : *const super::mfobjects::MFT_REGISTER_TYPE_INFO, coutputtypes : u32, poutputtypes : *const super::mfobjects::MFT_REGISTER_TYPE_INFO) -> windows_core::HRESULT);
    unsafe { MFTRegisterLocalByCLSID(clisdmft, guidcategory, pszname.param().abi(), flags, pinputtypes.map_or(0, |slice| slice.len().try_into().unwrap()), pinputtypes.map_or(core::ptr::null(), |slice| slice.as_ptr()), poutputtypes.map_or(0, |slice| slice.len().try_into().unwrap()), poutputtypes.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[inline]
pub unsafe fn MFTUnregister(clsidmft: windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFTUnregister(clsidmft : windows_core::GUID) -> windows_core::HRESULT);
    unsafe { MFTUnregister(clsidmft) }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn MFTUnregisterLocal<P0>(pclassfactory: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::unknwnbase::IClassFactory>,
{
    windows_core::link!("mfplat.dll" "system" fn MFTUnregisterLocal(pclassfactory : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFTUnregisterLocal(pclassfactory.param().abi()) }
}
#[inline]
pub unsafe fn MFTUnregisterLocalByCLSID(clsidmft: windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFTUnregisterLocalByCLSID(clsidmft : windows_core::GUID) -> windows_core::HRESULT);
    unsafe { MFTUnregisterLocalByCLSID(clsidmft) }
}
#[inline]
pub unsafe fn MFUnlockDXGIDeviceManager() -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFUnlockDXGIDeviceManager() -> windows_core::HRESULT);
    unsafe { MFUnlockDXGIDeviceManager() }
}
#[inline]
pub unsafe fn MFUnlockPlatform() -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFUnlockPlatform() -> windows_core::HRESULT);
    unsafe { MFUnlockPlatform() }
}
#[inline]
pub unsafe fn MFUnlockWorkQueue(dwworkqueue: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFUnlockWorkQueue(dwworkqueue : u32) -> windows_core::HRESULT);
    unsafe { MFUnlockWorkQueue(dwworkqueue) }
}
#[inline]
pub unsafe fn MFUnregisterPlatformFromMMCSS() -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFUnregisterPlatformFromMMCSS() -> windows_core::HRESULT);
    unsafe { MFUnregisterPlatformFromMMCSS() }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFUnwrapMediaType<P0>(pwrap: P0) -> windows_core::Result<super::mfobjects::IMFMediaType>
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFUnwrapMediaType(pwrap : *mut core::ffi::c_void, pporig : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFUnwrapMediaType(pwrap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFValidateMediaTypeSize(formattype: windows_core::GUID, pblock: Option<&[u8]>) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFValidateMediaTypeSize(formattype : windows_core::GUID, pblock : *const u8, cbsize : u32) -> windows_core::HRESULT);
    unsafe { MFValidateMediaTypeSize(formattype, pblock.map_or(core::ptr::null(), |slice| slice.as_ptr()), pblock.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFWrapMediaType<P0>(porig: P0, majortype: *const windows_core::GUID, subtype: *const windows_core::GUID) -> windows_core::Result<super::mfobjects::IMFMediaType>
where
    P0: windows_core::Param<super::mfobjects::IMFMediaType>,
{
    windows_core::link!("mfplat.dll" "system" fn MFWrapMediaType(porig : *mut core::ffi::c_void, majortype : *const windows_core::GUID, subtype : *const windows_core::GUID, ppwrap : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFWrapMediaType(porig.param().abi(), majortype, subtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFllMulDiv(a: i64, b: i64, c: i64, d: i64) -> i64 {
    windows_core::link!("mfplat.dll" "system" fn MFllMulDiv(a : i64, b : i64, c : i64, d : i64) -> i64);
    unsafe { MFllMulDiv(a, b, c, d) }
}
pub const AM_MEDIA_TYPE_REPRESENTATION: windows_core::GUID = windows_core::GUID::from_u128(0xe2e42ad2_132c_491e_a268_3c7c2dca181f);
pub const CLSID_MFSourceResolver: windows_core::GUID = windows_core::GUID::from_u128(0x90eab60f_e43a_4188_bcc4_e47fdf04868c);
pub const CODEC_API_QP_MAP_INT16: eAVEncVideoQPMapElementDataType = 1;
pub const CODEC_API_QP_MAP_INT32: eAVEncVideoQPMapElementDataType = 2;
pub const CODEC_API_QP_MAP_INT8: eAVEncVideoQPMapElementDataType = 0;
pub const CODEC_API_QP_MAP_UINT16: eAVEncVideoQPMapElementDataType = -2147483647;
pub const CODEC_API_QP_MAP_UINT32: eAVEncVideoQPMapElementDataType = -2147483646;
pub const CODEC_API_QP_MAP_UINT8: eAVEncVideoQPMapElementDataType = -2147483648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
pub const D3DFMT_A16B16G16R16F: u32 = 113;
pub const D3DFMT_A2B10G10R10: u32 = 31;
pub const D3DFMT_A8R8G8B8: u32 = 21;
pub const D3DFMT_D16: u32 = 80;
pub const D3DFMT_L16: u32 = 81;
pub const D3DFMT_L8: u32 = 50;
pub const D3DFMT_P8: u32 = 41;
pub const D3DFMT_R5G6B5: u32 = 23;
pub const D3DFMT_R8G8B8: u32 = 20;
pub const D3DFMT_X1R5G5B5: u32 = 24;
pub const D3DFMT_X8R8G8B8: u32 = 22;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIRTYRECT_INFO {
    pub FrameNumber: u32,
    pub NumDirtyRects: u32,
    pub DirtyRects: [super::windef::RECT; 1],
}
#[cfg(feature = "windef")]
impl Default for DIRTYRECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DigitalWindowSetting {
    pub OriginX: f64,
    pub OriginY: f64,
    pub WindowSize: f64,
}
pub const DistanceToFocalPlane: MFDepthMeasurement = 0;
pub const DistanceToOpticalCenter: MFDepthMeasurement = 1;
pub type EAllocationType = i32;
pub const FORMAT_MFVideoFormat: windows_core::GUID = windows_core::GUID::from_u128(0xaed4ab2d_7326_43cb_9464_c879cab9c43d);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FaceRectInfo {
    pub Region: super::windef::RECT,
    pub confidenceLevel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::windef::RECT,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HistogramHeader {
    pub Size: u32,
    pub Bins: u32,
    pub FourCC: u32,
    pub ChannelMasks: u32,
    pub Grid: HistogramGrid,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputQPSettings {
    pub minBlockSize: u32,
    pub maxBlockSize: u32,
    pub stepsBlockSize: u32,
    pub dataType: eAVEncVideoQPMapElementDataType,
    pub minValue: i16,
    pub maxValue: i16,
    pub steps: u16,
}
pub const LOCAL_D3DFMT_DEFINES: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MACROBLOCK_DATA {
    pub flags: u32,
    pub motionVectorX: i16,
    pub motionVectorY: i16,
    pub QPDelta: i32,
}
pub const MACROBLOCK_FLAG_DIRTY: u32 = 2;
pub const MACROBLOCK_FLAG_HAS_MOTION_VECTOR: u32 = 16;
pub const MACROBLOCK_FLAG_HAS_QP: u32 = 32;
pub const MACROBLOCK_FLAG_MOTION: u32 = 4;
pub const MACROBLOCK_FLAG_SKIP: u32 = 1;
pub const MACROBLOCK_FLAG_VIDEO: u32 = 8;
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "minwinbase", feature = "winnt"))]
pub struct MFASYNCRESULT {
    pub Base: core::mem::ManuallyDrop<Option<super::mfobjects::IMFAsyncResult>>,
    pub overlapped: super::minwinbase::OVERLAPPED,
    pub pCallback: core::mem::ManuallyDrop<Option<super::mfobjects::IMFAsyncCallback>>,
    pub hrStatusResult: windows_core::HRESULT,
    pub dwBytesTransferred: u32,
    pub hEvent: super::winnt::HANDLE,
}
#[cfg(all(feature = "mfobjects", feature = "minwinbase", feature = "winnt"))]
impl Clone for MFASYNCRESULT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwinbase", feature = "winnt"))]
impl Default for MFASYNCRESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MFASYNC_WORKQUEUE_TYPE = i32;
pub const MFAudioFormat_AAC_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0x419bce76_8b72_400f_adeb_84b57d63484d);
pub const MFAudioFormat_ADTS_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0xda4963a3_14d8_4dcf_92b7_193eb84363db);
pub const MFAudioFormat_Base_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0x3884b5bc_e277_43fd_983d_038aa8d9b605);
pub const MFAudioFormat_DTS_HD: windows_core::GUID = windows_core::GUID::from_u128(0xa2e58eb7_0fa9_48bb_a40c_fa0e156d0645);
pub const MFAudioFormat_DTS_LBR: windows_core::GUID = windows_core::GUID::from_u128(0xc2fe6f0a_4e3c_4df1_9b60_50863091e4b9);
pub const MFAudioFormat_DTS_RAW: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8033_db46_11cf_b4d1_00805f6cbbea);
pub const MFAudioFormat_DTS_UHD: windows_core::GUID = windows_core::GUID::from_u128(0x87020117_ace3_42de_b73e_c656706263f8);
pub const MFAudioFormat_DTS_UHDY: windows_core::GUID = windows_core::GUID::from_u128(0x9b9cca00_91b9_4ccc_883a_8f787ac3cc86);
pub const MFAudioFormat_DTS_XLL: windows_core::GUID = windows_core::GUID::from_u128(0x45b37c1b_8c70_4e59_a7be_a1e42c81c80d);
pub const MFAudioFormat_Dolby_AC3: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802c_db46_11cf_b4d1_00805f6cbbea);
pub const MFAudioFormat_Dolby_AC3_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0x97663a80_8ffb_4445_a6ba_792d908f497f);
pub const MFAudioFormat_Dolby_AC4_V1: windows_core::GUID = windows_core::GUID::from_u128(0x36b7927c_3d87_4a2a_9196_a21ad9e935e6);
pub const MFAudioFormat_Dolby_AC4_V1_ES: windows_core::GUID = windows_core::GUID::from_u128(0x9d8dccc6_d156_4fb8_979c_a85be7d21dfa);
pub const MFAudioFormat_Dolby_AC4_V2: windows_core::GUID = windows_core::GUID::from_u128(0x7998b2a0_17dd_49b6_8dfa_9b278552a2ac);
pub const MFAudioFormat_Dolby_AC4_V2_ES: windows_core::GUID = windows_core::GUID::from_u128(0x7e58c9f9_b070_45f4_8ccd_a99a0417c1ac);
pub const MFAudioFormat_Dolby_DDPlus: windows_core::GUID = windows_core::GUID::from_u128(0xa7fb87af_2d02_42fb_a4d4_05cd93843bdd);
pub const MFAudioFormat_Float_SpatialObjects: windows_core::GUID = windows_core::GUID::from_u128(0xfa39cd94_bc64_4ab1_9b71_dcd09d5a7e7a);
pub const MFAudioFormat_IAMF: windows_core::GUID = windows_core::GUID::from_u128(0x78a8eba0_f446_4851_a55d_5372280e6b0b);
pub const MFAudioFormat_LPCM: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8032_db46_11cf_b4d1_00805f6cbbea);
pub const MFAudioFormat_MPEGH: windows_core::GUID = windows_core::GUID::from_u128(0x7c13c441_ebf8_4931_b678_800b19242236);
pub const MFAudioFormat_MPEGH_ES: windows_core::GUID = windows_core::GUID::from_u128(0x19ee97fe_1be0_4255_a876_e99f53a42ae3);
pub const MFAudioFormat_PCM_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0xa5e7ff01_8411_4acc_a865_5f4941288d80);
pub const MFAudioFormat_Vorbis: windows_core::GUID = windows_core::GUID::from_u128(0x8d2fd10b_5841_4a6b_8905_588fec1aded9);
pub const MFCAPTURE_METADATA_SCANLINE_VERTICAL: u32 = 4;
pub const MFCAPTURE_METADATA_SCAN_BOTTOM_TOP: u32 = 2;
pub const MFCAPTURE_METADATA_SCAN_RIGHT_LEFT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFCameraExtrinsic_CalibratedTransform {
    pub CalibrationId: windows_core::GUID,
    pub Position: MF_FLOAT3,
    pub Orientation: MF_QUATERNION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MFCameraExtrinsics {
    pub TransformCount: u32,
    pub CalibratedTransforms: [MFCameraExtrinsic_CalibratedTransform; 1],
}
impl Default for MFCameraExtrinsics {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFCameraIntrinsic_DistortionModel {
    pub Radial_k1: f32,
    pub Radial_k2: f32,
    pub Radial_k3: f32,
    pub Tangential_p1: f32,
    pub Tangential_p2: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFCameraIntrinsic_PinholeCameraModel {
    pub FocalLength: MF_FLOAT2,
    pub PrincipalPoint: MF_FLOAT2,
}
pub type MFDepthMeasurement = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MFFOLDDOWN_MATRIX {
    pub cbSize: u32,
    pub cSrcChannels: u32,
    pub cDstChannels: u32,
    pub dwChannelMask: u32,
    pub Coeff: [i32; 64],
}
impl Default for MFFOLDDOWN_MATRIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MFFrameSourceTypes = i32;
pub const MFFrameSourceTypes_Color: MFFrameSourceTypes = 1;
pub const MFFrameSourceTypes_Custom: MFFrameSourceTypes = 128;
pub const MFFrameSourceTypes_Depth: MFFrameSourceTypes = 4;
pub const MFFrameSourceTypes_Image: MFFrameSourceTypes = 8;
pub const MFFrameSourceTypes_Infrared: MFFrameSourceTypes = 2;
pub const MFImageFormat_JPEG: windows_core::GUID = windows_core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const MFImageFormat_RGB32: windows_core::GUID = windows_core::GUID::from_u128(0x00000016_0000_0010_8000_00aa00389b71);
pub const MFMPEG4Format_Base: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_767a_494d_b478_f29d25dc9037);
pub const MFMediaType_Audio: windows_core::GUID = windows_core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const MFMediaType_Binary: windows_core::GUID = windows_core::GUID::from_u128(0x72178c25_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_Default: windows_core::GUID = windows_core::GUID::from_u128(0x81a412e6_8103_4b06_857f_1862781024ac);
pub const MFMediaType_FileTransfer: windows_core::GUID = windows_core::GUID::from_u128(0x72178c26_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_HTML: windows_core::GUID = windows_core::GUID::from_u128(0x72178c24_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_Image: windows_core::GUID = windows_core::GUID::from_u128(0x72178c23_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_Metadata: windows_core::GUID = windows_core::GUID::from_u128(0x2c8fa20c_82bb_4782_90a0_98a2a5bd8ef8);
pub const MFMediaType_MultiplexedFrames: windows_core::GUID = windows_core::GUID::from_u128(0x6ea542b0_281f_4231_a464_fe2f5022501c);
pub const MFMediaType_Perception: windows_core::GUID = windows_core::GUID::from_u128(0x597ff6f9_6ea2_4670_85b4_ea84073fe940);
pub const MFMediaType_Protected: windows_core::GUID = windows_core::GUID::from_u128(0x7b4b6fe6_9d04_4494_be14_7e0bd076c8e4);
pub const MFMediaType_SAMI: windows_core::GUID = windows_core::GUID::from_u128(0xe69669a0_3dcd_40cb_9e2e_3708387c0616);
pub const MFMediaType_Script: windows_core::GUID = windows_core::GUID::from_u128(0x72178c22_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const MFMediaType_Stream: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb83_524f_11ce_9f53_0020af0ba770);
pub const MFMediaType_Subtitle: windows_core::GUID = windows_core::GUID::from_u128(0xa6d13581_ed50_4e65_ae08_26065576aacc);
pub const MFMediaType_Video: windows_core::GUID = windows_core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
pub type MFPERIODICCALLBACK = Option<unsafe extern "system" fn(pcontext: windows_core::Ref<windows_core::IUnknown>)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFPinholeCameraIntrinsic_IntrinsicModel {
    pub Width: u32,
    pub Height: u32,
    pub CameraModel: MFCameraIntrinsic_PinholeCameraModel,
    pub DistortionModel: MFCameraIntrinsic_DistortionModel,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MFPinholeCameraIntrinsics {
    pub IntrinsicModelCount: u32,
    pub IntrinsicModels: [MFPinholeCameraIntrinsic_IntrinsicModel; 1],
}
impl Default for MFPinholeCameraIntrinsics {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFSESSIONCAP_DOES_NOT_USE_NETWORK: u32 = 64;
pub const MFSESSIONCAP_PAUSE: u32 = 4;
pub const MFSESSIONCAP_RATE_FORWARD: u32 = 16;
pub const MFSESSIONCAP_RATE_REVERSE: u32 = 32;
pub const MFSESSIONCAP_SEEK: u32 = 2;
pub const MFSESSIONCAP_START: u32 = 1;
pub const MFSTARTUP_FULL: u32 = 0;
pub const MFSTARTUP_LITE: u32 = 1;
pub const MFSTARTUP_NOSOCKET: u32 = 1;
pub type MFSampleEncryptionProtectionScheme = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFSampleExtensionPsnrYuv {
    pub psnrY: f32,
    pub psnrU: f32,
    pub psnrV: f32,
}
pub const MFSampleExtension_3DVideo: windows_core::GUID = windows_core::GUID::from_u128(0xf86f97a4_dd54_4e2e_9a5e_55fc2d74a005);
pub const MFSampleExtension_3DVideo_MultiView: MFVideo3DSampleFormat = 1;
pub const MFSampleExtension_3DVideo_Packed: MFVideo3DSampleFormat = 0;
pub const MFSampleExtension_3DVideo_SampleFormat: windows_core::GUID = windows_core::GUID::from_u128(0x08671772_e36f_4cff_97b3_d72e20987a48);
pub const MFSampleExtension_AccumulatedNonRefPicPercent: windows_core::GUID = windows_core::GUID::from_u128(0x79ea74df_a740_445b_bc98_c9ed1f260eee);
pub const MFSampleExtension_BottomFieldFirst: windows_core::GUID = windows_core::GUID::from_u128(0x941ce0a3_6ae3_4dda_9a08_a64298340617);
pub const MFSampleExtension_CameraExtrinsics: windows_core::GUID = windows_core::GUID::from_u128(0x6b761658_b7ec_4c3b_8225_8623cabec31d);
pub const MFSampleExtension_CaptureMetadata: windows_core::GUID = windows_core::GUID::from_u128(0x2ebe23a8_faf5_444a_a6a2_eb810880ab5d);
pub const MFSampleExtension_ChromaOnly: windows_core::GUID = windows_core::GUID::from_u128(0x1eb9179c_a01f_4845_8c04_0e65a26eb04f);
pub const MFSampleExtension_CleanPoint: windows_core::GUID = windows_core::GUID::from_u128(0x9cdf01d8_a0f0_43ba_b077_eaa06cbd728a);
pub const MFSampleExtension_ClosedCaption_CEA708: windows_core::GUID = windows_core::GUID::from_u128(0x26f09068_e744_47dc_aa03_dbf20403bde6);
pub const MFSampleExtension_ClosedCaption_CEA708_MAX_SIZE: u32 = 256;
pub const MFSampleExtension_Content_KeyID: windows_core::GUID = windows_core::GUID::from_u128(0xc6c7f5b0_acca_415b_87d9_10441469efc6);
pub const MFSampleExtension_DecodeTimestamp: windows_core::GUID = windows_core::GUID::from_u128(0x73a954d4_09e2_4861_befc_94bd97c08e6e);
pub const MFSampleExtension_Depth_MaxReliableDepth: windows_core::GUID = windows_core::GUID::from_u128(0xe45545d1_1f0f_4a32_a8a7_6101a24ea8be);
pub const MFSampleExtension_Depth_MinReliableDepth: windows_core::GUID = windows_core::GUID::from_u128(0x5f8582b2_e36b_47c8_9b87_fee1ca72c5b0);
pub const MFSampleExtension_DerivedFromTopField: windows_core::GUID = windows_core::GUID::from_u128(0x6852465a_ae1c_4553_8e9b_c3420fcb1637);
pub const MFSampleExtension_DescrambleData: windows_core::GUID = windows_core::GUID::from_u128(0x43483be6_4903_4314_b032_2951365936fc);
pub const MFSampleExtension_DirtyRects: windows_core::GUID = windows_core::GUID::from_u128(0x9ba70225_b342_4e97_9126_0b566ab7ea7e);
pub const MFSampleExtension_Discontinuity: windows_core::GUID = windows_core::GUID::from_u128(0x9cdf01d9_a0f0_43ba_b077_eaa06cbd728a);
pub const MFSampleExtension_Encryption_ClearSliceHeaderData: windows_core::GUID = windows_core::GUID::from_u128(0x5509a4f4_320d_4e6c_8d1a_94c66dd20cb0);
pub const MFSampleExtension_Encryption_CryptByteBlock: windows_core::GUID = windows_core::GUID::from_u128(0x9d84289b_0c7f_4713_ab95_108ab42ad801);
pub const MFSampleExtension_Encryption_HardwareProtection: windows_core::GUID = windows_core::GUID::from_u128(0x9a2b2d2b_8270_43e3_8448_994f426e8886);
pub const MFSampleExtension_Encryption_HardwareProtection_KeyInfo: windows_core::GUID = windows_core::GUID::from_u128(0xb2372080_455b_4dd7_9989_1a955784b754);
pub const MFSampleExtension_Encryption_HardwareProtection_KeyInfoID: windows_core::GUID = windows_core::GUID::from_u128(0x8cbfcceb_94a5_4de1_8231_a85e47cf81e7);
pub const MFSampleExtension_Encryption_HardwareProtection_VideoDecryptorContext: windows_core::GUID = windows_core::GUID::from_u128(0x693470c8_e837_47a0_88cb_535b905e3582);
pub const MFSampleExtension_Encryption_KeyID: windows_core::GUID = windows_core::GUID::from_u128(0x76376591_795f_4da1_86ed_9d46eca109a9);
pub const MFSampleExtension_Encryption_NALUTypes: windows_core::GUID = windows_core::GUID::from_u128(0xb0f067c7_714c_416c_8d59_5f4ddf8913b6);
pub const MFSampleExtension_Encryption_Opaque_Data: windows_core::GUID = windows_core::GUID::from_u128(0x224d77e5_1391_4ffb_9f41_b432f68c611d);
pub const MFSampleExtension_Encryption_ProtectionScheme: windows_core::GUID = windows_core::GUID::from_u128(0xd054d096_28bb_45da_87ec_74f351871406);
pub const MFSampleExtension_Encryption_ResumeVideoOutput: windows_core::GUID = windows_core::GUID::from_u128(0xa435aba5_afde_4cf5_bc1c_f6acaf13949d);
pub const MFSampleExtension_Encryption_SEIData: windows_core::GUID = windows_core::GUID::from_u128(0x3cf0e972_4542_4687_9999_585f565fba7d);
pub const MFSampleExtension_Encryption_SPSPPSData: windows_core::GUID = windows_core::GUID::from_u128(0xaede0fa2_0e0c_453c_b7f3_de8693364d11);
pub const MFSampleExtension_Encryption_SampleID: windows_core::GUID = windows_core::GUID::from_u128(0x6698b84e_0afa_4330_aeb2_1c0a98d7a44d);
pub const MFSampleExtension_Encryption_SkipByteBlock: windows_core::GUID = windows_core::GUID::from_u128(0x0d550548_8317_4ab1_845f_d06306e293e3);
pub const MFSampleExtension_Encryption_SubSampleMappingSplit: windows_core::GUID = windows_core::GUID::from_u128(0xfe0254b9_2aa5_4edc_99f7_17e89dbf9174);
pub const MFSampleExtension_Encryption_SubSample_Mapping: windows_core::GUID = windows_core::GUID::from_u128(0x8444f27a_69a1_48da_bd08_11cef36830d2);
pub const MFSampleExtension_FeatureMap: windows_core::GUID = windows_core::GUID::from_u128(0xa032d165_46fc_400a_b449_49de53e62a6e);
pub const MFSampleExtension_ForwardedDecodeUnitType: windows_core::GUID = windows_core::GUID::from_u128(0x089e57c7_47d3_4a26_bf9c_4b64fafb5d1e);
pub const MFSampleExtension_ForwardedDecodeUnits: windows_core::GUID = windows_core::GUID::from_u128(0x424c754c_97c8_48d6_8777_fc41f7b60879);
pub const MFSampleExtension_FrameCorruption: windows_core::GUID = windows_core::GUID::from_u128(0xb4dd4a8c_0beb_44c4_8b75_b02b913b04f0);
pub const MFSampleExtension_FramePsnrYuv: windows_core::GUID = windows_core::GUID::from_u128(0x1c633a3d_566f_4752_833b_2907df5415e1);
pub const MFSampleExtension_GenKeyCtx: windows_core::GUID = windows_core::GUID::from_u128(0x188120cb_d7da_4b59_9b3e_9252fd37301c);
pub const MFSampleExtension_GenKeyFunc: windows_core::GUID = windows_core::GUID::from_u128(0x441ca1ee_6b1f_4501_903a_de87df42f6ed);
pub const MFSampleExtension_HDCP_FrameCounter: windows_core::GUID = windows_core::GUID::from_u128(0x9d389c60_f507_4aa6_a40a_71027a02f3de);
pub const MFSampleExtension_HDCP_OptionalHeader: windows_core::GUID = windows_core::GUID::from_u128(0x9a2e7390_121f_455f_8376_c97428e0b540);
pub const MFSampleExtension_HDCP_StreamID: windows_core::GUID = windows_core::GUID::from_u128(0x177e5d74_c370_4a7a_95a2_36833c01d0af);
pub const MFSampleExtension_Interlaced: windows_core::GUID = windows_core::GUID::from_u128(0xb1d5830a_deb8_40e3_90fa_389943716461);
pub const MFSampleExtension_LastSlice: windows_core::GUID = windows_core::GUID::from_u128(0x2b5d5457_5547_4f07_b8c8_b4a3a9a1daac);
pub const MFSampleExtension_LongTermReferenceFrameInfo: windows_core::GUID = windows_core::GUID::from_u128(0x9154733f_e1bd_41bf_81d3_fcd918f71332);
pub const MFSampleExtension_MDLCacheCookie: windows_core::GUID = windows_core::GUID::from_u128(0x5f002af9_d8f9_41a3_b6c3_a2ad43f647ad);
pub const MFSampleExtension_MULTIPLEXED_MANAGER: windows_core::GUID = windows_core::GUID::from_u128(0x8dcdee79_6b5a_4c45_8db9_20b395f02fcf);
pub const MFSampleExtension_MaxDecodeFrameSize: windows_core::GUID = windows_core::GUID::from_u128(0xd3cc654f_f9f3_4a13_889f_f04eb2b5b957);
pub const MFSampleExtension_MeanAbsoluteDifference: windows_core::GUID = windows_core::GUID::from_u128(0x1cdbde11_08b4_4311_a6dd_0f9f371907aa);
pub const MFSampleExtension_MoveRegions: windows_core::GUID = windows_core::GUID::from_u128(0xe2a6c693_3a8b_4b8d_95d0_f60281a12fb7);
pub const MFSampleExtension_NALULengthInfo: windows_core::GUID = windows_core::GUID::from_u128(0x19124e7c_ad4b_465f_bb18_20186287b6af);
pub const MFSampleExtension_PacketCrossOffsets: windows_core::GUID = windows_core::GUID::from_u128(0x2789671d_389f_40bb_90d9_c282f77f9abd);
pub const MFSampleExtension_PhotoThumbnail: windows_core::GUID = windows_core::GUID::from_u128(0x74bbc85c_c8bb_42dc_b586_da17ffd35dcc);
pub const MFSampleExtension_PhotoThumbnailMediaType: windows_core::GUID = windows_core::GUID::from_u128(0x61ad5420_ebf8_4143_89af_6bf25f672def);
pub const MFSampleExtension_PinholeCameraIntrinsics: windows_core::GUID = windows_core::GUID::from_u128(0x4ee3b6c5_6a15_4e72_9761_70c1db8b9fe3);
pub const MFSampleExtension_ROIRectangle: windows_core::GUID = windows_core::GUID::from_u128(0x3414a438_4998_4d2c_be82_be3ca0b24d43);
pub const MFSampleExtension_RepeatFirstField: windows_core::GUID = windows_core::GUID::from_u128(0x304d257c_7493_4fbd_b149_9228de8d9a99);
pub const MFSampleExtension_RepeatFrame: windows_core::GUID = windows_core::GUID::from_u128(0x88be738f_0711_4f42_b458_344aed42ec2f);
pub const MFSampleExtension_SampleKeyID: windows_core::GUID = windows_core::GUID::from_u128(0x9ed713c8_9b87_4b26_8297_a93b0c5a8acc);
pub const MFSampleExtension_SingleField: windows_core::GUID = windows_core::GUID::from_u128(0x9d85f816_658b_455a_bde0_9fa7e15ab8f9);
pub const MFSampleExtension_SpatialLayerId: windows_core::GUID = windows_core::GUID::from_u128(0xb7aabc7b_2396_457a_879e_623bfab6e0ac);
pub const MFSampleExtension_TargetGlobalLuminance: windows_core::GUID = windows_core::GUID::from_u128(0x3f60ef36_31ef_4daf_8360_940397e41ef3);
pub const MFSampleExtension_TemporalLayerId: windows_core::GUID = windows_core::GUID::from_u128(0xb3c1fcd2_b331_4376_b974_ad647769b2b0);
pub const MFSampleExtension_Timestamp: windows_core::GUID = windows_core::GUID::from_u128(0x1e436999_69be_4c7a_9369_70068c0260cb);
pub const MFSampleExtension_Token: windows_core::GUID = windows_core::GUID::from_u128(0x8294da66_f328_4805_b551_00deb4c57a61);
pub const MFSampleExtension_VideoEncodeBitsUsedMap: windows_core::GUID = windows_core::GUID::from_u128(0x6894263d_e6e2_4bcc_849d_8570365f5114);
pub const MFSampleExtension_VideoEncodeD3D12ReconstructedPicture: windows_core::GUID = windows_core::GUID::from_u128(0x3e8a1b7f_5c92_4d6e_b834_f0a729e65c48);
pub const MFSampleExtension_VideoEncodeInputAbsoluteQPMap: windows_core::GUID = windows_core::GUID::from_u128(0x432a6e9a_f1ed_456e_8dc3_6f8985649eb9);
pub const MFSampleExtension_VideoEncodeInputDeltaQPMap: windows_core::GUID = windows_core::GUID::from_u128(0xdab419c3_bf21_4b46_8692_9a7bf0a71769);
pub const MFSampleExtension_VideoEncodePictureType: windows_core::GUID = windows_core::GUID::from_u128(0x973704e6_cd14_483c_8f20_c9fc0928bad5);
pub const MFSampleExtension_VideoEncodeQP: windows_core::GUID = windows_core::GUID::from_u128(0xb2efe478_f979_4c66_b95e_ee2b82c82f36);
pub const MFSampleExtension_VideoEncodeQPMap: windows_core::GUID = windows_core::GUID::from_u128(0x2c68a331_b712_49ca_860a_3a1d58237d88);
pub const MFSampleExtension_VideoEncodeSatdMap: windows_core::GUID = windows_core::GUID::from_u128(0xadf61d96_c2d3_4b57_a138_dde4d351eaa9);
pub const MFStreamExtension_CameraExtrinsics: windows_core::GUID = windows_core::GUID::from_u128(0x686196d0_13e2_41d9_9638_ef032c272a52);
pub const MFStreamExtension_PinholeCameraIntrinsics: windows_core::GUID = windows_core::GUID::from_u128(0xdbac0455_0ec8_4aef_9c32_7a3ee3456f53);
pub const MFStreamFormat_MPEG2Program: windows_core::GUID = windows_core::GUID::from_u128(0x263067d1_d330_45dc_b669_34d986e4e3e1);
pub const MFStreamFormat_MPEG2Transport: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8023_db46_11cf_b4d1_00805f6cbbea);
pub const MFSubtitleFormat_ATSC: windows_core::GUID = windows_core::GUID::from_u128(0x7fa7faa3_feae_4e16_aedf_36b9acfbb099);
pub const MFSubtitleFormat_CustomUserData: windows_core::GUID = windows_core::GUID::from_u128(0x1bb3d849_6614_4d80_8882_ed24aa82da92);
pub const MFSubtitleFormat_PGS: windows_core::GUID = windows_core::GUID::from_u128(0x71f40e4a_1278_4442_b30d_39dd1d7722bc);
pub const MFSubtitleFormat_SRT: windows_core::GUID = windows_core::GUID::from_u128(0x5e467f2e_77ca_4ca5_8391_d142ed4b76c8);
pub const MFSubtitleFormat_SSA: windows_core::GUID = windows_core::GUID::from_u128(0x57176a1b_1a9e_4eea_abef_c61760198ac4);
pub const MFSubtitleFormat_TTML: windows_core::GUID = windows_core::GUID::from_u128(0x73e73992_9a10_4356_9557_7194e91e3e54);
pub const MFSubtitleFormat_VobSub: windows_core::GUID = windows_core::GUID::from_u128(0x6b8e40f4_8d2c_4ced_ad91_5960e45b4433);
pub const MFSubtitleFormat_WebVTT: windows_core::GUID = windows_core::GUID::from_u128(0xc886d215_f485_40bb_8db6_fadbc619a45d);
pub const MFSubtitleFormat_XML: windows_core::GUID = windows_core::GUID::from_u128(0x2006f94f_29ca_4195_b8db_00ded8ff0c97);
pub const MFT_CATEGORY_AUDIO_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0x9ea73fb4_ef7a_4559_8d5d_719d8f0426c7);
pub const MFT_CATEGORY_AUDIO_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0x11064c48_3648_4ed0_932e_05ce8ac811b7);
pub const MFT_CATEGORY_AUDIO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x91c64bd0_f91e_4d8c_9276_db248279d975);
pub const MFT_CATEGORY_DEMULTIPLEXER: windows_core::GUID = windows_core::GUID::from_u128(0xa8700a7a_939b_44c5_99d7_76226b23b3f1);
pub const MFT_CATEGORY_ENCRYPTOR: windows_core::GUID = windows_core::GUID::from_u128(0xb0c687be_01cd_44b5_b8b2_7c1d7e058b1f);
pub const MFT_CATEGORY_MULTIPLEXER: windows_core::GUID = windows_core::GUID::from_u128(0x059c561e_05ae_4b61_b69d_55b61ee54a7b);
pub const MFT_CATEGORY_OTHER: windows_core::GUID = windows_core::GUID::from_u128(0x90175d57_b7ea_4901_aeb3_933a8747756f);
pub const MFT_CATEGORY_VIDEO_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0xd6c02d4b_6833_45b4_971a_05a4b04bab91);
pub const MFT_CATEGORY_VIDEO_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0x12e17c21_532c_4a6e_8a1c_40825a736397);
pub const MFT_CATEGORY_VIDEO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0xf79eac7d_e545_4387_bdee_d647d7bde42a);
pub const MFT_CATEGORY_VIDEO_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x302ea3fc_aa5f_47f9_9f7a_c2188bb16302);
pub const MFT_CATEGORY_VIDEO_RENDERER_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0x145cd8b4_92f4_4b23_8ae7_e0df06c2da95);
pub const MFT_ENCODER_ERROR: windows_core::GUID = windows_core::GUID::from_u128(0xc8d1eda4_98e4_41d5_9297_44f53852f90e);
pub const MFT_ENUM_ADAPTER_LUID: windows_core::GUID = windows_core::GUID::from_u128(0x1d39518c_e220_4da8_a07f_ba172552d6b1);
pub const MFT_ENUM_FLAG_ALL: _MFT_ENUM_FLAG = 63;
pub const MFT_ENUM_FLAG_ASYNCMFT: _MFT_ENUM_FLAG = 2;
pub const MFT_ENUM_FLAG_FIELDOFUSE: _MFT_ENUM_FLAG = 8;
pub const MFT_ENUM_FLAG_HARDWARE: _MFT_ENUM_FLAG = 4;
pub const MFT_ENUM_FLAG_LOCALMFT: _MFT_ENUM_FLAG = 16;
pub const MFT_ENUM_FLAG_SORTANDFILTER: _MFT_ENUM_FLAG = 64;
pub const MFT_ENUM_FLAG_SORTANDFILTER_APPROVED_ONLY: _MFT_ENUM_FLAG = 192;
pub const MFT_ENUM_FLAG_SORTANDFILTER_WEB_ONLY: _MFT_ENUM_FLAG = 320;
pub const MFT_ENUM_FLAG_SORTANDFILTER_WEB_ONLY_EDGEMODE: _MFT_ENUM_FLAG = 576;
pub const MFT_ENUM_FLAG_SYNCMFT: _MFT_ENUM_FLAG = 1;
pub const MFT_ENUM_FLAG_TRANSCODE_ONLY: _MFT_ENUM_FLAG = 32;
pub const MFT_ENUM_FLAG_UNTRUSTED_STOREMFT: _MFT_ENUM_FLAG = 1024;
pub const MFT_ENUM_VIDEO_RENDERER_EXTENSION_PROFILE: windows_core::GUID = windows_core::GUID::from_u128(0x62c56928_9a4e_443b_b9dc_cac830c24100);
pub const MFT_GFX_DRIVER_VERSION_ID_Attribute: windows_core::GUID = windows_core::GUID::from_u128(0xf34b9093_05e0_4b16_993d_3e2a2cde6ad3);
pub const MFT_SUPPORT_DYNAMIC_FORMAT_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x53476a11_3f13_49fb_ac42_ee2733c96741);
pub type MFVideo3DFormat = i32;
pub type MFVideo3DSampleFormat = i32;
pub const MFVideo3DSampleFormat_BaseView: MFVideo3DFormat = 0;
pub const MFVideo3DSampleFormat_MultiView: MFVideo3DFormat = 1;
pub const MFVideo3DSampleFormat_Packed_LeftRight: MFVideo3DFormat = 2;
pub const MFVideo3DSampleFormat_Packed_TopBottom: MFVideo3DFormat = 3;
pub const MFVideoDRMFlag_AnalogProtected: MFVideoDRMFlags = 1;
pub const MFVideoDRMFlag_DigitallyProtected: MFVideoDRMFlags = 2;
pub const MFVideoDRMFlag_None: MFVideoDRMFlags = 0;
pub type MFVideoDRMFlags = i32;
pub const MFVideoFormat_Base_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0xeac3b9d5_bd14_4237_8f1f_bab428e49312);
pub const MFVideoFormat_H264_ES: windows_core::GUID = windows_core::GUID::from_u128(0x3f40f4f0_5622_4ff8_b6d8_a17a584bee5e);
pub const MFVideoFormat_H264_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0x5d0ce9dd_9817_49da_bdfd_f5f5b98f18a6);
pub const MFVideoFormat_HEVC_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0x3cfe0fe6_05c4_47dc_9d70_4bdb2959720f);
pub const MFVideoFormat_MPEG2: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const MFVideoPadFlag_PAD_TO_16x9: MFVideoPadFlags = 2;
pub const MFVideoPadFlag_PAD_TO_4x3: MFVideoPadFlags = 1;
pub const MFVideoPadFlag_PAD_TO_None: MFVideoPadFlags = 0;
pub type MFVideoPadFlags = i32;
pub type MFVideoRotationFormat = i32;
pub const MFVideoRotationFormat_0: MFVideoRotationFormat = 0;
pub const MFVideoRotationFormat_180: MFVideoRotationFormat = 180;
pub const MFVideoRotationFormat_270: MFVideoRotationFormat = 270;
pub const MFVideoRotationFormat_90: MFVideoRotationFormat = 90;
pub const MFVideoSrcContentHintFlag_16x9: MFVideoSrcContentHintFlags = 1;
pub const MFVideoSrcContentHintFlag_235_1: MFVideoSrcContentHintFlags = 2;
pub const MFVideoSrcContentHintFlag_None: MFVideoSrcContentHintFlags = 0;
pub type MFVideoSrcContentHintFlags = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MFWORKITEM_KEY(pub u64);
pub const MFWaveFormatExConvertFlag_ForceExtensible: MFWaveFormatExConvertFlags = 1;
pub const MFWaveFormatExConvertFlag_Normal: MFWaveFormatExConvertFlags = 0;
pub type MFWaveFormatExConvertFlags = i32;
pub const MF_1024_BYTE_ALIGNMENT: u32 = 1023;
pub const MF_128_BYTE_ALIGNMENT: u32 = 127;
pub const MF_16_BYTE_ALIGNMENT: u32 = 15;
pub const MF_1_BYTE_ALIGNMENT: u32 = 0;
pub const MF_2048_BYTE_ALIGNMENT: u32 = 2047;
pub const MF_256_BYTE_ALIGNMENT: u32 = 255;
pub const MF_2_BYTE_ALIGNMENT: u32 = 1;
pub const MF_32_BYTE_ALIGNMENT: u32 = 31;
pub const MF_4096_BYTE_ALIGNMENT: u32 = 4095;
pub const MF_4_BYTE_ALIGNMENT: u32 = 3;
pub const MF_512_BYTE_ALIGNMENT: u32 = 511;
pub const MF_64_BYTE_ALIGNMENT: u32 = 63;
pub const MF_8192_BYTE_ALIGNMENT: u32 = 8191;
pub const MF_8_BYTE_ALIGNMENT: u32 = 7;
pub const MF_API_VERSION: u32 = 112;
pub const MF_CAPTURE_METADATA_DIGITALWINDOW: windows_core::GUID = windows_core::GUID::from_u128(0x276f72a2_59c8_4f69_97b4_068b8c0ec044);
pub const MF_CAPTURE_METADATA_EXIF: windows_core::GUID = windows_core::GUID::from_u128(0x2e9575b8_8c31_4a02_8575_42b197b71592);
pub const MF_CAPTURE_METADATA_EXPOSURE_COMPENSATION: windows_core::GUID = windows_core::GUID::from_u128(0xd198aa75_4b62_4345_abf3_3c31fa12c299);
pub const MF_CAPTURE_METADATA_EXPOSURE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x16b9ae99_cd84_4063_879d_a28c7633729e);
pub const MF_CAPTURE_METADATA_FACEROICHARACTERIZATIONS: windows_core::GUID = windows_core::GUID::from_u128(0xb927a1a8_18ef_46d3_b3af_69372f94d9b2);
pub const MF_CAPTURE_METADATA_FACEROIS: windows_core::GUID = windows_core::GUID::from_u128(0x864f25a6_349f_46b1_a30e_54cc22928a47);
pub const MF_CAPTURE_METADATA_FACEROITIMESTAMPS: windows_core::GUID = windows_core::GUID::from_u128(0xe94d50cc_3da0_44d4_bb34_83198a741868);
pub const MF_CAPTURE_METADATA_FIRST_SCANLINE_START_TIME_QPC: windows_core::GUID = windows_core::GUID::from_u128(0x6a2c49f1_e052_46b6_b2d9_73c1558709af);
pub const MF_CAPTURE_METADATA_FLASH: windows_core::GUID = windows_core::GUID::from_u128(0x4a51520b_fb36_446c_9df2_68171b9a0389);
pub const MF_CAPTURE_METADATA_FLASH_POWER: windows_core::GUID = windows_core::GUID::from_u128(0x9c0e0d49_0205_491a_bc9d_2d6e1f4d5684);
pub const MF_CAPTURE_METADATA_FOCUSSTATE: windows_core::GUID = windows_core::GUID::from_u128(0xa87ee154_997f_465d_b91f_29d53b982b88);
pub const MF_CAPTURE_METADATA_FRAME_BACKGROUND_MASK: windows_core::GUID = windows_core::GUID::from_u128(0x03f14dd3_75dd_433a_a8e2_1e3f5f2a50a0);
pub const MF_CAPTURE_METADATA_FRAME_ILLUMINATION: windows_core::GUID = windows_core::GUID::from_u128(0x6d688ffc_63d3_46fe_bada_5b947db0d080);
pub const MF_CAPTURE_METADATA_FRAME_RAWSTREAM: windows_core::GUID = windows_core::GUID::from_u128(0x9252077b_2680_49b9_ae02_b19075973b70);
pub const MF_CAPTURE_METADATA_HISTOGRAM: windows_core::GUID = windows_core::GUID::from_u128(0x85358432_2ef6_4ba9_a3fb_06d82974b895);
pub const MF_CAPTURE_METADATA_ISO_GAINS: windows_core::GUID = windows_core::GUID::from_u128(0x05802ac9_0e1d_41c7_a8c8_7e7369f84e1e);
pub const MF_CAPTURE_METADATA_ISO_SPEED: windows_core::GUID = windows_core::GUID::from_u128(0xe528a68f_b2e3_44fe_8b65_07bf4b5a13ff);
pub const MF_CAPTURE_METADATA_LAST_SCANLINE_END_TIME_QPC: windows_core::GUID = windows_core::GUID::from_u128(0xdccadecb_c4d4_400d_b418_10e88525e1f6);
pub const MF_CAPTURE_METADATA_LENS_POSITION: windows_core::GUID = windows_core::GUID::from_u128(0xb5fc8e86_11d1_4e70_819b_723a89fa4520);
pub const MF_CAPTURE_METADATA_PHOTO_FRAME_FLASH: windows_core::GUID = windows_core::GUID::from_u128(0x0f9dd6c6_6003_45d8_bd59_f1f53e3d04e8);
pub const MF_CAPTURE_METADATA_REQUESTED_FRAME_SETTING_ID: windows_core::GUID = windows_core::GUID::from_u128(0xbb3716d9_8a61_47a4_8197_459c7ff174d5);
pub const MF_CAPTURE_METADATA_SCANLINE_DIRECTION: windows_core::GUID = windows_core::GUID::from_u128(0x6496a3ba_1907_49e6_b0c3_123795f380a9);
pub const MF_CAPTURE_METADATA_SCANLINE_TIME_QPC_ACCURACY: windows_core::GUID = windows_core::GUID::from_u128(0x4cd79c51_f765_4b09_b1e1_27d1f7ebea09);
pub const MF_CAPTURE_METADATA_SCENE_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x9cc3b54d_5ed3_4bae_b388_7670aef59e13);
pub const MF_CAPTURE_METADATA_SENSORFRAMERATE: windows_core::GUID = windows_core::GUID::from_u128(0xdb51357e_9d3d_4962_b06d_07ce650d9a0a);
pub const MF_CAPTURE_METADATA_UVC_PAYLOADHEADER: windows_core::GUID = windows_core::GUID::from_u128(0xf9f88a87_e1dd_441e_95cb_42e21a64f1d9);
pub const MF_CAPTURE_METADATA_WHITEBALANCE: windows_core::GUID = windows_core::GUID::from_u128(0xc736fd77_0fb9_4e2e_97a2_fcd490739ee9);
pub const MF_CAPTURE_METADATA_WHITEBALANCE_GAINS: windows_core::GUID = windows_core::GUID::from_u128(0xe7570c8f_2dcb_4c7c_aace_22ece7cce647);
pub const MF_CAPTURE_METADATA_ZOOMFACTOR: windows_core::GUID = windows_core::GUID::from_u128(0xe50b0b81_e501_42c2_abf2_857ecb13fa5c);
pub type MF_CUSTOM_DECODE_UNIT_TYPE = i32;
pub const MF_DECODER_FWD_CUSTOM_SEI_DECODE_ORDER: windows_core::GUID = windows_core::GUID::from_u128(0xf13bbe3c_36d4_410a_b985_7a951a1e6294);
pub const MF_DECODE_UNIT_NAL: MF_CUSTOM_DECODE_UNIT_TYPE = 0;
pub const MF_DECODE_UNIT_SEI: MF_CUSTOM_DECODE_UNIT_TYPE = 1;
pub const MF_DEVICESTREAM_ATTRIBUTE_FACEAUTH_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0xcb6fd12a_2248_4e41_ad46_e78bb90ab9fc);
pub const MF_DEVICESTREAM_ATTRIBUTE_FRAMESOURCE_TYPES: windows_core::GUID = windows_core::GUID::from_u128(0x17145fd1_1b2b_423c_8001_2b6833ed3588);
pub const MF_DEVICESTREAM_ATTRIBUTE_SECURE_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x940fd626_ea6e_4684_9840_36bd6ec9fbef);
pub const MF_DEVICESTREAM_MULTIPLEXED_MANAGER: windows_core::GUID = windows_core::GUID::from_u128(0x6ea542b0_281f_4231_a464_fe2f5022501c);
pub const MF_DISABLE_FRAME_CORRUPTION_INFO: windows_core::GUID = windows_core::GUID::from_u128(0x7086e16c_49c5_4201_882a_8538f38cf13a);
pub const MF_EVENT_DO_THINNING: windows_core::GUID = windows_core::GUID::from_u128(0x321ea6fb_dad9_46e4_b31d_d2eae7090e30);
pub const MF_EVENT_MFT_CONTEXT: windows_core::GUID = windows_core::GUID::from_u128(0xb7cd31f1_899e_4b41_80c9_26a896d32977);
pub const MF_EVENT_MFT_INPUT_STREAM_ID: windows_core::GUID = windows_core::GUID::from_u128(0xf29c2cca_7ae6_42d2_b284_bf837cc874e2);
pub const MF_EVENT_OUTPUT_NODE: windows_core::GUID = windows_core::GUID::from_u128(0x830f1a8b_c060_46dd_a801_1c95dec9b107);
pub const MF_EVENT_PRESENTATION_TIME_OFFSET: windows_core::GUID = windows_core::GUID::from_u128(0x5ad914d1_9b45_4a8d_a2c0_81d1e50bfb07);
pub const MF_EVENT_SCRUBSAMPLE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x9ac712b3_dcb8_44d5_8d0c_37455a2782e3);
pub const MF_EVENT_SESSIONCAPS: windows_core::GUID = windows_core::GUID::from_u128(0x7e5ebcd0_11b8_4abe_afad_10f6599a7f42);
pub const MF_EVENT_SESSIONCAPS_DELTA: windows_core::GUID = windows_core::GUID::from_u128(0x7e5ebcd1_11b8_4abe_afad_10f6599a7f42);
pub const MF_EVENT_SOURCE_ACTUAL_START: windows_core::GUID = windows_core::GUID::from_u128(0xa8cc55a9_6b31_419f_845d_ffb351a2434b);
pub const MF_EVENT_SOURCE_CHARACTERISTICS: windows_core::GUID = windows_core::GUID::from_u128(0x47db8490_8b22_4f52_afda_9ce1b2d3cfa8);
pub const MF_EVENT_SOURCE_CHARACTERISTICS_OLD: windows_core::GUID = windows_core::GUID::from_u128(0x47db8491_8b22_4f52_afda_9ce1b2d3cfa8);
pub const MF_EVENT_SOURCE_FAKE_START: windows_core::GUID = windows_core::GUID::from_u128(0xa8cc55a7_6b31_419f_845d_ffb351a2434b);
pub const MF_EVENT_SOURCE_PROJECTSTART: windows_core::GUID = windows_core::GUID::from_u128(0xa8cc55a8_6b31_419f_845d_ffb351a2434b);
pub const MF_EVENT_SOURCE_TOPOLOGY_CANCELED: windows_core::GUID = windows_core::GUID::from_u128(0xdb62f650_9a5e_4704_acf3_563bc6a73364);
pub const MF_EVENT_START_PRESENTATION_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x5ad914d0_9b45_4a8d_a2c0_81d1e50bfb07);
pub const MF_EVENT_START_PRESENTATION_TIME_AT_OUTPUT: windows_core::GUID = windows_core::GUID::from_u128(0x5ad914d2_9b45_4a8d_a2c0_81d1e50bfb07);
pub const MF_EVENT_STREAM_METADATA_CONTENT_KEYIDS: windows_core::GUID = windows_core::GUID::from_u128(0x5063449d_cc29_4fc6_a75a_d247b35af85c);
pub const MF_EVENT_STREAM_METADATA_KEYDATA: windows_core::GUID = windows_core::GUID::from_u128(0xcd59a4a1_4a3b_4bbd_8665_72a40fbea776);
pub const MF_EVENT_STREAM_METADATA_SYSTEMID: windows_core::GUID = windows_core::GUID::from_u128(0x1ea2ef64_ba16_4a36_8719_fe7560ba32ad);
pub const MF_EVENT_TOPOLOGY_STATUS: windows_core::GUID = windows_core::GUID::from_u128(0x30c5018d_9a53_454b_ad9e_6d5f8fa7c43b);
pub const MF_E_DXGI_DEVICE_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80041000_u32 as _);
pub const MF_E_DXGI_NEW_VIDEO_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x80041001_u32 as _);
pub const MF_E_DXGI_VIDEO_DEVICE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80041002_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MF_FLOAT2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MF_FLOAT3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub const MF_HISTOGRAM_CHANNEL_B: u32 = 8;
pub const MF_HISTOGRAM_CHANNEL_Cb: u32 = 16;
pub const MF_HISTOGRAM_CHANNEL_Cr: u32 = 32;
pub const MF_HISTOGRAM_CHANNEL_G: u32 = 4;
pub const MF_HISTOGRAM_CHANNEL_R: u32 = 2;
pub const MF_HISTOGRAM_CHANNEL_Y: u32 = 1;
pub const MF_LOW_LATENCY: windows_core::GUID = windows_core::GUID::from_u128(0x9c27891a_ed7a_40e1_88e8_b22727a024ee);
pub const MF_MEDIATYPE_MULTIPLEXED_MANAGER: windows_core::GUID = windows_core::GUID::from_u128(0x13c78fb5_f275_4ea0_bb5f_0249832b0d6e);
pub const MF_METADATAFACIALEXPRESSION_SMILE: u32 = 1;
pub const MF_METADATATIMESTAMPS_DEVICE: u32 = 1;
pub const MF_METADATATIMESTAMPS_PRESENTATION: u32 = 2;
pub const MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION: windows_core::GUID = windows_core::GUID::from_u128(0x7632f0e6_9538_4d61_acda_ea29c8c14456);
pub const MF_MT_AAC_PAYLOAD_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xbfbabe79_7434_4d1c_94f0_72a3b9e17188);
pub const MF_MT_ALL_SAMPLES_INDEPENDENT: windows_core::GUID = windows_core::GUID::from_u128(0xc9173739_5e56_461c_b713_46fb995cb95f);
pub const MF_MT_ALPHA_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x5d959b0d_4cbf_4d04_919f_3f5f7f284211);
pub const MF_MT_AM_FORMAT_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x73d1072d_1870_4174_a063_29ff4ff6c11e);
pub const MF_MT_ARBITRARY_FORMAT: windows_core::GUID = windows_core::GUID::from_u128(0x5a75b249_0d7d_49a1_a1c3_e0d87f0cade5);
pub const MF_MT_ARBITRARY_HEADER: windows_core::GUID = windows_core::GUID::from_u128(0x9e6bd6f5_0109_4f95_84ac_9309153a19fc);
pub const MF_MT_AUDIO_AVG_BYTES_PER_SECOND: windows_core::GUID = windows_core::GUID::from_u128(0x1aab75c8_cfef_451c_ab95_ac034b8e1731);
pub const MF_MT_AUDIO_BITS_PER_SAMPLE: windows_core::GUID = windows_core::GUID::from_u128(0xf2deb57f_40fa_4764_aa33_ed4f2d1ff669);
pub const MF_MT_AUDIO_BLOCK_ALIGNMENT: windows_core::GUID = windows_core::GUID::from_u128(0x322de230_9eeb_43bd_ab7a_ff412251541d);
pub const MF_MT_AUDIO_CHANNEL_MASK: windows_core::GUID = windows_core::GUID::from_u128(0x55fb5765_644a_4caf_8479_938983bb1588);
pub const MF_MT_AUDIO_FLAC_MAX_BLOCK_SIZE: windows_core::GUID = windows_core::GUID::from_u128(0x8b81adae_4b5a_4d40_8022_f38d09ca3c5c);
pub const MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND: windows_core::GUID = windows_core::GUID::from_u128(0xfb3b724a_cfb5_4319_aefe_6e42b2406132);
pub const MF_MT_AUDIO_FOLDDOWN_MATRIX: windows_core::GUID = windows_core::GUID::from_u128(0x9d62927c_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AUDIO_NUM_CHANNELS: windows_core::GUID = windows_core::GUID::from_u128(0x37e48bf5_645e_4c5b_89de_ada9e29b696a);
pub const MF_MT_AUDIO_PREFER_WAVEFORMATEX: windows_core::GUID = windows_core::GUID::from_u128(0xa901aaba_e037_458a_bdf6_545be2074042);
pub const MF_MT_AUDIO_SAMPLES_PER_BLOCK: windows_core::GUID = windows_core::GUID::from_u128(0xaab15aac_e13a_4995_9222_501ea15c6877);
pub const MF_MT_AUDIO_SAMPLES_PER_SECOND: windows_core::GUID = windows_core::GUID::from_u128(0x5faeeae7_0290_4c31_9e8a_c534f68d9dba);
pub const MF_MT_AUDIO_VALID_BITS_PER_SAMPLE: windows_core::GUID = windows_core::GUID::from_u128(0xd9bf8d6a_9530_4b7c_9ddf_ff6fd58bbd06);
pub const MF_MT_AUDIO_WMADRC_AVGREF: windows_core::GUID = windows_core::GUID::from_u128(0x9d62927f_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AUDIO_WMADRC_AVGTARGET: windows_core::GUID = windows_core::GUID::from_u128(0x9d629280_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AUDIO_WMADRC_PEAKREF: windows_core::GUID = windows_core::GUID::from_u128(0x9d62927d_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AUDIO_WMADRC_PEAKTARGET: windows_core::GUID = windows_core::GUID::from_u128(0x9d62927e_36be_4cf2_b5c4_a3926e3e8711);
pub const MF_MT_AVG_BITRATE: windows_core::GUID = windows_core::GUID::from_u128(0x20332624_fb0d_4d9e_bd0d_cbf6786c102e);
pub const MF_MT_AVG_BIT_ERROR_RATE: windows_core::GUID = windows_core::GUID::from_u128(0x799cabd6_3508_4db4_a3c7_569cd533deb1);
pub const MF_MT_COMPRESSED: windows_core::GUID = windows_core::GUID::from_u128(0x3afd0cee_18f2_4ba5_a110_8bea502e1f92);
pub const MF_MT_CONTAINER_RATE_SCALING: windows_core::GUID = windows_core::GUID::from_u128(0x83877f5e_0444_4e28_8479_6db0989b8c09);
pub const MF_MT_CUSTOM_VIDEO_PRIMARIES: windows_core::GUID = windows_core::GUID::from_u128(0x47537213_8cfb_4722_aa34_fbc9e24d77b8);
pub const MF_MT_DECODER_MAX_DPB_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x67be144c_88b7_4ca9_9628_c808d5262217);
pub const MF_MT_DECODER_USE_MAX_RESOLUTION: windows_core::GUID = windows_core::GUID::from_u128(0x4c547c24_af9a_4f38_96ad_978773cf53e7);
pub const MF_MT_DEFAULT_STRIDE: windows_core::GUID = windows_core::GUID::from_u128(0x644b4e48_1e02_4516_b0eb_c01ca9d49ac6);
pub const MF_MT_DEPTH_MEASUREMENT: windows_core::GUID = windows_core::GUID::from_u128(0xfd5ac489_0917_4bb6_9d54_3122bf70144b);
pub const MF_MT_DEPTH_VALUE_UNIT: windows_core::GUID = windows_core::GUID::from_u128(0x21a800f5_3189_4797_beba_f13cd9a31a5e);
pub const MF_MT_DRM_FLAGS: windows_core::GUID = windows_core::GUID::from_u128(0x8772f323_355a_4cc7_bb78_6d61a048ae82);
pub const MF_MT_DV_AAUX_CTRL_PACK_0: windows_core::GUID = windows_core::GUID::from_u128(0xf731004e_1dd1_4515_aabe_f0c06aa536ac);
pub const MF_MT_DV_AAUX_CTRL_PACK_1: windows_core::GUID = windows_core::GUID::from_u128(0xcd1f470d_1f04_4fe0_bfb9_d07ae0386ad8);
pub const MF_MT_DV_AAUX_SRC_PACK_0: windows_core::GUID = windows_core::GUID::from_u128(0x84bd5d88_0fb8_4ac8_be4b_a8848bef98f3);
pub const MF_MT_DV_AAUX_SRC_PACK_1: windows_core::GUID = windows_core::GUID::from_u128(0x720e6544_0225_4003_a651_0196563a958e);
pub const MF_MT_DV_VAUX_CTRL_PACK: windows_core::GUID = windows_core::GUID::from_u128(0x2f84e1c4_0da1_4788_938e_0dfbfbb34b48);
pub const MF_MT_DV_VAUX_SRC_PACK: windows_core::GUID = windows_core::GUID::from_u128(0x41402d9d_7b57_43c6_b129_2cb997f15009);
pub const MF_MT_FIXED_SIZE_SAMPLES: windows_core::GUID = windows_core::GUID::from_u128(0xb8ebefaf_b718_4e04_b0a9_116775e3321b);
pub const MF_MT_FORWARD_CUSTOM_NALU: windows_core::GUID = windows_core::GUID::from_u128(0xed336efd_244f_428d_9153_28f399458890);
pub const MF_MT_FORWARD_CUSTOM_SEI: windows_core::GUID = windows_core::GUID::from_u128(0xe27362f1_b136_41d1_9594_3a7e4febf2d1);
pub const MF_MT_FRAME_RATE: windows_core::GUID = windows_core::GUID::from_u128(0xc459a2e8_3d2c_4e44_b132_fee5156c7bb0);
pub const MF_MT_FRAME_RATE_RANGE_MAX: windows_core::GUID = windows_core::GUID::from_u128(0xe3371d41_b4cf_4a05_bd4e_20b88bb2c4d6);
pub const MF_MT_FRAME_RATE_RANGE_MIN: windows_core::GUID = windows_core::GUID::from_u128(0xd2e7558c_dc1f_403f_9a72_d28bb1eb3b5e);
pub const MF_MT_FRAME_SIZE: windows_core::GUID = windows_core::GUID::from_u128(0x1652c33d_d6b2_4012_b834_72030849a37d);
pub const MF_MT_GEOMETRIC_APERTURE: windows_core::GUID = windows_core::GUID::from_u128(0x66758743_7e5f_400d_980a_aa8596c85696);
pub const MF_MT_H264_CAPABILITIES: windows_core::GUID = windows_core::GUID::from_u128(0xbb3bd508_490a_11e0_99e4_1316dfd72085);
pub const MF_MT_H264_LAYOUT_PER_STREAM: windows_core::GUID = windows_core::GUID::from_u128(0x85e299b2_90e3_4fe8_b2f5_c067e0bfe57a);
pub const MF_MT_H264_MAX_CODEC_CONFIG_DELAY: windows_core::GUID = windows_core::GUID::from_u128(0xf5929986_4c45_4fbb_bb49_6cc534d05b9b);
pub const MF_MT_H264_MAX_MB_PER_SEC: windows_core::GUID = windows_core::GUID::from_u128(0x45256d30_7215_4576_9336_b0f1bcd59bb2);
pub const MF_MT_H264_RATE_CONTROL_MODES: windows_core::GUID = windows_core::GUID::from_u128(0x705177d8_45cb_11e0_ac7d_b91ce0d72085);
pub const MF_MT_H264_RESOLUTION_SCALING: windows_core::GUID = windows_core::GUID::from_u128(0xe3854272_f715_4757_ba90_1b696c773457);
pub const MF_MT_H264_SIMULCAST_SUPPORT: windows_core::GUID = windows_core::GUID::from_u128(0x9ea2d63d_53f0_4a34_b94e_9de49a078cb3);
pub const MF_MT_H264_SUPPORTED_RATE_CONTROL_MODES: windows_core::GUID = windows_core::GUID::from_u128(0x6a8ac47e_519c_4f18_9bb3_7eeaaea5594d);
pub const MF_MT_H264_SUPPORTED_SLICE_MODES: windows_core::GUID = windows_core::GUID::from_u128(0xc8be1937_4d64_4549_8343_a8086c0bfda5);
pub const MF_MT_H264_SUPPORTED_SYNC_FRAME_TYPES: windows_core::GUID = windows_core::GUID::from_u128(0x89a52c01_f282_48d2_b522_22e6ae633199);
pub const MF_MT_H264_SUPPORTED_USAGES: windows_core::GUID = windows_core::GUID::from_u128(0x60b1a998_dc01_40ce_9736_aba845a2dbdc);
pub const MF_MT_H264_SVC_CAPABILITIES: windows_core::GUID = windows_core::GUID::from_u128(0xf8993abe_d937_4a8f_bbca_6966fe9e1152);
pub const MF_MT_H264_USAGE: windows_core::GUID = windows_core::GUID::from_u128(0x359ce3a5_af00_49ca_a2f4_2ac94ca82b61);
pub const MF_MT_IMAGE_LOSS_TOLERANT: windows_core::GUID = windows_core::GUID::from_u128(0xed062cf4_e34e_4922_be99_934032133d7c);
pub const MF_MT_INTERLACE_MODE: windows_core::GUID = windows_core::GUID::from_u128(0xe2724bb8_e676_4806_b4b2_a8d6efb44ccd);
pub const MF_MT_IN_BAND_PARAMETER_SET: windows_core::GUID = windows_core::GUID::from_u128(0x75da5090_910b_4a03_896c_7b898feea5af);
pub const MF_MT_MAJOR_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x48eba18e_f8c9_4687_bf11_0a74c9f96a8f);
pub const MF_MT_MAX_FRAME_AVERAGE_LUMINANCE_LEVEL: windows_core::GUID = windows_core::GUID::from_u128(0x58d4bf57_6f52_4733_a195_a9e29ecf9e27);
pub const MF_MT_MAX_KEYFRAME_SPACING: windows_core::GUID = windows_core::GUID::from_u128(0xc16eb52b_73a1_476f_8d62_839d6a020652);
pub const MF_MT_MAX_LUMINANCE_LEVEL: windows_core::GUID = windows_core::GUID::from_u128(0x50253128_c110_4de4_98ae_46a324fae6da);
pub const MF_MT_MAX_MASTERING_LUMINANCE: windows_core::GUID = windows_core::GUID::from_u128(0xd6c6b997_272f_4ca1_8d00_8042111a0ff6);
pub const MF_MT_MINIMUM_DISPLAY_APERTURE: windows_core::GUID = windows_core::GUID::from_u128(0xd7388766_18fe_48c6_a177_ee894867c8c4);
pub const MF_MT_MIN_MASTERING_LUMINANCE: windows_core::GUID = windows_core::GUID::from_u128(0x839a4460_4e7e_4b4f_ae79_cc08905c7b27);
pub const MF_MT_MPEG2_CONTENT_PACKET: windows_core::GUID = windows_core::GUID::from_u128(0x825d55e4_4f12_4197_9eb3_59b6e4710f06);
pub const MF_MT_MPEG2_FLAGS: windows_core::GUID = windows_core::GUID::from_u128(0x31e3991d_f701_4b2f_b426_8ae3bda9e04b);
pub const MF_MT_MPEG2_HDCP: windows_core::GUID = windows_core::GUID::from_u128(0x168f1b4a_3e91_450f_aea7_e4baeadae5ba);
pub const MF_MT_MPEG2_LEVEL: windows_core::GUID = windows_core::GUID::from_u128(0x96f66574_11c5_4015_8666_bff516436da7);
pub const MF_MT_MPEG2_ONE_FRAME_PER_PACKET: windows_core::GUID = windows_core::GUID::from_u128(0x91a49eb5_1d20_4b42_ace8_804269bf95ed);
pub const MF_MT_MPEG2_PROFILE: windows_core::GUID = windows_core::GUID::from_u128(0xad76a80b_2d5c_4e0b_b375_64e520137036);
pub const MF_MT_MPEG2_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0xa20af9e8_928a_4b26_aaa9_f05c74cac47c);
pub const MF_MT_MPEG2_TIMECODE: windows_core::GUID = windows_core::GUID::from_u128(0x5229ba10_e29d_4f80_a59c_df4f180207d2);
pub const MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY: windows_core::GUID = windows_core::GUID::from_u128(0x9aa7e155_b64a_4c1d_a500_455d600b6560);
pub const MF_MT_MPEG4_SAMPLE_DESCRIPTION: windows_core::GUID = windows_core::GUID::from_u128(0x261e9d83_9529_4b8f_a111_8b9c950a81a9);
pub const MF_MT_MPEG4_TRACK_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x54f486dd_9327_4f6d_80ab_6f709ebb4cce);
pub const MF_MT_MPEGH_AUDIO_PROFILE_LEVEL_INDICATION: windows_core::GUID = windows_core::GUID::from_u128(0x51267a39_dd0c_4bb9_a7bd_9173ad4b131c);
pub const MF_MT_MPEG_SEQUENCE_HEADER: windows_core::GUID = windows_core::GUID::from_u128(0x3c036de7_3ad0_4c9e_9216_ee6d6ac21cb3);
pub const MF_MT_MPEG_START_TIME_CODE: windows_core::GUID = windows_core::GUID::from_u128(0x91f67885_4333_4280_97cd_bd5a6c03a06e);
pub const MF_MT_ORIGINAL_4CC: windows_core::GUID = windows_core::GUID::from_u128(0xd7be3fe0_2bc7_492d_b843_61a1919b70c3);
pub const MF_MT_ORIGINAL_WAVE_FORMAT_TAG: windows_core::GUID = windows_core::GUID::from_u128(0x8cbbc843_9fd9_49c2_882f_a72586c408ad);
pub const MF_MT_OUTPUT_BUFFER_NUM: windows_core::GUID = windows_core::GUID::from_u128(0xa505d3ac_f930_436e_8ede_93a509ce23b2);
pub const MF_MT_PAD_CONTROL_FLAGS: windows_core::GUID = windows_core::GUID::from_u128(0x4d0e73e5_80ea_4354_a9d0_1176ceb028ea);
pub const MF_MT_PALETTE: windows_core::GUID = windows_core::GUID::from_u128(0x6d283f42_9846_4410_afd9_654d503b1a54);
pub const MF_MT_PAN_SCAN_APERTURE: windows_core::GUID = windows_core::GUID::from_u128(0x79614dde_9187_48fb_b8c7_4d52689de649);
pub const MF_MT_PAN_SCAN_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x4b7f6bc3_8b13_40b2_a993_abf630b8204e);
pub const MF_MT_PIXEL_ASPECT_RATIO: windows_core::GUID = windows_core::GUID::from_u128(0xc6376a1e_8d0a_4027_be45_6d9a0ad39bb6);
pub const MF_MT_REALTIME_CONTENT: windows_core::GUID = windows_core::GUID::from_u128(0xbb12d222_2bdb_425e_91ec_2308e189a58f);
pub const MF_MT_SAMPLE_SIZE: windows_core::GUID = windows_core::GUID::from_u128(0xdad3ab78_1990_408b_bce2_eba673dacc10);
pub const MF_MT_SECURE: windows_core::GUID = windows_core::GUID::from_u128(0xc5acc4fd_0304_4ecf_809f_47bc97ff63bd);
pub const MF_MT_SOURCE_CONTENT_HINT: windows_core::GUID = windows_core::GUID::from_u128(0x68aca3cc_22d0_44e6_85f8_28167197fa38);
pub const MF_MT_SPATIAL_AUDIO_DATA_PRESENT: windows_core::GUID = windows_core::GUID::from_u128(0x6842f6e7_d43e_4ebb_9c9c_c96f41784863);
pub const MF_MT_SPATIAL_AUDIO_IS_PREVIRTUALIZED: windows_core::GUID = windows_core::GUID::from_u128(0x4eacab51_ffe5_421a_a2a7_8b7409a1cac4);
pub const MF_MT_SPATIAL_AUDIO_MAX_DYNAMIC_OBJECTS: windows_core::GUID = windows_core::GUID::from_u128(0xdcfba24a_2609_4240_a721_3faea76a4df9);
pub const MF_MT_SPATIAL_AUDIO_MAX_METADATA_ITEMS: windows_core::GUID = windows_core::GUID::from_u128(0x11aa80b4_e0da_47c6_8060_96c1259ae50d);
pub const MF_MT_SPATIAL_AUDIO_MIN_METADATA_ITEM_OFFSET_SPACING: windows_core::GUID = windows_core::GUID::from_u128(0x83e96ec9_1184_417e_8254_9f269158fc06);
pub const MF_MT_SPATIAL_AUDIO_OBJECT_METADATA_FORMAT_ID: windows_core::GUID = windows_core::GUID::from_u128(0x2ab71bc0_6223_4ba7_ad64_7b94b47ae792);
pub const MF_MT_SPATIAL_AUDIO_OBJECT_METADATA_LENGTH: windows_core::GUID = windows_core::GUID::from_u128(0x094ba8be_d723_489f_92fa_766777b34726);
pub const MF_MT_SUBTYPE: windows_core::GUID = windows_core::GUID::from_u128(0xf7e34c9a_42e8_4714_b74b_cb29d72c35e5);
pub const MF_MT_TIMESTAMP_CAN_BE_DTS: windows_core::GUID = windows_core::GUID::from_u128(0x24974215_1b7b_41e4_8625_ac469f2dedaa);
pub const MF_MT_TRANSFER_FUNCTION: windows_core::GUID = windows_core::GUID::from_u128(0x5fb0fce9_be5c_4935_a811_ec838f8eed93);
pub const MF_MT_USER_DATA: windows_core::GUID = windows_core::GUID::from_u128(0xb6bc765f_4c3b_40a4_bd51_2535b66fe09d);
pub const MF_MT_VIDEO_3D: windows_core::GUID = windows_core::GUID::from_u128(0xcb5e88cf_7b5b_476b_85aa_1ca5ae187555);
pub const MF_MT_VIDEO_3D_FIRST_IS_LEFT: windows_core::GUID = windows_core::GUID::from_u128(0xec298493_0ada_4ea1_a4fe_cbbd36ce9331);
pub const MF_MT_VIDEO_3D_FORMAT: windows_core::GUID = windows_core::GUID::from_u128(0x5315d8a0_87c5_4697_b793_6606c67c049b);
pub const MF_MT_VIDEO_3D_LEFT_IS_BASE: windows_core::GUID = windows_core::GUID::from_u128(0x6d4b7bff_5629_4404_948c_c634f4ce26d4);
pub const MF_MT_VIDEO_3D_NUM_VIEWS: windows_core::GUID = windows_core::GUID::from_u128(0xbb077e8a_dcbf_42eb_af60_418df98aa495);
pub const MF_MT_VIDEO_CHROMA_SITING: windows_core::GUID = windows_core::GUID::from_u128(0x65df2370_c773_4c33_aa64_843e068efb0c);
pub const MF_MT_VIDEO_H264_NO_FMOASO: windows_core::GUID = windows_core::GUID::from_u128(0xed461cd6_ec9f_416a_a8a3_26d7d31018d7);
pub const MF_MT_VIDEO_LEVEL: windows_core::GUID = windows_core::GUID::from_u128(0x96f66574_11c5_4015_8666_bff516436da7);
pub const MF_MT_VIDEO_LIGHTING: windows_core::GUID = windows_core::GUID::from_u128(0x53a0529c_890b_4216_8bf9_599367ad6d20);
pub const MF_MT_VIDEO_NOMINAL_RANGE: windows_core::GUID = windows_core::GUID::from_u128(0xc21b8ee5_b956_4071_8daf_325edf5cab11);
pub const MF_MT_VIDEO_NO_FRAME_ORDERING: windows_core::GUID = windows_core::GUID::from_u128(0x3f5b106f_6bc2_4ee3_b7ed_8902c18f5351);
pub const MF_MT_VIDEO_PRIMARIES: windows_core::GUID = windows_core::GUID::from_u128(0xdbfbe4d7_0740_4ee0_8192_850ab0e21935);
pub const MF_MT_VIDEO_PROFILE: windows_core::GUID = windows_core::GUID::from_u128(0xad76a80b_2d5c_4e0b_b375_64e520137036);
pub const MF_MT_VIDEO_RENDERER_EXTENSION_PROFILE: windows_core::GUID = windows_core::GUID::from_u128(0x8437d4b9_d448_4fcd_9b6b_839bf96c7798);
pub const MF_MT_VIDEO_ROTATION: windows_core::GUID = windows_core::GUID::from_u128(0xc380465d_2271_428c_9b83_ecea3b4a85c1);
pub const MF_MT_WRAPPED_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x4d3f7b23_d02f_4e6c_9bee_e4bf2c6c695d);
pub const MF_MT_YUV_MATRIX: windows_core::GUID = windows_core::GUID::from_u128(0x3e23d450_2c75_4d25_a00e_b91670d12327);
pub const MF_MULTITHREADED_WORKQUEUE: MFASYNC_WORKQUEUE_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MF_QUATERNION {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub const MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_AES_CBC: MFSampleEncryptionProtectionScheme = 2;
pub const MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_AES_CTR: MFSampleEncryptionProtectionScheme = 1;
pub const MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_NONE: MFSampleEncryptionProtectionScheme = 0;
pub const MF_SDK_VERSION: u32 = 2;
pub const MF_SD_AMBISONICS_SAMPLE3D_DESCRIPTION: windows_core::GUID = windows_core::GUID::from_u128(0xf715cf3e_a964_4c3f_94ae_9d6ba7264641);
pub const MF_STANDARD_WORKQUEUE: MFASYNC_WORKQUEUE_TYPE = 0;
pub type MF_TOPOSTATUS = i32;
pub const MF_TOPOSTATUS_DYNAMIC_CHANGED: MF_TOPOSTATUS = 210;
pub const MF_TOPOSTATUS_ENDED: MF_TOPOSTATUS = 400;
pub const MF_TOPOSTATUS_INVALID: MF_TOPOSTATUS = 0;
pub const MF_TOPOSTATUS_READY: MF_TOPOSTATUS = 100;
pub const MF_TOPOSTATUS_SINK_SWITCHED: MF_TOPOSTATUS = 300;
pub const MF_TOPOSTATUS_STARTED_SOURCE: MF_TOPOSTATUS = 200;
pub const MF_VERSION: u32 = 131184;
pub const MF_VIDEO_MAX_MB_PER_SEC: windows_core::GUID = windows_core::GUID::from_u128(0xe3f2e203_d445_4b8c_9211_ae390d3ba017);
pub const MF_VIDEO_RENDERER_EFFECT_APP_SERVICE_NAME: windows_core::GUID = windows_core::GUID::from_u128(0xc6052a80_6d9c_40a3_9db8_f027a25c9ab9);
pub const MF_WINDOW_WORKQUEUE: MFASYNC_WORKQUEUE_TYPE = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MOVEREGION_INFO {
    pub FrameNumber: u32,
    pub NumMoveRegions: u32,
    pub MoveRegions: [MOVE_RECT; 1],
}
#[cfg(feature = "windef")]
impl Default for MOVEREGION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOVE_RECT {
    pub SourcePoint: super::windef::POINT,
    pub DestRect: super::windef::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPEG2VIDEOINFO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MT_ARBITRARY_HEADER {
    pub majortype: windows_core::GUID,
    pub subtype: windows_core::GUID,
    pub bFixedSizeSamples: windows_core::BOOL,
    pub bTemporalCompression: windows_core::BOOL,
    pub lSampleSize: u32,
    pub formattype: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MT_CUSTOM_VIDEO_PRIMARIES {
    pub fRx: f32,
    pub fRy: f32,
    pub fGx: f32,
    pub fGy: f32,
    pub fBx: f32,
    pub fBy: f32,
    pub fWx: f32,
    pub fWy: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
#[cfg(feature = "windef")]
pub type PROI_AREA = *mut ROI_AREA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROI_AREA {
    pub rect: super::windef::RECT,
    pub QPDelta: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIDEOINFOHEADER2(pub u8);
pub type _MFT_ENUM_FLAG = i32;
pub type eAVEncVideoQPMapElementDataType = i32;
pub const eAllocationTypeDynamic: EAllocationType = 0;
pub const eAllocationTypeIgnore: EAllocationType = 3;
pub const eAllocationTypePageable: EAllocationType = 2;
pub const eAllocationTypeRT: EAllocationType = 1;
