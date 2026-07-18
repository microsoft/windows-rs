#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerGetActiveScheme(userrootpowerkey : super::HKEY, activepolicyguid : *mut *mut windows_sys::core::GUID) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadACValue(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, r#type : *mut u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerReadDCValue(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, r#type : *mut u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerRegisterForEffectivePowerModeNotifications(version : u32, callback : EFFECTIVE_POWER_MODE_CALLBACK, context : *const core::ffi::c_void, registrationhandle : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerSetActiveScheme(userrootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID) -> u32);
#[cfg(all(feature = "winnt", feature = "winuser"))]
windows_link::link!("powrprof.dll" "system" fn PowerSettingRegisterNotification(settingguid : *const windows_sys::core::GUID, flags : u32, recipient : super::HANDLE, registrationhandle : *mut super::HPOWERNOTIFY) -> u32);
#[cfg(feature = "winuser")]
windows_link::link!("powrprof.dll" "system" fn PowerSettingUnregisterNotification(registrationhandle : super::HPOWERNOTIFY) -> u32);
windows_link::link!("powrprof.dll" "system" fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteACValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, acvalueindex : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("powrprof.dll" "system" fn PowerWriteDCValueIndex(rootpowerkey : super::HKEY, schemeguid : *const windows_sys::core::GUID, subgroupofpowersettingsguid : *const windows_sys::core::GUID, powersettingguid : *const windows_sys::core::GUID, dcvalueindex : u32) -> u32);
pub type EFFECTIVE_POWER_MODE = i32;
pub type EFFECTIVE_POWER_MODE_CALLBACK = Option<unsafe extern "system" fn(mode: EFFECTIVE_POWER_MODE, context: *const core::ffi::c_void)>;
pub const EFFECTIVE_POWER_MODE_V1: u32 = 1;
pub const EFFECTIVE_POWER_MODE_V2: u32 = 2;
pub const EffectivePowerModeBalanced: EFFECTIVE_POWER_MODE = 2;
pub const EffectivePowerModeBatterySaver: EFFECTIVE_POWER_MODE = 0;
pub const EffectivePowerModeBetterBattery: EFFECTIVE_POWER_MODE = 1;
pub const EffectivePowerModeEnergySaverHighSavings: EFFECTIVE_POWER_MODE = 0;
pub const EffectivePowerModeEnergySaverStandard: EFFECTIVE_POWER_MODE = 1;
pub const EffectivePowerModeGameMode: EFFECTIVE_POWER_MODE = 5;
pub const EffectivePowerModeHighPerformance: EFFECTIVE_POWER_MODE = 3;
pub const EffectivePowerModeMaxPerformance: EFFECTIVE_POWER_MODE = 4;
pub const EffectivePowerModeMixedReality: EFFECTIVE_POWER_MODE = 6;
