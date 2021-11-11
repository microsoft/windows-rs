#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDynamicTimeZoneInformation(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToSystemTime(lpfiletime: *const super::super::Foundation::FILETIME, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDynamicTimeZoneInformation(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeZoneInformation(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeZoneInformationForYear(wyear: u16, pdtzi: *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi: *mut TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalFileTimeToLocalSystemTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localfiletime: *const super::super::Foundation::FILETIME, localsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalSystemTimeToLocalFileTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localsystemtime: *const super::super::Foundation::SYSTEMTIME, localfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDynamicTimeZoneInformation(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTimeZoneInformation(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToFileTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
}
