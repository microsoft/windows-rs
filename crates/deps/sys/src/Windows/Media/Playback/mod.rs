#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AutoLoadedDisplayPropertyKind(pub i32);
impl AutoLoadedDisplayPropertyKind {
    pub const None: AutoLoadedDisplayPropertyKind = AutoLoadedDisplayPropertyKind(0i32);
    pub const MusicOrVideo: AutoLoadedDisplayPropertyKind = AutoLoadedDisplayPropertyKind(1i32);
    pub const Music: AutoLoadedDisplayPropertyKind = AutoLoadedDisplayPropertyKind(2i32);
    pub const Video: AutoLoadedDisplayPropertyKind = AutoLoadedDisplayPropertyKind(3i32);
}
#[repr(transparent)]
pub struct CurrentMediaPlaybackItemChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FailedMediaStreamKind(pub i32);
impl FailedMediaStreamKind {
    pub const Unknown: FailedMediaStreamKind = FailedMediaStreamKind(0i32);
    pub const Audio: FailedMediaStreamKind = FailedMediaStreamKind(1i32);
    pub const Video: FailedMediaStreamKind = FailedMediaStreamKind(2i32);
}
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
#[repr(transparent)]
pub struct MediaBreakInsertionMethod(pub i32);
impl MediaBreakInsertionMethod {
    pub const Interrupt: MediaBreakInsertionMethod = MediaBreakInsertionMethod(0i32);
    pub const Replace: MediaBreakInsertionMethod = MediaBreakInsertionMethod(1i32);
}
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
#[repr(transparent)]
pub struct MediaCommandEnablingRule(pub i32);
impl MediaCommandEnablingRule {
    pub const Auto: MediaCommandEnablingRule = MediaCommandEnablingRule(0i32);
    pub const Always: MediaCommandEnablingRule = MediaCommandEnablingRule(1i32);
    pub const Never: MediaCommandEnablingRule = MediaCommandEnablingRule(2i32);
}
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
#[repr(transparent)]
pub struct MediaPlaybackItemChangedReason(pub i32);
impl MediaPlaybackItemChangedReason {
    pub const InitialItem: MediaPlaybackItemChangedReason = MediaPlaybackItemChangedReason(0i32);
    pub const EndOfStream: MediaPlaybackItemChangedReason = MediaPlaybackItemChangedReason(1i32);
    pub const Error: MediaPlaybackItemChangedReason = MediaPlaybackItemChangedReason(2i32);
    pub const AppRequested: MediaPlaybackItemChangedReason = MediaPlaybackItemChangedReason(3i32);
}
#[repr(transparent)]
pub struct MediaPlaybackItemError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackItemErrorCode(pub i32);
impl MediaPlaybackItemErrorCode {
    pub const None: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(0i32);
    pub const Aborted: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(1i32);
    pub const NetworkError: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(2i32);
    pub const DecodeError: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(3i32);
    pub const SourceNotSupportedError: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(4i32);
    pub const EncryptionError: MediaPlaybackItemErrorCode = MediaPlaybackItemErrorCode(5i32);
}
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
#[repr(transparent)]
pub struct MediaPlaybackSessionVideoConstrictionReason(pub i32);
impl MediaPlaybackSessionVideoConstrictionReason {
    pub const None: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(0i32);
    pub const VirtualMachine: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(1i32);
    pub const UnsupportedDisplayAdapter: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(2i32);
    pub const UnsignedDriver: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(3i32);
    pub const FrameServerEnabled: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(4i32);
    pub const OutputProtectionFailed: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(5i32);
    pub const Unknown: MediaPlaybackSessionVideoConstrictionReason = MediaPlaybackSessionVideoConstrictionReason(6i32);
}
#[repr(transparent)]
pub struct MediaPlaybackSphericalVideoProjection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackState(pub i32);
impl MediaPlaybackState {
    pub const None: MediaPlaybackState = MediaPlaybackState(0i32);
    pub const Opening: MediaPlaybackState = MediaPlaybackState(1i32);
    pub const Buffering: MediaPlaybackState = MediaPlaybackState(2i32);
    pub const Playing: MediaPlaybackState = MediaPlaybackState(3i32);
    pub const Paused: MediaPlaybackState = MediaPlaybackState(4i32);
}
#[repr(transparent)]
pub struct MediaPlaybackTimedMetadataTrackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackVideoTrackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayerAudioCategory(pub i32);
impl MediaPlayerAudioCategory {
    pub const Other: MediaPlayerAudioCategory = MediaPlayerAudioCategory(0i32);
    pub const Communications: MediaPlayerAudioCategory = MediaPlayerAudioCategory(3i32);
    pub const Alerts: MediaPlayerAudioCategory = MediaPlayerAudioCategory(4i32);
    pub const SoundEffects: MediaPlayerAudioCategory = MediaPlayerAudioCategory(5i32);
    pub const GameEffects: MediaPlayerAudioCategory = MediaPlayerAudioCategory(6i32);
    pub const GameMedia: MediaPlayerAudioCategory = MediaPlayerAudioCategory(7i32);
    pub const GameChat: MediaPlayerAudioCategory = MediaPlayerAudioCategory(8i32);
    pub const Speech: MediaPlayerAudioCategory = MediaPlayerAudioCategory(9i32);
    pub const Movie: MediaPlayerAudioCategory = MediaPlayerAudioCategory(10i32);
    pub const Media: MediaPlayerAudioCategory = MediaPlayerAudioCategory(11i32);
}
#[repr(transparent)]
pub struct MediaPlayerAudioDeviceType(pub i32);
impl MediaPlayerAudioDeviceType {
    pub const Console: MediaPlayerAudioDeviceType = MediaPlayerAudioDeviceType(0i32);
    pub const Multimedia: MediaPlayerAudioDeviceType = MediaPlayerAudioDeviceType(1i32);
    pub const Communications: MediaPlayerAudioDeviceType = MediaPlayerAudioDeviceType(2i32);
}
#[repr(transparent)]
pub struct MediaPlayerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayerError(pub i32);
impl MediaPlayerError {
    pub const Unknown: MediaPlayerError = MediaPlayerError(0i32);
    pub const Aborted: MediaPlayerError = MediaPlayerError(1i32);
    pub const NetworkError: MediaPlayerError = MediaPlayerError(2i32);
    pub const DecodingError: MediaPlayerError = MediaPlayerError(3i32);
    pub const SourceNotSupported: MediaPlayerError = MediaPlayerError(4i32);
}
#[repr(transparent)]
pub struct MediaPlayerFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayerRateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayerState(pub i32);
impl MediaPlayerState {
    pub const Closed: MediaPlayerState = MediaPlayerState(0i32);
    pub const Opening: MediaPlayerState = MediaPlayerState(1i32);
    pub const Buffering: MediaPlayerState = MediaPlayerState(2i32);
    pub const Playing: MediaPlayerState = MediaPlayerState(3i32);
    pub const Paused: MediaPlayerState = MediaPlayerState(4i32);
    pub const Stopped: MediaPlayerState = MediaPlayerState(5i32);
}
#[repr(transparent)]
pub struct MediaPlayerSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackMediaMarker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackMediaMarkerReachedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackMediaMarkerSequence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SphericalVideoProjectionMode(pub i32);
impl SphericalVideoProjectionMode {
    pub const Spherical: SphericalVideoProjectionMode = SphericalVideoProjectionMode(0i32);
    pub const Flat: SphericalVideoProjectionMode = SphericalVideoProjectionMode(1i32);
}
#[repr(transparent)]
pub struct StereoscopicVideoRenderMode(pub i32);
impl StereoscopicVideoRenderMode {
    pub const Mono: StereoscopicVideoRenderMode = StereoscopicVideoRenderMode(0i32);
    pub const Stereo: StereoscopicVideoRenderMode = StereoscopicVideoRenderMode(1i32);
}
#[repr(transparent)]
pub struct TimedMetadataPresentationModeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimedMetadataTrackPresentationMode(pub i32);
impl TimedMetadataTrackPresentationMode {
    pub const Disabled: TimedMetadataTrackPresentationMode = TimedMetadataTrackPresentationMode(0i32);
    pub const Hidden: TimedMetadataTrackPresentationMode = TimedMetadataTrackPresentationMode(1i32);
    pub const ApplicationPresented: TimedMetadataTrackPresentationMode = TimedMetadataTrackPresentationMode(2i32);
    pub const PlatformPresented: TimedMetadataTrackPresentationMode = TimedMetadataTrackPresentationMode(3i32);
}
