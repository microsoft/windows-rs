#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const AdSyncTask: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 719734609, data2: 46888, data3: 19819, data4: [151, 160, 178, 218, 46, 125, 42, 59] };
#[repr(transparent)]
pub struct AdrClientDisplayFlags(pub i32);
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = AdrClientDisplayFlags(1i32);
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = AdrClientDisplayFlags(2i32);
impl ::core::marker::Copy for AdrClientDisplayFlags {}
impl ::core::clone::Clone for AdrClientDisplayFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdrClientErrorType(pub i32);
pub const AdrClientErrorType_Unknown: AdrClientErrorType = AdrClientErrorType(0i32);
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = AdrClientErrorType(1i32);
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = AdrClientErrorType(2i32);
impl ::core::marker::Copy for AdrClientErrorType {}
impl ::core::clone::Clone for AdrClientErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdrClientFlags(pub i32);
pub const AdrClientFlags_None: AdrClientFlags = AdrClientFlags(0i32);
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = AdrClientFlags(1i32);
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = AdrClientFlags(2i32);
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = AdrClientFlags(4i32);
impl ::core::marker::Copy for AdrClientFlags {}
impl ::core::clone::Clone for AdrClientFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdrEmailFlags(pub i32);
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = AdrEmailFlags(1i32);
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = AdrEmailFlags(2i32);
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = AdrEmailFlags(4i32);
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = AdrEmailFlags(8i32);
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = AdrEmailFlags(16i32);
impl ::core::marker::Copy for AdrEmailFlags {}
impl ::core::clone::Clone for AdrEmailFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DIFsrmClassificationEvents(pub *mut ::core::ffi::c_void);
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
pub const FsrmAccessDeniedRemediationClient: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 269176776,
    data2: 29889,
    data3: 18191,
    data4: [177, 183, 221, 123, 107, 174, 121, 189],
};
#[repr(transparent)]
pub struct FsrmAccountType(pub i32);
pub const FsrmAccountType_Unknown: FsrmAccountType = FsrmAccountType(0i32);
pub const FsrmAccountType_NetworkService: FsrmAccountType = FsrmAccountType(1i32);
pub const FsrmAccountType_LocalService: FsrmAccountType = FsrmAccountType(2i32);
pub const FsrmAccountType_LocalSystem: FsrmAccountType = FsrmAccountType(3i32);
pub const FsrmAccountType_InProc: FsrmAccountType = FsrmAccountType(4i32);
pub const FsrmAccountType_External: FsrmAccountType = FsrmAccountType(5i32);
pub const FsrmAccountType_Automatic: FsrmAccountType = FsrmAccountType(500i32);
impl ::core::marker::Copy for FsrmAccountType {}
impl ::core::clone::Clone for FsrmAccountType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmActionType(pub i32);
pub const FsrmActionType_Unknown: FsrmActionType = FsrmActionType(0i32);
pub const FsrmActionType_EventLog: FsrmActionType = FsrmActionType(1i32);
pub const FsrmActionType_Email: FsrmActionType = FsrmActionType(2i32);
pub const FsrmActionType_Command: FsrmActionType = FsrmActionType(3i32);
pub const FsrmActionType_Report: FsrmActionType = FsrmActionType(4i32);
impl ::core::marker::Copy for FsrmActionType {}
impl ::core::clone::Clone for FsrmActionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmClassificationLoggingFlags(pub i32);
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(0i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(1i32);
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(2i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(4i32);
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(8i32);
impl ::core::marker::Copy for FsrmClassificationLoggingFlags {}
impl ::core::clone::Clone for FsrmClassificationLoggingFlags {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmClassificationManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2975600199,
    data2: 50065,
    data3: 17849,
    data4: [149, 200, 235, 89, 108, 133, 63, 58],
};
#[repr(transparent)]
pub struct FsrmCollectionState(pub i32);
pub const FsrmCollectionState_Fetching: FsrmCollectionState = FsrmCollectionState(1i32);
pub const FsrmCollectionState_Committing: FsrmCollectionState = FsrmCollectionState(2i32);
pub const FsrmCollectionState_Complete: FsrmCollectionState = FsrmCollectionState(3i32);
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = FsrmCollectionState(4i32);
impl ::core::marker::Copy for FsrmCollectionState {}
impl ::core::clone::Clone for FsrmCollectionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmCommitOptions(pub i32);
pub const FsrmCommitOptions_None: FsrmCommitOptions = FsrmCommitOptions(0i32);
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = FsrmCommitOptions(1i32);
impl ::core::marker::Copy for FsrmCommitOptions {}
impl ::core::clone::Clone for FsrmCommitOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmDaysNotSpecified: i32 = -1i32;
#[repr(transparent)]
pub struct FsrmEnumOptions(pub i32);
pub const FsrmEnumOptions_None: FsrmEnumOptions = FsrmEnumOptions(0i32);
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = FsrmEnumOptions(1i32);
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = FsrmEnumOptions(2i32);
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = FsrmEnumOptions(4i32);
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = FsrmEnumOptions(8i32);
impl ::core::marker::Copy for FsrmEnumOptions {}
impl ::core::clone::Clone for FsrmEnumOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmEventType(pub i32);
pub const FsrmEventType_Unknown: FsrmEventType = FsrmEventType(0i32);
pub const FsrmEventType_Information: FsrmEventType = FsrmEventType(1i32);
pub const FsrmEventType_Warning: FsrmEventType = FsrmEventType(2i32);
pub const FsrmEventType_Error: FsrmEventType = FsrmEventType(3i32);
impl ::core::marker::Copy for FsrmEventType {}
impl ::core::clone::Clone for FsrmEventType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmExecutionOption(pub i32);
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = FsrmExecutionOption(0i32);
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = FsrmExecutionOption(1i32);
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = FsrmExecutionOption(2i32);
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = FsrmExecutionOption(3i32);
impl ::core::marker::Copy for FsrmExecutionOption {}
impl ::core::clone::Clone for FsrmExecutionOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmExportImport: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 344120375, data2: 64233, data3: 18311, data4: [144, 37, 140, 228, 224, 36, 171, 86] };
#[repr(transparent)]
pub struct FsrmFileConditionType(pub i32);
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = FsrmFileConditionType(0i32);
pub const FsrmFileConditionType_Property: FsrmFileConditionType = FsrmFileConditionType(1i32);
impl ::core::marker::Copy for FsrmFileConditionType {}
impl ::core::clone::Clone for FsrmFileConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmFileGroupManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2400412662,
    data2: 25967,
    data3: 17558,
    data4: [146, 38, 19, 174, 203, 215, 113, 143],
};
pub const FsrmFileManagementJobManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3944282546, data2: 19514, data3: 17185, data4: [178, 3, 32, 81, 32, 207, 246, 20] };
#[repr(transparent)]
pub struct FsrmFileManagementLoggingFlags(pub i32);
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(0i32);
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(1i32);
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(2i32);
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(4i32);
impl ::core::marker::Copy for FsrmFileManagementLoggingFlags {}
impl ::core::clone::Clone for FsrmFileManagementLoggingFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmFileManagementType(pub i32);
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = FsrmFileManagementType(0i32);
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = FsrmFileManagementType(1i32);
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = FsrmFileManagementType(2i32);
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = FsrmFileManagementType(3i32);
impl ::core::marker::Copy for FsrmFileManagementType {}
impl ::core::clone::Clone for FsrmFileManagementType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmFileScreenFlags(pub i32);
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = FsrmFileScreenFlags(1i32);
impl ::core::marker::Copy for FsrmFileScreenFlags {}
impl ::core::clone::Clone for FsrmFileScreenFlags {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmFileScreenManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2509508995,
    data2: 56147,
    data3: 19551,
    data4: [179, 123, 125, 9, 33, 207, 157, 199],
};
pub const FsrmFileScreenTemplateManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 607195615, data2: 58484, data3: 18090, data4: [160, 84, 234, 163, 62, 220, 41, 42] };
#[repr(transparent)]
pub struct FsrmFileStreamingInterfaceType(pub i32);
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(0i32);
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(1i32);
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(2i32);
impl ::core::marker::Copy for FsrmFileStreamingInterfaceType {}
impl ::core::clone::Clone for FsrmFileStreamingInterfaceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmFileStreamingMode(pub i32);
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = FsrmFileStreamingMode(0i32);
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = FsrmFileStreamingMode(1i32);
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = FsrmFileStreamingMode(2i32);
impl ::core::marker::Copy for FsrmFileStreamingMode {}
impl ::core::clone::Clone for FsrmFileStreamingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmFileSystemPropertyId(pub i32);
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(0i32);
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(1i32);
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(2i32);
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(3i32);
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(4i32);
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(5i32);
impl ::core::marker::Copy for FsrmFileSystemPropertyId {}
impl ::core::clone::Clone for FsrmFileSystemPropertyId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmGetFilePropertyOptions(pub i32);
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(0i32);
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(1i32);
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(2i32);
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(4i32);
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(8i32);
impl ::core::marker::Copy for FsrmGetFilePropertyOptions {}
impl ::core::clone::Clone for FsrmGetFilePropertyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmMaxExcludeFolders: u32 = 32u32;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
pub const FsrmMaxNumberThresholds: u32 = 16u32;
pub const FsrmMaxThresholdValue: u32 = 250u32;
pub const FsrmMinQuotaLimit: u32 = 1024u32;
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPathMapper: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4089332413,
    data2: 35522,
    data3: 16542,
    data4: [187, 216, 250, 249, 182, 180, 31, 235],
};
pub const FsrmPipelineModuleConnector: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3345232757, data2: 7861, data3: 17630, data4: [160, 98, 98, 53, 71, 217, 51, 188] };
#[repr(transparent)]
pub struct FsrmPipelineModuleType(pub i32);
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = FsrmPipelineModuleType(0i32);
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = FsrmPipelineModuleType(1i32);
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = FsrmPipelineModuleType(2i32);
impl ::core::marker::Copy for FsrmPipelineModuleType {}
impl ::core::clone::Clone for FsrmPipelineModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmPropertyBagField(pub i32);
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = FsrmPropertyBagField(0i32);
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = FsrmPropertyBagField(1i32);
impl ::core::marker::Copy for FsrmPropertyBagField {}
impl ::core::clone::Clone for FsrmPropertyBagField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmPropertyBagFlags(pub i32);
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = FsrmPropertyBagFlags(1i32);
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(2i32);
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(4i32);
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(8i32);
impl ::core::marker::Copy for FsrmPropertyBagFlags {}
impl ::core::clone::Clone for FsrmPropertyBagFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmPropertyConditionType(pub i32);
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = FsrmPropertyConditionType(0i32);
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = FsrmPropertyConditionType(1i32);
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = FsrmPropertyConditionType(2i32);
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = FsrmPropertyConditionType(3i32);
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = FsrmPropertyConditionType(4i32);
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = FsrmPropertyConditionType(5i32);
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = FsrmPropertyConditionType(6i32);
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = FsrmPropertyConditionType(7i32);
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = FsrmPropertyConditionType(8i32);
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = FsrmPropertyConditionType(9i32);
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = FsrmPropertyConditionType(10i32);
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(11i32);
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(12i32);
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = FsrmPropertyConditionType(13i32);
impl ::core::marker::Copy for FsrmPropertyConditionType {}
impl ::core::clone::Clone for FsrmPropertyConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmPropertyDefinitionAppliesTo(pub i32);
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(1i32);
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(2i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionAppliesTo {}
impl ::core::clone::Clone for FsrmPropertyDefinitionAppliesTo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmPropertyDefinitionFlags(pub i32);
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(1i32);
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(2i32);
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(4i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionFlags {}
impl ::core::clone::Clone for FsrmPropertyDefinitionFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmPropertyDefinitionType(pub i32);
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(0i32);
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(1i32);
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(2i32);
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(3i32);
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(4i32);
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(5i32);
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(6i32);
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(7i32);
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(8i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionType {}
impl ::core::clone::Clone for FsrmPropertyDefinitionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmPropertyFlags(pub i32);
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = FsrmPropertyFlags(0i32);
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = FsrmPropertyFlags(1i32);
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = FsrmPropertyFlags(2i32);
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = FsrmPropertyFlags(4i32);
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = FsrmPropertyFlags(8i32);
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = FsrmPropertyFlags(16i32);
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = FsrmPropertyFlags(32i32);
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = FsrmPropertyFlags(64i32);
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = FsrmPropertyFlags(128i32);
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = FsrmPropertyFlags(256i32);
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = FsrmPropertyFlags(512i32);
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = FsrmPropertyFlags(1024i32);
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = FsrmPropertyFlags(2048i32);
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = FsrmPropertyFlags(4096i32);
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = FsrmPropertyFlags(8192i32);
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = FsrmPropertyFlags(16384i32);
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = FsrmPropertyFlags(32768i32);
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = FsrmPropertyFlags(65536i32);
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = FsrmPropertyFlags(14i32);
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = FsrmPropertyFlags(20480i32);
impl ::core::marker::Copy for FsrmPropertyFlags {}
impl ::core::clone::Clone for FsrmPropertyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmPropertyValueType(pub i32);
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = FsrmPropertyValueType(0i32);
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = FsrmPropertyValueType(1i32);
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = FsrmPropertyValueType(2i32);
impl ::core::marker::Copy for FsrmPropertyValueType {}
impl ::core::clone::Clone for FsrmPropertyValueType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmQuotaFlags(pub i32);
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = FsrmQuotaFlags(256i32);
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = FsrmQuotaFlags(512i32);
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = FsrmQuotaFlags(65536i32);
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = FsrmQuotaFlags(131072i32);
impl ::core::marker::Copy for FsrmQuotaFlags {}
impl ::core::clone::Clone for FsrmQuotaFlags {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmQuotaManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2430380927, data2: 13436, data3: 19452, data4: [181, 67, 84, 3, 38, 48, 95, 190] };
pub const FsrmQuotaTemplateManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2547242051, data2: 9500, data3: 17207, data4: [129, 231, 179, 46, 143, 78, 230, 94] };
#[repr(transparent)]
pub struct FsrmReportFilter(pub i32);
pub const FsrmReportFilter_MinSize: FsrmReportFilter = FsrmReportFilter(1i32);
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = FsrmReportFilter(2i32);
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = FsrmReportFilter(3i32);
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = FsrmReportFilter(4i32);
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = FsrmReportFilter(5i32);
pub const FsrmReportFilter_Owners: FsrmReportFilter = FsrmReportFilter(6i32);
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = FsrmReportFilter(7i32);
pub const FsrmReportFilter_Property: FsrmReportFilter = FsrmReportFilter(8i32);
impl ::core::marker::Copy for FsrmReportFilter {}
impl ::core::clone::Clone for FsrmReportFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmReportFormat(pub i32);
pub const FsrmReportFormat_Unknown: FsrmReportFormat = FsrmReportFormat(0i32);
pub const FsrmReportFormat_DHtml: FsrmReportFormat = FsrmReportFormat(1i32);
pub const FsrmReportFormat_Html: FsrmReportFormat = FsrmReportFormat(2i32);
pub const FsrmReportFormat_Txt: FsrmReportFormat = FsrmReportFormat(3i32);
pub const FsrmReportFormat_Csv: FsrmReportFormat = FsrmReportFormat(4i32);
pub const FsrmReportFormat_Xml: FsrmReportFormat = FsrmReportFormat(5i32);
impl ::core::marker::Copy for FsrmReportFormat {}
impl ::core::clone::Clone for FsrmReportFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmReportGenerationContext(pub i32);
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = FsrmReportGenerationContext(1i32);
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = FsrmReportGenerationContext(2i32);
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = FsrmReportGenerationContext(3i32);
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = FsrmReportGenerationContext(4i32);
impl ::core::marker::Copy for FsrmReportGenerationContext {}
impl ::core::clone::Clone for FsrmReportGenerationContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmReportLimit(pub i32);
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = FsrmReportLimit(1i32);
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = FsrmReportLimit(2i32);
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = FsrmReportLimit(3i32);
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = FsrmReportLimit(4i32);
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = FsrmReportLimit(5i32);
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = FsrmReportLimit(6i32);
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = FsrmReportLimit(7i32);
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = FsrmReportLimit(8i32);
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = FsrmReportLimit(9i32);
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = FsrmReportLimit(10i32);
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = FsrmReportLimit(11i32);
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = FsrmReportLimit(12i32);
impl ::core::marker::Copy for FsrmReportLimit {}
impl ::core::clone::Clone for FsrmReportLimit {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmReportManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 5828407, data2: 43622, data3: 19528, data4: [189, 91, 47, 206, 67, 42, 176, 200] };
#[repr(transparent)]
pub struct FsrmReportRunningStatus(pub i32);
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = FsrmReportRunningStatus(0i32);
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = FsrmReportRunningStatus(1i32);
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = FsrmReportRunningStatus(2i32);
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = FsrmReportRunningStatus(3i32);
impl ::core::marker::Copy for FsrmReportRunningStatus {}
impl ::core::clone::Clone for FsrmReportRunningStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmReportScheduler: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3928355256,
    data2: 7053,
    data3: 17040,
    data4: [142, 232, 225, 124, 18, 194, 254, 32],
};
#[repr(transparent)]
pub struct FsrmReportType(pub i32);
pub const FsrmReportType_Unknown: FsrmReportType = FsrmReportType(0i32);
pub const FsrmReportType_LargeFiles: FsrmReportType = FsrmReportType(1i32);
pub const FsrmReportType_FilesByType: FsrmReportType = FsrmReportType(2i32);
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = FsrmReportType(3i32);
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = FsrmReportType(4i32);
pub const FsrmReportType_QuotaUsage: FsrmReportType = FsrmReportType(5i32);
pub const FsrmReportType_FilesByOwner: FsrmReportType = FsrmReportType(6i32);
pub const FsrmReportType_ExportReport: FsrmReportType = FsrmReportType(7i32);
pub const FsrmReportType_DuplicateFiles: FsrmReportType = FsrmReportType(8i32);
pub const FsrmReportType_FileScreenAudit: FsrmReportType = FsrmReportType(9i32);
pub const FsrmReportType_FilesByProperty: FsrmReportType = FsrmReportType(10i32);
pub const FsrmReportType_AutomaticClassification: FsrmReportType = FsrmReportType(11i32);
pub const FsrmReportType_Expiration: FsrmReportType = FsrmReportType(12i32);
pub const FsrmReportType_FoldersByProperty: FsrmReportType = FsrmReportType(13i32);
impl ::core::marker::Copy for FsrmReportType {}
impl ::core::clone::Clone for FsrmReportType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmRuleFlags(pub i32);
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = FsrmRuleFlags(256i32);
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(1024i32);
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(2048i32);
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = FsrmRuleFlags(4096i32);
impl ::core::marker::Copy for FsrmRuleFlags {}
impl ::core::clone::Clone for FsrmRuleFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmRuleType(pub i32);
pub const FsrmRuleType_Unknown: FsrmRuleType = FsrmRuleType(0i32);
pub const FsrmRuleType_Classification: FsrmRuleType = FsrmRuleType(1i32);
pub const FsrmRuleType_Generic: FsrmRuleType = FsrmRuleType(2i32);
impl ::core::marker::Copy for FsrmRuleType {}
impl ::core::clone::Clone for FsrmRuleType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsrmSetting: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4116109064, data2: 27981, data3: 17812, data4: [156, 97, 125, 187, 13, 174, 42, 70] };
#[repr(transparent)]
pub struct FsrmStorageModuleCaps(pub i32);
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = FsrmStorageModuleCaps(0i32);
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(1i32);
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(2i32);
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = FsrmStorageModuleCaps(4i32);
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = FsrmStorageModuleCaps(8i32);
impl ::core::marker::Copy for FsrmStorageModuleCaps {}
impl ::core::clone::Clone for FsrmStorageModuleCaps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmStorageModuleType(pub i32);
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = FsrmStorageModuleType(0i32);
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = FsrmStorageModuleType(1i32);
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = FsrmStorageModuleType(2i32);
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = FsrmStorageModuleType(3i32);
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = FsrmStorageModuleType(100i32);
impl ::core::marker::Copy for FsrmStorageModuleType {}
impl ::core::clone::Clone for FsrmStorageModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsrmTemplateApplyOptions(pub i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(1i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(2i32);
impl ::core::marker::Copy for FsrmTemplateApplyOptions {}
impl ::core::clone::Clone for FsrmTemplateApplyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsrmAccessDeniedRemediationClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmActionCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmActionEmail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmActionEmail2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmActionEventLog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmActionReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmAutoApplyQuota(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmClassificationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmClassificationManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmClassificationRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmClassifierModuleDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmClassifierModuleImplementation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmCommittableCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmDerivedObjectsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmExportImport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileCondition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileConditionProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileGroupImported(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileGroupManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileManagementJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileManagementJobManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileScreen(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileScreenBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileScreenException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileScreenManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileScreenTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateImported(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmMutableCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPathMapper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPipelineModuleConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPipelineModuleDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPipelineModuleImplementation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPropertyBag(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPropertyBag2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPropertyCondition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPropertyDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPropertyDefinition2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmPropertyDefinitionValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmQuota(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmQuotaBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmQuotaManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmQuotaManagerEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmQuotaObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmQuotaTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmQuotaTemplateImported(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmQuotaTemplateManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmReportJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmReportManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmReportScheduler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmSetting(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmStorageModuleDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFsrmStorageModuleImplementation(pub *mut ::core::ffi::c_void);
pub const MessageSizeLimit: u32 = 4096u32;
