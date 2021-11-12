#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioCaptureEffectsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioEffectType(pub i32);
impl AudioEffectType {
    pub const Other: AudioEffectType = AudioEffectType(0i32);
    pub const AcousticEchoCancellation: AudioEffectType = AudioEffectType(1i32);
    pub const NoiseSuppression: AudioEffectType = AudioEffectType(2i32);
    pub const AutomaticGainControl: AudioEffectType = AudioEffectType(3i32);
    pub const BeamForming: AudioEffectType = AudioEffectType(4i32);
    pub const ConstantToneRemoval: AudioEffectType = AudioEffectType(5i32);
    pub const Equalizer: AudioEffectType = AudioEffectType(6i32);
    pub const LoudnessEqualizer: AudioEffectType = AudioEffectType(7i32);
    pub const BassBoost: AudioEffectType = AudioEffectType(8i32);
    pub const VirtualSurround: AudioEffectType = AudioEffectType(9i32);
    pub const VirtualHeadphones: AudioEffectType = AudioEffectType(10i32);
    pub const SpeakerFill: AudioEffectType = AudioEffectType(11i32);
    pub const RoomCorrection: AudioEffectType = AudioEffectType(12i32);
    pub const BassManagement: AudioEffectType = AudioEffectType(13i32);
    pub const EnvironmentalEffects: AudioEffectType = AudioEffectType(14i32);
    pub const SpeakerProtection: AudioEffectType = AudioEffectType(15i32);
    pub const SpeakerCompensation: AudioEffectType = AudioEffectType(16i32);
    pub const DynamicRangeCompression: AudioEffectType = AudioEffectType(17i32);
    pub const FarFieldBeamForming: AudioEffectType = AudioEffectType(18i32);
    pub const DeepNoiseSuppression: AudioEffectType = AudioEffectType(19i32);
}
#[repr(transparent)]
pub struct AudioRenderEffectsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositeVideoFrameContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioCaptureEffectsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEffectsManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioRenderEffectsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioRenderEffectsManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBasicAudioEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBasicVideoEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositeVideoFrameContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessAudioFrameContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessVideoFrameContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlowMotionEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoCompositor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoCompositorDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoCompositorDefinitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoTransformEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoTransformEffectDefinition2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoTransformSphericalProjection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaEffectClosedReason(pub i32);
impl MediaEffectClosedReason {
    pub const Done: MediaEffectClosedReason = MediaEffectClosedReason(0i32);
    pub const UnknownError: MediaEffectClosedReason = MediaEffectClosedReason(1i32);
    pub const UnsupportedEncodingFormat: MediaEffectClosedReason = MediaEffectClosedReason(2i32);
    pub const EffectCurrentlyUnloaded: MediaEffectClosedReason = MediaEffectClosedReason(3i32);
}
#[repr(transparent)]
pub struct MediaMemoryTypes(pub i32);
impl MediaMemoryTypes {
    pub const Gpu: MediaMemoryTypes = MediaMemoryTypes(0i32);
    pub const Cpu: MediaMemoryTypes = MediaMemoryTypes(1i32);
    pub const GpuAndCpu: MediaMemoryTypes = MediaMemoryTypes(2i32);
}
#[repr(transparent)]
pub struct ProcessAudioFrameContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessVideoFrameContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SlowMotionEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoCompositorDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTransformEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTransformSphericalProjection(pub *mut ::core::ffi::c_void);
