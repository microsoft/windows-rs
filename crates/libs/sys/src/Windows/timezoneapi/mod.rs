#[cfg(feature = "minwinbase")]
windows_link::link!("advapi32.dll" "system" fn EnumDynamicTimeZoneInformation(dwindex : u32, lptimezoneinformation : *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn FileTimeToSystemTime(lpfiletime : *const super::minwindef::FILETIME, lpsystemtime : *mut super::minwinbase::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetDynamicTimeZoneInformation(ptimezoneinformation : *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32);
#[cfg(feature = "minwinbase")]
windows_link::link!("advapi32.dll" "system" fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear : *mut u32, lastyear : *mut u32) -> u32);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetTimeZoneInformation(lptimezoneinformation : *mut TIME_ZONE_INFORMATION) -> u32);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetTimeZoneInformationForYear(wyear : u16, pdtzi : *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi : *mut TIME_ZONE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn LocalFileTimeToLocalSystemTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localfiletime : *const super::minwindef::FILETIME, localsystemtime : *mut super::minwinbase::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn LocalSystemTimeToLocalFileTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localsystemtime : *const super::minwinbase::SYSTEMTIME, localfiletime : *mut super::minwindef::FILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn SetDynamicTimeZoneInformation(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn SetTimeZoneInformation(lptimezoneinformation : *const TIME_ZONE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn SystemTimeToFileTime(lpsystemtime : *const super::minwinbase::SYSTEMTIME, lpfiletime : *mut super::minwindef::FILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lpuniversaltime : *const super::minwinbase::SYSTEMTIME, lplocaltime : *mut super::minwinbase::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime : *const super::minwinbase::SYSTEMTIME, lplocaltime : *mut super::minwinbase::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lplocaltime : *const super::minwinbase::SYSTEMTIME, lpuniversaltime : *mut super::minwinbase::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime : *const super::minwinbase::SYSTEMTIME, lpuniversaltime : *mut super::minwinbase::SYSTEMTIME) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "minwinbase")]
#[derive(Clone, Copy)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::minwinbase::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::minwinbase::SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: bool,
}
#[cfg(feature = "minwinbase")]
impl Default for DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwinbase")]
pub type LPTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
#[cfg(feature = "minwinbase")]
pub type PDYNAMIC_TIME_ZONE_INFORMATION = *mut DYNAMIC_TIME_ZONE_INFORMATION;
#[cfg(feature = "minwinbase")]
pub type PTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
pub const TIME_ZONE_ID_INVALID: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "minwinbase")]
#[derive(Clone, Copy)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::minwinbase::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::minwinbase::SYSTEMTIME,
    pub DaylightBias: i32,
}
#[cfg(feature = "minwinbase")]
impl Default for TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
