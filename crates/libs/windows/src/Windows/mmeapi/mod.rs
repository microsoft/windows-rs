#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxGetDevCapsA(udeviceid : usize, pac : *mut AUXCAPSA, cbac : u32) -> super::mmsyscom::MMRESULT);
    unsafe { auxGetDevCapsA(udeviceid, pac as _, cbac) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxGetDevCapsW(udeviceid : usize, pac : *mut AUXCAPSW, cbac : u32) -> super::mmsyscom::MMRESULT);
    unsafe { auxGetDevCapsW(udeviceid, pac as _, cbac) }
}
#[inline]
pub unsafe fn auxGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn auxGetNumDevs() -> u32);
    unsafe { auxGetNumDevs() }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxGetVolume(udeviceid : u32, pdwvolume : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { auxGetVolume(udeviceid, pdwvolume as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: Option<usize>, dw2: Option<usize>) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxOutMessage(udeviceid : u32, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
    unsafe { auxOutMessage(udeviceid, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn auxSetVolume(udeviceid : u32, dwvolume : u32) -> super::mmsyscom::MMRESULT);
    unsafe { auxSetVolume(udeviceid, dwvolume) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: Option<*const core::ffi::c_void>) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiConnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> super::mmsyscom::MMRESULT);
    unsafe { midiConnect(hmi, hmo, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiDisconnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: Option<*const core::ffi::c_void>) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiDisconnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> super::mmsyscom::MMRESULT);
    unsafe { midiDisconnect(hmi, hmo, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInAddBuffer(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInAddBuffer(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInAddBuffer(hmi, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInClose(hmi: HMIDIIN) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInClose(hmi : HMIDIIN) -> super::mmsyscom::MMRESULT);
    unsafe { midiInClose(hmi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetDevCapsA(udeviceid : usize, pmic : *mut MIDIINCAPSA, cbmic : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInGetDevCapsA(udeviceid, pmic as _, cbmic) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetDevCapsW(udeviceid : usize, pmic : *mut MIDIINCAPSW, cbmic : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInGetDevCapsW(udeviceid, pmic as _, cbmic) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInGetErrorTextA(mmrerror: super::mmsyscom::MMRESULT, psztext: &mut [u8]) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetErrorTextA(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_core::PSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInGetErrorTextW(mmrerror: super::mmsyscom::MMRESULT, psztext: &mut [u16]) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetErrorTextW(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_core::PWSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInGetID(hmi: HMIDIIN, pudeviceid: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInGetID(hmi : HMIDIIN, pudeviceid : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInGetID(hmi, pudeviceid as _) }
}
#[inline]
pub unsafe fn midiInGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInGetNumDevs() -> u32);
    unsafe { midiInGetNumDevs() }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInMessage(hmi: Option<HMIDIIN>, umsg: u32, dw1: Option<usize>, dw2: Option<usize>) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInMessage(hmi : HMIDIIN, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
    unsafe { midiInMessage(hmi.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInOpen(phmi : *mut HMIDIIN, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInOpen(phmi as _, udeviceid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInPrepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInPrepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInPrepareHeader(hmi, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInReset(hmi: HMIDIIN) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInReset(hmi : HMIDIIN) -> super::mmsyscom::MMRESULT);
    unsafe { midiInReset(hmi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInStart(hmi: HMIDIIN) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInStart(hmi : HMIDIIN) -> super::mmsyscom::MMRESULT);
    unsafe { midiInStart(hmi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInStop(hmi: HMIDIIN) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInStop(hmi : HMIDIIN) -> super::mmsyscom::MMRESULT);
    unsafe { midiInStop(hmi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiInUnprepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiInUnprepareHeader(hmi, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: u32, pwkya: *const u16, fucache: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutCacheDrumPatches(hmo : HMIDIOUT, upatch : u32, pwkya : *const u16, fucache : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutCacheDrumPatches(hmo, upatch, pwkya, fucache) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutCachePatches(hmo: HMIDIOUT, ubank: u32, pwpa: *const u16, fucache: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutCachePatches(hmo : HMIDIOUT, ubank : u32, pwpa : *const u16, fucache : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutCachePatches(hmo, ubank, pwpa, fucache) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutClose(hmo: HMIDIOUT) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutClose(hmo : HMIDIOUT) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutClose(hmo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetDevCapsA(udeviceid : usize, pmoc : *mut MIDIOUTCAPSA, cbmoc : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutGetDevCapsA(udeviceid, pmoc as _, cbmoc) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetDevCapsW(udeviceid : usize, pmoc : *mut MIDIOUTCAPSW, cbmoc : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutGetDevCapsW(udeviceid, pmoc as _, cbmoc) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutGetErrorTextA(mmrerror: super::mmsyscom::MMRESULT, psztext: &mut [u8]) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetErrorTextA(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_core::PSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutGetErrorTextW(mmrerror: super::mmsyscom::MMRESULT, psztext: &mut [u16]) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetErrorTextW(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_core::PWSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetID(hmo : HMIDIOUT, pudeviceid : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutGetID(hmo, pudeviceid as _) }
}
#[inline]
pub unsafe fn midiOutGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutGetNumDevs() -> u32);
    unsafe { midiOutGetNumDevs() }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutGetVolume(hmo: Option<HMIDIOUT>, pdwvolume: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutGetVolume(hmo : HMIDIOUT, pdwvolume : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutGetVolume(hmo.unwrap_or(core::mem::zeroed()) as _, pdwvolume as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutLongMsg(hmo: HMIDIOUT, pmh: *const MIDIHDR, cbmh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutLongMsg(hmo : HMIDIOUT, pmh : *const MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutLongMsg(hmo, pmh, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutMessage(hmo: Option<HMIDIOUT>, umsg: u32, dw1: Option<usize>, dw2: Option<usize>) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutMessage(hmo : HMIDIOUT, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutMessage(hmo.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutOpen(phmo : *mut HMIDIOUT, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutOpen(phmo as _, udeviceid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutPrepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutPrepareHeader(hmo, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutReset(hmo: HMIDIOUT) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutReset(hmo : HMIDIOUT) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutReset(hmo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutSetVolume(hmo: Option<HMIDIOUT>, dwvolume: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutSetVolume(hmo : HMIDIOUT, dwvolume : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutSetVolume(hmo.unwrap_or(core::mem::zeroed()) as _, dwvolume) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutShortMsg(hmo : HMIDIOUT, dwmsg : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutShortMsg(hmo, dwmsg) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiOutUnprepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiOutUnprepareHeader(hmo, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamClose(hms: HMIDISTRM) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamClose(hms : HMIDISTRM) -> super::mmsyscom::MMRESULT);
    unsafe { midiStreamClose(hms) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: &mut [u32], dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamOpen(phms : *mut HMIDISTRM, pudeviceid : *mut u32, cmidi : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiStreamOpen(phms as _, core::mem::transmute(pudeviceid.as_ptr()), pudeviceid.len().try_into().unwrap(), dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamOut(hms: HMIDISTRM, pmh: *mut MIDIHDR, cbmh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamOut(hms : HMIDISTRM, pmh : *mut MIDIHDR, cbmh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiStreamOut(hms, pmh as _, cbmh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamPause(hms: HMIDISTRM) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamPause(hms : HMIDISTRM) -> super::mmsyscom::MMRESULT);
    unsafe { midiStreamPause(hms) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamPosition(hms: HMIDISTRM, lpmmt: *mut super::mmsyscom::MMTIME, cbmmt: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamPosition(hms : HMIDISTRM, lpmmt : *mut super::mmsyscom::MMTIME, cbmmt : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiStreamPosition(hms, lpmmt as _, cbmmt) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamProperty(hms: HMIDISTRM, lppropdata: *mut u8, dwproperty: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamProperty(hms : HMIDISTRM, lppropdata : *mut u8, dwproperty : u32) -> super::mmsyscom::MMRESULT);
    unsafe { midiStreamProperty(hms, lppropdata as _, dwproperty) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamRestart(hms: HMIDISTRM) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamRestart(hms : HMIDISTRM) -> super::mmsyscom::MMRESULT);
    unsafe { midiStreamRestart(hms) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn midiStreamStop(hms: HMIDISTRM) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn midiStreamStop(hms : HMIDISTRM) -> super::mmsyscom::MMRESULT);
    unsafe { midiStreamStop(hms) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerClose(hmx: HMIXER) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerClose(hmx : HMIXER) -> super::mmsyscom::MMRESULT);
    unsafe { mixerClose(hmx) }
}
#[cfg(all(feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn mixerGetControlDetailsA(hmxobj: Option<HMIXEROBJ>, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetControlDetailsA(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetControlDetailsA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd as _, fdwdetails) }
}
#[cfg(all(feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn mixerGetControlDetailsW(hmxobj: Option<HMIXEROBJ>, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetControlDetailsW(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetControlDetailsW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd as _, fdwdetails) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetDevCapsA(umxid : usize, pmxcaps : *mut MIXERCAPSA, cbmxcaps : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetDevCapsA(umxid, pmxcaps as _, cbmxcaps) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetDevCapsW(umxid : usize, pmxcaps : *mut MIXERCAPSW, cbmxcaps : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetDevCapsW(umxid, pmxcaps as _, cbmxcaps) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerGetID(hmxobj: Option<HMIXEROBJ>, pumxid: *mut u32, fdwid: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetID(hmxobj : HMIXEROBJ, pumxid : *mut u32, fdwid : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetID(hmxobj.unwrap_or(core::mem::zeroed()) as _, pumxid as _, fdwid) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerGetLineControlsA(hmxobj: Option<HMIXEROBJ>, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineControlsA(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSA, fdwcontrols : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetLineControlsA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxlc as _, fdwcontrols) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerGetLineControlsW(hmxobj: Option<HMIXEROBJ>, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineControlsW(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSW, fdwcontrols : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetLineControlsW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxlc as _, fdwcontrols) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerGetLineInfoA(hmxobj: Option<HMIXEROBJ>, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineInfoA(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEA, fdwinfo : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetLineInfoA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxl as _, fdwinfo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerGetLineInfoW(hmxobj: Option<HMIXEROBJ>, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineInfoW(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEW, fdwinfo : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerGetLineInfoW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxl as _, fdwinfo) }
}
#[inline]
pub unsafe fn mixerGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetNumDevs() -> u32);
    unsafe { mixerGetNumDevs() }
}
#[inline]
pub unsafe fn mixerMessage(hmx: Option<HMIXER>, umsg: u32, dwparam1: Option<usize>, dwparam2: Option<usize>) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerMessage(hmx : HMIXER, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> u32);
    unsafe { mixerMessage(hmx.unwrap_or(core::mem::zeroed()) as _, umsg, dwparam1.unwrap_or(core::mem::zeroed()) as _, dwparam2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn mixerOpen(phmx: Option<*mut HMIXER>, umxid: u32, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerOpen(phmx : *mut HMIXER, umxid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerOpen(phmx.unwrap_or(core::mem::zeroed()) as _, umxid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(all(feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn mixerSetControlDetails(hmxobj: Option<HMIXEROBJ>, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mixerSetControlDetails(hmxobj : HMIXEROBJ, pmxcd : *const MIXERCONTROLDETAILS, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mixerSetControlDetails(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd, fdwdetails) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInAddBuffer(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInAddBuffer(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInAddBuffer(hwi, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInClose(hwi: HWAVEIN) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInClose(hwi : HWAVEIN) -> super::mmsyscom::MMRESULT);
    unsafe { waveInClose(hwi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetDevCapsA(udeviceid : usize, pwic : *mut WAVEINCAPSA, cbwic : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInGetDevCapsA(udeviceid, pwic as _, cbwic) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetDevCapsW(udeviceid : usize, pwic : *mut WAVEINCAPSW, cbwic : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInGetDevCapsW(udeviceid, pwic as _, cbwic) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInGetErrorTextA(mmrerror: super::mmsyscom::MMRESULT, psztext: &mut [u8]) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetErrorTextA(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_core::PSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInGetErrorTextW(mmrerror: super::mmsyscom::MMRESULT, psztext: &mut [u16]) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetErrorTextW(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_core::PWSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInGetID(hwi: HWAVEIN, pudeviceid: *const u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetID(hwi : HWAVEIN, pudeviceid : *const u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInGetID(hwi, pudeviceid) }
}
#[inline]
pub unsafe fn waveInGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInGetNumDevs() -> u32);
    unsafe { waveInGetNumDevs() }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInGetPosition(hwi: HWAVEIN, pmmt: *mut super::mmsyscom::MMTIME, cbmmt: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInGetPosition(hwi : HWAVEIN, pmmt : *mut super::mmsyscom::MMTIME, cbmmt : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInGetPosition(hwi, pmmt as _, cbmmt) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInMessage(hwi: Option<HWAVEIN>, umsg: u32, dw1: Option<usize>, dw2: Option<usize>) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInMessage(hwi : HWAVEIN, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
    unsafe { waveInMessage(hwi.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInOpen(phwi: Option<*mut HWAVEIN>, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInOpen(phwi : *mut HWAVEIN, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInOpen(phwi.unwrap_or(core::mem::zeroed()) as _, udeviceid, pwfx, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInPrepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInPrepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInPrepareHeader(hwi, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInReset(hwi: HWAVEIN) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInReset(hwi : HWAVEIN) -> super::mmsyscom::MMRESULT);
    unsafe { waveInReset(hwi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInStart(hwi: HWAVEIN) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInStart(hwi : HWAVEIN) -> super::mmsyscom::MMRESULT);
    unsafe { waveInStart(hwi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInStop(hwi: HWAVEIN) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInStop(hwi : HWAVEIN) -> super::mmsyscom::MMRESULT);
    unsafe { waveInStop(hwi) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveInUnprepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveInUnprepareHeader(hwi, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutBreakLoop(hwo: HWAVEOUT) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutBreakLoop(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutBreakLoop(hwo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutClose(hwo: HWAVEOUT) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutClose(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutClose(hwo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetDevCapsA(udeviceid : usize, pwoc : *mut WAVEOUTCAPSA, cbwoc : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetDevCapsA(udeviceid, pwoc as _, cbwoc) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetDevCapsW(udeviceid : usize, pwoc : *mut WAVEOUTCAPSW, cbwoc : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetDevCapsW(udeviceid, pwoc as _, cbwoc) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetErrorTextA(mmrerror: super::mmsyscom::MMRESULT, psztext: &mut [u8]) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetErrorTextA(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_core::PSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetErrorTextW(mmrerror: super::mmsyscom::MMRESULT, psztext: &mut [u16]) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetErrorTextW(mmrerror : super::mmsyscom::MMRESULT, psztext : windows_core::PWSTR, cchtext : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetID(hwo : HWAVEOUT, pudeviceid : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetID(hwo, pudeviceid as _) }
}
#[inline]
pub unsafe fn waveOutGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetNumDevs() -> u32);
    unsafe { waveOutGetNumDevs() }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPitch(hwo : HWAVEOUT, pdwpitch : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetPitch(hwo, pdwpitch as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPlaybackRate(hwo : HWAVEOUT, pdwrate : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetPlaybackRate(hwo, pdwrate as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: *mut super::mmsyscom::MMTIME, cbmmt: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPosition(hwo : HWAVEOUT, pmmt : *mut super::mmsyscom::MMTIME, cbmmt : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetPosition(hwo, pmmt as _, cbmmt) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutGetVolume(hwo: Option<HWAVEOUT>, pdwvolume: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutGetVolume(hwo : HWAVEOUT, pdwvolume : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutGetVolume(hwo.unwrap_or(core::mem::zeroed()) as _, pdwvolume as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutMessage(hwo: Option<HWAVEOUT>, umsg: u32, dw1: usize, dw2: usize) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutMessage(hwo : HWAVEOUT, umsg : u32, dw1 : usize, dw2 : usize) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutMessage(hwo.unwrap_or(core::mem::zeroed()) as _, umsg, dw1, dw2) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutOpen(phwo: Option<*mut HWAVEOUT>, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutOpen(phwo : *mut HWAVEOUT, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutOpen(phwo.unwrap_or(core::mem::zeroed()) as _, udeviceid, pwfx, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutPause(hwo: HWAVEOUT) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutPause(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutPause(hwo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutPrepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutPrepareHeader(hwo, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutReset(hwo: HWAVEOUT) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutReset(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutReset(hwo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutRestart(hwo: HWAVEOUT) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutRestart(hwo : HWAVEOUT) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutRestart(hwo) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutSetPitch(hwo : HWAVEOUT, dwpitch : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutSetPitch(hwo, dwpitch) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutSetPlaybackRate(hwo : HWAVEOUT, dwrate : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutSetPlaybackRate(hwo, dwrate) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutSetVolume(hwo: Option<HWAVEOUT>, dwvolume: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutSetVolume(hwo : HWAVEOUT, dwvolume : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutSetVolume(hwo.unwrap_or(core::mem::zeroed()) as _, dwvolume) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutUnprepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutUnprepareHeader(hwo, pwh as _, cbwh) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn waveOutWrite(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn waveOutWrite(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> super::mmsyscom::MMRESULT);
    unsafe { waveOutWrite(hwo, pwh as _, cbwh) }
}
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIDI(pub *mut core::ffi::c_void);
impl HMIDI {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMIDI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIDIIN(pub *mut core::ffi::c_void);
impl HMIDIIN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMIDIIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIDIOUT(pub *mut core::ffi::c_void);
impl HMIDIOUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMIDIOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIDISTRM(pub *mut core::ffi::c_void);
impl HMIDISTRM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMIDISTRM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIXER(pub *mut core::ffi::c_void);
impl HMIXER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMIXER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIXEROBJ(pub *mut core::ffi::c_void);
impl HMIXEROBJ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMIXEROBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWAVE(pub *mut core::ffi::c_void);
impl HWAVE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HWAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWAVEIN(pub *mut core::ffi::c_void);
impl HWAVEIN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HWAVEIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWAVEOUT(pub *mut core::ffi::c_void);
impl HWAVEOUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HWAVEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KEYARRAY = [u16; 128];
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPAUXCAPS(pub LPAUXCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPAUXCAPS2(pub LPAUXCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAUXCAPS2A(pub *mut AUXCAPS2A);
#[cfg(feature = "mmsyscom")]
impl LPAUXCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPAUXCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAUXCAPS2W(pub *mut AUXCAPS2W);
#[cfg(feature = "mmsyscom")]
impl LPAUXCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPAUXCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAUXCAPSA(pub *mut AUXCAPSA);
#[cfg(feature = "mmsyscom")]
impl LPAUXCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPAUXCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAUXCAPSW(pub *mut AUXCAPSW);
#[cfg(feature = "mmsyscom")]
impl LPAUXCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPAUXCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCWAVEFORMATEX(pub *const WAVEFORMATEX);
impl LPCWAVEFORMATEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCWAVEFORMATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHMIDI(pub *mut HMIDI);
impl LPHMIDI {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHMIDI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHMIDIIN(pub *mut HMIDIIN);
impl LPHMIDIIN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHMIDIIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHMIDIOUT(pub *mut HMIDIOUT);
impl LPHMIDIOUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHMIDIOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHMIDISTRM(pub *mut HMIDISTRM);
impl LPHMIDISTRM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHMIDISTRM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHMIXER(pub *mut HMIXER);
impl LPHMIXER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHMIXER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHMIXEROBJ(pub *mut HMIXEROBJ);
impl LPHMIXEROBJ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHMIXEROBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHWAVEIN(pub *mut HWAVEIN);
impl LPHWAVEIN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHWAVEIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHWAVEOUT(pub *mut HWAVEOUT);
impl LPHWAVEOUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHWAVEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPKEYARRAY(pub *mut u16);
impl LPKEYARRAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPKEYARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPMIDICALLBACK = Option<unsafe extern "system" fn()>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIHDR(pub *mut MIDIHDR);
impl LPMIDIHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIDIHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIDIINCAPS(pub LPMIDIINCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIDIINCAPS2(pub LPMIDIINCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIINCAPS2A(pub *mut MIDIINCAPS2A);
#[cfg(feature = "mmsyscom")]
impl LPMIDIINCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIDIINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIINCAPS2W(pub *mut MIDIINCAPS2W);
#[cfg(feature = "mmsyscom")]
impl LPMIDIINCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIDIINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIINCAPSA(pub *mut MIDIINCAPSA);
#[cfg(feature = "mmsyscom")]
impl LPMIDIINCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIDIINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIINCAPSW(pub *mut MIDIINCAPSW);
#[cfg(feature = "mmsyscom")]
impl LPMIDIINCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIDIINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIDIOUTCAPS(pub LPMIDIOUTCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIDIOUTCAPS2(pub LPMIDIOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIOUTCAPS2A(pub *mut MIDIOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
impl LPMIDIOUTCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIOUTCAPS2W(pub *mut MIDIOUTCAPS2W);
#[cfg(feature = "mmsyscom")]
impl LPMIDIOUTCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIOUTCAPSA(pub *mut MIDIOUTCAPSA);
#[cfg(feature = "mmsyscom")]
impl LPMIDIOUTCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIOUTCAPSW(pub *mut MIDIOUTCAPSW);
#[cfg(feature = "mmsyscom")]
impl LPMIDIOUTCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIPROPTEMPO(pub *mut MIDIPROPTEMPO);
impl LPMIDIPROPTEMPO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIDIPROPTEMPO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIDIPROPTIMEDIV(pub *mut MIDIPROPTIMEDIV);
impl LPMIDIPROPTIMEDIV {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIDIPROPTIMEDIV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIXERCAPS(pub LPMIXERCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIXERCAPS2(pub LPMIXERCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCAPS2A(pub *mut MIXERCAPS2A);
#[cfg(feature = "mmsyscom")]
impl LPMIXERCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIXERCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCAPS2W(pub *mut MIXERCAPS2W);
#[cfg(feature = "mmsyscom")]
impl LPMIXERCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIXERCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCAPSA(pub *mut MIXERCAPSA);
#[cfg(feature = "mmsyscom")]
impl LPMIXERCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIXERCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCAPSW(pub *mut MIXERCAPSW);
#[cfg(feature = "mmsyscom")]
impl LPMIXERCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIXERCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIXERCONTROL(pub LPMIXERCONTROLA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCONTROLA(pub *mut MIXERCONTROLA);
impl LPMIXERCONTROLA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERCONTROLA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCONTROLDETAILS(pub *mut MIXERCONTROLDETAILS);
#[cfg(feature = "windef")]
impl LPMIXERCONTROLDETAILS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPMIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCONTROLDETAILS_BOOLEAN(pub *mut MIXERCONTROLDETAILS_BOOLEAN);
impl LPMIXERCONTROLDETAILS_BOOLEAN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERCONTROLDETAILS_BOOLEAN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIXERCONTROLDETAILS_LISTTEXT(pub LPMIXERCONTROLDETAILS_LISTTEXTA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCONTROLDETAILS_LISTTEXTA(pub *mut MIXERCONTROLDETAILS_LISTTEXTA);
impl LPMIXERCONTROLDETAILS_LISTTEXTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCONTROLDETAILS_LISTTEXTW(pub *mut MIXERCONTROLDETAILS_LISTTEXTW);
impl LPMIXERCONTROLDETAILS_LISTTEXTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCONTROLDETAILS_SIGNED(pub *mut MIXERCONTROLDETAILS_SIGNED);
impl LPMIXERCONTROLDETAILS_SIGNED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERCONTROLDETAILS_SIGNED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCONTROLDETAILS_UNSIGNED(pub *mut MIXERCONTROLDETAILS_UNSIGNED);
impl LPMIXERCONTROLDETAILS_UNSIGNED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERCONTROLDETAILS_UNSIGNED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERCONTROLW(pub *mut MIXERCONTROLW);
impl LPMIXERCONTROLW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERCONTROLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIXERLINE(pub LPMIXERLINEA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERLINEA(pub *mut MIXERLINEA);
#[cfg(feature = "mmsyscom")]
impl LPMIXERLINEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIXERLINEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPMIXERLINECONTROLS(pub LPMIXERLINECONTROLSA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERLINECONTROLSA(pub *mut MIXERLINECONTROLSA);
impl LPMIXERLINECONTROLSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERLINECONTROLSW(pub *mut MIXERLINECONTROLSW);
impl LPMIXERLINECONTROLSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMIXERLINEW(pub *mut MIXERLINEW);
#[cfg(feature = "mmsyscom")]
impl LPMIXERLINEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPMIXERLINEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPATCHARRAY(pub *mut u16);
impl LPPATCHARRAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPPATCHARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPCMWAVEFORMAT(pub *mut PCMWAVEFORMAT);
impl LPPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPWAVECALLBACK = Option<unsafe extern "system" fn()>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEFORMAT(pub *mut WAVEFORMAT);
impl LPWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEFORMATEX(pub *mut WAVEFORMATEX);
impl LPWAVEFORMATEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWAVEFORMATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEHDR(pub *mut WAVEHDR);
impl LPWAVEHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWAVEHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPWAVEINCAPS(pub LPWAVEINCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPWAVEINCAPS2(pub LPWAVEINCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEINCAPS2A(pub *mut WAVEINCAPS2A);
#[cfg(feature = "mmsyscom")]
impl LPWAVEINCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPWAVEINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEINCAPS2W(pub *mut WAVEINCAPS2W);
#[cfg(feature = "mmsyscom")]
impl LPWAVEINCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPWAVEINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEINCAPSA(pub *mut WAVEINCAPSA);
#[cfg(feature = "mmsyscom")]
impl LPWAVEINCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPWAVEINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEINCAPSW(pub *mut WAVEINCAPSW);
#[cfg(feature = "mmsyscom")]
impl LPWAVEINCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPWAVEINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPWAVEOUTCAPS(pub LPWAVEOUTCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPWAVEOUTCAPS2(pub LPWAVEOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEOUTCAPS2A(pub *mut WAVEOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
impl LPWAVEOUTCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPWAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEOUTCAPS2W(pub *mut WAVEOUTCAPS2W);
#[cfg(feature = "mmsyscom")]
impl LPWAVEOUTCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPWAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEOUTCAPSA(pub *mut WAVEOUTCAPSA);
#[cfg(feature = "mmsyscom")]
impl LPWAVEOUTCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPWAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEOUTCAPSW(pub *mut WAVEOUTCAPSW);
#[cfg(feature = "mmsyscom")]
impl LPWAVEOUTCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for LPWAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
    pub lpData: windows_core::PSTR,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPAUXCAPS(pub NPAUXCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPAUXCAPS2(pub NPAUXCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPAUXCAPS2A(pub *mut AUXCAPS2A);
#[cfg(feature = "mmsyscom")]
impl NPAUXCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPAUXCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPAUXCAPS2W(pub *mut AUXCAPS2W);
#[cfg(feature = "mmsyscom")]
impl NPAUXCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPAUXCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPAUXCAPSA(pub *mut AUXCAPSA);
#[cfg(feature = "mmsyscom")]
impl NPAUXCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPAUXCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPAUXCAPSW(pub *mut AUXCAPSW);
#[cfg(feature = "mmsyscom")]
impl NPAUXCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPAUXCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIHDR(pub *mut MIDIHDR);
impl NPMIDIHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPMIDIHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPMIDIINCAPS(pub NPMIDIINCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPMIDIINCAPS2(pub NPMIDIINCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIINCAPS2A(pub *mut MIDIINCAPS2A);
#[cfg(feature = "mmsyscom")]
impl NPMIDIINCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPMIDIINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIINCAPS2W(pub *mut MIDIINCAPS2W);
#[cfg(feature = "mmsyscom")]
impl NPMIDIINCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPMIDIINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIINCAPSA(pub *mut MIDIINCAPSA);
#[cfg(feature = "mmsyscom")]
impl NPMIDIINCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPMIDIINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIINCAPSW(pub *mut MIDIINCAPSW);
#[cfg(feature = "mmsyscom")]
impl NPMIDIINCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPMIDIINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPMIDIOUTCAPS(pub NPMIDIOUTCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPMIDIOUTCAPS2(pub NPMIDIOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIOUTCAPS2A(pub *mut MIDIOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
impl NPMIDIOUTCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPMIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIOUTCAPS2W(pub *mut MIDIOUTCAPS2W);
#[cfg(feature = "mmsyscom")]
impl NPMIDIOUTCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPMIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIOUTCAPSA(pub *mut MIDIOUTCAPSA);
#[cfg(feature = "mmsyscom")]
impl NPMIDIOUTCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPMIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMIDIOUTCAPSW(pub *mut MIDIOUTCAPSW);
#[cfg(feature = "mmsyscom")]
impl NPMIDIOUTCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPMIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPPCMWAVEFORMAT(pub *mut PCMWAVEFORMAT);
impl NPPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEFORMAT(pub *mut WAVEFORMAT);
impl NPWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEFORMATEX(pub *mut WAVEFORMATEX);
impl NPWAVEFORMATEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPWAVEFORMATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEHDR(pub *mut WAVEHDR);
impl NPWAVEHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPWAVEHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPWAVEINCAPS(pub NPWAVEINCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPWAVEINCAPS2(pub NPWAVEINCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEINCAPS2A(pub *mut WAVEINCAPS2A);
#[cfg(feature = "mmsyscom")]
impl NPWAVEINCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPWAVEINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEINCAPS2W(pub *mut WAVEINCAPS2W);
#[cfg(feature = "mmsyscom")]
impl NPWAVEINCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPWAVEINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEINCAPSA(pub *mut WAVEINCAPSA);
#[cfg(feature = "mmsyscom")]
impl NPWAVEINCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPWAVEINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEINCAPSW(pub *mut WAVEINCAPSW);
#[cfg(feature = "mmsyscom")]
impl NPWAVEINCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPWAVEINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPWAVEOUTCAPS(pub NPWAVEOUTCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPWAVEOUTCAPS2(pub NPWAVEOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEOUTCAPS2A(pub *mut WAVEOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
impl NPWAVEOUTCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPWAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEOUTCAPS2W(pub *mut WAVEOUTCAPS2W);
#[cfg(feature = "mmsyscom")]
impl NPWAVEOUTCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPWAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEOUTCAPSA(pub *mut WAVEOUTCAPSA);
#[cfg(feature = "mmsyscom")]
impl NPWAVEOUTCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPWAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEOUTCAPSW(pub *mut WAVEOUTCAPSW);
#[cfg(feature = "mmsyscom")]
impl NPWAVEOUTCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for NPWAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PATCHARRAY = [u16; 128];
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PAUXCAPS(pub PAUXCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PAUXCAPS2(pub PAUXCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUXCAPS2A(pub *mut AUXCAPS2A);
#[cfg(feature = "mmsyscom")]
impl PAUXCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PAUXCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUXCAPS2W(pub *mut AUXCAPS2W);
#[cfg(feature = "mmsyscom")]
impl PAUXCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PAUXCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUXCAPSA(pub *mut AUXCAPSA);
#[cfg(feature = "mmsyscom")]
impl PAUXCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PAUXCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUXCAPSW(pub *mut AUXCAPSW);
#[cfg(feature = "mmsyscom")]
impl PAUXCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PAUXCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIHDR(pub *mut MIDIHDR);
impl PMIDIHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIDIHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIDIINCAPS(pub PMIDIINCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIDIINCAPS2(pub PMIDIINCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIINCAPS2A(pub *mut MIDIINCAPS2A);
#[cfg(feature = "mmsyscom")]
impl PMIDIINCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIDIINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIINCAPS2W(pub *mut MIDIINCAPS2W);
#[cfg(feature = "mmsyscom")]
impl PMIDIINCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIDIINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIINCAPSA(pub *mut MIDIINCAPSA);
#[cfg(feature = "mmsyscom")]
impl PMIDIINCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIDIINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIINCAPSW(pub *mut MIDIINCAPSW);
#[cfg(feature = "mmsyscom")]
impl PMIDIINCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIDIINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIDIOUTCAPS(pub PMIDIOUTCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIDIOUTCAPS2(pub PMIDIOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIOUTCAPS2A(pub *mut MIDIOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
impl PMIDIOUTCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIOUTCAPS2W(pub *mut MIDIOUTCAPS2W);
#[cfg(feature = "mmsyscom")]
impl PMIDIOUTCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIOUTCAPSA(pub *mut MIDIOUTCAPSA);
#[cfg(feature = "mmsyscom")]
impl PMIDIOUTCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDIOUTCAPSW(pub *mut MIDIOUTCAPSW);
#[cfg(feature = "mmsyscom")]
impl PMIDIOUTCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIXERCAPS(pub PMIXERCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIXERCAPS2(pub PMIXERCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCAPS2A(pub *mut MIXERCAPS2A);
#[cfg(feature = "mmsyscom")]
impl PMIXERCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIXERCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCAPS2W(pub *mut MIXERCAPS2W);
#[cfg(feature = "mmsyscom")]
impl PMIXERCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIXERCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCAPSA(pub *mut MIXERCAPSA);
#[cfg(feature = "mmsyscom")]
impl PMIXERCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIXERCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCAPSW(pub *mut MIXERCAPSW);
#[cfg(feature = "mmsyscom")]
impl PMIXERCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIXERCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIXERCONTROL(pub PMIXERCONTROLA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCONTROLA(pub *mut MIXERCONTROLA);
impl PMIXERCONTROLA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERCONTROLA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCONTROLDETAILS(pub *mut MIXERCONTROLDETAILS);
#[cfg(feature = "windef")]
impl PMIXERCONTROLDETAILS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PMIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCONTROLDETAILS_BOOLEAN(pub *mut MIXERCONTROLDETAILS_BOOLEAN);
impl PMIXERCONTROLDETAILS_BOOLEAN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERCONTROLDETAILS_BOOLEAN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIXERCONTROLDETAILS_LISTTEXT(pub PMIXERCONTROLDETAILS_LISTTEXTA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCONTROLDETAILS_LISTTEXTA(pub *mut MIXERCONTROLDETAILS_LISTTEXTA);
impl PMIXERCONTROLDETAILS_LISTTEXTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCONTROLDETAILS_LISTTEXTW(pub *mut MIXERCONTROLDETAILS_LISTTEXTW);
impl PMIXERCONTROLDETAILS_LISTTEXTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCONTROLDETAILS_SIGNED(pub *mut MIXERCONTROLDETAILS_SIGNED);
impl PMIXERCONTROLDETAILS_SIGNED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERCONTROLDETAILS_SIGNED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCONTROLDETAILS_UNSIGNED(pub *mut MIXERCONTROLDETAILS_UNSIGNED);
impl PMIXERCONTROLDETAILS_UNSIGNED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERCONTROLDETAILS_UNSIGNED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERCONTROLW(pub *mut MIXERCONTROLW);
impl PMIXERCONTROLW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERCONTROLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIXERLINE(pub PMIXERLINEA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERLINEA(pub *mut MIXERLINEA);
#[cfg(feature = "mmsyscom")]
impl PMIXERLINEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIXERLINEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMIXERLINECONTROLS(pub PMIXERLINECONTROLSA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERLINECONTROLSA(pub *mut MIXERLINECONTROLSA);
impl PMIXERLINECONTROLSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERLINECONTROLSW(pub *mut MIXERLINECONTROLSW);
impl PMIXERLINECONTROLSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIXERLINEW(pub *mut MIXERLINEW);
#[cfg(feature = "mmsyscom")]
impl PMIXERLINEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PMIXERLINEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCMWAVEFORMAT(pub *mut PCMWAVEFORMAT);
impl PPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEFORMAT(pub *mut WAVEFORMAT);
impl PWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEFORMATEX(pub *mut WAVEFORMATEX);
impl PWAVEFORMATEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWAVEFORMATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEHDR(pub *mut WAVEHDR);
impl PWAVEHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWAVEHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PWAVEINCAPS(pub PWAVEINCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PWAVEINCAPS2(pub PWAVEINCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEINCAPS2A(pub *mut WAVEINCAPS2A);
#[cfg(feature = "mmsyscom")]
impl PWAVEINCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PWAVEINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEINCAPS2W(pub *mut WAVEINCAPS2W);
#[cfg(feature = "mmsyscom")]
impl PWAVEINCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PWAVEINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEINCAPSA(pub *mut WAVEINCAPSA);
#[cfg(feature = "mmsyscom")]
impl PWAVEINCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PWAVEINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEINCAPSW(pub *mut WAVEINCAPSW);
#[cfg(feature = "mmsyscom")]
impl PWAVEINCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PWAVEINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PWAVEOUTCAPS(pub PWAVEOUTCAPSA);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PWAVEOUTCAPS2(pub PWAVEOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEOUTCAPS2A(pub *mut WAVEOUTCAPS2A);
#[cfg(feature = "mmsyscom")]
impl PWAVEOUTCAPS2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PWAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEOUTCAPS2W(pub *mut WAVEOUTCAPS2W);
#[cfg(feature = "mmsyscom")]
impl PWAVEOUTCAPS2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PWAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEOUTCAPSA(pub *mut WAVEOUTCAPSA);
#[cfg(feature = "mmsyscom")]
impl PWAVEOUTCAPSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PWAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmsyscom")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEOUTCAPSW(pub *mut WAVEOUTCAPSW);
#[cfg(feature = "mmsyscom")]
impl PWAVEOUTCAPSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmsyscom")]
impl Default for PWAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
    pub lpData: windows_core::PSTR,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
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
