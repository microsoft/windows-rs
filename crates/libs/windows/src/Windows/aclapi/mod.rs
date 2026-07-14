#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildExplicitAccessWithNameA<P1>(pexplicitaccess: *mut super::accctrl::EXPLICIT_ACCESS_A, ptrusteename: P1, accesspermissions: u32, accessmode: super::accctrl::ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameA(pexplicitaccess : *mut super::accctrl::EXPLICIT_ACCESS_A, ptrusteename : windows_core::PCSTR, accesspermissions : u32, accessmode : super::accctrl::ACCESS_MODE, inheritance : u32));
    unsafe { BuildExplicitAccessWithNameA(pexplicitaccess as _, ptrusteename.param().abi(), accesspermissions, accessmode, inheritance) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildExplicitAccessWithNameW<P1>(pexplicitaccess: *mut super::accctrl::EXPLICIT_ACCESS_W, ptrusteename: P1, accesspermissions: u32, accessmode: super::accctrl::ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameW(pexplicitaccess : *mut super::accctrl::EXPLICIT_ACCESS_W, ptrusteename : windows_core::PCWSTR, accesspermissions : u32, accessmode : super::accctrl::ACCESS_MODE, inheritance : u32));
    unsafe { BuildExplicitAccessWithNameW(pexplicitaccess as _, ptrusteename.param().abi(), accesspermissions, accessmode, inheritance) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameA<P1>(pexplicitaccess: *mut super::accctrl::EXPLICIT_ACCESS_A, ptrusteename: P1, ptrustee: Option<*const super::accctrl::TRUSTEE_A>, accesspermissions: u32, accessmode: super::accctrl::ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess : *mut super::accctrl::EXPLICIT_ACCESS_A, ptrusteename : windows_core::PCSTR, ptrustee : *const super::accctrl::TRUSTEE_A, accesspermissions : u32, accessmode : super::accctrl::ACCESS_MODE, inheritance : u32));
    unsafe { BuildImpersonateExplicitAccessWithNameA(pexplicitaccess as _, ptrusteename.param().abi(), ptrustee.unwrap_or(core::mem::zeroed()) as _, accesspermissions, accessmode, inheritance) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameW<P1>(pexplicitaccess: *mut super::accctrl::EXPLICIT_ACCESS_W, ptrusteename: P1, ptrustee: Option<*const super::accctrl::TRUSTEE_W>, accesspermissions: u32, accessmode: super::accctrl::ACCESS_MODE, inheritance: u32)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess : *mut super::accctrl::EXPLICIT_ACCESS_W, ptrusteename : windows_core::PCWSTR, ptrustee : *const super::accctrl::TRUSTEE_W, accesspermissions : u32, accessmode : super::accctrl::ACCESS_MODE, inheritance : u32));
    unsafe { BuildImpersonateExplicitAccessWithNameW(pexplicitaccess as _, ptrusteename.param().abi(), ptrustee.unwrap_or(core::mem::zeroed()) as _, accesspermissions, accessmode, inheritance) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildImpersonateTrusteeA(ptrustee: *mut super::accctrl::TRUSTEE_A, pimpersonatetrustee: Option<*const super::accctrl::TRUSTEE_A>) {
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeA(ptrustee : *mut super::accctrl::TRUSTEE_A, pimpersonatetrustee : *const super::accctrl::TRUSTEE_A));
    unsafe { BuildImpersonateTrusteeA(ptrustee as _, pimpersonatetrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildImpersonateTrusteeW(ptrustee: *mut super::accctrl::TRUSTEE_W, pimpersonatetrustee: Option<*const super::accctrl::TRUSTEE_W>) {
    windows_core::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeW(ptrustee : *mut super::accctrl::TRUSTEE_W, pimpersonatetrustee : *const super::accctrl::TRUSTEE_W));
    unsafe { BuildImpersonateTrusteeW(ptrustee as _, pimpersonatetrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildSecurityDescriptorA(powner: Option<*const super::accctrl::TRUSTEE_A>, pgroup: Option<*const super::accctrl::TRUSTEE_A>, plistofaccessentries: Option<&[super::accctrl::EXPLICIT_ACCESS_A]>, plistofauditentries: Option<&[super::accctrl::EXPLICIT_ACCESS_A]>, poldsd: Option<super::winnt::PSECURITY_DESCRIPTOR>, psizenewsd: *mut u32, pnewsd: *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn BuildSecurityDescriptorA(powner : *const super::accctrl::TRUSTEE_A, pgroup : *const super::accctrl::TRUSTEE_A, ccountofaccessentries : u32, plistofaccessentries : *const super::accctrl::EXPLICIT_ACCESS_A, ccountofauditentries : u32, plistofauditentries : *const super::accctrl::EXPLICIT_ACCESS_A, poldsd : super::winnt::PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { BuildSecurityDescriptorA(powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, plistofaccessentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofaccessentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), plistofauditentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofauditentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), poldsd.unwrap_or(core::mem::zeroed()) as _, psizenewsd as _, pnewsd as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildSecurityDescriptorW(powner: Option<*const super::accctrl::TRUSTEE_W>, pgroup: Option<*const super::accctrl::TRUSTEE_W>, plistofaccessentries: Option<&[super::accctrl::EXPLICIT_ACCESS_W]>, plistofauditentries: Option<&[super::accctrl::EXPLICIT_ACCESS_W]>, poldsd: Option<super::winnt::PSECURITY_DESCRIPTOR>, psizenewsd: *mut u32, pnewsd: *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn BuildSecurityDescriptorW(powner : *const super::accctrl::TRUSTEE_W, pgroup : *const super::accctrl::TRUSTEE_W, ccountofaccessentries : u32, plistofaccessentries : *const super::accctrl::EXPLICIT_ACCESS_W, ccountofauditentries : u32, plistofauditentries : *const super::accctrl::EXPLICIT_ACCESS_W, poldsd : super::winnt::PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { BuildSecurityDescriptorW(powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, plistofaccessentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofaccessentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), plistofauditentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofauditentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), poldsd.unwrap_or(core::mem::zeroed()) as _, psizenewsd as _, pnewsd as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithNameA<P1>(ptrustee: *mut super::accctrl::TRUSTEE_A, pname: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithNameA(ptrustee : *mut super::accctrl::TRUSTEE_A, pname : windows_core::PCSTR));
    unsafe { BuildTrusteeWithNameA(ptrustee as _, pname.param().abi()) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithNameW<P1>(ptrustee: *mut super::accctrl::TRUSTEE_W, pname: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithNameW(ptrustee : *mut super::accctrl::TRUSTEE_W, pname : windows_core::PCWSTR));
    unsafe { BuildTrusteeWithNameW(ptrustee as _, pname.param().abi()) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameA<P3, P4, P5>(ptrustee: *mut super::accctrl::TRUSTEE_A, pobjname: Option<*const super::accctrl::OBJECTS_AND_NAME_A>, objecttype: Option<super::accctrl::SE_OBJECT_TYPE>, objecttypename: P3, inheritedobjecttypename: P4, name: P5)
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameA(ptrustee : *mut super::accctrl::TRUSTEE_A, pobjname : *const super::accctrl::OBJECTS_AND_NAME_A, objecttype : super::accctrl::SE_OBJECT_TYPE, objecttypename : windows_core::PCSTR, inheritedobjecttypename : windows_core::PCSTR, name : windows_core::PCSTR));
    unsafe { BuildTrusteeWithObjectsAndNameA(ptrustee as _, pobjname.unwrap_or(core::mem::zeroed()) as _, objecttype.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), inheritedobjecttypename.param().abi(), name.param().abi()) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameW<P3, P4, P5>(ptrustee: *mut super::accctrl::TRUSTEE_W, pobjname: Option<*const super::accctrl::OBJECTS_AND_NAME_W>, objecttype: Option<super::accctrl::SE_OBJECT_TYPE>, objecttypename: P3, inheritedobjecttypename: P4, name: P5)
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameW(ptrustee : *mut super::accctrl::TRUSTEE_W, pobjname : *const super::accctrl::OBJECTS_AND_NAME_W, objecttype : super::accctrl::SE_OBJECT_TYPE, objecttypename : windows_core::PCWSTR, inheritedobjecttypename : windows_core::PCWSTR, name : windows_core::PCWSTR));
    unsafe { BuildTrusteeWithObjectsAndNameW(ptrustee as _, pobjname.unwrap_or(core::mem::zeroed()) as _, objecttype.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), inheritedobjecttypename.param().abi(), name.param().abi()) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidA(ptrustee: *mut super::accctrl::TRUSTEE_A, pobjsid: Option<*const super::accctrl::OBJECTS_AND_SID>, pobjectguid: Option<*const windows_core::GUID>, pinheritedobjectguid: Option<*const windows_core::GUID>, psid: Option<super::winnt::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidA(ptrustee : *mut super::accctrl::TRUSTEE_A, pobjsid : *const super::accctrl::OBJECTS_AND_SID, pobjectguid : *const windows_core::GUID, pinheritedobjectguid : *const windows_core::GUID, psid : super::winnt::PSID));
    unsafe { BuildTrusteeWithObjectsAndSidA(ptrustee as _, pobjsid.unwrap_or(core::mem::zeroed()) as _, pobjectguid.unwrap_or(core::mem::zeroed()) as _, pinheritedobjectguid.unwrap_or(core::mem::zeroed()) as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidW(ptrustee: *mut super::accctrl::TRUSTEE_W, pobjsid: Option<*const super::accctrl::OBJECTS_AND_SID>, pobjectguid: Option<*const windows_core::GUID>, pinheritedobjectguid: Option<*const windows_core::GUID>, psid: Option<super::winnt::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidW(ptrustee : *mut super::accctrl::TRUSTEE_W, pobjsid : *const super::accctrl::OBJECTS_AND_SID, pobjectguid : *const windows_core::GUID, pinheritedobjectguid : *const windows_core::GUID, psid : super::winnt::PSID));
    unsafe { BuildTrusteeWithObjectsAndSidW(ptrustee as _, pobjsid.unwrap_or(core::mem::zeroed()) as _, pobjectguid.unwrap_or(core::mem::zeroed()) as _, pinheritedobjectguid.unwrap_or(core::mem::zeroed()) as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithSidA(ptrustee: *mut super::accctrl::TRUSTEE_A, psid: Option<super::winnt::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithSidA(ptrustee : *mut super::accctrl::TRUSTEE_A, psid : super::winnt::PSID));
    unsafe { BuildTrusteeWithSidA(ptrustee as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn BuildTrusteeWithSidW(ptrustee: *mut super::accctrl::TRUSTEE_W, psid: Option<super::winnt::PSID>) {
    windows_core::link!("advapi32.dll" "system" fn BuildTrusteeWithSidW(ptrustee : *mut super::accctrl::TRUSTEE_W, psid : super::winnt::PSID));
    unsafe { BuildTrusteeWithSidW(ptrustee as _, psid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "accctrl")]
#[inline]
pub unsafe fn FreeInheritedFromArray(pinheritarray: &[super::accctrl::INHERITED_FROMW], pfnarray: Option<*const super::accctrl::FN_OBJECT_MGR_FUNCTS>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn FreeInheritedFromArray(pinheritarray : *const super::accctrl::INHERITED_FROMW, acecnt : u16, pfnarray : *const super::accctrl::FN_OBJECT_MGR_FUNCTS) -> u32);
    unsafe { FreeInheritedFromArray(pinheritarray.as_ptr(), pinheritarray.len().try_into().unwrap(), pfnarray.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclA(pacl: *const super::winnt::ACL, ptrustee: *const super::accctrl::TRUSTEE_A, psuccessfulauditedrights: *mut super::winnt::ACCESS_MASK, pfailedauditrights: *mut super::winnt::ACCESS_MASK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclA(pacl : *const super::winnt::ACL, ptrustee : *const super::accctrl::TRUSTEE_A, psuccessfulauditedrights : *mut super::winnt::ACCESS_MASK, pfailedauditrights : *mut super::winnt::ACCESS_MASK) -> u32);
    unsafe { GetAuditedPermissionsFromAclA(pacl, ptrustee, psuccessfulauditedrights as _, pfailedauditrights as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclW(pacl: *const super::winnt::ACL, ptrustee: *const super::accctrl::TRUSTEE_W, psuccessfulauditedrights: *mut super::winnt::ACCESS_MASK, pfailedauditrights: *mut super::winnt::ACCESS_MASK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclW(pacl : *const super::winnt::ACL, ptrustee : *const super::accctrl::TRUSTEE_W, psuccessfulauditedrights : *mut super::winnt::ACCESS_MASK, pfailedauditrights : *mut super::winnt::ACCESS_MASK) -> u32);
    unsafe { GetAuditedPermissionsFromAclW(pacl, ptrustee, psuccessfulauditedrights as _, pfailedauditrights as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetEffectiveRightsFromAclA(pacl: *const super::winnt::ACL, ptrustee: *const super::accctrl::TRUSTEE_A, paccessrights: *mut super::winnt::ACCESS_MASK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclA(pacl : *const super::winnt::ACL, ptrustee : *const super::accctrl::TRUSTEE_A, paccessrights : *mut super::winnt::ACCESS_MASK) -> u32);
    unsafe { GetEffectiveRightsFromAclA(pacl, ptrustee, paccessrights as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetEffectiveRightsFromAclW(pacl: *const super::winnt::ACL, ptrustee: *const super::accctrl::TRUSTEE_W, paccessrights: *mut super::winnt::ACCESS_MASK) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclW(pacl : *const super::winnt::ACL, ptrustee : *const super::accctrl::TRUSTEE_W, paccessrights : *mut super::winnt::ACCESS_MASK) -> u32);
    unsafe { GetEffectiveRightsFromAclW(pacl, ptrustee, paccessrights as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetExplicitEntriesFromAclA(pacl: *const super::winnt::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut super::accctrl::PEXPLICIT_ACCESS_A) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclA(pacl : *const super::winnt::ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut super::accctrl::PEXPLICIT_ACCESS_A) -> u32);
    unsafe { GetExplicitEntriesFromAclA(pacl, pccountofexplicitentries as _, plistofexplicitentries as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetExplicitEntriesFromAclW(pacl: *const super::winnt::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut super::accctrl::PEXPLICIT_ACCESS_W) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclW(pacl : *const super::winnt::ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut super::accctrl::PEXPLICIT_ACCESS_W) -> u32);
    unsafe { GetExplicitEntriesFromAclW(pacl, pccountofexplicitentries as _, plistofexplicitentries as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetInheritanceSourceA<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, container: bool, pobjectclassguids: Option<&[*const windows_core::GUID]>, pacl: *const super::winnt::ACL, pfnarray: Option<*const super::accctrl::FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::winnt::GENERIC_MAPPING, pinheritarray: *mut super::accctrl::INHERITED_FROMA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetInheritanceSourceA(pobjectname : windows_core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, container : windows_core::BOOL, pobjectclassguids : *const *const windows_core::GUID, guidcount : u32, pacl : *const super::winnt::ACL, pfnarray : *const super::accctrl::FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super::winnt::GENERIC_MAPPING, pinheritarray : *mut super::accctrl::INHERITED_FROMA) -> u32);
    unsafe { GetInheritanceSourceA(pobjectname.param().abi(), objecttype, securityinfo, container.into(), pobjectclassguids.map_or(core::ptr::null(), |slice| slice.as_ptr()), pobjectclassguids.map_or(0, |slice| slice.len().try_into().unwrap()), pacl, pfnarray.unwrap_or(core::mem::zeroed()) as _, pgenericmapping, pinheritarray as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetInheritanceSourceW<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, container: bool, pobjectclassguids: Option<&[*const windows_core::GUID]>, pacl: *const super::winnt::ACL, pfnarray: Option<*const super::accctrl::FN_OBJECT_MGR_FUNCTS>, pgenericmapping: *const super::winnt::GENERIC_MAPPING, pinheritarray: *mut super::accctrl::INHERITED_FROMW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetInheritanceSourceW(pobjectname : windows_core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, container : windows_core::BOOL, pobjectclassguids : *const *const windows_core::GUID, guidcount : u32, pacl : *const super::winnt::ACL, pfnarray : *const super::accctrl::FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super::winnt::GENERIC_MAPPING, pinheritarray : *mut super::accctrl::INHERITED_FROMW) -> u32);
    unsafe { GetInheritanceSourceW(pobjectname.param().abi(), objecttype, securityinfo, container.into(), pobjectclassguids.map_or(core::ptr::null(), |slice| slice.as_ptr()), pobjectclassguids.map_or(0, |slice| slice.len().try_into().unwrap()), pacl, pfnarray.unwrap_or(core::mem::zeroed()) as _, pgenericmapping, pinheritarray as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetMultipleTrusteeA(ptrustee: Option<*const super::accctrl::TRUSTEE_A>) -> super::accctrl::PTRUSTEE_A {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeA(ptrustee : *const super::accctrl::TRUSTEE_A) -> super::accctrl::PTRUSTEE_A);
    unsafe { GetMultipleTrusteeA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetMultipleTrusteeOperationA(ptrustee: Option<*const super::accctrl::TRUSTEE_A>) -> super::accctrl::MULTIPLE_TRUSTEE_OPERATION {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationA(ptrustee : *const super::accctrl::TRUSTEE_A) -> super::accctrl::MULTIPLE_TRUSTEE_OPERATION);
    unsafe { GetMultipleTrusteeOperationA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetMultipleTrusteeOperationW(ptrustee: Option<*const super::accctrl::TRUSTEE_W>) -> super::accctrl::MULTIPLE_TRUSTEE_OPERATION {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationW(ptrustee : *const super::accctrl::TRUSTEE_W) -> super::accctrl::MULTIPLE_TRUSTEE_OPERATION);
    unsafe { GetMultipleTrusteeOperationW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetMultipleTrusteeW(ptrustee: Option<*const super::accctrl::TRUSTEE_W>) -> super::accctrl::PTRUSTEE_W {
    windows_core::link!("advapi32.dll" "system" fn GetMultipleTrusteeW(ptrustee : *const super::accctrl::TRUSTEE_W) -> super::accctrl::PTRUSTEE_W);
    unsafe { GetMultipleTrusteeW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, ppsidowner: Option<*mut super::winnt::PSID>, ppsidgroup: Option<*mut super::winnt::PSID>, ppdacl: Option<*mut super::winnt::PACL>, ppsacl: Option<*mut super::winnt::PACL>, ppsecuritydescriptor: *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, ppsidowner : *mut super::winnt::PSID, ppsidgroup : *mut super::winnt::PSID, ppdacl : *mut super::winnt::PACL, ppsacl : *mut super::winnt::PACL, ppsecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { GetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, ppsidowner: Option<*mut super::winnt::PSID>, ppsidgroup: Option<*mut super::winnt::PSID>, ppdacl: Option<*mut super::winnt::PACL>, ppsacl: Option<*mut super::winnt::PACL>, ppsecuritydescriptor: *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, ppsidowner : *mut super::winnt::PSID, ppsidgroup : *mut super::winnt::PSID, ppdacl : *mut super::winnt::PACL, ppsacl : *mut super::winnt::PACL, ppsecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { GetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetSecurityInfo(handle: super::winnt::HANDLE, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, ppsidowner: Option<*mut super::winnt::PSID>, ppsidgroup: Option<*mut super::winnt::PSID>, ppdacl: Option<*mut super::winnt::PACL>, ppsacl: Option<*mut super::winnt::PACL>, ppsecuritydescriptor: Option<*mut super::winnt::PSECURITY_DESCRIPTOR>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetSecurityInfo(handle : super::winnt::HANDLE, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, ppsidowner : *mut super::winnt::PSID, ppsidgroup : *mut super::winnt::PSID, ppdacl : *mut super::winnt::PACL, ppsacl : *mut super::winnt::PACL, ppsecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { GetSecurityInfo(handle, objecttype, securityinfo, ppsidowner.unwrap_or(core::mem::zeroed()) as _, ppsidgroup.unwrap_or(core::mem::zeroed()) as _, ppdacl.unwrap_or(core::mem::zeroed()) as _, ppsacl.unwrap_or(core::mem::zeroed()) as _, ppsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeFormA(ptrustee: *const super::accctrl::TRUSTEE_A) -> super::accctrl::TRUSTEE_FORM {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeFormA(ptrustee : *const super::accctrl::TRUSTEE_A) -> super::accctrl::TRUSTEE_FORM);
    unsafe { GetTrusteeFormA(ptrustee) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeFormW(ptrustee: *const super::accctrl::TRUSTEE_W) -> super::accctrl::TRUSTEE_FORM {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeFormW(ptrustee : *const super::accctrl::TRUSTEE_W) -> super::accctrl::TRUSTEE_FORM);
    unsafe { GetTrusteeFormW(ptrustee) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeNameA(ptrustee: *const super::accctrl::TRUSTEE_A) -> windows_core::PSTR {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeNameA(ptrustee : *const super::accctrl::TRUSTEE_A) -> windows_core::PSTR);
    unsafe { GetTrusteeNameA(ptrustee) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeNameW(ptrustee: *const super::accctrl::TRUSTEE_W) -> windows_core::PWSTR {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeNameW(ptrustee : *const super::accctrl::TRUSTEE_W) -> windows_core::PWSTR);
    unsafe { GetTrusteeNameW(ptrustee) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeTypeA(ptrustee: Option<*const super::accctrl::TRUSTEE_A>) -> super::accctrl::TRUSTEE_TYPE {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeTypeA(ptrustee : *const super::accctrl::TRUSTEE_A) -> super::accctrl::TRUSTEE_TYPE);
    unsafe { GetTrusteeTypeA(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn GetTrusteeTypeW(ptrustee: Option<*const super::accctrl::TRUSTEE_W>) -> super::accctrl::TRUSTEE_TYPE {
    windows_core::link!("advapi32.dll" "system" fn GetTrusteeTypeW(ptrustee : *const super::accctrl::TRUSTEE_W) -> super::accctrl::TRUSTEE_TYPE);
    unsafe { GetTrusteeTypeW(ptrustee.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsA(ppowner: Option<*mut super::accctrl::PTRUSTEE_A>, ppgroup: Option<*mut super::accctrl::PTRUSTEE_A>, pccountofaccessentries: Option<*mut u32>, pplistofaccessentries: *mut super::accctrl::PEXPLICIT_ACCESS_A, pccountofauditentries: Option<*mut u32>, pplistofauditentries: *mut super::accctrl::PEXPLICIT_ACCESS_A, psd: super::winnt::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsA(ppowner : *mut super::accctrl::PTRUSTEE_A, ppgroup : *mut super::accctrl::PTRUSTEE_A, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut super::accctrl::PEXPLICIT_ACCESS_A, pccountofauditentries : *mut u32, pplistofauditentries : *mut super::accctrl::PEXPLICIT_ACCESS_A, psd : super::winnt::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { LookupSecurityDescriptorPartsA(ppowner.unwrap_or(core::mem::zeroed()) as _, ppgroup.unwrap_or(core::mem::zeroed()) as _, pccountofaccessentries.unwrap_or(core::mem::zeroed()) as _, pplistofaccessentries as _, pccountofauditentries.unwrap_or(core::mem::zeroed()) as _, pplistofauditentries as _, psd) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsW(ppowner: Option<*mut super::accctrl::PTRUSTEE_W>, ppgroup: Option<*mut super::accctrl::PTRUSTEE_W>, pccountofaccessentries: Option<*mut u32>, pplistofaccessentries: *mut super::accctrl::PEXPLICIT_ACCESS_W, pccountofauditentries: Option<*mut u32>, pplistofauditentries: *mut super::accctrl::PEXPLICIT_ACCESS_W, psd: super::winnt::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsW(ppowner : *mut super::accctrl::PTRUSTEE_W, ppgroup : *mut super::accctrl::PTRUSTEE_W, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut super::accctrl::PEXPLICIT_ACCESS_W, pccountofauditentries : *mut u32, pplistofauditentries : *mut super::accctrl::PEXPLICIT_ACCESS_W, psd : super::winnt::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { LookupSecurityDescriptorPartsW(ppowner.unwrap_or(core::mem::zeroed()) as _, ppgroup.unwrap_or(core::mem::zeroed()) as _, pccountofaccessentries.unwrap_or(core::mem::zeroed()) as _, pplistofaccessentries as _, pccountofauditentries.unwrap_or(core::mem::zeroed()) as _, pplistofauditentries as _, psd) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetEntriesInAclA(plistofexplicitentries: Option<&[super::accctrl::EXPLICIT_ACCESS_A]>, oldacl: Option<*const super::winnt::ACL>, newacl: *mut super::winnt::PACL) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetEntriesInAclA(ccountofexplicitentries : u32, plistofexplicitentries : *const super::accctrl::EXPLICIT_ACCESS_A, oldacl : *const super::winnt::ACL, newacl : *mut super::winnt::PACL) -> u32);
    unsafe { SetEntriesInAclA(plistofexplicitentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofexplicitentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), oldacl.unwrap_or(core::mem::zeroed()) as _, newacl as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetEntriesInAclW(plistofexplicitentries: Option<&[super::accctrl::EXPLICIT_ACCESS_W]>, oldacl: Option<*const super::winnt::ACL>, newacl: *mut super::winnt::PACL) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetEntriesInAclW(ccountofexplicitentries : u32, plistofexplicitentries : *const super::accctrl::EXPLICIT_ACCESS_W, oldacl : *const super::winnt::ACL, newacl : *mut super::winnt::PACL) -> u32);
    unsafe { SetEntriesInAclW(plistofexplicitentries.map_or(0, |slice| slice.len().try_into().unwrap()), plistofexplicitentries.map_or(core::ptr::null(), |slice| slice.as_ptr()), oldacl.unwrap_or(core::mem::zeroed()) as _, newacl as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, psidowner: Option<super::winnt::PSID>, psidgroup: Option<super::winnt::PSID>, pdacl: Option<*const super::winnt::ACL>, psacl: Option<*const super::winnt::ACL>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, psidowner : super::winnt::PSID, psidgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL) -> u32);
    unsafe { SetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, psidowner: Option<super::winnt::PSID>, psidgroup: Option<super::winnt::PSID>, pdacl: Option<*const super::winnt::ACL>, psacl: Option<*const super::winnt::ACL>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, psidowner : super::winnt::PSID, psidgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL) -> u32);
    unsafe { SetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn SetSecurityInfo(handle: super::winnt::HANDLE, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, psidowner: Option<super::winnt::PSID>, psidgroup: Option<super::winnt::PSID>, pdacl: Option<*const super::winnt::ACL>, psacl: Option<*const super::winnt::ACL>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetSecurityInfo(handle : super::winnt::HANDLE, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, psidowner : super::winnt::PSID, psidgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL) -> u32);
    unsafe { SetSecurityInfo(handle, objecttype, securityinfo, psidowner.unwrap_or(core::mem::zeroed()) as _, psidgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, powner: Option<super::winnt::PSID>, pgroup: Option<super::winnt::PSID>, pdacl: Option<*const super::winnt::ACL>, psacl: Option<*const super::winnt::ACL>, keepexplicit: bool, fnprogress: FN_PROGRESS, progressinvokesetting: super::accctrl::PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, powner : super::winnt::PSID, pgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL, keepexplicit : windows_core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : super::accctrl::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
    unsafe { TreeResetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, keepexplicit.into(), fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, powner: Option<super::winnt::PSID>, pgroup: Option<super::winnt::PSID>, pdacl: Option<*const super::winnt::ACL>, psacl: Option<*const super::winnt::ACL>, keepexplicit: bool, fnprogress: FN_PROGRESS, progressinvokesetting: super::accctrl::PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, powner : super::winnt::PSID, pgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL, keepexplicit : windows_core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : super::accctrl::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
    unsafe { TreeResetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, keepexplicit.into(), fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoA<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, powner: Option<super::winnt::PSID>, pgroup: Option<super::winnt::PSID>, pdacl: Option<*const super::winnt::ACL>, psacl: Option<*const super::winnt::ACL>, dwaction: u32, fnprogress: FN_PROGRESS, progressinvokesetting: super::accctrl::PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoA(pobjectname : windows_core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, powner : super::winnt::PSID, pgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL, dwaction : u32, fnprogress : FN_PROGRESS, progressinvokesetting : super::accctrl::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
    unsafe { TreeSetNamedSecurityInfoA(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, dwaction, fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "accctrl", feature = "winnt"))]
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoW<P0>(pobjectname: P0, objecttype: super::accctrl::SE_OBJECT_TYPE, securityinfo: super::winnt::SECURITY_INFORMATION, powner: Option<super::winnt::PSID>, pgroup: Option<super::winnt::PSID>, pdacl: Option<*const super::winnt::ACL>, psacl: Option<*const super::winnt::ACL>, dwaction: u32, fnprogress: FN_PROGRESS, progressinvokesetting: super::accctrl::PROG_INVOKE_SETTING, args: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoW(pobjectname : windows_core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, powner : super::winnt::PSID, pgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL, dwaction : u32, fnprogress : FN_PROGRESS, progressinvokesetting : super::accctrl::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
    unsafe { TreeSetNamedSecurityInfoW(pobjectname.param().abi(), objecttype, securityinfo, powner.unwrap_or(core::mem::zeroed()) as _, pgroup.unwrap_or(core::mem::zeroed()) as _, pdacl.unwrap_or(core::mem::zeroed()) as _, psacl.unwrap_or(core::mem::zeroed()) as _, dwaction, fnprogress, progressinvokesetting, args.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "accctrl")]
pub type FN_PROGRESS = Option<unsafe extern "system" fn(pobjectname: windows_core::PCWSTR, status: u32, pinvokesetting: *mut super::accctrl::PROG_INVOKE_SETTING, args: *const core::ffi::c_void, securityset: windows_core::BOOL)>;
