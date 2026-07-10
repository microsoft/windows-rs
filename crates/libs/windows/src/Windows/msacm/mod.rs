#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmDriverAddA(phadid: *mut HACMDRIVERID, hinstmodule: super::minwindef::HINSTANCE, lparam: super::minwindef::LPARAM, dwpriority: u32, fdwadd: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverAddA(phadid : *mut HACMDRIVERID, hinstmodule : super::minwindef::HINSTANCE, lparam : super::minwindef::LPARAM, dwpriority : u32, fdwadd : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverAddA(phadid as _, hinstmodule, lparam, dwpriority, fdwadd) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmDriverAddW(phadid: *mut HACMDRIVERID, hinstmodule: super::minwindef::HINSTANCE, lparam: super::minwindef::LPARAM, dwpriority: u32, fdwadd: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverAddW(phadid : *mut HACMDRIVERID, hinstmodule : super::minwindef::HINSTANCE, lparam : super::minwindef::LPARAM, dwpriority : u32, fdwadd : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverAddW(phadid as _, hinstmodule, lparam, dwpriority, fdwadd) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmDriverClose(had: HACMDRIVER, fdwclose: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverClose(had : HACMDRIVER, fdwclose : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverClose(had, fdwclose) }
}
#[cfg(all(feature = "mmiscapi", feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn acmDriverDetailsA(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverDetailsA(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSA, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverDetailsA(hadid, padd as _, fdwdetails) }
}
#[cfg(all(feature = "mmiscapi", feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn acmDriverDetailsW(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverDetailsW(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSW, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverDetailsW(hadid, padd as _, fdwdetails) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmDriverEnum(fncallback: ACMDRIVERENUMCB, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverEnum(fncallback : ACMDRIVERENUMCB, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverEnum(fncallback, dwinstance, fdwenum) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmDriverID(hao: HACMOBJ, phadid: *mut HACMDRIVERID, fdwdriverid: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverID(hao : HACMOBJ, phadid : *mut HACMDRIVERID, fdwdriverid : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverID(hao, phadid as _, fdwdriverid) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn acmDriverMessage(had: HACMDRIVER, umsg: u32, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverMessage(had : HACMDRIVER, umsg : u32, lparam1 : super::minwindef::LPARAM, lparam2 : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { acmDriverMessage(had, umsg, lparam1, lparam2) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmDriverOpen(phad: *mut HACMDRIVER, hadid: HACMDRIVERID, fdwopen: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverOpen(phad : *mut HACMDRIVER, hadid : HACMDRIVERID, fdwopen : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverOpen(phad as _, hadid, fdwopen) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmDriverPriority(hadid: HACMDRIVERID, dwpriority: u32, fdwpriority: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverPriority(hadid : HACMDRIVERID, dwpriority : u32, fdwpriority : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverPriority(hadid, dwpriority, fdwpriority) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmDriverRemove(hadid: HACMDRIVERID, fdwremove: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverRemove(hadid : HACMDRIVERID, fdwremove : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmDriverRemove(hadid, fdwremove) }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterChooseA(pafltrc : *mut ACMFILTERCHOOSEA) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterChooseA(pafltrc as _) }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterChooseW(pafltrc : *mut ACMFILTERCHOOSEW) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterChooseW(pafltrc as _) }
}
#[cfg(all(feature = "mmreg", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFilterDetailsA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterDetailsA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterDetailsA(had, pafd as _, fdwdetails) }
}
#[cfg(all(feature = "mmreg", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFilterDetailsW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterDetailsW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterDetailsW(had, pafd as _, fdwdetails) }
}
#[cfg(all(feature = "mmreg", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFilterEnumA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fncallback: ACMFILTERENUMCBA, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterEnumA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fncallback : ACMFILTERENUMCBA, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterEnumA(had, pafd as _, fncallback, dwinstance, fdwenum) }
}
#[cfg(all(feature = "mmreg", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFilterEnumW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fncallback: ACMFILTERENUMCBW, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterEnumW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fncallback : ACMFILTERENUMCBW, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterEnumW(had, pafd as _, fncallback, dwinstance, fdwenum) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmFilterTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterTagDetailsA(had, paftd as _, fdwdetails) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmFilterTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterTagDetailsW(had, paftd as _, fdwdetails) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmFilterTagEnumA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ACMFILTERTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterTagEnumA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fncallback : ACMFILTERTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterTagEnumA(had, paftd as _, fncallback, dwinstance, fdwenum) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmFilterTagEnumW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ACMFILTERTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFilterTagEnumW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fncallback : ACMFILTERTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFilterTagEnumW(had, paftd as _, fncallback, dwinstance, fdwenum) }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatChooseA(pafmtc : *mut ACMFORMATCHOOSEA) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatChooseA(pafmtc as _) }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatChooseW(pafmtc : *mut ACMFORMATCHOOSEW) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatChooseW(pafmtc as _) }
}
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFormatDetailsA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatDetailsA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatDetailsA(had, pafd as _, fdwdetails) }
}
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFormatDetailsW(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSW, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatDetailsW(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSW, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatDetailsW(had, pafd as _, fdwdetails) }
}
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFormatEnumA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fncallback: ACMFORMATENUMCBA, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatEnumA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fncallback : ACMFORMATENUMCBA, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatEnumA(had, pafd as _, fncallback, dwinstance, fdwenum) }
}
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFormatEnumW(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSW, fncallback: ACMFORMATENUMCBW, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatEnumW(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSW, fncallback : ACMFORMATENUMCBW, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatEnumW(had, pafd as _, fncallback, dwinstance, fdwenum) }
}
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmFormatSuggest(had: HACMDRIVER, pwfxsrc: *mut super::mmeapi::WAVEFORMATEX, pwfxdst: *mut super::mmeapi::WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatSuggest(had : HACMDRIVER, pwfxsrc : *mut super::mmeapi::WAVEFORMATEX, pwfxdst : *mut super::mmeapi::WAVEFORMATEX, cbwfxdst : u32, fdwsuggest : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatSuggest(had, pwfxsrc as _, pwfxdst as _, cbwfxdst, fdwsuggest) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmFormatTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatTagDetailsA(had, paftd as _, fdwdetails) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmFormatTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fdwdetails : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatTagDetailsW(had, paftd as _, fdwdetails) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmFormatTagEnumA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ACMFORMATTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatTagEnumA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fncallback : ACMFORMATTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatTagEnumA(had, paftd as _, fncallback, dwinstance, fdwenum) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmFormatTagEnumW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ACMFORMATTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmFormatTagEnumW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fncallback : ACMFORMATTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmFormatTagEnumW(had, paftd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmGetVersion() -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmGetVersion() -> u32);
    unsafe { acmGetVersion() }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmMetrics(hao: HACMOBJ, umetric: u32, pmetric: *mut core::ffi::c_void) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmMetrics(hao : HACMOBJ, umetric : u32, pmetric : *mut core::ffi::c_void) -> super::mmsyscom::MMRESULT);
    unsafe { acmMetrics(hao, umetric, pmetric as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmStreamClose(has: HACMSTREAM, fdwclose: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmStreamClose(has : HACMSTREAM, fdwclose : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmStreamClose(has, fdwclose) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmStreamConvert(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmStreamConvert(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwconvert : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmStreamConvert(has, pash as _, fdwconvert) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmStreamMessage(has: HACMSTREAM, umsg: u32, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmStreamMessage(has : HACMSTREAM, umsg : u32, lparam1 : super::minwindef::LPARAM, lparam2 : super::minwindef::LPARAM) -> super::mmsyscom::MMRESULT);
    unsafe { acmStreamMessage(has, umsg, lparam1, lparam2) }
}
#[cfg(all(feature = "mmeapi", feature = "mmreg", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmStreamOpen(phas: *mut HACMSTREAM, had: HACMDRIVER, pwfxsrc: *mut super::mmeapi::WAVEFORMATEX, pwfxdst: *mut super::mmeapi::WAVEFORMATEX, pwfltr: *mut super::mmreg::WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmStreamOpen(phas : *mut HACMSTREAM, had : HACMDRIVER, pwfxsrc : *mut super::mmeapi::WAVEFORMATEX, pwfxdst : *mut super::mmeapi::WAVEFORMATEX, pwfltr : *mut super::mmreg::WAVEFILTER, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmStreamOpen(phas as _, had, pwfxsrc as _, pwfxdst as _, pwfltr as _, dwcallback, dwinstance, fdwopen) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmStreamPrepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmStreamPrepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwprepare : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmStreamPrepareHeader(has, pash as _, fdwprepare) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmStreamReset(has: HACMSTREAM, fdwreset: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmStreamReset(has : HACMSTREAM, fdwreset : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmStreamReset(has, fdwreset) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn acmStreamSize(has: HACMSTREAM, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmStreamSize(has : HACMSTREAM, cbinput : u32, pdwoutputbytes : *mut u32, fdwsize : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmStreamSize(has, cbinput, pdwoutputbytes as _, fdwsize) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn acmStreamUnprepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmStreamUnprepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwunprepare : u32) -> super::mmsyscom::MMRESULT);
    unsafe { acmStreamUnprepareHeader(has, pash as _, fdwunprepare) }
}
pub const ACMDM_BASE: u32 = 24576;
pub const ACMDM_DRIVER_ABOUT: u32 = 24587;
pub const ACMDM_RESERVED_HIGH: u32 = 28671;
pub const ACMDM_RESERVED_LOW: u32 = 24576;
pub const ACMDM_USER: u32 = 16384;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMDRIVERDETAILSA {
    pub cbStruct: u32,
    pub fccType: super::mmiscapi::FOURCC,
    pub fccComp: super::mmiscapi::FOURCC,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::windef::HICON,
    pub szShortName: [i8; 32],
    pub szLongName: [i8; 128],
    pub szCopyright: [i8; 80],
    pub szLicensing: [i8; 128],
    pub szFeatures: [i8; 512],
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMDRIVERDETAILSW {
    pub cbStruct: u32,
    pub fccType: super::mmiscapi::FOURCC,
    pub fccComp: super::mmiscapi::FOURCC,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::windef::HICON,
    pub szShortName: [u16; 32],
    pub szLongName: [u16; 128],
    pub szCopyright: [u16; 80],
    pub szLicensing: [u16; 128],
    pub szFeatures: [u16; 512],
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80;
pub const ACMDRIVERDETAILS_FCCCOMP_UNDEFINED: u32 = 0;
pub const ACMDRIVERDETAILS_FCCTYPE_AUDIOCODEC: u32 = 1667528033;
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512;
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128;
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128;
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32;
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: u32 = 16;
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: u32 = 1;
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: u32 = 2;
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: u32 = 2147483648;
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: u32 = 4;
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: u32 = 8;
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: u32 = 1073741824;
pub type ACMDRIVERENUMCB = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[cfg(feature = "minwindef")]
pub type ACMDRIVERPROC = Option<unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::minwindef::LPARAM, param4: super::minwindef::LPARAM) -> super::minwindef::LRESULT>;
pub const ACMERR_BASE: u32 = 512;
pub const ACMERR_BUSY: u32 = 513;
pub const ACMERR_CANCELED: u32 = 515;
pub const ACMERR_NOTPOSSIBLE: u32 = 512;
pub const ACMERR_UNPREPARED: u32 = 514;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::windef::HWND,
    pub pwfltr: super::mmreg::LPWAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: windows_core::PCSTR,
    pub szFilterTag: [i8; 48],
    pub szFilter: [i8; 128],
    pub pszName: windows_core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: super::mmreg::LPWAVEFILTER,
    pub hInstance: super::minwindef::HINSTANCE,
    pub pszTemplateName: windows_core::PCSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCA,
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ACMFILTERCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ACMFILTERCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::windef::HWND,
    pub pwfltr: super::mmreg::LPWAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: windows_core::PCWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: windows_core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: super::mmreg::LPWAVEFILTER,
    pub hInstance: super::minwindef::HINSTANCE,
    pub pszTemplateName: windows_core::PCWSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCW,
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: u32 = 128;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: u32 = 8;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: u32 = 16;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: u32 = 32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: u32 = 64;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: u32 = 4;
#[repr(C, packed(1))]
#[cfg(feature = "mmreg")]
#[derive(Clone, Copy)]
pub struct ACMFILTERDETAILSA {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: super::mmreg::LPWAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [i8; 128],
}
#[cfg(feature = "mmreg")]
impl Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmreg")]
#[derive(Clone, Copy)]
pub struct ACMFILTERDETAILSW {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: super::mmreg::LPWAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [u16; 128],
}
#[cfg(feature = "mmreg")]
impl Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128;
#[cfg(feature = "mmreg")]
pub type ACMFILTERENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[cfg(feature = "mmreg")]
pub type ACMFILTERENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [i8; 48],
}
impl Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u16; 48],
}
impl Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48;
pub type ACMFILTERTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
pub type ACMFILTERTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::windef::HWND,
    pub pwfx: super::mmeapi::LPWAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: windows_core::PCSTR,
    pub szFormatTag: [i8; 48],
    pub szFormat: [i8; 128],
    pub pszName: windows_core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: super::mmeapi::LPWAVEFORMATEX,
    pub hInstance: super::minwindef::HINSTANCE,
    pub pszTemplateName: windows_core::PCSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCA,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ACMFORMATCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ACMFORMATCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::windef::HWND,
    pub pwfx: super::mmeapi::LPWAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: windows_core::PCWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: windows_core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: super::mmeapi::LPWAVEFORMATEX,
    pub hInstance: super::minwindef::HINSTANCE,
    pub pszTemplateName: windows_core::PCWSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCW,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: u32 = 128;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: u32 = 8;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: u32 = 16;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: u32 = 32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: u32 = 64;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: u32 = 4;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct ACMFORMATDETAILSA {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: super::mmeapi::LPWAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [i8; 128],
}
#[cfg(feature = "mmeapi")]
impl Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct ACMFORMATDETAILSW {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: super::mmeapi::LPWAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [u16; 128],
}
#[cfg(feature = "mmeapi")]
impl Default for ACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128;
#[cfg(feature = "mmeapi")]
pub type ACMFORMATENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[cfg(feature = "mmeapi")]
pub type ACMFORMATENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [i8; 48],
}
impl Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u16; 48],
}
impl Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48;
pub type ACMFORMATTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
pub type ACMFORMATTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
pub const ACMHELPMSGCONTEXTHELPA: windows_core::PCSTR = windows_core::s!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPW: windows_core::PCWSTR = windows_core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTMENUA: windows_core::PCSTR = windows_core::s!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUW: windows_core::PCWSTR = windows_core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGSTRINGA: windows_core::PCSTR = windows_core::s!("acmchoose_help");
pub const ACMHELPMSGSTRINGW: windows_core::PCWSTR = windows_core::w!("acmchoose_help");
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: super::minwindef::LPBYTE,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: super::minwindef::LPBYTE,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 10],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: super::minwindef::LPBYTE,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: super::minwindef::LPBYTE,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 15],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMSTREAMHEADER_STATUSF_DONE: u32 = 65536;
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: u32 = 1048576;
pub const ACMSTREAMHEADER_STATUSF_PREPARED: u32 = 131072;
pub const ACM_DRIVERADDF_FUNCTION: u32 = 3;
pub const ACM_DRIVERADDF_GLOBAL: u32 = 8;
pub const ACM_DRIVERADDF_LOCAL: u32 = 0;
pub const ACM_DRIVERADDF_NAME: u32 = 1;
pub const ACM_DRIVERADDF_NOTIFYHWND: u32 = 4;
pub const ACM_DRIVERADDF_TYPEMASK: u32 = 7;
pub const ACM_DRIVERENUMF_DISABLED: u32 = 2147483648;
pub const ACM_DRIVERENUMF_NOLOCAL: u32 = 1073741824;
pub const ACM_DRIVERPRIORITYF_ABLEMASK: u32 = 3;
pub const ACM_DRIVERPRIORITYF_BEGIN: u32 = 65536;
pub const ACM_DRIVERPRIORITYF_DEFERMASK: u32 = 196608;
pub const ACM_DRIVERPRIORITYF_DISABLE: u32 = 2;
pub const ACM_DRIVERPRIORITYF_ENABLE: u32 = 1;
pub const ACM_DRIVERPRIORITYF_END: u32 = 131072;
pub const ACM_FILTERDETAILSF_FILTER: u32 = 1;
pub const ACM_FILTERDETAILSF_INDEX: u32 = 0;
pub const ACM_FILTERDETAILSF_QUERYMASK: u32 = 15;
pub const ACM_FILTERENUMF_DWFILTERTAG: u32 = 65536;
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: u32 = 1;
pub const ACM_FILTERTAGDETAILSF_INDEX: u32 = 0;
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: u32 = 2;
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: u32 = 15;
pub const ACM_FORMATDETAILSF_FORMAT: u32 = 1;
pub const ACM_FORMATDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATDETAILSF_QUERYMASK: u32 = 15;
pub const ACM_FORMATENUMF_CONVERT: u32 = 1048576;
pub const ACM_FORMATENUMF_HARDWARE: u32 = 4194304;
pub const ACM_FORMATENUMF_INPUT: u32 = 8388608;
pub const ACM_FORMATENUMF_NCHANNELS: u32 = 131072;
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: u32 = 262144;
pub const ACM_FORMATENUMF_OUTPUT: u32 = 16777216;
pub const ACM_FORMATENUMF_SUGGEST: u32 = 2097152;
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: u32 = 524288;
pub const ACM_FORMATENUMF_WFORMATTAG: u32 = 65536;
pub const ACM_FORMATSUGGESTF_NCHANNELS: u32 = 131072;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: u32 = 262144;
pub const ACM_FORMATSUGGESTF_TYPEMASK: u32 = 16711680;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: u32 = 524288;
pub const ACM_FORMATSUGGESTF_WFORMATTAG: u32 = 65536;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: u32 = 1;
pub const ACM_FORMATTAGDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: u32 = 2;
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: u32 = 15;
pub const ACM_METRIC_COUNT_CODECS: u32 = 2;
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3;
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5;
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1;
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4;
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6;
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21;
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22;
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24;
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20;
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23;
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101;
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100;
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30;
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31;
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51;
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50;
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4;
pub const ACM_STREAMCONVERTF_END: u32 = 32;
pub const ACM_STREAMCONVERTF_START: u32 = 16;
pub const ACM_STREAMOPENF_ASYNC: u32 = 2;
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4;
pub const ACM_STREAMOPENF_QUERY: u32 = 1;
pub const ACM_STREAMSIZEF_DESTINATION: u32 = 1;
pub const ACM_STREAMSIZEF_QUERYMASK: u32 = 15;
pub const ACM_STREAMSIZEF_SOURCE: u32 = 0;
pub const DRVM_MAPPER: u32 = 8192;
pub const DRVM_MAPPER_STATUS: u32 = 8192;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386;
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0;
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1;
pub const FILTERCHOOSE_MESSAGE: u32 = 0;
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0;
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1;
pub const FORMATCHOOSE_MESSAGE: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACMDRIVER(pub *mut core::ffi::c_void);
impl HACMDRIVER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACMDRIVERID(pub *mut core::ffi::c_void);
impl HACMDRIVERID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HACMDRIVERID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACMOBJ(pub *mut core::ffi::c_void);
impl HACMOBJ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HACMOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACMSTREAM(pub *mut core::ffi::c_void);
impl HACMSTREAM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HACMSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMDRIVERDETAILSA(pub *mut ACMDRIVERDETAILSA);
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl LPACMDRIVERDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl Default for LPACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMDRIVERDETAILSW(pub *mut ACMDRIVERDETAILSW);
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl LPACMDRIVERDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl Default for LPACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMDRIVERPROC(pub *mut ACMDRIVERPROC);
#[cfg(feature = "minwindef")]
impl LPACMDRIVERPROC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPACMDRIVERPROC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFILTERCHOOSEA(pub *mut ACMFILTERCHOOSEA);
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl LPACMFILTERCHOOSEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl Default for LPACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFILTERCHOOSEW(pub *mut ACMFILTERCHOOSEW);
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl LPACMFILTERCHOOSEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl Default for LPACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmreg")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFILTERDETAILSA(pub *mut ACMFILTERDETAILSA);
#[cfg(feature = "mmreg")]
impl LPACMFILTERDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmreg")]
impl Default for LPACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmreg")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFILTERDETAILSW(pub *mut ACMFILTERDETAILSW);
#[cfg(feature = "mmreg")]
impl LPACMFILTERDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmreg")]
impl Default for LPACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFILTERTAGDETAILSA(pub *mut ACMFILTERTAGDETAILSA);
impl LPACMFILTERTAGDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFILTERTAGDETAILSW(pub *mut ACMFILTERTAGDETAILSW);
impl LPACMFILTERTAGDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFORMATCHOOSEA(pub *mut ACMFORMATCHOOSEA);
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl LPACMFORMATCHOOSEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl Default for LPACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFORMATCHOOSEW(pub *mut ACMFORMATCHOOSEW);
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl LPACMFORMATCHOOSEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl Default for LPACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFORMATDETAILSA(pub *mut ACMFORMATDETAILSA);
#[cfg(feature = "mmeapi")]
impl LPACMFORMATDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFORMATDETAILSW(pub *mut ACMFORMATDETAILSW);
#[cfg(feature = "mmeapi")]
impl LPACMFORMATDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFORMATTAGDETAILSA(pub *mut ACMFORMATTAGDETAILSA);
impl LPACMFORMATTAGDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMFORMATTAGDETAILSW(pub *mut ACMFORMATTAGDETAILSW);
impl LPACMFORMATTAGDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPACMSTREAMHEADER(pub *mut ACMSTREAMHEADER);
#[cfg(feature = "minwindef")]
impl LPACMSTREAMHEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHACMDRIVER(pub *mut HACMDRIVER);
impl LPHACMDRIVER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHACMDRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHACMDRIVERID(pub *mut HACMDRIVERID);
impl LPHACMDRIVERID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHACMDRIVERID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHACMOBJ(pub *mut HACMOBJ);
impl LPHACMOBJ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHACMOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHACMSTREAM(pub *mut HACMSTREAM);
impl LPHACMSTREAM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPHACMSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MM_ACM_CLOSE: u32 = 981;
pub const MM_ACM_DONE: u32 = 982;
pub const MM_ACM_FILTERCHOOSE: u32 = 32768;
pub const MM_ACM_FORMATCHOOSE: u32 = 32768;
pub const MM_ACM_OPEN: u32 = 980;
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMDRIVERDETAILSA(pub *mut ACMDRIVERDETAILSA);
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl PACMDRIVERDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl Default for PACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMDRIVERDETAILSW(pub *mut ACMDRIVERDETAILSW);
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl PACMDRIVERDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl Default for PACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFILTERCHOOSEA(pub *mut ACMFILTERCHOOSEA);
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl PACMFILTERCHOOSEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl Default for PACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFILTERCHOOSEW(pub *mut ACMFILTERCHOOSEW);
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl PACMFILTERCHOOSEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl Default for PACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmreg")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFILTERDETAILSA(pub *mut ACMFILTERDETAILSA);
#[cfg(feature = "mmreg")]
impl PACMFILTERDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmreg")]
impl Default for PACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmreg")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFILTERDETAILSW(pub *mut ACMFILTERDETAILSW);
#[cfg(feature = "mmreg")]
impl PACMFILTERDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmreg")]
impl Default for PACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFILTERTAGDETAILSA(pub *mut ACMFILTERTAGDETAILSA);
impl PACMFILTERTAGDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFILTERTAGDETAILSW(pub *mut ACMFILTERTAGDETAILSW);
impl PACMFILTERTAGDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFORMATCHOOSEA(pub *mut ACMFORMATCHOOSEA);
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl PACMFORMATCHOOSEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl Default for PACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFORMATCHOOSEW(pub *mut ACMFORMATCHOOSEW);
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl PACMFORMATCHOOSEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl Default for PACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFORMATDETAILSA(pub *mut ACMFORMATDETAILSA);
#[cfg(feature = "mmeapi")]
impl PACMFORMATDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for PACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFORMATDETAILSW(pub *mut ACMFORMATDETAILSW);
#[cfg(feature = "mmeapi")]
impl PACMFORMATDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for PACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFORMATTAGDETAILSA(pub *mut ACMFORMATTAGDETAILSA);
impl PACMFORMATTAGDETAILSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMFORMATTAGDETAILSW(pub *mut ACMFORMATTAGDETAILSW);
impl PACMFORMATTAGDETAILSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACMSTREAMHEADER(pub *mut ACMSTREAMHEADER);
#[cfg(feature = "minwindef")]
impl PACMSTREAMHEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHACMDRIVER(pub *mut HACMDRIVER);
impl PHACMDRIVER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHACMDRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHACMDRIVERID(pub *mut HACMDRIVERID);
impl PHACMDRIVERID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHACMDRIVERID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHACMOBJ(pub *mut HACMOBJ);
impl PHACMOBJ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHACMOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHACMSTREAM(pub *mut HACMSTREAM);
impl PHACMSTREAM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHACMSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0;
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2;
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1;
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0;
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2;
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1;
pub const WIDM_MAPPER_STATUS: u32 = 8192;
pub const WODM_MAPPER_STATUS: u32 = 8192;
