#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn timeKillEvent(utimerid: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeKillEvent(utimerid : super::UINT) -> super::MMRESULT);
    unsafe { timeKillEvent(utimerid) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn timeSetEvent(udelay: super::UINT, uresolution: super::UINT, fptc: LPTIMECALLBACK, dwuser: super::DWORD_PTR, fuevent: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeSetEvent(udelay : super::UINT, uresolution : super::UINT, fptc : LPTIMECALLBACK, dwuser : super::DWORD_PTR, fuevent : super::UINT) -> super::MMRESULT);
    unsafe { timeSetEvent(udelay, uresolution, fptc, dwuser, fuevent) }
}
#[cfg(feature = "shared")]
pub type LPTIMECALLBACK = Option<unsafe extern "system" fn(utimerid: super::UINT, umsg: super::UINT, dwuser: super::DWORD_PTR, dw1: super::DWORD_PTR, dw2: super::DWORD_PTR)>;
#[cfg(feature = "shared")]
pub type TIMECALLBACK = Option<unsafe extern "system" fn(utimerid: super::UINT, umsg: super::UINT, dwuser: super::DWORD_PTR, dw1: super::DWORD_PTR, dw2: super::DWORD_PTR)>;
pub const TIME_CALLBACK_EVENT_PULSE: i32 = 32;
pub const TIME_CALLBACK_EVENT_SET: i32 = 16;
pub const TIME_CALLBACK_FUNCTION: i32 = 0;
pub const TIME_KILL_SYNCHRONOUS: i32 = 256;
pub const TIME_ONESHOT: i32 = 0;
pub const TIME_PERIODIC: i32 = 1;
