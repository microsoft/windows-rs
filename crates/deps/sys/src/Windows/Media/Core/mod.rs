#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Media_Core_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioDecoderDegradation(pub i32);
impl AudioDecoderDegradation {
    pub const None: Self = Self(0i32);
    pub const DownmixTo2Channels: Self = Self(1i32);
    pub const DownmixTo6Channels: Self = Self(2i32);
    pub const DownmixTo8Channels: Self = Self(3i32);
}
#[repr(transparent)]
pub struct AudioDecoderDegradationReason(pub i32);
impl AudioDecoderDegradationReason {
    pub const None: Self = Self(0i32);
    pub const LicensingRequirement: Self = Self(1i32);
    pub const SpatialAudioNotSupported: Self = Self(2i32);
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
    pub const Encoder: Self = Self(0i32);
    pub const Decoder: Self = Self(1i32);
}
#[repr(transparent)]
pub struct CodecInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CodecKind(pub i32);
impl CodecKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
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
    pub const HighPerformance: Self = Self(0i32);
    pub const Balanced: Self = Self(1i32);
    pub const HighQuality: Self = Self(2i32);
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
    pub const FullySupported: Self = Self(0i32);
    pub const UnsupportedSubtype: Self = Self(1i32);
    pub const UnsupportedEncoderProperties: Self = Self(2i32);
    pub const Degraded: Self = Self(3i32);
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
    pub const Initial: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Opened: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const Closed: Self = Self(4i32);
}
#[repr(transparent)]
pub struct MediaSourceStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceStatus(pub i32);
impl MediaSourceStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
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
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const AppReportedError: Self = Self(2i32);
    pub const UnsupportedProtectionSystem: Self = Self(3i32);
    pub const ProtectionSystemFailure: Self = Self(4i32);
    pub const UnsupportedEncodingFormat: Self = Self(5i32);
    pub const MissingSampleRequestedEventHandler: Self = Self(6i32);
}
#[repr(transparent)]
pub struct MediaStreamSourceClosedRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaStreamSourceErrorStatus(pub i32);
impl MediaStreamSourceErrorStatus {
    pub const Other: Self = Self(0i32);
    pub const OutOfMemory: Self = Self(1i32);
    pub const FailedToOpenFile: Self = Self(2i32);
    pub const FailedToConnectToServer: Self = Self(3i32);
    pub const ConnectionToServerLost: Self = Self(4i32);
    pub const UnspecifiedNetworkError: Self = Self(5i32);
    pub const DecodeError: Self = Self(6i32);
    pub const UnsupportedMediaFormat: Self = Self(7i32);
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
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
    pub const TimedMetadata: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MseAppendMode(pub i32);
impl MseAppendMode {
    pub const Segments: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MseEndOfStreamStatus(pub i32);
impl MseEndOfStreamStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const DecodeError: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
#[repr(transparent)]
pub struct MseReadyState(pub i32);
impl MseReadyState {
    pub const Closed: Self = Self(0i32);
    pub const Open: Self = Self(1i32);
    pub const Ended: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MseSourceBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MseSourceBufferList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MseStreamSource(pub *mut ::core::ffi::c_void);
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct MseTimeRange {
    pub Start: super::super::Foundation::TimeSpan,
    pub End: super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for MseTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for MseTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneAnalysisEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAnalysisEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAnalysisEffectFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAnalysisRecommendation(pub i32);
impl SceneAnalysisRecommendation {
    pub const Standard: Self = Self(0i32);
    pub const Hdr: Self = Self(1i32);
    pub const LowLight: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SceneAnalyzedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedMetadataKind(pub i32);
impl TimedMetadataKind {
    pub const Caption: Self = Self(0i32);
    pub const Chapter: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Data: Self = Self(3i32);
    pub const Description: Self = Self(4i32);
    pub const Subtitle: Self = Self(5i32);
    pub const ImageSubtitle: Self = Self(6i32);
    pub const Speech: Self = Self(7i32);
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
    pub const None: Self = Self(0i32);
    pub const DataFormatError: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const InternalError: Self = Self(3i32);
}
#[repr(transparent)]
pub struct TimedMetadataTrackFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextBouten(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextBoutenPosition(pub i32);
impl TimedTextBoutenPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
#[repr(transparent)]
pub struct TimedTextBoutenType(pub i32);
impl TimedTextBoutenType {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const FilledCircle: Self = Self(2i32);
    pub const OpenCircle: Self = Self(3i32);
    pub const FilledDot: Self = Self(4i32);
    pub const OpenDot: Self = Self(5i32);
    pub const FilledSesame: Self = Self(6i32);
    pub const OpenSesame: Self = Self(7i32);
}
#[repr(transparent)]
pub struct TimedTextCue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextDisplayAlignment(pub i32);
impl TimedTextDisplayAlignment {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
#[repr(C)]
pub struct TimedTextDouble {
    pub Value: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextDouble {}
impl ::core::clone::Clone for TimedTextDouble {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextFlowDirection(pub i32);
impl TimedTextFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
#[repr(transparent)]
pub struct TimedTextFontStyle(pub i32);
impl TimedTextFontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
#[repr(transparent)]
pub struct TimedTextLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextLineAlignment(pub i32);
impl TimedTextLineAlignment {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
#[repr(C)]
pub struct TimedTextPadding {
    pub Before: f64,
    pub After: f64,
    pub Start: f64,
    pub End: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPadding {}
impl ::core::clone::Clone for TimedTextPadding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TimedTextPoint {
    pub X: f64,
    pub Y: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPoint {}
impl ::core::clone::Clone for TimedTextPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextRuby(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedTextRubyAlign(pub i32);
impl TimedTextRubyAlign {
    pub const Center: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
    pub const SpaceAround: Self = Self(3i32);
    pub const SpaceBetween: Self = Self(4i32);
    pub const WithBase: Self = Self(5i32);
}
#[repr(transparent)]
pub struct TimedTextRubyPosition(pub i32);
impl TimedTextRubyPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
#[repr(transparent)]
pub struct TimedTextRubyReserve(pub i32);
impl TimedTextRubyReserve {
    pub const None: Self = Self(0i32);
    pub const Before: Self = Self(1i32);
    pub const After: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const Outside: Self = Self(4i32);
}
#[repr(transparent)]
pub struct TimedTextScrollMode(pub i32);
impl TimedTextScrollMode {
    pub const Popon: Self = Self(0i32);
    pub const Rollup: Self = Self(1i32);
}
#[repr(C)]
pub struct TimedTextSize {
    pub Height: f64,
    pub Width: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextSize {}
impl ::core::clone::Clone for TimedTextSize {
    fn clone(&self) -> Self {
        *self
    }
}
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
    pub const Pixels: Self = Self(0i32);
    pub const Percentage: Self = Self(1i32);
}
#[repr(transparent)]
pub struct TimedTextWeight(pub i32);
impl TimedTextWeight {
    pub const Normal: Self = Self(400i32);
    pub const Bold: Self = Self(700i32);
}
#[repr(transparent)]
pub struct TimedTextWrapping(pub i32);
impl TimedTextWrapping {
    pub const NoWrap: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
}
#[repr(transparent)]
pub struct TimedTextWritingMode(pub i32);
impl TimedTextWritingMode {
    pub const LeftRightTopBottom: Self = Self(0i32);
    pub const RightLeftTopBottom: Self = Self(1i32);
    pub const TopBottomRightLeft: Self = Self(2i32);
    pub const TopBottomLeftRight: Self = Self(3i32);
    pub const LeftRight: Self = Self(4i32);
    pub const RightLeft: Self = Self(5i32);
    pub const TopBottom: Self = Self(6i32);
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
    pub const Programmatic: Self = Self(0i32);
    pub const PixelRateTooHigh: Self = Self(1i32);
    pub const RunningSlowly: Self = Self(2i32);
}
#[repr(transparent)]
pub struct VideoStreamDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTrackSupportInfo(pub *mut ::core::ffi::c_void);
