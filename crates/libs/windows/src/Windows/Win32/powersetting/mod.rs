#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn PowerGetActiveScheme(userrootpowerkey: Option<super::minwindef::HKEY>, activepolicyguid: *mut *mut windows_core::GUID) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerGetActiveScheme(userrootpowerkey : super::minwindef::HKEY, activepolicyguid : *mut *mut windows_core::GUID) -> u32);
    unsafe { PowerGetActiveScheme(userrootpowerkey.unwrap_or(core::mem::zeroed()) as _, activepolicyguid as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn PowerReadACValue(rootpowerkey: Option<super::minwindef::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: Option<*mut u32>, buffer: Option<*mut u8>, buffersize: Option<*mut u32>) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadACValue(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : *mut u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadACValue(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, r#type.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn PowerReadDCValue(rootpowerkey: Option<super::minwindef::HKEY>, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: Option<*mut u32>, buffer: Option<*mut u8>, buffersize: *mut u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerReadDCValue(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : *mut u32, buffer : *mut u8, buffersize : *mut u32) -> u32);
    unsafe { PowerReadDCValue(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, r#type.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn PowerRegisterForEffectivePowerModeNotifications(version: u32, callback: EFFECTIVE_POWER_MODE_CALLBACK, context: Option<*const core::ffi::c_void>, registrationhandle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("powrprof.dll" "system" fn PowerRegisterForEffectivePowerModeNotifications(version : u32, callback : EFFECTIVE_POWER_MODE_CALLBACK, context : *const core::ffi::c_void, registrationhandle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PowerRegisterForEffectivePowerModeNotifications(version, callback, context.unwrap_or(core::mem::zeroed()) as _, registrationhandle as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn PowerSetActiveScheme(userrootpowerkey: Option<super::minwindef::HKEY>, schemeguid: Option<*const windows_core::GUID>) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerSetActiveScheme(userrootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_core::GUID) -> u32);
    unsafe { PowerSetActiveScheme(userrootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winuser"))]
#[inline]
pub unsafe fn PowerSettingRegisterNotification(settingguid: *const windows_core::GUID, flags: u32, recipient: super::winnt::HANDLE, registrationhandle: *mut super::winuser::HPOWERNOTIFY) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerSettingRegisterNotification(settingguid : *const windows_core::GUID, flags : u32, recipient : super::winnt::HANDLE, registrationhandle : *mut super::winuser::HPOWERNOTIFY) -> u32);
    unsafe { PowerSettingRegisterNotification(settingguid, flags, recipient, registrationhandle as _) }
}
#[cfg(feature = "Win32_winuser")]
#[inline]
pub unsafe fn PowerSettingUnregisterNotification(registrationhandle: super::winuser::HPOWERNOTIFY) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerSettingUnregisterNotification(registrationhandle : super::winuser::HPOWERNOTIFY) -> u32);
    unsafe { PowerSettingUnregisterNotification(registrationhandle as _) }
}
#[inline]
pub unsafe fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("powrprof.dll" "system" fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn PowerWriteACValueIndex(rootpowerkey: Option<super::minwindef::HKEY>, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, acvalueindex: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteACValueIndex(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, acvalueindex : u32) -> u32);
    unsafe { PowerWriteACValueIndex(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, acvalueindex) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn PowerWriteDCValueIndex(rootpowerkey: Option<super::minwindef::HKEY>, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, dcvalueindex: u32) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerWriteDCValueIndex(rootpowerkey : super::minwindef::HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, dcvalueindex : u32) -> u32);
    unsafe { PowerWriteDCValueIndex(rootpowerkey.unwrap_or(core::mem::zeroed()) as _, schemeguid, subgroupofpowersettingsguid.unwrap_or(core::mem::zeroed()) as _, powersettingguid.unwrap_or(core::mem::zeroed()) as _, dcvalueindex) }
}
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
