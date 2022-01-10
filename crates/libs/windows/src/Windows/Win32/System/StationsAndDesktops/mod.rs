#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub type BROADCAST_SYSTEM_MESSAGE_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_ALLOWSFW: BROADCAST_SYSTEM_MESSAGE_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_FLUSHDISK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_FORCEIFHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_IGNORECURRENTTASK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_NOHANG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_NOTIMEOUTIFNOTHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_POSTMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_QUERY: BROADCAST_SYSTEM_MESSAGE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_SENDNOTIFYMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_LUID: BROADCAST_SYSTEM_MESSAGE_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSF_RETURNHDESK: BROADCAST_SYSTEM_MESSAGE_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub type BROADCAST_SYSTEM_MESSAGE_INFO = u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSM_ALLCOMPONENTS: BROADCAST_SYSTEM_MESSAGE_INFO = 0u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSM_ALLDESKTOPS: BROADCAST_SYSTEM_MESSAGE_INFO = 16u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const BSM_APPLICATIONS: BROADCAST_SYSTEM_MESSAGE_INFO = 8u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
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
impl ::core::fmt::Debug for BSMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BSMINFO").field("cbSize", &self.cbSize).field("hdesk", &self.hdesk).field("hwnd", &self.hwnd).field("luid", &self.luid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BSMINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BSMINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BSMINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BSMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BSMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BroadcastSystemMessageA<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(flags: u32, lpinfo: *mut u32, msg: u32, wparam: Param3, lparam: Param4) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BroadcastSystemMessageA(flags: u32, lpinfo: *mut u32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32;
        }
        ::core::mem::transmute(BroadcastSystemMessageA(::core::mem::transmute(flags), ::core::mem::transmute(lpinfo), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BroadcastSystemMessageExA<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: Param3, lparam: Param4, pbsminfo: *mut BSMINFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BroadcastSystemMessageExA(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
        }
        ::core::mem::transmute(BroadcastSystemMessageExA(::core::mem::transmute(flags), ::core::mem::transmute(lpinfo), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(pbsminfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BroadcastSystemMessageExW<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: Param3, lparam: Param4, pbsminfo: *mut BSMINFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BroadcastSystemMessageExW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
        }
        ::core::mem::transmute(BroadcastSystemMessageExW(::core::mem::transmute(flags), ::core::mem::transmute(lpinfo), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(pbsminfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BroadcastSystemMessageW<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: Param3, lparam: Param4) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BroadcastSystemMessageW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32;
        }
        ::core::mem::transmute(BroadcastSystemMessageW(::core::mem::transmute(flags), ::core::mem::transmute(lpinfo), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseDesktop<'a, Param0: ::windows::core::IntoParam<'a, HDESK>>(hdesktop: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseDesktop(hdesktop.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseWindowStation<'a, Param0: ::windows::core::IntoParam<'a, HWINSTA>>(hwinsta: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseWindowStation(hwinsta.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszdesktop: Param0, lpszdevice: Param1, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDesktopA(lpszdesktop: super::super::Foundation::PSTR, lpszdevice: super::super::Foundation::PSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK;
        }
        ::core::mem::transmute(CreateDesktopA(lpszdesktop.into_param().abi(), lpszdevice.into_param().abi(), ::core::mem::transmute(pdevmode), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(lpsa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszdesktop: Param0, lpszdevice: Param1, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDesktopExA(lpszdesktop: super::super::Foundation::PSTR, lpszdevice: super::super::Foundation::PSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
        }
        ::core::mem::transmute(CreateDesktopExA(lpszdesktop.into_param().abi(), lpszdevice.into_param().abi(), ::core::mem::transmute(pdevmode), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(lpsa), ::core::mem::transmute(ulheapsize), ::core::mem::transmute(pvoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszdesktop: Param0, lpszdevice: Param1, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDesktopExW(lpszdesktop: super::super::Foundation::PWSTR, lpszdevice: super::super::Foundation::PWSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
        }
        ::core::mem::transmute(CreateDesktopExW(lpszdesktop.into_param().abi(), lpszdevice.into_param().abi(), ::core::mem::transmute(pdevmode), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(lpsa), ::core::mem::transmute(ulheapsize), ::core::mem::transmute(pvoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszdesktop: Param0, lpszdevice: Param1, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDesktopW(lpszdesktop: super::super::Foundation::PWSTR, lpszdevice: super::super::Foundation::PWSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK;
        }
        ::core::mem::transmute(CreateDesktopW(lpszdesktop.into_param().abi(), lpszdevice.into_param().abi(), ::core::mem::transmute(pdevmode), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(lpsa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWindowStationA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpwinsta: Param0, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWindowStationA(lpwinsta: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA;
        }
        ::core::mem::transmute(CreateWindowStationA(lpwinsta.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(lpsa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWindowStationW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpwinsta: Param0, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWindowStationW(lpwinsta: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA;
        }
        ::core::mem::transmute(CreateWindowStationW(lpwinsta.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(lpsa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type DESKTOPENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type DESKTOPENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnumDesktopWindows<'a, Param0: ::windows::core::IntoParam<'a, HDESK>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(hdesktop: Param0, lpfn: super::super::UI::WindowsAndMessaging::WNDENUMPROC, lparam: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDesktopWindows(hdesktop: HDESK, lpfn: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumDesktopWindows(hdesktop.into_param().abi(), ::core::mem::transmute(lpfn), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDesktopsA<'a, Param0: ::windows::core::IntoParam<'a, HWINSTA>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(hwinsta: Param0, lpenumfunc: DESKTOPENUMPROCA, lparam: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDesktopsA(hwinsta: HWINSTA, lpenumfunc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumDesktopsA(hwinsta.into_param().abi(), ::core::mem::transmute(lpenumfunc), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDesktopsW<'a, Param0: ::windows::core::IntoParam<'a, HWINSTA>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(hwinsta: Param0, lpenumfunc: DESKTOPENUMPROCW, lparam: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDesktopsW(hwinsta: HWINSTA, lpenumfunc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumDesktopsW(hwinsta.into_param().abi(), ::core::mem::transmute(lpenumfunc), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumWindowStationsA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(lpenumfunc: WINSTAENUMPROCA, lparam: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumWindowStationsA(lpenumfunc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumWindowStationsA(::core::mem::transmute(lpenumfunc), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumWindowStationsW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(lpenumfunc: WINSTAENUMPROCW, lparam: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumWindowStationsW(lpenumfunc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumWindowStationsW(::core::mem::transmute(lpenumfunc), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
#[inline]
pub unsafe fn GetProcessWindowStation() -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessWindowStation() -> HWINSTA;
        }
        ::core::mem::transmute(GetProcessWindowStation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
#[inline]
pub unsafe fn GetThreadDesktop(dwthreadid: u32) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadDesktop(dwthreadid: u32) -> HDESK;
        }
        ::core::mem::transmute(GetThreadDesktop(::core::mem::transmute(dwthreadid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserObjectInformationA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hobj: Param0, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUserObjectInformationA(hobj.into_param().abi(), ::core::mem::transmute(nindex), ::core::mem::transmute(pvinfo), ::core::mem::transmute(nlength), ::core::mem::transmute(lpnlengthneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserObjectInformationW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hobj: Param0, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUserObjectInformationW(hobj.into_param().abi(), ::core::mem::transmute(nindex), ::core::mem::transmute(pvinfo), ::core::mem::transmute(nlength), ::core::mem::transmute(lpnlengthneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HDESK(pub isize);
impl HDESK {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HDESK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDESK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDESK {}
impl ::core::fmt::Debug for HDESK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDESK").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HDESK {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HWINSTA(pub isize);
impl HWINSTA {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HWINSTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWINSTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWINSTA {}
impl ::core::fmt::Debug for HWINSTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWINSTA").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HWINSTA {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDesktopA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszdesktop: Param0, dwflags: u32, finherit: Param2, dwdesiredaccess: u32) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenDesktopA(lpszdesktop: super::super::Foundation::PSTR, dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
        }
        ::core::mem::transmute(OpenDesktopA(lpszdesktop.into_param().abi(), ::core::mem::transmute(dwflags), finherit.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDesktopW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszdesktop: Param0, dwflags: u32, finherit: Param2, dwdesiredaccess: u32) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenDesktopW(lpszdesktop: super::super::Foundation::PWSTR, dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
        }
        ::core::mem::transmute(OpenDesktopW(lpszdesktop.into_param().abi(), ::core::mem::transmute(dwflags), finherit.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenInputDesktop<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(dwflags: u32, finherit: Param1, dwdesiredaccess: u32) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenInputDesktop(dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
        }
        ::core::mem::transmute(OpenInputDesktop(::core::mem::transmute(dwflags), finherit.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWindowStationA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszwinsta: Param0, finherit: Param1, dwdesiredaccess: u32) -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenWindowStationA(lpszwinsta: super::super::Foundation::PSTR, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HWINSTA;
        }
        ::core::mem::transmute(OpenWindowStationA(lpszwinsta.into_param().abi(), finherit.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWindowStationW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszwinsta: Param0, finherit: Param1, dwdesiredaccess: u32) -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenWindowStationW(lpszwinsta: super::super::Foundation::PWSTR, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HWINSTA;
        }
        ::core::mem::transmute(OpenWindowStationW(lpszwinsta.into_param().abi(), finherit.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessWindowStation<'a, Param0: ::windows::core::IntoParam<'a, HWINSTA>>(hwinsta: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessWindowStation(hwinsta.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadDesktop<'a, Param0: ::windows::core::IntoParam<'a, HDESK>>(hdesktop: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadDesktop(hdesktop.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUserObjectInformationA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hobj: Param0, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUserObjectInformationA(hobj.into_param().abi(), ::core::mem::transmute(nindex), ::core::mem::transmute(pvinfo), ::core::mem::transmute(nlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUserObjectInformationW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hobj: Param0, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUserObjectInformationW(hobj.into_param().abi(), ::core::mem::transmute(nindex), ::core::mem::transmute(pvinfo), ::core::mem::transmute(nlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwitchDesktop<'a, Param0: ::windows::core::IntoParam<'a, HDESK>>(hdesktop: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwitchDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SwitchDesktop(hdesktop.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USEROBJECTFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USEROBJECTFLAGS").field("fInherit", &self.fInherit).field("fReserved", &self.fReserved).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USEROBJECTFLAGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USEROBJECTFLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USEROBJECTFLAGS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USEROBJECTFLAGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USEROBJECTFLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub type USER_OBJECT_INFORMATION_INDEX = u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const UOI_FLAGS: USER_OBJECT_INFORMATION_INDEX = 1u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const UOI_HEAPSIZE: USER_OBJECT_INFORMATION_INDEX = 5u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const UOI_IO: USER_OBJECT_INFORMATION_INDEX = 6u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const UOI_NAME: USER_OBJECT_INFORMATION_INDEX = 2u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const UOI_TYPE: USER_OBJECT_INFORMATION_INDEX = 3u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops'*"]
pub const UOI_USER_SID: USER_OBJECT_INFORMATION_INDEX = 4u32;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type WINSTAENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_StationsAndDesktops', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type WINSTAENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
