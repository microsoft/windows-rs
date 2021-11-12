#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CameraIntrinsics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameControlCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameExposureCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameExposureCompensationCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameExposureCompensationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameExposureControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameFlashCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameFlashControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameFlashMode(pub i32);
impl FrameFlashMode {
    pub const Disable: FrameFlashMode = FrameFlashMode(0i32);
    pub const Enable: FrameFlashMode = FrameFlashMode(1i32);
    pub const Global: FrameFlashMode = FrameFlashMode(2i32);
}
#[repr(transparent)]
pub struct FrameFocusCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameFocusControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameIsoSpeedCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameIsoSpeedControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraIntrinsics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraIntrinsics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraIntrinsicsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameControlCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameControlCapabilities2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameController2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameExposureCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameExposureCompensationCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameExposureCompensationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameExposureControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameFlashCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameFlashControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameFocusCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameFocusControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameIsoSpeedCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameIsoSpeedControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVariablePhotoSequenceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VariablePhotoSequenceController(pub *mut ::core::ffi::c_void);
