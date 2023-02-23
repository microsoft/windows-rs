#[doc = "*Required features: `\"Phone_Speech_Recognition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SpeechRecognitionUIStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRecognitionUIStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionUIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionUIStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechRecognitionUIStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Speech.Recognition.SpeechRecognitionUIStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
