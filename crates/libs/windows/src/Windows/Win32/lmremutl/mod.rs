#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetRemoteComputerSupports<P0>(uncservername: P0, optionswanted: super::DWORD, optionssupported: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRemoteComputerSupports(uncservername : windows_core::PCWSTR, optionswanted : super::DWORD, optionssupported : super::LPDWORD) -> super::DWORD);
    unsafe { NetRemoteComputerSupports(uncservername.param().abi(), optionswanted, optionssupported as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetRemoteTOD<P0>(uncservername: P0, bufferptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRemoteTOD(uncservername : windows_core::PCWSTR, bufferptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetRemoteTOD(uncservername.param().abi(), bufferptr as _) }
}
pub const ALLOCATE_RESPONSE: i32 = 2;
pub type DESC_CHAR = i8;
pub type LPDESC = *mut i8;
#[cfg(feature = "shared")]
pub type LPTIME_OF_DAY_INFO = *mut TIME_OF_DAY_INFO;
pub const NO_PERMISSION_REQUIRED: i32 = 1;
#[cfg(feature = "shared")]
pub type PTIME_OF_DAY_INFO = *mut TIME_OF_DAY_INFO;
pub const SUPPORTS_ANY: u32 = 4294967295;
pub const SUPPORTS_LOCAL: i32 = 32;
pub const SUPPORTS_REMOTE_ADMIN_PROTOCOL: i32 = 2;
pub const SUPPORTS_RPC: i32 = 4;
pub const SUPPORTS_SAM_PROTOCOL: i32 = 8;
pub const SUPPORTS_UNICODE: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TIME_OF_DAY_INFO {
    pub tod_elapsedt: super::DWORD,
    pub tod_msecs: super::DWORD,
    pub tod_hours: super::DWORD,
    pub tod_mins: super::DWORD,
    pub tod_secs: super::DWORD,
    pub tod_hunds: super::DWORD,
    pub tod_timezone: super::LONG,
    pub tod_tinterval: super::DWORD,
    pub tod_day: super::DWORD,
    pub tod_month: super::DWORD,
    pub tod_year: super::DWORD,
    pub tod_weekday: super::DWORD,
}
pub const USE_SPECIFIC_TRANSPORT: u32 = 2147483648;
