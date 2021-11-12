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
#[repr(transparent)]
pub struct AudioBuffer(pub *mut ::core::ffi::c_void);
pub struct AudioBufferAccessMode(i32);
#[repr(transparent)]
pub struct AudioFrame(pub *mut ::core::ffi::c_void);
pub struct AudioProcessing(i32);
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
#[repr(transparent)]
pub struct MediaControl(pub *mut ::core::ffi::c_void);
pub struct MediaControlContract(i32);
#[repr(transparent)]
pub struct MediaExtensionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaMarkerTypes(pub *mut ::core::ffi::c_void);
pub struct MediaPlaybackAutoRepeatMode(i32);
pub struct MediaPlaybackStatus(i32);
pub struct MediaPlaybackType(i32);
#[repr(transparent)]
pub struct MediaProcessingTriggerDetails(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
pub struct MediaTimeRange(i32);
#[repr(transparent)]
pub struct MediaTimelineController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaTimelineControllerFailedEventArgs(pub *mut ::core::ffi::c_void);
pub struct MediaTimelineControllerState(i32);
#[repr(transparent)]
pub struct MusicDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackPositionChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShuffleEnabledChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct SoundLevel(i32);
#[repr(transparent)]
pub struct SystemMediaTransportControls(pub *mut ::core::ffi::c_void);
pub struct SystemMediaTransportControlsButton(i32);
#[repr(transparent)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMediaTransportControlsDisplayUpdater(pub *mut ::core::ffi::c_void);
pub struct SystemMediaTransportControlsProperty(i32);
#[repr(transparent)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMediaTransportControlsTimelineProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoDisplayProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoEffects(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoFrame(pub *mut ::core::ffi::c_void);
