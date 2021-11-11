#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn CreateNamedPropertyStore(ppstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn CreatePropertyStore(ppstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn DXVA2CreateDirect3DDeviceManager9(presettoken: *mut u32, ppdevicemanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn DXVA2CreateVideoService(pdd: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn DXVAHD_CreateDevice(pd3ddevice: ::windows::runtime::RawPtr, pcontentdesc: *const DXVAHD_CONTENT_DESC, usage: DXVAHD_DEVICE_USAGE, pplugin: ::windows::runtime::RawPtr, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAddPeriodicCallback(callback: ::windows::runtime::RawPtr, pcontext: ::windows::runtime::RawPtr, pdwkey: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAllocateSerialWorkQueue(dwworkqueue: u32, pdwworkqueue: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAllocateWorkQueue(pdwworkqueue: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAllocateWorkQueueEx(workqueuetype: MFASYNC_WORKQUEUE_TYPE, pdwworkqueue: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAverageTimePerFrameToFrameRate(unaveragetimeperframe: u64, punnumerator: *mut u32, pundenominator: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFBeginCreateFile(accessmode: MF_FILE_ACCESSMODE, openmode: MF_FILE_OPENMODE, fflags: MF_FILE_FLAGS, pwszfilepath: super::super::Foundation::PWSTR, pcallback: ::windows::runtime::RawPtr, pstate: ::windows::runtime::RawPtr, ppcancelcookie: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFBeginRegisterWorkQueueWithMMCSS(dwworkqueueid: u32, wszclass: super::super::Foundation::PWSTR, dwtaskid: u32, pdonecallback: ::windows::runtime::RawPtr, pdonestate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFBeginRegisterWorkQueueWithMMCSSEx(dwworkqueueid: u32, wszclass: super::super::Foundation::PWSTR, dwtaskid: u32, lpriority: i32, pdonecallback: ::windows::runtime::RawPtr, pdonestate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFBeginUnregisterWorkQueueWithMMCSS(dwworkqueueid: u32, pdonecallback: ::windows::runtime::RawPtr, pdonestate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MFCalculateBitmapImageSize(pbmih: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, cbbufsize: u32, pcbimagesize: *mut u32, pbknown: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCalculateImageSize(guidsubtype: *const ::windows::runtime::GUID, unwidth: u32, unheight: u32, pcbimagesize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCancelCreateFile(pcancelcookie: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCancelWorkItem(key: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCombineSamples(psample: ::windows::runtime::RawPtr, psampletoadd: ::windows::runtime::RawPtr, dwmaxmergeddurationinms: u32, pmerged: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCompareFullToPartialMediaType(pmftypefull: ::windows::runtime::RawPtr, pmftypepartial: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFConvertColorInfoFromDXVA(ptoformat: *mut MFVIDEOFORMAT, dwfromdxva: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFConvertColorInfoToDXVA(pdwtodxva: *mut u32, pfromformat: *const MFVIDEOFORMAT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFConvertFromFP16Array(pdest: *mut f32, psrc: *const u16, dwcount: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFConvertToFP16Array(pdest: *mut u16, psrc: *const f32, dwcount: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCopyImage(pdest: *mut u8, ldeststride: i32, psrc: *const u8, lsrcstride: i32, dwwidthinbytes: u32, dwlines: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreate2DMediaBuffer(dwwidth: u32, dwheight: u32, dwfourcc: u32, fbottomup: super::super::Foundation::BOOL, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreate3GPMediaSink(pibytestream: ::windows::runtime::RawPtr, pvideomediatype: ::windows::runtime::RawPtr, paudiomediatype: ::windows::runtime::RawPtr, ppimediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAC3MediaSink(ptargetbytestream: ::windows::runtime::RawPtr, paudiomediatype: ::windows::runtime::RawPtr, ppmediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateADTSMediaSink(ptargetbytestream: ::windows::runtime::RawPtr, paudiomediatype: ::windows::runtime::RawPtr, ppmediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub fn MFCreateAMMediaTypeFromMFMediaType(pmftype: ::windows::runtime::RawPtr, guidformatblocktype: ::windows::runtime::GUID, ppamtype: *mut *mut super::DirectShow::AM_MEDIA_TYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFContentInfo(ppicontentinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFIndexer(ppiindexer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFIndexerByteStream(picontentbytestream: ::windows::runtime::RawPtr, cbindexstartoffset: u64, piindexbytestream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFMediaSink(pibytestream: ::windows::runtime::RawPtr, ppimediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateASFMediaSinkActivate(pwszfilename: super::super::Foundation::PWSTR, pcontentinfo: ::windows::runtime::RawPtr, ppiactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFMultiplexer(ppimultiplexer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFProfile(ppiprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFProfileFromPresentationDescriptor(pipd: ::windows::runtime::RawPtr, ppiprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFSplitter(ppisplitter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFStreamSelector(piasfprofile: ::windows::runtime::RawPtr, ppselector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFStreamingMediaSink(pibytestream: ::windows::runtime::RawPtr, ppimediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFStreamingMediaSinkActivate(pbytestreamactivate: ::windows::runtime::RawPtr, pcontentinfo: ::windows::runtime::RawPtr, ppiactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAVIMediaSink(pibytestream: ::windows::runtime::RawPtr, pvideomediatype: ::windows::runtime::RawPtr, paudiomediatype: ::windows::runtime::RawPtr, ppimediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAggregateSource(psourcecollection: ::windows::runtime::RawPtr, ppaggsource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAlignedMemoryBuffer(cbmaxlength: u32, cbaligment: u32, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAsyncResult(punkobject: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, punkstate: ::windows::runtime::RawPtr, ppasyncresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAttributes(ppmfattributes: *mut ::windows::runtime::RawPtr, cinitialsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Media_Audio`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFCreateAudioMediaType(paudioformat: *const super::Audio::WAVEFORMATEX, ppiaudiomediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAudioRenderer(paudioattributes: ::windows::runtime::RawPtr, ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAudioRendererActivate(ppactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateCameraOcclusionStateMonitor(symboliclink: super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, occlusionstatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateCollection(ppimfcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateContentDecryptorContext(guidmediaprotectionsystemid: *const ::windows::runtime::GUID, pd3dmanager: ::windows::runtime::RawPtr, pcontentprotectiondevice: ::windows::runtime::RawPtr, ppcontentdecryptorcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateContentProtectionDevice(protectionsystemid: *const ::windows::runtime::GUID, contentprotectiondevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateCredentialCache(ppcache: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Direct3D12`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn MFCreateD3D12SynchronizationObject(pdevice: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvsyncobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateDXGIDeviceManager(resettoken: *mut u32, ppdevicemanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateDXGISurfaceBuffer(riid: *const ::windows::runtime::GUID, punksurface: ::windows::runtime::RawPtr, usubresourceindex: u32, fbottomupwhenlinear: super::super::Foundation::BOOL, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateDXSurfaceBuffer(riid: *const ::windows::runtime::GUID, punksurface: ::windows::runtime::RawPtr, fbottomupwhenlinear: super::super::Foundation::BOOL, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateDeviceSource(pattributes: ::windows::runtime::RawPtr, ppsource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateDeviceSourceActivate(pattributes: ::windows::runtime::RawPtr, ppactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn MFCreateEncryptedMediaExtensionsStoreActivate(pmphost: ::windows::runtime::RawPtr, objectstream: ::windows::runtime::RawPtr, classid: super::super::Foundation::PWSTR, activate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateEventQueue(ppmediaeventqueue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateExtendedCameraIntrinsicModel(distortionmodeltype: MFCameraIntrinsic_DistortionModelType, ppextendedcameraintrinsicmodel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateExtendedCameraIntrinsics(ppextendedcameraintrinsics: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateFMPEG4MediaSink(pibytestream: ::windows::runtime::RawPtr, pvideomediatype: ::windows::runtime::RawPtr, paudiomediatype: ::windows::runtime::RawPtr, ppimediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateFile(accessmode: MF_FILE_ACCESSMODE, openmode: MF_FILE_OPENMODE, fflags: MF_FILE_FLAGS, pwszfileurl: super::super::Foundation::PWSTR, ppibytestream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Media_DxMediaObjects`*"]
    #[cfg(feature = "Win32_Media_DxMediaObjects")]
    pub fn MFCreateLegacyMediaBufferOnMFMediaBuffer(psample: ::windows::runtime::RawPtr, pmfmediabuffer: ::windows::runtime::RawPtr, cboffset: u32, ppmediabuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFCreateMFByteStreamOnStream(pstream: ::windows::runtime::RawPtr, ppbytestream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMFByteStreamOnStreamEx(punkstream: ::windows::runtime::RawPtr, ppbytestream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMFByteStreamWrapper(pstream: ::windows::runtime::RawPtr, ppstreamwrapper: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateMFVideoFormatFromMFMediaType(pmftype: ::windows::runtime::RawPtr, ppmfvf: *mut *mut MFVIDEOFORMAT, pcbsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMP3MediaSink(ptargetbytestream: ::windows::runtime::RawPtr, ppmediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMPEG4MediaSink(pibytestream: ::windows::runtime::RawPtr, pvideomediatype: ::windows::runtime::RawPtr, paudiomediatype: ::windows::runtime::RawPtr, ppimediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaBufferFromMediaType(pmediatype: ::windows::runtime::RawPtr, llduration: i64, dwminlength: u32, dwminalignment: u32, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaBufferWrapper(pbuffer: ::windows::runtime::RawPtr, cboffset: u32, dwlength: u32, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFCreateMediaEvent(met: u32, guidextendedtype: *const ::windows::runtime::GUID, hrstatus: ::windows::runtime::HRESULT, pvvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, ppevent: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateMediaExtensionActivate(szactivatableclassid: super::super::Foundation::PWSTR, pconfiguration: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaSession(pconfiguration: ::windows::runtime::RawPtr, ppmediasession: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaType(ppmftype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaTypeFromProperties(punkstream: ::windows::runtime::RawPtr, ppmediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaTypeFromRepresentation(guidrepresentation: ::windows::runtime::GUID, pvrepresentation: *const ::core::ffi::c_void, ppimediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMemoryBuffer(cbmaxlength: u32, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMuxSink(guidoutputsubtype: ::windows::runtime::GUID, poutputattributes: ::windows::runtime::RawPtr, poutputbytestream: ::windows::runtime::RawPtr, ppmuxsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMuxStreamAttributes(pattributestomux: ::windows::runtime::RawPtr, ppmuxattribs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMuxStreamMediaType(pmediatypestomux: ::windows::runtime::RawPtr, ppmuxmediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMuxStreamSample(psamplestomux: ::windows::runtime::RawPtr, ppmuxsample: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateNetSchemePlugin(riid: *const ::windows::runtime::GUID, ppvhandler: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePMPMediaSession(dwcreationflags: u32, pconfiguration: ::windows::runtime::RawPtr, ppmediasession: *mut ::windows::runtime::RawPtr, ppenableractivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePMPServer(dwcreationflags: u32, pppmpserver: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePresentationClock(pppresentationclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePresentationDescriptor(cstreamdescriptors: u32, apstreamdescriptors: *const ::windows::runtime::RawPtr, pppresentationdescriptor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePresentationDescriptorFromASFProfile(piprofile: ::windows::runtime::RawPtr, ppipd: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePropertiesFromMediaType(pmediatype: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateProtectedEnvironmentAccess(ppaccess: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn MFCreateProxyLocator(pszprotocol: super::super::Foundation::PWSTR, pproxyconfig: ::windows::runtime::RawPtr, ppproxylocator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateRelativePanelWatcher(videodeviceid: super::super::Foundation::PWSTR, displaymonitordeviceid: super::super::Foundation::PWSTR, pprelativepanelwatcher: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateRemoteDesktopPlugin(ppplugin: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSample(ppimfsample: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSampleCopierMFT(ppcopiermft: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSampleGrabberSinkActivate(pimfmediatype: ::windows::runtime::RawPtr, pimfsamplegrabbersinkcallback: ::windows::runtime::RawPtr, ppiactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSensorActivityMonitor(pcallback: ::windows::runtime::RawPtr, ppactivitymonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateSensorGroup(sensorgroupsymboliclink: super::super::Foundation::PWSTR, ppsensorgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateSensorProfile(profiletype: *const ::windows::runtime::GUID, profileindex: u32, constraints: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSensorProfileCollection(ppsensorprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSensorStream(streamid: u32, pattributes: ::windows::runtime::RawPtr, pmediatypecollection: ::windows::runtime::RawPtr, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFCreateSequencerSegmentOffset(dwid: u32, hnsoffset: i64, pvarsegmentoffset: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSequencerSource(preserved: ::windows::runtime::RawPtr, ppsequencersource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSimpleTypeHandler(pphandler: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSinkWriterFromMediaSink(pmediasink: ::windows::runtime::RawPtr, pattributes: ::windows::runtime::RawPtr, ppsinkwriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateSinkWriterFromURL(pwszoutputurl: super::super::Foundation::PWSTR, pbytestream: ::windows::runtime::RawPtr, pattributes: ::windows::runtime::RawPtr, ppsinkwriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSourceReaderFromByteStream(pbytestream: ::windows::runtime::RawPtr, pattributes: ::windows::runtime::RawPtr, ppsourcereader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSourceReaderFromMediaSource(pmediasource: ::windows::runtime::RawPtr, pattributes: ::windows::runtime::RawPtr, ppsourcereader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateSourceReaderFromURL(pwszurl: super::super::Foundation::PWSTR, pattributes: ::windows::runtime::RawPtr, ppsourcereader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSourceResolver(ppisourceresolver: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateStandardQualityManager(ppqualitymanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateStreamDescriptor(dwstreamidentifier: u32, cmediatypes: u32, apmediatypes: *const ::windows::runtime::RawPtr, ppdescriptor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFCreateStreamOnMFByteStream(pbytestream: ::windows::runtime::RawPtr, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateStreamOnMFByteStreamEx(pbytestream: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSystemTimeSource(ppsystemtimesource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTempFile(accessmode: MF_FILE_ACCESSMODE, openmode: MF_FILE_OPENMODE, fflags: MF_FILE_FLAGS, ppibytestream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTopoLoader(ppobj: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTopology(pptopo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTopologyNode(nodetype: MF_TOPOLOGY_TYPE, ppnode: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTrackedSample(ppmfsample: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTranscodeProfile(pptranscodeprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTranscodeSinkActivate(ppactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateTranscodeTopology(psrc: ::windows::runtime::RawPtr, pwszoutputfilepath: super::super::Foundation::PWSTR, pprofile: ::windows::runtime::RawPtr, pptranscodetopo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTranscodeTopologyFromByteStream(psrc: ::windows::runtime::RawPtr, poutputstream: ::windows::runtime::RawPtr, pprofile: ::windows::runtime::RawPtr, pptranscodetopo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTransformActivate(ppactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateVideoMediaType(pvideoformat: *const MFVIDEOFORMAT, ppivideomediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn MFCreateVideoMediaTypeFromBitMapInfoHeader(pbmihbitmapinfoheader: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, dwpixelaspectratiox: u32, dwpixelaspectratioy: u32, interlacemode: MFVideoInterlaceMode, videoflags: u64, qwframespersecondnumerator: u64, qwframesperseconddenominator: u64, dwmaxbitrate: u32, ppivideomediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn MFCreateVideoMediaTypeFromBitMapInfoHeaderEx(pbmihbitmapinfoheader: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, cbbitmapinfoheader: u32, dwpixelaspectratiox: u32, dwpixelaspectratioy: u32, interlacemode: MFVideoInterlaceMode, videoflags: u64, dwframespersecondnumerator: u32, dwframesperseconddenominator: u32, dwmaxbitrate: u32, ppivideomediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoMediaTypeFromSubtype(pamsubtype: *const ::windows::runtime::GUID, ppivideomediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoMixer(powner: ::windows::runtime::RawPtr, riiddevice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoMixerAndPresenter(pmixerowner: ::windows::runtime::RawPtr, ppresenterowner: ::windows::runtime::RawPtr, riidmixer: *const ::windows::runtime::GUID, ppvvideomixer: *mut *mut ::core::ffi::c_void, riidpresenter: *const ::windows::runtime::GUID, ppvvideopresenter: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoPresenter(powner: ::windows::runtime::RawPtr, riiddevice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvideopresenter: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoRenderer(riidrenderer: *const ::windows::runtime::GUID, ppvideorenderer: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateVideoRendererActivate(hwndvideo: super::super::Foundation::HWND, ppactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoSampleAllocator(riid: *const ::windows::runtime::GUID, ppsampleallocator: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoSampleAllocatorEx(riid: *const ::windows::runtime::GUID, ppsampleallocator: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoSampleFromSurface(punksurface: ::windows::runtime::RawPtr, ppsample: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateVirtualCamera(r#type: __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0001, lifetime: __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0002, access: __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0003, friendlyname: super::super::Foundation::PWSTR, sourceid: super::super::Foundation::PWSTR, categories: *const ::windows::runtime::GUID, categorycount: u32, virtualcamera: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateWAVEMediaSink(ptargetbytestream: ::windows::runtime::RawPtr, paudiomediatype: ::windows::runtime::RawPtr, ppmediasink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateWICBitmapBuffer(riid: *const ::windows::runtime::GUID, punksurface: ::windows::runtime::RawPtr, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn MFCreateWMAEncoderActivate(pmediatype: ::windows::runtime::RawPtr, pencodingconfigurationproperties: ::windows::runtime::RawPtr, ppactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn MFCreateWMVEncoderActivate(pmediatype: ::windows::runtime::RawPtr, pencodingconfigurationproperties: ::windows::runtime::RawPtr, ppactivate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Media_Audio`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFCreateWaveFormatExFromMFMediaType(pmftype: ::windows::runtime::RawPtr, ppwf: *mut *mut super::Audio::WAVEFORMATEX, pcbsize: *mut u32, flags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFDeserializeAttributesFromStream(pattr: ::windows::runtime::RawPtr, dwoptions: u32, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFDeserializePresentationDescriptor(cbdata: u32, pbdata: *const u8, pppd: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFEndCreateFile(presult: ::windows::runtime::RawPtr, ppfile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFEndRegisterWorkQueueWithMMCSS(presult: ::windows::runtime::RawPtr, pdwtaskid: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFEndUnregisterWorkQueueWithMMCSS(presult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFEnumDeviceSources(pattributes: ::windows::runtime::RawPtr, pppsourceactivate: *mut *mut ::windows::runtime::RawPtr, pcsourceactivate: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFFrameRateToAverageTimePerFrame(unnumerator: u32, undenominator: u32, punaveragetimeperframe: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetAttributesAsBlob(pattributes: ::windows::runtime::RawPtr, pbuf: *mut u8, cbbufsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetAttributesAsBlobSize(pattributes: ::windows::runtime::RawPtr, pcbbufsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetContentProtectionSystemCLSID(guidprotectionsystemid: *const ::windows::runtime::GUID, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetLocalId(verifier: *const u8, size: u32, id: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetMFTMerit(pmft: ::windows::runtime::RawPtr, cbverifier: u32, verifier: *const u8, merit: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetPlaneSize(format: u32, dwwidth: u32, dwheight: u32, pdwplanesize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetPluginControl(ppplugincontrol: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetService(punkobject: ::windows::runtime::RawPtr, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetStrideForBitmapInfoHeader(format: u32, dwwidth: u32, pstride: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFGetSupportedMimeTypes(ppropvarmimetypearray: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFGetSupportedSchemes(ppropvarschemearray: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetSystemId(ppid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetSystemTime() -> i64;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetTimerPeriodicity(periodicity: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetTopoNodeCurrentType(pnode: ::windows::runtime::RawPtr, dwstreamindex: u32, foutput: super::super::Foundation::BOOL, pptype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetUncompressedVideoFormat(pvideoformat: *const MFVIDEOFORMAT) -> u32;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetWorkQueueMMCSSClass(dwworkqueueid: u32, pwszclass: super::super::Foundation::PWSTR, pcchclass: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetWorkQueueMMCSSPriority(dwworkqueueid: u32, lpriority: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetWorkQueueMMCSSTaskId(dwworkqueueid: u32, pdwtaskid: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFHeapAlloc(nsize: usize, dwflags: u32, pszfile: super::super::Foundation::PSTR, line: i32, eat: EAllocationType) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFHeapFree(pv: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitAMMediaTypeFromMFMediaType(pmftype: ::windows::runtime::RawPtr, guidformatblocktype: ::windows::runtime::GUID, pamtype: *mut ::core::mem::ManuallyDrop<super::DirectShow::AM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFInitAttributesFromBlob(pattributes: ::windows::runtime::RawPtr, pbuf: *const u8, cbbufsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromAMMediaType(pmftype: ::windows::runtime::RawPtr, pamtype: *const ::core::mem::ManuallyDrop<super::DirectShow::AM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitMediaTypeFromMFVideoFormat(pmftype: ::windows::runtime::RawPtr, pmfvf: *const MFVIDEOFORMAT, cbbufsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromMPEG1VideoInfo(pmftype: ::windows::runtime::RawPtr, pmp1vi: *const super::DirectShow::MPEG1VIDEOINFO, cbbufsize: u32, psubtype: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromMPEG2VideoInfo(pmftype: ::windows::runtime::RawPtr, pmp2vi: *const super::DirectShow::MPEG2VIDEOINFO, cbbufsize: u32, psubtype: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromVideoInfoHeader(pmftype: ::windows::runtime::RawPtr, pvih: *const super::DirectShow::VIDEOINFOHEADER, cbbufsize: u32, psubtype: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromVideoInfoHeader2(pmftype: ::windows::runtime::RawPtr, pvih2: *const super::DirectShow::VIDEOINFOHEADER2, cbbufsize: u32, psubtype: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Media_Audio`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFInitMediaTypeFromWaveFormatEx(pmftype: ::windows::runtime::RawPtr, pwaveformat: *const super::Audio::WAVEFORMATEX, cbbufsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitVideoFormat(pvideoformat: *const MFVIDEOFORMAT, r#type: MFStandardVideoFormat) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitVideoFormat_RGB(pvideoformat: *const MFVIDEOFORMAT, dwwidth: u32, dwheight: u32, d3dfmt: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFInvokeCallback(pasyncresult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsContentProtectionDeviceSupported(protectionsystemid: *const ::windows::runtime::GUID, issupported: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsFormatYUV(format: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsVirtualCameraTypeSupported(r#type: __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0001, supported: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFLoadSignedLibrary(pszname: super::super::Foundation::PWSTR, pplib: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFLockDXGIDeviceManager(presettoken: *mut u32, ppmanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFLockPlatform() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFLockSharedWorkQueue(wszclass: super::super::Foundation::PWSTR, basepriority: i32, pdwtaskid: *mut u32, pid: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFLockWorkQueue(dwworkqueue: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub fn MFMapDX9FormatToDXGIFormat(dx9: u32) -> super::super::Graphics::Dxgi::Common::DXGI_FORMAT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub fn MFMapDXGIFormatToDX9Format(dx11: super::super::Graphics::Dxgi::Common::DXGI_FORMAT) -> u32;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFPCreateMediaPlayer(pwszurl: super::super::Foundation::PWSTR, fstartplayback: super::super::Foundation::BOOL, creationoptions: MFP_CREATION_OPTIONS, pcallback: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, ppmediaplayer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFPutWaitingWorkItem(hevent: super::super::Foundation::HANDLE, priority: i32, presult: ::windows::runtime::RawPtr, pkey: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFPutWorkItem(dwqueue: u32, pcallback: ::windows::runtime::RawPtr, pstate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFPutWorkItem2(dwqueue: u32, priority: i32, pcallback: ::windows::runtime::RawPtr, pstate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFPutWorkItemEx(dwqueue: u32, presult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFPutWorkItemEx2(dwqueue: u32, priority: i32, presult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFRegisterLocalByteStreamHandler(szfileextension: super::super::Foundation::PWSTR, szmimetype: super::super::Foundation::PWSTR, pactivate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFRegisterLocalSchemeHandler(szscheme: super::super::Foundation::PWSTR, pactivate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFRegisterPlatformWithMMCSS(wszclass: super::super::Foundation::PWSTR, pdwtaskid: *mut u32, lpriority: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFRemovePeriodicCallback(dwkey: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFRequireProtectedEnvironment(ppresentationdescriptor: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFScheduleWorkItem(pcallback: ::windows::runtime::RawPtr, pstate: ::windows::runtime::RawPtr, timeout: i64, pkey: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFScheduleWorkItemEx(presult: ::windows::runtime::RawPtr, timeout: i64, pkey: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFSerializeAttributesToStream(pattr: ::windows::runtime::RawPtr, dwoptions: u32, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFSerializePresentationDescriptor(ppd: ::windows::runtime::RawPtr, pcbdata: *mut u32, ppbdata: *mut *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFShutdown() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFShutdownObject(punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFSplitSample(psample: ::windows::runtime::RawPtr, poutputsamples: *mut ::windows::runtime::RawPtr, dwoutputsamplemaxcount: u32, pdwoutputsamplecount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFStartup(version: u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTEnum(guidcategory: ::windows::runtime::GUID, flags: u32, pinputtype: *const MFT_REGISTER_TYPE_INFO, poutputtype: *const MFT_REGISTER_TYPE_INFO, pattributes: ::windows::runtime::RawPtr, ppclsidmft: *mut *mut ::windows::runtime::GUID, pcmfts: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTEnum2(guidcategory: ::windows::runtime::GUID, flags: u32, pinputtype: *const MFT_REGISTER_TYPE_INFO, poutputtype: *const MFT_REGISTER_TYPE_INFO, pattributes: ::windows::runtime::RawPtr, pppmftactivate: *mut *mut ::windows::runtime::RawPtr, pnummftactivate: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTEnumEx(guidcategory: ::windows::runtime::GUID, flags: u32, pinputtype: *const MFT_REGISTER_TYPE_INFO, poutputtype: *const MFT_REGISTER_TYPE_INFO, pppmftactivate: *mut *mut ::windows::runtime::RawPtr, pnummftactivate: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFTGetInfo(clsidmft: ::windows::runtime::GUID, pszname: *mut super::super::Foundation::PWSTR, ppinputtypes: *mut *mut MFT_REGISTER_TYPE_INFO, pcinputtypes: *mut u32, ppoutputtypes: *mut *mut MFT_REGISTER_TYPE_INFO, pcoutputtypes: *mut u32, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFTRegister(clsidmft: ::windows::runtime::GUID, guidcategory: ::windows::runtime::GUID, pszname: super::super::Foundation::PWSTR, flags: u32, cinputtypes: u32, pinputtypes: *const MFT_REGISTER_TYPE_INFO, coutputtypes: u32, poutputtypes: *const MFT_REGISTER_TYPE_INFO, pattributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn MFTRegisterLocal(pclassfactory: ::windows::runtime::RawPtr, guidcategory: *const ::windows::runtime::GUID, pszname: super::super::Foundation::PWSTR, flags: u32, cinputtypes: u32, pinputtypes: *const MFT_REGISTER_TYPE_INFO, coutputtypes: u32, poutputtypes: *const MFT_REGISTER_TYPE_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFTRegisterLocalByCLSID(clisdmft: *const ::windows::runtime::GUID, guidcategory: *const ::windows::runtime::GUID, pszname: super::super::Foundation::PWSTR, flags: u32, cinputtypes: u32, pinputtypes: *const MFT_REGISTER_TYPE_INFO, coutputtypes: u32, poutputtypes: *const MFT_REGISTER_TYPE_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTUnregister(clsidmft: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFTUnregisterLocal(pclassfactory: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTUnregisterLocalByCLSID(clsidmft: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTranscodeGetAudioOutputAvailableTypes(guidsubtype: *const ::windows::runtime::GUID, dwmftflags: u32, pcodecconfig: ::windows::runtime::RawPtr, ppavailabletypes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnlockDXGIDeviceManager() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnlockPlatform() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnlockWorkQueue(dwworkqueue: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnregisterPlatformFromMMCSS() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnwrapMediaType(pwrap: ::windows::runtime::RawPtr, pporig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFValidateMediaTypeSize(formattype: ::windows::runtime::GUID, pblock: *const u8, cbsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFWrapMediaType(porig: ::windows::runtime::RawPtr, majortype: *const ::windows::runtime::GUID, subtype: *const ::windows::runtime::GUID, ppwrap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFllMulDiv(a: i64, b: i64, c: i64, d: i64) -> i64;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OPMGetVideoOutputForTarget(padapterluid: *const super::super::Foundation::LUID, vidpntarget: u32, vos: OPM_VIDEO_OUTPUT_SEMANTICS, ppopmvideooutput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OPMGetVideoOutputsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, vos: OPM_VIDEO_OUTPUT_SEMANTICS, pulnumvideooutputs: *mut u32, pppopmvideooutputarray: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn OPMGetVideoOutputsFromIDirect3DDevice9Object(pdirect3ddevice9: ::windows::runtime::RawPtr, vos: OPM_VIDEO_OUTPUT_SEMANTICS, pulnumvideooutputs: *mut u32, pppopmvideooutputarray: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn OPMXboxEnableHDCP(hdcptype: OPM_HDCP_TYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn OPMXboxGetHDCPStatus(phdcpstatus: *mut OPM_HDCP_STATUS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn OPMXboxGetHDCPStatusAndType(phdcpstatus: *mut OPM_HDCP_STATUS, phdcptype: *mut OPM_HDCP_TYPE) -> ::windows::runtime::HRESULT;
}
