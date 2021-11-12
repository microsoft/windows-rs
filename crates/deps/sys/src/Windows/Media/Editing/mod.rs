#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct MediaTrimmingPreference(pub i32);
impl MediaTrimmingPreference {
    pub const Fast: Self = Self(0i32);
    pub const Precise: Self = Self(1i32);
}
#[repr(transparent)]
pub struct VideoFramePrecision(pub i32);
impl VideoFramePrecision {
    pub const NearestFrame: Self = Self(0i32);
    pub const NearestKeyFrame: Self = Self(1i32);
}
