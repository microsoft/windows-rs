#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joyConfigChanged(dwflags: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyConfigChanged(dwflags : u32) -> super::mmsyscom::MMRESULT);
    unsafe { joyConfigChanged(dwflags) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joyGetDevCapsA(ujoyid: usize, pjc: *mut JOYCAPSA, cbjc: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetDevCapsA(ujoyid : usize, pjc : *mut JOYCAPSA, cbjc : u32) -> super::mmsyscom::MMRESULT);
    unsafe { joyGetDevCapsA(ujoyid, pjc as _, cbjc) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joyGetDevCapsW(ujoyid: usize, pjc: *mut JOYCAPSW, cbjc: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetDevCapsW(ujoyid : usize, pjc : *mut JOYCAPSW, cbjc : u32) -> super::mmsyscom::MMRESULT);
    unsafe { joyGetDevCapsW(ujoyid, pjc as _, cbjc) }
}
#[inline]
pub unsafe fn joyGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn joyGetNumDevs() -> u32);
    unsafe { joyGetNumDevs() }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joyGetPos(ujoyid: u32, pji: *mut JOYINFO) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetPos(ujoyid : u32, pji : *mut JOYINFO) -> super::mmsyscom::MMRESULT);
    unsafe { joyGetPos(ujoyid, pji as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joyGetPosEx(ujoyid: u32, pji: *mut JOYINFOEX) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetPosEx(ujoyid : u32, pji : *mut JOYINFOEX) -> super::mmsyscom::MMRESULT);
    unsafe { joyGetPosEx(ujoyid, pji as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joyGetThreshold(ujoyid: u32, puthreshold: *mut u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetThreshold(ujoyid : u32, puthreshold : *mut u32) -> super::mmsyscom::MMRESULT);
    unsafe { joyGetThreshold(ujoyid, puthreshold as _) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joyReleaseCapture(ujoyid: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyReleaseCapture(ujoyid : u32) -> super::mmsyscom::MMRESULT);
    unsafe { joyReleaseCapture(ujoyid) }
}
#[cfg(all(feature = "mmsyscom", feature = "windef"))]
#[inline]
pub unsafe fn joySetCapture(hwnd: super::windef::HWND, ujoyid: u32, uperiod: u32, fchanged: bool) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joySetCapture(hwnd : super::windef::HWND, ujoyid : u32, uperiod : u32, fchanged : windows_core::BOOL) -> super::mmsyscom::MMRESULT);
    unsafe { joySetCapture(hwnd, ujoyid, uperiod, fchanged.into()) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joySetThreshold(ujoyid: u32, uthreshold: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joySetThreshold(ujoyid : u32, uthreshold : u32) -> super::mmsyscom::MMRESULT);
    unsafe { joySetThreshold(ujoyid, uthreshold) }
}
pub type JOYCAPS = JOYCAPSA;
pub type JOYCAPS2 = JOYCAPS2A;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [i8; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [i8; 32],
    pub szOEMVxD: [i8; 260],
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for JOYCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for JOYCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [i8; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [i8; 32],
    pub szOEMVxD: [i8; 260],
}
impl Default for JOYCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JOYCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
}
impl Default for JOYCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JOYCAPS_HASPOV: u32 = 16;
pub const JOYCAPS_HASR: u32 = 2;
pub const JOYCAPS_HASU: u32 = 4;
pub const JOYCAPS_HASV: u32 = 8;
pub const JOYCAPS_HASZ: u32 = 1;
pub const JOYCAPS_POV4DIR: u32 = 32;
pub const JOYCAPS_POVCTS: u32 = 64;
pub const JOYERR_NOCANDO: u32 = 166;
pub const JOYERR_NOERROR: u32 = 0;
pub const JOYERR_PARMS: u32 = 165;
pub const JOYERR_UNPLUGGED: u32 = 167;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct JOYINFO {
    pub wXpos: u32,
    pub wYpos: u32,
    pub wZpos: u32,
    pub wButtons: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct JOYINFOEX {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwXpos: u32,
    pub dwYpos: u32,
    pub dwZpos: u32,
    pub dwRpos: u32,
    pub dwUpos: u32,
    pub dwVpos: u32,
    pub dwButtons: u32,
    pub dwButtonNumber: u32,
    pub dwPOV: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
pub const JOYSTICKID1: u32 = 0;
pub const JOYSTICKID2: u32 = 1;
pub const JOY_BUTTON1: u32 = 1;
pub const JOY_BUTTON10: u32 = 512;
pub const JOY_BUTTON11: u32 = 1024;
pub const JOY_BUTTON12: u32 = 2048;
pub const JOY_BUTTON13: u32 = 4096;
pub const JOY_BUTTON14: u32 = 8192;
pub const JOY_BUTTON15: u32 = 16384;
pub const JOY_BUTTON16: u32 = 32768;
pub const JOY_BUTTON17: u32 = 65536;
pub const JOY_BUTTON18: u32 = 131072;
pub const JOY_BUTTON19: u32 = 262144;
pub const JOY_BUTTON1CHG: u32 = 256;
pub const JOY_BUTTON2: u32 = 2;
pub const JOY_BUTTON20: u32 = 524288;
pub const JOY_BUTTON21: u32 = 1048576;
pub const JOY_BUTTON22: u32 = 2097152;
pub const JOY_BUTTON23: u32 = 4194304;
pub const JOY_BUTTON24: u32 = 8388608;
pub const JOY_BUTTON25: u32 = 16777216;
pub const JOY_BUTTON26: u32 = 33554432;
pub const JOY_BUTTON27: u32 = 67108864;
pub const JOY_BUTTON28: u32 = 134217728;
pub const JOY_BUTTON29: u32 = 268435456;
pub const JOY_BUTTON2CHG: u32 = 512;
pub const JOY_BUTTON3: u32 = 4;
pub const JOY_BUTTON30: u32 = 536870912;
pub const JOY_BUTTON31: u32 = 1073741824;
pub const JOY_BUTTON32: u32 = 2147483648;
pub const JOY_BUTTON3CHG: u32 = 1024;
pub const JOY_BUTTON4: u32 = 8;
pub const JOY_BUTTON4CHG: u32 = 2048;
pub const JOY_BUTTON5: u32 = 16;
pub const JOY_BUTTON6: u32 = 32;
pub const JOY_BUTTON7: u32 = 64;
pub const JOY_BUTTON8: u32 = 128;
pub const JOY_BUTTON9: u32 = 256;
pub const JOY_CAL_READ3: u32 = 262144;
pub const JOY_CAL_READ4: u32 = 524288;
pub const JOY_CAL_READ5: u32 = 4194304;
pub const JOY_CAL_READ6: u32 = 8388608;
pub const JOY_CAL_READALWAYS: u32 = 65536;
pub const JOY_CAL_READRONLY: u32 = 33554432;
pub const JOY_CAL_READUONLY: u32 = 67108864;
pub const JOY_CAL_READVONLY: u32 = 134217728;
pub const JOY_CAL_READXONLY: u32 = 1048576;
pub const JOY_CAL_READXYONLY: u32 = 131072;
pub const JOY_CAL_READYONLY: u32 = 2097152;
pub const JOY_CAL_READZONLY: u32 = 16777216;
pub const JOY_POVBACKWARD: u32 = 18000;
pub const JOY_POVCENTERED: u16 = 65535;
pub const JOY_POVFORWARD: u32 = 0;
pub const JOY_POVLEFT: u32 = 27000;
pub const JOY_POVRIGHT: u32 = 9000;
pub const JOY_RETURNALL: u32 = 255;
pub const JOY_RETURNBUTTONS: u32 = 128;
pub const JOY_RETURNCENTERED: u32 = 1024;
pub const JOY_RETURNPOV: u32 = 64;
pub const JOY_RETURNPOVCTS: u32 = 512;
pub const JOY_RETURNR: u32 = 8;
pub const JOY_RETURNRAWDATA: u32 = 256;
pub const JOY_RETURNU: u32 = 16;
pub const JOY_RETURNV: u32 = 32;
pub const JOY_RETURNX: u32 = 1;
pub const JOY_RETURNY: u32 = 2;
pub const JOY_RETURNZ: u32 = 4;
pub const JOY_USEDEADZONE: u32 = 2048;
pub type LPJOYCAPS = LPJOYCAPSA;
pub type LPJOYCAPS2 = LPJOYCAPS2A;
pub type LPJOYCAPS2A = *mut JOYCAPS2A;
pub type LPJOYCAPS2W = *mut JOYCAPS2W;
pub type LPJOYCAPSA = *mut JOYCAPSA;
pub type LPJOYCAPSW = *mut JOYCAPSW;
pub type LPJOYINFO = *mut JOYINFO;
pub type LPJOYINFOEX = *mut JOYINFOEX;
pub type NPJOYCAPS = NPJOYCAPSA;
pub type NPJOYCAPS2 = NPJOYCAPS2A;
pub type NPJOYCAPS2A = *mut JOYCAPS2A;
pub type NPJOYCAPS2W = *mut JOYCAPS2W;
pub type NPJOYCAPSA = *mut JOYCAPSA;
pub type NPJOYCAPSW = *mut JOYCAPSW;
pub type NPJOYINFO = *mut JOYINFO;
pub type NPJOYINFOEX = *mut JOYINFOEX;
pub type PJOYCAPS = PJOYCAPSA;
pub type PJOYCAPS2 = PJOYCAPS2A;
pub type PJOYCAPS2A = *mut JOYCAPS2A;
pub type PJOYCAPS2W = *mut JOYCAPS2W;
pub type PJOYCAPSA = *mut JOYCAPSA;
pub type PJOYCAPSW = *mut JOYCAPSW;
pub type PJOYINFO = *mut JOYINFO;
pub type PJOYINFOEX = *mut JOYINFOEX;
