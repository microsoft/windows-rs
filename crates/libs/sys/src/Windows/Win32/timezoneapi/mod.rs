#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn EnumDynamicTimeZoneInformation(dwindex : super::DWORD, lptimezoneinformation : PDYNAMIC_TIME_ZONE_INFORMATION) -> super::DWORD);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn FileTimeToSystemTime(lpfiletime : *const super::FILETIME, lpsystemtime : super::LPSYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetDynamicTimeZoneInformation(ptimezoneinformation : PDYNAMIC_TIME_ZONE_INFORMATION) -> super::DWORD);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation : PDYNAMIC_TIME_ZONE_INFORMATION, firstyear : super::LPDWORD, lastyear : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetTimeZoneInformation(lptimezoneinformation : LPTIME_ZONE_INFORMATION) -> super::DWORD);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetTimeZoneInformationForYear(wyear : super::USHORT, pdtzi : PDYNAMIC_TIME_ZONE_INFORMATION, ptzi : LPTIME_ZONE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalFileTimeToLocalSystemTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localfiletime : *const super::FILETIME, localsystemtime : *mut super::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalSystemTimeToLocalFileTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localsystemtime : *const super::SYSTEMTIME, localfiletime : *mut super::FILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetDynamicTimeZoneInformation(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetTimeZoneInformation(lptimezoneinformation : *const TIME_ZONE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn SystemTimeToFileTime(lpsystemtime : *const super::SYSTEMTIME, lpfiletime : super::LPFILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lpuniversaltime : *const super::SYSTEMTIME, lplocaltime : super::LPSYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime : *const super::SYSTEMTIME, lplocaltime : super::LPSYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lplocaltime : *const super::SYSTEMTIME, lpuniversaltime : super::LPSYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime : *const super::SYSTEMTIME, lpuniversaltime : super::LPSYSTEMTIME) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: super::LONG,
    pub StandardName: [super::WCHAR; 32],
    pub StandardDate: super::SYSTEMTIME,
    pub StandardBias: super::LONG,
    pub DaylightName: [super::WCHAR; 32],
    pub DaylightDate: super::SYSTEMTIME,
    pub DaylightBias: super::LONG,
    pub TimeZoneKeyName: [super::WCHAR; 128],
    pub DynamicDaylightTimeDisabled: super::BOOLEAN,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
impl Default for DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type LPTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PDYNAMIC_TIME_ZONE_INFORMATION = *mut DYNAMIC_TIME_ZONE_INFORMATION;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type PTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
#[cfg(feature = "shared")]
pub const TIME_ZONE_ID_INVALID: super::DWORD = 4294967295;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: super::LONG,
    pub StandardName: [super::WCHAR; 32],
    pub StandardDate: super::SYSTEMTIME,
    pub StandardBias: super::LONG,
    pub DaylightName: [super::WCHAR; 32],
    pub DaylightDate: super::SYSTEMTIME,
    pub DaylightBias: super::LONG,
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
impl Default for TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
