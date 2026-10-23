#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORCloseHive(handle: ORHKEY) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORCloseHive(handle : ORHKEY) -> super::DWORD);
    unsafe { ORCloseHive(handle) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORCloseKey(keyhandle: ORHKEY) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : ORHKEY) -> super::DWORD);
    unsafe { ORCloseKey(keyhandle) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORCreateHive(horkey: PORHKEY) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORCreateHive(horkey : PORHKEY) -> super::DWORD);
    unsafe { ORCreateHive(horkey as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORCreateKey<P1, P2>(keyhandle: ORHKEY, lpsubkey: P1, lpclass: P2, dwoptions: Option<super::DWORD>, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, phkresult: PORHKEY, pdwdisposition: Option<super::PDWORD>) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORCreateKey(keyhandle : ORHKEY, lpsubkey : windows_core::PCWSTR, lpclass : windows_core::PCWSTR, dwoptions : super::DWORD, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, phkresult : PORHKEY, pdwdisposition : super::PDWORD) -> super::DWORD);
    unsafe { ORCreateKey(keyhandle, lpsubkey.param().abi(), lpclass.param().abi(), dwoptions.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, phkresult as _, pdwdisposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORDeleteKey<P1>(handle: ORHKEY, lpsubkey: P1) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORDeleteKey(handle : ORHKEY, lpsubkey : windows_core::PCWSTR) -> super::DWORD);
    unsafe { ORDeleteKey(handle, lpsubkey.param().abi()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORDeleteValue<P1>(handle: ORHKEY, lpvaluename: P1) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORDeleteValue(handle : ORHKEY, lpvaluename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { ORDeleteValue(handle, lpvaluename.param().abi()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OREnumKey(handle: ORHKEY, dwindex: super::DWORD, lpname: windows_core::PWSTR, lpcname: super::PDWORD, lpclass: Option<windows_core::PWSTR>, lpcclass: Option<super::PDWORD>, lpftlastwritetime: Option<super::PFILETIME>) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn OREnumKey(handle : ORHKEY, dwindex : super::DWORD, lpname : windows_core::PWSTR, lpcname : super::PDWORD, lpclass : windows_core::PWSTR, lpcclass : super::PDWORD, lpftlastwritetime : super::PFILETIME) -> super::DWORD);
    unsafe { OREnumKey(handle, dwindex, lpname, lpcname as _, lpclass.unwrap_or(core::mem::zeroed()) as _, lpcclass.unwrap_or(core::mem::zeroed()) as _, lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OREnumValue(handle: ORHKEY, dwindex: super::DWORD, lpvaluename: windows_core::PWSTR, lpcvaluename: super::PDWORD, lptype: Option<super::PDWORD>, lpdata: Option<super::PBYTE>, lpcbdata: Option<super::PDWORD>) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn OREnumValue(handle : ORHKEY, dwindex : super::DWORD, lpvaluename : windows_core::PWSTR, lpcvaluename : super::PDWORD, lptype : super::PDWORD, lpdata : super::PBYTE, lpcbdata : super::PDWORD) -> super::DWORD);
    unsafe { OREnumValue(handle, dwindex, lpvaluename, lpcvaluename as _, lptype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORGetKeySecurity(handle: ORHKEY, securityinformation: super::SECURITY_INFORMATION, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, lpcbsecuritydescriptor: super::PDWORD) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORGetKeySecurity(handle : ORHKEY, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : super::PDWORD) -> super::DWORD);
    unsafe { ORGetKeySecurity(handle, securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, lpcbsecuritydescriptor as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORGetValue<P1, P2>(handle: ORHKEY, lpsubkey: P1, lpvalue: P2, pdwtype: Option<super::PDWORD>, pvdata: Option<super::PVOID>, pcbdata: Option<super::PDWORD>) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORGetValue(handle : ORHKEY, lpsubkey : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR, pdwtype : super::PDWORD, pvdata : super::PVOID, pcbdata : super::PDWORD) -> super::DWORD);
    unsafe { ORGetValue(handle, lpsubkey.param().abi(), lpvalue.param().abi(), pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ORGetVersion(pdwmajorversion: super::PDWORD, pdwminorversion: super::PDWORD) {
    windows_core::link!("offreg.dll" "system" fn ORGetVersion(pdwmajorversion : super::PDWORD, pdwminorversion : super::PDWORD));
    unsafe { ORGetVersion(pdwmajorversion as _, pdwminorversion as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORGetVirtualFlags(handle: ORHKEY, pdwflags: super::PDWORD) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORGetVirtualFlags(handle : ORHKEY, pdwflags : super::PDWORD) -> super::DWORD);
    unsafe { ORGetVirtualFlags(handle, pdwflags as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORMergeHives(hivehandles: &[ORHKEY], phkresult: PORHKEY) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORMergeHives(hivehandles : *const ORHKEY, hivecount : super::ULONG, phkresult : PORHKEY) -> super::DWORD);
    unsafe { ORMergeHives(hivehandles.as_ptr(), hivehandles.len().try_into().unwrap(), phkresult as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OROpenHive<P0>(filepath: P0, horkey: PORHKEY) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn OROpenHive(filepath : windows_core::PCWSTR, horkey : PORHKEY) -> super::DWORD);
    unsafe { OROpenHive(filepath.param().abi(), horkey as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OROpenHiveByHandle(filehandle: super::HANDLE, horkey: PORHKEY) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn OROpenHiveByHandle(filehandle : super::HANDLE, horkey : PORHKEY) -> super::DWORD);
    unsafe { OROpenHiveByHandle(filehandle, horkey as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OROpenKey<P1>(handle: ORHKEY, lpsubkey: P1, phkresult: PORHKEY) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn OROpenKey(handle : ORHKEY, lpsubkey : windows_core::PCWSTR, phkresult : PORHKEY) -> super::DWORD);
    unsafe { OROpenKey(handle, lpsubkey.param().abi(), phkresult as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORQueryInfoKey(handle: ORHKEY, lpclass: Option<windows_core::PWSTR>, lpcclass: Option<super::PDWORD>, lpcsubkeys: Option<super::PDWORD>, lpcmaxsubkeylen: Option<super::PDWORD>, lpcmaxclasslen: Option<super::PDWORD>, lpcvalues: Option<super::PDWORD>, lpcmaxvaluenamelen: Option<super::PDWORD>, lpcmaxvaluelen: Option<super::PDWORD>, lpcbsecuritydescriptor: Option<super::PDWORD>, lpftlastwritetime: Option<super::PFILETIME>) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORQueryInfoKey(handle : ORHKEY, lpclass : windows_core::PWSTR, lpcclass : super::PDWORD, lpcsubkeys : super::PDWORD, lpcmaxsubkeylen : super::PDWORD, lpcmaxclasslen : super::PDWORD, lpcvalues : super::PDWORD, lpcmaxvaluenamelen : super::PDWORD, lpcmaxvaluelen : super::PDWORD, lpcbsecuritydescriptor : super::PDWORD, lpftlastwritetime : super::PFILETIME) -> super::DWORD);
    unsafe {
        ORQueryInfoKey(
            handle,
            lpclass.unwrap_or(core::mem::zeroed()) as _,
            lpcclass.unwrap_or(core::mem::zeroed()) as _,
            lpcsubkeys.unwrap_or(core::mem::zeroed()) as _,
            lpcmaxsubkeylen.unwrap_or(core::mem::zeroed()) as _,
            lpcmaxclasslen.unwrap_or(core::mem::zeroed()) as _,
            lpcvalues.unwrap_or(core::mem::zeroed()) as _,
            lpcmaxvaluenamelen.unwrap_or(core::mem::zeroed()) as _,
            lpcmaxvaluelen.unwrap_or(core::mem::zeroed()) as _,
            lpcbsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _,
            lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORRenameKey<P1>(handle: ORHKEY, lpnewname: P1) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORRenameKey(handle : ORHKEY, lpnewname : windows_core::PCWSTR) -> super::DWORD);
    unsafe { ORRenameKey(handle, lpnewname.param().abi()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORSaveHive<P1>(horkey: ORHKEY, hivepath: P1, osmajorversion: super::DWORD, osminorversion: super::DWORD) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORSaveHive(horkey : ORHKEY, hivepath : windows_core::PCWSTR, osmajorversion : super::DWORD, osminorversion : super::DWORD) -> super::DWORD);
    unsafe { ORSaveHive(horkey, hivepath.param().abi(), osmajorversion, osminorversion) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORSetKeySecurity(handle: ORHKEY, securityinformation: super::SECURITY_INFORMATION, psecuritydescriptor: super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORSetKeySecurity(handle : ORHKEY, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { ORSetKeySecurity(handle, securityinformation, psecuritydescriptor) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORSetValue<P1>(handle: ORHKEY, lpvaluename: P1, dwtype: super::DWORD, lpdata: Option<&[u8]>) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORSetValue(handle : ORHKEY, lpvaluename : windows_core::PCWSTR, dwtype : super::DWORD, lpdata : *const super::BYTE, cbdata : super::DWORD) -> super::DWORD);
    unsafe { ORSetValue(handle, lpvaluename.param().abi(), dwtype, core::mem::transmute(lpdata.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ORSetVirtualFlags(handle: ORHKEY, dwflags: super::DWORD) -> super::DWORD {
    windows_core::link!("offreg.dll" "system" fn ORSetVirtualFlags(handle : ORHKEY, dwflags : super::DWORD) -> super::DWORD);
    unsafe { ORSetVirtualFlags(handle, dwflags) }
}
#[cfg(feature = "winnt")]
pub type ORHKEY = super::PVOID;
#[cfg(feature = "winnt")]
pub type PORHKEY = *mut ORHKEY;
