#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheck(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, clienttoken: super::HANDLE, desiredaccess: super::DWORD, genericmapping: super::PGENERIC_MAPPING, privilegeset: Option<super::PPRIVILEGE_SET>, privilegesetlength: super::LPDWORD, grantedaccess: super::LPDWORD, accessstatus: super::LPBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AccessCheck(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : super::DWORD, genericmapping : super::PGENERIC_MAPPING, privilegeset : super::PPRIVILEGE_SET, privilegesetlength : super::LPDWORD, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL) -> windows_core::BOOL);
    unsafe { AccessCheck(psecuritydescriptor, clienttoken, desiredaccess, genericmapping, privilegeset.unwrap_or(core::mem::zeroed()) as _, privilegesetlength as _, grantedaccess as _, accessstatus as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmW<P0, P2, P3>(subsystemname: P0, handleid: Option<super::LPVOID>, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, desiredaccess: super::DWORD, genericmapping: super::PGENERIC_MAPPING, objectcreation: bool, grantedaccess: super::LPDWORD, accessstatus: super::LPBOOL, pfgenerateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : super::LPVOID, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, desiredaccess : super::DWORD, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL, pfgenerateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe { AccessCheckAndAuditAlarmW(subsystemname.param().abi(), handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, desiredaccess, genericmapping, objectcreation.into(), grantedaccess as _, accessstatus as _, pfgenerateonclose as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckByType(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, clienttoken: super::HANDLE, desiredaccess: super::DWORD, objecttypelist: Option<&mut [super::OBJECT_TYPE_LIST]>, genericmapping: super::PGENERIC_MAPPING, privilegeset: Option<super::PPRIVILEGE_SET>, privilegesetlength: super::LPDWORD, grantedaccess: super::LPDWORD, accessstatus: super::LPBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByType(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, clienttoken : super::HANDLE, desiredaccess : super::DWORD, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, genericmapping : super::PGENERIC_MAPPING, privilegeset : super::PPRIVILEGE_SET, privilegesetlength : super::LPDWORD, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByType(psecuritydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, clienttoken, desiredaccess, core::mem::transmute(objecttypelist.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), genericmapping, privilegeset.unwrap_or(core::mem::zeroed()) as _, privilegesetlength as _, grantedaccess as _, accessstatus as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmW<P0, P2, P3>(subsystemname: P0, handleid: super::LPVOID, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::DWORD, audittype: super::AUDIT_EVENT_TYPE, flags: super::DWORD, objecttypelist: Option<&mut [super::OBJECT_TYPE_LIST]>, genericmapping: super::PGENERIC_MAPPING, objectcreation: bool, grantedaccess: super::LPDWORD, accessstatus: super::LPBOOL, pfgenerateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : super::LPVOID, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::DWORD, audittype : super::AUDIT_EVENT_TYPE, flags : super::DWORD, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL, pfgenerateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe {
        AccessCheckByTypeAndAuditAlarmW(
            subsystemname.param().abi(),
            handleid,
            objecttypename.param().abi(),
            objectname.param().abi(),
            securitydescriptor,
            principalselfsid.unwrap_or(core::mem::zeroed()) as _,
            desiredaccess,
            audittype,
            flags,
            core::mem::transmute(objecttypelist.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())),
            objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            genericmapping,
            objectcreation.into(),
            grantedaccess as _,
            accessstatus as _,
            pfgenerateonclose as _,
        )
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckByTypeResultList(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, clienttoken: super::HANDLE, desiredaccess: super::DWORD, objecttypelist: Option<super::POBJECT_TYPE_LIST>, objecttypelistlength: super::DWORD, genericmapping: super::PGENERIC_MAPPING, privilegeset: Option<super::PPRIVILEGE_SET>, privilegesetlength: super::LPDWORD, grantedaccesslist: super::LPDWORD, accessstatuslist: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeResultList(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, clienttoken : super::HANDLE, desiredaccess : super::DWORD, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, genericmapping : super::PGENERIC_MAPPING, privilegeset : super::PPRIVILEGE_SET, privilegesetlength : super::LPDWORD, grantedaccesslist : super::LPDWORD, accessstatuslist : super::LPDWORD) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeResultList(psecuritydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, clienttoken, desiredaccess, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, privilegeset.unwrap_or(core::mem::zeroed()) as _, privilegesetlength as _, grantedaccesslist as _, accessstatuslist as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleW<P0, P3, P4>(subsystemname: P0, handleid: super::LPVOID, clienttoken: super::HANDLE, objecttypename: P3, objectname: P4, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::DWORD, audittype: super::AUDIT_EVENT_TYPE, flags: super::DWORD, objecttypelist: Option<super::POBJECT_TYPE_LIST>, objecttypelistlength: super::DWORD, genericmapping: super::PGENERIC_MAPPING, objectcreation: bool, grantedaccesslist: super::LPDWORD, accessstatuslist: super::LPDWORD, pfgenerateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname : windows_core::PCWSTR, handleid : super::LPVOID, clienttoken : super::HANDLE, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::DWORD, audittype : super::AUDIT_EVENT_TYPE, flags : super::DWORD, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccesslist : super::LPDWORD, accessstatuslist : super::LPDWORD, pfgenerateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeResultListAndAuditAlarmByHandleW(subsystemname.param().abi(), handleid, clienttoken, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation.into(), grantedaccesslist as _, accessstatuslist as _, pfgenerateonclose as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmW<P0, P2, P3>(subsystemname: P0, handleid: super::LPVOID, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::DWORD, audittype: super::AUDIT_EVENT_TYPE, flags: super::DWORD, objecttypelist: Option<super::POBJECT_TYPE_LIST>, objecttypelistlength: super::DWORD, genericmapping: super::PGENERIC_MAPPING, objectcreation: bool, grantedaccesslist: super::LPDWORD, accessstatuslist: super::LPDWORD, pfgenerateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : super::LPVOID, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::DWORD, audittype : super::AUDIT_EVENT_TYPE, flags : super::DWORD, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccesslist : super::LPDWORD, accessstatuslist : super::LPDWORD, pfgenerateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeResultListAndAuditAlarmW(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation.into(), grantedaccesslist as _, accessstatuslist as _, pfgenerateonclose as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAccessAllowedAce(pacl: super::PACL, dwacerevision: super::DWORD, accessmask: super::DWORD, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessAllowedAce(pacl : super::PACL, dwacerevision : super::DWORD, accessmask : super::DWORD, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessAllowedAce(pacl as _, dwacerevision, accessmask, psid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAccessAllowedAceEx(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, accessmask: super::DWORD, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessAllowedAceEx(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, accessmask : super::DWORD, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessAllowedAceEx(pacl as _, dwacerevision, aceflags, accessmask, psid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAccessAllowedObjectAce(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, accessmask: super::DWORD, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessAllowedObjectAce(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, accessmask : super::DWORD, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessAllowedObjectAce(pacl as _, dwacerevision, aceflags, accessmask, objecttypeguid.unwrap_or(core::mem::zeroed()) as _, inheritedobjecttypeguid.unwrap_or(core::mem::zeroed()) as _, psid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAccessDeniedAce(pacl: super::PACL, dwacerevision: super::DWORD, accessmask: super::DWORD, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessDeniedAce(pacl : super::PACL, dwacerevision : super::DWORD, accessmask : super::DWORD, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessDeniedAce(pacl as _, dwacerevision, accessmask, psid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAccessDeniedAceEx(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, accessmask: super::DWORD, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessDeniedAceEx(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, accessmask : super::DWORD, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessDeniedAceEx(pacl as _, dwacerevision, aceflags, accessmask, psid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAccessDeniedObjectAce(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, accessmask: super::DWORD, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAccessDeniedObjectAce(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, accessmask : super::DWORD, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddAccessDeniedObjectAce(pacl as _, dwacerevision, aceflags, accessmask, objecttypeguid.unwrap_or(core::mem::zeroed()) as _, inheritedobjecttypeguid.unwrap_or(core::mem::zeroed()) as _, psid) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAce(pacl: super::PACL, dwacerevision: super::DWORD, dwstartingaceindex: super::DWORD, pacelist: super::LPVOID, nacelistlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAce(pacl : super::PACL, dwacerevision : super::DWORD, dwstartingaceindex : super::DWORD, pacelist : super::LPVOID, nacelistlength : super::DWORD) -> windows_core::BOOL);
    unsafe { AddAce(pacl as _, dwacerevision, dwstartingaceindex, pacelist, nacelistlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAuditAccessAce(pacl: super::PACL, dwacerevision: super::DWORD, dwaccessmask: super::DWORD, psid: super::PSID, bauditsuccess: bool, bauditfailure: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAuditAccessAce(pacl : super::PACL, dwacerevision : super::DWORD, dwaccessmask : super::DWORD, psid : super::PSID, bauditsuccess : windows_core::BOOL, bauditfailure : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AddAuditAccessAce(pacl as _, dwacerevision, dwaccessmask, psid, bauditsuccess.into(), bauditfailure.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAuditAccessAceEx(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, dwaccessmask: super::DWORD, psid: super::PSID, bauditsuccess: bool, bauditfailure: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAuditAccessAceEx(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, dwaccessmask : super::DWORD, psid : super::PSID, bauditsuccess : windows_core::BOOL, bauditfailure : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AddAuditAccessAceEx(pacl as _, dwacerevision, aceflags, dwaccessmask, psid, bauditsuccess.into(), bauditfailure.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddAuditAccessObjectAce(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, accessmask: super::DWORD, objecttypeguid: Option<*const windows_core::GUID>, inheritedobjecttypeguid: Option<*const windows_core::GUID>, psid: super::PSID, bauditsuccess: bool, bauditfailure: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddAuditAccessObjectAce(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, accessmask : super::DWORD, objecttypeguid : *const windows_core::GUID, inheritedobjecttypeguid : *const windows_core::GUID, psid : super::PSID, bauditsuccess : windows_core::BOOL, bauditfailure : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AddAuditAccessObjectAce(pacl as _, dwacerevision, aceflags, accessmask, objecttypeguid.unwrap_or(core::mem::zeroed()) as _, inheritedobjecttypeguid.unwrap_or(core::mem::zeroed()) as _, psid, bauditsuccess.into(), bauditfailure.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddMandatoryAce(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, mandatorypolicy: super::DWORD, plabelsid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddMandatoryAce(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, mandatorypolicy : super::DWORD, plabelsid : super::PSID) -> windows_core::BOOL);
    unsafe { AddMandatoryAce(pacl as _, dwacerevision, aceflags, mandatorypolicy, plabelsid) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddResourceAttributeAce(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, accessmask: super::DWORD, psid: super::PSID, pattributeinfo: super::PCLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AddResourceAttributeAce(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, accessmask : super::DWORD, psid : super::PSID, pattributeinfo : super::PCLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { AddResourceAttributeAce(pacl as _, dwacerevision, aceflags, accessmask, psid, pattributeinfo, preturnlength as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddScopedPolicyIDAce(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, accessmask: super::DWORD, psid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AddScopedPolicyIDAce(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, accessmask : super::DWORD, psid : super::PSID) -> windows_core::BOOL);
    unsafe { AddScopedPolicyIDAce(pacl as _, dwacerevision, aceflags, accessmask, psid) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AdjustTokenGroups(tokenhandle: super::HANDLE, resettodefault: bool, newstate: Option<super::PTOKEN_GROUPS>, bufferlength: super::DWORD, previousstate: Option<super::PTOKEN_GROUPS>, returnlength: Option<super::PDWORD>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AdjustTokenGroups(tokenhandle : super::HANDLE, resettodefault : windows_core::BOOL, newstate : super::PTOKEN_GROUPS, bufferlength : super::DWORD, previousstate : super::PTOKEN_GROUPS, returnlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { AdjustTokenGroups(tokenhandle, resettodefault.into(), newstate.unwrap_or(core::mem::zeroed()) as _, bufferlength, previousstate.unwrap_or(core::mem::zeroed()) as _, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AdjustTokenPrivileges(tokenhandle: super::HANDLE, disableallprivileges: bool, newstate: Option<super::PTOKEN_PRIVILEGES>, bufferlength: super::DWORD, previousstate: Option<super::PTOKEN_PRIVILEGES>, returnlength: Option<super::PDWORD>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AdjustTokenPrivileges(tokenhandle : super::HANDLE, disableallprivileges : windows_core::BOOL, newstate : super::PTOKEN_PRIVILEGES, bufferlength : super::DWORD, previousstate : super::PTOKEN_PRIVILEGES, returnlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { AdjustTokenPrivileges(tokenhandle, disableallprivileges.into(), newstate.unwrap_or(core::mem::zeroed()) as _, bufferlength, previousstate.unwrap_or(core::mem::zeroed()) as _, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AllocateAndInitializeSid(pidentifierauthority: super::PSID_IDENTIFIER_AUTHORITY, nsubauthoritycount: super::BYTE, nsubauthority0: super::DWORD, nsubauthority1: super::DWORD, nsubauthority2: super::DWORD, nsubauthority3: super::DWORD, nsubauthority4: super::DWORD, nsubauthority5: super::DWORD, nsubauthority6: super::DWORD, nsubauthority7: super::DWORD, psid: *mut super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AllocateAndInitializeSid(pidentifierauthority : super::PSID_IDENTIFIER_AUTHORITY, nsubauthoritycount : super::BYTE, nsubauthority0 : super::DWORD, nsubauthority1 : super::DWORD, nsubauthority2 : super::DWORD, nsubauthority3 : super::DWORD, nsubauthority4 : super::DWORD, nsubauthority5 : super::DWORD, nsubauthority6 : super::DWORD, nsubauthority7 : super::DWORD, psid : *mut super::PSID) -> windows_core::BOOL);
    unsafe { AllocateAndInitializeSid(pidentifierauthority, nsubauthoritycount, nsubauthority0, nsubauthority1, nsubauthority2, nsubauthority3, nsubauthority4, nsubauthority5, nsubauthority6, nsubauthority7, psid as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AllocateLocallyUniqueId(luid: super::PLUID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AllocateLocallyUniqueId(luid : super::PLUID) -> windows_core::BOOL);
    unsafe { AllocateLocallyUniqueId(luid as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn AreAllAccessesGranted(grantedaccess: super::DWORD, desiredaccess: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AreAllAccessesGranted(grantedaccess : super::DWORD, desiredaccess : super::DWORD) -> windows_core::BOOL);
    unsafe { AreAllAccessesGranted(grantedaccess, desiredaccess) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn AreAnyAccessesGranted(grantedaccess: super::DWORD, desiredaccess: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AreAnyAccessesGranted(grantedaccess : super::DWORD, desiredaccess : super::DWORD) -> windows_core::BOOL);
    unsafe { AreAnyAccessesGranted(grantedaccess, desiredaccess) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn CheckTokenCapability(tokenhandle: Option<super::HANDLE>, capabilitysidtocheck: super::PSID, hascapability: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CheckTokenCapability(tokenhandle : super::HANDLE, capabilitysidtocheck : super::PSID, hascapability : super::PBOOL) -> windows_core::BOOL);
    unsafe { CheckTokenCapability(tokenhandle.unwrap_or(core::mem::zeroed()) as _, capabilitysidtocheck, hascapability as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn CheckTokenMembership(tokenhandle: Option<super::HANDLE>, sidtocheck: super::PSID, ismember: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CheckTokenMembership(tokenhandle : super::HANDLE, sidtocheck : super::PSID, ismember : super::PBOOL) -> windows_core::BOOL);
    unsafe { CheckTokenMembership(tokenhandle.unwrap_or(core::mem::zeroed()) as _, sidtocheck, ismember as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CheckTokenMembershipEx(tokenhandle: Option<super::HANDLE>, sidtocheck: super::PSID, flags: super::DWORD, ismember: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CheckTokenMembershipEx(tokenhandle : super::HANDLE, sidtocheck : super::PSID, flags : super::DWORD, ismember : super::PBOOL) -> windows_core::BOOL);
    unsafe { CheckTokenMembershipEx(tokenhandle.unwrap_or(core::mem::zeroed()) as _, sidtocheck, flags, ismember as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor: Option<super::PSECURITY_DESCRIPTOR>, currentsecuritydescriptor: super::PSECURITY_DESCRIPTOR, newsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, objecttype: Option<*const windows_core::GUID>, isdirectoryobject: super::BOOLEAN, genericmapping: super::PGENERIC_MAPPING) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor : super::PSECURITY_DESCRIPTOR, currentsecuritydescriptor : super::PSECURITY_DESCRIPTOR, newsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttype : *const windows_core::GUID, isdirectoryobject : super::BOOLEAN, genericmapping : super::PGENERIC_MAPPING) -> windows_core::BOOL);
    unsafe { ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor.unwrap_or(core::mem::zeroed()) as _, currentsecuritydescriptor, newsecuritydescriptor as _, objecttype.unwrap_or(core::mem::zeroed()) as _, isdirectoryobject, genericmapping) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CopySid(ndestinationsidlength: super::DWORD, pdestinationsid: super::PSID, psourcesid: super::PSID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CopySid(ndestinationsidlength : super::DWORD, pdestinationsid : super::PSID, psourcesid : super::PSID) -> windows_core::BOOL);
    unsafe { CopySid(ndestinationsidlength, pdestinationsid as _, psourcesid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePrivateObjectSecurity(parentdescriptor: Option<super::PSECURITY_DESCRIPTOR>, creatordescriptor: Option<super::PSECURITY_DESCRIPTOR>, newdescriptor: *mut super::PSECURITY_DESCRIPTOR, isdirectoryobject: bool, token: Option<super::HANDLE>, genericmapping: super::PGENERIC_MAPPING) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurity(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, isdirectoryobject : windows_core::BOOL, token : super::HANDLE, genericmapping : super::PGENERIC_MAPPING) -> windows_core::BOOL);
    unsafe { CreatePrivateObjectSecurity(parentdescriptor.unwrap_or(core::mem::zeroed()) as _, creatordescriptor.unwrap_or(core::mem::zeroed()) as _, newdescriptor as _, isdirectoryobject.into(), token.unwrap_or(core::mem::zeroed()) as _, genericmapping) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePrivateObjectSecurityEx(parentdescriptor: Option<super::PSECURITY_DESCRIPTOR>, creatordescriptor: Option<super::PSECURITY_DESCRIPTOR>, newdescriptor: *mut super::PSECURITY_DESCRIPTOR, objecttype: Option<*const windows_core::GUID>, iscontainerobject: bool, autoinheritflags: super::ULONG, token: Option<super::HANDLE>, genericmapping: super::PGENERIC_MAPPING) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityEx(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttype : *const windows_core::GUID, iscontainerobject : windows_core::BOOL, autoinheritflags : super::ULONG, token : super::HANDLE, genericmapping : super::PGENERIC_MAPPING) -> windows_core::BOOL);
    unsafe { CreatePrivateObjectSecurityEx(parentdescriptor.unwrap_or(core::mem::zeroed()) as _, creatordescriptor.unwrap_or(core::mem::zeroed()) as _, newdescriptor as _, objecttype.unwrap_or(core::mem::zeroed()) as _, iscontainerobject.into(), autoinheritflags, token.unwrap_or(core::mem::zeroed()) as _, genericmapping) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor: Option<super::PSECURITY_DESCRIPTOR>, creatordescriptor: Option<super::PSECURITY_DESCRIPTOR>, newdescriptor: *mut super::PSECURITY_DESCRIPTOR, objecttypes: Option<&[*const windows_core::GUID]>, iscontainerobject: bool, autoinheritflags: super::ULONG, token: Option<super::HANDLE>, genericmapping: super::PGENERIC_MAPPING) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor : super::PSECURITY_DESCRIPTOR, creatordescriptor : super::PSECURITY_DESCRIPTOR, newdescriptor : *mut super::PSECURITY_DESCRIPTOR, objecttypes : *const *const windows_core::GUID, guidcount : super::ULONG, iscontainerobject : windows_core::BOOL, autoinheritflags : super::ULONG, token : super::HANDLE, genericmapping : super::PGENERIC_MAPPING) -> windows_core::BOOL);
    unsafe { CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor.unwrap_or(core::mem::zeroed()) as _, creatordescriptor.unwrap_or(core::mem::zeroed()) as _, newdescriptor as _, objecttypes.map_or(core::ptr::null(), |slice| slice.as_ptr()), objecttypes.map_or(0, |slice| slice.len().try_into().unwrap()), iscontainerobject.into(), autoinheritflags, token.unwrap_or(core::mem::zeroed()) as _, genericmapping) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateRestrictedToken(existingtokenhandle: super::HANDLE, flags: super::DWORD, sidstodisable: Option<&[super::SID_AND_ATTRIBUTES]>, privilegestodelete: Option<&[super::LUID_AND_ATTRIBUTES]>, sidstorestrict: Option<&[super::SID_AND_ATTRIBUTES]>, newtokenhandle: super::PHANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreateRestrictedToken(existingtokenhandle : super::HANDLE, flags : super::DWORD, disablesidcount : super::DWORD, sidstodisable : super::PSID_AND_ATTRIBUTES, deleteprivilegecount : super::DWORD, privilegestodelete : super::PLUID_AND_ATTRIBUTES, restrictedsidcount : super::DWORD, sidstorestrict : super::PSID_AND_ATTRIBUTES, newtokenhandle : super::PHANDLE) -> windows_core::BOOL);
    unsafe {
        CreateRestrictedToken(
            existingtokenhandle,
            flags,
            sidstodisable.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(sidstodisable.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            privilegestodelete.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(privilegestodelete.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            sidstorestrict.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(sidstorestrict.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            newtokenhandle as _,
        )
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWellKnownSid(wellknownsidtype: super::WELL_KNOWN_SID_TYPE, domainsid: Option<super::PSID>, psid: Option<super::PSID>, cbsid: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CreateWellKnownSid(wellknownsidtype : super::WELL_KNOWN_SID_TYPE, domainsid : super::PSID, psid : super::PSID, cbsid : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { CreateWellKnownSid(wellknownsidtype, domainsid.unwrap_or(core::mem::zeroed()) as _, psid.unwrap_or(core::mem::zeroed()) as _, cbsid as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CveEventWrite<P0, P1>(cveid: P0, additionaldetails: P1) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CveEventWrite(cveid : windows_core::PCWSTR, additionaldetails : windows_core::PCWSTR) -> super::LONG);
    unsafe { CveEventWrite(cveid.param().abi(), additionaldetails.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DeleteAce(pacl: super::PACL, dwaceindex: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DeleteAce(pacl : super::PACL, dwaceindex : super::DWORD) -> windows_core::BOOL);
    unsafe { DeleteAce(pacl as _, dwaceindex) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DeriveCapabilitySidsFromName<P0>(capname: P0, capabilitygroupsids: *mut *mut super::PSID, capabilitygroupsidcount: *mut super::DWORD, capabilitysids: *mut *mut super::PSID, capabilitysidcount: *mut super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-security-base-l1-2-2.dll" "system" fn DeriveCapabilitySidsFromName(capname : windows_core::PCWSTR, capabilitygroupsids : *mut *mut super::PSID, capabilitygroupsidcount : *mut super::DWORD, capabilitysids : *mut *mut super::PSID, capabilitysidcount : *mut super::DWORD) -> windows_core::BOOL);
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
pub unsafe fn DuplicateToken(existingtokenhandle: super::HANDLE, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle: super::PHANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DuplicateToken(existingtokenhandle : super::HANDLE, impersonationlevel : super::SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle : super::PHANDLE) -> windows_core::BOOL);
    unsafe { DuplicateToken(existingtokenhandle, impersonationlevel, duplicatetokenhandle as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DuplicateTokenEx(hexistingtoken: super::HANDLE, dwdesiredaccess: super::DWORD, lptokenattributes: Option<super::LPSECURITY_ATTRIBUTES>, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokentype: super::TOKEN_TYPE, phnewtoken: super::PHANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DuplicateTokenEx(hexistingtoken : super::HANDLE, dwdesiredaccess : super::DWORD, lptokenattributes : super::LPSECURITY_ATTRIBUTES, impersonationlevel : super::SECURITY_IMPERSONATION_LEVEL, tokentype : super::TOKEN_TYPE, phnewtoken : super::PHANDLE) -> windows_core::BOOL);
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFreeAce(pacl: super::PACL, pace: *mut super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn FindFirstFreeAce(pacl : super::PACL, pace : *mut super::LPVOID) -> windows_core::BOOL);
    unsafe { FindFirstFreeAce(pacl, pace as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FreeSid(psid: super::PSID) -> super::PVOID {
    windows_core::link!("advapi32.dll" "system" fn FreeSid(psid : super::PSID) -> super::PVOID);
    unsafe { FreeSid(psid) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetAce(pacl: super::PACL, dwaceindex: super::DWORD, pace: *mut super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetAce(pacl : super::PACL, dwaceindex : super::DWORD, pace : *mut super::LPVOID) -> windows_core::BOOL);
    unsafe { GetAce(pacl, dwaceindex, pace as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetAclInformation(pacl: super::PACL, paclinformation: super::LPVOID, naclinformationlength: super::DWORD, dwaclinformationclass: super::ACL_INFORMATION_CLASS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetAclInformation(pacl : super::PACL, paclinformation : super::LPVOID, naclinformationlength : super::DWORD, dwaclinformationclass : super::ACL_INFORMATION_CLASS) -> windows_core::BOOL);
    unsafe { GetAclInformation(pacl, paclinformation as _, naclinformationlength, dwaclinformationclass) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetAppContainerAce(acl: super::PACL, startingaceindex: super::DWORD, appcontainerace: *mut super::PVOID, appcontaineraceindex: Option<*mut super::DWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetAppContainerAce(acl : super::PACL, startingaceindex : super::DWORD, appcontainerace : *mut super::PVOID, appcontaineraceindex : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { GetAppContainerAce(acl, startingaceindex, appcontainerace as _, appcontaineraceindex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetCachedSigningLevel(file: super::HANDLE, flags: super::PULONG, signinglevel: super::PULONG, thumbprint: Option<super::PUCHAR>, thumbprintsize: Option<super::PULONG>, thumbprintalgorithm: Option<super::PULONG>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCachedSigningLevel(file : super::HANDLE, flags : super::PULONG, signinglevel : super::PULONG, thumbprint : super::PUCHAR, thumbprintsize : super::PULONG, thumbprintalgorithm : super::PULONG) -> windows_core::BOOL);
    unsafe { GetCachedSigningLevel(file, flags as _, signinglevel as _, thumbprint.unwrap_or(core::mem::zeroed()) as _, thumbprintsize.unwrap_or(core::mem::zeroed()) as _, thumbprintalgorithm.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileSecurityW<P0>(lpfilename: P0, requestedinformation: super::SECURITY_INFORMATION, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, nlength: super::DWORD, lpnlengthneeded: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetFileSecurityW(lpfilename : windows_core::PCWSTR, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : super::DWORD, lpnlengthneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetFileSecurityW(lpfilename.param().abi(), requestedinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetKernelObjectSecurity(handle: super::HANDLE, requestedinformation: super::SECURITY_INFORMATION, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, nlength: super::DWORD, lpnlengthneeded: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetKernelObjectSecurity(handle : super::HANDLE, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : super::DWORD, lpnlengthneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetKernelObjectSecurity(handle, requestedinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetLengthSid(psid: super::PSID) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn GetLengthSid(psid : super::PSID) -> super::DWORD);
    unsafe { GetLengthSid(psid) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPrivateObjectSecurity(objectdescriptor: super::PSECURITY_DESCRIPTOR, securityinformation: super::SECURITY_INFORMATION, resultantdescriptor: Option<super::PSECURITY_DESCRIPTOR>, descriptorlength: super::DWORD, returnlength: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetPrivateObjectSecurity(objectdescriptor : super::PSECURITY_DESCRIPTOR, securityinformation : super::SECURITY_INFORMATION, resultantdescriptor : super::PSECURITY_DESCRIPTOR, descriptorlength : super::DWORD, returnlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetPrivateObjectSecurity(objectdescriptor, securityinformation, resultantdescriptor.unwrap_or(core::mem::zeroed()) as _, descriptorlength, returnlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityDescriptorControl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pcontrol: super::PSECURITY_DESCRIPTOR_CONTROL, lpdwrevision: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorControl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pcontrol : super::PSECURITY_DESCRIPTOR_CONTROL, lpdwrevision : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorControl(psecuritydescriptor, pcontrol as _, lpdwrevision as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityDescriptorDacl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, lpbdaclpresent: super::LPBOOL, pdacl: *mut super::PACL, lpbdacldefaulted: super::LPBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorDacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpbdaclpresent : super::LPBOOL, pdacl : *mut super::PACL, lpbdacldefaulted : super::LPBOOL) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorDacl(psecuritydescriptor, lpbdaclpresent as _, pdacl as _, lpbdacldefaulted as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityDescriptorGroup(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pgroup: *mut super::PSID, lpbgroupdefaulted: super::LPBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorGroup(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pgroup : *mut super::PSID, lpbgroupdefaulted : super::LPBOOL) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorGroup(psecuritydescriptor, pgroup as _, lpbgroupdefaulted as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityDescriptorLength(psecuritydescriptor: super::PSECURITY_DESCRIPTOR) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorLength(psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> super::DWORD);
    unsafe { GetSecurityDescriptorLength(psecuritydescriptor) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityDescriptorOwner(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, powner: *mut super::PSID, lpbownerdefaulted: super::LPBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorOwner(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, powner : *mut super::PSID, lpbownerdefaulted : super::LPBOOL) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorOwner(psecuritydescriptor, powner as _, lpbownerdefaulted as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityDescriptorRMControl(securitydescriptor: super::PSECURITY_DESCRIPTOR, rmcontrol: super::PUCHAR) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorRMControl(securitydescriptor : super::PSECURITY_DESCRIPTOR, rmcontrol : super::PUCHAR) -> super::DWORD);
    unsafe { GetSecurityDescriptorRMControl(securitydescriptor, rmcontrol as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityDescriptorSacl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, lpbsaclpresent: super::LPBOOL, psacl: *mut super::PACL, lpbsacldefaulted: super::LPBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityDescriptorSacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpbsaclpresent : super::LPBOOL, psacl : *mut super::PACL, lpbsacldefaulted : super::LPBOOL) -> windows_core::BOOL);
    unsafe { GetSecurityDescriptorSacl(psecuritydescriptor, lpbsaclpresent as _, psacl as _, lpbsacldefaulted as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetSidIdentifierAuthority(psid: super::PSID) -> super::PSID_IDENTIFIER_AUTHORITY {
    windows_core::link!("advapi32.dll" "system" fn GetSidIdentifierAuthority(psid : super::PSID) -> super::PSID_IDENTIFIER_AUTHORITY);
    unsafe { GetSidIdentifierAuthority(psid) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetSidLengthRequired(nsubauthoritycount: super::UCHAR) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn GetSidLengthRequired(nsubauthoritycount : super::UCHAR) -> super::DWORD);
    unsafe { GetSidLengthRequired(nsubauthoritycount) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetSidSubAuthority(psid: super::PSID, nsubauthority: super::DWORD) -> super::PDWORD {
    windows_core::link!("advapi32.dll" "system" fn GetSidSubAuthority(psid : super::PSID, nsubauthority : super::DWORD) -> super::PDWORD);
    unsafe { GetSidSubAuthority(psid, nsubauthority) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetSidSubAuthorityCount(psid: super::PSID) -> super::PUCHAR {
    windows_core::link!("advapi32.dll" "system" fn GetSidSubAuthorityCount(psid : super::PSID) -> super::PUCHAR);
    unsafe { GetSidSubAuthorityCount(psid) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetTokenInformation(tokenhandle: super::HANDLE, tokeninformationclass: super::TOKEN_INFORMATION_CLASS, tokeninformation: Option<super::LPVOID>, tokeninformationlength: super::DWORD, returnlength: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetTokenInformation(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : super::LPVOID, tokeninformationlength : super::DWORD, returnlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetTokenInformation(tokenhandle, tokeninformationclass, tokeninformation.unwrap_or(core::mem::zeroed()) as _, tokeninformationlength, returnlength as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetWindowsAccountDomainSid(psid: super::PSID, pdomainsid: Option<super::PSID>, cbdomainsid: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetWindowsAccountDomainSid(psid : super::PSID, pdomainsid : super::PSID, cbdomainsid : *mut super::DWORD) -> windows_core::BOOL);
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeAcl(pacl: super::PACL, nacllength: super::DWORD, dwaclrevision: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn InitializeAcl(pacl : super::PACL, nacllength : super::DWORD, dwaclrevision : super::DWORD) -> windows_core::BOOL);
    unsafe { InitializeAcl(pacl as _, nacllength, dwaclrevision) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeSecurityDescriptor(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, dwrevision: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn InitializeSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, dwrevision : super::DWORD) -> windows_core::BOOL);
    unsafe { InitializeSecurityDescriptor(psecuritydescriptor as _, dwrevision) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeSid(sid: super::PSID, pidentifierauthority: super::PSID_IDENTIFIER_AUTHORITY, nsubauthoritycount: super::BYTE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn InitializeSid(sid : super::PSID, pidentifierauthority : super::PSID_IDENTIFIER_AUTHORITY, nsubauthoritycount : super::BYTE) -> windows_core::BOOL);
    unsafe { InitializeSid(sid as _, pidentifierauthority, nsubauthoritycount) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsTokenRestricted(tokenhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsTokenRestricted(tokenhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { IsTokenRestricted(tokenhandle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IsValidAcl(pacl: super::PACL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsValidAcl(pacl : super::PACL) -> windows_core::BOOL);
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MakeAbsoluteSD(pselfrelativesecuritydescriptor: super::PSECURITY_DESCRIPTOR, pabsolutesecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, lpdwabsolutesecuritydescriptorsize: super::LPDWORD, pdacl: Option<super::PACL>, lpdwdaclsize: super::LPDWORD, psacl: Option<super::PACL>, lpdwsaclsize: super::LPDWORD, powner: Option<super::PSID>, lpdwownersize: super::LPDWORD, pprimarygroup: Option<super::PSID>, lpdwprimarygroupsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn MakeAbsoluteSD(pselfrelativesecuritydescriptor : super::PSECURITY_DESCRIPTOR, pabsolutesecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize : super::LPDWORD, pdacl : super::PACL, lpdwdaclsize : super::LPDWORD, psacl : super::PACL, lpdwsaclsize : super::LPDWORD, powner : super::PSID, lpdwownersize : super::LPDWORD, pprimarygroup : super::PSID, lpdwprimarygroupsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { MakeAbsoluteSD(pselfrelativesecuritydescriptor, pabsolutesecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, lpdwabsolutesecuritydescriptorsize as _, pdacl.unwrap_or(core::mem::zeroed()) as _, lpdwdaclsize as _, psacl.unwrap_or(core::mem::zeroed()) as _, lpdwsaclsize as _, powner.unwrap_or(core::mem::zeroed()) as _, lpdwownersize as _, pprimarygroup.unwrap_or(core::mem::zeroed()) as _, lpdwprimarygroupsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MakeSelfRelativeSD(pabsolutesecuritydescriptor: super::PSECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, lpdwbufferlength: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn MakeSelfRelativeSD(pabsolutesecuritydescriptor : super::PSECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor : super::PSECURITY_DESCRIPTOR, lpdwbufferlength : super::LPDWORD) -> windows_core::BOOL);
    unsafe { MakeSelfRelativeSD(pabsolutesecuritydescriptor, pselfrelativesecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapGenericMask(accessmask: super::PDWORD, genericmapping: super::PGENERIC_MAPPING) {
    windows_core::link!("advapi32.dll" "system" fn MapGenericMask(accessmask : super::PDWORD, genericmapping : super::PGENERIC_MAPPING));
    unsafe { MapGenericMask(accessmask as _, genericmapping) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ObjectCloseAuditAlarmW<P0>(subsystemname: P0, handleid: super::LPVOID, generateonclose: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectCloseAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : super::LPVOID, generateonclose : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectCloseAuditAlarmW(subsystemname.param().abi(), handleid, generateonclose.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmW<P0>(subsystemname: P0, handleid: super::LPVOID, generateonclose: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectDeleteAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : super::LPVOID, generateonclose : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectDeleteAuditAlarmW(subsystemname.param().abi(), handleid, generateonclose.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ObjectOpenAuditAlarmW<P0, P2, P3>(subsystemname: P0, handleid: super::LPVOID, objecttypename: P2, objectname: P3, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, clienttoken: super::HANDLE, desiredaccess: super::DWORD, grantedaccess: super::DWORD, privileges: Option<super::PPRIVILEGE_SET>, objectcreation: bool, accessgranted: bool, generateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectOpenAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : super::LPVOID, objecttypename : windows_core::PCWSTR, objectname : windows_core::PCWSTR, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : super::DWORD, grantedaccess : super::DWORD, privileges : super::PPRIVILEGE_SET, objectcreation : windows_core::BOOL, accessgranted : windows_core::BOOL, generateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe { ObjectOpenAuditAlarmW(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), psecuritydescriptor, clienttoken, desiredaccess, grantedaccess, privileges.unwrap_or(core::mem::zeroed()) as _, objectcreation.into(), accessgranted.into(), generateonclose as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmW<P0>(subsystemname: P0, handleid: super::LPVOID, clienttoken: super::HANDLE, desiredaccess: super::DWORD, privileges: super::PPRIVILEGE_SET, accessgranted: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectPrivilegeAuditAlarmW(subsystemname : windows_core::PCWSTR, handleid : super::LPVOID, clienttoken : super::HANDLE, desiredaccess : super::DWORD, privileges : super::PPRIVILEGE_SET, accessgranted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectPrivilegeAuditAlarmW(subsystemname.param().abi(), handleid, clienttoken, desiredaccess, privileges, accessgranted.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrivilegeCheck(clienttoken: super::HANDLE, requiredprivileges: super::PPRIVILEGE_SET, pfresult: super::LPBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn PrivilegeCheck(clienttoken : super::HANDLE, requiredprivileges : super::PPRIVILEGE_SET, pfresult : super::LPBOOL) -> windows_core::BOOL);
    unsafe { PrivilegeCheck(clienttoken, requiredprivileges as _, pfresult as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmW<P0, P1>(subsystemname: P0, servicename: P1, clienttoken: super::HANDLE, privileges: super::PPRIVILEGE_SET, accessgranted: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PrivilegedServiceAuditAlarmW(subsystemname : windows_core::PCWSTR, servicename : windows_core::PCWSTR, clienttoken : super::HANDLE, privileges : super::PPRIVILEGE_SET, accessgranted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { PrivilegedServiceAuditAlarmW(subsystemname.param().abi(), servicename.param().abi(), clienttoken, privileges, accessgranted.into()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn QuerySecurityAccessMask(securityinformation: super::SECURITY_INFORMATION) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn QuerySecurityAccessMask(securityinformation : super::SECURITY_INFORMATION, desiredaccess : super::LPDWORD));
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetAclInformation(pacl: super::PACL, paclinformation: super::LPVOID, naclinformationlength: super::DWORD, dwaclinformationclass: super::ACL_INFORMATION_CLASS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetAclInformation(pacl : super::PACL, paclinformation : super::LPVOID, naclinformationlength : super::DWORD, dwaclinformationclass : super::ACL_INFORMATION_CLASS) -> windows_core::BOOL);
    unsafe { SetAclInformation(pacl as _, paclinformation, naclinformationlength, dwaclinformationclass) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetCachedSigningLevel(sourcefiles: &[*mut core::ffi::c_void], flags: super::ULONG, targetfile: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCachedSigningLevel(sourcefiles : super::PHANDLE, sourcefilecount : super::ULONG, flags : super::ULONG, targetfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetCachedSigningLevel(core::mem::transmute(sourcefiles.as_ptr()), sourcefiles.len().try_into().unwrap(), flags, targetfile.unwrap_or(core::mem::zeroed()) as _) }
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetPrivateObjectSecurity(securityinformation: super::SECURITY_INFORMATION, modificationdescriptor: super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, genericmapping: super::PGENERIC_MAPPING, token: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetPrivateObjectSecurity(securityinformation : super::SECURITY_INFORMATION, modificationdescriptor : super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, genericmapping : super::PGENERIC_MAPPING, token : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetPrivateObjectSecurity(securityinformation, modificationdescriptor, objectssecuritydescriptor as _, genericmapping, token.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetPrivateObjectSecurityEx(securityinformation: super::SECURITY_INFORMATION, modificationdescriptor: super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, autoinheritflags: super::ULONG, genericmapping: super::PGENERIC_MAPPING, token: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetPrivateObjectSecurityEx(securityinformation : super::SECURITY_INFORMATION, modificationdescriptor : super::PSECURITY_DESCRIPTOR, objectssecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, autoinheritflags : super::ULONG, genericmapping : super::PGENERIC_MAPPING, token : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetPrivateObjectSecurityEx(securityinformation, modificationdescriptor, objectssecuritydescriptor as _, autoinheritflags, genericmapping, token.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetSecurityAccessMask(securityinformation: super::SECURITY_INFORMATION) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityAccessMask(securityinformation : super::SECURITY_INFORMATION, desiredaccess : super::LPDWORD));
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetSecurityDescriptorDacl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, bdaclpresent: bool, pdacl: Option<super::PACL>, bdacldefaulted: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorDacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, bdaclpresent : windows_core::BOOL, pdacl : super::PACL, bdacldefaulted : windows_core::BOOL) -> windows_core::BOOL);
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetSecurityDescriptorRMControl(securitydescriptor: super::PSECURITY_DESCRIPTOR, rmcontrol: Option<super::PUCHAR>) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorRMControl(securitydescriptor : super::PSECURITY_DESCRIPTOR, rmcontrol : super::PUCHAR) -> super::DWORD);
    unsafe { SetSecurityDescriptorRMControl(securitydescriptor as _, rmcontrol.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetSecurityDescriptorSacl(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, bsaclpresent: bool, psacl: Option<super::PACL>, bsacldefaulted: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityDescriptorSacl(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, bsaclpresent : windows_core::BOOL, psacl : super::PACL, bsacldefaulted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSecurityDescriptorSacl(psecuritydescriptor as _, bsaclpresent.into(), psacl.unwrap_or(core::mem::zeroed()) as _, bsacldefaulted.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetTokenInformation(tokenhandle: super::HANDLE, tokeninformationclass: super::TOKEN_INFORMATION_CLASS, tokeninformation: super::LPVOID, tokeninformationlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetTokenInformation(tokenhandle : super::HANDLE, tokeninformationclass : super::TOKEN_INFORMATION_CLASS, tokeninformation : super::LPVOID, tokeninformationlength : super::DWORD) -> windows_core::BOOL);
    unsafe { SetTokenInformation(tokenhandle, tokeninformationclass, tokeninformation, tokeninformationlength) }
}
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_NOT_VALIDATED: i32 = 1;
pub const SIGNING_LEVEL_FILE_CACHE_FLAG_VALIDATE_ONLY: i32 = 4;
pub const SIGNING_LEVEL_MICROSOFT: i32 = 8;
