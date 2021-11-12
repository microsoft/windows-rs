#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDynamicTimeZoneInformation(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToSystemTime(lpfiletime: *const super::super::Foundation::FILETIME, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDynamicTimeZoneInformation(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeZoneInformation(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeZoneInformationForYear(wyear: u16, pdtzi: *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi: *mut TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalFileTimeToLocalSystemTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localfiletime: *const super::super::Foundation::FILETIME, localsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalSystemTimeToLocalFileTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localsystemtime: *const super::super::Foundation::SYSTEMTIME, localfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDynamicTimeZoneInformation(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTimeZoneInformation(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToFileTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TIME_ZONE_INFORMATION(i32);
pub const TSF_Authenticated: u32 = 2u32;
pub const TSF_Hardware: u32 = 1u32;
pub const TSF_IPv6: u32 = 4u32;
pub const TSF_SignatureAuthenticated: u32 = 8u32;
