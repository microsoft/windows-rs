#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub mod Apo;
#[cfg(feature = "Win32_Media_Audio_DirectMusic")]
pub mod DirectMusic;
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub mod DirectSound;
#[cfg(feature = "Win32_Media_Audio_Endpoints")]
pub mod Endpoints;
#[cfg(feature = "Win32_Media_Audio_XAudio2")]
pub mod XAudio2;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ActivateAudioInterfaceAsync();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CoRegisterMessageFilter();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitor();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitorForCategory();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitor();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitorForCategory();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceId();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverClose();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn acmDriverDetailsA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn acmDriverDetailsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverEnum();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverID();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverMessage();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverOpen();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverPriority();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverRemove();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterDetailsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFilterDetailsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagDetailsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFilterTagDetailsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatDetailsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatDetailsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatSuggest();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagDetailsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatTagDetailsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmGetVersion();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmMetrics();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamClose();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamConvert();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmStreamMessage();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamOpen();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamPrepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamReset();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamSize();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamUnprepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn auxGetDevCapsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetDevCapsW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetNumDevs();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetVolume();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxOutMessage();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxSetVolume();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiConnect();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiDisconnect();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInAddBuffer();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInClose();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetDevCapsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetDevCapsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetErrorTextA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetErrorTextW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetID();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetNumDevs();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInMessage();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInOpen();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInPrepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInReset();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInStart();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInStop();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInUnprepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutCacheDrumPatches();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutCachePatches();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutClose();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetDevCapsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetDevCapsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetErrorTextA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetErrorTextW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetID();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetNumDevs();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetVolume();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutLongMsg();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutMessage();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutOpen();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutPrepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutReset();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutSetVolume();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutShortMsg();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutUnprepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamClose();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamOpen();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiStreamOut();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamPause();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamPosition();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamProperty();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamRestart();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamStop();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerClose();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetDevCapsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetDevCapsW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetID();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineControlsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetLineControlsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineInfoA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetLineInfoW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetNumDevs();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerMessage();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerOpen();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerSetControlDetails();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInAddBuffer();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInClose();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetDevCapsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetDevCapsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetErrorTextA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetErrorTextW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetID();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetNumDevs();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetPosition();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInMessage();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInOpen();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInPrepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInReset();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInStart();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInStop();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInUnprepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutBreakLoop();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutClose();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetDevCapsA();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetDevCapsW();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetErrorTextA();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetErrorTextW();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetID();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetNumDevs();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPitch();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPlaybackRate();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPosition();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetVolume();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutMessage();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutOpen();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutPause();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutPrepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutReset();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutRestart();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetPitch();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetPlaybackRate();
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetVolume();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutUnprepareHeader();
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutWrite();
}
