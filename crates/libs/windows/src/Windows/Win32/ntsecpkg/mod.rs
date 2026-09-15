pub type CRED_FETCH = i32;
pub const CredFetchDPAPI: CRED_FETCH = 1;
pub const CredFetchDefault: CRED_FETCH = 0;
pub const CredFetchForced: CRED_FETCH = 2;
pub type KSEC_CONTEXT_TYPE = i32;
pub type KSEC_CREATE_CONTEXT_LIST = Option<unsafe extern "system" fn(r#type: KSEC_CONTEXT_TYPE) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type KSEC_DEREFERENCE_LIST_ENTRY = Option<unsafe extern "system" fn(entry: PKSEC_LIST_ENTRY, delete: *mut super::BOOLEAN)>;
#[cfg(feature = "winnt")]
pub type KSEC_INSERT_LIST_ENTRY = Option<unsafe extern "system" fn(list: *const core::ffi::c_void, entry: PKSEC_LIST_ENTRY)>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEC_LIST_ENTRY {
    pub List: super::LIST_ENTRY,
    pub RefCount: i32,
    pub Signature: u32,
    pub OwningList: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
pub type KSEC_LOCATE_PKG_BY_ID = Option<unsafe extern "system" fn(packageid: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type KSEC_REFERENCE_LIST_ENTRY = Option<unsafe extern "system" fn(entry: PKSEC_LIST_ENTRY, signature: u32, removenoref: super::BOOLEAN) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type KSEC_SERIALIZE_SCHANNEL_AUTH_DATA = Option<unsafe extern "system" fn(pvauthdata: *const core::ffi::c_void, size: super::PULONG, serializeddata: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type KSEC_SERIALIZE_WINNT_AUTH_DATA = Option<unsafe extern "system" fn(pvauthdata: *const core::ffi::c_void, size: super::PULONG, serializeddata: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
pub const KSecNonPaged: KSEC_CONTEXT_TYPE = 1;
pub const KSecPaged: KSEC_CONTEXT_TYPE = 0;
#[cfg(feature = "sspi")]
pub type KspCompleteTokenFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, token: super::PSecBufferDesc) -> windows_core::NTSTATUS>;
pub type KspDeleteContextFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, lsacontextid: PLSA_SEC_HANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type KspGetTokenFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, impersonationtoken: super::PHANDLE, rawtoken: *mut super::PACCESS_TOKEN) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type KspInitContextFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, contextdata: super::PSecBuffer, newcontextid: PLSA_SEC_HANDLE) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type KspInitPackageFn = Option<unsafe extern "system" fn(functiontable: PSECPKG_KERNEL_FUNCTIONS) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type KspMakeSignatureFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, fqop: u32, message: super::PSecBufferDesc, messageseqno: u32) -> windows_core::NTSTATUS>;
pub type KspMapHandleFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, lsacontextid: PLSA_SEC_HANDLE) -> windows_core::NTSTATUS>;
pub type KspQueryAttributesFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, attribute: u32, buffer: *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type KspSealMessageFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, fqop: u32, message: super::PSecBufferDesc, messageseqno: u32) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type KspSerializeAuthDataFn = Option<unsafe extern "system" fn(pvauthdata: *const core::ffi::c_void, size: super::PULONG, serializeddata: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type KspSetPagingModeFn = Option<unsafe extern "system" fn(pagingmode: super::BOOLEAN) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "sspi"))]
pub type KspUnsealMessageFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, message: super::PSecBufferDesc, messageseqno: u32, pfqop: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "sspi"))]
pub type KspVerifySignatureFn = Option<unsafe extern "system" fn(contextid: LSA_SEC_HANDLE, message: super::PSecBufferDesc, messageseqno: u32, pfqop: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "winnt"))]
pub type LSA_ADD_CREDENTIAL = Option<unsafe extern "system" fn(logonid: super::PLUID, authenticationpackage: u32, primarykeyvalue: super::PLSA_STRING, credentials: super::PLSA_STRING) -> windows_core::NTSTATUS>;
pub type LSA_ALLOCATE_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, lengthrequired: u32, clientbaseaddress: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
pub type LSA_ALLOCATE_LSA_HEAP = Option<unsafe extern "system" fn(length: u32) -> *mut core::ffi::c_void>;
pub type LSA_ALLOCATE_PRIVATE_HEAP = Option<unsafe extern "system" fn(length: usize) -> *mut core::ffi::c_void>;
pub type LSA_ALLOCATE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void, size: u32) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type LSA_AP_CALL_PACKAGE = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type LSA_AP_CALL_PACKAGE_PASSTHROUGH = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type LSA_AP_CALL_PACKAGE_UNTRUSTED = Option<unsafe extern "C" fn(clientrequest: PLSA_CLIENT_REQUEST, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "winnt"))]
pub type LSA_AP_INITIALIZE_PACKAGE = Option<unsafe extern "system" fn(authenticationpackageid: u32, lsadispatchtable: PLSA_DISPATCH_TABLE, database: super::PLSA_STRING, confidentiality: super::PLSA_STRING, authenticationpackagename: *mut super::PLSA_STRING) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_AP_LOGON_TERMINATED = Option<unsafe extern "system" fn(logonid: super::PLUID)>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_AP_LOGON_USER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, logontype: super::SECURITY_LOGON_TYPE, authenticationinformation: *const core::ffi::c_void, clientauthenticationbase: *const core::ffi::c_void, authenticationinformationlength: u32, profilebuffer: *mut *mut core::ffi::c_void, profilebufferlength: super::PULONG, logonid: super::PLUID, substatus: super::PNTSTATUS, tokeninformationtype: PLSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut super::PLSA_UNICODE_STRING, authenticatingauthority: *mut super::PLSA_UNICODE_STRING) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_AP_LOGON_USER_EX = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, logontype: super::SECURITY_LOGON_TYPE, authenticationinformation: *const core::ffi::c_void, clientauthenticationbase: *const core::ffi::c_void, authenticationinformationlength: u32, profilebuffer: *mut *mut core::ffi::c_void, profilebufferlength: super::PULONG, logonid: super::PLUID, substatus: super::PNTSTATUS, tokeninformationtype: PLSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut super::PUNICODE_STRING, authenticatingauthority: *mut super::PUNICODE_STRING, machinename: *mut super::PUNICODE_STRING) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type LSA_AP_LOGON_USER_EX2 = Option<unsafe extern "C" fn(clientrequest: PLSA_CLIENT_REQUEST, logontype: super::SECURITY_LOGON_TYPE, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbuffersize: u32, profilebuffer: *mut *mut core::ffi::c_void, profilebuffersize: super::PULONG, logonid: super::PLUID, substatus: super::PNTSTATUS, tokeninformationtype: PLSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut super::PUNICODE_STRING, authenticatingauthority: *mut super::PUNICODE_STRING, machinename: *mut super::PUNICODE_STRING, primarycredentials: PSECPKG_PRIMARY_CRED, supplementalcredentials: *mut PSECPKG_SUPPLEMENTAL_CRED_ARRAY) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type LSA_AP_LOGON_USER_EX3 = Option<unsafe extern "C" fn(clientrequest: PLSA_CLIENT_REQUEST, logontype: super::SECURITY_LOGON_TYPE, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbuffersize: u32, surrogatelogon: PSECPKG_SURROGATE_LOGON, profilebuffer: *mut *mut core::ffi::c_void, profilebuffersize: super::PULONG, logonid: super::PLUID, substatus: super::PNTSTATUS, tokeninformationtype: PLSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut super::PUNICODE_STRING, authenticatingauthority: *mut super::PUNICODE_STRING, machinename: *mut super::PUNICODE_STRING, primarycredentials: PSECPKG_PRIMARY_CRED, supplementalcredentials: *mut PSECPKG_SUPPLEMENTAL_CRED_ARRAY) -> windows_core::NTSTATUS>;
pub const LSA_AP_NAME_CALL_PACKAGE: windows_core::PCSTR = windows_core::s!("LsaApCallPackage\u{0}");
pub const LSA_AP_NAME_CALL_PACKAGE_PASSTHROUGH: windows_core::PCSTR = windows_core::s!("LsaApCallPackagePassthrough\u{0}");
pub const LSA_AP_NAME_CALL_PACKAGE_UNTRUSTED: windows_core::PCSTR = windows_core::s!("LsaApCallPackageUntrusted\u{0}");
pub const LSA_AP_NAME_INITIALIZE_PACKAGE: windows_core::PCSTR = windows_core::s!("LsaApInitializePackage\u{0}");
pub const LSA_AP_NAME_LOGON_TERMINATED: windows_core::PCSTR = windows_core::s!("LsaApLogonTerminated\u{0}");
pub const LSA_AP_NAME_LOGON_USER: windows_core::PCSTR = windows_core::s!("LsaApLogonUser\u{0}");
pub const LSA_AP_NAME_LOGON_USER_EX: windows_core::PCSTR = windows_core::s!("LsaApLogonUserEx\u{0}");
pub const LSA_AP_NAME_LOGON_USER_EX2: windows_core::PCSTR = windows_core::s!("LsaApLogonUserEx2\u{0}");
#[cfg(feature = "winnt")]
pub type LSA_AP_POST_LOGON_USER = Option<unsafe extern "system" fn(postlogonuserinfo: PSECPKG_POST_LOGON_USER_INFO) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type LSA_AP_POST_LOGON_USER_SURROGATE =
    Option<unsafe extern "C" fn(clientrequest: PLSA_CLIENT_REQUEST, logontype: super::SECURITY_LOGON_TYPE, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbuffersize: u32, surrogatelogon: PSECPKG_SURROGATE_LOGON, profilebuffer: *const core::ffi::c_void, profilebuffersize: u32, logonid: super::PLUID, status: windows_core::NTSTATUS, substatus: windows_core::NTSTATUS, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const core::ffi::c_void, accountname: super::PUNICODE_STRING, authenticatingauthority: super::PUNICODE_STRING, machinename: super::PUNICODE_STRING, primarycredentials: PSECPKG_PRIMARY_CRED, supplementalcredentials: PSECPKG_SUPPLEMENTAL_CRED_ARRAY) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_AP_PRE_LOGON_USER_SURROGATE = Option<unsafe extern "C" fn(clientrequest: PLSA_CLIENT_REQUEST, logontype: super::SECURITY_LOGON_TYPE, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbuffersize: u32, surrogatelogon: PSECPKG_SURROGATE_LOGON, substatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_AUDIT_ACCOUNT_LOGON = Option<unsafe extern "system" fn(auditid: u32, success: super::BOOLEAN, source: super::PUNICODE_STRING, clientname: super::PUNICODE_STRING, mappedname: super::PUNICODE_STRING, status: windows_core::NTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_AUDIT_LOGON = Option<unsafe extern "system" fn(status: windows_core::NTSTATUS, substatus: windows_core::NTSTATUS, accountname: super::PUNICODE_STRING, authenticatingauthority: super::PUNICODE_STRING, workstationname: super::PUNICODE_STRING, usersid: super::PSID, logontype: super::SECURITY_LOGON_TYPE, tokensource: super::PTOKEN_SOURCE, logonid: super::PLUID)>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_AUDIT_LOGON_EX = Option<unsafe extern "system" fn(status: windows_core::NTSTATUS, substatus: windows_core::NTSTATUS, accountname: super::PUNICODE_STRING, authenticatingauthority: super::PUNICODE_STRING, workstationname: super::PUNICODE_STRING, usersid: super::PSID, logontype: super::SECURITY_LOGON_TYPE, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokensource: super::PTOKEN_SOURCE, logonid: super::PLUID)>;
#[cfg(feature = "sspi")]
pub type LSA_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(argument1: usize, argument2: usize, inputbuffer: super::PSecBuffer, outputbuffer: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type LSA_CALL_PACKAGE = Option<unsafe extern "system" fn(authenticationpackage: super::PUNICODE_STRING, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type LSA_CALL_PACKAGEEX = Option<unsafe extern "system" fn(authenticationpackage: super::PUNICODE_STRING, clientbufferbase: *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type LSA_CALL_PACKAGE_PASSTHROUGH = Option<unsafe extern "system" fn(authenticationpackage: super::PUNICODE_STRING, clientbufferbase: *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_CANCEL_NOTIFICATION = Option<unsafe extern "system" fn(notifyhandle: super::HANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_CHECK_PROTECTED_USER_BY_TOKEN = Option<unsafe extern "system" fn(usertoken: super::HANDLE, protecteduser: super::PBOOLEAN) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type LSA_CLIENT_CALLBACK = Option<unsafe extern "system" fn(callback: super::PCHAR, argument1: usize, argument2: usize, input: super::PSecBuffer, output: super::PSecBuffer) -> windows_core::NTSTATUS>;
pub type LSA_CLOSE_SAM_USER = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_CONVERT_AUTH_DATA_TO_TOKEN = Option<unsafe extern "system" fn(userauthdata: *const core::ffi::c_void, userauthdatasize: u32, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokensource: super::PTOKEN_SOURCE, logontype: super::SECURITY_LOGON_TYPE, authorityname: super::PUNICODE_STRING, token: super::PHANDLE, logonid: super::PLUID, accountname: super::PUNICODE_STRING, substatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
pub type LSA_COPY_FROM_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, length: u32, buffertocopy: *mut core::ffi::c_void, clientbaseaddress: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
pub type LSA_COPY_TO_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, length: u32, clientbaseaddress: *mut core::ffi::c_void, buffertocopy: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_CRACK_SINGLE_NAME = Option<unsafe extern "system" fn(formatoffered: u32, performatgc: super::BOOLEAN, nameinput: super::PUNICODE_STRING, prefix: super::PUNICODE_STRING, requestedformat: u32, crackedname: super::PUNICODE_STRING, dnsdomainname: super::PUNICODE_STRING, substatus: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_CREATE_LOGON_SESSION = Option<unsafe extern "system" fn(logonid: super::PLUID) -> windows_core::NTSTATUS>;
pub type LSA_CREATE_SHARED_MEMORY = Option<unsafe extern "system" fn(maxsize: u32, initialsize: u32) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
pub type LSA_CREATE_THREAD = Option<unsafe extern "system" fn(securityattributes: SEC_ATTRS, stacksize: u32, startfunction: SEC_THREAD_START, threadparameter: *const core::ffi::c_void, creationflags: u32, threadid: super::PULONG) -> super::HANDLE>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_CREATE_TOKEN = Option<unsafe extern "system" fn(logonid: super::PLUID, tokensource: super::PTOKEN_SOURCE, logontype: super::SECURITY_LOGON_TYPE, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const core::ffi::c_void, tokengroups: super::PTOKEN_GROUPS, accountname: super::PUNICODE_STRING, authorityname: super::PUNICODE_STRING, workstation: super::PUNICODE_STRING, profilepath: super::PUNICODE_STRING, token: super::PHANDLE, substatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type LSA_CREATE_TOKEN_EX = Option<unsafe extern "system" fn(logonid: super::PLUID, tokensource: super::PTOKEN_SOURCE, logontype: super::SECURITY_LOGON_TYPE, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const core::ffi::c_void, tokengroups: super::PTOKEN_GROUPS, workstation: super::PUNICODE_STRING, profilepath: super::PUNICODE_STRING, sessioninformation: *const core::ffi::c_void, sessioninformationtype: SECPKG_SESSIONINFO_TYPE, token: super::PHANDLE, substatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "winnt"))]
pub type LSA_DELETE_CREDENTIAL = Option<unsafe extern "system" fn(logonid: super::PLUID, authenticationpackage: u32, primarykeyvalue: super::PLSA_STRING) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_DELETE_LOGON_SESSION = Option<unsafe extern "system" fn(logonid: super::PLUID) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_DELETE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void) -> super::BOOLEAN>;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct LSA_DISPATCH_TABLE {
    pub CreateLogonSession: PLSA_CREATE_LOGON_SESSION,
    pub DeleteLogonSession: PLSA_DELETE_LOGON_SESSION,
    pub AddCredential: PLSA_ADD_CREDENTIAL,
    pub GetCredentials: PLSA_GET_CREDENTIALS,
    pub DeleteCredential: PLSA_DELETE_CREDENTIAL,
    pub AllocateLsaHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeLsaHeap: PLSA_FREE_LSA_HEAP,
    pub AllocateClientBuffer: PLSA_ALLOCATE_CLIENT_BUFFER,
    pub FreeClientBuffer: PLSA_FREE_CLIENT_BUFFER,
    pub CopyToClientBuffer: PLSA_COPY_TO_CLIENT_BUFFER,
    pub CopyFromClientBuffer: PLSA_COPY_FROM_CLIENT_BUFFER,
}
#[cfg(feature = "winnt")]
pub type LSA_DUPLICATE_HANDLE = Option<unsafe extern "system" fn(sourcehandle: super::HANDLE, destionationhandle: super::PHANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type LSA_EXPAND_AUTH_DATA_FOR_DOMAIN = Option<unsafe extern "system" fn(userauthdata: super::PUCHAR, userauthdatasize: u32, reserved: *const core::ffi::c_void, expandedauthdata: *mut super::PUCHAR, expandedauthdatasize: super::PULONG) -> windows_core::NTSTATUS>;
pub type LSA_FREE_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, clientbaseaddress: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
pub type LSA_FREE_LSA_HEAP = Option<unsafe extern "system" fn(base: *const core::ffi::c_void)>;
pub type LSA_FREE_PRIVATE_HEAP = Option<unsafe extern "system" fn(base: *const core::ffi::c_void)>;
pub type LSA_FREE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void, memory: *mut core::ffi::c_void)>;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "sspi", feature = "winnt"))]
pub type LSA_GET_APP_MODE_INFO = Option<unsafe extern "system" fn(userfunction: super::PULONG, argument1: super::PULONG_PTR, argument2: super::PULONG_PTR, userdata: super::PSecBuffer, returntolsa: super::PBOOLEAN) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "sspi"))]
pub type LSA_GET_AUTH_DATA_FOR_USER = Option<unsafe extern "system" fn(name: super::PSECURITY_STRING, nametype: SECPKG_NAME_TYPE, prefix: super::PSECURITY_STRING, userauthdata: *mut super::PUCHAR, userauthdatasize: super::PULONG, userflatname: super::PUNICODE_STRING) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_GET_CALL_INFO = Option<unsafe extern "system" fn(info: PSECPKG_CALL_INFO) -> super::BOOLEAN>;
#[cfg(feature = "winnt")]
pub type LSA_GET_CLIENT_INFO = Option<unsafe extern "system" fn(clientinfo: PSECPKG_CLIENT_INFO) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_GET_CLIENT_INFO_EX = Option<unsafe extern "system" fn(clientinfo: PSECPKG_CLIENT_INFO_EX, structsize: u32) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "winnt"))]
pub type LSA_GET_CREDENTIALS = Option<unsafe extern "system" fn(logonid: super::PLUID, authenticationpackage: u32, querycontext: super::PULONG, retrieveallcredentials: super::BOOLEAN, primarykeyvalue: super::PLSA_STRING, primarykeylength: super::PULONG, credentials: super::PLSA_STRING) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type LSA_GET_EXTENDED_CALL_FLAGS = Option<unsafe extern "system" fn(flags: super::PULONG) -> windows_core::NTSTATUS>;
pub type LSA_GET_SECPKG_FAILURE_REASON = Option<unsafe extern "system" fn(packageid: usize, reason: *mut SECPKG_FAILURE_REASON) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type LSA_GET_SERVICE_ACCOUNT_PASSWORD = Option<unsafe extern "system" fn(accountname: super::PUNICODE_STRING, domainname: super::PUNICODE_STRING, credfetch: CRED_FETCH, filetimeexpiry: *mut super::FILETIME, currentpassword: super::PUNICODE_STRING, previouspassword: super::PUNICODE_STRING, filetimecurrpwdvalidforoutbound: *mut super::FILETIME) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type LSA_GET_USER_AUTH_DATA = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void, userauthdata: *mut super::PUCHAR, userauthdatasize: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type LSA_GET_USER_CREDENTIALS = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void, primarycreds: *mut *mut core::ffi::c_void, primarycredssize: super::PULONG, supplementalcreds: *mut *mut core::ffi::c_void, supplementalcredssize: super::PULONG) -> windows_core::NTSTATUS>;
pub type LSA_IMPERSONATE_CLIENT = Option<unsafe extern "system" fn() -> windows_core::NTSTATUS>;
pub type LSA_LOCATE_PKG_BY_ID = Option<unsafe extern "system" fn(packgeid: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "sspi")]
pub type LSA_MAP_BUFFER = Option<unsafe extern "system" fn(inputbuffer: super::PSecBuffer, outputbuffer: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type LSA_OPEN_SAM_USER = Option<unsafe extern "system" fn(name: super::PSECURITY_STRING, nametype: SECPKG_NAME_TYPE, prefix: super::PSECURITY_STRING, allowguest: super::BOOLEAN, reserved: u32, userhandle: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_OPEN_TOKEN_BY_LOGON_ID = Option<unsafe extern "system" fn(logonid: super::PLUID, rettokenhandle: *mut super::HANDLE) -> windows_core::NTSTATUS>;
pub type LSA_PROTECT_MEMORY = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, buffersize: u32)>;
pub const LSA_QUERY_CLIENT_PRELOGON_SESSION_ID: i32 = 1;
pub type LSA_QUERY_CLIENT_REQUEST = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, querytype: u32, replybuffer: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_REDIRECTED_LOGON_CALLBACK = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32, returnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_REDIRECTED_LOGON_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE)>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LSA_REDIRECTED_LOGON_GET_LOGON_CREDS = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, logonbuffer: *mut super::PBYTE, logonbufferlength: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type LSA_REDIRECTED_LOGON_GET_SID = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, sid: *mut super::PSID) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
pub type LSA_REDIRECTED_LOGON_GET_SUPP_CREDS = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, supplementalcredentials: *mut PSECPKG_SUPPLEMENTAL_CRED_ARRAY) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type LSA_REDIRECTED_LOGON_INIT = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, packagename: *const super::UNICODE_STRING, sessionid: u32, logonid: *const super::LUID) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type LSA_REGISTER_CALLBACK = Option<unsafe extern "system" fn(callbackid: u32, callback: PLSA_CALLBACK_FUNCTION) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type LSA_REGISTER_NOTIFICATION = Option<unsafe extern "system" fn(startfunction: SEC_THREAD_START, parameter: *const core::ffi::c_void, notificationtype: u32, notificationclass: u32, notificationflags: u32, intervalminutes: u32, waitevent: super::HANDLE) -> super::HANDLE>;
#[cfg(feature = "winnt")]
pub type LSA_SAVE_SUPPLEMENTAL_CREDENTIALS = Option<unsafe extern "system" fn(logonid: super::PLUID, supplementalcredsize: u32, supplementalcreds: *const core::ffi::c_void, synchronous: super::BOOLEAN) -> windows_core::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwinbase", feature = "minwindef", feature = "ntsecapi", feature = "sspi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct LSA_SECPKG_FUNCTION_TABLE {
    pub CreateLogonSession: PLSA_CREATE_LOGON_SESSION,
    pub DeleteLogonSession: PLSA_DELETE_LOGON_SESSION,
    pub AddCredential: PLSA_ADD_CREDENTIAL,
    pub GetCredentials: PLSA_GET_CREDENTIALS,
    pub DeleteCredential: PLSA_DELETE_CREDENTIAL,
    pub AllocateLsaHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeLsaHeap: PLSA_FREE_LSA_HEAP,
    pub AllocateClientBuffer: PLSA_ALLOCATE_CLIENT_BUFFER,
    pub FreeClientBuffer: PLSA_FREE_CLIENT_BUFFER,
    pub CopyToClientBuffer: PLSA_COPY_TO_CLIENT_BUFFER,
    pub CopyFromClientBuffer: PLSA_COPY_FROM_CLIENT_BUFFER,
    pub ImpersonateClient: PLSA_IMPERSONATE_CLIENT,
    pub UnloadPackage: PLSA_UNLOAD_PACKAGE,
    pub DuplicateHandle: PLSA_DUPLICATE_HANDLE,
    pub SaveSupplementalCredentials: PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS,
    pub CreateThread: PLSA_CREATE_THREAD,
    pub GetClientInfo: PLSA_GET_CLIENT_INFO,
    pub RegisterNotification: PLSA_REGISTER_NOTIFICATION,
    pub CancelNotification: PLSA_CANCEL_NOTIFICATION,
    pub MapBuffer: PLSA_MAP_BUFFER,
    pub CreateToken: PLSA_CREATE_TOKEN,
    pub AuditLogon: PLSA_AUDIT_LOGON,
    pub CallPackage: PLSA_CALL_PACKAGE,
    pub FreeReturnBuffer: PLSA_FREE_LSA_HEAP,
    pub GetCallInfo: PLSA_GET_CALL_INFO,
    pub CallPackageEx: PLSA_CALL_PACKAGEEX,
    pub CreateSharedMemory: PLSA_CREATE_SHARED_MEMORY,
    pub AllocateSharedMemory: PLSA_ALLOCATE_SHARED_MEMORY,
    pub FreeSharedMemory: PLSA_FREE_SHARED_MEMORY,
    pub DeleteSharedMemory: PLSA_DELETE_SHARED_MEMORY,
    pub OpenSamUser: PLSA_OPEN_SAM_USER,
    pub GetUserCredentials: PLSA_GET_USER_CREDENTIALS,
    pub GetUserAuthData: PLSA_GET_USER_AUTH_DATA,
    pub CloseSamUser: PLSA_CLOSE_SAM_USER,
    pub ConvertAuthDataToToken: PLSA_CONVERT_AUTH_DATA_TO_TOKEN,
    pub ClientCallback: PLSA_CLIENT_CALLBACK,
    pub UpdateCredentials: PLSA_UPDATE_PRIMARY_CREDENTIALS,
    pub GetAuthDataForUser: PLSA_GET_AUTH_DATA_FOR_USER,
    pub CrackSingleName: PLSA_CRACK_SINGLE_NAME,
    pub AuditAccountLogon: PLSA_AUDIT_ACCOUNT_LOGON,
    pub CallPackagePassthrough: PLSA_CALL_PACKAGE_PASSTHROUGH,
    pub DummyFunction1: PLSA_PROTECT_MEMORY,
    pub DummyFunction2: PLSA_PROTECT_MEMORY,
    pub DummyFunction3: PLSA_PROTECT_MEMORY,
    pub LsaProtectMemory: PLSA_PROTECT_MEMORY,
    pub LsaUnprotectMemory: PLSA_PROTECT_MEMORY,
    pub OpenTokenByLogonId: PLSA_OPEN_TOKEN_BY_LOGON_ID,
    pub ExpandAuthDataForDomain: PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN,
    pub AllocatePrivateHeap: PLSA_ALLOCATE_PRIVATE_HEAP,
    pub FreePrivateHeap: PLSA_FREE_PRIVATE_HEAP,
    pub CreateTokenEx: PLSA_CREATE_TOKEN_EX,
    pub DummyFunction4: PLSA_PROTECT_MEMORY,
    pub DummyFunction5: PLSA_PROTECT_MEMORY,
    pub DummyFunction6: PLSA_PROTECT_MEMORY,
    pub GetExtendedCallFlags: PLSA_GET_EXTENDED_CALL_FLAGS,
    pub DuplicateTokenHandle: PLSA_DUPLICATE_HANDLE,
    pub GetServiceAccountPassword: PLSA_GET_SERVICE_ACCOUNT_PASSWORD,
    pub DummyFunction7: PLSA_PROTECT_MEMORY,
    pub AuditLogonEx: PLSA_AUDIT_LOGON_EX,
    pub CheckProtectedUserByToken: PLSA_CHECK_PROTECTED_USER_BY_TOKEN,
    pub QueryClientRequest: PLSA_QUERY_CLIENT_REQUEST,
    pub GetAppModeInfo: PLSA_GET_APP_MODE_INFO,
    pub SetAppModeInfo: PLSA_SET_APP_MODE_INFO,
    pub GetClientInfoEx: PLSA_GET_CLIENT_INFO_EX,
    pub GetSecpkgFailureReason: PLSA_GET_SECPKG_FAILURE_REASON,
    pub SetSecpkgFailureReason: PLSA_SET_SECPKG_FAILURE_REASON,
}
pub type LSA_SEC_HANDLE = usize;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type LSA_SET_APP_MODE_INFO = Option<unsafe extern "system" fn(userfunction: u32, argument1: usize, argument2: usize, userdata: super::PSecBuffer, returntolsa: super::BOOLEAN) -> windows_core::NTSTATUS>;
pub type LSA_SET_SECPKG_FAILURE_REASON = Option<unsafe extern "system" fn(reason: SECPKG_FAILURE_REASON) -> windows_core::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LSA_TOKEN_INFORMATION_NULL {
    pub ExpirationTime: super::LARGE_INTEGER,
    pub Groups: super::PTOKEN_GROUPS,
}
#[cfg(feature = "winnt")]
impl Default for LSA_TOKEN_INFORMATION_NULL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LSA_TOKEN_INFORMATION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LSA_TOKEN_INFORMATION_V1 {
    pub ExpirationTime: super::LARGE_INTEGER,
    pub User: super::TOKEN_USER,
    pub Groups: super::PTOKEN_GROUPS,
    pub PrimaryGroup: super::TOKEN_PRIMARY_GROUP,
    pub Privileges: super::PTOKEN_PRIVILEGES,
    pub Owner: super::TOKEN_OWNER,
    pub DefaultDacl: super::TOKEN_DEFAULT_DACL,
}
#[cfg(feature = "winnt")]
impl Default for LSA_TOKEN_INFORMATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type LSA_TOKEN_INFORMATION_V2 = LSA_TOKEN_INFORMATION_V1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LSA_TOKEN_INFORMATION_V3 {
    pub ExpirationTime: super::LARGE_INTEGER,
    pub User: super::TOKEN_USER,
    pub Groups: super::PTOKEN_GROUPS,
    pub PrimaryGroup: super::TOKEN_PRIMARY_GROUP,
    pub Privileges: super::PTOKEN_PRIVILEGES,
    pub Owner: super::TOKEN_OWNER,
    pub DefaultDacl: super::TOKEN_DEFAULT_DACL,
    pub UserClaims: super::TOKEN_USER_CLAIMS,
    pub DeviceClaims: super::TOKEN_DEVICE_CLAIMS,
    pub DeviceGroups: super::PTOKEN_GROUPS,
}
#[cfg(feature = "winnt")]
impl Default for LSA_TOKEN_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LSA_UNLOAD_PACKAGE = Option<unsafe extern "system" fn() -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
pub type LSA_UPDATE_PRIMARY_CREDENTIALS = Option<unsafe extern "system" fn(primarycredentials: PSECPKG_PRIMARY_CRED, credentials: PSECPKG_SUPPLEMENTAL_CRED_ARRAY) -> windows_core::NTSTATUS>;
pub const LsaTokenInformationNull: LSA_TOKEN_INFORMATION_TYPE = 0;
pub const LsaTokenInformationV1: LSA_TOKEN_INFORMATION_TYPE = 1;
pub const LsaTokenInformationV2: LSA_TOKEN_INFORMATION_TYPE = 2;
pub const LsaTokenInformationV3: LSA_TOKEN_INFORMATION_TYPE = 3;
pub const MAX_CRED_SIZE: i32 = 1024;
pub const NOTIFIER_FLAG_NEW_THREAD: i32 = 1;
pub const NOTIFIER_FLAG_ONE_SHOT: i32 = 2;
pub const NOTIFIER_FLAG_SECONDS: u32 = 2147483648;
pub const NOTIFIER_TYPE_HANDLE_WAIT: i32 = 2;
pub const NOTIFIER_TYPE_IMMEDIATE: i32 = 16;
pub const NOTIFIER_TYPE_INTERVAL: i32 = 1;
pub const NOTIFIER_TYPE_NOTIFY_EVENT: i32 = 4;
pub const NOTIFIER_TYPE_STATE_CHANGE: i32 = 3;
pub const NOTIFY_CLASS_DOMAIN_CHANGE: i32 = 3;
pub const NOTIFY_CLASS_PACKAGE_CHANGE: i32 = 1;
pub const NOTIFY_CLASS_REGISTRY_CHANGE: i32 = 4;
pub const NOTIFY_CLASS_ROLE_CHANGE: i32 = 2;
pub const NO_LONG_NAMES: i32 = 2;
pub type PCRED_FETCH = *mut CRED_FETCH;
pub type PKSEC_CREATE_CONTEXT_LIST = Option<unsafe extern "system" fn(r#type: KSEC_CONTEXT_TYPE) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type PKSEC_DEREFERENCE_LIST_ENTRY = Option<unsafe extern "system" fn(entry: PKSEC_LIST_ENTRY, delete: *mut super::BOOLEAN)>;
#[cfg(feature = "winnt")]
pub type PKSEC_INSERT_LIST_ENTRY = Option<unsafe extern "system" fn(list: *const core::ffi::c_void, entry: PKSEC_LIST_ENTRY)>;
#[cfg(feature = "winnt")]
pub type PKSEC_LIST_ENTRY = *mut KSEC_LIST_ENTRY;
pub type PKSEC_LOCATE_PKG_BY_ID = Option<unsafe extern "system" fn(packageid: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type PKSEC_REFERENCE_LIST_ENTRY = Option<unsafe extern "system" fn(entry: PKSEC_LIST_ENTRY, signature: u32, removenoref: super::BOOLEAN) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA = Option<unsafe extern "system" fn(pvauthdata: *const core::ffi::c_void, size: super::PULONG, serializeddata: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type PKSEC_SERIALIZE_WINNT_AUTH_DATA = Option<unsafe extern "system" fn(pvauthdata: *const core::ffi::c_void, size: super::PULONG, serializeddata: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "winnt"))]
pub type PLSA_ADD_CREDENTIAL = Option<unsafe extern "system" fn(logonid: super::PLUID, authenticationpackage: u32, primarykeyvalue: super::PLSA_STRING, credentials: super::PLSA_STRING) -> windows_core::NTSTATUS>;
pub type PLSA_ALLOCATE_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, lengthrequired: u32, clientbaseaddress: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
pub type PLSA_ALLOCATE_LSA_HEAP = Option<unsafe extern "system" fn(length: u32) -> *mut core::ffi::c_void>;
pub type PLSA_ALLOCATE_PRIVATE_HEAP = Option<unsafe extern "system" fn(length: usize) -> *mut core::ffi::c_void>;
pub type PLSA_ALLOCATE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void, size: u32) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PLSA_AP_CALL_PACKAGE = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PLSA_AP_CALL_PACKAGE_PASSTHROUGH = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PLSA_AP_CALL_PACKAGE_UNTRUSTED = Option<unsafe extern "C" fn(param0: PLSA_CLIENT_REQUEST, param1: *mut core::ffi::c_void, param2: *mut core::ffi::c_void, param3: u32, param4: *mut *mut core::ffi::c_void, param5: super::PULONG, param6: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "winnt"))]
pub type PLSA_AP_INITIALIZE_PACKAGE = Option<unsafe extern "system" fn(authenticationpackageid: u32, lsadispatchtable: PLSA_DISPATCH_TABLE, database: super::PLSA_STRING, confidentiality: super::PLSA_STRING, authenticationpackagename: *mut super::PLSA_STRING) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_AP_LOGON_TERMINATED = Option<unsafe extern "system" fn(logonid: super::PLUID)>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_AP_LOGON_USER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, logontype: super::SECURITY_LOGON_TYPE, authenticationinformation: *const core::ffi::c_void, clientauthenticationbase: *const core::ffi::c_void, authenticationinformationlength: u32, profilebuffer: *mut *mut core::ffi::c_void, profilebufferlength: super::PULONG, logonid: super::PLUID, substatus: super::PNTSTATUS, tokeninformationtype: PLSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut super::PLSA_UNICODE_STRING, authenticatingauthority: *mut super::PLSA_UNICODE_STRING) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_AP_LOGON_USER_EX = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, logontype: super::SECURITY_LOGON_TYPE, authenticationinformation: *const core::ffi::c_void, clientauthenticationbase: *const core::ffi::c_void, authenticationinformationlength: u32, profilebuffer: *mut *mut core::ffi::c_void, profilebufferlength: super::PULONG, logonid: super::PLUID, substatus: super::PNTSTATUS, tokeninformationtype: PLSA_TOKEN_INFORMATION_TYPE, tokeninformation: *mut *mut core::ffi::c_void, accountname: *mut super::PUNICODE_STRING, authenticatingauthority: *mut super::PUNICODE_STRING, machinename: *mut super::PUNICODE_STRING) -> windows_core::NTSTATUS>;
pub type PLSA_AP_LOGON_USER_EX2 = *mut u8;
pub type PLSA_AP_LOGON_USER_EX3 = *mut u8;
pub type PLSA_AP_POST_LOGON_USER_SURROGATE = *mut u8;
pub type PLSA_AP_PRE_LOGON_USER_SURROGATE = *mut u8;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_AUDIT_ACCOUNT_LOGON = Option<unsafe extern "system" fn(auditid: u32, success: super::BOOLEAN, source: super::PUNICODE_STRING, clientname: super::PUNICODE_STRING, mappedname: super::PUNICODE_STRING, status: windows_core::NTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_AUDIT_LOGON = Option<unsafe extern "system" fn(status: windows_core::NTSTATUS, substatus: windows_core::NTSTATUS, accountname: super::PUNICODE_STRING, authenticatingauthority: super::PUNICODE_STRING, workstationname: super::PUNICODE_STRING, usersid: super::PSID, logontype: super::SECURITY_LOGON_TYPE, tokensource: super::PTOKEN_SOURCE, logonid: super::PLUID)>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_AUDIT_LOGON_EX = Option<unsafe extern "system" fn(status: windows_core::NTSTATUS, substatus: windows_core::NTSTATUS, accountname: super::PUNICODE_STRING, authenticatingauthority: super::PUNICODE_STRING, workstationname: super::PUNICODE_STRING, usersid: super::PSID, logontype: super::SECURITY_LOGON_TYPE, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokensource: super::PTOKEN_SOURCE, logonid: super::PLUID)>;
#[cfg(feature = "sspi")]
pub type PLSA_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(argument1: usize, argument2: usize, inputbuffer: super::PSecBuffer, outputbuffer: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type PLSA_CALL_PACKAGE = Option<unsafe extern "system" fn(authenticationpackage: super::PUNICODE_STRING, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type PLSA_CALL_PACKAGEEX = Option<unsafe extern "system" fn(authenticationpackage: super::PUNICODE_STRING, clientbufferbase: *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type PLSA_CALL_PACKAGE_PASSTHROUGH = Option<unsafe extern "system" fn(authenticationpackage: super::PUNICODE_STRING, clientbufferbase: *const core::ffi::c_void, protocolsubmitbuffer: *const core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: super::PULONG, protocolstatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_CANCEL_NOTIFICATION = Option<unsafe extern "system" fn(notifyhandle: super::HANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_CHECK_PROTECTED_USER_BY_TOKEN = Option<unsafe extern "system" fn(usertoken: super::HANDLE, protecteduser: super::PBOOLEAN) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type PLSA_CLIENT_CALLBACK = Option<unsafe extern "system" fn(callback: super::PCHAR, argument1: usize, argument2: usize, input: super::PSecBuffer, output: super::PSecBuffer) -> windows_core::NTSTATUS>;
pub type PLSA_CLIENT_REQUEST = *mut *mut core::ffi::c_void;
pub type PLSA_CLOSE_SAM_USER = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_CONVERT_AUTH_DATA_TO_TOKEN = Option<unsafe extern "system" fn(userauthdata: *const core::ffi::c_void, userauthdatasize: u32, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokensource: super::PTOKEN_SOURCE, logontype: super::SECURITY_LOGON_TYPE, authorityname: super::PUNICODE_STRING, token: super::PHANDLE, logonid: super::PLUID, accountname: super::PUNICODE_STRING, substatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
pub type PLSA_COPY_FROM_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, length: u32, buffertocopy: *mut core::ffi::c_void, clientbaseaddress: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
pub type PLSA_COPY_TO_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, length: u32, clientbaseaddress: *mut core::ffi::c_void, buffertocopy: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_CRACK_SINGLE_NAME = Option<unsafe extern "system" fn(formatoffered: u32, performatgc: super::BOOLEAN, nameinput: super::PUNICODE_STRING, prefix: super::PUNICODE_STRING, requestedformat: u32, crackedname: super::PUNICODE_STRING, dnsdomainname: super::PUNICODE_STRING, substatus: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_CREATE_LOGON_SESSION = Option<unsafe extern "system" fn(logonid: super::PLUID) -> windows_core::NTSTATUS>;
pub type PLSA_CREATE_SHARED_MEMORY = Option<unsafe extern "system" fn(maxsize: u32, initialsize: u32) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
pub type PLSA_CREATE_THREAD = Option<unsafe extern "system" fn(securityattributes: SEC_ATTRS, stacksize: u32, startfunction: SEC_THREAD_START, threadparameter: *const core::ffi::c_void, creationflags: u32, threadid: super::PULONG) -> super::HANDLE>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_CREATE_TOKEN = Option<unsafe extern "system" fn(logonid: super::PLUID, tokensource: super::PTOKEN_SOURCE, logontype: super::SECURITY_LOGON_TYPE, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const core::ffi::c_void, tokengroups: super::PTOKEN_GROUPS, accountname: super::PUNICODE_STRING, authorityname: super::PUNICODE_STRING, workstation: super::PUNICODE_STRING, profilepath: super::PUNICODE_STRING, token: super::PHANDLE, substatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PLSA_CREATE_TOKEN_EX = Option<unsafe extern "system" fn(logonid: super::PLUID, tokensource: super::PTOKEN_SOURCE, logontype: super::SECURITY_LOGON_TYPE, impersonationlevel: super::SECURITY_IMPERSONATION_LEVEL, tokeninformationtype: LSA_TOKEN_INFORMATION_TYPE, tokeninformation: *const core::ffi::c_void, tokengroups: super::PTOKEN_GROUPS, workstation: super::PUNICODE_STRING, profilepath: super::PUNICODE_STRING, sessioninformation: *const core::ffi::c_void, sessioninformationtype: SECPKG_SESSIONINFO_TYPE, token: super::PHANDLE, substatus: super::PNTSTATUS) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "winnt"))]
pub type PLSA_DELETE_CREDENTIAL = Option<unsafe extern "system" fn(logonid: super::PLUID, authenticationpackage: u32, primarykeyvalue: super::PLSA_STRING) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_DELETE_LOGON_SESSION = Option<unsafe extern "system" fn(logonid: super::PLUID) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_DELETE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void) -> super::BOOLEAN>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "winnt"))]
pub type PLSA_DISPATCH_TABLE = *mut LSA_DISPATCH_TABLE;
#[cfg(feature = "winnt")]
pub type PLSA_DUPLICATE_HANDLE = Option<unsafe extern "system" fn(sourcehandle: super::HANDLE, destionationhandle: super::PHANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type PLSA_EXPAND_AUTH_DATA_FOR_DOMAIN = Option<unsafe extern "system" fn(userauthdata: super::PUCHAR, userauthdatasize: u32, reserved: *const core::ffi::c_void, expandedauthdata: *mut super::PUCHAR, expandedauthdatasize: super::PULONG) -> windows_core::NTSTATUS>;
pub type PLSA_FREE_CLIENT_BUFFER = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, clientbaseaddress: *const core::ffi::c_void) -> windows_core::NTSTATUS>;
pub type PLSA_FREE_LSA_HEAP = Option<unsafe extern "system" fn(base: *const core::ffi::c_void)>;
pub type PLSA_FREE_PRIVATE_HEAP = Option<unsafe extern "system" fn(base: *const core::ffi::c_void)>;
pub type PLSA_FREE_SHARED_MEMORY = Option<unsafe extern "system" fn(sharedmem: *const core::ffi::c_void, memory: *mut core::ffi::c_void)>;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "sspi", feature = "winnt"))]
pub type PLSA_GET_APP_MODE_INFO = Option<unsafe extern "system" fn(userfunction: super::PULONG, argument1: super::PULONG_PTR, argument2: super::PULONG_PTR, userdata: super::PSecBuffer, returntolsa: super::PBOOLEAN) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "sspi"))]
pub type PLSA_GET_AUTH_DATA_FOR_USER = Option<unsafe extern "system" fn(name: super::PSECURITY_STRING, nametype: SECPKG_NAME_TYPE, prefix: super::PSECURITY_STRING, userauthdata: *mut super::PUCHAR, userauthdatasize: super::PULONG, userflatname: super::PUNICODE_STRING) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_GET_CALL_INFO = Option<unsafe extern "system" fn(info: PSECPKG_CALL_INFO) -> super::BOOLEAN>;
#[cfg(feature = "winnt")]
pub type PLSA_GET_CLIENT_INFO = Option<unsafe extern "system" fn(clientinfo: PSECPKG_CLIENT_INFO) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_GET_CLIENT_INFO_EX = Option<unsafe extern "system" fn(clientinfo: PSECPKG_CLIENT_INFO_EX, structsize: u32) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "winnt"))]
pub type PLSA_GET_CREDENTIALS = Option<unsafe extern "system" fn(logonid: super::PLUID, authenticationpackage: u32, querycontext: super::PULONG, retrieveallcredentials: super::BOOLEAN, primarykeyvalue: super::PLSA_STRING, primarykeylength: super::PULONG, credentials: super::PLSA_STRING) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type PLSA_GET_EXTENDED_CALL_FLAGS = Option<unsafe extern "system" fn(flags: super::PULONG) -> windows_core::NTSTATUS>;
pub type PLSA_GET_SECPKG_FAILURE_REASON = Option<unsafe extern "system" fn(packageid: usize, reason: *mut SECPKG_FAILURE_REASON) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type PLSA_GET_SERVICE_ACCOUNT_PASSWORD = Option<unsafe extern "system" fn(accountname: super::PUNICODE_STRING, domainname: super::PUNICODE_STRING, credfetch: CRED_FETCH, filetimeexpiry: *mut super::FILETIME, currentpassword: super::PUNICODE_STRING, previouspassword: super::PUNICODE_STRING, filetimecurrpwdvalidforoutbound: *mut super::FILETIME) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type PLSA_GET_USER_AUTH_DATA = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void, userauthdata: *mut super::PUCHAR, userauthdatasize: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type PLSA_GET_USER_CREDENTIALS = Option<unsafe extern "system" fn(userhandle: *const core::ffi::c_void, primarycreds: *mut *mut core::ffi::c_void, primarycredssize: super::PULONG, supplementalcreds: *mut *mut core::ffi::c_void, supplementalcredssize: super::PULONG) -> windows_core::NTSTATUS>;
pub type PLSA_IMPERSONATE_CLIENT = Option<unsafe extern "system" fn() -> windows_core::NTSTATUS>;
pub type PLSA_LOCATE_PKG_BY_ID = Option<unsafe extern "system" fn(packgeid: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "sspi")]
pub type PLSA_MAP_BUFFER = Option<unsafe extern "system" fn(inputbuffer: super::PSecBuffer, outputbuffer: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type PLSA_OPEN_SAM_USER = Option<unsafe extern "system" fn(name: super::PSECURITY_STRING, nametype: SECPKG_NAME_TYPE, prefix: super::PSECURITY_STRING, allowguest: super::BOOLEAN, reserved: u32, userhandle: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_OPEN_TOKEN_BY_LOGON_ID = Option<unsafe extern "system" fn(logonid: super::PLUID, rettokenhandle: *mut super::HANDLE) -> windows_core::NTSTATUS>;
pub type PLSA_PROTECT_MEMORY = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, buffersize: u32)>;
pub type PLSA_QUERY_CLIENT_REQUEST = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, querytype: u32, replybuffer: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_REDIRECTED_LOGON_CALLBACK = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32, returnbuffer: *mut *mut core::ffi::c_void, returnbufferlength: *mut u32) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE)>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, logonbuffer: *mut super::PBYTE, logonbufferlength: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_REDIRECTED_LOGON_GET_SID = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, sid: *mut super::PSID) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
pub type PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, supplementalcredentials: *mut PSECPKG_SUPPLEMENTAL_CRED_ARRAY) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type PLSA_REDIRECTED_LOGON_INIT = Option<unsafe extern "system" fn(redirectedlogonhandle: super::HANDLE, packagename: *const super::UNICODE_STRING, sessionid: u32, logonid: *const super::LUID) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type PLSA_REGISTER_CALLBACK = Option<unsafe extern "system" fn(callbackid: u32, callback: PLSA_CALLBACK_FUNCTION) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type PLSA_REGISTER_NOTIFICATION = Option<unsafe extern "system" fn(startfunction: SEC_THREAD_START, parameter: *const core::ffi::c_void, notificationtype: u32, notificationclass: u32, notificationflags: u32, intervalminutes: u32, waitevent: super::HANDLE) -> super::HANDLE>;
#[cfg(feature = "winnt")]
pub type PLSA_SAVE_SUPPLEMENTAL_CREDENTIALS = Option<unsafe extern "system" fn(logonid: super::PLUID, supplementalcredsize: u32, supplementalcreds: *const core::ffi::c_void, synchronous: super::BOOLEAN) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwinbase", feature = "minwindef", feature = "ntsecapi", feature = "sspi", feature = "winnt", feature = "winternl"))]
pub type PLSA_SECPKG_FUNCTION_TABLE = *mut LSA_SECPKG_FUNCTION_TABLE;
pub type PLSA_SEC_HANDLE = *mut LSA_SEC_HANDLE;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type PLSA_SET_APP_MODE_INFO = Option<unsafe extern "system" fn(userfunction: u32, argument1: usize, argument2: usize, userdata: super::PSecBuffer, returntolsa: super::BOOLEAN) -> windows_core::NTSTATUS>;
pub type PLSA_SET_SECPKG_FAILURE_REASON = Option<unsafe extern "system" fn(reason: SECPKG_FAILURE_REASON) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PLSA_TOKEN_INFORMATION_NULL = *mut LSA_TOKEN_INFORMATION_NULL;
pub type PLSA_TOKEN_INFORMATION_TYPE = *mut LSA_TOKEN_INFORMATION_TYPE;
#[cfg(feature = "winnt")]
pub type PLSA_TOKEN_INFORMATION_V1 = *mut LSA_TOKEN_INFORMATION_V1;
#[cfg(feature = "winnt")]
pub type PLSA_TOKEN_INFORMATION_V2 = *mut LSA_TOKEN_INFORMATION_V1;
#[cfg(feature = "winnt")]
pub type PLSA_TOKEN_INFORMATION_V3 = *mut LSA_TOKEN_INFORMATION_V3;
pub type PLSA_UNLOAD_PACKAGE = Option<unsafe extern "system" fn() -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
pub type PLSA_UPDATE_PRIMARY_CREDENTIALS = Option<unsafe extern "system" fn(primarycredentials: PSECPKG_PRIMARY_CRED, credentials: PSECPKG_SUPPLEMENTAL_CRED_ARRAY) -> windows_core::NTSTATUS>;
pub const PRIMARY_CRED_ARSO_LOGON: i32 = 2097152;
pub const PRIMARY_CRED_AUTH_ID: i32 = 512;
pub const PRIMARY_CRED_CACHED_INTERACTIVE_LOGON: i32 = 262144;
pub const PRIMARY_CRED_CACHED_LOGON: i32 = 8;
pub const PRIMARY_CRED_CLEAR_PASSWORD: i32 = 1;
pub const PRIMARY_CRED_DO_NOT_SPLIT: i32 = 1024;
pub const PRIMARY_CRED_ENCRYPTED_CREDGUARD_PASSWORD: i32 = 131072;
pub const PRIMARY_CRED_ENTERPRISE_INTERNET_USER: i32 = 65536;
pub const PRIMARY_CRED_EX: i32 = 4096;
pub const PRIMARY_CRED_FOR_PASSWORD_CHANGE: i32 = 8388608;
pub const PRIMARY_CRED_INTERACTIVE_FIDO_LOGON: i32 = 1048576;
pub const PRIMARY_CRED_INTERACTIVE_NGC_LOGON: i32 = 524288;
pub const PRIMARY_CRED_INTERACTIVE_SMARTCARD_LOGON: i32 = 64;
pub const PRIMARY_CRED_INTERNET_USER: i32 = 256;
pub const PRIMARY_CRED_LOCAL_USER: i32 = 16777216;
pub const PRIMARY_CRED_LOGON_LUA: i32 = 32;
pub const PRIMARY_CRED_LOGON_NO_TCB: i32 = 16;
pub const PRIMARY_CRED_LOGON_PACKAGE_SHIFT: i32 = 24;
pub const PRIMARY_CRED_OWF_PASSWORD: i32 = 2;
pub const PRIMARY_CRED_PACKAGE_MASK: u32 = 4278190080;
pub const PRIMARY_CRED_PACKED_CREDS: i32 = 32768;
pub const PRIMARY_CRED_PROTECTED_USER: i32 = 2048;
pub const PRIMARY_CRED_REFRESH_NEEDED: i32 = 128;
pub const PRIMARY_CRED_RESTRICTED_TS: i32 = 16384;
pub const PRIMARY_CRED_SUPPLEMENTAL: i32 = 4194304;
pub const PRIMARY_CRED_TRANSFER: i32 = 8192;
pub const PRIMARY_CRED_UPDATE: i32 = 4;
pub type PSAM_CREDENTIAL_UPDATE_FREE_ROUTINE = Option<unsafe extern "C" fn(p: *const core::ffi::c_void)>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PSAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE = Option<unsafe extern "C" fn(clearpassword: super::PUNICODE_STRING, oldcredentials: *const core::ffi::c_void, oldcredentialsize: u32, useraccountcontrol: u32, upn: super::PUNICODE_STRING, username: super::PUNICODE_STRING, netbiosdomainname: super::PUNICODE_STRING, dnsdomainname: super::PUNICODE_STRING, newcredentials: *mut *mut core::ffi::c_void, newcredentialsize: *mut u32) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PSAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE = Option<unsafe extern "C" fn(table: *mut SAM_REGISTER_MAPPING_TABLE) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PSAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE = Option<unsafe extern "C" fn(credentialname: super::PUNICODE_STRING) -> super::BOOLEAN>;
#[cfg(feature = "winnt")]
pub type PSAM_REGISTER_MAPPING_ELEMENT = *mut SAM_REGISTER_MAPPING_ELEMENT;
#[cfg(feature = "winnt")]
pub type PSAM_REGISTER_MAPPING_LIST = *mut SAM_REGISTER_MAPPING_LIST;
#[cfg(feature = "winnt")]
pub type PSAM_REGISTER_MAPPING_TABLE = *mut SAM_REGISTER_MAPPING_TABLE;
pub type PSECPKG_BYTE_VECTOR = *mut SECPKG_BYTE_VECTOR;
pub type PSECPKG_CALL_INFO = *mut SECPKG_CALL_INFO;
pub type PSECPKG_CALL_PACKAGE_MESSAGE_TYPE = *mut SECPKG_CALL_PACKAGE_MESSAGE_TYPE;
#[cfg(feature = "winternl")]
pub type PSECPKG_CALL_PACKAGE_PIN_DC_REQUEST = *mut SECPKG_CALL_PACKAGE_PIN_DC_REQUEST;
#[cfg(feature = "winnt")]
pub type PSECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST = *mut SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST;
pub type PSECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST = *mut SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST;
#[cfg(feature = "winnt")]
pub type PSECPKG_CLIENT_INFO = *mut SECPKG_CLIENT_INFO;
#[cfg(feature = "winnt")]
pub type PSECPKG_CLIENT_INFO_EX = *mut SECPKG_CLIENT_INFO_EX;
pub type PSECPKG_CONTEXT_THUNKS = *mut SECPKG_CONTEXT_THUNKS;
#[cfg(feature = "winnt")]
pub type PSECPKG_CREDENTIAL = *mut SECPKG_CREDENTIAL;
#[cfg(feature = "sspi")]
pub type PSECPKG_DLL_FUNCTIONS = *mut SECPKG_DLL_FUNCTIONS;
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type PSECPKG_EVENT_DOMAIN_CHANGE = *mut SECPKG_PARAMETERS;
pub type PSECPKG_EVENT_NOTIFY = *mut SECPKG_EVENT_NOTIFY;
#[cfg(feature = "sspi")]
pub type PSECPKG_EVENT_PACKAGE_CHANGE = *mut SECPKG_EVENT_PACKAGE_CHANGE;
pub type PSECPKG_EVENT_ROLE_CHANGE = *mut SECPKG_EVENT_ROLE_CHANGE;
#[cfg(feature = "sspi")]
pub type PSECPKG_EXTENDED_INFORMATION = *mut SECPKG_EXTENDED_INFORMATION;
pub type PSECPKG_EXTRA_OIDS = *mut SECPKG_EXTRA_OIDS;
pub type PSECPKG_FAILURE_REASON = *mut SECPKG_FAILURE_REASON;
pub type PSECPKG_FAILURE_SPECIAL_REASON = *mut SECPKG_FAILURE_SPECIAL_REASON;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwinbase", feature = "minwindef", feature = "ntsecapi", feature = "sspi", feature = "winnt", feature = "winternl"))]
pub type PSECPKG_FUNCTION_TABLE = *mut SECPKG_FUNCTION_TABLE;
pub type PSECPKG_GSS_INFO = *mut SECPKG_GSS_INFO;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSECPKG_KERNEL_FUNCTIONS = *mut SECPKG_KERNEL_FUNCTIONS;
#[cfg(all(feature = "minwindef", feature = "sspi", feature = "winnt"))]
pub type PSECPKG_KERNEL_FUNCTION_TABLE = *mut SECPKG_KERNEL_FUNCTION_TABLE;
pub type PSECPKG_MUTUAL_AUTH_LEVEL = *mut SECPKG_MUTUAL_AUTH_LEVEL;
pub type PSECPKG_NEGO2_INFO = *mut SECPKG_NEGO2_INFO;
#[cfg(feature = "minwindef")]
pub type PSECPKG_NTLM_TARGETINFO = *mut SECPKG_NTLM_TARGETINFO;
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type PSECPKG_PARAMETERS = *mut SECPKG_PARAMETERS;
#[cfg(feature = "winnt")]
pub type PSECPKG_POST_LOGON_USER_INFO = *mut SECPKG_POST_LOGON_USER_INFO;
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type PSECPKG_PRIMARY_CRED = *mut SECPKG_PRIMARY_CRED;
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type PSECPKG_PRIMARY_CRED_EX = *mut SECPKG_PRIMARY_CRED_EX;
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
pub type PSECPKG_REDIRECTED_LOGON_BUFFER = *mut SECPKG_REDIRECTED_LOGON_BUFFER;
pub type PSECPKG_SERIALIZED_OID = *mut SECPKG_SERIALIZED_OID;
pub type PSECPKG_SHORT_VECTOR = *mut SECPKG_SHORT_VECTOR;
#[cfg(all(feature = "minwindef", feature = "winternl"))]
pub type PSECPKG_SUPPLEMENTAL_CRED = *mut SECPKG_SUPPLEMENTAL_CRED;
#[cfg(all(feature = "minwindef", feature = "winternl"))]
pub type PSECPKG_SUPPLEMENTAL_CRED_ARRAY = *mut SECPKG_SUPPLEMENTAL_CRED_ARRAY;
pub type PSECPKG_SUPPLIED_CREDENTIAL = *mut SECPKG_SUPPLIED_CREDENTIAL;
#[cfg(feature = "winnt")]
pub type PSECPKG_SURROGATE_LOGON = *mut SECPKG_SURROGATE_LOGON;
pub type PSECPKG_SURROGATE_LOGON_ENTRY = *mut SECPKG_SURROGATE_LOGON_ENTRY;
#[cfg(feature = "winnt")]
pub type PSECPKG_TARGETINFO = *mut SECPKG_TARGETINFO;
#[cfg(all(feature = "minwindef", feature = "sspi", feature = "winnt"))]
pub type PSECPKG_USER_FUNCTION_TABLE = *mut SECPKG_USER_FUNCTION_TABLE;
#[cfg(feature = "sspi")]
pub type PSECPKG_WOW_CLIENT_DLL = *mut SECPKG_WOW_CLIENT_DLL;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type PSECURITY_USER_DATA = *mut SECURITY_USER_DATA;
pub type PSEC_WINNT_AUTH_IDENTITY32 = *mut SEC_WINNT_AUTH_IDENTITY32;
pub type PSEC_WINNT_AUTH_IDENTITY_EX32 = *mut SEC_WINNT_AUTH_IDENTITY_EX32;
pub type PSecPkgContext_SaslContext = *mut SecPkgContext_SaslContext;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type PSecurityUserData = *mut SECURITY_USER_DATA;
pub const SAM_CREDENTIAL_UPDATE_FREE_ROUTINE: windows_core::PCSTR = windows_core::s!("CredentialUpdateFree");
pub const SAM_CREDENTIAL_UPDATE_NOTIFY_ROUTINE: windows_core::PCSTR = windows_core::s!("CredentialUpdateNotify");
pub const SAM_CREDENTIAL_UPDATE_REGISTER_MAPPED_ENTRYPOINTS_ROUTINE: windows_core::PCSTR = windows_core::s!("RegisterMappedEntrypoints");
pub const SAM_CREDENTIAL_UPDATE_REGISTER_ROUTINE: windows_core::PCSTR = windows_core::s!("CredentialUpdateRegister");
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SAM_REGISTER_MAPPING_ELEMENT {
    pub Original: windows_core::PSTR,
    pub Mapped: windows_core::PSTR,
    pub Continuable: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SAM_REGISTER_MAPPING_LIST {
    pub Count: u32,
    pub Elements: PSAM_REGISTER_MAPPING_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SAM_REGISTER_MAPPING_TABLE {
    pub Count: u32,
    pub Lists: PSAM_REGISTER_MAPPING_LIST,
}
pub const SECBUFFER_KERNEL_MAP: i32 = 536870912;
pub const SECBUFFER_UNMAPPED: i32 = 1073741824;
pub const SECPKG_ALL_PACKAGES: u32 = 4294967294;
pub const SECPKG_ANSI_ATTRIBUTE: i32 = 0;
pub const SECPKG_ATTR_SASL_CONTEXT: i32 = 65536;
pub const SECPKG_ATTR_THUNK_ALL: i32 = 65536;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_BYTE_VECTOR {
    pub ByteArrayOffset: u32,
    pub ByteArrayLength: u16,
}
pub const SECPKG_CALL_AGENT_LOGON: i32 = 524288;
pub const SECPKG_CALL_ANSI: i32 = 2;
pub const SECPKG_CALL_ASYNC_UPDATE: i32 = 4096;
pub const SECPKG_CALL_BUFFER_MARSHAL: i32 = 65536;
pub const SECPKG_CALL_CLEANUP: i32 = 32;
pub const SECPKG_CALL_CLOUDAP_CONNECT: i32 = 262144;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CALL_INFO {
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub Attributes: u32,
    pub CallCount: u32,
    pub MechOid: *mut core::ffi::c_void,
}
pub const SECPKG_CALL_IN_PROC: i32 = 16;
pub const SECPKG_CALL_IS_TCB: i32 = 512;
pub const SECPKG_CALL_KERNEL_MODE: i32 = 1;
pub const SECPKG_CALL_NEGO: i32 = 16384;
pub const SECPKG_CALL_NEGO_EXTENDER: i32 = 32768;
pub const SECPKG_CALL_NETWORK_ONLY: i32 = 1024;
pub type SECPKG_CALL_PACKAGE_MESSAGE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winternl")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CALL_PACKAGE_PIN_DC_REQUEST {
    pub MessageType: u32,
    pub Flags: u32,
    pub DomainName: super::UNICODE_STRING,
    pub DcName: super::UNICODE_STRING,
    pub DcFlags: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST {
    pub MessageType: u32,
    pub OriginLogonId: super::LUID,
    pub DestinationLogonId: super::LUID,
    pub Flags: u32,
}
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_CLEANUP_CREDENTIALS: i32 = 2;
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_OPTIMISTIC_LOGON: i32 = 1;
pub const SECPKG_CALL_PACKAGE_TRANSFER_CRED_REQUEST_FLAG_TO_SSO_SESSION: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CALL_PACKAGE_UNPIN_ALL_DCS_REQUEST {
    pub MessageType: u32,
    pub Flags: u32,
}
pub const SECPKG_CALL_PROCESS_TERM: i32 = 256;
pub const SECPKG_CALL_RECURSIVE: i32 = 8;
pub const SECPKG_CALL_SYSTEM_PROC: i32 = 8192;
pub const SECPKG_CALL_THREAD_TERM: i32 = 128;
pub const SECPKG_CALL_UNLOCK: i32 = 131072;
pub const SECPKG_CALL_URGENT: i32 = 4;
pub const SECPKG_CALL_WINLOGON: i32 = 2048;
pub const SECPKG_CALL_WOWA32: i32 = 262144;
pub const SECPKG_CALL_WOWCLIENT: i32 = 64;
pub const SECPKG_CALL_WOWX86: i32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CLIENT_INFO {
    pub LogonId: super::LUID,
    pub ProcessID: u32,
    pub ThreadID: u32,
    pub HasTcbPrivilege: super::BOOLEAN,
    pub Impersonating: super::BOOLEAN,
    pub Restricted: super::BOOLEAN,
    pub ClientFlags: u8,
    pub ImpersonationLevel: super::SECURITY_IMPERSONATION_LEVEL,
    pub ClientToken: super::HANDLE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CLIENT_INFO_EX {
    pub LogonId: super::LUID,
    pub ProcessID: u32,
    pub ThreadID: u32,
    pub HasTcbPrivilege: super::BOOLEAN,
    pub Impersonating: super::BOOLEAN,
    pub Restricted: super::BOOLEAN,
    pub ClientFlags: u8,
    pub ImpersonationLevel: super::SECURITY_IMPERSONATION_LEVEL,
    pub ClientToken: super::HANDLE,
    pub IdentificationLogonId: super::LUID,
    pub IdentificationToken: super::HANDLE,
}
pub const SECPKG_CLIENT_PROCESS_TERMINATED: i32 = 1;
pub const SECPKG_CLIENT_THREAD_TERMINATED: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECPKG_CONTEXT_THUNKS {
    pub InfoLevelCount: u32,
    pub Levels: [u32; 1],
}
impl Default for SECPKG_CONTEXT_THUNKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_CREDENTIAL {
    pub Version: u64,
    pub cbHeaderLength: u16,
    pub cbStructureLength: u32,
    pub ClientProcess: u32,
    pub ClientThread: u32,
    pub LogonId: super::LUID,
    pub ClientToken: super::HANDLE,
    pub SessionId: u32,
    pub ModifiedId: super::LUID,
    pub fCredentials: u32,
    pub Flags: u32,
    pub PrincipalName: SECPKG_BYTE_VECTOR,
    pub PackageList: SECPKG_BYTE_VECTOR,
    pub MarshaledSuppliedCreds: SECPKG_BYTE_VECTOR,
}
pub const SECPKG_CREDENTIAL_ATTRIBUTE: i32 = 0;
pub const SECPKG_CREDENTIAL_FLAGS_CALLER_HAS_TCB: i32 = 1;
pub const SECPKG_CREDENTIAL_FLAGS_CREDMAN_CRED: i32 = 2;
pub const SECPKG_CREDENTIAL_VERSION: i32 = 201;
#[repr(C)]
#[cfg(feature = "sspi")]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_DLL_FUNCTIONS {
    pub AllocateHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeHeap: PLSA_FREE_LSA_HEAP,
    pub RegisterCallback: PLSA_REGISTER_CALLBACK,
    pub LocatePackageById: PLSA_LOCATE_PKG_BY_ID,
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type SECPKG_EVENT_DOMAIN_CHANGE = SECPKG_PARAMETERS;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_EVENT_NOTIFY {
    pub EventClass: u32,
    pub Reserved: u32,
    pub EventDataSize: u32,
    pub EventData: *mut core::ffi::c_void,
    pub PackageParameter: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "sspi")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_EVENT_PACKAGE_CHANGE {
    pub ChangeType: u32,
    pub PackageId: LSA_SEC_HANDLE,
    pub PackageName: super::SECURITY_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_EVENT_ROLE_CHANGE {
    pub PreviousRole: u32,
    pub NewRole: u32,
}
#[repr(C)]
#[cfg(feature = "sspi")]
#[derive(Clone, Copy)]
pub struct SECPKG_EXTENDED_INFORMATION {
    pub Class: SECPKG_EXTENDED_INFORMATION_CLASS,
    pub Info: SECPKG_EXTENDED_INFORMATION_0,
}
#[cfg(feature = "sspi")]
impl Default for SECPKG_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "sspi")]
#[derive(Clone, Copy)]
pub union SECPKG_EXTENDED_INFORMATION_0 {
    pub GssInfo: SECPKG_GSS_INFO,
    pub ContextThunks: SECPKG_CONTEXT_THUNKS,
    pub MutualAuthLevel: SECPKG_MUTUAL_AUTH_LEVEL,
    pub WowClientDll: SECPKG_WOW_CLIENT_DLL,
    pub ExtraOids: SECPKG_EXTRA_OIDS,
    pub Nego2Info: SECPKG_NEGO2_INFO,
}
#[cfg(feature = "sspi")]
impl Default for SECPKG_EXTENDED_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SECPKG_EXTENDED_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECPKG_EXTRA_OIDS {
    pub OidCount: u32,
    pub Oids: [SECPKG_SERIALIZED_OID; 1],
}
impl Default for SECPKG_EXTRA_OIDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_FAILURE_REASON {
    pub Status: windows_core::NTSTATUS,
    pub Reason: SECPKG_FAILURE_SPECIAL_REASON,
}
pub type SECPKG_FAILURE_SPECIAL_REASON = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwinbase", feature = "minwindef", feature = "ntsecapi", feature = "sspi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_FUNCTION_TABLE {
    pub InitializePackage: PLSA_AP_INITIALIZE_PACKAGE,
    pub LogonUserA: PLSA_AP_LOGON_USER,
    pub CallPackage: PLSA_AP_CALL_PACKAGE,
    pub LogonTerminated: PLSA_AP_LOGON_TERMINATED,
    pub CallPackageUntrusted: PLSA_AP_CALL_PACKAGE_UNTRUSTED,
    pub CallPackagePassthrough: PLSA_AP_CALL_PACKAGE_PASSTHROUGH,
    pub LogonUserExA: PLSA_AP_LOGON_USER_EX,
    pub LogonUserEx2: PLSA_AP_LOGON_USER_EX2,
    pub Initialize: SpInitializeFn,
    pub Shutdown: SpShutdownFn,
    pub GetInfo: SpGetInfoFn,
    pub AcceptCredentials: SpAcceptCredentialsFn,
    pub AcquireCredentialsHandleA: SpAcquireCredentialsHandleFn,
    pub QueryCredentialsAttributesA: SpQueryCredentialsAttributesFn,
    pub FreeCredentialsHandle: SpFreeCredentialsHandleFn,
    pub SaveCredentials: SpSaveCredentialsFn,
    pub GetCredentials: SpGetCredentialsFn,
    pub DeleteCredentials: SpDeleteCredentialsFn,
    pub InitLsaModeContext: SpInitLsaModeContextFn,
    pub AcceptLsaModeContext: SpAcceptLsaModeContextFn,
    pub DeleteContext: SpDeleteContextFn,
    pub ApplyControlToken: SpApplyControlTokenFn,
    pub GetUserInfo: SpGetUserInfoFn,
    pub GetExtendedInformation: SpGetExtendedInformationFn,
    pub QueryContextAttributesA: SpQueryContextAttributesFn,
    pub AddCredentialsA: SpAddCredentialsFn,
    pub SetExtendedInformation: SpSetExtendedInformationFn,
    pub SetContextAttributesA: SpSetContextAttributesFn,
    pub SetCredentialsAttributesA: SpSetCredentialsAttributesFn,
    pub ChangeAccountPasswordA: SpChangeAccountPasswordFn,
    pub QueryMetaData: SpQueryMetaDataFn,
    pub ExchangeMetaData: SpExchangeMetaDataFn,
    pub GetCredUIContext: SpGetCredUIContextFn,
    pub UpdateCredentials: SpUpdateCredentialsFn,
    pub ValidateTargetInfo: SpValidateTargetInfoFn,
    pub PostLogonUser: LSA_AP_POST_LOGON_USER,
    pub GetRemoteCredGuardLogonBuffer: SpGetRemoteCredGuardLogonBufferFn,
    pub GetRemoteCredGuardSupplementalCreds: SpGetRemoteCredGuardSupplementalCredsFn,
    pub GetTbalSupplementalCreds: SpGetTbalSupplementalCredsFn,
    pub LogonUserEx3: PLSA_AP_LOGON_USER_EX3,
    pub PreLogonUserSurrogate: PLSA_AP_PRE_LOGON_USER_SURROGATE,
    pub PostLogonUserSurrogate: PLSA_AP_POST_LOGON_USER_SURROGATE,
    pub ExtractTargetInfo: SpExtractTargetInfoFn,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECPKG_GSS_INFO {
    pub EncodedIdLength: u32,
    pub EncodedId: [u8; 4],
}
impl Default for SECPKG_GSS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECPKG_INTERFACE_VERSION: i32 = 65536;
pub const SECPKG_INTERFACE_VERSION_10: i32 = 33554432;
pub const SECPKG_INTERFACE_VERSION_11: i32 = 67108864;
pub const SECPKG_INTERFACE_VERSION_2: i32 = 131072;
pub const SECPKG_INTERFACE_VERSION_3: i32 = 262144;
pub const SECPKG_INTERFACE_VERSION_4: i32 = 524288;
pub const SECPKG_INTERFACE_VERSION_5: i32 = 1048576;
pub const SECPKG_INTERFACE_VERSION_6: i32 = 2097152;
pub const SECPKG_INTERFACE_VERSION_7: i32 = 4194304;
pub const SECPKG_INTERFACE_VERSION_8: i32 = 8388608;
pub const SECPKG_INTERFACE_VERSION_9: i32 = 16777216;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_KERNEL_FUNCTIONS {
    pub AllocateHeap: PLSA_ALLOCATE_LSA_HEAP,
    pub FreeHeap: PLSA_FREE_LSA_HEAP,
    pub CreateContextList: PKSEC_CREATE_CONTEXT_LIST,
    pub InsertListEntry: PKSEC_INSERT_LIST_ENTRY,
    pub ReferenceListEntry: PKSEC_REFERENCE_LIST_ENTRY,
    pub DereferenceListEntry: PKSEC_DEREFERENCE_LIST_ENTRY,
    pub SerializeWinntAuthData: PKSEC_SERIALIZE_WINNT_AUTH_DATA,
    pub SerializeSchannelAuthData: PKSEC_SERIALIZE_SCHANNEL_AUTH_DATA,
    pub LocatePackageById: PKSEC_LOCATE_PKG_BY_ID,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "sspi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_KERNEL_FUNCTION_TABLE {
    pub Initialize: KspInitPackageFn,
    pub DeleteContext: KspDeleteContextFn,
    pub InitContext: KspInitContextFn,
    pub MapHandle: KspMapHandleFn,
    pub Sign: KspMakeSignatureFn,
    pub Verify: KspVerifySignatureFn,
    pub Seal: KspSealMessageFn,
    pub Unseal: KspUnsealMessageFn,
    pub GetToken: KspGetTokenFn,
    pub QueryAttributes: KspQueryAttributesFn,
    pub CompleteToken: KspCompleteTokenFn,
    pub ExportContext: SpExportSecurityContextFn,
    pub ImportContext: SpImportSecurityContextFn,
    pub SetPackagePagingMode: KspSetPagingModeFn,
    pub SerializeAuthData: KspSerializeAuthDataFn,
}
pub const SECPKG_LSAMODEINIT_NAME: windows_core::PCSTR = windows_core::s!("SpLsaModeInitialize");
pub const SECPKG_MAX_OID_LENGTH: i32 = 32;
pub const SECPKG_MSVAV_FLAGS_VALID: i32 = 1;
pub const SECPKG_MSVAV_TIMESTAMP_VALID: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_MUTUAL_AUTH_LEVEL {
    pub MutualAuthLevel: u32,
}
pub type SECPKG_NAME_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECPKG_NEGO2_INFO {
    pub AuthScheme: [u8; 16],
    pub PackageFlags: u32,
}
impl Default for SECPKG_NEGO2_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_NTLM_TARGETINFO {
    pub Flags: u32,
    pub MsvAvNbComputerName: windows_core::PWSTR,
    pub MsvAvNbDomainName: windows_core::PWSTR,
    pub MsvAvDnsComputerName: windows_core::PWSTR,
    pub MsvAvDnsDomainName: windows_core::PWSTR,
    pub MsvAvDnsTreeName: windows_core::PWSTR,
    pub MsvAvFlags: u32,
    pub MsvAvTimestamp: super::FILETIME,
    pub MsvAvTargetName: windows_core::PWSTR,
}
pub const SECPKG_PACKAGE_CHANGE_LOAD: i32 = 0;
pub const SECPKG_PACKAGE_CHANGE_SELECT: i32 = 2;
pub const SECPKG_PACKAGE_CHANGE_UNLOAD: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_PARAMETERS {
    pub Version: u32,
    pub MachineState: u32,
    pub SetupMode: u32,
    pub DomainSid: super::PSID,
    pub DomainName: super::UNICODE_STRING,
    pub DnsDomainName: super::UNICODE_STRING,
    pub DomainGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_POST_LOGON_USER_INFO {
    pub Flags: u32,
    pub LogonId: super::LUID,
    pub LinkedLogonId: super::LUID,
}
#[repr(C)]
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_PRIMARY_CRED {
    pub LogonId: super::LUID,
    pub DownlevelName: super::UNICODE_STRING,
    pub DomainName: super::UNICODE_STRING,
    pub Password: super::UNICODE_STRING,
    pub OldPassword: super::UNICODE_STRING,
    pub UserSid: super::PSID,
    pub Flags: u32,
    pub DnsDomainName: super::UNICODE_STRING,
    pub Upn: super::UNICODE_STRING,
    pub LogonServer: super::UNICODE_STRING,
    pub Spare1: super::UNICODE_STRING,
    pub Spare2: super::UNICODE_STRING,
    pub Spare3: super::UNICODE_STRING,
    pub Spare4: super::UNICODE_STRING,
}
#[repr(C)]
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_PRIMARY_CRED_EX {
    pub LogonId: super::LUID,
    pub DownlevelName: super::UNICODE_STRING,
    pub DomainName: super::UNICODE_STRING,
    pub Password: super::UNICODE_STRING,
    pub OldPassword: super::UNICODE_STRING,
    pub UserSid: super::PSID,
    pub Flags: u32,
    pub DnsDomainName: super::UNICODE_STRING,
    pub Upn: super::UNICODE_STRING,
    pub LogonServer: super::UNICODE_STRING,
    pub Spare1: super::UNICODE_STRING,
    pub Spare2: super::UNICODE_STRING,
    pub Spare3: super::UNICODE_STRING,
    pub Spare4: super::UNICODE_STRING,
    pub PackageId: usize,
    pub PrevLogonId: super::LUID,
    pub FlagsEx: u32,
}
pub const SECPKG_PRIMARY_CRED_EX_FLAGS_EX_DELEGATION_TOKEN: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_REDIRECTED_LOGON_BUFFER {
    pub RedirectedLogonGuid: windows_core::GUID,
    pub RedirectedLogonHandle: super::HANDLE,
    pub Init: PLSA_REDIRECTED_LOGON_INIT,
    pub Callback: PLSA_REDIRECTED_LOGON_CALLBACK,
    pub CleanupCallback: PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK,
    pub GetLogonCreds: PLSA_REDIRECTED_LOGON_GET_LOGON_CREDS,
    pub GetSupplementalCreds: PLSA_REDIRECTED_LOGON_GET_SUPP_CREDS,
    pub GetRedirectedLogonSid: PLSA_REDIRECTED_LOGON_GET_SID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECPKG_SERIALIZED_OID {
    pub OidLength: u32,
    pub OidAttributes: u32,
    pub OidValue: [u8; 32],
}
impl Default for SECPKG_SERIALIZED_OID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SECPKG_SESSIONINFO_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_SHORT_VECTOR {
    pub ShortArrayOffset: u32,
    pub ShortArrayCount: u16,
}
pub const SECPKG_STATE_CRED_ISOLATION_ENABLED: i32 = 32;
pub const SECPKG_STATE_DOMAIN_CONTROLLER: i32 = 4;
pub const SECPKG_STATE_ENCRYPTION_PERMITTED: i32 = 1;
pub const SECPKG_STATE_RESERVED_1: u32 = 2147483648;
pub const SECPKG_STATE_STANDALONE: i32 = 16;
pub const SECPKG_STATE_STRONG_ENCRYPTION_PERMITTED: i32 = 2;
pub const SECPKG_STATE_WORKSTATION: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_SUPPLEMENTAL_CRED {
    pub PackageName: super::UNICODE_STRING,
    pub CredentialSize: u32,
    pub Credentials: super::PUCHAR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    pub CredentialCount: u32,
    pub Credentials: [SECPKG_SUPPLEMENTAL_CRED; 1],
}
#[cfg(all(feature = "minwindef", feature = "winternl"))]
impl Default for SECPKG_SUPPLEMENTAL_CRED_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_SUPPLIED_CREDENTIAL {
    pub cbHeaderLength: u16,
    pub cbStructureLength: u16,
    pub UserName: SECPKG_SHORT_VECTOR,
    pub DomainName: SECPKG_SHORT_VECTOR,
    pub PackedCredentials: SECPKG_BYTE_VECTOR,
    pub CredFlags: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_SURROGATE_LOGON {
    pub Version: u32,
    pub SurrogateLogonID: super::LUID,
    pub EntryCount: u32,
    pub Entries: PSECPKG_SURROGATE_LOGON_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_SURROGATE_LOGON_ENTRY {
    pub Type: windows_core::GUID,
    pub Data: *mut core::ffi::c_void,
}
pub const SECPKG_SURROGATE_LOGON_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_TARGETINFO {
    pub DomainSid: super::PSID,
    pub ComputerName: windows_core::PCWSTR,
}
pub const SECPKG_UNICODE_ATTRIBUTE: u32 = 2147483648;
pub const SECPKG_USERMODEINIT_NAME: windows_core::PCSTR = windows_core::s!("SpUserModeInitialize");
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "sspi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECPKG_USER_FUNCTION_TABLE {
    pub InstanceInit: SpInstanceInitFn,
    pub InitUserModeContext: SpInitUserModeContextFn,
    pub MakeSignature: SpMakeSignatureFn,
    pub VerifySignature: SpVerifySignatureFn,
    pub SealMessage: SpSealMessageFn,
    pub UnsealMessage: SpUnsealMessageFn,
    pub GetContextToken: SpGetContextTokenFn,
    pub QueryContextAttributesA: SpQueryContextAttributesFn,
    pub CompleteAuthToken: SpCompleteAuthTokenFn,
    pub DeleteUserModeContext: SpDeleteContextFn,
    pub FormatCredentials: SpFormatCredentialsFn,
    pub MarshallSupplementalCreds: SpMarshallSupplementalCredsFn,
    pub ExportContext: SpExportSecurityContextFn,
    pub ImportContext: SpImportSecurityContextFn,
    pub MarshalAttributeData: SpMarshalAttributeDataFn,
}
#[repr(C)]
#[cfg(feature = "sspi")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECPKG_WOW_CLIENT_DLL {
    pub WowClientDllPath: super::SECURITY_STRING,
}
#[repr(C)]
#[cfg(all(feature = "sspi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_USER_DATA {
    pub UserName: super::SECURITY_STRING,
    pub LogonDomainName: super::SECURITY_STRING,
    pub LogonServer: super::SECURITY_STRING,
    pub pSid: super::PSID,
}
#[cfg(feature = "minwinbase")]
pub type SEC_ATTRS = super::LPSECURITY_ATTRIBUTES;
#[cfg(feature = "minwinbase")]
pub type SEC_THREAD_START = super::LPTHREAD_START_ROUTINE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY32 {
    pub User: u32,
    pub UserLength: u32,
    pub Domain: u32,
    pub DomainLength: u32,
    pub Password: u32,
    pub PasswordLength: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_EX32 {
    pub Version: u32,
    pub Length: u32,
    pub User: u32,
    pub UserLength: u32,
    pub Domain: u32,
    pub DomainLength: u32,
    pub Password: u32,
    pub PasswordLength: u32,
    pub Flags: u32,
    pub PackageList: u32,
    pub PackageListLength: u32,
}
pub const SP_ACCEPT_CREDENTIALS_NAME: windows_core::PCSTR = windows_core::s!("SpAcceptCredentials\u{0}");
pub const SecNameAlternateId: SECPKG_NAME_TYPE = 1;
pub const SecNameDN: SECPKG_NAME_TYPE = 3;
pub const SecNameFlat: SECPKG_NAME_TYPE = 2;
pub const SecNameSPN: SECPKG_NAME_TYPE = 4;
pub const SecNameSamCompatible: SECPKG_NAME_TYPE = 0;
pub const SecPkgCallPackageMaxMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = 1026;
pub const SecPkgCallPackageMinMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = 1024;
pub const SecPkgCallPackagePinDcMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = 1024;
pub const SecPkgCallPackageTransferCredMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = 1026;
pub const SecPkgCallPackageUnpinAllDcsMessage: SECPKG_CALL_PACKAGE_MESSAGE_TYPE = 1025;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecPkgContext_SaslContext {
    pub SaslContext: *mut core::ffi::c_void,
}
pub const SecSessionPrimaryCred: SECPKG_SESSIONINFO_TYPE = 0;
pub const SecpkgContextThunks: SECPKG_EXTENDED_INFORMATION_CLASS = 2;
pub const SecpkgExtraOids: SECPKG_EXTENDED_INFORMATION_CLASS = 5;
pub const SecpkgFailureReason_CloudAccount: SECPKG_FAILURE_SPECIAL_REASON = 4;
pub const SecpkgFailureReason_DomainAccount: SECPKG_FAILURE_SPECIAL_REASON = 3;
pub const SecpkgFailureReason_DupTarget: SECPKG_FAILURE_SPECIAL_REASON = 8;
pub const SecpkgFailureReason_GlobalSPN: SECPKG_FAILURE_SPECIAL_REASON = 12;
pub const SecpkgFailureReason_IpAddress: SECPKG_FAILURE_SPECIAL_REASON = 7;
pub const SecpkgFailureReason_LocalAccount: SECPKG_FAILURE_SPECIAL_REASON = 2;
pub const SecpkgFailureReason_Loopback: SECPKG_FAILURE_SPECIAL_REASON = 10;
pub const SecpkgFailureReason_NoFailure: SECPKG_FAILURE_SPECIAL_REASON = 1;
pub const SecpkgFailureReason_NoLineOfSight: SECPKG_FAILURE_SPECIAL_REASON = 9;
pub const SecpkgFailureReason_NullSession: SECPKG_FAILURE_SPECIAL_REASON = 11;
pub const SecpkgFailureReason_NullTarget: SECPKG_FAILURE_SPECIAL_REASON = 5;
pub const SecpkgFailureReason_Unknown: SECPKG_FAILURE_SPECIAL_REASON = 0;
pub const SecpkgFailureReason_UnknownTarget: SECPKG_FAILURE_SPECIAL_REASON = 6;
pub const SecpkgGssInfo: SECPKG_EXTENDED_INFORMATION_CLASS = 1;
pub const SecpkgMaxInfo: SECPKG_EXTENDED_INFORMATION_CLASS = 6;
pub const SecpkgMutualAuthLevel: SECPKG_EXTENDED_INFORMATION_CLASS = 3;
pub const SecpkgNego2Info: SECPKG_EXTENDED_INFORMATION_CLASS = 7;
pub const SecpkgWowClientDll: SECPKG_EXTENDED_INFORMATION_CLASS = 4;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type SecurityUserData = SECURITY_USER_DATA;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type SpAcceptCredentialsFn = Option<unsafe extern "system" fn(logontype: super::SECURITY_LOGON_TYPE, accountname: super::PUNICODE_STRING, primarycredentials: PSECPKG_PRIMARY_CRED, supplementalcredentials: PSECPKG_SUPPLEMENTAL_CRED) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "sspi", feature = "winnt"))]
pub type SpAcceptLsaModeContextFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, contexthandle: LSA_SEC_HANDLE, inputbuffer: super::PSecBufferDesc, contextrequirements: u32, targetdatarep: u32, newcontexthandle: PLSA_SEC_HANDLE, outputbuffer: super::PSecBufferDesc, contextattributes: super::PULONG, expirationtime: super::PTimeStamp, mappedcontext: super::PBOOLEAN, contextdata: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "sspi", feature = "winnt"))]
pub type SpAcquireCredentialsHandleFn = Option<unsafe extern "system" fn(principalname: super::PUNICODE_STRING, credentialuseflags: u32, logonid: super::PLUID, authorizationdata: *const core::ffi::c_void, getkeyfunciton: *const core::ffi::c_void, getkeyargument: *const core::ffi::c_void, credentialhandle: PLSA_SEC_HANDLE, expirationtime: super::PTimeStamp) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "sspi", feature = "winnt"))]
pub type SpAddCredentialsFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, principalname: super::PUNICODE_STRING, package: super::PUNICODE_STRING, credentialuseflags: u32, authorizationdata: *const core::ffi::c_void, getkeyfunciton: *const core::ffi::c_void, getkeyargument: *const core::ffi::c_void, expirationtime: super::PTimeStamp) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpApplyControlTokenFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, controltoken: super::PSecBufferDesc) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "sspi", feature = "winnt"))]
pub type SpChangeAccountPasswordFn = Option<unsafe extern "system" fn(pdomainname: super::PUNICODE_STRING, paccountname: super::PUNICODE_STRING, poldpassword: super::PUNICODE_STRING, pnewpassword: super::PUNICODE_STRING, impersonating: super::BOOLEAN, poutput: super::PSecBufferDesc) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpCompleteAuthTokenFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, inputbuffer: super::PSecBufferDesc) -> windows_core::NTSTATUS>;
pub type SpDeleteContextFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpDeleteCredentialsFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, key: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type SpExchangeMetaDataFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, targetname: super::PUNICODE_STRING, contextrequirements: u32, metadatalength: u32, metadata: super::PUCHAR, contexthandle: PLSA_SEC_HANDLE) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type SpExportSecurityContextFn = Option<unsafe extern "system" fn(phcontext: LSA_SEC_HANDLE, fflags: u32, ppackedcontext: super::PSecBuffer, ptoken: super::PHANDLE) -> windows_core::NTSTATUS>;
pub type SpExtractTargetInfoFn = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, ppvtargetinfo: *mut *mut core::ffi::c_void, pcbtargetinfo: *mut u32) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpFormatCredentialsFn = Option<unsafe extern "system" fn(credentials: super::PSecBuffer, formattedcredentials: super::PSecBuffer) -> windows_core::NTSTATUS>;
pub type SpFreeCredentialsHandleFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type SpGetContextTokenFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, impersonationtoken: super::PHANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type SpGetCredUIContextFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, credtype: *const windows_core::GUID, flatcreduicontextlength: super::PULONG, flatcreduicontext: *mut super::PUCHAR) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpGetCredentialsFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, credentials: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpGetExtendedInformationFn = Option<unsafe extern "system" fn(class: SECPKG_EXTENDED_INFORMATION_CLASS, ppinformation: *mut PSECPKG_EXTENDED_INFORMATION) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpGetInfoFn = Option<unsafe extern "system" fn(packageinfo: super::PSecPkgInfoA) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
pub type SpGetRemoteCredGuardLogonBufferFn = Option<unsafe extern "system" fn(credhandle: LSA_SEC_HANDLE, contexthandle: LSA_SEC_HANDLE, targetname: *const super::UNICODE_STRING, redirectedlogonhandle: super::PHANDLE, callback: *mut PLSA_REDIRECTED_LOGON_CALLBACK, cleanupcallback: *mut PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK, logonbuffersize: super::PULONG, logonbuffer: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winternl"))]
pub type SpGetRemoteCredGuardSupplementalCredsFn = Option<unsafe extern "system" fn(credhandle: LSA_SEC_HANDLE, targetname: *const super::UNICODE_STRING, redirectedlogonhandle: super::PHANDLE, callback: *mut PLSA_REDIRECTED_LOGON_CALLBACK, cleanupcallback: *mut PLSA_REDIRECTED_LOGON_CLEANUP_CALLBACK, supplementalcredssize: super::PULONG, supplementalcreds: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type SpGetTbalSupplementalCredsFn = Option<unsafe extern "system" fn(logonid: super::LUID, supplementalcredssize: super::PULONG, supplementalcreds: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type SpGetUserInfoFn = Option<unsafe extern "system" fn(logonid: super::PLUID, flags: u32, userdata: *mut PSecurityUserData) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "sspi", feature = "winnt"))]
pub type SpImportSecurityContextFn = Option<unsafe extern "system" fn(ppackedcontext: super::PSecBuffer, token: super::HANDLE, phcontext: PLSA_SEC_HANDLE) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "sspi", feature = "winnt"))]
pub type SpInitLsaModeContextFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, contexthandle: LSA_SEC_HANDLE, targetname: super::PUNICODE_STRING, contextrequirements: u32, targetdatarep: u32, inputbuffers: super::PSecBufferDesc, newcontexthandle: PLSA_SEC_HANDLE, outputbuffers: super::PSecBufferDesc, contextattributes: super::PULONG, expirationtime: super::PTimeStamp, mappedcontext: super::PBOOLEAN, contextdata: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpInitUserModeContextFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, packedcontext: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwinbase", feature = "minwindef", feature = "ntsecapi", feature = "sspi", feature = "winnt", feature = "winternl"))]
pub type SpInitializeFn = Option<unsafe extern "system" fn(packageid: usize, parameters: PSECPKG_PARAMETERS, functiontable: PLSA_SECPKG_FUNCTION_TABLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpInstanceInitFn = Option<unsafe extern "system" fn(version: u32, functiontable: PSECPKG_DLL_FUNCTIONS, userfunctions: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "minwinbase", feature = "minwindef", feature = "ntsecapi", feature = "sspi", feature = "winnt", feature = "winternl"))]
pub type SpLsaModeInitializeFn = Option<unsafe extern "system" fn(lsaversion: u32, packageversion: super::PULONG, pptables: *mut PSECPKG_FUNCTION_TABLE, pctables: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpMakeSignatureFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, qualityofprotection: u32, messagebuffers: super::PSecBufferDesc, messagesequencenumber: u32) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type SpMarshalAttributeDataFn = Option<unsafe extern "system" fn(attributeinfo: u32, attribute: u32, attributedatasize: u32, attributedata: super::PBYTE, marshaledattributedatasize: super::PULONG, marshaledattributedata: *mut super::PBYTE) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type SpMarshallSupplementalCredsFn = Option<unsafe extern "system" fn(credentialsize: u32, credentials: super::PUCHAR, marshalledcredsize: super::PULONG, marshalledcreds: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
pub type SpQueryContextAttributesFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, contextattribute: u32, buffer: *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
pub type SpQueryCredentialsAttributesFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, credentialattribute: u32, buffer: *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type SpQueryMetaDataFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, targetname: super::PUNICODE_STRING, contextrequirements: u32, metadatalength: super::PULONG, metadata: *mut super::PUCHAR, contexthandle: PLSA_SEC_HANDLE) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpSaveCredentialsFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, credentials: super::PSecBuffer) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpSealMessageFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, qualityofprotection: u32, messagebuffers: super::PSecBufferDesc, messagesequencenumber: u32) -> windows_core::NTSTATUS>;
pub type SpSetContextAttributesFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, contextattribute: u32, buffer: *const core::ffi::c_void, buffersize: u32) -> windows_core::NTSTATUS>;
pub type SpSetCredentialsAttributesFn = Option<unsafe extern "system" fn(credentialhandle: LSA_SEC_HANDLE, credentialattribute: u32, buffer: *const core::ffi::c_void, buffersize: u32) -> windows_core::NTSTATUS>;
#[cfg(feature = "sspi")]
pub type SpSetExtendedInformationFn = Option<unsafe extern "system" fn(class: SECPKG_EXTENDED_INFORMATION_CLASS, info: PSECPKG_EXTENDED_INFORMATION) -> windows_core::NTSTATUS>;
pub type SpShutdownFn = Option<unsafe extern "system" fn() -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "sspi"))]
pub type SpUnsealMessageFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, messagebuffers: super::PSecBufferDesc, messagesequencenumber: u32, qualityofprotection: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "minwindef")]
pub type SpUpdateCredentialsFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, credtype: *const windows_core::GUID, flatcreduicontextlength: u32, flatcreduicontext: super::PUCHAR) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "sspi", feature = "winnt"))]
pub type SpUserModeInitializeFn = Option<unsafe extern "system" fn(lsaversion: u32, packageversion: super::PULONG, pptables: *mut PSECPKG_USER_FUNCTION_TABLE, pctables: super::PULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type SpValidateTargetInfoFn = Option<unsafe extern "system" fn(clientrequest: PLSA_CLIENT_REQUEST, protocolsubmitbuffer: *const core::ffi::c_void, clientbufferbase: *const core::ffi::c_void, submitbufferlength: u32, targetinfo: PSECPKG_TARGETINFO) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "sspi"))]
pub type SpVerifySignatureFn = Option<unsafe extern "system" fn(contexthandle: LSA_SEC_HANDLE, messagebuffers: super::PSecBufferDesc, messagesequencenumber: u32, qualityofprotection: super::PULONG) -> windows_core::NTSTATUS>;
pub const UNDERSTANDS_LONG_NAMES: i32 = 1;
