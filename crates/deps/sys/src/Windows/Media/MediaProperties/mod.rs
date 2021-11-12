#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioEncodingQuality(pub i32);
impl AudioEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Medium: Self = Self(2i32);
    pub const Low: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ContainerEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEncodingProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEncodingProperties3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEncodingPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEncodingPropertiesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEncodingPropertiesWithFormatUserData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContainerEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContainerEncodingProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IH264ProfileIdsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageEncodingProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageEncodingPropertiesStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingProfile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingProfile3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingProfileStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEncodingSubtypesStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaRatio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMpeg2ProfileIdsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataEncodingPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEncodingProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEncodingProperties3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEncodingProperties4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEncodingProperties5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEncodingPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEncodingPropertiesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaEncodingProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaMirroringOptions(pub u32);
impl MediaMirroringOptions {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
#[repr(transparent)]
pub struct MediaPixelFormat(pub i32);
impl MediaPixelFormat {
    pub const Nv12: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
    pub const P010: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MediaPropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaRatio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaRotation(pub i32);
impl MediaRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
#[repr(transparent)]
pub struct MediaThumbnailFormat(pub i32);
impl MediaThumbnailFormat {
    pub const Bmp: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
}
#[repr(transparent)]
pub struct SphericalVideoFrameFormat(pub i32);
impl SphericalVideoFrameFormat {
    pub const None: Self = Self(0i32);
    pub const Unsupported: Self = Self(1i32);
    pub const Equirectangular: Self = Self(2i32);
}
#[repr(transparent)]
pub struct StereoscopicVideoPackingMode(pub i32);
impl StereoscopicVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
#[repr(transparent)]
pub struct TimedMetadataEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoEncodingProperties(pub *mut ::core::ffi::c_void);
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
