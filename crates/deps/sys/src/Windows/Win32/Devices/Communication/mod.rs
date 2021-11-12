#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBA(lpdef: super::super::Foundation::PSTR, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBAndTimeoutsA(lpdef: super::super::Foundation::PSTR, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBAndTimeoutsW(lpdef: super::super::Foundation::PWSTR, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBW(lpdef: super::super::Foundation::PWSTR, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearCommBreak(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearCommError(hfile: super::super::Foundation::HANDLE, lperrors: *mut CLEAR_COMM_ERROR_FLAGS, lpstat: *mut COMSTAT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommConfigDialogA(lpszname: super::super::Foundation::PSTR, hwnd: super::super::Foundation::HWND, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommConfigDialogW(lpszname: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EscapeCommFunction(hfile: super::super::Foundation::HANDLE, dwfunc: ESCAPE_COMM_FUNCTION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommMask(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommModemStatus(hfile: super::super::Foundation::HANDLE, lpmodemstat: *mut MODEM_STATUS_FLAGS) -> super::super::Foundation::BOOL;
    pub fn GetCommPorts(lpportnumbers: *mut u32, uportnumberscount: u32, puportnumbersfound: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommProperties(hfile: super::super::Foundation::HANDLE, lpcommprop: *mut COMMPROP) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultCommConfigA(lpszname: super::super::Foundation::PSTR, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultCommConfigW(lpszname: super::super::Foundation::PWSTR, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenCommPort(uportnumber: u32, dwdesiredaccess: u32, dwflagsandattributes: u32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PurgeComm(hfile: super::super::Foundation::HANDLE, dwflags: PURGE_COMM_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommBreak(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommMask(hfile: super::super::Foundation::HANDLE, dwevtmask: COMM_EVENT_MASK) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *const DCB) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *const COMMTIMEOUTS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultCommConfigA(lpszname: super::super::Foundation::PSTR, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultCommConfigW(lpszname: super::super::Foundation::PWSTR, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupComm(hfile: super::super::Foundation::HANDLE, dwinqueue: u32, dwoutqueue: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TransmitCommChar(hfile: super::super::Foundation::HANDLE, cchar: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WaitCommEvent(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct CLEAR_COMM_ERROR_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COMMCONFIG(i32);
#[repr(C)]
pub struct COMMPROP(i32);
#[repr(C)]
pub struct COMMPROP_STOP_PARITY(i32);
#[repr(C)]
pub struct COMMTIMEOUTS(i32);
#[repr(C)]
pub struct COMM_EVENT_MASK(i32);
#[repr(C)]
pub struct COMSTAT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DCB(i32);
#[repr(C)]
pub struct ESCAPE_COMM_FUNCTION(i32);
pub const MAXLENGTH_NAI: u32 = 72u32;
pub const MAXLENGTH_UICCDATASTORE: u32 = 10u32;
pub const MDM_ANALOG_RLP_OFF: u32 = 1u32;
pub const MDM_ANALOG_RLP_ON: u32 = 0u32;
pub const MDM_ANALOG_V34: u32 = 2u32;
pub const MDM_AUTO_ML_2: u32 = 2u32;
pub const MDM_AUTO_ML_DEFAULT: u32 = 0u32;
pub const MDM_AUTO_ML_NONE: u32 = 1u32;
pub const MDM_AUTO_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_BEARERMODE_ANALOG: u32 = 0u32;
pub const MDM_BEARERMODE_GSM: u32 = 2u32;
pub const MDM_BEARERMODE_ISDN: u32 = 1u32;
pub const MDM_BLIND_DIAL: u32 = 512u32;
pub const MDM_CCITT_OVERRIDE: u32 = 64u32;
pub const MDM_CELLULAR: u32 = 8u32;
pub const MDM_COMPRESSION: u32 = 1u32;
pub const MDM_DIAGNOSTICS: u32 = 2048u32;
pub const MDM_ERROR_CONTROL: u32 = 2u32;
pub const MDM_FLOWCONTROL_HARD: u32 = 16u32;
pub const MDM_FLOWCONTROL_SOFT: u32 = 32u32;
pub const MDM_FORCED_EC: u32 = 4u32;
pub const MDM_HDLCPPP_AUTH_CHAP: u32 = 3u32;
pub const MDM_HDLCPPP_AUTH_DEFAULT: u32 = 0u32;
pub const MDM_HDLCPPP_AUTH_MSCHAP: u32 = 4u32;
pub const MDM_HDLCPPP_AUTH_NONE: u32 = 1u32;
pub const MDM_HDLCPPP_AUTH_PAP: u32 = 2u32;
pub const MDM_HDLCPPP_ML_2: u32 = 2u32;
pub const MDM_HDLCPPP_ML_DEFAULT: u32 = 0u32;
pub const MDM_HDLCPPP_ML_NONE: u32 = 1u32;
pub const MDM_HDLCPPP_SPEED_56K: u32 = 2u32;
pub const MDM_HDLCPPP_SPEED_64K: u32 = 1u32;
pub const MDM_HDLCPPP_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_MASK_AUTO_SPEED: u32 = 7u32;
pub const MDM_MASK_BEARERMODE: u32 = 61440u32;
pub const MDM_MASK_HDLCPPP_SPEED: u32 = 7u32;
pub const MDM_MASK_PROTOCOLDATA: u32 = 267386880u32;
pub const MDM_MASK_PROTOCOLID: u32 = 983040u32;
pub const MDM_MASK_V110_SPEED: u32 = 15u32;
pub const MDM_MASK_V120_SPEED: u32 = 7u32;
pub const MDM_MASK_X75_DATA: u32 = 7u32;
pub const MDM_PIAFS_INCOMING: u32 = 0u32;
pub const MDM_PIAFS_OUTGOING: u32 = 1u32;
pub const MDM_PROTOCOLID_ANALOG: u32 = 7u32;
pub const MDM_PROTOCOLID_AUTO: u32 = 6u32;
pub const MDM_PROTOCOLID_DEFAULT: u32 = 0u32;
pub const MDM_PROTOCOLID_GPRS: u32 = 8u32;
pub const MDM_PROTOCOLID_HDLCPPP: u32 = 1u32;
pub const MDM_PROTOCOLID_PIAFS: u32 = 9u32;
pub const MDM_PROTOCOLID_V110: u32 = 4u32;
pub const MDM_PROTOCOLID_V120: u32 = 5u32;
pub const MDM_PROTOCOLID_V128: u32 = 2u32;
pub const MDM_PROTOCOLID_X75: u32 = 3u32;
pub const MDM_SHIFT_AUTO_ML: u32 = 6u32;
pub const MDM_SHIFT_AUTO_SPEED: u32 = 0u32;
pub const MDM_SHIFT_BEARERMODE: u32 = 12u32;
pub const MDM_SHIFT_EXTENDEDINFO: u32 = 12u32;
pub const MDM_SHIFT_HDLCPPP_AUTH: u32 = 3u32;
pub const MDM_SHIFT_HDLCPPP_ML: u32 = 6u32;
pub const MDM_SHIFT_HDLCPPP_SPEED: u32 = 0u32;
pub const MDM_SHIFT_PROTOCOLDATA: u32 = 20u32;
pub const MDM_SHIFT_PROTOCOLID: u32 = 16u32;
pub const MDM_SHIFT_PROTOCOLINFO: u32 = 16u32;
pub const MDM_SHIFT_V110_SPEED: u32 = 0u32;
pub const MDM_SHIFT_V120_ML: u32 = 6u32;
pub const MDM_SHIFT_V120_SPEED: u32 = 0u32;
pub const MDM_SHIFT_X75_DATA: u32 = 0u32;
pub const MDM_SPEED_ADJUST: u32 = 128u32;
pub const MDM_TONE_DIAL: u32 = 256u32;
pub const MDM_V110_SPEED_12DOT0K: u32 = 5u32;
pub const MDM_V110_SPEED_14DOT4K: u32 = 6u32;
pub const MDM_V110_SPEED_19DOT2K: u32 = 7u32;
pub const MDM_V110_SPEED_1DOT2K: u32 = 1u32;
pub const MDM_V110_SPEED_28DOT8K: u32 = 8u32;
pub const MDM_V110_SPEED_2DOT4K: u32 = 2u32;
pub const MDM_V110_SPEED_38DOT4K: u32 = 9u32;
pub const MDM_V110_SPEED_4DOT8K: u32 = 3u32;
pub const MDM_V110_SPEED_57DOT6K: u32 = 10u32;
pub const MDM_V110_SPEED_9DOT6K: u32 = 4u32;
pub const MDM_V110_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_V120_ML_2: u32 = 2u32;
pub const MDM_V120_ML_DEFAULT: u32 = 0u32;
pub const MDM_V120_ML_NONE: u32 = 1u32;
pub const MDM_V120_SPEED_56K: u32 = 2u32;
pub const MDM_V120_SPEED_64K: u32 = 1u32;
pub const MDM_V120_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_V23_OVERRIDE: u32 = 1024u32;
pub const MDM_X75_DATA_128K: u32 = 2u32;
pub const MDM_X75_DATA_64K: u32 = 1u32;
pub const MDM_X75_DATA_BTX: u32 = 4u32;
pub const MDM_X75_DATA_DEFAULT: u32 = 0u32;
pub const MDM_X75_DATA_T_70: u32 = 3u32;
#[repr(C)]
pub struct MODEMDEVCAPS(i32);
#[repr(C)]
pub struct MODEMDEVCAPS_DIAL_OPTIONS(i32);
#[repr(C)]
pub struct MODEMDEVCAPS_SPEAKER_MODE(i32);
#[repr(C)]
pub struct MODEMDEVCAPS_SPEAKER_VOLUME(i32);
#[repr(C)]
pub struct MODEMSETTINGS(i32);
#[repr(C)]
pub struct MODEMSETTINGS_SPEAKER_MODE(i32);
#[repr(C)]
pub struct MODEM_SPEAKER_VOLUME(i32);
#[repr(C)]
pub struct MODEM_STATUS_FLAGS(i32);
#[repr(C)]
pub struct PURGE_COMM_FLAGS(i32);
pub const SID_3GPP_SUPSVCMODEL: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3620769287,
    data2: 55143,
    data3: 17528,
    data4: [177, 74, 238, 204, 135, 234, 18, 247],
};
