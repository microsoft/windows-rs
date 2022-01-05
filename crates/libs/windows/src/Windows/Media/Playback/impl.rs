#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundMediaPlayerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<MediaPlayer>;
    fn MessageReceivedFromBackground(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceivedFromBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MessageReceivedFromForeground(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceivedFromForeground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SendMessageToBackground(&self, value: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
    fn SendMessageToForeground(&self, value: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
    fn IsMediaPlaying(&self) -> ::windows::core::Result<bool>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentMediaPlaybackItemChangedEventArgsImpl: Sized {
    fn NewItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn OldItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentMediaPlaybackItemChangedEventArgs2Impl: Sized + ICurrentMediaPlaybackItemChangedEventArgsImpl {
    fn Reason(&self) -> ::windows::core::Result<MediaPlaybackItemChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakImpl: Sized {
    fn PlaybackList(&self) -> ::windows::core::Result<MediaPlaybackList>;
    fn PresentationPosition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn InsertionMethod(&self) -> ::windows::core::Result<MediaBreakInsertionMethod>;
    fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn CanStart(&self) -> ::windows::core::Result<bool>;
    fn SetCanStart(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakEndedEventArgsImpl: Sized {
    fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakFactoryImpl: Sized {
    fn Create(&self, insertionmethod: MediaBreakInsertionMethod) -> ::windows::core::Result<MediaBreak>;
    fn CreateWithPresentationPosition(&self, insertionmethod: MediaBreakInsertionMethod, presentationposition: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakManagerImpl: Sized {
    fn BreaksSeekedOver(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreaksSeekedOver(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakEnded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakSkipped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakSkipped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentBreak(&self) -> ::windows::core::Result<MediaBreak>;
    fn PlaybackSession(&self) -> ::windows::core::Result<MediaPlaybackSession>;
    fn PlayBreak(&self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn SkipCurrentBreak(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakScheduleImpl: Sized {
    fn ScheduleChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakSchedule, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScheduleChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InsertMidrollBreak(&self, mediabreak: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn RemoveMidrollBreak(&self, mediabreak: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn MidrollBreaks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>>;
    fn SetPrerollBreak(&self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn PrerollBreak(&self) -> ::windows::core::Result<MediaBreak>;
    fn SetPostrollBreak(&self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn PostrollBreak(&self) -> ::windows::core::Result<MediaBreak>;
    fn PlaybackItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakSeekedOverEventArgsImpl: Sized {
    fn SeekedOverBreaks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>>;
    fn OldPosition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn NewPosition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakSkippedEventArgsImpl: Sized {
    fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakStartedEventArgsImpl: Sized {
    fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "deprecated")]
pub trait IMediaEnginePlaybackSourceImpl: Sized {
    fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn SetPlaybackSource(&self, source: &::core::option::Option<IMediaPlaybackSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaItemDisplayPropertiesImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<super::MediaPlaybackType>;
    fn SetType(&self, value: super::MediaPlaybackType) -> ::windows::core::Result<()>;
    fn MusicProperties(&self) -> ::windows::core::Result<super::MusicDisplayProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<super::VideoDisplayProperties>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn ClearAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer>;
    fn PlayBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PauseBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn NextBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PreviousBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn FastForwardBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn RewindBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn ShuffleBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn AutoRepeatModeBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PositionBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn RateBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PlayReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlayReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PauseReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePauseReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NextReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNextReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviousReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviousReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FastForwardReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFastForwardReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RewindReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRewindReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShuffleReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShuffleReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoRepeatModeReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoRepeatModeReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RateReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRateReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoRepeatMode(&self) -> ::windows::core::Result<super::MediaPlaybackAutoRepeatMode>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerCommandBehaviorImpl: Sized {
    fn CommandManager(&self) -> ::windows::core::Result<MediaPlaybackCommandManager>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn EnablingRule(&self) -> ::windows::core::Result<MediaCommandEnablingRule>;
    fn SetEnablingRule(&self, value: MediaCommandEnablingRule) -> ::windows::core::Result<()>;
    fn IsEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerFastForwardReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerNextReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerPauseReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerPlayReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerPositionReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerPreviousReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerRateReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerRewindReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackCommandManagerShuffleReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsShuffleRequested(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemImpl: Sized + IMediaPlaybackSourceImpl {
    fn AudioTracksChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioTracksChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoTracksChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoTracksChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TimedMetadataTracksChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTimedMetadataTracksChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<super::Core::MediaSource>;
    fn AudioTracks(&self) -> ::windows::core::Result<MediaPlaybackAudioTrackList>;
    fn VideoTracks(&self) -> ::windows::core::Result<MediaPlaybackVideoTrackList>;
    fn TimedMetadataTracks(&self) -> ::windows::core::Result<MediaPlaybackTimedMetadataTrackList>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItem2Impl: Sized + IMediaPlaybackItemImpl + IMediaPlaybackSourceImpl {
    fn BreakSchedule(&self) -> ::windows::core::Result<MediaBreakSchedule>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn DurationLimit(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn CanSkip(&self) -> ::windows::core::Result<bool>;
    fn SetCanSkip(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDisplayProperties(&self) -> ::windows::core::Result<MediaItemDisplayProperties>;
    fn ApplyDisplayProperties(&self, value: &::core::option::Option<MediaItemDisplayProperties>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItem3Impl: Sized + IMediaPlaybackItemImpl + IMediaPlaybackItem2Impl + IMediaPlaybackSourceImpl {
    fn IsDisabledInPlaybackList(&self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledInPlaybackList(&self, value: bool) -> ::windows::core::Result<()>;
    fn TotalDownloadProgress(&self) -> ::windows::core::Result<f64>;
    fn AutoLoadedDisplayProperties(&self) -> ::windows::core::Result<AutoLoadedDisplayPropertyKind>;
    fn SetAutoLoadedDisplayProperties(&self, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemErrorImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<MediaPlaybackItemErrorCode>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemFactoryImpl: Sized {
    fn Create(&self, source: &::core::option::Option<super::Core::MediaSource>) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemFactory2Impl: Sized + IMediaPlaybackItemFactoryImpl {
    fn CreateWithStartTime(&self, source: &::core::option::Option<super::Core::MediaSource>, starttime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaPlaybackItem>;
    fn CreateWithStartTimeAndDurationLimit(&self, source: &::core::option::Option<super::Core::MediaSource>, starttime: &super::super::Foundation::TimeSpan, durationlimit: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemFailedEventArgsImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn Error(&self) -> ::windows::core::Result<MediaPlaybackItemError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemOpenedEventArgsImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemStaticsImpl: Sized {
    fn FindFromMediaSource(&self, source: &::core::option::Option<super::Core::MediaSource>) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackListImpl: Sized + IMediaPlaybackSourceImpl {
    fn ItemFailed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentItemChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentItemChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ItemOpened(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemOpened(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<MediaPlaybackItem>>;
    fn AutoRepeatEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAutoRepeatEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShuffleEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetShuffleEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn CurrentItemIndex(&self) -> ::windows::core::Result<u32>;
    fn MoveNext(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn MovePrevious(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn MoveTo(&self, itemindex: u32) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackList2Impl: Sized + IMediaPlaybackListImpl + IMediaPlaybackSourceImpl {
    fn MaxPrefetchTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetMaxPrefetchTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn StartingItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn SetStartingItem(&self, value: &::core::option::Option<MediaPlaybackItem>) -> ::windows::core::Result<()>;
    fn ShuffledItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaPlaybackItem>>;
    fn SetShuffledItems(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<MediaPlaybackItem>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackList3Impl: Sized + IMediaPlaybackListImpl + IMediaPlaybackList2Impl + IMediaPlaybackSourceImpl {
    fn MaxPlayedItemsToKeepOpen(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetMaxPlayedItemsToKeepOpen(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSessionImpl: Sized {
    fn PlaybackStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackRateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackRateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekCompleted(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingStarted(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingEnded(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingProgressChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingProgressChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadProgressChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadProgressChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NaturalDurationChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNaturalDurationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NaturalVideoSizeChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNaturalVideoSizeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer>;
    fn NaturalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PlaybackState(&self) -> ::windows::core::Result<MediaPlaybackState>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn CanPause(&self) -> ::windows::core::Result<bool>;
    fn IsProtected(&self) -> ::windows::core::Result<bool>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn BufferingProgress(&self) -> ::windows::core::Result<f64>;
    fn DownloadProgress(&self) -> ::windows::core::Result<f64>;
    fn NaturalVideoHeight(&self) -> ::windows::core::Result<u32>;
    fn NaturalVideoWidth(&self) -> ::windows::core::Result<u32>;
    fn NormalizedSourceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetNormalizedSourceRect(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn StereoscopicVideoPackingMode(&self) -> ::windows::core::Result<super::MediaProperties::StereoscopicVideoPackingMode>;
    fn SetStereoscopicVideoPackingMode(&self, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSession2Impl: Sized {
    fn BufferedRangesChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferedRangesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayedRangesChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlayedRangesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekableRangesChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekableRangesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SupportedPlaybackRatesChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSupportedPlaybackRatesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SphericalVideoProjection(&self) -> ::windows::core::Result<MediaPlaybackSphericalVideoProjection>;
    fn IsMirroring(&self) -> ::windows::core::Result<bool>;
    fn SetIsMirroring(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetBufferedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn GetPlayedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn GetSeekableRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn IsSupportedPlaybackRateRange(&self, rate1: f64, rate2: f64) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSession3Impl: Sized {
    fn PlaybackRotation(&self) -> ::windows::core::Result<super::MediaProperties::MediaRotation>;
    fn SetPlaybackRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()>;
    fn GetOutputDegradationPolicyState(&self) -> ::windows::core::Result<MediaPlaybackSessionOutputDegradationPolicyState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSessionBufferingStartedEventArgsImpl: Sized {
    fn IsPlaybackInterruption(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSessionOutputDegradationPolicyStateImpl: Sized {
    fn VideoConstrictionReason(&self) -> ::windows::core::Result<MediaPlaybackSessionVideoConstrictionReason>;
}
pub trait IMediaPlaybackSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSphericalVideoProjectionImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn FrameFormat(&self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat>;
    fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()>;
    fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
    fn ViewOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetViewOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn ProjectionMode(&self) -> ::windows::core::Result<SphericalVideoProjectionMode>;
    fn SetProjectionMode(&self, value: SphericalVideoProjectionMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackTimedMetadataTrackListImpl: Sized {
    fn PresentationModeChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePresentationModeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetPresentationMode(&self, index: u32) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
    fn SetPresentationMode(&self, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerImpl: Sized {
    fn AutoPlay(&self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()>;
    fn NaturalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BufferingProgress(&self) -> ::windows::core::Result<f64>;
    fn CurrentState(&self) -> ::windows::core::Result<MediaPlayerState>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn CanPause(&self) -> ::windows::core::Result<bool>;
    fn IsLoopingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsLoopingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsProtected(&self) -> ::windows::core::Result<bool>;
    fn IsMuted(&self) -> ::windows::core::Result<bool>;
    fn SetIsMuted(&self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackMediaMarkers(&self) -> ::windows::core::Result<PlaybackMediaMarkerSequence>;
    fn MediaOpened(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaOpened(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaEnded(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaFailed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackMediaMarkerReached(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackMediaMarkerReached(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaPlayerRateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaPlayerRateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VolumeChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVolumeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekCompleted(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingStarted(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingEnded(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn SetUriSource(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayer2Impl: Sized {
    fn SystemMediaTransportControls(&self) -> ::windows::core::Result<super::SystemMediaTransportControls>;
    fn AudioCategory(&self) -> ::windows::core::Result<MediaPlayerAudioCategory>;
    fn SetAudioCategory(&self, value: MediaPlayerAudioCategory) -> ::windows::core::Result<()>;
    fn AudioDeviceType(&self) -> ::windows::core::Result<MediaPlayerAudioDeviceType>;
    fn SetAudioDeviceType(&self, value: MediaPlayerAudioDeviceType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayer3Impl: Sized {
    fn IsMutedChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsMutedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AudioBalance(&self) -> ::windows::core::Result<f64>;
    fn SetAudioBalance(&self, value: f64) -> ::windows::core::Result<()>;
    fn RealTimePlayback(&self) -> ::windows::core::Result<bool>;
    fn SetRealTimePlayback(&self, value: bool) -> ::windows::core::Result<()>;
    fn StereoscopicVideoRenderMode(&self) -> ::windows::core::Result<StereoscopicVideoRenderMode>;
    fn SetStereoscopicVideoRenderMode(&self, value: StereoscopicVideoRenderMode) -> ::windows::core::Result<()>;
    fn BreakManager(&self) -> ::windows::core::Result<MediaBreakManager>;
    fn CommandManager(&self) -> ::windows::core::Result<MediaPlaybackCommandManager>;
    fn AudioDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
    fn SetAudioDevice(&self, value: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<()>;
    fn TimelineController(&self) -> ::windows::core::Result<super::MediaTimelineController>;
    fn SetTimelineController(&self, value: &::core::option::Option<super::MediaTimelineController>) -> ::windows::core::Result<()>;
    fn TimelineControllerPositionOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTimelineControllerPositionOffset(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PlaybackSession(&self) -> ::windows::core::Result<MediaPlaybackSession>;
    fn StepForwardOneFrame(&self) -> ::windows::core::Result<()>;
    fn StepBackwardOneFrame(&self) -> ::windows::core::Result<()>;
    fn GetAsCastingSource(&self) -> ::windows::core::Result<super::Casting::CastingSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayer4Impl: Sized {
    fn SetSurfaceSize(&self, size: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn GetSurface(&self, compositor: &::core::option::Option<super::super::UI::Composition::Compositor>) -> ::windows::core::Result<MediaPlayerSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayer5Impl: Sized {
    fn VideoFrameAvailable(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameAvailable(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsVideoFrameServerEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsVideoFrameServerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CopyFrameToVideoSurface(&self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
    fn CopyFrameToVideoSurfaceWithTargetRectangle(&self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, targetrectangle: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn CopyFrameToStereoscopicVideoSurfaces(&self, destinationlefteye: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, destinationrighteye: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayer6Impl: Sized {
    fn SubtitleFrameChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubtitleFrameChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RenderSubtitlesToSurface(&self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<bool>;
    fn RenderSubtitlesToSurfaceWithTargetRectangle(&self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, targetrectangle: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayer7Impl: Sized {
    fn AudioStateMonitor(&self) -> ::windows::core::Result<super::Audio::AudioStateMonitor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerDataReceivedEventArgsImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerEffectsImpl: Sized {
    fn AddAudioEffect(&self, activatableclassid: &::windows::core::HSTRING, effectoptional: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RemoveAllEffects(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerEffects2Impl: Sized {
    fn AddVideoEffect(&self, activatableclassid: &::windows::core::HSTRING, effectoptional: bool, effectconfiguration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerFailedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<MediaPlayerError>;
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerRateChangedEventArgsImpl: Sized {
    fn NewRate(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerSourceImpl: Sized {
    fn ProtectionManager(&self) -> ::windows::core::Result<super::Protection::MediaProtectionManager>;
    fn SetProtectionManager(&self, value: &::core::option::Option<super::Protection::MediaProtectionManager>) -> ::windows::core::Result<()>;
    fn SetFileSource(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
    fn SetStreamSource(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetMediaSource(&self, source: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerSource2Impl: Sized {
    fn Source(&self) -> ::windows::core::Result<IMediaPlaybackSource>;
    fn SetSource(&self, value: &::core::option::Option<IMediaPlaybackSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerSurfaceImpl: Sized {
    fn CompositionSurface(&self) -> ::windows::core::Result<super::super::UI::Composition::ICompositionSurface>;
    fn Compositor(&self) -> ::windows::core::Result<super::super::UI::Composition::Compositor>;
    fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackMediaMarkerImpl: Sized {
    fn Time(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MediaMarkerType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackMediaMarkerFactoryImpl: Sized {
    fn CreateFromTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<PlaybackMediaMarker>;
    fn Create(&self, value: &super::super::Foundation::TimeSpan, mediamarkettype: &::windows::core::HSTRING, text: &::windows::core::HSTRING) -> ::windows::core::Result<PlaybackMediaMarker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackMediaMarkerReachedEventArgsImpl: Sized {
    fn PlaybackMediaMarker(&self) -> ::windows::core::Result<PlaybackMediaMarker>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPlaybackMediaMarkerSequenceImpl: Sized + IIterableImpl<PlaybackMediaMarker> {
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn Insert(&self, value: &::core::option::Option<PlaybackMediaMarker>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataPresentationModeChangedEventArgsImpl: Sized {
    fn Track(&self) -> ::windows::core::Result<super::Core::TimedMetadataTrack>;
    fn OldPresentationMode(&self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
    fn NewPresentationMode(&self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
}
