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
    pub const Default: Self = Self(0i32);
    pub const MrfCrf444: Self = Self(1i32);
}
#[repr(transparent)]
pub struct PrepareTranscodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TranscodeFailureReason(pub i32);
impl TranscodeFailureReason {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidProfile: Self = Self(2i32);
    pub const CodecNotFound: Self = Self(3i32);
}
