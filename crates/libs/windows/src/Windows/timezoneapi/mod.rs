#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn EnumDynamicTimeZoneInformation(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EnumDynamicTimeZoneInformation(dwindex : u32, lptimezoneinformation : *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32);
    unsafe { EnumDynamicTimeZoneInformation(dwindex, lptimezoneinformation as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn FileTimeToSystemTime(lpfiletime: *const super::minwindef::FILETIME, lpsystemtime: *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FileTimeToSystemTime(lpfiletime : *const super::minwindef::FILETIME, lpsystemtime : *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { FileTimeToSystemTime(lpfiletime, lpsystemtime as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetDynamicTimeZoneInformation(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetDynamicTimeZoneInformation(ptimezoneinformation : *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32);
    unsafe { GetDynamicTimeZoneInformation(ptimezoneinformation as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear : *mut u32, lastyear : *mut u32) -> u32);
    unsafe { GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation, firstyear as _, lastyear as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetTimeZoneInformation(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTimeZoneInformation(lptimezoneinformation : *mut TIME_ZONE_INFORMATION) -> u32);
    unsafe { GetTimeZoneInformation(lptimezoneinformation as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetTimeZoneInformationForYear(wyear: u16, pdtzi: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, ptzi: *mut TIME_ZONE_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetTimeZoneInformationForYear(wyear : u16, pdtzi : *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi : *mut TIME_ZONE_INFORMATION) -> windows_core::BOOL);
    unsafe { GetTimeZoneInformationForYear(wyear, pdtzi.unwrap_or(core::mem::zeroed()) as _, ptzi as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn LocalFileTimeToLocalSystemTime(timezoneinformation: Option<*const TIME_ZONE_INFORMATION>, localfiletime: *const super::minwindef::FILETIME, localsystemtime: *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LocalFileTimeToLocalSystemTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localfiletime : *const super::minwindef::FILETIME, localsystemtime : *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { LocalFileTimeToLocalSystemTime(timezoneinformation.unwrap_or(core::mem::zeroed()) as _, localfiletime, localsystemtime as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn LocalSystemTimeToLocalFileTime(timezoneinformation: Option<*const TIME_ZONE_INFORMATION>, localsystemtime: *const super::minwinbase::SYSTEMTIME, localfiletime: *mut super::minwindef::FILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LocalSystemTimeToLocalFileTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localsystemtime : *const super::minwinbase::SYSTEMTIME, localfiletime : *mut super::minwindef::FILETIME) -> windows_core::BOOL);
    unsafe { LocalSystemTimeToLocalFileTime(timezoneinformation.unwrap_or(core::mem::zeroed()) as _, localsystemtime, localfiletime as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn SetDynamicTimeZoneInformation(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetDynamicTimeZoneInformation(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION) -> windows_core::BOOL);
    unsafe { SetDynamicTimeZoneInformation(lptimezoneinformation) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn SetTimeZoneInformation(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetTimeZoneInformation(lptimezoneinformation : *const TIME_ZONE_INFORMATION) -> windows_core::BOOL);
    unsafe { SetTimeZoneInformation(lptimezoneinformation) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn SystemTimeToFileTime(lpsystemtime: *const super::minwinbase::SYSTEMTIME, lpfiletime: *mut super::minwindef::FILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SystemTimeToFileTime(lpsystemtime : *const super::minwinbase::SYSTEMTIME, lpfiletime : *mut super::minwindef::FILETIME) -> windows_core::BOOL);
    unsafe { SystemTimeToFileTime(lpsystemtime, lpfiletime as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation: Option<*const TIME_ZONE_INFORMATION>, lpuniversaltime: *const super::minwinbase::SYSTEMTIME, lplocaltime: *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lpuniversaltime : *const super::minwinbase::SYSTEMTIME, lplocaltime : *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SystemTimeToTzSpecificLocalTime(lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _, lpuniversaltime, lplocaltime as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, lpuniversaltime: *const super::minwinbase::SYSTEMTIME, lplocaltime: *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime : *const super::minwinbase::SYSTEMTIME, lplocaltime : *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _, lpuniversaltime, lplocaltime as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation: Option<*const TIME_ZONE_INFORMATION>, lplocaltime: *const super::minwinbase::SYSTEMTIME, lpuniversaltime: *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lplocaltime : *const super::minwinbase::SYSTEMTIME, lpuniversaltime : *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { TzSpecificLocalTimeToSystemTime(lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _, lplocaltime, lpuniversaltime as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, lplocaltime: *const super::minwinbase::SYSTEMTIME, lpuniversaltime: *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime : *const super::minwinbase::SYSTEMTIME, lpuniversaltime : *mut super::minwinbase::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _, lplocaltime, lpuniversaltime as _) }
}
#[repr(C)]
#[cfg(feature = "minwinbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
