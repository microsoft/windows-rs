#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioCaptureEffectsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioEffectType(i32);
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
#[repr(C)]
pub struct MediaEffectClosedReason(i32);
#[repr(C)]
pub struct MediaMemoryTypes(i32);
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
