#[cfg(all(feature = "shared", feature = "windef"))]
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
#[cfg(all(feature = "shared", feature = "windef"))]
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
#[cfg(all(feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn RasEntryDlgA<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut tagRASENTRYDLGA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasEntryDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpinfo : *mut tagRASENTRYDLGA) -> windows_core::BOOL);
    unsafe { RasEntryDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RasEntryDlgW<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut tagRASENTRYDLGW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasEntryDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpinfo : *mut tagRASENTRYDLGW) -> windows_core::BOOL);
    unsafe { RasEntryDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn RasPhonebookDlgA<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut tagRASPBDLGA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasPhonebookDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpinfo : *mut tagRASPBDLGA) -> windows_core::BOOL);
    unsafe { RasPhonebookDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn RasPhonebookDlgW<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut tagRASPBDLGW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rasdlg.dll" "system" fn RasPhonebookDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpinfo : *mut tagRASPBDLGW) -> windows_core::BOOL);
    unsafe { RasPhonebookDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo as _) }
}
pub const RASDDFLAG_AoacRedial: i32 = 4;
pub const RASDDFLAG_LinkFailure: u32 = 2147483648;
pub const RASDDFLAG_NoPrompt: i32 = 2;
pub const RASDDFLAG_PositionDlg: i32 = 1;
pub const RASEDFLAG_IncomingConnection: i32 = 1024;
pub const RASEDFLAG_InternetEntry: i32 = 256;
pub const RASEDFLAG_NAT: i32 = 512;
pub const RASEDFLAG_NewBroadbandEntry: i32 = 128;
pub const RASEDFLAG_NewEntry: i32 = 2;
pub const RASEDFLAG_NewPhoneEntry: i32 = 16;
pub const RASEDFLAG_NewTunnelEntry: i32 = 32;
pub const RASEDFLAG_NoRename: i32 = 8;
pub const RASEDFLAG_PositionDlg: i32 = 1;
pub const RASEDFLAG_ShellOwned: i32 = 1073741824;
pub const RASNOUSER_SmartCard: i32 = 1;
pub const RASPBDEVENT_AddEntry: i32 = 1;
pub const RASPBDEVENT_DialEntry: i32 = 4;
pub const RASPBDEVENT_EditEntry: i32 = 2;
pub const RASPBDEVENT_EditGlobals: i32 = 5;
pub const RASPBDEVENT_NoUser: i32 = 6;
pub const RASPBDEVENT_NoUserEdit: i32 = 7;
pub const RASPBDEVENT_RemoveEntry: i32 = 3;
pub const RASPBDFLAG_ForceCloseOnDial: i32 = 2;
pub const RASPBDFLAG_NoUser: i32 = 16;
pub const RASPBDFLAG_PositionDlg: i32 = 1;
pub const RASPBDFLAG_UpdateDefaults: u32 = 2147483648;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type RASPBDLGFUNCA = Option<unsafe extern "system" fn(param0: super::ULONG_PTR, param1: super::DWORD, param2: windows_core::PSTR, param3: super::LPVOID)>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type RASPBDLGFUNCW = Option<unsafe extern "system" fn(param0: super::ULONG_PTR, param1: super::DWORD, param2: windows_core::PWSTR, param3: super::LPVOID)>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type RasCustomDialDlgFn = Option<unsafe extern "system" fn(hinstdll: super::HINSTANCE, dwflags: super::DWORD, lpszphonebook: windows_core::PWSTR, lpszentry: windows_core::PWSTR, lpszphonenumber: windows_core::PWSTR, lpinfo: *mut tagRASDIALDLG, pvinfo: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type RasCustomEntryDlgFn = Option<unsafe extern "system" fn(hinstdll: super::HINSTANCE, lpszphonebook: windows_core::PWSTR, lpszentry: windows_core::PWSTR, lpinfo: *mut tagRASENTRYDLGA, dwflags: super::DWORD) -> windows_core::BOOL>;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagRASDIALDLG {
    pub dwSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub xDlg: super::LONG,
    pub yDlg: super::LONG,
    pub dwSubEntry: super::DWORD,
    pub dwError: super::DWORD,
    pub reserved: super::ULONG_PTR,
    pub reserved2: super::ULONG_PTR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct tagRASENTRYDLGA {
    pub dwSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub xDlg: super::LONG,
    pub yDlg: super::LONG,
    pub szEntry: [super::CHAR; 257],
    pub dwError: super::DWORD,
    pub reserved: super::ULONG_PTR,
    pub reserved2: super::ULONG_PTR,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for tagRASENTRYDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASENTRYDLGW {
    pub dwSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub xDlg: super::LONG,
    pub yDlg: super::LONG,
    pub szEntry: [super::WCHAR; 257],
    pub dwError: super::DWORD,
    pub reserved: super::ULONG_PTR,
    pub reserved2: super::ULONG_PTR,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for tagRASENTRYDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASNOUSERA {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwTimeoutMs: super::DWORD,
    pub szUserName: [super::CHAR; 257],
    pub szPassword: [super::CHAR; 257],
    pub szDomain: [super::CHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for tagRASNOUSERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASNOUSERW {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwTimeoutMs: super::DWORD,
    pub szUserName: [super::WCHAR; 257],
    pub szPassword: [super::WCHAR; 257],
    pub szDomain: [super::WCHAR; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASNOUSERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagRASPBDLGA {
    pub dwSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub xDlg: super::LONG,
    pub yDlg: super::LONG,
    pub dwCallbackId: super::ULONG_PTR,
    pub pCallback: RASPBDLGFUNCA,
    pub dwError: super::DWORD,
    pub reserved: super::ULONG_PTR,
    pub reserved2: super::ULONG_PTR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagRASPBDLGW {
    pub dwSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub xDlg: super::LONG,
    pub yDlg: super::LONG,
    pub dwCallbackId: super::ULONG_PTR,
    pub pCallback: RASPBDLGFUNCW,
    pub dwError: super::DWORD,
    pub reserved: super::ULONG_PTR,
    pub reserved2: super::ULONG_PTR,
}
