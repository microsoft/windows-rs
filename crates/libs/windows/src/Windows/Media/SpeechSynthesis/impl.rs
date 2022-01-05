#[cfg(feature = "implement_exclusive")]
pub trait IInstalledVoicesStaticImpl: Sized {
    fn AllVoices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VoiceInformation>>;
    fn DefaultVoice(&self) -> ::windows::core::Result<VoiceInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstalledVoicesStatic2Impl: Sized {
    fn TrySetDefaultVoiceAsync(&self, voice: &::core::option::Option<VoiceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISpeechSynthesisStreamImpl: Sized + IClosableImpl + IContentTypeProviderImpl + IInputStreamImpl + IOutputStreamImpl + IRandomAccessStreamImpl + IRandomAccessStreamWithContentTypeImpl {
    fn Markers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::IMediaMarker>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizerImpl: Sized {
    fn SynthesizeTextToStreamAsync(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>;
    fn SynthesizeSsmlToStreamAsync(&self, ssml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>;
    fn SetVoice(&self, value: &::core::option::Option<VoiceInformation>) -> ::windows::core::Result<()>;
    fn Voice(&self) -> ::windows::core::Result<VoiceInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizer2Impl: Sized {
    fn Options(&self) -> ::windows::core::Result<SpeechSynthesizerOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizerOptionsImpl: Sized {
    fn IncludeWordBoundaryMetadata(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeWordBoundaryMetadata(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeSentenceBoundaryMetadata(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeSentenceBoundaryMetadata(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizerOptions2Impl: Sized {
    fn AudioVolume(&self) -> ::windows::core::Result<f64>;
    fn SetAudioVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn SpeakingRate(&self) -> ::windows::core::Result<f64>;
    fn SetSpeakingRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn AudioPitch(&self) -> ::windows::core::Result<f64>;
    fn SetAudioPitch(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizerOptions3Impl: Sized {
    fn AppendedSilence(&self) -> ::windows::core::Result<SpeechAppendedSilence>;
    fn SetAppendedSilence(&self, value: SpeechAppendedSilence) -> ::windows::core::Result<()>;
    fn PunctuationSilence(&self) -> ::windows::core::Result<SpeechPunctuationSilence>;
    fn SetPunctuationSilence(&self, value: SpeechPunctuationSilence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceInformationImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gender(&self) -> ::windows::core::Result<VoiceGender>;
}
