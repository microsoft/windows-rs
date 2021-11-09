#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_Speech_Recognition`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechRecognitionUIStatus(pub i32);
impl SpeechRecognitionUIStatus {
    pub const Succeeded: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(0i32);
    pub const Busy: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(1i32);
    pub const Cancelled: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(2i32);
    pub const Preempted: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(3i32);
    pub const PrivacyPolicyDeclined: SpeechRecognitionUIStatus = SpeechRecognitionUIStatus(4i32);
}
impl ::core::convert::From<i32> for SpeechRecognitionUIStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechRecognitionUIStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionUIStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Speech.Recognition.SpeechRecognitionUIStatus;i4)");
}
impl ::windows::runtime::DefaultType for SpeechRecognitionUIStatus {
    type DefaultType = Self;
}
