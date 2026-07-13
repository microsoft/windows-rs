#[cfg(feature = "winnt")]
windows_link::link!("winmm.dll" "system" fn mciDriverNotify(hwndcallback : super::winnt::HANDLE, wdeviceid : MCIDEVICEID, ustatus : u32) -> windows_sys::core::BOOL);
windows_link::link!("winmm.dll" "system" fn mciDriverYield(wdeviceid : MCIDEVICEID) -> u32);
windows_link::link!("winmm.dll" "system" fn mciFreeCommandResource(wtable : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("winmm.dll" "system" fn mciGetCreatorTask(mciid : MCIDEVICEID) -> super::minwindef::HTASK);
windows_link::link!("winmm.dll" "system" fn mciGetDeviceIDA(pszdevice : windows_sys::core::PCSTR) -> MCIDEVICEID);
windows_link::link!("winmm.dll" "system" fn mciGetDeviceIDFromElementIDA(dwelementid : u32, lpstrtype : windows_sys::core::PCSTR) -> MCIDEVICEID);
windows_link::link!("winmm.dll" "system" fn mciGetDeviceIDFromElementIDW(dwelementid : u32, lpstrtype : windows_sys::core::PCWSTR) -> MCIDEVICEID);
windows_link::link!("winmm.dll" "system" fn mciGetDeviceIDW(pszdevice : windows_sys::core::PCWSTR) -> MCIDEVICEID);
windows_link::link!("winmm.dll" "system" fn mciGetDriverData(wdeviceid : MCIDEVICEID) -> usize);
windows_link::link!("winmm.dll" "system" fn mciGetErrorStringA(mcierr : MCIERROR, psztext : windows_sys::core::PSTR, cchtext : u32) -> windows_sys::core::BOOL);
windows_link::link!("winmm.dll" "system" fn mciGetErrorStringW(mcierr : MCIERROR, psztext : windows_sys::core::PWSTR, cchtext : u32) -> windows_sys::core::BOOL);
windows_link::link!("winmm.dll" "system" fn mciGetYieldProc(mciid : MCIDEVICEID, pdwyielddata : *const u32) -> YIELDPROC);
#[cfg(feature = "winnt")]
windows_link::link!("winmm.dll" "system" fn mciLoadCommandResource(hinstance : super::winnt::HANDLE, lpresname : windows_sys::core::PCWSTR, wtype : u32) -> u32);
windows_link::link!("winmm.dll" "system" fn mciSendCommandA(mciid : MCIDEVICEID, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> MCIERROR);
windows_link::link!("winmm.dll" "system" fn mciSendCommandW(mciid : MCIDEVICEID, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> MCIERROR);
#[cfg(feature = "windef")]
windows_link::link!("winmm.dll" "system" fn mciSendStringA(lpstrcommand : windows_sys::core::PCSTR, lpstrreturnstring : windows_sys::core::PSTR, ureturnlength : u32, hwndcallback : super::windef::HWND) -> MCIERROR);
#[cfg(feature = "windef")]
windows_link::link!("winmm.dll" "system" fn mciSendStringW(lpstrcommand : windows_sys::core::PCWSTR, lpstrreturnstring : windows_sys::core::PWSTR, ureturnlength : u32, hwndcallback : super::windef::HWND) -> MCIERROR);
windows_link::link!("winmm.dll" "system" fn mciSetDriverData(wdeviceid : MCIDEVICEID, dwdata : usize) -> windows_sys::core::BOOL);
windows_link::link!("winmm.dll" "system" fn mciSetYieldProc(mciid : MCIDEVICEID, fpyieldproc : YIELDPROC, dwyielddata : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
pub type LPMCI_ANIM_OPEN_PARMS = LPMCI_ANIM_OPEN_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_ANIM_OPEN_PARMSA = *mut MCI_ANIM_OPEN_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_ANIM_OPEN_PARMSW = *mut MCI_ANIM_OPEN_PARMSW;
pub type LPMCI_ANIM_PLAY_PARMS = *mut MCI_ANIM_PLAY_PARMS;
#[cfg(feature = "windef")]
pub type LPMCI_ANIM_RECT_PARMS = *mut MCI_ANIM_RECT_PARMS;
pub type LPMCI_ANIM_STEP_PARMS = *mut MCI_ANIM_STEP_PARMS;
#[cfg(feature = "windef")]
pub type LPMCI_ANIM_UPDATE_PARMS = *mut MCI_ANIM_UPDATE_PARMS;
#[cfg(feature = "windef")]
pub type LPMCI_ANIM_WINDOW_PARMS = LPMCI_ANIM_WINDOW_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_ANIM_WINDOW_PARMSA = *mut MCI_ANIM_WINDOW_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_ANIM_WINDOW_PARMSW = *mut MCI_ANIM_WINDOW_PARMSW;
#[cfg(feature = "windef")]
pub type LPMCI_BREAK_PARMS = *mut MCI_BREAK_PARMS;
pub type LPMCI_GENERIC_PARMS = *mut MCI_GENERIC_PARMS;
pub type LPMCI_GETDEVCAPS_PARMS = *mut MCI_GETDEVCAPS_PARMS;
pub type LPMCI_INFO_PARMS = LPMCI_INFO_PARMSA;
pub type LPMCI_INFO_PARMSA = *mut MCI_INFO_PARMSA;
pub type LPMCI_INFO_PARMSW = *mut MCI_INFO_PARMSW;
pub type LPMCI_LOAD_PARMS = LPMCI_LOAD_PARMSA;
pub type LPMCI_LOAD_PARMSA = *mut MCI_LOAD_PARMSA;
pub type LPMCI_LOAD_PARMSW = *mut MCI_LOAD_PARMSW;
pub type LPMCI_OPEN_PARMS = LPMCI_OPEN_PARMSA;
pub type LPMCI_OPEN_PARMSA = *mut MCI_OPEN_PARMSA;
pub type LPMCI_OPEN_PARMSW = *mut MCI_OPEN_PARMSW;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_LOAD_PARMS = LPMCI_OVLY_LOAD_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_LOAD_PARMSA = *mut MCI_OVLY_LOAD_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_LOAD_PARMSW = *mut MCI_OVLY_LOAD_PARMSW;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_OPEN_PARMS = LPMCI_OVLY_OPEN_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_OPEN_PARMSA = *mut MCI_OVLY_OPEN_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_OPEN_PARMSW = *mut MCI_OVLY_OPEN_PARMSW;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_RECT_PARMS = *mut MCI_OVLY_RECT_PARMS;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_SAVE_PARMS = LPMCI_OVLY_SAVE_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_SAVE_PARMSA = *mut MCI_OVLY_SAVE_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_SAVE_PARMSW = *mut MCI_OVLY_SAVE_PARMSW;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_WINDOW_PARMS = LPMCI_OVLY_WINDOW_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_WINDOW_PARMSA = *mut MCI_OVLY_WINDOW_PARMSA;
#[cfg(feature = "windef")]
pub type LPMCI_OVLY_WINDOW_PARMSW = *mut MCI_OVLY_WINDOW_PARMSW;
pub type LPMCI_PLAY_PARMS = *mut MCI_PLAY_PARMS;
pub type LPMCI_RECORD_PARMS = *mut MCI_RECORD_PARMS;
pub type LPMCI_SAVE_PARMS = LPMCI_SAVE_PARMSA;
pub type LPMCI_SAVE_PARMSA = *mut MCI_SAVE_PARMSA;
pub type LPMCI_SAVE_PARMSW = *mut MCI_SAVE_PARMSW;
pub type LPMCI_SEEK_PARMS = *mut MCI_SEEK_PARMS;
pub type LPMCI_SEQ_SET_PARMS = *mut MCI_SEQ_SET_PARMS;
pub type LPMCI_SET_PARMS = *mut MCI_SET_PARMS;
pub type LPMCI_STATUS_PARMS = *mut MCI_STATUS_PARMS;
pub type LPMCI_SYSINFO_PARMS = LPMCI_SYSINFO_PARMSA;
pub type LPMCI_SYSINFO_PARMSA = *mut MCI_SYSINFO_PARMSA;
pub type LPMCI_SYSINFO_PARMSW = *mut MCI_SYSINFO_PARMSW;
pub type LPMCI_VD_ESCAPE_PARMS = LPMCI_VD_ESCAPE_PARMSA;
pub type LPMCI_VD_ESCAPE_PARMSA = *mut MCI_VD_ESCAPE_PARMSA;
pub type LPMCI_VD_ESCAPE_PARMSW = *mut MCI_VD_ESCAPE_PARMSW;
pub type LPMCI_VD_PLAY_PARMS = *mut MCI_VD_PLAY_PARMS;
pub type LPMCI_VD_STEP_PARMS = *mut MCI_VD_STEP_PARMS;
pub type LPMCI_WAVE_DELETE_PARMS = *mut MCI_WAVE_DELETE_PARMS;
pub type LPMCI_WAVE_OPEN_PARMS = LPMCI_WAVE_OPEN_PARMSA;
pub type LPMCI_WAVE_OPEN_PARMSA = *mut MCI_WAVE_OPEN_PARMSA;
pub type LPMCI_WAVE_OPEN_PARMSW = *mut MCI_WAVE_OPEN_PARMSW;
pub type LPMCI_WAVE_SET_PARMS = *mut MCI_WAVE_SET_PARMS;
pub type MCIDEVICEID = u32;
pub type MCIERROR = u32;
pub const MCIERR_BAD_CONSTANT: u32 = 290;
pub const MCIERR_BAD_INTEGER: u32 = 270;
pub const MCIERR_BAD_TIME_FORMAT: u32 = 293;
pub const MCIERR_CANNOT_LOAD_DRIVER: u32 = 266;
pub const MCIERR_CANNOT_USE_ALL: u32 = 279;
pub const MCIERR_CREATEWINDOW: u32 = 347;
pub const MCIERR_CUSTOM_DRIVER_BASE: u32 = 512;
pub const MCIERR_DEVICE_LENGTH: u32 = 310;
pub const MCIERR_DEVICE_LOCKED: u32 = 288;
pub const MCIERR_DEVICE_NOT_INSTALLED: u32 = 306;
pub const MCIERR_DEVICE_NOT_READY: u32 = 276;
pub const MCIERR_DEVICE_OPEN: u32 = 265;
pub const MCIERR_DEVICE_ORD_LENGTH: u32 = 311;
pub const MCIERR_DEVICE_TYPE_REQUIRED: u32 = 287;
pub const MCIERR_DRIVER: u32 = 278;
pub const MCIERR_DRIVER_INTERNAL: u32 = 272;
pub const MCIERR_DUPLICATE_ALIAS: u32 = 289;
pub const MCIERR_DUPLICATE_FLAGS: u32 = 295;
pub const MCIERR_EXTENSION_NOT_FOUND: u32 = 281;
pub const MCIERR_EXTRA_CHARACTERS: u32 = 305;
pub const MCIERR_FILENAME_REQUIRED: u32 = 304;
pub const MCIERR_FILE_NOT_FOUND: u32 = 275;
pub const MCIERR_FILE_NOT_SAVED: u32 = 286;
pub const MCIERR_FILE_READ: u32 = 348;
pub const MCIERR_FILE_WRITE: u32 = 349;
pub const MCIERR_FLAGS_NOT_COMPATIBLE: u32 = 284;
pub const MCIERR_GET_CD: u32 = 307;
pub const MCIERR_HARDWARE: u32 = 262;
pub const MCIERR_ILLEGAL_FOR_AUTO_OPEN: u32 = 303;
pub const MCIERR_INTERNAL: u32 = 277;
pub const MCIERR_INVALID_DEVICE_ID: u32 = 257;
pub const MCIERR_INVALID_DEVICE_NAME: u32 = 263;
pub const MCIERR_INVALID_FILE: u32 = 296;
pub const MCIERR_MISSING_COMMAND_STRING: u32 = 267;
pub const MCIERR_MISSING_DEVICE_NAME: u32 = 292;
pub const MCIERR_MISSING_PARAMETER: u32 = 273;
pub const MCIERR_MISSING_STRING_ARGUMENT: u32 = 269;
pub const MCIERR_MULTIPLE: u32 = 280;
pub const MCIERR_MUST_USE_SHAREABLE: u32 = 291;
pub const MCIERR_NEW_REQUIRES_ALIAS: u32 = 299;
pub const MCIERR_NONAPPLICABLE_FUNCTION: u32 = 302;
pub const MCIERR_NOTIFY_ON_AUTO_OPEN: u32 = 300;
pub const MCIERR_NO_CLOSING_QUOTE: u32 = 294;
pub const MCIERR_NO_ELEMENT_ALLOWED: u32 = 301;
pub const MCIERR_NO_IDENTITY: u32 = 350;
pub const MCIERR_NO_INTEGER: u32 = 312;
pub const MCIERR_NO_WINDOW: u32 = 346;
pub const MCIERR_NULL_PARAMETER_BLOCK: u32 = 297;
pub const MCIERR_OUTOFRANGE: u32 = 282;
pub const MCIERR_OUT_OF_MEMORY: u32 = 264;
pub const MCIERR_PARAM_OVERFLOW: u32 = 268;
pub const MCIERR_PARSER_INTERNAL: u32 = 271;
pub const MCIERR_SEQ_DIV_INCOMPATIBLE: u32 = 336;
pub const MCIERR_SEQ_NOMIDIPRESENT: u32 = 343;
pub const MCIERR_SEQ_PORTUNSPECIFIED: u32 = 342;
pub const MCIERR_SEQ_PORT_INUSE: u32 = 337;
pub const MCIERR_SEQ_PORT_MAPNODEVICE: u32 = 339;
pub const MCIERR_SEQ_PORT_MISCERROR: u32 = 340;
pub const MCIERR_SEQ_PORT_NONEXISTENT: u32 = 338;
pub const MCIERR_SEQ_TIMER: u32 = 341;
pub const MCIERR_SET_CD: u32 = 308;
pub const MCIERR_SET_DRIVE: u32 = 309;
pub const MCIERR_UNNAMED_RESOURCE: u32 = 298;
pub const MCIERR_UNRECOGNIZED_COMMAND: u32 = 261;
pub const MCIERR_UNRECOGNIZED_KEYWORD: u32 = 259;
pub const MCIERR_UNSUPPORTED_FUNCTION: u32 = 274;
pub const MCIERR_WAVE_INPUTSINUSE: u32 = 322;
pub const MCIERR_WAVE_INPUTSUNSUITABLE: u32 = 328;
pub const MCIERR_WAVE_INPUTUNSPECIFIED: u32 = 325;
pub const MCIERR_WAVE_OUTPUTSINUSE: u32 = 320;
pub const MCIERR_WAVE_OUTPUTSUNSUITABLE: u32 = 326;
pub const MCIERR_WAVE_OUTPUTUNSPECIFIED: u32 = 324;
pub const MCIERR_WAVE_SETINPUTINUSE: u32 = 323;
pub const MCIERR_WAVE_SETINPUTUNSUITABLE: u32 = 329;
pub const MCIERR_WAVE_SETOUTPUTINUSE: u32 = 321;
pub const MCIERR_WAVE_SETOUTPUTUNSUITABLE: u32 = 327;
pub const MCI_ALL_DEVICE_ID: MCIDEVICEID = 4294967295;
pub const MCI_ANIM_GETDEVCAPS_CAN_REVERSE: u32 = 16385;
pub const MCI_ANIM_GETDEVCAPS_CAN_STRETCH: u32 = 16391;
pub const MCI_ANIM_GETDEVCAPS_FAST_RATE: u32 = 16386;
pub const MCI_ANIM_GETDEVCAPS_MAX_WINDOWS: u32 = 16392;
pub const MCI_ANIM_GETDEVCAPS_NORMAL_RATE: u32 = 16388;
pub const MCI_ANIM_GETDEVCAPS_PALETTES: u32 = 16390;
pub const MCI_ANIM_GETDEVCAPS_SLOW_RATE: u32 = 16387;
pub const MCI_ANIM_INFO_TEXT: u32 = 65536;
pub const MCI_ANIM_OPEN_NOSTATIC: u32 = 262144;
pub const MCI_ANIM_OPEN_PARENT: u32 = 131072;
#[cfg(feature = "windef")]
pub type MCI_ANIM_OPEN_PARMS = MCI_ANIM_OPEN_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: MCIDEVICEID,
    pub lpstrDeviceType: windows_sys::core::PCSTR,
    pub lpstrElementName: windows_sys::core::PCSTR,
    pub lpstrAlias: windows_sys::core::PCSTR,
    pub dwStyle: u32,
    pub hWndParent: super::windef::HWND,
}
#[cfg(feature = "windef")]
impl Default for MCI_ANIM_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: MCIDEVICEID,
    pub lpstrDeviceType: windows_sys::core::PCWSTR,
    pub lpstrElementName: windows_sys::core::PCWSTR,
    pub lpstrAlias: windows_sys::core::PCWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::windef::HWND,
}
#[cfg(feature = "windef")]
impl Default for MCI_ANIM_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_ANIM_OPEN_WS: u32 = 65536;
pub const MCI_ANIM_PLAY_FAST: u32 = 262144;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_ANIM_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
pub const MCI_ANIM_PLAY_REVERSE: u32 = 131072;
pub const MCI_ANIM_PLAY_SCAN: u32 = 1048576;
pub const MCI_ANIM_PLAY_SLOW: u32 = 524288;
pub const MCI_ANIM_PLAY_SPEED: u32 = 65536;
pub const MCI_ANIM_PUT_DESTINATION: u32 = 262144;
pub const MCI_ANIM_PUT_SOURCE: u32 = 131072;
pub const MCI_ANIM_REALIZE_BKGD: u32 = 131072;
pub const MCI_ANIM_REALIZE_NORM: u32 = 65536;
pub const MCI_ANIM_RECT: u32 = 65536;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MCI_ANIM_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::windef::RECT,
}
pub const MCI_ANIM_STATUS_FORWARD: u32 = 16386;
pub const MCI_ANIM_STATUS_HPAL: u32 = 16388;
pub const MCI_ANIM_STATUS_HWND: u32 = 16387;
pub const MCI_ANIM_STATUS_SPEED: u32 = 16385;
pub const MCI_ANIM_STATUS_STRETCH: u32 = 16389;
pub const MCI_ANIM_STEP_FRAMES: u32 = 131072;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_ANIM_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
pub const MCI_ANIM_STEP_REVERSE: u32 = 65536;
pub const MCI_ANIM_UPDATE_HDC: u32 = 131072;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_UPDATE_PARMS {
    pub dwCallback: usize,
    pub rc: super::windef::RECT,
    pub hDC: super::windef::HDC,
}
#[cfg(feature = "windef")]
impl Default for MCI_ANIM_UPDATE_PARMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_ANIM_WHERE_DESTINATION: u32 = 262144;
pub const MCI_ANIM_WHERE_SOURCE: u32 = 131072;
pub const MCI_ANIM_WINDOW_DEFAULT: u32 = 0;
pub const MCI_ANIM_WINDOW_DISABLE_STRETCH: u32 = 2097152;
pub const MCI_ANIM_WINDOW_ENABLE_STRETCH: u32 = 1048576;
pub const MCI_ANIM_WINDOW_HWND: u32 = 65536;
#[cfg(feature = "windef")]
pub type MCI_ANIM_WINDOW_PARMS = MCI_ANIM_WINDOW_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::windef::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCSTR,
}
#[cfg(feature = "windef")]
impl Default for MCI_ANIM_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::windef::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCWSTR,
}
#[cfg(feature = "windef")]
impl Default for MCI_ANIM_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_ANIM_WINDOW_STATE: u32 = 262144;
pub const MCI_ANIM_WINDOW_TEXT: u32 = 524288;
pub const MCI_BREAK: u32 = 2065;
pub const MCI_BREAK_HWND: u32 = 512;
pub const MCI_BREAK_KEY: u32 = 256;
pub const MCI_BREAK_OFF: u32 = 1024;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_BREAK_PARMS {
    pub dwCallback: usize,
    pub nVirtKey: i32,
    pub hwndBreak: super::windef::HWND,
}
#[cfg(feature = "windef")]
impl Default for MCI_BREAK_PARMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_CDA_STATUS_TYPE_TRACK: u32 = 16385;
pub const MCI_CDA_TRACK_AUDIO: u32 = 1088;
pub const MCI_CDA_TRACK_OTHER: u32 = 1089;
pub const MCI_CLOSE: u32 = 2052;
pub const MCI_COPY: u32 = 2130;
pub const MCI_CUE: u32 = 2096;
pub const MCI_CUT: u32 = 2129;
pub const MCI_DELETE: u32 = 2134;
pub const MCI_DEVTYPE_ANIMATION: u32 = 519;
pub const MCI_DEVTYPE_CD_AUDIO: u32 = 516;
pub const MCI_DEVTYPE_DAT: u32 = 517;
pub const MCI_DEVTYPE_DIGITAL_VIDEO: u32 = 520;
pub const MCI_DEVTYPE_FIRST: u32 = 513;
pub const MCI_DEVTYPE_FIRST_USER: u32 = 4096;
pub const MCI_DEVTYPE_LAST: u32 = 523;
pub const MCI_DEVTYPE_OTHER: u32 = 521;
pub const MCI_DEVTYPE_OVERLAY: u32 = 515;
pub const MCI_DEVTYPE_SCANNER: u32 = 518;
pub const MCI_DEVTYPE_SEQUENCER: u32 = 523;
pub const MCI_DEVTYPE_VCR: u32 = 513;
pub const MCI_DEVTYPE_VIDEODISC: u32 = 514;
pub const MCI_DEVTYPE_WAVEFORM_AUDIO: u32 = 522;
pub const MCI_ESCAPE: u32 = 2053;
pub const MCI_FIRST: u32 = 2048;
pub const MCI_FORMAT_BYTES: u32 = 8;
pub const MCI_FORMAT_FRAMES: u32 = 3;
pub const MCI_FORMAT_HMS: u32 = 1;
pub const MCI_FORMAT_MILLISECONDS: u32 = 0;
pub const MCI_FORMAT_MSF: u32 = 2;
pub const MCI_FORMAT_SAMPLES: u32 = 9;
pub const MCI_FORMAT_SMPTE_24: u32 = 4;
pub const MCI_FORMAT_SMPTE_25: u32 = 5;
pub const MCI_FORMAT_SMPTE_30: u32 = 6;
pub const MCI_FORMAT_SMPTE_30DROP: u32 = 7;
pub const MCI_FORMAT_TMSF: u32 = 10;
pub const MCI_FREEZE: u32 = 2116;
pub const MCI_FROM: u32 = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_GENERIC_PARMS {
    pub dwCallback: usize,
}
pub const MCI_GETDEVCAPS: u32 = 2059;
pub const MCI_GETDEVCAPS_CAN_EJECT: u32 = 7;
pub const MCI_GETDEVCAPS_CAN_PLAY: u32 = 8;
pub const MCI_GETDEVCAPS_CAN_RECORD: u32 = 1;
pub const MCI_GETDEVCAPS_CAN_SAVE: u32 = 9;
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: u32 = 6;
pub const MCI_GETDEVCAPS_DEVICE_TYPE: u32 = 4;
pub const MCI_GETDEVCAPS_HAS_AUDIO: u32 = 2;
pub const MCI_GETDEVCAPS_HAS_VIDEO: u32 = 3;
pub const MCI_GETDEVCAPS_ITEM: u32 = 256;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_GETDEVCAPS_PARMS {
    pub dwCallback: usize,
    pub dwReturn: u32,
    pub dwItem: u32,
}
pub const MCI_GETDEVCAPS_USES_FILES: u32 = 5;
pub const MCI_INFO: u32 = 2058;
pub const MCI_INFO_COPYRIGHT: u32 = 8192;
pub const MCI_INFO_FILE: u32 = 512;
pub const MCI_INFO_MEDIA_IDENTITY: u32 = 2048;
pub const MCI_INFO_MEDIA_UPC: u32 = 1024;
pub const MCI_INFO_NAME: u32 = 4096;
pub type MCI_INFO_PARMS = MCI_INFO_PARMSA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_INFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PSTR,
    pub dwRetSize: u32,
}
impl Default for MCI_INFO_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_INFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PWSTR,
    pub dwRetSize: u32,
}
impl Default for MCI_INFO_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_INFO_PRODUCT: u32 = 256;
pub const MCI_LAST: u32 = 4095;
pub const MCI_LOAD: u32 = 2128;
pub const MCI_LOAD_FILE: u32 = 256;
pub type MCI_LOAD_PARMS = MCI_LOAD_PARMSA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_LOAD_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
}
impl Default for MCI_LOAD_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_LOAD_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCWSTR,
}
impl Default for MCI_LOAD_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_MODE_NOT_READY: u32 = 524;
pub const MCI_MODE_OPEN: u32 = 530;
pub const MCI_MODE_PAUSE: u32 = 529;
pub const MCI_MODE_PLAY: u32 = 526;
pub const MCI_MODE_RECORD: u32 = 527;
pub const MCI_MODE_SEEK: u32 = 528;
pub const MCI_MODE_STOP: u32 = 525;
pub const MCI_NOTIFY: u32 = 1;
pub const MCI_NOTIFY_ABORTED: u32 = 4;
pub const MCI_NOTIFY_FAILURE: u32 = 8;
pub const MCI_NOTIFY_SUCCESSFUL: u32 = 1;
pub const MCI_NOTIFY_SUPERSEDED: u32 = 2;
pub const MCI_OPEN: u32 = 2051;
pub const MCI_OPEN_ALIAS: u32 = 1024;
pub const MCI_OPEN_ELEMENT: u32 = 512;
pub const MCI_OPEN_ELEMENT_ID: u32 = 2048;
pub type MCI_OPEN_PARMS = MCI_OPEN_PARMSA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: MCIDEVICEID,
    pub lpstrDeviceType: windows_sys::core::PCSTR,
    pub lpstrElementName: windows_sys::core::PCSTR,
    pub lpstrAlias: windows_sys::core::PCSTR,
}
impl Default for MCI_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: MCIDEVICEID,
    pub lpstrDeviceType: windows_sys::core::PCWSTR,
    pub lpstrElementName: windows_sys::core::PCWSTR,
    pub lpstrAlias: windows_sys::core::PCWSTR,
}
impl Default for MCI_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OPEN_SHAREABLE: u32 = 256;
pub const MCI_OPEN_TYPE: u32 = 8192;
pub const MCI_OPEN_TYPE_ID: u32 = 4096;
pub const MCI_OVLY_GETDEVCAPS_CAN_FREEZE: u32 = 16386;
pub const MCI_OVLY_GETDEVCAPS_CAN_STRETCH: u32 = 16385;
pub const MCI_OVLY_GETDEVCAPS_MAX_WINDOWS: u32 = 16387;
pub const MCI_OVLY_INFO_TEXT: u32 = 65536;
#[cfg(feature = "windef")]
pub type MCI_OVLY_LOAD_PARMS = MCI_OVLY_LOAD_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_LOAD_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
    pub rc: super::windef::RECT,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_LOAD_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_LOAD_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCWSTR,
    pub rc: super::windef::RECT,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_LOAD_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OVLY_OPEN_PARENT: u32 = 131072;
