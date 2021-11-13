#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioCaptureEffectsManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioCaptureEffectsManager {}
impl ::core::clone::Clone for AudioCaptureEffectsManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioEffect {}
impl ::core::clone::Clone for AudioEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioEffectDefinition {}
impl ::core::clone::Clone for AudioEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioEffectType(pub i32);
impl AudioEffectType {
    pub const Other: Self = Self(0i32);
    pub const AcousticEchoCancellation: Self = Self(1i32);
    pub const NoiseSuppression: Self = Self(2i32);
    pub const AutomaticGainControl: Self = Self(3i32);
    pub const BeamForming: Self = Self(4i32);
    pub const ConstantToneRemoval: Self = Self(5i32);
    pub const Equalizer: Self = Self(6i32);
    pub const LoudnessEqualizer: Self = Self(7i32);
    pub const BassBoost: Self = Self(8i32);
    pub const VirtualSurround: Self = Self(9i32);
    pub const VirtualHeadphones: Self = Self(10i32);
    pub const SpeakerFill: Self = Self(11i32);
    pub const RoomCorrection: Self = Self(12i32);
    pub const BassManagement: Self = Self(13i32);
    pub const EnvironmentalEffects: Self = Self(14i32);
    pub const SpeakerProtection: Self = Self(15i32);
    pub const SpeakerCompensation: Self = Self(16i32);
    pub const DynamicRangeCompression: Self = Self(17i32);
    pub const FarFieldBeamForming: Self = Self(18i32);
    pub const DeepNoiseSuppression: Self = Self(19i32);
}
impl ::core::marker::Copy for AudioEffectType {}
impl ::core::clone::Clone for AudioEffectType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioRenderEffectsManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioRenderEffectsManager {}
impl ::core::clone::Clone for AudioRenderEffectsManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositeVideoFrameContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositeVideoFrameContext {}
impl ::core::clone::Clone for CompositeVideoFrameContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioCaptureEffectsManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioCaptureEffectsManager {}
impl ::core::clone::Clone for IAudioCaptureEffectsManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEffect {}
impl ::core::clone::Clone for IAudioEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEffectDefinition {}
impl ::core::clone::Clone for IAudioEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEffectDefinitionFactory {}
impl ::core::clone::Clone for IAudioEffectDefinitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEffectsManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEffectsManagerStatics {}
impl ::core::clone::Clone for IAudioEffectsManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioRenderEffectsManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioRenderEffectsManager {}
impl ::core::clone::Clone for IAudioRenderEffectsManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioRenderEffectsManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioRenderEffectsManager2 {}
impl ::core::clone::Clone for IAudioRenderEffectsManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBasicAudioEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBasicAudioEffect {}
impl ::core::clone::Clone for IBasicAudioEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBasicVideoEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBasicVideoEffect {}
impl ::core::clone::Clone for IBasicVideoEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositeVideoFrameContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositeVideoFrameContext {}
impl ::core::clone::Clone for ICompositeVideoFrameContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessAudioFrameContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessAudioFrameContext {}
impl ::core::clone::Clone for IProcessAudioFrameContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessVideoFrameContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessVideoFrameContext {}
impl ::core::clone::Clone for IProcessVideoFrameContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlowMotionEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlowMotionEffectDefinition {}
impl ::core::clone::Clone for ISlowMotionEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoCompositor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoCompositor {}
impl ::core::clone::Clone for IVideoCompositor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoCompositorDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoCompositorDefinition {}
impl ::core::clone::Clone for IVideoCompositorDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoCompositorDefinitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoCompositorDefinitionFactory {}
impl ::core::clone::Clone for IVideoCompositorDefinitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEffectDefinition {}
impl ::core::clone::Clone for IVideoEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEffectDefinitionFactory {}
impl ::core::clone::Clone for IVideoEffectDefinitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoTransformEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoTransformEffectDefinition {}
impl ::core::clone::Clone for IVideoTransformEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoTransformEffectDefinition2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoTransformEffectDefinition2 {}
impl ::core::clone::Clone for IVideoTransformEffectDefinition2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoTransformSphericalProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoTransformSphericalProjection {}
impl ::core::clone::Clone for IVideoTransformSphericalProjection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaEffectClosedReason(pub i32);
impl MediaEffectClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const UnsupportedEncodingFormat: Self = Self(2i32);
    pub const EffectCurrentlyUnloaded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaEffectClosedReason {}
impl ::core::clone::Clone for MediaEffectClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaMemoryTypes(pub i32);
impl MediaMemoryTypes {
    pub const Gpu: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const GpuAndCpu: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaMemoryTypes {}
impl ::core::clone::Clone for MediaMemoryTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessAudioFrameContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessAudioFrameContext {}
impl ::core::clone::Clone for ProcessAudioFrameContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessVideoFrameContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessVideoFrameContext {}
impl ::core::clone::Clone for ProcessVideoFrameContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SlowMotionEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SlowMotionEffectDefinition {}
impl ::core::clone::Clone for SlowMotionEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoCompositorDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoCompositorDefinition {}
impl ::core::clone::Clone for VideoCompositorDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoEffectDefinition {}
impl ::core::clone::Clone for VideoEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoTransformEffectDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoTransformEffectDefinition {}
impl ::core::clone::Clone for VideoTransformEffectDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoTransformSphericalProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoTransformSphericalProjection {}
impl ::core::clone::Clone for VideoTransformSphericalProjection {
    fn clone(&self) -> Self {
        *self
    }
}
