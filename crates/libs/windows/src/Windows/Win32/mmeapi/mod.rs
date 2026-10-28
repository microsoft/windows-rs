#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn auxGetDevCapsA(udeviceid: super::UINT_PTR, pac: LPAUXCAPSA, cbac: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxGetDevCapsA(udeviceid : super::UINT_PTR, pac : LPAUXCAPSA, cbac : super::UINT) -> super::MMRESULT);
    unsafe { auxGetDevCapsA(udeviceid, pac as _, cbac) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn auxGetDevCapsW(udeviceid: super::UINT_PTR, pac: LPAUXCAPSW, cbac: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxGetDevCapsW(udeviceid : super::UINT_PTR, pac : LPAUXCAPSW, cbac : super::UINT) -> super::MMRESULT);
    unsafe { auxGetDevCapsW(udeviceid, pac as _, cbac) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn auxGetNumDevs() -> super::UINT {
    windows_core::link!("winmm.dll" "system" fn auxGetNumDevs() -> super::UINT);
    unsafe { auxGetNumDevs() }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn auxGetVolume(udeviceid: super::UINT, pdwvolume: super::LPDWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxGetVolume(udeviceid : super::UINT, pdwvolume : super::LPDWORD) -> super::MMRESULT);
    unsafe { auxGetVolume(udeviceid, pdwvolume as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn auxOutMessage(udeviceid: super::UINT, umsg: super::UINT, dw1: Option<super::DWORD_PTR>, dw2: Option<super::DWORD_PTR>) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxOutMessage(udeviceid : super::UINT, umsg : super::UINT, dw1 : super::DWORD_PTR, dw2 : super::DWORD_PTR) -> super::MMRESULT);
    unsafe { auxOutMessage(udeviceid, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn auxSetVolume(udeviceid: super::UINT, dwvolume: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxSetVolume(udeviceid : super::UINT, dwvolume : super::DWORD) -> super::MMRESULT);
    unsafe { auxSetVolume(udeviceid, dwvolume) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: Option<super::LPVOID>) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiConnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : super::LPVOID) -> super::MMRESULT);
    unsafe { midiConnect(hmi, hmo, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn midiDisconnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: Option<super::LPVOID>) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiDisconnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : super::LPVOID) -> super::MMRESULT);
    unsafe { midiDisconnect(hmi, hmo, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInAddBuffer(hmi: HMIDIIN, pmh: LPMIDIHDR, cbmh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInAddBuffer(hmi : HMIDIIN, pmh : LPMIDIHDR, cbmh : super::UINT) -> super::MMRESULT);
    unsafe { midiInAddBuffer(hmi, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInClose(hmi: HMIDIIN) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInClose(hmi : HMIDIIN) -> super::MMRESULT);
    unsafe { midiInClose(hmi) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInGetDevCapsA(udeviceid: super::UINT_PTR, pmic: LPMIDIINCAPSA, cbmic: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetDevCapsA(udeviceid : super::UINT_PTR, pmic : LPMIDIINCAPSA, cbmic : super::UINT) -> super::MMRESULT);
    unsafe { midiInGetDevCapsA(udeviceid, pmic as _, cbmic) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn midiInGetDevCapsW(udeviceid: super::UINT_PTR, pmic: LPMIDIINCAPSW, cbmic: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetDevCapsW(udeviceid : super::UINT_PTR, pmic : LPMIDIINCAPSW, cbmic : super::UINT) -> super::MMRESULT);
    unsafe { midiInGetDevCapsW(udeviceid, pmic as _, cbmic) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInGetErrorTextA(mmrerror: super::MMRESULT, psztext: windows_core::PSTR, cchtext: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetErrorTextA(mmrerror : super::MMRESULT, psztext : windows_core::PSTR, cchtext : super::UINT) -> super::MMRESULT);
    unsafe { midiInGetErrorTextA(mmrerror, psztext, cchtext) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInGetErrorTextW(mmrerror: super::MMRESULT, psztext: windows_core::PWSTR, cchtext: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetErrorTextW(mmrerror : super::MMRESULT, psztext : windows_core::PWSTR, cchtext : super::UINT) -> super::MMRESULT);
    unsafe { midiInGetErrorTextW(mmrerror, psztext, cchtext) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInGetID(hmi: HMIDIIN, pudeviceid: super::LPUINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetID(hmi : HMIDIIN, pudeviceid : super::LPUINT) -> super::MMRESULT);
    unsafe { midiInGetID(hmi, pudeviceid as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn midiInGetNumDevs() -> super::UINT {
    windows_core::link!("winmm.dll" "system" fn midiInGetNumDevs() -> super::UINT);
    unsafe { midiInGetNumDevs() }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInMessage(hmi: Option<HMIDIIN>, umsg: super::UINT, dw1: Option<super::DWORD_PTR>, dw2: Option<super::DWORD_PTR>) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInMessage(hmi : HMIDIIN, umsg : super::UINT, dw1 : super::DWORD_PTR, dw2 : super::DWORD_PTR) -> super::MMRESULT);
    unsafe { midiInMessage(hmi.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInOpen(phmi: LPHMIDIIN, udeviceid: super::UINT, dwcallback: Option<super::DWORD_PTR>, dwinstance: Option<super::DWORD_PTR>, fdwopen: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInOpen(phmi : LPHMIDIIN, udeviceid : super::UINT, dwcallback : super::DWORD_PTR, dwinstance : super::DWORD_PTR, fdwopen : super::DWORD) -> super::MMRESULT);
    unsafe { midiInOpen(phmi as _, udeviceid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInPrepareHeader(hmi: HMIDIIN, pmh: LPMIDIHDR, cbmh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInPrepareHeader(hmi : HMIDIIN, pmh : LPMIDIHDR, cbmh : super::UINT) -> super::MMRESULT);
    unsafe { midiInPrepareHeader(hmi, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInReset(hmi: HMIDIIN) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInReset(hmi : HMIDIIN) -> super::MMRESULT);
    unsafe { midiInReset(hmi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInStart(hmi: HMIDIIN) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInStart(hmi : HMIDIIN) -> super::MMRESULT);
    unsafe { midiInStart(hmi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInStop(hmi: HMIDIIN) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInStop(hmi : HMIDIIN) -> super::MMRESULT);
    unsafe { midiInStop(hmi) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: LPMIDIHDR, cbmh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInUnprepareHeader(hmi : HMIDIIN, pmh : LPMIDIHDR, cbmh : super::UINT) -> super::MMRESULT);
    unsafe { midiInUnprepareHeader(hmi, pmh as _, cbmh) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: super::UINT, pwkya: &[u16; 128], fucache: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutCacheDrumPatches(hmo : HMIDIOUT, upatch : super::UINT, pwkya : super::LPWORD, fucache : super::UINT) -> super::MMRESULT);
    unsafe { midiOutCacheDrumPatches(hmo, upatch, core::mem::transmute(pwkya.as_ptr()), fucache) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutCachePatches(hmo: HMIDIOUT, ubank: super::UINT, pwpa: &[u16; 128], fucache: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutCachePatches(hmo : HMIDIOUT, ubank : super::UINT, pwpa : super::LPWORD, fucache : super::UINT) -> super::MMRESULT);
    unsafe { midiOutCachePatches(hmo, ubank, core::mem::transmute(pwpa.as_ptr()), fucache) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutClose(hmo: HMIDIOUT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutClose(hmo : HMIDIOUT) -> super::MMRESULT);
    unsafe { midiOutClose(hmo) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutGetDevCapsA(udeviceid: super::UINT_PTR, pmoc: LPMIDIOUTCAPSA, cbmoc: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetDevCapsA(udeviceid : super::UINT_PTR, pmoc : LPMIDIOUTCAPSA, cbmoc : super::UINT) -> super::MMRESULT);
    unsafe { midiOutGetDevCapsA(udeviceid, pmoc as _, cbmoc) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn midiOutGetDevCapsW(udeviceid: super::UINT_PTR, pmoc: LPMIDIOUTCAPSW, cbmoc: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetDevCapsW(udeviceid : super::UINT_PTR, pmoc : LPMIDIOUTCAPSW, cbmoc : super::UINT) -> super::MMRESULT);
    unsafe { midiOutGetDevCapsW(udeviceid, pmoc as _, cbmoc) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutGetErrorTextA(mmrerror: super::MMRESULT, psztext: windows_core::PSTR, cchtext: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetErrorTextA(mmrerror : super::MMRESULT, psztext : windows_core::PSTR, cchtext : super::UINT) -> super::MMRESULT);
    unsafe { midiOutGetErrorTextA(mmrerror, psztext, cchtext) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutGetErrorTextW(mmrerror: super::MMRESULT, psztext: windows_core::PWSTR, cchtext: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetErrorTextW(mmrerror : super::MMRESULT, psztext : windows_core::PWSTR, cchtext : super::UINT) -> super::MMRESULT);
    unsafe { midiOutGetErrorTextW(mmrerror, psztext, cchtext) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: super::LPUINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetID(hmo : HMIDIOUT, pudeviceid : super::LPUINT) -> super::MMRESULT);
    unsafe { midiOutGetID(hmo, pudeviceid as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn midiOutGetNumDevs() -> super::UINT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetNumDevs() -> super::UINT);
    unsafe { midiOutGetNumDevs() }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn midiOutGetVolume(hmo: Option<HMIDIOUT>, pdwvolume: super::LPDWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetVolume(hmo : HMIDIOUT, pdwvolume : super::LPDWORD) -> super::MMRESULT);
    unsafe { midiOutGetVolume(hmo.unwrap_or(core::mem::zeroed()) as _, pdwvolume as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutLongMsg(hmo: HMIDIOUT, pmh: LPMIDIHDR, cbmh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutLongMsg(hmo : HMIDIOUT, pmh : LPMIDIHDR, cbmh : super::UINT) -> super::MMRESULT);
    unsafe { midiOutLongMsg(hmo, pmh, cbmh) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutMessage(hmo: Option<HMIDIOUT>, umsg: super::UINT, dw1: Option<super::DWORD_PTR>, dw2: Option<super::DWORD_PTR>) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutMessage(hmo : HMIDIOUT, umsg : super::UINT, dw1 : super::DWORD_PTR, dw2 : super::DWORD_PTR) -> super::MMRESULT);
    unsafe { midiOutMessage(hmo.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutOpen(phmo: LPHMIDIOUT, udeviceid: super::UINT, dwcallback: Option<super::DWORD_PTR>, dwinstance: Option<super::DWORD_PTR>, fdwopen: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutOpen(phmo : LPHMIDIOUT, udeviceid : super::UINT, dwcallback : super::DWORD_PTR, dwinstance : super::DWORD_PTR, fdwopen : super::DWORD) -> super::MMRESULT);
    unsafe { midiOutOpen(phmo as _, udeviceid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: LPMIDIHDR, cbmh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutPrepareHeader(hmo : HMIDIOUT, pmh : LPMIDIHDR, cbmh : super::UINT) -> super::MMRESULT);
    unsafe { midiOutPrepareHeader(hmo, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutReset(hmo: HMIDIOUT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutReset(hmo : HMIDIOUT) -> super::MMRESULT);
    unsafe { midiOutReset(hmo) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutSetVolume(hmo: Option<HMIDIOUT>, dwvolume: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutSetVolume(hmo : HMIDIOUT, dwvolume : super::DWORD) -> super::MMRESULT);
    unsafe { midiOutSetVolume(hmo.unwrap_or(core::mem::zeroed()) as _, dwvolume) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutShortMsg(hmo : HMIDIOUT, dwmsg : super::DWORD) -> super::MMRESULT);
    unsafe { midiOutShortMsg(hmo, dwmsg) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: LPMIDIHDR, cbmh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutUnprepareHeader(hmo : HMIDIOUT, pmh : LPMIDIHDR, cbmh : super::UINT) -> super::MMRESULT);
    unsafe { midiOutUnprepareHeader(hmo, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamClose(hms: HMIDISTRM) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamClose(hms : HMIDISTRM) -> super::MMRESULT);
    unsafe { midiStreamClose(hms) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiStreamOpen(phms: LPHMIDISTRM, pudeviceid: &mut [u32], dwcallback: Option<super::DWORD_PTR>, dwinstance: Option<super::DWORD_PTR>, fdwopen: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamOpen(phms : LPHMIDISTRM, pudeviceid : super::LPUINT, cmidi : super::DWORD, dwcallback : super::DWORD_PTR, dwinstance : super::DWORD_PTR, fdwopen : super::DWORD) -> super::MMRESULT);
    unsafe { midiStreamOpen(phms as _, core::mem::transmute(pudeviceid.as_mut_ptr()), pudeviceid.len().try_into().unwrap(), dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiStreamOut(hms: HMIDISTRM, pmh: LPMIDIHDR, cbmh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamOut(hms : HMIDISTRM, pmh : LPMIDIHDR, cbmh : super::UINT) -> super::MMRESULT);
    unsafe { midiStreamOut(hms, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamPause(hms: HMIDISTRM) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamPause(hms : HMIDISTRM) -> super::MMRESULT);
    unsafe { midiStreamPause(hms) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiStreamPosition(hms: HMIDISTRM, lpmmt: super::LPMMTIME, cbmmt: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamPosition(hms : HMIDISTRM, lpmmt : super::LPMMTIME, cbmmt : super::UINT) -> super::MMRESULT);
    unsafe { midiStreamPosition(hms, lpmmt as _, cbmmt) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn midiStreamProperty(hms: HMIDISTRM, lppropdata: super::LPBYTE, dwproperty: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamProperty(hms : HMIDISTRM, lppropdata : super::LPBYTE, dwproperty : super::DWORD) -> super::MMRESULT);
    unsafe { midiStreamProperty(hms, lppropdata as _, dwproperty) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamRestart(hms: HMIDISTRM) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamRestart(hms : HMIDISTRM) -> super::MMRESULT);
    unsafe { midiStreamRestart(hms) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamStop(hms: HMIDISTRM) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamStop(hms : HMIDISTRM) -> super::MMRESULT);
    unsafe { midiStreamStop(hms) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerClose(hmx: HMIXER) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerClose(hmx : HMIXER) -> super::MMRESULT);
    unsafe { mixerClose(hmx) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn mixerGetControlDetailsA(hmxobj: Option<HMIXEROBJ>, pmxcd: LPMIXERCONTROLDETAILS, fdwdetails: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetControlDetailsA(hmxobj : HMIXEROBJ, pmxcd : LPMIXERCONTROLDETAILS, fdwdetails : super::DWORD) -> super::MMRESULT);
    unsafe { mixerGetControlDetailsA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd as _, fdwdetails) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn mixerGetControlDetailsW(hmxobj: Option<HMIXEROBJ>, pmxcd: LPMIXERCONTROLDETAILS, fdwdetails: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetControlDetailsW(hmxobj : HMIXEROBJ, pmxcd : LPMIXERCONTROLDETAILS, fdwdetails : super::DWORD) -> super::MMRESULT);
    unsafe { mixerGetControlDetailsW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd as _, fdwdetails) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mixerGetDevCapsA(umxid: super::UINT_PTR, pmxcaps: LPMIXERCAPSA, cbmxcaps: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetDevCapsA(umxid : super::UINT_PTR, pmxcaps : LPMIXERCAPSA, cbmxcaps : super::UINT) -> super::MMRESULT);
    unsafe { mixerGetDevCapsA(umxid, pmxcaps as _, cbmxcaps) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn mixerGetDevCapsW(umxid: super::UINT_PTR, pmxcaps: LPMIXERCAPSW, cbmxcaps: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetDevCapsW(umxid : super::UINT_PTR, pmxcaps : LPMIXERCAPSW, cbmxcaps : super::UINT) -> super::MMRESULT);
    unsafe { mixerGetDevCapsW(umxid, pmxcaps as _, cbmxcaps) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mixerGetID(hmxobj: Option<HMIXEROBJ>, pumxid: *mut super::UINT, fdwid: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetID(hmxobj : HMIXEROBJ, pumxid : *mut super::UINT, fdwid : super::DWORD) -> super::MMRESULT);
    unsafe { mixerGetID(hmxobj.unwrap_or(core::mem::zeroed()) as _, pumxid as _, fdwid) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mixerGetLineControlsA(hmxobj: Option<HMIXEROBJ>, pmxlc: LPMIXERLINECONTROLSA, fdwcontrols: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineControlsA(hmxobj : HMIXEROBJ, pmxlc : LPMIXERLINECONTROLSA, fdwcontrols : super::DWORD) -> super::MMRESULT);
    unsafe { mixerGetLineControlsA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxlc as _, fdwcontrols) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn mixerGetLineControlsW(hmxobj: Option<HMIXEROBJ>, pmxlc: LPMIXERLINECONTROLSW, fdwcontrols: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineControlsW(hmxobj : HMIXEROBJ, pmxlc : LPMIXERLINECONTROLSW, fdwcontrols : super::DWORD) -> super::MMRESULT);
    unsafe { mixerGetLineControlsW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxlc as _, fdwcontrols) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mixerGetLineInfoA(hmxobj: Option<HMIXEROBJ>, pmxl: LPMIXERLINEA, fdwinfo: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineInfoA(hmxobj : HMIXEROBJ, pmxl : LPMIXERLINEA, fdwinfo : super::DWORD) -> super::MMRESULT);
    unsafe { mixerGetLineInfoA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxl as _, fdwinfo) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn mixerGetLineInfoW(hmxobj: Option<HMIXEROBJ>, pmxl: LPMIXERLINEW, fdwinfo: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineInfoW(hmxobj : HMIXEROBJ, pmxl : LPMIXERLINEW, fdwinfo : super::DWORD) -> super::MMRESULT);
    unsafe { mixerGetLineInfoW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxl as _, fdwinfo) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn mixerGetNumDevs() -> super::UINT {
    windows_core::link!("winmm.dll" "system" fn mixerGetNumDevs() -> super::UINT);
    unsafe { mixerGetNumDevs() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn mixerMessage(hmx: Option<HMIXER>, umsg: super::UINT, dwparam1: Option<super::DWORD_PTR>, dwparam2: Option<super::DWORD_PTR>) -> super::DWORD {
    windows_core::link!("winmm.dll" "system" fn mixerMessage(hmx : HMIXER, umsg : super::UINT, dwparam1 : super::DWORD_PTR, dwparam2 : super::DWORD_PTR) -> super::DWORD);
    unsafe { mixerMessage(hmx.unwrap_or(core::mem::zeroed()) as _, umsg, dwparam1.unwrap_or(core::mem::zeroed()) as _, dwparam2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mixerOpen(phmx: Option<LPHMIXER>, umxid: super::UINT, dwcallback: Option<super::DWORD_PTR>, dwinstance: Option<super::DWORD_PTR>, fdwopen: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerOpen(phmx : LPHMIXER, umxid : super::UINT, dwcallback : super::DWORD_PTR, dwinstance : super::DWORD_PTR, fdwopen : super::DWORD) -> super::MMRESULT);
    unsafe { mixerOpen(phmx.unwrap_or(core::mem::zeroed()) as _, umxid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn mixerSetControlDetails(hmxobj: Option<HMIXEROBJ>, pmxcd: LPMIXERCONTROLDETAILS, fdwdetails: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerSetControlDetails(hmxobj : HMIXEROBJ, pmxcd : LPMIXERCONTROLDETAILS, fdwdetails : super::DWORD) -> super::MMRESULT);
    unsafe { mixerSetControlDetails(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd, fdwdetails) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInAddBuffer(hwi: HWAVEIN, pwh: LPWAVEHDR, cbwh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInAddBuffer(hwi : HWAVEIN, pwh : LPWAVEHDR, cbwh : super::UINT) -> super::MMRESULT);
    unsafe { waveInAddBuffer(hwi, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInClose(hwi: HWAVEIN) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInClose(hwi : HWAVEIN) -> super::MMRESULT);
    unsafe { waveInClose(hwi) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInGetDevCapsA(udeviceid: super::UINT_PTR, pwic: LPWAVEINCAPSA, cbwic: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetDevCapsA(udeviceid : super::UINT_PTR, pwic : LPWAVEINCAPSA, cbwic : super::UINT) -> super::MMRESULT);
    unsafe { waveInGetDevCapsA(udeviceid, pwic as _, cbwic) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn waveInGetDevCapsW(udeviceid: super::UINT_PTR, pwic: LPWAVEINCAPSW, cbwic: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetDevCapsW(udeviceid : super::UINT_PTR, pwic : LPWAVEINCAPSW, cbwic : super::UINT) -> super::MMRESULT);
    unsafe { waveInGetDevCapsW(udeviceid, pwic as _, cbwic) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInGetErrorTextA(mmrerror: super::MMRESULT, psztext: windows_core::PSTR, cchtext: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetErrorTextA(mmrerror : super::MMRESULT, psztext : windows_core::PSTR, cchtext : super::UINT) -> super::MMRESULT);
    unsafe { waveInGetErrorTextA(mmrerror, psztext, cchtext) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInGetErrorTextW(mmrerror: super::MMRESULT, psztext: windows_core::PWSTR, cchtext: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetErrorTextW(mmrerror : super::MMRESULT, psztext : windows_core::PWSTR, cchtext : super::UINT) -> super::MMRESULT);
    unsafe { waveInGetErrorTextW(mmrerror, psztext, cchtext) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInGetID(hwi: HWAVEIN, pudeviceid: super::LPUINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetID(hwi : HWAVEIN, pudeviceid : super::LPUINT) -> super::MMRESULT);
    unsafe { waveInGetID(hwi, pudeviceid) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn waveInGetNumDevs() -> super::UINT {
    windows_core::link!("winmm.dll" "system" fn waveInGetNumDevs() -> super::UINT);
    unsafe { waveInGetNumDevs() }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInGetPosition(hwi: HWAVEIN, pmmt: super::LPMMTIME, cbmmt: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetPosition(hwi : HWAVEIN, pmmt : super::LPMMTIME, cbmmt : super::UINT) -> super::MMRESULT);
    unsafe { waveInGetPosition(hwi, pmmt as _, cbmmt) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInMessage(hwi: Option<HWAVEIN>, umsg: super::UINT, dw1: Option<super::DWORD_PTR>, dw2: Option<super::DWORD_PTR>) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInMessage(hwi : HWAVEIN, umsg : super::UINT, dw1 : super::DWORD_PTR, dw2 : super::DWORD_PTR) -> super::MMRESULT);
    unsafe { waveInMessage(hwi.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInOpen(phwi: Option<LPHWAVEIN>, udeviceid: super::UINT, pwfx: LPCWAVEFORMATEX, dwcallback: Option<super::DWORD_PTR>, dwinstance: Option<super::DWORD_PTR>, fdwopen: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInOpen(phwi : LPHWAVEIN, udeviceid : super::UINT, pwfx : LPCWAVEFORMATEX, dwcallback : super::DWORD_PTR, dwinstance : super::DWORD_PTR, fdwopen : super::DWORD) -> super::MMRESULT);
    unsafe { waveInOpen(phwi.unwrap_or(core::mem::zeroed()) as _, udeviceid, pwfx, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInPrepareHeader(hwi: HWAVEIN, pwh: LPWAVEHDR, cbwh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInPrepareHeader(hwi : HWAVEIN, pwh : LPWAVEHDR, cbwh : super::UINT) -> super::MMRESULT);
    unsafe { waveInPrepareHeader(hwi, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInReset(hwi: HWAVEIN) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInReset(hwi : HWAVEIN) -> super::MMRESULT);
    unsafe { waveInReset(hwi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInStart(hwi: HWAVEIN) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInStart(hwi : HWAVEIN) -> super::MMRESULT);
    unsafe { waveInStart(hwi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInStop(hwi: HWAVEIN) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInStop(hwi : HWAVEIN) -> super::MMRESULT);
    unsafe { waveInStop(hwi) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: LPWAVEHDR, cbwh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInUnprepareHeader(hwi : HWAVEIN, pwh : LPWAVEHDR, cbwh : super::UINT) -> super::MMRESULT);
    unsafe { waveInUnprepareHeader(hwi, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutBreakLoop(hwo: HWAVEOUT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutBreakLoop(hwo : HWAVEOUT) -> super::MMRESULT);
    unsafe { waveOutBreakLoop(hwo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutClose(hwo: HWAVEOUT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutClose(hwo : HWAVEOUT) -> super::MMRESULT);
    unsafe { waveOutClose(hwo) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutGetDevCapsA(udeviceid: super::UINT_PTR, pwoc: LPWAVEOUTCAPSA, cbwoc: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetDevCapsA(udeviceid : super::UINT_PTR, pwoc : LPWAVEOUTCAPSA, cbwoc : super::UINT) -> super::MMRESULT);
    unsafe { waveOutGetDevCapsA(udeviceid, pwoc as _, cbwoc) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn waveOutGetDevCapsW(udeviceid: super::UINT_PTR, pwoc: LPWAVEOUTCAPSW, cbwoc: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetDevCapsW(udeviceid : super::UINT_PTR, pwoc : LPWAVEOUTCAPSW, cbwoc : super::UINT) -> super::MMRESULT);
    unsafe { waveOutGetDevCapsW(udeviceid, pwoc as _, cbwoc) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutGetErrorTextA(mmrerror: super::MMRESULT, psztext: windows_core::PSTR, cchtext: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetErrorTextA(mmrerror : super::MMRESULT, psztext : windows_core::PSTR, cchtext : super::UINT) -> super::MMRESULT);
    unsafe { waveOutGetErrorTextA(mmrerror, psztext, cchtext) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutGetErrorTextW(mmrerror: super::MMRESULT, psztext: windows_core::PWSTR, cchtext: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetErrorTextW(mmrerror : super::MMRESULT, psztext : windows_core::PWSTR, cchtext : super::UINT) -> super::MMRESULT);
    unsafe { waveOutGetErrorTextW(mmrerror, psztext, cchtext) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: super::LPUINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetID(hwo : HWAVEOUT, pudeviceid : super::LPUINT) -> super::MMRESULT);
    unsafe { waveOutGetID(hwo, pudeviceid as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn waveOutGetNumDevs() -> super::UINT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetNumDevs() -> super::UINT);
    unsafe { waveOutGetNumDevs() }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: super::LPDWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPitch(hwo : HWAVEOUT, pdwpitch : super::LPDWORD) -> super::MMRESULT);
    unsafe { waveOutGetPitch(hwo, pdwpitch as _) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: super::LPDWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPlaybackRate(hwo : HWAVEOUT, pdwrate : super::LPDWORD) -> super::MMRESULT);
    unsafe { waveOutGetPlaybackRate(hwo, pdwrate as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: super::LPMMTIME, cbmmt: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPosition(hwo : HWAVEOUT, pmmt : super::LPMMTIME, cbmmt : super::UINT) -> super::MMRESULT);
    unsafe { waveOutGetPosition(hwo, pmmt as _, cbmmt) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn waveOutGetVolume(hwo: Option<HWAVEOUT>, pdwvolume: super::LPDWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetVolume(hwo : HWAVEOUT, pdwvolume : super::LPDWORD) -> super::MMRESULT);
    unsafe { waveOutGetVolume(hwo.unwrap_or(core::mem::zeroed()) as _, pdwvolume as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutMessage(hwo: Option<HWAVEOUT>, umsg: super::UINT, dw1: super::DWORD_PTR, dw2: super::DWORD_PTR) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutMessage(hwo : HWAVEOUT, umsg : super::UINT, dw1 : super::DWORD_PTR, dw2 : super::DWORD_PTR) -> super::MMRESULT);
    unsafe { waveOutMessage(hwo.unwrap_or(core::mem::zeroed()) as _, umsg, dw1, dw2) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutOpen(phwo: Option<LPHWAVEOUT>, udeviceid: super::UINT, pwfx: LPCWAVEFORMATEX, dwcallback: Option<super::DWORD_PTR>, dwinstance: Option<super::DWORD_PTR>, fdwopen: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutOpen(phwo : LPHWAVEOUT, udeviceid : super::UINT, pwfx : LPCWAVEFORMATEX, dwcallback : super::DWORD_PTR, dwinstance : super::DWORD_PTR, fdwopen : super::DWORD) -> super::MMRESULT);
    unsafe { waveOutOpen(phwo.unwrap_or(core::mem::zeroed()) as _, udeviceid, pwfx, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutPause(hwo: HWAVEOUT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutPause(hwo : HWAVEOUT) -> super::MMRESULT);
    unsafe { waveOutPause(hwo) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutPrepareHeader(hwo : HWAVEOUT, pwh : LPWAVEHDR, cbwh : super::UINT) -> super::MMRESULT);
    unsafe { waveOutPrepareHeader(hwo, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutReset(hwo: HWAVEOUT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutReset(hwo : HWAVEOUT) -> super::MMRESULT);
    unsafe { waveOutReset(hwo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutRestart(hwo: HWAVEOUT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutRestart(hwo : HWAVEOUT) -> super::MMRESULT);
    unsafe { waveOutRestart(hwo) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutSetPitch(hwo : HWAVEOUT, dwpitch : super::DWORD) -> super::MMRESULT);
    unsafe { waveOutSetPitch(hwo, dwpitch) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutSetPlaybackRate(hwo : HWAVEOUT, dwrate : super::DWORD) -> super::MMRESULT);
    unsafe { waveOutSetPlaybackRate(hwo, dwrate) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutSetVolume(hwo: Option<HWAVEOUT>, dwvolume: super::DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutSetVolume(hwo : HWAVEOUT, dwvolume : super::DWORD) -> super::MMRESULT);
    unsafe { waveOutSetVolume(hwo.unwrap_or(core::mem::zeroed()) as _, dwvolume) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutUnprepareHeader(hwo : HWAVEOUT, pwh : LPWAVEHDR, cbwh : super::UINT) -> super::MMRESULT);
    unsafe { waveOutUnprepareHeader(hwo, pwh as _, cbwh) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn waveOutWrite(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutWrite(hwo : HWAVEOUT, pwh : LPWAVEHDR, cbwh : super::UINT) -> super::MMRESULT);
    unsafe { waveOutWrite(hwo, pwh as _, cbwh) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type AUXCAPS = AUXCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type AUXCAPS2 = AUXCAPS2A;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct AUXCAPS2A {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub wTechnology: super::WORD,
    pub wReserved1: super::WORD,
    pub dwSupport: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for AUXCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AUXCAPS2W {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub wTechnology: super::WORD,
    pub wReserved1: super::WORD,
    pub dwSupport: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for AUXCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct AUXCAPSA {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub wTechnology: super::WORD,
    pub wReserved1: super::WORD,
    pub dwSupport: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for AUXCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AUXCAPSW {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub wTechnology: super::WORD,
    pub wReserved1: super::WORD,
    pub dwSupport: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUXCAPS_AUXIN: i32 = 2;
pub const AUXCAPS_CDAUDIO: i32 = 1;
pub const AUXCAPS_LRVOLUME: i32 = 2;
pub const AUXCAPS_VOLUME: i32 = 1;
#[cfg(feature = "shared")]
pub const AUX_MAPPER: super::UINT = 4294967295;
pub type HMIDI = *mut HMIDI__;
pub type HMIDIIN = *mut HMIDIIN__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HMIDIIN__ {
    pub unused: i32,
}
pub type HMIDIOUT = *mut HMIDIOUT__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HMIDIOUT__ {
    pub unused: i32,
}
pub type HMIDISTRM = *mut HMIDISTRM__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HMIDISTRM__ {
    pub unused: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HMIDI__ {
    pub unused: i32,
}
pub type HMIXER = *mut HMIXER__;
pub type HMIXEROBJ = *mut HMIXEROBJ__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HMIXEROBJ__ {
    pub unused: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HMIXER__ {
    pub unused: i32,
}
pub type HWAVE = *mut HWAVE__;
pub type HWAVEIN = *mut HWAVEIN__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HWAVEIN__ {
    pub unused: i32,
}
pub type HWAVEOUT = *mut HWAVEOUT__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HWAVEOUT__ {
    pub unused: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HWAVE__ {
    pub unused: i32,
}
pub type KEYARRAY = [u16; 128];
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPAUXCAPS = *mut AUXCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPAUXCAPS2 = *mut AUXCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPAUXCAPS2A = *mut AUXCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPAUXCAPS2W = *mut AUXCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPAUXCAPSA = *mut AUXCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPAUXCAPSW = *mut AUXCAPSW;
#[cfg(feature = "shared")]
pub type LPCWAVEFORMATEX = *const WAVEFORMATEX;
pub type LPHMIDI = *mut *mut HMIDI__;
pub type LPHMIDIIN = *mut *mut HMIDIIN__;
pub type LPHMIDIOUT = *mut *mut HMIDIOUT__;
pub type LPHMIDISTRM = *mut *mut HMIDISTRM__;
pub type LPHMIXER = *mut *mut HMIXER__;
pub type LPHMIXEROBJ = *mut *mut HMIXEROBJ__;
pub type LPHWAVEIN = *mut *mut HWAVEIN__;
pub type LPHWAVEOUT = *mut *mut HWAVEOUT__;
pub type LPKEYARRAY = *mut u16;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDICALLBACK = Option<unsafe extern "system" fn(hdrvr: super::HDRVR, umsg: super::UINT, dwuser: super::DWORD_PTR, dw1: super::DWORD_PTR, dw2: super::DWORD_PTR)>;
#[cfg(feature = "shared")]
pub type LPMIDIHDR = *mut MIDIHDR;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDIINCAPS = *mut MIDIINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDIINCAPS2 = *mut MIDIINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDIINCAPS2A = *mut MIDIINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPMIDIINCAPS2W = *mut MIDIINCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDIINCAPSA = *mut MIDIINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPMIDIINCAPSW = *mut MIDIINCAPSW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDIOUTCAPS = *mut MIDIOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDIOUTCAPS2 = *mut MIDIOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
#[cfg(feature = "shared")]
pub type LPMIDIPROPTEMPO = *mut MIDIPROPTEMPO;
#[cfg(feature = "shared")]
pub type LPMIDIPROPTIMEDIV = *mut MIDIPROPTIMEDIV;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIXERCAPS = *mut MIXERCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIXERCAPS2 = *mut MIXERCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIXERCAPS2A = *mut MIXERCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPMIXERCAPS2W = *mut MIXERCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIXERCAPSA = *mut MIXERCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPMIXERCAPSW = *mut MIXERCAPSW;
#[cfg(feature = "shared")]
pub type LPMIXERCONTROL = *mut MIXERCONTROLA;
#[cfg(feature = "shared")]
pub type LPMIXERCONTROLA = *mut MIXERCONTROLA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPMIXERCONTROLDETAILS = *mut MIXERCONTROLDETAILS;
#[cfg(feature = "shared")]
pub type LPMIXERCONTROLDETAILS_BOOLEAN = *mut MIXERCONTROLDETAILS_BOOLEAN;
#[cfg(feature = "shared")]
pub type LPMIXERCONTROLDETAILS_LISTTEXT = *mut MIXERCONTROLDETAILS_LISTTEXTA;
#[cfg(feature = "shared")]
pub type LPMIXERCONTROLDETAILS_LISTTEXTA = *mut MIXERCONTROLDETAILS_LISTTEXTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPMIXERCONTROLDETAILS_LISTTEXTW = *mut MIXERCONTROLDETAILS_LISTTEXTW;
#[cfg(feature = "shared")]
pub type LPMIXERCONTROLDETAILS_SIGNED = *mut MIXERCONTROLDETAILS_SIGNED;
#[cfg(feature = "shared")]
pub type LPMIXERCONTROLDETAILS_UNSIGNED = *mut MIXERCONTROLDETAILS_UNSIGNED;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPMIXERCONTROLW = *mut MIXERCONTROLW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIXERLINE = *mut MIXERLINEA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPMIXERLINEA = *mut MIXERLINEA;
#[cfg(feature = "shared")]
pub type LPMIXERLINECONTROLS = *mut MIXERLINECONTROLSA;
#[cfg(feature = "shared")]
pub type LPMIXERLINECONTROLSA = *mut MIXERLINECONTROLSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPMIXERLINECONTROLSW = *mut MIXERLINECONTROLSW;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPMIXERLINEW = *mut MIXERLINEW;
pub type LPPATCHARRAY = *mut u16;
#[cfg(feature = "shared")]
pub type LPPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVECALLBACK = Option<unsafe extern "system" fn(hdrvr: super::HDRVR, umsg: super::UINT, dwuser: super::DWORD_PTR, dw1: super::DWORD_PTR, dw2: super::DWORD_PTR)>;
#[cfg(feature = "shared")]
pub type LPWAVEFORMAT = *mut WAVEFORMAT;
#[cfg(feature = "shared")]
pub type LPWAVEFORMATEX = *mut WAVEFORMATEX;
#[cfg(feature = "shared")]
pub type LPWAVEHDR = *mut WAVEHDR;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVEINCAPS = *mut WAVEINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVEINCAPS2 = *mut WAVEINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVEINCAPS2A = *mut WAVEINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPWAVEINCAPS2W = *mut WAVEINCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVEINCAPSA = *mut WAVEINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPWAVEINCAPSW = *mut WAVEINCAPSW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVEOUTCAPS = *mut WAVEOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVEOUTCAPS2 = *mut WAVEOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type LPWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type LPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
#[cfg(feature = "shared")]
pub const MEVT_COMMENT: super::BYTE = 130;
pub const MEVT_F_CALLBACK: i32 = 1073741824;
pub const MEVT_F_LONG: u32 = 2147483648;
pub const MEVT_F_SHORT: i32 = 0;
#[cfg(feature = "shared")]
pub const MEVT_LONGMSG: super::BYTE = 128;
#[cfg(feature = "shared")]
pub const MEVT_NOP: super::BYTE = 2;
#[cfg(feature = "shared")]
pub const MEVT_SHORTMSG: super::BYTE = 0;
#[cfg(feature = "shared")]
pub const MEVT_TEMPO: super::BYTE = 1;
#[cfg(feature = "shared")]
pub const MEVT_VERSION: super::BYTE = 132;
pub const MHDR_DONE: i32 = 1;
pub const MHDR_INQUEUE: i32 = 4;
pub const MHDR_ISSTRM: i32 = 8;
pub const MHDR_PREPARED: i32 = 2;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type MIDICALLBACK = Option<unsafe extern "system" fn(hdrvr: super::HDRVR, umsg: super::UINT, dwuser: super::DWORD_PTR, dw1: super::DWORD_PTR, dw2: super::DWORD_PTR)>;
pub const MIDICAPS_CACHE: i32 = 4;
pub const MIDICAPS_LRVOLUME: i32 = 2;
pub const MIDICAPS_STREAM: i32 = 8;
pub const MIDICAPS_VOLUME: i32 = 1;
pub const MIDIERR_BADOPENMODE: i32 = 70;
pub const MIDIERR_DONT_CONTINUE: i32 = 71;
pub const MIDIERR_INVALIDSETUP: i32 = 69;
pub const MIDIERR_LASTERROR: i32 = 71;
pub const MIDIERR_NODEVICE: i32 = 68;
pub const MIDIERR_NOMAP: i32 = 66;
pub const MIDIERR_NOTREADY: i32 = 67;
pub const MIDIERR_STILLPLAYING: i32 = 65;
pub const MIDIERR_UNPREPARED: i32 = 64;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIDIEVENT {
    pub dwDeltaTime: super::DWORD,
    pub dwStreamID: super::DWORD,
    pub dwEvent: super::DWORD,
    pub dwParms: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for MIDIEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIDIHDR {
    pub lpData: windows_core::PSTR,
    pub dwBufferLength: super::DWORD,
    pub dwBytesRecorded: super::DWORD,
    pub dwUser: super::DWORD_PTR,
    pub dwFlags: super::DWORD,
    pub lpNext: *mut Self,
    pub reserved: super::DWORD_PTR,
    pub dwOffset: super::DWORD,
    pub dwReserved: [super::DWORD_PTR; 8],
}
#[cfg(feature = "shared")]
impl Default for MIDIHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type MIDIINCAPS = MIDIINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type MIDIINCAPS2 = MIDIINCAPS2A;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIDIINCAPS2A {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub dwSupport: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for MIDIINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIDIINCAPS2W {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub dwSupport: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for MIDIINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIDIINCAPSA {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub dwSupport: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for MIDIINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIDIINCAPSW {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub dwSupport: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for MIDIINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const MIDIMAPPER: super::UINT = 4294967295;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type MIDIOUTCAPS = MIDIOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type MIDIOUTCAPS2 = MIDIOUTCAPS2A;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPS2A {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub wTechnology: super::WORD,
    pub wVoices: super::WORD,
    pub wNotes: super::WORD,
    pub wChannelMask: super::WORD,
    pub dwSupport: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for MIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPS2W {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub wTechnology: super::WORD,
    pub wVoices: super::WORD,
    pub wNotes: super::WORD,
    pub wChannelMask: super::WORD,
    pub dwSupport: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for MIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPSA {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub wTechnology: super::WORD,
    pub wVoices: super::WORD,
    pub wNotes: super::WORD,
    pub wChannelMask: super::WORD,
    pub dwSupport: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for MIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPSW {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub wTechnology: super::WORD,
    pub wVoices: super::WORD,
    pub wNotes: super::WORD,
    pub wChannelMask: super::WORD,
    pub dwSupport: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIDIPATCHSIZE: i32 = 128;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIDIPROPTEMPO {
    pub cbStruct: super::DWORD,
    pub dwTempo: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: super::DWORD,
    pub dwTimeDiv: super::DWORD,
}
pub const MIDIPROP_GET: i32 = 1073741824;
pub const MIDIPROP_SET: u32 = 2147483648;
pub const MIDIPROP_TEMPO: i32 = 2;
pub const MIDIPROP_TIMEDIV: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: super::DWORD,
    pub dwMid: super::DWORD,
    pub dwOEMVersion: super::DWORD,
}
pub const MIDISTRM_ERROR: i32 = -2;
pub const MIDI_CACHE_ALL: i32 = 1;
pub const MIDI_CACHE_BESTFIT: i32 = 2;
pub const MIDI_CACHE_QUERY: i32 = 3;
pub const MIDI_IO_STATUS: i32 = 32;
#[cfg(feature = "shared")]
pub const MIDI_MAPPER: super::UINT = 4294967295;
pub const MIDI_UNCACHE: i32 = 4;
pub const MIM_CLOSE: i32 = 962;
pub const MIM_DATA: i32 = 963;
pub const MIM_ERROR: i32 = 965;
pub const MIM_LONGDATA: i32 = 964;
pub const MIM_LONGERROR: i32 = 966;
pub const MIM_MOREDATA: i32 = 972;
pub const MIM_OPEN: i32 = 961;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type MIXERCAPS = MIXERCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type MIXERCAPS2 = MIXERCAPS2A;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIXERCAPS2A {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub fdwSupport: super::DWORD,
    pub cDestinations: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for MIXERCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIXERCAPS2W {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub fdwSupport: super::DWORD,
    pub cDestinations: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for MIXERCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIXERCAPSA {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub fdwSupport: super::DWORD,
    pub cDestinations: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for MIXERCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIXERCAPSW {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub fdwSupport: super::DWORD,
    pub cDestinations: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for MIXERCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type MIXERCONTROL = MIXERCONTROLA;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLA {
    pub cbStruct: super::DWORD,
    pub dwControlID: super::DWORD,
    pub dwControlType: super::DWORD,
    pub fdwControl: super::DWORD,
    pub cMultipleItems: super::DWORD,
    pub szShortName: [super::CHAR; 16],
    pub szName: [super::CHAR; 64],
    pub Bounds: MIXERCONTROLA_0,
    pub Metrics: MIXERCONTROLA_1,
}
#[cfg(feature = "shared")]
impl Default for MIXERCONTROLA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MIXERCONTROLA_0 {
    pub Anonymous: MIXERCONTROLA_0_0,
    pub Anonymous2: MIXERCONTROLA_0_1,
    pub dwReserved: [super::DWORD; 6],
}
#[cfg(feature = "shared")]
impl Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: super::LONG,
    pub lMaximum: super::LONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: super::DWORD,
    pub dwMaximum: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MIXERCONTROLA_1 {
    pub cSteps: super::DWORD,
    pub cbCustomData: super::DWORD,
    pub dwReserved: [super::DWORD; 6],
}
#[cfg(feature = "shared")]
impl Default for MIXERCONTROLA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS {
    pub cbStruct: super::DWORD,
    pub dwControlID: super::DWORD,
    pub cChannels: super::DWORD,
    pub Anonymous: MIXERCONTROLDETAILS_0,
    pub cbDetails: super::DWORD,
    pub paDetails: super::LPVOID,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl Default for MIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLDETAILS_0 {
    pub hwndOwner: super::HWND,
    pub cMultipleItems: super::DWORD,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: super::LONG,
}
#[cfg(feature = "shared")]
pub type MIXERCONTROLDETAILS_LISTTEXT = MIXERCONTROLDETAILS_LISTTEXTA;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: super::DWORD,
    pub dwParam2: super::DWORD,
    pub szName: [super::CHAR; 64],
}
#[cfg(feature = "shared")]
impl Default for MIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: super::DWORD,
    pub dwParam2: super::DWORD,
    pub szName: [super::WCHAR; 64],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: super::LONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLW {
    pub cbStruct: super::DWORD,
    pub dwControlID: super::DWORD,
    pub dwControlType: super::DWORD,
    pub fdwControl: super::DWORD,
    pub cMultipleItems: super::DWORD,
    pub szShortName: [super::WCHAR; 16],
    pub szName: [super::WCHAR; 64],
    pub Bounds: MIXERCONTROLW_0,
    pub Metrics: MIXERCONTROLW_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIXERCONTROLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLW_0 {
    pub Anonymous: MIXERCONTROLW_0_0,
    pub Anonymous2: MIXERCONTROLW_0_1,
    pub dwReserved: [super::DWORD; 6],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: super::LONG,
    pub lMaximum: super::LONG,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: super::DWORD,
    pub dwMaximum: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLW_1 {
    pub cSteps: super::DWORD,
    pub cbCustomData: super::DWORD,
    pub dwReserved: [super::DWORD; 6],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIXERCONTROL_CONTROLF_DISABLED: u32 = 2147483648;
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2;
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1;
pub const MIXERCONTROL_CONTROLTYPE_BASS: i32 = 1342373890;
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: i32 = 536945271;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: i32 = 536936448;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: i32 = 268500992;
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: i32 = 553713664;
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: i32 = 0;
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: i32 = 805568512;
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: i32 = 1342373892;
pub const MIXERCONTROL_CONTROLTYPE_FADER: i32 = 1342373888;
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: i32 = 536936452;
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: i32 = 1610809344;
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: i32 = 1627586560;
pub const MIXERCONTROL_CONTROLTYPE_MIXER: i32 = 1895890945;
pub const MIXERCONTROL_CONTROLTYPE_MONO: i32 = 536936451;
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: i32 = 1895890944;
pub const MIXERCONTROL_CONTROLTYPE_MUTE: i32 = 536936450;
pub const MIXERCONTROL_CONTROLTYPE_MUX: i32 = 1879113729;
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: i32 = 536936449;
pub const MIXERCONTROL_CONTROLTYPE_PAN: i32 = 1073872897;
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: i32 = 268566529;
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: i32 = 805634048;
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: i32 = 1073872898;
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: i32 = 805437440;
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: i32 = 268566528;
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: i32 = 1879113728;
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: i32 = 1073872896;
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: i32 = 536936453;
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: i32 = 1342373891;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: i32 = 805502976;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: i32 = 268632064;
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: i32 = 1342373889;
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0;
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280;
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192;
pub const MIXERCONTROL_CT_CLASS_MASK: u32 = 4026531840;
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456;
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368;
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824;
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912;
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736;
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216;
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0;
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0;
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0;
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216;
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0;
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216;
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240;
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536;
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0;
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144;
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680;
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680;
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072;
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type MIXERLINE = MIXERLINEA;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIXERLINEA {
    pub cbStruct: super::DWORD,
    pub dwDestination: super::DWORD,
    pub dwSource: super::DWORD,
    pub dwLineID: super::DWORD,
    pub fdwLine: super::DWORD,
    pub dwUser: super::DWORD_PTR,
    pub dwComponentType: super::DWORD,
    pub cChannels: super::DWORD,
    pub cConnections: super::DWORD,
    pub cControls: super::DWORD,
    pub szShortName: [super::CHAR; 16],
    pub szName: [super::CHAR; 64],
    pub Target: MIXERLINEA_0,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for MIXERLINEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIXERLINEA_0 {
    pub dwType: super::DWORD,
    pub dwDeviceID: super::DWORD,
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for MIXERLINEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type MIXERLINECONTROLS = MIXERLINECONTROLSA;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIXERLINECONTROLSA {
    pub cbStruct: super::DWORD,
    pub dwLineID: super::DWORD,
    pub Anonymous: MIXERLINECONTROLSA_0,
    pub cControls: super::DWORD,
    pub cbmxctrl: super::DWORD,
    pub pamxctrl: LPMIXERCONTROLA,
}
#[cfg(feature = "shared")]
impl Default for MIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MIXERLINECONTROLSA_0 {
    pub dwControlID: super::DWORD,
    pub dwControlType: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for MIXERLINECONTROLSA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIXERLINECONTROLSW {
    pub cbStruct: super::DWORD,
    pub dwLineID: super::DWORD,
    pub Anonymous: MIXERLINECONTROLSW_0,
    pub cControls: super::DWORD,
    pub cbmxctrl: super::DWORD,
    pub pamxctrl: LPMIXERCONTROLW,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MIXERLINECONTROLSW_0 {
    pub dwControlID: super::DWORD,
    pub dwControlType: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIXERLINECONTROLSW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIXERLINEW {
    pub cbStruct: super::DWORD,
    pub dwDestination: super::DWORD,
    pub dwSource: super::DWORD,
    pub dwLineID: super::DWORD,
    pub fdwLine: super::DWORD,
    pub dwUser: super::DWORD_PTR,
    pub dwComponentType: super::DWORD,
    pub cChannels: super::DWORD,
    pub cConnections: super::DWORD,
    pub cControls: super::DWORD,
    pub szShortName: [super::WCHAR; 16],
    pub szName: [super::WCHAR; 64],
    pub Target: MIXERLINEW_0,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for MIXERLINEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIXERLINEW_0 {
    pub dwType: super::DWORD,
    pub dwDeviceID: super::DWORD,
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for MIXERLINEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: i32 = 1;
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0;
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: i32 = 5;
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: i32 = 8;
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: i32 = 2;
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: i32 = 3;
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: i32 = 4;
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: i32 = 6;
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: i32 = 0;
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: i32 = 8;
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: i32 = 7;
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: i32 = 4106;
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: i32 = 4105;
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: i32 = 4101;
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: i32 = 4097;
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096;
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: i32 = 4106;
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: i32 = 4098;
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: i32 = 4099;
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: i32 = 4103;
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: i32 = 4100;
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: i32 = 4102;
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: i32 = 4096;
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: i32 = 4104;
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1;
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768;
pub const MIXERLINE_LINEF_SOURCE: u32 = 2147483648;
pub const MIXERLINE_TARGETTYPE_AUX: i32 = 5;
pub const MIXERLINE_TARGETTYPE_MIDIIN: i32 = 4;
pub const MIXERLINE_TARGETTYPE_MIDIOUT: i32 = 3;
pub const MIXERLINE_TARGETTYPE_UNDEFINED: i32 = 0;
pub const MIXERLINE_TARGETTYPE_WAVEIN: i32 = 2;
pub const MIXERLINE_TARGETTYPE_WAVEOUT: i32 = 1;
pub const MIXERR_INVALCONTROL: i32 = 1025;
pub const MIXERR_INVALLINE: i32 = 1024;
pub const MIXERR_INVALVALUE: i32 = 1026;
pub const MIXERR_LASTERROR: i32 = 1026;
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1;
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15;
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0;
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0;
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1;
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2;
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15;
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3;
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0;
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2;
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15;
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1;
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4;
pub const MIXER_LONG_NAME_CHARS: i32 = 64;
pub const MIXER_OBJECTF_AUX: i32 = 1342177280;
pub const MIXER_OBJECTF_HANDLE: u32 = 2147483648;
pub const MIXER_OBJECTF_HMIDIIN: u32 = 3221225472;
pub const MIXER_OBJECTF_HMIDIOUT: u32 = 2952790016;
pub const MIXER_OBJECTF_HMIXER: u32 = 2147483648;
pub const MIXER_OBJECTF_HWAVEIN: u32 = 2684354560;
pub const MIXER_OBJECTF_HWAVEOUT: u32 = 2415919104;
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824;
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368;
pub const MIXER_OBJECTF_MIXER: i32 = 0;
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912;
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456;
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1;
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15;
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0;
pub const MIXER_SHORT_NAME_CHARS: i32 = 16;
pub const MOD_FMSYNTH: i32 = 4;
pub const MOD_MAPPER: i32 = 5;
pub const MOD_MIDIPORT: i32 = 1;
pub const MOD_SQSYNTH: i32 = 3;
pub const MOD_SWSYNTH: i32 = 7;
pub const MOD_SYNTH: i32 = 2;
pub const MOD_WAVETABLE: i32 = 6;
pub const MOM_CLOSE: i32 = 968;
pub const MOM_DONE: i32 = 969;
pub const MOM_OPEN: i32 = 967;
pub const MOM_POSITIONCB: i32 = 970;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPAUXCAPS = *mut AUXCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPAUXCAPS2 = *mut AUXCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPAUXCAPS2A = *mut AUXCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPAUXCAPS2W = *mut AUXCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPAUXCAPSA = *mut AUXCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPAUXCAPSW = *mut AUXCAPSW;
#[cfg(feature = "shared")]
pub type NPMIDIHDR = *mut MIDIHDR;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPMIDIINCAPS = *mut MIDIINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPMIDIINCAPS2 = *mut MIDIINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPMIDIINCAPS2A = *mut MIDIINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPMIDIINCAPS2W = *mut MIDIINCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPMIDIINCAPSA = *mut MIDIINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPMIDIINCAPSW = *mut MIDIINCAPSW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPMIDIOUTCAPS = *mut MIDIOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPMIDIOUTCAPS2 = *mut MIDIOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
#[cfg(feature = "shared")]
pub type NPPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
#[cfg(feature = "shared")]
pub type NPWAVEFORMAT = *mut WAVEFORMAT;
#[cfg(feature = "shared")]
pub type NPWAVEFORMATEX = *mut WAVEFORMATEX;
#[cfg(feature = "shared")]
pub type NPWAVEHDR = *mut WAVEHDR;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPWAVEINCAPS = *mut WAVEINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPWAVEINCAPS2 = *mut WAVEINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPWAVEINCAPS2A = *mut WAVEINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPWAVEINCAPS2W = *mut WAVEINCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPWAVEINCAPSA = *mut WAVEINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPWAVEINCAPSW = *mut WAVEINCAPSW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPWAVEOUTCAPS = *mut WAVEOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPWAVEOUTCAPS2 = *mut WAVEOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type NPWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type NPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub type PATCHARRAY = [u16; 128];
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PAUXCAPS = *mut AUXCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PAUXCAPS2 = *mut AUXCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PAUXCAPS2A = *mut AUXCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PAUXCAPS2W = *mut AUXCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PAUXCAPSA = *mut AUXCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PAUXCAPSW = *mut AUXCAPSW;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: super::WORD,
}
#[cfg(feature = "shared")]
pub type PMIDIHDR = *mut MIDIHDR;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIDIINCAPS = *mut MIDIINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIDIINCAPS2 = *mut MIDIINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIDIINCAPS2A = *mut MIDIINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PMIDIINCAPS2W = *mut MIDIINCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIDIINCAPSA = *mut MIDIINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PMIDIINCAPSW = *mut MIDIINCAPSW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIDIOUTCAPS = *mut MIDIOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIDIOUTCAPS2 = *mut MIDIOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIXERCAPS = *mut MIXERCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIXERCAPS2 = *mut MIXERCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIXERCAPS2A = *mut MIXERCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PMIXERCAPS2W = *mut MIXERCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIXERCAPSA = *mut MIXERCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PMIXERCAPSW = *mut MIXERCAPSW;
#[cfg(feature = "shared")]
pub type PMIXERCONTROL = *mut MIXERCONTROLA;
#[cfg(feature = "shared")]
pub type PMIXERCONTROLA = *mut MIXERCONTROLA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PMIXERCONTROLDETAILS = *mut MIXERCONTROLDETAILS;
#[cfg(feature = "shared")]
pub type PMIXERCONTROLDETAILS_BOOLEAN = *mut MIXERCONTROLDETAILS_BOOLEAN;
#[cfg(feature = "shared")]
pub type PMIXERCONTROLDETAILS_LISTTEXT = *mut MIXERCONTROLDETAILS_LISTTEXTA;
#[cfg(feature = "shared")]
pub type PMIXERCONTROLDETAILS_LISTTEXTA = *mut MIXERCONTROLDETAILS_LISTTEXTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIXERCONTROLDETAILS_LISTTEXTW = *mut MIXERCONTROLDETAILS_LISTTEXTW;
#[cfg(feature = "shared")]
pub type PMIXERCONTROLDETAILS_SIGNED = *mut MIXERCONTROLDETAILS_SIGNED;
#[cfg(feature = "shared")]
pub type PMIXERCONTROLDETAILS_UNSIGNED = *mut MIXERCONTROLDETAILS_UNSIGNED;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIXERCONTROLW = *mut MIXERCONTROLW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIXERLINE = *mut MIXERLINEA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PMIXERLINEA = *mut MIXERLINEA;
#[cfg(feature = "shared")]
pub type PMIXERLINECONTROLS = *mut MIXERLINECONTROLSA;
#[cfg(feature = "shared")]
pub type PMIXERLINECONTROLSA = *mut MIXERLINECONTROLSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIXERLINECONTROLSW = *mut MIXERLINECONTROLSW;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PMIXERLINEW = *mut MIXERLINEW;
#[cfg(feature = "shared")]
pub type PPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
#[cfg(feature = "shared")]
pub type PWAVEFORMAT = *mut WAVEFORMAT;
#[cfg(feature = "shared")]
pub type PWAVEFORMATEX = *mut WAVEFORMATEX;
#[cfg(feature = "shared")]
pub type PWAVEHDR = *mut WAVEHDR;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PWAVEINCAPS = *mut WAVEINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PWAVEINCAPS2 = *mut WAVEINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PWAVEINCAPS2A = *mut WAVEINCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PWAVEINCAPS2W = *mut WAVEINCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PWAVEINCAPSA = *mut WAVEINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PWAVEINCAPSW = *mut WAVEINCAPSW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PWAVEOUTCAPS = *mut WAVEOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PWAVEOUTCAPS2 = *mut WAVEOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type PWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
pub type PWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type WAVECALLBACK = Option<unsafe extern "system" fn(hdrvr: super::HDRVR, umsg: super::UINT, dwuser: super::DWORD_PTR, dw1: super::DWORD_PTR, dw2: super::DWORD_PTR)>;
pub const WAVECAPS_LRVOLUME: i32 = 8;
pub const WAVECAPS_PITCH: i32 = 1;
pub const WAVECAPS_PLAYBACKRATE: i32 = 2;
pub const WAVECAPS_SAMPLEACCURATE: i32 = 32;
pub const WAVECAPS_SYNC: i32 = 16;
pub const WAVECAPS_VOLUME: i32 = 4;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WAVEFORMAT {
    pub wFormatTag: super::WORD,
    pub nChannels: super::WORD,
    pub nSamplesPerSec: super::DWORD,
    pub nAvgBytesPerSec: super::DWORD,
    pub nBlockAlign: super::WORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WAVEFORMATEX {
    pub wFormatTag: super::WORD,
    pub nChannels: super::WORD,
    pub nSamplesPerSec: super::DWORD,
    pub nAvgBytesPerSec: super::DWORD,
    pub nBlockAlign: super::WORD,
    pub wBitsPerSample: super::WORD,
    pub cbSize: super::WORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WAVEHDR {
    pub lpData: windows_core::PSTR,
    pub dwBufferLength: super::DWORD,
    pub dwBytesRecorded: super::DWORD,
    pub dwUser: super::DWORD_PTR,
    pub dwFlags: super::DWORD,
    pub dwLoops: super::DWORD,
    pub lpNext: *mut Self,
    pub reserved: super::DWORD_PTR,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type WAVEINCAPS = WAVEINCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type WAVEINCAPS2 = WAVEINCAPS2A;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct WAVEINCAPS2A {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub dwFormats: super::DWORD,
    pub wChannels: super::WORD,
    pub wReserved1: super::WORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for WAVEINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WAVEINCAPS2W {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub dwFormats: super::DWORD,
    pub wChannels: super::WORD,
    pub wReserved1: super::WORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for WAVEINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct WAVEINCAPSA {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub dwFormats: super::DWORD,
    pub wChannels: super::WORD,
    pub wReserved1: super::WORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for WAVEINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WAVEINCAPSW {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub dwFormats: super::DWORD,
    pub wChannels: super::WORD,
    pub wReserved1: super::WORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type WAVEOUTCAPS = WAVEOUTCAPSA;
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
pub type WAVEOUTCAPS2 = WAVEOUTCAPS2A;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPS2A {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub dwFormats: super::DWORD,
    pub wChannels: super::WORD,
    pub wReserved1: super::WORD,
    pub dwSupport: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for WAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPS2W {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub dwFormats: super::DWORD,
    pub wChannels: super::WORD,
    pub wReserved1: super::WORD,
    pub dwSupport: super::DWORD,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for WAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPSA {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::CHAR; 32],
    pub dwFormats: super::DWORD,
    pub wChannels: super::WORD,
    pub wReserved1: super::WORD,
    pub dwSupport: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
impl Default for WAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPSW {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vDriverVersion: super::MMVERSION,
    pub szPname: [super::WCHAR; 32],
    pub dwFormats: super::DWORD,
    pub wChannels: super::WORD,
    pub wReserved1: super::WORD,
    pub dwSupport: super::DWORD,
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
impl Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WAVERR_BADFORMAT: i32 = 32;
pub const WAVERR_LASTERROR: i32 = 35;
pub const WAVERR_STILLPLAYING: i32 = 33;
pub const WAVERR_SYNC: i32 = 35;
pub const WAVERR_UNPREPARED: i32 = 34;
pub const WAVE_ALLOWSYNC: i32 = 2;
pub const WAVE_FORMAT_1M08: i32 = 1;
pub const WAVE_FORMAT_1M16: i32 = 4;
pub const WAVE_FORMAT_1S08: i32 = 2;
pub const WAVE_FORMAT_1S16: i32 = 8;
pub const WAVE_FORMAT_2M08: i32 = 16;
pub const WAVE_FORMAT_2M16: i32 = 64;
pub const WAVE_FORMAT_2S08: i32 = 32;
pub const WAVE_FORMAT_2S16: i32 = 128;
pub const WAVE_FORMAT_44M08: i32 = 256;
pub const WAVE_FORMAT_44M16: i32 = 1024;
pub const WAVE_FORMAT_44S08: i32 = 512;
pub const WAVE_FORMAT_44S16: i32 = 2048;
pub const WAVE_FORMAT_48M08: i32 = 4096;
pub const WAVE_FORMAT_48M16: i32 = 16384;
pub const WAVE_FORMAT_48S08: i32 = 8192;
pub const WAVE_FORMAT_48S16: i32 = 32768;
pub const WAVE_FORMAT_4M08: i32 = 256;
pub const WAVE_FORMAT_4M16: i32 = 1024;
pub const WAVE_FORMAT_4S08: i32 = 512;
pub const WAVE_FORMAT_4S16: i32 = 2048;
pub const WAVE_FORMAT_96M08: i32 = 65536;
pub const WAVE_FORMAT_96M16: i32 = 262144;
pub const WAVE_FORMAT_96S08: i32 = 131072;
pub const WAVE_FORMAT_96S16: i32 = 524288;
pub const WAVE_FORMAT_DIRECT: i32 = 8;
pub const WAVE_FORMAT_DIRECT_QUERY: i32 = 9;
pub const WAVE_FORMAT_PCM: i32 = 1;
pub const WAVE_FORMAT_QUERY: i32 = 1;
pub const WAVE_INVALIDFORMAT: i32 = 0;
pub const WAVE_MAPPED: i32 = 4;
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: i32 = 16;
#[cfg(feature = "shared")]
pub const WAVE_MAPPER: super::UINT = 4294967295;
pub const WHDR_BEGINLOOP: i32 = 4;
pub const WHDR_DONE: i32 = 1;
pub const WHDR_ENDLOOP: i32 = 8;
pub const WHDR_INQUEUE: i32 = 16;
pub const WHDR_PREPARED: i32 = 2;
pub const WIM_CLOSE: i32 = 959;
pub const WIM_DATA: i32 = 960;
pub const WIM_OPEN: i32 = 958;
pub const WOM_CLOSE: i32 = 956;
pub const WOM_DONE: i32 = 957;
pub const WOM_OPEN: i32 = 955;
