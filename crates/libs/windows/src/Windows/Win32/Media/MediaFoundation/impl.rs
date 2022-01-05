pub trait IAdvancedMediaCaptureImpl: Sized {
    fn GetAdvancedMediaCaptureSettings();
}
pub trait IAdvancedMediaCaptureInitializationSettingsImpl: Sized {
    fn SetDirectxDeviceManager();
}
pub trait IAdvancedMediaCaptureSettingsImpl: Sized {
    fn GetDirectxDeviceManager();
}
pub trait IAudioSourceProviderImpl: Sized {
    fn ProvideInput();
}
pub trait IClusterDetectorImpl: Sized {
    fn Initialize();
    fn Detect();
}
pub trait ICodecAPIImpl: Sized {
    fn IsSupported();
    fn IsModifiable();
    fn GetParameterRange();
    fn GetParameterValues();
    fn GetDefaultValue();
    fn GetValue();
    fn SetValue();
    fn RegisterForEvent();
    fn UnregisterForEvent();
    fn SetAllDefaults();
    fn SetValueWithNotify();
    fn SetAllDefaultsWithNotify();
    fn GetAllSettings();
    fn SetAllSettings();
    fn SetAllSettingsWithNotify();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoDecodeCommandListImpl: Sized + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn Close();
    fn Reset();
    fn ClearState();
    fn ResourceBarrier();
    fn DiscardResource();
    fn BeginQuery();
    fn EndQuery();
    fn ResolveQueryData();
    fn SetPredication();
    fn SetMarker();
    fn BeginEvent();
    fn EndEvent();
    fn DecodeFrame();
    fn WriteBufferImmediate();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoDecodeCommandList1Impl: Sized + ID3D12VideoDecodeCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn DecodeFrame1();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoDecodeCommandList2Impl: Sized + ID3D12VideoDecodeCommandList1Impl + ID3D12VideoDecodeCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn SetProtectedResourceSession();
    fn InitializeExtensionCommand();
    fn ExecuteExtensionCommand();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoDecoderImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoDecoder1Impl: Sized + ID3D12VideoDecoderImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetProtectedResourceSession();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoDecoderHeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoDecoderHeap1Impl: Sized + ID3D12VideoDecoderHeapImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetProtectedResourceSession();
}
pub trait ID3D12VideoDeviceImpl: Sized {
    fn CheckFeatureSupport();
    fn CreateVideoDecoder();
    fn CreateVideoDecoderHeap();
    fn CreateVideoProcessor();
}
pub trait ID3D12VideoDevice1Impl: Sized + ID3D12VideoDeviceImpl {
    fn CreateVideoMotionEstimator();
    fn CreateVideoMotionVectorHeap();
}
pub trait ID3D12VideoDevice2Impl: Sized + ID3D12VideoDevice1Impl + ID3D12VideoDeviceImpl {
    fn CreateVideoDecoder1();
    fn CreateVideoDecoderHeap1();
    fn CreateVideoProcessor1();
    fn CreateVideoExtensionCommand();
    fn ExecuteExtensionCommand();
}
pub trait ID3D12VideoDevice3Impl: Sized + ID3D12VideoDevice2Impl + ID3D12VideoDevice1Impl + ID3D12VideoDeviceImpl {
    fn CreateVideoEncoder();
    fn CreateVideoEncoderHeap();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoEncodeCommandListImpl: Sized + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn Close();
    fn Reset();
    fn ClearState();
    fn ResourceBarrier();
    fn DiscardResource();
    fn BeginQuery();
    fn EndQuery();
    fn ResolveQueryData();
    fn SetPredication();
    fn SetMarker();
    fn BeginEvent();
    fn EndEvent();
    fn EstimateMotion();
    fn ResolveMotionVectorHeap();
    fn WriteBufferImmediate();
    fn SetProtectedResourceSession();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoEncodeCommandList1Impl: Sized + ID3D12VideoEncodeCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn InitializeExtensionCommand();
    fn ExecuteExtensionCommand();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoEncodeCommandList2Impl: Sized + ID3D12VideoEncodeCommandList1Impl + ID3D12VideoEncodeCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn EncodeFrame();
    fn ResolveEncoderOutputMetadata();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoEncoderImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetNodeMask();
    fn GetEncoderFlags();
    fn GetCodec();
    fn GetCodecProfile();
    fn GetCodecConfiguration();
    fn GetInputFormat();
    fn GetMaxMotionEstimationPrecision();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoEncoderHeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetNodeMask();
    fn GetEncoderHeapFlags();
    fn GetCodec();
    fn GetCodecProfile();
    fn GetCodecLevel();
    fn GetResolutionListCount();
    fn GetResolutionList();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoExtensionCommandImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
    fn GetProtectedResourceSession();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoMotionEstimatorImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
    fn GetProtectedResourceSession();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoMotionVectorHeapImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetDesc();
    fn GetProtectedResourceSession();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoProcessCommandListImpl: Sized + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn Close();
    fn Reset();
    fn ClearState();
    fn ResourceBarrier();
    fn DiscardResource();
    fn BeginQuery();
    fn EndQuery();
    fn ResolveQueryData();
    fn SetPredication();
    fn SetMarker();
    fn BeginEvent();
    fn EndEvent();
    fn ProcessFrames();
    fn WriteBufferImmediate();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoProcessCommandList1Impl: Sized + ID3D12VideoProcessCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn ProcessFrames1();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoProcessCommandList2Impl: Sized + ID3D12VideoProcessCommandList1Impl + ID3D12VideoProcessCommandListImpl + ID3D12CommandListImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn SetProtectedResourceSession();
    fn InitializeExtensionCommand();
    fn ExecuteExtensionCommand();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoProcessorImpl: Sized + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetNodeMask();
    fn GetNumInputStreamDescs();
    fn GetInputStreamDescs();
    fn GetOutputStreamDesc();
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ID3D12VideoProcessor1Impl: Sized + ID3D12VideoProcessorImpl + ID3D12PageableImpl + ID3D12DeviceChildImpl + ID3D12ObjectImpl {
    fn GetProtectedResourceSession();
}
pub trait IDXVAHD_DeviceImpl: Sized {
    fn CreateVideoSurface();
    fn GetVideoProcessorDeviceCaps();
    fn GetVideoProcessorOutputFormats();
    fn GetVideoProcessorInputFormats();
    fn GetVideoProcessorCaps();
    fn GetVideoProcessorCustomRates();
    fn GetVideoProcessorFilterRange();
    fn CreateVideoProcessor();
}
pub trait IDXVAHD_VideoProcessorImpl: Sized {
    fn SetVideoProcessBltState();
    fn GetVideoProcessBltState();
    fn SetVideoProcessStreamState();
    fn GetVideoProcessStreamState();
    fn VideoProcessBltHD();
}
pub trait IDirect3D9ExOverlayExtensionImpl: Sized {
    fn CheckDeviceOverlayType();
}
pub trait IDirect3DAuthenticatedChannel9Impl: Sized {
    fn GetCertificateSize();
    fn GetCertificate();
    fn NegotiateKeyExchange();
    fn Query();
    fn Configure();
}
pub trait IDirect3DCryptoSession9Impl: Sized {
    fn GetCertificateSize();
    fn GetCertificate();
    fn NegotiateKeyExchange();
    fn EncryptionBlt();
    fn DecryptionBlt();
    fn GetSurfacePitch();
    fn StartSessionKeyRefresh();
    fn FinishSessionKeyRefresh();
    fn GetEncryptionBltKey();
}
pub trait IDirect3DDevice9VideoImpl: Sized {
    fn GetContentProtectionCaps();
    fn CreateAuthenticatedChannel();
    fn CreateCryptoSession();
}
pub trait IDirect3DDeviceManager9Impl: Sized {
    fn ResetDevice();
    fn OpenDeviceHandle();
    fn CloseDeviceHandle();
    fn TestDevice();
    fn LockDevice();
    fn UnlockDevice();
    fn GetVideoService();
}
pub trait IDirectXVideoAccelerationServiceImpl: Sized {
    fn CreateSurface();
}
pub trait IDirectXVideoDecoderImpl: Sized {
    fn GetVideoDecoderService();
    fn GetCreationParameters();
    fn GetBuffer();
    fn ReleaseBuffer();
    fn BeginFrame();
    fn EndFrame();
    fn Execute();
}
pub trait IDirectXVideoDecoderServiceImpl: Sized + IDirectXVideoAccelerationServiceImpl {
    fn GetDecoderDeviceGuids();
    fn GetDecoderRenderTargets();
    fn GetDecoderConfigurations();
    fn CreateVideoDecoder();
}
pub trait IDirectXVideoMemoryConfigurationImpl: Sized {
    fn GetAvailableSurfaceTypeByIndex();
    fn SetSurfaceType();
}
pub trait IDirectXVideoProcessorImpl: Sized {
    fn GetVideoProcessorService();
    fn GetCreationParameters();
    fn GetVideoProcessorCaps();
    fn GetProcAmpRange();
    fn GetFilterPropertyRange();
    fn VideoProcessBlt();
}
pub trait IDirectXVideoProcessorServiceImpl: Sized + IDirectXVideoAccelerationServiceImpl {
    fn RegisterVideoProcessorSoftwareDevice();
    fn GetVideoProcessorDeviceGuids();
    fn GetVideoProcessorRenderTargets();
    fn GetVideoProcessorSubStreamFormats();
    fn GetVideoProcessorCaps();
    fn GetProcAmpRange();
    fn GetFilterPropertyRange();
    fn CreateVideoProcessor();
}
pub trait IEVRFilterConfigImpl: Sized {
    fn SetNumberOfStreams();
    fn GetNumberOfStreams();
}
pub trait IEVRFilterConfigExImpl: Sized + IEVRFilterConfigImpl {
    fn SetConfigPrefs();
    fn GetConfigPrefs();
}
pub trait IEVRTrustedVideoPluginImpl: Sized {
    fn IsInTrustedVideoMode();
    fn CanConstrict();
    fn SetConstriction();
    fn DisableImageExport();
}
pub trait IEVRVideoStreamControlImpl: Sized {
    fn SetStreamActiveState();
    fn GetStreamActiveState();
}
pub trait IFileClientImpl: Sized {
    fn GetObjectDiskSize();
    fn Write();
    fn Read();
}
pub trait IFileIoImpl: Sized {
    fn Initialize();
    fn GetLength();
    fn SetLength();
    fn GetCurrentPosition();
    fn SetCurrentPosition();
    fn IsEndOfStream();
    fn Read();
    fn Write();
    fn Seek();
    fn Close();
}
pub trait IMF2DBufferImpl: Sized {
    fn Lock2D();
    fn Unlock2D();
    fn GetScanline0AndPitch();
    fn IsContiguousFormat();
    fn GetContiguousLength();
    fn ContiguousCopyTo();
    fn ContiguousCopyFrom();
}
pub trait IMF2DBuffer2Impl: Sized + IMF2DBufferImpl {
    fn Lock2DSize();
    fn Copy2DTo();
}
pub trait IMFASFContentInfoImpl: Sized {
    fn GetHeaderSize();
    fn ParseHeader();
    fn GenerateHeader();
    fn GetProfile();
    fn SetProfile();
    fn GeneratePresentationDescriptor();
    fn GetEncodingConfigurationPropertyStore();
}
pub trait IMFASFIndexerImpl: Sized {
    fn SetFlags();
    fn GetFlags();
    fn Initialize();
    fn GetIndexPosition();
    fn SetIndexByteStreams();
    fn GetIndexByteStreamCount();
    fn GetIndexStatus();
    fn SetIndexStatus();
    fn GetSeekPositionForValue();
    fn GenerateIndexEntries();
    fn CommitIndex();
    fn GetIndexWriteSpace();
    fn GetCompletedIndex();
}
pub trait IMFASFMultiplexerImpl: Sized {
    fn Initialize();
    fn SetFlags();
    fn GetFlags();
    fn ProcessSample();
    fn GetNextPacket();
    fn Flush();
    fn End();
    fn GetStatistics();
    fn SetSyncTolerance();
}
pub trait IMFASFMutualExclusionImpl: Sized {
    fn GetType();
    fn SetType();
    fn GetRecordCount();
    fn GetStreamsForRecord();
    fn AddStreamForRecord();
    fn RemoveStreamFromRecord();
    fn RemoveRecord();
    fn AddRecord();
    fn Clone();
}
pub trait IMFASFProfileImpl: Sized + IMFAttributesImpl {
    fn GetStreamCount();
    fn GetStream();
    fn GetStreamByNumber();
    fn SetStream();
    fn RemoveStream();
    fn CreateStream();
    fn GetMutualExclusionCount();
    fn GetMutualExclusion();
    fn AddMutualExclusion();
    fn RemoveMutualExclusion();
    fn CreateMutualExclusion();
    fn GetStreamPrioritization();
    fn AddStreamPrioritization();
    fn RemoveStreamPrioritization();
    fn CreateStreamPrioritization();
    fn Clone();
}
pub trait IMFASFSplitterImpl: Sized {
    fn Initialize();
    fn SetFlags();
    fn GetFlags();
    fn SelectStreams();
    fn GetSelectedStreams();
    fn ParseData();
    fn GetNextSample();
    fn Flush();
    fn GetLastSendTime();
}
pub trait IMFASFStreamConfigImpl: Sized + IMFAttributesImpl {
    fn GetStreamType();
    fn GetStreamNumber();
    fn SetStreamNumber();
    fn GetMediaType();
    fn SetMediaType();
    fn GetPayloadExtensionCount();
    fn GetPayloadExtension();
    fn AddPayloadExtension();
    fn RemoveAllPayloadExtensions();
    fn Clone();
}
pub trait IMFASFStreamPrioritizationImpl: Sized {
    fn GetStreamCount();
    fn GetStream();
    fn AddStream();
    fn RemoveStream();
    fn Clone();
}
pub trait IMFASFStreamSelectorImpl: Sized {
    fn GetStreamCount();
    fn GetOutputCount();
    fn GetOutputStreamCount();
    fn GetOutputStreamNumbers();
    fn GetOutputFromStream();
    fn GetOutputOverride();
    fn SetOutputOverride();
    fn GetOutputMutexCount();
    fn GetOutputMutex();
    fn SetOutputMutexSelection();
    fn GetBandwidthStepCount();
    fn GetBandwidthStep();
    fn BitrateToStepNumber();
    fn SetStreamSelectorFlags();
}
pub trait IMFActivateImpl: Sized + IMFAttributesImpl {
    fn ActivateObject();
    fn ShutdownObject();
    fn DetachObject();
}
pub trait IMFAsyncCallbackImpl: Sized {
    fn GetParameters();
    fn Invoke();
}
pub trait IMFAsyncCallbackLoggingImpl: Sized + IMFAsyncCallbackImpl {
    fn GetObjectPointer();
    fn GetObjectTag();
}
pub trait IMFAsyncResultImpl: Sized {
    fn GetState();
    fn GetStatus();
    fn SetStatus();
    fn GetObject();
    fn GetStateNoAddRef();
}
pub trait IMFAttributesImpl: Sized {
    fn GetItem();
    fn GetItemType();
    fn CompareItem();
    fn Compare();
    fn GetUINT32();
    fn GetUINT64();
    fn GetDouble();
    fn GetGUID();
    fn GetStringLength();
    fn GetString();
    fn GetAllocatedString();
    fn GetBlobSize();
    fn GetBlob();
    fn GetAllocatedBlob();
    fn GetUnknown();
    fn SetItem();
    fn DeleteItem();
    fn DeleteAllItems();
    fn SetUINT32();
    fn SetUINT64();
    fn SetDouble();
    fn SetGUID();
    fn SetString();
    fn SetBlob();
    fn SetUnknown();
    fn LockStore();
    fn UnlockStore();
    fn GetCount();
    fn GetItemByIndex();
    fn CopyAllItems();
}
pub trait IMFAudioMediaTypeImpl: Sized + IMFMediaTypeImpl + IMFAttributesImpl {
    fn GetAudioFormat();
}
pub trait IMFAudioPolicyImpl: Sized {
    fn SetGroupingParam();
    fn GetGroupingParam();
    fn SetDisplayName();
    fn GetDisplayName();
    fn SetIconPath();
    fn GetIconPath();
}
pub trait IMFAudioStreamVolumeImpl: Sized {
    fn GetChannelCount();
    fn SetChannelVolume();
    fn GetChannelVolume();
    fn SetAllVolumes();
    fn GetAllVolumes();
}
pub trait IMFBufferListNotifyImpl: Sized {
    fn OnAddSourceBuffer();
    fn OnRemoveSourceBuffer();
}
pub trait IMFByteStreamImpl: Sized {
    fn GetCapabilities();
    fn GetLength();
    fn SetLength();
    fn GetCurrentPosition();
    fn SetCurrentPosition();
    fn IsEndOfStream();
    fn Read();
    fn BeginRead();
    fn EndRead();
    fn Write();
    fn BeginWrite();
    fn EndWrite();
    fn Seek();
    fn Flush();
    fn Close();
}
pub trait IMFByteStreamBufferingImpl: Sized {
    fn SetBufferingParams();
    fn EnableBuffering();
    fn StopBuffering();
}
pub trait IMFByteStreamCacheControlImpl: Sized {
    fn StopBackgroundTransfer();
}
pub trait IMFByteStreamCacheControl2Impl: Sized + IMFByteStreamCacheControlImpl {
    fn GetByteRanges();
    fn SetCacheLimit();
    fn IsBackgroundTransferActive();
}
pub trait IMFByteStreamHandlerImpl: Sized {
    fn BeginCreateObject();
    fn EndCreateObject();
    fn CancelObjectCreation();
    fn GetMaxNumberOfBytesRequiredForResolution();
}
pub trait IMFByteStreamProxyClassFactoryImpl: Sized {
    fn CreateByteStreamProxy();
}
pub trait IMFByteStreamTimeSeekImpl: Sized {
    fn IsTimeSeekSupported();
    fn TimeSeek();
    fn GetTimeSeekResult();
}
pub trait IMFCameraOcclusionStateMonitorImpl: Sized {
    fn Start();
    fn Stop();
    fn GetSupportedStates();
}
pub trait IMFCameraOcclusionStateReportImpl: Sized {
    fn GetOcclusionState();
}
pub trait IMFCameraOcclusionStateReportCallbackImpl: Sized {
    fn OnOcclusionStateReport();
}
pub trait IMFCameraSyncObjectImpl: Sized {
    fn WaitOnSignal();
    fn Shutdown();
}
pub trait IMFCaptureEngineImpl: Sized {
    fn Initialize();
    fn StartPreview();
    fn StopPreview();
    fn StartRecord();
    fn StopRecord();
    fn TakePhoto();
    fn GetSink();
    fn GetSource();
}
pub trait IMFCaptureEngineClassFactoryImpl: Sized {
    fn CreateInstance();
}
pub trait IMFCaptureEngineOnEventCallbackImpl: Sized {
    fn OnEvent();
}
pub trait IMFCaptureEngineOnSampleCallbackImpl: Sized {
    fn OnSample();
}
pub trait IMFCaptureEngineOnSampleCallback2Impl: Sized + IMFCaptureEngineOnSampleCallbackImpl {
    fn OnSynchronizedEvent();
}
pub trait IMFCapturePhotoConfirmationImpl: Sized {
    fn SetPhotoConfirmationCallback();
    fn SetPixelFormat();
    fn GetPixelFormat();
}
pub trait IMFCapturePhotoSinkImpl: Sized + IMFCaptureSinkImpl {
    fn SetOutputFileName();
    fn SetSampleCallback();
    fn SetOutputByteStream();
}
pub trait IMFCapturePreviewSinkImpl: Sized + IMFCaptureSinkImpl {
    fn SetRenderHandle();
    fn SetRenderSurface();
    fn UpdateVideo();
    fn SetSampleCallback();
    fn GetMirrorState();
    fn SetMirrorState();
    fn GetRotation();
    fn SetRotation();
    fn SetCustomSink();
}
pub trait IMFCaptureRecordSinkImpl: Sized + IMFCaptureSinkImpl {
    fn SetOutputByteStream();
    fn SetOutputFileName();
    fn SetSampleCallback();
    fn SetCustomSink();
    fn GetRotation();
    fn SetRotation();
}
pub trait IMFCaptureSinkImpl: Sized {
    fn GetOutputMediaType();
    fn GetService();
    fn AddStream();
    fn Prepare();
    fn RemoveAllStreams();
}
pub trait IMFCaptureSink2Impl: Sized + IMFCaptureSinkImpl {
    fn SetOutputMediaType();
}
pub trait IMFCaptureSourceImpl: Sized {
    fn GetCaptureDeviceSource();
    fn GetCaptureDeviceActivate();
    fn GetService();
    fn AddEffect();
    fn RemoveEffect();
    fn RemoveAllEffects();
    fn GetAvailableDeviceMediaType();
    fn SetCurrentDeviceMediaType();
    fn GetCurrentDeviceMediaType();
    fn GetDeviceStreamCount();
    fn GetDeviceStreamCategory();
    fn GetMirrorState();
    fn SetMirrorState();
    fn GetStreamIndexFromFriendlyName();
}
pub trait IMFCdmSuspendNotifyImpl: Sized {
    fn Begin();
    fn End();
}
pub trait IMFClockImpl: Sized {
    fn GetClockCharacteristics();
    fn GetCorrelatedTime();
    fn GetContinuityKey();
    fn GetState();
    fn GetProperties();
}
pub trait IMFClockConsumerImpl: Sized {
    fn SetPresentationClock();
    fn GetPresentationClock();
}
pub trait IMFClockStateSinkImpl: Sized {
    fn OnClockStart();
    fn OnClockStop();
    fn OnClockPause();
    fn OnClockRestart();
    fn OnClockSetRate();
}
pub trait IMFCollectionImpl: Sized {
    fn GetElementCount();
    fn GetElement();
    fn AddElement();
    fn RemoveElement();
    fn InsertElementAt();
    fn RemoveAllElements();
}
pub trait IMFContentDecryptionModuleImpl: Sized {
    fn SetContentEnabler();
    fn GetSuspendNotify();
    fn SetPMPHostApp();
    fn CreateSession();
    fn SetServerCertificate();
    fn CreateTrustedInput();
    fn GetProtectionSystemIds();
}
pub trait IMFContentDecryptionModuleAccessImpl: Sized {
    fn CreateContentDecryptionModule();
    fn GetConfiguration();
    fn GetKeySystem();
}
pub trait IMFContentDecryptionModuleFactoryImpl: Sized {
    fn IsTypeSupported();
    fn CreateContentDecryptionModuleAccess();
}
pub trait IMFContentDecryptionModuleSessionImpl: Sized {
    fn GetSessionId();
    fn GetExpiration();
    fn GetKeyStatuses();
    fn Load();
    fn GenerateRequest();
    fn Update();
    fn Close();
    fn Remove();
}
pub trait IMFContentDecryptionModuleSessionCallbacksImpl: Sized {
    fn KeyMessage();
    fn KeyStatusChanged();
}
pub trait IMFContentDecryptorContextImpl: Sized {
    fn InitializeHardwareKey();
}
pub trait IMFContentEnablerImpl: Sized {
    fn GetEnableType();
    fn GetEnableURL();
    fn GetEnableData();
    fn IsAutomaticSupported();
    fn AutomaticEnable();
    fn MonitorEnable();
    fn Cancel();
}
pub trait IMFContentProtectionDeviceImpl: Sized {
    fn InvokeFunction();
    fn GetPrivateDataByteCount();
}
pub trait IMFContentProtectionManagerImpl: Sized {
    fn BeginEnableContent();
    fn EndEnableContent();
}
pub trait IMFD3D12SynchronizationObjectImpl: Sized {
    fn SignalEventOnFinalResourceRelease();
    fn Reset();
}
pub trait IMFD3D12SynchronizationObjectCommandsImpl: Sized {
    fn EnqueueResourceReady();
    fn EnqueueResourceReadyWait();
    fn SignalEventOnResourceReady();
    fn EnqueueResourceRelease();
}
pub trait IMFDLNASinkInitImpl: Sized {
    fn Initialize();
}
pub trait IMFDRMNetHelperImpl: Sized {
    fn ProcessLicenseRequest();
    fn GetChainedLicenseResponse();
}
pub trait IMFDXGIBufferImpl: Sized {
    fn GetResource();
    fn GetSubresourceIndex();
    fn GetUnknown();
    fn SetUnknown();
}
pub trait IMFDXGIDeviceManagerImpl: Sized {
    fn CloseDeviceHandle();
    fn GetVideoService();
    fn LockDevice();
    fn OpenDeviceHandle();
    fn ResetDevice();
    fn TestDevice();
    fn UnlockDevice();
}
pub trait IMFDXGIDeviceManagerSourceImpl: Sized {
    fn GetManager();
}
pub trait IMFDesiredSampleImpl: Sized {
    fn GetDesiredSampleTimeAndDuration();
    fn SetDesiredSampleTimeAndDuration();
    fn Clear();
}
pub trait IMFExtendedCameraControlImpl: Sized {
    fn GetCapabilities();
    fn SetFlags();
    fn GetFlags();
    fn LockPayload();
    fn UnlockPayload();
    fn CommitSettings();
}
pub trait IMFExtendedCameraControllerImpl: Sized {
    fn GetExtendedCameraControl();
}
pub trait IMFExtendedCameraIntrinsicModelImpl: Sized {
    fn GetModel();
    fn SetModel();
    fn GetDistortionModelType();
}
pub trait IMFExtendedCameraIntrinsicsImpl: Sized {
    fn InitializeFromBuffer();
    fn GetBufferSize();
    fn SerializeToBuffer();
    fn GetIntrinsicModelCount();
    fn GetIntrinsicModelByIndex();
    fn AddIntrinsicModel();
}
pub trait IMFExtendedCameraIntrinsicsDistortionModel6KTImpl: Sized {
    fn GetDistortionModel();
    fn SetDistortionModel();
}
pub trait IMFExtendedCameraIntrinsicsDistortionModelArcTanImpl: Sized {
    fn GetDistortionModel();
    fn SetDistortionModel();
}
pub trait IMFExtendedDRMTypeSupportImpl: Sized {
    fn IsTypeSupportedEx();
}
pub trait IMFFieldOfUseMFTUnlockImpl: Sized {
    fn Unlock();
}
pub trait IMFFinalizableMediaSinkImpl: Sized + IMFMediaSinkImpl {
    fn BeginFinalize();
    fn EndFinalize();
}
pub trait IMFGetServiceImpl: Sized {
    fn GetService();
}
pub trait IMFHDCPStatusImpl: Sized {
    fn Query();
    fn Set();
}
pub trait IMFHttpDownloadRequestImpl: Sized {
    fn AddHeader();
    fn BeginSendRequest();
    fn EndSendRequest();
    fn BeginReceiveResponse();
    fn EndReceiveResponse();
    fn BeginReadPayload();
    fn EndReadPayload();
    fn QueryHeader();
    fn GetURL();
    fn HasNullSourceOrigin();
    fn GetTimeSeekResult();
    fn GetHttpStatus();
    fn GetAtEndOfPayload();
    fn GetTotalLength();
    fn GetRangeEndOffset();
    fn Close();
}
pub trait IMFHttpDownloadSessionImpl: Sized {
    fn SetServer();
    fn CreateRequest();
    fn Close();
}
pub trait IMFHttpDownloadSessionProviderImpl: Sized {
    fn CreateHttpDownloadSession();
}
pub trait IMFImageSharingEngineImpl: Sized {
    fn SetSource();
    fn GetDevice();
    fn Shutdown();
}
pub trait IMFImageSharingEngineClassFactoryImpl: Sized {
    fn CreateInstanceFromUDN();
}
pub trait IMFInputTrustAuthorityImpl: Sized {
    fn GetDecrypter();
    fn RequestAccess();
    fn GetPolicy();
    fn BindAccess();
    fn UpdateAccess();
    fn Reset();
}
pub trait IMFLocalMFTRegistrationImpl: Sized {
    fn RegisterMFTs();
}
pub trait IMFMediaBufferImpl: Sized {
    fn Lock();
    fn Unlock();
    fn GetCurrentLength();
    fn SetCurrentLength();
    fn GetMaxLength();
}
pub trait IMFMediaEngineImpl: Sized {
    fn GetError();
    fn SetErrorCode();
    fn SetSourceElements();
    fn SetSource();
    fn GetCurrentSource();
    fn GetNetworkState();
    fn GetPreload();
    fn SetPreload();
    fn GetBuffered();
    fn Load();
    fn CanPlayType();
    fn GetReadyState();
    fn IsSeeking();
    fn GetCurrentTime();
    fn SetCurrentTime();
    fn GetStartTime();
    fn GetDuration();
    fn IsPaused();
    fn GetDefaultPlaybackRate();
    fn SetDefaultPlaybackRate();
    fn GetPlaybackRate();
    fn SetPlaybackRate();
    fn GetPlayed();
    fn GetSeekable();
    fn IsEnded();
    fn GetAutoPlay();
    fn SetAutoPlay();
    fn GetLoop();
    fn SetLoop();
    fn Play();
    fn Pause();
    fn GetMuted();
    fn SetMuted();
    fn GetVolume();
    fn SetVolume();
    fn HasVideo();
    fn HasAudio();
    fn GetNativeVideoSize();
    fn GetVideoAspectRatio();
    fn Shutdown();
    fn TransferVideoFrame();
    fn OnVideoStreamTick();
}
pub trait IMFMediaEngineAudioEndpointIdImpl: Sized {
    fn SetAudioEndpointId();
    fn GetAudioEndpointId();
}
pub trait IMFMediaEngineClassFactoryImpl: Sized {
    fn CreateInstance();
    fn CreateTimeRange();
    fn CreateError();
}
pub trait IMFMediaEngineClassFactory2Impl: Sized {
    fn CreateMediaKeys2();
}
pub trait IMFMediaEngineClassFactory3Impl: Sized {
    fn CreateMediaKeySystemAccess();
}
pub trait IMFMediaEngineClassFactory4Impl: Sized {
    fn CreateContentDecryptionModuleFactory();
}
pub trait IMFMediaEngineClassFactoryExImpl: Sized + IMFMediaEngineClassFactoryImpl {
    fn CreateMediaSourceExtension();
    fn CreateMediaKeys();
    fn IsTypeSupported();
}
pub trait IMFMediaEngineEMEImpl: Sized {
    fn Keys();
    fn SetMediaKeys();
}
pub trait IMFMediaEngineEMENotifyImpl: Sized {
    fn Encrypted();
    fn WaitingForKey();
}
pub trait IMFMediaEngineExImpl: Sized + IMFMediaEngineImpl {
    fn SetSourceFromByteStream();
    fn GetStatistics();
    fn UpdateVideoStream();
    fn GetBalance();
    fn SetBalance();
    fn IsPlaybackRateSupported();
    fn FrameStep();
    fn GetResourceCharacteristics();
    fn GetPresentationAttribute();
    fn GetNumberOfStreams();
    fn GetStreamAttribute();
    fn GetStreamSelection();
    fn SetStreamSelection();
    fn ApplyStreamSelections();
    fn IsProtected();
    fn InsertVideoEffect();
    fn InsertAudioEffect();
    fn RemoveAllEffects();
    fn SetTimelineMarkerTimer();
    fn GetTimelineMarkerTimer();
    fn CancelTimelineMarkerTimer();
    fn IsStereo3D();
    fn GetStereo3DFramePackingMode();
    fn SetStereo3DFramePackingMode();
    fn GetStereo3DRenderMode();
    fn SetStereo3DRenderMode();
    fn EnableWindowlessSwapchainMode();
    fn GetVideoSwapchainHandle();
    fn EnableHorizontalMirrorMode();
    fn GetAudioStreamCategory();
    fn SetAudioStreamCategory();
    fn GetAudioEndpointRole();
    fn SetAudioEndpointRole();
    fn GetRealTimeMode();
    fn SetRealTimeMode();
    fn SetCurrentTimeEx();
    fn EnableTimeUpdateTimer();
}
pub trait IMFMediaEngineExtensionImpl: Sized {
    fn CanPlayType();
    fn BeginCreateObject();
    fn CancelObjectCreation();
    fn EndCreateObject();
}
pub trait IMFMediaEngineNeedKeyNotifyImpl: Sized {
    fn NeedKey();
}
pub trait IMFMediaEngineNotifyImpl: Sized {
    fn EventNotify();
}
pub trait IMFMediaEngineOPMInfoImpl: Sized {
    fn GetOPMInfo();
}
pub trait IMFMediaEngineProtectedContentImpl: Sized {
    fn ShareResources();
    fn GetRequiredProtections();
    fn SetOPMWindow();
    fn TransferVideoFrame();
    fn SetContentProtectionManager();
    fn SetApplicationCertificate();
}
pub trait IMFMediaEngineSrcElementsImpl: Sized {
    fn GetLength();
    fn GetURL();
    fn GetType();
    fn GetMedia();
    fn AddElement();
    fn RemoveAllElements();
}
pub trait IMFMediaEngineSrcElementsExImpl: Sized + IMFMediaEngineSrcElementsImpl {
    fn AddElementEx();
    fn GetKeySystem();
}
pub trait IMFMediaEngineSupportsSourceTransferImpl: Sized {
    fn ShouldTransferSource();
    fn DetachMediaSource();
    fn AttachMediaSource();
}
pub trait IMFMediaEngineTransferSourceImpl: Sized {
    fn TransferSourceToMediaEngine();
}
pub trait IMFMediaEngineWebSupportImpl: Sized {
    fn ShouldDelayTheLoadEvent();
    fn ConnectWebAudio();
    fn DisconnectWebAudio();
}
pub trait IMFMediaErrorImpl: Sized {
    fn GetErrorCode();
    fn GetExtendedErrorCode();
    fn SetErrorCode();
    fn SetExtendedErrorCode();
}
pub trait IMFMediaEventImpl: Sized + IMFAttributesImpl {
    fn GetType();
    fn GetExtendedType();
    fn GetStatus();
    fn GetValue();
}
pub trait IMFMediaEventGeneratorImpl: Sized {
    fn GetEvent();
    fn BeginGetEvent();
    fn EndGetEvent();
    fn QueueEvent();
}
pub trait IMFMediaEventQueueImpl: Sized {
    fn GetEvent();
    fn BeginGetEvent();
    fn EndGetEvent();
    fn QueueEvent();
    fn QueueEventParamVar();
    fn QueueEventParamUnk();
    fn Shutdown();
}
pub trait IMFMediaKeySessionImpl: Sized {
    fn GetError();
    fn KeySystem();
    fn SessionId();
    fn Update();
    fn Close();
}
pub trait IMFMediaKeySession2Impl: Sized + IMFMediaKeySessionImpl {
    fn KeyStatuses();
    fn Load();
    fn GenerateRequest();
    fn Expiration();
    fn Remove();
    fn Shutdown();
}
pub trait IMFMediaKeySessionNotifyImpl: Sized {
    fn KeyMessage();
    fn KeyAdded();
    fn KeyError();
}
pub trait IMFMediaKeySessionNotify2Impl: Sized + IMFMediaKeySessionNotifyImpl {
    fn KeyMessage2();
    fn KeyStatusChange();
}
pub trait IMFMediaKeySystemAccessImpl: Sized {
    fn CreateMediaKeys();
    fn SupportedConfiguration();
    fn KeySystem();
}
pub trait IMFMediaKeysImpl: Sized {
    fn CreateSession();
    fn KeySystem();
    fn Shutdown();
    fn GetSuspendNotify();
}
pub trait IMFMediaKeys2Impl: Sized + IMFMediaKeysImpl {
    fn CreateSession2();
    fn SetServerCertificate();
    fn GetDOMException();
}
pub trait IMFMediaSessionImpl: Sized + IMFMediaEventGeneratorImpl {
    fn SetTopology();
    fn ClearTopologies();
    fn Start();
    fn Pause();
    fn Stop();
    fn Close();
    fn Shutdown();
    fn GetClock();
    fn GetSessionCapabilities();
    fn GetFullTopology();
}
pub trait IMFMediaSharingEngineImpl: Sized + IMFMediaEngineImpl {
    fn GetDevice();
}
pub trait IMFMediaSharingEngineClassFactoryImpl: Sized {
    fn CreateInstance();
}
pub trait IMFMediaSinkImpl: Sized {
    fn GetCharacteristics();
    fn AddStreamSink();
    fn RemoveStreamSink();
    fn GetStreamSinkCount();
    fn GetStreamSinkByIndex();
    fn GetStreamSinkById();
    fn SetPresentationClock();
    fn GetPresentationClock();
    fn Shutdown();
}
pub trait IMFMediaSinkPrerollImpl: Sized {
    fn NotifyPreroll();
}
pub trait IMFMediaSourceImpl: Sized + IMFMediaEventGeneratorImpl {
    fn GetCharacteristics();
    fn CreatePresentationDescriptor();
    fn Start();
    fn Stop();
    fn Pause();
    fn Shutdown();
}
pub trait IMFMediaSource2Impl: Sized + IMFMediaSourceExImpl + IMFMediaSourceImpl + IMFMediaEventGeneratorImpl {
    fn SetMediaType();
}
pub trait IMFMediaSourceExImpl: Sized + IMFMediaSourceImpl + IMFMediaEventGeneratorImpl {
    fn GetSourceAttributes();
    fn GetStreamAttributes();
    fn SetD3DManager();
}
pub trait IMFMediaSourceExtensionImpl: Sized {
    fn GetSourceBuffers();
    fn GetActiveSourceBuffers();
    fn GetReadyState();
    fn GetDuration();
    fn SetDuration();
    fn AddSourceBuffer();
    fn RemoveSourceBuffer();
    fn SetEndOfStream();
    fn IsTypeSupported();
    fn GetSourceBuffer();
}
pub trait IMFMediaSourceExtensionLiveSeekableRangeImpl: Sized {
    fn SetLiveSeekableRange();
    fn ClearLiveSeekableRange();
}
pub trait IMFMediaSourceExtensionNotifyImpl: Sized {
    fn OnSourceOpen();
    fn OnSourceEnded();
    fn OnSourceClose();
}
pub trait IMFMediaSourcePresentationProviderImpl: Sized {
    fn ForceEndOfPresentation();
}
pub trait IMFMediaSourceTopologyProviderImpl: Sized {
    fn GetMediaSourceTopology();
}
pub trait IMFMediaStreamImpl: Sized + IMFMediaEventGeneratorImpl {
    fn GetMediaSource();
    fn GetStreamDescriptor();
    fn RequestSample();
}
pub trait IMFMediaStream2Impl: Sized + IMFMediaStreamImpl + IMFMediaEventGeneratorImpl {
    fn SetStreamState();
    fn GetStreamState();
}
pub trait IMFMediaStreamSourceSampleRequestImpl: Sized {
    fn SetSample();
}
pub trait IMFMediaTimeRangeImpl: Sized {
    fn GetLength();
    fn GetStart();
    fn GetEnd();
    fn ContainsTime();
    fn AddRange();
    fn Clear();
}
pub trait IMFMediaTypeImpl: Sized + IMFAttributesImpl {
    fn GetMajorType();
    fn IsCompressedFormat();
    fn IsEqual();
    fn GetRepresentation();
    fn FreeRepresentation();
}
pub trait IMFMediaTypeHandlerImpl: Sized {
    fn IsMediaTypeSupported();
    fn GetMediaTypeCount();
    fn GetMediaTypeByIndex();
    fn SetCurrentMediaType();
    fn GetCurrentMediaType();
    fn GetMajorType();
}
pub trait IMFMetadataImpl: Sized {
    fn SetLanguage();
    fn GetLanguage();
    fn GetAllLanguages();
    fn SetProperty();
    fn GetProperty();
    fn DeleteProperty();
    fn GetAllPropertyNames();
}
pub trait IMFMetadataProviderImpl: Sized {
    fn GetMFMetadata();
}
pub trait IMFMuxStreamAttributesManagerImpl: Sized {
    fn GetStreamCount();
    fn GetAttributes();
}
pub trait IMFMuxStreamMediaTypeManagerImpl: Sized {
    fn GetStreamCount();
    fn GetMediaType();
    fn GetStreamConfigurationCount();
    fn AddStreamConfiguration();
    fn RemoveStreamConfiguration();
    fn GetStreamConfiguration();
}
pub trait IMFMuxStreamSampleManagerImpl: Sized {
    fn GetStreamCount();
    fn GetSample();
    fn GetStreamConfiguration();
}
pub trait IMFNetCredentialImpl: Sized {
    fn SetUser();
    fn SetPassword();
    fn GetUser();
    fn GetPassword();
    fn LoggedOnUser();
}
pub trait IMFNetCredentialCacheImpl: Sized {
    fn GetCredential();
    fn SetGood();
    fn SetUserOptions();
}
pub trait IMFNetCredentialManagerImpl: Sized {
    fn BeginGetCredentials();
    fn EndGetCredentials();
    fn SetGood();
}
pub trait IMFNetCrossOriginSupportImpl: Sized {
    fn GetCrossOriginPolicy();
    fn GetSourceOrigin();
    fn IsSameOrigin();
}
pub trait IMFNetProxyLocatorImpl: Sized {
    fn FindFirstProxy();
    fn FindNextProxy();
    fn RegisterProxyResult();
    fn GetCurrentProxy();
    fn Clone();
}
pub trait IMFNetProxyLocatorFactoryImpl: Sized {
    fn CreateProxyLocator();
}
pub trait IMFNetResourceFilterImpl: Sized {
    fn OnRedirect();
    fn OnSendingRequest();
}
pub trait IMFNetSchemeHandlerConfigImpl: Sized {
    fn GetNumberOfSupportedProtocols();
    fn GetSupportedProtocolType();
    fn ResetProtocolRolloverSettings();
}
pub trait IMFObjectReferenceStreamImpl: Sized {
    fn SaveReference();
    fn LoadReference();
}
pub trait IMFOutputPolicyImpl: Sized + IMFAttributesImpl {
    fn GenerateRequiredSchemas();
    fn GetOriginatorID();
    fn GetMinimumGRLVersion();
}
pub trait IMFOutputSchemaImpl: Sized + IMFAttributesImpl {
    fn GetSchemaType();
    fn GetConfigurationData();
    fn GetOriginatorID();
}
pub trait IMFOutputTrustAuthorityImpl: Sized {
    fn GetAction();
    fn SetPolicy();
}
pub trait IMFPMPClientImpl: Sized {
    fn SetPMPHost();
}
pub trait IMFPMPClientAppImpl: Sized {
    fn SetPMPHost();
}
pub trait IMFPMPHostImpl: Sized {
    fn LockProcess();
    fn UnlockProcess();
    fn CreateObjectByCLSID();
}
pub trait IMFPMPHostAppImpl: Sized {
    fn LockProcess();
    fn UnlockProcess();
    fn ActivateClassById();
}
pub trait IMFPMPServerImpl: Sized {
    fn LockProcess();
    fn UnlockProcess();
    fn CreateObjectByCLSID();
}
pub trait IMFPMediaItemImpl: Sized {
    fn GetMediaPlayer();
    fn GetURL();
    fn GetObject();
    fn GetUserData();
    fn SetUserData();
    fn GetStartStopPosition();
    fn SetStartStopPosition();
    fn HasVideo();
    fn HasAudio();
    fn IsProtected();
    fn GetDuration();
    fn GetNumberOfStreams();
    fn GetStreamSelection();
    fn SetStreamSelection();
    fn GetStreamAttribute();
    fn GetPresentationAttribute();
    fn GetCharacteristics();
    fn SetStreamSink();
    fn GetMetadata();
}
pub trait IMFPMediaPlayerImpl: Sized {
    fn Play();
    fn Pause();
    fn Stop();
    fn FrameStep();
    fn SetPosition();
    fn GetPosition();
    fn GetDuration();
    fn SetRate();
    fn GetRate();
    fn GetSupportedRates();
    fn GetState();
    fn CreateMediaItemFromURL();
    fn CreateMediaItemFromObject();
    fn SetMediaItem();
    fn ClearMediaItem();
    fn GetMediaItem();
    fn GetVolume();
    fn SetVolume();
    fn GetBalance();
    fn SetBalance();
    fn GetMute();
    fn SetMute();
    fn GetNativeVideoSize();
    fn GetIdealVideoSize();
    fn SetVideoSourceRect();
    fn GetVideoSourceRect();
    fn SetAspectRatioMode();
    fn GetAspectRatioMode();
    fn GetVideoWindow();
    fn UpdateVideo();
    fn SetBorderColor();
    fn GetBorderColor();
    fn InsertEffect();
    fn RemoveEffect();
    fn RemoveAllEffects();
    fn Shutdown();
}
pub trait IMFPMediaPlayerCallbackImpl: Sized {
    fn OnMediaPlayerEvent();
}
pub trait IMFPluginControlImpl: Sized {
    fn GetPreferredClsid();
    fn GetPreferredClsidByIndex();
    fn SetPreferredClsid();
    fn IsDisabled();
    fn GetDisabledByIndex();
    fn SetDisabled();
}
pub trait IMFPluginControl2Impl: Sized + IMFPluginControlImpl {
    fn SetPolicy();
}
pub trait IMFPresentationClockImpl: Sized + IMFClockImpl {
    fn SetTimeSource();
    fn GetTimeSource();
    fn GetTime();
    fn AddClockStateSink();
    fn RemoveClockStateSink();
    fn Start();
    fn Stop();
    fn Pause();
}
pub trait IMFPresentationDescriptorImpl: Sized + IMFAttributesImpl {
    fn GetStreamDescriptorCount();
    fn GetStreamDescriptorByIndex();
    fn SelectStream();
    fn DeselectStream();
    fn Clone();
}
pub trait IMFPresentationTimeSourceImpl: Sized + IMFClockImpl {
    fn GetUnderlyingClock();
}
pub trait IMFProtectedEnvironmentAccessImpl: Sized {
    fn Call();
    fn ReadGRL();
}
pub trait IMFQualityAdviseImpl: Sized {
    fn SetDropMode();
    fn SetQualityLevel();
    fn GetDropMode();
    fn GetQualityLevel();
    fn DropTime();
}
pub trait IMFQualityAdvise2Impl: Sized + IMFQualityAdviseImpl {
    fn NotifyQualityEvent();
}
pub trait IMFQualityAdviseLimitsImpl: Sized {
    fn GetMaximumDropMode();
    fn GetMinimumQualityLevel();
}
pub trait IMFQualityManagerImpl: Sized {
    fn NotifyTopology();
    fn NotifyPresentationClock();
    fn NotifyProcessInput();
    fn NotifyProcessOutput();
    fn NotifyQualityEvent();
    fn Shutdown();
}
pub trait IMFRateControlImpl: Sized {
    fn SetRate();
    fn GetRate();
}
pub trait IMFRateSupportImpl: Sized {
    fn GetSlowestRate();
    fn GetFastestRate();
    fn IsRateSupported();
}
pub trait IMFReadWriteClassFactoryImpl: Sized {
    fn CreateInstanceFromURL();
    fn CreateInstanceFromObject();
}
pub trait IMFRealTimeClientImpl: Sized {
    fn RegisterThreads();
    fn UnregisterThreads();
    fn SetWorkQueue();
}
pub trait IMFRealTimeClientExImpl: Sized {
    fn RegisterThreadsEx();
    fn UnregisterThreads();
    fn SetWorkQueueEx();
}
pub trait IMFRelativePanelReportImpl: Sized {
    fn GetRelativePanel();
}
pub trait IMFRelativePanelWatcherImpl: Sized + IMFShutdownImpl {
    fn BeginGetReport();
    fn EndGetReport();
    fn GetReport();
}
pub trait IMFRemoteAsyncCallbackImpl: Sized {
    fn Invoke();
}
pub trait IMFRemoteDesktopPluginImpl: Sized {
    fn UpdateTopology();
}
pub trait IMFRemoteProxyImpl: Sized {
    fn GetRemoteObject();
    fn GetRemoteHost();
}
pub trait IMFSAMIStyleImpl: Sized {
    fn GetStyleCount();
    fn GetStyles();
    fn SetSelectedStyle();
    fn GetSelectedStyle();
}
pub trait IMFSSLCertificateManagerImpl: Sized {
    fn GetClientCertificate();
    fn BeginGetClientCertificate();
    fn EndGetClientCertificate();
    fn GetCertificatePolicy();
    fn OnServerCertificate();
}
pub trait IMFSampleImpl: Sized + IMFAttributesImpl {
    fn GetSampleFlags();
    fn SetSampleFlags();
    fn GetSampleTime();
    fn SetSampleTime();
    fn GetSampleDuration();
    fn SetSampleDuration();
    fn GetBufferCount();
    fn GetBufferByIndex();
    fn ConvertToContiguousBuffer();
    fn AddBuffer();
    fn RemoveBufferByIndex();
    fn RemoveAllBuffers();
    fn GetTotalLength();
    fn CopyToBuffer();
}
pub trait IMFSampleAllocatorControlImpl: Sized {
    fn SetDefaultAllocator();
    fn GetAllocatorUsage();
}
pub trait IMFSampleGrabberSinkCallbackImpl: Sized + IMFClockStateSinkImpl {
    fn OnSetPresentationClock();
    fn OnProcessSample();
    fn OnShutdown();
}
pub trait IMFSampleGrabberSinkCallback2Impl: Sized + IMFSampleGrabberSinkCallbackImpl + IMFClockStateSinkImpl {
    fn OnProcessSampleEx();
}
pub trait IMFSampleOutputStreamImpl: Sized {
    fn BeginWriteSample();
    fn EndWriteSample();
    fn Close();
}
pub trait IMFSampleProtectionImpl: Sized {
    fn GetInputProtectionVersion();
    fn GetOutputProtectionVersion();
    fn GetProtectionCertificate();
    fn InitOutputProtection();
    fn InitInputProtection();
}
pub trait IMFSaveJobImpl: Sized {
    fn BeginSave();
    fn EndSave();
    fn CancelSave();
    fn GetProgress();
}
pub trait IMFSchemeHandlerImpl: Sized {
    fn BeginCreateObject();
    fn EndCreateObject();
    fn CancelObjectCreation();
}
pub trait IMFSecureBufferImpl: Sized {
    fn GetIdentifier();
}
pub trait IMFSecureChannelImpl: Sized {
    fn GetCertificate();
    fn SetupSession();
}
pub trait IMFSeekInfoImpl: Sized {
    fn GetNearestKeyFrames();
}
pub trait IMFSensorActivitiesReportImpl: Sized {
    fn GetCount();
    fn GetActivityReport();
    fn GetActivityReportByDeviceName();
}
pub trait IMFSensorActivitiesReportCallbackImpl: Sized {
    fn OnActivitiesReport();
}
pub trait IMFSensorActivityMonitorImpl: Sized {
    fn Start();
    fn Stop();
}
pub trait IMFSensorActivityReportImpl: Sized {
    fn GetFriendlyName();
    fn GetSymbolicLink();
    fn GetProcessCount();
    fn GetProcessActivity();
}
pub trait IMFSensorDeviceImpl: Sized {
    fn GetDeviceId();
    fn GetDeviceType();
    fn GetFlags();
    fn GetSymbolicLink();
    fn GetDeviceAttributes();
    fn GetStreamAttributesCount();
    fn GetStreamAttributes();
    fn SetSensorDeviceMode();
    fn GetSensorDeviceMode();
}
pub trait IMFSensorGroupImpl: Sized {
    fn GetSymbolicLink();
    fn GetFlags();
    fn GetSensorGroupAttributes();
    fn GetSensorDeviceCount();
    fn GetSensorDevice();
    fn SetDefaultSensorDeviceIndex();
    fn GetDefaultSensorDeviceIndex();
    fn CreateMediaSource();
}
pub trait IMFSensorProcessActivityImpl: Sized {
    fn GetProcessId();
    fn GetStreamingState();
    fn GetStreamingMode();
    fn GetReportTime();
}
pub trait IMFSensorProfileImpl: Sized {
    fn GetProfileId();
    fn AddProfileFilter();
    fn IsMediaTypeSupported();
    fn AddBlockedControl();
}
pub trait IMFSensorProfileCollectionImpl: Sized {
    fn GetProfileCount();
    fn GetProfile();
    fn AddProfile();
    fn FindProfile();
    fn RemoveProfileByIndex();
    fn RemoveProfile();
}
pub trait IMFSensorStreamImpl: Sized + IMFAttributesImpl {
    fn GetMediaTypeCount();
    fn GetMediaType();
    fn CloneSensorStream();
}
pub trait IMFSensorTransformFactoryImpl: Sized {
    fn GetFactoryAttributes();
    fn InitializeFactory();
    fn GetTransformCount();
    fn GetTransformInformation();
    fn CreateTransform();
}
pub trait IMFSequencerSourceImpl: Sized {
    fn AppendTopology();
    fn DeleteTopology();
    fn GetPresentationContext();
    fn UpdateTopology();
    fn UpdateTopologyFlags();
}
pub trait IMFSharingEngineClassFactoryImpl: Sized {
    fn CreateInstance();
}
pub trait IMFShutdownImpl: Sized {
    fn Shutdown();
    fn GetShutdownStatus();
}
pub trait IMFSignedLibraryImpl: Sized {
    fn GetProcedureAddress();
}
pub trait IMFSimpleAudioVolumeImpl: Sized {
    fn SetMasterVolume();
    fn GetMasterVolume();
    fn SetMute();
    fn GetMute();
}
pub trait IMFSinkWriterImpl: Sized {
    fn AddStream();
    fn SetInputMediaType();
    fn BeginWriting();
    fn WriteSample();
    fn SendStreamTick();
    fn PlaceMarker();
    fn NotifyEndOfSegment();
    fn Flush();
    fn Finalize();
    fn GetServiceForStream();
    fn GetStatistics();
}
pub trait IMFSinkWriterCallbackImpl: Sized {
    fn OnFinalize();
    fn OnMarker();
}
pub trait IMFSinkWriterCallback2Impl: Sized + IMFSinkWriterCallbackImpl {
    fn OnTransformChange();
    fn OnStreamError();
}
pub trait IMFSinkWriterEncoderConfigImpl: Sized {
    fn SetTargetMediaType();
    fn PlaceEncodingParameters();
}
pub trait IMFSinkWriterExImpl: Sized + IMFSinkWriterImpl {
    fn GetTransformForStream();
}
pub trait IMFSourceBufferImpl: Sized {
    fn GetUpdating();
    fn GetBuffered();
    fn GetTimeStampOffset();
    fn SetTimeStampOffset();
    fn GetAppendWindowStart();
    fn SetAppendWindowStart();
    fn GetAppendWindowEnd();
    fn SetAppendWindowEnd();
    fn Append();
    fn AppendByteStream();
    fn Abort();
    fn Remove();
}
pub trait IMFSourceBufferAppendModeImpl: Sized {
    fn GetAppendMode();
    fn SetAppendMode();
}
pub trait IMFSourceBufferListImpl: Sized {
    fn GetLength();
    fn GetSourceBuffer();
}
pub trait IMFSourceBufferNotifyImpl: Sized {
    fn OnUpdateStart();
    fn OnAbort();
    fn OnError();
    fn OnUpdate();
    fn OnUpdateEnd();
}
pub trait IMFSourceOpenMonitorImpl: Sized {
    fn OnSourceEvent();
}
pub trait IMFSourceReaderImpl: Sized {
    fn GetStreamSelection();
    fn SetStreamSelection();
    fn GetNativeMediaType();
    fn GetCurrentMediaType();
    fn SetCurrentMediaType();
    fn SetCurrentPosition();
    fn ReadSample();
    fn Flush();
    fn GetServiceForStream();
    fn GetPresentationAttribute();
}
pub trait IMFSourceReaderCallbackImpl: Sized {
    fn OnReadSample();
    fn OnFlush();
    fn OnEvent();
}
pub trait IMFSourceReaderCallback2Impl: Sized + IMFSourceReaderCallbackImpl {
    fn OnTransformChange();
    fn OnStreamError();
}
pub trait IMFSourceReaderExImpl: Sized + IMFSourceReaderImpl {
    fn SetNativeMediaType();
    fn AddTransformForStream();
    fn RemoveAllTransformsForStream();
    fn GetTransformForStream();
}
pub trait IMFSourceResolverImpl: Sized {
    fn CreateObjectFromURL();
    fn CreateObjectFromByteStream();
    fn BeginCreateObjectFromURL();
    fn EndCreateObjectFromURL();
    fn BeginCreateObjectFromByteStream();
    fn EndCreateObjectFromByteStream();
    fn CancelObjectCreation();
}
pub trait IMFSpatialAudioObjectBufferImpl: Sized + IMFMediaBufferImpl {
    fn SetID();
    fn GetID();
    fn SetType();
    fn GetType();
    fn GetMetadataItems();
}
pub trait IMFSpatialAudioSampleImpl: Sized + IMFSampleImpl + IMFAttributesImpl {
    fn GetObjectCount();
    fn AddSpatialAudioObject();
    fn GetSpatialAudioObjectByIndex();
}
pub trait IMFStreamDescriptorImpl: Sized + IMFAttributesImpl {
    fn GetStreamIdentifier();
    fn GetMediaTypeHandler();
}
pub trait IMFStreamSinkImpl: Sized + IMFMediaEventGeneratorImpl {
    fn GetMediaSink();
    fn GetIdentifier();
    fn GetMediaTypeHandler();
    fn ProcessSample();
    fn PlaceMarker();
    fn Flush();
}
pub trait IMFStreamingSinkConfigImpl: Sized {
    fn StartStreaming();
}
pub trait IMFSystemIdImpl: Sized {
    fn GetData();
    fn Setup();
}
pub trait IMFTimecodeTranslateImpl: Sized {
    fn BeginConvertTimecodeToHNS();
    fn EndConvertTimecodeToHNS();
    fn BeginConvertHNSToTimecode();
    fn EndConvertHNSToTimecode();
}
pub trait IMFTimedTextImpl: Sized {
    fn RegisterNotifications();
    fn SelectTrack();
    fn AddDataSource();
    fn AddDataSourceFromUrl();
    fn AddTrack();
    fn RemoveTrack();
    fn GetCueTimeOffset();
    fn SetCueTimeOffset();
    fn GetTracks();
    fn GetActiveTracks();
    fn GetTextTracks();
    fn GetMetadataTracks();
    fn SetInBandEnabled();
    fn IsInBandEnabled();
}
pub trait IMFTimedTextBinaryImpl: Sized {
    fn GetData();
}
pub trait IMFTimedTextBoutenImpl: Sized {
    fn GetBoutenType();
    fn GetBoutenColor();
    fn GetBoutenPosition();
}
pub trait IMFTimedTextCueImpl: Sized {
    fn GetId();
    fn GetOriginalId();
    fn GetCueKind();
    fn GetStartTime();
    fn GetDuration();
    fn GetTrackId();
    fn GetData();
    fn GetRegion();
    fn GetStyle();
    fn GetLineCount();
    fn GetLine();
}
pub trait IMFTimedTextCueListImpl: Sized {
    fn GetLength();
    fn GetCueByIndex();
    fn GetCueById();
    fn GetCueByOriginalId();
    fn AddTextCue();
    fn AddDataCue();
    fn RemoveCue();
}
pub trait IMFTimedTextFormattedTextImpl: Sized {
    fn GetText();
    fn GetSubformattingCount();
    fn GetSubformatting();
}
pub trait IMFTimedTextNotifyImpl: Sized {
    fn TrackAdded();
    fn TrackRemoved();
    fn TrackSelected();
    fn TrackReadyStateChanged();
    fn Error();
    fn Cue();
    fn Reset();
}
pub trait IMFTimedTextRegionImpl: Sized {
    fn GetName();
    fn GetPosition();
    fn GetExtent();
    fn GetBackgroundColor();
    fn GetWritingMode();
    fn GetDisplayAlignment();
    fn GetLineHeight();
    fn GetClipOverflow();
    fn GetPadding();
    fn GetWrap();
    fn GetZIndex();
    fn GetScrollMode();
}
pub trait IMFTimedTextRubyImpl: Sized {
    fn GetRubyText();
    fn GetRubyPosition();
    fn GetRubyAlign();
    fn GetRubyReserve();
}
pub trait IMFTimedTextStyleImpl: Sized {
    fn GetName();
    fn IsExternal();
    fn GetFontFamily();
    fn GetFontSize();
    fn GetColor();
    fn GetBackgroundColor();
    fn GetShowBackgroundAlways();
    fn GetFontStyle();
    fn GetBold();
    fn GetRightToLeft();
    fn GetTextAlignment();
    fn GetTextDecoration();
    fn GetTextOutline();
}
pub trait IMFTimedTextStyle2Impl: Sized {
    fn GetRuby();
    fn GetBouten();
    fn IsTextCombined();
    fn GetFontAngleInDegrees();
}
pub trait IMFTimedTextTrackImpl: Sized {
    fn GetId();
    fn GetLabel();
    fn SetLabel();
    fn GetLanguage();
    fn GetTrackKind();
    fn IsInBand();
    fn GetInBandMetadataTrackDispatchType();
    fn IsActive();
    fn GetErrorCode();
    fn GetExtendedErrorCode();
    fn GetDataFormat();
    fn GetReadyState();
    fn GetCueList();
}
pub trait IMFTimedTextTrackListImpl: Sized {
    fn GetLength();
    fn GetTrack();
    fn GetTrackById();
}
pub trait IMFTimerImpl: Sized {
    fn SetTimer();
    fn CancelTimer();
}
pub trait IMFTopoLoaderImpl: Sized {
    fn Load();
}
pub trait IMFTopologyImpl: Sized + IMFAttributesImpl {
    fn GetTopologyID();
    fn AddNode();
    fn RemoveNode();
    fn GetNodeCount();
    fn GetNode();
    fn Clear();
    fn CloneFrom();
    fn GetNodeByID();
    fn GetSourceNodeCollection();
    fn GetOutputNodeCollection();
}
pub trait IMFTopologyNodeImpl: Sized + IMFAttributesImpl {
    fn SetObject();
    fn GetObject();
    fn GetNodeType();
    fn GetTopoNodeID();
    fn SetTopoNodeID();
    fn GetInputCount();
    fn GetOutputCount();
    fn ConnectOutput();
    fn DisconnectOutput();
    fn GetInput();
    fn GetOutput();
    fn SetOutputPrefType();
    fn GetOutputPrefType();
    fn SetInputPrefType();
    fn GetInputPrefType();
    fn CloneFrom();
}
pub trait IMFTopologyNodeAttributeEditorImpl: Sized {
    fn UpdateNodeAttributes();
}
pub trait IMFTopologyServiceLookupImpl: Sized {
    fn LookupService();
}
pub trait IMFTopologyServiceLookupClientImpl: Sized {
    fn InitServicePointers();
    fn ReleaseServicePointers();
}
pub trait IMFTrackedSampleImpl: Sized {
    fn SetAllocator();
}
pub trait IMFTranscodeProfileImpl: Sized {
    fn SetAudioAttributes();
    fn GetAudioAttributes();
    fn SetVideoAttributes();
    fn GetVideoAttributes();
    fn SetContainerAttributes();
    fn GetContainerAttributes();
}
pub trait IMFTranscodeSinkInfoProviderImpl: Sized {
    fn SetOutputFile();
    fn SetOutputByteStream();
    fn SetProfile();
    fn GetSinkInfo();
}
pub trait IMFTransformImpl: Sized {
    fn GetStreamLimits();
    fn GetStreamCount();
    fn GetStreamIDs();
    fn GetInputStreamInfo();
    fn GetOutputStreamInfo();
    fn GetAttributes();
    fn GetInputStreamAttributes();
    fn GetOutputStreamAttributes();
    fn DeleteInputStream();
    fn AddInputStreams();
    fn GetInputAvailableType();
    fn GetOutputAvailableType();
    fn SetInputType();
    fn SetOutputType();
    fn GetInputCurrentType();
    fn GetOutputCurrentType();
    fn GetInputStatus();
    fn GetOutputStatus();
    fn SetOutputBounds();
    fn ProcessEvent();
    fn ProcessMessage();
    fn ProcessInput();
    fn ProcessOutput();
}
pub trait IMFTrustedInputImpl: Sized {
    fn GetInputTrustAuthority();
}
pub trait IMFTrustedOutputImpl: Sized {
    fn GetOutputTrustAuthorityCount();
    fn GetOutputTrustAuthorityByIndex();
    fn IsFinal();
}
pub trait IMFVideoCaptureSampleAllocatorImpl: Sized + IMFVideoSampleAllocatorImpl {
    fn InitializeCaptureSampleAllocator();
}
pub trait IMFVideoDeviceIDImpl: Sized {
    fn GetDeviceID();
}
pub trait IMFVideoDisplayControlImpl: Sized {
    fn GetNativeVideoSize();
    fn GetIdealVideoSize();
    fn SetVideoPosition();
    fn GetVideoPosition();
    fn SetAspectRatioMode();
    fn GetAspectRatioMode();
    fn SetVideoWindow();
    fn GetVideoWindow();
    fn RepaintVideo();
    fn GetCurrentImage();
    fn SetBorderColor();
    fn GetBorderColor();
    fn SetRenderingPrefs();
    fn GetRenderingPrefs();
    fn SetFullscreen();
    fn GetFullscreen();
}
pub trait IMFVideoMediaTypeImpl: Sized + IMFMediaTypeImpl + IMFAttributesImpl {
    fn GetVideoFormat();
    fn GetVideoRepresentation();
}
pub trait IMFVideoMixerBitmapImpl: Sized {
    fn SetAlphaBitmap();
    fn ClearAlphaBitmap();
    fn UpdateAlphaBitmapParameters();
    fn GetAlphaBitmapParameters();
}
pub trait IMFVideoMixerControlImpl: Sized {
    fn SetStreamZOrder();
    fn GetStreamZOrder();
    fn SetStreamOutputRect();
    fn GetStreamOutputRect();
}
pub trait IMFVideoMixerControl2Impl: Sized + IMFVideoMixerControlImpl {
    fn SetMixingPrefs();
    fn GetMixingPrefs();
}
pub trait IMFVideoPositionMapperImpl: Sized {
    fn MapOutputCoordinateToInputStream();
}
pub trait IMFVideoPresenterImpl: Sized + IMFClockStateSinkImpl {
    fn ProcessMessage();
    fn GetCurrentMediaType();
}
pub trait IMFVideoProcessorImpl: Sized {
    fn GetAvailableVideoProcessorModes();
    fn GetVideoProcessorCaps();
    fn GetVideoProcessorMode();
    fn SetVideoProcessorMode();
    fn GetProcAmpRange();
    fn GetProcAmpValues();
    fn SetProcAmpValues();
    fn GetFilteringRange();
    fn GetFilteringValue();
    fn SetFilteringValue();
    fn GetBackgroundColor();
    fn SetBackgroundColor();
}
pub trait IMFVideoProcessorControlImpl: Sized {
    fn SetBorderColor();
    fn SetSourceRectangle();
    fn SetDestinationRectangle();
    fn SetMirror();
    fn SetRotation();
    fn SetConstrictionSize();
}
pub trait IMFVideoProcessorControl2Impl: Sized + IMFVideoProcessorControlImpl {
    fn SetRotationOverride();
    fn EnableHardwareEffects();
    fn GetSupportedHardwareEffects();
}
pub trait IMFVideoProcessorControl3Impl: Sized + IMFVideoProcessorControl2Impl + IMFVideoProcessorControlImpl {
    fn GetNaturalOutputType();
    fn EnableSphericalVideoProcessing();
    fn SetSphericalVideoProperties();
    fn SetOutputDevice();
}
pub trait IMFVideoRendererImpl: Sized {
    fn InitializeRenderer();
}
pub trait IMFVideoRendererEffectControlImpl: Sized {
    fn OnAppServiceConnectionEstablished();
}
pub trait IMFVideoSampleAllocatorImpl: Sized {
    fn SetDirectXManager();
    fn UninitializeSampleAllocator();
    fn InitializeSampleAllocator();
    fn AllocateSample();
}
pub trait IMFVideoSampleAllocatorCallbackImpl: Sized {
    fn SetCallback();
    fn GetFreeSampleCount();
}
pub trait IMFVideoSampleAllocatorExImpl: Sized + IMFVideoSampleAllocatorImpl {
    fn InitializeSampleAllocatorEx();
}
pub trait IMFVideoSampleAllocatorNotifyImpl: Sized {
    fn NotifyRelease();
}
pub trait IMFVideoSampleAllocatorNotifyExImpl: Sized + IMFVideoSampleAllocatorNotifyImpl {
    fn NotifyPrune();
}
pub trait IMFVirtualCameraImpl: Sized + IMFAttributesImpl {
    fn AddDeviceSourceInfo();
    fn AddProperty();
    fn AddRegistryEntry();
    fn Start();
    fn Stop();
    fn Remove();
    fn GetMediaSource();
    fn SendCameraProperty();
    fn CreateSyncEvent();
    fn CreateSyncSemaphore();
    fn Shutdown();
}
pub trait IMFWorkQueueServicesImpl: Sized {
    fn BeginRegisterTopologyWorkQueuesWithMMCSS();
    fn EndRegisterTopologyWorkQueuesWithMMCSS();
    fn BeginUnregisterTopologyWorkQueuesWithMMCSS();
    fn EndUnregisterTopologyWorkQueuesWithMMCSS();
    fn GetTopologyWorkQueueMMCSSClass();
    fn GetTopologyWorkQueueMMCSSTaskId();
    fn BeginRegisterPlatformWorkQueueWithMMCSS();
    fn EndRegisterPlatformWorkQueueWithMMCSS();
    fn BeginUnregisterPlatformWorkQueueWithMMCSS();
    fn EndUnregisterPlatformWorkQueueWithMMCSS();
    fn GetPlaftormWorkQueueMMCSSClass();
    fn GetPlatformWorkQueueMMCSSTaskId();
}
pub trait IMFWorkQueueServicesExImpl: Sized + IMFWorkQueueServicesImpl {
    fn GetTopologyWorkQueueMMCSSPriority();
    fn BeginRegisterPlatformWorkQueueWithMMCSSEx();
    fn GetPlatformWorkQueueMMCSSPriority();
}
pub trait IOPMVideoOutputImpl: Sized {
    fn StartInitialization();
    fn FinishInitialization();
    fn GetInformation();
    fn COPPCompatibleGetInformation();
    fn Configure();
}
pub trait IPlayToControlImpl: Sized {
    fn Connect();
    fn Disconnect();
}
pub trait IPlayToControlWithCapabilitiesImpl: Sized + IPlayToControlImpl {
    fn GetCapabilities();
}
pub trait IPlayToSourceClassFactoryImpl: Sized {
    fn CreateInstance();
}
pub trait ITocImpl: Sized {
    fn SetDescriptor();
    fn GetDescriptor();
    fn SetDescription();
    fn GetDescription();
    fn SetContext();
    fn GetContext();
    fn GetEntryListCount();
    fn GetEntryListByIndex();
    fn AddEntryList();
    fn AddEntryListByIndex();
    fn RemoveEntryListByIndex();
}
pub trait ITocCollectionImpl: Sized {
    fn GetEntryCount();
    fn GetEntryByIndex();
    fn AddEntry();
    fn AddEntryByIndex();
    fn RemoveEntryByIndex();
}
pub trait ITocEntryImpl: Sized {
    fn SetTitle();
    fn GetTitle();
    fn SetDescriptor();
    fn GetDescriptor();
    fn SetSubEntries();
    fn GetSubEntries();
    fn SetDescriptionData();
    fn GetDescriptionData();
}
pub trait ITocEntryListImpl: Sized {
    fn GetEntryCount();
    fn GetEntryByIndex();
    fn AddEntry();
    fn AddEntryByIndex();
    fn RemoveEntryByIndex();
}
pub trait ITocParserImpl: Sized {
    fn Init();
    fn GetTocCount();
    fn GetTocByIndex();
    fn GetTocByType();
    fn AddToc();
    fn RemoveTocByIndex();
    fn RemoveTocByType();
    fn Commit();
}
pub trait IValidateBindingImpl: Sized {
    fn GetIdentifier();
}
pub trait IWMCodecLeakyBucketImpl: Sized {
    fn SetBufferSizeBits();
    fn GetBufferSizeBits();
    fn SetBufferFullnessBits();
    fn GetBufferFullnessBits();
}
pub trait IWMCodecOutputTimestampImpl: Sized {
    fn GetNextOutputTime();
}
pub trait IWMCodecPrivateDataImpl: Sized {
    fn SetPartialOutputType();
    fn GetPrivateData();
}
pub trait IWMCodecPropsImpl: Sized {
    fn GetFormatProp();
    fn GetCodecProp();
}
pub trait IWMCodecStringsImpl: Sized {
    fn GetName();
    fn GetDescription();
}
pub trait IWMColorConvPropsImpl: Sized {
    fn SetMode();
    fn SetFullCroppingParam();
}
pub trait IWMColorLegalizerPropsImpl: Sized {
    fn SetColorLegalizerQuality();
}
pub trait IWMFrameInterpPropsImpl: Sized {
    fn SetFrameRateIn();
    fn SetFrameRateOut();
    fn SetFrameInterpEnabled();
    fn SetComplexityLevel();
}
pub trait IWMInterlacePropsImpl: Sized {
    fn SetProcessType();
    fn SetInitInverseTeleCinePattern();
    fn SetLastFrame();
}
pub trait IWMResamplerPropsImpl: Sized {
    fn SetHalfFilterLength();
    fn SetUserChannelMtx();
}
pub trait IWMResizerPropsImpl: Sized {
    fn SetResizerQuality();
    fn SetInterlaceMode();
    fn SetClipRegion();
    fn SetFullCropRegion();
    fn GetFullCropRegion();
}
pub trait IWMSampleExtensionSupportImpl: Sized {
    fn SetUseSampleExtensions();
}
pub trait IWMValidateImpl: Sized {
    fn SetIdentifier();
}
pub trait IWMVideoDecoderHurryupImpl: Sized {
    fn SetHurryup();
    fn GetHurryup();
}
pub trait IWMVideoDecoderReconBufferImpl: Sized {
    fn GetReconstructedVideoFrameSize();
    fn GetReconstructedVideoFrame();
    fn SetReconstructedVideoFrame();
}
pub trait IWMVideoForceKeyFrameImpl: Sized {
    fn SetKeyFrame();
}
pub trait MFASYNCRESULTImpl: Sized + IMFAsyncResultImpl {}
