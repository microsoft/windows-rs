#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn CreateNamedPropertyStore();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn CreatePropertyStore();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn DXVA2CreateDirect3DDeviceManager9();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn DXVA2CreateVideoService();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn DXVAHD_CreateDevice();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAddPeriodicCallback();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAllocateSerialWorkQueue();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAllocateWorkQueue();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAllocateWorkQueueEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFAverageTimePerFrameToFrameRate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFBeginCreateFile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFBeginRegisterWorkQueueWithMMCSS();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFBeginRegisterWorkQueueWithMMCSSEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFBeginUnregisterWorkQueueWithMMCSS();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MFCalculateBitmapImageSize();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCalculateImageSize();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCancelCreateFile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCancelWorkItem();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCombineSamples();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCompareFullToPartialMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFConvertColorInfoFromDXVA();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFConvertColorInfoToDXVA();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFConvertFromFP16Array();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFConvertToFP16Array();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCopyImage();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreate2DMediaBuffer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreate3GPMediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAC3MediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateADTSMediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub fn MFCreateAMMediaTypeFromMFMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFContentInfo();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFIndexer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFIndexerByteStream();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFMediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateASFMediaSinkActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFMultiplexer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFProfile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFProfileFromPresentationDescriptor();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFSplitter();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFStreamSelector();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFStreamingMediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateASFStreamingMediaSinkActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAVIMediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAggregateSource();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAlignedMemoryBuffer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAsyncResult();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAttributes();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Media_Audio`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFCreateAudioMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAudioRenderer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateAudioRendererActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateCameraOcclusionStateMonitor();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateCollection();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateContentDecryptorContext();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateContentProtectionDevice();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateCredentialCache();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Direct3D12`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn MFCreateD3D12SynchronizationObject();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateDXGIDeviceManager();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateDXGISurfaceBuffer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateDXSurfaceBuffer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateDeviceSource();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateDeviceSourceActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn MFCreateEncryptedMediaExtensionsStoreActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateEventQueue();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateExtendedCameraIntrinsicModel();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateExtendedCameraIntrinsics();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateFMPEG4MediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateFile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Media_DxMediaObjects`*"]
    #[cfg(feature = "Win32_Media_DxMediaObjects")]
    pub fn MFCreateLegacyMediaBufferOnMFMediaBuffer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFCreateMFByteStreamOnStream();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMFByteStreamOnStreamEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMFByteStreamWrapper();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateMFVideoFormatFromMFMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMP3MediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMPEG4MediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaBufferFromMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaBufferWrapper();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFCreateMediaEvent();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateMediaExtensionActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaSession();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaTypeFromProperties();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMediaTypeFromRepresentation();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMemoryBuffer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMuxSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMuxStreamAttributes();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMuxStreamMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateMuxStreamSample();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateNetSchemePlugin();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePMPMediaSession();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePMPServer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePresentationClock();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePresentationDescriptor();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePresentationDescriptorFromASFProfile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreatePropertiesFromMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateProtectedEnvironmentAccess();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn MFCreateProxyLocator();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateRelativePanelWatcher();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateRemoteDesktopPlugin();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSample();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSampleCopierMFT();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSampleGrabberSinkActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSensorActivityMonitor();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateSensorGroup();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateSensorProfile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSensorProfileCollection();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSensorStream();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFCreateSequencerSegmentOffset();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSequencerSource();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSimpleTypeHandler();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSinkWriterFromMediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateSinkWriterFromURL();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSourceReaderFromByteStream();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSourceReaderFromMediaSource();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateSourceReaderFromURL();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSourceResolver();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateStandardQualityManager();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateStreamDescriptor();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFCreateStreamOnMFByteStream();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateStreamOnMFByteStreamEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateSystemTimeSource();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTempFile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTopoLoader();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTopology();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTopologyNode();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTrackedSample();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTranscodeProfile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTranscodeSinkActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateTranscodeTopology();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTranscodeTopologyFromByteStream();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateTransformActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateVideoMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn MFCreateVideoMediaTypeFromBitMapInfoHeader();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn MFCreateVideoMediaTypeFromBitMapInfoHeaderEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoMediaTypeFromSubtype();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoMixer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoMixerAndPresenter();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoPresenter();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoRenderer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateVideoRendererActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoSampleAllocator();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoSampleAllocatorEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateVideoSampleFromSurface();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFCreateVirtualCamera();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateWAVEMediaSink();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFCreateWICBitmapBuffer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn MFCreateWMAEncoderActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub fn MFCreateWMVEncoderActivate();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Media_Audio`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFCreateWaveFormatExFromMFMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFDeserializeAttributesFromStream();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFDeserializePresentationDescriptor();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFEndCreateFile();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFEndRegisterWorkQueueWithMMCSS();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFEndUnregisterWorkQueueWithMMCSS();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFEnumDeviceSources();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFFrameRateToAverageTimePerFrame();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetAttributesAsBlob();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetAttributesAsBlobSize();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetContentProtectionSystemCLSID();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetLocalId();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetMFTMerit();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetPlaneSize();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetPluginControl();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetService();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetStrideForBitmapInfoHeader();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFGetSupportedMimeTypes();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn MFGetSupportedSchemes();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetSystemId();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetSystemTime();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetTimerPeriodicity();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetTopoNodeCurrentType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetUncompressedVideoFormat();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFGetWorkQueueMMCSSClass();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetWorkQueueMMCSSPriority();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFGetWorkQueueMMCSSTaskId();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFHeapAlloc();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFHeapFree();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitAMMediaTypeFromMFMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFInitAttributesFromBlob();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromAMMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitMediaTypeFromMFVideoFormat();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromMPEG1VideoInfo();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromMPEG2VideoInfo();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromVideoInfoHeader();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Media_DirectShow`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_DirectShow"))]
    pub fn MFInitMediaTypeFromVideoInfoHeader2();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Media_Audio`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub fn MFInitMediaTypeFromWaveFormatEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitVideoFormat();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFInitVideoFormat_RGB();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFInvokeCallback();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsContentProtectionDeviceSupported();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsFormatYUV();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFIsVirtualCameraTypeSupported();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFLoadSignedLibrary();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFLockDXGIDeviceManager();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFLockPlatform();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFLockSharedWorkQueue();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFLockWorkQueue();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub fn MFMapDX9FormatToDXGIFormat();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub fn MFMapDXGIFormatToDX9Format();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFPCreateMediaPlayer();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFPutWaitingWorkItem();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFPutWorkItem();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFPutWorkItem2();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFPutWorkItemEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFPutWorkItemEx2();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFRegisterLocalByteStreamHandler();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFRegisterLocalSchemeHandler();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFRegisterPlatformWithMMCSS();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFRemovePeriodicCallback();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFRequireProtectedEnvironment();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFScheduleWorkItem();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFScheduleWorkItemEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFSerializeAttributesToStream();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFSerializePresentationDescriptor();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFShutdown();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFShutdownObject();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFSplitSample();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFStartup();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTEnum();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTEnum2();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTEnumEx();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFTGetInfo();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFTRegister();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn MFTRegisterLocal();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MFTRegisterLocalByCLSID();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTUnregister();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn MFTUnregisterLocal();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTUnregisterLocalByCLSID();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFTranscodeGetAudioOutputAvailableTypes();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnlockDXGIDeviceManager();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnlockPlatform();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnlockWorkQueue();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnregisterPlatformFromMMCSS();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFUnwrapMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFValidateMediaTypeSize();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFWrapMediaType();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn MFllMulDiv();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OPMGetVideoOutputForTarget();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OPMGetVideoOutputsFromHMONITOR();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn OPMGetVideoOutputsFromIDirect3DDevice9Object();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn OPMXboxEnableHDCP();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn OPMXboxGetHDCPStatus();
    #[doc = "*Required features: `Win32_Media_MediaFoundation`*"]
    pub fn OPMXboxGetHDCPStatusAndType();
}
