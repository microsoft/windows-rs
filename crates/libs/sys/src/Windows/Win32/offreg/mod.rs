#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORCloseHive(handle : ORHKEY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : ORHKEY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORCreateHive(horkey : PORHKEY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORCreateKey(keyhandle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, lpclass : windows_sys::core::PCWSTR, dwoptions : super::DWORD, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, phkresult : PORHKEY, pdwdisposition : super::PDWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORDeleteKey(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORDeleteValue(handle : ORHKEY, lpvaluename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn OREnumKey(handle : ORHKEY, dwindex : super::DWORD, lpname : windows_sys::core::PWSTR, lpcname : super::PDWORD, lpclass : windows_sys::core::PWSTR, lpcclass : super::PDWORD, lpftlastwritetime : super::PFILETIME) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn OREnumValue(handle : ORHKEY, dwindex : super::DWORD, lpvaluename : windows_sys::core::PWSTR, lpcvaluename : super::PDWORD, lptype : super::PDWORD, lpdata : super::PBYTE, lpcbdata : super::PDWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORGetKeySecurity(handle : ORHKEY, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : super::PDWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORGetValue(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PCWSTR, pdwtype : super::PDWORD, pvdata : super::PVOID, pcbdata : super::PDWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
windows_link::link!("offreg.dll" "system" fn ORGetVersion(pdwmajorversion : super::PDWORD, pdwminorversion : super::PDWORD));
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORGetVirtualFlags(handle : ORHKEY, pdwflags : super::PDWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORMergeHives(hivehandles : *const ORHKEY, hivecount : super::ULONG, phkresult : PORHKEY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn OROpenHive(filepath : windows_sys::core::PCWSTR, horkey : PORHKEY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn OROpenHiveByHandle(filehandle : super::HANDLE, horkey : PORHKEY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn OROpenKey(handle : ORHKEY, lpsubkey : windows_sys::core::PCWSTR, phkresult : PORHKEY) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORQueryInfoKey(handle : ORHKEY, lpclass : windows_sys::core::PWSTR, lpcclass : super::PDWORD, lpcsubkeys : super::PDWORD, lpcmaxsubkeylen : super::PDWORD, lpcmaxclasslen : super::PDWORD, lpcvalues : super::PDWORD, lpcmaxvaluenamelen : super::PDWORD, lpcmaxvaluelen : super::PDWORD, lpcbsecuritydescriptor : super::PDWORD, lpftlastwritetime : super::PFILETIME) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORRenameKey(handle : ORHKEY, lpnewname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORSaveHive(horkey : ORHKEY, hivepath : windows_sys::core::PCWSTR, osmajorversion : super::DWORD, osminorversion : super::DWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORSetKeySecurity(handle : ORHKEY, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORSetValue(handle : ORHKEY, lpvaluename : windows_sys::core::PCWSTR, dwtype : super::DWORD, lpdata : *const super::BYTE, cbdata : super::DWORD) -> super::DWORD);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("offreg.dll" "system" fn ORSetVirtualFlags(handle : ORHKEY, dwflags : super::DWORD) -> super::DWORD);
#[cfg(feature = "winnt")]
pub type ORHKEY = super::PVOID;
#[cfg(feature = "winnt")]
pub type PORHKEY = *mut ORHKEY;
