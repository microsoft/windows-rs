#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn timeKillEvent(utimerid: u32) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeKillEvent(utimerid : u32) -> super::MMRESULT);
    unsafe { timeKillEvent(utimerid) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn timeSetEvent(udelay: u32, uresolution: u32, fptc: LPTIMECALLBACK, dwuser: usize, fuevent: u32) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn timeSetEvent(udelay : u32, uresolution : u32, fptc : LPTIMECALLBACK, dwuser : usize, fuevent : u32) -> super::MMRESULT);
    unsafe { timeSetEvent(udelay, uresolution, fptc, dwuser, fuevent) }
}
pub type LPTIMECALLBACK = Option<unsafe extern "system" fn(utimerid: u32, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub type TIMECALLBACK = Option<unsafe extern "system" fn(utimerid: u32, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub const TIME_CALLBACK_EVENT_PULSE: i32 = 32;
pub const TIME_CALLBACK_EVENT_SET: i32 = 16;
pub const TIME_CALLBACK_FUNCTION: i32 = 0;
pub const TIME_KILL_SYNCHRONOUS: i32 = 256;
pub const TIME_ONESHOT: i32 = 0;
pub const TIME_PERIODIC: i32 = 1;