#[cfg(feature = "windef")]
pub type MCI_OVLY_OPEN_PARMS = MCI_OVLY_OPEN_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: MCIDEVICEID,
    pub lpstrDeviceType: windows_sys::core::PCSTR,
    pub lpstrElementName: windows_sys::core::PCSTR,
    pub lpstrAlias: windows_sys::core::PCSTR,
    pub dwStyle: u32,
    pub hWndParent: super::windef::HWND,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: MCIDEVICEID,
    pub lpstrDeviceType: windows_sys::core::PCWSTR,
    pub lpstrElementName: windows_sys::core::PCWSTR,
    pub lpstrAlias: windows_sys::core::PCWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::windef::HWND,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OVLY_OPEN_WS: u32 = 65536;
pub const MCI_OVLY_PUT_DESTINATION: u32 = 262144;
pub const MCI_OVLY_PUT_FRAME: u32 = 524288;
pub const MCI_OVLY_PUT_SOURCE: u32 = 131072;
pub const MCI_OVLY_PUT_VIDEO: u32 = 1048576;
pub const MCI_OVLY_RECT: u32 = 65536;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MCI_OVLY_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::windef::RECT,
}
#[cfg(feature = "windef")]
pub type MCI_OVLY_SAVE_PARMS = MCI_OVLY_SAVE_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
    pub rc: super::windef::RECT,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCWSTR,
    pub rc: super::windef::RECT,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OVLY_STATUS_HWND: u32 = 16385;
