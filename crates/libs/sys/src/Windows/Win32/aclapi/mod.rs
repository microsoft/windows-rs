#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameA(pexplicitaccess : *mut super::accctrl::EXPLICIT_ACCESS_A, ptrusteename : windows_sys::core::PCSTR, accesspermissions : u32, accessmode : super::accctrl::ACCESS_MODE, inheritance : u32));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameW(pexplicitaccess : *mut super::accctrl::EXPLICIT_ACCESS_W, ptrusteename : windows_sys::core::PCWSTR, accesspermissions : u32, accessmode : super::accctrl::ACCESS_MODE, inheritance : u32));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess : *mut super::accctrl::EXPLICIT_ACCESS_A, ptrusteename : windows_sys::core::PCSTR, ptrustee : *const super::accctrl::TRUSTEE_A, accesspermissions : u32, accessmode : super::accctrl::ACCESS_MODE, inheritance : u32));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess : *mut super::accctrl::EXPLICIT_ACCESS_W, ptrusteename : windows_sys::core::PCWSTR, ptrustee : *const super::accctrl::TRUSTEE_W, accesspermissions : u32, accessmode : super::accctrl::ACCESS_MODE, inheritance : u32));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeA(ptrustee : *mut super::accctrl::TRUSTEE_A, pimpersonatetrustee : *const super::accctrl::TRUSTEE_A));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeW(ptrustee : *mut super::accctrl::TRUSTEE_W, pimpersonatetrustee : *const super::accctrl::TRUSTEE_W));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildSecurityDescriptorA(powner : *const super::accctrl::TRUSTEE_A, pgroup : *const super::accctrl::TRUSTEE_A, ccountofaccessentries : u32, plistofaccessentries : *const super::accctrl::EXPLICIT_ACCESS_A, ccountofauditentries : u32, plistofauditentries : *const super::accctrl::EXPLICIT_ACCESS_A, poldsd : super::winnt::PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildSecurityDescriptorW(powner : *const super::accctrl::TRUSTEE_W, pgroup : *const super::accctrl::TRUSTEE_W, ccountofaccessentries : u32, plistofaccessentries : *const super::accctrl::EXPLICIT_ACCESS_W, ccountofauditentries : u32, plistofauditentries : *const super::accctrl::EXPLICIT_ACCESS_W, poldsd : super::winnt::PSECURITY_DESCRIPTOR, psizenewsd : *mut u32, pnewsd : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithNameA(ptrustee : *mut super::accctrl::TRUSTEE_A, pname : windows_sys::core::PCSTR));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithNameW(ptrustee : *mut super::accctrl::TRUSTEE_W, pname : windows_sys::core::PCWSTR));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameA(ptrustee : *mut super::accctrl::TRUSTEE_A, pobjname : *const super::accctrl::OBJECTS_AND_NAME_A, objecttype : super::accctrl::SE_OBJECT_TYPE, objecttypename : windows_sys::core::PCSTR, inheritedobjecttypename : windows_sys::core::PCSTR, name : windows_sys::core::PCSTR));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameW(ptrustee : *mut super::accctrl::TRUSTEE_W, pobjname : *const super::accctrl::OBJECTS_AND_NAME_W, objecttype : super::accctrl::SE_OBJECT_TYPE, objecttypename : windows_sys::core::PCWSTR, inheritedobjecttypename : windows_sys::core::PCWSTR, name : windows_sys::core::PCWSTR));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidA(ptrustee : *mut super::accctrl::TRUSTEE_A, pobjsid : *const super::accctrl::OBJECTS_AND_SID, pobjectguid : *const windows_sys::core::GUID, pinheritedobjectguid : *const windows_sys::core::GUID, psid : super::winnt::PSID));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidW(ptrustee : *mut super::accctrl::TRUSTEE_W, pobjsid : *const super::accctrl::OBJECTS_AND_SID, pobjectguid : *const windows_sys::core::GUID, pinheritedobjectguid : *const windows_sys::core::GUID, psid : super::winnt::PSID));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithSidA(ptrustee : *mut super::accctrl::TRUSTEE_A, psid : super::winnt::PSID));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithSidW(ptrustee : *mut super::accctrl::TRUSTEE_W, psid : super::winnt::PSID));
#[cfg(feature = "accctrl")]
windows_link::link!("advapi32.dll" "system" fn FreeInheritedFromArray(pinheritarray : *const super::accctrl::INHERITED_FROMW, acecnt : u16, pfnarray : *const super::accctrl::FN_OBJECT_MGR_FUNCTS) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclA(pacl : *const super::winnt::ACL, ptrustee : *const super::accctrl::TRUSTEE_A, psuccessfulauditedrights : *mut super::winnt::ACCESS_MASK, pfailedauditrights : *mut super::winnt::ACCESS_MASK) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclW(pacl : *const super::winnt::ACL, ptrustee : *const super::accctrl::TRUSTEE_W, psuccessfulauditedrights : *mut super::winnt::ACCESS_MASK, pfailedauditrights : *mut super::winnt::ACCESS_MASK) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclA(pacl : *const super::winnt::ACL, ptrustee : *const super::accctrl::TRUSTEE_A, paccessrights : *mut super::winnt::ACCESS_MASK) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclW(pacl : *const super::winnt::ACL, ptrustee : *const super::accctrl::TRUSTEE_W, paccessrights : *mut super::winnt::ACCESS_MASK) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclA(pacl : *const super::winnt::ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut super::accctrl::PEXPLICIT_ACCESS_A) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclW(pacl : *const super::winnt::ACL, pccountofexplicitentries : *mut u32, plistofexplicitentries : *mut super::accctrl::PEXPLICIT_ACCESS_W) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetInheritanceSourceA(pobjectname : windows_sys::core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, container : windows_sys::core::BOOL, pobjectclassguids : *const *const windows_sys::core::GUID, guidcount : u32, pacl : *const super::winnt::ACL, pfnarray : *const super::accctrl::FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super::winnt::GENERIC_MAPPING, pinheritarray : *mut super::accctrl::INHERITED_FROMA) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetInheritanceSourceW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, container : windows_sys::core::BOOL, pobjectclassguids : *const *const windows_sys::core::GUID, guidcount : u32, pacl : *const super::winnt::ACL, pfnarray : *const super::accctrl::FN_OBJECT_MGR_FUNCTS, pgenericmapping : *const super::winnt::GENERIC_MAPPING, pinheritarray : *mut super::accctrl::INHERITED_FROMW) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetMultipleTrusteeA(ptrustee : *const super::accctrl::TRUSTEE_A) -> super::accctrl::PTRUSTEE_A);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationA(ptrustee : *const super::accctrl::TRUSTEE_A) -> super::accctrl::MULTIPLE_TRUSTEE_OPERATION);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationW(ptrustee : *const super::accctrl::TRUSTEE_W) -> super::accctrl::MULTIPLE_TRUSTEE_OPERATION);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetMultipleTrusteeW(ptrustee : *const super::accctrl::TRUSTEE_W) -> super::accctrl::PTRUSTEE_W);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetNamedSecurityInfoA(pobjectname : windows_sys::core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, ppsidowner : *mut super::winnt::PSID, ppsidgroup : *mut super::winnt::PSID, ppdacl : *mut super::winnt::PACL, ppsacl : *mut super::winnt::PACL, ppsecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetNamedSecurityInfoW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, ppsidowner : *mut super::winnt::PSID, ppsidgroup : *mut super::winnt::PSID, ppdacl : *mut super::winnt::PACL, ppsacl : *mut super::winnt::PACL, ppsecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSecurityInfo(handle : super::winnt::HANDLE, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, ppsidowner : *mut super::winnt::PSID, ppsidgroup : *mut super::winnt::PSID, ppdacl : *mut super::winnt::PACL, ppsacl : *mut super::winnt::PACL, ppsecuritydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeFormA(ptrustee : *const super::accctrl::TRUSTEE_A) -> super::accctrl::TRUSTEE_FORM);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeFormW(ptrustee : *const super::accctrl::TRUSTEE_W) -> super::accctrl::TRUSTEE_FORM);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeNameA(ptrustee : *const super::accctrl::TRUSTEE_A) -> windows_sys::core::PSTR);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeNameW(ptrustee : *const super::accctrl::TRUSTEE_W) -> windows_sys::core::PWSTR);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeTypeA(ptrustee : *const super::accctrl::TRUSTEE_A) -> super::accctrl::TRUSTEE_TYPE);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeTypeW(ptrustee : *const super::accctrl::TRUSTEE_W) -> super::accctrl::TRUSTEE_TYPE);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsA(ppowner : *mut super::accctrl::PTRUSTEE_A, ppgroup : *mut super::accctrl::PTRUSTEE_A, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut super::accctrl::PEXPLICIT_ACCESS_A, pccountofauditentries : *mut u32, pplistofauditentries : *mut super::accctrl::PEXPLICIT_ACCESS_A, psd : super::winnt::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsW(ppowner : *mut super::accctrl::PTRUSTEE_W, ppgroup : *mut super::accctrl::PTRUSTEE_W, pccountofaccessentries : *mut u32, pplistofaccessentries : *mut super::accctrl::PEXPLICIT_ACCESS_W, pccountofauditentries : *mut u32, pplistofauditentries : *mut super::accctrl::PEXPLICIT_ACCESS_W, psd : super::winnt::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetEntriesInAclA(ccountofexplicitentries : u32, plistofexplicitentries : *const super::accctrl::EXPLICIT_ACCESS_A, oldacl : *const super::winnt::ACL, newacl : *mut super::winnt::PACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetEntriesInAclW(ccountofexplicitentries : u32, plistofexplicitentries : *const super::accctrl::EXPLICIT_ACCESS_W, oldacl : *const super::winnt::ACL, newacl : *mut super::winnt::PACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetNamedSecurityInfoA(pobjectname : windows_sys::core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, psidowner : super::winnt::PSID, psidgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetNamedSecurityInfoW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, psidowner : super::winnt::PSID, psidgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetSecurityInfo(handle : super::winnt::HANDLE, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, psidowner : super::winnt::PSID, psidgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoA(pobjectname : windows_sys::core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, powner : super::winnt::PSID, pgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL, keepexplicit : windows_sys::core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : super::accctrl::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, powner : super::winnt::PSID, pgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL, keepexplicit : windows_sys::core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : super::accctrl::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoA(pobjectname : windows_sys::core::PCSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, powner : super::winnt::PSID, pgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL, dwaction : u32, fnprogress : FN_PROGRESS, progressinvokesetting : super::accctrl::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::accctrl::SE_OBJECT_TYPE, securityinfo : super::winnt::SECURITY_INFORMATION, powner : super::winnt::PSID, pgroup : super::winnt::PSID, pdacl : *const super::winnt::ACL, psacl : *const super::winnt::ACL, dwaction : u32, fnprogress : FN_PROGRESS, progressinvokesetting : super::accctrl::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
#[cfg(feature = "accctrl")]
pub type FN_PROGRESS = Option<unsafe extern "system" fn(pobjectname: windows_sys::core::PCWSTR, status: u32, pinvokesetting: *mut super::accctrl::PROG_INVOKE_SETTING, args: *const core::ffi::c_void, securityset: windows_sys::core::BOOL)>;
