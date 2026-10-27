#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzAccessCheck(flags : super::DWORD, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : PAUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray : *const super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount : super::DWORD, preply : PAUTHZ_ACCESS_REPLY, phaccesscheckresults : PAUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzAddSidsToContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, sids : super::PSID_AND_ATTRIBUTES, sidcount : super::DWORD, restrictedsids : super::PSID_AND_ATTRIBUTES, restrictedsidcount : super::DWORD, phnewauthzclientcontext : PAUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzCachedAccessCheck(flags : super::DWORD, haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, prequest : PAUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, preply : PAUTHZ_ACCESS_REPLY) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzEnumerateSecurityEventSources(dwflags : super::DWORD, buffer : PAUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount : super::PDWORD, pdwlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzEvaluateSacl(authzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : PAUTHZ_ACCESS_REQUEST, sacl : super::PACL, grantedaccess : super::ACCESS_MASK, accessgranted : windows_sys::core::BOOL, pbgenerateaudit : super::PBOOL) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeAuditEvent(hauditevent : AUTHZ_AUDIT_EVENT_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeCentralAccessPolicyCache() -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeHandle(haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeResourceManager(hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzGetInformationFromContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, infoclass : AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize : super::DWORD, psizerequired : super::PDWORD, buffer : super::PVOID) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzInitializeCompoundContext(usercontext : AUTHZ_CLIENT_CONTEXT_HANDLE, devicecontext : AUTHZ_CLIENT_CONTEXT_HANDLE, phcompoundcontext : PAUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzInitializeContextFromAuthzContext(flags : super::DWORD, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, pexpirationtime : super::PLARGE_INTEGER, identifier : super::LUID, dynamicgroupargs : super::PVOID, phnewauthzclientcontext : PAUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzInitializeContextFromSid(flags : super::DWORD, usersid : super::PSID, hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime : super::PLARGE_INTEGER, identifier : super::LUID, dynamicgroupargs : super::PVOID, phauthzclientcontext : PAUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzInitializeContextFromToken(flags : super::DWORD, tokenhandle : super::HANDLE, hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime : super::PLARGE_INTEGER, identifier : super::LUID, dynamicgroupargs : super::PVOID, phauthzclientcontext : PAUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("authz.dll" "C" fn AuthzInitializeObjectAccessAuditEvent(flags : super::DWORD, hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype : windows_sys::core::PCWSTR, szobjecttype : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szadditionalinfo : windows_sys::core::PCWSTR, phauditevent : PAUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount : super::DWORD, ...) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("authz.dll" "C" fn AuthzInitializeObjectAccessAuditEvent2(flags : super::DWORD, hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype : windows_sys::core::PCWSTR, szobjecttype : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szadditionalinfo : windows_sys::core::PCWSTR, szadditionalinfo2 : windows_sys::core::PCWSTR, phauditevent : PAUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount : super::DWORD, ...) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("authz.dll" "system" fn AuthzInitializeRemoteResourceManager(prpcinitinfo : PAUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager : PAUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzInitializeResourceManager(flags : super::DWORD, pfndynamicaccesscheck : PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups : PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups : PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername : windows_sys::core::PCWSTR, phauthzresourcemanager : PAUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzInitializeResourceManagerEx(flags : super::DWORD, pauthzinitinfo : PAUTHZ_INIT_INFO, phauthzresourcemanager : PAUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzInstallSecurityEventSource(dwflags : super::DWORD, pregistration : PAUTHZ_SOURCE_SCHEMA_REGISTRATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzModifyClaims(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, claimclass : AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations : PAUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims : PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzModifySecurityAttributes(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, poperations : PAUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes : PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzModifySids(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, sidclass : AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations : PAUTHZ_SID_OPERATION, psids : super::PTOKEN_GROUPS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzOpenObjectAudit(flags : super::DWORD, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : PAUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray : *const super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount : super::DWORD, preply : PAUTHZ_ACCESS_REPLY) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzRegisterCapChangeNotification(phcapchangesubscription : PAUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE, pfncapchangecallback : super::LPTHREAD_START_ROUTINE, pcallbackcontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("authz.dll" "system" fn AuthzRegisterSecurityEventSource(dwflags : super::DWORD, szeventsourcename : windows_sys::core::PCWSTR, pheventprovider : PAUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "C" fn AuthzReportSecurityEvent(dwflags : super::DWORD, heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : super::DWORD, pusersid : super::PSID, dwcount : super::DWORD, ...) -> windows_sys::core::BOOL);
#[cfg(all(feature = "adtgen", feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzReportSecurityEventFromParams(dwflags : super::DWORD, heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : super::DWORD, pusersid : super::PSID, pparams : super::PAUDIT_PARAMS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzSetAppContainerInformation(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, pappcontainersid : super::PSID, capabilitycount : super::DWORD, pcapabilitysids : super::PSID_AND_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("authz.dll" "system" fn AuthzUninstallSecurityEventSource(dwflags : super::DWORD, szeventsourcename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzUnregisterCapChangeNotification(hcapchangesubscription : AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("authz.dll" "system" fn AuthzUnregisterSecurityEventSource(dwflags : super::DWORD, pheventprovider : PAUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_sys::core::BOOL);
pub const AUTHZ_ACCESS_CHECK_NO_DEEP_COPY_SD: i32 = 1;
pub type AUTHZ_ACCESS_CHECK_RESULTS_HANDLE = *mut AUTHZ_ACCESS_CHECK_RESULTS_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_ACCESS_CHECK_RESULTS_HANDLE__ {
    pub unused: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_ACCESS_REPLY {
    pub ResultListLength: super::DWORD,
    pub GrantedAccessMask: super::PACCESS_MASK,
    pub SaclEvaluationResults: super::PDWORD,
    pub Error: super::PDWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_ACCESS_REQUEST {
    pub DesiredAccess: super::ACCESS_MASK,
    pub PrincipalSelfSid: super::PSID,
    pub ObjectTypeList: super::POBJECT_TYPE_LIST,
    pub ObjectTypeListLength: super::DWORD,
    pub OptionalArguments: super::PVOID,
}
pub type AUTHZ_AUDIT_EVENT_HANDLE = *mut AUTHZ_AUDIT_EVENT_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_AUDIT_EVENT_HANDLE__ {
    pub unused: i32,
}
pub type AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = i32;
pub type AUTHZ_AUDIT_EVENT_TYPE_HANDLE = *mut AUTHZ_AUDIT_EVENT_TYPE_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_HANDLE__ {
    pub unused: i32,
}
pub type AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE = *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    pub unused: i32,
}
pub type AUTHZ_CLIENT_CONTEXT_HANDLE = *mut AUTHZ_CLIENT_CONTEXT_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_CLIENT_CONTEXT_HANDLE__ {
    pub unused: i32,
}
pub const AUTHZ_COMPUTE_PRIVILEGES: i32 = 8;
pub type AUTHZ_CONTEXT_INFORMATION_CLASS = i32;
pub const AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES: i32 = 1;
pub const AUTHZ_GENERATE_FAILURE_AUDIT: i32 = 2;
pub const AUTHZ_GENERATE_SUCCESS_AUDIT: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_INIT_INFO {
    pub version: super::USHORT,
    pub szResourceManagerName: windows_sys::core::PCWSTR,
    pub pfnDynamicAccessCheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK,
    pub pfnComputeDynamicGroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS,
    pub pfnFreeDynamicGroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS,
    pub pfnGetCentralAccessPolicy: PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY,
    pub pfnFreeCentralAccessPolicy: PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY,
}
pub const AUTHZ_INIT_INFO_VERSION_V1: i32 = 1;
pub const AUTHZ_NO_ALLOC_STRINGS: i32 = 4;
pub const AUTHZ_NO_FAILURE_AUDIT: i32 = 2;
pub const AUTHZ_NO_SUCCESS_AUDIT: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    pub szObjectTypeName: windows_sys::core::PWSTR,
    pub dwOffset: super::DWORD,
}
pub const AUTHZ_REQUIRE_S4U_LOGON: i32 = 4;
pub type AUTHZ_RESOURCE_MANAGER_HANDLE = *mut AUTHZ_RESOURCE_MANAGER_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_RESOURCE_MANAGER_HANDLE__ {
    pub unused: i32,
}
pub const AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION: i32 = 2;
pub const AUTHZ_RM_FLAG_NO_AUDIT: i32 = 1;
pub const AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_RPC_INIT_INFO_CLIENT {
    pub version: super::USHORT,
    pub ObjectUuid: windows_sys::core::PWSTR,
    pub ProtSeq: windows_sys::core::PWSTR,
    pub NetworkAddr: windows_sys::core::PWSTR,
    pub Endpoint: windows_sys::core::PWSTR,
    pub Options: windows_sys::core::PWSTR,
    pub ServerSpn: windows_sys::core::PWSTR,
}
pub const AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: super::USHORT,
    pub Reserved: super::USHORT,
    pub AttributeCount: super::ULONG,
    pub Attribute: AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0,
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
impl Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: PAUTHZ_SECURITY_ATTRIBUTE_V1,
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
impl Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION: i32 = 1;
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: super::ULONG64,
    pub pName: windows_sys::core::PWSTR,
}
pub const AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: super::PVOID,
    pub ValueLength: super::ULONG,
}
pub type AUTHZ_SECURITY_ATTRIBUTE_OPERATION = i32;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_ADD: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 2;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_DELETE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 3;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_NONE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 0;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 4;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 1;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: i32 = 6;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN: i32 = 4;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INT64: i32 = 1;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INVALID: i32 = 0;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: i32 = 16;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_SID: i32 = 5;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING: i32 = 3;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_UINT64: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_V1 {
    pub pName: windows_sys::core::PWSTR,
    pub ValueType: super::USHORT,
    pub Reserved: super::USHORT,
    pub Flags: super::ULONG,
    pub ValueCount: super::ULONG,
    pub Values: AUTHZ_SECURITY_ATTRIBUTE_V1_0,
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
impl Default for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: super::PLONG64,
    pub pUint64: super::PULONG64,
    pub ppString: *mut windows_sys::core::PWSTR,
    pub pFqbn: PAUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: PAUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
impl Default for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTE_VALID_FLAGS: i32 = 3;
pub const AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: i32 = 2;
pub type AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE = *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE__ {
    pub unused: i32,
}
pub type AUTHZ_SID_OPERATION = i32;
pub const AUTHZ_SID_OPERATION_ADD: AUTHZ_SID_OPERATION = 2;
pub const AUTHZ_SID_OPERATION_DELETE: AUTHZ_SID_OPERATION = 3;
pub const AUTHZ_SID_OPERATION_NONE: AUTHZ_SID_OPERATION = 0;
pub const AUTHZ_SID_OPERATION_REPLACE: AUTHZ_SID_OPERATION = 4;
pub const AUTHZ_SID_OPERATION_REPLACE_ALL: AUTHZ_SID_OPERATION = 1;
pub const AUTHZ_SKIP_TOKEN_GROUPS: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    pub dwFlags: super::DWORD,
    pub szEventSourceName: windows_sys::core::PWSTR,
    pub szEventMessageFile: windows_sys::core::PWSTR,
    pub szEventSourceXmlSchemaFile: windows_sys::core::PWSTR,
    pub szEventAccessStringsFile: windows_sys::core::PWSTR,
    pub szExecutableImagePath: windows_sys::core::PWSTR,
    pub Anonymous: AUTHZ_SOURCE_SCHEMA_REGISTRATION_0,
    pub dwObjectTypeNameCount: super::DWORD,
    pub ObjectTypeNames: [AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    pub pReserved: super::PVOID,
    pub pProviderGuid: *mut windows_sys::core::GUID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_VALID_OBJECT_ACCESS_AUDIT_FLAGS: i32 = 23;
pub const AUTHZ_VALID_RM_INIT_FLAGS: i32 = 7;
pub const AUTHZ_WPD_CATEGORY_FLAG: i32 = 16;
pub const AuthzAuditEventInfoAdditionalInfo: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 5;
pub const AuthzAuditEventInfoFlags: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 1;
pub const AuthzAuditEventInfoObjectName: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 4;
pub const AuthzAuditEventInfoObjectType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 3;
pub const AuthzAuditEventInfoOperationType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = 2;
pub const AuthzContextInfoAll: AUTHZ_CONTEXT_INFORMATION_CLASS = 9;
pub const AuthzContextInfoAppContainerSid: AUTHZ_CONTEXT_INFORMATION_CLASS = 15;
pub const AuthzContextInfoAuthenticationId: AUTHZ_CONTEXT_INFORMATION_CLASS = 10;
pub const AuthzContextInfoCapabilitySids: AUTHZ_CONTEXT_INFORMATION_CLASS = 16;
pub const AuthzContextInfoDeviceClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = 14;
pub const AuthzContextInfoDeviceSids: AUTHZ_CONTEXT_INFORMATION_CLASS = 12;
pub const AuthzContextInfoExpirationTime: AUTHZ_CONTEXT_INFORMATION_CLASS = 5;
pub const AuthzContextInfoGroupsSids: AUTHZ_CONTEXT_INFORMATION_CLASS = 2;
pub const AuthzContextInfoIdentifier: AUTHZ_CONTEXT_INFORMATION_CLASS = 7;
pub const AuthzContextInfoPrivileges: AUTHZ_CONTEXT_INFORMATION_CLASS = 4;
pub const AuthzContextInfoRestrictedSids: AUTHZ_CONTEXT_INFORMATION_CLASS = 3;
pub const AuthzContextInfoSecurityAttributes: AUTHZ_CONTEXT_INFORMATION_CLASS = 11;
pub const AuthzContextInfoServerContext: AUTHZ_CONTEXT_INFORMATION_CLASS = 6;
pub const AuthzContextInfoSource: AUTHZ_CONTEXT_INFORMATION_CLASS = 8;
pub const AuthzContextInfoUserClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = 13;
pub const AuthzContextInfoUserSid: AUTHZ_CONTEXT_INFORMATION_CLASS = 1;
pub type PAUTHZ_ACCESS_CHECK_RESULTS_HANDLE = *mut *mut AUTHZ_ACCESS_CHECK_RESULTS_HANDLE__;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PAUTHZ_ACCESS_REPLY = *mut AUTHZ_ACCESS_REPLY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAUTHZ_ACCESS_REQUEST = *mut AUTHZ_ACCESS_REQUEST;
pub type PAUTHZ_AUDIT_EVENT_HANDLE = *mut *mut AUTHZ_AUDIT_EVENT_HANDLE__;
pub type PAUTHZ_AUDIT_EVENT_TYPE_HANDLE = *mut *mut AUTHZ_AUDIT_EVENT_TYPE_HANDLE__;
pub type PAUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE = *mut *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__;
pub type PAUTHZ_CLIENT_CONTEXT_HANDLE = *mut *mut AUTHZ_CLIENT_CONTEXT_HANDLE__;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PAUTHZ_INIT_INFO = *mut AUTHZ_INIT_INFO;
#[cfg(feature = "shared")]
pub type PAUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET = *mut AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET;
pub type PAUTHZ_RESOURCE_MANAGER_HANDLE = *mut *mut AUTHZ_RESOURCE_MANAGER_HANDLE__;
#[cfg(feature = "shared")]
pub type PAUTHZ_RPC_INIT_INFO_CLIENT = *mut AUTHZ_RPC_INIT_INFO_CLIENT;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION = *mut AUTHZ_SECURITY_ATTRIBUTES_INFORMATION;
#[cfg(feature = "shared")]
pub type PAUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE = *mut AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE = *mut AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE;
pub type PAUTHZ_SECURITY_ATTRIBUTE_OPERATION = *mut AUTHZ_SECURITY_ATTRIBUTE_OPERATION;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PAUTHZ_SECURITY_ATTRIBUTE_V1 = *mut AUTHZ_SECURITY_ATTRIBUTE_V1;
pub type PAUTHZ_SECURITY_EVENT_PROVIDER_HANDLE = *mut *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE__;
pub type PAUTHZ_SID_OPERATION = *mut AUTHZ_SID_OPERATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAUTHZ_SOURCE_SCHEMA_REGISTRATION = *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, args: super::PVOID, psidattrarray: *mut super::PSID_AND_ATTRIBUTES, psidcount: super::PDWORD, prestrictedsidattrarray: *mut super::PSID_AND_ATTRIBUTES, prestrictedsidcount: super::PDWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFN_AUTHZ_DYNAMIC_ACCESS_CHECK = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pace: super::PACE_HEADER, pargs: super::PVOID, pbaceapplicable: super::PBOOL) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY = Option<unsafe extern "system" fn(pcentralaccesspolicy: super::PVOID)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFN_AUTHZ_FREE_DYNAMIC_GROUPS = Option<unsafe extern "system" fn(psidattrarray: super::PSID_AND_ATTRIBUTES)>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, capid: super::PSID, pargs: super::PVOID, pcentralaccesspolicyapplicable: super::PBOOL, ppcentralaccesspolicy: *mut super::PVOID) -> windows_sys::core::BOOL>;
