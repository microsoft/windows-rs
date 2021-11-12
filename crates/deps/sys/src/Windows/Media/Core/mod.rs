#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Media_Core_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
pub struct AudioDecoderDegradation(i32);
pub struct AudioDecoderDegradationReason(i32);
#[repr(transparent)]
pub struct AudioStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioTrackSupportInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChapterCue(pub *mut ::core::ffi::c_void);
pub struct CodecCategory(i32);
#[repr(transparent)]
pub struct CodecInfo(pub *mut ::core::ffi::c_void);
pub struct CodecKind(i32);
#[repr(transparent)]
pub struct CodecQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CodecSubtypes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FaceDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FaceDetectionEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FaceDetectionEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FaceDetectionEffectFrame(pub *mut ::core::ffi::c_void);
pub struct FaceDetectionMode(i32);
#[repr(transparent)]
pub struct HighDynamicRangeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HighDynamicRangeOutput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStreamDescriptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStreamDescriptor3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStreamDescriptorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioTrackSupportInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChapterCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICodecInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICodecQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICodecSubtypesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataCue2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaceDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaceDetectionEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaceDetectionEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaceDetectionEffectFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHighDynamicRangeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHighDynamicRangeOutput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLightFusionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLightFusionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBindingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBindingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBindingEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCueEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSource3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSource4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSource5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnectionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceOpenOperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamDescriptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSample(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSample2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSampleProtectionProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSampleStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSampleStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSource3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSource4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceClosedRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRenderedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMseSourceBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMseSourceBufferList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMseStreamSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMseStreamSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMseStreamSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneAnalysisEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneAnalyzedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISingleSelectMediaTrackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataTrack2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataTrackError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataTrackFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataTrackFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataTrackProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextBouten(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextRuby(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextSourceResolveResultEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextSourceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextStyle2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextStyle3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedTextSubformat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoStabilizationEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoStreamDescriptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoStreamDescriptorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoTrackSupportInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InitializeMediaStreamSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLightFusion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLightFusionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBindingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCueEventArgs(pub *mut ::core::ffi::c_void);
pub struct MediaDecoderStatus(i32);
#[repr(transparent)]
pub struct MediaSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceAppServiceConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceOpenOperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
pub struct MediaSourceState(i32);
#[repr(transparent)]
pub struct MediaSourceStateChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct MediaSourceStatus(i32);
#[repr(transparent)]
pub struct MediaStreamSample(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSamplePropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSampleProtectionProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceClosedEventArgs(pub *mut ::core::ffi::c_void);
pub struct MediaStreamSourceClosedReason(i32);
#[repr(transparent)]
pub struct MediaStreamSourceClosedRequest(pub *mut ::core::ffi::c_void);
pub struct MediaStreamSourceErrorStatus(i32);
#[repr(transparent)]
pub struct MediaStreamSourceSampleRenderedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct MediaTrackKind(i32);
pub struct MseAppendMode(i32);
pub struct MseEndOfStreamStatus(i32);
pub struct MseReadyState(i32);
#[repr(transparent)]
pub struct MseSourceBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MseSourceBufferList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MseStreamSource(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
pub struct MseTimeRange(i32);
#[repr(transparent)]
pub struct SceneAnalysisEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAnalysisEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAnalysisEffectFrame(pub *mut ::core::ffi::c_void);
pub struct SceneAnalysisRecommendation(i32);
#[repr(transparent)]
pub struct SceneAnalyzedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechCue(pub *mut ::core::ffi::c_void);
pub struct TimedMetadataKind(i32);
#[repr(transparent)]
pub struct TimedMetadataStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedMetadataTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedMetadataTrackError(pub *mut ::core::ffi::c_void);
pub struct TimedMetadataTrackErrorCode(i32);
#[repr(transparent)]
pub struct TimedMetadataTrackFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextBouten(pub *mut ::core::ffi::c_void);
pub struct TimedTextBoutenPosition(i32);
pub struct TimedTextBoutenType(i32);
#[repr(transparent)]
pub struct TimedTextCue(pub *mut ::core::ffi::c_void);
pub struct TimedTextDisplayAlignment(i32);
pub struct TimedTextDouble(i32);
pub struct TimedTextFlowDirection(i32);
pub struct TimedTextFontStyle(i32);
#[repr(transparent)]
pub struct TimedTextLine(pub *mut ::core::ffi::c_void);
pub struct TimedTextLineAlignment(i32);
pub struct TimedTextPadding(i32);
pub struct TimedTextPoint(i32);
#[repr(transparent)]
pub struct TimedTextRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextRuby(pub *mut ::core::ffi::c_void);
pub struct TimedTextRubyAlign(i32);
pub struct TimedTextRubyPosition(i32);
pub struct TimedTextRubyReserve(i32);
pub struct TimedTextScrollMode(i32);
pub struct TimedTextSize(i32);
#[repr(transparent)]
pub struct TimedTextSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextSourceResolveResultEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextSubformat(pub *mut ::core::ffi::c_void);
pub struct TimedTextUnit(i32);
pub struct TimedTextWeight(i32);
pub struct TimedTextWrapping(i32);
pub struct TimedTextWritingMode(i32);
#[repr(transparent)]
pub struct VideoStabilizationEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoStabilizationEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct VideoStabilizationEffectEnabledChangedReason(i32);
#[repr(transparent)]
pub struct VideoStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrackSupportInfo(pub *mut ::core::ffi::c_void);
