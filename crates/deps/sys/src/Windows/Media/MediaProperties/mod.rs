#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioEncodingQuality(pub i32);
impl AudioEncodingQuality {
    pub const Auto: AudioEncodingQuality = AudioEncodingQuality(0i32);
    pub const High: AudioEncodingQuality = AudioEncodingQuality(1i32);
    pub const Medium: AudioEncodingQuality = AudioEncodingQuality(2i32);
    pub const Low: AudioEncodingQuality = AudioEncodingQuality(3i32);
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
    pub const None: MediaMirroringOptions = MediaMirroringOptions(0u32);
    pub const Horizontal: MediaMirroringOptions = MediaMirroringOptions(1u32);
    pub const Vertical: MediaMirroringOptions = MediaMirroringOptions(2u32);
}
#[repr(transparent)]
pub struct MediaPixelFormat(pub i32);
impl MediaPixelFormat {
    pub const Nv12: MediaPixelFormat = MediaPixelFormat(0i32);
    pub const Bgra8: MediaPixelFormat = MediaPixelFormat(1i32);
    pub const P010: MediaPixelFormat = MediaPixelFormat(2i32);
}
#[repr(transparent)]
pub struct MediaPropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaRatio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaRotation(pub i32);
impl MediaRotation {
    pub const None: MediaRotation = MediaRotation(0i32);
    pub const Clockwise90Degrees: MediaRotation = MediaRotation(1i32);
    pub const Clockwise180Degrees: MediaRotation = MediaRotation(2i32);
    pub const Clockwise270Degrees: MediaRotation = MediaRotation(3i32);
}
#[repr(transparent)]
pub struct MediaThumbnailFormat(pub i32);
impl MediaThumbnailFormat {
    pub const Bmp: MediaThumbnailFormat = MediaThumbnailFormat(0i32);
    pub const Bgra8: MediaThumbnailFormat = MediaThumbnailFormat(1i32);
}
#[repr(transparent)]
pub struct SphericalVideoFrameFormat(pub i32);
impl SphericalVideoFrameFormat {
    pub const None: SphericalVideoFrameFormat = SphericalVideoFrameFormat(0i32);
    pub const Unsupported: SphericalVideoFrameFormat = SphericalVideoFrameFormat(1i32);
    pub const Equirectangular: SphericalVideoFrameFormat = SphericalVideoFrameFormat(2i32);
}
#[repr(transparent)]
pub struct StereoscopicVideoPackingMode(pub i32);
impl StereoscopicVideoPackingMode {
    pub const None: StereoscopicVideoPackingMode = StereoscopicVideoPackingMode(0i32);
    pub const SideBySide: StereoscopicVideoPackingMode = StereoscopicVideoPackingMode(1i32);
    pub const TopBottom: StereoscopicVideoPackingMode = StereoscopicVideoPackingMode(2i32);
}
#[repr(transparent)]
pub struct TimedMetadataEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoEncodingQuality(pub i32);
impl VideoEncodingQuality {
    pub const Auto: VideoEncodingQuality = VideoEncodingQuality(0i32);
    pub const HD1080p: VideoEncodingQuality = VideoEncodingQuality(1i32);
    pub const HD720p: VideoEncodingQuality = VideoEncodingQuality(2i32);
    pub const Wvga: VideoEncodingQuality = VideoEncodingQuality(3i32);
    pub const Ntsc: VideoEncodingQuality = VideoEncodingQuality(4i32);
    pub const Pal: VideoEncodingQuality = VideoEncodingQuality(5i32);
    pub const Vga: VideoEncodingQuality = VideoEncodingQuality(6i32);
    pub const Qvga: VideoEncodingQuality = VideoEncodingQuality(7i32);
    pub const Uhd2160p: VideoEncodingQuality = VideoEncodingQuality(8i32);
    pub const Uhd4320p: VideoEncodingQuality = VideoEncodingQuality(9i32);
}
