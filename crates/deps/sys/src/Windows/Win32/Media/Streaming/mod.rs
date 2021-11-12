#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CapturedMetadataExposureCompensation(i32);
pub struct CapturedMetadataISOGains(i32);
pub struct CapturedMetadataWhiteBalanceGains(i32);
pub struct FaceCharacterization(i32);
pub struct FaceCharacterizationBlobHeader(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FaceRectInfo(i32);
pub struct FaceRectInfoBlobHeader(i32);
pub struct HistogramBlobHeader(i32);
pub struct HistogramDataHeader(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct HistogramGrid(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct HistogramHeader(i32);
pub struct IMFDeviceTransform(pub *mut ::core::ffi::c_void);
pub struct IMFDeviceTransformCallback(pub *mut ::core::ffi::c_void);
pub struct MF_MEDIASOURCE_STATUS_INFO(i32);
pub struct MF_TRANSFER_VIDEO_FRAME_FLAGS(i32);
pub struct MetadataTimeStamps(i32);
