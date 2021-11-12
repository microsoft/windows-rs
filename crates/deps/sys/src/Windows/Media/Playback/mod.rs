#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AutoLoadedDisplayPropertyKind(pub i32);
impl AutoLoadedDisplayPropertyKind {
    pub const None: Self = Self(0i32);
    pub const MusicOrVideo: Self = Self(1i32);
    pub const Music: Self = Self(2i32);
    pub const Video: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoLoadedDisplayPropertyKind {}
impl ::core::clone::Clone for AutoLoadedDisplayPropertyKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CurrentMediaPlaybackItemChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CurrentMediaPlaybackItemChangedEventArgs {}
impl ::core::clone::Clone for CurrentMediaPlaybackItemChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FailedMediaStreamKind(pub i32);
impl FailedMediaStreamKind {
    pub const Unknown: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for FailedMediaStreamKind {}
impl ::core::clone::Clone for FailedMediaStreamKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundMediaPlayerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundMediaPlayerStatics {}
impl ::core::clone::Clone for IBackgroundMediaPlayerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentMediaPlaybackItemChangedEventArgs {}
impl ::core::clone::Clone for ICurrentMediaPlaybackItemChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentMediaPlaybackItemChangedEventArgs2 {}
impl ::core::clone::Clone for ICurrentMediaPlaybackItemChangedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBreak(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBreak {}
impl ::core::clone::Clone for IMediaBreak {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBreakEndedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBreakEndedEventArgs {}
impl ::core::clone::Clone for IMediaBreakEndedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBreakFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBreakFactory {}
impl ::core::clone::Clone for IMediaBreakFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBreakManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBreakManager {}
impl ::core::clone::Clone for IMediaBreakManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBreakSchedule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBreakSchedule {}
impl ::core::clone::Clone for IMediaBreakSchedule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBreakSeekedOverEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBreakSeekedOverEventArgs {}
impl ::core::clone::Clone for IMediaBreakSeekedOverEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBreakSkippedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBreakSkippedEventArgs {}
impl ::core::clone::Clone for IMediaBreakSkippedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaBreakStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaBreakStartedEventArgs {}
impl ::core::clone::Clone for IMediaBreakStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaEnginePlaybackSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaEnginePlaybackSource {}
impl ::core::clone::Clone for IMediaEnginePlaybackSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaItemDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaItemDisplayProperties {}
impl ::core::clone::Clone for IMediaItemDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManager {}
impl ::core::clone::Clone for IMediaPlaybackCommandManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerCommandBehavior(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerCommandBehavior {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerCommandBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerNextReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerPauseReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerPlayReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerPositionReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerRateReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerRewindReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItem {}
impl ::core::clone::Clone for IMediaPlaybackItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItem2 {}
impl ::core::clone::Clone for IMediaPlaybackItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItem3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItem3 {}
impl ::core::clone::Clone for IMediaPlaybackItem3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItemError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItemError {}
impl ::core::clone::Clone for IMediaPlaybackItemError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItemFactory {}
impl ::core::clone::Clone for IMediaPlaybackItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItemFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItemFactory2 {}
impl ::core::clone::Clone for IMediaPlaybackItemFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItemFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItemFailedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackItemFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItemOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItemOpenedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackItemOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackItemStatics {}
impl ::core::clone::Clone for IMediaPlaybackItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackList {}
impl ::core::clone::Clone for IMediaPlaybackList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackList2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackList2 {}
impl ::core::clone::Clone for IMediaPlaybackList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackList3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackList3 {}
impl ::core::clone::Clone for IMediaPlaybackList3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackSession {}
impl ::core::clone::Clone for IMediaPlaybackSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackSession2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackSession2 {}
impl ::core::clone::Clone for IMediaPlaybackSession2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackSession3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackSession3 {}
impl ::core::clone::Clone for IMediaPlaybackSession3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackSessionBufferingStartedEventArgs {}
impl ::core::clone::Clone for IMediaPlaybackSessionBufferingStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackSessionOutputDegradationPolicyState {}
impl ::core::clone::Clone for IMediaPlaybackSessionOutputDegradationPolicyState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackSource {}
impl ::core::clone::Clone for IMediaPlaybackSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackSphericalVideoProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackSphericalVideoProjection {}
impl ::core::clone::Clone for IMediaPlaybackSphericalVideoProjection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlaybackTimedMetadataTrackList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlaybackTimedMetadataTrackList {}
impl ::core::clone::Clone for IMediaPlaybackTimedMetadataTrackList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayer {}
impl ::core::clone::Clone for IMediaPlayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayer2 {}
impl ::core::clone::Clone for IMediaPlayer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayer3 {}
impl ::core::clone::Clone for IMediaPlayer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayer4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayer4 {}
impl ::core::clone::Clone for IMediaPlayer4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayer5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayer5 {}
impl ::core::clone::Clone for IMediaPlayer5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayer6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayer6 {}
impl ::core::clone::Clone for IMediaPlayer6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayer7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayer7 {}
impl ::core::clone::Clone for IMediaPlayer7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerDataReceivedEventArgs {}
impl ::core::clone::Clone for IMediaPlayerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerEffects(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerEffects {}
impl ::core::clone::Clone for IMediaPlayerEffects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerEffects2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerEffects2 {}
impl ::core::clone::Clone for IMediaPlayerEffects2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerFailedEventArgs {}
impl ::core::clone::Clone for IMediaPlayerFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerRateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerRateChangedEventArgs {}
impl ::core::clone::Clone for IMediaPlayerRateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerSource {}
impl ::core::clone::Clone for IMediaPlayerSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerSource2 {}
impl ::core::clone::Clone for IMediaPlayerSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerSurface {}
impl ::core::clone::Clone for IMediaPlayerSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackMediaMarker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackMediaMarker {}
impl ::core::clone::Clone for IPlaybackMediaMarker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackMediaMarkerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackMediaMarkerFactory {}
impl ::core::clone::Clone for IPlaybackMediaMarkerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackMediaMarkerReachedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackMediaMarkerReachedEventArgs {}
impl ::core::clone::Clone for IPlaybackMediaMarkerReachedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackMediaMarkerSequence(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackMediaMarkerSequence {}
impl ::core::clone::Clone for IPlaybackMediaMarkerSequence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimedMetadataPresentationModeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimedMetadataPresentationModeChangedEventArgs {}
impl ::core::clone::Clone for ITimedMetadataPresentationModeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBreak(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBreak {}
impl ::core::clone::Clone for MediaBreak {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBreakEndedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBreakEndedEventArgs {}
impl ::core::clone::Clone for MediaBreakEndedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBreakInsertionMethod(pub i32);
impl MediaBreakInsertionMethod {
    pub const Interrupt: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaBreakInsertionMethod {}
impl ::core::clone::Clone for MediaBreakInsertionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBreakManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBreakManager {}
impl ::core::clone::Clone for MediaBreakManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBreakSchedule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBreakSchedule {}
impl ::core::clone::Clone for MediaBreakSchedule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBreakSeekedOverEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBreakSeekedOverEventArgs {}
impl ::core::clone::Clone for MediaBreakSeekedOverEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBreakSkippedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBreakSkippedEventArgs {}
impl ::core::clone::Clone for MediaBreakSkippedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaBreakStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaBreakStartedEventArgs {}
impl ::core::clone::Clone for MediaBreakStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCommandEnablingRule(pub i32);
impl MediaCommandEnablingRule {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaCommandEnablingRule {}
impl ::core::clone::Clone for MediaCommandEnablingRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaItemDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaItemDisplayProperties {}
impl ::core::clone::Clone for MediaItemDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackAudioTrackList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackAudioTrackList {}
impl ::core::clone::Clone for MediaPlaybackAudioTrackList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManager {}
impl ::core::clone::Clone for MediaPlaybackCommandManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerCommandBehavior(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerCommandBehavior {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerCommandBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerFastForwardReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerNextReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerNextReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPauseReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPlayReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPositionReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPreviousReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerRateReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerRateReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerRewindReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerShuffleReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackItem {}
impl ::core::clone::Clone for MediaPlaybackItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackItemChangedReason(pub i32);
impl MediaPlaybackItemChangedReason {
    pub const InitialItem: Self = Self(0i32);
    pub const EndOfStream: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
    pub const AppRequested: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaPlaybackItemChangedReason {}
impl ::core::clone::Clone for MediaPlaybackItemChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackItemError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackItemError {}
impl ::core::clone::Clone for MediaPlaybackItemError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackItemErrorCode(pub i32);
impl MediaPlaybackItemErrorCode {
    pub const None: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodeError: Self = Self(3i32);
    pub const SourceNotSupportedError: Self = Self(4i32);
    pub const EncryptionError: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaPlaybackItemErrorCode {}
impl ::core::clone::Clone for MediaPlaybackItemErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackItemFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackItemFailedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackItemFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackItemOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackItemOpenedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackItemOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackList {}
impl ::core::clone::Clone for MediaPlaybackList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackSession {}
impl ::core::clone::Clone for MediaPlaybackSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackSessionBufferingStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackSessionBufferingStartedEventArgs {}
impl ::core::clone::Clone for MediaPlaybackSessionBufferingStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackSessionOutputDegradationPolicyState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackSessionOutputDegradationPolicyState {}
impl ::core::clone::Clone for MediaPlaybackSessionOutputDegradationPolicyState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackSessionVideoConstrictionReason(pub i32);
impl MediaPlaybackSessionVideoConstrictionReason {
    pub const None: Self = Self(0i32);
    pub const VirtualMachine: Self = Self(1i32);
    pub const UnsupportedDisplayAdapter: Self = Self(2i32);
    pub const UnsignedDriver: Self = Self(3i32);
    pub const FrameServerEnabled: Self = Self(4i32);
    pub const OutputProtectionFailed: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaPlaybackSessionVideoConstrictionReason {}
impl ::core::clone::Clone for MediaPlaybackSessionVideoConstrictionReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackSphericalVideoProjection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackSphericalVideoProjection {}
impl ::core::clone::Clone for MediaPlaybackSphericalVideoProjection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackState(pub i32);
impl MediaPlaybackState {
    pub const None: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlaybackState {}
impl ::core::clone::Clone for MediaPlaybackState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackTimedMetadataTrackList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackTimedMetadataTrackList {}
impl ::core::clone::Clone for MediaPlaybackTimedMetadataTrackList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackVideoTrackList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlaybackVideoTrackList {}
impl ::core::clone::Clone for MediaPlaybackVideoTrackList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlayer {}
impl ::core::clone::Clone for MediaPlayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerAudioCategory(pub i32);
impl MediaPlayerAudioCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for MediaPlayerAudioCategory {}
impl ::core::clone::Clone for MediaPlayerAudioCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerAudioDeviceType(pub i32);
impl MediaPlayerAudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPlayerAudioDeviceType {}
impl ::core::clone::Clone for MediaPlayerAudioDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlayerDataReceivedEventArgs {}
impl ::core::clone::Clone for MediaPlayerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerError(pub i32);
impl MediaPlayerError {
    pub const Unknown: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodingError: Self = Self(3i32);
    pub const SourceNotSupported: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlayerError {}
impl ::core::clone::Clone for MediaPlayerError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlayerFailedEventArgs {}
impl ::core::clone::Clone for MediaPlayerFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerRateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlayerRateChangedEventArgs {}
impl ::core::clone::Clone for MediaPlayerRateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerState(pub i32);
impl MediaPlayerState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaPlayerState {}
impl ::core::clone::Clone for MediaPlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerSurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlayerSurface {}
impl ::core::clone::Clone for MediaPlayerSurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackMediaMarker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaybackMediaMarker {}
impl ::core::clone::Clone for PlaybackMediaMarker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackMediaMarkerReachedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaybackMediaMarkerReachedEventArgs {}
impl ::core::clone::Clone for PlaybackMediaMarkerReachedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackMediaMarkerSequence(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaybackMediaMarkerSequence {}
impl ::core::clone::Clone for PlaybackMediaMarkerSequence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SphericalVideoProjectionMode(pub i32);
impl SphericalVideoProjectionMode {
    pub const Spherical: Self = Self(0i32);
    pub const Flat: Self = Self(1i32);
}
impl ::core::marker::Copy for SphericalVideoProjectionMode {}
impl ::core::clone::Clone for SphericalVideoProjectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StereoscopicVideoRenderMode(pub i32);
impl StereoscopicVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl ::core::marker::Copy for StereoscopicVideoRenderMode {}
impl ::core::clone::Clone for StereoscopicVideoRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedMetadataPresentationModeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimedMetadataPresentationModeChangedEventArgs {}
impl ::core::clone::Clone for TimedMetadataPresentationModeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimedMetadataTrackPresentationMode(pub i32);
impl TimedMetadataTrackPresentationMode {
    pub const Disabled: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const ApplicationPresented: Self = Self(2i32);
    pub const PlatformPresented: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackPresentationMode {}
impl ::core::clone::Clone for TimedMetadataTrackPresentationMode {
    fn clone(&self) -> Self {
        *self
    }
}
