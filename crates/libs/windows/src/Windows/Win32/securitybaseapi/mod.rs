#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AccessCheck(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, clienttoken: super::HANDLE, desiredaccess: u32, genericmapping: *const super::GENERIC_MAPPING, privilegeset: Option<*mut super::PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AccessCheck(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : u32, genericmapping : *const super::GENERIC_MAPPING, privilegeset : *mut super::PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccess : *mut u32, accessstatus : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AccessCheck(psecuritydescriptor, clienttoken, desiredaccess, genericmapping, privilegeset.unwrap_or(core::mem::zeroed()) as _, privilegesetlength as _, grantedaccess as _, accessstatus as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmW<P0, P2, P3>(subsystemname: P0, handleid: Option<*const core::ffi::c_void>, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, desiredaccess: u32, genericmapping: *const super::GENERIC_MAPPING, objectcreation: bool, grantedaccess: *mut u32, accessstatus: *mut windows_core::BOOL, pfgenerateonclose: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : *const super::GENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccess : *mut u32, accessstatus : *mut windows_core::BOOL, pfgenerateonclose : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AccessCheckAndAuditAlarmW(subsystemname.param().abi(), handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, desiredaccess, genericmapping, objectcreation.into(), grantedaccess as _, accessstatus as _, pfgenerateonclose as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AccessCheckByType(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, clienttoken: super::HANDLE, desiredaccess: u32, objecttypelist: Option<&mut [super::OBJECT_TYPE_LIST]>, genericmapping: *const super::GENERIC_MAPPING, privilegeset: Option<*mut super::PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByType(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, clienttoken : super::HANDLE, desiredaccess : u32, objecttypelist : *mut super::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::GENERIC_MAPPING, privilegeset : *mut super::PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccess : *mut u32, accessstatus : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByType(psecuritydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, clienttoken, desiredaccess, objecttypelist.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, privilegeset.unwrap_or(core::mem::zeroed()) as _, privilegesetlength as _, grantedaccess as _, accessstatus as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmW<P0, P2, P3>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: u32, audittype: super::AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<&mut [super::OBJECT_TYPE_LIST]>, genericmapping: *const super::GENERIC_MAPPING, objectcreation: bool, grantedaccess: *mut u32, accessstatus: *mut windows_core::BOOL, pfgenerateonclose: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::GENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccess : *mut u32, accessstatus : *mut windows_core::BOOL, pfgenerateonclose : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeAndAuditAlarmW(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, objectcreation.into(), grantedaccess as _, accessstatus as _, pfgenerateonclose as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AccessCheckByTypeResultList(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, clienttoken: super::HANDLE, desiredaccess: u32, objecttypelist: Option<*mut super::OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const super::GENERIC_MAPPING, privilegeset: Option<*mut super::PRIVILEGE_SET>, privilegesetlength: *mut u32, grantedaccesslist: *mut u32, accessstatuslist: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeResultList(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, clienttoken : super::HANDLE, desiredaccess : u32, objecttypelist : *mut super::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::GENERIC_MAPPING, privilegeset : *mut super::PRIVILEGE_SET, privilegesetlength : *mut u32, grantedaccesslist : *mut u32, accessstatuslist : *mut u32) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeResultList(psecuritydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, clienttoken, desiredaccess, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, privilegeset.unwrap_or(core::mem::zeroed()) as _, privilegesetlength as _, grantedaccesslist as _, accessstatuslist as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleW<P0, P3, P4>(subsystemname: P0, handleid: *const core::ffi::c_void, clienttoken: super::HANDLE, objecttypename: P3, objectname: P4, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: u32, audittype: super::AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<*mut super::OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const super::GENERIC_MAPPING, objectcreation: bool, grantedaccesslist: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, clienttoken : super::HANDLE, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::GENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccesslist : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname.param().abi(), handleid, clienttoken, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation.into(), grantedaccesslist as _, accessstatuslist as _, pfgenerateonclose as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmW<P0, P2, P3>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: u32, audittype: super::AUDIT_EVENT_TYPE, flags: u32, objecttypelist: Option<*mut super::OBJECT_TYPE_LIST>, objecttypelistlength: u32, genericmapping: *const super::GENERIC_MAPPING, objectcreation: bool, grantedaccesslist: *mut u32, accessstatuslist: *mut u32, pfgenerateonclose: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::GENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccesslist : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeResultListAndAuditAlarmW(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation.into(), grantedaccesslist as _, accessstatuslist as _, pfgenerateonclose as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAccessAllowedAce(pacl: *mut super::ACL, dwacerevision: u32, accessmask: u32, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessAllowedAce(pacl : *mut super::ACL, dwacerevision : u32, accessmask : u32, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessAllowedAce(pacl as _, dwacerevision, accessmask, psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAccessAllowedAceEx(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, accessmask: u32, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessAllowedAceEx(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessAllowedAceEx(pacl as _, dwacerevision, aceflags, accessmask, psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAccessAllowedObjectAce(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, accessmask: u32, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessAllowedObjectAce(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessAllowedObjectAce(pacl as _, dwacerevision, aceflags, accessmask, objecttypeguid.unwrap_or(core::mem::zeroed()) as _, inheritedobjecttypeguid.unwrap_or(core::mem::zeroed()) as _, psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAccessDeniedAce(pacl: *mut super::ACL, dwacerevision: u32, accessmask: u32, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessDeniedAce(pacl : *mut super::ACL, dwacerevision : u32, accessmask : u32, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessDeniedAce(pacl as _, dwacerevision, accessmask, psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAccessDeniedAceEx(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, accessmask: u32, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessDeniedAceEx(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessDeniedAceEx(pacl as _, dwacerevision, aceflags, accessmask, psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAccessDeniedObjectAce(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, accessmask: u32, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessDeniedObjectAce(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessDeniedObjectAce(pacl as _, dwacerevision, aceflags, accessmask, objecttypeguid.unwrap_or(core::mem::zeroed()) as _, inheritedobjecttypeguid.unwrap_or(core::mem::zeroed()) as _, psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAce(pacl: *mut super::ACL, dwacerevision: u32, dwstartingaceindex: u32, pacelist: *const core::ffi::c_void, nacelistlength: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAce(pacl : *mut super::ACL, dwacerevision : u32, dwstartingaceindex : u32, pacelist : *const core::ffi::c_void, nacelistlength : u32) -> windows_core::BOOL);
    unsafe { AddAce(pacl as _, dwacerevision, dwstartingaceindex, pacelist, nacelistlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAuditAccessAce(pacl: *mut super::ACL, dwacerevision: u32, dwaccessmask: u32, psid: super::PSID, bauditsuccess: bool, bauditfailure: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAuditAccessAce(pacl : *mut super::ACL, dwacerevision : u32, dwaccessmask : u32, psid : super::PSID, bauditsuccess : windows_core::BOOL, bauditfailure : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AddAuditAccessAce(pacl as _, dwacerevision, dwaccessmask, psid, bauditsuccess.into(), bauditfailure.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAuditAccessAceEx(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, dwaccessmask: u32, psid: super::PSID, bauditsuccess: bool, bauditfailure: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAuditAccessAceEx(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, dwaccessmask : u32, psid : super::PSID, bauditsuccess : windows_core::BOOL, bauditfailure : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AddAuditAccessAceEx(pacl as _, dwacerevision, aceflags, dwaccessmask, psid, bauditsuccess.into(), bauditfailure.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddAuditAccessObjectAce(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, accessmask: u32, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: super::PSID, bauditsuccess: bool, bauditfailure: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAuditAccessObjectAce(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::PSID, bauditsuccess : windows_core::BOOL, bauditfailure : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AddAuditAccessObjectAce(pacl as _, dwacerevision, aceflags, accessmask, objecttypeguid.unwrap_or(core::mem::zeroed()) as _, inheritedobjecttypeguid.unwrap_or(core::mem::zeroed()) as _, psid, bauditsuccess.into(), bauditfailure.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddMandatoryAce(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, mandatorypolicy: u32, plabelsid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddMandatoryAce(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, mandatorypolicy : u32, plabelsid : super::PSID) -> windows_core::BOOL);
    unsafe { AddMandatoryAce(pacl as _, dwacerevision, aceflags, mandatorypolicy, plabelsid) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn AddResourceAttributeAce(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, accessmask: u32, psid: super::PSID, pattributeinfo: *const super::CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AddResourceAttributeAce(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::PSID, pattributeinfo : *const super::CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength : *mut u32) -> windows_core::BOOL);
    unsafe { AddResourceAttributeAce(pacl as _, dwacerevision, aceflags, accessmask, psid, pattributeinfo, preturnlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddScopedPolicyIDAce(pacl: *mut super::ACL, dwacerevision: u32, aceflags: u32, accessmask: u32, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AddScopedPolicyIDAce(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, accessmask : u32, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddScopedPolicyIDAce(pacl as _, dwacerevision, aceflags, accessmask, psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AdjustTokenGroups(tokenhandle: super::HANDLE, resettodefault: bool, newstate: Option<*const super::TOKEN_GROUPS>, bufferlength: u32, previousstate: Option<*mut super::TOKEN_GROUPS>, returnlength: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AdjustTokenGroups(tokenhandle : super::HANDLE, resettodefault : windows_core::BOOL, newstate : *const super::TOKEN_GROUPS, bufferlength : u32, previousstate : *mut super::TOKEN_GROUPS, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe { AdjustTokenGroups(tokenhandle, resettodefault.into(), newstate.unwrap_or(core::mem::zeroed()) as _, bufferlength, previousstate.unwrap_or(core::mem::zeroed()) as _, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AdjustTokenPrivileges(tokenhandle: super::HANDLE, disableallprivileges: bool, newstate: Option<*const super::TOKEN_PRIVILEGES>, bufferlength: u32, previousstate: Option<*mut super::TOKEN_PRIVILEGES>, returnlength: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AdjustTokenPrivileges(tokenhandle : super::HANDLE, disableallprivileges : windows_core::BOOL, newstate : *const super::TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut super::TOKEN_PRIVILEGES, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe { AdjustTokenPrivileges(tokenhandle, disableallprivileges.into(), newstate.unwrap_or(core::mem::zeroed()) as _, bufferlength, previousstate.unwrap_or(core::mem::zeroed()) as _, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocateAndInitializeSid(pidentifierauthority: *const super::SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AllocateAndInitializeSid(pidentifierauthority : *const super::SID_IDENTIFIER_AUTHORITY, nsubauthoritycount : u8, nsubauthority0 : u32, nsubauthority1 : u32, nsubauthority2 : u32, nsubauthority3 : u32, nsubauthority4 : u32, nsubauthority5 : u32, nsubauthority6 : u32, nsubauthority7 : u32, psid : *mut super::PSID) -> windows_core::BOOL);
    unsafe { AllocateAndInitializeSid(pidentifierauthority, nsubauthoritycount, nsubauthority0, nsubauthority1, nsubauthority2, nsubauthority3, nsubauthority4, nsubauthority5, nsubauthority6, nsubauthority7, psid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocateLocallyUniqueId(luid: *mut super::LUID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AllocateLocallyUniqueId(luid : *mut super::LUID) -> windows_core::BOOL);
    unsafe { AllocateLocallyUniqueId(luid as _) }
}
#[inline]
pub unsafe fn AreAllAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AreAllAccessesGranted(grantedaccess : u32, desiredaccess : u32) -> windows_core::BOOL);
    unsafe { AreAllAccessesGranted(grantedaccess, desiredaccess) }
}
#[inline]
pub unsafe fn AreAnyAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AreAnyAccessesGranted(grantedaccess : u32, desiredaccess : u32) -> windows_core::BOOL);
    unsafe { AreAnyAccessesGranted(grantedaccess, desiredaccess) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CheckTokenCapability(tokenhandle: Option<super::HANDLE>, capabilitysidtocheck: super::PSID, hascapability: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CheckTokenCapability(tokenhandle : super::HANDLE, capabilitysidtocheck : super::PSID, hascapability : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CheckTokenCapability(tokenhandle.unwrap_or(core::mem::zeroed()) as _, capabilitysidtocheck, hascapability as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CheckTokenMembership(tokenhandle: Option<super::HANDLE>, sidtocheck: super::PSID, ismember: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CheckTokenMembership(tokenhandle : super::HANDLE, sidtocheck : super::PSID, ismember : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CheckTokenMembership(tokenhandle.unwrap_or(core::mem::zeroed()) as _, sidtocheck, ismember as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CheckTokenMembershipEx(tokenhandle: Option<super::HANDLE>, sidtocheck: super::PSID, flags: u32, ismember: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CheckTokenMembershipEx(tokenhandle : super::HANDLE, sidtocheck : super::PSID, flags : u32, ismember : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CheckTokenMembershipEx(tokenhandle.unwrap_or(core::mem::zeroed()) as _, sidtocheck, flags, ismember as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor: Option<super::PSECURITY_DESCRIPTOR>, currentsecuritydescriptor: super::PSECURITY_DESCRIPTOR, newsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, objecttype: Option<*const windows_core::GUID>, isdirectoryobject: bool, genericmapping: *const super::GENERIC_MAPPING) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor : super::PSECURITY_DESCRIPTOR, currentsecuritydescriptor : super::PSECURITY_DESCRIPTOR, newsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttype : *const windows_core::GUID, isdirectoryobject : bool, genericmapping : *const super::GENERIC_MAPPING) -> windows_core::BOOL);
    unsafe { ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor.unwrap_or(core::mem::zeroed()) as _, currentsecuritydescriptor, newsecuritydescriptor as _, objecttype.unwrap_or(core::mem::zeroed()) as _, isdirectoryobject, genericmapping) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CopySid(ndestinationsidlength: u32, pdestinationsid: super::PSID, psourcesid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CopySid(ndestinationsidlength : u32, pdestinationsid : super::PSID, psourcesid : super::PSID) -> windows_core::BOOL);
    unsafe { CopySid(ndestinationsidlength, pdestinationsid as _, psourcesid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurity(parentdescriptor: Option<super::PSECURITY_DESCRIPTOR>, creatordescriptor: Option<super::PSECURITY_DESCRIPTOR>, newdescriptor: *mut super::PSECURITY_DESCRIPTOR, isdirectoryobject: bool, token: Option<super::HANDLE>, genericmapping: *const super::GENERIC_MAPPING) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurity(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, isdirectoryobject : windows_core::BOOL, token : super::HANDLE, genericmapping : *const super::GENERIC_MAPPING) -> windows_core::BOOL);
    unsafe { CreatePrivateObjectSecurity(parentdescriptor.unwrap_or(core::mem::zeroed()) as _, creatordescriptor.unwrap_or(core::mem::zeroed()) as _, newdescriptor as _, isdirectoryobject.into(), token.unwrap_or(core::mem::zeroed()) as _, genericmapping) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurityEx(parentdescriptor: Option<super::PSECURITY_DESCRIPTOR>, creatordescriptor: Option<super::PSECURITY_DESCRIPTOR>, newdescriptor: *mut super::PSECURITY_DESCRIPTOR, objecttype: Option<*const windows_core::GUID>, iscontainerobject: bool, autoinheritflags: u32, token: Option<super::HANDLE>, genericmapping: *const super::GENERIC_MAPPING) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityEx(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttype : *const windows_core::GUID, iscontainerobject : windows_core::BOOL, autoinheritflags : u32, token : super::HANDLE, genericmapping : *const super::GENERIC_MAPPING) -> windows_core::BOOL);
    unsafe { CreatePrivateObjectSecurityEx(parentdescriptor.unwrap_or(core::mem::zeroed()) as _, creatordescriptor.unwrap_or(core::mem::zeroed()) as _, newdescriptor as _, objecttype.unwrap_or(core::mem::zeroed()) as _, iscontainerobject.into(), autoinheritflags, token.unwrap_or(core::mem::zeroed()) as _, genericmapping) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor: Option<super::PSECURITY_DESCRIPTOR>, creatordescriptor: Option<super::PSECURITY_DESCRIPTOR>, newdescriptor: *mut super::PSECURITY_DESCRIPTOR, objecttypes: Option<&[*const windows_core::GUID]>, iscontainerobject: bool, autoinheritflags: u32, token: Option<super::HANDLE>, genericmapping: *const super::GENERIC_MAPPING) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttypes : *const *const windows_core::GUID, guidcount : u32, iscontainerobject : windows_core::BOOL, autoinheritflags : u32, token : super::HANDLE, genericmapping : *const super::GENERIC_MAPPING) -> windows_core::BOOL);
    unsafe { CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor.unwrap_or(core::mem::zeroed()) as _, creatordescriptor.unwrap_or(core::mem::zeroed()) as _, newdescriptor as _, objecttypes.map_or(core::ptr::null(), |slice| slice.as_ptr()), objecttypes.map_or(0, |slice| slice.len().try_into().unwrap()), iscontainerobject.into(), autoinheritflags, token.unwrap_or(core::mem::zeroed()) as _, genericmapping) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateRestrictedToken(existingtokenhandle: super::HANDLE, flags: u32, sidstodisable: Option<&[super::SID_AND_ATTRIBUTES]>, privilegestodelete: Option<&[super::LUID_AND_ATTRIBUTES]>, sidstorestrict: Option<&[super::SID_AND_ATTRIBUTES]>, newtokenhandle: *mut super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreateRestrictedToken(existingtokenhandle : super::HANDLE, flags : u32, disablesidcount : u32, sidstodisable : *const super::SID_AND_ATTRIBUTES, deleteprivilegecount : u32, privilegestodelete : *const super::LUID_AND_ATTRIBUTES, restrictedsidcount : u32, sidstorestrict : *const super::SID_AND_ATTRIBUTES, newtokenhandle : *mut super::HANDLE) -> windows_core::BOOL);
    unsafe { CreateRestrictedToken(existingtokenhandle, flags, sidstodisable.map_or(0, |slice| slice.len().try_into().unwrap()), sidstodisable.map_or(core::ptr::null(), |slice| slice.as_ptr()), privilegestodelete.map_or(0, |slice| slice.len().try_into().unwrap()), privilegestodelete.map_or(core::ptr::null(), |slice| slice.as_ptr()), sidstorestrict.map_or(0, |slice| slice.len().try_into().unwrap()), sidstorestrict.map_or(core::ptr::null(), |slice| slice.as_ptr()), newtokenhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateWellKnownSid(wellknownsidtype: super::WELL_KNOWN_SID_TYPE, domainsid: Option<super::PSID>, psid: Option<super::PSID>, cbsid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreateWellKnownSid(wellknownsidtype : super::WELL_KNOWN_SID_TYPE, domainsid : super::PSID, psid : super::PSID, cbsid : *mut u32) -> windows_core::BOOL);
    unsafe { CreateWellKnownSid(wellknownsidtype, domainsid.unwrap_or(core::mem::zeroed()) as _, psid.unwrap_or(core::mem::zeroed()) as _, cbsid as _) }
}
#[inline]
pub unsafe fn CveEventWrite<P0, P1>(cveid: P0, additionaldetails: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CveEventWrite(cveid : windows_core::PCWSTR, additionaldetails : windows_core::PCWSTR) -> i32);
    unsafe { CveEventWrite(cveid.param().abi(), additionaldetails.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteAce(pacl: *mut super::ACL, dwaceindex: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DeleteAce(pacl : *mut super::ACL, dwaceindex : u32) -> windows_core::BOOL);
    unsafe { DeleteAce(pacl as _, dwaceindex) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeriveCapabilitySidsFromName<P0>(capname: P0, capabilitygroupsids: *mut *mut super::PSID, capabilitygroupsidcount: *mut u32, capabilitysids: *mut *mut super::PSID, capabilitysidcount: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-security-base-l1-2-2.dll" "system" fn DeriveCapabilitySidsFromName(capname : windows_core::PCWSTR, capabilitygroupsids : *mut *mut super::PSID, capabilitygroupsidcount : *mut u32, capabilitysids : *mut *mut super::PSID, capabilitysidcount : *mut u32) -> windows_core::BOOL);
    unsafe { DeriveCapabilitySidsFromName(capname.param().abi(), capabilitygroupsids as _, capabilitygroupsidcount as _, capabilitysids as _, capabilitysidcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DestroyPrivateObjectSecurity(objectdescriptor: *mut super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DestroyPrivateObjectSecurity(objectdescriptor : *mut super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { DestroyPrivateObjectSecurity(objectdescriptor as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DuplicateToken(existingtokenhandle: super::HANDLE, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle: *mut super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DuplicateToken(existingtokenhandle : super::HANDLE, impersonationlevel : super::SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle : *mut super::HANDLE) -> windows_core::BOOL);
    unsafe { DuplicateToken(existingtokenhandle, impersonationlevel, duplicatetokenhandle as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn DuplicateTokenEx(hexistingtoken: super::HANDLE, dwdesiredaccess: u32, lptokenattributes: Option<*const super::SECURITY_ATTRIBUTES>, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokentype: super::TOKEN_TYPE, phnewtoken: *mut super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DuplicateTokenEx(hexistingtoken : super::HANDLE, dwdesiredaccess : u32, lptokenattributes : *const super::SECURITY_ATTRIBUTES, impersonationlevel : super::SECURITY_IMPERSONATION_LEVEL, tokentype : super::TOKEN_TYPE, phnewtoken : *mut super::HANDLE) -> windows_core::BOOL);
    unsafe { DuplicateTokenEx(hexistingtoken, dwdesiredaccess, lptokenattributes.unwrap_or(core::mem::zeroed()) as _, impersonationlevel, tokentype, phnewtoken as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EqualDomainSid(psid1: super::PSID, psid2: super::PSID, pfequal: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn EqualDomainSid(psid1 : super::PSID, psid2 : super::PSID, pfequal : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EqualDomainSid(psid1, psid2, pfequal as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EqualPrefixSid(psid1: super::PSID, psid2: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn EqualPrefixSid(psid1 : super::PSID, psid2 : super::PSID) -> windows_core::BOOL);
    unsafe { EqualPrefixSid(psid1, psid2) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EqualSid(psid1: super::PSID, psid2: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn EqualSid(psid1 : super::PSID, psid2 : super::PSID) -> windows_core::BOOL);
    unsafe { EqualSid(psid1, psid2) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFirstFreeAce(pacl: *const super::ACL, pace: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn FindFirstFreeAce(pacl : *const super::ACL, pace : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindFirstFreeAce(pacl, pace as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FreeSid(psid: super::PSID) -> *mut core::ffi::c_void {
    windows_core::link!("advapi32.dll" "system" fn FreeSid(psid : super::PSID) -> *mut core::ffi::c_void);
    unsafe { FreeSid(psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetAce(pacl: *const super::ACL, dwaceindex: u32, pace: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetAce(pacl : *const super::ACL, dwaceindex : u32, pace : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetAce(pacl, dwaceindex, pace as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetAclInformation(pacl: *const super::ACL, paclinformation: *mut core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: super::ACL_INFORMATION_CLASS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetAclInformation(pacl : *const super::ACL, paclinformation : *mut core::ffi::c_void, naclinformationlength : u32, dwaclinformationclass : super::ACL_INFORMATION_CLASS) -> windows_core::BOOL);
    unsafe { GetAclInformation(pacl, paclinformation as _, naclinformationlength, dwaclinformationclass) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetAppContainerAce(acl: *const super::ACL, startingaceindex: u32, appcontainerace: *mut *mut core::ffi::c_void, appcontaineraceindex: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetAppContainerAce(acl : *const super::ACL, startingaceindex : u32, appcontainerace : *mut *mut core::ffi::c_void, appcontaineraceindex : *mut u32) -> windows_core::BOOL);
    unsafe { GetAppContainerAce(acl, startingaceindex, appcontainerace as _, appcontaineraceindex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetCachedSigningLevel(file: super::HANDLE, flags: *mut u32, signinglevel: *mut u32, thumbprint: Option<*mut u8>, thumbprintsize: Option<*mut u32>, thumbprintalgorithm: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCachedSigningLevel(file : super::HANDLE, flags : *mut u32, signinglevel : *mut u32, thumbprint : *mut u8, thumbprintsize : *mut u32, thumbprintalgorithm : *mut u32) -> windows_core::BOOL);
    unsafe { GetCachedSigningLevel(file, flags as _, signinglevel as _, thumbprint.unwrap_or(core::mem::zeroed()) as _, thumbprintsize.unwrap_or(core::mem::zeroed()) as _, thumbprintalgorithm.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetFileSecurityW<P0>(lpfilename: P0, requestedinformation: super::SECURITY_INFORMATION, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetFileSecurityW(lpfilename : windows_core::PCWSTR, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetFileSecurityW(lpfilename.param().abi(), requestedinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetKernelObjectSecurity(handle: super::HANDLE, requestedinformation: super::SECURITY_INFORMATION, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetKernelObjectSecurity(handle : super::HANDLE, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetKernelObjectSecurity(handle, requestedinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetLengthSid(psid: super::PSID) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetLengthSid(psid : super::PSID) -> u32);
    unsafe { GetLengthSid(psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetPrivateObjectSecurity(objectdescriptor: super::PSECURITY_DESCRIPTOR, securityinformation: super::SECURITY_INFORMATION, resultantdescriptor: Option<super::PSECURITY_DESCRIPTOR>, descriptorlength: u32, returnlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetPrivateObjectSecurity(objectdescriptor : super::PSECURITY_DESCRIPTOR, securityinformation : super::SECURITY_INFORMATION, resultantdescriptor : super::PSECURITY_DESCRIPTOR, descriptorlength : u32, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetPrivateObjectSecurity(objectdescriptor, securityinformation, resultantdescriptor.unwrap_or(core::mem::zeroed()) as _, descriptorlength, returnlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSecurityDescriptorControl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pcontrol: *mut u16, lpdwrevision: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorControl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pcontrol : *mut u16, lpdwrevision : *mut u32) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorControl(psecuritydescriptor, pcontrol as _, lpdwrevision as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSecurityDescriptorDacl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, lpbdaclpresent: *mut windows_core::BOOL, pdacl: *mut super::PACL, lpbdacldefaulted: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorDacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpbdaclpresent : *mut windows_core::BOOL, pdacl : *mut super::PACL, lpbdacldefaulted : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorDacl(psecuritydescriptor, lpbdaclpresent as _, pdacl as _, lpbdacldefaulted as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSecurityDescriptorGroup(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pgroup: *mut super::PSID, lpbgroupdefaulted: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorGroup(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pgroup : *mut super::PSID, lpbgroupdefaulted : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorGroup(psecuritydescriptor, pgroup as _, lpbgroupdefaulted as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSecurityDescriptorLength(psecuritydescriptor: super::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorLength(psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { GetSecurityDescriptorLength(psecuritydescriptor) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSecurityDescriptorOwner(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, powner: *mut super::PSID, lpbownerdefaulted: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorOwner(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, powner : *mut super::PSID, lpbownerdefaulted : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorOwner(psecuritydescriptor, powner as _, lpbownerdefaulted as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSecurityDescriptorRMControl(securitydescriptor: super::PSECURITY_DESCRIPTOR, rmcontrol: *mut u8) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorRMControl(securitydescriptor : super::PSECURITY_DESCRIPTOR, rmcontrol : *mut u8) -> u32);
    unsafe { GetSecurityDescriptorRMControl(securitydescriptor, rmcontrol as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSecurityDescriptorSacl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, lpbsaclpresent: *mut windows_core::BOOL, psacl: *mut super::PACL, lpbsacldefaulted: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorSacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpbsaclpresent : *mut windows_core::BOOL, psacl : *mut super::PACL, lpbsacldefaulted : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorSacl(psecuritydescriptor, lpbsaclpresent as _, psacl as _, lpbsacldefaulted as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSidIdentifierAuthority(psid: super::PSID) -> super::PSID_IDENTIFIER_AUTHORITY {
    windows_core::link!("advapi32.dll" "system" fn GetSidIdentifierAuthority(psid : super::PSID) -> super::PSID_IDENTIFIER_AUTHORITY);
    unsafe { GetSidIdentifierAuthority(psid) }
}
#[inline]
pub unsafe fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetSidLengthRequired(nsubauthoritycount : u8) -> u32);
    unsafe { GetSidLengthRequired(nsubauthoritycount) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetSidSubAuthority(psid: super::PSID, nsubauthority: u32) -> super::PDWORD {
    windows_core::link!("advapi32.dll" "system" fn GetSidSubAuthority(psid : super::PSID, nsubauthority : u32) -> super::PDWORD);
    unsafe { GetSidSubAuthority(psid, nsubauthority) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetSidSubAuthorityCount(psid: super::PSID) -> super::PUCHAR {
    windows_core::link!("advapi32.dll" "system" fn GetSidSubAuthorityCount(psid : super::PSID) -> super::PUCHAR);
    unsafe { GetSidSubAuthorityCount(psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetTokenInformation(tokenhandle: super::HANDLE, tokeninformationclass: super::TOKEN_INFORMATION_CLASS, tokeninformation: Option<*mut core::ffi::c_void>, tokeninformationlength: u32, returnlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetTokenInformation(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetTokenInformation(tokenhandle, tokeninformationclass, tokeninformation.unwrap_or(core::mem::zeroed()) as _, tokeninformationlength, returnlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetWindowsAccountDomainSid(psid: super::PSID, pdomainsid: Option<super::PSID>, cbdomainsid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetWindowsAccountDomainSid(psid : super::PSID, pdomainsid : super::PSID, cbdomainsid : *mut u32) -> windows_core::BOOL);
    unsafe { GetWindowsAccountDomainSid(psid, pdomainsid.unwrap_or(core::mem::zeroed()) as _, cbdomainsid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ImpersonateAnonymousToken(threadhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ImpersonateAnonymousToken(threadhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { ImpersonateAnonymousToken(threadhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ImpersonateLoggedOnUser(htoken: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ImpersonateLoggedOnUser(htoken : super::HANDLE) -> windows_core::BOOL);
    unsafe { ImpersonateLoggedOnUser(htoken) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ImpersonateSelf(impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ImpersonateSelf(impersonationlevel : super::SECURITY_IMPERSONATION_LEVEL) -> windows_core::BOOL);
    unsafe { ImpersonateSelf(impersonationlevel) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeAcl(pacl: *mut super::ACL, nacllength: u32, dwaclrevision: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn InitializeAcl(pacl : *mut super::ACL, nacllength : u32, dwaclrevision : u32) -> windows_core::BOOL);
    unsafe { InitializeAcl(pacl as _, nacllength, dwaclrevision) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeSecurityDescriptor(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, dwrevision: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn InitializeSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, dwrevision : u32) -> windows_core::BOOL);
    unsafe { InitializeSecurityDescriptor(psecuritydescriptor as _, dwrevision) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeSid(sid: super::PSID, pidentifierauthority: *const super::SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn InitializeSid(sid : super::PSID, pidentifierauthority : *const super::SID_IDENTIFIER_AUTHORITY, nsubauthoritycount : u8) -> windows_core::BOOL);
    unsafe { InitializeSid(sid as _, pidentifierauthority, nsubauthoritycount) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsTokenRestricted(tokenhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsTokenRestricted(tokenhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { IsTokenRestricted(tokenhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsValidAcl(pacl: *const super::ACL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsValidAcl(pacl : *const super::ACL) -> windows_core::BOOL);
    unsafe { IsValidAcl(pacl) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsValidSecurityDescriptor(psecuritydescriptor: super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsValidSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { IsValidSecurityDescriptor(psecuritydescriptor) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsValidSid(psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsValidSid(psid : super::PSID) -> windows_core::BOOL);
    unsafe { IsValidSid(psid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsWellKnownSid(psid: super::PSID, wellknownsidtype: super::WELL_KNOWN_SID_TYPE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsWellKnownSid(psid : super::PSID, wellknownsidtype : super::WELL_KNOWN_SID_TYPE) -> windows_core::BOOL);
    unsafe { IsWellKnownSid(psid, wellknownsidtype) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MakeAbsoluteSD(pselfrelativesecuritydescriptor: super::PSECURITY_DESCRIPTOR, pabsolutesecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, lpdwabsolutesecuritydescriptorsize: *mut u32, pdacl: Option<*mut super::ACL>, lpdwdaclsize: *mut u32, psacl: Option<*mut super::ACL>, lpdwsaclsize: *mut u32, powner: Option<super::PSID>, lpdwownersize: *mut u32, pprimarygroup: Option<super::PSID>, lpdwprimarygroupsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn MakeAbsoluteSD(pselfrelativesecuritydescriptor : super::PSECURITY_DESCRIPTOR, pabsolutesecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize : *mut u32, pdacl : *mut super::ACL, lpdwdaclsize : *mut u32, psacl : *mut super::ACL, lpdwsaclsize : *mut u32, powner : super::PSID, lpdwownersize : *mut u32, pprimarygroup : super::PSID, lpdwprimarygroupsize : *mut u32) -> windows_core::BOOL);
    unsafe { MakeAbsoluteSD(pselfrelativesecuritydescriptor, pabsolutesecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, lpdwabsolutesecuritydescriptorsize as _, pdacl.unwrap_or(core::mem::zeroed()) as _, lpdwdaclsize as _, psacl.unwrap_or(core::mem::zeroed()) as _, lpdwsaclsize as _, powner.unwrap_or(core::mem::zeroed()) as _, lpdwownersize as _, pprimarygroup.unwrap_or(core::mem::zeroed()) as _, lpdwprimarygroupsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MakeSelfRelativeSD(pabsolutesecuritydescriptor: super::PSECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, lpdwbufferlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn MakeSelfRelativeSD(pabsolutesecuritydescriptor : super::PSECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { MakeSelfRelativeSD(pabsolutesecuritydescriptor, pselfrelativesecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MapGenericMask(accessmask: *mut u32, genericmapping: *const super::GENERIC_MAPPING) {
    windows_core::link!("advapi32.dll" "system" fn MapGenericMask(accessmask : *mut u32, genericmapping : *const super::GENERIC_MAPPING));
    unsafe { MapGenericMask(accessmask as _, genericmapping) }
}
#[inline]
pub unsafe fn ObjectCloseAuditAlarmW<P0>(subsystemname: P0, handleid: *const core::ffi::c_void, generateonclose: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectCloseAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, generateonclose : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectCloseAuditAlarmW(subsystemname.param().abi(), handleid, generateonclose.into()) }
}
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmW<P0>(subsystemname: P0, handleid: *const core::ffi::c_void, generateonclose: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectDeleteAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, generateonclose : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectDeleteAuditAlarmW(subsystemname.param().abi(), handleid, generateonclose.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ObjectOpenAuditAlarmW<P0, P2, P3>(subsystemname: P0, handleid: *const core::ffi::c_void, objecttypename: P2, objectname: P3, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, clienttoken: super::HANDLE, desiredaccess: u32, grantedaccess: u32, privileges: Option<*const super::PRIVILEGE_SET>, objectcreation: bool, accessgranted: bool, generateonclose: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectOpenAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : *const super::PRIVILEGE_SET, objectcreation : windows_core::BOOL, accessgranted : windows_core::BOOL, generateonclose : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectOpenAuditAlarmW(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), psecuritydescriptor, clienttoken, desiredaccess, grantedaccess, privileges.unwrap_or(core::mem::zeroed()) as _, objectcreation.into(), accessgranted.into(), generateonclose as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmW<P0>(subsystemname: P0, handleid: *const core::ffi::c_void, clienttoken: super::HANDLE, desiredaccess: u32, privileges: *const super::PRIVILEGE_SET, accessgranted: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectPrivilegeAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : *const core::ffi::c_void, clienttoken : super::HANDLE, desiredaccess : u32, privileges : *const super::PRIVILEGE_SET, accessgranted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectPrivilegeAuditAlarmW(subsystemname.param().abi(), handleid, clienttoken, desiredaccess, privileges, accessgranted.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PrivilegeCheck(clienttoken: super::HANDLE, requiredprivileges: *mut super::PRIVILEGE_SET, pfresult: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn PrivilegeCheck(clienttoken : super::HANDLE, requiredprivileges : *mut super::PRIVILEGE_SET, pfresult : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { PrivilegeCheck(clienttoken, requiredprivileges as _, pfresult as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmW<P0, P1>(subsystemname: P0, servicename: P1, clienttoken: super::HANDLE, privileges: *const super::PRIVILEGE_SET, accessgranted: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PrivilegedServiceAuditAlarmW(subsystemname : windows_core::PCWSTR, servicename : windows_core::PCWSTR, clienttoken : super::HANDLE, privileges : *const super::PRIVILEGE_SET, accessgranted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { PrivilegedServiceAuditAlarmW(subsystemname.param().abi(), servicename.param().abi(), clienttoken, privileges, accessgranted.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QuerySecurityAccessMask(securityinformation: super::SECURITY_INFORMATION) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn QuerySecurityAccessMask(securityinformation : super::SECURITY_INFORMATION, desiredaccess : *mut u32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        QuerySecurityAccessMask(securityinformation, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn RevertToSelf() -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn RevertToSelf() -> windows_core::BOOL);
    unsafe { RevertToSelf() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetAclInformation(pacl: *mut super::ACL, paclinformation: *const core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: super::ACL_INFORMATION_CLASS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetAclInformation(pacl : *mut super::ACL, paclinformation : *const core::ffi::c_void, naclinformationlength : u32, dwaclinformationclass : super::ACL_INFORMATION_CLASS) -> windows_core::BOOL);
    unsafe { SetAclInformation(pacl as _, paclinformation, naclinformationlength, dwaclinformationclass) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetCachedSigningLevel(sourcefiles: &[super::HANDLE], flags: u32, targetfile: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCachedSigningLevel(sourcefiles : *const super::HANDLE, sourcefilecount : u32, flags : u32, targetfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetCachedSigningLevel(sourcefiles.as_ptr(), sourcefiles.len().try_into().unwrap(), flags, targetfile.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetFileSecurityW<P0>(lpfilename: P0, securityinformation: super::SECURITY_INFORMATION, psecuritydescriptor: super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetFileSecurityW(lpfilename : windows_core::PCWSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { SetFileSecurityW(lpfilename.param().abi(), securityinformation, psecuritydescriptor) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetKernelObjectSecurity(handle: super::HANDLE, securityinformation: super::SECURITY_INFORMATION, securitydescriptor: super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetKernelObjectSecurity(handle : super::HANDLE, securityinformation : super::SECURITY_INFORMATION, securitydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { SetKernelObjectSecurity(handle, securityinformation, securitydescriptor) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetPrivateObjectSecurity(securityinformation: super::SECURITY_INFORMATION, modificationdescriptor: super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, genericmapping: *const super::GENERIC_MAPPING, token: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetPrivateObjectSecurity(securityinformation : super::SECURITY_INFORMATION, modificationdescriptor : super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, genericmapping : *const super::GENERIC_MAPPING, token : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetPrivateObjectSecurity(securityinformation, modificationdescriptor, objectssecuritydescriptor as _, genericmapping, token.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetPrivateObjectSecurityEx(securityinformation: super::SECURITY_INFORMATION, modificationdescriptor: super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, autoinheritflags: u32, genericmapping: *const super::GENERIC_MAPPING, token: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetPrivateObjectSecurityEx(securityinformation : super::SECURITY_INFORMATION, modificationdescriptor : super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, autoinheritflags : u32, genericmapping : *const super::GENERIC_MAPPING, token : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetPrivateObjectSecurityEx(securityinformation, modificationdescriptor, objectssecuritydescriptor as _, autoinheritflags, genericmapping, token.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetSecurityAccessMask(securityinformation: super::SECURITY_INFORMATION) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityAccessMask(securityinformation : super::SECURITY_INFORMATION, desiredaccess : *mut u32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        SetSecurityAccessMask(securityinformation, &mut result__);
        result__
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetSecurityDescriptorControl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, controlbitsofinterest: super::SECURITY_DESCRIPTOR_CONTROL, controlbitstoset: super::SECURITY_DESCRIPTOR_CONTROL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorControl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, controlbitsofinterest : super::SECURITY_DESCRIPTOR_CONTROL, controlbitstoset : super::SECURITY_DESCRIPTOR_CONTROL) -> windows_core::BOOL);
    unsafe { SetSecurityDescriptorControl(psecuritydescriptor, controlbitsofinterest, controlbitstoset) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetSecurityDescriptorDacl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, bdaclpresent: bool, pdacl: Option<*const super::ACL>, bdacldefaulted: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorDacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, bdaclpresent : windows_core::BOOL, pdacl : *const super::ACL, bdacldefaulted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSecurityDescriptorDacl(psecuritydescriptor as _, bdaclpresent.into(), pdacl.unwrap_or(core::mem::zeroed()) as _, bdacldefaulted.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetSecurityDescriptorGroup(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pgroup: Option<super::PSID>, bgroupdefaulted: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorGroup(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pgroup : super::PSID, bgroupdefaulted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSecurityDescriptorGroup(psecuritydescriptor as _, pgroup.unwrap_or(core::mem::zeroed()) as _, bgroupdefaulted.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetSecurityDescriptorOwner(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, powner: Option<super::PSID>, bownerdefaulted: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorOwner(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, powner : super::PSID, bownerdefaulted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSecurityDescriptorOwner(psecuritydescriptor as _, powner.unwrap_or(core::mem::zeroed()) as _, bownerdefaulted.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetSecurityDescriptorRMControl(securitydescriptor: super::PSECURITY_DESCRIPTOR, rmcontrol: Option<*const u8>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorRMControl(securitydescriptor : super::PSECURITY_DESCRIPTOR, rmcontrol : *const u8) -> u32);
    unsafe { SetSecurityDescriptorRMControl(securitydescriptor as _, rmcontrol.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetSecurityDescriptorSacl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, bsaclpresent: bool, psacl: Option<*const super::ACL>, bsacldefaulted: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorSacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, bsaclpresent : windows_core::BOOL, psacl : *const super::ACL, bsacldefaulted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSecurityDescriptorSacl(psecuritydescriptor as _, bsaclpresent.into(), psacl.unwrap_or(core::mem::zeroed()) as _, bsacldefaulted.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetTokenInformation(tokenhandle: super::HANDLE, tokeninformationclass: super::TOKEN_INFORMATION_CLASS, tokeninformation: *const core::ffi::c_void, tokeninformationlength: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetTokenInformation(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : *const core::ffi::c_void, tokeninformationlength : u32) -> windows_core::BOOL);
    unsafe { SetTokenInformation(tokenhandle, tokeninformationclass, tokeninformation, tokeninformationlength) }
}
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_NOT_VALIDATED: u32 = 1;
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_VALIDATE_ONLY: u32 = 4;
pub const SIGNING_LEVEL_MICROSOFT: u32 = 8;
