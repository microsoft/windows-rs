pub const ABL_5_WO: STORAGE_MEDIA_TYPE = 64;
pub const ADR_1: STORAGE_MEDIA_TYPE = 90;
pub const ADR_2: STORAGE_MEDIA_TYPE = 91;
pub const AIT1_8mm: STORAGE_MEDIA_TYPE = 38;
pub const AIT_8mm: STORAGE_MEDIA_TYPE = 89;
pub const AME_8mm: STORAGE_MEDIA_TYPE = 37;
pub const ASSERT_ALTERNATE: i32 = 9;
pub const ASSERT_PRIMARY: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ASYNC_DUPLICATE_EXTENTS_STATUS {
    pub Version: super::DWORD,
    pub State: DUPLICATE_EXTENTS_STATE,
    pub SourceFileOffset: super::DWORDLONG,
    pub TargetFileOffset: super::DWORDLONG,
    pub ByteCount: super::DWORDLONG,
    pub BytesDuplicated: super::DWORDLONG,
}
#[cfg(target_arch = "x86")]
pub const ASYNC_DUPLICATE_EXTENTS_STATUS_V1: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ASYNC_DUPLICATE_EXTENTS_STATUS_V1: u64 = 40;
pub const ATAPI_ID_CMD: i32 = 161;
pub const AVATAR_F2: STORAGE_MEDIA_TYPE = 78;
pub const AllElements: ELEMENT_TYPE = 0;
pub const AtaDataTypeIdentify: STORAGE_PROTOCOL_ATA_DATA_TYPE = 1;
pub const AtaDataTypeLogPage: STORAGE_PROTOCOL_ATA_DATA_TYPE = 2;
pub const AtaDataTypeUnknown: STORAGE_PROTOCOL_ATA_DATA_TYPE = 0;
pub type BAD_TRACK_NUMBER = u16;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BIN_COUNT {
    pub BinRange: BIN_RANGE,
    pub BinCount: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for BIN_COUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BIN_RANGE {
    pub StartValue: super::LARGE_INTEGER,
    pub Length: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for BIN_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BIN_RESULTS {
    pub NumberOfBins: super::DWORD,
    pub BinCounts: [BIN_COUNT; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for BIN_RESULTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BIN_TYPES = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BOOT_AREA_INFO {
    pub BootSectorCount: super::DWORD,
    pub BootSectors: [BOOT_AREA_INFO_0; 2],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for BOOT_AREA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BOOT_AREA_INFO_0 {
    pub Offset: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for BOOT_AREA_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BULK_SECURITY_TEST_DATA {
    pub DesiredAccess: super::ACCESS_MASK,
    pub SecurityIds: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for BULK_SECURITY_TEST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BusType1394: STORAGE_BUS_TYPE = 4;
pub const BusTypeAta: STORAGE_BUS_TYPE = 3;
pub const BusTypeAtapi: STORAGE_BUS_TYPE = 2;
pub const BusTypeFibre: STORAGE_BUS_TYPE = 6;
pub const BusTypeFileBackedVirtual: STORAGE_BUS_TYPE = 15;
pub const BusTypeMax: STORAGE_BUS_TYPE = 21;
pub const BusTypeMaxReserved: STORAGE_BUS_TYPE = 127;
pub const BusTypeMmc: STORAGE_BUS_TYPE = 13;
pub const BusTypeNvme: STORAGE_BUS_TYPE = 17;
pub const BusTypeNvmeof: STORAGE_BUS_TYPE = 20;
pub const BusTypeRAID: STORAGE_BUS_TYPE = 8;
pub const BusTypeSCM: STORAGE_BUS_TYPE = 18;
pub const BusTypeSas: STORAGE_BUS_TYPE = 10;
pub const BusTypeSata: STORAGE_BUS_TYPE = 11;
pub const BusTypeScsi: STORAGE_BUS_TYPE = 1;
pub const BusTypeSd: STORAGE_BUS_TYPE = 12;
pub const BusTypeSpaces: STORAGE_BUS_TYPE = 16;
pub const BusTypeSsa: STORAGE_BUS_TYPE = 5;
pub const BusTypeUfs: STORAGE_BUS_TYPE = 19;
pub const BusTypeUnknown: STORAGE_BUS_TYPE = 0;
pub const BusTypeUsb: STORAGE_BUS_TYPE = 7;
pub const BusTypeVirtual: STORAGE_BUS_TYPE = 14;
pub const BusTypeiScsi: STORAGE_BUS_TYPE = 9;
pub const CAP_ATAPI_ID_CMD: i32 = 2;
pub const CAP_ATA_ID_CMD: i32 = 1;
pub const CAP_SMART_CMD: i32 = 4;
pub const CDB_SIZE: i32 = 16;
pub const CD_R: STORAGE_MEDIA_TYPE = 52;
pub const CD_ROM: STORAGE_MEDIA_TYPE = 51;
pub const CD_RW: STORAGE_MEDIA_TYPE = 53;
pub const CHANGER_BAR_CODE_SCANNER_INSTALLED: i32 = 1;
pub const CHANGER_CARTRIDGE_MAGAZINE: i32 = 256;
pub const CHANGER_CLEANER_ACCESS_NOT_VALID: i32 = 262144;
pub const CHANGER_CLEANER_AUTODISMOUNT: u32 = 2147483652;
pub const CHANGER_CLEANER_OPS_NOT_SUPPORTED: u32 = 2147483712;
pub const CHANGER_CLEANER_SLOT: i32 = 64;
pub const CHANGER_CLOSE_IEPORT: i32 = 4;
pub type CHANGER_DEVICE_PROBLEM_TYPE = i32;
pub const CHANGER_DEVICE_REINITIALIZE_CAPABLE: i32 = 134217728;
pub const CHANGER_DRIVE_CLEANING_REQUIRED: i32 = 65536;
pub const CHANGER_DRIVE_EMPTY_ON_DOOR_ACCESS: i32 = 536870912;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_ELEMENT {
    pub ElementType: ELEMENT_TYPE,
    pub ElementAddress: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_ELEMENT_LIST {
    pub Element: CHANGER_ELEMENT,
    pub NumberOfElements: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGER_ELEMENT_STATUS {
    pub Element: CHANGER_ELEMENT,
    pub SrcElementAddress: CHANGER_ELEMENT,
    pub Flags: super::DWORD,
    pub ExceptionCode: super::DWORD,
    pub TargetId: super::BYTE,
    pub Lun: super::BYTE,
    pub Reserved: super::WORD,
    pub PrimaryVolumeID: [super::BYTE; 36],
    pub AlternateVolumeID: [super::BYTE; 36],
}
#[cfg(feature = "shared")]
impl Default for CHANGER_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGER_ELEMENT_STATUS_EX {
    pub Element: CHANGER_ELEMENT,
    pub SrcElementAddress: CHANGER_ELEMENT,
    pub Flags: super::DWORD,
    pub ExceptionCode: super::DWORD,
    pub TargetId: super::BYTE,
    pub Lun: super::BYTE,
    pub Reserved: super::WORD,
    pub PrimaryVolumeID: [super::BYTE; 36],
    pub AlternateVolumeID: [super::BYTE; 36],
    pub VendorIdentification: [super::BYTE; 8],
    pub ProductIdentification: [super::BYTE; 16],
    pub SerialNumber: [super::BYTE; 32],
}
#[cfg(feature = "shared")]
impl Default for CHANGER_ELEMENT_STATUS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CHANGER_EXCHANGE_MEDIA: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_EXCHANGE_MEDIUM {
    pub Transport: CHANGER_ELEMENT,
    pub Source: CHANGER_ELEMENT,
    pub Destination1: CHANGER_ELEMENT,
    pub Destination2: CHANGER_ELEMENT,
    pub Flip1: super::BOOLEAN,
    pub Flip2: super::BOOLEAN,
}
pub const CHANGER_IEPORT_USER_CONTROL_CLOSE: u32 = 2147483904;
pub const CHANGER_IEPORT_USER_CONTROL_OPEN: u32 = 2147483776;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_INITIALIZE_ELEMENT_STATUS {
    pub ElementList: CHANGER_ELEMENT_LIST,
    pub BarCodeScan: super::BOOLEAN,
}
pub const CHANGER_INIT_ELEM_STAT_WITH_RANGE: i32 = 2;
pub const CHANGER_KEYPAD_ENABLE_DISABLE: i32 = 268435456;
pub const CHANGER_LOCK_UNLOCK: i32 = 128;
pub const CHANGER_MEDIUM_FLIP: i32 = 512;
pub const CHANGER_MOVE_EXTENDS_IEPORT: u32 = 2147484160;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_MOVE_MEDIUM {
    pub Transport: CHANGER_ELEMENT,
    pub Source: CHANGER_ELEMENT,
    pub Destination: CHANGER_ELEMENT,
    pub Flip: super::BOOLEAN,
}
pub const CHANGER_MOVE_RETRACTS_IEPORT: u32 = 2147484672;
pub const CHANGER_OPEN_IEPORT: i32 = 8;
pub const CHANGER_POSITION_TO_ELEMENT: i32 = 1024;
pub const CHANGER_PREDISMOUNT_ALIGN_TO_DRIVE: u32 = 2147483650;
pub const CHANGER_PREDISMOUNT_ALIGN_TO_SLOT: u32 = 2147483649;
pub const CHANGER_PREDISMOUNT_EJECT_REQUIRED: i32 = 131072;
pub const CHANGER_PREMOUNT_EJECT_REQUIRED: i32 = 524288;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGER_PRODUCT_DATA {
    pub VendorId: [super::BYTE; 8],
    pub ProductId: [super::BYTE; 16],
    pub Revision: [super::BYTE; 4],
    pub SerialNumber: [super::BYTE; 32],
    pub DeviceType: super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for CHANGER_PRODUCT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_READ_ELEMENT_STATUS {
    pub ElementList: CHANGER_ELEMENT_LIST,
    pub VolumeTagInfo: super::BOOLEAN,
}
pub const CHANGER_REPORT_IEPORT_STATE: i32 = 2048;
pub const CHANGER_RESERVED_BIT: u32 = 2147483648;
pub const CHANGER_RTN_MEDIA_TO_ORIGINAL_ADDR: u32 = 2147483680;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGER_SEND_VOLUME_TAG_INFORMATION {
    pub StartingElement: CHANGER_ELEMENT,
    pub ActionCode: super::DWORD,
    pub VolumeIDTemplate: [super::BYTE; 40],
}
#[cfg(feature = "shared")]
impl Default for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CHANGER_SERIAL_NUMBER_VALID: i32 = 67108864;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_SET_ACCESS {
    pub Element: CHANGER_ELEMENT,
    pub Control: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGER_SET_POSITION {
    pub Transport: CHANGER_ELEMENT,
    pub Destination: CHANGER_ELEMENT,
    pub Flip: super::BOOLEAN,
}
pub const CHANGER_SLOTS_USE_TRAYS: u32 = 2147483664;
pub const CHANGER_STATUS_NON_VOLATILE: i32 = 16;
pub const CHANGER_STORAGE_DRIVE: i32 = 4096;
pub const CHANGER_STORAGE_IEPORT: i32 = 8192;
pub const CHANGER_STORAGE_SLOT: i32 = 16384;
pub const CHANGER_STORAGE_TRANSPORT: i32 = 32768;
pub const CHANGER_TO_DRIVE: i32 = 8;
pub const CHANGER_TO_IEPORT: i32 = 4;
pub const CHANGER_TO_SLOT: i32 = 2;
pub const CHANGER_TO_TRANSPORT: i32 = 1;
pub const CHANGER_TRUE_EXCHANGE_CAPABLE: u32 = 2147483656;
pub const CHANGER_VOLUME_ASSERT: i32 = 4194304;
pub const CHANGER_VOLUME_IDENTIFICATION: i32 = 1048576;
pub const CHANGER_VOLUME_REPLACE: i32 = 8388608;
pub const CHANGER_VOLUME_SEARCH: i32 = 2097152;
pub const CHANGER_VOLUME_UNDEFINE: i32 = 16777216;
pub const CHECKSUM_TYPE_CRC32: i32 = 1;
pub const CHECKSUM_TYPE_CRC64: i32 = 2;
pub const CHECKSUM_TYPE_ECC: i32 = 3;
pub const CHECKSUM_TYPE_FIRST_UNUSED_TYPE: i32 = 6;
pub const CHECKSUM_TYPE_NONE: i32 = 0;
pub const CHECKSUM_TYPE_SHA256: i32 = 4;
#[cfg(feature = "shared")]
pub const CHECKSUM_TYPE_UNCHANGED: super::WORD = 65535;
pub const CHECKSUM_TYPE_XXH64: i32 = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLASS_MEDIA_CHANGE_CONTEXT {
    pub MediaChangeCount: super::DWORD,
    pub NewState: super::DWORD,
}
pub const CLEANER_CARTRIDGE: STORAGE_MEDIA_TYPE = 50;
pub type CLSN = u64;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CLUSTER_RANGE {
    pub StartingCluster: super::LARGE_INTEGER,
    pub ClusterCount: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CLUSTER_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONTAINER_ROOT_INFO_FLAG_BIND_DO_NOT_MAP_NAME: i32 = 256;
pub const CONTAINER_ROOT_INFO_FLAG_BIND_EXCEPTION_ROOT: i32 = 128;
pub const CONTAINER_ROOT_INFO_FLAG_BIND_ROOT: i32 = 32;
pub const CONTAINER_ROOT_INFO_FLAG_BIND_TARGET_ROOT: i32 = 64;
pub const CONTAINER_ROOT_INFO_FLAG_LAYER_ROOT: i32 = 2;
pub const CONTAINER_ROOT_INFO_FLAG_SCRATCH_ROOT: i32 = 1;
pub const CONTAINER_ROOT_INFO_FLAG_UNION_LAYER_ROOT: i32 = 512;
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_EXCEPTION_ROOT: i32 = 16;
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_ROOT: i32 = 4;
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_TARGET_ROOT: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTAINER_ROOT_INFO_INPUT {
    pub Flags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONTAINER_ROOT_INFO_OUTPUT {
    pub ContainerRootIdLength: super::WORD,
    pub ContainerRootId: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for CONTAINER_ROOT_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONTAINER_ROOT_INFO_VALID_FLAGS: i32 = 1023;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTAINER_VOLUME_STATE {
    pub Flags: super::DWORD,
}
pub const CONTAINER_VOLUME_STATE_HOSTING_CONTAINER: i32 = 1;
pub const COPYFILE_SIS_FLAGS: i32 = 3;
pub const COPYFILE_SIS_LINK: i32 = 1;
pub const COPYFILE_SIS_REPLACE: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CREATE_DISK {
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: CREATE_DISK_0,
}
#[cfg(feature = "shared")]
impl Default for CREATE_DISK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union CREATE_DISK_0 {
    pub Mbr: CREATE_DISK_MBR,
    pub Gpt: CREATE_DISK_GPT,
}
#[cfg(feature = "shared")]
impl Default for CREATE_DISK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_DISK_GPT {
    pub DiskId: windows_core::GUID,
    pub MaxPartitionCount: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_DISK_MBR {
    pub Signature: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_USN_JOURNAL_DATA {
    pub MaximumSize: super::DWORDLONG,
    pub AllocationDelta: super::DWORDLONG,
}
pub type CSVFS_DISK_CONNECTIVITY = i32;
pub type CSV_CONTROL_OP = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_CONTROL_PARAM {
    pub Operation: CSV_CONTROL_OP,
    pub Unused: super::LONGLONG,
}
pub const CSV_INVALID_DEVICE_NUMBER: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_IS_OWNED_BY_CSVFS {
    pub OwnedByCSVFS: super::BOOLEAN,
}
pub const CSV_MGMTLOCK_CHECK_VOLUME_REDIRECTED: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_MGMT_LOCK {
    pub Flags: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CSV_NAMESPACE_INFO {
    pub Version: super::DWORD,
    pub DeviceNumber: super::DWORD,
    pub StartingOffset: super::LARGE_INTEGER,
    pub SectorSize: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CSV_NAMESPACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const CSV_NAMESPACE_INFO_V1: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CSV_NAMESPACE_INFO_V1: u64 = 24;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION {
    pub FileId: super::LONGLONG,
    pub FileRevision: [super::LONGLONG; 3],
}
#[cfg(feature = "shared")]
impl Default for CSV_QUERY_FILE_REVISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    pub FileId: super::FILE_ID_128,
    pub FileRevision: [super::LONGLONG; 3],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_QUERY_MDS_PATH {
    pub MdsNodeId: super::DWORD,
    pub DsNodeId: super::DWORD,
    pub PathLength: super::DWORD,
    pub Path: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CSV_QUERY_MDS_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSV_QUERY_MDS_PATH_FLAG_CSV_DIRECT_IO_ENABLED: i32 = 2;
pub const CSV_QUERY_MDS_PATH_FLAG_SMB_BYPASS_CSV_ENABLED: i32 = 4;
pub const CSV_QUERY_MDS_PATH_FLAG_STORAGE_ON_THIS_NODE_IS_CONNECTED: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_QUERY_MDS_PATH_V2 {
    pub Version: super::LONGLONG,
    pub RequiredSize: super::DWORD,
    pub MdsNodeId: super::DWORD,
    pub DsNodeId: super::DWORD,
    pub Flags: super::DWORD,
    pub DiskConnectivity: CSVFS_DISK_CONNECTIVITY,
    pub VolumeId: windows_core::GUID,
    pub IpAddressOffset: super::DWORD,
    pub IpAddressLength: super::DWORD,
    pub PathOffset: super::DWORD,
    pub PathLength: super::DWORD,
}
pub const CSV_QUERY_MDS_PATH_V2_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_QUERY_REDIRECT_STATE {
    pub MdsNodeId: super::DWORD,
    pub DsNodeId: super::DWORD,
    pub FileRedirected: super::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    pub VetoedFromAltitudeIntegral: super::DWORDLONG,
    pub VetoedFromAltitudeDecimal: super::DWORDLONG,
    pub Reason: [super::WCHAR; 256],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_QUERY_VOLUME_ID {
    pub VolumeId: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_QUERY_VOLUME_REDIRECT_STATE {
    pub MdsNodeId: super::DWORD,
    pub DsNodeId: super::DWORD,
    pub IsDiskConnected: super::BOOLEAN,
    pub ClusterEnableDirectIo: super::BOOLEAN,
    pub DiskConnectivity: CSVFS_DISK_CONNECTIVITY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSV_SET_VOLUME_ID {
    pub VolumeId: windows_core::GUID,
}
pub const CYGNET_12_WO: STORAGE_MEDIA_TYPE = 69;
pub const ChangerDoor: ELEMENT_TYPE = 5;
pub const ChangerDrive: ELEMENT_TYPE = 4;
pub const ChangerIEPort: ELEMENT_TYPE = 3;
pub const ChangerKeypad: ELEMENT_TYPE = 6;
pub const ChangerMaxElement: ELEMENT_TYPE = 7;
pub const ChangerSlot: ELEMENT_TYPE = 2;
pub const ChangerTransport: ELEMENT_TYPE = 1;
pub const CsvControlDisableCaching: CSV_CONTROL_OP = 19;
pub const CsvControlEnableCaching: CSV_CONTROL_OP = 20;
pub const CsvControlEnableUSNRangeModificationTracking: CSV_CONTROL_OP = 13;
pub const CsvControlGetCsvFsMdsPathV2: CSV_CONTROL_OP = 18;
pub const CsvControlMarkHandleLocalVolumeMount: CSV_CONTROL_OP = 14;
pub const CsvControlQueryFileRevision: CSV_CONTROL_OP = 6;
pub const CsvControlQueryFileRevisionFileId128: CSV_CONTROL_OP = 9;
pub const CsvControlQueryMdsPath: CSV_CONTROL_OP = 8;
pub const CsvControlQueryMdsPathNoPause: CSV_CONTROL_OP = 23;
pub const CsvControlQueryRedirectState: CSV_CONTROL_OP = 4;
pub const CsvControlQueryVolumeId: CSV_CONTROL_OP = 25;
pub const CsvControlQueryVolumeRedirectState: CSV_CONTROL_OP = 10;
pub const CsvControlSetVolumeId: CSV_CONTROL_OP = 24;
pub const CsvControlStartForceDFO: CSV_CONTROL_OP = 21;
pub const CsvControlStartRedirectFile: CSV_CONTROL_OP = 2;
pub const CsvControlStopForceDFO: CSV_CONTROL_OP = 22;
pub const CsvControlStopRedirectFile: CSV_CONTROL_OP = 3;
pub const CsvControlUnmarkHandleLocalVolumeMount: CSV_CONTROL_OP = 15;
pub const CsvFsDiskConnectivityAllNodes: CSVFS_DISK_CONNECTIVITY = 3;
pub const CsvFsDiskConnectivityMdsNodeOnly: CSVFS_DISK_CONNECTIVITY = 1;
pub const CsvFsDiskConnectivityNone: CSVFS_DISK_CONNECTIVITY = 0;
pub const CsvFsDiskConnectivitySubsetOfNodes: CSVFS_DISK_CONNECTIVITY = 2;
pub const DAX_ALLOC_ALIGNMENT_FLAG_FALLBACK_SPECIFIED: i32 = 2;
pub const DAX_ALLOC_ALIGNMENT_FLAG_MANDATORY: i32 = 1;
pub const DDS_4mm: STORAGE_MEDIA_TYPE = 32;
pub const DDUMP_FLAG_DATA_READ_FROM_DEVICE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECRYPTION_STATUS_BUFFER {
    pub NoEncryptedStreams: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DELETE_USN_JOURNAL_DATA {
    pub UsnJournalID: super::DWORDLONG,
    pub DeleteFlags: super::DWORD,
}
pub type DETECTION_TYPE = i32;
pub const DEVICEDUMP_CAP_PRIVATE_SECTION: i32 = 1;
pub const DEVICEDUMP_CAP_RESTRICTED_SECTION: i32 = 2;
pub type DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE = i32;
pub const DEVICEDUMP_MAX_IDSTRING: i32 = 32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_PRIVATE_SUBSECTION {
    pub dwFlags: super::DWORD,
    pub GPLogId: GP_LOG_PAGE_DESCRIPTOR,
    pub bData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICEDUMP_PRIVATE_SUBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_PUBLIC_SUBSECTION {
    pub dwFlags: super::DWORD,
    pub GPLogTable: [GP_LOG_PAGE_DESCRIPTOR; 16],
    pub szDescription: [super::CHAR; 16],
    pub bData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICEDUMP_PUBLIC_SUBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_RESTRICTED_SUBSECTION {
    pub bData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_SECTION_HEADER {
    pub guidDeviceDataId: windows_core::GUID,
    pub sOrganizationID: [super::BYTE; 16],
    pub dwFirmwareRevision: super::DWORD,
    pub sModelNumber: [super::BYTE; 32],
    pub szDeviceManufacturingID: [super::BYTE; 32],
    pub dwFlags: super::DWORD,
    pub bRestrictedPrivateDataVersion: super::DWORD,
    pub dwFirmwareIssueId: super::DWORD,
    pub szIssueDescriptionString: [super::BYTE; 132],
}
#[cfg(feature = "shared")]
impl Default for DEVICEDUMP_SECTION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STORAGEDEVICE_DATA {
    pub Descriptor: DEVICEDUMP_STRUCTURE_VERSION,
    pub SectionHeader: DEVICEDUMP_SECTION_HEADER,
    pub dwBufferSize: super::DWORD,
    pub dwReasonForCollection: super::DWORD,
    pub PublicData: DEVICEDUMP_SUBSECTION_POINTER,
    pub RestrictedData: DEVICEDUMP_SUBSECTION_POINTER,
    pub PrivateData: DEVICEDUMP_SUBSECTION_POINTER,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    pub Descriptor: DEVICEDUMP_STRUCTURE_VERSION,
    pub dwReasonForCollection: super::DWORD,
    pub cDriverName: [super::BYTE; 16],
    pub uiNumRecords: super::DWORD,
    pub RecordArray: [DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    pub Cdb: [super::BYTE; 16],
    pub Command: [super::BYTE; 16],
    pub StartTime: super::DWORDLONG,
    pub EndTime: super::DWORDLONG,
    pub OperationStatus: super::DWORD,
    pub OperationError: super::DWORD,
    pub StackSpecific: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0,
}
#[cfg(feature = "shared")]
impl Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    pub ExternalStack: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0,
    pub AtaPort: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1,
    pub StorPort: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2,
}
#[cfg(feature = "shared")]
impl Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {
    pub dwReserved: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {
    pub dwAtaPortSpecific: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {
    pub SrbTag: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_STRUCTURE_VERSION {
    pub dwSignature: super::DWORD,
    pub dwVersion: super::DWORD,
    pub dwSize: super::DWORD,
}
pub const DEVICEDUMP_STRUCTURE_VERSION_V1: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVICEDUMP_SUBSECTION_POINTER {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwOffset: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub MaximumTokenLifetime: super::DWORD,
    pub DefaultTokenLifetime: super::DWORD,
    pub MaximumTransferSize: super::DWORDLONG,
    pub OptimalTransferCount: super::DWORDLONG,
    pub MaximumDataDescriptors: super::DWORD,
    pub MaximumTransferLengthPerDescriptor: super::DWORD,
    pub OptimalTransferLengthPerDescriptor: super::DWORD,
    pub OptimalTransferLengthGranularity: super::WORD,
    pub Reserved: [super::BYTE; 2],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DEVICE_DATA_MANAGEMENT_SET_ACTION = u32;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_LBP_STATE_PARAMETERS = _DEVICE_DATA_SET_LBP_STATE_PARAMETERS;
pub const DEVICE_DATA_SET_LBP_STATE_PARAMETERS_VERSION_V1: i32 = 1;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_LB_PROVISIONING_STATE = _DEVICE_DATA_SET_LB_PROVISIONING_STATE;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 = _DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2;
#[cfg(target_arch = "x86")]
pub const DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V1: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V1: u64 = 32;
#[cfg(target_arch = "x86")]
pub const DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V2: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V2: u64 = 40;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_RANGE = _DEVICE_DATA_SET_RANGE;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_REPAIR_OUTPUT = _DEVICE_DATA_SET_REPAIR_OUTPUT;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_REPAIR_PARAMETERS = _DEVICE_DATA_SET_REPAIR_PARAMETERS;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_SCRUB_EX_OUTPUT = _DEVICE_DATA_SET_SCRUB_EX_OUTPUT;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_SCRUB_OUTPUT = _DEVICE_DATA_SET_SCRUB_OUTPUT;
#[cfg(feature = "shared")]
pub type DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT = _DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT;
pub type DEVICE_DSM_ACTION = u32;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_ALLOCATION_OUTPUT = _DEVICE_DATA_SET_LB_PROVISIONING_STATE;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_ALLOCATION_OUTPUT2 = _DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2;
#[cfg(target_arch = "x86")]
pub const DEVICE_DSM_ALLOCATION_OUTPUT_V1: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DEVICE_DSM_ALLOCATION_OUTPUT_V1: u64 = 32;
#[cfg(target_arch = "x86")]
pub const DEVICE_DSM_ALLOCATION_OUTPUT_V2: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DEVICE_DSM_ALLOCATION_OUTPUT_V2: u64 = 40;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_ALLOCATION_PARAMETERS = _DEVICE_DATA_SET_LBP_STATE_PARAMETERS;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_DSM_CONVERSION_OUTPUT {
    pub Version: super::DWORD,
    pub Source: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_DSM_DEFINITION {
    pub Action: DEVICE_DSM_ACTION,
    pub SingleRange: super::BOOLEAN,
    pub ParameterBlockAlignment: super::DWORD,
    pub ParameterBlockLength: super::DWORD,
    pub HasOutput: super::BOOLEAN,
    pub OutputBlockAlignment: super::DWORD,
    pub OutputBlockLength: super::DWORD,
}
pub const DEVICE_DSM_FLAG_ALLOCATION_CONSOLIDATEABLE_ONLY: i32 = 1073741824;
pub const DEVICE_DSM_FLAG_ENTIRE_DATA_SET_RANGE: i32 = 1;
pub const DEVICE_DSM_FLAG_PHYSICAL_ADDRESSES_OMIT_TOTAL_RANGES: i32 = 268435456;
pub const DEVICE_DSM_FLAG_REPAIR_INPUT_TOPOLOGY_ID_PRESENT: i32 = 1073741824;
pub const DEVICE_DSM_FLAG_REPAIR_OUTPUT_PARITY_EXTENT: i32 = 536870912;
pub const DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT: i32 = 536870912;
pub const DEVICE_DSM_FLAG_SCRUB_SKIP_IN_SYNC: i32 = 268435456;
pub const DEVICE_DSM_FLAG_TRIM_BYPASS_RZAT: i32 = 1073741824;
pub const DEVICE_DSM_FLAG_TRIM_NOT_FS_ALLOCATED: u32 = 2147483648;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_DSM_FREE_SPACE_OUTPUT {
    pub Version: super::DWORD,
    pub FreeSpace: super::DWORDLONG,
}
#[cfg(feature = "shared")]
pub type DEVICE_DSM_INPUT = _DEVICE_MANAGE_DATA_SET_ATTRIBUTES;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_DSM_LOST_QUERY_OUTPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Alignment: super::DWORDLONG,
    pub NumberOfBits: super::DWORD,
    pub BitMap: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_DSM_LOST_QUERY_PARAMETERS {
    pub Version: super::DWORD,
    pub Granularity: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_DSM_NOTIFICATION_PARAMETERS {
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub NumFileTypeIDs: super::DWORD,
    pub FileTypeID: [windows_core::GUID; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_DSM_NOTIFY_FLAG_BEGIN: i32 = 1;
pub const DEVICE_DSM_NOTIFY_FLAG_END: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    pub Size: super::DWORD,
    pub TargetPriority: super::BYTE,
    pub Reserved: [super::BYTE; 3],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    pub Flags: super::DWORD,
    pub TimeToLive: super::DWORD,
    pub Reserved: [super::DWORD; 2],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    pub Flags: super::DWORD,
    pub Reserved: super::DWORD,
    pub TokenOffset: super::DWORDLONG,
    pub Token: STORAGE_OFFLOAD_TOKEN,
}
#[cfg(feature = "shared")]
impl Default for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type DEVICE_DSM_OUTPUT = _DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT;
pub const DEVICE_DSM_PARAMETERS_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub TotalNumberOfRanges: super::DWORD,
    pub NumberOfRangesReturned: super::DWORD,
    pub Ranges: [DEVICE_STORAGE_ADDRESS_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_V1: i32 = 1;
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_VERSION_V1: i32 = 1;
#[cfg(feature = "shared")]
pub const DEVICE_DSM_PHYSICAL_ADDRESS_HAS_MEMORY_ERROR: super::LONGLONG = -1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_DSM_QUERY_PREFER_LOCAL_REPAIR_OUTPUT {
    pub Version: super::DWORD,
    pub PreferLocalRepair: super::BOOLEAN,
}
#[cfg(feature = "shared")]
pub type DEVICE_DSM_RANGE = _DEVICE_DATA_SET_RANGE;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_RANGE_ERROR_INFO = _DEVICE_DSM_RANGE_ERROR_INFO;
pub const DEVICE_DSM_RANGE_ERROR_INFO_VERSION_V1: i32 = 1;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_RANGE_ERROR_OUTPUT = _DEVICE_DSM_RANGE_ERROR_INFO;
pub const DEVICE_DSM_RANGE_ERROR_OUTPUT_V1: i32 = 1;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_REPAIR_OUTPUT = _DEVICE_DATA_SET_REPAIR_OUTPUT;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_REPAIR_PARAMETERS = _DEVICE_DATA_SET_REPAIR_PARAMETERS;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_REPORT_ZONES_DATA = _DEVICE_DSM_REPORT_ZONES_DATA;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_REPORT_ZONES_OUTPUT = _DEVICE_DSM_REPORT_ZONES_DATA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    pub Size: super::DWORD,
    pub ReportOption: super::BYTE,
    pub Partial: super::BYTE,
    pub Reserved: [super::BYTE; 2],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type DEVICE_DSM_SCRUB_OUTPUT = _DEVICE_DATA_SET_SCRUB_OUTPUT;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_SCRUB_OUTPUT2 = _DEVICE_DATA_SET_SCRUB_EX_OUTPUT;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_TIERING_QUERY_INPUT = _DEVICE_DSM_TIERING_QUERY_INPUT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_DSM_TIERING_QUERY_OUTPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub Reserved: super::DWORD,
    pub Alignment: super::DWORDLONG,
    pub TotalNumberOfRegions: super::DWORD,
    pub NumberOfRegionsReturned: super::DWORD,
    pub Regions: [STORAGE_TIER_REGION; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type DEVICE_DSM_TIERING_QUERY_PARAMETERS = _DEVICE_DSM_TIERING_QUERY_INPUT;
#[cfg(feature = "shared")]
pub type DEVICE_DSM_TOPOLOGY_ID_QUERY_OUTPUT = _DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_INTERNAL_STATUS_DATA {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub T10VendorId: super::DWORDLONG,
    pub DataSet1Length: super::DWORD,
    pub DataSet2Length: super::DWORD,
    pub DataSet3Length: super::DWORD,
    pub DataSet4Length: super::DWORD,
    pub StatusDataVersion: super::BYTE,
    pub Reserved: [super::BYTE; 3],
    pub ReasonIdentifier: [super::BYTE; 128],
    pub StatusDataLength: super::DWORD,
    pub StatusData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVICE_INTERNAL_STATUS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = i32;
pub type DEVICE_INTERNAL_STATUS_DATA_SET = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_LB_PROVISIONING_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub _bitfield: u8,
    pub Reserved1: [super::BYTE; 7],
    pub OptimalUnmapGranularity: super::DWORDLONG,
    pub UnmapGranularityAlignment: super::DWORDLONG,
    pub MaxUnmapLbaCount: super::DWORD,
    pub MaxUnmapBlockDescriptorCount: super::DWORD,
}
#[cfg(feature = "shared")]
impl DEVICE_LB_PROVISIONING_DESCRIPTOR {
    pub fn ThinProvisioningEnabled(&self) -> super::BYTE {
        (self._bitfield) & 1
    }
    pub fn set_ThinProvisioningEnabled(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ThinProvisioningReadZeros(&self) -> super::BYTE {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ThinProvisioningReadZeros(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn AnchorSupported(&self) -> super::BYTE {
        (self._bitfield >> 2) & 7
    }
    pub fn set_AnchorSupported(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(7 << 2)) | ((value & 7) << 2);
    }
    pub fn UnmapGranularityAlignmentValid(&self) -> super::BYTE {
        (self._bitfield >> 5) & 1
    }
    pub fn set_UnmapGranularityAlignmentValid(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn GetFreeSpaceSupported(&self) -> super::BYTE {
        (self._bitfield >> 6) & 1
    }
    pub fn set_GetFreeSpaceSupported(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn MapSupported(&self) -> super::BYTE {
        self._bitfield >> 7
    }
    pub fn set_MapSupported(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[cfg(feature = "shared")]
impl Default for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const DEVICE_LB_PROVISIONING_DESCRIPTOR_V1_SIZE: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DEVICE_LB_PROVISIONING_DESCRIPTOR_V1_SIZE: u64 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICE_LOCATION {
    pub Socket: super::DWORD,
    pub Slot: super::DWORD,
    pub Adapter: super::DWORD,
    pub Port: super::DWORD,
    pub Anonymous: DEVICE_LOCATION_0,
}
#[cfg(feature = "shared")]
impl Default for DEVICE_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DEVICE_LOCATION_0 {
    pub Anonymous: DEVICE_LOCATION_0_0,
    pub Anonymous2: DEVICE_LOCATION_0_1,
}
#[cfg(feature = "shared")]
impl Default for DEVICE_LOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_LOCATION_0_0 {
    pub Channel: super::DWORD,
    pub Device: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_LOCATION_0_1 {
    pub Target: super::DWORD,
    pub Lun: super::DWORD,
}
#[cfg(feature = "shared")]
pub type DEVICE_MANAGE_DATA_SET_ATTRIBUTES = _DEVICE_MANAGE_DATA_SET_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT = _DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEVICE_MEDIA_INFO {
    pub DeviceSpecific: DEVICE_MEDIA_INFO_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEVICE_MEDIA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DEVICE_MEDIA_INFO_0 {
    pub DiskInfo: DEVICE_MEDIA_INFO_0_0,
    pub RemovableDiskInfo: DEVICE_MEDIA_INFO_0_1,
    pub TapeInfo: DEVICE_MEDIA_INFO_0_2,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEVICE_MEDIA_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEVICE_MEDIA_INFO_0_0 {
    pub Cylinders: super::LARGE_INTEGER,
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub TracksPerCylinder: super::DWORD,
    pub SectorsPerTrack: super::DWORD,
    pub BytesPerSector: super::DWORD,
    pub NumberMediaSides: super::DWORD,
    pub MediaCharacteristics: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEVICE_MEDIA_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEVICE_MEDIA_INFO_0_1 {
    pub Cylinders: super::LARGE_INTEGER,
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub TracksPerCylinder: super::DWORD,
    pub SectorsPerTrack: super::DWORD,
    pub BytesPerSector: super::DWORD,
    pub NumberMediaSides: super::DWORD,
    pub MediaCharacteristics: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEVICE_MEDIA_INFO_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEVICE_MEDIA_INFO_0_2 {
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub MediaCharacteristics: super::DWORD,
    pub CurrentBlockSize: super::DWORD,
    pub BusType: STORAGE_BUS_TYPE,
    pub BusSpecificData: DEVICE_MEDIA_INFO_0_2_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEVICE_MEDIA_INFO_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DEVICE_MEDIA_INFO_0_2_0 {
    pub ScsiInformation: DEVICE_MEDIA_INFO_0_2_0_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEVICE_MEDIA_INFO_0_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_MEDIA_INFO_0_2_0_0 {
    pub MediumType: super::BYTE,
    pub DensityCode: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DEVICE_POWER_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceAttentionSupported: super::BOOLEAN,
    pub AsynchronousNotificationSupported: super::BOOLEAN,
    pub IdlePowerManagementEnabled: super::BOOLEAN,
    pub D3ColdEnabled: super::BOOLEAN,
    pub D3ColdSupported: super::BOOLEAN,
    pub NoVerifyDuringIdlePower: super::BOOLEAN,
    pub Reserved: [super::BYTE; 2],
    pub IdleTimeoutInMS: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for DEVICE_POWER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_SEEK_PENALTY_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub IncursSeekPenalty: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_STORAGE_ADDRESS_RANGE {
    pub StartAddress: super::LONGLONG,
    pub LengthInBytes: super::DWORDLONG,
}
pub const DEVICE_STORAGE_NO_ERRORS: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVICE_STORAGE_RANGE_ATTRIBUTES {
    pub LengthInBytes: super::DWORDLONG,
    pub Anonymous: DEVICE_STORAGE_RANGE_ATTRIBUTES_0,
    pub Reserved: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for DEVICE_STORAGE_RANGE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    pub AllFlags: super::DWORD,
    pub Anonymous: DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0,
}
#[cfg(feature = "shared")]
impl Default for DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    pub fn IsRangeBad(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_IsRangeBad(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_TRIM_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub TrimEnabled: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub BenefitsFromWriteAggregation: super::BOOLEAN,
}
pub const DISABLE_SMART: i32 = 217;
pub const DISK_ATTRIBUTE_OFFLINE: i32 = 1;
pub const DISK_ATTRIBUTE_READ_ONLY: i32 = 2;
pub const DISK_BINNING: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISK_CACHE_INFORMATION {
    pub ParametersSavable: super::BOOLEAN,
    pub ReadCacheEnabled: super::BOOLEAN,
    pub WriteCacheEnabled: super::BOOLEAN,
    pub ReadRetentionPriority: DISK_CACHE_RETENTION_PRIORITY,
    pub WriteRetentionPriority: DISK_CACHE_RETENTION_PRIORITY,
    pub DisablePrefetchTransferLength: super::WORD,
    pub PrefetchScalar: super::BOOLEAN,
    pub Anonymous: DISK_CACHE_INFORMATION_0,
}
#[cfg(feature = "shared")]
impl Default for DISK_CACHE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DISK_CACHE_INFORMATION_0 {
    pub ScalarPrefetch: DISK_CACHE_INFORMATION_0_0,
    pub BlockPrefetch: DISK_CACHE_INFORMATION_0_1,
}
#[cfg(feature = "shared")]
impl Default for DISK_CACHE_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_CACHE_INFORMATION_0_0 {
    pub Minimum: super::WORD,
    pub Maximum: super::WORD,
    pub MaximumBlocks: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_CACHE_INFORMATION_0_1 {
    pub Minimum: super::WORD,
    pub Maximum: super::WORD,
}
pub type DISK_CACHE_RETENTION_PRIORITY = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_CONTROLLER_NUMBER {
    pub ControllerNumber: super::DWORD,
    pub DiskNumber: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISK_DETECTION_INFO {
    pub SizeOfDetectInfo: super::DWORD,
    pub DetectionType: DETECTION_TYPE,
    pub Anonymous: DISK_DETECTION_INFO_0,
}
#[cfg(feature = "shared")]
impl Default for DISK_DETECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DISK_DETECTION_INFO_0 {
    pub Anonymous: DISK_DETECTION_INFO_0_0,
}
#[cfg(feature = "shared")]
impl Default for DISK_DETECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_DETECTION_INFO_0_0 {
    pub Int13: DISK_INT13_INFO,
    pub ExInt13: DISK_EX_INT13_INFO,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISK_EXTENT {
    pub DiskNumber: super::DWORD,
    pub StartingOffset: super::LARGE_INTEGER,
    pub ExtentLength: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISK_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_EX_INT13_INFO {
    pub ExBufferSize: super::WORD,
    pub ExFlags: super::WORD,
    pub ExCylinders: super::DWORD,
    pub ExHeads: super::DWORD,
    pub ExSectorsPerTrack: super::DWORD,
    pub ExSectorsPerDrive: super::DWORD64,
    pub ExSectorSize: super::WORD,
    pub ExReserved: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISK_GEOMETRY {
    pub Cylinders: super::LARGE_INTEGER,
    pub MediaType: MEDIA_TYPE,
    pub TracksPerCylinder: super::DWORD,
    pub SectorsPerTrack: super::DWORD,
    pub BytesPerSector: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISK_GEOMETRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISK_GEOMETRY_EX {
    pub Geometry: DISK_GEOMETRY,
    pub DiskSize: super::LARGE_INTEGER,
    pub Data: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISK_GEOMETRY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISK_GROW_PARTITION {
    pub PartitionNumber: super::DWORD,
    pub BytesToGrow: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISK_GROW_PARTITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISK_HISTOGRAM {
    pub DiskSize: super::LARGE_INTEGER,
    pub Start: super::LARGE_INTEGER,
    pub End: super::LARGE_INTEGER,
    pub Average: super::LARGE_INTEGER,
    pub AverageRead: super::LARGE_INTEGER,
    pub AverageWrite: super::LARGE_INTEGER,
    pub Granularity: super::DWORD,
    pub Size: super::DWORD,
    pub ReadCount: super::DWORD,
    pub WriteCount: super::DWORD,
    pub Histogram: PHISTOGRAM_BUCKET,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISK_HISTOGRAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const DISK_HISTOGRAM_SIZE: u32 = 72;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DISK_HISTOGRAM_SIZE: u64 = 72;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_INT13_INFO {
    pub DriveSelect: super::WORD,
    pub MaxCylinders: super::DWORD,
    pub SectorsPerTrack: super::WORD,
    pub MaxHeads: super::WORD,
    pub NumberDrives: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_LOGGING {
    pub Function: super::BYTE,
    pub BufferAddress: super::PVOID,
    pub BufferSize: super::DWORD,
}
pub const DISK_LOGGING_DUMP: i32 = 2;
pub const DISK_LOGGING_START: i32 = 0;
pub const DISK_LOGGING_STOP: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISK_PARTITION_INFO {
    pub SizeOfPartitionInfo: super::DWORD,
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: DISK_PARTITION_INFO_0,
}
#[cfg(feature = "shared")]
impl Default for DISK_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DISK_PARTITION_INFO_0 {
    pub Mbr: DISK_PARTITION_INFO_0_0,
    pub Gpt: DISK_PARTITION_INFO_0_1,
}
#[cfg(feature = "shared")]
impl Default for DISK_PARTITION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_PARTITION_INFO_0_0 {
    pub Signature: super::DWORD,
    pub CheckSum: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISK_PARTITION_INFO_0_1 {
    pub DiskId: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISK_PERFORMANCE {
    pub BytesRead: super::LARGE_INTEGER,
    pub BytesWritten: super::LARGE_INTEGER,
    pub ReadTime: super::LARGE_INTEGER,
    pub WriteTime: super::LARGE_INTEGER,
    pub IdleTime: super::LARGE_INTEGER,
    pub ReadCount: super::DWORD,
    pub WriteCount: super::DWORD,
    pub QueueDepth: super::DWORD,
    pub SplitCount: super::DWORD,
    pub QueryTime: super::LARGE_INTEGER,
    pub StorageDeviceNumber: super::DWORD,
    pub StorageManagerName: [super::WCHAR; 8],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISK_PERFORMANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISK_RECORD {
    pub ByteOffset: super::LARGE_INTEGER,
    pub StartTime: super::LARGE_INTEGER,
    pub EndTime: super::LARGE_INTEGER,
    pub VirtualAddress: super::PVOID,
    pub NumberOfBytes: super::DWORD,
    pub DeviceNumber: super::BYTE,
    pub ReadRequest: super::BOOLEAN,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISK_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DLT: STORAGE_MEDIA_TYPE = 39;
pub const DMI: STORAGE_MEDIA_TYPE = 48;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DRIVERSTATUS {
    pub bDriverError: super::BYTE,
    pub bIDEError: super::BYTE,
    pub bReserved: [super::BYTE; 2],
    pub dwReserved: [super::DWORD; 2],
}
#[cfg(feature = "shared")]
impl Default for DRIVERSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DRIVE_LAYOUT_INFORMATION {
    pub PartitionCount: super::DWORD,
    pub Signature: super::DWORD,
    pub PartitionEntry: [PARTITION_INFORMATION; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DRIVE_LAYOUT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DRIVE_LAYOUT_INFORMATION_EX {
    pub PartitionStyle: super::DWORD,
    pub PartitionCount: super::DWORD,
    pub Anonymous: DRIVE_LAYOUT_INFORMATION_EX_0,
    pub PartitionEntry: [PARTITION_INFORMATION_EX; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DRIVE_LAYOUT_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DRIVE_LAYOUT_INFORMATION_EX_0 {
    pub Mbr: DRIVE_LAYOUT_INFORMATION_MBR,
    pub Gpt: DRIVE_LAYOUT_INFORMATION_GPT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DRIVE_LAYOUT_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DRIVE_LAYOUT_INFORMATION_GPT {
    pub DiskId: windows_core::GUID,
    pub StartingUsableOffset: super::LARGE_INTEGER,
    pub UsableLength: super::LARGE_INTEGER,
    pub MaxPartitionCount: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DRIVE_LAYOUT_INFORMATION_GPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRIVE_LAYOUT_INFORMATION_MBR {
    pub Signature: super::DWORD,
    pub CheckSum: super::DWORD,
}
pub const DST_L: STORAGE_MEDIA_TYPE = 82;
pub const DST_M: STORAGE_MEDIA_TYPE = 81;
pub const DST_S: STORAGE_MEDIA_TYPE = 80;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DUPLICATE_EXTENTS_DATA {
    pub FileHandle: super::HANDLE,
    pub SourceFileOffset: super::LARGE_INTEGER,
    pub TargetFileOffset: super::LARGE_INTEGER,
    pub ByteCount: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DUPLICATE_EXTENTS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DUPLICATE_EXTENTS_DATA32 {
    pub FileHandle: super::UINT32,
    pub SourceFileOffset: super::LARGE_INTEGER,
    pub TargetFileOffset: super::LARGE_INTEGER,
    pub ByteCount: super::LARGE_INTEGER,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DUPLICATE_EXTENTS_DATA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DUPLICATE_EXTENTS_DATA_EX {
    pub Size: super::SIZE_T,
    pub FileHandle: super::HANDLE,
    pub SourceFileOffset: super::LARGE_INTEGER,
    pub TargetFileOffset: super::LARGE_INTEGER,
    pub ByteCount: super::LARGE_INTEGER,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DUPLICATE_EXTENTS_DATA_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DUPLICATE_EXTENTS_DATA_EX32 {
    pub Size: super::DWORD32,
    pub FileHandle: super::DWORD32,
    pub SourceFileOffset: super::LARGE_INTEGER,
    pub TargetFileOffset: super::LARGE_INTEGER,
    pub ByteCount: super::LARGE_INTEGER,
    pub Flags: super::DWORD,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
impl Default for DUPLICATE_EXTENTS_DATA_EX32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DUPLICATE_EXTENTS_DATA_EX_ASYNC: i32 = 2;
pub const DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC: i32 = 1;
pub type DUPLICATE_EXTENTS_STATE = i32;
pub const DVD_R: STORAGE_MEDIA_TYPE = 55;
pub const DVD_RAM: STORAGE_MEDIA_TYPE = 88;
pub const DVD_ROM: STORAGE_MEDIA_TYPE = 54;
pub const DVD_RW: STORAGE_MEDIA_TYPE = 56;
pub const DV_6mm: STORAGE_MEDIA_TYPE = 47;
pub const DetectExInt13: DETECTION_TYPE = 2;
pub const DetectInt13: DETECTION_TYPE = 1;
pub const DetectNone: DETECTION_TYPE = 0;
pub const DeviceCurrentInternalStatusData: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 2;
pub const DeviceCurrentInternalStatusDataHeader: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 1;
pub const DeviceDsmActionFlag_NonDestructive: u32 = 2147483648;
pub const DeviceDsmAction_Allocation: u32 = 2147483653;
pub const DeviceDsmAction_CloseZone: u32 = 2147483670;
pub const DeviceDsmAction_ConversionQuery: u32 = 2147483676;
pub const DeviceDsmAction_DrtClear: u32 = 2147483657;
pub const DeviceDsmAction_DrtDisable: u32 = 2147483658;
pub const DeviceDsmAction_DrtQuery: u32 = 2147483656;
pub const DeviceDsmAction_FinishZone: u32 = 2147483669;
pub const DeviceDsmAction_GetFreeSpace: u32 = 2147483675;
pub const DeviceDsmAction_GetPhysicalAddresses: u32 = 2147483665;
pub const DeviceDsmAction_GetRangeErrorInfo: u32 = 2147483672;
pub const DeviceDsmAction_LostQuery: u32 = 2147483674;
pub const DeviceDsmAction_Map: u32 = 2147483660;
pub const DeviceDsmAction_None: u32 = 0;
pub const DeviceDsmAction_Notification: u32 = 2147483650;
pub const DeviceDsmAction_NvCache_Change_Priority: u32 = 2147483662;
pub const DeviceDsmAction_NvCache_Evict: u32 = 2147483663;
pub const DeviceDsmAction_OffloadRead: u32 = 2147483651;
pub const DeviceDsmAction_OffloadWrite: u32 = 4;
pub const DeviceDsmAction_OpenZone: u32 = 2147483668;
pub const DeviceDsmAction_QueryPreferLocalRepair: u32 = 2147483678;
pub const DeviceDsmAction_RegenerateParity: u32 = 2147483661;
pub const DeviceDsmAction_Repair: u32 = 2147483654;
pub const DeviceDsmAction_ReportZones: u32 = 2147483667;
pub const DeviceDsmAction_ResetWritePointer: u32 = 23;
pub const DeviceDsmAction_ScopeRegen: u32 = 2147483666;
pub const DeviceDsmAction_Scrub: u32 = 2147483655;
pub const DeviceDsmAction_TieringQuery: u32 = 2147483659;
pub const DeviceDsmAction_TopologyIdQuery: u32 = 2147483664;
pub const DeviceDsmAction_Trim: u32 = 1;
pub const DeviceDsmAction_VdtSet: u32 = 29;
pub const DeviceDsmAction_WriteZeroes: u32 = 25;
pub const DeviceInternalStatusDataRequestTypeUndefined: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 0;
pub const DeviceProblemCHMError: CHANGER_DEVICE_PROBLEM_TYPE = 2;
pub const DeviceProblemCHMMoveError: CHANGER_DEVICE_PROBLEM_TYPE = 6;
pub const DeviceProblemCHMZeroError: CHANGER_DEVICE_PROBLEM_TYPE = 7;
pub const DeviceProblemCalibrationError: CHANGER_DEVICE_PROBLEM_TYPE = 4;
pub const DeviceProblemCartridgeEjectError: CHANGER_DEVICE_PROBLEM_TYPE = 11;
pub const DeviceProblemCartridgeInsertError: CHANGER_DEVICE_PROBLEM_TYPE = 8;
pub const DeviceProblemDoorOpen: CHANGER_DEVICE_PROBLEM_TYPE = 3;
pub const DeviceProblemDriveError: CHANGER_DEVICE_PROBLEM_TYPE = 13;
pub const DeviceProblemGripperError: CHANGER_DEVICE_PROBLEM_TYPE = 12;
pub const DeviceProblemHardware: CHANGER_DEVICE_PROBLEM_TYPE = 1;
pub const DeviceProblemNone: CHANGER_DEVICE_PROBLEM_TYPE = 0;
pub const DeviceProblemPositionError: CHANGER_DEVICE_PROBLEM_TYPE = 9;
pub const DeviceProblemSensorError: CHANGER_DEVICE_PROBLEM_TYPE = 10;
pub const DeviceProblemTargetFailure: CHANGER_DEVICE_PROBLEM_TYPE = 5;
pub const DeviceSavedInternalStatusData: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 4;
pub const DeviceSavedInternalStatusDataHeader: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 3;
pub const DeviceStatusDataSet1: DEVICE_INTERNAL_STATUS_DATA_SET = 1;
pub const DeviceStatusDataSet2: DEVICE_INTERNAL_STATUS_DATA_SET = 2;
pub const DeviceStatusDataSet3: DEVICE_INTERNAL_STATUS_DATA_SET = 3;
pub const DeviceStatusDataSet4: DEVICE_INTERNAL_STATUS_DATA_SET = 4;
pub const DeviceStatusDataSetMax: DEVICE_INTERNAL_STATUS_DATA_SET = 5;
pub const DeviceStatusDataSetUndefined: DEVICE_INTERNAL_STATUS_DATA_SET = 0;
pub const DiskHealthHealthy: STORAGE_DISK_HEALTH_STATUS = 3;
pub const DiskHealthMax: STORAGE_DISK_HEALTH_STATUS = 4;
pub const DiskHealthUnhealthy: STORAGE_DISK_HEALTH_STATUS = 1;
pub const DiskHealthUnknown: STORAGE_DISK_HEALTH_STATUS = 0;
pub const DiskHealthWarning: STORAGE_DISK_HEALTH_STATUS = 2;
pub const DiskOpReasonBackgroundOperation: STORAGE_OPERATIONAL_STATUS_REASON = 12;
pub const DiskOpReasonComponent: STORAGE_OPERATIONAL_STATUS_REASON = 10;
pub const DiskOpReasonConfiguration: STORAGE_OPERATIONAL_STATUS_REASON = 7;
pub const DiskOpReasonDataPersistenceLossImminent: STORAGE_OPERATIONAL_STATUS_REASON = 18;
pub const DiskOpReasonDeviceController: STORAGE_OPERATIONAL_STATUS_REASON = 8;
pub const DiskOpReasonDisabledByPlatform: STORAGE_OPERATIONAL_STATUS_REASON = 16;
pub const DiskOpReasonEnergySource: STORAGE_OPERATIONAL_STATUS_REASON = 6;
pub const DiskOpReasonHealthCheck: STORAGE_OPERATIONAL_STATUS_REASON = 14;
pub const DiskOpReasonInvalidFirmware: STORAGE_OPERATIONAL_STATUS_REASON = 13;
pub const DiskOpReasonIo: STORAGE_OPERATIONAL_STATUS_REASON = 3;
pub const DiskOpReasonLostData: STORAGE_OPERATIONAL_STATUS_REASON = 5;
pub const DiskOpReasonLostDataPersistence: STORAGE_OPERATIONAL_STATUS_REASON = 15;
pub const DiskOpReasonLostWritePersistence: STORAGE_OPERATIONAL_STATUS_REASON = 17;
pub const DiskOpReasonMax: STORAGE_OPERATIONAL_STATUS_REASON = 20;
pub const DiskOpReasonMedia: STORAGE_OPERATIONAL_STATUS_REASON = 2;
pub const DiskOpReasonMediaController: STORAGE_OPERATIONAL_STATUS_REASON = 9;
pub const DiskOpReasonNVDIMM_N: STORAGE_OPERATIONAL_STATUS_REASON = 11;
pub const DiskOpReasonScsiSenseCode: STORAGE_OPERATIONAL_STATUS_REASON = 1;
pub const DiskOpReasonThresholdExceeded: STORAGE_OPERATIONAL_STATUS_REASON = 4;
pub const DiskOpReasonUnknown: STORAGE_OPERATIONAL_STATUS_REASON = 0;
pub const DiskOpReasonWritePersistenceLossImminent: STORAGE_OPERATIONAL_STATUS_REASON = 19;
pub const DiskOpStatusHardwareError: STORAGE_DISK_OPERATIONAL_STATUS = 5;
pub const DiskOpStatusInService: STORAGE_DISK_OPERATIONAL_STATUS = 4;
pub const DiskOpStatusMissing: STORAGE_DISK_OPERATIONAL_STATUS = 8;
pub const DiskOpStatusNone: STORAGE_DISK_OPERATIONAL_STATUS = 0;
pub const DiskOpStatusNotUsable: STORAGE_DISK_OPERATIONAL_STATUS = 6;
pub const DiskOpStatusOk: STORAGE_DISK_OPERATIONAL_STATUS = 2;
pub const DiskOpStatusPredictingFailure: STORAGE_DISK_OPERATIONAL_STATUS = 3;
pub const DiskOpStatusTransientError: STORAGE_DISK_OPERATIONAL_STATUS = 7;
pub const DiskOpStatusUnknown: STORAGE_DISK_OPERATIONAL_STATUS = 1;
pub const EFS_TRACKED_OFFSET_HEADER_FLAG: i32 = 1;
pub const ELEMENT_STATUS_ACCESS: i32 = 8;
pub const ELEMENT_STATUS_AVOLTAG: i32 = 536870912;
pub const ELEMENT_STATUS_EXCEPT: i32 = 4;
pub const ELEMENT_STATUS_EXENAB: i32 = 16;
pub const ELEMENT_STATUS_FULL: i32 = 1;
pub const ELEMENT_STATUS_ID_VALID: i32 = 8192;
pub const ELEMENT_STATUS_IMPEXP: i32 = 2;
pub const ELEMENT_STATUS_INENAB: i32 = 32;
pub const ELEMENT_STATUS_INVERT: i32 = 4194304;
pub const ELEMENT_STATUS_LUN_VALID: i32 = 4096;
pub const ELEMENT_STATUS_NOT_BUS: i32 = 32768;
pub const ELEMENT_STATUS_PRODUCT_DATA: i32 = 64;
pub const ELEMENT_STATUS_PVOLTAG: i32 = 268435456;
pub const ELEMENT_STATUS_SVALID: i32 = 8388608;
pub type ELEMENT_TYPE = i32;
pub const ENABLE_DISABLE_AUTOSAVE: i32 = 210;
pub const ENABLE_DISABLE_AUTO_OFFLINE: i32 = 219;
pub const ENABLE_SMART: i32 = 216;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTED_DATA_INFO {
    pub StartingFileOffset: super::DWORDLONG,
    pub OutputBufferOffset: super::DWORD,
    pub BytesWithinFileSize: super::DWORD,
    pub BytesWithinValidDataLength: super::DWORD,
    pub CompressionFormat: super::WORD,
    pub DataUnitShift: super::BYTE,
    pub ChunkShift: super::BYTE,
    pub ClusterShift: super::BYTE,
    pub EncryptionFormat: super::BYTE,
    pub NumberOfDataBlocks: super::WORD,
    pub DataBlockSize: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for ENCRYPTED_DATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENCRYPTED_DATA_INFO_4K_SPARSE_UNIT: i32 = 4;
pub const ENCRYPTED_DATA_INFO_SPARSE_DATA: i32 = 2;
pub const ENCRYPTED_DATA_INFO_SPARSE_FILE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENCRYPTION_BUFFER {
    pub EncryptionOperation: super::DWORD,
    pub Private: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for ENCRYPTION_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENCRYPTION_FORMAT_DEFAULT: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCRYPTION_KEY_CTRL_INPUT {
    pub HeaderSize: super::DWORD,
    pub StructureSize: super::DWORD,
    pub KeyOffset: super::WORD,
    pub KeySize: super::WORD,
    pub DplLock: super::DWORD,
    pub DplUserId: super::DWORDLONG,
    pub DplCredentialId: super::DWORDLONG,
}
pub const ERROR_DRIVE_NOT_INSTALLED: i32 = 8;
pub const ERROR_HISTORY_DIRECTORY_ENTRY_DEFAULT_COUNT: i32 = 8;
pub const ERROR_INIT_STATUS_NEEDED: i32 = 17;
pub const ERROR_LABEL_QUESTIONABLE: i32 = 2;
pub const ERROR_LABEL_UNREADABLE: i32 = 1;
pub const ERROR_SLOT_NOT_PRESENT: i32 = 4;
pub const ERROR_TRAY_MALFUNCTION: i32 = 16;
pub const ERROR_UNHANDLED_ERROR: u32 = 4294967295;
pub const EXECUTE_OFFLINE_DIAGS: i32 = 212;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXFAT_STATISTICS {
    pub CreateHits: super::DWORD,
    pub SuccessfulCreates: super::DWORD,
    pub FailedCreates: super::DWORD,
    pub NonCachedReads: super::DWORD,
    pub NonCachedReadBytes: super::DWORD,
    pub NonCachedWrites: super::DWORD,
    pub NonCachedWriteBytes: super::DWORD,
    pub NonCachedDiskReads: super::DWORD,
    pub NonCachedDiskWrites: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXTENDED_ENCRYPTED_DATA_INFO {
    pub ExtendedCode: super::DWORD,
    pub Length: super::DWORD,
    pub Flags: super::DWORD,
    pub Reserved: super::DWORD,
}
pub const EXTEND_IEPORT: i32 = 2;
pub const EqualPriority: DISK_CACHE_RETENTION_PRIORITY = 0;
pub const F3_120M_512: MEDIA_TYPE = 13;
pub const F3_128Mb_512: MEDIA_TYPE = 20;
pub const F3_1Pt23_1024: MEDIA_TYPE = 18;
pub const F3_1Pt2_512: MEDIA_TYPE = 17;
pub const F3_1Pt44_512: MEDIA_TYPE = 2;
pub const F3_200Mb_512: MEDIA_TYPE = 23;
pub const F3_20Pt8_512: MEDIA_TYPE = 4;
pub const F3_230Mb_512: MEDIA_TYPE = 21;
pub const F3_240M_512: MEDIA_TYPE = 24;
pub const F3_2Pt88_512: MEDIA_TYPE = 3;
pub const F3_32M_512: MEDIA_TYPE = 25;
pub const F3_640_512: MEDIA_TYPE = 14;
pub const F3_720_512: MEDIA_TYPE = 5;
pub const F5_160_512: MEDIA_TYPE = 10;
pub const F5_180_512: MEDIA_TYPE = 9;
pub const F5_1Pt23_1024: MEDIA_TYPE = 19;
pub const F5_1Pt2_512: MEDIA_TYPE = 1;
pub const F5_320_1024: MEDIA_TYPE = 8;
pub const F5_320_512: MEDIA_TYPE = 7;
pub const F5_360_512: MEDIA_TYPE = 6;
pub const F5_640_512: MEDIA_TYPE = 15;
pub const F5_720_512: MEDIA_TYPE = 16;
pub const F8_256_128: MEDIA_TYPE = 22;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAT_STATISTICS {
    pub CreateHits: super::DWORD,
    pub SuccessfulCreates: super::DWORD,
    pub FailedCreates: super::DWORD,
    pub NonCachedReads: super::DWORD,
    pub NonCachedReadBytes: super::DWORD,
    pub NonCachedWrites: super::DWORD,
    pub NonCachedWriteBytes: super::DWORD,
    pub NonCachedDiskReads: super::DWORD,
    pub NonCachedDiskWrites: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILESYSTEM_STATISTICS {
    pub FileSystemType: super::WORD,
    pub Version: super::WORD,
    pub SizeOfCompleteStructure: super::DWORD,
    pub UserFileReads: super::DWORD,
    pub UserFileReadBytes: super::DWORD,
    pub UserDiskReads: super::DWORD,
    pub UserFileWrites: super::DWORD,
    pub UserFileWriteBytes: super::DWORD,
    pub UserDiskWrites: super::DWORD,
    pub MetaDataReads: super::DWORD,
    pub MetaDataReadBytes: super::DWORD,
    pub MetaDataDiskReads: super::DWORD,
    pub MetaDataWrites: super::DWORD,
    pub MetaDataWriteBytes: super::DWORD,
    pub MetaDataDiskWrites: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILESYSTEM_STATISTICS_EX {
    pub FileSystemType: super::WORD,
    pub Version: super::WORD,
    pub SizeOfCompleteStructure: super::DWORD,
    pub UserFileReads: super::DWORDLONG,
    pub UserFileReadBytes: super::DWORDLONG,
    pub UserDiskReads: super::DWORDLONG,
    pub UserFileWrites: super::DWORDLONG,
    pub UserFileWriteBytes: super::DWORDLONG,
    pub UserDiskWrites: super::DWORDLONG,
    pub MetaDataReads: super::DWORDLONG,
    pub MetaDataReadBytes: super::DWORDLONG,
    pub MetaDataDiskReads: super::DWORDLONG,
    pub MetaDataWrites: super::DWORDLONG,
    pub MetaDataWriteBytes: super::DWORDLONG,
    pub MetaDataDiskWrites: super::DWORDLONG,
}
pub const FILESYSTEM_STATISTICS_TYPE_EXFAT: i32 = 3;
pub const FILESYSTEM_STATISTICS_TYPE_FAT: i32 = 2;
pub const FILESYSTEM_STATISTICS_TYPE_NTFS: i32 = 1;
pub const FILESYSTEM_STATISTICS_TYPE_REFS: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_ALLOCATED_RANGE_BUFFER {
    pub FileOffset: super::LARGE_INTEGER,
    pub Length: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_ALLOCATED_RANGE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_ANY_ACCESS: i32 = 0;
pub const FILE_CLEAR_ENCRYPTION: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    pub Class: FILE_STORAGE_TIER_CLASS,
    pub Flags: super::DWORD,
}
pub const FILE_DEVICE_8042_PORT: i32 = 39;
pub const FILE_DEVICE_ACPI: i32 = 50;
pub const FILE_DEVICE_BATTERY: i32 = 41;
pub const FILE_DEVICE_BEEP: i32 = 1;
pub const FILE_DEVICE_BIOMETRIC: i32 = 68;
pub const FILE_DEVICE_BLUETOOTH: i32 = 65;
pub const FILE_DEVICE_BUS_EXTENDER: i32 = 42;
pub const FILE_DEVICE_CD_ROM: i32 = 2;
pub const FILE_DEVICE_CD_ROM_FILE_SYSTEM: i32 = 3;
pub const FILE_DEVICE_CHANGER: i32 = 48;
pub const FILE_DEVICE_CONSOLE: i32 = 80;
pub const FILE_DEVICE_CONTROLLER: i32 = 4;
pub const FILE_DEVICE_CRYPT_PROVIDER: i32 = 63;
pub const FILE_DEVICE_CXL_TYPE3: i32 = 103;
pub const FILE_DEVICE_DATALINK: i32 = 5;
pub const FILE_DEVICE_DEVAPI: i32 = 71;
pub const FILE_DEVICE_DFS: i32 = 6;
pub const FILE_DEVICE_DFS_FILE_SYSTEM: i32 = 53;
pub const FILE_DEVICE_DFS_VOLUME: i32 = 54;
pub const FILE_DEVICE_DISK: i32 = 7;
pub const FILE_DEVICE_DISK_FILE_SYSTEM: i32 = 8;
pub const FILE_DEVICE_DVD: i32 = 51;
pub const FILE_DEVICE_EHSTOR: i32 = 70;
pub const FILE_DEVICE_EVENT_COLLECTOR: i32 = 95;
pub const FILE_DEVICE_FABRIC_NVME: i32 = 98;
pub const FILE_DEVICE_FILE_SYSTEM: i32 = 9;
pub const FILE_DEVICE_FIPS: i32 = 58;
pub const FILE_DEVICE_FULLSCREEN_VIDEO: i32 = 52;
pub const FILE_DEVICE_GPIO: i32 = 72;
pub const FILE_DEVICE_HARDWARE_ACCELERATOR: i32 = 100;
pub const FILE_DEVICE_HOLOGRAPHIC: i32 = 91;
pub const FILE_DEVICE_I3C: i32 = 101;
pub const FILE_DEVICE_INFINIBAND: i32 = 59;
pub const FILE_DEVICE_INPORT_PORT: i32 = 10;
pub const FILE_DEVICE_KEYBOARD: i32 = 11;
pub const FILE_DEVICE_KS: i32 = 47;
pub const FILE_DEVICE_KSEC: i32 = 57;
pub const FILE_DEVICE_MAILSLOT: i32 = 12;
pub const FILE_DEVICE_MASS_STORAGE: i32 = 45;
pub const FILE_DEVICE_MIDI_IN: i32 = 13;
pub const FILE_DEVICE_MIDI_OUT: i32 = 14;
pub const FILE_DEVICE_MODEM: i32 = 43;
pub const FILE_DEVICE_MOUSE: i32 = 15;
pub const FILE_DEVICE_MT_COMPOSITE: i32 = 66;
pub const FILE_DEVICE_MT_TRANSPORT: i32 = 67;
pub const FILE_DEVICE_MULTITIER_MEMORY: i32 = 102;
pub const FILE_DEVICE_MULTI_UNC_PROVIDER: i32 = 16;
pub const FILE_DEVICE_NAMED_PIPE: i32 = 17;
pub const FILE_DEVICE_NETWORK: i32 = 18;
pub const FILE_DEVICE_NETWORK_BROWSER: i32 = 19;
pub const FILE_DEVICE_NETWORK_FILE_SYSTEM: i32 = 20;
pub const FILE_DEVICE_NETWORK_REDIRECTOR: i32 = 40;
pub const FILE_DEVICE_NFP: i32 = 81;
pub const FILE_DEVICE_NULL: i32 = 21;
pub const FILE_DEVICE_NVDIMM: i32 = 90;
pub const FILE_DEVICE_PARALLEL_PORT: i32 = 22;
pub const FILE_DEVICE_PERSISTENT_MEMORY: i32 = 89;
pub const FILE_DEVICE_PHYSICAL_NETCARD: i32 = 23;
pub const FILE_DEVICE_PMI: i32 = 69;
pub const FILE_DEVICE_POINT_OF_SERVICE: i32 = 84;
pub const FILE_DEVICE_PRINTER: i32 = 24;
pub const FILE_DEVICE_PRM: i32 = 94;
pub const FILE_DEVICE_SCANNER: i32 = 25;
pub const FILE_DEVICE_SCREEN: i32 = 28;
pub const FILE_DEVICE_SDFXHCI: i32 = 92;
pub const FILE_DEVICE_SERENUM: i32 = 55;
pub const FILE_DEVICE_SERIAL_MOUSE_PORT: i32 = 26;
pub const FILE_DEVICE_SERIAL_PORT: i32 = 27;
pub const FILE_DEVICE_SMB: i32 = 46;
pub const FILE_DEVICE_SOUND: i32 = 29;
pub const FILE_DEVICE_SOUNDWIRE: i32 = 97;
pub const FILE_DEVICE_STORAGE_REPLICATION: i32 = 85;
pub const FILE_DEVICE_STREAMS: i32 = 30;
pub const FILE_DEVICE_SVM: i32 = 99;
pub const FILE_DEVICE_SYSENV: i32 = 82;
pub const FILE_DEVICE_TAPE: i32 = 31;
pub const FILE_DEVICE_TAPE_FILE_SYSTEM: i32 = 32;
pub const FILE_DEVICE_TERMSRV: i32 = 56;
pub const FILE_DEVICE_TRANSPORT: i32 = 33;
pub const FILE_DEVICE_TRUST_ENV: i32 = 86;
pub const FILE_DEVICE_UCM: i32 = 87;
pub const FILE_DEVICE_UCMTCPCI: i32 = 88;
pub const FILE_DEVICE_UCMUCSI: i32 = 93;
pub const FILE_DEVICE_UNKNOWN: i32 = 34;
pub const FILE_DEVICE_USB4: i32 = 96;
pub const FILE_DEVICE_USBEX: i32 = 73;
pub const FILE_DEVICE_VDM: i32 = 44;
pub const FILE_DEVICE_VIDEO: i32 = 35;
pub const FILE_DEVICE_VIRTUAL_BLOCK: i32 = 83;
pub const FILE_DEVICE_VIRTUAL_DISK: i32 = 36;
pub const FILE_DEVICE_VMBUS: i32 = 62;
pub const FILE_DEVICE_WAVE_IN: i32 = 37;
pub const FILE_DEVICE_WAVE_OUT: i32 = 38;
pub const FILE_DEVICE_WPD: i32 = 64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    pub VolumeFlags: super::DWORD,
    pub FlagMask: super::DWORD,
    pub Version: super::DWORD,
    pub Reserved: super::DWORD,
}
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NON_RESIDENT: i64 = 137438953472;
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NOT_FOUND: i32 = 4096;
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_TOO_SMALL: i64 = 68719476736;
pub const FILE_INITIATE_REPAIR_HINT1_CLUSTERS_ALREADY_IN_USE: i32 = 32768;
pub const FILE_INITIATE_REPAIR_HINT1_DENY_DEFRAG: i64 = 274877906944;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_IS_BASE_RECORD: i32 = 524288;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_BASE_RECORD: i32 = 8;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_EXIST: i32 = 4;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_IN_USE: i32 = 1;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_ORPHAN: i32 = 262144;
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_REUSED: i32 = 2;
pub const FILE_INITIATE_REPAIR_HINT1_INDEX_ENTRY_MISMATCH: i64 = 1099511627776;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ARRAY_LENGTH_COUNT: i32 = 1048576;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_LCN: i64 = 4294967296;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ORPHAN_RECOVERY_NAME: i64 = 2199023255552;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_PARENT: i32 = 8388608;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_RUN_LENGTH: i32 = 131072;
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_VCN: i64 = 8589934592;
pub const FILE_INITIATE_REPAIR_HINT1_LCN_NOT_EXIST: i32 = 65536;
pub const FILE_INITIATE_REPAIR_HINT1_MULTIPLE_FILE_NAME_ATTRIBUTES: i64 = 4398046511104;
pub const FILE_INITIATE_REPAIR_HINT1_NAME_CONFLICT: i64 = 17179869184;
pub const FILE_INITIATE_REPAIR_HINT1_NOTHING_WRONG: i32 = 2048;
pub const FILE_INITIATE_REPAIR_HINT1_NOT_IMPLEMENTED: i32 = 32;
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN: i64 = 34359738368;
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN_GENERATED: i32 = 512;
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_GENERIC_NAMES: i32 = 1073741824;
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_RESOURCE: u32 = 2147483648;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_BASE_RECORD: i32 = 134217728;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_EXIST: i32 = 67108864;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_INDEX: i32 = 268435456;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_IN_USE: i32 = 16777216;
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_REUSED: i32 = 33554432;
pub const FILE_INITIATE_REPAIR_HINT1_POTENTIAL_CROSSLINK: i32 = 8192;
pub const FILE_INITIATE_REPAIR_HINT1_PREVIOUS_PARENT_STILL_VALID: i64 = 549755813888;
pub const FILE_INITIATE_REPAIR_HINT1_RECURSIVELY_CORRUPTED: i32 = 256;
pub const FILE_INITIATE_REPAIR_HINT1_REPAIRED: i32 = 1024;
pub const FILE_INITIATE_REPAIR_HINT1_REPAIR_DISABLED: i32 = 128;
pub const FILE_INITIATE_REPAIR_HINT1_SID_MISMATCH: i32 = 4194304;
pub const FILE_INITIATE_REPAIR_HINT1_SID_VALID: i32 = 2097152;
pub const FILE_INITIATE_REPAIR_HINT1_STALE_INFORMATION: i32 = 16384;
pub const FILE_INITIATE_REPAIR_HINT1_SYSTEM_FILE: i32 = 16;
pub const FILE_INITIATE_REPAIR_HINT1_UNABLE_TO_REPAIR: i32 = 64;
pub const FILE_INITIATE_REPAIR_HINT1_VALID_INDEX_ENTRY: i32 = 536870912;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    pub Hint1: super::DWORDLONG,
    pub Hint2: super::DWORDLONG,
    pub Clsn: CLSN,
    pub Status: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_LAYOUT_ENTRY {
    pub Version: super::DWORD,
    pub NextFileOffset: super::DWORD,
    pub Flags: super::DWORD,
    pub FileAttributes: super::DWORD,
    pub FileReferenceNumber: super::DWORDLONG,
    pub FirstNameOffset: super::DWORD,
    pub FirstStreamOffset: super::DWORD,
    pub ExtraInfoOffset: super::DWORD,
    pub ExtraInfoLength: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_LAYOUT_INFO_ENTRY {
    pub BasicInformation: FILE_LAYOUT_INFO_ENTRY_0,
    pub OwnerId: super::DWORD,
    pub SecurityId: super::DWORD,
    pub Usn: super::USN,
    pub StorageReserveId: STORAGE_RESERVE_ID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_LAYOUT_INFO_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_LAYOUT_INFO_ENTRY_0 {
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub FileAttributes: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_LAYOUT_INFO_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LAYOUT_NAME_ENTRY {
    pub NextNameOffset: super::DWORD,
    pub Flags: super::DWORD,
    pub ParentFileReferenceNumber: super::DWORDLONG,
    pub FileNameLength: super::DWORD,
    pub Reserved: super::DWORD,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_LAYOUT_NAME_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_LAYOUT_NAME_ENTRY_DOS: i32 = 2;
pub const FILE_LAYOUT_NAME_ENTRY_PRIMARY: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_LEVEL_TRIM {
    pub Key: super::DWORD,
    pub NumRanges: super::DWORD,
    pub Ranges: [FILE_LEVEL_TRIM_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for FILE_LEVEL_TRIM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_LEVEL_TRIM_OUTPUT {
    pub NumRangesProcessed: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_LEVEL_TRIM_RANGE {
    pub Offset: super::DWORDLONG,
    pub Length: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_MAKE_COMPATIBLE_BUFFER {
    pub CloseDisc: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct FILE_OBJECTID_BUFFER {
    pub ObjectId: [super::BYTE; 16],
    pub Anonymous: FILE_OBJECTID_BUFFER_0,
}
#[cfg(feature = "shared")]
impl Default for FILE_OBJECTID_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union FILE_OBJECTID_BUFFER_0 {
    pub Anonymous: FILE_OBJECTID_BUFFER_0_0,
    pub ExtendedInfo: [super::BYTE; 48],
}
#[cfg(feature = "shared")]
impl Default for FILE_OBJECTID_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_OBJECTID_BUFFER_0_0 {
    pub BirthVolumeId: [super::BYTE; 16],
    pub BirthObjectId: [super::BYTE; 16],
    pub DomainId: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for FILE_OBJECTID_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PREFETCH {
    pub Type: super::DWORD,
    pub Count: super::DWORD,
    pub Prefetch: [super::DWORDLONG; 1],
}
#[cfg(feature = "shared")]
impl Default for FILE_PREFETCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_PREFETCH_EX {
    pub Type: super::DWORD,
    pub Count: super::DWORD,
    pub Context: super::PVOID,
    pub Prefetch: [super::DWORDLONG; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_PREFETCH_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PREFETCH_TYPE_FOR_CREATE: i32 = 1;
pub const FILE_PREFETCH_TYPE_FOR_CREATE_EX: i32 = 3;
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM: i32 = 2;
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM_EX: i32 = 4;
pub const FILE_PREFETCH_TYPE_MAX: i32 = 4;
pub const FILE_PROVIDER_COMPRESSION_LZX: i32 = 1;
pub const FILE_PROVIDER_COMPRESSION_MAXIMUM: i32 = 4;
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: i32 = 3;
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: i32 = 0;
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: i32 = 2;
pub const FILE_PROVIDER_CURRENT_VERSION: i32 = 1;
#[cfg(feature = "shared")]
pub type FILE_PROVIDER_EXTERNAL_INFO = FILE_PROVIDER_EXTERNAL_INFO_V1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PROVIDER_EXTERNAL_INFO_V0 {
    pub Version: super::DWORD,
    pub Algorithm: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_PROVIDER_EXTERNAL_INFO_V1 {
    pub Version: super::DWORD,
    pub Algorithm: super::DWORD,
    pub Flags: super::DWORD,
}
pub const FILE_PROVIDER_FLAG_COMPRESS_ON_WRITE: i32 = 1;
pub const FILE_PROVIDER_SINGLE_FILE: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    pub DirectoryCount: super::LARGE_INTEGER,
    pub FileCount: super::LARGE_INTEGER,
    pub FsFormatMajVersion: super::WORD,
    pub FsFormatMinVersion: super::WORD,
    pub FsFormatName: [super::WCHAR; 12],
    pub FormatTime: super::LARGE_INTEGER,
    pub LastUpdateTime: super::LARGE_INTEGER,
    pub CopyrightInfo: [super::WCHAR; 34],
    pub AbstractInfo: [super::WCHAR; 34],
    pub FormattingImplementationInfo: [super::WCHAR; 34],
    pub LastModifyingImplementationInfo: [super::WCHAR; 34],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_QUERY_SPARING_BUFFER {
    pub SparingUnitBytes: super::DWORD,
    pub SoftwareSparing: super::BOOLEAN,
    pub TotalSpareBlocks: super::DWORD,
    pub FreeSpareBlocks: super::DWORD,
}
pub const FILE_READ_ACCESS: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REFERENCE_RANGE {
    pub StartingFileReferenceNumber: super::DWORDLONG,
    pub EndingFileReferenceNumber: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REGION_INFO {
    pub FileOffset: super::LONGLONG,
    pub Length: super::LONGLONG,
    pub Usage: super::DWORD,
    pub Reserved: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REGION_INPUT {
    pub FileOffset: super::LONGLONG,
    pub Length: super::LONGLONG,
    pub DesiredUsage: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REGION_OUTPUT {
    pub Flags: super::DWORD,
    pub TotalRegionEntryCount: super::DWORD,
    pub RegionEntryCount: super::DWORD,
    pub Reserved: super::DWORD,
    pub Region: [FILE_REGION_INFO; 1],
}
#[cfg(feature = "shared")]
impl Default for FILE_REGION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FILE_REGION_USAGE_HUGE_PAGE_ALIGNMENT: i32 = 16;
pub const FILE_REGION_USAGE_LARGE_PAGE_ALIGNMENT: i32 = 8;
pub const FILE_REGION_USAGE_OTHER_PAGE_ALIGNMENT: i32 = 4;
#[cfg(target_arch = "x86")]
pub const FILE_REGION_USAGE_QUERY_ALIGNMENT: i32 = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FILE_REGION_USAGE_QUERY_ALIGNMENT: i32 = 24;
pub const FILE_REGION_USAGE_VALID_CACHED_DATA: i32 = 1;
pub const FILE_REGION_USAGE_VALID_NONCACHED_DATA: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_SET_DEFECT_MGMT_BUFFER {
    pub Disable: super::BOOLEAN,
}
pub const FILE_SET_ENCRYPTION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_SET_SPARSE_BUFFER {
    pub SetSparse: super::BOOLEAN,
}
pub const FILE_SPECIAL_ACCESS: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_STORAGE_TIER {
    pub Id: windows_core::GUID,
    pub Name: [super::WCHAR; 256],
    pub Description: [super::WCHAR; 256],
    pub Flags: super::DWORDLONG,
    pub ProvisionedCapacity: super::DWORDLONG,
    pub MediaType: FILE_STORAGE_TIER_MEDIA_TYPE,
    pub Class: FILE_STORAGE_TIER_CLASS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_STORAGE_TIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FILE_STORAGE_TIER_CLASS = i32;
pub const FILE_STORAGE_TIER_DESCRIPTION_LENGTH: i32 = 512;
pub const FILE_STORAGE_TIER_FLAG_NO_SEEK_PENALTY: i32 = 131072;
pub const FILE_STORAGE_TIER_FLAG_PARITY: i32 = 8388608;
pub const FILE_STORAGE_TIER_FLAG_READ_CACHE: i32 = 4194304;
pub const FILE_STORAGE_TIER_FLAG_SMR: i32 = 16777216;
pub const FILE_STORAGE_TIER_FLAG_WRITE_BACK_CACHE: i32 = 2097152;
pub type FILE_STORAGE_TIER_MEDIA_TYPE = i32;
pub const FILE_STORAGE_TIER_NAME_LENGTH: i32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_STORAGE_TIER_REGION {
    pub TierId: windows_core::GUID,
    pub Offset: super::DWORDLONG,
    pub Length: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_SYSTEM_RECOGNITION_INFORMATION {
    pub FileSystem: [super::CHAR; 9],
}
#[cfg(feature = "shared")]
impl Default for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_BEGIN: i32 = 1;
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_END: i32 = 2;
pub const FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE: windows_core::GUID = windows_core::GUID::from_u128(0x9d453eb7_d2a6_4dbd_a2e3_fbd0ed9109a9);
pub const FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE: windows_core::GUID = windows_core::GUID::from_u128(0xb7624d64_b9a3_4cf8_8011_5b86c940e7b7);
pub const FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE: windows_core::GUID = windows_core::GUID::from_u128(0x0d0a64a1_38fc_4db8_9fe7_3f4352cd7c5c);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_TYPE_NOTIFICATION_INPUT {
    pub Flags: super::DWORD,
    pub NumFileTypeIDs: super::DWORD,
    pub FileTypeID: [windows_core::GUID; 1],
}
#[cfg(feature = "shared")]
impl Default for FILE_TYPE_NOTIFICATION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_WRITE_ACCESS: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_ZERO_DATA_INFORMATION {
    pub FileOffset: super::LARGE_INTEGER,
    pub BeyondFinalZero: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_ZERO_DATA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_ZERO_DATA_INFORMATION_EX {
    pub FileOffset: super::LARGE_INTEGER,
    pub BeyondFinalZero: super::LARGE_INTEGER,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_ZERO_DATA_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_ZERO_DATA_INFORMATION_FLAG_PRESERVE_CACHED_DATA: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FIND_BY_SID_DATA {
    pub Restart: super::DWORD,
    pub Sid: super::SID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FIND_BY_SID_OUTPUT {
    pub NextEntryOffset: super::DWORD,
    pub FileIndex: super::DWORD,
    pub FileNameLength: super::DWORD,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FIND_BY_SID_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLAG_USN_TRACK_MODIFIED_RANGES_ENABLE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FORMAT_EX_PARAMETERS {
    pub MediaType: MEDIA_TYPE,
    pub StartCylinderNumber: super::DWORD,
    pub EndCylinderNumber: super::DWORD,
    pub StartHeadNumber: super::DWORD,
    pub EndHeadNumber: super::DWORD,
    pub FormatGapLength: super::WORD,
    pub SectorsPerTrack: super::WORD,
    pub SectorNumber: [super::WORD; 1],
}
#[cfg(feature = "shared")]
impl Default for FORMAT_EX_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FORMAT_PARAMETERS {
    pub MediaType: MEDIA_TYPE,
    pub StartCylinderNumber: super::DWORD,
    pub EndCylinderNumber: super::DWORD,
    pub StartHeadNumber: super::DWORD,
    pub EndHeadNumber: super::DWORD,
}
pub const FSBPIO_INFL_None: FS_BPIO_INFLAGS = 0;
pub const FSBPIO_INFL_SKIP_STORAGE_STACK_QUERY: FS_BPIO_INFLAGS = 1;
pub const FSBPIO_OUTFL_COMPATIBLE_STORAGE_DRIVER: FS_BPIO_OUTFLAGS = 8;
pub const FSBPIO_OUTFL_FILTER_ATTACH_BLOCKED: FS_BPIO_OUTFLAGS = 4;
pub const FSBPIO_OUTFL_None: FS_BPIO_OUTFLAGS = 0;
pub const FSBPIO_OUTFL_STREAM_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = 2;
pub const FSBPIO_OUTFL_VOLUME_STACK_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = 1;
pub const FSCTL_ADD_OVERLAY: i32 = 623408;
pub const FSCTL_ADVANCE_FILE_ID: i32 = 590532;
pub const FSCTL_ALLOW_EXTENDED_DASD_IO: i32 = 589955;
pub const FSCTL_CASCADES_REFS_SET_FILE_REMOTE: i32 = 591004;
pub const FSCTL_CIMFS_QUERY_BACKING_REGION_NAMES: i32 = 591008;
pub const FSCTL_CLEAN_VOLUME_METADATA: i32 = 590716;
pub const FSCTL_CLEAR_ALL_LCN_WEAK_REFERENCES: i32 = 590956;
pub const FSCTL_CLEAR_LCN_WEAK_REFERENCE: i32 = 590948;
pub const FSCTL_CORRUPTION_HANDLING: i32 = 590432;
pub const FSCTL_CREATE_LCN_WEAK_REFERENCE: i32 = 590944;
pub const FSCTL_CREATE_OR_GET_OBJECT_ID: i32 = 590016;
pub const FSCTL_CREATE_USN_JOURNAL: i32 = 590055;
pub const FSCTL_CSC_INTERNAL: i32 = 590255;
pub const FSCTL_CSV_CONTROL: i32 = 590548;
pub const FSCTL_CSV_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT: i32 = 590420;
pub const FSCTL_CSV_GET_VOLUME_PATH_NAME: i32 = 590416;
pub const FSCTL_CSV_GET_VOLUME_PATH_NAMES_FOR_VOLUME_NAME: i32 = 590424;
pub const FSCTL_CSV_H_BREAKING_SYNC_TUNNEL_REQUEST: i32 = 590564;
pub const FSCTL_CSV_INTERNAL: i32 = 590444;
pub const FSCTL_CSV_MGMT_LOCK: i32 = 590524;
pub const FSCTL_CSV_QUERY_DOWN_LEVEL_FILE_SYSTEM_CHARACTERISTICS: i32 = 590528;
pub const FSCTL_CSV_QUERY_VETO_FILE_DIRECT_IO: i32 = 590540;
pub const FSCTL_CSV_SYNC_TUNNEL_REQUEST: i32 = 590536;
pub const FSCTL_CSV_TUNNEL_REQUEST: i32 = 590404;
pub const FSCTL_DELETE_CORRUPTED_REFS_CONTAINER: i32 = 590836;
pub const FSCTL_DELETE_EXTERNAL_BACKING: i32 = 590612;
pub const FSCTL_DELETE_OBJECT_ID: i32 = 589984;
pub const FSCTL_DELETE_REPARSE_POINT: i32 = 589996;
pub const FSCTL_DELETE_USN_JOURNAL: i32 = 590072;
pub const FSCTL_DFSR_SET_GHOST_HANDLE_STATE: i32 = 590264;
pub const FSCTL_DISABLE_LOCAL_BUFFERING: i32 = 590520;
pub const FSCTL_DISMOUNT_VOLUME: i32 = 589856;
pub const FSCTL_DUPLICATE_CLUSTER: i32 = 590940;
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE: i32 = 623428;
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX: i32 = 623592;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FSCTL_ENABLE_PER_IO_FLAGS: i32 = 590892;
pub const FSCTL_ENABLE_UPGRADE: i32 = 622800;
pub const FSCTL_ENCRYPTION_FSCTL_IO: i32 = 590043;
pub const FSCTL_ENCRYPTION_KEY_CONTROL: i32 = 590852;
pub const FSCTL_ENUM_EXTERNAL_BACKING: i32 = 590616;
pub const FSCTL_ENUM_OVERLAY: i32 = 590623;
pub const FSCTL_ENUM_USN_DATA: i32 = 590003;
pub const FSCTL_EXTEND_VOLUME: i32 = 590064;
pub const FSCTL_FILESYSTEM_GET_STATISTICS: i32 = 589920;
pub const FSCTL_FILESYSTEM_GET_STATISTICS_EX: i32 = 590732;
pub const FSCTL_FILE_LEVEL_TRIM: i32 = 623112;
pub const FSCTL_FILE_PREFETCH: i32 = 590112;
pub const FSCTL_FILE_SOV_CHECK_RANGE: i32 = 591000;
pub const FSCTL_FILE_TYPE_NOTIFICATION: i32 = 590340;
pub const FSCTL_FIND_FILES_BY_SID: i32 = 589967;
pub const FSCTL_GET_BOOT_AREA_INFO: i32 = 590384;
pub const FSCTL_GET_COMPRESSION: i32 = 589884;
pub const FSCTL_GET_EXTERNAL_BACKING: i32 = 590608;
pub const FSCTL_GET_FILTER_FILE_IDENTIFIER: i32 = 590788;
pub const FSCTL_GET_INTEGRITY_INFORMATION: i32 = 590460;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    pub ChecksumAlgorithm: super::WORD,
    pub Reserved: super::WORD,
    pub Flags: super::DWORD,
    pub ChecksumChunkSizeInBytes: super::DWORD,
    pub ClusterSizeInBytes: super::DWORD,
}
pub const FSCTL_GET_NTFS_FILE_RECORD: i32 = 589928;
pub const FSCTL_GET_NTFS_VOLUME_DATA: i32 = 589924;
pub const FSCTL_GET_OBJECT_ID: i32 = 589980;
pub const FSCTL_GET_REFS_VOLUME_DATA: i32 = 590552;
pub const FSCTL_GET_REPAIR: i32 = 590236;
pub const FSCTL_GET_REPARSE_POINT: i32 = 589992;
pub const FSCTL_GET_RETRIEVAL_POINTERS: i32 = 589939;
pub const FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT: i32 = 590803;
pub const FSCTL_GET_RETRIEVAL_POINTER_BASE: i32 = 590388;
pub const FSCTL_GET_RETRIEVAL_POINTER_COUNT: i32 = 590891;
pub const FSCTL_GET_VOLUME_BITMAP: i32 = 589935;
pub const FSCTL_GET_WOF_VERSION: i32 = 590696;
pub const FSCTL_GHOST_FILE_EXTENTS: i32 = 623532;
pub const FSCTL_HCS_ASYNC_TUNNEL_REQUEST: i32 = 590704;
pub const FSCTL_HCS_SYNC_NO_WRITE_TUNNEL_REQUEST: i32 = 590776;
pub const FSCTL_HCS_SYNC_TUNNEL_REQUEST: i32 = 590700;
pub const FSCTL_INITIATE_FILE_METADATA_OPTIMIZATION: i32 = 590684;
pub const FSCTL_INITIATE_REPAIR: i32 = 590248;
pub const FSCTL_INTEGRITY_FLAG_CHECKSUM_ENFORCEMENT_OFF: i32 = 1;
pub const FSCTL_INVALIDATE_VOLUMES: i32 = 589908;
pub const FSCTL_IS_CSV_FILE: i32 = 590408;
pub const FSCTL_IS_FILE_ON_CSV_VOLUME: i32 = 590428;
pub const FSCTL_IS_PATHNAME_VALID: i32 = 589868;
pub const FSCTL_IS_VOLUME_DIRTY: i32 = 589944;
pub const FSCTL_IS_VOLUME_MOUNTED: i32 = 589864;
pub const FSCTL_IS_VOLUME_OWNED_BYCSVFS: i32 = 590456;
pub const FSCTL_LMR_QUERY_INFO: i32 = 590968;
pub const FSCTL_LOCK_VOLUME: i32 = 589848;
pub const FSCTL_LOOKUP_STREAM_FROM_CLUSTER: i32 = 590332;
pub const FSCTL_MAKE_MEDIA_COMPATIBLE: i32 = 622896;
pub const FSCTL_MANAGE_BYPASS_IO: i32 = 590920;
pub const FSCTL_MARK_AS_SYSTEM_HIVE: i32 = 589903;
pub const FSCTL_MARK_HANDLE: i32 = 590076;
pub const FSCTL_MARK_VOLUME_DIRTY: i32 = 589872;
pub const FSCTL_MOVE_FILE: i32 = 589940;
pub const FSCTL_NOTIFY_DATA_CHANGE: i32 = 590844;
pub const FSCTL_NOTIFY_STORAGE_SPACE_ALLOCATION: i32 = 590748;
pub const FSCTL_OFFLOAD_READ: i32 = 606820;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSCTL_OFFLOAD_READ_INPUT {
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub TokenTimeToLive: super::DWORD,
    pub Reserved: super::DWORD,
    pub FileOffset: super::DWORDLONG,
    pub CopyLength: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_OFFLOAD_READ_OUTPUT {
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub TransferLength: super::DWORDLONG,
    pub Token: [super::BYTE; 512],
}
#[cfg(feature = "shared")]
impl Default for FSCTL_OFFLOAD_READ_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_OFFLOAD_WRITE: i32 = 623208;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_OFFLOAD_WRITE_INPUT {
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub FileOffset: super::DWORDLONG,
    pub CopyLength: super::DWORDLONG,
    pub TransferOffset: super::DWORDLONG,
    pub Token: [super::BYTE; 512],
}
#[cfg(feature = "shared")]
impl Default for FSCTL_OFFLOAD_WRITE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSCTL_OFFLOAD_WRITE_OUTPUT {
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub LengthWritten: super::DWORDLONG,
}
pub const FSCTL_OPBATCH_ACK_CLOSE_PENDING: i32 = 589840;
pub const FSCTL_OPLOCK_BREAK_ACKNOWLEDGE: i32 = 589836;
pub const FSCTL_OPLOCK_BREAK_ACK_NO_2: i32 = 589904;
pub const FSCTL_OPLOCK_BREAK_NOTIFY: i32 = 589844;
pub const FSCTL_QUERY_ALLOCATED_RANGES: i32 = 606415;
pub const FSCTL_QUERY_ASYNC_DUPLICATE_EXTENTS_STATUS: i32 = 590896;
pub const FSCTL_QUERY_BAD_RANGES: i32 = 590828;
pub const FSCTL_QUERY_DEPENDENT_VOLUME: i32 = 590320;
pub const FSCTL_QUERY_DIRECT_ACCESS_EXTENTS: i32 = 590747;
pub const FSCTL_QUERY_DIRECT_IMAGE_ORIGINAL_BASE: i32 = 590756;
pub const FSCTL_QUERY_EXTENT_READ_CACHE_INFO: i32 = 590711;
pub const FSCTL_QUERY_FAT_BPB: i32 = 589912;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_QUERY_FAT_BPB_BUFFER {
    pub First0x24BytesOfBootSector: [super::BYTE; 36],
}
#[cfg(feature = "shared")]
impl Default for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_QUERY_FILE_LAYOUT: i32 = 590455;
pub const FSCTL_QUERY_FILE_METADATA_OPTIMIZATION: i32 = 590688;
pub const FSCTL_QUERY_FILE_REGIONS: i32 = 590468;
pub const FSCTL_QUERY_FILE_SYSTEM_RECOGNITION: i32 = 590412;
pub const FSCTL_QUERY_GHOSTED_FILE_EXTENTS: i32 = 590768;
pub const FSCTL_QUERY_LCN_WEAK_REFERENCE: i32 = 590952;
pub const FSCTL_QUERY_ON_DISK_VOLUME_INFO: i32 = 590140;
pub const FSCTL_QUERY_PAGEFILE_ENCRYPTION: i32 = 590312;
pub const FSCTL_QUERY_PERSISTENT_VOLUME_STATE: i32 = 590396;
pub const FSCTL_QUERY_REFS_SMR_VOLUME_INFO: i32 = 590812;
pub const FSCTL_QUERY_REFS_VOLUME_COUNTER_INFO: i32 = 590715;
pub const FSCTL_QUERY_REGION_INFO: i32 = 590576;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_QUERY_REGION_INFO_INPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub NumberOfTierIds: super::DWORD,
    pub TierIds: [windows_core::GUID; 1],
}
#[cfg(feature = "shared")]
impl Default for FSCTL_QUERY_REGION_INFO_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const FSCTL_QUERY_REGION_INFO_INPUT_VERSION: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FSCTL_QUERY_REGION_INFO_INPUT_VERSION: u64 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_QUERY_REGION_INFO_OUTPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub Reserved: super::DWORD,
    pub Alignment: super::DWORDLONG,
    pub TotalNumberOfRegions: super::DWORD,
    pub NumberOfRegionsReturned: super::DWORD,
    pub Regions: [FILE_STORAGE_TIER_REGION; 1],
}
#[cfg(feature = "shared")]
impl Default for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const FSCTL_QUERY_REGION_INFO_OUTPUT_VERSION: u32 = 64;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FSCTL_QUERY_REGION_INFO_OUTPUT_VERSION: u64 = 64;
pub const FSCTL_QUERY_RETRIEVAL_POINTERS: i32 = 589883;
pub const FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT: i32 = 590592;
pub const FSCTL_QUERY_SPARING_INFO: i32 = 590136;
pub const FSCTL_QUERY_STORAGE_CLASSES: i32 = 590572;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub TotalNumberOfTiers: super::DWORD,
    pub NumberOfTiersReturned: super::DWORD,
    pub Tiers: [FILE_STORAGE_TIER; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const FSCTL_QUERY_STORAGE_CLASSES_OUTPUT_VERSION: u32 = 1088;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FSCTL_QUERY_STORAGE_CLASSES_OUTPUT_VERSION: u64 = 1088;
pub const FSCTL_QUERY_USN_JOURNAL: i32 = 590068;
pub const FSCTL_QUERY_VOLUME_CONTAINER_STATE: i32 = 590736;
pub const FSCTL_QUERY_VOLUME_NUMA_INFO: i32 = 590804;
pub const FSCTL_READ_FILE_USN_DATA: i32 = 590059;
pub const FSCTL_READ_FROM_PLEX: i32 = 606494;
pub const FSCTL_READ_RAW_ENCRYPTED: i32 = 590051;
pub const FSCTL_READ_UNPRIVILEGED_USN_JOURNAL: i32 = 590763;
pub const FSCTL_READ_USN_JOURNAL: i32 = 590011;
pub const FSCTL_REARRANGE_FILE: i32 = 640032;
pub const FSCTL_RECALL_FILE: i32 = 590103;
pub const FSCTL_REFS_CHECKPOINT_VOLUME: i32 = 590972;
pub const FSCTL_REFS_DEALLOCATE_RANGES: i32 = 590808;
pub const FSCTL_REFS_DEALLOCATE_RANGES_EX: i32 = 590924;
pub const FSCTL_REFS_QUERY_ROLLBACK_PROTECTION_INFO: i32 = 590996;
pub const FSCTL_REFS_QUERY_VOLUME_COMPRESSION_INFO: i32 = 590936;
pub const FSCTL_REFS_QUERY_VOLUME_DEDUP_INFO: i32 = 590964;
pub const FSCTL_REFS_QUERY_VOLUME_IO_METRICS_INFO: i32 = 590988;
pub const FSCTL_REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS: i32 = 590976;
pub const FSCTL_REFS_SET_ROLLBACK_PROTECTION_INFO: i32 = 590992;
pub const FSCTL_REFS_SET_VOLUME_COMPRESSION_INFO: i32 = 590932;
pub const FSCTL_REFS_SET_VOLUME_DEDUP_INFO: i32 = 590960;
pub const FSCTL_REFS_SET_VOLUME_IO_METRICS_INFO: i32 = 590984;
pub const FSCTL_REFS_STREAM_SNAPSHOT_MANAGEMENT: i32 = 590912;
pub const FSCTL_REFS_VOLUME_ATTESTATION_INJECT_CERTIFICATE: i32 = 591016;
pub const FSCTL_REFS_VOLUME_ATTESTATION_PREPARE_TO_SIGN: i32 = 591012;
pub const FSCTL_REFS_VOLUME_ATTESTATION_QUERY_STATUS: i32 = 591020;
pub const FSCTL_REMOVE_OVERLAY: i32 = 623412;
pub const FSCTL_REPAIR_COPIES: i32 = 639668;
pub const FSCTL_REQUEST_BATCH_OPLOCK: i32 = 589832;
pub const FSCTL_REQUEST_FILTER_OPLOCK: i32 = 589916;
pub const FSCTL_REQUEST_OPLOCK: i32 = 590400;
pub const FSCTL_REQUEST_OPLOCK_LEVEL_1: i32 = 589824;
pub const FSCTL_REQUEST_OPLOCK_LEVEL_2: i32 = 589828;
pub const FSCTL_RESET_VOLUME_ALLOCATION_HINTS: i32 = 590316;
pub const FSCTL_RKF_INTERNAL: i32 = 590511;
pub const FSCTL_SCRUB_DATA: i32 = 590512;
pub const FSCTL_SCRUB_UNDISCOVERABLE_ID: i32 = 590840;
pub const FSCTL_SD_GLOBAL_CHANGE: i32 = 590324;
pub const FSCTL_SECURITY_ID_CHECK: i32 = 606391;
pub const FSCTL_SET_BOOTLOADER_ACCESSED: i32 = 589903;
pub const FSCTL_SET_CACHED_RUNS_STATE: i32 = 590928;
pub const FSCTL_SET_COMPRESSION: i32 = 639040;
pub const FSCTL_SET_DAX_ALLOC_ALIGNMENT_HINT: i32 = 590832;
pub const FSCTL_SET_DEFECT_MANAGEMENT: i32 = 622900;
pub const FSCTL_SET_ENCRYPTION: i32 = 590039;
pub const FSCTL_SET_EXTERNAL_BACKING: i32 = 590604;
pub const FSCTL_SET_INTEGRITY_INFORMATION: i32 = 639616;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    pub ChecksumAlgorithm: super::WORD,
    pub Reserved: super::WORD,
    pub Flags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    pub EnableIntegrity: super::BYTE,
    pub KeepIntegrityStateUnchanged: super::BYTE,
    pub Reserved: super::WORD,
    pub Flags: super::DWORD,
    pub Version: super::BYTE,
    pub Reserved2: [super::BYTE; 7],
}
#[cfg(feature = "shared")]
impl Default for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_SET_INTEGRITY_INFORMATION_EX: i32 = 590720;
pub const FSCTL_SET_LAYER_ROOT: i32 = 590740;
pub const FSCTL_SET_OBJECT_ID: i32 = 589976;
pub const FSCTL_SET_OBJECT_ID_EXTENDED: i32 = 590012;
pub const FSCTL_SET_PERSISTENT_VOLUME_STATE: i32 = 590392;
pub const FSCTL_SET_PURGE_FAILURE_MODE: i32 = 590448;
pub const FSCTL_SET_REFS_FILE_STRICTLY_SEQUENTIAL: i32 = 590820;
pub const FSCTL_SET_REFS_SMR_VOLUME_GC_PARAMETERS: i32 = 590816;
pub const FSCTL_SET_REPAIR: i32 = 590232;
pub const FSCTL_SET_REPARSE_POINT: i32 = 589988;
pub const FSCTL_SET_REPARSE_POINT_EX: i32 = 590860;
pub const FSCTL_SET_SHORT_NAME_BEHAVIOR: i32 = 590260;
pub const FSCTL_SET_SPARSE: i32 = 590020;
pub const FSCTL_SET_VOLUME_COMPRESSION_STATE: i32 = 590144;
pub const FSCTL_SET_ZERO_DATA: i32 = 622792;
pub const FSCTL_SET_ZERO_ON_DEALLOCATION: i32 = 590228;
pub const FSCTL_SHRINK_VOLUME: i32 = 590256;
pub const FSCTL_SHUFFLE_FILE: i32 = 639808;
pub const FSCTL_SIS_COPYFILE: i32 = 590080;
pub const FSCTL_SIS_LINK_FILES: i32 = 639236;
pub const FSCTL_SMB_SHARE_FLUSH_AND_PURGE: i32 = 590908;
pub const FSCTL_SPARSE_OVERALLOCATE: i32 = 590668;
pub const FSCTL_SSDI_STORAGE_REQUEST: i32 = 590752;
pub const FSCTL_START_VIRTUALIZATION_INSTANCE: i32 = 590784;
pub const FSCTL_START_VIRTUALIZATION_INSTANCE_EX: i32 = 590848;
pub const FSCTL_STORAGE_QOS_CONTROL: i32 = 590672;
pub const FSCTL_STREAMS_ASSOCIATE_ID: i32 = 590792;
pub const FSCTL_STREAMS_QUERY_ID: i32 = 590796;
pub const FSCTL_STREAMS_QUERY_PARAMETERS: i32 = 590788;
pub const FSCTL_SUSPEND_OVERLAY: i32 = 590724;
pub const FSCTL_SVHDX_ASYNC_TUNNEL_REQUEST: i32 = 590692;
pub const FSCTL_SVHDX_SET_INITIATOR_INFORMATION: i32 = 590600;
pub const FSCTL_SVHDX_SYNC_TUNNEL_REQUEST: i32 = 590596;
pub const FSCTL_TXFS_CREATE_MINIVERSION: i32 = 622972;
pub const FSCTL_TXFS_CREATE_SECONDARY_RM: i32 = 622952;
pub const FSCTL_TXFS_GET_METADATA_INFO: i32 = 606572;
pub const FSCTL_TXFS_GET_TRANSACTED_VERSION: i32 = 606576;
pub const FSCTL_TXFS_LIST_TRANSACTIONS: i32 = 606692;
pub const FSCTL_TXFS_LIST_TRANSACTION_LOCKED_FILES: i32 = 606688;
pub const FSCTL_TXFS_MODIFY_RM: i32 = 622916;
pub const FSCTL_TXFS_QUERY_RM_INFORMATION: i32 = 606536;
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION: i32 = 606560;
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION2: i32 = 590328;
pub const FSCTL_TXFS_ROLLFORWARD_REDO: i32 = 622928;
pub const FSCTL_TXFS_ROLLFORWARD_UNDO: i32 = 622932;
pub const FSCTL_TXFS_SAVEPOINT_INFORMATION: i32 = 622968;
pub const FSCTL_TXFS_SHUTDOWN_RM: i32 = 622940;
pub const FSCTL_TXFS_START_RM: i32 = 622936;
pub const FSCTL_TXFS_TRANSACTION_ACTIVE: i32 = 606604;
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION: i32 = 622948;
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION2: i32 = 590336;
pub const FSCTL_UNLOCK_VOLUME: i32 = 589852;
pub const FSCTL_UNMAP_SPACE: i32 = 590772;
pub const FSCTL_UPDATE_OVERLAY: i32 = 623416;
pub const FSCTL_UPGRADE_VOLUME: i32 = 590980;
pub const FSCTL_USN_TRACK_MODIFIED_RANGES: i32 = 590580;
pub const FSCTL_VIRTUAL_STORAGE_PASSTHROUGH: i32 = 590884;
pub const FSCTL_VIRTUAL_STORAGE_QUERY_PROPERTY: i32 = 590728;
pub const FSCTL_VIRTUAL_STORAGE_SET_BEHAVIOR: i32 = 590856;
pub const FSCTL_WAIT_FOR_REPAIR: i32 = 590240;
pub const FSCTL_WRITE_RAW_ENCRYPTED: i32 = 590047;
pub const FSCTL_WRITE_USN_CLOSE_RECORD: i32 = 590063;
pub const FSCTL_WRITE_USN_REASON: i32 = 590544;
pub type FS_BPIO_INFLAGS = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_BPIO_INFO {
    pub ActiveBypassIoCount: super::DWORD,
    pub StorageDriverNameLen: super::WORD,
    pub StorageDriverName: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FS_BPIO_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FS_BPIO_INPUT {
    pub Operation: FS_BPIO_OPERATIONS,
    pub InFlags: FS_BPIO_INFLAGS,
    pub Reserved1: super::DWORDLONG,
    pub Reserved2: super::DWORDLONG,
}
pub type FS_BPIO_OPERATIONS = i32;
pub const FS_BPIO_OP_DISABLE: FS_BPIO_OPERATIONS = 2;
pub const FS_BPIO_OP_ENABLE: FS_BPIO_OPERATIONS = 1;
pub const FS_BPIO_OP_GET_INFO: FS_BPIO_OPERATIONS = 8;
pub const FS_BPIO_OP_MAX_OPERATION: FS_BPIO_OPERATIONS = 9;
pub const FS_BPIO_OP_QUERY: FS_BPIO_OPERATIONS = 3;
pub const FS_BPIO_OP_STREAM_PAUSE: FS_BPIO_OPERATIONS = 6;
pub const FS_BPIO_OP_STREAM_RESUME: FS_BPIO_OPERATIONS = 7;
pub const FS_BPIO_OP_VOLUME_STACK_PAUSE: FS_BPIO_OPERATIONS = 4;
pub const FS_BPIO_OP_VOLUME_STACK_RESUME: FS_BPIO_OPERATIONS = 5;
pub type FS_BPIO_OUTFLAGS = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FS_BPIO_OUTPUT {
    pub Operation: FS_BPIO_OPERATIONS,
    pub OutFlags: FS_BPIO_OUTFLAGS,
    pub Reserved1: super::DWORDLONG,
    pub Reserved2: super::DWORDLONG,
    pub Anonymous: FS_BPIO_OUTPUT_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FS_BPIO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FS_BPIO_OUTPUT_0 {
    pub Enable: FS_BPIO_RESULTS,
    pub Query: FS_BPIO_RESULTS,
    pub VolumeStackResume: FS_BPIO_RESULTS,
    pub StreamResume: FS_BPIO_RESULTS,
    pub GetInfo: FS_BPIO_INFO,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FS_BPIO_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const FS_BPIO_OUTPUT_DISABLE_SIZE: super::size_t = 24 as _;
#[cfg(target_arch = "x86")]
pub const FS_BPIO_OUTPUT_ENABLE_SIZE: u32 = 352;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FS_BPIO_OUTPUT_ENABLE_SIZE: u64 = 352;
#[cfg(target_arch = "x86")]
pub const FS_BPIO_OUTPUT_GET_INFO_SIZE: u32 = 96;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FS_BPIO_OUTPUT_GET_INFO_SIZE: u64 = 96;
#[cfg(target_arch = "x86")]
pub const FS_BPIO_OUTPUT_QUERY_SIZE: u32 = 352;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FS_BPIO_OUTPUT_QUERY_SIZE: u64 = 352;
#[cfg(feature = "shared")]
pub const FS_BPIO_OUTPUT_STREAM_PAUSE_SIZE: super::size_t = 24 as _;
#[cfg(target_arch = "x86")]
pub const FS_BPIO_OUTPUT_STREAM_RESUME_SIZE: u32 = 352;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FS_BPIO_OUTPUT_STREAM_RESUME_SIZE: u64 = 352;
#[cfg(feature = "shared")]
pub const FS_BPIO_OUTPUT_VOLUME_STACK_PAUSE_SIZE: super::size_t = 24 as _;
#[cfg(target_arch = "x86")]
pub const FS_BPIO_OUTPUT_VOLUME_STACK_RESUME_SIZE: u32 = 352;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FS_BPIO_OUTPUT_VOLUME_STACK_RESUME_SIZE: u64 = 352;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FS_BPIO_RESULTS {
    pub OpStatus: super::DWORD,
    pub FailingDriverNameLen: super::WORD,
    pub FailingDriverName: [super::WCHAR; 32],
    pub FailureReasonLen: super::WORD,
    pub FailureReason: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FS_BPIO_RESULTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FW_ISSUEID_NO_ISSUE: i32 = 0;
pub const FW_ISSUEID_UNKNOWN: u32 = 4294967295;
pub const FileSnapStateInactive: DUPLICATE_EXTENTS_STATE = 0;
pub const FileSnapStateSource: DUPLICATE_EXTENTS_STATE = 1;
pub const FileSnapStateTarget: DUPLICATE_EXTENTS_STATE = 2;
pub const FileStorageTierClassCapacity: FILE_STORAGE_TIER_CLASS = 1;
pub const FileStorageTierClassMax: FILE_STORAGE_TIER_CLASS = 3;
pub const FileStorageTierClassPerformance: FILE_STORAGE_TIER_CLASS = 2;
pub const FileStorageTierClassUnspecified: FILE_STORAGE_TIER_CLASS = 0;
pub const FileStorageTierMediaTypeDisk: FILE_STORAGE_TIER_MEDIA_TYPE = 1;
pub const FileStorageTierMediaTypeMax: FILE_STORAGE_TIER_MEDIA_TYPE = 5;
pub const FileStorageTierMediaTypeScm: FILE_STORAGE_TIER_MEDIA_TYPE = 4;
pub const FileStorageTierMediaTypeSsd: FILE_STORAGE_TIER_MEDIA_TYPE = 2;
pub const FileStorageTierMediaTypeUnspecified: FILE_STORAGE_TIER_MEDIA_TYPE = 0;
pub const FixedMedia: MEDIA_TYPE = 12;
pub const FormFactor1_8: STORAGE_DEVICE_FORM_FACTOR = 3;
pub const FormFactor1_8Less: STORAGE_DEVICE_FORM_FACTOR = 4;
pub const FormFactor2_5: STORAGE_DEVICE_FORM_FACTOR = 2;
pub const FormFactor3_5: STORAGE_DEVICE_FORM_FACTOR = 1;
pub const FormFactorDimm: STORAGE_DEVICE_FORM_FACTOR = 10;
pub const FormFactorEmbedded: STORAGE_DEVICE_FORM_FACTOR = 5;
pub const FormFactorM_2: STORAGE_DEVICE_FORM_FACTOR = 8;
pub const FormFactorMemoryCard: STORAGE_DEVICE_FORM_FACTOR = 6;
pub const FormFactorPCIeBoard: STORAGE_DEVICE_FORM_FACTOR = 9;
pub const FormFactorUnknown: STORAGE_DEVICE_FORM_FACTOR = 0;
pub const FormFactormSata: STORAGE_DEVICE_FORM_FACTOR = 7;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct GETVERSIONINPARAMS {
    pub bVersion: super::BYTE,
    pub bRevision: super::BYTE,
    pub bReserved: super::BYTE,
    pub bIDEDeviceMap: super::BYTE,
    pub fCapabilities: super::DWORD,
    pub dwReserved: [super::DWORD; 4],
}
#[cfg(feature = "shared")]
impl Default for GETVERSIONINPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_CHANGER_PARAMETERS {
    pub Size: super::DWORD,
    pub NumberTransportElements: super::WORD,
    pub NumberStorageElements: super::WORD,
    pub NumberCleanerSlots: super::WORD,
    pub NumberIEElements: super::WORD,
    pub NumberDataTransferElements: super::WORD,
    pub NumberOfDoors: super::WORD,
    pub FirstSlotNumber: super::WORD,
    pub FirstDriveNumber: super::WORD,
    pub FirstTransportNumber: super::WORD,
    pub FirstIEPortNumber: super::WORD,
    pub FirstCleanerSlotAddress: super::WORD,
    pub MagazineSize: super::WORD,
    pub DriveCleanTimeout: super::DWORD,
    pub Features0: super::DWORD,
    pub Features1: super::DWORD,
    pub MoveFromTransport: super::BYTE,
    pub MoveFromSlot: super::BYTE,
    pub MoveFromIePort: super::BYTE,
    pub MoveFromDrive: super::BYTE,
    pub ExchangeFromTransport: super::BYTE,
    pub ExchangeFromSlot: super::BYTE,
    pub ExchangeFromIePort: super::BYTE,
    pub ExchangeFromDrive: super::BYTE,
    pub LockUnlockCapabilities: super::BYTE,
    pub PositionCapabilities: super::BYTE,
    pub Reserved1: [super::BYTE; 2],
    pub Reserved2: [super::DWORD; 2],
}
#[cfg(feature = "shared")]
impl Default for GET_CHANGER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub RequestDataType: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE,
    pub RequestDataSet: DEVICE_INTERNAL_STATUS_DATA_SET,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_DISK_ATTRIBUTES {
    pub Version: super::DWORD,
    pub Reserved1: super::DWORD,
    pub Attributes: super::DWORDLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_FILTER_FILE_IDENTIFIER_INPUT {
    pub AltitudeLength: super::WORD,
    pub Altitude: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    pub FilterFileIdentifierLength: super::WORD,
    pub FilterFileIdentifier: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct GET_LENGTH_INFORMATION {
    pub Length: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for GET_LENGTH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct GET_MEDIA_TYPES {
    pub DeviceType: super::DWORD,
    pub MediaInfoCount: super::DWORD,
    pub MediaInfo: [DEVICE_MEDIA_INFO; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for GET_MEDIA_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GET_VOLUME_BITMAP_FLAG_MASK_METADATA: i32 = 1;
pub const GPT_ATTRIBUTE_LEGACY_BIOS_BOOTABLE: i32 = 4;
pub const GPT_ATTRIBUTE_NO_BLOCK_IO_PROTOCOL: i32 = 2;
pub const GPT_ATTRIBUTE_PLATFORM_REQUIRED: i32 = 1;
pub const GPT_BASIC_DATA_ATTRIBUTE_DAX: i64 = 288230376151711744;
pub const GPT_BASIC_DATA_ATTRIBUTE_HIDDEN: i64 = 4611686018427387904;
pub const GPT_BASIC_DATA_ATTRIBUTE_NO_DRIVE_LETTER: u64 = 9223372036854775808;
pub const GPT_BASIC_DATA_ATTRIBUTE_OFFLINE: i64 = 576460752303423488;
pub const GPT_BASIC_DATA_ATTRIBUTE_READ_ONLY: i64 = 1152921504606846976;
pub const GPT_BASIC_DATA_ATTRIBUTE_SERVICE: i64 = 144115188075855872;
pub const GPT_BASIC_DATA_ATTRIBUTE_SHADOW_COPY: i64 = 2305843009213693952;
pub const GPT_SPACES_ATTRIBUTE_NO_METADATA: u64 = 9223372036854775808;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GP_LOG_PAGE_DESCRIPTOR {
    pub LogAddress: super::WORD,
    pub LogSectors: super::WORD,
}
pub const GUID_DEVINTERFACE_SCM_PHYSICAL_DEVICE: windows_core::GUID = windows_core::GUID::from_u128(0x4283609d_4dc2_43be_bbb4_4f15dfce2c61);
pub const GUID_SCM_PD_HEALTH_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x9da2d386_72f5_4ee3_8155_eca0678e3b06);
pub const GUID_SCM_PD_PASSTHROUGH_INVDIMM: windows_core::GUID = windows_core::GUID::from_u128(0x4309ac30_0d11_11e4_9191_0800200c9a66);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HISTOGRAM_BUCKET {
    pub Reads: super::DWORD,
    pub Writes: super::DWORD,
}
#[cfg(target_arch = "x86")]
pub const HISTOGRAM_BUCKET_SIZE: u32 = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const HISTOGRAM_BUCKET_SIZE: u64 = 8;
pub const HIST_NO_OF_BUCKETS: i32 = 24;
pub const HITACHI_12_WO: STORAGE_MEDIA_TYPE = 68;
pub const HealthStatusDisabled: STORAGE_COMPONENT_HEALTH_STATUS = 4;
pub const HealthStatusFailed: STORAGE_COMPONENT_HEALTH_STATUS = 5;
pub const HealthStatusNormal: STORAGE_COMPONENT_HEALTH_STATUS = 1;
pub const HealthStatusThrottled: STORAGE_COMPONENT_HEALTH_STATUS = 2;
pub const HealthStatusUnknown: STORAGE_COMPONENT_HEALTH_STATUS = 0;
pub const HealthStatusWarning: STORAGE_COMPONENT_HEALTH_STATUS = 3;
pub const IBM_3480: STORAGE_MEDIA_TYPE = 41;
pub const IBM_3490E: STORAGE_MEDIA_TYPE = 42;
pub const IBM_Magstar_3590: STORAGE_MEDIA_TYPE = 43;
pub const IBM_Magstar_MP: STORAGE_MEDIA_TYPE = 44;
pub const IDENTIFY_BUFFER_SIZE: i32 = 512;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IDEREGS {
    pub bFeaturesReg: super::BYTE,
    pub bSectorCountReg: super::BYTE,
    pub bSectorNumberReg: super::BYTE,
    pub bCylLowReg: super::BYTE,
    pub bCylHighReg: super::BYTE,
    pub bDriveHeadReg: super::BYTE,
    pub bCommandReg: super::BYTE,
    pub bReserved: super::BYTE,
}
pub const ID_CMD: i32 = 236;
pub const IOCTL_CHANGER_BASE: i32 = 48;
pub const IOCTL_CHANGER_EXCHANGE_MEDIUM: i32 = 3162144;
pub const IOCTL_CHANGER_GET_ELEMENT_STATUS: i32 = 3194900;
pub const IOCTL_CHANGER_GET_PARAMETERS: i32 = 3162112;
pub const IOCTL_CHANGER_GET_PRODUCT_DATA: i32 = 3162120;
pub const IOCTL_CHANGER_GET_STATUS: i32 = 3162116;
pub const IOCTL_CHANGER_INITIALIZE_ELEMENT_STATUS: i32 = 3162136;
pub const IOCTL_CHANGER_MOVE_MEDIUM: i32 = 3162148;
pub const IOCTL_CHANGER_QUERY_VOLUME_TAGS: i32 = 3194924;
pub const IOCTL_CHANGER_REINITIALIZE_TRANSPORT: i32 = 3162152;
pub const IOCTL_CHANGER_SET_ACCESS: i32 = 3194896;
pub const IOCTL_CHANGER_SET_POSITION: i32 = 3162140;
pub const IOCTL_DISK_BASE: i32 = 7;
pub const IOCTL_DISK_CHECK_VERIFY: i32 = 477184;
pub const IOCTL_DISK_CONTROLLER_NUMBER: i32 = 458820;
pub const IOCTL_DISK_CREATE_DISK: i32 = 507992;
pub const IOCTL_DISK_DELETE_DRIVE_LAYOUT: i32 = 508160;
pub const IOCTL_DISK_EJECT_MEDIA: i32 = 477192;
pub const IOCTL_DISK_FIND_NEW_DEVICES: i32 = 477208;
pub const IOCTL_DISK_FORMAT_DRIVE: i32 = 508876;
pub const IOCTL_DISK_FORMAT_TRACKS: i32 = 507928;
pub const IOCTL_DISK_FORMAT_TRACKS_EX: i32 = 507948;
pub const IOCTL_DISK_GET_CACHE_INFORMATION: i32 = 475348;
pub const IOCTL_DISK_GET_DISK_ATTRIBUTES: i32 = 458992;
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY: i32 = 458752;
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY_EX: i32 = 458912;
pub const IOCTL_DISK_GET_DRIVE_LAYOUT: i32 = 475148;
pub const IOCTL_DISK_GET_DRIVE_LAYOUT_EX: i32 = 458832;
pub const IOCTL_DISK_GET_LENGTH_INFO: i32 = 475228;
pub const IOCTL_DISK_GET_MEDIA_TYPES: i32 = 461824;
pub const IOCTL_DISK_GET_PARTITION_INFO: i32 = 475140;
pub const IOCTL_DISK_GET_PARTITION_INFO_EX: i32 = 458824;
pub const IOCTL_DISK_GROW_PARTITION: i32 = 508112;
pub const IOCTL_DISK_HISTOGRAM_DATA: i32 = 458804;
pub const IOCTL_DISK_HISTOGRAM_RESET: i32 = 458808;
pub const IOCTL_DISK_HISTOGRAM_STRUCTURE: i32 = 458800;
pub const IOCTL_DISK_IS_WRITABLE: i32 = 458788;
pub const IOCTL_DISK_LOAD_MEDIA: i32 = 477196;
pub const IOCTL_DISK_LOGGING: i32 = 458792;
pub const IOCTL_DISK_MEDIA_REMOVAL: i32 = 477188;
pub const IOCTL_DISK_PERFORMANCE: i32 = 458784;
pub const IOCTL_DISK_PERFORMANCE_OFF: i32 = 458848;
pub const IOCTL_DISK_REASSIGN_BLOCKS: i32 = 507932;
pub const IOCTL_DISK_REASSIGN_BLOCKS_EX: i32 = 508068;
pub const IOCTL_DISK_RELEASE: i32 = 477204;
pub const IOCTL_DISK_REQUEST_DATA: i32 = 458816;
pub const IOCTL_DISK_REQUEST_STRUCTURE: i32 = 458812;
pub const IOCTL_DISK_RESERVE: i32 = 477200;
pub const IOCTL_DISK_RESET_SNAPSHOT_INFO: i32 = 508432;
pub const IOCTL_DISK_SENSE_DEVICE: i32 = 459744;
pub const IOCTL_DISK_SET_CACHE_INFORMATION: i32 = 508120;
pub const IOCTL_DISK_SET_DISK_ATTRIBUTES: i32 = 508148;
pub const IOCTL_DISK_SET_DRIVE_LAYOUT: i32 = 507920;
pub const IOCTL_DISK_SET_DRIVE_LAYOUT_EX: i32 = 507988;
pub const IOCTL_DISK_SET_PARTITION_INFO: i32 = 507912;
pub const IOCTL_DISK_SET_PARTITION_INFO_EX: i32 = 507980;
pub const IOCTL_DISK_UPDATE_DRIVE_SIZE: i32 = 508104;
pub const IOCTL_DISK_UPDATE_PROPERTIES: i32 = 459072;
pub const IOCTL_DISK_VERIFY: i32 = 458772;
pub const IOCTL_SCMBUS_BASE: i32 = 89;
pub const IOCTL_SCMBUS_DEVICE_FUNCTION_BASE: i32 = 0;
pub const IOCTL_SCM_BUS_GET_LOGICAL_DEVICES: i32 = 5832704;
pub const IOCTL_SCM_BUS_GET_PHYSICAL_DEVICES: i32 = 5832708;
pub const IOCTL_SCM_BUS_GET_REGIONS: i32 = 5832712;
pub const IOCTL_SCM_BUS_QUERY_PROPERTY: i32 = 5832716;
pub const IOCTL_SCM_BUS_REFRESH_NAMESPACE: i32 = 5832728;
pub const IOCTL_SCM_BUS_RUNTIME_FW_ACTIVATE: i32 = 5865488;
pub const IOCTL_SCM_BUS_SET_PROPERTY: i32 = 5865492;
pub const IOCTL_SCM_LD_GET_INTERLEAVE_SET: i32 = 5835776;
pub const IOCTL_SCM_LOGICAL_DEVICE_FUNCTION_BASE: i32 = 768;
pub const IOCTL_SCM_PD_FIRMWARE_ACTIVATE: i32 = 5871624;
pub const IOCTL_SCM_PD_FIRMWARE_DOWNLOAD: i32 = 5871620;
pub const IOCTL_SCM_PD_PASSTHROUGH: i32 = 5888012;
pub const IOCTL_SCM_PD_QUERY_PROPERTY: i32 = 5838848;
pub const IOCTL_SCM_PD_REINITIALIZE_MEDIA: i32 = 5871636;
pub const IOCTL_SCM_PD_SET_PROPERTY: i32 = 5871640;
pub const IOCTL_SCM_PD_UPDATE_MANAGEMENT_STATUS: i32 = 5838864;
pub const IOCTL_SCM_PHYSICAL_DEVICE_FUNCTION_BASE: i32 = 1536;
pub const IOCTL_STORAGE_ALLOCATE_BC_STREAM: i32 = 3004420;
pub const IOCTL_STORAGE_ATTRIBUTE_MANAGEMENT: i32 = 3005596;
pub const IOCTL_STORAGE_BASE: i32 = 45;
pub const IOCTL_STORAGE_BC_VERSION: i32 = 1;
pub const IOCTL_STORAGE_BOOT_PARTITION_ACTIVATE: i32 = 3005456;
pub const IOCTL_STORAGE_BOOT_PARTITION_DOWNLOAD: i32 = 3005452;
pub const IOCTL_STORAGE_BOOT_PARTITION_GET_INFO: i32 = 2956308;
pub const IOCTL_STORAGE_BREAK_RESERVATION: i32 = 2969620;
pub const IOCTL_STORAGE_CHECK_PRIORITY_HINT_SUPPORT: i32 = 2955392;
pub const IOCTL_STORAGE_CHECK_VERIFY: i32 = 2967552;
pub const IOCTL_STORAGE_CHECK_VERIFY2: i32 = 2951168;
pub const IOCTL_STORAGE_DEVICE_POWER_CAP: i32 = 2956436;
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_NOTIFY: i32 = 3002820;
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_QUERY_CAPS: i32 = 3002824;
pub const IOCTL_STORAGE_DIAGNOSTIC: i32 = 2956448;
pub const IOCTL_STORAGE_EJECTION_CONTROL: i32 = 2951488;
pub const IOCTL_STORAGE_EJECT_MEDIA: i32 = 2967560;
pub const IOCTL_STORAGE_ENABLE_IDLE_POWER: i32 = 2956416;
pub const IOCTL_STORAGE_EVENT_NOTIFICATION: i32 = 2956432;
pub const IOCTL_STORAGE_FAILURE_PREDICTION_CONFIG: i32 = 2953476;
pub const IOCTL_STORAGE_FIND_NEW_DEVICES: i32 = 2967576;
pub const IOCTL_STORAGE_FIRMWARE_ACTIVATE: i32 = 3005448;
pub const IOCTL_STORAGE_FIRMWARE_DOWNLOAD: i32 = 3005444;
pub const IOCTL_STORAGE_FIRMWARE_GET_INFO: i32 = 2956288;
pub const IOCTL_STORAGE_FREE_BC_STREAM: i32 = 3004424;
pub const IOCTL_STORAGE_GET_BC_PROPERTIES: i32 = 2971648;
pub const IOCTL_STORAGE_GET_COUNTERS: i32 = 2953480;
pub const IOCTL_STORAGE_GET_DEVICE_INTERNAL_LOG: i32 = 2956484;
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER: i32 = 2953344;
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER_EX: i32 = 2953348;
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY: i32 = 3002816;
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY_RAW: i32 = 3002828;
pub const IOCTL_STORAGE_GET_HOTPLUG_INFO: i32 = 2952212;
pub const IOCTL_STORAGE_GET_IDLE_POWERUP_REASON: i32 = 2956420;
pub const IOCTL_STORAGE_GET_LB_PROVISIONING_MAP_RESOURCES: i32 = 2970632;
pub const IOCTL_STORAGE_GET_MEDIA_SERIAL_NUMBER: i32 = 2952208;
pub const IOCTL_STORAGE_GET_MEDIA_TYPES: i32 = 2952192;
pub const IOCTL_STORAGE_GET_MEDIA_TYPES_EX: i32 = 2952196;
pub const IOCTL_STORAGE_GET_PHYSICAL_ELEMENT_STATUS: i32 = 2956452;
pub const IOCTL_STORAGE_GET_SYSTEM_FEATURE_SUPPORT: i32 = 2968604;
pub const IOCTL_STORAGE_LOAD_MEDIA: i32 = 2967564;
pub const IOCTL_STORAGE_LOAD_MEDIA2: i32 = 2951180;
pub const IOCTL_STORAGE_MANAGE_DATA_SET_ATTRIBUTES: i32 = 2987012;
pub const IOCTL_STORAGE_MCN_CONTROL: i32 = 2951492;
pub const IOCTL_STORAGE_MEDIA_REMOVAL: i32 = 2967556;
pub const IOCTL_STORAGE_MINIPORT_PASSTHROUGH_REQUEST: i32 = 3002448;
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_IN: i32 = 2969624;
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_OUT: i32 = 3002396;
pub const IOCTL_STORAGE_POWER_ACTIVE: i32 = 2956424;
pub const IOCTL_STORAGE_POWER_IDLE: i32 = 2956428;
pub const IOCTL_STORAGE_PREDICT_FAILURE: i32 = 2953472;
pub const IOCTL_STORAGE_PROTOCOL_COMMAND: i32 = 3003328;
pub const IOCTL_STORAGE_QUERY_PROPERTY: i32 = 2954240;
pub const IOCTL_STORAGE_READ_CAPACITY: i32 = 2969920;
pub const IOCTL_STORAGE_REINITIALIZE_MEDIA: i32 = 2987584;
pub const IOCTL_STORAGE_RELEASE: i32 = 2967572;
pub const IOCTL_STORAGE_REMOVE_ELEMENT_AND_TRUNCATE: i32 = 2956480;
pub const IOCTL_STORAGE_RESERVE: i32 = 2967568;
pub const IOCTL_STORAGE_RESET_BUS: i32 = 2969600;
pub const IOCTL_STORAGE_RESET_DEVICE: i32 = 2969604;
pub const IOCTL_STORAGE_RPMB_COMMAND: i32 = 2956440;
pub const IOCTL_STORAGE_SET_HOTPLUG_INFO: i32 = 3001368;
pub const IOCTL_STORAGE_SET_PROPERTY: i32 = 2987004;
pub const IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD: i32 = 3002880;
pub const IOCTL_STORAGE_START_DATA_INTEGRITY_CHECK: i32 = 3004548;
pub const IOCTL_STORAGE_STOP_DATA_INTEGRITY_CHECK: i32 = 3004552;
pub const IOCTL_VOLUME_BASE: i32 = 86;
pub const IOCTL_VOLUME_GET_GPT_ATTRIBUTES: i32 = 5636152;
pub const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: i32 = 5636096;
pub const IOCTL_VOLUME_IS_CLUSTERED: i32 = 5636144;
pub const IOCTL_VOLUME_OFFLINE: i32 = 5685260;
pub const IOCTL_VOLUME_ONLINE: i32 = 5685256;
pub const IOMEGA_JAZ: STORAGE_MEDIA_TYPE = 74;
pub const IOMEGA_ZIP: STORAGE_MEDIA_TYPE = 73;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default)]
pub struct IO_IRP_EXT_TRACK_OFFSET_HEADER {
    pub Validation: super::WORD,
    pub Flags: super::WORD,
    pub TrackedOffsetCallback: PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK,
}
pub const IRP_EXT_TRACK_OFFSET_HEADER_VALIDATION_VALUE: i32 = 21583;
pub const KODAK_14_WO: STORAGE_MEDIA_TYPE = 70;
pub const KeepPrefetchedData: DISK_CACHE_RETENTION_PRIORITY = 1;
pub const KeepReadData: DISK_CACHE_RETENTION_PRIORITY = 2;
pub const LMRQuerySessionInfo: LMR_QUERY_INFO_CLASS = 1;
pub type LMR_QUERY_INFO_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LMR_QUERY_INFO_PARAM {
    pub Operation: LMR_QUERY_INFO_CLASS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LMR_QUERY_SESSION_INFO {
    pub SessionId: super::UINT64,
}
pub const LOCK_ELEMENT: i32 = 0;
pub const LOCK_UNLOCK_DOOR: i32 = 2;
pub const LOCK_UNLOCK_IEPORT: i32 = 1;
pub const LOCK_UNLOCK_KEYPAD: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    pub OffsetToNext: super::DWORD,
    pub Flags: super::DWORD,
    pub Reserved: super::LARGE_INTEGER,
    pub Cluster: super::LARGE_INTEGER,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_DATA: i32 = 16777216;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_INDEX: i32 = 33554432;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_MASK: u32 = 4278190080;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_SYSTEM: i32 = 50331648;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_DENY_DEFRAG_SET: i32 = 2;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_FS_SYSTEM_FILE: i32 = 4;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_PAGE_FILE: i32 = 1;
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_TXF_SYSTEM_FILE: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    pub Flags: super::DWORD,
    pub NumberOfClusters: super::DWORD,
    pub Cluster: [super::LARGE_INTEGER; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    pub Offset: super::DWORD,
    pub NumberOfMatches: super::DWORD,
    pub BufferSizeRequired: super::DWORD,
}
#[cfg(feature = "shared")]
pub type LPDRIVERSTATUS = *mut DRIVERSTATUS;
#[cfg(feature = "shared")]
pub type LPGETVERSIONINPARAMS = *mut GETVERSIONINPARAMS;
#[cfg(feature = "shared")]
pub type LPIDEREGS = *mut IDEREGS;
#[cfg(feature = "shared")]
pub type LPSENDCMDINPARAMS = *mut SENDCMDINPARAMS;
#[cfg(feature = "shared")]
pub type LPSENDCMDOUTPARAMS = *mut SENDCMDOUTPARAMS;
pub const LTO_Accelis: STORAGE_MEDIA_TYPE = 87;
pub const LTO_Ultrium: STORAGE_MEDIA_TYPE = 86;
pub const MARK_HANDLE_CLOUD_SYNC: i32 = 2048;
pub const MARK_HANDLE_DISABLE_FILE_METADATA_OPTIMIZATION: i32 = 4096;
pub const MARK_HANDLE_ENABLE_CPU_CACHE: i32 = 268435456;
pub const MARK_HANDLE_ENABLE_USN_SOURCE_ON_PAGING_IO: i32 = 8192;
pub const MARK_HANDLE_FILTER_METADATA: i32 = 512;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MARK_HANDLE_INFO {
    pub Anonymous: MARK_HANDLE_INFO_0,
    pub VolumeHandle: super::HANDLE,
    pub HandleInfo: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MARK_HANDLE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MARK_HANDLE_INFO_0 {
    pub UsnSourceInfo: super::DWORD,
    pub CopyNumber: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MARK_HANDLE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MARK_HANDLE_INFO32 {
    pub Anonymous: MARK_HANDLE_INFO32_0,
    pub VolumeHandle: super::UINT32,
    pub HandleInfo: super::DWORD,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for MARK_HANDLE_INFO32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MARK_HANDLE_INFO32_0 {
    pub UsnSourceInfo: super::DWORD,
    pub CopyNumber: super::DWORD,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for MARK_HANDLE_INFO32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MARK_HANDLE_NOT_READ_COPY: i32 = 256;
pub const MARK_HANDLE_NOT_REALTIME: i32 = 64;
pub const MARK_HANDLE_NOT_TXF_SYSTEM_LOG: i32 = 8;
pub const MARK_HANDLE_PROTECT_CLUSTERS: i32 = 1;
pub const MARK_HANDLE_READ_COPY: i32 = 128;
pub const MARK_HANDLE_REALTIME: i32 = 32;
pub const MARK_HANDLE_RETURN_PURGE_FAILURE: i32 = 1024;
pub const MARK_HANDLE_SKIP_COHERENCY_SYNC_DISALLOW_WRITES: i32 = 16384;
pub const MARK_HANDLE_SUPPRESS_VOLUME_OPEN_FLUSH: i32 = 32768;
pub const MARK_HANDLE_TXF_SYSTEM_LOG: i32 = 4;
pub const MAXIMUM_ENCRYPTION_VALUE: i32 = 4;
pub const MAX_FW_BUCKET_ID_LENGTH: i32 = 132;
pub const MAX_INTERFACE_CODES: i32 = 8;
pub const MAX_VOLUME_ID_SIZE: i32 = 36;
pub const MAX_VOLUME_TEMPLATE_SIZE: i32 = 40;
pub const MEDIA_CURRENTLY_MOUNTED: u32 = 2147483648;
pub const MEDIA_ERASEABLE: i32 = 1;
pub const MEDIA_READ_ONLY: i32 = 4;
pub const MEDIA_READ_WRITE: i32 = 8;
pub type MEDIA_TYPE = i32;
pub const MEDIA_WRITE_ONCE: i32 = 2;
pub const MEDIA_WRITE_PROTECTED: i32 = 256;
pub const METHOD_BUFFERED: i32 = 0;
pub const METHOD_DIRECT_FROM_HARDWARE: i32 = 2;
pub const METHOD_DIRECT_TO_HARDWARE: i32 = 1;
pub const METHOD_IN_DIRECT: i32 = 1;
pub const METHOD_NEITHER: i32 = 3;
pub const METHOD_OUT_DIRECT: i32 = 2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type MFT_ENUM_DATA = MFT_ENUM_DATA_V1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_ENUM_DATA_V0 {
    pub StartFileReferenceNumber: super::DWORDLONG,
    pub LowUsn: super::USN,
    pub HighUsn: super::USN,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_ENUM_DATA_V1 {
    pub StartFileReferenceNumber: super::DWORDLONG,
    pub LowUsn: super::USN,
    pub HighUsn: super::USN,
    pub MinMajorVersion: super::WORD,
    pub MaxMajorVersion: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MOVE_FILE_DATA {
    pub FileHandle: super::HANDLE,
    pub StartingVcn: super::LARGE_INTEGER,
    pub StartingLcn: super::LARGE_INTEGER,
    pub ClusterCount: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MOVE_FILE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MOVE_FILE_DATA32 {
    pub FileHandle: super::UINT32,
    pub StartingVcn: super::LARGE_INTEGER,
    pub StartingLcn: super::LARGE_INTEGER,
    pub ClusterCount: super::DWORD,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MOVE_FILE_DATA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MOVE_FILE_RECORD_DATA {
    pub FileHandle: super::HANDLE,
    pub SourceFileRecord: super::LARGE_INTEGER,
    pub TargetFileRecord: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MOVE_FILE_RECORD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MO_3_RW: STORAGE_MEDIA_TYPE = 57;
pub const MO_5_LIMDOW: STORAGE_MEDIA_TYPE = 60;
pub const MO_5_RW: STORAGE_MEDIA_TYPE = 59;
pub const MO_5_WO: STORAGE_MEDIA_TYPE = 58;
pub const MO_NFR_525: STORAGE_MEDIA_TYPE = 71;
pub const MP2_8mm: STORAGE_MEDIA_TYPE = 79;
pub const MP_8mm: STORAGE_MEDIA_TYPE = 36;
pub const MiniQic: STORAGE_MEDIA_TYPE = 33;
pub const NCTP: STORAGE_MEDIA_TYPE = 40;
pub const NIKON_12_RW: STORAGE_MEDIA_TYPE = 72;
#[cfg(feature = "shared")]
pub const NO_SRBTYPE_ADAPTER_DESCRIPTOR_SIZE: super::DWORD = 30;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_EXTENDED_VOLUME_DATA {
    pub ByteCount: super::DWORD,
    pub MajorVersion: super::WORD,
    pub MinorVersion: super::WORD,
    pub BytesPerPhysicalSector: super::DWORD,
    pub LfsMajorVersion: super::WORD,
    pub LfsMinorVersion: super::WORD,
    pub MaxDeviceTrimExtentCount: super::DWORD,
    pub MaxDeviceTrimByteCount: super::DWORD,
    pub MaxVolumeTrimExtentCount: super::DWORD,
    pub MaxVolumeTrimByteCount: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct NTFS_FILE_RECORD_INPUT_BUFFER {
    pub FileReferenceNumber: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct NTFS_FILE_RECORD_OUTPUT_BUFFER {
    pub FileReferenceNumber: super::LARGE_INTEGER,
    pub FileRecordLength: super::DWORD,
    pub FileRecordBuffer: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS {
    pub LogFileFullExceptions: super::DWORD,
    pub OtherExceptions: super::DWORD,
    pub MftReads: super::DWORD,
    pub MftReadBytes: super::DWORD,
    pub MftWrites: super::DWORD,
    pub MftWriteBytes: super::DWORD,
    pub MftWritesUserLevel: NTFS_STATISTICS_0,
    pub MftWritesFlushForLogFileFull: super::WORD,
    pub MftWritesLazyWriter: super::WORD,
    pub MftWritesUserRequest: super::WORD,
    pub Mft2Writes: super::DWORD,
    pub Mft2WriteBytes: super::DWORD,
    pub Mft2WritesUserLevel: NTFS_STATISTICS_1,
    pub Mft2WritesFlushForLogFileFull: super::WORD,
    pub Mft2WritesLazyWriter: super::WORD,
    pub Mft2WritesUserRequest: super::WORD,
    pub RootIndexReads: super::DWORD,
    pub RootIndexReadBytes: super::DWORD,
    pub RootIndexWrites: super::DWORD,
    pub RootIndexWriteBytes: super::DWORD,
    pub BitmapReads: super::DWORD,
    pub BitmapReadBytes: super::DWORD,
    pub BitmapWrites: super::DWORD,
    pub BitmapWriteBytes: super::DWORD,
    pub BitmapWritesFlushForLogFileFull: super::WORD,
    pub BitmapWritesLazyWriter: super::WORD,
    pub BitmapWritesUserRequest: super::WORD,
    pub BitmapWritesUserLevel: NTFS_STATISTICS_2,
    pub MftBitmapReads: super::DWORD,
    pub MftBitmapReadBytes: super::DWORD,
    pub MftBitmapWrites: super::DWORD,
    pub MftBitmapWriteBytes: super::DWORD,
    pub MftBitmapWritesFlushForLogFileFull: super::WORD,
    pub MftBitmapWritesLazyWriter: super::WORD,
    pub MftBitmapWritesUserRequest: super::WORD,
    pub MftBitmapWritesUserLevel: NTFS_STATISTICS_3,
    pub UserIndexReads: super::DWORD,
    pub UserIndexReadBytes: super::DWORD,
    pub UserIndexWrites: super::DWORD,
    pub UserIndexWriteBytes: super::DWORD,
    pub LogFileReads: super::DWORD,
    pub LogFileReadBytes: super::DWORD,
    pub LogFileWrites: super::DWORD,
    pub LogFileWriteBytes: super::DWORD,
    pub Allocate: NTFS_STATISTICS_4,
    pub DiskResourcesExhausted: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_0 {
    pub Write: super::WORD,
    pub Create: super::WORD,
    pub SetInfo: super::WORD,
    pub Flush: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_1 {
    pub Write: super::WORD,
    pub Create: super::WORD,
    pub SetInfo: super::WORD,
    pub Flush: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_2 {
    pub Write: super::WORD,
    pub Create: super::WORD,
    pub SetInfo: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_3 {
    pub Write: super::WORD,
    pub Create: super::WORD,
    pub SetInfo: super::WORD,
    pub Flush: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_4 {
    pub Calls: super::DWORD,
    pub Clusters: super::DWORD,
    pub Hints: super::DWORD,
    pub RunsReturned: super::DWORD,
    pub HintsHonored: super::DWORD,
    pub HintsClusters: super::DWORD,
    pub Cache: super::DWORD,
    pub CacheClusters: super::DWORD,
    pub CacheMiss: super::DWORD,
    pub CacheMissClusters: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_EX {
    pub LogFileFullExceptions: super::DWORD,
    pub OtherExceptions: super::DWORD,
    pub MftReads: super::DWORDLONG,
    pub MftReadBytes: super::DWORDLONG,
    pub MftWrites: super::DWORDLONG,
    pub MftWriteBytes: super::DWORDLONG,
    pub MftWritesUserLevel: NTFS_STATISTICS_EX_0,
    pub MftWritesFlushForLogFileFull: super::DWORD,
    pub MftWritesLazyWriter: super::DWORD,
    pub MftWritesUserRequest: super::DWORD,
    pub Mft2Writes: super::DWORDLONG,
    pub Mft2WriteBytes: super::DWORDLONG,
    pub Mft2WritesUserLevel: NTFS_STATISTICS_EX_1,
    pub Mft2WritesFlushForLogFileFull: super::DWORD,
    pub Mft2WritesLazyWriter: super::DWORD,
    pub Mft2WritesUserRequest: super::DWORD,
    pub RootIndexReads: super::DWORDLONG,
    pub RootIndexReadBytes: super::DWORDLONG,
    pub RootIndexWrites: super::DWORDLONG,
    pub RootIndexWriteBytes: super::DWORDLONG,
    pub BitmapReads: super::DWORDLONG,
    pub BitmapReadBytes: super::DWORDLONG,
    pub BitmapWrites: super::DWORDLONG,
    pub BitmapWriteBytes: super::DWORDLONG,
    pub BitmapWritesFlushForLogFileFull: super::DWORD,
    pub BitmapWritesLazyWriter: super::DWORD,
    pub BitmapWritesUserRequest: super::DWORD,
    pub BitmapWritesUserLevel: NTFS_STATISTICS_EX_2,
    pub MftBitmapReads: super::DWORDLONG,
    pub MftBitmapReadBytes: super::DWORDLONG,
    pub MftBitmapWrites: super::DWORDLONG,
    pub MftBitmapWriteBytes: super::DWORDLONG,
    pub MftBitmapWritesFlushForLogFileFull: super::DWORD,
    pub MftBitmapWritesLazyWriter: super::DWORD,
    pub MftBitmapWritesUserRequest: super::DWORD,
    pub MftBitmapWritesUserLevel: NTFS_STATISTICS_EX_3,
    pub UserIndexReads: super::DWORDLONG,
    pub UserIndexReadBytes: super::DWORDLONG,
    pub UserIndexWrites: super::DWORDLONG,
    pub UserIndexWriteBytes: super::DWORDLONG,
    pub LogFileReads: super::DWORDLONG,
    pub LogFileReadBytes: super::DWORDLONG,
    pub LogFileWrites: super::DWORDLONG,
    pub LogFileWriteBytes: super::DWORDLONG,
    pub Allocate: NTFS_STATISTICS_EX_4,
    pub DiskResourcesExhausted: super::DWORD,
    pub VolumeTrimCount: super::DWORDLONG,
    pub VolumeTrimTime: super::DWORDLONG,
    pub VolumeTrimByteCount: super::DWORDLONG,
    pub FileLevelTrimCount: super::DWORDLONG,
    pub FileLevelTrimTime: super::DWORDLONG,
    pub FileLevelTrimByteCount: super::DWORDLONG,
    pub VolumeTrimSkippedCount: super::DWORDLONG,
    pub VolumeTrimSkippedByteCount: super::DWORDLONG,
    pub NtfsFillStatInfoFromMftRecordCalledCount: super::DWORDLONG,
    pub NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount: super::DWORDLONG,
    pub NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_EX_0 {
    pub Write: super::DWORD,
    pub Create: super::DWORD,
    pub SetInfo: super::DWORD,
    pub Flush: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_EX_1 {
    pub Write: super::DWORD,
    pub Create: super::DWORD,
    pub SetInfo: super::DWORD,
    pub Flush: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_EX_2 {
    pub Write: super::DWORD,
    pub Create: super::DWORD,
    pub SetInfo: super::DWORD,
    pub Flush: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_EX_3 {
    pub Write: super::DWORD,
    pub Create: super::DWORD,
    pub SetInfo: super::DWORD,
    pub Flush: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTFS_STATISTICS_EX_4 {
    pub Calls: super::DWORD,
    pub RunsReturned: super::DWORD,
    pub Hints: super::DWORD,
    pub HintsHonored: super::DWORD,
    pub Cache: super::DWORD,
    pub CacheMiss: super::DWORD,
    pub Clusters: super::DWORDLONG,
    pub HintsClusters: super::DWORDLONG,
    pub CacheClusters: super::DWORDLONG,
    pub CacheMissClusters: super::DWORDLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct NTFS_VOLUME_DATA_BUFFER {
    pub VolumeSerialNumber: super::LARGE_INTEGER,
    pub NumberSectors: super::LARGE_INTEGER,
    pub TotalClusters: super::LARGE_INTEGER,
    pub FreeClusters: super::LARGE_INTEGER,
    pub TotalReserved: super::LARGE_INTEGER,
    pub BytesPerSector: super::DWORD,
    pub BytesPerCluster: super::DWORD,
    pub BytesPerFileRecordSegment: super::DWORD,
    pub ClustersPerFileRecordSegment: super::DWORD,
    pub MftValidDataLength: super::LARGE_INTEGER,
    pub MftStartLcn: super::LARGE_INTEGER,
    pub Mft2StartLcn: super::LARGE_INTEGER,
    pub MftZoneStart: super::LARGE_INTEGER,
    pub MftZoneEnd: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for NTFS_VOLUME_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVMeDataTypeFeature: STORAGE_PROTOCOL_NVME_DATA_TYPE = 3;
pub const NVMeDataTypeFeatureEx: STORAGE_PROTOCOL_NVME_DATA_TYPE = 5;
pub const NVMeDataTypeIdentify: STORAGE_PROTOCOL_NVME_DATA_TYPE = 1;
pub const NVMeDataTypeLogPage: STORAGE_PROTOCOL_NVME_DATA_TYPE = 2;
pub const NVMeDataTypeLogPageEx: STORAGE_PROTOCOL_NVME_DATA_TYPE = 4;
pub const NVMeDataTypeUnknown: STORAGE_PROTOCOL_NVME_DATA_TYPE = 0;
pub const OBSOLETE_DISK_GET_WRITE_CACHE_STATE: i32 = 475356;
pub const OBSOLETE_IOCTL_STORAGE_RESET_BUS: i32 = 3002368;
pub const OBSOLETE_IOCTL_STORAGE_RESET_DEVICE: i32 = 3002372;
pub const OFFLOAD_READ_FLAG_ALL_ZERO_BEYOND_CURRENT_RANGE: i32 = 1;
pub const OPLOCK_LEVEL_CACHE_HANDLE: i32 = 2;
pub const OPLOCK_LEVEL_CACHE_READ: i32 = 1;
pub const OPLOCK_LEVEL_CACHE_WRITE: i32 = 4;
pub const PARTIITON_OS_DATA: i32 = 41;
pub const PARTITION_BSP: i32 = 43;
pub const PARTITION_DM: i32 = 84;
pub const PARTITION_DPP: i32 = 44;
pub const PARTITION_ENTRY_UNUSED: i32 = 0;
pub const PARTITION_EXTENDED: i32 = 5;
pub const PARTITION_EZDRIVE: i32 = 85;
pub const PARTITION_FAT32: i32 = 11;
pub const PARTITION_FAT32_XINT13: i32 = 12;
pub const PARTITION_FAT_12: i32 = 1;
pub const PARTITION_FAT_16: i32 = 4;
pub const PARTITION_GPT: i32 = 238;
pub const PARTITION_HUGE: i32 = 6;
pub const PARTITION_IFS: i32 = 7;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PARTITION_INFORMATION {
    pub StartingOffset: super::LARGE_INTEGER,
    pub PartitionLength: super::LARGE_INTEGER,
    pub HiddenSectors: super::DWORD,
    pub PartitionNumber: super::DWORD,
    pub PartitionType: super::BYTE,
    pub BootIndicator: super::BOOLEAN,
    pub RecognizedPartition: super::BOOLEAN,
    pub RewritePartition: super::BOOLEAN,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PARTITION_INFORMATION_EX {
    pub PartitionStyle: PARTITION_STYLE,
    pub PartitionOrdinal: super::WORD,
    pub StartingOffset: super::LARGE_INTEGER,
    pub PartitionLength: super::LARGE_INTEGER,
    pub PartitionNumber: super::DWORD,
    pub RewritePartition: super::BOOLEAN,
    pub IsServicePartition: super::BOOLEAN,
    pub Anonymous: PARTITION_INFORMATION_EX_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union PARTITION_INFORMATION_EX_0 {
    pub Mbr: PARTITION_INFORMATION_MBR,
    pub Gpt: PARTITION_INFORMATION_GPT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PARTITION_INFORMATION_GPT {
    pub PartitionType: windows_core::GUID,
    pub PartitionId: windows_core::GUID,
    pub Attributes: super::DWORD64,
    pub Name: [super::WCHAR; 36],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PARTITION_INFORMATION_GPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PARTITION_INFORMATION_MBR {
    pub PartitionType: super::BYTE,
    pub BootIndicator: super::BOOLEAN,
    pub RecognizedPartition: super::BOOLEAN,
    pub HiddenSectors: super::DWORD,
    pub PartitionId: windows_core::GUID,
}
pub const PARTITION_LDM: i32 = 66;
pub const PARTITION_MAIN_OS: i32 = 40;
pub const PARTITION_MSFT_RECOVERY: i32 = 39;
pub const PARTITION_NTFT: i32 = 128;
pub const PARTITION_OS2BOOTMGR: i32 = 10;
pub const PARTITION_PREP: i32 = 65;
pub const PARTITION_PRE_INSTALLED: i32 = 42;
pub const PARTITION_SPACES: i32 = 231;
pub const PARTITION_SPACES_DATA: i32 = 215;
pub type PARTITION_STYLE = i32;
pub const PARTITION_STYLE_GPT: PARTITION_STYLE = 1;
pub const PARTITION_STYLE_MBR: PARTITION_STYLE = 0;
pub const PARTITION_STYLE_RAW: PARTITION_STYLE = 2;
pub const PARTITION_SYSTEM: i32 = 239;
pub const PARTITION_UNIX: i32 = 99;
pub const PARTITION_WINDOWS_SYSTEM: i32 = 45;
pub const PARTITION_XENIX_1: i32 = 2;
pub const PARTITION_XENIX_2: i32 = 3;
pub const PARTITION_XINT13: i32 = 14;
pub const PARTITION_XINT13_EXTENDED: i32 = 15;
#[cfg(feature = "shared")]
pub type PASYNC_DUPLICATE_EXTENTS_STATUS = *mut ASYNC_DUPLICATE_EXTENTS_STATUS;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PATHNAME_BUFFER {
    pub PathNameLength: super::DWORD,
    pub Name: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PATHNAME_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PBAD_TRACK_NUMBER = *mut u16;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBIN_COUNT = *mut BIN_COUNT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBIN_RANGE = *mut BIN_RANGE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBIN_RESULTS = *mut BIN_RESULTS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBOOT_AREA_INFO = *mut BOOT_AREA_INFO;
#[cfg(feature = "shared")]
pub type PBULK_SECURITY_TEST_DATA = *mut BULK_SECURITY_TEST_DATA;
pub type PCHANGER_DEVICE_PROBLEM_TYPE = *mut CHANGER_DEVICE_PROBLEM_TYPE;
#[cfg(feature = "shared")]
pub type PCHANGER_ELEMENT = *mut CHANGER_ELEMENT;
#[cfg(feature = "shared")]
pub type PCHANGER_ELEMENT_LIST = *mut CHANGER_ELEMENT_LIST;
#[cfg(feature = "shared")]
pub type PCHANGER_ELEMENT_STATUS = *mut CHANGER_ELEMENT_STATUS;
#[cfg(feature = "shared")]
pub type PCHANGER_ELEMENT_STATUS_EX = *mut CHANGER_ELEMENT_STATUS_EX;
#[cfg(feature = "shared")]
pub type PCHANGER_EXCHANGE_MEDIUM = *mut CHANGER_EXCHANGE_MEDIUM;
#[cfg(feature = "shared")]
pub type PCHANGER_INITIALIZE_ELEMENT_STATUS = *mut CHANGER_INITIALIZE_ELEMENT_STATUS;
#[cfg(feature = "shared")]
pub type PCHANGER_MOVE_MEDIUM = *mut CHANGER_MOVE_MEDIUM;
#[cfg(feature = "shared")]
pub type PCHANGER_PRODUCT_DATA = *mut CHANGER_PRODUCT_DATA;
#[cfg(feature = "shared")]
pub type PCHANGER_READ_ELEMENT_STATUS = *mut CHANGER_READ_ELEMENT_STATUS;
#[cfg(feature = "shared")]
pub type PCHANGER_SEND_VOLUME_TAG_INFORMATION = *mut CHANGER_SEND_VOLUME_TAG_INFORMATION;
#[cfg(feature = "shared")]
pub type PCHANGER_SET_ACCESS = *mut CHANGER_SET_ACCESS;
#[cfg(feature = "shared")]
pub type PCHANGER_SET_POSITION = *mut CHANGER_SET_POSITION;
#[cfg(feature = "shared")]
pub type PCLASS_MEDIA_CHANGE_CONTEXT = *mut CLASS_MEDIA_CHANGE_CONTEXT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLUSTER_RANGE = *mut CLUSTER_RANGE;
#[cfg(feature = "shared")]
pub type PCONTAINER_ROOT_INFO_INPUT = *mut CONTAINER_ROOT_INFO_INPUT;
#[cfg(feature = "shared")]
pub type PCONTAINER_ROOT_INFO_OUTPUT = *mut CONTAINER_ROOT_INFO_OUTPUT;
#[cfg(feature = "shared")]
pub type PCONTAINER_VOLUME_STATE = *mut CONTAINER_VOLUME_STATE;
#[cfg(feature = "shared")]
pub type PCREATE_DISK = *mut CREATE_DISK;
#[cfg(feature = "shared")]
pub type PCREATE_DISK_GPT = *mut CREATE_DISK_GPT;
#[cfg(feature = "shared")]
pub type PCREATE_DISK_MBR = *mut CREATE_DISK_MBR;
#[cfg(feature = "shared")]
pub type PCREATE_USN_JOURNAL_DATA = *mut CREATE_USN_JOURNAL_DATA;
#[cfg(feature = "shared")]
pub type PCSMB_SHARE_FLUSH_AND_PURGE_INPUT = *const SMB_SHARE_FLUSH_AND_PURGE_INPUT;
#[cfg(feature = "shared")]
pub type PCSMB_SHARE_FLUSH_AND_PURGE_OUTPUT = *const SMB_SHARE_FLUSH_AND_PURGE_OUTPUT;
pub type PCSVFS_DISK_CONNECTIVITY = *mut CSVFS_DISK_CONNECTIVITY;
pub type PCSV_CONTROL_OP = *mut CSV_CONTROL_OP;
#[cfg(feature = "shared")]
pub type PCSV_CONTROL_PARAM = *mut CSV_CONTROL_PARAM;
#[cfg(feature = "shared")]
pub type PCSV_IS_OWNED_BY_CSVFS = *mut CSV_IS_OWNED_BY_CSVFS;
#[cfg(feature = "shared")]
pub type PCSV_MGMT_LOCK = *mut CSV_MGMT_LOCK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCSV_NAMESPACE_INFO = *mut CSV_NAMESPACE_INFO;
#[cfg(feature = "shared")]
pub type PCSV_QUERY_FILE_REVISION = *mut CSV_QUERY_FILE_REVISION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCSV_QUERY_FILE_REVISION_FILE_ID_128 = *mut CSV_QUERY_FILE_REVISION_FILE_ID_128;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCSV_QUERY_MDS_PATH = *mut CSV_QUERY_MDS_PATH;
#[cfg(feature = "shared")]
pub type PCSV_QUERY_MDS_PATH_V2 = *mut CSV_QUERY_MDS_PATH_V2;
#[cfg(feature = "shared")]
pub type PCSV_QUERY_REDIRECT_STATE = *mut CSV_QUERY_REDIRECT_STATE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT = *mut CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT;
pub type PCSV_QUERY_VOLUME_ID = *mut CSV_QUERY_VOLUME_ID;
#[cfg(feature = "shared")]
pub type PCSV_QUERY_VOLUME_REDIRECT_STATE = *mut CSV_QUERY_VOLUME_REDIRECT_STATE;
pub type PCSV_SET_VOLUME_ID = *mut CSV_SET_VOLUME_ID;
pub const PC_5_RW: STORAGE_MEDIA_TYPE = 62;
pub const PC_5_WO: STORAGE_MEDIA_TYPE = 61;
#[cfg(feature = "shared")]
pub type PDECRYPTION_STATUS_BUFFER = *mut DECRYPTION_STATUS_BUFFER;
#[cfg(feature = "shared")]
pub type PDELETE_USN_JOURNAL_DATA = *mut DELETE_USN_JOURNAL_DATA;
pub type PDEVICEDUMP_COLLECTION_TYPE = *mut DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_PRIVATE_SUBSECTION = *mut DEVICEDUMP_PRIVATE_SUBSECTION;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_PUBLIC_SUBSECTION = *mut DEVICEDUMP_PUBLIC_SUBSECTION;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_RESTRICTED_SUBSECTION = *mut DEVICEDUMP_RESTRICTED_SUBSECTION;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_SECTION_HEADER = *mut DEVICEDUMP_SECTION_HEADER;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_STORAGEDEVICE_DATA = *mut DEVICEDUMP_STORAGEDEVICE_DATA;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_STORAGESTACK_PUBLIC_DUMP = *mut DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD = *mut DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_STRUCTURE_VERSION = *mut DEVICEDUMP_STRUCTURE_VERSION;
#[cfg(feature = "shared")]
pub type PDEVICEDUMP_SUBSECTION_POINTER = *mut DEVICEDUMP_SUBSECTION_POINTER;
#[cfg(feature = "shared")]
pub type PDEVICE_COPY_OFFLOAD_DESCRIPTOR = *mut DEVICE_COPY_OFFLOAD_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_LBP_STATE_PARAMETERS = *mut _DEVICE_DATA_SET_LBP_STATE_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_LB_PROVISIONING_STATE = *mut _DEVICE_DATA_SET_LB_PROVISIONING_STATE;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 = *mut _DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_RANGE = *mut _DEVICE_DATA_SET_RANGE;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_REPAIR_OUTPUT = *mut _DEVICE_DATA_SET_REPAIR_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_REPAIR_PARAMETERS = *mut _DEVICE_DATA_SET_REPAIR_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_SCRUB_EX_OUTPUT = *mut _DEVICE_DATA_SET_SCRUB_EX_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_SCRUB_OUTPUT = *mut _DEVICE_DATA_SET_SCRUB_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT = *mut _DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_ALLOCATION_OUTPUT = *mut _DEVICE_DATA_SET_LB_PROVISIONING_STATE;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_ALLOCATION_OUTPUT2 = *mut _DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_ALLOCATION_PARAMETERS = *mut _DEVICE_DATA_SET_LBP_STATE_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_CONVERSION_OUTPUT = *mut DEVICE_DSM_CONVERSION_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_DEFINITION = *mut DEVICE_DSM_DEFINITION;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_FREE_SPACE_OUTPUT = *mut DEVICE_DSM_FREE_SPACE_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_INPUT = *mut _DEVICE_MANAGE_DATA_SET_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_LOST_QUERY_OUTPUT = *mut DEVICE_DSM_LOST_QUERY_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_LOST_QUERY_PARAMETERS = *mut DEVICE_DSM_LOST_QUERY_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_NOTIFICATION_PARAMETERS = *mut DEVICE_DSM_NOTIFICATION_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS = *mut DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_OFFLOAD_READ_PARAMETERS = *mut DEVICE_DSM_OFFLOAD_READ_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_OFFLOAD_WRITE_PARAMETERS = *mut DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_OUTPUT = *mut _DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT = *mut DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_QUERY_PREFER_LOCAL_REPAIR_OUTPUT = *mut DEVICE_DSM_QUERY_PREFER_LOCAL_REPAIR_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_RANGE = *mut _DEVICE_DATA_SET_RANGE;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_RANGE_ERROR_INFO = *mut _DEVICE_DSM_RANGE_ERROR_INFO;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_RANGE_ERROR_OUTPUT = *mut _DEVICE_DSM_RANGE_ERROR_INFO;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_REPAIR_OUTPUT = *mut _DEVICE_DATA_SET_REPAIR_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_REPAIR_PARAMETERS = *mut _DEVICE_DATA_SET_REPAIR_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_REPORT_ZONES_DATA = *mut _DEVICE_DSM_REPORT_ZONES_DATA;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_REPORT_ZONES_OUTPUT = *mut _DEVICE_DSM_REPORT_ZONES_DATA;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_REPORT_ZONES_PARAMETERS = *mut DEVICE_DSM_REPORT_ZONES_PARAMETERS;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_SCRUB_OUTPUT = *mut _DEVICE_DATA_SET_SCRUB_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_SCRUB_OUTPUT2 = *mut _DEVICE_DATA_SET_SCRUB_EX_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_TIERING_QUERY_INPUT = *mut _DEVICE_DSM_TIERING_QUERY_INPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_TIERING_QUERY_OUTPUT = *mut DEVICE_DSM_TIERING_QUERY_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_TIERING_QUERY_PARAMETERS = *mut _DEVICE_DSM_TIERING_QUERY_INPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_DSM_TOPOLOGY_ID_QUERY_OUTPUT = *mut _DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT;
#[cfg(feature = "shared")]
pub type PDEVICE_INTERNAL_STATUS_DATA = *mut DEVICE_INTERNAL_STATUS_DATA;
pub type PDEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = *mut DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE;
pub type PDEVICE_INTERNAL_STATUS_DATA_SET = *mut DEVICE_INTERNAL_STATUS_DATA_SET;
#[cfg(feature = "shared")]
pub type PDEVICE_LB_PROVISIONING_DESCRIPTOR = *mut DEVICE_LB_PROVISIONING_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PDEVICE_LOCATION = *mut DEVICE_LOCATION;
#[cfg(feature = "shared")]
pub type PDEVICE_MANAGE_DATA_SET_ATTRIBUTES = *mut _DEVICE_MANAGE_DATA_SET_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PDEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT = *mut _DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDEVICE_MEDIA_INFO = *mut DEVICE_MEDIA_INFO;
#[cfg(feature = "shared")]
pub type PDEVICE_POWER_DESCRIPTOR = *mut DEVICE_POWER_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PDEVICE_SEEK_PENALTY_DESCRIPTOR = *mut DEVICE_SEEK_PENALTY_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PDEVICE_STORAGE_ADDRESS_RANGE = *mut DEVICE_STORAGE_ADDRESS_RANGE;
#[cfg(feature = "shared")]
pub type PDEVICE_STORAGE_RANGE_ATTRIBUTES = *mut DEVICE_STORAGE_RANGE_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PDEVICE_TRIM_DESCRIPTOR = *mut DEVICE_TRIM_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PDEVICE_WRITE_AGGREGATION_DESCRIPTOR = *mut DEVICE_WRITE_AGGREGATION_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PDISK_CACHE_INFORMATION = *mut DISK_CACHE_INFORMATION;
#[cfg(feature = "shared")]
pub type PDISK_CONTROLLER_NUMBER = *mut DISK_CONTROLLER_NUMBER;
#[cfg(feature = "shared")]
pub type PDISK_DETECTION_INFO = *mut DISK_DETECTION_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISK_EXTENT = *mut DISK_EXTENT;
#[cfg(feature = "shared")]
pub type PDISK_EX_INT13_INFO = *mut DISK_EX_INT13_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISK_GEOMETRY = *mut DISK_GEOMETRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISK_GEOMETRY_EX = *mut DISK_GEOMETRY_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISK_GROW_PARTITION = *mut DISK_GROW_PARTITION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISK_HISTOGRAM = *mut DISK_HISTOGRAM;
#[cfg(feature = "shared")]
pub type PDISK_INT13_INFO = *mut DISK_INT13_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISK_LOGGING = *mut DISK_LOGGING;
#[cfg(feature = "shared")]
pub type PDISK_PARTITION_INFO = *mut DISK_PARTITION_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISK_PERFORMANCE = *mut DISK_PERFORMANCE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISK_RECORD = *mut DISK_RECORD;
#[cfg(feature = "shared")]
pub type PDRIVERSTATUS = *mut DRIVERSTATUS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDRIVE_LAYOUT_INFORMATION = *mut DRIVE_LAYOUT_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDRIVE_LAYOUT_INFORMATION_EX = *mut DRIVE_LAYOUT_INFORMATION_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDRIVE_LAYOUT_INFORMATION_GPT = *mut DRIVE_LAYOUT_INFORMATION_GPT;
#[cfg(feature = "shared")]
pub type PDRIVE_LAYOUT_INFORMATION_MBR = *mut DRIVE_LAYOUT_INFORMATION_MBR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDUPLICATE_EXTENTS_DATA = *mut DUPLICATE_EXTENTS_DATA;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDUPLICATE_EXTENTS_DATA32 = *mut DUPLICATE_EXTENTS_DATA32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDUPLICATE_EXTENTS_DATA_EX = *mut DUPLICATE_EXTENTS_DATA_EX;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
pub type PDUPLICATE_EXTENTS_DATA_EX32 = *mut DUPLICATE_EXTENTS_DATA_EX32;
pub type PDUPLICATE_EXTENTS_STATE = *mut DUPLICATE_EXTENTS_STATE;
pub const PD_5_RW: STORAGE_MEDIA_TYPE = 63;
pub type PELEMENT_TYPE = *mut ELEMENT_TYPE;
#[cfg(feature = "shared")]
pub type PENCRYPTED_DATA_INFO = *mut ENCRYPTED_DATA_INFO;
#[cfg(feature = "shared")]
pub type PENCRYPTION_BUFFER = *mut ENCRYPTION_BUFFER;
#[cfg(feature = "shared")]
pub type PENCRYPTION_KEY_CTRL_INPUT = *mut ENCRYPTION_KEY_CTRL_INPUT;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PERF_BIN {
    pub NumberOfBins: super::DWORD,
    pub TypeOfBin: super::DWORD,
    pub BinsRanges: [BIN_RANGE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PERF_BIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PERSISTENT_RESERVE_COMMAND {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Anonymous: PERSISTENT_RESERVE_COMMAND_0,
}
#[cfg(feature = "shared")]
impl Default for PERSISTENT_RESERVE_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PERSISTENT_RESERVE_COMMAND_0 {
    pub PR_IN: PERSISTENT_RESERVE_COMMAND_0_0,
    pub PR_OUT: PERSISTENT_RESERVE_COMMAND_0_1,
}
#[cfg(feature = "shared")]
impl Default for PERSISTENT_RESERVE_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PERSISTENT_RESERVE_COMMAND_0_0 {
    pub _bitfield: u8,
    pub AllocationLength: super::WORD,
}
#[cfg(feature = "shared")]
impl PERSISTENT_RESERVE_COMMAND_0_0 {
    pub fn ServiceAction(&self) -> super::BYTE {
        (self._bitfield) & 31
    }
    pub fn set_ServiceAction(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !31) | (value & 31);
    }
    pub fn Reserved1(&self) -> super::BYTE {
        self._bitfield >> 5
    }
    pub fn set_Reserved1(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PERSISTENT_RESERVE_COMMAND_0_1 {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub ParameterList: [super::BYTE; 0],
}
#[cfg(feature = "shared")]
impl PERSISTENT_RESERVE_COMMAND_0_1 {
    pub fn ServiceAction(&self) -> super::BYTE {
        (self._bitfield1) & 31
    }
    pub fn set_ServiceAction(&mut self, value: super::BYTE) {
        self._bitfield1 = (self._bitfield1 & !31) | (value & 31);
    }
    pub fn Reserved1(&self) -> super::BYTE {
        self._bitfield1 >> 5
    }
    pub fn set_Reserved1(&mut self, value: super::BYTE) {
        self._bitfield1 = (self._bitfield1 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn Type(&self) -> super::BYTE {
        (self._bitfield2) & 15
    }
    pub fn set_Type(&mut self, value: super::BYTE) {
        self._bitfield2 = (self._bitfield2 & !15) | (value & 15);
    }
    pub fn Scope(&self) -> super::BYTE {
        self._bitfield2 >> 4
    }
    pub fn set_Scope(&mut self, value: super::BYTE) {
        self._bitfield2 = (self._bitfield2 & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[cfg(feature = "shared")]
impl Default for PERSISTENT_RESERVE_COMMAND_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PERSISTENT_VOLUME_STATE_BACKED_BY_WIM: i32 = 64;
pub const PERSISTENT_VOLUME_STATE_CHKDSK_RAN_ONCE: i32 = 1024;
pub const PERSISTENT_VOLUME_STATE_CONTAINS_BACKING_WIM: i32 = 32;
pub const PERSISTENT_VOLUME_STATE_DAX_FORMATTED: i32 = 4096;
pub const PERSISTENT_VOLUME_STATE_DEV_VOLUME: i32 = 8192;
pub const PERSISTENT_VOLUME_STATE_GLOBAL_METADATA_NO_SEEK_PENALTY: i32 = 4;
pub const PERSISTENT_VOLUME_STATE_LOCAL_METADATA_NO_SEEK_PENALTY: i32 = 8;
pub const PERSISTENT_VOLUME_STATE_MODIFIED_BY_CHKDSK: i32 = 2048;
pub const PERSISTENT_VOLUME_STATE_NO_HEAT_GATHERING: i32 = 16;
pub const PERSISTENT_VOLUME_STATE_NO_WRITE_AUTO_TIERING: i32 = 128;
pub const PERSISTENT_VOLUME_STATE_REALLOCATE_ALL_DATA_WRITES: i32 = 512;
pub const PERSISTENT_VOLUME_STATE_SHORT_NAME_CREATION_DISABLED: i32 = 1;
pub const PERSISTENT_VOLUME_STATE_TRUSTED_VOLUME: i32 = 16384;
pub const PERSISTENT_VOLUME_STATE_TXF_DISABLED: i32 = 256;
pub const PERSISTENT_VOLUME_STATE_VOLUME_SCRUB_DISABLED: i32 = 2;
#[cfg(feature = "shared")]
pub type PEXFAT_STATISTICS = *mut EXFAT_STATISTICS;
#[cfg(feature = "shared")]
pub type PEXTENDED_ENCRYPTED_DATA_INFO = *mut EXTENDED_ENCRYPTED_DATA_INFO;
#[cfg(feature = "shared")]
pub type PFAT_STATISTICS = *mut FAT_STATISTICS;
#[cfg(feature = "shared")]
pub type PFILESYSTEM_STATISTICS = *mut FILESYSTEM_STATISTICS;
#[cfg(feature = "shared")]
pub type PFILESYSTEM_STATISTICS_EX = *mut FILESYSTEM_STATISTICS_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_ALLOCATED_RANGE_BUFFER = *mut FILE_ALLOCATED_RANGE_BUFFER;
#[cfg(feature = "shared")]
pub type PFILE_DESIRED_STORAGE_CLASS_INFORMATION = *mut FILE_DESIRED_STORAGE_CLASS_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_FS_PERSISTENT_VOLUME_INFORMATION = *mut FILE_FS_PERSISTENT_VOLUME_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_INITIATE_REPAIR_OUTPUT_BUFFER = *mut FILE_INITIATE_REPAIR_OUTPUT_BUFFER;
#[cfg(feature = "shared")]
pub type PFILE_LAYOUT_ENTRY = *mut FILE_LAYOUT_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_LAYOUT_INFO_ENTRY = *mut FILE_LAYOUT_INFO_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_LAYOUT_NAME_ENTRY = *mut FILE_LAYOUT_NAME_ENTRY;
#[cfg(feature = "shared")]
pub type PFILE_LEVEL_TRIM = *mut FILE_LEVEL_TRIM;
#[cfg(feature = "shared")]
pub type PFILE_LEVEL_TRIM_OUTPUT = *mut FILE_LEVEL_TRIM_OUTPUT;
#[cfg(feature = "shared")]
pub type PFILE_LEVEL_TRIM_RANGE = *mut FILE_LEVEL_TRIM_RANGE;
#[cfg(feature = "shared")]
pub type PFILE_MAKE_COMPATIBLE_BUFFER = *mut FILE_MAKE_COMPATIBLE_BUFFER;
#[cfg(feature = "shared")]
pub type PFILE_OBJECTID_BUFFER = *mut FILE_OBJECTID_BUFFER;
#[cfg(feature = "shared")]
pub type PFILE_PREFETCH = *mut FILE_PREFETCH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_PREFETCH_EX = *mut FILE_PREFETCH_EX;
#[cfg(feature = "shared")]
pub type PFILE_PROVIDER_EXTERNAL_INFO = *mut FILE_PROVIDER_EXTERNAL_INFO_V1;
#[cfg(feature = "shared")]
pub type PFILE_PROVIDER_EXTERNAL_INFO_V0 = *mut FILE_PROVIDER_EXTERNAL_INFO_V0;
#[cfg(feature = "shared")]
pub type PFILE_PROVIDER_EXTERNAL_INFO_V1 = *mut FILE_PROVIDER_EXTERNAL_INFO_V1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_QUERY_ON_DISK_VOL_INFO_BUFFER = *mut FILE_QUERY_ON_DISK_VOL_INFO_BUFFER;
#[cfg(feature = "shared")]
pub type PFILE_QUERY_SPARING_BUFFER = *mut FILE_QUERY_SPARING_BUFFER;
#[cfg(feature = "shared")]
pub type PFILE_REFERENCE_RANGE = *mut FILE_REFERENCE_RANGE;
#[cfg(feature = "shared")]
pub type PFILE_REGION_INFO = *mut FILE_REGION_INFO;
#[cfg(feature = "shared")]
pub type PFILE_REGION_INPUT = *mut FILE_REGION_INPUT;
#[cfg(feature = "shared")]
pub type PFILE_REGION_OUTPUT = *mut FILE_REGION_OUTPUT;
#[cfg(feature = "shared")]
pub type PFILE_SET_DEFECT_MGMT_BUFFER = *mut FILE_SET_DEFECT_MGMT_BUFFER;
#[cfg(feature = "shared")]
pub type PFILE_SET_SPARSE_BUFFER = *mut FILE_SET_SPARSE_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_STORAGE_TIER = *mut FILE_STORAGE_TIER;
pub type PFILE_STORAGE_TIER_CLASS = *mut FILE_STORAGE_TIER_CLASS;
pub type PFILE_STORAGE_TIER_MEDIA_TYPE = *mut FILE_STORAGE_TIER_MEDIA_TYPE;
#[cfg(feature = "shared")]
pub type PFILE_STORAGE_TIER_REGION = *mut FILE_STORAGE_TIER_REGION;
#[cfg(feature = "shared")]
pub type PFILE_SYSTEM_RECOGNITION_INFORMATION = *mut FILE_SYSTEM_RECOGNITION_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_TYPE_NOTIFICATION_INPUT = *mut FILE_TYPE_NOTIFICATION_INPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_ZERO_DATA_INFORMATION = *mut FILE_ZERO_DATA_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_ZERO_DATA_INFORMATION_EX = *mut FILE_ZERO_DATA_INFORMATION_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFIND_BY_SID_DATA = *mut FIND_BY_SID_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFIND_BY_SID_OUTPUT = *mut FIND_BY_SID_OUTPUT;
#[cfg(feature = "shared")]
pub type PFORMAT_EX_PARAMETERS = *mut FORMAT_EX_PARAMETERS;
#[cfg(feature = "shared")]
pub type PFORMAT_PARAMETERS = *mut FORMAT_PARAMETERS;
#[cfg(feature = "shared")]
pub type PFSCTL_GET_INTEGRITY_INFORMATION_BUFFER = *mut FSCTL_GET_INTEGRITY_INFORMATION_BUFFER;
#[cfg(feature = "shared")]
pub type PFSCTL_OFFLOAD_READ_INPUT = *mut FSCTL_OFFLOAD_READ_INPUT;
#[cfg(feature = "shared")]
pub type PFSCTL_OFFLOAD_READ_OUTPUT = *mut FSCTL_OFFLOAD_READ_OUTPUT;
#[cfg(feature = "shared")]
pub type PFSCTL_OFFLOAD_WRITE_INPUT = *mut FSCTL_OFFLOAD_WRITE_INPUT;
#[cfg(feature = "shared")]
pub type PFSCTL_OFFLOAD_WRITE_OUTPUT = *mut FSCTL_OFFLOAD_WRITE_OUTPUT;
#[cfg(feature = "shared")]
pub type PFSCTL_QUERY_FAT_BPB_BUFFER = *mut FSCTL_QUERY_FAT_BPB_BUFFER;
#[cfg(feature = "shared")]
pub type PFSCTL_QUERY_REGION_INFO_INPUT = *mut FSCTL_QUERY_REGION_INFO_INPUT;
#[cfg(feature = "shared")]
pub type PFSCTL_QUERY_REGION_INFO_OUTPUT = *mut FSCTL_QUERY_REGION_INFO_OUTPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFSCTL_QUERY_STORAGE_CLASSES_OUTPUT = *mut FSCTL_QUERY_STORAGE_CLASSES_OUTPUT;
#[cfg(feature = "shared")]
pub type PFSCTL_SET_INTEGRITY_INFORMATION_BUFFER = *mut FSCTL_SET_INTEGRITY_INFORMATION_BUFFER;
#[cfg(feature = "shared")]
pub type PFSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX = *mut FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFS_BPIO_INFO = *mut FS_BPIO_INFO;
#[cfg(feature = "shared")]
pub type PFS_BPIO_INPUT = *mut FS_BPIO_INPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFS_BPIO_OUTPUT = *mut FS_BPIO_OUTPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFS_BPIO_RESULTS = *mut FS_BPIO_RESULTS;
#[cfg(feature = "shared")]
pub type PGETVERSIONINPARAMS = *mut GETVERSIONINPARAMS;
#[cfg(feature = "shared")]
pub type PGET_CHANGER_PARAMETERS = *mut GET_CHANGER_PARAMETERS;
#[cfg(feature = "shared")]
pub type PGET_DEVICE_INTERNAL_STATUS_DATA_REQUEST = *mut GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST;
#[cfg(feature = "shared")]
pub type PGET_DISK_ATTRIBUTES = *mut GET_DISK_ATTRIBUTES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGET_FILTER_FILE_IDENTIFIER_INPUT = *mut GET_FILTER_FILE_IDENTIFIER_INPUT;
#[cfg(feature = "shared")]
pub type PGET_FILTER_FILE_IDENTIFIER_OUTPUT = *mut GET_FILTER_FILE_IDENTIFIER_OUTPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGET_LENGTH_INFORMATION = *mut GET_LENGTH_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGET_MEDIA_TYPES = *mut GET_MEDIA_TYPES;
#[cfg(feature = "shared")]
pub type PGP_LOG_PAGE_DESCRIPTOR = *mut GP_LOG_PAGE_DESCRIPTOR;
pub const PHILIPS_12_WO: STORAGE_MEDIA_TYPE = 67;
#[cfg(feature = "shared")]
pub type PHISTOGRAM_BUCKET = *mut HISTOGRAM_BUCKET;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_ELEMENT_STATUS {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DescriptorCount: super::DWORD,
    pub ReturnedDescriptorCount: super::DWORD,
    pub ElementIdentifierBeingDepoped: super::DWORD,
    pub Reserved: super::DWORD,
    pub Descriptors: [PHYSICAL_ELEMENT_STATUS_DESCRIPTOR; 1],
}
#[cfg(feature = "shared")]
impl Default for PHYSICAL_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub ElementIdentifier: super::DWORD,
    pub PhysicalElementType: super::BYTE,
    pub PhysicalElementHealth: super::BYTE,
    pub Reserved1: [super::BYTE; 2],
    pub AssociatedCapacity: super::DWORDLONG,
    pub Reserved2: [super::DWORD; 4],
}
#[cfg(feature = "shared")]
impl Default for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PHYSICAL_ELEMENT_STATUS_REQUEST {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub StartingElement: super::DWORD,
    pub Filter: super::BYTE,
    pub ReportType: super::BYTE,
    pub Reserved: [super::BYTE; 2],
}
#[cfg(feature = "shared")]
impl Default for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PIDEREGS = *mut IDEREGS;
pub const PINNACLE_APEX_5_RW: STORAGE_MEDIA_TYPE = 65;
#[cfg(feature = "shared")]
pub type PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK = Option<unsafe extern "C" fn(sourcecontext: *const IO_IRP_EXT_TRACK_OFFSET_HEADER, targetcontext: *mut IO_IRP_EXT_TRACK_OFFSET_HEADER, relativeoffset: super::LONGLONG)>;
#[cfg(feature = "shared")]
pub type PIO_IRP_EXT_TRACK_OFFSET_HEADER = *mut IO_IRP_EXT_TRACK_OFFSET_HEADER;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PLEX_READ_DATA_REQUEST {
    pub ByteOffset: super::LARGE_INTEGER,
    pub ByteLength: super::DWORD,
    pub PlexNumber: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PLEX_READ_DATA_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PLMR_QUERY_INFO_CLASS = *mut LMR_QUERY_INFO_CLASS;
pub type PLMR_QUERY_INFO_PARAM = *mut LMR_QUERY_INFO_PARAM;
#[cfg(feature = "shared")]
pub type PLMR_QUERY_SESSION_INFO = *mut LMR_QUERY_SESSION_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PLOOKUP_STREAM_FROM_CLUSTER_ENTRY = *mut LOOKUP_STREAM_FROM_CLUSTER_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PLOOKUP_STREAM_FROM_CLUSTER_INPUT = *mut LOOKUP_STREAM_FROM_CLUSTER_INPUT;
#[cfg(feature = "shared")]
pub type PLOOKUP_STREAM_FROM_CLUSTER_OUTPUT = *mut LOOKUP_STREAM_FROM_CLUSTER_OUTPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMARK_HANDLE_INFO = *mut MARK_HANDLE_INFO;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PMARK_HANDLE_INFO32 = *mut MARK_HANDLE_INFO32;
pub type PMEDIA_TYPE = *mut MEDIA_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMFT_ENUM_DATA = *mut MFT_ENUM_DATA_V1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMFT_ENUM_DATA_V0 = *mut MFT_ENUM_DATA_V0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMFT_ENUM_DATA_V1 = *mut MFT_ENUM_DATA_V1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMOVE_FILE_DATA = *mut MOVE_FILE_DATA;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMOVE_FILE_DATA32 = *mut MOVE_FILE_DATA32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMOVE_FILE_RECORD_DATA = *mut MOVE_FILE_RECORD_DATA;
#[cfg(feature = "shared")]
pub type PNTFS_EXTENDED_VOLUME_DATA = *mut NTFS_EXTENDED_VOLUME_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PNTFS_FILE_RECORD_INPUT_BUFFER = *mut NTFS_FILE_RECORD_INPUT_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PNTFS_FILE_RECORD_OUTPUT_BUFFER = *mut NTFS_FILE_RECORD_OUTPUT_BUFFER;
#[cfg(feature = "shared")]
pub type PNTFS_STATISTICS = *mut NTFS_STATISTICS;
#[cfg(feature = "shared")]
pub type PNTFS_STATISTICS_EX = *mut NTFS_STATISTICS_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PNTFS_VOLUME_DATA_BUFFER = *mut NTFS_VOLUME_DATA_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPARTITION_INFORMATION = *mut PARTITION_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPARTITION_INFORMATION_EX = *mut PARTITION_INFORMATION_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPARTITION_INFORMATION_GPT = *mut PARTITION_INFORMATION_GPT;
#[cfg(feature = "shared")]
pub type PPARTITION_INFORMATION_MBR = *mut PARTITION_INFORMATION_MBR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPATHNAME_BUFFER = *mut PATHNAME_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPERF_BIN = *mut PERF_BIN;
#[cfg(feature = "shared")]
pub type PPERSISTENT_RESERVE_COMMAND = *mut PERSISTENT_RESERVE_COMMAND;
#[cfg(feature = "shared")]
pub type PPHYSICAL_ELEMENT_STATUS = *mut PHYSICAL_ELEMENT_STATUS;
#[cfg(feature = "shared")]
pub type PPHYSICAL_ELEMENT_STATUS_DESCRIPTOR = *mut PHYSICAL_ELEMENT_STATUS_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PPHYSICAL_ELEMENT_STATUS_REQUEST = *mut PHYSICAL_ELEMENT_STATUS_REQUEST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPLEX_READ_DATA_REQUEST = *mut PLEX_READ_DATA_REQUEST;
#[cfg(feature = "shared")]
pub type PPREVENT_MEDIA_REMOVAL = *mut PREVENT_MEDIA_REMOVAL;
#[cfg(feature = "shared")]
pub type PQUERY_BAD_RANGES_INPUT = *mut QUERY_BAD_RANGES_INPUT;
#[cfg(feature = "shared")]
pub type PQUERY_BAD_RANGES_INPUT_RANGE = *mut QUERY_BAD_RANGES_INPUT_RANGE;
#[cfg(feature = "shared")]
pub type PQUERY_BAD_RANGES_OUTPUT = *mut QUERY_BAD_RANGES_OUTPUT;
#[cfg(feature = "shared")]
pub type PQUERY_BAD_RANGES_OUTPUT_RANGE = *mut QUERY_BAD_RANGES_OUTPUT_RANGE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PQUERY_FILE_LAYOUT_INPUT = *mut QUERY_FILE_LAYOUT_INPUT;
#[cfg(feature = "shared")]
pub type PQUERY_FILE_LAYOUT_OUTPUT = *mut QUERY_FILE_LAYOUT_OUTPUT;
#[cfg(feature = "shared")]
pub type PREAD_ELEMENT_ADDRESS_INFO = *mut READ_ELEMENT_ADDRESS_INFO;
#[cfg(feature = "shared")]
pub type PREAD_FILE_USN_DATA = *mut READ_FILE_USN_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREAD_USN_JOURNAL_DATA = *mut READ_USN_JOURNAL_DATA_V1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREAD_USN_JOURNAL_DATA_V0 = *mut READ_USN_JOURNAL_DATA_V0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREAD_USN_JOURNAL_DATA_V1 = *mut READ_USN_JOURNAL_DATA_V1;
#[cfg(feature = "shared")]
pub type PREASSIGN_BLOCKS = *mut REASSIGN_BLOCKS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREASSIGN_BLOCKS_EX = *mut REASSIGN_BLOCKS_EX;
pub type PREFS_SMR_VOLUME_GC_ACTION = *mut REFS_SMR_VOLUME_GC_ACTION;
pub type PREFS_SMR_VOLUME_GC_METHOD = *mut REFS_SMR_VOLUME_GC_METHOD;
#[cfg(feature = "shared")]
pub type PREFS_SMR_VOLUME_GC_PARAMETERS = *mut REFS_SMR_VOLUME_GC_PARAMETERS;
pub type PREFS_SMR_VOLUME_GC_STATE = *mut REFS_SMR_VOLUME_GC_STATE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREFS_SMR_VOLUME_INFO_OUTPUT = *mut REFS_SMR_VOLUME_INFO_OUTPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREFS_VOLUME_DATA_BUFFER = *mut REFS_VOLUME_DATA_BUFFER;
#[cfg(feature = "shared")]
pub type PREMOVE_ELEMENT_AND_TRUNCATE_REQUEST = *mut REMOVE_ELEMENT_AND_TRUNCATE_REQUEST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREPAIR_COPIES_INPUT = *mut REPAIR_COPIES_INPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREPAIR_COPIES_OUTPUT = *mut REPAIR_COPIES_OUTPUT;
#[cfg(feature = "shared")]
pub type PREQUEST_OPLOCK_INPUT_BUFFER = *mut REQUEST_OPLOCK_INPUT_BUFFER;
#[cfg(feature = "shared")]
pub type PREQUEST_OPLOCK_OUTPUT_BUFFER = *mut REQUEST_OPLOCK_OUTPUT_BUFFER;
#[cfg(feature = "shared")]
pub type PREQUEST_RAW_ENCRYPTED_DATA = *mut REQUEST_RAW_ENCRYPTED_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER = *mut RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRETRIEVAL_POINTERS_BUFFER = *mut RETRIEVAL_POINTERS_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRETRIEVAL_POINTER_BASE = *mut RETRIEVAL_POINTER_BASE;
#[cfg(feature = "shared")]
pub type PRETRIEVAL_POINTER_COUNT = *mut RETRIEVAL_POINTER_COUNT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PREVENT_MEDIA_REMOVAL {
    pub PreventMediaRemoval: super::BOOLEAN,
}
pub const PRODUCT_ID_LENGTH: i32 = 16;
pub const PROJFS_PROTOCOL_VERSION: i32 = 3;
#[cfg(feature = "shared")]
pub type PSCM_BUS_DEDICATED_MEMORY_DEVICES_INFO = *mut SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO;
#[cfg(feature = "shared")]
pub type PSCM_BUS_DEDICATED_MEMORY_DEVICE_INFO = *mut SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO;
#[cfg(feature = "shared")]
pub type PSCM_BUS_DEDICATED_MEMORY_STATE = *mut SCM_BUS_DEDICATED_MEMORY_STATE;
pub type PSCM_BUS_FIRMWARE_ACTIVATION_STATE = *mut SCM_BUS_FIRMWARE_ACTIVATION_STATE;
pub type PSCM_BUS_PROPERTY_ID = *mut SCM_BUS_PROPERTY_ID;
#[cfg(feature = "shared")]
pub type PSCM_BUS_PROPERTY_QUERY = *mut SCM_BUS_PROPERTY_QUERY;
#[cfg(feature = "shared")]
pub type PSCM_BUS_PROPERTY_SET = *mut SCM_BUS_PROPERTY_SET;
pub type PSCM_BUS_QUERY_TYPE = *mut SCM_BUS_QUERY_TYPE;
#[cfg(feature = "shared")]
pub type PSCM_BUS_RUNTIME_FW_ACTIVATION_INFO = *mut SCM_BUS_RUNTIME_FW_ACTIVATION_INFO;
pub type PSCM_BUS_SET_TYPE = *mut SCM_BUS_SET_TYPE;
#[cfg(feature = "shared")]
pub type PSCM_INTERLEAVED_PD_INFO = *mut SCM_INTERLEAVED_PD_INFO;
#[cfg(feature = "shared")]
pub type PSCM_LD_INTERLEAVE_SET_INFO = *mut SCM_LD_INTERLEAVE_SET_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCM_LOGICAL_DEVICES = *mut SCM_LOGICAL_DEVICES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCM_LOGICAL_DEVICE_INSTANCE = *mut SCM_LOGICAL_DEVICE_INSTANCE;
#[cfg(feature = "shared")]
pub type PSCM_PD_DESCRIPTOR_HEADER = *mut SCM_PD_DESCRIPTOR_HEADER;
#[cfg(feature = "shared")]
pub type PSCM_PD_DEVICE_HANDLE = *mut SCM_PD_DEVICE_HANDLE;
#[cfg(feature = "shared")]
pub type PSCM_PD_DEVICE_INFO = *mut SCM_PD_DEVICE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCM_PD_DEVICE_SPECIFIC_INFO = *mut SCM_PD_DEVICE_SPECIFIC_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCM_PD_DEVICE_SPECIFIC_PROPERTY = *mut SCM_PD_DEVICE_SPECIFIC_PROPERTY;
#[cfg(feature = "shared")]
pub type PSCM_PD_FIRMWARE_ACTIVATE = *mut SCM_PD_FIRMWARE_ACTIVATE;
pub type PSCM_PD_FIRMWARE_ACTIVATION_STATE = *mut SCM_PD_FIRMWARE_ACTIVATION_STATE;
#[cfg(feature = "shared")]
pub type PSCM_PD_FIRMWARE_DOWNLOAD = *mut SCM_PD_FIRMWARE_DOWNLOAD;
#[cfg(feature = "shared")]
pub type PSCM_PD_FIRMWARE_INFO = *mut SCM_PD_FIRMWARE_INFO;
#[cfg(feature = "shared")]
pub type PSCM_PD_FIRMWARE_SLOT_INFO = *mut SCM_PD_FIRMWARE_SLOT_INFO;
#[cfg(feature = "shared")]
pub type PSCM_PD_FRU_ID_STRING = *mut SCM_PD_FRU_ID_STRING;
pub type PSCM_PD_HEALTH_NOTIFICATION_DATA = *mut SCM_PD_HEALTH_NOTIFICATION_DATA;
pub type PSCM_PD_HEALTH_STATUS = *mut SCM_PD_HEALTH_STATUS;
pub type PSCM_PD_LAST_FW_ACTIVATION_STATUS = *mut SCM_PD_LAST_FW_ACTIVATION_STATUS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCM_PD_LOCATION_STRING = *mut SCM_PD_LOCATION_STRING;
#[cfg(feature = "shared")]
pub type PSCM_PD_MANAGEMENT_STATUS = *mut SCM_PD_MANAGEMENT_STATUS;
pub type PSCM_PD_MEDIA_REINITIALIZATION_STATUS = *mut SCM_PD_MEDIA_REINITIALIZATION_STATUS;
pub type PSCM_PD_OPERATIONAL_STATUS = *mut SCM_PD_OPERATIONAL_STATUS;
pub type PSCM_PD_OPERATIONAL_STATUS_REASON = *mut SCM_PD_OPERATIONAL_STATUS_REASON;
#[cfg(feature = "shared")]
pub type PSCM_PD_PASSTHROUGH_INPUT = *mut SCM_PD_PASSTHROUGH_INPUT;
#[cfg(feature = "shared")]
pub type PSCM_PD_PASSTHROUGH_INVDIMM_INPUT = *mut SCM_PD_PASSTHROUGH_INVDIMM_INPUT;
#[cfg(feature = "shared")]
pub type PSCM_PD_PASSTHROUGH_INVDIMM_OUTPUT = *mut SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT;
#[cfg(feature = "shared")]
pub type PSCM_PD_PASSTHROUGH_OUTPUT = *mut SCM_PD_PASSTHROUGH_OUTPUT;
pub type PSCM_PD_PROPERTY_ID = *mut SCM_PD_PROPERTY_ID;
#[cfg(feature = "shared")]
pub type PSCM_PD_PROPERTY_QUERY = *mut SCM_PD_PROPERTY_QUERY;
#[cfg(feature = "shared")]
pub type PSCM_PD_PROPERTY_SET = *mut SCM_PD_PROPERTY_SET;
pub type PSCM_PD_QUERY_TYPE = *mut SCM_PD_QUERY_TYPE;
#[cfg(feature = "shared")]
pub type PSCM_PD_REINITIALIZE_MEDIA_INPUT = *mut SCM_PD_REINITIALIZE_MEDIA_INPUT;
#[cfg(feature = "shared")]
pub type PSCM_PD_REINITIALIZE_MEDIA_OUTPUT = *mut SCM_PD_REINITIALIZE_MEDIA_OUTPUT;
#[cfg(feature = "shared")]
pub type PSCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE = *mut SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE;
#[cfg(feature = "shared")]
pub type PSCM_PD_RUNTIME_FW_ACTIVATION_INFO = *mut SCM_PD_RUNTIME_FW_ACTIVATION_INFO;
pub type PSCM_PD_SET_TYPE = *mut SCM_PD_SET_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCM_PHYSICAL_DEVICES = *mut SCM_PHYSICAL_DEVICES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSCM_PHYSICAL_DEVICE_INSTANCE = *mut SCM_PHYSICAL_DEVICE_INSTANCE;
#[cfg(feature = "shared")]
pub type PSCM_REGION = *mut SCM_REGION;
#[cfg(feature = "shared")]
pub type PSCM_REGIONS = *mut SCM_REGIONS;
pub type PSCM_REGION_FLAG = *mut SCM_REGION_FLAG;
#[cfg(feature = "shared")]
pub type PSD_CHANGE_MACHINE_SID_INPUT = *mut SD_CHANGE_MACHINE_SID_INPUT;
#[cfg(feature = "shared")]
pub type PSD_CHANGE_MACHINE_SID_OUTPUT = *mut SD_CHANGE_MACHINE_SID_OUTPUT;
#[cfg(feature = "shared")]
pub type PSD_ENUM_SDS_ENTRY = *mut SD_ENUM_SDS_ENTRY;
#[cfg(feature = "shared")]
pub type PSD_ENUM_SDS_INPUT = *mut SD_ENUM_SDS_INPUT;
#[cfg(feature = "shared")]
pub type PSD_ENUM_SDS_OUTPUT = *mut SD_ENUM_SDS_OUTPUT;
#[cfg(feature = "shared")]
pub type PSD_GLOBAL_CHANGE_INPUT = *mut SD_GLOBAL_CHANGE_INPUT;
#[cfg(feature = "shared")]
pub type PSD_GLOBAL_CHANGE_OUTPUT = *mut SD_GLOBAL_CHANGE_OUTPUT;
#[cfg(feature = "shared")]
pub type PSD_QUERY_STATS_INPUT = *mut SD_QUERY_STATS_INPUT;
#[cfg(feature = "shared")]
pub type PSD_QUERY_STATS_OUTPUT = *mut SD_QUERY_STATS_OUTPUT;
#[cfg(feature = "shared")]
pub type PSENDCMDINPARAMS = *mut SENDCMDINPARAMS;
#[cfg(feature = "shared")]
pub type PSENDCMDOUTPARAMS = *mut SENDCMDOUTPARAMS;
#[cfg(feature = "shared")]
pub type PSET_DAX_ALLOC_ALIGNMENT_HINT_INPUT = *mut SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT;
#[cfg(feature = "shared")]
pub type PSET_DISK_ATTRIBUTES = *mut SET_DISK_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PSET_PARTITION_INFORMATION = *mut SET_PARTITION_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSET_PARTITION_INFORMATION_EX = *mut SET_PARTITION_INFORMATION_EX;
#[cfg(feature = "shared")]
pub type PSET_PURGE_FAILURE_MODE_INPUT = *mut SET_PURGE_FAILURE_MODE_INPUT;
#[cfg(feature = "shared")]
pub type PSHRINK_VOLUME_INFORMATION = *mut SHRINK_VOLUME_INFORMATION;
pub type PSHRINK_VOLUME_REQUEST_TYPES = *mut SHRINK_VOLUME_REQUEST_TYPES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSI_COPYFILE = *mut SI_COPYFILE;
#[cfg(feature = "shared")]
pub type PSMB_SHARE_FLUSH_AND_PURGE_INPUT = *mut SMB_SHARE_FLUSH_AND_PURGE_INPUT;
#[cfg(feature = "shared")]
pub type PSMB_SHARE_FLUSH_AND_PURGE_OUTPUT = *mut SMB_SHARE_FLUSH_AND_PURGE_OUTPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTARTING_LCN_INPUT_BUFFER = *mut STARTING_LCN_INPUT_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTARTING_LCN_INPUT_BUFFER_EX = *mut STARTING_LCN_INPUT_BUFFER_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTARTING_VCN_INPUT_BUFFER = *mut STARTING_VCN_INPUT_BUFFER;
#[cfg(feature = "shared")]
pub type PSTORAGE_ACCESS_ALIGNMENT_DESCRIPTOR = *mut STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_ADAPTER_DESCRIPTOR = *mut STORAGE_ADAPTER_DESCRIPTOR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTORAGE_ADAPTER_SERIAL_NUMBER = *mut STORAGE_ADAPTER_SERIAL_NUMBER;
#[cfg(feature = "shared")]
pub type PSTORAGE_ALLOCATE_BC_STREAM_INPUT = *mut STORAGE_ALLOCATE_BC_STREAM_INPUT;
#[cfg(feature = "shared")]
pub type PSTORAGE_ALLOCATE_BC_STREAM_OUTPUT = *mut STORAGE_ALLOCATE_BC_STREAM_OUTPUT;
pub type PSTORAGE_ASSOCIATION_TYPE = *mut STORAGE_ASSOCIATION_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_ATTRIBUTE_MGMT = *mut STORAGE_ATTRIBUTE_MGMT;
pub type PSTORAGE_ATTRIBUTE_MGMT_ACTION = *mut STORAGE_ATTRIBUTE_MGMT_ACTION;
#[cfg(feature = "shared")]
pub type PSTORAGE_BREAK_RESERVATION_REQUEST = *mut STORAGE_BREAK_RESERVATION_REQUEST;
#[cfg(feature = "shared")]
pub type PSTORAGE_BUS_RESET_REQUEST = *mut STORAGE_BUS_RESET_REQUEST;
pub type PSTORAGE_BUS_TYPE = *mut STORAGE_BUS_TYPE;
pub type PSTORAGE_COMPONENT_HEALTH_STATUS = *mut STORAGE_COMPONENT_HEALTH_STATUS;
#[cfg(feature = "shared")]
pub type PSTORAGE_COUNTER = *mut STORAGE_COUNTER;
#[cfg(feature = "shared")]
pub type PSTORAGE_COUNTERS = *mut STORAGE_COUNTERS;
pub type PSTORAGE_COUNTER_TYPE = *mut STORAGE_COUNTER_TYPE;
pub type PSTORAGE_CRYPTO_ALGORITHM_ID = *mut STORAGE_CRYPTO_ALGORITHM_ID;
#[cfg(feature = "shared")]
pub type PSTORAGE_CRYPTO_CAPABILITY = *mut STORAGE_CRYPTO_CAPABILITY;
#[cfg(feature = "shared")]
pub type PSTORAGE_CRYPTO_CAPABILITY_V2 = *mut STORAGE_CRYPTO_CAPABILITY_V2;
#[cfg(feature = "shared")]
pub type PSTORAGE_CRYPTO_DESCRIPTOR = *mut STORAGE_CRYPTO_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_CRYPTO_DESCRIPTOR_V2 = *mut STORAGE_CRYPTO_DESCRIPTOR_V2;
pub type PSTORAGE_CRYPTO_KEY_SIZE = *mut STORAGE_CRYPTO_KEY_SIZE;
#[cfg(feature = "shared")]
pub type PSTORAGE_DESCRIPTOR_HEADER = *mut STORAGE_DESCRIPTOR_HEADER;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR = *mut STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_DESCRIPTOR = *mut STORAGE_DEVICE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR = *mut STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR;
pub type PSTORAGE_DEVICE_FORM_FACTOR = *mut STORAGE_DEVICE_FORM_FACTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_ID_DESCRIPTOR = *mut STORAGE_DEVICE_ID_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR = *mut STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_LED_STATE_DESCRIPTOR = *mut STORAGE_DEVICE_LED_STATE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_LOCATION_DESCRIPTOR = *mut STORAGE_DEVICE_LOCATION_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_MANAGEMENT_STATUS = *mut STORAGE_DEVICE_MANAGEMENT_STATUS;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_NUMA_PROPERTY = *mut STORAGE_DEVICE_NUMA_PROPERTY;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_NUMBER = *mut STORAGE_DEVICE_NUMBER;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_NUMBERS = *mut STORAGE_DEVICE_NUMBERS;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_NUMBER_EX = *mut STORAGE_DEVICE_NUMBER_EX;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_POWER_CAP = *mut STORAGE_DEVICE_POWER_CAP;
pub type PSTORAGE_DEVICE_POWER_CAP_UNITS = *mut STORAGE_DEVICE_POWER_CAP_UNITS;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_RESILIENCY_DESCRIPTOR = *mut STORAGE_DEVICE_RESILIENCY_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY = *mut STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2 = *mut STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTORAGE_DEVICE_TIERING_DESCRIPTOR = *mut STORAGE_DEVICE_TIERING_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT = *mut STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT;
#[cfg(feature = "shared")]
pub type PSTORAGE_DIAGNOSTIC_DATA = *mut STORAGE_DIAGNOSTIC_DATA;
pub type PSTORAGE_DIAGNOSTIC_LEVEL = *mut STORAGE_DIAGNOSTIC_LEVEL;
#[cfg(feature = "shared")]
pub type PSTORAGE_DIAGNOSTIC_REQUEST = *mut STORAGE_DIAGNOSTIC_REQUEST;
pub type PSTORAGE_DIAGNOSTIC_TARGET_TYPE = *mut STORAGE_DIAGNOSTIC_TARGET_TYPE;
pub type PSTORAGE_DISK_HEALTH_STATUS = *mut STORAGE_DISK_HEALTH_STATUS;
pub type PSTORAGE_DISK_OPERATIONAL_STATUS = *mut STORAGE_DISK_OPERATIONAL_STATUS;
pub type PSTORAGE_ENCRYPTION_TYPE = *mut STORAGE_ENCRYPTION_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_EVENT_NOTIFICATION = *mut STORAGE_EVENT_NOTIFICATION;
#[cfg(feature = "shared")]
pub type PSTORAGE_FAILURE_PREDICTION_CONFIG = *mut STORAGE_FAILURE_PREDICTION_CONFIG;
#[cfg(feature = "shared")]
pub type PSTORAGE_FEATURE_SUPPORT = *mut STORAGE_FEATURE_SUPPORT;
#[cfg(feature = "shared")]
pub type PSTORAGE_FRU_ID_DESCRIPTOR = *mut STORAGE_FRU_ID_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_GET_BC_PROPERTIES_OUTPUT = *mut STORAGE_GET_BC_PROPERTIES_OUTPUT;
#[cfg(feature = "shared")]
pub type PSTORAGE_HOTPLUG_INFO = *mut STORAGE_HOTPLUG_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_BOOT_PARTITION_ACTIVATE = *mut STORAGE_HW_BOOT_PARTITION_ACTIVATE;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_BOOT_PARTITION_DOWNLOAD = *mut STORAGE_HW_BOOT_PARTITION_DOWNLOAD;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_BOOT_PARTITION_INFO = *mut STORAGE_HW_BOOT_PARTITION_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_CRYPTO_CAPABILITY = *mut STORAGE_HW_CRYPTO_CAPABILITY;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_CRYPTO_DESCRIPTOR = *mut STORAGE_HW_CRYPTO_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_ENDURANCE_DATA_DESCRIPTOR = *mut STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_ENDURANCE_INFO = *mut STORAGE_HW_ENDURANCE_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_FIRMWARE_ACTIVATE = *mut STORAGE_HW_FIRMWARE_ACTIVATE;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_FIRMWARE_DOWNLOAD = *mut STORAGE_HW_FIRMWARE_DOWNLOAD;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_FIRMWARE_DOWNLOAD_V2 = *mut STORAGE_HW_FIRMWARE_DOWNLOAD_V2;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_FIRMWARE_INFO = *mut STORAGE_HW_FIRMWARE_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_FIRMWARE_INFO_QUERY = *mut STORAGE_HW_FIRMWARE_INFO_QUERY;
#[cfg(feature = "shared")]
pub type PSTORAGE_HW_FIRMWARE_SLOT_INFO = *mut STORAGE_HW_FIRMWARE_SLOT_INFO;
pub type PSTORAGE_ICE_TYPE = *mut STORAGE_ICE_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_IDENTIFIER = *mut STORAGE_IDENTIFIER;
pub type PSTORAGE_IDENTIFIER_CODE_SET = *mut STORAGE_IDENTIFIER_CODE_SET;
pub type PSTORAGE_IDENTIFIER_TYPE = *mut STORAGE_IDENTIFIER_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_IDLE_POWER = *mut STORAGE_IDLE_POWER;
#[cfg(feature = "shared")]
pub type PSTORAGE_IDLE_POWERUP_REASON = *mut STORAGE_IDLE_POWERUP_REASON;
pub type PSTORAGE_ID_NAA_FORMAT = *mut STORAGE_ID_NAA_FORMAT;
#[cfg(feature = "shared")]
pub type PSTORAGE_LB_PROVISIONING_MAP_RESOURCES = *mut STORAGE_LB_PROVISIONING_MAP_RESOURCES;
#[cfg(feature = "shared")]
pub type PSTORAGE_MEDIA_SERIAL_NUMBER_DATA = *mut STORAGE_MEDIA_SERIAL_NUMBER_DATA;
pub type PSTORAGE_MEDIA_TYPE = *mut STORAGE_MEDIA_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR = *mut STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_MINIPORT_DESCRIPTOR = *mut STORAGE_MINIPORT_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_OFFLOAD_READ_OUTPUT = *mut STORAGE_OFFLOAD_READ_OUTPUT;
#[cfg(feature = "shared")]
pub type PSTORAGE_OFFLOAD_TOKEN = *mut STORAGE_OFFLOAD_TOKEN;
#[cfg(feature = "shared")]
pub type PSTORAGE_OFFLOAD_WRITE_OUTPUT = *mut STORAGE_OFFLOAD_WRITE_OUTPUT;
#[cfg(feature = "shared")]
pub type PSTORAGE_OPERATIONAL_REASON = *mut STORAGE_OPERATIONAL_REASON;
pub type PSTORAGE_OPERATIONAL_STATUS_REASON = *mut STORAGE_OPERATIONAL_STATUS_REASON;
#[cfg(feature = "shared")]
pub type PSTORAGE_PHYSICAL_ADAPTER_DATA = *mut STORAGE_PHYSICAL_ADAPTER_DATA;
#[cfg(feature = "shared")]
pub type PSTORAGE_PHYSICAL_DEVICE_DATA = *mut STORAGE_PHYSICAL_DEVICE_DATA;
#[cfg(feature = "shared")]
pub type PSTORAGE_PHYSICAL_NODE_DATA = *mut STORAGE_PHYSICAL_NODE_DATA;
#[cfg(feature = "shared")]
pub type PSTORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR = *mut STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR;
pub type PSTORAGE_PORT_CODE_SET = *mut STORAGE_PORT_CODE_SET;
pub type PSTORAGE_POWERUP_REASON_TYPE = *mut STORAGE_POWERUP_REASON_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_PREDICT_FAILURE = *mut STORAGE_PREDICT_FAILURE;
#[cfg(feature = "shared")]
pub type PSTORAGE_PRIORITY_HINT_SUPPORT = *mut STORAGE_PRIORITY_HINT_SUPPORT;
pub type PSTORAGE_PROPERTY_ID = *mut STORAGE_PROPERTY_ID;
#[cfg(feature = "shared")]
pub type PSTORAGE_PROPERTY_QUERY = *mut STORAGE_PROPERTY_QUERY;
#[cfg(feature = "shared")]
pub type PSTORAGE_PROPERTY_SET = *mut STORAGE_PROPERTY_SET;
pub type PSTORAGE_PROTOCOL_ATA_DATA_TYPE = *mut STORAGE_PROTOCOL_ATA_DATA_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_PROTOCOL_COMMAND = *mut STORAGE_PROTOCOL_COMMAND;
#[cfg(feature = "shared")]
pub type PSTORAGE_PROTOCOL_DATA_DESCRIPTOR = *mut STORAGE_PROTOCOL_DATA_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT = *mut STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT;
#[cfg(feature = "shared")]
pub type PSTORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE = *mut STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE;
pub type PSTORAGE_PROTOCOL_NVME_DATA_TYPE = *mut STORAGE_PROTOCOL_NVME_DATA_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_PROTOCOL_SPECIFIC_DATA = *mut STORAGE_PROTOCOL_SPECIFIC_DATA;
#[cfg(feature = "shared")]
pub type PSTORAGE_PROTOCOL_SPECIFIC_DATA_EXT = *mut STORAGE_PROTOCOL_SPECIFIC_DATA_EXT;
pub type PSTORAGE_PROTOCOL_TYPE = *mut STORAGE_PROTOCOL_TYPE;
pub type PSTORAGE_PROTOCOL_UFS_DATA_TYPE = *mut STORAGE_PROTOCOL_UFS_DATA_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY = *mut STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY;
#[cfg(feature = "shared")]
pub type PSTORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY = *mut STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY;
#[cfg(feature = "shared")]
pub type PSTORAGE_QUERY_DEPENDENT_VOLUME_REQUEST = *mut STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST;
pub type PSTORAGE_QUERY_TYPE = *mut STORAGE_QUERY_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTORAGE_READ_CAPACITY = *mut STORAGE_READ_CAPACITY;
#[cfg(feature = "shared")]
pub type PSTORAGE_REINITIALIZE_MEDIA = *mut STORAGE_REINITIALIZE_MEDIA;
pub type PSTORAGE_RESERVE_ID = *mut STORAGE_RESERVE_ID;
pub type PSTORAGE_RPMB_COMMAND_TYPE = *mut STORAGE_RPMB_COMMAND_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_RPMB_DATA_FRAME = *mut STORAGE_RPMB_DATA_FRAME;
#[cfg(feature = "shared")]
pub type PSTORAGE_RPMB_DESCRIPTOR = *mut STORAGE_RPMB_DESCRIPTOR;
pub type PSTORAGE_RPMB_FRAME_TYPE = *mut STORAGE_RPMB_FRAME_TYPE;
pub type PSTORAGE_SANITIZE_METHOD = *mut STORAGE_SANITIZE_METHOD;
pub type PSTORAGE_SET_TYPE = *mut STORAGE_SET_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_SPEC_VERSION = *mut STORAGE_SPEC_VERSION;
#[cfg(feature = "shared")]
pub type PSTORAGE_STACK_DESCRIPTOR = *mut STORAGE_STACK_DESCRIPTOR;
pub type PSTORAGE_STACK_TYPE = *mut STORAGE_STACK_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_TEMPERATURE_DATA_DESCRIPTOR = *mut STORAGE_TEMPERATURE_DATA_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_TEMPERATURE_INFO = *mut STORAGE_TEMPERATURE_INFO;
#[cfg(feature = "shared")]
pub type PSTORAGE_TEMPERATURE_THRESHOLD = *mut STORAGE_TEMPERATURE_THRESHOLD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTORAGE_TIER = *mut STORAGE_TIER;
pub type PSTORAGE_TIER_CLASS = *mut STORAGE_TIER_CLASS;
pub type PSTORAGE_TIER_MEDIA_TYPE = *mut STORAGE_TIER_MEDIA_TYPE;
#[cfg(feature = "shared")]
pub type PSTORAGE_TIER_REGION = *mut STORAGE_TIER_REGION;
#[cfg(feature = "shared")]
pub type PSTORAGE_WRITE_CACHE_PROPERTY = *mut STORAGE_WRITE_CACHE_PROPERTY;
#[cfg(feature = "shared")]
pub type PSTORAGE_ZONED_DEVICE_DESCRIPTOR = *mut STORAGE_ZONED_DEVICE_DESCRIPTOR;
pub type PSTORAGE_ZONED_DEVICE_TYPES = *mut STORAGE_ZONED_DEVICE_TYPES;
pub type PSTORAGE_ZONES_ATTRIBUTES = *mut STORAGE_ZONES_ATTRIBUTES;
pub type PSTORAGE_ZONE_CONDITION = *mut STORAGE_ZONE_CONDITION;
#[cfg(feature = "shared")]
pub type PSTORAGE_ZONE_DESCRIPTOR = *mut STORAGE_ZONE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PSTORAGE_ZONE_GROUP = *mut STORAGE_ZONE_GROUP;
pub type PSTORAGE_ZONE_TYPES = *mut STORAGE_ZONE_TYPES;
#[cfg(feature = "shared")]
pub type PSTREAMS_ASSOCIATE_ID_INPUT_BUFFER = *mut STREAMS_ASSOCIATE_ID_INPUT_BUFFER;
#[cfg(feature = "shared")]
pub type PSTREAMS_QUERY_ID_OUTPUT_BUFFER = *mut STREAMS_QUERY_ID_OUTPUT_BUFFER;
#[cfg(feature = "shared")]
pub type PSTREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER = *mut STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTREAM_EXTENT_ENTRY = *mut STREAM_EXTENT_ENTRY;
#[cfg(feature = "shared")]
pub type PSTREAM_INFORMATION_ENTRY = *mut STREAM_INFORMATION_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTREAM_LAYOUT_ENTRY = *mut STREAM_LAYOUT_ENTRY;
#[cfg(feature = "shared")]
pub type PTAPE_GET_STATISTICS = *mut TAPE_GET_STATISTICS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTAPE_STATISTICS = *mut TAPE_STATISTICS;
#[cfg(feature = "shared")]
pub type PTXFS_CREATE_MINIVERSION_INFO = *mut TXFS_CREATE_MINIVERSION_INFO;
#[cfg(feature = "shared")]
pub type PTXFS_GET_METADATA_INFO_OUT = *mut TXFS_GET_METADATA_INFO_OUT;
#[cfg(feature = "shared")]
pub type PTXFS_GET_TRANSACTED_VERSION = *mut TXFS_GET_TRANSACTED_VERSION;
#[cfg(feature = "shared")]
pub type PTXFS_LIST_TRANSACTIONS = *mut TXFS_LIST_TRANSACTIONS;
#[cfg(feature = "shared")]
pub type PTXFS_LIST_TRANSACTIONS_ENTRY = *mut TXFS_LIST_TRANSACTIONS_ENTRY;
#[cfg(feature = "shared")]
pub type PTXFS_LIST_TRANSACTION_LOCKED_FILES = *mut TXFS_LIST_TRANSACTION_LOCKED_FILES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY = *mut TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY;
#[cfg(feature = "shared")]
pub type PTXFS_MODIFY_RM = *mut TXFS_MODIFY_RM;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTXFS_QUERY_RM_INFORMATION = *mut TXFS_QUERY_RM_INFORMATION;
#[cfg(feature = "shared")]
pub type PTXFS_READ_BACKUP_INFORMATION_OUT = *mut TXFS_READ_BACKUP_INFORMATION_OUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTXFS_ROLLFORWARD_REDO_INFORMATION = *mut TXFS_ROLLFORWARD_REDO_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTXFS_SAVEPOINT_INFORMATION = *mut TXFS_SAVEPOINT_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTXFS_START_RM_INFORMATION = *mut TXFS_START_RM_INFORMATION;
#[cfg(feature = "shared")]
pub type PTXFS_TRANSACTION_ACTIVE_INFO = *mut TXFS_TRANSACTION_ACTIVE_INFO;
#[cfg(feature = "shared")]
pub type PTXFS_WRITE_BACKUP_INFORMATION = *mut TXFS_WRITE_BACKUP_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_JOURNAL_DATA = *mut USN_JOURNAL_DATA_V1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_JOURNAL_DATA_V0 = *mut USN_JOURNAL_DATA_V0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_JOURNAL_DATA_V1 = *mut USN_JOURNAL_DATA_V1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_JOURNAL_DATA_V2 = *mut USN_JOURNAL_DATA_V2;
#[cfg(feature = "winnt")]
pub type PUSN_RANGE_TRACK_OUTPUT = *mut USN_RANGE_TRACK_OUTPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_RECORD = *mut USN_RECORD_V2;
#[cfg(feature = "shared")]
pub type PUSN_RECORD_COMMON_HEADER = *mut USN_RECORD_COMMON_HEADER;
#[cfg(feature = "shared")]
pub type PUSN_RECORD_EXTENT = *mut USN_RECORD_EXTENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_RECORD_UNION = *mut USN_RECORD_UNION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_RECORD_V2 = *mut USN_RECORD_V2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_RECORD_V3 = *mut USN_RECORD_V3;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSN_RECORD_V4 = *mut USN_RECORD_V4;
#[cfg(feature = "shared")]
pub type PUSN_TRACK_MODIFIED_RANGES = *mut USN_TRACK_MODIFIED_RANGES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PVERIFY_INFORMATION = *mut VERIFY_INFORMATION;
#[cfg(feature = "shared")]
pub type PVIRTUALIZATION_INSTANCE_INFO_INPUT = *mut VIRTUALIZATION_INSTANCE_INFO_INPUT;
#[cfg(feature = "shared")]
pub type PVIRTUALIZATION_INSTANCE_INFO_INPUT_EX = *mut VIRTUALIZATION_INSTANCE_INFO_INPUT_EX;
pub type PVIRTUALIZATION_INSTANCE_INFO_OUTPUT = *mut VIRTUALIZATION_INSTANCE_INFO_OUTPUT;
pub type PVIRTUAL_STORAGE_BEHAVIOR_CODE = *mut VIRTUAL_STORAGE_BEHAVIOR_CODE;
#[cfg(feature = "shared")]
pub type PVIRTUAL_STORAGE_SET_BEHAVIOR_INPUT = *mut VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT;
#[cfg(feature = "shared")]
pub type PVIRTUAL_STORAGE_TYPE = *mut VIRTUAL_STORAGE_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PVOLUME_BITMAP_BUFFER = *mut VOLUME_BITMAP_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PVOLUME_DISK_EXTENTS = *mut VOLUME_DISK_EXTENTS;
#[cfg(feature = "shared")]
pub type PVOLUME_GET_GPT_ATTRIBUTES_INFORMATION = *mut VOLUME_GET_GPT_ATTRIBUTES_INFORMATION;
#[cfg(feature = "shared")]
pub type PWIM_PROVIDER_ADD_OVERLAY_INPUT = *mut WIM_PROVIDER_ADD_OVERLAY_INPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWIM_PROVIDER_EXTERNAL_INFO = *mut WIM_PROVIDER_EXTERNAL_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWIM_PROVIDER_OVERLAY_ENTRY = *mut WIM_PROVIDER_OVERLAY_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWIM_PROVIDER_REMOVE_OVERLAY_INPUT = *mut WIM_PROVIDER_REMOVE_OVERLAY_INPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWIM_PROVIDER_SUSPEND_OVERLAY_INPUT = *mut WIM_PROVIDER_SUSPEND_OVERLAY_INPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWIM_PROVIDER_UPDATE_OVERLAY_INPUT = *mut WIM_PROVIDER_UPDATE_OVERLAY_INPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWOF_EXTERNAL_FILE_ID = *mut WOF_EXTERNAL_FILE_ID;
#[cfg(feature = "shared")]
pub type PWOF_EXTERNAL_INFO = *mut WOF_EXTERNAL_INFO;
#[cfg(feature = "shared")]
pub type PWOF_VERSION_INFO = *mut WOF_VERSION_INFO;
#[cfg(feature = "shared")]
pub type PWRITE_USN_REASON_INPUT = *mut WRITE_USN_REASON_INPUT;
pub const PropertyExistsQuery: STORAGE_QUERY_TYPE = 1;
pub const PropertyExistsSet: STORAGE_SET_TYPE = 1;
pub const PropertyMaskQuery: STORAGE_QUERY_TYPE = 2;
pub const PropertyQueryMaxDefined: STORAGE_QUERY_TYPE = 3;
pub const PropertySetMaxDefined: STORAGE_SET_TYPE = 2;
pub const PropertyStandardQuery: STORAGE_QUERY_TYPE = 0;
pub const PropertyStandardSet: STORAGE_SET_TYPE = 0;
pub const ProtocolTypeAta: STORAGE_PROTOCOL_TYPE = 2;
pub const ProtocolTypeMaxReserved: STORAGE_PROTOCOL_TYPE = 127;
pub const ProtocolTypeNvme: STORAGE_PROTOCOL_TYPE = 3;
pub const ProtocolTypeProprietary: STORAGE_PROTOCOL_TYPE = 126;
pub const ProtocolTypeScsi: STORAGE_PROTOCOL_TYPE = 1;
pub const ProtocolTypeSd: STORAGE_PROTOCOL_TYPE = 4;
pub const ProtocolTypeUfs: STORAGE_PROTOCOL_TYPE = 5;
pub const ProtocolTypeUnknown: STORAGE_PROTOCOL_TYPE = 0;
pub const QIC: STORAGE_MEDIA_TYPE = 35;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_BAD_RANGES_INPUT {
    pub Flags: super::DWORD,
    pub NumRanges: super::DWORD,
    pub Ranges: [QUERY_BAD_RANGES_INPUT_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for QUERY_BAD_RANGES_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_BAD_RANGES_INPUT_RANGE {
    pub StartOffset: super::DWORDLONG,
    pub LengthInBytes: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_BAD_RANGES_OUTPUT {
    pub Flags: super::DWORD,
    pub NumBadRanges: super::DWORD,
    pub NextOffsetToLookUp: super::DWORDLONG,
    pub BadRanges: [QUERY_BAD_RANGES_OUTPUT_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for QUERY_BAD_RANGES_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_BAD_RANGES_OUTPUT_RANGE {
    pub Flags: super::DWORD,
    pub Reserved: super::DWORD,
    pub StartOffset: super::DWORDLONG,
    pub LengthInBytes: super::DWORDLONG,
}
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_GUEST_VOLUMES: i32 = 2;
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_HOST_VOLUMES: i32 = 1;
pub type QUERY_FILE_LAYOUT_FILTER_TYPE = i32;
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_CLUSTERS: QUERY_FILE_LAYOUT_FILTER_TYPE = 1;
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_FILEID: QUERY_FILE_LAYOUT_FILTER_TYPE = 2;
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_NONE: QUERY_FILE_LAYOUT_FILTER_TYPE = 0;
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_STORAGE_RESERVE_ID: QUERY_FILE_LAYOUT_FILTER_TYPE = 3;
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTENTS: i32 = 8;
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTRA_INFO: i32 = 16;
pub const QUERY_FILE_LAYOUT_INCLUDE_FILES_WITH_DSC_ATTRIBUTE: i32 = 4096;
pub const QUERY_FILE_LAYOUT_INCLUDE_FULL_PATH_IN_NAMES: i32 = 64;
pub const QUERY_FILE_LAYOUT_INCLUDE_NAMES: i32 = 2;
pub const QUERY_FILE_LAYOUT_INCLUDE_ONLY_FILES_WITH_SPECIFIC_ATTRIBUTES: i32 = 2048;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS: i32 = 4;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS_WITH_NO_CLUSTERS_ALLOCATED: i32 = 32;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION: i32 = 128;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DATA_ATTRIBUTE: i32 = 8192;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DSC_ATTRIBUTE: i32 = 256;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EA_ATTRIBUTE: i32 = 32768;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EFS_ATTRIBUTE: i32 = 1024;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_REPARSE_ATTRIBUTE: i32 = 16384;
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_TXF_ATTRIBUTE: i32 = 512;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct QUERY_FILE_LAYOUT_INPUT {
    pub Anonymous: QUERY_FILE_LAYOUT_INPUT_0,
    pub Flags: super::DWORD,
    pub FilterType: QUERY_FILE_LAYOUT_FILTER_TYPE,
    pub Reserved: super::DWORD,
    pub Filter: QUERY_FILE_LAYOUT_INPUT_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for QUERY_FILE_LAYOUT_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union QUERY_FILE_LAYOUT_INPUT_0 {
    pub FilterEntryCount: super::DWORD,
    pub NumberOfPairs: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for QUERY_FILE_LAYOUT_INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union QUERY_FILE_LAYOUT_INPUT_1 {
    pub ClusterRanges: [CLUSTER_RANGE; 1],
    pub FileReferenceRanges: [FILE_REFERENCE_RANGE; 1],
    pub StorageReserveIds: [STORAGE_RESERVE_ID; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for QUERY_FILE_LAYOUT_INPUT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QUERY_FILE_LAYOUT_NUM_FILTER_TYPES: QUERY_FILE_LAYOUT_FILTER_TYPE = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_FILE_LAYOUT_OUTPUT {
    pub FileEntryCount: super::DWORD,
    pub FirstFileOffset: super::DWORD,
    pub Flags: super::DWORD,
    pub Reserved: super::DWORD,
}
pub const QUERY_FILE_LAYOUT_REPARSE_DATA_INVALID: i32 = 1;
pub const QUERY_FILE_LAYOUT_REPARSE_TAG_INVALID: i32 = 2;
pub const QUERY_FILE_LAYOUT_RESTART: i32 = 1;
pub const QUERY_FILE_LAYOUT_SINGLE_INSTANCED: i32 = 1;
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_READ: i32 = 1073741824;
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_WRITE: u32 = 2147483648;
pub const QUERY_STORAGE_CLASSES_FLAGS_NO_DEFRAG_VOLUME: i32 = 536870912;
pub const READ_ATTRIBUTES: i32 = 208;
pub const READ_ATTRIBUTE_BUFFER_SIZE: i32 = 512;
pub const READ_COMPRESSION_INFO_VALID: i32 = 32;
pub const READ_COPY_NUMBER_BYPASS_CACHE_FLAG: i32 = 256;
pub const READ_COPY_NUMBER_KEY: i32 = 1380142592;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct READ_ELEMENT_ADDRESS_INFO {
    pub NumberOfElements: super::DWORD,
    pub ElementStatus: [CHANGER_ELEMENT_STATUS; 1],
}
#[cfg(feature = "shared")]
impl Default for READ_ELEMENT_ADDRESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READ_FILE_USN_DATA {
    pub MinMajorVersion: super::WORD,
    pub MaxMajorVersion: super::WORD,
}
pub const READ_THRESHOLDS: i32 = 209;
pub const READ_THRESHOLD_BUFFER_SIZE: i32 = 512;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type READ_USN_JOURNAL_DATA = READ_USN_JOURNAL_DATA_V1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READ_USN_JOURNAL_DATA_V0 {
    pub StartUsn: super::USN,
    pub ReasonMask: super::DWORD,
    pub ReturnOnlyOnClose: super::DWORD,
    pub Timeout: super::DWORDLONG,
    pub BytesToWaitFor: super::DWORDLONG,
    pub UsnJournalID: super::DWORDLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READ_USN_JOURNAL_DATA_V1 {
    pub StartUsn: super::USN,
    pub ReasonMask: super::DWORD,
    pub ReturnOnlyOnClose: super::DWORD,
    pub Timeout: super::DWORDLONG,
    pub BytesToWaitFor: super::DWORDLONG,
    pub UsnJournalID: super::DWORDLONG,
    pub MinMajorVersion: super::WORD,
    pub MaxMajorVersion: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REASSIGN_BLOCKS {
    pub Reserved: super::WORD,
    pub Count: super::WORD,
    pub BlockNumber: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for REASSIGN_BLOCKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct REASSIGN_BLOCKS_EX {
    pub Reserved: super::WORD,
    pub Count: super::WORD,
    pub BlockNumber: [super::LARGE_INTEGER; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for REASSIGN_BLOCKS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RECOVERED_READS_VALID: i32 = 4;
pub const RECOVERED_WRITES_VALID: i32 = 1;
pub type REFS_SMR_VOLUME_GC_ACTION = i32;
pub type REFS_SMR_VOLUME_GC_METHOD = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REFS_SMR_VOLUME_GC_PARAMETERS {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub Action: REFS_SMR_VOLUME_GC_ACTION,
    pub Method: REFS_SMR_VOLUME_GC_METHOD,
    pub IoGranularity: super::DWORD,
    pub CompressionFormat: super::DWORD,
    pub Unused: [super::DWORDLONG; 8],
}
#[cfg(feature = "shared")]
impl Default for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_SMR_VOLUME_GC_PARAMETERS_VERSION_V1: i32 = 1;
pub type REFS_SMR_VOLUME_GC_STATE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct REFS_SMR_VOLUME_INFO_OUTPUT {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub SizeOfRandomlyWritableTier: super::LARGE_INTEGER,
    pub FreeSpaceInRandomlyWritableTier: super::LARGE_INTEGER,
    pub SizeofSMRTier: super::LARGE_INTEGER,
    pub FreeSpaceInSMRTier: super::LARGE_INTEGER,
    pub UsableFreeSpaceInSMRTier: super::LARGE_INTEGER,
    pub VolumeGcState: REFS_SMR_VOLUME_GC_STATE,
    pub VolumeGcLastStatus: super::DWORD,
    pub CurrentGcBandFillPercentage: super::DWORD,
    pub Unused: [super::DWORDLONG; 6],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V0: i32 = 0;
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct REFS_VOLUME_DATA_BUFFER {
    pub ByteCount: super::DWORD,
    pub MajorVersion: super::DWORD,
    pub MinorVersion: super::DWORD,
    pub BytesPerPhysicalSector: super::DWORD,
    pub VolumeSerialNumber: super::LARGE_INTEGER,
    pub NumberSectors: super::LARGE_INTEGER,
    pub TotalClusters: super::LARGE_INTEGER,
    pub FreeClusters: super::LARGE_INTEGER,
    pub TotalReserved: super::LARGE_INTEGER,
    pub BytesPerSector: super::DWORD,
    pub BytesPerCluster: super::DWORD,
    pub MaximumSizeOfResidentFile: super::LARGE_INTEGER,
    pub FastTierDataFillRatio: super::WORD,
    pub SlowTierDataFillRatio: super::WORD,
    pub DestagesFastTierToSlowTierRate: super::DWORD,
    pub MetadataChecksumType: super::WORD,
    pub Reserved0: [super::BYTE; 6],
    pub DriverMajorVersion: super::DWORD,
    pub DriverMinorVersion: super::DWORD,
    pub Reserved: [super::LARGE_INTEGER; 7],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for REFS_VOLUME_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub RequestCapacity: super::DWORDLONG,
    pub ElementIdentifier: super::DWORD,
    pub Reserved: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct REPAIR_COPIES_INPUT {
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub FileOffset: super::LARGE_INTEGER,
    pub Length: super::DWORD,
    pub SourceCopy: super::DWORD,
    pub NumberOfRepairCopies: super::DWORD,
    pub RepairCopies: [super::DWORD; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for REPAIR_COPIES_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct REPAIR_COPIES_OUTPUT {
    pub Size: super::DWORD,
    pub Status: super::DWORD,
    pub ResumeFileOffset: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for REPAIR_COPIES_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REPLACE_ALTERNATE: i32 = 11;
pub const REPLACE_PRIMARY: i32 = 10;
pub const REQUEST_OPLOCK_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REQUEST_OPLOCK_INPUT_BUFFER {
    pub StructureVersion: super::WORD,
    pub StructureLength: super::WORD,
    pub RequestedOplockLevel: super::DWORD,
    pub Flags: super::DWORD,
}
pub const REQUEST_OPLOCK_INPUT_FLAG_ACK: i32 = 2;
pub const REQUEST_OPLOCK_INPUT_FLAG_COMPLETE_ACK_ON_CLOSE: i32 = 4;
pub const REQUEST_OPLOCK_INPUT_FLAG_REQUEST: i32 = 1;
pub const REQUEST_OPLOCK_INPUT_FLAG_RH_ALWAYS_BLOCK_UNTIL_ACK: i32 = 8;
pub const REQUEST_OPLOCK_INPUT_FLAG_RH_IGNORE_WRITES: i32 = 16;
pub const REQUEST_OPLOCK_INPUT_FLAG_RH_NO_NON_CACHED_IO: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REQUEST_OPLOCK_OUTPUT_BUFFER {
    pub StructureVersion: super::WORD,
    pub StructureLength: super::WORD,
    pub OriginalOplockLevel: super::DWORD,
    pub NewOplockLevel: super::DWORD,
    pub Flags: super::DWORD,
    pub AccessMode: super::ACCESS_MASK,
    pub ShareMode: super::WORD,
}
pub const REQUEST_OPLOCK_OUTPUT_FLAG_ACK_REQUIRED: i32 = 1;
pub const REQUEST_OPLOCK_OUTPUT_FLAG_MODES_PROVIDED: i32 = 2;
pub const REQUEST_OPLOCK_OUTPUT_FLAG_NON_CACHED_IO_PRESENT: i32 = 8;
pub const REQUEST_OPLOCK_OUTPUT_FLAG_WRITABLE_SECTION_PRESENT: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REQUEST_RAW_ENCRYPTED_DATA {
    pub FileOffset: super::LONGLONG,
    pub Length: super::DWORD,
}
pub const RETRACT_IEPORT: i32 = 3;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    pub ExtentCount: super::DWORD,
    pub StartingVcn: super::LARGE_INTEGER,
    pub Extents: [RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    pub NextVcn: super::LARGE_INTEGER,
    pub Lcn: super::LARGE_INTEGER,
    pub ReferenceCount: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RETRIEVAL_POINTERS_BUFFER {
    pub ExtentCount: super::DWORD,
    pub StartingVcn: super::LARGE_INTEGER,
    pub Extents: [RETRIEVAL_POINTERS_BUFFER_0; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RETRIEVAL_POINTERS_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RETRIEVAL_POINTERS_BUFFER_0 {
    pub NextVcn: super::LARGE_INTEGER,
    pub Lcn: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RETRIEVAL_POINTERS_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RETRIEVAL_POINTER_BASE {
    pub FileAreaOffset: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RETRIEVAL_POINTER_BASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RETRIEVAL_POINTER_COUNT {
    pub ExtentCount: super::DWORD,
}
pub const RETURN_SMART_STATUS: i32 = 218;
pub const REVISION_LENGTH: i32 = 4;
pub const RemovableMedia: MEDIA_TYPE = 11;
pub const RequestLocation: BIN_TYPES = 1;
pub const RequestSize: BIN_TYPES = 0;
pub const SAIT: STORAGE_MEDIA_TYPE = 93;
pub const SAVE_ATTRIBUTE_VALUES: i32 = 211;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceCount: super::DWORD,
    pub Devices: [SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    pub DeviceGuid: windows_core::GUID,
    pub DeviceNumber: super::DWORD,
    pub Flags: SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0,
    pub DeviceSize: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    pub fn ForcedByRegistry(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_ForcedByRegistry(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Initialized(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Initialized(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::DWORD {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_DEDICATED_MEMORY_STATE {
    pub ActivateState: super::BOOLEAN,
}
pub type SCM_BUS_FIRMWARE_ACTIVATION_STATE = i32;
pub type SCM_BUS_PROPERTY_ID = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_BUS_PROPERTY_QUERY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub PropertyId: SCM_BUS_PROPERTY_ID,
    pub QueryType: SCM_BUS_QUERY_TYPE,
    pub AdditionalParameters: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_BUS_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_BUS_PROPERTY_SET {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub PropertyId: SCM_BUS_PROPERTY_ID,
    pub SetType: SCM_BUS_SET_TYPE,
    pub AdditionalParameters: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_BUS_PROPERTY_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SCM_BUS_QUERY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub RuntimeFwActivationSupported: super::BOOLEAN,
    pub FirmwareActivationState: SCM_BUS_FIRMWARE_ACTIVATION_STATE,
    pub FirmwareActivationCapability: SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0,
    pub EstimatedFirmwareActivationTimeInUSecs: super::DWORDLONG,
    pub EstimatedProcessorAccessQuiesceTimeInUSecs: super::DWORDLONG,
    pub EstimatedIOAccessQuiesceTimeInUSecs: super::DWORDLONG,
    pub PlatformSupportedMaxIOAccessQuiesceTimeInUSecs: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    pub fn FwManagedIoQuiesceFwActivationSupported(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_FwManagedIoQuiesceFwActivationSupported(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn OsManagedIoQuiesceFwActivationSupported(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_OsManagedIoQuiesceFwActivationSupported(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn WarmResetBasedFwActivationSupported(&self) -> super::DWORD {
        (self._bitfield >> 2) & 1
    }
    pub fn set_WarmResetBasedFwActivationSupported(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::DWORD {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
pub type SCM_BUS_SET_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_INTERLEAVED_PD_INFO {
    pub DeviceHandle: super::DWORD,
    pub DeviceGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_LD_INTERLEAVE_SET_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub InterleaveSetSize: super::DWORD,
    pub InterleaveSet: [SCM_INTERLEAVED_PD_INFO; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_LD_INTERLEAVE_SET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_LOGICAL_DEVICES {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceCount: super::DWORD,
    pub Devices: [SCM_LOGICAL_DEVICE_INSTANCE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SCM_LOGICAL_DEVICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_LOGICAL_DEVICE_INSTANCE {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceGuid: windows_core::GUID,
    pub SymbolicLink: [super::WCHAR; 256],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SCM_LOGICAL_DEVICE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCM_MAX_SYMLINK_LEN_IN_CHARS: i32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_DESCRIPTOR_HEADER {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_DEVICE_HANDLE {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceGuid: windows_core::GUID,
    pub DeviceHandle: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_DEVICE_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceGuid: windows_core::GUID,
    pub UnsafeShutdownCount: super::DWORD,
    pub PersistentMemorySizeInBytes: super::DWORD64,
    pub VolatileMemorySizeInBytes: super::DWORD64,
    pub TotalMemorySizeInBytes: super::DWORD64,
    pub SlotNumber: super::DWORD,
    pub DeviceHandle: super::DWORD,
    pub PhysicalId: super::WORD,
    pub NumberOfFormatInterfaceCodes: super::BYTE,
    pub FormatInterfaceCodes: [super::WORD; 8],
    pub VendorId: super::DWORD,
    pub ProductId: super::DWORD,
    pub SubsystemDeviceId: super::DWORD,
    pub SubsystemVendorId: super::DWORD,
    pub ManufacturingLocation: super::BYTE,
    pub ManufacturingWeek: super::BYTE,
    pub ManufacturingYear: super::BYTE,
    pub SerialNumber4Byte: super::DWORD,
    pub SerialNumberLengthInChars: super::DWORD,
    pub SerialNumber: [super::CHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_DEVICE_SPECIFIC_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NumberOfProperties: super::DWORD,
    pub DeviceSpecificProperties: [SCM_PD_DEVICE_SPECIFIC_PROPERTY; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    pub Name: [super::WCHAR; 128],
    pub Value: super::LONGLONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_FIRMWARE_ACTIVATE {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub Slot: super::BYTE,
}
pub type SCM_PD_FIRMWARE_ACTIVATION_STATE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_FIRMWARE_DOWNLOAD {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub Slot: super::BYTE,
    pub Reserved: [super::BYTE; 3],
    pub Offset: super::DWORD64,
    pub FirmwareImageSizeInBytes: super::DWORD,
    pub FirmwareImage: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_FIRMWARE_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub ActiveSlot: super::BYTE,
    pub NextActiveSlot: super::BYTE,
    pub SlotCount: super::BYTE,
    pub Slots: [SCM_PD_FIRMWARE_SLOT_INFO; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCM_PD_FIRMWARE_LAST_DOWNLOAD: i32 = 1;
pub const SCM_PD_FIRMWARE_REVISION_LENGTH_BYTES: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_FIRMWARE_SLOT_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub SlotNumber: super::BYTE,
    pub _bitfield: u8,
    pub Reserved1: [super::BYTE; 6],
    pub Revision: [super::BYTE; 32],
}
#[cfg(feature = "shared")]
impl SCM_PD_FIRMWARE_SLOT_INFO {
    pub fn ReadOnly(&self) -> super::BYTE {
        (self._bitfield) & 1
    }
    pub fn set_ReadOnly(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::BYTE {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_FRU_ID_STRING {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub IdentifierSize: super::DWORD,
    pub Identifier: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_FRU_ID_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_HEALTH_NOTIFICATION_DATA {
    pub DeviceGuid: windows_core::GUID,
}
pub type SCM_PD_HEALTH_STATUS = i32;
pub type SCM_PD_LAST_FW_ACTIVATION_STATUS = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_LOCATION_STRING {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Location: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SCM_PD_LOCATION_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_MANAGEMENT_STATUS {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Health: SCM_PD_HEALTH_STATUS,
    pub NumberOfOperationalStatus: super::DWORD,
    pub NumberOfAdditionalReasons: super::DWORD,
    pub OperationalStatus: [SCM_PD_OPERATIONAL_STATUS; 16],
    pub AdditionalReasons: [SCM_PD_OPERATIONAL_STATUS_REASON; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_MANAGEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCM_PD_MAX_OPERATIONAL_STATUS: i32 = 16;
pub type SCM_PD_MEDIA_REINITIALIZATION_STATUS = i32;
#[cfg(feature = "shared")]
pub const SCM_PD_MEMORY_SIZE_UNKNOWN: super::DWORD64 = 18446744073709551615;
pub type SCM_PD_OPERATIONAL_STATUS = i32;
pub type SCM_PD_OPERATIONAL_STATUS_REASON = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_PASSTHROUGH_INPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub ProtocolGuid: windows_core::GUID,
    pub DataSize: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_PASSTHROUGH_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    pub Opcode: super::DWORD,
    pub OpcodeParametersLength: super::DWORD,
    pub OpcodeParameters: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    pub GeneralStatus: super::WORD,
    pub ExtendedStatus: super::WORD,
    pub OutputDataLength: super::DWORD,
    pub OutputData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_PASSTHROUGH_OUTPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub ProtocolGuid: windows_core::GUID,
    pub DataSize: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_PASSTHROUGH_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SCM_PD_PROPERTY_ID = i32;
pub const SCM_PD_PROPERTY_NAME_LENGTH_IN_CHARS: i32 = 128;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_PROPERTY_QUERY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub PropertyId: SCM_PD_PROPERTY_ID,
    pub QueryType: SCM_PD_QUERY_TYPE,
    pub AdditionalParameters: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PD_PROPERTY_SET {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub PropertyId: SCM_PD_PROPERTY_ID,
    pub SetType: SCM_PD_SET_TYPE,
    pub AdditionalParameters: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_PD_PROPERTY_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SCM_PD_QUERY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_REINITIALIZE_MEDIA_INPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Options: SCM_PD_REINITIALIZE_MEDIA_INPUT_0,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    pub fn Overwrite(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_Overwrite(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Status: SCM_PD_MEDIA_REINITIALIZATION_STATUS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    pub ArmState: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub LastFirmwareActivationStatus: SCM_PD_LAST_FW_ACTIVATION_STATUS,
    pub FirmwareActivationState: SCM_PD_FIRMWARE_ACTIVATION_STATE,
}
pub type SCM_PD_SET_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PHYSICAL_DEVICES {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceCount: super::DWORD,
    pub Devices: [SCM_PHYSICAL_DEVICE_INSTANCE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SCM_PHYSICAL_DEVICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_PHYSICAL_DEVICE_INSTANCE {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NfitHandle: super::DWORD,
    pub SymbolicLink: [super::WCHAR; 256],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCM_REGION {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub NfitHandle: super::DWORD,
    pub LogicalDeviceGuid: windows_core::GUID,
    pub AddressRangeType: windows_core::GUID,
    pub AssociatedId: super::DWORD,
    pub Length: super::DWORD64,
    pub StartingDPA: super::DWORD64,
    pub BaseSPA: super::DWORD64,
    pub SPAOffset: super::DWORD64,
    pub RegionOffset: super::DWORD64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCM_REGIONS {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub RegionCount: super::DWORD,
    pub Regions: [SCM_REGION; 1],
}
#[cfg(feature = "shared")]
impl Default for SCM_REGIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SCM_REGION_FLAG = i32;
#[cfg(feature = "shared")]
pub const SCM_REGION_SPA_UNKNOWN: super::DWORD64 = 18446744073709551615;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SD_CHANGE_MACHINE_SID_INPUT {
    pub CurrentMachineSIDOffset: super::WORD,
    pub CurrentMachineSIDLength: super::WORD,
    pub NewMachineSIDOffset: super::WORD,
    pub NewMachineSIDLength: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SD_CHANGE_MACHINE_SID_OUTPUT {
    pub NumSDChangedSuccess: super::DWORDLONG,
    pub NumSDChangedFail: super::DWORDLONG,
    pub NumSDUnused: super::DWORDLONG,
    pub NumSDTotal: super::DWORDLONG,
    pub NumMftSDChangedSuccess: super::DWORDLONG,
    pub NumMftSDChangedFail: super::DWORDLONG,
    pub NumMftSDTotal: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SD_ENUM_SDS_ENTRY {
    pub Hash: super::DWORD,
    pub SecurityId: super::DWORD,
    pub Offset: super::DWORDLONG,
    pub Length: super::DWORD,
    pub Descriptor: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SD_ENUM_SDS_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SD_ENUM_SDS_INPUT {
    pub StartingOffset: super::DWORDLONG,
    pub MaxSDEntriesToReturn: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SD_ENUM_SDS_OUTPUT {
    pub NextOffset: super::DWORDLONG,
    pub NumSDEntriesReturned: super::DWORDLONG,
    pub NumSDBytesReturned: super::DWORDLONG,
    pub SDEntry: [SD_ENUM_SDS_ENTRY; 1],
}
#[cfg(feature = "shared")]
impl Default for SD_ENUM_SDS_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SD_GLOBAL_CHANGE_INPUT {
    pub Flags: super::DWORD,
    pub ChangeType: super::DWORD,
    pub Anonymous: SD_GLOBAL_CHANGE_INPUT_0,
}
#[cfg(feature = "shared")]
impl Default for SD_GLOBAL_CHANGE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union SD_GLOBAL_CHANGE_INPUT_0 {
    pub SdChange: SD_CHANGE_MACHINE_SID_INPUT,
    pub SdQueryStats: SD_QUERY_STATS_INPUT,
    pub SdEnumSds: SD_ENUM_SDS_INPUT,
}
#[cfg(feature = "shared")]
impl Default for SD_GLOBAL_CHANGE_INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SD_GLOBAL_CHANGE_OUTPUT {
    pub Flags: super::DWORD,
    pub ChangeType: super::DWORD,
    pub Anonymous: SD_GLOBAL_CHANGE_OUTPUT_0,
}
#[cfg(feature = "shared")]
impl Default for SD_GLOBAL_CHANGE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union SD_GLOBAL_CHANGE_OUTPUT_0 {
    pub SdChange: SD_CHANGE_MACHINE_SID_OUTPUT,
    pub SdQueryStats: SD_QUERY_STATS_OUTPUT,
    pub SdEnumSds: SD_ENUM_SDS_OUTPUT,
}
#[cfg(feature = "shared")]
impl Default for SD_GLOBAL_CHANGE_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SD_GLOBAL_CHANGE_TYPE_ENUM_SDS: i32 = 131072;
pub const SD_GLOBAL_CHANGE_TYPE_MACHINE_SID: i32 = 1;
pub const SD_GLOBAL_CHANGE_TYPE_QUERY_STATS: i32 = 65536;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SD_QUERY_STATS_INPUT {
    pub Reserved: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SD_QUERY_STATS_OUTPUT {
    pub SdsStreamSize: super::DWORDLONG,
    pub SdsAllocationSize: super::DWORDLONG,
    pub SiiStreamSize: super::DWORDLONG,
    pub SiiAllocationSize: super::DWORDLONG,
    pub SdhStreamSize: super::DWORDLONG,
    pub SdhAllocationSize: super::DWORDLONG,
    pub NumSDTotal: super::DWORDLONG,
    pub NumSDUnused: super::DWORDLONG,
}
pub const SEARCH_ALL: i32 = 0;
pub const SEARCH_ALL_NO_SEQ: i32 = 4;
pub const SEARCH_ALTERNATE: i32 = 2;
pub const SEARCH_ALT_NO_SEQ: i32 = 6;
pub const SEARCH_PRIMARY: i32 = 1;
pub const SEARCH_PRI_NO_SEQ: i32 = 5;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SENDCMDINPARAMS {
    pub cBufferSize: super::DWORD,
    pub irDriveRegs: IDEREGS,
    pub bDriveNumber: super::BYTE,
    pub bReserved: [super::BYTE; 3],
    pub dwReserved: [super::DWORD; 4],
    pub bBuffer: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SENDCMDINPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SENDCMDOUTPARAMS {
    pub cBufferSize: super::DWORD,
    pub DriverStatus: DRIVERSTATUS,
    pub bBuffer: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SENDCMDOUTPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERIAL_NUMBER_LENGTH: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    pub Flags: super::DWORD,
    pub AlignmentShift: super::DWORD,
    pub FileOffsetToAlign: super::DWORDLONG,
    pub FallbackAlignmentShift: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SET_DISK_ATTRIBUTES {
    pub Version: super::DWORD,
    pub Persist: super::BOOLEAN,
    pub Reserved1: [super::BYTE; 3],
    pub Attributes: super::DWORDLONG,
    pub AttributesMask: super::DWORDLONG,
    pub Reserved2: [super::DWORD; 4],
}
#[cfg(feature = "shared")]
impl Default for SET_DISK_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_PARTITION_INFORMATION {
    pub PartitionType: super::BYTE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SET_PARTITION_INFORMATION_EX {
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: SET_PARTITION_INFORMATION_EX_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SET_PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union SET_PARTITION_INFORMATION_EX_0 {
    pub Mbr: SET_PARTITION_INFORMATION_MBR,
    pub Gpt: SET_PARTITION_INFORMATION_GPT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SET_PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SET_PARTITION_INFORMATION_GPT = PARTITION_INFORMATION_GPT;
#[cfg(feature = "shared")]
pub type SET_PARTITION_INFORMATION_MBR = SET_PARTITION_INFORMATION;
pub const SET_PURGE_FAILURE_MODE_DISABLED: i32 = 2;
pub const SET_PURGE_FAILURE_MODE_ENABLED: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_PURGE_FAILURE_MODE_INPUT {
    pub Flags: super::DWORD,
}
pub const SET_REPAIR_DISABLED_AND_BUGCHECK_ON_CORRUPT: i32 = 16;
pub const SET_REPAIR_ENABLED: i32 = 1;
pub const SET_REPAIR_VALID_MASK: i32 = 25;
pub const SET_REPAIR_WARN_ABOUT_DATA_LOSS: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHRINK_VOLUME_INFORMATION {
    pub ShrinkRequestType: SHRINK_VOLUME_REQUEST_TYPES,
    pub Flags: super::DWORDLONG,
    pub NewNumberOfSectors: super::LONGLONG,
}
pub type SHRINK_VOLUME_REQUEST_TYPES = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SI_COPYFILE {
    pub SourceFileNameLength: super::DWORD,
    pub DestinationFileNameLength: super::DWORD,
    pub Flags: super::DWORD,
    pub FileNameBuffer: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SI_COPYFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SMART_ABORT_OFFLINE_SELFTEST: i32 = 127;
pub const SMART_CMD: i32 = 176;
pub const SMART_CYL_HI: i32 = 194;
pub const SMART_CYL_LOW: i32 = 79;
pub const SMART_ERROR_NO_MEM: i32 = 7;
pub const SMART_EXTENDED_SELFTEST_CAPTIVE: i32 = 130;
pub const SMART_EXTENDED_SELFTEST_OFFLINE: i32 = 2;
pub const SMART_GET_VERSION: i32 = 475264;
pub const SMART_IDE_ERROR: i32 = 1;
pub const SMART_INVALID_BUFFER: i32 = 4;
pub const SMART_INVALID_COMMAND: i32 = 3;
pub const SMART_INVALID_DRIVE: i32 = 5;
pub const SMART_INVALID_FLAG: i32 = 2;
pub const SMART_INVALID_IOCTL: i32 = 6;
pub const SMART_INVALID_REGISTER: i32 = 8;
pub const SMART_LOG_SECTOR_SIZE: i32 = 512;
pub const SMART_NOT_SUPPORTED: i32 = 9;
pub const SMART_NO_ERROR: i32 = 0;
pub const SMART_NO_IDE_DEVICE: i32 = 10;
pub const SMART_OFFLINE_ROUTINE_OFFLINE: i32 = 0;
pub const SMART_RCV_DRIVE_DATA: i32 = 508040;
pub const SMART_RCV_DRIVE_DATA_EX: i32 = 458892;
pub const SMART_READ_LOG: i32 = 213;
pub const SMART_SEND_DRIVE_COMMAND: i32 = 508036;
pub const SMART_SHORT_SELFTEST_CAPTIVE: i32 = 129;
pub const SMART_SHORT_SELFTEST_OFFLINE: i32 = 1;
pub const SMART_WRITE_LOG: i32 = 214;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    pub Version: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    pub cEntriesPurged: super::DWORD,
}
pub const SONY_12_WO: STORAGE_MEDIA_TYPE = 66;
pub const SONY_D2: STORAGE_MEDIA_TYPE = 49;
pub const SONY_DTF: STORAGE_MEDIA_TYPE = 46;
pub const SPACES_TRACKED_OFFSET_HEADER_FLAG: i32 = 2;
pub const SRB_TYPE_SCSI_REQUEST_BLOCK: i32 = 0;
pub const SRB_TYPE_STORAGE_REQUEST_BLOCK: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STARTING_LCN_INPUT_BUFFER {
    pub StartingLcn: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STARTING_LCN_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STARTING_LCN_INPUT_BUFFER_EX {
    pub StartingLcn: super::LARGE_INTEGER,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STARTING_LCN_INPUT_BUFFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STARTING_VCN_INPUT_BUFFER {
    pub StartingVcn: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STARTING_VCN_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STK_9840: STORAGE_MEDIA_TYPE = 85;
pub const STK_9940: STORAGE_MEDIA_TYPE = 92;
pub const STK_DATA_D3: STORAGE_MEDIA_TYPE = 45;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub BytesPerCacheLine: super::DWORD,
    pub BytesOffsetForCacheAlignment: super::DWORD,
    pub BytesPerLogicalSector: super::DWORD,
    pub BytesPerPhysicalSector: super::DWORD,
    pub BytesOffsetForSectorAlignment: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ADAPTER_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub MaximumTransferLength: super::DWORD,
    pub MaximumPhysicalPages: super::DWORD,
    pub AlignmentMask: super::DWORD,
    pub AdapterUsesPio: super::BOOLEAN,
    pub AdapterScansDown: super::BOOLEAN,
    pub CommandQueueing: super::BOOLEAN,
    pub AcceleratedTransfer: super::BOOLEAN,
    pub BusType: super::BYTE,
    pub BusMajorVersion: super::WORD,
    pub BusMinorVersion: super::WORD,
    pub SrbType: super::BYTE,
    pub AddressType: super::BYTE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_ADAPTER_SERIAL_NUMBER {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub SerialNumber: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_MAX_LENGTH: i32 = 128;
#[cfg(target_arch = "x86")]
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_SIZE: u32 = 264;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_SIZE: u64 = 264;
#[cfg(target_arch = "x86")]
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_VERSION: u32 = 264;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_VERSION: u64 = 264;
pub const STORAGE_ADDRESS_TYPE_BTL8: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_ALLOCATE_BC_STREAM_INPUT {
    pub Version: super::DWORD,
    pub RequestsPerPeriod: super::DWORD,
    pub Period: super::DWORD,
    pub RetryFailures: super::BOOLEAN,
    pub Discardable: super::BOOLEAN,
    pub Reserved1: [super::BOOLEAN; 2],
    pub AccessType: super::DWORD,
    pub AccessMode: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: super::DWORDLONG,
    pub NumOutStandingRequests: super::DWORD,
}
pub type STORAGE_ASSOCIATION_TYPE = i32;
pub const STORAGE_ATTRIBUTE_ASYNC_EVENT_NOTIFICATION: i32 = 16;
pub const STORAGE_ATTRIBUTE_BLOCK_IO: i32 = 2;
pub const STORAGE_ATTRIBUTE_BYTE_ADDRESSABLE_IO: i32 = 1;
pub const STORAGE_ATTRIBUTE_DYNAMIC_PERSISTENCE: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ATTRIBUTE_MGMT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Action: STORAGE_ATTRIBUTE_MGMT_ACTION,
    pub Attribute: super::DWORD,
}
pub type STORAGE_ATTRIBUTE_MGMT_ACTION = i32;
pub const STORAGE_ATTRIBUTE_PERF_SIZE_INDEPENDENT: i32 = 32;
pub const STORAGE_ATTRIBUTE_VOLATILE: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_BREAK_RESERVATION_REQUEST {
    pub Length: super::DWORD,
    pub _unused: super::BYTE,
    pub PathId: super::BYTE,
    pub TargetId: super::BYTE,
    pub Lun: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_BUS_RESET_REQUEST {
    pub PathId: super::BYTE,
}
pub type STORAGE_BUS_TYPE = i32;
pub type STORAGE_COMPONENT_HEALTH_STATUS = i32;
pub const STORAGE_COMPONENT_ROLE_CACHE: i32 = 1;
pub const STORAGE_COMPONENT_ROLE_DATA: i32 = 4;
pub const STORAGE_COMPONENT_ROLE_TIERING: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_COUNTER {
    pub Type: STORAGE_COUNTER_TYPE,
    pub Value: STORAGE_COUNTER_0,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_COUNTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_COUNTER_0 {
    pub ManufactureDate: STORAGE_COUNTER_0_0,
    pub AsUlonglong: super::DWORDLONG,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_COUNTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_COUNTER_0_0 {
    pub Week: super::DWORD,
    pub Year: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_COUNTERS {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NumberOfCounters: super::DWORD,
    pub Counters: [STORAGE_COUNTER; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const STORAGE_COUNTERS_VERSION_V1: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const STORAGE_COUNTERS_VERSION_V1: u64 = 32;
pub type STORAGE_COUNTER_TYPE = i32;
pub const STORAGE_CRASH_TELEMETRY_REGKEY: windows_core::PCWSTR = windows_core::w!("\\Registry\\Machine\\System\\CurrentControlSet\\Control\\CrashControl\\StorageTelemetry");
pub type STORAGE_CRYPTO_ALGORITHM_ID = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_CRYPTO_CAPABILITY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub CryptoCapabilityIndex: super::DWORD,
    pub AlgorithmId: STORAGE_CRYPTO_ALGORITHM_ID,
    pub KeySize: STORAGE_CRYPTO_KEY_SIZE,
    pub DataUnitSizeBitmask: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_CRYPTO_CAPABILITY_V2 {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub CryptoCapabilityIndex: super::DWORD,
    pub AlgorithmId: STORAGE_CRYPTO_ALGORITHM_ID,
    pub KeySize: STORAGE_CRYPTO_KEY_SIZE,
    pub DataUnitSizeBitmask: super::DWORD,
    pub MaxIVBitSize: super::WORD,
    pub Reserved: super::WORD,
    pub SecurityComplianceBitmask: STORAGE_SECURITY_COMPLIANCE_BITMASK,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_CRYPTO_CAPABILITY_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_CRYPTO_CAPABILITY_VERSION_1: i32 = 1;
pub const STORAGE_CRYPTO_CAPABILITY_VERSION_2: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_CRYPTO_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NumKeysSupported: super::DWORD,
    pub NumCryptoCapabilities: super::DWORD,
    pub CryptoCapabilities: [STORAGE_CRYPTO_CAPABILITY; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_CRYPTO_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_CRYPTO_DESCRIPTOR_V2 {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NumKeysSupported: super::DWORD,
    pub NumCryptoCapabilities: super::DWORD,
    pub IceType: STORAGE_ICE_TYPE,
    pub SecurityComplianceBitmask: STORAGE_SECURITY_COMPLIANCE_BITMASK,
    pub KeyTypeBitmask: STORAGE_CRYPTO_KEY_TYPE,
    pub CryptoCapabilities: [STORAGE_CRYPTO_CAPABILITY_V2; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_CRYPTO_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_CRYPTO_DESCRIPTOR_VERSION_1: i32 = 1;
pub const STORAGE_CRYPTO_DESCRIPTOR_VERSION_2: i32 = 2;
pub type STORAGE_CRYPTO_KEY_SIZE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_CRYPTO_KEY_TYPE {
    pub Anonymous: STORAGE_CRYPTO_KEY_TYPE_0,
    pub AsUchar: super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_CRYPTO_KEY_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_CRYPTO_KEY_TYPE_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl STORAGE_CRYPTO_KEY_TYPE_0 {
    pub fn DirectKey(&self) -> super::BYTE {
        (self._bitfield) & 1
    }
    pub fn set_DirectKey(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn PlatformWrappedKey(&self) -> super::BYTE {
        (self._bitfield >> 1) & 1
    }
    pub fn set_PlatformWrappedKey(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn PlutonWrappedKey(&self) -> super::BYTE {
        (self._bitfield >> 2) & 1
    }
    pub fn set_PlutonWrappedKey(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::BYTE {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DESCRIPTOR_HEADER {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Attributes: super::DWORD64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DEVICE_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceType: super::BYTE,
    pub DeviceTypeModifier: super::BYTE,
    pub RemovableMedia: super::BOOLEAN,
    pub CommandQueueing: super::BOOLEAN,
    pub VendorIdOffset: super::DWORD,
    pub ProductIdOffset: super::DWORD,
    pub ProductRevisionOffset: super::DWORD,
    pub SerialNumberOffset: super::DWORD,
    pub BusType: STORAGE_BUS_TYPE,
    pub RawPropertiesLength: super::DWORD,
    pub RawDeviceProperties: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NumberOfFaultDomains: super::DWORD,
    pub FaultDomainIds: [windows_core::GUID; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_DEVICE_FLAGS_PAGE_83_DEVICEGUID: i32 = 4;
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_CONFLICT: i32 = 1;
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_NOHWID: i32 = 2;
pub type STORAGE_DEVICE_FORM_FACTOR = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DEVICE_ID_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NumberOfIdentifiers: super::DWORD,
    pub Identifiers: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub LunMaxIoCount: super::DWORD,
    pub AdapterMaxIoCount: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub State: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Location: DEVICE_LOCATION,
    pub StringOffset: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_DEVICE_MANAGEMENT_STATUS {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Health: STORAGE_DISK_HEALTH_STATUS,
    pub NumberOfOperationalStatus: super::DWORD,
    pub NumberOfAdditionalReasons: super::DWORD,
    pub OperationalStatus: [STORAGE_DISK_OPERATIONAL_STATUS; 16],
    pub AdditionalReasons: [STORAGE_OPERATIONAL_REASON; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_DEVICE_MANAGEMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_DEVICE_MAX_OPERATIONAL_STATUS: i32 = 16;
pub const STORAGE_DEVICE_NUMA_NODE_UNKNOWN: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_NUMA_PROPERTY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NumaNode: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_NUMBER {
    pub DeviceType: super::DWORD,
    pub DeviceNumber: super::DWORD,
    pub PartitionNumber: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DEVICE_NUMBERS {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NumberOfDevices: super::DWORD,
    pub Devices: [STORAGE_DEVICE_NUMBER; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_DEVICE_NUMBERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_NUMBER_EX {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub DeviceType: super::DWORD,
    pub DeviceNumber: super::DWORD,
    pub DeviceGuid: windows_core::GUID,
    pub PartitionNumber: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_POWER_CAP {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Units: STORAGE_DEVICE_POWER_CAP_UNITS,
    pub MaxPower: super::DWORDLONG,
}
pub type STORAGE_DEVICE_POWER_CAP_UNITS = i32;
pub const STORAGE_DEVICE_POWER_CAP_VERSION_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NameOffset: super::DWORD,
    pub NumberOfLogicalCopies: super::DWORD,
    pub NumberOfPhysicalCopies: super::DWORD,
    pub PhysicalDiskRedundancy: super::DWORD,
    pub NumberOfColumns: super::DWORD,
    pub Interleave: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub SupportsSelfEncryption: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2 {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub SupportsSelfEncryption: super::BOOLEAN,
    pub EncryptionType: STORAGE_ENCRYPTION_TYPE,
}
pub const STORAGE_DEVICE_TELEMETRY_REGKEY: windows_core::PCWSTR = windows_core::w!("\\Registry\\Machine\\System\\CurrentControlSet\\Control\\Storage\\StorageTelemetry");
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DEVICE_TIERING_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub TotalNumberOfTiers: super::DWORD,
    pub NumberOfTiersReturned: super::DWORD,
    pub Tiers: [STORAGE_TIER; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub UnsafeShutdownCount: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DIAGNOSTIC_DATA {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub ProviderId: windows_core::GUID,
    pub BufferSize: super::DWORD,
    pub Reserved: super::DWORD,
    pub DiagnosticDataBuffer: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_DIAGNOSTIC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_DIAGNOSTIC_FLAG_ADAPTER_REQUEST: i32 = 1;
pub type STORAGE_DIAGNOSTIC_LEVEL = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DIAGNOSTIC_REQUEST {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub TargetType: STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: STORAGE_DIAGNOSTIC_LEVEL,
}
pub type STORAGE_DIAGNOSTIC_TARGET_TYPE = i32;
pub type STORAGE_DISK_HEALTH_STATUS = i32;
pub type STORAGE_DISK_OPERATIONAL_STATUS = i32;
pub type STORAGE_ENCRYPTION_TYPE = i32;
pub const STORAGE_EVENT_ALL: i32 = 7;
pub const STORAGE_EVENT_DEVICE_OPERATION: i32 = 4;
pub const STORAGE_EVENT_DEVICE_STATUS: i32 = 2;
pub const STORAGE_EVENT_MEDIA_STATUS: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_EVENT_NOTIFICATION {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Events: super::DWORDLONG,
}
pub const STORAGE_EVENT_NOTIFICATION_VERSION_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_FAILURE_PREDICTION_CONFIG {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Set: super::BOOLEAN,
    pub Enabled: super::BOOLEAN,
    pub Reserved: super::WORD,
}
pub const STORAGE_FAILURE_PREDICTION_CONFIG_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_FEATURE_SUPPORT {
    pub Size: super::DWORD,
    pub Version: super::DWORD,
    pub Flags: STORAGE_FEATURE_SUPPORT_0,
    pub Reserved: [super::DWORDLONG; 6],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FEATURE_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_FEATURE_SUPPORT_0 {
    pub Anonymous: STORAGE_FEATURE_SUPPORT_0_0,
    pub AsUlonglong: super::DWORDLONG,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FEATURE_SUPPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_FEATURE_SUPPORT_0_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "shared")]
impl STORAGE_FEATURE_SUPPORT_0_0 {
    pub fn StorMQMiniportsSupported(&self) -> super::DWORDLONG {
        (self._bitfield) & 1
    }
    pub fn set_StorMQMiniportsSupported(&mut self, value: super::DWORDLONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::DWORDLONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::DWORDLONG) {
        self._bitfield = (self._bitfield & !(9223372036854775807 << 1)) | ((value & 9223372036854775807) << 1);
    }
}
pub const STORAGE_FEATURE_SUPPORT_V1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_FRU_ID_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub IdentifierSize: super::DWORD,
    pub Identifier: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_FRU_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: super::DWORD,
    pub MinimumPeriod: super::DWORD,
    pub MaximumRequestSize: super::DWORDLONG,
    pub EstimatedTimePerRequest: super::DWORD,
    pub NumOutStandingRequests: super::DWORD,
    pub RequestSize: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_HOTPLUG_INFO {
    pub Size: super::DWORD,
    pub MediaRemovable: super::BOOLEAN,
    pub MediaHotplug: super::BOOLEAN,
    pub DeviceHotplug: super::BOOLEAN,
    pub WriteCacheEnableOverride: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_HW_BOOT_PARTITION_ACTIVATE {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub BPID: super::BYTE,
    pub Reserved: [super::BYTE; 3],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_BOOT_PARTITION_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_HW_BOOT_PARTITION_ACTIVATE_STRUCTURE_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_HW_BOOT_PARTITION_DOWNLOAD {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub BPID: super::BYTE,
    pub Reserved: [super::BYTE; 3],
    pub Offset: super::DWORDLONG,
    pub BufferSize: super::DWORDLONG,
    pub ImageBuffer: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_BOOT_PARTITION_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_HW_BOOT_PARTITION_DOWNLOAD_STRUCTURE_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_HW_BOOT_PARTITION_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub BPSZ: super::DWORDLONG,
    pub Flags: super::DWORD,
    pub ImagePayloadAlignment: super::DWORD,
    pub ImagePayloadMaxSize: super::DWORD,
    pub SlotCount: super::BYTE,
    pub ABPID: super::BYTE,
}
pub const STORAGE_HW_BOOT_PARTITION_INFO_STRUCTURE_VERSION_V1: i32 = 1;
pub const STORAGE_HW_BOOT_PARTITION_REQUEST_ACTIVATE_EXISTING_BOOT_PARTITION: u32 = 2147483648;
pub const STORAGE_HW_BOOT_PARTITION_REQUEST_FLAG_CONTROLLER: i32 = 1;
pub const STORAGE_HW_BOOT_PARTITION_REQUEST_REPLACE_EXISTING_BOOT_PARTITION: i32 = 1073741824;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_HW_CRYPTO_CAPABILITY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub CryptoCapabilityIndex: super::DWORD,
    pub AlgorithmId: STORAGE_CRYPTO_ALGORITHM_ID,
    pub KeySize: STORAGE_CRYPTO_KEY_SIZE,
    pub DataUnitSizeBitmask: super::DWORD,
    pub MaxIVBitSize: super::WORD,
    pub Reserved: super::WORD,
    pub SecurityComplianceBitmask: STORAGE_SECURITY_COMPLIANCE_BITMASK,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_CRYPTO_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_HW_CRYPTO_CAPABILITY_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_HW_CRYPTO_DESCRIPTOR {
    pub Header: STORAGE_DESCRIPTOR_HEADER,
    pub NumKeysSupported: super::DWORD,
    pub NumCryptoCapabilities: super::DWORD,
    pub OffsetToCryptoCapabilities: super::DWORD,
    pub SizeOfCryptoCapability: super::DWORD,
    pub IceType: STORAGE_ICE_TYPE,
    pub SecurityComplianceBitmask: STORAGE_SECURITY_COMPLIANCE_BITMASK,
    pub KeyTypeBitmask: STORAGE_CRYPTO_KEY_TYPE,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_CRYPTO_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_HW_CRYPTO_DESCRIPTOR_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub EnduranceInfo: STORAGE_HW_ENDURANCE_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_HW_ENDURANCE_INFO {
    pub ValidFields: super::DWORD,
    pub GroupId: super::DWORD,
    pub Flags: STORAGE_HW_ENDURANCE_INFO_0,
    pub LifePercentage: super::DWORD,
    pub BytesReadCount: [super::BYTE; 16],
    pub ByteWriteCount: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_HW_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl STORAGE_HW_ENDURANCE_INFO_0 {
    pub fn Shared(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_Shared(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::DWORD {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_ACTIVATE {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub Slot: super::BYTE,
    pub Reserved0: [super::BYTE; 3],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_DOWNLOAD {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub Slot: super::BYTE,
    pub Reserved: [super::BYTE; 3],
    pub Offset: super::DWORDLONG,
    pub BufferSize: super::DWORDLONG,
    pub ImageBuffer: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub Slot: super::BYTE,
    pub Reserved: [super::BYTE; 3],
    pub Offset: super::DWORDLONG,
    pub BufferSize: super::DWORDLONG,
    pub ImageSize: super::DWORD,
    pub Reserved2: super::DWORD,
    pub ImageBuffer: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub _bitfield: u8,
    pub SlotCount: super::BYTE,
    pub ActiveSlot: super::BYTE,
    pub PendingActivateSlot: super::BYTE,
    pub FirmwareShared: super::BOOLEAN,
    pub Reserved: [super::BYTE; 3],
    pub ImagePayloadAlignment: super::DWORD,
    pub ImagePayloadMaxSize: super::DWORD,
    pub Slot: [STORAGE_HW_FIRMWARE_SLOT_INFO; 1],
}
#[cfg(feature = "shared")]
impl STORAGE_HW_FIRMWARE_INFO {
    pub fn SupportUpgrade(&self) -> super::BYTE {
        (self._bitfield) & 1
    }
    pub fn set_SupportUpgrade(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::BYTE {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_INFO_QUERY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub Reserved: super::DWORD,
}
pub const STORAGE_HW_FIRMWARE_INVALID_SLOT: i32 = 255;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_CONTROLLER: i32 = 1;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: i32 = 4;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: i32 = 2;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_REPLACE_AND_SWITCH_UPON_RESET: i32 = 536870912;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_REPLACE_EXISTING_IMAGE: i32 = 1073741824;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648;
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_SWITCH_TO_FIRMWARE_WITHOUT_RESET: i32 = 268435456;
pub const STORAGE_HW_FIRMWARE_REVISION_LENGTH: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_HW_FIRMWARE_SLOT_INFO {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub SlotNumber: super::BYTE,
    pub _bitfield: u8,
    pub Reserved1: [super::BYTE; 6],
    pub Revision: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl STORAGE_HW_FIRMWARE_SLOT_INFO {
    pub fn ReadOnly(&self) -> super::BYTE {
        (self._bitfield) & 1
    }
    pub fn set_ReadOnly(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::BYTE {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STORAGE_ICE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_IDENTIFIER {
    pub CodeSet: STORAGE_IDENTIFIER_CODE_SET,
    pub Type: STORAGE_IDENTIFIER_TYPE,
    pub IdentifierSize: super::WORD,
    pub NextOffset: super::WORD,
    pub Association: STORAGE_ASSOCIATION_TYPE,
    pub Identifier: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STORAGE_IDENTIFIER_CODE_SET = i32;
pub type STORAGE_IDENTIFIER_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_IDLE_POWER {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub _bitfield: u32,
    pub D3IdleTimeout: super::DWORD,
}
#[cfg(feature = "shared")]
impl STORAGE_IDLE_POWER {
    pub fn WakeCapableHint(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_WakeCapableHint(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn D3ColdSupported(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_D3ColdSupported(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::DWORD {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_IDLE_POWERUP_REASON {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub PowerupReason: STORAGE_POWERUP_REASON_TYPE,
}
pub const STORAGE_IDLE_POWERUP_REASON_VERSION_V1: i32 = 1;
pub type STORAGE_ID_NAA_FORMAT = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    pub Size: super::DWORD,
    pub Version: super::DWORD,
    pub _bitfield1: u8,
    pub Reserved1: [super::BYTE; 3],
    pub _bitfield2: u8,
    pub Reserved3: [super::BYTE; 3],
    pub AvailableMappingResources: super::DWORDLONG,
    pub UsedMappingResources: super::DWORDLONG,
}
#[cfg(feature = "shared")]
impl STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    pub fn AvailableMappingResourcesValid(&self) -> super::BYTE {
        (self._bitfield1) & 1
    }
    pub fn set_AvailableMappingResourcesValid(&mut self, value: super::BYTE) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn UsedMappingResourcesValid(&self) -> super::BYTE {
        (self._bitfield1 >> 1) & 1
    }
    pub fn set_UsedMappingResourcesValid(&mut self, value: super::BYTE) {
        self._bitfield1 = (self._bitfield1 & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved0(&self) -> super::BYTE {
        self._bitfield1 >> 2
    }
    pub fn set_Reserved0(&mut self, value: super::BYTE) {
        self._bitfield1 = (self._bitfield1 & !(63 << 2)) | ((value & 63) << 2);
    }
    pub fn AvailableMappingResourcesScope(&self) -> super::BYTE {
        (self._bitfield2) & 3
    }
    pub fn set_AvailableMappingResourcesScope(&mut self, value: super::BYTE) {
        self._bitfield2 = (self._bitfield2 & !3) | (value & 3);
    }
    pub fn UsedMappingResourcesScope(&self) -> super::BYTE {
        (self._bitfield2 >> 2) & 3
    }
    pub fn set_UsedMappingResourcesScope(&mut self, value: super::BYTE) {
        self._bitfield2 = (self._bitfield2 & !(3 << 2)) | ((value & 3) << 2);
    }
    pub fn Reserved2(&self) -> super::BYTE {
        self._bitfield2 >> 4
    }
    pub fn set_Reserved2(&mut self, value: super::BYTE) {
        self._bitfield2 = (self._bitfield2 & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[cfg(feature = "shared")]
impl Default for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    pub Reserved: super::WORD,
    pub SerialNumberLength: super::WORD,
    pub SerialNumber: [super::BYTE; 0],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STORAGE_MEDIA_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub MediumProductType: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_MINIPORT_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Portdriver: STORAGE_PORT_CODE_SET,
    pub LUNResetSupported: super::BOOLEAN,
    pub TargetResetSupported: super::BOOLEAN,
    pub IoTimeoutValue: super::WORD,
    pub ExtraIoInfoSupported: super::BOOLEAN,
    pub Flags: STORAGE_MINIPORT_DESCRIPTOR_0,
    pub Reserved0: [super::BYTE; 2],
    pub Reserved1: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_MINIPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_MINIPORT_DESCRIPTOR_0 {
    pub Anonymous: STORAGE_MINIPORT_DESCRIPTOR_0_0,
    pub AsBYTE: super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_MINIPORT_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    pub fn LogicalPoFxForDisk(&self) -> super::BYTE {
        (self._bitfield) & 1
    }
    pub fn set_LogicalPoFxForDisk(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ForwardIo(&self) -> super::BYTE {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ForwardIo(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::BYTE {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[cfg(target_arch = "x86")]
pub const STORAGE_MINIPORT_DESCRIPTOR_V1_SIZE: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const STORAGE_MINIPORT_DESCRIPTOR_V1_SIZE: u64 = 16;
pub const STORAGE_OFFLOAD_MAX_TOKEN_LENGTH: i32 = 512;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_OFFLOAD_READ_OUTPUT {
    pub OffloadReadFlags: super::DWORD,
    pub Reserved: super::DWORD,
    pub LengthProtected: super::DWORDLONG,
    pub TokenLength: super::DWORD,
    pub Token: STORAGE_OFFLOAD_TOKEN,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_OFFLOAD_READ_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_OFFLOAD_READ_RANGE_TRUNCATED: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_OFFLOAD_TOKEN {
    pub TokenType: [super::BYTE; 4],
    pub Reserved: [super::BYTE; 2],
    pub TokenIdLength: [super::BYTE; 2],
    pub Anonymous: STORAGE_OFFLOAD_TOKEN_0,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_OFFLOAD_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_OFFLOAD_TOKEN_0 {
    pub StorageOffloadZeroDataToken: STORAGE_OFFLOAD_TOKEN_0_0,
    pub Token: [super::BYTE; 504],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_OFFLOAD_TOKEN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_OFFLOAD_TOKEN_0_0 {
    pub Reserved2: [super::BYTE; 504],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_OFFLOAD_TOKEN_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_OFFLOAD_TOKEN_ID_LENGTH: i32 = 504;
pub const STORAGE_OFFLOAD_TOKEN_INVALID: i32 = 2;
pub const STORAGE_OFFLOAD_TOKEN_TYPE_ZERO_DATA: u32 = 4294901761;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_OFFLOAD_WRITE_OUTPUT {
    pub OffloadWriteFlags: super::DWORD,
    pub Reserved: super::DWORD,
    pub LengthCopied: super::DWORDLONG,
}
pub const STORAGE_OFFLOAD_WRITE_RANGE_TRUNCATED: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_OPERATIONAL_REASON {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Reason: STORAGE_OPERATIONAL_STATUS_REASON,
    pub RawBytes: STORAGE_OPERATIONAL_REASON_0,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_OPERATIONAL_REASON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_OPERATIONAL_REASON_0 {
    pub ScsiSenseKey: STORAGE_OPERATIONAL_REASON_0_0,
    pub NVDIMM_N: STORAGE_OPERATIONAL_REASON_0_1,
    pub AsUlong: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_OPERATIONAL_REASON_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_OPERATIONAL_REASON_0_0 {
    pub SenseKey: super::BYTE,
    pub ASC: super::BYTE,
    pub ASCQ: super::BYTE,
    pub Reserved: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_OPERATIONAL_REASON_0_1 {
    pub CriticalHealth: super::BYTE,
    pub ModuleHealth: [super::BYTE; 2],
    pub ErrorThresholdStatus: super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_OPERATIONAL_REASON_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STORAGE_OPERATIONAL_STATUS_REASON = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_PHYSICAL_ADAPTER_DATA {
    pub AdapterId: super::DWORD,
    pub HealthStatus: STORAGE_COMPONENT_HEALTH_STATUS,
    pub CommandProtocol: STORAGE_PROTOCOL_TYPE,
    pub SpecVersion: STORAGE_SPEC_VERSION,
    pub Vendor: [super::BYTE; 8],
    pub Model: [super::BYTE; 40],
    pub FirmwareRevision: [super::BYTE; 16],
    pub PhysicalLocation: [super::BYTE; 32],
    pub ExpanderConnected: super::BOOLEAN,
    pub Reserved0: [super::BYTE; 3],
    pub Reserved1: [super::DWORD; 3],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PHYSICAL_ADAPTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_PHYSICAL_DEVICE_DATA {
    pub DeviceId: super::DWORD,
    pub Role: super::DWORD,
    pub HealthStatus: STORAGE_COMPONENT_HEALTH_STATUS,
    pub CommandProtocol: STORAGE_PROTOCOL_TYPE,
    pub SpecVersion: STORAGE_SPEC_VERSION,
    pub FormFactor: STORAGE_DEVICE_FORM_FACTOR,
    pub Vendor: [super::BYTE; 8],
    pub Model: [super::BYTE; 40],
    pub FirmwareRevision: [super::BYTE; 16],
    pub Capacity: super::DWORDLONG,
    pub PhysicalLocation: [super::BYTE; 32],
    pub Reserved: [super::DWORD; 2],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PHYSICAL_DEVICE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_PHYSICAL_NODE_DATA {
    pub NodeId: super::DWORD,
    pub AdapterCount: super::DWORD,
    pub AdapterDataLength: super::DWORD,
    pub AdapterDataOffset: super::DWORD,
    pub DeviceCount: super::DWORD,
    pub DeviceDataLength: super::DWORD,
    pub DeviceDataOffset: super::DWORD,
    pub Reserved: [super::DWORD; 3],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PHYSICAL_NODE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub NodeCount: super::DWORD,
    pub Reserved: super::DWORD,
    pub Node: [STORAGE_PHYSICAL_NODE_DATA; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STORAGE_PORT_CODE_SET = i32;
pub type STORAGE_POWERUP_REASON_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_PREDICT_FAILURE {
    pub PredictFailure: super::DWORD,
    pub VendorSpecific: [super::BYTE; 512],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PREDICT_FAILURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PRIORITY_HINT_SUPPORT {
    pub SupportFlags: super::DWORD,
}
pub const STORAGE_PRIORITY_HINT_SUPPORTED: i32 = 1;
pub type STORAGE_PROPERTY_ID = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_PROPERTY_QUERY {
    pub PropertyId: STORAGE_PROPERTY_ID,
    pub QueryType: STORAGE_QUERY_TYPE,
    pub AdditionalParameters: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_PROPERTY_SET {
    pub PropertyId: STORAGE_PROPERTY_ID,
    pub SetType: STORAGE_SET_TYPE,
    pub AdditionalParameters: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PROPERTY_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STORAGE_PROTOCOL_ATA_DATA_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_COMMAND {
    pub Version: super::DWORD,
    pub Length: super::DWORD,
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub Flags: super::DWORD,
    pub ReturnStatus: super::DWORD,
    pub ErrorCode: super::DWORD,
    pub CommandLength: super::DWORD,
    pub ErrorInfoLength: super::DWORD,
    pub DataToDeviceTransferLength: super::DWORD,
    pub DataFromDeviceTransferLength: super::DWORD,
    pub TimeOutValue: super::DWORD,
    pub ErrorInfoOffset: super::DWORD,
    pub DataToDeviceBufferOffset: super::DWORD,
    pub DataFromDeviceBufferOffset: super::DWORD,
    pub CommandSpecific: super::DWORD,
    pub Reserved0: super::DWORD,
    pub FixedProtocolReturnData: super::DWORD,
    pub FixedProtocolReturnData2: super::DWORD,
    pub Reserved1: [super::DWORD; 2],
    pub Command: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PROTOCOL_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST: u32 = 2147483648;
pub const STORAGE_PROTOCOL_COMMAND_LENGTH_NVME: i32 = 64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub ProtocolSpecificData: STORAGE_PROTOCOL_SPECIFIC_DATA,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub ProtocolSpecificData: STORAGE_PROTOCOL_SPECIFIC_DATA_EXT,
}
pub const STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    pub Anonymous: STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0,
    pub AsUlong: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    pub fn RetainAsynEvent(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_RetainAsynEvent(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn LogSpecificField(&self) -> super::DWORD {
        (self._bitfield >> 1) & 15
    }
    pub fn set_LogSpecificField(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(15 << 1)) | ((value & 15) << 1);
    }
    pub fn Reserved0(&self) -> super::DWORD {
        (self._bitfield >> 5) & 7
    }
    pub fn set_Reserved0(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn UUIDIndex(&self) -> super::DWORD {
        (self._bitfield >> 8) & 127
    }
    pub fn set_UUIDIndex(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(127 << 8)) | ((value & 127) << 8);
    }
    pub fn Reserved(&self) -> super::DWORD {
        self._bitfield >> 15
    }
    pub fn set_Reserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(131071 << 15)) | ((value & 131071) << 15);
    }
}
pub type STORAGE_PROTOCOL_NVME_DATA_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_SPECIFIC_DATA {
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub DataType: super::DWORD,
    pub ProtocolDataRequestValue: super::DWORD,
    pub ProtocolDataRequestSubValue: super::DWORD,
    pub ProtocolDataOffset: super::DWORD,
    pub ProtocolDataLength: super::DWORD,
    pub FixedProtocolReturnData: super::DWORD,
    pub ProtocolDataRequestSubValue2: super::DWORD,
    pub ProtocolDataRequestSubValue3: super::DWORD,
    pub ProtocolDataRequestSubValue4: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub DataType: super::DWORD,
    pub ProtocolDataValue: super::DWORD,
    pub ProtocolDataSubValue: super::DWORD,
    pub ProtocolDataOffset: super::DWORD,
    pub ProtocolDataLength: super::DWORD,
    pub FixedProtocolReturnData: super::DWORD,
    pub ProtocolDataSubValue2: super::DWORD,
    pub ProtocolDataSubValue3: super::DWORD,
    pub ProtocolDataSubValue4: super::DWORD,
    pub ProtocolDataSubValue5: super::DWORD,
    pub ProtocolDataSubValue6: super::DWORD,
    pub Reserved: [super::DWORD; 4],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND: i32 = 1;
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_NVM_COMMAND: i32 = 2;
pub const STORAGE_PROTOCOL_STATUS_BUSY: i32 = 5;
pub const STORAGE_PROTOCOL_STATUS_DATA_OVERRUN: i32 = 6;
pub const STORAGE_PROTOCOL_STATUS_ERROR: i32 = 2;
pub const STORAGE_PROTOCOL_STATUS_INSUFFICIENT_RESOURCES: i32 = 7;
pub const STORAGE_PROTOCOL_STATUS_INVALID_REQUEST: i32 = 3;
pub const STORAGE_PROTOCOL_STATUS_NOT_SUPPORTED: i32 = 255;
pub const STORAGE_PROTOCOL_STATUS_NO_DEVICE: i32 = 4;
pub const STORAGE_PROTOCOL_STATUS_PENDING: i32 = 0;
pub const STORAGE_PROTOCOL_STATUS_SUCCESS: i32 = 1;
pub const STORAGE_PROTOCOL_STATUS_THROTTLED_REQUEST: i32 = 8;
pub const STORAGE_PROTOCOL_STRUCTURE_VERSION: i32 = 1;
pub type STORAGE_PROTOCOL_TYPE = i32;
pub type STORAGE_PROTOCOL_UFS_DATA_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    pub EntryLength: super::DWORD,
    pub DependencyTypeFlags: super::DWORD,
    pub ProviderSpecificFlags: super::DWORD,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    pub EntryLength: super::DWORD,
    pub DependencyTypeFlags: super::DWORD,
    pub ProviderSpecificFlags: super::DWORD,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub AncestorLevel: super::DWORD,
    pub HostVolumeNameOffset: super::DWORD,
    pub HostVolumeNameSize: super::DWORD,
    pub DependentVolumeNameOffset: super::DWORD,
    pub DependentVolumeNameSize: super::DWORD,
    pub RelativePathOffset: super::DWORD,
    pub RelativePathSize: super::DWORD,
    pub DependentDeviceNameOffset: super::DWORD,
    pub DependentDeviceNameSize: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    pub RequestLevel: super::DWORD,
    pub RequestFlags: super::DWORD,
}
pub type STORAGE_QUERY_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STORAGE_READ_CAPACITY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub BlockLength: super::DWORD,
    pub NumberOfBlocks: super::LARGE_INTEGER,
    pub DiskLength: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STORAGE_READ_CAPACITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_REINITIALIZE_MEDIA {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub TimeoutInSeconds: super::DWORD,
    pub SanitizeOption: STORAGE_REINITIALIZE_MEDIA_0,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_REINITIALIZE_MEDIA_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl STORAGE_REINITIALIZE_MEDIA_0 {
    pub fn SanitizeMethod(&self) -> super::DWORD {
        (self._bitfield) & 15
    }
    pub fn set_SanitizeMethod(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn DisallowUnrestrictedSanitizeExit(&self) -> super::DWORD {
        (self._bitfield >> 4) & 1
    }
    pub fn set_DisallowUnrestrictedSanitizeExit(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::DWORD {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(134217727 << 5)) | ((value & 134217727) << 5);
    }
}
pub type STORAGE_RESERVE_ID = i32;
pub type STORAGE_RPMB_COMMAND_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_RPMB_DATA_FRAME {
    pub Stuff: [super::BYTE; 196],
    pub KeyOrMAC: [super::BYTE; 32],
    pub Data: [super::BYTE; 256],
    pub Nonce: [super::BYTE; 16],
    pub WriteCounter: [super::BYTE; 4],
    pub Address: [super::BYTE; 2],
    pub BlockCount: [super::BYTE; 2],
    pub OperationResult: [super::BYTE; 2],
    pub RequestOrResponseType: [super::BYTE; 2],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_RPMB_DATA_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_RPMB_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub SizeInBytes: super::DWORD,
    pub MaxReliableWriteSizeInBytes: super::DWORD,
    pub FrameFormat: STORAGE_RPMB_FRAME_TYPE,
}
pub const STORAGE_RPMB_DESCRIPTOR_VERSION_1: i32 = 1;
pub type STORAGE_RPMB_FRAME_TYPE = i32;
pub const STORAGE_RPMB_MINIMUM_RELIABLE_WRITE_SIZE: i32 = 512;
pub type STORAGE_SANITIZE_METHOD = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_SECURITY_COMPLIANCE_BITMASK {
    pub Anonymous: STORAGE_SECURITY_COMPLIANCE_BITMASK_0,
    pub AsUchar: super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_SECURITY_COMPLIANCE_BITMASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_SECURITY_COMPLIANCE_BITMASK_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl STORAGE_SECURITY_COMPLIANCE_BITMASK_0 {
    pub fn FIPS(&self) -> super::BYTE {
        (self._bitfield) & 1
    }
    pub fn set_FIPS(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::BYTE {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::BYTE) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
pub type STORAGE_SET_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_SPEC_VERSION {
    pub Anonymous: STORAGE_SPEC_VERSION_0,
    pub AsUlong: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_SPEC_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_SPEC_VERSION_0 {
    pub MinorVersion: STORAGE_SPEC_VERSION_0_0,
    pub MajorVersion: super::WORD,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_SPEC_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_SPEC_VERSION_0_0 {
    pub Anonymous: STORAGE_SPEC_VERSION_0_0_0,
    pub AsUshort: super::WORD,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_SPEC_VERSION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_SPEC_VERSION_0_0_0 {
    pub SubMinor: super::BYTE,
    pub Minor: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_STACK_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub StorageStackType: STORAGE_STACK_TYPE,
}
pub type STORAGE_STACK_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub CriticalTemperature: super::SHORT,
    pub WarningTemperature: super::SHORT,
    pub InfoCount: super::WORD,
    pub Reserved0: [super::BYTE; 2],
    pub Reserved1: [super::DWORD; 2],
    pub TemperatureInfo: [STORAGE_TEMPERATURE_INFO; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_TEMPERATURE_INFO {
    pub Index: super::WORD,
    pub Temperature: super::SHORT,
    pub OverThreshold: super::SHORT,
    pub UnderThreshold: super::SHORT,
    pub OverThresholdChangable: super::BOOLEAN,
    pub UnderThresholdChangable: super::BOOLEAN,
    pub EventGenerated: super::BOOLEAN,
    pub Reserved0: super::BYTE,
    pub Reserved1: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_TEMPERATURE_THRESHOLD {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::WORD,
    pub Index: super::WORD,
    pub Threshold: super::SHORT,
    pub OverThreshold: super::BOOLEAN,
    pub Reserved: super::BYTE,
}
pub const STORAGE_TEMPERATURE_THRESHOLD_FLAG_ADAPTER_REQUEST: i32 = 1;
pub const STORAGE_TEMPERATURE_VALUE_NOT_REPORTED: i32 = 32768;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_TIER {
    pub Id: windows_core::GUID,
    pub Name: [super::WCHAR; 256],
    pub Description: [super::WCHAR; 256],
    pub Flags: super::DWORDLONG,
    pub ProvisionedCapacity: super::DWORDLONG,
    pub MediaType: STORAGE_TIER_MEDIA_TYPE,
    pub Class: STORAGE_TIER_CLASS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STORAGE_TIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STORAGE_TIER_CLASS = i32;
pub const STORAGE_TIER_DESCRIPTION_LENGTH: i32 = 512;
pub const STORAGE_TIER_FLAG_NO_SEEK_PENALTY: i32 = 131072;
pub const STORAGE_TIER_FLAG_PARITY: i32 = 8388608;
pub const STORAGE_TIER_FLAG_READ_CACHE: i32 = 4194304;
pub const STORAGE_TIER_FLAG_SMR: i32 = 16777216;
pub const STORAGE_TIER_FLAG_WRITE_BACK_CACHE: i32 = 2097152;
pub type STORAGE_TIER_MEDIA_TYPE = i32;
pub const STORAGE_TIER_NAME_LENGTH: i32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_TIER_REGION {
    pub TierId: windows_core::GUID,
    pub Offset: super::DWORDLONG,
    pub Length: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_WRITE_CACHE_PROPERTY {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub WriteCacheType: WRITE_CACHE_TYPE,
    pub WriteCacheEnabled: WRITE_CACHE_ENABLE,
    pub WriteCacheChangeable: WRITE_CACHE_CHANGE,
    pub WriteThroughSupported: WRITE_THROUGH,
    pub FlushCacheSupported: super::BOOLEAN,
    pub UserDefinedPowerProtection: super::BOOLEAN,
    pub NVCacheEnabled: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub DeviceType: STORAGE_ZONED_DEVICE_TYPES,
    pub ZoneCount: super::DWORD,
    pub ZoneAttributes: STORAGE_ZONED_DEVICE_DESCRIPTOR_0,
    pub ZoneGroupCount: super::DWORD,
    pub ZoneGroup: [STORAGE_ZONE_GROUP; 1],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_ZONED_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    pub SequentialRequiredZone: STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0,
    pub SequentialPreferredZone: STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    pub MaxOpenZoneCount: super::DWORD,
    pub UnrestrictedRead: super::BOOLEAN,
    pub Reserved: [super::BYTE; 3],
}
#[cfg(feature = "shared")]
impl Default for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {
    pub OptimalOpenZoneCount: super::DWORD,
    pub Reserved: super::DWORD,
}
pub type STORAGE_ZONED_DEVICE_TYPES = i32;
pub type STORAGE_ZONES_ATTRIBUTES = i32;
pub type STORAGE_ZONE_CONDITION = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_ZONE_DESCRIPTOR {
    pub Size: super::DWORD,
    pub ZoneType: STORAGE_ZONE_TYPES,
    pub ZoneCondition: STORAGE_ZONE_CONDITION,
    pub ResetWritePointerRecommend: super::BOOLEAN,
    pub Reserved0: [super::BYTE; 3],
    pub ZoneSize: super::DWORDLONG,
    pub WritePointerOffset: super::DWORDLONG,
}
#[cfg(feature = "shared")]
impl Default for STORAGE_ZONE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_ZONE_GROUP {
    pub ZoneCount: super::DWORD,
    pub ZoneType: STORAGE_ZONE_TYPES,
    pub ZoneSize: super::DWORDLONG,
}
pub type STORAGE_ZONE_TYPES = i32;
pub const STORATTRIBUTE_MANAGEMENT_STATE: i32 = 1;
pub const STORATTRIBUTE_NONE: i32 = 0;
pub const STREAMS_ASSOCIATE_ID_CLEAR: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    pub Flags: super::DWORD,
    pub StreamId: super::DWORD,
}
pub const STREAMS_ASSOCIATE_ID_SET: i32 = 2;
pub const STREAMS_INVALID_ID: i32 = 0;
pub const STREAMS_MAX_ID: i32 = 65535;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAMS_QUERY_ID_OUTPUT_BUFFER {
    pub StreamId: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    pub OptimalWriteSize: super::DWORD,
    pub StreamGranularitySize: super::DWORD,
    pub StreamIdMin: super::DWORD,
    pub StreamIdMax: super::DWORD,
}
pub const STREAM_CLEAR_ENCRYPTION: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STREAM_EXTENT_ENTRY {
    pub Flags: super::DWORD,
    pub ExtentInformation: STREAM_EXTENT_ENTRY_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STREAM_EXTENT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union STREAM_EXTENT_ENTRY_0 {
    pub RetrievalPointers: RETRIEVAL_POINTERS_BUFFER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STREAM_EXTENT_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STREAM_EXTENT_ENTRY_ALL_EXTENTS: i32 = 2;
pub const STREAM_EXTENT_ENTRY_AS_RETRIEVAL_POINTERS: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STREAM_INFORMATION_ENTRY {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub StreamInformation: STREAM_INFORMATION_ENTRY_0,
}
#[cfg(feature = "shared")]
impl Default for STREAM_INFORMATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union STREAM_INFORMATION_ENTRY_0 {
    pub DesiredStorageClass: STREAM_INFORMATION_ENTRY_0_1,
    pub DataStream: STREAM_INFORMATION_ENTRY_0_0,
    pub Reparse: STREAM_INFORMATION_ENTRY_0_3,
    pub Ea: STREAM_INFORMATION_ENTRY_0_2,
}
#[cfg(feature = "shared")]
impl Default for STREAM_INFORMATION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAM_INFORMATION_ENTRY_0_0 {
    pub Length: super::WORD,
    pub Flags: super::WORD,
    pub Reserved: super::DWORD,
    pub Vdl: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAM_INFORMATION_ENTRY_0_1 {
    pub Class: FILE_STORAGE_TIER_CLASS,
    pub Flags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAM_INFORMATION_ENTRY_0_2 {
    pub Length: super::WORD,
    pub Flags: super::WORD,
    pub EaSize: super::DWORD,
    pub EaInformationOffset: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAM_INFORMATION_ENTRY_0_3 {
    pub Length: super::WORD,
    pub Flags: super::WORD,
    pub ReparseDataSize: super::DWORD,
    pub ReparseDataOffset: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STREAM_LAYOUT_ENTRY {
    pub Version: super::DWORD,
    pub NextStreamOffset: super::DWORD,
    pub Flags: super::DWORD,
    pub ExtentInformationOffset: super::DWORD,
    pub AllocationSize: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub StreamInformationOffset: super::DWORD,
    pub AttributeTypeCode: super::DWORD,
    pub AttributeFlags: super::DWORD,
    pub StreamIdentifierLength: super::DWORD,
    pub StreamIdentifier: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STREAM_LAYOUT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STREAM_LAYOUT_ENTRY_HAS_INFORMATION: i32 = 16;
pub const STREAM_LAYOUT_ENTRY_IMMOVABLE: i32 = 1;
pub const STREAM_LAYOUT_ENTRY_NO_CLUSTERS_ALLOCATED: i32 = 8;
pub const STREAM_LAYOUT_ENTRY_PINNED: i32 = 2;
pub const STREAM_LAYOUT_ENTRY_RESIDENT: i32 = 4;
pub const STREAM_SET_ENCRYPTION: i32 = 3;
pub const SYQUEST_EZ135: STORAGE_MEDIA_TYPE = 75;
pub const SYQUEST_EZFLYER: STORAGE_MEDIA_TYPE = 76;
pub const SYQUEST_SYJET: STORAGE_MEDIA_TYPE = 77;
pub const ScmBusFirmwareActivationState_Armed: SCM_BUS_FIRMWARE_ACTIVATION_STATE = 1;
pub const ScmBusFirmwareActivationState_Busy: SCM_BUS_FIRMWARE_ACTIVATION_STATE = 2;
pub const ScmBusFirmwareActivationState_Idle: SCM_BUS_FIRMWARE_ACTIVATION_STATE = 0;
pub const ScmBusProperty_DedicatedMemoryInfo: SCM_BUS_PROPERTY_ID = 1;
pub const ScmBusProperty_DedicatedMemoryState: SCM_BUS_PROPERTY_ID = 2;
pub const ScmBusProperty_Max: SCM_BUS_PROPERTY_ID = 3;
pub const ScmBusProperty_RuntimeFwActivationInfo: SCM_BUS_PROPERTY_ID = 0;
pub const ScmBusQuery_Descriptor: SCM_BUS_QUERY_TYPE = 0;
pub const ScmBusQuery_IsSupported: SCM_BUS_QUERY_TYPE = 1;
pub const ScmBusQuery_Max: SCM_BUS_QUERY_TYPE = 2;
pub const ScmBusSet_Descriptor: SCM_BUS_SET_TYPE = 0;
pub const ScmBusSet_IsSupported: SCM_BUS_SET_TYPE = 1;
pub const ScmBusSet_Max: SCM_BUS_SET_TYPE = 2;
pub const ScmPdFirmwareActivationState_Armed: SCM_PD_FIRMWARE_ACTIVATION_STATE = 1;
pub const ScmPdFirmwareActivationState_Busy: SCM_PD_FIRMWARE_ACTIVATION_STATE = 2;
pub const ScmPdFirmwareActivationState_Idle: SCM_PD_FIRMWARE_ACTIVATION_STATE = 0;
pub const ScmPdLastFwActivaitonStatus_ActivationInProgress: SCM_PD_LAST_FW_ACTIVATION_STATUS = 4;
pub const ScmPdLastFwActivaitonStatus_FwUnsupported: SCM_PD_LAST_FW_ACTIVATION_STATUS = 6;
pub const ScmPdLastFwActivaitonStatus_Retry: SCM_PD_LAST_FW_ACTIVATION_STATUS = 5;
pub const ScmPdLastFwActivaitonStatus_UnknownError: SCM_PD_LAST_FW_ACTIVATION_STATUS = 7;
pub const ScmPdLastFwActivationStatus_ColdRebootRequired: SCM_PD_LAST_FW_ACTIVATION_STATUS = 3;
pub const ScmPdLastFwActivationStatus_FwNotFound: SCM_PD_LAST_FW_ACTIVATION_STATUS = 2;
pub const ScmPdLastFwActivationStatus_None: SCM_PD_LAST_FW_ACTIVATION_STATUS = 0;
pub const ScmPdLastFwActivationStatus_Success: SCM_PD_LAST_FW_ACTIVATION_STATUS = 1;
pub const ScmPhysicalDeviceHealth_Healthy: SCM_PD_HEALTH_STATUS = 3;
pub const ScmPhysicalDeviceHealth_Max: SCM_PD_HEALTH_STATUS = 4;
pub const ScmPhysicalDeviceHealth_Unhealthy: SCM_PD_HEALTH_STATUS = 1;
pub const ScmPhysicalDeviceHealth_Unknown: SCM_PD_HEALTH_STATUS = 0;
pub const ScmPhysicalDeviceHealth_Warning: SCM_PD_HEALTH_STATUS = 2;
pub const ScmPhysicalDeviceOpReason_BackgroundOperation: SCM_PD_OPERATIONAL_STATUS_REASON = 9;
pub const ScmPhysicalDeviceOpReason_Component: SCM_PD_OPERATIONAL_STATUS_REASON = 8;
pub const ScmPhysicalDeviceOpReason_Configuration: SCM_PD_OPERATIONAL_STATUS_REASON = 5;
pub const ScmPhysicalDeviceOpReason_DataPersistenceLossImminent: SCM_PD_OPERATIONAL_STATUS_REASON = 17;
pub const ScmPhysicalDeviceOpReason_DeviceController: SCM_PD_OPERATIONAL_STATUS_REASON = 6;
pub const ScmPhysicalDeviceOpReason_DisabledByPlatform: SCM_PD_OPERATIONAL_STATUS_REASON = 13;
pub const ScmPhysicalDeviceOpReason_EnergySource: SCM_PD_OPERATIONAL_STATUS_REASON = 4;
pub const ScmPhysicalDeviceOpReason_ExcessiveTemperature: SCM_PD_OPERATIONAL_STATUS_REASON = 21;
pub const ScmPhysicalDeviceOpReason_FatalError: SCM_PD_OPERATIONAL_STATUS_REASON = 16;
pub const ScmPhysicalDeviceOpReason_HealthCheck: SCM_PD_OPERATIONAL_STATUS_REASON = 11;
pub const ScmPhysicalDeviceOpReason_InternalFailure: SCM_PD_OPERATIONAL_STATUS_REASON = 22;
pub const ScmPhysicalDeviceOpReason_InvalidFirmware: SCM_PD_OPERATIONAL_STATUS_REASON = 10;
pub const ScmPhysicalDeviceOpReason_LostData: SCM_PD_OPERATIONAL_STATUS_REASON = 3;
pub const ScmPhysicalDeviceOpReason_LostDataPersistence: SCM_PD_OPERATIONAL_STATUS_REASON = 12;
pub const ScmPhysicalDeviceOpReason_LostWritePersistence: SCM_PD_OPERATIONAL_STATUS_REASON = 15;
pub const ScmPhysicalDeviceOpReason_Max: SCM_PD_OPERATIONAL_STATUS_REASON = 23;
pub const ScmPhysicalDeviceOpReason_Media: SCM_PD_OPERATIONAL_STATUS_REASON = 1;
pub const ScmPhysicalDeviceOpReason_MediaController: SCM_PD_OPERATIONAL_STATUS_REASON = 7;
pub const ScmPhysicalDeviceOpReason_MediaRemainingSpareBlock: SCM_PD_OPERATIONAL_STATUS_REASON = 19;
pub const ScmPhysicalDeviceOpReason_PerformanceDegradation: SCM_PD_OPERATIONAL_STATUS_REASON = 20;
pub const ScmPhysicalDeviceOpReason_PermanentError: SCM_PD_OPERATIONAL_STATUS_REASON = 14;
pub const ScmPhysicalDeviceOpReason_ThresholdExceeded: SCM_PD_OPERATIONAL_STATUS_REASON = 2;
pub const ScmPhysicalDeviceOpReason_Unknown: SCM_PD_OPERATIONAL_STATUS_REASON = 0;
pub const ScmPhysicalDeviceOpReason_WritePersistenceLossImminent: SCM_PD_OPERATIONAL_STATUS_REASON = 18;
pub const ScmPhysicalDeviceOpStatus_HardwareError: SCM_PD_OPERATIONAL_STATUS = 4;
pub const ScmPhysicalDeviceOpStatus_InService: SCM_PD_OPERATIONAL_STATUS = 3;
pub const ScmPhysicalDeviceOpStatus_Max: SCM_PD_OPERATIONAL_STATUS = 8;
pub const ScmPhysicalDeviceOpStatus_Missing: SCM_PD_OPERATIONAL_STATUS = 7;
pub const ScmPhysicalDeviceOpStatus_NotUsable: SCM_PD_OPERATIONAL_STATUS = 5;
pub const ScmPhysicalDeviceOpStatus_Ok: SCM_PD_OPERATIONAL_STATUS = 1;
pub const ScmPhysicalDeviceOpStatus_PredictingFailure: SCM_PD_OPERATIONAL_STATUS = 2;
pub const ScmPhysicalDeviceOpStatus_TransientError: SCM_PD_OPERATIONAL_STATUS = 6;
pub const ScmPhysicalDeviceOpStatus_Unknown: SCM_PD_OPERATIONAL_STATUS = 0;
pub const ScmPhysicalDeviceProperty_DeviceHandle: SCM_PD_PROPERTY_ID = 5;
pub const ScmPhysicalDeviceProperty_DeviceInfo: SCM_PD_PROPERTY_ID = 0;
pub const ScmPhysicalDeviceProperty_DeviceSpecificInfo: SCM_PD_PROPERTY_ID = 4;
pub const ScmPhysicalDeviceProperty_FirmwareInfo: SCM_PD_PROPERTY_ID = 2;
pub const ScmPhysicalDeviceProperty_FruIdString: SCM_PD_PROPERTY_ID = 6;
pub const ScmPhysicalDeviceProperty_LocationString: SCM_PD_PROPERTY_ID = 3;
pub const ScmPhysicalDeviceProperty_ManagementStatus: SCM_PD_PROPERTY_ID = 1;
pub const ScmPhysicalDeviceProperty_Max: SCM_PD_PROPERTY_ID = 9;
pub const ScmPhysicalDeviceProperty_RuntimeFwActivationArmState: SCM_PD_PROPERTY_ID = 8;
pub const ScmPhysicalDeviceProperty_RuntimeFwActivationInfo: SCM_PD_PROPERTY_ID = 7;
pub const ScmPhysicalDeviceQuery_Descriptor: SCM_PD_QUERY_TYPE = 0;
pub const ScmPhysicalDeviceQuery_IsSupported: SCM_PD_QUERY_TYPE = 1;
pub const ScmPhysicalDeviceQuery_Max: SCM_PD_QUERY_TYPE = 2;
pub const ScmPhysicalDeviceReinit_ColdBootNeeded: SCM_PD_MEDIA_REINITIALIZATION_STATUS = 2;
pub const ScmPhysicalDeviceReinit_Max: SCM_PD_MEDIA_REINITIALIZATION_STATUS = 3;
pub const ScmPhysicalDeviceReinit_RebootNeeded: SCM_PD_MEDIA_REINITIALIZATION_STATUS = 1;
pub const ScmPhysicalDeviceReinit_Success: SCM_PD_MEDIA_REINITIALIZATION_STATUS = 0;
pub const ScmPhysicalDeviceSet_Descriptor: SCM_PD_SET_TYPE = 0;
pub const ScmPhysicalDeviceSet_IsSupported: SCM_PD_SET_TYPE = 1;
pub const ScmPhysicalDeviceSet_Max: SCM_PD_SET_TYPE = 2;
pub const ScmRegionFlagLabel: SCM_REGION_FLAG = 1;
pub const ScmRegionFlagNone: SCM_REGION_FLAG = 0;
pub const ShrinkAbort: SHRINK_VOLUME_REQUEST_TYPES = 3;
pub const ShrinkCommit: SHRINK_VOLUME_REQUEST_TYPES = 2;
pub const ShrinkPrepare: SHRINK_VOLUME_REQUEST_TYPES = 1;
pub const SmrGcActionPause: REFS_SMR_VOLUME_GC_ACTION = 3;
pub const SmrGcActionStart: REFS_SMR_VOLUME_GC_ACTION = 1;
pub const SmrGcActionStartFullSpeed: REFS_SMR_VOLUME_GC_ACTION = 2;
pub const SmrGcActionStop: REFS_SMR_VOLUME_GC_ACTION = 4;
pub const SmrGcMethodCompaction: REFS_SMR_VOLUME_GC_METHOD = 1;
pub const SmrGcMethodCompression: REFS_SMR_VOLUME_GC_METHOD = 2;
pub const SmrGcMethodRotation: REFS_SMR_VOLUME_GC_METHOD = 3;
pub const SmrGcStateActive: REFS_SMR_VOLUME_GC_STATE = 2;
pub const SmrGcStateActiveFullSpeed: REFS_SMR_VOLUME_GC_STATE = 3;
pub const SmrGcStateInactive: REFS_SMR_VOLUME_GC_STATE = 0;
pub const SmrGcStatePaused: REFS_SMR_VOLUME_GC_STATE = 1;
pub const StorAttributeMgmt_ClearAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = 0;
pub const StorAttributeMgmt_ResetAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = 2;
pub const StorAttributeMgmt_SetAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = 1;
pub const StorCryptoAlgorithmAESECB: STORAGE_CRYPTO_ALGORITHM_ID = 3;
pub const StorCryptoAlgorithmBitlockerAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = 2;
pub const StorCryptoAlgorithmESSIVAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = 4;
pub const StorCryptoAlgorithmUnknown: STORAGE_CRYPTO_ALGORITHM_ID = 0;
pub const StorCryptoAlgorithmXTSAES: STORAGE_CRYPTO_ALGORITHM_ID = 1;
pub const StorCryptoKeySize128Bits: STORAGE_CRYPTO_KEY_SIZE = 1;
pub const StorCryptoKeySize192Bits: STORAGE_CRYPTO_KEY_SIZE = 2;
pub const StorCryptoKeySize256Bits: STORAGE_CRYPTO_KEY_SIZE = 3;
pub const StorCryptoKeySize512Bits: STORAGE_CRYPTO_KEY_SIZE = 4;
pub const StorCryptoKeySizeUnknown: STORAGE_CRYPTO_KEY_SIZE = 0;
pub const StorRpmbAuthenticatedDeviceConfigRead: STORAGE_RPMB_COMMAND_TYPE = 7;
pub const StorRpmbAuthenticatedDeviceConfigWrite: STORAGE_RPMB_COMMAND_TYPE = 6;
pub const StorRpmbAuthenticatedRead: STORAGE_RPMB_COMMAND_TYPE = 4;
pub const StorRpmbAuthenticatedWrite: STORAGE_RPMB_COMMAND_TYPE = 3;
pub const StorRpmbProgramAuthKey: STORAGE_RPMB_COMMAND_TYPE = 1;
pub const StorRpmbQueryWriteCounter: STORAGE_RPMB_COMMAND_TYPE = 2;
pub const StorRpmbReadResultRequest: STORAGE_RPMB_COMMAND_TYPE = 5;
pub const StorageAccessAlignmentProperty: STORAGE_PROPERTY_ID = 6;
pub const StorageAdapterCryptoProperty: STORAGE_PROPERTY_ID = 17;
pub const StorageAdapterPhysicalTopologyProperty: STORAGE_PROPERTY_ID = 53;
pub const StorageAdapterProperty: STORAGE_PROPERTY_ID = 1;
pub const StorageAdapterProtocolSpecificProperty: STORAGE_PROPERTY_ID = 49;
pub const StorageAdapterProtocolSpecificPropertyEx: STORAGE_PROPERTY_ID = 67;
pub const StorageAdapterRpmbProperty: STORAGE_PROPERTY_ID = 16;
pub const StorageAdapterSerialNumberProperty: STORAGE_PROPERTY_ID = 57;
pub const StorageAdapterTemperatureProperty: STORAGE_PROPERTY_ID = 51;
pub const StorageCounterTypeFlushLatency100NSMax: STORAGE_COUNTER_TYPE = 20;
pub const StorageCounterTypeLoadUnloadCycleCount: STORAGE_COUNTER_TYPE = 12;
pub const StorageCounterTypeLoadUnloadCycleCountMax: STORAGE_COUNTER_TYPE = 13;
pub const StorageCounterTypeManufactureDate: STORAGE_COUNTER_TYPE = 9;
pub const StorageCounterTypeMax: STORAGE_COUNTER_TYPE = 21;
pub const StorageCounterTypePowerOnHours: STORAGE_COUNTER_TYPE = 17;
pub const StorageCounterTypeReadErrorsCorrected: STORAGE_COUNTER_TYPE = 4;
pub const StorageCounterTypeReadErrorsTotal: STORAGE_COUNTER_TYPE = 3;
pub const StorageCounterTypeReadErrorsUncorrected: STORAGE_COUNTER_TYPE = 5;
pub const StorageCounterTypeReadLatency100NSMax: STORAGE_COUNTER_TYPE = 18;
pub const StorageCounterTypeStartStopCycleCount: STORAGE_COUNTER_TYPE = 10;
pub const StorageCounterTypeStartStopCycleCountMax: STORAGE_COUNTER_TYPE = 11;
pub const StorageCounterTypeTemperatureCelsius: STORAGE_COUNTER_TYPE = 1;
pub const StorageCounterTypeTemperatureCelsiusMax: STORAGE_COUNTER_TYPE = 2;
pub const StorageCounterTypeUnknown: STORAGE_COUNTER_TYPE = 0;
pub const StorageCounterTypeWearPercentage: STORAGE_COUNTER_TYPE = 14;
pub const StorageCounterTypeWearPercentageMax: STORAGE_COUNTER_TYPE = 16;
pub const StorageCounterTypeWearPercentageWarning: STORAGE_COUNTER_TYPE = 15;
pub const StorageCounterTypeWriteErrorsCorrected: STORAGE_COUNTER_TYPE = 7;
pub const StorageCounterTypeWriteErrorsTotal: STORAGE_COUNTER_TYPE = 6;
pub const StorageCounterTypeWriteErrorsUncorrected: STORAGE_COUNTER_TYPE = 8;
pub const StorageCounterTypeWriteLatency100NSMax: STORAGE_COUNTER_TYPE = 19;
pub const StorageCryptoAlgorithmAESECB: STORAGE_CRYPTO_ALGORITHM_ID = 3;
pub const StorageCryptoAlgorithmBitlockerAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = 2;
pub const StorageCryptoAlgorithmESSIVAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = 4;
pub const StorageCryptoAlgorithmMax: STORAGE_CRYPTO_ALGORITHM_ID = 5;
pub const StorageCryptoAlgorithmUnknown: STORAGE_CRYPTO_ALGORITHM_ID = 0;
pub const StorageCryptoAlgorithmXTSAES: STORAGE_CRYPTO_ALGORITHM_ID = 1;
pub const StorageCryptoKeySize128Bits: STORAGE_CRYPTO_KEY_SIZE = 1;
pub const StorageCryptoKeySize192Bits: STORAGE_CRYPTO_KEY_SIZE = 2;
pub const StorageCryptoKeySize256Bits: STORAGE_CRYPTO_KEY_SIZE = 3;
pub const StorageCryptoKeySize512Bits: STORAGE_CRYPTO_KEY_SIZE = 4;
pub const StorageCryptoKeySizeMax: STORAGE_CRYPTO_KEY_SIZE = 5;
pub const StorageCryptoKeySizeUnknown: STORAGE_CRYPTO_KEY_SIZE = 0;
pub const StorageDeviceAttributesProperty: STORAGE_PROPERTY_ID = 55;
pub const StorageDeviceCopyOffloadProperty: STORAGE_PROPERTY_ID = 13;
pub const StorageDeviceDeviceTelemetryProperty: STORAGE_PROPERTY_ID = 10;
pub const StorageDeviceEnduranceProperty: STORAGE_PROPERTY_ID = 62;
pub const StorageDeviceIdProperty: STORAGE_PROPERTY_ID = 2;
pub const StorageDeviceIoCapabilityProperty: STORAGE_PROPERTY_ID = 48;
pub const StorageDeviceLBProvisioningProperty: STORAGE_PROPERTY_ID = 11;
pub const StorageDeviceLedStateProperty: STORAGE_PROPERTY_ID = 63;
pub const StorageDeviceLocationProperty: STORAGE_PROPERTY_ID = 58;
pub const StorageDeviceManagementStatus: STORAGE_PROPERTY_ID = 56;
pub const StorageDeviceMediumProductType: STORAGE_PROPERTY_ID = 15;
pub const StorageDeviceNumaProperty: STORAGE_PROPERTY_ID = 59;
pub const StorageDevicePhysicalTopologyProperty: STORAGE_PROPERTY_ID = 54;
pub const StorageDevicePowerCapUnitsMilliwatts: STORAGE_DEVICE_POWER_CAP_UNITS = 1;
pub const StorageDevicePowerCapUnitsPercent: STORAGE_DEVICE_POWER_CAP_UNITS = 0;
pub const StorageDevicePowerProperty: STORAGE_PROPERTY_ID = 12;
pub const StorageDeviceProperty: STORAGE_PROPERTY_ID = 0;
pub const StorageDeviceProtocolSpecificProperty: STORAGE_PROPERTY_ID = 50;
pub const StorageDeviceProtocolSpecificPropertyEx: STORAGE_PROPERTY_ID = 68;
pub const StorageDeviceResiliencyProperty: STORAGE_PROPERTY_ID = 14;
pub const StorageDeviceSeekPenaltyProperty: STORAGE_PROPERTY_ID = 7;
pub const StorageDeviceSelfEncryptionProperty: STORAGE_PROPERTY_ID = 64;
pub const StorageDeviceTemperatureProperty: STORAGE_PROPERTY_ID = 52;
pub const StorageDeviceTrimProperty: STORAGE_PROPERTY_ID = 8;
pub const StorageDeviceUniqueIdProperty: STORAGE_PROPERTY_ID = 3;
pub const StorageDeviceUnsafeShutdownCount: STORAGE_PROPERTY_ID = 61;
pub const StorageDeviceWriteAggregationProperty: STORAGE_PROPERTY_ID = 9;
pub const StorageDeviceWriteCacheProperty: STORAGE_PROPERTY_ID = 4;
pub const StorageDeviceZonedDeviceProperty: STORAGE_PROPERTY_ID = 60;
pub const StorageDiagnosticLevelDefault: STORAGE_DIAGNOSTIC_LEVEL = 0;
pub const StorageDiagnosticLevelMax: STORAGE_DIAGNOSTIC_LEVEL = 1;
pub const StorageDiagnosticTargetTypeHbaFirmware: STORAGE_DIAGNOSTIC_TARGET_TYPE = 3;
pub const StorageDiagnosticTargetTypeMax: STORAGE_DIAGNOSTIC_TARGET_TYPE = 4;
pub const StorageDiagnosticTargetTypeMiniport: STORAGE_DIAGNOSTIC_TARGET_TYPE = 2;
pub const StorageDiagnosticTargetTypePort: STORAGE_DIAGNOSTIC_TARGET_TYPE = 1;
pub const StorageDiagnosticTargetTypeUndefined: STORAGE_DIAGNOSTIC_TARGET_TYPE = 0;
pub const StorageEncryptionTypeEDrive: STORAGE_ENCRYPTION_TYPE = 1;
pub const StorageEncryptionTypeTcgOpal: STORAGE_ENCRYPTION_TYPE = 2;
pub const StorageEncryptionTypeUnknown: STORAGE_ENCRYPTION_TYPE = 0;
pub const StorageFruIdProperty: STORAGE_PROPERTY_ID = 65;
pub const StorageHwCryptoProperty: STORAGE_PROPERTY_ID = 69;
pub const StorageIceTypeNvme: STORAGE_ICE_TYPE = 2;
pub const StorageIceTypeUfs: STORAGE_ICE_TYPE = 1;
pub const StorageIceTypeUnknown: STORAGE_ICE_TYPE = 0;
pub const StorageIdAssocDevice: STORAGE_ASSOCIATION_TYPE = 0;
pub const StorageIdAssocPort: STORAGE_ASSOCIATION_TYPE = 1;
pub const StorageIdAssocTarget: STORAGE_ASSOCIATION_TYPE = 2;
pub const StorageIdCodeSetAscii: STORAGE_IDENTIFIER_CODE_SET = 2;
pub const StorageIdCodeSetBinary: STORAGE_IDENTIFIER_CODE_SET = 1;
pub const StorageIdCodeSetReserved: STORAGE_IDENTIFIER_CODE_SET = 0;
pub const StorageIdCodeSetUtf8: STORAGE_IDENTIFIER_CODE_SET = 3;
pub const StorageIdNAAFormatIEEEERegisteredExtended: STORAGE_ID_NAA_FORMAT = 5;
pub const StorageIdNAAFormatIEEEExtended: STORAGE_ID_NAA_FORMAT = 2;
pub const StorageIdNAAFormatIEEERegistered: STORAGE_ID_NAA_FORMAT = 3;
pub const StorageIdTypeEUI64: STORAGE_IDENTIFIER_TYPE = 2;
pub const StorageIdTypeFCPHName: STORAGE_IDENTIFIER_TYPE = 3;
pub const StorageIdTypeLogicalUnitGroup: STORAGE_IDENTIFIER_TYPE = 6;
pub const StorageIdTypeMD5LogicalUnitIdentifier: STORAGE_IDENTIFIER_TYPE = 7;
pub const StorageIdTypeNAA: STORAGE_IDENTIFIER_TYPE = 3;
pub const StorageIdTypePortRelative: STORAGE_IDENTIFIER_TYPE = 4;
pub const StorageIdTypeScsiNameString: STORAGE_IDENTIFIER_TYPE = 8;
pub const StorageIdTypeTargetPortGroup: STORAGE_IDENTIFIER_TYPE = 5;
pub const StorageIdTypeVendorId: STORAGE_IDENTIFIER_TYPE = 1;
pub const StorageIdTypeVendorSpecific: STORAGE_IDENTIFIER_TYPE = 0;
pub const StorageMiniportProperty: STORAGE_PROPERTY_ID = 5;
pub const StoragePortCodeSetATAport: STORAGE_PORT_CODE_SET = 4;
pub const StoragePortCodeSetReserved: STORAGE_PORT_CODE_SET = 0;
pub const StoragePortCodeSetSBP2port: STORAGE_PORT_CODE_SET = 6;
pub const StoragePortCodeSetSCSIport: STORAGE_PORT_CODE_SET = 2;
pub const StoragePortCodeSetSDport: STORAGE_PORT_CODE_SET = 7;
pub const StoragePortCodeSetSpaceport: STORAGE_PORT_CODE_SET = 3;
pub const StoragePortCodeSetStorport: STORAGE_PORT_CODE_SET = 1;
pub const StoragePortCodeSetUSBport: STORAGE_PORT_CODE_SET = 5;
pub const StoragePowerupDeviceAttention: STORAGE_POWERUP_REASON_TYPE = 2;
pub const StoragePowerupIO: STORAGE_POWERUP_REASON_TYPE = 1;
pub const StoragePowerupUnknown: STORAGE_POWERUP_REASON_TYPE = 0;
pub const StorageReserveIdHard: STORAGE_RESERVE_ID = 1;
pub const StorageReserveIdMax: STORAGE_RESERVE_ID = 4;
pub const StorageReserveIdNone: STORAGE_RESERVE_ID = 0;
pub const StorageReserveIdSoft: STORAGE_RESERVE_ID = 2;
pub const StorageReserveIdUpdateScratch: STORAGE_RESERVE_ID = 3;
pub const StorageRpmbFrameTypeMax: STORAGE_RPMB_FRAME_TYPE = 2;
pub const StorageRpmbFrameTypeStandard: STORAGE_RPMB_FRAME_TYPE = 1;
pub const StorageRpmbFrameTypeUnknown: STORAGE_RPMB_FRAME_TYPE = 0;
pub const StorageSanitizeMethodBlockErase: STORAGE_SANITIZE_METHOD = 1;
pub const StorageSanitizeMethodCryptoErase: STORAGE_SANITIZE_METHOD = 2;
pub const StorageSanitizeMethodDefault: STORAGE_SANITIZE_METHOD = 0;
pub const StorageStackProperty: STORAGE_PROPERTY_ID = 66;
pub const StorageStackTypeNVMe: STORAGE_STACK_TYPE = 2;
pub const StorageStackTypeScsi: STORAGE_STACK_TYPE = 1;
pub const StorageStackTypeUnknown: STORAGE_STACK_TYPE = 0;
pub const StorageTierClassCapacity: STORAGE_TIER_CLASS = 1;
pub const StorageTierClassMax: STORAGE_TIER_CLASS = 3;
pub const StorageTierClassPerformance: STORAGE_TIER_CLASS = 2;
pub const StorageTierClassUnspecified: STORAGE_TIER_CLASS = 0;
pub const StorageTierMediaTypeDisk: STORAGE_TIER_MEDIA_TYPE = 1;
pub const StorageTierMediaTypeMax: STORAGE_TIER_MEDIA_TYPE = 5;
pub const StorageTierMediaTypeScm: STORAGE_TIER_MEDIA_TYPE = 4;
pub const StorageTierMediaTypeSsd: STORAGE_TIER_MEDIA_TYPE = 2;
pub const StorageTierMediaTypeUnspecified: STORAGE_TIER_MEDIA_TYPE = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TAPE_GET_STATISTICS {
    pub Operation: super::DWORD,
}
pub const TAPE_RESET_STATISTICS: i32 = 2;
pub const TAPE_RETURN_ENV_INFO: i32 = 1;
pub const TAPE_RETURN_STATISTICS: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TAPE_STATISTICS {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub RecoveredWrites: super::LARGE_INTEGER,
    pub UnrecoveredWrites: super::LARGE_INTEGER,
    pub RecoveredReads: super::LARGE_INTEGER,
    pub UnrecoveredReads: super::LARGE_INTEGER,
    pub CompressionRatioReads: super::BYTE,
    pub CompressionRatioWrites: super::BYTE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TAPE_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TCCollectionApplicationRequested: DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE = 2;
pub const TCCollectionBugCheck: DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE = 1;
pub const TCCollectionDeviceRequested: DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE = 3;
pub const TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH: i32 = 16;
pub const TC_PUBLIC_DATA_TYPE_ATAGP: windows_core::PCSTR = windows_core::s!("ATAGPLogPages");
pub const TC_PUBLIC_DATA_TYPE_ATASMART: windows_core::PCSTR = windows_core::s!("ATASMARTPages");
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG: i32 = 2;
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX: i32 = 16;
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_SMART: i32 = 1;
pub const TELEMETRY_COMMAND_SIZE: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_CREATE_MINIVERSION_INFO {
    pub StructureVersion: super::WORD,
    pub StructureLength: super::WORD,
    pub BaseVersion: super::DWORD,
    pub MiniVersion: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_GET_METADATA_INFO_OUT {
    pub TxfFileId: TXFS_GET_METADATA_INFO_OUT_0,
    pub LockingTransaction: windows_core::GUID,
    pub LastLsn: super::DWORDLONG,
    pub TransactionState: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_GET_METADATA_INFO_OUT_0 {
    pub LowPart: super::LONGLONG,
    pub HighPart: super::LONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_GET_TRANSACTED_VERSION {
    pub ThisBaseVersion: super::DWORD,
    pub LatestVersion: super::DWORD,
    pub ThisMiniVersion: super::WORD,
    pub FirstMiniVersion: super::WORD,
    pub LatestMiniVersion: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_LIST_TRANSACTIONS {
    pub NumberOfTransactions: super::DWORDLONG,
    pub BufferSizeRequired: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_LIST_TRANSACTIONS_ENTRY {
    pub TransactionId: windows_core::GUID,
    pub TransactionState: super::DWORD,
    pub Reserved1: super::DWORD,
    pub Reserved2: super::DWORD,
    pub Reserved3: super::LONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES {
    pub KtmTransaction: windows_core::GUID,
    pub NumberOfFiles: super::DWORDLONG,
    pub BufferSizeRequired: super::DWORDLONG,
    pub Offset: super::DWORDLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    pub Offset: super::DWORDLONG,
    pub NameFlags: super::DWORD,
    pub FileId: super::LONGLONG,
    pub Reserved1: super::DWORD,
    pub Reserved2: super::DWORD,
    pub Reserved3: super::LONGLONG,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_CREATED: i32 = 1;
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_DELETED: i32 = 2;
pub const TXFS_LOGGING_MODE_FULL: i32 = 2;
pub const TXFS_LOGGING_MODE_SIMPLE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_MODIFY_RM {
    pub Flags: super::DWORD,
    pub LogContainerCountMax: super::DWORD,
    pub LogContainerCountMin: super::DWORD,
    pub LogContainerCount: super::DWORD,
    pub LogGrowthIncrement: super::DWORD,
    pub LogAutoShrinkPercentage: super::DWORD,
    pub Reserved: super::DWORDLONG,
    pub LoggingMode: super::WORD,
}
pub const TXFS_MODIFY_RM_VALID_FLAGS: i32 = 261631;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TXFS_QUERY_RM_INFORMATION {
    pub BytesRequired: super::DWORD,
    pub TailLsn: super::DWORDLONG,
    pub CurrentLsn: super::DWORDLONG,
    pub ArchiveTailLsn: super::DWORDLONG,
    pub LogContainerSize: super::DWORDLONG,
    pub HighestVirtualClock: super::LARGE_INTEGER,
    pub LogContainerCount: super::DWORD,
    pub LogContainerCountMax: super::DWORD,
    pub LogContainerCountMin: super::DWORD,
    pub LogGrowthIncrement: super::DWORD,
    pub LogAutoShrinkPercentage: super::DWORD,
    pub Flags: super::DWORD,
    pub LoggingMode: super::WORD,
    pub Reserved: super::WORD,
    pub RmState: super::DWORD,
    pub LogCapacity: super::DWORDLONG,
    pub LogFree: super::DWORDLONG,
    pub TopsSize: super::DWORDLONG,
    pub TopsUsed: super::DWORDLONG,
    pub TransactionCount: super::DWORDLONG,
    pub OnePCCount: super::DWORDLONG,
    pub TwoPCCount: super::DWORDLONG,
    pub NumberLogFileFull: super::DWORDLONG,
    pub OldestTransactionAge: super::DWORDLONG,
    pub RMName: windows_core::GUID,
    pub TmLogPathOffset: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TXFS_QUERY_RM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TXFS_QUERY_RM_INFORMATION_VALID_FLAGS: i32 = 246192;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct TXFS_READ_BACKUP_INFORMATION_OUT {
    pub Anonymous: TXFS_READ_BACKUP_INFORMATION_OUT_0,
}
#[cfg(feature = "shared")]
impl Default for TXFS_READ_BACKUP_INFORMATION_OUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    pub BufferLength: super::DWORD,
    pub Buffer: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START: i32 = 32768;
pub const TXFS_RM_FLAG_ENFORCE_MINIMUM_SIZE: i32 = 4096;
pub const TXFS_RM_FLAG_GROW_LOG: i32 = 1024;
pub const TXFS_RM_FLAG_LOGGING_MODE: i32 = 1;
pub const TXFS_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: i32 = 64;
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX: i32 = 4;
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN: i32 = 8;
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: i32 = 16;
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: i32 = 32;
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: i32 = 128;
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: i32 = 256;
pub const TXFS_RM_FLAG_PREFER_AVAILABILITY: i32 = 131072;
pub const TXFS_RM_FLAG_PREFER_CONSISTENCY: i32 = 65536;
pub const TXFS_RM_FLAG_PRESERVE_CHANGES: i32 = 8192;
pub const TXFS_RM_FLAG_RENAME_RM: i32 = 2;
pub const TXFS_RM_FLAG_RESET_RM_AT_NEXT_START: i32 = 16384;
pub const TXFS_RM_FLAG_SHRINK_LOG: i32 = 2048;
pub const TXFS_RM_STATE_ACTIVE: i32 = 2;
pub const TXFS_RM_STATE_NOT_STARTED: i32 = 0;
pub const TXFS_RM_STATE_SHUTTING_DOWN: i32 = 3;
pub const TXFS_RM_STATE_STARTING: i32 = 1;
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_REDO_LSN: i32 = 1;
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_VIRTUAL_CLOCK: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TXFS_ROLLFORWARD_REDO_INFORMATION {
    pub LastVirtualClock: super::LARGE_INTEGER,
    pub LastRedoLsn: super::DWORDLONG,
    pub HighestRecoveryLsn: super::DWORDLONG,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TXFS_ROLLFORWARD_REDO_VALID_FLAGS: i32 = 3;
pub const TXFS_SAVEPOINT_CLEAR: i32 = 4;
pub const TXFS_SAVEPOINT_CLEAR_ALL: i32 = 16;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_SAVEPOINT_INFORMATION {
    pub KtmTransaction: super::HANDLE,
    pub ActionCode: super::DWORD,
    pub SavepointId: super::DWORD,
}
pub const TXFS_SAVEPOINT_ROLLBACK: i32 = 2;
pub const TXFS_SAVEPOINT_SET: i32 = 1;
pub const TXFS_START_RM_FLAG_LOGGING_MODE: i32 = 1024;
pub const TXFS_START_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: i32 = 32;
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MAX: i32 = 1;
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MIN: i32 = 2;
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_SIZE: i32 = 4;
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: i32 = 8;
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: i32 = 16;
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: i32 = 64;
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: i32 = 128;
pub const TXFS_START_RM_FLAG_PREFER_AVAILABILITY: i32 = 8192;
pub const TXFS_START_RM_FLAG_PREFER_CONSISTENCY: i32 = 4096;
pub const TXFS_START_RM_FLAG_PRESERVE_CHANGES: i32 = 2048;
pub const TXFS_START_RM_FLAG_RECOVER_BEST_EFFORT: i32 = 512;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TXFS_START_RM_INFORMATION {
    pub Flags: super::DWORD,
    pub LogContainerSize: super::DWORDLONG,
    pub LogContainerCountMin: super::DWORD,
    pub LogContainerCountMax: super::DWORD,
    pub LogGrowthIncrement: super::DWORD,
    pub LogAutoShrinkPercentage: super::DWORD,
    pub TmLogPathOffset: super::DWORD,
    pub TmLogPathLength: super::WORD,
    pub LoggingMode: super::WORD,
    pub LogPathLength: super::WORD,
    pub Reserved: super::WORD,
    pub LogPath: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TXFS_START_RM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TXFS_START_RM_VALID_FLAGS: i32 = 15999;
pub const TXFS_TRANSACTED_VERSION_NONTRANSACTED: u32 = 4294967294;
pub const TXFS_TRANSACTED_VERSION_UNCOMMITTED: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXFS_TRANSACTION_ACTIVE_INFO {
    pub TransactionsActiveAtSnapshot: super::BOOLEAN,
}
pub const TXFS_TRANSACTION_STATE_ACTIVE: i32 = 1;
pub const TXFS_TRANSACTION_STATE_NONE: i32 = 0;
pub const TXFS_TRANSACTION_STATE_NOTACTIVE: i32 = 3;
pub const TXFS_TRANSACTION_STATE_PREPARED: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TXFS_WRITE_BACKUP_INFORMATION {
    pub Buffer: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for TXFS_WRITE_BACKUP_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Travan: STORAGE_MEDIA_TYPE = 34;
pub const UNDEFINE_ALTERNATE: i32 = 13;
pub const UNDEFINE_PRIMARY: i32 = 12;
pub const UNLOCK_ELEMENT: i32 = 1;
pub const UNRECOVERED_READS_VALID: i32 = 8;
pub const UNRECOVERED_WRITES_VALID: i32 = 2;
pub const USN_DELETE_FLAG_DELETE: i32 = 1;
pub const USN_DELETE_FLAG_NOTIFY: i32 = 2;
pub const USN_DELETE_VALID_FLAGS: i32 = 3;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type USN_JOURNAL_DATA = USN_JOURNAL_DATA_V1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_JOURNAL_DATA_V0 {
    pub UsnJournalID: super::DWORDLONG,
    pub FirstUsn: super::USN,
    pub NextUsn: super::USN,
    pub LowestValidUsn: super::USN,
    pub MaxUsn: super::USN,
    pub MaximumSize: super::DWORDLONG,
    pub AllocationDelta: super::DWORDLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_JOURNAL_DATA_V1 {
    pub UsnJournalID: super::DWORDLONG,
    pub FirstUsn: super::USN,
    pub NextUsn: super::USN,
    pub LowestValidUsn: super::USN,
    pub MaxUsn: super::USN,
    pub MaximumSize: super::DWORDLONG,
    pub AllocationDelta: super::DWORDLONG,
    pub MinSupportedMajorVersion: super::WORD,
    pub MaxSupportedMajorVersion: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_JOURNAL_DATA_V2 {
    pub UsnJournalID: super::DWORDLONG,
    pub FirstUsn: super::USN,
    pub NextUsn: super::USN,
    pub LowestValidUsn: super::USN,
    pub MaxUsn: super::USN,
    pub MaximumSize: super::DWORDLONG,
    pub AllocationDelta: super::DWORDLONG,
    pub MinSupportedMajorVersion: super::WORD,
    pub MaxSupportedMajorVersion: super::WORD,
    pub Flags: super::DWORD,
    pub RangeTrackChunkSize: super::DWORDLONG,
    pub RangeTrackFileSizeThreshold: super::LONGLONG,
}
pub const USN_PAGE_SIZE: i32 = 4096;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_RANGE_TRACK_OUTPUT {
    pub Usn: super::USN,
}
pub const USN_REASON_BASIC_INFO_CHANGE: i32 = 32768;
pub const USN_REASON_CLOSE: u32 = 2147483648;
pub const USN_REASON_COMPRESSION_CHANGE: i32 = 131072;
pub const USN_REASON_DATA_EXTEND: i32 = 2;
pub const USN_REASON_DATA_OVERWRITE: i32 = 1;
pub const USN_REASON_DATA_TRUNCATION: i32 = 4;
pub const USN_REASON_DESIRED_STORAGE_CLASS_CHANGE: i32 = 16777216;
pub const USN_REASON_EA_CHANGE: i32 = 1024;
pub const USN_REASON_ENCRYPTION_CHANGE: i32 = 262144;
pub const USN_REASON_FILE_CREATE: i32 = 256;
pub const USN_REASON_FILE_DELETE: i32 = 512;
pub const USN_REASON_HARD_LINK_CHANGE: i32 = 65536;
pub const USN_REASON_INDEXABLE_CHANGE: i32 = 16384;
pub const USN_REASON_INTEGRITY_CHANGE: i32 = 8388608;
pub const USN_REASON_NAMED_DATA_EXTEND: i32 = 32;
pub const USN_REASON_NAMED_DATA_OVERWRITE: i32 = 16;
pub const USN_REASON_NAMED_DATA_TRUNCATION: i32 = 64;
pub const USN_REASON_OBJECT_ID_CHANGE: i32 = 524288;
pub const USN_REASON_RENAME_NEW_NAME: i32 = 8192;
pub const USN_REASON_RENAME_OLD_NAME: i32 = 4096;
pub const USN_REASON_REPARSE_POINT_CHANGE: i32 = 1048576;
pub const USN_REASON_SECURITY_CHANGE: i32 = 2048;
pub const USN_REASON_STREAM_CHANGE: i32 = 2097152;
pub const USN_REASON_TRANSACTED_CHANGE: i32 = 4194304;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type USN_RECORD = USN_RECORD_V2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_RECORD_COMMON_HEADER {
    pub RecordLength: super::DWORD,
    pub MajorVersion: super::WORD,
    pub MinorVersion: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_RECORD_EXTENT {
    pub Offset: super::LONGLONG,
    pub Length: super::LONGLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union USN_RECORD_UNION {
    pub Header: USN_RECORD_COMMON_HEADER,
    pub V2: USN_RECORD_V2,
    pub V3: USN_RECORD_V3,
    pub V4: USN_RECORD_V4,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USN_RECORD_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USN_RECORD_V2 {
    pub RecordLength: super::DWORD,
    pub MajorVersion: super::WORD,
    pub MinorVersion: super::WORD,
    pub FileReferenceNumber: super::DWORDLONG,
    pub ParentFileReferenceNumber: super::DWORDLONG,
    pub Usn: super::USN,
    pub TimeStamp: super::LARGE_INTEGER,
    pub Reason: super::DWORD,
    pub SourceInfo: super::DWORD,
    pub SecurityId: super::DWORD,
    pub FileAttributes: super::DWORD,
    pub FileNameLength: super::WORD,
    pub FileNameOffset: super::WORD,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USN_RECORD_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USN_RECORD_V3 {
    pub RecordLength: super::DWORD,
    pub MajorVersion: super::WORD,
    pub MinorVersion: super::WORD,
    pub FileReferenceNumber: super::FILE_ID_128,
    pub ParentFileReferenceNumber: super::FILE_ID_128,
    pub Usn: super::USN,
    pub TimeStamp: super::LARGE_INTEGER,
    pub Reason: super::DWORD,
    pub SourceInfo: super::DWORD,
    pub SecurityId: super::DWORD,
    pub FileAttributes: super::DWORD,
    pub FileNameLength: super::WORD,
    pub FileNameOffset: super::WORD,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USN_RECORD_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USN_RECORD_V4 {
    pub Header: USN_RECORD_COMMON_HEADER,
    pub FileReferenceNumber: super::FILE_ID_128,
    pub ParentFileReferenceNumber: super::FILE_ID_128,
    pub Usn: super::USN,
    pub Reason: super::DWORD,
    pub SourceInfo: super::DWORD,
    pub RemainingExtents: super::DWORD,
    pub NumberOfExtents: super::WORD,
    pub ExtentSize: super::WORD,
    pub Extents: [USN_RECORD_EXTENT; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USN_RECORD_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USN_SOURCE_AUXILIARY_DATA: i32 = 2;
pub const USN_SOURCE_CLIENT_REPLICATION_MANAGEMENT: i32 = 8;
pub const USN_SOURCE_DATA_MANAGEMENT: i32 = 1;
pub const USN_SOURCE_REPLICATION_MANAGEMENT: i32 = 4;
pub const USN_SOURCE_VALID_FLAGS: i32 = 15;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USN_TRACK_MODIFIED_RANGES {
    pub Flags: super::DWORD,
    pub Unused: super::DWORD,
    pub ChunkSize: super::DWORDLONG,
    pub FileSizeThreshold: super::LONGLONG,
}
pub const UfsDataTypeMax: STORAGE_PROTOCOL_UFS_DATA_TYPE = 6;
pub const UfsDataTypeQueryAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = 2;
pub const UfsDataTypeQueryDescriptor: STORAGE_PROTOCOL_UFS_DATA_TYPE = 1;
pub const UfsDataTypeQueryDmeAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = 4;
pub const UfsDataTypeQueryDmePeerAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = 5;
pub const UfsDataTypeQueryFlag: STORAGE_PROTOCOL_UFS_DATA_TYPE = 3;
pub const UfsDataTypeUnknown: STORAGE_PROTOCOL_UFS_DATA_TYPE = 0;
pub const Unknown: MEDIA_TYPE = 0;
pub const VALID_NTFT: i32 = 192;
pub const VALID_WRITE_USN_REASON_MASK: u32 = 2147483649;
pub const VENDOR_ID_LENGTH: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct VERIFY_INFORMATION {
    pub StartingOffset: super::LARGE_INTEGER,
    pub Length: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for VERIFY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT {
    pub NumberOfWorkerThreads: super::DWORD,
    pub Flags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    pub HeaderSize: super::WORD,
    pub Flags: super::DWORD,
    pub NotificationInfoSize: super::DWORD,
    pub NotificationInfoOffset: super::WORD,
    pub ProviderMajorVersion: super::WORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    pub VirtualizationInstanceID: windows_core::GUID,
}
pub type VIRTUAL_STORAGE_BEHAVIOR_CODE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    pub Size: super::DWORD,
    pub BehaviorCode: VIRTUAL_STORAGE_BEHAVIOR_CODE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_STORAGE_TYPE {
    pub DeviceId: super::DWORD,
    pub VendorId: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct VOLUME_BITMAP_BUFFER {
    pub StartingLcn: super::LARGE_INTEGER,
    pub BitmapSize: super::LARGE_INTEGER,
    pub Buffer: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for VOLUME_BITMAP_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct VOLUME_DISK_EXTENTS {
    pub NumberOfDiskExtents: super::DWORD,
    pub Extents: [DISK_EXTENT; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for VOLUME_DISK_EXTENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: super::DWORDLONG,
}
pub const VOLUME_IS_DIRTY: i32 = 1;
pub const VOLUME_SESSION_OPEN: i32 = 4;
pub const VOLUME_UPGRADE_SCHEDULED: i32 = 2;
pub const VXATape: STORAGE_MEDIA_TYPE = 94;
pub const VXATape_1: STORAGE_MEDIA_TYPE = 83;
pub const VXATape_2: STORAGE_MEDIA_TYPE = 84;
pub const VirtualStorageBehaviorCacheWriteBack: VIRTUAL_STORAGE_BEHAVIOR_CODE = 2;
pub const VirtualStorageBehaviorCacheWriteThrough: VIRTUAL_STORAGE_BEHAVIOR_CODE = 1;
pub const VirtualStorageBehaviorRestartIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = 4;
pub const VirtualStorageBehaviorStopIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = 3;
pub const VirtualStorageBehaviorUndefined: VIRTUAL_STORAGE_BEHAVIOR_CODE = 0;
pub const WIM_BOOT_NOT_OS_WIM: i32 = 0;
pub const WIM_BOOT_OS_WIM: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIM_PROVIDER_ADD_OVERLAY_INPUT {
    pub WimType: super::DWORD,
    pub WimIndex: super::DWORD,
    pub WimFileNameOffset: super::DWORD,
    pub WimFileNameLength: super::DWORD,
}
pub const WIM_PROVIDER_CURRENT_VERSION: i32 = 1;
pub const WIM_PROVIDER_EXTERNAL_FLAG_NOT_ACTIVE: i32 = 1;
pub const WIM_PROVIDER_EXTERNAL_FLAG_SUSPENDED: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WIM_PROVIDER_EXTERNAL_INFO {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub DataSourceId: super::LARGE_INTEGER,
    pub ResourceHash: [super::BYTE; 20],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIM_PROVIDER_EXTERNAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIM_PROVIDER_HASH_SIZE: i32 = 20;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WIM_PROVIDER_OVERLAY_ENTRY {
    pub NextEntryOffset: super::DWORD,
    pub DataSourceId: super::LARGE_INTEGER,
    pub WimGuid: windows_core::GUID,
    pub WimFileNameOffset: super::DWORD,
    pub WimType: super::DWORD,
    pub WimIndex: super::DWORD,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIM_PROVIDER_OVERLAY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    pub DataSourceId: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    pub DataSourceId: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    pub DataSourceId: super::LARGE_INTEGER,
    pub WimFileNameOffset: super::DWORD,
    pub WimFileNameLength: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WOF_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WOF_EXTERNAL_FILE_ID {
    pub FileId: super::FILE_ID_128,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WOF_EXTERNAL_INFO {
    pub Version: super::DWORD,
    pub Provider: super::DWORD,
}
pub const WOF_PROVIDER_CLOUD: i32 = 3;
pub const WOF_PROVIDER_FILE: i32 = 2;
pub const WOF_PROVIDER_WIM: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WOF_VERSION_INFO {
    pub WofVersion: super::DWORD,
}
pub type WRITE_CACHE_CHANGE = i32;
pub type WRITE_CACHE_ENABLE = i32;
pub type WRITE_CACHE_TYPE = i32;
pub const WRITE_COMPRESSION_INFO_VALID: i32 = 16;
pub type WRITE_THROUGH = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRITE_USN_REASON_INPUT {
    pub Flags: super::DWORD,
    pub UsnReasonToWrite: super::DWORD,
}
pub const WriteCacheChangeUnknown: WRITE_CACHE_CHANGE = 0;
pub const WriteCacheChangeable: WRITE_CACHE_CHANGE = 2;
pub const WriteCacheDisabled: WRITE_CACHE_ENABLE = 1;
pub const WriteCacheEnableUnknown: WRITE_CACHE_ENABLE = 0;
pub const WriteCacheEnabled: WRITE_CACHE_ENABLE = 2;
pub const WriteCacheNotChangeable: WRITE_CACHE_CHANGE = 1;
pub const WriteCacheTypeNone: WRITE_CACHE_TYPE = 1;
pub const WriteCacheTypeUnknown: WRITE_CACHE_TYPE = 0;
pub const WriteCacheTypeWriteBack: WRITE_CACHE_TYPE = 2;
pub const WriteCacheTypeWriteThrough: WRITE_CACHE_TYPE = 3;
pub const WriteThroughNotSupported: WRITE_THROUGH = 1;
pub const WriteThroughSupported: WRITE_THROUGH = 2;
pub const WriteThroughUnknown: WRITE_THROUGH = 0;
pub const ZoneConditionClosed: STORAGE_ZONE_CONDITION = 4;
pub const ZoneConditionConventional: STORAGE_ZONE_CONDITION = 0;
pub const ZoneConditionEmpty: STORAGE_ZONE_CONDITION = 1;
pub const ZoneConditionExplicitlyOpened: STORAGE_ZONE_CONDITION = 3;
pub const ZoneConditionFull: STORAGE_ZONE_CONDITION = 14;
pub const ZoneConditionImplicitlyOpened: STORAGE_ZONE_CONDITION = 2;
pub const ZoneConditionOffline: STORAGE_ZONE_CONDITION = 15;
pub const ZoneConditionReadOnly: STORAGE_ZONE_CONDITION = 13;
pub const ZoneTypeConventional: STORAGE_ZONE_TYPES = 1;
pub const ZoneTypeMax: STORAGE_ZONE_TYPES = 4;
pub const ZoneTypeSequentialWritePreferred: STORAGE_ZONE_TYPES = 3;
pub const ZoneTypeSequentialWriteRequired: STORAGE_ZONE_TYPES = 2;
pub const ZoneTypeUnknown: STORAGE_ZONE_TYPES = 0;
pub const ZonedDeviceTypeDeviceManaged: STORAGE_ZONED_DEVICE_TYPES = 3;
pub const ZonedDeviceTypeHostAware: STORAGE_ZONED_DEVICE_TYPES = 2;
pub const ZonedDeviceTypeHostManaged: STORAGE_ZONED_DEVICE_TYPES = 1;
pub const ZonedDeviceTypeUnknown: STORAGE_ZONED_DEVICE_TYPES = 0;
pub const ZonesAttributeTypeAndLengthMayDifferent: STORAGE_ZONES_ATTRIBUTES = 0;
pub const ZonesAttributeTypeMayDifferentLengthSame: STORAGE_ZONES_ATTRIBUTES = 3;
pub const ZonesAttributeTypeSameLastZoneLengthDifferent: STORAGE_ZONES_ATTRIBUTES = 2;
pub const ZonesAttributeTypeSameLengthSame: STORAGE_ZONES_ATTRIBUTES = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub OutputVersion: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    pub Size: super::DWORD,
    pub Version: super::DWORD,
    pub SlabSizeInBytes: super::DWORDLONG,
    pub SlabOffsetDeltaInBytes: super::DWORD,
    pub SlabAllocationBitMapBitCount: super::DWORD,
    pub SlabAllocationBitMapLength: super::DWORD,
    pub SlabAllocationBitMap: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for _DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    pub Size: super::DWORD,
    pub Version: super::DWORD,
    pub SlabSizeInBytes: super::DWORDLONG,
    pub SlabOffsetDeltaInBytes: super::DWORDLONG,
    pub SlabAllocationBitMapBitCount: super::DWORD,
    pub SlabAllocationBitMapLength: super::DWORD,
    pub SlabAllocationBitMap: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for _DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_RANGE {
    pub StartingOffset: super::LONGLONG,
    pub LengthInBytes: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_REPAIR_OUTPUT {
    pub ParityExtent: DEVICE_DSM_RANGE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_REPAIR_PARAMETERS {
    pub NumberOfRepairCopies: super::DWORD,
    pub SourceCopy: super::DWORD,
    pub RepairCopies: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for _DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    pub BytesProcessed: super::DWORDLONG,
    pub BytesRepaired: super::DWORDLONG,
    pub BytesFailed: super::DWORDLONG,
    pub ParityExtent: DEVICE_DSM_RANGE,
    pub BytesScrubbed: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_SCRUB_OUTPUT {
    pub BytesProcessed: super::DWORDLONG,
    pub BytesRepaired: super::DWORDLONG,
    pub BytesFailed: super::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    pub TopologyRangeBytes: super::DWORDLONG,
    pub TopologyId: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for _DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct _DEVICE_DSM_RANGE_ERROR_INFO {
    pub Version: super::DWORD,
    pub Flags: super::DWORD,
    pub TotalNumberOfRanges: super::DWORD,
    pub NumberOfRangesReturned: super::DWORD,
    pub Ranges: [DEVICE_STORAGE_RANGE_ATTRIBUTES; 1],
}
#[cfg(feature = "shared")]
impl Default for _DEVICE_DSM_RANGE_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _DEVICE_DSM_REPORT_ZONES_DATA {
    pub Size: super::DWORD,
    pub ZoneCount: super::DWORD,
    pub Attributes: STORAGE_ZONES_ATTRIBUTES,
    pub Reserved0: super::DWORD,
    pub ZoneDescriptors: [STORAGE_ZONE_DESCRIPTOR; 1],
}
#[cfg(feature = "shared")]
impl Default for _DEVICE_DSM_REPORT_ZONES_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _DEVICE_DSM_TIERING_QUERY_INPUT {
    pub Version: super::DWORD,
    pub Size: super::DWORD,
    pub Flags: super::DWORD,
    pub NumberOfTierIds: super::DWORD,
    pub TierIds: [windows_core::GUID; 1],
}
#[cfg(feature = "shared")]
impl Default for _DEVICE_DSM_TIERING_QUERY_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    pub Size: super::DWORD,
    pub Action: DEVICE_DSM_ACTION,
    pub Flags: super::DWORD,
    pub ParameterBlockOffset: super::DWORD,
    pub ParameterBlockLength: super::DWORD,
    pub DataSetRangesOffset: super::DWORD,
    pub DataSetRangesLength: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    pub Size: super::DWORD,
    pub Action: DEVICE_DSM_ACTION,
    pub Flags: super::DWORD,
    pub OperationStatus: super::DWORD,
    pub ExtendedError: super::DWORD,
    pub TargetDetailedError: super::DWORD,
    pub ReservedStatus: super::DWORD,
    pub OutputBlockOffset: super::DWORD,
    pub OutputBlockLength: super::DWORD,
}