pub const MCI_OVLY_STATUS_STRETCH: u32 = 16386;
pub const MCI_OVLY_WHERE_DESTINATION: u32 = 262144;
pub const MCI_OVLY_WHERE_FRAME: u32 = 524288;
pub const MCI_OVLY_WHERE_SOURCE: u32 = 131072;
pub const MCI_OVLY_WHERE_VIDEO: u32 = 1048576;
pub const MCI_OVLY_WINDOW_DEFAULT: u32 = 0;
pub const MCI_OVLY_WINDOW_DISABLE_STRETCH: u32 = 2097152;
pub const MCI_OVLY_WINDOW_ENABLE_STRETCH: u32 = 1048576;
pub const MCI_OVLY_WINDOW_HWND: u32 = 65536;
#[cfg(feature = "windef")]
pub type MCI_OVLY_WINDOW_PARMS = MCI_OVLY_WINDOW_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::windef::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCSTR,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::windef::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCWSTR,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OVLY_WINDOW_STATE: u32 = 262144;
pub const MCI_OVLY_WINDOW_TEXT: u32 = 524288;
pub const MCI_PASTE: u32 = 2131;
pub const MCI_PAUSE: u32 = 2057;
pub const MCI_PLAY: u32 = 2054;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
pub const MCI_PUT: u32 = 2114;
pub const MCI_REALIZE: u32 = 2112;
pub const MCI_RECORD: u32 = 2063;
pub const MCI_RECORD_INSERT: u32 = 256;
pub const MCI_RECORD_OVERWRITE: u32 = 512;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_RECORD_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
pub const MCI_RESUME: u32 = 2133;
pub const MCI_SAVE: u32 = 2067;
pub const MCI_SAVE_FILE: u32 = 256;
pub type MCI_SAVE_PARMS = MCI_SAVE_PARMSA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
}
impl Default for MCI_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCWSTR,
}
impl Default for MCI_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_SEEK: u32 = 2055;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_SEEK_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
}
pub const MCI_SEEK_TO_END: u32 = 512;
pub const MCI_SEEK_TO_START: u32 = 256;
pub const MCI_SEQ_DIV_PPQN: u32 = 1216;
pub const MCI_SEQ_DIV_SMPTE_24: u32 = 1217;
pub const MCI_SEQ_DIV_SMPTE_25: u32 = 1218;
pub const MCI_SEQ_DIV_SMPTE_30: u32 = 1220;
pub const MCI_SEQ_DIV_SMPTE_30DROP: u32 = 1219;
pub const MCI_SEQ_FILE: u32 = 16386;
pub const MCI_SEQ_FORMAT_SONGPTR: u32 = 16385;
pub const MCI_SEQ_MAPPER: u32 = 65535;
pub const MCI_SEQ_MIDI: u32 = 16387;
pub const MCI_SEQ_NONE: u32 = 65533;
pub const MCI_SEQ_SET_MASTER: u32 = 524288;
pub const MCI_SEQ_SET_OFFSET: u32 = 16777216;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_SEQ_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub dwTempo: u32,
    pub dwPort: u32,
    pub dwSlave: u32,
    pub dwMaster: u32,
    pub dwOffset: u32,
}
pub const MCI_SEQ_SET_PORT: u32 = 131072;
pub const MCI_SEQ_SET_SLAVE: u32 = 262144;
pub const MCI_SEQ_SET_TEMPO: u32 = 65536;
pub const MCI_SEQ_SMPTE: u32 = 16388;
pub const MCI_SEQ_STATUS_COPYRIGHT: u32 = 16396;
pub const MCI_SEQ_STATUS_DIVTYPE: u32 = 16394;
pub const MCI_SEQ_STATUS_MASTER: u32 = 16392;
pub const MCI_SEQ_STATUS_NAME: u32 = 16395;
pub const MCI_SEQ_STATUS_OFFSET: u32 = 16393;
pub const MCI_SEQ_STATUS_PORT: u32 = 16387;
pub const MCI_SEQ_STATUS_SLAVE: u32 = 16391;
pub const MCI_SEQ_STATUS_TEMPO: u32 = 16386;
pub const MCI_SET: u32 = 2061;
pub const MCI_SET_AUDIO: u32 = 2048;
pub const MCI_SET_AUDIO_ALL: u32 = 0;
pub const MCI_SET_AUDIO_LEFT: u32 = 1;
pub const MCI_SET_AUDIO_RIGHT: u32 = 2;
pub const MCI_SET_DOOR_CLOSED: u32 = 512;
pub const MCI_SET_DOOR_OPEN: u32 = 256;
pub const MCI_SET_OFF: u32 = 16384;
pub const MCI_SET_ON: u32 = 8192;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
}
pub const MCI_SET_TIME_FORMAT: u32 = 1024;
pub const MCI_SET_VIDEO: u32 = 4096;
pub const MCI_SPIN: u32 = 2060;
pub const MCI_STATUS_CURRENT_TRACK: u32 = 8;
pub const MCI_STATUS_ITEM: u32 = 256;
pub const MCI_STATUS_LENGTH: u32 = 1;
pub const MCI_STATUS_MEDIA_PRESENT: u32 = 5;
pub const MCI_STATUS_MODE: u32 = 4;
pub const MCI_STATUS_NUMBER_OF_TRACKS: u32 = 3;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATUS_PARMS {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
}
pub const MCI_STATUS_POSITION: u32 = 2;
pub const MCI_STATUS_READY: u32 = 7;
pub const MCI_STATUS_START: u32 = 512;
pub const MCI_STATUS_TIME_FORMAT: u32 = 6;
pub const MCI_STEP: u32 = 2062;
pub const MCI_STOP: u32 = 2056;
pub const MCI_SYSINFO: u32 = 2064;
pub const MCI_SYSINFO_INSTALLNAME: u32 = 2048;
pub const MCI_SYSINFO_NAME: u32 = 1024;
pub const MCI_SYSINFO_OPEN: u32 = 512;
pub type MCI_SYSINFO_PARMS = MCI_SYSINFO_PARMSA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SYSINFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PSTR,
    pub dwRetSize: u32,
    pub dwNumber: u32,
    pub wDeviceType: u32,
}
impl Default for MCI_SYSINFO_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_SYSINFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: windows_sys::core::PWSTR,
    pub dwRetSize: u32,
    pub dwNumber: u32,
    pub wDeviceType: u32,
}
impl Default for MCI_SYSINFO_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_SYSINFO_QUANTITY: u32 = 256;
pub const MCI_TO: u32 = 8;
pub const MCI_TRACK: u32 = 16;
pub const MCI_UNFREEZE: u32 = 2117;
pub const MCI_UPDATE: u32 = 2132;
pub const MCI_USER_MESSAGES: u32 = 3072;
pub type MCI_VD_ESCAPE_PARMS = MCI_VD_ESCAPE_PARMSA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_VD_ESCAPE_PARMSA {
    pub dwCallback: usize,
    pub lpstrCommand: windows_sys::core::PCSTR,
}
impl Default for MCI_VD_ESCAPE_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_VD_ESCAPE_PARMSW {
    pub dwCallback: usize,
    pub lpstrCommand: windows_sys::core::PCWSTR,
}
impl Default for MCI_VD_ESCAPE_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_VD_ESCAPE_STRING: u32 = 256;
pub const MCI_VD_FORMAT_TRACK: u32 = 16385;
pub const MCI_VD_GETDEVCAPS_CAN_REVERSE: u32 = 16386;
pub const MCI_VD_GETDEVCAPS_CAV: u32 = 131072;
pub const MCI_VD_GETDEVCAPS_CLV: u32 = 65536;
pub const MCI_VD_GETDEVCAPS_FAST_RATE: u32 = 16387;
pub const MCI_VD_GETDEVCAPS_NORMAL_RATE: u32 = 16389;
pub const MCI_VD_GETDEVCAPS_SLOW_RATE: u32 = 16388;
pub const MCI_VD_MEDIA_CAV: u32 = 1027;
pub const MCI_VD_MEDIA_CLV: u32 = 1026;
pub const MCI_VD_MEDIA_OTHER: u32 = 1028;
pub const MCI_VD_MODE_PARK: u32 = 1025;
pub const MCI_VD_PLAY_FAST: u32 = 131072;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_VD_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
pub const MCI_VD_PLAY_REVERSE: u32 = 65536;
pub const MCI_VD_PLAY_SCAN: u32 = 524288;
pub const MCI_VD_PLAY_SLOW: u32 = 1048576;
pub const MCI_VD_PLAY_SPEED: u32 = 262144;
pub const MCI_VD_SEEK_REVERSE: u32 = 65536;
pub const MCI_VD_SPIN_DOWN: u32 = 131072;
pub const MCI_VD_SPIN_UP: u32 = 65536;
pub const MCI_VD_STATUS_DISC_SIZE: u32 = 16390;
pub const MCI_VD_STATUS_FORWARD: u32 = 16387;
pub const MCI_VD_STATUS_MEDIA_TYPE: u32 = 16388;
pub const MCI_VD_STATUS_SIDE: u32 = 16389;
pub const MCI_VD_STATUS_SPEED: u32 = 16386;
pub const MCI_VD_STEP_FRAMES: u32 = 65536;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_VD_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
pub const MCI_VD_STEP_REVERSE: u32 = 131072;
pub const MCI_WAIT: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_WAVE_DELETE_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
pub const MCI_WAVE_GETDEVCAPS_INPUTS: u32 = 16385;
pub const MCI_WAVE_GETDEVCAPS_OUTPUTS: u32 = 16386;
pub const MCI_WAVE_INPUT: u32 = 4194304;
pub const MCI_WAVE_MAPPER: u32 = 1153;
pub const MCI_WAVE_OPEN_BUFFER: u32 = 65536;
pub type MCI_WAVE_OPEN_PARMS = MCI_WAVE_OPEN_PARMSA;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_WAVE_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: MCIDEVICEID,
    pub lpstrDeviceType: windows_sys::core::PCSTR,
    pub lpstrElementName: windows_sys::core::PCSTR,
    pub lpstrAlias: windows_sys::core::PCSTR,
    pub dwBufferSeconds: u32,
}
impl Default for MCI_WAVE_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_WAVE_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: MCIDEVICEID,
    pub lpstrDeviceType: windows_sys::core::PCWSTR,
    pub lpstrElementName: windows_sys::core::PCWSTR,
    pub lpstrAlias: windows_sys::core::PCWSTR,
    pub dwBufferSeconds: u32,
}
impl Default for MCI_WAVE_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_WAVE_OUTPUT: u32 = 8388608;
pub const MCI_WAVE_PCM: u32 = 1152;
pub const MCI_WAVE_SET_ANYINPUT: u32 = 67108864;
pub const MCI_WAVE_SET_ANYOUTPUT: u32 = 134217728;
pub const MCI_WAVE_SET_AVGBYTESPERSEC: u32 = 524288;
pub const MCI_WAVE_SET_BITSPERSAMPLE: u32 = 2097152;
pub const MCI_WAVE_SET_BLOCKALIGN: u32 = 1048576;
pub const MCI_WAVE_SET_CHANNELS: u32 = 131072;
pub const MCI_WAVE_SET_FORMATTAG: u32 = 65536;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_WAVE_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub wInput: u32,
    pub wOutput: u32,
    pub wFormatTag: u16,
    pub wReserved2: u16,
    pub nChannels: u16,
    pub wReserved3: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wReserved4: u16,
    pub wBitsPerSample: u16,
    pub wReserved5: u16,
}
pub const MCI_WAVE_SET_SAMPLESPERSEC: u32 = 262144;
pub const MCI_WAVE_STATUS_AVGBYTESPERSEC: u32 = 16388;
pub const MCI_WAVE_STATUS_BITSPERSAMPLE: u32 = 16390;
pub const MCI_WAVE_STATUS_BLOCKALIGN: u32 = 16389;
pub const MCI_WAVE_STATUS_CHANNELS: u32 = 16386;
pub const MCI_WAVE_STATUS_FORMATTAG: u32 = 16385;
pub const MCI_WAVE_STATUS_LEVEL: u32 = 16391;
pub const MCI_WAVE_STATUS_SAMPLESPERSEC: u32 = 16387;
pub const MCI_WHERE: u32 = 2115;
pub const MCI_WINDOW: u32 = 2113;
#[cfg(feature = "windef")]
pub type PMCI_ANIM_OPEN_PARMS = PMCI_ANIM_OPEN_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_ANIM_OPEN_PARMSA = *mut MCI_ANIM_OPEN_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_ANIM_OPEN_PARMSW = *mut MCI_ANIM_OPEN_PARMSW;
pub type PMCI_ANIM_PLAY_PARMS = *mut MCI_ANIM_PLAY_PARMS;
#[cfg(feature = "windef")]
pub type PMCI_ANIM_RECT_PARMS = *mut MCI_ANIM_RECT_PARMS;
pub type PMCI_ANIM_STEP_PARMS = *mut MCI_ANIM_STEP_PARMS;
#[cfg(feature = "windef")]
pub type PMCI_ANIM_UPDATE_PARMS = *mut MCI_ANIM_UPDATE_PARMS;
#[cfg(feature = "windef")]
pub type PMCI_ANIM_WINDOW_PARMS = PMCI_ANIM_WINDOW_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_ANIM_WINDOW_PARMSA = *mut MCI_ANIM_WINDOW_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_ANIM_WINDOW_PARMSW = *mut MCI_ANIM_WINDOW_PARMSW;
#[cfg(feature = "windef")]
pub type PMCI_BREAK_PARMS = *mut MCI_BREAK_PARMS;
pub type PMCI_GENERIC_PARMS = *mut MCI_GENERIC_PARMS;
pub type PMCI_GETDEVCAPS_PARMS = *mut MCI_GETDEVCAPS_PARMS;
pub type PMCI_LOAD_PARMS = PMCI_LOAD_PARMSA;
pub type PMCI_LOAD_PARMSA = *mut MCI_LOAD_PARMSA;
pub type PMCI_LOAD_PARMSW = *mut MCI_LOAD_PARMSW;
pub type PMCI_OPEN_PARMS = PMCI_OPEN_PARMSA;
pub type PMCI_OPEN_PARMSA = *mut MCI_OPEN_PARMSA;
pub type PMCI_OPEN_PARMSW = *mut MCI_OPEN_PARMSW;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_LOAD_PARMS = PMCI_OVLY_LOAD_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_LOAD_PARMSA = *mut MCI_OVLY_LOAD_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_LOAD_PARMSW = *mut MCI_OVLY_LOAD_PARMSW;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_OPEN_PARMS = PMCI_OVLY_OPEN_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_OPEN_PARMSA = *mut MCI_OVLY_OPEN_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_OPEN_PARMSW = *mut MCI_OVLY_OPEN_PARMSW;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_RECT_PARMS = *mut MCI_OVLY_RECT_PARMS;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_SAVE_PARMS = PMCI_OVLY_SAVE_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_SAVE_PARMSA = *mut MCI_OVLY_SAVE_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_SAVE_PARMSW = *mut MCI_OVLY_SAVE_PARMSW;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_WINDOW_PARMS = PMCI_OVLY_WINDOW_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_WINDOW_PARMSA = *mut MCI_OVLY_WINDOW_PARMSA;
#[cfg(feature = "windef")]
pub type PMCI_OVLY_WINDOW_PARMSW = *mut MCI_OVLY_WINDOW_PARMSW;
pub type PMCI_PLAY_PARMS = *mut MCI_PLAY_PARMS;
pub type PMCI_SAVE_PARMS = PMCI_SAVE_PARMSA;
pub type PMCI_SAVE_PARMSA = *mut MCI_SAVE_PARMSA;
pub type PMCI_SAVE_PARMSW = *mut MCI_SAVE_PARMSW;
pub type PMCI_SEEK_PARMS = *mut MCI_SEEK_PARMS;
pub type PMCI_SEQ_SET_PARMS = *mut MCI_SEQ_SET_PARMS;
pub type PMCI_SET_PARMS = *mut MCI_SET_PARMS;
pub type PMCI_STATUS_PARMS = *mut MCI_STATUS_PARMS;
pub type PMCI_SYSINFO_PARMS = PMCI_SYSINFO_PARMSA;
pub type PMCI_SYSINFO_PARMSA = *mut MCI_SYSINFO_PARMSA;
pub type PMCI_SYSINFO_PARMSW = *mut MCI_SYSINFO_PARMSW;
pub type PMCI_VD_ESCAPE_PARMS = PMCI_VD_ESCAPE_PARMSA;
pub type PMCI_VD_ESCAPE_PARMSA = *mut MCI_VD_ESCAPE_PARMSA;
pub type PMCI_VD_ESCAPE_PARMSW = *mut MCI_VD_ESCAPE_PARMSW;
pub type PMCI_VD_PLAY_PARMS = *mut MCI_VD_PLAY_PARMS;
pub type PMCI_VD_STEP_PARMS = *mut MCI_VD_STEP_PARMS;
pub type PMCI_WAVE_DELETE_PARMS = *mut MCI_WAVE_DELETE_PARMS;
pub type PMCI_WAVE_OPEN_PARMS = PMCI_WAVE_OPEN_PARMSA;
pub type PMCI_WAVE_OPEN_PARMSA = *mut MCI_WAVE_OPEN_PARMSA;
pub type PMCI_WAVE_OPEN_PARMSW = *mut MCI_WAVE_OPEN_PARMSW;
pub type PMCI_WAVE_SET_PARMS = *mut MCI_WAVE_SET_PARMS;
pub type YIELDPROC = Option<unsafe extern "system" fn(mciid: MCIDEVICEID, dwyielddata: u32) -> u32>;
