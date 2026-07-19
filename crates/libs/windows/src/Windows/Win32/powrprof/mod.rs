#[inline]
pub unsafe fn CanUserWritePwrScheme() -> bool {
    windows_core::link!("powrprof.dll" "system" fn CanUserWritePwrScheme() -> bool);
    unsafe { CanUserWritePwrScheme() }
}
#[inline]
pub unsafe fn DeletePwrScheme(uiid: u32) -> bool {
    windows_core::link!("powrprof.dll" "system" fn DeletePwrScheme(uiid : u32) -> bool);
    unsafe { DeletePwrScheme(uiid) }
}
#[inline]
pub unsafe fn DevicePowerClose() -> bool {
    windows_core::link!("powrprof.dll" "system" fn DevicePowerClose() -> bool);
    unsafe { DevicePowerClose() }
}
#[inline]
pub unsafe fn DevicePowerEnumDevices(queryindex: u32, queryinterpretationflags: u32, queryflags: u32, preturnbuffer: Option<*mut u8>, pbuffersize: *mut u32) -> bool {
    windows_core::link!("powrprof.dll" "system" fn DevicePowerEnumDevices(queryindex : u32, queryinterpretationflags : u32, queryflags : u32, preturnbuffer : *mut u8, pbuffersize : *mut u32) -> bool);
    unsafe { DevicePowerEnumDevices(queryindex, queryinterpretationflags, queryflags, preturnbuffer.unwrap_or(core::mem::zeroed()) as _, pbuffersize as _) }
}
#[inline]
pub unsafe fn DevicePowerOpen(debugmask: Option<u32>) -> bool {
    windows_core::link!("powrprof.dll" "system" fn DevicePowerOpen(debugmask : u32) -> bool);
    unsafe { DevicePowerOpen(debugmask.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DevicePowerSetDeviceState<P0>(devicedescription: P0, setflags: u32, setdata: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("powrprof.dll" "system" fn DevicePowerSetDeviceState(devicedescription : windows_core::PCWSTR, setflags : u32, setdata : *const core::ffi::c_void) -> u32);
    unsafe { DevicePowerSetDeviceState(devicedescription.param().abi(), setflags, setdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumPwrSchemes(lpfn: PWRSCHEMESENUMPROC, lparam: super::LPARAM) -> bool {
    windows_core::link!("powrprof.dll" "system" fn EnumPwrSchemes(lpfn : PWRSCHEMESENUMPROC, lparam : super::LPARAM) -> bool);
    unsafe { EnumPwrSchemes(lpfn, lparam) }
}
#[inline]
pub unsafe fn GetActivePwrScheme(puiid: *mut u32) -> bool {
    windows_core::link!("powrprof.dll" "system" fn GetActivePwrScheme(puiid : *mut u32) -> bool);
    unsafe { GetActivePwrScheme(puiid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetCurrentPowerPolicies(pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY, ppowerpolicy: *mut POWER_POLICY) -> bool {
    windows_core::link!("powrprof.dll" "system" fn GetCurrentPowerPolicies(pglobalpowerpolicy : *mut GLOBAL_POWER_POLICY, ppowerpolicy : *mut POWER_POLICY) -> bool);
    unsafe { GetCurrentPowerPolicies(pglobalpowerpolicy as _, ppowerpolicy as _) }
}
#[inline]
pub unsafe fn GetPwrDiskSpindownRange(puimax: *mut u32, puimin: *mut u32) -> bool {
    windows_core::link!("powrprof.dll" "system" fn GetPwrDiskSpindownRange(puimax : *mut u32, puimin : *mut u32) -> bool);
    unsafe { GetPwrDiskSpindownRange(puimax as _, puimin as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsAdminOverrideActive(papp: *const super::ADMINISTRATOR_POWER_POLICY) -> bool {
    windows_core::link!("powrprof.dll" "system" fn IsAdminOverrideActive(papp : *const super::ADMINISTRATOR_POWER_POLICY) -> bool);
    unsafe { IsAdminOverrideActive(papp) }
}
#[inline]
pub unsafe fn IsPwrHibernateAllowed() -> bool {
    windows_core::link!("powrprof.dll" "system" fn IsPwrHibernateAllowed() -> bool);
    unsafe { IsPwrHibernateAllowed() }
}
#[inline]
pub unsafe fn IsPwrShutdownAllowed() -> bool {
    windows_core::link!("powrprof.dll" "system" fn IsPwrShutdownAllowed() -> bool);
    unsafe { IsPwrShutdownAllowed() }
}
#[inline]
pub unsafe fn IsPwrSuspendAllowed() -> bool {
    windows_core::link!("powrprof.dll" "system" fn IsPwrSuspendAllowed() -> bool);
    unsafe { IsPwrSuspendAllowed() }
}
#[inline]
pub unsafe fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid: *const windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid : *const windows_core::GUID) -> u32);
    unsafe { PowerCanRestoreIndividualDefaultPowerScheme(schemeguid) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerCreatePossibleSetting(rootsystempowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: *const windows_core::GUID, powersettingguid: *const windows_core::GUID, possiblesettingindex: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerCreatePossibleSetting(rootsystempowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32) -> u32);
    unsafe { PowerCreatePossibleSetting(rootsystempowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid, powersettingguid, possiblesettingindex) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerCreateSetting(rootsystempowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: *const windows_core::GUID, powersettingguid: *const windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerCreateSetting(rootsystempowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID) -> u32);
    unsafe { PowerCreateSetting(rootsystempowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid, powersettingguid) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerDeleteScheme(rootpowerkey: Option<super::HKEY>, schemeguid: *const windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerDeleteScheme(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID) -> u32);
    unsafe { PowerDeleteScheme(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PowerDeterminePlatformRole() -> super::POWER_PLATFORM_ROLE {
    windows_core::link!("powrprof.dll" "system" fn PowerDeterminePlatformRole() -> super::POWER_PLATFORM_ROLE);
    unsafe { PowerDeterminePlatformRole() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerDuplicateScheme(rootpowerkey: Option<super::HKEY>, sourceschemeguid: *const windows_core::GUID, destinationschemeguid: *mut *mut windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerDuplicateScheme(rootpowerkey : super::HKEY, sourceschemeguid : *const windows_core::GUID, destinationschemeguid : *mut *mut windows_core::GUID) -> u32);
    unsafe { PowerDuplicateScheme(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, sourceschemeguid, destinationschemeguid as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerEnumerate(rootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, accessflags: POWER_DATA_ACCESSOR, index: u32, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerEnumerate(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, accessflags : POWER_DATA_ACCESSOR, index : u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerEnumerate(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, accessflags, index, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn PowerGetUserConfiguredACPowerMode(powermodeguid: *mut windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerGetUserConfiguredACPowerMode(powermodeguid : *mut windows_core::GUID) -> u32);
    unsafe { PowerGetUserConfiguredACPowerMode(powermodeguid as _) }
}
#[inline]
pub unsafe fn PowerGetUserConfiguredDCPowerMode(powermodeguid: *mut windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerGetUserConfiguredDCPowerMode(powermodeguid : *mut windows_core::GUID) -> u32);
    unsafe { PowerGetUserConfiguredDCPowerMode(powermodeguid as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerImportPowerScheme<P1>(rootpowerkey: Option<super::HKEY>, importfilenamepath: P1, destinationschemeguid: *mut *mut windows_core::GUID) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("powrprof.dll" "system" fn PowerImportPowerScheme(rootpowerkey : super::HKEY, importfilenamepath : windows_core::PCWSTR, destinationschemeguid : *mut *mut windows_core::GUID) -> u32);
    unsafe { PowerImportPowerScheme(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, importfilenamepath.param().abi(), destinationschemeguid as _) }
}
#[inline]
pub unsafe fn PowerIsSettingRangeDefined(subkeyguid: Option<*const windows_core::GUID>, settingguid: Option<*const windows_core::GUID>) -> bool {
    windows_core::link!("powrprof.dll" "system" fn PowerIsSettingRangeDefined(subkeyguid : *const windows_core::GUID, settingguid : *const windows_core::GUID) -> bool);
    unsafe { PowerIsSettingRangeDefined(subkeyguid.unwrap_or(core::mem::zeroed()) as _, settingguid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn PowerOpenSystemPowerKey(phsystempowerkey: *mut super::HKEY, access: super::REGSAM, openexisting: bool) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerOpenSystemPowerKey(phsystempowerkey : *mut super::HKEY, access : super::REGSAM, openexisting : windows_core::BOOL) -> u32);
    unsafe { PowerOpenSystemPowerKey(phsystempowerkey as _, access, openexisting.into()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn PowerOpenUserPowerKey(phuserpowerkey: *mut super::HKEY, access: super::REGSAM, openexisting: bool) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerOpenUserPowerKey(phuserpowerkey : *mut super::HKEY, access : super::REGSAM, openexisting : windows_core::BOOL) -> u32);
    unsafe { PowerOpenUserPowerKey(phuserpowerkey as _, access, openexisting.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadACDefaultIndex(rootpowerkey: Option<super::HKEY>, schemepersonalityguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: *const windows_core::GUID, acdefaultindex: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadACDefaultIndex(rootpowerkey : super::HKEY, schemepersonalityguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, acdefaultindex : *mut u32) -> u32);
    unsafe { PowerReadACDefaultIndex(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemepersonalityguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid, acdefaultindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadACValueIndex(rootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, acvalueindex: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadACValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, acvalueindex : *mut u32) -> u32);
    unsafe { PowerReadACValueIndex(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, acvalueindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadDCDefaultIndex(rootpowerkey: Option<super::HKEY>, schemepersonalityguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: *const windows_core::GUID, dcdefaultindex: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadDCDefaultIndex(rootpowerkey : super::HKEY, schemepersonalityguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, dcdefaultindex : *mut u32) -> u32);
    unsafe { PowerReadDCDefaultIndex(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemepersonalityguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid, dcdefaultindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadDCValueIndex(rootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, dcvalueindex: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadDCValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, dcvalueindex : *mut u32) -> u32);
    unsafe { PowerReadDCValueIndex(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, dcvalueindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadDescription(rootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadDescription(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadDescription(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadFriendlyName(rootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadFriendlyName(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadFriendlyName(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadIconResourceSpecifier(rootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadIconResourceSpecifier(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadIconResourceSpecifier(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadPossibleDescription(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, possiblesettingindex: u32, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadPossibleDescription(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadPossibleDescription(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, possiblesettingindex, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadPossibleFriendlyName(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, possiblesettingindex: u32, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadPossibleFriendlyName(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadPossibleFriendlyName(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, possiblesettingindex, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadPossibleValue(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: Option<*mut u32>, possiblesettingindex: u32, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadPossibleValue(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : *mut u32, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadPossibleValue(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, r#type.unwrap_or(core::mem::zeroed()) as _, possiblesettingindex, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn PowerReadSettingAttributes(subgroupguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadSettingAttributes(subgroupguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID) -> u32);
    unsafe { PowerReadSettingAttributes(subgroupguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadValueIncrement(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valueincrement: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadValueIncrement(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valueincrement : *mut u32) -> u32);
    unsafe { PowerReadValueIncrement(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, valueincrement as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadValueMax(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valuemaximum: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadValueMax(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valuemaximum : *mut u32) -> u32);
    unsafe { PowerReadValueMax(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, valuemaximum as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadValueMin(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valueminimum: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadValueMin(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valueminimum : *mut u32) -> u32);
    unsafe { PowerReadValueMin(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, valueminimum as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerReadValueUnitsSpecifier(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadValueUnitsSpecifier(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadValueUnitsSpecifier(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn PowerRemovePowerSetting(powersettingsubkeyguid: *const windows_core::GUID, powersettingguid: *const windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerRemovePowerSetting(powersettingsubkeyguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID) -> u32);
    unsafe { PowerRemovePowerSetting(powersettingsubkeyguid, powersettingguid) }
}
#[inline]
pub unsafe fn PowerReplaceDefaultPowerSchemes() -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReplaceDefaultPowerSchemes() -> u32);
    unsafe { PowerReplaceDefaultPowerSchemes() }
}
#[inline]
pub unsafe fn PowerReportThermalEvent(event: *const THERMAL_EVENT) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReportThermalEvent(event : *const THERMAL_EVENT) -> u32);
    unsafe { PowerReportThermalEvent(event) }
}
#[inline]
pub unsafe fn PowerRestoreDefaultPowerSchemes() -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerRestoreDefaultPowerSchemes() -> u32);
    unsafe { PowerRestoreDefaultPowerSchemes() }
}
#[inline]
pub unsafe fn PowerRestoreIndividualDefaultPowerScheme(schemeguid: *const windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerRestoreIndividualDefaultPowerScheme(schemeguid : *const windows_core::GUID) -> u32);
    unsafe { PowerRestoreIndividualDefaultPowerScheme(schemeguid) }
}
#[inline]
pub unsafe fn PowerSetUserConfiguredACPowerMode(powermodeguid: *const windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerSetUserConfiguredACPowerMode(powermodeguid : *const windows_core::GUID) -> u32);
    unsafe { PowerSetUserConfiguredACPowerMode(powermodeguid) }
}
#[inline]
pub unsafe fn PowerSetUserConfiguredDCPowerMode(powermodeguid: *const windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerSetUserConfiguredDCPowerMode(powermodeguid : *const windows_core::GUID) -> u32);
    unsafe { PowerSetUserConfiguredDCPowerMode(powermodeguid) }
}
#[inline]
pub unsafe fn PowerSettingAccessCheck(accessflags: POWER_DATA_ACCESSOR, powerguid: Option<*const windows_core::GUID>) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerSettingAccessCheck(accessflags : POWER_DATA_ACCESSOR, powerguid : *const windows_core::GUID) -> u32);
    unsafe { PowerSettingAccessCheck(accessflags, powerguid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn PowerSettingAccessCheckEx(accessflags: POWER_DATA_ACCESSOR, powerguid: Option<*const windows_core::GUID>, accesstype: super::REGSAM) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerSettingAccessCheckEx(accessflags : POWER_DATA_ACCESSOR, powerguid : *const windows_core::GUID, accesstype : super::REGSAM) -> u32);
    unsafe { PowerSettingAccessCheckEx(accessflags, powerguid.unwrap_or(core::mem::zeroed()) as _, accesstype) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteACDefaultIndex(rootsystempowerkey: Option<super::HKEY>, schemepersonalityguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: *const windows_core::GUID, defaultacindex: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteACDefaultIndex(rootsystempowerkey : super::HKEY, schemepersonalityguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, defaultacindex : u32) -> u32);
    unsafe { PowerWriteACDefaultIndex(rootsystempowerkey.unwrap_or(core::mem::zeroed()) as _, schemepersonalityguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid, defaultacindex) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteDCDefaultIndex(rootsystempowerkey: Option<super::HKEY>, schemepersonalityguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: *const windows_core::GUID, defaultdcindex: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteDCDefaultIndex(rootsystempowerkey : super::HKEY, schemepersonalityguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, defaultdcindex : u32) -> u32);
    unsafe { PowerWriteDCDefaultIndex(rootsystempowerkey.unwrap_or(core::mem::zeroed()) as _, schemepersonalityguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid, defaultdcindex) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteDescription(rootpowerkey: Option<super::HKEY>, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: &[u8]) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteDescription(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *const u8, buffersize : u32) -> u32);
    unsafe { PowerWriteDescription(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, buffer.as_ptr(), buffer.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteFriendlyName(rootpowerkey: Option<super::HKEY>, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: &[u8]) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteFriendlyName(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *const u8, buffersize : u32) -> u32);
    unsafe { PowerWriteFriendlyName(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, buffer.as_ptr(), buffer.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteIconResourceSpecifier(rootpowerkey: Option<super::HKEY>, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: &[u8]) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteIconResourceSpecifier(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *const u8, buffersize : u32) -> u32);
    unsafe { PowerWriteIconResourceSpecifier(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, buffer.as_ptr(), buffer.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWritePossibleDescription(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, possiblesettingindex: u32, buffer: &[u8]) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWritePossibleDescription(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> u32);
    unsafe { PowerWritePossibleDescription(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, possiblesettingindex, buffer.as_ptr(), buffer.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWritePossibleFriendlyName(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, possiblesettingindex: u32, buffer: &[u8]) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWritePossibleFriendlyName(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> u32);
    unsafe { PowerWritePossibleFriendlyName(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, possiblesettingindex, buffer.as_ptr(), buffer.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWritePossibleValue(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: u32, possiblesettingindex: u32, buffer: &[u8]) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWritePossibleValue(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : u32, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> u32);
    unsafe { PowerWritePossibleValue(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, r#type, possiblesettingindex, buffer.as_ptr(), buffer.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PowerWriteSettingAttributes(subgroupguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, attributes: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteSettingAttributes(subgroupguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, attributes : u32) -> u32);
    unsafe { PowerWriteSettingAttributes(subgroupguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, attributes) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteValueIncrement(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valueincrement: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteValueIncrement(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valueincrement : u32) -> u32);
    unsafe { PowerWriteValueIncrement(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, valueincrement) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteValueMax(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valuemaximum: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteValueMax(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valuemaximum : u32) -> u32);
    unsafe { PowerWriteValueMax(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, valuemaximum) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteValueMin(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valueminimum: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteValueMin(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valueminimum : u32) -> u32);
    unsafe { PowerWriteValueMin(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, valueminimum) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PowerWriteValueUnitsSpecifier(rootpowerkey: Option<super::HKEY>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: &[u8]) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteValueUnitsSpecifier(rootpowerkey : super::HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *const u8, buffersize : u32) -> u32);
    unsafe { PowerWriteValueUnitsSpecifier(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, buffer.as_ptr(), buffer.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReadGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> bool {
    windows_core::link!("powrprof.dll" "system" fn ReadGlobalPwrPolicy(pglobalpowerpolicy : *const GLOBAL_POWER_POLICY) -> bool);
    unsafe { ReadGlobalPwrPolicy(pglobalpowerpolicy) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReadProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *mut MACHINE_PROCESSOR_POWER_POLICY) -> bool {
    windows_core::link!("powrprof.dll" "system" fn ReadProcessorPwrScheme(uiid : u32, pmachineprocessorpowerpolicy : *mut MACHINE_PROCESSOR_POWER_POLICY) -> bool);
    unsafe { ReadProcessorPwrScheme(uiid, pmachineprocessorpowerpolicy as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReadPwrScheme(uiid: u32, ppowerpolicy: *mut POWER_POLICY) -> bool {
    windows_core::link!("powrprof.dll" "system" fn ReadPwrScheme(uiid : u32, ppowerpolicy : *mut POWER_POLICY) -> bool);
    unsafe { ReadPwrScheme(uiid, ppowerpolicy as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetActivePwrScheme(uiid: u32, pglobalpowerpolicy: Option<*const GLOBAL_POWER_POLICY>, ppowerpolicy: Option<*const POWER_POLICY>) -> bool {
    windows_core::link!("powrprof.dll" "system" fn SetActivePwrScheme(uiid : u32, pglobalpowerpolicy : *const GLOBAL_POWER_POLICY, ppowerpolicy : *const POWER_POLICY) -> bool);
    unsafe { SetActivePwrScheme(uiid, pglobalpowerpolicy.unwrap_or(core::mem::zeroed()) as _, ppowerpolicy.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetSuspendState(bhibernate: bool, bforce: bool, bwakeupeventsdisabled: bool) -> bool {
    windows_core::link!("powrprof.dll" "system" fn SetSuspendState(bhibernate : bool, bforce : bool, bwakeupeventsdisabled : bool) -> bool);
    unsafe { SetSuspendState(bhibernate, bforce, bwakeupeventsdisabled) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ValidatePowerPolicies(pglobalpowerpolicy: Option<*mut GLOBAL_POWER_POLICY>, ppowerpolicy: Option<*mut POWER_POLICY>) -> bool {
    windows_core::link!("powrprof.dll" "system" fn ValidatePowerPolicies(pglobalpowerpolicy : *mut GLOBAL_POWER_POLICY, ppowerpolicy : *mut POWER_POLICY) -> bool);
    unsafe { ValidatePowerPolicies(pglobalpowerpolicy.unwrap_or(core::mem::zeroed()) as _, ppowerpolicy.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WriteGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> bool {
    windows_core::link!("powrprof.dll" "system" fn WriteGlobalPwrPolicy(pglobalpowerpolicy : *const GLOBAL_POWER_POLICY) -> bool);
    unsafe { WriteGlobalPwrPolicy(pglobalpowerpolicy) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WriteProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *const MACHINE_PROCESSOR_POWER_POLICY) -> bool {
    windows_core::link!("powrprof.dll" "system" fn WriteProcessorPwrScheme(uiid : u32, pmachineprocessorpowerpolicy : *const MACHINE_PROCESSOR_POWER_POLICY) -> bool);
    unsafe { WriteProcessorPwrScheme(uiid, pmachineprocessorpowerpolicy) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WritePwrScheme<P1, P2>(puiid: *const u32, lpszschemename: P1, lpszdescription: P2, lpscheme: *const POWER_POLICY) -> bool
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("powrprof.dll" "system" fn WritePwrScheme(puiid : *const u32, lpszschemename : windows_core::PCWSTR, lpszdescription : windows_core::PCWSTR, lpscheme : *const POWER_POLICY) -> bool);
    unsafe { WritePwrScheme(puiid, lpszschemename.param().abi(), lpszdescription.param().abi(), lpscheme) }
}
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
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBAL_MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub LidOpenWakeAc: super::SYSTEM_POWER_STATE,
    pub LidOpenWakeDc: super::SYSTEM_POWER_STATE,
    pub BroadcastCapacityResolution: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBAL_POWER_POLICY {
    pub user: GLOBAL_USER_POWER_POLICY,
    pub mach: GLOBAL_MACHINE_POWER_POLICY,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GLOBAL_USER_POWER_POLICY {
    pub Revision: u32,
    pub PowerButtonAc: super::POWER_ACTION_POLICY,
    pub PowerButtonDc: super::POWER_ACTION_POLICY,
    pub SleepButtonAc: super::POWER_ACTION_POLICY,
    pub SleepButtonDc: super::POWER_ACTION_POLICY,
    pub LidCloseAc: super::POWER_ACTION_POLICY,
    pub LidCloseDc: super::POWER_ACTION_POLICY,
    pub DischargePolicy: [super::SYSTEM_POWER_LEVEL; 4],
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub MinSleepAc: super::SYSTEM_POWER_STATE,
    pub MinSleepDc: super::SYSTEM_POWER_STATE,
    pub ReducedLatencySleepAc: super::SYSTEM_POWER_STATE,
    pub ReducedLatencySleepDc: super::SYSTEM_POWER_STATE,
    pub DozeTimeoutAc: u32,
    pub DozeTimeoutDc: u32,
    pub DozeS4TimeoutAc: u32,
    pub DozeS4TimeoutDc: u32,
    pub MinThrottleAc: u8,
    pub MinThrottleDc: u8,
    pub pad1: [u8; 2],
    pub OverThrottledAc: super::POWER_ACTION_POLICY,
    pub OverThrottledDc: super::POWER_ACTION_POLICY,
}
#[cfg(feature = "winnt")]
impl Default for MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MACHINE_PROCESSOR_POWER_POLICY {
    pub Revision: u32,
    pub ProcessorPolicyAc: super::PROCESSOR_POWER_POLICY,
    pub ProcessorPolicyDc: super::PROCESSOR_POWER_POLICY,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub type PWRSCHEMESENUMPROC = Option<unsafe extern "system" fn(index: u32, namesize: u32, name: windows_core::PCWSTR, descriptionsize: u32, description: windows_core::PCWSTR, policy: *const POWER_POLICY, context: super::LPARAM) -> bool>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PWRSCHEMESENUMPROC_V1 = Option<unsafe extern "system" fn(index: u32, namesize: u32, name: super::LPTSTR, descriptionsize: u32, description: super::LPTSTR, policy: *const POWER_POLICY, context: super::LPARAM) -> bool>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PWRSCHEMESENUMPROC_V2 = Option<unsafe extern "system" fn(index: u32, namesize: u32, name: windows_core::PCWSTR, descriptionsize: u32, description: windows_core::PCWSTR, policy: *const POWER_POLICY, context: super::LPARAM) -> bool>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THERMAL_EVENT {
    pub Version: u32,
    pub Size: u32,
    pub Type: u32,
    pub Temperature: u32,
    pub TripPointTemperature: u32,
    pub Initiator: windows_core::PWSTR,
}
pub const THERMAL_EVENT_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_POWER_POLICY {
    pub Revision: u32,
    pub IdleAc: super::POWER_ACTION_POLICY,
    pub IdleDc: super::POWER_ACTION_POLICY,
    pub IdleTimeoutAc: u32,
    pub IdleTimeoutDc: u32,
    pub IdleSensitivityAc: u8,
    pub IdleSensitivityDc: u8,
    pub ThrottlePolicyAc: u8,
    pub ThrottlePolicyDc: u8,
    pub MaxSleepAc: super::SYSTEM_POWER_STATE,
    pub MaxSleepDc: super::SYSTEM_POWER_STATE,
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
