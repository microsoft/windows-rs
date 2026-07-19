#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn timeBeginPeriod(uperiod : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn timeEndPeriod(uperiod : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn timeGetDevCaps(ptc : *mut TIMECAPS, cbtc : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn timeGetSystemTime(pmmt : *mut super::MMTIME, cbmmt : u32) -> super::MMRESULT);
windows_link::link!("winmm.dll" "system" fn timeGetTime() -> u32);
pub type LPTIMECAPS = *mut TIMECAPS;
pub type NPTIMECAPS = *mut TIMECAPS;
pub type PTIMECAPS = *mut TIMECAPS;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct TIMECAPS {
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
}
pub const TIMERR_NOCANDO: u32 = 97;
pub const TIMERR_NOERROR: u32 = 0;
pub const TIMERR_STRUCT: u32 = 129;
