windows_link::link!("offreg.dll" "C" fn ORCloseHive(handle : ORHKEY) -> u32);
windows_link::link!("offreg.dll" "C" fn ORCloseKey(keyhandle : ORHKEY) -> u32);
windows_link::link!("offreg.dll" "C" fn ORCreateHive(horkey : *mut ORHKEY) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "C" fn ORCreateKey(keyhandle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, lpclass : windows_sys::core::PCWSTR, dwoptions : u32, psecuritydescriptor : super::super::Win32::winnt::PSECURITY_DESCRIPTOR, phkresult : *mut ORHKEY, pdwdisposition : *mut u32) -> u32);
windows_link::link!("offreg.dll" "C" fn ORDeleteKey(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("offreg.dll" "C" fn ORDeleteValue(handle : ORHKEY, lpvaluename : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "C" fn OREnumKey(handle : ORHKEY, dwindex : u32, lpname : windows_sys::core::PWSTR, lpcname : *mut u32, lpclass : windows_sys::core::PWSTR, lpcclass : *mut u32, lpftlastwritetime : *mut super::super::Win32::minwindef::FILETIME) -> u32);
windows_link::link!("offreg.dll" "C" fn OREnumValue(handle : ORHKEY, dwindex : u32, lpvaluename : windows_sys::core::PWSTR, lpcvaluename : *mut u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "C" fn ORGetKeySecurity(handle : ORHKEY, securityinformation : super::super::Win32::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::super::Win32::winnt::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
windows_link::link!("offreg.dll" "C" fn ORGetValue(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PCWSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> u32);
windows_link::link!("offreg.dll" "C" fn ORGetVersion(pdwmajorversion : *mut u32, pdwminorversion : *mut u32));
windows_link::link!("offreg.dll" "C" fn ORGetVirtualFlags(handle : ORHKEY, pdwflags : *mut u32) -> u32);
windows_link::link!("offreg.dll" "C" fn ORMergeHives(hivehandles : *const ORHKEY, hivecount : u32, phkresult : *mut ORHKEY) -> u32);
windows_link::link!("offreg.dll" "C" fn OROpenHive(filepath : windows_sys::core::PCWSTR, horkey : *mut ORHKEY) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "C" fn OROpenHiveByHandle(filehandle : super::super::Win32::winnt::HANDLE, horkey : *mut ORHKEY) -> u32);
windows_link::link!("offreg.dll" "C" fn OROpenKey(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, phkresult : *mut ORHKEY) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "C" fn ORQueryInfoKey(handle : ORHKEY, lpclass : windows_sys::core::PWSTR, lpcclass : *mut u32, lpcsubkeys : *mut u32, lpcmaxsubkeylen : *mut u32, lpcmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcmaxvaluenamelen : *mut u32, lpcmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::Win32::minwindef::FILETIME) -> u32);
windows_link::link!("offreg.dll" "C" fn ORRenameKey(handle : ORHKEY, lpnewname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("offreg.dll" "C" fn ORSaveHive(horkey : ORHKEY, hivepath : windows_sys::core::PCWSTR, osmajorversion : u32, osminorversion : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("offreg.dll" "C" fn ORSetKeySecurity(handle : ORHKEY, securityinformation : super::super::Win32::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::super::Win32::winnt::PSECURITY_DESCRIPTOR) -> u32);
windows_link::link!("offreg.dll" "C" fn ORSetValue(handle : ORHKEY, lpvaluename : windows_sys::core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> u32);
windows_link::link!("offreg.dll" "C" fn ORSetVirtualFlags(handle : ORHKEY, dwflags : u32) -> u32);
pub type ORHKEY = *mut core::ffi::c_void;
pub type PORHKEY = *mut ORHKEY;
