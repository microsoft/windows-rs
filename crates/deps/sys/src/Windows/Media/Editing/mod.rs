#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BackgroundAudioTrack(i32);
pub struct EmbeddedAudioTrack(i32);
pub struct IBackgroundAudioTrack(pub *mut ::core::ffi::c_void);
pub struct IBackgroundAudioTrackStatics(pub *mut ::core::ffi::c_void);
pub struct IEmbeddedAudioTrack(pub *mut ::core::ffi::c_void);
pub struct IMediaClip(pub *mut ::core::ffi::c_void);
pub struct IMediaClipStatics(pub *mut ::core::ffi::c_void);
pub struct IMediaClipStatics2(pub *mut ::core::ffi::c_void);
pub struct IMediaComposition(pub *mut ::core::ffi::c_void);
pub struct IMediaComposition2(pub *mut ::core::ffi::c_void);
pub struct IMediaCompositionStatics(pub *mut ::core::ffi::c_void);
pub struct IMediaOverlay(pub *mut ::core::ffi::c_void);
pub struct IMediaOverlayFactory(pub *mut ::core::ffi::c_void);
pub struct IMediaOverlayLayer(pub *mut ::core::ffi::c_void);
pub struct IMediaOverlayLayerFactory(pub *mut ::core::ffi::c_void);
pub struct MediaClip(i32);
pub struct MediaComposition(i32);
pub struct MediaOverlay(i32);
pub struct MediaOverlayLayer(i32);
pub struct MediaTrimmingPreference(i32);
pub struct VideoFramePrecision(i32);
