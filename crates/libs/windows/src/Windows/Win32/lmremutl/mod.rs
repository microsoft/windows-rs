#[inline]
pub unsafe fn NetRemoteComputerSupports<P0>(uncservername: P0, optionswanted: u32, optionssupported: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRemoteComputerSupports(uncservername : windows_core::PCWSTR, optionswanted : u32, optionssupported : *mut u32) -> u32);
    unsafe { NetRemoteComputerSupports(uncservername.param().abi(), optionswanted, optionssupported as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetRemoteTOD<P0>(uncservername: P0, bufferptr: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRemoteTOD(uncservername : windows_core::PCWSTR, bufferptr : *mut super::LPBYTE) -> u32);
    unsafe { NetRemoteTOD(uncservername.param().abi(), bufferptr as _) }
}
pub const ALLOCATE_RESPONSE: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DESC_CHAR(pub i8);
pub type LPDESC = windows_core::PSTR;
pub type LPTIME_OF_DAY_INFO = *mut TIME_OF_DAY_INFO;
pub const NO_PERMISSION_REQUIRED: u32 = 1;
pub type PTIME_OF_DAY_INFO = *mut TIME_OF_DAY_INFO;
pub const SUPPORTS_ANY: u32 = 4294967295;
pub const SUPPORTS_LOCAL: u32 = 32;
pub const SUPPORTS_REMOTE_ADMIN_PROTOCOL: u32 = 2;
pub const SUPPORTS_RPC: u32 = 4;
pub const SUPPORTS_SAM_PROTOCOL: u32 = 8;
pub const SUPPORTS_UNICODE: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TIME_OF_DAY_INFO {
    pub tod_elapsedt: u32,
    pub tod_msecs: u32,
    pub tod_hours: u32,
    pub tod_mins: u32,
    pub tod_secs: u32,
    pub tod_hunds: u32,
    pub tod_timezone: i32,
    pub tod_tinterval: u32,
    pub tod_day: u32,
    pub tod_month: u32,
    pub tod_year: u32,
    pub tod_weekday: u32,
}
pub const USE_SPECIFIC_TRANSPORT: u32 = 2147483648;
