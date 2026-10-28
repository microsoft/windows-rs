#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor: super::PSECURITY_DESCRIPTOR, requestedstringsdrevision: super::DWORD, securityinformation: super::SECURITY_INFORMATION, stringsecuritydescriptor: *mut windows_core::PSTR, stringsecuritydescriptorlen: Option<super::PULONG>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor : super::PSECURITY_DESCRIPTOR, requestedstringsdrevision : super::DWORD, securityinformation : super::SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_core::PSTR, stringsecuritydescriptorlen : super::PULONG) -> windows_core::BOOL);
    unsafe { ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor, requestedstringsdrevision, securityinformation, stringsecuritydescriptor as _, stringsecuritydescriptorlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor: super::PSECURITY_DESCRIPTOR, requestedstringsdrevision: super::DWORD, securityinformation: super::SECURITY_INFORMATION, stringsecuritydescriptor: *mut windows_core::PWSTR, stringsecuritydescriptorlen: Option<super::PULONG>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor : super::PSECURITY_DESCRIPTOR, requestedstringsdrevision : super::DWORD, securityinformation : super::SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_core::PWSTR, stringsecuritydescriptorlen : super::PULONG) -> windows_core::BOOL);
    unsafe { ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor, requestedstringsdrevision, securityinformation, stringsecuritydescriptor as _, stringsecuritydescriptorlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertSidToStringSidA(sid: super::PSID, stringsid: *mut windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSidToStringSidA(sid : super::PSID, stringsid : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { ConvertSidToStringSidA(sid, stringsid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertSidToStringSidW(sid: super::PSID, stringsid: *mut windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSidToStringSidW(sid : super::PSID, stringsid : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { ConvertSidToStringSidW(sid, stringsid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorA<P0>(stringsecuritydescriptor: P0, stringsdrevision: super::DWORD, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: Option<super::PULONG>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor : windows_core::PCSTR, stringsdrevision : super::DWORD, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize : super::PULONG) -> windows_core::BOOL);
    unsafe { ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor.param().abi(), stringsdrevision, securitydescriptor as _, securitydescriptorsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorW<P0>(stringsecuritydescriptor: P0, stringsdrevision: super::DWORD, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: Option<super::PULONG>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor : windows_core::PCWSTR, stringsdrevision : super::DWORD, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize : super::PULONG) -> windows_core::BOOL);
    unsafe { ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor.param().abi(), stringsdrevision, securitydescriptor as _, securitydescriptorsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertStringSidToSidA<P0>(stringsid: P0, sid: *mut super::PSID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSidToSidA(stringsid : windows_core::PCSTR, sid : *mut super::PSID) -> windows_core::BOOL);
    unsafe { ConvertStringSidToSidA(stringsid.param().abi(), sid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertStringSidToSidW<P0>(stringsid: P0, sid: *mut super::PSID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSidToSidW(stringsid : windows_core::PCWSTR, sid : *mut super::PSID) -> windows_core::BOOL);
    unsafe { ConvertStringSidToSidW(stringsid.param().abi(), sid as _) }
}
pub const SDDL_ACCESS_ALLOWED: windows_core::PCSTR = windows_core::s!("A");
pub const SDDL_ACCESS_CONTROL_ASSISTANCE_OPS: windows_core::PCSTR = windows_core::s!("AA");
pub const SDDL_ACCESS_DENIED: windows_core::PCSTR = windows_core::s!("D");
pub const SDDL_ACCESS_FILTER: windows_core::PCSTR = windows_core::s!("FL");
pub const SDDL_ACCOUNT_OPERATORS: windows_core::PCSTR = windows_core::s!("AO");
pub const SDDL_ACE_BEGIN: windows_core::PCSTR = windows_core::s!("(");
pub const SDDL_ACE_BEGINC: i8 = 40;
pub const SDDL_ACE_COND_ATTRIBUTE_PREFIX: windows_core::PCSTR = windows_core::s!("@");
pub const SDDL_ACE_COND_BEGIN: windows_core::PCSTR = windows_core::s!("(");
pub const SDDL_ACE_COND_BEGINC: i8 = 40;
pub const SDDL_ACE_COND_BLOB_PREFIX: windows_core::PCSTR = windows_core::s!("#");
pub const SDDL_ACE_COND_BLOB_PREFIXC: i8 = 35;
pub const SDDL_ACE_COND_COMPOSITEVALUE_BEGINC: i8 = 123;
pub const SDDL_ACE_COND_COMPOSITEVALUE_ENDC: i8 = 125;
pub const SDDL_ACE_COND_COMPOSITEVALUE_SEPERATORC: i8 = 44;
pub const SDDL_ACE_COND_DEVICE_ATTRIBUTE_PREFIX: windows_core::PCSTR = windows_core::s!("@DEVICE.");
pub const SDDL_ACE_COND_END: windows_core::PCSTR = windows_core::s!(")");
pub const SDDL_ACE_COND_ENDC: i8 = 41;
pub const SDDL_ACE_COND_RESOURCE_ATTRIBUTE_PREFIX: windows_core::PCSTR = windows_core::s!("@RESOURCE.");
pub const SDDL_ACE_COND_SID_BEGINC: i8 = 40;
pub const SDDL_ACE_COND_SID_ENDC: i8 = 41;
pub const SDDL_ACE_COND_SID_PREFIX: windows_core::PCSTR = windows_core::s!("SID");
pub const SDDL_ACE_COND_STRING_BEGINC: i8 = 34;
pub const SDDL_ACE_COND_STRING_ENDC: i8 = 34;
pub const SDDL_ACE_COND_TOKEN_ATTRIBUTE_PREFIX: windows_core::PCSTR = windows_core::s!("@TOKEN.");
pub const SDDL_ACE_COND_USER_ATTRIBUTE_PREFIX: windows_core::PCSTR = windows_core::s!("@USER.");
pub const SDDL_ACE_END: windows_core::PCSTR = windows_core::s!(")");
pub const SDDL_ACE_ENDC: i8 = 41;
pub const SDDL_ALARM: windows_core::PCSTR = windows_core::s!("AL");
pub const SDDL_ALIAS_PREW2KCOMPACC: windows_core::PCSTR = windows_core::s!("RU");
pub const SDDL_ALIAS_SIZE: i32 = 2;
pub const SDDL_ALL_APP_PACKAGES: windows_core::PCSTR = windows_core::s!("AC");
pub const SDDL_ANONYMOUS: windows_core::PCSTR = windows_core::s!("AN");
pub const SDDL_AUDIT: windows_core::PCSTR = windows_core::s!("AU");
pub const SDDL_AUDIT_FAILURE: windows_core::PCSTR = windows_core::s!("FA");
pub const SDDL_AUDIT_SUCCESS: windows_core::PCSTR = windows_core::s!("SA");
pub const SDDL_AUTHENTICATED_USERS: windows_core::PCSTR = windows_core::s!("AU");
pub const SDDL_AUTHORITY_ASSERTED: windows_core::PCSTR = windows_core::s!("AS");
pub const SDDL_AUTO_INHERITED: windows_core::PCSTR = windows_core::s!("AI");
pub const SDDL_AUTO_INHERIT_REQ: windows_core::PCSTR = windows_core::s!("AR");
pub const SDDL_BACKUP_OPERATORS: windows_core::PCSTR = windows_core::s!("BO");
pub const SDDL_BLOB: windows_core::PCSTR = windows_core::s!("TX");
pub const SDDL_BOOLEAN: windows_core::PCSTR = windows_core::s!("TB");
pub const SDDL_BUILTIN_ADMINISTRATORS: windows_core::PCSTR = windows_core::s!("BA");
pub const SDDL_BUILTIN_GUESTS: windows_core::PCSTR = windows_core::s!("BG");
pub const SDDL_BUILTIN_USERS: windows_core::PCSTR = windows_core::s!("BU");
pub const SDDL_CALLBACK_ACCESS_ALLOWED: windows_core::PCSTR = windows_core::s!("XA");
pub const SDDL_CALLBACK_ACCESS_DENIED: windows_core::PCSTR = windows_core::s!("XD");
pub const SDDL_CALLBACK_AUDIT: windows_core::PCSTR = windows_core::s!("XU");
pub const SDDL_CALLBACK_OBJECT_ACCESS_ALLOWED: windows_core::PCSTR = windows_core::s!("ZA");
pub const SDDL_CERTSVC_DCOM_ACCESS: windows_core::PCSTR = windows_core::s!("CD");
pub const SDDL_CERT_SERV_ADMINISTRATORS: windows_core::PCSTR = windows_core::s!("CA");
pub const SDDL_CLONEABLE_CONTROLLERS: windows_core::PCSTR = windows_core::s!("CN");
pub const SDDL_CONTAINER_INHERIT: windows_core::PCSTR = windows_core::s!("CI");
pub const SDDL_CONTROL_ACCESS: windows_core::PCSTR = windows_core::s!("CR");
pub const SDDL_CREATE_CHILD: windows_core::PCSTR = windows_core::s!("CC");
pub const SDDL_CREATOR_GROUP: windows_core::PCSTR = windows_core::s!("CG");
pub const SDDL_CREATOR_OWNER: windows_core::PCSTR = windows_core::s!("CO");
pub const SDDL_CRITICAL: windows_core::PCSTR = windows_core::s!("CR");
pub const SDDL_CRYPTO_OPERATORS: windows_core::PCSTR = windows_core::s!("CY");
pub const SDDL_CUA_USERS: windows_core::PCSTR = windows_core::s!("CU");
pub const SDDL_DACL: windows_core::PCSTR = windows_core::s!("D");
pub const SDDL_DELETE_CHILD: windows_core::PCSTR = windows_core::s!("DC");
pub const SDDL_DELETE_TREE: windows_core::PCSTR = windows_core::s!("DT");
pub const SDDL_DELIMINATOR: windows_core::PCSTR = windows_core::s!(":");
pub const SDDL_DELIMINATORC: i8 = 58;
pub const SDDL_DOMAIN_ADMINISTRATORS: windows_core::PCSTR = windows_core::s!("DA");
pub const SDDL_DOMAIN_COMPUTERS: windows_core::PCSTR = windows_core::s!("DC");
pub const SDDL_DOMAIN_DOMAIN_CONTROLLERS: windows_core::PCSTR = windows_core::s!("DD");
pub const SDDL_DOMAIN_GUESTS: windows_core::PCSTR = windows_core::s!("DG");
pub const SDDL_DOMAIN_USERS: windows_core::PCSTR = windows_core::s!("DU");
pub const SDDL_ENTERPRISE_ADMINS: windows_core::PCSTR = windows_core::s!("EA");
pub const SDDL_ENTERPRISE_DOMAIN_CONTROLLERS: windows_core::PCSTR = windows_core::s!("ED");
pub const SDDL_ENTERPRISE_KEY_ADMINS: windows_core::PCSTR = windows_core::s!("EK");
pub const SDDL_ENTERPRISE_RO_DCs: windows_core::PCSTR = windows_core::s!("RO");
pub const SDDL_EVENT_LOG_READERS: windows_core::PCSTR = windows_core::s!("ER");
pub const SDDL_EVERYONE: windows_core::PCSTR = windows_core::s!("WD");
pub const SDDL_FILE_ALL: windows_core::PCSTR = windows_core::s!("FA");
pub const SDDL_FILE_EXECUTE: windows_core::PCSTR = windows_core::s!("FX");
pub const SDDL_FILE_READ: windows_core::PCSTR = windows_core::s!("FR");
pub const SDDL_FILE_WRITE: windows_core::PCSTR = windows_core::s!("FW");
pub const SDDL_GENERIC_ALL: windows_core::PCSTR = windows_core::s!("GA");
pub const SDDL_GENERIC_EXECUTE: windows_core::PCSTR = windows_core::s!("GX");
pub const SDDL_GENERIC_READ: windows_core::PCSTR = windows_core::s!("GR");
pub const SDDL_GENERIC_WRITE: windows_core::PCSTR = windows_core::s!("GW");
pub const SDDL_GROUP: windows_core::PCSTR = windows_core::s!("G");
pub const SDDL_GROUP_POLICY_ADMINS: windows_core::PCSTR = windows_core::s!("PA");
pub const SDDL_HYPER_V_ADMINS: windows_core::PCSTR = windows_core::s!("HA");
pub const SDDL_IIS_USERS: windows_core::PCSTR = windows_core::s!("IS");
pub const SDDL_INHERITED: windows_core::PCSTR = windows_core::s!("ID");
pub const SDDL_INHERIT_ONLY: windows_core::PCSTR = windows_core::s!("IO");
pub const SDDL_INT: windows_core::PCSTR = windows_core::s!("TI");
pub const SDDL_INTERACTIVE: windows_core::PCSTR = windows_core::s!("IU");
pub const SDDL_KEY_ADMINS: windows_core::PCSTR = windows_core::s!("KA");
pub const SDDL_KEY_ALL: windows_core::PCSTR = windows_core::s!("KA");
pub const SDDL_KEY_EXECUTE: windows_core::PCSTR = windows_core::s!("KX");
pub const SDDL_KEY_READ: windows_core::PCSTR = windows_core::s!("KR");
pub const SDDL_KEY_WRITE: windows_core::PCSTR = windows_core::s!("KW");
pub const SDDL_LIST_CHILDREN: windows_core::PCSTR = windows_core::s!("LC");
pub const SDDL_LIST_OBJECT: windows_core::PCSTR = windows_core::s!("LO");
pub const SDDL_LOCAL_ADMIN: windows_core::PCSTR = windows_core::s!("LA");
pub const SDDL_LOCAL_GUEST: windows_core::PCSTR = windows_core::s!("LG");
pub const SDDL_LOCAL_SERVICE: windows_core::PCSTR = windows_core::s!("LS");
pub const SDDL_LOCAL_SYSTEM: windows_core::PCSTR = windows_core::s!("SY");
pub const SDDL_MANDATORY_LABEL: windows_core::PCSTR = windows_core::s!("ML");
pub const SDDL_ML_HIGH: windows_core::PCSTR = windows_core::s!("HI");
pub const SDDL_ML_LOW: windows_core::PCSTR = windows_core::s!("LW");
pub const SDDL_ML_MEDIUM: windows_core::PCSTR = windows_core::s!("ME");
pub const SDDL_ML_MEDIUM_PLUS: windows_core::PCSTR = windows_core::s!("MP");
pub const SDDL_ML_SYSTEM: windows_core::PCSTR = windows_core::s!("SI");
pub const SDDL_NETWORK: windows_core::PCSTR = windows_core::s!("NU");
pub const SDDL_NETWORK_CONFIGURATION_OPS: windows_core::PCSTR = windows_core::s!("NO");
pub const SDDL_NETWORK_SERVICE: windows_core::PCSTR = windows_core::s!("NS");
pub const SDDL_NO_EXECUTE_UP: windows_core::PCSTR = windows_core::s!("NX");
pub const SDDL_NO_PROPAGATE: windows_core::PCSTR = windows_core::s!("NP");
pub const SDDL_NO_READ_UP: windows_core::PCSTR = windows_core::s!("NR");
pub const SDDL_NO_WRITE_UP: windows_core::PCSTR = windows_core::s!("NW");
pub const SDDL_NULL_ACL: windows_core::PCSTR = windows_core::s!("NO_ACCESS_CONTROL");
pub const SDDL_OBJECT_ACCESS_ALLOWED: windows_core::PCSTR = windows_core::s!("OA");
pub const SDDL_OBJECT_ACCESS_DENIED: windows_core::PCSTR = windows_core::s!("OD");
pub const SDDL_OBJECT_ALARM: windows_core::PCSTR = windows_core::s!("OL");
pub const SDDL_OBJECT_AUDIT: windows_core::PCSTR = windows_core::s!("OU");
pub const SDDL_OBJECT_INHERIT: windows_core::PCSTR = windows_core::s!("OI");
pub const SDDL_OPENSSH_USERS: windows_core::PCSTR = windows_core::s!("SH");
pub const SDDL_OWNER: windows_core::PCSTR = windows_core::s!("O");
pub const SDDL_OWNER_RIGHTS: windows_core::PCSTR = windows_core::s!("OW");
pub const SDDL_PERFLOG_USERS: windows_core::PCSTR = windows_core::s!("LU");
pub const SDDL_PERFMON_USERS: windows_core::PCSTR = windows_core::s!("MU");
pub const SDDL_PERSONAL_SELF: windows_core::PCSTR = windows_core::s!("PS");
pub const SDDL_POWER_USERS: windows_core::PCSTR = windows_core::s!("PU");
pub const SDDL_PRINTER_OPERATORS: windows_core::PCSTR = windows_core::s!("PO");
pub const SDDL_PROCESS_TRUST_LABEL: windows_core::PCSTR = windows_core::s!("TL");
pub const SDDL_PROTECTED: windows_core::PCSTR = windows_core::s!("P");
pub const SDDL_PROTECTED_USERS: windows_core::PCSTR = windows_core::s!("AP");
pub const SDDL_RAS_SERVERS: windows_core::PCSTR = windows_core::s!("RS");
pub const SDDL_RDS_ENDPOINT_SERVERS: windows_core::PCSTR = windows_core::s!("ES");
pub const SDDL_RDS_MANAGEMENT_SERVERS: windows_core::PCSTR = windows_core::s!("MS");
pub const SDDL_RDS_REMOTE_ACCESS_SERVERS: windows_core::PCSTR = windows_core::s!("RA");
pub const SDDL_READ_CONTROL: windows_core::PCSTR = windows_core::s!("RC");
pub const SDDL_READ_PROPERTY: windows_core::PCSTR = windows_core::s!("RP");
pub const SDDL_REMOTE_DESKTOP: windows_core::PCSTR = windows_core::s!("RD");
pub const SDDL_REMOTE_MANAGEMENT_USERS: windows_core::PCSTR = windows_core::s!("RM");
pub const SDDL_REPLICATOR: windows_core::PCSTR = windows_core::s!("RE");
pub const SDDL_RESOURCE_ATTRIBUTE: windows_core::PCSTR = windows_core::s!("RA");
pub const SDDL_RESTRICTED_CODE: windows_core::PCSTR = windows_core::s!("RC");
pub const SDDL_REVISION: i32 = 1;
pub const SDDL_REVISION_1: i32 = 1;
pub const SDDL_SACL: windows_core::PCSTR = windows_core::s!("S");
pub const SDDL_SCHEMA_ADMINISTRATORS: windows_core::PCSTR = windows_core::s!("SA");
pub const SDDL_SCOPED_POLICY_ID: windows_core::PCSTR = windows_core::s!("SP");
pub const SDDL_SELF_WRITE: windows_core::PCSTR = windows_core::s!("SW");
pub const SDDL_SEPERATOR: windows_core::PCSTR = windows_core::s!(";");
pub const SDDL_SEPERATORC: i8 = 59;
pub const SDDL_SERVER_OPERATORS: windows_core::PCSTR = windows_core::s!("SO");
pub const SDDL_SERVICE: windows_core::PCSTR = windows_core::s!("SU");
pub const SDDL_SERVICE_ASSERTED: windows_core::PCSTR = windows_core::s!("SS");
pub const SDDL_SID: windows_core::PCSTR = windows_core::s!("TD");
pub const SDDL_SPACE: windows_core::PCSTR = windows_core::s!(" ");
pub const SDDL_SPACEC: i8 = 32;
pub const SDDL_STANDARD_DELETE: windows_core::PCSTR = windows_core::s!("SD");
pub const SDDL_TRUST_PROTECTED_FILTER: windows_core::PCSTR = windows_core::s!("TP");
pub const SDDL_UINT: windows_core::PCSTR = windows_core::s!("TU");
pub const SDDL_USER_MODE_DRIVERS: windows_core::PCSTR = windows_core::s!("UD");
pub const SDDL_USER_MODE_HARDWARE_OPERATORS: windows_core::PCSTR = windows_core::s!("HO");
pub const SDDL_WRITE_DAC: windows_core::PCSTR = windows_core::s!("WD");
pub const SDDL_WRITE_OWNER: windows_core::PCSTR = windows_core::s!("WO");
pub const SDDL_WRITE_PROPERTY: windows_core::PCSTR = windows_core::s!("WP");
pub const SDDL_WRITE_RESTRICTED_CODE: windows_core::PCSTR = windows_core::s!("WR");
pub const SDDL_WSTRING: windows_core::PCSTR = windows_core::s!("TS");
