#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioDeviceInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioDeviceInputNode {}
impl ::core::clone::Clone for AudioDeviceInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioDeviceNodeCreationStatus {}
impl ::core::clone::Clone for AudioDeviceNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDeviceOutputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioDeviceOutputNode {}
impl ::core::clone::Clone for AudioDeviceOutputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioFileInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioFileInputNode {}
impl ::core::clone::Clone for AudioFileInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FileNotFound: Self = Self(1i32);
    pub const InvalidFileType: Self = Self(2i32);
    pub const FormatNotSupported: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioFileNodeCreationStatus {}
impl ::core::clone::Clone for AudioFileNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioFileOutputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioFileOutputNode {}
impl ::core::clone::Clone for AudioFileOutputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioFrameCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioFrameCompletedEventArgs {}
impl ::core::clone::Clone for AudioFrameCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioFrameInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioFrameInputNode {}
impl ::core::clone::Clone for AudioFrameInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioFrameOutputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioFrameOutputNode {}
impl ::core::clone::Clone for AudioFrameOutputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioGraph(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioGraph {}
impl ::core::clone::Clone for AudioGraph {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioGraphBatchUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioGraphBatchUpdater {}
impl ::core::clone::Clone for AudioGraphBatchUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioGraphConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioGraphConnection {}
impl ::core::clone::Clone for AudioGraphConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphCreationStatus {}
impl ::core::clone::Clone for AudioGraphCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioGraphSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioGraphSettings {}
impl ::core::clone::Clone for AudioGraphSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: Self = Self(0i32);
    pub const AudioDeviceLost: Self = Self(1i32);
    pub const AudioSessionDisconnected: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphUnrecoverableError {}
impl ::core::clone::Clone for AudioGraphUnrecoverableError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioGraphUnrecoverableErrorOccurredEventArgs {}
impl ::core::clone::Clone for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioNodeEmitter {}
impl ::core::clone::Clone for AudioNodeEmitter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterConeProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioNodeEmitterConeProperties {}
impl ::core::clone::Clone for AudioNodeEmitterConeProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterDecayKind {}
impl ::core::clone::Clone for AudioNodeEmitterDecayKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterDecayModel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioNodeEmitterDecayModel {}
impl ::core::clone::Clone for AudioNodeEmitterDecayModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioNodeEmitterNaturalDecayModelProperties {}
impl ::core::clone::Clone for AudioNodeEmitterNaturalDecayModelProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: Self = Self(0u32);
    pub const DisableDoppler: Self = Self(1u32);
}
impl ::core::marker::Copy for AudioNodeEmitterSettings {}
impl ::core::clone::Clone for AudioNodeEmitterSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioNodeEmitterShape {}
impl ::core::clone::Clone for AudioNodeEmitterShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: Self = Self(0i32);
    pub const Cone: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterShapeKind {}
impl ::core::clone::Clone for AudioNodeEmitterShapeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioNodeListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioNodeListener {}
impl ::core::clone::Clone for AudioNodeListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioPlaybackConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioPlaybackConnection {}
impl ::core::clone::Clone for AudioPlaybackConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioPlaybackConnectionOpenResult {}
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: Self = Self(0i32);
    pub const RequestTimedOut: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionOpenResultStatus {}
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionState {}
impl ::core::clone::Clone for AudioPlaybackConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioStateMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioStateMonitor {}
impl ::core::clone::Clone for AudioStateMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioSubmixNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioSubmixNode {}
impl ::core::clone::Clone for AudioSubmixNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CreateAudioDeviceInputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CreateAudioDeviceInputNodeResult {}
impl ::core::clone::Clone for CreateAudioDeviceInputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CreateAudioDeviceOutputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CreateAudioDeviceOutputNodeResult {}
impl ::core::clone::Clone for CreateAudioDeviceOutputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CreateAudioFileInputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CreateAudioFileInputNodeResult {}
impl ::core::clone::Clone for CreateAudioFileInputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CreateAudioFileOutputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CreateAudioFileOutputNodeResult {}
impl ::core::clone::Clone for CreateAudioFileOutputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CreateAudioGraphResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CreateAudioGraphResult {}
impl ::core::clone::Clone for CreateAudioGraphResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CreateMediaSourceAudioInputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CreateMediaSourceAudioInputNodeResult {}
impl ::core::clone::Clone for CreateMediaSourceAudioInputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EchoEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EchoEffectDefinition {}
impl ::core::clone::Clone for EchoEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EqualizerBand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EqualizerBand {}
impl ::core::clone::Clone for EqualizerBand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EqualizerEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EqualizerEffectDefinition {}
impl ::core::clone::Clone for EqualizerEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameInputNodeQuantumStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameInputNodeQuantumStartedEventArgs {}
impl ::core::clone::Clone for FrameInputNodeQuantumStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceInputNode {}
impl ::core::clone::Clone for IAudioDeviceInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceOutputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceOutputNode {}
impl ::core::clone::Clone for IAudioDeviceOutputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioFileInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFileInputNode {}
impl ::core::clone::Clone for IAudioFileInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioFileOutputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFileOutputNode {}
impl ::core::clone::Clone for IAudioFileOutputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioFrameCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFrameCompletedEventArgs {}
impl ::core::clone::Clone for IAudioFrameCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioFrameInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFrameInputNode {}
impl ::core::clone::Clone for IAudioFrameInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioFrameOutputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFrameOutputNode {}
impl ::core::clone::Clone for IAudioFrameOutputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraph(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraph {}
impl ::core::clone::Clone for IAudioGraph {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraph2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraph2 {}
impl ::core::clone::Clone for IAudioGraph2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraph3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraph3 {}
impl ::core::clone::Clone for IAudioGraph3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraphConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraphConnection {}
impl ::core::clone::Clone for IAudioGraphConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraphSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraphSettings {}
impl ::core::clone::Clone for IAudioGraphSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraphSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraphSettings2 {}
impl ::core::clone::Clone for IAudioGraphSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraphSettingsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraphSettingsFactory {}
impl ::core::clone::Clone for IAudioGraphSettingsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraphStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraphStatics {}
impl ::core::clone::Clone for IAudioGraphStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioGraphUnrecoverableErrorOccurredEventArgs {}
impl ::core::clone::Clone for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioInputNode {}
impl ::core::clone::Clone for IAudioInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioInputNode2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioInputNode2 {}
impl ::core::clone::Clone for IAudioInputNode2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNode {}
impl ::core::clone::Clone for IAudioNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitter {}
impl ::core::clone::Clone for IAudioNodeEmitter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitter2 {}
impl ::core::clone::Clone for IAudioNodeEmitter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitterConeProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitterConeProperties {}
impl ::core::clone::Clone for IAudioNodeEmitterConeProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitterDecayModel {}
impl ::core::clone::Clone for IAudioNodeEmitterDecayModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitterDecayModelStatics {}
impl ::core::clone::Clone for IAudioNodeEmitterDecayModelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitterFactory {}
impl ::core::clone::Clone for IAudioNodeEmitterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitterNaturalDecayModelProperties {}
impl ::core::clone::Clone for IAudioNodeEmitterNaturalDecayModelProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitterShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitterShape {}
impl ::core::clone::Clone for IAudioNodeEmitterShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeEmitterShapeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeEmitterShapeStatics {}
impl ::core::clone::Clone for IAudioNodeEmitterShapeStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeListener {}
impl ::core::clone::Clone for IAudioNodeListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioNodeWithListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioNodeWithListener {}
impl ::core::clone::Clone for IAudioNodeWithListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioPlaybackConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioPlaybackConnection {}
impl ::core::clone::Clone for IAudioPlaybackConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioPlaybackConnectionOpenResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioPlaybackConnectionOpenResult {}
impl ::core::clone::Clone for IAudioPlaybackConnectionOpenResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioPlaybackConnectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioPlaybackConnectionStatics {}
impl ::core::clone::Clone for IAudioPlaybackConnectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioStateMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioStateMonitor {}
impl ::core::clone::Clone for IAudioStateMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioStateMonitorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioStateMonitorStatics {}
impl ::core::clone::Clone for IAudioStateMonitorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioDeviceInputNodeResult {}
impl ::core::clone::Clone for ICreateAudioDeviceInputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioDeviceInputNodeResult2 {}
impl ::core::clone::Clone for ICreateAudioDeviceInputNodeResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioDeviceOutputNodeResult {}
impl ::core::clone::Clone for ICreateAudioDeviceOutputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioDeviceOutputNodeResult2 {}
impl ::core::clone::Clone for ICreateAudioDeviceOutputNodeResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioFileInputNodeResult {}
impl ::core::clone::Clone for ICreateAudioFileInputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioFileInputNodeResult2 {}
impl ::core::clone::Clone for ICreateAudioFileInputNodeResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioFileOutputNodeResult {}
impl ::core::clone::Clone for ICreateAudioFileOutputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioFileOutputNodeResult2 {}
impl ::core::clone::Clone for ICreateAudioFileOutputNodeResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioGraphResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioGraphResult {}
impl ::core::clone::Clone for ICreateAudioGraphResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateAudioGraphResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateAudioGraphResult2 {}
impl ::core::clone::Clone for ICreateAudioGraphResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateMediaSourceAudioInputNodeResult {}
impl ::core::clone::Clone for ICreateMediaSourceAudioInputNodeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateMediaSourceAudioInputNodeResult2 {}
impl ::core::clone::Clone for ICreateMediaSourceAudioInputNodeResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEchoEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEchoEffectDefinition {}
impl ::core::clone::Clone for IEchoEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEchoEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEchoEffectDefinitionFactory {}
impl ::core::clone::Clone for IEchoEffectDefinitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEqualizerBand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEqualizerBand {}
impl ::core::clone::Clone for IEqualizerBand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEqualizerEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEqualizerEffectDefinition {}
impl ::core::clone::Clone for IEqualizerEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEqualizerEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEqualizerEffectDefinitionFactory {}
impl ::core::clone::Clone for IEqualizerEffectDefinitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameInputNodeQuantumStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameInputNodeQuantumStartedEventArgs {}
impl ::core::clone::Clone for IFrameInputNodeQuantumStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILimiterEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILimiterEffectDefinition {}
impl ::core::clone::Clone for ILimiterEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILimiterEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILimiterEffectDefinitionFactory {}
impl ::core::clone::Clone for ILimiterEffectDefinitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaSourceAudioInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaSourceAudioInputNode {}
impl ::core::clone::Clone for IMediaSourceAudioInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReverbEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReverbEffectDefinition {}
impl ::core::clone::Clone for IReverbEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReverbEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReverbEffectDefinitionFactory {}
impl ::core::clone::Clone for IReverbEffectDefinitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetDefaultSpatialAudioFormatResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetDefaultSpatialAudioFormatResult {}
impl ::core::clone::Clone for ISetDefaultSpatialAudioFormatResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAudioDeviceConfiguration {}
impl ::core::clone::Clone for ISpatialAudioDeviceConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfigurationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAudioDeviceConfigurationStatics {}
impl ::core::clone::Clone for ISpatialAudioDeviceConfigurationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAudioFormatConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAudioFormatConfiguration {}
impl ::core::clone::Clone for ISpatialAudioFormatConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAudioFormatConfigurationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAudioFormatConfigurationStatics {}
impl ::core::clone::Clone for ISpatialAudioFormatConfigurationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAudioFormatSubtypeStatics {}
impl ::core::clone::Clone for ISpatialAudioFormatSubtypeStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAudioFormatSubtypeStatics2 {}
impl ::core::clone::Clone for ISpatialAudioFormatSubtypeStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LimiterEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LimiterEffectDefinition {}
impl ::core::clone::Clone for LimiterEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSourceAudioInputNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaSourceAudioInputNode {}
impl ::core::clone::Clone for MediaSourceAudioInputNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaSourceAudioInputNodeCreationStatus {}
impl ::core::clone::Clone for MediaSourceAudioInputNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: Self = Self(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: Self = Self(1i32);
}
impl ::core::marker::Copy for MixedRealitySpatialAudioFormatPolicy {}
impl ::core::clone::Clone for MixedRealitySpatialAudioFormatPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: Self = Self(0i32);
    pub const LowestLatency: Self = Self(1i32);
    pub const ClosestToDesired: Self = Self(2i32);
}
impl ::core::marker::Copy for QuantumSizeSelectionMode {}
impl ::core::clone::Clone for QuantumSizeSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ReverbEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ReverbEffectDefinition {}
impl ::core::clone::Clone for ReverbEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SetDefaultSpatialAudioFormatResult {}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const LicenseExpired: Self = Self(2i32);
    pub const LicenseNotValidForAudioEndpoint: Self = Self(3i32);
    pub const NotSupportedOnAudioEndpoint: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl ::core::marker::Copy for SetDefaultSpatialAudioFormatStatus {}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAudioDeviceConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialAudioDeviceConfiguration {}
impl ::core::clone::Clone for SpatialAudioDeviceConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAudioFormatConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialAudioFormatConfiguration {}
impl ::core::clone::Clone for SpatialAudioFormatConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: Self = Self(0i32);
    pub const FoldDown: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAudioModel {}
impl ::core::clone::Clone for SpatialAudioModel {
    fn clone(&self) -> Self {
        *self
    }
}
