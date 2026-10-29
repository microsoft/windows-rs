#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn I_NetLogonControl2(servername : windows_sys::core::PCWSTR, functioncode : super::DWORD, querylevel : super::DWORD, data : super::LPBYTE, buffer : *mut super::LPBYTE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetAccessAdd(servername : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetAccessDel(servername : windows_sys::core::PCWSTR, resource : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetAccessEnum(servername : windows_sys::core::PCWSTR, basepath : windows_sys::core::PCWSTR, recursive : super::DWORD, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetAccessGetInfo(servername : windows_sys::core::PCWSTR, resource : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetAccessGetUserPerms(servername : windows_sys::core::PCWSTR, ugname : windows_sys::core::PCWSTR, resource : windows_sys::core::PCWSTR, perms : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetAccessSetInfo(servername : windows_sys::core::PCWSTR, resource : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetAddServiceAccount(servername : windows_sys::core::PCWSTR, accountname : windows_sys::core::PCWSTR, password : windows_sys::core::PCWSTR, flags : super::DWORD) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn NetEnumerateServiceAccounts(servername : windows_sys::core::PCWSTR, flags : super::DWORD, accountscount : *mut super::DWORD, accounts : *mut super::PZPWSTR) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetGetAnyDCName(servername : windows_sys::core::PCWSTR, domainname : windows_sys::core::PCWSTR, buffer : *mut super::LPBYTE) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetGetDCName(servername : windows_sys::core::PCWSTR, domainname : windows_sys::core::PCWSTR, buffer : *mut super::LPBYTE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetGetDisplayInformationIndex(servername : windows_sys::core::PCWSTR, level : super::DWORD, prefix : windows_sys::core::PCWSTR, index : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetGroupAdd(servername : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetGroupAddUser(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetGroupDel(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetGroupDelUser(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetGroupEnum(servername : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::PDWORD_PTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetGroupGetInfo(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetGroupGetUsers(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::PDWORD_PTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetGroupSetInfo(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetGroupSetUsers(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, totalentries : super::DWORD) -> super::DWORD);
windows_link::link!("netapi32.dll" "system" fn NetIsServiceAccount(servername : windows_sys::core::PCWSTR, accountname : windows_sys::core::PCWSTR, isservice : *mut windows_sys::core::BOOL) -> windows_sys::core::NTSTATUS);
windows_link::link!("netapi32.dll" "system" fn NetIsServiceAccount2(servername : windows_sys::core::PCWSTR, accountname : windows_sys::core::PCWSTR, isservice : *mut windows_sys::core::BOOL, accounttype : *mut MSA_INFO_ACCOUNT_TYPE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupAdd(servername : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupAddMember(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, membersid : super::PSID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupAddMembers(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, totalentries : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupDel(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupDelMember(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, membersid : super::PSID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupDelMembers(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, totalentries : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupEnum(servername : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::PDWORD_PTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupGetInfo(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupGetMembers(servername : windows_sys::core::PCWSTR, localgroupname : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::PDWORD_PTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupSetInfo(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetLocalGroupSetMembers(servername : windows_sys::core::PCWSTR, groupname : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, totalentries : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn NetQueryDisplayInformation(servername : windows_sys::core::PCWSTR, level : super::DWORD, index : super::DWORD, entriesrequested : super::DWORD, preferredmaximumlength : super::DWORD, returnedentrycount : super::LPDWORD, sortedbuffer : *mut super::PVOID) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetQueryServiceAccount(servername : windows_sys::core::PCWSTR, accountname : windows_sys::core::PCWSTR, infolevel : super::DWORD, buffer : *mut super::PBYTE) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetRemoveServiceAccount(servername : windows_sys::core::PCWSTR, accountname : windows_sys::core::PCWSTR, flags : super::DWORD) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetUserAdd(servername : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetUserChangePassword(domainname : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, oldpassword : windows_sys::core::PCWSTR, newpassword : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetUserDel(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetUserEnum(servername : windows_sys::core::PCWSTR, level : super::DWORD, filter : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::PDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetUserGetGroups(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetUserGetInfo(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetUserGetLocalGroups(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, level : super::DWORD, flags : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetUserModalsGet(servername : windows_sys::core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetUserModalsSet(servername : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetUserSetGroups(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, num_entries : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetUserSetInfo(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetValidatePasswordPolicy(servername : windows_sys::core::PCWSTR, qualifier : super::LPVOID, validationtype : NET_VALIDATE_PASSWORD_TYPE, inputarg : super::LPVOID, outputarg : *mut super::LPVOID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetValidatePasswordPolicyFree(outputarg : *mut super::LPVOID) -> super::DWORD);
pub const ACCESS_ACCESS_LIST_INFOLEVEL: i32 = 1004;
pub const ACCESS_ACCESS_LIST_PARMNUM: i32 = 4;
pub const ACCESS_ALL: i32 = 127;
pub const ACCESS_ATRIB: i32 = 32;
pub const ACCESS_ATTR_INFOLEVEL: i32 = 1002;
pub const ACCESS_ATTR_PARMNUM: i32 = 2;
pub const ACCESS_AUDIT: i32 = 1;
pub const ACCESS_COUNT_INFOLEVEL: i32 = 1003;
pub const ACCESS_COUNT_PARMNUM: i32 = 3;
pub const ACCESS_CREATE: i32 = 4;
pub const ACCESS_DELETE: i32 = 16;
pub const ACCESS_EXEC: i32 = 8;
pub const ACCESS_FAIL_ACL: i32 = 2048;
pub const ACCESS_FAIL_DELETE: i32 = 1024;
pub const ACCESS_FAIL_MASK: i32 = 3840;
pub const ACCESS_FAIL_OPEN: i32 = 256;
pub const ACCESS_FAIL_SHIFT: i32 = 4;
pub const ACCESS_FAIL_WRITE: i32 = 512;
pub const ACCESS_GROUP: i32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ACCESS_INFO_0 {
    pub acc0_resource_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ACCESS_INFO_1 {
    pub acc1_resource_name: windows_sys::core::PWSTR,
    pub acc1_attr: super::DWORD,
    pub acc1_count: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ACCESS_INFO_1002 {
    pub acc1002_attr: super::DWORD,
}
pub const ACCESS_LETTERS: windows_sys::core::PCSTR = windows_sys::core::s!("RWCXDAP         ");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ACCESS_LIST {
    pub acl_ugname: windows_sys::core::PWSTR,
    pub acl_access: super::DWORD,
}
pub const ACCESS_NONE: i32 = 0;
pub const ACCESS_PERM: i32 = 64;
pub const ACCESS_READ: i32 = 1;
pub const ACCESS_RESOURCE_NAME_INFOLEVEL: i32 = 1001;
pub const ACCESS_RESOURCE_NAME_PARMNUM: i32 = 1;
pub const ACCESS_SUCCESS_ACL: i32 = 128;
pub const ACCESS_SUCCESS_DELETE: i32 = 64;
pub const ACCESS_SUCCESS_MASK: i32 = 240;
pub const ACCESS_SUCCESS_OPEN: i32 = 16;
pub const ACCESS_SUCCESS_WRITE: i32 = 32;
pub const ACCESS_WRITE: i32 = 2;
pub const AF_OP_ACCOUNTS: i32 = 8;
pub const AF_OP_COMM: i32 = 2;
pub const AF_OP_PRINT: i32 = 1;
pub const AF_OP_SERVER: i32 = 4;
pub const AF_SETTABLE_BITS: i32 = 15;
pub const DEF_FORCE_LOGOFF: u32 = 4294967295;
pub const DEF_MAX_BADPW: i32 = 0;
pub const DEF_MAX_PWAGE: u32 = 4294967295;
pub const DEF_MAX_PWHIST: i32 = 8;
pub const DEF_MIN_PWAGE: u32 = 0;
pub const DEF_MIN_PWLEN: i32 = 6;
pub const DEF_PWUNIQUENESS: i32 = 5;
pub const DELEGATED_MANAGED_SERVICE_ACCOUNT_PASSWORD: windows_sys::core::PCSTR = windows_sys::core::s!("_SA_{F8262F4C-499B-4770-88B4-A75C91D0D8E9}");
pub const DelegatedManagedServiceAccount: MSA_INFO_ACCOUNT_TYPE = 3;
pub const FILTER_INTERDOMAIN_TRUST_ACCOUNT: i32 = 8;
pub const FILTER_NORMAL_ACCOUNT: i32 = 2;
pub const FILTER_SERVER_TRUST_ACCOUNT: i32 = 32;
pub const FILTER_TEMP_DUPLICATE_ACCOUNT: i32 = 1;
pub const FILTER_WORKSTATION_TRUST_ACCOUNT: i32 = 16;
pub const GROUPIDMASK: i32 = 32768;
pub const GROUP_ALL_INFOLEVEL: i32 = 1000;
pub const GROUP_ALL_PARMNUM: i32 = 0;
pub const GROUP_ATTRIBUTES_INFOLEVEL: i32 = 1003;
pub const GROUP_ATTRIBUTES_PARMNUM: i32 = 3;
pub const GROUP_COMMENT_INFOLEVEL: i32 = 1002;
pub const GROUP_COMMENT_PARMNUM: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GROUP_INFO_0 {
    pub grpi0_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GROUP_INFO_1 {
    pub grpi1_name: windows_sys::core::PWSTR,
    pub grpi1_comment: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GROUP_INFO_1002 {
    pub grpi1002_comment: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GROUP_INFO_1005 {
    pub grpi1005_attributes: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GROUP_INFO_2 {
    pub grpi2_name: windows_sys::core::PWSTR,
    pub grpi2_comment: windows_sys::core::PWSTR,
    pub grpi2_group_id: super::DWORD,
    pub grpi2_attributes: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct GROUP_INFO_3 {
    pub grpi3_name: windows_sys::core::PWSTR,
    pub grpi3_comment: windows_sys::core::PWSTR,
    pub grpi3_group_sid: super::PSID,
    pub grpi3_attributes: super::DWORD,
}
pub const GROUP_NAME_INFOLEVEL: i32 = 1001;
pub const GROUP_NAME_PARMNUM: i32 = 1;
pub const GROUP_SPECIALGRP_ADMINS: windows_sys::core::PCWSTR = windows_sys::core::w!("ADMINS");
pub const GROUP_SPECIALGRP_GUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("GUESTS");
pub const GROUP_SPECIALGRP_LOCAL: windows_sys::core::PCWSTR = windows_sys::core::w!("LOCAL");
pub const GROUP_SPECIALGRP_USERS: windows_sys::core::PCWSTR = windows_sys::core::w!("USERS");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GROUP_USERS_INFO_0 {
    pub grui0_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GROUP_USERS_INFO_1 {
    pub grui1_name: windows_sys::core::PWSTR,
    pub grui1_attributes: super::DWORD,
}
pub const GroupManagedServiceAccount: MSA_INFO_ACCOUNT_TYPE = 2;
pub const LG_INCLUDE_INDIRECT: i32 = 1;
pub const LOCALGROUP_COMMENT_PARMNUM: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LOCALGROUP_INFO_0 {
    pub lgrpi0_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LOCALGROUP_INFO_1 {
    pub lgrpi1_name: windows_sys::core::PWSTR,
    pub lgrpi1_comment: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LOCALGROUP_INFO_1002 {
    pub lgrpi1002_comment: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct LOCALGROUP_MEMBERS_INFO_0 {
    pub lgrmi0_sid: super::PSID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct LOCALGROUP_MEMBERS_INFO_1 {
    pub lgrmi1_sid: super::PSID,
    pub lgrmi1_sidusage: super::SID_NAME_USE,
    pub lgrmi1_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct LOCALGROUP_MEMBERS_INFO_2 {
    pub lgrmi2_sid: super::PSID,
    pub lgrmi2_sidusage: super::SID_NAME_USE,
    pub lgrmi2_domainandname: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LOCALGROUP_MEMBERS_INFO_3 {
    pub lgrmi3_domainandname: windows_sys::core::PWSTR,
}
pub const LOCALGROUP_NAME_PARMNUM: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LOCALGROUP_USERS_INFO_0 {
    pub lgrui0_name: windows_sys::core::PWSTR,
}
pub type LPACCESS_INFO_0 = *mut ACCESS_INFO_0;
#[cfg(feature = "shared")]
pub type LPACCESS_INFO_1 = *mut ACCESS_INFO_1;
#[cfg(feature = "shared")]
pub type LPACCESS_INFO_1002 = *mut ACCESS_INFO_1002;
#[cfg(feature = "shared")]
pub type LPACCESS_LIST = *mut ACCESS_LIST;
pub type LPGROUP_INFO_0 = *mut GROUP_INFO_0;
pub type LPGROUP_INFO_1 = *mut GROUP_INFO_1;
pub type LPGROUP_INFO_1002 = *mut GROUP_INFO_1002;
#[cfg(feature = "shared")]
pub type LPGROUP_INFO_1005 = *mut GROUP_INFO_1005;
pub type LPGROUP_USERS_INFO_0 = *mut GROUP_USERS_INFO_0;
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1 = *mut USER_INFO_1;
pub type LPUSER_INFO_10 = *mut USER_INFO_10;
pub type LPUSER_INFO_1003 = *mut USER_INFO_1003;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1005 = *mut USER_INFO_1005;
pub type LPUSER_INFO_1006 = *mut USER_INFO_1006;
pub type LPUSER_INFO_1007 = *mut USER_INFO_1007;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1008 = *mut USER_INFO_1008;
pub type LPUSER_INFO_1009 = *mut USER_INFO_1009;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1010 = *mut USER_INFO_1010;
pub type LPUSER_INFO_1011 = *mut USER_INFO_1011;
pub type LPUSER_INFO_1012 = *mut USER_INFO_1012;
pub type LPUSER_INFO_1013 = *mut USER_INFO_1013;
pub type LPUSER_INFO_1014 = *mut USER_INFO_1014;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1017 = *mut USER_INFO_1017;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1018 = *mut USER_INFO_1018;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1020 = *mut USER_INFO_1020;
pub type LPUSER_INFO_1023 = *mut USER_INFO_1023;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1024 = *mut USER_INFO_1024;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1025 = *mut USER_INFO_1025;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_1051 = *mut USER_INFO_1051;
pub type LPUSER_INFO_1052 = *mut USER_INFO_1052;
pub type LPUSER_INFO_1053 = *mut USER_INFO_1053;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_11 = *mut USER_INFO_11;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_2 = *mut USER_INFO_2;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_20 = *mut USER_INFO_20;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_21 = *mut USER_INFO_21;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_22 = *mut USER_INFO_22;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPUSER_INFO_23 = *mut USER_INFO_23;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPUSER_INFO_24 = *mut USER_INFO_24;
#[cfg(feature = "shared")]
pub type LPUSER_INFO_3 = *mut USER_INFO_3;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPUSER_INFO_4 = *mut USER_INFO_4;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_0 = *mut USER_MODALS_INFO_0;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_1 = *mut USER_MODALS_INFO_1;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_1001 = *mut USER_MODALS_INFO_1001;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_1002 = *mut USER_MODALS_INFO_1002;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_1003 = *mut USER_MODALS_INFO_1003;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_1004 = *mut USER_MODALS_INFO_1004;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_1005 = *mut USER_MODALS_INFO_1005;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_1006 = *mut USER_MODALS_INFO_1006;
pub type LPUSER_MODALS_INFO_1007 = *mut USER_MODALS_INFO_1007;
#[cfg(feature = "winnt")]
pub type LPUSER_MODALS_INFO_2 = *mut USER_MODALS_INFO_2;
#[cfg(feature = "shared")]
pub type LPUSER_MODALS_INFO_3 = *mut USER_MODALS_INFO_3;
pub const MAXPERMENTRIES: i32 = 64;
pub const MAX_PASSWD_LEN: i32 = 256;
pub const MODALS_DOMAIN_ID_INFOLEVEL: i32 = 1009;
pub const MODALS_DOMAIN_ID_PARMNUM: i32 = 9;
pub const MODALS_DOMAIN_NAME_INFOLEVEL: i32 = 1008;
pub const MODALS_DOMAIN_NAME_PARMNUM: i32 = 8;
pub const MODALS_FORCE_LOGOFF_INFOLEVEL: i32 = 1004;
pub const MODALS_FORCE_LOGOFF_PARMNUM: i32 = 4;
pub const MODALS_LOCKOUT_DURATION_PARMNUM: i32 = 10;
pub const MODALS_LOCKOUT_OBSERVATION_WINDOW_PARMNUM: i32 = 11;
pub const MODALS_LOCKOUT_THRESHOLD_PARMNUM: i32 = 12;
pub const MODALS_MAX_PASSWD_AGE_INFOLEVEL: i32 = 1002;
pub const MODALS_MAX_PASSWD_AGE_PARMNUM: i32 = 2;
pub const MODALS_MIN_PASSWD_AGE_INFOLEVEL: i32 = 1003;
pub const MODALS_MIN_PASSWD_AGE_PARMNUM: i32 = 3;
pub const MODALS_MIN_PASSWD_LEN_INFOLEVEL: i32 = 1001;
pub const MODALS_MIN_PASSWD_LEN_PARMNUM: i32 = 1;
pub const MODALS_PASSWD_HIST_LEN_INFOLEVEL: i32 = 1005;
pub const MODALS_PASSWD_HIST_LEN_PARMNUM: i32 = 5;
pub const MODALS_PRIMARY_INFOLEVEL: i32 = 1007;
pub const MODALS_PRIMARY_PARMNUM: i32 = 7;
pub const MODALS_ROLE_INFOLEVEL: i32 = 1006;
pub const MODALS_ROLE_PARMNUM: i32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MSA_INFO_0 {
    pub State: MSA_INFO_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const NETLOGON_CONTROL_BACKUP_CHANGE_LOG: i32 = 65532;
pub const NETLOGON_CONTROL_BREAKPOINT: i32 = 65535;
pub const NETLOGON_CONTROL_CHANGE_PASSWORD: i32 = 9;
pub const NETLOGON_CONTROL_FIND_USER: i32 = 8;
pub const NETLOGON_CONTROL_FORCE_DNS_REG: i32 = 11;
pub const NETLOGON_CONTROL_PDC_REPLICATE: i32 = 4;
pub const NETLOGON_CONTROL_QUERY: i32 = 1;
pub const NETLOGON_CONTROL_QUERY_DNS_REG: i32 = 12;
pub const NETLOGON_CONTROL_QUERY_ENC_TYPES: i32 = 13;
pub const NETLOGON_CONTROL_REDISCOVER: i32 = 5;
pub const NETLOGON_CONTROL_REPLICATE: i32 = 2;
pub const NETLOGON_CONTROL_SET_DBFLAG: i32 = 65534;
pub const NETLOGON_CONTROL_SYNCHRONIZE: i32 = 3;
pub const NETLOGON_CONTROL_TC_QUERY: i32 = 6;
pub const NETLOGON_CONTROL_TC_VERIFY: i32 = 10;
pub const NETLOGON_CONTROL_TRANSPORT_NOTIFY: i32 = 7;
pub const NETLOGON_CONTROL_TRUNCATE_LOG: i32 = 65533;
pub const NETLOGON_CONTROL_UNLOAD_NETLOGON_DLL: i32 = 65531;
pub const NETLOGON_DNS_UPDATE_FAILURE: i32 = 64;
pub const NETLOGON_FULL_SYNC_REPLICATION: i32 = 4;
pub const NETLOGON_HAS_IP: i32 = 16;
pub const NETLOGON_HAS_TIMESERV: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NETLOGON_INFO_1 {
    pub netlog1_flags: super::DWORD,
    pub netlog1_pdc_connection_status: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NETLOGON_INFO_2 {
    pub netlog2_flags: super::DWORD,
    pub netlog2_pdc_connection_status: super::DWORD,
    pub netlog2_trusted_dc_name: windows_sys::core::PWSTR,
    pub netlog2_tc_connection_status: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NETLOGON_INFO_3 {
    pub netlog3_flags: super::DWORD,
    pub netlog3_logon_attempts: super::DWORD,
    pub netlog3_reserved1: super::DWORD,
    pub netlog3_reserved2: super::DWORD,
    pub netlog3_reserved3: super::DWORD,
    pub netlog3_reserved4: super::DWORD,
    pub netlog3_reserved5: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NETLOGON_INFO_4 {
    pub netlog4_trusted_dc_name: windows_sys::core::PWSTR,
    pub netlog4_trusted_domain_name: windows_sys::core::PWSTR,
}
pub const NETLOGON_REDO_NEEDED: i32 = 8;
pub const NETLOGON_REPLICATION_IN_PROGRESS: i32 = 2;
pub const NETLOGON_REPLICATION_NEEDED: i32 = 1;
pub const NETLOGON_VERIFY_STATUS_RETURNED: i32 = 128;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NET_DISPLAY_GROUP {
    pub grpi3_name: windows_sys::core::PWSTR,
    pub grpi3_comment: windows_sys::core::PWSTR,
    pub grpi3_group_id: super::DWORD,
    pub grpi3_attributes: super::DWORD,
    pub grpi3_next_index: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NET_DISPLAY_MACHINE {
    pub usri2_name: windows_sys::core::PWSTR,
    pub usri2_comment: windows_sys::core::PWSTR,
    pub usri2_flags: super::DWORD,
    pub usri2_user_id: super::DWORD,
    pub usri2_next_index: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NET_DISPLAY_USER {
    pub usri1_name: windows_sys::core::PWSTR,
    pub usri1_comment: windows_sys::core::PWSTR,
    pub usri1_flags: super::DWORD,
    pub usri1_full_name: windows_sys::core::PWSTR,
    pub usri1_user_id: super::DWORD,
    pub usri1_next_index: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub PasswordMatched: super::BOOLEAN,
}
pub const NET_VALIDATE_BAD_PASSWORD_COUNT: i32 = 8;
pub const NET_VALIDATE_BAD_PASSWORD_TIME: i32 = 2;
pub const NET_VALIDATE_LOCKOUT_TIME: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct NET_VALIDATE_OUTPUT_ARG {
    pub ChangedPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ValidationStatus: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: windows_sys::core::PWSTR,
    pub UserAccountName: windows_sys::core::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMatch: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NET_VALIDATE_PASSWORD_HASH {
    pub Length: super::ULONG,
    pub Hash: super::LPBYTE,
}
pub const NET_VALIDATE_PASSWORD_HISTORY: i32 = 32;
pub const NET_VALIDATE_PASSWORD_HISTORY_LENGTH: i32 = 16;
pub const NET_VALIDATE_PASSWORD_LAST_SET: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: windows_sys::core::PWSTR,
    pub UserAccountName: windows_sys::core::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMustChangeAtNextLogon: super::BOOLEAN,
    pub ClearLockout: super::BOOLEAN,
}
pub type NET_VALIDATE_PASSWORD_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct NET_VALIDATE_PERSISTED_FIELDS {
    pub PresentFields: super::ULONG,
    pub PasswordLastSet: super::FILETIME,
    pub BadPasswordTime: super::FILETIME,
    pub LockoutTime: super::FILETIME,
    pub BadPasswordCount: super::ULONG,
    pub PasswordHistoryLength: super::ULONG,
    pub PasswordHistory: PNET_VALIDATE_PASSWORD_HASH,
}
pub const NON_VALIDATED_LOGON: i32 = 3;
pub const NULL_USERSETINFO_PASSWD: windows_sys::core::PCSTR = windows_sys::core::s!("              ");
pub const NetValidateAuthentication: NET_VALIDATE_PASSWORD_TYPE = 1;
pub const NetValidatePasswordChange: NET_VALIDATE_PASSWORD_TYPE = 2;
pub const NetValidatePasswordReset: NET_VALIDATE_PASSWORD_TYPE = 3;
pub const ONE_DAY: u32 = 86400;
pub type PACCESS_INFO_0 = *mut ACCESS_INFO_0;
#[cfg(feature = "shared")]
pub type PACCESS_INFO_1 = *mut ACCESS_INFO_1;
#[cfg(feature = "shared")]
pub type PACCESS_INFO_1002 = *mut ACCESS_INFO_1002;
#[cfg(feature = "shared")]
pub type PACCESS_LIST = *mut ACCESS_LIST;
pub const PASSWORD_EXPIRED: i32 = 2;
pub type PGROUP_INFO_0 = *mut GROUP_INFO_0;
pub type PGROUP_INFO_1 = *mut GROUP_INFO_1;
pub type PGROUP_INFO_1002 = *mut GROUP_INFO_1002;
#[cfg(feature = "shared")]
pub type PGROUP_INFO_1005 = *mut GROUP_INFO_1005;
#[cfg(feature = "shared")]
pub type PGROUP_INFO_2 = *mut GROUP_INFO_2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGROUP_INFO_3 = *mut GROUP_INFO_3;
pub type PGROUP_USERS_INFO_0 = *mut GROUP_USERS_INFO_0;
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
pub type PNETLOGON_INFO_1 = *mut NETLOGON_INFO_1;
#[cfg(feature = "shared")]
pub type PNETLOGON_INFO_2 = *mut NETLOGON_INFO_2;
#[cfg(feature = "shared")]
pub type PNETLOGON_INFO_3 = *mut NETLOGON_INFO_3;
pub type PNETLOGON_INFO_4 = *mut NETLOGON_INFO_4;
#[cfg(feature = "shared")]
pub type PNET_DISPLAY_GROUP = *mut NET_DISPLAY_GROUP;
#[cfg(feature = "shared")]
pub type PNET_DISPLAY_MACHINE = *mut NET_DISPLAY_MACHINE;
#[cfg(feature = "shared")]
pub type PNET_DISPLAY_USER = *mut NET_DISPLAY_USER;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PNET_VALIDATE_AUTHENTICATION_INPUT_ARG = *mut NET_VALIDATE_AUTHENTICATION_INPUT_ARG;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PNET_VALIDATE_OUTPUT_ARG = *mut NET_VALIDATE_OUTPUT_ARG;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PNET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG = *mut NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG;
#[cfg(feature = "shared")]
pub type PNET_VALIDATE_PASSWORD_HASH = *mut NET_VALIDATE_PASSWORD_HASH;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PNET_VALIDATE_PASSWORD_RESET_INPUT_ARG = *mut NET_VALIDATE_PASSWORD_RESET_INPUT_ARG;
pub type PNET_VALIDATE_PASSWORD_TYPE = *mut NET_VALIDATE_PASSWORD_TYPE;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PNET_VALIDATE_PERSISTED_FIELDS = *mut NET_VALIDATE_PERSISTED_FIELDS;
pub type PUSER_INFO_0 = *mut USER_INFO_0;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1 = *mut USER_INFO_1;
pub type PUSER_INFO_10 = *mut USER_INFO_10;
pub type PUSER_INFO_1003 = *mut USER_INFO_1003;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1005 = *mut USER_INFO_1005;
pub type PUSER_INFO_1006 = *mut USER_INFO_1006;
pub type PUSER_INFO_1007 = *mut USER_INFO_1007;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1008 = *mut USER_INFO_1008;
pub type PUSER_INFO_1009 = *mut USER_INFO_1009;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1010 = *mut USER_INFO_1010;
pub type PUSER_INFO_1011 = *mut USER_INFO_1011;
pub type PUSER_INFO_1012 = *mut USER_INFO_1012;
pub type PUSER_INFO_1013 = *mut USER_INFO_1013;
pub type PUSER_INFO_1014 = *mut USER_INFO_1014;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1017 = *mut USER_INFO_1017;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1018 = *mut USER_INFO_1018;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1020 = *mut USER_INFO_1020;
pub type PUSER_INFO_1023 = *mut USER_INFO_1023;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1024 = *mut USER_INFO_1024;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1025 = *mut USER_INFO_1025;
#[cfg(feature = "shared")]
pub type PUSER_INFO_1051 = *mut USER_INFO_1051;
pub type PUSER_INFO_1052 = *mut USER_INFO_1052;
pub type PUSER_INFO_1053 = *mut USER_INFO_1053;
#[cfg(feature = "shared")]
pub type PUSER_INFO_11 = *mut USER_INFO_11;
#[cfg(feature = "shared")]
pub type PUSER_INFO_2 = *mut USER_INFO_2;
#[cfg(feature = "shared")]
pub type PUSER_INFO_20 = *mut USER_INFO_20;
#[cfg(feature = "shared")]
pub type PUSER_INFO_21 = *mut USER_INFO_21;
#[cfg(feature = "shared")]
pub type PUSER_INFO_22 = *mut USER_INFO_22;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSER_INFO_23 = *mut USER_INFO_23;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSER_INFO_24 = *mut USER_INFO_24;
#[cfg(feature = "shared")]
pub type PUSER_INFO_3 = *mut USER_INFO_3;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSER_INFO_4 = *mut USER_INFO_4;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_0 = *mut USER_MODALS_INFO_0;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_1 = *mut USER_MODALS_INFO_1;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_1001 = *mut USER_MODALS_INFO_1001;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_1002 = *mut USER_MODALS_INFO_1002;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_1003 = *mut USER_MODALS_INFO_1003;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_1004 = *mut USER_MODALS_INFO_1004;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_1005 = *mut USER_MODALS_INFO_1005;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_1006 = *mut USER_MODALS_INFO_1006;
pub type PUSER_MODALS_INFO_1007 = *mut USER_MODALS_INFO_1007;
#[cfg(feature = "winnt")]
pub type PUSER_MODALS_INFO_2 = *mut USER_MODALS_INFO_2;
#[cfg(feature = "shared")]
pub type PUSER_MODALS_INFO_3 = *mut USER_MODALS_INFO_3;
pub const SERVICE_ACCOUNT_FLAG_ADD_AGAINST_RODC: i32 = 2;
pub const SERVICE_ACCOUNT_FLAG_LINK_TO_HOST_ONLY: i32 = 1;
pub const SERVICE_ACCOUNT_FLAG_REMOVE_OFFLINE: i32 = 2;
pub const SERVICE_ACCOUNT_FLAG_UNLINK_FROM_HOST_ONLY: i32 = 1;
pub const SERVICE_ACCOUNT_PASSWORD: windows_sys::core::PCSTR = windows_sys::core::s!("_SA_{262E99C9-6160-4871-ACEC-4E61736B6F21}");
pub const SERVICE_ACCOUNT_SECRET_PREFIX: windows_sys::core::PCSTR = windows_sys::core::s!("_SC_{262E99C9-6160-4871-ACEC-4E61736B6F21}_");
pub const ServiceAccountPasswordGUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x262e99c9_6160_4871_acec_4e61736b6f21);
pub const StandAloneManagedServiceAccount: MSA_INFO_ACCOUNT_TYPE = 1;
pub const TIMEQ_FOREVER: u32 = 4294967295;
pub const UAS_ROLE_BACKUP: i32 = 2;
pub const UAS_ROLE_MEMBER: i32 = 1;
pub const UAS_ROLE_PRIMARY: i32 = 3;
pub const UAS_ROLE_STANDALONE: i32 = 0;
pub const UF_ACCOUNTDISABLE: i32 = 2;
pub const UF_ACCOUNT_TYPE_MASK: i32 = 15104;
pub const UF_DONT_EXPIRE_PASSWD: i32 = 65536;
pub const UF_DONT_REQUIRE_PREAUTH: i32 = 4194304;
pub const UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: i32 = 128;
pub const UF_HOMEDIR_REQUIRED: i32 = 8;
pub const UF_INTERDOMAIN_TRUST_ACCOUNT: i32 = 2048;
pub const UF_LOCKOUT: i32 = 16;
pub const UF_MACHINE_ACCOUNT_MASK: i32 = 14336;
pub const UF_MNS_LOGON_ACCOUNT: i32 = 131072;
pub const UF_NORMAL_ACCOUNT: i32 = 512;
pub const UF_NOT_DELEGATED: i32 = 1048576;
pub const UF_NO_AUTH_DATA_REQUIRED: i32 = 33554432;
pub const UF_PARTIAL_SECRETS_ACCOUNT: i32 = 67108864;
pub const UF_PASSWD_CANT_CHANGE: i32 = 64;
pub const UF_PASSWD_NOTREQD: i32 = 32;
pub const UF_PASSWORD_EXPIRED: i32 = 8388608;
pub const UF_SCRIPT: i32 = 1;
pub const UF_SERVER_TRUST_ACCOUNT: i32 = 8192;
pub const UF_SETTABLE_BITS: i32 = 268385275;
pub const UF_SMARTCARD_REQUIRED: i32 = 262144;
pub const UF_TEMP_DUPLICATE_ACCOUNT: i32 = 256;
pub const UF_TRUSTED_FOR_DELEGATION: i32 = 524288;
pub const UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: i32 = 16777216;
pub const UF_USE_AES_KEYS: i32 = 134217728;
pub const UF_USE_DES_KEY_ONLY: i32 = 2097152;
pub const UF_WORKSTATION_TRUST_ACCOUNT: i32 = 4096;
pub const UNITS_PER_DAY: i32 = 24;
pub const UNITS_PER_WEEK: i32 = 168;
pub const USER_ACCT_EXPIRES_INFOLEVEL: i32 = 1017;
pub const USER_ACCT_EXPIRES_PARMNUM: i32 = 17;
pub const USER_AUTH_FLAGS_INFOLEVEL: i32 = 1010;
pub const USER_AUTH_FLAGS_PARMNUM: i32 = 10;
pub const USER_CODE_PAGE_INFOLEVEL: i32 = 1025;
pub const USER_CODE_PAGE_PARMNUM: i32 = 25;
pub const USER_COMMENT_INFOLEVEL: i32 = 1007;
pub const USER_COMMENT_PARMNUM: i32 = 7;
pub const USER_COUNTRY_CODE_INFOLEVEL: i32 = 1024;
pub const USER_COUNTRY_CODE_PARMNUM: i32 = 24;
pub const USER_FLAGS_INFOLEVEL: i32 = 1008;
pub const USER_FLAGS_PARMNUM: i32 = 8;
pub const USER_FULL_NAME_INFOLEVEL: i32 = 1011;
pub const USER_FULL_NAME_PARMNUM: i32 = 11;
pub const USER_HOME_DIR_DRIVE_INFOLEVEL: i32 = 1053;
pub const USER_HOME_DIR_DRIVE_PARMNUM: i32 = 53;
pub const USER_HOME_DIR_INFOLEVEL: i32 = 1006;
pub const USER_HOME_DIR_PARMNUM: i32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_0 {
    pub usri0_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1 {
    pub usri1_name: windows_sys::core::PWSTR,
    pub usri1_password: windows_sys::core::PWSTR,
    pub usri1_password_age: super::DWORD,
    pub usri1_priv: super::DWORD,
    pub usri1_home_dir: windows_sys::core::PWSTR,
    pub usri1_comment: windows_sys::core::PWSTR,
    pub usri1_flags: super::DWORD,
    pub usri1_script_path: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_10 {
    pub usri10_name: windows_sys::core::PWSTR,
    pub usri10_comment: windows_sys::core::PWSTR,
    pub usri10_usr_comment: windows_sys::core::PWSTR,
    pub usri10_full_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1003 {
    pub usri1003_password: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1005 {
    pub usri1005_priv: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1006 {
    pub usri1006_home_dir: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1007 {
    pub usri1007_comment: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1008 {
    pub usri1008_flags: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1009 {
    pub usri1009_script_path: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1010 {
    pub usri1010_auth_flags: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1011 {
    pub usri1011_full_name: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1012 {
    pub usri1012_usr_comment: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1013 {
    pub usri1013_parms: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1014 {
    pub usri1014_workstations: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1017 {
    pub usri1017_acct_expires: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1018 {
    pub usri1018_max_storage: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1020 {
    pub usri1020_units_per_week: super::DWORD,
    pub usri1020_logon_hours: super::LPBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1023 {
    pub usri1023_logon_server: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1024 {
    pub usri1024_country_code: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1025 {
    pub usri1025_code_page: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1051 {
    pub usri1051_primary_group_id: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1052 {
    pub usri1052_profile: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_1053 {
    pub usri1053_home_dir_drive: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_11 {
    pub usri11_name: windows_sys::core::PWSTR,
    pub usri11_comment: windows_sys::core::PWSTR,
    pub usri11_usr_comment: windows_sys::core::PWSTR,
    pub usri11_full_name: windows_sys::core::PWSTR,
    pub usri11_priv: super::DWORD,
    pub usri11_auth_flags: super::DWORD,
    pub usri11_password_age: super::DWORD,
    pub usri11_home_dir: windows_sys::core::PWSTR,
    pub usri11_parms: windows_sys::core::PWSTR,
    pub usri11_last_logon: super::DWORD,
    pub usri11_last_logoff: super::DWORD,
    pub usri11_bad_pw_count: super::DWORD,
    pub usri11_num_logons: super::DWORD,
    pub usri11_logon_server: windows_sys::core::PWSTR,
    pub usri11_country_code: super::DWORD,
    pub usri11_workstations: windows_sys::core::PWSTR,
    pub usri11_max_storage: super::DWORD,
    pub usri11_units_per_week: super::DWORD,
    pub usri11_logon_hours: super::PBYTE,
    pub usri11_code_page: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_2 {
    pub usri2_name: windows_sys::core::PWSTR,
    pub usri2_password: windows_sys::core::PWSTR,
    pub usri2_password_age: super::DWORD,
    pub usri2_priv: super::DWORD,
    pub usri2_home_dir: windows_sys::core::PWSTR,
    pub usri2_comment: windows_sys::core::PWSTR,
    pub usri2_flags: super::DWORD,
    pub usri2_script_path: windows_sys::core::PWSTR,
    pub usri2_auth_flags: super::DWORD,
    pub usri2_full_name: windows_sys::core::PWSTR,
    pub usri2_usr_comment: windows_sys::core::PWSTR,
    pub usri2_parms: windows_sys::core::PWSTR,
    pub usri2_workstations: windows_sys::core::PWSTR,
    pub usri2_last_logon: super::DWORD,
    pub usri2_last_logoff: super::DWORD,
    pub usri2_acct_expires: super::DWORD,
    pub usri2_max_storage: super::DWORD,
    pub usri2_units_per_week: super::DWORD,
    pub usri2_logon_hours: super::PBYTE,
    pub usri2_bad_pw_count: super::DWORD,
    pub usri2_num_logons: super::DWORD,
    pub usri2_logon_server: windows_sys::core::PWSTR,
    pub usri2_country_code: super::DWORD,
    pub usri2_code_page: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_20 {
    pub usri20_name: windows_sys::core::PWSTR,
    pub usri20_full_name: windows_sys::core::PWSTR,
    pub usri20_comment: windows_sys::core::PWSTR,
    pub usri20_flags: super::DWORD,
    pub usri20_user_id: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct USER_INFO_21 {
    pub usri21_password: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for USER_INFO_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct USER_INFO_22 {
    pub usri22_name: windows_sys::core::PWSTR,
    pub usri22_password: [super::BYTE; 16],
    pub usri22_password_age: super::DWORD,
    pub usri22_priv: super::DWORD,
    pub usri22_home_dir: windows_sys::core::PWSTR,
    pub usri22_comment: windows_sys::core::PWSTR,
    pub usri22_flags: super::DWORD,
    pub usri22_script_path: windows_sys::core::PWSTR,
    pub usri22_auth_flags: super::DWORD,
    pub usri22_full_name: windows_sys::core::PWSTR,
    pub usri22_usr_comment: windows_sys::core::PWSTR,
    pub usri22_parms: windows_sys::core::PWSTR,
    pub usri22_workstations: windows_sys::core::PWSTR,
    pub usri22_last_logon: super::DWORD,
    pub usri22_last_logoff: super::DWORD,
    pub usri22_acct_expires: super::DWORD,
    pub usri22_max_storage: super::DWORD,
    pub usri22_units_per_week: super::DWORD,
    pub usri22_logon_hours: super::PBYTE,
    pub usri22_bad_pw_count: super::DWORD,
    pub usri22_num_logons: super::DWORD,
    pub usri22_logon_server: windows_sys::core::PWSTR,
    pub usri22_country_code: super::DWORD,
    pub usri22_code_page: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for USER_INFO_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_23 {
    pub usri23_name: windows_sys::core::PWSTR,
    pub usri23_full_name: windows_sys::core::PWSTR,
    pub usri23_comment: windows_sys::core::PWSTR,
    pub usri23_flags: super::DWORD,
    pub usri23_user_sid: super::PSID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_24 {
    pub usri24_internet_identity: windows_sys::core::BOOL,
    pub usri24_flags: super::DWORD,
    pub usri24_internet_provider_name: windows_sys::core::PWSTR,
    pub usri24_internet_principal_name: windows_sys::core::PWSTR,
    pub usri24_user_sid: super::PSID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_3 {
    pub usri3_name: windows_sys::core::PWSTR,
    pub usri3_password: windows_sys::core::PWSTR,
    pub usri3_password_age: super::DWORD,
    pub usri3_priv: super::DWORD,
    pub usri3_home_dir: windows_sys::core::PWSTR,
    pub usri3_comment: windows_sys::core::PWSTR,
    pub usri3_flags: super::DWORD,
    pub usri3_script_path: windows_sys::core::PWSTR,
    pub usri3_auth_flags: super::DWORD,
    pub usri3_full_name: windows_sys::core::PWSTR,
    pub usri3_usr_comment: windows_sys::core::PWSTR,
    pub usri3_parms: windows_sys::core::PWSTR,
    pub usri3_workstations: windows_sys::core::PWSTR,
    pub usri3_last_logon: super::DWORD,
    pub usri3_last_logoff: super::DWORD,
    pub usri3_acct_expires: super::DWORD,
    pub usri3_max_storage: super::DWORD,
    pub usri3_units_per_week: super::DWORD,
    pub usri3_logon_hours: super::PBYTE,
    pub usri3_bad_pw_count: super::DWORD,
    pub usri3_num_logons: super::DWORD,
    pub usri3_logon_server: windows_sys::core::PWSTR,
    pub usri3_country_code: super::DWORD,
    pub usri3_code_page: super::DWORD,
    pub usri3_user_id: super::DWORD,
    pub usri3_primary_group_id: super::DWORD,
    pub usri3_profile: windows_sys::core::PWSTR,
    pub usri3_home_dir_drive: windows_sys::core::PWSTR,
    pub usri3_password_expired: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct USER_INFO_4 {
    pub usri4_name: windows_sys::core::PWSTR,
    pub usri4_password: windows_sys::core::PWSTR,
    pub usri4_password_age: super::DWORD,
    pub usri4_priv: super::DWORD,
    pub usri4_home_dir: windows_sys::core::PWSTR,
    pub usri4_comment: windows_sys::core::PWSTR,
    pub usri4_flags: super::DWORD,
    pub usri4_script_path: windows_sys::core::PWSTR,
    pub usri4_auth_flags: super::DWORD,
    pub usri4_full_name: windows_sys::core::PWSTR,
    pub usri4_usr_comment: windows_sys::core::PWSTR,
    pub usri4_parms: windows_sys::core::PWSTR,
    pub usri4_workstations: windows_sys::core::PWSTR,
    pub usri4_last_logon: super::DWORD,
    pub usri4_last_logoff: super::DWORD,
    pub usri4_acct_expires: super::DWORD,
    pub usri4_max_storage: super::DWORD,
    pub usri4_units_per_week: super::DWORD,
    pub usri4_logon_hours: super::PBYTE,
    pub usri4_bad_pw_count: super::DWORD,
    pub usri4_num_logons: super::DWORD,
    pub usri4_logon_server: windows_sys::core::PWSTR,
    pub usri4_country_code: super::DWORD,
    pub usri4_code_page: super::DWORD,
    pub usri4_user_sid: super::PSID,
    pub usri4_primary_group_id: super::DWORD,
    pub usri4_profile: windows_sys::core::PWSTR,
    pub usri4_home_dir_drive: windows_sys::core::PWSTR,
    pub usri4_password_expired: super::DWORD,
}
pub const USER_LAST_LOGOFF_INFOLEVEL: i32 = 1016;
pub const USER_LAST_LOGOFF_PARMNUM: i32 = 16;
pub const USER_LAST_LOGON_INFOLEVEL: i32 = 1015;
pub const USER_LAST_LOGON_PARMNUM: i32 = 15;
pub const USER_LOGON_HOURS_INFOLEVEL: i32 = 1020;
pub const USER_LOGON_HOURS_PARMNUM: i32 = 20;
pub const USER_LOGON_SERVER_INFOLEVEL: i32 = 1023;
pub const USER_LOGON_SERVER_PARMNUM: i32 = 23;
pub const USER_MAXSTORAGE_UNLIMITED: u32 = 4294967295;
pub const USER_MAX_STORAGE_INFOLEVEL: i32 = 1018;
pub const USER_MAX_STORAGE_PARMNUM: i32 = 18;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_0 {
    pub usrmod0_min_passwd_len: super::DWORD,
    pub usrmod0_max_passwd_age: super::DWORD,
    pub usrmod0_min_passwd_age: super::DWORD,
    pub usrmod0_force_logoff: super::DWORD,
    pub usrmod0_password_hist_len: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_1 {
    pub usrmod1_role: super::DWORD,
    pub usrmod1_primary: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_1001 {
    pub usrmod1001_min_passwd_len: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_1002 {
    pub usrmod1002_max_passwd_age: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_1003 {
    pub usrmod1003_min_passwd_age: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_1004 {
    pub usrmod1004_force_logoff: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_1005 {
    pub usrmod1005_password_hist_len: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_1006 {
    pub usrmod1006_role: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_1007 {
    pub usrmod1007_primary: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_2 {
    pub usrmod2_domain_name: windows_sys::core::PWSTR,
    pub usrmod2_domain_id: super::PSID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USER_MODALS_INFO_3 {
    pub usrmod3_lockout_duration: super::DWORD,
    pub usrmod3_lockout_observation_window: super::DWORD,
    pub usrmod3_lockout_threshold: super::DWORD,
}
pub const USER_NAME_INFOLEVEL: i32 = 1001;
pub const USER_NAME_PARMNUM: i32 = 1;
pub const USER_NO_LOGOFF: u32 = 4294967295;
pub const USER_NUM_LOGONS_INFOLEVEL: i32 = 1022;
pub const USER_NUM_LOGONS_PARMNUM: i32 = 22;
pub const USER_PAD_PW_COUNT_INFOLEVEL: i32 = 1021;
pub const USER_PAD_PW_COUNT_PARMNUM: i32 = 21;
pub const USER_PARMS_INFOLEVEL: i32 = 1013;
pub const USER_PARMS_PARMNUM: i32 = 13;
pub const USER_PASSWORD_AGE_INFOLEVEL: i32 = 1004;
pub const USER_PASSWORD_AGE_PARMNUM: i32 = 4;
pub const USER_PASSWORD_INFOLEVEL: i32 = 1003;
pub const USER_PASSWORD_PARMNUM: i32 = 3;
pub const USER_PRIMARY_GROUP_INFOLEVEL: i32 = 1051;
pub const USER_PRIMARY_GROUP_PARMNUM: i32 = 51;
pub const USER_PRIV_ADMIN: i32 = 2;
pub const USER_PRIV_GUEST: i32 = 0;
pub const USER_PRIV_INFOLEVEL: i32 = 1005;
pub const USER_PRIV_MASK: i32 = 3;
pub const USER_PRIV_PARMNUM: i32 = 5;
pub const USER_PRIV_USER: i32 = 1;
pub const USER_PROFILE: i32 = 52;
pub const USER_PROFILE_PARMNUM: i32 = 52;
pub const USER_SCRIPT_PATH_INFOLEVEL: i32 = 1009;
pub const USER_SCRIPT_PATH_PARMNUM: i32 = 9;
pub const USER_UNITS_PER_WEEK_INFOLEVEL: i32 = 1019;
pub const USER_UNITS_PER_WEEK_PARMNUM: i32 = 19;
pub const USER_USR_COMMENT_INFOLEVEL: i32 = 1012;
pub const USER_USR_COMMENT_PARMNUM: i32 = 12;
pub const USER_WORKSTATIONS_INFOLEVEL: i32 = 1014;
pub const USER_WORKSTATIONS_PARMNUM: i32 = 14;
pub const VALIDATED_LOGON: i32 = 0;
pub const VALID_LOGOFF: i32 = 1;
