#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CameraIntrinsics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraIntrinsics {}
impl ::core::clone::Clone for CameraIntrinsics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DepthCorrelatedCoordinateMapper {}
impl ::core::clone::Clone for DepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameControlCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameControlCapabilities {}
impl ::core::clone::Clone for FrameControlCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameController {}
impl ::core::clone::Clone for FrameController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameExposureCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameExposureCapabilities {}
impl ::core::clone::Clone for FrameExposureCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameExposureCompensationCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameExposureCompensationCapabilities {}
impl ::core::clone::Clone for FrameExposureCompensationCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameExposureCompensationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameExposureCompensationControl {}
impl ::core::clone::Clone for FrameExposureCompensationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameExposureControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameExposureControl {}
impl ::core::clone::Clone for FrameExposureControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameFlashCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameFlashCapabilities {}
impl ::core::clone::Clone for FrameFlashCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameFlashControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameFlashControl {}
impl ::core::clone::Clone for FrameFlashControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameFlashMode(pub i32);
impl FrameFlashMode {
    pub const Disable: Self = Self(0i32);
    pub const Enable: Self = Self(1i32);
    pub const Global: Self = Self(2i32);
}
impl ::core::marker::Copy for FrameFlashMode {}
impl ::core::clone::Clone for FrameFlashMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameFocusCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameFocusCapabilities {}
impl ::core::clone::Clone for FrameFocusCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameFocusControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameFocusControl {}
impl ::core::clone::Clone for FrameFocusControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameIsoSpeedCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameIsoSpeedCapabilities {}
impl ::core::clone::Clone for FrameIsoSpeedCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameIsoSpeedControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameIsoSpeedControl {}
impl ::core::clone::Clone for FrameIsoSpeedControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraIntrinsics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraIntrinsics {}
impl ::core::clone::Clone for ICameraIntrinsics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraIntrinsics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraIntrinsics2 {}
impl ::core::clone::Clone for ICameraIntrinsics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraIntrinsicsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraIntrinsicsFactory {}
impl ::core::clone::Clone for ICameraIntrinsicsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDepthCorrelatedCoordinateMapper {}
impl ::core::clone::Clone for IDepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameControlCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameControlCapabilities {}
impl ::core::clone::Clone for IFrameControlCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameControlCapabilities2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameControlCapabilities2 {}
impl ::core::clone::Clone for IFrameControlCapabilities2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameController {}
impl ::core::clone::Clone for IFrameController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameController2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameController2 {}
impl ::core::clone::Clone for IFrameController2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameExposureCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameExposureCapabilities {}
impl ::core::clone::Clone for IFrameExposureCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameExposureCompensationCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameExposureCompensationCapabilities {}
impl ::core::clone::Clone for IFrameExposureCompensationCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameExposureCompensationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameExposureCompensationControl {}
impl ::core::clone::Clone for IFrameExposureCompensationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameExposureControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameExposureControl {}
impl ::core::clone::Clone for IFrameExposureControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameFlashCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameFlashCapabilities {}
impl ::core::clone::Clone for IFrameFlashCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameFlashControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameFlashControl {}
impl ::core::clone::Clone for IFrameFlashControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameFocusCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameFocusCapabilities {}
impl ::core::clone::Clone for IFrameFocusCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameFocusControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameFocusControl {}
impl ::core::clone::Clone for IFrameFocusControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameIsoSpeedCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameIsoSpeedCapabilities {}
impl ::core::clone::Clone for IFrameIsoSpeedCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameIsoSpeedControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameIsoSpeedControl {}
impl ::core::clone::Clone for IFrameIsoSpeedControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVariablePhotoSequenceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVariablePhotoSequenceController {}
impl ::core::clone::Clone for IVariablePhotoSequenceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VariablePhotoSequenceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VariablePhotoSequenceController {}
impl ::core::clone::Clone for VariablePhotoSequenceController {
    fn clone(&self) -> Self {
        *self
    }
}
