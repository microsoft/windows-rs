#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerGetActiveScheme(userrootpowerkey : super::HKEY, activepolicyguid : *mut *mut windows_sys::core::GUID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadACValue(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, r#type : super::PULONG, buffer : super::LPBYTE, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerReadDCValue(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, r#type : super::PULONG, buffer : super::PUCHAR, buffersize : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn PowerRegisterForEffectivePowerModeNotifications(version : super::ULONG, callback : EFFECTIVE_POWER_MODE_CALLBACK, context : super::PVOID, registrationhandle : *mut super::PVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerSetActiveScheme(userrootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID) -> super::DWORD);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt", feature = "winuser"))]
windows_link::link!("powrprof.dll" "system" fn PowerSettingRegisterNotification(settingguid : super::LPCGUID, flags : super::DWORD, recipient : super::HANDLE, registrationhandle : super::PHPOWERNOTIFY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winuser"))]
windows_link::link!("powrprof.dll" "system" fn PowerSettingUnregisterNotification(registrationhandle : super::HPOWERNOTIFY) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle : super::PVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteACValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, acvalueindex : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("powrprof.dll" "system" fn PowerWriteDCValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, dcvalueindex : super::DWORD) -> super::DWORD);
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
