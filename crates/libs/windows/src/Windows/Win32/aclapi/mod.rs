#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildExplicitAccessWithNameA<P1>(pexplicitaccess: *mut super::EXPLICIT_ACCESS_A, ptrusteename: P1, accesspermissions: u32, accessmode: super::ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameA(pexplicitaccess : *mut super::EXPLICIT_ACCESS_A, ptrusteename : windows_core::PCSTR, accesspermissions : u32, accessmode : super::ACCESS_MODE, inheritance : u32));
    unsafe { BuildExplicitAccessWithNameA(pexplicitaccess as _, ptrusteename.param().abi(), accesspermissions, accessmode, inheritance) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildExplicitAccessWithNameW<P1>(pexplicitaccess: *mut super::EXPLICIT_ACCESS_W, ptrusteename: P1, accesspermissions: u32, accessmode: super::ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameW(pexplicitaccess : *mut super::EXPLICIT_ACCESS_W, ptrusteename : windows_core::PCWSTR, accesspermissions : u32, accessmode : super::ACCESS_MODE, inheritance : u32));
    unsafe { BuildExplicitAccessWithNameW(pexplicitaccess as _, ptrusteename.param().abi(), accesspermissions, accessmode, inheritance) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameA<P1>(pexplicitaccess: *mut super::EXPLICIT_ACCESS_A, ptrusteename: P1, ptrustee: Option<*const super::TRUSTEE_A>, accesspermissions: u32, accessmode: super::ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess : *mut super::EXPLICIT_ACCESS_A, ptrusteename : windows_core::PCSTR, ptrustee : *const super::TRUSTEE_A, accesspermissions : u32, accessmode : super::ACCESS_MODE, inheritance : u32));
    unsafe { BuildImpersonateExplicitAccessWithNameA(pexplicitaccess as _, ptrusteename.param().abi(), ptrustee.unwrap_or(core::mem::zeroed()) as _, accesspermissions, accessmode, inheritance) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameW<P1>(pexplicitaccess: *mut super::EXPLICIT_ACCESS_W, ptrusteename: P1, ptrustee: Option<*const super::TRUSTEE_W>, accesspermissions: u32, accessmode: super::ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess : *mut super::EXPLICIT_ACCESS_W, ptrusteename : windows_core::PCWSTR, ptrustee : *const super::TRUSTEE_W, accesspermissions : u32, accessmode : super::ACCESS_MODE, inheritance : u32));
    unsafe { BuildImpersonateExplicitAccessWithNameW(pexplicitaccess as _, ptrusteename.param().abi(), ptrustee.unwrap_or(core::mem::zeroed()) as _, accesspermissions, accessmode, inheritance) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildImpersonateTrusteeA(ptrustee: *mut super::TRUSTEE_A, pimpersonatetrustee: Option<*const super::TRUSTEE_A>) {
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeA(ptrustee : *mut super::TRUSTEE_A, pimpersonatetrustee : *const super::TRUSTEE_A));
    unsafe { BuildImpersonateTrusteeA(ptrustee as _, pimpersonatetrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildImpersonateTrusteeW(ptrustee: *mut super::TRUSTEE_W, pimpersonatetrustee: Option<*const super::TRUSTEE_W>) {
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeW(ptrustee : *mut super::TRUSTEE_W, pimpersonatetrustee : *const super::TRUSTEE_W));
    unsafe { BuildImpersonateTrusteeW(ptrustee as _, pimpersonatetrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildSecurityDescriptorA(powner: Option<*const super::TRUSTEE_A>, pgroup: Option<*const super::TRUSTEE_A>, plistofaccessentries: Option<&[super::EXPLICIT_ACCESS_A]>, plistofauditentries: Option<&[super::EXPLICIT_ACCESS_A]>, poldsd: Option<super::PSECURITY_DESCRIPTOR>, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn BuildSecurityDescriptorA(powner : *const super::TRUSTEE_A, pgroup : *const super::TRUSTEE_A, ccountofaccessentries : u32, plistofaccessentries : *const super::EXPLICIT_ACCESS_A, ccountofauditentries : u32, plistofauditentries : *const super::EXPLICIT_ACCESS_A, poldsd : super::PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { BuildSecurityDescriptorA(powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, plistofaccessentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofaccessentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), plistofauditentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofauditentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), poldsd.unwrap_or(core::mem::zeroed()) as _, psizenewsd as _, pnewsd as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildSecurityDescriptorW(powner: Option<*const super::TRUSTEE_W>, pgroup: Option<*const super::TRUSTEE_W>, plistofaccessentries: Option<&[super::EXPLICIT_ACCESS_W]>, plistofauditentries: Option<&[super::EXPLICIT_ACCESS_W]>, poldsd: Option<super::PSECURITY_DESCRIPTOR>, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn BuildSecurityDescriptorW(powner : *const super::TRUSTEE_W, pgroup : *const super::TRUSTEE_W, ccountofaccessentries : u32, plistofaccessentries : *const super::EXPLICIT_ACCESS_W, ccountofauditentries : u32, plistofauditentries : *const super::EXPLICIT_ACCESS_W, poldsd : super::PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { BuildSecurityDescriptorW(powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, plistofaccessentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofaccessentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), plistofauditentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofauditentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), poldsd.unwrap_or(core::mem::zeroed()) as _, psizenewsd as _, pnewsd as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithNameA<P1>(ptrustee: *mut super::TRUSTEE_A, pname: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithNameA(ptrustee : *mut super::TRUSTEE_A, pname : windows_core::PCSTR));
    unsafe { BuildTrusteeWithNameA(ptrustee as _, pname.param().abi()) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithNameW<P1>(ptrustee: *mut super::TRUSTEE_W, pname: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithNameW(ptrustee : *mut super::TRUSTEE_W, pname : windows_core::PCWSTR));
    unsafe { BuildTrusteeWithNameW(ptrustee as _, pname.param().abi()) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameA<P3, P4, P5>(ptrustee: *mut super::TRUSTEE_A, pobjname: Option<*const super::OBJECTS_AND_NAME_A>, objecttype: Option<super::SE_OBJECT_TYPE>, objecttypename: P3, inheritedobjecttypename: P4, name: P5)
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameA(ptrustee : *mut super::TRUSTEE_A, pobjname : *const super::OBJECTS_AND_NAME_A, objecttype : super::SE_OBJECT_TYPE, objecttypename : windows_core::PCSTR, inheritedobjecttypename : windows_core::PCSTR, name : windows_core::PCSTR));
    unsafe { BuildTrusteeWithObjectsAndNameA(ptrustee as _, pobjname.unwrap_or(core::mem::zeroed()) as _, objecttype.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), inheritedobjecttypename.param().abi(), name.param().abi()) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameW<P3, P4, P5>(ptrustee: *mut super::TRUSTEE_W, pobjname: Option<*const super::OBJECTS_AND_NAME_W>, objecttype: Option<super::SE_OBJECT_TYPE>, objecttypename: P3, inheritedobjecttypename: P4, name: P5)
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameW(ptrustee : *mut super::TRUSTEE_W, pobjname : *const super::OBJECTS_AND_NAME_W, objecttype : super::SE_OBJECT_TYPE, objecttypename : windows_core::PCWSTR, inheritedobjecttypename : windows_core::PCWSTR, name : windows_core::PCWSTR));
    unsafe { BuildTrusteeWithObjectsAndNameW(ptrustee as _, pobjname.unwrap_or(core::mem::zeroed()) as _, objecttype.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), inheritedobjecttypename.param().abi(), name.param().abi()) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidA(ptrustee: *mut super::TRUSTEE_A, pobjsid: Option<*const super::OBJECTS_AND_SID>, pobjectguid: Option<*const windows_core::GUID>, pinheritedobjectguid: Option<*const windows_core::GUID>, psid: Option<super::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidA(ptrustee : *mut super::TRUSTEE_A, pobjsid : *const super::OBJECTS_AND_SID, pobjectguid : *const windows_core::GUID, pinheritedobjectguid : *const windows_core::GUID, psid : super::PSID));
    unsafe { BuildTrusteeWithObjectsAndSidA(ptrustee as _, pobjsid.unwrap_or(core::mem::zeroed()) as _, pobjectguid.unwrap_or(core::mem::zeroed()) as _, pinheritedobjectguid.unwrap_or(core::mem::zeroed()) as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidW(ptrustee: *mut super::TRUSTEE_W, pobjsid: Option<*const super::OBJECTS_AND_SID>, pobjectguid: Option<*const windows_core::GUID>, pinheritedobjectguid: Option<*const windows_core::GUID>, psid: Option<super::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidW(ptrustee : *mut super::TRUSTEE_W, pobjsid : *const super::OBJECTS_AND_SID, pobjectguid : *const windows_core::GUID, pinheritedobjectguid : *const windows_core::GUID, psid : super::PSID));
    unsafe { BuildTrusteeWithObjectsAndSidW(ptrustee as _, pobjsid.unwrap_or(core::mem::zeroed()) as _, pobjectguid.unwrap_or(core::mem::zeroed()) as _, pinheritedobjectguid.unwrap_or(core::mem::zeroed()) as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithSidA(ptrustee: *mut super::TRUSTEE_A, psid: Option<super::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithSidA(ptrustee : *mut super::TRUSTEE_A, psid : super::PSID));
    unsafe { BuildTrusteeWithSidA(ptrustee as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithSidW(ptrustee: *mut super::TRUSTEE_W, psid: Option<super::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithSidW(ptrustee : *mut super::TRUSTEE_W, psid : super::PSID));
    unsafe { BuildTrusteeWithSidW(ptrustee as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "accctrl")]
#[inline]
pub unsafe fn FreeInheritedFromArray(pinheritarray: &[super::INHERITED_FROMW], pfnarray: Option<*const super::FN_OBJECT_MGR_FUNCTS>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn FreeInheritedFromArray(pinheritarray : *const super::INHERITED_FROMW, acecnt : u16, pfnarray : *const super::FN_OBJECT_MGR_FUNCTS) -> u32);
    unsafe { FreeInheritedFromArray(pinheritarray.as_ptr(), pinheritarray.len().try_into().unwrap(), pfnarray.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclA(pacl: *const super::ACL, ptrustee: *const super::TRUSTEE_A, psuccessfulauditedrights: *mut super::ACCESS_MASK, pfailedauditrights: *mut super::ACCESS_MASK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclA(pacl : *const super::ACL, ptrustee : *const super::TRUSTEE_A, psuccessfulauditedrights : *mut super::ACCESS_MASK, pfailedauditrights : *mut super::ACCESS_MASK) -> u32);
    unsafe { GetAuditedPermissionsFromAclA(pacl, ptrustee, psuccessfulauditedrights as _, pfailedauditrights as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclW(pacl: *const super::ACL, ptrustee: *const super::TRUSTEE_W, psuccessfulauditedrights: *mut super::ACCESS_MASK, pfailedauditrights: *mut super::ACCESS_MASK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclW(pacl : *const super::ACL, ptrustee : *const super::TRUSTEE_W, psuccessfulauditedrights : *mut super::ACCESS_MASK, pfailedauditrights : *mut super::ACCESS_MASK) -> u32);
    unsafe { GetAuditedPermissionsFromAclW(pacl, ptrustee, psuccessfulauditedrights as _, pfailedauditrights as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetEffectiveRightsFromAclA(pacl: *const super::ACL, ptrustee: *const super::TRUSTEE_A, paccessrights: *mut super::ACCESS_MASK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclA(pacl : *const super::ACL, ptrustee : *const super::TRUSTEE_A, paccessrights : *mut super::ACCESS_MASK) -> u32);
    unsafe { GetEffectiveRightsFromAclA(pacl, ptrustee, paccessrights as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetEffectiveRightsFromAclW(pacl: *const super::ACL, ptrustee: *const super::TRUSTEE_W, paccessrights: *mut super::ACCESS_MASK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclW(pacl : *const super::ACL, ptrustee : *const super::TRUSTEE_W, paccessrights : *mut super::ACCESS_MASK) -> u32);
    unsafe { GetEffectiveRightsFromAclW(pacl, ptrustee, paccessrights as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetExplicitEntriesFromAclA(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut super::PEXPLICIT_ACCESS_A) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclA(pacl : *const super::ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut super::PEXPLICIT_ACCESS_A) -> u32);
    unsafe { GetExplicitEntriesFromAclA(pacl, pccountofexplicitentries as _, plistofexplicitentries as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetExplicitEntriesFromAclW(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut super::PEXPLICIT_ACCESS_W) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclW(pacl : *const super::ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut super::PEXPLICIT_ACCESS_W) -> u32);
    unsafe { GetExplicitEntriesFromAclW(pacl, pccountofexplicitentries as _, plistofexplicitentries as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetInheritanceSourceA<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, container: bool, pobjectclassguids: Option<&[*const windows_core::GUID]>, pacl: *const super::ACL, pfnarray: Option<*const super::FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut super::INHERITED_FROMA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetInheritanceSourceA(pobjectname : windows_core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, container : windows_core::BOOL, pobjectclassguids : *const *const windows_core::GUID, guidcount : u32, pacl : *const super::ACL, pfnarray : *const super::FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super::GENERIC_MAPPING, pinheritarray : *mut super::INHERITED_FROMA) -> u32);
    unsafe { GetInheritanceSourceA(pobjectname.param().abi(), objecttype, securityinfo, container.into(), pobjectclassguids.map_or(core::ptr::null(), |slice| slice.as_ptr()), pobjectclassguids.map_or(0, |slice| slice.len().try_into().unwrap()), pacl, pfnarray.unwrap_or(core::mem::zeroed()) as _, pgenericmapping, pinheritarray as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetInheritanceSourceW<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, container: bool, pobjectclassguids: Option<&[*const windows_core::GUID]>, pacl: *const super::ACL, pfnarray: Option<*const super::FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut super::INHERITED_FROMW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetInheritanceSourceW(pobjectname : windows_core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, container : windows_core::BOOL, pobjectclassguids : *const *const windows_core::GUID, guidcount : u32, pacl : *const super::ACL, pfnarray : *const super::FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super::GENERIC_MAPPING, pinheritarray : *mut super::INHERITED_FROMW) -> u32);
    unsafe { GetInheritanceSourceW(pobjectname.param().abi(), objecttype, securityinfo, container.into(), pobjectclassguids.map_or(core::ptr::null(), |slice| slice.as_ptr()), pobjectclassguids.map_or(0, |slice| slice.len().try_into().unwrap()), pacl, pfnarray.unwrap_or(core::mem::zeroed()) as _, pgenericmapping, pinheritarray as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetMultipleTrusteeA(ptrustee: Option<*const super::TRUSTEE_A>) -> super::PTRUSTEE_A {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeA(ptrustee : *const super::TRUSTEE_A) -> super::PTRUSTEE_A);
    unsafe { GetMultipleTrusteeA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetMultipleTrusteeOperationA(ptrustee: Option<*const super::TRUSTEE_A>) -> super::MULTIPLE_TRUSTEE_OPERATION {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationA(ptrustee : *const super::TRUSTEE_A) -> super::MULTIPLE_TRUSTEE_OPERATION);
    unsafe { GetMultipleTrusteeOperationA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetMultipleTrusteeOperationW(ptrustee: Option<*const super::TRUSTEE_W>) -> super::MULTIPLE_TRUSTEE_OPERATION {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationW(ptrustee : *const super::TRUSTEE_W) -> super::MULTIPLE_TRUSTEE_OPERATION);
    unsafe { GetMultipleTrusteeOperationW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetMultipleTrusteeW(ptrustee: Option<*const super::TRUSTEE_W>) -> super::PTRUSTEE_W {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeW(ptrustee : *const super::TRUSTEE_W) -> super::PTRUSTEE_W);
    unsafe { GetMultipleTrusteeW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut super::PACL>, ppsacl: Option<*mut super::PACL>, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, ppsidowner : *mut super::PSID, ppsidgroup : *mut super::PSID, ppdacl : *mut super::PACL, ppsacl : *mut super::PACL, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { GetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut super::PACL>, ppsacl: Option<*mut super::PACL>, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, ppsidowner : *mut super::PSID, ppsidgroup : *mut super::PSID, ppdacl : *mut super::PACL, ppsacl : *mut super::PACL, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { GetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityInfo(handle: super::HANDLE, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, ppsidowner: Option<*mut super::PSID>, ppsidgroup: Option<*mut super::PSID>, ppdacl: Option<*mut super::PACL>, ppsacl: Option<*mut super::PACL>, ppsecuritydescriptor: Option<*mut super::PSECURITY_DESCRIPTOR>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityInfo(handle : super::HANDLE, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, ppsidowner : *mut super::PSID, ppsidgroup : *mut super::PSID, ppdacl : *mut super::PACL, ppsacl : *mut super::PACL, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { GetSecurityInfo(handle, objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeFormA(ptrustee: *const super::TRUSTEE_A) -> super::TRUSTEE_FORM {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeFormA(ptrustee : *const super::TRUSTEE_A) -> super::TRUSTEE_FORM);
    unsafe { GetTrusteeFormA(ptrustee) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeFormW(ptrustee: *const super::TRUSTEE_W) -> super::TRUSTEE_FORM {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeFormW(ptrustee : *const super::TRUSTEE_W) -> super::TRUSTEE_FORM);
    unsafe { GetTrusteeFormW(ptrustee) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeNameA(ptrustee: *const super::TRUSTEE_A) -> windows_core::PSTR {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeNameA(ptrustee : *const super::TRUSTEE_A) -> windows_core::PSTR);
    unsafe { GetTrusteeNameA(ptrustee) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeNameW(ptrustee: *const super::TRUSTEE_W) -> windows_core::PWSTR {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeNameW(ptrustee : *const super::TRUSTEE_W) -> windows_core::PWSTR);
    unsafe { GetTrusteeNameW(ptrustee) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeTypeA(ptrustee: Option<*const super::TRUSTEE_A>) -> super::TRUSTEE_TYPE {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeTypeA(ptrustee : *const super::TRUSTEE_A) -> super::TRUSTEE_TYPE);
    unsafe { GetTrusteeTypeA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeTypeW(ptrustee: Option<*const super::TRUSTEE_W>) -> super::TRUSTEE_TYPE {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeTypeW(ptrustee : *const super::TRUSTEE_W) -> super::TRUSTEE_TYPE);
    unsafe { GetTrusteeTypeW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsA(ppowner: Option<*mut super::PTRUSTEE_A>, ppgroup: Option<*mut super::PTRUSTEE_A>, pccountofaccessentries: Option<*mut u32>, pplistofaccessentries: *mut super::PEXPLICIT_ACCESS_A, pccountofauditentries: Option<*mut u32>, pplistofauditentries: *mut super::PEXPLICIT_ACCESS_A, psd: super::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsA(ppowner : *mut super::PTRUSTEE_A, ppgroup : *mut super::PTRUSTEE_A, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut super::PEXPLICIT_ACCESS_A, pccountofauditentries : *mut u32, pplistofauditentries : *mut super::PEXPLICIT_ACCESS_A, psd : super::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { LookupSecurityDescriptorPartsA(ppowner.unwrap_or(core::mem::zeroed()) as _, ppgroup.unwrap_or(core::mem::zeroed()) as _, pccountofaccessentries.unwrap_or(core::mem::zeroed()) as _, pplistofaccessentries as _, pccountofauditentries.unwrap_or(core::mem::zeroed()) as _, pplistofauditentries as _, psd) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsW(ppowner: Option<*mut super::PTRUSTEE_W>, ppgroup: Option<*mut super::PTRUSTEE_W>, pccountofaccessentries: Option<*mut u32>, pplistofaccessentries: *mut super::PEXPLICIT_ACCESS_W, pccountofauditentries: Option<*mut u32>, pplistofauditentries: *mut super::PEXPLICIT_ACCESS_W, psd: super::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsW(ppowner : *mut super::PTRUSTEE_W, ppgroup : *mut super::PTRUSTEE_W, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut super::PEXPLICIT_ACCESS_W, pccountofauditentries : *mut u32, pplistofauditentries : *mut super::PEXPLICIT_ACCESS_W, psd : super::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { LookupSecurityDescriptorPartsW(ppowner.unwrap_or(core::mem::zeroed()) as _, ppgroup.unwrap_or(core::mem::zeroed()) as _, pccountofaccessentries.unwrap_or(core::mem::zeroed()) as _, pplistofaccessentries as _, pccountofauditentries.unwrap_or(core::mem::zeroed()) as _, pplistofauditentries as _, psd) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetEntriesInAclA(plistofexplicitentries: Option<&[super::EXPLICIT_ACCESS_A]>, oldacl: Option<*const super::ACL>, newacl: *mut super::PACL) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetEntriesInAclA(ccountofexplicitentries : u32, plistofexplicitentries : *const super::EXPLICIT_ACCESS_A, oldacl : *const super::ACL, newacl : *mut super::PACL) -> u32);
    unsafe { SetEntriesInAclA(plistofexplicitentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofexplicitentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), oldacl.unwrap_or(core::mem::zeroed()) as _, newacl as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetEntriesInAclW(plistofexplicitentries: Option<&[super::EXPLICIT_ACCESS_W]>, oldacl: Option<*const super::ACL>, newacl: *mut super::PACL) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetEntriesInAclW(ccountofexplicitentries : u32, plistofexplicitentries : *const super::EXPLICIT_ACCESS_W, oldacl : *const super::ACL, newacl : *mut super::PACL) -> u32);
    unsafe { SetEntriesInAclW(plistofexplicitentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofexplicitentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), oldacl.unwrap_or(core::mem::zeroed()) as _, newacl as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, psidowner: Option<super::PSID>, psidgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, psidowner : super::PSID, psidgroup : super::PSID, pdacl : *const super::ACL, psacl : *const super::ACL) -> u32);
    unsafe { SetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, psidowner: Option<super::PSID>, psidgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, psidowner : super::PSID, psidgroup : super::PSID, pdacl : *const super::ACL, psacl : *const super::ACL) -> u32);
    unsafe { SetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetSecurityInfo(handle: super::HANDLE, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, psidowner: Option<super::PSID>, psidgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityInfo(handle : super::HANDLE, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, psidowner : super::PSID, psidgroup : super::PSID, pdacl : *const super::ACL, psacl : *const super::ACL) -> u32);
    unsafe { SetSecurityInfo(handle, objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, powner: Option<super::PSID>, pgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, keepexplicit: bool, fnprogress: FN_PROGRESS, progressinvokesetting: super::PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, powner : super::PSID, pgroup : super::PSID, pdacl : *const super::ACL, psacl : *const super::ACL, keepexplicit : windows_core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : super::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
    unsafe { TreeResetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, keepexplicit.into(), fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, powner: Option<super::PSID>, pgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, keepexplicit: bool, fnprogress: FN_PROGRESS, progressinvokesetting: super::PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, powner : super::PSID, pgroup : super::PSID, pdacl : *const super::ACL, psacl : *const super::ACL, keepexplicit : windows_core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : super::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
    unsafe { TreeResetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, keepexplicit.into(), fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, powner: Option<super::PSID>, pgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, dwaction: u32, fnprogress: FN_PROGRESS, progressinvokesetting: super::PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, powner : super::PSID, pgroup : super::PSID, pdacl : *const super::ACL, psacl : *const super::ACL, dwaction : u32, fnprogress : FN_PROGRESS, progressinvokesetting : super::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
    unsafe { TreeSetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, dwaction, fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: super::SE_OBJECT_TYPE, securityinfo: super::SECURITY_INFORMATION, powner: Option<super::PSID>, pgroup: Option<super::PSID>, pdacl: Option<*const super::ACL>, psacl: Option<*const super::ACL>, dwaction: u32, fnprogress: FN_PROGRESS, progressinvokesetting: super::PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, powner : super::PSID, pgroup : super::PSID, pdacl : *const super::ACL, psacl : *const super::ACL, dwaction : u32, fnprogress : FN_PROGRESS, progressinvokesetting : super::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
    unsafe { TreeSetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, dwaction, fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "accctrl")]
pub type FN_PROGRESS = Option<unsafe extern "system" fn(pobjectname: windows_core::PCWSTR, status: u32, pinvokesetting: *mut super::PROG_INVOKE_SETTING, args: *const core::ffi::c_void, securityset: windows_core::BOOL)>;
