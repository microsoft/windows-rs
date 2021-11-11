#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn CallNtPowerInformation(informationlevel: POWER_INFORMATION_LEVEL, inputbuffer: *const ::core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::core::ffi::c_void, outputbufferlength: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanUserWritePwrScheme() -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePwrScheme(uiid: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DevicePowerClose() -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DevicePowerEnumDevices(queryindex: u32, queryinterpretationflags: u32, queryflags: u32, preturnbuffer: *mut u8, pbuffersize: *mut u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DevicePowerOpen(debugmask: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DevicePowerSetDeviceState(devicedescription: super::super::Foundation::PWSTR, setflags: u32, setdata: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPwrSchemes(lpfn: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetActivePwrScheme(puiid: *mut u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPowerPolicies(pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDevicePowerState(hdevice: super::super::Foundation::HANDLE, pfon: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPwrCapabilities(lpspc: *mut SYSTEM_POWER_CAPABILITIES) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPwrDiskSpindownRange(puimax: *mut u32, puimin: *mut u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemPowerStatus(lpsystempowerstatus: *mut SYSTEM_POWER_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsAdminOverrideActive(papp: *const ADMINISTRATOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsPwrHibernateAllowed() -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsPwrShutdownAllowed() -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsPwrSuspendAllowed() -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsSystemResumeAutomatic() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerClearRequest(powerrequest: super::super::Foundation::HANDLE, requesttype: POWER_REQUEST_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerCreatePossibleSetting(rootsystempowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, possiblesettingindex: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn PowerCreateRequest(context: *const super::Threading::REASON_CONTEXT) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerCreateSetting(rootsystempowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerDeleteScheme(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerDeterminePlatformRole() -> POWER_PLATFORM_ROLE;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerDeterminePlatformRoleEx(version: POWER_PLATFORM_ROLE_VERSION) -> POWER_PLATFORM_ROLE;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerDuplicateScheme(rootpowerkey: super::Registry::HKEY, sourceschemeguid: *const ::windows::runtime::GUID, destinationschemeguid: *mut *mut ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerEnumerate(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, accessflags: POWER_DATA_ACCESSOR, index: u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerGetActiveScheme(userrootpowerkey: super::Registry::HKEY, activepolicyguid: *mut *mut ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PowerImportPowerScheme(rootpowerkey: super::Registry::HKEY, importfilenamepath: super::super::Foundation::PWSTR, destinationschemeguid: *mut *mut ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerIsSettingRangeDefined(subkeyguid: *const ::windows::runtime::GUID, settingguid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PowerOpenSystemPowerKey(phsystempowerkey: *mut super::Registry::HKEY, access: u32, openexisting: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PowerOpenUserPowerKey(phuserpowerkey: *mut super::Registry::HKEY, access: u32, openexisting: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadACDefaultIndex(rootpowerkey: super::Registry::HKEY, schemepersonalityguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, acdefaultindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadACValue(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, r#type: *mut u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadACValueIndex(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, acvalueindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadDCDefaultIndex(rootpowerkey: super::Registry::HKEY, schemepersonalityguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, dcdefaultindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadDCValue(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, r#type: *mut u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadDCValueIndex(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, dcvalueindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadDescription(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadFriendlyName(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadIconResourceSpecifier(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadPossibleDescription(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadPossibleFriendlyName(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadPossibleValue(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, r#type: *mut u32, possiblesettingindex: u32, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerReadSettingAttributes(subgroupguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadValueIncrement(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, valueincrement: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadValueMax(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, valuemaximum: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadValueMin(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, valueminimum: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadValueUnitsSpecifier(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, buffer: *mut u8, buffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerRegisterForEffectivePowerModeNotifications(version: u32, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, registrationhandle: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerRegisterSuspendResumeNotification(flags: u32, recipient: super::super::Foundation::HANDLE, registrationhandle: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerRemovePowerSetting(powersettingsubkeyguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerReplaceDefaultPowerSchemes() -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerReportThermalEvent(event: *const THERMAL_EVENT) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerRestoreDefaultPowerSchemes() -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerRestoreIndividualDefaultPowerScheme(schemeguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerSetActiveScheme(userrootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerSetRequest(powerrequest: super::super::Foundation::HANDLE, requesttype: POWER_REQUEST_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerSettingAccessCheck(accessflags: POWER_DATA_ACCESSOR, powerguid: *const ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerSettingAccessCheckEx(accessflags: POWER_DATA_ACCESSOR, powerguid: *const ::windows::runtime::GUID, accesstype: super::Registry::REG_SAM_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerSettingRegisterNotification(settingguid: *const ::windows::runtime::GUID, flags: POWER_SETTING_REGISTER_NOTIFICATION_FLAGS, recipient: super::super::Foundation::HANDLE, registrationhandle: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerSettingUnregisterNotification(registrationhandle: HPOWERNOTIFY) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerUnregisterSuspendResumeNotification(registrationhandle: HPOWERNOTIFY) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteACDefaultIndex(rootsystempowerkey: super::Registry::HKEY, schemepersonalityguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, defaultacindex: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteACValueIndex(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, acvalueindex: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteDCDefaultIndex(rootsystempowerkey: super::Registry::HKEY, schemepersonalityguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, defaultdcindex: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteDCValueIndex(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, dcvalueindex: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteDescription(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, buffer: *const u8, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteFriendlyName(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, buffer: *const u8, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteIconResourceSpecifier(rootpowerkey: super::Registry::HKEY, schemeguid: *const ::windows::runtime::GUID, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, buffer: *const u8, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWritePossibleDescription(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWritePossibleFriendlyName(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWritePossibleValue(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, r#type: u32, possiblesettingindex: u32, buffer: *const u8, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerWriteSettingAttributes(subgroupguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, attributes: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteValueIncrement(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, valueincrement: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteValueMax(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, valuemaximum: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteValueMin(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, valueminimum: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteValueUnitsSpecifier(rootpowerkey: super::Registry::HKEY, subgroupofpowersettingsguid: *const ::windows::runtime::GUID, powersettingguid: *const ::windows::runtime::GUID, buffer: *const u8, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *mut MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadPwrScheme(uiid: u32, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterPowerSettingNotification(hrecipient: super::super::Foundation::HANDLE, powersettingguid: *const ::windows::runtime::GUID, flags: u32) -> HPOWERNOTIFY;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterSuspendResumeNotification(hrecipient: super::super::Foundation::HANDLE, flags: u32) -> HPOWERNOTIFY;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RequestWakeupLatency(latency: LATENCY_TIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetActivePwrScheme(uiid: u32, pglobalpowerpolicy: *const GLOBAL_POWER_POLICY, ppowerpolicy: *const POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSuspendState(bhibernate: super::super::Foundation::BOOLEAN, bforce: super::super::Foundation::BOOLEAN, bwakeupeventsdisabled: super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemPowerState(fsuspend: super::super::Foundation::BOOL, fforce: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn SetThreadExecutionState(esflags: EXECUTION_STATE) -> EXECUTION_STATE;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterPowerSettingNotification(handle: HPOWERNOTIFY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterSuspendResumeNotification(handle: HPOWERNOTIFY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidatePowerPolicies(pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *const MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePwrScheme(puiid: *const u32, lpszschemename: super::super::Foundation::PWSTR, lpszdescription: super::super::Foundation::PWSTR, lpscheme: *const POWER_POLICY) -> super::super::Foundation::BOOLEAN;
}
