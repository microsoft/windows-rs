#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMediaTranscoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTranscoder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrepareTranscodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaTranscoder(pub *mut ::core::ffi::c_void);
pub struct MediaVideoProcessingAlgorithm(i32);
#[repr(transparent)]
pub struct PrepareTranscodeResult(pub *mut ::core::ffi::c_void);
pub struct TranscodeFailureReason(i32);
