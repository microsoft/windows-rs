impl ::core::default::Default for SpeechAppendedSilence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpeechAppendedSilence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechAppendedSilence").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpeechPunctuationSilence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpeechPunctuationSilence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechPunctuationSilence").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesisStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesisStream {}
impl ::core::fmt::Debug for SpeechSynthesisStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesisStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesizer {}
impl ::core::fmt::Debug for SpeechSynthesizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesizer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesizerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesizerOptions {}
impl ::core::fmt::Debug for SpeechSynthesizerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesizerOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for VoiceGender {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VoiceGender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceGender").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VoiceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceInformation {}
impl ::core::fmt::Debug for VoiceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceInformation").field(&self.0).finish()
    }
}
