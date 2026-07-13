#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzAccessCheck(flags : u32, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray : *const super::winnt::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount : u32, preply : *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults : *mut AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzAddSidsToContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, sids : *const super::winnt::SID_AND_ATTRIBUTES, sidcount : u32, restrictedsids : *const super::winnt::SID_AND_ATTRIBUTES, restrictedsidcount : u32, phnewauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzCachedAccessCheck(flags : u32, haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, preply : *mut AUTHZ_ACCESS_REPLY) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzEnumerateSecurityEventSources(dwflags : u32, buffer : *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount : *mut u32, pdwlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzEvaluateSacl(authzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, sacl : *const super::winnt::ACL, grantedaccess : super::winnt::ACCESS_MASK, accessgranted : windows_sys::core::BOOL, pbgenerateaudit : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeAuditEvent(hauditevent : AUTHZ_AUDIT_EVENT_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeCentralAccessPolicyCache() -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeHandle(haccesscheckresults : AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzFreeResourceManager(hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzGetInformationFromContext(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, infoclass : AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize : u32, psizerequired : *mut u32, buffer : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzInitializeCompoundContext(usercontext : AUTHZ_CLIENT_CONTEXT_HANDLE, devicecontext : AUTHZ_CLIENT_CONTEXT_HANDLE, phcompoundcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzInitializeContextFromAuthzContext(flags : u32, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, pexpirationtime : *const i64, identifier : super::winnt::LUID, dynamicgroupargs : *const core::ffi::c_void, phnewauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzInitializeContextFromSid(flags : u32, usersid : super::winnt::PSID, hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime : *const i64, identifier : super::winnt::LUID, dynamicgroupargs : *const core::ffi::c_void, phauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzInitializeContextFromToken(flags : u32, tokenhandle : super::winnt::HANDLE, hauthzresourcemanager : AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime : *const i64, identifier : super::winnt::LUID, dynamicgroupargs : *const core::ffi::c_void, phauthzclientcontext : *mut AUTHZ_CLIENT_CONTEXT_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "C" fn AuthzInitializeObjectAccessAuditEvent(flags : u32, hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype : windows_sys::core::PCWSTR, szobjecttype : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szadditionalinfo : windows_sys::core::PCWSTR, phauditevent : *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount : u32, ...) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "C" fn AuthzInitializeObjectAccessAuditEvent2(flags : u32, hauditeventtype : AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype : windows_sys::core::PCWSTR, szobjecttype : windows_sys::core::PCWSTR, szobjectname : windows_sys::core::PCWSTR, szadditionalinfo : windows_sys::core::PCWSTR, szadditionalinfo2 : windows_sys::core::PCWSTR, phauditevent : *mut AUTHZ_AUDIT_EVENT_HANDLE, dwadditionalparametercount : u32, ...) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzInitializeRemoteResourceManager(prpcinitinfo : *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzInitializeResourceManager(flags : u32, pfndynamicaccesscheck : PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups : PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups : PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername : windows_sys::core::PCWSTR, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzInitializeResourceManagerEx(flags : u32, pauthzinitinfo : *const AUTHZ_INIT_INFO, phauthzresourcemanager : *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzInstallSecurityEventSource(dwflags : u32, pregistration : *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> windows_sys::core::BOOL);
#[cfg(feature = "basetsd")]
windows_link::link!("authz.dll" "system" fn AuthzModifyClaims(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, claimclass : AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations : *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims : *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "basetsd")]
windows_link::link!("authz.dll" "system" fn AuthzModifySecurityAttributes(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, poperations : *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes : *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzModifySids(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, sidclass : AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations : *const AUTHZ_SID_OPERATION, psids : *const super::winnt::TOKEN_GROUPS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzOpenObjectAudit(flags : u32, hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, prequest : *const AUTHZ_ACCESS_REQUEST, hauditevent : AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray : *const super::winnt::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount : u32, preply : *const AUTHZ_ACCESS_REPLY) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("authz.dll" "system" fn AuthzRegisterCapChangeNotification(phcapchangesubscription : *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE, pfncapchangecallback : super::minwinbase::LPTHREAD_START_ROUTINE, pcallbackcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzRegisterSecurityEventSource(dwflags : u32, szeventsourcename : windows_sys::core::PCWSTR, pheventprovider : *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "C" fn AuthzReportSecurityEvent(dwflags : u32, heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : u32, pusersid : super::winnt::PSID, dwcount : u32, ...) -> windows_sys::core::BOOL);
#[cfg(all(feature = "adtgen", feature = "winnt"))]
windows_link::link!("authz.dll" "system" fn AuthzReportSecurityEventFromParams(dwflags : u32, heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : u32, pusersid : super::winnt::PSID, pparams : *const super::adtgen::AUDIT_PARAMS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("authz.dll" "system" fn AuthzSetAppContainerInformation(hauthzclientcontext : AUTHZ_CLIENT_CONTEXT_HANDLE, pappcontainersid : super::winnt::PSID, capabilitycount : u32, pcapabilitysids : *const super::winnt::SID_AND_ATTRIBUTES) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzUninstallSecurityEventSource(dwflags : u32, szeventsourcename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzUnregisterCapChangeNotification(hcapchangesubscription : AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("authz.dll" "system" fn AuthzUnregisterSecurityEventSource(dwflags : u32, pheventprovider : *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE) -> windows_sys::core::BOOL);
pub const AUTHZ_ACCESS_CHECK_NO_DEEP_COPY_SD: u32 = 1;
pub type AUTHZ_ACCESS_CHECK_RESULTS_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AUTHZ_ACCESS_REPLY {
    pub ResultListLength: u32,
    pub GrantedAccessMask: super::winnt::PACCESS_MASK,
    pub SaclEvaluationResults: super::minwindef::PDWORD,
    pub Error: super::minwindef::PDWORD,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for AUTHZ_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct AUTHZ_ACCESS_REQUEST {
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub PrincipalSelfSid: super::winnt::PSID,
    pub ObjectTypeList: super::winnt::POBJECT_TYPE_LIST,
    pub ObjectTypeListLength: u32,
    pub OptionalArguments: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for AUTHZ_ACCESS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AUTHZ_AUDIT_EVENT_HANDLE = *mut core::ffi::c_void;
pub type AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = i32;
pub type AUTHZ_AUDIT_EVENT_TYPE_HANDLE = *mut core::ffi::c_void;
pub type AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE = *mut core::ffi::c_void;
pub type AUTHZ_CLIENT_CONTEXT_HANDLE = *mut core::ffi::c_void;
pub const AUTHZ_COMPUTE_PRIVILEGES: u32 = 8;
pub type AUTHZ_CONTEXT_INFORMATION_CLASS = i32;
pub const AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1;
pub const AUTHZ_GENERATE_FAILURE_AUDIT: u32 = 2;
pub const AUTHZ_GENERATE_SUCCESS_AUDIT: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct AUTHZ_INIT_INFO {
    pub version: u16,
    pub szResourceManagerName: windows_sys::core::PCWSTR,
    pub pfnDynamicAccessCheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK,
    pub pfnComputeDynamicGroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS,
    pub pfnFreeDynamicGroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS,
    pub pfnGetCentralAccessPolicy: PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY,
    pub pfnFreeCentralAccessPolicy: PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY,
}
#[cfg(feature = "winnt")]
impl Default for AUTHZ_INIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_INIT_INFO_VERSION_V1: u32 = 1;
pub const AUTHZ_NO_ALLOC_STRINGS: u32 = 4;
pub const AUTHZ_NO_FAILURE_AUDIT: u32 = 2;
pub const AUTHZ_NO_SUCCESS_AUDIT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    pub szObjectTypeName: windows_sys::core::PWSTR,
    pub dwOffset: u32,
}
impl Default for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_REQUIRE_S4U_LOGON: u32 = 4;
pub type AUTHZ_RESOURCE_MANAGER_HANDLE = *mut core::ffi::c_void;
pub const AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION: u32 = 2;
pub const AUTHZ_RM_FLAG_NO_AUDIT: u32 = 1;
pub const AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_RPC_INIT_INFO_CLIENT {
    pub version: u16,
    pub ObjectUuid: windows_sys::core::PWSTR,
    pub ProtSeq: windows_sys::core::PWSTR,
    pub NetworkAddr: windows_sys::core::PWSTR,
    pub Endpoint: windows_sys::core::PWSTR,
    pub Options: windows_sys::core::PWSTR,
    pub ServerSpn: windows_sys::core::PWSTR,
}
impl Default for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1: u32 = 1;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0,
}
#[cfg(feature = "basetsd")]
impl Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: PAUTHZ_SECURITY_ATTRIBUTE_V1,
}
#[cfg(feature = "basetsd")]
impl Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1;
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub pName: windows_sys::core::PWSTR,
}
impl Default for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut core::ffi::c_void,
    pub ValueLength: u32,
}
impl Default for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AUTHZ_SECURITY_ATTRIBUTE_OPERATION = i32;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_ADD: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 2;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_DELETE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 3;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_NONE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 0;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 4;
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = 1;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_V1 {
    pub pName: windows_sys::core::PWSTR,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: AUTHZ_SECURITY_ATTRIBUTE_V1_0,
}
#[cfg(feature = "basetsd")]
impl Default for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: super::basetsd::PLONG64,
    pub pUint64: super::basetsd::PULONG64,
    pub ppString: *mut windows_sys::core::PWSTR,
    pub pFqbn: PAUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: PAUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
#[cfg(feature = "basetsd")]
impl Default for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTE_VALID_FLAGS: u32 = 3;
pub const AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: u32 = 2;
pub type AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE = *mut core::ffi::c_void;
pub type AUTHZ_SID_OPERATION = i32;
pub const AUTHZ_SID_OPERATION_ADD: AUTHZ_SID_OPERATION = 2;
pub const AUTHZ_SID_OPERATION_DELETE: AUTHZ_SID_OPERATION = 3;
pub const AUTHZ_SID_OPERATION_NONE: AUTHZ_SID_OPERATION = 0;
pub const AUTHZ_SID_OPERATION_REPLACE: AUTHZ_SID_OPERATION = 4;
pub const AUTHZ_SID_OPERATION_REPLACE_ALL: AUTHZ_SID_OPERATION = 1;
pub const AUTHZ_SKIP_TOKEN_GROUPS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    pub dwFlags: u32,
    pub szEventSourceName: windows_sys::core::PWSTR,
    pub szEventMessageFile: windows_sys::core::PWSTR,
    pub szEventSourceXmlSchemaFile: windows_sys::core::PWSTR,
    pub szEventAccessStringsFile: windows_sys::core::PWSTR,
    pub szExecutableImagePath: windows_sys::core::PWSTR,
    pub Anonymous: AUTHZ_SOURCE_SCHEMA_REGISTRATION_0,
    pub dwObjectTypeNameCount: u32,
    pub ObjectTypeNames: [AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET; 1],
}
impl Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    pub pReserved: *mut core::ffi::c_void,
    pub pProviderGuid: *mut windows_sys::core::GUID,
}
impl Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUTHZ_VALID_OBJECT_ACCESS_AUDIT_FLAGS: u32 = 23;
pub const AUTHZ_VALID_RM_INIT_FLAGS: u32 = 7;
pub const AUTHZ_WPD_CATEGORY_FLAG: u32 = 16;
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
pub type PAUTHZ_ACCESS_CHECK_RESULTS_HANDLE = *mut AUTHZ_ACCESS_CHECK_RESULTS_HANDLE;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PAUTHZ_ACCESS_REPLY = *mut AUTHZ_ACCESS_REPLY;
#[cfg(feature = "winnt")]
pub type PAUTHZ_ACCESS_REQUEST = *mut AUTHZ_ACCESS_REQUEST;
pub type PAUTHZ_AUDIT_EVENT_HANDLE = *mut AUTHZ_AUDIT_EVENT_HANDLE;
pub type PAUTHZ_AUDIT_EVENT_TYPE_HANDLE = *mut AUTHZ_AUDIT_EVENT_TYPE_HANDLE;
pub type PAUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE = *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE;
pub type PAUTHZ_CLIENT_CONTEXT_HANDLE = *mut AUTHZ_CLIENT_CONTEXT_HANDLE;
#[cfg(feature = "winnt")]
pub type PAUTHZ_INIT_INFO = *mut AUTHZ_INIT_INFO;
pub type PAUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET = *mut AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET;
pub type PAUTHZ_RESOURCE_MANAGER_HANDLE = *mut AUTHZ_RESOURCE_MANAGER_HANDLE;
pub type PAUTHZ_RPC_INIT_INFO_CLIENT = *mut AUTHZ_RPC_INIT_INFO_CLIENT;
#[cfg(feature = "basetsd")]
pub type PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION = *mut AUTHZ_SECURITY_ATTRIBUTES_INFORMATION;
pub type PAUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE = *mut AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE;
pub type PAUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE = *mut AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE;
pub type PAUTHZ_SECURITY_ATTRIBUTE_OPERATION = *mut AUTHZ_SECURITY_ATTRIBUTE_OPERATION;
#[cfg(feature = "basetsd")]
pub type PAUTHZ_SECURITY_ATTRIBUTE_V1 = *mut AUTHZ_SECURITY_ATTRIBUTE_V1;
pub type PAUTHZ_SECURITY_EVENT_PROVIDER_HANDLE = *mut AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE;
pub type PAUTHZ_SID_OPERATION = *mut AUTHZ_SID_OPERATION;
pub type PAUTHZ_SOURCE_SCHEMA_REGISTRATION = *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION;
#[cfg(feature = "winnt")]
pub type PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, args: *const core::ffi::c_void, psidattrarray: *mut super::winnt::PSID_AND_ATTRIBUTES, psidcount: *mut u32, prestrictedsidattrarray: *mut super::winnt::PSID_AND_ATTRIBUTES, prestrictedsidcount: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PFN_AUTHZ_DYNAMIC_ACCESS_CHECK = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pace: *const super::winnt::ACE_HEADER, pargs: *const core::ffi::c_void, pbaceapplicable: *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL>;
pub type PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY = Option<unsafe extern "system" fn(pcentralaccesspolicy: *const core::ffi::c_void)>;
#[cfg(feature = "winnt")]
pub type PFN_AUTHZ_FREE_DYNAMIC_GROUPS = Option<unsafe extern "system" fn(psidattrarray: *const super::winnt::SID_AND_ATTRIBUTES)>;
#[cfg(feature = "winnt")]
pub type PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY = Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, capid: super::winnt::PSID, pargs: *const core::ffi::c_void, pcentralaccesspolicyapplicable: *mut windows_sys::core::BOOL, ppcentralaccesspolicy: *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
