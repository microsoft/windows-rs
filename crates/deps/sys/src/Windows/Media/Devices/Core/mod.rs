#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CameraIntrinsics(i32);
pub struct DepthCorrelatedCoordinateMapper(i32);
pub struct FrameControlCapabilities(i32);
pub struct FrameController(i32);
pub struct FrameExposureCapabilities(i32);
pub struct FrameExposureCompensationCapabilities(i32);
pub struct FrameExposureCompensationControl(i32);
pub struct FrameExposureControl(i32);
pub struct FrameFlashCapabilities(i32);
pub struct FrameFlashControl(i32);
pub struct FrameFlashMode(i32);
pub struct FrameFocusCapabilities(i32);
pub struct FrameFocusControl(i32);
pub struct FrameIsoSpeedCapabilities(i32);
pub struct FrameIsoSpeedControl(i32);
pub struct ICameraIntrinsics(pub *mut ::core::ffi::c_void);
pub struct ICameraIntrinsics2(pub *mut ::core::ffi::c_void);
pub struct ICameraIntrinsicsFactory(pub *mut ::core::ffi::c_void);
pub struct IDepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
pub struct IFrameControlCapabilities(pub *mut ::core::ffi::c_void);
pub struct IFrameControlCapabilities2(pub *mut ::core::ffi::c_void);
pub struct IFrameController(pub *mut ::core::ffi::c_void);
pub struct IFrameController2(pub *mut ::core::ffi::c_void);
pub struct IFrameExposureCapabilities(pub *mut ::core::ffi::c_void);
pub struct IFrameExposureCompensationCapabilities(pub *mut ::core::ffi::c_void);
pub struct IFrameExposureCompensationControl(pub *mut ::core::ffi::c_void);
pub struct IFrameExposureControl(pub *mut ::core::ffi::c_void);
pub struct IFrameFlashCapabilities(pub *mut ::core::ffi::c_void);
pub struct IFrameFlashControl(pub *mut ::core::ffi::c_void);
pub struct IFrameFocusCapabilities(pub *mut ::core::ffi::c_void);
pub struct IFrameFocusControl(pub *mut ::core::ffi::c_void);
pub struct IFrameIsoSpeedCapabilities(pub *mut ::core::ffi::c_void);
pub struct IFrameIsoSpeedControl(pub *mut ::core::ffi::c_void);
pub struct IVariablePhotoSequenceController(pub *mut ::core::ffi::c_void);
pub struct VariablePhotoSequenceController(i32);
