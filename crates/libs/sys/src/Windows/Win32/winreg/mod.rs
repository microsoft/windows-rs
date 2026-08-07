windows_link::link!("advapi32.dll" "system" fn AbortSystemShutdownA(lpmachinename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn AbortSystemShutdownW(lpmachinename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CheckForHiberboot(phiberboot : *mut bool, bclearflag : bool) -> u32);
windows_link::link!("advapi32.dll" "system" fn InitiateShutdownA(lpmachinename : windows_sys::core::PCSTR, lpmessage : windows_sys::core::PCSTR, dwgraceperiod : u32, dwshutdownflags : u32, dwreason : u32) -> u32);
windows_link::link!("advapi32.dll" "system" fn InitiateShutdownW(lpmachinename : windows_sys::core::PCWSTR, lpmessage : windows_sys::core::PCWSTR, dwgraceperiod : u32, dwshutdownflags : u32, dwreason : u32) -> u32);
windows_link::link!("advapi32.dll" "system" fn InitiateSystemShutdownA(lpmachinename : windows_sys::core::PCSTR, lpmessage : windows_sys::core::PCSTR, dwtimeout : u32, bforceappsclosed : windows_sys::core::BOOL, brebootaftershutdown : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn InitiateSystemShutdownExA(lpmachinename : windows_sys::core::PCSTR, lpmessage : windows_sys::core::PCSTR, dwtimeout : u32, bforceappsclosed : windows_sys::core::BOOL, brebootaftershutdown : windows_sys::core::BOOL, dwreason : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn InitiateSystemShutdownExW(lpmachinename : windows_sys::core::PCWSTR, lpmessage : windows_sys::core::PCWSTR, dwtimeout : u32, bforceappsclosed : windows_sys::core::BOOL, brebootaftershutdown : windows_sys::core::BOOL, dwreason : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn InitiateSystemShutdownW(lpmachinename : windows_sys::core::PCWSTR, lpmessage : windows_sys::core::PCWSTR, dwtimeout : u32, bforceappsclosed : windows_sys::core::BOOL, brebootaftershutdown : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCloseKey(hkey : super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegConnectRegistryA(lpmachinename : windows_sys::core::PCSTR, hkey : super::HKEY, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegConnectRegistryExA(lpmachinename : windows_sys::core::PCSTR, hkey : super::HKEY, flags : u32, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegConnectRegistryExW(lpmachinename : windows_sys::core::PCWSTR, hkey : super::HKEY, flags : u32, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegConnectRegistryW(lpmachinename : windows_sys::core::PCWSTR, hkey : super::HKEY, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCopyTreeA(hkeysrc : super::HKEY, lpsubkey : windows_sys::core::PCSTR, hkeydest : super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCopyTreeW(hkeysrc : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, hkeydest : super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyExA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, reserved : u32, lpclass : windows_sys::core::PCSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, phkresult : *mut super::HKEY, lpdwdisposition : *mut u32) -> LSTATUS);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyExW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, reserved : u32, lpclass : windows_sys::core::PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, phkresult : *mut super::HKEY, lpdwdisposition : *mut u32) -> LSTATUS);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyTransactedA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, reserved : u32, lpclass : windows_sys::core::PCSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, phkresult : *mut super::HKEY, lpdwdisposition : *mut u32, htransaction : super::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> LSTATUS);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyTransactedW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, reserved : u32, lpclass : windows_sys::core::PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, phkresult : *mut super::HKEY, lpdwdisposition : *mut u32, htransaction : super::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyExA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, samdesired : REGSAM, reserved : u32) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyExW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, samdesired : REGSAM, reserved : u32) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, samdesired : REGSAM, reserved : u32, htransaction : super::HANDLE, pextendedparameter : *const core::ffi::c_void) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, samdesired : REGSAM, reserved : u32, htransaction : super::HANDLE, pextendedparameter : *const core::ffi::c_void) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyValueA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, lpvaluename : windows_sys::core::PCSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyValueW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpvaluename : windows_sys::core::PCWSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteTreeA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteTreeW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteValueA(hkey : super::HKEY, lpvaluename : windows_sys::core::PCSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteValueW(hkey : super::HKEY, lpvaluename : windows_sys::core::PCWSTR) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegDisablePredefinedCache() -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegDisablePredefinedCacheEx() -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDisableReflectionKey(hbase : super::HKEY) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnableReflectionKey(hbase : super::HKEY) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyA(hkey : super::HKEY, dwindex : u32, lpname : windows_sys::core::PSTR, cchname : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyExA(hkey : super::HKEY, dwindex : u32, lpname : windows_sys::core::PSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : windows_sys::core::PSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::FILETIME) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyExW(hkey : super::HKEY, dwindex : u32, lpname : windows_sys::core::PWSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : windows_sys::core::PWSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::FILETIME) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyW(hkey : super::HKEY, dwindex : u32, lpname : windows_sys::core::PWSTR, cchname : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumValueA(hkey : super::HKEY, dwindex : u32, lpvaluename : windows_sys::core::PSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumValueW(hkey : super::HKEY, dwindex : u32, lpvaluename : windows_sys::core::PWSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegFlushKey(hkey : super::HKEY) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegGetKeySecurity(hkey : super::HKEY, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegGetValueA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, lpvalue : windows_sys::core::PCSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegGetValueW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PCWSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegLoadAppKeyA(lpfile : windows_sys::core::PCSTR, phkresult : *mut super::HKEY, samdesired : REGSAM, dwoptions : u32, reserved : u32) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegLoadAppKeyW(lpfile : windows_sys::core::PCWSTR, phkresult : *mut super::HKEY, samdesired : REGSAM, dwoptions : u32, reserved : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegLoadKeyA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, lpfile : windows_sys::core::PCSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegLoadKeyW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpfile : windows_sys::core::PCWSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegLoadMUIStringA(hkey : super::HKEY, pszvalue : windows_sys::core::PCSTR, pszoutbuf : windows_sys::core::PSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : windows_sys::core::PCSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegLoadMUIStringW(hkey : super::HKEY, pszvalue : windows_sys::core::PCWSTR, pszoutbuf : windows_sys::core::PWSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : windows_sys::core::PCWSTR) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegNotifyChangeKeyValue(hkey : super::HKEY, bwatchsubtree : windows_sys::core::BOOL, dwnotifyfilter : u32, hevent : super::HANDLE, fasynchronous : windows_sys::core::BOOL) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenCurrentUser(samdesired : REGSAM, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyExA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyTransactedA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::HKEY, htransaction : super::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyTransactedW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::HKEY, htransaction : super::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenUserClassesRoot(htoken : super::HANDLE, dwoptions : u32, samdesired : REGSAM, phkresult : *mut super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegOverridePredefKey(hkey : super::HKEY, hnewhkey : super::HKEY) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryInfoKeyA(hkey : super::HKEY, lpclass : windows_sys::core::PSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::FILETIME) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryInfoKeyW(hkey : super::HKEY, lpclass : windows_sys::core::PWSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::FILETIME) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryMultipleValuesA(hkey : super::HKEY, val_list : *mut VALENTA, num_vals : u32, lpvaluebuf : windows_sys::core::PSTR, ldwtotsize : *mut u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryMultipleValuesW(hkey : super::HKEY, val_list : *mut VALENTW, num_vals : u32, lpvaluebuf : windows_sys::core::PWSTR, ldwtotsize : *mut u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryReflectionKey(hbase : super::HKEY, bisreflectiondisabled : *mut windows_sys::core::BOOL) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryValueA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, lpdata : windows_sys::core::PSTR, lpcbdata : *mut i32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryValueExA(hkey : super::HKEY, lpvaluename : windows_sys::core::PCSTR, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryValueExW(hkey : super::HKEY, lpvaluename : windows_sys::core::PCWSTR, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryValueW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpdata : windows_sys::core::PWSTR, lpcbdata : *mut i32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegRenameKey(hkey : super::HKEY, lpsubkeyname : windows_sys::core::PCWSTR, lpnewkeyname : windows_sys::core::PCWSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegReplaceKeyA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, lpnewfile : windows_sys::core::PCSTR, lpoldfile : windows_sys::core::PCSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegReplaceKeyW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpnewfile : windows_sys::core::PCWSTR, lpoldfile : windows_sys::core::PCWSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegRestoreKeyA(hkey : super::HKEY, lpfile : windows_sys::core::PCSTR, dwflags : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegRestoreKeyW(hkey : super::HKEY, lpfile : windows_sys::core::PCWSTR, dwflags : u32) -> LSTATUS);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("advapi32.dll" "system" fn RegSaveKeyA(hkey : super::HKEY, lpfile : windows_sys::core::PCSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> LSTATUS);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("advapi32.dll" "system" fn RegSaveKeyExA(hkey : super::HKEY, lpfile : windows_sys::core::PCSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, flags : u32) -> LSTATUS);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("advapi32.dll" "system" fn RegSaveKeyExW(hkey : super::HKEY, lpfile : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, flags : u32) -> LSTATUS);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("advapi32.dll" "system" fn RegSaveKeyW(hkey : super::HKEY, lpfile : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> LSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegSetKeySecurity(hkey : super::HKEY, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetKeyValueA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, lpvaluename : windows_sys::core::PCSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetKeyValueW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpvaluename : windows_sys::core::PCWSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetValueA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR, dwtype : u32, lpdata : windows_sys::core::PCSTR, cbdata : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetValueExA(hkey : super::HKEY, lpvaluename : windows_sys::core::PCSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetValueExW(hkey : super::HKEY, lpvaluename : windows_sys::core::PCWSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetValueW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR, dwtype : u32, lpdata : windows_sys::core::PCWSTR, cbdata : u32) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegUnLoadKeyA(hkey : super::HKEY, lpsubkey : windows_sys::core::PCSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegUnLoadKeyW(hkey : super::HKEY, lpsubkey : windows_sys::core::PCWSTR) -> LSTATUS);
#[cfg(feature = "minwindef")]
pub const HKEY_CLASSES_ROOT: super::HKEY = -2147483648 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_CURRENT_CONFIG: super::HKEY = -2147483643 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_CURRENT_USER: super::HKEY = -2147483647 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: super::HKEY = -2147483641 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_DYN_DATA: super::HKEY = -2147483642 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_LOCAL_MACHINE: super::HKEY = -2147483646 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_PERFORMANCE_DATA: super::HKEY = -2147483644 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_PERFORMANCE_NLSTEXT: super::HKEY = -2147483552 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_PERFORMANCE_TEXT: super::HKEY = -2147483568 as _;
#[cfg(feature = "minwindef")]
pub const HKEY_USERS: super::HKEY = -2147483645 as _;
pub type LSTATUS = i32;
pub const MAX_SHUTDOWN_TIMEOUT: i32 = 315360000;
pub type PPROVIDER = *mut REG_PROVIDER;
pub type PPVALUE = PPVALUEA;
pub type PPVALUEA = *mut PVALUEA;
pub type PPVALUEW = *mut PVALUEW;
pub type PQUERYHANDLER = Option<unsafe extern "C" fn(keycontext: *mut core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
pub const PROVIDER_KEEPS_VALUE_LENGTH: i32 = 1;
pub type PVALCONTEXT = *mut val_context;
pub type PVALENT = PVALENTA;
pub type PVALENTA = *mut VALENTA;
pub type PVALENTW = *mut VALENTW;
pub type PVALUE = PVALUEA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PVALUEA {
    pub pv_valuename: windows_sys::core::PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut core::ffi::c_void,
    pub pv_type: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PVALUEW {
    pub pv_valuename: windows_sys::core::PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut core::ffi::c_void,
    pub pv_type: u32,
}
pub type QUERYHANDLER = Option<unsafe extern "C" fn(keycontext: *mut core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
pub const REASON_HWINSTALL: i32 = 65538;
pub const REASON_LEGACY_API: u32 = 2147942400;
pub const REASON_OTHER: i32 = 0;
pub const REASON_PLANNED_FLAG: u32 = 2147483648;
pub const REASON_SERVICEHANG: i32 = 196613;
pub const REASON_SWHWRECONF: i32 = 196612;
pub const REASON_SWINSTALL: i32 = 196610;
pub const REASON_UNKNOWN: i32 = 255;
pub const REASON_UNSTABLE: i32 = 327686;
#[cfg(feature = "winnt")]
pub type REGSAM = super::ACCESS_MASK;
pub const REG_ALLOW_TRANSPORT_FALLBACK: i32 = 2;
pub const REG_ALLOW_UNSECURE_CONNECTION: i32 = 4;
pub const REG_MUI_STRING_TRUNCATE: i32 = 1;
pub const REG_PROCESS_APPKEY: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REG_PROVIDER {
    pub pi_R0_1val: PQUERYHANDLER,
    pub pi_R0_allvals: PQUERYHANDLER,
    pub pi_R3_1val: PQUERYHANDLER,
    pub pi_R3_allvals: PQUERYHANDLER,
    pub pi_flags: u32,
    pub pi_key_context: *mut core::ffi::c_void,
}
pub const REG_SECURE_CONNECTION: i32 = 1;
pub const REG_USE_CURRENT_SECURITY_CONTEXT: i32 = 2;
pub const RRF_NOEXPAND: i32 = 268435456;
pub const RRF_RT_ANY: i32 = 65535;
pub const RRF_RT_DWORD: i32 = 24;
pub const RRF_RT_QWORD: i32 = 72;
pub const RRF_RT_REG_BINARY: i32 = 8;
pub const RRF_RT_REG_DWORD: i32 = 16;
pub const RRF_RT_REG_EXPAND_SZ: i32 = 4;
pub const RRF_RT_REG_MULTI_SZ: i32 = 32;
pub const RRF_RT_REG_NONE: i32 = 1;
pub const RRF_RT_REG_QWORD: i32 = 64;
pub const RRF_RT_REG_SZ: i32 = 2;
pub const RRF_SUBKEY_WOW6432KEY: i32 = 131072;
pub const RRF_SUBKEY_WOW6464KEY: i32 = 65536;
pub const RRF_WOW64_MASK: i32 = 196608;
pub const RRF_ZEROONFAILURE: i32 = 536870912;
pub const SHUTDOWN_ARSO: i32 = 8192;
pub const SHUTDOWN_CHECK_SAFE_FOR_SERVER: i32 = 16384;
pub const SHUTDOWN_FORCE_OTHERS: i32 = 1;
pub const SHUTDOWN_FORCE_SELF: i32 = 2;
pub const SHUTDOWN_GRACE_OVERRIDE: i32 = 32;
pub const SHUTDOWN_HYBRID: i32 = 512;
pub const SHUTDOWN_INSTALL_UPDATES: i32 = 64;
pub const SHUTDOWN_MOBILE_UI: i32 = 4096;
pub const SHUTDOWN_NOREBOOT: i32 = 16;
pub const SHUTDOWN_POWEROFF: i32 = 8;
pub const SHUTDOWN_RESTART: i32 = 4;
pub const SHUTDOWN_RESTARTAPPS: i32 = 128;
pub const SHUTDOWN_RESTART_BOOTOPTIONS: i32 = 1024;
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: i32 = 256;
pub const SHUTDOWN_SOFT_REBOOT: i32 = 2048;
pub const SHUTDOWN_SYSTEM_INITIATED: i32 = 65536;
pub const SHUTDOWN_UPDATE_POWEROFF: i32 = 131072;
pub const SHUTDOWN_VAIL_CONTAINER: i32 = 32768;
pub type VALENT = VALENTA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VALENTA {
    pub ve_valuename: windows_sys::core::PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VALENTW {
    pub ve_valuename: windows_sys::core::PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
}
pub const WIN31_CLASS: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct val_context {
    pub valuelen: i32,
    pub value_context: *mut core::ffi::c_void,
    pub val_buff_ptr: *mut core::ffi::c_void,
}
