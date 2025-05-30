#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACTIVE_LATENCY_CONFIGURATION {
    pub Anonymous: ACTIVE_LATENCY_CONFIGURATION_0,
}
impl Default for ACTIVE_LATENCY_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union ACTIVE_LATENCY_CONFIGURATION_0 {
    pub Anonymous: ACTIVE_LATENCY_CONFIGURATION_0_0,
    pub AsUshort: u16,
}
impl Default for ACTIVE_LATENCY_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ACTIVE_LATENCY_CONFIGURATION_0_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct BUCKET_COUNTER {
    pub Reserved: u32,
    pub Trim: u32,
    pub Write: u32,
    pub Read: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_BIT_FIELD {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DSSD_POWER_STATE_DESCRIPTOR {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct FIRMWARE_ACTIVATION_HISTORY_ENTRY {
    pub VersionNumber: u8,
    pub Length: u8,
    pub Reserved0: u16,
    pub ActivationCount: u16,
    pub Timestamp: u64,
    pub Reserved1: u64,
    pub PowerCycleCount: u64,
    pub PreviousFirmware: u64,
    pub NewFirmware: u64,
    pub SlotNumber: u8,
    pub CommitActionType: u8,
    pub Result: u16,
    pub Reserved2: [u8; 14],
}
impl Default for FIRMWARE_ACTIVATION_HISTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIRMWARE_ACTIVATION_HISTORY_ENTRY_VERSION_1: u32 = 1u32;
pub const GUID_MFND_CHILD_CONTROLLER_EVENT_LOG_PAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x98bcce18_a5f0_bf35_a544_d97f259d669c);
pub const GUID_MFND_CHILD_CONTROLLER_EVENT_LOG_PAGEGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x98bcce18_a5f0_bf35_a544_d97f259d669c);
pub const GUID_OCP_DEVICE_DEVICE_CAPABILITIES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0d054297_e1d1_98c9_5d49_584b913c05b7);
pub const GUID_OCP_DEVICE_DEVICE_CAPABILITIESGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0d054297_e1d1_98c9_5d49_584b913c05b7);
pub const GUID_OCP_DEVICE_ERROR_RECOVERY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2131d944_30fe_ae34_ab4d_fd3dba83195a);
pub const GUID_OCP_DEVICE_ERROR_RECOVERYGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2131d944_30fe_ae34_ab4d_fd3dba83195a);
pub const GUID_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x769a796d_dab4_a3f6_e24d_b28aacf31cd1);
pub const GUID_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORYGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x769a796d_dab4_a3f6_e24d_b28aacf31cd1);
pub const GUID_OCP_DEVICE_LATENCY_MONITOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8cc07a92_84d0_9c6c_7043_e6d4585ed485);
pub const GUID_OCP_DEVICE_LATENCY_MONITORGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8cc07a92_84d0_9c6c_7043_e6d4585ed485);
pub const GUID_OCP_DEVICE_SMART_INFORMATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2810afc5_bfea_a4f2_9c4f_6f7cc914d5af);
pub const GUID_OCP_DEVICE_SMART_INFORMATIONGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2810afc5_bfea_a4f2_9c4f_6f7cc914d5af);
pub const GUID_OCP_DEVICE_TCG_CONFIGURATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbd244006_e07e_83e6_c047_54fa9d2ae054);
pub const GUID_OCP_DEVICE_TCG_CONFIGURATIONGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbd244006_e07e_83e6_c047_54fa9d2ae054);
pub const GUID_OCP_DEVICE_TCG_HISTORY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x704b513e_09c6_9490_274e_d0969690d788);
pub const GUID_OCP_DEVICE_TCG_HISTORYGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x704b513e_09c6_9490_274e_d0969690d788);
pub const GUID_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e9c722f_2399_bb2c_6348_32d0b798bbc7);
pub const GUID_OCP_DEVICE_UNSUPPORTED_REQUIREMENTSGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e9c722f_2399_bb2c_6348_32d0b798bbc7);
pub const GUID_WCS_DEVICE_ERROR_RECOVERY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2131d944_30fe_ae34_ab4d_fd3dba83195a);
pub const GUID_WCS_DEVICE_ERROR_RECOVERYGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2131d944_30fe_ae34_ab4d_fd3dba83195a);
pub const GUID_WCS_DEVICE_SMART_ATTRIBUTES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2810afc5_bfea_a4f2_9c4f_6f7cc914d5af);
pub const GUID_WCS_DEVICE_SMART_ATTRIBUTESGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2810afc5_bfea_a4f2_9c4f_6f7cc914d5af);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LATENCY_MONITOR_FEATURE_STATUS {
    pub Anonymous: LATENCY_MONITOR_FEATURE_STATUS_0,
}
impl Default for LATENCY_MONITOR_FEATURE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union LATENCY_MONITOR_FEATURE_STATUS_0 {
    pub Anonymous: LATENCY_MONITOR_FEATURE_STATUS_0_0,
    pub AsUchar: u8,
}
impl Default for LATENCY_MONITOR_FEATURE_STATUS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LATENCY_MONITOR_FEATURE_STATUS_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct LATENCY_STAMP {
    pub Trim3: u64,
    pub Write3: u64,
    pub Read3: u64,
    pub Trim2: u64,
    pub Write2: u64,
    pub Read2: u64,
    pub Trim1: u64,
    pub Write1: u64,
    pub Read1: u64,
    pub Trim0: u64,
    pub Write0: u64,
    pub Read0: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct LATENCY_STAMP_UNITS {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MEASURED_LATENCY {
    pub Trim3: u16,
    pub Write3: u16,
    pub Read3: u16,
    pub Trim2: u16,
    pub Write2: u16,
    pub Read2: u16,
    pub Trim1: u16,
    pub Write1: u16,
    pub Read1: u16,
    pub Trim0: u16,
    pub Write0: u16,
    pub Read0: u16,
}
pub type NVME_ACCESS_FREQUENCIES = i32;
pub const NVME_ACCESS_FREQUENCY_FR_WRITE_FR_READ: NVME_ACCESS_FREQUENCIES = 5i32;
pub const NVME_ACCESS_FREQUENCY_FR_WRITE_INFR_READ: NVME_ACCESS_FREQUENCIES = 4i32;
pub const NVME_ACCESS_FREQUENCY_INFR_WRITE_FR_READ: NVME_ACCESS_FREQUENCIES = 3i32;
pub const NVME_ACCESS_FREQUENCY_INFR_WRITE_INFR_READ: NVME_ACCESS_FREQUENCIES = 2i32;
pub const NVME_ACCESS_FREQUENCY_NONE: NVME_ACCESS_FREQUENCIES = 0i32;
pub const NVME_ACCESS_FREQUENCY_ONE_TIME_READ: NVME_ACCESS_FREQUENCIES = 6i32;
pub const NVME_ACCESS_FREQUENCY_SPECULATIVE_READ: NVME_ACCESS_FREQUENCIES = 7i32;
pub const NVME_ACCESS_FREQUENCY_TYPICAL: NVME_ACCESS_FREQUENCIES = 1i32;
pub const NVME_ACCESS_FREQUENCY_WILL_BE_OVERWRITTEN: NVME_ACCESS_FREQUENCIES = 8i32;
pub type NVME_ACCESS_LATENCIES = i32;
pub const NVME_ACCESS_LATENCY_IDLE: NVME_ACCESS_LATENCIES = 1i32;
pub const NVME_ACCESS_LATENCY_LOW: NVME_ACCESS_LATENCIES = 3i32;
pub const NVME_ACCESS_LATENCY_NONE: NVME_ACCESS_LATENCIES = 0i32;
pub const NVME_ACCESS_LATENCY_NORMAL: NVME_ACCESS_LATENCIES = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_ACTIVE_NAMESPACE_ID_LIST {
    pub NSID: [u32; 1024],
}
impl Default for NVME_ACTIVE_NAMESPACE_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_ADMIN_COMMANDS = i32;
pub const NVME_ADMIN_COMMAND_ABORT: NVME_ADMIN_COMMANDS = 8i32;
pub const NVME_ADMIN_COMMAND_ASYNC_EVENT_REQUEST: NVME_ADMIN_COMMANDS = 12i32;
pub const NVME_ADMIN_COMMAND_CREATE_IO_CQ: NVME_ADMIN_COMMANDS = 5i32;
pub const NVME_ADMIN_COMMAND_CREATE_IO_SQ: NVME_ADMIN_COMMANDS = 1i32;
pub const NVME_ADMIN_COMMAND_DELETE_IO_CQ: NVME_ADMIN_COMMANDS = 4i32;
pub const NVME_ADMIN_COMMAND_DELETE_IO_SQ: NVME_ADMIN_COMMANDS = 0i32;
pub const NVME_ADMIN_COMMAND_DEVICE_SELF_TEST: NVME_ADMIN_COMMANDS = 20i32;
pub const NVME_ADMIN_COMMAND_DIRECTIVE_RECEIVE: NVME_ADMIN_COMMANDS = 26i32;
pub const NVME_ADMIN_COMMAND_DIRECTIVE_SEND: NVME_ADMIN_COMMANDS = 25i32;
pub const NVME_ADMIN_COMMAND_DOORBELL_BUFFER_CONFIG: NVME_ADMIN_COMMANDS = 124i32;
pub const NVME_ADMIN_COMMAND_FIRMWARE_ACTIVATE: NVME_ADMIN_COMMANDS = 16i32;
pub const NVME_ADMIN_COMMAND_FIRMWARE_COMMIT: NVME_ADMIN_COMMANDS = 16i32;
pub const NVME_ADMIN_COMMAND_FIRMWARE_IMAGE_DOWNLOAD: NVME_ADMIN_COMMANDS = 17i32;
pub const NVME_ADMIN_COMMAND_FORMAT_NVM: NVME_ADMIN_COMMANDS = 128i32;
pub const NVME_ADMIN_COMMAND_GET_FEATURES: NVME_ADMIN_COMMANDS = 10i32;
pub const NVME_ADMIN_COMMAND_GET_LBA_STATUS: NVME_ADMIN_COMMANDS = 134i32;
pub const NVME_ADMIN_COMMAND_GET_LOG_PAGE: NVME_ADMIN_COMMANDS = 2i32;
pub const NVME_ADMIN_COMMAND_IDENTIFY: NVME_ADMIN_COMMANDS = 6i32;
pub const NVME_ADMIN_COMMAND_NAMESPACE_ATTACHMENT: NVME_ADMIN_COMMANDS = 21i32;
pub const NVME_ADMIN_COMMAND_NAMESPACE_MANAGEMENT: NVME_ADMIN_COMMANDS = 13i32;
pub const NVME_ADMIN_COMMAND_NVME_MI_RECEIVE: NVME_ADMIN_COMMANDS = 30i32;
pub const NVME_ADMIN_COMMAND_NVME_MI_SEND: NVME_ADMIN_COMMANDS = 29i32;
pub const NVME_ADMIN_COMMAND_SANITIZE: NVME_ADMIN_COMMANDS = 132i32;
pub const NVME_ADMIN_COMMAND_SECURITY_RECEIVE: NVME_ADMIN_COMMANDS = 130i32;
pub const NVME_ADMIN_COMMAND_SECURITY_SEND: NVME_ADMIN_COMMANDS = 129i32;
pub const NVME_ADMIN_COMMAND_SET_FEATURES: NVME_ADMIN_COMMANDS = 9i32;
pub const NVME_ADMIN_COMMAND_VIRTUALIZATION_MANAGEMENT: NVME_ADMIN_COMMANDS = 28i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS {
    pub Anonymous: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_0,
    pub AsUlonglong: u64,
}
impl Default for NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_QUEUE_ATTRIBUTES {
    pub Anonymous: NVME_ADMIN_QUEUE_ATTRIBUTES_0,
    pub AsUlong: u32,
}
impl Default for NVME_ADMIN_QUEUE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_ADMIN_QUEUE_ATTRIBUTES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS {
    pub Anonymous: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_0,
    pub AsUlonglong: u64,
}
impl Default for NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_0 {
    pub _bitfield: u64,
}
pub type NVME_AMS_OPTION = i32;
pub const NVME_AMS_ROUND_ROBIN: NVME_AMS_OPTION = 0i32;
pub const NVME_AMS_WEIGHTED_ROUND_ROBIN_URGENT: NVME_AMS_OPTION = 1i32;
pub const NVME_ASYNC_ERROR_DIAG_FAILURE: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 2i32;
pub const NVME_ASYNC_ERROR_FIRMWARE_IMAGE_LOAD_ERROR: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 5i32;
pub const NVME_ASYNC_ERROR_INVALID_DOORBELL_WRITE_VALUE: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 1i32;
pub const NVME_ASYNC_ERROR_INVALID_SUBMISSION_QUEUE: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 0i32;
pub const NVME_ASYNC_ERROR_PERSISTENT_INTERNAL_DEVICE_ERROR: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 3i32;
pub const NVME_ASYNC_ERROR_TRANSIENT_INTERNAL_DEVICE_ERROR: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 4i32;
pub type NVME_ASYNC_EVENT_ERROR_STATUS_CODES = i32;
pub type NVME_ASYNC_EVENT_HEALTH_STATUS_CODES = i32;
pub type NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES = i32;
pub type NVME_ASYNC_EVENT_NOTICE_CODES = i32;
pub type NVME_ASYNC_EVENT_TYPES = i32;
pub const NVME_ASYNC_EVENT_TYPE_ERROR_STATUS: NVME_ASYNC_EVENT_TYPES = 0i32;
pub const NVME_ASYNC_EVENT_TYPE_HEALTH_STATUS: NVME_ASYNC_EVENT_TYPES = 1i32;
pub const NVME_ASYNC_EVENT_TYPE_IO_COMMAND_SET_STATUS: NVME_ASYNC_EVENT_TYPES = 6i32;
pub const NVME_ASYNC_EVENT_TYPE_NOTICE: NVME_ASYNC_EVENT_TYPES = 2i32;
pub const NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC: NVME_ASYNC_EVENT_TYPES = 7i32;
pub type NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES = i32;
pub const NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_DEVICE_PANIC: NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES = 1i32;
pub const NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_RESERVED: NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES = 0i32;
pub const NVME_ASYNC_HEALTH_NVM_SUBSYSTEM_RELIABILITY: NVME_ASYNC_EVENT_HEALTH_STATUS_CODES = 0i32;
pub const NVME_ASYNC_HEALTH_SPARE_BELOW_THRESHOLD: NVME_ASYNC_EVENT_HEALTH_STATUS_CODES = 2i32;
pub const NVME_ASYNC_HEALTH_TEMPERATURE_THRESHOLD: NVME_ASYNC_EVENT_HEALTH_STATUS_CODES = 1i32;
pub const NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED: NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES = 1i32;
pub const NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED_WITH_UNEXPECTED_DEALLOCATION: NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES = 2i32;
pub const NVME_ASYNC_IO_CMD_SET_RESERVATION_LOG_PAGE_AVAILABLE: NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES = 0i32;
pub const NVME_ASYNC_NOTICE_ASYMMETRIC_ACCESS_CHANGE: NVME_ASYNC_EVENT_NOTICE_CODES = 3i32;
pub const NVME_ASYNC_NOTICE_ENDURANCE_GROUP_EVENT_AGGREGATE_LOG_CHANGE: NVME_ASYNC_EVENT_NOTICE_CODES = 6i32;
pub const NVME_ASYNC_NOTICE_FIRMWARE_ACTIVATION_STARTING: NVME_ASYNC_EVENT_NOTICE_CODES = 1i32;
pub const NVME_ASYNC_NOTICE_LBA_STATUS_INFORMATION_ALERT: NVME_ASYNC_EVENT_NOTICE_CODES = 5i32;
pub const NVME_ASYNC_NOTICE_NAMESPACE_ATTRIBUTE_CHANGED: NVME_ASYNC_EVENT_NOTICE_CODES = 0i32;
pub const NVME_ASYNC_NOTICE_PREDICTABLE_LATENCY_EVENT_AGGREGATE_LOG_CHANGE: NVME_ASYNC_EVENT_NOTICE_CODES = 4i32;
pub const NVME_ASYNC_NOTICE_TELEMETRY_LOG_CHANGED: NVME_ASYNC_EVENT_NOTICE_CODES = 2i32;
pub const NVME_ASYNC_NOTICE_ZONE_DESCRIPTOR_CHANGED: NVME_ASYNC_EVENT_NOTICE_CODES = 239i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_AUTO_POWER_STATE_TRANSITION_ENTRY {
    pub _bitfield: u32,
    pub Reserved1: u32,
}
pub const NVME_CC_SHN_ABRUPT_SHUTDOWN: NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS = 2i32;
pub const NVME_CC_SHN_NORMAL_SHUTDOWN: NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS = 1i32;
pub const NVME_CC_SHN_NO_NOTIFICATION: NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS = 0i32;
pub type NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO {
    pub Anonymous: NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_ERROR_INJECTION {
    pub Anonymous: NVME_CDW0_FEATURE_ERROR_INJECTION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW0_FEATURE_ERROR_INJECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW0_FEATURE_ERROR_INJECTION_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE {
    pub Anonymous: NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW0_RESERVATION_PERSISTENCE {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_ABORT {
    pub Anonymous: NVME_CDW10_ABORT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_ABORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_ABORT_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_CREATE_IO_QUEUE {
    pub Anonymous: NVME_CDW10_CREATE_IO_QUEUE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_CREATE_IO_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_CREATE_IO_QUEUE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DATASET_MANAGEMENT {
    pub Anonymous: NVME_CDW10_DATASET_MANAGEMENT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_DATASET_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_DATASET_MANAGEMENT_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_DIRECTIVE_RECEIVE {
    pub NUMD: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_DIRECTIVE_SEND {
    pub NUMD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_FIRMWARE_ACTIVATE {
    pub Anonymous: NVME_CDW10_FIRMWARE_ACTIVATE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_FIRMWARE_ACTIVATE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_FIRMWARE_DOWNLOAD {
    pub NUMD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_FORMAT_NVM {
    pub Anonymous: NVME_CDW10_FORMAT_NVM_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_FORMAT_NVM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_FORMAT_NVM_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_FEATURES {
    pub Anonymous: NVME_CDW10_GET_FEATURES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_GET_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_GET_FEATURES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE_V13 {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_V13_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_GET_LOG_PAGE_V13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_GET_LOG_PAGE_V13_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_IDENTIFY {
    pub Anonymous: NVME_CDW10_IDENTIFY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_IDENTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_IDENTIFY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_ACQUIRE {
    pub Anonymous: NVME_CDW10_RESERVATION_ACQUIRE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_RESERVATION_ACQUIRE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_RESERVATION_ACQUIRE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_REGISTER {
    pub Anonymous: NVME_CDW10_RESERVATION_REGISTER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_RESERVATION_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_RESERVATION_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_RELEASE {
    pub Anonymous: NVME_CDW10_RESERVATION_RELEASE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_RESERVATION_RELEASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_RESERVATION_RELEASE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_REPORT {
    pub Anonymous: NVME_CDW10_RESERVATION_REPORT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_RESERVATION_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_RESERVATION_REPORT_0 {
    pub NUMD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SANITIZE {
    pub Anonymous: NVME_CDW10_SANITIZE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_SANITIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_SANITIZE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SECURITY_SEND_RECEIVE {
    pub Anonymous: NVME_CDW10_SECURITY_SEND_RECEIVE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_SECURITY_SEND_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_SECURITY_SEND_RECEIVE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SET_FEATURES {
    pub Anonymous: NVME_CDW10_SET_FEATURES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_SET_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_SET_FEATURES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_ZONE_APPEND {
    pub SLBA: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_ZONE_MANAGEMENT_RECEIVE {
    pub SLBA: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW10_ZONE_MANAGEMENT_SEND {
    pub SLBA: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_CREATE_IO_CQ {
    pub Anonymous: NVME_CDW11_CREATE_IO_CQ_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_CREATE_IO_CQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_CREATE_IO_CQ_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_CREATE_IO_SQ {
    pub Anonymous: NVME_CDW11_CREATE_IO_SQ_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_CREATE_IO_SQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_CREATE_IO_SQ_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DATASET_MANAGEMENT {
    pub Anonymous: NVME_CDW11_DATASET_MANAGEMENT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_DATASET_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_DATASET_MANAGEMENT_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DIRECTIVE_RECEIVE {
    pub Anonymous: NVME_CDW11_DIRECTIVE_RECEIVE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_DIRECTIVE_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_DIRECTIVE_RECEIVE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DIRECTIVE_SEND {
    pub Anonymous: NVME_CDW11_DIRECTIVE_SEND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_DIRECTIVE_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_DIRECTIVE_SEND_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURES {
    pub NumberOfQueues: NVME_CDW11_FEATURE_NUMBER_OF_QUEUES,
    pub InterruptCoalescing: NVME_CDW11_FEATURE_INTERRUPT_COALESCING,
    pub InterruptVectorConfig: NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG,
    pub LbaRangeType: NVME_CDW11_FEATURE_LBA_RANGE_TYPE,
    pub Arbitration: NVME_CDW11_FEATURE_ARBITRATION,
    pub VolatileWriteCache: NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE,
    pub AsyncEventConfig: NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG,
    pub PowerManagement: NVME_CDW11_FEATURE_POWER_MANAGEMENT,
    pub AutoPowerStateTransition: NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION,
    pub TemperatureThreshold: NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD,
    pub ErrorRecovery: NVME_CDW11_FEATURE_ERROR_RECOVERY,
    pub HostMemoryBuffer: NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER,
    pub WriteAtomicityNormal: NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL,
    pub NonOperationalPowerState: NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE,
    pub IoCommandSetProfile: NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE,
    pub ErrorInjection: NVME_CDW0_FEATURE_ERROR_INJECTION,
    pub HostIdentifier: NVME_CDW11_FEATURE_HOST_IDENTIFIER,
    pub ReservationPersistence: NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE,
    pub ReservationNotificationMask: NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK,
    pub GetHostMetadata: NVME_CDW11_FEATURE_GET_HOST_METADATA,
    pub SetHostMetadata: NVME_CDW11_FEATURE_SET_HOST_METADATA,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ARBITRATION {
    pub Anonymous: NVME_CDW11_FEATURE_ARBITRATION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ARBITRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ARBITRATION_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG {
    pub Anonymous: NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION {
    pub Anonymous: NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY {
    pub Anonymous: NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS {
    pub Anonymous: NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO {
    pub Anonymous: NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ERROR_RECOVERY {
    pub Anonymous: NVME_CDW11_FEATURE_ERROR_RECOVERY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ERROR_RECOVERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_ERROR_RECOVERY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_GET_HOST_METADATA {
    pub Anonymous: NVME_CDW11_FEATURE_GET_HOST_METADATA_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_GET_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_GET_HOST_METADATA_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_HOST_IDENTIFIER {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_INTERRUPT_COALESCING {
    pub Anonymous: NVME_CDW11_FEATURE_INTERRUPT_COALESCING_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_INTERRUPT_COALESCING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_INTERRUPT_COALESCING_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG {
    pub Anonymous: NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE {
    pub Anonymous: NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_LBA_RANGE_TYPE {
    pub Anonymous: NVME_CDW11_FEATURE_LBA_RANGE_TYPE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_LBA_RANGE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_LBA_RANGE_TYPE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE {
    pub Anonymous: NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_NUMBER_OF_QUEUES {
    pub Anonymous: NVME_CDW11_FEATURE_NUMBER_OF_QUEUES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_NUMBER_OF_QUEUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_NUMBER_OF_QUEUES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_POWER_MANAGEMENT {
    pub Anonymous: NVME_CDW11_FEATURE_POWER_MANAGEMENT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_POWER_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_POWER_MANAGEMENT_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE {
    pub Anonymous: NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_SET_HOST_METADATA {
    pub Anonymous: NVME_CDW11_FEATURE_SET_HOST_METADATA_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_SET_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_SET_HOST_METADATA_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY {
    pub Anonymous: NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD {
    pub Anonymous: NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE {
    pub Anonymous: NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL {
    pub Anonymous: NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_FIRMWARE_DOWNLOAD {
    pub OFST: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW11_GET_LOG_PAGE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_IDENTIFY {
    pub Anonymous1: NVME_CDW11_IDENTIFY_0,
    pub Anonymous2: NVME_CDW11_IDENTIFY_1,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_IDENTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_IDENTIFY_0 {
    pub NVMSETID: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_IDENTIFY_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_RESERVATION_REPORT {
    pub Anonymous: NVME_CDW11_RESERVATION_REPORT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_RESERVATION_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_RESERVATION_REPORT_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_SANITIZE {
    pub Anonymous: NVME_CDW11_SANITIZE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_SANITIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_SANITIZE_0 {
    pub OVRPAT: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_SECURITY_RECEIVE {
    pub AL: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW11_SECURITY_SEND {
    pub TL: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_RECEIVE {
    pub AllocateResources: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_DIRECTIVE_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    pub Anonymous: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_SEND {
    pub EnableDirective: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_DIRECTIVE_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
    pub Anonymous: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_FEATURES {
    pub HostMemoryBuffer: NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HSIZE: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW12_GET_LOG_PAGE {
    pub LPOL: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_READ_WRITE {
    pub Anonymous: NVME_CDW12_READ_WRITE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW12_READ_WRITE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_ZONE_APPEND {
    pub Anonymous: NVME_CDW12_ZONE_APPEND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_ZONE_APPEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW12_ZONE_APPEND_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_FEATURES {
    pub HostMemoryBuffer: NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW13_GET_LOG_PAGE {
    pub LPOU: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_READ_WRITE {
    pub Anonymous: NVME_CDW13_READ_WRITE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW13_READ_WRITE_0 {
    pub DSM: NVME_CDW13_READ_WRITE_0_0,
    pub Reserved: u8,
    pub DSPEC: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW13_READ_WRITE_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_ZONE_MANAGEMENT_RECEIVE {
    pub Anonymous: NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_ZONE_MANAGEMENT_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_ZONE_MANAGEMENT_SEND {
    pub Anonymous: NVME_CDW13_ZONE_MANAGEMENT_SEND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_ZONE_MANAGEMENT_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW13_ZONE_MANAGEMENT_SEND_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW14_FEATURES {
    pub HostMemoryBuffer: NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}
impl Default for NVME_CDW14_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HMDLUA: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW14_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW14_GET_LOG_PAGE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW14_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW14_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_FEATURES {
    pub HostMemoryBuffer: NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HMDLEC: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_READ_WRITE {
    pub Anonymous: NVME_CDW15_READ_WRITE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW15_READ_WRITE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_ZONE_APPEND {
    pub Anonymous: NVME_CDW15_ZONE_APPEND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_ZONE_APPEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CDW15_ZONE_APPEND_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_CHANGED_NAMESPACE_LIST_LOG {
    pub NSID: [u32; 1024],
}
impl Default for NVME_CHANGED_NAMESPACE_LIST_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_CHANGED_ZONE_LIST_LOG {
    pub ZoneIdentifiersCount: u16,
    pub Reserved: [u8; 6],
    pub ZoneIdentifier: [u64; 511],
}
impl Default for NVME_CHANGED_ZONE_LIST_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_CMBSZ_SIZE_UNITS = i32;
pub const NVME_CMBSZ_SIZE_UNITS_16MB: NVME_CMBSZ_SIZE_UNITS = 3i32;
pub const NVME_CMBSZ_SIZE_UNITS_1MB: NVME_CMBSZ_SIZE_UNITS = 2i32;
pub const NVME_CMBSZ_SIZE_UNITS_256MB: NVME_CMBSZ_SIZE_UNITS = 4i32;
pub const NVME_CMBSZ_SIZE_UNITS_4GB: NVME_CMBSZ_SIZE_UNITS = 5i32;
pub const NVME_CMBSZ_SIZE_UNITS_4KB: NVME_CMBSZ_SIZE_UNITS = 0i32;
pub const NVME_CMBSZ_SIZE_UNITS_64GB: NVME_CMBSZ_SIZE_UNITS = 6i32;
pub const NVME_CMBSZ_SIZE_UNITS_64KB: NVME_CMBSZ_SIZE_UNITS = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND {
    pub CDW0: NVME_COMMAND_DWORD0,
    pub NSID: u32,
    pub Reserved0: [u32; 2],
    pub MPTR: u64,
    pub PRP1: u64,
    pub PRP2: u64,
    pub u: NVME_COMMAND_0,
}
impl Default for NVME_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_0 {
    pub GENERAL: NVME_COMMAND_0_0,
    pub IDENTIFY: NVME_COMMAND_0_1,
    pub ABORT: NVME_COMMAND_0_2,
    pub GETFEATURES: NVME_COMMAND_0_3,
    pub SETFEATURES: NVME_COMMAND_0_4,
    pub GETLOGPAGE: NVME_COMMAND_0_5,
    pub CREATEIOCQ: NVME_COMMAND_0_6,
    pub CREATEIOSQ: NVME_COMMAND_0_7,
    pub DATASETMANAGEMENT: NVME_COMMAND_0_8,
    pub SECURITYSEND: NVME_COMMAND_0_9,
    pub SECURITYRECEIVE: NVME_COMMAND_0_10,
    pub FIRMWAREDOWNLOAD: NVME_COMMAND_0_11,
    pub FIRMWAREACTIVATE: NVME_COMMAND_0_12,
    pub FORMATNVM: NVME_COMMAND_0_13,
    pub DIRECTIVERECEIVE: NVME_COMMAND_0_14,
    pub DIRECTIVESEND: NVME_COMMAND_0_15,
    pub SANITIZE: NVME_COMMAND_0_16,
    pub READWRITE: NVME_COMMAND_0_17,
    pub RESERVATIONACQUIRE: NVME_COMMAND_0_18,
    pub RESERVATIONREGISTER: NVME_COMMAND_0_19,
    pub RESERVATIONRELEASE: NVME_COMMAND_0_20,
    pub RESERVATIONREPORT: NVME_COMMAND_0_21,
    pub ZONEMANAGEMENTSEND: NVME_COMMAND_0_22,
    pub ZONEMANAGEMENTRECEIVE: NVME_COMMAND_0_23,
    pub ZONEAPPEND: NVME_COMMAND_0_24,
}
impl Default for NVME_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_2 {
    pub CDW10: NVME_CDW10_ABORT,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_6 {
    pub CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    pub CDW11: NVME_CDW11_CREATE_IO_CQ,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_7 {
    pub CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    pub CDW11: NVME_CDW11_CREATE_IO_SQ,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_8 {
    pub CDW10: NVME_CDW10_DATASET_MANAGEMENT,
    pub CDW11: NVME_CDW11_DATASET_MANAGEMENT,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_14 {
    pub CDW10: NVME_CDW10_DIRECTIVE_RECEIVE,
    pub CDW11: NVME_CDW11_DIRECTIVE_RECEIVE,
    pub CDW12: NVME_CDW12_DIRECTIVE_RECEIVE,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_15 {
    pub CDW10: NVME_CDW10_DIRECTIVE_SEND,
    pub CDW11: NVME_CDW11_DIRECTIVE_SEND,
    pub CDW12: NVME_CDW12_DIRECTIVE_SEND,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_12 {
    pub CDW10: NVME_CDW10_FIRMWARE_ACTIVATE,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND_0_11 {
    pub CDW10: NVME_CDW10_FIRMWARE_DOWNLOAD,
    pub CDW11: NVME_CDW11_FIRMWARE_DOWNLOAD,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_13 {
    pub CDW10: NVME_CDW10_FORMAT_NVM,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND_0_0 {
    pub CDW10: u32,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_3 {
    pub CDW10: NVME_CDW10_GET_FEATURES,
    pub CDW11: NVME_CDW11_FEATURES,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_5 {
    pub Anonymous: NVME_COMMAND_0_5_0,
    pub CDW11: NVME_CDW11_GET_LOG_PAGE,
    pub CDW12: NVME_CDW12_GET_LOG_PAGE,
    pub CDW13: NVME_CDW13_GET_LOG_PAGE,
    pub CDW14: NVME_CDW14_GET_LOG_PAGE,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_0_5_0 {
    pub CDW10: NVME_CDW10_GET_LOG_PAGE,
    pub CDW10_V13: NVME_CDW10_GET_LOG_PAGE_V13,
}
impl Default for NVME_COMMAND_0_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_1 {
    pub CDW10: NVME_CDW10_IDENTIFY,
    pub CDW11: NVME_CDW11_IDENTIFY,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_17 {
    pub LBALOW: u32,
    pub LBAHIGH: u32,
    pub CDW12: NVME_CDW12_READ_WRITE,
    pub CDW13: NVME_CDW13_READ_WRITE,
    pub CDW14: u32,
    pub CDW15: NVME_CDW15_READ_WRITE,
}
impl Default for NVME_COMMAND_0_17 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_18 {
    pub CDW10: NVME_CDW10_RESERVATION_ACQUIRE,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_19 {
    pub CDW10: NVME_CDW10_RESERVATION_REGISTER,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_20 {
    pub CDW10: NVME_CDW10_RESERVATION_RELEASE,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_21 {
    pub CDW10: NVME_CDW10_RESERVATION_REPORT,
    pub CDW11: NVME_CDW11_RESERVATION_REPORT,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_16 {
    pub CDW10: NVME_CDW10_SANITIZE,
    pub CDW11: NVME_CDW11_SANITIZE,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_10 {
    pub CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    pub CDW11: NVME_CDW11_SECURITY_RECEIVE,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_9 {
    pub CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    pub CDW11: NVME_CDW11_SECURITY_SEND,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_4 {
    pub CDW10: NVME_CDW10_SET_FEATURES,
    pub CDW11: NVME_CDW11_FEATURES,
    pub CDW12: NVME_CDW12_FEATURES,
    pub CDW13: NVME_CDW13_FEATURES,
    pub CDW14: NVME_CDW14_FEATURES,
    pub CDW15: NVME_CDW15_FEATURES,
}
impl Default for NVME_COMMAND_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_24 {
    pub CDW1011: NVME_CDW10_ZONE_APPEND,
    pub CDW12: NVME_CDW12_ZONE_APPEND,
    pub CDW13: u32,
    pub ILBRT: u32,
    pub CDW15: NVME_CDW15_ZONE_APPEND,
}
impl Default for NVME_COMMAND_0_24 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_23 {
    pub CDW1011: NVME_CDW10_ZONE_MANAGEMENT_RECEIVE,
    pub DWORDCOUNT: u32,
    pub CDW13: NVME_CDW13_ZONE_MANAGEMENT_RECEIVE,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_23 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_0_22 {
    pub CDW1011: NVME_CDW10_ZONE_MANAGEMENT_SEND,
    pub CDW12: u32,
    pub CDW13: NVME_CDW13_ZONE_MANAGEMENT_SEND,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_0_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_DWORD0 {
    pub Anonymous: NVME_COMMAND_DWORD0_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMMAND_DWORD0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND_DWORD0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_EFFECTS_DATA {
    pub Anonymous: NVME_COMMAND_EFFECTS_DATA_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMMAND_EFFECTS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND_EFFECTS_DATA_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_EFFECTS_LOG {
    pub ACS: [NVME_COMMAND_EFFECTS_DATA; 256],
    pub IOCS: [NVME_COMMAND_EFFECTS_DATA; 256],
    pub Reserved: [u8; 2048],
}
impl Default for NVME_COMMAND_EFFECTS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS = i32;
pub const NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_NONE: NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS = 0i32;
pub const NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_SINGLE_PER_CONTROLLER: NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS = 2i32;
pub const NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_SINGLE_PER_NAMESPACE: NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS = 1i32;
pub type NVME_COMMAND_SET_IDENTIFIERS = i32;
pub const NVME_COMMAND_SET_KEY_VALUE: NVME_COMMAND_SET_IDENTIFIERS = 1i32;
pub const NVME_COMMAND_SET_NVM: NVME_COMMAND_SET_IDENTIFIERS = 0i32;
pub const NVME_COMMAND_SET_ZONED_NAMESPACE: NVME_COMMAND_SET_IDENTIFIERS = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_STATUS {
    pub Anonymous: NVME_COMMAND_STATUS_0,
    pub AsUshort: u16,
}
impl Default for NVME_COMMAND_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMMAND_STATUS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    pub Anonymous: NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0,
    pub AsUlong: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMPLETION_ENTRY {
    pub DW0: u32,
    pub DW1: u32,
    pub DW2: NVME_COMPLETION_ENTRY_0,
    pub DW3: NVME_COMPLETION_ENTRY_1,
}
impl Default for NVME_COMPLETION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_ENTRY_0 {
    pub Anonymous: NVME_COMPLETION_ENTRY_0_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMPLETION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMPLETION_ENTRY_0_0 {
    pub SQHD: u16,
    pub SQID: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_ENTRY_1 {
    pub Anonymous: NVME_COMPLETION_ENTRY_1_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMPLETION_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMPLETION_ENTRY_1_0 {
    pub CID: u16,
    pub Status: NVME_COMMAND_STATUS,
}
impl Default for NVME_COMPLETION_ENTRY_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_QUEUE_HEAD_DOORBELL {
    pub Anonymous: NVME_COMPLETION_QUEUE_HEAD_DOORBELL_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMPLETION_QUEUE_HEAD_DOORBELL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_COMPLETION_QUEUE_HEAD_DOORBELL_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTEXT_ATTRIBUTES {
    pub Anonymous: NVME_CONTEXT_ATTRIBUTES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CONTEXT_ATTRIBUTES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_CAPABILITIES {
    pub Anonymous: NVME_CONTROLLER_CAPABILITIES_0,
    pub AsUlonglong: u64,
}
impl Default for NVME_CONTROLLER_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CONTROLLER_CAPABILITIES_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_CONFIGURATION {
    pub Anonymous: NVME_CONTROLLER_CONFIGURATION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CONTROLLER_CONFIGURATION_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_CONTROLLER_LIST {
    pub NumberOfIdentifiers: u16,
    pub ControllerID: [u16; 2047],
}
impl Default for NVME_CONTROLLER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_MEMORY_BUFFER_LOCATION {
    pub Anonymous: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_MEMORY_BUFFER_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_MEMORY_BUFFER_SIZE {
    pub Anonymous: NVME_CONTROLLER_MEMORY_BUFFER_SIZE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_MEMORY_BUFFER_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CONTROLLER_MEMORY_BUFFER_SIZE_0 {
    pub _bitfield: u32,
}
pub const NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 8i32;
pub const NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 9i32;
pub const NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 14i32;
pub const NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 15i32;
pub type NVME_CONTROLLER_METADATA_ELEMENT_TYPES = i32;
pub const NVME_CONTROLLER_METADATA_FIRMWARE_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 12i32;
pub const NVME_CONTROLLER_METADATA_HOST_DETERMINED_FAILURE_RECORD: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 16i32;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_CONTROLLER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 1i32;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_FILENAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 13i32;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 2i32;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 3i32;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_NAME_AND_BUILD: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 10i32;
pub const NVME_CONTROLLER_METADATA_PREBOOT_CONTROLLER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 4i32;
pub const NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 5i32;
pub const NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 6i32;
pub const NVME_CONTROLLER_METADATA_SYSTEM_PROCESSOR_MODEL: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 7i32;
pub const NVME_CONTROLLER_METADATA_SYSTEM_PRODUCT_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 11i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_CONTROLLER_REGISTERS {
    pub CAP: NVME_CONTROLLER_CAPABILITIES,
    pub VS: NVME_VERSION,
    pub INTMS: u32,
    pub INTMC: u32,
    pub CC: NVME_CONTROLLER_CONFIGURATION,
    pub Reserved0: u32,
    pub CSTS: NVME_CONTROLLER_STATUS,
    pub NSSR: NVME_NVM_SUBSYSTEM_RESET,
    pub AQA: NVME_ADMIN_QUEUE_ATTRIBUTES,
    pub ASQ: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS,
    pub ACQ: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS,
    pub CMBLOC: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION,
    pub CMBSZ: NVME_CONTROLLER_MEMORY_BUFFER_SIZE,
    pub Reserved2: [u32; 944],
    pub Reserved3: [u32; 64],
    pub Doorbells: [u32; 1],
}
impl Default for NVME_CONTROLLER_REGISTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_STATUS {
    pub Anonymous: NVME_CONTROLLER_STATUS_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_CONTROLLER_STATUS_0 {
    pub _bitfield: u32,
}
pub const NVME_CSS_ADMIN_COMMAND_SET_ONLY: NVME_CSS_COMMAND_SETS = 7i32;
pub const NVME_CSS_ALL_SUPPORTED_IO_COMMAND_SET: NVME_CSS_COMMAND_SETS = 6i32;
pub type NVME_CSS_COMMAND_SETS = i32;
pub const NVME_CSS_NVM_COMMAND_SET: NVME_CSS_COMMAND_SETS = 0i32;
pub const NVME_CSTS_SHST_NO_SHUTDOWN: NVME_CSTS_SHST_SHUTDOWN_STATUS = 0i32;
pub const NVME_CSTS_SHST_SHUTDOWN_COMPLETED: NVME_CSTS_SHST_SHUTDOWN_STATUS = 2i32;
pub const NVME_CSTS_SHST_SHUTDOWN_IN_PROCESS: NVME_CSTS_SHST_SHUTDOWN_STATUS = 1i32;
pub type NVME_CSTS_SHST_SHUTDOWN_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_DEVICE_SELF_TEST_LOG {
    pub CurrentOperation: NVME_DEVICE_SELF_TEST_LOG_0,
    pub CurrentCompletion: NVME_DEVICE_SELF_TEST_LOG_1,
    pub Reserved: [u8; 2],
    pub ResultData: [NVME_DEVICE_SELF_TEST_RESULT_DATA; 20],
}
impl Default for NVME_DEVICE_SELF_TEST_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_LOG_1 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_LOG_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA {
    pub Status: NVME_DEVICE_SELF_TEST_RESULT_DATA_0,
    pub SegmentNumber: u8,
    pub ValidDiagnostics: NVME_DEVICE_SELF_TEST_RESULT_DATA_1,
    pub Reserved: u8,
    pub POH: u64,
    pub NSID: u32,
    pub FailingLBA: u64,
    pub StatusCodeType: NVME_DEVICE_SELF_TEST_RESULT_DATA_2,
    pub StatusCode: u8,
    pub VendorSpecific: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_2 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_1 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS {
    pub DirectivesSupported: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
    pub DirectivesEnabled: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    pub _bitfield: u8,
    pub Reserved1: [u8; 31],
}
impl Default for NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATIONS = i32;
pub const NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATION_RETURN_PARAMETERS: NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATIONS = 1i32;
pub type NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS = i32;
pub const NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_ALLOCATE_RESOURCES: NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS = 3i32;
pub const NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_GET_STATUS: NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS = 2i32;
pub const NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_RETURN_PARAMETERS: NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS = 1i32;
pub type NVME_DIRECTIVE_SEND_IDENTIFY_OPERATIONS = i32;
pub const NVME_DIRECTIVE_SEND_IDENTIFY_OPERATION_ENABLE_DIRECTIVE: NVME_DIRECTIVE_SEND_IDENTIFY_OPERATIONS = 1i32;
pub type NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS = i32;
pub const NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_IDENTIFIER: NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS = 1i32;
pub const NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_RESOURCES: NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    pub OpenStreamCount: u16,
    pub StreamIdentifiers: [u16; 65535],
}
impl Default for NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS {
    pub MSL: u16,
    pub NSSA: u16,
    pub NSSO: u16,
    pub Reserved0: [u8; 10],
    pub SWS: u32,
    pub SGS: u16,
    pub NSA: u16,
    pub NSO: u16,
    pub Reserved1: [u8; 6],
}
impl Default for NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_DIRECTIVE_TYPES = i32;
pub const NVME_DIRECTIVE_TYPE_IDENTIFY: NVME_DIRECTIVE_TYPES = 0i32;
pub const NVME_DIRECTIVE_TYPE_STREAMS: NVME_DIRECTIVE_TYPES = 1i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_ENDURANCE_GROUP_LOG {
    pub Reserved0: u32,
    pub AvailableSpareThreshold: u8,
    pub PercentageUsed: u8,
    pub Reserved1: [u8; 26],
    pub EnduranceEstimate: [u8; 16],
    pub DataUnitsRead: [u8; 16],
    pub DataUnitsWritten: [u8; 16],
    pub MediaUnitsWritten: [u8; 16],
    pub Reserved2: [u8; 416],
}
impl Default for NVME_ENDURANCE_GROUP_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_ERROR_INFO_LOG {
    pub ErrorCount: u64,
    pub SQID: u16,
    pub CMDID: u16,
    pub Status: NVME_COMMAND_STATUS,
    pub ParameterErrorLocation: NVME_ERROR_INFO_LOG_0,
    pub Lba: u64,
    pub NameSpace: u32,
    pub VendorInfoAvailable: u8,
    pub Reserved0: [u8; 3],
    pub CommandSpecificInfo: u64,
    pub Reserved1: [u8; 24],
}
impl Default for NVME_ERROR_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_ERROR_INFO_LOG_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_ERROR_INJECTION_ENTRY {
    pub Flags: NVME_ERROR_INJECTION_ENTRY_0,
    pub Reserved1: u8,
    pub ErrorInjectionType: u16,
    pub ErrorInjectionTypeSpecific: [u8; 28],
}
impl Default for NVME_ERROR_INJECTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_ERROR_INJECTION_ENTRY_0 {
    pub Anonymous: NVME_ERROR_INJECTION_ENTRY_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_ERROR_INJECTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_ERROR_INJECTION_ENTRY_0_0 {
    pub _bitfield: u8,
}
pub type NVME_ERROR_INJECTION_TYPES = i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_CPU_CONTROLLER_HANG: NVME_ERROR_INJECTION_TYPES = 1i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_CRITICAL: NVME_ERROR_INJECTION_TYPES = 5i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_NONCRITICAL: NVME_ERROR_INJECTION_TYPES = 6i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_HW_MALFUNCTION: NVME_ERROR_INJECTION_TYPES = 9i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_LOGICAL_FW_ERROR: NVME_ERROR_INJECTION_TYPES = 4i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_CORRUPTION: NVME_ERROR_INJECTION_TYPES = 7i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_HANG: NVME_ERROR_INJECTION_TYPES = 2i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_PLP_DEFECT: NVME_ERROR_INJECTION_TYPES = 3i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_SRAM_CORRUPTION: NVME_ERROR_INJECTION_TYPES = 8i32;
pub const NVME_ERROR_INJECTION_TYPE_MAX: NVME_ERROR_INJECTION_TYPES = 65535i32;
pub const NVME_ERROR_INJECTION_TYPE_RESERVED0: NVME_ERROR_INJECTION_TYPES = 0i32;
pub const NVME_ERROR_INJECTION_TYPE_RESERVED1: NVME_ERROR_INJECTION_TYPES = 10i32;
pub const NVME_EXTENDED_HOST_IDENTIFIER_SIZE: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_EXTENDED_REPORT_ZONE_INFO {
    pub ZoneCount: u64,
    pub Reserved: [u64; 7],
    pub Desc: [NVME_ZONE_EXTENDED_REPORT_ZONE_DESC; 1],
}
impl Default for NVME_EXTENDED_REPORT_ZONE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_FEATURES = i32;
pub const NVME_FEATURE_ARBITRATION: NVME_FEATURES = 1i32;
pub const NVME_FEATURE_ASYNC_EVENT_CONFIG: NVME_FEATURES = 11i32;
pub const NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION: NVME_FEATURES = 12i32;
pub const NVME_FEATURE_CLEAR_FW_UPDATE_HISTORY: NVME_FEATURES = 193i32;
pub const NVME_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS: NVME_FEATURES = 195i32;
pub const NVME_FEATURE_CONTROLLER_METADATA: NVME_FEATURES = 126i32;
pub const NVME_FEATURE_ENABLE_IEEE1667_SILO: NVME_FEATURES = 196i32;
pub const NVME_FEATURE_ENDURANCE_GROUP_EVENT_CONFIG: NVME_FEATURES = 24i32;
pub const NVME_FEATURE_ENHANCED_CONTROLLER_METADATA: NVME_FEATURES = 125i32;
pub const NVME_FEATURE_ERROR_INJECTION: NVME_FEATURES = 192i32;
pub const NVME_FEATURE_ERROR_RECOVERY: NVME_FEATURES = 5i32;
pub const NVME_FEATURE_HOST_BEHAVIOR_SUPPORT: NVME_FEATURES = 22i32;
pub const NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT: NVME_FEATURES = 16i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_FEATURE_HOST_IDENTIFIER_DATA {
    pub HOSTID: [u8; 16],
}
impl Default for NVME_FEATURE_HOST_IDENTIFIER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_FEATURE_HOST_MEMORY_BUFFER: NVME_FEATURES = 13i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_FEATURE_HOST_METADATA_DATA {
    pub NumberOfMetadataElementDescriptors: u8,
    pub Reserved0: u8,
    pub MetadataElementDescriptors: [u8; 4094],
}
impl Default for NVME_FEATURE_HOST_METADATA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_FEATURE_INTERRUPT_COALESCING: NVME_FEATURES = 8i32;
pub const NVME_FEATURE_INTERRUPT_VECTOR_CONFIG: NVME_FEATURES = 9i32;
pub const NVME_FEATURE_IO_COMMAND_SET_PROFILE: NVME_FEATURES = 25i32;
pub const NVME_FEATURE_KEEP_ALIVE: NVME_FEATURES = 15i32;
pub const NVME_FEATURE_LBA_RANGE_TYPE: NVME_FEATURES = 3i32;
pub const NVME_FEATURE_LBA_STATUS_INFORMATION_REPORT_INTERVAL: NVME_FEATURES = 21i32;
pub const NVME_FEATURE_NAMESPACE_METADATA: NVME_FEATURES = 127i32;
pub const NVME_FEATURE_NONOPERATIONAL_POWER_STATE: NVME_FEATURES = 17i32;
pub const NVME_FEATURE_NUMBER_OF_QUEUES: NVME_FEATURES = 7i32;
pub const NVME_FEATURE_NVM_HOST_IDENTIFIER: NVME_FEATURES = 129i32;
pub const NVME_FEATURE_NVM_NAMESPACE_WRITE_PROTECTION_CONFIG: NVME_FEATURES = 132i32;
pub const NVME_FEATURE_NVM_RESERVATION_NOTIFICATION_MASK: NVME_FEATURES = 130i32;
pub const NVME_FEATURE_NVM_RESERVATION_PERSISTANCE: NVME_FEATURES = 131i32;
pub const NVME_FEATURE_NVM_SOFTWARE_PROGRESS_MARKER: NVME_FEATURES = 128i32;
pub const NVME_FEATURE_PLP_HEALTH_MONITOR: NVME_FEATURES = 197i32;
pub const NVME_FEATURE_POWER_MANAGEMENT: NVME_FEATURES = 2i32;
pub const NVME_FEATURE_PREDICTABLE_LATENCY_MODE_CONFIG: NVME_FEATURES = 19i32;
pub const NVME_FEATURE_PREDICTABLE_LATENCY_MODE_WINDOW: NVME_FEATURES = 20i32;
pub const NVME_FEATURE_READONLY_WRITETHROUGH_MODE: NVME_FEATURES = 194i32;
pub const NVME_FEATURE_READ_RECOVERY_LEVEL_CONFIG: NVME_FEATURES = 18i32;
pub const NVME_FEATURE_SANITIZE_CONFIG: NVME_FEATURES = 23i32;
pub const NVME_FEATURE_TEMPERATURE_THRESHOLD: NVME_FEATURES = 4i32;
pub const NVME_FEATURE_TIMESTAMP: NVME_FEATURES = 14i32;
pub type NVME_FEATURE_VALUE_CODES = i32;
pub const NVME_FEATURE_VALUE_CURRENT: NVME_FEATURE_VALUE_CODES = 0i32;
pub const NVME_FEATURE_VALUE_DEFAULT: NVME_FEATURE_VALUE_CODES = 1i32;
pub const NVME_FEATURE_VALUE_SAVED: NVME_FEATURE_VALUE_CODES = 2i32;
pub const NVME_FEATURE_VALUE_SUPPORTED_CAPABILITIES: NVME_FEATURE_VALUE_CODES = 3i32;
pub const NVME_FEATURE_VOLATILE_WRITE_CACHE: NVME_FEATURES = 6i32;
pub const NVME_FEATURE_WRITE_ATOMICITY: NVME_FEATURES = 10i32;
pub type NVME_FIRMWARE_ACTIVATE_ACTIONS = i32;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_ACTIVATE: NVME_FIRMWARE_ACTIVATE_ACTIONS = 2i32;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT: NVME_FIRMWARE_ACTIVATE_ACTIONS = 0i32;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE: NVME_FIRMWARE_ACTIVATE_ACTIONS = 1i32;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE_IMMEDIATE: NVME_FIRMWARE_ACTIVATE_ACTIONS = 3i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_FIRMWARE_SLOT_INFO_LOG {
    pub AFI: NVME_FIRMWARE_SLOT_INFO_LOG_0,
    pub Reserved0: [u8; 7],
    pub FRS: [u64; 7],
    pub Reserved1: [u8; 448],
}
impl Default for NVME_FIRMWARE_SLOT_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_FIRMWARE_SLOT_INFO_LOG_0 {
    pub _bitfield: u8,
}
pub type NVME_FUSED_OPERATION_CODES = i32;
pub const NVME_FUSED_OPERATION_FIRST_CMD: NVME_FUSED_OPERATION_CODES = 1i32;
pub const NVME_FUSED_OPERATION_NORMAL: NVME_FUSED_OPERATION_CODES = 0i32;
pub const NVME_FUSED_OPERATION_SECOND_CMD: NVME_FUSED_OPERATION_CODES = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_HEALTH_INFO_LOG {
    pub CriticalWarning: NVME_HEALTH_INFO_LOG_0,
    pub Temperature: [u8; 2],
    pub AvailableSpare: u8,
    pub AvailableSpareThreshold: u8,
    pub PercentageUsed: u8,
    pub Reserved0: [u8; 26],
    pub DataUnitRead: [u8; 16],
    pub DataUnitWritten: [u8; 16],
    pub HostReadCommands: [u8; 16],
    pub HostWrittenCommands: [u8; 16],
    pub ControllerBusyTime: [u8; 16],
    pub PowerCycle: [u8; 16],
    pub PowerOnHours: [u8; 16],
    pub UnsafeShutdowns: [u8; 16],
    pub MediaErrors: [u8; 16],
    pub ErrorInfoLogEntryCount: [u8; 16],
    pub WarningCompositeTemperatureTime: u32,
    pub CriticalCompositeTemperatureTime: u32,
    pub TemperatureSensor1: u16,
    pub TemperatureSensor2: u16,
    pub TemperatureSensor3: u16,
    pub TemperatureSensor4: u16,
    pub TemperatureSensor5: u16,
    pub TemperatureSensor6: u16,
    pub TemperatureSensor7: u16,
    pub TemperatureSensor8: u16,
    pub Reserved1: [u8; 296],
}
impl Default for NVME_HEALTH_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_HEALTH_INFO_LOG_0 {
    pub Anonymous: NVME_HEALTH_INFO_LOG_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_HEALTH_INFO_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_HEALTH_INFO_LOG_0_0 {
    pub _bitfield: u8,
}
pub const NVME_HOST_IDENTIFIER_SIZE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY {
    pub BADD: u64,
    pub BSIZE: u32,
    pub Reserved: u32,
}
pub const NVME_HOST_METADATA_ADD_ENTRY_MULTIPLE: NVME_HOST_METADATA_ELEMENT_ACTIONS = 2i32;
pub const NVME_HOST_METADATA_ADD_REPLACE_ENTRY: NVME_HOST_METADATA_ELEMENT_ACTIONS = 0i32;
pub const NVME_HOST_METADATA_DELETE_ENTRY_MULTIPLE: NVME_HOST_METADATA_ELEMENT_ACTIONS = 1i32;
pub type NVME_HOST_METADATA_ELEMENT_ACTIONS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    pub _bitfield: u32,
    pub EVAL: [u8; 1],
}
impl Default for NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_IDENTIFIER_TYPE = i32;
pub const NVME_IDENTIFIER_TYPE_CSI: NVME_IDENTIFIER_TYPE = 4i32;
pub const NVME_IDENTIFIER_TYPE_CSI_LENGTH: NVME_IDENTIFIER_TYPE_LENGTH = 1i32;
pub const NVME_IDENTIFIER_TYPE_EUI64: NVME_IDENTIFIER_TYPE = 1i32;
pub const NVME_IDENTIFIER_TYPE_EUI64_LENGTH: NVME_IDENTIFIER_TYPE_LENGTH = 8i32;
pub type NVME_IDENTIFIER_TYPE_LENGTH = i32;
pub const NVME_IDENTIFIER_TYPE_NGUID: NVME_IDENTIFIER_TYPE = 2i32;
pub const NVME_IDENTIFIER_TYPE_NGUID_LENGTH: NVME_IDENTIFIER_TYPE_LENGTH = 16i32;
pub const NVME_IDENTIFIER_TYPE_UUID: NVME_IDENTIFIER_TYPE = 3i32;
pub const NVME_IDENTIFIER_TYPE_UUID_LENGTH: NVME_IDENTIFIER_TYPE_LENGTH = 16i32;
pub const NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES: NVME_IDENTIFY_CNS_CODES = 2i32;
pub const NVME_IDENTIFY_CNS_ACTIVE_NAMESPACE_LIST_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 7i32;
pub const NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE: NVME_IDENTIFY_CNS_CODES = 17i32;
pub const NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 27i32;
pub const NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_LIST: NVME_IDENTIFY_CNS_CODES = 16i32;
pub const NVME_IDENTIFY_CNS_ALLOCATED_NAMSPACE_LIST_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 26i32;
pub type NVME_IDENTIFY_CNS_CODES = i32;
pub const NVME_IDENTIFY_CNS_CONTROLLER: NVME_IDENTIFY_CNS_CODES = 1i32;
pub const NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NSID: NVME_IDENTIFY_CNS_CODES = 18i32;
pub const NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NVM_SUBSYSTEM: NVME_IDENTIFY_CNS_CODES = 19i32;
pub const NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE: NVME_IDENTIFY_CNS_CODES = 3i32;
pub const NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE_SIZE: u32 = 4096u32;
pub const NVME_IDENTIFY_CNS_DOMAIN_LIST: NVME_IDENTIFY_CNS_CODES = 24i32;
pub const NVME_IDENTIFY_CNS_ENDURANCE_GROUP_LIST: NVME_IDENTIFY_CNS_CODES = 25i32;
pub const NVME_IDENTIFY_CNS_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 28i32;
pub const NVME_IDENTIFY_CNS_NAMESPACE_GRANULARITY_LIST: NVME_IDENTIFY_CNS_CODES = 22i32;
pub const NVME_IDENTIFY_CNS_NVM_SET: NVME_IDENTIFY_CNS_CODES = 4i32;
pub const NVME_IDENTIFY_CNS_PRIMARY_CONTROLLER_CAPABILITIES: NVME_IDENTIFY_CNS_CODES = 20i32;
pub const NVME_IDENTIFY_CNS_SECONDARY_CONTROLLER_LIST: NVME_IDENTIFY_CNS_CODES = 21i32;
pub const NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 6i32;
pub const NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE: NVME_IDENTIFY_CNS_CODES = 0i32;
pub const NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 5i32;
pub const NVME_IDENTIFY_CNS_UUID_LIST: NVME_IDENTIFY_CNS_CODES = 23i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA {
    pub VID: u16,
    pub SSVID: u16,
    pub SN: [u8; 20],
    pub MN: [u8; 40],
    pub FR: [u8; 8],
    pub RAB: u8,
    pub IEEE: [u8; 3],
    pub CMIC: NVME_IDENTIFY_CONTROLLER_DATA_0,
    pub MDTS: u8,
    pub CNTLID: u16,
    pub VER: u32,
    pub RTD3R: u32,
    pub RTD3E: u32,
    pub OAES: NVME_IDENTIFY_CONTROLLER_DATA_1,
    pub CTRATT: NVME_IDENTIFY_CONTROLLER_DATA_2,
    pub RRLS: NVME_IDENTIFY_CONTROLLER_DATA_3,
    pub Reserved0: [u8; 9],
    pub CNTRLTYPE: u8,
    pub FGUID: [u8; 16],
    pub CRDT1: u16,
    pub CRDT2: u16,
    pub CRDT3: u16,
    pub Reserved0_1: [u8; 106],
    pub ReservedForManagement: [u8; 16],
    pub OACS: NVME_IDENTIFY_CONTROLLER_DATA_4,
    pub ACL: u8,
    pub AERL: u8,
    pub FRMW: NVME_IDENTIFY_CONTROLLER_DATA_5,
    pub LPA: NVME_IDENTIFY_CONTROLLER_DATA_6,
    pub ELPE: u8,
    pub NPSS: u8,
    pub AVSCC: NVME_IDENTIFY_CONTROLLER_DATA_7,
    pub APSTA: NVME_IDENTIFY_CONTROLLER_DATA_8,
    pub WCTEMP: u16,
    pub CCTEMP: u16,
    pub MTFA: u16,
    pub HMPRE: u32,
    pub HMMIN: u32,
    pub TNVMCAP: [u8; 16],
    pub UNVMCAP: [u8; 16],
    pub RPMBS: NVME_IDENTIFY_CONTROLLER_DATA_9,
    pub EDSTT: u16,
    pub DSTO: u8,
    pub FWUG: u8,
    pub KAS: u16,
    pub HCTMA: NVME_IDENTIFY_CONTROLLER_DATA_10,
    pub MNTMT: u16,
    pub MXTMT: u16,
    pub SANICAP: NVME_IDENTIFY_CONTROLLER_DATA_11,
    pub HMMINDS: u32,
    pub HMMAXD: u16,
    pub NSETIDMAX: u16,
    pub ENDGIDMAX: u16,
    pub ANATT: u8,
    pub ANACAP: NVME_IDENTIFY_CONTROLLER_DATA_12,
    pub ANAGRPMAX: u32,
    pub NANAGRPID: u32,
    pub PELS: u32,
    pub Reserved1: [u8; 156],
    pub SQES: NVME_IDENTIFY_CONTROLLER_DATA_13,
    pub CQES: NVME_IDENTIFY_CONTROLLER_DATA_14,
    pub MAXCMD: u16,
    pub NN: u32,
    pub ONCS: NVME_IDENTIFY_CONTROLLER_DATA_15,
    pub FUSES: NVME_IDENTIFY_CONTROLLER_DATA_16,
    pub FNA: NVME_IDENTIFY_CONTROLLER_DATA_17,
    pub VWC: NVME_IDENTIFY_CONTROLLER_DATA_18,
    pub AWUN: u16,
    pub AWUPF: u16,
    pub NVSCC: NVME_IDENTIFY_CONTROLLER_DATA_19,
    pub NWPC: NVME_IDENTIFY_CONTROLLER_DATA_20,
    pub ACWU: u16,
    pub Reserved4: [u8; 2],
    pub SGLS: NVME_IDENTIFY_CONTROLLER_DATA_21,
    pub MNAN: u32,
    pub Reserved6: [u8; 224],
    pub SUBNQN: [u8; 256],
    pub Reserved7: [u8; 768],
    pub Reserved8: [u8; 256],
    pub PDS: [NVME_POWER_STATE_DESC; 32],
    pub VS: [u8; 1024],
}
impl Default for NVME_IDENTIFY_CONTROLLER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_12 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_8 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_7 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_14 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_2 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_17 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_5 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_16 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_10 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_6 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_19 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_20 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_4 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_15 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_9 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_3 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_11 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_21 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_13 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_18 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_IO_COMMAND_SET {
    pub IOCommandSetVector: [u64; 512],
}
impl Default for NVME_IDENTIFY_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA {
    pub NSZE: u64,
    pub NCAP: u64,
    pub NUSE: u64,
    pub NSFEAT: NVME_IDENTIFY_NAMESPACE_DATA_0,
    pub NLBAF: u8,
    pub FLBAS: NVME_IDENTIFY_NAMESPACE_DATA_1,
    pub MC: NVME_IDENTIFY_NAMESPACE_DATA_2,
    pub DPC: NVME_IDENTIFY_NAMESPACE_DATA_3,
    pub DPS: NVME_IDENTIFY_NAMESPACE_DATA_4,
    pub NMIC: NVME_IDENTIFY_NAMESPACE_DATA_5,
    pub RESCAP: NVM_RESERVATION_CAPABILITIES,
    pub FPI: NVME_IDENTIFY_NAMESPACE_DATA_6,
    pub DLFEAT: NVME_IDENTIFY_NAMESPACE_DATA_7,
    pub NAWUN: u16,
    pub NAWUPF: u16,
    pub NACWU: u16,
    pub NABSN: u16,
    pub NABO: u16,
    pub NABSPF: u16,
    pub NOIOB: u16,
    pub NVMCAP: [u8; 16],
    pub NPWG: u16,
    pub NPWA: u16,
    pub NPDG: u16,
    pub NPDA: u16,
    pub NOWS: u16,
    pub MSSRL: u16,
    pub MCL: u32,
    pub MSRC: u8,
    pub Reserved2: [u8; 11],
    pub ANAGRPID: u32,
    pub Reserved3: [u8; 3],
    pub NSATTR: NVME_IDENTIFY_NAMESPACE_DATA_8,
    pub NVMSETID: u16,
    pub ENDGID: u16,
    pub NGUID: [u8; 16],
    pub EUI64: [u8; 8],
    pub LBAF: [NVME_LBA_FORMAT; 16],
    pub Reserved4: [u8; 192],
    pub VS: [u8; 3712],
}
impl Default for NVME_IDENTIFY_NAMESPACE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_7 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_3 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_4 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_1 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_6 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_2 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_5 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_8 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_NAMESPACE_DESCRIPTOR {
    pub NIDT: u8,
    pub NIDL: u8,
    pub Reserved: [u8; 2],
    pub NID: [u8; 1],
}
impl Default for NVME_IDENTIFY_NAMESPACE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    pub VSL: u8,
    pub WZSL: u8,
    pub WUSL: u8,
    pub DMRL: u8,
    pub DMRSL: u32,
    pub DMSL: u64,
    pub Reserved: [u8; 4080],
}
impl Default for NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    pub ZOC: NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_0,
    pub OZCS: NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_1,
    pub MAR: u32,
    pub MOR: u32,
    pub RRL: u32,
    pub FRL: u32,
    pub Reserved0: [u8; 2796],
    pub LBAEF: [NVME_LBA_ZONE_FORMAT; 16],
    pub Reserved1: [u8; 768],
    pub VS: [u8; 256],
}
impl Default for NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_1 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    pub ZASL: u8,
    pub Reserved: [u8; 4095],
}
impl Default for NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_IO_COMMAND_SET_COMBINATION_REJECTED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 43i32;
pub const NVME_IO_COMMAND_SET_INVALID: NVME_STATUS_COMMAND_SPECIFIC_CODES = 44i32;
pub const NVME_IO_COMMAND_SET_NOT_ENABLED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 42i32;
pub const NVME_IO_COMMAND_SET_NOT_SUPPORTED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 41i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_LBA_FORMAT {
    pub Anonymous: NVME_LBA_FORMAT_0,
    pub AsUlong: u32,
}
impl Default for NVME_LBA_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_LBA_FORMAT_0 {
    pub MS: u16,
    pub LBADS: u8,
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_LBA_RANGE {
    pub Attributes: NVME_CONTEXT_ATTRIBUTES,
    pub LogicalBlockCount: u32,
    pub StartingLBA: u64,
}
impl Default for NVME_LBA_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_LBA_RANGET_TYPE_ENTRY {
    pub Type: u8,
    pub Attributes: NVME_LBA_RANGET_TYPE_ENTRY_0,
    pub Reserved0: [u8; 14],
    pub SLBA: u64,
    pub NLB: u64,
    pub GUID: [u8; 16],
    pub Reserved1: [u8; 16],
}
impl Default for NVME_LBA_RANGET_TYPE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_LBA_RANGET_TYPE_ENTRY_0 {
    pub _bitfield: u8,
}
pub type NVME_LBA_RANGE_TYPES = i32;
pub const NVME_LBA_RANGE_TYPE_CACHE: NVME_LBA_RANGE_TYPES = 3i32;
pub const NVME_LBA_RANGE_TYPE_FILESYSTEM: NVME_LBA_RANGE_TYPES = 1i32;
pub const NVME_LBA_RANGE_TYPE_PAGE_SWAP_FILE: NVME_LBA_RANGE_TYPES = 4i32;
pub const NVME_LBA_RANGE_TYPE_RAID: NVME_LBA_RANGE_TYPES = 2i32;
pub const NVME_LBA_RANGE_TYPE_RESERVED: NVME_LBA_RANGE_TYPES = 0i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_LBA_ZONE_FORMAT {
    pub ZoneSize: u64,
    pub ZDES: u8,
    pub Reserved: [u8; 7],
}
impl Default for NVME_LBA_ZONE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_LOG_PAGES = i32;
pub const NVME_LOG_PAGE_ASYMMETRIC_NAMESPACE_ACCESS: NVME_LOG_PAGES = 12i32;
pub const NVME_LOG_PAGE_CHANGED_NAMESPACE_LIST: NVME_LOG_PAGES = 4i32;
pub const NVME_LOG_PAGE_CHANGED_ZONE_LIST: NVME_LOG_PAGES = 191i32;
pub const NVME_LOG_PAGE_COMMAND_EFFECTS: NVME_LOG_PAGES = 5i32;
pub const NVME_LOG_PAGE_DEVICE_SELF_TEST: NVME_LOG_PAGES = 6i32;
pub const NVME_LOG_PAGE_ENDURANCE_GROUP_EVENT_AGGREGATE: NVME_LOG_PAGES = 15i32;
pub const NVME_LOG_PAGE_ENDURANCE_GROUP_INFORMATION: NVME_LOG_PAGES = 9i32;
pub const NVME_LOG_PAGE_ERROR_INFO: NVME_LOG_PAGES = 1i32;
pub const NVME_LOG_PAGE_FIRMWARE_SLOT_INFO: NVME_LOG_PAGES = 3i32;
pub const NVME_LOG_PAGE_HEALTH_INFO: NVME_LOG_PAGES = 2i32;
pub const NVME_LOG_PAGE_LBA_STATUS_INFORMATION: NVME_LOG_PAGES = 14i32;
pub const NVME_LOG_PAGE_OCP_DEVICE_CAPABILITIES: NVME_VENDOR_LOG_PAGES = 196i32;
pub const NVME_LOG_PAGE_OCP_DEVICE_ERROR_RECOVERY: NVME_VENDOR_LOG_PAGES = 193i32;
pub const NVME_LOG_PAGE_OCP_DEVICE_SMART_INFORMATION: NVME_VENDOR_LOG_PAGES = 192i32;
pub const NVME_LOG_PAGE_OCP_FIRMWARE_ACTIVATION_HISTORY: NVME_VENDOR_LOG_PAGES = 194i32;
pub const NVME_LOG_PAGE_OCP_LATENCY_MONITOR: NVME_VENDOR_LOG_PAGES = 195i32;
pub const NVME_LOG_PAGE_OCP_TCG_CONFIGURATION: NVME_VENDOR_LOG_PAGES = 200i32;
pub const NVME_LOG_PAGE_OCP_TCG_HISTORY: NVME_VENDOR_LOG_PAGES = 201i32;
pub const NVME_LOG_PAGE_OCP_UNSUPPORTED_REQUIREMENTS: NVME_VENDOR_LOG_PAGES = 197i32;
pub const NVME_LOG_PAGE_PERSISTENT_EVENT_LOG: NVME_LOG_PAGES = 13i32;
pub const NVME_LOG_PAGE_PREDICTABLE_LATENCY_EVENT_AGGREGATE: NVME_LOG_PAGES = 11i32;
pub const NVME_LOG_PAGE_PREDICTABLE_LATENCY_NVM_SET: NVME_LOG_PAGES = 10i32;
pub const NVME_LOG_PAGE_RESERVATION_NOTIFICATION: NVME_LOG_PAGES = 128i32;
pub const NVME_LOG_PAGE_SANITIZE_STATUS: NVME_LOG_PAGES = 129i32;
pub const NVME_LOG_PAGE_TELEMETRY_CTLR_INITIATED: NVME_LOG_PAGES = 8i32;
pub const NVME_LOG_PAGE_TELEMETRY_HOST_INITIATED: NVME_LOG_PAGES = 7i32;
pub const NVME_MAX_HOST_IDENTIFIER_SIZE: u32 = 16u32;
pub const NVME_MAX_LOG_SIZE: u32 = 4096u32;
pub const NVME_MEDIA_ADDITIONALLY_MODIFIED_AFTER_SANITIZE_NOT_DEFINED: NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = 0i32;
pub const NVME_MEDIA_ADDITIONALLY_MOFIDIED_AFTER_SANITIZE: NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = 2i32;
pub const NVME_MEDIA_NOT_ADDITIONALLY_MODIFIED_AFTER_SANITIZE: NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = 1i32;
pub const NVME_NAMESPACE_ALL: u32 = 4294967295u32;
pub type NVME_NAMESPACE_METADATA_ELEMENT_TYPES = i32;
pub const NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME: NVME_NAMESPACE_METADATA_ELEMENT_TYPES = 1i32;
pub const NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_1: NVME_NAMESPACE_METADATA_ELEMENT_TYPES = 3i32;
pub const NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_2: NVME_NAMESPACE_METADATA_ELEMENT_TYPES = 4i32;
pub const NVME_NAMESPACE_METADATA_PREBOOT_NAMESPACE_NAME: NVME_NAMESPACE_METADATA_ELEMENT_TYPES = 2i32;
pub type NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = i32;
pub type NVME_NVM_COMMANDS = i32;
pub const NVME_NVM_COMMAND_COMPARE: NVME_NVM_COMMANDS = 5i32;
pub const NVME_NVM_COMMAND_COPY: NVME_NVM_COMMANDS = 25i32;
pub const NVME_NVM_COMMAND_DATASET_MANAGEMENT: NVME_NVM_COMMANDS = 9i32;
pub const NVME_NVM_COMMAND_FLUSH: NVME_NVM_COMMANDS = 0i32;
pub const NVME_NVM_COMMAND_READ: NVME_NVM_COMMANDS = 2i32;
pub const NVME_NVM_COMMAND_RESERVATION_ACQUIRE: NVME_NVM_COMMANDS = 17i32;
pub const NVME_NVM_COMMAND_RESERVATION_REGISTER: NVME_NVM_COMMANDS = 13i32;
pub const NVME_NVM_COMMAND_RESERVATION_RELEASE: NVME_NVM_COMMANDS = 21i32;
pub const NVME_NVM_COMMAND_RESERVATION_REPORT: NVME_NVM_COMMANDS = 14i32;
pub const NVME_NVM_COMMAND_VERIFY: NVME_NVM_COMMANDS = 12i32;
pub const NVME_NVM_COMMAND_WRITE: NVME_NVM_COMMANDS = 1i32;
pub const NVME_NVM_COMMAND_WRITE_UNCORRECTABLE: NVME_NVM_COMMANDS = 4i32;
pub const NVME_NVM_COMMAND_WRITE_ZEROES: NVME_NVM_COMMANDS = 8i32;
pub const NVME_NVM_COMMAND_ZONE_APPEND: NVME_NVM_COMMANDS = 125i32;
pub const NVME_NVM_COMMAND_ZONE_MANAGEMENT_RECEIVE: NVME_NVM_COMMANDS = 122i32;
pub const NVME_NVM_COMMAND_ZONE_MANAGEMENT_SEND: NVME_NVM_COMMANDS = 121i32;
pub type NVME_NVM_QUEUE_PRIORITIES = i32;
pub const NVME_NVM_QUEUE_PRIORITY_HIGH: NVME_NVM_QUEUE_PRIORITIES = 1i32;
pub const NVME_NVM_QUEUE_PRIORITY_LOW: NVME_NVM_QUEUE_PRIORITIES = 3i32;
pub const NVME_NVM_QUEUE_PRIORITY_MEDIUM: NVME_NVM_QUEUE_PRIORITIES = 2i32;
pub const NVME_NVM_QUEUE_PRIORITY_URGENT: NVME_NVM_QUEUE_PRIORITIES = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_NVM_SUBSYSTEM_RESET {
    pub NSSRC: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG {
    pub PciePorts: u16,
    pub OobMgmtSupport: NVME_OCP_DEVICE_CAPABILITIES_LOG_0,
    pub WriteZeroesCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_1,
    pub SanitizeCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_2,
    pub DatasetMgmtCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_3,
    pub WriteUncorrectableCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_4,
    pub FusedCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_5,
    pub MinimumValidDSSDPowerState: u16,
    pub Reserved0: u8,
    pub DssdDescriptors: [DSSD_POWER_STATE_DESCRIPTOR; 127],
    pub Reserved1: [u8; 3934],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_3 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_3_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_3_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_5 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_5_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_5_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_0_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_0_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_2 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_2_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_2_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_4 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_4_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_4_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_1 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_1_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_1_0 {
    pub _bitfield: u16,
}
pub const NVME_OCP_DEVICE_CAPABILITIES_LOG_VERSION_1: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 {
    pub PanicResetWaitTime: u16,
    pub PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION,
    pub DeviceRecoveryAction1: u8,
    pub PanicId: u64,
    pub DeviceCapabilitiesA: NVME_WCS_DEVICE_CAPABILITIES,
    pub VendorSpecificRecoveryCode: u8,
    pub Reserved0: [u8; 3],
    pub VendorSpecificCommandCDW12: u32,
    pub VendorSpecificCommandCDW13: u32,
    pub VendorSpecificCommandTimeout: u8,
    pub DeviceRecoveryAction2: u8,
    pub DeviceRecoveryAction2Timeout: u8,
    pub Reserved1: [u8; 463],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_VERSION_2: u32 = 2u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG {
    pub LID: u8,
    pub Reserved0: [u8; 3],
    pub ValidNumberOfEntries: u32,
    pub Entries: [FIRMWARE_ACTIVATION_HISTORY_ENTRY; 20],
    pub Reserved1: [u8; 2790],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG_VERSION_1: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_LATENCY_MONITOR_LOG {
    pub FeatureStatus: LATENCY_MONITOR_FEATURE_STATUS,
    pub Reserved0: u8,
    pub ActiveBucketTimer: u16,
    pub ActiveBucketTimerThreshold: u16,
    pub ActiveThresholdA: u8,
    pub ActiveThresholdB: u8,
    pub ActiveThresholdC: u8,
    pub ActiveThresholdD: u8,
    pub ActiveLatencyConfig: ACTIVE_LATENCY_CONFIGURATION,
    pub ActiveLatencyMinimumWindow: u8,
    pub Reserved1: [u8; 19],
    pub ActiveBucketCounter0: BUCKET_COUNTER,
    pub ActiveBucketCounter1: BUCKET_COUNTER,
    pub ActiveBucketCounter2: BUCKET_COUNTER,
    pub ActiveBucketCounter3: BUCKET_COUNTER,
    pub ActiveLatencyStamp: LATENCY_STAMP,
    pub ActiveMeasuredLatency: MEASURED_LATENCY,
    pub ActiveLatencyStampUnits: LATENCY_STAMP_UNITS,
    pub Reserved2: [u8; 22],
    pub StaticBucketCounter0: BUCKET_COUNTER,
    pub StaticBucketCounter1: BUCKET_COUNTER,
    pub StaticBucketCounter2: BUCKET_COUNTER,
    pub StaticBucketCounter3: BUCKET_COUNTER,
    pub StaticLatencyStamp: LATENCY_STAMP,
    pub StaticMeasuredLatency: MEASURED_LATENCY,
    pub StaticLatencyStampUnits: LATENCY_STAMP_UNITS,
    pub Reserved3: [u8; 22],
    pub DebugLogTriggerEnable: DEBUG_BIT_FIELD,
    pub DebugLogMeasuredLatency: u16,
    pub DebugLogLatencyStamp: u64,
    pub DebugLogPointer: u16,
    pub DebugCounterTriggerSource: DEBUG_BIT_FIELD,
    pub DebugLogStampUnits: NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0,
    pub Reserved4: [u8; 29],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_OCP_DEVICE_LATENCY_MONITOR_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0_0 {
    pub _bitfield: u8,
}
pub const NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_VERSION_1: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 {
    pub MediaUnitsWritten: [u8; 16],
    pub MediaUnitsRead: [u8; 16],
    pub BadUserNANDBlockCount: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0,
    pub BadSystemNANDBlockCount: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1,
    pub XORRecoveryCount: u64,
    pub UnrecoverableReadErrorCount: u64,
    pub SoftECCErrorCount: u64,
    pub EndToEndCorrectionCounts: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_2,
    pub PercentageSystemDataUsed: u8,
    pub RefreshCount: [u8; 7],
    pub UserDataEraseCounts: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_3,
    pub ThermalThrottling: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_4,
    pub DSSDSpecVersion: [u8; 6],
    pub PCIeCorrectableErrorCount: u64,
    pub IncompleteShutdownCount: u32,
    pub Reserved1: u32,
    pub PercentageFreeBlocks: u8,
    pub Reserved2: [u8; 7],
    pub CapacitorHealth: u16,
    pub NvmeErrata: u8,
    pub Reserved3: [u8; 5],
    pub UnalignedIOCount: u64,
    pub SecurityVersionNumber: u64,
    pub NUSE: u64,
    pub PLPStartCount: [u8; 16],
    pub EnduranceEstimate: [u8; 16],
    pub PCIeLinkRetrainingCount: u64,
    pub PowerStateChangeCount: u64,
    pub Reserved4: [u8; 286],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1 {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0 {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_2 {
    pub DetectedCounts: u32,
    pub CorrectedCounts: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_4 {
    pub EventCount: u8,
    pub Status: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_3 {
    pub MaximumCount: u32,
    pub MinimumCount: u32,
}
pub const NVME_OCP_DEVICE_SMART_INFORMATION_LOG_VERSION_3: u32 = 3u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG {
    pub State: NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0,
    pub Reserved0: [u8; 3],
    pub LSPActivationCount: u8,
    pub TPRevertCount: u8,
    pub LSPRevertCount: u8,
    pub LOCount: u8,
    pub SUMLOCount: u8,
    pub RPLOCount: u8,
    pub NPLOCount: u8,
    pub RLLOCount: u8,
    pub WLLOCount: u8,
    pub RULOCount: u8,
    pub WULOCount: u8,
    pub Reserved1: u8,
    pub SIDAuthTryCount: u32,
    pub SIDAuthTryLimit: u32,
    pub ResetCount: u32,
    pub ResetLockCount: u32,
    pub Reserved2: [u8; 462],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0_0 {
    pub _bitfield: u8,
}
pub const NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_VERSION_1: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_TCG_HISTORY_LOG {
    pub LID: u8,
    pub Reserved0: [u8; 3],
    pub HistoryEntryCount: u32,
    pub HistoryEntries: [TCG_HISTORY_ENTRY; 84],
    pub Reserved1: [u8; 38],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_OCP_DEVICE_TCG_HISTORY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_TCG_HISTORY_LOG_VERSION_1: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG {
    pub UnsupportedCount: u16,
    pub Reserved0: [u8; 14],
    pub UnsupportedReqList: [UNSUPPORTED_REQUIREMENT; 253],
    pub Reserved1: [u8; 14],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG_VERSION_1: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER {
    pub EventType: u8,
    pub EventTypeRevision: u8,
    pub EventHeaderLength: u8,
    pub Reserved0: u8,
    pub ControllerIdentifier: u16,
    pub EventTimestamp: u64,
    pub Reserved1: [u8; 6],
    pub VendorSpecificInformationLength: u16,
    pub EventLength: u16,
}
impl Default for NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_PERSISTENT_EVENT_LOG_HEADER {
    pub LogIdentifier: u8,
    pub Reserved0: [u8; 3],
    pub TotalNumberOfEvents: u32,
    pub TotalLogLength: u64,
    pub LogRevision: u8,
    pub Reserved1: u8,
    pub LogHeaderLength: u16,
    pub Timestamp: u64,
    pub PowerOnHours: [u8; 16],
    pub PowerCycleCount: u64,
    pub PciVendorId: u16,
    pub PciSubsystemVendorId: u16,
    pub SerialNumber: [u8; 20],
    pub ModelNumber: [u8; 40],
    pub NVMSubsystemNVMeQualifiedName: [u8; 256],
    pub Reserved: [u8; 108],
    pub SupportedEventsBitmap: [u8; 32],
}
impl Default for NVME_PERSISTENT_EVENT_LOG_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_PERSISTENT_EVENT_TYPE_CHANGE_NAMESPACE: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 6i32;
pub const NVME_PERSISTENT_EVENT_TYPE_FIRMWARE_COMMIT: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 2i32;
pub const NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_COMPLETION: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 8i32;
pub const NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_START: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 7i32;
pub const NVME_PERSISTENT_EVENT_TYPE_MAX: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 255i32;
pub const NVME_PERSISTENT_EVENT_TYPE_NVM_SUBSYSTEM_HARDWARE_ERROR: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 5i32;
pub const NVME_PERSISTENT_EVENT_TYPE_POWER_ON_OR_RESET: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 4i32;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED0: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 0i32;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED1_BEGIN: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 14i32;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED1_END: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 221i32;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED2_BEGIN: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 224i32;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED2_END: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 255i32;
pub const NVME_PERSISTENT_EVENT_TYPE_SANITIZE_COMPLETION: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 10i32;
pub const NVME_PERSISTENT_EVENT_TYPE_SANITIZE_START: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 9i32;
pub const NVME_PERSISTENT_EVENT_TYPE_SET_FEATURE: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 11i32;
pub const NVME_PERSISTENT_EVENT_TYPE_SMART_HEALTH_LOG_SNAPSHOT: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 1i32;
pub const NVME_PERSISTENT_EVENT_TYPE_TCG_DEFINED: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 223i32;
pub const NVME_PERSISTENT_EVENT_TYPE_TELEMETRY_LOG_CREATED: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 12i32;
pub const NVME_PERSISTENT_EVENT_TYPE_THERMAL_EXCURSION: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 13i32;
pub const NVME_PERSISTENT_EVENT_TYPE_TIMESTAMP_CHANGE: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 3i32;
pub const NVME_PERSISTENT_EVENT_TYPE_VENDOR_SPECIFIC_EVENT: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 222i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_POWER_STATE_DESC {
    pub MP: u16,
    pub Reserved0: u8,
    pub _bitfield1: u8,
    pub ENLAT: u32,
    pub EXLAT: u32,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
    pub _bitfield5: u8,
    pub IDLP: u16,
    pub _bitfield6: u8,
    pub Reserved7: u8,
    pub ACTP: u16,
    pub _bitfield7: u8,
    pub Reserved9: [u8; 9],
}
impl Default for NVME_POWER_STATE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_PROTECTION_INFORMATION_NOT_ENABLED: NVME_PROTECTION_INFORMATION_TYPES = 0i32;
pub const NVME_PROTECTION_INFORMATION_TYPE1: NVME_PROTECTION_INFORMATION_TYPES = 1i32;
pub const NVME_PROTECTION_INFORMATION_TYPE2: NVME_PROTECTION_INFORMATION_TYPES = 2i32;
pub const NVME_PROTECTION_INFORMATION_TYPE3: NVME_PROTECTION_INFORMATION_TYPES = 3i32;
pub type NVME_PROTECTION_INFORMATION_TYPES = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_PRP_ENTRY {
    pub Anonymous: NVME_PRP_ENTRY_0,
    pub AsUlonglong: u64,
}
impl Default for NVME_PRP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_PRP_ENTRY_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_REGISTERED_CONTROLLER_DATA {
    pub CNTLID: u16,
    pub RCSTS: NVME_REGISTERED_CONTROLLER_DATA_0,
    pub Reserved: [u8; 5],
    pub HOSTID: [u8; 8],
    pub RKEY: u64,
}
impl Default for NVME_REGISTERED_CONTROLLER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_REGISTERED_CONTROLLER_DATA_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA {
    pub CNTLID: u16,
    pub RCSTS: NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_0,
    pub Reserved: [u8; 5],
    pub RKEY: u64,
    pub HOSTID: [u8; 16],
    pub Reserved1: [u8; 32],
}
impl Default for NVME_REGISTERED_CONTROLLER_EXTENDED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_REPORT_ZONE_INFO {
    pub ZoneCount: u64,
    pub Reserved: [u64; 7],
    pub ZoneDescriptor: [NVME_ZONE_DESCRIPTOR; 1],
}
impl Default for NVME_REPORT_ZONE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_RESERVATION_ACQUIRE_ACTIONS = i32;
pub const NVME_RESERVATION_ACQUIRE_ACTION_ACQUIRE: NVME_RESERVATION_ACQUIRE_ACTIONS = 0i32;
pub const NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT: NVME_RESERVATION_ACQUIRE_ACTIONS = 1i32;
pub const NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT_AND_ABORT: NVME_RESERVATION_ACQUIRE_ACTIONS = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_RESERVATION_ACQUIRE_DATA_STRUCTURE {
    pub CRKEY: u64,
    pub PRKEY: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_NOTIFICATION_LOG {
    pub LogPageCount: u64,
    pub LogPageType: u8,
    pub AvailableLogPageCount: u8,
    pub Reserved0: [u8; 2],
    pub NameSpaceId: u32,
    pub Reserved1: [u8; 48],
}
impl Default for NVME_RESERVATION_NOTIFICATION_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_RESERVATION_NOTIFICATION_TYPES = i32;
pub const NVME_RESERVATION_NOTIFICATION_TYPE_EMPTY_LOG_PAGE: NVME_RESERVATION_NOTIFICATION_TYPES = 0i32;
pub const NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_PREEMPTED: NVME_RESERVATION_NOTIFICATION_TYPES = 1i32;
pub const NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_RELEASED: NVME_RESERVATION_NOTIFICATION_TYPES = 2i32;
pub const NVME_RESERVATION_NOTIFICATION_TYPE_RESERVATION_PREEPMPTED: NVME_RESERVATION_NOTIFICATION_TYPES = 3i32;
pub type NVME_RESERVATION_REGISTER_ACTIONS = i32;
pub const NVME_RESERVATION_REGISTER_ACTION_REGISTER: NVME_RESERVATION_REGISTER_ACTIONS = 0i32;
pub const NVME_RESERVATION_REGISTER_ACTION_REPLACE: NVME_RESERVATION_REGISTER_ACTIONS = 2i32;
pub const NVME_RESERVATION_REGISTER_ACTION_UNREGISTER: NVME_RESERVATION_REGISTER_ACTIONS = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_RESERVATION_REGISTER_DATA_STRUCTURE {
    pub CRKEY: u64,
    pub NRKEY: u64,
}
pub type NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = i32;
pub const NVME_RESERVATION_REGISTER_PTPL_STATE_NO_CHANGE: NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = 0i32;
pub const NVME_RESERVATION_REGISTER_PTPL_STATE_RESERVED: NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = 1i32;
pub const NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_0: NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = 2i32;
pub const NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_1: NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = 3i32;
pub type NVME_RESERVATION_RELEASE_ACTIONS = i32;
pub const NVME_RESERVATION_RELEASE_ACTION_CLEAR: NVME_RESERVATION_RELEASE_ACTIONS = 1i32;
pub const NVME_RESERVATION_RELEASE_ACTION_RELEASE: NVME_RESERVATION_RELEASE_ACTIONS = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_RESERVATION_RELEASE_DATA_STRUCTURE {
    pub CRKEY: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE {
    pub Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    pub RegisteredControllersData: [NVME_REGISTERED_CONTROLLER_DATA; 1],
}
impl Default for NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    pub Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    pub Reserved1: [u8; 40],
    pub RegisteredControllersExtendedData: [NVME_REGISTERED_CONTROLLER_EXTENDED_DATA; 1],
}
impl Default for NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_HEADER {
    pub GEN: u32,
    pub RTYPE: u8,
    pub REGCTL: u16,
    pub Reserved: [u8; 2],
    pub PTPLS: u8,
    pub Reserved1: [u8; 14],
}
impl Default for NVME_RESERVATION_REPORT_STATUS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_RESERVATION_TYPES = i32;
pub const NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS: NVME_RESERVATION_TYPES = 2i32;
pub const NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_ALL_REGISTRANTS: NVME_RESERVATION_TYPES = 6i32;
pub const NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_REGISTRANTS_ONLY: NVME_RESERVATION_TYPES = 4i32;
pub const NVME_RESERVATION_TYPE_RESERVED: NVME_RESERVATION_TYPES = 0i32;
pub const NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE: NVME_RESERVATION_TYPES = 1i32;
pub const NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_ALL_REGISTRANTS: NVME_RESERVATION_TYPES = 5i32;
pub const NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_REGISTRANTS_ONLY: NVME_RESERVATION_TYPES = 3i32;
pub type NVME_SANITIZE_ACTION = i32;
pub const NVME_SANITIZE_ACTION_EXIT_FAILURE_MODE: NVME_SANITIZE_ACTION = 1i32;
pub const NVME_SANITIZE_ACTION_RESERVED: NVME_SANITIZE_ACTION = 0i32;
pub const NVME_SANITIZE_ACTION_START_BLOCK_ERASE_SANITIZE: NVME_SANITIZE_ACTION = 2i32;
pub const NVME_SANITIZE_ACTION_START_CRYPTO_ERASE_SANITIZE: NVME_SANITIZE_ACTION = 4i32;
pub const NVME_SANITIZE_ACTION_START_OVERWRITE_SANITIZE: NVME_SANITIZE_ACTION = 3i32;
pub const NVME_SANITIZE_OPERATION_FAILED: NVME_SANITIZE_OPERATION_STATUS = 3i32;
pub const NVME_SANITIZE_OPERATION_IN_PROGRESS: NVME_SANITIZE_OPERATION_STATUS = 2i32;
pub const NVME_SANITIZE_OPERATION_NONE: NVME_SANITIZE_OPERATION_STATUS = 0i32;
pub type NVME_SANITIZE_OPERATION_STATUS = i32;
pub const NVME_SANITIZE_OPERATION_SUCCEEDED: NVME_SANITIZE_OPERATION_STATUS = 1i32;
pub const NVME_SANITIZE_OPERATION_SUCCEEDED_WITH_FORCED_DEALLOCATION: NVME_SANITIZE_OPERATION_STATUS = 4i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_SANITIZE_STATUS {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SANITIZE_STATUS_LOG {
    pub SPROG: u16,
    pub SSTAT: NVME_SANITIZE_STATUS,
    pub SCDW10: u32,
    pub EstimatedTimeForOverwrite: u32,
    pub EstimatedTimeForBlockErase: u32,
    pub EstimatedTimeForCryptoErase: u32,
    pub EstimatedTimeForOverwriteWithNoDeallocateMediaModification: u32,
    pub EstimatedTimeForBlockEraseWithNoDeallocateMediaModification: u32,
    pub EstimatedTimeForCryptoEraseWithNoDeallocateMediaModification: u32,
    pub Reserved: [u8; 480],
}
impl Default for NVME_SANITIZE_STATUS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SCSI_NAME_STRING {
    pub PCIVendorID: [i8; 4],
    pub ModelNumber: [i8; 40],
    pub NamespaceID: [i8; 4],
    pub SerialNumber: [i8; 20],
}
impl Default for NVME_SCSI_NAME_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_SECURE_ERASE_CRYPTOGRAPHIC: NVME_SECURE_ERASE_SETTINGS = 2i32;
pub const NVME_SECURE_ERASE_NONE: NVME_SECURE_ERASE_SETTINGS = 0i32;
pub type NVME_SECURE_ERASE_SETTINGS = i32;
pub const NVME_SECURE_ERASE_USER_DATA: NVME_SECURE_ERASE_SETTINGS = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SET_ATTRIBUTES_ENTRY {
    pub Identifier: u16,
    pub ENDGID: u16,
    pub Reserved1: u32,
    pub Random4KBReadTypical: u32,
    pub OptimalWriteSize: u32,
    pub TotalCapacity: [u8; 16],
    pub UnallocatedCapacity: [u8; 16],
    pub Reserved2: [u8; 80],
}
impl Default for NVME_SET_ATTRIBUTES_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_STATE_ZSC: ZONE_STATE = 4i32;
pub const NVME_STATE_ZSE: ZONE_STATE = 1i32;
pub const NVME_STATE_ZSEO: ZONE_STATE = 3i32;
pub const NVME_STATE_ZSF: ZONE_STATE = 14i32;
pub const NVME_STATE_ZSIO: ZONE_STATE = 2i32;
pub const NVME_STATE_ZSO: ZONE_STATE = 15i32;
pub const NVME_STATE_ZSRO: ZONE_STATE = 13i32;
pub const NVME_STATUS_ABORT_COMMAND_LIMIT_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 3i32;
pub const NVME_STATUS_ANA_ATTACH_FAILED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 37i32;
pub const NVME_STATUS_ASYNC_EVENT_REQUEST_LIMIT_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 5i32;
pub const NVME_STATUS_ATOMIC_WRITE_UNIT_EXCEEDED: NVME_STATUS_GENERIC_COMMAND_CODES = 20i32;
pub const NVME_STATUS_BOOT_PARTITION_WRITE_PROHIBITED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 30i32;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_FUSED_COMMAND: NVME_STATUS_GENERIC_COMMAND_CODES = 9i32;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_MISSING_COMMAND: NVME_STATUS_GENERIC_COMMAND_CODES = 10i32;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_POWER_LOSS_NOTIFICATION: NVME_STATUS_GENERIC_COMMAND_CODES = 5i32;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_PREEMPT_ABORT: NVME_STATUS_GENERIC_COMMAND_CODES = 27i32;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_SQ_DELETION: NVME_STATUS_GENERIC_COMMAND_CODES = 8i32;
pub const NVME_STATUS_COMMAND_ABORT_REQUESTED: NVME_STATUS_GENERIC_COMMAND_CODES = 7i32;
pub const NVME_STATUS_COMMAND_ID_CONFLICT: NVME_STATUS_GENERIC_COMMAND_CODES = 3i32;
pub const NVME_STATUS_COMMAND_SEQUENCE_ERROR: NVME_STATUS_GENERIC_COMMAND_CODES = 12i32;
pub type NVME_STATUS_COMMAND_SPECIFIC_CODES = i32;
pub const NVME_STATUS_COMPLETION_QUEUE_INVALID: NVME_STATUS_COMMAND_SPECIFIC_CODES = 0i32;
pub const NVME_STATUS_CONTROLLER_LIST_INVALID: NVME_STATUS_COMMAND_SPECIFIC_CODES = 28i32;
pub const NVME_STATUS_DATA_SGL_LENGTH_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 15i32;
pub const NVME_STATUS_DATA_TRANSFER_ERROR: NVME_STATUS_GENERIC_COMMAND_CODES = 4i32;
pub const NVME_STATUS_DEVICE_SELF_TEST_IN_PROGRESS: NVME_STATUS_COMMAND_SPECIFIC_CODES = 29i32;
pub const NVME_STATUS_DIRECTIVE_ID_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 113i32;
pub const NVME_STATUS_DIRECTIVE_TYPE_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 112i32;
pub const NVME_STATUS_FEATURE_ID_NOT_SAVEABLE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 13i32;
pub const NVME_STATUS_FEATURE_NOT_CHANGEABLE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 14i32;
pub const NVME_STATUS_FEATURE_NOT_NAMESPACE_SPECIFIC: NVME_STATUS_COMMAND_SPECIFIC_CODES = 15i32;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_PROHIBITED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 19i32;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_CONVENTIONAL_RESET: NVME_STATUS_COMMAND_SPECIFIC_CODES = 11i32;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_MAX_TIME_VIOLATION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 18i32;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_NVM_SUBSYSTEM_RESET: NVME_STATUS_COMMAND_SPECIFIC_CODES = 16i32;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_RESET: NVME_STATUS_COMMAND_SPECIFIC_CODES = 17i32;
pub const NVME_STATUS_FORMAT_IN_PROGRESS: NVME_STATUS_GENERIC_COMMAND_CODES = 132i32;
pub type NVME_STATUS_GENERIC_COMMAND_CODES = i32;
pub const NVME_STATUS_HOST_IDENTIFIER_INCONSISTENT_FORMAT: NVME_STATUS_GENERIC_COMMAND_CODES = 24i32;
pub const NVME_STATUS_INTERNAL_DEVICE_ERROR: NVME_STATUS_GENERIC_COMMAND_CODES = 6i32;
pub const NVME_STATUS_INVALID_ANA_GROUP_IDENTIFIER: NVME_STATUS_COMMAND_SPECIFIC_CODES = 36i32;
pub const NVME_STATUS_INVALID_COMMAND_OPCODE: NVME_STATUS_GENERIC_COMMAND_CODES = 1i32;
pub const NVME_STATUS_INVALID_CONTROLLER_IDENTIFIER: NVME_STATUS_COMMAND_SPECIFIC_CODES = 31i32;
pub const NVME_STATUS_INVALID_FIELD_IN_COMMAND: NVME_STATUS_GENERIC_COMMAND_CODES = 2i32;
pub const NVME_STATUS_INVALID_FIRMWARE_IMAGE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 7i32;
pub const NVME_STATUS_INVALID_FIRMWARE_SLOT: NVME_STATUS_COMMAND_SPECIFIC_CODES = 6i32;
pub const NVME_STATUS_INVALID_FORMAT: NVME_STATUS_COMMAND_SPECIFIC_CODES = 10i32;
pub const NVME_STATUS_INVALID_INTERRUPT_VECTOR: NVME_STATUS_COMMAND_SPECIFIC_CODES = 8i32;
pub const NVME_STATUS_INVALID_LOG_PAGE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 9i32;
pub const NVME_STATUS_INVALID_NAMESPACE_OR_FORMAT: NVME_STATUS_GENERIC_COMMAND_CODES = 11i32;
pub const NVME_STATUS_INVALID_NUMBER_OF_CONTROLLER_RESOURCES: NVME_STATUS_COMMAND_SPECIFIC_CODES = 33i32;
pub const NVME_STATUS_INVALID_NUMBER_OF_SGL_DESCR: NVME_STATUS_GENERIC_COMMAND_CODES = 14i32;
pub const NVME_STATUS_INVALID_QUEUE_DELETION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 12i32;
pub const NVME_STATUS_INVALID_QUEUE_IDENTIFIER: NVME_STATUS_COMMAND_SPECIFIC_CODES = 1i32;
pub const NVME_STATUS_INVALID_RESOURCE_IDENTIFIER: NVME_STATUS_COMMAND_SPECIFIC_CODES = 34i32;
pub const NVME_STATUS_INVALID_SECONDARY_CONTROLLER_STATE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 32i32;
pub const NVME_STATUS_INVALID_SGL_LAST_SEGMENT_DESCR: NVME_STATUS_GENERIC_COMMAND_CODES = 13i32;
pub const NVME_STATUS_INVALID_USE_OF_CONTROLLER_MEMORY_BUFFER: NVME_STATUS_GENERIC_COMMAND_CODES = 18i32;
pub const NVME_STATUS_KEEP_ALIVE_TIMEOUT_EXPIRED: NVME_STATUS_GENERIC_COMMAND_CODES = 25i32;
pub const NVME_STATUS_KEEP_ALIVE_TIMEOUT_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 26i32;
pub const NVME_STATUS_MAX_QUEUE_SIZE_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 2i32;
pub type NVME_STATUS_MEDIA_ERROR_CODES = i32;
pub const NVME_STATUS_METADATA_SGL_LENGTH_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 16i32;
pub const NVME_STATUS_NAMESPACE_ALREADY_ATTACHED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 24i32;
pub const NVME_STATUS_NAMESPACE_IDENTIFIER_UNAVAILABLE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 22i32;
pub const NVME_STATUS_NAMESPACE_INSUFFICIENT_CAPACITY: NVME_STATUS_COMMAND_SPECIFIC_CODES = 21i32;
pub const NVME_STATUS_NAMESPACE_IS_PRIVATE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 25i32;
pub const NVME_STATUS_NAMESPACE_NOT_ATTACHED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 26i32;
pub const NVME_STATUS_NAMESPACE_THIN_PROVISIONING_NOT_SUPPORTED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 27i32;
pub const NVME_STATUS_NVM_ACCESS_DENIED: NVME_STATUS_MEDIA_ERROR_CODES = 134i32;
pub const NVME_STATUS_NVM_ATTEMPTED_WRITE_TO_READ_ONLY_RANGE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 130i32;
pub const NVME_STATUS_NVM_CAPACITY_EXCEEDED: NVME_STATUS_GENERIC_COMMAND_CODES = 129i32;
pub const NVME_STATUS_NVM_COMMAND_SIZE_LIMIT_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 131i32;
pub const NVME_STATUS_NVM_COMPARE_FAILURE: NVME_STATUS_MEDIA_ERROR_CODES = 133i32;
pub const NVME_STATUS_NVM_CONFLICTING_ATTRIBUTES: NVME_STATUS_COMMAND_SPECIFIC_CODES = 128i32;
pub const NVME_STATUS_NVM_DEALLOCATED_OR_UNWRITTEN_LOGICAL_BLOCK: NVME_STATUS_MEDIA_ERROR_CODES = 135i32;
pub const NVME_STATUS_NVM_END_TO_END_APPLICATION_TAG_CHECK_ERROR: NVME_STATUS_MEDIA_ERROR_CODES = 131i32;
pub const NVME_STATUS_NVM_END_TO_END_GUARD_CHECK_ERROR: NVME_STATUS_MEDIA_ERROR_CODES = 130i32;
pub const NVME_STATUS_NVM_END_TO_END_REFERENCE_TAG_CHECK_ERROR: NVME_STATUS_MEDIA_ERROR_CODES = 132i32;
pub const NVME_STATUS_NVM_INVALID_PROTECTION_INFORMATION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 129i32;
pub const NVME_STATUS_NVM_LBA_OUT_OF_RANGE: NVME_STATUS_GENERIC_COMMAND_CODES = 128i32;
pub const NVME_STATUS_NVM_NAMESPACE_NOT_READY: NVME_STATUS_GENERIC_COMMAND_CODES = 130i32;
pub const NVME_STATUS_NVM_RESERVATION_CONFLICT: NVME_STATUS_GENERIC_COMMAND_CODES = 131i32;
pub const NVME_STATUS_NVM_UNRECOVERED_READ_ERROR: NVME_STATUS_MEDIA_ERROR_CODES = 129i32;
pub const NVME_STATUS_NVM_WRITE_FAULT: NVME_STATUS_MEDIA_ERROR_CODES = 128i32;
pub const NVME_STATUS_OPERATION_DENIED: NVME_STATUS_GENERIC_COMMAND_CODES = 21i32;
pub const NVME_STATUS_OVERLAPPING_RANGE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 20i32;
pub const NVME_STATUS_PRP_OFFSET_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 19i32;
pub const NVME_STATUS_RESERVED: NVME_STATUS_GENERIC_COMMAND_CODES = 23i32;
pub const NVME_STATUS_SANITIZE_FAILED: NVME_STATUS_GENERIC_COMMAND_CODES = 28i32;
pub const NVME_STATUS_SANITIZE_IN_PROGRESS: NVME_STATUS_GENERIC_COMMAND_CODES = 29i32;
pub const NVME_STATUS_SANITIZE_PROHIBITED_ON_PERSISTENT_MEMORY: NVME_STATUS_COMMAND_SPECIFIC_CODES = 35i32;
pub const NVME_STATUS_SGL_DATA_BLOCK_GRANULARITY_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 30i32;
pub const NVME_STATUS_SGL_DESCR_TYPE_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 17i32;
pub const NVME_STATUS_SGL_OFFSET_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 22i32;
pub const NVME_STATUS_STREAM_RESOURCE_ALLOCATION_FAILED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 127i32;
pub const NVME_STATUS_SUCCESS_COMPLETION: NVME_STATUS_GENERIC_COMMAND_CODES = 0i32;
pub type NVME_STATUS_TYPES = i32;
pub const NVME_STATUS_TYPE_COMMAND_SPECIFIC: NVME_STATUS_TYPES = 1i32;
pub const NVME_STATUS_TYPE_GENERIC_COMMAND: NVME_STATUS_TYPES = 0i32;
pub const NVME_STATUS_TYPE_MEDIA_ERROR: NVME_STATUS_TYPES = 2i32;
pub const NVME_STATUS_TYPE_VENDOR_SPECIFIC: NVME_STATUS_TYPES = 7i32;
pub const NVME_STATUS_ZONE_BOUNDARY_ERROR: NVME_STATUS_COMMAND_SPECIFIC_CODES = 184i32;
pub const NVME_STATUS_ZONE_FULL: NVME_STATUS_COMMAND_SPECIFIC_CODES = 185i32;
pub const NVME_STATUS_ZONE_INVALID_FORMAT: NVME_STATUS_COMMAND_SPECIFIC_CODES = 127i32;
pub const NVME_STATUS_ZONE_INVALID_STATE_TRANSITION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 191i32;
pub const NVME_STATUS_ZONE_INVALID_WRITE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 188i32;
pub const NVME_STATUS_ZONE_OFFLINE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 187i32;
pub const NVME_STATUS_ZONE_READ_ONLY: NVME_STATUS_COMMAND_SPECIFIC_CODES = 186i32;
pub const NVME_STATUS_ZONE_TOO_MANY_ACTIVE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 189i32;
pub const NVME_STATUS_ZONE_TOO_MANY_OPEN: NVME_STATUS_COMMAND_SPECIFIC_CODES = 190i32;
pub const NVME_STREAMS_GET_STATUS_MAX_IDS: u32 = 65535u32;
pub const NVME_STREAMS_ID_MAX: u32 = 65535u32;
pub const NVME_STREAMS_ID_MIN: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SUBMISSION_QUEUE_TAIL_DOORBELL {
    pub Anonymous: NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_0,
    pub AsUlong: u32,
}
impl Default for NVME_SUBMISSION_QUEUE_TAIL_DOORBELL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    pub LogIdentifier: u8,
    pub Reserved0: [u8; 4],
    pub OrganizationID: [u8; 3],
    pub Area1LastBlock: u16,
    pub Area2LastBlock: u16,
    pub Area3LastBlock: u16,
    pub Reserved1: [u8; 2],
    pub Area4LastBlock: u32,
    pub Reserved2: [u8; 362],
    pub ControllerInitiatedDataAvailable: u8,
    pub ControllerInitiatedDataGenerationNumber: u8,
    pub ReasonIdentifier: [u8; 128],
}
impl Default for NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_TELEMETRY_DATA_BLOCK_SIZE: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_TELEMETRY_HOST_INITIATED_LOG {
    pub LogIdentifier: u8,
    pub Reserved0: [u8; 4],
    pub OrganizationID: [u8; 3],
    pub Area1LastBlock: u16,
    pub Area2LastBlock: u16,
    pub Area3LastBlock: u16,
    pub Reserved1: [u8; 2],
    pub Area4LastBlock: u32,
    pub Reserved2: [u8; 361],
    pub HostInitiatedDataGenerationNumber: u8,
    pub ControllerInitiatedDataAvailable: u8,
    pub ControllerInitiatedDataGenerationNumber: u8,
    pub ReasonIdentifier: [u8; 128],
}
impl Default for NVME_TELEMETRY_HOST_INITIATED_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_TEMPERATURE_OVER_THRESHOLD: NVME_TEMPERATURE_THRESHOLD_TYPES = 0i32;
pub type NVME_TEMPERATURE_THRESHOLD_TYPES = i32;
pub const NVME_TEMPERATURE_UNDER_THRESHOLD: NVME_TEMPERATURE_THRESHOLD_TYPES = 1i32;
pub type NVME_VENDOR_LOG_PAGES = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_VERSION {
    pub Anonymous: NVME_VERSION_0,
    pub AsUlong: u32,
}
impl Default for NVME_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_VERSION_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_CAPABILITIES {
    pub Anonymous: NVME_WCS_DEVICE_CAPABILITIES_0,
}
impl Default for NVME_WCS_DEVICE_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_WCS_DEVICE_CAPABILITIES_0 {
    pub Anonymous: NVME_WCS_DEVICE_CAPABILITIES_0_0,
    pub AsULONG: u32,
}
impl Default for NVME_WCS_DEVICE_CAPABILITIES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_CAPABILITIES_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    pub PanicResetWaitTime: u16,
    pub PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION,
    pub DriveRecoveryAction: u8,
    pub PanicId: u64,
    pub DeviceCapabilitiesA: NVME_WCS_DEVICE_CAPABILITIES,
    pub VendorSpecificRecoveryCode: u8,
    pub Reserved0: [u8; 3],
    pub VendorSpecificCommandCDW12: u32,
    pub VendorSpecificCommandCDW13: u32,
    pub Reserved1: [u8; 466],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_WCS_DEVICE_ERROR_RECOVERY_LOG_VERSION_1: u32 = 1u32;
pub type NVME_WCS_DEVICE_RECOVERY_ACTION1 = i32;
pub type NVME_WCS_DEVICE_RECOVERY_ACTION2 = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_RESET_ACTION {
    pub Anonymous: NVME_WCS_DEVICE_RESET_ACTION_0,
}
impl Default for NVME_WCS_DEVICE_RESET_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_WCS_DEVICE_RESET_ACTION_0 {
    pub Anonymous: NVME_WCS_DEVICE_RESET_ACTION_0_0,
    pub AsUCHAR: u8,
}
impl Default for NVME_WCS_DEVICE_RESET_ACTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_RESET_ACTION_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    pub VersionSpecificData: [u8; 494],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    pub MediaUnitsWritten: [u8; 16],
    pub MediaUnitsRead: [u8; 16],
    pub BadUserNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0,
    pub BadSystemNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1,
    pub XORRecoveryCount: u64,
    pub UnrecoverableReadErrorCount: u64,
    pub SoftECCErrorCount: u64,
    pub EndToEndCorrectionCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_2,
    pub PercentageSystemDataUsed: u8,
    pub RefreshCount: [u8; 7],
    pub UserDataEraseCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_3,
    pub ThermalThrottling: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_4,
    pub Reserved0: [u8; 6],
    pub PCIeCorrectableErrorCount: u64,
    pub IncompleteShutdownCount: u32,
    pub Reserved1: u32,
    pub PercentageFreeBlocks: u8,
    pub Reserved2: [u8; 7],
    pub CapacitorHealth: u16,
    pub Reserved3: [u8; 6],
    pub UnalignedIOCount: u64,
    pub SecurityVersionNumber: u64,
    pub NUSE: u64,
    pub PLPStartCount: [u8; 16],
    pub EnduranceEstimate: [u8; 16],
    pub Reserved4: [u8; 302],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_sys::core::GUID,
}
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1 {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0 {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_2 {
    pub DetectedCounts: u32,
    pub CorrectedCounts: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_4 {
    pub EventCount: u8,
    pub Status: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_3 {
    pub MaximumCount: u32,
    pub MinimumCount: u32,
}
pub const NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_VERSION_2: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_ZONE_DESCRIPTOR {
    pub Anonymous1: NVME_ZONE_DESCRIPTOR_0,
    pub Anonymous2: NVME_ZONE_DESCRIPTOR_1,
    pub ZA: NVME_ZONE_DESCRIPTOR_2,
    pub Reserved3: [u8; 5],
    pub ZCAP: u64,
    pub ZSLBA: u64,
    pub WritePointer: u64,
    pub Reserved4: [u8; 32],
}
impl Default for NVME_ZONE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_ZONE_DESCRIPTOR_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_ZONE_DESCRIPTOR_1 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVME_ZONE_DESCRIPTOR_2 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_ZONE_DESCRIPTOR_EXTENSION {
    pub ZoneDescriptorExtensionInfo: [u8; 64],
}
impl Default for NVME_ZONE_DESCRIPTOR_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_ZONE_EXTENDED_REPORT_ZONE_DESC {
    pub ZoneDescriptor: NVME_ZONE_DESCRIPTOR,
    pub ZoneDescriptorExtension: [NVME_ZONE_DESCRIPTOR_EXTENSION; 1],
}
impl Default for NVME_ZONE_EXTENDED_REPORT_ZONE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_ZONE_RECEIVE_ACTION = i32;
pub type NVME_ZONE_RECEIVE_ACTION_SPECIFIC = i32;
pub const NVME_ZONE_RECEIVE_EXTENDED_REPORT_ZONES: NVME_ZONE_RECEIVE_ACTION = 1i32;
pub const NVME_ZONE_RECEIVE_REPORT_ZONES: NVME_ZONE_RECEIVE_ACTION = 0i32;
pub type NVME_ZONE_SEND_ACTION = i32;
pub const NVME_ZONE_SEND_CLOSE: NVME_ZONE_SEND_ACTION = 1i32;
pub const NVME_ZONE_SEND_FINISH: NVME_ZONE_SEND_ACTION = 2i32;
pub const NVME_ZONE_SEND_OFFLINE: NVME_ZONE_SEND_ACTION = 5i32;
pub const NVME_ZONE_SEND_OPEN: NVME_ZONE_SEND_ACTION = 3i32;
pub const NVME_ZONE_SEND_RESET: NVME_ZONE_SEND_ACTION = 4i32;
pub const NVME_ZONE_SEND_SET_ZONE_DESCRIPTOR: NVME_ZONE_SEND_ACTION = 16i32;
pub const NVME_ZRA_ALL_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 0i32;
pub const NVME_ZRA_CLOSED_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 4i32;
pub const NVME_ZRA_EMPTY_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 1i32;
pub const NVME_ZRA_EO_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 3i32;
pub const NVME_ZRA_FULL_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 5i32;
pub const NVME_ZRA_IO_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 2i32;
pub const NVME_ZRA_OFFLINE_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 7i32;
pub const NVME_ZRA_RO_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 6i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVM_RESERVATION_CAPABILITIES {
    pub Anonymous: NVM_RESERVATION_CAPABILITIES_0,
    pub AsUchar: u8,
}
impl Default for NVM_RESERVATION_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVM_RESERVATION_CAPABILITIES_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVM_SET_LIST {
    pub IdentifierCount: u8,
    pub Reserved: [u8; 127],
    pub Entry: [NVME_SET_ATTRIBUTES_ENTRY; 1],
}
impl Default for NVM_SET_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVMeDeviceRecovery1Max: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 15i32;
pub const NVMeDeviceRecovery2Max: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 15i32;
pub const NVMeDeviceRecoveryControllerReset: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 0i32;
pub const NVMeDeviceRecoveryDeviceReplacement: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 4i32;
pub const NVMeDeviceRecoveryFormatNVM: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 1i32;
pub const NVMeDeviceRecoveryNoAction: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 0i32;
pub const NVMeDeviceRecoveryPERST: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 3i32;
pub const NVMeDeviceRecoveryPcieFunctionReset: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 2i32;
pub const NVMeDeviceRecoveryPcieHotReset: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 5i32;
pub const NVMeDeviceRecoveryPowerCycle: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 4i32;
pub const NVMeDeviceRecoverySanitize: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 5i32;
pub const NVMeDeviceRecoverySubsystemReset: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 1i32;
pub const NVMeDeviceRecoveryVendorAnalysis: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 3i32;
pub const NVMeDeviceRecoveryVendorSpecificCommand: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TCG_ACTIVATE_METHOD_SPECIFIC {
    pub RangeStartLengthPolicy: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct TCG_ASSIGN_METHOD_SPECIFIC {
    pub NamespaceId: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct TCG_AUTH_METHOD_SPECIFIC {
    pub AuthorityId: u64,
    pub TriesCount: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TCG_BLOCKSID_METHOD_SPECIFIC {
    pub ClearEvents: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct TCG_HISTORY_ENTRY {
    pub VersionNumber: u8,
    pub EntryLength: u8,
    pub PowerCycleCount: u16,
    pub TcgCommandCount: u32,
    pub TcgCommandCompletionTS: u64,
    pub InvokingId: u64,
    pub MethodId: u64,
    pub ComId: u16,
    pub ProtocolId: u8,
    pub TcgStatus: u8,
    pub ProcessTime: u16,
    pub CommandSpecific: [u8; 10],
}
impl Default for TCG_HISTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TCG_HISTORY_ENTRY_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TCG_REACTIVATE_METHOD_SPECIFIC {
    pub RangeStartLengthPolicy: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UNSUPPORTED_REQUIREMENT {
    pub ReqId: [u8; 16],
}
impl Default for UNSUPPORTED_REQUIREMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ZONE_STATE = i32;
