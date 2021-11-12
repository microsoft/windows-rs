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
extern "system" {}
pub struct AudioBuffer(i32);
pub struct AudioBufferAccessMode(i32);
pub struct AudioFrame(i32);
pub struct AudioProcessing(i32);
pub struct AutoRepeatModeChangeRequestedEventArgs(i32);
pub struct IAudioBuffer(pub *mut ::core::ffi::c_void);
pub struct IAudioFrame(pub *mut ::core::ffi::c_void);
pub struct IAudioFrameFactory(pub *mut ::core::ffi::c_void);
pub struct IAutoRepeatModeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IImageDisplayProperties(pub *mut ::core::ffi::c_void);
pub struct IMediaControl(pub *mut ::core::ffi::c_void);
pub struct IMediaExtension(pub *mut ::core::ffi::c_void);
pub struct IMediaExtensionManager(pub *mut ::core::ffi::c_void);
pub struct IMediaExtensionManager2(pub *mut ::core::ffi::c_void);
pub struct IMediaFrame(pub *mut ::core::ffi::c_void);
pub struct IMediaMarker(pub *mut ::core::ffi::c_void);
pub struct IMediaMarkerTypesStatics(pub *mut ::core::ffi::c_void);
pub struct IMediaMarkers(pub *mut ::core::ffi::c_void);
pub struct IMediaProcessingTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IMediaTimelineController(pub *mut ::core::ffi::c_void);
pub struct IMediaTimelineController2(pub *mut ::core::ffi::c_void);
pub struct IMediaTimelineControllerFailedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IMusicDisplayProperties(pub *mut ::core::ffi::c_void);
pub struct IMusicDisplayProperties2(pub *mut ::core::ffi::c_void);
pub struct IMusicDisplayProperties3(pub *mut ::core::ffi::c_void);
pub struct IPlaybackPositionChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IShuffleEnabledChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISystemMediaTransportControls(pub *mut ::core::ffi::c_void);
pub struct ISystemMediaTransportControls2(pub *mut ::core::ffi::c_void);
pub struct ISystemMediaTransportControlsButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISystemMediaTransportControlsDisplayUpdater(pub *mut ::core::ffi::c_void);
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISystemMediaTransportControlsStatics(pub *mut ::core::ffi::c_void);
pub struct ISystemMediaTransportControlsTimelineProperties(pub *mut ::core::ffi::c_void);
pub struct IVideoDisplayProperties(pub *mut ::core::ffi::c_void);
pub struct IVideoDisplayProperties2(pub *mut ::core::ffi::c_void);
pub struct IVideoEffectsStatics(pub *mut ::core::ffi::c_void);
pub struct IVideoFrame(pub *mut ::core::ffi::c_void);
pub struct IVideoFrame2(pub *mut ::core::ffi::c_void);
pub struct IVideoFrameFactory(pub *mut ::core::ffi::c_void);
pub struct IVideoFrameStatics(pub *mut ::core::ffi::c_void);
pub struct ImageDisplayProperties(i32);
pub struct MediaControl(i32);
pub struct MediaControlContract(i32);
pub struct MediaExtensionManager(i32);
pub struct MediaMarkerTypes(i32);
pub struct MediaPlaybackAutoRepeatMode(i32);
pub struct MediaPlaybackStatus(i32);
pub struct MediaPlaybackType(i32);
pub struct MediaProcessingTriggerDetails(i32);
#[cfg(feature = "Foundation")]
pub struct MediaTimeRange(i32);
pub struct MediaTimelineController(i32);
pub struct MediaTimelineControllerFailedEventArgs(i32);
pub struct MediaTimelineControllerState(i32);
pub struct MusicDisplayProperties(i32);
pub struct PlaybackPositionChangeRequestedEventArgs(i32);
pub struct PlaybackRateChangeRequestedEventArgs(i32);
pub struct ShuffleEnabledChangeRequestedEventArgs(i32);
pub struct SoundLevel(i32);
pub struct SystemMediaTransportControls(i32);
pub struct SystemMediaTransportControlsButton(i32);
pub struct SystemMediaTransportControlsButtonPressedEventArgs(i32);
pub struct SystemMediaTransportControlsDisplayUpdater(i32);
pub struct SystemMediaTransportControlsProperty(i32);
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(i32);
pub struct SystemMediaTransportControlsTimelineProperties(i32);
pub struct VideoDisplayProperties(i32);
pub struct VideoEffects(i32);
pub struct VideoFrame(i32);
