#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Media_Core_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioDecoderDegradation(pub i32);
impl AudioDecoderDegradation {
    pub const None: AudioDecoderDegradation = AudioDecoderDegradation(0i32);
    pub const DownmixTo2Channels: AudioDecoderDegradation = AudioDecoderDegradation(1i32);
    pub const DownmixTo6Channels: AudioDecoderDegradation = AudioDecoderDegradation(2i32);
    pub const DownmixTo8Channels: AudioDecoderDegradation = AudioDecoderDegradation(3i32);
}
#[repr(transparent)]
pub struct AudioDecoderDegradationReason(pub i32);
impl AudioDecoderDegradationReason {
    pub const None: AudioDecoderDegradationReason = AudioDecoderDegradationReason(0i32);
    pub const LicensingRequirement: AudioDecoderDegradationReason = AudioDecoderDegradationReason(1i32);
    pub const SpatialAudioNotSupported: AudioDecoderDegradationReason = AudioDecoderDegradationReason(2i32);
}
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
#[repr(transparent)]
pub struct CodecCategory(pub i32);
impl CodecCategory {
    pub const Encoder: CodecCategory = CodecCategory(0i32);
    pub const Decoder: CodecCategory = CodecCategory(1i32);
}
#[repr(transparent)]
pub struct CodecInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CodecKind(pub i32);
impl CodecKind {
    pub const Audio: CodecKind = CodecKind(0i32);
    pub const Video: CodecKind = CodecKind(1i32);
}
#[repr(transparent)]
pub struct CodecQuery(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct FaceDetectionMode(pub i32);
impl FaceDetectionMode {
    pub const HighPerformance: FaceDetectionMode = FaceDetectionMode(0i32);
    pub const Balanced: FaceDetectionMode = FaceDetectionMode(1i32);
    pub const HighQuality: FaceDetectionMode = FaceDetectionMode(2i32);
}
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
pub struct LowLightFusionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBindingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCueEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaDecoderStatus(pub i32);
impl MediaDecoderStatus {
    pub const FullySupported: MediaDecoderStatus = MediaDecoderStatus(0i32);
    pub const UnsupportedSubtype: MediaDecoderStatus = MediaDecoderStatus(1i32);
    pub const UnsupportedEncoderProperties: MediaDecoderStatus = MediaDecoderStatus(2i32);
    pub const Degraded: MediaDecoderStatus = MediaDecoderStatus(3i32);
}
#[repr(transparent)]
pub struct MediaSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceAppServiceConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceOpenOperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceState(pub i32);
impl MediaSourceState {
    pub const Initial: MediaSourceState = MediaSourceState(0i32);
    pub const Opening: MediaSourceState = MediaSourceState(1i32);
    pub const Opened: MediaSourceState = MediaSourceState(2i32);
    pub const Failed: MediaSourceState = MediaSourceState(3i32);
    pub const Closed: MediaSourceState = MediaSourceState(4i32);
}
#[repr(transparent)]
pub struct MediaSourceStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceStatus(pub i32);
impl MediaSourceStatus {
    pub const FullySupported: MediaSourceStatus = MediaSourceStatus(0i32);
    pub const Unknown: MediaSourceStatus = MediaSourceStatus(1i32);
}
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
#[repr(transparent)]
pub struct MediaStreamSourceClosedReason(pub i32);
impl MediaStreamSourceClosedReason {
    pub const Done: MediaStreamSourceClosedReason = MediaStreamSourceClosedReason(0i32);
    pub const UnknownError: MediaStreamSourceClosedReason = MediaStreamSourceClosedReason(1i32);
    pub const AppReportedError: MediaStreamSourceClosedReason = MediaStreamSourceClosedReason(2i32);
    pub const UnsupportedProtectionSystem: MediaStreamSourceClosedReason = MediaStreamSourceClosedReason(3i32);
    pub const ProtectionSystemFailure: MediaStreamSourceClosedReason = MediaStreamSourceClosedReason(4i32);
    pub const UnsupportedEncodingFormat: MediaStreamSourceClosedReason = MediaStreamSourceClosedReason(5i32);
    pub const MissingSampleRequestedEventHandler: MediaStreamSourceClosedReason = MediaStreamSourceClosedReason(6i32);
}
#[repr(transparent)]
pub struct MediaStreamSourceClosedRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceErrorStatus(pub i32);
impl MediaStreamSourceErrorStatus {
    pub const Other: MediaStreamSourceErrorStatus = MediaStreamSourceErrorStatus(0i32);
    pub const OutOfMemory: MediaStreamSourceErrorStatus = MediaStreamSourceErrorStatus(1i32);
    pub const FailedToOpenFile: MediaStreamSourceErrorStatus = MediaStreamSourceErrorStatus(2i32);
    pub const FailedToConnectToServer: MediaStreamSourceErrorStatus = MediaStreamSourceErrorStatus(3i32);
    pub const ConnectionToServerLost: MediaStreamSourceErrorStatus = MediaStreamSourceErrorStatus(4i32);
    pub const UnspecifiedNetworkError: MediaStreamSourceErrorStatus = MediaStreamSourceErrorStatus(5i32);
    pub const DecodeError: MediaStreamSourceErrorStatus = MediaStreamSourceErrorStatus(6i32);
    pub const UnsupportedMediaFormat: MediaStreamSourceErrorStatus = MediaStreamSourceErrorStatus(7i32);
}
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
#[repr(transparent)]
pub struct MediaTrackKind(pub i32);
impl MediaTrackKind {
    pub const Audio: MediaTrackKind = MediaTrackKind(0i32);
    pub const Video: MediaTrackKind = MediaTrackKind(1i32);
    pub const TimedMetadata: MediaTrackKind = MediaTrackKind(2i32);
}
#[repr(transparent)]
pub struct MseAppendMode(pub i32);
impl MseAppendMode {
    pub const Segments: MseAppendMode = MseAppendMode(0i32);
    pub const Sequence: MseAppendMode = MseAppendMode(1i32);
}
#[repr(transparent)]
pub struct MseEndOfStreamStatus(pub i32);
impl MseEndOfStreamStatus {
    pub const Success: MseEndOfStreamStatus = MseEndOfStreamStatus(0i32);
    pub const NetworkError: MseEndOfStreamStatus = MseEndOfStreamStatus(1i32);
    pub const DecodeError: MseEndOfStreamStatus = MseEndOfStreamStatus(2i32);
    pub const UnknownError: MseEndOfStreamStatus = MseEndOfStreamStatus(3i32);
}
#[repr(transparent)]
pub struct MseReadyState(pub i32);
impl MseReadyState {
    pub const Closed: MseReadyState = MseReadyState(0i32);
    pub const Open: MseReadyState = MseReadyState(1i32);
    pub const Ended: MseReadyState = MseReadyState(2i32);
}
#[repr(transparent)]
pub struct MseSourceBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MseSourceBufferList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MseStreamSource(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct MseTimeRange(i32);
#[repr(transparent)]
pub struct SceneAnalysisEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAnalysisEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAnalysisEffectFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAnalysisRecommendation(pub i32);
impl SceneAnalysisRecommendation {
    pub const Standard: SceneAnalysisRecommendation = SceneAnalysisRecommendation(0i32);
    pub const Hdr: SceneAnalysisRecommendation = SceneAnalysisRecommendation(1i32);
    pub const LowLight: SceneAnalysisRecommendation = SceneAnalysisRecommendation(2i32);
}
#[repr(transparent)]
pub struct SceneAnalyzedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedMetadataKind(pub i32);
impl TimedMetadataKind {
    pub const Caption: TimedMetadataKind = TimedMetadataKind(0i32);
    pub const Chapter: TimedMetadataKind = TimedMetadataKind(1i32);
    pub const Custom: TimedMetadataKind = TimedMetadataKind(2i32);
    pub const Data: TimedMetadataKind = TimedMetadataKind(3i32);
    pub const Description: TimedMetadataKind = TimedMetadataKind(4i32);
    pub const Subtitle: TimedMetadataKind = TimedMetadataKind(5i32);
    pub const ImageSubtitle: TimedMetadataKind = TimedMetadataKind(6i32);
    pub const Speech: TimedMetadataKind = TimedMetadataKind(7i32);
}
#[repr(transparent)]
pub struct TimedMetadataStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedMetadataTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedMetadataTrackError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedMetadataTrackErrorCode(pub i32);
impl TimedMetadataTrackErrorCode {
    pub const None: TimedMetadataTrackErrorCode = TimedMetadataTrackErrorCode(0i32);
    pub const DataFormatError: TimedMetadataTrackErrorCode = TimedMetadataTrackErrorCode(1i32);
    pub const NetworkError: TimedMetadataTrackErrorCode = TimedMetadataTrackErrorCode(2i32);
    pub const InternalError: TimedMetadataTrackErrorCode = TimedMetadataTrackErrorCode(3i32);
}
#[repr(transparent)]
pub struct TimedMetadataTrackFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextBouten(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextBoutenPosition(pub i32);
impl TimedTextBoutenPosition {
    pub const Before: TimedTextBoutenPosition = TimedTextBoutenPosition(0i32);
    pub const After: TimedTextBoutenPosition = TimedTextBoutenPosition(1i32);
    pub const Outside: TimedTextBoutenPosition = TimedTextBoutenPosition(2i32);
}
#[repr(transparent)]
pub struct TimedTextBoutenType(pub i32);
impl TimedTextBoutenType {
    pub const None: TimedTextBoutenType = TimedTextBoutenType(0i32);
    pub const Auto: TimedTextBoutenType = TimedTextBoutenType(1i32);
    pub const FilledCircle: TimedTextBoutenType = TimedTextBoutenType(2i32);
    pub const OpenCircle: TimedTextBoutenType = TimedTextBoutenType(3i32);
    pub const FilledDot: TimedTextBoutenType = TimedTextBoutenType(4i32);
    pub const OpenDot: TimedTextBoutenType = TimedTextBoutenType(5i32);
    pub const FilledSesame: TimedTextBoutenType = TimedTextBoutenType(6i32);
    pub const OpenSesame: TimedTextBoutenType = TimedTextBoutenType(7i32);
}
#[repr(transparent)]
pub struct TimedTextCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextDisplayAlignment(pub i32);
impl TimedTextDisplayAlignment {
    pub const Before: TimedTextDisplayAlignment = TimedTextDisplayAlignment(0i32);
    pub const After: TimedTextDisplayAlignment = TimedTextDisplayAlignment(1i32);
    pub const Center: TimedTextDisplayAlignment = TimedTextDisplayAlignment(2i32);
}
#[repr(C)]
pub struct TimedTextDouble(i32);
#[repr(transparent)]
pub struct TimedTextFlowDirection(pub i32);
impl TimedTextFlowDirection {
    pub const LeftToRight: TimedTextFlowDirection = TimedTextFlowDirection(0i32);
    pub const RightToLeft: TimedTextFlowDirection = TimedTextFlowDirection(1i32);
}
#[repr(transparent)]
pub struct TimedTextFontStyle(pub i32);
impl TimedTextFontStyle {
    pub const Normal: TimedTextFontStyle = TimedTextFontStyle(0i32);
    pub const Oblique: TimedTextFontStyle = TimedTextFontStyle(1i32);
    pub const Italic: TimedTextFontStyle = TimedTextFontStyle(2i32);
}
#[repr(transparent)]
pub struct TimedTextLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextLineAlignment(pub i32);
impl TimedTextLineAlignment {
    pub const Start: TimedTextLineAlignment = TimedTextLineAlignment(0i32);
    pub const End: TimedTextLineAlignment = TimedTextLineAlignment(1i32);
    pub const Center: TimedTextLineAlignment = TimedTextLineAlignment(2i32);
}
#[repr(C)]
pub struct TimedTextPadding(i32);
#[repr(C)]
pub struct TimedTextPoint(i32);
#[repr(transparent)]
pub struct TimedTextRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextRuby(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextRubyAlign(pub i32);
impl TimedTextRubyAlign {
    pub const Center: TimedTextRubyAlign = TimedTextRubyAlign(0i32);
    pub const Start: TimedTextRubyAlign = TimedTextRubyAlign(1i32);
    pub const End: TimedTextRubyAlign = TimedTextRubyAlign(2i32);
    pub const SpaceAround: TimedTextRubyAlign = TimedTextRubyAlign(3i32);
    pub const SpaceBetween: TimedTextRubyAlign = TimedTextRubyAlign(4i32);
    pub const WithBase: TimedTextRubyAlign = TimedTextRubyAlign(5i32);
}
#[repr(transparent)]
pub struct TimedTextRubyPosition(pub i32);
impl TimedTextRubyPosition {
    pub const Before: TimedTextRubyPosition = TimedTextRubyPosition(0i32);
    pub const After: TimedTextRubyPosition = TimedTextRubyPosition(1i32);
    pub const Outside: TimedTextRubyPosition = TimedTextRubyPosition(2i32);
}
#[repr(transparent)]
pub struct TimedTextRubyReserve(pub i32);
impl TimedTextRubyReserve {
    pub const None: TimedTextRubyReserve = TimedTextRubyReserve(0i32);
    pub const Before: TimedTextRubyReserve = TimedTextRubyReserve(1i32);
    pub const After: TimedTextRubyReserve = TimedTextRubyReserve(2i32);
    pub const Both: TimedTextRubyReserve = TimedTextRubyReserve(3i32);
    pub const Outside: TimedTextRubyReserve = TimedTextRubyReserve(4i32);
}
#[repr(transparent)]
pub struct TimedTextScrollMode(pub i32);
impl TimedTextScrollMode {
    pub const Popon: TimedTextScrollMode = TimedTextScrollMode(0i32);
    pub const Rollup: TimedTextScrollMode = TimedTextScrollMode(1i32);
}
#[repr(C)]
pub struct TimedTextSize(i32);
#[repr(transparent)]
pub struct TimedTextSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextSourceResolveResultEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextSubformat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextUnit(pub i32);
impl TimedTextUnit {
    pub const Pixels: TimedTextUnit = TimedTextUnit(0i32);
    pub const Percentage: TimedTextUnit = TimedTextUnit(1i32);
}
#[repr(transparent)]
pub struct TimedTextWeight(pub i32);
impl TimedTextWeight {
    pub const Normal: TimedTextWeight = TimedTextWeight(400i32);
    pub const Bold: TimedTextWeight = TimedTextWeight(700i32);
}
#[repr(transparent)]
pub struct TimedTextWrapping(pub i32);
impl TimedTextWrapping {
    pub const NoWrap: TimedTextWrapping = TimedTextWrapping(0i32);
    pub const Wrap: TimedTextWrapping = TimedTextWrapping(1i32);
}
#[repr(transparent)]
pub struct TimedTextWritingMode(pub i32);
impl TimedTextWritingMode {
    pub const LeftRightTopBottom: TimedTextWritingMode = TimedTextWritingMode(0i32);
    pub const RightLeftTopBottom: TimedTextWritingMode = TimedTextWritingMode(1i32);
    pub const TopBottomRightLeft: TimedTextWritingMode = TimedTextWritingMode(2i32);
    pub const TopBottomLeftRight: TimedTextWritingMode = TimedTextWritingMode(3i32);
    pub const LeftRight: TimedTextWritingMode = TimedTextWritingMode(4i32);
    pub const RightLeft: TimedTextWritingMode = TimedTextWritingMode(5i32);
    pub const TopBottom: TimedTextWritingMode = TimedTextWritingMode(6i32);
}
#[repr(transparent)]
pub struct VideoStabilizationEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoStabilizationEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedReason(pub i32);
impl VideoStabilizationEffectEnabledChangedReason {
    pub const Programmatic: VideoStabilizationEffectEnabledChangedReason = VideoStabilizationEffectEnabledChangedReason(0i32);
    pub const PixelRateTooHigh: VideoStabilizationEffectEnabledChangedReason = VideoStabilizationEffectEnabledChangedReason(1i32);
    pub const RunningSlowly: VideoStabilizationEffectEnabledChangedReason = VideoStabilizationEffectEnabledChangedReason(2i32);
}
#[repr(transparent)]
pub struct VideoStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrackSupportInfo(pub *mut ::core::ffi::c_void);
