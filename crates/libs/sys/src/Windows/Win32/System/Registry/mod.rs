#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegistryValueWithFallbackW(hkeyprimary: HKEY, pwszprimarysubkey: super::super::Foundation::PWSTR, hkeyfallback: HKEY, pwszfallbacksubkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR, dwflags: u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, cbdatain: u32, pcbdataout: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCloseKey(hkey: HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryA(lpmachinename: super::super::Foundation::PSTR, hkey: HKEY, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryExA(lpmachinename: super::super::Foundation::PSTR, hkey: HKEY, flags: u32, phkresult: *mut HKEY) -> i32;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryExW(lpmachinename: super::super::Foundation::PWSTR, hkey: HKEY, flags: u32, phkresult: *mut HKEY) -> i32;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegConnectRegistryW(lpmachinename: super::super::Foundation::PWSTR, hkey: HKEY, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCopyTreeA(hkeysrc: HKEY, lpsubkey: super::super::Foundation::PSTR, hkeydest: HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCopyTreeW(hkeysrc: HKEY, lpsubkey: super::super::Foundation::PWSTR, hkeydest: HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCreateKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyExA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, reserved: u32, lpclass: super::super::Foundation::PSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyExW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, reserved: u32, lpclass: super::super::Foundation::PWSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyTransactedA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, reserved: u32, lpclass: super::super::Foundation::PSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegCreateKeyTransactedW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, reserved: u32, lpclass: super::super::Foundation::PWSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegCreateKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyExA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyExW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyTransactedA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, samdesired: u32, reserved: u32, htransaction: super::super::Foundation::HANDLE, pextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyTransactedW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, samdesired: u32, reserved: u32, htransaction: super::super::Foundation::HANDLE, pextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpvaluename: super::super::Foundation::PSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpvaluename: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteTreeA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteTreeW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteValueA(hkey: HKEY, lpvaluename: super::super::Foundation::PSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDeleteValueW(hkey: HKEY, lpvaluename: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDisablePredefinedCache() -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDisablePredefinedCacheEx() -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegDisableReflectionKey(hbase: HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnableReflectionKey(hbase: HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyA(hkey: HKEY, dwindex: u32, lpname: super::super::Foundation::PSTR, cchname: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyExA(hkey: HKEY, dwindex: u32, lpname: super::super::Foundation::PSTR, lpcchname: *mut u32, lpreserved: *mut u32, lpclass: super::super::Foundation::PSTR, lpcchclass: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyExW(hkey: HKEY, dwindex: u32, lpname: super::super::Foundation::PWSTR, lpcchname: *mut u32, lpreserved: *mut u32, lpclass: super::super::Foundation::PWSTR, lpcchclass: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumKeyW(hkey: HKEY, dwindex: u32, lpname: super::super::Foundation::PWSTR, cchname: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumValueA(hkey: HKEY, dwindex: u32, lpvaluename: super::super::Foundation::PSTR, lpcchvaluename: *mut u32, lpreserved: *mut u32, lptype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegEnumValueW(hkey: HKEY, dwindex: u32, lpvaluename: super::super::Foundation::PWSTR, lpcchvaluename: *mut u32, lpreserved: *mut u32, lptype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegFlushKey(hkey: HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegGetKeySecurity(hkey: HKEY, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegGetValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpvalue: super::super::Foundation::PSTR, dwflags: RRF_RT, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegGetValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpvalue: super::super::Foundation::PWSTR, dwflags: RRF_RT, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadAppKeyA(lpfile: super::super::Foundation::PSTR, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadAppKeyW(lpfile: super::super::Foundation::PWSTR, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpfile: super::super::Foundation::PSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpfile: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadMUIStringA(hkey: HKEY, pszvalue: super::super::Foundation::PSTR, pszoutbuf: super::super::Foundation::PSTR, cboutbuf: u32, pcbdata: *mut u32, flags: u32, pszdirectory: super::super::Foundation::PSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegLoadMUIStringW(hkey: HKEY, pszvalue: super::super::Foundation::PWSTR, pszoutbuf: super::super::Foundation::PWSTR, cboutbuf: u32, pcbdata: *mut u32, flags: u32, pszdirectory: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegNotifyChangeKeyValue(hkey: HKEY, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: REG_NOTIFY_FILTER, hevent: super::super::Foundation::HANDLE, fasynchronous: super::super::Foundation::BOOL) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenCurrentUser(samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyExA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyExW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyTransactedA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyTransactedW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOpenUserClassesRoot(htoken: super::super::Foundation::HANDLE, dwoptions: u32, samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegOverridePredefKey(hkey: HKEY, hnewhkey: HKEY) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryInfoKeyA(hkey: HKEY, lpclass: super::super::Foundation::PSTR, lpcchclass: *mut u32, lpreserved: *mut u32, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcbmaxclasslen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryInfoKeyW(hkey: HKEY, lpclass: super::super::Foundation::PWSTR, lpcchclass: *mut u32, lpreserved: *mut u32, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcbmaxclasslen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryMultipleValuesA(hkey: HKEY, val_list: *mut VALENTA, num_vals: u32, lpvaluebuf: super::super::Foundation::PSTR, ldwtotsize: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryMultipleValuesW(hkey: HKEY, val_list: *mut VALENTW, num_vals: u32, lpvaluebuf: super::super::Foundation::PWSTR, ldwtotsize: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryReflectionKey(hbase: HKEY, bisreflectiondisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpdata: super::super::Foundation::PSTR, lpcbdata: *mut i32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueExA(hkey: HKEY, lpvaluename: super::super::Foundation::PSTR, lpreserved: *mut u32, lptype: *mut REG_VALUE_TYPE, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueExW(hkey: HKEY, lpvaluename: super::super::Foundation::PWSTR, lpreserved: *mut u32, lptype: *mut REG_VALUE_TYPE, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegQueryValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpdata: super::super::Foundation::PWSTR, lpcbdata: *mut i32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRenameKey(hkey: HKEY, lpsubkeyname: super::super::Foundation::PWSTR, lpnewkeyname: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegReplaceKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpnewfile: super::super::Foundation::PSTR, lpoldfile: super::super::Foundation::PSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegReplaceKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpnewfile: super::super::Foundation::PWSTR, lpoldfile: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRestoreKeyA(hkey: HKEY, lpfile: super::super::Foundation::PSTR, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegRestoreKeyW(hkey: HKEY, lpfile: super::super::Foundation::PWSTR, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyA(hkey: HKEY, lpfile: super::super::Foundation::PSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyExA(hkey: HKEY, lpfile: super::super::Foundation::PSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyExW(hkey: HKEY, lpfile: super::super::Foundation::PWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSaveKeyW(hkey: HKEY, lpfile: super::super::Foundation::PWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RegSetKeySecurity(hkey: HKEY, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetKeyValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, lpvaluename: super::super::Foundation::PSTR, dwtype: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetKeyValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, lpvaluename: super::super::Foundation::PWSTR, dwtype: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR, dwtype: REG_VALUE_TYPE, lpdata: super::super::Foundation::PSTR, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueExA(hkey: HKEY, lpvaluename: super::super::Foundation::PSTR, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: *const u8, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueExW(hkey: HKEY, lpvaluename: super::super::Foundation::PWSTR, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: *const u8, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegSetValueW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR, dwtype: REG_VALUE_TYPE, lpdata: super::super::Foundation::PWSTR, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegUnLoadKeyA(hkey: HKEY, lpsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::WIN32_ERROR;
    #[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegUnLoadKeyW(hkey: HKEY, lpsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::WIN32_ERROR;
}
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_NO_1X_RATE: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_NO_2X_RATE: i32 = 2i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_NO_4X_RATE: i32 = 4i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_NO_8X_RATE: i32 = 8i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_NO_FW_ENABLE: i32 = 512i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_NO_SBA_ENABLE: i32 = 256i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_REVERSE_INITIALIZATION: i32 = 128i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_SPECIAL_RESERVE: i32 = 1015808i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const AGP_FLAG_SPECIAL_TARGET: i32 = 1048575i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const APMMENUSUSPEND_DISABLED: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const APMMENUSUSPEND_ENABLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const APMMENUSUSPEND_NOCHANGE: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const APMMENUSUSPEND_UNDOCKED: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const APMTIMEOUT_DISABLED: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const BIF_RAWDEVICENEEDSDRIVER: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const BIF_SHOWSIMILARDRIVERS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_BOOT_DEVICE: u32 = 262144u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_CANTSTOPACHILD: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_DISABLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_FAILEDINSTALL: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_FINISHINSTALL_ACTION: u32 = 131072u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_FINISHINSTALL_UI: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_FINISH_INSTALL: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_IGNORE_BOOT_LC: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_MANUAL_INSTALL: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_NEEDS_CLASS_CONFIG: u32 = 524288u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_NEEDS_FORCED_CONFIG: u32 = 2048u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_NETBOOT_CARD: u32 = 4096u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_NET_BOOT: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_NOREMOVEEXIT: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_OKREMOVEROM: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_PARTIAL_LOG_CONF: u32 = 8192u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_REINSTALL: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_REMOVED: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_SUPPRESS_SURPRISE: u32 = 16384u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CONFIGFLAG_VERIFY_HARDWARE: u32 = 32768u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CSCONFIGFLAG_BITS: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CSCONFIGFLAG_DISABLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CSCONFIGFLAG_DO_NOT_CREATE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const CSCONFIGFLAG_DO_NOT_START: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DMSTATEFLAG_APPLYTOALL: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTF_ALWAYSUSE: i32 = 4i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTF_DEFAULT: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTF_INDOSSTART: i32 = 64i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTF_MULTIPLE: i32 = 128i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTF_NEEDSETUP: i32 = 32i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTF_PROVIDESUMB: i32 = 16i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTF_SUPPORTED: i32 = 2i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTF_USESPMODE: i32 = 8i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DOSOPTGF_DEFCLEAN: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DRIVERSIGN_BLOCKING: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DRIVERSIGN_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DRIVERSIGN_WARNING: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub struct DSKTLSYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
    pub wResult: u16,
}
impl ::core::marker::Copy for DSKTLSYSTEMTIME {}
impl ::core::clone::Clone for DSKTLSYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DTRESULTFIX: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DTRESULTOK: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DTRESULTPART: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const DTRESULTPROB: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const EISAFLAG_NO_IO_MERGE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const EISAFLAG_SLOT_IO_FIRST: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const EISA_NO_MAX_FUNCTION: u32 = 255u32;
pub type HKEY = isize;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_CLASSES_ROOT: HKEY = -2147483648i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_CURRENT_CONFIG: HKEY = -2147483643i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_CURRENT_USER: HKEY = -2147483647i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = -2147483641i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_DYN_DATA: HKEY = -2147483642i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_LOCAL_MACHINE: HKEY = -2147483646i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_PERFORMANCE_DATA: HKEY = -2147483644i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = -2147483552i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_PERFORMANCE_TEXT: HKEY = -2147483568i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const HKEY_USERS: HKEY = -2147483645i32 as _;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const IT_COMPACT: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const IT_CUSTOM: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const IT_PORTABLE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const IT_TYPICAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const LASTGOOD_OPERATION: u32 = 255u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const LASTGOOD_OPERATION_DELETE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const LASTGOOD_OPERATION_NOPOSTPROC: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const MF_FLAGS_CREATE_BUT_NO_SHOW_DISABLED: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const MF_FLAGS_EVEN_IF_NO_RESOURCE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const MF_FLAGS_FILL_IN_UNKNOWN_RESOURCE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const MF_FLAGS_NO_CREATE_IF_NO_RESOURCE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const NUM_EISA_RANGES: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const NUM_RESOURCE_MAP: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCIC_DEFAULT_IRQMASK: u32 = 20152u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCIC_DEFAULT_NUMSOCKETS: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCI_OPTIONS_USE_BIOS: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCI_OPTIONS_USE_IRQ_STEERING: i32 = 2i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_DEF_MEMBEGIN: u32 = 786432u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_DEF_MEMEND: u32 = 16777215u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_DEF_MEMLEN: u32 = 4096u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_DEF_MIN_REGION: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_OPT_AUTOMEM: i32 = 4i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_OPT_HAVE_SOCKET: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_OPT_NO_APMREMOVE: i32 = 32i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_OPT_NO_AUDIO: i32 = 16i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PCMCIA_OPT_NO_SOUND: i32 = 8i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_OPTION_DEFAULT: u32 = 15u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_OPTION_ENABLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_OPTION_MSSPEC: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_OPTION_REALMODE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_OPTION_REGISTRY: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_DISABLED: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_ENABLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_ERROR: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MAX: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_COMPATIBLE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_ERROR: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_INVALID: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_MAX: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_NOKEY: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_NONE: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_NORMAL: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_OVERRIDE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_MINIPORT_SUCCESS: u32 = 6u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_TABLE_BAD: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_TABLE_ERROR: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_TABLE_MAX: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_TABLE_MSSPEC: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_TABLE_NONE: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_TABLE_REALMODE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_TABLE_REGISTRY: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PIR_STATUS_TABLE_SUCCESS: u32 = 6u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type PQUERYHANDLER = ::core::option::Option<unsafe extern "system" fn(keycontext: *mut ::core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut ::core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const PROVIDER_KEEPS_VALUE_LENGTH: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_CONFLICTDMA: u32 = 524288u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_CONFLICTIO: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_CONFLICTIRQ: u32 = 262144u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_CONFLICTMEM: u32 = 131072u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_GENFORCEDCONFIG: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_MAPIRQ2TO9: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_NEEDFULLCONFIG: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_NODETCONFIG: u32 = 32768u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_NOTDETDMA: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_NOTDETIO: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_NOTDETIRQ: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_NOTDETMEM: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGDF_NOTVERIFIED: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGSTR_MAX_VALUE_LENGTH: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REGSTR_VAL_MAX_HCID_LEN: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type REG_CREATE_KEY_DISPOSITION = u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_CREATED_NEW_KEY: REG_CREATE_KEY_DISPOSITION = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_OPENED_EXISTING_KEY: REG_CREATE_KEY_DISPOSITION = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_MUI_STRING_TRUNCATE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type REG_NOTIFY_FILTER = u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_NOTIFY_CHANGE_NAME: REG_NOTIFY_FILTER = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: REG_NOTIFY_FILTER = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_NOTIFY_CHANGE_LAST_SET: REG_NOTIFY_FILTER = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_NOTIFY_CHANGE_SECURITY: REG_NOTIFY_FILTER = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_NOTIFY_THREAD_AGNOSTIC: REG_NOTIFY_FILTER = 268435456u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type REG_OPEN_CREATE_OPTIONS = u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_OPTION_RESERVED: REG_OPEN_CREATE_OPTIONS = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_OPTION_NON_VOLATILE: REG_OPEN_CREATE_OPTIONS = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_OPTION_VOLATILE: REG_OPEN_CREATE_OPTIONS = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_OPTION_CREATE_LINK: REG_OPEN_CREATE_OPTIONS = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_OPTION_BACKUP_RESTORE: REG_OPEN_CREATE_OPTIONS = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_OPTION_OPEN_LINK: REG_OPEN_CREATE_OPTIONS = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_OPTION_DONT_VIRTUALIZE: REG_OPEN_CREATE_OPTIONS = 16u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_PROCESS_APPKEY: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type REG_RESTORE_KEY_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_FORCE_RESTORE: REG_RESTORE_KEY_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_WHOLE_HIVE_VOLATILE: REG_RESTORE_KEY_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type REG_SAM_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_QUERY_VALUE: REG_SAM_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_SET_VALUE: REG_SAM_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_CREATE_SUB_KEY: REG_SAM_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_ENUMERATE_SUB_KEYS: REG_SAM_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_NOTIFY: REG_SAM_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_CREATE_LINK: REG_SAM_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_WOW64_32KEY: REG_SAM_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_WOW64_64KEY: REG_SAM_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_WOW64_RES: REG_SAM_FLAGS = 768u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_READ: REG_SAM_FLAGS = 131097u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_WRITE: REG_SAM_FLAGS = 131078u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_EXECUTE: REG_SAM_FLAGS = 131097u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const KEY_ALL_ACCESS: REG_SAM_FLAGS = 983103u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type REG_SAVE_FORMAT = u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_STANDARD_FORMAT: REG_SAVE_FORMAT = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_LATEST_FORMAT: REG_SAVE_FORMAT = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_NO_COMPRESSION: REG_SAVE_FORMAT = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_SECURE_CONNECTION: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type REG_VALUE_TYPE = u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_NONE: REG_VALUE_TYPE = 0u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_SZ: REG_VALUE_TYPE = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_EXPAND_SZ: REG_VALUE_TYPE = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_BINARY: REG_VALUE_TYPE = 3u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_DWORD: REG_VALUE_TYPE = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_DWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_DWORD_BIG_ENDIAN: REG_VALUE_TYPE = 5u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_LINK: REG_VALUE_TYPE = 6u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_MULTI_SZ: REG_VALUE_TYPE = 7u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_RESOURCE_LIST: REG_VALUE_TYPE = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_FULL_RESOURCE_DESCRIPTOR: REG_VALUE_TYPE = 9u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_RESOURCE_REQUIREMENTS_LIST: REG_VALUE_TYPE = 10u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_QWORD: REG_VALUE_TYPE = 11u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const REG_QWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = 11u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_NOEXPAND: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub type RRF_RT = u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_ANY: RRF_RT = 65535u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_DWORD: RRF_RT = 24u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_QWORD: RRF_RT = 72u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_REG_BINARY: RRF_RT = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_REG_DWORD: RRF_RT = 16u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_REG_EXPAND_SZ: RRF_RT = 4u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_REG_MULTI_SZ: RRF_RT = 32u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_REG_NONE: RRF_RT = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_REG_QWORD: RRF_RT = 64u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_RT_REG_SZ: RRF_RT = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_SUBKEY_WOW6432KEY: u32 = 131072u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_SUBKEY_WOW6464KEY: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_WOW64_MASK: u32 = 196608u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const RRF_ZEROONFAILURE: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_BATCHINF: i32 = 4i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_CLEAN: i32 = 8i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_EXPRESS: i32 = 2i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_FIRSTTIME: i32 = 1i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_INSETUP: i32 = 16i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_NETHDBOOT: i32 = 64i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_NETRPLBOOT: i32 = 128i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_NETSETUP: i32 = 32i32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const SUF_SBSCOPYOK: i32 = 256i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VALENTA {
    pub ve_valuename: super::super::Foundation::PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VALENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VALENTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VALENTW {
    pub ve_valuename: super::super::Foundation::PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VALENTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VALENTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const VPDF_DISABLEPWRMGMT: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const VPDF_DISABLEPWRSTATUSPOLL: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const VPDF_DISABLERINGRESUME: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const VPDF_FORCEAPM10MODE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const VPDF_SHOWMULTIBATT: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub const VPDF_SKIPINTELSLCHECK: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub struct provider_info {
    pub pi_R0_1val: PQUERYHANDLER,
    pub pi_R0_allvals: PQUERYHANDLER,
    pub pi_R3_1val: PQUERYHANDLER,
    pub pi_R3_allvals: PQUERYHANDLER,
    pub pi_flags: u32,
    pub pi_key_context: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for provider_info {}
impl ::core::clone::Clone for provider_info {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct pvalueA {
    pub pv_valuename: super::super::Foundation::PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut ::core::ffi::c_void,
    pub pv_type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for pvalueA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for pvalueA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Registry', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct pvalueW {
    pub pv_valuename: super::super::Foundation::PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut ::core::ffi::c_void,
    pub pv_type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for pvalueW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for pvalueW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Registry'*"]
pub struct val_context {
    pub valuelen: i32,
    pub value_context: *mut ::core::ffi::c_void,
    pub val_buff_ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for val_context {}
impl ::core::clone::Clone for val_context {
    fn clone(&self) -> Self {
        *self
    }
}
