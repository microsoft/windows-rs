#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AdSyncTask(i32);
pub struct AdrClientDisplayFlags(i32);
pub struct AdrClientErrorType(i32);
pub struct AdrClientFlags(i32);
pub struct AdrEmailFlags(i32);
pub struct DIFsrmClassificationEvents(i32);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_ADR_MAX_EMAILS_SENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200130i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200110i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_ADR_PATH_IS_LOCAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200111i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200112i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200253i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_AUTO_QUOTA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(283419i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CACHE_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200187i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200186i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_AGGREGATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200201i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200170i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200197i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200132i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200135i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_REMOVE_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200109i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_RENAME_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200198i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_STORE_PROPERTIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200171i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200143i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200145i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200195i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CLASSIFICATION_CANCELED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200141i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200194i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200136i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200148i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CLASSIFICATION_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200137i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CLUSTER_NOT_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200210i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200106i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200207i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_DRIVER_NOT_READY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200237i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_DUPLICATE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200240i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_EMAIL_NOT_SENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200228i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_ENUM_PROPERTIES_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200173i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_ERROR_NOT_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200133i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200105i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200104i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200103i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FAIL_BATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200247i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_ENCRYPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200156i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200134i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200152i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200153i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200185i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200184i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200193i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200191i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200102i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200192i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200108i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200146i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200190i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200147i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200120i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_OPEN_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200189i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_FILE_SYSTEM_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200225i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INCOMPATIBLE_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200157i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INPROC_MODULE_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200174i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INSECURE_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200233i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INSUFFICIENT_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200236i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_AD_CLAIM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200142i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_COMBINATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200241i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200220i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_EMAIL_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200226i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200223i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_FILENAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200214i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200140i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_IMPORT_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200245i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200249i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200248i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200250i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_REPORT_DESC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200215i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_REPORT_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200216i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200254i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_SMTP_SERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200232i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_TEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200246i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_INVALID_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200251i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200176i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_LEGACY_SCHEDULE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200107i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_LOADING_DISABLED_MODULE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200202i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_LONG_CMDLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200224i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200196i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200200i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_MODULE_INITIALIZATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200150i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_MODULE_INVALID_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200151i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200149i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_MODULE_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200101i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_NOT_CLUSTER_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200208i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200255i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200239i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_NO_EMAIL_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200131i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_NO_PROPERTY_VALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200175i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_OBJECT_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200199i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200243i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200169i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PATH_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200252i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200129i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200155i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200166i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PROPERTY_DELETED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200183i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200138i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200124i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200122i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200123i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200139i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_REPORT_GENERATION_ERR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200204i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200205i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_REPORT_TASK_TRIGGER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200203i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200206i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_REQD_PARAM_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200242i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200126i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200125i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200128i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200127i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_SET_PROPERTY_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200172i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_SHADOW_COPY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200212i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_STORE_NOT_INSTALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200209i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200119i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_SYNC_TASK_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200144i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200158i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200160i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200167i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200168i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_TEXTREADER_STREAM_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200159i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200234i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200188i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_VOLUME_OFFLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200154i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_VOLUME_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200235i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_WMI_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200121i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_E_XML_CORRUPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147200211i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(283398i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_S_PARTIAL_BATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(283396i32 as _);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FSRM_S_PARTIAL_CLASSIFICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(283397i32 as _);
pub struct FsrmAccessDeniedRemediationClient(i32);
pub struct FsrmAccountType(i32);
pub struct FsrmActionType(i32);
pub struct FsrmClassificationLoggingFlags(i32);
pub struct FsrmClassificationManager(i32);
pub struct FsrmCollectionState(i32);
pub struct FsrmCommitOptions(i32);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FsrmDaysNotSpecified: i32 = -1i32;
pub struct FsrmEnumOptions(i32);
pub struct FsrmEventType(i32);
pub struct FsrmExecutionOption(i32);
pub struct FsrmExportImport(i32);
pub struct FsrmFileConditionType(i32);
pub struct FsrmFileGroupManager(i32);
pub struct FsrmFileManagementJobManager(i32);
pub struct FsrmFileManagementLoggingFlags(i32);
pub struct FsrmFileManagementType(i32);
pub struct FsrmFileScreenFlags(i32);
pub struct FsrmFileScreenManager(i32);
pub struct FsrmFileScreenTemplateManager(i32);
pub struct FsrmFileStreamingInterfaceType(i32);
pub struct FsrmFileStreamingMode(i32);
pub struct FsrmFileSystemPropertyId(i32);
pub struct FsrmGetFilePropertyOptions(i32);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FsrmMaxExcludeFolders: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FsrmMaxNumberThresholds: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FsrmMaxThresholdValue: u32 = 250u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FsrmMinQuotaLimit: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const FsrmMinThresholdValue: u32 = 1u32;
pub struct FsrmPathMapper(i32);
pub struct FsrmPipelineModuleConnector(i32);
pub struct FsrmPipelineModuleType(i32);
pub struct FsrmPropertyBagField(i32);
pub struct FsrmPropertyBagFlags(i32);
pub struct FsrmPropertyConditionType(i32);
pub struct FsrmPropertyDefinitionAppliesTo(i32);
pub struct FsrmPropertyDefinitionFlags(i32);
pub struct FsrmPropertyDefinitionType(i32);
pub struct FsrmPropertyFlags(i32);
pub struct FsrmPropertyValueType(i32);
pub struct FsrmQuotaFlags(i32);
pub struct FsrmQuotaManager(i32);
pub struct FsrmQuotaTemplateManager(i32);
pub struct FsrmReportFilter(i32);
pub struct FsrmReportFormat(i32);
pub struct FsrmReportGenerationContext(i32);
pub struct FsrmReportLimit(i32);
pub struct FsrmReportManager(i32);
pub struct FsrmReportRunningStatus(i32);
pub struct FsrmReportScheduler(i32);
pub struct FsrmReportType(i32);
pub struct FsrmRuleFlags(i32);
pub struct FsrmRuleType(i32);
pub struct FsrmSetting(i32);
pub struct FsrmStorageModuleCaps(i32);
pub struct FsrmStorageModuleType(i32);
pub struct FsrmTemplateApplyOptions(i32);
pub struct IFsrmAccessDeniedRemediationClient(i32);
pub struct IFsrmAction(i32);
pub struct IFsrmActionCommand(i32);
pub struct IFsrmActionEmail(i32);
pub struct IFsrmActionEmail2(i32);
pub struct IFsrmActionEventLog(i32);
pub struct IFsrmActionReport(i32);
pub struct IFsrmAutoApplyQuota(i32);
pub struct IFsrmClassificationManager(i32);
pub struct IFsrmClassificationManager2(i32);
pub struct IFsrmClassificationRule(i32);
pub struct IFsrmClassifierModuleDefinition(i32);
pub struct IFsrmClassifierModuleImplementation(i32);
pub struct IFsrmCollection(i32);
pub struct IFsrmCommittableCollection(i32);
pub struct IFsrmDerivedObjectsResult(i32);
pub struct IFsrmExportImport(i32);
pub struct IFsrmFileCondition(i32);
pub struct IFsrmFileConditionProperty(i32);
pub struct IFsrmFileGroup(i32);
pub struct IFsrmFileGroupImported(i32);
pub struct IFsrmFileGroupManager(i32);
pub struct IFsrmFileManagementJob(i32);
pub struct IFsrmFileManagementJobManager(i32);
pub struct IFsrmFileScreen(i32);
pub struct IFsrmFileScreenBase(i32);
pub struct IFsrmFileScreenException(i32);
pub struct IFsrmFileScreenManager(i32);
pub struct IFsrmFileScreenTemplate(i32);
pub struct IFsrmFileScreenTemplateImported(i32);
pub struct IFsrmFileScreenTemplateManager(i32);
pub struct IFsrmMutableCollection(i32);
pub struct IFsrmObject(i32);
pub struct IFsrmPathMapper(i32);
pub struct IFsrmPipelineModuleConnector(i32);
pub struct IFsrmPipelineModuleDefinition(i32);
pub struct IFsrmPipelineModuleImplementation(i32);
pub struct IFsrmProperty(i32);
pub struct IFsrmPropertyBag(i32);
pub struct IFsrmPropertyBag2(i32);
pub struct IFsrmPropertyCondition(i32);
pub struct IFsrmPropertyDefinition(i32);
pub struct IFsrmPropertyDefinition2(i32);
pub struct IFsrmPropertyDefinitionValue(i32);
pub struct IFsrmQuota(i32);
pub struct IFsrmQuotaBase(i32);
pub struct IFsrmQuotaManager(i32);
pub struct IFsrmQuotaManagerEx(i32);
pub struct IFsrmQuotaObject(i32);
pub struct IFsrmQuotaTemplate(i32);
pub struct IFsrmQuotaTemplateImported(i32);
pub struct IFsrmQuotaTemplateManager(i32);
pub struct IFsrmReport(i32);
pub struct IFsrmReportJob(i32);
pub struct IFsrmReportManager(i32);
pub struct IFsrmReportScheduler(i32);
pub struct IFsrmRule(i32);
pub struct IFsrmSetting(i32);
pub struct IFsrmStorageModuleDefinition(i32);
pub struct IFsrmStorageModuleImplementation(i32);
#[doc = "*Required features: `Win32_Storage_FileServerResourceManager`*"]
pub const MessageSizeLimit: u32 = 4096u32;
