#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RasDialDlgA<P0, P1, P2>(lpszphonebook: P0, lpszentry: P1, lpszphonenumber: P2, lpinfo: *mut tagRASDIALDLG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasDialDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpszphonenumber : windows_core::PCSTR, lpinfo : *mut tagRASDIALDLG) -> windows_core::BOOL);
    unsafe { RasDialDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpszphonenumber.param().abi(), lpinfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RasDialDlgW<P0, P1, P2>(lpszphonebook: P0, lpszentry: P1, lpszphonenumber: P2, lpinfo: *mut tagRASDIALDLG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasDialDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpszphonenumber : windows_core::PCWSTR, lpinfo : *mut tagRASDIALDLG) -> windows_core::BOOL);
    unsafe { RasDialDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpszphonenumber.param().abi(), lpinfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RasEntryDlgA<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut tagRASENTRYDLGA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasEntryDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpinfo : *mut tagRASENTRYDLGA) -> windows_core::BOOL);
    unsafe { RasEntryDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RasEntryDlgW<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut tagRASENTRYDLGW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasEntryDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpinfo : *mut tagRASENTRYDLGW) -> windows_core::BOOL);
    unsafe { RasEntryDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RasPhonebookDlgA<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut tagRASPBDLGA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasPhonebookDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpinfo : *mut tagRASPBDLGA) -> windows_core::BOOL);
    unsafe { RasPhonebookDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RasPhonebookDlgW<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut tagRASPBDLGW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasPhonebookDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpinfo : *mut tagRASPBDLGW) -> windows_core::BOOL);
    unsafe { RasPhonebookDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
pub const RASDDFLAG_AoacRedial: u32 = 4;
pub const RASDDFLAG_LinkFailure: u32 = 2147483648;
pub const RASDDFLAG_NoPrompt: u32 = 2;
pub const RASDDFLAG_PositionDlg: u32 = 1;
pub const RASEDFLAG_IncomingConnection: u32 = 1024;
pub const RASEDFLAG_InternetEntry: u32 = 256;
pub const RASEDFLAG_NAT: u32 = 512;
pub const RASEDFLAG_NewBroadbandEntry: u32 = 128;
pub const RASEDFLAG_NewEntry: u32 = 2;
pub const RASEDFLAG_NewPhoneEntry: u32 = 16;
pub const RASEDFLAG_NewTunnelEntry: u32 = 32;
pub const RASEDFLAG_NoRename: u32 = 8;
pub const RASEDFLAG_PositionDlg: u32 = 1;
pub const RASEDFLAG_ShellOwned: u32 = 1073741824;
pub const RASNOUSER_SmartCard: u32 = 1;
pub const RASPBDEVENT_AddEntry: u32 = 1;
pub const RASPBDEVENT_DialEntry: u32 = 4;
pub const RASPBDEVENT_EditEntry: u32 = 2;
pub const RASPBDEVENT_EditGlobals: u32 = 5;
pub const RASPBDEVENT_NoUser: u32 = 6;
pub const RASPBDEVENT_NoUserEdit: u32 = 7;
pub const RASPBDEVENT_RemoveEntry: u32 = 3;
pub const RASPBDFLAG_ForceCloseOnDial: u32 = 2;
pub const RASPBDFLAG_NoUser: u32 = 16;
pub const RASPBDFLAG_PositionDlg: u32 = 1;
pub const RASPBDFLAG_UpdateDefaults: u32 = 2147483648;
pub type RASPBDLGFUNCA = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: windows_core::PCSTR, param3: *mut core::ffi::c_void)>;
pub type RASPBDLGFUNCW = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: windows_core::PCWSTR, param3: *mut core::ffi::c_void)>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type RasCustomDialDlgFn = Option<unsafe extern "system" fn(hinstdll: super::minwindef::HINSTANCE, dwflags: u32, lpszphonebook: windows_core::PCWSTR, lpszentry: windows_core::PCWSTR, lpszphonenumber: windows_core::PCWSTR, lpinfo: *mut tagRASDIALDLG, pvinfo: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type RasCustomEntryDlgFn = Option<unsafe extern "system" fn(hinstdll: super::minwindef::HINSTANCE, lpszphonebook: windows_core::PCWSTR, lpszentry: windows_core::PCWSTR, lpinfo: *mut tagRASENTRYDLGA, dwflags: u32) -> windows_core::BOOL>;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct tagRASDIALDLG {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwSubEntry: u32,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct tagRASDIALDLG {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwSubEntry: u32,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tagRASENTRYDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [i8; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
impl Default for tagRASENTRYDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct tagRASENTRYDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [i8; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
impl Default for tagRASENTRYDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tagRASENTRYDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [u16; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
impl Default for tagRASENTRYDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct tagRASENTRYDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [u16; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
impl Default for tagRASENTRYDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tagRASNOUSERA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
}
impl Default for tagRASNOUSERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct tagRASNOUSERW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl Default for tagRASNOUSERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default)]
pub struct tagRASPBDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCA,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct tagRASPBDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCA,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default)]
pub struct tagRASPBDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCW,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct tagRASPBDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCW,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
