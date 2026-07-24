#[cfg(feature = "winnt")]
windows_link::link!("winmm.dll" "system" fn mciDriverNotify(hwndcallback : super::HANDLE, wdeviceid : MCIDEVICEID, ustatus : u32) -> windows_sys::core::BOOL);
windows_link::link!("winmm.dll" "system" fn mciDriverYield(wdeviceid : MCIDEVICEID) -> u32);
windows_link::link!("winmm.dll" "system" fn mciFreeCommandResource(wtable : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("winmm.dll" "system" fn mciGetCreatorTask(mciid : MCIDEVICEID) -> super::HTASK);
windows_link::link!("winmm.dll" "system" fn mciGetDeviceIDA(pszdevice : windows_sys::core::PCSTR) -> MCIDEVICEID);
windows_link::link!("winmm.dll" "system" fn mciGetDeviceIDFromElementIDA(dwelementid : u32, lpstrtype : windows_sys::core::PCSTR) -> MCIDEVICEID);
windows_link::link!("winmm.dll" "system" fn mciGetDeviceIDFromElementIDW(dwelementid : u32, lpstrtype : windows_sys::core::PCWSTR) -> MCIDEVICEID);
windows_link::link!("winmm.dll" "system" fn mciGetDeviceIDW(pszdevice : windows_sys::core::PCWSTR) -> MCIDEVICEID);
windows_link::link!("winmm.dll" "system" fn mciGetDriverData(wdeviceid : MCIDEVICEID) -> usize);
windows_link::link!("winmm.dll" "system" fn mciGetErrorStringA(mcierr : MCIERROR, psztext : windows_sys::core::PSTR, cchtext : u32) -> windows_sys::core::BOOL);
windows_link::link!("winmm.dll" "system" fn mciGetErrorStringW(mcierr : MCIERROR, psztext : windows_sys::core::PWSTR, cchtext : u32) -> windows_sys::core::BOOL);
windows_link::link!("winmm.dll" "system" fn mciGetYieldProc(mciid : MCIDEVICEID, pdwyielddata : *const u32) -> YIELDPROC);
#[cfg(feature = "winnt")]
windows_link::link!("winmm.dll" "system" fn mciLoadCommandResource(hinstance : super::HANDLE, lpresname : windows_sys::core::PCWSTR, wtype : u32) -> u32);
windows_link::link!("winmm.dll" "system" fn mciSendCommandA(mciid : MCIDEVICEID, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> MCIERROR);
windows_link::link!("winmm.dll" "system" fn mciSendCommandW(mciid : MCIDEVICEID, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> MCIERROR);
#[cfg(feature = "windef")]
windows_link::link!("winmm.dll" "system" fn mciSendStringA(lpstrcommand : windows_sys::core::PCSTR, lpstrreturnstring : windows_sys::core::PSTR, ureturnlength : u32, hwndcallback : super::HWND) -> MCIERROR);
#[cfg(feature = "windef")]
windows_link::link!("winmm.dll" "system" fn mciSendStringW(lpstrcommand : windows_sys::core::PCWSTR, lpstrreturnstring : windows_sys::core::PWSTR, ureturnlength : u32, hwndcallback : super::HWND) -> MCIERROR);
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
pub const MCIERR_BAD_CONSTANT: i32 = 290;
pub const MCIERR_BAD_INTEGER: i32 = 270;
pub const MCIERR_BAD_TIME_FORMAT: i32 = 293;
pub const MCIERR_CANNOT_LOAD_DRIVER: i32 = 266;
pub const MCIERR_CANNOT_USE_ALL: i32 = 279;
pub const MCIERR_CREATEWINDOW: i32 = 347;
pub const MCIERR_CUSTOM_DRIVER_BASE: i32 = 512;
pub const MCIERR_DEVICE_LENGTH: i32 = 310;
pub const MCIERR_DEVICE_LOCKED: i32 = 288;
pub const MCIERR_DEVICE_NOT_INSTALLED: i32 = 306;
pub const MCIERR_DEVICE_NOT_READY: i32 = 276;
pub const MCIERR_DEVICE_OPEN: i32 = 265;
pub const MCIERR_DEVICE_ORD_LENGTH: i32 = 311;
pub const MCIERR_DEVICE_TYPE_REQUIRED: i32 = 287;
pub const MCIERR_DRIVER: i32 = 278;
pub const MCIERR_DRIVER_INTERNAL: i32 = 272;
pub const MCIERR_DUPLICATE_ALIAS: i32 = 289;
pub const MCIERR_DUPLICATE_FLAGS: i32 = 295;
pub const MCIERR_EXTENSION_NOT_FOUND: i32 = 281;
pub const MCIERR_EXTRA_CHARACTERS: i32 = 305;
pub const MCIERR_FILENAME_REQUIRED: i32 = 304;
pub const MCIERR_FILE_NOT_FOUND: i32 = 275;
pub const MCIERR_FILE_NOT_SAVED: i32 = 286;
pub const MCIERR_FILE_READ: i32 = 348;
pub const MCIERR_FILE_WRITE: i32 = 349;
pub const MCIERR_FLAGS_NOT_COMPATIBLE: i32 = 284;
pub const MCIERR_GET_CD: i32 = 307;
pub const MCIERR_HARDWARE: i32 = 262;
pub const MCIERR_ILLEGAL_FOR_AUTO_OPEN: i32 = 303;
pub const MCIERR_INTERNAL: i32 = 277;
pub const MCIERR_INVALID_DEVICE_ID: i32 = 257;
pub const MCIERR_INVALID_DEVICE_NAME: i32 = 263;
pub const MCIERR_INVALID_FILE: i32 = 296;
pub const MCIERR_MISSING_COMMAND_STRING: i32 = 267;
pub const MCIERR_MISSING_DEVICE_NAME: i32 = 292;
pub const MCIERR_MISSING_PARAMETER: i32 = 273;
pub const MCIERR_MISSING_STRING_ARGUMENT: i32 = 269;
pub const MCIERR_MULTIPLE: i32 = 280;
pub const MCIERR_MUST_USE_SHAREABLE: i32 = 291;
pub const MCIERR_NEW_REQUIRES_ALIAS: i32 = 299;
pub const MCIERR_NONAPPLICABLE_FUNCTION: i32 = 302;
pub const MCIERR_NOTIFY_ON_AUTO_OPEN: i32 = 300;
pub const MCIERR_NO_CLOSING_QUOTE: i32 = 294;
pub const MCIERR_NO_ELEMENT_ALLOWED: i32 = 301;
pub const MCIERR_NO_IDENTITY: i32 = 350;
pub const MCIERR_NO_INTEGER: i32 = 312;
pub const MCIERR_NO_WINDOW: i32 = 346;
pub const MCIERR_NULL_PARAMETER_BLOCK: i32 = 297;
pub const MCIERR_OUTOFRANGE: i32 = 282;
pub const MCIERR_OUT_OF_MEMORY: i32 = 264;
pub const MCIERR_PARAM_OVERFLOW: i32 = 268;
pub const MCIERR_PARSER_INTERNAL: i32 = 271;
pub const MCIERR_SEQ_DIV_INCOMPATIBLE: i32 = 336;
pub const MCIERR_SEQ_NOMIDIPRESENT: i32 = 343;
pub const MCIERR_SEQ_PORTUNSPECIFIED: i32 = 342;
pub const MCIERR_SEQ_PORT_INUSE: i32 = 337;
pub const MCIERR_SEQ_PORT_MAPNODEVICE: i32 = 339;
pub const MCIERR_SEQ_PORT_MISCERROR: i32 = 340;
pub const MCIERR_SEQ_PORT_NONEXISTENT: i32 = 338;
pub const MCIERR_SEQ_TIMER: i32 = 341;
pub const MCIERR_SET_CD: i32 = 308;
pub const MCIERR_SET_DRIVE: i32 = 309;
pub const MCIERR_UNNAMED_RESOURCE: i32 = 298;
pub const MCIERR_UNRECOGNIZED_COMMAND: i32 = 261;
pub const MCIERR_UNRECOGNIZED_KEYWORD: i32 = 259;
pub const MCIERR_UNSUPPORTED_FUNCTION: i32 = 274;
pub const MCIERR_WAVE_INPUTSINUSE: i32 = 322;
pub const MCIERR_WAVE_INPUTSUNSUITABLE: i32 = 328;
pub const MCIERR_WAVE_INPUTUNSPECIFIED: i32 = 325;
pub const MCIERR_WAVE_OUTPUTSINUSE: i32 = 320;
pub const MCIERR_WAVE_OUTPUTSUNSUITABLE: i32 = 326;
pub const MCIERR_WAVE_OUTPUTUNSPECIFIED: i32 = 324;
pub const MCIERR_WAVE_SETINPUTINUSE: i32 = 323;
pub const MCIERR_WAVE_SETINPUTUNSUITABLE: i32 = 329;
pub const MCIERR_WAVE_SETOUTPUTINUSE: i32 = 321;
pub const MCIERR_WAVE_SETOUTPUTUNSUITABLE: i32 = 327;
pub const MCI_ALL_DEVICE_ID: MCIDEVICEID = 4294967295;
pub const MCI_ANIM_GETDEVCAPS_CAN_REVERSE: i32 = 16385;
pub const MCI_ANIM_GETDEVCAPS_CAN_STRETCH: i32 = 16391;
pub const MCI_ANIM_GETDEVCAPS_FAST_RATE: i32 = 16386;
pub const MCI_ANIM_GETDEVCAPS_MAX_WINDOWS: i32 = 16392;
pub const MCI_ANIM_GETDEVCAPS_NORMAL_RATE: i32 = 16388;
pub const MCI_ANIM_GETDEVCAPS_PALETTES: i32 = 16390;
pub const MCI_ANIM_GETDEVCAPS_SLOW_RATE: i32 = 16387;
pub const MCI_ANIM_INFO_TEXT: i32 = 65536;
pub const MCI_ANIM_OPEN_NOSTATIC: i32 = 262144;
pub const MCI_ANIM_OPEN_PARENT: i32 = 131072;
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
    pub hWndParent: super::HWND,
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
    pub hWndParent: super::HWND,
}
#[cfg(feature = "windef")]
impl Default for MCI_ANIM_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_ANIM_OPEN_WS: i32 = 65536;
pub const MCI_ANIM_PLAY_FAST: i32 = 262144;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_ANIM_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
pub const MCI_ANIM_PLAY_REVERSE: i32 = 131072;
pub const MCI_ANIM_PLAY_SCAN: i32 = 1048576;
pub const MCI_ANIM_PLAY_SLOW: i32 = 524288;
pub const MCI_ANIM_PLAY_SPEED: i32 = 65536;
pub const MCI_ANIM_PUT_DESTINATION: i32 = 262144;
pub const MCI_ANIM_PUT_SOURCE: i32 = 131072;
pub const MCI_ANIM_REALIZE_BKGD: i32 = 131072;
pub const MCI_ANIM_REALIZE_NORM: i32 = 65536;
pub const MCI_ANIM_RECT: i32 = 65536;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MCI_ANIM_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::RECT,
}
pub const MCI_ANIM_STATUS_FORWARD: i32 = 16386;
pub const MCI_ANIM_STATUS_HPAL: i32 = 16388;
pub const MCI_ANIM_STATUS_HWND: i32 = 16387;
pub const MCI_ANIM_STATUS_SPEED: i32 = 16385;
pub const MCI_ANIM_STATUS_STRETCH: i32 = 16389;
pub const MCI_ANIM_STEP_FRAMES: i32 = 131072;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_ANIM_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
pub const MCI_ANIM_STEP_REVERSE: i32 = 65536;
pub const MCI_ANIM_UPDATE_HDC: i32 = 131072;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_UPDATE_PARMS {
    pub dwCallback: usize,
    pub rc: super::RECT,
    pub hDC: super::HDC,
}
#[cfg(feature = "windef")]
impl Default for MCI_ANIM_UPDATE_PARMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_ANIM_WHERE_DESTINATION: i32 = 262144;
pub const MCI_ANIM_WHERE_SOURCE: i32 = 131072;
pub const MCI_ANIM_WINDOW_DEFAULT: i32 = 0;
pub const MCI_ANIM_WINDOW_DISABLE_STRETCH: i32 = 2097152;
pub const MCI_ANIM_WINDOW_ENABLE_STRETCH: i32 = 1048576;
pub const MCI_ANIM_WINDOW_HWND: i32 = 65536;
#[cfg(feature = "windef")]
pub type MCI_ANIM_WINDOW_PARMS = MCI_ANIM_WINDOW_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_ANIM_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::HWND,
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
    pub hWnd: super::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCWSTR,
}
#[cfg(feature = "windef")]
impl Default for MCI_ANIM_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_ANIM_WINDOW_STATE: i32 = 262144;
pub const MCI_ANIM_WINDOW_TEXT: i32 = 524288;
pub const MCI_BREAK: i32 = 2065;
pub const MCI_BREAK_HWND: i32 = 512;
pub const MCI_BREAK_KEY: i32 = 256;
pub const MCI_BREAK_OFF: i32 = 1024;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_BREAK_PARMS {
    pub dwCallback: usize,
    pub nVirtKey: i32,
    pub hwndBreak: super::HWND,
}
#[cfg(feature = "windef")]
impl Default for MCI_BREAK_PARMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_CDA_STATUS_TYPE_TRACK: i32 = 16385;
pub const MCI_CDA_TRACK_AUDIO: i32 = 1088;
pub const MCI_CDA_TRACK_OTHER: i32 = 1089;
pub const MCI_CLOSE: i32 = 2052;
pub const MCI_COPY: i32 = 2130;
pub const MCI_CUE: i32 = 2096;
pub const MCI_CUT: i32 = 2129;
pub const MCI_DELETE: i32 = 2134;
pub const MCI_DEVTYPE_ANIMATION: i32 = 519;
pub const MCI_DEVTYPE_CD_AUDIO: i32 = 516;
pub const MCI_DEVTYPE_DAT: i32 = 517;
pub const MCI_DEVTYPE_DIGITAL_VIDEO: i32 = 520;
pub const MCI_DEVTYPE_FIRST: i32 = 513;
pub const MCI_DEVTYPE_FIRST_USER: i32 = 4096;
pub const MCI_DEVTYPE_LAST: i32 = 523;
pub const MCI_DEVTYPE_OTHER: i32 = 521;
pub const MCI_DEVTYPE_OVERLAY: i32 = 515;
pub const MCI_DEVTYPE_SCANNER: i32 = 518;
pub const MCI_DEVTYPE_SEQUENCER: i32 = 523;
pub const MCI_DEVTYPE_VCR: i32 = 513;
pub const MCI_DEVTYPE_VIDEODISC: i32 = 514;
pub const MCI_DEVTYPE_WAVEFORM_AUDIO: i32 = 522;
pub const MCI_ESCAPE: i32 = 2053;
pub const MCI_FIRST: i32 = 2048;
pub const MCI_FORMAT_BYTES: i32 = 8;
pub const MCI_FORMAT_FRAMES: i32 = 3;
pub const MCI_FORMAT_HMS: i32 = 1;
pub const MCI_FORMAT_MILLISECONDS: i32 = 0;
pub const MCI_FORMAT_MSF: i32 = 2;
pub const MCI_FORMAT_SAMPLES: i32 = 9;
pub const MCI_FORMAT_SMPTE_24: i32 = 4;
pub const MCI_FORMAT_SMPTE_25: i32 = 5;
pub const MCI_FORMAT_SMPTE_30: i32 = 6;
pub const MCI_FORMAT_SMPTE_30DROP: i32 = 7;
pub const MCI_FORMAT_TMSF: i32 = 10;
pub const MCI_FREEZE: i32 = 2116;
pub const MCI_FROM: i32 = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_GENERIC_PARMS {
    pub dwCallback: usize,
}
pub const MCI_GETDEVCAPS: i32 = 2059;
pub const MCI_GETDEVCAPS_CAN_EJECT: i32 = 7;
pub const MCI_GETDEVCAPS_CAN_PLAY: i32 = 8;
pub const MCI_GETDEVCAPS_CAN_RECORD: i32 = 1;
pub const MCI_GETDEVCAPS_CAN_SAVE: i32 = 9;
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: i32 = 6;
pub const MCI_GETDEVCAPS_DEVICE_TYPE: i32 = 4;
pub const MCI_GETDEVCAPS_HAS_AUDIO: i32 = 2;
pub const MCI_GETDEVCAPS_HAS_VIDEO: i32 = 3;
pub const MCI_GETDEVCAPS_ITEM: i32 = 256;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_GETDEVCAPS_PARMS {
    pub dwCallback: usize,
    pub dwReturn: u32,
    pub dwItem: u32,
}
pub const MCI_GETDEVCAPS_USES_FILES: i32 = 5;
pub const MCI_INFO: i32 = 2058;
pub const MCI_INFO_COPYRIGHT: i32 = 8192;
pub const MCI_INFO_FILE: i32 = 512;
pub const MCI_INFO_MEDIA_IDENTITY: i32 = 2048;
pub const MCI_INFO_MEDIA_UPC: i32 = 1024;
pub const MCI_INFO_NAME: i32 = 4096;
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
pub const MCI_INFO_PRODUCT: i32 = 256;
pub const MCI_LAST: i32 = 4095;
pub const MCI_LOAD: i32 = 2128;
pub const MCI_LOAD_FILE: i32 = 256;
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
pub const MCI_MODE_NOT_READY: i32 = 524;
pub const MCI_MODE_OPEN: i32 = 530;
pub const MCI_MODE_PAUSE: i32 = 529;
pub const MCI_MODE_PLAY: i32 = 526;
pub const MCI_MODE_RECORD: i32 = 527;
pub const MCI_MODE_SEEK: i32 = 528;
pub const MCI_MODE_STOP: i32 = 525;
pub const MCI_NOTIFY: i32 = 1;
pub const MCI_NOTIFY_ABORTED: i32 = 4;
pub const MCI_NOTIFY_FAILURE: i32 = 8;
pub const MCI_NOTIFY_SUCCESSFUL: i32 = 1;
pub const MCI_NOTIFY_SUPERSEDED: i32 = 2;
pub const MCI_OPEN: i32 = 2051;
pub const MCI_OPEN_ALIAS: i32 = 1024;
pub const MCI_OPEN_ELEMENT: i32 = 512;
pub const MCI_OPEN_ELEMENT_ID: i32 = 2048;
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
pub const MCI_OPEN_SHAREABLE: i32 = 256;
pub const MCI_OPEN_TYPE: i32 = 8192;
pub const MCI_OPEN_TYPE_ID: i32 = 4096;
pub const MCI_OVLY_GETDEVCAPS_CAN_FREEZE: i32 = 16386;
pub const MCI_OVLY_GETDEVCAPS_CAN_STRETCH: i32 = 16385;
pub const MCI_OVLY_GETDEVCAPS_MAX_WINDOWS: i32 = 16387;
pub const MCI_OVLY_INFO_TEXT: i32 = 65536;
#[cfg(feature = "windef")]
pub type MCI_OVLY_LOAD_PARMS = MCI_OVLY_LOAD_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_LOAD_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
    pub rc: super::RECT,
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
    pub rc: super::RECT,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_LOAD_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OVLY_OPEN_PARENT: i32 = 131072;
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
    pub hWndParent: super::HWND,
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
    pub hWndParent: super::HWND,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OVLY_OPEN_WS: i32 = 65536;
pub const MCI_OVLY_PUT_DESTINATION: i32 = 262144;
pub const MCI_OVLY_PUT_FRAME: i32 = 524288;
pub const MCI_OVLY_PUT_SOURCE: i32 = 131072;
pub const MCI_OVLY_PUT_VIDEO: i32 = 1048576;
pub const MCI_OVLY_RECT: i32 = 65536;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MCI_OVLY_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::RECT,
}
#[cfg(feature = "windef")]
pub type MCI_OVLY_SAVE_PARMS = MCI_OVLY_SAVE_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: windows_sys::core::PCSTR,
    pub rc: super::RECT,
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
    pub rc: super::RECT,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OVLY_STATUS_HWND: i32 = 16385;
