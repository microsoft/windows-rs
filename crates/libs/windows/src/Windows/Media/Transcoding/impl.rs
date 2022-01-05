#[cfg(feature = "implement_exclusive")]
pub trait IMediaTranscoderImpl: Sized {
    fn SetTrimStartTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimStartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimStopTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimStopTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetAlwaysReencode(&self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysReencode(&self) -> ::windows::core::Result<bool>;
    fn SetHardwareAccelerationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn HardwareAccelerationEnabled(&self) -> ::windows::core::Result<bool>;
    fn AddAudioEffect(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddAudioEffectWithSettings(&self, activatableclassid: &::windows::core::HSTRING, effectrequired: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn AddVideoEffect(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddVideoEffectWithSettings(&self, activatableclassid: &::windows::core::HSTRING, effectrequired: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn ClearEffects(&self) -> ::windows::core::Result<()>;
    fn PrepareFileTranscodeAsync(&self, source: &::core::option::Option<super::super::Storage::IStorageFile>, destination: &::core::option::Option<super::super::Storage::IStorageFile>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
    fn PrepareStreamTranscodeAsync(&self, source: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, destination: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTranscoder2Impl: Sized {
    fn PrepareMediaStreamSourceTranscodeAsync(&self, source: &::core::option::Option<super::Core::IMediaSource>, destination: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
    fn SetVideoProcessingAlgorithm(&self, value: MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()>;
    fn VideoProcessingAlgorithm(&self) -> ::windows::core::Result<MediaVideoProcessingAlgorithm>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrepareTranscodeResultImpl: Sized {
    fn CanTranscode(&self) -> ::windows::core::Result<bool>;
    fn FailureReason(&self) -> ::windows::core::Result<TranscodeFailureReason>;
    fn TranscodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<f64>>;
}
