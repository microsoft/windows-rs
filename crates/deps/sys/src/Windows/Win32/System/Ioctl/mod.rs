#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ASSERT_ALTERNATE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ASSERT_PRIMARY: u32 = 8u32;
pub struct ASYNC_DUPLICATE_EXTENTS_STATUS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ATAPI_ID_CMD: u32 = 161u32;
pub struct BIN_COUNT(i32);
pub struct BIN_RANGE(i32);
pub struct BIN_RESULTS(i32);
pub struct BIN_TYPES(i32);
pub struct BOOT_AREA_INFO(i32);
pub struct BULK_SECURITY_TEST_DATA(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CAP_ATAPI_ID_CMD: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CAP_ATA_ID_CMD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CAP_SMART_CMD: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CDB_SIZE: u32 = 16u32;
pub struct CHANGER_DEVICE_PROBLEM_TYPE(i32);
pub struct CHANGER_ELEMENT(i32);
pub struct CHANGER_ELEMENT_LIST(i32);
pub struct CHANGER_ELEMENT_STATUS(i32);
pub struct CHANGER_ELEMENT_STATUS_EX(i32);
pub struct CHANGER_ELEMENT_STATUS_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_EXCHANGE_MEDIUM(i32);
pub struct CHANGER_FEATURES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_INITIALIZE_ELEMENT_STATUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_MOVE_MEDIUM(i32);
pub struct CHANGER_PRODUCT_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_READ_ELEMENT_STATUS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHANGER_RESERVED_BIT: u32 = 2147483648u32;
pub struct CHANGER_SEND_VOLUME_TAG_INFORMATION(i32);
pub struct CHANGER_SET_ACCESS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_SET_POSITION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHANGER_TO_DRIVE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHANGER_TO_IEPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHANGER_TO_SLOT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHANGER_TO_TRANSPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHECKSUM_TYPE_CRC32: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHECKSUM_TYPE_CRC64: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHECKSUM_TYPE_ECC: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHECKSUM_TYPE_FIRST_UNUSED_TYPE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHECKSUM_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CHECKSUM_TYPE_UNCHANGED: i32 = -1i32;
pub struct CLASS_MEDIA_CHANGE_CONTEXT(i32);
pub struct CLUSTER_RANGE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_DO_NOT_MAP_NAME: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_EXCEPTION_ROOT: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_ROOT: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_TARGET_ROOT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_LAYER_ROOT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_SCRATCH_ROOT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_UNION_LAYER_ROOT: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_EXCEPTION_ROOT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_ROOT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_TARGET_ROOT: u32 = 8u32;
pub struct CONTAINER_ROOT_INFO_INPUT(i32);
pub struct CONTAINER_ROOT_INFO_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_ROOT_INFO_VALID_FLAGS: u32 = 1023u32;
pub struct CONTAINER_VOLUME_STATE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CONTAINER_VOLUME_STATE_HOSTING_CONTAINER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const COPYFILE_SIS_FLAGS: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const COPYFILE_SIS_LINK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const COPYFILE_SIS_REPLACE: u32 = 2u32;
pub struct CREATE_DISK(i32);
pub struct CREATE_DISK_GPT(i32);
pub struct CREATE_DISK_MBR(i32);
pub struct CREATE_USN_JOURNAL_DATA(i32);
pub struct CSVFS_DISK_CONNECTIVITY(i32);
pub struct CSV_CONTROL_OP(i32);
pub struct CSV_CONTROL_PARAM(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CSV_INVALID_DEVICE_NUMBER: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_IS_OWNED_BY_CSVFS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CSV_MGMTLOCK_CHECK_VOLUME_REDIRECTED: u32 = 1u32;
pub struct CSV_MGMT_LOCK(i32);
pub struct CSV_NAMESPACE_INFO(i32);
pub struct CSV_QUERY_FILE_REVISION(i32);
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct CSV_QUERY_FILE_REVISION_FILE_ID_128(i32);
pub struct CSV_QUERY_MDS_PATH(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_CSV_DIRECT_IO_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_SMB_BYPASS_CSV_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_STORAGE_ON_THIS_NODE_IS_CONNECTED: u32 = 1u32;
pub struct CSV_QUERY_MDS_PATH_V2(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const CSV_QUERY_MDS_PATH_V2_VERSION_1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_QUERY_REDIRECT_STATE(i32);
pub struct CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT(i32);
pub struct CSV_QUERY_VOLUME_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_QUERY_VOLUME_REDIRECT_STATE(i32);
pub struct CSV_SET_VOLUME_ID(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DAX_ALLOC_ALIGNMENT_FLAG_FALLBACK_SPECIFIED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DAX_ALLOC_ALIGNMENT_FLAG_MANDATORY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DDUMP_FLAG_DATA_READ_FROM_DEVICE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DECRYPTION_STATUS_BUFFER(i32);
pub struct DELETE_USN_JOURNAL_DATA(i32);
pub struct DETECTION_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICEDUMP_CAP_PRIVATE_SECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICEDUMP_CAP_RESTRICTED_SECTION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICEDUMP_MAX_IDSTRING: u32 = 32u32;
pub struct DEVICEDUMP_PRIVATE_SUBSECTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEDUMP_PUBLIC_SUBSECTION(i32);
pub struct DEVICEDUMP_RESTRICTED_SUBSECTION(i32);
pub struct DEVICEDUMP_SECTION_HEADER(i32);
pub struct DEVICEDUMP_STORAGEDEVICE_DATA(i32);
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP(i32);
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD(i32);
pub struct DEVICEDUMP_STRUCTURE_VERSION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICEDUMP_STRUCTURE_VERSION_V1: u32 = 1u32;
pub struct DEVICEDUMP_SUBSECTION_POINTER(i32);
pub struct DEVICE_COPY_OFFLOAD_DESCRIPTOR(i32);
pub struct DEVICE_DATA_SET_LBP_STATE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DATA_SET_LBP_STATE_PARAMETERS_VERSION_V1: u32 = 1u32;
pub struct DEVICE_DATA_SET_LB_PROVISIONING_STATE(i32);
pub struct DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2(i32);
pub struct DEVICE_DATA_SET_RANGE(i32);
pub struct DEVICE_DATA_SET_REPAIR_OUTPUT(i32);
pub struct DEVICE_DATA_SET_REPAIR_PARAMETERS(i32);
pub struct DEVICE_DATA_SET_SCRUB_EX_OUTPUT(i32);
pub struct DEVICE_DATA_SET_SCRUB_OUTPUT(i32);
pub struct DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT(i32);
pub struct DEVICE_DSM_CONVERSION_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_DSM_DEFINITION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_ALLOCATION_CONSOLIDATEABLE_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_ENTIRE_DATA_SET_RANGE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_PHYSICAL_ADDRESSES_OMIT_TOTAL_RANGES: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_REPAIR_INPUT_TOPOLOGY_ID_PRESENT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_REPAIR_OUTPUT_PARITY_EXTENT: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_SCRUB_SKIP_IN_SYNC: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_TRIM_BYPASS_RZAT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_FLAG_TRIM_NOT_FS_ALLOCATED: u32 = 2147483648u32;
pub struct DEVICE_DSM_FREE_SPACE_OUTPUT(i32);
pub struct DEVICE_DSM_LOST_QUERY_OUTPUT(i32);
pub struct DEVICE_DSM_LOST_QUERY_PARAMETERS(i32);
pub struct DEVICE_DSM_NOTIFICATION_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_NOTIFY_FLAG_BEGIN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_NOTIFY_FLAG_END: u32 = 2u32;
pub struct DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS(i32);
pub struct DEVICE_DSM_OFFLOAD_READ_PARAMETERS(i32);
pub struct DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_PARAMETERS_V1: u32 = 1u32;
pub struct DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_VERSION_V1: u32 = 1u32;
pub struct DEVICE_DSM_RANGE_ERROR_INFO(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_RANGE_ERROR_INFO_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_DSM_RANGE_ERROR_OUTPUT_V1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_DSM_REPORT_ZONES_DATA(i32);
pub struct DEVICE_DSM_REPORT_ZONES_PARAMETERS(i32);
pub struct DEVICE_DSM_TIERING_QUERY_INPUT(i32);
pub struct DEVICE_DSM_TIERING_QUERY_OUTPUT(i32);
pub struct DEVICE_INTERNAL_STATUS_DATA(i32);
pub struct DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(i32);
pub struct DEVICE_INTERNAL_STATUS_DATA_SET(i32);
pub struct DEVICE_LB_PROVISIONING_DESCRIPTOR(i32);
pub struct DEVICE_LOCATION(i32);
pub struct DEVICE_MANAGE_DATA_SET_ATTRIBUTES(i32);
pub struct DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT(i32);
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct DEVICE_MEDIA_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_POWER_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_SEEK_PENALTY_DESCRIPTOR(i32);
pub struct DEVICE_STORAGE_ADDRESS_RANGE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DEVICE_STORAGE_NO_ERRORS: u32 = 1u32;
pub struct DEVICE_STORAGE_RANGE_ATTRIBUTES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_TRIM_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_WRITE_AGGREGATION_DESCRIPTOR(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_Ioctl`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Storage_Disk_Number: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_Ioctl`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Storage_Gpt_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_Ioctl`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Storage_Gpt_Type: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_Ioctl`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Storage_Mbr_Type: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_Ioctl`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Storage_Partition_Number: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_Ioctl`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Storage_Portable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_Ioctl`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Storage_Removable_Media: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_Ioctl`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Storage_System_Critical: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] },
    pid: 4u32,
};
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DISABLE_SMART: u32 = 217u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DISK_ATTRIBUTE_OFFLINE: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DISK_ATTRIBUTE_READ_ONLY: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DISK_BINNING: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DISK_CACHE_INFORMATION(i32);
pub struct DISK_CACHE_RETENTION_PRIORITY(i32);
pub struct DISK_CONTROLLER_NUMBER(i32);
pub struct DISK_DETECTION_INFO(i32);
pub struct DISK_EXTENT(i32);
pub struct DISK_EX_INT13_INFO(i32);
pub struct DISK_GEOMETRY(i32);
pub struct DISK_GEOMETRY_EX(i32);
pub struct DISK_GROW_PARTITION(i32);
pub struct DISK_HISTOGRAM(i32);
pub struct DISK_INT13_INFO(i32);
pub struct DISK_LOGGING(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DISK_LOGGING_DUMP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DISK_LOGGING_START: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DISK_LOGGING_STOP: u32 = 1u32;
pub struct DISK_PARTITION_INFO(i32);
pub struct DISK_PERFORMANCE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DISK_RECORD(i32);
pub struct DRIVERSTATUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVE_LAYOUT_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVE_LAYOUT_INFORMATION_EX(i32);
pub struct DRIVE_LAYOUT_INFORMATION_GPT(i32);
pub struct DRIVE_LAYOUT_INFORMATION_MBR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DUPLICATE_EXTENTS_DATA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct DUPLICATE_EXTENTS_DATA32(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DUPLICATE_EXTENTS_DATA_EX(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct DUPLICATE_EXTENTS_DATA_EX32(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DUPLICATE_EXTENTS_DATA_EX_ASYNC: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC: u32 = 1u32;
pub struct DUPLICATE_EXTENTS_STATE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const DeviceDsmActionFlag_NonDestructive: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const EFS_TRACKED_OFFSET_HEADER_FLAG: u32 = 1u32;
pub struct ELEMENT_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ENABLE_DISABLE_AUTOSAVE: u32 = 210u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ENABLE_DISABLE_AUTO_OFFLINE: u32 = 219u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ENABLE_SMART: u32 = 216u32;
pub struct ENCRYPTED_DATA_INFO(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ENCRYPTED_DATA_INFO_SPARSE_FILE: u32 = 1u32;
pub struct ENCRYPTION_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ENCRYPTION_FORMAT_DEFAULT: u32 = 1u32;
pub struct ENCRYPTION_KEY_CTRL_INPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ERROR_DRIVE_NOT_INSTALLED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ERROR_HISTORY_DIRECTORY_ENTRY_DEFAULT_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ERROR_INIT_STATUS_NEEDED: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ERROR_LABEL_QUESTIONABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ERROR_LABEL_UNREADABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ERROR_SLOT_NOT_PRESENT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ERROR_TRAY_MALFUNCTION: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ERROR_UNHANDLED_ERROR: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const EXECUTE_OFFLINE_DIAGS: u32 = 212u32;
pub struct EXFAT_STATISTICS(i32);
pub struct EXTENDED_ENCRYPTED_DATA_INFO(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const EXTEND_IEPORT: u32 = 2u32;
pub struct FAT_STATISTICS(i32);
pub struct FILESYSTEM_STATISTICS(i32);
pub struct FILESYSTEM_STATISTICS_EX(i32);
pub struct FILESYSTEM_STATISTICS_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILESYSTEM_STATISTICS_TYPE_REFS: u32 = 4u32;
pub struct FILE_ALLOCATED_RANGE_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_ANY_ACCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_CLEAR_ENCRYPTION: u32 = 2u32;
pub struct FILE_DESIRED_STORAGE_CLASS_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_8042_PORT: u32 = 39u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_ACPI: u32 = 50u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_BATTERY: u32 = 41u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_BEEP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_BIOMETRIC: u32 = 68u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_BLUETOOTH: u32 = 65u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_BUS_EXTENDER: u32 = 42u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_CD_ROM_FILE_SYSTEM: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_CHANGER: u32 = 48u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_CONSOLE: u32 = 80u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_CONTROLLER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_CRYPT_PROVIDER: u32 = 63u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_DATALINK: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_DEVAPI: u32 = 71u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_DFS: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_DFS_FILE_SYSTEM: u32 = 53u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_DFS_VOLUME: u32 = 54u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_DISK_FILE_SYSTEM: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_EHSTOR: u32 = 70u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_EVENT_COLLECTOR: u32 = 95u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_FILE_SYSTEM: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_FIPS: u32 = 58u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_FULLSCREEN_VIDEO: u32 = 52u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_GPIO: u32 = 72u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_HOLOGRAPHIC: u32 = 91u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_INFINIBAND: u32 = 59u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_INPORT_PORT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_KEYBOARD: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_KS: u32 = 47u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_KSEC: u32 = 57u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MAILSLOT: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MASS_STORAGE: u32 = 45u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MIDI_IN: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MIDI_OUT: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MODEM: u32 = 43u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MOUSE: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MT_COMPOSITE: u32 = 66u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MT_TRANSPORT: u32 = 67u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_MULTI_UNC_PROVIDER: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_NAMED_PIPE: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_NETWORK: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_NETWORK_BROWSER: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_NETWORK_FILE_SYSTEM: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_NETWORK_REDIRECTOR: u32 = 40u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_NFP: u32 = 81u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_NULL: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_NVDIMM: u32 = 90u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_PARALLEL_PORT: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_PERSISTENT_MEMORY: u32 = 89u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_PHYSICAL_NETCARD: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_PMI: u32 = 69u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_POINT_OF_SERVICE: u32 = 84u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_PRINTER: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_PRM: u32 = 94u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SCANNER: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SCREEN: u32 = 28u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SDFXHCI: u32 = 92u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SERENUM: u32 = 55u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SERIAL_MOUSE_PORT: u32 = 26u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SERIAL_PORT: u32 = 27u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SMB: u32 = 46u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SOUND: u32 = 29u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SOUNDWIRE: u32 = 97u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_STORAGE_REPLICATION: u32 = 85u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_STREAMS: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_SYSENV: u32 = 82u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_TAPE_FILE_SYSTEM: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_TERMSRV: u32 = 56u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_TRANSPORT: u32 = 33u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_TRUST_ENV: u32 = 86u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_UCM: u32 = 87u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_UCMTCPCI: u32 = 88u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_UCMUCSI: u32 = 93u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_UNKNOWN: u32 = 34u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_USB4: u32 = 96u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_USBEX: u32 = 73u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_VDM: u32 = 44u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_VIDEO: u32 = 35u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_VIRTUAL_BLOCK: u32 = 83u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_VIRTUAL_DISK: u32 = 36u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_VMBUS: u32 = 62u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_WAVE_IN: u32 = 37u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_WAVE_OUT: u32 = 38u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_DEVICE_WPD: u32 = 64u32;
pub struct FILE_FS_PERSISTENT_VOLUME_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NON_RESIDENT: u64 = 137438953472u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NOT_FOUND: u64 = 4096u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_TOO_SMALL: u64 = 68719476736u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_CLUSTERS_ALREADY_IN_USE: u64 = 32768u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_DENY_DEFRAG: u64 = 274877906944u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_IS_BASE_RECORD: u64 = 524288u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_BASE_RECORD: u64 = 8u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_EXIST: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_IN_USE: u64 = 1u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_ORPHAN: u64 = 262144u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_REUSED: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INDEX_ENTRY_MISMATCH: u64 = 1099511627776u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ARRAY_LENGTH_COUNT: u64 = 1048576u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_LCN: u64 = 4294967296u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ORPHAN_RECOVERY_NAME: u64 = 2199023255552u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_PARENT: u64 = 8388608u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_RUN_LENGTH: u64 = 131072u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_VCN: u64 = 8589934592u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_LCN_NOT_EXIST: u64 = 65536u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_MULTIPLE_FILE_NAME_ATTRIBUTES: u64 = 4398046511104u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NAME_CONFLICT: u64 = 17179869184u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NOTHING_WRONG: u64 = 2048u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NOT_IMPLEMENTED: u64 = 32u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN: u64 = 34359738368u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN_GENERATED: u64 = 512u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_GENERIC_NAMES: u64 = 1073741824u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_RESOURCE: u64 = 2147483648u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_BASE_RECORD: u64 = 134217728u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_EXIST: u64 = 67108864u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_INDEX: u64 = 268435456u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_IN_USE: u64 = 16777216u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_REUSED: u64 = 33554432u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_POTENTIAL_CROSSLINK: u64 = 8192u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PREVIOUS_PARENT_STILL_VALID: u64 = 549755813888u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_RECURSIVELY_CORRUPTED: u64 = 256u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_REPAIRED: u64 = 1024u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_REPAIR_DISABLED: u64 = 128u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SID_MISMATCH: u64 = 4194304u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SID_VALID: u64 = 2097152u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_STALE_INFORMATION: u64 = 16384u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SYSTEM_FILE: u64 = 16u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_UNABLE_TO_REPAIR: u64 = 64u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_INITIATE_REPAIR_HINT1_VALID_INDEX_ENTRY: u64 = 536870912u64;
pub struct FILE_INITIATE_REPAIR_OUTPUT_BUFFER(i32);
pub struct FILE_LAYOUT_ENTRY(i32);
pub struct FILE_LAYOUT_INFO_ENTRY(i32);
pub struct FILE_LAYOUT_NAME_ENTRY(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_LAYOUT_NAME_ENTRY_DOS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_LAYOUT_NAME_ENTRY_PRIMARY: u32 = 1u32;
pub struct FILE_LEVEL_TRIM(i32);
pub struct FILE_LEVEL_TRIM_OUTPUT(i32);
pub struct FILE_LEVEL_TRIM_RANGE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_MAKE_COMPATIBLE_BUFFER(i32);
pub struct FILE_OBJECTID_BUFFER(i32);
pub struct FILE_PREFETCH(i32);
pub struct FILE_PREFETCH_EX(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PREFETCH_TYPE_FOR_CREATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PREFETCH_TYPE_FOR_CREATE_EX: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM_EX: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PREFETCH_TYPE_MAX: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PROVIDER_COMPRESSION_MAXIMUM: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PROVIDER_CURRENT_VERSION: u32 = 1u32;
pub struct FILE_PROVIDER_EXTERNAL_INFO_V0(i32);
pub struct FILE_PROVIDER_EXTERNAL_INFO_V1(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PROVIDER_FLAG_COMPRESS_ON_WRITE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_PROVIDER_SINGLE_FILE: u32 = 1u32;
pub struct FILE_QUERY_ON_DISK_VOL_INFO_BUFFER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_QUERY_SPARING_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_READ_ACCESS: u32 = 1u32;
pub struct FILE_REFERENCE_RANGE(i32);
pub struct FILE_REGION_INFO(i32);
pub struct FILE_REGION_INPUT(i32);
pub struct FILE_REGION_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_REGION_USAGE_HUGE_PAGE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_REGION_USAGE_LARGE_PAGE_ALIGNMENT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_REGION_USAGE_OTHER_PAGE_ALIGNMENT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_REGION_USAGE_QUERY_ALIGNMENT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_REGION_USAGE_VALID_CACHED_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_REGION_USAGE_VALID_NONCACHED_DATA: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_SET_DEFECT_MGMT_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_SET_ENCRYPTION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_SET_SPARSE_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_SPECIAL_ACCESS: u32 = 0u32;
pub struct FILE_STORAGE_TIER(i32);
pub struct FILE_STORAGE_TIER_CLASS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_STORAGE_TIER_DESCRIPTION_LENGTH: u32 = 512u32;
pub struct FILE_STORAGE_TIER_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_STORAGE_TIER_FLAG_PARITY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_STORAGE_TIER_FLAG_READ_CACHE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_STORAGE_TIER_FLAG_SMR: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_STORAGE_TIER_FLAG_WRITE_BACK_CACHE: u32 = 2097152u32;
pub struct FILE_STORAGE_TIER_MEDIA_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
pub struct FILE_STORAGE_TIER_REGION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_SYSTEM_RECOGNITION_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_BEGIN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_END: u32 = 2u32;
pub const FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2638560951,
    data2: 53926,
    data3: 19901,
    data4: [162, 227, 251, 208, 237, 145, 9, 169],
};
pub const FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3076672868,
    data2: 47523,
    data3: 19704,
    data4: [128, 17, 91, 134, 201, 64, 231, 183],
};
pub const FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 218784929, data2: 14588, data3: 19896, data4: [159, 231, 63, 67, 82, 205, 124, 92] };
pub struct FILE_TYPE_NOTIFICATION_INPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_WRITE_ACCESS: u32 = 2u32;
pub struct FILE_ZERO_DATA_INFORMATION(i32);
pub struct FILE_ZERO_DATA_INFORMATION_EX(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FILE_ZERO_DATA_INFORMATION_FLAG_PRESERVE_CACHED_DATA: u32 = 1u32;
#[cfg(feature = "Win32_Security")]
pub struct FIND_BY_SID_DATA(i32);
pub struct FIND_BY_SID_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FLAG_USN_TRACK_MODIFIED_RANGES_ENABLE: u32 = 1u32;
pub struct FORMAT_EX_PARAMETERS(i32);
pub struct FORMAT_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ADD_OVERLAY: u32 = 623408u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ADVANCE_FILE_ID: u32 = 590532u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ALLOW_EXTENDED_DASD_IO: u32 = 589955u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CLEAN_VOLUME_METADATA: u32 = 590716u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CORRUPTION_HANDLING: u32 = 590432u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CREATE_OR_GET_OBJECT_ID: u32 = 590016u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CREATE_USN_JOURNAL: u32 = 590055u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSC_INTERNAL: u32 = 590255u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_CONTROL: u32 = 590548u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT: u32 = 590420u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_GET_VOLUME_PATH_NAME: u32 = 590416u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_GET_VOLUME_PATH_NAMES_FOR_VOLUME_NAME: u32 = 590424u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_H_BREAKING_SYNC_TUNNEL_REQUEST: u32 = 590564u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_INTERNAL: u32 = 590444u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_MGMT_LOCK: u32 = 590524u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_QUERY_DOWN_LEVEL_FILE_SYSTEM_CHARACTERISTICS: u32 = 590528u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_QUERY_VETO_FILE_DIRECT_IO: u32 = 590540u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_SYNC_TUNNEL_REQUEST: u32 = 590536u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_CSV_TUNNEL_REQUEST: u32 = 590404u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DELETE_CORRUPTED_REFS_CONTAINER: u32 = 590836u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DELETE_EXTERNAL_BACKING: u32 = 590612u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DELETE_OBJECT_ID: u32 = 589984u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DELETE_REPARSE_POINT: u32 = 589996u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DELETE_USN_JOURNAL: u32 = 590072u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DFSR_SET_GHOST_HANDLE_STATE: u32 = 590264u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DISABLE_LOCAL_BUFFERING: u32 = 590520u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DISMOUNT_VOLUME: u32 = 589856u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE: u32 = 623428u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX: u32 = 623592u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ENABLE_PER_IO_FLAGS: u32 = 590892u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ENABLE_UPGRADE: u32 = 622800u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ENCRYPTION_FSCTL_IO: u32 = 590043u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ENCRYPTION_KEY_CONTROL: u32 = 590852u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ENUM_EXTERNAL_BACKING: u32 = 590616u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ENUM_OVERLAY: u32 = 590623u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_ENUM_USN_DATA: u32 = 590003u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_EXTEND_VOLUME: u32 = 590064u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_FILESYSTEM_GET_STATISTICS: u32 = 589920u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_FILESYSTEM_GET_STATISTICS_EX: u32 = 590732u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_FILE_LEVEL_TRIM: u32 = 623112u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_FILE_PREFETCH: u32 = 590112u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_FILE_TYPE_NOTIFICATION: u32 = 590340u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_FIND_FILES_BY_SID: u32 = 589967u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_BOOT_AREA_INFO: u32 = 590384u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_COMPRESSION: u32 = 589884u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_EXTERNAL_BACKING: u32 = 590608u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_FILTER_FILE_IDENTIFIER: u32 = 590788u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_INTEGRITY_INFORMATION: u32 = 590460u32;
pub struct FSCTL_GET_INTEGRITY_INFORMATION_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_NTFS_FILE_RECORD: u32 = 589928u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_NTFS_VOLUME_DATA: u32 = 589924u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_OBJECT_ID: u32 = 589980u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_REFS_VOLUME_DATA: u32 = 590552u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_REPAIR: u32 = 590236u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_REPARSE_POINT: u32 = 589992u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_RETRIEVAL_POINTERS: u32 = 589939u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT: u32 = 590803u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_RETRIEVAL_POINTER_BASE: u32 = 590388u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_RETRIEVAL_POINTER_COUNT: u32 = 590891u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_VOLUME_BITMAP: u32 = 589935u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GET_WOF_VERSION: u32 = 590696u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_GHOST_FILE_EXTENTS: u32 = 623532u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_HCS_ASYNC_TUNNEL_REQUEST: u32 = 590704u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_HCS_SYNC_NO_WRITE_TUNNEL_REQUEST: u32 = 590776u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_HCS_SYNC_TUNNEL_REQUEST: u32 = 590700u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_INITIATE_FILE_METADATA_OPTIMIZATION: u32 = 590684u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_INITIATE_REPAIR: u32 = 590248u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_INTEGRITY_FLAG_CHECKSUM_ENFORCEMENT_OFF: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_INVALIDATE_VOLUMES: u32 = 589908u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_IS_CSV_FILE: u32 = 590408u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_IS_FILE_ON_CSV_VOLUME: u32 = 590428u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_IS_PATHNAME_VALID: u32 = 589868u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_IS_VOLUME_DIRTY: u32 = 589944u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_IS_VOLUME_MOUNTED: u32 = 589864u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_IS_VOLUME_OWNED_BYCSVFS: u32 = 590456u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_LOCK_VOLUME: u32 = 589848u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_LOOKUP_STREAM_FROM_CLUSTER: u32 = 590332u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_MAKE_MEDIA_COMPATIBLE: u32 = 622896u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_MANAGE_BYPASS_IO: u32 = 590920u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_MARK_AS_SYSTEM_HIVE: u32 = 589903u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_MARK_HANDLE: u32 = 590076u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_MARK_VOLUME_DIRTY: u32 = 589872u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_MOVE_FILE: u32 = 589940u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_NOTIFY_DATA_CHANGE: u32 = 590844u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_NOTIFY_STORAGE_SPACE_ALLOCATION: u32 = 590748u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_OFFLOAD_READ: u32 = 606820u32;
pub struct FSCTL_OFFLOAD_READ_INPUT(i32);
pub struct FSCTL_OFFLOAD_READ_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_OFFLOAD_WRITE: u32 = 623208u32;
pub struct FSCTL_OFFLOAD_WRITE_INPUT(i32);
pub struct FSCTL_OFFLOAD_WRITE_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_OPBATCH_ACK_CLOSE_PENDING: u32 = 589840u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_OPLOCK_BREAK_ACKNOWLEDGE: u32 = 589836u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_OPLOCK_BREAK_ACK_NO_2: u32 = 589904u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_OPLOCK_BREAK_NOTIFY: u32 = 589844u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_ALLOCATED_RANGES: u32 = 606415u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_ASYNC_DUPLICATE_EXTENTS_STATUS: u32 = 590896u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_BAD_RANGES: u32 = 590828u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_DEPENDENT_VOLUME: u32 = 590320u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_DIRECT_ACCESS_EXTENTS: u32 = 590747u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_DIRECT_IMAGE_ORIGINAL_BASE: u32 = 590756u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_EXTENT_READ_CACHE_INFO: u32 = 590711u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_FAT_BPB: u32 = 589912u32;
pub struct FSCTL_QUERY_FAT_BPB_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_FILE_LAYOUT: u32 = 590455u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_FILE_METADATA_OPTIMIZATION: u32 = 590688u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_FILE_REGIONS: u32 = 590468u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_FILE_SYSTEM_RECOGNITION: u32 = 590412u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_GHOSTED_FILE_EXTENTS: u32 = 590768u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_ON_DISK_VOLUME_INFO: u32 = 590140u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_PAGEFILE_ENCRYPTION: u32 = 590312u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_PERSISTENT_VOLUME_STATE: u32 = 590396u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_REFS_SMR_VOLUME_INFO: u32 = 590812u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_REFS_VOLUME_COUNTER_INFO: u32 = 590715u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_REGION_INFO: u32 = 590576u32;
pub struct FSCTL_QUERY_REGION_INFO_INPUT(i32);
pub struct FSCTL_QUERY_REGION_INFO_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_RETRIEVAL_POINTERS: u32 = 589883u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT: u32 = 590592u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_SPARING_INFO: u32 = 590136u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_STORAGE_CLASSES: u32 = 590572u32;
pub struct FSCTL_QUERY_STORAGE_CLASSES_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_USN_JOURNAL: u32 = 590068u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_VOLUME_CONTAINER_STATE: u32 = 590736u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_QUERY_VOLUME_NUMA_INFO: u32 = 590804u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_READ_FILE_USN_DATA: u32 = 590059u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_READ_FROM_PLEX: u32 = 606494u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_READ_RAW_ENCRYPTED: u32 = 590051u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_READ_UNPRIVILEGED_USN_JOURNAL: u32 = 590763u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_READ_USN_JOURNAL: u32 = 590011u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REARRANGE_FILE: u32 = 640032u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_RECALL_FILE: u32 = 590103u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REFS_DEALLOCATE_RANGES: u32 = 590808u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REFS_STREAM_SNAPSHOT_MANAGEMENT: u32 = 590912u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REMOVE_OVERLAY: u32 = 623412u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REPAIR_COPIES: u32 = 639668u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REQUEST_BATCH_OPLOCK: u32 = 589832u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REQUEST_FILTER_OPLOCK: u32 = 589916u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REQUEST_OPLOCK: u32 = 590400u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REQUEST_OPLOCK_LEVEL_1: u32 = 589824u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_REQUEST_OPLOCK_LEVEL_2: u32 = 589828u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_RESET_VOLUME_ALLOCATION_HINTS: u32 = 590316u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_RKF_INTERNAL: u32 = 590511u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SCRUB_DATA: u32 = 590512u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SCRUB_UNDISCOVERABLE_ID: u32 = 590840u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SD_GLOBAL_CHANGE: u32 = 590324u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SECURITY_ID_CHECK: u32 = 606391u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_BOOTLOADER_ACCESSED: u32 = 589903u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_COMPRESSION: u32 = 639040u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_DAX_ALLOC_ALIGNMENT_HINT: u32 = 590832u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_DEFECT_MANAGEMENT: u32 = 622900u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_ENCRYPTION: u32 = 590039u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_EXTERNAL_BACKING: u32 = 590604u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_INTEGRITY_INFORMATION: u32 = 639616u32;
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER(i32);
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_INTEGRITY_INFORMATION_EX: u32 = 590720u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_LAYER_ROOT: u32 = 590740u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_OBJECT_ID: u32 = 589976u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_OBJECT_ID_EXTENDED: u32 = 590012u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_PERSISTENT_VOLUME_STATE: u32 = 590392u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_PURGE_FAILURE_MODE: u32 = 590448u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_REFS_FILE_STRICTLY_SEQUENTIAL: u32 = 590820u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_REFS_SMR_VOLUME_GC_PARAMETERS: u32 = 590816u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_REPAIR: u32 = 590232u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_REPARSE_POINT: u32 = 589988u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_REPARSE_POINT_EX: u32 = 590860u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_SHORT_NAME_BEHAVIOR: u32 = 590260u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_SPARSE: u32 = 590020u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_VOLUME_COMPRESSION_STATE: u32 = 590144u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_ZERO_DATA: u32 = 622792u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SET_ZERO_ON_DEALLOCATION: u32 = 590228u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SHRINK_VOLUME: u32 = 590256u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SHUFFLE_FILE: u32 = 639808u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SIS_COPYFILE: u32 = 590080u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SIS_LINK_FILES: u32 = 639236u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SMB_SHARE_FLUSH_AND_PURGE: u32 = 590908u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SPARSE_OVERALLOCATE: u32 = 590668u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SSDI_STORAGE_REQUEST: u32 = 590752u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_START_VIRTUALIZATION_INSTANCE: u32 = 590784u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_START_VIRTUALIZATION_INSTANCE_EX: u32 = 590848u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_STORAGE_QOS_CONTROL: u32 = 590672u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_STREAMS_ASSOCIATE_ID: u32 = 590792u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_STREAMS_QUERY_ID: u32 = 590796u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_STREAMS_QUERY_PARAMETERS: u32 = 590788u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SUSPEND_OVERLAY: u32 = 590724u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SVHDX_ASYNC_TUNNEL_REQUEST: u32 = 590692u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SVHDX_SET_INITIATOR_INFORMATION: u32 = 590600u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_SVHDX_SYNC_TUNNEL_REQUEST: u32 = 590596u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_CREATE_MINIVERSION: u32 = 622972u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_CREATE_SECONDARY_RM: u32 = 622952u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_GET_METADATA_INFO: u32 = 606572u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_GET_TRANSACTED_VERSION: u32 = 606576u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_LIST_TRANSACTIONS: u32 = 606692u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_LIST_TRANSACTION_LOCKED_FILES: u32 = 606688u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_MODIFY_RM: u32 = 622916u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_QUERY_RM_INFORMATION: u32 = 606536u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION: u32 = 606560u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION2: u32 = 590328u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_ROLLFORWARD_REDO: u32 = 622928u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_ROLLFORWARD_UNDO: u32 = 622932u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_SAVEPOINT_INFORMATION: u32 = 622968u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_SHUTDOWN_RM: u32 = 622940u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_START_RM: u32 = 622936u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_TRANSACTION_ACTIVE: u32 = 606604u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION: u32 = 622948u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION2: u32 = 590336u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_UNLOCK_VOLUME: u32 = 589852u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_UNMAP_SPACE: u32 = 590772u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_UPDATE_OVERLAY: u32 = 623416u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_USN_TRACK_MODIFIED_RANGES: u32 = 590580u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_VIRTUAL_STORAGE_PASSTHROUGH: u32 = 590884u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_VIRTUAL_STORAGE_QUERY_PROPERTY: u32 = 590728u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_VIRTUAL_STORAGE_SET_BEHAVIOR: u32 = 590856u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_WAIT_FOR_REPAIR: u32 = 590240u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_WRITE_RAW_ENCRYPTED: u32 = 590047u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_WRITE_USN_CLOSE_RECORD: u32 = 590063u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FSCTL_WRITE_USN_REASON: u32 = 590544u32;
pub struct FS_BPIO_INFLAGS(i32);
pub struct FS_BPIO_INFO(i32);
pub struct FS_BPIO_INPUT(i32);
pub struct FS_BPIO_OPERATIONS(i32);
pub struct FS_BPIO_OUTFLAGS(i32);
pub struct FS_BPIO_OUTPUT(i32);
pub struct FS_BPIO_RESULTS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FW_ISSUEID_NO_ISSUE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const FW_ISSUEID_UNKNOWN: u32 = 4294967295u32;
pub struct GETVERSIONINPARAMS(i32);
pub struct GET_CHANGER_PARAMETERS(i32);
pub struct GET_CHANGER_PARAMETERS_FEATURES1(i32);
pub struct GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST(i32);
pub struct GET_DISK_ATTRIBUTES(i32);
pub struct GET_FILTER_FILE_IDENTIFIER_INPUT(i32);
pub struct GET_FILTER_FILE_IDENTIFIER_OUTPUT(i32);
pub struct GET_LENGTH_INFORMATION(i32);
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct GET_MEDIA_TYPES(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const GET_VOLUME_BITMAP_FLAG_MASK_METADATA: u32 = 1u32;
pub struct GPT_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const GPT_ATTRIBUTE_LEGACY_BIOS_BOOTABLE: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const GPT_ATTRIBUTE_NO_BLOCK_IO_PROTOCOL: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_DAX: u64 = 288230376151711744u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_OFFLINE: u64 = 576460752303423488u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_SERVICE: u64 = 144115188075855872u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const GPT_SPACES_ATTRIBUTE_NO_METADATA: u64 = 9223372036854775808u64;
pub struct GP_LOG_PAGE_DESCRIPTOR(i32);
pub const GUID_DEVICEDUMP_DRIVER_STORAGE_PORT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3665970205, data2: 28994, data3: 19393, data4: [184, 68, 8, 7, 197, 164, 182, 127] };
pub const GUID_DEVICEDUMP_STORAGE_DEVICE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3638712623, data2: 6827, data3: 19798, data4: [167, 70, 31, 117, 133, 223, 64, 244] };
pub const GUID_DEVINTERFACE_CDCHANGER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590610, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_CDROM: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590600, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_COMPORT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2262880736, data2: 32905, data3: 4560, data4: [156, 228, 8, 0, 62, 48, 31, 115] };
pub const GUID_DEVINTERFACE_DISK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590599, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_FLOPPY: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590609, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_HIDDEN_VOLUME: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2131790376,
    data2: 38963,
    data3: 19259,
    data4: [183, 128, 44, 107, 95, 165, 192, 98],
};
pub const GUID_DEVINTERFACE_MEDIUMCHANGER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590608, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_PARTITION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590602, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_SCM_PHYSICAL_DEVICE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1115906205, data2: 19906, data3: 17342, data4: [187, 180, 79, 21, 223, 206, 44, 97] };
pub const GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1295444344, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVINTERFACE_SERVICE_VOLUME: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1856847234,
    data2: 9708,
    data3: 18108,
    data4: [183, 253, 193, 240, 223, 143, 80, 55],
};
pub const GUID_DEVINTERFACE_SES: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395364844, data2: 18389, data3: 19955, data4: [181, 175, 154, 223, 60, 242, 62, 72] };
pub const GUID_DEVINTERFACE_STORAGEPORT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 718077536, data2: 49456, data3: 4562, data4: [176, 130, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_TAPE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590603, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_UNIFIED_ACCESS_RPMB: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 658799649, data2: 48323, data3: 19719, data4: [160, 91, 163, 57, 91, 180, 238, 231] };
pub const GUID_DEVINTERFACE_VMLUN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1866556953, data2: 40745, data3: 17061, data4: [178, 11, 55, 226, 25, 202, 2, 176] };
pub const GUID_DEVINTERFACE_VOLUME: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590605, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_WRITEONCEDISK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590604, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_ZNSDISK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3094954437, data2: 65499, data3: 17351, data4: [182, 177, 32, 182, 50, 240, 177, 9] };
pub const GUID_SCM_PD_HEALTH_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2644693894, data2: 29429, data3: 20195, data4: [129, 85, 236, 160, 103, 142, 59, 6] };
pub const GUID_SCM_PD_PASSTHROUGH_INVDIMM: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1124707376, data2: 3345, data3: 4580, data4: [145, 145, 8, 0, 32, 12, 154, 102] };
pub struct HISTOGRAM_BUCKET(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const HIST_NO_OF_BUCKETS: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IDENTIFY_BUFFER_SIZE: u32 = 512u32;
pub struct IDEREGS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const ID_CMD: u32 = 236u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_BASE: u32 = 48u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_EXCHANGE_MEDIUM: u32 = 3162144u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_GET_ELEMENT_STATUS: u32 = 3194900u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_GET_PARAMETERS: u32 = 3162112u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_GET_PRODUCT_DATA: u32 = 3162120u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_GET_STATUS: u32 = 3162116u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_INITIALIZE_ELEMENT_STATUS: u32 = 3162136u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_MOVE_MEDIUM: u32 = 3162148u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_QUERY_VOLUME_TAGS: u32 = 3194924u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_REINITIALIZE_TRANSPORT: u32 = 3162152u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_SET_ACCESS: u32 = 3194896u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_CHANGER_SET_POSITION: u32 = 3162140u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_BASE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_CHECK_VERIFY: u32 = 477184u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_CONTROLLER_NUMBER: u32 = 458820u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_CREATE_DISK: u32 = 507992u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_DELETE_DRIVE_LAYOUT: u32 = 508160u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_EJECT_MEDIA: u32 = 477192u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_FIND_NEW_DEVICES: u32 = 477208u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_FORMAT_DRIVE: u32 = 508876u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_FORMAT_TRACKS: u32 = 507928u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_FORMAT_TRACKS_EX: u32 = 507948u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_CACHE_INFORMATION: u32 = 475348u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_DISK_ATTRIBUTES: u32 = 458992u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY: u32 = 458752u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY_EX: u32 = 458912u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_DRIVE_LAYOUT: u32 = 475148u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_DRIVE_LAYOUT_EX: u32 = 458832u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_LENGTH_INFO: u32 = 475228u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_MEDIA_TYPES: u32 = 461824u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_PARTITION_INFO: u32 = 475140u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_PARTITION_INFO_EX: u32 = 458824u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GET_WRITE_CACHE_STATE: u32 = 475356u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_GROW_PARTITION: u32 = 508112u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_HISTOGRAM_DATA: u32 = 458804u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_HISTOGRAM_RESET: u32 = 458808u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_HISTOGRAM_STRUCTURE: u32 = 458800u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_IS_WRITABLE: u32 = 458788u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_LOAD_MEDIA: u32 = 477196u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_LOGGING: u32 = 458792u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_MEDIA_REMOVAL: u32 = 477188u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_PERFORMANCE: u32 = 458784u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_PERFORMANCE_OFF: u32 = 458848u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_REASSIGN_BLOCKS: u32 = 507932u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_REASSIGN_BLOCKS_EX: u32 = 508068u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_RELEASE: u32 = 477204u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_REQUEST_DATA: u32 = 458816u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_REQUEST_STRUCTURE: u32 = 458812u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_RESERVE: u32 = 477200u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_RESET_SNAPSHOT_INFO: u32 = 508432u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_SENSE_DEVICE: u32 = 459744u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_SET_CACHE_INFORMATION: u32 = 508120u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_SET_DISK_ATTRIBUTES: u32 = 508148u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_SET_DRIVE_LAYOUT: u32 = 507920u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_SET_DRIVE_LAYOUT_EX: u32 = 507988u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_SET_PARTITION_INFO: u32 = 507912u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_SET_PARTITION_INFO_EX: u32 = 507980u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_UPDATE_DRIVE_SIZE: u32 = 508104u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_UPDATE_PROPERTIES: u32 = 459072u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_DISK_VERIFY: u32 = 458772u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCMBUS_BASE: u32 = 89u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCMBUS_DEVICE_FUNCTION_BASE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_BUS_GET_LOGICAL_DEVICES: u32 = 5832704u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_BUS_GET_PHYSICAL_DEVICES: u32 = 5832708u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_BUS_GET_REGIONS: u32 = 5832712u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_BUS_QUERY_PROPERTY: u32 = 5832716u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_BUS_RUNTIME_FW_ACTIVATE: u32 = 5865488u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_BUS_SET_PROPERTY: u32 = 5865492u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_LD_GET_INTERLEAVE_SET: u32 = 5835776u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_LOGICAL_DEVICE_FUNCTION_BASE: u32 = 768u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_PD_FIRMWARE_ACTIVATE: u32 = 5871624u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_PD_FIRMWARE_DOWNLOAD: u32 = 5871620u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_PD_PASSTHROUGH: u32 = 5888012u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_PD_QUERY_PROPERTY: u32 = 5838848u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_PD_REINITIALIZE_MEDIA: u32 = 5871636u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_PD_SET_PROPERTY: u32 = 5871640u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_PD_UPDATE_MANAGEMENT_STATUS: u32 = 5838864u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SCM_PHYSICAL_DEVICE_FUNCTION_BASE: u32 = 1536u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SERENUM_EXPOSE_HARDWARE: u32 = 3604992u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SERENUM_GET_PORT_NAME: u32 = 3605004u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SERENUM_PORT_DESC: u32 = 3605000u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SERENUM_REMOVE_HARDWARE: u32 = 3604996u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_SERIAL_LSRMST_INSERT: u32 = 1769596u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_ALLOCATE_BC_STREAM: u32 = 3004420u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_ATTRIBUTE_MANAGEMENT: u32 = 3005596u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_BASE: u32 = 45u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_BC_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_BREAK_RESERVATION: u32 = 2969620u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_CHECK_PRIORITY_HINT_SUPPORT: u32 = 2955392u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_CHECK_VERIFY: u32 = 2967552u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_CHECK_VERIFY2: u32 = 2951168u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_DEVICE_POWER_CAP: u32 = 2956436u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_NOTIFY: u32 = 3002820u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_QUERY_CAPS: u32 = 3002824u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_DIAGNOSTIC: u32 = 2956448u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_EJECTION_CONTROL: u32 = 2951488u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_EJECT_MEDIA: u32 = 2967560u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_ENABLE_IDLE_POWER: u32 = 2956416u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_EVENT_NOTIFICATION: u32 = 2956432u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_FAILURE_PREDICTION_CONFIG: u32 = 2953476u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_FIND_NEW_DEVICES: u32 = 2967576u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_FIRMWARE_ACTIVATE: u32 = 3005448u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_FIRMWARE_DOWNLOAD: u32 = 3005444u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_FIRMWARE_GET_INFO: u32 = 2956288u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_FREE_BC_STREAM: u32 = 3004424u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_BC_PROPERTIES: u32 = 2971648u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_COUNTERS: u32 = 2953480u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_DEVICE_INTERNAL_LOG: u32 = 2956484u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER: u32 = 2953344u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER_EX: u32 = 2953348u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY: u32 = 3002816u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY_RAW: u32 = 3002828u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_HOTPLUG_INFO: u32 = 2952212u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_IDLE_POWERUP_REASON: u32 = 2956420u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_LB_PROVISIONING_MAP_RESOURCES: u32 = 2970632u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_MEDIA_SERIAL_NUMBER: u32 = 2952208u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_MEDIA_TYPES: u32 = 2952192u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_MEDIA_TYPES_EX: u32 = 2952196u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_GET_PHYSICAL_ELEMENT_STATUS: u32 = 2956452u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_LOAD_MEDIA: u32 = 2967564u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_LOAD_MEDIA2: u32 = 2951180u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_MANAGE_BYPASS_IO: u32 = 2951360u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_MANAGE_DATA_SET_ATTRIBUTES: u32 = 2987012u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_MCN_CONTROL: u32 = 2951492u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_MEDIA_REMOVAL: u32 = 2967556u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_IN: u32 = 2969624u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_OUT: u32 = 3002396u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_POWER_ACTIVE: u32 = 2956424u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_POWER_IDLE: u32 = 2956428u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_PREDICT_FAILURE: u32 = 2953472u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_PROTOCOL_COMMAND: u32 = 3003328u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_QUERY_PROPERTY: u32 = 2954240u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_READ_CAPACITY: u32 = 2969920u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_REINITIALIZE_MEDIA: u32 = 2987584u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_RELEASE: u32 = 2967572u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_REMOVE_ELEMENT_AND_TRUNCATE: u32 = 2956480u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_RESERVE: u32 = 2967568u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_RESET_BUS: u32 = 2969600u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_RESET_DEVICE: u32 = 2969604u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_RPMB_COMMAND: u32 = 2956440u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_SET_HOTPLUG_INFO: u32 = 3001368u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_SET_PROPERTY: u32 = 2987004u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD: u32 = 3002880u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_START_DATA_INTEGRITY_CHECK: u32 = 3004548u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const IOCTL_STORAGE_STOP_DATA_INTEGRITY_CHECK: u32 = 3004552u32;
pub struct IO_IRP_EXT_TRACK_OFFSET_HEADER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOCK_ELEMENT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOCK_UNLOCK_DOOR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOCK_UNLOCK_IEPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOCK_UNLOCK_KEYPAD: u32 = 4u32;
pub struct LOOKUP_STREAM_FROM_CLUSTER_ENTRY(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_DATA: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_INDEX: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_SYSTEM: u32 = 50331648u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_DENY_DEFRAG_SET: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_FS_SYSTEM_FILE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_PAGE_FILE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_TXF_SYSTEM_FILE: u32 = 8u32;
pub struct LOOKUP_STREAM_FROM_CLUSTER_INPUT(i32);
pub struct LOOKUP_STREAM_FROM_CLUSTER_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_CLOUD_SYNC: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_DISABLE_FILE_METADATA_OPTIMIZATION: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_ENABLE_CPU_CACHE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_ENABLE_USN_SOURCE_ON_PAGING_IO: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_FILTER_METADATA: u32 = 512u32;
#[cfg(feature = "Win32_Foundation")]
pub struct MARK_HANDLE_INFO(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct MARK_HANDLE_INFO32(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_NOT_READ_COPY: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_NOT_REALTIME: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_NOT_TXF_SYSTEM_LOG: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_PROTECT_CLUSTERS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_READ_COPY: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_REALTIME: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_RETURN_PURGE_FAILURE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_SKIP_COHERENCY_SYNC_DISALLOW_WRITES: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_SUPPRESS_VOLUME_OPEN_FLUSH: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MARK_HANDLE_TXF_SYSTEM_LOG: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MAXIMUM_ENCRYPTION_VALUE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MAX_FW_BUCKET_ID_LENGTH: u32 = 132u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MAX_INTERFACE_CODES: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MAX_VOLUME_ID_SIZE: u32 = 36u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MAX_VOLUME_TEMPLATE_SIZE: u32 = 40u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MEDIA_CURRENTLY_MOUNTED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MEDIA_ERASEABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MEDIA_READ_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MEDIA_READ_WRITE: u32 = 8u32;
pub struct MEDIA_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MEDIA_WRITE_ONCE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const MEDIA_WRITE_PROTECTED: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const METHOD_BUFFERED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const METHOD_DIRECT_FROM_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const METHOD_DIRECT_TO_HARDWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const METHOD_IN_DIRECT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const METHOD_NEITHER: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const METHOD_OUT_DIRECT: u32 = 2u32;
pub struct MFT_ENUM_DATA_V0(i32);
pub struct MFT_ENUM_DATA_V1(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MOVE_FILE_DATA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct MOVE_FILE_DATA32(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MOVE_FILE_RECORD_DATA(i32);
pub struct NTFS_EXTENDED_VOLUME_DATA(i32);
pub struct NTFS_FILE_RECORD_INPUT_BUFFER(i32);
pub struct NTFS_FILE_RECORD_OUTPUT_BUFFER(i32);
pub struct NTFS_STATISTICS(i32);
pub struct NTFS_STATISTICS_EX(i32);
pub struct NTFS_VOLUME_DATA_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const OBSOLETE_DISK_GET_WRITE_CACHE_STATE: u32 = 475356u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const OBSOLETE_IOCTL_STORAGE_RESET_BUS: u32 = 3002368u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const OBSOLETE_IOCTL_STORAGE_RESET_DEVICE: u32 = 3002372u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const OFFLOAD_READ_FLAG_ALL_ZERO_BEYOND_CURRENT_RANGE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const OPLOCK_LEVEL_CACHE_HANDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const OPLOCK_LEVEL_CACHE_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const OPLOCK_LEVEL_CACHE_WRITE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTIITON_OS_DATA: u32 = 41u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_BSP: u32 = 43u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_DM: u32 = 84u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_DPP: u32 = 44u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_ENTRY_UNUSED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_EXTENDED: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_EZDRIVE: u32 = 85u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_FAT32: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_FAT32_XINT13: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_FAT_12: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_FAT_16: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_GPT: u32 = 238u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_HUGE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_IFS: u32 = 7u32;
#[cfg(feature = "Win32_Foundation")]
pub struct PARTITION_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PARTITION_INFORMATION_EX(i32);
pub struct PARTITION_INFORMATION_GPT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PARTITION_INFORMATION_MBR(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_LDM: u32 = 66u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_MAIN_OS: u32 = 40u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_MSFT_RECOVERY: u32 = 39u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_NTFT: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_OS2BOOTMGR: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_PREP: u32 = 65u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_PRE_INSTALLED: u32 = 42u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_SPACES: u32 = 231u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_SPACES_DATA: u32 = 215u32;
pub struct PARTITION_STYLE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_SYSTEM: u32 = 239u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_UNIX: u32 = 99u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_WINDOWS_SYSTEM: u32 = 45u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_XENIX_1: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_XENIX_2: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_XINT13: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PARTITION_XINT13_EXTENDED: u32 = 15u32;
pub struct PATHNAME_BUFFER(i32);
pub struct PERF_BIN(i32);
pub struct PERSISTENT_RESERVE_COMMAND(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_BACKED_BY_WIM: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_CHKDSK_RAN_ONCE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_CONTAINS_BACKING_WIM: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_DAX_FORMATTED: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_GLOBAL_METADATA_NO_SEEK_PENALTY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_LOCAL_METADATA_NO_SEEK_PENALTY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_MODIFIED_BY_CHKDSK: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_NO_HEAT_GATHERING: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_NO_WRITE_AUTO_TIERING: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_REALLOCATE_ALL_DATA_WRITES: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_SHORT_NAME_CREATION_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_TXF_DISABLED: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PERSISTENT_VOLUME_STATE_VOLUME_SCRUB_DISABLED: u32 = 2u32;
pub struct PHYSICAL_ELEMENT_STATUS(i32);
pub struct PHYSICAL_ELEMENT_STATUS_DESCRIPTOR(i32);
pub struct PHYSICAL_ELEMENT_STATUS_REQUEST(i32);
pub struct PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK(i32);
pub struct PLEX_READ_DATA_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PREVENT_MEDIA_REMOVAL(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PRODUCT_ID_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const PROJFS_PROTOCOL_VERSION: u32 = 3u32;
pub struct QUERY_BAD_RANGES_INPUT(i32);
pub struct QUERY_BAD_RANGES_INPUT_RANGE(i32);
pub struct QUERY_BAD_RANGES_OUTPUT(i32);
pub struct QUERY_BAD_RANGES_OUTPUT_RANGE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_GUEST_VOLUMES: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_HOST_VOLUMES: u32 = 1u32;
pub struct QUERY_FILE_LAYOUT_FILTER_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTENTS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTRA_INFO: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_FILES_WITH_DSC_ATTRIBUTE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_FULL_PATH_IN_NAMES: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_NAMES: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_ONLY_FILES_WITH_SPECIFIC_ATTRIBUTES: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS_WITH_NO_CLUSTERS_ALLOCATED: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DATA_ATTRIBUTE: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DSC_ATTRIBUTE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EA_ATTRIBUTE: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EFS_ATTRIBUTE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_REPARSE_ATTRIBUTE: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_TXF_ATTRIBUTE: u32 = 512u32;
pub struct QUERY_FILE_LAYOUT_INPUT(i32);
pub struct QUERY_FILE_LAYOUT_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_REPARSE_DATA_INVALID: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_REPARSE_TAG_INVALID: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_RESTART: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_FILE_LAYOUT_SINGLE_INSTANCED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_READ: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_WRITE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_NO_DEFRAG_VOLUME: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const READ_ATTRIBUTES: u32 = 208u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const READ_ATTRIBUTE_BUFFER_SIZE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const READ_COMPRESSION_INFO_VALID: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const READ_COPY_NUMBER_BYPASS_CACHE_FLAG: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const READ_COPY_NUMBER_KEY: u32 = 1380142592u32;
pub struct READ_ELEMENT_ADDRESS_INFO(i32);
pub struct READ_FILE_USN_DATA(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const READ_THRESHOLDS: u32 = 209u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const READ_THRESHOLD_BUFFER_SIZE: u32 = 512u32;
pub struct READ_USN_JOURNAL_DATA_V0(i32);
pub struct READ_USN_JOURNAL_DATA_V1(i32);
pub struct REASSIGN_BLOCKS(i32);
pub struct REASSIGN_BLOCKS_EX(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const RECOVERED_READS_VALID: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const RECOVERED_WRITES_VALID: u32 = 1u32;
pub struct REFS_SMR_VOLUME_GC_ACTION(i32);
pub struct REFS_SMR_VOLUME_GC_METHOD(i32);
pub struct REFS_SMR_VOLUME_GC_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REFS_SMR_VOLUME_GC_PARAMETERS_VERSION_V1: u32 = 1u32;
pub struct REFS_SMR_VOLUME_GC_STATE(i32);
pub struct REFS_SMR_VOLUME_INFO_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V0: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V1: u32 = 1u32;
pub struct REFS_VOLUME_DATA_BUFFER(i32);
pub struct REMOVE_ELEMENT_AND_TRUNCATE_REQUEST(i32);
pub struct REPAIR_COPIES_INPUT(i32);
pub struct REPAIR_COPIES_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REPLACE_ALTERNATE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REPLACE_PRIMARY: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REQUEST_OPLOCK_CURRENT_VERSION: u32 = 1u32;
pub struct REQUEST_OPLOCK_INPUT_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_ACK: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_COMPLETE_ACK_ON_CLOSE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_REQUEST: u32 = 1u32;
pub struct REQUEST_OPLOCK_OUTPUT_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REQUEST_OPLOCK_OUTPUT_FLAG_ACK_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REQUEST_OPLOCK_OUTPUT_FLAG_MODES_PROVIDED: u32 = 2u32;
pub struct REQUEST_RAW_ENCRYPTED_DATA(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const RETRACT_IEPORT: u32 = 3u32;
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER(i32);
pub struct RETRIEVAL_POINTERS_BUFFER(i32);
pub struct RETRIEVAL_POINTER_BASE(i32);
pub struct RETRIEVAL_POINTER_COUNT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const RETURN_SMART_STATUS: u32 = 218u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const REVISION_LENGTH: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SAVE_ATTRIBUTE_VALUES: u32 = 211u32;
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO(i32);
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_BUS_DEDICATED_MEMORY_STATE(i32);
pub struct SCM_BUS_FIRMWARE_ACTIVATION_STATE(i32);
pub struct SCM_BUS_PROPERTY_ID(i32);
pub struct SCM_BUS_PROPERTY_QUERY(i32);
pub struct SCM_BUS_PROPERTY_SET(i32);
pub struct SCM_BUS_QUERY_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_BUS_RUNTIME_FW_ACTIVATION_INFO(i32);
pub struct SCM_BUS_SET_TYPE(i32);
pub struct SCM_INTERLEAVED_PD_INFO(i32);
pub struct SCM_LD_INTERLEAVE_SET_INFO(i32);
pub struct SCM_LOGICAL_DEVICES(i32);
pub struct SCM_LOGICAL_DEVICE_INSTANCE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SCM_MAX_SYMLINK_LEN_IN_CHARS: u32 = 256u32;
pub struct SCM_PD_DESCRIPTOR_HEADER(i32);
pub struct SCM_PD_DEVICE_HANDLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_PD_DEVICE_INFO(i32);
pub struct SCM_PD_DEVICE_SPECIFIC_INFO(i32);
pub struct SCM_PD_DEVICE_SPECIFIC_PROPERTY(i32);
pub struct SCM_PD_FIRMWARE_ACTIVATE(i32);
pub struct SCM_PD_FIRMWARE_ACTIVATION_STATE(i32);
pub struct SCM_PD_FIRMWARE_DOWNLOAD(i32);
pub struct SCM_PD_FIRMWARE_INFO(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SCM_PD_FIRMWARE_LAST_DOWNLOAD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SCM_PD_FIRMWARE_REVISION_LENGTH_BYTES: u32 = 32u32;
pub struct SCM_PD_FIRMWARE_SLOT_INFO(i32);
pub struct SCM_PD_FRU_ID_STRING(i32);
pub struct SCM_PD_HEALTH_NOTIFICATION_DATA(i32);
pub struct SCM_PD_HEALTH_STATUS(i32);
pub struct SCM_PD_LAST_FW_ACTIVATION_STATUS(i32);
pub struct SCM_PD_LOCATION_STRING(i32);
pub struct SCM_PD_MANAGEMENT_STATUS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SCM_PD_MAX_OPERATIONAL_STATUS: u32 = 16u32;
pub struct SCM_PD_MEDIA_REINITIALIZATION_STATUS(i32);
pub struct SCM_PD_OPERATIONAL_STATUS(i32);
pub struct SCM_PD_OPERATIONAL_STATUS_REASON(i32);
pub struct SCM_PD_PASSTHROUGH_INPUT(i32);
pub struct SCM_PD_PASSTHROUGH_INVDIMM_INPUT(i32);
pub struct SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT(i32);
pub struct SCM_PD_PASSTHROUGH_OUTPUT(i32);
pub struct SCM_PD_PROPERTY_ID(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SCM_PD_PROPERTY_NAME_LENGTH_IN_CHARS: u32 = 128u32;
pub struct SCM_PD_PROPERTY_QUERY(i32);
pub struct SCM_PD_PROPERTY_SET(i32);
pub struct SCM_PD_QUERY_TYPE(i32);
pub struct SCM_PD_REINITIALIZE_MEDIA_INPUT(i32);
pub struct SCM_PD_REINITIALIZE_MEDIA_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE(i32);
pub struct SCM_PD_RUNTIME_FW_ACTIVATION_INFO(i32);
pub struct SCM_PD_SET_TYPE(i32);
pub struct SCM_PHYSICAL_DEVICES(i32);
pub struct SCM_PHYSICAL_DEVICE_INSTANCE(i32);
pub struct SCM_REGION(i32);
pub struct SCM_REGIONS(i32);
pub struct SCM_REGION_FLAG(i32);
pub struct SD_CHANGE_MACHINE_SID_INPUT(i32);
pub struct SD_CHANGE_MACHINE_SID_OUTPUT(i32);
pub struct SD_ENUM_SDS_ENTRY(i32);
pub struct SD_ENUM_SDS_INPUT(i32);
pub struct SD_ENUM_SDS_OUTPUT(i32);
pub struct SD_GLOBAL_CHANGE_INPUT(i32);
pub struct SD_GLOBAL_CHANGE_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SD_GLOBAL_CHANGE_TYPE_ENUM_SDS: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SD_GLOBAL_CHANGE_TYPE_MACHINE_SID: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SD_GLOBAL_CHANGE_TYPE_QUERY_STATS: u32 = 65536u32;
pub struct SD_QUERY_STATS_INPUT(i32);
pub struct SD_QUERY_STATS_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SEARCH_ALL: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SEARCH_ALL_NO_SEQ: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SEARCH_ALTERNATE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SEARCH_ALT_NO_SEQ: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SEARCH_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SEARCH_PRI_NO_SEQ: u32 = 5u32;
pub struct SENDCMDINPARAMS(i32);
pub struct SENDCMDOUTPARAMS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SERIAL_NUMBER_LENGTH: u32 = 32u32;
pub struct SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SET_DISK_ATTRIBUTES(i32);
pub struct SET_PARTITION_INFORMATION(i32);
pub struct SET_PARTITION_INFORMATION_EX(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SET_PURGE_FAILURE_MODE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SET_PURGE_FAILURE_MODE_ENABLED: u32 = 1u32;
pub struct SET_PURGE_FAILURE_MODE_INPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SET_REPAIR_DISABLED_AND_BUGCHECK_ON_CORRUPT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SET_REPAIR_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SET_REPAIR_VALID_MASK: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SET_REPAIR_WARN_ABOUT_DATA_LOSS: u32 = 8u32;
pub struct SHRINK_VOLUME_INFORMATION(i32);
pub struct SHRINK_VOLUME_REQUEST_TYPES(i32);
pub struct SI_COPYFILE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_ABORT_OFFLINE_SELFTEST: u32 = 127u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_CMD: u32 = 176u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_CYL_HI: u32 = 194u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_CYL_LOW: u32 = 79u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_ERROR_NO_MEM: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_EXTENDED_SELFTEST_CAPTIVE: u32 = 130u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_EXTENDED_SELFTEST_OFFLINE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_GET_VERSION: u32 = 475264u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_IDE_ERROR: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_INVALID_BUFFER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_INVALID_COMMAND: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_INVALID_DRIVE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_INVALID_FLAG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_INVALID_IOCTL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_INVALID_REGISTER: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_LOG_SECTOR_SIZE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_NOT_SUPPORTED: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_NO_ERROR: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_NO_IDE_DEVICE: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_OFFLINE_ROUTINE_OFFLINE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_RCV_DRIVE_DATA: u32 = 508040u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_RCV_DRIVE_DATA_EX: u32 = 458892u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_READ_LOG: u32 = 213u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_SEND_DRIVE_COMMAND: u32 = 508036u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_SHORT_SELFTEST_CAPTIVE: u32 = 129u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_SHORT_SELFTEST_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SMART_WRITE_LOG: u32 = 214u32;
pub struct SMB_SHARE_FLUSH_AND_PURGE_INPUT(i32);
pub struct SMB_SHARE_FLUSH_AND_PURGE_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SPACES_TRACKED_OFFSET_HEADER_FLAG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SRB_TYPE_SCSI_REQUEST_BLOCK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const SRB_TYPE_STORAGE_REQUEST_BLOCK: u32 = 1u32;
pub struct STARTING_LCN_INPUT_BUFFER(i32);
pub struct STARTING_LCN_INPUT_BUFFER_EX(i32);
pub struct STARTING_VCN_INPUT_BUFFER(i32);
pub struct STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ADAPTER_DESCRIPTOR(i32);
pub struct STORAGE_ADAPTER_SERIAL_NUMBER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_MAX_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_ADDRESS_TYPE_BTL8: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ALLOCATE_BC_STREAM_INPUT(i32);
pub struct STORAGE_ALLOCATE_BC_STREAM_OUTPUT(i32);
pub struct STORAGE_ASSOCIATION_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_ATTRIBUTE_ASYNC_EVENT_NOTIFICATION: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_ATTRIBUTE_BLOCK_IO: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_ATTRIBUTE_BYTE_ADDRESSABLE_IO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_ATTRIBUTE_DYNAMIC_PERSISTENCE: u32 = 4u32;
pub struct STORAGE_ATTRIBUTE_MGMT(i32);
pub struct STORAGE_ATTRIBUTE_MGMT_ACTION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_ATTRIBUTE_PERF_SIZE_INDEPENDENT: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_ATTRIBUTE_VOLATILE: u32 = 8u32;
pub struct STORAGE_BREAK_RESERVATION_REQUEST(i32);
pub struct STORAGE_BUS_RESET_REQUEST(i32);
pub struct STORAGE_COMPONENT_HEALTH_STATUS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_COMPONENT_ROLE_CACHE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_COMPONENT_ROLE_DATA: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_COMPONENT_ROLE_TIERING: u32 = 2u32;
pub struct STORAGE_COUNTER(i32);
pub struct STORAGE_COUNTERS(i32);
pub struct STORAGE_COUNTER_TYPE(i32);
pub struct STORAGE_CRYPTO_ALGORITHM_ID(i32);
pub struct STORAGE_CRYPTO_CAPABILITY(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_CRYPTO_CAPABILITY_VERSION_1: u32 = 1u32;
pub struct STORAGE_CRYPTO_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_CRYPTO_DESCRIPTOR_VERSION_1: u32 = 1u32;
pub struct STORAGE_CRYPTO_KEY_SIZE(i32);
pub struct STORAGE_DESCRIPTOR_HEADER(i32);
pub struct STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct STORAGE_DEVICE_DESCRIPTOR(i32);
pub struct STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_DEVICE_FLAGS_PAGE_83_DEVICEGUID: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_CONFLICT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_NOHWID: u32 = 2u32;
pub struct STORAGE_DEVICE_FORM_FACTOR(i32);
pub struct STORAGE_DEVICE_ID_DESCRIPTOR(i32);
pub struct STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR(i32);
pub struct STORAGE_DEVICE_LED_STATE_DESCRIPTOR(i32);
pub struct STORAGE_DEVICE_LOCATION_DESCRIPTOR(i32);
pub struct STORAGE_DEVICE_MANAGEMENT_STATUS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_DEVICE_MAX_OPERATIONAL_STATUS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_DEVICE_NUMA_NODE_UNKNOWN: u32 = 4294967295u32;
pub struct STORAGE_DEVICE_NUMA_PROPERTY(i32);
pub struct STORAGE_DEVICE_NUMBER(i32);
pub struct STORAGE_DEVICE_NUMBERS(i32);
pub struct STORAGE_DEVICE_NUMBER_EX(i32);
pub struct STORAGE_DEVICE_POWER_CAP(i32);
pub struct STORAGE_DEVICE_POWER_CAP_UNITS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_DEVICE_POWER_CAP_VERSION_V1: u32 = 1u32;
pub struct STORAGE_DEVICE_RESILIENCY_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY(i32);
pub struct STORAGE_DEVICE_TIERING_DESCRIPTOR(i32);
pub struct STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT(i32);
pub struct STORAGE_DIAGNOSTIC_DATA(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_DIAGNOSTIC_FLAG_ADAPTER_REQUEST: u32 = 1u32;
pub struct STORAGE_DIAGNOSTIC_LEVEL(i32);
pub struct STORAGE_DIAGNOSTIC_REQUEST(i32);
pub struct STORAGE_DIAGNOSTIC_TARGET_TYPE(i32);
pub struct STORAGE_DISK_HEALTH_STATUS(i32);
pub struct STORAGE_DISK_OPERATIONAL_STATUS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_EVENT_DEVICE_OPERATION: u64 = 4u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_EVENT_DEVICE_STATUS: u64 = 2u64;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_EVENT_MEDIA_STATUS: u64 = 1u64;
pub struct STORAGE_EVENT_NOTIFICATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_EVENT_NOTIFICATION_VERSION_V1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FAILURE_PREDICTION_CONFIG(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_FAILURE_PREDICTION_CONFIG_V1: u32 = 1u32;
pub struct STORAGE_FRU_ID_DESCRIPTOR(i32);
pub struct STORAGE_GET_BC_PROPERTIES_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_HOTPLUG_INFO(i32);
pub struct STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR(i32);
pub struct STORAGE_HW_ENDURANCE_INFO(i32);
pub struct STORAGE_HW_FIRMWARE_ACTIVATE(i32);
pub struct STORAGE_HW_FIRMWARE_DOWNLOAD(i32);
pub struct STORAGE_HW_FIRMWARE_DOWNLOAD_V2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_HW_FIRMWARE_INFO(i32);
pub struct STORAGE_HW_FIRMWARE_INFO_QUERY(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_HW_FIRMWARE_INVALID_SLOT: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_HW_FIRMWARE_REVISION_LENGTH: u32 = 16u32;
pub struct STORAGE_HW_FIRMWARE_SLOT_INFO(i32);
pub struct STORAGE_IDENTIFIER(i32);
pub struct STORAGE_IDENTIFIER_CODE_SET(i32);
pub struct STORAGE_IDENTIFIER_TYPE(i32);
pub struct STORAGE_IDLE_POWER(i32);
pub struct STORAGE_IDLE_POWERUP_REASON(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_IDLE_POWERUP_REASON_VERSION_V1: u32 = 1u32;
pub struct STORAGE_ID_NAA_FORMAT(i32);
pub struct STORAGE_LB_PROVISIONING_MAP_RESOURCES(i32);
pub struct STORAGE_MEDIA_SERIAL_NUMBER_DATA(i32);
pub struct STORAGE_MEDIA_TYPE(i32);
pub struct STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_MINIPORT_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_OFFLOAD_MAX_TOKEN_LENGTH: u32 = 512u32;
pub struct STORAGE_OFFLOAD_READ_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_OFFLOAD_READ_RANGE_TRUNCATED: u32 = 1u32;
pub struct STORAGE_OFFLOAD_TOKEN(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_OFFLOAD_TOKEN_ID_LENGTH: u32 = 504u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_OFFLOAD_TOKEN_INVALID: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_OFFLOAD_TOKEN_TYPE_ZERO_DATA: u32 = 4294901761u32;
pub struct STORAGE_OFFLOAD_WRITE_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_OFFLOAD_WRITE_RANGE_TRUNCATED: u32 = 1u32;
pub struct STORAGE_OPERATIONAL_REASON(i32);
pub struct STORAGE_OPERATIONAL_STATUS_REASON(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_PHYSICAL_ADAPTER_DATA(i32);
pub struct STORAGE_PHYSICAL_DEVICE_DATA(i32);
pub struct STORAGE_PHYSICAL_NODE_DATA(i32);
pub struct STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR(i32);
pub struct STORAGE_PORT_CODE_SET(i32);
pub struct STORAGE_POWERUP_REASON_TYPE(i32);
pub struct STORAGE_PREDICT_FAILURE(i32);
pub struct STORAGE_PRIORITY_HINT_SUPPORT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PRIORITY_HINT_SUPPORTED: u32 = 1u32;
pub struct STORAGE_PROPERTY_ID(i32);
pub struct STORAGE_PROPERTY_QUERY(i32);
pub struct STORAGE_PROPERTY_SET(i32);
pub struct STORAGE_PROTOCOL_ATA_DATA_TYPE(i32);
pub struct STORAGE_PROTOCOL_COMMAND(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_COMMAND_LENGTH_NVME: u32 = 64u32;
pub struct STORAGE_PROTOCOL_DATA_DESCRIPTOR(i32);
pub struct STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT(i32);
pub struct STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE(i32);
pub struct STORAGE_PROTOCOL_NVME_DATA_TYPE(i32);
pub struct STORAGE_PROTOCOL_SPECIFIC_DATA(i32);
pub struct STORAGE_PROTOCOL_SPECIFIC_DATA_EXT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_NVM_COMMAND: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_BUSY: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_DATA_OVERRUN: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_INSUFFICIENT_RESOURCES: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_INVALID_REQUEST: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_NOT_SUPPORTED: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_NO_DEVICE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_PENDING: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STATUS_THROTTLED_REQUEST: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_PROTOCOL_STRUCTURE_VERSION: u32 = 1u32;
pub struct STORAGE_PROTOCOL_TYPE(i32);
pub struct STORAGE_PROTOCOL_UFS_DATA_TYPE(i32);
#[cfg(feature = "Win32_Storage_Vhd")]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY(i32);
#[cfg(feature = "Win32_Storage_Vhd")]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY(i32);
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST(i32);
#[cfg(feature = "Win32_Storage_Vhd")]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE(i32);
pub struct STORAGE_QUERY_TYPE(i32);
pub struct STORAGE_READ_CAPACITY(i32);
pub struct STORAGE_REINITIALIZE_MEDIA(i32);
pub struct STORAGE_RESERVE_ID(i32);
pub struct STORAGE_RPMB_COMMAND_TYPE(i32);
pub struct STORAGE_RPMB_DATA_FRAME(i32);
pub struct STORAGE_RPMB_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_RPMB_DESCRIPTOR_VERSION_1: u32 = 1u32;
pub struct STORAGE_RPMB_FRAME_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_RPMB_MINIMUM_RELIABLE_WRITE_SIZE: u32 = 512u32;
pub struct STORAGE_SANITIZE_METHOD(i32);
pub struct STORAGE_SET_TYPE(i32);
pub struct STORAGE_SPEC_VERSION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_SUPPORTED_FEATURES_BYPASS_IO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_SUPPORTED_FEATURES_MASK: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_TEMPERATURE_DATA_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_TEMPERATURE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_TEMPERATURE_THRESHOLD(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TEMPERATURE_THRESHOLD_FLAG_ADAPTER_REQUEST: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TEMPERATURE_VALUE_NOT_REPORTED: u32 = 32768u32;
pub struct STORAGE_TIER(i32);
pub struct STORAGE_TIER_CLASS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TIER_DESCRIPTION_LENGTH: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TIER_FLAG_NO_SEEK_PENALTY: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TIER_FLAG_PARITY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TIER_FLAG_READ_CACHE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TIER_FLAG_SMR: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TIER_FLAG_WRITE_BACK_CACHE: u32 = 2097152u32;
pub struct STORAGE_TIER_MEDIA_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
pub struct STORAGE_TIER_REGION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_WRITE_CACHE_PROPERTY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR(i32);
pub struct STORAGE_ZONED_DEVICE_TYPES(i32);
pub struct STORAGE_ZONES_ATTRIBUTES(i32);
pub struct STORAGE_ZONE_CONDITION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ZONE_DESCRIPTOR(i32);
pub struct STORAGE_ZONE_GROUP(i32);
pub struct STORAGE_ZONE_TYPES(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORATTRIBUTE_MANAGEMENT_STATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STORATTRIBUTE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAMS_ASSOCIATE_ID_CLEAR: u32 = 1u32;
pub struct STREAMS_ASSOCIATE_ID_INPUT_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAMS_ASSOCIATE_ID_SET: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAMS_INVALID_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAMS_MAX_ID: u32 = 65535u32;
pub struct STREAMS_QUERY_ID_OUTPUT_BUFFER(i32);
pub struct STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_CLEAR_ENCRYPTION: u32 = 4u32;
pub struct STREAM_EXTENT_ENTRY(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_EXTENT_ENTRY_ALL_EXTENTS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_EXTENT_ENTRY_AS_RETRIEVAL_POINTERS: u32 = 1u32;
pub struct STREAM_INFORMATION_ENTRY(i32);
pub struct STREAM_LAYOUT_ENTRY(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_LAYOUT_ENTRY_HAS_INFORMATION: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_LAYOUT_ENTRY_IMMOVABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_LAYOUT_ENTRY_NO_CLUSTERS_ALLOCATED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_LAYOUT_ENTRY_PINNED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_LAYOUT_ENTRY_RESIDENT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const STREAM_SET_ENCRYPTION: u32 = 3u32;
pub struct TAPE_GET_STATISTICS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TAPE_RESET_STATISTICS: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TAPE_RETURN_ENV_INFO: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TAPE_RETURN_STATISTICS: i32 = 0i32;
pub struct TAPE_STATISTICS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_SMART: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TELEMETRY_COMMAND_SIZE: u32 = 16u32;
pub struct TXFS_CREATE_MINIVERSION_INFO(i32);
pub struct TXFS_GET_METADATA_INFO_OUT(i32);
pub struct TXFS_GET_TRANSACTED_VERSION(i32);
pub struct TXFS_LIST_TRANSACTIONS(i32);
pub struct TXFS_LIST_TRANSACTIONS_ENTRY(i32);
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES(i32);
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_CREATED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_DELETED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_LOGGING_MODE_FULL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_LOGGING_MODE_SIMPLE: u32 = 1u32;
pub struct TXFS_MODIFY_RM(i32);
pub struct TXFS_QUERY_RM_INFORMATION(i32);
pub struct TXFS_READ_BACKUP_INFORMATION_OUT(i32);
pub struct TXFS_RMF_LAGS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_RM_STATE_ACTIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_RM_STATE_NOT_STARTED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_RM_STATE_SHUTTING_DOWN: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_RM_STATE_STARTING: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_REDO_LSN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_VIRTUAL_CLOCK: u32 = 2u32;
pub struct TXFS_ROLLFORWARD_REDO_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_SAVEPOINT_CLEAR: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_SAVEPOINT_CLEAR_ALL: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct TXFS_SAVEPOINT_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_SAVEPOINT_ROLLBACK: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_SAVEPOINT_SET: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOGGING_MODE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MAX: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MIN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_SIZE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_PREFER_AVAILABILITY: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_PREFER_CONSISTENCY: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_PRESERVE_CHANGES: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_START_RM_FLAG_RECOVER_BEST_EFFORT: u32 = 512u32;
pub struct TXFS_START_RM_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_TRANSACTED_VERSION_NONTRANSACTED: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_TRANSACTED_VERSION_UNCOMMITTED: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
pub struct TXFS_TRANSACTION_ACTIVE_INFO(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_TRANSACTION_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_TRANSACTION_STATE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_TRANSACTION_STATE_NOTACTIVE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const TXFS_TRANSACTION_STATE_PREPARED: u32 = 2u32;
pub struct TXFS_WRITE_BACKUP_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const UNDEFINE_ALTERNATE: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const UNDEFINE_PRIMARY: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const UNLOCK_ELEMENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const UNRECOVERED_READS_VALID: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const UNRECOVERED_WRITES_VALID: u32 = 2u32;
pub struct USN_DELETE_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_DELETE_VALID_FLAGS: u32 = 3u32;
pub struct USN_JOURNAL_DATA_V0(i32);
pub struct USN_JOURNAL_DATA_V1(i32);
pub struct USN_JOURNAL_DATA_V2(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_PAGE_SIZE: u32 = 4096u32;
pub struct USN_RANGE_TRACK_OUTPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_BASIC_INFO_CHANGE: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_CLOSE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_COMPRESSION_CHANGE: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_DATA_EXTEND: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_DATA_OVERWRITE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_DATA_TRUNCATION: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_DESIRED_STORAGE_CLASS_CHANGE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_EA_CHANGE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_ENCRYPTION_CHANGE: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_FILE_CREATE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_FILE_DELETE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_HARD_LINK_CHANGE: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_INDEXABLE_CHANGE: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_INTEGRITY_CHANGE: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_NAMED_DATA_EXTEND: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_NAMED_DATA_OVERWRITE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_NAMED_DATA_TRUNCATION: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_OBJECT_ID_CHANGE: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_RENAME_NEW_NAME: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_RENAME_OLD_NAME: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_REPARSE_POINT_CHANGE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_SECURITY_CHANGE: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_STREAM_CHANGE: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const USN_REASON_TRANSACTED_CHANGE: u32 = 4194304u32;
pub struct USN_RECORD_COMMON_HEADER(i32);
pub struct USN_RECORD_EXTENT(i32);
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct USN_RECORD_UNION(i32);
pub struct USN_RECORD_V2(i32);
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct USN_RECORD_V3(i32);
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct USN_RECORD_V4(i32);
pub struct USN_SOURCE_INFO_ID(i32);
pub struct USN_TRACK_MODIFIED_RANGES(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const VALID_NTFT: u32 = 192u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const VENDOR_ID_LENGTH: u32 = 8u32;
pub struct VERIFY_INFORMATION(i32);
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT(i32);
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT_EX(i32);
pub struct VIRTUALIZATION_INSTANCE_INFO_OUTPUT(i32);
pub struct VIRTUAL_STORAGE_BEHAVIOR_CODE(i32);
pub struct VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT(i32);
pub struct VOLUME_BITMAP_BUFFER(i32);
pub struct VOLUME_DISK_EXTENTS(i32);
pub struct VOLUME_GET_GPT_ATTRIBUTES_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const VOLUME_IS_DIRTY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const VOLUME_SESSION_OPEN: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const VOLUME_UPGRADE_SCHEDULED: u32 = 2u32;
pub struct WIM_PROVIDER_ADD_OVERLAY_INPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const WIM_PROVIDER_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const WIM_PROVIDER_EXTERNAL_FLAG_NOT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const WIM_PROVIDER_EXTERNAL_FLAG_SUSPENDED: u32 = 2u32;
pub struct WIM_PROVIDER_EXTERNAL_INFO(i32);
pub struct WIM_PROVIDER_OVERLAY_ENTRY(i32);
pub struct WIM_PROVIDER_REMOVE_OVERLAY_INPUT(i32);
pub struct WIM_PROVIDER_SUSPEND_OVERLAY_INPUT(i32);
pub struct WIM_PROVIDER_UPDATE_OVERLAY_INPUT(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const WOF_CURRENT_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct WOF_EXTERNAL_FILE_ID(i32);
pub struct WOF_EXTERNAL_INFO(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const WOF_PROVIDER_CLOUD: u32 = 3u32;
pub struct WOF_VERSION_INFO(i32);
pub struct WRITE_CACHE_CHANGE(i32);
pub struct WRITE_CACHE_ENABLE(i32);
pub struct WRITE_CACHE_TYPE(i32);
#[doc = "*Required features: `Win32_System_Ioctl`*"]
pub const WRITE_COMPRESSION_INFO_VALID: u32 = 16u32;
pub struct WRITE_THROUGH(i32);
pub struct WRITE_USN_REASON_INPUT(i32);
pub struct _DEVICEDUMP_COLLECTION_TYPE(i32);
