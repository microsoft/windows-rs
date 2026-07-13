#[cfg(all(feature = "minwindef", feature = "setupapi", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DiInstallDevice(hwndparent: Option<super::windef::HWND>, deviceinfoset: super::setupapi::HDEVINFO, deviceinfodata: *const super::setupapi::SP_DEVINFO_DATA, driverinfodata: Option<super::setupapi::PSP_DRVINFO_DATA>, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL {
    windows_core::link!("newdev.dll" "system" fn DiInstallDevice(hwndparent : super::windef::HWND, deviceinfoset : super::setupapi::HDEVINFO, deviceinfodata : *const super::setupapi::SP_DEVINFO_DATA, driverinfodata : super::setupapi::PSP_DRVINFO_DATA, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiInstallDevice(hwndparent.unwrap_or(core::mem::zeroed()) as _, deviceinfoset, deviceinfodata, driverinfodata.unwrap_or(core::mem::zeroed()) as _, flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DiInstallDriverA<P1>(hwndparent: Option<super::windef::HWND>, infpath: P1, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiInstallDriverA(hwndparent : super::windef::HWND, infpath : windows_core::PCSTR, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiInstallDriverA(hwndparent.unwrap_or(core::mem::zeroed()) as _, infpath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DiInstallDriverW<P1>(hwndparent: Option<super::windef::HWND>, infpath: P1, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiInstallDriverW(hwndparent : super::windef::HWND, infpath : windows_core::PCWSTR, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiInstallDriverW(hwndparent.unwrap_or(core::mem::zeroed()) as _, infpath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "setupapi", feature = "windef"))]
#[inline]
pub unsafe fn DiRollbackDriver(deviceinfoset: super::setupapi::HDEVINFO, deviceinfodata: *const super::setupapi::SP_DEVINFO_DATA, hwndparent: Option<super::windef::HWND>, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL {
    windows_core::link!("newdev.dll" "system" fn DiRollbackDriver(deviceinfoset : super::setupapi::HDEVINFO, deviceinfodata : *const super::setupapi::SP_DEVINFO_DATA, hwndparent : super::windef::HWND, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiRollbackDriver(deviceinfoset, deviceinfodata, hwndparent.unwrap_or(core::mem::zeroed()) as _, flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "setupapi", feature = "windef"))]
#[inline]
pub unsafe fn DiShowUpdateDevice(hwndparent: Option<super::windef::HWND>, deviceinfoset: super::setupapi::HDEVINFO, deviceinfodata: *const super::setupapi::SP_DEVINFO_DATA, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL {
    windows_core::link!("newdev.dll" "system" fn DiShowUpdateDevice(hwndparent : super::windef::HWND, deviceinfoset : super::setupapi::HDEVINFO, deviceinfodata : *const super::setupapi::SP_DEVINFO_DATA, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiShowUpdateDevice(hwndparent.unwrap_or(core::mem::zeroed()) as _, deviceinfoset, deviceinfodata, flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DiShowUpdateDriver<P1>(hwndparent: super::windef::HWND, filepath: P1, flags: u32, needreboot: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiShowUpdateDriver(hwndparent : super::windef::HWND, filepath : windows_core::PCWSTR, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiShowUpdateDriver(hwndparent, filepath.param().abi(), flags, needreboot as _) }
}
#[cfg(all(feature = "setupapi", feature = "windef"))]
#[inline]
pub unsafe fn DiUninstallDevice(hwndparent: super::windef::HWND, deviceinfoset: super::setupapi::HDEVINFO, deviceinfodata: *const super::setupapi::SP_DEVINFO_DATA, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL {
    windows_core::link!("newdev.dll" "system" fn DiUninstallDevice(hwndparent : super::windef::HWND, deviceinfoset : super::setupapi::HDEVINFO, deviceinfodata : *const super::setupapi::SP_DEVINFO_DATA, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiUninstallDevice(hwndparent, deviceinfoset, deviceinfodata, flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DiUninstallDriverA<P1>(hwndparent: Option<super::windef::HWND>, infpath: P1, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiUninstallDriverA(hwndparent : super::windef::HWND, infpath : windows_core::PCSTR, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiUninstallDriverA(hwndparent.unwrap_or(core::mem::zeroed()) as _, infpath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DiUninstallDriverW<P1>(hwndparent: Option<super::windef::HWND>, infpath: P1, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiUninstallDriverW(hwndparent : super::windef::HWND, infpath : windows_core::PCWSTR, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiUninstallDriverW(hwndparent.unwrap_or(core::mem::zeroed()) as _, infpath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UpdateDriverForPlugAndPlayDevicesA<P1, P2>(hwndparent: Option<super::windef::HWND>, hardwareid: P1, fullinfpath: P2, installflags: u32, brebootrequired: Option<*mut windows_core::BOOL>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("newdev.dll" "system" fn UpdateDriverForPlugAndPlayDevicesA(hwndparent : super::windef::HWND, hardwareid : windows_core::PCSTR, fullinfpath : windows_core::PCSTR, installflags : u32, brebootrequired : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { UpdateDriverForPlugAndPlayDevicesA(hwndparent.unwrap_or(core::mem::zeroed()) as _, hardwareid.param().abi(), fullinfpath.param().abi(), installflags, brebootrequired.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UpdateDriverForPlugAndPlayDevicesW<P1, P2>(hwndparent: Option<super::windef::HWND>, hardwareid: P1, fullinfpath: P2, installflags: u32, brebootrequired: Option<*mut windows_core::BOOL>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("newdev.dll" "system" fn UpdateDriverForPlugAndPlayDevicesW(hwndparent : super::windef::HWND, hardwareid : windows_core::PCWSTR, fullinfpath : windows_core::PCWSTR, installflags : u32, brebootrequired : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { UpdateDriverForPlugAndPlayDevicesW(hwndparent.unwrap_or(core::mem::zeroed()) as _, hardwareid.param().abi(), fullinfpath.param().abi(), installflags, brebootrequired.unwrap_or(core::mem::zeroed()) as _) }
}
pub const DIIDFLAG_BITS: u32 = 15;
pub const DIIDFLAG_INSTALLCOPYINFDRIVERS: u32 = 8;
pub const DIIDFLAG_INSTALLNULLDRIVER: u32 = 4;
pub const DIIDFLAG_NOFINISHINSTALLUI: u32 = 2;
pub const DIIDFLAG_SHOWSEARCHUI: u32 = 1;
pub const DIIRFLAG_BITS: u32 = 106;
pub const DIIRFLAG_FORCE_INF: u32 = 2;
pub const DIIRFLAG_HOTPATCH: u32 = 8;
pub const DIIRFLAG_HW_USING_THE_INF: u32 = 4;
pub const DIIRFLAG_INF_ALREADY_COPIED: u32 = 1;
pub const DIIRFLAG_INSTALL_AS_SET: u32 = 64;
pub const DIIRFLAG_NOBACKUP: u32 = 16;
pub const DIIRFLAG_PRE_CONFIGURE_INF: u32 = 32;
pub const DIIRFLAG_SYSTEM_BITS: u32 = 127;
pub const DIURFLAG_NO_REMOVE_INF: u32 = 1;
pub const DIURFLAG_RESERVED: u32 = 2;
pub const DIURFLAG_VALID: u32 = 3;
pub const INSTALLFLAG_BITS: u32 = 7;
pub const INSTALLFLAG_FORCE: u32 = 1;
pub const INSTALLFLAG_NONINTERACTIVE: u32 = 4;
pub const INSTALLFLAG_READONLY: u32 = 2;
pub const ROLLBACK_BITS: u32 = 1;
pub const ROLLBACK_FLAG_NO_UI: u32 = 1;
