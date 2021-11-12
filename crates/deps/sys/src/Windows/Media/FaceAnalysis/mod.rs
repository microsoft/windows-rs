#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DetectedFace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FaceDetector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FaceTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDetectedFace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaceDetector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaceDetectorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaceTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaceTrackerStatics(pub *mut ::core::ffi::c_void);
