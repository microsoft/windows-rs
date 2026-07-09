#[cfg(all(feature = "Win32_minwindef", feature = "Win32_setupapi", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("newdev.dll" "system" fn DiInstallDevice(hwndparent : super::windef::HWND, deviceinfoset : super::setupapi::HDEVINFO, deviceinfodata : *const super::setupapi::SP_DEVINFO_DATA, driverinfodata : super::setupapi::PSP_DRVINFO_DATA, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("newdev.dll" "system" fn DiInstallDriverA(hwndparent : super::windef::HWND, infpath : windows_sys::core::PCSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("newdev.dll" "system" fn DiInstallDriverW(hwndparent : super::windef::HWND, infpath : windows_sys::core::PCWSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_setupapi", feature = "Win32_windef"))]
windows_link::link!("newdev.dll" "system" fn DiRollbackDriver(deviceinfoset : super::setupapi::HDEVINFO, deviceinfodata : *const super::setupapi::SP_DEVINFO_DATA, hwndparent : super::windef::HWND, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_setupapi", feature = "Win32_windef"))]
windows_link::link!("newdev.dll" "system" fn DiShowUpdateDevice(hwndparent : super::windef::HWND, deviceinfoset : super::setupapi::HDEVINFO, deviceinfodata : *const super::setupapi::SP_DEVINFO_DATA, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("newdev.dll" "system" fn DiShowUpdateDriver(hwndparent : super::windef::HWND, filepath : windows_sys::core::PCWSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_setupapi", feature = "Win32_windef"))]
windows_link::link!("newdev.dll" "system" fn DiUninstallDevice(hwndparent : super::windef::HWND, deviceinfoset : super::setupapi::HDEVINFO, deviceinfodata : *const super::setupapi::SP_DEVINFO_DATA, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("newdev.dll" "system" fn DiUninstallDriverA(hwndparent : super::windef::HWND, infpath : windows_sys::core::PCSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("newdev.dll" "system" fn DiUninstallDriverW(hwndparent : super::windef::HWND, infpath : windows_sys::core::PCWSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("newdev.dll" "system" fn UpdateDriverForPlugAndPlayDevicesA(hwndparent : super::windef::HWND, hardwareid : windows_sys::core::PCSTR, fullinfpath : windows_sys::core::PCSTR, installflags : u32, brebootrequired : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("newdev.dll" "system" fn UpdateDriverForPlugAndPlayDevicesW(hwndparent : super::windef::HWND, hardwareid : windows_sys::core::PCWSTR, fullinfpath : windows_sys::core::PCWSTR, installflags : u32, brebootrequired : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
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
