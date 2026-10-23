pub const ATA_FLAGS_48BIT_COMMAND: i32 = 8;
pub const ATA_FLAGS_DATA_IN: i32 = 2;
pub const ATA_FLAGS_DATA_OUT: i32 = 4;
pub const ATA_FLAGS_DRDY_REQUIRED: i32 = 1;
pub const ATA_FLAGS_NO_MULTIPLE: i32 = 32;
pub const ATA_FLAGS_USE_DMA: i32 = 16;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ATA_PASS_THROUGH_DIRECT {
    pub Length: super::USHORT,
    pub AtaFlags: super::USHORT,
    pub PathId: super::UCHAR,
    pub TargetId: super::UCHAR,
    pub Lun: super::UCHAR,
    pub ReservedAsUchar: super::UCHAR,
    pub DataTransferLength: super::ULONG,
    pub TimeOutValue: super::ULONG,
    pub ReservedAsUlong: super::ULONG,
    pub DataBuffer: super::PVOID,
    pub PreviousTaskFile: [super::UCHAR; 8],
    pub CurrentTaskFile: [super::UCHAR; 8],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for ATA_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ATA_PASS_THROUGH_EX {
    pub Length: super::USHORT,
    pub AtaFlags: super::USHORT,
    pub PathId: super::UCHAR,
    pub TargetId: super::UCHAR,
    pub Lun: super::UCHAR,
    pub ReservedAsUchar: super::UCHAR,
    pub DataTransferLength: super::ULONG,
    pub TimeOutValue: super::ULONG,
    pub ReservedAsUlong: super::ULONG,
    pub DataBufferOffset: super::ULONG_PTR,
    pub PreviousTaskFile: [super::UCHAR; 8],
    pub CurrentTaskFile: [super::UCHAR; 8],
}
#[cfg(feature = "shared")]
impl Default for ATA_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct ATA_PASS_THROUGH_EX32 {
    pub Length: super::USHORT,
    pub AtaFlags: super::USHORT,
    pub PathId: super::UCHAR,
    pub TargetId: super::UCHAR,
    pub Lun: super::UCHAR,
    pub ReservedAsUchar: super::UCHAR,
    pub DataTransferLength: super::ULONG,
    pub TimeOutValue: super::ULONG,
    pub ReservedAsUlong: super::ULONG,
    pub DataBufferOffset: super::ULONG32,
    pub PreviousTaskFile: [super::UCHAR; 8],
    pub CurrentTaskFile: [super::UCHAR; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for ATA_PASS_THROUGH_EX32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BOOT_PARTITION_FUNCTION_ACTIVATE: i32 = 2;
pub const BOOT_PARTITION_FUNCTION_DOWNLOAD: i32 = 1;
pub const BOOT_PARTITION_FUNCTION_GET_INFO: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BOOT_PARTITION_REQUEST_BLOCK {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub Function: super::ULONG,
    pub Flags: super::ULONG,
    pub DataBufferOffset: super::ULONG,
    pub DataBufferLength: super::ULONG,
}
pub const BOOT_PARTITION_REQUEST_BLOCK_STRUCTURE_VERSION: i32 = 1;
pub const BOOT_PARTITION_STATUS_ERROR: i32 = 1;
pub const BOOT_PARTITION_STATUS_ILLEGAL_REQUEST: i32 = 2;
pub const BOOT_PARTITION_STATUS_INPUT_BUFFER_TOO_BIG: i32 = 4;
pub const BOOT_PARTITION_STATUS_INVALID_ALIGNMENT: i32 = 8;
pub const BOOT_PARTITION_STATUS_INVALID_IMAGE: i32 = 7;
pub const BOOT_PARTITION_STATUS_INVALID_PARAMETER: i32 = 3;
pub const BOOT_PARTITION_STATUS_INVALID_SLOT: i32 = 6;
pub const BOOT_PARTITION_STATUS_OUTPUT_BUFFER_TOO_SMALL: i32 = 5;
pub const BOOT_PARTITION_STATUS_SUCCESS: i32 = 0;
pub const BOOT_PARTITION_STATUS_WRITE_PROHIBITED: i32 = 9;
pub const DD_SCSI_DEVICE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("\\Device\\ScsiPort");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DSM_NOTIFICATION_REQUEST_BLOCK {
    pub Size: super::ULONG,
    pub Version: super::ULONG,
    pub NotifyFlags: super::ULONG,
    pub DataSetProfile: super::ULONG,
    pub Reserved: [super::ULONG; 3],
    pub DataSetRangesCount: super::ULONG,
    pub DataSetRanges: [MP_DEVICE_DATA_SET_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type DUMP_DEVICE_POWERON_ROUTINE = Option<unsafe extern "C" fn(context: super::PVOID) -> super::LONG>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DUMP_DRIVER {
    pub DumpDriverList: super::PVOID,
    pub DriverName: [super::WCHAR; 15],
    pub BaseName: [super::WCHAR; 15],
}
#[cfg(feature = "winnt")]
impl Default for DUMP_DRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DUMP_DRIVER_EX {
    pub DumpDriverList: super::PVOID,
    pub DriverName: [super::WCHAR; 15],
    pub BaseName: [super::WCHAR; 15],
    pub DriverFullPath: NTSCSI_UNICODE_STRING,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DUMP_DRIVER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DUMP_DRIVER_NAME_LENGTH: i32 = 15;
pub const DUMP_EX_FLAG_DRIVER_FULL_PATH_SUPPORT: i32 = 8;
pub const DUMP_EX_FLAG_RESUME_SUPPORT: i32 = 4;
pub const DUMP_EX_FLAG_SUPPORT_64BITMEMORY: i32 = 1;
pub const DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DUMP_POINTERS {
    pub AdapterObject: *mut _ADAPTER_OBJECT,
    pub MappedRegisterBase: super::PVOID,
    pub DumpData: super::PVOID,
    pub CommonBufferVa: super::PVOID,
    pub CommonBufferPa: super::LARGE_INTEGER,
    pub CommonBufferSize: super::ULONG,
    pub AllocateCommonBuffers: super::BOOLEAN,
    pub UseDiskDump: super::BOOLEAN,
    pub Spare1: [super::UCHAR; 2],
    pub DeviceObject: super::PVOID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DUMP_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DUMP_POINTERS_EX {
    pub Header: DUMP_POINTERS_VERSION,
    pub DumpData: super::PVOID,
    pub CommonBufferVa: super::PVOID,
    pub CommonBufferSize: super::ULONG,
    pub AllocateCommonBuffers: super::BOOLEAN,
    pub DeviceObject: super::PVOID,
    pub DriverList: super::PVOID,
    pub dwPortFlags: super::ULONG,
    pub MaxDeviceDumpSectionSize: super::ULONG,
    pub MaxDeviceDumpLevel: super::ULONG,
    pub MaxTransferSize: super::ULONG,
    pub AdapterObject: super::PVOID,
    pub MappedRegisterBase: super::PVOID,
    pub DeviceReady: super::PBOOLEAN,
    pub DumpDevicePowerOn: PDUMP_DEVICE_POWERON_ROUTINE,
    pub DumpDevicePowerOnContext: super::PVOID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const DUMP_POINTERS_EX_V2_SIZE: super::ULONG = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const DUMP_POINTERS_EX_V2_SIZE: super::ULONG = 48;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const DUMP_POINTERS_EX_V3_SIZE: super::ULONG = 60;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const DUMP_POINTERS_EX_V3_SIZE: super::ULONG = 88;
#[cfg(target_arch = "x86")]
pub const DUMP_POINTERS_EX_V4_SIZE: u32 = 68;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DUMP_POINTERS_EX_V4_SIZE: u64 = 104;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DUMP_POINTERS_VERSION {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
}
pub const DUMP_POINTERS_VERSION_1: i32 = 1;
pub const DUMP_POINTERS_VERSION_2: i32 = 2;
pub const DUMP_POINTERS_VERSION_3: i32 = 3;
pub const DUMP_POINTERS_VERSION_4: i32 = 4;
pub const FILE_DEVICE_SCSI: i32 = 27;
pub const FIRMWARE_FUNCTION_ACTIVATE: i32 = 3;
pub const FIRMWARE_FUNCTION_DOWNLOAD: i32 = 2;
pub const FIRMWARE_FUNCTION_GET_INFO: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FIRMWARE_REQUEST_BLOCK {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub Function: super::ULONG,
    pub Flags: super::ULONG,
    pub DataBufferOffset: super::ULONG,
    pub DataBufferLength: super::ULONG,
}
pub const FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION: i32 = 1;
pub const FIRMWARE_REQUEST_FLAG_CONTROLLER: i32 = 1;
pub const FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: i32 = 4;
pub const FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: i32 = 2;
pub const FIRMWARE_REQUEST_FLAG_REPLACE_AND_SWITCH_UPON_RESET: i32 = 536870912;
pub const FIRMWARE_REQUEST_FLAG_REPLACE_EXISTING_IMAGE: i32 = 1073741824;
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648;
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_FIRMWARE_WITHOUT_RESET: i32 = 268435456;
pub const FIRMWARE_STATUS_COMMAND_ABORT: i32 = 133;
pub const FIRMWARE_STATUS_CONTROLLER_ERROR: i32 = 16;
pub const FIRMWARE_STATUS_DEVICE_ERROR: i32 = 64;
pub const FIRMWARE_STATUS_END_OF_MEDIA: i32 = 134;
pub const FIRMWARE_STATUS_ERROR: i32 = 1;
pub const FIRMWARE_STATUS_ID_NOT_FOUND: i32 = 131;
pub const FIRMWARE_STATUS_ILLEGAL_LENGTH: i32 = 135;
pub const FIRMWARE_STATUS_ILLEGAL_REQUEST: i32 = 2;
pub const FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG: i32 = 4;
pub const FIRMWARE_STATUS_INTERFACE_CRC_ERROR: i32 = 128;
pub const FIRMWARE_STATUS_INVALID_IMAGE: i32 = 7;
pub const FIRMWARE_STATUS_INVALID_PARAMETER: i32 = 3;
pub const FIRMWARE_STATUS_INVALID_SLOT: i32 = 6;
pub const FIRMWARE_STATUS_MEDIA_CHANGE: i32 = 130;
pub const FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST: i32 = 132;
pub const FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL: i32 = 5;
pub const FIRMWARE_STATUS_POWER_CYCLE_REQUIRED: i32 = 32;
pub const FIRMWARE_STATUS_SUCCESS: i32 = 0;
pub const FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR: i32 = 129;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_DEMOTE_BY_SIZE {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub SourcePriority: super::UCHAR,
    pub TargetPriority: super::UCHAR,
    pub Reserved0: super::USHORT,
    pub Reserved1: super::ULONG,
    pub LbaCount: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_DIRTY_THRESHOLDS {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub DirtyLowThreshold: super::ULONG,
    pub DirtyHighThreshold: super::ULONG,
}
pub const HYBRID_FUNCTION_DEMOTE_BY_SIZE: i32 = 19;
pub const HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM: i32 = 16;
pub const HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM: i32 = 17;
pub const HYBRID_FUNCTION_GET_INFO: i32 = 1;
pub const HYBRID_FUNCTION_SET_DIRTY_THRESHOLD: i32 = 18;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_INFORMATION {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub HybridSupported: super::BOOLEAN,
    pub Status: NVCACHE_STATUS,
    pub CacheTypeEffective: NVCACHE_TYPE,
    pub CacheTypeDefault: NVCACHE_TYPE,
    pub FractionBase: super::ULONG,
    pub CacheSize: super::ULONGLONG,
    pub Attributes: HYBRID_INFORMATION_0,
    pub Priorities: HYBRID_INFORMATION_1,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_INFORMATION_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HYBRID_INFORMATION_1 {
    pub PriorityLevelCount: super::UCHAR,
    pub MaxPriorityBehavior: super::BOOLEAN,
    pub OptimalWriteGranularity: super::UCHAR,
    pub Reserved: super::UCHAR,
    pub DirtyThresholdLow: super::ULONG,
    pub DirtyThresholdHigh: super::ULONG,
    pub SupportedCommands: HYBRID_INFORMATION_1_0,
    pub Priority: [NVCACHE_PRIORITY_LEVEL_DESCRIPTOR; 0],
}
#[cfg(feature = "shared")]
impl Default for HYBRID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_INFORMATION_1_0 {
    pub _bitfield: u32,
    pub MaxEvictCommands: super::ULONG,
    pub MaxLbaRangeCountForEvict: super::ULONG,
    pub MaxLbaRangeCountForChangeLba: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HYBRID_REQUEST_BLOCK {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub Function: super::ULONG,
    pub Flags: super::ULONG,
    pub DataBufferOffset: super::ULONG,
    pub DataBufferLength: super::ULONG,
}
pub const HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION: i32 = 1;
pub const HYBRID_REQUEST_INFO_STRUCTURE_VERSION: i32 = 1;
pub const HYBRID_STATUS_ENABLE_REFCOUNT_HOLD: i32 = 16;
pub const HYBRID_STATUS_ILLEGAL_REQUEST: i32 = 1;
pub const HYBRID_STATUS_INVALID_PARAMETER: i32 = 2;
pub const HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL: i32 = 3;
pub const HYBRID_STATUS_SUCCESS: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IDE_IO_CONTROL {
    pub HeaderLength: super::ULONG,
    pub Signature: [super::UCHAR; 8],
    pub Timeout: super::ULONG,
    pub ControlCode: super::ULONG,
    pub ReturnStatus: super::ULONG,
    pub DataLength: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for IDE_IO_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IOCTL_ATA_MINIPORT: i32 = 315444;
pub const IOCTL_ATA_PASS_THROUGH: i32 = 315436;
pub const IOCTL_ATA_PASS_THROUGH_DIRECT: i32 = 315440;
pub const IOCTL_IDE_PASS_THROUGH: i32 = 315432;
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP: i32 = 315448;
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
pub const IOCTL_MPIO_PASS_THROUGH_PATH: i32 = 315452;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT: i32 = 315456;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX: i32 = 315472;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX: i32 = 315468;
pub const IOCTL_SCSI_BASE: i32 = 4;
pub const IOCTL_SCSI_FREE_DUMP_POINTERS: i32 = 266276;
pub const IOCTL_SCSI_GET_ADDRESS: i32 = 266264;
pub const IOCTL_SCSI_GET_CAPABILITIES: i32 = 266256;
pub const IOCTL_SCSI_GET_DUMP_POINTERS: i32 = 266272;
pub const IOCTL_SCSI_GET_INQUIRY_DATA: i32 = 266252;
pub const IOCTL_SCSI_MINIPORT: i32 = 315400;
pub const IOCTL_SCSI_MINIPORT_BOOT_PARTITION: i32 = 1771408;
pub const IOCTL_SCSI_MINIPORT_DIAGNOSTIC: i32 = 1771776;
pub const IOCTL_SCSI_MINIPORT_FIRMWARE: i32 = 1771392;
pub const IOCTL_SCSI_MINIPORT_HYBRID: i32 = 1771040;
pub const IOCTL_SCSI_MINIPORT_NVCACHE: i32 = 1771008;
pub const IOCTL_SCSI_PASS_THROUGH: i32 = 315396;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT: i32 = 315412;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT_EX: i32 = 315464;
pub const IOCTL_SCSI_PASS_THROUGH_EX: i32 = 315460;
pub const IOCTL_SCSI_RESCAN_BUS: i32 = 266268;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IO_SCSI_CAPABILITIES {
    pub Length: super::ULONG,
    pub MaximumTransferLength: super::ULONG,
    pub MaximumPhysicalPages: super::ULONG,
    pub SupportedAsynchronousEvents: super::ULONG,
    pub AlignmentMask: super::ULONG,
    pub TaggedQueuing: super::BOOLEAN,
    pub AdapterScansDown: super::BOOLEAN,
    pub AdapterUsesPio: super::BOOLEAN,
}
pub const MINIPORT_DSM_NOTIFICATION_VERSION: i32 = 1;
pub const MINIPORT_DSM_NOTIFICATION_VERSION_1: i32 = 1;
pub const MINIPORT_DSM_NOTIFY_FLAG_BEGIN: i32 = 1;
pub const MINIPORT_DSM_NOTIFY_FLAG_END: i32 = 2;
pub const MINIPORT_DSM_PROFILE_CRASHDUMP_FILE: i32 = 3;
pub const MINIPORT_DSM_PROFILE_HIBERNATION_FILE: i32 = 2;
pub const MINIPORT_DSM_PROFILE_PAGE_FILE: i32 = 1;
pub const MINIPORT_DSM_PROFILE_UNKNOWN: i32 = 0;
pub const MPIO_IOCTL_FLAG_INVOLVE_DSM: i32 = 4;
pub const MPIO_IOCTL_FLAG_USE_PATHID: i32 = 1;
pub const MPIO_IOCTL_FLAG_USE_SCSIADDRESS: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH {
    pub PassThrough: SCSI_PASS_THROUGH,
    pub Version: super::ULONG,
    pub Length: super::USHORT,
    pub Flags: super::UCHAR,
    pub PortNumber: super::UCHAR,
    pub MpioPathId: super::ULONGLONG,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH32 {
    pub PassThrough: SCSI_PASS_THROUGH32,
    pub Version: super::ULONG,
    pub Length: super::USHORT,
    pub Flags: super::UCHAR,
    pub PortNumber: super::UCHAR,
    pub MpioPathId: super::ULONGLONG,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH32_EX {
    pub PassThroughOffset: super::ULONG,
    pub Version: super::ULONG,
    pub Length: super::USHORT,
    pub Flags: super::UCHAR,
    pub PortNumber: super::UCHAR,
    pub MpioPathId: super::ULONGLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT,
    pub Version: super::ULONG,
    pub Length: super::USHORT,
    pub Flags: super::UCHAR,
    pub PortNumber: super::UCHAR,
    pub MpioPathId: super::ULONGLONG,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    pub PassThroughOffset: super::ULONG,
    pub Version: super::ULONG,
    pub Length: super::USHORT,
    pub Flags: super::UCHAR,
    pub PortNumber: super::UCHAR,
    pub MpioPathId: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    pub PassThroughOffset: super::ULONG,
    pub Version: super::ULONG,
    pub Length: super::USHORT,
    pub Flags: super::UCHAR,
    pub PortNumber: super::UCHAR,
    pub MpioPathId: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MPIO_PASS_THROUGH_PATH_EX {
    pub PassThroughOffset: super::ULONG,
    pub Version: super::ULONG,
    pub Length: super::USHORT,
    pub Flags: super::UCHAR,
    pub PortNumber: super::UCHAR,
    pub MpioPathId: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MP_DEVICE_DATA_SET_RANGE {
    pub StartingOffset: super::LONGLONG,
    pub LengthInBytes: super::ULONGLONG,
}
pub type MP_STORAGE_DIAGNOSTIC_LEVEL = i32;
pub type MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = i32;
pub const MpStorageDiagnosticLevelDefault: MP_STORAGE_DIAGNOSTIC_LEVEL = 0;
pub const MpStorageDiagnosticLevelMax: MP_STORAGE_DIAGNOSTIC_LEVEL = 1;
pub const MpStorageDiagnosticTargetTypeHbaFirmware: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 3;
pub const MpStorageDiagnosticTargetTypeMax: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 4;
pub const MpStorageDiagnosticTargetTypeMiniport: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 2;
pub const MpStorageDiagnosticTargetTypeUndefined: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 0;
pub const NRB_FUNCTION_ADD_LBAS_PINNED_SET: i32 = 16;
pub const NRB_FUNCTION_FLUSH_NVCACHE: i32 = 20;
pub const NRB_FUNCTION_NVCACHE_INFO: i32 = 236;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN: i32 = 1;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_SET: i32 = 0;
pub const NRB_FUNCTION_NVSEPARATED_FLUSH: i32 = 193;
pub const NRB_FUNCTION_NVSEPARATED_INFO: i32 = 192;
pub const NRB_FUNCTION_NVSEPARATED_WB_DISABLE: i32 = 194;
pub const NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT: i32 = 195;
pub const NRB_FUNCTION_PASS_HINT_PAYLOAD: i32 = 224;
pub const NRB_FUNCTION_QUERY_ASCENDER_STATUS: i32 = 208;
pub const NRB_FUNCTION_QUERY_CACHE_MISS: i32 = 19;
pub const NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS: i32 = 209;
pub const NRB_FUNCTION_QUERY_PINNED_SET: i32 = 18;
pub const NRB_FUNCTION_REMOVE_LBAS_PINNED_SET: i32 = 17;
pub const NRB_FUNCTION_SPINDLE_STATUS: i32 = 229;
pub const NRB_ILLEGAL_REQUEST: i32 = 1;
pub const NRB_INPUT_DATA_OVERRUN: i32 = 3;
pub const NRB_INPUT_DATA_UNDERRUN: i32 = 4;
pub const NRB_INVALID_PARAMETER: i32 = 2;
pub const NRB_OUTPUT_DATA_OVERRUN: i32 = 5;
pub const NRB_OUTPUT_DATA_UNDERRUN: i32 = 6;
pub const NRB_SUCCESS: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct NTSCSI_UNICODE_STRING {
    pub Length: super::USHORT,
    pub MaximumLength: super::USHORT,
    pub Buffer: super::PWCH,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVCACHE_HINT_PAYLOAD {
    pub Command: super::UCHAR,
    pub Feature7_0: super::UCHAR,
    pub Feature15_8: super::UCHAR,
    pub Count15_8: super::UCHAR,
    pub LBA7_0: super::UCHAR,
    pub LBA15_8: super::UCHAR,
    pub LBA23_16: super::UCHAR,
    pub LBA31_24: super::UCHAR,
    pub LBA39_32: super::UCHAR,
    pub LBA47_40: super::UCHAR,
    pub Auxiliary7_0: super::UCHAR,
    pub Auxiliary23_16: super::UCHAR,
    pub Reserved: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for NVCACHE_HINT_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    pub PriorityLevel: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
    pub ConsumedNVMSizeFraction: super::ULONG,
    pub ConsumedMappingResourcesFraction: super::ULONG,
    pub ConsumedNVMSizeForDirtyDataFraction: super::ULONG,
    pub ConsumedMappingResourcesForDirtyDataFraction: super::ULONG,
    pub Reserved1: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVCACHE_REQUEST_BLOCK {
    pub NRBSize: super::ULONG,
    pub Function: super::USHORT,
    pub NRBFlags: super::ULONG,
    pub NRBStatus: super::ULONG,
    pub Count: super::ULONG,
    pub LBA: super::ULONGLONG,
    pub DataBufSize: super::ULONG,
    pub NVCacheStatus: super::ULONG,
    pub NVCacheSubStatus: super::ULONG,
}
pub type NVCACHE_STATUS = i32;
pub type NVCACHE_TYPE = i32;
pub const NVSEPWriteCacheTypeNone: NV_SEP_WRITE_CACHE_TYPE = 1;
pub const NVSEPWriteCacheTypeUnknown: NV_SEP_WRITE_CACHE_TYPE = 0;
pub const NVSEPWriteCacheTypeWriteBack: NV_SEP_WRITE_CACHE_TYPE = 2;
pub const NVSEPWriteCacheTypeWriteThrough: NV_SEP_WRITE_CACHE_TYPE = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NV_FEATURE_PARAMETER {
    pub NVPowerModeEnabled: super::USHORT,
    pub NVParameterReserv1: super::USHORT,
    pub NVCmdEnabled: super::USHORT,
    pub NVParameterReserv2: super::USHORT,
    pub NVPowerModeVer: super::USHORT,
    pub NVCmdVer: super::USHORT,
    pub NVSize: super::ULONG,
    pub NVReadSpeed: super::USHORT,
    pub NVWrtSpeed: super::USHORT,
    pub DeviceSpinUpTime: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NV_SEP_CACHE_PARAMETER {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub Flags: NV_SEP_CACHE_PARAMETER_0,
    pub WriteCacheType: super::UCHAR,
    pub WriteCacheTypeEffective: super::UCHAR,
    pub ParameterReserve1: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl Default for NV_SEP_CACHE_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NV_SEP_CACHE_PARAMETER_0 {
    pub CacheFlags: NV_SEP_CACHE_PARAMETER_0_0,
    pub CacheFlagsSet: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NV_SEP_CACHE_PARAMETER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NV_SEP_CACHE_PARAMETER_0_0 {
    pub _bitfield: u8,
}
pub const NV_SEP_CACHE_PARAMETER_VERSION: i32 = 1;
pub const NV_SEP_CACHE_PARAMETER_VERSION_1: i32 = 1;
pub type NV_SEP_WRITE_CACHE_TYPE = i32;
pub const NvCacheStatusDisabled: NVCACHE_STATUS = 2;
pub const NvCacheStatusDisabling: NVCACHE_STATUS = 1;
pub const NvCacheStatusEnabled: NVCACHE_STATUS = 3;
pub const NvCacheStatusUnknown: NVCACHE_STATUS = 0;
pub const NvCacheTypeNone: NVCACHE_TYPE = 1;
pub const NvCacheTypeUnknown: NVCACHE_TYPE = 0;
pub const NvCacheTypeWriteBack: NVCACHE_TYPE = 2;
pub const NvCacheTypeWriteThrough: NVCACHE_TYPE = 3;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PATA_PASS_THROUGH_DIRECT = *mut ATA_PASS_THROUGH_DIRECT;
#[cfg(feature = "shared")]
pub type PATA_PASS_THROUGH_EX = *mut ATA_PASS_THROUGH_EX;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PATA_PASS_THROUGH_EX32 = *mut ATA_PASS_THROUGH_EX32;
#[cfg(feature = "shared")]
pub type PBOOT_PARTITION_REQUEST_BLOCK = *mut BOOT_PARTITION_REQUEST_BLOCK;
#[cfg(feature = "shared")]
pub type PDSM_NOTIFICATION_REQUEST_BLOCK = *mut DSM_NOTIFICATION_REQUEST_BLOCK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDUMP_DEVICE_POWERON_ROUTINE = Option<unsafe extern "C" fn(context: super::PVOID) -> super::LONG>;
#[cfg(feature = "winnt")]
pub type PDUMP_DRIVER = *mut DUMP_DRIVER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDUMP_DRIVER_EX = *mut DUMP_DRIVER_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDUMP_POINTERS = *mut DUMP_POINTERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDUMP_POINTERS_EX = *mut DUMP_POINTERS_EX;
#[cfg(feature = "shared")]
pub type PDUMP_POINTERS_VERSION = *mut DUMP_POINTERS_VERSION;
#[cfg(feature = "shared")]
pub type PFIRMWARE_REQUEST_BLOCK = *mut FIRMWARE_REQUEST_BLOCK;
#[cfg(feature = "shared")]
pub type PHYBRID_DEMOTE_BY_SIZE = *mut HYBRID_DEMOTE_BY_SIZE;
#[cfg(feature = "shared")]
pub type PHYBRID_DIRTY_THRESHOLDS = *mut HYBRID_DIRTY_THRESHOLDS;
#[cfg(feature = "shared")]
pub type PHYBRID_INFORMATION = *mut HYBRID_INFORMATION;
#[cfg(feature = "shared")]
pub type PHYBRID_REQUEST_BLOCK = *mut HYBRID_REQUEST_BLOCK;
#[cfg(feature = "shared")]
pub type PIDE_IO_CONTROL = *mut IDE_IO_CONTROL;
#[cfg(feature = "shared")]
pub type PIO_SCSI_CAPABILITIES = *mut IO_SCSI_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PMPIO_PASS_THROUGH_PATH = *mut MPIO_PASS_THROUGH_PATH;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PMPIO_PASS_THROUGH_PATH32 = *mut MPIO_PASS_THROUGH_PATH32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PMPIO_PASS_THROUGH_PATH32_EX = *mut MPIO_PASS_THROUGH_PATH32_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPIO_PASS_THROUGH_PATH_DIRECT = *mut MPIO_PASS_THROUGH_PATH_DIRECT;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PMPIO_PASS_THROUGH_PATH_DIRECT32_EX = *mut MPIO_PASS_THROUGH_PATH_DIRECT32_EX;
#[cfg(feature = "shared")]
pub type PMPIO_PASS_THROUGH_PATH_DIRECT_EX = *mut MPIO_PASS_THROUGH_PATH_DIRECT_EX;
#[cfg(feature = "shared")]
pub type PMPIO_PASS_THROUGH_PATH_EX = *mut MPIO_PASS_THROUGH_PATH_EX;
#[cfg(feature = "shared")]
pub type PMP_DEVICE_DATA_SET_RANGE = *mut MP_DEVICE_DATA_SET_RANGE;
pub type PMP_STORAGE_DIAGNOSTIC_LEVEL = *mut MP_STORAGE_DIAGNOSTIC_LEVEL;
pub type PMP_STORAGE_DIAGNOSTIC_TARGET_TYPE = *mut MP_STORAGE_DIAGNOSTIC_TARGET_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PNTSCSI_UNICODE_STRING = *mut NTSCSI_UNICODE_STRING;
#[cfg(feature = "shared")]
pub type PNVCACHE_HINT_PAYLOAD = *mut NVCACHE_HINT_PAYLOAD;
#[cfg(feature = "shared")]
pub type PNVCACHE_PRIORITY_LEVEL_DESCRIPTOR = *mut NVCACHE_PRIORITY_LEVEL_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PNVCACHE_REQUEST_BLOCK = *mut NVCACHE_REQUEST_BLOCK;
#[cfg(feature = "shared")]
pub type PNV_FEATURE_PARAMETER = *mut NV_FEATURE_PARAMETER;
#[cfg(feature = "shared")]
pub type PNV_SEP_CACHE_PARAMETER = *mut NV_SEP_CACHE_PARAMETER;
pub type PNV_SEP_WRITE_CACHE_TYPE = *mut NV_SEP_WRITE_CACHE_TYPE;
#[cfg(feature = "shared")]
pub type PSCSI_ADAPTER_BUS_INFO = *mut SCSI_ADAPTER_BUS_INFO;
#[cfg(feature = "shared")]
pub type PSCSI_ADDRESS = *mut SCSI_ADDRESS;
#[cfg(feature = "shared")]
pub type PSCSI_BUS_DATA = *mut SCSI_BUS_DATA;
#[cfg(feature = "shared")]
pub type PSCSI_INQUIRY_DATA = *mut SCSI_INQUIRY_DATA;
#[cfg(feature = "shared")]
pub type PSCSI_PASS_THROUGH = *mut SCSI_PASS_THROUGH;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PSCSI_PASS_THROUGH32 = *mut SCSI_PASS_THROUGH32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PSCSI_PASS_THROUGH32_EX = *mut SCSI_PASS_THROUGH32_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCSI_PASS_THROUGH_DIRECT = *mut SCSI_PASS_THROUGH_DIRECT;
#[cfg(feature = "shared")]
pub type PSCSI_PASS_THROUGH_DIRECT_EX = *mut SCSI_PASS_THROUGH_DIRECT_EX;
#[cfg(feature = "shared")]
pub type PSCSI_PASS_THROUGH_EX = *mut SCSI_PASS_THROUGH_EX;
#[cfg(feature = "shared")]
pub type PSRB_IO_CONTROL = *mut SRB_IO_CONTROL;
#[cfg(feature = "shared")]
pub type PSTORAGE_BOOT_PARTITION_ACTIVATE = *mut STORAGE_BOOT_PARTITION_ACTIVATE;
#[cfg(feature = "shared")]
pub type PSTORAGE_BOOT_PARTITION_DOWNLOAD = *mut STORAGE_BOOT_PARTITION_DOWNLOAD;
#[cfg(feature = "shared")]
pub type PSTORAGE_BOOT_PARTITION_INFO = *mut STORAGE_BOOT_PARTITION_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_DIAGNOSTIC_MP_REQUEST = *mut STORAGE_DIAGNOSTIC_MP_REQUEST;
#[cfg(feature = "shared")]
pub type PSTORAGE_ENDURANCE_DATA_DESCRIPTOR = *mut STORAGE_ENDURANCE_DATA_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_ENDURANCE_INFO = *mut STORAGE_ENDURANCE_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_FIRMWARE_ACTIVATE = *mut STORAGE_FIRMWARE_ACTIVATE;
#[cfg(feature = "shared")]
pub type PSTORAGE_FIRMWARE_DOWNLOAD = *mut STORAGE_FIRMWARE_DOWNLOAD;
#[cfg(feature = "shared")]
pub type PSTORAGE_FIRMWARE_DOWNLOAD_V2 = *mut STORAGE_FIRMWARE_DOWNLOAD_V2;
#[cfg(feature = "shared")]
pub type PSTORAGE_FIRMWARE_INFO = *mut STORAGE_FIRMWARE_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_FIRMWARE_INFO_V2 = *mut STORAGE_FIRMWARE_INFO_V2;
#[cfg(feature = "shared")]
pub type PSTORAGE_FIRMWARE_SLOT_INFO = *mut STORAGE_FIRMWARE_SLOT_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_FIRMWARE_SLOT_INFO_V2 = *mut STORAGE_FIRMWARE_SLOT_INFO_V2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SCSI_ADAPTER_BUS_INFO {
    pub NumberOfBuses: super::UCHAR,
    pub BusData: [SCSI_BUS_DATA; 1],
}
#[cfg(feature = "shared")]
impl Default for SCSI_ADAPTER_BUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SCSI_ADDRESS {
    pub Length: super::ULONG,
    pub PortNumber: super::UCHAR,
    pub PathId: super::UCHAR,
    pub TargetId: super::UCHAR,
    pub Lun: super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SCSI_BUS_DATA {
    pub NumberOfLogicalUnits: super::UCHAR,
    pub InitiatorBusId: super::UCHAR,
    pub InquiryDataOffset: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SCSI_INQUIRY_DATA {
    pub PathId: super::UCHAR,
    pub TargetId: super::UCHAR,
    pub Lun: super::UCHAR,
    pub DeviceClaimed: super::BOOLEAN,
    pub InquiryDataLength: super::ULONG,
    pub NextInquiryDataOffset: super::ULONG,
    pub InquiryData: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for SCSI_INQUIRY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCSI_IOCTL_DATA_BIDIRECTIONAL: i32 = 3;
pub const SCSI_IOCTL_DATA_IN: i32 = 1;
pub const SCSI_IOCTL_DATA_OUT: i32 = 0;
pub const SCSI_IOCTL_DATA_UNSPECIFIED: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH {
    pub Length: super::USHORT,
    pub ScsiStatus: super::UCHAR,
    pub PathId: super::UCHAR,
    pub TargetId: super::UCHAR,
    pub Lun: super::UCHAR,
    pub CdbLength: super::UCHAR,
    pub SenseInfoLength: super::UCHAR,
    pub DataIn: super::UCHAR,
    pub DataTransferLength: super::ULONG,
    pub TimeOutValue: super::ULONG,
    pub DataBufferOffset: super::ULONG_PTR,
    pub SenseInfoOffset: super::ULONG,
    pub Cdb: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for SCSI_PASS_THROUGH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH32 {
    pub Length: super::USHORT,
    pub ScsiStatus: super::UCHAR,
    pub PathId: super::UCHAR,
    pub TargetId: super::UCHAR,
    pub Lun: super::UCHAR,
    pub CdbLength: super::UCHAR,
    pub SenseInfoLength: super::UCHAR,
    pub DataIn: super::UCHAR,
    pub DataTransferLength: super::ULONG,
    pub TimeOutValue: super::ULONG,
    pub DataBufferOffset: super::ULONG32,
    pub SenseInfoOffset: super::ULONG,
    pub Cdb: [super::UCHAR; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for SCSI_PASS_THROUGH32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH32_EX {
    pub Version: super::ULONG,
    pub Length: super::ULONG,
    pub CdbLength: super::ULONG,
    pub StorAddressLength: super::ULONG,
    pub ScsiStatus: super::UCHAR,
    pub SenseInfoLength: super::UCHAR,
    pub DataDirection: super::UCHAR,
    pub Reserved: super::UCHAR,
    pub TimeOutValue: super::ULONG,
    pub StorAddressOffset: super::ULONG,
    pub SenseInfoOffset: super::ULONG,
    pub DataOutTransferLength: super::ULONG,
    pub DataInTransferLength: super::ULONG,
    pub DataOutBufferOffset: super::ULONG32,
    pub DataInBufferOffset: super::ULONG32,
    pub Cdb: [super::UCHAR; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for SCSI_PASS_THROUGH32_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH_DIRECT {
    pub Length: super::USHORT,
    pub ScsiStatus: super::UCHAR,
    pub PathId: super::UCHAR,
    pub TargetId: super::UCHAR,
    pub Lun: super::UCHAR,
    pub CdbLength: super::UCHAR,
    pub SenseInfoLength: super::UCHAR,
    pub DataIn: super::UCHAR,
    pub DataTransferLength: super::ULONG,
    pub TimeOutValue: super::ULONG,
    pub DataBuffer: super::PVOID,
    pub SenseInfoOffset: super::ULONG,
    pub Cdb: [super::UCHAR; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SCSI_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH_DIRECT_EX {
    pub Version: super::ULONG,
    pub Length: super::ULONG,
    pub CdbLength: super::ULONG,
    pub StorAddressLength: super::ULONG,
    pub ScsiStatus: super::UCHAR,
    pub SenseInfoLength: super::UCHAR,
    pub DataDirection: super::UCHAR,
    pub Reserved: super::UCHAR,
    pub TimeOutValue: super::ULONG,
    pub StorAddressOffset: super::ULONG,
    pub SenseInfoOffset: super::ULONG,
    pub DataOutTransferLength: super::ULONG,
    pub DataInTransferLength: super::ULONG,
    pub DataOutBuffer: *mut core::ffi::c_void,
    pub DataInBuffer: *mut core::ffi::c_void,
    pub Cdb: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for SCSI_PASS_THROUGH_DIRECT_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SCSI_PASS_THROUGH_EX {
    pub Version: super::ULONG,
    pub Length: super::ULONG,
    pub CdbLength: super::ULONG,
    pub StorAddressLength: super::ULONG,
    pub ScsiStatus: super::UCHAR,
    pub SenseInfoLength: super::UCHAR,
    pub DataDirection: super::UCHAR,
    pub Reserved: super::UCHAR,
    pub TimeOutValue: super::ULONG,
    pub StorAddressOffset: super::ULONG,
    pub SenseInfoOffset: super::ULONG,
    pub DataOutTransferLength: super::ULONG,
    pub DataInTransferLength: super::ULONG,
    pub DataOutBufferOffset: super::ULONG_PTR,
    pub DataInBufferOffset: super::ULONG_PTR,
    pub Cdb: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for SCSI_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SRB_IO_CONTROL {
    pub HeaderLength: super::ULONG,
    pub Signature: [super::UCHAR; 8],
    pub Timeout: super::ULONG,
    pub ControlCode: super::ULONG,
    pub ReturnCode: super::ULONG,
    pub Length: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for SRB_IO_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_BOOT_PARTITION_ACTIVATE {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub BPID: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_BOOT_PARTITION_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_BOOT_PARTITION_ACTIVATE_STRUCTURE_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_BOOT_PARTITION_DOWNLOAD {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub BPID: super::UCHAR,
    pub Offset: super::ULONGLONG,
    pub BufferSize: super::ULONGLONG,
    pub ImageBuffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_BOOT_PARTITION_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_BOOT_PARTITION_DOWNLOAD_STRUCTURE_VERSION: i32 = 1;
pub const STORAGE_BOOT_PARTITION_GET_INFO_STRUCTURE_VERSION_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct STORAGE_BOOT_PARTITION_INFO {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub BPSZ: super::ULONGLONG,
    pub ImagePayloadAlignment: super::ULONG,
    pub ImagePayloadMaxSize: super::ULONG,
    pub SlotCount: super::UCHAR,
    pub ABPID: super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_DIAGNOSTIC_MP_REQUEST {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub TargetType: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: MP_STORAGE_DIAGNOSTIC_LEVEL,
    pub ProviderId: windows_sys::core::GUID,
    pub BufferSize: super::ULONG,
    pub Reserved: super::ULONG,
    pub DataBuffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_DIAGNOSTIC_STATUS_BUFFER_TOO_SMALL: i32 = 1;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_PARAMETER: i32 = 3;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_SIGNATURE: i32 = 4;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_TARGET_TYPE: i32 = 5;
pub const STORAGE_DIAGNOSTIC_STATUS_MORE_DATA: i32 = 6;
pub const STORAGE_DIAGNOSTIC_STATUS_SUCCESS: i32 = 0;
pub const STORAGE_DIAGNOSTIC_STATUS_UNSUPPORTED_VERSION: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub EnduranceInfo: STORAGE_ENDURANCE_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_ENDURANCE_INFO {
    pub ValidFields: super::ULONG,
    pub GroupId: super::ULONG,
    pub Flags: STORAGE_ENDURANCE_INFO_0,
    pub LifePercentage: super::ULONG,
    pub BytesReadCount: [super::UCHAR; 16],
    pub ByteWriteCount: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct STORAGE_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_ACTIVATE {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub SlotToActivate: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_DOWNLOAD {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub Offset: super::ULONGLONG,
    pub BufferSize: super::ULONGLONG,
    pub ImageBuffer: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION: i32 = 1;
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_DOWNLOAD_V2 {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub Offset: super::ULONGLONG,
    pub BufferSize: super::ULONGLONG,
    pub Slot: super::UCHAR,
    pub Reserved: [super::UCHAR; 3],
    pub ImageSize: super::ULONG,
    pub ImageBuffer: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_INFO {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub UpgradeSupport: super::BOOLEAN,
    pub SlotCount: super::UCHAR,
    pub ActiveSlot: super::UCHAR,
    pub PendingActivateSlot: super::UCHAR,
    pub Reserved: super::ULONG,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO; 0],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_FIRMWARE_INFO_INVALID_SLOT: i32 = 255;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION: i32 = 1;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_INFO_V2 {
    pub Version: super::ULONG,
    pub Size: super::ULONG,
    pub UpgradeSupport: super::BOOLEAN,
    pub SlotCount: super::UCHAR,
    pub ActiveSlot: super::UCHAR,
    pub PendingActivateSlot: super::UCHAR,
    pub FirmwareShared: super::BOOLEAN,
    pub Reserved: [super::UCHAR; 3],
    pub ImagePayloadAlignment: super::ULONG,
    pub ImagePayloadMaxSize: super::ULONG,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO_V2; 0],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FIRMWARE_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_SLOT_INFO {
    pub SlotNumber: super::UCHAR,
    pub ReadOnly: super::BOOLEAN,
    pub Reserved: [super::UCHAR; 6],
    pub Revision: STORAGE_FIRMWARE_SLOT_INFO_0,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_FIRMWARE_SLOT_INFO_0 {
    pub Info: [super::UCHAR; 8],
    pub AsUlonglong: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FIRMWARE_SLOT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_FIRMWARE_SLOT_INFO_V2 {
    pub SlotNumber: super::UCHAR,
    pub ReadOnly: super::BOOLEAN,
    pub Reserved: [super::UCHAR; 6],
    pub Revision: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH: i32 = 16;
pub const ScsiRawInterfaceGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x53f56309_b6bf_11d0_94f2_00a0c91efb8b);
pub const WmiScsiAddressGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x53f5630f_b6bf_11d0_94f2_00a0c91efb8b);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _ADAPTER_OBJECT(pub u8);
