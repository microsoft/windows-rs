#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn timeBeginPeriod(uperiod: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeBeginPeriod(uperiod : u32) -> super::mmsyscom::MMRESULT);
    unsafe { timeBeginPeriod(uperiod) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn timeEndPeriod(uperiod: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeEndPeriod(uperiod : u32) -> super::mmsyscom::MMRESULT);
    unsafe { timeEndPeriod(uperiod) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn timeGetDevCaps(ptc: *mut TIMECAPS, cbtc: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeGetDevCaps(ptc : *mut TIMECAPS, cbtc : u32) -> super::mmsyscom::MMRESULT);
    unsafe { timeGetDevCaps(ptc as _, cbtc) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn timeGetSystemTime(pmmt: *mut super::mmsyscom::MMTIME, cbmmt: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeGetSystemTime(pmmt : *mut super::mmsyscom::MMTIME, cbmmt : u32) -> super::mmsyscom::MMRESULT);
    unsafe { timeGetSystemTime(pmmt as _, cbmmt) }
}
#[inline]
pub unsafe fn timeGetTime() -> u32 {
    windows_core::link!("winmm.dll" "system" fn timeGetTime() -> u32);
    unsafe { timeGetTime() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTIMECAPS(pub *mut TIMECAPS);
impl LPTIMECAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTIMECAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPTIMECAPS(pub *mut TIMECAPS);
impl NPTIMECAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPTIMECAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTIMECAPS(pub *mut TIMECAPS);
impl PTIMECAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTIMECAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct TIMECAPS {
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
}
pub const TIMERR_NOCANDO: u32 = 97;
pub const TIMERR_NOERROR: u32 = 0;
pub const TIMERR_STRUCT: u32 = 129;
