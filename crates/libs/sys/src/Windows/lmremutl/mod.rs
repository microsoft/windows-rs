windows_link::link!("netapi32.dll" "system" fn NetRemoteComputerSupports(uncservername : windows_sys::core::PCWSTR, optionswanted : u32, optionssupported : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetRemoteTOD(uncservername : windows_sys::core::PCWSTR, bufferptr : *mut super::minwindef::LPBYTE) -> u32);
pub const ALLOCATE_RESPONSE: u32 = 2;
pub type DESC_CHAR = i8;
pub type LPDESC = windows_sys::core::PSTR;
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
#[derive(Clone, Copy, Default)]
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
