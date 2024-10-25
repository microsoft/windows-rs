pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = -1i32;
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = 2i32;
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = 3i32;
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = 1i32;
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = 4i32;
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = 0i32;
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE = 2i32;
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = 1i32;
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = 3i32;
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = 0i32;
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = 2i32;
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = 1i32;
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = 4i32;
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = 8i32;
pub const VSS_BS_AUTHORITATIVE_RESTORE: VSS_BACKUP_SCHEMA = 16384i32;
pub const VSS_BS_COPY: VSS_BACKUP_SCHEMA = 16i32;
pub const VSS_BS_DIFFERENTIAL: VSS_BACKUP_SCHEMA = 1i32;
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: VSS_BACKUP_SCHEMA = 4i32;
pub const VSS_BS_INCREMENTAL: VSS_BACKUP_SCHEMA = 2i32;
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: VSS_BACKUP_SCHEMA = 1024i32;
pub const VSS_BS_LAST_MODIFY: VSS_BACKUP_SCHEMA = 64i32;
pub const VSS_BS_LOG: VSS_BACKUP_SCHEMA = 8i32;
pub const VSS_BS_LSN: VSS_BACKUP_SCHEMA = 128i32;
pub const VSS_BS_RESTORE_RENAME: VSS_BACKUP_SCHEMA = 8192i32;
pub const VSS_BS_ROLLFORWARD_RESTORE: VSS_BACKUP_SCHEMA = 4096i32;
pub const VSS_BS_TIMESTAMPED: VSS_BACKUP_SCHEMA = 32i32;
pub const VSS_BS_UNDEFINED: VSS_BACKUP_SCHEMA = 0i32;
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: VSS_BACKUP_SCHEMA = 256i32;
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: VSS_BACKUP_SCHEMA = 32768i32;
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: VSS_BACKUP_SCHEMA = 512i32;
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = 5i32;
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = 3i32;
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = 1i32;
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = 2i32;
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = 4i32;
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = 6i32;
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = 0i32;
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = 2i32;
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = 1i32;
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = 4i32;
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = -1i32;
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 9i32;
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = 0i32;
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = 29i32;
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = 13i32;
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = 16i32;
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 25i32;
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = 1i32;
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = 2i32;
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = 0i32;
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: windows_core::HRESULT = 0x80042408_u32 as _;
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: windows_core::HRESULT = 0x80042415_u32 as _;
pub const VSS_E_ASRERROR_DATADISK_RDISK0: windows_core::HRESULT = 0x80042406_u32 as _;
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: windows_core::HRESULT = 0x80042401_u32 as _;
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: windows_core::HRESULT = 0x80042402_u32 as _;
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: windows_core::HRESULT = 0x8004240A_u32 as _;
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: windows_core::HRESULT = 0x80042414_u32 as _;
pub const VSS_E_ASRERROR_MISSING_DYNDISK: windows_core::HRESULT = 0x80042404_u32 as _;
pub const VSS_E_ASRERROR_NO_ARCPATH: windows_core::HRESULT = 0x80042403_u32 as _;
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: windows_core::HRESULT = 0x80042413_u32 as _;
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: windows_core::HRESULT = 0x80042407_u32 as _;
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: windows_core::HRESULT = 0x80042412_u32 as _;
pub const VSS_E_ASRERROR_SHARED_CRIDISK: windows_core::HRESULT = 0x80042405_u32 as _;
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: windows_core::HRESULT = 0x80042416_u32 as _;
pub const VSS_E_AUTORECOVERY_FAILED: windows_core::HRESULT = 0x800423FB_u32 as _;
pub const VSS_E_BAD_STATE: windows_core::HRESULT = 0x80042301_u32 as _;
pub const VSS_E_BREAK_REVERT_ID_FAILED: windows_core::HRESULT = 0x800423F6_u32 as _;
pub const VSS_E_CANNOT_REVERT_DISKID: windows_core::HRESULT = 0x800423FE_u32 as _;
pub const VSS_E_CLUSTER_ERROR: windows_core::HRESULT = 0x80042400_u32 as _;
pub const VSS_E_CLUSTER_TIMEOUT: windows_core::HRESULT = 0x8004232E_u32 as _;
pub const VSS_E_CORRUPT_XML_DOCUMENT: windows_core::HRESULT = 0x80042310_u32 as _;
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: windows_core::HRESULT = 0x80042411_u32 as _;
pub const VSS_E_DYNAMIC_DISK_ERROR: windows_core::HRESULT = 0x800423FC_u32 as _;
pub const VSS_E_FLUSH_WRITES_TIMEOUT: windows_core::HRESULT = 0x80042313_u32 as _;
pub const VSS_E_FSS_TIMEOUT: windows_core::HRESULT = 0x80042417_u32 as _;
pub const VSS_E_HOLD_WRITES_TIMEOUT: windows_core::HRESULT = 0x80042314_u32 as _;
pub const VSS_E_INSUFFICIENT_STORAGE: windows_core::HRESULT = 0x8004231F_u32 as _;
pub const VSS_E_INVALID_XML_DOCUMENT: windows_core::HRESULT = 0x80042311_u32 as _;
pub const VSS_E_LEGACY_PROVIDER: windows_core::HRESULT = 0x800423F7_u32 as _;
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: windows_core::HRESULT = 0x8004231E_u32 as _;
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: windows_core::HRESULT = 0x80042322_u32 as _;
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: windows_core::HRESULT = 0x80042317_u32 as _;
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: windows_core::HRESULT = 0x80042312_u32 as _;
pub const VSS_E_MISSING_DISK: windows_core::HRESULT = 0x800423F8_u32 as _;
pub const VSS_E_MISSING_HIDDEN_VOLUME: windows_core::HRESULT = 0x800423F9_u32 as _;
pub const VSS_E_MISSING_VOLUME: windows_core::HRESULT = 0x800423FA_u32 as _;
pub const VSS_E_NESTED_VOLUME_LIMIT: windows_core::HRESULT = 0x8004232C_u32 as _;
pub const VSS_E_NONTRANSPORTABLE_BCD: windows_core::HRESULT = 0x800423FD_u32 as _;
pub const VSS_E_NOT_SUPPORTED: windows_core::HRESULT = 0x8004232F_u32 as _;
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: windows_core::HRESULT = 0x80042320_u32 as _;
pub const VSS_E_OBJECT_ALREADY_EXISTS: windows_core::HRESULT = 0x8004230D_u32 as _;
pub const VSS_E_OBJECT_NOT_FOUND: windows_core::HRESULT = 0x80042308_u32 as _;
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: windows_core::HRESULT = 0x80042303_u32 as _;
pub const VSS_E_PROVIDER_IN_USE: windows_core::HRESULT = 0x80042307_u32 as _;
pub const VSS_E_PROVIDER_NOT_REGISTERED: windows_core::HRESULT = 0x80042304_u32 as _;
pub const VSS_E_PROVIDER_VETO: windows_core::HRESULT = 0x80042306_u32 as _;
pub const VSS_E_REBOOT_REQUIRED: windows_core::HRESULT = 0x80042327_u32 as _;
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: windows_core::HRESULT = 0x80042323_u32 as _;
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: windows_core::HRESULT = 0x80042324_u32 as _;
pub const VSS_E_RESYNC_IN_PROGRESS: windows_core::HRESULT = 0x800423FF_u32 as _;
pub const VSS_E_REVERT_IN_PROGRESS: windows_core::HRESULT = 0x80042325_u32 as _;
pub const VSS_E_REVERT_VOLUME_LOST: windows_core::HRESULT = 0x80042326_u32 as _;
pub const VSS_E_SNAPSHOT_NOT_IN_SET: windows_core::HRESULT = 0x8004232B_u32 as _;
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: windows_core::HRESULT = 0x80042316_u32 as _;
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: windows_core::HRESULT = 0x80042321_u32 as _;
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: windows_core::HRESULT = 0x80042328_u32 as _;
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: windows_core::HRESULT = 0x80042329_u32 as _;
pub const VSS_E_UNEXPECTED: windows_core::HRESULT = 0x80042302_u32 as _;
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: windows_core::HRESULT = 0x8004230F_u32 as _;
pub const VSS_E_UNEXPECTED_WRITER_ERROR: windows_core::HRESULT = 0x80042315_u32 as _;
pub const VSS_E_UNSELECTED_VOLUME: windows_core::HRESULT = 0x8004232A_u32 as _;
pub const VSS_E_UNSUPPORTED_CONTEXT: windows_core::HRESULT = 0x8004231B_u32 as _;
pub const VSS_E_VOLUME_IN_USE: windows_core::HRESULT = 0x8004231D_u32 as _;
pub const VSS_E_VOLUME_NOT_LOCAL: windows_core::HRESULT = 0x8004232D_u32 as _;
pub const VSS_E_VOLUME_NOT_SUPPORTED: windows_core::HRESULT = 0x8004230C_u32 as _;
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: windows_core::HRESULT = 0x8004230E_u32 as _;
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: windows_core::HRESULT = 0x800423F0_u32 as _;
pub const VSS_E_WRITERERROR_NONRETRYABLE: windows_core::HRESULT = 0x800423F4_u32 as _;
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: windows_core::HRESULT = 0x800423F1_u32 as _;
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: windows_core::HRESULT = 0x80042336_u32 as _;
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: windows_core::HRESULT = 0x800423F5_u32 as _;
pub const VSS_E_WRITERERROR_RETRYABLE: windows_core::HRESULT = 0x800423F3_u32 as _;
pub const VSS_E_WRITERERROR_TIMEOUT: windows_core::HRESULT = 0x800423F2_u32 as _;
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: windows_core::HRESULT = 0x8004231A_u32 as _;
pub const VSS_E_WRITER_INFRASTRUCTURE: windows_core::HRESULT = 0x80042318_u32 as _;
pub const VSS_E_WRITER_NOT_RESPONDING: windows_core::HRESULT = 0x80042319_u32 as _;
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: windows_core::HRESULT = 0x80042409_u32 as _;
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 15i32;
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 3840i32;
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE = 65536i32;
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 2i32;
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 512i32;
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 1i32;
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 256i32;
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 4i32;
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 1024i32;
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 8i32;
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 2048i32;
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = 3i32;
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = 2i32;
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = 0i32;
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = 1i32;
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = 1i32;
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = 4i32;
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = 3i32;
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = 2i32;
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = 5i32;
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = 0i32;
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = 2048i32;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = 1024i32;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = 512i32;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = 256i32;
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: VSS_PROTECTION_FAULT = 6i32;
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: VSS_PROTECTION_FAULT = 7i32;
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: VSS_PROTECTION_FAULT = 11i32;
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: VSS_PROTECTION_FAULT = 8i32;
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: VSS_PROTECTION_FAULT = 1i32;
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: VSS_PROTECTION_FAULT = 14i32;
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: VSS_PROTECTION_FAULT = 15i32;
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: VSS_PROTECTION_FAULT = 12i32;
pub const VSS_PROTECTION_FAULT_GROW_FAILED: VSS_PROTECTION_FAULT = 10i32;
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: VSS_PROTECTION_FAULT = 9i32;
pub const VSS_PROTECTION_FAULT_IO_FAILURE: VSS_PROTECTION_FAULT = 13i32;
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: VSS_PROTECTION_FAULT = 2i32;
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: VSS_PROTECTION_FAULT = 5i32;
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: VSS_PROTECTION_FAULT = 4i32;
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: VSS_PROTECTION_FAULT = 3i32;
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: VSS_PROTECTION_FAULT = 16i32;
pub const VSS_PROTECTION_FAULT_NONE: VSS_PROTECTION_FAULT = 0i32;
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = 0i32;
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = 1i32;
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = 4i32;
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = 3i32;
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = 2i32;
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = 1i32;
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = 0i32;
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES = 512i32;
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = 2i32;
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES = 256i32;
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = 1i32;
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES = 4i32;
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES = 8i32;
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES = 32i32;
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES = 16i32;
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = 128i32;
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES = 64i32;
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = 512i32;
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = 256i32;
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = 2i32;
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = 1i32;
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = 3i32;
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = 0i32;
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = 7i32;
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = 5i32;
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM = 6i32;
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = 2i32;
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = 1i32;
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = 8i32;
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM = 4i32;
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = 3i32;
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = 0i32;
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = 2i32;
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = 3i32;
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = 1i32;
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = 0i32;
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = 1i32;
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = 2i32;
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = 3i32;
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = 0i32;
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = 2i32;
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = 3i32;
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = 1i32;
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = 4i32;
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = 0i32;
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY = 2i32;
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = 1i32;
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = -1i32;
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = 2i32;
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = 8i32;
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = 1i32;
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = 4i32;
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID = 12i32;
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = 8i32;
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = 9i32;
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = 5i32;
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = 6i32;
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = 10i32;
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = 7i32;
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = 3i32;
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID = 11i32;
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = 4i32;
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = 1i32;
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = 2i32;
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = 13i32;
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = 0i32;
pub const VSS_SS_ABORTED: VSS_SNAPSHOT_STATE = 13i32;
pub const VSS_SS_COMMITTED: VSS_SNAPSHOT_STATE = 7i32;
pub const VSS_SS_COUNT: VSS_SNAPSHOT_STATE = 16i32;
pub const VSS_SS_CREATED: VSS_SNAPSHOT_STATE = 12i32;
pub const VSS_SS_DELETED: VSS_SNAPSHOT_STATE = 14i32;
pub const VSS_SS_POSTCOMMITTED: VSS_SNAPSHOT_STATE = 15i32;
pub const VSS_SS_PRECOMMITTED: VSS_SNAPSHOT_STATE = 5i32;
pub const VSS_SS_PREFINALCOMMITTED: VSS_SNAPSHOT_STATE = 10i32;
pub const VSS_SS_PREPARED: VSS_SNAPSHOT_STATE = 3i32;
pub const VSS_SS_PREPARING: VSS_SNAPSHOT_STATE = 1i32;
pub const VSS_SS_PROCESSING_COMMIT: VSS_SNAPSHOT_STATE = 6i32;
pub const VSS_SS_PROCESSING_POSTCOMMIT: VSS_SNAPSHOT_STATE = 8i32;
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: VSS_SNAPSHOT_STATE = 11i32;
pub const VSS_SS_PROCESSING_PRECOMMIT: VSS_SNAPSHOT_STATE = 4i32;
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: VSS_SNAPSHOT_STATE = 9i32;
pub const VSS_SS_PROCESSING_PREPARE: VSS_SNAPSHOT_STATE = 2i32;
pub const VSS_SS_UNKNOWN: VSS_SNAPSHOT_STATE = 0i32;
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = 2i32;
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = 3i32;
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = 1i32;
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = 0i32;
pub const VSS_S_ASYNC_CANCELLED: windows_core::HRESULT = 0x4230B_u32 as _;
pub const VSS_S_ASYNC_FINISHED: windows_core::HRESULT = 0x4230A_u32 as _;
pub const VSS_S_ASYNC_PENDING: windows_core::HRESULT = 0x42309_u32 as _;
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: windows_core::HRESULT = 0x42321_u32 as _;
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = 1i32;
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = 4i32;
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = 2i32;
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = 0i32;
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = 3i32;
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 4194304i32;
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 4i32;
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 16777216i32;
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 131072i32;
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 1048576i32;
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 2097152i32;
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 67108864i32;
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 65536i32;
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 524288i32;
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 64i32;
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 128i32;
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 2i32;
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 8i32;
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 16i32;
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 1i32;
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 262144i32;
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 8388608i32;
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 32i32;
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 33554432i32;
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = 3i32;
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = 2i32;
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = 1i32;
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = 0i32;
pub const VSS_WS_COUNT: VSS_WRITER_STATE = 16i32;
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: VSS_WRITER_STATE = 15i32;
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: VSS_WRITER_STATE = 12i32;
pub const VSS_WS_FAILED_AT_FREEZE: VSS_WRITER_STATE = 9i32;
pub const VSS_WS_FAILED_AT_IDENTIFY: VSS_WRITER_STATE = 6i32;
pub const VSS_WS_FAILED_AT_POST_RESTORE: VSS_WRITER_STATE = 14i32;
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: VSS_WRITER_STATE = 11i32;
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: VSS_WRITER_STATE = 7i32;
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: VSS_WRITER_STATE = 8i32;
pub const VSS_WS_FAILED_AT_PRE_RESTORE: VSS_WRITER_STATE = 13i32;
pub const VSS_WS_FAILED_AT_THAW: VSS_WRITER_STATE = 10i32;
pub const VSS_WS_STABLE: VSS_WRITER_STATE = 1i32;
pub const VSS_WS_UNKNOWN: VSS_WRITER_STATE = 0i32;
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: VSS_WRITER_STATE = 5i32;
pub const VSS_WS_WAITING_FOR_FREEZE: VSS_WRITER_STATE = 2i32;
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: VSS_WRITER_STATE = 4i32;
pub const VSS_WS_WAITING_FOR_THAW: VSS_WRITER_STATE = 3i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_ALTERNATE_WRITER_STATE(pub i32);
impl windows_core::TypeKind for VSS_ALTERNATE_WRITER_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_APPLICATION_LEVEL(pub i32);
impl windows_core::TypeKind for VSS_APPLICATION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_BACKUP_SCHEMA(pub i32);
impl windows_core::TypeKind for VSS_BACKUP_SCHEMA {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_BACKUP_TYPE(pub i32);
impl windows_core::TypeKind for VSS_BACKUP_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_COMPONENT_FLAGS(pub i32);
impl windows_core::TypeKind for VSS_COMPONENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_COMPONENT_TYPE(pub i32);
impl windows_core::TypeKind for VSS_COMPONENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_FILE_RESTORE_STATUS(pub i32);
impl windows_core::TypeKind for VSS_FILE_RESTORE_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_FILE_SPEC_BACKUP_TYPE(pub i32);
impl windows_core::TypeKind for VSS_FILE_SPEC_BACKUP_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_HARDWARE_OPTIONS(pub i32);
impl windows_core::TypeKind for VSS_HARDWARE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_MGMT_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for VSS_MGMT_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for VSS_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_PROTECTION_FAULT(pub i32);
impl windows_core::TypeKind for VSS_PROTECTION_FAULT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_PROTECTION_LEVEL(pub i32);
impl windows_core::TypeKind for VSS_PROTECTION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_PROVIDER_CAPABILITIES(pub i32);
impl windows_core::TypeKind for VSS_PROVIDER_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_PROVIDER_TYPE(pub i32);
impl windows_core::TypeKind for VSS_PROVIDER_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_RECOVERY_OPTIONS(pub i32);
impl windows_core::TypeKind for VSS_RECOVERY_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_RESTOREMETHOD_ENUM(pub i32);
impl windows_core::TypeKind for VSS_RESTOREMETHOD_ENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_RESTORE_TARGET(pub i32);
impl windows_core::TypeKind for VSS_RESTORE_TARGET {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_RESTORE_TYPE(pub i32);
impl windows_core::TypeKind for VSS_RESTORE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_ROLLFORWARD_TYPE(pub i32);
impl windows_core::TypeKind for VSS_ROLLFORWARD_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_SNAPSHOT_COMPATIBILITY(pub i32);
impl windows_core::TypeKind for VSS_SNAPSHOT_COMPATIBILITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_SNAPSHOT_CONTEXT(pub i32);
impl windows_core::TypeKind for VSS_SNAPSHOT_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_SNAPSHOT_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for VSS_SNAPSHOT_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_SNAPSHOT_STATE(pub i32);
impl windows_core::TypeKind for VSS_SNAPSHOT_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_SOURCE_TYPE(pub i32);
impl windows_core::TypeKind for VSS_SOURCE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_SUBSCRIBE_MASK(pub i32);
impl windows_core::TypeKind for VSS_SUBSCRIBE_MASK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_USAGE_TYPE(pub i32);
impl windows_core::TypeKind for VSS_USAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_VOLUME_SNAPSHOT_ATTRIBUTES(pub i32);
impl windows_core::TypeKind for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_WRITERRESTORE_ENUM(pub i32);
impl windows_core::TypeKind for VSS_WRITERRESTORE_ENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSS_WRITER_STATE(pub i32);
impl windows_core::TypeKind for VSS_WRITER_STATE {
    type TypeKind = windows_core::CopyType;
}
pub const VSSCoordinator: windows_core::GUID = windows_core::GUID::from_u128(0xe579ab5f_1cc4_44b4_bed9_de0991ff0623);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl Default for VSS_DIFF_AREA_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_DIFF_AREA_PROP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl Default for VSS_DIFF_VOLUME_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_DIFF_VOLUME_PROP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl Default for VSS_MGMT_OBJECT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_MGMT_OBJECT_PROP {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl Default for VSS_MGMT_OBJECT_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_MGMT_OBJECT_UNION {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_OBJECT_PROP {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_OBJECT_UNION {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: windows_core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: windows_core::GUID,
    pub m_ClassId: windows_core::GUID,
}
impl Default for VSS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_PROVIDER_PROP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: windows_core::GUID,
    pub m_SnapshotSetId: windows_core::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: windows_core::GUID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: i64,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
impl Default for VSS_SNAPSHOT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_SNAPSHOT_PROP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl Default for VSS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_VOLUME_PROP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_VOLUME_PROTECTION_INFO {
    pub m_protectionLevel: VSS_PROTECTION_LEVEL,
    pub m_volumeIsOfflineForProtection: super::super::Foundation::BOOL,
    pub m_protectionFault: VSS_PROTECTION_FAULT,
    pub m_failureStatus: i32,
    pub m_volumeHasUnusedDiffArea: super::super::Foundation::BOOL,
    pub m_reserved: u32,
}
impl Default for VSS_VOLUME_PROTECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VSS_VOLUME_PROTECTION_INFO {
    type TypeKind = windows_core::CloneType;
}
pub const VssSnapshotMgmt: windows_core::GUID = windows_core::GUID::from_u128(0x0b5a2c52_3eb9_470a_96e2_6c6d4570e40f);
