#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBA(lpdef: super::super::Foundation::PSTR, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBAndTimeoutsA(lpdef: super::super::Foundation::PSTR, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBAndTimeoutsW(lpdef: super::super::Foundation::PWSTR, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildCommDCBW(lpdef: super::super::Foundation::PWSTR, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearCommBreak(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClearCommError(hfile: super::super::Foundation::HANDLE, lperrors: *mut CLEAR_COMM_ERROR_FLAGS, lpstat: *mut COMSTAT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommConfigDialogA(lpszname: super::super::Foundation::PSTR, hwnd: super::super::Foundation::HWND, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommConfigDialogW(lpszname: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EscapeCommFunction(hfile: super::super::Foundation::HANDLE, dwfunc: ESCAPE_COMM_FUNCTION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommMask(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommModemStatus(hfile: super::super::Foundation::HANDLE, lpmodemstat: *mut MODEM_STATUS_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`*"]
    pub fn GetCommPorts(lpportnumbers: *mut u32, uportnumberscount: u32, puportnumbersfound: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommProperties(hfile: super::super::Foundation::HANDLE, lpcommprop: *mut COMMPROP) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultCommConfigA(lpszname: super::super::Foundation::PSTR, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultCommConfigW(lpszname: super::super::Foundation::PWSTR, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenCommPort(uportnumber: u32, dwdesiredaccess: u32, dwflagsandattributes: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PurgeComm(hfile: super::super::Foundation::HANDLE, dwflags: PURGE_COMM_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommBreak(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommMask(hfile: super::super::Foundation::HANDLE, dwevtmask: COMM_EVENT_MASK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *const DCB) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *const COMMTIMEOUTS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultCommConfigA(lpszname: super::super::Foundation::PSTR, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultCommConfigW(lpszname: super::super::Foundation::PWSTR, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupComm(hfile: super::super::Foundation::HANDLE, dwinqueue: u32, dwoutqueue: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TransmitCommChar(hfile: super::super::Foundation::HANDLE, cchar: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Communication`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WaitCommEvent(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
}
