#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BackgroundAudioTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundAudioTrack {}
impl ::core::clone::Clone for BackgroundAudioTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmbeddedAudioTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmbeddedAudioTrack {}
impl ::core::clone::Clone for EmbeddedAudioTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundAudioTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundAudioTrack {}
impl ::core::clone::Clone for IBackgroundAudioTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundAudioTrackStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundAudioTrackStatics {}
impl ::core::clone::Clone for IBackgroundAudioTrackStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmbeddedAudioTrack(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmbeddedAudioTrack {}
impl ::core::clone::Clone for IEmbeddedAudioTrack {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaClip {}
impl ::core::clone::Clone for IMediaClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaClipStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaClipStatics {}
impl ::core::clone::Clone for IMediaClipStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaClipStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaClipStatics2 {}
impl ::core::clone::Clone for IMediaClipStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaComposition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaComposition {}
impl ::core::clone::Clone for IMediaComposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaComposition2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaComposition2 {}
impl ::core::clone::Clone for IMediaComposition2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCompositionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCompositionStatics {}
impl ::core::clone::Clone for IMediaCompositionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaOverlay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaOverlay {}
impl ::core::clone::Clone for IMediaOverlay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaOverlayFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaOverlayFactory {}
impl ::core::clone::Clone for IMediaOverlayFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaOverlayLayer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaOverlayLayer {}
impl ::core::clone::Clone for IMediaOverlayLayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaOverlayLayerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaOverlayLayerFactory {}
impl ::core::clone::Clone for IMediaOverlayLayerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaClip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaClip {}
impl ::core::clone::Clone for MediaClip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaComposition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaComposition {}
impl ::core::clone::Clone for MediaComposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaOverlay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaOverlay {}
impl ::core::clone::Clone for MediaOverlay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaOverlayLayer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaOverlayLayer {}
impl ::core::clone::Clone for MediaOverlayLayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTrimmingPreference(pub i32);
impl MediaTrimmingPreference {
    pub const Fast: Self = Self(0i32);
    pub const Precise: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaTrimmingPreference {}
impl ::core::clone::Clone for MediaTrimmingPreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoFramePrecision(pub i32);
impl VideoFramePrecision {
    pub const NearestFrame: Self = Self(0i32);
    pub const NearestKeyFrame: Self = Self(1i32);
}
impl ::core::marker::Copy for VideoFramePrecision {}
impl ::core::clone::Clone for VideoFramePrecision {
    fn clone(&self) -> Self {
        *self
    }
}
