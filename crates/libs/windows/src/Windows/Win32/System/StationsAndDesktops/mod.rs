#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BROADCAST_SYSTEM_MESSAGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_ALLOWSFW: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_FLUSHDISK: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_FORCEIFHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_IGNORECURRENTTASK: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_NOHANG: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_NOTIMEOUTIFNOTHUNG: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_POSTMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_QUERY: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_SENDNOTIFYMESSAGE: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_LUID: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSF_RETURNHDESK: BROADCAST_SYSTEM_MESSAGE_FLAGS = BROADCAST_SYSTEM_MESSAGE_FLAGS(512u32);
impl ::core::marker::Copy for BROADCAST_SYSTEM_MESSAGE_FLAGS {}
impl ::core::clone::Clone for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BROADCAST_SYSTEM_MESSAGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BROADCAST_SYSTEM_MESSAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BROADCAST_SYSTEM_MESSAGE_INFO(pub u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSM_ALLCOMPONENTS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(0u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSM_ALLDESKTOPS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(16u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const BSM_APPLICATIONS: BROADCAST_SYSTEM_MESSAGE_INFO = BROADCAST_SYSTEM_MESSAGE_INFO(8u32);
impl ::core::marker::Copy for BROADCAST_SYSTEM_MESSAGE_INFO {}
impl ::core::clone::Clone for BROADCAST_SYSTEM_MESSAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BROADCAST_SYSTEM_MESSAGE_INFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BROADCAST_SYSTEM_MESSAGE_INFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for BROADCAST_SYSTEM_MESSAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BROADCAST_SYSTEM_MESSAGE_INFO").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BROADCAST_SYSTEM_MESSAGE_INFO {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BROADCAST_SYSTEM_MESSAGE_INFO {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BROADCAST_SYSTEM_MESSAGE_INFO {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BROADCAST_SYSTEM_MESSAGE_INFO {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BROADCAST_SYSTEM_MESSAGE_INFO {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BroadcastSystemMessageA<'a, P0, P1>(flags: u32, lpinfo: *mut u32, msg: u32, wparam: P0, lparam: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BroadcastSystemMessageA(flags: u32, lpinfo: *mut u32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    BroadcastSystemMessageA(flags, ::core::mem::transmute(lpinfo), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BroadcastSystemMessageExA<'a, P0, P1>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: P0, lparam: P1, pbsminfo: *mut BSMINFO) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BroadcastSystemMessageExA(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
    }
    BroadcastSystemMessageExA(flags, ::core::mem::transmute(lpinfo), msg, wparam.into(), lparam.into(), ::core::mem::transmute(pbsminfo))
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BroadcastSystemMessageExW<'a, P0, P1>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: P0, lparam: P1, pbsminfo: *mut BSMINFO) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BroadcastSystemMessageExW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
    }
    BroadcastSystemMessageExW(flags, ::core::mem::transmute(lpinfo), msg, wparam.into(), lparam.into(), ::core::mem::transmute(pbsminfo))
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BroadcastSystemMessageW<'a, P0, P1>(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: P0, lparam: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BroadcastSystemMessageW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32;
    }
    BroadcastSystemMessageW(flags, ::core::mem::transmute(lpinfo), msg, wparam.into(), lparam.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseDesktop<'a, P0>(hdesktop: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDESK>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
    }
    CloseDesktop(hdesktop.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseWindowStation<'a, P0>(hwinsta: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HWINSTA>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
    }
    CloseWindowStation(hwinsta.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopA<'a, P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<HDESK>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDesktopA(lpszdesktop: ::windows::core::PCSTR, lpszdevice: ::windows::core::PCSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK;
    }
    let result__ = CreateDesktopA(lpszdesktop.into(), lpszdevice.into(), ::core::mem::transmute(pdevmode), dwflags, dwdesiredaccess, ::core::mem::transmute(lpsa));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopExA<'a, P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::Result<HDESK>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDesktopExA(lpszdesktop: ::windows::core::PCSTR, lpszdevice: ::windows::core::PCSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
    }
    let result__ = CreateDesktopExA(lpszdesktop.into(), lpszdevice.into(), ::core::mem::transmute(pdevmode), dwflags, dwdesiredaccess, ::core::mem::transmute(lpsa), ulheapsize, ::core::mem::transmute(pvoid));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopExW<'a, P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::Result<HDESK>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDesktopExW(lpszdesktop: ::windows::core::PCWSTR, lpszdevice: ::windows::core::PCWSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
    }
    let result__ = CreateDesktopExW(lpszdesktop.into(), lpszdevice.into(), ::core::mem::transmute(pdevmode), dwflags, dwdesiredaccess, ::core::mem::transmute(lpsa), ulheapsize, ::core::mem::transmute(pvoid));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDesktopW<'a, P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<HDESK>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDesktopW(lpszdesktop: ::windows::core::PCWSTR, lpszdevice: ::windows::core::PCWSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK;
    }
    let result__ = CreateDesktopW(lpszdesktop.into(), lpszdevice.into(), ::core::mem::transmute(pdevmode), dwflags, dwdesiredaccess, ::core::mem::transmute(lpsa));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWindowStationA<'a, P0>(lpwinsta: P0, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<HWINSTA>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateWindowStationA(lpwinsta: ::windows::core::PCSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA;
    }
    let result__ = CreateWindowStationA(lpwinsta.into(), dwflags, dwdesiredaccess, ::core::mem::transmute(lpsa));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWindowStationW<'a, P0>(lpwinsta: P0, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<HWINSTA>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateWindowStationW(lpwinsta: ::windows::core::PCWSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA;
    }
    let result__ = CreateWindowStationW(lpwinsta.into(), dwflags, dwdesiredaccess, ::core::mem::transmute(lpsa));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DESKTOPENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DESKTOPENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnumDesktopWindows<'a, P0, P1>(hdesktop: P0, lpfn: super::super::UI::WindowsAndMessaging::WNDENUMPROC, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDESK>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDesktopWindows(hdesktop: HDESK, lpfn: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumDesktopWindows(hdesktop.into(), ::core::mem::transmute(lpfn), lparam.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDesktopsA<'a, P0, P1>(hwinsta: P0, lpenumfunc: DESKTOPENUMPROCA, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HWINSTA>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDesktopsA(hwinsta: HWINSTA, lpenumfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumDesktopsA(hwinsta.into(), ::core::mem::transmute(lpenumfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDesktopsW<'a, P0, P1>(hwinsta: P0, lpenumfunc: DESKTOPENUMPROCW, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HWINSTA>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDesktopsW(hwinsta: HWINSTA, lpenumfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumDesktopsW(hwinsta.into(), ::core::mem::transmute(lpenumfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumWindowStationsA<'a, P0>(lpenumfunc: WINSTAENUMPROCA, lparam: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumWindowStationsA(lpenumfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumWindowStationsA(::core::mem::transmute(lpenumfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumWindowStationsW<'a, P0>(lpenumfunc: WINSTAENUMPROCW, lparam: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumWindowStationsW(lpenumfunc: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    EnumWindowStationsW(::core::mem::transmute(lpenumfunc), lparam.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
#[inline]
pub unsafe fn GetProcessWindowStation() -> ::windows::core::Result<HWINSTA> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetProcessWindowStation() -> HWINSTA;
    }
    let result__ = GetProcessWindowStation();
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
#[inline]
pub unsafe fn GetThreadDesktop(dwthreadid: u32) -> ::windows::core::Result<HDESK> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetThreadDesktop(dwthreadid: u32) -> HDESK;
    }
    let result__ = GetThreadDesktop(dwthreadid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserObjectInformationA<'a, P0>(hobj: P0, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetUserObjectInformationA(hobj.into(), nindex, ::core::mem::transmute(pvinfo), nlength, ::core::mem::transmute(lpnlengthneeded))
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserObjectInformationW<'a, P0>(hobj: P0, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetUserObjectInformationW(hobj.into(), nindex, ::core::mem::transmute(pvinfo), nlength, ::core::mem::transmute(lpnlengthneeded))
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDESK(pub isize);
impl HDESK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::core::convert::From<::core::option::Option<HDESK>> for HDESK {
    fn from(optional: ::core::option::Option<HDESK>) -> HDESK {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HDESK {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HWINSTA(pub isize);
impl HWINSTA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::core::convert::From<::core::option::Option<HWINSTA>> for HWINSTA {
    fn from(optional: ::core::option::Option<HWINSTA>) -> HWINSTA {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HWINSTA {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDesktopA<'a, P0, P1>(lpszdesktop: P0, dwflags: u32, finherit: P1, dwdesiredaccess: u32) -> ::windows::core::Result<HDESK>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenDesktopA(lpszdesktop: ::windows::core::PCSTR, dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    }
    let result__ = OpenDesktopA(lpszdesktop.into(), dwflags, finherit.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDesktopW<'a, P0, P1>(lpszdesktop: P0, dwflags: u32, finherit: P1, dwdesiredaccess: u32) -> ::windows::core::Result<HDESK>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenDesktopW(lpszdesktop: ::windows::core::PCWSTR, dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    }
    let result__ = OpenDesktopW(lpszdesktop.into(), dwflags, finherit.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenInputDesktop<'a, P0>(dwflags: u32, finherit: P0, dwdesiredaccess: u32) -> ::windows::core::Result<HDESK>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenInputDesktop(dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    }
    let result__ = OpenInputDesktop(dwflags, finherit.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWindowStationA<'a, P0, P1>(lpszwinsta: P0, finherit: P1, dwdesiredaccess: u32) -> ::windows::core::Result<HWINSTA>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenWindowStationA(lpszwinsta: ::windows::core::PCSTR, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HWINSTA;
    }
    let result__ = OpenWindowStationA(lpszwinsta.into(), finherit.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWindowStationW<'a, P0, P1>(lpszwinsta: P0, finherit: P1, dwdesiredaccess: u32) -> ::windows::core::Result<HWINSTA>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenWindowStationW(lpszwinsta: ::windows::core::PCWSTR, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HWINSTA;
    }
    let result__ = OpenWindowStationW(lpszwinsta.into(), finherit.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessWindowStation<'a, P0>(hwinsta: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HWINSTA>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetProcessWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
    }
    SetProcessWindowStation(hwinsta.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadDesktop<'a, P0>(hdesktop: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDESK>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetThreadDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
    }
    SetThreadDesktop(hdesktop.into())
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUserObjectInformationA<'a, P0>(hobj: P0, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL;
    }
    SetUserObjectInformationA(hobj.into(), nindex, ::core::mem::transmute(pvinfo), nlength)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUserObjectInformationW<'a, P0>(hobj: P0, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL;
    }
    SetUserObjectInformationW(hobj.into(), nindex, ::core::mem::transmute(pvinfo), nlength)
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwitchDesktop<'a, P0>(hdesktop: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDESK>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SwitchDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
    }
    SwitchDesktop(hdesktop.into())
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USER_OBJECT_INFORMATION_INDEX(pub u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const UOI_FLAGS: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(1u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const UOI_HEAPSIZE: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(5u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const UOI_IO: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(6u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const UOI_NAME: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(2u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const UOI_TYPE: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(3u32);
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`*"]
pub const UOI_USER_SID: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(4u32);
impl ::core::marker::Copy for USER_OBJECT_INFORMATION_INDEX {}
impl ::core::clone::Clone for USER_OBJECT_INFORMATION_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_OBJECT_INFORMATION_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for USER_OBJECT_INFORMATION_INDEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for USER_OBJECT_INFORMATION_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_OBJECT_INFORMATION_INDEX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WINSTAENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_StationsAndDesktops\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WINSTAENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
