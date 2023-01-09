impl ::core::cmp::PartialEq for MediaTranscoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTranscoder {}
impl ::core::fmt::Debug for MediaTranscoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTranscoder").field(&self.0).finish()
    }
}
impl ::core::default::Default for MediaVideoProcessingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MediaVideoProcessingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaVideoProcessingAlgorithm").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrepareTranscodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrepareTranscodeResult {}
impl ::core::fmt::Debug for PrepareTranscodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrepareTranscodeResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for TranscodeFailureReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TranscodeFailureReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TranscodeFailureReason").field(&self.0).finish()
    }
}
