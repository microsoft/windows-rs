windows_link::link!("offreg.dll" "system" fn ORCloseHive(handle : ORHKEY) -> u32);
windows_link::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : ORHKEY) -> u32);
windows_link::link!("offreg.dll" "system" fn ORCreateHive(horkey : *mut ORHKEY) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "system" fn ORCreateKey(keyhandle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, lpclass : windows_sys::core::PCWSTR, dwoptions : u32, psecuritydescriptor : super::super::Win32::PSECURITY_DESCRIPTOR, phkresult : *mut ORHKEY, pdwdisposition : *mut u32) -> u32);
windows_link::link!("offreg.dll" "system" fn ORDeleteKey(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("offreg.dll" "system" fn ORDeleteValue(handle : ORHKEY, lpvaluename : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn OREnumKey(handle : ORHKEY, dwindex : u32, lpname : windows_sys::core::PWSTR, lpcname : *mut u32, lpclass : windows_sys::core::PWSTR, lpcclass : *mut u32, lpftlastwritetime : *mut super::super::Win32::FILETIME) -> u32);
windows_link::link!("offreg.dll" "system" fn OREnumValue(handle : ORHKEY, dwindex : u32, lpvaluename : windows_sys::core::PWSTR, lpcvaluename : *mut u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "system" fn ORGetKeySecurity(handle : ORHKEY, securityinformation : super::super::Win32::SECURITY_INFORMATION, psecuritydescriptor : super::super::Win32::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
windows_link::link!("offreg.dll" "system" fn ORGetValue(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PCWSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> u32);
windows_link::link!("offreg.dll" "system" fn ORGetVersion(pdwmajorversion : *mut u32, pdwminorversion : *mut u32));
windows_link::link!("offreg.dll" "system" fn ORGetVirtualFlags(handle : ORHKEY, pdwflags : *mut u32) -> u32);
windows_link::link!("offreg.dll" "system" fn ORMergeHives(hivehandles : *const ORHKEY, hivecount : u32, phkresult : *mut ORHKEY) -> u32);
windows_link::link!("offreg.dll" "system" fn OROpenHive(filepath : windows_sys::core::PCWSTR, horkey : *mut ORHKEY) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "system" fn OROpenHiveByHandle(filehandle : super::super::Win32::HANDLE, horkey : *mut ORHKEY) -> u32);
windows_link::link!("offreg.dll" "system" fn OROpenKey(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, phkresult : *mut ORHKEY) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn ORQueryInfoKey(handle : ORHKEY, lpclass : windows_sys::core::PWSTR, lpcclass : *mut u32, lpcsubkeys : *mut u32, lpcmaxsubkeylen : *mut u32, lpcmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcmaxvaluenamelen : *mut u32, lpcmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::Win32::FILETIME) -> u32);
windows_link::link!("offreg.dll" "system" fn ORRenameKey(handle : ORHKEY, lpnewname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("offreg.dll" "system" fn ORSaveHive(horkey : ORHKEY, hivepath : windows_sys::core::PCWSTR, osmajorversion : u32, osminorversion : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "system" fn ORSetKeySecurity(handle : ORHKEY, securityinformation : super::super::Win32::SECURITY_INFORMATION, psecuritydescriptor : super::super::Win32::PSECURITY_DESCRIPTOR) -> u32);
windows_link::link!("offreg.dll" "system" fn ORSetValue(handle : ORHKEY, lpvaluename : windows_sys::core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> u32);
windows_link::link!("offreg.dll" "system" fn ORSetVirtualFlags(handle : ORHKEY, dwflags : u32) -> u32);
pub type ORHKEY = *mut core::ffi::c_void;
pub type PORHKEY = *mut ORHKEY;
