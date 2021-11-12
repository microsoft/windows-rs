#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct SpeechRecognitionUIStatus(pub i32);
impl SpeechRecognitionUIStatus {
    pub const Succeeded: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(0i32);
    pub const Busy: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(1i32);
    pub const Cancelled: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(2i32);
    pub const Preempted: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(3i32);
    pub const PrivacyPolicyDeclined: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(4i32);
}
