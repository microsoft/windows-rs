#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct MediaVideoProcessingAlgorithm(pub i32);
impl MediaVideoProcessingAlgorithm {
    pub const Default: MediaVideoProcessingAlgorithm = MediaVideoProcessingAlgorithm(0i32);
    pub const MrfCrf444: MediaVideoProcessingAlgorithm = MediaVideoProcessingAlgorithm(1i32);
}
#[repr(transparent)]
pub struct PrepareTranscodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TranscodeFailureReason(pub i32);
impl TranscodeFailureReason {
    pub const None: TranscodeFailureReason = TranscodeFailureReason(0i32);
    pub const Unknown: TranscodeFailureReason = TranscodeFailureReason(1i32);
    pub const InvalidProfile: TranscodeFailureReason = TranscodeFailureReason(2i32);
    pub const CodecNotFound: TranscodeFailureReason = TranscodeFailureReason(3i32);
}
