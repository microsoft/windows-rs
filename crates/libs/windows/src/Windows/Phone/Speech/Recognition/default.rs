impl ::core::default::Default for SpeechRecognitionUIStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpeechRecognitionUIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionUIStatus").field(&self.0).finish()
    }
}