pub const MCI_OVLY_STATUS_STRETCH: i32 = 16386;
pub const MCI_OVLY_WHERE_DESTINATION: i32 = 262144;
pub const MCI_OVLY_WHERE_FRAME: i32 = 524288;
pub const MCI_OVLY_WHERE_SOURCE: i32 = 131072;
pub const MCI_OVLY_WHERE_VIDEO: i32 = 1048576;
pub const MCI_OVLY_WINDOW_DEFAULT: i32 = 0;
pub const MCI_OVLY_WINDOW_DISABLE_STRETCH: i32 = 2097152;
pub const MCI_OVLY_WINDOW_ENABLE_STRETCH: i32 = 1048576;
pub const MCI_OVLY_WINDOW_HWND: i32 = 65536;
#[cfg(feature = "windef")]
pub type MCI_OVLY_WINDOW_PARMS = MCI_OVLY_WINDOW_PARMSA;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MCI_OVLY_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::HWND,
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
    pub hWnd: super::HWND,
    pub nCmdShow: u32,
    pub lpstrText: windows_sys::core::PCWSTR,
}
#[cfg(feature = "windef")]
impl Default for MCI_OVLY_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_OVLY_WINDOW_STATE: i32 = 262144;
pub const MCI_OVLY_WINDOW_TEXT: i32 = 524288;
pub const MCI_PASTE: i32 = 2131;
pub const MCI_PAUSE: i32 = 2057;
pub const MCI_PLAY: i32 = 2054;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
pub const MCI_PUT: i32 = 2114;
pub const MCI_REALIZE: i32 = 2112;
pub const MCI_RECORD: i32 = 2063;
pub const MCI_RECORD_INSERT: i32 = 256;
pub const MCI_RECORD_OVERWRITE: i32 = 512;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_RECORD_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
pub const MCI_RESUME: i32 = 2133;
pub const MCI_SAVE: i32 = 2067;
pub const MCI_SAVE_FILE: i32 = 256;
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
pub const MCI_SEEK: i32 = 2055;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_SEEK_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
}
pub const MCI_SEEK_TO_END: i32 = 512;
pub const MCI_SEEK_TO_START: i32 = 256;
pub const MCI_SEQ_DIV_PPQN: i32 = 1216;
pub const MCI_SEQ_DIV_SMPTE_24: i32 = 1217;
pub const MCI_SEQ_DIV_SMPTE_25: i32 = 1218;
pub const MCI_SEQ_DIV_SMPTE_30: i32 = 1220;
pub const MCI_SEQ_DIV_SMPTE_30DROP: i32 = 1219;
pub const MCI_SEQ_FILE: i32 = 16386;
pub const MCI_SEQ_FORMAT_SONGPTR: i32 = 16385;
pub const MCI_SEQ_MAPPER: i32 = 65535;
pub const MCI_SEQ_MIDI: i32 = 16387;
pub const MCI_SEQ_NONE: i32 = 65533;
pub const MCI_SEQ_SET_MASTER: i32 = 524288;
pub const MCI_SEQ_SET_OFFSET: i32 = 16777216;
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
pub const MCI_SEQ_SET_PORT: i32 = 131072;
pub const MCI_SEQ_SET_SLAVE: i32 = 262144;
pub const MCI_SEQ_SET_TEMPO: i32 = 65536;
pub const MCI_SEQ_SMPTE: i32 = 16388;
pub const MCI_SEQ_STATUS_COPYRIGHT: i32 = 16396;
pub const MCI_SEQ_STATUS_DIVTYPE: i32 = 16394;
pub const MCI_SEQ_STATUS_MASTER: i32 = 16392;
pub const MCI_SEQ_STATUS_NAME: i32 = 16395;
pub const MCI_SEQ_STATUS_OFFSET: i32 = 16393;
pub const MCI_SEQ_STATUS_PORT: i32 = 16387;
pub const MCI_SEQ_STATUS_SLAVE: i32 = 16391;
pub const MCI_SEQ_STATUS_TEMPO: i32 = 16386;
pub const MCI_SET: i32 = 2061;
pub const MCI_SET_AUDIO: i32 = 2048;
pub const MCI_SET_AUDIO_ALL: i32 = 0;
pub const MCI_SET_AUDIO_LEFT: i32 = 1;
pub const MCI_SET_AUDIO_RIGHT: i32 = 2;
pub const MCI_SET_DOOR_CLOSED: i32 = 512;
pub const MCI_SET_DOOR_OPEN: i32 = 256;
pub const MCI_SET_OFF: i32 = 16384;
pub const MCI_SET_ON: i32 = 8192;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
}
pub const MCI_SET_TIME_FORMAT: i32 = 1024;
pub const MCI_SET_VIDEO: i32 = 4096;
pub const MCI_SPIN: i32 = 2060;
pub const MCI_STATUS_CURRENT_TRACK: i32 = 8;
pub const MCI_STATUS_ITEM: i32 = 256;
pub const MCI_STATUS_LENGTH: i32 = 1;
pub const MCI_STATUS_MEDIA_PRESENT: i32 = 5;
pub const MCI_STATUS_MODE: i32 = 4;
pub const MCI_STATUS_NUMBER_OF_TRACKS: i32 = 3;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATUS_PARMS {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
}
pub const MCI_STATUS_POSITION: i32 = 2;
pub const MCI_STATUS_READY: i32 = 7;
pub const MCI_STATUS_START: i32 = 512;
pub const MCI_STATUS_TIME_FORMAT: i32 = 6;
pub const MCI_STEP: i32 = 2062;
pub const MCI_STOP: i32 = 2056;
pub const MCI_SYSINFO: i32 = 2064;
pub const MCI_SYSINFO_INSTALLNAME: i32 = 2048;
pub const MCI_SYSINFO_NAME: i32 = 1024;
pub const MCI_SYSINFO_OPEN: i32 = 512;
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
pub const MCI_SYSINFO_QUANTITY: i32 = 256;
pub const MCI_TO: i32 = 8;
pub const MCI_TRACK: i32 = 16;
pub const MCI_UNFREEZE: i32 = 2117;
pub const MCI_UPDATE: i32 = 2132;
pub const MCI_USER_MESSAGES: i32 = 3072;
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
pub const MCI_VD_ESCAPE_STRING: i32 = 256;
pub const MCI_VD_FORMAT_TRACK: i32 = 16385;
pub const MCI_VD_GETDEVCAPS_CAN_REVERSE: i32 = 16386;
pub const MCI_VD_GETDEVCAPS_CAV: i32 = 131072;
pub const MCI_VD_GETDEVCAPS_CLV: i32 = 65536;
pub const MCI_VD_GETDEVCAPS_FAST_RATE: i32 = 16387;
pub const MCI_VD_GETDEVCAPS_NORMAL_RATE: i32 = 16389;
pub const MCI_VD_GETDEVCAPS_SLOW_RATE: i32 = 16388;
pub const MCI_VD_MEDIA_CAV: i32 = 1027;
pub const MCI_VD_MEDIA_CLV: i32 = 1026;
pub const MCI_VD_MEDIA_OTHER: i32 = 1028;
pub const MCI_VD_MODE_PARK: i32 = 1025;
pub const MCI_VD_PLAY_FAST: i32 = 131072;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_VD_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
pub const MCI_VD_PLAY_REVERSE: i32 = 65536;
pub const MCI_VD_PLAY_SCAN: i32 = 524288;
pub const MCI_VD_PLAY_SLOW: i32 = 1048576;
pub const MCI_VD_PLAY_SPEED: i32 = 262144;
pub const MCI_VD_SEEK_REVERSE: i32 = 65536;
pub const MCI_VD_SPIN_DOWN: i32 = 131072;
pub const MCI_VD_SPIN_UP: i32 = 65536;
pub const MCI_VD_STATUS_DISC_SIZE: i32 = 16390;
pub const MCI_VD_STATUS_FORWARD: i32 = 16387;
pub const MCI_VD_STATUS_MEDIA_TYPE: i32 = 16388;
pub const MCI_VD_STATUS_SIDE: i32 = 16389;
pub const MCI_VD_STATUS_SPEED: i32 = 16386;
pub const MCI_VD_STEP_FRAMES: i32 = 65536;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_VD_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
pub const MCI_VD_STEP_REVERSE: i32 = 131072;
pub const MCI_WAIT: i32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_WAVE_DELETE_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
pub const MCI_WAVE_GETDEVCAPS_INPUTS: i32 = 16385;
pub const MCI_WAVE_GETDEVCAPS_OUTPUTS: i32 = 16386;
pub const MCI_WAVE_INPUT: i32 = 4194304;
pub const MCI_WAVE_MAPPER: i32 = 1153;
pub const MCI_WAVE_OPEN_BUFFER: i32 = 65536;
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
pub const MCI_WAVE_OUTPUT: i32 = 8388608;
pub const MCI_WAVE_PCM: i32 = 1152;
pub const MCI_WAVE_SET_ANYINPUT: i32 = 67108864;
pub const MCI_WAVE_SET_ANYOUTPUT: i32 = 134217728;
pub const MCI_WAVE_SET_AVGBYTESPERSEC: i32 = 524288;
pub const MCI_WAVE_SET_BITSPERSAMPLE: i32 = 2097152;
pub const MCI_WAVE_SET_BLOCKALIGN: i32 = 1048576;
pub const MCI_WAVE_SET_CHANNELS: i32 = 131072;
pub const MCI_WAVE_SET_FORMATTAG: i32 = 65536;
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
pub const MCI_WAVE_SET_SAMPLESPERSEC: i32 = 262144;
pub const MCI_WAVE_STATUS_AVGBYTESPERSEC: i32 = 16388;
pub const MCI_WAVE_STATUS_BITSPERSAMPLE: i32 = 16390;
pub const MCI_WAVE_STATUS_BLOCKALIGN: i32 = 16389;
pub const MCI_WAVE_STATUS_CHANNELS: i32 = 16386;
pub const MCI_WAVE_STATUS_FORMATTAG: i32 = 16385;
pub const MCI_WAVE_STATUS_LEVEL: i32 = 16391;
pub const MCI_WAVE_STATUS_SAMPLESPERSEC: i32 = 16387;
pub const MCI_WHERE: i32 = 2115;
pub const MCI_WINDOW: i32 = 2113;
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
