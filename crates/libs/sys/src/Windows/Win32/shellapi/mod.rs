#[cfg(feature = "Win32_minwindef")]
windows_link::link!("shell32.dll" "system" fn AssocCreateForClasses(rgclasses : *const ASSOCIATIONELEMENT, cclasses : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn CommandLineToArgvW(lpcmdline : windows_sys::core::PCWSTR, pnumargs : *mut i32) -> *mut windows_sys::core::PWSTR);
windows_link::link!("shell32.dll" "system" fn DoEnvironmentSubstA(pszsrc : windows_sys::core::PSTR, cchsrc : u32) -> u32);
windows_link::link!("shell32.dll" "system" fn DoEnvironmentSubstW(pszsrc : windows_sys::core::PWSTR, cchsrc : u32) -> u32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn DragAcceptFiles(hwnd : super::windef::HWND, faccept : windows_sys::core::BOOL));
windows_link::link!("shell32.dll" "system" fn DragFinish(hdrop : HDROP));
windows_link::link!("shell32.dll" "system" fn DragQueryFileA(hdrop : HDROP, ifile : u32, lpszfile : windows_sys::core::PSTR, cch : u32) -> u32);
windows_link::link!("shell32.dll" "system" fn DragQueryFileW(hdrop : HDROP, ifile : u32, lpszfile : windows_sys::core::PWSTR, cch : u32) -> u32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn DragQueryPoint(hdrop : HDROP, ppt : *mut super::windef::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn DuplicateIcon(hinst : super::minwindef::HINSTANCE, hicon : super::windef::HICON) -> super::windef::HICON);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn ExtractAssociatedIconA(hinst : super::minwindef::HINSTANCE, psziconpath : windows_sys::core::PSTR, piicon : *mut u16) -> super::windef::HICON);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn ExtractAssociatedIconExA(hinst : super::minwindef::HINSTANCE, psziconpath : windows_sys::core::PSTR, piiconindex : *mut u16, piiconid : *mut u16) -> super::windef::HICON);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn ExtractAssociatedIconExW(hinst : super::minwindef::HINSTANCE, psziconpath : windows_sys::core::PWSTR, piiconindex : *mut u16, piiconid : *mut u16) -> super::windef::HICON);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn ExtractAssociatedIconW(hinst : super::minwindef::HINSTANCE, psziconpath : windows_sys::core::PWSTR, piicon : *mut u16) -> super::windef::HICON);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn ExtractIconA(hinst : super::minwindef::HINSTANCE, pszexefilename : windows_sys::core::PCSTR, niconindex : u32) -> super::windef::HICON);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn ExtractIconExA(lpszfile : windows_sys::core::PCSTR, niconindex : i32, phiconlarge : *mut super::windef::HICON, phiconsmall : *mut super::windef::HICON, nicons : u32) -> u32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn ExtractIconExW(lpszfile : windows_sys::core::PCWSTR, niconindex : i32, phiconlarge : *mut super::windef::HICON, phiconsmall : *mut super::windef::HICON, nicons : u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn ExtractIconW(hinst : super::minwindef::HINSTANCE, pszexefilename : windows_sys::core::PCWSTR, niconindex : u32) -> super::windef::HICON);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("shell32.dll" "system" fn FindExecutableA(lpfile : windows_sys::core::PCSTR, lpdirectory : windows_sys::core::PCSTR, lpresult : windows_sys::core::PSTR) -> super::minwindef::HINSTANCE);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("shell32.dll" "system" fn FindExecutableW(lpfile : windows_sys::core::PCWSTR, lpdirectory : windows_sys::core::PCWSTR, lpresult : windows_sys::core::PWSTR) -> super::minwindef::HINSTANCE);
windows_link::link!("shell32.dll" "system" fn InitNetworkAddressControl() -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn IsLFNDriveA(pszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn IsLFNDriveW(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn SHAppBarMessage(dwmessage : u32, pdata : *mut APPBARDATA) -> usize);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_processthreadsapi", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("shell32.dll" "system" fn SHCreateProcessAsUserW(pscpi : *mut SHCREATEPROCESSINFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn SHEmptyRecycleBinA(hwnd : super::windef::HWND, pszrootpath : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn SHEmptyRecycleBinW(hwnd : super::windef::HWND, pszrootpath : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("shell32.dll" "system" fn SHEnumerateUnreadMailAccountsW(hkeyuser : super::minwindef::HKEY, dwindex : u32, pszmailaddress : windows_sys::core::PWSTR, cchmailaddress : i32) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHEvaluateSystemCommandTemplate(pszcmdtemplate : windows_sys::core::PCWSTR, ppszapplication : *mut windows_sys::core::PWSTR, ppszcommandline : *mut windows_sys::core::PWSTR, ppszparameters : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("shell32.dll" "system" fn SHFileOperationA(lpfileop : *mut SHFILEOPSTRUCTA) -> i32);
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("shell32.dll" "system" fn SHFileOperationW(lpfileop : *mut SHFILEOPSTRUCTW) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("shell32.dll" "system" fn SHFreeNameMappings(hnamemappings : super::winnt::HANDLE));
windows_link::link!("shell32.dll" "system" fn SHGetDiskFreeSpaceExA(pszdirectoryname : windows_sys::core::PCSTR, pulfreebytesavailabletocaller : *mut u64, pultotalnumberofbytes : *mut u64, pultotalnumberoffreebytes : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn SHGetDiskFreeSpaceExW(pszdirectoryname : windows_sys::core::PCWSTR, pulfreebytesavailabletocaller : *mut u64, pultotalnumberofbytes : *mut u64, pultotalnumberoffreebytes : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn SHGetDriveMedia(pszdrive : windows_sys::core::PCWSTR, pdwmediacontent : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn SHGetFileInfoA(pszpath : windows_sys::core::PCSTR, dwfileattributes : u32, psfi : *mut SHFILEINFOA, cbfileinfo : u32, uflags : u32) -> usize);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn SHGetFileInfoW(pszpath : windows_sys::core::PCWSTR, dwfileattributes : u32, psfi : *mut SHFILEINFOW, cbfileinfo : u32, uflags : u32) -> usize);
windows_link::link!("shell32.dll" "system" fn SHGetImageList(iimagelist : i32, riid : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHGetLocalizedName(pszpath : windows_sys::core::PCWSTR, pszresmodule : windows_sys::core::PWSTR, cch : u32, pidsres : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHGetNewLinkInfoA(pszlinkto : windows_sys::core::PCSTR, pszdir : windows_sys::core::PCSTR, pszname : windows_sys::core::PSTR, pfmustcopy : *mut windows_sys::core::BOOL, uflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn SHGetNewLinkInfoW(pszlinkto : windows_sys::core::PCWSTR, pszdir : windows_sys::core::PCWSTR, pszname : windows_sys::core::PWSTR, pfmustcopy : *mut windows_sys::core::BOOL, uflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn SHGetPropertyStoreForWindow(hwnd : super::windef::HWND, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn SHGetStockIconInfo(siid : SHSTOCKICONID, uflags : u32, psii : *mut SHSTOCKICONINFO) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("shell32.dll" "system" fn SHGetUnreadMailCountW(hkeyuser : super::minwindef::HKEY, pszmailaddress : windows_sys::core::PCWSTR, pdwcount : *mut u32, pfiletime : *mut super::minwindef::FILETIME, pszshellexecutecommand : windows_sys::core::PWSTR, cchshellexecutecommand : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn SHInvokePrinterCommandA(hwnd : super::windef::HWND, uaction : u32, lpbuf1 : windows_sys::core::PCSTR, lpbuf2 : windows_sys::core::PCSTR, fmodal : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn SHInvokePrinterCommandW(hwnd : super::windef::HWND, uaction : u32, lpbuf1 : windows_sys::core::PCWSTR, lpbuf2 : windows_sys::core::PCWSTR, fmodal : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("shell32.dll" "system" fn SHIsFileAvailableOffline(pwszpath : windows_sys::core::PCWSTR, pdwstatus : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHLoadNonloadedIconOverlayIdentifiers() -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHQueryRecycleBinA(pszrootpath : windows_sys::core::PCSTR, pshqueryrbinfo : *mut SHQUERYRBINFO) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHQueryRecycleBinW(pszrootpath : windows_sys::core::PCWSTR, pshqueryrbinfo : *mut SHQUERYRBINFO) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHQueryUserNotificationState(pquns : *mut QUERY_USER_NOTIFICATION_STATE) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHRemoveLocalizedName(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHSetLocalizedName(pszpath : windows_sys::core::PCWSTR, pszresmodule : windows_sys::core::PCWSTR, idsres : i32) -> windows_sys::core::HRESULT);
windows_link::link!("shell32.dll" "system" fn SHSetUnreadMailCountW(pszmailaddress : windows_sys::core::PCWSTR, dwcount : u32, pszshellexecutecommand : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("shell32.dll" "system" fn SHTestTokenMembership(htoken : super::winnt::HANDLE, ulrid : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn ShellAboutA(hwnd : super::windef::HWND, szapp : windows_sys::core::PCSTR, szotherstuff : windows_sys::core::PCSTR, hicon : super::windef::HICON) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn ShellAboutW(hwnd : super::windef::HWND, szapp : windows_sys::core::PCWSTR, szotherstuff : windows_sys::core::PCWSTR, hicon : super::windef::HICON) -> i32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn ShellExecuteA(hwnd : super::windef::HWND, lpoperation : windows_sys::core::PCSTR, lpfile : windows_sys::core::PCSTR, lpparameters : windows_sys::core::PCSTR, lpdirectory : windows_sys::core::PCSTR, nshowcmd : i32) -> super::minwindef::HINSTANCE);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("shell32.dll" "system" fn ShellExecuteExA(pexecinfo : *mut SHELLEXECUTEINFOA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("shell32.dll" "system" fn ShellExecuteExW(pexecinfo : *mut SHELLEXECUTEINFOW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shell32.dll" "system" fn ShellExecuteW(hwnd : super::windef::HWND, lpoperation : windows_sys::core::PCWSTR, lpfile : windows_sys::core::PCWSTR, lpparameters : windows_sys::core::PCWSTR, lpdirectory : windows_sys::core::PCWSTR, nshowcmd : i32) -> super::minwindef::HINSTANCE);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shlwapi.dll" "C" fn ShellMessageBoxA(happinst : super::minwindef::HINSTANCE, hwnd : super::windef::HWND, lpctext : windows_sys::core::PCSTR, lpctitle : windows_sys::core::PCSTR, fustyle : u32, ...) -> i32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("shlwapi.dll" "C" fn ShellMessageBoxW(happinst : super::minwindef::HINSTANCE, hwnd : super::windef::HWND, lpctext : windows_sys::core::PCWSTR, lpctitle : windows_sys::core::PCWSTR, fustyle : u32, ...) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn Shell_NotifyIconA(dwmessage : u32, lpdata : *const NOTIFYICONDATAA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn Shell_NotifyIconGetRect(identifier : *const NOTIFYICONIDENTIFIER, iconlocation : *mut super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("shell32.dll" "system" fn Shell_NotifyIconW(dwmessage : u32, lpdata : *const NOTIFYICONDATAW) -> windows_sys::core::BOOL);
pub const ABC_OVERLAYDESKTOPICONS: u32 = 1;
pub const ABE_BOTTOM: u32 = 3;
pub const ABE_LEFT: u32 = 0;
pub const ABE_RIGHT: u32 = 2;
pub const ABE_TOP: u32 = 1;
pub const ABM_ACTIVATE: u32 = 6;
pub const ABM_GETAUTOHIDEBAR: u32 = 7;
pub const ABM_GETAUTOHIDEBAREX: u32 = 11;
pub const ABM_GETSTATE: u32 = 4;
pub const ABM_GETTASKBARPOS: u32 = 5;
pub const ABM_NEW: u32 = 0;
pub const ABM_QUERYPOS: u32 = 2;
pub const ABM_REMOVE: u32 = 1;
pub const ABM_SETAUTOHIDEBAR: u32 = 8;
pub const ABM_SETAUTOHIDEBAREX: u32 = 12;
pub const ABM_SETPOS: u32 = 3;
pub const ABM_SETSTATE: u32 = 10;
pub const ABM_WINDOWPOSCHANGED: u32 = 9;
pub const ABN_FULLSCREENAPP: u32 = 2;
pub const ABN_POSCHANGED: u32 = 1;
pub const ABN_STATECHANGE: u32 = 0;
pub const ABN_WINDOWARRANGE: u32 = 3;
pub const ABS_ALWAYSONTOP: u32 = 2;
pub const ABS_AUTOHIDE: u32 = 1;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct APPBARDATA {
    pub cbSize: u32,
    pub hWnd: super::windef::HWND,
    pub uCallbackMessage: u32,
    pub uEdge: u32,
    pub rc: super::windef::RECT,
    pub lParam: super::minwindef::LPARAM,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for APPBARDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct APPBARDATA {
    pub cbSize: u32,
    pub hWnd: super::windef::HWND,
    pub uCallbackMessage: u32,
    pub uEdge: u32,
    pub rc: super::windef::RECT,
    pub lParam: super::minwindef::LPARAM,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for APPBARDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ASSOCCLASS = i32;
pub const ASSOCCLASS_APP_KEY: ASSOCCLASS = 5;
pub const ASSOCCLASS_APP_STR: ASSOCCLASS = 6;
pub const ASSOCCLASS_CLSID_KEY: ASSOCCLASS = 3;
pub const ASSOCCLASS_CLSID_STR: ASSOCCLASS = 4;
pub const ASSOCCLASS_FIXED_PROGID_STR: ASSOCCLASS = 10;
pub const ASSOCCLASS_FOLDER: ASSOCCLASS = 8;
pub const ASSOCCLASS_PROGID_KEY: ASSOCCLASS = 1;
pub const ASSOCCLASS_PROGID_STR: ASSOCCLASS = 2;
pub const ASSOCCLASS_PROTOCOL_STR: ASSOCCLASS = 11;
pub const ASSOCCLASS_SHELL_KEY: ASSOCCLASS = 0;
pub const ASSOCCLASS_STAR: ASSOCCLASS = 9;
pub const ASSOCCLASS_SYSTEM_STR: ASSOCCLASS = 7;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct ASSOCIATIONELEMENT {
    pub ac: ASSOCCLASS,
    pub hkClass: super::minwindef::HKEY,
    pub pszClass: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_minwindef")]
impl Default for ASSOCIATIONELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct ASSOCIATIONELEMENT {
    pub ac: ASSOCCLASS,
    pub hkClass: super::minwindef::HKEY,
    pub pszClass: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_minwindef")]
impl Default for ASSOCIATIONELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type DRAGINFO = DRAGINFOA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DRAGINFOA {
    pub uSize: u32,
    pub pt: super::windef::POINT,
    pub fNC: windows_sys::core::BOOL,
    pub lpFileList: super::winnt::PZZSTR,
    pub grfKeyState: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for DRAGINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DRAGINFOA {
    pub uSize: u32,
    pub pt: super::windef::POINT,
    pub fNC: windows_sys::core::BOOL,
    pub lpFileList: super::winnt::PZZSTR,
    pub grfKeyState: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for DRAGINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DRAGINFOW {
    pub uSize: u32,
    pub pt: super::windef::POINT,
    pub fNC: windows_sys::core::BOOL,
    pub lpFileList: super::winnt::PZZWSTR,
    pub grfKeyState: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for DRAGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DRAGINFOW {
    pub uSize: u32,
    pub pt: super::windef::POINT,
    pub fNC: windows_sys::core::BOOL,
    pub lpFileList: super::winnt::PZZWSTR,
    pub grfKeyState: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for DRAGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FILEOP_FLAGS = u16;
pub const FOF_ALLOWUNDO: u32 = 64;
pub const FOF_CONFIRMMOUSE: u32 = 2;
pub const FOF_FILESONLY: u32 = 128;
pub const FOF_MULTIDESTFILES: u32 = 1;
pub const FOF_NOCONFIRMATION: u32 = 16;
pub const FOF_NOCONFIRMMKDIR: u32 = 512;
pub const FOF_NOCOPYSECURITYATTRIBS: u32 = 2048;
pub const FOF_NOERRORUI: u32 = 1024;
pub const FOF_NORECURSEREPARSE: u32 = 32768;
pub const FOF_NORECURSION: u32 = 4096;
pub const FOF_NO_CONNECTED_ELEMENTS: u32 = 8192;
pub const FOF_NO_UI: u32 = 1556;
pub const FOF_RENAMEONCOLLISION: u32 = 8;
pub const FOF_SILENT: u32 = 4;
pub const FOF_SIMPLEPROGRESS: u32 = 256;
pub const FOF_WANTMAPPINGHANDLE: u32 = 32;
pub const FOF_WANTNUKEWARNING: u32 = 16384;
pub const FO_COPY: u32 = 2;
pub const FO_DELETE: u32 = 3;
pub const FO_MOVE: u32 = 1;
pub const FO_RENAME: u32 = 4;
pub type HDROP = *mut core::ffi::c_void;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPDRAGINFO = LPDRAGINFOA;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPDRAGINFOA = *mut DRAGINFOA;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPDRAGINFOW = *mut DRAGINFOW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPSHELLEXECUTEINFO = LPSHELLEXECUTEINFOA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPSHELLEXECUTEINFOA = *mut SHELLEXECUTEINFOA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPSHELLEXECUTEINFOW = *mut SHELLEXECUTEINFOW;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPSHFILEOPSTRUCT = LPSHFILEOPSTRUCTA;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPSHFILEOPSTRUCTA = *mut SHFILEOPSTRUCTA;
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPSHFILEOPSTRUCTW = *mut SHFILEOPSTRUCTW;
pub type LPSHNAMEMAPPING = LPSHNAMEMAPPINGA;
pub type LPSHNAMEMAPPINGA = *mut SHNAMEMAPPINGA;
pub type LPSHNAMEMAPPINGW = *mut SHNAMEMAPPINGW;
pub type LPSHQUERYRBINFO = *mut SHQUERYRBINFO;
pub const NCM_DISPLAYERRORTIP: u32 = 1028;
pub const NCM_GETADDRESS: u32 = 1025;
pub const NCM_GETALLOWTYPE: u32 = 1027;
pub const NCM_SETALLOWTYPE: u32 = 1026;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NC_ADDRESS {
    pub pAddrInfo: *mut NET_ADDRESS_INFO_,
    pub PortNumber: u16,
    pub PrefixLength: u8,
}
impl Default for NC_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NET_ADDRESS_INFO_(pub u8);
pub const NIF_GUID: u32 = 32;
pub const NIF_ICON: u32 = 2;
pub const NIF_INFO: u32 = 16;
pub const NIF_MESSAGE: u32 = 1;
pub const NIF_REALTIME: u32 = 64;
pub const NIF_SHOWTIP: u32 = 128;
pub const NIF_STATE: u32 = 8;
pub const NIF_TIP: u32 = 4;
pub const NIIF_ERROR: u32 = 3;
pub const NIIF_ICON_MASK: u32 = 15;
pub const NIIF_INFO: u32 = 1;
pub const NIIF_LARGE_ICON: u32 = 32;
pub const NIIF_NONE: u32 = 0;
pub const NIIF_NOSOUND: u32 = 16;
pub const NIIF_RESPECT_QUIET_TIME: u32 = 128;
pub const NIIF_USER: u32 = 4;
pub const NIIF_WARNING: u32 = 2;
pub const NIM_ADD: u32 = 0;
pub const NIM_DELETE: u32 = 2;
pub const NIM_MODIFY: u32 = 1;
pub const NIM_SETFOCUS: u32 = 3;
pub const NIM_SETVERSION: u32 = 4;
pub const NINF_KEY: u32 = 1;
pub const NIN_BALLOONHIDE: u32 = 1027;
pub const NIN_BALLOONSHOW: u32 = 1026;
pub const NIN_BALLOONTIMEOUT: u32 = 1028;
pub const NIN_BALLOONUSERCLICK: u32 = 1029;
pub const NIN_KEYSELECT: u32 = 1025;
pub const NIN_POPUPCLOSE: u32 = 1031;
pub const NIN_POPUPOPEN: u32 = 1030;
pub const NIN_SELECT: u32 = 1024;
pub const NIS_HIDDEN: u32 = 1;
pub const NIS_SHAREDICON: u32 = 2;
#[cfg(feature = "Win32_windef")]
pub type NOTIFYICONDATA = NOTIFYICONDATAA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAA {
    pub cbSize: u32,
    pub hWnd: super::windef::HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: super::windef::HICON,
    pub szTip: [i8; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [i8; 256],
    pub Anonymous: NOTIFYICONDATAA_0,
    pub szInfoTitle: [i8; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_sys::core::GUID,
    pub hBalloonIcon: super::windef::HICON,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONDATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAA_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONDATAA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAA {
    pub cbSize: u32,
    pub hWnd: super::windef::HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: super::windef::HICON,
    pub szTip: [i8; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [i8; 256],
    pub Anonymous: NOTIFYICONDATAA_0,
    pub szInfoTitle: [i8; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_sys::core::GUID,
    pub hBalloonIcon: super::windef::HICON,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONDATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAA_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONDATAA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATAA_V1_SIZE: u32 = 88;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATAA_V1_SIZE: u32 = 104;
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATAA_V2_SIZE: u32 = 488;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATAA_V2_SIZE: u32 = 504;
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATAA_V3_SIZE: u32 = 504;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATAA_V3_SIZE: u32 = 520;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: super::windef::HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: super::windef::HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_sys::core::GUID,
    pub hBalloonIcon: super::windef::HICON,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: super::windef::HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: super::windef::HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_sys::core::GUID,
    pub hBalloonIcon: super::windef::HICON,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATAW_V1_SIZE: u32 = 152;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATAW_V1_SIZE: u32 = 168;
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATAW_V2_SIZE: u32 = 936;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATAW_V2_SIZE: u32 = 952;
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATAW_V3_SIZE: u32 = 952;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATAW_V3_SIZE: u32 = 968;
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATA_V1_SIZE: u32 = 88;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATA_V1_SIZE: u32 = 104;
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATA_V2_SIZE: u32 = 488;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATA_V2_SIZE: u32 = 504;
#[cfg(target_arch = "x86")]
pub const NOTIFYICONDATA_V3_SIZE: u32 = 504;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const NOTIFYICONDATA_V3_SIZE: u32 = 520;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONIDENTIFIER {
    pub cbSize: u32,
    pub hWnd: super::windef::HWND,
    pub uID: u32,
    pub guidItem: windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONIDENTIFIER {
    pub cbSize: u32,
    pub hWnd: super::windef::HWND,
    pub uID: u32,
    pub guidItem: windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
impl Default for NOTIFYICONIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NOTIFYICON_VERSION: u32 = 3;
pub const NOTIFYICON_VERSION_4: u32 = 4;
pub const OFFLINE_STATUS_INCOMPLETE: u32 = 4;
pub const OFFLINE_STATUS_LOCAL: u32 = 1;
pub const OFFLINE_STATUS_REMOTE: u32 = 2;
pub type OPEN_PRINTER_PROPS_INFO = OPEN_PRINTER_PROPS_INFOA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OPEN_PRINTER_PROPS_INFOA {
    pub dwSize: u32,
    pub pszSheetName: windows_sys::core::PSTR,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: windows_sys::core::BOOL,
}
#[cfg(target_arch = "x86")]
impl Default for OPEN_PRINTER_PROPS_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OPEN_PRINTER_PROPS_INFOA {
    pub dwSize: u32,
    pub pszSheetName: windows_sys::core::PSTR,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: windows_sys::core::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPEN_PRINTER_PROPS_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OPEN_PRINTER_PROPS_INFOW {
    pub dwSize: u32,
    pub pszSheetName: windows_sys::core::PWSTR,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: windows_sys::core::BOOL,
}
#[cfg(target_arch = "x86")]
impl Default for OPEN_PRINTER_PROPS_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OPEN_PRINTER_PROPS_INFOW {
    pub dwSize: u32,
    pub pszSheetName: windows_sys::core::PWSTR,
    pub uSheetIndex: u32,
    pub dwFlags: u32,
    pub bModal: windows_sys::core::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPEN_PRINTER_PROPS_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type PAPPBARDATA = *mut APPBARDATA;
pub type PFNCANSHAREFOLDERW = Option<unsafe extern "system" fn(pszpath: windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_windef")]
pub type PFNSHOWSHAREFOLDERUIW = Option<unsafe extern "system" fn(hwndparent: super::windef::HWND, pszpath: windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT>;
pub type PNC_ADDRESS = *mut NC_ADDRESS;
#[cfg(feature = "Win32_windef")]
pub type PNOTIFYICONDATA = PNOTIFYICONDATAA;
#[cfg(feature = "Win32_windef")]
pub type PNOTIFYICONDATAA = *mut NOTIFYICONDATAA;
#[cfg(feature = "Win32_windef")]
pub type PNOTIFYICONDATAW = *mut NOTIFYICONDATAW;
#[cfg(feature = "Win32_windef")]
pub type PNOTIFYICONIDENTIFIER = *mut NOTIFYICONIDENTIFIER;
pub type POPEN_PRINTER_PROPS_INFO = POPEN_PRINTER_PROPS_INFOA;
pub type POPEN_PRINTER_PROPS_INFOA = *mut OPEN_PRINTER_PROPS_INFOA;
pub type POPEN_PRINTER_PROPS_INFOW = *mut OPEN_PRINTER_PROPS_INFOW;
pub const PO_DELETE: u32 = 19;
pub const PO_PORTCHANGE: u32 = 32;
pub const PO_RENAME: u32 = 20;
pub const PO_REN_PORT: u32 = 52;
pub const PRINTACTION_DOCUMENTDEFAULTS: u32 = 6;
pub const PRINTACTION_NETINSTALL: u32 = 2;
pub const PRINTACTION_NETINSTALLLINK: u32 = 3;
pub const PRINTACTION_OPEN: u32 = 0;
pub const PRINTACTION_OPENNETPRN: u32 = 5;
pub const PRINTACTION_PROPERTIES: u32 = 1;
pub const PRINTACTION_SERVERPROPERTIES: u32 = 7;
pub const PRINTACTION_TESTPAGE: u32 = 4;
pub type PRINTEROP_FLAGS = u16;
pub const PRINT_PROP_FORCE_NAME: u32 = 1;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_processthreadsapi", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PSHCREATEPROCESSINFOW = *mut SHCREATEPROCESSINFOW;
pub type QUERY_USER_NOTIFICATION_STATE = i32;
pub const QUNS_ACCEPTS_NOTIFICATIONS: QUERY_USER_NOTIFICATION_STATE = 5;
pub const QUNS_APP: QUERY_USER_NOTIFICATION_STATE = 7;
pub const QUNS_BUSY: QUERY_USER_NOTIFICATION_STATE = 2;
pub const QUNS_NOT_PRESENT: QUERY_USER_NOTIFICATION_STATE = 1;
pub const QUNS_PRESENTATION_MODE: QUERY_USER_NOTIFICATION_STATE = 4;
pub const QUNS_QUIET_TIME: QUERY_USER_NOTIFICATION_STATE = 6;
pub const QUNS_RUNNING_D3D_FULL_SCREEN: QUERY_USER_NOTIFICATION_STATE = 3;
pub const SEE_MASK_ASYNCOK: u32 = 1048576;
pub const SEE_MASK_CLASSKEY: u32 = 3;
pub const SEE_MASK_CLASSNAME: u32 = 1;
pub const SEE_MASK_CONNECTNETDRV: u32 = 128;
pub const SEE_MASK_DEFAULT: u32 = 0;
pub const SEE_MASK_DOENVSUBST: u32 = 512;
pub const SEE_MASK_FLAG_DDEWAIT: u32 = 256;
pub const SEE_MASK_FLAG_HINST_IS_SITE: u32 = 134217728;
pub const SEE_MASK_FLAG_LOG_USAGE: u32 = 67108864;
pub const SEE_MASK_FLAG_NO_UI: u32 = 1024;
pub const SEE_MASK_HMONITOR: u32 = 2097152;
pub const SEE_MASK_HOTKEY: u32 = 32;
pub const SEE_MASK_IDLIST: u32 = 4;
pub const SEE_MASK_INVOKEIDLIST: u32 = 12;
pub const SEE_MASK_NOASYNC: u32 = 256;
pub const SEE_MASK_NOCLOSEPROCESS: u32 = 64;
pub const SEE_MASK_NOQUERYCLASSSTORE: u32 = 16777216;
pub const SEE_MASK_NOZONECHECKS: u32 = 8388608;
pub const SEE_MASK_NO_CONSOLE: u32 = 32768;
pub const SEE_MASK_UNICODE: u32 = 16384;
pub const SEE_MASK_WAITFORINPUTIDLE: u32 = 33554432;
pub const SE_ERR_ACCESSDENIED: u32 = 5;
pub const SE_ERR_ASSOCINCOMPLETE: u32 = 27;
pub const SE_ERR_DDEBUSY: u32 = 30;
pub const SE_ERR_DDEFAIL: u32 = 29;
pub const SE_ERR_DDETIMEOUT: u32 = 28;
pub const SE_ERR_DLLNOTFOUND: u32 = 32;
pub const SE_ERR_FNF: u32 = 2;
pub const SE_ERR_NOASSOC: u32 = 31;
pub const SE_ERR_OOM: u32 = 8;
pub const SE_ERR_PNF: u32 = 3;
pub const SE_ERR_SHARE: u32 = 26;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_processthreadsapi", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHCREATEPROCESSINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub pszFile: windows_sys::core::PCWSTR,
    pub pszParameters: windows_sys::core::PCWSTR,
    pub pszCurrentDirectory: windows_sys::core::PCWSTR,
    pub hUserToken: super::winnt::HANDLE,
    pub lpProcessAttributes: super::minwinbase::LPSECURITY_ATTRIBUTES,
    pub lpThreadAttributes: super::minwinbase::LPSECURITY_ATTRIBUTES,
    pub bInheritHandles: windows_sys::core::BOOL,
    pub dwCreationFlags: u32,
    pub lpStartupInfo: super::processthreadsapi::LPSTARTUPINFOW,
    pub lpProcessInformation: super::processthreadsapi::LPPROCESS_INFORMATION,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_processthreadsapi", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHCREATEPROCESSINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_processthreadsapi", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHCREATEPROCESSINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub pszFile: windows_sys::core::PCWSTR,
    pub pszParameters: windows_sys::core::PCWSTR,
    pub pszCurrentDirectory: windows_sys::core::PCWSTR,
    pub hUserToken: super::winnt::HANDLE,
    pub lpProcessAttributes: super::minwinbase::LPSECURITY_ATTRIBUTES,
    pub lpThreadAttributes: super::minwinbase::LPSECURITY_ATTRIBUTES,
    pub bInheritHandles: windows_sys::core::BOOL,
    pub dwCreationFlags: u32,
    pub lpStartupInfo: super::processthreadsapi::LPSTARTUPINFOW,
    pub lpProcessInformation: super::processthreadsapi::LPPROCESS_INFORMATION,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_processthreadsapi", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHCREATEPROCESSINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type SHELLEXECUTEINFO = SHELLEXECUTEINFOA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHELLEXECUTEINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub lpVerb: windows_sys::core::PCSTR,
    pub lpFile: windows_sys::core::PCSTR,
    pub lpParameters: windows_sys::core::PCSTR,
    pub lpDirectory: windows_sys::core::PCSTR,
    pub nShow: i32,
    pub hInstApp: super::minwindef::HINSTANCE,
    pub lpIDList: *mut core::ffi::c_void,
    pub lpClass: windows_sys::core::PCSTR,
    pub hkeyClass: super::minwindef::HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOA_0,
    pub hProcess: super::winnt::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHELLEXECUTEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union SHELLEXECUTEINFOA_0 {
    pub hIcon: super::winnt::HANDLE,
    pub hMonitor: super::winnt::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHELLEXECUTEINFOA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHELLEXECUTEINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub lpVerb: windows_sys::core::PCSTR,
    pub lpFile: windows_sys::core::PCSTR,
    pub lpParameters: windows_sys::core::PCSTR,
    pub lpDirectory: windows_sys::core::PCSTR,
    pub nShow: i32,
    pub hInstApp: super::minwindef::HINSTANCE,
    pub lpIDList: *mut core::ffi::c_void,
    pub lpClass: windows_sys::core::PCSTR,
    pub hkeyClass: super::minwindef::HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOA_0,
    pub hProcess: super::winnt::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHELLEXECUTEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union SHELLEXECUTEINFOA_0 {
    pub hIcon: super::winnt::HANDLE,
    pub hMonitor: super::winnt::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHELLEXECUTEINFOA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHELLEXECUTEINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub lpVerb: windows_sys::core::PCWSTR,
    pub lpFile: windows_sys::core::PCWSTR,
    pub lpParameters: windows_sys::core::PCWSTR,
    pub lpDirectory: windows_sys::core::PCWSTR,
    pub nShow: i32,
    pub hInstApp: super::minwindef::HINSTANCE,
    pub lpIDList: *mut core::ffi::c_void,
    pub lpClass: windows_sys::core::PCWSTR,
    pub hkeyClass: super::minwindef::HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOW_0,
    pub hProcess: super::winnt::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHELLEXECUTEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union SHELLEXECUTEINFOW_0 {
    pub hIcon: super::winnt::HANDLE,
    pub hMonitor: super::winnt::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHELLEXECUTEINFOW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHELLEXECUTEINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub hwnd: super::windef::HWND,
    pub lpVerb: windows_sys::core::PCWSTR,
    pub lpFile: windows_sys::core::PCWSTR,
    pub lpParameters: windows_sys::core::PCWSTR,
    pub lpDirectory: windows_sys::core::PCWSTR,
    pub nShow: i32,
    pub hInstApp: super::minwindef::HINSTANCE,
    pub lpIDList: *mut core::ffi::c_void,
    pub lpClass: windows_sys::core::PCWSTR,
    pub hkeyClass: super::minwindef::HKEY,
    pub dwHotKey: u32,
    pub Anonymous: SHELLEXECUTEINFOW_0,
    pub hProcess: super::winnt::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHELLEXECUTEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union SHELLEXECUTEINFOW_0 {
    pub hIcon: super::winnt::HANDLE,
    pub hMonitor: super::winnt::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHELLEXECUTEINFOW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHERB_NOCONFIRMATION: u32 = 1;
pub const SHERB_NOPROGRESSUI: u32 = 2;
pub const SHERB_NOSOUND: u32 = 4;
#[cfg(feature = "Win32_windef")]
pub type SHFILEINFO = SHFILEINFOA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct SHFILEINFOA {
    pub hIcon: super::windef::HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [i8; 260],
    pub szTypeName: [i8; 80],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
impl Default for SHFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct SHFILEINFOA {
    pub hIcon: super::windef::HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [i8; 260],
    pub szTypeName: [i8; 80],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
impl Default for SHFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct SHFILEINFOW {
    pub hIcon: super::windef::HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [u16; 260],
    pub szTypeName: [u16; 80],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
impl Default for SHFILEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct SHFILEINFOW {
    pub hIcon: super::windef::HICON,
    pub iIcon: i32,
    pub dwAttributes: u32,
    pub szDisplayName: [u16; 260],
    pub szTypeName: [u16; 80],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
impl Default for SHFILEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
pub type SHFILEOPSTRUCT = SHFILEOPSTRUCTA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHFILEOPSTRUCTA {
    pub hwnd: super::windef::HWND,
    pub wFunc: u32,
    pub pFrom: super::winnt::PCZZSTR,
    pub pTo: super::winnt::PCZZSTR,
    pub fFlags: FILEOP_FLAGS,
    pub fAnyOperationsAborted: windows_sys::core::BOOL,
    pub hNameMappings: *mut core::ffi::c_void,
    pub lpszProgressTitle: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHFILEOPSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHFILEOPSTRUCTA {
    pub hwnd: super::windef::HWND,
    pub wFunc: u32,
    pub pFrom: super::winnt::PCZZSTR,
    pub pTo: super::winnt::PCZZSTR,
    pub fFlags: FILEOP_FLAGS,
    pub fAnyOperationsAborted: windows_sys::core::BOOL,
    pub hNameMappings: *mut core::ffi::c_void,
    pub lpszProgressTitle: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHFILEOPSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHFILEOPSTRUCTW {
    pub hwnd: super::windef::HWND,
    pub wFunc: u32,
    pub pFrom: super::winnt::PCZZWSTR,
    pub pTo: super::winnt::PCZZWSTR,
    pub fFlags: FILEOP_FLAGS,
    pub fAnyOperationsAborted: windows_sys::core::BOOL,
    pub hNameMappings: *mut core::ffi::c_void,
    pub lpszProgressTitle: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHFILEOPSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SHFILEOPSTRUCTW {
    pub hwnd: super::windef::HWND,
    pub wFunc: u32,
    pub pFrom: super::winnt::PCZZWSTR,
    pub pTo: super::winnt::PCZZWSTR,
    pub fFlags: FILEOP_FLAGS,
    pub fAnyOperationsAborted: windows_sys::core::BOOL,
    pub hNameMappings: *mut core::ffi::c_void,
    pub lpszProgressTitle: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for SHFILEOPSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHGFI_ADDOVERLAYS: u32 = 32;
pub const SHGFI_ATTRIBUTES: u32 = 2048;
pub const SHGFI_ATTR_SPECIFIED: u32 = 131072;
pub const SHGFI_DISPLAYNAME: u32 = 512;
pub const SHGFI_EXETYPE: u32 = 8192;
pub const SHGFI_ICON: u32 = 256;
pub const SHGFI_ICONLOCATION: u32 = 4096;
pub const SHGFI_LARGEICON: u32 = 0;
pub const SHGFI_LINKOVERLAY: u32 = 32768;
pub const SHGFI_OPENICON: u32 = 2;
pub const SHGFI_OVERLAYINDEX: u32 = 64;
pub const SHGFI_PIDL: u32 = 8;
pub const SHGFI_SELECTED: u32 = 65536;
pub const SHGFI_SHELLICONSIZE: u32 = 4;
pub const SHGFI_SMALLICON: u32 = 1;
pub const SHGFI_SYSICONINDEX: u32 = 16384;
pub const SHGFI_TYPENAME: u32 = 1024;
pub const SHGFI_USEFILEATTRIBUTES: u32 = 16;
pub const SHGNLI_NOLNK: u32 = 8;
pub const SHGNLI_NOLOCNAME: u32 = 16;
pub const SHGNLI_NOUNIQUE: u32 = 4;
pub const SHGNLI_PIDL: u32 = 1;
pub const SHGNLI_PREFIXNAME: u32 = 2;
pub const SHGNLI_USEURLEXT: u32 = 32;
pub const SHGSI_ICON: u32 = 256;
pub const SHGSI_ICONLOCATION: u32 = 0;
pub const SHGSI_LARGEICON: u32 = 0;
pub const SHGSI_LINKOVERLAY: u32 = 32768;
pub const SHGSI_SELECTED: u32 = 65536;
pub const SHGSI_SHELLICONSIZE: u32 = 4;
pub const SHGSI_SMALLICON: u32 = 1;
pub const SHGSI_SYSICONINDEX: u32 = 16384;
pub const SHIL_EXTRALARGE: u32 = 2;
pub const SHIL_JUMBO: u32 = 4;
pub const SHIL_LARGE: u32 = 0;
pub const SHIL_LAST: u32 = 4;
pub const SHIL_SMALL: u32 = 1;
pub const SHIL_SYSSMALL: u32 = 3;
pub type SHNAMEMAPPING = SHNAMEMAPPINGA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SHNAMEMAPPINGA {
    pub pszOldPath: windows_sys::core::PSTR,
    pub pszNewPath: windows_sys::core::PSTR,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
#[cfg(target_arch = "x86")]
impl Default for SHNAMEMAPPINGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SHNAMEMAPPINGA {
    pub pszOldPath: windows_sys::core::PSTR,
    pub pszNewPath: windows_sys::core::PSTR,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SHNAMEMAPPINGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SHNAMEMAPPINGW {
    pub pszOldPath: windows_sys::core::PWSTR,
    pub pszNewPath: windows_sys::core::PWSTR,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
#[cfg(target_arch = "x86")]
impl Default for SHNAMEMAPPINGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SHNAMEMAPPINGW {
    pub pszOldPath: windows_sys::core::PWSTR,
    pub pszNewPath: windows_sys::core::PWSTR,
    pub cchOldPath: i32,
    pub cchNewPath: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SHNAMEMAPPINGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SHQUERYRBINFO {
    pub cbSize: u32,
    pub i64Size: i64,
    pub i64NumItems: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SHQUERYRBINFO {
    pub cbSize: u32,
    pub i64Size: i64,
    pub i64NumItems: i64,
}
pub type SHSTOCKICONID = i32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct SHSTOCKICONINFO {
    pub cbSize: u32,
    pub hIcon: super::windef::HICON,
    pub iSysImageIndex: i32,
    pub iIcon: i32,
    pub szPath: [u16; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_windef")]
impl Default for SHSTOCKICONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct SHSTOCKICONINFO {
    pub cbSize: u32,
    pub hIcon: super::windef::HICON,
    pub iSysImageIndex: i32,
    pub iIcon: i32,
    pub szPath: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_windef")]
impl Default for SHSTOCKICONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIID_APPLICATION: SHSTOCKICONID = 2;
pub const SIID_AUDIOFILES: SHSTOCKICONID = 71;
pub const SIID_AUTOLIST: SHSTOCKICONID = 49;
pub const SIID_CLUSTEREDDRIVE: SHSTOCKICONID = 140;
pub const SIID_DELETE: SHSTOCKICONID = 84;
pub const SIID_DESKTOPPC: SHSTOCKICONID = 94;
pub const SIID_DEVICEAUDIOPLAYER: SHSTOCKICONID = 102;
pub const SIID_DEVICECAMERA: SHSTOCKICONID = 100;
pub const SIID_DEVICECELLPHONE: SHSTOCKICONID = 99;
pub const SIID_DEVICEVIDEOCAMERA: SHSTOCKICONID = 101;
pub const SIID_DOCASSOC: SHSTOCKICONID = 1;
pub const SIID_DOCNOASSOC: SHSTOCKICONID = 0;
pub const SIID_DRIVE35: SHSTOCKICONID = 6;
pub const SIID_DRIVE525: SHSTOCKICONID = 5;
pub const SIID_DRIVEBD: SHSTOCKICONID = 133;
pub const SIID_DRIVECD: SHSTOCKICONID = 11;
pub const SIID_DRIVEDVD: SHSTOCKICONID = 59;
pub const SIID_DRIVEFIXED: SHSTOCKICONID = 8;
pub const SIID_DRIVEHDDVD: SHSTOCKICONID = 132;
pub const SIID_DRIVENET: SHSTOCKICONID = 9;
pub const SIID_DRIVENETDISABLED: SHSTOCKICONID = 10;
pub const SIID_DRIVERAM: SHSTOCKICONID = 12;
pub const SIID_DRIVEREMOVE: SHSTOCKICONID = 7;
pub const SIID_DRIVEUNKNOWN: SHSTOCKICONID = 58;
pub const SIID_ERROR: SHSTOCKICONID = 80;
pub const SIID_FIND: SHSTOCKICONID = 22;
pub const SIID_FOLDER: SHSTOCKICONID = 3;
pub const SIID_FOLDERBACK: SHSTOCKICONID = 75;
pub const SIID_FOLDERFRONT: SHSTOCKICONID = 76;
pub const SIID_FOLDEROPEN: SHSTOCKICONID = 4;
pub const SIID_HELP: SHSTOCKICONID = 23;
pub const SIID_IMAGEFILES: SHSTOCKICONID = 72;
pub const SIID_INFO: SHSTOCKICONID = 79;
pub const SIID_INTERNET: SHSTOCKICONID = 104;
pub const SIID_INVALID: SHSTOCKICONID = -1;
pub const SIID_KEY: SHSTOCKICONID = 81;
pub const SIID_LINK: SHSTOCKICONID = 29;
pub const SIID_LOCK: SHSTOCKICONID = 47;
pub const SIID_MAX_ICONS: SHSTOCKICONID = 181;
pub const SIID_MEDIAAUDIODVD: SHSTOCKICONID = 85;
pub const SIID_MEDIABDR: SHSTOCKICONID = 138;
pub const SIID_MEDIABDRE: SHSTOCKICONID = 139;
pub const SIID_MEDIABDROM: SHSTOCKICONID = 137;
pub const SIID_MEDIABLANKCD: SHSTOCKICONID = 69;
pub const SIID_MEDIABLURAY: SHSTOCKICONID = 90;
pub const SIID_MEDIACDAUDIO: SHSTOCKICONID = 40;
pub const SIID_MEDIACDAUDIOPLUS: SHSTOCKICONID = 65;
pub const SIID_MEDIACDBURN: SHSTOCKICONID = 68;
pub const SIID_MEDIACDR: SHSTOCKICONID = 67;
pub const SIID_MEDIACDROM: SHSTOCKICONID = 70;
pub const SIID_MEDIACDRW: SHSTOCKICONID = 66;
pub const SIID_MEDIACOMPACTFLASH: SHSTOCKICONID = 98;
pub const SIID_MEDIADVD: SHSTOCKICONID = 60;
pub const SIID_MEDIADVDPLUSR: SHSTOCKICONID = 92;
pub const SIID_MEDIADVDPLUSRW: SHSTOCKICONID = 93;
pub const SIID_MEDIADVDR: SHSTOCKICONID = 63;
pub const SIID_MEDIADVDRAM: SHSTOCKICONID = 61;
pub const SIID_MEDIADVDROM: SHSTOCKICONID = 64;
pub const SIID_MEDIADVDRW: SHSTOCKICONID = 62;
pub const SIID_MEDIAENHANCEDCD: SHSTOCKICONID = 87;
pub const SIID_MEDIAENHANCEDDVD: SHSTOCKICONID = 88;
pub const SIID_MEDIAHDDVD: SHSTOCKICONID = 89;
pub const SIID_MEDIAHDDVDR: SHSTOCKICONID = 135;
pub const SIID_MEDIAHDDVDRAM: SHSTOCKICONID = 136;
pub const SIID_MEDIAHDDVDROM: SHSTOCKICONID = 134;
pub const SIID_MEDIAMOVIEDVD: SHSTOCKICONID = 86;
pub const SIID_MEDIASMARTMEDIA: SHSTOCKICONID = 97;
pub const SIID_MEDIASVCD: SHSTOCKICONID = 56;
pub const SIID_MEDIAVCD: SHSTOCKICONID = 91;
pub const SIID_MIXEDFILES: SHSTOCKICONID = 74;
pub const SIID_MOBILEPC: SHSTOCKICONID = 95;
pub const SIID_MYNETWORK: SHSTOCKICONID = 17;
pub const SIID_NETWORKCONNECT: SHSTOCKICONID = 103;
pub const SIID_PRINTER: SHSTOCKICONID = 16;
pub const SIID_PRINTERFAX: SHSTOCKICONID = 52;
pub const SIID_PRINTERFAXNET: SHSTOCKICONID = 53;
pub const SIID_PRINTERFILE: SHSTOCKICONID = 54;
pub const SIID_PRINTERNET: SHSTOCKICONID = 50;
pub const SIID_RECYCLER: SHSTOCKICONID = 31;
pub const SIID_RECYCLERFULL: SHSTOCKICONID = 32;
pub const SIID_RENAME: SHSTOCKICONID = 83;
pub const SIID_SERVER: SHSTOCKICONID = 15;
pub const SIID_SERVERSHARE: SHSTOCKICONID = 51;
pub const SIID_SETTINGS: SHSTOCKICONID = 106;
pub const SIID_SHARE: SHSTOCKICONID = 28;
pub const SIID_SHIELD: SHSTOCKICONID = 77;
pub const SIID_SLOWFILE: SHSTOCKICONID = 30;
pub const SIID_SOFTWARE: SHSTOCKICONID = 82;
pub const SIID_STACK: SHSTOCKICONID = 55;
pub const SIID_STUFFEDFOLDER: SHSTOCKICONID = 57;
pub const SIID_USERS: SHSTOCKICONID = 96;
pub const SIID_VIDEOFILES: SHSTOCKICONID = 73;
pub const SIID_WARNING: SHSTOCKICONID = 78;
pub const SIID_WORLD: SHSTOCKICONID = 13;
pub const SIID_ZIPFILE: SHSTOCKICONID = 105;
pub const WC_NETADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("msctls_netaddress");
