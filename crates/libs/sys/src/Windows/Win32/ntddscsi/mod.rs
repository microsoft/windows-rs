pub const ATA_FLAGS_48BIT_COMMAND: u32 = 8;
pub const ATA_FLAGS_DATA_IN: u32 = 2;
pub const ATA_FLAGS_DATA_OUT: u32 = 4;
pub const ATA_FLAGS_DRDY_REQUIRED: u32 = 1;
pub const ATA_FLAGS_NO_MULTIPLE: u32 = 32;
pub const ATA_FLAGS_USE_DMA: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ATA_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl Default for ATA_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct ATA_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ATA_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ATA_PASS_THROUGH_EX {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: usize,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl Default for ATA_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct ATA_PASS_THROUGH_EX32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: u32,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ATA_PASS_THROUGH_EX32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BOOT_PARTITION_FUNCTION_ACTIVATE: u32 = 2;
pub const BOOT_PARTITION_FUNCTION_DOWNLOAD: u32 = 1;
pub const BOOT_PARTITION_FUNCTION_GET_INFO: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BOOT_PARTITION_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
pub const BOOT_PARTITION_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1;
pub const BOOT_PARTITION_STATUS_ERROR: u32 = 1;
pub const BOOT_PARTITION_STATUS_ILLEGAL_REQUEST: u32 = 2;
pub const BOOT_PARTITION_STATUS_INPUT_BUFFER_TOO_BIG: u32 = 4;
pub const BOOT_PARTITION_STATUS_INVALID_ALIGNMENT: u32 = 8;
pub const BOOT_PARTITION_STATUS_INVALID_IMAGE: u32 = 7;
pub const BOOT_PARTITION_STATUS_INVALID_PARAMETER: u32 = 3;
pub const BOOT_PARTITION_STATUS_INVALID_SLOT: u32 = 6;
pub const BOOT_PARTITION_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 5;
pub const BOOT_PARTITION_STATUS_SUCCESS: u32 = 0;
pub const BOOT_PARTITION_STATUS_WRITE_PROHIBITED: u32 = 9;
pub const DD_SCSI_DEVICE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("\\Device\\ScsiPort");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSM_NOTIFICATION_REQUEST_BLOCK {
    pub Size: u32,
    pub Version: u32,
    pub NotifyFlags: u32,
    pub DataSetProfile: u32,
    pub Reserved: [u32; 3],
    pub DataSetRangesCount: u32,
    pub DataSetRanges: [MP_DEVICE_DATA_SET_RANGE; 1],
}
impl Default for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DUMP_DEVICE_POWERON_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> i32>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DUMP_DRIVER {
    pub DumpDriverList: *mut core::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
}
impl Default for DUMP_DRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct DUMP_DRIVER_EX {
    pub DumpDriverList: *mut core::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
    pub DriverFullPath: NTSCSI_UNICODE_STRING,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DUMP_DRIVER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DUMP_DRIVER_NAME_LENGTH: u32 = 15;
pub const DUMP_EX_FLAG_DRIVER_FULL_PATH_SUPPORT: u32 = 8;
pub const DUMP_EX_FLAG_RESUME_SUPPORT: u32 = 4;
pub const DUMP_EX_FLAG_SUPPORT_64BITMEMORY: u32 = 1;
pub const DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DUMP_POINTERS {
    pub AdapterObject: *mut _ADAPTER_OBJECT,
    pub MappedRegisterBase: *mut core::ffi::c_void,
    pub DumpData: *mut core::ffi::c_void,
    pub CommonBufferVa: *mut core::ffi::c_void,
    pub CommonBufferPa: i64,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: bool,
    pub UseDiskDump: bool,
    pub Spare1: [u8; 2],
    pub DeviceObject: *mut core::ffi::c_void,
}
impl Default for DUMP_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct DUMP_POINTERS_EX {
    pub Header: DUMP_POINTERS_VERSION,
    pub DumpData: *mut core::ffi::c_void,
    pub CommonBufferVa: *mut core::ffi::c_void,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: bool,
    pub DeviceObject: *mut core::ffi::c_void,
    pub DriverList: *mut core::ffi::c_void,
    pub dwPortFlags: u32,
    pub MaxDeviceDumpSectionSize: u32,
    pub MaxDeviceDumpLevel: u32,
    pub MaxTransferSize: u32,
    pub AdapterObject: *mut core::ffi::c_void,
    pub MappedRegisterBase: *mut core::ffi::c_void,
    pub DeviceReady: super::winnt::PBOOLEAN,
    pub DumpDevicePowerOn: PDUMP_DEVICE_POWERON_ROUTINE,
    pub DumpDevicePowerOnContext: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DUMP_POINTERS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const DUMP_POINTERS_EX_V2_SIZE: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DUMP_POINTERS_EX_V2_SIZE: u32 = 48;
#[cfg(target_arch = "x86")]
pub const DUMP_POINTERS_EX_V3_SIZE: u32 = 60;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DUMP_POINTERS_EX_V3_SIZE: u32 = 88;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DUMP_POINTERS_VERSION {
    pub Version: u32,
    pub Size: u32,
}
pub const DUMP_POINTERS_VERSION_1: u32 = 1;
pub const DUMP_POINTERS_VERSION_2: u32 = 2;
pub const DUMP_POINTERS_VERSION_3: u32 = 3;
pub const DUMP_POINTERS_VERSION_4: u32 = 4;
pub const FILE_DEVICE_SCSI: u32 = 27;
pub const FIRMWARE_FUNCTION_ACTIVATE: u32 = 3;
pub const FIRMWARE_FUNCTION_DOWNLOAD: u32 = 2;
pub const FIRMWARE_FUNCTION_GET_INFO: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FIRMWARE_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
pub const FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1;
pub const FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1;
pub const FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4;
pub const FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2;
pub const FIRMWARE_REQUEST_FLAG_REPLACE_AND_SWITCH_UPON_RESET: u32 = 536870912;
pub const FIRMWARE_REQUEST_FLAG_REPLACE_EXISTING_IMAGE: u32 = 1073741824;
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648;
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_FIRMWARE_WITHOUT_RESET: u32 = 268435456;
pub const FIRMWARE_STATUS_COMMAND_ABORT: u32 = 133;
pub const FIRMWARE_STATUS_CONTROLLER_ERROR: u32 = 16;
pub const FIRMWARE_STATUS_DEVICE_ERROR: u32 = 64;
pub const FIRMWARE_STATUS_END_OF_MEDIA: u32 = 134;
pub const FIRMWARE_STATUS_ERROR: u32 = 1;
pub const FIRMWARE_STATUS_ID_NOT_FOUND: u32 = 131;
pub const FIRMWARE_STATUS_ILLEGAL_LENGTH: u32 = 135;
pub const FIRMWARE_STATUS_ILLEGAL_REQUEST: u32 = 2;
pub const FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG: u32 = 4;
pub const FIRMWARE_STATUS_INTERFACE_CRC_ERROR: u32 = 128;
pub const FIRMWARE_STATUS_INVALID_IMAGE: u32 = 7;
pub const FIRMWARE_STATUS_INVALID_PARAMETER: u32 = 3;
pub const FIRMWARE_STATUS_INVALID_SLOT: u32 = 6;
pub const FIRMWARE_STATUS_MEDIA_CHANGE: u32 = 130;
pub const FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST: u32 = 132;
pub const FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 5;
pub const FIRMWARE_STATUS_POWER_CYCLE_REQUIRED: u32 = 32;
pub const FIRMWARE_STATUS_SUCCESS: u32 = 0;
pub const FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR: u32 = 129;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_DEMOTE_BY_SIZE {
    pub Version: u32,
    pub Size: u32,
    pub SourcePriority: u8,
    pub TargetPriority: u8,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub LbaCount: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_DIRTY_THRESHOLDS {
    pub Version: u32,
    pub Size: u32,
    pub DirtyLowThreshold: u32,
    pub DirtyHighThreshold: u32,
}
pub const HYBRID_FUNCTION_DEMOTE_BY_SIZE: u32 = 19;
pub const HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM: u32 = 16;
pub const HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM: u32 = 17;
pub const HYBRID_FUNCTION_GET_INFO: u32 = 1;
pub const HYBRID_FUNCTION_SET_DIRTY_THRESHOLD: u32 = 18;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_INFORMATION {
    pub Version: u32,
    pub Size: u32,
    pub HybridSupported: bool,
    pub Status: NVCACHE_STATUS,
    pub CacheTypeEffective: NVCACHE_TYPE,
    pub CacheTypeDefault: NVCACHE_TYPE,
    pub FractionBase: u32,
    pub CacheSize: u64,
    pub Attributes: HYBRID_INFORMATION_0,
    pub Priorities: HYBRID_INFORMATION_1,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_INFORMATION_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HYBRID_INFORMATION_1 {
    pub PriorityLevelCount: u8,
    pub MaxPriorityBehavior: bool,
    pub OptimalWriteGranularity: u8,
    pub Reserved: u8,
    pub DirtyThresholdLow: u32,
    pub DirtyThresholdHigh: u32,
    pub SupportedCommands: HYBRID_INFORMATION_1_0,
    pub Priority: [NVCACHE_PRIORITY_LEVEL_DESCRIPTOR; 0],
}
impl Default for HYBRID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_INFORMATION_1_0 {
    pub _bitfield: u32,
    pub MaxEvictCommands: u32,
    pub MaxLbaRangeCountForEvict: u32,
    pub MaxLbaRangeCountForChangeLba: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
pub const HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1;
pub const HYBRID_REQUEST_INFO_STRUCTURE_VERSION: u32 = 1;
pub const HYBRID_STATUS_ENABLE_REFCOUNT_HOLD: u32 = 16;
pub const HYBRID_STATUS_ILLEGAL_REQUEST: u32 = 1;
pub const HYBRID_STATUS_INVALID_PARAMETER: u32 = 2;
pub const HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 3;
pub const HYBRID_STATUS_SUCCESS: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IDE_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnStatus: u32,
    pub DataLength: u32,
}
impl Default for IDE_IO_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IOCTL_ATA_MINIPORT: u32 = 315444;
pub const IOCTL_ATA_PASS_THROUGH: u32 = 315436;
pub const IOCTL_ATA_PASS_THROUGH_DIRECT: u32 = 315440;
pub const IOCTL_IDE_PASS_THROUGH: u32 = 315432;
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP: u32 = 315448;
pub const IOCTL_MINIPORT_SIGNATURE_BOOT_PARTITION: windows_sys::core::PCSTR = windows_sys::core::s!("BOOTPINF");
pub const IOCTL_MINIPORT_SIGNATURE_DSM_GENERAL: windows_sys::core::PCSTR = windows_sys::core::s!("MPDSMGEN");
pub const IOCTL_MINIPORT_SIGNATURE_DSM_NOTIFICATION: windows_sys::core::PCSTR = windows_sys::core::s!("MPDSM   ");
pub const IOCTL_MINIPORT_SIGNATURE_ENDURANCE_INFO: windows_sys::core::PCSTR = windows_sys::core::s!("ENDURINF");
pub const IOCTL_MINIPORT_SIGNATURE_FIRMWARE: windows_sys::core::PCSTR = windows_sys::core::s!("FIRMWARE");
pub const IOCTL_MINIPORT_SIGNATURE_HYBRDISK: windows_sys::core::PCSTR = windows_sys::core::s!("HYBRDISK");
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PHYSICAL_TOPOLOGY: windows_sys::core::PCSTR = windows_sys::core::s!("TOPOLOGY");
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PROTOCOL: windows_sys::core::PCSTR = windows_sys::core::s!("PROTOCOL");
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_TEMPERATURE: windows_sys::core::PCSTR = windows_sys::core::s!("TEMPERAT");
pub const IOCTL_MINIPORT_SIGNATURE_SCSIDISK: windows_sys::core::PCSTR = windows_sys::core::s!("SCSIDISK");
pub const IOCTL_MINIPORT_SIGNATURE_SET_PROTOCOL: windows_sys::core::PCSTR = windows_sys::core::s!("SETPROTO");
pub const IOCTL_MINIPORT_SIGNATURE_SET_TEMPERATURE_THRESHOLD: windows_sys::core::PCSTR = windows_sys::core::s!("SETTEMPT");
pub const IOCTL_MPIO_PASS_THROUGH_PATH: u32 = 315452;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT: u32 = 315456;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX: u32 = 315472;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX: u32 = 315468;
pub const IOCTL_SCSI_BASE: u32 = 4;
pub const IOCTL_SCSI_FREE_DUMP_POINTERS: u32 = 266276;
pub const IOCTL_SCSI_GET_ADDRESS: u32 = 266264;
pub const IOCTL_SCSI_GET_CAPABILITIES: u32 = 266256;
pub const IOCTL_SCSI_GET_DUMP_POINTERS: u32 = 266272;
pub const IOCTL_SCSI_GET_INQUIRY_DATA: u32 = 266252;
pub const IOCTL_SCSI_MINIPORT: u32 = 315400;
pub const IOCTL_SCSI_MINIPORT_BOOT_PARTITION: u32 = 1771408;
pub const IOCTL_SCSI_MINIPORT_DIAGNOSTIC: u32 = 1771776;
pub const IOCTL_SCSI_MINIPORT_FIRMWARE: u32 = 1771392;
pub const IOCTL_SCSI_MINIPORT_HYBRID: u32 = 1771040;
pub const IOCTL_SCSI_MINIPORT_NVCACHE: u32 = 1771008;
pub const IOCTL_SCSI_PASS_THROUGH: u32 = 315396;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT: u32 = 315412;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT_EX: u32 = 315464;
pub const IOCTL_SCSI_PASS_THROUGH_EX: u32 = 315460;
pub const IOCTL_SCSI_RESCAN_BUS: u32 = 266268;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IO_SCSI_CAPABILITIES {
    pub Length: u32,
    pub MaximumTransferLength: u32,
    pub MaximumPhysicalPages: u32,
    pub SupportedAsynchronousEvents: u32,
    pub AlignmentMask: u32,
    pub TaggedQueuing: bool,
    pub AdapterScansDown: bool,
    pub AdapterUsesPio: bool,
}
pub const MINIPORT_DSM_NOTIFICATION_VERSION: u32 = 1;
pub const MINIPORT_DSM_NOTIFICATION_VERSION_1: u32 = 1;
pub const MINIPORT_DSM_NOTIFY_FLAG_BEGIN: u32 = 1;
pub const MINIPORT_DSM_NOTIFY_FLAG_END: u32 = 2;
pub const MINIPORT_DSM_PROFILE_CRASHDUMP_FILE: u32 = 3;
pub const MINIPORT_DSM_PROFILE_HIBERNATION_FILE: u32 = 2;
pub const MINIPORT_DSM_PROFILE_PAGE_FILE: u32 = 1;
pub const MINIPORT_DSM_PROFILE_UNKNOWN: u32 = 0;
pub const MPIO_IOCTL_FLAG_INVOLVE_DSM: u32 = 4;
pub const MPIO_IOCTL_FLAG_USE_PATHID: u32 = 1;
pub const MPIO_IOCTL_FLAG_USE_SCSIADDRESS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH {
    pub PassThrough: SCSI_PASS_THROUGH,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH32 {
    pub PassThrough: SCSI_PASS_THROUGH32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32 {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MP_DEVICE_DATA_SET_RANGE {
    pub StartingOffset: i64,
    pub LengthInBytes: u64,
}
pub type MP_STORAGE_DIAGNOSTIC_LEVEL = i32;
pub type MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = i32;
pub const MpStorageDiagnosticLevelDefault: MP_STORAGE_DIAGNOSTIC_LEVEL = 0;
pub const MpStorageDiagnosticLevelMax: MP_STORAGE_DIAGNOSTIC_LEVEL = 1;
pub const MpStorageDiagnosticTargetTypeHbaFirmware: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 3;
pub const MpStorageDiagnosticTargetTypeMax: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 4;
pub const MpStorageDiagnosticTargetTypeMiniport: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 2;
pub const MpStorageDiagnosticTargetTypeUndefined: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 0;
pub const NRB_FUNCTION_ADD_LBAS_PINNED_SET: u32 = 16;
pub const NRB_FUNCTION_FLUSH_NVCACHE: u32 = 20;
pub const NRB_FUNCTION_NVCACHE_INFO: u32 = 236;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN: u32 = 1;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_SET: u32 = 0;
pub const NRB_FUNCTION_NVSEPARATED_FLUSH: u32 = 193;
pub const NRB_FUNCTION_NVSEPARATED_INFO: u32 = 192;
pub const NRB_FUNCTION_NVSEPARATED_WB_DISABLE: u32 = 194;
pub const NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT: u32 = 195;
pub const NRB_FUNCTION_PASS_HINT_PAYLOAD: u32 = 224;
pub const NRB_FUNCTION_QUERY_ASCENDER_STATUS: u32 = 208;
pub const NRB_FUNCTION_QUERY_CACHE_MISS: u32 = 19;
pub const NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS: u32 = 209;
pub const NRB_FUNCTION_QUERY_PINNED_SET: u32 = 18;
pub const NRB_FUNCTION_REMOVE_LBAS_PINNED_SET: u32 = 17;
pub const NRB_FUNCTION_SPINDLE_STATUS: u32 = 229;
pub const NRB_ILLEGAL_REQUEST: u32 = 1;
pub const NRB_INPUT_DATA_OVERRUN: u32 = 3;
pub const NRB_INPUT_DATA_UNDERRUN: u32 = 4;
pub const NRB_INVALID_PARAMETER: u32 = 2;
pub const NRB_OUTPUT_DATA_OVERRUN: u32 = 5;
pub const NRB_OUTPUT_DATA_UNDERRUN: u32 = 6;
pub const NRB_SUCCESS: u32 = 0;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct NTSCSI_UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: super::winnt::PWCH,
}
#[cfg(feature = "Win32_winnt")]
impl Default for NTSCSI_UNICODE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVCACHE_HINT_PAYLOAD {
    pub Command: u8,
    pub Feature7_0: u8,
    pub Feature15_8: u8,
    pub Count15_8: u8,
    pub LBA7_0: u8,
    pub LBA15_8: u8,
    pub LBA23_16: u8,
    pub LBA31_24: u8,
    pub LBA39_32: u8,
    pub LBA47_40: u8,
    pub Auxiliary7_0: u8,
    pub Auxiliary23_16: u8,
    pub Reserved: [u8; 4],
}
impl Default for NVCACHE_HINT_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    pub PriorityLevel: u8,
    pub Reserved0: [u8; 3],
    pub ConsumedNVMSizeFraction: u32,
    pub ConsumedMappingResourcesFraction: u32,
    pub ConsumedNVMSizeForDirtyDataFraction: u32,
    pub ConsumedMappingResourcesForDirtyDataFraction: u32,
    pub Reserved1: u32,
}
impl Default for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NVCACHE_REQUEST_BLOCK {
    pub NRBSize: u32,
    pub Function: u16,
    pub NRBFlags: u32,
    pub NRBStatus: u32,
    pub Count: u32,
    pub LBA: u64,
    pub DataBufSize: u32,
    pub NVCacheStatus: u32,
    pub NVCacheSubStatus: u32,
}
pub type NVCACHE_STATUS = i32;
pub type NVCACHE_TYPE = i32;
pub const NVSEPWriteCacheTypeNone: NV_SEP_WRITE_CACHE_TYPE = 1;
pub const NVSEPWriteCacheTypeUnknown: NV_SEP_WRITE_CACHE_TYPE = 0;
pub const NVSEPWriteCacheTypeWriteBack: NV_SEP_WRITE_CACHE_TYPE = 2;
pub const NVSEPWriteCacheTypeWriteThrough: NV_SEP_WRITE_CACHE_TYPE = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NV_FEATURE_PARAMETER {
    pub NVPowerModeEnabled: u16,
    pub NVParameterReserv1: u16,
    pub NVCmdEnabled: u16,
    pub NVParameterReserv2: u16,
    pub NVPowerModeVer: u16,
    pub NVCmdVer: u16,
    pub NVSize: u32,
    pub NVReadSpeed: u16,
    pub NVWrtSpeed: u16,
    pub DeviceSpinUpTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NV_SEP_CACHE_PARAMETER {
    pub Version: u32,
    pub Size: u32,
    pub Flags: NV_SEP_CACHE_PARAMETER_0,
    pub WriteCacheType: u8,
    pub WriteCacheTypeEffective: u8,
    pub ParameterReserve1: [u8; 3],
}
impl Default for NV_SEP_CACHE_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NV_SEP_CACHE_PARAMETER_0 {
    pub CacheFlags: NV_SEP_CACHE_PARAMETER_0_0,
    pub CacheFlagsSet: u8,
}
impl Default for NV_SEP_CACHE_PARAMETER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NV_SEP_CACHE_PARAMETER_0_0 {
    pub _bitfield: u8,
}
pub const NV_SEP_CACHE_PARAMETER_VERSION: u32 = 1;
pub const NV_SEP_CACHE_PARAMETER_VERSION_1: u32 = 1;
pub type NV_SEP_WRITE_CACHE_TYPE = i32;
pub const NvCacheStatusDisabled: NVCACHE_STATUS = 2;
pub const NvCacheStatusDisabling: NVCACHE_STATUS = 1;
pub const NvCacheStatusEnabled: NVCACHE_STATUS = 3;
pub const NvCacheStatusUnknown: NVCACHE_STATUS = 0;
pub const NvCacheTypeNone: NVCACHE_TYPE = 1;
pub const NvCacheTypeUnknown: NVCACHE_TYPE = 0;
pub const NvCacheTypeWriteBack: NVCACHE_TYPE = 2;
pub const NvCacheTypeWriteThrough: NVCACHE_TYPE = 3;
pub type PATA_PASS_THROUGH_DIRECT = *mut ATA_PASS_THROUGH_DIRECT;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PATA_PASS_THROUGH_DIRECT32 = *mut ATA_PASS_THROUGH_DIRECT32;
pub type PATA_PASS_THROUGH_EX = *mut ATA_PASS_THROUGH_EX;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PATA_PASS_THROUGH_EX32 = *mut ATA_PASS_THROUGH_EX32;
pub type PBOOT_PARTITION_REQUEST_BLOCK = *mut BOOT_PARTITION_REQUEST_BLOCK;
pub type PDSM_NOTIFICATION_REQUEST_BLOCK = *mut DSM_NOTIFICATION_REQUEST_BLOCK;
pub type PDUMP_DEVICE_POWERON_ROUTINE = *mut DUMP_DEVICE_POWERON_ROUTINE;
pub type PDUMP_DRIVER = *mut DUMP_DRIVER;
#[cfg(feature = "Win32_winnt")]
pub type PDUMP_DRIVER_EX = *mut DUMP_DRIVER_EX;
pub type PDUMP_POINTERS = *mut DUMP_POINTERS;
#[cfg(feature = "Win32_winnt")]
pub type PDUMP_POINTERS_EX = *mut DUMP_POINTERS_EX;
pub type PDUMP_POINTERS_VERSION = *mut DUMP_POINTERS_VERSION;
pub type PFIRMWARE_REQUEST_BLOCK = *mut FIRMWARE_REQUEST_BLOCK;
pub type PHYBRID_DEMOTE_BY_SIZE = *mut HYBRID_DEMOTE_BY_SIZE;
pub type PHYBRID_DIRTY_THRESHOLDS = *mut HYBRID_DIRTY_THRESHOLDS;
pub type PHYBRID_INFORMATION = *mut HYBRID_INFORMATION;
pub type PHYBRID_REQUEST_BLOCK = *mut HYBRID_REQUEST_BLOCK;
pub type PIDE_IO_CONTROL = *mut IDE_IO_CONTROL;
pub type PIO_SCSI_CAPABILITIES = *mut IO_SCSI_CAPABILITIES;
pub type PMPIO_PASS_THROUGH_PATH = *mut MPIO_PASS_THROUGH_PATH;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PMPIO_PASS_THROUGH_PATH32 = *mut MPIO_PASS_THROUGH_PATH32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PMPIO_PASS_THROUGH_PATH32_EX = *mut MPIO_PASS_THROUGH_PATH32_EX;
pub type PMPIO_PASS_THROUGH_PATH_DIRECT = *mut MPIO_PASS_THROUGH_PATH_DIRECT;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PMPIO_PASS_THROUGH_PATH_DIRECT32 = *mut MPIO_PASS_THROUGH_PATH_DIRECT32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PMPIO_PASS_THROUGH_PATH_DIRECT32_EX = *mut MPIO_PASS_THROUGH_PATH_DIRECT32_EX;
pub type PMPIO_PASS_THROUGH_PATH_DIRECT_EX = *mut MPIO_PASS_THROUGH_PATH_DIRECT_EX;
pub type PMPIO_PASS_THROUGH_PATH_EX = *mut MPIO_PASS_THROUGH_PATH_EX;
pub type PMP_DEVICE_DATA_SET_RANGE = *mut MP_DEVICE_DATA_SET_RANGE;
pub type PMP_STORAGE_DIAGNOSTIC_LEVEL = *mut MP_STORAGE_DIAGNOSTIC_LEVEL;
pub type PMP_STORAGE_DIAGNOSTIC_TARGET_TYPE = *mut MP_STORAGE_DIAGNOSTIC_TARGET_TYPE;
#[cfg(feature = "Win32_winnt")]
pub type PNTSCSI_UNICODE_STRING = *mut NTSCSI_UNICODE_STRING;
pub type PNVCACHE_HINT_PAYLOAD = *mut NVCACHE_HINT_PAYLOAD;
pub type PNVCACHE_PRIORITY_LEVEL_DESCRIPTOR = *mut NVCACHE_PRIORITY_LEVEL_DESCRIPTOR;
pub type PNVCACHE_REQUEST_BLOCK = *mut NVCACHE_REQUEST_BLOCK;
pub type PNV_FEATURE_PARAMETER = *mut NV_FEATURE_PARAMETER;
pub type PNV_SEP_CACHE_PARAMETER = *mut NV_SEP_CACHE_PARAMETER;
pub type PNV_SEP_WRITE_CACHE_TYPE = *mut NV_SEP_WRITE_CACHE_TYPE;
pub type PSCSI_ADAPTER_BUS_INFO = *mut SCSI_ADAPTER_BUS_INFO;
pub type PSCSI_ADDRESS = *mut SCSI_ADDRESS;
pub type PSCSI_BUS_DATA = *mut SCSI_BUS_DATA;
pub type PSCSI_INQUIRY_DATA = *mut SCSI_INQUIRY_DATA;
pub type PSCSI_PASS_THROUGH = *mut SCSI_PASS_THROUGH;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSCSI_PASS_THROUGH32 = *mut SCSI_PASS_THROUGH32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSCSI_PASS_THROUGH32_EX = *mut SCSI_PASS_THROUGH32_EX;
pub type PSCSI_PASS_THROUGH_DIRECT = *mut SCSI_PASS_THROUGH_DIRECT;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSCSI_PASS_THROUGH_DIRECT32 = *mut SCSI_PASS_THROUGH_DIRECT32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSCSI_PASS_THROUGH_DIRECT32_EX = *mut SCSI_PASS_THROUGH_DIRECT32_EX;
pub type PSCSI_PASS_THROUGH_DIRECT_EX = *mut SCSI_PASS_THROUGH_DIRECT_EX;
pub type PSCSI_PASS_THROUGH_EX = *mut SCSI_PASS_THROUGH_EX;
pub type PSRB_IO_CONTROL = *mut SRB_IO_CONTROL;
pub type PSTORAGE_BOOT_PARTITION_ACTIVATE = *mut STORAGE_BOOT_PARTITION_ACTIVATE;
pub type PSTORAGE_BOOT_PARTITION_DOWNLOAD = *mut STORAGE_BOOT_PARTITION_DOWNLOAD;
pub type PSTORAGE_BOOT_PARTITION_INFO = *mut STORAGE_BOOT_PARTITION_INFO;
pub type PSTORAGE_DIAGNOSTIC_MP_REQUEST = *mut STORAGE_DIAGNOSTIC_MP_REQUEST;
pub type PSTORAGE_ENDURANCE_DATA_DESCRIPTOR = *mut STORAGE_ENDURANCE_DATA_DESCRIPTOR;
pub type PSTORAGE_ENDURANCE_INFO = *mut STORAGE_ENDURANCE_INFO;
pub type PSTORAGE_FIRMWARE_ACTIVATE = *mut STORAGE_FIRMWARE_ACTIVATE;
pub type PSTORAGE_FIRMWARE_DOWNLOAD = *mut STORAGE_FIRMWARE_DOWNLOAD;
pub type PSTORAGE_FIRMWARE_DOWNLOAD_V2 = *mut STORAGE_FIRMWARE_DOWNLOAD_V2;
pub type PSTORAGE_FIRMWARE_INFO = *mut STORAGE_FIRMWARE_INFO;
pub type PSTORAGE_FIRMWARE_INFO_V2 = *mut STORAGE_FIRMWARE_INFO_V2;
pub type PSTORAGE_FIRMWARE_SLOT_INFO = *mut STORAGE_FIRMWARE_SLOT_INFO;
pub type PSTORAGE_FIRMWARE_SLOT_INFO_V2 = *mut STORAGE_FIRMWARE_SLOT_INFO_V2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCSI_ADAPTER_BUS_INFO {
    pub NumberOfBuses: u8,
    pub BusData: [SCSI_BUS_DATA; 1],
}
impl Default for SCSI_ADAPTER_BUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCSI_ADDRESS {
    pub Length: u32,
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCSI_BUS_DATA {
    pub NumberOfLogicalUnits: u8,
    pub InitiatorBusId: u8,
    pub InquiryDataOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCSI_INQUIRY_DATA {
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub DeviceClaimed: bool,
    pub InquiryDataLength: u32,
    pub NextInquiryDataOffset: u32,
    pub InquiryData: [u8; 1],
}
impl Default for SCSI_INQUIRY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCSI_IOCTL_DATA_BIDIRECTIONAL: u32 = 3;
pub const SCSI_IOCTL_DATA_IN: u32 = 1;
pub const SCSI_IOCTL_DATA_OUT: u32 = 0;
pub const SCSI_IOCTL_DATA_UNSPECIFIED: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: usize,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl Default for SCSI_PASS_THROUGH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: u32,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SCSI_PASS_THROUGH32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: u32,
    pub DataInBufferOffset: u32,
    pub Cdb: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SCSI_PASS_THROUGH32_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl Default for SCSI_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SCSI_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH_DIRECT32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut core::ffi::c_void,
    pub DataInBuffer: *mut core::ffi::c_void,
    pub Cdb: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH_DIRECT_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut core::ffi::c_void,
    pub DataInBuffer: *mut core::ffi::c_void,
    pub Cdb: [u8; 1],
}
impl Default for SCSI_PASS_THROUGH_DIRECT_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: usize,
    pub DataInBufferOffset: usize,
    pub Cdb: [u8; 1],
}
impl Default for SCSI_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SRB_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnCode: u32,
    pub Length: u32,
}
impl Default for SRB_IO_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_BOOT_PARTITION_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub BPID: u8,
    pub Reserved0: [u8; 3],
}
impl Default for STORAGE_BOOT_PARTITION_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_BOOT_PARTITION_ACTIVATE_STRUCTURE_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_BOOT_PARTITION_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub BPID: u8,
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageBuffer: [u8; 1],
}
impl Default for STORAGE_BOOT_PARTITION_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_BOOT_PARTITION_DOWNLOAD_STRUCTURE_VERSION: u32 = 1;
pub const STORAGE_BOOT_PARTITION_GET_INFO_STRUCTURE_VERSION_V1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STORAGE_BOOT_PARTITION_INFO {
    pub Version: u32,
    pub Size: u32,
    pub BPSZ: u64,
    pub ImagePayloadAlignment: u32,
    pub ImagePayloadMaxSize: u32,
    pub SlotCount: u8,
    pub ABPID: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_DIAGNOSTIC_MP_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub TargetType: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: MP_STORAGE_DIAGNOSTIC_LEVEL,
    pub ProviderId: windows_sys::core::GUID,
    pub BufferSize: u32,
    pub Reserved: u32,
    pub DataBuffer: [u8; 1],
}
impl Default for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_DIAGNOSTIC_STATUS_BUFFER_TOO_SMALL: u32 = 1;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_PARAMETER: u32 = 3;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_SIGNATURE: u32 = 4;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_TARGET_TYPE: u32 = 5;
pub const STORAGE_DIAGNOSTIC_STATUS_MORE_DATA: u32 = 6;
pub const STORAGE_DIAGNOSTIC_STATUS_SUCCESS: u32 = 0;
pub const STORAGE_DIAGNOSTIC_STATUS_UNSUPPORTED_VERSION: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub EnduranceInfo: STORAGE_ENDURANCE_INFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_ENDURANCE_INFO {
    pub ValidFields: u32,
    pub GroupId: u32,
    pub Flags: STORAGE_ENDURANCE_INFO_0,
    pub LifePercentage: u32,
    pub BytesReadCount: [u8; 16],
    pub ByteWriteCount: [u8; 16],
}
impl Default for STORAGE_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STORAGE_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub SlotToActivate: u8,
    pub Reserved0: [u8; 3],
}
impl Default for STORAGE_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageBuffer: [u8; 0],
}
impl Default for STORAGE_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION: u32 = 1;
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_DOWNLOAD_V2 {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub ImageSize: u32,
    pub ImageBuffer: [u8; 0],
}
impl Default for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: bool,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub Reserved: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO; 0],
}
impl Default for STORAGE_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_FIRMWARE_INFO_INVALID_SLOT: u32 = 255;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION: u32 = 1;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_INFO_V2 {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: bool,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub FirmwareShared: bool,
    pub Reserved: [u8; 3],
    pub ImagePayloadAlignment: u32,
    pub ImagePayloadMaxSize: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO_V2; 0],
}
impl Default for STORAGE_FIRMWARE_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_SLOT_INFO {
    pub SlotNumber: u8,
    pub ReadOnly: bool,
    pub Reserved: [u8; 6],
    pub Revision: STORAGE_FIRMWARE_SLOT_INFO_0,
}
impl Default for STORAGE_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_FIRMWARE_SLOT_INFO_0 {
    pub Info: [u8; 8],
    pub AsUlonglong: u64,
}
impl Default for STORAGE_FIRMWARE_SLOT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_SLOT_INFO_V2 {
    pub SlotNumber: u8,
    pub ReadOnly: bool,
    pub Reserved: [u8; 6],
    pub Revision: [u8; 16],
}
impl Default for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH: u32 = 16;
pub const ScsiRawInterfaceGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x53f56309_b6bf_11d0_94f2_00a0c91efb8b);
pub const WmiScsiAddressGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x53f5630f_b6bf_11d0_94f2_00a0c91efb8b);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _ADAPTER_OBJECT(pub u8);
