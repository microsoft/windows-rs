#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IInstalledVoicesStatic(pub *mut ::core::ffi::c_void);
pub struct IInstalledVoicesStatic2(pub *mut ::core::ffi::c_void);
pub struct ISpeechSynthesisStream(pub *mut ::core::ffi::c_void);
pub struct ISpeechSynthesizer(pub *mut ::core::ffi::c_void);
pub struct ISpeechSynthesizer2(pub *mut ::core::ffi::c_void);
pub struct ISpeechSynthesizerOptions(pub *mut ::core::ffi::c_void);
pub struct ISpeechSynthesizerOptions2(pub *mut ::core::ffi::c_void);
pub struct ISpeechSynthesizerOptions3(pub *mut ::core::ffi::c_void);
pub struct IVoiceInformation(pub *mut ::core::ffi::c_void);
pub struct SpeechAppendedSilence(i32);
pub struct SpeechPunctuationSilence(i32);
pub struct SpeechSynthesisStream(i32);
pub struct SpeechSynthesizer(i32);
pub struct SpeechSynthesizerOptions(i32);
pub struct VoiceGender(i32);
pub struct VoiceInformation(i32);
