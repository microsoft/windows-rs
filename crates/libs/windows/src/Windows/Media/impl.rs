#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioBufferImpl: Sized + IClosableImpl + IMemoryBufferImpl {
    fn Capacity(&self) -> ::windows::core::Result<u32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SetLength(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn LockBuffer(&self, mode: AudioBufferAccessMode) -> ::windows::core::Result<AudioBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioFrameFactoryImpl: Sized {
    fn Create(&self, capacity: u32) -> ::windows::core::Result<AudioFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoRepeatModeChangeRequestedEventArgsImpl: Sized {
    fn RequestedAutoRepeatMode(&self) -> ::windows::core::Result<MediaPlaybackAutoRepeatMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageDisplayPropertiesImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IMediaControlImpl: Sized {
    fn SoundLevelChanged(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveSoundLevelChanged(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlayPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PausePressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePausePressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StopPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayPauseTogglePressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlayPauseTogglePressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecordPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRecordPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NextTrackPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveNextTrackPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviousTrackPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePreviousTrackPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FastForwardPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveFastForwardPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RewindPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRewindPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChannelUpPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveChannelUpPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChannelDownPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveChannelDownPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SoundLevel(&self) -> ::windows::core::Result<SoundLevel>;
    fn SetTrackName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TrackName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArtistName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ArtistName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIsPlaying(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPlaying(&self) -> ::windows::core::Result<bool>;
    fn SetAlbumArt(&self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AlbumArt(&self) -> ::windows::core::Result<super::Foundation::Uri>;
}
pub trait IMediaExtensionImpl: Sized {
    fn SetProperties(&self, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaExtensionManagerImpl: Sized {
    fn RegisterSchemeHandler(&self, activatableclassid: &::windows::core::HSTRING, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RegisterSchemeHandlerWithSettings(&self, activatableclassid: &::windows::core::HSTRING, scheme: &::windows::core::HSTRING, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterByteStreamHandler(&self, activatableclassid: &::windows::core::HSTRING, fileextension: &::windows::core::HSTRING, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RegisterByteStreamHandlerWithSettings(&self, activatableclassid: &::windows::core::HSTRING, fileextension: &::windows::core::HSTRING, mimetype: &::windows::core::HSTRING, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterAudioDecoder(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterAudioDecoderWithSettings(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterAudioEncoder(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterAudioEncoderWithSettings(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterVideoDecoder(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterVideoDecoderWithSettings(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterVideoEncoder(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterVideoEncoderWithSettings(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaExtensionManager2Impl: Sized + IMediaExtensionManagerImpl {
    fn RegisterMediaExtensionForAppService(&self, extension: &::core::option::Option<IMediaExtension>, connection: &::core::option::Option<super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
pub trait IMediaFrameImpl: Sized + IClosableImpl {
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetRelativeTime(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn RelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetSystemRelativeTime(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SystemRelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetIsDiscontinuous(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDiscontinuous(&self) -> ::windows::core::Result<bool>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet>;
}
pub trait IMediaMarkerImpl: Sized {
    fn Time(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn MediaMarkerType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaMarkerTypesStaticsImpl: Sized {
    fn Bookmark(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IMediaMarkersImpl: Sized {
    fn Markers(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProcessingTriggerDetailsImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTimelineControllerImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ClockRate(&self) -> ::windows::core::Result<f64>;
    fn SetClockRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<MediaTimelineControllerState>;
    fn PositionChanged(&self, positionchangedeventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StateChanged(&self, statechangedeventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTimelineController2Impl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn IsLoopingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsLoopingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Failed(&self, eventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveFailed(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Ended(&self, eventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnded(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTimelineControllerFailedEventArgsImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicDisplayPropertiesImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbumArtist(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArtist(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicDisplayProperties2Impl: Sized {
    fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbumTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TrackNumber(&self) -> ::windows::core::Result<u32>;
    fn SetTrackNumber(&self, value: u32) -> ::windows::core::Result<()>;
    fn Genres(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicDisplayProperties3Impl: Sized {
    fn AlbumTrackCount(&self) -> ::windows::core::Result<u32>;
    fn SetAlbumTrackCount(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackPositionChangeRequestedEventArgsImpl: Sized {
    fn RequestedPlaybackPosition(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackRateChangeRequestedEventArgsImpl: Sized {
    fn RequestedPlaybackRate(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShuffleEnabledChangeRequestedEventArgsImpl: Sized {
    fn RequestedShuffleEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsImpl: Sized {
    fn PlaybackStatus(&self) -> ::windows::core::Result<MediaPlaybackStatus>;
    fn SetPlaybackStatus(&self, value: MediaPlaybackStatus) -> ::windows::core::Result<()>;
    fn DisplayUpdater(&self) -> ::windows::core::Result<SystemMediaTransportControlsDisplayUpdater>;
    fn SoundLevel(&self) -> ::windows::core::Result<SoundLevel>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPlayEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPlayEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsStopEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStopEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPauseEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPauseEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRecordEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRecordEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFastForwardEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFastForwardEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRewindEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRewindEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPreviousEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPreviousEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsNextEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsNextEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsChannelUpEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsChannelUpEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsChannelDownEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsChannelDownEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ButtonPressed(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveButtonPressed(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertyChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePropertyChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControls2Impl: Sized {
    fn AutoRepeatMode(&self) -> ::windows::core::Result<MediaPlaybackAutoRepeatMode>;
    fn SetAutoRepeatMode(&self, value: MediaPlaybackAutoRepeatMode) -> ::windows::core::Result<()>;
    fn ShuffleEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetShuffleEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn UpdateTimelineProperties(&self, timelineproperties: &::core::option::Option<SystemMediaTransportControlsTimelineProperties>) -> ::windows::core::Result<()>;
    fn PlaybackPositionChangeRequested(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackPositionChangeRequested(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackRateChangeRequested(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackRateChangeRequested(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShuffleEnabledChangeRequested(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShuffleEnabledChangeRequested(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoRepeatModeChangeRequested(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAutoRepeatModeChangeRequested(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsButtonPressedEventArgsImpl: Sized {
    fn Button(&self) -> ::windows::core::Result<SystemMediaTransportControlsButton>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsDisplayUpdaterImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<MediaPlaybackType>;
    fn SetType(&self, value: MediaPlaybackType) -> ::windows::core::Result<()>;
    fn AppMediaId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppMediaId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn MusicProperties(&self) -> ::windows::core::Result<MusicDisplayProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<VideoDisplayProperties>;
    fn ImageProperties(&self) -> ::windows::core::Result<ImageDisplayProperties>;
    fn CopyFromFileAsync(&self, r#type: MediaPlaybackType, source: &::core::option::Option<super::Storage::StorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn ClearAll(&self) -> ::windows::core::Result<()>;
    fn Update(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsPropertyChangedEventArgsImpl: Sized {
    fn Property(&self) -> ::windows::core::Result<SystemMediaTransportControlsProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SystemMediaTransportControls>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsTimelinePropertiesImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetStartTime(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn EndTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetEndTime(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MinSeekTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetMinSeekTime(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MaxSeekTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetMaxSeekTime(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDisplayPropertiesImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDisplayProperties2Impl: Sized {
    fn Genres(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEffectsStaticsImpl: Sized {
    fn VideoStabilization(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVideoFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::Graphics::Imaging::SoftwareBitmap>;
    fn CopyToAsync(&self, frame: &::core::option::Option<VideoFrame>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn Direct3DSurface(&self) -> ::windows::core::Result<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoFrame2Impl: Sized {
    fn CopyToWithBoundsAsync(&self, frame: &::core::option::Option<VideoFrame>, sourcebounds: &::core::option::Option<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>, destinationbounds: &::core::option::Option<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoFrameFactoryImpl: Sized {
    fn Create(&self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithAlpha(&self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<VideoFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoFrameStaticsImpl: Sized {
    fn CreateAsDirect3D11SurfaceBacked(&self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame>;
    fn CreateAsDirect3D11SurfaceBackedWithDevice(&self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: &::core::option::Option<super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithSoftwareBitmap(&self, bitmap: &::core::option::Option<super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithDirect3D11Surface(&self, surface: &::core::option::Option<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<VideoFrame>;
}
