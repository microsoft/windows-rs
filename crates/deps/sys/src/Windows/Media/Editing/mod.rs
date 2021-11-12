#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BackgroundAudioTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmbeddedAudioTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundAudioTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundAudioTrackStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmbeddedAudioTrack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaClipStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaClipStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaComposition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaComposition2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCompositionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaOverlay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaOverlayFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaOverlayLayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaOverlayLayerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaComposition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaOverlay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaOverlayLayer(pub *mut ::core::ffi::c_void);
pub struct MediaTrimmingPreference(i32);
pub struct VideoFramePrecision(i32);
