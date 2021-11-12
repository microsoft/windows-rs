#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct SpeechAppendedSilence(i32);
#[repr(C)]
pub struct SpeechPunctuationSilence(i32);
#[repr(transparent)]
pub struct SpeechSynthesisStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechSynthesizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpeechSynthesizerOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VoiceGender(i32);
#[repr(transparent)]
pub struct VoiceInformation(pub *mut ::core::ffi::c_void);
