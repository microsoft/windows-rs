#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IMediaTranscoder(i32);
pub struct IMediaTranscoder2(i32);
pub struct IPrepareTranscodeResult(i32);
pub struct MediaTranscoder(i32);
pub struct MediaVideoProcessingAlgorithm(i32);
pub struct PrepareTranscodeResult(i32);
pub struct TranscodeFailureReason(i32);
