#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DetectedFace(i32);
pub struct FaceDetector(i32);
pub struct FaceTracker(i32);
pub struct IDetectedFace(pub *mut ::core::ffi::c_void);
pub struct IFaceDetector(pub *mut ::core::ffi::c_void);
pub struct IFaceDetectorStatics(pub *mut ::core::ffi::c_void);
pub struct IFaceTracker(pub *mut ::core::ffi::c_void);
pub struct IFaceTrackerStatics(pub *mut ::core::ffi::c_void);
