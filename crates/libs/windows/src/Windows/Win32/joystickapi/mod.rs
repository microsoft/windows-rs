#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn joyGetDevCapsA(ujoyid: super::UINT_PTR, pjc: LPJOYCAPSA, cbjc: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetDevCapsA(ujoyid : super::UINT_PTR, pjc : LPJOYCAPSA, cbjc : super::UINT) -> super::MMRESULT);
    unsafe { joyGetDevCapsA(ujoyid, pjc as _, cbjc) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn joyGetDevCapsW(ujoyid: super::UINT_PTR, pjc: LPJOYCAPSW, cbjc: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetDevCapsW(ujoyid : super::UINT_PTR, pjc : LPJOYCAPSW, cbjc : super::UINT) -> super::MMRESULT);
    unsafe { joyGetDevCapsW(ujoyid, pjc as _, cbjc) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn joyGetNumDevs() -> super::UINT {
    windows_core::link!("winmm.dll" "system" fn joyGetNumDevs() -> super::UINT);
    unsafe { joyGetNumDevs() }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn joyGetPos(ujoyid: super::UINT, pji: LPJOYINFO) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetPos(ujoyid : super::UINT, pji : LPJOYINFO) -> super::MMRESULT);
    unsafe { joyGetPos(ujoyid, pji as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn joyGetPosEx(ujoyid: super::UINT, pji: LPJOYINFOEX) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetPosEx(ujoyid : super::UINT, pji : LPJOYINFOEX) -> super::MMRESULT);
    unsafe { joyGetPosEx(ujoyid, pji as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn joyGetThreshold(ujoyid: super::UINT, puthreshold: super::LPUINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyGetThreshold(ujoyid : super::UINT, puthreshold : super::LPUINT) -> super::MMRESULT);
    unsafe { joyGetThreshold(ujoyid, puthreshold as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn joyReleaseCapture(ujoyid: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyReleaseCapture(ujoyid : super::UINT) -> super::MMRESULT);
    unsafe { joyReleaseCapture(ujoyid) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn joySetCapture(hwnd: super::HWND, ujoyid: super::UINT, uperiod: super::UINT, fchanged: bool) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joySetCapture(hwnd : super::HWND, ujoyid : super::UINT, uperiod : super::UINT, fchanged : windows_core::BOOL) -> super::MMRESULT);
    unsafe { joySetCapture(hwnd, ujoyid, uperiod, fchanged.into()) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn joySetThreshold(ujoyid: super::UINT, uthreshold: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joySetThreshold(ujoyid : super::UINT, uthreshold : super::UINT) -> super::MMRESULT);
    unsafe { joySetThreshold(ujoyid, uthreshold) }
}
#[cfg(feature = "shared")]
pub type JOYCAPS = JOYCAPSA;
#[cfg(feature = "shared")]
pub type JOYCAPS2 = JOYCAPS2A;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct JOYCAPS2A {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub szPname: [super::CHAR; 32],
    pub wXmin: super::UINT,
    pub wXmax: super::UINT,
    pub wYmin: super::UINT,
    pub wYmax: super::UINT,
    pub wZmin: super::UINT,
    pub wZmax: super::UINT,
    pub wNumButtons: super::UINT,
    pub wPeriodMin: super::UINT,
    pub wPeriodMax: super::UINT,
    pub wRmin: super::UINT,
    pub wRmax: super::UINT,
    pub wUmin: super::UINT,
    pub wUmax: super::UINT,
    pub wVmin: super::UINT,
    pub wVmax: super::UINT,
    pub wCaps: super::UINT,
    pub wMaxAxes: super::UINT,
    pub wNumAxes: super::UINT,
    pub wMaxButtons: super::UINT,
    pub szRegKey: [super::CHAR; 32],
    pub szOEMVxD: [super::CHAR; 260],
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for JOYCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct JOYCAPS2W {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub szPname: [super::WCHAR; 32],
    pub wXmin: super::UINT,
    pub wXmax: super::UINT,
    pub wYmin: super::UINT,
    pub wYmax: super::UINT,
    pub wZmin: super::UINT,
    pub wZmax: super::UINT,
    pub wNumButtons: super::UINT,
    pub wPeriodMin: super::UINT,
    pub wPeriodMax: super::UINT,
    pub wRmin: super::UINT,
    pub wRmax: super::UINT,
    pub wUmin: super::UINT,
    pub wUmax: super::UINT,
    pub wVmin: super::UINT,
    pub wVmax: super::UINT,
    pub wCaps: super::UINT,
    pub wMaxAxes: super::UINT,
    pub wNumAxes: super::UINT,
    pub wMaxButtons: super::UINT,
    pub szRegKey: [super::WCHAR; 32],
    pub szOEMVxD: [super::WCHAR; 260],
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for JOYCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct JOYCAPSA {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub szPname: [super::CHAR; 32],
    pub wXmin: super::UINT,
    pub wXmax: super::UINT,
    pub wYmin: super::UINT,
    pub wYmax: super::UINT,
    pub wZmin: super::UINT,
    pub wZmax: super::UINT,
    pub wNumButtons: super::UINT,
    pub wPeriodMin: super::UINT,
    pub wPeriodMax: super::UINT,
    pub wRmin: super::UINT,
    pub wRmax: super::UINT,
    pub wUmin: super::UINT,
    pub wUmax: super::UINT,
    pub wVmin: super::UINT,
    pub wVmax: super::UINT,
    pub wCaps: super::UINT,
    pub wMaxAxes: super::UINT,
    pub wNumAxes: super::UINT,
    pub wMaxButtons: super::UINT,
    pub szRegKey: [super::CHAR; 32],
    pub szOEMVxD: [super::CHAR; 260],
}
#[cfg(feature = "shared")]
impl Default for JOYCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct JOYCAPSW {
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub szPname: [super::WCHAR; 32],
    pub wXmin: super::UINT,
    pub wXmax: super::UINT,
    pub wYmin: super::UINT,
    pub wYmax: super::UINT,
    pub wZmin: super::UINT,
    pub wZmax: super::UINT,
    pub wNumButtons: super::UINT,
    pub wPeriodMin: super::UINT,
    pub wPeriodMax: super::UINT,
    pub wRmin: super::UINT,
    pub wRmax: super::UINT,
    pub wUmin: super::UINT,
    pub wUmax: super::UINT,
    pub wVmin: super::UINT,
    pub wVmax: super::UINT,
    pub wCaps: super::UINT,
    pub wMaxAxes: super::UINT,
    pub wNumAxes: super::UINT,
    pub wMaxButtons: super::UINT,
    pub szRegKey: [super::WCHAR; 32],
    pub szOEMVxD: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct JOYINFO {
    pub wXpos: super::UINT,
    pub wYpos: super::UINT,
    pub wZpos: super::UINT,
    pub wButtons: super::UINT,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct JOYINFOEX {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwXpos: super::DWORD,
    pub dwYpos: super::DWORD,
    pub dwZpos: super::DWORD,
    pub dwRpos: super::DWORD,
    pub dwUpos: super::DWORD,
    pub dwVpos: super::DWORD,
    pub dwButtons: super::DWORD,
    pub dwButtonNumber: super::DWORD,
    pub dwPOV: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
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
#[cfg(feature = "shared")]
pub const JOY_POVCENTERED: super::WORD = 65535;
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
#[cfg(feature = "shared")]
pub type LPJOYCAPS = *mut JOYCAPSA;
#[cfg(feature = "shared")]
pub type LPJOYCAPS2 = *mut JOYCAPS2A;
#[cfg(feature = "shared")]
pub type LPJOYCAPS2A = *mut JOYCAPS2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPJOYCAPS2W = *mut JOYCAPS2W;
#[cfg(feature = "shared")]
pub type LPJOYCAPSA = *mut JOYCAPSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPJOYCAPSW = *mut JOYCAPSW;
#[cfg(feature = "shared")]
pub type LPJOYINFO = *mut JOYINFO;
#[cfg(feature = "shared")]
pub type LPJOYINFOEX = *mut JOYINFOEX;
#[cfg(feature = "shared")]
pub type NPJOYCAPS = *mut JOYCAPSA;
#[cfg(feature = "shared")]
pub type NPJOYCAPS2 = *mut JOYCAPS2A;
#[cfg(feature = "shared")]
pub type NPJOYCAPS2A = *mut JOYCAPS2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type NPJOYCAPS2W = *mut JOYCAPS2W;
#[cfg(feature = "shared")]
pub type NPJOYCAPSA = *mut JOYCAPSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type NPJOYCAPSW = *mut JOYCAPSW;
#[cfg(feature = "shared")]
pub type NPJOYINFO = *mut JOYINFO;
#[cfg(feature = "shared")]
pub type NPJOYINFOEX = *mut JOYINFOEX;
#[cfg(feature = "shared")]
pub type PJOYCAPS = *mut JOYCAPSA;
#[cfg(feature = "shared")]
pub type PJOYCAPS2 = *mut JOYCAPS2A;
#[cfg(feature = "shared")]
pub type PJOYCAPS2A = *mut JOYCAPS2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PJOYCAPS2W = *mut JOYCAPS2W;
#[cfg(feature = "shared")]
pub type PJOYCAPSA = *mut JOYCAPSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PJOYCAPSW = *mut JOYCAPSW;
#[cfg(feature = "shared")]
pub type PJOYINFO = *mut JOYINFO;
#[cfg(feature = "shared")]
pub type PJOYINFOEX = *mut JOYINFOEX;
