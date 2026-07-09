windows_link::link!("advapi32.dll" "system" fn AbortSystemShutdownA(lpmachinename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn AbortSystemShutdownW(lpmachinename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CheckForHiberboot(phiberboot : *mut bool, bclearflag : bool) -> u32);
windows_link::link!("advapi32.dll" "system" fn InitiateShutdownA(lpmachinename : windows_sys::core::PCSTR, lpmessage : windows_sys::core::PCSTR, dwgraceperiod : u32, dwshutdownflags : u32, dwreason : u32) -> u32);
windows_link::link!("advapi32.dll" "system" fn InitiateShutdownW(lpmachinename : windows_sys::core::PCWSTR, lpmessage : windows_sys::core::PCWSTR, dwgraceperiod : u32, dwshutdownflags : u32, dwreason : u32) -> u32);
windows_link::link!("advapi32.dll" "system" fn InitiateSystemShutdownA(lpmachinename : windows_sys::core::PCSTR, lpmessage : windows_sys::core::PCSTR, dwtimeout : u32, bforceappsclosed : windows_sys::core::BOOL, brebootaftershutdown : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn InitiateSystemShutdownExA(lpmachinename : windows_sys::core::PCSTR, lpmessage : windows_sys::core::PCSTR, dwtimeout : u32, bforceappsclosed : windows_sys::core::BOOL, brebootaftershutdown : windows_sys::core::BOOL, dwreason : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn InitiateSystemShutdownExW(lpmachinename : windows_sys::core::PCWSTR, lpmessage : windows_sys::core::PCWSTR, dwtimeout : u32, bforceappsclosed : windows_sys::core::BOOL, brebootaftershutdown : windows_sys::core::BOOL, dwreason : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn InitiateSystemShutdownW(lpmachinename : windows_sys::core::PCWSTR, lpmessage : windows_sys::core::PCWSTR, dwtimeout : u32, bforceappsclosed : windows_sys::core::BOOL, brebootaftershutdown : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCloseKey(hkey : super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegConnectRegistryA(lpmachinename : windows_sys::core::PCSTR, hkey : super::minwindef::HKEY, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegConnectRegistryExA(lpmachinename : windows_sys::core::PCSTR, hkey : super::minwindef::HKEY, flags : u32, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegConnectRegistryExW(lpmachinename : windows_sys::core::PCWSTR, hkey : super::minwindef::HKEY, flags : u32, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegConnectRegistryW(lpmachinename : windows_sys::core::PCWSTR, hkey : super::minwindef::HKEY, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCopyTreeA(hkeysrc : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, hkeydest : super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCopyTreeW(hkeysrc : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, hkeydest : super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyExA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, reserved : u32, lpclass : windows_sys::core::PCSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyExW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, reserved : u32, lpclass : windows_sys::core::PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyTransactedA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, reserved : u32, lpclass : windows_sys::core::PCSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32, htransaction : super::winnt::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyTransactedW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, reserved : u32, lpclass : windows_sys::core::PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32, htransaction : super::winnt::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyExA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, samdesired : REGSAM, reserved : u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyExW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, samdesired : REGSAM, reserved : u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, samdesired : REGSAM, reserved : u32, htransaction : super::winnt::HANDLE, pextendedparameter : *const core::ffi::c_void) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, samdesired : REGSAM, reserved : u32, htransaction : super::winnt::HANDLE, pextendedparameter : *const core::ffi::c_void) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, lpvaluename : windows_sys::core::PCSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpvaluename : windows_sys::core::PCWSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteTreeA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteTreeW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteValueA(hkey : super::minwindef::HKEY, lpvaluename : windows_sys::core::PCSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDeleteValueW(hkey : super::minwindef::HKEY, lpvaluename : windows_sys::core::PCWSTR) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegDisablePredefinedCache() -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegDisablePredefinedCacheEx() -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegDisableReflectionKey(hbase : super::minwindef::HKEY) -> i32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnableReflectionKey(hbase : super::minwindef::HKEY) -> i32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyA(hkey : super::minwindef::HKEY, dwindex : u32, lpname : windows_sys::core::PSTR, cchname : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyExA(hkey : super::minwindef::HKEY, dwindex : u32, lpname : windows_sys::core::PSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : windows_sys::core::PSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyExW(hkey : super::minwindef::HKEY, dwindex : u32, lpname : windows_sys::core::PWSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : windows_sys::core::PWSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyW(hkey : super::minwindef::HKEY, dwindex : u32, lpname : windows_sys::core::PWSTR, cchname : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumValueA(hkey : super::minwindef::HKEY, dwindex : u32, lpvaluename : windows_sys::core::PSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegEnumValueW(hkey : super::minwindef::HKEY, dwindex : u32, lpvaluename : windows_sys::core::PWSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegFlushKey(hkey : super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegGetKeySecurity(hkey : super::minwindef::HKEY, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegGetValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, lpvalue : windows_sys::core::PCSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegGetValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PCWSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegLoadAppKeyA(lpfile : windows_sys::core::PCSTR, phkresult : *mut super::minwindef::HKEY, samdesired : REGSAM, dwoptions : u32, reserved : u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegLoadAppKeyW(lpfile : windows_sys::core::PCWSTR, phkresult : *mut super::minwindef::HKEY, samdesired : REGSAM, dwoptions : u32, reserved : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegLoadKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, lpfile : windows_sys::core::PCSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegLoadKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpfile : windows_sys::core::PCWSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegLoadMUIStringA(hkey : super::minwindef::HKEY, pszvalue : windows_sys::core::PCSTR, pszoutbuf : windows_sys::core::PSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : windows_sys::core::PCSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegLoadMUIStringW(hkey : super::minwindef::HKEY, pszvalue : windows_sys::core::PCWSTR, pszoutbuf : windows_sys::core::PWSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : windows_sys::core::PCWSTR) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegNotifyChangeKeyValue(hkey : super::minwindef::HKEY, bwatchsubtree : windows_sys::core::BOOL, dwnotifyfilter : u32, hevent : super::winnt::HANDLE, fasynchronous : windows_sys::core::BOOL) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenCurrentUser(samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyExA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyTransactedA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY, htransaction : super::winnt::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyTransactedW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY, htransaction : super::winnt::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegOpenUserClassesRoot(htoken : super::winnt::HANDLE, dwoptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegOverridePredefKey(hkey : super::minwindef::HKEY, hnewhkey : super::minwindef::HKEY) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryInfoKeyA(hkey : super::minwindef::HKEY, lpclass : windows_sys::core::PSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryInfoKeyW(hkey : super::minwindef::HKEY, lpclass : windows_sys::core::PWSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryMultipleValuesA(hkey : super::minwindef::HKEY, val_list : *mut VALENTA, num_vals : u32, lpvaluebuf : windows_sys::core::PSTR, ldwtotsize : *mut u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryMultipleValuesW(hkey : super::minwindef::HKEY, val_list : *mut VALENTW, num_vals : u32, lpvaluebuf : windows_sys::core::PWSTR, ldwtotsize : *mut u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryReflectionKey(hbase : super::minwindef::HKEY, bisreflectiondisabled : *mut windows_sys::core::BOOL) -> i32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, lpdata : windows_sys::core::PSTR, lpcbdata : *mut i32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryValueExA(hkey : super::minwindef::HKEY, lpvaluename : windows_sys::core::PCSTR, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryValueExW(hkey : super::minwindef::HKEY, lpvaluename : windows_sys::core::PCWSTR, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegQueryValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpdata : windows_sys::core::PWSTR, lpcbdata : *mut i32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegRenameKey(hkey : super::minwindef::HKEY, lpsubkeyname : windows_sys::core::PCWSTR, lpnewkeyname : windows_sys::core::PCWSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegReplaceKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, lpnewfile : windows_sys::core::PCSTR, lpoldfile : windows_sys::core::PCSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegReplaceKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpnewfile : windows_sys::core::PCWSTR, lpoldfile : windows_sys::core::PCWSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegRestoreKeyA(hkey : super::minwindef::HKEY, lpfile : windows_sys::core::PCSTR, dwflags : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegRestoreKeyW(hkey : super::minwindef::HKEY, lpfile : windows_sys::core::PCWSTR, dwflags : u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
windows_link::link!("advapi32.dll" "system" fn RegSaveKeyA(hkey : super::minwindef::HKEY, lpfile : windows_sys::core::PCSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
windows_link::link!("advapi32.dll" "system" fn RegSaveKeyExA(hkey : super::minwindef::HKEY, lpfile : windows_sys::core::PCSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, flags : u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
windows_link::link!("advapi32.dll" "system" fn RegSaveKeyExW(hkey : super::minwindef::HKEY, lpfile : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, flags : u32) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
windows_link::link!("advapi32.dll" "system" fn RegSaveKeyW(hkey : super::minwindef::HKEY, lpfile : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> WIN32_ERROR);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RegSetKeySecurity(hkey : super::minwindef::HKEY, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetKeyValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, lpvaluename : windows_sys::core::PCSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetKeyValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, lpvaluename : windows_sys::core::PCWSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR, dwtype : u32, lpdata : windows_sys::core::PCSTR, cbdata : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetValueExA(hkey : super::minwindef::HKEY, lpvaluename : windows_sys::core::PCSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetValueExW(hkey : super::minwindef::HKEY, lpvaluename : windows_sys::core::PCWSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegSetValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR, dwtype : u32, lpdata : windows_sys::core::PCWSTR, cbdata : u32) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegUnLoadKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn RegUnLoadKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_sys::core::PCWSTR) -> WIN32_ERROR);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_CLASSES_ROOT: super::minwindef::HKEY = -2147483648 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_CURRENT_CONFIG: super::minwindef::HKEY = -2147483643 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_CURRENT_USER: super::minwindef::HKEY = -2147483647 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: super::minwindef::HKEY = -2147483641 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_DYN_DATA: super::minwindef::HKEY = -2147483642 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_LOCAL_MACHINE: super::minwindef::HKEY = -2147483646 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_PERFORMANCE_DATA: super::minwindef::HKEY = -2147483644 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_PERFORMANCE_NLSTEXT: super::minwindef::HKEY = -2147483552 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_PERFORMANCE_TEXT: super::minwindef::HKEY = -2147483568 as _;
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_USERS: super::minwindef::HKEY = -2147483645 as _;
pub const MAX_SHUTDOWN_TIMEOUT: u32 = 315360000;
pub type PPROVIDER = *mut REG_PROVIDER;
pub type PPVALUE = PPVALUEA;
pub type PPVALUEA = *mut PVALUEA;
pub type PPVALUEW = *mut PVALUEW;
pub type PQUERYHANDLER = Option<unsafe extern "C" fn(keycontext: *mut core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
pub const PROVIDER_KEEPS_VALUE_LENGTH: u32 = 1;
pub type PVALCONTEXT = *mut val_context;
pub type PVALENT = PVALENTA;
pub type PVALENTA = *mut VALENTA;
pub type PVALENTW = *mut VALENTW;
pub type PVALUE = PVALUEA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PVALUEA {
    pub pv_valuename: windows_sys::core::PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut core::ffi::c_void,
    pub pv_type: u32,
}
impl Default for PVALUEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PVALUEW {
    pub pv_valuename: windows_sys::core::PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut core::ffi::c_void,
    pub pv_type: u32,
}
impl Default for PVALUEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type QUERYHANDLER = Option<unsafe extern "C" fn(keycontext: *mut core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
pub const REASON_HWINSTALL: u32 = 65538;
pub const REASON_LEGACY_API: i32 = -2147024896;
pub const REASON_OTHER: u32 = 0;
pub const REASON_PLANNED_FLAG: i32 = -2147483648;
pub const REASON_SERVICEHANG: u32 = 196613;
pub const REASON_SWHWRECONF: u32 = 196612;
pub const REASON_SWINSTALL: u32 = 196610;
pub const REASON_UNKNOWN: u32 = 255;
pub const REASON_UNSTABLE: u32 = 327686;
#[cfg(feature = "Win32_winnt")]
pub type REGSAM = super::winnt::ACCESS_MASK;
pub const REG_ALLOW_TRANSPORT_FALLBACK: u32 = 2;
pub const REG_ALLOW_UNSECURE_CONNECTION: u32 = 4;
pub const REG_MUI_STRING_TRUNCATE: u32 = 1;
pub const REG_PROCESS_APPKEY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_PROVIDER {
    pub pi_R0_1val: PQUERYHANDLER,
    pub pi_R0_allvals: PQUERYHANDLER,
    pub pi_R3_1val: PQUERYHANDLER,
    pub pi_R3_allvals: PQUERYHANDLER,
    pub pi_flags: u32,
    pub pi_key_context: *mut core::ffi::c_void,
}
impl Default for REG_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REG_SECURE_CONNECTION: u32 = 1;
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2;
pub const RRF_NOEXPAND: u32 = 268435456;
pub const RRF_RT_ANY: u32 = 65535;
pub const RRF_RT_DWORD: u32 = 24;
pub const RRF_RT_QWORD: u32 = 72;
pub const RRF_RT_REG_BINARY: u32 = 8;
pub const RRF_RT_REG_DWORD: u32 = 16;
pub const RRF_RT_REG_EXPAND_SZ: u32 = 4;
pub const RRF_RT_REG_MULTI_SZ: u32 = 32;
pub const RRF_RT_REG_NONE: u32 = 1;
pub const RRF_RT_REG_QWORD: u32 = 64;
pub const RRF_RT_REG_SZ: u32 = 2;
pub const RRF_SUBKEY_WOW6432KEY: u32 = 131072;
pub const RRF_SUBKEY_WOW6464KEY: u32 = 65536;
pub const RRF_WOW64_MASK: u32 = 196608;
pub const RRF_ZEROONFAILURE: u32 = 536870912;
pub const SHUTDOWN_ARSO: u32 = 8192;
pub const SHUTDOWN_CHECK_SAFE_FOR_SERVER: u32 = 16384;
pub const SHUTDOWN_FORCE_OTHERS: u32 = 1;
pub const SHUTDOWN_FORCE_SELF: u32 = 2;
pub const SHUTDOWN_GRACE_OVERRIDE: u32 = 32;
pub const SHUTDOWN_HYBRID: u32 = 512;
pub const SHUTDOWN_INSTALL_UPDATES: u32 = 64;
pub const SHUTDOWN_MOBILE_UI: u32 = 4096;
pub const SHUTDOWN_NOREBOOT: u32 = 16;
pub const SHUTDOWN_POWEROFF: u32 = 8;
pub const SHUTDOWN_RESTART: u32 = 4;
pub const SHUTDOWN_RESTARTAPPS: u32 = 128;
pub const SHUTDOWN_RESTART_BOOTOPTIONS: u32 = 1024;
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: u32 = 256;
pub const SHUTDOWN_SOFT_REBOOT: u32 = 2048;
pub const SHUTDOWN_SYSTEM_INITIATED: u32 = 65536;
pub const SHUTDOWN_UPDATE_POWEROFF: u32 = 131072;
pub const SHUTDOWN_VAIL_CONTAINER: u32 = 32768;
pub type VALENT = VALENTA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VALENTA {
    pub ve_valuename: windows_sys::core::PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
}
impl Default for VALENTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VALENTW {
    pub ve_valuename: windows_sys::core::PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
}
impl Default for VALENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIN31_CLASS: u32 = 0;
pub type WIN32_ERROR = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct val_context {
    pub valuelen: i32,
    pub value_context: *mut core::ffi::c_void,
    pub val_buff_ptr: *mut core::ffi::c_void,
}
impl Default for val_context {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
