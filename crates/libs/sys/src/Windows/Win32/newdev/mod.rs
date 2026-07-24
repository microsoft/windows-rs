#[cfg(all(feature = "minwindef", feature = "setupapi", feature = "windef", feature = "winnt"))]
windows_link::link!("newdev.dll" "system" fn DiInstallDevice(hwndparent : super::HWND, deviceinfoset : super::HDEVINFO, deviceinfodata : *const super::SP_DEVINFO_DATA, driverinfodata : super::PSP_DRVINFO_DATA, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("newdev.dll" "system" fn DiInstallDriverA(hwndparent : super::HWND, infpath : windows_sys::core::PCSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("newdev.dll" "system" fn DiInstallDriverW(hwndparent : super::HWND, infpath : windows_sys::core::PCWSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "setupapi", feature = "windef"))]
windows_link::link!("newdev.dll" "system" fn DiRollbackDriver(deviceinfoset : super::HDEVINFO, deviceinfodata : *const super::SP_DEVINFO_DATA, hwndparent : super::HWND, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "setupapi", feature = "windef"))]
windows_link::link!("newdev.dll" "system" fn DiShowUpdateDevice(hwndparent : super::HWND, deviceinfoset : super::HDEVINFO, deviceinfodata : *const super::SP_DEVINFO_DATA, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("newdev.dll" "system" fn DiShowUpdateDriver(hwndparent : super::HWND, filepath : windows_sys::core::PCWSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "setupapi", feature = "windef"))]
windows_link::link!("newdev.dll" "system" fn DiUninstallDevice(hwndparent : super::HWND, deviceinfoset : super::HDEVINFO, deviceinfodata : *const super::SP_DEVINFO_DATA, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("newdev.dll" "system" fn DiUninstallDriverA(hwndparent : super::HWND, infpath : windows_sys::core::PCSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("newdev.dll" "system" fn DiUninstallDriverW(hwndparent : super::HWND, infpath : windows_sys::core::PCWSTR, flags : u32, needreboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("newdev.dll" "system" fn UpdateDriverForPlugAndPlayDevicesA(hwndparent : super::HWND, hardwareid : windows_sys::core::PCSTR, fullinfpath : windows_sys::core::PCSTR, installflags : u32, brebootrequired : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("newdev.dll" "system" fn UpdateDriverForPlugAndPlayDevicesW(hwndparent : super::HWND, hardwareid : windows_sys::core::PCWSTR, fullinfpath : windows_sys::core::PCWSTR, installflags : u32, brebootrequired : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub const DIIDFLAG_BITS: i32 = 15;
pub const DIIDFLAG_INSTALLCOPYINFDRIVERS: i32 = 8;
pub const DIIDFLAG_INSTALLNULLDRIVER: i32 = 4;
pub const DIIDFLAG_NOFINISHINSTALLUI: i32 = 2;
pub const DIIDFLAG_SHOWSEARCHUI: i32 = 1;
pub const DIIRFLAG_BITS: i32 = 106;
pub const DIIRFLAG_FORCE_INF: i32 = 2;
pub const DIIRFLAG_HOTPATCH: i32 = 8;
pub const DIIRFLAG_HW_USING_THE_INF: i32 = 4;
pub const DIIRFLAG_INF_ALREADY_COPIED: i32 = 1;
pub const DIIRFLAG_INSTALL_AS_SET: i32 = 64;
pub const DIIRFLAG_NOBACKUP: i32 = 16;
pub const DIIRFLAG_PRE_CONFIGURE_INF: i32 = 32;
pub const DIIRFLAG_SYSTEM_BITS: i32 = 127;
pub const DIURFLAG_NO_REMOVE_INF: i32 = 1;
pub const DIURFLAG_RESERVED: i32 = 2;
pub const DIURFLAG_VALID: i32 = 3;
pub const INSTALLFLAG_BITS: i32 = 7;
pub const INSTALLFLAG_FORCE: i32 = 1;
pub const INSTALLFLAG_NONINTERACTIVE: i32 = 4;
pub const INSTALLFLAG_READONLY: i32 = 2;
pub const ROLLBACK_BITS: i32 = 1;
pub const ROLLBACK_FLAG_NO_UI: i32 = 1;
