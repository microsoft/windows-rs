#![allow(non_snake_case, non_camel_case_types)]
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
extern "system" {
    fn AudioBuffer();
    fn AudioBufferAccessMode();
    fn AudioFrame();
    fn AudioProcessing();
    fn AutoRepeatModeChangeRequestedEventArgs();
    fn IAudioBuffer();
    fn IAudioFrame();
    fn IAudioFrameFactory();
    fn IAutoRepeatModeChangeRequestedEventArgs();
    fn IImageDisplayProperties();
    fn IMediaControl();
    fn IMediaExtension();
    fn IMediaExtensionManager();
    fn IMediaExtensionManager2();
    fn IMediaFrame();
    fn IMediaMarker();
    fn IMediaMarkerTypesStatics();
    fn IMediaMarkers();
    fn IMediaProcessingTriggerDetails();
    fn IMediaTimelineController();
    fn IMediaTimelineController2();
    fn IMediaTimelineControllerFailedEventArgs();
    fn IMusicDisplayProperties();
    fn IMusicDisplayProperties2();
    fn IMusicDisplayProperties3();
    fn IPlaybackPositionChangeRequestedEventArgs();
    fn IPlaybackRateChangeRequestedEventArgs();
    fn IShuffleEnabledChangeRequestedEventArgs();
    fn ISystemMediaTransportControls();
    fn ISystemMediaTransportControls2();
    fn ISystemMediaTransportControlsButtonPressedEventArgs();
    fn ISystemMediaTransportControlsDisplayUpdater();
    fn ISystemMediaTransportControlsPropertyChangedEventArgs();
    fn ISystemMediaTransportControlsStatics();
    fn ISystemMediaTransportControlsTimelineProperties();
    fn IVideoDisplayProperties();
    fn IVideoDisplayProperties2();
    fn IVideoEffectsStatics();
    fn IVideoFrame();
    fn IVideoFrame2();
    fn IVideoFrameFactory();
    fn IVideoFrameStatics();
    fn ImageDisplayProperties();
    fn MediaControl();
    fn MediaControlContract();
    fn MediaExtensionManager();
    fn MediaMarkerTypes();
    fn MediaPlaybackAutoRepeatMode();
    fn MediaPlaybackStatus();
    fn MediaPlaybackType();
    fn MediaProcessingTriggerDetails();
    fn MediaTimeRange();
    fn MediaTimelineController();
    fn MediaTimelineControllerFailedEventArgs();
    fn MediaTimelineControllerState();
    fn MusicDisplayProperties();
    fn PlaybackPositionChangeRequestedEventArgs();
    fn PlaybackRateChangeRequestedEventArgs();
    fn ShuffleEnabledChangeRequestedEventArgs();
    fn SoundLevel();
    fn SystemMediaTransportControls();
    fn SystemMediaTransportControlsButton();
    fn SystemMediaTransportControlsButtonPressedEventArgs();
    fn SystemMediaTransportControlsDisplayUpdater();
    fn SystemMediaTransportControlsProperty();
    fn SystemMediaTransportControlsPropertyChangedEventArgs();
    fn SystemMediaTransportControlsTimelineProperties();
    fn VideoDisplayProperties();
    fn VideoEffects();
    fn VideoFrame();
}
