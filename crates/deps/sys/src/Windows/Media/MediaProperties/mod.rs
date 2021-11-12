#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioEncodingQuality(i32);
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
#[repr(C)]
pub struct MediaMirroringOptions(i32);
#[repr(C)]
pub struct MediaPixelFormat(i32);
#[repr(transparent)]
pub struct MediaPropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaRatio(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaRotation(i32);
#[repr(C)]
pub struct MediaThumbnailFormat(i32);
#[repr(C)]
pub struct SphericalVideoFrameFormat(i32);
#[repr(C)]
pub struct StereoscopicVideoPackingMode(i32);
#[repr(transparent)]
pub struct TimedMetadataEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoEncodingProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VideoEncodingQuality(i32);
