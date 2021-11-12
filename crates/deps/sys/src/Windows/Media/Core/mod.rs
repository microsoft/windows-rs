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
impl ::core::marker::Copy for AudioDecoderDegradation {}
impl ::core::clone::Clone for AudioDecoderDegradation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDecoderDegradationReason(pub i32);
impl AudioDecoderDegradationReason {
    pub const None: Self = Self(0i32);
    pub const LicensingRequirement: Self = Self(1i32);
    pub const SpatialAudioNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDecoderDegradationReason {}
impl ::core::clone::Clone for AudioDecoderDegradationReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioStreamDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioStreamDescriptor {}
impl ::core::clone::Clone for AudioStreamDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioTrack {}
impl ::core::clone::Clone for AudioTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioTrackOpenFailedEventArgs {}
impl ::core::clone::Clone for AudioTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioTrackSupportInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioTrackSupportInfo {}
impl ::core::clone::Clone for AudioTrackSupportInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChapterCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChapterCue {}
impl ::core::clone::Clone for ChapterCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CodecCategory(pub i32);
impl CodecCategory {
    pub const Encoder: Self = Self(0i32);
    pub const Decoder: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecCategory {}
impl ::core::clone::Clone for CodecCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CodecInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CodecInfo {}
impl ::core::clone::Clone for CodecInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CodecKind(pub i32);
impl CodecKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecKind {}
impl ::core::clone::Clone for CodecKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CodecQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CodecQuery {}
impl ::core::clone::Clone for CodecQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataCue {}
impl ::core::clone::Clone for DataCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FaceDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FaceDetectedEventArgs {}
impl ::core::clone::Clone for FaceDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FaceDetectionEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FaceDetectionEffect {}
impl ::core::clone::Clone for FaceDetectionEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FaceDetectionEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FaceDetectionEffectDefinition {}
impl ::core::clone::Clone for FaceDetectionEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FaceDetectionEffectFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FaceDetectionEffectFrame {}
impl ::core::clone::Clone for FaceDetectionEffectFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FaceDetectionMode(pub i32);
impl FaceDetectionMode {
    pub const HighPerformance: Self = Self(0i32);
    pub const Balanced: Self = Self(1i32);
    pub const HighQuality: Self = Self(2i32);
}
impl ::core::marker::Copy for FaceDetectionMode {}
impl ::core::clone::Clone for FaceDetectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HighDynamicRangeControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HighDynamicRangeControl {}
impl ::core::clone::Clone for HighDynamicRangeControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HighDynamicRangeOutput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HighDynamicRangeOutput {}
impl ::core::clone::Clone for HighDynamicRangeOutput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioStreamDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioStreamDescriptor {}
impl ::core::clone::Clone for IAudioStreamDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioStreamDescriptor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioStreamDescriptor2 {}
impl ::core::clone::Clone for IAudioStreamDescriptor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioStreamDescriptor3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioStreamDescriptor3 {}
impl ::core::clone::Clone for IAudioStreamDescriptor3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioStreamDescriptorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioStreamDescriptorFactory {}
impl ::core::clone::Clone for IAudioStreamDescriptorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioTrack {}
impl ::core::clone::Clone for IAudioTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioTrackOpenFailedEventArgs {}
impl ::core::clone::Clone for IAudioTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioTrackSupportInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioTrackSupportInfo {}
impl ::core::clone::Clone for IAudioTrackSupportInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChapterCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChapterCue {}
impl ::core::clone::Clone for IChapterCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICodecInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICodecInfo {}
impl ::core::clone::Clone for ICodecInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICodecQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICodecQuery {}
impl ::core::clone::Clone for ICodecQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICodecSubtypesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICodecSubtypesStatics {}
impl ::core::clone::Clone for ICodecSubtypesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataCue {}
impl ::core::clone::Clone for IDataCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataCue2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataCue2 {}
impl ::core::clone::Clone for IDataCue2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFaceDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFaceDetectedEventArgs {}
impl ::core::clone::Clone for IFaceDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFaceDetectionEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFaceDetectionEffect {}
impl ::core::clone::Clone for IFaceDetectionEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFaceDetectionEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFaceDetectionEffectDefinition {}
impl ::core::clone::Clone for IFaceDetectionEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFaceDetectionEffectFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFaceDetectionEffectFrame {}
impl ::core::clone::Clone for IFaceDetectionEffectFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHighDynamicRangeControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHighDynamicRangeControl {}
impl ::core::clone::Clone for IHighDynamicRangeControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHighDynamicRangeOutput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHighDynamicRangeOutput {}
impl ::core::clone::Clone for IHighDynamicRangeOutput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageCue {}
impl ::core::clone::Clone for IImageCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInitializeMediaStreamSourceRequestedEventArgs {}
impl ::core::clone::Clone for IInitializeMediaStreamSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLightFusionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLightFusionResult {}
impl ::core::clone::Clone for ILowLightFusionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLightFusionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLightFusionStatics {}
impl ::core::clone::Clone for ILowLightFusionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBinder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBinder {}
impl ::core::clone::Clone for IMediaBinder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBindingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBindingEventArgs {}
impl ::core::clone::Clone for IMediaBindingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBindingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBindingEventArgs2 {}
impl ::core::clone::Clone for IMediaBindingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBindingEventArgs3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBindingEventArgs3 {}
impl ::core::clone::Clone for IMediaBindingEventArgs3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCue {}
impl ::core::clone::Clone for IMediaCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCueEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCueEventArgs {}
impl ::core::clone::Clone for IMediaCueEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSource {}
impl ::core::clone::Clone for IMediaSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSource2 {}
impl ::core::clone::Clone for IMediaSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSource3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSource3 {}
impl ::core::clone::Clone for IMediaSource3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSource4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSource4 {}
impl ::core::clone::Clone for IMediaSource4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSource5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSource5 {}
impl ::core::clone::Clone for IMediaSource5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceAppServiceConnection {}
impl ::core::clone::Clone for IMediaSourceAppServiceConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnectionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceAppServiceConnectionFactory {}
impl ::core::clone::Clone for IMediaSourceAppServiceConnectionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceError {}
impl ::core::clone::Clone for IMediaSourceError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceOpenOperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceOpenOperationCompletedEventArgs {}
impl ::core::clone::Clone for IMediaSourceOpenOperationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceStateChangedEventArgs {}
impl ::core::clone::Clone for IMediaSourceStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceStatics {}
impl ::core::clone::Clone for IMediaSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceStatics2 {}
impl ::core::clone::Clone for IMediaSourceStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceStatics3 {}
impl ::core::clone::Clone for IMediaSourceStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceStatics4 {}
impl ::core::clone::Clone for IMediaSourceStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamDescriptor {}
impl ::core::clone::Clone for IMediaStreamDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamDescriptor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamDescriptor2 {}
impl ::core::clone::Clone for IMediaStreamDescriptor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSample(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSample {}
impl ::core::clone::Clone for IMediaStreamSample {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSample2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSample2 {}
impl ::core::clone::Clone for IMediaStreamSample2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSampleProtectionProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSampleProtectionProperties {}
impl ::core::clone::Clone for IMediaStreamSampleProtectionProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSampleStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSampleStatics {}
impl ::core::clone::Clone for IMediaStreamSampleStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSampleStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSampleStatics2 {}
impl ::core::clone::Clone for IMediaStreamSampleStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSource {}
impl ::core::clone::Clone for IMediaStreamSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSource2 {}
impl ::core::clone::Clone for IMediaStreamSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSource3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSource3 {}
impl ::core::clone::Clone for IMediaStreamSource3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSource4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSource4 {}
impl ::core::clone::Clone for IMediaStreamSource4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceClosedEventArgs {}
impl ::core::clone::Clone for IMediaStreamSourceClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceClosedRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceClosedRequest {}
impl ::core::clone::Clone for IMediaStreamSourceClosedRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceFactory {}
impl ::core::clone::Clone for IMediaStreamSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRenderedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceSampleRenderedEventArgs {}
impl ::core::clone::Clone for IMediaStreamSourceSampleRenderedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceSampleRequest {}
impl ::core::clone::Clone for IMediaStreamSourceSampleRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceSampleRequestDeferral {}
impl ::core::clone::Clone for IMediaStreamSourceSampleRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceSampleRequestedEventArgs {}
impl ::core::clone::Clone for IMediaStreamSourceSampleRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceStartingEventArgs {}
impl ::core::clone::Clone for IMediaStreamSourceStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceStartingRequest {}
impl ::core::clone::Clone for IMediaStreamSourceStartingRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceStartingRequestDeferral {}
impl ::core::clone::Clone for IMediaStreamSourceStartingRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceSwitchStreamsRequest {}
impl ::core::clone::Clone for IMediaStreamSourceSwitchStreamsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceSwitchStreamsRequestDeferral {}
impl ::core::clone::Clone for IMediaStreamSourceSwitchStreamsRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaStreamSourceSwitchStreamsRequestedEventArgs {}
impl ::core::clone::Clone for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTrack {}
impl ::core::clone::Clone for IMediaTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMseSourceBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMseSourceBuffer {}
impl ::core::clone::Clone for IMseSourceBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMseSourceBufferList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMseSourceBufferList {}
impl ::core::clone::Clone for IMseSourceBufferList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMseStreamSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMseStreamSource {}
impl ::core::clone::Clone for IMseStreamSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMseStreamSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMseStreamSource2 {}
impl ::core::clone::Clone for IMseStreamSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMseStreamSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMseStreamSourceStatics {}
impl ::core::clone::Clone for IMseStreamSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneAnalysisEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneAnalysisEffect {}
impl ::core::clone::Clone for ISceneAnalysisEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneAnalysisEffectFrame {}
impl ::core::clone::Clone for ISceneAnalysisEffectFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneAnalysisEffectFrame2 {}
impl ::core::clone::Clone for ISceneAnalysisEffectFrame2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneAnalyzedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneAnalyzedEventArgs {}
impl ::core::clone::Clone for ISceneAnalyzedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISingleSelectMediaTrackList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISingleSelectMediaTrackList {}
impl ::core::clone::Clone for ISingleSelectMediaTrackList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechCue {}
impl ::core::clone::Clone for ISpeechCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataStreamDescriptor {}
impl ::core::clone::Clone for ITimedMetadataStreamDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataStreamDescriptorFactory {}
impl ::core::clone::Clone for ITimedMetadataStreamDescriptorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataTrack {}
impl ::core::clone::Clone for ITimedMetadataTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataTrack2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataTrack2 {}
impl ::core::clone::Clone for ITimedMetadataTrack2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataTrackError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataTrackError {}
impl ::core::clone::Clone for ITimedMetadataTrackError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataTrackFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataTrackFactory {}
impl ::core::clone::Clone for ITimedMetadataTrackFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataTrackFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataTrackFailedEventArgs {}
impl ::core::clone::Clone for ITimedMetadataTrackFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataTrackProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataTrackProvider {}
impl ::core::clone::Clone for ITimedMetadataTrackProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextBouten(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextBouten {}
impl ::core::clone::Clone for ITimedTextBouten {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextCue {}
impl ::core::clone::Clone for ITimedTextCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextLine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextLine {}
impl ::core::clone::Clone for ITimedTextLine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextRegion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextRegion {}
impl ::core::clone::Clone for ITimedTextRegion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextRuby(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextRuby {}
impl ::core::clone::Clone for ITimedTextRuby {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextSource {}
impl ::core::clone::Clone for ITimedTextSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextSourceResolveResultEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextSourceResolveResultEventArgs {}
impl ::core::clone::Clone for ITimedTextSourceResolveResultEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextSourceStatics {}
impl ::core::clone::Clone for ITimedTextSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextSourceStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextSourceStatics2 {}
impl ::core::clone::Clone for ITimedTextSourceStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextStyle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextStyle {}
impl ::core::clone::Clone for ITimedTextStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextStyle2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextStyle2 {}
impl ::core::clone::Clone for ITimedTextStyle2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextStyle3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextStyle3 {}
impl ::core::clone::Clone for ITimedTextStyle3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedTextSubformat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedTextSubformat {}
impl ::core::clone::Clone for ITimedTextSubformat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoStabilizationEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoStabilizationEffect {}
impl ::core::clone::Clone for IVideoStabilizationEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoStabilizationEffectEnabledChangedEventArgs {}
impl ::core::clone::Clone for IVideoStabilizationEffectEnabledChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoStreamDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoStreamDescriptor {}
impl ::core::clone::Clone for IVideoStreamDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoStreamDescriptor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoStreamDescriptor2 {}
impl ::core::clone::Clone for IVideoStreamDescriptor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoStreamDescriptorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoStreamDescriptorFactory {}
impl ::core::clone::Clone for IVideoStreamDescriptorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoTrack {}
impl ::core::clone::Clone for IVideoTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoTrackOpenFailedEventArgs {}
impl ::core::clone::Clone for IVideoTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoTrackSupportInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoTrackSupportInfo {}
impl ::core::clone::Clone for IVideoTrackSupportInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageCue {}
impl ::core::clone::Clone for ImageCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InitializeMediaStreamSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InitializeMediaStreamSourceRequestedEventArgs {}
impl ::core::clone::Clone for InitializeMediaStreamSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LowLightFusionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LowLightFusionResult {}
impl ::core::clone::Clone for LowLightFusionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBinder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBinder {}
impl ::core::clone::Clone for MediaBinder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBindingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBindingEventArgs {}
impl ::core::clone::Clone for MediaBindingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCueEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCueEventArgs {}
impl ::core::clone::Clone for MediaCueEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaDecoderStatus(pub i32);
impl MediaDecoderStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const UnsupportedSubtype: Self = Self(1i32);
    pub const UnsupportedEncoderProperties: Self = Self(2i32);
    pub const Degraded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaDecoderStatus {}
impl ::core::clone::Clone for MediaDecoderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaSource {}
impl ::core::clone::Clone for MediaSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSourceAppServiceConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaSourceAppServiceConnection {}
impl ::core::clone::Clone for MediaSourceAppServiceConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSourceError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaSourceError {}
impl ::core::clone::Clone for MediaSourceError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSourceOpenOperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaSourceOpenOperationCompletedEventArgs {}
impl ::core::clone::Clone for MediaSourceOpenOperationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSourceState(pub i32);
impl MediaSourceState {
    pub const Initial: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Opened: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const Closed: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaSourceState {}
impl ::core::clone::Clone for MediaSourceState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSourceStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaSourceStateChangedEventArgs {}
impl ::core::clone::Clone for MediaSourceStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSourceStatus(pub i32);
impl MediaSourceStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaSourceStatus {}
impl ::core::clone::Clone for MediaSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSample(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSample {}
impl ::core::clone::Clone for MediaStreamSample {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSamplePropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSamplePropertySet {}
impl ::core::clone::Clone for MediaStreamSamplePropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSampleProtectionProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSampleProtectionProperties {}
impl ::core::clone::Clone for MediaStreamSampleProtectionProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSource {}
impl ::core::clone::Clone for MediaStreamSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceClosedEventArgs {}
impl ::core::clone::Clone for MediaStreamSourceClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MediaStreamSourceClosedReason {}
impl ::core::clone::Clone for MediaStreamSourceClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceClosedRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceClosedRequest {}
impl ::core::clone::Clone for MediaStreamSourceClosedRequest {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MediaStreamSourceErrorStatus {}
impl ::core::clone::Clone for MediaStreamSourceErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceSampleRenderedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceSampleRenderedEventArgs {}
impl ::core::clone::Clone for MediaStreamSourceSampleRenderedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceSampleRequest {}
impl ::core::clone::Clone for MediaStreamSourceSampleRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceSampleRequestDeferral {}
impl ::core::clone::Clone for MediaStreamSourceSampleRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceSampleRequestedEventArgs {}
impl ::core::clone::Clone for MediaStreamSourceSampleRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceStartingEventArgs {}
impl ::core::clone::Clone for MediaStreamSourceStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceStartingRequest {}
impl ::core::clone::Clone for MediaStreamSourceStartingRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceStartingRequestDeferral {}
impl ::core::clone::Clone for MediaStreamSourceStartingRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceSwitchStreamsRequest {}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceSwitchStreamsRequestDeferral {}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTrackKind(pub i32);
impl MediaTrackKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
    pub const TimedMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaTrackKind {}
impl ::core::clone::Clone for MediaTrackKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MseAppendMode(pub i32);
impl MseAppendMode {
    pub const Segments: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
}
impl ::core::marker::Copy for MseAppendMode {}
impl ::core::clone::Clone for MseAppendMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MseEndOfStreamStatus(pub i32);
impl MseEndOfStreamStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const DecodeError: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MseEndOfStreamStatus {}
impl ::core::clone::Clone for MseEndOfStreamStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MseReadyState(pub i32);
impl MseReadyState {
    pub const Closed: Self = Self(0i32);
    pub const Open: Self = Self(1i32);
    pub const Ended: Self = Self(2i32);
}
impl ::core::marker::Copy for MseReadyState {}
impl ::core::clone::Clone for MseReadyState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MseSourceBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MseSourceBuffer {}
impl ::core::clone::Clone for MseSourceBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MseSourceBufferList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MseSourceBufferList {}
impl ::core::clone::Clone for MseSourceBufferList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MseStreamSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MseStreamSource {}
impl ::core::clone::Clone for MseStreamSource {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SceneAnalysisEffect {}
impl ::core::clone::Clone for SceneAnalysisEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneAnalysisEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneAnalysisEffectDefinition {}
impl ::core::clone::Clone for SceneAnalysisEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneAnalysisEffectFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneAnalysisEffectFrame {}
impl ::core::clone::Clone for SceneAnalysisEffectFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneAnalysisRecommendation(pub i32);
impl SceneAnalysisRecommendation {
    pub const Standard: Self = Self(0i32);
    pub const Hdr: Self = Self(1i32);
    pub const LowLight: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneAnalysisRecommendation {}
impl ::core::clone::Clone for SceneAnalysisRecommendation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneAnalyzedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneAnalyzedEventArgs {}
impl ::core::clone::Clone for SceneAnalyzedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechCue {}
impl ::core::clone::Clone for SpeechCue {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TimedMetadataKind {}
impl ::core::clone::Clone for TimedMetadataKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedMetadataStreamDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedMetadataStreamDescriptor {}
impl ::core::clone::Clone for TimedMetadataStreamDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedMetadataTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedMetadataTrack {}
impl ::core::clone::Clone for TimedMetadataTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedMetadataTrackError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedMetadataTrackError {}
impl ::core::clone::Clone for TimedMetadataTrackError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedMetadataTrackErrorCode(pub i32);
impl TimedMetadataTrackErrorCode {
    pub const None: Self = Self(0i32);
    pub const DataFormatError: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const InternalError: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackErrorCode {}
impl ::core::clone::Clone for TimedMetadataTrackErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedMetadataTrackFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedMetadataTrackFailedEventArgs {}
impl ::core::clone::Clone for TimedMetadataTrackFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextBouten(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedTextBouten {}
impl ::core::clone::Clone for TimedTextBouten {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextBoutenPosition(pub i32);
impl TimedTextBoutenPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextBoutenPosition {}
impl ::core::clone::Clone for TimedTextBoutenPosition {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for TimedTextBoutenType {}
impl ::core::clone::Clone for TimedTextBoutenType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextCue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedTextCue {}
impl ::core::clone::Clone for TimedTextCue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextDisplayAlignment(pub i32);
impl TimedTextDisplayAlignment {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextDisplayAlignment {}
impl ::core::clone::Clone for TimedTextDisplayAlignment {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for TimedTextFlowDirection {}
impl ::core::clone::Clone for TimedTextFlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextFontStyle(pub i32);
impl TimedTextFontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextFontStyle {}
impl ::core::clone::Clone for TimedTextFontStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextLine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedTextLine {}
impl ::core::clone::Clone for TimedTextLine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextLineAlignment(pub i32);
impl TimedTextLineAlignment {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextLineAlignment {}
impl ::core::clone::Clone for TimedTextLineAlignment {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for TimedTextRegion {}
impl ::core::clone::Clone for TimedTextRegion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextRuby(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedTextRuby {}
impl ::core::clone::Clone for TimedTextRuby {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TimedTextRubyAlign {}
impl ::core::clone::Clone for TimedTextRubyAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextRubyPosition(pub i32);
impl TimedTextRubyPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextRubyPosition {}
impl ::core::clone::Clone for TimedTextRubyPosition {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for TimedTextRubyReserve {}
impl ::core::clone::Clone for TimedTextRubyReserve {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextScrollMode(pub i32);
impl TimedTextScrollMode {
    pub const Popon: Self = Self(0i32);
    pub const Rollup: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextScrollMode {}
impl ::core::clone::Clone for TimedTextScrollMode {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for TimedTextSource {}
impl ::core::clone::Clone for TimedTextSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextSourceResolveResultEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedTextSourceResolveResultEventArgs {}
impl ::core::clone::Clone for TimedTextSourceResolveResultEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextStyle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedTextStyle {}
impl ::core::clone::Clone for TimedTextStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextSubformat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedTextSubformat {}
impl ::core::clone::Clone for TimedTextSubformat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextUnit(pub i32);
impl TimedTextUnit {
    pub const Pixels: Self = Self(0i32);
    pub const Percentage: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextUnit {}
impl ::core::clone::Clone for TimedTextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextWeight(pub i32);
impl TimedTextWeight {
    pub const Normal: Self = Self(400i32);
    pub const Bold: Self = Self(700i32);
}
impl ::core::marker::Copy for TimedTextWeight {}
impl ::core::clone::Clone for TimedTextWeight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedTextWrapping(pub i32);
impl TimedTextWrapping {
    pub const NoWrap: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextWrapping {}
impl ::core::clone::Clone for TimedTextWrapping {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for TimedTextWritingMode {}
impl ::core::clone::Clone for TimedTextWritingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoStabilizationEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoStabilizationEffect {}
impl ::core::clone::Clone for VideoStabilizationEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoStabilizationEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoStabilizationEffectDefinition {}
impl ::core::clone::Clone for VideoStabilizationEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoStabilizationEffectEnabledChangedEventArgs {}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedReason(pub i32);
impl VideoStabilizationEffectEnabledChangedReason {
    pub const Programmatic: Self = Self(0i32);
    pub const PixelRateTooHigh: Self = Self(1i32);
    pub const RunningSlowly: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoStabilizationEffectEnabledChangedReason {}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoStreamDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoStreamDescriptor {}
impl ::core::clone::Clone for VideoStreamDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoTrack {}
impl ::core::clone::Clone for VideoTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoTrackOpenFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoTrackOpenFailedEventArgs {}
impl ::core::clone::Clone for VideoTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoTrackSupportInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoTrackSupportInfo {}
impl ::core::clone::Clone for VideoTrackSupportInfo {
    fn clone(&self) -> Self {
        *self
    }
}
