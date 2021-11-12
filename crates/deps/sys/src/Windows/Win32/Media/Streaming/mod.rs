#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CapturedMetadataExposureCompensation(i32);
#[repr(C)]
pub struct CapturedMetadataISOGains(i32);
#[repr(C)]
pub struct CapturedMetadataWhiteBalanceGains(i32);
#[repr(C)]
pub struct FaceCharacterization(i32);
#[repr(C)]
pub struct FaceCharacterizationBlobHeader(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FaceRectInfo(i32);
#[repr(C)]
pub struct FaceRectInfoBlobHeader(i32);
#[repr(C)]
pub struct HistogramBlobHeader(i32);
#[repr(C)]
pub struct HistogramDataHeader(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HistogramGrid(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HistogramHeader(i32);
#[repr(transparent)]
pub struct IMFDeviceTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMFDeviceTransformCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MF_MEDIASOURCE_STATUS_INFO(pub i32);
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(0i32);
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(1i32);
#[repr(transparent)]
pub struct MF_TRANSFER_VIDEO_FRAME_FLAGS(pub i32);
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(0i32);
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(1i32);
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(2i32);
#[repr(C)]
pub struct MetadataTimeStamps(i32);
