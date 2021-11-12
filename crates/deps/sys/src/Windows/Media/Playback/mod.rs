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
#[repr(transparent)]
pub struct MediaPlaybackVideoTrackList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayer(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MediaPlayerRateChangedEventArgs(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct PlaybackMediaMarker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackMediaMarkerReachedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackMediaMarkerSequence(pub *mut ::core::ffi::c_void);
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
