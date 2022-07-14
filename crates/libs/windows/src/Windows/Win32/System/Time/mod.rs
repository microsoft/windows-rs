#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DYNAMIC_TIME_ZONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DYNAMIC_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DYNAMIC_TIME_ZONE_INFORMATION").field("Bias", &self.Bias).field("StandardName", &self.StandardName).field("StandardDate", &self.StandardDate).field("StandardBias", &self.StandardBias).field("DaylightName", &self.DaylightName).field("DaylightDate", &self.DaylightDate).field("DaylightBias", &self.DaylightBias).field("TimeZoneKeyName", &self.TimeZoneKeyName).field("DynamicDaylightTimeDisabled", &self.DynamicDaylightTimeDisabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DYNAMIC_TIME_ZONE_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DYNAMIC_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DYNAMIC_TIME_ZONE_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DYNAMIC_TIME_ZONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDynamicTimeZoneInformation(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDynamicTimeZoneInformation(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
    }
    EnumDynamicTimeZoneInformation(dwindex, ::core::mem::transmute(lptimezoneinformation))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileTimeToSystemTime(lpfiletime: *const super::super::Foundation::FILETIME, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FileTimeToSystemTime(lpfiletime: *const super::super::Foundation::FILETIME, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    }
    FileTimeToSystemTime(::core::mem::transmute(lpfiletime), ::core::mem::transmute(lpsystemtime))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDynamicTimeZoneInformation(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDynamicTimeZoneInformation(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32;
    }
    GetDynamicTimeZoneInformation(::core::mem::transmute(ptimezoneinformation))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32;
    }
    GetDynamicTimeZoneInformationEffectiveYears(::core::mem::transmute(lptimezoneinformation), ::core::mem::transmute(firstyear), ::core::mem::transmute(lastyear))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTimeZoneInformation(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTimeZoneInformation(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32;
    }
    GetTimeZoneInformation(::core::mem::transmute(lptimezoneinformation))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTimeZoneInformationForYear(wyear: u16, pdtzi: *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi: *mut TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTimeZoneInformationForYear(wyear: u16, pdtzi: *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi: *mut TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    }
    GetTimeZoneInformationForYear(wyear, ::core::mem::transmute(pdtzi), ::core::mem::transmute(ptzi))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalFileTimeToLocalSystemTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localfiletime: *const super::super::Foundation::FILETIME, localsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LocalFileTimeToLocalSystemTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localfiletime: *const super::super::Foundation::FILETIME, localsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    }
    LocalFileTimeToLocalSystemTime(::core::mem::transmute(timezoneinformation), ::core::mem::transmute(localfiletime), ::core::mem::transmute(localsystemtime))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalSystemTimeToLocalFileTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localsystemtime: *const super::super::Foundation::SYSTEMTIME, localfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LocalSystemTimeToLocalFileTime(timezoneinformation: *const TIME_ZONE_INFORMATION, localsystemtime: *const super::super::Foundation::SYSTEMTIME, localfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    }
    LocalSystemTimeToLocalFileTime(::core::mem::transmute(timezoneinformation), ::core::mem::transmute(localsystemtime), ::core::mem::transmute(localfiletime))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDynamicTimeZoneInformation(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDynamicTimeZoneInformation(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    }
    SetDynamicTimeZoneInformation(::core::mem::transmute(lptimezoneinformation))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTimeZoneInformation(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetTimeZoneInformation(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> super::super::Foundation::BOOL;
    }
    SetTimeZoneInformation(::core::mem::transmute(lptimezoneinformation))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemTimeToFileTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SystemTimeToFileTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    }
    SystemTimeToFileTime(::core::mem::transmute(lpsystemtime), ::core::mem::transmute(lpfiletime))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    }
    SystemTimeToTzSpecificLocalTime(::core::mem::transmute(lptimezoneinformation), ::core::mem::transmute(lpuniversaltime), ::core::mem::transmute(lplocaltime))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    }
    SystemTimeToTzSpecificLocalTimeEx(::core::mem::transmute(lptimezoneinformation), ::core::mem::transmute(lpuniversaltime), ::core::mem::transmute(lplocaltime))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TIME_ZONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIME_ZONE_INFORMATION").field("Bias", &self.Bias).field("StandardName", &self.StandardName).field("StandardDate", &self.StandardDate).field("StandardBias", &self.StandardBias).field("DaylightName", &self.DaylightName).field("DaylightDate", &self.DaylightDate).field("DaylightBias", &self.DaylightBias).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TIME_ZONE_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TIME_ZONE_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TIME_ZONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const TSF_Authenticated: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const TSF_Hardware: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const TSF_IPv6: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const TSF_SignatureAuthenticated: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation: *const TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    }
    TzSpecificLocalTimeToSystemTime(::core::mem::transmute(lptimezoneinformation), ::core::mem::transmute(lplocaltime), ::core::mem::transmute(lpuniversaltime))
}
#[doc = "*Required features: `\"Win32_System_Time\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    }
    TzSpecificLocalTimeToSystemTimeEx(::core::mem::transmute(lptimezoneinformation), ::core::mem::transmute(lplocaltime), ::core::mem::transmute(lpuniversaltime))
}
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const wszW32TimeRegKeyPolicyTimeProviders: &str = "Software\\Policies\\Microsoft\\W32Time\\TimeProviders";
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const wszW32TimeRegKeyTimeProviders: &str = "System\\CurrentControlSet\\Services\\W32Time\\TimeProviders";
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const wszW32TimeRegValueDllName: &str = "DllName";
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const wszW32TimeRegValueEnabled: &str = "Enabled";
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const wszW32TimeRegValueInputProvider: &str = "InputProvider";
#[doc = "*Required features: `\"Win32_System_Time\"`*"]
pub const wszW32TimeRegValueMetaDataProvider: &str = "MetaDataProvider";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
