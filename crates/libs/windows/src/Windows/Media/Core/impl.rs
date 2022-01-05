#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptorImpl: Sized + IMediaStreamDescriptorImpl {
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptor2Impl: Sized + IMediaStreamDescriptorImpl {
    fn SetLeadingEncoderPadding(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn LeadingEncoderPadding(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetTrailingEncoderPadding(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn TrailingEncoderPadding(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptor3Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<AudioStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptorFactoryImpl: Sized {
    fn Create(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackImpl: Sized {
    fn OpenFailed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportInfo(&self) -> ::windows::core::Result<AudioTrackSupportInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackOpenFailedEventArgsImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackSupportInfoImpl: Sized {
    fn DecoderStatus(&self) -> ::windows::core::Result<MediaDecoderStatus>;
    fn Degradation(&self) -> ::windows::core::Result<AudioDecoderDegradation>;
    fn DegradationReason(&self) -> ::windows::core::Result<AudioDecoderDegradationReason>;
    fn MediaSourceStatus(&self) -> ::windows::core::Result<MediaSourceStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChapterCueImpl: Sized + IMediaCueImpl {
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecInfoImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<CodecKind>;
    fn Category(&self) -> ::windows::core::Result<CodecCategory>;
    fn Subtypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsTrusted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecQueryImpl: Sized {
    fn FindAllAsync(&self, kind: CodecKind, category: CodecCategory, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<CodecInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecSubtypesStaticsImpl: Sized {
    fn VideoFormatDV25(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDV50(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvh1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvhD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvsd(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvsl(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH263(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH264(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH265(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH264ES(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatHevc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatHevcES(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatM4S2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMjpg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP43(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP4S(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP4V(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMpeg2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatVP80(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatVP90(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMpg1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMss1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMss2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWvc1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormat420O(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAdts(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAlac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrNB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrWB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrWP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyAC3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyAC3Spdif(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyDDPlus(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDrm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDts(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatFlac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatFloat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMP3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMPeg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMsp1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatOpus(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatPcm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWmaSpdif(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioLossless(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioV8(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioV9(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataCueImpl: Sized + IMediaCueImpl {
    fn SetData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataCue2Impl: Sized + IDataCueImpl + IMediaCueImpl {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::PropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectedEventArgsImpl: Sized {
    fn ResultFrame(&self) -> ::windows::core::Result<FaceDetectionEffectFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectionEffectImpl: Sized + IMediaExtensionImpl {
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetDesiredDetectionInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DesiredDetectionInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn FaceDetected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFaceDetected(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectDefinitionImpl: Sized + IVideoEffectDefinitionImpl {
    fn SetDetectionMode(&self, value: FaceDetectionMode) -> ::windows::core::Result<()>;
    fn DetectionMode(&self) -> ::windows::core::Result<FaceDetectionMode>;
    fn SetSynchronousDetectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SynchronousDetectionEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn DetectedFaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHighDynamicRangeControlImpl: Sized {
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHighDynamicRangeOutputImpl: Sized {
    fn Certainty(&self) -> ::windows::core::Result<f64>;
    fn FrameControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Devices::Core::FrameController>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageCueImpl: Sized + IMediaCueImpl {
    fn Position(&self) -> ::windows::core::Result<TimedTextPoint>;
    fn SetPosition(&self, value: &TimedTextPoint) -> ::windows::core::Result<()>;
    fn Extent(&self) -> ::windows::core::Result<TimedTextSize>;
    fn SetExtent(&self, value: &TimedTextSize) -> ::windows::core::Result<()>;
    fn SetSoftwareBitmap(&self, value: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInitializeMediaStreamSourceRequestedEventArgsImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<MediaStreamSource>;
    fn RandomAccessStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLightFusionResultImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLightFusionStaticsImpl: Sized {
    fn SupportedBitmapPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn MaxSupportedFrameCount(&self) -> ::windows::core::Result<i32>;
    fn FuseAsync(&self, frameset: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBinderImpl: Sized {
    fn Binding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBinding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgsImpl: Sized {
    fn Canceled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaBinder(&self) -> ::windows::core::Result<MediaBinder>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
    fn SetUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetStream(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetStreamReference(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgs2Impl: Sized {
    fn SetAdaptiveMediaSource(&self, mediasource: &::core::option::Option<super::Streaming::Adaptive::AdaptiveMediaSource>) -> ::windows::core::Result<()>;
    fn SetStorageFile(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgs3Impl: Sized {
    fn SetDownloadOperation(&self, downloadoperation: &::core::option::Option<super::super::Networking::BackgroundTransfer::DownloadOperation>) -> ::windows::core::Result<()>;
}
pub trait IMediaCueImpl: Sized {
    fn SetStartTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCueEventArgsImpl: Sized {
    fn Cue(&self) -> ::windows::core::Result<IMediaCue>;
}
pub trait IMediaSourceImpl: Sized {}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource2Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl {
    fn OpenOperationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenOperationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn ExternalTimedTextSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<TimedTextSource>>;
    fn ExternalTimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<TimedMetadataTrack>>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource3Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl + IMediaSource2Impl {
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<MediaSourceState>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource4Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl + IMediaSource2Impl + IMediaSource3Impl {
    fn AdaptiveMediaSource(&self) -> ::windows::core::Result<super::Streaming::Adaptive::AdaptiveMediaSource>;
    fn MediaStreamSource(&self) -> ::windows::core::Result<MediaStreamSource>;
    fn MseStreamSource(&self) -> ::windows::core::Result<MseStreamSource>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSource5Impl: Sized {
    fn DownloadOperation(&self) -> ::windows::core::Result<super::super::Networking::BackgroundTransfer::DownloadOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceAppServiceConnectionImpl: Sized {
    fn InitializeMediaStreamSourceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInitializeMediaStreamSourceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceAppServiceConnectionFactoryImpl: Sized {
    fn Create(&self, appserviceconnection: &::core::option::Option<super::super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<MediaSourceAppServiceConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceErrorImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceOpenOperationCompletedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<MediaSourceError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStateChangedEventArgsImpl: Sized {
    fn OldState(&self) -> ::windows::core::Result<MediaSourceState>;
    fn NewState(&self) -> ::windows::core::Result<MediaSourceState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStaticsImpl: Sized {
    fn CreateFromAdaptiveMediaSource(&self, mediasource: &::core::option::Option<super::Streaming::Adaptive::AdaptiveMediaSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromMediaStreamSource(&self, mediasource: &::core::option::Option<MediaStreamSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromMseStreamSource(&self, mediasource: &::core::option::Option<MseStreamSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromIMediaSource(&self, mediasource: &::core::option::Option<IMediaSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStorageFile(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStream(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStreamReference(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<MediaSource>;
    fn CreateFromUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics2Impl: Sized {
    fn CreateFromMediaBinder(&self, binder: &::core::option::Option<MediaBinder>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics3Impl: Sized {
    fn CreateFromMediaFrameSource(&self, framesource: &::core::option::Option<super::Capture::Frames::MediaFrameSource>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics4Impl: Sized {
    fn CreateFromDownloadOperation(&self, downloadoperation: &::core::option::Option<super::super::Networking::BackgroundTransfer::DownloadOperation>) -> ::windows::core::Result<MediaSource>;
}
pub trait IMediaStreamDescriptorImpl: Sized {
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IMediaStreamDescriptor2Impl: Sized + IMediaStreamDescriptorImpl {
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleImpl: Sized {
    fn Processed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProcessed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<MediaStreamSamplePropertySet>;
    fn Protection(&self) -> ::windows::core::Result<MediaStreamSampleProtectionProperties>;
    fn SetDecodeTimestamp(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DecodeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetKeyFrame(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyFrame(&self) -> ::windows::core::Result<bool>;
    fn SetDiscontinuous(&self, value: bool) -> ::windows::core::Result<()>;
    fn Discontinuous(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSample2Impl: Sized {
    fn Direct3D11Surface(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleProtectionPropertiesImpl: Sized {
    fn SetKeyIdentifier(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetKeyIdentifier(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetInitializationVector(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetInitializationVector(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetSubSampleMapping(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetSubSampleMapping(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleStaticsImpl: Sized {
    fn CreateFromBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaStreamSample>;
    fn CreateFromStreamAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, count: u32, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaStreamSample>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleStatics2Impl: Sized {
    fn CreateFromDirect3D11Surface(&self, surface: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaStreamSample>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceImpl: Sized + IMediaSourceImpl {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Starting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Paused(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePaused(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SampleRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSampleRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SwitchStreamsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSwitchStreamsRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyError(&self, errorstatus: MediaStreamSourceErrorStatus) -> ::windows::core::Result<()>;
    fn AddStreamDescriptor(&self, descriptor: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<()>;
    fn SetMediaProtectionManager(&self, value: &::core::option::Option<super::Protection::MediaProtectionManager>) -> ::windows::core::Result<()>;
    fn MediaProtectionManager(&self) -> ::windows::core::Result<super::Protection::MediaProtectionManager>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetCanSeek(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn SetBufferTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BufferTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBufferedRange(&self, startoffset: &super::super::Foundation::TimeSpan, endoffset: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MusicProperties(&self) -> ::windows::core::Result<super::super::Storage::FileProperties::MusicProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<super::super::Storage::FileProperties::VideoProperties>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn AddProtectionKey(&self, streamdescriptor: &::core::option::Option<IMediaStreamDescriptor>, keyidentifier: &[<u8 as ::windows::core::DefaultType>::DefaultType], licensedata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource2Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SampleRendered(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSampleRendered(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource3Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SetMaxSupportedPlaybackRate(&self, value: &::core::option::Option<super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn MaxSupportedPlaybackRate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource4Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SetIsLive(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsLive(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceClosedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<MediaStreamSourceClosedRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceClosedRequestImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<MediaStreamSourceClosedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceFactoryImpl: Sized {
    fn CreateFromDescriptor(&self, descriptor: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<MediaStreamSource>;
    fn CreateFromDescriptors(&self, descriptor: &::core::option::Option<IMediaStreamDescriptor>, descriptor2: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<MediaStreamSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRenderedEventArgsImpl: Sized {
    fn SampleLag(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestImpl: Sized {
    fn StreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceSampleRequestDeferral>;
    fn SetSample(&self, value: &::core::option::Option<MediaStreamSample>) -> ::windows::core::Result<()>;
    fn Sample(&self) -> ::windows::core::Result<MediaStreamSample>;
    fn ReportSampleProgress(&self, progress: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<MediaStreamSourceSampleRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<MediaStreamSourceStartingRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingRequestImpl: Sized {
    fn StartPosition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceStartingRequestDeferral>;
    fn SetActualStartPosition(&self, position: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestImpl: Sized {
    fn OldStreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn NewStreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceSwitchStreamsRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<MediaStreamSourceSwitchStreamsRequest>;
}
pub trait IMediaTrackImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrackKind(&self) -> ::windows::core::Result<MediaTrackKind>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseSourceBufferImpl: Sized {
    fn UpdateStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UpdateEnded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Aborted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAborted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Mode(&self) -> ::windows::core::Result<MseAppendMode>;
    fn SetMode(&self, value: MseAppendMode) -> ::windows::core::Result<()>;
    fn IsUpdating(&self) -> ::windows::core::Result<bool>;
    fn Buffered(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MseTimeRange>>;
    fn TimestampOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTimestampOffset(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AppendWindowStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetAppendWindowStart(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AppendWindowEnd(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetAppendWindowEnd(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn AppendBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AppendStream(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn AppendStreamMaxSize(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, maxsize: u64) -> ::windows::core::Result<()>;
    fn Abort(&self) -> ::windows::core::Result<()>;
    fn Remove(&self, start: &super::super::Foundation::TimeSpan, end: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseSourceBufferListImpl: Sized {
    fn SourceBufferAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceBufferAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceBufferRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceBufferRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Buffers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MseSourceBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSourceImpl: Sized + IMediaSourceImpl {
    fn Opened(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Ended(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceBuffers(&self) -> ::windows::core::Result<MseSourceBufferList>;
    fn ActiveSourceBuffers(&self) -> ::windows::core::Result<MseSourceBufferList>;
    fn ReadyState(&self) -> ::windows::core::Result<MseReadyState>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetDuration(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn AddSourceBuffer(&self, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<MseSourceBuffer>;
    fn RemoveSourceBuffer(&self, buffer: &::core::option::Option<MseSourceBuffer>) -> ::windows::core::Result<()>;
    fn EndOfStream(&self, status: MseEndOfStreamStatus) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSource2Impl: Sized {
    fn LiveSeekableRange(&self) -> ::windows::core::Result<super::super::Foundation::IReference<MseTimeRange>>;
    fn SetLiveSeekableRange(&self, value: &::core::option::Option<super::super::Foundation::IReference<MseTimeRange>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSourceStaticsImpl: Sized {
    fn IsContentTypeSupported(&self, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneAnalysisEffectImpl: Sized + IMediaExtensionImpl {
    fn HighDynamicRangeAnalyzer(&self) -> ::windows::core::Result<HighDynamicRangeControl>;
    fn SetDesiredAnalysisInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DesiredAnalysisInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SceneAnalyzed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSceneAnalyzed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn FrameControlValues(&self) -> ::windows::core::Result<super::Capture::CapturedFrameControlValues>;
    fn HighDynamicRange(&self) -> ::windows::core::Result<HighDynamicRangeOutput>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectFrame2Impl: Sized + IClosableImpl + IMediaFrameImpl {
    fn AnalysisRecommendation(&self) -> ::windows::core::Result<SceneAnalysisRecommendation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneAnalyzedEventArgsImpl: Sized {
    fn ResultFrame(&self) -> ::windows::core::Result<SceneAnalysisEffectFrame>;
}
pub trait ISingleSelectMediaTrackListImpl: Sized {
    fn SelectedIndexChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedIndexChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechCueImpl: Sized + IMediaCueImpl {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartPositionInInput(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetStartPositionInInput(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn EndPositionInInput(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetEndPositionInInput(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataStreamDescriptorImpl: Sized {
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::TimedMetadataEncodingProperties>;
    fn Copy(&self) -> ::windows::core::Result<TimedMetadataStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataStreamDescriptorFactoryImpl: Sized {
    fn Create(&self, encodingproperties: &::core::option::Option<super::MediaProperties::TimedMetadataEncodingProperties>) -> ::windows::core::Result<TimedMetadataStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackImpl: Sized + IMediaTrackImpl {
    fn CueEntered(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCueEntered(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CueExited(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCueExited(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TrackFailed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTrackFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Cues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>>;
    fn ActiveCues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>>;
    fn TimedMetadataKind(&self) -> ::windows::core::Result<TimedMetadataKind>;
    fn DispatchType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddCue(&self, cue: &::core::option::Option<IMediaCue>) -> ::windows::core::Result<()>;
    fn RemoveCue(&self, cue: &::core::option::Option<IMediaCue>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrack2Impl: Sized + IMediaTrackImpl + ITimedMetadataTrackImpl {
    fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackErrorImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<TimedMetadataTrackErrorCode>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, language: &::windows::core::HSTRING, kind: TimedMetadataKind) -> ::windows::core::Result<TimedMetadataTrack>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackFailedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<TimedMetadataTrackError>;
}
pub trait ITimedMetadataTrackProviderImpl: Sized {
    fn TimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextBoutenImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<TimedTextBoutenType>;
    fn SetType(&self, value: TimedTextBoutenType) -> ::windows::core::Result<()>;
    fn Color(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<TimedTextBoutenPosition>;
    fn SetPosition(&self, value: TimedTextBoutenPosition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextCueImpl: Sized + IMediaCueImpl {
    fn CueRegion(&self) -> ::windows::core::Result<TimedTextRegion>;
    fn SetCueRegion(&self, value: &::core::option::Option<TimedTextRegion>) -> ::windows::core::Result<()>;
    fn CueStyle(&self) -> ::windows::core::Result<TimedTextStyle>;
    fn SetCueStyle(&self, value: &::core::option::Option<TimedTextStyle>) -> ::windows::core::Result<()>;
    fn Lines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<TimedTextLine>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextLineImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subformats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<TimedTextSubformat>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextRegionImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<TimedTextPoint>;
    fn SetPosition(&self, value: &TimedTextPoint) -> ::windows::core::Result<()>;
    fn Extent(&self) -> ::windows::core::Result<TimedTextSize>;
    fn SetExtent(&self, value: &TimedTextSize) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackground(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn WritingMode(&self) -> ::windows::core::Result<TimedTextWritingMode>;
    fn SetWritingMode(&self, value: TimedTextWritingMode) -> ::windows::core::Result<()>;
    fn DisplayAlignment(&self) -> ::windows::core::Result<TimedTextDisplayAlignment>;
    fn SetDisplayAlignment(&self, value: TimedTextDisplayAlignment) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetLineHeight(&self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn IsOverflowClipped(&self) -> ::windows::core::Result<bool>;
    fn SetIsOverflowClipped(&self, value: bool) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<TimedTextPadding>;
    fn SetPadding(&self, value: &TimedTextPadding) -> ::windows::core::Result<()>;
    fn TextWrapping(&self) -> ::windows::core::Result<TimedTextWrapping>;
    fn SetTextWrapping(&self, value: TimedTextWrapping) -> ::windows::core::Result<()>;
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn ScrollMode(&self) -> ::windows::core::Result<TimedTextScrollMode>;
    fn SetScrollMode(&self, value: TimedTextScrollMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextRubyImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<TimedTextRubyPosition>;
    fn SetPosition(&self, value: TimedTextRubyPosition) -> ::windows::core::Result<()>;
    fn Align(&self) -> ::windows::core::Result<TimedTextRubyAlign>;
    fn SetAlign(&self, value: TimedTextRubyAlign) -> ::windows::core::Result<()>;
    fn Reserve(&self) -> ::windows::core::Result<TimedTextRubyReserve>;
    fn SetReserve(&self, value: TimedTextRubyReserve) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceImpl: Sized {
    fn Resolved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResolved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceResolveResultEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<TimedMetadataTrackError>;
    fn Tracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceStaticsImpl: Sized {
    fn CreateFromStream(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromStreamWithLanguage(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithLanguage(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceStatics2Impl: Sized {
    fn CreateFromStreamWithIndex(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, indexstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithIndex(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, indexuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromStreamWithIndexAndLanguage(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, indexstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithIndexAndLanguage(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, indexuri: &::core::option::Option<super::super::Foundation::Uri>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyleImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFontFamily(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontSize(&self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetFontSize(&self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn FontWeight(&self) -> ::windows::core::Result<TimedTextWeight>;
    fn SetFontWeight(&self, value: TimedTextWeight) -> ::windows::core::Result<()>;
    fn Foreground(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetForeground(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackground(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn IsBackgroundAlwaysShown(&self) -> ::windows::core::Result<bool>;
    fn SetIsBackgroundAlwaysShown(&self, value: bool) -> ::windows::core::Result<()>;
    fn FlowDirection(&self) -> ::windows::core::Result<TimedTextFlowDirection>;
    fn SetFlowDirection(&self, value: TimedTextFlowDirection) -> ::windows::core::Result<()>;
    fn LineAlignment(&self) -> ::windows::core::Result<TimedTextLineAlignment>;
    fn SetLineAlignment(&self, value: TimedTextLineAlignment) -> ::windows::core::Result<()>;
    fn OutlineColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetOutlineColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn OutlineThickness(&self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetOutlineThickness(&self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn OutlineRadius(&self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetOutlineRadius(&self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyle2Impl: Sized {
    fn FontStyle(&self) -> ::windows::core::Result<TimedTextFontStyle>;
    fn SetFontStyle(&self, value: TimedTextFontStyle) -> ::windows::core::Result<()>;
    fn IsUnderlineEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsUnderlineEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsLineThroughEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsLineThroughEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOverlineEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsOverlineEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyle3Impl: Sized {
    fn Ruby(&self) -> ::windows::core::Result<TimedTextRuby>;
    fn Bouten(&self) -> ::windows::core::Result<TimedTextBouten>;
    fn IsTextCombined(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextCombined(&self, value: bool) -> ::windows::core::Result<()>;
    fn FontAngleInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetFontAngleInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSubformatImpl: Sized {
    fn StartIndex(&self) -> ::windows::core::Result<i32>;
    fn SetStartIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn SetLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn SubformatStyle(&self) -> ::windows::core::Result<TimedTextStyle>;
    fn SetSubformatStyle(&self, value: &::core::option::Option<TimedTextStyle>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStabilizationEffectImpl: Sized + IMediaExtensionImpl {
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn EnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnabledChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetRecommendedStreamConfiguration(&self, controller: &::core::option::Option<super::Devices::VideoDeviceController>, desiredproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>) -> ::windows::core::Result<super::Capture::VideoStreamConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStabilizationEffectEnabledChangedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<VideoStabilizationEffectEnabledChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptorImpl: Sized + IMediaStreamDescriptorImpl {
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptor2Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<VideoStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptorFactoryImpl: Sized {
    fn Create(&self, encodingproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>) -> ::windows::core::Result<VideoStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackImpl: Sized {
    fn OpenFailed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportInfo(&self) -> ::windows::core::Result<VideoTrackSupportInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackOpenFailedEventArgsImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackSupportInfoImpl: Sized {
    fn DecoderStatus(&self) -> ::windows::core::Result<MediaDecoderStatus>;
    fn MediaSourceStatus(&self) -> ::windows::core::Result<MediaSourceStatus>;
}
