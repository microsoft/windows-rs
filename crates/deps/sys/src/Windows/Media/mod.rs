#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Media_AppBroadcasting")]
pub mod AppBroadcasting;
#[cfg(feature = "Media_AppRecording")]
pub mod AppRecording;
#[cfg(feature = "Media_Audio")]
pub mod Audio;
#[cfg(feature = "Media_Capture")]
pub mod Capture;
#[cfg(feature = "Media_Casting")]
pub mod Casting;
#[cfg(feature = "Media_ClosedCaptioning")]
pub mod ClosedCaptioning;
#[cfg(feature = "Media_ContentRestrictions")]
pub mod ContentRestrictions;
#[cfg(feature = "Media_Control")]
pub mod Control;
#[cfg(feature = "Media_Core")]
pub mod Core;
#[cfg(feature = "Media_Devices")]
pub mod Devices;
#[cfg(feature = "Media_DialProtocol")]
pub mod DialProtocol;
#[cfg(feature = "Media_Editing")]
pub mod Editing;
#[cfg(feature = "Media_Effects")]
pub mod Effects;
#[cfg(feature = "Media_FaceAnalysis")]
pub mod FaceAnalysis;
#[cfg(feature = "Media_Import")]
pub mod Import;
#[cfg(feature = "Media_MediaProperties")]
pub mod MediaProperties;
#[cfg(feature = "Media_Miracast")]
pub mod Miracast;
#[cfg(feature = "Media_Ocr")]
pub mod Ocr;
#[cfg(feature = "Media_PlayTo")]
pub mod PlayTo;
#[cfg(feature = "Media_Playback")]
pub mod Playback;
#[cfg(feature = "Media_Playlists")]
pub mod Playlists;
#[cfg(feature = "Media_Protection")]
pub mod Protection;
#[cfg(feature = "Media_Render")]
pub mod Render;
#[cfg(feature = "Media_SpeechRecognition")]
pub mod SpeechRecognition;
#[cfg(feature = "Media_SpeechSynthesis")]
pub mod SpeechSynthesis;
#[cfg(feature = "Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Media_Transcoding")]
pub mod Transcoding;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioBufferAccessMode(pub i32);
impl AudioBufferAccessMode {
    pub const Read: AudioBufferAccessMode = AudioBufferAccessMode(0i32);
    pub const ReadWrite: AudioBufferAccessMode = AudioBufferAccessMode(1i32);
    pub const Write: AudioBufferAccessMode = AudioBufferAccessMode(2i32);
}
#[repr(transparent)]
pub struct AudioFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioProcessing(pub i32);
impl AudioProcessing {
    pub const Default: AudioProcessing = AudioProcessing(0i32);
    pub const Raw: AudioProcessing = AudioProcessing(1i32);
}
#[repr(transparent)]
pub struct AutoRepeatModeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioFrameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoRepeatModeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaExtensionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaExtensionManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaMarker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaMarkerTypesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaMarkers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaProcessingTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTimelineController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTimelineController2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTimelineControllerFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMusicDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMusicDisplayProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMusicDisplayProperties3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackPositionChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShuffleEnabledChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaTransportControls(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaTransportControls2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaTransportControlsDisplayUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaTransportControlsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMediaTransportControlsTimelineProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoDisplayProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEffectsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoFrame2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoFrameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaControlContract(i32);
#[repr(transparent)]
pub struct MediaExtensionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlaybackAutoRepeatMode(pub i32);
impl MediaPlaybackAutoRepeatMode {
    pub const None: MediaPlaybackAutoRepeatMode = MediaPlaybackAutoRepeatMode(0i32);
    pub const Track: MediaPlaybackAutoRepeatMode = MediaPlaybackAutoRepeatMode(1i32);
    pub const List: MediaPlaybackAutoRepeatMode = MediaPlaybackAutoRepeatMode(2i32);
}
#[repr(transparent)]
pub struct MediaPlaybackStatus(pub i32);
impl MediaPlaybackStatus {
    pub const Closed: MediaPlaybackStatus = MediaPlaybackStatus(0i32);
    pub const Changing: MediaPlaybackStatus = MediaPlaybackStatus(1i32);
    pub const Stopped: MediaPlaybackStatus = MediaPlaybackStatus(2i32);
    pub const Playing: MediaPlaybackStatus = MediaPlaybackStatus(3i32);
    pub const Paused: MediaPlaybackStatus = MediaPlaybackStatus(4i32);
}
#[repr(transparent)]
pub struct MediaPlaybackType(pub i32);
impl MediaPlaybackType {
    pub const Unknown: MediaPlaybackType = MediaPlaybackType(0i32);
    pub const Music: MediaPlaybackType = MediaPlaybackType(1i32);
    pub const Video: MediaPlaybackType = MediaPlaybackType(2i32);
    pub const Image: MediaPlaybackType = MediaPlaybackType(3i32);
}
#[repr(transparent)]
pub struct MediaProcessingTriggerDetails(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct MediaTimeRange(i32);
#[repr(transparent)]
pub struct MediaTimelineController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaTimelineControllerFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaTimelineControllerState(pub i32);
impl MediaTimelineControllerState {
    pub const Paused: MediaTimelineControllerState = MediaTimelineControllerState(0i32);
    pub const Running: MediaTimelineControllerState = MediaTimelineControllerState(1i32);
    pub const Stalled: MediaTimelineControllerState = MediaTimelineControllerState(2i32);
    pub const Error: MediaTimelineControllerState = MediaTimelineControllerState(3i32);
}
#[repr(transparent)]
pub struct MusicDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackPositionChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShuffleEnabledChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SoundLevel(pub i32);
impl SoundLevel {
    pub const Muted: SoundLevel = SoundLevel(0i32);
    pub const Low: SoundLevel = SoundLevel(1i32);
    pub const Full: SoundLevel = SoundLevel(2i32);
}
#[repr(transparent)]
pub struct SystemMediaTransportControls(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMediaTransportControlsButton(pub i32);
impl SystemMediaTransportControlsButton {
    pub const Play: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(0i32);
    pub const Pause: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(1i32);
    pub const Stop: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(2i32);
    pub const Record: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(3i32);
    pub const FastForward: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(4i32);
    pub const Rewind: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(5i32);
    pub const Next: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(6i32);
    pub const Previous: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(7i32);
    pub const ChannelUp: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(8i32);
    pub const ChannelDown: SystemMediaTransportControlsButton = SystemMediaTransportControlsButton(9i32);
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMediaTransportControlsDisplayUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMediaTransportControlsProperty(pub i32);
impl SystemMediaTransportControlsProperty {
    pub const SoundLevel: SystemMediaTransportControlsProperty = SystemMediaTransportControlsProperty(0i32);
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMediaTransportControlsTimelineProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoFrame(pub *mut ::core::ffi::c_void);
