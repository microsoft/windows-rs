#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDynamicTimeZoneInformation();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToSystemTime();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDynamicTimeZoneInformation();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDynamicTimeZoneInformationEffectiveYears();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeZoneInformation();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeZoneInformationForYear();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalFileTimeToLocalSystemTime();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalSystemTimeToLocalFileTime();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDynamicTimeZoneInformation();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTimeZoneInformation();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToFileTime();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToTzSpecificLocalTime();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToTzSpecificLocalTimeEx();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TzSpecificLocalTimeToSystemTime();
    #[doc = "*Required features: `Win32_System_Time`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TzSpecificLocalTimeToSystemTimeEx();
}
