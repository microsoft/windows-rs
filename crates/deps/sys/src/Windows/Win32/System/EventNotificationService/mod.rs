#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableA(lpszdestination: super::super::Foundation::PSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableW(lpszdestination: super::super::Foundation::PWSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNetworkAlive(lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `Win32_System_EventNotificationService`*"]
pub const CONNECTION_AOL: u32 = 4u32;
pub struct ISensLogon(i32);
pub struct ISensLogon2(i32);
pub struct ISensNetwork(i32);
pub struct ISensOnNow(i32);
#[doc = "*Required features: `Win32_System_EventNotificationService`*"]
pub const NETWORK_ALIVE_AOL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_EventNotificationService`*"]
pub const NETWORK_ALIVE_INTERNET: u32 = 8u32;
#[doc = "*Required features: `Win32_System_EventNotificationService`*"]
pub const NETWORK_ALIVE_LAN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_EventNotificationService`*"]
pub const NETWORK_ALIVE_WAN: u32 = 2u32;
pub struct QOCINFO(i32);
pub struct SENS(i32);
pub const SENSGUID_EVENTCLASS_LOGON: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3583477296, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_LOGON2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3583477328, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_NETWORK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3583477280, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_ONNOW: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3583477312, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_PUBLISHER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1609440214, data2: 23451, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_SUBSCRIBER_LCE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3549661872, data2: 23453, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_SUBSCRIBER_WININET: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3549661877, data2: 23453, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub struct SENS_CONNECTION_TYPE(i32);
pub struct SENS_QOCINFO(i32);
