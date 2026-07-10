#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn auxGetDevCapsA(udeviceid : usize, pac : *mut AUXCAPSA, cbac : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn auxGetDevCapsW(udeviceid : usize, pac : *mut AUXCAPSW, cbac : u32) -> super::mmsyscom::MMRESULT);
windows_link::link!("winmm.dll" "system" fn auxGetNumDevs() -> u32);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn auxGetVolume(udeviceid : u32, pdwvolume : *mut u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn auxOutMessage(udeviceid : u32, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn auxSetVolume(udeviceid : u32, dwvolume : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiConnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiDisconnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInAddBuffer(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInClose(hmi : HMIDIIN) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInGetDevCapsA(udeviceid : usize, pmic : *mut MIDIINCAPSA, cbmic : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInGetDevCapsW(udeviceid : usize, pmic : *mut MIDIINCAPSW, cbmic : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInGetErrorTextA(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_sys::core::PSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInGetErrorTextW(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_sys::core::PWSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInGetID(hmi : HMIDIIN, pudeviceid : *mut u32) -> super::mmsyscom::MMRESULT);
windows_link::link!("winmm.dll" "system" fn midiInGetNumDevs() -> u32);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInMessage(hmi : HMIDIIN, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInOpen(phmi : *mut HMIDIIN, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInPrepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInReset(hmi : HMIDIIN) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInStart(hmi : HMIDIIN) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInStop(hmi : HMIDIIN) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiInUnprepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutCacheDrumPatches(hmo : HMIDIOUT, upatch : u32, pwkya : *const u16, fucache : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutCachePatches(hmo : HMIDIOUT, ubank : u32, pwpa : *const u16, fucache : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutClose(hmo : HMIDIOUT) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutGetDevCapsA(udeviceid : usize, pmoc : *mut MIDIOUTCAPSA, cbmoc : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutGetDevCapsW(udeviceid : usize, pmoc : *mut MIDIOUTCAPSW, cbmoc : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutGetErrorTextA(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_sys::core::PSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutGetErrorTextW(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_sys::core::PWSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutGetID(hmo : HMIDIOUT, pudeviceid : *mut u32) -> super::mmsyscom::MMRESULT);
windows_link::link!("winmm.dll" "system" fn midiOutGetNumDevs() -> u32);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutGetVolume(hmo : HMIDIOUT, pdwvolume : *mut u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutLongMsg(hmo : HMIDIOUT, pmh : *const MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutMessage(hmo : HMIDIOUT, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutOpen(phmo : *mut HMIDIOUT, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutPrepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutReset(hmo : HMIDIOUT) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutSetVolume(hmo : HMIDIOUT, dwvolume : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutShortMsg(hmo : HMIDIOUT, dwmsg : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiOutUnprepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiStreamClose(hms : HMIDISTRM) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiStreamOpen(phms : *mut HMIDISTRM, pudeviceid : *mut u32, cmidi : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiStreamOut(hms : HMIDISTRM, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiStreamPause(hms : HMIDISTRM) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiStreamPosition(hms : HMIDISTRM, lpmmt : *mut super::mmsyscom::MMTIME, cbmmt : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiStreamProperty(hms : HMIDISTRM, lppropdata : *mut u8, dwproperty : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiStreamRestart(hms : HMIDISTRM) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn midiStreamStop(hms : HMIDISTRM) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerClose(hmx : HMIXER) -> super::mmsyscom::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "windef"))]
windows_link::link!("winmm.dll" "system" fn mixerGetControlDetailsA(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "windef"))]
windows_link::link!("winmm.dll" "system" fn mixerGetControlDetailsW(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerGetDevCapsA(umxid : usize, pmxcaps : *mut MIXERCAPSA, cbmxcaps : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerGetDevCapsW(umxid : usize, pmxcaps : *mut MIXERCAPSW, cbmxcaps : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerGetID(hmxobj : HMIXEROBJ, pumxid : *mut u32, fdwid : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerGetLineControlsA(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSA, fdwcontrols : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerGetLineControlsW(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSW, fdwcontrols : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerGetLineInfoA(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEA, fdwinfo : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerGetLineInfoW(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEW, fdwinfo : u32) -> super::mmsyscom::MMRESULT);
windows_link::link!("winmm.dll" "system" fn mixerGetNumDevs() -> u32);
windows_link::link!("winmm.dll" "system" fn mixerMessage(hmx : HMIXER, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> u32);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mixerOpen(phmx : *mut HMIXER, umxid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "windef"))]
windows_link::link!("winmm.dll" "system" fn mixerSetControlDetails(hmxobj : HMIXEROBJ, pmxcd : *const MIXERCONTROLDETAILS, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInAddBuffer(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInClose(hwi : HWAVEIN) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInGetDevCapsA(udeviceid : usize, pwic : *mut WAVEINCAPSA, cbwic : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInGetDevCapsW(udeviceid : usize, pwic : *mut WAVEINCAPSW, cbwic : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInGetErrorTextA(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_sys::core::PSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInGetErrorTextW(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_sys::core::PWSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInGetID(hwi : HWAVEIN, pudeviceid : *const u32) -> super::mmsyscom::MMRESULT);
windows_link::link!("winmm.dll" "system" fn waveInGetNumDevs() -> u32);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInGetPosition(hwi : HWAVEIN, pmmt : *mut super::mmsyscom::MMTIME, cbmmt : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInMessage(hwi : HWAVEIN, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInOpen(phwi : *mut HWAVEIN, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInPrepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInReset(hwi : HWAVEIN) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInStart(hwi : HWAVEIN) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInStop(hwi : HWAVEIN) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveInUnprepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutBreakLoop(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutClose(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetDevCapsA(udeviceid : usize, pwoc : *mut WAVEOUTCAPSA, cbwoc : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetDevCapsW(udeviceid : usize, pwoc : *mut WAVEOUTCAPSW, cbwoc : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetErrorTextA(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_sys::core::PSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetErrorTextW(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_sys::core::PWSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetID(hwo : HWAVEOUT, pudeviceid : *mut u32) -> super::mmsyscom::MMRESULT);
windows_link::link!("winmm.dll" "system" fn waveOutGetNumDevs() -> u32);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetPitch(hwo : HWAVEOUT, pdwpitch : *mut u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetPlaybackRate(hwo : HWAVEOUT, pdwrate : *mut u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetPosition(hwo : HWAVEOUT, pmmt : *mut super::mmsyscom::MMTIME, cbmmt : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutGetVolume(hwo : HWAVEOUT, pdwvolume : *mut u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutMessage(hwo : HWAVEOUT, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutOpen(phwo : *mut HWAVEOUT, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutPause(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutPrepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutReset(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutRestart(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutSetPitch(hwo : HWAVEOUT, dwpitch : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutSetPlaybackRate(hwo : HWAVEOUT, dwrate : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutSetVolume(hwo : HWAVEOUT, dwvolume : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutUnprepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn waveOutWrite(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
#[cfg(feature = "mmsyscom")]
pub type AUXCAPS = AUXCAPSA;
#[cfg(feature = "mmsyscom")]
pub type AUXCAPS2 = AUXCAPS2A;
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct AUXCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for AUXCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct AUXCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for AUXCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct AUXCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for AUXCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct AUXCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUXCAPS_AUXIN: u32 = 2;
pub const AUXCAPS_CDAUDIO: u32 = 1;
pub const AUXCAPS_LRVOLUME: u32 = 2;
pub const AUXCAPS_VOLUME: u32 = 1;
pub const AUX_MAPPER: u32 = 4294967295;
pub type HMIDI = *mut core::ffi::c_void;
pub type HMIDIIN = *mut core::ffi::c_void;
pub type HMIDIOUT = *mut core::ffi::c_void;
pub type HMIDISTRM = *mut core::ffi::c_void;
pub type HMIXER = *mut core::ffi::c_void;
pub type HMIXEROBJ = *mut core::ffi::c_void;
pub type HWAVE = *mut core::ffi::c_void;
pub type HWAVEIN = *mut core::ffi::c_void;
pub type HWAVEOUT = *mut core::ffi::c_void;
pub type KEYARRAY = [u16; 128];
#[cfg(feature = "mmsyscom")]
pub type LPAUXCAPS = LPAUXCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPAUXCAPS2 = LPAUXCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPAUXCAPS2A = *mut AUXCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPAUXCAPS2W = *mut AUXCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type LPAUXCAPSA = *mut AUXCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPAUXCAPSW = *mut AUXCAPSW;
pub type LPCWAVEFORMATEX = *const WAVEFORMATEX;
pub type LPHMIDI = *mut HMIDI;
pub type LPHMIDIIN = *mut HMIDIIN;
pub type LPHMIDIOUT = *mut HMIDIOUT;
pub type LPHMIDISTRM = *mut HMIDISTRM;
pub type LPHMIXER = *mut HMIXER;
pub type LPHMIXEROBJ = *mut HMIXEROBJ;
pub type LPHWAVEIN = *mut HWAVEIN;
pub type LPHWAVEOUT = *mut HWAVEOUT;
pub type LPKEYARRAY = *mut u16;
pub type LPMIDICALLBACK = Option<unsafe extern "system" fn()>;
pub type LPMIDIHDR = *mut MIDIHDR;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIINCAPS = LPMIDIINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIINCAPS2 = LPMIDIINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIINCAPS2A = *mut MIDIINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIINCAPS2W = *mut MIDIINCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIINCAPSA = *mut MIDIINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIINCAPSW = *mut MIDIINCAPSW;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIOUTCAPS = LPMIDIOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIOUTCAPS2 = LPMIDIOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
pub type LPMIDIPROPTEMPO = *mut MIDIPROPTEMPO;
pub type LPMIDIPROPTIMEDIV = *mut MIDIPROPTIMEDIV;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERCAPS = LPMIXERCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERCAPS2 = LPMIXERCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERCAPS2A = *mut MIXERCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERCAPS2W = *mut MIXERCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERCAPSA = *mut MIXERCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERCAPSW = *mut MIXERCAPSW;
pub type LPMIXERCONTROL = LPMIXERCONTROLA;
pub type LPMIXERCONTROLA = *mut MIXERCONTROLA;
#[cfg(feature = "windef")]
pub type LPMIXERCONTROLDETAILS = *mut MIXERCONTROLDETAILS;
pub type LPMIXERCONTROLDETAILS_BOOLEAN = *mut MIXERCONTROLDETAILS_BOOLEAN;
pub type LPMIXERCONTROLDETAILS_LISTTEXT = LPMIXERCONTROLDETAILS_LISTTEXTA;
pub type LPMIXERCONTROLDETAILS_LISTTEXTA = *mut MIXERCONTROLDETAILS_LISTTEXTA;
pub type LPMIXERCONTROLDETAILS_LISTTEXTW = *mut MIXERCONTROLDETAILS_LISTTEXTW;
pub type LPMIXERCONTROLDETAILS_SIGNED = *mut MIXERCONTROLDETAILS_SIGNED;
pub type LPMIXERCONTROLDETAILS_UNSIGNED = *mut MIXERCONTROLDETAILS_UNSIGNED;
pub type LPMIXERCONTROLW = *mut MIXERCONTROLW;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERLINE = LPMIXERLINEA;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERLINEA = *mut MIXERLINEA;
pub type LPMIXERLINECONTROLS = LPMIXERLINECONTROLSA;
pub type LPMIXERLINECONTROLSA = *mut MIXERLINECONTROLSA;
pub type LPMIXERLINECONTROLSW = *mut MIXERLINECONTROLSW;
#[cfg(feature = "mmsyscom")]
pub type LPMIXERLINEW = *mut MIXERLINEW;
pub type LPPATCHARRAY = *mut u16;
pub type LPPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
pub type LPWAVECALLBACK = Option<unsafe extern "system" fn()>;
pub type LPWAVEFORMAT = *mut WAVEFORMAT;
pub type LPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type LPWAVEHDR = *mut WAVEHDR;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEINCAPS = LPWAVEINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEINCAPS2 = LPWAVEINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEINCAPS2A = *mut WAVEINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEINCAPS2W = *mut WAVEINCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEINCAPSA = *mut WAVEINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEINCAPSW = *mut WAVEINCAPSW;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEOUTCAPS = LPWAVEOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEOUTCAPS2 = LPWAVEOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type LPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub const MEVT_COMMENT: u8 = 130;
pub const MEVT_F_CALLBACK: u32 = 1073741824;
pub const MEVT_F_LONG: u32 = 2147483648;
pub const MEVT_F_SHORT: u32 = 0;
pub const MEVT_LONGMSG: u8 = 128;
pub const MEVT_NOP: u8 = 2;
pub const MEVT_SHORTMSG: u8 = 0;
pub const MEVT_TEMPO: u8 = 1;
pub const MEVT_VERSION: u8 = 132;
pub const MHDR_DONE: u32 = 1;
pub const MHDR_INQUEUE: u32 = 4;
pub const MHDR_ISSTRM: u32 = 8;
pub const MHDR_PREPARED: u32 = 2;
#[cfg(feature = "mmsyscom")]
pub type MIDICALLBACK = Option<unsafe extern "system" fn(hdrvr: super::mmsyscom::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub const MIDICAPS_CACHE: u32 = 4;
pub const MIDICAPS_LRVOLUME: u32 = 2;
pub const MIDICAPS_STREAM: u32 = 8;
pub const MIDICAPS_VOLUME: u32 = 1;
pub const MIDIERR_BADOPENMODE: u32 = 70;
pub const MIDIERR_DONT_CONTINUE: u32 = 71;
pub const MIDIERR_INVALIDSETUP: u32 = 69;
pub const MIDIERR_LASTERROR: u32 = 71;
pub const MIDIERR_NODEVICE: u32 = 68;
pub const MIDIERR_NOMAP: u32 = 66;
pub const MIDIERR_NOTREADY: u32 = 67;
pub const MIDIERR_STILLPLAYING: u32 = 65;
pub const MIDIERR_UNPREPARED: u32 = 64;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl Default for MIDIEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIHDR {
    pub lpData: windows_sys::core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub lpNext: *mut Self,
    pub reserved: usize,
    pub dwOffset: u32,
    pub dwReserved: [usize; 8],
}
impl Default for MIDIHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
pub type MIDIINCAPS = MIDIINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type MIDIINCAPS2 = MIDIINCAPS2A;
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIDIINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIDIINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIDIINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIDIINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIDIINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub dwSupport: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIDIINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIDIINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIDIINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIDIMAPPER: u32 = 4294967295;
#[cfg(feature = "mmsyscom")]
pub type MIDIOUTCAPS = MIDIOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type MIDIOUTCAPS2 = MIDIOUTCAPS2A;
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIDIPATCHSIZE: u32 = 128;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
pub const MIDIPROP_GET: u32 = 1073741824;
pub const MIDIPROP_SET: u32 = 2147483648;
pub const MIDIPROP_TEMPO: u32 = 2;
pub const MIDIPROP_TIMEDIV: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
pub const MIDISTRM_ERROR: i32 = -2;
pub const MIDI_CACHE_ALL: u32 = 1;
pub const MIDI_CACHE_BESTFIT: u32 = 2;
pub const MIDI_CACHE_QUERY: u32 = 3;
pub const MIDI_IO_STATUS: u32 = 32;
pub const MIDI_MAPPER: u32 = 4294967295;
pub const MIDI_UNCACHE: u32 = 4;
pub const MIM_CLOSE: u32 = 962;
pub const MIM_DATA: u32 = 963;
pub const MIM_ERROR: u32 = 965;
pub const MIM_LONGDATA: u32 = 964;
pub const MIM_LONGERROR: u32 = 966;
pub const MIM_MOREDATA: u32 = 972;
pub const MIM_OPEN: u32 = 961;
#[cfg(feature = "mmsyscom")]
pub type MIXERCAPS = MIXERCAPSA;
#[cfg(feature = "mmsyscom")]
pub type MIXERCAPS2 = MIXERCAPS2A;
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIXERCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIXERCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIXERCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIXERCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIXERCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIXERCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIXERCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIXERCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIXERCONTROL = MIXERCONTROLA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLA {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [i8; 16],
    pub szName: [i8; 64],
    pub Bounds: MIXERCONTROLA_0,
    pub Metrics: MIXERCONTROLA_1,
}
impl Default for MIXERCONTROLA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLA_0 {
    pub Anonymous: MIXERCONTROLA_0_0,
    pub Anonymous2: MIXERCONTROLA_0_1,
    pub dwReserved: [u32; 6],
}
impl Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLA_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl Default for MIXERCONTROLA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub cChannels: u32,
    pub Anonymous: MIXERCONTROLDETAILS_0,
    pub cbDetails: u32,
    pub paDetails: *mut core::ffi::c_void,
}
#[cfg(feature = "windef")]
impl Default for MIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union MIXERCONTROLDETAILS_0 {
    pub hwndOwner: super::windef::HWND,
    pub cMultipleItems: u32,
}
#[cfg(feature = "windef")]
impl Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
pub type MIXERCONTROLDETAILS_LISTTEXT = MIXERCONTROLDETAILS_LISTTEXTA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [i8; 64],
}
impl Default for MIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u16; 64],
}
impl Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
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
impl Default for MIXERCONTROLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLW_0 {
    pub Anonymous: MIXERCONTROLW_0_0,
    pub Anonymous2: MIXERCONTROLW_0_1,
    pub dwReserved: [u32; 6],
}
impl Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLW_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIXERCONTROL_CONTROLF_DISABLED: u32 = 2147483648;
pub const MIXERCONTROL_CONTROLF_MULTIPLE: u32 = 2;
pub const MIXERCONTROL_CONTROLF_UNIFORM: u32 = 1;
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890;
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992;
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664;
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0;
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512;
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892;
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888;
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452;
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344;
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560;
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945;
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451;
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944;
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450;
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729;
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449;
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897;
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529;
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048;
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898;
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440;
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528;
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728;
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896;
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453;
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064;
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889;
pub const MIXERCONTROL_CT_CLASS_CUSTOM: u32 = 0;
pub const MIXERCONTROL_CT_CLASS_FADER: u32 = 1342177280;
pub const MIXERCONTROL_CT_CLASS_LIST: u32 = 1879048192;
pub const MIXERCONTROL_CT_CLASS_MASK: u32 = 4026531840;
pub const MIXERCONTROL_CT_CLASS_METER: u32 = 268435456;
pub const MIXERCONTROL_CT_CLASS_NUMBER: u32 = 805306368;
pub const MIXERCONTROL_CT_CLASS_SLIDER: u32 = 1073741824;
pub const MIXERCONTROL_CT_CLASS_SWITCH: u32 = 536870912;
pub const MIXERCONTROL_CT_CLASS_TIME: u32 = 1610612736;
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: u32 = 16777216;
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: u32 = 0;
pub const MIXERCONTROL_CT_SC_METER_POLLED: u32 = 0;
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: u32 = 0;
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: u32 = 16777216;
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: u32 = 0;
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: u32 = 16777216;
pub const MIXERCONTROL_CT_SUBCLASS_MASK: u32 = 251658240;
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: u32 = 65536;
pub const MIXERCONTROL_CT_UNITS_CUSTOM: u32 = 0;
pub const MIXERCONTROL_CT_UNITS_DECIBELS: u32 = 262144;
pub const MIXERCONTROL_CT_UNITS_MASK: u32 = 16711680;
pub const MIXERCONTROL_CT_UNITS_PERCENT: u32 = 327680;
pub const MIXERCONTROL_CT_UNITS_SIGNED: u32 = 131072;
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: u32 = 196608;
#[cfg(feature = "mmsyscom")]
pub type MIXERLINE = MIXERLINEA;
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIXERLINEA {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: u32,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [i8; 16],
    pub szName: [i8; 64],
    pub Target: MIXERLINEA_0,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIXERLINEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIXERLINEA_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
}
#[cfg(feature = "mmsyscom")]
impl Default for MIXERLINEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIXERLINECONTROLS = MIXERLINECONTROLSA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERLINECONTROLSA {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSA_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: LPMIXERCONTROLA,
}
impl Default for MIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERLINECONTROLSA_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl Default for MIXERLINECONTROLSA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERLINECONTROLSW {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSW_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: LPMIXERCONTROLW,
}
impl Default for MIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERLINECONTROLSW_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl Default for MIXERLINECONTROLSW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIXERLINEW {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: u32,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Target: MIXERLINEW_0,
}
#[cfg(feature = "mmsyscom")]
impl Default for MIXERLINEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct MIXERLINEW_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
}
#[cfg(feature = "mmsyscom")]
impl Default for MIXERLINEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: u32 = 1;
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: u32 = 0;
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: u32 = 5;
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8;
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: u32 = 2;
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: u32 = 3;
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: u32 = 4;
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: u32 = 6;
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: u32 = 0;
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: u32 = 8;
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: u32 = 7;
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: u32 = 4106;
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: u32 = 4105;
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: u32 = 4101;
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: u32 = 4097;
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: u32 = 4096;
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106;
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: u32 = 4098;
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: u32 = 4099;
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: u32 = 4103;
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: u32 = 4100;
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: u32 = 4102;
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: u32 = 4096;
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: u32 = 4104;
pub const MIXERLINE_LINEF_ACTIVE: u32 = 1;
pub const MIXERLINE_LINEF_DISCONNECTED: u32 = 32768;
pub const MIXERLINE_LINEF_SOURCE: u32 = 2147483648;
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5;
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4;
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3;
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0;
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2;
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1;
pub const MIXERR_INVALCONTROL: u32 = 1025;
pub const MIXERR_INVALLINE: u32 = 1024;
pub const MIXERR_INVALVALUE: u32 = 1026;
pub const MIXERR_LASTERROR: u32 = 1026;
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: u32 = 1;
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: u32 = 15;
pub const MIXER_GETCONTROLDETAILSF_VALUE: u32 = 0;
pub const MIXER_GETLINECONTROLSF_ALL: u32 = 0;
pub const MIXER_GETLINECONTROLSF_ONEBYID: u32 = 1;
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: u32 = 2;
pub const MIXER_GETLINECONTROLSF_QUERYMASK: u32 = 15;
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: u32 = 3;
pub const MIXER_GETLINEINFOF_DESTINATION: u32 = 0;
pub const MIXER_GETLINEINFOF_LINEID: u32 = 2;
pub const MIXER_GETLINEINFOF_QUERYMASK: u32 = 15;
pub const MIXER_GETLINEINFOF_SOURCE: u32 = 1;
pub const MIXER_GETLINEINFOF_TARGETTYPE: u32 = 4;
pub const MIXER_LONG_NAME_CHARS: u32 = 64;
pub const MIXER_OBJECTF_AUX: u32 = 1342177280;
pub const MIXER_OBJECTF_HANDLE: u32 = 2147483648;
pub const MIXER_OBJECTF_HMIDIIN: i32 = -1073741824;
pub const MIXER_OBJECTF_HMIDIOUT: i32 = -1342177280;
pub const MIXER_OBJECTF_HMIXER: i32 = -2147483648;
pub const MIXER_OBJECTF_HWAVEIN: i32 = -1610612736;
pub const MIXER_OBJECTF_HWAVEOUT: i32 = -1879048192;
pub const MIXER_OBJECTF_MIDIIN: u32 = 1073741824;
pub const MIXER_OBJECTF_MIDIOUT: u32 = 805306368;
pub const MIXER_OBJECTF_MIXER: u32 = 0;
pub const MIXER_OBJECTF_WAVEIN: u32 = 536870912;
pub const MIXER_OBJECTF_WAVEOUT: u32 = 268435456;
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: u32 = 1;
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: u32 = 15;
pub const MIXER_SETCONTROLDETAILSF_VALUE: u32 = 0;
pub const MIXER_SHORT_NAME_CHARS: u32 = 16;
pub const MOD_FMSYNTH: u32 = 4;
pub const MOD_MAPPER: u32 = 5;
pub const MOD_MIDIPORT: u32 = 1;
pub const MOD_SQSYNTH: u32 = 3;
pub const MOD_SWSYNTH: u32 = 7;
pub const MOD_SYNTH: u32 = 2;
pub const MOD_WAVETABLE: u32 = 6;
pub const MOM_CLOSE: u32 = 968;
pub const MOM_DONE: u32 = 969;
pub const MOM_OPEN: u32 = 967;
pub const MOM_POSITIONCB: u32 = 970;
#[cfg(feature = "mmsyscom")]
pub type NPAUXCAPS = NPAUXCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPAUXCAPS2 = NPAUXCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPAUXCAPS2A = *mut AUXCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPAUXCAPS2W = *mut AUXCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type NPAUXCAPSA = *mut AUXCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPAUXCAPSW = *mut AUXCAPSW;
pub type NPMIDIHDR = *mut MIDIHDR;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIINCAPS = NPMIDIINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIINCAPS2 = NPMIDIINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIINCAPS2A = *mut MIDIINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIINCAPS2W = *mut MIDIINCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIINCAPSA = *mut MIDIINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIINCAPSW = *mut MIDIINCAPSW;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIOUTCAPS = NPMIDIOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIOUTCAPS2 = NPMIDIOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
pub type NPPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
pub type NPWAVEFORMAT = *mut WAVEFORMAT;
pub type NPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type NPWAVEHDR = *mut WAVEHDR;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEINCAPS = NPWAVEINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEINCAPS2 = NPWAVEINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEINCAPS2A = *mut WAVEINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEINCAPS2W = *mut WAVEINCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEINCAPSA = *mut WAVEINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEINCAPSW = *mut WAVEINCAPSW;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEOUTCAPS = NPWAVEOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEOUTCAPS2 = NPWAVEOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type NPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub type PATCHARRAY = [u16; 128];
#[cfg(feature = "mmsyscom")]
pub type PAUXCAPS = PAUXCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PAUXCAPS2 = PAUXCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PAUXCAPS2A = *mut AUXCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PAUXCAPS2W = *mut AUXCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type PAUXCAPSA = *mut AUXCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PAUXCAPSW = *mut AUXCAPSW;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
pub type PMIDIHDR = *mut MIDIHDR;
#[cfg(feature = "mmsyscom")]
pub type PMIDIINCAPS = PMIDIINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PMIDIINCAPS2 = PMIDIINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PMIDIINCAPS2A = *mut MIDIINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PMIDIINCAPS2W = *mut MIDIINCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type PMIDIINCAPSA = *mut MIDIINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PMIDIINCAPSW = *mut MIDIINCAPSW;
#[cfg(feature = "mmsyscom")]
pub type PMIDIOUTCAPS = PMIDIOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PMIDIOUTCAPS2 = PMIDIOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type PMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
#[cfg(feature = "mmsyscom")]
pub type PMIXERCAPS = PMIXERCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PMIXERCAPS2 = PMIXERCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PMIXERCAPS2A = *mut MIXERCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PMIXERCAPS2W = *mut MIXERCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type PMIXERCAPSA = *mut MIXERCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PMIXERCAPSW = *mut MIXERCAPSW;
pub type PMIXERCONTROL = PMIXERCONTROLA;
pub type PMIXERCONTROLA = *mut MIXERCONTROLA;
#[cfg(feature = "windef")]
pub type PMIXERCONTROLDETAILS = *mut MIXERCONTROLDETAILS;
pub type PMIXERCONTROLDETAILS_BOOLEAN = *mut MIXERCONTROLDETAILS_BOOLEAN;
pub type PMIXERCONTROLDETAILS_LISTTEXT = PMIXERCONTROLDETAILS_LISTTEXTA;
pub type PMIXERCONTROLDETAILS_LISTTEXTA = *mut MIXERCONTROLDETAILS_LISTTEXTA;
pub type PMIXERCONTROLDETAILS_LISTTEXTW = *mut MIXERCONTROLDETAILS_LISTTEXTW;
pub type PMIXERCONTROLDETAILS_SIGNED = *mut MIXERCONTROLDETAILS_SIGNED;
pub type PMIXERCONTROLDETAILS_UNSIGNED = *mut MIXERCONTROLDETAILS_UNSIGNED;
pub type PMIXERCONTROLW = *mut MIXERCONTROLW;
#[cfg(feature = "mmsyscom")]
pub type PMIXERLINE = PMIXERLINEA;
#[cfg(feature = "mmsyscom")]
pub type PMIXERLINEA = *mut MIXERLINEA;
pub type PMIXERLINECONTROLS = PMIXERLINECONTROLSA;
pub type PMIXERLINECONTROLSA = *mut MIXERLINECONTROLSA;
pub type PMIXERLINECONTROLSW = *mut MIXERLINECONTROLSW;
#[cfg(feature = "mmsyscom")]
pub type PMIXERLINEW = *mut MIXERLINEW;
pub type PPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
pub type PWAVEFORMAT = *mut WAVEFORMAT;
pub type PWAVEFORMATEX = *mut WAVEFORMATEX;
pub type PWAVEHDR = *mut WAVEHDR;
#[cfg(feature = "mmsyscom")]
pub type PWAVEINCAPS = PWAVEINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PWAVEINCAPS2 = PWAVEINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PWAVEINCAPS2A = *mut WAVEINCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PWAVEINCAPS2W = *mut WAVEINCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type PWAVEINCAPSA = *mut WAVEINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PWAVEINCAPSW = *mut WAVEINCAPSW;
#[cfg(feature = "mmsyscom")]
pub type PWAVEOUTCAPS = PWAVEOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PWAVEOUTCAPS2 = PWAVEOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
#[cfg(feature = "mmsyscom")]
pub type PWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
#[cfg(feature = "mmsyscom")]
pub type PWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type PWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
#[cfg(feature = "mmsyscom")]
pub type WAVECALLBACK = Option<unsafe extern "system" fn(hdrvr: super::mmsyscom::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub const WAVECAPS_LRVOLUME: u32 = 8;
pub const WAVECAPS_PITCH: u32 = 1;
pub const WAVECAPS_PLAYBACKRATE: u32 = 2;
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32;
pub const WAVECAPS_SYNC: u32 = 16;
pub const WAVECAPS_VOLUME: u32 = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEHDR {
    pub lpData: windows_sys::core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwLoops: u32,
    pub lpNext: *mut Self,
    pub reserved: usize,
}
impl Default for WAVEHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
pub type WAVEINCAPS = WAVEINCAPSA;
#[cfg(feature = "mmsyscom")]
pub type WAVEINCAPS2 = WAVEINCAPS2A;
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct WAVEINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for WAVEINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct WAVEINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for WAVEINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct WAVEINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
#[cfg(feature = "mmsyscom")]
impl Default for WAVEINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct WAVEINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
#[cfg(feature = "mmsyscom")]
impl Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
pub type WAVEOUTCAPS = WAVEOUTCAPSA;
#[cfg(feature = "mmsyscom")]
pub type WAVEOUTCAPS2 = WAVEOUTCAPS2A;
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for WAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
}
#[cfg(feature = "mmsyscom")]
impl Default for WAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [i8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for WAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmsyscom")]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: super::mmsyscom::MMVERSION,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "mmsyscom")]
impl Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WAVERR_BADFORMAT: u32 = 32;
pub const WAVERR_LASTERROR: u32 = 35;
pub const WAVERR_STILLPLAYING: u32 = 33;
pub const WAVERR_SYNC: u32 = 35;
pub const WAVERR_UNPREPARED: u32 = 34;
pub const WAVE_ALLOWSYNC: u32 = 2;
pub const WAVE_FORMAT_1M08: u32 = 1;
pub const WAVE_FORMAT_1M16: u32 = 4;
pub const WAVE_FORMAT_1S08: u32 = 2;
pub const WAVE_FORMAT_1S16: u32 = 8;
pub const WAVE_FORMAT_2M08: u32 = 16;
pub const WAVE_FORMAT_2M16: u32 = 64;
pub const WAVE_FORMAT_2S08: u32 = 32;
pub const WAVE_FORMAT_2S16: u32 = 128;
pub const WAVE_FORMAT_44M08: u32 = 256;
pub const WAVE_FORMAT_44M16: u32 = 1024;
pub const WAVE_FORMAT_44S08: u32 = 512;
pub const WAVE_FORMAT_44S16: u32 = 2048;
pub const WAVE_FORMAT_48M08: u32 = 4096;
pub const WAVE_FORMAT_48M16: u32 = 16384;
pub const WAVE_FORMAT_48S08: u32 = 8192;
pub const WAVE_FORMAT_48S16: u32 = 32768;
pub const WAVE_FORMAT_4M08: u32 = 256;
pub const WAVE_FORMAT_4M16: u32 = 1024;
pub const WAVE_FORMAT_4S08: u32 = 512;
pub const WAVE_FORMAT_4S16: u32 = 2048;
pub const WAVE_FORMAT_96M08: u32 = 65536;
pub const WAVE_FORMAT_96M16: u32 = 262144;
pub const WAVE_FORMAT_96S08: u32 = 131072;
pub const WAVE_FORMAT_96S16: u32 = 524288;
pub const WAVE_FORMAT_DIRECT: u32 = 8;
pub const WAVE_FORMAT_DIRECT_QUERY: u32 = 9;
pub const WAVE_FORMAT_PCM: u32 = 1;
pub const WAVE_FORMAT_QUERY: u32 = 1;
pub const WAVE_INVALIDFORMAT: u32 = 0;
pub const WAVE_MAPPED: u32 = 4;
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: u32 = 16;
pub const WAVE_MAPPER: u32 = 4294967295;
pub const WHDR_BEGINLOOP: u32 = 4;
pub const WHDR_DONE: u32 = 1;
pub const WHDR_ENDLOOP: u32 = 8;
pub const WHDR_INQUEUE: u32 = 16;
pub const WHDR_PREPARED: u32 = 2;
pub const WIM_CLOSE: u32 = 959;
pub const WIM_DATA: u32 = 960;
pub const WIM_OPEN: u32 = 958;
pub const WOM_CLOSE: u32 = 956;
pub const WOM_DONE: u32 = 957;
pub const WOM_OPEN: u32 = 955;
