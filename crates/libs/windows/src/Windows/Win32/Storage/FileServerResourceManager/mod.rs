pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = 1i32;
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = 2i32;
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = 1i32;
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = 2i32;
pub const AdrClientErrorType_Unknown: AdrClientErrorType = 0i32;
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = 1i32;
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = 4i32;
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = 2i32;
pub const AdrClientFlags_None: AdrClientFlags = 0i32;
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = 16i32;
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = 4i32;
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = 8i32;
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = 2i32;
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = 1i32;
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
pub const FSRM_E_ADR_MAX_EMAILS_SENT: windows_core::HRESULT = 0x8004537E_u32 as _;
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: windows_core::HRESULT = 0x80045392_u32 as _;
pub const FSRM_E_ADR_PATH_IS_LOCAL: windows_core::HRESULT = 0x80045391_u32 as _;
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: windows_core::HRESULT = 0x80045390_u32 as _;
pub const FSRM_E_ALREADY_EXISTS: windows_core::HRESULT = 0x80045303_u32 as _;
pub const FSRM_E_AUTO_QUOTA: windows_core::HRESULT = 0x4531B_u32 as _;
pub const FSRM_E_CACHE_INVALID: windows_core::HRESULT = 0x80045345_u32 as _;
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: windows_core::HRESULT = 0x80045346_u32 as _;
pub const FSRM_E_CANNOT_AGGREGATE: windows_core::HRESULT = 0x80045337_u32 as _;
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: windows_core::HRESULT = 0x80045356_u32 as _;
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: windows_core::HRESULT = 0x8004533B_u32 as _;
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: windows_core::HRESULT = 0x8004537C_u32 as _;
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: windows_core::HRESULT = 0x80045379_u32 as _;
pub const FSRM_E_CANNOT_REMOVE_READONLY: windows_core::HRESULT = 0x80045393_u32 as _;
pub const FSRM_E_CANNOT_RENAME_PROPERTY: windows_core::HRESULT = 0x8004533A_u32 as _;
pub const FSRM_E_CANNOT_STORE_PROPERTIES: windows_core::HRESULT = 0x80045355_u32 as _;
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: windows_core::HRESULT = 0x80045371_u32 as _;
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: windows_core::HRESULT = 0x8004536F_u32 as _;
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: windows_core::HRESULT = 0x8004533D_u32 as _;
pub const FSRM_E_CLASSIFICATION_CANCELED: windows_core::HRESULT = 0x80045373_u32 as _;
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: windows_core::HRESULT = 0x8004533E_u32 as _;
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: windows_core::HRESULT = 0x80045378_u32 as _;
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: windows_core::HRESULT = 0x8004536C_u32 as _;
pub const FSRM_E_CLASSIFICATION_TIMEOUT: windows_core::HRESULT = 0x80045377_u32 as _;
pub const FSRM_E_CLUSTER_NOT_RUNNING: windows_core::HRESULT = 0x8004532E_u32 as _;
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: windows_core::HRESULT = 0x80045396_u32 as _;
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: windows_core::HRESULT = 0x80045331_u32 as _;
pub const FSRM_E_DRIVER_NOT_READY: windows_core::HRESULT = 0x80045313_u32 as _;
pub const FSRM_E_DUPLICATE_NAME: windows_core::HRESULT = 0x80045310_u32 as _;
pub const FSRM_E_EMAIL_NOT_SENT: windows_core::HRESULT = 0x8004531C_u32 as _;
pub const FSRM_E_ENUM_PROPERTIES_FAILED: windows_core::HRESULT = 0x80045353_u32 as _;
pub const FSRM_E_ERROR_NOT_ENABLED: windows_core::HRESULT = 0x8004537B_u32 as _;
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: windows_core::HRESULT = 0x80045397_u32 as _;
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: windows_core::HRESULT = 0x80045398_u32 as _;
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: windows_core::HRESULT = 0x80045399_u32 as _;
pub const FSRM_E_FAIL_BATCH: windows_core::HRESULT = 0x80045309_u32 as _;
pub const FSRM_E_FILE_ENCRYPTED: windows_core::HRESULT = 0x80045364_u32 as _;
pub const FSRM_E_FILE_IN_USE: windows_core::HRESULT = 0x8004537A_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: windows_core::HRESULT = 0x80045368_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: windows_core::HRESULT = 0x80045367_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: windows_core::HRESULT = 0x80045347_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: windows_core::HRESULT = 0x80045348_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: windows_core::HRESULT = 0x8004533F_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: windows_core::HRESULT = 0x80045341_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: windows_core::HRESULT = 0x8004539A_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: windows_core::HRESULT = 0x80045340_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: windows_core::HRESULT = 0x80045394_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: windows_core::HRESULT = 0x8004536E_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: windows_core::HRESULT = 0x80045342_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: windows_core::HRESULT = 0x8004536D_u32 as _;
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: windows_core::HRESULT = 0x80045388_u32 as _;
pub const FSRM_E_FILE_OPEN_ERROR: windows_core::HRESULT = 0x80045343_u32 as _;
pub const FSRM_E_FILE_SYSTEM_CORRUPT: windows_core::HRESULT = 0x8004531F_u32 as _;
pub const FSRM_E_INCOMPATIBLE_FORMAT: windows_core::HRESULT = 0x80045363_u32 as _;
pub const FSRM_E_INPROC_MODULE_BLOCKED: windows_core::HRESULT = 0x80045352_u32 as _;
pub const FSRM_E_INSECURE_PATH: windows_core::HRESULT = 0x80045317_u32 as _;
pub const FSRM_E_INSUFFICIENT_DISK: windows_core::HRESULT = 0x80045314_u32 as _;
pub const FSRM_E_INVALID_AD_CLAIM: windows_core::HRESULT = 0x80045372_u32 as _;
pub const FSRM_E_INVALID_COMBINATION: windows_core::HRESULT = 0x8004530F_u32 as _;
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: windows_core::HRESULT = 0x80045324_u32 as _;
pub const FSRM_E_INVALID_EMAIL_ADDRESS: windows_core::HRESULT = 0x8004531E_u32 as _;
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: windows_core::HRESULT = 0x80045321_u32 as _;
pub const FSRM_E_INVALID_FILENAME: windows_core::HRESULT = 0x8004532A_u32 as _;
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: windows_core::HRESULT = 0x80045374_u32 as _;
pub const FSRM_E_INVALID_IMPORT_VERSION: windows_core::HRESULT = 0x8004530B_u32 as _;
pub const FSRM_E_INVALID_LIMIT: windows_core::HRESULT = 0x80045307_u32 as _;
pub const FSRM_E_INVALID_NAME: windows_core::HRESULT = 0x80045308_u32 as _;
pub const FSRM_E_INVALID_PATH: windows_core::HRESULT = 0x80045306_u32 as _;
pub const FSRM_E_INVALID_REPORT_DESC: windows_core::HRESULT = 0x80045329_u32 as _;
pub const FSRM_E_INVALID_REPORT_FORMAT: windows_core::HRESULT = 0x80045328_u32 as _;
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: windows_core::HRESULT = 0x80045302_u32 as _;
pub const FSRM_E_INVALID_SMTP_SERVER: windows_core::HRESULT = 0x80045318_u32 as _;
pub const FSRM_E_INVALID_TEXT: windows_core::HRESULT = 0x8004530A_u32 as _;
pub const FSRM_E_INVALID_USER: windows_core::HRESULT = 0x80045305_u32 as _;
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: windows_core::HRESULT = 0x80045350_u32 as _;
pub const FSRM_E_LEGACY_SCHEDULE: windows_core::HRESULT = 0x80045395_u32 as _;
pub const FSRM_E_LOADING_DISABLED_MODULE: windows_core::HRESULT = 0x80045336_u32 as _;
pub const FSRM_E_LONG_CMDLINE: windows_core::HRESULT = 0x80045320_u32 as _;
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: windows_core::HRESULT = 0x8004533C_u32 as _;
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: windows_core::HRESULT = 0x80045338_u32 as _;
pub const FSRM_E_MODULE_INITIALIZATION: windows_core::HRESULT = 0x8004536A_u32 as _;
pub const FSRM_E_MODULE_INVALID_PARAM: windows_core::HRESULT = 0x80045369_u32 as _;
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: windows_core::HRESULT = 0x8004536B_u32 as _;
pub const FSRM_E_MODULE_TIMEOUT: windows_core::HRESULT = 0x8004539B_u32 as _;
pub const FSRM_E_NOT_CLUSTER_VOLUME: windows_core::HRESULT = 0x80045330_u32 as _;
pub const FSRM_E_NOT_FOUND: windows_core::HRESULT = 0x80045301_u32 as _;
pub const FSRM_E_NOT_SUPPORTED: windows_core::HRESULT = 0x80045311_u32 as _;
pub const FSRM_E_NO_EMAIL_ADDRESS: windows_core::HRESULT = 0x8004537D_u32 as _;
pub const FSRM_E_NO_PROPERTY_VALUE: windows_core::HRESULT = 0x80045351_u32 as _;
pub const FSRM_E_OBJECT_IN_USE: windows_core::HRESULT = 0x80045339_u32 as _;
pub const FSRM_E_OUT_OF_RANGE: windows_core::HRESULT = 0x8004530D_u32 as _;
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: windows_core::HRESULT = 0x80045357_u32 as _;
pub const FSRM_E_PATH_NOT_FOUND: windows_core::HRESULT = 0x80045304_u32 as _;
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: windows_core::HRESULT = 0x8004537F_u32 as _;
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: windows_core::HRESULT = 0x80045365_u32 as _;
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: windows_core::HRESULT = 0x8004535A_u32 as _;
pub const FSRM_E_PROPERTY_DELETED: windows_core::HRESULT = 0x80045349_u32 as _;
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: windows_core::HRESULT = 0x80045376_u32 as _;
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: windows_core::HRESULT = 0x80045384_u32 as _;
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: windows_core::HRESULT = 0x80045386_u32 as _;
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: windows_core::HRESULT = 0x80045385_u32 as _;
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: windows_core::HRESULT = 0x80045375_u32 as _;
pub const FSRM_E_REPORT_GENERATION_ERR: windows_core::HRESULT = 0x80045334_u32 as _;
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: windows_core::HRESULT = 0x80045333_u32 as _;
pub const FSRM_E_REPORT_TASK_TRIGGER: windows_core::HRESULT = 0x80045335_u32 as _;
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: windows_core::HRESULT = 0x80045332_u32 as _;
pub const FSRM_E_REQD_PARAM_MISSING: windows_core::HRESULT = 0x8004530E_u32 as _;
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: windows_core::HRESULT = 0x80045382_u32 as _;
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: windows_core::HRESULT = 0x80045383_u32 as _;
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: windows_core::HRESULT = 0x80045380_u32 as _;
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: windows_core::HRESULT = 0x80045381_u32 as _;
pub const FSRM_E_SET_PROPERTY_FAILED: windows_core::HRESULT = 0x80045354_u32 as _;
pub const FSRM_E_SHADOW_COPY: windows_core::HRESULT = 0x8004532C_u32 as _;
pub const FSRM_E_STORE_NOT_INSTALLED: windows_core::HRESULT = 0x8004532F_u32 as _;
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: windows_core::HRESULT = 0x80045389_u32 as _;
pub const FSRM_E_SYNC_TASK_TIMEOUT: windows_core::HRESULT = 0x80045370_u32 as _;
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: windows_core::HRESULT = 0x80045362_u32 as _;
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: windows_core::HRESULT = 0x80045360_u32 as _;
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: windows_core::HRESULT = 0x80045359_u32 as _;
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: windows_core::HRESULT = 0x80045358_u32 as _;
pub const FSRM_E_TEXTREADER_STREAM_ERROR: windows_core::HRESULT = 0x80045361_u32 as _;
pub const FSRM_E_UNEXPECTED: windows_core::HRESULT = 0x80045316_u32 as _;
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: windows_core::HRESULT = 0x80045344_u32 as _;
pub const FSRM_E_VOLUME_OFFLINE: windows_core::HRESULT = 0x80045366_u32 as _;
pub const FSRM_E_VOLUME_UNSUPPORTED: windows_core::HRESULT = 0x80045315_u32 as _;
pub const FSRM_E_WMI_FAILURE: windows_core::HRESULT = 0x80045387_u32 as _;
pub const FSRM_E_XML_CORRUPTED: windows_core::HRESULT = 0x8004532D_u32 as _;
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: windows_core::HRESULT = 0x45306_u32 as _;
pub const FSRM_S_PARTIAL_BATCH: windows_core::HRESULT = 0x45304_u32 as _;
pub const FSRM_S_PARTIAL_CLASSIFICATION: windows_core::HRESULT = 0x45305_u32 as _;
pub const FsrmAccountType_Automatic: FsrmAccountType = 500i32;
pub const FsrmAccountType_External: FsrmAccountType = 5i32;
pub const FsrmAccountType_InProc: FsrmAccountType = 4i32;
pub const FsrmAccountType_LocalService: FsrmAccountType = 2i32;
pub const FsrmAccountType_LocalSystem: FsrmAccountType = 3i32;
pub const FsrmAccountType_NetworkService: FsrmAccountType = 1i32;
pub const FsrmAccountType_Unknown: FsrmAccountType = 0i32;
pub const FsrmActionType_Command: FsrmActionType = 3i32;
pub const FsrmActionType_Email: FsrmActionType = 2i32;
pub const FsrmActionType_EventLog: FsrmActionType = 1i32;
pub const FsrmActionType_Report: FsrmActionType = 4i32;
pub const FsrmActionType_Unknown: FsrmActionType = 0i32;
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = 1i32;
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = 4i32;
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = 2i32;
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = 8i32;
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = 0i32;
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = 4i32;
pub const FsrmCollectionState_Committing: FsrmCollectionState = 2i32;
pub const FsrmCollectionState_Complete: FsrmCollectionState = 3i32;
pub const FsrmCollectionState_Fetching: FsrmCollectionState = 1i32;
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = 1i32;
pub const FsrmCommitOptions_None: FsrmCommitOptions = 0i32;
pub const FsrmDaysNotSpecified: i32 = -1i32;
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = 1i32;
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = 2i32;
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = 4i32;
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = 8i32;
pub const FsrmEnumOptions_None: FsrmEnumOptions = 0i32;
pub const FsrmEventType_Error: FsrmEventType = 3i32;
pub const FsrmEventType_Information: FsrmEventType = 1i32;
pub const FsrmEventType_Unknown: FsrmEventType = 0i32;
pub const FsrmEventType_Warning: FsrmEventType = 2i32;
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = 1i32;
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = 2i32;
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = 3i32;
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = 0i32;
pub const FsrmFileConditionType_Property: FsrmFileConditionType = 1i32;
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = 0i32;
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = 4i32;
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = 1i32;
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = 2i32;
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = 0i32;
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = 2i32;
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = 1i32;
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = 3i32;
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = 0i32;
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = 1i32;
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = 1i32;
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = 2i32;
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = 0i32;
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = 1i32;
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = 0i32;
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = 2i32;
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = 2i32;
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = 3i32;
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = 4i32;
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = 5i32;
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = 1i32;
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = 0i32;
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = 4i32;
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = 1i32;
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = 0i32;
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = 2i32;
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = 8i32;
pub const FsrmMaxExcludeFolders: u32 = 32u32;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
pub const FsrmMaxNumberThresholds: u32 = 16u32;
pub const FsrmMaxThresholdValue: u32 = 250u32;
pub const FsrmMinQuotaLimit: u32 = 1024u32;
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = 2i32;
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = 1i32;
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = 0i32;
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = 0i32;
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = 1i32;
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = 8i32;
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = 2i32;
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = 4i32;
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = 1i32;
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = 5i32;
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = 10i32;
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = 9i32;
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = 1i32;
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = 6i32;
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = 3i32;
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = 4i32;
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = 13i32;
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = 2i32;
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = 7i32;
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = 11i32;
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = 8i32;
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = 12i32;
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = 0i32;
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = 1i32;
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = 2i32;
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = 2i32;
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = 1i32;
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = 4i32;
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = 7i32;
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = 8i32;
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = 6i32;
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = 2i32;
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = 5i32;
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = 1i32;
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = 3i32;
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = 4i32;
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = 0i32;
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = 64i32;
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = 16i32;
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = 128i32;
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = 32768i32;
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = 512i32;
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = 256i32;
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = 1024i32;
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = 8192i32;
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = 16384i32;
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = 0i32;
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = 1i32;
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = 20480i32;
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = 4096i32;
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = 65536i32;
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = 14i32;
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = 32i32;
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = 2i32;
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = 4i32;
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = 2048i32;
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = 8i32;
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = 2i32;
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = 1i32;
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = 0i32;
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = 512i32;
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = 256i32;
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = 65536i32;
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = 131072i32;
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = 5i32;
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = 3i32;
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = 2i32;
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = 4i32;
pub const FsrmReportFilter_MinSize: FsrmReportFilter = 1i32;
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = 7i32;
pub const FsrmReportFilter_Owners: FsrmReportFilter = 6i32;
pub const FsrmReportFilter_Property: FsrmReportFilter = 8i32;
pub const FsrmReportFormat_Csv: FsrmReportFormat = 4i32;
pub const FsrmReportFormat_DHtml: FsrmReportFormat = 1i32;
pub const FsrmReportFormat_Html: FsrmReportFormat = 2i32;
pub const FsrmReportFormat_Txt: FsrmReportFormat = 3i32;
pub const FsrmReportFormat_Unknown: FsrmReportFormat = 0i32;
pub const FsrmReportFormat_Xml: FsrmReportFormat = 5i32;
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = 4i32;
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = 3i32;
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = 2i32;
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = 1i32;
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = 7i32;
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = 2i32;
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = 9i32;
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = 1i32;
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = 6i32;
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = 4i32;
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = 5i32;
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = 11i32;
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = 12i32;
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = 3i32;
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = 10i32;
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = 8i32;
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = 1i32;
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = 2i32;
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = 3i32;
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = 0i32;
pub const FsrmReportType_AutomaticClassification: FsrmReportType = 11i32;
pub const FsrmReportType_DuplicateFiles: FsrmReportType = 8i32;
pub const FsrmReportType_Expiration: FsrmReportType = 12i32;
pub const FsrmReportType_ExportReport: FsrmReportType = 7i32;
pub const FsrmReportType_FileScreenAudit: FsrmReportType = 9i32;
pub const FsrmReportType_FilesByOwner: FsrmReportType = 6i32;
pub const FsrmReportType_FilesByProperty: FsrmReportType = 10i32;
pub const FsrmReportType_FilesByType: FsrmReportType = 2i32;
pub const FsrmReportType_FoldersByProperty: FsrmReportType = 13i32;
pub const FsrmReportType_LargeFiles: FsrmReportType = 1i32;
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = 3i32;
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = 4i32;
pub const FsrmReportType_QuotaUsage: FsrmReportType = 5i32;
pub const FsrmReportType_Unknown: FsrmReportType = 0i32;
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = 1024i32;
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = 2048i32;
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = 256i32;
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = 4096i32;
pub const FsrmRuleType_Classification: FsrmRuleType = 1i32;
pub const FsrmRuleType_Generic: FsrmRuleType = 2i32;
pub const FsrmRuleType_Unknown: FsrmRuleType = 0i32;
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = 1i32;
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = 4i32;
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = 8i32;
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = 2i32;
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = 0i32;
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = 1i32;
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = 3i32;
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = 2i32;
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = 100i32;
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = 0i32;
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = 2i32;
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = 1i32;
pub const MessageSizeLimit: u32 = 4096u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdrClientDisplayFlags(pub i32);
impl windows_core::TypeKind for AdrClientDisplayFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdrClientErrorType(pub i32);
impl windows_core::TypeKind for AdrClientErrorType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdrClientFlags(pub i32);
impl windows_core::TypeKind for AdrClientFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdrEmailFlags(pub i32);
impl windows_core::TypeKind for AdrEmailFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmAccountType(pub i32);
impl windows_core::TypeKind for FsrmAccountType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmActionType(pub i32);
impl windows_core::TypeKind for FsrmActionType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmClassificationLoggingFlags(pub i32);
impl windows_core::TypeKind for FsrmClassificationLoggingFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmCollectionState(pub i32);
impl windows_core::TypeKind for FsrmCollectionState {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmCommitOptions(pub i32);
impl windows_core::TypeKind for FsrmCommitOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmEnumOptions(pub i32);
impl windows_core::TypeKind for FsrmEnumOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmEventType(pub i32);
impl windows_core::TypeKind for FsrmEventType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmExecutionOption(pub i32);
impl windows_core::TypeKind for FsrmExecutionOption {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmFileConditionType(pub i32);
impl windows_core::TypeKind for FsrmFileConditionType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmFileManagementLoggingFlags(pub i32);
impl windows_core::TypeKind for FsrmFileManagementLoggingFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmFileManagementType(pub i32);
impl windows_core::TypeKind for FsrmFileManagementType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmFileScreenFlags(pub i32);
impl windows_core::TypeKind for FsrmFileScreenFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmFileStreamingInterfaceType(pub i32);
impl windows_core::TypeKind for FsrmFileStreamingInterfaceType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmFileStreamingMode(pub i32);
impl windows_core::TypeKind for FsrmFileStreamingMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmFileSystemPropertyId(pub i32);
impl windows_core::TypeKind for FsrmFileSystemPropertyId {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmGetFilePropertyOptions(pub i32);
impl windows_core::TypeKind for FsrmGetFilePropertyOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPipelineModuleType(pub i32);
impl windows_core::TypeKind for FsrmPipelineModuleType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPropertyBagField(pub i32);
impl windows_core::TypeKind for FsrmPropertyBagField {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPropertyBagFlags(pub i32);
impl windows_core::TypeKind for FsrmPropertyBagFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPropertyConditionType(pub i32);
impl windows_core::TypeKind for FsrmPropertyConditionType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPropertyDefinitionAppliesTo(pub i32);
impl windows_core::TypeKind for FsrmPropertyDefinitionAppliesTo {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPropertyDefinitionFlags(pub i32);
impl windows_core::TypeKind for FsrmPropertyDefinitionFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPropertyDefinitionType(pub i32);
impl windows_core::TypeKind for FsrmPropertyDefinitionType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPropertyFlags(pub i32);
impl windows_core::TypeKind for FsrmPropertyFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmPropertyValueType(pub i32);
impl windows_core::TypeKind for FsrmPropertyValueType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmQuotaFlags(pub i32);
impl windows_core::TypeKind for FsrmQuotaFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmReportFilter(pub i32);
impl windows_core::TypeKind for FsrmReportFilter {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmReportFormat(pub i32);
impl windows_core::TypeKind for FsrmReportFormat {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmReportGenerationContext(pub i32);
impl windows_core::TypeKind for FsrmReportGenerationContext {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmReportLimit(pub i32);
impl windows_core::TypeKind for FsrmReportLimit {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmReportRunningStatus(pub i32);
impl windows_core::TypeKind for FsrmReportRunningStatus {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmReportType(pub i32);
impl windows_core::TypeKind for FsrmReportType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmRuleFlags(pub i32);
impl windows_core::TypeKind for FsrmRuleFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmRuleType(pub i32);
impl windows_core::TypeKind for FsrmRuleType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmStorageModuleCaps(pub i32);
impl windows_core::TypeKind for FsrmStorageModuleCaps {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmStorageModuleType(pub i32);
impl windows_core::TypeKind for FsrmStorageModuleType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FsrmTemplateApplyOptions(pub i32);
impl windows_core::TypeKind for FsrmTemplateApplyOptions {
    type TypeKind = windows_core::CopyType;
}
pub const AdSyncTask: windows_core::GUID = windows_core::GUID::from_u128(0x2ae64751_b728_4d6b_97a0_b2da2e7d2a3b);
pub const FsrmAccessDeniedRemediationClient: windows_core::GUID = windows_core::GUID::from_u128(0x100b4fc8_74c1_470f_b1b7_dd7b6bae79bd);
pub const FsrmClassificationManager: windows_core::GUID = windows_core::GUID::from_u128(0xb15c0e47_c391_45b9_95c8_eb596c853f3a);
pub const FsrmExportImport: windows_core::GUID = windows_core::GUID::from_u128(0x1482dc37_fae9_4787_9025_8ce4e024ab56);
pub const FsrmFileGroupManager: windows_core::GUID = windows_core::GUID::from_u128(0x8f1363f6_656f_4496_9226_13aecbd7718f);
pub const FsrmFileManagementJobManager: windows_core::GUID = windows_core::GUID::from_u128(0xeb18f9b2_4c3a_4321_b203_205120cff614);
pub const FsrmFileScreenManager: windows_core::GUID = windows_core::GUID::from_u128(0x95941183_db53_4c5f_b37b_7d0921cf9dc7);
pub const FsrmFileScreenTemplateManager: windows_core::GUID = windows_core::GUID::from_u128(0x243111df_e474_46aa_a054_eaa33edc292a);
pub const FsrmPathMapper: windows_core::GUID = windows_core::GUID::from_u128(0xf3be42bd_8ac2_409e_bbd8_faf9b6b41feb);
pub const FsrmPipelineModuleConnector: windows_core::GUID = windows_core::GUID::from_u128(0xc7643375_1eb5_44de_a062_623547d933bc);
pub const FsrmQuotaManager: windows_core::GUID = windows_core::GUID::from_u128(0x90dcab7f_347c_4bfc_b543_540326305fbe);
pub const FsrmQuotaTemplateManager: windows_core::GUID = windows_core::GUID::from_u128(0x97d3d443_251c_4337_81e7_b32e8f4ee65e);
pub const FsrmReportManager: windows_core::GUID = windows_core::GUID::from_u128(0x0058ef37_aa66_4c48_bd5b_2fce432ab0c8);
pub const FsrmReportScheduler: windows_core::GUID = windows_core::GUID::from_u128(0xea25f1b8_1b8d_4290_8ee8_e17c12c2fe20);
pub const FsrmSetting: windows_core::GUID = windows_core::GUID::from_u128(0xf556d708_6d4d_4594_9c61_7dbb0dae2a46);
