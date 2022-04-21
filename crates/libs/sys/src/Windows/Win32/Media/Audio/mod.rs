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
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ActivateAudioInterfaceAsync(deviceinterfacepath: ::windows_sys::core::PCWSTR, riid: *const ::windows_sys::core::GUID, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, completionhandler: IActivateAudioInterfaceCompletionHandler, activationoperation: *mut IActivateAudioInterfaceAsyncOperation) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CoRegisterMessageFilter(lpmessagefilter: IMessageFilter, lplpmessagefilter: *mut IMessageFilter) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CreateCaptureAudioStateMonitor(audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: ::windows_sys::core::PCWSTR, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CreateRenderAudioStateMonitor(audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: ::windows_sys::core::PCWSTR, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundA(pszsound: ::windows_sys::core::PCSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundW(pszsound: ::windows_sys::core::PCWSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddA(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddW(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmDriverClose(had: HACMDRIVER, fdwclose: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn acmDriverDetailsA(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn acmDriverDetailsW(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverEnum(fncallback: ACMDRIVERENUMCB, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmDriverID(hao: HACMOBJ, phadid: *mut isize, fdwdriverid: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverMessage(had: HACMDRIVER, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmDriverOpen(phad: *mut isize, hadid: HACMDRIVERID, fdwopen: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmDriverPriority(hadid: HACMDRIVERID, dwpriority: u32, fdwpriority: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmDriverRemove(hadid: HACMDRIVERID, fdwremove: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterDetailsA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmFilterDetailsW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fncallback: ACMFILTERENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fncallback: ACMFILTERENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmFilterTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ACMFILTERTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ACMFILTERTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatDetailsA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmFormatDetailsW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fncallback: ACMFORMATENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fncallback: ACMFORMATENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmFormatSuggest(had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmFormatTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ACMFORMATTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ACMFORMATTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmGetVersion() -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmMetrics(hao: HACMOBJ, umetric: u32, pmetric: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmStreamClose(has: HACMSTREAM, fdwclose: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmStreamConvert(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmStreamMessage(has: HACMSTREAM, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmStreamOpen(phas: *mut isize, had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmStreamPrepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmStreamReset(has: HACMSTREAM, fdwreset: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmStreamSize(has: HACMSTREAM, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn acmStreamUnprepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn auxGetNumDevs() -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiDisconnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInAddBuffer(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInClose(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInGetErrorTextA(mmrerror: u32, psztext: ::windows_sys::core::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInGetErrorTextW(mmrerror: u32, psztext: ::windows_sys::core::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInGetID(hmi: HMIDIIN, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInGetNumDevs() -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInMessage(hmi: HMIDIIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInPrepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInReset(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInStart(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInStop(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: u32, pwkya: *const u16, fucache: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutCachePatches(hmo: HMIDIOUT, ubank: u32, pwpa: *const u16, fucache: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutClose(hmo: HMIDIOUT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutGetErrorTextA(mmrerror: u32, psztext: ::windows_sys::core::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutGetErrorTextW(mmrerror: u32, psztext: ::windows_sys::core::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutGetNumDevs() -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutGetVolume(hmo: HMIDIOUT, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutLongMsg(hmo: HMIDIOUT, pmh: *const MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutMessage(hmo: HMIDIOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutReset(hmo: HMIDIOUT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutSetVolume(hmo: HMIDIOUT, dwvolume: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiStreamClose(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: *mut u32, cmidi: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiStreamOut(hms: HMIDISTRM, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiStreamPause(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiStreamPosition(hms: HMIDISTRM, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiStreamProperty(hms: HMIDISTRM, lppropdata: *mut u8, dwproperty: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiStreamRestart(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn midiStreamStop(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn mixerClose(hmx: HMIXER) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsA(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsW(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn mixerGetID(hmxobj: HMIXEROBJ, pumxid: *mut u32, fdwid: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineControlsA(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn mixerGetLineControlsW(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineInfoA(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn mixerGetLineInfoW(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn mixerGetNumDevs() -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn mixerMessage(hmx: HMIXER, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn mixerOpen(phmx: *mut isize, umxid: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerSetControlDetails(hmxobj: HMIXEROBJ, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundA(pszsound: ::windows_sys::core::PCSTR, fusound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundW(pszsound: ::windows_sys::core::PCWSTR, fusound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInAddBuffer(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInClose(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInGetErrorTextA(mmrerror: u32, psztext: ::windows_sys::core::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInGetErrorTextW(mmrerror: u32, psztext: ::windows_sys::core::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInGetID(hwi: HWAVEIN, pudeviceid: *const u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInGetNumDevs() -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInGetPosition(hwi: HWAVEIN, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInMessage(hwi: HWAVEIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInOpen(phwi: *mut HWAVEIN, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInPrepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInReset(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInStart(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInStop(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutBreakLoop(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutClose(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetErrorTextA(mmrerror: u32, psztext: ::windows_sys::core::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetErrorTextW(mmrerror: u32, psztext: ::windows_sys::core::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetNumDevs() -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutGetVolume(hwo: HWAVEOUT, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutMessage(hwo: HWAVEOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutOpen(phwo: *mut HWAVEOUT, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutPause(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutReset(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutRestart(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutSetVolume(hwo: HWAVEOUT, dwvolume: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub fn waveOutWrite(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_USER: u32 = 16384u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct ACMDRIVERDETAILSA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [super::super::Foundation::CHAR; 32],
    pub szLongName: [super::super::Foundation::CHAR; 128],
    pub szCopyright: [super::super::Foundation::CHAR; 80],
    pub szLicensing: [super::super::Foundation::CHAR; 128],
    pub szFeatures: [super::super::Foundation::CHAR; 512],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for ACMDRIVERDETAILSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for ACMDRIVERDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct ACMDRIVERDETAILSW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [u16; 32],
    pub szLongName: [u16; 128],
    pub szCopyright: [u16; 80],
    pub szLicensing: [u16; 128],
    pub szFeatures: [u16; 512],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for ACMDRIVERDETAILSW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for ACMDRIVERDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMDRIVERENUMCB = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVFORMATSUGGEST {
    pub cbStruct: u32,
    pub fdwSuggest: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub cbwfxSrc: u32,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub cbwfxDst: u32,
}
impl ::core::marker::Copy for ACMDRVFORMATSUGGEST {}
impl ::core::clone::Clone for ACMDRVFORMATSUGGEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub fdwConvert: u32,
    pub padshNext: *mut ACMDRVSTREAMHEADER,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub fdwPrepared: u32,
    pub dwPrepared: usize,
    pub pbPreparedSrc: *mut u8,
    pub cbPreparedSrcLength: u32,
    pub pbPreparedDst: *mut u8,
    pub cbPreparedDstLength: u32,
}
impl ::core::marker::Copy for ACMDRVSTREAMHEADER {}
impl ::core::clone::Clone for ACMDRVSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVSTREAMINSTANCE {
    pub cbStruct: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub pwfltr: *mut WAVEFILTER,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub fdwOpen: u32,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub has: HACMSTREAM,
}
impl ::core::marker::Copy for ACMDRVSTREAMINSTANCE {}
impl ::core::clone::Clone for ACMDRVSTREAMINSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVSTREAMSIZE {
    pub cbStruct: u32,
    pub fdwSize: u32,
    pub cbSrcLength: u32,
    pub cbDstLength: u32,
}
impl ::core::marker::Copy for ACMDRVSTREAMSIZE {}
impl ::core::clone::Clone for ACMDRVSTREAMSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_BASE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_BUSY: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_CANCELED: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: ::windows_sys::core::PCSTR,
    pub szFilterTag: [super::super::Foundation::CHAR; 48],
    pub szFilter: [super::super::Foundation::CHAR; 128],
    pub pszName: ::windows_sys::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: ::windows_sys::core::PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFILTERCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: ::windows_sys::core::PCWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: ::windows_sys::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: ::windows_sys::core::PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFILTERCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERDETAILSA {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFILTERDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFILTERDETAILSW {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [u16; 128],
}
impl ::core::marker::Copy for ACMFILTERDETAILSW {}
impl ::core::clone::Clone for ACMFILTERDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [super::super::Foundation::CHAR; 48],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFILTERTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERTAGDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u16; 48],
}
impl ::core::marker::Copy for ACMFILTERTAGDETAILSW {}
impl ::core::clone::Clone for ACMFILTERTAGDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: ::windows_sys::core::PCSTR,
    pub szFormatTag: [super::super::Foundation::CHAR; 48],
    pub szFormat: [super::super::Foundation::CHAR; 128],
    pub pszName: ::windows_sys::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: ::windows_sys::core::PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFORMATCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATCHOOSEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: ::windows_sys::core::PCWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: ::windows_sys::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: ::windows_sys::core::PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFORMATCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATCHOOSEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATDETAILSA {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFORMATDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [super::super::Foundation::CHAR; 48],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFORMATTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATTAGDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u16; 48],
}
impl ::core::marker::Copy for ACMFORMATTAGDETAILSW {}
impl ::core::clone::Clone for ACMFORMATTAGDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELP: &str = "acmchoose_contexthelp";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELPA: &str = "acmchoose_contexthelp";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELPW: &str = "acmchoose_contexthelp";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENU: &str = "acmchoose_contextmenu";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENUA: &str = "acmchoose_contextmenu";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENUW: &str = "acmchoose_contextmenu";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRING: &str = "acmchoose_help";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRINGA: &str = "acmchoose_help";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRINGW: &str = "acmchoose_help";
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 15],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ACMSTREAMHEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ACMSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[cfg(target_arch = "x86")]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 10],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for ACMSTREAMHEADER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for ACMSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AMBISONICS_CHANNEL_ORDERING = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AMBISONICS_NORMALIZATION = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AMBISONICS_PARAMS {
    pub u32Size: u32,
    pub u32Version: u32,
    pub u32Type: AMBISONICS_TYPE,
    pub u32ChannelOrdering: AMBISONICS_CHANNEL_ORDERING,
    pub u32Normalization: AMBISONICS_NORMALIZATION,
    pub u32Order: u32,
    pub u32NumChannels: u32,
    pub pu32ChannelMap: *mut u32,
}
impl ::core::marker::Copy for AMBISONICS_PARAMS {}
impl ::core::clone::Clone for AMBISONICS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AMBISONICS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = -2004287486i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: ::windows_sys::core::HRESULT = -2004287469i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_ERROR: ::windows_sys::core::HRESULT = -2004287464i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: ::windows_sys::core::HRESULT = -2004287477i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: ::windows_sys::core::HRESULT = -2004287466i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: ::windows_sys::core::HRESULT = -2004287463i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_TOO_LARGE: ::windows_sys::core::HRESULT = -2004287482i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: ::windows_sys::core::HRESULT = -2004287465i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_DEVICE_INVALIDATED: ::windows_sys::core::HRESULT = -2004287484i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_DEVICE_IN_USE: ::windows_sys::core::HRESULT = -2004287478i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2004287423i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: ::windows_sys::core::HRESULT = -2004287422i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: ::windows_sys::core::HRESULT = -2004287473i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: ::windows_sys::core::HRESULT = -2004287454i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: ::windows_sys::core::HRESULT = -2004287447i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: ::windows_sys::core::HRESULT = -2004287448i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: ::windows_sys::core::HRESULT = -2004287471i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: ::windows_sys::core::HRESULT = -2004287468i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2004287474i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: ::windows_sys::core::HRESULT = -2004287470i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_HEADTRACKING_ENABLED: ::windows_sys::core::HRESULT = -2004287440i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: ::windows_sys::core::HRESULT = -2004287424i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: ::windows_sys::core::HRESULT = -2004287467i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: ::windows_sys::core::HRESULT = -2004287456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_SIZE: ::windows_sys::core::HRESULT = -2004287479i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_STREAM_FLAG: ::windows_sys::core::HRESULT = -2004287455i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: ::windows_sys::core::HRESULT = -2004287451i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2004287487i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NOT_STOPPED: ::windows_sys::core::HRESULT = -2004287483i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: ::windows_sys::core::HRESULT = -2004287452i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: ::windows_sys::core::HRESULT = -2004287453i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OUT_OF_ORDER: ::windows_sys::core::HRESULT = -2004287481i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: ::windows_sys::core::HRESULT = -2004287449i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_RESOURCES_INVALIDATED: ::windows_sys::core::HRESULT = -2004287450i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: ::windows_sys::core::HRESULT = -2004287472i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: ::windows_sys::core::HRESULT = -2004287476i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: ::windows_sys::core::HRESULT = -2004287480i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: ::windows_sys::core::HRESULT = -2004287485i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AUDCLNT_SHAREMODE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AUDCLNT_STREAMOPTIONS = u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_BUFFER_EMPTY: ::windows_sys::core::HRESULT = 143196161i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_POSITION_STALLED: ::windows_sys::core::HRESULT = 143196163i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = 143196162i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0,
}
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_PARAMS {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    pub ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
}
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_PARAMS_0 {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AUDIOCLIENT_ACTIVATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
impl ::core::marker::Copy for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl ::core::clone::Clone for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AUDIO_DUCKING_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_EFFECT {
    pub id: ::windows_sys::core::GUID,
    pub canSetState: super::super::Foundation::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_EFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AUDIO_EFFECT_STATE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AUDIO_STREAM_CATEGORY = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Other: AUDIO_STREAM_CATEGORY = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_ForegroundOnlyMedia: AUDIO_STREAM_CATEGORY = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Communications: AUDIO_STREAM_CATEGORY = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Alerts: AUDIO_STREAM_CATEGORY = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_SoundEffects: AUDIO_STREAM_CATEGORY = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameEffects: AUDIO_STREAM_CATEGORY = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameMedia: AUDIO_STREAM_CATEGORY = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameChat: AUDIO_STREAM_CATEGORY = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Speech: AUDIO_STREAM_CATEGORY = 9i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Movie: AUDIO_STREAM_CATEGORY = 10i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Media: AUDIO_STREAM_CATEGORY = 11i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_FarFieldSpeech: AUDIO_STREAM_CATEGORY = 12i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_UniformSpeech: AUDIO_STREAM_CATEGORY = 13i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_VoiceTyping: AUDIO_STREAM_CATEGORY = 14i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: ::windows_sys::core::GUID,
    pub bMuted: super::super::Foundation::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_VOLUME_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUXCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUXCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUXCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUXCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for AUXCAPS2W {}
impl ::core::clone::Clone for AUXCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUXCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUXCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUXCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUXCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for AUXCAPSW {}
impl ::core::clone::Clone for AUXCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_AUXIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_VOLUME: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for AudioClient3ActivationParams {}
impl ::core::clone::Clone for AudioClient3ActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: super::super::Foundation::BOOL,
    pub eCategory: AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AudioClientProperties {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AudioClientProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AudioExtensionParams {
    pub AddPageParam: super::super::Foundation::LPARAM,
    pub pEndpoint: IMMDevice,
    pub pPnpInterface: IMMDevice,
    pub pPnpDevnode: IMMDevice,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AudioExtensionParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AudioExtensionParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AudioObjectType = u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_None: AudioObjectType = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_Dynamic: AudioObjectType = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontLeft: AudioObjectType = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontRight: AudioObjectType = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontCenter: AudioObjectType = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_LowFrequency: AudioObjectType = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_SideLeft: AudioObjectType = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_SideRight: AudioObjectType = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackLeft: AudioObjectType = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackRight: AudioObjectType = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopFrontLeft: AudioObjectType = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopFrontRight: AudioObjectType = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopBackLeft: AudioObjectType = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopBackRight: AudioObjectType = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomFrontRight: AudioObjectType = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomBackLeft: AudioObjectType = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomBackRight: AudioObjectType = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackCenter: AudioObjectType = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AudioSessionDisconnectReason = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AudioSessionState = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateInactive: AudioSessionState = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateActive: AudioSessionState = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateExpired: AudioSessionState = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type AudioStateMonitorSoundLevel = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Muted: AudioStateMonitorSoundLevel = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Low: AudioStateMonitorSoundLevel = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Full: AudioStateMonitorSoundLevel = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ConnectorType(pub i32);
impl ConnectorType {
    pub const Unknown_Connector: Self = Self(0i32);
    pub const Physical_Internal: Self = Self(1i32);
    pub const Physical_External: Self = Self(2i32);
    pub const Software_IO: Self = Self(3i32);
    pub const Software_Fixed: Self = Self(4i32);
    pub const Network: Self = Self(5i32);
}
impl ::core::marker::Copy for ConnectorType {}
impl ::core::clone::Clone for ConnectorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_UNPLUGGED: u32 = 8u32;
pub const DEVINTERFACE_AUDIO_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 787448254, data2: 13306, data3: 18432, data4: [150, 112, 28, 212, 116, 151, 44, 63] };
pub const DEVINTERFACE_AUDIO_RENDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3862068397, data2: 56556, data3: 18761, data4: [174, 138, 153, 30, 151, 106, 121, 210] };
pub const DEVINTERFACE_MIDI_INPUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1347150636, data2: 52470, data3: 19756, data4: [183, 63, 111, 139, 55, 71, 226, 43] };
pub const DEVINTERFACE_MIDI_OUTPUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1841443616, data2: 43827, data3: 19684, data4: [128, 212, 187, 179, 235, 191, 40, 20] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: ::windows_sys::core::GUID,
    pub dwAudioStreamFlags: u32,
}
impl ::core::marker::Copy for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl ::core::clone::Clone for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRVM_MAPPER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type DataFlow = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const In: DataFlow = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Out: DataFlow = 1i32;
pub const DeviceTopology: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 502675920, data2: 24257, data3: 18346, data4: [147, 121, 130, 141, 193, 170, 140, 89] };
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
impl ::core::marker::Copy for ECHOWAVEFILTER {}
impl ::core::clone::Clone for ECHOWAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type EDataFlow = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eRender: EDataFlow = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eCapture: EDataFlow = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eAll: EDataFlow = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const EDataFlow_enum_count: EDataFlow = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type ERole = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eConsole: ERole = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eMultimedia: ERole = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eCommunications: ERole = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ERole_enum_count: ERole = 3i32;
pub const EVENTCONTEXT_VOLUMESLIDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3804424670, data2: 2481, data3: 19204, data4: [132, 229, 7, 147, 18, 37, 238, 4] };
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type EndpointFormFactor = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const RemoteNetworkDevice: EndpointFormFactor = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Speakers: EndpointFormFactor = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const LineLevel: EndpointFormFactor = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Headphones: EndpointFormFactor = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Microphone: EndpointFormFactor = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Headset: EndpointFormFactor = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Handset: EndpointFormFactor = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const UnknownDigitalPassthrough: EndpointFormFactor = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPDIF: EndpointFormFactor = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DigitalAudioDisplayDevice: EndpointFormFactor = 9i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const UnknownFormFactor: EndpointFormFactor = 10i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const EndpointFormFactor_enum_count: EndpointFormFactor = 11i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
pub type HACMDRIVER = isize;
pub type HACMDRIVERID = isize;
pub type HACMOBJ = isize;
pub type HACMSTREAM = isize;
pub type HMIDI = isize;
pub type HMIDIIN = isize;
pub type HMIDIOUT = isize;
pub type HMIDISTRM = isize;
pub type HMIXER = isize;
pub type HMIXEROBJ = isize;
pub type HWAVE = isize;
pub type HWAVEIN = isize;
pub type HWAVEOUT = isize;
pub type IActivateAudioInterfaceAsyncOperation = *mut ::core::ffi::c_void;
pub type IActivateAudioInterfaceCompletionHandler = *mut ::core::ffi::c_void;
pub type IAudioAmbisonicsControl = *mut ::core::ffi::c_void;
pub type IAudioAutoGainControl = *mut ::core::ffi::c_void;
pub type IAudioBass = *mut ::core::ffi::c_void;
pub type IAudioCaptureClient = *mut ::core::ffi::c_void;
pub type IAudioChannelConfig = *mut ::core::ffi::c_void;
pub type IAudioClient = *mut ::core::ffi::c_void;
pub type IAudioClient2 = *mut ::core::ffi::c_void;
pub type IAudioClient3 = *mut ::core::ffi::c_void;
pub type IAudioClientDuckingControl = *mut ::core::ffi::c_void;
pub type IAudioClock = *mut ::core::ffi::c_void;
pub type IAudioClock2 = *mut ::core::ffi::c_void;
pub type IAudioClockAdjustment = *mut ::core::ffi::c_void;
pub type IAudioEffectsChangedNotificationClient = *mut ::core::ffi::c_void;
pub type IAudioEffectsManager = *mut ::core::ffi::c_void;
pub type IAudioFormatEnumerator = *mut ::core::ffi::c_void;
pub type IAudioInputSelector = *mut ::core::ffi::c_void;
pub type IAudioLoudness = *mut ::core::ffi::c_void;
pub type IAudioMidrange = *mut ::core::ffi::c_void;
pub type IAudioMute = *mut ::core::ffi::c_void;
pub type IAudioOutputSelector = *mut ::core::ffi::c_void;
pub type IAudioPeakMeter = *mut ::core::ffi::c_void;
pub type IAudioRenderClient = *mut ::core::ffi::c_void;
pub type IAudioSessionControl = *mut ::core::ffi::c_void;
pub type IAudioSessionControl2 = *mut ::core::ffi::c_void;
pub type IAudioSessionEnumerator = *mut ::core::ffi::c_void;
pub type IAudioSessionEvents = *mut ::core::ffi::c_void;
pub type IAudioSessionManager = *mut ::core::ffi::c_void;
pub type IAudioSessionManager2 = *mut ::core::ffi::c_void;
pub type IAudioSessionNotification = *mut ::core::ffi::c_void;
pub type IAudioStateMonitor = *mut ::core::ffi::c_void;
pub type IAudioStreamVolume = *mut ::core::ffi::c_void;
pub type IAudioSystemEffectsPropertyChangeNotificationClient = *mut ::core::ffi::c_void;
pub type IAudioSystemEffectsPropertyStore = *mut ::core::ffi::c_void;
pub type IAudioTreble = *mut ::core::ffi::c_void;
pub type IAudioVolumeDuckNotification = *mut ::core::ffi::c_void;
pub type IAudioVolumeLevel = *mut ::core::ffi::c_void;
pub type IChannelAudioVolume = *mut ::core::ffi::c_void;
pub type IConnector = *mut ::core::ffi::c_void;
pub type IControlChangeNotify = *mut ::core::ffi::c_void;
pub type IControlInterface = *mut ::core::ffi::c_void;
pub type IDeviceSpecificProperty = *mut ::core::ffi::c_void;
pub type IDeviceTopology = *mut ::core::ffi::c_void;
pub type IMMDevice = *mut ::core::ffi::c_void;
pub type IMMDeviceActivator = *mut ::core::ffi::c_void;
pub type IMMDeviceCollection = *mut ::core::ffi::c_void;
pub type IMMDeviceEnumerator = *mut ::core::ffi::c_void;
pub type IMMEndpoint = *mut ::core::ffi::c_void;
pub type IMMNotificationClient = *mut ::core::ffi::c_void;
pub type IMessageFilter = *mut ::core::ffi::c_void;
pub type IPart = *mut ::core::ffi::c_void;
pub type IPartsList = *mut ::core::ffi::c_void;
pub type IPerChannelDbLevel = *mut ::core::ffi::c_void;
pub type ISimpleAudioVolume = *mut ::core::ffi::c_void;
pub type ISpatialAudioClient = *mut ::core::ffi::c_void;
pub type ISpatialAudioClient2 = *mut ::core::ffi::c_void;
pub type ISpatialAudioMetadataClient = *mut ::core::ffi::c_void;
pub type ISpatialAudioMetadataCopier = *mut ::core::ffi::c_void;
pub type ISpatialAudioMetadataItems = *mut ::core::ffi::c_void;
pub type ISpatialAudioMetadataItemsBuffer = *mut ::core::ffi::c_void;
pub type ISpatialAudioMetadataReader = *mut ::core::ffi::c_void;
pub type ISpatialAudioMetadataWriter = *mut ::core::ffi::c_void;
pub type ISpatialAudioObject = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectBase = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectForHrtf = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectForMetadataCommands = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectForMetadataItems = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectRenderStream = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectRenderStreamBase = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectRenderStreamForHrtf = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectRenderStreamForMetadata = *mut ::core::ffi::c_void;
pub type ISpatialAudioObjectRenderStreamNotify = *mut ::core::ffi::c_void;
pub type ISubunit = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPACMDRIVERPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Media_Multimedia\"`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = ::core::option::Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Media_Multimedia\"`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = ::core::option::Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_LONG: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_SHORT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_INQUEUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_ISSTRM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_STREAM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_LASTERROR: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NODEVICE: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NOMAP: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NOTREADY: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl ::core::marker::Copy for MIDIEVENT {}
impl ::core::clone::Clone for MIDIEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIHDR {
    pub lpData: ::windows_sys::core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub lpNext: *mut MIDIHDR,
    pub reserved: usize,
    pub dwOffset: u32,
    pub dwReserved: [usize; 8],
}
impl ::core::marker::Copy for MIDIHDR {}
impl ::core::clone::Clone for MIDIHDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIDIINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIDIINCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for MIDIINCAPS2W {}
impl ::core::clone::Clone for MIDIINCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIDIINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIDIINCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIINCAPSW {}
impl ::core::clone::Clone for MIDIINCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIDIOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIDIOUTCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for MIDIOUTCAPS2W {}
impl ::core::clone::Clone for MIDIOUTCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIDIOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIDIOUTCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIOUTCAPSW {}
impl ::core::clone::Clone for MIDIOUTCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPATCHSIZE: u32 = 128u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
impl ::core::marker::Copy for MIDIPROPTEMPO {}
impl ::core::clone::Clone for MIDIPROPTEMPO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
impl ::core::marker::Copy for MIDIPROPTIMEDIV {}
impl ::core::clone::Clone for MIDIPROPTIMEDIV {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_GET: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_SET: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_TEMPO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
impl ::core::marker::Copy for MIDISTRMBUFFVER {}
impl ::core::clone::Clone for MIDISTRMBUFFVER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDISTRM_ERROR: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_QUERY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_UNCACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type MIDI_WAVE_OPEN_TYPE = u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = 458752u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = 196608u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = 327680u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_QUERY: MIDI_WAVE_OPEN_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_ALLOWSYNC: MIDI_WAVE_OPEN_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPED: MIDI_WAVE_OPEN_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_DIRECT: MIDI_WAVE_OPEN_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_DIRECT_QUERY: MIDI_WAVE_OPEN_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = 32u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for MIXERCAPS2W {}
impl ::core::clone::Clone for MIXERCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl ::core::marker::Copy for MIXERCAPSW {}
impl ::core::clone::Clone for MIXERCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLA {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [super::super::Foundation::CHAR; 16],
    pub szName: [super::super::Foundation::CHAR; 64],
    pub Bounds: MIXERCONTROLA_0,
    pub Metrics: MIXERCONTROLA_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLA_0 {
    pub Anonymous1: MIXERCONTROLA_0_0,
    pub Anonymous2: MIXERCONTROLA_0_1,
    pub dwReserved: [u32; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLA_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLDETAILS {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub cChannels: u32,
    pub Anonymous: MIXERCONTROLDETAILS_0,
    pub cbDetails: u32,
    pub paDetails: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLDETAILS_0 {
    pub hwndOwner: super::super::Foundation::HWND,
    pub cMultipleItems: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLDETAILS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLDETAILS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_BOOLEAN {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_BOOLEAN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLDETAILS_LISTTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLDETAILS_LISTTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u16; 64],
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_LISTTEXTW {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_LISTTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_SIGNED {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_SIGNED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_UNSIGNED {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_UNSIGNED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLW {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Bounds: MIXERCONTROLW_0,
    pub Metrics: MIXERCONTROLW_1,
}
impl ::core::marker::Copy for MIXERCONTROLW {}
impl ::core::clone::Clone for MIXERCONTROLW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERCONTROLW_0 {
    pub Anonymous1: MIXERCONTROLW_0_0,
    pub Anonymous2: MIXERCONTROLW_0_1,
    pub dwReserved: [u32; 6],
}
impl ::core::marker::Copy for MIXERCONTROLW_0 {}
impl ::core::clone::Clone for MIXERCONTROLW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl ::core::marker::Copy for MIXERCONTROLW_0_0 {}
impl ::core::clone::Clone for MIXERCONTROLW_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl ::core::marker::Copy for MIXERCONTROLW_0_1 {}
impl ::core::clone::Clone for MIXERCONTROLW_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERCONTROLW_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl ::core::marker::Copy for MIXERCONTROLW_1 {}
impl ::core::clone::Clone for MIXERCONTROLW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERLINEA {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [super::super::Foundation::CHAR; 16],
    pub szName: [super::super::Foundation::CHAR; 64],
    pub Target: MIXERLINEA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERLINEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERLINEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERLINEA_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERLINEA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERLINEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERLINECONTROLSA {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSA_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERLINECONTROLSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERLINECONTROLSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERLINECONTROLSA_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERLINECONTROLSA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERLINECONTROLSA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINECONTROLSW {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSW_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLW,
}
impl ::core::marker::Copy for MIXERLINECONTROLSW {}
impl ::core::clone::Clone for MIXERLINECONTROLSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERLINECONTROLSW_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl ::core::marker::Copy for MIXERLINECONTROLSW_0 {}
impl ::core::clone::Clone for MIXERLINECONTROLSW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINEW {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Target: MIXERLINEW_0,
}
impl ::core::marker::Copy for MIXERLINEW {}
impl ::core::clone::Clone for MIXERLINEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINEW_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
}
impl ::core::marker::Copy for MIXERLINEW_0 {}
impl ::core::clone::Clone for MIXERLINEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type MIXERLINE_COMPONENTTYPE = u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE = 4106u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE = 4105u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE = 4101u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE = 4097u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE = 4098u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE = 4099u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE = 4103u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE = 4100u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE = 4102u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE = 4104u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALLINE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALVALUE: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_LASTERROR: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
pub const MMDeviceEnumerator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3168666517, data2: 58671, data3: 18044, data4: [142, 61, 196, 87, 146, 145, 105, 46] };
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_FMSYNTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_MAPPER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_MIDIPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SQSYNTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SWSYNTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SYNTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_WAVETABLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type PAudioStateMonitorCallback = ::core::option::Option<unsafe extern "system" fn(audiostatemonitor: IAudioStateMonitor, context: *const ::core::ffi::c_void)>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
impl ::core::marker::Copy for PCMWAVEFORMAT {}
impl ::core::clone::Clone for PCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4054546445, data2: 8208, data3: 20179, data4: [163, 166, 139, 135, 240, 240, 196, 118] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4054546445, data2: 8208, data3: 20179, data4: [163, 166, 139, 135, 240, 240, 196, 118] }, pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 337911810, data2: 800, data3: 19940, data4: [149, 85, 167, 216, 43, 115, 194, 134] }, pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_MenuText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 337911810, data2: 800, data3: 19940, data4: [149, 85, 167, 216, 43, 115, 194, 134] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 497408003, data2: 54418, data3: 20189, data4: [140, 35, 224, 192, 255, 238, 127, 14] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 4053730893, data2: 2092, data3: 20007, data4: [188, 115, 104, 130, 161, 187, 142, 76] }, pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 3834056230, data2: 15557, data3: 19666, data4: [186, 70, 202, 10, 154, 112, 237, 4] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type PROCESS_LOOPBACK_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type PartType = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Connector: PartType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Subunit: PartType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS_ID: i32 = 1114112i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_APPLICATION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ASYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_FILENAME: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_LOOP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_MEMORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NODEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NOSTOP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NOWAIT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_PURGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_RESOURCE: i32 = 262148i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_RING: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SENTRY: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SYNC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SYSTEM: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type SPATIAL_AUDIO_STREAM_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_DESTROYED: ::windows_sys::core::HRESULT = -2004287232i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: ::windows_sys::core::HRESULT = -2004287227i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_INTERNAL: ::windows_sys::core::HRESULT = -2004287219i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_INVALID_LICENSE: ::windows_sys::core::HRESULT = -2004287224i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2004287226i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: ::windows_sys::core::HRESULT = -2004287229i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: ::windows_sys::core::HRESULT = -2004287220i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: ::windows_sys::core::HRESULT = -2004287231i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2004287228i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: ::windows_sys::core::HRESULT = -2004287230i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2004287221i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2004287225i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: ::windows_sys::core::HRESULT = -2004287222i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: ::windows_sys::core::HRESULT = -2004286956i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: ::windows_sys::core::HRESULT = -2004286969i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: ::windows_sys::core::HRESULT = -2004286968i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: ::windows_sys::core::HRESULT = -2004286940i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: ::windows_sys::core::HRESULT = -2004286942i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: ::windows_sys::core::HRESULT = -2004286976i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: ::windows_sys::core::HRESULT = -2004286955i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: ::windows_sys::core::HRESULT = -2004286941i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: ::windows_sys::core::HRESULT = -2004286967i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: ::windows_sys::core::HRESULT = -2004286952i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: ::windows_sys::core::HRESULT = -2004286974i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: ::windows_sys::core::HRESULT = -2004286957i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: ::windows_sys::core::HRESULT = -2004286939i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: ::windows_sys::core::HRESULT = -2004286959i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: ::windows_sys::core::HRESULT = -2004286951i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: ::windows_sys::core::HRESULT = -2004286971i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: ::windows_sys::core::HRESULT = -2004286973i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: ::windows_sys::core::HRESULT = -2004286954i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: ::windows_sys::core::HRESULT = -2004286944i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: ::windows_sys::core::HRESULT = -2004286960i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: ::windows_sys::core::HRESULT = -2004286958i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: ::windows_sys::core::HRESULT = -2004286943i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: ::windows_sys::core::HRESULT = -2004286970i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: ::windows_sys::core::HRESULT = -2004286953i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2004286975i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: ::windows_sys::core::HRESULT = -2004286972i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: ::windows_sys::core::GUID,
    pub appId: ::windows_sys::core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
impl ::core::marker::Copy for SpatialAudioClientActivationParams {}
impl ::core::clone::Clone for SpatialAudioClientActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioHrtfActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
    pub DistanceDecay: *mut SpatialAudioHrtfDistanceDecay,
    pub Directivity: *mut SpatialAudioHrtfDirectivityUnion,
    pub Environment: *mut SpatialAudioHrtfEnvironmentType,
    pub Orientation: *mut f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SpatialAudioHrtfActivationParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SpatialAudioHrtfActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioHrtfActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
    pub DistanceDecay: *mut SpatialAudioHrtfDistanceDecay,
    pub Directivity: *mut SpatialAudioHrtfDirectivityUnion,
    pub Environment: *mut SpatialAudioHrtfEnvironmentType,
    pub Orientation: *mut f32,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SpatialAudioHrtfActivationParams2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SpatialAudioHrtfActivationParams2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivity {
    pub Type: SpatialAudioHrtfDirectivityType,
    pub Scaling: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivity {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivityCardioid {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub Order: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityCardioid {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityCardioid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivityCone {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub InnerAngle: f32,
    pub OuterAngle: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityCone {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityCone {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type SpatialAudioHrtfDirectivityType = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union SpatialAudioHrtfDirectivityUnion {
    pub Cone: SpatialAudioHrtfDirectivityCone,
    pub Cardiod: SpatialAudioHrtfDirectivityCardioid,
    pub Omni: SpatialAudioHrtfDirectivity,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityUnion {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityUnion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDistanceDecay {
    pub Type: SpatialAudioHrtfDistanceDecayType,
    pub MaxGain: f32,
    pub MinGain: f32,
    pub UnityGainDistance: f32,
    pub CutoffDistance: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDistanceDecay {}
impl ::core::clone::Clone for SpatialAudioHrtfDistanceDecay {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type SpatialAudioHrtfDistanceDecayType = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type SpatialAudioHrtfEnvironmentType = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type SpatialAudioMetadataCopyMode = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = 3i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
impl ::core::marker::Copy for SpatialAudioMetadataItemsInfo {}
impl ::core::clone::Clone for SpatialAudioMetadataItemsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type SpatialAudioMetadataWriterOverflowMode = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = 2i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SpatialAudioObjectRenderStreamActivationParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SpatialAudioObjectRenderStreamActivationParams2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamActivationParams2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows_sys::core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for SpatialAudioObjectRenderStreamForMetadataActivationParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows_sys::core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: ISpatialAudioObjectRenderStreamNotify,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK: &str = "VAD\\Process_Loopback";
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
impl ::core::marker::Copy for VOLUMEWAVEFILTER {}
impl ::core::clone::Clone for VOLUMEWAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_PITCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_SYNC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl ::core::marker::Copy for WAVEFILTER {}
impl ::core::clone::Clone for WAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
impl ::core::marker::Copy for WAVEFORMAT {}
impl ::core::clone::Clone for WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for WAVEFORMATEX {}
impl ::core::clone::Clone for WAVEFORMATEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for WAVEFORMATEXTENSIBLE {}
impl ::core::clone::Clone for WAVEFORMATEXTENSIBLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
impl ::core::marker::Copy for WAVEFORMATEXTENSIBLE_0 {}
impl ::core::clone::Clone for WAVEFORMATEXTENSIBLE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEHDR {
    pub lpData: ::windows_sys::core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwLoops: u32,
    pub lpNext: *mut WAVEHDR,
    pub reserved: usize,
}
impl ::core::marker::Copy for WAVEHDR {}
impl ::core::clone::Clone for WAVEHDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAVEINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAVEINCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for WAVEINCAPS2W {}
impl ::core::clone::Clone for WAVEINCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAVEINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAVEINCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for WAVEINCAPSW {}
impl ::core::clone::Clone for WAVEINCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAVEOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAVEOUTCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows_sys::core::GUID,
    pub ProductGuid: ::windows_sys::core::GUID,
    pub NameGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for WAVEOUTCAPS2W {}
impl ::core::clone::Clone for WAVEOUTCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAVEOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAVEOUTCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for WAVEOUTCAPSW {}
impl ::core::clone::Clone for WAVEOUTCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_BADFORMAT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_LASTERROR: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_STILLPLAYING: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_SYNC: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_UNPREPARED: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1M08: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1M16: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1S08: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1S16: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2M08: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2M16: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2S08: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2S16: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44M08: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44S08: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4M08: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4S08: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_PCM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_BEGINLOOP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_ENDLOOP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_INQUEUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type _AUDCLNT_BUFFERFLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = 3i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct tACMDRVOPENDESCA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: ::windows_sys::core::PCSTR,
    pub pszAliasName: ::windows_sys::core::PCSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for tACMDRVOPENDESCA {}
impl ::core::clone::Clone for tACMDRVOPENDESCA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct tACMDRVOPENDESCW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: ::windows_sys::core::PCWSTR,
    pub pszAliasName: ::windows_sys::core::PCWSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for tACMDRVOPENDESCW {}
impl ::core::clone::Clone for tACMDRVOPENDESCW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct tACMFORMATDETAILSW {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [u16; 128],
}
impl ::core::marker::Copy for tACMFORMATDETAILSW {}
impl ::core::clone::Clone for tACMFORMATDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
