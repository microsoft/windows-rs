#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegistryValueWithFallbackW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCloseKey();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryExA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryExW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCopyTreeA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCopyTreeW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCreateKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyExA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyExW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyTransactedA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyTransactedW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCreateKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyExA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyExW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyTransactedA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyTransactedW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyValueA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyValueW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteTreeA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteTreeW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteValueA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteValueW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDisablePredefinedCache();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDisablePredefinedCacheEx();
    #[doc = "*Required features: `Win32_System_Registry`*"]
    pub fn RegDisableReflectionKey();
    #[doc = "*Required features: `Win32_System_Registry`*"]
    pub fn RegEnableReflectionKey();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyExA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyExW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumValueA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumValueW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegFlushKey();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegGetKeySecurity();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegGetValueA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegGetValueW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadAppKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadAppKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadMUIStringA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadMUIStringW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegNotifyChangeKeyValue();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenCurrentUser();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyExA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyExW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyTransactedA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyTransactedW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenUserClassesRoot();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOverridePredefKey();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryInfoKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryInfoKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryMultipleValuesA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryMultipleValuesW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryReflectionKey();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueExA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueExW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRenameKey();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegReplaceKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegReplaceKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRestoreKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRestoreKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyExA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyExW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSetKeySecurity();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetKeyValueA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetKeyValueW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueExA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueExW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueW();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegUnLoadKeyA();
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegUnLoadKeyW();
}
