#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ASSERT_ALTERNATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ASSERT_PRIMARY: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct ASYNC_DUPLICATE_EXTENTS_STATUS {
    pub Version: u32,
    pub State: DUPLICATE_EXTENTS_STATE,
    pub SourceFileOffset: u64,
    pub TargetFileOffset: u64,
    pub ByteCount: u64,
    pub BytesDuplicated: u64,
}
impl ::core::marker::Copy for ASYNC_DUPLICATE_EXTENTS_STATUS {}
impl ::core::clone::Clone for ASYNC_DUPLICATE_EXTENTS_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ATAPI_ID_CMD: u32 = 161u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct BIN_COUNT {
    pub BinRange: BIN_RANGE,
    pub BinCount: u32,
}
impl ::core::marker::Copy for BIN_COUNT {}
impl ::core::clone::Clone for BIN_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct BIN_RANGE {
    pub StartValue: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for BIN_RANGE {}
impl ::core::clone::Clone for BIN_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct BIN_RESULTS {
    pub NumberOfBins: u32,
    pub BinCounts: [BIN_COUNT; 1],
}
impl ::core::marker::Copy for BIN_RESULTS {}
impl ::core::clone::Clone for BIN_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type BIN_TYPES = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RequestSize: BIN_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RequestLocation: BIN_TYPES = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct BOOT_AREA_INFO {
    pub BootSectorCount: u32,
    pub BootSectors: [BOOT_AREA_INFO_0; 2],
}
impl ::core::marker::Copy for BOOT_AREA_INFO {}
impl ::core::clone::Clone for BOOT_AREA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct BOOT_AREA_INFO_0 {
    pub Offset: i64,
}
impl ::core::marker::Copy for BOOT_AREA_INFO_0 {}
impl ::core::clone::Clone for BOOT_AREA_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct BULK_SECURITY_TEST_DATA {
    pub DesiredAccess: u32,
    pub SecurityIds: [u32; 1],
}
impl ::core::marker::Copy for BULK_SECURITY_TEST_DATA {}
impl ::core::clone::Clone for BULK_SECURITY_TEST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CAP_ATAPI_ID_CMD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CAP_ATA_ID_CMD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CAP_SMART_CMD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CDB_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type CHANGER_DEVICE_PROBLEM_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemNone: CHANGER_DEVICE_PROBLEM_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemHardware: CHANGER_DEVICE_PROBLEM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCHMError: CHANGER_DEVICE_PROBLEM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemDoorOpen: CHANGER_DEVICE_PROBLEM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCalibrationError: CHANGER_DEVICE_PROBLEM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemTargetFailure: CHANGER_DEVICE_PROBLEM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCHMMoveError: CHANGER_DEVICE_PROBLEM_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCHMZeroError: CHANGER_DEVICE_PROBLEM_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCartridgeInsertError: CHANGER_DEVICE_PROBLEM_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemPositionError: CHANGER_DEVICE_PROBLEM_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemSensorError: CHANGER_DEVICE_PROBLEM_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCartridgeEjectError: CHANGER_DEVICE_PROBLEM_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemGripperError: CHANGER_DEVICE_PROBLEM_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemDriveError: CHANGER_DEVICE_PROBLEM_TYPE = 13i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CHANGER_ELEMENT {
    pub ElementType: ELEMENT_TYPE,
    pub ElementAddress: u32,
}
impl ::core::marker::Copy for CHANGER_ELEMENT {}
impl ::core::clone::Clone for CHANGER_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CHANGER_ELEMENT_LIST {
    pub Element: CHANGER_ELEMENT,
    pub NumberOfElements: u32,
}
impl ::core::marker::Copy for CHANGER_ELEMENT_LIST {}
impl ::core::clone::Clone for CHANGER_ELEMENT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CHANGER_ELEMENT_STATUS {
    pub Element: CHANGER_ELEMENT,
    pub SrcElementAddress: CHANGER_ELEMENT,
    pub Flags: CHANGER_ELEMENT_STATUS_FLAGS,
    pub ExceptionCode: u32,
    pub TargetId: u8,
    pub Lun: u8,
    pub Reserved: u16,
    pub PrimaryVolumeID: [u8; 36],
    pub AlternateVolumeID: [u8; 36],
}
impl ::core::marker::Copy for CHANGER_ELEMENT_STATUS {}
impl ::core::clone::Clone for CHANGER_ELEMENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CHANGER_ELEMENT_STATUS_EX {
    pub Element: CHANGER_ELEMENT,
    pub SrcElementAddress: CHANGER_ELEMENT,
    pub Flags: CHANGER_ELEMENT_STATUS_FLAGS,
    pub ExceptionCode: u32,
    pub TargetId: u8,
    pub Lun: u8,
    pub Reserved: u16,
    pub PrimaryVolumeID: [u8; 36],
    pub AlternateVolumeID: [u8; 36],
    pub VendorIdentification: [u8; 8],
    pub ProductIdentification: [u8; 16],
    pub SerialNumber: [u8; 32],
}
impl ::core::marker::Copy for CHANGER_ELEMENT_STATUS_EX {}
impl ::core::clone::Clone for CHANGER_ELEMENT_STATUS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type CHANGER_ELEMENT_STATUS_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_ACCESS: CHANGER_ELEMENT_STATUS_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_AVOLTAG: CHANGER_ELEMENT_STATUS_FLAGS = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_EXCEPT: CHANGER_ELEMENT_STATUS_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_EXENAB: CHANGER_ELEMENT_STATUS_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_FULL: CHANGER_ELEMENT_STATUS_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_ID_VALID: CHANGER_ELEMENT_STATUS_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_IMPEXP: CHANGER_ELEMENT_STATUS_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_INENAB: CHANGER_ELEMENT_STATUS_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_INVERT: CHANGER_ELEMENT_STATUS_FLAGS = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_LUN_VALID: CHANGER_ELEMENT_STATUS_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_NOT_BUS: CHANGER_ELEMENT_STATUS_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_PVOLTAG: CHANGER_ELEMENT_STATUS_FLAGS = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_SVALID: CHANGER_ELEMENT_STATUS_FLAGS = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_PRODUCT_DATA: CHANGER_ELEMENT_STATUS_FLAGS = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_EXCHANGE_MEDIUM {
    pub Transport: CHANGER_ELEMENT,
    pub Source: CHANGER_ELEMENT,
    pub Destination1: CHANGER_ELEMENT,
    pub Destination2: CHANGER_ELEMENT,
    pub Flip1: super::super::Foundation::BOOLEAN,
    pub Flip2: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGER_EXCHANGE_MEDIUM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGER_EXCHANGE_MEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type CHANGER_FEATURES = u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_BAR_CODE_SCANNER_INSTALLED: CHANGER_FEATURES = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CARTRIDGE_MAGAZINE: CHANGER_FEATURES = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLEANER_ACCESS_NOT_VALID: CHANGER_FEATURES = 262144u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLEANER_SLOT: CHANGER_FEATURES = 64u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLOSE_IEPORT: CHANGER_FEATURES = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_DEVICE_REINITIALIZE_CAPABLE: CHANGER_FEATURES = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_DRIVE_CLEANING_REQUIRED: CHANGER_FEATURES = 65536u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_DRIVE_EMPTY_ON_DOOR_ACCESS: CHANGER_FEATURES = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_EXCHANGE_MEDIA: CHANGER_FEATURES = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_INIT_ELEM_STAT_WITH_RANGE: CHANGER_FEATURES = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_KEYPAD_ENABLE_DISABLE: CHANGER_FEATURES = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_LOCK_UNLOCK: CHANGER_FEATURES = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_MEDIUM_FLIP: CHANGER_FEATURES = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_OPEN_IEPORT: CHANGER_FEATURES = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_POSITION_TO_ELEMENT: CHANGER_FEATURES = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_PREDISMOUNT_EJECT_REQUIRED: CHANGER_FEATURES = 131072u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_PREMOUNT_EJECT_REQUIRED: CHANGER_FEATURES = 524288u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_REPORT_IEPORT_STATE: CHANGER_FEATURES = 2048u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_SERIAL_NUMBER_VALID: CHANGER_FEATURES = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STATUS_NON_VOLATILE: CHANGER_FEATURES = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STORAGE_DRIVE: CHANGER_FEATURES = 4096u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STORAGE_IEPORT: CHANGER_FEATURES = 8192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STORAGE_SLOT: CHANGER_FEATURES = 16384u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STORAGE_TRANSPORT: CHANGER_FEATURES = 32768u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_ASSERT: CHANGER_FEATURES = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_IDENTIFICATION: CHANGER_FEATURES = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_REPLACE: CHANGER_FEATURES = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_SEARCH: CHANGER_FEATURES = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_UNDEFINE: CHANGER_FEATURES = 16777216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_INITIALIZE_ELEMENT_STATUS {
    pub ElementList: CHANGER_ELEMENT_LIST,
    pub BarCodeScan: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGER_INITIALIZE_ELEMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGER_INITIALIZE_ELEMENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_MOVE_MEDIUM {
    pub Transport: CHANGER_ELEMENT,
    pub Source: CHANGER_ELEMENT,
    pub Destination: CHANGER_ELEMENT,
    pub Flip: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGER_MOVE_MEDIUM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGER_MOVE_MEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CHANGER_PRODUCT_DATA {
    pub VendorId: [u8; 8],
    pub ProductId: [u8; 16],
    pub Revision: [u8; 4],
    pub SerialNumber: [u8; 32],
    pub DeviceType: u8,
}
impl ::core::marker::Copy for CHANGER_PRODUCT_DATA {}
impl ::core::clone::Clone for CHANGER_PRODUCT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_READ_ELEMENT_STATUS {
    pub ElementList: CHANGER_ELEMENT_LIST,
    pub VolumeTagInfo: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGER_READ_ELEMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGER_READ_ELEMENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_RESERVED_BIT: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CHANGER_SEND_VOLUME_TAG_INFORMATION {
    pub StartingElement: CHANGER_ELEMENT,
    pub ActionCode: u32,
    pub VolumeIDTemplate: [u8; 40],
}
impl ::core::marker::Copy for CHANGER_SEND_VOLUME_TAG_INFORMATION {}
impl ::core::clone::Clone for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CHANGER_SET_ACCESS {
    pub Element: CHANGER_ELEMENT,
    pub Control: u32,
}
impl ::core::marker::Copy for CHANGER_SET_ACCESS {}
impl ::core::clone::Clone for CHANGER_SET_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGER_SET_POSITION {
    pub Transport: CHANGER_ELEMENT,
    pub Destination: CHANGER_ELEMENT,
    pub Flip: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGER_SET_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGER_SET_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_TO_DRIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_TO_IEPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_TO_SLOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_TO_TRANSPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHECKSUM_TYPE_CRC32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHECKSUM_TYPE_CRC64: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHECKSUM_TYPE_ECC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHECKSUM_TYPE_FIRST_UNUSED_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHECKSUM_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHECKSUM_TYPE_UNCHANGED: i32 = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CLASS_MEDIA_CHANGE_CONTEXT {
    pub MediaChangeCount: u32,
    pub NewState: u32,
}
impl ::core::marker::Copy for CLASS_MEDIA_CHANGE_CONTEXT {}
impl ::core::clone::Clone for CLASS_MEDIA_CHANGE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CLUSTER_RANGE {
    pub StartingCluster: i64,
    pub ClusterCount: i64,
}
impl ::core::marker::Copy for CLUSTER_RANGE {}
impl ::core::clone::Clone for CLUSTER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_DO_NOT_MAP_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_EXCEPTION_ROOT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_ROOT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_BIND_TARGET_ROOT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_LAYER_ROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_SCRATCH_ROOT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_UNION_LAYER_ROOT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_EXCEPTION_ROOT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_ROOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_TARGET_ROOT: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CONTAINER_ROOT_INFO_INPUT {
    pub Flags: u32,
}
impl ::core::marker::Copy for CONTAINER_ROOT_INFO_INPUT {}
impl ::core::clone::Clone for CONTAINER_ROOT_INFO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CONTAINER_ROOT_INFO_OUTPUT {
    pub ContainerRootIdLength: u16,
    pub ContainerRootId: [u8; 1],
}
impl ::core::marker::Copy for CONTAINER_ROOT_INFO_OUTPUT {}
impl ::core::clone::Clone for CONTAINER_ROOT_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_ROOT_INFO_VALID_FLAGS: u32 = 1023u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CONTAINER_VOLUME_STATE {
    pub Flags: u32,
}
impl ::core::marker::Copy for CONTAINER_VOLUME_STATE {}
impl ::core::clone::Clone for CONTAINER_VOLUME_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CONTAINER_VOLUME_STATE_HOSTING_CONTAINER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const COPYFILE_SIS_FLAGS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const COPYFILE_SIS_LINK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const COPYFILE_SIS_REPLACE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CREATE_DISK {
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: CREATE_DISK_0,
}
impl ::core::marker::Copy for CREATE_DISK {}
impl ::core::clone::Clone for CREATE_DISK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union CREATE_DISK_0 {
    pub Mbr: CREATE_DISK_MBR,
    pub Gpt: CREATE_DISK_GPT,
}
impl ::core::marker::Copy for CREATE_DISK_0 {}
impl ::core::clone::Clone for CREATE_DISK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CREATE_DISK_GPT {
    pub DiskId: ::windows_sys::core::GUID,
    pub MaxPartitionCount: u32,
}
impl ::core::marker::Copy for CREATE_DISK_GPT {}
impl ::core::clone::Clone for CREATE_DISK_GPT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CREATE_DISK_MBR {
    pub Signature: u32,
}
impl ::core::marker::Copy for CREATE_DISK_MBR {}
impl ::core::clone::Clone for CREATE_DISK_MBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CREATE_USN_JOURNAL_DATA {
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
}
impl ::core::marker::Copy for CREATE_USN_JOURNAL_DATA {}
impl ::core::clone::Clone for CREATE_USN_JOURNAL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type CSVFS_DISK_CONNECTIVITY = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvFsDiskConnectivityNone: CSVFS_DISK_CONNECTIVITY = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvFsDiskConnectivityMdsNodeOnly: CSVFS_DISK_CONNECTIVITY = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvFsDiskConnectivitySubsetOfNodes: CSVFS_DISK_CONNECTIVITY = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvFsDiskConnectivityAllNodes: CSVFS_DISK_CONNECTIVITY = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type CSV_CONTROL_OP = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlStartRedirectFile: CSV_CONTROL_OP = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlStopRedirectFile: CSV_CONTROL_OP = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryRedirectState: CSV_CONTROL_OP = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryFileRevision: CSV_CONTROL_OP = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryMdsPath: CSV_CONTROL_OP = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryFileRevisionFileId128: CSV_CONTROL_OP = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryVolumeRedirectState: CSV_CONTROL_OP = 10i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlEnableUSNRangeModificationTracking: CSV_CONTROL_OP = 13i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlMarkHandleLocalVolumeMount: CSV_CONTROL_OP = 14i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlUnmarkHandleLocalVolumeMount: CSV_CONTROL_OP = 15i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlGetCsvFsMdsPathV2: CSV_CONTROL_OP = 18i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlDisableCaching: CSV_CONTROL_OP = 19i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlEnableCaching: CSV_CONTROL_OP = 20i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlStartForceDFO: CSV_CONTROL_OP = 21i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlStopForceDFO: CSV_CONTROL_OP = 22i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryMdsPathNoPause: CSV_CONTROL_OP = 23i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlSetVolumeId: CSV_CONTROL_OP = 24i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryVolumeId: CSV_CONTROL_OP = 25i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_CONTROL_PARAM {
    pub Operation: CSV_CONTROL_OP,
    pub Unused: i64,
}
impl ::core::marker::Copy for CSV_CONTROL_PARAM {}
impl ::core::clone::Clone for CSV_CONTROL_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CSV_INVALID_DEVICE_NUMBER: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_IS_OWNED_BY_CSVFS {
    pub OwnedByCSVFS: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSV_IS_OWNED_BY_CSVFS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSV_IS_OWNED_BY_CSVFS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CSV_MGMTLOCK_CHECK_VOLUME_REDIRECTED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_MGMT_LOCK {
    pub Flags: u32,
}
impl ::core::marker::Copy for CSV_MGMT_LOCK {}
impl ::core::clone::Clone for CSV_MGMT_LOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_NAMESPACE_INFO {
    pub Version: u32,
    pub DeviceNumber: u32,
    pub StartingOffset: i64,
    pub SectorSize: u32,
}
impl ::core::marker::Copy for CSV_NAMESPACE_INFO {}
impl ::core::clone::Clone for CSV_NAMESPACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_QUERY_FILE_REVISION {
    pub FileId: i64,
    pub FileRevision: [i64; 3],
}
impl ::core::marker::Copy for CSV_QUERY_FILE_REVISION {}
impl ::core::clone::Clone for CSV_QUERY_FILE_REVISION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    pub FileId: super::super::Storage::FileSystem::FILE_ID_128,
    pub FileRevision: [i64; 3],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for CSV_QUERY_FILE_REVISION_FILE_ID_128 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_QUERY_MDS_PATH {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub PathLength: u32,
    pub Path: [u16; 1],
}
impl ::core::marker::Copy for CSV_QUERY_MDS_PATH {}
impl ::core::clone::Clone for CSV_QUERY_MDS_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_CSV_DIRECT_IO_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_SMB_BYPASS_CSV_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CSV_QUERY_MDS_PATH_FLAG_STORAGE_ON_THIS_NODE_IS_CONNECTED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_QUERY_MDS_PATH_V2 {
    pub Version: i64,
    pub RequiredSize: u32,
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub Flags: u32,
    pub DiskConnectivity: CSVFS_DISK_CONNECTIVITY,
    pub VolumeId: ::windows_sys::core::GUID,
    pub IpAddressOffset: u32,
    pub IpAddressLength: u32,
    pub PathOffset: u32,
    pub PathLength: u32,
}
impl ::core::marker::Copy for CSV_QUERY_MDS_PATH_V2 {}
impl ::core::clone::Clone for CSV_QUERY_MDS_PATH_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CSV_QUERY_MDS_PATH_V2_VERSION_1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_QUERY_REDIRECT_STATE {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub FileRedirected: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSV_QUERY_REDIRECT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSV_QUERY_REDIRECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    pub VetoedFromAltitudeIntegral: u64,
    pub VetoedFromAltitudeDecimal: u64,
    pub Reason: [u16; 256],
}
impl ::core::marker::Copy for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {}
impl ::core::clone::Clone for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_QUERY_VOLUME_ID {
    pub VolumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CSV_QUERY_VOLUME_ID {}
impl ::core::clone::Clone for CSV_QUERY_VOLUME_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CSV_QUERY_VOLUME_REDIRECT_STATE {
    pub MdsNodeId: u32,
    pub DsNodeId: u32,
    pub IsDiskConnected: super::super::Foundation::BOOLEAN,
    pub ClusterEnableDirectIo: super::super::Foundation::BOOLEAN,
    pub DiskConnectivity: CSVFS_DISK_CONNECTIVITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSV_QUERY_VOLUME_REDIRECT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSV_QUERY_VOLUME_REDIRECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_SET_VOLUME_ID {
    pub VolumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CSV_SET_VOLUME_ID {}
impl ::core::clone::Clone for CSV_SET_VOLUME_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DAX_ALLOC_ALIGNMENT_FLAG_FALLBACK_SPECIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DAX_ALLOC_ALIGNMENT_FLAG_MANDATORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DDUMP_FLAG_DATA_READ_FROM_DEVICE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DECRYPTION_STATUS_BUFFER {
    pub NoEncryptedStreams: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DECRYPTION_STATUS_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DECRYPTION_STATUS_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DELETE_USN_JOURNAL_DATA {
    pub UsnJournalID: u64,
    pub DeleteFlags: USN_DELETE_FLAGS,
}
impl ::core::marker::Copy for DELETE_USN_JOURNAL_DATA {}
impl ::core::clone::Clone for DELETE_USN_JOURNAL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type DETECTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DetectNone: DETECTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DetectInt13: DETECTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DetectExInt13: DETECTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICEDUMP_CAP_PRIVATE_SECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICEDUMP_CAP_RESTRICTED_SECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICEDUMP_MAX_IDSTRING: u32 = 32u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_PRIVATE_SUBSECTION {
    pub dwFlags: u32,
    pub GPLogId: GP_LOG_PAGE_DESCRIPTOR,
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for DEVICEDUMP_PRIVATE_SUBSECTION {}
impl ::core::clone::Clone for DEVICEDUMP_PRIVATE_SUBSECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEDUMP_PUBLIC_SUBSECTION {
    pub dwFlags: u32,
    pub GPLogTable: [GP_LOG_PAGE_DESCRIPTOR; 16],
    pub szDescription: [super::super::Foundation::CHAR; 16],
    pub bData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICEDUMP_PUBLIC_SUBSECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICEDUMP_PUBLIC_SUBSECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_RESTRICTED_SUBSECTION {
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for DEVICEDUMP_RESTRICTED_SUBSECTION {}
impl ::core::clone::Clone for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_SECTION_HEADER {
    pub guidDeviceDataId: ::windows_sys::core::GUID,
    pub sOrganizationID: [u8; 16],
    pub dwFirmwareRevision: u32,
    pub sModelNumber: [u8; 32],
    pub szDeviceManufacturingID: [u8; 32],
    pub dwFlags: u32,
    pub bRestrictedPrivateDataVersion: u32,
    pub dwFirmwareIssueId: u32,
    pub szIssueDescriptionString: [u8; 132],
}
impl ::core::marker::Copy for DEVICEDUMP_SECTION_HEADER {}
impl ::core::clone::Clone for DEVICEDUMP_SECTION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_STORAGEDEVICE_DATA {
    pub Descriptor: DEVICEDUMP_STRUCTURE_VERSION,
    pub SectionHeader: DEVICEDUMP_SECTION_HEADER,
    pub dwBufferSize: u32,
    pub dwReasonForCollection: u32,
    pub PublicData: DEVICEDUMP_SUBSECTION_POINTER,
    pub RestrictedData: DEVICEDUMP_SUBSECTION_POINTER,
    pub PrivateData: DEVICEDUMP_SUBSECTION_POINTER,
}
impl ::core::marker::Copy for DEVICEDUMP_STORAGEDEVICE_DATA {}
impl ::core::clone::Clone for DEVICEDUMP_STORAGEDEVICE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    pub Descriptor: DEVICEDUMP_STRUCTURE_VERSION,
    pub dwReasonForCollection: u32,
    pub cDriverName: [u8; 16],
    pub uiNumRecords: u32,
    pub RecordArray: [DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD; 1],
}
impl ::core::marker::Copy for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {}
impl ::core::clone::Clone for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    pub Cdb: [u8; 16],
    pub Command: [u8; 16],
    pub StartTime: u64,
    pub EndTime: u64,
    pub OperationStatus: u32,
    pub OperationError: u32,
    pub StackSpecific: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0,
}
impl ::core::marker::Copy for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {}
impl ::core::clone::Clone for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    pub ExternalStack: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1,
    pub AtaPort: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0,
    pub StorPort: DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2,
}
impl ::core::marker::Copy for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {}
impl ::core::clone::Clone for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {
    pub dwAtaPortSpecific: u32,
}
impl ::core::marker::Copy for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {}
impl ::core::clone::Clone for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {}
impl ::core::clone::Clone for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {
    pub SrbTag: u32,
}
impl ::core::marker::Copy for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {}
impl ::core::clone::Clone for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_STRUCTURE_VERSION {
    pub dwSignature: u32,
    pub dwVersion: u32,
    pub dwSize: u32,
}
impl ::core::marker::Copy for DEVICEDUMP_STRUCTURE_VERSION {}
impl ::core::clone::Clone for DEVICEDUMP_STRUCTURE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICEDUMP_STRUCTURE_VERSION_V1: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_SUBSECTION_POINTER {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for DEVICEDUMP_SUBSECTION_POINTER {}
impl ::core::clone::Clone for DEVICEDUMP_SUBSECTION_POINTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub MaximumTokenLifetime: u32,
    pub DefaultTokenLifetime: u32,
    pub MaximumTransferSize: u64,
    pub OptimalTransferCount: u64,
    pub MaximumDataDescriptors: u32,
    pub MaximumTransferLengthPerDescriptor: u32,
    pub OptimalTransferLengthPerDescriptor: u32,
    pub OptimalTransferLengthGranularity: u16,
    pub Reserved: [u8; 2],
}
impl ::core::marker::Copy for DEVICE_COPY_OFFLOAD_DESCRIPTOR {}
impl ::core::clone::Clone for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub OutputVersion: u32,
}
impl ::core::marker::Copy for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DATA_SET_LBP_STATE_PARAMETERS_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    pub Size: u32,
    pub Version: u32,
    pub SlabSizeInBytes: u64,
    pub SlabOffsetDeltaInBytes: u32,
    pub SlabAllocationBitMapBitCount: u32,
    pub SlabAllocationBitMapLength: u32,
    pub SlabAllocationBitMap: [u32; 1],
}
impl ::core::marker::Copy for DEVICE_DATA_SET_LB_PROVISIONING_STATE {}
impl ::core::clone::Clone for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    pub Size: u32,
    pub Version: u32,
    pub SlabSizeInBytes: u64,
    pub SlabOffsetDeltaInBytes: u64,
    pub SlabAllocationBitMapBitCount: u32,
    pub SlabAllocationBitMapLength: u32,
    pub SlabAllocationBitMap: [u32; 1],
}
impl ::core::marker::Copy for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {}
impl ::core::clone::Clone for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_RANGE {
    pub StartingOffset: i64,
    pub LengthInBytes: u64,
}
impl ::core::marker::Copy for DEVICE_DATA_SET_RANGE {}
impl ::core::clone::Clone for DEVICE_DATA_SET_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_REPAIR_OUTPUT {
    pub ParityExtent: DEVICE_DATA_SET_RANGE,
}
impl ::core::marker::Copy for DEVICE_DATA_SET_REPAIR_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DATA_SET_REPAIR_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_REPAIR_PARAMETERS {
    pub NumberOfRepairCopies: u32,
    pub SourceCopy: u32,
    pub RepairCopies: [u32; 1],
}
impl ::core::marker::Copy for DEVICE_DATA_SET_REPAIR_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    pub BytesProcessed: u64,
    pub BytesRepaired: u64,
    pub BytesFailed: u64,
    pub ParityExtent: DEVICE_DATA_SET_RANGE,
    pub BytesScrubbed: u64,
}
impl ::core::marker::Copy for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_SCRUB_OUTPUT {
    pub BytesProcessed: u64,
    pub BytesRepaired: u64,
    pub BytesFailed: u64,
}
impl ::core::marker::Copy for DEVICE_DATA_SET_SCRUB_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DATA_SET_SCRUB_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    pub TopologyRangeBytes: u64,
    pub TopologyId: [u8; 16],
}
impl ::core::marker::Copy for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_CONVERSION_OUTPUT {
    pub Version: u32,
    pub Source: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DEVICE_DSM_CONVERSION_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DSM_CONVERSION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_DSM_DEFINITION {
    pub Action: u32,
    pub SingleRange: super::super::Foundation::BOOLEAN,
    pub ParameterBlockAlignment: u32,
    pub ParameterBlockLength: u32,
    pub HasOutput: super::super::Foundation::BOOLEAN,
    pub OutputBlockAlignment: u32,
    pub OutputBlockLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICE_DSM_DEFINITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICE_DSM_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_ALLOCATION_CONSOLIDATEABLE_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_ENTIRE_DATA_SET_RANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_PHYSICAL_ADDRESSES_OMIT_TOTAL_RANGES: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_REPAIR_INPUT_TOPOLOGY_ID_PRESENT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_REPAIR_OUTPUT_PARITY_EXTENT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_SCRUB_SKIP_IN_SYNC: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_TRIM_BYPASS_RZAT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_FLAG_TRIM_NOT_FS_ALLOCATED: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_FREE_SPACE_OUTPUT {
    pub Version: u32,
    pub FreeSpace: u64,
}
impl ::core::marker::Copy for DEVICE_DSM_FREE_SPACE_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DSM_FREE_SPACE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_LOST_QUERY_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Alignment: u64,
    pub NumberOfBits: u32,
    pub BitMap: [u32; 1],
}
impl ::core::marker::Copy for DEVICE_DSM_LOST_QUERY_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_LOST_QUERY_PARAMETERS {
    pub Version: u32,
    pub Granularity: u64,
}
impl ::core::marker::Copy for DEVICE_DSM_LOST_QUERY_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DSM_LOST_QUERY_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_NOTIFICATION_PARAMETERS {
    pub Size: u32,
    pub Flags: u32,
    pub NumFileTypeIDs: u32,
    pub FileTypeID: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for DEVICE_DSM_NOTIFICATION_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_NOTIFY_FLAG_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_NOTIFY_FLAG_END: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    pub Size: u32,
    pub TargetPriority: u8,
    pub Reserved: [u8; 3],
}
impl ::core::marker::Copy for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    pub Flags: u32,
    pub TimeToLive: u32,
    pub Reserved: [u32; 2],
}
impl ::core::marker::Copy for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    pub Flags: u32,
    pub Reserved: u32,
    pub TokenOffset: u64,
    pub Token: STORAGE_OFFLOAD_TOKEN,
}
impl ::core::marker::Copy for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_PARAMETERS_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    pub Version: u32,
    pub Flags: u32,
    pub TotalNumberOfRanges: u32,
    pub NumberOfRangesReturned: u32,
    pub Ranges: [DEVICE_STORAGE_ADDRESS_RANGE; 1],
}
impl ::core::marker::Copy for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_RANGE_ERROR_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub TotalNumberOfRanges: u32,
    pub NumberOfRangesReturned: u32,
    pub Ranges: [DEVICE_STORAGE_RANGE_ATTRIBUTES; 1],
}
impl ::core::marker::Copy for DEVICE_DSM_RANGE_ERROR_INFO {}
impl ::core::clone::Clone for DEVICE_DSM_RANGE_ERROR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_RANGE_ERROR_INFO_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_DSM_RANGE_ERROR_OUTPUT_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_DSM_REPORT_ZONES_DATA {
    pub Size: u32,
    pub ZoneCount: u32,
    pub Attributes: STORAGE_ZONES_ATTRIBUTES,
    pub Reserved0: u32,
    pub ZoneDescriptors: [STORAGE_ZONE_DESCRIPTOR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICE_DSM_REPORT_ZONES_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICE_DSM_REPORT_ZONES_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    pub Size: u32,
    pub ReportOption: u8,
    pub Partial: u8,
    pub Reserved: [u8; 2],
}
impl ::core::marker::Copy for DEVICE_DSM_REPORT_ZONES_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_TIERING_QUERY_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NumberOfTierIds: u32,
    pub TierIds: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for DEVICE_DSM_TIERING_QUERY_INPUT {}
impl ::core::clone::Clone for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_TIERING_QUERY_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Alignment: u64,
    pub TotalNumberOfRegions: u32,
    pub NumberOfRegionsReturned: u32,
    pub Regions: [STORAGE_TIER_REGION; 1],
}
impl ::core::marker::Copy for DEVICE_DSM_TIERING_QUERY_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_INTERNAL_STATUS_DATA {
    pub Version: u32,
    pub Size: u32,
    pub T10VendorId: u64,
    pub DataSet1Length: u32,
    pub DataSet2Length: u32,
    pub DataSet3Length: u32,
    pub DataSet4Length: u32,
    pub StatusDataVersion: u8,
    pub Reserved: [u8; 3],
    pub ReasonIdentifier: [u8; 128],
    pub StatusDataLength: u32,
    pub StatusData: [u8; 1],
}
impl ::core::marker::Copy for DEVICE_INTERNAL_STATUS_DATA {}
impl ::core::clone::Clone for DEVICE_INTERNAL_STATUS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceInternalStatusDataRequestTypeUndefined: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceCurrentInternalStatusDataHeader: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceCurrentInternalStatusData: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceSavedInternalStatusDataHeader: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceSavedInternalStatusData: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type DEVICE_INTERNAL_STATUS_DATA_SET = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSetUndefined: DEVICE_INTERNAL_STATUS_DATA_SET = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSet1: DEVICE_INTERNAL_STATUS_DATA_SET = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSet2: DEVICE_INTERNAL_STATUS_DATA_SET = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSet3: DEVICE_INTERNAL_STATUS_DATA_SET = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSet4: DEVICE_INTERNAL_STATUS_DATA_SET = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSetMax: DEVICE_INTERNAL_STATUS_DATA_SET = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_LB_PROVISIONING_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub _bitfield: u8,
    pub Reserved1: [u8; 7],
    pub OptimalUnmapGranularity: u64,
    pub UnmapGranularityAlignment: u64,
    pub MaxUnmapLbaCount: u32,
    pub MaxUnmapBlockDescriptorCount: u32,
}
impl ::core::marker::Copy for DEVICE_LB_PROVISIONING_DESCRIPTOR {}
impl ::core::clone::Clone for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_LOCATION {
    pub Socket: u32,
    pub Slot: u32,
    pub Adapter: u32,
    pub Port: u32,
    pub Anonymous: DEVICE_LOCATION_0,
}
impl ::core::marker::Copy for DEVICE_LOCATION {}
impl ::core::clone::Clone for DEVICE_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union DEVICE_LOCATION_0 {
    pub Anonymous1: DEVICE_LOCATION_0_0,
    pub Anonymous2: DEVICE_LOCATION_0_1,
}
impl ::core::marker::Copy for DEVICE_LOCATION_0 {}
impl ::core::clone::Clone for DEVICE_LOCATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_LOCATION_0_0 {
    pub Channel: u32,
    pub Device: u32,
}
impl ::core::marker::Copy for DEVICE_LOCATION_0_0 {}
impl ::core::clone::Clone for DEVICE_LOCATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_LOCATION_0_1 {
    pub Target: u32,
    pub Lun: u32,
}
impl ::core::marker::Copy for DEVICE_LOCATION_0_1 {}
impl ::core::clone::Clone for DEVICE_LOCATION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    pub Size: u32,
    pub Action: u32,
    pub Flags: u32,
    pub ParameterBlockOffset: u32,
    pub ParameterBlockLength: u32,
    pub DataSetRangesOffset: u32,
    pub DataSetRangesLength: u32,
}
impl ::core::marker::Copy for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {}
impl ::core::clone::Clone for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    pub Size: u32,
    pub Action: u32,
    pub Flags: u32,
    pub OperationStatus: u32,
    pub ExtendedError: u32,
    pub TargetDetailedError: u32,
    pub ReservedStatus: u32,
    pub OutputBlockOffset: u32,
    pub OutputBlockLength: u32,
}
impl ::core::marker::Copy for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {}
impl ::core::clone::Clone for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct DEVICE_MEDIA_INFO {
    pub DeviceSpecific: DEVICE_MEDIA_INFO_0,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for DEVICE_MEDIA_INFO {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for DEVICE_MEDIA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub union DEVICE_MEDIA_INFO_0 {
    pub DiskInfo: DEVICE_MEDIA_INFO_0_0,
    pub RemovableDiskInfo: DEVICE_MEDIA_INFO_0_1,
    pub TapeInfo: DEVICE_MEDIA_INFO_0_2,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for DEVICE_MEDIA_INFO_0 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for DEVICE_MEDIA_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct DEVICE_MEDIA_INFO_0_0 {
    pub Cylinders: i64,
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub TracksPerCylinder: u32,
    pub SectorsPerTrack: u32,
    pub BytesPerSector: u32,
    pub NumberMediaSides: u32,
    pub MediaCharacteristics: u32,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for DEVICE_MEDIA_INFO_0_0 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for DEVICE_MEDIA_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct DEVICE_MEDIA_INFO_0_1 {
    pub Cylinders: i64,
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub TracksPerCylinder: u32,
    pub SectorsPerTrack: u32,
    pub BytesPerSector: u32,
    pub NumberMediaSides: u32,
    pub MediaCharacteristics: u32,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for DEVICE_MEDIA_INFO_0_1 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for DEVICE_MEDIA_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct DEVICE_MEDIA_INFO_0_2 {
    pub MediaType: STORAGE_MEDIA_TYPE,
    pub MediaCharacteristics: u32,
    pub CurrentBlockSize: u32,
    pub BusType: super::super::Storage::FileSystem::STORAGE_BUS_TYPE,
    pub BusSpecificData: DEVICE_MEDIA_INFO_0_2_0,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for DEVICE_MEDIA_INFO_0_2 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for DEVICE_MEDIA_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub union DEVICE_MEDIA_INFO_0_2_0 {
    pub ScsiInformation: DEVICE_MEDIA_INFO_0_2_0_0,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for DEVICE_MEDIA_INFO_0_2_0 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for DEVICE_MEDIA_INFO_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct DEVICE_MEDIA_INFO_0_2_0_0 {
    pub MediumType: u8,
    pub DensityCode: u8,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for DEVICE_MEDIA_INFO_0_2_0_0 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for DEVICE_MEDIA_INFO_0_2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_POWER_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub DeviceAttentionSupported: super::super::Foundation::BOOLEAN,
    pub AsynchronousNotificationSupported: super::super::Foundation::BOOLEAN,
    pub IdlePowerManagementEnabled: super::super::Foundation::BOOLEAN,
    pub D3ColdEnabled: super::super::Foundation::BOOLEAN,
    pub D3ColdSupported: super::super::Foundation::BOOLEAN,
    pub NoVerifyDuringIdlePower: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 2],
    pub IdleTimeoutInMS: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICE_POWER_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICE_POWER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_SEEK_PENALTY_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub IncursSeekPenalty: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICE_SEEK_PENALTY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICE_SEEK_PENALTY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_STORAGE_ADDRESS_RANGE {
    pub StartAddress: i64,
    pub LengthInBytes: u64,
}
impl ::core::marker::Copy for DEVICE_STORAGE_ADDRESS_RANGE {}
impl ::core::clone::Clone for DEVICE_STORAGE_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DEVICE_STORAGE_NO_ERRORS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_STORAGE_RANGE_ATTRIBUTES {
    pub LengthInBytes: u64,
    pub Anonymous: DEVICE_STORAGE_RANGE_ATTRIBUTES_0,
    pub Reserved: u32,
}
impl ::core::marker::Copy for DEVICE_STORAGE_RANGE_ATTRIBUTES {}
impl ::core::clone::Clone for DEVICE_STORAGE_RANGE_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    pub AllFlags: u32,
    pub Anonymous: DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0,
}
impl ::core::marker::Copy for DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {}
impl ::core::clone::Clone for DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {}
impl ::core::clone::Clone for DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_TRIM_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub TrimEnabled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICE_TRIM_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICE_TRIM_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub BenefitsFromWriteAggregation: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Disk_Number: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] }, pid: 5u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Gpt_Name: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] }, pid: 9u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Gpt_Type: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] }, pid: 8u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Mbr_Type: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] }, pid: 7u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Partition_Number: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] }, pid: 6u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Portable: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Removable_Media: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_System_Critical: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows_sys::core::GUID { data1: 1293860584, data2: 2051, data3: 18292, data4: [152, 66, 183, 125, 181, 2, 101, 233] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DISABLE_SMART: u32 = 217u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DISK_ATTRIBUTE_OFFLINE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DISK_ATTRIBUTE_READ_ONLY: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DISK_BINNING: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISK_CACHE_INFORMATION {
    pub ParametersSavable: super::super::Foundation::BOOLEAN,
    pub ReadCacheEnabled: super::super::Foundation::BOOLEAN,
    pub WriteCacheEnabled: super::super::Foundation::BOOLEAN,
    pub ReadRetentionPriority: DISK_CACHE_RETENTION_PRIORITY,
    pub WriteRetentionPriority: DISK_CACHE_RETENTION_PRIORITY,
    pub DisablePrefetchTransferLength: u16,
    pub PrefetchScalar: super::super::Foundation::BOOLEAN,
    pub Anonymous: DISK_CACHE_INFORMATION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISK_CACHE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISK_CACHE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DISK_CACHE_INFORMATION_0 {
    pub ScalarPrefetch: DISK_CACHE_INFORMATION_0_1,
    pub BlockPrefetch: DISK_CACHE_INFORMATION_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISK_CACHE_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISK_CACHE_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISK_CACHE_INFORMATION_0_0 {
    pub Minimum: u16,
    pub Maximum: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISK_CACHE_INFORMATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISK_CACHE_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISK_CACHE_INFORMATION_0_1 {
    pub Minimum: u16,
    pub Maximum: u16,
    pub MaximumBlocks: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISK_CACHE_INFORMATION_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISK_CACHE_INFORMATION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type DISK_CACHE_RETENTION_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const EqualPriority: DISK_CACHE_RETENTION_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const KeepPrefetchedData: DISK_CACHE_RETENTION_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const KeepReadData: DISK_CACHE_RETENTION_PRIORITY = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_CONTROLLER_NUMBER {
    pub ControllerNumber: u32,
    pub DiskNumber: u32,
}
impl ::core::marker::Copy for DISK_CONTROLLER_NUMBER {}
impl ::core::clone::Clone for DISK_CONTROLLER_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_DETECTION_INFO {
    pub SizeOfDetectInfo: u32,
    pub DetectionType: DETECTION_TYPE,
    pub Anonymous: DISK_DETECTION_INFO_0,
}
impl ::core::marker::Copy for DISK_DETECTION_INFO {}
impl ::core::clone::Clone for DISK_DETECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union DISK_DETECTION_INFO_0 {
    pub Anonymous: DISK_DETECTION_INFO_0_0,
}
impl ::core::marker::Copy for DISK_DETECTION_INFO_0 {}
impl ::core::clone::Clone for DISK_DETECTION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_DETECTION_INFO_0_0 {
    pub Int13: DISK_INT13_INFO,
    pub ExInt13: DISK_EX_INT13_INFO,
}
impl ::core::marker::Copy for DISK_DETECTION_INFO_0_0 {}
impl ::core::clone::Clone for DISK_DETECTION_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_EXTENT {
    pub DiskNumber: u32,
    pub StartingOffset: i64,
    pub ExtentLength: i64,
}
impl ::core::marker::Copy for DISK_EXTENT {}
impl ::core::clone::Clone for DISK_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_EX_INT13_INFO {
    pub ExBufferSize: u16,
    pub ExFlags: u16,
    pub ExCylinders: u32,
    pub ExHeads: u32,
    pub ExSectorsPerTrack: u32,
    pub ExSectorsPerDrive: u64,
    pub ExSectorSize: u16,
    pub ExReserved: u16,
}
impl ::core::marker::Copy for DISK_EX_INT13_INFO {}
impl ::core::clone::Clone for DISK_EX_INT13_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_GEOMETRY {
    pub Cylinders: i64,
    pub MediaType: MEDIA_TYPE,
    pub TracksPerCylinder: u32,
    pub SectorsPerTrack: u32,
    pub BytesPerSector: u32,
}
impl ::core::marker::Copy for DISK_GEOMETRY {}
impl ::core::clone::Clone for DISK_GEOMETRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_GEOMETRY_EX {
    pub Geometry: DISK_GEOMETRY,
    pub DiskSize: i64,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for DISK_GEOMETRY_EX {}
impl ::core::clone::Clone for DISK_GEOMETRY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_GROW_PARTITION {
    pub PartitionNumber: u32,
    pub BytesToGrow: i64,
}
impl ::core::marker::Copy for DISK_GROW_PARTITION {}
impl ::core::clone::Clone for DISK_GROW_PARTITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_HISTOGRAM {
    pub DiskSize: i64,
    pub Start: i64,
    pub End: i64,
    pub Average: i64,
    pub AverageRead: i64,
    pub AverageWrite: i64,
    pub Granularity: u32,
    pub Size: u32,
    pub ReadCount: u32,
    pub WriteCount: u32,
    pub Histogram: *mut HISTOGRAM_BUCKET,
}
impl ::core::marker::Copy for DISK_HISTOGRAM {}
impl ::core::clone::Clone for DISK_HISTOGRAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_INT13_INFO {
    pub DriveSelect: u16,
    pub MaxCylinders: u32,
    pub SectorsPerTrack: u16,
    pub MaxHeads: u16,
    pub NumberDrives: u16,
}
impl ::core::marker::Copy for DISK_INT13_INFO {}
impl ::core::clone::Clone for DISK_INT13_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_LOGGING {
    pub Function: u8,
    pub BufferAddress: *mut ::core::ffi::c_void,
    pub BufferSize: u32,
}
impl ::core::marker::Copy for DISK_LOGGING {}
impl ::core::clone::Clone for DISK_LOGGING {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DISK_LOGGING_DUMP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DISK_LOGGING_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DISK_LOGGING_STOP: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_PARTITION_INFO {
    pub SizeOfPartitionInfo: u32,
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: DISK_PARTITION_INFO_0,
}
impl ::core::marker::Copy for DISK_PARTITION_INFO {}
impl ::core::clone::Clone for DISK_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union DISK_PARTITION_INFO_0 {
    pub Mbr: DISK_PARTITION_INFO_0_1,
    pub Gpt: DISK_PARTITION_INFO_0_0,
}
impl ::core::marker::Copy for DISK_PARTITION_INFO_0 {}
impl ::core::clone::Clone for DISK_PARTITION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_PARTITION_INFO_0_0 {
    pub DiskId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DISK_PARTITION_INFO_0_0 {}
impl ::core::clone::Clone for DISK_PARTITION_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_PARTITION_INFO_0_1 {
    pub Signature: u32,
    pub CheckSum: u32,
}
impl ::core::marker::Copy for DISK_PARTITION_INFO_0_1 {}
impl ::core::clone::Clone for DISK_PARTITION_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_PERFORMANCE {
    pub BytesRead: i64,
    pub BytesWritten: i64,
    pub ReadTime: i64,
    pub WriteTime: i64,
    pub IdleTime: i64,
    pub ReadCount: u32,
    pub WriteCount: u32,
    pub QueueDepth: u32,
    pub SplitCount: u32,
    pub QueryTime: i64,
    pub StorageDeviceNumber: u32,
    pub StorageManagerName: [u16; 8],
}
impl ::core::marker::Copy for DISK_PERFORMANCE {}
impl ::core::clone::Clone for DISK_PERFORMANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISK_RECORD {
    pub ByteOffset: i64,
    pub StartTime: i64,
    pub EndTime: i64,
    pub VirtualAddress: *mut ::core::ffi::c_void,
    pub NumberOfBytes: u32,
    pub DeviceNumber: u8,
    pub ReadRequest: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISK_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISK_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DRIVERSTATUS {
    pub bDriverError: u8,
    pub bIDEError: u8,
    pub bReserved: [u8; 2],
    pub dwReserved: [u32; 2],
}
impl ::core::marker::Copy for DRIVERSTATUS {}
impl ::core::clone::Clone for DRIVERSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVE_LAYOUT_INFORMATION {
    pub PartitionCount: u32,
    pub Signature: u32,
    pub PartitionEntry: [PARTITION_INFORMATION; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVE_LAYOUT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVE_LAYOUT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVE_LAYOUT_INFORMATION_EX {
    pub PartitionStyle: u32,
    pub PartitionCount: u32,
    pub Anonymous: DRIVE_LAYOUT_INFORMATION_EX_0,
    pub PartitionEntry: [PARTITION_INFORMATION_EX; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVE_LAYOUT_INFORMATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVE_LAYOUT_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DRIVE_LAYOUT_INFORMATION_EX_0 {
    pub Mbr: DRIVE_LAYOUT_INFORMATION_MBR,
    pub Gpt: DRIVE_LAYOUT_INFORMATION_GPT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVE_LAYOUT_INFORMATION_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVE_LAYOUT_INFORMATION_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DRIVE_LAYOUT_INFORMATION_GPT {
    pub DiskId: ::windows_sys::core::GUID,
    pub StartingUsableOffset: i64,
    pub UsableLength: i64,
    pub MaxPartitionCount: u32,
}
impl ::core::marker::Copy for DRIVE_LAYOUT_INFORMATION_GPT {}
impl ::core::clone::Clone for DRIVE_LAYOUT_INFORMATION_GPT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DRIVE_LAYOUT_INFORMATION_MBR {
    pub Signature: u32,
    pub CheckSum: u32,
}
impl ::core::marker::Copy for DRIVE_LAYOUT_INFORMATION_MBR {}
impl ::core::clone::Clone for DRIVE_LAYOUT_INFORMATION_MBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUPLICATE_EXTENTS_DATA {
    pub FileHandle: super::super::Foundation::HANDLE,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUPLICATE_EXTENTS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUPLICATE_EXTENTS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DUPLICATE_EXTENTS_DATA32 {
    pub FileHandle: u32,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DUPLICATE_EXTENTS_DATA32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DUPLICATE_EXTENTS_DATA32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUPLICATE_EXTENTS_DATA_EX {
    pub Size: usize,
    pub FileHandle: super::super::Foundation::HANDLE,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUPLICATE_EXTENTS_DATA_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUPLICATE_EXTENTS_DATA_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DUPLICATE_EXTENTS_DATA_EX32 {
    pub Size: u32,
    pub FileHandle: u32,
    pub SourceFileOffset: i64,
    pub TargetFileOffset: i64,
    pub ByteCount: i64,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DUPLICATE_EXTENTS_DATA_EX32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DUPLICATE_EXTENTS_DATA_EX32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DUPLICATE_EXTENTS_DATA_EX_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type DUPLICATE_EXTENTS_STATE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileSnapStateInactive: DUPLICATE_EXTENTS_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileSnapStateSource: DUPLICATE_EXTENTS_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileSnapStateTarget: DUPLICATE_EXTENTS_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceDsmActionFlag_NonDestructive: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const EFS_TRACKED_OFFSET_HEADER_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type ELEMENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AllElements: ELEMENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerTransport: ELEMENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerSlot: ELEMENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerIEPort: ELEMENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerDrive: ELEMENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerDoor: ELEMENT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerKeypad: ELEMENT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerMaxElement: ELEMENT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ENABLE_DISABLE_AUTOSAVE: u32 = 210u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ENABLE_DISABLE_AUTO_OFFLINE: u32 = 219u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ENABLE_SMART: u32 = 216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct ENCRYPTED_DATA_INFO {
    pub StartingFileOffset: u64,
    pub OutputBufferOffset: u32,
    pub BytesWithinFileSize: u32,
    pub BytesWithinValidDataLength: u32,
    pub CompressionFormat: u16,
    pub DataUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub EncryptionFormat: u8,
    pub NumberOfDataBlocks: u16,
    pub DataBlockSize: [u32; 1],
}
impl ::core::marker::Copy for ENCRYPTED_DATA_INFO {}
impl ::core::clone::Clone for ENCRYPTED_DATA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ENCRYPTED_DATA_INFO_SPARSE_FILE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct ENCRYPTION_BUFFER {
    pub EncryptionOperation: u32,
    pub Private: [u8; 1],
}
impl ::core::marker::Copy for ENCRYPTION_BUFFER {}
impl ::core::clone::Clone for ENCRYPTION_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ENCRYPTION_FORMAT_DEFAULT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct ENCRYPTION_KEY_CTRL_INPUT {
    pub HeaderSize: u32,
    pub StructureSize: u32,
    pub KeyOffset: u16,
    pub KeySize: u16,
    pub DplLock: u32,
    pub DplUserId: u64,
    pub DplCredentialId: u64,
}
impl ::core::marker::Copy for ENCRYPTION_KEY_CTRL_INPUT {}
impl ::core::clone::Clone for ENCRYPTION_KEY_CTRL_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ERROR_DRIVE_NOT_INSTALLED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ERROR_HISTORY_DIRECTORY_ENTRY_DEFAULT_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ERROR_INIT_STATUS_NEEDED: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ERROR_LABEL_QUESTIONABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ERROR_LABEL_UNREADABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ERROR_SLOT_NOT_PRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ERROR_TRAY_MALFUNCTION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ERROR_UNHANDLED_ERROR: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const EXECUTE_OFFLINE_DIAGS: u32 = 212u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct EXFAT_STATISTICS {
    pub CreateHits: u32,
    pub SuccessfulCreates: u32,
    pub FailedCreates: u32,
    pub NonCachedReads: u32,
    pub NonCachedReadBytes: u32,
    pub NonCachedWrites: u32,
    pub NonCachedWriteBytes: u32,
    pub NonCachedDiskReads: u32,
    pub NonCachedDiskWrites: u32,
}
impl ::core::marker::Copy for EXFAT_STATISTICS {}
impl ::core::clone::Clone for EXFAT_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct EXTENDED_ENCRYPTED_DATA_INFO {
    pub ExtendedCode: u32,
    pub Length: u32,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for EXTENDED_ENCRYPTED_DATA_INFO {}
impl ::core::clone::Clone for EXTENDED_ENCRYPTED_DATA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const EXTEND_IEPORT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FAT_STATISTICS {
    pub CreateHits: u32,
    pub SuccessfulCreates: u32,
    pub FailedCreates: u32,
    pub NonCachedReads: u32,
    pub NonCachedReadBytes: u32,
    pub NonCachedWrites: u32,
    pub NonCachedWriteBytes: u32,
    pub NonCachedDiskReads: u32,
    pub NonCachedDiskWrites: u32,
}
impl ::core::marker::Copy for FAT_STATISTICS {}
impl ::core::clone::Clone for FAT_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILESYSTEM_STATISTICS {
    pub FileSystemType: FILESYSTEM_STATISTICS_TYPE,
    pub Version: u16,
    pub SizeOfCompleteStructure: u32,
    pub UserFileReads: u32,
    pub UserFileReadBytes: u32,
    pub UserDiskReads: u32,
    pub UserFileWrites: u32,
    pub UserFileWriteBytes: u32,
    pub UserDiskWrites: u32,
    pub MetaDataReads: u32,
    pub MetaDataReadBytes: u32,
    pub MetaDataDiskReads: u32,
    pub MetaDataWrites: u32,
    pub MetaDataWriteBytes: u32,
    pub MetaDataDiskWrites: u32,
}
impl ::core::marker::Copy for FILESYSTEM_STATISTICS {}
impl ::core::clone::Clone for FILESYSTEM_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILESYSTEM_STATISTICS_EX {
    pub FileSystemType: FILESYSTEM_STATISTICS_TYPE,
    pub Version: u16,
    pub SizeOfCompleteStructure: u32,
    pub UserFileReads: u64,
    pub UserFileReadBytes: u64,
    pub UserDiskReads: u64,
    pub UserFileWrites: u64,
    pub UserFileWriteBytes: u64,
    pub UserDiskWrites: u64,
    pub MetaDataReads: u64,
    pub MetaDataReadBytes: u64,
    pub MetaDataDiskReads: u64,
    pub MetaDataWrites: u64,
    pub MetaDataWriteBytes: u64,
    pub MetaDataDiskWrites: u64,
}
impl ::core::marker::Copy for FILESYSTEM_STATISTICS_EX {}
impl ::core::clone::Clone for FILESYSTEM_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type FILESYSTEM_STATISTICS_TYPE = u16;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_EXFAT: FILESYSTEM_STATISTICS_TYPE = 3u16;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_FAT: FILESYSTEM_STATISTICS_TYPE = 2u16;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_NTFS: FILESYSTEM_STATISTICS_TYPE = 1u16;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_REFS: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_ALLOCATED_RANGE_BUFFER {
    pub FileOffset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for FILE_ALLOCATED_RANGE_BUFFER {}
impl ::core::clone::Clone for FILE_ALLOCATED_RANGE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_ANY_ACCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_CLEAR_ENCRYPTION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    pub Class: FILE_STORAGE_TIER_CLASS,
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_DESIRED_STORAGE_CLASS_INFORMATION {}
impl ::core::clone::Clone for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_8042_PORT: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_ACPI: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_BATTERY: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_BEEP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_BIOMETRIC: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_BLUETOOTH: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_BUS_EXTENDER: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_CD_ROM_FILE_SYSTEM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_CHANGER: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_CONSOLE: u32 = 80u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_CONTROLLER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_CRYPT_PROVIDER: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_DATALINK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_DEVAPI: u32 = 71u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_DFS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_DFS_FILE_SYSTEM: u32 = 53u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_DFS_VOLUME: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_DISK_FILE_SYSTEM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_EHSTOR: u32 = 70u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_EVENT_COLLECTOR: u32 = 95u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_FILE_SYSTEM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_FIPS: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_FULLSCREEN_VIDEO: u32 = 52u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_GPIO: u32 = 72u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_HOLOGRAPHIC: u32 = 91u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_INFINIBAND: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_INPORT_PORT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_KEYBOARD: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_KS: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_KSEC: u32 = 57u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MAILSLOT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MASS_STORAGE: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MIDI_IN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MIDI_OUT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MODEM: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MOUSE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MT_COMPOSITE: u32 = 66u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MT_TRANSPORT: u32 = 67u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_MULTI_UNC_PROVIDER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_NAMED_PIPE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_NETWORK: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_NETWORK_BROWSER: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_NETWORK_FILE_SYSTEM: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_NETWORK_REDIRECTOR: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_NFP: u32 = 81u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_NULL: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_NVDIMM: u32 = 90u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_PARALLEL_PORT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_PERSISTENT_MEMORY: u32 = 89u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_PHYSICAL_NETCARD: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_PMI: u32 = 69u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_POINT_OF_SERVICE: u32 = 84u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_PRINTER: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_PRM: u32 = 94u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SCANNER: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SCREEN: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SDFXHCI: u32 = 92u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SERENUM: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SERIAL_MOUSE_PORT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SERIAL_PORT: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SMB: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SOUND: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SOUNDWIRE: u32 = 97u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_STORAGE_REPLICATION: u32 = 85u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_STREAMS: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_SYSENV: u32 = 82u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_TAPE_FILE_SYSTEM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_TERMSRV: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_TRANSPORT: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_TRUST_ENV: u32 = 86u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_UCM: u32 = 87u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_UCMTCPCI: u32 = 88u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_UCMUCSI: u32 = 93u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_UNKNOWN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_USB4: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_USBEX: u32 = 73u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_VDM: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_VIDEO: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_VIRTUAL_BLOCK: u32 = 83u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_VIRTUAL_DISK: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_VMBUS: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_WAVE_IN: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_WAVE_OUT: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_DEVICE_WPD: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    pub VolumeFlags: u32,
    pub FlagMask: u32,
    pub Version: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for FILE_FS_PERSISTENT_VOLUME_INFORMATION {}
impl ::core::clone::Clone for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NON_RESIDENT: u64 = 137438953472u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NOT_FOUND: u64 = 4096u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_TOO_SMALL: u64 = 68719476736u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_CLUSTERS_ALREADY_IN_USE: u64 = 32768u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_DENY_DEFRAG: u64 = 274877906944u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_IS_BASE_RECORD: u64 = 524288u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_BASE_RECORD: u64 = 8u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_EXIST: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_IN_USE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_ORPHAN: u64 = 262144u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_REUSED: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INDEX_ENTRY_MISMATCH: u64 = 1099511627776u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ARRAY_LENGTH_COUNT: u64 = 1048576u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_LCN: u64 = 4294967296u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_ORPHAN_RECOVERY_NAME: u64 = 2199023255552u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_PARENT: u64 = 8388608u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_RUN_LENGTH: u64 = 131072u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_INVALID_VCN: u64 = 8589934592u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_LCN_NOT_EXIST: u64 = 65536u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_MULTIPLE_FILE_NAME_ATTRIBUTES: u64 = 4398046511104u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NAME_CONFLICT: u64 = 17179869184u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NOTHING_WRONG: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_NOT_IMPLEMENTED: u64 = 32u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN: u64 = 34359738368u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_ORPHAN_GENERATED: u64 = 512u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_GENERIC_NAMES: u64 = 1073741824u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_OUT_OF_RESOURCE: u64 = 2147483648u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_BASE_RECORD: u64 = 134217728u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_EXIST: u64 = 67108864u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_INDEX: u64 = 268435456u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_IN_USE: u64 = 16777216u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_REUSED: u64 = 33554432u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_POTENTIAL_CROSSLINK: u64 = 8192u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_PREVIOUS_PARENT_STILL_VALID: u64 = 549755813888u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_RECURSIVELY_CORRUPTED: u64 = 256u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_REPAIRED: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_REPAIR_DISABLED: u64 = 128u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SID_MISMATCH: u64 = 4194304u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SID_VALID: u64 = 2097152u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_STALE_INFORMATION: u64 = 16384u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_SYSTEM_FILE: u64 = 16u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_UNABLE_TO_REPAIR: u64 = 64u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_INITIATE_REPAIR_HINT1_VALID_INDEX_ENTRY: u64 = 536870912u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    pub Hint1: u64,
    pub Hint2: u64,
    pub Clsn: u64,
    pub Status: u32,
}
impl ::core::marker::Copy for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {}
impl ::core::clone::Clone for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_LAYOUT_ENTRY {
    pub Version: u32,
    pub NextFileOffset: u32,
    pub Flags: u32,
    pub FileAttributes: u32,
    pub FileReferenceNumber: u64,
    pub FirstNameOffset: u32,
    pub FirstStreamOffset: u32,
    pub ExtraInfoOffset: u32,
    pub ExtraInfoLength: u32,
}
impl ::core::marker::Copy for FILE_LAYOUT_ENTRY {}
impl ::core::clone::Clone for FILE_LAYOUT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_LAYOUT_INFO_ENTRY {
    pub BasicInformation: FILE_LAYOUT_INFO_ENTRY_0,
    pub OwnerId: u32,
    pub SecurityId: u32,
    pub Usn: i64,
    pub StorageReserveId: STORAGE_RESERVE_ID,
}
impl ::core::marker::Copy for FILE_LAYOUT_INFO_ENTRY {}
impl ::core::clone::Clone for FILE_LAYOUT_INFO_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_LAYOUT_INFO_ENTRY_0 {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl ::core::marker::Copy for FILE_LAYOUT_INFO_ENTRY_0 {}
impl ::core::clone::Clone for FILE_LAYOUT_INFO_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_LAYOUT_NAME_ENTRY {
    pub NextNameOffset: u32,
    pub Flags: u32,
    pub ParentFileReferenceNumber: u64,
    pub FileNameLength: u32,
    pub Reserved: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_LAYOUT_NAME_ENTRY {}
impl ::core::clone::Clone for FILE_LAYOUT_NAME_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_LAYOUT_NAME_ENTRY_DOS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_LAYOUT_NAME_ENTRY_PRIMARY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_LEVEL_TRIM {
    pub Key: u32,
    pub NumRanges: u32,
    pub Ranges: [FILE_LEVEL_TRIM_RANGE; 1],
}
impl ::core::marker::Copy for FILE_LEVEL_TRIM {}
impl ::core::clone::Clone for FILE_LEVEL_TRIM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_LEVEL_TRIM_OUTPUT {
    pub NumRangesProcessed: u32,
}
impl ::core::marker::Copy for FILE_LEVEL_TRIM_OUTPUT {}
impl ::core::clone::Clone for FILE_LEVEL_TRIM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_LEVEL_TRIM_RANGE {
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for FILE_LEVEL_TRIM_RANGE {}
impl ::core::clone::Clone for FILE_LEVEL_TRIM_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_MAKE_COMPATIBLE_BUFFER {
    pub CloseDisc: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_MAKE_COMPATIBLE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_MAKE_COMPATIBLE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_OBJECTID_BUFFER {
    pub ObjectId: [u8; 16],
    pub Anonymous: FILE_OBJECTID_BUFFER_0,
}
impl ::core::marker::Copy for FILE_OBJECTID_BUFFER {}
impl ::core::clone::Clone for FILE_OBJECTID_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union FILE_OBJECTID_BUFFER_0 {
    pub Anonymous: FILE_OBJECTID_BUFFER_0_0,
    pub ExtendedInfo: [u8; 48],
}
impl ::core::marker::Copy for FILE_OBJECTID_BUFFER_0 {}
impl ::core::clone::Clone for FILE_OBJECTID_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_OBJECTID_BUFFER_0_0 {
    pub BirthVolumeId: [u8; 16],
    pub BirthObjectId: [u8; 16],
    pub DomainId: [u8; 16],
}
impl ::core::marker::Copy for FILE_OBJECTID_BUFFER_0_0 {}
impl ::core::clone::Clone for FILE_OBJECTID_BUFFER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_PREFETCH {
    pub Type: u32,
    pub Count: u32,
    pub Prefetch: [u64; 1],
}
impl ::core::marker::Copy for FILE_PREFETCH {}
impl ::core::clone::Clone for FILE_PREFETCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_PREFETCH_EX {
    pub Type: u32,
    pub Count: u32,
    pub Context: *mut ::core::ffi::c_void,
    pub Prefetch: [u64; 1],
}
impl ::core::marker::Copy for FILE_PREFETCH_EX {}
impl ::core::clone::Clone for FILE_PREFETCH_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PREFETCH_TYPE_FOR_CREATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PREFETCH_TYPE_FOR_CREATE_EX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PREFETCH_TYPE_FOR_DIRENUM_EX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PREFETCH_TYPE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PROVIDER_COMPRESSION_MAXIMUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PROVIDER_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_PROVIDER_EXTERNAL_INFO_V0 {
    pub Version: u32,
    pub Algorithm: u32,
}
impl ::core::marker::Copy for FILE_PROVIDER_EXTERNAL_INFO_V0 {}
impl ::core::clone::Clone for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_PROVIDER_EXTERNAL_INFO_V1 {
    pub Version: u32,
    pub Algorithm: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_PROVIDER_EXTERNAL_INFO_V1 {}
impl ::core::clone::Clone for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PROVIDER_FLAG_COMPRESS_ON_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_PROVIDER_SINGLE_FILE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    pub DirectoryCount: i64,
    pub FileCount: i64,
    pub FsFormatMajVersion: u16,
    pub FsFormatMinVersion: u16,
    pub FsFormatName: [u16; 12],
    pub FormatTime: i64,
    pub LastUpdateTime: i64,
    pub CopyrightInfo: [u16; 34],
    pub AbstractInfo: [u16; 34],
    pub FormattingImplementationInfo: [u16; 34],
    pub LastModifyingImplementationInfo: [u16; 34],
}
impl ::core::marker::Copy for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {}
impl ::core::clone::Clone for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_QUERY_SPARING_BUFFER {
    pub SparingUnitBytes: u32,
    pub SoftwareSparing: super::super::Foundation::BOOLEAN,
    pub TotalSpareBlocks: u32,
    pub FreeSpareBlocks: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_QUERY_SPARING_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_QUERY_SPARING_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_READ_ACCESS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_REFERENCE_RANGE {
    pub StartingFileReferenceNumber: u64,
    pub EndingFileReferenceNumber: u64,
}
impl ::core::marker::Copy for FILE_REFERENCE_RANGE {}
impl ::core::clone::Clone for FILE_REFERENCE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_REGION_INFO {
    pub FileOffset: i64,
    pub Length: i64,
    pub Usage: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for FILE_REGION_INFO {}
impl ::core::clone::Clone for FILE_REGION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_REGION_INPUT {
    pub FileOffset: i64,
    pub Length: i64,
    pub DesiredUsage: u32,
}
impl ::core::marker::Copy for FILE_REGION_INPUT {}
impl ::core::clone::Clone for FILE_REGION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_REGION_OUTPUT {
    pub Flags: u32,
    pub TotalRegionEntryCount: u32,
    pub RegionEntryCount: u32,
    pub Reserved: u32,
    pub Region: [FILE_REGION_INFO; 1],
}
impl ::core::marker::Copy for FILE_REGION_OUTPUT {}
impl ::core::clone::Clone for FILE_REGION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_REGION_USAGE_HUGE_PAGE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_REGION_USAGE_LARGE_PAGE_ALIGNMENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_REGION_USAGE_OTHER_PAGE_ALIGNMENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_REGION_USAGE_QUERY_ALIGNMENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_REGION_USAGE_VALID_CACHED_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_REGION_USAGE_VALID_NONCACHED_DATA: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_SET_DEFECT_MGMT_BUFFER {
    pub Disable: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_SET_DEFECT_MGMT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_SET_DEFECT_MGMT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_SET_ENCRYPTION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_SET_SPARSE_BUFFER {
    pub SetSparse: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_SET_SPARSE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_SET_SPARSE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_SPECIAL_ACCESS: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_STORAGE_TIER {
    pub Id: ::windows_sys::core::GUID,
    pub Name: [u16; 256],
    pub Description: [u16; 256],
    pub Flags: FILE_STORAGE_TIER_FLAGS,
    pub ProvisionedCapacity: u64,
    pub MediaType: FILE_STORAGE_TIER_MEDIA_TYPE,
    pub Class: FILE_STORAGE_TIER_CLASS,
}
impl ::core::marker::Copy for FILE_STORAGE_TIER {}
impl ::core::clone::Clone for FILE_STORAGE_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type FILE_STORAGE_TIER_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierClassUnspecified: FILE_STORAGE_TIER_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierClassCapacity: FILE_STORAGE_TIER_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierClassPerformance: FILE_STORAGE_TIER_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierClassMax: FILE_STORAGE_TIER_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_DESCRIPTION_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type FILE_STORAGE_TIER_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_NO_SEEK_PENALTY: FILE_STORAGE_TIER_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_PARITY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_READ_CACHE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_SMR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_WRITE_BACK_CACHE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type FILE_STORAGE_TIER_MEDIA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeUnspecified: FILE_STORAGE_TIER_MEDIA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeDisk: FILE_STORAGE_TIER_MEDIA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeSsd: FILE_STORAGE_TIER_MEDIA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeScm: FILE_STORAGE_TIER_MEDIA_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeMax: FILE_STORAGE_TIER_MEDIA_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_STORAGE_TIER_REGION {
    pub TierId: ::windows_sys::core::GUID,
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for FILE_STORAGE_TIER_REGION {}
impl ::core::clone::Clone for FILE_STORAGE_TIER_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_SYSTEM_RECOGNITION_INFORMATION {
    pub FileSystem: [super::super::Foundation::CHAR; 9],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_SYSTEM_RECOGNITION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_END: u32 = 2u32;
pub const FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2638560951, data2: 53926, data3: 19901, data4: [162, 227, 251, 208, 237, 145, 9, 169] };
pub const FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3076672868, data2: 47523, data3: 19704, data4: [128, 17, 91, 134, 201, 64, 231, 183] };
pub const FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 218784929, data2: 14588, data3: 19896, data4: [159, 231, 63, 67, 82, 205, 124, 92] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_TYPE_NOTIFICATION_INPUT {
    pub Flags: u32,
    pub NumFileTypeIDs: u32,
    pub FileTypeID: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for FILE_TYPE_NOTIFICATION_INPUT {}
impl ::core::clone::Clone for FILE_TYPE_NOTIFICATION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_WRITE_ACCESS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_ZERO_DATA_INFORMATION {
    pub FileOffset: i64,
    pub BeyondFinalZero: i64,
}
impl ::core::marker::Copy for FILE_ZERO_DATA_INFORMATION {}
impl ::core::clone::Clone for FILE_ZERO_DATA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_ZERO_DATA_INFORMATION_EX {
    pub FileOffset: i64,
    pub BeyondFinalZero: i64,
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_ZERO_DATA_INFORMATION_EX {}
impl ::core::clone::Clone for FILE_ZERO_DATA_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_ZERO_DATA_INFORMATION_FLAG_PRESERVE_CACHED_DATA: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct FIND_BY_SID_DATA {
    pub Restart: u32,
    pub Sid: super::super::Security::SID,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for FIND_BY_SID_DATA {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for FIND_BY_SID_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FIND_BY_SID_OUTPUT {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FIND_BY_SID_OUTPUT {}
impl ::core::clone::Clone for FIND_BY_SID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FLAG_USN_TRACK_MODIFIED_RANGES_ENABLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FORMAT_EX_PARAMETERS {
    pub MediaType: MEDIA_TYPE,
    pub StartCylinderNumber: u32,
    pub EndCylinderNumber: u32,
    pub StartHeadNumber: u32,
    pub EndHeadNumber: u32,
    pub FormatGapLength: u16,
    pub SectorsPerTrack: u16,
    pub SectorNumber: [u16; 1],
}
impl ::core::marker::Copy for FORMAT_EX_PARAMETERS {}
impl ::core::clone::Clone for FORMAT_EX_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FORMAT_PARAMETERS {
    pub MediaType: MEDIA_TYPE,
    pub StartCylinderNumber: u32,
    pub EndCylinderNumber: u32,
    pub StartHeadNumber: u32,
    pub EndHeadNumber: u32,
}
impl ::core::marker::Copy for FORMAT_PARAMETERS {}
impl ::core::clone::Clone for FORMAT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ADD_OVERLAY: u32 = 623408u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ADVANCE_FILE_ID: u32 = 590532u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ALLOW_EXTENDED_DASD_IO: u32 = 589955u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CLEAN_VOLUME_METADATA: u32 = 590716u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CORRUPTION_HANDLING: u32 = 590432u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CREATE_OR_GET_OBJECT_ID: u32 = 590016u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CREATE_USN_JOURNAL: u32 = 590055u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSC_INTERNAL: u32 = 590255u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_CONTROL: u32 = 590548u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT: u32 = 590420u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_GET_VOLUME_PATH_NAME: u32 = 590416u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_GET_VOLUME_PATH_NAMES_FOR_VOLUME_NAME: u32 = 590424u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_H_BREAKING_SYNC_TUNNEL_REQUEST: u32 = 590564u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_INTERNAL: u32 = 590444u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_MGMT_LOCK: u32 = 590524u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_QUERY_DOWN_LEVEL_FILE_SYSTEM_CHARACTERISTICS: u32 = 590528u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_QUERY_VETO_FILE_DIRECT_IO: u32 = 590540u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_SYNC_TUNNEL_REQUEST: u32 = 590536u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_CSV_TUNNEL_REQUEST: u32 = 590404u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DELETE_CORRUPTED_REFS_CONTAINER: u32 = 590836u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DELETE_EXTERNAL_BACKING: u32 = 590612u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DELETE_OBJECT_ID: u32 = 589984u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DELETE_REPARSE_POINT: u32 = 589996u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DELETE_USN_JOURNAL: u32 = 590072u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DFSR_SET_GHOST_HANDLE_STATE: u32 = 590264u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DISABLE_LOCAL_BUFFERING: u32 = 590520u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DISMOUNT_VOLUME: u32 = 589856u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE: u32 = 623428u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX: u32 = 623592u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ENABLE_PER_IO_FLAGS: u32 = 590892u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ENABLE_UPGRADE: u32 = 622800u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ENCRYPTION_FSCTL_IO: u32 = 590043u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ENCRYPTION_KEY_CONTROL: u32 = 590852u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ENUM_EXTERNAL_BACKING: u32 = 590616u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ENUM_OVERLAY: u32 = 590623u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_ENUM_USN_DATA: u32 = 590003u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_EXTEND_VOLUME: u32 = 590064u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_FILESYSTEM_GET_STATISTICS: u32 = 589920u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_FILESYSTEM_GET_STATISTICS_EX: u32 = 590732u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_FILE_LEVEL_TRIM: u32 = 623112u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_FILE_PREFETCH: u32 = 590112u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_FILE_TYPE_NOTIFICATION: u32 = 590340u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_FIND_FILES_BY_SID: u32 = 589967u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_BOOT_AREA_INFO: u32 = 590384u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_COMPRESSION: u32 = 589884u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_EXTERNAL_BACKING: u32 = 590608u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_FILTER_FILE_IDENTIFIER: u32 = 590788u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_INTEGRITY_INFORMATION: u32 = 590460u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    pub ChecksumAlgorithm: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ChecksumChunkSizeInBytes: u32,
    pub ClusterSizeInBytes: u32,
}
impl ::core::marker::Copy for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {}
impl ::core::clone::Clone for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_NTFS_FILE_RECORD: u32 = 589928u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_NTFS_VOLUME_DATA: u32 = 589924u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_OBJECT_ID: u32 = 589980u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_REFS_VOLUME_DATA: u32 = 590552u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_REPAIR: u32 = 590236u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_REPARSE_POINT: u32 = 589992u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_RETRIEVAL_POINTERS: u32 = 589939u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT: u32 = 590803u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_RETRIEVAL_POINTER_BASE: u32 = 590388u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_RETRIEVAL_POINTER_COUNT: u32 = 590891u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_VOLUME_BITMAP: u32 = 589935u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GET_WOF_VERSION: u32 = 590696u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_GHOST_FILE_EXTENTS: u32 = 623532u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_HCS_ASYNC_TUNNEL_REQUEST: u32 = 590704u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_HCS_SYNC_NO_WRITE_TUNNEL_REQUEST: u32 = 590776u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_HCS_SYNC_TUNNEL_REQUEST: u32 = 590700u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_INITIATE_FILE_METADATA_OPTIMIZATION: u32 = 590684u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_INITIATE_REPAIR: u32 = 590248u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_INTEGRITY_FLAG_CHECKSUM_ENFORCEMENT_OFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_INVALIDATE_VOLUMES: u32 = 589908u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_IS_CSV_FILE: u32 = 590408u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_IS_FILE_ON_CSV_VOLUME: u32 = 590428u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_IS_PATHNAME_VALID: u32 = 589868u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_IS_VOLUME_DIRTY: u32 = 589944u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_IS_VOLUME_MOUNTED: u32 = 589864u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_IS_VOLUME_OWNED_BYCSVFS: u32 = 590456u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_LOCK_VOLUME: u32 = 589848u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_LOOKUP_STREAM_FROM_CLUSTER: u32 = 590332u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_MAKE_MEDIA_COMPATIBLE: u32 = 622896u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_MANAGE_BYPASS_IO: u32 = 590920u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_MARK_AS_SYSTEM_HIVE: u32 = 589903u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_MARK_HANDLE: u32 = 590076u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_MARK_VOLUME_DIRTY: u32 = 589872u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_MOVE_FILE: u32 = 589940u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_NOTIFY_DATA_CHANGE: u32 = 590844u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_NOTIFY_STORAGE_SPACE_ALLOCATION: u32 = 590748u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_OFFLOAD_READ: u32 = 606820u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_OFFLOAD_READ_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub TokenTimeToLive: u32,
    pub Reserved: u32,
    pub FileOffset: u64,
    pub CopyLength: u64,
}
impl ::core::marker::Copy for FSCTL_OFFLOAD_READ_INPUT {}
impl ::core::clone::Clone for FSCTL_OFFLOAD_READ_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_OFFLOAD_READ_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub TransferLength: u64,
    pub Token: [u8; 512],
}
impl ::core::marker::Copy for FSCTL_OFFLOAD_READ_OUTPUT {}
impl ::core::clone::Clone for FSCTL_OFFLOAD_READ_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_OFFLOAD_WRITE: u32 = 623208u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_OFFLOAD_WRITE_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub FileOffset: u64,
    pub CopyLength: u64,
    pub TransferOffset: u64,
    pub Token: [u8; 512],
}
impl ::core::marker::Copy for FSCTL_OFFLOAD_WRITE_INPUT {}
impl ::core::clone::Clone for FSCTL_OFFLOAD_WRITE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_OFFLOAD_WRITE_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub LengthWritten: u64,
}
impl ::core::marker::Copy for FSCTL_OFFLOAD_WRITE_OUTPUT {}
impl ::core::clone::Clone for FSCTL_OFFLOAD_WRITE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_OPBATCH_ACK_CLOSE_PENDING: u32 = 589840u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_OPLOCK_BREAK_ACKNOWLEDGE: u32 = 589836u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_OPLOCK_BREAK_ACK_NO_2: u32 = 589904u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_OPLOCK_BREAK_NOTIFY: u32 = 589844u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_ALLOCATED_RANGES: u32 = 606415u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_ASYNC_DUPLICATE_EXTENTS_STATUS: u32 = 590896u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_BAD_RANGES: u32 = 590828u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_DEPENDENT_VOLUME: u32 = 590320u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_DIRECT_ACCESS_EXTENTS: u32 = 590747u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_DIRECT_IMAGE_ORIGINAL_BASE: u32 = 590756u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_EXTENT_READ_CACHE_INFO: u32 = 590711u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_FAT_BPB: u32 = 589912u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_QUERY_FAT_BPB_BUFFER {
    pub First0x24BytesOfBootSector: [u8; 36],
}
impl ::core::marker::Copy for FSCTL_QUERY_FAT_BPB_BUFFER {}
impl ::core::clone::Clone for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_FILE_LAYOUT: u32 = 590455u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_FILE_METADATA_OPTIMIZATION: u32 = 590688u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_FILE_REGIONS: u32 = 590468u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_FILE_SYSTEM_RECOGNITION: u32 = 590412u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_GHOSTED_FILE_EXTENTS: u32 = 590768u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_ON_DISK_VOLUME_INFO: u32 = 590140u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_PAGEFILE_ENCRYPTION: u32 = 590312u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_PERSISTENT_VOLUME_STATE: u32 = 590396u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_REFS_SMR_VOLUME_INFO: u32 = 590812u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_REFS_VOLUME_COUNTER_INFO: u32 = 590715u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_REGION_INFO: u32 = 590576u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_QUERY_REGION_INFO_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NumberOfTierIds: u32,
    pub TierIds: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for FSCTL_QUERY_REGION_INFO_INPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_REGION_INFO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_QUERY_REGION_INFO_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Alignment: u64,
    pub TotalNumberOfRegions: u32,
    pub NumberOfRegionsReturned: u32,
    pub Regions: [FILE_STORAGE_TIER_REGION; 1],
}
impl ::core::marker::Copy for FSCTL_QUERY_REGION_INFO_OUTPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_RETRIEVAL_POINTERS: u32 = 589883u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT: u32 = 590592u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_SPARING_INFO: u32 = 590136u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_STORAGE_CLASSES: u32 = 590572u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: FILE_STORAGE_TIER_FLAGS,
    pub TotalNumberOfTiers: u32,
    pub NumberOfTiersReturned: u32,
    pub Tiers: [FILE_STORAGE_TIER; 1],
}
impl ::core::marker::Copy for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_USN_JOURNAL: u32 = 590068u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_VOLUME_CONTAINER_STATE: u32 = 590736u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_QUERY_VOLUME_NUMA_INFO: u32 = 590804u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_READ_FILE_USN_DATA: u32 = 590059u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_READ_FROM_PLEX: u32 = 606494u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_READ_RAW_ENCRYPTED: u32 = 590051u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_READ_UNPRIVILEGED_USN_JOURNAL: u32 = 590763u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_READ_USN_JOURNAL: u32 = 590011u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REARRANGE_FILE: u32 = 640032u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_RECALL_FILE: u32 = 590103u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REFS_DEALLOCATE_RANGES: u32 = 590808u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REFS_STREAM_SNAPSHOT_MANAGEMENT: u32 = 590912u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REMOVE_OVERLAY: u32 = 623412u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REPAIR_COPIES: u32 = 639668u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REQUEST_BATCH_OPLOCK: u32 = 589832u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REQUEST_FILTER_OPLOCK: u32 = 589916u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REQUEST_OPLOCK: u32 = 590400u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REQUEST_OPLOCK_LEVEL_1: u32 = 589824u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_REQUEST_OPLOCK_LEVEL_2: u32 = 589828u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_RESET_VOLUME_ALLOCATION_HINTS: u32 = 590316u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_RKF_INTERNAL: u32 = 590511u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SCRUB_DATA: u32 = 590512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SCRUB_UNDISCOVERABLE_ID: u32 = 590840u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SD_GLOBAL_CHANGE: u32 = 590324u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SECURITY_ID_CHECK: u32 = 606391u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_BOOTLOADER_ACCESSED: u32 = 589903u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_COMPRESSION: u32 = 639040u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_DAX_ALLOC_ALIGNMENT_HINT: u32 = 590832u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_DEFECT_MANAGEMENT: u32 = 622900u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_ENCRYPTION: u32 = 590039u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_EXTERNAL_BACKING: u32 = 590604u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_INTEGRITY_INFORMATION: u32 = 639616u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    pub ChecksumAlgorithm: u16,
    pub Reserved: u16,
    pub Flags: u32,
}
impl ::core::marker::Copy for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {}
impl ::core::clone::Clone for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    pub EnableIntegrity: u8,
    pub KeepIntegrityStateUnchanged: u8,
    pub Reserved: u16,
    pub Flags: u32,
    pub Version: u8,
    pub Reserved2: [u8; 7],
}
impl ::core::marker::Copy for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {}
impl ::core::clone::Clone for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_INTEGRITY_INFORMATION_EX: u32 = 590720u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_LAYER_ROOT: u32 = 590740u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_OBJECT_ID: u32 = 589976u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_OBJECT_ID_EXTENDED: u32 = 590012u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_PERSISTENT_VOLUME_STATE: u32 = 590392u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_PURGE_FAILURE_MODE: u32 = 590448u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_REFS_FILE_STRICTLY_SEQUENTIAL: u32 = 590820u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_REFS_SMR_VOLUME_GC_PARAMETERS: u32 = 590816u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_REPAIR: u32 = 590232u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_REPARSE_POINT: u32 = 589988u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_REPARSE_POINT_EX: u32 = 590860u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_SHORT_NAME_BEHAVIOR: u32 = 590260u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_SPARSE: u32 = 590020u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_VOLUME_COMPRESSION_STATE: u32 = 590144u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_ZERO_DATA: u32 = 622792u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SET_ZERO_ON_DEALLOCATION: u32 = 590228u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SHRINK_VOLUME: u32 = 590256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SHUFFLE_FILE: u32 = 639808u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SIS_COPYFILE: u32 = 590080u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SIS_LINK_FILES: u32 = 639236u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SMB_SHARE_FLUSH_AND_PURGE: u32 = 590908u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SPARSE_OVERALLOCATE: u32 = 590668u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SSDI_STORAGE_REQUEST: u32 = 590752u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_START_VIRTUALIZATION_INSTANCE: u32 = 590784u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_START_VIRTUALIZATION_INSTANCE_EX: u32 = 590848u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_STORAGE_QOS_CONTROL: u32 = 590672u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_STREAMS_ASSOCIATE_ID: u32 = 590792u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_STREAMS_QUERY_ID: u32 = 590796u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_STREAMS_QUERY_PARAMETERS: u32 = 590788u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SUSPEND_OVERLAY: u32 = 590724u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SVHDX_ASYNC_TUNNEL_REQUEST: u32 = 590692u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SVHDX_SET_INITIATOR_INFORMATION: u32 = 590600u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_SVHDX_SYNC_TUNNEL_REQUEST: u32 = 590596u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_CREATE_MINIVERSION: u32 = 622972u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_CREATE_SECONDARY_RM: u32 = 622952u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_GET_METADATA_INFO: u32 = 606572u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_GET_TRANSACTED_VERSION: u32 = 606576u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_LIST_TRANSACTIONS: u32 = 606692u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_LIST_TRANSACTION_LOCKED_FILES: u32 = 606688u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_MODIFY_RM: u32 = 622916u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_QUERY_RM_INFORMATION: u32 = 606536u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION: u32 = 606560u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_READ_BACKUP_INFORMATION2: u32 = 590328u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_ROLLFORWARD_REDO: u32 = 622928u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_ROLLFORWARD_UNDO: u32 = 622932u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_SAVEPOINT_INFORMATION: u32 = 622968u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_SHUTDOWN_RM: u32 = 622940u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_START_RM: u32 = 622936u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_TRANSACTION_ACTIVE: u32 = 606604u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION: u32 = 622948u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_TXFS_WRITE_BACKUP_INFORMATION2: u32 = 590336u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_UNLOCK_VOLUME: u32 = 589852u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_UNMAP_SPACE: u32 = 590772u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_UPDATE_OVERLAY: u32 = 623416u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_USN_TRACK_MODIFIED_RANGES: u32 = 590580u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_VIRTUAL_STORAGE_PASSTHROUGH: u32 = 590884u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_VIRTUAL_STORAGE_QUERY_PROPERTY: u32 = 590728u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_VIRTUAL_STORAGE_SET_BEHAVIOR: u32 = 590856u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_WAIT_FOR_REPAIR: u32 = 590240u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_WRITE_RAW_ENCRYPTED: u32 = 590047u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_WRITE_USN_CLOSE_RECORD: u32 = 590063u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSCTL_WRITE_USN_REASON: u32 = 590544u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type FS_BPIO_INFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_INFL_None: FS_BPIO_INFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_INFL_SKIP_STORAGE_STACK_QUERY: FS_BPIO_INFLAGS = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FS_BPIO_INFO {
    pub ActiveBypassIoCount: u32,
    pub StorageDriverNameLen: u16,
    pub StorageDriverName: [u16; 32],
}
impl ::core::marker::Copy for FS_BPIO_INFO {}
impl ::core::clone::Clone for FS_BPIO_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FS_BPIO_INPUT {
    pub Operation: FS_BPIO_OPERATIONS,
    pub InFlags: FS_BPIO_INFLAGS,
    pub Reserved1: u64,
    pub Reserved2: u64,
}
impl ::core::marker::Copy for FS_BPIO_INPUT {}
impl ::core::clone::Clone for FS_BPIO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type FS_BPIO_OPERATIONS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_ENABLE: FS_BPIO_OPERATIONS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_DISABLE: FS_BPIO_OPERATIONS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_QUERY: FS_BPIO_OPERATIONS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_VOLUME_STACK_PAUSE: FS_BPIO_OPERATIONS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_VOLUME_STACK_RESUME: FS_BPIO_OPERATIONS = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_STREAM_PAUSE: FS_BPIO_OPERATIONS = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_STREAM_RESUME: FS_BPIO_OPERATIONS = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_GET_INFO: FS_BPIO_OPERATIONS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_MAX_OPERATION: FS_BPIO_OPERATIONS = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type FS_BPIO_OUTFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_None: FS_BPIO_OUTFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_VOLUME_STACK_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_STREAM_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_FILTER_ATTACH_BLOCKED: FS_BPIO_OUTFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_COMPATIBLE_STORAGE_DRIVER: FS_BPIO_OUTFLAGS = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FS_BPIO_OUTPUT {
    pub Operation: FS_BPIO_OPERATIONS,
    pub OutFlags: FS_BPIO_OUTFLAGS,
    pub Reserved1: u64,
    pub Reserved2: u64,
    pub Anonymous: FS_BPIO_OUTPUT_0,
}
impl ::core::marker::Copy for FS_BPIO_OUTPUT {}
impl ::core::clone::Clone for FS_BPIO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union FS_BPIO_OUTPUT_0 {
    pub Enable: FS_BPIO_RESULTS,
    pub Query: FS_BPIO_RESULTS,
    pub VolumeStackResume: FS_BPIO_RESULTS,
    pub StreamResume: FS_BPIO_RESULTS,
    pub GetInfo: FS_BPIO_INFO,
}
impl ::core::marker::Copy for FS_BPIO_OUTPUT_0 {}
impl ::core::clone::Clone for FS_BPIO_OUTPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FS_BPIO_RESULTS {
    pub OpStatus: u32,
    pub FailingDriverNameLen: u16,
    pub FailingDriverName: [u16; 32],
    pub FailureReasonLen: u16,
    pub FailureReason: [u16; 128],
}
impl ::core::marker::Copy for FS_BPIO_RESULTS {}
impl ::core::clone::Clone for FS_BPIO_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FW_ISSUEID_NO_ISSUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FW_ISSUEID_UNKNOWN: u32 = 4294967295u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct GETVERSIONINPARAMS {
    pub bVersion: u8,
    pub bRevision: u8,
    pub bReserved: u8,
    pub bIDEDeviceMap: u8,
    pub fCapabilities: u32,
    pub dwReserved: [u32; 4],
}
impl ::core::marker::Copy for GETVERSIONINPARAMS {}
impl ::core::clone::Clone for GETVERSIONINPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct GET_CHANGER_PARAMETERS {
    pub Size: u32,
    pub NumberTransportElements: u16,
    pub NumberStorageElements: u16,
    pub NumberCleanerSlots: u16,
    pub NumberIEElements: u16,
    pub NumberDataTransferElements: u16,
    pub NumberOfDoors: u16,
    pub FirstSlotNumber: u16,
    pub FirstDriveNumber: u16,
    pub FirstTransportNumber: u16,
    pub FirstIEPortNumber: u16,
    pub FirstCleanerSlotAddress: u16,
    pub MagazineSize: u16,
    pub DriveCleanTimeout: u32,
    pub Features0: CHANGER_FEATURES,
    pub Features1: GET_CHANGER_PARAMETERS_FEATURES1,
    pub MoveFromTransport: u8,
    pub MoveFromSlot: u8,
    pub MoveFromIePort: u8,
    pub MoveFromDrive: u8,
    pub ExchangeFromTransport: u8,
    pub ExchangeFromSlot: u8,
    pub ExchangeFromIePort: u8,
    pub ExchangeFromDrive: u8,
    pub LockUnlockCapabilities: u8,
    pub PositionCapabilities: u8,
    pub Reserved1: [u8; 2],
    pub Reserved2: [u32; 2],
}
impl ::core::marker::Copy for GET_CHANGER_PARAMETERS {}
impl ::core::clone::Clone for GET_CHANGER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type GET_CHANGER_PARAMETERS_FEATURES1 = u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLEANER_AUTODISMOUNT: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483652u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLEANER_OPS_NOT_SUPPORTED: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483712u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_IEPORT_USER_CONTROL_CLOSE: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483904u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_IEPORT_USER_CONTROL_OPEN: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483776u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_MOVE_EXTENDS_IEPORT: GET_CHANGER_PARAMETERS_FEATURES1 = 2147484160u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_MOVE_RETRACTS_IEPORT: GET_CHANGER_PARAMETERS_FEATURES1 = 2147484672u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_PREDISMOUNT_ALIGN_TO_DRIVE: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483650u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_PREDISMOUNT_ALIGN_TO_SLOT: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_RTN_MEDIA_TO_ORIGINAL_ADDR: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483680u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_SLOTS_USE_TRAYS: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483664u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_TRUE_EXCHANGE_CAPABLE: GET_CHANGER_PARAMETERS_FEATURES1 = 2147483656u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub RequestDataType: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE,
    pub RequestDataSet: DEVICE_INTERNAL_STATUS_DATA_SET,
}
impl ::core::marker::Copy for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {}
impl ::core::clone::Clone for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct GET_DISK_ATTRIBUTES {
    pub Version: u32,
    pub Reserved1: u32,
    pub Attributes: u64,
}
impl ::core::marker::Copy for GET_DISK_ATTRIBUTES {}
impl ::core::clone::Clone for GET_DISK_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct GET_FILTER_FILE_IDENTIFIER_INPUT {
    pub AltitudeLength: u16,
    pub Altitude: [u16; 1],
}
impl ::core::marker::Copy for GET_FILTER_FILE_IDENTIFIER_INPUT {}
impl ::core::clone::Clone for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    pub FilterFileIdentifierLength: u16,
    pub FilterFileIdentifier: [u8; 1],
}
impl ::core::marker::Copy for GET_FILTER_FILE_IDENTIFIER_OUTPUT {}
impl ::core::clone::Clone for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct GET_LENGTH_INFORMATION {
    pub Length: i64,
}
impl ::core::marker::Copy for GET_LENGTH_INFORMATION {}
impl ::core::clone::Clone for GET_LENGTH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct GET_MEDIA_TYPES {
    pub DeviceType: u32,
    pub MediaInfoCount: u32,
    pub MediaInfo: [DEVICE_MEDIA_INFO; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for GET_MEDIA_TYPES {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for GET_MEDIA_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GET_VOLUME_BITMAP_FLAG_MASK_METADATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type GPT_ATTRIBUTES = u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_ATTRIBUTE_PLATFORM_REQUIRED: GPT_ATTRIBUTES = 1u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_NO_DRIVE_LETTER: GPT_ATTRIBUTES = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_HIDDEN: GPT_ATTRIBUTES = 4611686018427387904u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_SHADOW_COPY: GPT_ATTRIBUTES = 2305843009213693952u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_READ_ONLY: GPT_ATTRIBUTES = 1152921504606846976u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_ATTRIBUTE_LEGACY_BIOS_BOOTABLE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_ATTRIBUTE_NO_BLOCK_IO_PROTOCOL: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_DAX: u64 = 288230376151711744u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_OFFLINE: u64 = 576460752303423488u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_SERVICE: u64 = 144115188075855872u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_SPACES_ATTRIBUTE_NO_METADATA: u64 = 9223372036854775808u64;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct GP_LOG_PAGE_DESCRIPTOR {
    pub LogAddress: u16,
    pub LogSectors: u16,
}
impl ::core::marker::Copy for GP_LOG_PAGE_DESCRIPTOR {}
impl ::core::clone::Clone for GP_LOG_PAGE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const GUID_DEVICEDUMP_DRIVER_STORAGE_PORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3665970205, data2: 28994, data3: 19393, data4: [184, 68, 8, 7, 197, 164, 182, 127] };
pub const GUID_DEVICEDUMP_STORAGE_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3638712623, data2: 6827, data3: 19798, data4: [167, 70, 31, 117, 133, 223, 64, 244] };
pub const GUID_DEVINTERFACE_CDCHANGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590610, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_CDROM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590600, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_COMPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2262880736, data2: 32905, data3: 4560, data4: [156, 228, 8, 0, 62, 48, 31, 115] };
pub const GUID_DEVINTERFACE_DISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590599, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_FLOPPY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590609, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_HIDDEN_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2131790376, data2: 38963, data3: 19259, data4: [183, 128, 44, 107, 95, 165, 192, 98] };
pub const GUID_DEVINTERFACE_MEDIUMCHANGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590608, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_PARTITION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590602, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_SCM_PHYSICAL_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1115906205, data2: 19906, data3: 17342, data4: [187, 180, 79, 21, 223, 206, 44, 97] };
pub const GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444344, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVINTERFACE_SERVICE_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1856847234, data2: 9708, data3: 18108, data4: [183, 253, 193, 240, 223, 143, 80, 55] };
pub const GUID_DEVINTERFACE_SES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395364844, data2: 18389, data3: 19955, data4: [181, 175, 154, 223, 60, 242, 62, 72] };
pub const GUID_DEVINTERFACE_STORAGEPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 718077536, data2: 49456, data3: 4562, data4: [176, 130, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_TAPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590603, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_UNIFIED_ACCESS_RPMB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 658799649, data2: 48323, data3: 19719, data4: [160, 91, 163, 57, 91, 180, 238, 231] };
pub const GUID_DEVINTERFACE_VMLUN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1866556953, data2: 40745, data3: 17061, data4: [178, 11, 55, 226, 25, 202, 2, 176] };
pub const GUID_DEVINTERFACE_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590605, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_WRITEONCEDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590604, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_DEVINTERFACE_ZNSDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3094954437, data2: 65499, data3: 17351, data4: [182, 177, 32, 182, 50, 240, 177, 9] };
pub const GUID_SCM_PD_HEALTH_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2644693894, data2: 29429, data3: 20195, data4: [129, 85, 236, 160, 103, 142, 59, 6] };
pub const GUID_SCM_PD_PASSTHROUGH_INVDIMM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1124707376, data2: 3345, data3: 4580, data4: [145, 145, 8, 0, 32, 12, 154, 102] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct HISTOGRAM_BUCKET {
    pub Reads: u32,
    pub Writes: u32,
}
impl ::core::marker::Copy for HISTOGRAM_BUCKET {}
impl ::core::clone::Clone for HISTOGRAM_BUCKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HIST_NO_OF_BUCKETS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IDENTIFY_BUFFER_SIZE: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct IDEREGS {
    pub bFeaturesReg: u8,
    pub bSectorCountReg: u8,
    pub bSectorNumberReg: u8,
    pub bCylLowReg: u8,
    pub bCylHighReg: u8,
    pub bDriveHeadReg: u8,
    pub bCommandReg: u8,
    pub bReserved: u8,
}
impl ::core::marker::Copy for IDEREGS {}
impl ::core::clone::Clone for IDEREGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ID_CMD: u32 = 236u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_BASE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_EXCHANGE_MEDIUM: u32 = 3162144u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_GET_ELEMENT_STATUS: u32 = 3194900u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_GET_PARAMETERS: u32 = 3162112u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_GET_PRODUCT_DATA: u32 = 3162120u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_GET_STATUS: u32 = 3162116u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_INITIALIZE_ELEMENT_STATUS: u32 = 3162136u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_MOVE_MEDIUM: u32 = 3162148u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_QUERY_VOLUME_TAGS: u32 = 3194924u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_REINITIALIZE_TRANSPORT: u32 = 3162152u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_SET_ACCESS: u32 = 3194896u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_CHANGER_SET_POSITION: u32 = 3162140u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_BASE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_CHECK_VERIFY: u32 = 477184u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_CONTROLLER_NUMBER: u32 = 458820u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_CREATE_DISK: u32 = 507992u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_DELETE_DRIVE_LAYOUT: u32 = 508160u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_EJECT_MEDIA: u32 = 477192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_FIND_NEW_DEVICES: u32 = 477208u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_FORMAT_DRIVE: u32 = 508876u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_FORMAT_TRACKS: u32 = 507928u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_FORMAT_TRACKS_EX: u32 = 507948u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_CACHE_INFORMATION: u32 = 475348u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_DISK_ATTRIBUTES: u32 = 458992u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY_EX: u32 = 458912u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_DRIVE_LAYOUT: u32 = 475148u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_DRIVE_LAYOUT_EX: u32 = 458832u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_LENGTH_INFO: u32 = 475228u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_MEDIA_TYPES: u32 = 461824u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_PARTITION_INFO: u32 = 475140u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_PARTITION_INFO_EX: u32 = 458824u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GET_WRITE_CACHE_STATE: u32 = 475356u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_GROW_PARTITION: u32 = 508112u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_HISTOGRAM_DATA: u32 = 458804u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_HISTOGRAM_RESET: u32 = 458808u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_HISTOGRAM_STRUCTURE: u32 = 458800u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_IS_WRITABLE: u32 = 458788u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_LOAD_MEDIA: u32 = 477196u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_LOGGING: u32 = 458792u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_MEDIA_REMOVAL: u32 = 477188u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_PERFORMANCE: u32 = 458784u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_PERFORMANCE_OFF: u32 = 458848u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_REASSIGN_BLOCKS: u32 = 507932u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_REASSIGN_BLOCKS_EX: u32 = 508068u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_RELEASE: u32 = 477204u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_REQUEST_DATA: u32 = 458816u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_REQUEST_STRUCTURE: u32 = 458812u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_RESERVE: u32 = 477200u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_RESET_SNAPSHOT_INFO: u32 = 508432u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_SENSE_DEVICE: u32 = 459744u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_SET_CACHE_INFORMATION: u32 = 508120u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_SET_DISK_ATTRIBUTES: u32 = 508148u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_SET_DRIVE_LAYOUT: u32 = 507920u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_SET_DRIVE_LAYOUT_EX: u32 = 507988u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_SET_PARTITION_INFO: u32 = 507912u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_SET_PARTITION_INFO_EX: u32 = 507980u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_UPDATE_DRIVE_SIZE: u32 = 508104u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_UPDATE_PROPERTIES: u32 = 459072u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_DISK_VERIFY: u32 = 458772u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCMBUS_BASE: u32 = 89u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCMBUS_DEVICE_FUNCTION_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_BUS_GET_LOGICAL_DEVICES: u32 = 5832704u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_BUS_GET_PHYSICAL_DEVICES: u32 = 5832708u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_BUS_GET_REGIONS: u32 = 5832712u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_BUS_QUERY_PROPERTY: u32 = 5832716u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_BUS_RUNTIME_FW_ACTIVATE: u32 = 5865488u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_BUS_SET_PROPERTY: u32 = 5865492u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_LD_GET_INTERLEAVE_SET: u32 = 5835776u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_LOGICAL_DEVICE_FUNCTION_BASE: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_PD_FIRMWARE_ACTIVATE: u32 = 5871624u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_PD_FIRMWARE_DOWNLOAD: u32 = 5871620u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_PD_PASSTHROUGH: u32 = 5888012u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_PD_QUERY_PROPERTY: u32 = 5838848u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_PD_REINITIALIZE_MEDIA: u32 = 5871636u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_PD_SET_PROPERTY: u32 = 5871640u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_PD_UPDATE_MANAGEMENT_STATUS: u32 = 5838864u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SCM_PHYSICAL_DEVICE_FUNCTION_BASE: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SERENUM_EXPOSE_HARDWARE: u32 = 3604992u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SERENUM_GET_PORT_NAME: u32 = 3605004u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SERENUM_PORT_DESC: u32 = 3605000u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SERENUM_REMOVE_HARDWARE: u32 = 3604996u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_SERIAL_LSRMST_INSERT: u32 = 1769596u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_ALLOCATE_BC_STREAM: u32 = 3004420u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_ATTRIBUTE_MANAGEMENT: u32 = 3005596u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_BASE: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_BC_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_BREAK_RESERVATION: u32 = 2969620u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_CHECK_PRIORITY_HINT_SUPPORT: u32 = 2955392u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_CHECK_VERIFY: u32 = 2967552u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_CHECK_VERIFY2: u32 = 2951168u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_DEVICE_POWER_CAP: u32 = 2956436u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_NOTIFY: u32 = 3002820u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_DEVICE_TELEMETRY_QUERY_CAPS: u32 = 3002824u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_DIAGNOSTIC: u32 = 2956448u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_EJECTION_CONTROL: u32 = 2951488u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_EJECT_MEDIA: u32 = 2967560u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_ENABLE_IDLE_POWER: u32 = 2956416u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_EVENT_NOTIFICATION: u32 = 2956432u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_FAILURE_PREDICTION_CONFIG: u32 = 2953476u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_FIND_NEW_DEVICES: u32 = 2967576u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_FIRMWARE_ACTIVATE: u32 = 3005448u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_FIRMWARE_DOWNLOAD: u32 = 3005444u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_FIRMWARE_GET_INFO: u32 = 2956288u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_FREE_BC_STREAM: u32 = 3004424u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_BC_PROPERTIES: u32 = 2971648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_COUNTERS: u32 = 2953480u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_DEVICE_INTERNAL_LOG: u32 = 2956484u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER: u32 = 2953344u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_DEVICE_NUMBER_EX: u32 = 2953348u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY: u32 = 3002816u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY_RAW: u32 = 3002828u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_HOTPLUG_INFO: u32 = 2952212u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_IDLE_POWERUP_REASON: u32 = 2956420u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_LB_PROVISIONING_MAP_RESOURCES: u32 = 2970632u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_MEDIA_SERIAL_NUMBER: u32 = 2952208u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_MEDIA_TYPES: u32 = 2952192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_MEDIA_TYPES_EX: u32 = 2952196u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_GET_PHYSICAL_ELEMENT_STATUS: u32 = 2956452u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_LOAD_MEDIA: u32 = 2967564u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_LOAD_MEDIA2: u32 = 2951180u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_MANAGE_BYPASS_IO: u32 = 2951360u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_MANAGE_DATA_SET_ATTRIBUTES: u32 = 2987012u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_MCN_CONTROL: u32 = 2951492u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_MEDIA_REMOVAL: u32 = 2967556u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_IN: u32 = 2969624u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_PERSISTENT_RESERVE_OUT: u32 = 3002396u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_POWER_ACTIVE: u32 = 2956424u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_POWER_IDLE: u32 = 2956428u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_PREDICT_FAILURE: u32 = 2953472u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_PROTOCOL_COMMAND: u32 = 3003328u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_QUERY_PROPERTY: u32 = 2954240u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_READ_CAPACITY: u32 = 2969920u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_REINITIALIZE_MEDIA: u32 = 2987584u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_RELEASE: u32 = 2967572u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_REMOVE_ELEMENT_AND_TRUNCATE: u32 = 2956480u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_RESERVE: u32 = 2967568u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_RESET_BUS: u32 = 2969600u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_RESET_DEVICE: u32 = 2969604u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_RPMB_COMMAND: u32 = 2956440u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_SET_HOTPLUG_INFO: u32 = 3001368u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_SET_PROPERTY: u32 = 2987004u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD: u32 = 3002880u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_START_DATA_INTEGRITY_CHECK: u32 = 3004548u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOCTL_STORAGE_STOP_DATA_INTEGRITY_CHECK: u32 = 3004552u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct IO_IRP_EXT_TRACK_OFFSET_HEADER {
    pub Validation: u16,
    pub Flags: u16,
    pub TrackedOffsetCallback: PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK,
}
impl ::core::marker::Copy for IO_IRP_EXT_TRACK_OFFSET_HEADER {}
impl ::core::clone::Clone for IO_IRP_EXT_TRACK_OFFSET_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOCK_ELEMENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOCK_UNLOCK_DOOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOCK_UNLOCK_IEPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOCK_UNLOCK_KEYPAD: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    pub OffsetToNext: u32,
    pub Flags: u32,
    pub Reserved: i64,
    pub Cluster: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {}
impl ::core::clone::Clone for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_DATA: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_INDEX: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_SYSTEM: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_DENY_DEFRAG_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_FS_SYSTEM_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_PAGE_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_TXF_SYSTEM_FILE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    pub Flags: u32,
    pub NumberOfClusters: u32,
    pub Cluster: [i64; 1],
}
impl ::core::marker::Copy for LOOKUP_STREAM_FROM_CLUSTER_INPUT {}
impl ::core::clone::Clone for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    pub Offset: u32,
    pub NumberOfMatches: u32,
    pub BufferSizeRequired: u32,
}
impl ::core::marker::Copy for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {}
impl ::core::clone::Clone for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_CLOUD_SYNC: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_DISABLE_FILE_METADATA_OPTIMIZATION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_ENABLE_CPU_CACHE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_ENABLE_USN_SOURCE_ON_PAGING_IO: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_FILTER_METADATA: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MARK_HANDLE_INFO {
    pub Anonymous: MARK_HANDLE_INFO_0,
    pub VolumeHandle: super::super::Foundation::HANDLE,
    pub HandleInfo: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MARK_HANDLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MARK_HANDLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MARK_HANDLE_INFO_0 {
    pub UsnSourceInfo: u32,
    pub CopyNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MARK_HANDLE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MARK_HANDLE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MARK_HANDLE_INFO32 {
    pub Anonymous: MARK_HANDLE_INFO32_0,
    pub VolumeHandle: u32,
    pub HandleInfo: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MARK_HANDLE_INFO32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MARK_HANDLE_INFO32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub union MARK_HANDLE_INFO32_0 {
    pub UsnSourceInfo: u32,
    pub CopyNumber: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MARK_HANDLE_INFO32_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MARK_HANDLE_INFO32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_NOT_READ_COPY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_NOT_REALTIME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_NOT_TXF_SYSTEM_LOG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_PROTECT_CLUSTERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_READ_COPY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_REALTIME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_RETURN_PURGE_FAILURE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_SKIP_COHERENCY_SYNC_DISALLOW_WRITES: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_SUPPRESS_VOLUME_OPEN_FLUSH: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MARK_HANDLE_TXF_SYSTEM_LOG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MAXIMUM_ENCRYPTION_VALUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MAX_FW_BUCKET_ID_LENGTH: u32 = 132u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MAX_INTERFACE_CODES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MAX_VOLUME_ID_SIZE: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MAX_VOLUME_TEMPLATE_SIZE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MEDIA_CURRENTLY_MOUNTED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MEDIA_ERASEABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MEDIA_READ_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MEDIA_READ_WRITE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type MEDIA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const Unknown: MEDIA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_1Pt2_512: MEDIA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_1Pt44_512: MEDIA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_2Pt88_512: MEDIA_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_20Pt8_512: MEDIA_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_720_512: MEDIA_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_360_512: MEDIA_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_320_512: MEDIA_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_320_1024: MEDIA_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_180_512: MEDIA_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_160_512: MEDIA_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RemovableMedia: MEDIA_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FixedMedia: MEDIA_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_120M_512: MEDIA_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_640_512: MEDIA_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_640_512: MEDIA_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_720_512: MEDIA_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_1Pt2_512: MEDIA_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_1Pt23_1024: MEDIA_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_1Pt23_1024: MEDIA_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_128Mb_512: MEDIA_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_230Mb_512: MEDIA_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F8_256_128: MEDIA_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_200Mb_512: MEDIA_TYPE = 23i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_240M_512: MEDIA_TYPE = 24i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_32M_512: MEDIA_TYPE = 25i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MEDIA_WRITE_ONCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MEDIA_WRITE_PROTECTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const METHOD_BUFFERED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const METHOD_DIRECT_FROM_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const METHOD_DIRECT_TO_HARDWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const METHOD_IN_DIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const METHOD_NEITHER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const METHOD_OUT_DIRECT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct MFT_ENUM_DATA_V0 {
    pub StartFileReferenceNumber: u64,
    pub LowUsn: i64,
    pub HighUsn: i64,
}
impl ::core::marker::Copy for MFT_ENUM_DATA_V0 {}
impl ::core::clone::Clone for MFT_ENUM_DATA_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct MFT_ENUM_DATA_V1 {
    pub StartFileReferenceNumber: u64,
    pub LowUsn: i64,
    pub HighUsn: i64,
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
impl ::core::marker::Copy for MFT_ENUM_DATA_V1 {}
impl ::core::clone::Clone for MFT_ENUM_DATA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MOVE_FILE_DATA {
    pub FileHandle: super::super::Foundation::HANDLE,
    pub StartingVcn: i64,
    pub StartingLcn: i64,
    pub ClusterCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MOVE_FILE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MOVE_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MOVE_FILE_DATA32 {
    pub FileHandle: u32,
    pub StartingVcn: i64,
    pub StartingLcn: i64,
    pub ClusterCount: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MOVE_FILE_DATA32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MOVE_FILE_DATA32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MOVE_FILE_RECORD_DATA {
    pub FileHandle: super::super::Foundation::HANDLE,
    pub SourceFileRecord: i64,
    pub TargetFileRecord: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MOVE_FILE_RECORD_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MOVE_FILE_RECORD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_EXTENDED_VOLUME_DATA {
    pub ByteCount: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BytesPerPhysicalSector: u32,
    pub LfsMajorVersion: u16,
    pub LfsMinorVersion: u16,
    pub MaxDeviceTrimExtentCount: u32,
    pub MaxDeviceTrimByteCount: u32,
    pub MaxVolumeTrimExtentCount: u32,
    pub MaxVolumeTrimByteCount: u32,
}
impl ::core::marker::Copy for NTFS_EXTENDED_VOLUME_DATA {}
impl ::core::clone::Clone for NTFS_EXTENDED_VOLUME_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_FILE_RECORD_INPUT_BUFFER {
    pub FileReferenceNumber: i64,
}
impl ::core::marker::Copy for NTFS_FILE_RECORD_INPUT_BUFFER {}
impl ::core::clone::Clone for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_FILE_RECORD_OUTPUT_BUFFER {
    pub FileReferenceNumber: i64,
    pub FileRecordLength: u32,
    pub FileRecordBuffer: [u8; 1],
}
impl ::core::marker::Copy for NTFS_FILE_RECORD_OUTPUT_BUFFER {}
impl ::core::clone::Clone for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS {
    pub LogFileFullExceptions: u32,
    pub OtherExceptions: u32,
    pub MftReads: u32,
    pub MftReadBytes: u32,
    pub MftWrites: u32,
    pub MftWriteBytes: u32,
    pub MftWritesUserLevel: NTFS_STATISTICS_4,
    pub MftWritesFlushForLogFileFull: u16,
    pub MftWritesLazyWriter: u16,
    pub MftWritesUserRequest: u16,
    pub Mft2Writes: u32,
    pub Mft2WriteBytes: u32,
    pub Mft2WritesUserLevel: NTFS_STATISTICS_2,
    pub Mft2WritesFlushForLogFileFull: u16,
    pub Mft2WritesLazyWriter: u16,
    pub Mft2WritesUserRequest: u16,
    pub RootIndexReads: u32,
    pub RootIndexReadBytes: u32,
    pub RootIndexWrites: u32,
    pub RootIndexWriteBytes: u32,
    pub BitmapReads: u32,
    pub BitmapReadBytes: u32,
    pub BitmapWrites: u32,
    pub BitmapWriteBytes: u32,
    pub BitmapWritesFlushForLogFileFull: u16,
    pub BitmapWritesLazyWriter: u16,
    pub BitmapWritesUserRequest: u16,
    pub BitmapWritesUserLevel: NTFS_STATISTICS_1,
    pub MftBitmapReads: u32,
    pub MftBitmapReadBytes: u32,
    pub MftBitmapWrites: u32,
    pub MftBitmapWriteBytes: u32,
    pub MftBitmapWritesFlushForLogFileFull: u16,
    pub MftBitmapWritesLazyWriter: u16,
    pub MftBitmapWritesUserRequest: u16,
    pub MftBitmapWritesUserLevel: NTFS_STATISTICS_3,
    pub UserIndexReads: u32,
    pub UserIndexReadBytes: u32,
    pub UserIndexWrites: u32,
    pub UserIndexWriteBytes: u32,
    pub LogFileReads: u32,
    pub LogFileReadBytes: u32,
    pub LogFileWrites: u32,
    pub LogFileWriteBytes: u32,
    pub Allocate: NTFS_STATISTICS_0,
    pub DiskResourcesExhausted: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS {}
impl ::core::clone::Clone for NTFS_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_0 {
    pub Calls: u32,
    pub Clusters: u32,
    pub Hints: u32,
    pub RunsReturned: u32,
    pub HintsHonored: u32,
    pub HintsClusters: u32,
    pub Cache: u32,
    pub CacheClusters: u32,
    pub CacheMiss: u32,
    pub CacheMissClusters: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_0 {}
impl ::core::clone::Clone for NTFS_STATISTICS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_1 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
}
impl ::core::marker::Copy for NTFS_STATISTICS_1 {}
impl ::core::clone::Clone for NTFS_STATISTICS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_2 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
impl ::core::marker::Copy for NTFS_STATISTICS_2 {}
impl ::core::clone::Clone for NTFS_STATISTICS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_3 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
impl ::core::marker::Copy for NTFS_STATISTICS_3 {}
impl ::core::clone::Clone for NTFS_STATISTICS_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_4 {
    pub Write: u16,
    pub Create: u16,
    pub SetInfo: u16,
    pub Flush: u16,
}
impl ::core::marker::Copy for NTFS_STATISTICS_4 {}
impl ::core::clone::Clone for NTFS_STATISTICS_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_EX {
    pub LogFileFullExceptions: u32,
    pub OtherExceptions: u32,
    pub MftReads: u64,
    pub MftReadBytes: u64,
    pub MftWrites: u64,
    pub MftWriteBytes: u64,
    pub MftWritesUserLevel: NTFS_STATISTICS_EX_4,
    pub MftWritesFlushForLogFileFull: u32,
    pub MftWritesLazyWriter: u32,
    pub MftWritesUserRequest: u32,
    pub Mft2Writes: u64,
    pub Mft2WriteBytes: u64,
    pub Mft2WritesUserLevel: NTFS_STATISTICS_EX_2,
    pub Mft2WritesFlushForLogFileFull: u32,
    pub Mft2WritesLazyWriter: u32,
    pub Mft2WritesUserRequest: u32,
    pub RootIndexReads: u64,
    pub RootIndexReadBytes: u64,
    pub RootIndexWrites: u64,
    pub RootIndexWriteBytes: u64,
    pub BitmapReads: u64,
    pub BitmapReadBytes: u64,
    pub BitmapWrites: u64,
    pub BitmapWriteBytes: u64,
    pub BitmapWritesFlushForLogFileFull: u32,
    pub BitmapWritesLazyWriter: u32,
    pub BitmapWritesUserRequest: u32,
    pub BitmapWritesUserLevel: NTFS_STATISTICS_EX_1,
    pub MftBitmapReads: u64,
    pub MftBitmapReadBytes: u64,
    pub MftBitmapWrites: u64,
    pub MftBitmapWriteBytes: u64,
    pub MftBitmapWritesFlushForLogFileFull: u32,
    pub MftBitmapWritesLazyWriter: u32,
    pub MftBitmapWritesUserRequest: u32,
    pub MftBitmapWritesUserLevel: NTFS_STATISTICS_EX_3,
    pub UserIndexReads: u64,
    pub UserIndexReadBytes: u64,
    pub UserIndexWrites: u64,
    pub UserIndexWriteBytes: u64,
    pub LogFileReads: u64,
    pub LogFileReadBytes: u64,
    pub LogFileWrites: u64,
    pub LogFileWriteBytes: u64,
    pub Allocate: NTFS_STATISTICS_EX_0,
    pub DiskResourcesExhausted: u32,
    pub VolumeTrimCount: u64,
    pub VolumeTrimTime: u64,
    pub VolumeTrimByteCount: u64,
    pub FileLevelTrimCount: u64,
    pub FileLevelTrimTime: u64,
    pub FileLevelTrimByteCount: u64,
    pub VolumeTrimSkippedCount: u64,
    pub VolumeTrimSkippedByteCount: u64,
    pub NtfsFillStatInfoFromMftRecordCalledCount: u64,
    pub NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount: u64,
    pub NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount: u64,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_EX_0 {
    pub Calls: u32,
    pub RunsReturned: u32,
    pub Hints: u32,
    pub HintsHonored: u32,
    pub Cache: u32,
    pub CacheMiss: u32,
    pub Clusters: u64,
    pub HintsClusters: u64,
    pub CacheClusters: u64,
    pub CacheMissClusters: u64,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_0 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_EX_1 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_1 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_EX_2 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_2 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_EX_3 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_3 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_STATISTICS_EX_4 {
    pub Write: u32,
    pub Create: u32,
    pub SetInfo: u32,
    pub Flush: u32,
}
impl ::core::marker::Copy for NTFS_STATISTICS_EX_4 {}
impl ::core::clone::Clone for NTFS_STATISTICS_EX_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct NTFS_VOLUME_DATA_BUFFER {
    pub VolumeSerialNumber: i64,
    pub NumberSectors: i64,
    pub TotalClusters: i64,
    pub FreeClusters: i64,
    pub TotalReserved: i64,
    pub BytesPerSector: u32,
    pub BytesPerCluster: u32,
    pub BytesPerFileRecordSegment: u32,
    pub ClustersPerFileRecordSegment: u32,
    pub MftValidDataLength: i64,
    pub MftStartLcn: i64,
    pub Mft2StartLcn: i64,
    pub MftZoneStart: i64,
    pub MftZoneEnd: i64,
}
impl ::core::marker::Copy for NTFS_VOLUME_DATA_BUFFER {}
impl ::core::clone::Clone for NTFS_VOLUME_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const OBSOLETE_DISK_GET_WRITE_CACHE_STATE: u32 = 475356u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const OBSOLETE_IOCTL_STORAGE_RESET_BUS: u32 = 3002368u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const OBSOLETE_IOCTL_STORAGE_RESET_DEVICE: u32 = 3002372u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const OFFLOAD_READ_FLAG_ALL_ZERO_BEYOND_CURRENT_RANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const OPLOCK_LEVEL_CACHE_HANDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const OPLOCK_LEVEL_CACHE_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const OPLOCK_LEVEL_CACHE_WRITE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTIITON_OS_DATA: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_BSP: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_DM: u32 = 84u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_DPP: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_ENTRY_UNUSED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_EXTENDED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_EZDRIVE: u32 = 85u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_FAT32: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_FAT32_XINT13: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_FAT_12: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_FAT_16: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_GPT: u32 = 238u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_HUGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_IFS: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PARTITION_INFORMATION {
    pub StartingOffset: i64,
    pub PartitionLength: i64,
    pub HiddenSectors: u32,
    pub PartitionNumber: u32,
    pub PartitionType: u8,
    pub BootIndicator: super::super::Foundation::BOOLEAN,
    pub RecognizedPartition: super::super::Foundation::BOOLEAN,
    pub RewritePartition: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PARTITION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PARTITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PARTITION_INFORMATION_EX {
    pub PartitionStyle: PARTITION_STYLE,
    pub StartingOffset: i64,
    pub PartitionLength: i64,
    pub PartitionNumber: u32,
    pub RewritePartition: super::super::Foundation::BOOLEAN,
    pub IsServicePartition: super::super::Foundation::BOOLEAN,
    pub Anonymous: PARTITION_INFORMATION_EX_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PARTITION_INFORMATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PARTITION_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PARTITION_INFORMATION_EX_0 {
    pub Mbr: PARTITION_INFORMATION_MBR,
    pub Gpt: PARTITION_INFORMATION_GPT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PARTITION_INFORMATION_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PARTITION_INFORMATION_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PARTITION_INFORMATION_GPT {
    pub PartitionType: ::windows_sys::core::GUID,
    pub PartitionId: ::windows_sys::core::GUID,
    pub Attributes: GPT_ATTRIBUTES,
    pub Name: [u16; 36],
}
impl ::core::marker::Copy for PARTITION_INFORMATION_GPT {}
impl ::core::clone::Clone for PARTITION_INFORMATION_GPT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PARTITION_INFORMATION_MBR {
    pub PartitionType: u8,
    pub BootIndicator: super::super::Foundation::BOOLEAN,
    pub RecognizedPartition: super::super::Foundation::BOOLEAN,
    pub HiddenSectors: u32,
    pub PartitionId: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PARTITION_INFORMATION_MBR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PARTITION_INFORMATION_MBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_LDM: u32 = 66u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_MAIN_OS: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_MSFT_RECOVERY: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_NTFT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_OS2BOOTMGR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_PREP: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_PRE_INSTALLED: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_SPACES: u32 = 231u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_SPACES_DATA: u32 = 215u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type PARTITION_STYLE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_STYLE_MBR: PARTITION_STYLE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_STYLE_GPT: PARTITION_STYLE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_STYLE_RAW: PARTITION_STYLE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_SYSTEM: u32 = 239u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_UNIX: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_WINDOWS_SYSTEM: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_XENIX_1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_XENIX_2: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_XINT13: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_XINT13_EXTENDED: u32 = 15u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PATHNAME_BUFFER {
    pub PathNameLength: u32,
    pub Name: [u16; 1],
}
impl ::core::marker::Copy for PATHNAME_BUFFER {}
impl ::core::clone::Clone for PATHNAME_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PERF_BIN {
    pub NumberOfBins: u32,
    pub TypeOfBin: u32,
    pub BinsRanges: [BIN_RANGE; 1],
}
impl ::core::marker::Copy for PERF_BIN {}
impl ::core::clone::Clone for PERF_BIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PERSISTENT_RESERVE_COMMAND {
    pub Version: u32,
    pub Size: u32,
    pub Anonymous: PERSISTENT_RESERVE_COMMAND_0,
}
impl ::core::marker::Copy for PERSISTENT_RESERVE_COMMAND {}
impl ::core::clone::Clone for PERSISTENT_RESERVE_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union PERSISTENT_RESERVE_COMMAND_0 {
    pub PR_IN: PERSISTENT_RESERVE_COMMAND_0_0,
    pub PR_OUT: PERSISTENT_RESERVE_COMMAND_0_1,
}
impl ::core::marker::Copy for PERSISTENT_RESERVE_COMMAND_0 {}
impl ::core::clone::Clone for PERSISTENT_RESERVE_COMMAND_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PERSISTENT_RESERVE_COMMAND_0_0 {
    pub _bitfield: u8,
    pub AllocationLength: u16,
}
impl ::core::marker::Copy for PERSISTENT_RESERVE_COMMAND_0_0 {}
impl ::core::clone::Clone for PERSISTENT_RESERVE_COMMAND_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PERSISTENT_RESERVE_COMMAND_0_1 {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub ParameterList: [u8; 1],
}
impl ::core::marker::Copy for PERSISTENT_RESERVE_COMMAND_0_1 {}
impl ::core::clone::Clone for PERSISTENT_RESERVE_COMMAND_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_BACKED_BY_WIM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_CHKDSK_RAN_ONCE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_CONTAINS_BACKING_WIM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_DAX_FORMATTED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_GLOBAL_METADATA_NO_SEEK_PENALTY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_LOCAL_METADATA_NO_SEEK_PENALTY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_MODIFIED_BY_CHKDSK: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_NO_HEAT_GATHERING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_NO_WRITE_AUTO_TIERING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_REALLOCATE_ALL_DATA_WRITES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_SHORT_NAME_CREATION_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_TXF_DISABLED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PERSISTENT_VOLUME_STATE_VOLUME_SCRUB_DISABLED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PHYSICAL_ELEMENT_STATUS {
    pub Version: u32,
    pub Size: u32,
    pub DescriptorCount: u32,
    pub ReturnedDescriptorCount: u32,
    pub ElementIdentifierBeingDepoped: u32,
    pub Reserved: u32,
    pub Descriptors: [PHYSICAL_ELEMENT_STATUS_DESCRIPTOR; 1],
}
impl ::core::marker::Copy for PHYSICAL_ELEMENT_STATUS {}
impl ::core::clone::Clone for PHYSICAL_ELEMENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub ElementIdentifier: u32,
    pub PhysicalElementType: u8,
    pub PhysicalElementHealth: u8,
    pub Reserved1: [u8; 2],
    pub AssociatedCapacity: u64,
    pub Reserved2: [u32; 4],
}
impl ::core::marker::Copy for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {}
impl ::core::clone::Clone for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PHYSICAL_ELEMENT_STATUS_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub StartingElement: u32,
    pub Filter: u8,
    pub ReportType: u8,
    pub Reserved: [u8; 2],
}
impl ::core::marker::Copy for PHYSICAL_ELEMENT_STATUS_REQUEST {}
impl ::core::clone::Clone for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK = ::core::option::Option<unsafe extern "system" fn(sourcecontext: *const IO_IRP_EXT_TRACK_OFFSET_HEADER, targetcontext: *mut IO_IRP_EXT_TRACK_OFFSET_HEADER, relativeoffset: i64)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PLEX_READ_DATA_REQUEST {
    pub ByteOffset: i64,
    pub ByteLength: u32,
    pub PlexNumber: u32,
}
impl ::core::marker::Copy for PLEX_READ_DATA_REQUEST {}
impl ::core::clone::Clone for PLEX_READ_DATA_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PREVENT_MEDIA_REMOVAL {
    pub PreventMediaRemoval: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PREVENT_MEDIA_REMOVAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PREVENT_MEDIA_REMOVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PRODUCT_ID_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PROJFS_PROTOCOL_VERSION: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct QUERY_BAD_RANGES_INPUT {
    pub Flags: u32,
    pub NumRanges: u32,
    pub Ranges: [QUERY_BAD_RANGES_INPUT_RANGE; 1],
}
impl ::core::marker::Copy for QUERY_BAD_RANGES_INPUT {}
impl ::core::clone::Clone for QUERY_BAD_RANGES_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct QUERY_BAD_RANGES_INPUT_RANGE {
    pub StartOffset: u64,
    pub LengthInBytes: u64,
}
impl ::core::marker::Copy for QUERY_BAD_RANGES_INPUT_RANGE {}
impl ::core::clone::Clone for QUERY_BAD_RANGES_INPUT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct QUERY_BAD_RANGES_OUTPUT {
    pub Flags: u32,
    pub NumBadRanges: u32,
    pub NextOffsetToLookUp: u64,
    pub BadRanges: [QUERY_BAD_RANGES_OUTPUT_RANGE; 1],
}
impl ::core::marker::Copy for QUERY_BAD_RANGES_OUTPUT {}
impl ::core::clone::Clone for QUERY_BAD_RANGES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct QUERY_BAD_RANGES_OUTPUT_RANGE {
    pub Flags: u32,
    pub Reserved: u32,
    pub StartOffset: u64,
    pub LengthInBytes: u64,
}
impl ::core::marker::Copy for QUERY_BAD_RANGES_OUTPUT_RANGE {}
impl ::core::clone::Clone for QUERY_BAD_RANGES_OUTPUT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_GUEST_VOLUMES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_HOST_VOLUMES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type QUERY_FILE_LAYOUT_FILTER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_NONE: QUERY_FILE_LAYOUT_FILTER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_CLUSTERS: QUERY_FILE_LAYOUT_FILTER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_FILEID: QUERY_FILE_LAYOUT_FILTER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_STORAGE_RESERVE_ID: QUERY_FILE_LAYOUT_FILTER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_NUM_FILTER_TYPES: QUERY_FILE_LAYOUT_FILTER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTENTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_EXTRA_INFO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_FILES_WITH_DSC_ATTRIBUTE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_FULL_PATH_IN_NAMES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_NAMES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_ONLY_FILES_WITH_SPECIFIC_ATTRIBUTES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAMS_WITH_NO_CLUSTERS_ALLOCATED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DATA_ATTRIBUTE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DSC_ATTRIBUTE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EA_ATTRIBUTE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EFS_ATTRIBUTE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_REPARSE_ATTRIBUTE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_TXF_ATTRIBUTE: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct QUERY_FILE_LAYOUT_INPUT {
    pub Anonymous: QUERY_FILE_LAYOUT_INPUT_0,
    pub Flags: u32,
    pub FilterType: QUERY_FILE_LAYOUT_FILTER_TYPE,
    pub Reserved: u32,
    pub Filter: QUERY_FILE_LAYOUT_INPUT_1,
}
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_INPUT {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union QUERY_FILE_LAYOUT_INPUT_0 {
    pub FilterEntryCount: u32,
    pub NumberOfPairs: u32,
}
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_INPUT_0 {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union QUERY_FILE_LAYOUT_INPUT_1 {
    pub ClusterRanges: [CLUSTER_RANGE; 1],
    pub FileReferenceRanges: [FILE_REFERENCE_RANGE; 1],
    pub StorageReserveIds: [STORAGE_RESERVE_ID; 1],
}
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_INPUT_1 {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_INPUT_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct QUERY_FILE_LAYOUT_OUTPUT {
    pub FileEntryCount: u32,
    pub FirstFileOffset: u32,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_OUTPUT {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_REPARSE_DATA_INVALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_REPARSE_TAG_INVALID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_RESTART: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_SINGLE_INSTANCED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_READ: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_MEASURE_WRITE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_STORAGE_CLASSES_FLAGS_NO_DEFRAG_VOLUME: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const READ_ATTRIBUTES: u32 = 208u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const READ_ATTRIBUTE_BUFFER_SIZE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const READ_COMPRESSION_INFO_VALID: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const READ_COPY_NUMBER_BYPASS_CACHE_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const READ_COPY_NUMBER_KEY: u32 = 1380142592u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct READ_ELEMENT_ADDRESS_INFO {
    pub NumberOfElements: u32,
    pub ElementStatus: [CHANGER_ELEMENT_STATUS; 1],
}
impl ::core::marker::Copy for READ_ELEMENT_ADDRESS_INFO {}
impl ::core::clone::Clone for READ_ELEMENT_ADDRESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct READ_FILE_USN_DATA {
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
impl ::core::marker::Copy for READ_FILE_USN_DATA {}
impl ::core::clone::Clone for READ_FILE_USN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const READ_THRESHOLDS: u32 = 209u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const READ_THRESHOLD_BUFFER_SIZE: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct READ_USN_JOURNAL_DATA_V0 {
    pub StartUsn: i64,
    pub ReasonMask: u32,
    pub ReturnOnlyOnClose: u32,
    pub Timeout: u64,
    pub BytesToWaitFor: u64,
    pub UsnJournalID: u64,
}
impl ::core::marker::Copy for READ_USN_JOURNAL_DATA_V0 {}
impl ::core::clone::Clone for READ_USN_JOURNAL_DATA_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct READ_USN_JOURNAL_DATA_V1 {
    pub StartUsn: i64,
    pub ReasonMask: u32,
    pub ReturnOnlyOnClose: u32,
    pub Timeout: u64,
    pub BytesToWaitFor: u64,
    pub UsnJournalID: u64,
    pub MinMajorVersion: u16,
    pub MaxMajorVersion: u16,
}
impl ::core::marker::Copy for READ_USN_JOURNAL_DATA_V1 {}
impl ::core::clone::Clone for READ_USN_JOURNAL_DATA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REASSIGN_BLOCKS {
    pub Reserved: u16,
    pub Count: u16,
    pub BlockNumber: [u32; 1],
}
impl ::core::marker::Copy for REASSIGN_BLOCKS {}
impl ::core::clone::Clone for REASSIGN_BLOCKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REASSIGN_BLOCKS_EX {
    pub Reserved: u16,
    pub Count: u16,
    pub BlockNumber: [i64; 1],
}
impl ::core::marker::Copy for REASSIGN_BLOCKS_EX {}
impl ::core::clone::Clone for REASSIGN_BLOCKS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RECOVERED_READS_VALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RECOVERED_WRITES_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type REFS_SMR_VOLUME_GC_ACTION = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcActionStart: REFS_SMR_VOLUME_GC_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcActionStartFullSpeed: REFS_SMR_VOLUME_GC_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcActionPause: REFS_SMR_VOLUME_GC_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcActionStop: REFS_SMR_VOLUME_GC_ACTION = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type REFS_SMR_VOLUME_GC_METHOD = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcMethodCompaction: REFS_SMR_VOLUME_GC_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcMethodCompression: REFS_SMR_VOLUME_GC_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcMethodRotation: REFS_SMR_VOLUME_GC_METHOD = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REFS_SMR_VOLUME_GC_PARAMETERS {
    pub Version: u32,
    pub Flags: u32,
    pub Action: REFS_SMR_VOLUME_GC_ACTION,
    pub Method: REFS_SMR_VOLUME_GC_METHOD,
    pub IoGranularity: u32,
    pub CompressionFormat: u32,
    pub Unused: [u64; 8],
}
impl ::core::marker::Copy for REFS_SMR_VOLUME_GC_PARAMETERS {}
impl ::core::clone::Clone for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REFS_SMR_VOLUME_GC_PARAMETERS_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type REFS_SMR_VOLUME_GC_STATE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcStateInactive: REFS_SMR_VOLUME_GC_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcStatePaused: REFS_SMR_VOLUME_GC_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcStateActive: REFS_SMR_VOLUME_GC_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcStateActiveFullSpeed: REFS_SMR_VOLUME_GC_STATE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REFS_SMR_VOLUME_INFO_OUTPUT {
    pub Version: u32,
    pub Flags: u32,
    pub SizeOfRandomlyWritableTier: i64,
    pub FreeSpaceInRandomlyWritableTier: i64,
    pub SizeofSMRTier: i64,
    pub FreeSpaceInSMRTier: i64,
    pub UsableFreeSpaceInSMRTier: i64,
    pub VolumeGcState: REFS_SMR_VOLUME_GC_STATE,
    pub VolumeGcLastStatus: u32,
    pub CurrentGcBandFillPercentage: u32,
    pub Unused: [u64; 6],
}
impl ::core::marker::Copy for REFS_SMR_VOLUME_INFO_OUTPUT {}
impl ::core::clone::Clone for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REFS_VOLUME_DATA_BUFFER {
    pub ByteCount: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BytesPerPhysicalSector: u32,
    pub VolumeSerialNumber: i64,
    pub NumberSectors: i64,
    pub TotalClusters: i64,
    pub FreeClusters: i64,
    pub TotalReserved: i64,
    pub BytesPerSector: u32,
    pub BytesPerCluster: u32,
    pub MaximumSizeOfResidentFile: i64,
    pub FastTierDataFillRatio: u16,
    pub SlowTierDataFillRatio: u16,
    pub DestagesFastTierToSlowTierRate: u32,
    pub Reserved: [i64; 9],
}
impl ::core::marker::Copy for REFS_VOLUME_DATA_BUFFER {}
impl ::core::clone::Clone for REFS_VOLUME_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub RequestCapacity: u64,
    pub ElementIdentifier: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {}
impl ::core::clone::Clone for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REPAIR_COPIES_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub FileOffset: i64,
    pub Length: u32,
    pub SourceCopy: u32,
    pub NumberOfRepairCopies: u32,
    pub RepairCopies: [u32; 1],
}
impl ::core::marker::Copy for REPAIR_COPIES_INPUT {}
impl ::core::clone::Clone for REPAIR_COPIES_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REPAIR_COPIES_OUTPUT {
    pub Size: u32,
    pub Status: u32,
    pub ResumeFileOffset: i64,
}
impl ::core::marker::Copy for REPAIR_COPIES_OUTPUT {}
impl ::core::clone::Clone for REPAIR_COPIES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REPLACE_ALTERNATE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REPLACE_PRIMARY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REQUEST_OPLOCK_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REQUEST_OPLOCK_INPUT_BUFFER {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub RequestedOplockLevel: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for REQUEST_OPLOCK_INPUT_BUFFER {}
impl ::core::clone::Clone for REQUEST_OPLOCK_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_ACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_COMPLETE_ACK_ON_CLOSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REQUEST_OPLOCK_INPUT_FLAG_REQUEST: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REQUEST_OPLOCK_OUTPUT_BUFFER {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub OriginalOplockLevel: u32,
    pub NewOplockLevel: u32,
    pub Flags: u32,
    pub AccessMode: u32,
    pub ShareMode: u16,
}
impl ::core::marker::Copy for REQUEST_OPLOCK_OUTPUT_BUFFER {}
impl ::core::clone::Clone for REQUEST_OPLOCK_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REQUEST_OPLOCK_OUTPUT_FLAG_ACK_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REQUEST_OPLOCK_OUTPUT_FLAG_MODES_PROVIDED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct REQUEST_RAW_ENCRYPTED_DATA {
    pub FileOffset: i64,
    pub Length: u32,
}
impl ::core::marker::Copy for REQUEST_RAW_ENCRYPTED_DATA {}
impl ::core::clone::Clone for REQUEST_RAW_ENCRYPTED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RETRACT_IEPORT: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    pub ExtentCount: u32,
    pub StartingVcn: i64,
    pub Extents: [RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0; 1],
}
impl ::core::marker::Copy for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {}
impl ::core::clone::Clone for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    pub NextVcn: i64,
    pub Lcn: i64,
    pub ReferenceCount: u32,
}
impl ::core::marker::Copy for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {}
impl ::core::clone::Clone for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct RETRIEVAL_POINTERS_BUFFER {
    pub ExtentCount: u32,
    pub StartingVcn: i64,
    pub Extents: [RETRIEVAL_POINTERS_BUFFER_0; 1],
}
impl ::core::marker::Copy for RETRIEVAL_POINTERS_BUFFER {}
impl ::core::clone::Clone for RETRIEVAL_POINTERS_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct RETRIEVAL_POINTERS_BUFFER_0 {
    pub NextVcn: i64,
    pub Lcn: i64,
}
impl ::core::marker::Copy for RETRIEVAL_POINTERS_BUFFER_0 {}
impl ::core::clone::Clone for RETRIEVAL_POINTERS_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct RETRIEVAL_POINTER_BASE {
    pub FileAreaOffset: i64,
}
impl ::core::marker::Copy for RETRIEVAL_POINTER_BASE {}
impl ::core::clone::Clone for RETRIEVAL_POINTER_BASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct RETRIEVAL_POINTER_COUNT {
    pub ExtentCount: u32,
}
impl ::core::marker::Copy for RETRIEVAL_POINTER_COUNT {}
impl ::core::clone::Clone for RETRIEVAL_POINTER_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RETURN_SMART_STATUS: u32 = 218u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REVISION_LENGTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SAVE_ATTRIBUTE_VALUES: u32 = 211u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    pub Version: u32,
    pub Size: u32,
    pub DeviceCount: u32,
    pub Devices: [SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO; 1],
}
impl ::core::marker::Copy for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {}
impl ::core::clone::Clone for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    pub DeviceGuid: ::windows_sys::core::GUID,
    pub DeviceNumber: u32,
    pub Flags: SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0,
    pub DeviceSize: u64,
}
impl ::core::marker::Copy for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {}
impl ::core::clone::Clone for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {}
impl ::core::clone::Clone for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_BUS_DEDICATED_MEMORY_STATE {
    pub ActivateState: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCM_BUS_DEDICATED_MEMORY_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCM_BUS_DEDICATED_MEMORY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_BUS_FIRMWARE_ACTIVATION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusFirmwareActivationState_Idle: SCM_BUS_FIRMWARE_ACTIVATION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusFirmwareActivationState_Armed: SCM_BUS_FIRMWARE_ACTIVATION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusFirmwareActivationState_Busy: SCM_BUS_FIRMWARE_ACTIVATION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_BUS_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusProperty_RuntimeFwActivationInfo: SCM_BUS_PROPERTY_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusProperty_DedicatedMemoryInfo: SCM_BUS_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusProperty_DedicatedMemoryState: SCM_BUS_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusProperty_Max: SCM_BUS_PROPERTY_ID = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_BUS_PROPERTY_QUERY {
    pub Version: u32,
    pub Size: u32,
    pub PropertyId: SCM_BUS_PROPERTY_ID,
    pub QueryType: SCM_BUS_QUERY_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl ::core::marker::Copy for SCM_BUS_PROPERTY_QUERY {}
impl ::core::clone::Clone for SCM_BUS_PROPERTY_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_BUS_PROPERTY_SET {
    pub Version: u32,
    pub Size: u32,
    pub PropertyId: SCM_BUS_PROPERTY_ID,
    pub SetType: SCM_BUS_SET_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl ::core::marker::Copy for SCM_BUS_PROPERTY_SET {}
impl ::core::clone::Clone for SCM_BUS_PROPERTY_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_BUS_QUERY_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusQuery_Descriptor: SCM_BUS_QUERY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusQuery_IsSupported: SCM_BUS_QUERY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusQuery_Max: SCM_BUS_QUERY_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    pub Version: u32,
    pub Size: u32,
    pub RuntimeFwActivationSupported: super::super::Foundation::BOOLEAN,
    pub FirmwareActivationState: SCM_BUS_FIRMWARE_ACTIVATION_STATE,
    pub FirmwareActivationCapability: SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0,
    pub EstimatedFirmwareActivationTimeInUSecs: u64,
    pub EstimatedProcessorAccessQuiesceTimeInUSecs: u64,
    pub EstimatedIOAccessQuiesceTimeInUSecs: u64,
    pub PlatformSupportedMaxIOAccessQuiesceTimeInUSecs: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_BUS_SET_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusSet_Descriptor: SCM_BUS_SET_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusSet_IsSupported: SCM_BUS_SET_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusSet_Max: SCM_BUS_SET_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_INTERLEAVED_PD_INFO {
    pub DeviceHandle: u32,
    pub DeviceGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SCM_INTERLEAVED_PD_INFO {}
impl ::core::clone::Clone for SCM_INTERLEAVED_PD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_LD_INTERLEAVE_SET_INFO {
    pub Version: u32,
    pub Size: u32,
    pub InterleaveSetSize: u32,
    pub InterleaveSet: [SCM_INTERLEAVED_PD_INFO; 1],
}
impl ::core::marker::Copy for SCM_LD_INTERLEAVE_SET_INFO {}
impl ::core::clone::Clone for SCM_LD_INTERLEAVE_SET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_LOGICAL_DEVICES {
    pub Version: u32,
    pub Size: u32,
    pub DeviceCount: u32,
    pub Devices: [SCM_LOGICAL_DEVICE_INSTANCE; 1],
}
impl ::core::marker::Copy for SCM_LOGICAL_DEVICES {}
impl ::core::clone::Clone for SCM_LOGICAL_DEVICES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_LOGICAL_DEVICE_INSTANCE {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: ::windows_sys::core::GUID,
    pub SymbolicLink: [u16; 256],
}
impl ::core::marker::Copy for SCM_LOGICAL_DEVICE_INSTANCE {}
impl ::core::clone::Clone for SCM_LOGICAL_DEVICE_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SCM_MAX_SYMLINK_LEN_IN_CHARS: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_DESCRIPTOR_HEADER {
    pub Version: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for SCM_PD_DESCRIPTOR_HEADER {}
impl ::core::clone::Clone for SCM_PD_DESCRIPTOR_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_DEVICE_HANDLE {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: ::windows_sys::core::GUID,
    pub DeviceHandle: u32,
}
impl ::core::marker::Copy for SCM_PD_DEVICE_HANDLE {}
impl ::core::clone::Clone for SCM_PD_DEVICE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_PD_DEVICE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: ::windows_sys::core::GUID,
    pub UnsafeShutdownCount: u32,
    pub PersistentMemorySizeInBytes: u64,
    pub VolatileMemorySizeInBytes: u64,
    pub TotalMemorySizeInBytes: u64,
    pub SlotNumber: u32,
    pub DeviceHandle: u32,
    pub PhysicalId: u16,
    pub NumberOfFormatInterfaceCodes: u8,
    pub FormatInterfaceCodes: [u16; 8],
    pub VendorId: u32,
    pub ProductId: u32,
    pub SubsystemDeviceId: u32,
    pub SubsystemVendorId: u32,
    pub ManufacturingLocation: u8,
    pub ManufacturingWeek: u8,
    pub ManufacturingYear: u8,
    pub SerialNumber4Byte: u32,
    pub SerialNumberLengthInChars: u32,
    pub SerialNumber: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCM_PD_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCM_PD_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_DEVICE_SPECIFIC_INFO {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfProperties: u32,
    pub DeviceSpecificProperties: [SCM_PD_DEVICE_SPECIFIC_PROPERTY; 1],
}
impl ::core::marker::Copy for SCM_PD_DEVICE_SPECIFIC_INFO {}
impl ::core::clone::Clone for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    pub Name: [u16; 128],
    pub Value: i64,
}
impl ::core::marker::Copy for SCM_PD_DEVICE_SPECIFIC_PROPERTY {}
impl ::core::clone::Clone for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
}
impl ::core::marker::Copy for SCM_PD_FIRMWARE_ACTIVATE {}
impl ::core::clone::Clone for SCM_PD_FIRMWARE_ACTIVATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_FIRMWARE_ACTIVATION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdFirmwareActivationState_Idle: SCM_PD_FIRMWARE_ACTIVATION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdFirmwareActivationState_Armed: SCM_PD_FIRMWARE_ACTIVATION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdFirmwareActivationState_Busy: SCM_PD_FIRMWARE_ACTIVATION_STATE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub Offset: u64,
    pub FirmwareImageSizeInBytes: u32,
    pub FirmwareImage: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_FIRMWARE_DOWNLOAD {}
impl ::core::clone::Clone for SCM_PD_FIRMWARE_DOWNLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub ActiveSlot: u8,
    pub NextActiveSlot: u8,
    pub SlotCount: u8,
    pub Slots: [SCM_PD_FIRMWARE_SLOT_INFO; 1],
}
impl ::core::marker::Copy for SCM_PD_FIRMWARE_INFO {}
impl ::core::clone::Clone for SCM_PD_FIRMWARE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SCM_PD_FIRMWARE_LAST_DOWNLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SCM_PD_FIRMWARE_REVISION_LENGTH_BYTES: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_FIRMWARE_SLOT_INFO {
    pub Version: u32,
    pub Size: u32,
    pub SlotNumber: u8,
    pub _bitfield: u8,
    pub Reserved1: [u8; 6],
    pub Revision: [u8; 32],
}
impl ::core::marker::Copy for SCM_PD_FIRMWARE_SLOT_INFO {}
impl ::core::clone::Clone for SCM_PD_FIRMWARE_SLOT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_FRU_ID_STRING {
    pub Version: u32,
    pub Size: u32,
    pub IdentifierSize: u32,
    pub Identifier: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_FRU_ID_STRING {}
impl ::core::clone::Clone for SCM_PD_FRU_ID_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_HEALTH_NOTIFICATION_DATA {
    pub DeviceGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SCM_PD_HEALTH_NOTIFICATION_DATA {}
impl ::core::clone::Clone for SCM_PD_HEALTH_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_HEALTH_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Unknown: SCM_PD_HEALTH_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Unhealthy: SCM_PD_HEALTH_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Warning: SCM_PD_HEALTH_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Healthy: SCM_PD_HEALTH_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Max: SCM_PD_HEALTH_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_LAST_FW_ACTIVATION_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivationStatus_None: SCM_PD_LAST_FW_ACTIVATION_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivationStatus_Success: SCM_PD_LAST_FW_ACTIVATION_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivationStatus_FwNotFound: SCM_PD_LAST_FW_ACTIVATION_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivationStatus_ColdRebootRequired: SCM_PD_LAST_FW_ACTIVATION_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivaitonStatus_ActivationInProgress: SCM_PD_LAST_FW_ACTIVATION_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivaitonStatus_Retry: SCM_PD_LAST_FW_ACTIVATION_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivaitonStatus_FwUnsupported: SCM_PD_LAST_FW_ACTIVATION_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivaitonStatus_UnknownError: SCM_PD_LAST_FW_ACTIVATION_STATUS = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_LOCATION_STRING {
    pub Version: u32,
    pub Size: u32,
    pub Location: [u16; 1],
}
impl ::core::marker::Copy for SCM_PD_LOCATION_STRING {}
impl ::core::clone::Clone for SCM_PD_LOCATION_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_MANAGEMENT_STATUS {
    pub Version: u32,
    pub Size: u32,
    pub Health: SCM_PD_HEALTH_STATUS,
    pub NumberOfOperationalStatus: u32,
    pub NumberOfAdditionalReasons: u32,
    pub OperationalStatus: [SCM_PD_OPERATIONAL_STATUS; 16],
    pub AdditionalReasons: [SCM_PD_OPERATIONAL_STATUS_REASON; 1],
}
impl ::core::marker::Copy for SCM_PD_MANAGEMENT_STATUS {}
impl ::core::clone::Clone for SCM_PD_MANAGEMENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SCM_PD_MAX_OPERATIONAL_STATUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_MEDIA_REINITIALIZATION_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceReinit_Success: SCM_PD_MEDIA_REINITIALIZATION_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceReinit_RebootNeeded: SCM_PD_MEDIA_REINITIALIZATION_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceReinit_ColdBootNeeded: SCM_PD_MEDIA_REINITIALIZATION_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceReinit_Max: SCM_PD_MEDIA_REINITIALIZATION_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_OPERATIONAL_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_Unknown: SCM_PD_OPERATIONAL_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_Ok: SCM_PD_OPERATIONAL_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_PredictingFailure: SCM_PD_OPERATIONAL_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_InService: SCM_PD_OPERATIONAL_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_HardwareError: SCM_PD_OPERATIONAL_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_NotUsable: SCM_PD_OPERATIONAL_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_TransientError: SCM_PD_OPERATIONAL_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_Missing: SCM_PD_OPERATIONAL_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_Max: SCM_PD_OPERATIONAL_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_OPERATIONAL_STATUS_REASON = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Unknown: SCM_PD_OPERATIONAL_STATUS_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Media: SCM_PD_OPERATIONAL_STATUS_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_ThresholdExceeded: SCM_PD_OPERATIONAL_STATUS_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_LostData: SCM_PD_OPERATIONAL_STATUS_REASON = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_EnergySource: SCM_PD_OPERATIONAL_STATUS_REASON = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Configuration: SCM_PD_OPERATIONAL_STATUS_REASON = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_DeviceController: SCM_PD_OPERATIONAL_STATUS_REASON = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_MediaController: SCM_PD_OPERATIONAL_STATUS_REASON = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Component: SCM_PD_OPERATIONAL_STATUS_REASON = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_BackgroundOperation: SCM_PD_OPERATIONAL_STATUS_REASON = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_InvalidFirmware: SCM_PD_OPERATIONAL_STATUS_REASON = 10i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_HealthCheck: SCM_PD_OPERATIONAL_STATUS_REASON = 11i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_LostDataPersistence: SCM_PD_OPERATIONAL_STATUS_REASON = 12i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_DisabledByPlatform: SCM_PD_OPERATIONAL_STATUS_REASON = 13i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_PermanentError: SCM_PD_OPERATIONAL_STATUS_REASON = 14i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_LostWritePersistence: SCM_PD_OPERATIONAL_STATUS_REASON = 15i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_FatalError: SCM_PD_OPERATIONAL_STATUS_REASON = 16i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_DataPersistenceLossImminent: SCM_PD_OPERATIONAL_STATUS_REASON = 17i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_WritePersistenceLossImminent: SCM_PD_OPERATIONAL_STATUS_REASON = 18i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_MediaRemainingSpareBlock: SCM_PD_OPERATIONAL_STATUS_REASON = 19i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_PerformanceDegradation: SCM_PD_OPERATIONAL_STATUS_REASON = 20i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_ExcessiveTemperature: SCM_PD_OPERATIONAL_STATUS_REASON = 21i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_InternalFailure: SCM_PD_OPERATIONAL_STATUS_REASON = 22i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Max: SCM_PD_OPERATIONAL_STATUS_REASON = 23i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_PASSTHROUGH_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolGuid: ::windows_sys::core::GUID,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_PASSTHROUGH_INPUT {}
impl ::core::clone::Clone for SCM_PD_PASSTHROUGH_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    pub Opcode: u32,
    pub OpcodeParametersLength: u32,
    pub OpcodeParameters: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {}
impl ::core::clone::Clone for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    pub GeneralStatus: u16,
    pub ExtendedStatus: u16,
    pub OutputDataLength: u32,
    pub OutputData: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {}
impl ::core::clone::Clone for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_PASSTHROUGH_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolGuid: ::windows_sys::core::GUID,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_PASSTHROUGH_OUTPUT {}
impl ::core::clone::Clone for SCM_PD_PASSTHROUGH_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_DeviceInfo: SCM_PD_PROPERTY_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_ManagementStatus: SCM_PD_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_FirmwareInfo: SCM_PD_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_LocationString: SCM_PD_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_DeviceSpecificInfo: SCM_PD_PROPERTY_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_DeviceHandle: SCM_PD_PROPERTY_ID = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_FruIdString: SCM_PD_PROPERTY_ID = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_RuntimeFwActivationInfo: SCM_PD_PROPERTY_ID = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_RuntimeFwActivationArmState: SCM_PD_PROPERTY_ID = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_Max: SCM_PD_PROPERTY_ID = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SCM_PD_PROPERTY_NAME_LENGTH_IN_CHARS: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_PROPERTY_QUERY {
    pub Version: u32,
    pub Size: u32,
    pub PropertyId: SCM_PD_PROPERTY_ID,
    pub QueryType: SCM_PD_QUERY_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_PROPERTY_QUERY {}
impl ::core::clone::Clone for SCM_PD_PROPERTY_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_PROPERTY_SET {
    pub Version: u32,
    pub Size: u32,
    pub PropertyId: SCM_PD_PROPERTY_ID,
    pub SetType: SCM_PD_SET_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_PROPERTY_SET {}
impl ::core::clone::Clone for SCM_PD_PROPERTY_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_QUERY_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceQuery_Descriptor: SCM_PD_QUERY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceQuery_IsSupported: SCM_PD_QUERY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceQuery_Max: SCM_PD_QUERY_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_REINITIALIZE_MEDIA_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub Options: SCM_PD_REINITIALIZE_MEDIA_INPUT_0,
}
impl ::core::marker::Copy for SCM_PD_REINITIALIZE_MEDIA_INPUT {}
impl ::core::clone::Clone for SCM_PD_REINITIALIZE_MEDIA_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {}
impl ::core::clone::Clone for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub Status: SCM_PD_MEDIA_REINITIALIZATION_STATUS,
}
impl ::core::marker::Copy for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {}
impl ::core::clone::Clone for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    pub ArmState: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    pub Version: u32,
    pub Size: u32,
    pub LastFirmwareActivationStatus: SCM_PD_LAST_FW_ACTIVATION_STATUS,
    pub FirmwareActivationState: SCM_PD_FIRMWARE_ACTIVATION_STATE,
}
impl ::core::marker::Copy for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {}
impl ::core::clone::Clone for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_PD_SET_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceSet_Descriptor: SCM_PD_SET_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceSet_IsSupported: SCM_PD_SET_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceSet_Max: SCM_PD_SET_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PHYSICAL_DEVICES {
    pub Version: u32,
    pub Size: u32,
    pub DeviceCount: u32,
    pub Devices: [SCM_PHYSICAL_DEVICE_INSTANCE; 1],
}
impl ::core::marker::Copy for SCM_PHYSICAL_DEVICES {}
impl ::core::clone::Clone for SCM_PHYSICAL_DEVICES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PHYSICAL_DEVICE_INSTANCE {
    pub Version: u32,
    pub Size: u32,
    pub NfitHandle: u32,
    pub SymbolicLink: [u16; 256],
}
impl ::core::marker::Copy for SCM_PHYSICAL_DEVICE_INSTANCE {}
impl ::core::clone::Clone for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_REGION {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NfitHandle: u32,
    pub LogicalDeviceGuid: ::windows_sys::core::GUID,
    pub AddressRangeType: ::windows_sys::core::GUID,
    pub AssociatedId: u32,
    pub Length: u64,
    pub StartingDPA: u64,
    pub BaseSPA: u64,
    pub SPAOffset: u64,
    pub RegionOffset: u64,
}
impl ::core::marker::Copy for SCM_REGION {}
impl ::core::clone::Clone for SCM_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_REGIONS {
    pub Version: u32,
    pub Size: u32,
    pub RegionCount: u32,
    pub Regions: [SCM_REGION; 1],
}
impl ::core::marker::Copy for SCM_REGIONS {}
impl ::core::clone::Clone for SCM_REGIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SCM_REGION_FLAG = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmRegionFlagNone: SCM_REGION_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmRegionFlagLabel: SCM_REGION_FLAG = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_CHANGE_MACHINE_SID_INPUT {
    pub CurrentMachineSIDOffset: u16,
    pub CurrentMachineSIDLength: u16,
    pub NewMachineSIDOffset: u16,
    pub NewMachineSIDLength: u16,
}
impl ::core::marker::Copy for SD_CHANGE_MACHINE_SID_INPUT {}
impl ::core::clone::Clone for SD_CHANGE_MACHINE_SID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_CHANGE_MACHINE_SID_OUTPUT {
    pub NumSDChangedSuccess: u64,
    pub NumSDChangedFail: u64,
    pub NumSDUnused: u64,
    pub NumSDTotal: u64,
    pub NumMftSDChangedSuccess: u64,
    pub NumMftSDChangedFail: u64,
    pub NumMftSDTotal: u64,
}
impl ::core::marker::Copy for SD_CHANGE_MACHINE_SID_OUTPUT {}
impl ::core::clone::Clone for SD_CHANGE_MACHINE_SID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_ENUM_SDS_ENTRY {
    pub Hash: u32,
    pub SecurityId: u32,
    pub Offset: u64,
    pub Length: u32,
    pub Descriptor: [u8; 1],
}
impl ::core::marker::Copy for SD_ENUM_SDS_ENTRY {}
impl ::core::clone::Clone for SD_ENUM_SDS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_ENUM_SDS_INPUT {
    pub StartingOffset: u64,
    pub MaxSDEntriesToReturn: u64,
}
impl ::core::marker::Copy for SD_ENUM_SDS_INPUT {}
impl ::core::clone::Clone for SD_ENUM_SDS_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_ENUM_SDS_OUTPUT {
    pub NextOffset: u64,
    pub NumSDEntriesReturned: u64,
    pub NumSDBytesReturned: u64,
    pub SDEntry: [SD_ENUM_SDS_ENTRY; 1],
}
impl ::core::marker::Copy for SD_ENUM_SDS_OUTPUT {}
impl ::core::clone::Clone for SD_ENUM_SDS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_GLOBAL_CHANGE_INPUT {
    pub Flags: u32,
    pub ChangeType: u32,
    pub Anonymous: SD_GLOBAL_CHANGE_INPUT_0,
}
impl ::core::marker::Copy for SD_GLOBAL_CHANGE_INPUT {}
impl ::core::clone::Clone for SD_GLOBAL_CHANGE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union SD_GLOBAL_CHANGE_INPUT_0 {
    pub SdChange: SD_CHANGE_MACHINE_SID_INPUT,
    pub SdQueryStats: SD_QUERY_STATS_INPUT,
    pub SdEnumSds: SD_ENUM_SDS_INPUT,
}
impl ::core::marker::Copy for SD_GLOBAL_CHANGE_INPUT_0 {}
impl ::core::clone::Clone for SD_GLOBAL_CHANGE_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_GLOBAL_CHANGE_OUTPUT {
    pub Flags: u32,
    pub ChangeType: u32,
    pub Anonymous: SD_GLOBAL_CHANGE_OUTPUT_0,
}
impl ::core::marker::Copy for SD_GLOBAL_CHANGE_OUTPUT {}
impl ::core::clone::Clone for SD_GLOBAL_CHANGE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union SD_GLOBAL_CHANGE_OUTPUT_0 {
    pub SdChange: SD_CHANGE_MACHINE_SID_OUTPUT,
    pub SdQueryStats: SD_QUERY_STATS_OUTPUT,
    pub SdEnumSds: SD_ENUM_SDS_OUTPUT,
}
impl ::core::marker::Copy for SD_GLOBAL_CHANGE_OUTPUT_0 {}
impl ::core::clone::Clone for SD_GLOBAL_CHANGE_OUTPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SD_GLOBAL_CHANGE_TYPE_ENUM_SDS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SD_GLOBAL_CHANGE_TYPE_MACHINE_SID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SD_GLOBAL_CHANGE_TYPE_QUERY_STATS: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_QUERY_STATS_INPUT {
    pub Reserved: u32,
}
impl ::core::marker::Copy for SD_QUERY_STATS_INPUT {}
impl ::core::clone::Clone for SD_QUERY_STATS_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SD_QUERY_STATS_OUTPUT {
    pub SdsStreamSize: u64,
    pub SdsAllocationSize: u64,
    pub SiiStreamSize: u64,
    pub SiiAllocationSize: u64,
    pub SdhStreamSize: u64,
    pub SdhAllocationSize: u64,
    pub NumSDTotal: u64,
    pub NumSDUnused: u64,
}
impl ::core::marker::Copy for SD_QUERY_STATS_OUTPUT {}
impl ::core::clone::Clone for SD_QUERY_STATS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SEARCH_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SEARCH_ALL_NO_SEQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SEARCH_ALTERNATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SEARCH_ALT_NO_SEQ: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SEARCH_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SEARCH_PRI_NO_SEQ: u32 = 5u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SENDCMDINPARAMS {
    pub cBufferSize: u32,
    pub irDriveRegs: IDEREGS,
    pub bDriveNumber: u8,
    pub bReserved: [u8; 3],
    pub dwReserved: [u32; 4],
    pub bBuffer: [u8; 1],
}
impl ::core::marker::Copy for SENDCMDINPARAMS {}
impl ::core::clone::Clone for SENDCMDINPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SENDCMDOUTPARAMS {
    pub cBufferSize: u32,
    pub DriverStatus: DRIVERSTATUS,
    pub bBuffer: [u8; 1],
}
impl ::core::marker::Copy for SENDCMDOUTPARAMS {}
impl ::core::clone::Clone for SENDCMDOUTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SERIAL_NUMBER_LENGTH: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    pub Flags: u32,
    pub AlignmentShift: u32,
    pub FileOffsetToAlign: u64,
    pub FallbackAlignmentShift: u32,
}
impl ::core::marker::Copy for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {}
impl ::core::clone::Clone for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SET_DISK_ATTRIBUTES {
    pub Version: u32,
    pub Persist: super::super::Foundation::BOOLEAN,
    pub Reserved1: [u8; 3],
    pub Attributes: u64,
    pub AttributesMask: u64,
    pub Reserved2: [u32; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SET_DISK_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SET_DISK_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SET_PARTITION_INFORMATION {
    pub PartitionType: u8,
}
impl ::core::marker::Copy for SET_PARTITION_INFORMATION {}
impl ::core::clone::Clone for SET_PARTITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SET_PARTITION_INFORMATION_EX {
    pub PartitionStyle: PARTITION_STYLE,
    pub Anonymous: SET_PARTITION_INFORMATION_EX_0,
}
impl ::core::marker::Copy for SET_PARTITION_INFORMATION_EX {}
impl ::core::clone::Clone for SET_PARTITION_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union SET_PARTITION_INFORMATION_EX_0 {
    pub Mbr: SET_PARTITION_INFORMATION,
    pub Gpt: PARTITION_INFORMATION_GPT,
}
impl ::core::marker::Copy for SET_PARTITION_INFORMATION_EX_0 {}
impl ::core::clone::Clone for SET_PARTITION_INFORMATION_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SET_PURGE_FAILURE_MODE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SET_PURGE_FAILURE_MODE_ENABLED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SET_PURGE_FAILURE_MODE_INPUT {
    pub Flags: u32,
}
impl ::core::marker::Copy for SET_PURGE_FAILURE_MODE_INPUT {}
impl ::core::clone::Clone for SET_PURGE_FAILURE_MODE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SET_REPAIR_DISABLED_AND_BUGCHECK_ON_CORRUPT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SET_REPAIR_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SET_REPAIR_VALID_MASK: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SET_REPAIR_WARN_ABOUT_DATA_LOSS: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SHRINK_VOLUME_INFORMATION {
    pub ShrinkRequestType: SHRINK_VOLUME_REQUEST_TYPES,
    pub Flags: u64,
    pub NewNumberOfSectors: i64,
}
impl ::core::marker::Copy for SHRINK_VOLUME_INFORMATION {}
impl ::core::clone::Clone for SHRINK_VOLUME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type SHRINK_VOLUME_REQUEST_TYPES = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ShrinkPrepare: SHRINK_VOLUME_REQUEST_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ShrinkCommit: SHRINK_VOLUME_REQUEST_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ShrinkAbort: SHRINK_VOLUME_REQUEST_TYPES = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SI_COPYFILE {
    pub SourceFileNameLength: u32,
    pub DestinationFileNameLength: u32,
    pub Flags: u32,
    pub FileNameBuffer: [u16; 1],
}
impl ::core::marker::Copy for SI_COPYFILE {}
impl ::core::clone::Clone for SI_COPYFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_ABORT_OFFLINE_SELFTEST: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_CMD: u32 = 176u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_CYL_HI: u32 = 194u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_CYL_LOW: u32 = 79u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_ERROR_NO_MEM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_EXTENDED_SELFTEST_CAPTIVE: u32 = 130u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_EXTENDED_SELFTEST_OFFLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_GET_VERSION: u32 = 475264u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_IDE_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_INVALID_BUFFER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_INVALID_COMMAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_INVALID_DRIVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_INVALID_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_INVALID_IOCTL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_INVALID_REGISTER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_LOG_SECTOR_SIZE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_NOT_SUPPORTED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_NO_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_NO_IDE_DEVICE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_OFFLINE_ROUTINE_OFFLINE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_RCV_DRIVE_DATA: u32 = 508040u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_RCV_DRIVE_DATA_EX: u32 = 458892u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_READ_LOG: u32 = 213u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_SEND_DRIVE_COMMAND: u32 = 508036u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_SHORT_SELFTEST_CAPTIVE: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_SHORT_SELFTEST_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SMART_WRITE_LOG: u32 = 214u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    pub Version: u16,
}
impl ::core::marker::Copy for SMB_SHARE_FLUSH_AND_PURGE_INPUT {}
impl ::core::clone::Clone for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    pub cEntriesPurged: u32,
}
impl ::core::marker::Copy for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {}
impl ::core::clone::Clone for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SPACES_TRACKED_OFFSET_HEADER_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SRB_TYPE_SCSI_REQUEST_BLOCK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SRB_TYPE_STORAGE_REQUEST_BLOCK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STARTING_LCN_INPUT_BUFFER {
    pub StartingLcn: i64,
}
impl ::core::marker::Copy for STARTING_LCN_INPUT_BUFFER {}
impl ::core::clone::Clone for STARTING_LCN_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STARTING_LCN_INPUT_BUFFER_EX {
    pub StartingLcn: i64,
    pub Flags: u32,
}
impl ::core::marker::Copy for STARTING_LCN_INPUT_BUFFER_EX {}
impl ::core::clone::Clone for STARTING_LCN_INPUT_BUFFER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STARTING_VCN_INPUT_BUFFER {
    pub StartingVcn: i64,
}
impl ::core::marker::Copy for STARTING_VCN_INPUT_BUFFER {}
impl ::core::clone::Clone for STARTING_VCN_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub BytesPerCacheLine: u32,
    pub BytesOffsetForCacheAlignment: u32,
    pub BytesPerLogicalSector: u32,
    pub BytesPerPhysicalSector: u32,
    pub BytesOffsetForSectorAlignment: u32,
}
impl ::core::marker::Copy for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ADAPTER_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub MaximumTransferLength: u32,
    pub MaximumPhysicalPages: u32,
    pub AlignmentMask: u32,
    pub AdapterUsesPio: super::super::Foundation::BOOLEAN,
    pub AdapterScansDown: super::super::Foundation::BOOLEAN,
    pub CommandQueueing: super::super::Foundation::BOOLEAN,
    pub AcceleratedTransfer: super::super::Foundation::BOOLEAN,
    pub BusType: u8,
    pub BusMajorVersion: u16,
    pub BusMinorVersion: u16,
    pub SrbType: u8,
    pub AddressType: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_ADAPTER_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_ADAPTER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_ADAPTER_SERIAL_NUMBER {
    pub Version: u32,
    pub Size: u32,
    pub SerialNumber: [u16; 128],
}
impl ::core::marker::Copy for STORAGE_ADAPTER_SERIAL_NUMBER {}
impl ::core::clone::Clone for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_ADAPTER_SERIAL_NUMBER_V1_MAX_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_ADDRESS_TYPE_BTL8: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ALLOCATE_BC_STREAM_INPUT {
    pub Version: u32,
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: super::super::Foundation::BOOLEAN,
    pub Discardable: super::super::Foundation::BOOLEAN,
    pub Reserved1: [super::super::Foundation::BOOLEAN; 2],
    pub AccessType: u32,
    pub AccessMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: u64,
    pub NumOutStandingRequests: u32,
}
impl ::core::marker::Copy for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::core::clone::Clone for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_ASSOCIATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdAssocDevice: STORAGE_ASSOCIATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdAssocPort: STORAGE_ASSOCIATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdAssocTarget: STORAGE_ASSOCIATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_ATTRIBUTE_ASYNC_EVENT_NOTIFICATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_ATTRIBUTE_BLOCK_IO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_ATTRIBUTE_BYTE_ADDRESSABLE_IO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_ATTRIBUTE_DYNAMIC_PERSISTENCE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_ATTRIBUTE_MGMT {
    pub Version: u32,
    pub Size: u32,
    pub Action: STORAGE_ATTRIBUTE_MGMT_ACTION,
    pub Attribute: u32,
}
impl ::core::marker::Copy for STORAGE_ATTRIBUTE_MGMT {}
impl ::core::clone::Clone for STORAGE_ATTRIBUTE_MGMT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_ATTRIBUTE_MGMT_ACTION = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorAttributeMgmt_ClearAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorAttributeMgmt_SetAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorAttributeMgmt_ResetAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_ATTRIBUTE_PERF_SIZE_INDEPENDENT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_ATTRIBUTE_VOLATILE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_BREAK_RESERVATION_REQUEST {
    pub Length: u32,
    pub _unused: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl ::core::marker::Copy for STORAGE_BREAK_RESERVATION_REQUEST {}
impl ::core::clone::Clone for STORAGE_BREAK_RESERVATION_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_BUS_RESET_REQUEST {
    pub PathId: u8,
}
impl ::core::marker::Copy for STORAGE_BUS_RESET_REQUEST {}
impl ::core::clone::Clone for STORAGE_BUS_RESET_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_COMPONENT_HEALTH_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusUnknown: STORAGE_COMPONENT_HEALTH_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusNormal: STORAGE_COMPONENT_HEALTH_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusThrottled: STORAGE_COMPONENT_HEALTH_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusWarning: STORAGE_COMPONENT_HEALTH_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusDisabled: STORAGE_COMPONENT_HEALTH_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusFailed: STORAGE_COMPONENT_HEALTH_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_COMPONENT_ROLE_CACHE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_COMPONENT_ROLE_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_COMPONENT_ROLE_TIERING: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_COUNTER {
    pub Type: STORAGE_COUNTER_TYPE,
    pub Value: STORAGE_COUNTER_0,
}
impl ::core::marker::Copy for STORAGE_COUNTER {}
impl ::core::clone::Clone for STORAGE_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union STORAGE_COUNTER_0 {
    pub ManufactureDate: STORAGE_COUNTER_0_0,
    pub AsUlonglong: u64,
}
impl ::core::marker::Copy for STORAGE_COUNTER_0 {}
impl ::core::clone::Clone for STORAGE_COUNTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_COUNTER_0_0 {
    pub Week: u32,
    pub Year: u32,
}
impl ::core::marker::Copy for STORAGE_COUNTER_0_0 {}
impl ::core::clone::Clone for STORAGE_COUNTER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_COUNTERS {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfCounters: u32,
    pub Counters: [STORAGE_COUNTER; 1],
}
impl ::core::marker::Copy for STORAGE_COUNTERS {}
impl ::core::clone::Clone for STORAGE_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_COUNTER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeUnknown: STORAGE_COUNTER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeTemperatureCelsius: STORAGE_COUNTER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeTemperatureCelsiusMax: STORAGE_COUNTER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeReadErrorsTotal: STORAGE_COUNTER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeReadErrorsCorrected: STORAGE_COUNTER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeReadErrorsUncorrected: STORAGE_COUNTER_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWriteErrorsTotal: STORAGE_COUNTER_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWriteErrorsCorrected: STORAGE_COUNTER_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWriteErrorsUncorrected: STORAGE_COUNTER_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeManufactureDate: STORAGE_COUNTER_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeStartStopCycleCount: STORAGE_COUNTER_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeStartStopCycleCountMax: STORAGE_COUNTER_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeLoadUnloadCycleCount: STORAGE_COUNTER_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeLoadUnloadCycleCountMax: STORAGE_COUNTER_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWearPercentage: STORAGE_COUNTER_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWearPercentageWarning: STORAGE_COUNTER_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWearPercentageMax: STORAGE_COUNTER_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypePowerOnHours: STORAGE_COUNTER_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeReadLatency100NSMax: STORAGE_COUNTER_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWriteLatency100NSMax: STORAGE_COUNTER_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeFlushLatency100NSMax: STORAGE_COUNTER_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeMax: STORAGE_COUNTER_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_CRASH_TELEMETRY_REGKEY: &str = "\\Registry\\Machine\\System\\CurrentControlSet\\Control\\CrashControl\\StorageTelemetry";
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_CRYPTO_ALGORITHM_ID = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmUnknown: STORAGE_CRYPTO_ALGORITHM_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmXTSAES: STORAGE_CRYPTO_ALGORITHM_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmBitlockerAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmAESECB: STORAGE_CRYPTO_ALGORITHM_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmESSIVAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmMax: STORAGE_CRYPTO_ALGORITHM_ID = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_CRYPTO_CAPABILITY {
    pub Version: u32,
    pub Size: u32,
    pub CryptoCapabilityIndex: u32,
    pub AlgorithmId: STORAGE_CRYPTO_ALGORITHM_ID,
    pub KeySize: STORAGE_CRYPTO_KEY_SIZE,
    pub DataUnitSizeBitmask: u32,
}
impl ::core::marker::Copy for STORAGE_CRYPTO_CAPABILITY {}
impl ::core::clone::Clone for STORAGE_CRYPTO_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_CRYPTO_CAPABILITY_VERSION_1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_CRYPTO_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NumKeysSupported: u32,
    pub NumCryptoCapabilities: u32,
    pub CryptoCapabilities: [STORAGE_CRYPTO_CAPABILITY; 1],
}
impl ::core::marker::Copy for STORAGE_CRYPTO_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_CRYPTO_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_CRYPTO_DESCRIPTOR_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_CRYPTO_KEY_SIZE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySizeUnknown: STORAGE_CRYPTO_KEY_SIZE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySize128Bits: STORAGE_CRYPTO_KEY_SIZE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySize192Bits: STORAGE_CRYPTO_KEY_SIZE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySize256Bits: STORAGE_CRYPTO_KEY_SIZE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySize512Bits: STORAGE_CRYPTO_KEY_SIZE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DESCRIPTOR_HEADER {
    pub Version: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for STORAGE_DESCRIPTOR_HEADER {}
impl ::core::clone::Clone for STORAGE_DESCRIPTOR_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub Attributes: u64,
}
impl ::core::marker::Copy for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct STORAGE_DEVICE_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub DeviceType: u8,
    pub DeviceTypeModifier: u8,
    pub RemovableMedia: super::super::Foundation::BOOLEAN,
    pub CommandQueueing: super::super::Foundation::BOOLEAN,
    pub VendorIdOffset: u32,
    pub ProductIdOffset: u32,
    pub ProductRevisionOffset: u32,
    pub SerialNumberOffset: u32,
    pub BusType: super::super::Storage::FileSystem::STORAGE_BUS_TYPE,
    pub RawPropertiesLength: u32,
    pub RawDeviceProperties: [u8; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for STORAGE_DEVICE_DESCRIPTOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for STORAGE_DEVICE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfFaultDomains: u32,
    pub FaultDomainIds: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_FLAGS_PAGE_83_DEVICEGUID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_CONFLICT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_NOHWID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_DEVICE_FORM_FACTOR = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorUnknown: STORAGE_DEVICE_FORM_FACTOR = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactor3_5: STORAGE_DEVICE_FORM_FACTOR = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactor2_5: STORAGE_DEVICE_FORM_FACTOR = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactor1_8: STORAGE_DEVICE_FORM_FACTOR = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactor1_8Less: STORAGE_DEVICE_FORM_FACTOR = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorEmbedded: STORAGE_DEVICE_FORM_FACTOR = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorMemoryCard: STORAGE_DEVICE_FORM_FACTOR = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactormSata: STORAGE_DEVICE_FORM_FACTOR = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorM_2: STORAGE_DEVICE_FORM_FACTOR = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorPCIeBoard: STORAGE_DEVICE_FORM_FACTOR = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorDimm: STORAGE_DEVICE_FORM_FACTOR = 10i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_ID_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfIdentifiers: u32,
    pub Identifiers: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub LunMaxIoCount: u32,
    pub AdapterMaxIoCount: u32,
}
impl ::core::marker::Copy for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub State: u64,
}
impl ::core::marker::Copy for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub Location: DEVICE_LOCATION,
    pub StringOffset: u32,
}
impl ::core::marker::Copy for STORAGE_DEVICE_LOCATION_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_MANAGEMENT_STATUS {
    pub Version: u32,
    pub Size: u32,
    pub Health: STORAGE_DISK_HEALTH_STATUS,
    pub NumberOfOperationalStatus: u32,
    pub NumberOfAdditionalReasons: u32,
    pub OperationalStatus: [STORAGE_DISK_OPERATIONAL_STATUS; 16],
    pub AdditionalReasons: [STORAGE_OPERATIONAL_REASON; 1],
}
impl ::core::marker::Copy for STORAGE_DEVICE_MANAGEMENT_STATUS {}
impl ::core::clone::Clone for STORAGE_DEVICE_MANAGEMENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_MAX_OPERATIONAL_STATUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_NUMA_NODE_UNKNOWN: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_NUMA_PROPERTY {
    pub Version: u32,
    pub Size: u32,
    pub NumaNode: u32,
}
impl ::core::marker::Copy for STORAGE_DEVICE_NUMA_PROPERTY {}
impl ::core::clone::Clone for STORAGE_DEVICE_NUMA_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_NUMBER {
    pub DeviceType: u32,
    pub DeviceNumber: u32,
    pub PartitionNumber: u32,
}
impl ::core::marker::Copy for STORAGE_DEVICE_NUMBER {}
impl ::core::clone::Clone for STORAGE_DEVICE_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_NUMBERS {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfDevices: u32,
    pub Devices: [STORAGE_DEVICE_NUMBER; 1],
}
impl ::core::marker::Copy for STORAGE_DEVICE_NUMBERS {}
impl ::core::clone::Clone for STORAGE_DEVICE_NUMBERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_NUMBER_EX {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub DeviceType: u32,
    pub DeviceNumber: u32,
    pub DeviceGuid: ::windows_sys::core::GUID,
    pub PartitionNumber: u32,
}
impl ::core::marker::Copy for STORAGE_DEVICE_NUMBER_EX {}
impl ::core::clone::Clone for STORAGE_DEVICE_NUMBER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_POWER_CAP {
    pub Version: u32,
    pub Size: u32,
    pub Units: STORAGE_DEVICE_POWER_CAP_UNITS,
    pub MaxPower: u64,
}
impl ::core::marker::Copy for STORAGE_DEVICE_POWER_CAP {}
impl ::core::clone::Clone for STORAGE_DEVICE_POWER_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_DEVICE_POWER_CAP_UNITS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDevicePowerCapUnitsPercent: STORAGE_DEVICE_POWER_CAP_UNITS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDevicePowerCapUnitsMilliwatts: STORAGE_DEVICE_POWER_CAP_UNITS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_POWER_CAP_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NameOffset: u32,
    pub NumberOfLogicalCopies: u32,
    pub NumberOfPhysicalCopies: u32,
    pub PhysicalDiskRedundancy: u32,
    pub NumberOfColumns: u32,
    pub Interleave: u32,
}
impl ::core::marker::Copy for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    pub Version: u32,
    pub Size: u32,
    pub SupportsSelfEncryption: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_TELEMETRY_REGKEY: &str = "\\Registry\\Machine\\System\\CurrentControlSet\\Control\\Storage\\StorageTelemetry";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_TIERING_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub TotalNumberOfTiers: u32,
    pub NumberOfTiersReturned: u32,
    pub Tiers: [STORAGE_TIER; 1],
}
impl ::core::marker::Copy for STORAGE_DEVICE_TIERING_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    pub Version: u32,
    pub Size: u32,
    pub UnsafeShutdownCount: u32,
}
impl ::core::marker::Copy for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {}
impl ::core::clone::Clone for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DIAGNOSTIC_DATA {
    pub Version: u32,
    pub Size: u32,
    pub ProviderId: ::windows_sys::core::GUID,
    pub BufferSize: u32,
    pub Reserved: u32,
    pub DiagnosticDataBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_DIAGNOSTIC_DATA {}
impl ::core::clone::Clone for STORAGE_DIAGNOSTIC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DIAGNOSTIC_FLAG_ADAPTER_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_DIAGNOSTIC_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticLevelDefault: STORAGE_DIAGNOSTIC_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticLevelMax: STORAGE_DIAGNOSTIC_LEVEL = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DIAGNOSTIC_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub TargetType: STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: STORAGE_DIAGNOSTIC_LEVEL,
}
impl ::core::marker::Copy for STORAGE_DIAGNOSTIC_REQUEST {}
impl ::core::clone::Clone for STORAGE_DIAGNOSTIC_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_DIAGNOSTIC_TARGET_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypeUndefined: STORAGE_DIAGNOSTIC_TARGET_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypePort: STORAGE_DIAGNOSTIC_TARGET_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypeMiniport: STORAGE_DIAGNOSTIC_TARGET_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypeHbaFirmware: STORAGE_DIAGNOSTIC_TARGET_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypeMax: STORAGE_DIAGNOSTIC_TARGET_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_DISK_HEALTH_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthUnknown: STORAGE_DISK_HEALTH_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthUnhealthy: STORAGE_DISK_HEALTH_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthWarning: STORAGE_DISK_HEALTH_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthHealthy: STORAGE_DISK_HEALTH_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthMax: STORAGE_DISK_HEALTH_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_DISK_OPERATIONAL_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusNone: STORAGE_DISK_OPERATIONAL_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusUnknown: STORAGE_DISK_OPERATIONAL_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusOk: STORAGE_DISK_OPERATIONAL_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusPredictingFailure: STORAGE_DISK_OPERATIONAL_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusInService: STORAGE_DISK_OPERATIONAL_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusHardwareError: STORAGE_DISK_OPERATIONAL_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusNotUsable: STORAGE_DISK_OPERATIONAL_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusTransientError: STORAGE_DISK_OPERATIONAL_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusMissing: STORAGE_DISK_OPERATIONAL_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_EVENT_DEVICE_OPERATION: u64 = 4u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_EVENT_DEVICE_STATUS: u64 = 2u64;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_EVENT_MEDIA_STATUS: u64 = 1u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_EVENT_NOTIFICATION {
    pub Version: u32,
    pub Size: u32,
    pub Events: u64,
}
impl ::core::marker::Copy for STORAGE_EVENT_NOTIFICATION {}
impl ::core::clone::Clone for STORAGE_EVENT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_EVENT_NOTIFICATION_VERSION_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FAILURE_PREDICTION_CONFIG {
    pub Version: u32,
    pub Size: u32,
    pub Set: super::super::Foundation::BOOLEAN,
    pub Enabled: super::super::Foundation::BOOLEAN,
    pub Reserved: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FAILURE_PREDICTION_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FAILURE_PREDICTION_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_FAILURE_PREDICTION_CONFIG_V1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_FRU_ID_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub IdentifierSize: u32,
    pub Identifier: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_FRU_ID_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_FRU_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MaximumRequestSize: u64,
    pub EstimatedTimePerRequest: u32,
    pub NumOutStandingRequests: u32,
    pub RequestSize: u64,
}
impl ::core::marker::Copy for STORAGE_GET_BC_PROPERTIES_OUTPUT {}
impl ::core::clone::Clone for STORAGE_GET_BC_PROPERTIES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_HOTPLUG_INFO {
    pub Size: u32,
    pub MediaRemovable: super::super::Foundation::BOOLEAN,
    pub MediaHotplug: super::super::Foundation::BOOLEAN,
    pub DeviceHotplug: super::super::Foundation::BOOLEAN,
    pub WriteCacheEnableOverride: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_HOTPLUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_HOTPLUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub EnduranceInfo: STORAGE_HW_ENDURANCE_INFO,
}
impl ::core::marker::Copy for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_HW_ENDURANCE_INFO {
    pub ValidFields: u32,
    pub GroupId: u32,
    pub Flags: STORAGE_HW_ENDURANCE_INFO_0,
    pub LifePercentage: u32,
    pub BytesReadCount: [u8; 16],
    pub ByteWriteCount: [u8; 16],
}
impl ::core::marker::Copy for STORAGE_HW_ENDURANCE_INFO {}
impl ::core::clone::Clone for STORAGE_HW_ENDURANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_HW_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for STORAGE_HW_ENDURANCE_INFO_0 {}
impl ::core::clone::Clone for STORAGE_HW_ENDURANCE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_HW_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
    pub Reserved0: [u8; 3],
}
impl ::core::marker::Copy for STORAGE_HW_FIRMWARE_ACTIVATE {}
impl ::core::clone::Clone for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_HW_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_HW_FIRMWARE_DOWNLOAD {}
impl ::core::clone::Clone for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageSize: u32,
    pub Reserved2: u32,
    pub ImageBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {}
impl ::core::clone::Clone for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_HW_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub _bitfield: u8,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub FirmwareShared: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 3],
    pub ImagePayloadAlignment: u32,
    pub ImagePayloadMaxSize: u32,
    pub Slot: [STORAGE_HW_FIRMWARE_SLOT_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_HW_FIRMWARE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_HW_FIRMWARE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_HW_FIRMWARE_INFO_QUERY {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for STORAGE_HW_FIRMWARE_INFO_QUERY {}
impl ::core::clone::Clone for STORAGE_HW_FIRMWARE_INFO_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_HW_FIRMWARE_INVALID_SLOT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_HW_FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_HW_FIRMWARE_REVISION_LENGTH: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_HW_FIRMWARE_SLOT_INFO {
    pub Version: u32,
    pub Size: u32,
    pub SlotNumber: u8,
    pub _bitfield: u8,
    pub Reserved1: [u8; 6],
    pub Revision: [u8; 16],
}
impl ::core::marker::Copy for STORAGE_HW_FIRMWARE_SLOT_INFO {}
impl ::core::clone::Clone for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_IDENTIFIER {
    pub CodeSet: STORAGE_IDENTIFIER_CODE_SET,
    pub Type: STORAGE_IDENTIFIER_TYPE,
    pub IdentifierSize: u16,
    pub NextOffset: u16,
    pub Association: STORAGE_ASSOCIATION_TYPE,
    pub Identifier: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_IDENTIFIER {}
impl ::core::clone::Clone for STORAGE_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_IDENTIFIER_CODE_SET = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdCodeSetReserved: STORAGE_IDENTIFIER_CODE_SET = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdCodeSetBinary: STORAGE_IDENTIFIER_CODE_SET = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdCodeSetAscii: STORAGE_IDENTIFIER_CODE_SET = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdCodeSetUtf8: STORAGE_IDENTIFIER_CODE_SET = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_IDENTIFIER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeVendorSpecific: STORAGE_IDENTIFIER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeVendorId: STORAGE_IDENTIFIER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeEUI64: STORAGE_IDENTIFIER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeFCPHName: STORAGE_IDENTIFIER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypePortRelative: STORAGE_IDENTIFIER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeTargetPortGroup: STORAGE_IDENTIFIER_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeLogicalUnitGroup: STORAGE_IDENTIFIER_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeMD5LogicalUnitIdentifier: STORAGE_IDENTIFIER_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeScsiNameString: STORAGE_IDENTIFIER_TYPE = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_IDLE_POWER {
    pub Version: u32,
    pub Size: u32,
    pub _bitfield: u32,
    pub D3IdleTimeout: u32,
}
impl ::core::marker::Copy for STORAGE_IDLE_POWER {}
impl ::core::clone::Clone for STORAGE_IDLE_POWER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_IDLE_POWERUP_REASON {
    pub Version: u32,
    pub Size: u32,
    pub PowerupReason: STORAGE_POWERUP_REASON_TYPE,
}
impl ::core::marker::Copy for STORAGE_IDLE_POWERUP_REASON {}
impl ::core::clone::Clone for STORAGE_IDLE_POWERUP_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_IDLE_POWERUP_REASON_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_ID_NAA_FORMAT = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdNAAFormatIEEEExtended: STORAGE_ID_NAA_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdNAAFormatIEEERegistered: STORAGE_ID_NAA_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdNAAFormatIEEEERegisteredExtended: STORAGE_ID_NAA_FORMAT = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    pub Size: u32,
    pub Version: u32,
    pub _bitfield1: u8,
    pub Reserved1: [u8; 3],
    pub _bitfield2: u8,
    pub Reserved3: [u8; 3],
    pub AvailableMappingResources: u64,
    pub UsedMappingResources: u64,
}
impl ::core::marker::Copy for STORAGE_LB_PROVISIONING_MAP_RESOURCES {}
impl ::core::clone::Clone for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    pub Reserved: u16,
    pub SerialNumberLength: u16,
    pub SerialNumber: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_MEDIA_SERIAL_NUMBER_DATA {}
impl ::core::clone::Clone for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_MEDIA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DDS_4mm: STORAGE_MEDIA_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MiniQic: STORAGE_MEDIA_TYPE = 33i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const Travan: STORAGE_MEDIA_TYPE = 34i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QIC: STORAGE_MEDIA_TYPE = 35i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MP_8mm: STORAGE_MEDIA_TYPE = 36i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AME_8mm: STORAGE_MEDIA_TYPE = 37i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AIT1_8mm: STORAGE_MEDIA_TYPE = 38i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DLT: STORAGE_MEDIA_TYPE = 39i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NCTP: STORAGE_MEDIA_TYPE = 40i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IBM_3480: STORAGE_MEDIA_TYPE = 41i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IBM_3490E: STORAGE_MEDIA_TYPE = 42i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IBM_Magstar_3590: STORAGE_MEDIA_TYPE = 43i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IBM_Magstar_MP: STORAGE_MEDIA_TYPE = 44i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STK_DATA_D3: STORAGE_MEDIA_TYPE = 45i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SONY_DTF: STORAGE_MEDIA_TYPE = 46i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DV_6mm: STORAGE_MEDIA_TYPE = 47i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DMI: STORAGE_MEDIA_TYPE = 48i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SONY_D2: STORAGE_MEDIA_TYPE = 49i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CLEANER_CARTRIDGE: STORAGE_MEDIA_TYPE = 50i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CD_ROM: STORAGE_MEDIA_TYPE = 51i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CD_R: STORAGE_MEDIA_TYPE = 52i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CD_RW: STORAGE_MEDIA_TYPE = 53i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DVD_ROM: STORAGE_MEDIA_TYPE = 54i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DVD_R: STORAGE_MEDIA_TYPE = 55i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DVD_RW: STORAGE_MEDIA_TYPE = 56i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_3_RW: STORAGE_MEDIA_TYPE = 57i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_5_WO: STORAGE_MEDIA_TYPE = 58i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_5_RW: STORAGE_MEDIA_TYPE = 59i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_5_LIMDOW: STORAGE_MEDIA_TYPE = 60i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PC_5_WO: STORAGE_MEDIA_TYPE = 61i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PC_5_RW: STORAGE_MEDIA_TYPE = 62i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PD_5_RW: STORAGE_MEDIA_TYPE = 63i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ABL_5_WO: STORAGE_MEDIA_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PINNACLE_APEX_5_RW: STORAGE_MEDIA_TYPE = 65i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SONY_12_WO: STORAGE_MEDIA_TYPE = 66i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PHILIPS_12_WO: STORAGE_MEDIA_TYPE = 67i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HITACHI_12_WO: STORAGE_MEDIA_TYPE = 68i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CYGNET_12_WO: STORAGE_MEDIA_TYPE = 69i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const KODAK_14_WO: STORAGE_MEDIA_TYPE = 70i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_NFR_525: STORAGE_MEDIA_TYPE = 71i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NIKON_12_RW: STORAGE_MEDIA_TYPE = 72i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOMEGA_ZIP: STORAGE_MEDIA_TYPE = 73i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOMEGA_JAZ: STORAGE_MEDIA_TYPE = 74i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SYQUEST_EZ135: STORAGE_MEDIA_TYPE = 75i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SYQUEST_EZFLYER: STORAGE_MEDIA_TYPE = 76i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SYQUEST_SYJET: STORAGE_MEDIA_TYPE = 77i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AVATAR_F2: STORAGE_MEDIA_TYPE = 78i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MP2_8mm: STORAGE_MEDIA_TYPE = 79i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DST_S: STORAGE_MEDIA_TYPE = 80i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DST_M: STORAGE_MEDIA_TYPE = 81i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DST_L: STORAGE_MEDIA_TYPE = 82i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VXATape_1: STORAGE_MEDIA_TYPE = 83i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VXATape_2: STORAGE_MEDIA_TYPE = 84i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STK_9840: STORAGE_MEDIA_TYPE = 85i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LTO_Ultrium: STORAGE_MEDIA_TYPE = 86i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LTO_Accelis: STORAGE_MEDIA_TYPE = 87i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DVD_RAM: STORAGE_MEDIA_TYPE = 88i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AIT_8mm: STORAGE_MEDIA_TYPE = 89i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ADR_1: STORAGE_MEDIA_TYPE = 90i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ADR_2: STORAGE_MEDIA_TYPE = 91i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STK_9940: STORAGE_MEDIA_TYPE = 92i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SAIT: STORAGE_MEDIA_TYPE = 93i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VXATape: STORAGE_MEDIA_TYPE = 94i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub MediumProductType: u32,
}
impl ::core::marker::Copy for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_MINIPORT_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub Portdriver: STORAGE_PORT_CODE_SET,
    pub LUNResetSupported: super::super::Foundation::BOOLEAN,
    pub TargetResetSupported: super::super::Foundation::BOOLEAN,
    pub IoTimeoutValue: u16,
    pub ExtraIoInfoSupported: super::super::Foundation::BOOLEAN,
    pub Flags: STORAGE_MINIPORT_DESCRIPTOR_0,
    pub Reserved0: [u8; 2],
    pub Reserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_MINIPORT_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_MINIPORT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union STORAGE_MINIPORT_DESCRIPTOR_0 {
    pub Anonymous: STORAGE_MINIPORT_DESCRIPTOR_0_0,
    pub AsBYTE: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_MINIPORT_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_MINIPORT_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_MINIPORT_DESCRIPTOR_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_OFFLOAD_MAX_TOKEN_LENGTH: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_OFFLOAD_READ_OUTPUT {
    pub OffloadReadFlags: u32,
    pub Reserved: u32,
    pub LengthProtected: u64,
    pub TokenLength: u32,
    pub Token: STORAGE_OFFLOAD_TOKEN,
}
impl ::core::marker::Copy for STORAGE_OFFLOAD_READ_OUTPUT {}
impl ::core::clone::Clone for STORAGE_OFFLOAD_READ_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_OFFLOAD_READ_RANGE_TRUNCATED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_OFFLOAD_TOKEN {
    pub TokenType: [u8; 4],
    pub Reserved: [u8; 2],
    pub TokenIdLength: [u8; 2],
    pub Anonymous: STORAGE_OFFLOAD_TOKEN_0,
}
impl ::core::marker::Copy for STORAGE_OFFLOAD_TOKEN {}
impl ::core::clone::Clone for STORAGE_OFFLOAD_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union STORAGE_OFFLOAD_TOKEN_0 {
    pub StorageOffloadZeroDataToken: STORAGE_OFFLOAD_TOKEN_0_0,
    pub Token: [u8; 504],
}
impl ::core::marker::Copy for STORAGE_OFFLOAD_TOKEN_0 {}
impl ::core::clone::Clone for STORAGE_OFFLOAD_TOKEN_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_OFFLOAD_TOKEN_0_0 {
    pub Reserved2: [u8; 504],
}
impl ::core::marker::Copy for STORAGE_OFFLOAD_TOKEN_0_0 {}
impl ::core::clone::Clone for STORAGE_OFFLOAD_TOKEN_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_OFFLOAD_TOKEN_ID_LENGTH: u32 = 504u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_OFFLOAD_TOKEN_INVALID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_OFFLOAD_TOKEN_TYPE_ZERO_DATA: u32 = 4294901761u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_OFFLOAD_WRITE_OUTPUT {
    pub OffloadWriteFlags: u32,
    pub Reserved: u32,
    pub LengthCopied: u64,
}
impl ::core::marker::Copy for STORAGE_OFFLOAD_WRITE_OUTPUT {}
impl ::core::clone::Clone for STORAGE_OFFLOAD_WRITE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_OFFLOAD_WRITE_RANGE_TRUNCATED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_OPERATIONAL_REASON {
    pub Version: u32,
    pub Size: u32,
    pub Reason: STORAGE_OPERATIONAL_STATUS_REASON,
    pub RawBytes: STORAGE_OPERATIONAL_REASON_0,
}
impl ::core::marker::Copy for STORAGE_OPERATIONAL_REASON {}
impl ::core::clone::Clone for STORAGE_OPERATIONAL_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union STORAGE_OPERATIONAL_REASON_0 {
    pub ScsiSenseKey: STORAGE_OPERATIONAL_REASON_0_1,
    pub NVDIMM_N: STORAGE_OPERATIONAL_REASON_0_0,
    pub AsUlong: u32,
}
impl ::core::marker::Copy for STORAGE_OPERATIONAL_REASON_0 {}
impl ::core::clone::Clone for STORAGE_OPERATIONAL_REASON_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_OPERATIONAL_REASON_0_0 {
    pub CriticalHealth: u8,
    pub ModuleHealth: [u8; 2],
    pub ErrorThresholdStatus: u8,
}
impl ::core::marker::Copy for STORAGE_OPERATIONAL_REASON_0_0 {}
impl ::core::clone::Clone for STORAGE_OPERATIONAL_REASON_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_OPERATIONAL_REASON_0_1 {
    pub SenseKey: u8,
    pub ASC: u8,
    pub ASCQ: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for STORAGE_OPERATIONAL_REASON_0_1 {}
impl ::core::clone::Clone for STORAGE_OPERATIONAL_REASON_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_OPERATIONAL_STATUS_REASON = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonUnknown: STORAGE_OPERATIONAL_STATUS_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonScsiSenseCode: STORAGE_OPERATIONAL_STATUS_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonMedia: STORAGE_OPERATIONAL_STATUS_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonIo: STORAGE_OPERATIONAL_STATUS_REASON = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonThresholdExceeded: STORAGE_OPERATIONAL_STATUS_REASON = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonLostData: STORAGE_OPERATIONAL_STATUS_REASON = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonEnergySource: STORAGE_OPERATIONAL_STATUS_REASON = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonConfiguration: STORAGE_OPERATIONAL_STATUS_REASON = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonDeviceController: STORAGE_OPERATIONAL_STATUS_REASON = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonMediaController: STORAGE_OPERATIONAL_STATUS_REASON = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonComponent: STORAGE_OPERATIONAL_STATUS_REASON = 10i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonNVDIMM_N: STORAGE_OPERATIONAL_STATUS_REASON = 11i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonBackgroundOperation: STORAGE_OPERATIONAL_STATUS_REASON = 12i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonInvalidFirmware: STORAGE_OPERATIONAL_STATUS_REASON = 13i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonHealthCheck: STORAGE_OPERATIONAL_STATUS_REASON = 14i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonLostDataPersistence: STORAGE_OPERATIONAL_STATUS_REASON = 15i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonDisabledByPlatform: STORAGE_OPERATIONAL_STATUS_REASON = 16i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonLostWritePersistence: STORAGE_OPERATIONAL_STATUS_REASON = 17i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonDataPersistenceLossImminent: STORAGE_OPERATIONAL_STATUS_REASON = 18i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonWritePersistenceLossImminent: STORAGE_OPERATIONAL_STATUS_REASON = 19i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonMax: STORAGE_OPERATIONAL_STATUS_REASON = 20i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_PHYSICAL_ADAPTER_DATA {
    pub AdapterId: u32,
    pub HealthStatus: STORAGE_COMPONENT_HEALTH_STATUS,
    pub CommandProtocol: STORAGE_PROTOCOL_TYPE,
    pub SpecVersion: STORAGE_SPEC_VERSION,
    pub Vendor: [u8; 8],
    pub Model: [u8; 40],
    pub FirmwareRevision: [u8; 16],
    pub PhysicalLocation: [u8; 32],
    pub ExpanderConnected: super::super::Foundation::BOOLEAN,
    pub Reserved0: [u8; 3],
    pub Reserved1: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_PHYSICAL_ADAPTER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_PHYSICAL_ADAPTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PHYSICAL_DEVICE_DATA {
    pub DeviceId: u32,
    pub Role: u32,
    pub HealthStatus: STORAGE_COMPONENT_HEALTH_STATUS,
    pub CommandProtocol: STORAGE_PROTOCOL_TYPE,
    pub SpecVersion: STORAGE_SPEC_VERSION,
    pub FormFactor: STORAGE_DEVICE_FORM_FACTOR,
    pub Vendor: [u8; 8],
    pub Model: [u8; 40],
    pub FirmwareRevision: [u8; 16],
    pub Capacity: u64,
    pub PhysicalLocation: [u8; 32],
    pub Reserved: [u32; 2],
}
impl ::core::marker::Copy for STORAGE_PHYSICAL_DEVICE_DATA {}
impl ::core::clone::Clone for STORAGE_PHYSICAL_DEVICE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PHYSICAL_NODE_DATA {
    pub NodeId: u32,
    pub AdapterCount: u32,
    pub AdapterDataLength: u32,
    pub AdapterDataOffset: u32,
    pub DeviceCount: u32,
    pub DeviceDataLength: u32,
    pub DeviceDataOffset: u32,
    pub Reserved: [u32; 3],
}
impl ::core::marker::Copy for STORAGE_PHYSICAL_NODE_DATA {}
impl ::core::clone::Clone for STORAGE_PHYSICAL_NODE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NodeCount: u32,
    pub Reserved: u32,
    pub Node: [STORAGE_PHYSICAL_NODE_DATA; 1],
}
impl ::core::marker::Copy for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_PORT_CODE_SET = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetReserved: STORAGE_PORT_CODE_SET = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetStorport: STORAGE_PORT_CODE_SET = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetSCSIport: STORAGE_PORT_CODE_SET = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetSpaceport: STORAGE_PORT_CODE_SET = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetATAport: STORAGE_PORT_CODE_SET = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetUSBport: STORAGE_PORT_CODE_SET = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetSBP2port: STORAGE_PORT_CODE_SET = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetSDport: STORAGE_PORT_CODE_SET = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_POWERUP_REASON_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePowerupUnknown: STORAGE_POWERUP_REASON_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePowerupIO: STORAGE_POWERUP_REASON_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePowerupDeviceAttention: STORAGE_POWERUP_REASON_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PREDICT_FAILURE {
    pub PredictFailure: u32,
    pub VendorSpecific: [u8; 512],
}
impl ::core::marker::Copy for STORAGE_PREDICT_FAILURE {}
impl ::core::clone::Clone for STORAGE_PREDICT_FAILURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PRIORITY_HINT_SUPPORT {
    pub SupportFlags: u32,
}
impl ::core::marker::Copy for STORAGE_PRIORITY_HINT_SUPPORT {}
impl ::core::clone::Clone for STORAGE_PRIORITY_HINT_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PRIORITY_HINT_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceProperty: STORAGE_PROPERTY_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterProperty: STORAGE_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceIdProperty: STORAGE_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceUniqueIdProperty: STORAGE_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceWriteCacheProperty: STORAGE_PROPERTY_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageMiniportProperty: STORAGE_PROPERTY_ID = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAccessAlignmentProperty: STORAGE_PROPERTY_ID = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceSeekPenaltyProperty: STORAGE_PROPERTY_ID = 7i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceTrimProperty: STORAGE_PROPERTY_ID = 8i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceWriteAggregationProperty: STORAGE_PROPERTY_ID = 9i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceDeviceTelemetryProperty: STORAGE_PROPERTY_ID = 10i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceLBProvisioningProperty: STORAGE_PROPERTY_ID = 11i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDevicePowerProperty: STORAGE_PROPERTY_ID = 12i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceCopyOffloadProperty: STORAGE_PROPERTY_ID = 13i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceResiliencyProperty: STORAGE_PROPERTY_ID = 14i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceMediumProductType: STORAGE_PROPERTY_ID = 15i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterRpmbProperty: STORAGE_PROPERTY_ID = 16i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterCryptoProperty: STORAGE_PROPERTY_ID = 17i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceIoCapabilityProperty: STORAGE_PROPERTY_ID = 48i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterProtocolSpecificProperty: STORAGE_PROPERTY_ID = 49i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceProtocolSpecificProperty: STORAGE_PROPERTY_ID = 50i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterTemperatureProperty: STORAGE_PROPERTY_ID = 51i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceTemperatureProperty: STORAGE_PROPERTY_ID = 52i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterPhysicalTopologyProperty: STORAGE_PROPERTY_ID = 53i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDevicePhysicalTopologyProperty: STORAGE_PROPERTY_ID = 54i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceAttributesProperty: STORAGE_PROPERTY_ID = 55i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceManagementStatus: STORAGE_PROPERTY_ID = 56i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterSerialNumberProperty: STORAGE_PROPERTY_ID = 57i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceLocationProperty: STORAGE_PROPERTY_ID = 58i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceNumaProperty: STORAGE_PROPERTY_ID = 59i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceZonedDeviceProperty: STORAGE_PROPERTY_ID = 60i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceUnsafeShutdownCount: STORAGE_PROPERTY_ID = 61i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceEnduranceProperty: STORAGE_PROPERTY_ID = 62i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceLedStateProperty: STORAGE_PROPERTY_ID = 63i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceSelfEncryptionProperty: STORAGE_PROPERTY_ID = 64i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageFruIdProperty: STORAGE_PROPERTY_ID = 65i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PROPERTY_QUERY {
    pub PropertyId: STORAGE_PROPERTY_ID,
    pub QueryType: STORAGE_QUERY_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_PROPERTY_QUERY {}
impl ::core::clone::Clone for STORAGE_PROPERTY_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PROPERTY_SET {
    pub PropertyId: STORAGE_PROPERTY_ID,
    pub SetType: STORAGE_SET_TYPE,
    pub AdditionalParameters: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_PROPERTY_SET {}
impl ::core::clone::Clone for STORAGE_PROPERTY_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_PROTOCOL_ATA_DATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AtaDataTypeUnknown: STORAGE_PROTOCOL_ATA_DATA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AtaDataTypeIdentify: STORAGE_PROTOCOL_ATA_DATA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AtaDataTypeLogPage: STORAGE_PROTOCOL_ATA_DATA_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PROTOCOL_COMMAND {
    pub Version: u32,
    pub Length: u32,
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub Flags: u32,
    pub ReturnStatus: u32,
    pub ErrorCode: u32,
    pub CommandLength: u32,
    pub ErrorInfoLength: u32,
    pub DataToDeviceTransferLength: u32,
    pub DataFromDeviceTransferLength: u32,
    pub TimeOutValue: u32,
    pub ErrorInfoOffset: u32,
    pub DataToDeviceBufferOffset: u32,
    pub DataFromDeviceBufferOffset: u32,
    pub CommandSpecific: u32,
    pub Reserved0: u32,
    pub FixedProtocolReturnData: u32,
    pub Reserved1: [u32; 3],
    pub Command: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_PROTOCOL_COMMAND {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_COMMAND_LENGTH_NVME: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolSpecificData: STORAGE_PROTOCOL_SPECIFIC_DATA,
}
impl ::core::marker::Copy for STORAGE_PROTOCOL_DATA_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolSpecificData: STORAGE_PROTOCOL_SPECIFIC_DATA_EXT,
}
impl ::core::marker::Copy for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    pub Anonymous: STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0,
    pub AsUlong: u32,
}
impl ::core::marker::Copy for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_PROTOCOL_NVME_DATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NVMeDataTypeUnknown: STORAGE_PROTOCOL_NVME_DATA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NVMeDataTypeIdentify: STORAGE_PROTOCOL_NVME_DATA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NVMeDataTypeLogPage: STORAGE_PROTOCOL_NVME_DATA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NVMeDataTypeFeature: STORAGE_PROTOCOL_NVME_DATA_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PROTOCOL_SPECIFIC_DATA {
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub DataType: u32,
    pub ProtocolDataRequestValue: u32,
    pub ProtocolDataRequestSubValue: u32,
    pub ProtocolDataOffset: u32,
    pub ProtocolDataLength: u32,
    pub FixedProtocolReturnData: u32,
    pub ProtocolDataRequestSubValue2: u32,
    pub ProtocolDataRequestSubValue3: u32,
    pub ProtocolDataRequestSubValue4: u32,
}
impl ::core::marker::Copy for STORAGE_PROTOCOL_SPECIFIC_DATA {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    pub ProtocolType: STORAGE_PROTOCOL_TYPE,
    pub DataType: u32,
    pub ProtocolDataValue: u32,
    pub ProtocolDataSubValue: u32,
    pub ProtocolDataOffset: u32,
    pub ProtocolDataLength: u32,
    pub FixedProtocolReturnData: u32,
    pub ProtocolDataSubValue2: u32,
    pub ProtocolDataSubValue3: u32,
    pub ProtocolDataSubValue4: u32,
    pub ProtocolDataSubValue5: u32,
    pub Reserved: [u32; 5],
}
impl ::core::marker::Copy for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_SPECIFIC_NVME_NVM_COMMAND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_BUSY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_DATA_OVERRUN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_INSUFFICIENT_RESOURCES: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_INVALID_REQUEST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_NOT_SUPPORTED: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_NO_DEVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_PENDING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STATUS_THROTTLED_REQUEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PROTOCOL_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_PROTOCOL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeUnknown: STORAGE_PROTOCOL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeScsi: STORAGE_PROTOCOL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeAta: STORAGE_PROTOCOL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeNvme: STORAGE_PROTOCOL_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeSd: STORAGE_PROTOCOL_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeUfs: STORAGE_PROTOCOL_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeProprietary: STORAGE_PROTOCOL_TYPE = 126i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeMaxReserved: STORAGE_PROTOCOL_TYPE = 127i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_PROTOCOL_UFS_DATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeUnknown: STORAGE_PROTOCOL_UFS_DATA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryDescriptor: STORAGE_PROTOCOL_UFS_DATA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryFlag: STORAGE_PROTOCOL_UFS_DATA_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryDmeAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryDmePeerAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeMax: STORAGE_PROTOCOL_UFS_DATA_TYPE = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_Vhd\"`*"]
#[cfg(feature = "Win32_Storage_Vhd")]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    pub EntryLength: u32,
    pub DependencyTypeFlags: u32,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: super::super::Storage::Vhd::VIRTUAL_STORAGE_TYPE,
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_Vhd\"`*"]
#[cfg(feature = "Win32_Storage_Vhd")]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    pub EntryLength: u32,
    pub DependencyTypeFlags: u32,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: super::super::Storage::Vhd::VIRTUAL_STORAGE_TYPE,
    pub AncestorLevel: u32,
    pub HostVolumeNameOffset: u32,
    pub HostVolumeNameSize: u32,
    pub DependentVolumeNameOffset: u32,
    pub DependentVolumeNameSize: u32,
    pub RelativePathOffset: u32,
    pub RelativePathSize: u32,
    pub DependentDeviceNameOffset: u32,
    pub DependentDeviceNameSize: u32,
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    pub RequestLevel: u32,
    pub RequestFlags: u32,
}
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {}
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_Vhd\"`*"]
#[cfg(feature = "Win32_Storage_Vhd")]
pub struct STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    pub ResponseLevel: u32,
    pub NumberEntries: u32,
    pub Anonymous: STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0,
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_Vhd\"`*"]
#[cfg(feature = "Win32_Storage_Vhd")]
pub union STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    pub Lev1Depends: [STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY; 1],
    pub Lev2Depends: [STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY; 1],
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::marker::Copy for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::clone::Clone for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_QUERY_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyStandardQuery: STORAGE_QUERY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyExistsQuery: STORAGE_QUERY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyMaskQuery: STORAGE_QUERY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyQueryMaxDefined: STORAGE_QUERY_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_READ_CAPACITY {
    pub Version: u32,
    pub Size: u32,
    pub BlockLength: u32,
    pub NumberOfBlocks: i64,
    pub DiskLength: i64,
}
impl ::core::marker::Copy for STORAGE_READ_CAPACITY {}
impl ::core::clone::Clone for STORAGE_READ_CAPACITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_REINITIALIZE_MEDIA {
    pub Version: u32,
    pub Size: u32,
    pub TimeoutInSeconds: u32,
    pub SanitizeOption: STORAGE_REINITIALIZE_MEDIA_0,
}
impl ::core::marker::Copy for STORAGE_REINITIALIZE_MEDIA {}
impl ::core::clone::Clone for STORAGE_REINITIALIZE_MEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_REINITIALIZE_MEDIA_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for STORAGE_REINITIALIZE_MEDIA_0 {}
impl ::core::clone::Clone for STORAGE_REINITIALIZE_MEDIA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_RESERVE_ID = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdNone: STORAGE_RESERVE_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdHard: STORAGE_RESERVE_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdSoft: STORAGE_RESERVE_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdUpdateScratch: STORAGE_RESERVE_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdMax: STORAGE_RESERVE_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_RPMB_COMMAND_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbProgramAuthKey: STORAGE_RPMB_COMMAND_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbQueryWriteCounter: STORAGE_RPMB_COMMAND_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbAuthenticatedWrite: STORAGE_RPMB_COMMAND_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbAuthenticatedRead: STORAGE_RPMB_COMMAND_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbReadResultRequest: STORAGE_RPMB_COMMAND_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbAuthenticatedDeviceConfigWrite: STORAGE_RPMB_COMMAND_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbAuthenticatedDeviceConfigRead: STORAGE_RPMB_COMMAND_TYPE = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_RPMB_DATA_FRAME {
    pub Stuff: [u8; 196],
    pub KeyOrMAC: [u8; 32],
    pub Data: [u8; 256],
    pub Nonce: [u8; 16],
    pub WriteCounter: [u8; 4],
    pub Address: [u8; 2],
    pub BlockCount: [u8; 2],
    pub OperationResult: [u8; 2],
    pub RequestOrResponseType: [u8; 2],
}
impl ::core::marker::Copy for STORAGE_RPMB_DATA_FRAME {}
impl ::core::clone::Clone for STORAGE_RPMB_DATA_FRAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_RPMB_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub SizeInBytes: u32,
    pub MaxReliableWriteSizeInBytes: u32,
    pub FrameFormat: STORAGE_RPMB_FRAME_TYPE,
}
impl ::core::marker::Copy for STORAGE_RPMB_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_RPMB_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_RPMB_DESCRIPTOR_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_RPMB_FRAME_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageRpmbFrameTypeUnknown: STORAGE_RPMB_FRAME_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageRpmbFrameTypeStandard: STORAGE_RPMB_FRAME_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageRpmbFrameTypeMax: STORAGE_RPMB_FRAME_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_RPMB_MINIMUM_RELIABLE_WRITE_SIZE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_SANITIZE_METHOD = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageSanitizeMethodDefault: STORAGE_SANITIZE_METHOD = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageSanitizeMethodBlockErase: STORAGE_SANITIZE_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageSanitizeMethodCryptoErase: STORAGE_SANITIZE_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_SET_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyStandardSet: STORAGE_SET_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyExistsSet: STORAGE_SET_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertySetMaxDefined: STORAGE_SET_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union STORAGE_SPEC_VERSION {
    pub Anonymous: STORAGE_SPEC_VERSION_0,
    pub AsUlong: u32,
}
impl ::core::marker::Copy for STORAGE_SPEC_VERSION {}
impl ::core::clone::Clone for STORAGE_SPEC_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_SPEC_VERSION_0 {
    pub MinorVersion: STORAGE_SPEC_VERSION_0_0,
    pub MajorVersion: u16,
}
impl ::core::marker::Copy for STORAGE_SPEC_VERSION_0 {}
impl ::core::clone::Clone for STORAGE_SPEC_VERSION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union STORAGE_SPEC_VERSION_0_0 {
    pub Anonymous: STORAGE_SPEC_VERSION_0_0_0,
    pub AsUshort: u16,
}
impl ::core::marker::Copy for STORAGE_SPEC_VERSION_0_0 {}
impl ::core::clone::Clone for STORAGE_SPEC_VERSION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_SPEC_VERSION_0_0_0 {
    pub SubMinor: u8,
    pub Minor: u8,
}
impl ::core::marker::Copy for STORAGE_SPEC_VERSION_0_0_0 {}
impl ::core::clone::Clone for STORAGE_SPEC_VERSION_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_SUPPORTED_FEATURES_BYPASS_IO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_SUPPORTED_FEATURES_MASK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub CriticalTemperature: i16,
    pub WarningTemperature: i16,
    pub InfoCount: u16,
    pub Reserved0: [u8; 2],
    pub Reserved1: [u32; 2],
    pub TemperatureInfo: [STORAGE_TEMPERATURE_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_TEMPERATURE_INFO {
    pub Index: u16,
    pub Temperature: i16,
    pub OverThreshold: i16,
    pub UnderThreshold: i16,
    pub OverThresholdChangable: super::super::Foundation::BOOLEAN,
    pub UnderThresholdChangable: super::super::Foundation::BOOLEAN,
    pub EventGenerated: super::super::Foundation::BOOLEAN,
    pub Reserved0: u8,
    pub Reserved1: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_TEMPERATURE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_TEMPERATURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_TEMPERATURE_THRESHOLD {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u16,
    pub Index: u16,
    pub Threshold: i16,
    pub OverThreshold: super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_TEMPERATURE_THRESHOLD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_TEMPERATURE_THRESHOLD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TEMPERATURE_THRESHOLD_FLAG_ADAPTER_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TEMPERATURE_VALUE_NOT_REPORTED: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_TIER {
    pub Id: ::windows_sys::core::GUID,
    pub Name: [u16; 256],
    pub Description: [u16; 256],
    pub Flags: u64,
    pub ProvisionedCapacity: u64,
    pub MediaType: STORAGE_TIER_MEDIA_TYPE,
    pub Class: STORAGE_TIER_CLASS,
}
impl ::core::marker::Copy for STORAGE_TIER {}
impl ::core::clone::Clone for STORAGE_TIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_TIER_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierClassUnspecified: STORAGE_TIER_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierClassCapacity: STORAGE_TIER_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierClassPerformance: STORAGE_TIER_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierClassMax: STORAGE_TIER_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TIER_DESCRIPTION_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TIER_FLAG_NO_SEEK_PENALTY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TIER_FLAG_PARITY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TIER_FLAG_READ_CACHE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TIER_FLAG_SMR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TIER_FLAG_WRITE_BACK_CACHE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_TIER_MEDIA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeUnspecified: STORAGE_TIER_MEDIA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeDisk: STORAGE_TIER_MEDIA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeSsd: STORAGE_TIER_MEDIA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeScm: STORAGE_TIER_MEDIA_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeMax: STORAGE_TIER_MEDIA_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_TIER_REGION {
    pub TierId: ::windows_sys::core::GUID,
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for STORAGE_TIER_REGION {}
impl ::core::clone::Clone for STORAGE_TIER_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_WRITE_CACHE_PROPERTY {
    pub Version: u32,
    pub Size: u32,
    pub WriteCacheType: WRITE_CACHE_TYPE,
    pub WriteCacheEnabled: WRITE_CACHE_ENABLE,
    pub WriteCacheChangeable: WRITE_CACHE_CHANGE,
    pub WriteThroughSupported: WRITE_THROUGH,
    pub FlushCacheSupported: super::super::Foundation::BOOLEAN,
    pub UserDefinedPowerProtection: super::super::Foundation::BOOLEAN,
    pub NVCacheEnabled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_WRITE_CACHE_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_WRITE_CACHE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub DeviceType: STORAGE_ZONED_DEVICE_TYPES,
    pub ZoneCount: u32,
    pub ZoneAttributes: STORAGE_ZONED_DEVICE_DESCRIPTOR_0,
    pub ZoneGroupCount: u32,
    pub ZoneGroup: [STORAGE_ZONE_GROUP; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_ZONED_DEVICE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_ZONED_DEVICE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    pub SequentialRequiredZone: STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1,
    pub SequentialPreferredZone: STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    pub OptimalOpenZoneCount: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {
    pub MaxOpenZoneCount: u32,
    pub UnrestrictedRead: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_ZONED_DEVICE_TYPES = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonedDeviceTypeUnknown: STORAGE_ZONED_DEVICE_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonedDeviceTypeHostManaged: STORAGE_ZONED_DEVICE_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonedDeviceTypeHostAware: STORAGE_ZONED_DEVICE_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonedDeviceTypeDeviceManaged: STORAGE_ZONED_DEVICE_TYPES = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_ZONES_ATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonesAttributeTypeAndLengthMayDifferent: STORAGE_ZONES_ATTRIBUTES = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonesAttributeTypeSameLengthSame: STORAGE_ZONES_ATTRIBUTES = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonesAttributeTypeSameLastZoneLengthDifferent: STORAGE_ZONES_ATTRIBUTES = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonesAttributeTypeMayDifferentLengthSame: STORAGE_ZONES_ATTRIBUTES = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_ZONE_CONDITION = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionConventional: STORAGE_ZONE_CONDITION = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionEmpty: STORAGE_ZONE_CONDITION = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionImplicitlyOpened: STORAGE_ZONE_CONDITION = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionExplicitlyOpened: STORAGE_ZONE_CONDITION = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionClosed: STORAGE_ZONE_CONDITION = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionReadOnly: STORAGE_ZONE_CONDITION = 13i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionFull: STORAGE_ZONE_CONDITION = 14i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionOffline: STORAGE_ZONE_CONDITION = 15i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_ZONE_DESCRIPTOR {
    pub Size: u32,
    pub ZoneType: STORAGE_ZONE_TYPES,
    pub ZoneCondition: STORAGE_ZONE_CONDITION,
    pub ResetWritePointerRecommend: super::super::Foundation::BOOLEAN,
    pub Reserved0: [u8; 3],
    pub ZoneSize: u64,
    pub WritePointerOffset: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_ZONE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_ZONE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_ZONE_GROUP {
    pub ZoneCount: u32,
    pub ZoneType: STORAGE_ZONE_TYPES,
    pub ZoneSize: u64,
}
impl ::core::marker::Copy for STORAGE_ZONE_GROUP {}
impl ::core::clone::Clone for STORAGE_ZONE_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type STORAGE_ZONE_TYPES = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeUnknown: STORAGE_ZONE_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeConventional: STORAGE_ZONE_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeSequentialWriteRequired: STORAGE_ZONE_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeSequentialWritePreferred: STORAGE_ZONE_TYPES = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeMax: STORAGE_ZONE_TYPES = 4i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORATTRIBUTE_MANAGEMENT_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORATTRIBUTE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAMS_ASSOCIATE_ID_CLEAR: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    pub Flags: u32,
    pub StreamId: u32,
}
impl ::core::marker::Copy for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {}
impl ::core::clone::Clone for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAMS_ASSOCIATE_ID_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAMS_INVALID_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAMS_MAX_ID: u32 = 65535u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAMS_QUERY_ID_OUTPUT_BUFFER {
    pub StreamId: u32,
}
impl ::core::marker::Copy for STREAMS_QUERY_ID_OUTPUT_BUFFER {}
impl ::core::clone::Clone for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    pub OptimalWriteSize: u32,
    pub StreamGranularitySize: u32,
    pub StreamIdMin: u32,
    pub StreamIdMax: u32,
}
impl ::core::marker::Copy for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {}
impl ::core::clone::Clone for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_CLEAR_ENCRYPTION: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAM_EXTENT_ENTRY {
    pub Flags: u32,
    pub ExtentInformation: STREAM_EXTENT_ENTRY_0,
}
impl ::core::marker::Copy for STREAM_EXTENT_ENTRY {}
impl ::core::clone::Clone for STREAM_EXTENT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union STREAM_EXTENT_ENTRY_0 {
    pub RetrievalPointers: RETRIEVAL_POINTERS_BUFFER,
}
impl ::core::marker::Copy for STREAM_EXTENT_ENTRY_0 {}
impl ::core::clone::Clone for STREAM_EXTENT_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_EXTENT_ENTRY_ALL_EXTENTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_EXTENT_ENTRY_AS_RETRIEVAL_POINTERS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAM_INFORMATION_ENTRY {
    pub Version: u32,
    pub Flags: u32,
    pub StreamInformation: STREAM_INFORMATION_ENTRY_0,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union STREAM_INFORMATION_ENTRY_0 {
    pub DesiredStorageClass: STREAM_INFORMATION_ENTRY_0_1,
    pub DataStream: STREAM_INFORMATION_ENTRY_0_0,
    pub Reparse: STREAM_INFORMATION_ENTRY_0_3,
    pub Ea: STREAM_INFORMATION_ENTRY_0_2,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAM_INFORMATION_ENTRY_0_0 {
    pub Length: u16,
    pub Flags: u16,
    pub Reserved: u32,
    pub Vdl: u64,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0_0 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAM_INFORMATION_ENTRY_0_1 {
    pub Class: FILE_STORAGE_TIER_CLASS,
    pub Flags: u32,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0_1 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAM_INFORMATION_ENTRY_0_2 {
    pub Length: u16,
    pub Flags: u16,
    pub EaSize: u32,
    pub EaInformationOffset: u32,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0_2 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAM_INFORMATION_ENTRY_0_3 {
    pub Length: u16,
    pub Flags: u16,
    pub ReparseDataSize: u32,
    pub ReparseDataOffset: u32,
}
impl ::core::marker::Copy for STREAM_INFORMATION_ENTRY_0_3 {}
impl ::core::clone::Clone for STREAM_INFORMATION_ENTRY_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STREAM_LAYOUT_ENTRY {
    pub Version: u32,
    pub NextStreamOffset: u32,
    pub Flags: u32,
    pub ExtentInformationOffset: u32,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub StreamInformationOffset: u32,
    pub AttributeTypeCode: u32,
    pub AttributeFlags: u32,
    pub StreamIdentifierLength: u32,
    pub StreamIdentifier: [u16; 1],
}
impl ::core::marker::Copy for STREAM_LAYOUT_ENTRY {}
impl ::core::clone::Clone for STREAM_LAYOUT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_LAYOUT_ENTRY_HAS_INFORMATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_LAYOUT_ENTRY_IMMOVABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_LAYOUT_ENTRY_NO_CLUSTERS_ALLOCATED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_LAYOUT_ENTRY_PINNED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_LAYOUT_ENTRY_RESIDENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STREAM_SET_ENCRYPTION: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TAPE_GET_STATISTICS {
    pub Operation: u32,
}
impl ::core::marker::Copy for TAPE_GET_STATISTICS {}
impl ::core::clone::Clone for TAPE_GET_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TAPE_RESET_STATISTICS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TAPE_RETURN_ENV_INFO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TAPE_RETURN_STATISTICS: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TAPE_STATISTICS {
    pub Version: u32,
    pub Flags: u32,
    pub RecoveredWrites: i64,
    pub UnrecoveredWrites: i64,
    pub RecoveredReads: i64,
    pub UnrecoveredReads: i64,
    pub CompressionRatioReads: u8,
    pub CompressionRatioWrites: u8,
}
impl ::core::marker::Copy for TAPE_STATISTICS {}
impl ::core::clone::Clone for TAPE_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TC_PUBLIC_DATA_TYPE_ATAGP: &str = "ATAGPLogPages";
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TC_PUBLIC_DATA_TYPE_ATASMART: &str = "ATASMARTPages";
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TC_PUBLIC_DEVICEDUMP_CONTENT_SMART: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TELEMETRY_COMMAND_SIZE: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_CREATE_MINIVERSION_INFO {
    pub StructureVersion: u16,
    pub StructureLength: u16,
    pub BaseVersion: u32,
    pub MiniVersion: u16,
}
impl ::core::marker::Copy for TXFS_CREATE_MINIVERSION_INFO {}
impl ::core::clone::Clone for TXFS_CREATE_MINIVERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_GET_METADATA_INFO_OUT {
    pub TxfFileId: TXFS_GET_METADATA_INFO_OUT_0,
    pub LockingTransaction: ::windows_sys::core::GUID,
    pub LastLsn: u64,
    pub TransactionState: u32,
}
impl ::core::marker::Copy for TXFS_GET_METADATA_INFO_OUT {}
impl ::core::clone::Clone for TXFS_GET_METADATA_INFO_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_GET_METADATA_INFO_OUT_0 {
    pub LowPart: i64,
    pub HighPart: i64,
}
impl ::core::marker::Copy for TXFS_GET_METADATA_INFO_OUT_0 {}
impl ::core::clone::Clone for TXFS_GET_METADATA_INFO_OUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_GET_TRANSACTED_VERSION {
    pub ThisBaseVersion: u32,
    pub LatestVersion: u32,
    pub ThisMiniVersion: u16,
    pub FirstMiniVersion: u16,
    pub LatestMiniVersion: u16,
}
impl ::core::marker::Copy for TXFS_GET_TRANSACTED_VERSION {}
impl ::core::clone::Clone for TXFS_GET_TRANSACTED_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_LIST_TRANSACTIONS {
    pub NumberOfTransactions: u64,
    pub BufferSizeRequired: u64,
}
impl ::core::marker::Copy for TXFS_LIST_TRANSACTIONS {}
impl ::core::clone::Clone for TXFS_LIST_TRANSACTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_LIST_TRANSACTIONS_ENTRY {
    pub TransactionId: ::windows_sys::core::GUID,
    pub TransactionState: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
}
impl ::core::marker::Copy for TXFS_LIST_TRANSACTIONS_ENTRY {}
impl ::core::clone::Clone for TXFS_LIST_TRANSACTIONS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES {
    pub KtmTransaction: ::windows_sys::core::GUID,
    pub NumberOfFiles: u64,
    pub BufferSizeRequired: u64,
    pub Offset: u64,
}
impl ::core::marker::Copy for TXFS_LIST_TRANSACTION_LOCKED_FILES {}
impl ::core::clone::Clone for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    pub Offset: u64,
    pub NameFlags: u32,
    pub FileId: i64,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {}
impl ::core::clone::Clone for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_CREATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_DELETED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_LOGGING_MODE_FULL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_LOGGING_MODE_SIMPLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_MODIFY_RM {
    pub Flags: TXFS_RMF_LAGS,
    pub LogContainerCountMax: u32,
    pub LogContainerCountMin: u32,
    pub LogContainerCount: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub Reserved: u64,
    pub LoggingMode: u16,
}
impl ::core::marker::Copy for TXFS_MODIFY_RM {}
impl ::core::clone::Clone for TXFS_MODIFY_RM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_QUERY_RM_INFORMATION {
    pub BytesRequired: u32,
    pub TailLsn: u64,
    pub CurrentLsn: u64,
    pub ArchiveTailLsn: u64,
    pub LogContainerSize: u64,
    pub HighestVirtualClock: i64,
    pub LogContainerCount: u32,
    pub LogContainerCountMax: u32,
    pub LogContainerCountMin: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub Flags: TXFS_RMF_LAGS,
    pub LoggingMode: u16,
    pub Reserved: u16,
    pub RmState: u32,
    pub LogCapacity: u64,
    pub LogFree: u64,
    pub TopsSize: u64,
    pub TopsUsed: u64,
    pub TransactionCount: u64,
    pub OnePCCount: u64,
    pub TwoPCCount: u64,
    pub NumberLogFileFull: u64,
    pub OldestTransactionAge: u64,
    pub RMName: ::windows_sys::core::GUID,
    pub TmLogPathOffset: u32,
}
impl ::core::marker::Copy for TXFS_QUERY_RM_INFORMATION {}
impl ::core::clone::Clone for TXFS_QUERY_RM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_READ_BACKUP_INFORMATION_OUT {
    pub Anonymous: TXFS_READ_BACKUP_INFORMATION_OUT_0,
}
impl ::core::marker::Copy for TXFS_READ_BACKUP_INFORMATION_OUT {}
impl ::core::clone::Clone for TXFS_READ_BACKUP_INFORMATION_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub union TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    pub BufferLength: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for TXFS_READ_BACKUP_INFORMATION_OUT_0 {}
impl ::core::clone::Clone for TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type TXFS_RMF_LAGS = u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOGGING_MODE: TXFS_RMF_LAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_RENAME_RM: TXFS_RMF_LAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX: TXFS_RMF_LAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN: TXFS_RMF_LAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: TXFS_RMF_LAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: TXFS_RMF_LAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: TXFS_RMF_LAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: TXFS_RMF_LAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: TXFS_RMF_LAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_GROW_LOG: TXFS_RMF_LAGS = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_SHRINK_LOG: TXFS_RMF_LAGS = 2048u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_ENFORCE_MINIMUM_SIZE: TXFS_RMF_LAGS = 4096u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_PRESERVE_CHANGES: TXFS_RMF_LAGS = 8192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_RESET_RM_AT_NEXT_START: TXFS_RMF_LAGS = 16384u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START: TXFS_RMF_LAGS = 32768u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_PREFER_CONSISTENCY: TXFS_RMF_LAGS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_PREFER_AVAILABILITY: TXFS_RMF_LAGS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_STATE_ACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_STATE_NOT_STARTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_STATE_SHUTTING_DOWN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_STATE_STARTING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_REDO_LSN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_VIRTUAL_CLOCK: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_ROLLFORWARD_REDO_INFORMATION {
    pub LastVirtualClock: i64,
    pub LastRedoLsn: u64,
    pub HighestRecoveryLsn: u64,
    pub Flags: u32,
}
impl ::core::marker::Copy for TXFS_ROLLFORWARD_REDO_INFORMATION {}
impl ::core::clone::Clone for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_SAVEPOINT_CLEAR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_SAVEPOINT_CLEAR_ALL: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TXFS_SAVEPOINT_INFORMATION {
    pub KtmTransaction: super::super::Foundation::HANDLE,
    pub ActionCode: u32,
    pub SavepointId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TXFS_SAVEPOINT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TXFS_SAVEPOINT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_SAVEPOINT_ROLLBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_SAVEPOINT_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOGGING_MODE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MAX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOG_CONTAINER_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_PREFER_AVAILABILITY: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_PREFER_CONSISTENCY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_PRESERVE_CHANGES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_START_RM_FLAG_RECOVER_BEST_EFFORT: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_START_RM_INFORMATION {
    pub Flags: u32,
    pub LogContainerSize: u64,
    pub LogContainerCountMin: u32,
    pub LogContainerCountMax: u32,
    pub LogGrowthIncrement: u32,
    pub LogAutoShrinkPercentage: u32,
    pub TmLogPathOffset: u32,
    pub TmLogPathLength: u16,
    pub LoggingMode: u16,
    pub LogPathLength: u16,
    pub Reserved: u16,
    pub LogPath: [u16; 1],
}
impl ::core::marker::Copy for TXFS_START_RM_INFORMATION {}
impl ::core::clone::Clone for TXFS_START_RM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_TRANSACTED_VERSION_NONTRANSACTED: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_TRANSACTED_VERSION_UNCOMMITTED: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TXFS_TRANSACTION_ACTIVE_INFO {
    pub TransactionsActiveAtSnapshot: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TXFS_TRANSACTION_ACTIVE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TXFS_TRANSACTION_ACTIVE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_TRANSACTION_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_TRANSACTION_STATE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_TRANSACTION_STATE_NOTACTIVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_TRANSACTION_STATE_PREPARED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_WRITE_BACKUP_INFORMATION {
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for TXFS_WRITE_BACKUP_INFORMATION {}
impl ::core::clone::Clone for TXFS_WRITE_BACKUP_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UNDEFINE_ALTERNATE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UNDEFINE_PRIMARY: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UNLOCK_ELEMENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UNRECOVERED_READS_VALID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UNRECOVERED_WRITES_VALID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type USN_DELETE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_DELETE_FLAG_DELETE: USN_DELETE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_DELETE_FLAG_NOTIFY: USN_DELETE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_DELETE_VALID_FLAGS: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct USN_JOURNAL_DATA_V0 {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
}
impl ::core::marker::Copy for USN_JOURNAL_DATA_V0 {}
impl ::core::clone::Clone for USN_JOURNAL_DATA_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct USN_JOURNAL_DATA_V1 {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
    pub MinSupportedMajorVersion: u16,
    pub MaxSupportedMajorVersion: u16,
}
impl ::core::marker::Copy for USN_JOURNAL_DATA_V1 {}
impl ::core::clone::Clone for USN_JOURNAL_DATA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct USN_JOURNAL_DATA_V2 {
    pub UsnJournalID: u64,
    pub FirstUsn: i64,
    pub NextUsn: i64,
    pub LowestValidUsn: i64,
    pub MaxUsn: i64,
    pub MaximumSize: u64,
    pub AllocationDelta: u64,
    pub MinSupportedMajorVersion: u16,
    pub MaxSupportedMajorVersion: u16,
    pub Flags: u32,
    pub RangeTrackChunkSize: u64,
    pub RangeTrackFileSizeThreshold: i64,
}
impl ::core::marker::Copy for USN_JOURNAL_DATA_V2 {}
impl ::core::clone::Clone for USN_JOURNAL_DATA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_PAGE_SIZE: u32 = 4096u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct USN_RANGE_TRACK_OUTPUT {
    pub Usn: i64,
}
impl ::core::marker::Copy for USN_RANGE_TRACK_OUTPUT {}
impl ::core::clone::Clone for USN_RANGE_TRACK_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_BASIC_INFO_CHANGE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_CLOSE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_COMPRESSION_CHANGE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_DATA_EXTEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_DATA_OVERWRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_DATA_TRUNCATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_DESIRED_STORAGE_CLASS_CHANGE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_EA_CHANGE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_ENCRYPTION_CHANGE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_FILE_CREATE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_FILE_DELETE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_HARD_LINK_CHANGE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_INDEXABLE_CHANGE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_INTEGRITY_CHANGE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_NAMED_DATA_EXTEND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_NAMED_DATA_OVERWRITE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_NAMED_DATA_TRUNCATION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_OBJECT_ID_CHANGE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_RENAME_NEW_NAME: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_RENAME_OLD_NAME: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_REPARSE_POINT_CHANGE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_SECURITY_CHANGE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_STREAM_CHANGE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_REASON_TRANSACTED_CHANGE: u32 = 4194304u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct USN_RECORD_COMMON_HEADER {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for USN_RECORD_COMMON_HEADER {}
impl ::core::clone::Clone for USN_RECORD_COMMON_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct USN_RECORD_EXTENT {
    pub Offset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for USN_RECORD_EXTENT {}
impl ::core::clone::Clone for USN_RECORD_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub union USN_RECORD_UNION {
    pub Header: USN_RECORD_COMMON_HEADER,
    pub V2: USN_RECORD_V2,
    pub V3: USN_RECORD_V3,
    pub V4: USN_RECORD_V4,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for USN_RECORD_UNION {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for USN_RECORD_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct USN_RECORD_V2 {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub FileReferenceNumber: u64,
    pub ParentFileReferenceNumber: u64,
    pub Usn: i64,
    pub TimeStamp: i64,
    pub Reason: u32,
    pub SourceInfo: u32,
    pub SecurityId: u32,
    pub FileAttributes: u32,
    pub FileNameLength: u16,
    pub FileNameOffset: u16,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for USN_RECORD_V2 {}
impl ::core::clone::Clone for USN_RECORD_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct USN_RECORD_V3 {
    pub RecordLength: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub FileReferenceNumber: super::super::Storage::FileSystem::FILE_ID_128,
    pub ParentFileReferenceNumber: super::super::Storage::FileSystem::FILE_ID_128,
    pub Usn: i64,
    pub TimeStamp: i64,
    pub Reason: u32,
    pub SourceInfo: u32,
    pub SecurityId: u32,
    pub FileAttributes: u32,
    pub FileNameLength: u16,
    pub FileNameOffset: u16,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for USN_RECORD_V3 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for USN_RECORD_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct USN_RECORD_V4 {
    pub Header: USN_RECORD_COMMON_HEADER,
    pub FileReferenceNumber: super::super::Storage::FileSystem::FILE_ID_128,
    pub ParentFileReferenceNumber: super::super::Storage::FileSystem::FILE_ID_128,
    pub Usn: i64,
    pub Reason: u32,
    pub SourceInfo: USN_SOURCE_INFO_ID,
    pub RemainingExtents: u32,
    pub NumberOfExtents: u16,
    pub ExtentSize: u16,
    pub Extents: [USN_RECORD_EXTENT; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for USN_RECORD_V4 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for USN_RECORD_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type USN_SOURCE_INFO_ID = u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_SOURCE_AUXILIARY_DATA: USN_SOURCE_INFO_ID = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_SOURCE_DATA_MANAGEMENT: USN_SOURCE_INFO_ID = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_SOURCE_REPLICATION_MANAGEMENT: USN_SOURCE_INFO_ID = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_SOURCE_CLIENT_REPLICATION_MANAGEMENT: USN_SOURCE_INFO_ID = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct USN_TRACK_MODIFIED_RANGES {
    pub Flags: u32,
    pub Unused: u32,
    pub ChunkSize: u64,
    pub FileSizeThreshold: i64,
}
impl ::core::marker::Copy for USN_TRACK_MODIFIED_RANGES {}
impl ::core::clone::Clone for USN_TRACK_MODIFIED_RANGES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VALID_NTFT: u32 = 192u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VENDOR_ID_LENGTH: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VERIFY_INFORMATION {
    pub StartingOffset: i64,
    pub Length: u32,
}
impl ::core::marker::Copy for VERIFY_INFORMATION {}
impl ::core::clone::Clone for VERIFY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT {
    pub NumberOfWorkerThreads: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for VIRTUALIZATION_INSTANCE_INFO_INPUT {}
impl ::core::clone::Clone for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    pub HeaderSize: u16,
    pub Flags: u32,
    pub NotificationInfoSize: u32,
    pub NotificationInfoOffset: u16,
    pub ProviderMajorVersion: u16,
}
impl ::core::marker::Copy for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {}
impl ::core::clone::Clone for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    pub VirtualizationInstanceID: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {}
impl ::core::clone::Clone for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type VIRTUAL_STORAGE_BEHAVIOR_CODE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorUndefined: VIRTUAL_STORAGE_BEHAVIOR_CODE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorCacheWriteThrough: VIRTUAL_STORAGE_BEHAVIOR_CODE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorCacheWriteBack: VIRTUAL_STORAGE_BEHAVIOR_CODE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorStopIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorRestartIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    pub Size: u32,
    pub BehaviorCode: VIRTUAL_STORAGE_BEHAVIOR_CODE,
}
impl ::core::marker::Copy for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {}
impl ::core::clone::Clone for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VOLUME_BITMAP_BUFFER {
    pub StartingLcn: i64,
    pub BitmapSize: i64,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for VOLUME_BITMAP_BUFFER {}
impl ::core::clone::Clone for VOLUME_BITMAP_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VOLUME_DISK_EXTENTS {
    pub NumberOfDiskExtents: u32,
    pub Extents: [DISK_EXTENT; 1],
}
impl ::core::marker::Copy for VOLUME_DISK_EXTENTS {}
impl ::core::clone::Clone for VOLUME_DISK_EXTENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: u64,
}
impl ::core::marker::Copy for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {}
impl ::core::clone::Clone for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VOLUME_IS_DIRTY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VOLUME_SESSION_OPEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VOLUME_UPGRADE_SCHEDULED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WIM_PROVIDER_ADD_OVERLAY_INPUT {
    pub WimType: u32,
    pub WimIndex: u32,
    pub WimFileNameOffset: u32,
    pub WimFileNameLength: u32,
}
impl ::core::marker::Copy for WIM_PROVIDER_ADD_OVERLAY_INPUT {}
impl ::core::clone::Clone for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WIM_PROVIDER_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WIM_PROVIDER_EXTERNAL_FLAG_NOT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WIM_PROVIDER_EXTERNAL_FLAG_SUSPENDED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WIM_PROVIDER_EXTERNAL_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub DataSourceId: i64,
    pub ResourceHash: [u8; 20],
}
impl ::core::marker::Copy for WIM_PROVIDER_EXTERNAL_INFO {}
impl ::core::clone::Clone for WIM_PROVIDER_EXTERNAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WIM_PROVIDER_OVERLAY_ENTRY {
    pub NextEntryOffset: u32,
    pub DataSourceId: i64,
    pub WimGuid: ::windows_sys::core::GUID,
    pub WimFileNameOffset: u32,
    pub WimType: u32,
    pub WimIndex: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for WIM_PROVIDER_OVERLAY_ENTRY {}
impl ::core::clone::Clone for WIM_PROVIDER_OVERLAY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    pub DataSourceId: i64,
}
impl ::core::marker::Copy for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {}
impl ::core::clone::Clone for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    pub DataSourceId: i64,
}
impl ::core::marker::Copy for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {}
impl ::core::clone::Clone for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    pub DataSourceId: i64,
    pub WimFileNameOffset: u32,
    pub WimFileNameLength: u32,
}
impl ::core::marker::Copy for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {}
impl ::core::clone::Clone for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WOF_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct WOF_EXTERNAL_FILE_ID {
    pub FileId: super::super::Storage::FileSystem::FILE_ID_128,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for WOF_EXTERNAL_FILE_ID {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for WOF_EXTERNAL_FILE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WOF_EXTERNAL_INFO {
    pub Version: u32,
    pub Provider: u32,
}
impl ::core::marker::Copy for WOF_EXTERNAL_INFO {}
impl ::core::clone::Clone for WOF_EXTERNAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WOF_PROVIDER_CLOUD: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WOF_VERSION_INFO {
    pub WofVersion: u32,
}
impl ::core::marker::Copy for WOF_VERSION_INFO {}
impl ::core::clone::Clone for WOF_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type WRITE_CACHE_CHANGE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheChangeUnknown: WRITE_CACHE_CHANGE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheNotChangeable: WRITE_CACHE_CHANGE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheChangeable: WRITE_CACHE_CHANGE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type WRITE_CACHE_ENABLE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheEnableUnknown: WRITE_CACHE_ENABLE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheDisabled: WRITE_CACHE_ENABLE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheEnabled: WRITE_CACHE_ENABLE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type WRITE_CACHE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheTypeUnknown: WRITE_CACHE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheTypeNone: WRITE_CACHE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheTypeWriteBack: WRITE_CACHE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheTypeWriteThrough: WRITE_CACHE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WRITE_COMPRESSION_INFO_VALID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type WRITE_THROUGH = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteThroughUnknown: WRITE_THROUGH = 0i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteThroughNotSupported: WRITE_THROUGH = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteThroughSupported: WRITE_THROUGH = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WRITE_USN_REASON_INPUT {
    pub Flags: u32,
    pub UsnReasonToWrite: u32,
}
impl ::core::marker::Copy for WRITE_USN_REASON_INPUT {}
impl ::core::clone::Clone for WRITE_USN_REASON_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub type _DEVICEDUMP_COLLECTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TCCollectionBugCheck: _DEVICEDUMP_COLLECTION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TCCollectionApplicationRequested: _DEVICEDUMP_COLLECTION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TCCollectionDeviceRequested: _DEVICEDUMP_COLLECTION_TYPE = 3i32;
