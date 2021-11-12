#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ActivateAudioInterfaceAsync(deviceinterfacepath: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, completionhandler: IActivateAudioInterfaceCompletionHandler, activationoperation: *mut IActivateAudioInterfaceAsyncOperation) -> ::windows_sys::core::HRESULT;
    pub fn CoRegisterMessageFilter(lpmessagefilter: IMessageFilter, lplpmessagefilter: *mut IMessageFilter) -> ::windows_sys::core::HRESULT;
    pub fn CreateCaptureAudioStateMonitor(audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    pub fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    pub fn CreateRenderAudioStateMonitor(audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    pub fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundA(pszsound: super::super::Foundation::PSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundW(pszsound: super::super::Foundation::PWSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddA(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddW(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
    pub fn acmDriverClose(had: HACMDRIVER, fdwclose: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn acmDriverDetailsA(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn acmDriverDetailsW(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverEnum(fncallback: ACMDRIVERENUMCB, dwinstance: usize, fdwenum: u32) -> u32;
    pub fn acmDriverID(hao: HACMOBJ, phadid: *mut isize, fdwdriverid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverMessage(had: HACMDRIVER, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    pub fn acmDriverOpen(phad: *mut isize, hadid: HACMDRIVERID, fdwopen: u32) -> u32;
    pub fn acmDriverPriority(hadid: HACMDRIVERID, dwpriority: u32, fdwpriority: u32) -> u32;
    pub fn acmDriverRemove(hadid: HACMDRIVERID, fdwremove: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterDetailsA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32;
    pub fn acmFilterDetailsW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fncallback: ACMFILTERENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fncallback: ACMFILTERENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32;
    pub fn acmFilterTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ACMFILTERTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ACMFILTERTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatDetailsA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32;
    pub fn acmFormatDetailsW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fncallback: ACMFORMATENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fncallback: ACMFORMATENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    pub fn acmFormatSuggest(had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32;
    pub fn acmFormatTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ACMFORMATTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ACMFORMATTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    pub fn acmGetVersion() -> u32;
    pub fn acmMetrics(hao: HACMOBJ, umetric: u32, pmetric: *mut ::core::ffi::c_void) -> u32;
    pub fn acmStreamClose(has: HACMSTREAM, fdwclose: u32) -> u32;
    pub fn acmStreamConvert(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmStreamMessage(has: HACMSTREAM, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> u32;
    pub fn acmStreamOpen(phas: *mut isize, had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    pub fn acmStreamPrepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32;
    pub fn acmStreamReset(has: HACMSTREAM, fdwreset: u32) -> u32;
    pub fn acmStreamSize(has: HACMSTREAM, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32;
    pub fn acmStreamUnprepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32;
    pub fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32;
    pub fn auxGetNumDevs() -> u32;
    pub fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32;
    pub fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: usize, dw2: usize) -> u32;
    pub fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32;
    pub fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
    pub fn midiDisconnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInAddBuffer(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    pub fn midiInClose(hmi: HMIDIIN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32;
    pub fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    pub fn midiInGetID(hmi: HMIDIIN, pudeviceid: *mut u32) -> u32;
    pub fn midiInGetNumDevs() -> u32;
    pub fn midiInMessage(hmi: HMIDIIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
    pub fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInPrepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    pub fn midiInReset(hmi: HMIDIIN) -> u32;
    pub fn midiInStart(hmi: HMIDIIN) -> u32;
    pub fn midiInStop(hmi: HMIDIIN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    pub fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: u32, pwkya: *const u16, fucache: u32) -> u32;
    pub fn midiOutCachePatches(hmo: HMIDIOUT, ubank: u32, pwpa: *const u16, fucache: u32) -> u32;
    pub fn midiOutClose(hmo: HMIDIOUT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32;
    pub fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    pub fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: *mut u32) -> u32;
    pub fn midiOutGetNumDevs() -> u32;
    pub fn midiOutGetVolume(hmo: HMIDIOUT, pdwvolume: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutLongMsg(hmo: HMIDIOUT, pmh: *const MIDIHDR, cbmh: u32) -> u32;
    pub fn midiOutMessage(hmo: HMIDIOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
    pub fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    pub fn midiOutReset(hmo: HMIDIOUT) -> u32;
    pub fn midiOutSetVolume(hmo: HMIDIOUT, dwvolume: u32) -> u32;
    pub fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    pub fn midiStreamClose(hms: HMIDISTRM) -> u32;
    pub fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: *mut u32, cmidi: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiStreamOut(hms: HMIDISTRM, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    pub fn midiStreamPause(hms: HMIDISTRM) -> u32;
    pub fn midiStreamPosition(hms: HMIDISTRM, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    pub fn midiStreamProperty(hms: HMIDISTRM, lppropdata: *mut u8, dwproperty: u32) -> u32;
    pub fn midiStreamRestart(hms: HMIDISTRM) -> u32;
    pub fn midiStreamStop(hms: HMIDISTRM) -> u32;
    pub fn mixerClose(hmx: HMIXER) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsA(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsW(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32;
    pub fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32;
    pub fn mixerGetID(hmxobj: HMIXEROBJ, pumxid: *mut u32, fdwid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineControlsA(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32;
    pub fn mixerGetLineControlsW(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineInfoA(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32;
    pub fn mixerGetLineInfoW(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32;
    pub fn mixerGetNumDevs() -> u32;
    pub fn mixerMessage(hmx: HMIXER, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
    pub fn mixerOpen(phmx: *mut isize, umxid: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerSetControlDetails(hmxobj: HMIXEROBJ, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundA(pszsound: super::super::Foundation::PSTR, fusound: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundW(pszsound: super::super::Foundation::PWSTR, fusound: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInAddBuffer(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    pub fn waveInClose(hwi: HWAVEIN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32;
    pub fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    pub fn waveInGetID(hwi: HWAVEIN, pudeviceid: *const u32) -> u32;
    pub fn waveInGetNumDevs() -> u32;
    pub fn waveInGetPosition(hwi: HWAVEIN, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    pub fn waveInMessage(hwi: HWAVEIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
    pub fn waveInOpen(phwi: *mut HWAVEIN, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInPrepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    pub fn waveInReset(hwi: HWAVEIN) -> u32;
    pub fn waveInStart(hwi: HWAVEIN) -> u32;
    pub fn waveInStop(hwi: HWAVEIN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    pub fn waveOutBreakLoop(hwo: HWAVEOUT) -> u32;
    pub fn waveOutClose(hwo: HWAVEOUT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32;
    pub fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    pub fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: *mut u32) -> u32;
    pub fn waveOutGetNumDevs() -> u32;
    pub fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: *mut u32) -> u32;
    pub fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: *mut u32) -> u32;
    pub fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    pub fn waveOutGetVolume(hwo: HWAVEOUT, pdwvolume: *mut u32) -> u32;
    pub fn waveOutMessage(hwo: HWAVEOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
    pub fn waveOutOpen(phwo: *mut HWAVEOUT, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    pub fn waveOutPause(hwo: HWAVEOUT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    pub fn waveOutReset(hwo: HWAVEOUT) -> u32;
    pub fn waveOutRestart(hwo: HWAVEOUT) -> u32;
    pub fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: u32) -> u32;
    pub fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: u32) -> u32;
    pub fn waveOutSetVolume(hwo: HWAVEOUT, dwvolume: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutWrite(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
}
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
pub const ACMDM_USER: u32 = 16384u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct ACMDRIVERDETAILSA(i32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct ACMDRIVERDETAILSW(i32);
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMDRIVERENUMCB = unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[repr(C)]
pub struct ACMDRVFORMATSUGGEST(i32);
#[repr(C)]
pub struct ACMDRVSTREAMHEADER(i32);
#[repr(C)]
pub struct ACMDRVSTREAMINSTANCE(i32);
#[repr(C)]
pub struct ACMDRVSTREAMSIZE(i32);
pub const ACMERR_BASE: u32 = 512u32;
pub const ACMERR_BUSY: u32 = 513u32;
pub const ACMERR_CANCELED: u32 = 515u32;
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACMFILTERCHOOSEA(i32);
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCA = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCW = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACMFILTERCHOOSEW(i32);
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACMFILTERDETAILSA(i32);
#[repr(C)]
pub struct ACMFILTERDETAILSW(i32);
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACMFILTERTAGDETAILSA(i32);
#[repr(C)]
pub struct ACMFILTERTAGDETAILSW(i32);
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACMFORMATCHOOSEA(i32);
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCA = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCW = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACMFORMATCHOOSEW(i32);
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACMFORMATDETAILSA(i32);
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACMFORMATTAGDETAILSA(i32);
#[repr(C)]
pub struct ACMFORMATTAGDETAILSW(i32);
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct ACMSTREAMHEADER(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct ACMSTREAMHEADER(i32);
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[repr(transparent)]
pub struct AMBISONICS_CHANNEL_ORDERING(pub i32);
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = AMBISONICS_CHANNEL_ORDERING(0i32);
#[repr(transparent)]
pub struct AMBISONICS_NORMALIZATION(pub i32);
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(0i32);
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(1i32);
#[repr(C)]
pub struct AMBISONICS_PARAMS(i32);
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[repr(transparent)]
pub struct AMBISONICS_TYPE(pub i32);
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = AMBISONICS_TYPE(0i32);
pub const AUDCLNT_E_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287486i32 as _);
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287469i32 as _);
pub const AUDCLNT_E_BUFFER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287464i32 as _);
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287477i32 as _);
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287466i32 as _);
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287463i32 as _);
pub const AUDCLNT_E_BUFFER_TOO_LARGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287482i32 as _);
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287465i32 as _);
pub const AUDCLNT_E_DEVICE_INVALIDATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287484i32 as _);
pub const AUDCLNT_E_DEVICE_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287478i32 as _);
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287423i32 as _);
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287422i32 as _);
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287473i32 as _);
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287454i32 as _);
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287447i32 as _);
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287448i32 as _);
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287471i32 as _);
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287468i32 as _);
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287474i32 as _);
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287470i32 as _);
pub const AUDCLNT_E_HEADTRACKING_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287440i32 as _);
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287424i32 as _);
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287467i32 as _);
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287456i32 as _);
pub const AUDCLNT_E_INVALID_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287479i32 as _);
pub const AUDCLNT_E_INVALID_STREAM_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287455i32 as _);
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287451i32 as _);
pub const AUDCLNT_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287487i32 as _);
pub const AUDCLNT_E_NOT_STOPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287483i32 as _);
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287452i32 as _);
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287453i32 as _);
pub const AUDCLNT_E_OUT_OF_ORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287481i32 as _);
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287449i32 as _);
pub const AUDCLNT_E_RESOURCES_INVALIDATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287450i32 as _);
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287472i32 as _);
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287476i32 as _);
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287480i32 as _);
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287485i32 as _);
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[repr(transparent)]
pub struct AUDCLNT_SHAREMODE(pub i32);
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(0i32);
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(1i32);
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[repr(transparent)]
pub struct AUDCLNT_STREAMOPTIONS(pub u32);
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(0u32);
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(1u32);
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(2u32);
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(4u32);
pub const AUDCLNT_S_BUFFER_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(143196161i32 as _);
pub const AUDCLNT_S_POSITION_STALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(143196163i32 as _);
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(143196162i32 as _);
#[repr(C)]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS(i32);
#[repr(transparent)]
pub struct AUDIOCLIENT_ACTIVATION_TYPE(pub i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(0i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(1i32);
#[repr(C)]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS(i32);
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[repr(transparent)]
pub struct AUDIO_DUCKING_OPTIONS(pub u32);
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(0u32);
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AUDIO_EFFECT(i32);
#[repr(transparent)]
pub struct AUDIO_EFFECT_STATE(pub i32);
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(0i32);
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(1i32);
#[repr(transparent)]
pub struct AUDIO_STREAM_CATEGORY(pub i32);
pub const AudioCategory_Other: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(0i32);
pub const AudioCategory_ForegroundOnlyMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(1i32);
pub const AudioCategory_Communications: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(3i32);
pub const AudioCategory_Alerts: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(4i32);
pub const AudioCategory_SoundEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(5i32);
pub const AudioCategory_GameEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(6i32);
pub const AudioCategory_GameMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(7i32);
pub const AudioCategory_GameChat: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(8i32);
pub const AudioCategory_Speech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(9i32);
pub const AudioCategory_Movie: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(10i32);
pub const AudioCategory_Media: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(11i32);
pub const AudioCategory_FarFieldSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(12i32);
pub const AudioCategory_UniformSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(13i32);
pub const AudioCategory_VoiceTyping: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(14i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AUXCAPS2A(i32);
#[repr(C)]
pub struct AUXCAPS2W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AUXCAPSA(i32);
#[repr(C)]
pub struct AUXCAPSW(i32);
pub const AUXCAPS_AUXIN: u32 = 2u32;
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
pub const AUXCAPS_VOLUME: u32 = 1u32;
#[repr(C)]
pub struct AudioClient3ActivationParams(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AudioClientProperties(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AudioExtensionParams(i32);
#[repr(transparent)]
pub struct AudioObjectType(pub u32);
pub const AudioObjectType_None: AudioObjectType = AudioObjectType(0u32);
pub const AudioObjectType_Dynamic: AudioObjectType = AudioObjectType(1u32);
pub const AudioObjectType_FrontLeft: AudioObjectType = AudioObjectType(2u32);
pub const AudioObjectType_FrontRight: AudioObjectType = AudioObjectType(4u32);
pub const AudioObjectType_FrontCenter: AudioObjectType = AudioObjectType(8u32);
pub const AudioObjectType_LowFrequency: AudioObjectType = AudioObjectType(16u32);
pub const AudioObjectType_SideLeft: AudioObjectType = AudioObjectType(32u32);
pub const AudioObjectType_SideRight: AudioObjectType = AudioObjectType(64u32);
pub const AudioObjectType_BackLeft: AudioObjectType = AudioObjectType(128u32);
pub const AudioObjectType_BackRight: AudioObjectType = AudioObjectType(256u32);
pub const AudioObjectType_TopFrontLeft: AudioObjectType = AudioObjectType(512u32);
pub const AudioObjectType_TopFrontRight: AudioObjectType = AudioObjectType(1024u32);
pub const AudioObjectType_TopBackLeft: AudioObjectType = AudioObjectType(2048u32);
pub const AudioObjectType_TopBackRight: AudioObjectType = AudioObjectType(4096u32);
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = AudioObjectType(8192u32);
pub const AudioObjectType_BottomFrontRight: AudioObjectType = AudioObjectType(16384u32);
pub const AudioObjectType_BottomBackLeft: AudioObjectType = AudioObjectType(32768u32);
pub const AudioObjectType_BottomBackRight: AudioObjectType = AudioObjectType(65536u32);
pub const AudioObjectType_BackCenter: AudioObjectType = AudioObjectType(131072u32);
#[repr(transparent)]
pub struct AudioSessionDisconnectReason(pub i32);
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = AudioSessionDisconnectReason(0i32);
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = AudioSessionDisconnectReason(1i32);
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = AudioSessionDisconnectReason(2i32);
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = AudioSessionDisconnectReason(3i32);
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = AudioSessionDisconnectReason(4i32);
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = AudioSessionDisconnectReason(5i32);
#[repr(transparent)]
pub struct AudioSessionState(pub i32);
pub const AudioSessionStateInactive: AudioSessionState = AudioSessionState(0i32);
pub const AudioSessionStateActive: AudioSessionState = AudioSessionState(1i32);
pub const AudioSessionStateExpired: AudioSessionState = AudioSessionState(2i32);
#[repr(transparent)]
pub struct AudioStateMonitorSoundLevel(pub i32);
pub const Muted: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(0i32);
pub const Low: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(1i32);
pub const Full: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(2i32);
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
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
pub const DEVICE_STATE_ACTIVE: u32 = 1u32;
pub const DEVICE_STATE_DISABLED: u32 = 2u32;
pub const DEVICE_STATE_NOTPRESENT: u32 = 4u32;
pub const DEVICE_STATE_UNPLUGGED: u32 = 8u32;
pub const DEVINTERFACE_AUDIO_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 787448254, data2: 13306, data3: 18432, data4: [150, 112, 28, 212, 116, 151, 44, 63] };
pub const DEVINTERFACE_AUDIO_RENDER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3862068397,
    data2: 56556,
    data3: 18761,
    data4: [174, 138, 153, 30, 151, 106, 121, 210],
};
pub const DEVINTERFACE_MIDI_INPUT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1347150636, data2: 52470, data3: 19756, data4: [183, 63, 111, 139, 55, 71, 226, 43] };
pub const DEVINTERFACE_MIDI_OUTPUT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1841443616,
    data2: 43827,
    data3: 19684,
    data4: [128, 212, 187, 179, 235, 191, 40, 20],
};
#[repr(C)]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS(i32);
pub const DRVM_MAPPER: u32 = 8192u32;
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[repr(transparent)]
pub struct DataFlow(pub i32);
pub const In: DataFlow = DataFlow(0i32);
pub const Out: DataFlow = DataFlow(1i32);
#[repr(C)]
pub struct DeviceTopology(i32);
#[repr(C)]
pub struct ECHOWAVEFILTER(i32);
#[repr(transparent)]
pub struct EDataFlow(pub i32);
pub const eRender: EDataFlow = EDataFlow(0i32);
pub const eCapture: EDataFlow = EDataFlow(1i32);
pub const eAll: EDataFlow = EDataFlow(2i32);
pub const EDataFlow_enum_count: EDataFlow = EDataFlow(3i32);
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
#[repr(transparent)]
pub struct ERole(pub i32);
pub const eConsole: ERole = ERole(0i32);
pub const eMultimedia: ERole = ERole(1i32);
pub const eCommunications: ERole = ERole(2i32);
pub const ERole_enum_count: ERole = ERole(3i32);
pub const EVENTCONTEXT_VOLUMESLIDER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3804424670, data2: 2481, data3: 19204, data4: [132, 229, 7, 147, 18, 37, 238, 4] };
#[repr(transparent)]
pub struct EndpointFormFactor(pub i32);
pub const RemoteNetworkDevice: EndpointFormFactor = EndpointFormFactor(0i32);
pub const Speakers: EndpointFormFactor = EndpointFormFactor(1i32);
pub const LineLevel: EndpointFormFactor = EndpointFormFactor(2i32);
pub const Headphones: EndpointFormFactor = EndpointFormFactor(3i32);
pub const Microphone: EndpointFormFactor = EndpointFormFactor(4i32);
pub const Headset: EndpointFormFactor = EndpointFormFactor(5i32);
pub const Handset: EndpointFormFactor = EndpointFormFactor(6i32);
pub const UnknownDigitalPassthrough: EndpointFormFactor = EndpointFormFactor(7i32);
pub const SPDIF: EndpointFormFactor = EndpointFormFactor(8i32);
pub const DigitalAudioDisplayDevice: EndpointFormFactor = EndpointFormFactor(9i32);
pub const UnknownFormFactor: EndpointFormFactor = EndpointFormFactor(10i32);
pub const EndpointFormFactor_enum_count: EndpointFormFactor = EndpointFormFactor(11i32);
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
#[repr(C)]
pub struct HACMDRIVER(i32);
#[repr(C)]
pub struct HACMDRIVERID(i32);
#[repr(C)]
pub struct HACMOBJ(i32);
#[repr(C)]
pub struct HACMSTREAM(i32);
#[repr(C)]
pub struct HMIDI(i32);
#[repr(C)]
pub struct HMIDIIN(i32);
#[repr(C)]
pub struct HMIDIOUT(i32);
#[repr(C)]
pub struct HMIDISTRM(i32);
#[repr(C)]
pub struct HMIXER(i32);
#[repr(C)]
pub struct HMIXEROBJ(i32);
#[repr(C)]
pub struct HWAVE(i32);
#[repr(C)]
pub struct HWAVEIN(i32);
#[repr(C)]
pub struct HWAVEOUT(i32);
#[repr(transparent)]
pub struct IActivateAudioInterfaceAsyncOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivateAudioInterfaceCompletionHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioAmbisonicsControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioAutoGainControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioBass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioCaptureClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioChannelConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioClient2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioClient3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioClientDuckingControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioClock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioClock2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioClockAdjustment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEffectsChangedNotificationClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEffectsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioFormatEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioInputSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioLoudness(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioMidrange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioMute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioOutputSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioPeakMeter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioRenderClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSessionControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSessionControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSessionEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSessionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSessionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSessionManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSessionNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStateMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStreamVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSystemEffectsPropertyStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioTreble(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioVolumeDuckNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioVolumeLevel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChannelAudioVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlChangeNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlInterface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceSpecificProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceTopology(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMMDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMMDeviceActivator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMMDeviceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMMDeviceEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMMEndpoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMMNotificationClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPart(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPartsList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerChannelDbLevel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleAudioVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioClient2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioMetadataClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioMetadataCopier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioMetadataItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioMetadataItemsBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioMetadataReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioMetadataWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectForHrtf(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectForMetadataCommands(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectForMetadataItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamForHrtf(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamForMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISubunit(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type LPACMDRIVERPROC = unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
pub const MEVT_F_LONG: i32 = -2147483648i32;
pub const MEVT_F_SHORT: i32 = 0i32;
pub const MHDR_DONE: u32 = 1u32;
pub const MHDR_INQUEUE: u32 = 4u32;
pub const MHDR_ISSTRM: u32 = 8u32;
pub const MHDR_PREPARED: u32 = 2u32;
pub const MIDICAPS_CACHE: u32 = 4u32;
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
pub const MIDICAPS_STREAM: u32 = 8u32;
pub const MIDICAPS_VOLUME: u32 = 1u32;
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
pub const MIDIERR_LASTERROR: u32 = 71u32;
pub const MIDIERR_NODEVICE: u32 = 68u32;
pub const MIDIERR_NOMAP: u32 = 66u32;
pub const MIDIERR_NOTREADY: u32 = 67u32;
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[repr(C)]
pub struct MIDIEVENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIDIHDR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIDIINCAPS2A(i32);
#[repr(C)]
pub struct MIDIINCAPS2W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIDIINCAPSA(i32);
#[repr(C)]
pub struct MIDIINCAPSW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIDIOUTCAPS2A(i32);
#[repr(C)]
pub struct MIDIOUTCAPS2W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIDIOUTCAPSA(i32);
#[repr(C)]
pub struct MIDIOUTCAPSW(i32);
pub const MIDIPATCHSIZE: u32 = 128u32;
#[repr(C)]
pub struct MIDIPROPTEMPO(i32);
#[repr(C)]
pub struct MIDIPROPTIMEDIV(i32);
pub const MIDIPROP_GET: i32 = 1073741824i32;
pub const MIDIPROP_SET: i32 = -2147483648i32;
pub const MIDIPROP_TEMPO: i32 = 2i32;
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[repr(C)]
pub struct MIDISTRMBUFFVER(i32);
pub const MIDISTRM_ERROR: i32 = -2i32;
pub const MIDI_CACHE_ALL: u32 = 1u32;
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
pub const MIDI_CACHE_QUERY: u32 = 3u32;
pub const MIDI_UNCACHE: u32 = 4u32;
#[repr(transparent)]
pub struct MIDI_WAVE_OPEN_TYPE(pub u32);
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(458752u32);
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(0u32);
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(65536u32);
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(196608u32);
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(327680u32);
pub const WAVE_FORMAT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(1u32);
pub const WAVE_ALLOWSYNC: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(2u32);
pub const WAVE_MAPPED: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(4u32);
pub const WAVE_FORMAT_DIRECT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(8u32);
pub const WAVE_FORMAT_DIRECT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(9u32);
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(16u32);
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(32u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIXERCAPS2A(i32);
#[repr(C)]
pub struct MIXERCAPS2W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIXERCAPSA(i32);
#[repr(C)]
pub struct MIXERCAPSW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIXERCONTROLA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIXERCONTROLDETAILS(i32);
#[repr(C)]
pub struct MIXERCONTROLDETAILS_BOOLEAN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIXERCONTROLDETAILS_LISTTEXTA(i32);
#[repr(C)]
pub struct MIXERCONTROLDETAILS_LISTTEXTW(i32);
#[repr(C)]
pub struct MIXERCONTROLDETAILS_SIGNED(i32);
#[repr(C)]
pub struct MIXERCONTROLDETAILS_UNSIGNED(i32);
#[repr(C)]
pub struct MIXERCONTROLW(i32);
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIXERLINEA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MIXERLINECONTROLSA(i32);
#[repr(C)]
pub struct MIXERLINECONTROLSW(i32);
#[repr(C)]
pub struct MIXERLINEW(i32);
#[repr(transparent)]
pub struct MIXERLINE_COMPONENTTYPE(pub u32);
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(1u32);
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(5u32);
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(2u32);
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(3u32);
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4u32);
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(6u32);
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(0u32);
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(8u32);
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(7u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4106u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4105u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4101u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4097u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4098u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4099u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4103u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4100u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4102u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4096u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4104u32);
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
pub const MIXERR_INVALLINE: u32 = 1024u32;
pub const MIXERR_INVALVALUE: u32 = 1026u32;
pub const MIXERR_LASTERROR: u32 = 1026u32;
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
#[repr(C)]
pub struct MMDeviceEnumerator(i32);
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
pub const MOD_FMSYNTH: u32 = 4u32;
pub const MOD_MAPPER: u32 = 5u32;
pub const MOD_MIDIPORT: u32 = 1u32;
pub const MOD_SQSYNTH: u32 = 3u32;
pub const MOD_SWSYNTH: u32 = 7u32;
pub const MOD_SYNTH: u32 = 2u32;
pub const MOD_WAVETABLE: u32 = 6u32;
pub type PAudioStateMonitorCallback = unsafe extern "system" fn(audiostatemonitor: IAudioStateMonitor, context: *const ::core::ffi::c_void);
#[repr(C)]
pub struct PCMWAVEFORMAT(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 4054546445,
        data2: 8208,
        data3: 20179,
        data4: [163, 166, 139, 135, 240, 240, 196, 118],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 4054546445,
        data2: 8208,
        data3: 20179,
        data4: [163, 166, 139, 135, 240, 240, 196, 118],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 337911810, data2: 800, data3: 19940, data4: [149, 85, 167, 216, 43, 115, 194, 134] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_MenuText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 337911810, data2: 800, data3: 19940, data4: [149, 85, 167, 216, 43, 115, 194, 134] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 4053730893,
        data2: 2092,
        data3: 20007,
        data4: [188, 115, 104, 130, 161, 187, 142, 76],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3834056230, data2: 15557, data3: 19666, data4: [186, 70, 202, 10, 154, 112, 237, 4] },
    pid: 3u32,
};
#[repr(transparent)]
pub struct PROCESS_LOOPBACK_MODE(pub i32);
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(0i32);
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(1i32);
#[repr(transparent)]
pub struct PartType(pub i32);
pub const Connector: PartType = PartType(0i32);
pub const Subunit: PartType = PartType(1i32);
pub const SND_ALIAS: i32 = 65536i32;
pub const SND_ALIAS_ID: i32 = 1114112i32;
pub const SND_ALIAS_START: u32 = 0u32;
pub const SND_APPLICATION: u32 = 128u32;
pub const SND_ASYNC: u32 = 1u32;
pub const SND_FILENAME: i32 = 131072i32;
pub const SND_LOOP: u32 = 8u32;
pub const SND_MEMORY: u32 = 4u32;
pub const SND_NODEFAULT: u32 = 2u32;
pub const SND_NOSTOP: u32 = 16u32;
pub const SND_NOWAIT: i32 = 8192i32;
pub const SND_PURGE: u32 = 64u32;
pub const SND_RESOURCE: i32 = 262148i32;
pub const SND_RING: i32 = 1048576i32;
pub const SND_SENTRY: i32 = 524288i32;
pub const SND_SYNC: u32 = 0u32;
pub const SND_SYSTEM: i32 = 2097152i32;
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[repr(transparent)]
pub struct SPATIAL_AUDIO_STREAM_OPTIONS(pub u32);
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(0u32);
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(1u32);
pub const SPTLAUDCLNT_E_DESTROYED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287232i32 as _);
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287227i32 as _);
pub const SPTLAUDCLNT_E_INTERNAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287219i32 as _);
pub const SPTLAUDCLNT_E_INVALID_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287224i32 as _);
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287226i32 as _);
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287229i32 as _);
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287220i32 as _);
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287231i32 as _);
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287228i32 as _);
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287230i32 as _);
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287221i32 as _);
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287225i32 as _);
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287222i32 as _);
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286956i32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286969i32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286968i32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286940i32 as _);
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286942i32 as _);
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286976i32 as _);
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286955i32 as _);
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286941i32 as _);
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286967i32 as _);
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286952i32 as _);
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286974i32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286957i32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286939i32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286959i32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286951i32 as _);
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286971i32 as _);
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286973i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286954i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286944i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286960i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286958i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286943i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286970i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286953i32 as _);
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286975i32 as _);
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286972i32 as _);
#[repr(C)]
pub struct SpatialAudioClientActivationParams(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SpatialAudioHrtfActivationParams(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SpatialAudioHrtfActivationParams2(i32);
#[repr(C)]
pub struct SpatialAudioHrtfDirectivity(i32);
#[repr(C)]
pub struct SpatialAudioHrtfDirectivityCardioid(i32);
#[repr(C)]
pub struct SpatialAudioHrtfDirectivityCone(i32);
#[repr(transparent)]
pub struct SpatialAudioHrtfDirectivityType(pub i32);
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(0i32);
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(1i32);
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(2i32);
#[repr(C)]
pub struct SpatialAudioHrtfDirectivityUnion(i32);
#[repr(C)]
pub struct SpatialAudioHrtfDistanceDecay(i32);
#[repr(transparent)]
pub struct SpatialAudioHrtfDistanceDecayType(pub i32);
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(0i32);
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(1i32);
#[repr(transparent)]
pub struct SpatialAudioHrtfEnvironmentType(pub i32);
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(0i32);
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(1i32);
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(2i32);
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(3i32);
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(4i32);
#[repr(transparent)]
pub struct SpatialAudioMetadataCopyMode(pub i32);
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(0i32);
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(1i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(2i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(3i32);
#[repr(C)]
pub struct SpatialAudioMetadataItemsInfo(i32);
#[repr(transparent)]
pub struct SpatialAudioMetadataWriterOverflowMode(pub i32);
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(0i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(1i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SpatialAudioObjectRenderStreamActivationParams(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SpatialAudioObjectRenderStreamActivationParams2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2(i32);
#[repr(C)]
pub struct VOLUMEWAVEFILTER(i32);
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
pub const WAVECAPS_PITCH: u32 = 1u32;
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
pub const WAVECAPS_SYNC: u32 = 16u32;
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[repr(C)]
pub struct WAVEFILTER(i32);
#[repr(C)]
pub struct WAVEFORMAT(i32);
#[repr(C)]
pub struct WAVEFORMATEX(i32);
#[repr(C)]
pub struct WAVEFORMATEXTENSIBLE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WAVEHDR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WAVEINCAPS2A(i32);
#[repr(C)]
pub struct WAVEINCAPS2W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WAVEINCAPSA(i32);
#[repr(C)]
pub struct WAVEINCAPSW(i32);
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WAVEOUTCAPS2A(i32);
#[repr(C)]
pub struct WAVEOUTCAPS2W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WAVEOUTCAPSA(i32);
#[repr(C)]
pub struct WAVEOUTCAPSW(i32);
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
pub const WAVERR_BADFORMAT: u32 = 32u32;
pub const WAVERR_LASTERROR: u32 = 35u32;
pub const WAVERR_STILLPLAYING: u32 = 33u32;
pub const WAVERR_SYNC: u32 = 35u32;
pub const WAVERR_UNPREPARED: u32 = 34u32;
pub const WAVE_FORMAT_1M08: u32 = 1u32;
pub const WAVE_FORMAT_1M16: u32 = 4u32;
pub const WAVE_FORMAT_1S08: u32 = 2u32;
pub const WAVE_FORMAT_1S16: u32 = 8u32;
pub const WAVE_FORMAT_2M08: u32 = 16u32;
pub const WAVE_FORMAT_2M16: u32 = 64u32;
pub const WAVE_FORMAT_2S08: u32 = 32u32;
pub const WAVE_FORMAT_2S16: u32 = 128u32;
pub const WAVE_FORMAT_44M08: u32 = 256u32;
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
pub const WAVE_FORMAT_44S08: u32 = 512u32;
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
pub const WAVE_FORMAT_4M08: u32 = 256u32;
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
pub const WAVE_FORMAT_4S08: u32 = 512u32;
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
pub const WAVE_FORMAT_PCM: u32 = 1u32;
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
pub const WAVE_MAPPER: u32 = 4294967295u32;
pub const WHDR_BEGINLOOP: u32 = 4u32;
pub const WHDR_DONE: u32 = 1u32;
pub const WHDR_ENDLOOP: u32 = 8u32;
pub const WHDR_INQUEUE: u32 = 16u32;
pub const WHDR_PREPARED: u32 = 2u32;
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[repr(transparent)]
pub struct _AUDCLNT_BUFFERFLAGS(pub i32);
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(1i32);
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(2i32);
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(4i32);
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(pub i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(0i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(1i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(2i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct tACMDRVOPENDESCA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct tACMDRVOPENDESCW(i32);
#[repr(C)]
pub struct tACMFORMATDETAILSW(i32);
