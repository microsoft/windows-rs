windows_link::link!("offreg.dll" "system" fn ORCloseHive(handle : ORHKEY) -> u32);
windows_link::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : ORHKEY) -> u32);
windows_link::link!("offreg.dll" "system" fn ORCreateHive(horkey : PORHKEY) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORCreateKey(keyhandle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, lpclass : windows_sys::core::PCWSTR, dwoptions : u32, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, phkresult : PORHKEY, pdwdisposition : super::PDWORD) -> u32);
windows_link::link!("offreg.dll" "system" fn ORDeleteKey(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("offreg.dll" "system" fn ORDeleteValue(handle : ORHKEY, lpvaluename : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn OREnumKey(handle : ORHKEY, dwindex : u32, lpname : windows_sys::core::PWSTR, lpcname : super::PDWORD, lpclass : windows_sys::core::PWSTR, lpcclass : super::PDWORD, lpftlastwritetime : super::PFILETIME) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn OREnumValue(handle : ORHKEY, dwindex : u32, lpvaluename : windows_sys::core::PWSTR, lpcvaluename : super::PDWORD, lptype : super::PDWORD, lpdata : super::PBYTE, lpcbdata : super::PDWORD) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORGetKeySecurity(handle : ORHKEY, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : super::PDWORD) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn ORGetValue(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PCWSTR, pdwtype : super::PDWORD, pvdata : *mut core::ffi::c_void, pcbdata : super::PDWORD) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn ORGetVersion(pdwmajorversion : super::PDWORD, pdwminorversion : super::PDWORD));
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn ORGetVirtualFlags(handle : ORHKEY, pdwflags : super::PDWORD) -> u32);
windows_link::link!("offreg.dll" "system" fn ORMergeHives(hivehandles : *const ORHKEY, hivecount : u32, phkresult : PORHKEY) -> u32);
windows_link::link!("offreg.dll" "system" fn OROpenHive(filepath : windows_sys::core::PCWSTR, horkey : PORHKEY) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "system" fn OROpenHiveByHandle(filehandle : super::HANDLE, horkey : PORHKEY) -> u32);
windows_link::link!("offreg.dll" "system" fn OROpenKey(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, phkresult : PORHKEY) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn ORQueryInfoKey(handle : ORHKEY, lpclass : windows_sys::core::PWSTR, lpcclass : super::PDWORD, lpcsubkeys : super::PDWORD, lpcmaxsubkeylen : super::PDWORD, lpcmaxclasslen : super::PDWORD, lpcvalues : super::PDWORD, lpcmaxvaluenamelen : super::PDWORD, lpcmaxvaluelen : super::PDWORD, lpcbsecuritydescriptor : super::PDWORD, lpftlastwritetime : super::PFILETIME) -> u32);
windows_link::link!("offreg.dll" "system" fn ORRenameKey(handle : ORHKEY, lpnewname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("offreg.dll" "system" fn ORSaveHive(horkey : ORHKEY, hivepath : windows_sys::core::PCWSTR, osmajorversion : u32, osminorversion : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "system" fn ORSetKeySecurity(handle : ORHKEY, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> u32);
windows_link::link!("offreg.dll" "system" fn ORSetValue(handle : ORHKEY, lpvaluename : windows_sys::core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> u32);
windows_link::link!("offreg.dll" "system" fn ORSetVirtualFlags(handle : ORHKEY, dwflags : u32) -> u32);
pub type ORHKEY = *mut core::ffi::c_void;
pub type PORHKEY = *mut ORHKEY;
