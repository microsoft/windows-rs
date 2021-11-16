#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageA(flags: u32, lpinfo: *mut u32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageExA(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageExW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopA(lpszdesktop: super::super::Foundation::PSTR, lpszdevice: super::super::Foundation::PSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopExA(lpszdesktop: super::super::Foundation::PSTR, lpszdevice: super::super::Foundation::PSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopExW(lpszdesktop: super::super::Foundation::PWSTR, lpszdevice: super::super::Foundation::PWSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopW(lpszdesktop: super::super::Foundation::PWSTR, lpszdevice: super::super::Foundation::PWSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWindowStationA(lpwinsta: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWindowStationW(lpwinsta: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn EnumDesktopWindows(hdesktop: HDESK, lpfn: super::super::UI::WindowsAndMessaging::WNDENUMPROC, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDesktopsA(hwinsta: HWINSTA, lpenumfunc: DESKTOPENUMPROCA, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDesktopsW(hwinsta: HWINSTA, lpenumfunc: DESKTOPENUMPROCW, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumWindowStationsA(lpenumfunc: WINSTAENUMPROCA, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumWindowStationsW(lpenumfunc: WINSTAENUMPROCW, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    pub fn GetProcessWindowStation() -> HWINSTA;
    pub fn GetThreadDesktop(dwthreadid: u32) -> HDESK;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDesktopA(lpszdesktop: super::super::Foundation::PSTR, dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDesktopW(lpszdesktop: super::super::Foundation::PWSTR, dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenInputDesktop(dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWindowStationA(lpszwinsta: super::super::Foundation::PSTR, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HWINSTA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWindowStationW(lpszwinsta: super::super::Foundation::PWSTR, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HWINSTA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwitchDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
}
pub const BSF_ALLOWSFW: u32 = 128u32;
pub const BSF_FLUSHDISK: u32 = 4u32;
pub const BSF_FORCEIFHUNG: u32 = 32u32;
pub const BSF_IGNORECURRENTTASK: u32 = 2u32;
pub const BSF_NOHANG: u32 = 8u32;
pub const BSF_NOTIMEOUTIFNOTHUNG: u32 = 64u32;
pub const BSF_POSTMESSAGE: u32 = 16u32;
pub const BSF_QUERY: u32 = 1u32;
pub const BSF_SENDNOTIFYMESSAGE: u32 = 256u32;
pub const BSF_LUID: u32 = 1024u32;
pub const BSF_RETURNHDESK: u32 = 512u32;
pub const BSM_ALLCOMPONENTS: u32 = 0u32;
pub const BSM_ALLDESKTOPS: u32 = 16u32;
pub const BSM_APPLICATIONS: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BSMINFO {
    pub cbSize: u32,
    pub hdesk: HDESK,
    pub hwnd: super::super::Foundation::HWND,
    pub luid: super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BSMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BSMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type DESKTOPENUMPROCA = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type DESKTOPENUMPROCW = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
pub type HDESK = isize;
pub type HWINSTA = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct USEROBJECTFLAGS {
    pub fInherit: super::super::Foundation::BOOL,
    pub fReserved: super::super::Foundation::BOOL,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USEROBJECTFLAGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USEROBJECTFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const UOI_FLAGS: u32 = 1u32;
pub const UOI_HEAPSIZE: u32 = 5u32;
pub const UOI_IO: u32 = 6u32;
pub const UOI_NAME: u32 = 2u32;
pub const UOI_TYPE: u32 = 3u32;
pub const UOI_USER_SID: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type WINSTAENUMPROCA = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type WINSTAENUMPROCW = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
