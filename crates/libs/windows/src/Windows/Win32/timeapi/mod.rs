#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn timeBeginPeriod(uperiod: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeBeginPeriod(uperiod : super::UINT) -> super::MMRESULT);
    unsafe { timeBeginPeriod(uperiod) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn timeEndPeriod(uperiod: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeEndPeriod(uperiod : super::UINT) -> super::MMRESULT);
    unsafe { timeEndPeriod(uperiod) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn timeGetDevCaps(ptc: LPTIMECAPS, cbtc: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeGetDevCaps(ptc : LPTIMECAPS, cbtc : super::UINT) -> super::MMRESULT);
    unsafe { timeGetDevCaps(ptc as _, cbtc) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn timeGetSystemTime(pmmt: super::LPMMTIME, cbmmt: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeGetSystemTime(pmmt : super::LPMMTIME, cbmmt : super::UINT) -> super::MMRESULT);
    unsafe { timeGetSystemTime(pmmt as _, cbmmt) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn timeGetTime() -> super::DWORD {
    windows_core::link!("winmm.dll" "system" fn timeGetTime() -> super::DWORD);
    unsafe { timeGetTime() }
}
#[cfg(feature = "shared")]
pub type LPTIMECAPS = *mut TIMECAPS;
#[cfg(feature = "shared")]
pub type NPTIMECAPS = *mut TIMECAPS;
#[cfg(feature = "shared")]
pub type PTIMECAPS = *mut TIMECAPS;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TIMECAPS {
    pub wPeriodMin: super::UINT,
    pub wPeriodMax: super::UINT,
}
pub const TIMERR_NOCANDO: i32 = 97;
pub const TIMERR_NOERROR: i32 = 0;
pub const TIMERR_STRUCT: i32 = 129;
