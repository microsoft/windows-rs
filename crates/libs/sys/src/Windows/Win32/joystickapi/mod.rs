#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn joyConfigChanged(dwflags : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn joyGetDevCapsA(ujoyid : usize, pjc : *mut JOYCAPSA, cbjc : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn joyGetDevCapsW(ujoyid : usize, pjc : *mut JOYCAPSW, cbjc : u32) -> super::MMRESULT);
windows_link::link!("winmm.dll" "system" fn joyGetNumDevs() -> u32);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn joyGetPos(ujoyid : u32, pji : *mut JOYINFO) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn joyGetPosEx(ujoyid : u32, pji : *mut JOYINFOEX) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn joyGetThreshold(ujoyid : u32, puthreshold : *mut u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn joyReleaseCapture(ujoyid : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "windef"))]
windows_link::link!("winmm.dll" "system" fn joySetCapture(hwnd : super::HWND, ujoyid : u32, uperiod : u32, fchanged : windows_sys::core::BOOL) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn joySetThreshold(ujoyid : u32, uthreshold : u32) -> super::MMRESULT);
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
pub const JOYCAPS_HASPOV: i32 = 16;
pub const JOYCAPS_HASR: i32 = 2;
pub const JOYCAPS_HASU: i32 = 4;
pub const JOYCAPS_HASV: i32 = 8;
pub const JOYCAPS_HASZ: i32 = 1;
pub const JOYCAPS_POV4DIR: i32 = 32;
pub const JOYCAPS_POVCTS: i32 = 64;
pub const JOYERR_NOCANDO: i32 = 166;
pub const JOYERR_NOERROR: i32 = 0;
pub const JOYERR_PARMS: i32 = 165;
pub const JOYERR_UNPLUGGED: i32 = 167;
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
pub const JOYSTICKID1: i32 = 0;
pub const JOYSTICKID2: i32 = 1;
pub const JOY_BUTTON1: i32 = 1;
pub const JOY_BUTTON10: i32 = 512;
pub const JOY_BUTTON11: i32 = 1024;
pub const JOY_BUTTON12: i32 = 2048;
pub const JOY_BUTTON13: i32 = 4096;
pub const JOY_BUTTON14: i32 = 8192;
pub const JOY_BUTTON15: i32 = 16384;
pub const JOY_BUTTON16: i32 = 32768;
pub const JOY_BUTTON17: i32 = 65536;
pub const JOY_BUTTON18: i32 = 131072;
pub const JOY_BUTTON19: i32 = 262144;
pub const JOY_BUTTON1CHG: i32 = 256;
pub const JOY_BUTTON2: i32 = 2;
pub const JOY_BUTTON20: i32 = 524288;
pub const JOY_BUTTON21: i32 = 1048576;
pub const JOY_BUTTON22: i32 = 2097152;
pub const JOY_BUTTON23: i32 = 4194304;
pub const JOY_BUTTON24: i32 = 8388608;
pub const JOY_BUTTON25: i32 = 16777216;
pub const JOY_BUTTON26: i32 = 33554432;
pub const JOY_BUTTON27: i32 = 67108864;
pub const JOY_BUTTON28: i32 = 134217728;
pub const JOY_BUTTON29: i32 = 268435456;
pub const JOY_BUTTON2CHG: i32 = 512;
pub const JOY_BUTTON3: i32 = 4;
pub const JOY_BUTTON30: i32 = 536870912;
pub const JOY_BUTTON31: i32 = 1073741824;
pub const JOY_BUTTON32: u32 = 2147483648;
pub const JOY_BUTTON3CHG: i32 = 1024;
pub const JOY_BUTTON4: i32 = 8;
pub const JOY_BUTTON4CHG: i32 = 2048;
pub const JOY_BUTTON5: i32 = 16;
pub const JOY_BUTTON6: i32 = 32;
pub const JOY_BUTTON7: i32 = 64;
pub const JOY_BUTTON8: i32 = 128;
pub const JOY_BUTTON9: i32 = 256;
pub const JOY_CAL_READ3: i32 = 262144;
pub const JOY_CAL_READ4: i32 = 524288;
pub const JOY_CAL_READ5: i32 = 4194304;
pub const JOY_CAL_READ6: i32 = 8388608;
pub const JOY_CAL_READALWAYS: i32 = 65536;
pub const JOY_CAL_READRONLY: i32 = 33554432;
pub const JOY_CAL_READUONLY: i32 = 67108864;
pub const JOY_CAL_READVONLY: i32 = 134217728;
pub const JOY_CAL_READXONLY: i32 = 1048576;
pub const JOY_CAL_READXYONLY: i32 = 131072;
pub const JOY_CAL_READYONLY: i32 = 2097152;
pub const JOY_CAL_READZONLY: i32 = 16777216;
pub const JOY_POVBACKWARD: i32 = 18000;
pub const JOY_POVCENTERED: u16 = 65535;
pub const JOY_POVFORWARD: i32 = 0;
pub const JOY_POVLEFT: i32 = 27000;
pub const JOY_POVRIGHT: i32 = 9000;
pub const JOY_RETURNALL: i32 = 255;
pub const JOY_RETURNBUTTONS: i32 = 128;
pub const JOY_RETURNCENTERED: i32 = 1024;
pub const JOY_RETURNPOV: i32 = 64;
pub const JOY_RETURNPOVCTS: i32 = 512;
pub const JOY_RETURNR: i32 = 8;
pub const JOY_RETURNRAWDATA: i32 = 256;
pub const JOY_RETURNU: i32 = 16;
pub const JOY_RETURNV: i32 = 32;
pub const JOY_RETURNX: i32 = 1;
pub const JOY_RETURNY: i32 = 2;
pub const JOY_RETURNZ: i32 = 4;
pub const JOY_USEDEADZONE: i32 = 2048;
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
