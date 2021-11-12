#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IMediaTranscoder(pub *mut ::core::ffi::c_void);
pub struct IMediaTranscoder2(pub *mut ::core::ffi::c_void);
pub struct IPrepareTranscodeResult(pub *mut ::core::ffi::c_void);
pub struct MediaTranscoder(i32);
pub struct MediaVideoProcessingAlgorithm(i32);
pub struct PrepareTranscodeResult(i32);
pub struct TranscodeFailureReason(i32);
