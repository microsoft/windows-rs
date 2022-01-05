#[cfg(feature = "implement_exclusive")]
pub trait ICurrentSessionChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalSystemMediaTransportControlsSessionImpl: Sized {
    fn SourceAppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryGetMediaPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>>;
    fn GetTimelineProperties(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionTimelineProperties>;
    fn GetPlaybackInfo(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackInfo>;
    fn TryPlayAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryPauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryStopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryFastForwardAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRewindAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySkipNextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySkipPreviousAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryChangeChannelUpAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryChangeChannelDownAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryTogglePlayPauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryChangeAutoRepeatModeAsync(&self, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryChangePlaybackRateAsync(&self, requestedplaybackrate: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryChangeShuffleActiveAsync(&self, requestedshufflestate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryChangePlaybackPositionAsync(&self, requestedplaybackposition: i64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TimelinePropertiesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, TimelinePropertiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTimelinePropertiesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackInfoChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, PlaybackInfoChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackInfoChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaPropertiesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, MediaPropertiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaPropertiesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalSystemMediaTransportControlsSessionManagerImpl: Sized {
    fn GetCurrentSession(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSession>;
    fn GetSessions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>>;
    fn CurrentSessionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, CurrentSessionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentSessionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SessionsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, SessionsChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalSystemMediaTransportControlsSessionManagerStaticsImpl: Sized {
    fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalSystemMediaTransportControlsSessionMediaPropertiesImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrackNumber(&self) -> ::windows::core::Result<i32>;
    fn Genres(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn AlbumTrackCount(&self) -> ::windows::core::Result<i32>;
    fn PlaybackType(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalSystemMediaTransportControlsSessionPlaybackControlsImpl: Sized {
    fn IsPlayEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsPauseEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsStopEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsRecordEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsFastForwardEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsRewindEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsNextEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsPreviousEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsChannelUpEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsChannelDownEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsPlayPauseToggleEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsShuffleEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsRepeatEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsPlaybackRateEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsPlaybackPositionEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalSystemMediaTransportControlsSessionPlaybackInfoImpl: Sized {
    fn Controls(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackControls>;
    fn PlaybackStatus(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackStatus>;
    fn PlaybackType(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>>;
    fn AutoRepeatMode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackAutoRepeatMode>>;
    fn PlaybackRate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn IsShuffleActive(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalSystemMediaTransportControlsSessionTimelinePropertiesImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MinSeekTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MaxSeekTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn LastUpdatedTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPropertiesChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackInfoChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISessionsChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITimelinePropertiesChangedEventArgsImpl: Sized {}
