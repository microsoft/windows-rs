#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundAudioTrackImpl: Sized {
    fn TrimTimeFromStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromStart(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimTimeFromEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromEnd(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn OriginalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrimmedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SetDelay(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn Clone(&self) -> ::windows::core::Result<BackgroundAudioTrack>;
    fn GetAudioEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn AudioEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundAudioTrackStaticsImpl: Sized {
    fn CreateFromEmbeddedAudioTrack(&self, embeddedaudiotrack: &::core::option::Option<EmbeddedAudioTrack>) -> ::windows::core::Result<BackgroundAudioTrack>;
    fn CreateFromFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAudioTrack>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmbeddedAudioTrackImpl: Sized {
    fn GetAudioEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaClipImpl: Sized {
    fn TrimTimeFromStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromStart(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimTimeFromEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromEnd(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn OriginalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrimmedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Clone(&self) -> ::windows::core::Result<MediaClip>;
    fn StartTimeInComposition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn EndTimeInComposition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn EmbeddedAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmbeddedAudioTrack>>;
    fn SelectedEmbeddedAudioTrackIndex(&self) -> ::windows::core::Result<u32>;
    fn SetSelectedEmbeddedAudioTrackIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn GetVideoEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn AudioEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn VideoEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IVideoEffectDefinition>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaClipStaticsImpl: Sized {
    fn CreateFromColor(&self, color: &super::super::UI::Color, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaClip>;
    fn CreateFromFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>>;
    fn CreateFromImageFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaClipStatics2Impl: Sized {
    fn CreateFromSurface(&self, surface: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaClip>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCompositionImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Clips(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaClip>>;
    fn BackgroundAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundAudioTrack>>;
    fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Clone(&self) -> ::windows::core::Result<MediaComposition>;
    fn SaveAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetThumbnailAsync(&self, timefromstart: &super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Graphics::Imaging::ImageStream>>;
    fn GetThumbnailsAsync(&self, timesfromstart: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::TimeSpan>>, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::ImageStream>>>;
    fn RenderToFileAsync(&self, destination: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn RenderToFileWithTrimmingPreferenceAsync(&self, destination: &::core::option::Option<super::super::Storage::IStorageFile>, trimmingpreference: MediaTrimmingPreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn RenderToFileWithProfileAsync(&self, destination: &::core::option::Option<super::super::Storage::IStorageFile>, trimmingpreference: MediaTrimmingPreference, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn CreateDefaultEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile>;
    fn GenerateMediaStreamSource(&self) -> ::windows::core::Result<super::Core::MediaStreamSource>;
    fn GenerateMediaStreamSourceWithProfile(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::Core::MediaStreamSource>;
    fn GeneratePreviewMediaStreamSource(&self, scaledwidth: i32, scaledheight: i32) -> ::windows::core::Result<super::Core::MediaStreamSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaComposition2Impl: Sized {
    fn OverlayLayers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlayLayer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCompositionStaticsImpl: Sized {
    fn LoadAsync(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaComposition>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaOverlayImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetPosition(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetDelay(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<MediaOverlay>;
    fn Clip(&self) -> ::windows::core::Result<MediaClip>;
    fn AudioEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAudioEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaOverlayFactoryImpl: Sized {
    fn Create(&self, clip: &::core::option::Option<MediaClip>) -> ::windows::core::Result<MediaOverlay>;
    fn CreateWithPositionAndOpacity(&self, clip: &::core::option::Option<MediaClip>, position: &super::super::Foundation::Rect, opacity: f64) -> ::windows::core::Result<MediaOverlay>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaOverlayLayerImpl: Sized {
    fn Clone(&self) -> ::windows::core::Result<MediaOverlayLayer>;
    fn Overlays(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlay>>;
    fn CustomCompositorDefinition(&self) -> ::windows::core::Result<super::Effects::IVideoCompositorDefinition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaOverlayLayerFactoryImpl: Sized {
    fn CreateWithCompositorDefinition(&self, compositordefinition: &::core::option::Option<super::Effects::IVideoCompositorDefinition>) -> ::windows::core::Result<MediaOverlayLayer>;
}
