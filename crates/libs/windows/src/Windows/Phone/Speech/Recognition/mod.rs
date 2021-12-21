#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Phone_Speech_Recognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionUIStatus(pub i32);
impl SpeechRecognitionUIStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Busy: Self = Self(1i32);
    pub const Cancelled: Self = Self(2i32);
    pub const Preempted: Self = Self(3i32);
    pub const PrivacyPolicyDeclined: Self = Self(4i32);
}
impl ::core::marker::Copy for SpeechRecognitionUIStatus {}
impl ::core::clone::Clone for SpeechRecognitionUIStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechRecognitionUIStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechRecognitionUIStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionUIStatus {}
impl ::core::fmt::Debug for SpeechRecognitionUIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionUIStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionUIStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Speech.Recognition.SpeechRecognitionUIStatus;i4)");
}
impl ::windows::core::DefaultType for SpeechRecognitionUIStatus {
    type DefaultType = Self;
}
