#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AutoLoadedDisplayPropertyKind(i32);
#[repr(transparent)]
pub struct BackgroundMediaPlayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CurrentMediaPlaybackItemChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct FailedMediaStreamKind(i32);
#[repr(transparent)]
pub struct IBackgroundMediaPlayerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBreak(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBreakEndedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBreakFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBreakManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBreakSchedule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBreakSeekedOverEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBreakSkippedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBreakStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEnginePlaybackSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaItemDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerCommandBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItem3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItemError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItemFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItemFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItemOpenedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackList2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackList3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackSession3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackSphericalVideoProjection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlaybackTimedMetadataTrackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayer4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayer5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayer6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayer7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerEffects(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerEffects2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerRateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackMediaMarker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackMediaMarkerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackMediaMarkerReachedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackMediaMarkerSequence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimedMetadataPresentationModeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBreak(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBreakEndedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaBreakInsertionMethod(i32);
#[repr(transparent)]
pub struct MediaBreakManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBreakSchedule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBreakSeekedOverEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBreakSkippedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaBreakStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaCommandEnablingRule(i32);
#[repr(transparent)]
pub struct MediaItemDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackAudioTrackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerCommandBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerFastForwardReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerNextReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPauseReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPlayReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPositionReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPreviousReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerRateReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerRewindReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerShuffleReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackItem(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaPlaybackItemChangedReason(i32);
#[repr(transparent)]
pub struct MediaPlaybackItemError(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaPlaybackItemErrorCode(i32);
#[repr(transparent)]
pub struct MediaPlaybackItemFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackItemOpenedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackSessionBufferingStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackSessionOutputDegradationPolicyState(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaPlaybackSessionVideoConstrictionReason(i32);
#[repr(transparent)]
pub struct MediaPlaybackSphericalVideoProjection(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaPlaybackState(i32);
#[repr(transparent)]
pub struct MediaPlaybackTimedMetadataTrackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackVideoTrackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaPlayerAudioCategory(i32);
#[repr(C)]
pub struct MediaPlayerAudioDeviceType(i32);
#[repr(transparent)]
pub struct MediaPlayerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaPlayerError(i32);
#[repr(transparent)]
pub struct MediaPlayerFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayerRateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaPlayerState(i32);
#[repr(transparent)]
pub struct MediaPlayerSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackMediaMarker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackMediaMarkerReachedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackMediaMarkerSequence(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SphericalVideoProjectionMode(i32);
#[repr(C)]
pub struct StereoscopicVideoRenderMode(i32);
#[repr(transparent)]
pub struct TimedMetadataPresentationModeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TimedMetadataTrackPresentationMode(i32);
