#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn CanUserWritePwrScheme() -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn DeletePwrScheme(uiid : super::UINT) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn DevicePowerClose() -> super::BOOLEAN);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn DevicePowerEnumDevices(queryindex : super::ULONG, queryinterpretationflags : super::ULONG, queryflags : super::ULONG, preturnbuffer : super::PBYTE, pbuffersize : super::PULONG) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn DevicePowerOpen(debugmask : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn DevicePowerSetDeviceState(devicedescription : windows_sys::core::PCWSTR, setflags : super::ULONG, setdata : super::PVOID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn EnumPwrSchemes(lpfn : PWRSCHEMESENUMPROC, lparam : super::LPARAM) -> super::BOOLEAN);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn GetActivePwrScheme(puiid : super::PUINT) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn GetCurrentPowerPolicies(pglobalpowerpolicy : PGLOBAL_POWER_POLICY, ppowerpolicy : PPOWER_POLICY) -> super::BOOLEAN);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn GetPwrDiskSpindownRange(puimax : super::PUINT, puimin : super::PUINT) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn IsAdminOverrideActive(papp : super::PADMINISTRATOR_POWER_POLICY) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn IsPwrHibernateAllowed() -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn IsPwrShutdownAllowed() -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn IsPwrSuspendAllowed() -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerCreatePossibleSetting(rootsystempowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : super::ULONG) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerCreateSetting(rootsystempowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerDeleteScheme(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn PowerDeterminePlatformRole() -> super::POWER_PLATFORM_ROLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerDuplicateScheme(rootpowerkey : super::HKEY, sourceschemeguid : *const windows_sys::core::GUID, destinationschemeguid : *mut *mut windows_sys::core::GUID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerEnumerate(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, accessflags : POWER_DATA_ACCESSOR, index : super::ULONG, buffer : *mut super::UCHAR, buffersize : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerGetUserConfiguredACPowerMode(powermodeguid : *mut windows_sys::core::GUID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerGetUserConfiguredDCPowerMode(powermodeguid : *mut windows_sys::core::GUID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerImportPowerScheme(rootpowerkey : super::HKEY, importfilenamepath : windows_sys::core::PCWSTR, destinationschemeguid : *mut *mut windows_sys::core::GUID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerIsSettingRangeDefined(subkeyguid : *const windows_sys::core::GUID, settingguid : *const windows_sys::core::GUID) -> super::BOOLEAN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("powrprof.dll" "system" fn PowerOpenSystemPowerKey(phsystempowerkey : *mut super::HKEY, access : super::REGSAM, openexisting : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
windows_link::link!("powrprof.dll" "system" fn PowerOpenUserPowerKey(phuserpowerkey : *mut super::HKEY, access : super::REGSAM, openexisting : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadACDefaultIndex(rootpowerkey : super::HKEY, schemepersonalityguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, acdefaultindex : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadACValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, acvalueindex : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadDCDefaultIndex(rootpowerkey : super::HKEY, schemepersonalityguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, dcdefaultindex : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadDCValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, dcvalueindex : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadDescription(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : super::PUCHAR, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadFriendlyName(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : super::PUCHAR, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadIconResourceSpecifier(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : super::PUCHAR, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadPossibleDescription(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : super::ULONG, buffer : super::PUCHAR, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadPossibleFriendlyName(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : super::ULONG, buffer : super::PUCHAR, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadPossibleValue(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, r#type : super::PULONG, possiblesettingindex : super::ULONG, buffer : super::PUCHAR, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerReadSettingAttributes(subgroupguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadValueIncrement(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valueincrement : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadValueMax(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valuemaximum : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadValueMin(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valueminimum : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadValueUnitsSpecifier(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *mut super::UCHAR, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerRemovePowerSetting(powersettingsubkeyguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerReplaceDefaultPowerSchemes() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerReportThermalEvent(event : PTHERMAL_EVENT) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerRestoreDefaultPowerSchemes() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerRestoreIndividualDefaultPowerScheme(schemeguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerSetUserConfiguredACPowerMode(powermodeguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerSetUserConfiguredDCPowerMode(powermodeguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerSettingAccessCheck(accessflags : POWER_DATA_ACCESSOR, powerguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winreg"))]
windows_link::link!("powrprof.dll" "system" fn PowerSettingAccessCheckEx(accessflags : POWER_DATA_ACCESSOR, powerguid : *const windows_sys::core::GUID, accesstype : super::REGSAM) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteACDefaultIndex(rootsystempowerkey : super::HKEY, schemepersonalityguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, defaultacindex : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteDCDefaultIndex(rootsystempowerkey : super::HKEY, schemepersonalityguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, defaultdcindex : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteDescription(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *const super::UCHAR, buffersize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteFriendlyName(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *const super::UCHAR, buffersize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteIconResourceSpecifier(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *const super::UCHAR, buffersize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWritePossibleDescription(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : super::ULONG, buffer : *const super::UCHAR, buffersize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWritePossibleFriendlyName(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : super::ULONG, buffer : *const super::UCHAR, buffersize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWritePossibleValue(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, r#type : super::ULONG, possiblesettingindex : super::ULONG, buffer : *const super::UCHAR, buffersize : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteSettingAttributes(subgroupguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, attributes : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteValueIncrement(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valueincrement : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteValueMax(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valuemaximum : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteValueMin(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valueminimum : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteValueUnitsSpecifier(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *const super::UCHAR, buffersize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn ReadGlobalPwrPolicy(pglobalpowerpolicy : PGLOBAL_POWER_POLICY) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn ReadProcessorPwrScheme(uiid : super::UINT, pmachineprocessorpowerpolicy : PMACHINE_PROCESSOR_POWER_POLICY) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn ReadPwrScheme(uiid : super::UINT, ppowerpolicy : PPOWER_POLICY) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn SetActivePwrScheme(uiid : super::UINT, pglobalpowerpolicy : PGLOBAL_POWER_POLICY, ppowerpolicy : PPOWER_POLICY) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("powrprof.dll" "system" fn SetSuspendState(bhibernate : super::BOOLEAN, bforce : super::BOOLEAN, bwakeupeventsdisabled : super::BOOLEAN) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn ValidatePowerPolicies(pglobalpowerpolicy : PGLOBAL_POWER_POLICY, ppowerpolicy : PPOWER_POLICY) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn WriteGlobalPwrPolicy(pglobalpowerpolicy : PGLOBAL_POWER_POLICY) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn WriteProcessorPwrScheme(uiid : super::UINT, pmachineprocessorpowerpolicy : PMACHINE_PROCESSOR_POWER_POLICY) -> super::BOOLEAN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn WritePwrScheme(puiid : super::PUINT, lpszschemename : windows_sys::core::PCWSTR, lpszdescription : windows_sys::core::PCWSTR, lpscheme : PPOWER_POLICY) -> super::BOOLEAN);
pub const ACCESS_ACTIVE_OVERLAY_SCHEME: POWER_DATA_ACCESSOR = 27;
pub const ACCESS_ACTIVE_SCHEME: POWER_DATA_ACCESSOR = 19;
pub const ACCESS_AC_POWER_SETTING_INDEX: POWER_DATA_ACCESSOR = 0;
pub const ACCESS_AC_POWER_SETTING_MAX: POWER_DATA_ACCESSOR = 21;
pub const ACCESS_AC_POWER_SETTING_MIN: POWER_DATA_ACCESSOR = 23;
pub const ACCESS_ATTRIBUTES: POWER_DATA_ACCESSOR = 15;
pub const ACCESS_CREATE_SCHEME: POWER_DATA_ACCESSOR = 20;
pub const ACCESS_DC_POWER_SETTING_INDEX: POWER_DATA_ACCESSOR = 1;
pub const ACCESS_DC_POWER_SETTING_MAX: POWER_DATA_ACCESSOR = 22;
pub const ACCESS_DC_POWER_SETTING_MIN: POWER_DATA_ACCESSOR = 24;
pub const ACCESS_DEFAULT_AC_POWER_SETTING: POWER_DATA_ACCESSOR = 7;
pub const ACCESS_DEFAULT_DC_POWER_SETTING: POWER_DATA_ACCESSOR = 8;
pub const ACCESS_DEFAULT_SECURITY_DESCRIPTOR: POWER_DATA_ACCESSOR = 14;
pub const ACCESS_DESCRIPTION: POWER_DATA_ACCESSOR = 3;
pub const ACCESS_FRIENDLY_NAME: POWER_DATA_ACCESSOR = 2;
pub const ACCESS_ICON_RESOURCE: POWER_DATA_ACCESSOR = 13;
pub const ACCESS_INDIVIDUAL_SETTING: POWER_DATA_ACCESSOR = 18;
pub const ACCESS_OVERLAY_SCHEME: POWER_DATA_ACCESSOR = 26;
pub const ACCESS_POSSIBLE_POWER_SETTING: POWER_DATA_ACCESSOR = 4;
pub const ACCESS_POSSIBLE_POWER_SETTING_DESCRIPTION: POWER_DATA_ACCESSOR = 6;
pub const ACCESS_POSSIBLE_POWER_SETTING_FRIENDLY_NAME: POWER_DATA_ACCESSOR = 5;
pub const ACCESS_POSSIBLE_VALUE_INCREMENT: POWER_DATA_ACCESSOR = 11;
pub const ACCESS_POSSIBLE_VALUE_MAX: POWER_DATA_ACCESSOR = 10;
pub const ACCESS_POSSIBLE_VALUE_MIN: POWER_DATA_ACCESSOR = 9;
pub const ACCESS_POSSIBLE_VALUE_UNITS: POWER_DATA_ACCESSOR = 12;
pub const ACCESS_POWER_MODE: POWER_DATA_ACCESSOR = 26;
pub const ACCESS_PROFILE: POWER_DATA_ACCESSOR = 25;
pub const ACCESS_SCHEME: POWER_DATA_ACCESSOR = 16;
pub const ACCESS_SUBGROUP: POWER_DATA_ACCESSOR = 17;
pub const DEVICEPOWER_AND_OPERATION: i32 = 1073741824;
pub const DEVICEPOWER_CLEAR_WAKEENABLED: i32 = 2;
pub const DEVICEPOWER_FILTER_DEVICES_PRESENT: i32 = 536870912;
pub const DEVICEPOWER_FILTER_HARDWARE: i32 = 268435456;
pub const DEVICEPOWER_FILTER_ON_NAME: i32 = 33554432;
pub const DEVICEPOWER_FILTER_WAKEENABLED: i32 = 134217728;
pub const DEVICEPOWER_FILTER_WAKEPROGRAMMABLE: i32 = 67108864;
pub const DEVICEPOWER_HARDWAREID: u32 = 2147483648;
pub const DEVICEPOWER_SET_WAKEENABLED: i32 = 1;
pub const DEVICE_NOTIFY_CALLBACK: i32 = 2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type DEVICE_NOTIFY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(context: super::PVOID, r#type: super::ULONG, setting: super::PVOID) -> super::ULONG>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    pub Callback: PDEVICE_NOTIFY_CALLBACK_ROUTINE,
    pub Context: super::PVOID,
}
pub const EnableMultiBatteryDisplay: i32 = 2;
pub const EnablePasswordLogon: i32 = 4;
pub const EnableSysTrayBatteryMeter: i32 = 1;
pub const EnableVideoDimDisplay: i32 = 16;
pub const EnableWakeOnRing: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct GLOBAL_MACHINE_POWER_POLICY {
    pub Revision: super::ULONG,
    pub LidOpenWakeAc: super::SYSTEM_POWER_STATE,
    pub LidOpenWakeDc: super::SYSTEM_POWER_STATE,
    pub BroadcastCapacityResolution: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct GLOBAL_POWER_POLICY {
    pub user: GLOBAL_USER_POWER_POLICY,
    pub mach: GLOBAL_MACHINE_POWER_POLICY,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct GLOBAL_USER_POWER_POLICY {
    pub Revision: super::ULONG,
    pub PowerButtonAc: super::POWER_ACTION_POLICY,
    pub PowerButtonDc: super::POWER_ACTION_POLICY,
    pub SleepButtonAc: super::POWER_ACTION_POLICY,
    pub SleepButtonDc: super::POWER_ACTION_POLICY,
    pub LidCloseAc: super::POWER_ACTION_POLICY,
    pub LidCloseDc: super::POWER_ACTION_POLICY,
    pub DischargePolicy: [super::SYSTEM_POWER_LEVEL; 4],
    pub GlobalFlags: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for GLOBAL_USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MACHINE_POWER_POLICY {
    pub Revision: super::ULONG,
    pub MinSleepAc: super::SYSTEM_POWER_STATE,
    pub MinSleepDc: super::SYSTEM_POWER_STATE,
    pub ReducedLatencySleepAc: super::SYSTEM_POWER_STATE,
    pub ReducedLatencySleepDc: super::SYSTEM_POWER_STATE,
    pub DozeTimeoutAc: super::ULONG,
    pub DozeTimeoutDc: super::ULONG,
    pub DozeS4TimeoutAc: super::ULONG,
    pub DozeS4TimeoutDc: super::ULONG,
    pub MinThrottleAc: super::UCHAR,
    pub MinThrottleDc: super::UCHAR,
    pub pad1: [super::UCHAR; 2],
    pub OverThrottledAc: super::POWER_ACTION_POLICY,
    pub OverThrottledDc: super::POWER_ACTION_POLICY,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MACHINE_PROCESSOR_POWER_POLICY {
    pub Revision: super::ULONG,
    pub ProcessorPolicyAc: super::PROCESSOR_POWER_POLICY,
    pub ProcessorPolicyDc: super::PROCESSOR_POWER_POLICY,
}
#[cfg(feature = "shared")]
pub const NEWSCHEME: super::UINT = 4294967295;
pub const PDCAP_S0_SUPPORTED: i32 = 65536;
pub const PDCAP_S1_SUPPORTED: i32 = 131072;
pub const PDCAP_S2_SUPPORTED: i32 = 262144;
pub const PDCAP_S3_SUPPORTED: i32 = 524288;
pub const PDCAP_S4_SUPPORTED: i32 = 16777216;
pub const PDCAP_S5_SUPPORTED: i32 = 33554432;
pub const PDCAP_WAKE_FROM_S0_SUPPORTED: i32 = 1048576;
pub const PDCAP_WAKE_FROM_S1_SUPPORTED: i32 = 2097152;
pub const PDCAP_WAKE_FROM_S2_SUPPORTED: i32 = 4194304;
pub const PDCAP_WAKE_FROM_S3_SUPPORTED: i32 = 8388608;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDEVICE_NOTIFY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(context: super::PVOID, r#type: super::ULONG, setting: super::PVOID) -> super::ULONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDEVICE_NOTIFY_SUBSCRIBE_PARAMETERS = *mut DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGLOBAL_MACHINE_POWER_POLICY = *mut GLOBAL_MACHINE_POWER_POLICY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGLOBAL_POWER_POLICY = *mut GLOBAL_POWER_POLICY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGLOBAL_USER_POWER_POLICY = *mut GLOBAL_USER_POWER_POLICY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMACHINE_POWER_POLICY = *mut MACHINE_POWER_POLICY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMACHINE_PROCESSOR_POWER_POLICY = *mut MACHINE_PROCESSOR_POWER_POLICY;
pub const POWER_ATTRIBUTE_HIDE: i32 = 1;
pub const POWER_ATTRIBUTE_SHOW_AOAC: i32 = 2;
pub type POWER_DATA_ACCESSOR = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct POWER_POLICY {
    pub user: USER_POWER_POLICY,
    pub mach: MACHINE_POWER_POLICY,
}
pub type PPOWER_DATA_ACCESSOR = *mut POWER_DATA_ACCESSOR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPOWER_POLICY = *mut POWER_POLICY;
#[cfg(feature = "shared")]
pub type PTHERMAL_EVENT = *mut THERMAL_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSER_POWER_POLICY = *mut USER_POWER_POLICY;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWRSCHEMESENUMPROC = Option<unsafe extern "system" fn(index: super::UINT, namesize: super::DWORD, name: windows_sys::core::PCWSTR, descriptionsize: super::DWORD, description: windows_sys::core::PCWSTR, policy: PPOWER_POLICY, context: super::LPARAM) -> super::BOOLEAN>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWRSCHEMESENUMPROC_V1 = Option<unsafe extern "system" fn(index: super::UINT, namesize: super::DWORD, name: super::LPTSTR, descriptionsize: super::DWORD, description: super::LPTSTR, policy: PPOWER_POLICY, context: super::LPARAM) -> super::BOOLEAN>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWRSCHEMESENUMPROC_V2 = Option<unsafe extern "system" fn(index: super::UINT, namesize: super::DWORD, name: windows_sys::core::PCWSTR, descriptionsize: super::DWORD, description: windows_sys::core::PCWSTR, policy: PPOWER_POLICY, context: super::LPARAM) -> super::BOOLEAN>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct THERMAL_EVENT {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub Type: super::ULONG,
    pub Temperature: super::ULONG,
    pub TripPointTemperature: super::ULONG,
    pub Initiator: windows_sys::core::PWSTR,
}
pub const THERMAL_EVENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USER_POWER_POLICY {
    pub Revision: super::ULONG,
    pub IdleAc: super::POWER_ACTION_POLICY,
    pub IdleDc: super::POWER_ACTION_POLICY,
    pub IdleTimeoutAc: super::ULONG,
    pub IdleTimeoutDc: super::ULONG,
    pub IdleSensitivityAc: super::UCHAR,
    pub IdleSensitivityDc: super::UCHAR,
    pub ThrottlePolicyAc: super::UCHAR,
    pub ThrottlePolicyDc: super::UCHAR,
    pub MaxSleepAc: super::SYSTEM_POWER_STATE,
    pub MaxSleepDc: super::SYSTEM_POWER_STATE,
    pub Reserved: [super::ULONG; 2],
    pub VideoTimeoutAc: super::ULONG,
    pub VideoTimeoutDc: super::ULONG,
    pub SpindownTimeoutAc: super::ULONG,
    pub SpindownTimeoutDc: super::ULONG,
    pub OptimizeForPowerAc: super::BOOLEAN,
    pub OptimizeForPowerDc: super::BOOLEAN,
    pub FanThrottleToleranceAc: super::UCHAR,
    pub FanThrottleToleranceDc: super::UCHAR,
    pub ForcedThrottleAc: super::UCHAR,
    pub ForcedThrottleDc: super::UCHAR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
