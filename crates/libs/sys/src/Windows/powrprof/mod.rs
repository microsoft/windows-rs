windows_link::link!("powrprof.dll" "system" fn CanUserWritePwrScheme() -> bool);
windows_link::link!("powrprof.dll" "system" fn DeletePwrScheme(uiid : u32) -> bool);
windows_link::link!("powrprof.dll" "system" fn DevicePowerClose() -> bool);
windows_link::link!("powrprof.dll" "system" fn DevicePowerEnumDevices(queryindex : u32, queryinterpretationflags : u32, queryflags : u32, preturnbuffer : *mut u8, pbuffersize : *mut u32) -> bool);
windows_link::link!("powrprof.dll" "system" fn DevicePowerOpen(debugmask : u32) -> bool);
windows_link::link!("powrprof.dll" "system" fn DevicePowerSetDeviceState(devicedescription : windows_sys::core::PCWSTR, setflags : u32, setdata : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn EnumPwrSchemes(lpfn : PWRSCHEMESENUMPROC, lparam : super::minwindef::LPARAM) -> bool);
windows_link::link!("powrprof.dll" "system" fn GetActivePwrScheme(puiid : *mut u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn GetCurrentPowerPolicies(pglobalpowerpolicy : *mut GLOBAL_POWER_POLICY, ppowerpolicy : *mut POWER_POLICY) -> bool);
windows_link::link!("powrprof.dll" "system" fn GetPwrDiskSpindownRange(puimax : *mut u32, puimin : *mut u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn IsAdminOverrideActive(papp : *const super::winnt::ADMINISTRATOR_POWER_POLICY) -> bool);
windows_link::link!("powrprof.dll" "system" fn IsPwrHibernateAllowed() -> bool);
windows_link::link!("powrprof.dll" "system" fn IsPwrShutdownAllowed() -> bool);
windows_link::link!("powrprof.dll" "system" fn IsPwrSuspendAllowed() -> bool);
windows_link::link!("powrprof.dll" "system" fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerCreatePossibleSetting(rootsystempowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerCreateSetting(rootsystempowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerDeleteScheme(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn PowerDeterminePlatformRole() -> super::winnt::POWER_PLATFORM_ROLE);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerDuplicateScheme(rootpowerkey : super::minwindef::HKEY, sourceschemeguid : *const windows_sys::core::GUID, destinationschemeguid : *mut *mut windows_sys::core::GUID) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerEnumerate(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, accessflags : POWER_DATA_ACCESSOR, index : u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerGetUserConfiguredACPowerMode(powermodeguid : *mut windows_sys::core::GUID) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerGetUserConfiguredDCPowerMode(powermodeguid : *mut windows_sys::core::GUID) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerImportPowerScheme(rootpowerkey : super::minwindef::HKEY, importfilenamepath : windows_sys::core::PCWSTR, destinationschemeguid : *mut *mut windows_sys::core::GUID) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerIsSettingRangeDefined(subkeyguid : *const windows_sys::core::GUID, settingguid : *const windows_sys::core::GUID) -> bool);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("powrprof.dll" "system" fn PowerOpenSystemPowerKey(phsystempowerkey : *mut super::minwindef::HKEY, access : super::winreg::REGSAM, openexisting : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("powrprof.dll" "system" fn PowerOpenUserPowerKey(phuserpowerkey : *mut super::minwindef::HKEY, access : super::winreg::REGSAM, openexisting : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadACDefaultIndex(rootpowerkey : super::minwindef::HKEY, schemepersonalityguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, acdefaultindex : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadACValueIndex(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, acvalueindex : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadDCDefaultIndex(rootpowerkey : super::minwindef::HKEY, schemepersonalityguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, dcdefaultindex : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadDCValueIndex(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, dcvalueindex : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadDescription(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *mut u8, buffersize : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadFriendlyName(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *mut u8, buffersize : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadIconResourceSpecifier(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *mut u8, buffersize : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadPossibleDescription(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadPossibleFriendlyName(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadPossibleValue(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, r#type : *mut u32, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerReadSettingAttributes(subgroupguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadValueIncrement(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valueincrement : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadValueMax(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valuemaximum : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadValueMin(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valueminimum : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadValueUnitsSpecifier(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *mut u8, buffersize : *mut u32) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerRemovePowerSetting(powersettingsubkeyguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerReplaceDefaultPowerSchemes() -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerReportThermalEvent(event : *const THERMAL_EVENT) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerRestoreDefaultPowerSchemes() -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerRestoreIndividualDefaultPowerScheme(schemeguid : *const windows_sys::core::GUID) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerSetUserConfiguredACPowerMode(powermodeguid : *const windows_sys::core::GUID) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerSetUserConfiguredDCPowerMode(powermodeguid : *const windows_sys::core::GUID) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerSettingAccessCheck(accessflags : POWER_DATA_ACCESSOR, powerguid : *const windows_sys::core::GUID) -> u32);
#[cfg(all(feature = "winnt", feature = "winreg"))]
windows_link::link!("powrprof.dll" "system" fn PowerSettingAccessCheckEx(accessflags : POWER_DATA_ACCESSOR, powerguid : *const windows_sys::core::GUID, accesstype : super::winreg::REGSAM) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteACDefaultIndex(rootsystempowerkey : super::minwindef::HKEY, schemepersonalityguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, defaultacindex : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteDCDefaultIndex(rootsystempowerkey : super::minwindef::HKEY, schemepersonalityguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, defaultdcindex : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteDescription(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *const u8, buffersize : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteFriendlyName(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *const u8, buffersize : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteIconResourceSpecifier(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *const u8, buffersize : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWritePossibleDescription(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWritePossibleFriendlyName(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWritePossibleValue(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, r#type : u32, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerWriteSettingAttributes(subgroupguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, attributes : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteValueIncrement(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valueincrement : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteValueMax(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valuemaximum : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteValueMin(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, valueminimum : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteValueUnitsSpecifier(rootpowerkey : super::minwindef::HKEY, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, buffer : *const u8, buffersize : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn ReadGlobalPwrPolicy(pglobalpowerpolicy : *const GLOBAL_POWER_POLICY) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn ReadProcessorPwrScheme(uiid : u32, pmachineprocessorpowerpolicy : *mut MACHINE_PROCESSOR_POWER_POLICY) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn ReadPwrScheme(uiid : u32, ppowerpolicy : *mut POWER_POLICY) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn SetActivePwrScheme(uiid : u32, pglobalpowerpolicy : *const GLOBAL_POWER_POLICY, ppowerpolicy : *const POWER_POLICY) -> bool);
windows_link::link!("powrprof.dll" "system" fn SetSuspendState(bhibernate : bool, bforce : bool, bwakeupeventsdisabled : bool) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn ValidatePowerPolicies(pglobalpowerpolicy : *mut GLOBAL_POWER_POLICY, ppowerpolicy : *mut POWER_POLICY) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn WriteGlobalPwrPolicy(pglobalpowerpolicy : *const GLOBAL_POWER_POLICY) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn WriteProcessorPwrScheme(uiid : u32, pmachineprocessorpowerpolicy : *const MACHINE_PROCESSOR_POWER_POLICY) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn WritePwrScheme(puiid : *const u32, lpszschemename : windows_sys::core::PCWSTR, lpszdescription : windows_sys::core::PCWSTR, lpscheme : *const POWER_POLICY) -> bool);
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
pub const DEVICEPOWER_AND_OPERATION: u32 = 1073741824;
pub const DEVICEPOWER_CLEAR_WAKEENABLED: u32 = 2;
pub const DEVICEPOWER_FILTER_DEVICES_PRESENT: u32 = 536870912;
pub const DEVICEPOWER_FILTER_HARDWARE: u32 = 268435456;
pub const DEVICEPOWER_FILTER_ON_NAME: u32 = 33554432;
pub const DEVICEPOWER_FILTER_WAKEENABLED: u32 = 134217728;
pub const DEVICEPOWER_FILTER_WAKEPROGRAMMABLE: u32 = 67108864;
pub const DEVICEPOWER_HARDWAREID: u32 = 2147483648;
pub const DEVICEPOWER_SET_WAKEENABLED: u32 = 1;
pub const DEVICE_NOTIFY_CALLBACK: u32 = 2;
pub type DEVICE_NOTIFY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, r#type: u32, setting: *const core::ffi::c_void) -> u32>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    pub Callback: PDEVICE_NOTIFY_CALLBACK_ROUTINE,
    pub Context: *mut core::ffi::c_void,
}
impl Default for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EnableMultiBatteryDisplay: u32 = 2;
pub const EnablePasswordLogon: u32 = 4;
pub const EnableSysTrayBatteryMeter: u32 = 1;
pub const EnableVideoDimDisplay: u32 = 16;
pub const EnableWakeOnRing: u32 = 8;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct GLOBAL_MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub LidOpenWakeAc: super::winnt::SYSTEM_POWER_STATE,
    pub LidOpenWakeDc: super::winnt::SYSTEM_POWER_STATE,
    pub BroadcastCapacityResolution: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct GLOBAL_POWER_POLICY {
    pub user: GLOBAL_USER_POWER_POLICY,
    pub mach: GLOBAL_MACHINE_POWER_POLICY,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GLOBAL_USER_POWER_POLICY {
    pub Revision: u32,
    pub PowerButtonAc: super::winnt::POWER_ACTION_POLICY,
    pub PowerButtonDc: super::winnt::POWER_ACTION_POLICY,
    pub SleepButtonAc: super::winnt::POWER_ACTION_POLICY,
    pub SleepButtonDc: super::winnt::POWER_ACTION_POLICY,
    pub LidCloseAc: super::winnt::POWER_ACTION_POLICY,
    pub LidCloseDc: super::winnt::POWER_ACTION_POLICY,
    pub DischargePolicy: [super::winnt::SYSTEM_POWER_LEVEL; 4],
    pub GlobalFlags: u32,
}
#[cfg(feature = "winnt")]
impl Default for GLOBAL_USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub MinSleepAc: super::winnt::SYSTEM_POWER_STATE,
    pub MinSleepDc: super::winnt::SYSTEM_POWER_STATE,
    pub ReducedLatencySleepAc: super::winnt::SYSTEM_POWER_STATE,
    pub ReducedLatencySleepDc: super::winnt::SYSTEM_POWER_STATE,
    pub DozeTimeoutAc: u32,
    pub DozeTimeoutDc: u32,
    pub DozeS4TimeoutAc: u32,
    pub DozeS4TimeoutDc: u32,
    pub MinThrottleAc: u8,
    pub MinThrottleDc: u8,
    pub pad1: [u8; 2],
    pub OverThrottledAc: super::winnt::POWER_ACTION_POLICY,
    pub OverThrottledDc: super::winnt::POWER_ACTION_POLICY,
}
#[cfg(feature = "winnt")]
impl Default for MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct MACHINE_PROCESSOR_POWER_POLICY {
    pub Revision: u32,
    pub ProcessorPolicyAc: super::winnt::PROCESSOR_POWER_POLICY,
    pub ProcessorPolicyDc: super::winnt::PROCESSOR_POWER_POLICY,
}
pub const NEWSCHEME: u32 = 4294967295;
pub const PDCAP_S0_SUPPORTED: u32 = 65536;
pub const PDCAP_S1_SUPPORTED: u32 = 131072;
pub const PDCAP_S2_SUPPORTED: u32 = 262144;
pub const PDCAP_S3_SUPPORTED: u32 = 524288;
pub const PDCAP_S4_SUPPORTED: u32 = 16777216;
pub const PDCAP_S5_SUPPORTED: u32 = 33554432;
pub const PDCAP_WAKE_FROM_S0_SUPPORTED: u32 = 1048576;
pub const PDCAP_WAKE_FROM_S1_SUPPORTED: u32 = 2097152;
pub const PDCAP_WAKE_FROM_S2_SUPPORTED: u32 = 4194304;
pub const PDCAP_WAKE_FROM_S3_SUPPORTED: u32 = 8388608;
pub type PDEVICE_NOTIFY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, r#type: u32, setting: *const core::ffi::c_void) -> u32>;
pub type PDEVICE_NOTIFY_SUBSCRIBE_PARAMETERS = *mut DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS;
#[cfg(feature = "winnt")]
pub type PGLOBAL_MACHINE_POWER_POLICY = *mut GLOBAL_MACHINE_POWER_POLICY;
#[cfg(feature = "winnt")]
pub type PGLOBAL_POWER_POLICY = *mut GLOBAL_POWER_POLICY;
#[cfg(feature = "winnt")]
pub type PGLOBAL_USER_POWER_POLICY = *mut GLOBAL_USER_POWER_POLICY;
#[cfg(feature = "winnt")]
pub type PMACHINE_POWER_POLICY = *mut MACHINE_POWER_POLICY;
#[cfg(feature = "winnt")]
pub type PMACHINE_PROCESSOR_POWER_POLICY = *mut MACHINE_PROCESSOR_POWER_POLICY;
pub const POWER_ATTRIBUTE_HIDE: u32 = 1;
pub const POWER_ATTRIBUTE_SHOW_AOAC: u32 = 2;
pub type POWER_DATA_ACCESSOR = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct POWER_POLICY {
    pub user: USER_POWER_POLICY,
    pub mach: MACHINE_POWER_POLICY,
}
pub type PPOWER_DATA_ACCESSOR = *mut POWER_DATA_ACCESSOR;
#[cfg(feature = "winnt")]
pub type PPOWER_POLICY = *mut POWER_POLICY;
pub type PTHERMAL_EVENT = *mut THERMAL_EVENT;
#[cfg(feature = "winnt")]
pub type PUSER_POWER_POLICY = *mut USER_POWER_POLICY;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PWRSCHEMESENUMPROC = Option<unsafe extern "system" fn(index: u32, namesize: u32, name: windows_sys::core::PCWSTR, descriptionsize: u32, description: windows_sys::core::PCWSTR, policy: *const POWER_POLICY, context: super::minwindef::LPARAM) -> bool>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PWRSCHEMESENUMPROC_V1 = Option<unsafe extern "system" fn(index: u32, namesize: u32, name: super::winnt::LPTSTR, descriptionsize: u32, description: super::winnt::LPTSTR, policy: *const POWER_POLICY, context: super::minwindef::LPARAM) -> bool>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PWRSCHEMESENUMPROC_V2 = Option<unsafe extern "system" fn(index: u32, namesize: u32, name: windows_sys::core::PCWSTR, descriptionsize: u32, description: windows_sys::core::PCWSTR, policy: *const POWER_POLICY, context: super::minwindef::LPARAM) -> bool>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct THERMAL_EVENT {
    pub Version: u32,
    pub Size: u32,
    pub Type: u32,
    pub Temperature: u32,
    pub TripPointTemperature: u32,
    pub Initiator: windows_sys::core::PWSTR,
}
impl Default for THERMAL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const THERMAL_EVENT_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct USER_POWER_POLICY {
    pub Revision: u32,
    pub IdleAc: super::winnt::POWER_ACTION_POLICY,
    pub IdleDc: super::winnt::POWER_ACTION_POLICY,
    pub IdleTimeoutAc: u32,
    pub IdleTimeoutDc: u32,
    pub IdleSensitivityAc: u8,
    pub IdleSensitivityDc: u8,
    pub ThrottlePolicyAc: u8,
    pub ThrottlePolicyDc: u8,
    pub MaxSleepAc: super::winnt::SYSTEM_POWER_STATE,
    pub MaxSleepDc: super::winnt::SYSTEM_POWER_STATE,
    pub Reserved: [u32; 2],
    pub VideoTimeoutAc: u32,
    pub VideoTimeoutDc: u32,
    pub SpindownTimeoutAc: u32,
    pub SpindownTimeoutDc: u32,
    pub OptimizeForPowerAc: bool,
    pub OptimizeForPowerDc: bool,
    pub FanThrottleToleranceAc: u8,
    pub FanThrottleToleranceDc: u8,
    pub ForcedThrottleAc: u8,
    pub ForcedThrottleDc: u8,
}
#[cfg(feature = "winnt")]
impl Default for USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
