#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameA(pexplicitaccess : super::PEXPLICIT_ACCESS_A, ptrusteename : windows_sys::core::PCSTR, accesspermissions : u32, accessmode : super::ACCESS_MODE, inheritance : u32));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildExplicitAccessWithNameW(pexplicitaccess : super::PEXPLICIT_ACCESS_W, ptrusteename : windows_sys::core::PCWSTR, accesspermissions : u32, accessmode : super::ACCESS_MODE, inheritance : u32));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess : super::PEXPLICIT_ACCESS_A, ptrusteename : windows_sys::core::PCSTR, ptrustee : super::PTRUSTEE_A, accesspermissions : u32, accessmode : super::ACCESS_MODE, inheritance : u32));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess : super::PEXPLICIT_ACCESS_W, ptrusteename : windows_sys::core::PCWSTR, ptrustee : super::PTRUSTEE_W, accesspermissions : u32, accessmode : super::ACCESS_MODE, inheritance : u32));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeA(ptrustee : super::PTRUSTEE_A, pimpersonatetrustee : super::PTRUSTEE_A));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildImpersonateTrusteeW(ptrustee : super::PTRUSTEE_W, pimpersonatetrustee : super::PTRUSTEE_W));
#[cfg(all(feature = "accctrl", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildSecurityDescriptorA(powner : super::PTRUSTEE_A, pgroup : super::PTRUSTEE_A, ccountofaccessentries : u32, plistofaccessentries : super::PEXPLICIT_ACCESS_A, ccountofauditentries : u32, plistofauditentries : super::PEXPLICIT_ACCESS_A, poldsd : super::PSECURITY_DESCRIPTOR, psizenewsd : super::PULONG, pnewsd : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildSecurityDescriptorW(powner : super::PTRUSTEE_W, pgroup : super::PTRUSTEE_W, ccountofaccessentries : u32, plistofaccessentries : super::PEXPLICIT_ACCESS_W, ccountofauditentries : u32, plistofauditentries : super::PEXPLICIT_ACCESS_W, poldsd : super::PSECURITY_DESCRIPTOR, psizenewsd : super::PULONG, pnewsd : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithNameA(ptrustee : super::PTRUSTEE_A, pname : windows_sys::core::PCSTR));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithNameW(ptrustee : super::PTRUSTEE_W, pname : windows_sys::core::PCWSTR));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameA(ptrustee : super::PTRUSTEE_A, pobjname : super::POBJECTS_AND_NAME_A, objecttype : super::SE_OBJECT_TYPE, objecttypename : windows_sys::core::PCSTR, inheritedobjecttypename : windows_sys::core::PCSTR, name : windows_sys::core::PCSTR));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndNameW(ptrustee : super::PTRUSTEE_W, pobjname : super::POBJECTS_AND_NAME_W, objecttype : super::SE_OBJECT_TYPE, objecttypename : windows_sys::core::PCWSTR, inheritedobjecttypename : windows_sys::core::PCWSTR, name : windows_sys::core::PCWSTR));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidA(ptrustee : super::PTRUSTEE_A, pobjsid : super::POBJECTS_AND_SID, pobjectguid : *const windows_sys::core::GUID, pinheritedobjectguid : *const windows_sys::core::GUID, psid : super::PSID));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithObjectsAndSidW(ptrustee : super::PTRUSTEE_W, pobjsid : super::POBJECTS_AND_SID, pobjectguid : *const windows_sys::core::GUID, pinheritedobjectguid : *const windows_sys::core::GUID, psid : super::PSID));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithSidA(ptrustee : super::PTRUSTEE_A, psid : super::PSID));
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn BuildTrusteeWithSidW(ptrustee : super::PTRUSTEE_W, psid : super::PSID));
#[cfg(feature = "accctrl")]
windows_link::link!("advapi32.dll" "system" fn FreeInheritedFromArray(pinheritarray : super::PINHERITED_FROMW, acecnt : u16, pfnarray : super::PFN_OBJECT_MGR_FUNCTS) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclA(pacl : super::PACL, ptrustee : super::PTRUSTEE_A, psuccessfulauditedrights : super::PACCESS_MASK, pfailedauditrights : super::PACCESS_MASK) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetAuditedPermissionsFromAclW(pacl : super::PACL, ptrustee : super::PTRUSTEE_W, psuccessfulauditedrights : super::PACCESS_MASK, pfailedauditrights : super::PACCESS_MASK) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclA(pacl : super::PACL, ptrustee : super::PTRUSTEE_A, paccessrights : super::PACCESS_MASK) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetEffectiveRightsFromAclW(pacl : super::PACL, ptrustee : super::PTRUSTEE_W, paccessrights : super::PACCESS_MASK) -> u32);
#[cfg(all(feature = "accctrl", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclA(pacl : super::PACL, pccountofexplicitentries : super::PULONG, plistofexplicitentries : *mut super::PEXPLICIT_ACCESS_A) -> u32);
#[cfg(all(feature = "accctrl", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetExplicitEntriesFromAclW(pacl : super::PACL, pccountofexplicitentries : super::PULONG, plistofexplicitentries : *mut super::PEXPLICIT_ACCESS_W) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetInheritanceSourceA(pobjectname : windows_sys::core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, container : windows_sys::core::BOOL, pobjectclassguids : *const *const windows_sys::core::GUID, guidcount : u32, pacl : super::PACL, pfnarray : super::PFN_OBJECT_MGR_FUNCTS, pgenericmapping : super::PGENERIC_MAPPING, pinheritarray : super::PINHERITED_FROMA) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetInheritanceSourceW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, container : windows_sys::core::BOOL, pobjectclassguids : *const *const windows_sys::core::GUID, guidcount : u32, pacl : super::PACL, pfnarray : super::PFN_OBJECT_MGR_FUNCTS, pgenericmapping : super::PGENERIC_MAPPING, pinheritarray : super::PINHERITED_FROMW) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetMultipleTrusteeA(ptrustee : super::PTRUSTEE_A) -> super::PTRUSTEE_A);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationA(ptrustee : super::PTRUSTEE_A) -> super::MULTIPLE_TRUSTEE_OPERATION);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetMultipleTrusteeOperationW(ptrustee : super::PTRUSTEE_W) -> super::MULTIPLE_TRUSTEE_OPERATION);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetMultipleTrusteeW(ptrustee : super::PTRUSTEE_W) -> super::PTRUSTEE_W);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetNamedSecurityInfoA(pobjectname : windows_sys::core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, ppsidowner : *mut super::PSID, ppsidgroup : *mut super::PSID, ppdacl : *mut super::PACL, ppsacl : *mut super::PACL, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetNamedSecurityInfoW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, ppsidowner : *mut super::PSID, ppsidgroup : *mut super::PSID, ppdacl : *mut super::PACL, ppsacl : *mut super::PACL, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetSecurityInfo(handle : super::HANDLE, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, ppsidowner : *mut super::PSID, ppsidgroup : *mut super::PSID, ppdacl : *mut super::PACL, ppsacl : *mut super::PACL, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeFormA(ptrustee : super::PTRUSTEE_A) -> super::TRUSTEE_FORM);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeFormW(ptrustee : super::PTRUSTEE_W) -> super::TRUSTEE_FORM);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeNameA(ptrustee : super::PTRUSTEE_A) -> windows_sys::core::PSTR);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeNameW(ptrustee : super::PTRUSTEE_W) -> windows_sys::core::PWSTR);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeTypeA(ptrustee : super::PTRUSTEE_A) -> super::TRUSTEE_TYPE);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn GetTrusteeTypeW(ptrustee : super::PTRUSTEE_W) -> super::TRUSTEE_TYPE);
#[cfg(all(feature = "accctrl", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsA(ppowner : *mut super::PTRUSTEE_A, ppgroup : *mut super::PTRUSTEE_A, pccountofaccessentries : super::PULONG, pplistofaccessentries : *mut super::PEXPLICIT_ACCESS_A, pccountofauditentries : super::PULONG, pplistofauditentries : *mut super::PEXPLICIT_ACCESS_A, psd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn LookupSecurityDescriptorPartsW(ppowner : *mut super::PTRUSTEE_W, ppgroup : *mut super::PTRUSTEE_W, pccountofaccessentries : super::PULONG, pplistofaccessentries : *mut super::PEXPLICIT_ACCESS_W, pccountofauditentries : super::PULONG, pplistofauditentries : *mut super::PEXPLICIT_ACCESS_W, psd : super::PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetEntriesInAclA(ccountofexplicitentries : u32, plistofexplicitentries : super::PEXPLICIT_ACCESS_A, oldacl : super::PACL, newacl : *mut super::PACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetEntriesInAclW(ccountofexplicitentries : u32, plistofexplicitentries : super::PEXPLICIT_ACCESS_W, oldacl : super::PACL, newacl : *mut super::PACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetNamedSecurityInfoA(pobjectname : windows_sys::core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, psidowner : super::PSID, psidgroup : super::PSID, pdacl : super::PACL, psacl : super::PACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetNamedSecurityInfoW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, psidowner : super::PSID, psidgroup : super::PSID, pdacl : super::PACL, psacl : super::PACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetSecurityInfo(handle : super::HANDLE, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, psidowner : super::PSID, psidgroup : super::PSID, pdacl : super::PACL, psacl : super::PACL) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoA(pobjectname : windows_sys::core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, powner : super::PSID, pgroup : super::PSID, pdacl : super::PACL, psacl : super::PACL, keepexplicit : windows_sys::core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : super::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TreeResetNamedSecurityInfoW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, powner : super::PSID, pgroup : super::PSID, pdacl : super::PACL, psacl : super::PACL, keepexplicit : windows_sys::core::BOOL, fnprogress : FN_PROGRESS, progressinvokesetting : super::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoA(pobjectname : windows_sys::core::PCSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, powner : super::PSID, pgroup : super::PSID, pdacl : super::PACL, psacl : super::PACL, dwaction : u32, fnprogress : FN_PROGRESS, progressinvokesetting : super::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "accctrl", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn TreeSetNamedSecurityInfoW(pobjectname : windows_sys::core::PCWSTR, objecttype : super::SE_OBJECT_TYPE, securityinfo : super::SECURITY_INFORMATION, powner : super::PSID, pgroup : super::PSID, pdacl : super::PACL, psacl : super::PACL, dwaction : u32, fnprogress : FN_PROGRESS, progressinvokesetting : super::PROG_INVOKE_SETTING, args : *const core::ffi::c_void) -> u32);
#[cfg(feature = "accctrl")]
pub type FN_PROGRESS = Option<unsafe extern "system" fn(pobjectname: windows_sys::core::PCWSTR, status: u32, pinvokesetting: super::PPROG_INVOKE_SETTING, args: *const core::ffi::c_void, securityset: windows_sys::core::BOOL)>;
