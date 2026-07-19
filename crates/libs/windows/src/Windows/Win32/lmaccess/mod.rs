#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn I_NetLogonControl2<P0>(servername: P0, functioncode: u32, querylevel: u32, data: *const u8, buffer: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn I_NetLogonControl2(servername : windows_core::PCWSTR, functioncode : u32, querylevel : u32, data : *const u8, buffer : *mut super::LPBYTE) -> u32);
    unsafe { I_NetLogonControl2(servername.param().abi(), functioncode, querylevel, data, buffer as _) }
}
#[inline]
pub unsafe fn NetAccessAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAccessAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    unsafe { NetAccessAdd(servername.param().abi(), level, buf, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetAccessDel<P0, P1>(servername: P0, resource: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAccessDel(servername : windows_core::PCWSTR, resource : windows_core::PCWSTR) -> u32);
    unsafe { NetAccessDel(servername.param().abi(), resource.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetAccessEnum<P0, P1>(servername: P0, basepath: P1, recursive: u32, level: u32, bufptr: *mut super::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAccessEnum(servername : windows_core::PCWSTR, basepath : windows_core::PCWSTR, recursive : u32, level : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    unsafe { NetAccessEnum(servername.param().abi(), basepath.param().abi(), recursive, level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetAccessGetInfo<P0, P1>(servername: P0, resource: P1, level: u32, bufptr: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAccessGetInfo(servername : windows_core::PCWSTR, resource : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE) -> u32);
    unsafe { NetAccessGetInfo(servername.param().abi(), resource.param().abi(), level, bufptr as _) }
}
#[inline]
pub unsafe fn NetAccessGetUserPerms<P0, P1, P2>(servername: P0, ugname: P1, resource: P2, perms: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAccessGetUserPerms(servername : windows_core::PCWSTR, ugname : windows_core::PCWSTR, resource : windows_core::PCWSTR, perms : *mut u32) -> u32);
    unsafe { NetAccessGetUserPerms(servername.param().abi(), ugname.param().abi(), resource.param().abi(), perms as _) }
}
#[inline]
pub unsafe fn NetAccessSetInfo<P0, P1>(servername: P0, resource: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAccessSetInfo(servername : windows_core::PCWSTR, resource : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    unsafe { NetAccessSetInfo(servername.param().abi(), resource.param().abi(), level, buf, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetAddServiceAccount<P0, P1, P2>(servername: P0, accountname: P1, password: P2, flags: u32) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "C" fn NetAddServiceAccount(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, password : windows_core::PCWSTR, flags : u32) -> windows_core::NTSTATUS);
    unsafe { NetAddServiceAccount(servername.param().abi(), accountname.param().abi(), password.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetEnumerateServiceAccounts<P0>(servername: P0, flags: u32, accountscount: *mut u32, accounts: *mut super::PZPWSTR) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "C" fn NetEnumerateServiceAccounts(servername : windows_core::PCWSTR, flags : u32, accountscount : *mut u32, accounts : *mut super::PZPWSTR) -> windows_core::NTSTATUS);
    unsafe { NetEnumerateServiceAccounts(servername.param().abi(), flags, accountscount as _, accounts as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetGetAnyDCName<P0, P1>(servername: P0, domainname: P1, buffer: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGetAnyDCName(servername : windows_core::PCWSTR, domainname : windows_core::PCWSTR, buffer : *mut super::LPBYTE) -> u32);
    unsafe { NetGetAnyDCName(servername.param().abi(), domainname.param().abi(), buffer as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetGetDCName<P0, P1>(servername: P0, domainname: P1, buffer: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGetDCName(servername : windows_core::PCWSTR, domainname : windows_core::PCWSTR, buffer : *mut super::LPBYTE) -> u32);
    unsafe { NetGetDCName(servername.param().abi(), domainname.param().abi(), buffer as _) }
}
#[inline]
pub unsafe fn NetGetDisplayInformationIndex<P0, P2>(servername: P0, level: u32, prefix: P2, index: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGetDisplayInformationIndex(servername : windows_core::PCWSTR, level : u32, prefix : windows_core::PCWSTR, index : *mut u32) -> u32);
    unsafe { NetGetDisplayInformationIndex(servername.param().abi(), level, prefix.param().abi(), index as _) }
}
#[inline]
pub unsafe fn NetGroupAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    unsafe { NetGroupAdd(servername.param().abi(), level, buf, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetGroupAddUser<P0, P1, P2>(servername: P0, groupname: P1, username: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupAddUser(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, username : windows_core::PCWSTR) -> u32);
    unsafe { NetGroupAddUser(servername.param().abi(), groupname.param().abi(), username.param().abi()) }
}
#[inline]
pub unsafe fn NetGroupDel<P0, P1>(servername: P0, groupname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupDel(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR) -> u32);
    unsafe { NetGroupDel(servername.param().abi(), groupname.param().abi()) }
}
#[inline]
pub unsafe fn NetGroupDelUser<P0, P1, P2>(servername: P0, groupname: P1, username: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupDelUser(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, username : windows_core::PCWSTR) -> u32);
    unsafe { NetGroupDelUser(servername.param().abi(), groupname.param().abi(), username.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetGroupEnum<P0>(servername: P0, level: u32, bufptr: *mut super::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut usize) -> u32);
    unsafe { NetGroupEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetGroupGetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupGetInfo(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE) -> u32);
    unsafe { NetGroupGetInfo(servername.param().abi(), groupname.param().abi(), level, bufptr as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetGroupGetUsers<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut super::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupGetUsers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut usize) -> u32);
    unsafe { NetGroupGetUsers(servername.param().abi(), groupname.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetGroupSetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupSetInfo(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    unsafe { NetGroupSetInfo(servername.param().abi(), groupname.param().abi(), level, buf, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetGroupSetUsers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGroupSetUsers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, totalentries : u32) -> u32);
    unsafe { NetGroupSetUsers(servername.param().abi(), groupname.param().abi(), level, buf, totalentries) }
}
#[inline]
pub unsafe fn NetIsServiceAccount<P0, P1>(servername: P0, accountname: P1, isservice: *mut windows_core::BOOL) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "C" fn NetIsServiceAccount(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, isservice : *mut windows_core::BOOL) -> windows_core::NTSTATUS);
    unsafe { NetIsServiceAccount(servername.param().abi(), accountname.param().abi(), isservice as _) }
}
#[inline]
pub unsafe fn NetIsServiceAccount2<P0, P1>(servername: P0, accountname: P1, isservice: *mut windows_core::BOOL, accounttype: *mut MSA_INFO_ACCOUNT_TYPE) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "C" fn NetIsServiceAccount2(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, isservice : *mut windows_core::BOOL, accounttype : *mut MSA_INFO_ACCOUNT_TYPE) -> windows_core::NTSTATUS);
    unsafe { NetIsServiceAccount2(servername.param().abi(), accountname.param().abi(), isservice as _, accounttype as _) }
}
#[inline]
pub unsafe fn NetLocalGroupAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    unsafe { NetLocalGroupAdd(servername.param().abi(), level, buf, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetLocalGroupAddMember<P0, P1>(servername: P0, groupname: P1, membersid: super::PSID) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupAddMember(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, membersid : super::PSID) -> u32);
    unsafe { NetLocalGroupAddMember(servername.param().abi(), groupname.param().abi(), membersid) }
}
#[inline]
pub unsafe fn NetLocalGroupAddMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupAddMembers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, totalentries : u32) -> u32);
    unsafe { NetLocalGroupAddMembers(servername.param().abi(), groupname.param().abi(), level, buf, totalentries) }
}
#[inline]
pub unsafe fn NetLocalGroupDel<P0, P1>(servername: P0, groupname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupDel(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR) -> u32);
    unsafe { NetLocalGroupDel(servername.param().abi(), groupname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetLocalGroupDelMember<P0, P1>(servername: P0, groupname: P1, membersid: super::PSID) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupDelMember(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, membersid : super::PSID) -> u32);
    unsafe { NetLocalGroupDelMember(servername.param().abi(), groupname.param().abi(), membersid) }
}
#[inline]
pub unsafe fn NetLocalGroupDelMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupDelMembers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, totalentries : u32) -> u32);
    unsafe { NetLocalGroupDelMembers(servername.param().abi(), groupname.param().abi(), level, buf, totalentries) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetLocalGroupEnum<P0>(servername: P0, level: u32, bufptr: *mut super::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut usize) -> u32);
    unsafe { NetLocalGroupEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetLocalGroupGetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupGetInfo(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE) -> u32);
    unsafe { NetLocalGroupGetInfo(servername.param().abi(), groupname.param().abi(), level, bufptr as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetLocalGroupGetMembers<P0, P1>(servername: P0, localgroupname: P1, level: u32, bufptr: *mut super::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupGetMembers(servername : windows_core::PCWSTR, localgroupname : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut usize) -> u32);
    unsafe { NetLocalGroupGetMembers(servername.param().abi(), localgroupname.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetLocalGroupSetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupSetInfo(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    unsafe { NetLocalGroupSetInfo(servername.param().abi(), groupname.param().abi(), level, buf, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetLocalGroupSetMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetLocalGroupSetMembers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, totalentries : u32) -> u32);
    unsafe { NetLocalGroupSetMembers(servername.param().abi(), groupname.param().abi(), level, buf, totalentries) }
}
#[inline]
pub unsafe fn NetQueryDisplayInformation<P0>(servername: P0, level: u32, index: u32, entriesrequested: u32, preferredmaximumlength: u32, returnedentrycount: *mut u32, sortedbuffer: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetQueryDisplayInformation(servername : windows_core::PCWSTR, level : u32, index : u32, entriesrequested : u32, preferredmaximumlength : u32, returnedentrycount : *mut u32, sortedbuffer : *mut *mut core::ffi::c_void) -> u32);
    unsafe { NetQueryDisplayInformation(servername.param().abi(), level, index, entriesrequested, preferredmaximumlength, returnedentrycount as _, sortedbuffer as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetQueryServiceAccount<P0, P1>(servername: P0, accountname: P1, infolevel: u32, buffer: *mut super::PBYTE) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "C" fn NetQueryServiceAccount(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, infolevel : u32, buffer : *mut super::PBYTE) -> windows_core::NTSTATUS);
    unsafe { NetQueryServiceAccount(servername.param().abi(), accountname.param().abi(), infolevel, buffer as _) }
}
#[inline]
pub unsafe fn NetRemoveServiceAccount<P0, P1>(servername: P0, accountname: P1, flags: u32) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "C" fn NetRemoveServiceAccount(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, flags : u32) -> windows_core::NTSTATUS);
    unsafe { NetRemoveServiceAccount(servername.param().abi(), accountname.param().abi(), flags) }
}
#[inline]
pub unsafe fn NetUserAdd<P0>(servername: P0, level: u32, buf: *mut u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserAdd(servername : windows_core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
    unsafe { NetUserAdd(servername.param().abi(), level, buf as _, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetUserChangePassword<P0, P1, P2, P3>(domainname: P0, username: P1, oldpassword: P2, newpassword: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserChangePassword(domainname : windows_core::PCWSTR, username : windows_core::PCWSTR, oldpassword : windows_core::PCWSTR, newpassword : windows_core::PCWSTR) -> u32);
    unsafe { NetUserChangePassword(domainname.param().abi(), username.param().abi(), oldpassword.param().abi(), newpassword.param().abi()) }
}
#[inline]
pub unsafe fn NetUserDel<P0, P1>(servername: P0, username: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserDel(servername : windows_core::PCWSTR, username : windows_core::PCWSTR) -> u32);
    unsafe { NetUserDel(servername.param().abi(), username.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetUserEnum<P0>(servername: P0, level: u32, filter: u32, bufptr: *mut super::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserEnum(servername : windows_core::PCWSTR, level : u32, filter : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    unsafe { NetUserEnum(servername.param().abi(), level, filter, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetUserGetGroups<P0, P1>(servername: P0, username: P1, level: u32, bufptr: *mut super::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserGetGroups(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32) -> u32);
    unsafe { NetUserGetGroups(servername.param().abi(), username.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetUserGetInfo<P0, P1>(servername: P0, username: P1, level: u32, bufptr: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserGetInfo(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE) -> u32);
    unsafe { NetUserGetInfo(servername.param().abi(), username.param().abi(), level, bufptr as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetUserGetLocalGroups<P0, P1>(servername: P0, username: P1, level: u32, flags: u32, bufptr: *mut super::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserGetLocalGroups(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, flags : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32) -> u32);
    unsafe { NetUserGetLocalGroups(servername.param().abi(), username.param().abi(), level, flags, bufptr as _, prefmaxlen, entriesread as _, totalentries as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetUserModalsGet<P0>(servername: P0, level: u32, bufptr: *mut super::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserModalsGet(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE) -> u32);
    unsafe { NetUserModalsGet(servername.param().abi(), level, bufptr as _) }
}
#[inline]
pub unsafe fn NetUserModalsSet<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserModalsSet(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    unsafe { NetUserModalsSet(servername.param().abi(), level, buf, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetUserSetGroups<P0, P1>(servername: P0, username: P1, level: u32, buf: *const u8, num_entries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserSetGroups(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, buf : *const u8, num_entries : u32) -> u32);
    unsafe { NetUserSetGroups(servername.param().abi(), username.param().abi(), level, buf, num_entries) }
}
#[inline]
pub unsafe fn NetUserSetInfo<P0, P1>(servername: P0, username: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUserSetInfo(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    unsafe { NetUserSetInfo(servername.param().abi(), username.param().abi(), level, buf, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetValidatePasswordPolicy<P0>(servername: P0, qualifier: *mut core::ffi::c_void, validationtype: NET_VALIDATE_PASSWORD_TYPE, inputarg: *mut core::ffi::c_void, outputarg: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetValidatePasswordPolicy(servername : windows_core::PCWSTR, qualifier : *mut core::ffi::c_void, validationtype : NET_VALIDATE_PASSWORD_TYPE, inputarg : *mut core::ffi::c_void, outputarg : *mut *mut core::ffi::c_void) -> u32);
    unsafe { NetValidatePasswordPolicy(servername.param().abi(), qualifier as _, validationtype, inputarg as _, outputarg as _) }
}
#[inline]
pub unsafe fn NetValidatePasswordPolicyFree(outputarg: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("netapi32.dll" "system" fn NetValidatePasswordPolicyFree(outputarg : *mut *mut core::ffi::c_void) -> u32);
    unsafe { NetValidatePasswordPolicyFree(outputarg as _) }
}
pub const ACCESS_ACCESS_LIST_INFOLEVEL: u32 = 1004;
pub const ACCESS_ACCESS_LIST_PARMNUM: u32 = 4;
pub const ACCESS_ALL: u32 = 127;
pub const ACCESS_ATRIB: u32 = 32;
pub const ACCESS_ATTR_INFOLEVEL: u32 = 1002;
pub const ACCESS_ATTR_PARMNUM: u32 = 2;
pub const ACCESS_AUDIT: u32 = 1;
pub const ACCESS_COUNT_INFOLEVEL: u32 = 1003;
pub const ACCESS_COUNT_PARMNUM: u32 = 3;
pub const ACCESS_CREATE: u32 = 4;
pub const ACCESS_DELETE: u32 = 16;
pub const ACCESS_EXEC: u32 = 8;
pub const ACCESS_FAIL_ACL: u32 = 2048;
pub const ACCESS_FAIL_DELETE: u32 = 1024;
pub const ACCESS_FAIL_MASK: u32 = 3840;
pub const ACCESS_FAIL_OPEN: u32 = 256;
pub const ACCESS_FAIL_SHIFT: u32 = 4;
pub const ACCESS_FAIL_WRITE: u32 = 512;
pub const ACCESS_GROUP: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCESS_INFO_0 {
    pub acc0_resource_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCESS_INFO_1 {
    pub acc1_resource_name: windows_core::PWSTR,
    pub acc1_attr: u32,
    pub acc1_count: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCESS_INFO_1002 {
    pub acc1002_attr: u32,
}
pub const ACCESS_LETTERS: windows_core::PCSTR = windows_core::s!("RWCXDAP         ");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCESS_LIST {
    pub acl_ugname: windows_core::PWSTR,
    pub acl_access: u32,
}
pub const ACCESS_NONE: u32 = 0;
pub const ACCESS_PERM: u32 = 64;
pub const ACCESS_READ: u32 = 1;
pub const ACCESS_RESOURCE_NAME_INFOLEVEL: u32 = 1001;
pub const ACCESS_RESOURCE_NAME_PARMNUM: u32 = 1;
pub const ACCESS_SUCCESS_ACL: u32 = 128;
pub const ACCESS_SUCCESS_DELETE: u32 = 64;
pub const ACCESS_SUCCESS_MASK: u32 = 240;
pub const ACCESS_SUCCESS_OPEN: u32 = 16;
pub const ACCESS_SUCCESS_WRITE: u32 = 32;
pub const ACCESS_WRITE: u32 = 2;
pub const AF_OP_ACCOUNTS: u32 = 8;
pub const AF_OP_COMM: u32 = 2;
pub const AF_OP_PRINT: u32 = 1;
pub const AF_OP_SERVER: u32 = 4;
pub const AF_SETTABLE_BITS: u32 = 15;
pub const DEF_FORCE_LOGOFF: i32 = -1;
pub const DEF_MAX_BADPW: u32 = 0;
pub const DEF_MAX_PWAGE: i32 = -1;
pub const DEF_MAX_PWHIST: u32 = 8;
pub const DEF_MIN_PWAGE: u32 = 0;
pub const DEF_MIN_PWLEN: u32 = 6;
pub const DEF_PWUNIQUENESS: u32 = 5;
pub const DelegatedManagedServiceAccount: MSA_INFO_ACCOUNT_TYPE = 3;
pub const FILTER_INTERDOMAIN_TRUST_ACCOUNT: u32 = 8;
pub const FILTER_NORMAL_ACCOUNT: u32 = 2;
pub const FILTER_SERVER_TRUST_ACCOUNT: u32 = 32;
pub const FILTER_TEMP_DUPLICATE_ACCOUNT: u32 = 1;
pub const FILTER_WORKSTATION_TRUST_ACCOUNT: u32 = 16;
pub const GROUPIDMASK: u32 = 32768;
pub const GROUP_ALL_INFOLEVEL: u32 = 1000;
pub const GROUP_ALL_PARMNUM: u32 = 0;
pub const GROUP_ATTRIBUTES_INFOLEVEL: u32 = 1003;
pub const GROUP_ATTRIBUTES_PARMNUM: u32 = 3;
pub const GROUP_COMMENT_INFOLEVEL: u32 = 1002;
pub const GROUP_COMMENT_PARMNUM: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_INFO_0 {
    pub grpi0_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_INFO_1 {
    pub grpi1_name: windows_core::PWSTR,
    pub grpi1_comment: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_INFO_1002 {
    pub grpi1002_comment: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_INFO_1005 {
    pub grpi1005_attributes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_INFO_2 {
    pub grpi2_name: windows_core::PWSTR,
    pub grpi2_comment: windows_core::PWSTR,
    pub grpi2_group_id: u32,
    pub grpi2_attributes: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_INFO_3 {
    pub grpi3_name: windows_core::PWSTR,
    pub grpi3_comment: windows_core::PWSTR,
    pub grpi3_group_sid: super::PSID,
    pub grpi3_attributes: u32,
}
pub const GROUP_NAME_INFOLEVEL: u32 = 1001;
pub const GROUP_NAME_PARMNUM: u32 = 1;
pub const GROUP_SPECIALGRP_ADMINS: windows_core::PCWSTR = windows_core::w!("ADMINS");
pub const GROUP_SPECIALGRP_GUESTS: windows_core::PCWSTR = windows_core::w!("GUESTS");
pub const GROUP_SPECIALGRP_LOCAL: windows_core::PCWSTR = windows_core::w!("LOCAL");
pub const GROUP_SPECIALGRP_USERS: windows_core::PCWSTR = windows_core::w!("USERS");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_USERS_INFO_0 {
    pub grui0_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_USERS_INFO_1 {
    pub grui1_name: windows_core::PWSTR,
    pub grui1_attributes: u32,
}
pub const GroupManagedServiceAccount: MSA_INFO_ACCOUNT_TYPE = 2;
pub const LG_INCLUDE_INDIRECT: u32 = 1;
pub const LOCALGROUP_COMMENT_PARMNUM: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCALGROUP_INFO_0 {
    pub lgrpi0_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCALGROUP_INFO_1 {
    pub lgrpi1_name: windows_core::PWSTR,
    pub lgrpi1_comment: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCALGROUP_INFO_1002 {
    pub lgrpi1002_comment: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCALGROUP_MEMBERS_INFO_0 {
    pub lgrmi0_sid: super::PSID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCALGROUP_MEMBERS_INFO_1 {
    pub lgrmi1_sid: super::PSID,
    pub lgrmi1_sidusage: super::SID_NAME_USE,
    pub lgrmi1_name: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCALGROUP_MEMBERS_INFO_2 {
    pub lgrmi2_sid: super::PSID,
    pub lgrmi2_sidusage: super::SID_NAME_USE,
    pub lgrmi2_domainandname: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCALGROUP_MEMBERS_INFO_3 {
    pub lgrmi3_domainandname: windows_core::PWSTR,
}
pub const LOCALGROUP_NAME_PARMNUM: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCALGROUP_USERS_INFO_0 {
    pub lgrui0_name: windows_core::PWSTR,
}
pub type LPACCESS_INFO_0 = *mut ACCESS_INFO_0;
pub type LPACCESS_INFO_1 = *mut ACCESS_INFO_1;
pub type LPACCESS_INFO_1002 = *mut ACCESS_INFO_1002;
pub type LPACCESS_LIST = *mut ACCESS_LIST;
pub type LPGROUP_INFO_0 = *mut GROUP_INFO_0;
pub type LPGROUP_INFO_1 = *mut GROUP_INFO_1;
pub type LPGROUP_INFO_1002 = *mut GROUP_INFO_1002;
pub type LPGROUP_INFO_1005 = *mut GROUP_INFO_1005;
pub type LPGROUP_USERS_INFO_0 = *mut GROUP_USERS_INFO_0;
pub type LPGROUP_USERS_INFO_1 = *mut GROUP_USERS_INFO_1;
pub type LPLOCALGROUP_INFO_0 = *mut LOCALGROUP_INFO_0;
pub type LPLOCALGROUP_INFO_1 = *mut LOCALGROUP_INFO_1;
pub type LPLOCALGROUP_INFO_1002 = *mut LOCALGROUP_INFO_1002;
#[cfg(feature = "winnt")]
pub type LPLOCALGROUP_MEMBERS_INFO_0 = *mut LOCALGROUP_MEMBERS_INFO_0;
#[cfg(feature = "winnt")]
pub type LPLOCALGROUP_MEMBERS_INFO_1 = *mut LOCALGROUP_MEMBERS_INFO_1;
#[cfg(feature = "winnt")]
pub type LPLOCALGROUP_MEMBERS_INFO_2 = *mut LOCALGROUP_MEMBERS_INFO_2;
pub type LPLOCALGROUP_MEMBERS_INFO_3 = *mut LOCALGROUP_MEMBERS_INFO_3;
pub type LPLOCALGROUP_USERS_INFO_0 = *mut LOCALGROUP_USERS_INFO_0;
pub type LPMSA_INFO_0 = *mut MSA_INFO_0;
pub type LPMSA_INFO_1 = *mut MSA_INFO_1;
pub type LPUSER_INFO_0 = *mut USER_INFO_0;
pub type LPUSER_INFO_1 = *mut USER_INFO_1;
pub type LPUSER_INFO_10 = *mut USER_INFO_10;
pub type LPUSER_INFO_1003 = *mut USER_INFO_1003;
pub type LPUSER_INFO_1005 = *mut USER_INFO_1005;
pub type LPUSER_INFO_1006 = *mut USER_INFO_1006;
pub type LPUSER_INFO_1007 = *mut USER_INFO_1007;
pub type LPUSER_INFO_1008 = *mut USER_INFO_1008;
pub type LPUSER_INFO_1009 = *mut USER_INFO_1009;
pub type LPUSER_INFO_1010 = *mut USER_INFO_1010;
pub type LPUSER_INFO_1011 = *mut USER_INFO_1011;
pub type LPUSER_INFO_1012 = *mut USER_INFO_1012;
pub type LPUSER_INFO_1013 = *mut USER_INFO_1013;
pub type LPUSER_INFO_1014 = *mut USER_INFO_1014;
pub type LPUSER_INFO_1017 = *mut USER_INFO_1017;
pub type LPUSER_INFO_1018 = *mut USER_INFO_1018;
#[cfg(feature = "minwindef")]
pub type LPUSER_INFO_1020 = *mut USER_INFO_1020;
pub type LPUSER_INFO_1023 = *mut USER_INFO_1023;
pub type LPUSER_INFO_1024 = *mut USER_INFO_1024;
pub type LPUSER_INFO_1025 = *mut USER_INFO_1025;
pub type LPUSER_INFO_1051 = *mut USER_INFO_1051;
pub type LPUSER_INFO_1052 = *mut USER_INFO_1052;
pub type LPUSER_INFO_1053 = *mut USER_INFO_1053;
#[cfg(feature = "minwindef")]
pub type LPUSER_INFO_11 = *mut USER_INFO_11;
#[cfg(feature = "minwindef")]
pub type LPUSER_INFO_2 = *mut USER_INFO_2;
pub type LPUSER_INFO_20 = *mut USER_INFO_20;
pub type LPUSER_INFO_21 = *mut USER_INFO_21;
#[cfg(feature = "minwindef")]
pub type LPUSER_INFO_22 = *mut USER_INFO_22;
#[cfg(feature = "winnt")]
pub type LPUSER_INFO_23 = *mut USER_INFO_23;
#[cfg(feature = "winnt")]
pub type LPUSER_INFO_24 = *mut USER_INFO_24;
#[cfg(feature = "minwindef")]
pub type LPUSER_INFO_3 = *mut USER_INFO_3;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPUSER_INFO_4 = *mut USER_INFO_4;
pub type LPUSER_MODALS_INFO_0 = *mut USER_MODALS_INFO_0;
pub type LPUSER_MODALS_INFO_1 = *mut USER_MODALS_INFO_1;
pub type LPUSER_MODALS_INFO_1001 = *mut USER_MODALS_INFO_1001;
pub type LPUSER_MODALS_INFO_1002 = *mut USER_MODALS_INFO_1002;
pub type LPUSER_MODALS_INFO_1003 = *mut USER_MODALS_INFO_1003;
pub type LPUSER_MODALS_INFO_1004 = *mut USER_MODALS_INFO_1004;
pub type LPUSER_MODALS_INFO_1005 = *mut USER_MODALS_INFO_1005;
pub type LPUSER_MODALS_INFO_1006 = *mut USER_MODALS_INFO_1006;
pub type LPUSER_MODALS_INFO_1007 = *mut USER_MODALS_INFO_1007;
#[cfg(feature = "winnt")]
pub type LPUSER_MODALS_INFO_2 = *mut USER_MODALS_INFO_2;
pub type LPUSER_MODALS_INFO_3 = *mut USER_MODALS_INFO_3;
pub const MAXPERMENTRIES: u32 = 64;
pub const MAX_PASSWD_LEN: u32 = 256;
pub const MODALS_DOMAIN_ID_INFOLEVEL: u32 = 1009;
pub const MODALS_DOMAIN_ID_PARMNUM: u32 = 9;
pub const MODALS_DOMAIN_NAME_INFOLEVEL: u32 = 1008;
pub const MODALS_DOMAIN_NAME_PARMNUM: u32 = 8;
pub const MODALS_FORCE_LOGOFF_INFOLEVEL: u32 = 1004;
pub const MODALS_FORCE_LOGOFF_PARMNUM: u32 = 4;
pub const MODALS_LOCKOUT_DURATION_PARMNUM: u32 = 10;
pub const MODALS_LOCKOUT_OBSERVATION_WINDOW_PARMNUM: u32 = 11;
pub const MODALS_LOCKOUT_THRESHOLD_PARMNUM: u32 = 12;
pub const MODALS_MAX_PASSWD_AGE_INFOLEVEL: u32 = 1002;
pub const MODALS_MAX_PASSWD_AGE_PARMNUM: u32 = 2;
pub const MODALS_MIN_PASSWD_AGE_INFOLEVEL: u32 = 1003;
pub const MODALS_MIN_PASSWD_AGE_PARMNUM: u32 = 3;
pub const MODALS_MIN_PASSWD_LEN_INFOLEVEL: u32 = 1001;
pub const MODALS_MIN_PASSWD_LEN_PARMNUM: u32 = 1;
pub const MODALS_PASSWD_HIST_LEN_INFOLEVEL: u32 = 1005;
pub const MODALS_PASSWD_HIST_LEN_PARMNUM: u32 = 5;
pub const MODALS_PRIMARY_INFOLEVEL: u32 = 1007;
pub const MODALS_PRIMARY_PARMNUM: u32 = 7;
pub const MODALS_ROLE_INFOLEVEL: u32 = 1006;
pub const MODALS_ROLE_PARMNUM: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSA_INFO_0 {
    pub State: MSA_INFO_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSA_INFO_1 {
    pub State: MSA_INFO_STATE,
    pub AccountType: MSA_INFO_ACCOUNT_TYPE,
}
pub type MSA_INFO_ACCOUNT_TYPE = i32;
pub type MSA_INFO_LEVEL = i32;
pub type MSA_INFO_STATE = i32;
pub const MsaAccountFalse: MSA_INFO_ACCOUNT_TYPE = 0;
pub const MsaInfoCanInstall: MSA_INFO_STATE = 4;
pub const MsaInfoCannotInstall: MSA_INFO_STATE = 3;
pub const MsaInfoInstalled: MSA_INFO_STATE = 5;
pub const MsaInfoLevel0: MSA_INFO_LEVEL = 0;
pub const MsaInfoLevel1: MSA_INFO_LEVEL = 1;
pub const MsaInfoLevelMax: MSA_INFO_LEVEL = 2;
pub const MsaInfoNotExist: MSA_INFO_STATE = 1;
pub const MsaInfoNotService: MSA_INFO_STATE = 2;
pub const NETLOGON_CONTROL_BACKUP_CHANGE_LOG: u32 = 65532;
pub const NETLOGON_CONTROL_BREAKPOINT: u32 = 65535;
pub const NETLOGON_CONTROL_CHANGE_PASSWORD: u32 = 9;
pub const NETLOGON_CONTROL_FIND_USER: u32 = 8;
pub const NETLOGON_CONTROL_FORCE_DNS_REG: u32 = 11;
pub const NETLOGON_CONTROL_PDC_REPLICATE: u32 = 4;
pub const NETLOGON_CONTROL_QUERY: u32 = 1;
pub const NETLOGON_CONTROL_QUERY_DNS_REG: u32 = 12;
pub const NETLOGON_CONTROL_QUERY_ENC_TYPES: u32 = 13;
pub const NETLOGON_CONTROL_REDISCOVER: u32 = 5;
pub const NETLOGON_CONTROL_REPLICATE: u32 = 2;
pub const NETLOGON_CONTROL_SET_DBFLAG: u32 = 65534;
pub const NETLOGON_CONTROL_SYNCHRONIZE: u32 = 3;
pub const NETLOGON_CONTROL_TC_QUERY: u32 = 6;
pub const NETLOGON_CONTROL_TC_VERIFY: u32 = 10;
pub const NETLOGON_CONTROL_TRANSPORT_NOTIFY: u32 = 7;
pub const NETLOGON_CONTROL_TRUNCATE_LOG: u32 = 65533;
pub const NETLOGON_CONTROL_UNLOAD_NETLOGON_DLL: u32 = 65531;
pub const NETLOGON_DNS_UPDATE_FAILURE: u32 = 64;
pub const NETLOGON_FULL_SYNC_REPLICATION: u32 = 4;
pub const NETLOGON_HAS_IP: u32 = 16;
pub const NETLOGON_HAS_TIMESERV: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETLOGON_INFO_1 {
    pub netlog1_flags: u32,
    pub netlog1_pdc_connection_status: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETLOGON_INFO_2 {
    pub netlog2_flags: u32,
    pub netlog2_pdc_connection_status: u32,
    pub netlog2_trusted_dc_name: windows_core::PWSTR,
    pub netlog2_tc_connection_status: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETLOGON_INFO_3 {
    pub netlog3_flags: u32,
    pub netlog3_logon_attempts: u32,
    pub netlog3_reserved1: u32,
    pub netlog3_reserved2: u32,
    pub netlog3_reserved3: u32,
    pub netlog3_reserved4: u32,
    pub netlog3_reserved5: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETLOGON_INFO_4 {
    pub netlog4_trusted_dc_name: windows_core::PWSTR,
    pub netlog4_trusted_domain_name: windows_core::PWSTR,
}
pub const NETLOGON_REDO_NEEDED: u32 = 8;
pub const NETLOGON_REPLICATION_IN_PROGRESS: u32 = 2;
pub const NETLOGON_REPLICATION_NEEDED: u32 = 1;
pub const NETLOGON_VERIFY_STATUS_RETURNED: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_DISPLAY_GROUP {
    pub grpi3_name: windows_core::PWSTR,
    pub grpi3_comment: windows_core::PWSTR,
    pub grpi3_group_id: u32,
    pub grpi3_attributes: u32,
    pub grpi3_next_index: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_DISPLAY_MACHINE {
    pub usri2_name: windows_core::PWSTR,
    pub usri2_comment: windows_core::PWSTR,
    pub usri2_flags: u32,
    pub usri2_user_id: u32,
    pub usri2_next_index: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_DISPLAY_USER {
    pub usri1_name: windows_core::PWSTR,
    pub usri1_comment: windows_core::PWSTR,
    pub usri1_flags: u32,
    pub usri1_full_name: windows_core::PWSTR,
    pub usri1_user_id: u32,
    pub usri1_next_index: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub PasswordMatched: bool,
}
pub const NET_VALIDATE_BAD_PASSWORD_COUNT: u32 = 8;
pub const NET_VALIDATE_BAD_PASSWORD_TIME: u32 = 2;
pub const NET_VALIDATE_LOCKOUT_TIME: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_VALIDATE_OUTPUT_ARG {
    pub ChangedPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ValidationStatus: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: windows_core::PWSTR,
    pub UserAccountName: windows_core::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMatch: bool,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_VALIDATE_PASSWORD_HASH {
    pub Length: u32,
    pub Hash: super::LPBYTE,
}
pub const NET_VALIDATE_PASSWORD_HISTORY: u32 = 32;
pub const NET_VALIDATE_PASSWORD_HISTORY_LENGTH: u32 = 16;
pub const NET_VALIDATE_PASSWORD_LAST_SET: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: windows_core::PWSTR,
    pub UserAccountName: windows_core::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMustChangeAtNextLogon: bool,
    pub ClearLockout: bool,
}
pub type NET_VALIDATE_PASSWORD_TYPE = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_VALIDATE_PERSISTED_FIELDS {
    pub PresentFields: u32,
    pub PasswordLastSet: super::FILETIME,
    pub BadPasswordTime: super::FILETIME,
    pub LockoutTime: super::FILETIME,
    pub BadPasswordCount: u32,
    pub PasswordHistoryLength: u32,
    pub PasswordHistory: PNET_VALIDATE_PASSWORD_HASH,
}
pub const NON_VALIDATED_LOGON: u32 = 3;
pub const NULL_USERSETINFO_PASSWD: windows_core::PCSTR = windows_core::s!("              ");
pub const NetValidateAuthentication: NET_VALIDATE_PASSWORD_TYPE = 1;
pub const NetValidatePasswordChange: NET_VALIDATE_PASSWORD_TYPE = 2;
pub const NetValidatePasswordReset: NET_VALIDATE_PASSWORD_TYPE = 3;
pub const ONE_DAY: u32 = 86400;
pub type PACCESS_INFO_0 = *mut ACCESS_INFO_0;
pub type PACCESS_INFO_1 = *mut ACCESS_INFO_1;
pub type PACCESS_INFO_1002 = *mut ACCESS_INFO_1002;
pub type PACCESS_LIST = *mut ACCESS_LIST;
pub const PASSWORD_EXPIRED: u32 = 2;
pub type PGROUP_INFO_0 = *mut GROUP_INFO_0;
pub type PGROUP_INFO_1 = *mut GROUP_INFO_1;
pub type PGROUP_INFO_1002 = *mut GROUP_INFO_1002;
pub type PGROUP_INFO_1005 = *mut GROUP_INFO_1005;
pub type PGROUP_INFO_2 = *mut GROUP_INFO_2;
#[cfg(feature = "winnt")]
pub type PGROUP_INFO_3 = *mut GROUP_INFO_3;
pub type PGROUP_USERS_INFO_0 = *mut GROUP_USERS_INFO_0;
pub type PGROUP_USERS_INFO_1 = *mut GROUP_USERS_INFO_1;
pub type PLOCALGROUP_INFO_0 = *mut LOCALGROUP_INFO_0;
pub type PLOCALGROUP_INFO_1 = *mut LOCALGROUP_INFO_1;
pub type PLOCALGROUP_INFO_1002 = *mut LOCALGROUP_INFO_1002;
#[cfg(feature = "winnt")]
pub type PLOCALGROUP_MEMBERS_INFO_0 = *mut LOCALGROUP_MEMBERS_INFO_0;
#[cfg(feature = "winnt")]
pub type PLOCALGROUP_MEMBERS_INFO_1 = *mut LOCALGROUP_MEMBERS_INFO_1;
#[cfg(feature = "winnt")]
pub type PLOCALGROUP_MEMBERS_INFO_2 = *mut LOCALGROUP_MEMBERS_INFO_2;
pub type PLOCALGROUP_MEMBERS_INFO_3 = *mut LOCALGROUP_MEMBERS_INFO_3;
pub type PLOCALGROUP_USERS_INFO_0 = *mut LOCALGROUP_USERS_INFO_0;
pub type PMSA_INFO_0 = *mut MSA_INFO_0;
pub type PMSA_INFO_1 = *mut MSA_INFO_1;
pub type PMSA_INFO_ACCOUNT_TYPE = *mut MSA_INFO_ACCOUNT_TYPE;
pub type PMSA_INFO_LEVEL = *mut MSA_INFO_LEVEL;
pub type PMSA_INFO_STATE = *mut MSA_INFO_STATE;
pub type PNETLOGON_INFO_1 = *mut NETLOGON_INFO_1;
pub type PNETLOGON_INFO_2 = *mut NETLOGON_INFO_2;
pub type PNETLOGON_INFO_3 = *mut NETLOGON_INFO_3;
pub type PNETLOGON_INFO_4 = *mut NETLOGON_INFO_4;
pub type PNET_DISPLAY_GROUP = *mut NET_DISPLAY_GROUP;
pub type PNET_DISPLAY_MACHINE = *mut NET_DISPLAY_MACHINE;
pub type PNET_DISPLAY_USER = *mut NET_DISPLAY_USER;
#[cfg(feature = "minwindef")]
pub type PNET_VALIDATE_AUTHENTICATION_INPUT_ARG = *mut NET_VALIDATE_AUTHENTICATION_INPUT_ARG;
#[cfg(feature = "minwindef")]
pub type PNET_VALIDATE_OUTPUT_ARG = *mut NET_VALIDATE_OUTPUT_ARG;
#[cfg(feature = "minwindef")]
pub type PNET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG = *mut NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG;
#[cfg(feature = "minwindef")]
pub type PNET_VALIDATE_PASSWORD_HASH = *mut NET_VALIDATE_PASSWORD_HASH;
#[cfg(feature = "minwindef")]
pub type PNET_VALIDATE_PASSWORD_RESET_INPUT_ARG = *mut NET_VALIDATE_PASSWORD_RESET_INPUT_ARG;
pub type PNET_VALIDATE_PASSWORD_TYPE = *mut NET_VALIDATE_PASSWORD_TYPE;
#[cfg(feature = "minwindef")]
pub type PNET_VALIDATE_PERSISTED_FIELDS = *mut NET_VALIDATE_PERSISTED_FIELDS;
pub type PUSER_INFO_0 = *mut USER_INFO_0;
pub type PUSER_INFO_1 = *mut USER_INFO_1;
pub type PUSER_INFO_10 = *mut USER_INFO_10;
pub type PUSER_INFO_1003 = *mut USER_INFO_1003;
pub type PUSER_INFO_1005 = *mut USER_INFO_1005;
pub type PUSER_INFO_1006 = *mut USER_INFO_1006;
pub type PUSER_INFO_1007 = *mut USER_INFO_1007;
pub type PUSER_INFO_1008 = *mut USER_INFO_1008;
pub type PUSER_INFO_1009 = *mut USER_INFO_1009;
pub type PUSER_INFO_1010 = *mut USER_INFO_1010;
pub type PUSER_INFO_1011 = *mut USER_INFO_1011;
pub type PUSER_INFO_1012 = *mut USER_INFO_1012;
pub type PUSER_INFO_1013 = *mut USER_INFO_1013;
pub type PUSER_INFO_1014 = *mut USER_INFO_1014;
pub type PUSER_INFO_1017 = *mut USER_INFO_1017;
pub type PUSER_INFO_1018 = *mut USER_INFO_1018;
#[cfg(feature = "minwindef")]
pub type PUSER_INFO_1020 = *mut USER_INFO_1020;
pub type PUSER_INFO_1023 = *mut USER_INFO_1023;
pub type PUSER_INFO_1024 = *mut USER_INFO_1024;
pub type PUSER_INFO_1025 = *mut USER_INFO_1025;
pub type PUSER_INFO_1051 = *mut USER_INFO_1051;
pub type PUSER_INFO_1052 = *mut USER_INFO_1052;
pub type PUSER_INFO_1053 = *mut USER_INFO_1053;
#[cfg(feature = "minwindef")]
pub type PUSER_INFO_11 = *mut USER_INFO_11;
#[cfg(feature = "minwindef")]
pub type PUSER_INFO_2 = *mut USER_INFO_2;
pub type PUSER_INFO_20 = *mut USER_INFO_20;
pub type PUSER_INFO_21 = *mut USER_INFO_21;
#[cfg(feature = "minwindef")]
pub type PUSER_INFO_22 = *mut USER_INFO_22;
#[cfg(feature = "winnt")]
pub type PUSER_INFO_23 = *mut USER_INFO_23;
#[cfg(feature = "winnt")]
pub type PUSER_INFO_24 = *mut USER_INFO_24;
#[cfg(feature = "minwindef")]
pub type PUSER_INFO_3 = *mut USER_INFO_3;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PUSER_INFO_4 = *mut USER_INFO_4;
pub type PUSER_MODALS_INFO_0 = *mut USER_MODALS_INFO_0;
pub type PUSER_MODALS_INFO_1 = *mut USER_MODALS_INFO_1;
pub type PUSER_MODALS_INFO_1001 = *mut USER_MODALS_INFO_1001;
pub type PUSER_MODALS_INFO_1002 = *mut USER_MODALS_INFO_1002;
pub type PUSER_MODALS_INFO_1003 = *mut USER_MODALS_INFO_1003;
pub type PUSER_MODALS_INFO_1004 = *mut USER_MODALS_INFO_1004;
pub type PUSER_MODALS_INFO_1005 = *mut USER_MODALS_INFO_1005;
pub type PUSER_MODALS_INFO_1006 = *mut USER_MODALS_INFO_1006;
pub type PUSER_MODALS_INFO_1007 = *mut USER_MODALS_INFO_1007;
#[cfg(feature = "winnt")]
pub type PUSER_MODALS_INFO_2 = *mut USER_MODALS_INFO_2;
pub type PUSER_MODALS_INFO_3 = *mut USER_MODALS_INFO_3;
pub const SERVICE_ACCOUNT_FLAG_ADD_AGAINST_RODC: u32 = 2;
pub const SERVICE_ACCOUNT_FLAG_LINK_TO_HOST_ONLY: u32 = 1;
pub const SERVICE_ACCOUNT_FLAG_REMOVE_OFFLINE: u32 = 2;
pub const SERVICE_ACCOUNT_FLAG_UNLINK_FROM_HOST_ONLY: u32 = 1;
pub const ServiceAccountPasswordGUID: windows_core::GUID = windows_core::GUID::from_u128(0x262e99c9_6160_4871_acec_4e61736b6f21);
pub const StandAloneManagedServiceAccount: MSA_INFO_ACCOUNT_TYPE = 1;
pub const TIMEQ_FOREVER: i32 = -1;
pub const UAS_ROLE_BACKUP: u32 = 2;
pub const UAS_ROLE_MEMBER: u32 = 1;
pub const UAS_ROLE_PRIMARY: u32 = 3;
pub const UAS_ROLE_STANDALONE: u32 = 0;
pub const UF_ACCOUNTDISABLE: u32 = 2;
pub const UF_ACCOUNT_TYPE_MASK: u32 = 15104;
pub const UF_DONT_EXPIRE_PASSWD: u32 = 65536;
pub const UF_DONT_REQUIRE_PREAUTH: u32 = 4194304;
pub const UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: u32 = 128;
pub const UF_HOMEDIR_REQUIRED: u32 = 8;
pub const UF_INTERDOMAIN_TRUST_ACCOUNT: u32 = 2048;
pub const UF_LOCKOUT: u32 = 16;
pub const UF_MACHINE_ACCOUNT_MASK: u32 = 14336;
pub const UF_MNS_LOGON_ACCOUNT: u32 = 131072;
pub const UF_NORMAL_ACCOUNT: u32 = 512;
pub const UF_NOT_DELEGATED: u32 = 1048576;
pub const UF_NO_AUTH_DATA_REQUIRED: u32 = 33554432;
pub const UF_PARTIAL_SECRETS_ACCOUNT: u32 = 67108864;
pub const UF_PASSWD_CANT_CHANGE: u32 = 64;
pub const UF_PASSWD_NOTREQD: u32 = 32;
pub const UF_PASSWORD_EXPIRED: u32 = 8388608;
pub const UF_SCRIPT: u32 = 1;
pub const UF_SERVER_TRUST_ACCOUNT: u32 = 8192;
pub const UF_SETTABLE_BITS: u32 = 268385275;
pub const UF_SMARTCARD_REQUIRED: u32 = 262144;
pub const UF_TEMP_DUPLICATE_ACCOUNT: u32 = 256;
pub const UF_TRUSTED_FOR_DELEGATION: u32 = 524288;
pub const UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: u32 = 16777216;
pub const UF_USE_AES_KEYS: u32 = 134217728;
pub const UF_USE_DES_KEY_ONLY: u32 = 2097152;
pub const UF_WORKSTATION_TRUST_ACCOUNT: u32 = 4096;
pub const UNITS_PER_DAY: u32 = 24;
pub const UNITS_PER_WEEK: u32 = 168;
pub const USER_ACCT_EXPIRES_INFOLEVEL: u32 = 1017;
pub const USER_ACCT_EXPIRES_PARMNUM: u32 = 17;
pub const USER_AUTH_FLAGS_INFOLEVEL: u32 = 1010;
pub const USER_AUTH_FLAGS_PARMNUM: u32 = 10;
pub const USER_CODE_PAGE_INFOLEVEL: u32 = 1025;
pub const USER_CODE_PAGE_PARMNUM: u32 = 25;
pub const USER_COMMENT_INFOLEVEL: u32 = 1007;
pub const USER_COMMENT_PARMNUM: u32 = 7;
pub const USER_COUNTRY_CODE_INFOLEVEL: u32 = 1024;
pub const USER_COUNTRY_CODE_PARMNUM: u32 = 24;
pub const USER_FLAGS_INFOLEVEL: u32 = 1008;
pub const USER_FLAGS_PARMNUM: u32 = 8;
pub const USER_FULL_NAME_INFOLEVEL: u32 = 1011;
pub const USER_FULL_NAME_PARMNUM: u32 = 11;
pub const USER_HOME_DIR_DRIVE_INFOLEVEL: u32 = 1053;
pub const USER_HOME_DIR_DRIVE_PARMNUM: u32 = 53;
pub const USER_HOME_DIR_INFOLEVEL: u32 = 1006;
pub const USER_HOME_DIR_PARMNUM: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_0 {
    pub usri0_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1 {
    pub usri1_name: windows_core::PWSTR,
    pub usri1_password: windows_core::PWSTR,
    pub usri1_password_age: u32,
    pub usri1_priv: u32,
    pub usri1_home_dir: windows_core::PWSTR,
    pub usri1_comment: windows_core::PWSTR,
    pub usri1_flags: u32,
    pub usri1_script_path: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_10 {
    pub usri10_name: windows_core::PWSTR,
    pub usri10_comment: windows_core::PWSTR,
    pub usri10_usr_comment: windows_core::PWSTR,
    pub usri10_full_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1003 {
    pub usri1003_password: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1005 {
    pub usri1005_priv: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1006 {
    pub usri1006_home_dir: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1007 {
    pub usri1007_comment: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1008 {
    pub usri1008_flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1009 {
    pub usri1009_script_path: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1010 {
    pub usri1010_auth_flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1011 {
    pub usri1011_full_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1012 {
    pub usri1012_usr_comment: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1013 {
    pub usri1013_parms: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1014 {
    pub usri1014_workstations: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1017 {
    pub usri1017_acct_expires: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1018 {
    pub usri1018_max_storage: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1020 {
    pub usri1020_units_per_week: u32,
    pub usri1020_logon_hours: super::LPBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1023 {
    pub usri1023_logon_server: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1024 {
    pub usri1024_country_code: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1025 {
    pub usri1025_code_page: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1051 {
    pub usri1051_primary_group_id: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1052 {
    pub usri1052_profile: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_1053 {
    pub usri1053_home_dir_drive: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_11 {
    pub usri11_name: windows_core::PWSTR,
    pub usri11_comment: windows_core::PWSTR,
    pub usri11_usr_comment: windows_core::PWSTR,
    pub usri11_full_name: windows_core::PWSTR,
    pub usri11_priv: u32,
    pub usri11_auth_flags: u32,
    pub usri11_password_age: u32,
    pub usri11_home_dir: windows_core::PWSTR,
    pub usri11_parms: windows_core::PWSTR,
    pub usri11_last_logon: u32,
    pub usri11_last_logoff: u32,
    pub usri11_bad_pw_count: u32,
    pub usri11_num_logons: u32,
    pub usri11_logon_server: windows_core::PWSTR,
    pub usri11_country_code: u32,
    pub usri11_workstations: windows_core::PWSTR,
    pub usri11_max_storage: u32,
    pub usri11_units_per_week: u32,
    pub usri11_logon_hours: super::PBYTE,
    pub usri11_code_page: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_2 {
    pub usri2_name: windows_core::PWSTR,
    pub usri2_password: windows_core::PWSTR,
    pub usri2_password_age: u32,
    pub usri2_priv: u32,
    pub usri2_home_dir: windows_core::PWSTR,
    pub usri2_comment: windows_core::PWSTR,
    pub usri2_flags: u32,
    pub usri2_script_path: windows_core::PWSTR,
    pub usri2_auth_flags: u32,
    pub usri2_full_name: windows_core::PWSTR,
    pub usri2_usr_comment: windows_core::PWSTR,
    pub usri2_parms: windows_core::PWSTR,
    pub usri2_workstations: windows_core::PWSTR,
    pub usri2_last_logon: u32,
    pub usri2_last_logoff: u32,
    pub usri2_acct_expires: u32,
    pub usri2_max_storage: u32,
    pub usri2_units_per_week: u32,
    pub usri2_logon_hours: super::PBYTE,
    pub usri2_bad_pw_count: u32,
    pub usri2_num_logons: u32,
    pub usri2_logon_server: windows_core::PWSTR,
    pub usri2_country_code: u32,
    pub usri2_code_page: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_20 {
    pub usri20_name: windows_core::PWSTR,
    pub usri20_full_name: windows_core::PWSTR,
    pub usri20_comment: windows_core::PWSTR,
    pub usri20_flags: u32,
    pub usri20_user_id: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_21 {
    pub usri21_password: [u8; 16],
}
impl Default for USER_INFO_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_22 {
    pub usri22_name: windows_core::PWSTR,
    pub usri22_password: [u8; 16],
    pub usri22_password_age: u32,
    pub usri22_priv: u32,
    pub usri22_home_dir: windows_core::PWSTR,
    pub usri22_comment: windows_core::PWSTR,
    pub usri22_flags: u32,
    pub usri22_script_path: windows_core::PWSTR,
    pub usri22_auth_flags: u32,
    pub usri22_full_name: windows_core::PWSTR,
    pub usri22_usr_comment: windows_core::PWSTR,
    pub usri22_parms: windows_core::PWSTR,
    pub usri22_workstations: windows_core::PWSTR,
    pub usri22_last_logon: u32,
    pub usri22_last_logoff: u32,
    pub usri22_acct_expires: u32,
    pub usri22_max_storage: u32,
    pub usri22_units_per_week: u32,
    pub usri22_logon_hours: super::PBYTE,
    pub usri22_bad_pw_count: u32,
    pub usri22_num_logons: u32,
    pub usri22_logon_server: windows_core::PWSTR,
    pub usri22_country_code: u32,
    pub usri22_code_page: u32,
}
#[cfg(feature = "minwindef")]
impl Default for USER_INFO_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_23 {
    pub usri23_name: windows_core::PWSTR,
    pub usri23_full_name: windows_core::PWSTR,
    pub usri23_comment: windows_core::PWSTR,
    pub usri23_flags: u32,
    pub usri23_user_sid: super::PSID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_24 {
    pub usri24_internet_identity: windows_core::BOOL,
    pub usri24_flags: u32,
    pub usri24_internet_provider_name: windows_core::PWSTR,
    pub usri24_internet_principal_name: windows_core::PWSTR,
    pub usri24_user_sid: super::PSID,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_3 {
    pub usri3_name: windows_core::PWSTR,
    pub usri3_password: windows_core::PWSTR,
    pub usri3_password_age: u32,
    pub usri3_priv: u32,
    pub usri3_home_dir: windows_core::PWSTR,
    pub usri3_comment: windows_core::PWSTR,
    pub usri3_flags: u32,
    pub usri3_script_path: windows_core::PWSTR,
    pub usri3_auth_flags: u32,
    pub usri3_full_name: windows_core::PWSTR,
    pub usri3_usr_comment: windows_core::PWSTR,
    pub usri3_parms: windows_core::PWSTR,
    pub usri3_workstations: windows_core::PWSTR,
    pub usri3_last_logon: u32,
    pub usri3_last_logoff: u32,
    pub usri3_acct_expires: u32,
    pub usri3_max_storage: u32,
    pub usri3_units_per_week: u32,
    pub usri3_logon_hours: super::PBYTE,
    pub usri3_bad_pw_count: u32,
    pub usri3_num_logons: u32,
    pub usri3_logon_server: windows_core::PWSTR,
    pub usri3_country_code: u32,
    pub usri3_code_page: u32,
    pub usri3_user_id: u32,
    pub usri3_primary_group_id: u32,
    pub usri3_profile: windows_core::PWSTR,
    pub usri3_home_dir_drive: windows_core::PWSTR,
    pub usri3_password_expired: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_INFO_4 {
    pub usri4_name: windows_core::PWSTR,
    pub usri4_password: windows_core::PWSTR,
    pub usri4_password_age: u32,
    pub usri4_priv: u32,
    pub usri4_home_dir: windows_core::PWSTR,
    pub usri4_comment: windows_core::PWSTR,
    pub usri4_flags: u32,
    pub usri4_script_path: windows_core::PWSTR,
    pub usri4_auth_flags: u32,
    pub usri4_full_name: windows_core::PWSTR,
    pub usri4_usr_comment: windows_core::PWSTR,
    pub usri4_parms: windows_core::PWSTR,
    pub usri4_workstations: windows_core::PWSTR,
    pub usri4_last_logon: u32,
    pub usri4_last_logoff: u32,
    pub usri4_acct_expires: u32,
    pub usri4_max_storage: u32,
    pub usri4_units_per_week: u32,
    pub usri4_logon_hours: super::PBYTE,
    pub usri4_bad_pw_count: u32,
    pub usri4_num_logons: u32,
    pub usri4_logon_server: windows_core::PWSTR,
    pub usri4_country_code: u32,
    pub usri4_code_page: u32,
    pub usri4_user_sid: super::PSID,
    pub usri4_primary_group_id: u32,
    pub usri4_profile: windows_core::PWSTR,
    pub usri4_home_dir_drive: windows_core::PWSTR,
    pub usri4_password_expired: u32,
}
pub const USER_LAST_LOGOFF_INFOLEVEL: u32 = 1016;
pub const USER_LAST_LOGOFF_PARMNUM: u32 = 16;
pub const USER_LAST_LOGON_INFOLEVEL: u32 = 1015;
pub const USER_LAST_LOGON_PARMNUM: u32 = 15;
pub const USER_LOGON_HOURS_INFOLEVEL: u32 = 1020;
pub const USER_LOGON_HOURS_PARMNUM: u32 = 20;
pub const USER_LOGON_SERVER_INFOLEVEL: u32 = 1023;
pub const USER_LOGON_SERVER_PARMNUM: u32 = 23;
pub const USER_MAXSTORAGE_UNLIMITED: i32 = -1;
pub const USER_MAX_STORAGE_INFOLEVEL: u32 = 1018;
pub const USER_MAX_STORAGE_PARMNUM: u32 = 18;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_0 {
    pub usrmod0_min_passwd_len: u32,
    pub usrmod0_max_passwd_age: u32,
    pub usrmod0_min_passwd_age: u32,
    pub usrmod0_force_logoff: u32,
    pub usrmod0_password_hist_len: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1 {
    pub usrmod1_role: u32,
    pub usrmod1_primary: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1001 {
    pub usrmod1001_min_passwd_len: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1002 {
    pub usrmod1002_max_passwd_age: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1003 {
    pub usrmod1003_min_passwd_age: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1004 {
    pub usrmod1004_force_logoff: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1005 {
    pub usrmod1005_password_hist_len: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1006 {
    pub usrmod1006_role: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1007 {
    pub usrmod1007_primary: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_2 {
    pub usrmod2_domain_name: windows_core::PWSTR,
    pub usrmod2_domain_id: super::PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MODALS_INFO_3 {
    pub usrmod3_lockout_duration: u32,
    pub usrmod3_lockout_observation_window: u32,
    pub usrmod3_lockout_threshold: u32,
}
pub const USER_NAME_INFOLEVEL: u32 = 1001;
pub const USER_NAME_PARMNUM: u32 = 1;
pub const USER_NO_LOGOFF: i32 = -1;
pub const USER_NUM_LOGONS_INFOLEVEL: u32 = 1022;
pub const USER_NUM_LOGONS_PARMNUM: u32 = 22;
pub const USER_PAD_PW_COUNT_INFOLEVEL: u32 = 1021;
pub const USER_PAD_PW_COUNT_PARMNUM: u32 = 21;
pub const USER_PARMS_INFOLEVEL: u32 = 1013;
pub const USER_PARMS_PARMNUM: u32 = 13;
pub const USER_PASSWORD_AGE_INFOLEVEL: u32 = 1004;
pub const USER_PASSWORD_AGE_PARMNUM: u32 = 4;
pub const USER_PASSWORD_INFOLEVEL: u32 = 1003;
pub const USER_PASSWORD_PARMNUM: u32 = 3;
pub const USER_PRIMARY_GROUP_INFOLEVEL: u32 = 1051;
pub const USER_PRIMARY_GROUP_PARMNUM: u32 = 51;
pub const USER_PRIV_ADMIN: u32 = 2;
pub const USER_PRIV_GUEST: u32 = 0;
pub const USER_PRIV_INFOLEVEL: u32 = 1005;
pub const USER_PRIV_MASK: u32 = 3;
pub const USER_PRIV_PARMNUM: u32 = 5;
pub const USER_PRIV_USER: u32 = 1;
pub const USER_PROFILE: u32 = 52;
pub const USER_PROFILE_PARMNUM: u32 = 52;
pub const USER_SCRIPT_PATH_INFOLEVEL: u32 = 1009;
pub const USER_SCRIPT_PATH_PARMNUM: u32 = 9;
pub const USER_UNITS_PER_WEEK_INFOLEVEL: u32 = 1019;
pub const USER_UNITS_PER_WEEK_PARMNUM: u32 = 19;
pub const USER_USR_COMMENT_INFOLEVEL: u32 = 1012;
pub const USER_USR_COMMENT_PARMNUM: u32 = 12;
pub const USER_WORKSTATIONS_INFOLEVEL: u32 = 1014;
pub const USER_WORKSTATIONS_PARMNUM: u32 = 14;
pub const VALIDATED_LOGON: u32 = 0;
pub const VALID_LOGOFF: u32 = 1;
