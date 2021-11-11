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
    pub fn ActivateAudioInterfaceAsync(deviceinterfacepath: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, activationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, completionhandler: ::windows::runtime::RawPtr, activationoperation: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CoRegisterMessageFilter(lpmessagefilter: ::windows::runtime::RawPtr, lplpmessagefilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitor(audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitor(audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundA(pszsound: super::super::Foundation::PSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundW(pszsound: super::super::Foundation::PWSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddA(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddW(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverClose(had: HACMDRIVER, fdwclose: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn acmDriverDetailsA(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn acmDriverDetailsW(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverEnum(fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverID(hao: HACMOBJ, phadid: *mut isize, fdwdriverid: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverMessage(had: HACMDRIVER, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverOpen(phad: *mut isize, hadid: HACMDRIVERID, fdwopen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverPriority(hadid: HACMDRIVERID, dwpriority: u32, fdwpriority: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverRemove(hadid: HACMDRIVERID, fdwremove: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseA(pafltrc: *mut ::core::mem::ManuallyDrop<ACMFILTERCHOOSEA>) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseW(pafltrc: *mut ::core::mem::ManuallyDrop<ACMFILTERCHOOSEW>) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterDetailsA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFilterDetailsW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFilterTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseA(pafmtc: *mut ::core::mem::ManuallyDrop<ACMFORMATCHOOSEA>) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseW(pafmtc: *mut ::core::mem::ManuallyDrop<ACMFORMATCHOOSEW>) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatDetailsA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatDetailsW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatSuggest(had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmGetVersion() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmMetrics(hao: HACMOBJ, umetric: u32, pmetric: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamClose(has: HACMSTREAM, fdwclose: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamConvert(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmStreamMessage(has: HACMSTREAM, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamOpen(phas: *mut isize, had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamPrepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamReset(has: HACMSTREAM, fdwreset: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamSize(has: HACMSTREAM, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamUnprepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiDisconnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInAddBuffer(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInClose(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetID(hmi: HMIDIIN, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInMessage(hmi: HMIDIIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInPrepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInReset(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInStart(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInStop(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: u32, pwkya: *const u16, fucache: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutCachePatches(hmo: HMIDIOUT, ubank: u32, pwpa: *const u16, fucache: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutClose(hmo: HMIDIOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetVolume(hmo: HMIDIOUT, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutLongMsg(hmo: HMIDIOUT, pmh: *const MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutMessage(hmo: HMIDIOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutReset(hmo: HMIDIOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutSetVolume(hmo: HMIDIOUT, dwvolume: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamClose(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: *mut u32, cmidi: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiStreamOut(hms: HMIDISTRM, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamPause(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamPosition(hms: HMIDISTRM, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamProperty(hms: HMIDISTRM, lppropdata: *mut u8, dwproperty: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamRestart(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamStop(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerClose(hmx: HMIXER) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsA(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsW(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetID(hmxobj: HMIXEROBJ, pumxid: *mut u32, fdwid: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineControlsA(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetLineControlsW(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineInfoA(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetLineInfoW(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerMessage(hmx: HMIXER, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerOpen(phmx: *mut isize, umxid: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerSetControlDetails(hmxobj: HMIXEROBJ, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundA(pszsound: super::super::Foundation::PSTR, fusound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundW(pszsound: super::super::Foundation::PWSTR, fusound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInAddBuffer(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInClose(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetID(hwi: HWAVEIN, pudeviceid: *const u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetPosition(hwi: HWAVEIN, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInMessage(hwi: HWAVEIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInOpen(phwi: *mut HWAVEIN, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInPrepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInReset(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInStart(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInStop(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutBreakLoop(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutClose(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetVolume(hwo: HWAVEOUT, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutMessage(hwo: HWAVEOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutOpen(phwo: *mut HWAVEOUT, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutPause(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutReset(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutRestart(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetVolume(hwo: HWAVEOUT, dwvolume: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutWrite(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
}
