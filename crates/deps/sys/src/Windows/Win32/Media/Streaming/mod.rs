#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct MF_MEDIASOURCE_STATUS_INFO(i32);
#[repr(C)]
pub struct MF_TRANSFER_VIDEO_FRAME_FLAGS(i32);
#[repr(C)]
pub struct MetadataTimeStamps(i32);
