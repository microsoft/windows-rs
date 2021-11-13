#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DetectedFace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DetectedFace {}
impl ::core::clone::Clone for DetectedFace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FaceDetector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FaceDetector {}
impl ::core::clone::Clone for FaceDetector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FaceTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FaceTracker {}
impl ::core::clone::Clone for FaceTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDetectedFace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDetectedFace {}
impl ::core::clone::Clone for IDetectedFace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFaceDetector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFaceDetector {}
impl ::core::clone::Clone for IFaceDetector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFaceDetectorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFaceDetectorStatics {}
impl ::core::clone::Clone for IFaceDetectorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFaceTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFaceTracker {}
impl ::core::clone::Clone for IFaceTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFaceTrackerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFaceTrackerStatics {}
impl ::core::clone::Clone for IFaceTrackerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
