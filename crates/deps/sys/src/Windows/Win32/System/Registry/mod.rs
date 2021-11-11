#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegistryValueWithFallbackW(hkeyprimary: HKEY, pwszprimarysubkey: super::super::Foundation::PWSTR, hkeyfallback: HKEY, pwszfallbacksubkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR, dwflags: u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, cbdatain: u32, pcbdataout: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCloseKey(hkey: HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryA(lpmachinename: super::super::Foundation::PSTR, hkey: HKEY, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryExA(lpmachinename: super::super::Foundation::PSTR, hkey: HKEY, flags: u32, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryExW(lpmachinename: super::super::Foundation::PWSTR, hkey: HKEY, flags: u32, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryW(lpmachinename: super::super::Foundation::PWSTR, hkey: HKEY, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCopyTreeA(hkeysrc: HKEY, lpsubkey: super::super::Foundation::PSTR, hkeydest: HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCopyTreeW(hkeysrc: HKEY, lpsubkey: super::super::Foundation::PWSTR, hkeydest: HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCreateKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyExA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, reserved: u32, lpclass: super::super::Foundation::PSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyExW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, reserved: u32, lpclass: super::super::Foundation::PWSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyTransactedA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, reserved: u32, lpclass: super::super::Foundation::PSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyTransactedW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, reserved: u32, lpclass: super::super::Foundation::PWSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCreateKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyExA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, samdesired: u32, reserved: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyExW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, samdesired: u32, reserved: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyTransactedA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, samdesired: u32, reserved: u32, htransaction: super::super::Foundation::HANDLE, pextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyTransactedW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, samdesired: u32, reserved: u32, htransaction: super::super::Foundation::HANDLE, pextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpvaluename: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpvaluename: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteTreeA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteTreeW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteValueA(hkey: HKEY, lpvaluename: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteValueW(hkey: HKEY, lpvaluename: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDisablePredefinedCache() -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDisablePredefinedCacheEx() -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`*"]
    pub fn RegDisableReflectionKey(hbase: HKEY) -> i32;
    #[doc = "*Required features: `Win32_System_Registry`*"]
    pub fn RegEnableReflectionKey(hbase: HKEY) -> i32;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyA(hkey: HKEY, dwindex: u32, lpname: super::super::Foundation::PSTR, cchname: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyExA(hkey: HKEY, dwindex: u32, lpname: super::super::Foundation::PSTR, lpcchname: *mut u32, lpreserved: *mut u32, lpclass: super::super::Foundation::PSTR, lpcchclass: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyExW(hkey: HKEY, dwindex: u32, lpname: super::super::Foundation::PWSTR, lpcchname: *mut u32, lpreserved: *mut u32, lpclass: super::super::Foundation::PWSTR, lpcchclass: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyW(hkey: HKEY, dwindex: u32, lpname: super::super::Foundation::PWSTR, cchname: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumValueA(hkey: HKEY, dwindex: u32, lpvaluename: super::super::Foundation::PSTR, lpcchvaluename: *mut u32, lpreserved: *mut u32, lptype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumValueW(hkey: HKEY, dwindex: u32, lpvaluename: super::super::Foundation::PWSTR, lpcchvaluename: *mut u32, lpreserved: *mut u32, lptype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegFlushKey(hkey: HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegGetKeySecurity(hkey: HKEY, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegGetValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpvalue: super::super::Foundation::PSTR, dwflags: RRF_RT, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegGetValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpvalue: super::super::Foundation::PWSTR, dwflags: RRF_RT, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadAppKeyA(lpfile: super::super::Foundation::PSTR, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadAppKeyW(lpfile: super::super::Foundation::PWSTR, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpfile: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpfile: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadMUIStringA(hkey: HKEY, pszvalue: super::super::Foundation::PSTR, pszoutbuf: super::super::Foundation::PSTR, cboutbuf: u32, pcbdata: *mut u32, flags: u32, pszdirectory: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadMUIStringW(hkey: HKEY, pszvalue: super::super::Foundation::PWSTR, pszoutbuf: super::super::Foundation::PWSTR, cboutbuf: u32, pcbdata: *mut u32, flags: u32, pszdirectory: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegNotifyChangeKeyValue(hkey: HKEY, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: REG_NOTIFY_FILTER, hevent: super::super::Foundation::HANDLE, fasynchronous: super::super::Foundation::BOOL) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenCurrentUser(samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyExA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyExW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyTransactedA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyTransactedW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenUserClassesRoot(htoken: super::super::Foundation::HANDLE, dwoptions: u32, samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOverridePredefKey(hkey: HKEY, hnewhkey: HKEY) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryInfoKeyA(hkey: HKEY, lpclass: super::super::Foundation::PSTR, lpcchclass: *mut u32, lpreserved: *mut u32, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcbmaxclasslen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryInfoKeyW(hkey: HKEY, lpclass: super::super::Foundation::PWSTR, lpcchclass: *mut u32, lpreserved: *mut u32, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcbmaxclasslen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryMultipleValuesA(hkey: HKEY, val_list: *mut VALENTA, num_vals: u32, lpvaluebuf: super::super::Foundation::PSTR, ldwtotsize: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryMultipleValuesW(hkey: HKEY, val_list: *mut VALENTW, num_vals: u32, lpvaluebuf: super::super::Foundation::PWSTR, ldwtotsize: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryReflectionKey(hbase: HKEY, bisreflectiondisabled: *mut super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpdata: super::super::Foundation::PSTR, lpcbdata: *mut i32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueExA(hkey: HKEY, lpvaluename: super::super::Foundation::PSTR, lpreserved: *mut u32, lptype: *mut REG_VALUE_TYPE, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueExW(hkey: HKEY, lpvaluename: super::super::Foundation::PWSTR, lpreserved: *mut u32, lptype: *mut REG_VALUE_TYPE, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpdata: super::super::Foundation::PWSTR, lpcbdata: *mut i32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRenameKey(hkey: HKEY, lpsubkeyname: super::super::Foundation::PWSTR, lpnewkeyname: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegReplaceKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpnewfile: super::super::Foundation::PSTR, lpoldfile: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegReplaceKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpnewfile: super::super::Foundation::PWSTR, lpoldfile: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRestoreKeyA(hkey: HKEY, lpfile: super::super::Foundation::PSTR, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRestoreKeyW(hkey: HKEY, lpfile: super::super::Foundation::PWSTR, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyA(hkey: HKEY, lpfile: super::super::Foundation::PSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyExA(hkey: HKEY, lpfile: super::super::Foundation::PSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flags: REG_SAVE_FORMAT) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyExW(hkey: HKEY, lpfile: super::super::Foundation::PWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flags: REG_SAVE_FORMAT) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyW(hkey: HKEY, lpfile: super::super::Foundation::PWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSetKeySecurity(hkey: HKEY, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetKeyValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpvaluename: super::super::Foundation::PSTR, dwtype: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetKeyValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpvaluename: super::super::Foundation::PWSTR, dwtype: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, dwtype: REG_VALUE_TYPE, lpdata: super::super::Foundation::PSTR, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueExA(hkey: HKEY, lpvaluename: super::super::Foundation::PSTR, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: *const u8, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueExW(hkey: HKEY, lpvaluename: super::super::Foundation::PWSTR, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: *const u8, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, dwtype: REG_VALUE_TYPE, lpdata: super::super::Foundation::PWSTR, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegUnLoadKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_System_Registry`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegUnLoadKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
}
