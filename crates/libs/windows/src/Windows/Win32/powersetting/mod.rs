#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PowerGetActiveScheme(userrootpowerkey: Option<super::HKEY>, activepolicyguid: *mut *mut windows_core::GUID) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerGetActiveScheme(userrootpowerkey : super::HKEY, activepolicyguid : *mut *mut windows_core::GUID) -> super::DWORD);
    unsafe { PowerGetActiveScheme(userrootpowerkey.unwrap_or(core::mem::zeroed()) as _, activepolicyguid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PowerReadACValue(rootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: Option<super::PULONG>, buffer: Option<super::LPBYTE>, buffersize: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerReadACValue(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : super::PULONG, buffer : super::LPBYTE, buffersize : super::LPDWORD) -> super::DWORD);
    unsafe { PowerReadACValue(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, r#type.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PowerReadDCValue(rootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: Option<super::PULONG>, buffer: Option<super::PUCHAR>, buffersize: super::LPDWORD) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerReadDCValue(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : super::PULONG, buffer : super::PUCHAR, buffersize : super::LPDWORD) -> super::DWORD);
    unsafe { PowerReadDCValue(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, r#type.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PowerRegisterForEffectivePowerModeNotifications(version: super::ULONG, callback: EFFECTIVE_POWER_MODE_CALLBACK, context: Option<super::PVOID>, registrationhandle: *mut super::PVOID) -> windows_core::HRESULT {
    windows_core::link!("powrprof.dll" "system" fn PowerRegisterForEffectivePowerModeNotifications(version : super::ULONG, callback : EFFECTIVE_POWER_MODE_CALLBACK, context : super::PVOID, registrationhandle : *mut super::PVOID) -> windows_core::HRESULT);
    unsafe { PowerRegisterForEffectivePowerModeNotifications(version, callback, context.unwrap_or(core::mem::zeroed()) as _, registrationhandle as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PowerSetActiveScheme(userrootpowerkey: Option<super::HKEY>, schemeguid: Option<*const windows_core::GUID>) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerSetActiveScheme(userrootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID) -> super::DWORD);
    unsafe { PowerSetActiveScheme(userrootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt", feature = "winuser"))]
#[inline]
pub unsafe fn PowerSettingRegisterNotification(settingguid: super::LPCGUID, flags: super::DWORD, recipient: super::HANDLE, registrationhandle: super::PHPOWERNOTIFY) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerSettingRegisterNotification(settingguid : super::LPCGUID, flags : super::DWORD, recipient : super::HANDLE, registrationhandle : super::PHPOWERNOTIFY) -> super::DWORD);
    unsafe { PowerSettingRegisterNotification(settingguid, flags, recipient, registrationhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winuser"))]
#[inline]
pub unsafe fn PowerSettingUnregisterNotification(registrationhandle: super::HPOWERNOTIFY) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerSettingUnregisterNotification(registrationhandle : super::HPOWERNOTIFY) -> super::DWORD);
    unsafe { PowerSettingUnregisterNotification(registrationhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle: super::PVOID) -> windows_core::HRESULT {
    windows_core::link!("powrprof.dll" "system" fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle : super::PVOID) -> windows_core::HRESULT);
    unsafe { PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PowerWriteACValueIndex(rootpowerkey: Option<super::HKEY>, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, acvalueindex: super::DWORD) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteACValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, acvalueindex : super::DWORD) -> super::DWORD);
    unsafe { PowerWriteACValueIndex(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, acvalueindex) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PowerWriteDCValueIndex(rootpowerkey: Option<super::HKEY>, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, dcvalueindex: super::DWORD) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteDCValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, dcvalueindex : super::DWORD) -> super::DWORD);
    unsafe { PowerWriteDCValueIndex(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, dcvalueindex) }
}
pub type EFFECTIVE_POWER_MODE = i32;
pub type EFFECTIVE_POWER_MODE_CALLBACK = Option<unsafe extern "system" fn(mode: EFFECTIVE_POWER_MODE, context: *const core::ffi::c_void)>;
pub const EFFECTIVE_POWER_MODE_V1: i32 = 1;
pub const EFFECTIVE_POWER_MODE_V2: i32 = 2;
pub const EffectivePowerModeBalanced: EFFECTIVE_POWER_MODE = 2;
pub const EffectivePowerModeBatterySaver: EFFECTIVE_POWER_MODE = 0;
pub const EffectivePowerModeBetterBattery: EFFECTIVE_POWER_MODE = 1;
pub const EffectivePowerModeEnergySaverHighSavings: EFFECTIVE_POWER_MODE = 0;
pub const EffectivePowerModeEnergySaverStandard: EFFECTIVE_POWER_MODE = 1;
pub const EffectivePowerModeGameMode: EFFECTIVE_POWER_MODE = 5;
pub const EffectivePowerModeHighPerformance: EFFECTIVE_POWER_MODE = 3;
pub const EffectivePowerModeMaxPerformance: EFFECTIVE_POWER_MODE = 4;
pub const EffectivePowerModeMixedReality: EFFECTIVE_POWER_MODE = 6;
