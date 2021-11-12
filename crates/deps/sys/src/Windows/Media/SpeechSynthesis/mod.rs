#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInstalledVoicesStatic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstalledVoicesStatic2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechSynthesisStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechSynthesizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechSynthesizer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVoiceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechAppendedSilence(pub i32);
impl SpeechAppendedSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechAppendedSilence {}
impl ::core::clone::Clone for SpeechAppendedSilence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechPunctuationSilence(pub i32);
impl SpeechPunctuationSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechPunctuationSilence {}
impl ::core::clone::Clone for SpeechPunctuationSilence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechSynthesisStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechSynthesizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechSynthesizerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VoiceGender(pub i32);
impl VoiceGender {
    pub const Male: Self = Self(0i32);
    pub const Female: Self = Self(1i32);
}
impl ::core::marker::Copy for VoiceGender {}
impl ::core::clone::Clone for VoiceGender {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceInformation(pub *mut ::core::ffi::c_void);
