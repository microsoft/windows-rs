#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const AdSyncTask: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 719734609, data2: 46888, data3: 19819, data4: [151, 160, 178, 218, 46, 125, 42, 59] };
pub const AdrClientDisplayFlags_AllowEmailRequests: i32 = 1i32;
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: i32 = 2i32;
pub const AdrClientErrorType_Unknown: i32 = 0i32;
pub const AdrClientErrorType_AccessDenied: i32 = 1i32;
pub const AdrClientErrorType_FileNotFound: i32 = 2i32;
pub const AdrClientFlags_None: i32 = 0i32;
pub const AdrClientFlags_FailForLocalPaths: i32 = 1i32;
pub const AdrClientFlags_FailIfNotSupportedByServer: i32 = 2i32;
pub const AdrClientFlags_FailIfNotDomainJoined: i32 = 4i32;
pub const AdrEmailFlags_PutDataOwnerOnToLine: i32 = 1i32;
pub const AdrEmailFlags_PutAdminOnToLine: i32 = 2i32;
pub const AdrEmailFlags_IncludeDeviceClaims: i32 = 4i32;
pub const AdrEmailFlags_IncludeUserInfo: i32 = 8i32;
pub const AdrEmailFlags_GenerateEventLog: i32 = 16i32;
#[repr(transparent)]
pub struct DIFsrmClassificationEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DIFsrmClassificationEvents {}
impl ::core::clone::Clone for DIFsrmClassificationEvents {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
pub const FSRM_E_ADR_MAX_EMAILS_SENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200130i32 as _);
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200110i32 as _);
pub const FSRM_E_ADR_PATH_IS_LOCAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200111i32 as _);
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200112i32 as _);
pub const FSRM_E_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200253i32 as _);
pub const FSRM_E_AUTO_QUOTA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(283419i32 as _);
pub const FSRM_E_CACHE_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200187i32 as _);
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200186i32 as _);
pub const FSRM_E_CANNOT_AGGREGATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200201i32 as _);
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200170i32 as _);
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200197i32 as _);
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200132i32 as _);
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200135i32 as _);
pub const FSRM_E_CANNOT_REMOVE_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200109i32 as _);
pub const FSRM_E_CANNOT_RENAME_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200198i32 as _);
pub const FSRM_E_CANNOT_STORE_PROPERTIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200171i32 as _);
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200143i32 as _);
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200145i32 as _);
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200195i32 as _);
pub const FSRM_E_CLASSIFICATION_CANCELED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200141i32 as _);
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200194i32 as _);
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200136i32 as _);
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200148i32 as _);
pub const FSRM_E_CLASSIFICATION_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200137i32 as _);
pub const FSRM_E_CLUSTER_NOT_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200210i32 as _);
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200106i32 as _);
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200207i32 as _);
pub const FSRM_E_DRIVER_NOT_READY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200237i32 as _);
pub const FSRM_E_DUPLICATE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200240i32 as _);
pub const FSRM_E_EMAIL_NOT_SENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200228i32 as _);
pub const FSRM_E_ENUM_PROPERTIES_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200173i32 as _);
pub const FSRM_E_ERROR_NOT_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200133i32 as _);
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200105i32 as _);
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200104i32 as _);
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200103i32 as _);
pub const FSRM_E_FAIL_BATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200247i32 as _);
pub const FSRM_E_FILE_ENCRYPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200156i32 as _);
pub const FSRM_E_FILE_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200134i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200152i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200153i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200185i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200184i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200193i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200191i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200102i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200192i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200108i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200146i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200190i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200147i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200120i32 as _);
pub const FSRM_E_FILE_OPEN_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200189i32 as _);
pub const FSRM_E_FILE_SYSTEM_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200225i32 as _);
pub const FSRM_E_INCOMPATIBLE_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200157i32 as _);
pub const FSRM_E_INPROC_MODULE_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200174i32 as _);
pub const FSRM_E_INSECURE_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200233i32 as _);
pub const FSRM_E_INSUFFICIENT_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200236i32 as _);
pub const FSRM_E_INVALID_AD_CLAIM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200142i32 as _);
pub const FSRM_E_INVALID_COMBINATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200241i32 as _);
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200220i32 as _);
pub const FSRM_E_INVALID_EMAIL_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200226i32 as _);
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200223i32 as _);
pub const FSRM_E_INVALID_FILENAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200214i32 as _);
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200140i32 as _);
pub const FSRM_E_INVALID_IMPORT_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200245i32 as _);
pub const FSRM_E_INVALID_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200249i32 as _);
pub const FSRM_E_INVALID_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200248i32 as _);
pub const FSRM_E_INVALID_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200250i32 as _);
pub const FSRM_E_INVALID_REPORT_DESC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200215i32 as _);
pub const FSRM_E_INVALID_REPORT_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200216i32 as _);
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200254i32 as _);
pub const FSRM_E_INVALID_SMTP_SERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200232i32 as _);
pub const FSRM_E_INVALID_TEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200246i32 as _);
pub const FSRM_E_INVALID_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200251i32 as _);
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200176i32 as _);
pub const FSRM_E_LEGACY_SCHEDULE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200107i32 as _);
pub const FSRM_E_LOADING_DISABLED_MODULE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200202i32 as _);
pub const FSRM_E_LONG_CMDLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200224i32 as _);
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200196i32 as _);
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200200i32 as _);
pub const FSRM_E_MODULE_INITIALIZATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200150i32 as _);
pub const FSRM_E_MODULE_INVALID_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200151i32 as _);
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200149i32 as _);
pub const FSRM_E_MODULE_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200101i32 as _);
pub const FSRM_E_NOT_CLUSTER_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200208i32 as _);
pub const FSRM_E_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200255i32 as _);
pub const FSRM_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200239i32 as _);
pub const FSRM_E_NO_EMAIL_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200131i32 as _);
pub const FSRM_E_NO_PROPERTY_VALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200175i32 as _);
pub const FSRM_E_OBJECT_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200199i32 as _);
pub const FSRM_E_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200243i32 as _);
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200169i32 as _);
pub const FSRM_E_PATH_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200252i32 as _);
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200129i32 as _);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200155i32 as _);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200166i32 as _);
pub const FSRM_E_PROPERTY_DELETED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200183i32 as _);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200138i32 as _);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200124i32 as _);
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200122i32 as _);
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200123i32 as _);
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200139i32 as _);
pub const FSRM_E_REPORT_GENERATION_ERR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200204i32 as _);
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200205i32 as _);
pub const FSRM_E_REPORT_TASK_TRIGGER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200203i32 as _);
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200206i32 as _);
pub const FSRM_E_REQD_PARAM_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200242i32 as _);
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200126i32 as _);
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200125i32 as _);
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200128i32 as _);
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200127i32 as _);
pub const FSRM_E_SET_PROPERTY_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200172i32 as _);
pub const FSRM_E_SHADOW_COPY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200212i32 as _);
pub const FSRM_E_STORE_NOT_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200209i32 as _);
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200119i32 as _);
pub const FSRM_E_SYNC_TASK_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200144i32 as _);
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200158i32 as _);
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200160i32 as _);
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200167i32 as _);
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200168i32 as _);
pub const FSRM_E_TEXTREADER_STREAM_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200159i32 as _);
pub const FSRM_E_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200234i32 as _);
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200188i32 as _);
pub const FSRM_E_VOLUME_OFFLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200154i32 as _);
pub const FSRM_E_VOLUME_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200235i32 as _);
pub const FSRM_E_WMI_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200121i32 as _);
pub const FSRM_E_XML_CORRUPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200211i32 as _);
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(283398i32 as _);
pub const FSRM_S_PARTIAL_BATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(283396i32 as _);
pub const FSRM_S_PARTIAL_CLASSIFICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(283397i32 as _);
pub const FsrmAccessDeniedRemediationClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 269176776,
    data2: 29889,
    data3: 18191,
    data4: [177, 183, 221, 123, 107, 174, 121, 189],
};
pub const FsrmAccountType_Unknown: i32 = 0i32;
pub const FsrmAccountType_NetworkService: i32 = 1i32;
pub const FsrmAccountType_LocalService: i32 = 2i32;
pub const FsrmAccountType_LocalSystem: i32 = 3i32;
pub const FsrmAccountType_InProc: i32 = 4i32;
pub const FsrmAccountType_External: i32 = 5i32;
pub const FsrmAccountType_Automatic: i32 = 500i32;
pub const FsrmActionType_Unknown: i32 = 0i32;
pub const FsrmActionType_EventLog: i32 = 1i32;
pub const FsrmActionType_Email: i32 = 2i32;
pub const FsrmActionType_Command: i32 = 3i32;
pub const FsrmActionType_Report: i32 = 4i32;
pub const FsrmClassificationLoggingFlags_None: i32 = 0i32;
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: i32 = 1i32;
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: i32 = 2i32;
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: i32 = 4i32;
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: i32 = 8i32;
pub const FsrmClassificationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2975600199,
    data2: 50065,
    data3: 17849,
    data4: [149, 200, 235, 89, 108, 133, 63, 58],
};
pub const FsrmCollectionState_Fetching: i32 = 1i32;
pub const FsrmCollectionState_Committing: i32 = 2i32;
pub const FsrmCollectionState_Complete: i32 = 3i32;
pub const FsrmCollectionState_Cancelled: i32 = 4i32;
pub const FsrmCommitOptions_None: i32 = 0i32;
pub const FsrmCommitOptions_Asynchronous: i32 = 1i32;
pub const FsrmDaysNotSpecified: i32 = -1i32;
pub const FsrmEnumOptions_None: i32 = 0i32;
pub const FsrmEnumOptions_Asynchronous: i32 = 1i32;
pub const FsrmEnumOptions_CheckRecycleBin: i32 = 2i32;
pub const FsrmEnumOptions_IncludeClusterNodes: i32 = 4i32;
pub const FsrmEnumOptions_IncludeDeprecatedObjects: i32 = 8i32;
pub const FsrmEventType_Unknown: i32 = 0i32;
pub const FsrmEventType_Information: i32 = 1i32;
pub const FsrmEventType_Warning: i32 = 2i32;
pub const FsrmEventType_Error: i32 = 3i32;
pub const FsrmExecutionOption_Unknown: i32 = 0i32;
pub const FsrmExecutionOption_EvaluateUnset: i32 = 1i32;
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: i32 = 2i32;
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: i32 = 3i32;
pub const FsrmExportImport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 344120375, data2: 64233, data3: 18311, data4: [144, 37, 140, 228, 224, 36, 171, 86] };
pub const FsrmFileConditionType_Unknown: i32 = 0i32;
pub const FsrmFileConditionType_Property: i32 = 1i32;
pub const FsrmFileGroupManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2400412662,
    data2: 25967,
    data3: 17558,
    data4: [146, 38, 19, 174, 203, 215, 113, 143],
};
pub const FsrmFileManagementJobManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3944282546, data2: 19514, data3: 17185, data4: [178, 3, 32, 81, 32, 207, 246, 20] };
pub const FsrmFileManagementLoggingFlags_None: i32 = 0i32;
pub const FsrmFileManagementLoggingFlags_Error: i32 = 1i32;
pub const FsrmFileManagementLoggingFlags_Information: i32 = 2i32;
pub const FsrmFileManagementLoggingFlags_Audit: i32 = 4i32;
pub const FsrmFileManagementType_Unknown: i32 = 0i32;
pub const FsrmFileManagementType_Expiration: i32 = 1i32;
pub const FsrmFileManagementType_Custom: i32 = 2i32;
pub const FsrmFileManagementType_Rms: i32 = 3i32;
pub const FsrmFileScreenFlags_Enforce: i32 = 1i32;
pub const FsrmFileScreenManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2509508995,
    data2: 56147,
    data3: 19551,
    data4: [179, 123, 125, 9, 33, 207, 157, 199],
};
pub const FsrmFileScreenTemplateManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 607195615, data2: 58484, data3: 18090, data4: [160, 84, 234, 163, 62, 220, 41, 42] };
pub const FsrmFileStreamingInterfaceType_Unknown: i32 = 0i32;
pub const FsrmFileStreamingInterfaceType_ILockBytes: i32 = 1i32;
pub const FsrmFileStreamingInterfaceType_IStream: i32 = 2i32;
pub const FsrmFileStreamingMode_Unknown: i32 = 0i32;
pub const FsrmFileStreamingMode_Read: i32 = 1i32;
pub const FsrmFileStreamingMode_Write: i32 = 2i32;
pub const FsrmFileSystemPropertyId_Undefined: i32 = 0i32;
pub const FsrmFileSystemPropertyId_FileName: i32 = 1i32;
pub const FsrmFileSystemPropertyId_DateCreated: i32 = 2i32;
pub const FsrmFileSystemPropertyId_DateLastAccessed: i32 = 3i32;
pub const FsrmFileSystemPropertyId_DateLastModified: i32 = 4i32;
pub const FsrmFileSystemPropertyId_DateNow: i32 = 5i32;
pub const FsrmGetFilePropertyOptions_None: i32 = 0i32;
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: i32 = 1i32;
pub const FsrmGetFilePropertyOptions_Persistent: i32 = 2i32;
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: i32 = 4i32;
pub const FsrmGetFilePropertyOptions_SkipOrphaned: i32 = 8i32;
pub const FsrmMaxExcludeFolders: u32 = 32u32;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
pub const FsrmMaxNumberThresholds: u32 = 16u32;
pub const FsrmMaxThresholdValue: u32 = 250u32;
pub const FsrmMinQuotaLimit: u32 = 1024u32;
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPathMapper: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4089332413,
    data2: 35522,
    data3: 16542,
    data4: [187, 216, 250, 249, 182, 180, 31, 235],
};
pub const FsrmPipelineModuleConnector: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3345232757, data2: 7861, data3: 17630, data4: [160, 98, 98, 53, 71, 217, 51, 188] };
pub const FsrmPipelineModuleType_Unknown: i32 = 0i32;
pub const FsrmPipelineModuleType_Storage: i32 = 1i32;
pub const FsrmPipelineModuleType_Classifier: i32 = 2i32;
pub const FsrmPropertyBagField_AccessVolume: i32 = 0i32;
pub const FsrmPropertyBagField_VolumeGuidName: i32 = 1i32;
pub const FsrmPropertyBagFlags_UpdatedByClassifier: i32 = 1i32;
pub const FsrmPropertyBagFlags_FailedLoadingProperties: i32 = 2i32;
pub const FsrmPropertyBagFlags_FailedSavingProperties: i32 = 4i32;
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: i32 = 8i32;
pub const FsrmPropertyConditionType_Unknown: i32 = 0i32;
pub const FsrmPropertyConditionType_Equal: i32 = 1i32;
pub const FsrmPropertyConditionType_NotEqual: i32 = 2i32;
pub const FsrmPropertyConditionType_GreaterThan: i32 = 3i32;
pub const FsrmPropertyConditionType_LessThan: i32 = 4i32;
pub const FsrmPropertyConditionType_Contain: i32 = 5i32;
pub const FsrmPropertyConditionType_Exist: i32 = 6i32;
pub const FsrmPropertyConditionType_NotExist: i32 = 7i32;
pub const FsrmPropertyConditionType_StartWith: i32 = 8i32;
pub const FsrmPropertyConditionType_EndWith: i32 = 9i32;
pub const FsrmPropertyConditionType_ContainedIn: i32 = 10i32;
pub const FsrmPropertyConditionType_PrefixOf: i32 = 11i32;
pub const FsrmPropertyConditionType_SuffixOf: i32 = 12i32;
pub const FsrmPropertyConditionType_MatchesPattern: i32 = 13i32;
pub const FsrmPropertyDefinitionAppliesTo_Files: i32 = 1i32;
pub const FsrmPropertyDefinitionAppliesTo_Folders: i32 = 2i32;
pub const FsrmPropertyDefinitionFlags_Global: i32 = 1i32;
pub const FsrmPropertyDefinitionFlags_Deprecated: i32 = 2i32;
pub const FsrmPropertyDefinitionFlags_Secure: i32 = 4i32;
pub const FsrmPropertyDefinitionType_Unknown: i32 = 0i32;
pub const FsrmPropertyDefinitionType_OrderedList: i32 = 1i32;
pub const FsrmPropertyDefinitionType_MultiChoiceList: i32 = 2i32;
pub const FsrmPropertyDefinitionType_SingleChoiceList: i32 = 3i32;
pub const FsrmPropertyDefinitionType_String: i32 = 4i32;
pub const FsrmPropertyDefinitionType_MultiString: i32 = 5i32;
pub const FsrmPropertyDefinitionType_Int: i32 = 6i32;
pub const FsrmPropertyDefinitionType_Bool: i32 = 7i32;
pub const FsrmPropertyDefinitionType_Date: i32 = 8i32;
pub const FsrmPropertyFlags_None: i32 = 0i32;
pub const FsrmPropertyFlags_Orphaned: i32 = 1i32;
pub const FsrmPropertyFlags_RetrievedFromCache: i32 = 2i32;
pub const FsrmPropertyFlags_RetrievedFromStorage: i32 = 4i32;
pub const FsrmPropertyFlags_SetByClassifier: i32 = 8i32;
pub const FsrmPropertyFlags_Deleted: i32 = 16i32;
pub const FsrmPropertyFlags_Reclassified: i32 = 32i32;
pub const FsrmPropertyFlags_AggregationFailed: i32 = 64i32;
pub const FsrmPropertyFlags_Existing: i32 = 128i32;
pub const FsrmPropertyFlags_FailedLoadingProperties: i32 = 256i32;
pub const FsrmPropertyFlags_FailedClassifyingProperties: i32 = 512i32;
pub const FsrmPropertyFlags_FailedSavingProperties: i32 = 1024i32;
pub const FsrmPropertyFlags_Secure: i32 = 2048i32;
pub const FsrmPropertyFlags_PolicyDerived: i32 = 4096i32;
pub const FsrmPropertyFlags_Inherited: i32 = 8192i32;
pub const FsrmPropertyFlags_Manual: i32 = 16384i32;
pub const FsrmPropertyFlags_ExplicitValueDeleted: i32 = 32768i32;
pub const FsrmPropertyFlags_PropertyDeletedFromClear: i32 = 65536i32;
pub const FsrmPropertyFlags_PropertySourceMask: i32 = 14i32;
pub const FsrmPropertyFlags_PersistentMask: i32 = 20480i32;
pub const FsrmPropertyValueType_Undefined: i32 = 0i32;
pub const FsrmPropertyValueType_Literal: i32 = 1i32;
pub const FsrmPropertyValueType_DateOffset: i32 = 2i32;
pub const FsrmQuotaFlags_Enforce: i32 = 256i32;
pub const FsrmQuotaFlags_Disable: i32 = 512i32;
pub const FsrmQuotaFlags_StatusIncomplete: i32 = 65536i32;
pub const FsrmQuotaFlags_StatusRebuilding: i32 = 131072i32;
pub const FsrmQuotaManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2430380927, data2: 13436, data3: 19452, data4: [181, 67, 84, 3, 38, 48, 95, 190] };
pub const FsrmQuotaTemplateManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2547242051, data2: 9500, data3: 17207, data4: [129, 231, 179, 46, 143, 78, 230, 94] };
pub const FsrmReportFilter_MinSize: i32 = 1i32;
pub const FsrmReportFilter_MinAgeDays: i32 = 2i32;
pub const FsrmReportFilter_MaxAgeDays: i32 = 3i32;
pub const FsrmReportFilter_MinQuotaUsage: i32 = 4i32;
pub const FsrmReportFilter_FileGroups: i32 = 5i32;
pub const FsrmReportFilter_Owners: i32 = 6i32;
pub const FsrmReportFilter_NamePattern: i32 = 7i32;
pub const FsrmReportFilter_Property: i32 = 8i32;
pub const FsrmReportFormat_Unknown: i32 = 0i32;
pub const FsrmReportFormat_DHtml: i32 = 1i32;
pub const FsrmReportFormat_Html: i32 = 2i32;
pub const FsrmReportFormat_Txt: i32 = 3i32;
pub const FsrmReportFormat_Csv: i32 = 4i32;
pub const FsrmReportFormat_Xml: i32 = 5i32;
pub const FsrmReportGenerationContext_Undefined: i32 = 1i32;
pub const FsrmReportGenerationContext_ScheduledReport: i32 = 2i32;
pub const FsrmReportGenerationContext_InteractiveReport: i32 = 3i32;
pub const FsrmReportGenerationContext_IncidentReport: i32 = 4i32;
pub const FsrmReportLimit_MaxFiles: i32 = 1i32;
pub const FsrmReportLimit_MaxFileGroups: i32 = 2i32;
pub const FsrmReportLimit_MaxOwners: i32 = 3i32;
pub const FsrmReportLimit_MaxFilesPerFileGroup: i32 = 4i32;
pub const FsrmReportLimit_MaxFilesPerOwner: i32 = 5i32;
pub const FsrmReportLimit_MaxFilesPerDuplGroup: i32 = 6i32;
pub const FsrmReportLimit_MaxDuplicateGroups: i32 = 7i32;
pub const FsrmReportLimit_MaxQuotas: i32 = 8i32;
pub const FsrmReportLimit_MaxFileScreenEvents: i32 = 9i32;
pub const FsrmReportLimit_MaxPropertyValues: i32 = 10i32;
pub const FsrmReportLimit_MaxFilesPerPropertyValue: i32 = 11i32;
pub const FsrmReportLimit_MaxFolders: i32 = 12i32;
pub const FsrmReportManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 5828407, data2: 43622, data3: 19528, data4: [189, 91, 47, 206, 67, 42, 176, 200] };
pub const FsrmReportRunningStatus_Unknown: i32 = 0i32;
pub const FsrmReportRunningStatus_NotRunning: i32 = 1i32;
pub const FsrmReportRunningStatus_Queued: i32 = 2i32;
pub const FsrmReportRunningStatus_Running: i32 = 3i32;
pub const FsrmReportScheduler: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3928355256,
    data2: 7053,
    data3: 17040,
    data4: [142, 232, 225, 124, 18, 194, 254, 32],
};
pub const FsrmReportType_Unknown: i32 = 0i32;
pub const FsrmReportType_LargeFiles: i32 = 1i32;
pub const FsrmReportType_FilesByType: i32 = 2i32;
pub const FsrmReportType_LeastRecentlyAccessed: i32 = 3i32;
pub const FsrmReportType_MostRecentlyAccessed: i32 = 4i32;
pub const FsrmReportType_QuotaUsage: i32 = 5i32;
pub const FsrmReportType_FilesByOwner: i32 = 6i32;
pub const FsrmReportType_ExportReport: i32 = 7i32;
pub const FsrmReportType_DuplicateFiles: i32 = 8i32;
pub const FsrmReportType_FileScreenAudit: i32 = 9i32;
pub const FsrmReportType_FilesByProperty: i32 = 10i32;
pub const FsrmReportType_AutomaticClassification: i32 = 11i32;
pub const FsrmReportType_Expiration: i32 = 12i32;
pub const FsrmReportType_FoldersByProperty: i32 = 13i32;
pub const FsrmRuleFlags_Disabled: i32 = 256i32;
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: i32 = 1024i32;
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: i32 = 2048i32;
pub const FsrmRuleFlags_Invalid: i32 = 4096i32;
pub const FsrmRuleType_Unknown: i32 = 0i32;
pub const FsrmRuleType_Classification: i32 = 1i32;
pub const FsrmRuleType_Generic: i32 = 2i32;
pub const FsrmSetting: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4116109064, data2: 27981, data3: 17812, data4: [156, 97, 125, 187, 13, 174, 42, 70] };
pub const FsrmStorageModuleCaps_Unknown: i32 = 0i32;
pub const FsrmStorageModuleCaps_CanGet: i32 = 1i32;
pub const FsrmStorageModuleCaps_CanSet: i32 = 2i32;
pub const FsrmStorageModuleCaps_CanHandleDirectories: i32 = 4i32;
pub const FsrmStorageModuleCaps_CanHandleFiles: i32 = 8i32;
pub const FsrmStorageModuleType_Unknown: i32 = 0i32;
pub const FsrmStorageModuleType_Cache: i32 = 1i32;
pub const FsrmStorageModuleType_InFile: i32 = 2i32;
pub const FsrmStorageModuleType_Database: i32 = 3i32;
pub const FsrmStorageModuleType_System: i32 = 100i32;
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: i32 = 1i32;
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: i32 = 2i32;
#[repr(transparent)]
pub struct IFsrmAccessDeniedRemediationClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmAccessDeniedRemediationClient {}
impl ::core::clone::Clone for IFsrmAccessDeniedRemediationClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmAction {}
impl ::core::clone::Clone for IFsrmAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmActionCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmActionCommand {}
impl ::core::clone::Clone for IFsrmActionCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmActionEmail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmActionEmail {}
impl ::core::clone::Clone for IFsrmActionEmail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmActionEmail2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmActionEmail2 {}
impl ::core::clone::Clone for IFsrmActionEmail2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmActionEventLog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmActionEventLog {}
impl ::core::clone::Clone for IFsrmActionEventLog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmActionReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmActionReport {}
impl ::core::clone::Clone for IFsrmActionReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmAutoApplyQuota(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmAutoApplyQuota {}
impl ::core::clone::Clone for IFsrmAutoApplyQuota {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmClassificationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmClassificationManager {}
impl ::core::clone::Clone for IFsrmClassificationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmClassificationManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmClassificationManager2 {}
impl ::core::clone::Clone for IFsrmClassificationManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmClassificationRule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmClassificationRule {}
impl ::core::clone::Clone for IFsrmClassificationRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmClassifierModuleDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmClassifierModuleDefinition {}
impl ::core::clone::Clone for IFsrmClassifierModuleDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmClassifierModuleImplementation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmClassifierModuleImplementation {}
impl ::core::clone::Clone for IFsrmClassifierModuleImplementation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmCollection {}
impl ::core::clone::Clone for IFsrmCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmCommittableCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmCommittableCollection {}
impl ::core::clone::Clone for IFsrmCommittableCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmDerivedObjectsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmDerivedObjectsResult {}
impl ::core::clone::Clone for IFsrmDerivedObjectsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmExportImport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmExportImport {}
impl ::core::clone::Clone for IFsrmExportImport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileCondition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileCondition {}
impl ::core::clone::Clone for IFsrmFileCondition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileConditionProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileConditionProperty {}
impl ::core::clone::Clone for IFsrmFileConditionProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileGroup {}
impl ::core::clone::Clone for IFsrmFileGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileGroupImported(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileGroupImported {}
impl ::core::clone::Clone for IFsrmFileGroupImported {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileGroupManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileGroupManager {}
impl ::core::clone::Clone for IFsrmFileGroupManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileManagementJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileManagementJob {}
impl ::core::clone::Clone for IFsrmFileManagementJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileManagementJobManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileManagementJobManager {}
impl ::core::clone::Clone for IFsrmFileManagementJobManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileScreen(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileScreen {}
impl ::core::clone::Clone for IFsrmFileScreen {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileScreenBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileScreenBase {}
impl ::core::clone::Clone for IFsrmFileScreenBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileScreenException(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileScreenException {}
impl ::core::clone::Clone for IFsrmFileScreenException {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileScreenManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileScreenManager {}
impl ::core::clone::Clone for IFsrmFileScreenManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileScreenTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileScreenTemplate {}
impl ::core::clone::Clone for IFsrmFileScreenTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateImported(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileScreenTemplateImported {}
impl ::core::clone::Clone for IFsrmFileScreenTemplateImported {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmFileScreenTemplateManager {}
impl ::core::clone::Clone for IFsrmFileScreenTemplateManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmMutableCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmMutableCollection {}
impl ::core::clone::Clone for IFsrmMutableCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmObject {}
impl ::core::clone::Clone for IFsrmObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPathMapper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPathMapper {}
impl ::core::clone::Clone for IFsrmPathMapper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPipelineModuleConnector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPipelineModuleConnector {}
impl ::core::clone::Clone for IFsrmPipelineModuleConnector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPipelineModuleDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPipelineModuleDefinition {}
impl ::core::clone::Clone for IFsrmPipelineModuleDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPipelineModuleImplementation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPipelineModuleImplementation {}
impl ::core::clone::Clone for IFsrmPipelineModuleImplementation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmProperty {}
impl ::core::clone::Clone for IFsrmProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPropertyBag(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPropertyBag {}
impl ::core::clone::Clone for IFsrmPropertyBag {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPropertyBag2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPropertyBag2 {}
impl ::core::clone::Clone for IFsrmPropertyBag2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPropertyCondition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPropertyCondition {}
impl ::core::clone::Clone for IFsrmPropertyCondition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPropertyDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPropertyDefinition {}
impl ::core::clone::Clone for IFsrmPropertyDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPropertyDefinition2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPropertyDefinition2 {}
impl ::core::clone::Clone for IFsrmPropertyDefinition2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmPropertyDefinitionValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmPropertyDefinitionValue {}
impl ::core::clone::Clone for IFsrmPropertyDefinitionValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmQuota(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmQuota {}
impl ::core::clone::Clone for IFsrmQuota {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmQuotaBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmQuotaBase {}
impl ::core::clone::Clone for IFsrmQuotaBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmQuotaManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmQuotaManager {}
impl ::core::clone::Clone for IFsrmQuotaManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmQuotaManagerEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmQuotaManagerEx {}
impl ::core::clone::Clone for IFsrmQuotaManagerEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmQuotaObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmQuotaObject {}
impl ::core::clone::Clone for IFsrmQuotaObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmQuotaTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmQuotaTemplate {}
impl ::core::clone::Clone for IFsrmQuotaTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmQuotaTemplateImported(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmQuotaTemplateImported {}
impl ::core::clone::Clone for IFsrmQuotaTemplateImported {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmQuotaTemplateManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmQuotaTemplateManager {}
impl ::core::clone::Clone for IFsrmQuotaTemplateManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmReport {}
impl ::core::clone::Clone for IFsrmReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmReportJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmReportJob {}
impl ::core::clone::Clone for IFsrmReportJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmReportManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmReportManager {}
impl ::core::clone::Clone for IFsrmReportManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmReportScheduler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmReportScheduler {}
impl ::core::clone::Clone for IFsrmReportScheduler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmRule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmRule {}
impl ::core::clone::Clone for IFsrmRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmSetting(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmSetting {}
impl ::core::clone::Clone for IFsrmSetting {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmStorageModuleDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmStorageModuleDefinition {}
impl ::core::clone::Clone for IFsrmStorageModuleDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmStorageModuleImplementation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsrmStorageModuleImplementation {}
impl ::core::clone::Clone for IFsrmStorageModuleImplementation {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MessageSizeLimit: u32 = 4096u32;
