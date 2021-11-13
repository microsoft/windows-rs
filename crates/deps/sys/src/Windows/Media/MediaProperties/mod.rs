#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioEncodingProperties {}
impl ::core::clone::Clone for AudioEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioEncodingQuality(pub i32);
impl AudioEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Medium: Self = Self(2i32);
    pub const Low: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioEncodingQuality {}
impl ::core::clone::Clone for AudioEncodingQuality {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContainerEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContainerEncodingProperties {}
impl ::core::clone::Clone for ContainerEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEncodingProperties {}
impl ::core::clone::Clone for IAudioEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEncodingProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEncodingProperties2 {}
impl ::core::clone::Clone for IAudioEncodingProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEncodingProperties3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEncodingProperties3 {}
impl ::core::clone::Clone for IAudioEncodingProperties3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEncodingPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEncodingPropertiesStatics {}
impl ::core::clone::Clone for IAudioEncodingPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEncodingPropertiesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEncodingPropertiesStatics2 {}
impl ::core::clone::Clone for IAudioEncodingPropertiesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEncodingPropertiesWithFormatUserData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEncodingPropertiesWithFormatUserData {}
impl ::core::clone::Clone for IAudioEncodingPropertiesWithFormatUserData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContainerEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContainerEncodingProperties {}
impl ::core::clone::Clone for IContainerEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContainerEncodingProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContainerEncodingProperties2 {}
impl ::core::clone::Clone for IContainerEncodingProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IH264ProfileIdsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IH264ProfileIdsStatics {}
impl ::core::clone::Clone for IH264ProfileIdsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageEncodingProperties {}
impl ::core::clone::Clone for IImageEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageEncodingProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageEncodingProperties2 {}
impl ::core::clone::Clone for IImageEncodingProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageEncodingPropertiesStatics {}
impl ::core::clone::Clone for IImageEncodingPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageEncodingPropertiesStatics2 {}
impl ::core::clone::Clone for IImageEncodingPropertiesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageEncodingPropertiesStatics3 {}
impl ::core::clone::Clone for IImageEncodingPropertiesStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingProfile {}
impl ::core::clone::Clone for IMediaEncodingProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingProfile2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingProfile2 {}
impl ::core::clone::Clone for IMediaEncodingProfile2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingProfile3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingProfile3 {}
impl ::core::clone::Clone for IMediaEncodingProfile3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingProfileStatics {}
impl ::core::clone::Clone for IMediaEncodingProfileStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingProfileStatics2 {}
impl ::core::clone::Clone for IMediaEncodingProfileStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingProfileStatics3 {}
impl ::core::clone::Clone for IMediaEncodingProfileStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingProperties {}
impl ::core::clone::Clone for IMediaEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingSubtypesStatics {}
impl ::core::clone::Clone for IMediaEncodingSubtypesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingSubtypesStatics2 {}
impl ::core::clone::Clone for IMediaEncodingSubtypesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingSubtypesStatics3 {}
impl ::core::clone::Clone for IMediaEncodingSubtypesStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingSubtypesStatics4 {}
impl ::core::clone::Clone for IMediaEncodingSubtypesStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingSubtypesStatics5 {}
impl ::core::clone::Clone for IMediaEncodingSubtypesStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEncodingSubtypesStatics6 {}
impl ::core::clone::Clone for IMediaEncodingSubtypesStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaRatio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaRatio {}
impl ::core::clone::Clone for IMediaRatio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMpeg2ProfileIdsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMpeg2ProfileIdsStatics {}
impl ::core::clone::Clone for IMpeg2ProfileIdsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataEncodingProperties {}
impl ::core::clone::Clone for ITimedMetadataEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataEncodingPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataEncodingPropertiesStatics {}
impl ::core::clone::Clone for ITimedMetadataEncodingPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEncodingProperties {}
impl ::core::clone::Clone for IVideoEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEncodingProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEncodingProperties2 {}
impl ::core::clone::Clone for IVideoEncodingProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEncodingProperties3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEncodingProperties3 {}
impl ::core::clone::Clone for IVideoEncodingProperties3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEncodingProperties4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEncodingProperties4 {}
impl ::core::clone::Clone for IVideoEncodingProperties4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEncodingProperties5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEncodingProperties5 {}
impl ::core::clone::Clone for IVideoEncodingProperties5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEncodingPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEncodingPropertiesStatics {}
impl ::core::clone::Clone for IVideoEncodingPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEncodingPropertiesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEncodingPropertiesStatics2 {}
impl ::core::clone::Clone for IVideoEncodingPropertiesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageEncodingProperties {}
impl ::core::clone::Clone for ImageEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaEncodingProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaEncodingProfile {}
impl ::core::clone::Clone for MediaEncodingProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaMirroringOptions(pub u32);
impl MediaMirroringOptions {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl ::core::marker::Copy for MediaMirroringOptions {}
impl ::core::clone::Clone for MediaMirroringOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPixelFormat(pub i32);
impl MediaPixelFormat {
    pub const Nv12: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
    pub const P010: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPixelFormat {}
impl ::core::clone::Clone for MediaPixelFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPropertySet {}
impl ::core::clone::Clone for MediaPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaRatio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaRatio {}
impl ::core::clone::Clone for MediaRatio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaRotation(pub i32);
impl MediaRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaRotation {}
impl ::core::clone::Clone for MediaRotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaThumbnailFormat(pub i32);
impl MediaThumbnailFormat {
    pub const Bmp: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaThumbnailFormat {}
impl ::core::clone::Clone for MediaThumbnailFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SphericalVideoFrameFormat(pub i32);
impl SphericalVideoFrameFormat {
    pub const None: Self = Self(0i32);
    pub const Unsupported: Self = Self(1i32);
    pub const Equirectangular: Self = Self(2i32);
}
impl ::core::marker::Copy for SphericalVideoFrameFormat {}
impl ::core::clone::Clone for SphericalVideoFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StereoscopicVideoPackingMode(pub i32);
impl StereoscopicVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
impl ::core::marker::Copy for StereoscopicVideoPackingMode {}
impl ::core::clone::Clone for StereoscopicVideoPackingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedMetadataEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedMetadataEncodingProperties {}
impl ::core::clone::Clone for TimedMetadataEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoEncodingProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoEncodingProperties {}
impl ::core::clone::Clone for VideoEncodingProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoEncodingQuality(pub i32);
impl VideoEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const HD1080p: Self = Self(1i32);
    pub const HD720p: Self = Self(2i32);
    pub const Wvga: Self = Self(3i32);
    pub const Ntsc: Self = Self(4i32);
    pub const Pal: Self = Self(5i32);
    pub const Vga: Self = Self(6i32);
    pub const Qvga: Self = Self(7i32);
    pub const Uhd2160p: Self = Self(8i32);
    pub const Uhd4320p: Self = Self(9i32);
}
impl ::core::marker::Copy for VideoEncodingQuality {}
impl ::core::clone::Clone for VideoEncodingQuality {
    fn clone(&self) -> Self {
        *self
    }
}
