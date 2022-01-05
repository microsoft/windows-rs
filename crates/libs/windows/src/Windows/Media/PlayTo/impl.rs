#[cfg(feature = "implement_exclusive")]
pub trait ICurrentTimeChangeRequestedEventArgsImpl: Sized {
    fn Time(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMuteChangeRequestedEventArgsImpl: Sized {
    fn Mute(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToConnectionImpl: Sized {
    fn State(&self) -> ::windows::core::Result<PlayToConnectionState>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Transferred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransferred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Error(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveError(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToConnectionErrorEventArgsImpl: Sized {
    fn Code(&self) -> ::windows::core::Result<PlayToConnectionError>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToConnectionStateChangedEventArgsImpl: Sized {
    fn PreviousState(&self) -> ::windows::core::Result<PlayToConnectionState>;
    fn CurrentState(&self) -> ::windows::core::Result<PlayToConnectionState>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToConnectionTransferredEventArgsImpl: Sized {
    fn PreviousSource(&self) -> ::windows::core::Result<PlayToSource>;
    fn CurrentSource(&self) -> ::windows::core::Result<PlayToSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToManagerImpl: Sized {
    fn SourceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceSelected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceSelected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDefaultSourceSelection(&self, value: bool) -> ::windows::core::Result<()>;
    fn DefaultSourceSelection(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<PlayToManager>;
    fn ShowPlayToUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlayToReceiverImpl: Sized {
    fn PlayRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlayRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PauseRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePauseRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceChangeRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceChangeRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackRateChangeRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackRateChangeRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentTimeChangeRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentTimeChangeRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MuteChangeRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMuteChangeRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VolumeChangeRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVolumeChangeRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TimeUpdateRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTimeUpdateRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StopRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyVolumeChange(&self, volume: f64, mute: bool) -> ::windows::core::Result<()>;
    fn NotifyRateChange(&self, rate: f64) -> ::windows::core::Result<()>;
    fn NotifyLoadedMetadata(&self) -> ::windows::core::Result<()>;
    fn NotifyTimeUpdate(&self, currenttime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn NotifyDurationChange(&self, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn NotifySeeking(&self) -> ::windows::core::Result<()>;
    fn NotifySeeked(&self) -> ::windows::core::Result<()>;
    fn NotifyPaused(&self) -> ::windows::core::Result<()>;
    fn NotifyPlaying(&self) -> ::windows::core::Result<()>;
    fn NotifyEnded(&self) -> ::windows::core::Result<()>;
    fn NotifyError(&self) -> ::windows::core::Result<()>;
    fn NotifyStopped(&self) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetSupportsImage(&self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsImage(&self) -> ::windows::core::Result<bool>;
    fn SetSupportsAudio(&self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsAudio(&self) -> ::windows::core::Result<bool>;
    fn SetSupportsVideo(&self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsVideo(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<PlayToConnection>;
    fn Next(&self) -> ::windows::core::Result<PlayToSource>;
    fn SetNext(&self, value: &::core::option::Option<PlayToSource>) -> ::windows::core::Result<()>;
    fn PlayNext(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceRequestImpl: Sized {
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn DisplayErrorString(&self, errorstring: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<PlayToSourceDeferral>;
    fn SetSource(&self, value: &::core::option::Option<PlayToSource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceRequestedEventArgsImpl: Sized {
    fn SourceRequest(&self) -> ::windows::core::Result<PlayToSourceRequest>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceSelectedEventArgsImpl: Sized {
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Icon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn SupportsImage(&self) -> ::windows::core::Result<bool>;
    fn SupportsAudio(&self) -> ::windows::core::Result<bool>;
    fn SupportsVideo(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceWithPreferredSourceUriImpl: Sized {
    fn PreferredSourceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetPreferredSourceUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackRateChangeRequestedEventArgsImpl: Sized {
    fn Rate(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISourceChangeRequestedEventArgsImpl: Sized {
    fn Stream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Genre(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Date(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn Rating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVolumeChangeRequestedEventArgsImpl: Sized {
    fn Volume(&self) -> ::windows::core::Result<f64>;
}
